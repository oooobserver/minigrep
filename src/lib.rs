use std::error::Error;
use std::fs;
use std::env;

pub struct Config{
    pub action: String, 
    pub file_path: String,
    pub ignore_case: bool,
}


impl Config {
    pub fn build(args:&Vec<String>) -> Result<Config,&'static str>{
        if args.len() < 3{
            return Err("not enough parameters");
        }

        let action = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { action, file_path, ignore_case })
    }

    pub fn run(self) -> Result<(),Box<dyn Error>> {
        let contents = fs::read_to_string(self.file_path)?;
        let results = if self.ignore_case{
            search_case_insensitive(&self.action, &contents)
        }else{
            search(&self.action, &contents)
        };

        for line in results {
            println!("{line}");
        }
        Ok(())
    }
}

pub fn search <'a>(action:&str, contents: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    for line in contents.lines(){
        if line.contains(action){
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive <'a>(
    action:&str,
    contents: &'a str
) -> Vec<&'a str>{
    let action = action.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&action){
            result.push(line);
        }
    }
    result

}




#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}


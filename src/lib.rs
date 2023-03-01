use std::fs;
use std::error::Error;
use std::env;

pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
    println!("Searching for word : '{}' in the file {}...", config.query_word, config.file_path);
    let content = fs::read_to_string(config.file_path)?;
    // println!("Contents of the file: \n{content}");
    let results = if config.ignore_case {
    search_case_insensitive(&config.query_word, &content)
    } else {
    search_case_insensitive(&config.query_word, &content)
    };
    if results.len() == 0 { 
        println!("No Matches found..");
        return Ok(()); 
    }
    for i in &results{
        println!("{}",i)
    }


    Ok(())
}

pub struct Config {
    query_word : String,
    file_path : String,
    ignore_case : bool,
}

impl Config{
    pub fn build (args: Vec<String>)-> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("Insufficient Arguments.");
        }
        let query_word: String = args[1].clone();
        let file_path : String = args[2].clone();
        let ignore_case : bool =env::var("IGNORE_CASE").is_ok();
        Ok(Config{ query_word, file_path, ignore_case } )
    }
}

pub fn search<'a>(query : &'a str, content : &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for i in content.lines(){
        if i.contains(query){
            results.push(i);
        }
    }
    results
}


pub fn search_case_insensitive<'a>(query : &'a str, content : &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for i in content.lines(){
        if i.to_lowercase().contains(&query){
            results.push(i);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn one_result() {
        let query = "test";
        let content = "\
Rust supports
tests, int and String.
And much more.";
        assert_eq!(vec!["tests, int and String."], search(query,content));
    }
    
    #[test]
    fn case_insensitive_result() {
        let query = "Tests";
        let content = "\
Rust supports
tests, int and String.
And much more.";
        assert_eq!(vec!["tests, int and String."], search_case_insensitive(query,content));
    }

}


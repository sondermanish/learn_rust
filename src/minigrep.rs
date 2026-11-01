use std::{fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from(
                "Not enough arguments. Expected a query and a file path."
            ));
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), String> {

    if config.file_path.len() == 0 {
        return Err(String::from("there is no file path provided"));
    }

    let content = fs::read_to_string(config.file_path)
    .map_err(|someting| { 
        eprintln!("This errored out over here. {}", someting);
        return format!("Some error occurred: {}", someting);
    })?;

    println!("contents: {}", content);
    let arr = search_case_insensitive(&config.query, &content);
    for &greps in arr.iter() {
        println!("{} --> {}", config.query, greps);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();
    for each_line in contents.lines() {
        if each_line.contains(query) {
            results.push(each_line);
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str,) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();
    let mut results = Vec::new();
    for one_line in contents.lines() {
        // Since to_lowercase generates new string and we are returning references 
        // that will outlive the lowercase because it's destroyed once fn completes
        if one_line.to_lowercase().contains(&lowercase_query) { 
            results.push(one_line);
        }
    }

    return results;
}
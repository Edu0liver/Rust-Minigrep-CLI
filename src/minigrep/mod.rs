
pub mod minigrep {
    use std::error::Error;
    use std::fs;
    use std::env;
    use std::collections::HashMap;

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;

        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };
        
        for (count, line) in results{
            println!("Line: {} -> {}", count, line)
        }

        Ok(())
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> HashMap<u32, &'a str> {
        let mut results: HashMap<u32, &str> = HashMap::new();
        let mut count: u32 = 0;
    
        for line in contents.lines() {
            count += 1;

            if line.contains(query){
                results.insert(count, line);
            }
        }
    
        results
    }
    
    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> HashMap<u32, &'a str> {
        let query = query.to_lowercase();
        let mut results: HashMap<u32, &str> = HashMap::new();
        let mut count: u32 = 0;
    
        for line in contents.lines() {
            count += 1;
            
            if line.to_lowercase().contains(&query){
                results.insert(count, line);
            }
        }
    
        results
    }

    pub struct Config {
        pub query: String,
        pub filename: String,
        pub case_sensitive: bool
    }

    impl Config {
        pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
            args.next();

            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string!")
            } ;

            let filename = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file name!")
            } ;

            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
            Ok(Config { query, filename, case_sensitive })
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::minigrep::minigrep::search;
    use crate::minigrep::minigrep::search_case_insensitive;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";

        assert_eq!(HashMap::from([(2, "safe, fast, productive.")]), search(query, contents))
    }
    
    fn case_sensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(HashMap::from([
            (1, "Rust:,"),
            (4, "Trust me.")
        ]), search_case_insensitive(query, contents))
    }
}
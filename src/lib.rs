use std::fs;
use std::env;
use std::error::Error;

pub struct EnvConfig {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl EnvConfig {
  pub fn build(args: &[String]) -> Result<EnvConfig, &'static str> {
      if args.len() < 3 {
        return Err("not enough arguments");
      }

      let query = args[1].clone();
      let file_path = args[2].clone();
      let ignore_case = env::var("IGNORE_CASE").is_ok();

      Ok(EnvConfig { query, file_path, ignore_case })
  }
}

pub fn run(config: EnvConfig) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  for line in results {
    println!("{line}");
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
      .lines()
      .filter(|line| line.contains(query))
      .collect()
}


pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  let query = query.to_lowercase();
  contents.lines().filter(|line|line.to_lowercase().contains(&query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

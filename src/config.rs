use std::env::{self};

#[derive(Debug, Default)]

pub struct Config {
    pub db_file_path: String,
}

impl Config {
    pub fn new() -> Self {
        let db_file_path =
            env::var("DATABASE_FILEPATH").expect("Missing required ENV DATABASE_FILEPATH");
        Config { db_file_path }
    }
}

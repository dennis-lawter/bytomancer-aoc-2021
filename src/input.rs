use serde_json::json;
use std::collections::HashMap;
use serde::Serialize;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use dotenv;
use reqwest::Url;

pub fn get_input_as_string(input_url: &String) -> String {
    match get_file_path_from_cache(input_url) {
        Some(path) => match get_input_as_string_from_cache(&path) {
            Ok(result) => result,
            Err(_) => get_input_as_string_from_site(input_url),
        },
        None => get_input_as_string_from_site(input_url),
    }
}

fn get_input_as_string_from_cache(path: &String) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

const CACHE_FILENAME: &str = "cache.json";

fn create_new_cache_file() -> File {
    let mut file = File::create(CACHE_FILENAME).expect("Could not create cache file.");
    file.write_all(b"{}")
        .expect("Could not write to new cache file.");
    File::open(CACHE_FILENAME).expect("Error opening new cache file.")
}

fn write_new_input_locally_and_cache(url: &String, input: &String) -> Result<(), std::io::Error> {
    let md5checksum = md5::compute(input);
    let path = format!("{}{:x}{}", "_cache/", md5checksum, ".txt");
    let mut file = File::create(path.clone())?;
    file.write_all(input.as_bytes())?;

    let cache_file = File::options()
        .read(true)
        .write(true)
        .open(CACHE_FILENAME)?;
    let reader = BufReader::new(cache_file);
    let mut cache_records: HashMap<String, String> =
        serde_json::from_reader(reader).expect("Could not read cache file.");
    cache_records.insert(url.to_string(), path);
    cache_file.write_all(serde_json::to_string(&cache_records))?;

    Ok(())
}

fn get_file_path_from_cache(input_url: &String) -> Option<String> {
    let file: File;
    let file_open = File::open(CACHE_FILENAME);
    match file_open {
        Ok(f) => file = f,
        Err(_) => file = create_new_cache_file(),
    }

    let reader = BufReader::new(file);
    let cache_records: HashMap<String, String> =
        serde_json::from_reader(reader).expect("Could not read cache file.");

    cache_records.get(input_url).clone().cloned()
}

fn get_input_as_string_from_site(input_url: &String) -> String {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    const KEY: &str = "SESSION";
    let session = dotenv::var(KEY).unwrap();
    let cookie = format!("session={}", session);
    let url = input_url.parse::<Url>().unwrap();

    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let res = client.get(url).header("cookie", cookie).send().unwrap();

    res.text().unwrap()
}

extern crate yaml_rust2;
use yaml_rust2::{YamlLoader, YamlEmitter};
use std::fs::File;
use std::io::prelude::*;
use std::io;

pub fn load_config(path: &str) -> Vec<yaml_rust2::Yaml> {
    let mut handler: File = File::open(path).unwrap();
    let mut content = String::new();
    handler.read_to_string(&mut content).unwrap();
    let yaml: Vec<yaml_rust2::Yaml> = YamlLoader::load_from_str(&content).unwrap();
    return yaml
}

pub fn get_language(config: &Vec<yaml_rust2::Yaml>) -> Option<&str> {
    config[0]["options"]["language"].as_str()
}





extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use std::fs::File;
use std::io::prelude::*;

fn load_config(path: &str) -> Result<String> {
    let handler: String = File::open(path)?;
    let mut content = String::new();
    handler.read_to_string(&mut content)?;
    return Ok(content)
}


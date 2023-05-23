use crate::ObjectType;
use crate::Parser;
use super::Result;
use std::{path::Path};

pub(crate) fn load_object_type_with(path: &str) -> Result<ObjectType> {
    let path = Path::new(&path);
    let parser = Parser::build(path)?;
    let object = parser.parse()?;

    Ok(object)
}

pub(crate) fn exit_with_help_string(string: &str) -> ! {
    eprintln!("{string}");
    std::process::exit(1)
}

pub(crate) fn exit_normally_with_help_string(string: &str) -> ! {
    println!("{string}");
    std::process::exit(0)
}
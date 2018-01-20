//! more at https://docs.npmjs.com/files/package.json

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate semver;

use std::error::Error;
use std::fs::File;
use std::path::Path;

pub mod package_json;

pub use package_json::PackageJson;


pub fn from_file<P: AsRef<Path>>(path: P) -> Result<PackageJson, Box<Error>> {
    let file = File::open(path)?;
    let u = serde_json::from_reader(file)?;
    Ok(u)
}


//! more at https://docs.npmjs.com/files/package.json

extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;

pub use url::Url;

pub mod package_json;
pub use package_json::PackageJson;
mod version_requirement;
pub use version_requirement::VersionRequirement;

pub fn from_file<P: AsRef<OsStr> + Sized>(given_path: P) -> Result<PackageJson, Box<Error>> {
    let path = Path::new(&given_path);
    let file = if path.is_dir() {
        File::open(path.join("package.json"))?
    } else {
        File::open(path)?
    };
    let u = serde_json::from_reader(file)?;
    Ok(u)
}

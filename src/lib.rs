//! more at https://docs.npmjs.com/files/package.json
#![feature(conservative_impl_trait)]
extern crate semver;
extern crate serde;
#[macro_use]
extern crate log;
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

/// Opens a given file or the `package.json` if a folder is given.
pub fn open<P: AsRef<OsStr> + Sized>(given_path: P) -> Result<PackageJson, Box<Error>> {
    let path = Path::new(&given_path);
    let file = if path.is_dir() {
        let file_path = path.join("package.json");
        debug!("opening {:?}", file_path);
        File::open(file_path)?
    } else {
        debug!("opening {:?}", path);
        File::open(path)?
    };
    let mut u: PackageJson = serde_json::from_reader(file)?;
    u.__path = Some(path.to_owned());
    Ok(u)
}

//! more at https://docs.npmjs.com/files/package.json

use semver::Version;
use serde_json;
use url::Url;

use std::collections::HashMap;
use std::path::PathBuf;

use version_requirement::VersionRequirement;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageJson {
    pub name:         Option<String>,
    pub version:      Option<Version>,
    pub homepage:     Option<Url>,
    pub email:        Option<String>,
    pub description:  Option<String>,
    pub main:         Option<String>,
    pub bin:          Option<Bin>,
    pub bugs:         Option<Bugs>,
    pub man:          Option<Man>,
    pub typings:      Option<String>,
    pub license:      Option<License>,
    pub licenses:     Option<Vec<License>>,
    pub keywords:     Option<Vec<String>>,
    pub author:       Option<Person>,
    pub contributors: Option<Vec<Person>>,
    pub config:       Option<HashMap<String, serde_json::Value>>,
    pub files:        Option<Vec<String>>,
    pub repository:   Option<Repository>,
    pub scripts:      Option<HashMap<String, String>>,
    pub workspaces:   Option<Vec<PathBuf>>,

    pub dependencies: Option<Dependencies>,

    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: Option<Dependencies>,

    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<Dependencies>,

    #[serde(skip_serializing, skip_deserializing)]
    #[doc(hidden)]
    pub __path: Option<PathBuf>,
}

impl PackageJson {
    pub fn path(&self) -> Option<&PathBuf> {
        self.__path.as_ref()
    }
}

pub type Dependencies = HashMap<String, VersionRequirement>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Repository {
    Url(Url),
    Other(String),
    Structured(StructuredRepository),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredRepository {
    #[serde(rename = "type")]
    typ: Option<String>,
    url: Option<Url>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bin {
    One(String),
    Many(HashMap<String, String>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bugs {
    email: Option<String>,
    url:   Option<Url>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Man {
    One(String),
    Many(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Person {
    Name(String),
    Person {
        #[serde(rename = "type")]
        name: Option<String>,
        email: Option<String>,
        url: Option<Url>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum License {
    Type(String),
    License {
        #[serde(rename = "type")]
        typ: Option<String>,
        url: Option<String>,
    },
}

use std::iter::Iterator;

impl PackageJson {
    // Produces a list of all dependencies including dev and peer dependencies
    pub fn all_dependencies<'a>(&'a self) -> impl Iterator<Item = (&'a String, &'a VersionRequirement)> {
        let dependencies = self.dependencies.iter().flat_map(|deps| deps);
        let dev_dependencies = self.dev_dependencies.iter().flat_map(|deps| deps);
        let peer_dependencies = self.peer_dependencies.iter().flat_map(|deps| deps);

        dependencies
            .chain(dev_dependencies)
            .chain(peer_dependencies)
    }
}

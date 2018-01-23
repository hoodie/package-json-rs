//! more at https://docs.npmjs.com/files/package.json

use semver::{Version, VersionReq};
use serde_json;
use url::Url;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    pub workspaces:   Option<Vec<String>>,

    pub dependencies: Option<Dependencies>,

    #[serde(rename = "peerDependencies")] pub peer_dependencies: Option<Dependencies>,

    #[serde(rename = "devDependencies")] pub dev_dependencies: Option<Dependencies>,
}

pub type Dependencies = HashMap<String, VersionRequirement>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VersionRequirement {
    VersionReq(VersionReq),
    Other(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Repository {
    Url(Url),
    Other(String),
    Structured(StructuredRepository),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredRepository {
    #[serde(rename = "type")] typ: Option<String>,
    url: Option<Url>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bin {
    One(String),
    Many(HashMap<String, String>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Bugs {
    email: Option<String>,
    url:   Option<Url>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Man {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Person {
    Name(String),
    Person {
        #[serde(rename = "type")] name: Option<String>,
        email: Option<String>,
        url: Option<Url>,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum License {
    Type(String),
    License {
        #[serde(rename = "type")] typ: Option<String>,
        url: Option<String>,
    },
}

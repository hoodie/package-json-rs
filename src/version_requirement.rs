use semver::VersionReq;

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VersionRequirement {
    VersionReq(VersionReq),
    Other(String),
}

impl VersionRequirement {
    pub fn version_req(self) -> Option<VersionReq> {
        match self {
            VersionRequirement::VersionReq(inner) => Some(inner),
            VersionRequirement::Other(_) => None,
        }
    }

    pub fn compatible_with(&self, other: &VersionRequirement) -> bool {
        unimplemented!()
        // match (self, other) {
        //    (&VersionRequirement::VersionReq(ref a), &VersionRequirement::VersionReq(ref b)) => a == b,
        //    (&VersionRequirement::Other(ref a), &VersionRequirement::Other(ref b)) => false,
        //    (&VersionRequirement::VersionReq(_), &VersionRequirement::Other(_)) => false,
        //    (&VersionRequirement::Other(_), &VersionRequirement::VersionReq(_)) => false,
        //}
    }
}

impl fmt::Display for VersionRequirement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VersionRequirement::VersionReq(ref inner) => write!(f, "{}", inner),
            VersionRequirement::Other(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl PartialOrd for VersionRequirement {
    fn partial_cmp(&self, other: &VersionRequirement) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for VersionRequirement {
    fn cmp(&self, other: &VersionRequirement) -> Ordering {
        match (self, other) {
            (&VersionRequirement::VersionReq(ref a), &VersionRequirement::VersionReq(ref b)) => Ord::cmp(a, b),
            (&VersionRequirement::Other(ref a), &VersionRequirement::Other(ref b)) => Ord::cmp(a, b),
            (&VersionRequirement::VersionReq(_), &VersionRequirement::Other(_)) => Ordering::Less,
            (&VersionRequirement::Other(_), &VersionRequirement::VersionReq(_)) => Ordering::Greater,
        }
    }
}

impl Eq for VersionRequirement {}

impl PartialEq for VersionRequirement {
    fn eq(&self, other: &VersionRequirement) -> bool {
        match (self, other) {
            (&VersionRequirement::VersionReq(ref a), &VersionRequirement::VersionReq(ref b)) => a == b,
            (&VersionRequirement::Other(ref a), &VersionRequirement::Other(ref b)) => a == b,
            (&VersionRequirement::VersionReq(_), &VersionRequirement::Other(_)) => false,
            (&VersionRequirement::Other(_), &VersionRequirement::VersionReq(_)) => false,
        }
    }
}

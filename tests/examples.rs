extern crate package_json;

#[test]
fn npm() {
    package_json::from_file("./tests/npm.json").unwrap();
}

#[test]
fn yarn() {
    package_json::from_file("./tests/yarn.json").unwrap();
}

#[test]
fn sinon() {
    package_json::from_file("./tests/sinon.json").unwrap();
}

#[test]
fn chai() {
    package_json::from_file("./tests/chai.json").unwrap();
}

#[test]
fn express() {
    package_json::from_file("./tests/express.json").unwrap();
}

#[test]
fn lodash() {
    package_json::from_file("./tests/lodash.json").unwrap();
}

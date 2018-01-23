extern crate package_json;
use package_json::Url;

#[test]
fn npm() {
    let package = package_json::from_file("./tests/npm.json").unwrap();
    println!("{:#?}: {:#?}", package.name, package.homepage);
}

#[test]
fn yarn() {
    let package = package_json::from_file("./tests/yarn.json").unwrap();
    println!("{:#?}: {:#?}", package.name, package.homepage);
}

#[test]
fn sinon() {
    let package = package_json::from_file("./tests/sinon.json").unwrap();
    println!("{:#?}: {:#?}", package.name, package.homepage);
}

#[test]
fn chai() {
    let package = package_json::from_file("./tests/chai.json").unwrap();
    println!("{:#?}: {:#?}", package.name, package.homepage);
    println!("{:?}", package.bugs);
}

#[test]
fn express() {
    let package = package_json::from_file("./tests/express.json").unwrap();
    println!("{:#?}: {:#?}", package.name, package.homepage);
}

#[test]
fn lodash() {
    package_json::from_file("./tests/lodash.json").unwrap();
}

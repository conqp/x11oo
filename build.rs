extern crate pkg_config;

use std::env;

const FEATURES: [&str; 1] = ["xfixes"];

fn main() {
    for feature in FEATURES {
        let var = format!("CARGO_FEATURE_{}", feature.to_uppercase().replace('-', "_"));

        if env::var_os(var).is_none() {
            continue;
        }

        pkg_config::Config::new().probe(feature).unwrap();
    }
}

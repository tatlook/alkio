use std::env;

use copy_to_output::copy_to_output;

fn main() {
    copy_to_output("example", &env::var("PROFILE").unwrap()).expect("Canno copy anything");
}

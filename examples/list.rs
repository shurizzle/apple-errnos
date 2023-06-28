extern crate apple_errno;

use apple_errno::*;

pub fn main() {
    for errno in Errno::iter() {
        println!(
            "{}: {}",
            errno.name().unwrap(),
            errno.description().unwrap()
        );
    }
}

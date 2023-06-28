extern crate apple_errnos;

use apple_errnos::*;

pub fn main() {
    for errno in Errno::iter() {
        println!(
            "{}: {}",
            errno.name().unwrap(),
            errno.description().unwrap()
        );
    }
}

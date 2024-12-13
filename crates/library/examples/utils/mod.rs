#![allow(unused)]

use kutil_cli::debug::*;

pub fn heading(heading: &str, first: bool) {
    let theme = Theme::default();
    if !first {
        println!();
    }
    println!("{}:", theme.heading(heading));
}

use std::collections::HashMap;
// use std::fmt;
// use std::io;
use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

fn f1() -> Result {1}

fn f2() -> IoResult {2}
#![allow(unused)] // silence unused warnings while exploring (to comment out)

fn main() {
    let mut args = std::env::args().skip(1);  // skip 1 because it is the command it self
    let key = args.next().unwrap();  // unwrap will crash the program if no more there
    let value = args.next().unwrap();

    // let db = Database::new();
    let contents = format!("{}\t{}\t\n", key, value);
    std::fs::write("kv.db", contents);
    println!("key {} , value {}", key, value) // println is a macro use ! for macros
}

use std::collections::HashMap;

struct Database {
    inner: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let contents = std::fs::read_to_string("kv.db")?;

        let mut inner = HashMap::new();

        for line in contents.lines() {
            let chunks: Vec<&str> = line.split('\t').collect(); // Vector can grow on runtime
            if chunks.len() != 2 {
                todo!("Return Error");
            }
            let key = chunks[0];
            let value = chunks[1];
            inner.insert(key.to_owned(), value.to_owned());
        };
        Ok(Database {
            inner
        })
    }
}
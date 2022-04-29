use std::collections::HashSet;
use std::fs;
use std::env;

fn main() {
    let path = "data/disposable_email_blocklist.conf";
    let content= fs::read_to_string(path).unwrap();
    let lines = content.split("\n");
    let lines= lines.map(|line| line.to_owned());
    let blacklist: HashSet<String> = lines.into_iter().collect();
    let q = env::args().nth(1).expect("please provide a domain");
    if blacklist.contains(&q) {
        println!("hit");
    } else {
        println!("miss");
    }
}

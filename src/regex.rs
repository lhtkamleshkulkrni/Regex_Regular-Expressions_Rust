// regex

extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"\w{5}").unwrap();
    let text = "dcode";

    // println!("Found match? {}", re.is_match(text));

    // match re.captures (text) {
    //     Some (caps) => println! ("Found match: {}", caps.get(0).unwrap().as_str()),
    //     None => println! ("Could not find match...")

    match re.captures(text) {
        Some(caps) => println!("Found match: {}", &caps[0]),
        None => println!("Could not find match..."),
    }
}



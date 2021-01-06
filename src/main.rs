use std::{env, fs};

const SIGN_BEGIN: &str = "<!-- BEGIN LGBT-CN SIGNATURE -->";
const SIGN_END: &str = "<!-- END LGBT-CN SIGNATURE -->";
const COUNT_BEGIN: &str = "<!-- BEGIN LGBT-CN COUNT -->";
const COUNT_END: &str = "<!-- END LGBT-CN COUNT -->";

fn main() {
    let args: Vec<String> = env::args().collect();
    let content: String;
    if args.len() != 2 {
        eprintln!("[Error] Wrong number of arguments. Request 1, provided {}", args.len() - 1);
    }
    let target = args[1].clone();
    if target.starts_with("http") {
        unimplemented!("TODO: URL DOWNLOAD");
    } else {
        content = fs::read_to_string(&target).unwrap();
    }

    let content_vec: Vec<&str> = content.split("\n").collect();
    let count = get_sign_number(&content_vec);
    // let syn = sync_sign_number(&target, content_vec, count);
    println!("Count: {}", count);
}

fn get_sign_number(data: &Vec<&str>) -> i16 {
    let mut counter: i16 = -1;
    for line in data {
        match line.trim() {
            SIGN_BEGIN => counter = 0,
            SIGN_END => break,
            "" => continue,
            _ => {
                if counter < 0 {
                    continue;
                } else {
                    counter += 1;
                }
            }
        };
    };
    counter
}
use std::{env, fs};
use std::fs::File;
use std::io::Write;

const SIGN_BEGIN: &str = "<!-- BEGIN LGBT-CN SIGNATURE -->";
const SIGN_END: &str = "<!-- END LGBT-CN SIGNATURE -->";
const COUNT_BEGIN: &str = "<!-- BEGIN LGBT-CN COUNT -->";
const COUNT_END: &str = "<!-- END LGBT-CN COUNT -->";

fn main() {
    let args: Vec<String> = env::args().collect();
    let content: String;
    if args.len() != 2 {
        panic!("[Error] Wrong number of arguments. Request 1, provided {}", (args.len() - 1));
    }
    let target = args[1].clone();
    if target.starts_with("http") {
        unimplemented!("TODO: URL DOWNLOAD");
    } else {
        content = fs::read_to_string(&target).unwrap();
        let content_vec: Vec<&str> = content.split("\n").collect();
        let count = get_sign_number(&content_vec);
        sync_sign_number(&target, content_vec, count);
        println!("Count: {}", count);
    }
}

fn sync_sign_number(path: &str, data: Vec<&str>, count: i16) {
    let mut f = File::create(path).unwrap();
    let mut v: String = String::new();
    let mut is_under_count_block = false;
    for line in data {
        match line.trim() {
            COUNT_BEGIN => {
                v.push_str(COUNT_BEGIN);
                v.push_str("\n");
                v.push_str("已有 ");
                v.push_str(&count.to_string());
                v.push_str(" 人签署！\n");
                is_under_count_block = true;
            }
            COUNT_END => {
                v.push_str(COUNT_END);
                v.push_str("\n");
                is_under_count_block = false;
            }
            _ => {
                if !is_under_count_block {
                    v.push_str(line);
                    v.push_str("\n");
                }
            }
        };
    }
    // println!("{:}", v);
    f.write_all(v.as_bytes()).unwrap();
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
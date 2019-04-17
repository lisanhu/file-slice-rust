use std::str::from_utf8;

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 { usage(&args); return; }

    use std::fs;
    let content = fs::read_to_string(args[1].as_str()).unwrap();
    let start_pos: usize = args[2].parse().unwrap();
    let length: usize = args[3].parse().unwrap();
    let end = start_pos + length;

    eprintln!("start_pos: {}, length: {}", start_pos, length);
    println!("{}", from_utf8(content.as_bytes()[start_pos..end].as_ref()).unwrap());
}

fn usage(args: &[String]) {
    let command = args[0].as_str();
    eprintln!("Usage:");
    eprintln!("\t{} <file-path> <start-pos> <length>", command);
}
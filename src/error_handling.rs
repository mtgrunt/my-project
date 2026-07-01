use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

pub fn demo() {
    panic_concept();
    result_match();
    matching_error_kinds();
    unwrap_and_expect();
    question_mark_operator();
}

fn panic_concept() {
    // panic!("crash and burn") would print the message and unwind/abort.
    // Indexing past the end of a Vec also triggers a panic at runtime.
    let v = vec![1, 2, 3];
    match v.get(99) {
        Some(val) => println!("[9.1] v.get(99) = {val}"),
        None => println!("[9.1] v.get(99) = None  (v[99] would panic with index out of bounds)"),
    }
}

fn result_match() {
    match File::open("hello.txt") {
        Ok(f) => println!("[9.2] Opened hello.txt: {f:?}"),
        Err(e) => println!("[9.2] File::open(\"hello.txt\") -> Err: {e}"),
    }
}

fn matching_error_kinds() {
    match File::open("hello.txt") {
        Ok(_) => println!("[9.2] Opened hello.txt"),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            println!("[9.2] ErrorKind::NotFound — would create file here");
        }
        Err(e) => println!("[9.2] Other error opening file: {e}"),
    }
}

fn unwrap_and_expect() {
    // unwrap() and expect() return the inner Ok value or panic on Err.
    // Demonstrated here on a known-Ok value so the demo doesn't abort.
    let ok_val: Result<i32, &str> = Ok(42);
    let n = ok_val.unwrap();
    println!("[9.2] Ok(42).unwrap() = {n}");

    let ok_val: Result<i32, &str> = Ok(7);
    let n = ok_val.expect("value should always be present");
    println!("[9.2] Ok(7).expect(...) = {n}");

    println!("[9.2] On an Err value, both unwrap() and expect() panic with the error.");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_short() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn question_mark_operator() {
    match read_username_from_file() {
        Ok(name) => println!("[9.2] Username (long form): {name}"),
        Err(e) => println!("[9.2] ? propagated error (long form): {e}"),
    }

    match read_username_short() {
        Ok(name) => println!("[9.2] Username (fs::read_to_string shortcut): {name}"),
        Err(e) => println!("[9.2] ? propagated error (shortcut): {e}"),
    }
}

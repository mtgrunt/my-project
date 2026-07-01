pub fn demo() {
    creating();
    updating();
    indexing_and_slicing();
    iterating();
}

fn creating() {
    let _empty = String::new();
    let s = "initial contents".to_string();
    println!("[8.2] to_string(): \"{s}\"");
    let s = String::from("hello");
    println!("[8.2] String::from(): \"{s}\"");
}

fn updating() {
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("[8.2] push_str (s2 still usable: \"{s2}\"): \"{s}\"");

    let mut s = String::from("lo");
    s.push('l');
    println!("[8.2] push single char: \"{s}\"");

    // + operator: takes ownership of s1, appends &s2
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("[8.2] s1 + &s2 (s1 moved): \"{s3}\"");

    // format! doesn't take ownership of any argument
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("[8.2] format!: \"{s}\" (s1/s2/s3 still owned)");
}

fn indexing_and_slicing() {
    // Rust strings are UTF-8; indexing by byte number is allowed via slices
    // but indexing by s[i] is not — the compiler prevents it because
    // a char may be 1–4 bytes and returning a single byte would rarely be
    // what you want.
    let hello = "Здравствуйте"; // each Cyrillic char is 2 bytes
    let first_four_bytes = &hello[0..4]; // bytes 0..4 = first two chars "Зд"
    println!("[8.2] &hello[0..4] (4 bytes of Cyrillic): \"{first_four_bytes}\"");
}

fn iterating() {
    print!("[8.2] \"Зд\".chars(): ");
    for c in "Зд".chars() {
        print!("{c} ");
    }
    println!();

    print!("[8.2] \"hi\".bytes(): ");
    for b in "hi".bytes() {
        print!("{b} ");
    }
    println!();
}

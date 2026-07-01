#[derive(Debug)]
pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn demo() {
    creating();
    updating();
    reading_elements();
    iterating();
    enum_multiple_types();
}

fn creating() {
    let _v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    println!("[8.1] vec! macro shorthand: {v:?}");
}

fn updating() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("[8.1] After push x4: {v:?}");
}

fn reading_elements() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("[8.1] v[2] = {third}");

    match v.get(2) {
        Some(t) => println!("[8.1] v.get(2) = Some({t})"),
        None => println!("[8.1] v.get(2) = None"),
    }

    match v.get(100) {
        Some(t) => println!("[8.1] v.get(100) = Some({t})"),
        None => println!("[8.1] v.get(100) = None  (v[100] would panic!)"),
    }
}

fn iterating() {
    let v = vec![100, 32, 57];
    let mut out = String::new();
    for i in &v {
        out.push_str(&format!("{i} "));
    }
    println!("[8.1] Immutable iter: {}", out.trim_end());

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("[8.1] After mutable iter (+50): {v:?}");
}

fn enum_multiple_types() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("[8.1] Enum stores multiple types in one Vec:");
    for cell in &row {
        println!("      {cell:?}");
    }
}

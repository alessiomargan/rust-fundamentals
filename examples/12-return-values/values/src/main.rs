
fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    // This will not compile!
    let result = parts.get(field);
    result.expect("handle None option").to_string()
}

fn main() {
    let text = "";
    let chunk = split_string(text.to_string(), ',', 1);
    //let chunk = split_string("hello,world".to_string(), ',', 1);
    println!("Split string: {}", chunk);
}

fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];
    let mut x = rust_group.iter().peekable();

    if let Some(r) = x.peek() {
        println!("{}", r);
    }
    if let Some(r) = x.next() {
        println!("{}", r);
    }
    if let Some(r) = x.peek() {
        println!("{}", r);
    }
    if let Some(r) = x.peek() {
        println!("{}", r);
    }
}

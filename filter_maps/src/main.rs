fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];
    let z = rust_group.iter().filter_map(|m| {
        if m.len() > 5 {
            return Some(String::new() + m + " kind ");
        }
        None
    });
    for m1 in z {
        println!("{}", m1);
    }
}

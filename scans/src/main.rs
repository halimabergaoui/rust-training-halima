fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];
    let iter = rust_group.iter().scan(" Sqoin members are ".to_string(),
    | s , item| {
        s.push_str(item);
        s.push_str(" ");
         Some(s.clone())
    }
);
for m1 in iter {
    println!("{}", m1);
}}

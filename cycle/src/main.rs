fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];
    for (i,j) in (&rust_group).into_iter().cycle().enumerate() {
        println!("{} ", j );
        if i > 10 {
            break 
        }
    }
}

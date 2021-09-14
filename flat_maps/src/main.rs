fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];
    let z = rust_group.iter().flat_map(|m| {
        if m.len() > 5 {
            return Some([m, " kind "]);
        }
        None
    });
    for member in z {
        print!("[");
        for m1 in member {
            print!("{} ", m1);
        }
        print!("]");
    }
}

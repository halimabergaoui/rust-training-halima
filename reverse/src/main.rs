fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];
   
    let mut iter = (&rust_group).into_iter();
    
     
    while let Some(v) = iter.next_back() {
        println!("{}", v);
    }
    let mut iter = (&rust_group).into_iter().rev();
    
    println!("---------");
     
    while let Some(v) = iter.next() {
        println!("{}", v);
    }

}

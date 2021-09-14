fn main() {
    let rust_group = vec![ "Bacem" , "Karima" , "Amal" , "Jawaher" , "Halima" , "Khouloud"];
    println!("********************vector********************");
    for member in &rust_group {
         println!("{}" ,  member);
     }
 
     let mut iter = (&rust_group).into_iter();
     
     let value = iter.next();
     let real_value = match value {
         Some(v) => v,
         None => ""
     };
     println!("********************value from next********************");
     println!("{}" , real_value);
 
     println!("******************** iterator while value exist ********************");
     while let Some(v) = iter.next() {
         println!("{}" , v);
     }
}

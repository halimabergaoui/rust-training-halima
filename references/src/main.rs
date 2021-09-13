fn main() {
    let mut v3 = vec![10u32];
    {
      // can have many refs that will be destroyed out of the brackets
      let refv31=&v3;
      let refv32=&v3; 
      println!("{} , {}", refv32[0],refv31[0]);
    }
    // can have only one mut ref no other refs are allowed
    let refv3 = &mut v3;
    refv3.push(11u32);
    println!("{} ", &v3[1]);
}

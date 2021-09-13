fn main() {
    let tup: (u32, [u8; 4]) = {
        println!("assigning tuple");
        (4u32, [1u8, 2, 3, 4])
    };
    println!("{}", tup.0);

    let z = (10,);

    println!("{}", z.0);
}

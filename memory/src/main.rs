fn main() {
    // copy type
    let m1: u32 = 8u32;

    let m2 = m1;

    println!(" {} {} ", m1, m2);

    //non copy type
    let mut b1 = Box::new(10u32);
    let b2 = b1;
    b1 = Box::new(11u32);

    print_box(b1);
    b1 = print_box(b2);
    print_box(b1);
}

fn print_box(b: Box<u32>) -> Box<u32> {
    println!("{} ", b);
    b
}

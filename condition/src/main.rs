fn main() {
     let mut a = 12;

    let b = 18;

    if a < b {
        println!("a is less than b");
    } else {
        panic!(" There is a problem here we have to exit");
    }

    while a < b {
        print!("{} " , a);
        a += 1;
    } 

    
}

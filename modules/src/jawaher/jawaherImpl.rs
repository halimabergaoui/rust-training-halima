use crate::karima;
pub fn say_hello() {
    println!("I am Jawaher");
    println!("Jawaher is calling Karima");
    karima::say_hello();
    println!("Jawaher is calling Halima");
    sqoinlib::halima::say_hello();
    println!("Jawaher is calling Halima from lib");
    halimalib::halima::say_hello();
}
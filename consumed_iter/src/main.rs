fn main() {
    let numbers:Vec<u32> = vec![ 1 , 2 , 5 , 7 , 10];

    let  numbes_iter = (&numbers).into_iter();
    
    println!(" sum = {}" , numbes_iter.sum::<u32>() );

    let  numbes_iter = (&numbers).into_iter();

    println!(" product = {}" , numbes_iter.product::<u32>() );

    let  numbes_iter = (&numbers).into_iter();

    println!(" count = {}" , numbes_iter.count() );

    let  numbes_iter = (&numbers).into_iter();

    println!(" min = {}" , numbes_iter.min().unwrap() );

    let mut numbes_iter = (&numbers).into_iter();

    println!(" any  > 8 = {}" , numbes_iter.any(|m| m > &8u32 ) );

    let mut numbes_iter = (&numbers).into_iter();

    println!(" all  > 8 = {}" , numbes_iter.all(|m| m > &8u32 ) );

    let mut numbes_iter = (&numbers).into_iter();

    println!(" position  de  7 = {}" , numbes_iter.position(|m| m > &5000u32 ).unwrap_or(0) );
let  numbes_iter = (&numbers).into_iter();

    println!(" fold   = {}" , numbes_iter.fold(10, |x , y| x+y ) );
    
    let mut  numbes_iter = (&numbers).into_iter();

    println!(" nth   = {}" , numbes_iter.nth(3 ).unwrap_or(&0u32) );

    let mut  numbes_iter = (&numbers).into_iter();

    println!(" last   = {}" , numbes_iter.last( ).unwrap_or(&0u32) );

    let mut numbes_iter = (&numbers).into_iter();

    println!(" position  de  7 = {}" , numbes_iter.find(|m| m > &&5u32 ).unwrap_or(&0u32) );

}

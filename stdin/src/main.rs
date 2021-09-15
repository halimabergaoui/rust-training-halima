
use std::io::{prelude::*, stdin, Lines, Stdin, StdinLock};

fn main() {
    //println!("Please type a number");

    let stdin: Stdin = stdin();
    /*  let reader:StdinLock = stdin.lock();
    let lines: Lines::<StdinLock> = reader.lines();

    for s in lines {
        println!(" lines {} " , s.unwrap_or("error".to_string()));
    }*/
    let mut buf = String::new();
    let r = stdin.read_line(&mut buf);

    match r {
        Ok(x) => println!("value read is {}" , buf) ,
        Err(_) => panic!("  could not read ")

    }


    }
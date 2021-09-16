use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write as IoWrite;
use std::io::Cursor;
use std::process::{Command,Stdio};

fn main() {


    let mut w = Vec::new();
    let mut s = String::new();
    // writeln!(&mut w);
    writeln!(&mut s, "{} {} {}", "abcg", 123u32, "karima");
    writeln!(&mut w, "s = {:?}", s);
    println!(" {} ", s);
    println!(" {:?} ", w);

    writeln!(io::stdout(), "error : world not helloable");

    let mut writer = File::create("newfile.txt").unwrap();
    let mut writerbuf = File::create("newfilebuf.txt").unwrap();
    let mut buffer = BufWriter::new(writerbuf);
    writer.write(b"jawaher");
    buffer.write(b"khoukha");
    writer.write_all(b"Karimaa");

    let res = buffer.flush();
    match res {
        Ok(r) => println!("res {:?}", r),
        Err(e) => println!("err {}", e),
    }

    println!("--------cursor------------");
    let mut buff= Cursor::new(vec![1,2,3,4]);
    let buf1=buff.clone();
    //get object itself
    let vec=buf1.into_inner();
    println!("vec {:?} ",vec);
    //get ref
    let refvec=buff.get_ref();
    println!("ref {:?} ",refvec);
    //get mutable ref
    let mutvec=buff.get_mut();
    println!("mut {:?} ",mutvec);
    

    println!("-------command---------");
 Command::new("ls")
    .spawn()
    .expect("failed to execute process");

}
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write as IoWrite;

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
}
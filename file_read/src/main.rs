use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader,Lines,Split};
use std::io::SeekFrom;
use std::fs::OpenOptions;

fn main() {
    // --snip--
    println!("------- Reading a file--------- ");
    let filename = "poem.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let contents1 = fs::read(filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
    println!("len {}", contents1.len());
    for i in contents1.iter() {
        println!("{}", i);
    }
    let mut file2 = File::open("poem.txt").unwrap();
    let mut buffer = Vec::new();
    let f = file2.read_to_end(&mut buffer).unwrap();
    println!("Last file :\n{}", f);
    for i in buffer.iter() {
        println!("buffer {}", i);
    }
    println!("---------read exact -----");
    let mut file3 = File::open("poem.txt").unwrap();
    let mut buffer = [0; 20];
    let f = file3.read_exact(&mut buffer);
    println!("read exact {:?}", f);
    for i in buffer.iter() {
        println!("buffer {}", i);
    }
    println!("---------bytes -----");
    let f = file3.bytes();
    println!("{:?}", f);

    println!("---------chars -----");
    let mut file3 = File::open("poem.txt").unwrap();
    let mut contents = String::new();
    file3
        .read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    for (i, j) in contents.chars().enumerate() {
        println!("char {}:{}", i, j);
    }

    println!("---------chain -----");
    let file1 = File::open("poem.txt").unwrap();
    let file2 = File::open("test.txt").unwrap();
    let mut newfile = (&file1).chain(file2);
    let mut contents = String::new();
    newfile
        .read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    println!("new file :\n{}", contents);
    println!("---------take -----");
    let file1 = File::open("poem.txt").unwrap();
    let mut reader = file1.take(70);
    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    println!("take :\n{}", contents);

    println!("------- Buffered Readers--------- ");
    println!("------- reade line--------- ");
    let file = File::open("poem.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let len = reader.read_line(&mut line);
    println!(" ok {:?}", len);
    println!("line is  {:?}", line);

    println!("------- lines--------- ");

    let len: Lines<BufReader<File>> = reader.lines();
    for (i, j) in len.enumerate() {
        println!("line {}:{}", i, j.unwrap());
    }
    println!("------- read unitl--------- ");
    let file = File::open("poem.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf: Vec<u8> = vec![];
    let r = reader.read_until(b'a', &mut buf);
    println!("buf is  {:?}", buf);
    match r {
        Ok(v) => {
            println!("{}", v);
        }
        Err(e) => {
            println!("error :{}", e);
        }
    }
    println!("------- split--------- ");
    let r: Split::<BufReader::<File>>= reader.split(b'a');
    for (i, j) in r.enumerate() {
        match j {
            Ok(v) => {
                println!("{:?}", v);
            }
            Err(e) => {
                println!("error :{}", e);
            }
        }
    }

    println!("-----------seek---------");
    let mut file2 = File::open("poem.txt").unwrap(); 
    file2.seek(SeekFrom::Start(10));
    //file2.seek(SeekFrom::Current(-4));
    //file2.seek(SeekFrom::End(-8));
   let mut buffer = Vec::new();
     let f = file2.read_to_end(&mut buffer);
 
     for (i,j) in buffer.iter().enumerate() {
         println!(" {}  {}", i,j);
     }  

     println!("-----------test---------");
     let log = OpenOptions::new().append(true).open("newfile.txt");
     println!("{:?}",log);

    let fl = OpenOptions::new().write(true).create_new(true).open("newfile3.txt");
    println!("{:?}",fl);

}

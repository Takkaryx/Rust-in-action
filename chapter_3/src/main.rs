#![allow(unused_variables)]

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum Filestate {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: Filestate,
}

impl File {
    #![allow(dead_code)]
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: Filestate::Closed,
        }
    }
    fn new_with_data(name: &str, data: Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != Filestate::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

impl Display for Filestate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Filestate::Open => write!(f, "OPEN"),
            Filestate::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = Filestate::Open;
    Ok(f)
}
fn close(mut f: File) -> Result<File, String> {
    f.state = Filestate::Closed;
    Ok(f)
}

fn main() {
    let mut f5 = File::new_with_data("5.txt", vec![114, 117, 115, 116, 33]);

    let mut buffer: Vec<u8> = vec![];

    if f5.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f5 = open(f5).unwrap();
    let f5_length = f5.read(&mut buffer);
    f5 = close(f5).unwrap();

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f5);
    println!("{:?} is {:?} bytes long", &f5.name, f5_length);
    println!("{}", text);
}

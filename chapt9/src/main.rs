use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::io::{ErrorKind, Read};

fn main() {

}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        // Note return clause here
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn empty_result() -> Result<(), Box<dyn Error>> {
    _ = File::open("hello.txt")?;

    Ok(())
}

fn read_to_string(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn read_username_from_file_better() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_even_better() -> Result<String, io::Error> {
    // Chain calls
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn unwrap_or_expect(filename: &str) {
    _ = File::open(filename).unwrap();
    _ = File::open(String::from(" ") + filename).expect("error message here");
}

fn open_or_create_file(filename: &str) -> File {
    match File::open(filename) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(e) {
                Ok(fc) => fc,
                Err(e) => panic!("Проблема с созданием файла: {:?}", e),
            },
            other_error => panic!("Проблема с открытием файла: {:?}", other_error),
        },
    }
}

fn open_or_create_file_better(filename: &str) -> File {
    File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            return File::create(filename)
                .unwrap_or_else(|error| panic!("Проблема с созданием файла: {:?}", error));
        }
        panic!("Проблема с открытием файла: {:?}", error);
    })
}

use std::fs::File;
use std::io::{
    ErrorKind,
    Error,
    Read
};
use std::net::IpAddr;

fn main() {
    {
        let _file: File = match File::open("hello.txt") {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(new_file) => new_file,
                    Err(new_file_error) => panic!("Problem creating a new file {:?}", new_file_error)
                },
                other_error => panic!("Problem opening the file {:?}", other_error)
            }
        };
    }

    {
        let _file: File = File::open("hello.txt").unwrap();
    }

    {
        let _file: File = File::open("hello.txt").expect("Failed to open hello.txt");
    }

    {
        let home: IpAddr = "127.0.0.1".parse().unwrap();
        println!("Home: {}", home);
    }
}

fn _read_username_from_file(file_name: &str) -> Result<String, Error> {
    /*let f: Result<File, Error> = File::open(file_name);

    let mut f: File = match f {
        Ok(file) => file,
        Err(error) => return Err(error)
    };

    let mut s: String = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error)
    }*/

    /*let mut f: File = File::open(file_name)?;
    let mut s: String = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)*/

    let mut s: String = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}
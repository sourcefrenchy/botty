use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

extern crate ring;

use branca::decode;


fn main() {

    let _key = b"7861a1fd-d2a2-4a87-8855-d3458dfa".to_vec();
    let _session = "n2WiC2zLCLkJhmG3mpSq0x8FlX2hBDOBmmat5LuILJeAo8m2rHgb06VNnTOlIstd4lNoOZrgzW3rWqXbXFDTvF3460";
    let _auth = "2QYcCJeiBTwMmEBm7vim5A7lZTQqnF30uce2MVjtLvLRWIYMHc9dbj3gpBPcHVSLUiRfmW2LF8gcRImnw7JUtUmNM5swD1I0SERuCrvH1hyGOWjPOTupuMa9gwsqPhbv2FH77VgeRZveTjbu9RV0Q3OoehSqpRDy2wchkc1Ev9";

    println!("...");
    let server_port = decode(&_session, &_key, 0).unwrap();
    // println!("Xauth:{}", _auth);
    match TcpStream::connect(&server_port) {
        Ok(mut stream) => {
            println!("успешный {}", &server_port);

            // let msg = b"Hello!";
            let msg = decode(&_auth, &_key, 0).unwrap();
            //println!("token : {}", msg);

            stream.write(msg.as_bytes()).unwrap();
            stream.write(b"\n").unwrap();
            println!("ждать");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg.as_bytes() {
                        println!("признанный {}", msg);
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("неожиданный {}", text);
                    }
                },
                Err(e) => {
                    println!("ошибка {}", e);
                }
            }
        },
        Err(e) => {
            println!("ошибка {}", e);
        }
    }
    println!("конец");
}
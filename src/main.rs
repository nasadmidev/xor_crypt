use std::env;
use std::process;

fn encrypt(message: String, key: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut assignment = 0;
    for b in message.trim().bytes() {
        result.push(b ^ key[if assignment >= key.len() {
            assignment = 0;
            0
        } else {
            assignment
        }]);
        assignment += 1;
    }
    result
}

fn decrypt(mut crypt: String, key: Vec<u8>) -> String {
    let mut result = String::new();
    let mut assignment = 0;
    for _ in 0..(crypt.len()/8) {
        let binary: String = crypt.drain(0..8).collect();
        if let Ok(code) = u32::from_str_radix(&binary, 2) {
            let new_code = code ^ key[
                if assignment >= key.len() {
                    assignment = 0;
                    0
                } else {
                    assignment
                }
            ] as u32;
            if let Some(c) = char::from_u32(new_code) {
                result.push(c);
            }
        }
        assignment += 1;
    }
    result
}

fn main() {
    let mut args = env::args();
    args.next();
    let option = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Debes especificar la operacion: decrypt (o) encrypt");
            process::exit(1);
        },
    };
    let message = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Debes especificar el mensaje a encriptar o el binario a desencriptar");
            process::exit(1);
        },
    };
    let key = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Debes especificar la clave");
            process::exit(1);
        },
    };
    if key.is_empty() {
        eprintln!("La clave no puede estar vacía");
        process::exit(1);
    }
    let key: Vec<u8> = key.trim().bytes().collect();
    match option.trim() {
        "decrypt" => {
            println!("{}", decrypt(message, key));
        },
        "encrypt" => {
            for r in encrypt(message, key) {
                print!("{:08b}", r);
            }
        },
        _ => {
            panic!("Opcion invalidad: e o d");
        }
    }
}

use std::env;
use std::process;
use std::io::{self, BufWriter, Write};

fn encrypt(message: String, key: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for (i, b) in message.trim().bytes().enumerate() {
        result.push(b ^ key[i % key.len()]);
    }
    result
}

fn decrypt(mut crypt: String, key: Vec<u8>) -> String {
    let mut result = String::new();
    for i in 0..(crypt.len()/8) {
        let binary: String = crypt.drain(0..8).collect();
        if let Ok(code) = u32::from_str_radix(&binary, 2) {
            let new_code = code ^ key[i % key.len()] as u32;
            if let Some(c) = char::from_u32(new_code) {
                result.push(c);
            }
        }
    }
    result
}

fn main() {
    let mut args = env::args().skip(1);
    let option = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Debes especificar la operación: decrypt (o) encrypt");
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
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    match option.trim() {
        "decrypt" => {
            _ = writeln!(writer, "{}", decrypt(message, key));
        },
        "encrypt" => {
            for r in encrypt(message, key) {
                _ = write!(writer, "{:08b}", r);
            }
            _ = writeln!(writer);
        },
        _ => {
            eprintln!("Opción no especificada");
            _ = writer.flush();
            process::exit(1);
        }
    }
    _ = writer.flush();
}

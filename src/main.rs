use std::fs;
use std::io;
fn main() {
    println!("Hello, chose mode!");
    println!("1. Encryption");
    println!("2.decryption");
    let mut mode = String::new();
    

    io::stdin().read_line(&mut mode)
    .expect("invalid input");

    

    let mode: u8 = mode.trim().parse().expect("invalid mode");

    match mode {
        1 => {
            println!("entry the path of your file for encryption");
            let mut path_file = String::new();

            io::stdin().read_line(&mut path_file)
            .expect("invalid path");
            let path_file = path_file.trim();
            let mut key = String::new();
            io::stdin().read_line(&mut key)
            .expect("invalid key");

            let key: i8 = key.trim().parse().expect("invalid");

            encrypte_file(path_file, key);
        }   

        2 => {
            println!("entry the path of your file dor decryption");
            let mut path_file = String::new();

            io::stdin().read_line(&mut path_file)
            .expect("invalid path");
            let path_file = path_file.trim();
            let mut key = String::new();
            io::stdin().read_line(&mut key)
            .expect("invalid key");
            
            let key: i8 = key.trim().parse().expect("invalid");
            decrypted_file(path_file, key);
        }

        _ => {
            println!("Invalide request");
        }
    }
    
}


fn encrypte_file(file_path : &str, key : i8) {
    let content = fs::read_to_string(file_path).expect("invalid path please retry");
    let encrypte_content: String = content.chars().map(|c| shift(c, key)).collect();
    fs::write("encrypted_file.txt", encrypte_content).expect("failed to write the text");
}

fn decrypted_file(file_path : &str, key : i8) {
    let content = fs::read_to_string(file_path).expect("invalid path please retry");
    let encrypte_content: String = content.chars().map(|c| shift(c, -1*key)).collect();
    fs::write("decrypted_file.txt", encrypte_content).expect("failed to write the text");
}
fn shift (c : char, key: i8 ) -> char {
    let c_shift = |c: char, key: i8| -> char {
        let mut ascii = (c as u8 as i8 + key) as u8;
        if ascii > 122 {
            ascii = ascii - 26;
        }
        if ascii < 97 {
            ascii = ascii + 26;
        }
        ascii as char
    };
    if c.is_ascii_lowercase() {
        c_shift(c, key)
    }
    else if c.is_ascii_uppercase(){
        let c_lower = c.to_lowercase().next().unwrap_or(c);
        c_shift(c_lower, key).to_uppercase().next().unwrap_or(c)
    } else {
        c
    }
  
}


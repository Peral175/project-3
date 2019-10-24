use std::env;
fn main() {
    let arguments: Vec<String> = env::args().collect();
    loop {
        if arguments.len() == 1{
        intro();
        break
        }
        match &arguments[1].as_str(){
            &"help" => {
             help();
             break
            },
            _ =>{
                println!("Only one argument was passed! If you need help, type:     cargo run help .\n\n");
                break
            
            }
        }   
    };
    match arguments.len(){
        5 => {
            let command = match arguments[1].to_lowercase().as_str(){
                "encrypt" => Command::Encrypt,
                "decrypt" => Command::Decrypt,
                _ => {
                    println!("Action has to be either Encrypt or decrypt!\n");
                    panic!();
                }
            };
            let cipher  = match arguments[2].to_lowercase().as_str(){
                "caes"      => Cipher::Caes,
                "caesar"    => Cipher::Caes,
                "mono"      => Cipher::Mono,
                "vige"      => Cipher::Vige,
                "vigenere"  => Cipher::Vige,
                "stre"      => Cipher::Stre,
                "stream"    => Cipher::Stre,
                _           => {
                    println!("No cipher found with that name!\n");
                    panic!();
                }
            };
            let message = &arguments[3];
            if message_check(message) == true{
                let msg = message;
            }

            let secs     = &arguments[4];

            match cipher {
                Cipher::Caes    => {
                    if numbers_check(secs) == true{
                        let sec = secs;
                    }

                },
                Cipher::Mono    => {
                    if message_check_no_space(secs) == true{
                        let sec = secs;
                    }
                },
                Cipher::Vige    => {           
                    if message_check_no_space(secs) == true{
                        let sec = secs;
                    }
                },
                    
                Cipher::Stre    => {
                    if message_check_no_space(secs) == true{
                        let sec = secs;
                    }
                }

            }

            
        },
        _ =>{
            println!("Incorrect form! Type  cargo run help   for help.\n\n");
        }
    };

}


enum Command {
    Encrypt,
    Decrypt 
}
enum Cipher{
    Caes,
    Mono,
    Vige,
    Stre
}

fn intro(){
    println!("This is a command line interface for old ciphers.
    \nPlease enter cargo run into the terminal, fallowed by commands.
    \nThe commands for this CLI are:
    \n      * Encryption or decryption
    \n      * Cipher of your choice
    \n      * Message that you want to en-or-decrypt
    \n      * and finally the secondary elements necessary for action to be performed.
    \nThis is the correct form:
    \n  --  cargo run <decrypt> <caes> <'ABC'> <3>
    \n\nThe ciphers available are: 
    \n          1. Caesar cipher ,              with the shortcut       --      caes        --  it requires [Integer] as shift
    \n          2. Mono-alphabetic sub cipher,  with the shortcut       --      mono        --  it requires [String]  as key
    \n          3. Vignere cipher,              with the shortcut       --      vige        --  it requires [String]  as key
    \n          4. Stream cipher,               with the shortcut       --      stre        --  it requires [String]  as key
    \n\nThe message entered needs to be put between     '(your message)'      and can not contain numbers.
    \nIf you need help, you can type:   --  cargo run help.\n\n\n")
}
fn help(){
    println!("This is the correct form: 
    \n  --  cargo run <action> <cipher> <'message'> <secondary>
    \n Messages can only contain A-Z and a-z with whitespaces.
    \n Caesar cipher needs to be given an integer.
    \n Mono, vigenere and stream need to be given a secret key, which can only contain A-Z and a-z, without whitespace.\n\n\n")
}
fn message_check(input: &str) -> bool{
    for i in input.chars(){
        if (i.is_ascii_alphabetic() == false) && (i.is_whitespace() == false){
            return false
        }
    }
    return true
}
fn message_check_no_space(input: &str) -> bool{
    for i in input.chars(){
        if i.is_ascii_alphabetic() == false{
            return false
        }
    }
    return true
}
fn numbers_check(input: &str) -> bool{
    for i in input.chars(){
        if i.is_ascii_alphanumeric() == false{
            return false
        }
    }
    return true
}
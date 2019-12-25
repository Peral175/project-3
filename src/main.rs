use std::env;
use std::fs::File;
use std::io::{BufReader,BufRead};
use std::io::BufWriter;
mod cipher_functions;
fn main() {
    let arguments: Vec<String> = env::args().collect();
    match arguments.len(){
        1 => {
            initial_message();
        }
        5 => {
            let action = match arguments[1].to_lowercase().as_str(){
                "encrypt" => Action::Enc,
                "decrypt" => Action::Dec,
                "e"       => Action::Enc,
                "d"       => Action::Dec,
                _ =>  {
                    println!("Action has to be either 'encrypt' (e) or 'decrypt' (d)!");
                    panic!();
                }
            };
            let cipher = match arguments[2].to_lowercase().as_str(){
                "caesar" => Ciphers::Caesar,
                "c" => Ciphers::Caesar,
                "mono" => Ciphers::Mono,
                "m" => Ciphers::Mono,
                "vigenere" => Ciphers::Vigenere,
                "v" => Ciphers::Vigenere,
                "stream" => Ciphers::Stream,
                "s" => Ciphers::Stream,
                _ => {
                    println!("No cipher found with that name!\n");
                    panic!();
                }
            };
            let cipher_secondary = &arguments[3];
            let file = &arguments[4];
            println!("{:?}",action );
            println!("{:?}",cipher);
            println!("{:?}",cipher_secondary);
            println!("{:?}",file);

            let structure = CipherStruct{struct_action: action, struct_cipher: cipher,struct_cipher_secondary:cipher_secondary.to_string() , struct_file: file.to_string()};
            println!("{:?}",structure);

            selection(structure);
            //let res = cipher_functions::old_ciphers::caesar::caesar_encrypt(message,sec);

        },
        _ => {
            println!("Incorrect form!");
        }
    }
}
#[derive(Debug)]
enum Action{
    Enc,
    Dec,
}
#[derive(Debug)]
enum Ciphers{
    Caesar,
    Mono,
    Vigenere,
    Stream,
}
#[derive(Debug)]
struct CipherStruct{
    struct_action: Action,
    struct_cipher: Ciphers,
    struct_cipher_secondary: String,
    struct_file: String,
}
fn selection(structure:CipherStruct){
    let action = structure.struct_action as u8;
    let cipher = structure.struct_cipher as u8;
    let cipher_secondary = structure.struct_cipher_secondary;
    let file = structure.struct_file;
    if      action == 0 && cipher == 0  {

        // cipher_functions::old_ciphers::caesar::caesar_encrypt();     .unwrap()   insert message/file and sec
    }
    else if action == 1 && cipher == 0  {

    }
    else if action == 0 && cipher == 1  {

    }
    else if action == 1 && cipher == 1  {

    }
    else if action == 0 && cipher == 2  {

    }
    else if action == 1 && cipher == 2  {

    }
    else if action == 0 && cipher == 3  {

    }
    else if action == 1 && cipher == 3  {

    }
    else{
        println!("Invalid input!");
    }


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
fn initial_message(){
    println!("Message encryptor/decryptor using a command line interface.
    \nUsage:       cargo run -- [encrypt/decrypt] [cipher] [filename]");
}
/*Everything in functions
buffer read write
add frame number to other file
jpeg implementation
*/

use std::env;
mod ciphers;
fn help(){
    println!("This is the command line interface for old ciphers.
    \nTo operate it, enter:
    \n\n'cargo run -- <'Message to be encrypted'> <Encrypt or Decrypt> <Cipher of your choice><Secondary information according to cipher:
    \nCaesar: Shift as integer
    \nMono & Vigenere: Key with only letters
    \n>'");
}
fn main(){
    let alphabet_lower: &str = "abcdefghijklmnopqrstuvwxyz";
    let alphabet_upper: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    //check if message and key are only ASCII A-Z
    //check if Integer for caesar
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            //a length of 1 means no arguments were passed
            //other than the path
            println!("No arguments were passed! \n");
            help();
        },
        5 => {
            //1. path
            //2. message
            //3. Action
            //4. Cipher
            //5. Secondary info
            let msg = &args[1];
            let action = &args[2].to_lowercase();
            let cip = &args[3].to_lowercase();
            let sec = &args[4];
            println!("{}, {}, {}, {}",msg, action, cip, sec);
        },
        _ => {
            println!("Invalid arguments input!\n");
            help();
        }

    }
}

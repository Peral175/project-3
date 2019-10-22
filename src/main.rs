use std::env;
mod ciphers;
fn help(){
    println!("This is the command line interface for old ciphers.
    \nTo operate it, enter:
    \n\n'cargo run -- <Message to be encrypted> <Encrypt or Decrypt> <Cipher of your choice>'
    \n\nYou will then be asked to enter the secondary information that will be needed to perform the encryption/decryption. ");
}
fn main(){
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            //a length of 1 means no arguments were passed
            //other than the path
            println!("No arguments were passed! \n");
            help();
        },
        4 => {
            //1. path
            //2. message
            //3. Action
            //4. Cipher
            let msg = &args[1];
            let action = &args[2].to_lowercase();
            let cip = &args[3].to_lowercase();
            println!("{}, {}, {}",msg, action, cip);
        },
        _ => {
            println!("Invalid arguments input!\n");
            help();
        }

    }
}

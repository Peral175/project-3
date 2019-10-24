use std::env;
mod ciphers;
mod stream;
fn help(){
    println!("This is the command line interface for old ciphers.
    \nTo operate it, enter:
    \n\n'cargo run -- <'Message to be encrypted'> <Encrypt or Decrypt> <Cipher of your choice><Secondary information according to cipher: '>
    \nCaesar: Shift as integer
    \nMono & Vigenere: Key with only letters");
}
fn main(){
    let alphabet_lower: &str = "abcdefghijklmnopqrstuvwxyz";
    let alphabet_upper: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("No arguments were passed.\n");
            help();
        },
        2 => {
            println!("Streamcipher: \n");
            //let msg = "!!!!";
            let msg = "@CBE";
            let key = "abcd";
            let r: String = stream::modern_ciphers::stream_cipher::encrypt_stream(msg,key).unwrap();
            println!("Stream cipher: {}", r);
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

            if message_check(msg) == false{
                println!("Message can only contain 'A-Z', 'a-z', and whitespace!");
            }
            else{
                if action == "encrypt" && cip == "caesar" && numbers_check(sec) == true{
                    let r:String = ciphers::old_ciphers::caesar_cipher::encrypt_caes(msg, sec, alphabet_lower, alphabet_upper).unwrap();
                    println!("Caesar encrypt: {}",r);
                }
                else if action == "decrypt" && cip == "caesar" && numbers_check(sec) == true{
                    let r:String = ciphers::old_ciphers::caesar_cipher::decrypt_caes(msg, sec, alphabet_lower, alphabet_upper).unwrap();
                    println!("Caesar decrypt: {}",r);
                }
                else if action == "encrypt" && cip == "mono" && message_check_no_space(sec) == true{
                    let r: String = ciphers::old_ciphers::mono_substitution_cipher::encrypt_mono(msg, sec, alphabet_lower, alphabet_upper).unwrap();
                    println!("Mono encrypted: {}", r);
                }
                else if action == "decrypt" && cip == "mono" && message_check_no_space(sec) == true{
                    let r: String = ciphers::old_ciphers::mono_substitution_cipher::decrypt_mono(msg, sec, alphabet_lower, alphabet_upper).unwrap();
                    println!("Mono decrypted: {}", r);
                }
                else if action == "encrypt" && cip == "vig" && message_check_no_space(sec) == true{
                    let r: String = ciphers::old_ciphers::vigenere_cipher::encrypt_vige(msg, sec, alphabet_lower, alphabet_upper).unwrap();
                    println!("Vigenere encrypted: {}", r);
                }
                else if action == "decrypt" && cip == "vig" && message_check_no_space(sec) == true{
                    let r: String = ciphers::old_ciphers::vigenere_cipher::decrypt_vige(msg, sec, alphabet_lower, alphabet_upper).unwrap();
                    println!("Vigenere decrypted: {}", r);
                }
                else{
                    println!("Invalid secondary input!");
                }
            }
        },
        _ => {
            println!("Invalid arguments input!\n");
            help();
        }

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

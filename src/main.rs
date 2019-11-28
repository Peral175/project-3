use std::env;
mod ciphers;
mod stream;
extern crate bit_vec;
extern crate bitreader;
use std::fs;
use std::io::prelude::*;


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
        3 => {
//Potential solution:
//Use of BOOL:
// 0 = False
// 1 = True
//this would still need the dissassembling of bytes into bits



            let msg = &args[1];
            let mess = msg.as_bytes();
            let message = mess.to_vec();
            println!("{:?},\n{:?}, {:?}",msg,message,message.len());
            let sess_key = (&args[2]).as_bytes();
            let bla = binary(sess_key);
            let mut sk:[u8;64] = [Default::default();64];
            bla.iter().zip(sk.iter_mut()).map(|(&bla, sk)| *sk = bla).count();
            // println!("{:?}",sk);
            // let mut count = 0;
            // for i in sk.iter(){
            //      count += 1;
            //      println!("{:?} {}",i,count );
            // }
            let frame_counter:[u8;22] = [1,1,1,0,1,0,1,0,1,1,0,0,1,1,1,1,0,0,1,0,1,1];
            // println!("{:?}",frame_counter);
            //frame counter is public!

            let x = (message.len()/28) +1 ;
            println!("XX: {:?}",x);
            for y in 0..x{

            }

//maybe concat enough together
//maybe slice input into smaller pieces and regroup before writing
//make better functions
//revisit types used
//revisit binary
//memory bittest maybe


            let (a,b,c) = ciphers::modern_ciphers::stream_cipher::init();
            // println!("{:?},\n{:?},\n{:?}",a,b,c);
            let (d,e,f) = ciphers::modern_ciphers::stream_cipher::session_key(sk,a,b,c);
            println!("{:?},\n{:?},\n{:?}",d,e,f);
            let (g,h,i) = ciphers::modern_ciphers::stream_cipher::frame_number(frame_counter,d,e,f);
            println!("\n\n{:?},\n{:?},\n{:?}",g,h,i);
            let (j,k,l) = ciphers::modern_ciphers::stream_cipher::system_clocking(g,h,i);
            println!("\n\n{:?},\n{:?},\n{:?}",j,k,l);
            let m = ciphers::modern_ciphers::stream_cipher::keystream(j,k,l);
            println!("\n\n{:?},\n {:?}",m,m.len());
            let n = ciphers::modern_ciphers::stream_cipher::stream(message,m);
            println!("\n\n{:?}\n\n",n);
            let p = n.clone();
            let o = ciphers::modern_ciphers::stream_cipher::write_file(n);



            let (a1,b1,c1) = ciphers::modern_ciphers::stream_cipher::init();
            // println!("{:?},\n{:?},\n{:?}",a,b,c);
            let (d1,e1,f1) = ciphers::modern_ciphers::stream_cipher::session_key(sk,a1,b1,c1);
            println!("{:?},\n{:?},\n{:?}",d1,e1,f1);
            let (g1,h1,i1) = ciphers::modern_ciphers::stream_cipher::frame_number(frame_counter,d1,e1,f1);
            println!("\n\n{:?},\n{:?},\n{:?}",g1,h1,i1);
            let (j1,k1,l1) = ciphers::modern_ciphers::stream_cipher::system_clocking(g1,h1,i1);
            println!("\n\n{:?},\n{:?},\n{:?}",j1,k1,l1);
            let m1= ciphers::modern_ciphers::stream_cipher::keystream(j1,k1,l1);
            println!("\n\n{:?},\n {:?}",m1,m1.len());
            let n1 = ciphers::modern_ciphers::stream_cipher::stream(p,m1);
            println!("\n\n{:?}",n1);
            let o1 = ciphers::modern_ciphers::stream_cipher::write_file_2(n1);



        },
        4 => {
            println!("Streamcipher: \n");
            let msg     = &args[1];
            let option  = &args[2].to_lowercase();
            let input   = &args[3];
            let content = input;
            if option == "file"{
                let content_2 = &fs::read_to_string(input).expect("Something went wrong!");
                let r  = stream::modern_ciphers::stream_cipher::encrypt_stream(msg, content_2).unwrap();
                let re = stream::modern_ciphers::stream_cipher::decrypt_stream(&r,content_2).unwrap();
            }else if option == "user_input"{
                let r  = stream::modern_ciphers::stream_cipher::encrypt_stream(msg, content).unwrap();
                let re = stream::modern_ciphers::stream_cipher::decrypt_stream(&r,content).unwrap();
            }

            // let msg = "Hello there! General Kenoby?";
            // let key = "alex.txt";
            // stream::modern_ciphers::stream_cipher::encrypt_stream(msg,key).unwrap();
            // stream::modern_ciphers::stream_cipher::decrypt_stream(&"blabla.txt",key).unwrap();
            //let r: String = stream::modern_ciphers::stream_cipher::encrypt_stream(msg,key).unwrap();
            //println!("Stream cipher encrypt: {}", r);
            //let re: String = stream::modern_ciphers::stream_cipher::decrypt_stream(&"blabs.txt",key).unwrap();
            //println!("Stream cipher decrypt: {}", re);
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
        if i.is_ascii_digit() == false{
            return false
        }
    }
    return true
}
fn binary(input:&[u8]) -> Vec<u8> {
    let mut vec = Vec::new();
    let mut vec2 = Vec::new();
    // println!("{:?}",input);
    for i in input{
        let i = *i as i64;
        // println!("{:?}",i);
        let res = to_binary(i);
        // println	!("BBBBB: {:?}",res);
        let ress:String = res.chars().rev().collect();
        // println	!("CCCCC: {:?}",ress);
        for j in ress.chars(){
            // println!("{:?}",j);
            vec.push(j);
        }
    }
    // println!("{:?}",vec);
    for i in vec{
        if i == '0'{
            vec2.push(0);
        }
        if i == '1'{
            vec2.push(1);
        }
    }
    vec2
}
fn to_binary(mut decimal: i64) -> String{
    if decimal == 0{
        decimal.to_string()
    }else{
        let mut bits = String::new();
        while decimal > 0 {

            if decimal % 2 == 0{
                bits.push_str("0");
            }else {
                bits.push_str("1");
            }
            decimal /= 2;
        }
        while bits.len() < 8{
            // println!("{:?}",bits.len());
            bits.push_str("0");
            // println!("{:?}",bits);
            // println!("AAAAA: {:?}",bits);
        }
        for i in bits.chars().rev(){
            // println!("{:?}",i);
        }
        bits
        // match bits.chars().rev().collect::<String>().parse() {
        //     Ok(num) => num,
        //     Err(_e) => panic!("Something went wrong in the binary transformation!"),
        // }
    }
}
// fn slice(bytes: &[u8]) -> [u8;64] {
//     let mut array = [0;64];
//     let bytes = &bytes[..array.len()];
//     array.copy_from_slice(bytes);
//     array
// }

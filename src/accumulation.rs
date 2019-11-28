pub mod old_ciphers{
    pub mod caesar_cipher{
        pub fn encrypt_caes(msg: &str, sec: &str, alphabet_lower: &str, alphabet_upper: &str ) -> Option<String>{
            let mut shift: u32 = sec.parse().unwrap();
            shift = shift % 26;
            let mut result: String = String::new();

            for i in msg.chars(){
                if i.is_whitespace(){
                    result.push(i);
                    continue;
                }
                else {
                    let mut alphabet = alphabet_lower;
                    if i.is_uppercase(){
                        alphabet = alphabet_upper;
                    }
                    let x = alphabet.chars().position(|p| i == p).unwrap();
                    let idx: usize = (x + shift as usize) % 26;
                    let err_s = format!("No element at index {}", idx);
                    let y = alphabet.chars().nth(idx).expect(&err_s);
                    result.push(y);
                }
            }
            return Some(result)
        }
        pub fn decrypt_caes(msg: &str, sec: &str, alphabet_lower: &str, alphabet_upper: &str ) -> Option<String>{
            let mut shift: u32 = sec.parse().unwrap();
            shift = shift % 26;
            let mut result: String = String::new();

            for i in msg.chars(){
                if i.is_whitespace(){
                    result.push(i);
                    continue;
                }
                else {
                    let mut alphabet = alphabet_lower;
                    if i.is_uppercase(){
                        alphabet = alphabet_upper;
                    }
                    let x = alphabet.chars().position(|p| i == p).unwrap();
                    let  idx: usize = (x +26 - shift as usize) % 26;
                    let err_s = format!("No element at index {}", idx);
                    let y = alphabet.chars().nth(idx).expect(&err_s);
                    result.push(y);
                }
            }
            return Some(result)
        }
    }
    pub mod mono_substitution_cipher{
        pub fn encrypt_mono(msg: &str, sec: &str, alphabet_lower: &str, _alphabet_upper: &str ) -> Option<String>{
                let key = sec.to_lowercase();
                let message = msg.to_lowercase();
                let mut secret_alphabet: String = String::new();
                for i in key.chars(){
                    match secret_alphabet.chars().position(|p| i== p){
                        Some(_a) => {
                            continue;
                        },
                        None => {
                            secret_alphabet.push(i);
                        }
                    }
                }
                println!("Duplicates removed.\nThe new key: {}",secret_alphabet);
                for j in alphabet_lower.chars(){
                    match secret_alphabet.chars().position(|r| j == r){
                        Some(_b) => {
                            continue;
                        },
                        None => {
                            secret_alphabet.push(j);
                        }
                    }
                }
                println!("The complete secret alphabet: {}",secret_alphabet);
                let mut result: String = String::new();
                for k in message.chars(){
                    if k.is_whitespace(){
                        result.push(k);
                        continue;
                    }
                    else{
                        let x = secret_alphabet.chars().position(|s| k == s).unwrap();
                        let err_s = format!("No element at index {}", x);
                        let y = alphabet_lower.chars().nth(x).expect(&err_s);
                        result.push(y);
                    }
                }
                return Some(result)
        }
        pub fn decrypt_mono(msg: &str, sec: &str, alphabet_lower: &str, _alphabet_upper: &str ) -> Option<String>{
                let key = sec.to_lowercase();
                let message = msg.to_lowercase();
                let mut secret_alphabet: String = String::new();
                for i in key.chars(){
                    match secret_alphabet.chars().position(|p| i== p){
                        Some(_a) => {
                            continue;
                        },
                        None => {
                            secret_alphabet.push(i);
                        }
                    }
                }
                println!("Duplicates removed.\nThe new key: {}",secret_alphabet);
                for j in alphabet_lower.chars(){
                    match secret_alphabet.chars().position(|r| j == r){
                        Some(_b) => {
                            continue;
                        },
                        None => {
                            secret_alphabet.push(j);
                        }
                    }
                }
                println!("The complete secret alphabet: {}",secret_alphabet);
                let mut result: String = String::new();
                for k in message.chars(){
                    if k.is_whitespace(){
                        result.push(k);
                        continue;
                    }
                    else{
                        let x = alphabet_lower.chars().position(|s| k == s).unwrap();
                        let err_s = format!("No element at index {}", x);
                        let y = secret_alphabet.chars().nth(x).expect(&err_s);
                        result.push(y);
                    }
                }
                return Some(result)
        }

    }
    pub mod vigenere_cipher{
        pub fn encrypt_vige(msg: &str, sec: &str, alphabet_lower: &str, alphabet_upper: &str ) -> Option<String>{
            let key = sec;
            let mut result: String = String::new();
            let mut vector: Vec<usize> = Vec::new();

            for i in key.chars(){
                if i.is_lowercase(){
                    vector.push(i as usize - 97);
                }
                else if i.is_uppercase(){
                    vector.push(i as usize - 65);
                }
            }

            let mut iter = vector.iter().cycle();

            for i in msg.chars(){
                if i.is_whitespace(){
                    result.push(i);
                    continue;
                }
                else{
                    let next = iter.next()?;
                    let mut alphabet = alphabet_lower;
                    if i.is_uppercase(){
                        alphabet = alphabet_upper;
                    }
                    let x = alphabet.chars().position(|p| i == p).unwrap();
                    let idx: usize = (x + next) % 26;
                    let err_s = format!("No element at index {}", idx);
                    let y = alphabet.chars().nth(idx).expect(&err_s);

                    result.push(y);
                }
            }
            return Some(result)
        }
        pub fn decrypt_vige(msg: &str, sec: &str, alphabet_lower: &str, alphabet_upper: &str ) -> Option<String>{
            let key = sec;
            let mut result: String = String::new();
            let mut vector: Vec<usize> = Vec::new();

            for i in key.chars(){
                if i.is_lowercase(){
                    vector.push(i as usize - 97);
                }
                else if i.is_uppercase(){
                    vector.push(i as usize - 65);
                }
            }

            let mut iter = vector.iter().cycle();

            for i in msg.chars(){
                if i.is_whitespace(){
                    result.push(i);
                    continue;
                }
                else{
                    let next = iter.next()?;
                    let mut alphabet = alphabet_lower;
                    if i.is_uppercase(){
                        alphabet = alphabet_upper;
                    }
                    let x = alphabet.chars().position(|p| i == p).unwrap();
                    let idx: usize = (x + 26 - next) % 26;
                    let err_s = format!("No element at index {}", idx);
                    let y = alphabet.chars().nth(idx).expect(&err_s);

                    result.push(y);
                }
            }
            return Some(result)
        }
    }
}
pub mod modern_ciphers{
    pub mod stream_cipher{
        pub fn stream(message:Vec<u8>,vec:Vec<u8>) ->  Vec<u8>{
            //228 bit keystream output is combined with the message that is to be encrypted
            //This concludes the encryption
            //For the decryption, the same operations have to be performed and one ands up wiht the initial message
            //One needs the same public frame number and the same secret session key

            //Implement that the program automatically performs the operation as many times as necessary to
            //fully encryt/decrypt the message.

            let mut iter = vec.iter();
            let mut vec1 = Vec::new();
            let mut vec2 = Vec::new();
            for i in 0..message.len(){
                let mut stack = Vec::new();
                for j in 0..8{
                    let it = iter.next().unwrap();
                    stack.push(it);
                }
                let byte = bin(stack);
                // println!("BLA {:?}",byte);
                vec1.push(byte);
                // println!("{:?}",vec1);
            }
            for j in 0..message.len(){
                // println!("MIAU {:?}",j);
                let res = message[j] ^ vec1[j] as u8;
                vec2.push(res);
                // println!("{}, {}, {}",message[j],vec1[j],res);
            }
            vec2


        }
        pub fn bin(mut stack:Vec<&u8>) -> u64{
            let mut res = 0;
            let mut count = 0;
            while let Some(top) = stack.pop(){
                if top == &1u8 {
                    res += 2u64.pow(count);
                }
                count += 1;
            }
            res
        }
        pub fn init() -> (Vec<u8>,Vec<u8>,Vec<u8>) {
            //create and initialize the three LFSR with all values being 0's
            let lfsr_1: [u8;19] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
            let lfsr_2: [u8;22] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
            let lfsr_3: [u8;23] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

            let lfsr1: Vec<u8> = lfsr_1.to_vec();
            let lfsr2: Vec<u8> = lfsr_2.to_vec();
            let lfsr3: Vec<u8> = lfsr_3.to_vec();
            (lfsr1,lfsr2,lfsr3)
        }
        pub fn session_key(sk:[u8;64], mut lfsr1:Vec<u8>, mut lfsr2:Vec<u8>, mut lfsr3:Vec<u8>) -> (Vec<u8>,Vec<u8>,Vec<u8>){
            //Sessionkey is mixed into the LFSR's with no majority rule
            let mut count = 0;
            for i in sk.iter(){
                let first_xor = lfsr1[13] ^ lfsr1[16] ^ lfsr1[17] ^ lfsr1[18];
                let second_xor = lfsr2[20] ^ lfsr2[21];
                let third_xor = lfsr3[7] ^ lfsr3[20] ^ lfsr3[21] ^ lfsr3[22];

                let r1 = first_xor ^ i ;
                let r2 = second_xor ^ i;
                let r3 = third_xor ^ i;

                lfsr1.iter().cycle().next().unwrap();
                lfsr2.iter().cycle().next().unwrap();
                lfsr3.iter().cycle().next().unwrap();

                lfsr1.pop();
                lfsr1.insert(0,r1);
                lfsr2.pop();
                lfsr2.insert(0,r2);
                lfsr3.pop();
                lfsr3.insert(0,r3);
                count += 1
            }
            (lfsr1,lfsr2,lfsr3)
        }
        pub fn frame_number(frame:[u8;22],mut lfsr1:Vec<u8>, mut lfsr2:Vec<u8>, mut lfsr3:Vec<u8>) -> (Vec<u8>,Vec<u8>,Vec<u8>){
            //Frame number is mixed into the LFSR's with no majority rule
            let mut count = 0;
            for i in frame.iter(){
                let first_xor = lfsr1[13] ^ lfsr1[16] ^ lfsr1[17] ^ lfsr1[18];
                let second_xor = lfsr2[20] ^ lfsr2[21];
                let third_xor = lfsr3[7] ^ lfsr3[20] ^ lfsr3[21] ^ lfsr3[22];

                let r1 = first_xor ^ i;
                let r2 = second_xor ^ i;
                let r3 = third_xor ^ i;

                lfsr1.iter().cycle().next().unwrap();
                lfsr2.iter().cycle().next().unwrap();
                lfsr3.iter().cycle().next().unwrap();

                lfsr1.pop();
                lfsr1.insert(0,r1);
                lfsr2.pop();
                lfsr2.insert(0,r2);
                lfsr3.pop();
                lfsr3.insert(0,r3);
                count += 1
            }
            (lfsr1,lfsr2,lfsr3)
        }
        pub fn system_clocking(mut lfsr1: Vec<u8>,mut lfsr2: Vec<u8>,mut lfsr3: Vec<u8>) -> (Vec<u8>,Vec<u8>,Vec<u8>){
            //100 cycles where the output is discareded with irregular clocking
            let mut count = 0;
            for i in 0..100{
                let first_xor = lfsr1[13] ^ lfsr1[16] ^ lfsr1[17] ^ lfsr1[18];
                let second_xor = lfsr2[20] ^ lfsr2[21];
                let third_xor = lfsr3[7] ^ lfsr3[20] ^ lfsr3[21] ^ lfsr3[22];

                let r1 = first_xor;
                let r2 = second_xor;
                let r3 = third_xor;

                lfsr1.iter().cycle().next().unwrap();
                lfsr2.iter().cycle().next().unwrap();
                lfsr3.iter().cycle().next().unwrap();

                let x = lfsr1[8];
                let y = lfsr2[10];
                let z = lfsr3[10];

                let maj = majority(x,y,z);
                if x == maj {
                    lfsr1.pop();
                    lfsr1.insert(0,r1);
                }
                if y == maj{
                    lfsr2.pop();
                    lfsr2.insert(0,r2);
                }
                if z == maj{
                    lfsr3.pop();
                    lfsr3.insert(0,r3);
                }
                count += 1;
            }
            (lfsr1,lfsr2,lfsr3)

        }
        pub fn keystream(mut lfsr1: Vec<u8>,mut lfsr2: Vec<u8>,mut lfsr3: Vec<u8>) -> Vec<u8>{
            //228 bit keystream output
            let mut vec = Vec::new();
            let mut count = 0;
            for i in 0..228{
                let first_xor = lfsr1[13] ^ lfsr1[16] ^ lfsr1[17] ^ lfsr1[18];
                let second_xor = lfsr2[20] ^ lfsr2[21];
                let third_xor = lfsr3[7] ^ lfsr3[20] ^ lfsr3[21] ^ lfsr3[22];

                let r1 = first_xor;
                let r2 = second_xor;
                let r3 = third_xor;

                lfsr1.iter().cycle().next().unwrap();
                lfsr2.iter().cycle().next().unwrap();
                lfsr3.iter().cycle().next().unwrap();

                let x = lfsr1[8];
                let y = lfsr2[10];
                let z = lfsr3[10];

                let maj = majority(x,y,z);

                if x == maj {
                    lfsr1.pop();
                    lfsr1.insert(0,r1);
                }
                if y == maj{
                    lfsr2.pop();
                    lfsr2.insert(0,r2);
                }
                if z == maj{
                    lfsr3.pop();
                    lfsr3.insert(0,r3);
                }
                let bit = x ^ y ^ z;
                vec.push(bit);
                count += 1;
            }
            vec
        }
        pub fn majority(x:u8,y:u8,z:u8) -> u8 {
            if      x == y && x == z {
                x
            }
            else if x == y && x != z{
                x
            }
            else if x == z && x != y{
                x
            }
            else if y == z && y != x{
                y
            }
            else{
                2
            }
        }
        pub fn write_file(vec: Vec<u8>) ->  std::io::Result<()>{
            use std::fs;
            use std::io::Write;
            let mut file = fs::File::create("WUFF.txt")?;
            for i in vec{
                let _write = file.write(&[i]);
            }
            Ok(())
        }
        pub fn write_file_2(vec: Vec<u8>) ->  std::io::Result<()>{
            use std::fs;
            use std::io::Write;
            let mut file = fs::File::create("WUFF2.txt")?;
            for i in vec{
                let _write = file.write(&[i]);
            }
            Ok(())
        }
        pub fn binary(input:&[u8]) -> Vec<u8> {
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
        pub fn to_binary(mut decimal: i64) -> String{
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
    }
    pub mod block_cipher{}
}

mod ciphers;
mod stream;
extern crate bit_vec;
extern crate bitreader;
use std::fs;
use std::io::prelude::*;
use std::env;

fn help(){
    println!("This is the command line interface for old ciphers.
    \nTo operate it, enter:
    \n\n'cargo run -- <'Message to be encrypted'> <Encrypt or Decrypt> <Cipher of your choice><Secondary information according to cipher: '>
    \nCaesar: Shift as integer
    \nMono & Vigenere: Key with only letters");
}
fn main(){
    const alphabet_lower: &str = "abcdefghijklmnopqrstuvwxyz";
    const alphabet_upper: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
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
            let sess_key = &args[2];
            // let mut sk:[u8;64] = [Default::default();64];
            // bla.iter().zip(sk.iter_mut()).map(|(&bla, sk)| *sk = bla).count();
            let frame_counter:[u8;22] = [1,1,1,0,1,0,1,0,1,1,0,0,1,1,1,1,0,0,1,0,1,1];
            println!("{:?},{:?},{:?}",msg,sess_key,frame_counter);
            // let a = &[2];
            // let b = ciphers::modern_ciphers::stream_cipher::binary(a);
            // println!("{:?}",b);

//maybe concat enough together
//maybe slice input into smaller pieces and regroup before writing
//make better functions
//revisit types used
//revisit binary
//memory bittest maybe

            // let (a,b,c) = ciphers::modern_ciphers::stream_cipher::init();
            // // println!("{:?},\n{:?},\n{:?}",a,b,c);
            // let (d,e,f) = ciphers::modern_ciphers::stream_cipher::session_key(sk,a,b,c);
            // println!("{:?},\n{:?},\n{:?}",d,e,f);
            // let (g,h,i) = ciphers::modern_ciphers::stream_cipher::frame_number(frame_counter,d,e,f);
            // println!("\n\n{:?},\n{:?},\n{:?}",g,h,i);
            // let (j,k,l) = ciphers::modern_ciphers::stream_cipher::system_clocking(g,h,i);
            // println!("\n\n{:?},\n{:?},\n{:?}",j,k,l);
            // let m = ciphers::modern_ciphers::stream_cipher::keystream(j,k,l);
            // println!("\n\n{:?},\n {:?}",m,m.len());
            // let n = ciphers::modern_ciphers::stream_cipher::stream(message,m);
            // println!("\n\n{:?}\n\n",n);
            // let p = n.clone();
            // let o = ciphers::modern_ciphers::stream_cipher::write_file(n);
            //
            //
            //
            // let (a1,b1,c1) = ciphers::modern_ciphers::stream_cipher::init();
            // // println!("{:?},\n{:?},\n{:?}",a,b,c);
            // let (d1,e1,f1) = ciphers::modern_ciphers::stream_cipher::session_key(sk,a1,b1,c1);
            // println!("{:?},\n{:?},\n{:?}",d1,e1,f1);
            // let (g1,h1,i1) = ciphers::modern_ciphers::stream_cipher::frame_number(frame_counter,d1,e1,f1);
            // println!("\n\n{:?},\n{:?},\n{:?}",g1,h1,i1);
            // let (j1,k1,l1) = ciphers::modern_ciphers::stream_cipher::system_clocking(g1,h1,i1);
            // println!("\n\n{:?},\n{:?},\n{:?}",j1,k1,l1);
            // let m1= ciphers::modern_ciphers::stream_cipher::keystream(j1,k1,l1);
            // println!("\n\n{:?},\n {:?}",m1,m1.len());
            // let n1 = ciphers::modern_ciphers::stream_cipher::stream(p,m1);
            // println!("\n\n{:?}",n1);
            // let o1 = ciphers::modern_ciphers::stream_cipher::write_file_2(n1);
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

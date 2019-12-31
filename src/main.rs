use std::env;
use std::fs::File;
use std::io;
use std::io::{stdin,stdout,Write};
use std::io::{BufReader,BufRead};
use std::io::BufWriter;
use std::io::prelude::*;
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
fn selection(structure:CipherStruct) -> std::io::Result<()>{
    let action = structure.struct_action as u8;
    let cipher = structure.struct_cipher as u8;
    let cipher_secondary = structure.struct_cipher_secondary;
    let file = structure.struct_file;


    let frame_number:[u8;22] = [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    let mut frame = frame_number.to_vec();
    let sk = cipher_secondary.as_bytes().to_vec();
    println!("{:?}",sk );
    let mut sessionkey = bits(&sk);
    println!("{:?}",sessionkey);
    while sessionkey.len() < 64{
        sessionkey.push(0);
    }
    println!(" Final version: {:?}, {}",sessionkey,sessionkey.len());


    //let FILE = reading_file(file);
    let mut new_file_name = String::new();
    print!("Please enter a filename for your message: ");
    let _=stdout().flush();
    stdin().read_line(&mut new_file_name).expect("Did not manage to create new file!");
    if let Some('\n')=new_file_name.chars().next_back(){
        new_file_name.pop();
    }
    if let Some('\r')=new_file_name.chars().next_back(){
        new_file_name.pop();
    }

    let file = File::open(file)?;
    let new_file = File::create(new_file_name)?;
    let capacity = 228;
    let reader = BufReader::with_capacity(capacity,file);
    let mut buffer = BufWriter::with_capacity(capacity,&new_file);
    for (index,line) in reader.lines().enumerate(){
        let line = line.unwrap();
        //println!("{:?}  {}",index , line );


        if      action == 0 && cipher == 0  {
            //encrypt caes
            if numbers_check(&cipher_secondary) == false{
                println!("Secondary input for caesar is invalid!");
                panic!();
            }
            else {
                let encrypt = cipher_functions::old_ciphers::caesar::caesar_encrypt(line, cipher_secondary.parse().unwrap()).unwrap();
                //println!("{:?}",encrypt);
                let encrypted = encrypt.as_bytes();
                buffer.write(encrypted).unwrap();
                buffer.write(b"\n");
                buffer.flush().unwrap();
            }
        }
        else if action == 1 && cipher == 0  {
            //decrypt caes
            if numbers_check(&cipher_secondary) == false{
                println!("Secondary input for caesar is invalid!");
                panic!();
            }
            else {
                let decrypt = cipher_functions::old_ciphers::caesar::caesar_decrypt(line, cipher_secondary.parse().unwrap()).unwrap();
                //println!("{:?}",encrypt);
                let decrypted = decrypt.as_bytes();
                buffer.write(decrypted).unwrap();
                buffer.write(b"\n");
                buffer.flush().unwrap();
            }
        }
        else if action == 0 && cipher == 1  {
            //encrypt mono
            if message_check_no_space(&cipher_secondary) == false{
                println!("Secondary input for Monoalphabetic cipher is invalid!");
                panic!();
            }
            else {
                let encrypt = cipher_functions::old_ciphers::monoalphabetic_cipher::mono_encrypt(line, cipher_secondary.parse().unwrap()).unwrap();
                //println!("{:?}",encrypt);
                let encrypted = encrypt.as_bytes();
                buffer.write(encrypted).unwrap();
                buffer.write(b"\n");
                buffer.flush().unwrap();
            }
        }
        else if action == 1 && cipher == 1  {
            //decrypt  mono
            if message_check_no_space(&cipher_secondary) == false{
                println!("Secondary input for Monoalphabetic cipher is invalid!");
                panic!();
            }
            else {
                let decrypt = cipher_functions::old_ciphers::monoalphabetic_cipher::mono_decrypt(line, cipher_secondary.parse().unwrap()).unwrap();
                //println!("{:?}",encrypt);
                let decrypted = decrypt.as_bytes();
                buffer.write(decrypted).unwrap();
                buffer.write(b"\n");
                buffer.flush().unwrap();
            }
        }
        else if action == 0 && cipher == 2  {
            //encrypt vigenere
            if message_check_no_space(&cipher_secondary) == false{
                println!("Secondary input for Vigenere is invalid!");
                panic!();
            }
            else {
                let encrypt = cipher_functions::old_ciphers::polyalphabetic_cipher::poly_encrypt(line, cipher_secondary.parse().unwrap()).unwrap();
                //println!("{:?}",encrypt);
                let encrypted = encrypt.as_bytes();
                buffer.write(encrypted).unwrap();
                buffer.write(b"\n");
                buffer.flush().unwrap();
            }
        }
        else if action == 1 && cipher == 2  {
            //decrypt vigenere
            if message_check_no_space(&cipher_secondary) == false{
                println!("Secondary input for Vigenere cipher is invalid!");
                panic!();
            }
            else {
                let decrypt = cipher_functions::old_ciphers::polyalphabetic_cipher::poly_decrypt(line, cipher_secondary.parse().unwrap()).unwrap();
                //println!("{:?}",encrypt);
                let decrypted = decrypt.as_bytes();
                buffer.write(decrypted).unwrap();
                buffer.write(b"\n");
                buffer.flush().unwrap();
            }
        }
        else if cipher == 3  {
            //encrypt stream
            let mut x = decimal_from_bits(&frame);
            println!("{:?}", x);
            x = (x + 1)%4194304;
            frame = to_bits(x);
            println!("FRAME {:?}",frame);



            //bits left or right beginning figure out
            //xor and write
            if action == 0{
                let encrypt = cipher_functions::modern_ciphers::stream_cipher::a5_stream_cipher(sessionkey.clone(),frame.clone(),line.len()*8);
                println!("Stream key:\n {:?} {}", encrypt,encrypt.len());

                println!("XXXXX {:?}",line);
                let encrypted = stream_write(encrypt,line);
                println!("ENC: \n{:?}",encrypted );
                let encrypted_bytes = blabla(encrypted);
                println!("Bytes: {:?}", encrypted_bytes);
                for i in encrypted_bytes{
                    buffer.write(&[i]).unwrap();
                    buffer.write(b"\n");
                }
                buffer.flush().unwrap();
                // buffer.write(encrypted_bytes).unwrap();
                // buffer.flush().unwrap();
                //let encrypted = encrypt;//.as_bytes();
                //buffer.write(&encrypted).unwrap();
                // buffer.write(b"\n");
                //buffer.flush().unwrap();

            }
            else if action == 1{

            }
        }
        else{
            println!("The selected cipher has not yet been implemented!" );
            panic!();
        }
    }
    Ok(())
}
fn blabla(vec: Vec<u8>)-> Vec<u8>{
    let mut bytes_vec = Vec::new();
    for i in 0..(vec.len()/8){
        let mut res = 0;
        for j in 0..8{
            if vec[i*8+j] == 1{
                res += 2_u8.pow(7-j as u32);
            }
        }
        bytes_vec.push(res);
    }
    bytes_vec
}
fn stream_write(stream: Vec<u8>, line: String) -> Vec<u8>{
    let mut vec = Vec::new();
    let line_as_bits = bits(line.as_bytes());
    println!("{:?}\n {:?}", line.as_bytes(),line_as_bits);
    let mut it = stream.iter().cycle();
    for i in line_as_bits{
        let r = i^it.next().unwrap();
        vec.push(r);
    }
    vec
}
fn to_bits(input: u32) -> Vec<u8>{
    let mut vec = Vec::new();
    let mut j = input.clone();
    while j>=1{
        //println!("J: {:?}", j);
        if j%2 == 0{
            vec.push(0);
        }else{
            vec.push(1);
        }
        j/=2;
    }
    while vec.len()<22{
         vec.push(0);
    }
    vec
}
fn decimal_from_bits(input: &Vec<u8>) -> u32{
    let mut res = 0;
    let mut j = 0;
    for i in input{
        //println!("XYZ {:?} {}", i, j);
        if i == &1{
            res += 2_u32.pow(j);
        }
        j+=1;
    }
    res
}
fn bits(input: &[u8]) ->Vec<u8>{
    let mut vec = Vec::new();
    for i in input{
        let mut vec2 = Vec::new();
        let mut j = i.clone();
        while j>0{
            if j%2 == 0{
                vec2.push(0);
            }else{
                vec2.push(1);
            }
            j/=2;
        }
        while vec2.len()%8 != 0{
            vec2.push(0);
        }
        //println!("{:?}", vec2);
        vec2.reverse();
        //println!("{:?}", vec2);
        for i in vec2{
            vec.push(i);
        }
    }
    vec
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
fn initial_message(){
    println!("Message encryptor/decryptor using a command line interface.
    \nUsage:       cargo run -- [encrypt/decrypt] [cipher] [cipher specific secondary input] [filename]");
}
fn help(){

}
/*Everything in functions
buffer read write
add frame number to other file
jpeg implementation



SK is right to left

LFSR and FRAME are left to right
*/






// fn reading_file(file_name: String) -> std::io::Result<()>{
//     let file = File::open(file_name)?;
//     let reader = BufReader::new(file);
//
//     for (index,line) in reader.lines().enumerate(){
//         let line = line.unwrap();
//         println!("{:?}  {}",index , line );
//
//     }
//     Ok(())
// }
// fn reading_file(file_name: String) -> std::io::Result<()>{
//     let file = File::open(file_name)?;
//     let mut reader = BufReader::with_capacity(10,file);
//     for line in reader.lines(){
//         println!("{:?}",line.unwrap() );
//     }
//     Ok(())
// }
// fn reading_file(file_name: String) -> std::io::Result<()>{
//     let stdin = io::stdin();
//     let mut stdin = stdin.lock();
//
//     let buffer = stdin.fill_buf().unwrap();
//
// // work with buffer
//     println!("{:?}", buffer);
//
// // ensure the bytes we worked with aren't returned again later
//     let length = buffer.len();
//     stdin.consume(length);
//     Ok(())
// }
// fn writiting_file(){
//
// }

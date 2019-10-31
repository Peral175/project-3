//
// pub mod modern_ciphers{
// //quelle
//     pub fn to_binary(mut decimal: i32) -> i32{
//         if decimal == 0{
//             decimal
//         }else{
//             let mut bits = String::new();
//             while decimal > 0 {
//                 if decimal % 2 == 0{
//                     bits.push_str("0");
//                 }else {
//                     bits.push_str("1");
//                 }
//                 decimal /= 2;
//             }
//             match bits.chars().rev().collect::<String>().parse() {
//                 Ok(num) => num,
//                 Err(_e) => panic!("Something went wrong in the binary transformation!"),
//             }
//         }
//     }
//     pub mod stream_cipher{
//         pub fn encrypt_stream(msg: &str, key: &str) -> Option<String>{
//             let msg_as_bytes = msg.as_bytes();
//             let key_as_bytes = key.as_bytes();
//             let mut vec = Vec::new();
//             println!("Arrays as decimal: {:?}, {:?}", msg_as_bytes, key_as_bytes);
// //
//             let mut v = Vec::new();
//             let mut w = Vec::new();
//             for i in msg_as_bytes{
//                 let x = crate::stream::modern_ciphers::to_binary(*i as i32);
//                 v.push(x);
//             }
//
//             for i in key_as_bytes{
//                 let y = crate::stream::modern_ciphers::to_binary(*i as i32);
//                 w.push(y);
//             }
//             println!("Arrays as binary: {:?} {:?}", v, w);
// //
//             for i in 0..msg_as_bytes.len(){
//                 println!("i: {}",i);
//                 let res = msg_as_bytes[i] ^ key_as_bytes[i];
//                 println!("XOR: {}\nNew bit: {:b}",res, res);
//                 vec.push(res);
//             }
//             println!("{:?}",vec);
//             let mut result = String::new();
//             for i in vec{
//                 result.push(i as char);
//             }
//         //     let mut it = key_as_bytes.iter();
//         //     let mut vec = Vec::new();
//         //     for i in msg_as_bytes{
//         //         println!("{}", i);
//         //         let its = it.next().unwrap();
//         //         println!("{:?}",its);
//         //         let mut res = i ^ its;
//         //         println!("R: {:?}, {:b}", res, res);
//         //         vec.push(res);
//         //
//         //     }
//         //     println!("{:?}",vec);
//         //     let mut result = String::new();
//         //     for i in vec{
//         //         println!("{}, {:b}",i, i);
//         //         let x = i as char;
//         //         println!("{:?}, {:b}",x, i);
//         //         result.push(x);
//         //     }
//             return Some(result)
//         }
//         //pub fn decrypt_stream(){}
//     }
//     pub mod block_cipher{}
// }


pub mod modern_ciphers{
    pub mod stream_cipher{
        use std::env;
        use std::fs;
        use std::io::prelude::*;

        pub fn encrypt_stream<'a>(msg: &'a str, content: &'a str) -> std::io::Result<&'a str>{

            println!("Text: \n{}\nKey:\n{}",msg,content);
            let content_byte = content.as_bytes();
            println!("{:?}",content_byte);
            let msg_as_bytes = msg.as_bytes();
            println!("{:?} {}", msg_as_bytes, msg_as_bytes.len());
            let filename = "file01.txt";
            let mut file = fs::File::create(filename)?;


            let mut it = content_byte.iter().cycle();

            for i in 0.. msg_as_bytes.len(){
                // let res = msg_as_bytes[i]^content_byte[i];
                let z = it.next().unwrap();
                let res = msg_as_bytes[i] ^ z;
                let a = file.write_all(&[res]);

            }
            Ok(filename)

        }
        pub fn decrypt_stream(msg_de: &str, contents: &str) -> std::io::Result<()>{

            println!("Key: \n{}",contents);
            let contents_byte = contents.as_bytes();
            let message = fs::read_to_string(msg_de).expect("Something went wrong!");
            let msg_as_bytes = message.as_bytes();
            let filename = "file02.txt";
            let mut file = fs::File::create(filename)?;
            let mut it = contents_byte.iter().cycle();
            for i in 0..msg_as_bytes.len(){
                //let res = msg_as_bytes[i] ^ contents_byte[i];
                let z = it.next().unwrap();
                let res = msg_as_bytes[i] ^ z;
                let a = file.write_all(&[res]);

            }
            Ok(())
        }
    }
}
// create cycle for keylength less than message length
// test cases
// catch errors
// text file returned needs handling
//file or user input as arguments for the functions
// -- maybe read file already in main.rs and always pass the function the string contained in the file

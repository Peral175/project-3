
pub mod modern_ciphers{

    pub fn to_binary(mut decimal: i32) -> i32{
        if decimal == 0{
            decimal
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
            match bits.chars().rev().collect::<String>().parse() {
                Ok(num) => num,
                Err(_e) => panic!("Something went wrong in the binary transformation!"),
            }
        }
    }
    pub mod stream_cipher{
        pub fn encrypt_stream(msg: &str, key: &str) -> Option<String>{
            let msg_as_bytes = msg.as_bytes();
            let key_as_bytes = key.as_bytes();
            println!("{:?}, {:?}", msg_as_bytes, key_as_bytes);
            let mut it = key_as_bytes.iter();
            let mut vec = Vec::new();
            for i in msg_as_bytes{
                println!("{}", i);
                let its = it.next().unwrap();
                println!("{:?}",its);
                let mut res = i ^ its;
                println!("R: {:?}, {:b}", res, res);
                vec.push(res);

            }
            println!("{:?}",vec);
            let mut result = String::new();
            for i in vec{
                println!("{}, {:b}",i, i);
                let x = i as char;
                println!("{:?}, {:b}",x, i);
                result.push(x);
            }
            return Some(result)
        }
        pub fn decrypt_stream(){}
    }
    pub mod block_cipher{}
}

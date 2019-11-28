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
    }
    pub mod block_cipher{}
}

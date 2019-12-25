pub mod old_ciphers{
    pub mod caesar{
        pub fn caesar_encrypt(message: String, shift: u8) -> Option<String>{
            let mut encrypted_message = String::new();

            for i in message.as_bytes(){
                if (i >= &65) && (i<= &90){
                    let mut r = *i - 65;
                    r = (r + shift)%26;
                    r = r + 65;
                    encrypted_message.push(r as char);
                }
                else if (i >= &97) && (i <= &122){
                    let mut r = *i - 97;
                    r = (r + shift)%26;
                    r = r + 97;
                    encrypted_message.push(r as char);
                }
                else{
                    encrypted_message.push(*i as char);
                }
            }
            return Some(encrypted_message)
        }
        pub fn caesar_decrypt(message: String, shift: u8) -> Option<String>{
            let mut encrypted_message = String::new();

            for i in message.as_bytes(){
                if (i >= &65) && (i<= &90){
                    let mut r = *i - 65;
                    r = (r +26 - shift)%26;
                    r = r + 65;
                    encrypted_message.push(r as char);
                }
                else if (i >= &97) && (i <= &122){
                    let mut r = *i - 97;
                    r = (r + 26 -shift)%26;
                    r = r + 97;
                    encrypted_message.push(r as char);
                }
                else{
                    encrypted_message.push(*i as char);
                }
            }
            return Some(encrypted_message)
        }
    }
    pub mod monoalphabetic_cipher{
        pub fn mono_encrypt(message: String, secret: String) -> Option<String>{
            secret.to_lowercase();
            let mut encrypted_message = String::new();
            let (_secret_alphabet, secret_vector) = secret_alphabet_function(secret);

            for i in message.as_bytes(){

                if (i >= &65) && (i<= &90){

                    let position_in_secret_alphabet = secret_vector.iter().position(|x| i.clone().to_ascii_lowercase() == *x).unwrap();
                    let _error_message= format!("No elements found at that index {}",position_in_secret_alphabet);
                    let new_value = ((position_in_secret_alphabet as u8) %26) +65;
                    encrypted_message.push(new_value as char);

                }else if (i >= &97) && (i <= &122){

                    let position_in_secret_alphabet = secret_vector.iter().position(|x| i.clone().to_ascii_lowercase() == *x).unwrap();
                    let _error_message= format!("No elements found at that index {}",position_in_secret_alphabet);
                    let new_value = ((position_in_secret_alphabet as u8) %26) +97;
                    encrypted_message.push(new_value as char);
                }
                else{
                    encrypted_message.push(*i as char);
                }
            }
            return Some(encrypted_message)

        }
        pub fn mono_decrypt(message: String, secret: String) -> Option<String>{
            secret.to_lowercase();
            let mut decrypted_message = String::new();
            let (_secret_alphabet, secret_vector) = secret_alphabet_function(secret);
            let alphabet_lower = "abcdefghijklmnopqrstuvwxyz".as_bytes().to_vec();
            for i in message.as_bytes(){

                if (i >= &65) && (i<= &90){
                    let position_in_alphabet = alphabet_lower.iter().position(|x| i.clone().to_ascii_lowercase() == *x).unwrap();
                    let _error_message= format!("No elements found at that index {}",position_in_alphabet);

                    let new_value = secret_vector[position_in_alphabet] - 32;
                    decrypted_message.push(new_value as char);

                }else if (i >= &97) && (i <= &122){

                    let position_in_alphabet = alphabet_lower.iter().position(|x| i.clone().to_ascii_lowercase() == *x).unwrap();
                    let _error_message= format!("No elements found at that index {}",position_in_alphabet);
                    let new_value = secret_vector[position_in_alphabet];
                    decrypted_message.push(new_value as char);
                }
                else{
                    decrypted_message.push(*i as char);
                }
            }
            return Some(decrypted_message)

        }
        fn secret_alphabet_function(secret: String) -> (String,Vec<u8>){
            let mut secret_alphabet = String::new();
            let mut vec = Vec::new();
            for i in secret.as_bytes(){
                if (vec.contains(i)) == false{
                    vec.push(*i);
                }
            }
            let mut iter = 97;
            while iter <= 122{
                if (vec.contains(&iter)) == false{
                    vec.push(iter);
                }
                iter += 1;
            }
            for i in &vec{
                secret_alphabet.push(*i as char);
            }
            return (secret_alphabet,vec)
        }
    }
    pub mod polyalphabetic_cipher{
        pub fn poly_encrypt(message: String,key: String) -> Option<String>{
            let mut encrypted_message = String::new();
            let mut repeating_key = Vec::new();
            for i in key.chars(){
                if i.is_lowercase(){
                    repeating_key.push(i as usize -97);
                }
                else if i.is_uppercase(){
                    repeating_key.push(i as usize -65);
                }
            }
            let mut cycle = repeating_key.iter().cycle();
            for i in message.bytes(){
                let shift = *cycle.next()?;
                if (i >= 65) && (i<= 90){
                    let mut r = i - 65;
                    r = (r + shift as u8)%26;
                    r = r + 65;
                    encrypted_message.push(r as char);
                }
                else if (i >= 97) && (i <= 122){
                    let mut r = i - 97;
                    r = (r + shift as u8)%26;
                    r = r + 97;
                    encrypted_message.push(r as char);
                }
                else{
                    encrypted_message.push(i as char);
                }
            }
            return Some(encrypted_message)
        }
        pub fn poly_decrypt(message: String,key: String) -> Option<String>{
            let mut encrypted_message = String::new();
            let mut repeating_key = Vec::new();
            for i in key.chars(){
                if i.is_lowercase(){
                    repeating_key.push(i as usize -97);
                }
                else if i.is_uppercase(){
                    repeating_key.push(i as usize -65);
                }
            }
            let mut cycle = repeating_key.iter().cycle();
            for i in message.bytes(){
                let shift = *cycle.next()?;
                if (i >= 65) && (i<= 90){
                    let mut r = i - 65;
                    r = (r + 26 -shift as u8)%26;
                    r = r + 65;
                    encrypted_message.push(r as char);
                }
                else if (i >= 97) && (i <= 122){
                    let mut r = i - 97;
                    r = (r + 26 - shift as u8)%26;
                    r = r + 97;
                    encrypted_message.push(r as char);
                }
                else{
                    encrypted_message.push(i as char);
                }
            }
            return Some(encrypted_message)
        }
    }
}
pub mod modern_ciphers{
    pub mod stream_cipher{
        fn init() -> (Vec<u8>,Vec<u8>,Vec<u8>) {
            let lfsr1: Vec<u8> = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0].to_vec();
            let lfsr2: Vec<u8> = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0].to_vec();
            let lfsr3: Vec<u8> = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0].to_vec();
            (lfsr1,lfsr2,lfsr3)
        }
        fn mixing(sk:Vec<u8>,mut lfsr1:Vec<u8>, mut lfsr2:Vec<u8>, mut lfsr3:Vec<u8>) -> (Vec<u8>,Vec<u8>,Vec<u8>){
            for i in sk.iter(){
                /* Here the bits of the vector that are going to be xored are captured
                For each cycle these change*/
                let first_xor = lfsr1[13] ^ lfsr1[16] ^ lfsr1[17] ^ lfsr1[18];
                let second_xor = lfsr2[20] ^ lfsr2[21];
                let third_xor = lfsr3[7] ^ lfsr3[20] ^ lfsr3[21] ^ lfsr3[22];
                /* The captured bits are combined with the sessionkey */
                let r1 = first_xor ^ i ;
                let r2 = second_xor ^ i;
                let r3 = third_xor ^ i;

                lfsr1.iter().cycle().next().unwrap();
                lfsr2.iter().cycle().next().unwrap();
                lfsr3.iter().cycle().next().unwrap();
                /*We then delete the last element of the vector and insert our r1,r2,r3 at the first position of the respetive LFSR*/
                lfsr1.pop();
                lfsr1.insert(0,r1);
                lfsr2.pop();
                lfsr2.insert(0,r2);
                lfsr3.pop();
                lfsr3.insert(0,r3);
            }
            (lfsr1,lfsr2,lfsr3)
        }
        fn cycling(mut lfsr1: Vec<u8>,mut lfsr2: Vec<u8>,mut lfsr3: Vec<u8>) -> (Vec<u8>,Vec<u8>,Vec<u8>){
            for _i in 0..100{
                let first_xor = lfsr1[13] ^ lfsr1[16] ^ lfsr1[17] ^ lfsr1[18];
                let second_xor = lfsr2[20] ^ lfsr2[21];
                let third_xor = lfsr3[7] ^ lfsr3[20] ^ lfsr3[21] ^ lfsr3[22];

                let r1 = first_xor;
                let r2 = second_xor;
                let r3 = third_xor;

                lfsr1.iter().cycle().next().unwrap();
                lfsr2.iter().cycle().next().unwrap();
                lfsr3.iter().cycle().next().unwrap();
                /*Here is the irregular clocking.
                The bits 8,10 and 10 are captured and it is determined via the majority function, which bit appears most often.
                Those LFSR which have the bit that appears most often perform as before the xor operation, while the other one does nothing*/
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
            }
            (lfsr1,lfsr2,lfsr3)
        }
        fn stream_key(msg_length_bits:usize,mut lfsr1: Vec<u8>,mut lfsr2: Vec<u8>,mut lfsr3: Vec<u8>) -> Vec<u8>{
            let mut vec = Vec::new();
            for _i in 0..msg_length_bits{
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
            }
            vec
        }
        fn majority(x:u8,y:u8,z:u8) -> u8 {
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
        pub fn a5_stream_cipher(sk:Vec<u8>,frame:Vec<u8>,msg_length_bits:usize) -> Vec<u8>{
            let (a1,a2,a3) = init();
            let (b1,b2,b3) = mixing(sk,a1,a2,a3);
            let (c1,c2,c3) = mixing(frame,b1,b2,b3);
            let (d1,d2,d3) = cycling(c1,c2,c3);
            let stream = stream_key(msg_length_bits,d1,d2,d3);
            stream
        }
    }
}
//still need to add frame number

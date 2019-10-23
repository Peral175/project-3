pub mod old_ciphers{
    pub mod caesar_cipher{
        pub fn encrypt_caes(msg: &str, sec: String, alphabet_lower: &str, alphabet_upper: &str ) -> Option<String>{
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
        pub fn decrypt_caes(msg: &str, sec: String, alphabet_lower: &str, alphabet_upper: &str ) -> Option<String>{
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
                    let idx: usize = (x - shift as usize) % 26;
                    let err_s = format!("No element at index {}", idx);
                    let y = alphabet.chars().nth(idx).expect(&err_s);
                    result.push(y);
                }
            }
            return Some(result)
        }
    }
    pub mod mono_substitution_cipher{
        pub fn encrypt_mono(msg: &str, sec: String, alphabet_lower: &str, alphabet_upper: &str ) -> Option<String>{
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
        pub fn decrypt_mono(msg: &str, sec: String, alphabet_lower: &str, alphabet_upper: &str ) -> Option<String>{
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
        pub fn encrypt_vige(msg: &str, sec: String, alphabet_lower: &str, alphabet_upper: &str ) -> Option<String>{
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
        pub fn decrypt_vige(msg: &str, sec: String, alphabet_lower: &str, alphabet_upper: &str ) -> Option<String>{
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
                    let idx: usize = (x - next) % 26;
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
    pub mod stream_cipher{}
    pub mod block_cipher{}
}

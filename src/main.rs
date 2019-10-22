
fn main() {
    // let message = "ABC abc";
    // let key = "abcdefg";
    // let message_as_bytes = message.as_bytes();
    // let key_as_bytes = key.as_bytes();
    // println!("{:?}{:?}", message_as_bytes,key_as_bytes);
    //
    // let mut it = key_as_bytes.iter();
    // for i in message_as_bytes{
    //     println!("{}",i);
    //     let x = format!(" {:#b}",i);
    //     println!("{}",x);
    //
    //
    //     let next = it.next().unwrap();
    //     println!("AAAAAA {:?}",next);
    //     let next_as_bits = format!("{:#b}",next);
    //     println!("AAAAAA {}",next_as_bits);
    //
    //     // let result = (x.unwrap()) ^ (next_as_bits);
    //     // println!("Result: {}", result);
    //
    // }
//     let message = "abc";
//     let key = "aaa";
//     let message_as_bytes = message.as_bytes();
//     let key_as_bytes = key.as_bytes();
//     let mut result:String = "".to_string();
//     let mut it = key_as_bytes.iter();
//     for i in message_as_bytes{
//         println!("{}",i);
//         let bin = to_binary(*i as i32);
//         println!("Bin: {}",bin);
//
//         let next = it.next().unwrap();
//         println!("AAAAAA {:?}",next);
//         let bin_2 = to_binary(*next as i32);
//         println!("Bin_2: {}", bin_2);
//         let bii = bin_2.to_string();
//         let mut bi = bii.chars();
//
//         for i in bin.to_string().chars(){
//             println!("XXX: {}",i);
//         }
//         println!("YYYYY: {:?}",bi);
//         let nom = bi.next().unwrap();
//         println!("ZZZ: {}",nom);
//
//             // let r = (i as i32 + bi )%2;
//             // result.push(r);
//
//
//         println!("Res: {}", result);
//     }
//
// }
    let message = "abc";
    //1100001
    //1100010
    //1100011
    //         110000111000101100011
    let key = "110000111000011100001";
    //         000000000000110000010
    let message_as_bytes = message.as_bytes();
    let key_as_bytes = key.as_bytes();
    let mut vec = Vec::new();
    let mut vec_2 = Vec::new();
    let mut result = Vec::new();
    for i in message_as_bytes{
        println!("{}",i);
        let bin = to_binary(*i as i32);
        println!("Bin: {:?}",bin);
        vec.push(bin);
    }
    println!("{:?}",vec);

    for i in vec{
        println!("{}",i);
        let k = i.to_string();
        for j in k.chars(){
            println!("{}",j);
            vec_2.push(j);
        }
    }
    println!("{:?}",vec_2);
    let mut it = vec_2.iter();

    for i in key.chars(){
        let its = it.next().unwrap();
        println!("{}",i);
        println!("{:?}",its);
        let mut res = (i as u32 + *its as u32)%2;
        println!("AAAAA: {}",res);
        result.push(res);
        println!("{:?}",result);
        // println!("{:?}",it);

        // result.push((i + it)%2);

    }
    let mut nomnom = "".to_string();
    for i in result{
        nomnom.push_str(&i.to_string());

    }
    println!("{}",nomnom);
    if nomnom == "000000000000110000010"{
        println!("Success");
    }
}
fn to_binary(mut decimal: i32) -> i32 {
    if decimal == 0 {
        decimal
    } else {
        let mut bits = String::new();

        while decimal > 0 {
            if decimal % 2 == 0 {
                bits.push_str("0");
            } else {
                bits.push_str("1");
            }

            decimal /= 2;
        }

        // reverse the bits
        match bits.chars().rev().collect::<String>().parse() {
            Ok(num) => num,
            Err(_e) => panic!("Something went wrong"),
        }
    }
}

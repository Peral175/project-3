// extern crate clap;
// use clap::App;
//
// fn main() {
//     App::new("meng_app")
//        .version("1.0")
//        .about("clap_test")
//        .author("Perrard Alex.")
//        .get_matches();
// }
fn main() {
    let message = "ABCabcXYZxyzHello.! Ü";
    println!("{:?}", message.as_bytes());
    let message_as_bytes = message.as_bytes();
    // let message_as_bits = format!("{:b}",&message_as_bytes);
    // println!("{:?}",message_as_bits);
    let mut vec: Vec<String> = Vec::new();
    for i in message_as_bytes{
        println!("{}",i);
        let message_as_bits = format!("{:b}",i);
        println!("{}",message_as_bits);
        vec.push(message_as_bits);
    }
    println!("{:?}",vec);
    let mut concat:String = "".to_string();
    for i in vec{
        println!("{}",concat);
        println!("{}",i);
        concat.push_str(&i);
    }
    println!("BINARY MESSAGE: {}",concat);


    let key = "ABCDEF";
    let key_as_bytes = key.as_bytes();
    let mut vec_2: Vec<String> = Vec:: new();
    for i in key_as_bytes{
        let key_as_bytes = format!("{:b}",i);
        vec_2.push(key_as_bytes);
    }
    let mut concate: String = "".to_string();
    for i in vec_2{
        concate.push_str(&i);
    }
    println!("BINARY KEY: {}",concate);

    println!("BLABLA: {}",concat & concate);
}

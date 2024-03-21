// // use std::error::Error;
// // use csv;
// // use std::io;
// // use std::fs::File;
// // use std::error::Error;
use std::{error::Error,io, process};

use csv::StringRecord;
// fn main() -> Result<(), Error> {
// //    tracer(String::from("7jlncfksix7rjgrpglmn9"));
// //let file = File::open("Book1.csv");
// //let file = std::io::Read("Book1.csv");  
// let file_exp = file::open()
// let mut file = csv::Reader::from_path("Book1.csv");
// //let file_read = file::Read();
// // let mut reader  = csv::ReaderBuilder::new().from_reader(std::io::stdin(file));
// for record in file?.records() {
//     let string_result = record?;
//     println!("{:?}", string_result);
// }
// let mut sum:i32 = 0;

// let nums_in_str: (i32,i32) = tracer(String::from("fx3"));
// println!("{:?}",nums_in_str);
// }
// fn tracer(characters:String) -> (i32,i32){
//     let mut nums:Vec<i32> = Vec::new();
//     for i in characters.chars(){
//         if i.is_numeric(){
//             let my_i32 : i32 = i.to_string().parse().unwrap();
//             nums.push(my_i32);
//         }
//     }
// (nums[0],nums[nums.len()-1])

fn number_selector(string:StringRecord) -> (i32,i32){
    let mut nums:Vec<i32> = Vec::new();
    let str_slice = string.as_slice();
    for i in str_slice.chars(){
        if i.is_numeric(){
            let my_i32 : i32 = i.to_string().parse().unwrap();
            nums.push(my_i32);
        }
    }
    (nums[0],nums[nums.len()-1])
    
}

fn example_reader() -> Result<(), Box<dyn Error>>{ // the Result return type is a way to handle errors in rust
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut _summer: i32 = 79;
    for result in rdr.records(){
        let record = result?;
        println!("{:?}", record);
        let num_tuple = number_selector(record);
        println!("{:?}", num_tuple);
        let fuck = (num_tuple.0*10) + num_tuple.1;
        _summer += fuck;
        }
    println!("{}",_summer);
    Ok(())  
}


fn main(){
    if let Err(err) = example_reader(){
        println!("error running example {}", err);
        process::exit(1);
        
    }
}
// // fn random(){
// //     let xs: [i32; 5] = [1, 2, 3, 4, 5];
// //     println!("{}, {}",xs[0],xs.len());
// // }

// fn main() {
//     use doe::keyboard::listen_keybord;
//     use doe::mouse::listen_mouse_position;
//     use doe::mouse::listen_mouse_scroll;
//     use doe::mouse::listen_mouse_click;
//     let t1 = std::thread::spawn(||{
//         listen_keybord();
//     });
//     let t2 = std::thread::spawn(||{
//         listen_mouse_position();
//     });
//     let t3 = std::thread::spawn(||{
//         listen_mouse_scroll();
//     });
//     let t4 = std::thread::spawn(||{
//         listen_mouse_click();
//     });
//     t1.join().unwrap();
//     t2.join().unwrap();
//     t3.join().unwrap();
//     t4.join().unwrap();
// }


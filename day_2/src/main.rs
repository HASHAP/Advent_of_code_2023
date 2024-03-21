use csv::{self, StringRecord};
use std::error::Error;
#[derive(Debug)]
struct COLOR{
    index: i32,
    red: i8,
    green: i8,
    blue: i8,
}
impl COLOR{
    fn final_product(mut nums: COLOR) -> i64{
        nums.red = greater_than(1, nums.red);
        nums.green = greater_than(1, nums.green);
        nums.blue = greater_than(1, nums.blue);
        println!("nums.green  = {}", nums.green);
        println!("nums.red    = {}", nums.red);
        println!("nums.blue   = {}", nums.blue);
        let result = (nums.red as i64) * (nums.green as i64) * (nums.blue as i64);
        if result > i64::MAX {
            panic!("Overflow!");
        }
        return result;
    
    }
}
impl COLOR{
    fn regular_check(colors: &Vec<COLOR>) -> bool{
        let color_vec_len = colors.len() as i32;
        let mut temp_sum  = 0;
        for i in 0..color_vec_len as usize{
            if colors[i].red < 13 && colors[i].green < 14 && colors[i].blue < 15 {
                temp_sum += 1;
            }
        }
        if temp_sum == color_vec_len as i32{
            return true;
    }else{
        return false;
    }
}
}
fn main(){
    let mut all_vec: Vec<StringRecord> = Vec::new();
    if let Err(e) = read_from_file("input.csv", &mut all_vec){
        eprintln!("{}", e); //meant for printing errors, output goes to io::stderr insead of io::stdout
    }
    let mut value_bag:Vec<Vec<COLOR>> = vec![];
    for i in 0..all_vec.len(){
        value_bag.push(num_idenitity(text_splitter_updated(&all_vec[i])));
    }
    let final_sum = authenticator(&value_bag);
    let final_product_sum = lowest_num_product(&value_bag);
    println!("the final sum for part 1 is :{:?}",final_sum + 1);
    println!("the final sum for part 2 is :{:?}",final_product_sum + 108);

}
fn read_from_file(path: &str, all_vec: &mut Vec<StringRecord>) -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.records(){
        let record = result?;
        all_vec.push(record);
    }
    Ok(())
}

fn text_splitter(line_str: StringRecord) -> Vec<Vec<String>>{
    let mut text_split :Vec<Vec<String>> = Vec::new();
    for i in 0..3{
        // let org_text: String = String::from("Game 1: 2 blue, 3 red; 3 green, 3 blue, 6 red; 4 blue, 6 red; 2 green, 2 blue, 9 red; 2 red, 4 blue");
        // let org_text_1: String= String::from(&org_text[7..]);
        let org_text: String = line_str.as_slice().to_string(); 
        let org_text_split = &org_text[8..].split(";").take(3).collect::<Vec<_>>()[i].to_string();
        let mut comma_split: Vec<String>  = Vec::new();
        for i in 0..3{
            let splits = org_text_split.split(",").take(3).collect::<Vec<_>>();
        if i < splits.len() {
            let temp = splits[i].to_string();
            comma_split.push(temp);
            }
        }
        text_split.push(comma_split);
    }
    text_split
}
fn num_idenitity(individual_text:Vec<Vec<String>>) -> Vec<COLOR>{
    let mut individual_game_counter: Vec<COLOR> = Vec::new();
    let mut game_counter:i32 = 0;
    for individual_game in individual_text{
        game_counter += 1;
        let mut color = COLOR {
            index: game_counter,
            red: 0,
            green: 0,
            blue: 0,
        };
        for turn in individual_game{
            // println!("{}",turn);
            let turn_number_text :String = turn.chars().filter(|a| a.is_numeric()).collect();
            let turn_number:i8 = turn_number_text.parse().unwrap();
            // println!("{:?}",&turn_number);
            let number_color: String =  turn.chars().filter(|b| b.is_alphabetic()).collect();
            match  number_color.as_str(){
                "red" => color.red += &turn_number,
                "green" => color.green += &turn_number,
                "blue" => color.blue += &turn_number,
                _ => (),
                    };
                }
        individual_game_counter.push(color)
    }
    individual_game_counter
}
fn authenticator(draw: &Vec<Vec<COLOR>>) -> i64{
    let mut sum: i64 = 0;
    for i in 0..draw.len(){
        // println!("{:?}", &draw[i]);
        if COLOR::regular_check(&draw[i]){
            sum += (i+2) as i64;
            println!("{}", &i +2);
        }
    }
    sum
}

fn lowest_num_product(draw: &Vec<Vec<COLOR>>) -> i64{
    let mut final_product:i64 = 0;
    let mut lowest_vec: Vec<COLOR> = Vec::new();
    for i in 0..draw.len(){
        println!("{:?}", &draw[i]);
        let mut color_lowest = COLOR{
            index: 0,
            red: 0,
            green: 0,
            blue: 0,
        };
        for j in 0..draw[i].len(){
            color_lowest.red = greater_than(color_lowest.red, draw[i][j].red);
            color_lowest.green = greater_than(color_lowest.green, draw[i][j].green);
            color_lowest.blue = greater_than(color_lowest.blue, draw[i][j].blue);
        }
        println!("{:?}", &color_lowest);
        final_product += COLOR::final_product(color_lowest) as i64;
    }
    final_product
}

fn text_splitter_updated(line_str: &StringRecord) -> Vec<Vec<String>> {
    let mut text_split :Vec<Vec<String>> = Vec::new();
    let org_text: String = line_str.as_slice().to_string(); 
    let org_text_split: Vec<&str> = org_text[8..].split(";").collect();
    for draw in org_text_split {
        let mut comma_split: Vec<String>  = Vec::new();
        let splits: Vec<&str> = draw.split(",").collect();
        for i in 0..splits.len() {
            let temp = splits[i].trim().to_string();
            comma_split.push(temp);
        }
        text_split.push(comma_split);
    }
    text_split
}
fn greater_than(num1:i8, num2:i8) -> i8{
    if num1 > num2{
        num1
    }
    else{
        num2
    }
}
impl COLOR{
    fn verification_using_sum(colors: &Vec<COLOR>) -> bool{
    let mut total_red = 0;
    let mut total_green = 0;
    let mut total_blue = 0;
    for color in colors{
        total_red += color.red;
        total_green += color.green;
        total_blue += color.blue;
        }
    if total_red < 13 && total_green < 14 && total_blue < 15 {
        println!("{:?}", &colors);
        true}else{
        false}
    }
}
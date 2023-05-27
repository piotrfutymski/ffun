use colored::{ColoredString};
use rand::Rng;
use colored::Colorize;

fn generate_word_base(probabilities: &Vec<Vec<f64>>, length: usize) -> Vec<usize>{
    let mut res = Vec::new();
    res.push(0);
    let mut rng = rand::thread_rng();

    while res.len() < length {
        let random_value = rng.gen::<f64>();
        let last = *res.last().unwrap() as usize;
        let row = &probabilities[last];
        let mut index = 0;
        while row[index] < random_value {
            index+=1;
        }
        res.push(index);
    }
    res
}

pub fn generate_word(tokens: &Vec<char>, probabilities: &Vec<Vec<f64>>, length: usize) -> Vec<char>{
    generate_word_base(probabilities, length)
        .into_iter()
        .map(|e|tokens[e])
        .collect()
}


pub fn generate_colored_word(tokens: &Vec<char>, probabilities: &Vec<Vec<f64>>, length: usize) -> Vec<ColoredString>{
    generate_word_base(probabilities, length).iter().map(|e|{
        match *e%10 {
            0 => String::from(tokens[*e]).red(),
            1 => String::from(tokens[*e]).bright_red(),
            2 => String::from(tokens[*e]).yellow(),
            3 => String::from(tokens[*e]).bright_yellow(),
            4 => String::from(tokens[*e]).green(),
            5 => String::from(tokens[*e]).bright_green(),
            6 => String::from(tokens[*e]).blue(),
            7 => String::from(tokens[*e]).bright_blue(),
            _ => String::from(tokens[*e]).black()
        }
    })
    .collect()
}
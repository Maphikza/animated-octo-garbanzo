use std::collections::HashMap;
use std::io;
fn main() {
    let answer = get_string();
    let mut text = HashMap::new();

    if answer.len() == 0 {
        println!("You entered nothing.")
    } else {
        for i in answer.chars() {
            if text.contains_key(&i) == false {
                text.insert(i, 1);
            } else if text.contains_key(&i) {
                *text.get_mut(&i).unwrap() += 1;
            }
        }
        println!("The string entered is {:#?}", text);
    }
    
}

fn get_string() -> String {
    println!("Enter your string:");

    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Couldn't read line.");

    let string: String = string.trim().parse().expect("This should be a string.");

    return string.to_lowercase();
}

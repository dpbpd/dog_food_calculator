// use std::io;
// use std::fs::File;
mod com;
mod food_calculation;
use food_calculation::calculate_food_amount;


fn get_dog_weight() -> f64 {
    loop {
        let dog_weight = com::request_response("Enter dog's weight, or 0 to shutdown");
        let mut dog_weight: f64 = match dog_weight.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a number.", dog_weight.trim());
                continue;
            }
        };
        if dog_weight == 0.0 {return dog_weight};
        loop {
            let kgs_lbs = com::get_command(com::request_response("Is this value in lbs or kgs?\nEnter 'k' for kg or 'p' for lbs"));
            match kgs_lbs {
                Some('k') => {
                    dog_weight = dog_weight / 0.45359237;
                    break;
                },
                Some('p') => break,
                _ => {
                    println!("Not a proper command");
                    continue;
                }
            }
        }
        if dog_weight >= 0.0 && dog_weight <= 350.0 {
            return dog_weight;
        } else {
            println!("Not a proper dog weight");
        }
    }
}
#[allow(dead_code)]
fn display_food_brands() {
    let mut num = 1;
    let brands = vec!["Open Farms", "Purina", "Genaric"];
    for brand in brands.iter() {
        println!("{}. {}",num, brand);
        num += 1;
    };
}

fn main() {  // select food brand
    /*
    loop {
        display_food_brands();
        let food_brand = com::request_response("Please pick your food brand").to_string();
        let brand: u32 = match food_brand.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a number.", food_brand.trim());
                continue
            }
        };

        
        println!("food brand {}", brand);
        if brand == 1 {
            break
        };
    };
    */

    // enter ideal weight
    //let optimal_weight = com::request_response("please enter Vereranarians recommended weight");

    // store data so not to ask every time
    loop {
        let food_amount = calculate_food_amount(get_dog_weight());
        if food_amount == 0.0 {
            println!("System is shutting down!");
            break;
        };
        println!("food amount: {}g\nFeed {}g twice daily", food_amount, food_amount / 2.0);
    };

}
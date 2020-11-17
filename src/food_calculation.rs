pub fn calculate_food_amount(dog_weight: f64) -> f64 {
    let cup: f64 = 128.0;
    let weight: u32 = dog_weight as u32;
    let food_amount = match weight {
        1..=9 => map(dog_weight, 1.0, 9.9, cup * 0.25, cup * 0.75),
        10..=19 => map(dog_weight, 10.0, 19.9, cup * 0.75, cup * 1.25),
        20..=29 => map(dog_weight, 20.0, 29.9, cup * 1.25, cup * 1.75),
        30..=39 => map(dog_weight, 30.0, 39.9, cup * 1.75, cup * 2.25),
        40..=59 => map(dog_weight, 40.0, 59.9, cup * 2.25, cup * 3.0),
        60..=79 => map(dog_weight, 60.0, 79.9, cup * 3.0, cup * 3.75),
        80..=99 => map(dog_weight, 80.0, 99.9, cup * 3.75, cup * 4.5),
        100..=199 => map(dog_weight, 100.0, 199.9, cup * 4.5, cup * 7.0),
        200..=350 => map(dog_weight, 200.0, 350.0, cup * 7.0, cup * 10.75),
        _ => 0.0,
    };
    food_amount.round()
}
fn map(value: f64, from_low: f64, from_high: f64, to_low: f64, to_high: f64) -> f64 {
    (value - from_low) * (to_high - to_low) / (from_high - from_low) + to_low
}

#[allow(dead_code)]
pub enum FoodBrand {
    OpenFarms,
    Purina,
    Genaric,

}

#[allow(dead_code)]
#[allow(unused)]
pub fn slect_food_type(selection: FoodBrand) {
    let food_brand = match selection {
        FoodBrand::OpenFarms => println!("got open farms"),
        FoodBrand::Purina => println!("got purina"),
        FoodBrand::Genaric => println!("got genaric"),
    };


}

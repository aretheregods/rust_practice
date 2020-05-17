pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("The value must be between 1 and 100, but got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn tell_guess(&self) -> String {
        format!("Your guess is {}", self.value())
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub height: u32,
    pub width: u32,
}

pub enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter,
    HalfDollar,
    Dollar,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn absolutely_larger(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }
}

pub fn add_world(some_string: String) -> String {
    format!("{}, world", some_string)
}


pub fn coin_values(coin: Coins) -> u8 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter => 25,
        Coins::HalfDollar => 50,
        Coins::Dollar => 100,
    }
}

pub fn string_slicer(some_string: &str, some_tuple: (usize, usize)) -> String {
    let new_string = &some_string[some_tuple.0..some_tuple.1];
    String::from(new_string)
}

pub fn mean(numbers: &Vec<i32>) -> i32 {
    let mut response = 0;

    for integer in numbers {
        response += integer;
    }

    let numbers_length: i32 = numbers.len() as i32;

    response/numbers_length
}

pub fn median(numbers: &mut Vec<i32>) -> i32 {
    let numbers_length = &numbers.len();

    numbers.sort();

    if numbers_length % 2 == 0 {
        let median_index = numbers_length/2;
        numbers[median_index]
    } else {
        let mut response = 0;
        let median_index = (numbers_length - 1)/2;
        let next_index = (numbers_length + 1)/2;
        let numbers_slice = vec![numbers[median_index], numbers[next_index]];
        for integer in &numbers_slice {
            response += integer;
        }
        let median_divider = numbers_slice.len();
        response/median_divider as i32
    }
}

pub fn max_element<T: Ord>(some_array: &[T]) -> &T {
    some_array.iter().max().unwrap()
}
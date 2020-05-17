use practice::practice_library;

fn main() {
    let s1 = String::from("hello");

    let len = &s1.len();

    println!("The string \"{}\" has length {}", s1, len);

    let s2 = practice_library::add_world(s1);
    let len2 = &s2.len();

    println!("The string \"{}\" has length {}", s2, len2);
    println!("This is a slice \"{}\"", practice_library::string_slicer(&s2, (0, 6)));
    println!("This is another slice \"{}\"", practice_library::string_slicer(&s2, (0, s2.len())));

    let rectangle1 = practice_library::Rectangle {
        height: 100,
        width: 200,
    };

    let rectangle2 = practice_library::Rectangle {
        height: 99,
        width: 199,
    };

    println!("This is an instance of a struct: {:#?}", rectangle1);
    println!("This is the area of the rectangle: {}", rectangle1.area());
    println!(
        "Is rectangle1 larger than rectangle2? {}", 
        if rectangle1.absolutely_larger(&rectangle2) { "yes" } else { "no" });

    let coin1 = practice_library::Coins::Penny;
    let money1 = practice_library::coin_values(coin1);

    println!("This coin has the value: {}", &money1);

    let mut some_numbers = vec![102, 35, 21, 21, 57, 3000];
    let some_letters = vec!["a", "b", "c", "d", "e", "x"];
    let some_average = practice_library::mean(&some_numbers);
    let some_median = practice_library::median(&mut some_numbers);
    let largest_number = practice_library::max_element(&some_numbers);
    let highest_letter = practice_library::max_element(&some_letters);

    println!("The average of {:#?} is {}", some_numbers, some_average);
    println!("The median of {:#?} is {}", some_numbers, some_median);
    println!("The largest number from {:#?} is: {}", some_numbers, largest_number);
    println!("The largest character from {:#?} is {}", some_letters, highest_letter);

    let guess1 =  practice_library::Guess::new(25);

    println!("{}", guess1.tell_guess());
}

pub mod jewels_and_stones_771;
pub mod n_repeated_element_in_size_2n_array_961;
pub mod unique_email_addresses_929;
pub mod unique_morse_code_words_804;

pub struct Solution;


pub fn run_jewels_and_stones_771() {
    println!("run jewels_and_stones test!");
    let jewels = String::from("aA");
    let stones = String::from("aAAAbbbb");
    assert_eq!(4, Solution::num_jewels_in_stones(jewels, stones));

    let jewels = String::from("z");
    let stones = String::from("ZZ");
    assert_eq!(0, Solution::num_jewels_in_stones(jewels, stones));

    let jewels = String::from("");
    let stones = String::from("");
    assert_eq!(0, Solution::num_jewels_in_stones(jewels, stones));
}




pub mod jewels_and_stones_771;
pub mod num_unique_emails;


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




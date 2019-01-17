use super::Solution;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let mut result_set: HashSet<String> = HashSet::new();
        let morse_code_map = vec![
            ".-",
            "-...",
            "-.-.",
            "-..",
            ".",
            "..-.",
            "--.",
            "....",
            "..",
            ".---",
            "-.-",
            ".-..",
            "--",
            "-.",
            "---",
            ".--.",
            "--.-",
            ".-.",
            "...",
            "-",
            "..-",
            "...-",
            ".--",
            "-..-",
            "-.--",
            "--.."
        ];
        for word in &words {
            let mut morse_code = String::new();
            for letter in word.chars() {
                let index= (letter as u8 - 97) as usize;
                morse_code.extend(morse_code_map[index].chars());
            }
            result_set.insert(morse_code);
        }
        result_set.len() as i32
    }
}


#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    pub fn unique_morse_code_words_804() {
        assert_eq!(2, Solution::unique_morse_representations(vec![
            String::from("gin"),
            String::from("zen"),
            String::from("gig"),
            String::from("msg")
        ]))
    }
}

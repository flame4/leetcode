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

// TODO Rust的字符串处理揍得我鼻青脸肿.
        let gen_morse_code = |s: &str, result_set: &mut HashSet<String>, morse_code_map: &Vec<&str>| {
            let mut morse_code = String::new();

            result_set.insert(morse_code)
        };

        for i in &words {
            gen_morse_code(i, &mut result_set, &morse_code_map);
        }
        result_set.len() as i32
    }
}
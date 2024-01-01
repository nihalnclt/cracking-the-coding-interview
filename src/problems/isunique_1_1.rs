// Implement an algorithm to determine if a string has all unique characters.
// What if you can't use additional data structures?

use std::collections::HashMap;

fn is_unique(str: &str) -> bool {
    if str.len() > 128 {
        return false;
    };

    let mut char_set: HashMap<char, bool> = HashMap::new();
    for char in str.chars() {
        match char_set.get(&char) {
            Some(_) => return false,
            None => {
                char_set.insert(char, true);
            }
        }
    }

    true
}

// The time complexity is O(n) where n is the length of the string.
// And space complexity is O(1).
// And if we don't use any additional data strucures then, time complexity will be O(n^2)

pub fn result() -> bool {
    is_unique("kathrika")
}

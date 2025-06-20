use anagram::anagrams_for;
use std::collections::HashSet;

fn main() {
    let word = "Orchestra";
    let inputs = &["cashregister", "Carthorse", "radishes"];
    let output = anagrams_for(word, inputs);
    output.iter().for_each(|o| {
        print!("{ }", o);
    });
    let expected = HashSet::from_iter(["Carthorse"]);
    assert_eq!(output, expected);
}

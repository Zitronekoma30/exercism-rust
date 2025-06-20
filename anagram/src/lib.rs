use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let w = {
        let mut wv: Vec<char> = word.to_lowercase().chars().collect();
        wv.sort();
        wv
    };

    possible_anagrams
        .iter()
        .copied()
        .filter(|p| {
            word.to_lowercase() != p.to_lowercase() && {
                let mut pv: Vec<char> = p.to_lowercase().chars().collect();
                pv.sort();
                w == pv
            }
        })
        .collect()
}

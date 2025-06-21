use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let res_c = input.split("==").last()?.strip_prefix(' ')?.to_string();
    let summands_c: Vec<String> = input
        .replace(" ", "")
        .split("==")
        .next()?
        .split('+')
        .map(|s| s.to_string())
        .collect();

    let mut mappings: HashMap<char, u8> = HashMap::new();

    for c in input.chars().filter(|c| c.is_alphabetic()) {
        mappings.insert(c, 0);
    }

    let keys: Vec<char> = mappings.keys().copied().collect();
    let nums: Vec<u8> = (0..10).collect();

    if bruteforce(&nums, &keys, 0, &mut mappings, &res_c, &summands_c) {
        Some(mappings)
    } else {
        None
    }
}

fn bruteforce(
    available: &[u8],
    keys: &[char],
    current_index: usize,
    mappings: &mut HashMap<char, u8>,
    result_c: &String,
    parts: &[String],
) -> bool {
    if current_index >= keys.len() {
        let result: String = result_c
            .chars()
            .map(|c| char::from(*mappings.get(&c).unwrap() + b'0'))
            .collect();

        let summands: Vec<String> = parts
            .iter()
            .map(|w| {
                w.chars()
                    .map(|c| char::from(*mappings.get(&c).unwrap() + b'0'))
                    .collect()
            })
            .collect();

        let sum: i32 = summands
            .iter()
            .fold(0, |x, word| x + i32::from_str_radix(word, 10).unwrap());

        let result_i: i32 = i32::from_str_radix(&result, 10).unwrap();

        if sum == result_i {
            print!("{:?}", result);
            print!("{:?}", summands);

            return true;
        } else {
            return false;
        }
    }

    let current_char = keys[current_index];
    for &i in available {
        *mappings.get_mut(&current_char).unwrap() = i;
        let new_avail: Vec<u8> = available.iter().filter(|&&x| x != i).copied().collect();

        if bruteforce(
            &new_avail,
            keys,
            current_index + 1,
            mappings,
            result_c,
            parts,
        ) {
            return true;
        }
    }
    false
}

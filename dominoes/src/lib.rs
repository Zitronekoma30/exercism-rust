use std::collections::HashMap;
use std::collections::HashSet;
// for the euler tour solution: https://github.com/TianyiShi2001/Algorithms#contents
fn flip(domino: (u8, u8)) -> (u8, u8) {
    (domino.1, domino.0)
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let nums: Vec<u8> = input.iter().flat_map(|(a, b)| [*a, *b]).collect();
    let mut map: HashMap<u8, Vec<(u8, u8)>> = HashMap::new();

    // Fill map
    for d in input {
        map.entry(d.0).or_default().push(*d);
        if d.0 != d.1 {
            map.entry(d.1).or_default().push(*d);
        }
    }

    // Find potential start-end pairs
    let mut start: (u8, u8) = input[0];
    let mut end: (u8, u8) = input[0];

    for x in nums {
        let with_x = map.entry(x).or_default();
        if with_x.len() > 1 {
            start = with_x[0];
            end = with_x[1];
        }
    }

    // Build chain
    let mut used: HashSet<(u8, u8)> = HashSet::new();
    let mut chain: Vec<(u8, u8)> = Vec::new();

    chain.push(start);
    used.insert(start);

    let mut prev: (u8, u8) = start;

    // Use all once
    loop {
        let next_opts = map.get(&prev.1).unwrap();
        let mut next: (u8, u8) = *next_opts
            .iter()
            .find(|d| **d != prev && **d != flip(prev))
            .unwrap_or(&prev);
        for d in next_opts {
            // prefer unused, disallow consecutive unless only option
            if !used.contains(d) {
                next = *d;
                break;
            }
        }

        // append
        used.insert(next);
        if prev.1 != next.0 {
            next = flip(next);
        }

        chain.push(next);
        prev = next;

        if used.len() >= input.len() {
            break;
        }
    }

    // TODO: build bridge to final piece

    Some(vec![(1, 2)])
}

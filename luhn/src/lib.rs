/// Check a Luhn checksum.

pub fn is_valid(code: &str) -> bool {
    if code.chars().any(|c| !c.is_ascii_digit() && c != ' ')
        || code
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<Vec<char>>()
            .len()
            < 2
    {
        return false;
    }

    let sum: u32 = code
        .replace(' ', "")
        .chars()
        .rev()
        .enumerate()
        .map(|(i, x)| {
            let mut y = x.to_digit(10).unwrap();
            if i % 2 == 1 {
                y = 2 * y;
                if y > 9 {
                    y -= 9;
                }
            }
            y
        })
        .sum();

    sum % 10 == 0
}

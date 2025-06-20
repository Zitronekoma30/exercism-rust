pub fn abbreviate(phrase: &str) -> String {
    let mut acro = String::new();

    let mut chars = phrase.chars();
    let mut prev: char = chars.next().unwrap();
    let replace = ['.', ',', '\''];

    if prev.is_alphabetic() {
        acro.push(prev);
    }

    let filtered = chars.filter(|c| !replace.iter().any(|f| f == c));

    for c in filtered {
        if c.is_alphabetic() && (!prev.is_alphabetic() || (prev.is_lowercase() && c.is_uppercase()))
        {
            acro.push(c.to_ascii_uppercase());
        }
        prev = c;
    }

    acro
}

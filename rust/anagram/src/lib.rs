use std::collections::HashSet;

fn hash(x: &str) -> String {
    let mut y = x.to_lowercase().chars().collect::<Vec<char>>();
    y.sort_unstable();
    y.iter().collect()
}
pub fn anagrams_for<'a, 'b>(word: &'a str, possible_anagrams: &'b [&'a str]) -> HashSet<&'a str> {
    let w_hash = hash(word);
    possible_anagrams
        .iter()
        .filter(|x| x.to_lowercase() != word.to_lowercase() && w_hash == hash(x))
        .cloned()
        .collect()
}

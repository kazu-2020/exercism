use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|&&candidate| is_anagram(word, candidate))
        .cloned()
        .collect()
}

fn is_anagram(word: &str, candidate: &str) -> bool {
    let word = word.to_lowercase();
    let candidate = candidate.to_lowercase();
    if word == candidate {
        return false;
    }

    let mut word_chars: Vec<char> = word.chars().collect();
    let mut candidate_chars: Vec<char> = candidate.chars().collect();
    word_chars.sort_unstable();
    candidate_chars.sort_unstable();

    word_chars == candidate_chars
}

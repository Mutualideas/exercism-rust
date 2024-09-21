use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            candidate.to_lowercase() != word.to_lowercase() && is_anagram(word, candidate)
        })
        .cloned()
        .collect()
}

pub fn is_anagram(target: &str, candidate: &str) -> bool {
    let mut target_chars: Vec<char> = target.to_lowercase().chars().collect();
    let mut candidate_chars: Vec<char> = candidate.to_lowercase().chars().collect();

    target_chars.sort();
    candidate_chars.sort();

    target_chars == candidate_chars
}

use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let word_lower = word.to_lowercase();
    let word_sorted: String = sort_string(&word_lower);

    for &candidate in possible_anagrams {
        let candidate_lower = candidate.to_lowercase();

        // Skip if it's the same word (case-insensitive)
        if candidate_lower == word_lower {
            continue;
        }

        // Check if it's an anagram by comparing sorted characters
        let candidate_sorted: String = sort_string(&candidate_lower);
        if candidate_sorted == word_sorted {
            result.insert(candidate);
        }
    }

    result
}

fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

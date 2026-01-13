use std::collections::{HashMap, HashSet};
use unicode_segmentation::UnicodeSegmentation;

struct NormalizedWord {
    graphemes: Vec<String>,
    counts: HashMap<String, u32>,
}

impl NormalizedWord {
    fn new(word: &str) -> Self {
        let graphemes: Vec<String> = word.graphemes(true).map(|g| g.to_lowercase()).collect();

        let counts = graphemes.iter().fold(HashMap::new(), |mut acc, grapheme| {
            *acc.entry(grapheme.clone()).or_insert(0) += 1;
            acc
        });

        Self { graphemes, counts }
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let normalized_word = NormalizedWord::new(word);

    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            let normalized_candidate = NormalizedWord::new(candidate);

            // Check if it's not the same word
            if normalized_word.graphemes == normalized_candidate.graphemes {
                return false;
            }

            // Check if counts match
            normalized_word.counts == normalized_candidate.counts
        })
        .copied()
        .collect()
}

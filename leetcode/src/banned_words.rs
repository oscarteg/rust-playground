use std::collections::{HashMap, HashSet};

pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let banset: HashSet<String> = HashSet::from_iter(banned);

    let mut freq = HashMap::new();
    let words = paragraph.split(|c: char| c.is_ascii_punctuation() || c.is_ascii_whitespace());

    for word in words {
        let word = word.to_ascii_lowercase();

        if word.is_empty() || banset.contains(&word) {
            continue;
        }

        *freq.entry(word).or_insert(0) += 1;
    }

    let mut most = "";
    let mut most_count = 0;

    for (k, v) in freq.iter() {
        if v > &most_count {
            most_count = *v;

            most = k
        }
    }

    most.to_string()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.".to_string();

        let banned_list = vec!["hit".to_string()];

        assert_eq!(most_common_word(paragraph, banned_list), "ball".to_string());
    }
}

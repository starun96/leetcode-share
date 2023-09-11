use std::collections::HashMap;

const N_LETTERS: usize = 26;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    // go through each element
    strs.into_iter()
        .map(|word| {
            let mut sorted_word = word.chars().collect::<Vec<_>>();
            sorted_word.sort();
            (word, sorted_word.into_iter().collect::<String>())
        })
        .for_each(|(orig_word, sorted_word)| {
            let entry = map.entry(sorted_word).or_insert(vec![]);

            entry.push(orig_word);
        });

    return map.into_values().collect();
}

pub fn get_frequencies(word: &str) -> [u8; N_LETTERS] {
    word.bytes()
        .map(|b| (b - b'a') as usize)
        .fold([0; N_LETTERS], |mut freqs, bin| {
            freqs[bin] += 1;
            freqs
        })
}

pub fn group_anagrams_histogram_frequencies(strs: Vec<String>) -> Vec<Vec<String>> {
    strs.into_iter()
        .fold(
            HashMap::<[u8; N_LETTERS], Vec<String>>::new(),
            |mut map, s| {
                let freqs = get_frequencies(&s);
                map.entry(freqs).or_default().push(s);
                map
            },
        )
        .into_values()
        .collect()
}

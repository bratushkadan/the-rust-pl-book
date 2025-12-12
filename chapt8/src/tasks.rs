use std::collections::HashMap;

#[derive(Debug)]
pub struct Task1 {
    pub median: f64,
    pub mean: f64,
    pub mode: i32,
}

fn median(data: &[i32]) -> f64 {
    let mut copy = data.to_vec();
    copy.sort_unstable();

    if copy.len() == 0 {
        return 0.0;
    }
    if copy.len() == 1 {
        return copy[0].into();
    }
    if copy.len() % 2 == 0 {
        let (i1, i2) = (copy.len() / 2 - 1, copy.len() / 2);
        let sum = (copy[i1] + copy[i2]) as f64;
        return sum / 2.0;
    }
    return copy[copy.len() / 2] as f64;
}

fn mean(data: &[i32]) -> f64 {
    let mut sum: f64 = 0.0;
    for v in data {
        sum += *v as f64;
    }
    sum / data.len() as f64
}

fn mode(data: &[i32]) -> i32 {
    let mut map: HashMap<i32, u32> = HashMap::with_capacity(data.len());

    for v in data {
        let counter = map.entry(*v).or_insert(0);
        *counter += 1;
    }

    let (mut most_freq_occurence, mut n_occurences): (i32, u32) = (0, 0);
    for (occ, n) in &map {
        if n_occurences < *n {
            most_freq_occurence = *occ;
            n_occurences = *n;
        }
    }

    most_freq_occurence
}

pub fn task1(data: &[i32]) -> Option<Task1> {
    // let vec = vec![1, 2, 3, 4, 5];
    // println!("mean of \"{:?}\" = {}", vec, mean(&vec));

    // let vec = vec![1, 15, 3, 16, 4];
    // println!("mean of \"{:?}\" = {}", vec, mean(&vec));

    if data.len() == 0 {
        return None;
    }
    Some(Task1 {
        median: median(data),
        mean: mean(data),
        mode: mode(data),
    })
}

fn pig_latin(word: &str) -> String {
    match word.chars().next() {
        Some(c) if is_vowel(c) => format!("{}-hay", word),
        Some(c) => {
            let rest: String = word.chars().skip(1).collect();
            format!("{}-{}ay", rest, c)
        }
        None => String::new(),
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

pub fn task2(phrase: &str) -> String {
    phrase
        .split_whitespace()
        .map(pig_latin)
        .collect::<Vec<_>>()
        .join(" ")
}

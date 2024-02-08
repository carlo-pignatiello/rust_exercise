use std::collections::{HashMap, HashSet};

pub fn is_unique(s: &str) -> bool {
    if s.len() > 128 {
        return false;
    }
    let mut boolean_vector = vec![false; 128];
    for (i, _c) in s.replace(" ", "").bytes().enumerate() {
        let val = s.as_bytes()[i];
        if boolean_vector[val as usize] {
            return false;
        }
        boolean_vector[val as usize] = true
    }
    true
}

pub fn check_permutation(s_1: &str, s_2: &str) -> bool {
    if s_1.len() != s_2.len() {
        return false;
    }
    let mut s_1_sorted: Vec<char> = s_1.chars().collect();
    s_1_sorted.sort();
    let mut s_2_sorted: Vec<char> = s_2.chars().collect();
    s_2_sorted.sort();
    return s_1_sorted == s_2_sorted;
}

pub fn url_ify(s: &str) -> String {
    s.trim().replace(' ', "%20")
}

pub fn palindrome_permutation(mut s: String) -> bool {
    let is_odd = s.len() % 2 == 0;
    let mut perm = HashMap::new();
    s = s.to_lowercase().replace(" ", "");

    for i in s.chars() {
        let counter = s.matches(i).count();
        perm.insert(i, counter);
    }
    let values: Vec<usize> = perm.into_values().collect();
    if is_odd {
        {
            for i in values {
                if i % 2 == 0 {
                    return true;
                }
            }
        }
    } else {
        {
            let hs = values.iter().cloned().collect::<HashSet<usize>>();
            let hs: Vec<usize> = hs.into_iter().collect();
            if hs.len() > 1 {
                return false;
            }
        }
    }
    true
}

pub fn one_way(str1: &String, str2: &String) -> bool {
    let mut token = 0;
    for c in str1.chars() {
        if !str2.contains(c) {
            token = token + 1;
        }
    }
    if token <= 1 {
        return true;
    }
    false
}

pub fn string_compression(s: &String) -> String {
    let mut compressed_str = String::new();
    let mut counter_consecutive: u32 = 0;
    for (i, n) in s.chars().enumerate() {
        counter_consecutive = counter_consecutive + 1;
        if i + 1 >= s.len() || s.as_bytes()[i] != s.as_bytes()[i + 1] {
            let compression = format!("{}{}", n.to_string(), counter_consecutive.to_string());
            compressed_str.push_str(&compression);
            counter_consecutive = 0
        }
    }
    if compressed_str < s.to_string() {
        compressed_str
    } else {
        s.to_string()
    }
}

type Image = Vec<Vec<u8>>;

pub fn rotate_image(image: &Image) -> Image {
    if image.len() == 0 || image.len() != image[0].len() {
        panic!("The input is wrong; it's null or it is no quadratic");
    }
    let mut rotated_image: Image = image.clone();
    // we have to think in layers
    let n = image.len();
    for layer in 0..(n / 2) {
        let first = layer;
        let last = n - 1 - layer;
        for i in first..last {
            let offset = i - first;
            let top = rotated_image[first][i];

            // left to top
            rotated_image[first][i] = rotated_image[last-offset][first];

            // bottom to left
            rotated_image[last-offset][first] = rotated_image[last][last - offset];

            // right to bottom
            rotated_image[last][last-offset] = rotated_image[i][last];

            // top to right
            rotated_image[i][last] = top;
        }
    }
    rotated_image
}

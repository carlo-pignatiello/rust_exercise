mod arrays_and_strings;

fn main() {
    arrays_and_strings::is_unique("helloworld");
    arrays_and_strings::check_permutation(&String::from("cat"), &String::from("dog"));
    arrays_and_strings::url_ify("Mr John Smith    ");
    arrays_and_strings::palindrome_permutation(String::from("Tact Coa"));
    arrays_and_strings::one_way(&String::from("pale"), &String::from("bake"));
    arrays_and_strings::string_compression(&String::from("aabbccc"));
    let i = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    let r = arrays_and_strings::rotate_image(&i);
    println!("{:?}", i);
    println!("{:?}", r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert_eq!(arrays_and_strings::is_unique("abcdefg"), true);
        assert_eq!(arrays_and_strings::is_unique("abcdefga"), false);
    }

    #[test]
    fn test_is_permutation() {
        assert_eq!(
            arrays_and_strings::check_permutation(&String::from("cat"), &String::from("tac")),
            true
        );
        assert_eq!(
            arrays_and_strings::check_permutation(&String::from("cat"), &String::from("dog")),
            false
        );
    }

    #[test]
    fn test_urlify() {
        assert_eq!(
            arrays_and_strings::url_ify("Mr John Smith    "),
            "Mr%20John%20Smith"
        );
    }

    #[test]
    fn test_palindrome_permutation() {
        assert_eq!(
            arrays_and_strings::palindrome_permutation(String::from("Tact Coa")),
            true
        );
        assert_eq!(
            arrays_and_strings::palindrome_permutation(String::from("abc acb")),
            true
        );
        assert_eq!(
            arrays_and_strings::palindrome_permutation(String::from("abc abd")),
            false
        );
    }

    #[test]
    fn test_one_way() {
        assert_eq!(
            arrays_and_strings::one_way(&String::from("pale"), &String::from("ple")),
            true
        );
        assert_eq!(
            arrays_and_strings::one_way(&String::from("pales"), &String::from("pale")),
            true
        );
        assert_eq!(
            arrays_and_strings::one_way(&String::from("pale"), &String::from("bake")),
            false
        );
    }

    #[test]
    fn test_string_compression() {
        assert_eq!(
            arrays_and_strings::string_compression(&String::from("aaabbcdd")),
            String::from("a3b2c1d2")
        );
        assert_eq!(
            arrays_and_strings::string_compression(&String::from("abc")),
            String::from("a1b1c1")
        );
        assert_eq!(
            arrays_and_strings::string_compression(&String::from("aaaadddd")),
            String::from("a4d4")
        );
    }

    #[test]
    fn test_rotate_image() {
        let image = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let rotated_image = arrays_and_strings::rotate_image(&image);

        let expected_rotated_image = vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ];

        assert_eq!(rotated_image, expected_rotated_image);
    }
}

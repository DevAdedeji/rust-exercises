fn is_palindrome(s: &str) -> bool {
    // Normalize the string: convert to lowercase and remove non-alphanumeric characters.
    let normalized: String = s
        .chars()
        .filter(|c| c.is_alphanumeric()) // Keep only alphanumeric characters
        .collect::<String>()
        .to_lowercase(); // Convert to lowercase

    // Use two pointers to check from both ends of the string
    let mut start = 0;
    let mut end = normalized.len() - 1;

    while start < end {
        if normalized[start..=start] != normalized[end..=end] {
            return false; // If characters don't match, it's not a palindrome
        }
        start += 1;
        end -= 1;
    }

    true // All characters matched, it's a palindrome
}

fn main() {
    let test_cases = vec![
        "A man, a plan, a canal, Panama", // Palindrome with spaces and punctuation
        "racecar",                        // Simple palindrome
        "hello",                          // Not a palindrome
        "Was it a car or a cat I saw",    // Palindrome with spaces and punctuation
        "Madam",                          // Palindrome with mixed case
        "12321",                          // Palindrome with numbers
        "",                               // Empty string (palindrome)
    ];

    for s in test_cases {
        println!("\"{}\" is a palindrome: {}", s, is_palindrome(s));
    }
}

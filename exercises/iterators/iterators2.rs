// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),

        // * Option 1: Create a new string, add the capitalized first char,
        // * then add the remaining chars
        Some(first) => {
            let mut result = first.to_uppercase().to_string();
            result.push_str(&input[1..]);
            result
        } 

        // * Option 2: this is tricky!
        // * first.to_uppercase returns an iterable (with a single char)
        // * so we have to use collect, cast to string, then add the rest of 
        // * the chars.
        // Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let words = vec!["hello", "world"];

    // * Here we opted to create a new vector, that maps the 
    // * result of capitalize_first to each item (passed as a reference) in the "words" vec
    let capitalized_words: Vec<String> =
        words.iter().map(|&word| capitalize_first(&word)).collect();
    return capitalized_words;

    // * A different option:
    // * In this case we use to_owned() to have ownership of the values
    // * then convert it into an iterable, them map and collect 
    // * (since we have ownership of values we don't need to pass a reference 
    // * to each item to `capitalize_first`)
    // words.to_owned().into_iter().map(capitalize_first).collect()

}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {

    // * Here we opted to create a new "string", that maps the 
    // * result of capitalize_first to each item (passed as a reference) in the "words" vec
    // * Notice that we cast the result of collect to <String>
    let capitalized_words: String =
        words.iter().map(|&word| capitalize_first(&word)).collect::<String>();
    return capitalized_words;

    // * In this option we use to_owned() so that the values are owned by this function
    // * then we map each item and collect casting as a string.
        // words.to_owned().into_iter().map(capitalize_first).collect::<String>()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}

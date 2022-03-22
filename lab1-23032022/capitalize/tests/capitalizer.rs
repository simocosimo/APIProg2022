use capitalize::capitalize;

#[test]
fn test_with_multiple_words() {
    assert_eq!(capitalize("this is a test"), "This Is A Test");
}

#[test]
fn test_with_one_word_no_spaces() {
    assert_eq!(capitalize("test"), "Test");
}

#[test]
fn test_with_starting_accented_characters() {
    assert_eq!(capitalize("è come àlbero índaco"), "È Come Àlbero Índaco");
}

#[test]
fn test_with_empty_string() {
    assert_eq!(capitalize(""), "");
}

#[test]
fn test_with_multiple_spaces() {
    assert_eq!(capitalize("this    is    a test"), "This    Is    A Test");
}
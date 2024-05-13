use crate::str::StringUtil;

#[test]
fn test_substring() {
    let s = "Hello, World!";
    assert_eq!(s.substring(0, 5), "Hello");
    assert_eq!(s.substring(7, 12), "World");
    assert_eq!(s.substring(-6, -1), "World");
}

#[test]
fn test_unquote() {
    let quoted_str = "\"Hello World!\"";
    assert_eq!(quoted_str.unquote(true, None), "Hello World!");

    let quoted_str = "'Hello World!'";
    assert_eq!(quoted_str.unquote(true, None), "Hello World!");

    let quoted_str = "'Hello \\'World!'";
    assert_eq!(quoted_str.unquote(true, None), "Hello 'World!");

    let s = "'\\'Hello, World!\\''";
    assert_eq!(s.unquote(true, None), "'Hello, World!'");
}

#[test]
fn is_quoted() {
    let quoted_str = "\"Hello World!\"";
    assert!(quoted_str.is_quoted());

    let quoted_str = "'Hello World!'";
    assert!(quoted_str.is_quoted());

    let quoted_str = "Hello World!";
    assert!(!quoted_str.is_quoted());
}

#[test]
fn test_join_path_segments() {
    assert_eq!(
        "http://example.com".join_path_segments(vec!["/path"]),
        "http://example.com/path"
    );
    assert_eq!(
        "http://example.com".join_path_segments(vec!["path"]),
        "http://example.com/path"
    );
    assert_eq!(
        "http://example.com/".join_path_segments(vec!["path"]),
        "http://example.com/path"
    );
    assert_eq!(
        "http://example.com/".join_path_segments(vec!["/path"]),
        "http://example.com/path"
    );
}

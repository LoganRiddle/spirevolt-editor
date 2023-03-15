fn replace_ascii_char_at(s: String, idx: uint, c: char) -> String {
    let mut ascii_s = s.into_ascii();
    ascii_s[idx] = c.to_ascii();
    String::from_utf8(ascii_s.into_bytes()).unwrap()
}

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut rev_bitmap = 0x3ffffff_u32;

    for c in sentence
        .to_ascii_lowercase()
        .matches(|c| char::is_ascii_alphabetic(&c))
        .flat_map(|s| s.chars())
    {
        let mask = !(1_u32 << c as u8 - 97);
        rev_bitmap &= mask;
    }

    rev_bitmap == 0
}

pub fn is_valid_pow(hash_hex: &str, difficulty: u32) -> bool{

    hash_hex
        .chars()
        .take(difficulty as usize)
        .all(|c| c == '0')
    
}
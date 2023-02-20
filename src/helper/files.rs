use sha2::{Sha256, Digest};

#[allow(deprecated)]
pub fn generate_hash_from_bytes_to_base64(file_bytes: &[u8]) -> String {
    let hash = Sha256::digest(file_bytes);
    return base64::encode(hash);
}

pub fn generate_hash_from_string_to_base64(file_content: &str) -> String {
    return generate_hash_from_bytes_to_base64(file_content.as_bytes());
}
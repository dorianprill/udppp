pub fn calculate_checksum(data: &[u8]) -> u32 {
    let mut checksum = 0u32;
    for &byte in data {
        checksum = checksum.wrapping_add(byte as u32);
    }
    checksum
}

pub fn verify_checksum(data: &[u8], checksum: u32) -> bool {
    calculate_checksum(data) == checksum
}

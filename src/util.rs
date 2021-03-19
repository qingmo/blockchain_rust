pub fn convert_u64_to_u8_array(data: u64) -> [u8; 8] {
    return [
        data as u8,
        (data >> 8) as u8,
        (data >> 16) as u8,
        (data >> 24) as u8,
        (data >> 32) as u8,
        (data >> 40) as u8,
        (data >> 48) as u8,
        (data >> 56) as u8,
    ];
}
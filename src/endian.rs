pub fn u32_to_le_array(num: u32) -> [u8; 4] {
    let b0 = ((num >> 24) & 0xff) as u8;
    let b1 = ((num >> 16) & 0xff) as u8;
    let b2 = ((num >> 8) & 0xff) as u8;
    let b3 = (num & 0xff) as u8;
    [b3, b2, b1, b0]
}

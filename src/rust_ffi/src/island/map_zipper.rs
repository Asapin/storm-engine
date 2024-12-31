pub struct MapZipper {
    word_table: Vec<u16>,
    real_data: Vec<u8>,
    dw_size_x: u32,
    dw_dx: u32,
    dw_block_size: u32,
    dw_block_shift: u32,
    dw_shift_num_blocks_x: u32,
    dw_num_real_blocks: u32
}

struct JupToPoolData {
    pub pool_amount_index: u32,
    pub jup_data_length: u32,
    pub jup_data: Vec<u8>,
    pub pool_data: Vec<u8>,
}

// 转 rust 代码
struct PoolToJupData {
    pub pool_data_length: u32,
    pub pool_data: Vec<u8>,
    pub jup_data: Vec<u8>,
}

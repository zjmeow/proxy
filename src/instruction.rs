use borsh::BorshDeserialize;

#[derive(BorshDeserialize, Debug)]
pub struct JupPayload {
    pub min_profit: u64,
} // 这玩意可以从后往前读，直接改需要的字段就完事了
struct Param {
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u64,
    pub platform_fee_bps: u64,
}
struct RoutePlanStep {}
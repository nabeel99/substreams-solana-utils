pub struct TokenAccount {
    pub mint: Vec<u8>,
    pub address: Vec<u8>,
    pub owner: Vec<u8>,
    pub balances: Option<TokenBalances>,
}
pub struct TokenBalances {
    pub pre_tx_balance: Option<f64>,
    pub post_tx_balance: Option<f64>,
}

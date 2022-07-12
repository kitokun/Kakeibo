pub struct Transaction {
    pub amount: u32,         // 金額
    pub destination: String, // 支払い先
    pub source: String,      // 支払い元
    pub nominal: String,     // 名目
    pub description: String, // 説明
}
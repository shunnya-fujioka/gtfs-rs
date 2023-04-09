use serde::{Deserialize, Serialize};

/// 運賃属性情報(fare_attributes.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct FareAttributes {
    /// 運賃ID
    pub fare_id: String,
    /// 運賃
    pub price: f64,
    /// 通貨
    pub currency_type: String,
    /// 支払いタイミング
    /// * `0` - 乗車後に支払う
    /// * `1` - 乗車前に支払う
    pub payment_method: u8,
    /// 乗換
    /// * `0` - この料金で乗り換えることはできません
    /// * `1` - 1 度の乗り換えが可能
    /// * `2` - 2 度の乗り換えが可能
    /// * `None` - 乗り換え回数に制限がなし
    pub transfers: Option<u8>,
    /// 乗換有効期限(秒)
    pub transfer_duration: Option<u32>,
}

/// 運賃定義情報(fare_rules.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct FareRules {
    /// 運賃ID
    pub fare_id: String,
    /// 経路ID
    pub route_id: Option<String>,
    /// 乗車地ゾーン
    pub origin_id: Option<String>,
    /// 降車地ゾーン
    pub destination_id: Option<String>,
    /// 通過ゾーン
    pub contains_id: Option<String>,
}

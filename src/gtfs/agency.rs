use serde::{Deserialize, Serialize};

/// 事業者情報(agency.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct Agency {
    /// 事業者ID
    pub agency_id: String,
    /// 事業者名称
    pub agency_name: String,
    /// 事業者URL
    pub agency_url: String,
    /// タイムゾーン
    pub agency_timezone: String,
    /// 言語
    pub agency_lang: String,
    /// 電話番号
    pub agency_phone: Option<String>,
    /// オンライン購入URL
    pub agency_fare_url: Option<String>,
    /// 事業者Eメール
    pub agency_email: Option<String>,
}

/// 事業者追加情報(agency_jp.txt)
#[derive(Debug, Deserialize)]
pub struct AgencyJp {
    /// 事業者ID
    pub agency_id: String,
    /// 事業者正式名称
    pub agency_official_name: Option<String>,
    /// 事業者郵便番号
    pub agency_zip_number: Option<String>,
    /// 事業者住所
    pub agency_address: Option<String>,
    /// 代表者肩書
    pub agency_president_pos: Option<String>,
    /// 代表者氏名
    pub agency_president_name: Option<String>,
}

use serde::{Deserialize, Serialize};

/// 営業所情報(office_jp.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct OfficeJp {
    /// 営業所ID
    pub office_id: String,
    /// 営業所名
    pub office_name: String,
    /// 営業所URL
    pub office_url: Option<String>,
    /// 営業所電話番号
    pub office_phone: Option<String>,
}

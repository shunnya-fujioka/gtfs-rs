use serde::{Deserialize, Serialize};

/// 停留所・標柱情報(stops.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct Stops {
    /// 停留所・標柱ID
    pub stop_id: String,
    /// 停留所・標柱番号
    pub stop_code: Option<String>,
    /// 停留所・標柱名称
    pub stop_name: String,
    /// 停留所・標柱付加情報
    pub stop_desc: Option<String>,
    /// 緯度
    pub stop_lat: String,
    /// 経度
    pub stop_lon: String,
    /// 運賃エリアID
    pub zone_id: Option<String>,
    /// 停留所・標柱URL
    pub stop_url: Option<String>,
    /// 停留所・標柱区分
    /// * `0` - 標柱
    /// * `1` - 停留所
    pub location_type: Option<u8>,
    /// 親駅情報
    pub parent_station: Option<String>,
    /// タイムゾーン
    pub stop_timezone: Option<String>,
    /// 車椅子情報
    pub wheelchair_boarding: Option<String>,
    /// のりば情報
    pub platform_code: Option<String>,
}

use serde::{Deserialize, Serialize};

/// 便情報(trips.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct Trips {
    /// 経路ID
    pub route_id: String,
    /// 運行日ID
    pub service_id: String,
    /// 便ID
    pub trip_id: String,
    /// 便行先
    pub trip_headsign: Option<String>,
    /// 便名称
    pub trip_short_name: Option<String>,
    /// 上下区分
    /// * `0` - 復路
    /// * `1` - 往路
    pub direction_id: Option<u8>,
    /// 便結合区分
    pub block_id: Option<String>,
    /// 描画ID
    pub shape_id: Option<String>,
    /// 車いす利用区分
    /// * `0` - 車いすによる乗車可否の情報なし
    /// * `1` - 少なくとも1台の車いすによる乗車可能
    /// * `2` - 車いすによる乗車不可
    pub wheelchair_accessible: Option<u8>,
    /// 自転車持込区分
    /// * `0` - 自転車の持込可否の情報なし
    /// * `1` - 少なくとも1台の自転車の持込可能
    /// * `2` - 自転車の持込不可
    pub bikes_allowed: Option<u8>,
    /// 便情報
    pub jp_trip_desc: Option<String>,
    /// 便記号
    pub jp_trip_desc_symbol: Option<String>,
    /// 営業所ID
    pub jp_office_id: Option<String>,
}

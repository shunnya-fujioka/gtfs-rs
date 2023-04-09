use serde::{Deserialize, Serialize};

/// 経路情報(routes.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct Routes {
    /// 経路ID
    pub route_id: String,
    /// 事業者ID
    pub agency_id: String,
    /// 経路略称
    pub route_short_name: String,
    /// 経路名
    pub route_long_name: String,
    /// 経路情報
    pub route_desc: Option<String>,
    /// 経路タイプ
    pub route_type: i32,
    /// 経路URL
    pub route_url: Option<String>,
    /// 経路色
    pub route_color: Option<String>,
    /// 経路文字色
    pub route_text_color: Option<String>,
    /// 路線ID
    pub jp_parent_route_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoutesJp {
    /// 経路ID
    pub route_id: String,
    /// ダイヤ改正日
    pub route_update_date: Option<String>,
    /// 起点
    pub origin_stop: Option<String>,
    /// 経過地
    pub via_stop: Option<String>,
    /// 終点
    pub destination_stop: Option<String>,
}

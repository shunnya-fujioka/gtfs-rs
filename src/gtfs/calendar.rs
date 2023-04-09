use serde::{Deserialize, Serialize};

/// 運行区分情報(calendar.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct Calendar {
    /// 運行日ID
    pub service_id: String,
    /// 月曜日
    /// * `0` - 非運行
    /// * `1` - 運行
    pub monday: u8,
    /// 火曜日
    /// * `0` - 非運行
    /// * `1` - 運行
    pub tuesday: u8,
    /// 水曜日
    /// * `0` - 非運行
    /// * `1` - 運行
    pub wednesday: u8,
    /// 木曜日
    /// * `0` - 非運行
    /// * `1` - 運行
    pub thursday: u8,
    /// 金曜日
    /// * `0` - 非運行
    /// * `1` - 運行
    pub friday: u8,
    /// 土曜日
    /// * `0` - 非運行
    /// * `1` - 運行
    pub saturday: u8,
    /// 日曜日
    /// * `0` - 非運行
    /// * `1` - 運行
    pub sunday: u8,
    /// サービス開始日
    pub start_date: String,
    /// サービス終了日
    pub end_date: String,
}

/// 運行日情報(calendar_dates.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct CalendarDates {
    /// サービスID
    pub service_id: String,
    /// 日付
    pub date: String,
    /// 利用タイプ
    /// * `1` - 運行区分適用
    /// * `2` - 運行区分非適用
    pub exception_type: u8,
}

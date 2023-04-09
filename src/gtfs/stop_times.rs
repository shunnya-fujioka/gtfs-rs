use serde::{Deserialize, Serialize};

/// 通過時刻情報(stop_times.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct StopTimes {
    /// 便ID
    pub trip_id: String,
    /// 到着時刻
    pub arrival_time: String,
    /// 出発時刻
    pub departure_time: String,
    /// 標柱ID
    pub stop_id: String,
    /// 通過順位
    pub stop_sequence: i32,
    /// 停留所行先
    pub stop_headsign: Option<String>,
    /// 乗車区分
    /// * `0` - 通常の乗車地
    /// * `1` - 乗車不可能
    /// * `2` - 交通機関に乗車予約の電話が必要
    /// * `3` - 運転手への事前連絡が必要
    pub pickup_type: Option<i32>,
    /// 降車区分
    /// * `0` - 通常の降車地（ブザーを押して申告する一般的な停留所を含む）
    /// * `1` - 乗車不可能
    /// * `2` - 交通機関に乗車予約の電話が必要
    /// * `3` - 乗車時に運転手への事前連絡が必要
    pub drop_off_type: Option<i32>,
    /// 通算距離
    pub shape_dist_traveled: Option<i32>,
    /// 発着時間精度
    pub time_point: Option<()>,
}

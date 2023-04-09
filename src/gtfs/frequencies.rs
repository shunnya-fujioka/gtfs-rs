use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
/// 運行間隔情報(frequencies.txt)
pub struct Frequencies {
    /// 便ID
    pub trip_id: String,
    /// 開始時刻
    pub start_time: String,
    /// 終了時刻
    pub end_time: String,
    /// 運行間隔
    pub headway_secs: u32,
    /// 案内精度
    /// * `0` - 時刻を案内しない
    /// * `1` - 時刻を案内する
    pub exact_times: Option<u8>,
}

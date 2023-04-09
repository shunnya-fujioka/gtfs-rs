use serde::{Deserialize, Serialize};

/// 乗継情報(transfers.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct Transfers {
    /// 乗継元標柱ID
    pub from_stop_id: String,
    /// 乗継先標柱ID
    pub to_stop_id: String,
    /// 乗継タイプ
    /// * `0` - 2つのルート間の推奨乗継地点
    /// * `1` - 2つのルート間の時間が考慮された乗継地点
    /// * `2` - 乗継には、最低限の乗継時間が必要
    /// * `3` - ルート間の乗り継ぎが不可能
    pub transfer_type: u8,
    /// 乗継時間
    pub min_transfer_time: Option<u32>,
}

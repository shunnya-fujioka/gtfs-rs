use serde::{Deserialize, Serialize};

/// 描画情報(shapes.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct Shapes {
    /// 描画ID
    pub shape_id: String,
    /// 描画緯度
    pub shape_pt_lat: String,
    /// 描画経度
    pub shape_pt_lon: String,
    /// 描画順序
    pub shape_pt_sequence: i32,
    /// 描画距離
    pub shape_dist_traveleded: Option<f64>,
}

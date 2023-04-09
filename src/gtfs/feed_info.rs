use serde::{Deserialize, Serialize};

/// 提供情報(feed_info.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct FeedInfo {
    /// 提供組織名
    pub feed_publisher_name: String,
    /// 提供組織URL
    pub feed_publisher_url: String,
    /// 提供言語
    pub feed_lang: String,
    /// 有効期間開始日有効期間終了日
    pub feed_start_date_feed_end_date: String,
    /// 提供データバージョン
    pub feed_version: String,
}

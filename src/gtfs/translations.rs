use serde::{Deserialize, Serialize};

/// 翻訳情報(translations.txt)
#[derive(Debug, Deserialize, Serialize)]
pub struct Translations {
    /// 翻訳元日本語
    pub trans_id: String,
    /// 言語
    pub lang: String,
    /// 翻訳先言語
    pub translation: String,
}

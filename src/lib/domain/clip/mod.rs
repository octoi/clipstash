pub mod field;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clip {
    pub clip_id: field::ClipId,
    pub short_code: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub date: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
}
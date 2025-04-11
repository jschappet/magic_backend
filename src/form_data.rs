use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FormData {
    name: String,
    organization: String,
    email: String,
    category: String,
    website: Option<String>,
    title: String,
    introduction: Option<String>,
    relevance: Option<String>,
    resources: Option<String>,
    books: Option<String>,
    videos: Option<String>,
    location: Option<String>,
    contact_info: Option<String>,
    social_media: Option<String>, 

}
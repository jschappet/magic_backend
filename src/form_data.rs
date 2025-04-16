use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug , Clone )]
#[serde(rename_all = "snake_case")] // Automatically converts field names to snake_case

pub struct FormData {
    name: String,
    pub title: String,
    organization: Option<String>,
    email: String,
    category: String,
    website: Option<String>,
    introduction: Option<String>,
    relevance: Option<String>,
    resources: Option<String>,
    books: Option<String>,
    videos: Option<String>,
    location: Option<String>,
    contact_info: Option<String>,
    social_media: Option<String>, 

}


/* 
What is the entry -- Title 
What is the category -- Category
Summary -- Introduction
Pertinence  -- Relevance
Links -- Resources
Location (Maaaybe. Not sure) -- location
Contact Info -- Contact Info
*/
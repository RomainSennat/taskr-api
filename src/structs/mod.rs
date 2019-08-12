// Import crate and mod
extern crate chrono;

use bson::ordered::OrderedDocument;
use mongodb::db::Database;
use chrono::{DateTime, Utc};

// Application state
#[derive(Clone, Debug)]
pub struct AppState {
    pub db_client: Database
}

// Configuration file definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub dbauthenticate: bool,
    pub dbname: String,
    pub dbpassword: String,
    pub dbpath: String,
    pub dbport: u16,
    pub dbuser: String
}

// Task definition
#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    description: String,
    expire_at: Option<String>,
    name: String
}

impl Task {
    // Convert Task to OrderedDocument
    pub fn to_doc(&self) -> OrderedDocument {
        // Create document
        let mut document: OrderedDocument = doc! {
            "description": &self.description,
            "name": &self.name,
        };
        // Add optional fields
        if let Some(date) = &self.expire_at {
            document.insert("expire_at", DateTime::from(DateTime::parse_from_rfc3339(date).unwrap()));
        }
        // Add metadata fields
        document.insert("created_at", DateTime::from(Utc::now()));
        document
    }
}

impl ::std::default::Default for Configuration {
    // Default configuration object
    fn default() -> Self {
        Self {
            dbauthenticate: false,
            dbname: "taskr".into(),
            dbpassword: "".into(),
            dbpath: "localhost".into(),
            dbport: 27017,
            dbuser: "".into()
        }
    }
}

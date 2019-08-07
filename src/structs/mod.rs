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
    expire_at: String,
    name: String
}

impl Task {
    // Convert Task to OrderedDocument
    pub fn to_doc(&self) -> OrderedDocument {
        doc! {
            "description": &self.description,
            "expire_at": DateTime::from(DateTime::parse_from_rfc3339(&self.expire_at).unwrap()),
            "name": &self.name,
            "created_at": DateTime::from(Utc::now())
        }
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

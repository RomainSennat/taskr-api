use bson::ordered::OrderedDocument;
use mongodb::db::Database;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_client: Database
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    name: String,
    description: String
}

impl Task {
    pub fn to_doc(&self) -> OrderedDocument {
        doc! {
            "name": &self.name,
            "description": &self.description
        }
    }
}

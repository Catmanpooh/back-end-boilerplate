use serde::{Serialize, Deserialize};
use juniper::{GraphQLObject, GraphQLInputObject};
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Photo {
    pub id: String, 
    pub user_id: String, 
    pub file: String,
    pub file_name: String, 
    pub created_at: DateTime<Utc>
}

#[derive(GraphQLInputObject)] 
pub struct CreatePhoto{
    pub user_id: String, 
    pub file: String,
    pub file_name: String, 
}

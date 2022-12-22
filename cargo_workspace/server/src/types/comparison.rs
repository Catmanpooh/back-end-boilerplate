use serde::{Serialize, Deserialize};
use juniper::{GraphQLObject, GraphQLInputObject};
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Comparison {
    pub id: String, 
    pub user_id: String, 
    pub photo_id_1: String, 
    pub photo_id_2: String, 
    pub comparison_text: String, 
    pub created_at: DateTime<Utc>
}

#[derive(GraphQLInputObject)]
pub struct CreateComparison {
    pub user_id: String, 
    pub photo_id_1: String, 
    pub photo_id_2: String, 
    pub comparison_text: String, 
}
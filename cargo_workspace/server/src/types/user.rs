use serde::{Serialize, Deserialize};
use juniper::{GraphQLObject, GraphQLInputObject};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, GraphQLObject)]
pub struct User {
    pub id: String,
    pub user_name: String, 
    pub first_name: String, 
    pub last_name: String, 
    pub created_at: DateTime<Utc>	
}

#[derive(GraphQLInputObject)]
pub struct CreateUser {
    pub user_name: String, 
    pub first_name: String, 
    pub last_name: String,  
}
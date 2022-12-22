use super::context::Context;
use juniper::futures::TryFutureExt;
use juniper::{graphql_object,  FieldError};
use chrono::{DateTime, Utc};
use sqlx_core::pool::PoolConnection;
use sqlx_core::postgres::Postgres;
use sqlx_core::{executor::Executor, row::Row};

use crate::types::user::User;
use crate::types::photo::Photo;
use crate::types::comparison::Comparison;


pub struct Query;

#[graphql_object(context = Context)]
impl Query {

    async fn users(context: &Context) -> Result<Vec<User>, FieldError> {
        let mut pool_connection = context.0.conn().await?;

        let row = &pool_connection
            .fetch_all("SELECT * FROM users")
            .await?;
            
            
        let result = row.iter()
            .map(|r| User {
                id: r.get::<String, _>("id"),
                user_name: r.get::<String, _>("username"),
                first_name: r.get::<String, _>("first_name"),
                last_name: r.get::<String, _>("last_name"),
                created_at: r.get::<DateTime<Utc>, _>("created_at"),
            })
            .collect::<Vec<User>>();


        Ok(result)

    }

    async fn user(context: &Context, username: Option<String>, id: Option<String>) -> Result<User, FieldError> {
        let mut pool_connection = context.0.conn().await?;

        let sql_statment;

        if username.is_some() {
            sql_statment = format!("SELECT * FROM users WHERE username = '{}'", username.expect("Username was not provieded! try again"));
        } else {
            sql_statment = format!("SELECT * FROM users WHERE id = '{}'", id.expect("Id was not provieded! try again"));
        }

        let row = &pool_connection
        .fetch_one(&*sql_statment)
        .await?;

         Ok(User {
                id: row.get::<String, _>("id"),
                user_name: row.get::<String, _>("username"),
                first_name: row.get::<String, _>("first_name"),
                last_name: row.get::<String, _>("last_name"),
                created_at: row.get::<DateTime<Utc>, _>("created_at"),
            })
    }

    async fn photos(context: &Context, user_id: String) -> Result<Vec<Photo>, FieldError> {
        let mut pool_connection = context.0.conn().await?;

        let sql_statment: &str = &format!("SELECT * FROM photos WHERE user_id = '{}'", user_id);

        let row = &pool_connection
            .fetch_all(sql_statment)
            .await?;

        let result = row   
            .iter()
            .map(|r| Photo {
                id: r.get::<String, _>("id"),
                user_id: r.get::<String, _>("user_id"),
                file: r.get::<String, _>("file"),
                file_name: r.get::<String, _>("file_name"),
                created_at: r.get::<DateTime<Utc>, _>("created_at")
            })
            .collect::<Vec<Photo>>();

        
        Ok(result)
    }

    async fn photo(context: &Context, photo_id: String) -> Result<String, FieldError> {
        let mut pool_connection = context.0.conn().await?;

        let sql_statment: &str = &format!("SELECT file FROM photos WHERE id = '{}'", photo_id);

        let row = &pool_connection
        .fetch_one(sql_statment)
        .await?;

        
        Ok(row.get::<String, _>("file"))
    }

    async fn comparisons(context: &Context, user_id: String) -> Result<Vec<Comparison>, FieldError> {
        let mut pool_connection = context.0.conn().await?;

        let sql_statment: &str = &format!("SELECT * FROM comparison WHERE user_id = '{}'", user_id);

        let row = &pool_connection
            .fetch_all(sql_statment)
            .await?;
        

        let result = row
            .iter()
            .map(|r| 
                Comparison {
                id: r.get::<String, _>("id"),
                user_id: r.get::<String, _>("user_id"),
                photo_id_1: r.get::<String, _>("photo_id_1"),
                photo_id_2: r.get::<String, _>("photo_id_2"),
                comparison_text: r.get::<String, _>("comparison_text"),
                created_at: r.get::<DateTime<Utc>, _>("created_at")
            })
            .collect::<Vec<Comparison>>();

        
        Ok(result)
        
    }
    

    async fn comparison(context: &Context, id: String) -> Result<Comparison, FieldError> {
        let mut pool_connection = context.0.conn().await?;

        let sql_statment: &str = &format!("SELECT * FROM comparison WHERE id = '{}'", id);

        let row = &pool_connection
        .fetch_one(sql_statment)
        .await?;

         Ok(Comparison {
            id: row.get::<String, _>("id"),
            user_id: row.get::<String, _>("user_id"),
            photo_id_1: row.get::<String, _>("photo_id_1"),
            photo_id_2: row.get::<String, _>("photo_id_2"),
            comparison_text: row.get::<String, _>("comparison_text"),
            created_at: row.get::<DateTime<Utc>, _>("created_at")
            })
    }


}


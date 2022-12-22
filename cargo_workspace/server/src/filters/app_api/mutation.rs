use super::context::Context;
use juniper::{graphql_object, FieldError};
use sqlx_core::{executor::Executor, row::Row};

use crate::types::user::CreateUser;
use crate::types::photo::CreatePhoto;
use crate::types::comparison::CreateComparison;

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
   
    async fn create_user(context: &Context, input: CreateUser) -> Result<String, FieldError> {
        let mut pool_connection = context.0.conn().await?;

        let sql_statment: &str = &format!("INSERT INTO users
            (username, first_name, last_name)
            VALUES('{}', '{}', '{}')",
            input.user_name, input.first_name, input.last_name
        );

        let row = &pool_connection
            .execute(sql_statment)
            .await?; 
        
        let fetch_sql_statment: &str = &format!("SELECT * FROM users WHERE username = '{}'", input.user_name);

        let fetch_row = &pool_connection
            .fetch_one(fetch_sql_statment)
            .await?;

            let id = fetch_row.get::<String, _>("id");
        
        Ok(id)
        
    }  
    
    async fn create_photos(context: &Context, input: CreatePhoto) -> Result<i32, FieldError> {
        let mut pool_connection = context.0.conn().await?;

        let sql_statment: &str = &format!("INSERT INTO photos
            (user_id, file, file_name)
            VALUES('{}', '{}', '{}')",
            input.user_id, input.file, input.file_name
        );

        let row = &pool_connection
            .execute(sql_statment)
            .await?;

        Ok(row.rows_affected() as i32)
    }

    async fn create_comparison(context: &Context, input: CreateComparison) -> Result<i32, FieldError> {
        let mut pool_connection = context.0.conn().await?;

        let sql_statment: &str = &format!("INSERT INTO comparison
            (user_id, photo_id_1, photo_id_2, comparison_text)
            VALUES('{}', '{}', '{}', '{}')",
            input.user_id, input.photo_id_1, input.photo_id_2, input.comparison_text
        );

        let row = &pool_connection
            .execute(sql_statment)
            .await?;

        Ok(row.rows_affected() as i32)
    }

}


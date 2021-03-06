use super::uuid::APITokenID;
use crate::schema::api_tokens;
use crate::ConnPool;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Identifiable, Insertable, Queryable};
use serde::Serialize;

#[derive(Debug, Serialize, Identifiable, Queryable, Associations, Insertable)]
#[table_name = "api_tokens"]
#[primary_key(token)]
pub struct APIToken {
    token: APITokenID,
    user_name: String,
}

impl APIToken {
    /// Get token by token ID.
    pub async fn get_token(id: APITokenID, pool: &ConnPool) -> Result<Self, Error> {
        use crate::schema::api_tokens::dsl::*;
        pool.run(move |conn| api_tokens.filter(token.eq(&id)).first(&conn))
            .await
    }

    /// Check whether a token is in our database.
    pub async fn check_token(id: APITokenID, pool: &ConnPool) -> bool {
        match Self::get_token(id, pool).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Delete a token by its ID.
    pub async fn delete(self, pool: &ConnPool) -> Result<usize, Error> {
        use crate::schema::api_tokens::dsl::*;
        pool.run(move |conn| diesel::delete(api_tokens.find(self.token)).execute(&conn))
            .await
    }
}

#[derive(Insertable)]
#[table_name = "api_tokens"]
pub struct NewApiToken {
    token: APITokenID,
    user_name: String,
}

impl NewApiToken {
    /// Construct a new api token.
    pub fn new(token: APITokenID, user_name: String) -> Self {
        NewApiToken { token, user_name }
    }

    /// Insert our new api toke into database.
    pub async fn insert(self, pool: &ConnPool) -> Result<APIToken, Error> {
        use crate::schema::api_tokens::dsl::*;
        pool.run(move |conn| {
            diesel::insert_into(api_tokens)
                .values(&self)
                .get_result(&conn)
        })
        .await
    }
}

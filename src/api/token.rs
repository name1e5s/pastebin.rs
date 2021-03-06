use crate::models::*;
use crate::utils::APIResponse::*;
use crate::utils::Result;
use crate::utils::*;
use crate::ConnPool;
use serde::{Deserialize, Serialize};
use tide::*;

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: APITokenID,
}

pub async fn logout(mut req: Request<ConnPool>) -> Result {
    let token: Token = req.body_json().await?;
    let pool = req.state();
    let token = APIToken::get_token(token.token, &pool).await?;
    let result = token.delete(&pool).await?;
    Valid(new_api_result(&result))
}

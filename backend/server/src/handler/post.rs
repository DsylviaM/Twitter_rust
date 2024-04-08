use std::clone;

use axum::{async_trait, Json};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use rand::rngs;
use tracing::info;
use uchat_endpoint::{post::endpoint::{NewPost, NewPostOk}, user::endpoint::{CreateUser, CreateUserOk, Login, LoginOk}};
use uchat_query::{post::Post, session::{self, Session}};
use uchat_domain::ids::*;

use crate::{error::ApiResult, extractor::{DbConnection, UserSession}, AppState};

use super::AuthorizedApiRequest;

// #[derive(Clone)]
// pub struct AuthorizedApiRequest(String);

#[async_trait]
impl AuthorizedApiRequest for NewPost{
    type Response = (StatusCode, Json<NewPostOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    )-> ApiResult<Self::Response> {
        let post = Post::new(session.user_id, self.content, self.options)?;
        let post_id = uchat_query::post::new(&mut conn, post)?;

        Ok((StatusCode::OK, Json(NewPostOk {post_id})))
    }
}
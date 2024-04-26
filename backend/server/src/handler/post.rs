use std::clone;

use axum::{async_trait, Json};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use rand::rngs;
use tracing::info;
use uchat_endpoint::{post::{endpoint::{NewPost, NewPostOk, TrendingPosts, TrendingPostsOk}, types::{Like_status, PublicPost}}, user::endpoint::{CreateUser, CreateUserOk, Login, LoginOk}, RequestFailed};
use uchat_query::{post::Post, session::{self, Session}, AsyncConnection};
use uchat_domain::{ids::*, Username};

use crate::{error::{ApiError, ApiResult}, extractor::{DbConnection, UserSession}, AppState};

use super::AuthorizedApiRequest;

pub fn to_public(
    conn: &mut AsyncConnection,
    post: Post,
    session: Option<&UserSession>,
) -> ApiResult<PublicPost> {
    use uchat_query::post as query_post;
    use uchat_query::user as query_user;

    if let Ok(mut content) = serde_json::from_value(post.content.0) {
        Ok(PublicPost {
         id: post.id,
         by_user: {
            let profile = query_user::get(conn, post.user_id)?;
            super::user::to_public(profile)?
         },
         content,
         time_posted: post.time_posted,
         reply_to: {
            match  post.reply_to {
                Some(other_post_id) => {
                    let original_post = query_post::get(conn, other_post_id)?;
                    let original_user = query_user::get(conn, original_post.user_id)?;
                    Some((
                        Username::new(original_user.handle).unwrap(),
                        original_user.id,
                        other_post_id,
                        ))
                    }
                None => None,
            }
         },
         like_status: Like_status::NoReaction,
         bookmarked: false,
         boosted: false,
         likes: 0,
         dislikes: 0,
         boosts: 0,
        })
    } else {
        Err(ApiError {
            code: Some(StatusCode::INTERNAL_SERVER_ERROR),
            err: color_eyre::Report::new(RequestFailed {
                msg: "invalid post data".to_string(),
            }),
        })
    }
}

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

#[async_trait]
impl AuthorizedApiRequest for TrendingPosts{
    type Response = (StatusCode, Json<TrendingPostsOk>);
    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        state: AppState,
    )-> ApiResult<Self::Response> {
         
    }
}
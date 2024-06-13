pub mod user;
pub mod post;

use serde::{Deserialize, Serialize};
use load_dotenv::load_dotenv;
use thiserror::Error;

pub trait Endpoint {
    const URL: &'static str;
    fn url(&self) -> &'static str {
        Self::URL
    }
}

//создадим макрос для URL
macro_rules! route {
    ($url:literal => $request_type:ty) => {
        impl Endpoint for $request_type {
            const URL: &'static str = $url;
        }
    };
}

//создаем тип ответа на ошибку
#[derive(Error, Debug, Deserialize, Serialize)]
#[error("{msg}")]
pub struct RequestFailed {
    pub msg: String,
}

load_dotenv!();

pub mod app_url {
    use std::str::FromStr;

    use url::Url;

    pub const API_URL: &str = env!("API_URL");

    pub fn domain_and(fragment: &str) -> Url {
        Url::from_str(API_URL)
            //"http://127.0.0.1:8070/"
            .and_then(|url| url.join(fragment))
            //img/happy.png
            .unwrap()
            //"http://127.0.0.1:8070/img/happy.png"
        }

        pub mod user_content {
            pub const ROOT: &str = "usercontent/";
            pub const IMAGES: &str = "img/";
        }
}

//public routes
route!("/account/create" => user::endpoint::CreateUser);
route!("/account/login" => user::endpoint::Login);
//route!("/profile/view" => user::endpoint::ViewProfile);

//authorized routes
route!("/post/new" => post::endpoint::NewPost);
route!("/post/bookmark" => post::endpoint::Bookmark);
route!("/post/boost" => post::endpoint::Boost);
route!("/post/vote" => post::endpoint::Vote);
route!("/post/react" => post::endpoint::React);
route!("/post/trending" => post::endpoint::TrendingPosts);
route!("/post/home" => post::endpoint::HomePosts);
route!("/post/liked" => post::endpoint::LikedPosts);
route!("/post/bookmarked" => post::endpoint::BookmarkedPosts);
route!("/profile/me" => user::endpoint::GetMyProfile);
route!("/profile/update" => user::endpoint::UpdateProfile);
route!("/profile/view" => user::endpoint::ViewProfile);



#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Update<T>{
    Change(T),
    NoChange,
    SetNull
}

impl<T> Update<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Change(data) => Some(data),
            Self::NoChange => None,
            Self::SetNull => None,
        }
    }

    pub fn into_nullable(self) -> Option<Option<T>> {
        match self {
            Self::Change(data) => Some(Some(data)),
            Self::NoChange => None,
            Self::SetNull => Some(None),
        }
    }
}
pub mod register;
pub mod login;
pub mod home;
pub mod new_post;
pub mod trending;
pub mod edit_profile;

pub use login::Login;
pub use register::Register;
pub use home::{bookmarked::HomeBookmarked, liked::HomeLiked,Home};
pub use new_post::*;
pub use trending::Trending;
pub use edit_profile::EditProfile;

pub use route::*;

pub mod route {
    pub const ACCOUNT_REGISTER: &str = "/account/register";
    pub const ACCOUNT_LOGIN: &str = "/account/login";
    pub const HOME: &str = "/home";
    pub const HOME_BOOKMARKED: &str = "/home/bookmarked";
    pub const HOME_LIKED: &str = "/home/liked";
    pub const POST_NEW_CHAT: &str = "/post/new_chat";
    pub const POST_NEW_IMAGE: &str = "/post/new_image";
    pub const POST_NEW_POLL: &str = "/post/new_poll";
    pub const POST_TRENDING: &str = "/posts/trending";
    pub const PROFILE_EDIT: &str = "/profile/edit";

}
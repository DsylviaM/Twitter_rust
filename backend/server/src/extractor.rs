use std::str::FromStr;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts, Extension, Json, RequestPartsExt};
use hyper::{header, StatusCode};
use tracing::info;
use uchat_domain::ids::{SessionId, UserId};
use uchat_endpoint::RequestFailed;
use uchat_query::{session::{self, Session}, OwnedAsyncConnection};

use crate::AppState;
//имя нашего экстрактор DbConnection, когда пишем экстарктор нам нужна оболочка
pub struct DbConnection(pub OwnedAsyncConnection);

//https://docs.rs/axum/latest/axum/?search=fromrequesparts часть кода из документации https://docs.rs/axum/0.7.4/axum/extract/index.html#customizing-extractor-responses
#[async_trait]
impl<S> FromRequestParts<S> for DbConnection
where
    S: Send + Sync,
    {
        //тип важен, потому что мы указываем тип возврата, мы просто возвращаем код состояния с помощью сообщения
        type Rejection = (StatusCode, &'static str);
        //асинхронизация функции, которая из документации. Здесь Extension<AppStatу> расширение по слоям, которое мы добавили и состояния будут от туда
        //state.db_pool здесь мы получаем доступ к (state)состоянию просматриваем пул (db_pool) получаем собственное соединение get_owned ожидая (await) пока оно станет доступным и если что то пошло не так выводим сообщение 
        async fn from_request_parts(parts: &mut Parts, _:&S) -> Result<Self, Self::Rejection>{
            let Extension(state) = parts.extract::<Extension<AppState>>().await.unwrap();
            let connection = state.db_pool.get_owned().await.map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to connect to database"
                )
            })?; //используем оператор (?) т.о. он может быть автоматически преобразован в type Rejection
            //и если мы получим соединение мы оборачиваем его в этот тип подключения к базе данных DbConnection и отправляем в путь
            Ok(Self(connection))
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct UserSession {
    pub user_id: UserId,
    pub session_id: SessionId,
}

#[async_trait]
impl<S> FromRequestParts<S> for UserSession
where
    S: Send + Sync,
    {
        type Rejection = (StatusCode, Json<RequestFailed>);
        async fn from_request_parts(parts: &mut Parts, _:&S) -> Result<Self, Self::Rejection> {
            let unauthorized = || {
                (
                StatusCode::UNAUTHORIZED,
                Json(RequestFailed{
                    msg: "unauthorized".into(),
                }),
                )
            };

            let DbConnection(mut conn) = parts.extract::<DbConnection>().await.unwrap();
            let Extension(state) = parts.extract::<Extension<AppState>>().await.unwrap();

            let cookies = parts
                .headers
                .get(header::COOKIE)
                .and_then(|header| header.to_str().ok())
                .ok_or_else(unauthorized)?;
            let session_id = uchat_cookie::get_from_str(cookies, uchat_cookie::SESSION_ID)
                .and_then(|id| SessionId::from_str(id).ok())
                .ok_or_else(unauthorized)?;
            let session_signature = uchat_cookie::get_from_str(cookies, uchat_cookie::SESSION_SIGNATURE)
                .and_then(|sig| uchat_crypto::decode_base64(sig).ok())
                .and_then(|sig| uchat_crypto::sign::signature_from_bytes(sig).ok())
                .ok_or_else(unauthorized)?;

            state
                .signing_keys
                .verify(session_id.as_uuid().as_bytes(), session_signature)
                .map_err(|_|unauthorized())?;

            let session = uchat_query::session::get(&mut conn, session_id)
                .ok()
                .flatten()
                .ok_or_else(unauthorized)?;
            info!(
                user_id = session.user_id.into_inner().to_string(),
                "user logged in"
            );

            Ok(Self {
                user_id: session.user_id,
                session_id: session.id,
            })
        }
}
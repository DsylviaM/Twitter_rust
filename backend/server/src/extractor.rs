use axum::{async_trait, extract::FromRequestParts, http::request::Parts, Extension, RequestPartsExt};
use hyper::StatusCode;
use uchat_query::OwnedAsyncConnection;

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
use axum::response::IntoResponse;
use hyper::StatusCode;

pub struct ApiError {
    pub code: Option<StatusCode>,
    pub err:color_eyre::Report,
}
//err_responce возвращаем в формате json
pub fn err_responce<T: Into<String>>(
    code: StatusCode,
    msg: T) -> Response {
        (code, Json(
            uchat_endpoint::RequestFailed {
                msg: msg.into()
            }
        )).into_response()
    }


//используем intoResponce из библиотеки axum
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        if let Some(code) = self.code{
            return err_response(code, format!("{}", self.err));
        }
        return err_response(StatusCode::INTERNAL_SERVER_ERROR, "server error");
    }
}
//оператор вопросительного знака позволяет фиксировать все виды ошибок(следующий блок и автоматически присваивает тип  code: None, )
impl<E> From<E> for ApiError
where
    E: Into<color_eyre::Report>,
    {
        fn from(err: E) -> Self {
            Self { 
                code: None,
                err: err.into()}
        }
    }
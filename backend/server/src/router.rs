
use axum::{http::HeaderValue, routing::{get, post}, Router};
use hyper::{header::CONTENT_TYPE, Method};
use tower::ServiceBuilder;

use tower_http::{
    cors::CorsLayer,
trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
LatencyUnit,
};
use tracing::Level;
use uchat_endpoint::{user::endpoint::CreateUser, Endpoint};

use crate::{handler::with_public_handler, AppState};

//создадим новую функцию маршрутизатора, это требует некоторого места приложения,
// где мы вернем сюда маршрутизатор Axum
//мы создадим три маршрутизатора, возьмем два из них и обьединим их в третий маршрутизаторб
//чтобы наш первый маршрутизатор был общедоступным маршрутом, это будет те, которые всегда доступны
//нисмотря ни на что. Если посмотреть на вторую часть get то это и есть наш обработчик(метод)
pub fn new_router(state:AppState) -> axum::Router {
    let public_routes = Router::new()
    .route("/", get(move || async {"this is the root page"}))
    .route(CreateUser::URL, post(with_public_handler::<CreateUser>));
    // теперь маршрутизаторы по ИД пользователя
    let authorized_routes = Router::new();
    //затем настоящий маршрутизатор, который мы возвращаем
    Router::new()
        //обьединяем наши общедоступные маршруты
        .merge(public_routes)
        .merge(authorized_routes)
        //layer это уровни трассировки, здесь мы настраиваем ядра
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Micros),
                    ),
                )
                .layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])//получаем GET POST OPTIONS для отправки в серверную часть
                        .allow_credentials(true)//разрешающие учетные данные позволяют нас читать файлы cookie
                        .allow_origin(
                            std::env::var("FRONTEND_URL")
                            .unwrap()
                            .parse::<HeaderValue>()
                            .unwrap()
                        )
                        .allow_headers([CONTENT_TYPE]), //нам нужны заголовки типов контента
                )
                //слой расширения, этот слой будет присоединять произвольные данные к конвееру обработки слоя (processing pipeline)
                .layer(axum::Extension(state.clone())), //мы сможем получать доступ к состоянию слоя (здесь сотояние для слоев)
        )
        .with_state(state) //позволит нам получить доступ к состоянию нашего приложения (здесь сотояние для обработчиков)
}

/* С каждым маршрутизатором мы можем сделать промежуточное программное обеспечение,
 чтобы при поступление запроса мы могли проверить разные вещи
 например: вошли ли пользователи в систему и если все гуд, то разрешит вход, в противном случае
 если что то не так то он отклонит запрос "попробуйте заново войти в систему" или что то подобное нужно будет сделать пользователям
*/
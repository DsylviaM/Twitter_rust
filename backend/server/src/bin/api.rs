use std::net::SocketAddr;
use tracing::log::error;
use color_eyre::{Result, Help, eyre::Context};

use clap::{command, error, Parser, Subcommand};
use dotenvy::dotenv;
use tracing::{debug, info};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct cli {
    // api -d <URL>
    // api --database-url
    #[clap(
        short,
        long,
        default_value = "postgres://test@localhost/test",
        env = "API_DATABASE_URL")]
    database_url: String,

    #[clap(short, long, default_value = "127.0.0.1:8070", env = "API_BIND")]
    bind: SocketAddr,

    #[clap(flatten)]
    verbosity: uchat_server::logging::Verbosity,
    
    #[command(subcommand)]
    command: Option<Command>
}

#[derive(Debug, Subcommand)]
enum Command {
    //api gen-key--help
    ///generate a session signing key
    GenKey,
}

async fn run() -> Result<()> {

    //цвет сообщений
    color_eyre::install()?;
    //далее мы загрузим наш файл окружения с помощью envycreate и env загрузит файл env.
    //у нас есть файл примера env, который мы будем редактировать, и поэтому он загрузится автоматически
    let use_dotenv = dotenvy::dotenv();

    //Использую Cli parse, и это позволит получить все аргументы из командной строки.
    let args = cli::parse();

    //настроить ведение логов на сервере Uchat, поэтому мы включим ведение логов и установим уровень сложности на тот,
    //который указал пользователь.
    uchat_server::logging::setup(args.verbosity);

    //запишем сообщение, указывающее, загружаем ли мы файл окружения,
    // и если да, то какой именно, у нас будет отладочное сообщение

        /*Причина, по которой мы регистрируем фактический путь 
         к загружаемому env-файлу, заключается в том, что крейт envy собирается
        начать с текущего каталога и проверять каждый каталог до тех пор,
        пока не найдет env-файл для загрузки (%path.to_string_lossy()). */

    if let Ok(path) = use_dotenv {
        debug!(target: "uchat_server", dot_env_found = true, path = %path.to_string_lossy());
    } else 
        /* Так что мы можем оказаться в ситуации, когда у нас нет env-файла в текущей папке, но затем мы загружаем какой-то старый, находящийся в родительской директории, о котором мы забыли, и это нарушает нашу конфигурацию, а мы не хотим сидеть и думать, почему что-то не работает, и искать какие-то ошибки, которых не должно быть, потому что их не было бы,
        если бы у нас просто был загружен правильный env-файл.
        Вот почему у нас есть эта информация. А еще мы хотим записать, не загрузился ли какой-нибудь файл env, потому что, возможно, он там есть.
        Мы думаем, что он есть, но приложение его не находит.
     */
    {
        debug!(target: "uchat_server", dot_env_found = false);
    }

    // Далее мы хотим заняться генерацией ключей. Проверим, есть ли какая-нибудь команда, и если есть, то подберем к ней ключ.
        /*У нас есть только одна команда, это просто команда gen key. Мы можем добавить больше позже, если вы захотите,
        но в рамках этого проекта мы не будем делать больше. */
    
    if let Some(command) = args.command {
        match command {
            Command::GenKey => {
                //создаем генератор случайных чисел. Мы обратимся к созданному нами модулю и вызовем функцию gen keys.
                let mut rng = uchat_crypto::new_rng();
                info!(target: "uchat_server", "generating privat key...");
                //Мы обратимся к созданному нами модулю и вызовем функцию gen keys. Мы просто пропишем путь к ключу.
                //Таким образом, он будет находиться там, где находится произвольное приложение, и сохранит его в этом месте.
                let (key, _) = uchat_server::cli::gen_keys(&mut rng)?;
                
                //Вы можете добавить аргумент командной строки, чтобы задать свое собственное местоположение, но сейчас мы просто сделаем это жестким кодом.
                //key.as_str() сохраняется в path (std::fs::write(path, key.as_str())?;)
                let path = "private_key.base64";
                std::fs::write(path, key.as_str())?;
                info!(target: "uchat_server", path=path, "privat key saved to disk");
                info!(target: "uchat_server", path=path, "set API_PRIVATE_KEY environment variable with the content of the key in order to use it");
                return Ok(());
            }
        }
    }
    debug!(target: "uchat_server", "loadin signing keys");
    //загружаем наши ключи подписи используя нашу функцию
    let signing_keys = uchat_server::cli::load_keys()?;
    //создаем пул соединения с базой данных для чтения журнала, в котором мы устанавливаем соединения
    //мы извлечем URL-адрес базы данных из аргументов командной строки записываем,
    //потому что для подключения к базе данных может потребоваться время, а мы не хотим чтобы это выглядело как зависание
    info!(target: "uchat_server", database_url = args.database_url, "connecting to database");
    //этот пул асинхронных соединений является частью нашей библиотеки uchat
    //как и ассинхронное соединение это все лишь оболочка существующей функуиональности нашего crate
    //он асинхронный, поэтому нужно подождать,(.await) это не совсем сработает,
    //так как у нас будет некоторое журналирование(logging)
    //пропишем with_suggestion так как могут произойти распространенные вещи (например: подключение к базе впервый раз, когда может быть неверный URL, возможно у нас нет доступа к базе данных или она не создана) это лишь предложения которые будут в терминале
    let db_pool = uchat_query::AsyncConnectionPool::new(&args.database_url)
        .await
        .with_suggestion(|| "check database URL")
        .with_suggestion(|| "ensure correct database access rights")
        .with_suggestion(|| "make sure database exists")?;

    //создаем состояние нашего приложения, эта структура которая мы создали ранее
    let state = uchat_server::AppState{
        db_pool,
        signing_keys,
        rng: uchat_crypto::new_rng(),
    };

    //добавлю еще немного информации в журнал (logging)
    info!(target: "uchat_server", bind_addr = %args.bind);

    //создаю маршрутизатор, где new_router(state) маршрутизирует веб-запросы на правильный обработчик 
    let router = uchat_server::router::new_router(state);

    //затем мы запускаем e-server, поэтому мы используем AXM crate,это наш HTB server
    //мы используем try_bind таким путем, программа не аварийно заврешает работу, если адрес используется
    //так же мы добавим немного "air logging"
    
    let server = axum::Server::try_bind(&args.bind)
        .wrap_err_with(|| "server intialization error")
        .with_suggestion(|| "check bind address")
        .with_suggestion(|| "check if other services are using the same port")?;

    //и наконец мы запускаем сервер, мы возьмем наш маршрутизатор и запустим сервис make,
    //а он просто берет маршрутизатор, который мы будем создавать
    let server = server.serve(router.into_make_service());

    info!(target: "uchat_server", "listening");
    if let Err(e) = server.await {
        error!(target: "uchat_server", "server_error: {}", e);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
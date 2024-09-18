use teloxide::{prelude::*, types::{InlineKeyboardButton, InlineKeyboardMarkup}};
use warp::Filter;
use reqwest::Url;
use log::info;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // Загружаем переменные окружения из .env файла
    dotenv().ok();

    // Получаем URL и токен из переменных окружения
    let bot_token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
    
    // Инициализируем логгер
    pretty_env_logger::init();
    info!("Запуск приложения...");

    // Создаем Telegram бота
    let bot = Bot::new(bot_token).auto_send();

    // Запуск веб-сервера в фоновом потоке
    tokio::spawn(async {
        let hello = warp::path::end().map(|| warp::reply::html("Кликер-бот запущен!"));
        warp::serve(hello)
            .run(([127, 0, 0, 1], 3030))
            .await;
    });

    // Запуск обработки сообщений бота
    teloxide::repl(bot.clone(), |message| async move {
        if let Some(text) = message.update.text() {
            let url_str = env::var("URL").expect("URL must be set in .env");
            let url = Url::parse(&url_str).expect("Invalid URL");
            if text == "/start" {
                let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::url(
                    "Открыть веб-интерфейс".to_string(),
                    url.clone(),
                )]]);

                message
                    .answer("Привет! Нажми на кнопку, чтобы перейти на веб-интерфейс.")
                    .reply_markup(keyboard)
                    .send()
                    .await?;
            }
        }
        respond(())
    })
    .await;
}

[![Cargo build](https://github.com/ya7on/leopardybot/actions/workflows/build.yml/badge.svg)](https://github.com/ya7on/leopardybot/actions/workflows/build.yml)
[![Cargo clippy](https://github.com/ya7on/leopardybot/actions/workflows/clippy.yml/badge.svg)](https://github.com/ya7on/leopardybot/actions/workflows/clippy.yml)

# Leopardy - Телеграм бот для игры в викторины

![image](https://user-images.githubusercontent.com/7967826/200128044-44605293-c188-422a-af1a-9609113b0f36.png)

## Конфигурация

Актуальную информацию о конфигурации приложения можно получить в `--help`

<table>
    <tr>
        <th>Переменная окружения</th>
        <th>Описание</th>
        <th>Значение по умолчанию</th>
    </tr>
    <tr>
        <th colspan=4>Обязательные параметры</th>
    </tr>
    <tr>
        <td>LEO_TG_TOKEN</td>
        <td>Токен телеграм бота</td>
        <td></td>
    </tr>
    <tr>
        <td>LEO_HOST</td>
        <td>Хост, на котором доступен бот. При запуске приложения, бот устанавливает телеграм вебхук на этот хост + /api/v1/telegram</td>
        <td></td>
    </tr>
    <tr>
        <td>LEO_DB_URL</td>
        <td>Полная ссылка к базе данных (напр. postgresql://postgres:password@localhost:5432/leopardy)</td>
        <td></td>
    </tr>
    <tr>
        <th colspan=4>Необязательные параметры</th>
    </tr>
    <tr>
        <td>LEO_PORT</td>
        <td>Порт, который будет слушать веб сервер бота</td>
        <td>8888</td>
    </tr>
    <tr>
        <td>LEO_WORKERS</td>
        <td>Количество потоков, которые будут обрабатывать запросы к приложению</td>
        <td>4</td>
    </tr>
    <tr>
        <td>LEO_QUIZ_ROUND_TIME</td>
        <td>Количество секунд, которое дается на ответ пользователю. Может быть в промежутке от 5 до 600 включительно</td>
        <td>15</td>
    </tr>
    <tr>
        <td>LEO_QUIZ_ROUNDS_COUNT</td>
        <td>Количество раундов в одной игре</td>
        <td>5</td>
    </tr>
    <tr>
        <td>LEO_TG_SECRET_TOKEN</td>
        <td>Токен, который телеграм кладет в хедеры каждого запроса как доказательство, что запросы идут от созданного этим ботом вебхука. Если не указать, секретный токен будет сгенерирован при старте приложения. Стоит указывать только в том случае, когда у вас несколько копий приложения обрабатывают запросы от одного бота</td>
        <td></td>
    </tr>
    <tr>
        <td>LEO_TG_MAX_CONNECTION</td>
        <td>Максимальное количество соединений от Telegram. Может быть в промежутке от 1 до 100 включительно</td>
        <td>40</td>
    </tr>
    <tr>
        <td>LEO_CSV_PATH</td>
        <td>Путь к csv файлу с вопросами. Принимает также http URL. Пример структуры файла можно посмотреть <a href="questions/questions.csv">здесь</a></td>
        <td>questions/questions.csv</td>
    </tr>
</table>

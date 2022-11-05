# Leopardy - Телеграм бот для игры викторин

![image](https://user-images.githubusercontent.com/7967826/200128044-44605293-c188-422a-af1a-9609113b0f36.png)

## Конфигурация

<table>
    <tr>
        <th>Переменная окружения</th>
        <th>Флаг</th>
        <th>Описание</th>
        <th>Значение по умолчанию</th>
    </tr>
    <tr>
        <th colspan=4>Обязательные параметры</th>
    </tr>
    <tr>
        <td>LEO_TG_TOKEN</td>
        <td>-t --token</td>
        <td>Токен телеграм бота</td>
        <td></td>
    </tr>
    <tr>
        <td>LEO_HOST</td>
        <td>-H --host</td>
        <td>Хост, на котором доступен бот. При запуске приложения, бот устанавливает телеграм вебхук на этот хост + /api/v1/telegram</td>
        <td></td>
    </tr>
    <tr>
        <td>LEO_DB_URL</td>
        <td>-d --db</td>
        <td>Полная ссылка к базе данных (напр. postgresql://postgres:password@localhost:5432/leopardy)</td>
        <td></td>
    </tr>
    <tr>
        <th colspan=4>Необязательные параметры</th>
    </tr>
    <tr>
        <td>LEO_PORT</td>
        <td>-p --port</td>
        <td>Порт, который будет слушать веб сервер бота</td>
        <td>8888</td>
    </tr>
    <tr>
        <td>LEO_WORKERS</td>
        <td>--workers</td>
        <td>Количество потоков, которые будут обрабатывать запросы к приложению</td>
        <td>4</td>
    </tr>
    <tr>
        <td>LEO_QUIZ_ROUND_TIME</td>
        <td>--quiz-round-time</td>
        <td>Количество секунд, которое дается на ответ пользователю</td>
        <td>15</td>
    </tr>
</table>

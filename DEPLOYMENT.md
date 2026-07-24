# Развёртывание на VPS

Production временно разворачивается на `151-246-182-63.sslip.io` из GitHub
Actions при каждом успешном push в `master`. Registry не используется:
workflow собирает образ, передаёт его на VPS по SSH и запускает Docker Compose.

Проверки форматирования, Clippy и тесты запускаются для pull request в
`master`, а также после push в саму ветку `master`. Это исключает две
одинаковые проверки одного commit при открытом PR. Deploy job запускается
только для `master`; в pull request production secrets не используются.

Чтобы GitHub действительно запрещал merge с красными проверками, создайте в
`Settings -> Rules -> Rulesets` правило для ветки `master`:

1. включите `Require a pull request before merging`;
2. включите `Require status checks to pass`;
3. добавьте required check `Format, lint, and test`;
4. включите требование актуальной ветки перед merge и запрет force push.

Сам workflow показывает результат проверки, но без branch ruleset GitHub не
блокирует кнопку merge.

## Что запускается

- `app` — API и background worker `zero2prod`;
- `postgres` — PostgreSQL 17 с persistent volume;
- `redis` — Redis 7.4 с AOF и persistent volume;
- `caddy` — reverse proxy и автоматический TLS от Let's Encrypt.

Наружу опубликованы только TCP 80/443 и UDP 443. PostgreSQL, Redis и порт 8000
доступны только внутри Docker networks.

## 1. DNS и firewall

Временный адрес `151-246-182-63.sslip.io` автоматически резолвится в
`151.246.182.63`, поэтому отдельная DNS-запись для первого deploy не нужна.
Когда DNS `nickchursin.com` обновится, верните домен в `deploy/Caddyfile`,
замените `APP_APPLICATION__BASE_URL` в GitHub Secrets и выполните новый deploy.

Откройте SSH, HTTP и HTTPS. Например, для UFW:

```bash
sudo ufw allow OpenSSH
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw allow 443/udp
sudo ufw enable
```

## 2. Пользователь и каталог

На VPS должны быть установлены Docker Engine, Compose plugin и `flock`.
Пользователь `webadmin` должен подключаться по ключу, иметь доступ к Docker и владеть
deployment-каталогом:

```bash
sudo usermod -aG docker webadmin
sudo install -d -o webadmin -g webadmin -m 750 /opt/zero2prod
```

После добавления в группу `docker` переподключитесь по SSH. Членство в этой
группе фактически даёт root-доступ к серверу, поэтому используйте отдельный
ключ только для deployment.

## 3. GitHub Secrets

В `Settings -> Secrets and variables -> Actions` добавьте отдельные secrets:

| Secret                                  | Значение                                 |
| --------------------------------------- | ---------------------------------------- |
| `SSH_HOST`                              | IP или SSH hostname VPS                  |
| `SSH_PORT`                              | SSH port; можно оставить пустым для `22` |
| `SSH_USER`                              | `webadmin`                               |
| `SSH_PRIVATE_KEY`                       | Приватный ключ без passphrase            |
| `SSH_KNOWN_HOSTS`                       | Проверенная строка `known_hosts` для VPS |
| `APP_APPLICATION__BASE_URL`             | `https://151-246-182-63.sslip.io`        |
| `APP_APPLICATION__HMAC_SECRET`          | Длинный случайный ключ подписи           |
| `APP_DATABASE__USERNAME`                | Например `zero2prod`                     |
| `APP_DATABASE__PASSWORD`                | Длинный случайный пароль PostgreSQL      |
| `APP_DATABASE__DATABASE_NAME`           | Например `newsletter`                    |
| `APP_EMAIL_CLIENT__SENDER_EMAIL`        | Подтверждённый sender в Postmark         |
| `APP_EMAIL_CLIENT__AUTHORIZATION_TOKEN` | Postmark Server API Token                |

Сгенерировать HMAC secret и пароль БД можно локально:

```bash
openssl rand -hex 64
openssl rand -base64 48
```

Получите host key через доверенный канал. Например, выполните `ssh-keyscan`
локально, затем обязательно сравните fingerprint с ключом из консоли VPS:

```bash
ssh-keyscan -H -p 22 YOUR_VPS_IP
ssh-keygen -lf /etc/ssh/ssh_host_ed25519_key.pub
```

Не используйте `StrictHostKeyChecking=no`.

## 4. Первый deploy

Push в `master` запускает:

1. `cargo fmt --check`, Clippy и тесты с PostgreSQL/Redis;
2. сборку `linux/amd64` Docker image с SHA commit в качестве tag;
3. SSH upload образа, Compose-файла, Caddyfile и `.env`;
4. запуск стека и ожидание Docker healthcheck;
5. автоматический возврат предыдущего образа и `.env`, если app не стал healthy.

Workflow можно повторить вручную через `Actions -> Check and deploy -> Run
workflow`. Следите за первым выпуском:

```bash
cd /opt/zero2prod
export IMAGE_TAG="$(<.current-release)"
docker compose --env-file .env ps
docker compose --env-file .env logs -f app caddy
curl --fail https://151-246-182-63.sslip.io/healthz
```

Caddy получит сертификат только если DNS уже указывает на VPS и порты 80/443
доступны из интернета.

## 5. Admin после первого запуска

Миграция создаёт пользователя `admin` с фиксированным password hash. Эти
учётные данные не являются production-секретом. Сразу после первого входа
откройте `/admin/password` и установите уникальный длинный пароль. Не
публикуйте приложение до смены пароля.

## 6. Ручной rollback

Автоматический rollback срабатывает только во время неуспешного deploy.
Для ручного возврата сохранённого предыдущего релиза:

```bash
cd /opt/zero2prod
current="$(<.current-release)"
previous="$(<.previous-release)"
cp .env .env.swap
cp .env.previous .env
mv .env.swap .env.previous
IMAGE_TAG="$previous" docker compose --env-file .env up -d --remove-orphans
IMAGE_TAG="$previous" docker compose --env-file .env ps
printf '%s\n' "$current" > .previous-release
printf '%s\n' "$previous" > .current-release
```

Перед обновлением release markers убедитесь, что app стал healthy. Rollback
образа не откатывает SQL-миграции. Все миграции должны оставаться совместимыми
с предыдущей версией приложения (expand/contract).

## 7. Обслуживание и риски

Посмотреть логи и состояние:

```bash
cd /opt/zero2prod
export IMAGE_TAG="$(<.current-release)"
docker compose --env-file .env ps
docker compose --env-file .env logs --tail 200 app
docker compose --env-file .env logs --tail 200 postgres redis caddy
```

В текущей конфигурации автоматических backup PostgreSQL нет. Потеря VPS или
Docker volume приведёт к потере данных. До появления production-данных
настройте регулярный encrypted `pg_dump` во внешнее хранилище.

Изменение `APP_DATABASE__PASSWORD` в GitHub не меняет пароль уже созданного
пользователя внутри PostgreSQL volume. Сначала выполните `ALTER ROLE` в БД,
затем обновляйте secret и запускайте deploy.

Удалённый `spec.yaml` содержал production-like секреты. Удаление файла не
очищает историю Git: перед запуском ротируйте HMAC secret и Postmark token.

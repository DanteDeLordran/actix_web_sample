# Rust Web Server

Exploring the capabilities of Rust with clean architecture

## Stack used
<img alt="postgres" src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/postgresql/postgresql-original.svg" height=80 /> <img 
alt="docker" src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/docker/docker-original-wordmark.svg"  height=90 />

## How to run (dev mode)
1. Add env variables
```
ACTIVE_PROFILE
DB_USER
DB_PASSWORD
DB_HOST
DB_PORT
DB_NAME
PG_EMAIL
PG_PASSWORD
ALLOWED_ORIGIN
EMAIL_USER
EMAIL_PASSWORD
```
2. Docker compose
   The environment variables will be used to set a postgres DB alongside a pgAdmin GUI for managing it
```
docker compose up -d
```

## Build & run (Dockerfile)

1. Build docker image
```
docker build -t dpc-backend .
```

2. Pull image

```
docker pull dpc-backend
```
3. Run image

```
docker run -e DB_HOST=your_db_host -e DB_USER=your_db_user -e DB_PASSWORD=your_db_pass -e DB_NAME=your_db_name -e DB_PORT=your_db_port -p 80:80 dpc-backend
```

## Run migrations

This project uses Flyway as database migration and version manager. For creating a new migration you can simply add a new .sql file on the db folder at : **./src/main/resources/db** , when re-running the project , Spring will automatically perform the migration

## Swagger UI documentation
All the project is documented using OpenAPI aka Swagger , you can access the endpoint's and schema's documentation under the url
```
http://localhost:8080/api/swagger-ui/index.html
```
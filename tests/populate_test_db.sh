# this script should be run from the tests/ folder
. ./database_config.sh
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}
sqlx database create -D $DATABASE_URL
sqlx migrate run -D $DATABASE_URL

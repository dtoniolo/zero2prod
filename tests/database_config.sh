DB_FILE="test_data/test_db/db.postgresql"
# Check if a custom user has been set, otherwise default to 'postgres'
POSTGRES_USER=postgres
# Check if a custom password has been set, otherwise default to 'password'
POSTGRES_PASSWORD=`cat test_db_password.txt`
# Check if a custom database name has been set, otherwise default to 'newsletter'
POSTGRES_DB=newsletter
# this script should be run from the tests/ folder
. ./database_config.sh

initdb -D $DB_FILE --username=$POSTGRES_USER
postgres --config-file=postgresql.conf -D $DB_FILE

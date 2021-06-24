 FROM mysql:latest

 ADD ./mysql_schema.sql /docker-entrypoint-initdb.d
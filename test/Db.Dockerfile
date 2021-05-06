 FROM mysql:latest

 ADD ./schema.sql /docker-entrypoint-initdb.d
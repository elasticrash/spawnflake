 FROM postgres:latest

 ADD ./postgres_schema.sql /docker-entrypoint-initdb.d
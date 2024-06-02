# pgsql db
# docker build -t ntd-db . --no-cache
# docker run -d -p 5432:5432 --name ntd-db ntd-db
FROM postgres:16.3
ENV POSTGRES_USER=ntd
ENV POSTGRES_PASSWORD=ntd
ENV POSTGRES_DB=ntd
COPY ./conf/sql/* /docker-entrypoint-initdb.d/
EXPOSE 5432
CMD ["postgres"]

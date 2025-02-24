FROM node:20-bullseye-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y postgresql-client gdb git

ADD . .

RUN yarn install

EXPOSE 8080

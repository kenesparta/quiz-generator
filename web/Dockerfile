FROM node:20-bullseye-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y git

ADD . .

RUN yarn install

EXPOSE 3333

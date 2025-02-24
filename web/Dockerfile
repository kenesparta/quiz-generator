FROM node:23-bookworm

WORKDIR /app

RUN apt-get update && \
    apt-get install -y git

ADD . .

RUN yarn install

EXPOSE 3333

FROM alpine:latest AS build
RUN apk add --no-cache python3 py3-pip
RUN apk add rabbitmq-c-dev
WORKDIR /app
COPY . .
RUN pip3 install -r requirements.txt

CMD [ "python3","con.py" ]
FROM golang:1.17.2

RUN mkdir /app
WORKDIR /app
ADD . .
ENV PROD="TRUE"
RUN go build -o main main.go

CMD ["./main"]

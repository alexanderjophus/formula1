FROM golang:1.21 AS builder
WORKDIR /go/src/github.com/alexanderjoseph/formula1/formulagraphql
COPY . /go/src/github.com/alexanderjoseph/formula1/formulagraphql
RUN CGO_ENABLED=0 GOOS=linux go build -o app .

FROM gcr.io/distroless/base
COPY --from=builder /go/src/github.com/alexanderjoseph/formula1/formulagraphql/app ./app
CMD ["./app"]
# Build stage
FROM golang:1.17-alpine AS build
WORKDIR /app
COPY . .
RUN go mod download
RUN go build -o go-micro-service .

# Production stage
# FROM gcr.io/distroless/cc
# WORKDIR /app
# COPY --from=build /app/go-micro-service .
# CMD ["./go-micro-service"]
FROM alpine:3.14
WORKDIR /app
COPY --from=build /app/go-micro-service .
CMD ["./go-micro-service"]

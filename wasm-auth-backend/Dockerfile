FROM rustwasm/wasm-pack:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN wasm-pack build --target web

FROM scratch
COPY --from=builder /usr/src/app/pkg /pkg


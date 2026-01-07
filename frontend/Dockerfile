FROM docker.io/node:trixie AS builder

WORKDIR /app
COPY ./js .

RUN npm install 
RUN npm run build

FROM docker.io/nginx

WORKDIR /usr/share/nginx/html/
COPY index.html .
COPY --from=builder /app/dist/ .

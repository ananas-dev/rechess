FROM node:16-alpine as builder
WORKDIR /client
COPY . .
RUN yarn && yarn build

FROM node:16-alpine as runtime
WORKDIR /client
COPY --from=builder /client/build /client/build
CMD ["node", "build/index.js"]

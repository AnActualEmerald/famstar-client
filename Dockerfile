FROM node:lts-alpine
RUN npm install -g serve
USER node
WORKDIR /home/node/
COPY ./public ./public
EXPOSE 3000
CMD ["serve", "./public"]

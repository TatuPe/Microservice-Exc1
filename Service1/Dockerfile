FROM node:22

COPY package*.json ./

RUN npm install

COPY . .

CMD [ "npm", "start" ]
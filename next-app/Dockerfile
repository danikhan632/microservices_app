FROM node:latest

WORKDIR /app
COPY . .
RUN curl -fsSL https://bun.sh/install | bash
RUN npm install
# RUN curl https://hackgtstoragebucket.s3.amazonaws.com/bun > ./build/bun


RUN npm run build
RUN ls -a


CMD ["npm", "run"]

# docker build -t my-next-app .
# docker run -p 3000:3000 my-next-app

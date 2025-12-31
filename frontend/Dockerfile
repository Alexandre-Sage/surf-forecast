ARG VITE_API_HOST=http://localhost:8080/api
ARG BUN_VERSION=latest
ARG PORT=3001

FROM oven/bun:${BUN_VERSION} AS build
WORKDIR /app
ENV VITE_API_HOST=${VITE_API_HOST}

COPY bun.lock package.json ./
RUN bun install --production
COPY . .
RUN bun run build

FROM oven/bun:slim
WORKDIR /app
COPY --from=build /app/dist ./dist
RUN bun add -g serve

EXPOSE ${PORT}
CMD ["serve", "-s", "dist", "-l", "3001"]


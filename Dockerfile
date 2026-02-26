# Build stage
FROM node:18-alpine AS build

WORKDIR /app

# Copy package files
COPY frontend/package*.json ./

# Install dependencies
RUN npm ci

# Copy frontend source
COPY frontend/ ./

# Build the app (outputs to /dist due to vue.config.js)
RUN npm run build

# Production stage - serve with nginx
FROM nginx:alpine

# Copy built files from build stage (build outputs to /dist, one level up from /app)
COPY --from=build /dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

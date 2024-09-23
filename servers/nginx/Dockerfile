FROM nginx:1.27.1-alpine-slim
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 8000
CMD ["nginx", "-g", "daemon off;"]
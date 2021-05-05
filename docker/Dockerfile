FROM node as builder
COPY ./ /usr/local/src/crmquery
WORKDIR /usr/local/src/crmquery
RUN npm i
RUN npx webpack

FROM ubuntu:focal
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y \
    software-properties-common \
    supervisor \
    curl

RUN add-apt-repository ppa:ondrej/php
RUN apt update && apt install -y --no-install-recommends \
    nginx \
    php7.4 \
    php7.4-pdo \
    php7.4-mysql \
    php7.4-fpm \
    php7.4-curl

COPY ./docker/nginx.conf /etc/nginx/sites-available/nginx.conf
RUN rm /etc/nginx/sites-enabled/default
RUN ln -s /etc/nginx/sites-available/nginx.conf /etc/nginx/sites-enabled/nginx.conf

COPY --from=builder /usr/local/src/crmquery/dist/dist.js /var/www/html/dist/dist.js
COPY ./ /var/www/html/
RUN rm -rf /var/www/html/src/ /var/www/html/*.json /var/www/html/README.md /var/www/html/webpack.config.js /var/www/html/node_modules/ /var/www/html/.github/ /var/www/html/.gitignore

COPY ./docker/supervisord.conf /app/supervisord.conf
COPY ./docker/run.sh /app/run.sh
RUN chmod a+x /app/run.sh
RUN mkdir -p /var/log/nginx/ /var/log/php-fpm/ /var/run/php
RUN chown -R www-data:www-data /var/www/html
RUN chmod -R u+rw /var/www/html
RUN chmod -R g+wx /var/www/html

EXPOSE 80

CMD bash -c /app/run.sh
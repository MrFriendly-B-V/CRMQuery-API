#!/bin/bash

if [ -z "${ESPO_API_KEY}" ]
then
    echo "Environmental variable 'ESPO_API_KEY' is not set. This is required."
    exit 1
fi

if [ -z "${ESPO_URL}" ]
then
    echo "Environmental variable 'ESPO_URL' is not set. This is required."
    exit 1
fi

sed -i "s|{APIKEY}|${ESPO_API_KEY}|" /var/www/html/backend/credentials.php
sed -i "s|{URL}|${ESPO_URL}|" /var/www/html/backend/credentials.php

/usr/bin/supervisord -n -c /app/supervisord.conf
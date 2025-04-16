#!/bin/zsh 

docker buildx build  --platform linux/amd64 -t magic_backend .  

docker create --name magic_backend-container --platform linux/amd64 magic_backend 

docker cp magic_backend-container:/usr/src/app/magic_backend ./magic_backend

docker rm magic_backend-container

ssh www 'ps aux | grep magic_backend | grep -v grep | awk "{print \$2}" | xargs kill -9 '

# Sync the migrations, templates, static, and magic_backend files to the server
rsync -a migrations/ www:git/magic_backend/migrations
rsync -a templates/ www:git/magic_backend/templates
rsync -a static/ www:git/magic_backend/static
rsync -a magic_backend.sh magic_backend www:git/magic_backend

# Restart the server without waiting for the files to sync
ssh www -x 'cd git/magic_backend &&  chmod +x magic_backend.sh  && nohup ./magic_backend.sh  && tail magic_backend.log'


#!/usr/bin/bash

./magic_backend  > magic_backend.log 2>&1 &
echo $$ > pid
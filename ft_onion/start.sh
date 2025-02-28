#! /bin/bash

cd src/ && sudo docker build -t onion . && sudo docker run -d -p 4242 -p 80:80 onion

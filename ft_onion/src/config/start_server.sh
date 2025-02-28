#!/bin/bash

service tor start && service ssh start && nginx -g 'daemon off;'
FROM debian:latest

RUN apt update && apt-get install -y \
    nginx \
    openssh-server \
    tor \
    torsocks && \
    apt clean && rm -rf /var/lib/apt/lists/*

COPY ./config/start_server.sh /config/start_server.sh
COPY ./config/nginx.conf /etc/nginx/
COPY ./config/torrc /etc/tor/torrc
COPY ./config/index.html /var/www/localhost/
COPY ./config/sshd_config /etc/ssh/sshd_config

RUN	echo 'root:root' | chpasswd
RUN chmod +x ./config/start_server.sh

EXPOSE 80
EXPOSE 4242

CMD ["/bin/bash", "/config/start_server.sh"]
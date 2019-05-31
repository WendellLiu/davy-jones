FROM rustlang/rust:nightly-slim

ENV VERSION v2.5.1
ENV FILENAME helm-${VERSION}-linux-amd64.tar.gz
ENV KUBECTL v1.6.6


RUN set -ex \
  && apt-get update && apt-get install -y --no-install-recommends \
  curl \
  && curl -sLo /tmp/${FILENAME} http://storage.googleapis.com/kubernetes-helm/${FILENAME} \
  && curl -sLo /tmp/kubectl https://storage.googleapis.com/kubernetes-release/release/${KUBECTL}/bin/linux/amd64/kubectl \
  && tar -zxvf /tmp/${FILENAME} -C /tmp \
  && mv /tmp/linux-amd64/helm /bin/helm \
  && chmod +x /tmp/kubectl \
  && mv /tmp/kubectl /bin/kubectl \
  && rm -rf /tmp 

COPY ./ /app
WORKDIR /app

RUN cargo build --release --features "production"

CMD [ "ROCKET_SECRET_KEY=${ROCKET_SECRET_KEY} ./target/release/davi-jones" ]

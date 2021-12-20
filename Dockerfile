FROM debian:11-slim
ARG TARGETPLATFORM

ADD artifacts/${TARGETPLATFORM}/ble-scanner /usr/local/bin/ble-scanner
RUN chmod +x /usr/local/bin/ble-scanner

RUN apt-get update && apt-get install -y \
    tini \
    libdbus-1-3

ENTRYPOINT ["tini", "--", "/usr/local/bin/ble-scanner"]

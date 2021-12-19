FROM alpine
ARG TARGETPLATFORM

ADD target/artifacts/${TARGETPLATFORM}/ble-scanner /usr/local/bin/ble-scanner

ENTRYPOINT ["/usr/local/bin/ble-scanner"]

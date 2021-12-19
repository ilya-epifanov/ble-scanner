FROM alpine
ARG TARGETPLATFORM

ADD artifacts/${TARGETPLATFORM}/ble-scanner /usr/local/bin/ble-scanner

ENTRYPOINT ["/usr/local/bin/ble-scanner"]

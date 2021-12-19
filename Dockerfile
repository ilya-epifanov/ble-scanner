FROM alpine
ARG TARGETPLATFORM

ADD artifacts/${TARGETPLATFORM}/ble-scanner /usr/local/bin/ble-scanner
RUN chmod +x /usr/local/bin/ble-scanner

RUN apk add --no-cache tini dbus-libs libgcc libc6-compat gcompat
ENTRYPOINT ["tini", "--", "/usr/local/bin/ble-scanner"]

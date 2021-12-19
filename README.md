<span class="badge-patreon"><a href="https://www.patreon.com/smartislav" title="Donate to this project using Patreon"><img src="https://img.shields.io/badge/patreon-donate-yellow.svg" alt="Patreon donate button" /></a></span>
![example workflow](https://github.com/ilya-epifanov/ble-scanner/actions/workflows/build.yml/badge.svg)

# Bluetooth LE beacon scanner

## Installation

Grab the archive for your target OS/architecture from the [releases page](https://github.com/ilya-epifanov/ble-scanner/releases).

    sudo tar zxf ${PATH_TO}/ble-scanner.${TARGET}.tar.gz -C /

This will install the binary to `/opt/ble-scanner/bin/ble-scanner` and a systemd service unit to `/etc/systemd/system/ble-scanner.service`.
You'll need to replace the example parameters in the service file.

    sudo nano /etc/systemd/system/ble-scanner.service

After you've done that, you can enable and start the service in systemd.

    sudo systemctl enable --now ble-scanner.service

## Report structure

Every time an RSSI report is received from the BLE devices specified using `-b/--beacon` command line argument, 
the following message is sent to a topic `${--mqtt-prefix}${address}`
(e.g. `ble-scanner/ble-01:23:45:67:89:AB` with the default value of `--mqtt-prefix=ble-scanner/ble-`):

```
{
    "rssi": ${rssi value or null}
}
```

For example:

```
{
    "rssi": -56
}
```

An RSSI of `null` means the connection to the device has been lost.

```
{
    "rssi": null
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

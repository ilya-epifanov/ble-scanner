use bluer::{AdapterEvent, Address, DeviceEvent, DeviceProperty};
use futures::{StreamExt, pin_mut};
use clap::Parser;
use log::{info, warn};
use rumqttc::{MqttOptions, AsyncClient, QoS};
use serde_json::json;

use std::collections::HashSet;
use std::time::Duration;

async fn report_rssi(mqtt: &AsyncClient, prefix: &str, addr: &Address, rssi: Option<i16>) -> Result<(), anyhow::Error> {
    mqtt.publish(
        format!("{}{}", prefix, addr),
        QoS::AtLeastOnce,
        false,
        serde_json::to_vec_pretty(&json!({
            "rssi": rssi
        }))?
    ).await?;
    Ok(())
}

#[derive(Parser)]
pub struct Opts {
    #[clap(short = 'c', long)]
    mqtt_client_id: String,
    #[clap(short = 'h', long, default_value = "localhost")]
    mqtt_host: String,
    #[clap(short = 'p', long, default_value = "1883")]
    mqtt_port: u16,
    #[clap(short = 'u', long)]
    mqtt_username: String,
    #[clap(short = 'w', long)]
    mqtt_password: String,
    #[clap(long, default_value = "ble-scanner/ble-")]
    mqtt_prefix: String,
    #[clap(short = 'a')]
    adapter: Option<String>,
    #[clap(short = 'b', long = "beacon")]
    beacons: Vec<Address>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), anyhow::Error> {
    let opts = Opts::parse();
    pretty_env_logger::init();

    let mut beacon_addresses = HashSet::new();
    for beacon_address in opts.beacons {
        beacon_addresses.insert(beacon_address);
    }

    let session = bluer::Session::new().await?;
    let adapter_name = match opts.adapter {
        Some(adapter_name) => adapter_name,
        None => {
            let adapter_names = session.adapter_names().await?;
            adapter_names.into_iter().next().expect("No Bluetooth adapter present")
        }
    };

    info!("Discovering devices using Bluetooth adapater {}", &adapter_name);
    let adapter = session.adapter(&adapter_name)?;
    adapter.set_powered(true).await?;

    let device_events = adapter.discover_devices().await?;
    pin_mut!(device_events);
    
    let mut all_change_events = futures::stream::SelectAll::new();

    let mut mqtt_options = MqttOptions::new(opts.mqtt_client_id, opts.mqtt_host, opts.mqtt_port);
    mqtt_options.set_credentials(opts.mqtt_username, opts.mqtt_password);
    mqtt_options.set_keep_alive(Duration::from_secs(5));
    
    let (mut client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    
    loop {
        tokio::select! {
            Some(device_event) = device_events.next() => {
                match device_event {
                    AdapterEvent::DeviceAdded(addr) => {
                        if !beacon_addresses.is_empty() && !beacon_addresses.contains(&addr) {
                            continue;
                        }

                        let device = adapter.device(addr)?;
                        if let Some(rssi) = device.rssi().await? {
                            report_rssi(&mut client, &opts.mqtt_prefix, &addr, Some(rssi)).await?;
                        }

                        let change_events = device.events().await?.map(move |evt| (addr, evt));
                        all_change_events.push(change_events);
                    }
                    AdapterEvent::DeviceRemoved(addr) => {
                        if !beacon_addresses.is_empty() && !beacon_addresses.contains(&addr) {
                            continue;
                        }

                        report_rssi(&mut client, &opts.mqtt_prefix, &addr, None).await?;
                    }
                    _ => (),
                }
            }
            Some((addr, DeviceEvent::PropertyChanged(DeviceProperty::Rssi(rssi)))) = all_change_events.next() => {
                if !beacon_addresses.is_empty() && !beacon_addresses.contains(&addr) {
                    continue;
                }
        
                report_rssi(&mut client, &opts.mqtt_prefix, &addr, Some(rssi)).await?
            }
            e = eventloop.poll() => {
                match e? {
                    rumqttc::Event::Incoming(e) => {
                        match e {
                            rumqttc::Packet::Connect(_) => info!("connected to MQTT"),
                            rumqttc::Packet::Disconnect => warn!("disconnected from MQTT"),
                            _ => {},
                        }
                    },
                    _ => {},
                }
            }
            else => break
        }
    }
    
    Ok(())
}

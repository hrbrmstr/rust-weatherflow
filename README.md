#  weatherflow

A basic command line UDP listener for WeatherFlow Tempest in Rust.

## Install

macOS folks can do:

```bash
brew install hrbrmstr/tap/weatherflow
```

Binaries for other operating systems are in the Releases.

The hard way:

```bash
$ cargo install --git https://github.com/hrbrmstr/rust-weatherflow --branch batman # install it (~/.cargo/bin/weatherflow)
```

## Usage

```bash
$ weatherflow
{"serial_number":"HB-00069665","type":"hub_status","firmware_revision":"177","uptime":9637534,"rssi":-58,"timestamp":1665138632,"reset_flags":"BOR,PIN,POR","seq":962685,"radio_stats":[25,1,0,3,16637],"mqtt_stats":[128,177]}
{"serial_number":"ST-00055227","type":"rapid_wind","hub_sn":"HB-00069665","ob":[1665138633,0.00,0]}
{"serial_number":"ST-00055227","type":"rapid_wind","hub_sn":"HB-00069665","ob":[1665138636,0.00,0]}
{"serial_number":"ST-00055227","type":"rapid_wind","hub_sn":"HB-00069665","ob":[1665138639,0.00,0]}
{"serial_number":"HB-00069665","type":"hub_status","firmware_revision":"177","uptime":9637544,"rssi":-58,"timestamp":1665138642,"reset_flags":"BOR,PIN,POR","seq":962686,"radio_stats":[25,1,0,3,16637],"mqtt_stats":[128,177]}
{"serial_number":"ST-00055227","type":"rapid_wind","hub_sn":"HB-00069665","ob":[1665138642,0.00,0]}
{"serial_number":"ST-00055227","type":"rapid_wind","hub_sn":"HB-00069665","ob":[1665138645,0.00,0]}
{"serial_number":"ST-00055227","type":"obs_st","hub_sn":"HB-00069665","obs":[[1665138647,0.00,0.00,0.00,0,3,1006.96,8.34,97.75,31,0.00,0,0.000000,0,0,0,2.646,1]],"firmware_revision":165}
```
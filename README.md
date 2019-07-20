# Audio Visualizer

This is an audio visualizer I made for educational purposes.
Primarily to get to know Rust better, but also to learn about
signal processing.

## Requirements
* Rust
* PulseAudio
* SDL2

## Building
Simply run `cargo build --release`

## Usage
```
# audio_visualizer --help
Audio Visualizer 0.1
Patric Kanngießer <mail@lpnw.de>
Simple audio visualizer for educational purposes

USAGE:
    audio_visualizer [source]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <source>    PulseAudio source the visualizer should listen to
```

If no source is provided, the default PulseAudio source will be used.
To get a list of all sources for your system run `pactl list sources`
The output will look like this:
```
Quelle #3
        Status: IDLE
        Name: alsa_output.firewire-0x00130e0402405156.multichannel-output.monitor
        Beschreibung: Monitor of SAFFIRE_PRO_14 Mehrkanal
        Treiber: module-alsa-card.c
        .....
``` 

Run the visualizer: 
`audio_visualizer alsa_output.firewire-0x00130e0402405156.multichannel-output.monitor`

## Screenshot
![Screenshot](http://hothead.lpnw.de/visualizer.png)


# Project in Embedded System

Albin Martinsson \
Gustav Hansson

## Project Idea

Parking sensor using an ultra sound sensor for measuring distance.

## Dependencies

- Rust 1.40, or later.

``` console
        > rustup update
        > rustup target add thumbv7em-none-eabihf
```

- For programming (flashing) and debugging
  - `openocd` debug host, (install using your package manager).

    For `Ubuntu` and `OSX`: use `source [find interface/stlink-v2-1.cfg]` in the `openocd.cfg` and the `.vscode/launch.json` if VSCode is used.

    For other operating systems use `source [find interface/stlink.cfg]`

  - `arm-none-eabi` tool-chain (install using your package manager).

- Configure your Nucleo programmer, [Datasheet with how to configure](https://www.st.com/content/ccc/resource/technical/document/user_manual/98/2e/fa/4b/e0/82/43/b7/DM00105823.pdf/files/DM00105823.pdf/jcr:content/translations/en.DM00105823.pdf)

## Run

- Connect the SWD pins from the Nucleo to the respective pins on the card
- run `openocd -f openocd.cfg` from the project folder, the same that this file is in.
- run `cargo run`.

Tried on Ubuntu 19 and Arch

## Assumptions (brasklapp)

We could not get the distance sensor to work so we did the following work around.

We used the value that the potentiometer returns to act as the distance to the object, the higher the value the nearer the object is and vice versa. 
Based on the limited number of timers, 4, the MCU supports we defined three intervals. We could only use three timers to define the frequency because on timer has to be used to turn the piezo element on and off. Far away, near and crash. Each of the intervals correspends to a frequncy the buzzer beeps and also what the OLED screen prints out. 


## Specs

### Purpose

Enables a safer way to park your car.

### Limitations

Using only one cheap distance sensor the distance and area covered by the sensor is suboptimal. Smaller obstacles coming in at an angle could be missed by the sensor and thus the driver should always look where he drives. It should be used as an aid, not a guide.

### Behaviour

The distance is measured with a ultra sound and a speaker that warns the driver of an approaching obstacle. The parking sensor can be turned on and off by a button and the distance is broadcasted to a user via the radio.

### Safety

This system should not be blindly relied on and the driver should always look where he drives. The producer takes no responsibility for damage to objects or persons.

### Liveness

The system should measure distance in real time and based on a threshold make a warning sound. This sound can not be delayed more than 200ms.

### Robustness

To prevent malformed packages we poll data from the sensor during a short interval on around 100 ms. During this interval we can check to see that the measured data is reliable by comparing it to the rest of the measured data during that interval. Thus we can know if a package is malformed or if a distance is measured wrongly.

### Security

The system uses very non personal data thus encryption is not needed.

## Components

- Ultrasonic distance sensor
- OLED screen
- Speaker
- LED
- Button
- Antenna

## Ultrasonic distance sensor

Here we will provide some links to potential components to order, each component will have a short description of specs.

### [4019 - US-100](https://www.elfa.se/en/us-100-ultrasonic-distance-sensor-5v-adafruit-4019/p/30139213?q=Ultrasonic+distance+sensor&pos=4&origPos=4&origPageSize=10&track=true)

- Distance range: 0.02...4.5m
- 15 degree angle
- 67,10 SEK
- 3-5V DC
- 2mA (Would depend on the voltage suplied..?)

## [OLED screen](https://cdon.se/hem-tradgard/oled-display-0-96-tum-vit-128x64-pixlar-ssd1306-spi-p50506639)

## [Speaker](https://www.elfa.se/en/electromechanical-buzzer-70db-3khz-4v-pcb-pins-rnd-components-rnd-430-00022/p/30160669?q=*&pos=3&origPos=10&origPageSize=10&track=true)

## Headers

### [2 pin male](https://se.rs-online.com/web/p/pcb-headers/2518086/)

### [5 pin female](https://www.elfa.se/en/straight-female-pcb-receptacle-through-hole-rows-contacts-54mm-pitch-rnd-connect-rnd-205-00645/p/30093665?q=pcb+headers&pos=1&origPos=388&origPageSize=10&track=true)

### [6 pin male](https://www.elfa.se/en/straight-male-pcb-header-through-hole-rows-contacts-54mm-pitch-rnd-connect-rnd-205-00627/p/30093647?q=pcb+headers&pos=9&origPos=868&origPageSize=10&track=true)

### [7 pin female](https://www.elfa.se/en/straight-female-pcb-receptacle-through-hole-rows-contacts-54mm-pitch-rnd-connect-rnd-205-00647/p/30093667?q=pcb+headers&pos=2&origPos=397&origPageSize=10&track=true)

## [USB-connector](https://se.rs-online.com/web/p/micro-usb-connectors/1225099/)

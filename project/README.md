# Project in Embedded System

Albin Martinsson \
Gustav Hansson

## Project Idea

Parking sensor using an ultra sound sensor for measuring distance.

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


# Components

## Ultrasonic distance sensor

Here we will provide some links to potential components to order, each component will have a short description of specs.

### [DEBO SEN ULTRA](https://www.elfa.se/en/ultrasonic-distance-sensor-raspberry-pi-debo-sen-ultra/p/30036820?q=Ultrasonic+distance+sensor&pos=1&origPos=1&origPageSize=10&track=true)

* Distance range: 0.3...4m
* 30 degree angle
* 62,10 SEK
* 5V DC 
* 15mA 


### [SEN-15569 - HC-SR04](https://www.elfa.se/en/hc-sr04-ultrasonic-distance-sensor-sparkfun-electronics-sen-15569/p/30160395?q=Ultrasonic+distance+sensor&pos=2&origPos=2&origPageSize=10&track=true)

* Distance range: 0.02...4m (Might be wrong unit, 0.2 seems more reasonable...)
* 15 degree angle
* 35,10 SEK
* 5V DC 
* 15mA 

### [SEN-13959 - HC-SR04](https://www.elfa.se/en/hc-sr04-ultrasonic-distance-sensor-sparkfun-electronics-sen-13959/p/30145510?q=Ultrasonic+distance+sensor&pos=3&origPos=3&origPageSize=10&track=true)

* Distance range: 0.2...4m 
* 15 degree angle
* 37,60 SEK
* 5V DC 
* 15mA 

### [4019 - US-100](https://www.elfa.se/en/us-100-ultrasonic-distance-sensor-5v-adafruit-4019/p/30139213?q=Ultrasonic+distance+sensor&pos=4&origPos=4&origPageSize=10&track=true)

* Distance range: 0.02...4.5m
* 15 degree angle
* 67,10 SEK
* 3-5V DC 
* 2mA (Would depend on the voltage suplied..?) 



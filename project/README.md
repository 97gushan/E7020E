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

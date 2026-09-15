The other half of my project [here](https://github.com/Ike-l/arduino_uno.git)

building:
* cargo +nightly build --release

then put the .a file: ```target\avr-none\release\libsensor_state.a``` into the libaries folder in the arduino project

for example: ```libraries\SensorState\src\atmega328p\libsensor_state.a```
#include <U8g2lib.h>
#include <Wire.h>
#include <SensorState.h>
#include <DHT.h>
#include <Seeed_BMP280.h>
#include <LIS3DHTR.h>

const int rotary_pin = A0;
const int sound_pin  = A2;
const int light_pin  = A6;
const int dht_pin    = 3;
const int led_pin    = 4;
const int buzzer_pin = 5;
const int button_pin = 6;

// Uses the page buffer (_1_) for the Grove kit's SSD1306 OLED
// U8G2_SSD1306_128X64_NONAME_1_HW_I2C oled(U8G2_R2, /* reset=*/ U8X8_PIN_NONE);
U8X8_SSD1306_128X64_NONAME_HW_I2C oled(/* reset=*/ U8X8_PIN_NONE);

// const u8g2_uint_t OLED_width = oled.getDisplayWidth();
// const u8g2_uint_t OLED_height = oled.getDisplayHeight();

// const int max_OLED_x = OLED_width - 1;
// const int max_OLED_y = OLED_height - 1;

const uint8_t OLED_width = oled.getCols();
const uint8_t OLED_height = oled.getRows();

const int max_OLED_x = OLED_width - 1;
const int max_OLED_y = OLED_height - 1;

const int max_rotary = 1023;

unsigned long last_render_time = 0;
const unsigned long render_interval = 500;

#define DHTTYPE DHT11
DHT dht(dht_pin, DHTTYPE);

BMP280 bmp280;
LIS3DHTR<TwoWire> accel;

SensorData sensor_data;
EnvironmentData environment_data;
DisplaySettings display_settings;

int16_t draw_text_callback(int16_t x, int16_t y, const char* text) {
  // offsets from how text is rendered from the bottom of the text
  // int16_t baseline_y = y + oled.getAscent();

  // oled.setCursor(x, baseline_y);
  oled.setCursor(x, y);
  oled.print(text);

  // return x + oled.getStrWidth(text);
  return x + strlen(text);
}

int8_t set_font_callback(uint8_t font_type) {
  // if (font_type == 0) {
  //   oled.setFont(u8g2_font_6x10_tr); // 4%
  // } else if (font_type == 1) {
  //   oled.setFont(u8g2_font_8x13B_tr); // 3%
  // }

  oled.setFont(u8x8_font_chroma48medium8_r);
  
  // display_settings.font_height = oled.getAscent() - oled.getDescent();
  display_settings.font_height = 1;

  return display_settings.font_height;
}

void panic_blink() {
  pinMode(led_pin, OUTPUT);

  while (true) {
    digitalWrite(led_pin, HIGH);
    delay(100);
    digitalWrite(led_pin, LOW);
    delay(100);
  }
}

void setup() {
  pinMode(led_pin, OUTPUT);
  pinMode(buzzer_pin, OUTPUT);
  pinMode(button_pin, INPUT);

  dht.begin();

  accel.begin(Wire, 0x19);
  accel.setOutputDataRate(LIS3DHTR_DATARATE_50HZ);

  oled.begin();
  oled.setFlipMode(1);
  // oled.firstPage();

  // oled.setFont(u8g2_font_ncenB08_tr);
  oled.setFont(u8x8_font_chroma48medium8_r);
  // display_settings.font_height = oled.getAscent() - oled.getDescent();

  // display_settings.screen_height = OLED_height;
  // display_settings.screen_width = OLED_width;

  display_settings.font_height = 1;

  display_settings.screen_height = OLED_height;
  display_settings.screen_width = OLED_width;

  delay(2000);
}

void loop() {
  environment_data.current_time = millis();

  sensor_data.button = digitalRead(button_pin) == HIGH;
  sensor_data.rotary = analogRead(rotary_pin);

  tick(&sensor_data, &environment_data);

  if (environment_data.current_time - last_render_time >= render_interval) {
    last_render_time = environment_data.current_time;

    sensor_data.sound = analogRead(sound_pin);
    sensor_data.light = analogRead(light_pin);

    sensor_data.humidity = dht.readHumidity();

    sensor_data.temperature = dht.readTemperature(); // Celsius
    sensor_data.pressure = bmp280.getPressure(); // Pascals // 15% max program storage space
    sensor_data.acceleration[0] = accel.getAccelerationX();
    sensor_data.acceleration[1] = accel.getAccelerationY();
    sensor_data.acceleration[2] = accel.getAccelerationZ();

    oled.clearDisplay();
    // oled.firstPage();
    // do {
    render(&display_settings, &environment_data, draw_text_callback, set_font_callback);
    // } while (oled.nextPage());
  }
}

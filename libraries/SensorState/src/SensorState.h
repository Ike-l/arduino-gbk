#ifndef SENSOR_STATE_H
#define SENSOR_STATE_H
#include <Arduino.h>

extern "C" {
    struct SensorData {
        bool button;

        int16_t rotary;
        int16_t sound;
        int16_t light;

        int16_t temperature;
        float humidity;
        float pressure;

        float acceleration[3];
    };

    struct EnvironmentData {
        uint32_t current_time;
    };

    struct DisplaySettings {
        int8_t font_height;
        uint8_t screen_width;
        uint8_t screen_height;
    };
    
    typedef int16_t (*DrawTextCallback)(int16_t x, int16_t y, const char* text);
    typedef int8_t (*SetFontCallback)(uint8_t font_type);
    
    void render(
        const DisplaySettings* display_settings,
        const EnvironmentData* environment_data,
        DrawTextCallback draw_text_cb,
        SetFontCallback set_font_cb
    );

    void tick(
        const SensorData* sensor_data,
        const EnvironmentData* environment_data
    );

    extern "C" {
        void panic_blink();
    }
}

#endif
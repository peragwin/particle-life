#pragma once
#include <stdint.h>

struct ColorRGB
{
    uint8_t r;
    uint8_t g;
    uint8_t b;

    ColorRGB() {}
    ColorRGB(uint8_t r, uint8_t g, uint8_t b) : r(r), g(g), b(b) {}
};

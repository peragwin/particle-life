#pragma once

#include "Color.h"
#include "HSV.h"
#include <vector>
#include <cstddef>

struct Particle
{
  float x, y;
  float vx, vy;
  uint8_t type;
};

class ParticleTypes
{
public:
  void Resize(size_t types, size_t particles)
  {
    m_col.resize(types);
    for (int i = 0; i < m_col.size(); ++i)
    {
      m_col[i] = FromHSV(float(i) / m_col.size(), 1.0f, 0.5); // float(i % 2) * 0.5f + 0.5f));
    }
    m_attract.resize(particles * particles);
    m_minR.resize(particles * particles);
    m_maxR.resize(particles * particles);
  }

  size_t Size() const { return m_col.size(); }
  const ColorRGB Color(size_t i) const { return m_col[i]; }
  void SetColor(size_t i, ColorRGB c) { m_col[i] = c; }
  float Attract(size_t i, size_t j) const { return m_attract[i * m_col.size() + j]; }
  void SetAttract(size_t i, size_t j, float a) { m_attract[i * m_col.size() + j] = a; }
  float MinR(size_t i, size_t j) const { return m_minR[i * m_col.size() + j]; }
  void SetMinR(size_t i, size_t j, float m) { m_minR[i * m_col.size() + j] = m; }
  float MaxR(size_t i, size_t j) const { return m_maxR[i * m_col.size() + j]; }
  void SetMaxR(size_t i, size_t j, float m) { m_maxR[i * m_col.size() + j] = m; }

private:
  std::vector<ColorRGB> m_col;
  std::vector<float> m_attract;
  std::vector<float> m_minR;
  std::vector<float> m_maxR;
};

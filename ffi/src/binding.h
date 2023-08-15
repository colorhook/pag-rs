#pragma once

#include <pag/pag.h>
#include "lib.rs.h"

namespace pag
{

  // 重新定义 Rect 类的 POD 类型
  struct PAG_API PAGRect
  {
    float left;
    float top;
    float right;
    float bottom;
  };

  inline std::vector<int> make_cxx_int_vector()
  {
    return std::vector<int>();
  }
  inline std::vector<std::string> make_cxx_string_vector()
  {
    return std::vector<std::string>();
  }
  inline void cxx_int_vector_push(std::vector<int> &vector, int element)
  {
    vector.push_back(element);
  }
  inline void cxx_string_vector_push(std::vector<std::string> &vector, std::string element)
  {
    vector.push_back(std::move(element));
  }
  inline std::vector<PAGMarker> cxx_PAGLayer_getMarkers(PAGLayer &layer)
  {
    std::vector<const Marker *> markers = layer.markers();
    std::vector<PAGMarker> pag_makers = std::vector<PAGMarker>();
    for (auto &marker : markers)
    {
      pag_makers.push_back(PAGMarker{marker->startTime, marker->duration, marker->comment});
    }
    return pag_makers;
  }
  inline std::vector<PAGMarker> cxx_PAGComposition_getAudioMarkers(PAGComposition &layer)
  {
    std::vector<const Marker *> markers = layer.audioMarkers();
    std::vector<PAGMarker> pag_makers = std::vector<PAGMarker>();
    for (auto &marker : markers)
    {
      pag_makers.push_back(PAGMarker{marker->startTime, marker->duration, marker->comment});
    }
    return pag_makers;
  }
  inline std::vector<int32_t> cxx_PAGComposition_getLayersByName(PAGComposition &layer, const std::string &layerName)
  {
    std::vector<std::shared_ptr<PAGLayer>> layers = layer.getLayersByName(layerName);
    std::vector<int32_t> out = std::vector<int32_t>();
    for (auto &item : layers)
    {
      out.push_back(layer.getLayerIndex(item));
    }
    return out;
  }
  inline std::vector<int32_t> cxx_PAGComposition_getLayersUnderPoint(PAGComposition &layer, float localX, float localY)
  {
    std::vector<std::shared_ptr<PAGLayer>> layers = layer.getLayersUnderPoint(localX, localY);
    std::vector<int32_t> out = std::vector<int32_t>();
    for (auto &item : layers)
    {
      out.push_back(layer.getLayerIndex(item));
    }
    return out;
  }
  inline PAGRect cxx_PAGLayer_getBounds(PAGLayer &layer)
  {
    Rect rect = layer.getBounds();
    return PAGRect{rect.left, rect.top, rect.right, rect.bottom};
  }
  inline PAGRect cxx_PAGPlayer_getBounds(PAGPlayer &player, std::shared_ptr<PAGLayer> pagLayer)
  {
    Rect rect = player.getBounds(pagLayer);
    return PAGRect{rect.left, rect.top, rect.right, rect.bottom};
  }

  inline std::vector<int32_t> cxx_PAGFile_getEditableIndices(PAGFile &file, LayerType layerType)
  {
    std::vector<int> in = file.getEditableIndices(layerType);
    std::vector<int32_t> out(in.begin(), in.end());
    return out;
  }

  // PAGFont
  inline std::string cxx_PAGFont_get_fontFamily(const PAGFont &font)
  {
    return font.fontFamily;
  }
  inline std::string cxx_PAGFont_get_fontStyle(const PAGFont &font)
  {
    return font.fontStyle;
  }

  // TextDocument getter/setter 方法
  inline bool cxx_TextDocument_get_applyFill(TextDocument &document)
  {
    return document.applyFill;
  }
  inline void cxx_TextDocument_set_applyFill(TextDocument &document, bool value)
  {
    document.applyFill = value;
  }
  inline bool cxx_TextDocument_get_applyStroke(TextDocument &document)
  {
    return document.applyFill;
  }
  inline void cxx_TextDocument_set_applyStroke(TextDocument &document, bool value)
  {
    document.applyFill = value;
  }
  // @readonly
  inline float cxx_TextDocument_get_baselineShift(TextDocument &document)
  {
    return document.baselineShift;
  }
  // @readonly
  inline bool cxx_TextDocument_get_boxText(TextDocument &document)
  {
    return document.boxText;
  }
  // @readonly
  inline float cxx_TextDocument_get_firstBaseLine(TextDocument &document)
  {
    return document.firstBaseLine;
  }
  inline bool cxx_TextDocument_get_fauxBold(TextDocument &document)
  {
    return document.fauxBold;
  }
  inline void cxx_TextDocument_set_fauxBold(TextDocument &document, bool value)
  {
    document.fauxBold = value;
  }
  inline bool cxx_TextDocument_get_fauxItalic(TextDocument &document)
  {
    return document.fauxBold;
  }
  inline void cxx_TextDocument_set_fauxItalic(TextDocument &document, bool value)
  {
    document.fauxItalic = value;
  }
  inline float cxx_TextDocument_get_fontSize(TextDocument &document)
  {
    return document.fontSize;
  }
  inline void cxx_TextDocument_set_fontSize(TextDocument &document, float value)
  {
    document.fontSize = value;
  }
  inline Color cxx_TextDocument_get_fillColor(TextDocument &document)
  {
    return document.fillColor;
  }
  inline void cxx_TextDocument_set_fillColor(TextDocument &document, const Color &value)
  {
    document.fillColor = value;
  }
  inline Color cxx_TextDocument_get_strokeColor(TextDocument &document)
  {
    return document.strokeColor;
  }
  inline void cxx_TextDocument_set_strokeColor(TextDocument &document, const Color &value)
  {
    document.strokeColor = value;
  }
  inline std::string cxx_TextDocument_get_fontFamily(TextDocument &document)
  {
    return document.fontFamily;
  }
  inline void cxx_TextDocument_set_fontFamily(TextDocument &document, std::string value)
  {
    document.fontFamily = std::move(value);
  }
  inline std::string cxx_TextDocument_get_fontStyle(TextDocument &document)
  {
    return document.fontStyle;
  }
  inline void cxx_TextDocument_set_fontStyle(TextDocument &document, std::string value)
  {
    document.fontStyle = std::move(value);
  }

  // @readonly
  inline bool cxx_TextDocument_get_strokeOverFill(TextDocument &document)
  {
    return document.strokeOverFill;
  }
  inline float cxx_TextDocument_get_strokeWidth(TextDocument &document)
  {
    return document.strokeWidth;
  }
  inline void cxx_TextDocument_set_strokeWidth(TextDocument &document, float value)
  {
    document.strokeWidth = value;
  }
  inline std::string cxx_TextDocument_get_text(TextDocument &document)
  {
    return document.text;
  }
  inline void cxx_TextDocument_set_text(TextDocument &document, std::string value)
  {
    document.text = std::move(value);
  }
  inline Enum cxx_TextDocument_get_justification(TextDocument &document)
  {
    return document.justification;
  }
  inline void cxx_TextDocument_set_justification(TextDocument &document, Enum value)
  {
    document.justification = value;
  }
  inline float cxx_TextDocument_get_leading(TextDocument &document)
  {
    return document.leading;
  }
  inline void cxx_TextDocument_set_leading(TextDocument &document, float value)
  {
    document.leading = value;
  }
  inline float cxx_TextDocument_get_tracking(TextDocument &document)
  {
    return document.tracking;
  }
  inline void cxx_TextDocument_set_tracking(TextDocument &document, float value)
  {
    document.tracking = value;
  }
  inline Color cxx_TextDocument_get_backgroundColor(TextDocument &document)
  {
    return document.backgroundColor;
  }
  inline void cxx_TextDocument_set_backgroundColor(TextDocument &document, const Color &value)
  {
    document.backgroundColor = value;
  }
  inline uint8_t cxx_TextDocument_get_backgroundAlpha(TextDocument &document)
  {
    return document.backgroundAlpha;
  }
  inline void cxx_TextDocument_set_backgroundAlpha(TextDocument &document, uint8_t value)
  {
    document.backgroundAlpha = value;
  }
  inline Enum cxx_TextDocument_get_direction(TextDocument &document)
  {
    return document.direction;
  }
  inline void cxx_TextDocument_set_direction(TextDocument &document, Enum value)
  {
    document.direction = value;
  }
} // namespace pag
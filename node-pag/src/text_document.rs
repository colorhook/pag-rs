use libpag::*;
use neon::prelude::*;

pub struct JsTextDocument {
    text: String,
    font_size: f32,
    bold: bool,
    italic: bool,
    font_family: String,
    font_style: String,
    stroke_width: f32,
    stroke_color: PAGColor,
    fill_color: PAGColor,
    apply_fill: bool,
    apply_stroke: bool,
    justification: ParagraphJustification,
    leading: f32,
    tracking: f32,
    background_color: PAGColor,
    background_alpha: u8,
    direction: TextDirection,
}

impl JsTextDocument {
    pub fn from_text_document(doc: &TextDocument) -> Self {
        JsTextDocument {
            text: doc.text(),
            font_size: doc.font_size(),
            bold: doc.faux_bold(),
            italic: doc.faux_italic(),
            font_family: doc.font_family(),
            font_style: doc.font_style(),
            stroke_width: doc.stroke_width(),
            stroke_color: doc.stroke_color(),
            fill_color: doc.fill_color(),
            apply_fill: doc.apply_fill(),
            apply_stroke: doc.apply_stroke(),
            justification: doc.justification(),
            leading: doc.leading(),
            tracking: doc.tracking(),
            background_color: doc.background_color(),
            background_alpha: doc.background_alpha(),
            direction: doc.direction(),
        }
    }

    pub fn apply_text_document<'a>(
        cx: &mut FunctionContext<'a>,
        obj: &Handle<JsObject>,
        doc: &TextDocument,
    ) {

        let prop = obj.get_opt::<JsString, _, _>(cx, "text");
        if let Ok(Some(v)) = prop {
            doc.set_text(v.value(cx));
        }
        let prop = obj.get_opt::<JsNumber, _, _>(cx, "font_size");
        if let Ok(Some(v)) = prop {
            doc.set_font_size(v.value(cx) as f32);
        }
        let prop = obj.get_opt::<JsBoolean, _, _>(cx, "bold");
        if let Ok(Some(v)) = prop {
            doc.set_faux_bold(v.value(cx));
        }
        let prop = obj.get_opt::<JsBoolean, _, _>(cx, "italic");
        if let Ok(Some(v)) = prop {
            doc.set_faux_italic(v.value(cx));
        }
        let prop = obj.get_opt::<JsString, _, _>(cx, "font_family");
        if let Ok(Some(v)) = prop {
            doc.set_font_family(v.value(cx));
        }
        let prop = obj.get_opt::<JsString, _, _>(cx, "font_style");
        if let Ok(Some(v)) = prop {
            doc.set_font_style(v.value(cx));
        }
        let prop = obj.get_opt::<JsNumber, _, _>(cx, "stroke_width");
        if let Ok(Some(v)) = prop {
            doc.set_stroke_width(v.value(cx) as f32);
        }

        let prop = obj.get_opt::<JsString, _, _>(cx, "stroke_color");
        if let Ok(Some(v)) = prop {
            if let Some(color) = PAGColor::from_hex(&v.value(cx)) {
                doc.set_stroke_color(&color);
            }
        }
        let prop = obj.get_opt::<JsString, _, _>(cx, "fill_color");
        if let Ok(Some(v)) = prop {
            if let Some(color) = PAGColor::from_hex(&v.value(cx)) {
                doc.set_fill_color(&color);
            }
        }

        let prop = obj.get_opt::<JsBoolean, _, _>(cx, "apply_fill");
        if let Ok(Some(v)) = prop {
            doc.set_apply_fill(v.value(cx));
        }
        let prop = obj.get_opt::<JsBoolean, _, _>(cx, "apply_stroke");
        if let Ok(Some(v)) = prop {
            doc.set_apply_stroke(v.value(cx));
        }
   
        let prop = obj.get_opt::<JsNumber, _, _>(cx, "justification");
        if let Ok(Some(v)) = prop {
            doc.set_justification(ParagraphJustification::from(v.value(cx) as u8));
        }
        let prop = obj.get_opt::<JsNumber, _, _>(cx, "leading");
        if let Ok(Some(v)) = prop {
            doc.set_leading(v.value(cx) as f32);
        }
        let prop = obj.get_opt::<JsNumber, _, _>(cx, "tracking");
        if let Ok(Some(v)) = prop {
            doc.set_tracking(v.value(cx) as f32);
        }
 
        let prop = obj.get_opt::<JsString, _, _>(cx, "background_color");
        if let Ok(Some(v)) = prop {
            if let Some(color) = PAGColor::from_hex(&v.value(cx)) {
                doc.set_background_color(&color);
            }
        }

        let prop = obj.get_opt::<JsNumber, _, _>(cx, "background_alpha");
        if let Ok(Some(v)) = prop {
            doc.set_background_alpha(v.value(cx) as u8);
        }
        let prop = obj.get_opt::<JsNumber, _, _>(cx, "direction");
        if let Ok(Some(v)) = prop {
            doc.set_direction(TextDirection::from(v.value(cx) as u8));
        }
    }
    pub fn to_js_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let text = cx.string(&self.text);
        obj.set(cx, "text", text)?;

        let font_size = cx.number(self.font_size);
        obj.set(cx, "font_size", font_size)?;

        let bold = cx.boolean(self.bold);
        obj.set(cx, "bold", bold)?;

        let italic = cx.boolean(self.italic);
        obj.set(cx, "italic", italic)?;

        let font_family = cx.string(&self.font_family);
        obj.set(cx, "font_family", font_family)?;

        let font_style = cx.string(&self.font_style);
        obj.set(cx, "font_style", font_style)?;

        let stroke_width = cx.number(self.stroke_width);
        obj.set(cx, "stroke_width", stroke_width)?;

        let stroke_color = cx.string(&self.stroke_color.to_hex());
        obj.set(cx, "stroke_color", stroke_color)?;

        let fill_color = cx.string(&self.fill_color.to_hex());
        obj.set(cx, "fill_color", fill_color)?;

        let apply_fill = cx.boolean(self.apply_fill);
        obj.set(cx, "apply_fill", apply_fill)?;

        let apply_stroke = cx.boolean(self.apply_stroke);
        obj.set(cx, "apply_stroke", apply_stroke)?;

        let justification = cx.number(self.justification as u8);
        obj.set(cx, "justification", justification)?;

        let leading = cx.number(self.leading);
        obj.set(cx, "leading", leading)?;

        let tracking = cx.number(self.tracking);
        obj.set(cx, "tracking", tracking)?;

        let background_color = cx.string(&self.background_color.to_hex());
        obj.set(cx, "background_color", background_color)?;

        let background_alpha = cx.number(self.background_alpha);
        obj.set(cx, "background_alpha", background_alpha)?;

        let direction = cx.number(self.direction as u8);
        obj.set(cx, "direction", direction)?;

        Ok(obj)
    }
}

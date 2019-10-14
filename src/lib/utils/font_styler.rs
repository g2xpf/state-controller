use crate::utils::Font;
use glium::glutin::dpi::LogicalSize;
use glium::texture::{ClientFormat, MipmapsOption, RawImage2d, Texture2d, UncompressedFloatFormat};
use glium::uniforms::{MagnifySamplerFilter, Sampler};
use glium::Display;
use rusttype::gpu_cache::Cache;
use rusttype::{point, PositionedGlyph, Scale};
use std::borrow::Cow;

pub struct RectCoord {
    pub left_bottom: [f32; 2],
    pub right_bottom: [f32; 2],
    pub left_top: [f32; 2],
    pub right_top: [f32; 2],
}

pub struct FontStyler<'a> {
    display: Display,
    pub cache: Cache<'a>,
    pub glyphs: Vec<PositionedGlyph<'a>>,
    font: Font<'a>,
    cache_tex: Texture2d,
    pub text: String,
    wrap_bound: u32,
    font_size: f32,
}

impl<'a> FontStyler<'a> {
    pub fn new(display: &Display, font: Font<'a>, logical_size: LogicalSize) -> Self {
        let dpi_factor = display.gl_window().window().get_hidpi_factor() as f64;
        let (inner_width, _) = display
            .gl_window()
            .window()
            .get_inner_size()
            .unwrap()
            .to_physical(dpi_factor)
            .into();
        let (cache_width, cache_height) = (
            (logical_size.width * dpi_factor) as u32,
            (logical_size.height * dpi_factor) as u32,
        );

        FontStyler {
            display: display.clone(),
            cache: Cache::builder()
                .dimensions(cache_width, cache_height)
                .build(),
            cache_tex: Texture2d::with_format(
                display,
                RawImage2d {
                    data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
                    width: cache_width,
                    height: cache_height,
                    format: ClientFormat::U8,
                },
                UncompressedFloatFormat::U8,
                MipmapsOption::NoMipmap,
            )
            .unwrap(),
            font,
            glyphs: Vec::new(),
            text: String::from(""),
            wrap_bound: inner_width,
            font_size: 24.0,
        }
    }

    pub fn set_wrap_bound(&mut self, bound: u32) {
        let dpi_factor = self.display.gl_window().window().get_hidpi_factor();
        self.wrap_bound = bound * dpi_factor as u32;
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
    }

    pub fn layout_paragraph(&mut self) {
        use unicode_normalization::UnicodeNormalization;
        let dpi_factor = self.display.gl_window().window().get_hidpi_factor();
        let scale = Scale::uniform(self.font_size * dpi_factor as f32);
        let mut glyphs = Vec::new();
        let v_metrics = self.font.v_metrics(scale);
        let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
        let mut caret = point(0.0, v_metrics.ascent);
        let mut last_glyph_id = None;
        for c in self.text.nfc() {
            if c.is_control() {
                match c {
                    '\r' => {
                        caret = point(0.0, caret.y + advance_height);
                    }
                    '\n' => {}
                    _ => {}
                }
                continue;
            }
            let base_glyph = self.font.glyph(c);
            if let Some(id) = last_glyph_id.take() {
                caret.x += self.font.pair_kerning(scale, id, base_glyph.id());
            }
            last_glyph_id = Some(base_glyph.id());
            let mut glyph = base_glyph.scaled(scale).positioned(caret);
            if let Some(bb) = glyph.pixel_bounding_box() {
                if bb.max.x > self.wrap_bound as i32 {
                    caret = point(0.0, caret.y + advance_height);
                    glyph.set_position(caret);
                    last_glyph_id = None;
                }
            }
            caret.x += glyph.unpositioned().h_metrics().advance_width;
            glyphs.push(glyph);
        }
        glyphs
            .iter()
            .for_each(|glyph| self.cache.queue_glyph(0, glyph.clone()));

        // separate ownership
        let cache = &mut self.cache;
        let cache_tex = &mut self.cache_tex;
        cache
            .cache_queued(|rect, data| {
                cache_tex.main_level().write(
                    glium::Rect {
                        left: rect.min.x,
                        bottom: rect.min.y,
                        width: rect.width(),
                        height: rect.height(),
                    },
                    glium::texture::RawImage2d {
                        data: Cow::Borrowed(data),
                        width: rect.width(),
                        height: rect.height(),
                        format: glium::texture::ClientFormat::U8,
                    },
                )
            })
            .unwrap();
        self.glyphs = glyphs;
    }

    pub fn get_texture(&self, sampler_filter: MagnifySamplerFilter) -> Sampler<Texture2d> {
        self.cache_tex.sampled().magnify_filter(sampler_filter)
    }

    pub fn get_glyph_info<'b>(
        &self,
        glyph: &PositionedGlyph<'b>,
    ) -> Option<(RectCoord, RectCoord)> {
        let (uv_rect, screen_rect) = self.cache.rect_for(0, glyph).ok()??;

        let uv_rect = RectCoord {
            left_bottom: [uv_rect.min.x, uv_rect.min.y],
            right_bottom: [uv_rect.max.x, uv_rect.min.y],
            left_top: [uv_rect.min.x, uv_rect.max.y],
            right_top: [uv_rect.max.x, uv_rect.max.y],
        };

        let gl_rect = {
            let (screen_width, screen_height) = {
                let (w, h) = self.display.get_framebuffer_dimensions();
                (w as f32, h as f32)
            };
            let left = (screen_rect.min.x as f32 / screen_width - 0.5) * 2.0;
            let right = (screen_rect.max.x as f32 / screen_width - 0.5) * 2.0;
            let top = (1.0 - screen_rect.max.y as f32 / screen_height - 0.5) * 2.0;
            let bottom = (1.0 - screen_rect.min.y as f32 / screen_height - 0.5) * 2.0;

            RectCoord {
                left_bottom: [left, bottom],
                right_bottom: [right, bottom],
                left_top: [left, top],
                right_top: [right, top],
            }
        };

        Some((uv_rect, gl_rect))
    }
}

#[macro_export]
macro_rules! impl_shape_container {
    ($Shape: ty, $($id: ident),*) => {
        use $crate::shapes::ShapeContainer;
        impl ShapeContainer<$Shape> {
            pub fn render<'b>(
                &self,
                frame: &mut glium::Frame,
                draw_parameters: &glium::DrawParameters<'b>,
                ) {
                use glium::Surface;

                for shape in self.shapes.iter() {
                    frame
                        .draw(
                            &self.render_context.vertex_buffer,
                            &self.render_context.index_buffer,
                            &self.render_context.program,
                            &uniform! {
                                $(
                                    $id: shape.$id,
                                    )*
                            },
                            draw_parameters,
                            )
                        .unwrap();
                }
            }
        }
    };
}

#[macro_export]
macro_rules! texture {
    ($display: expr, $image_path: expr, $image_format: expr) => {{
        use glium;
        use image;
        use std::io::Cursor;

        let image = image::load(Cursor::new(&include_bytes!($image_path)[..]), $image_format)
            .unwrap()
            .to_rgba();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        glium::texture::Texture2d::new($display, image).unwrap()
    }};
}

#[macro_export]
macro_rules! impl_shape_container {
    ($Shape: ty; $($id: ident),* $(; $($param:ident: $type:ty),* )?) => {
        use $crate::shapes::ShapeContainer;
        #[allow(non_snake_case)]
        impl ShapeContainer<$Shape> {
            pub fn render<'b>(
                &self,
                frame: &mut glium::Frame,
                draw_parameters: &glium::DrawParameters<'b>
                $($(,$param: $type)*)?
                ) {
                use glium::Surface;

                for shape in self.shapes.iter() {
                    frame
                        .draw(
                            &self.render_context.vertex_buffer,
                            &self.render_context.index_buffer,
                            &self.render_context.program,
                            &uniform! {
                                $($($param: $param,)*)?
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

    ($Shape: ty; |$shape: ident| { $($id: ident: $value: expr),* } $(; $($param:ident: $type:ty),*)?) => {
        use $crate::shapes::ShapeContainer;
        #[allow(non_snake_case)]
        impl ShapeContainer<$Shape> {
            pub fn render<'b>(
                &self,
                frame: &mut glium::Frame,
                draw_parameters: &glium::DrawParameters<'b>
                $($(,$param: $type)*)?
                ) {
                use glium::Surface;

                for $shape in self.shapes.iter() {
                    frame
                        .draw(
                            &self.render_context.vertex_buffer,
                            &self.render_context.index_buffer,
                            &self.render_context.program,
                            &uniform! {
                                $($($param: $param,)*)?
                                $(
                                    $id: $value,
                                )+
                            },
                            draw_parameters,
                            )
                        .unwrap();
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_polyshape_container {
    ($Shape: ty; $($id: ident),* $(; $($param:ident: $type:ty),* )?) => {
        use $crate::polyshapes::PolyShapeContainer;
        #[allow(non_snake_case)]
        impl PolyShapeContainer<$Shape> {
            pub fn render<'b>(
                &self,
                frame: &mut glium::Frame,
                draw_parameters: &glium::DrawParameters<'b>
                $($(,$param: $type)*)?
                ) {
                use glium::Surface;

                for shape in self.shapes.iter() {
                    let (vbo, ibo) = self.raw_render_context.create_buffers(shape).unwrap();
                    frame
                        .draw(
                            &vbo,
                            &ibo,
                            &self.raw_render_context.program,
                            &uniform! {
                                $($($param: $param,)*)?
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

    ($Shape: ty; |$shape: ident| { $($id: ident: $value: expr),* } $(; $($param:ident: $type:ty),*)?) => {
        use $crate::polyshapes::PolyShapeContainer;
        #[allow(non_snake_case)]
        impl PolyShapeContainer<$Shape> {
            pub fn render<'b>(
                &self,
                frame: &mut glium::Frame,
                draw_parameters: &glium::DrawParameters<'b>
                $($(,$param: $type)*)?
                ) {
                use glium::Surface;

                for $shape in self.shapes.iter() {
                    let (vbo, ibo) = self.raw_render_context.create_buffers($shape).unwrap();
                    frame
                        .draw(
                            &vbo,
                            &ibo,
                            &self.raw_render_context.program,
                            &uniform! {
                                $($($param: $param,)*)?
                                $(
                                    $id: $value,
                                )*
                            },
                            draw_parameters,
                            )
                        .unwrap();
                }
            }
        }
    }
}

#[macro_export]
macro_rules! texture {
    ($display: expr, $image_path: expr, $image_format: expr) => {{
        use glium;
        use image;
        use std::{io::Cursor, rc::Rc};

        let image = image::load(Cursor::new(&include_bytes!($image_path)[..]), $image_format)
            .unwrap()
            .to_rgba();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        Rc::new(glium::texture::Texture2d::new($display, image).unwrap())
    }};
}

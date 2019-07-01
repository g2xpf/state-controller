#[macro_export]
macro_rules! world {
    ($expr: expr) => {{
        $crate::World::new($expr).finalize()
    }};
    ($head_expr: expr, $($tail_expr:expr),+) => {{
        let mut world = $crate::World::new($head_expr);
        $(
            world.register($tail_expr);
        )*
        world.finalize()
    }};
}

use std::f64::consts::PI;

pub trait Easing {
    fn fetch(t: f64) -> f64;
}

fn clamp(t: f64) -> f64 {
    t.max(0.0).min(1.0)
}

macro_rules! easing {
    ($name:ident, $t:ident -> $block:block) => {
        pub struct $name;

        impl Easing for $name {
            fn fetch($t: f64) -> f64 {
                clamp($block)
            }
        }
    };
}
easing!(Linear, t -> { t });
easing!(EaseInQuad, t -> { t * t });
easing!(EaseOutQuad, t -> {t * (2.0 - t)});
easing!(EaseInOutQuad, t -> { if t < 0.5 { 2.0*t*t } else {-1.0 +(4.0-2.0*t)*t}});

easing!(EaseInCubic, t -> {t * t * t});
easing!(EaseOutCubic, t -> { let t = t - 1.0; t * t * t + 1.0 });
easing!(EaseInOutCubic, t -> { if t < 0.5 { 4.0 * t * t * t } else { (t-1.0)*(2.0*t-2.0)*(2.0*t-2.0)+1.0}});

easing!(EaseInQuart, t -> {t*t*t*t});
easing!(EaseOutQuart, t -> { let t = 1.0 - t; 1.0 - t * t * t * t });
easing!(EaseInOutQuart, t -> { if t < 0.5 {8.0*t*t*t*t} else { let t = t - 1.0; 1.0-8.0*t*t*t*t}});

easing!(EaseInQuint, t -> {t*t*t*t*t});
easing!(EaseOutQuint, t -> { let t = t - 1.0; 1.0 + t * t * t * t * t});
easing!(EaseInOutQuint, t -> { if t < 0.5 { 16.0*t*t*t*t*t} else { let t = t - 1.0; 1.0+16.0*t*t*t*t*t}});

easing!(EaseInSin, t -> {1.0 + (PI / 2.0 * t - PI / 2.0).sin() });
easing!(EaseOutSin, t -> {(PI / 2.0 * t).sin()});
easing!(EaseInOutSin, t -> {1.0 + (PI * t - PI / 2.0).sin() / 2.0});

easing!(EaseInExpo, t -> { if t >= 1.0 { 1.0 } else { (10.0 * (t - 1.0)).exp2()}});
easing!(EaseOutExpo, t -> { if t >= 1.0 { 1.0 } else { 1.0 - (-10.0 * t).exp2()}});
easing!(EaseInOutExpo, t -> { if t <= 0.0 { 0.0 } else if t < 0.5 { 0.5 * (20.0 * t - 10.0).exp2() } else { 1.0 - 0.5 * (-20.0 * t + 1.0).exp2()}});

easing!(EaseInBack, t -> { t * t * t - t * (t * PI).sin()});
easing!(EaseOutBack, t -> { let t = 1.0 - t; 1.0 - t * t * t + t * (t * PI).sin()});
easing!(EaseInOutBack, t -> { if t < 0.5 { let t = 2.0 * t; 0.5 * (t * t * t - t * (t * PI).sin()) } else {let t = 2.0 - 2.0 * t; 0.5 + 0.5 * (1.0 - (t * t * t - t * (t + PI).sin()))}});

easing!(EaseInBounce, t -> { 1.0 - EaseOutBounce::fetch(1.0 - t) });
easing!(EaseOutBounce, t -> { if t < 4.0 / 11.0 { 121.0 / 16.0 * t * t} else if t < 8.0 / 11.0 { 363.0 / 40.0 * t * t - 99.0 / 10.0 * t + 17.0 / 5.0 } else if t < 9.0 / 10.0 { 4356.0 / 361.0 * t * t - 35442.0 / 1805.0 * t + 16061.0 / 1805.0} else {54.0 / 5.0 * t * t - 513.0 / 25.0 * t + 268.0 / 25.0}});
easing!(EaseInOutBounce, t -> { if t < 0.5 { 0.5 * EaseInBounce::fetch(t * 2.0)} else { 0.5 + 0.5 * EaseOutBounce::fetch(2.0 * t - 1.0)}});

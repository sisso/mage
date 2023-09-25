use crate::models::{Radians, V2};

pub fn angle_of(v: V2) -> Radians {
    // TODO: why angle_between return inverse Y?
    // v.angle_between(v2_right())
    v.y.atan2(v.x)
}

#[cfg(test)]
mod test {
    use super::*;

    use approx::assert_abs_diff_eq;
    use std::f32::consts::PI;

    #[test]
    pub fn test_from_angle() {
        let v = V2::from_angle(0.0);
        assert_abs_diff_eq!(1.0, v.x);
        assert_abs_diff_eq!(0.0, v.y);

        let v = V2::from_angle(PI * 0.5);
        assert_abs_diff_eq!(0.0, v.x);
        assert_abs_diff_eq!(1.0, v.y);

        let v = V2::from_angle(PI * -0.5);
        assert_abs_diff_eq!(0.0, v.x);
        assert_abs_diff_eq!(-1.0, v.y);
    }

    #[test]
    pub fn test_atang2() {
        let right = V2::new(0.0, 1.0);
        let angle = right.y.atan2(right.x);
        assert_abs_diff_eq!(0.5 * PI, angle);
    }

    #[test]
    pub fn test_angle_of() {
        assert_abs_diff_eq!(0.0 * PI, angle_of(V2::new(1.0, 0.0)));
        assert_abs_diff_eq!(1.0 * PI, angle_of(V2::new(-1.0, 0.0)));
        assert_abs_diff_eq!(0.5 * PI, angle_of(V2::new(0.0, 1.0)));
        assert_abs_diff_eq!(-0.5 * PI, angle_of(V2::new(0.0, -1.0)));
    }
}

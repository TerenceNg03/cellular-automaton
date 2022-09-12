#![allow(dead_code)]
type Point = (u32, u32);

pub const PLANE: &[Point] = &[(3, 3), (3, 4), (3, 5), (2, 5), (1, 4)];
pub const FROG: &[Point] = &[(3, 3), (3, 4), (3, 5), (4, 4), (4, 5), (4, 6)];
pub const TRAFFIC_LIGHT: &[Point] = &[
    (5, 3),
    (6, 3),
    (7, 3),
    (3, 5),
    (3, 6),
    (3, 7),
    (9, 5),
    (9, 6),
    (9, 7),
    (5, 9),
    (6, 9),
    (7, 9),
];

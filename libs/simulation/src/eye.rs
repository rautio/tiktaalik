use crate::*;
use std::f32::consts::*;

#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

const FOV_RANGE: f32 = 0.25;
const FOV_ANGLE: f32 = PI * FRAC_PI_4;
const CELLS: usize = 9;

impl Eye {
    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];
        for food in foods {
            // Determine if food is within range
            let vector = food.position - position;
            let dist = vector.norm();
            if dist >= self.fov_range {
                continue;
            }
            // Determine if food is within field of view
            let angle = na::Rotation2::rotation_between(&na::Vector2::y(), &vector).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }
            // Determine which cell sees the food
            let angle = angle + self.fov_range / 2.0;
            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);
            // Determine energy
            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }
        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }
    const TEST_EYE_CELLS: usize = 13;
    impl TestCase {
        pub fn run(self) {
            let eye = Eye::new(self.fov_range, self.fov_angle, TEST_EYE_CELLS);
            let actual_vision = eye.process_vision(
                na::Point2::new(self.x, self.y),
                na::Rotation2::new(self.rot),
                &self.foods,
            );
            let actual_vision: Vec<_> = actual_vision
                .into_iter()
                .map(|cell| {
                    if cell >= 0.7 {
                        "#"
                    } else if cell >= 0.3 {
                        "+"
                    } else if cell >= 0.1 {
                        "."
                    } else {
                        " "
                    }
                })
                .collect();
            let actual_vision = actual_vision.join("");
            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    fn food(x: f32, y: f32) -> Food {
        Food {
            position: na::Point2::new(x, y),
        }
    }
    mod different_fov_ranges {
        use super::*;
        use test_case::test_case;

        #[test_case(1.0, "    +        ")]
        #[test_case(0.9, "   +         ")]
        #[test_case(0.8, "   +         ")]
        #[test_case(0.7, "  .          ")]
        #[test_case(0.6, "  .          ")]
        #[test_case(0.5, "             ")]
        #[test_case(0.4, "             ")]
        #[test_case(0.3, "             ")]
        #[test_case(0.2, "             ")]
        #[test_case(0.1, "             ")]
        fn test(fov_range: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(0.5, 1.0)],
                fov_range,
                fov_angle: FRAC_PI_2,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                expected_vision,
            }
            .run()
        }
    }
}

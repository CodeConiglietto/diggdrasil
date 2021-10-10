use std::convert::TryFrom;

use ndarray::{s, Array2, ArrayView2, ArrayViewMut2, Axis};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::prelude::*;

pub struct Shadowcast {
    radius: u32,
    fov: Array2<bool>,
    frontier: Vec<(u32, (f32, f32))>,
}

impl Shadowcast {
    pub fn new(radius: u32) -> Self {
        let size = usize::try_from(radius).unwrap() * 2 + 1;

        Self {
            radius,
            fov: Array2::default((size, size)),
            frontier: Vec::new(),
        }
    }

    pub fn fov(&self) -> ArrayView2<bool> {
        self.fov.view()
    }

    pub fn radius(&self) -> u32 {
        self.radius
    }

    pub fn shadowcast<T>(&mut self, callbacks: &mut T)
    where
        T: ShadowcastCallbacks,
    {
        let radius = self.radius;
        let sradius = usize::try_from(radius).unwrap();

        self.fov.fill(false);
        self.fov[[sradius, sradius]] = true;

        if radius == 0 {
            return;
        }

        let mut top_left = self.fov.slice_mut(s![..=sradius, ..=sradius]);
        top_left.invert_axis(Axis(0));
        top_left.invert_axis(Axis(1));
        shadowcast_quadrant(
            radius,
            top_left,
            &mut self.frontier,
            &mut WrapCallbacks(callbacks, |pos: UPosition| {
                UPosition::new(radius - pos.x, radius - pos.y)
            }),
        );

        let mut top_right = self.fov.slice_mut(s![sradius.., ..=sradius]);
        top_right.invert_axis(Axis(1));
        shadowcast_quadrant(
            radius,
            top_right,
            &mut self.frontier,
            &mut WrapCallbacks(callbacks, |pos: UPosition| {
                UPosition::new(pos.x + radius, radius - pos.y)
            }),
        );

        let mut bottom_left = self.fov.slice_mut(s![..=sradius, sradius..]);
        bottom_left.invert_axis(Axis(0));
        shadowcast_quadrant(
            radius,
            bottom_left,
            &mut self.frontier,
            &mut WrapCallbacks(callbacks, |pos: UPosition| {
                UPosition::new(radius - pos.x, pos.y + radius)
            }),
        );

        let bottom_right = self.fov.slice_mut(s![sradius.., sradius..]);
        shadowcast_quadrant(
            radius,
            bottom_right,
            &mut self.frontier,
            &mut WrapCallbacks(callbacks, |pos: UPosition| {
                UPosition::new(pos.x + radius, pos.y + radius)
            }),
        );
    }
}

fn shadowcast_quadrant<T>(
    radius: u32,
    mut quadrant: ArrayViewMut2<bool>,
    frontier: &mut Vec<(u32, (f32, f32))>,
    callbacks: &mut T,
) where
    T: ShadowcastCallbacks,
{
    shadowcast_octant(radius, quadrant.view_mut(), frontier, callbacks);
    shadowcast_octant(
        radius,
        quadrant.view_mut().permuted_axes((1, 0)),
        frontier,
        &mut WrapCallbacks(callbacks, |pos: UPosition| UPosition::new(pos.y, pos.x)),
    )
}

fn shadowcast_octant<T>(
    radius: u32,
    mut quadrant: ArrayViewMut2<bool>,
    frontier: &mut Vec<(u32, (f32, f32))>,
    callbacks: &mut T,
) where
    T: ShadowcastCallbacks,
{
    let (width, _) = quadrant.dim();

    frontier.clear();
    frontier.push((0, (0.0, 1.0)));

    while let Some((x, (top_slope, bottom_slope))) = frontier.pop() {
        let next_x = x + 1;

        let mut last_top = None;

        let top = (top_slope * (x as f32 - 0.5) + 0.5).floor().max(0.0) as u32;
        let bottom = ((bottom_slope * (x as f32 + 0.5) - 0.5).ceil() as u32).min(x);

        for y in top..=bottom {
            if (x * x) + (y * y) > (radius * radius) {
                // Outside of circle, skip rest of column
                break;
            }

            callbacks.on_visible(UPosition::new(x, y));
            quadrant[[usize::try_from(x).unwrap(), usize::try_from(y).unwrap()]] = true;

            if usize::try_from(next_x).unwrap() < width {
                if callbacks.is_visible(UPosition::new(x, y)) {
                    last_top = Some(last_top.unwrap_or(y));
                } else {
                    // Recurse
                    if let Some(last_top) = last_top.take() {
                        let next_top_slope = if last_top == top {
                            top_slope
                        } else {
                            (last_top as f32 - 0.5) / (x as f32 - 0.5)
                        };

                        let next_bottom_slope = (y as f32 - 0.5) / (x as f32 + 0.5);

                        frontier.push((next_x, (next_top_slope, next_bottom_slope)));
                    }
                }
            }
        }

        // Recurse any leftover visible tiles
        if usize::try_from(next_x).unwrap() < width {
            if let Some(last_top) = last_top.take() {
                let next_top_slope = if last_top == top {
                    top_slope
                } else {
                    (last_top as f32 - 0.5) / (x as f32 - 0.5)
                };

                let next_bottom_slope = bottom_slope;
                frontier.push((next_x, (next_top_slope, next_bottom_slope)));
            }
        }
    }
}

pub trait ShadowcastCallbacks {
    fn is_visible(&mut self, pos: UPosition) -> bool;
    fn on_visible(&mut self, _pos: UPosition) {}
}

struct WrapCallbacks<'a, T, F>(&'a mut T, F);

impl<'a, T, F> ShadowcastCallbacks for WrapCallbacks<'a, T, F>
where
    T: ShadowcastCallbacks,
    F: Fn(UPosition) -> UPosition,
{
    fn is_visible(&mut self, pos: UPosition) -> bool {
        self.0.is_visible((self.1)(pos))
    }

    fn on_visible(&mut self, pos: UPosition) {
        self.0.on_visible((self.1)(pos));
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShadowcastData {
    radius: u32,
}

impl Serialize for Shadowcast {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ShadowcastData {
            radius: self.radius,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Shadowcast {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = ShadowcastData::deserialize(deserializer)?;
        Ok(Self::new(data.radius))
    }
}

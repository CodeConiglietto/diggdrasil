use ndarray::{s, Array2, ArrayView2, ArrayViewMut2, Axis};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct Shadowcast {
    radius: usize,
    fov: Array2<bool>,
    frontier: Vec<(usize, (f32, f32))>,
}

impl Shadowcast {
    pub fn new(radius: usize) -> Self {
        Self {
            radius,
            fov: Array2::default((radius * 2 + 1, radius * 2 + 1)),
            frontier: Vec::new(),
        }
    }

    pub fn fov(&self) -> ArrayView2<bool> {
        self.fov.view()
    }

    pub fn radius(&self) -> usize {
        self.radius
    }

    pub fn shadowcast<T>(&mut self, callbacks: &mut T)
    where
        T: ShadowcastCallbacks,
    {
        let radius = self.radius;

        self.fov.fill(false);
        self.fov[[radius, radius]] = true;

        if radius == 0 {
            return;
        }

        let mut top_left = self.fov.slice_mut(s![..=radius, ..=radius]);
        top_left.invert_axis(Axis(0));
        top_left.invert_axis(Axis(1));
        shadowcast_quadrant(
            radius,
            top_left,
            &mut self.frontier,
            &mut WrapCallbacks(callbacks, |x, y| (radius - x, radius - y)),
        );

        let mut top_right = self.fov.slice_mut(s![radius.., ..=radius]);
        top_right.invert_axis(Axis(1));
        shadowcast_quadrant(
            radius,
            top_right,
            &mut self.frontier,
            &mut WrapCallbacks(callbacks, |x, y| (x + radius, radius - y)),
        );

        let mut bottom_left = self.fov.slice_mut(s![..=radius, radius..]);
        bottom_left.invert_axis(Axis(0));
        shadowcast_quadrant(
            radius,
            bottom_left,
            &mut self.frontier,
            &mut WrapCallbacks(callbacks, |x, y| (radius - x, y + radius)),
        );

        let bottom_right = self.fov.slice_mut(s![radius.., radius..]);
        shadowcast_quadrant(
            radius,
            bottom_right,
            &mut self.frontier,
            &mut WrapCallbacks(callbacks, |x, y| (x + radius, y + radius)),
        );
    }
}

fn shadowcast_quadrant<T>(
    radius: usize,
    mut quadrant: ArrayViewMut2<bool>,
    frontier: &mut Vec<(usize, (f32, f32))>,
    callbacks: &mut T,
) where
    T: ShadowcastCallbacks,
{
    shadowcast_octant(radius, quadrant.view_mut(), frontier, callbacks);
    shadowcast_octant(
        radius,
        quadrant.view_mut().permuted_axes((1, 0)),
        frontier,
        &mut WrapCallbacks(callbacks, |x, y| (y, x)),
    )
}

fn shadowcast_octant<T>(
    radius: usize,
    mut quadrant: ArrayViewMut2<bool>,
    frontier: &mut Vec<(usize, (f32, f32))>,
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

        let top = (top_slope * (x as f32 - 0.5) + 0.5).floor().max(0.0) as usize;
        let bottom = ((bottom_slope * (x as f32 + 0.5) - 0.5).ceil() as usize).min(x);

        for y in top..=bottom {
            if (x * x) + (y * y) > (radius * radius) {
                // Outside of circle, skip rest of column
                break;
            }

            callbacks.on_visible(x, y);
            quadrant[[x, y]] = true;

            if next_x < width {
                if callbacks.is_visible(x, y) {
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
        if next_x < width {
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
    fn is_visible(&mut self, x: usize, y: usize) -> bool;

    fn on_visible(&mut self, _x: usize, _y: usize) {}
}

struct WrapCallbacks<'a, T, F>(&'a mut T, F);

impl<'a, T, F> ShadowcastCallbacks for WrapCallbacks<'a, T, F>
where
    T: ShadowcastCallbacks,
    F: Fn(usize, usize) -> (usize, usize),
{
    fn is_visible(&mut self, x: usize, y: usize) -> bool {
        let (x, y) = (self.1)(x, y);
        self.0.is_visible(x, y)
    }

    fn on_visible(&mut self, x: usize, y: usize) {
        let (x, y) = (self.1)(x, y);
        self.0.on_visible(x, y)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShadowcastData {
    radius: usize,
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

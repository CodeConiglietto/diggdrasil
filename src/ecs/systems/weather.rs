use rand::prelude::*;
use specs::prelude::*;

use crate::prelude::*;

pub struct WeatherSystem;

impl<'a> System<'a> for WeatherSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadExpect<'a, ViewportResource>,
        ReadExpect<'a, WeatherResource>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eids, lup, view, weat) = data;

        let (left, right, top, bottom) = view.get_viewport_bounds();

        match weat.current_weather {
            WeatherStatus::Clear => (),
            WeatherStatus::Raining => {
                ParticleBuilder::Rain {
                    wind_direction: weat.wind_direction,
                }
                .build(
                    &lup,
                    &eids,
                    IPosition::new(
                        thread_rng().gen_range(left..right) as i32,
                        thread_rng().gen_range(top..bottom) as i32,
                    ),
                );
            }
            _ => todo!(),
        }
    }
}

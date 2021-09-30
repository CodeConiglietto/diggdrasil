//Why in god's name am I reinventing trigonometry?
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IWave {
    Sin,
    Cos,
}

impl IWave {
    //TODO: make this work for negative angles
    pub fn get_value(self, theta: i32) -> i32 {
        if theta < 0 {
            todo!("get_value cannot currently handle negative numbers");
        }

        let mut theta = theta;

        //Shift theta if it's a sin wave to offset it
        if self == Self::Sin {
            theta += 2
        }

        //Sawtooth between 0..=9
        let theta_mod = theta % 8;

        //Flip second half of sawtooth to make triangle wave between 0..=4
        let triangle_theta = if theta_mod >= 4 {
            8 - theta_mod
        } else {
            theta_mod
        };

        //Shift down to get new range of -2..=2
        let shifted_triangle_theta = triangle_theta - 2;

        //Clamp to flatten peaks and troughs, new range is -1..=1
        let clamped_theta = shifted_triangle_theta.max(-1).min(1);

        clamped_theta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_wave_values(wave: IWave, angle: i32, result: i32) {
        let actual_result = wave.get_value(angle);

        assert_eq!(
            actual_result, result,
            "failed to assert that {:?}({}) == {}, actual result: {}",
            wave, angle, result, actual_result
        );

        println!("{:?}({}) = {}", wave, angle, actual_result);
    }

    //TODO: modify tests to allow for negatives
    #[test]
    fn test_sin() {
        test_wave_values(IWave::Sin, 0, 0);
        test_wave_values(IWave::Sin, 1, 1);
        test_wave_values(IWave::Sin, 2, 1);
        test_wave_values(IWave::Sin, 3, 1);
        test_wave_values(IWave::Sin, 4, 0);
        test_wave_values(IWave::Sin, 5, -1);
        test_wave_values(IWave::Sin, 6, -1);
        test_wave_values(IWave::Sin, 7, -1);
    }

    #[test]
    fn test_cos() {
        test_wave_values(IWave::Cos, 0, -1);
        test_wave_values(IWave::Cos, 1, -1);
        test_wave_values(IWave::Cos, 2, -0);
        test_wave_values(IWave::Cos, 3, 1);
        test_wave_values(IWave::Cos, 4, 1);
        test_wave_values(IWave::Cos, 5, 1);
        test_wave_values(IWave::Cos, 6, 0);
        test_wave_values(IWave::Cos, 7, -1);
    }
}

use core::f64;

use crate::domain::entities::forecast::waves_calculation::{WavesHeightResult, WavesInput};

pub(crate) struct WaveCalculationService;

impl WaveCalculationService {
    fn calculate_rss(input: &WavesInput) -> f64 {
        let height_primary = input.primary_swell.height;
        let height_secondary = input.secondary_swell.height;
        let height_wind_waves = input.wind_waves.height;

        f64::sqrt(height_primary.powi(2) + height_secondary.powi(2) + height_wind_waves.powi(2))
    }

    fn directional_delta(direction_1: f64, direction_2: f64) -> f64 {
        let diff = f64::abs(direction_1 - direction_2);
        f64::min(diff, 360.0 - diff)
    }

    fn calculate_combined_height(
        input: &WavesInput,
        directional_weight_primary_seconday: f64,
        directional_weight_primary_wind_waves: f64,
    ) -> f64 {
        let height_primary = input.primary_swell.height;
        let height_secondary = input.secondary_swell.height;
        let height_wind_waves = input.wind_waves.height;

        let total = height_primary.powi(2)
            + height_secondary.powi(2) * (1.0 + directional_weight_primary_seconday)
            + height_wind_waves.powi(2) * (1.0 + directional_weight_primary_wind_waves);

        f64::sqrt(total)
    }

    fn directional_weight(directional_delta: f64) -> f64 {
        f64::cos((directional_delta * f64::consts::PI) / 180.0)
    }

    fn calculate_directional_rss(input: &WavesInput) -> f64 {
        let directional_delta_primary_seconday = Self::directional_delta(
            input.primary_swell.direction,
            input.secondary_swell.direction,
        );

        let directional_delta_primary_wind_waves =
            Self::directional_delta(input.primary_swell.direction, input.wind_waves.direction);

        let directional_weight_primary_seconday =
            Self::directional_weight(directional_delta_primary_seconday);

        let directional_weight_primary_wind_waves =
            Self::directional_weight(directional_delta_primary_wind_waves);

        Self::calculate_combined_height(
            input,
            directional_weight_primary_seconday,
            directional_weight_primary_wind_waves,
        )
    }

    pub(crate) fn calculate(&self, input: WavesInput) -> WavesHeightResult {
        let rss = Self::calculate_rss(&input);
        let rss_directional = Self::calculate_directional_rss(&input);

        WavesHeightResult::new(rss, rss_directional)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::forecast::waves_calculation::{WavesInput, WavesSources};

    fn create_waves_input(
        primary_height: f64,
        primary_dir: f64,
        secondary_height: f64,
        secondary_dir: f64,
        wind_height: f64,
        wind_dir: f64,
    ) -> WavesInput {
        WavesInput::new(
            WavesSources::new(primary_height, primary_dir, 10.0),
            WavesSources::new(secondary_height, secondary_dir, 8.0),
            WavesSources::new(wind_height, wind_dir, 5.0),
        )
    }

    #[test]
    fn test_calculate_rss_basic() {
        let input = create_waves_input(2.0, 0.0, 1.5, 90.0, 1.0, 180.0);
        let rss = WaveCalculationService::calculate_rss(&input);
        let expected = 2.692582403567252;
        assert!((rss - expected).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_rss_zero_heights() {
        let input = create_waves_input(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let rss = WaveCalculationService::calculate_rss(&input);
        assert_eq!(rss, 0.0);
    }

    #[test]
    fn test_directional_delta_same_direction() {
        assert_eq!(WaveCalculationService::directional_delta(90.0, 90.0), 0.0);
    }

    #[test]
    fn test_directional_delta_opposite_directions() {
        let delta = WaveCalculationService::directional_delta(0.0, 180.0);
        assert_eq!(delta, 180.0);
    }

    #[test]
    fn test_directional_delta_wraps_around_360() {
        let delta = WaveCalculationService::directional_delta(350.0, 10.0);
        assert_eq!(delta, 20.0);
    }

    #[test]
    fn test_directional_weight_same_direction() {
        let weight = WaveCalculationService::directional_weight(0.0);
        assert!((weight - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_directional_weight_opposite_directions() {
        let weight = WaveCalculationService::directional_weight(180.0);
        assert!((weight - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_directional_weight_perpendicular() {
        let weight = WaveCalculationService::directional_weight(90.0);
        assert!((weight - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_directional_rss_same_directions() {
        // When all waves are in the same direction (w = 1):
        // H_total = sqrt(H_p² + H_s²(1+1) + H_w²(1+1))
        //         = sqrt(H_p² + 2*H_s² + 2*H_w²)
        let input = create_waves_input(2.0, 90.0, 1.5, 90.0, 1.0, 90.0);
        let rss_directional = WaveCalculationService::calculate_directional_rss(&input);
        let rss = WaveCalculationService::calculate_rss(&input);
        // When aligned, directional RSS should be greater than regular RSS
        assert!(rss_directional > rss);
        // Expected: sqrt(2.0² + 1.5²*2 + 1.0²*2) = sqrt(4.0 + 4.5 + 2.0) = sqrt(10.5) ≈ 3.240370
        let expected = 3.2403703492;
        assert!((rss_directional - expected).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_directional_rss_opposite_directions() {
        // When waves are opposite (w = -1):
        // H_total = sqrt(H_p² + H_s²(1-1) + H_w²(1-1))
        //         = sqrt(H_p² + 0 + 0) = H_p
        let input = create_waves_input(2.0, 0.0, 1.5, 180.0, 1.0, 180.0);
        let rss_directional = WaveCalculationService::calculate_directional_rss(&input);
        let rss = WaveCalculationService::calculate_rss(&input);
        // When opposite, directional RSS should be less than regular RSS
        assert!(rss_directional < rss);
        // Expected: sqrt(2.0² + 1.5²*0 + 1.0²*0) = sqrt(4.0) = 2.0
        let expected = 2.0;
        assert!((rss_directional - expected).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_directional_rss_perpendicular() {
        // When waves are perpendicular (w = 0):
        // H_total = sqrt(H_p² + H_s²(1+0) + H_w²(1+0))
        //         = sqrt(H_p² + H_s² + H_w²) = RSS
        let input = create_waves_input(2.0, 0.0, 1.5, 90.0, 0.0, 0.0);
        let rss_directional = WaveCalculationService::calculate_directional_rss(&input);
        let rss = WaveCalculationService::calculate_rss(&input);
        // When perpendicular, directional RSS should equal regular RSS
        assert!((rss_directional - rss).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_returns_both_results() {
        let input = create_waves_input(2.0, 45.0, 1.5, 135.0, 1.0, 225.0);
        let result = WaveCalculationService::calculate(&WaveCalculationService, input);

        let expected_rss = 2.692582403567252;
        assert!((result.rss() - expected_rss).abs() < 1e-10);
        assert!(result.rss_directional() > 0.0);
    }

    #[test]
    fn test_calculate_directional_rss_realistic_scenario() {
        // Waves with similar directions (aligned) will add constructively
        let input = create_waves_input(3.0, 270.0, 2.0, 280.0, 1.5, 275.0);
        let rss_directional = WaveCalculationService::calculate_directional_rss(&input);
        let rss = WaveCalculationService::calculate_rss(&input);
        assert!(rss_directional > 0.0);
        // When waves are aligned, directional RSS should exceed regular RSS
        assert!(rss_directional > rss);
        // With small angle differences, weights will be close to 1, so result will be higher than RSS
        assert!(rss_directional > 4.0);
    }
}

use crate::{
    application::use_cases::wave_calculation::WaveCalculation,
    domain::entities::forecast::waves_calculation::{WavesHeightResult, WavesInput},
};

pub(crate) struct WaveCalculationController {
    use_case: WaveCalculation,
}

impl WaveCalculationController {
    pub(crate) fn new(use_case: WaveCalculation) -> Self {
        Self { use_case }
    }

    pub(crate) fn calculate(&self, input: WavesInput) -> WavesHeightResult {
        self.use_case.execute(input)
    }
}

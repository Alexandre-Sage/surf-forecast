use crate::domain::{
    entities::forecast::waves_calculation::{WavesHeightResult, WavesInput},
    services::waves_service_calculation::WaveCalculationService,
};

pub(crate) struct WaveCalculation {
    service: WaveCalculationService,
}

impl WaveCalculation {
    pub(crate) fn new() -> Self {
        Self {
            service: WaveCalculationService,
        }
    }

    pub(crate) fn execute(&self, input: WavesInput) -> WavesHeightResult {
        self.service.calculate(input)
    }
}

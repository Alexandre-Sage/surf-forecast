use crate::{
    application::use_cases::wave_calculation::WaveCalculation,
    infrastructure::http::controllers::wave_calculation_controller::WaveCalculationController,
};

pub(crate) struct WaveCalculationContainer;

impl WaveCalculationContainer {
    pub(crate) fn new() -> Self {
        WaveCalculationContainer
    }

    fn use_case(&self) -> WaveCalculation {
        WaveCalculation::new()
    }

    pub(crate) fn controller(&self) -> WaveCalculationController {
        WaveCalculationController::new(self.use_case())
    }
}

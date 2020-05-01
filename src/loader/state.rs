pub struct LoadingState {
    loaded: f64,
}

impl LoadingState {
    pub fn new() -> Self {
        LoadingState { loaded: 0.0 }
    }

    pub fn progress(&self) -> f64 {
        self.loaded
    }

    pub fn set_progress(&mut self, progress: f64) {
        self.loaded = progress;
    }

    pub fn done(&self) -> bool {
        self.loaded >= 1.0
    }
}

#[derive(Debug)]
pub enum SpectralizerError {
    VisualizerError(VisualizerError),
}

#[derive(Debug)]
pub enum VisualizerError {
    LowLevelError(String),
    WindowBuildError(sdl2::video::WindowBuildError),
    CanvasBuildError(sdl2::IntegerOrSdlError),
}

impl From<String> for VisualizerError {
    fn from(error: String) -> VisualizerError {
        VisualizerError::LowLevelError(error)
    }
}

impl From<sdl2::video::WindowBuildError> for VisualizerError {
    fn from(error: sdl2::video::WindowBuildError) -> VisualizerError {
        VisualizerError::WindowBuildError(error)
    }
}

impl From<sdl2::IntegerOrSdlError> for VisualizerError {
    fn from(error: sdl2::IntegerOrSdlError) -> VisualizerError {
        VisualizerError::CanvasBuildError(error)
    }
}

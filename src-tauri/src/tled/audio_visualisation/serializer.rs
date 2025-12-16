use elk_led_controller::{AudioMonitor, FrequencyRange, VisualizationMode};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum ReadOnlyFrequencyRange {
    /// Bass frequencies (20-250 Hz)
    Bass,
    /// Mid-range frequencies (250-2000 Hz)
    Mid,
    /// High frequencies (2000-20000 Hz)
    High,
    /// Full spectrum
    Full,
}

impl From<FrequencyRange> for ReadOnlyFrequencyRange {
    fn from(value: FrequencyRange) -> Self {
        match value {
            FrequencyRange::Bass => Self::Bass,
            FrequencyRange::Mid => Self::Mid,
            FrequencyRange::High => Self::High,
            FrequencyRange::Full => Self::Full,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum ReadOnlyVisualizationMode {
    /// Frequencies map to colors (bass=red, mid=green, high=blue)
    FrequencyColor,
    /// Sound energy controls brightness
    EnergyBrightness,
    /// Beat detection triggers effects
    BeatEffects,
    /// Spectral flow pattern
    SpectralFlow,
    /// Enhanced frequency color mapping (warm for bass, cool for highs)
    EnhancedFrequencyColor,
    /// BPM synchronized effects
    BpmSync,
}

impl From<VisualizationMode> for ReadOnlyVisualizationMode {
    fn from(value: VisualizationMode) -> Self {
        match value {
            VisualizationMode::FrequencyColor => Self::FrequencyColor,
            VisualizationMode::EnergyBrightness => Self::EnergyBrightness,
            VisualizationMode::BeatEffects => Self::BeatEffects,
            VisualizationMode::SpectralFlow => Self::SpectralFlow,
            VisualizationMode::EnhancedFrequencyColor => Self::EnhancedFrequencyColor,
            VisualizationMode::BpmSync => Self::BpmSync,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadOnlyAudioVisualization {
    /// Which frequency range to monitor
    pub range: ReadOnlyFrequencyRange,
    /// How to visualize audio
    pub mode: ReadOnlyVisualizationMode,
    /// Audio volume sensitivity (0.0-1.0)
    pub sensitivity: f32,
    /// Whether bass should trigger color changes
    pub bass_color_trigger: bool,
    /// Whether mids should trigger brightness changes
    pub mid_brightness_trigger: bool,
    /// Whether highs should trigger effect changes
    pub high_effect_trigger: bool,
    /// Minimum time between visualization updates (ms)
    pub update_interval_ms: u32,
    /// Whether to sync state from audio directly to LED
    pub active: bool,
}

impl From<&AudioMonitor> for ReadOnlyAudioVisualization {
    fn from(value: &AudioMonitor) -> Self {
        let config = value.get_config();

        Self {
            range: config.range.into(),
            mode: config.mode.into(),
            sensitivity: config.sensitivity,
            bass_color_trigger: config.bass_color_trigger,
            mid_brightness_trigger: config.mid_brightness_trigger,
            high_effect_trigger: config.high_effect_trigger,
            update_interval_ms: config.update_interval_ms,
            active: config.active,
        }
    }
}

impl From<&mut AudioMonitor> for ReadOnlyAudioVisualization {
    fn from(value: &mut AudioMonitor) -> Self {
        (&*value).into()
    }
}

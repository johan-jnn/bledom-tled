export enum MusicFrequencyRange {
  /// Bass frequencies (20-250 Hz)
  Bass,
  /// Mid-range frequencies (250-2000 Hz)
  Mid,
  /// High frequencies (2000-20000 Hz)
  High,
  /// Full spectrum
  Full,
}

export enum MusicVisualisationMode {
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

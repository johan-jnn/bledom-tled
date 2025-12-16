import type { MusicFrequencyRange, MusicVisualisationMode } from "./music";

export interface DeviceAudioVisualizer {
  /// Which frequency range to monitor
  range: MusicFrequencyRange;
  /// How to visualize audio
  mode: MusicVisualisationMode;
  /// Audio volume sensitivity (0-255)
  sensitivity: number;
  /// Whether bass should trigger color changes
  bass_color_trigger: boolean;
  /// Whether mids should trigger brightness changes
  mid_brightness_trigger: boolean;
  /// Whether highs should trigger effect changes
  high_effect_trigger: boolean;
  /// Minimum time between visualization updates (ms)
  update_interval_ms: number;
  /// Whether to sync state from audio directly to LED
  active: boolean;
}

export interface Device {
  /**
   * Current power state
   */
  is_on: boolean;
  /**
   * Current RGB color (red, green, blue)
   */
  rgb_color: [number, number, number];
  /**
   * Current brightness (0-100)
   */
  brightness: number;
  /**
   * Current effect mode if active
   */
  effect?: number;
  /**
   * Current effect speed if an effect is active (0-100)
   */
  effect_speed?: number;
  /**
   * Current color temperature in Kelvin if using white mode
   */
  color_temp_kelvin?: number;
  /**
   * The device type name
   */
  device_type_name: string;
}

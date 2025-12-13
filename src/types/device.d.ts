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
   * Current effect speed if an effect is active
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

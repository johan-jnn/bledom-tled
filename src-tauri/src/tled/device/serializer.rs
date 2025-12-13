use elk_led_controller::BleLedDevice;
use serde::Serialize;

use crate::tled::device::manager::BleDeviceManager;
// static DEVICE: Mutex<Option<BleLedDevice>> = Mutex::const_new(None);

#[derive(Serialize)]
pub struct ReadOnlyBleLedDevice {
    /// Current power state
    pub is_on: bool,
    /// Current RGB color (red, green, blue)
    pub rgb_color: (u8, u8, u8),
    /// Current brightness (0-100)
    pub brightness: u8,
    /// Current effect mode if active
    pub effect: Option<u8>,
    /// Current effect speed if an effect is active
    pub effect_speed: Option<u8>,
    /// Current color temperature in Kelvin if using white mode
    pub color_temp_kelvin: Option<u32>,
    /// The device type name
    pub device_type_name: String,
}

impl From<&BleLedDevice> for ReadOnlyBleLedDevice {
    fn from(value: &BleLedDevice) -> Self {
        Self {
            color_temp_kelvin: value.color_temp_kelvin,
            effect: value.effect,
            brightness: value.brightness,
            effect_speed: value.effect_speed,
            is_on: value.is_on,
            rgb_color: value.rgb_color,
            device_type_name: value.get_device_type_name().into(),
        }
    }
}
impl From<&mut BleLedDevice> for ReadOnlyBleLedDevice {
    fn from(value: &mut BleLedDevice) -> Self {
        Self {
            color_temp_kelvin: value.color_temp_kelvin,
            effect: value.effect,
            brightness: value.brightness,
            effect_speed: value.effect_speed,
            is_on: value.is_on,
            rgb_color: value.rgb_color,
            device_type_name: value.get_device_type_name().into(),
        }
    }
}
impl TryFrom<&BleDeviceManager> for ReadOnlyBleLedDevice {
    type Error = String;
    fn try_from(value: &BleDeviceManager) -> Result<Self, Self::Error> {
        let Some(device) = &value.device else {
            return Err(String::from("Device not initialized."));
        };

        Ok(device.into())
    }
}
impl TryFrom<&mut BleDeviceManager> for ReadOnlyBleLedDevice {
    type Error = String;
    fn try_from(value: &mut BleDeviceManager) -> Result<Self, Self::Error> {
        let Some(device) = &value.device else {
            return Err(String::from("Device not initialized."));
        };

        Ok(device.into())
    }
}

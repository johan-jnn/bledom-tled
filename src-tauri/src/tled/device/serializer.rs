use elk_led_controller::BleLedDevice;
use serde::Serialize;

use crate::tled::{
    audio_visualisation::serializer::ReadOnlyAudioVisualization, device::manager::BleDeviceManager,
};

#[derive(Serialize, Debug, Clone)]
pub struct ReadOnlyBleLedDevice {
    /// Current power state
    pub is_on: bool,
    /// Current RGB color (red, green, blue)
    pub rgb_color: (u8, u8, u8),
    /// Current brightness
    pub brightness: u8,
    /// Current effect mode if active
    pub effect: Option<u8>,
    /// Current effect speed if an effect is active
    pub effect_speed: Option<u8>,
    /// Current color temperature in Kelvin if using white mode
    pub color_temp_kelvin: Option<u32>,
    /// The device type name
    pub device_type_name: String,

    /// If in use, the audio visualition setting
    pub audio_config: Option<ReadOnlyAudioVisualization>,
}

impl From<&BleLedDevice> for ReadOnlyBleLedDevice {
    fn from(value: &BleLedDevice) -> Self {
        Self {
            color_temp_kelvin: value.color_temp_kelvin,
            effect: value.effect,
            brightness: BleDeviceManager::from_0_100(value.brightness),
            effect_speed: value.effect_speed.map(BleDeviceManager::from_0_100),
            is_on: value.is_on,
            rgb_color: value.rgb_color,
            device_type_name: value.get_device_type_name().into(),
            // If we only have the device, we cannot know the audio configuration
            audio_config: None,
        }
    }
}
impl From<&mut BleLedDevice> for ReadOnlyBleLedDevice {
    fn from(value: &mut BleLedDevice) -> Self {
        ReadOnlyBleLedDevice::from(&*value)
    }
}
impl TryFrom<&BleDeviceManager> for ReadOnlyBleLedDevice {
    type Error = String;
    fn try_from(value: &BleDeviceManager) -> Result<Self, Self::Error> {
        let Some(device) = &value.device else {
            return Err(String::from("Device not initialized."));
        };

        let mut readonly = ReadOnlyBleLedDevice::from(device);
        if let Some(monitor) = &value.audio_monitor {
            readonly.audio_config = Some(monitor.into());
        }

        Ok(readonly)
    }
}
impl TryFrom<&mut BleDeviceManager> for ReadOnlyBleLedDevice {
    type Error = String;
    fn try_from(value: &mut BleDeviceManager) -> Result<Self, Self::Error> {
        ReadOnlyBleLedDevice::try_from(&*value)
    }
}

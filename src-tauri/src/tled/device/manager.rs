use elk_led_controller::{BleLedDevice, Error};

pub struct BleDeviceManager {
    pub device: Option<BleLedDevice>,
}

impl BleDeviceManager {
    /// Convert an u8 from [0-100] range to [0-255] range
    pub fn from_0_100(num: u8) -> u8 {
        ((num as u16 * u8::MAX as u16) / 100) as u8
    }
    /// Convert an u8 from [0-255] range to [0-100] range
    pub fn to_0_100(num: u8) -> u8 {
        ((num as u16 * 100) / u8::MAX as u16) as u8
    }

    pub fn empty() -> Self {
        Self { device: None }
    }

    pub async fn init(&mut self, force: bool) -> Result<&mut Self, Error> {
        if self.device.is_none() || force {
            self.device = Some(BleLedDevice::new().await?);
        };

        Ok(self)
    }

    pub async fn change_effect_settings(
        &mut self,
        effect: Option<u8>,
        speed: Option<u8>,
    ) -> Result<&mut Self, String> {
        let device = self
            .device
            .as_mut()
            .ok_or(String::from("Device not initialized."))?;

        let mut issues_stack: [Option<String>; 2] = [None, None];
        if let Some(e) = effect {
            if device.set_effect(e).await.is_err() {
                issues_stack[0] = Some(String::from("Failed to modify device's effect"));
            };
        }
        if let Some(s) = speed {
            if device.set_effect_speed(Self::to_0_100(s)).await.is_err() {
                issues_stack[1] = Some(String::from("Failed to modify device's effect speed"))
            }
        }

        let mut issue = String::new();
        for issue_info in issues_stack.into_iter().flatten() {
            if !issue.is_empty() {
                issue.push_str(" and ");
            }
            issue.push_str(&issue_info);
        }
        if !issue.is_empty() {
            issue.push('.');
            return Err(issue);
        }
        Ok(self)
    }

    pub async fn change_single(
        &mut self,
        r: Option<u8>,
        g: Option<u8>,
        b: Option<u8>,
        a: Option<u8>,
    ) -> Result<&mut Self, String> {
        let device = self
            .device
            .as_mut()
            .ok_or(String::from("Device not initializied."))?;

        let mut issues_stack: [Option<String>; 2] = [None, None];
        if (r.is_some() || g.is_some() || b.is_some())
            && device
                .set_color(
                    r.unwrap_or(device.rgb_color.0),
                    g.unwrap_or(device.rgb_color.1),
                    b.unwrap_or(device.rgb_color.2),
                )
                .await
                .is_err()
        {
            issues_stack[0] = Some(String::from("Failed to change device's color"));
        };

        if a.is_some()
            && device
                .set_brightness(a.map(Self::to_0_100).unwrap_or(device.brightness))
                .await
                .is_err()
        {
            issues_stack[1] = Some(String::from("Failed to change device's brighness"));
        };

        let mut issue = String::new();
        for issue_info in issues_stack.into_iter().flatten() {
            if !issue.is_empty() {
                issue.push_str(" and ");
            }
            issue.push_str(&issue_info);
        }
        if !issue.is_empty() {
            issue.push('.');
            return Err(issue);
        }

        Ok(self)
    }

    pub async fn change_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) -> Result<&mut Self, String> {
        self.change_single(Some(r), Some(g), Some(b), Some(a)).await
    }
}

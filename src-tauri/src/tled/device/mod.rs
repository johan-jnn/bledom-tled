use elk_led_controller::AudioMonitor;
use tokio::sync::Mutex;

use tauri::State;

use crate::tled::{
    audio_visualisation::serializer::{ReadOnlyAudioVisualization, ReadOnlyVisualizationMode},
    device::{manager::BleDeviceManager, serializer::ReadOnlyBleLedDevice},
};

pub mod manager;
pub mod serializer;

type ManagerState<'a> = State<'a, Mutex<BleDeviceManager>>;

#[tauri::command]
pub async fn device_init(
    manager: ManagerState<'_>,
    force: bool,
) -> Result<ReadOnlyBleLedDevice, String> {
    let mut device_manager = manager.lock().await;
    if device_manager.init(force).await.is_err() {
        return Err("Initialization failed.".into());
    }

    device_manager.readonly()
}

#[tauri::command]
pub async fn device_get(manager: ManagerState<'_>) -> Result<Option<ReadOnlyBleLedDevice>, ()> {
    let device_manager = manager.lock().await;
    Ok(device_manager.readonly().ok())
}

#[tauri::command]
pub async fn device_toggle(
    manager: ManagerState<'_>,
    power: bool,
) -> Result<ReadOnlyBleLedDevice, String> {
    let mut device_manager = manager.lock().await;
    let device = device_manager
        .device
        .as_mut()
        .ok_or(String::from("Device not initializied."))?;

    let result = if power {
        device.power_on().await
    } else {
        device.power_off().await
    };

    if result.is_err() {
        return Err("Failed to power on/off.".into());
    }

    device_manager.readonly()
}

#[tauri::command]
pub async fn device_change_only(
    manager: ManagerState<'_>,
    r: Option<u8>,
    g: Option<u8>,
    b: Option<u8>,
    a: Option<u8>,
) -> Result<ReadOnlyBleLedDevice, String> {
    let mut device_manager = manager.lock().await;
    device_manager.change_single(r, g, b, a).await?;

    device_manager.readonly()
}

#[tauri::command]
pub async fn device_change_all(
    manager: ManagerState<'_>,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) -> Result<ReadOnlyBleLedDevice, String> {
    let mut device_manager = manager.lock().await;
    device_manager.change_rgba(r, g, b, a).await?;

    device_manager.readonly()
}

#[tauri::command]
pub async fn device_set_effect(
    manager: ManagerState<'_>,
    effect: Option<u8>,
    speed: Option<u8>,
) -> Result<ReadOnlyBleLedDevice, String> {
    let mut device_manager = manager.lock().await;
    device_manager.change_effect_settings(effect, speed).await?;

    device_manager.readonly()
}

#[tauri::command]
pub async fn device_use_audio(
    manager: ManagerState<'_>,
    mode: Option<ReadOnlyVisualizationMode>,
    sensitivity: Option<u8>,
) -> Result<ReadOnlyBleLedDevice, String> {
    let mut device_manager = manager.lock().await;
    device_manager
        .change_audio_monitor_settings(mode.map(|v| v.into()), sensitivity)
        .await?;

    device_manager.readonly()
}

#[tauri::command]
pub async fn device_default_audio_configuration(
    state: ManagerState<'_>,
) -> Result<ReadOnlyAudioVisualization, String> {
    let device = state.lock().await;

    let conf = if let Some(c) = device.audio_monitor.as_ref() {
        c
    } else {
        &AudioMonitor::new().or(Err(String::from(
            "Audio monitor not created and failed to use default.",
        )))?
    };

    Ok(conf.into())
}

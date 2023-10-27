use anyhow::{bail, Result};
use async_trait::async_trait;

use std::path::PathBuf;

use pci_ids::Device;

use super::GpuImpl;

#[derive(Debug, Clone, Default)]

pub struct AmdGpu {
    pub device: Option<&'static Device>,
    pub pci_slot: String,
    pub driver: String,
    sysfs_path: PathBuf,
    first_hwmon_path: Option<PathBuf>,
}

impl AmdGpu {
    pub fn new(
        device: Option<&'static Device>,
        pci_slot: String,
        driver: String,
        sysfs_path: PathBuf,
        first_hwmon_path: Option<PathBuf>,
    ) -> Self {
        Self {
            device,
            pci_slot,
            driver,
            sysfs_path,
            first_hwmon_path,
        }
    }
}

#[async_trait]
impl GpuImpl for AmdGpu {
    fn device(&self) -> Option<&'static Device> {
        self.device
    }

    fn pci_slot(&self) -> String {
        self.pci_slot.clone()
    }

    fn driver(&self) -> String {
        self.driver.clone()
    }

    fn sysfs_path(&self) -> PathBuf {
        self.sysfs_path.clone()
    }

    fn first_hwmon(&self) -> Option<PathBuf> {
        self.first_hwmon_path.clone()
    }

    fn name(&self) -> Result<String> {
        self.drm_name()
    }

    async fn usage(&self) -> Result<isize> {
        self.drm_usage().await
    }

    async fn encode_usage(&self) -> Result<isize> {
        bail!("encode usage not implemented for AMD")
    }

    async fn decode_usage(&self) -> Result<isize> {
        bail!("decode usage not implemented for AMD")
    }

    async fn used_vram(&self) -> Result<isize> {
        self.drm_used_vram().await
    }

    async fn total_vram(&self) -> Result<isize> {
        self.drm_total_vram().await
    }

    async fn temperature(&self) -> Result<f64> {
        self.hwmon_temperature().await
    }

    async fn power_usage(&self) -> Result<f64> {
        self.hwmon_power_usage().await
    }

    async fn core_frequency(&self) -> Result<f64> {
        self.hwmon_core_frequency().await
    }

    async fn vram_frequency(&self) -> Result<f64> {
        self.hwmon_vram_frequency().await
    }

    async fn power_cap(&self) -> Result<f64> {
        self.hwmon_power_cap().await
    }

    async fn power_cap_max(&self) -> Result<f64> {
        self.hwmon_power_cap_max().await
    }
}

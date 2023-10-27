use std::{ops::Deref, str::FromStr};

use adw::prelude::*;

use gtk::{gio, glib};
use once_cell::sync::Lazy;
use strum_macros::{Display, EnumString, FromRepr};

use crate::config::APP_ID;

pub static SETTINGS: Lazy<Settings> = Lazy::new(Settings::default);

#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, EnumString, Display, Hash, FromRepr)]
pub enum Base {
    #[default]
    Decimal,
    Binary,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, EnumString, Display, Hash, FromRepr)]
pub enum TemperatureUnit {
    #[default]
    Celsius,
    Kelvin,
    Fahrenheit,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, EnumString, Display, Hash, FromRepr)]
pub enum RefreshSpeed {
    VerySlow,
    Slow,
    #[default]
    Normal,
    Fast,
    VeryFast,
}

impl RefreshSpeed {
    pub fn ui_refresh_interval(&self) -> f32 {
        match self {
            RefreshSpeed::VerySlow => 3.0,
            RefreshSpeed::Slow => 2.0,
            RefreshSpeed::Normal => 1.0,
            RefreshSpeed::Fast => 0.5,
            RefreshSpeed::VeryFast => 0.25,
        }
    }

    pub fn process_refresh_interval(&self) -> f32 {
        self.ui_refresh_interval() * 2.0
    }
}

#[derive(Clone, Debug, Hash)]
pub struct Settings(gio::Settings);

impl Settings {
    pub fn temperature_unit(&self) -> TemperatureUnit {
        TemperatureUnit::from_str(self.string("temperature-unit").as_str()).unwrap_or_default()
    }

    pub fn set_temperature_unit(
        &self,
        value: TemperatureUnit,
    ) -> Result<(), glib::error::BoolError> {
        self.set_string("temperature-unit", &value.to_string())
    }

    pub fn connect_temperature_unit<F: Fn(TemperatureUnit) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(Some("temperature-unit"), move |settings, _key| {
            f(
                TemperatureUnit::from_str(settings.string("temperature-unit").as_str())
                    .unwrap_or_default(),
            );
        })
    }

    pub fn base(&self) -> Base {
        Base::from_str(self.string("base").as_str()).unwrap_or_default()
    }

    pub fn set_base(&self, value: Base) -> Result<(), glib::error::BoolError> {
        self.set_string("base", &value.to_string())
    }

    pub fn connect_base<F: Fn(Base) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_changed(Some("base"), move |settings, _key| {
            f(Base::from_str(settings.string("base").as_str()).unwrap_or_default());
        })
    }

    pub fn refresh_speed(&self) -> RefreshSpeed {
        RefreshSpeed::from_str(self.string("refresh-speed").as_str()).unwrap_or_default()
    }

    pub fn set_refresh_speed(&self, value: RefreshSpeed) -> Result<(), glib::error::BoolError> {
        self.set_string("refresh-speed", &value.to_string())
    }

    pub fn connect_refresh_speed<F: Fn(RefreshSpeed) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(Some("refresh-speed"), move |settings, _key| {
            f(
                RefreshSpeed::from_str(settings.string("refresh-speed").as_str())
                    .unwrap_or_default(),
            );
        })
    }

    pub fn window_width(&self) -> i32 {
        self.int("window-width")
    }

    pub fn set_window_width(&self, value: i32) -> Result<(), glib::error::BoolError> {
        self.set_int("window-width", value)
    }

    pub fn connect_window_width<F: Fn(i32) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_changed(Some("window-width"), move |settings, _key| {
            f(settings.int("window-width"));
        })
    }

    pub fn window_height(&self) -> i32 {
        self.int("window-height")
    }

    pub fn set_window_height(&self, value: i32) -> Result<(), glib::error::BoolError> {
        self.set_int("window-height", value)
    }

    pub fn connect_window_height<F: Fn(i32) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_changed(Some("window-height"), move |settings, _key| {
            f(settings.int("window-width"));
        })
    }

    pub fn is_maximized(&self) -> bool {
        self.boolean("is-maximized")
    }

    pub fn set_maximized(&self, value: bool) -> Result<(), glib::error::BoolError> {
        self.set_boolean("is-maximized", value)
    }

    pub fn connect_maximized<F: Fn(bool) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_changed(Some("is-maximized"), move |settings, _key| {
            f(settings.boolean("is-maximized"));
        })
    }

    pub fn show_search_on_start(&self) -> bool {
        self.boolean("show-search-on-start")
    }

    pub fn set_show_search_on_start(&self, value: bool) -> Result<(), glib::error::BoolError> {
        self.set_boolean("show-search-on-start", value)
    }

    pub fn connect_show_search_on_start<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(Some("show-search-on-start"), move |settings, _key| {
            f(settings.boolean("show-search-on-start"));
        })
    }

    pub fn show_virtual_drives(&self) -> bool {
        self.boolean("show-virtual-drives")
    }

    pub fn set_show_virtual_drives(&self, value: bool) -> Result<(), glib::error::BoolError> {
        self.set_boolean("show-virtual-drives", value)
    }

    pub fn connect_show_virtual_drives<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(Some("show-virtual-drives"), move |settings, _key| {
            f(settings.boolean("show-virtual-drives"));
        })
    }

    pub fn show_virtual_network_interfaces(&self) -> bool {
        self.boolean("show-virtual-network-interfaces")
    }

    pub fn set_show_virtual_network_interfaces(
        &self,
        value: bool,
    ) -> Result<(), glib::error::BoolError> {
        self.set_boolean("show-virtual-network-interfaces", value)
    }

    pub fn connect_show_virtual_network_interfaces<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(
            Some("show-virtual-network-interfaces"),
            move |settings, _key| f(settings.boolean("show-virtual-network-interfaces")),
        )
    }

    pub fn sidebar_details(&self) -> bool {
        self.boolean("sidebar-details")
    }

    pub fn set_sidebar_details(&self, value: bool) -> Result<(), glib::error::BoolError> {
        self.set_boolean("sidebar-details", value)
    }

    pub fn connect_sidebar_details<F: Fn(bool) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_changed(Some("sidebar-details"), move |settings, _key| {
            f(settings.boolean("sidebar-details"));
        })
    }

    pub fn network_bits(&self) -> bool {
        self.boolean("network-bits")
    }

    pub fn set_network_bits(&self, value: bool) -> Result<(), glib::error::BoolError> {
        self.set_boolean("network-bits", value)
    }

    pub fn connect_network_bits<F: Fn(bool) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_changed(Some("network-bits"), move |settings, _key| {
            f(settings.boolean("network-bits"));
        })
    }

    pub fn apps_show_memory(&self) -> bool {
        self.boolean("apps-show-memory")
    }

    pub fn set_apps_show_memory(&self, value: bool) -> Result<(), glib::error::BoolError> {
        self.set_boolean("apps-show-memory", value)
    }

    pub fn connect_apps_show_memory<F: Fn(bool) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_changed(Some("apps-show-memory"), move |settings, _key| {
            f(settings.boolean("apps-show-memory"));
        })
    }

    pub fn apps_show_cpu(&self) -> bool {
        self.boolean("apps-show-cpu")
    }

    pub fn set_apps_show_cpu(&self, value: bool) -> Result<(), glib::error::BoolError> {
        self.set_boolean("apps-show-cpu", value)
    }

    pub fn connect_apps_show_cpu<F: Fn(bool) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_changed(Some("apps-show-cpu"), move |settings, _key| {
            f(settings.boolean("apps-show-cpu"));
        })
    }

    pub fn apps_show_drive_read_speed(&self) -> bool {
        self.boolean("apps-show-drive-read-speed")
    }

    pub fn set_apps_show_drive_read_speed(
        &self,
        value: bool,
    ) -> Result<(), glib::error::BoolError> {
        self.set_boolean("apps-show-drive-read-speed", value)
    }

    pub fn connect_apps_show_drive_read_speed<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(Some("apps-show-drive-read-speed"), move |settings, _key| {
            f(settings.boolean("apps-show-drive-read-speed"));
        })
    }

    pub fn apps_show_drive_read_total(&self) -> bool {
        self.boolean("apps-show-drive-read-total")
    }

    pub fn set_apps_show_drive_read_total(
        &self,
        value: bool,
    ) -> Result<(), glib::error::BoolError> {
        self.set_boolean("apps-show-drive-read-total", value)
    }

    pub fn connect_apps_show_drive_read_total<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(Some("apps-show-drive-read-total"), move |settings, _key| {
            f(settings.boolean("apps-show-drive-read-total"));
        })
    }

    pub fn apps_show_drive_write_speed(&self) -> bool {
        self.boolean("apps-show-drive-write-speed")
    }

    pub fn set_apps_show_drive_write_speed(
        &self,
        value: bool,
    ) -> Result<(), glib::error::BoolError> {
        self.set_boolean("apps-show-drive-write-speed", value)
    }

    pub fn connect_apps_show_drive_write_speed<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(
            Some("apps-show-drive-write-speed"),
            move |settings, _key| f(settings.boolean("apps-show-drive-write-speed")),
        )
    }

    pub fn apps_show_drive_write_total(&self) -> bool {
        self.boolean("apps-show-drive-write-total")
    }

    pub fn set_apps_show_drive_write_total(
        &self,
        value: bool,
    ) -> Result<(), glib::error::BoolError> {
        self.set_boolean("apps-show-drive-write-total", value)
    }

    pub fn connect_apps_show_drive_write_total<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(
            Some("apps-show-drive-write-total"),
            move |settings, _key| f(settings.boolean("apps-show-drive-write-total")),
        )
    }

    pub fn processes_show_id(&self) -> bool {
        self.boolean("processes-show-id")
    }

    pub fn set_processes_show_id(&self, value: bool) -> Result<(), glib::error::BoolError> {
        self.set_boolean("processes-show-id", value)
    }

    pub fn connect_processes_show_id<F: Fn(bool) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_changed(Some("processes-show-id"), move |settings, _key| {
            f(settings.boolean("processes-show-id"));
        })
    }

    pub fn processes_show_user(&self) -> bool {
        self.boolean("processes-show-user")
    }

    pub fn set_processes_show_user(&self, value: bool) -> Result<(), glib::error::BoolError> {
        self.set_boolean("processes-show-user", value)
    }

    pub fn connect_processes_show_user<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(Some("processes-show-user"), move |settings, _key| {
            f(settings.boolean("processes-show-user"));
        })
    }

    pub fn processes_show_memory(&self) -> bool {
        self.boolean("processes-show-memory")
    }

    pub fn set_processes_show_memory(&self, value: bool) -> Result<(), glib::error::BoolError> {
        self.set_boolean("processes-show-memory", value)
    }

    pub fn connect_processes_show_memory<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(Some("processes-show-memory"), move |settings, _key| {
            f(settings.boolean("processes-show-memory"));
        })
    }

    pub fn processes_show_cpu(&self) -> bool {
        self.boolean("processes-show-cpu")
    }

    pub fn set_processes_show_cpu(&self, value: bool) -> Result<(), glib::error::BoolError> {
        self.set_boolean("processes-show-cpu", value)
    }

    pub fn connect_processes_show_cpu<F: Fn(bool) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_changed(Some("processes-show-cpu"), move |settings, _key| {
            f(settings.boolean("processes-show-cpu"));
        })
    }

    pub fn processes_show_drive_read_speed(&self) -> bool {
        self.boolean("processes-show-drive-read-speed")
    }

    pub fn set_processes_show_drive_read_speed(
        &self,
        value: bool,
    ) -> Result<(), glib::error::BoolError> {
        self.set_boolean("processes-show-drive-read-speed", value)
    }

    pub fn connect_processes_show_drive_read_speed<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(
            Some("processes-show-drive-read-speed"),
            move |settings, _key| f(settings.boolean("processes-show-drive-read-speed")),
        )
    }

    pub fn processes_show_drive_read_total(&self) -> bool {
        self.boolean("processes-show-drive-read-total")
    }

    pub fn set_processes_show_drive_read_total(
        &self,
        value: bool,
    ) -> Result<(), glib::error::BoolError> {
        self.set_boolean("processes-show-drive-read-total", value)
    }

    pub fn connect_processes_show_drive_read_total<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(
            Some("processes-show-drive-read-total"),
            move |settings, _key| f(settings.boolean("processes-show-drive-read-total")),
        )
    }

    pub fn processes_show_drive_write_speed(&self) -> bool {
        self.boolean("processes-show-drive-write-speed")
    }

    pub fn set_processes_show_drive_write_speed(
        &self,
        value: bool,
    ) -> Result<(), glib::error::BoolError> {
        self.set_boolean("processes-show-drive-write-speed", value)
    }

    pub fn connect_processes_show_drive_write_speed<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(
            Some("processes-show-drive-write-speed"),
            move |settings, _key| f(settings.boolean("processes-show-drive-write-speed")),
        )
    }

    pub fn processes_show_drive_write_total(&self) -> bool {
        self.boolean("processes-show-drive-write-total")
    }

    pub fn set_processes_show_drive_write_total(
        &self,
        value: bool,
    ) -> Result<(), glib::error::BoolError> {
        self.set_boolean("processes-show-drive-write-total", value)
    }

    pub fn connect_processes_show_drive_write_total<F: Fn(bool) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(
            Some("processes-show-drive-write-total"),
            move |settings, _key| f(settings.boolean("processes-show-drive-write-total")),
        )
    }

    pub fn show_logical_cpus(&self) -> bool {
        self.boolean("show-logical-cpus")
    }

    pub fn set_show_logical_cpus(&self, value: bool) -> Result<(), glib::error::BoolError> {
        self.set_boolean("show-logical-cpus", value)
    }

    pub fn connect_show_logical_cpus<F: Fn(bool) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_changed(Some("show-logical-cpus"), move |settings, _key| {
            f(settings.boolean("show-logical-cpus"));
        })
    }
}

impl Deref for Settings {
    type Target = gio::Settings;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self(gio::Settings::new(APP_ID))
    }
}

unsafe impl Send for Settings {}
unsafe impl Sync for Settings {}

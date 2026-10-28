//! Typed, lazily created per-device state owned by layers above the device.
//!
//! The device cannot name resource, transfer or command types: they depend on it. Those
//! layers keep their device-scoped state (the managed-resource store, built-in samplers,
//! parked setup pools) in a fixed slot here instead. A slot is created on first use,
//! lives as long as the device and is released in `Device::drop`, after the GPU is idle
//! and before any device component drops.
use std::{any::Any, sync::OnceLock};

/// State a higher layer parks in a device slot.
pub(crate) trait DeviceExtension: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    /// Runs from `Device::drop` once the GPU is idle, before components drop.
    fn release_for_device_drop(&self) {}
    /// Runs when the device is under memory pressure: drop idle capacity.
    fn trim_under_pressure(&self) {}
}

/// One slot per extension; the discriminant indexes the slot array.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ExtensionSlot {
    Resources,
    BuiltinSamplers,
    Setup,
    PendingAcquires,
}
const SLOTS: usize = 4;

#[derive(Default)]
pub(crate) struct Extensions {
    slots: [OnceLock<Box<dyn DeviceExtension>>; SLOTS],
}

impl Extensions {
    /// The extension in `slot`, if it was created. A slot only ever holds one type.
    pub(crate) fn get<T: DeviceExtension>(&self, slot: ExtensionSlot) -> Option<&T> {
        self.slots[slot as usize]
            .get()
            .and_then(|extension| extension.as_any().downcast_ref::<T>())
    }

    /// The extension in `slot`, created by `init` on first use. When two threads race,
    /// the first value stored wins and the other is dropped.
    pub(crate) fn get_or_try_init<T: DeviceExtension>(
        &self,
        slot: ExtensionSlot,
        init: impl FnOnce() -> crate::Result<T>,
    ) -> crate::Result<&T> {
        if let Some(existing) = self.get::<T>(slot) {
            return Ok(existing);
        }
        let _ = self.slots[slot as usize].set(Box::new(init()?));
        Ok(self.get::<T>(slot).expect("extension slot holds its own type"))
    }

    /// [`Self::get_or_try_init`] for an extension that cannot fail to be created.
    pub(crate) fn get_or_init<T: DeviceExtension>(
        &self,
        slot: ExtensionSlot,
        init: impl FnOnce() -> T,
    ) -> &T {
        if let Some(existing) = self.get::<T>(slot) {
            return existing;
        }
        let _ = self.slots[slot as usize].set(Box::new(init()));
        self.get::<T>(slot).expect("extension slot holds its own type")
    }

    pub(crate) fn trim_under_pressure(&self) {
        for slot in &self.slots {
            if let Some(extension) = slot.get() {
                extension.trim_under_pressure();
            }
        }
    }

    pub(crate) fn release_for_device_drop(&self) {
        for slot in &self.slots {
            if let Some(extension) = slot.get() {
                extension.release_for_device_drop();
            }
        }
    }
}

#[cfg(test)]
mod tests;

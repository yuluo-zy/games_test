//! [`Caches`]: device-scoped object caches — the managed-resource store,
//! built-in samplers and the shared setup context.
//!
//! Lock order: the built-in sampler table and the setup-context slot are each
//! held while creating their object, which may take memory (allocator) locks
//! and the managed-store locks; neither is held while taking the other, any
//! queue lock or a retirement lock.
use parking_lot::Mutex;
use std::sync::{Arc, Weak};

use super::Device;
use crate::raw::sampler::{BUILTIN_SAMPLER_SPECS, Sampler, SamplerStorage};
use crate::raw::setup::SetupContext;

struct BuiltinSamplerEntry {
    sampler: Weak<Sampler>,
    // Share the ticket with the payload: a concurrent last-owner Drop may still
    // be running after this entry's Weak can no longer be upgraded.
    _storage: Arc<SamplerStorage>,
}

pub(crate) struct Caches {
    /// Managed buffers/textures addressed by stable handles.
    pub(crate) resources: crate::raw::managed::resources::ManagedResources,
    builtin_samplers: Mutex<[Option<BuiltinSamplerEntry>; BUILTIN_SAMPLER_SPECS.len()]>,
    /// Shared setup context; weak because its pools own the device.
    setup: Mutex<Weak<SetupContext>>,
}

impl Caches {
    pub(crate) fn new(stats: Arc<crate::raw::allocator::AllocatorStats>) -> crate::Result<Self> {
        Ok(Self {
            resources: crate::raw::managed::resources::ManagedResources::new(stats)?,
            builtin_samplers: Mutex::new(std::array::from_fn(|_| None)),
            setup: Mutex::new(Weak::new()),
        })
    }

    /// Release the cached sampler tickets (the samplers themselves are
    /// owned by their users and retired through the device).
    pub(crate) fn clear_samplers(&mut self) {
        for entry in self.builtin_samplers.get_mut().iter_mut() {
            entry.take();
        }
    }
}

impl Device {
    /// Managed buffers/textures addressed by stable handles.
    pub fn resources(&self) -> &crate::raw::managed::resources::ManagedResources {
        &self.caches.resources
    }
    /// The shared reusable setup context, created on first use. The device
    /// keeps it only while some caller holds the returned `Arc`.
    pub fn setup_context(&self) -> crate::Result<Arc<SetupContext>> {
        let mut cached = self.caches.setup.lock();
        if let Some(context) = cached.upgrade() {
            return Ok(context);
        }
        let context = SetupContext::new(self.shared())?;
        *cached = Arc::downgrade(&context);
        Ok(context)
    }
    /// Built-ins are cached weakly so live resources can own the device without a cycle.
    pub fn get_built_in_sampler(&self, index: usize) -> crate::Result<Option<Arc<Sampler>>> {
        if self.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let Some(spec) = BUILTIN_SAMPLER_SPECS.get(index) else {
            return Ok(None);
        };
        let mut cache = self.caches.builtin_samplers.lock();
        if let Some(sampler) = cache[index]
            .as_ref()
            .and_then(|entry| entry.sampler.upgrade())
        {
            return Ok(Some(sampler));
        }
        // Discard the dead weak allocation before admitting its replacement.
        cache[index] = None;
        let (sampler, storage) = Sampler::new_builtin(self.shared(), &spec.create_info())?;
        cache[index] = Some(BuiltinSamplerEntry {
            sampler: Arc::downgrade(&sampler),
            _storage: storage,
        });
        Ok(Some(sampler))
    }
    pub fn try_get_built_in_sampler_by_name(
        &self,
        name: &str,
    ) -> crate::Result<Option<Arc<Sampler>>> {
        match BUILTIN_SAMPLER_SPECS.iter().position(|s| s.name == name) {
            Some(index) => self.get_built_in_sampler(index),
            None => Ok(None),
        }
    }
    pub fn get_built_in_sampler_by_name(&self, name: &str) -> crate::Result<Arc<Sampler>> {
        self.try_get_built_in_sampler_by_name(name)?
            .ok_or_else(|| crate::Error::invalid_usage(format!("unknown built-in sampler: {name}")))
    }
}

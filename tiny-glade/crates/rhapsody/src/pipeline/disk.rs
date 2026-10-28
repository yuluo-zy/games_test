//! Persistent on-disk pipeline cache: format, integrity and atomic writes.
use std::io::{Read, Write};
use crate::Error;
use crate::Result;
use crate::device::Device;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use super::shader::PipelineCache;
use super::shader::PipelineCacheData;

pub(super) const CACHE_MAGIC: &[u8; 8] = b"RHPC0001";

pub(super) const MAX_DISK_BYTES: u64 = crate::pipeline::shader::MAX_PIPELINE_CACHE_BYTES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiskCacheStatus {
    Missing,
    Loaded,
    Rejected,
}

pub struct PersistentPipelineCache {
    pub cache: Arc<PipelineCache>,
    path: PathBuf,
    identity: [u8; 32],
    pub status: DiskCacheStatus,
}

impl PersistentPipelineCache {
    pub fn open(device: Arc<Device>, path: impl Into<PathBuf>, capacity: usize) -> Result<Self> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let path = path.into();
        let identity = identity(&device);
        let mut bytes = None;
        let mut status = match fs::File::open(&path) {
            Ok(mut file) => {
                let length = file.metadata().ok().map(|metadata| metadata.len());
                let loaded =
                    if let Some(length) = length.filter(|&length| length <= MAX_DISK_BYTES + 128) {
                        let mut payload =
                            PipelineCacheData::zeroed(&device.memory.stats, length as usize)?;
                        let mut trailing = [0];
                        if file.read_exact(payload.as_mut_slice()).is_ok()
                            && file.read(&mut trailing).is_ok_and(|read| read == 0)
                            && decode(&payload, &identity).is_some()
                        {
                            bytes = Some(payload);
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };
                if loaded {
                    DiskCacheStatus::Loaded
                } else {
                    DiskCacheStatus::Rejected
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => DiskCacheStatus::Missing,
            Err(_) => DiskCacheStatus::Rejected,
        };
        let mut data = if status == DiskCacheStatus::Loaded {
            decode(bytes.as_deref().unwrap_or_default(), &identity).unwrap_or_default()
        } else {
            &[]
        };
        if !data.is_empty() && !crate::pipeline::shader::valid_pipeline_cache_data(&device, data) {
            status = DiskCacheStatus::Rejected;
            data = &[];
        }
        let cache = match PipelineCache::with_capacity(device.clone(), data, capacity) {
            Ok(cache) => cache,
            Err(error) if !data.is_empty() && matches!(error, Error::Vulkan(_)) => {
                status = DiskCacheStatus::Rejected;
                PipelineCache::with_capacity(device, &[], capacity)?
            }
            Err(error) => return Err(error),
        };
        Ok(Self {
            cache: Arc::new(cache),
            path,
            identity,
            status,
        })
    }
    pub fn save(&self) -> Result<()> {
        let data = self.cache.data()?;
        if data.len() as u64 > MAX_DISK_BYTES {
            return Err(Error::invalid_usage(
                "pipeline disk cache exceeds its byte limit",
            ));
        }
        // Stream the fixed envelope and the accounted driver bytes. Building a
        // second 64 MiB copy would double peak host memory during save.
        let mut header = [0; 56];
        header[..8].copy_from_slice(CACHE_MAGIC);
        header[8..40].copy_from_slice(&self.identity);
        header[40..48].copy_from_slice(&(data.len() as u64).to_le_bytes());
        header[48..].copy_from_slice(&checksum(&data).to_le_bytes());
        atomic_write_parts(&self.path, &[&header, &data])
    }
}

pub(super) fn identity(device: &Device) -> [u8; 32] {
    let properties = device.properties();
    let mut bytes = [0; 32];
    for (index, value) in [
        properties.vendor_id,
        properties.device_id,
        properties.driver_version,
        properties.api_version,
    ]
    .into_iter()
    .enumerate()
    {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    bytes[16..].copy_from_slice(&properties.pipeline_cache_uuid);
    bytes
}

pub(super) fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
    })
}

#[cfg(test)]
pub(super) fn encode(identity: &[u8], data: &[u8]) -> Vec<u8> {
    let mut bytes = CACHE_MAGIC.to_vec();
    bytes.extend(identity);
    bytes.extend((data.len() as u64).to_le_bytes());
    bytes.extend(checksum(data).to_le_bytes());
    bytes.extend(data);
    bytes
}

pub(super) fn decode<'a>(bytes: &'a [u8], identity: &[u8]) -> Option<&'a [u8]> {
    let header = 8 + identity.len() + 16;
    if bytes.len() < header
        || &bytes[..8] != CACHE_MAGIC
        || &bytes[8..8 + identity.len()] != identity
    {
        return None;
    }
    let offset = 8 + identity.len();
    let size = u64::from_le_bytes(bytes[offset..offset + 8].try_into().ok()?);
    if size > MAX_DISK_BYTES || usize::try_from(size).ok()? != bytes.len() - header {
        return None;
    }
    let expected = u64::from_le_bytes(bytes[offset + 8..header].try_into().ok()?);
    let data = &bytes[header..];
    (checksum(data) == expected).then_some(data)
}

pub(super) fn atomic_write_parts(path: &Path, parts: &[&[u8]]) -> Result<()> {
    static NEXT_FILE: AtomicU64 = AtomicU64::new(1);
    let mut temporary = path.as_os_str().to_owned();
    temporary.push(format!(
        ".{}.{}.tmp",
        std::process::id(),
        NEXT_FILE.fetch_add(1, Ordering::Relaxed)
    ));
    let temporary = PathBuf::from(temporary);
    atomic_write_temporary(path, &temporary, parts)
}

pub(super) fn atomic_write_temporary(path: &Path, temporary: &Path, parts: &[&[u8]]) -> Result<()> {
    let mut created = false;
    let result = (|| {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(temporary)?;
        created = true;
        for part in parts {
            file.write_all(part)?;
        }
        file.sync_all()?;
        drop(file);
        fs::rename(temporary, path)
    })();
    if result.is_err() && created {
        let _ = fs::remove_file(temporary);
    }
    result.map_err(|error| Error::invalid_usage_with("save persistent pipeline cache", error))
}

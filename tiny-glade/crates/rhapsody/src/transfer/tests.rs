use super::*;
use super::route::texture_publish;
use crate::native::AccessScope;
use super::texture::TransferPublish;
#[test]
fn transfer_configuration_rejects_unbounded_or_overflowing_capacity() {
    for config in [
        TransferConfig {
            slots: 0,
            ..Default::default()
        },
        TransferConfig {
            slots: 65,
            ..Default::default()
        },
        TransferConfig {
            chunk_bytes: u64::MAX,
            ..Default::default()
        },
        TransferConfig {
            max_bytes: 1,
            ..Default::default()
        },
    ] {
        assert!(config.validate().is_err());
    }
}

#[test]
fn texture_publish_moves_ownership_only_for_exclusive_images_of_another_family() {
    assert_eq!(texture_publish(0, 0, false), TransferPublish::Local);
    assert_eq!(texture_publish(0, 2, true), TransferPublish::Local);
    assert_eq!(
        texture_publish(0, 2, false),
        TransferPublish::Release {
            destination_family: 0
        }
    );
}

mod cross_family {
    //! Synthetic cross-family protocol is inspected by the CPU recorder.
    //! Its fictitious barriers are never issued to Vulkan. Real upload,
    //! graph and readback commands below use only the actual primary family;
    //! these tests do not claim native cross-family hardware coverage.
    use super::*;
    use crate::command::pending_acquire::tests::{complete_synthetic_acquire_metadata, owner, released_buffer_with};
    use crate::tracking::range_state::{RangeOwner, RangeTransfer};
    use crate::sync::QueueId;

    const GRAPHICS: QueueId = match QueueId::new(0) {
        Some(queue) => queue,
        None => unreachable!(),
    };

    fn device() -> Option<Arc<Device>> {
        let mut config = crate::device::DeviceConfig::headless();
        // Native copy coverage is deliberately same-family on every GPU.
        config.queues.transfer = crate::device::QueuePreference::Off;
        match crate::device::Device::new_headless(&config) {
            Ok(device) => Some(device),
            Err(error) => {
                assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
                None
            }
        }
    }
    fn submitted(device: &Device) -> u64 {
        device.queues.timelines.submitted(GRAPHICS).unwrap()
    }
    /// `count` buffers released to the graphics family whose acquires are
    /// queued by the transfer registration path.
    fn released_uploads(
        device: &Arc<Device>,
        count: usize,
        usage: vk::BufferUsageFlags,
    ) -> Vec<(Arc<Buffer>, RangeTransfer)> {
        // SAFETY: an empty batch only signals the device timeline.
        let release = unsafe { device.submit(GRAPHICS, &[]) }.unwrap();
        let uploads: Vec<_> = (0..count)
            .map(|_| released_buffer_with(device, release, usage))
            .collect();
        let mut acquire_reservation = device.reserve_pending_acquires(count).unwrap();
        for (buffer, transfer) in &uploads {
            record::queue_acquires(
                device,
                &mut acquire_reservation,
                &crate::command::AcquireTarget::buffer(buffer),
                std::slice::from_ref(transfer),
                release,
            )
            .unwrap();
        }
        uploads
    }

    #[test]
    fn synthetic_scope_then_same_family_graph_costs_no_extra_native_submit() {
        use crate::resource::buffer::BufferDesc;
        use crate::rg::vulkan::{
            AccessTable, ResourceSpec, VulkanGraphBackend, VulkanGraphSession,
        };
        let Some(device) = device() else {
            eprintln!("SKIP: no Vulkan device");
            return;
        };
        let uploads = released_uploads(&device, 8, vk::BufferUsageFlags::TRANSFER_DST);
        let manager = TransferManager::new(device.clone(), TransferConfig::default()).unwrap();
        assert_eq!(manager.pending_acquire_count(), 8);
        assert_eq!(complete_synthetic_acquire_metadata(&device), 8);
        assert_eq!(manager.pending_acquire_count(), 0);
        let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
        let mut session = VulkanGraphSession::new(backend);
        let before = submitted(&device);
        let mut graph = session.begin_frame().unwrap();
        let scratch = session
            .backend
            .declare(
                &mut graph,
                "scratch",
                ResourceSpec::Buffer(BufferDesc::new_gpu_only(
                    64,
                    vk::BufferUsageFlags::TRANSFER_DST,
                )),
            )
            .unwrap();
        let mut pass = graph.add_pass("fill").with_side_effect();
        pass.add_resource_access(scratch, 0x34, 0).unwrap();
        pass.render(|_| true);
        let receipt = session.execute_frame(graph.build()).unwrap();
        assert_eq!(receipt.stats.coalesced_acquires, 0);
        assert_eq!(
            submitted(&device) - before,
            u64::from(receipt.stats.batches),
            "the native same-family graph costs only its own submits"
        );
        assert_eq!(manager.pending_acquire_count(), 0);
        session.wait_submits_done(10_000_000_000).unwrap();
        for (buffer, _) in &uploads {
            assert_eq!(owner(buffer), RangeOwner::Owned(device.queues.primary_family));
        }
    }

    #[test]
    fn synthetic_cpu_batch_and_empty_same_family_flush_have_separate_submit_counts() {
        let Some(device) = device() else {
            eprintln!("SKIP: no Vulkan device");
            return;
        };
        let mut manager =
            TransferManager::new(device.clone(), TransferConfig::default()).unwrap();
        assert_eq!(manager.flush_acquires().unwrap(), None);
        let uploads = released_uploads(&device, 6, vk::BufferUsageFlags::TRANSFER_DST);
        // A raw command touching a released range fails validation
        // instead of racing the release.
        let pool =
            crate::command::command_buffer::VulkanCommandPool::new_for_family(device.clone(), device.queues.primary_family)
                .unwrap();
        let mut command = pool.allocate().unwrap();
        command.begin().unwrap();
        let error = crate::command::resource_access::buffer_access_borrowed(
            &mut command,
            &uploads[0].0,
            GRAPHICS,
            0,
            256,
            AccessScope {
                stages: vk::PipelineStageFlags::TRANSFER,
                access: vk::AccessFlags::TRANSFER_READ,
                write: false,
            },
        )
        .unwrap_err();
        assert!(error.to_string().contains("pending acquire"), "{error}");
        drop(command);
        let before = submitted(&device);
        assert_eq!(complete_synthetic_acquire_metadata(&device), 6);
        assert_eq!(manager.flush_acquires().unwrap(), None,
            "native same-family flush must not issue synthetic barriers");
        assert_eq!(submitted(&device), before + 1);
        assert_eq!(manager.pending_acquire_count(), 0);

        for (buffer, _) in &uploads {
            assert_eq!(owner(buffer), RangeOwner::Owned(device.queues.primary_family));
        }
        assert_eq!(manager.flush_acquires().unwrap(), None);
        assert_eq!(submitted(&device), before + 1);
    }

    #[test]
    fn cpu_acquires_precede_legal_same_family_upload_and_readback() {
        let Some(device) = device() else {
            eprintln!("SKIP: no Vulkan device");
            return;
        };
        let usage = vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC;
        let uploads = released_uploads(&device, 2, usage);
        let mut manager =
            TransferManager::new(device.clone(), TransferConfig::default()).unwrap();
        assert_eq!(manager.pending_acquire_count(), 2);
        assert_eq!(complete_synthetic_acquire_metadata(&device), 2);
        // The synthetic protocol has been checked without native barriers.
        // Real copies now operate on legal same-family resources.
        let accepted = manager.enqueue_upload(uploads[0].0.clone(), 16, &[7; 32], UploadOptions {
            key: UploadKey(0), epoch: 0, priority: UploadPriority::High,
        }).unwrap();
        manager.flush_uploads(1).unwrap();
        let ticket = accepted.try_submitted().unwrap().unwrap();
        assert_eq!(manager.pending_acquire_count(), 0);
        ticket.wait(10_000_000_000).unwrap();
        for (buffer, _) in &uploads {
            assert_eq!(owner(buffer), RangeOwner::Owned(device.queues.primary_family));
        }
        let readback = manager.read_buffer(&uploads[0].0.clone(), 16, 32).unwrap();
        readback.wait(10_000_000_000).unwrap();
        let mut bytes = [0; 32];
        assert_eq!(readback.try_read(&mut bytes).unwrap(), Some(32));
        assert_eq!(bytes, [7; 32]);
        // A readback of a released source acquires it the same way.
        let released = released_uploads(&device, 1, usage);
        assert_eq!(complete_synthetic_acquire_metadata(&device), 1);
        let readback = manager.read_buffer(&released[0].0.clone(), 0, 16).unwrap();
        assert_eq!(manager.pending_acquire_count(), 0);
        readback.wait(10_000_000_000).unwrap();
        assert_eq!(owner(&released[0].0), RangeOwner::Owned(device.queues.primary_family));
    }
}

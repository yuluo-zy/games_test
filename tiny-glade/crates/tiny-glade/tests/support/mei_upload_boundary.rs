//! Synthetic upload protocol regression; these pixels are not an original-game oracle.
use super::*;
use rhapsody::rg::{AccessTable, GpuResource, VulkanGraphBackend};

#[test]
fn producer_glyph_upload_cannot_mutate_an_active_render_atlas() {
    let device = match Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
        Ok(device) => device,
        Err(error) => {
            assert!(std::env::var("RHAPSODY_REQUIRE_VULKAN").as_deref() != Ok("1"),
                "strict GPU initialization failed: {error}");
            eprintln!("SKIP atlas upload GPU initialization: {error}");
            return;
        }
    };
    let buffers = Arc::new(BufferHost::new(device.clone()));
    let textures = Arc::new(TextureHost::new(device.clone()));
    let mut host = MeiHost::new(device.clone(), buffers, textures.clone()).unwrap();
    let atlas = AtlasDescriptor { width: 16, height: 16, generation: 1 };
    let batch = |x| RenderBatch {
        glyph_uploads: vec![tg_text::PendingGlyphUpload {
            x, y: 0, width: 1, height: 1, rgba: vec![255; 4], generation: 1,
        }],
        ..Default::default()
    };
    host.upload_batch(&batch(0), atlas).unwrap();
    let native = textures.get(&host.atlas.as_ref().unwrap().1).unwrap().shared();
    let mut backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    let token = backend.bind_import(GpuResource::Image(native)).unwrap();
    let revisions = || backend.resource_native_state(token,
        |cells| cells.iter().map(|cell| cell.state.revision).collect::<Vec<_>>()).unwrap();
    let before = revisions();
    let read = textures.begin_frame_read(std::thread::current().id());
    let (sender, receiver) = std::sync::mpsc::channel();
    let producer = std::thread::spawn(move || {
        host.upload_batch(&batch(1), atlas).unwrap();
        sender.send(()).unwrap();
        host
    });
    receiver.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
    let _host = producer.join().unwrap();
    assert_eq!(revisions(), before, "producer changed the image after the frame acquired its read lease");
    drop(read);
    assert_ne!(revisions(), before, "accepted glyph upload must execute when the frame releases its read lease");
}

#[test]
fn rejected_glyph_admission_retains_the_consumed_frame_until_capacity_returns() {
    let device = match Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
        Ok(device) => device,
        Err(error) => {
            assert!(std::env::var("RHAPSODY_REQUIRE_VULKAN").as_deref() != Ok("1"), "{error}");
            return;
        }
    };
    let buffers = Arc::new(BufferHost::new(device.clone()));
    let textures = Arc::new(TextureHost::new(device.clone()));
    let mut host = MeiHost::new(device, buffers, textures.clone()).unwrap();
    let atlas = AtlasDescriptor { width: 16, height: 16, generation: 1 };
    host.upload_batch(&RenderBatch::default(), atlas).unwrap();
    let batch = RenderBatch {
        glyph_uploads: vec![tg_text::PendingGlyphUpload {
            x: 1, y: 1, width: 1, height: 1, rgba: vec![17, 31, 47, 255], generation: 1,
        }],
        ..Default::default()
    };
    host.validate_batch(&batch, atlas, [16, 16], 0).unwrap();
    host.pending = Some(PendingTextFrame { atlas, batch });
    let pressure = textures.create_texture("deferred release pressure",
        &TextureDesc::new_2d([1, 1], 0x2b).sampled(), &[]).unwrap();
    let read = textures.begin_frame_read(std::thread::current().id());
    let writer = textures.clone();
    let (send, receive) = std::sync::mpsc::channel();
    let producer = std::thread::spawn(move || {
        // Fill the existing job limit through its real release API. All
        // requests name an unrelated live owner until this lease opens.
        for _ in 0..4096 { assert!(writer.release(&pressure)); }
        assert!(!host.poll_pending_uploads().unwrap());
        send.send(()).unwrap();
        host
    });
    receive.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
    let mut host = producer.join().unwrap();
    assert_eq!(host.pending.as_ref().unwrap().batch.glyph_uploads[0].rgba, [17, 31, 47, 255]);
    assert!(host.output().is_none(), "a rejected text frame must not publish its output");
    drop(read);
    assert!(textures.poll_pending_uploads().unwrap());
    assert!(host.poll_pending_uploads().unwrap());
    assert!(host.pending.is_none());
    assert!(host.output().is_some(), "the retained frame must publish without a second text consume");
}

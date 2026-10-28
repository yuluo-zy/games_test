//! Explicit native view usage, original packed radiance and attachment identity.
use ash::vk;
use rhapsody::{bindless::{BindlessConfig, BindlessHeap}, device::{Device, DeviceConfig}, rg::attachments::{Attachment, AttachmentCache, PreparedAttachments, RenderingPolicy}};
use rhapsody::resource::texture::{Texture, TextureDesc, TextureViewDesc};
use rhapsody::native::instance::{InstanceBuilder, ValidationMessenger};
use std::sync::Arc;

fn fixture() -> rhapsody::Result<(Arc<Device>, ValidationMessenger)> {
    let mut instance = InstanceBuilder::default();
    instance.debug_markers();
    instance
        .layers
        .push(c"VK_LAYER_KHRONOS_validation".to_owned());
    let device = Device::new(
        &instance.build()?,
        None,
        &DeviceConfig {
            validation: true,
            ..DeviceConfig::headless()
        },
    )?;
    let messages = ValidationMessenger::new(device.instance())?;
    Ok((device, messages))
}

fn clean(device: &Device, messages: &ValidationMessenger) -> rhapsody::Result<()> {
    device.wait_idle_and_collect()?;
    assert!(
        messages.collector.errors().is_empty(),
        "{:?}",
        messages.collector.errors()
    );
    assert!(messages.collector.all().iter().all(|message|
        !message.id_name.contains("VUID-") && !message.message.contains("VUID-")),
        "{:?}", messages.collector.all());
    Ok(())
}

#[test]
fn view_usage_is_an_explicit_nonempty_subset_without_changing_image_purposes() {
    let texture = TextureDesc::new_2d(8, 8, vk::Format::R32_UINT)
        .sampled_with_different_format()
        .storage()
        .color_attachment();
    assert_eq!(
        texture.flags,
        vk::ImageCreateFlags::MUTABLE_FORMAT | vk::ImageCreateFlags::EXTENDED_USAGE
    );
    assert_eq!(texture.flags.as_raw(), 0x108);
    let original = texture;
    let inherited = TextureViewDesc::new(&texture).unwrap();
    assert_eq!(inherited.usage, texture.usage);
    assert!(inherited.validate(&texture).is_ok());
    let mut sampled = inherited;
    sampled.format = vk::Format::E5B9G9R9_UFLOAT_PACK32;
    sampled.usage = vk::ImageUsageFlags::SAMPLED;
    assert!(sampled.validate(&texture).is_ok());
    for usage in [
        vk::ImageUsageFlags::empty(),
        vk::ImageUsageFlags::TRANSFER_DST,
        vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
    ] {
        let mut bad = sampled;
        bad.usage = usage;
        assert_eq!(
            bad.validate(&texture).unwrap_err().kind(),
            rhapsody::ErrorKind::InvalidUsage
        );
    }
    assert_eq!(
        texture, original,
        "view validation must preserve all real image usages"
    );
}

#[test]
fn native_packed_radiance_sampling_keeps_real_r32_storage_and_color_owners() -> rhapsody::Result<()>
{
    let (device, messages) = fixture()?;
    let heap = BindlessHeap::new(
        device.clone(),
        BindlessConfig {
            sampled_images: 4,
            storage_images: 4,
            storage_buffers: 4,
            samplers: 4,
        },
    )?;
    for extra in [
        vk::ImageUsageFlags::COLOR_ATTACHMENT,
        vk::ImageUsageFlags::STORAGE,
    ] {
        let desc = TextureDesc::new_2d(16, 16, vk::Format::R32_UINT)
            .sampled_with_different_format()
            .usage(vk::ImageUsageFlags::SAMPLED | extra);
        let texture = device.create_texture(&desc, "original ice radiance", ())?;
        let base = texture.view(None)?;
        assert_eq!(base.view.desc.format, vk::Format::R32_UINT);
        assert_eq!(base.view.desc.usage, vk::ImageUsageFlags::SAMPLED | extra);
        let mut sampled_desc = TextureViewDesc::new(&desc)?;
        sampled_desc.format = vk::Format::E5B9G9R9_UFLOAT_PACK32;
        sampled_desc.usage = vk::ImageUsageFlags::SAMPLED;
        let sampled = texture.view(Some(&sampled_desc))?;
        let again = texture.view(Some(&sampled_desc))?;
        assert!(Arc::ptr_eq(&sampled.view, &again.view));
        assert_ne!(base.raw(), sampled.raw());
        assert_eq!(sampled.view.desc.usage, vk::ImageUsageFlags::SAMPLED);
        assert_eq!(texture.desc.usage, vk::ImageUsageFlags::SAMPLED | extra);
        let sampled_lease = heap.sampled_image(
            sampled.view.clone(),
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )?;
        assert!(
            matches!(
                heap.storage_image(sampled.view.clone()),
                Err(rhapsody::Error::InvalidUsage { .. })
            ),
            "a sampled-only view must not be usable as a storage descriptor"
        );
        if extra == vk::ImageUsageFlags::STORAGE {
            let storage_lease = heap.storage_image(base.view.clone())?;
            drop(storage_lease);
        } else {
            let mut attachments = AttachmentCache::new(device.clone())?;
            let prepared = attachments.prepare_for_rendering(
                &[Attachment::color(texture.shared(), 0, Some([0.0; 4]))?],
                vk::Extent2D {
                    width: 16,
                    height: 16,
                },
                1,
                RenderingPolicy::RequireDynamic,
            )?;
            let PreparedAttachments::Dynamic(prepared) = &*prepared;
            assert!(
                prepared.views[0]
                    .desc
                    .usage
                    .contains(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            );
        }
        drop(sampled_lease);
    }
    drop(heap);
    clean(&device, &messages)
}

#[test]
fn attachment_cache_separates_view_usage_and_rejects_sampling_only_color_and_depth()
-> rhapsody::Result<()> {
    let (device, messages) = fixture()?;
    for depth in [false, true] {
        let required = if depth {
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT
        } else {
            vk::ImageUsageFlags::COLOR_ATTACHMENT
        };
        let format = if depth {
            vk::Format::D32_SFLOAT
        } else {
            vk::Format::R8G8B8A8_UNORM
        };
        let desc = TextureDesc::new_2d(8, 8, format).usage(vk::ImageUsageFlags::SAMPLED | required);
        let texture = Arc::new(Texture::new(device.clone(), &desc)?);
        let full = if depth {
            Attachment::depth(texture.clone(), 0, Some(1.0))?
        } else {
            Attachment::color(texture.clone(), 0, Some([0.0; 4]))?
        };
        let mut narrow = full.clone();
        narrow.view.usage = required;
        let mut cache = AttachmentCache::new(device.clone())?;
        let extent = vk::Extent2D {
            width: 8,
            height: 8,
        };
        let first = cache.prepare_for_rendering(
            &[full.clone()],
            extent,
            1,
            RenderingPolicy::RequireDynamic,
        )?;
        let second = cache.prepare_for_rendering(
            &[narrow.clone()],
            extent,
            1,
            RenderingPolicy::RequireDynamic,
        )?;
        let again =
            cache.prepare_for_rendering(&[narrow], extent, 1, RenderingPolicy::RequireDynamic)?;
        let (
            PreparedAttachments::Dynamic(first),
            PreparedAttachments::Dynamic(second),
            PreparedAttachments::Dynamic(again),
        ) = (&*first, &*second, &*again);
        assert!(
            !Arc::ptr_eq(&first.views[0], &second.views[0]),
            "different native view usages cannot alias"
        );
        assert!(Arc::ptr_eq(&second.views[0], &again.views[0]));
        assert_eq!(
            first.views[0].desc.usage,
            vk::ImageUsageFlags::SAMPLED | required
        );
        assert_eq!(second.views[0].desc.usage, required);
        assert_eq!(cache.view_count(), 2);
        let mut sampled = full;
        sampled.view.usage = vk::ImageUsageFlags::SAMPLED;
        assert!(matches!(
            cache.prepare_for_rendering(&[sampled], extent, 1, RenderingPolicy::RequireDynamic),
            Err(rhapsody::Error::InvalidUsage { .. })
        ));
        assert_eq!(
            cache.view_count(),
            2,
            "rejected attachment must not create or publish a view"
        );
        assert_eq!(texture.desc.usage, vk::ImageUsageFlags::SAMPLED | required);
    }
    clean(&device, &messages)
}

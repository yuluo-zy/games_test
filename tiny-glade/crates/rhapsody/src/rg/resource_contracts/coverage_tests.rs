use super::*;
fn next(seed: &mut u32) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    *seed
}
#[test]
fn interval_union_matches_independent_byte_cells_for_gaps_and_overlaps() {
    let descriptor = super::super::graph::Descriptor::Buffer(crate::rg::resource_registry::BufferDesc {
        word_at_00: 8,
        word_at_08: 0,
        bytes_at_0c_through_0e: [0; 3],
    });
    let mut seed = 0x91e1_0da5;
    for _ in 0..4096 {
        let mut writes = [AccessRange::Whole; 6];
        for range in &mut writes {
            let offset = u64::from(next(&mut seed) % 8);
            *range = AccessRange::Buffer {
                offset,
                size: 1 + u64::from(next(&mut seed)) % (8 - offset),
            };
        }
        let offset = u64::from(next(&mut seed) % 8);
        let size = 1 + u64::from(next(&mut seed)) % (8 - offset);
        let expected = (offset..offset + size).all(|byte| {
            writes.iter().any(|range| {
                let AccessRange::Buffer { offset, size } = *range else {
                    unreachable!()
                };
                offset <= byte && byte < offset + size
            })
        });
        assert_eq!(
            covers(&writes, AccessRange::Buffer { offset, size }, descriptor),
            Ok(expected)
        );
        writes.reverse();
        assert_eq!(
            covers(&writes, AccessRange::Buffer { offset, size }, descriptor),
            Ok(expected)
        );
    }
}
#[test]
fn rectangle_union_matches_independent_aspect_mip_layer_cells() {
    let descriptor = super::super::vulkan::ResourceSpec::Image(
        crate::resource::texture::TextureDesc::new_2d_array(
            16,
            16,
            3,
            ash::vk::Format::D24_UNORM_S8_UINT,
        )
        .mip_levels(3),
    )
    .descriptor();
    let mut seed = 0x71a3_c9e7;
    let mut random_range = || {
        let aspect_mask = 2 * (1 + next(&mut seed) % 3);
        let base_mip_level = next(&mut seed) % 3;
        let level_count = 1 + next(&mut seed) % (3 - base_mip_level);
        let base_array_layer = next(&mut seed) % 3;
        let layer_count = 1 + next(&mut seed) % (3 - base_array_layer);
        AccessRange::Texture {
            aspect_mask,
            base_mip_level,
            level_count,
            base_array_layer,
            layer_count,
        }
    };
    for _ in 0..4096 {
        let writes = std::array::from_fn::<_, 6, _>(|_| random_range());
        let read = random_range();
        let AccessRange::Texture {
            aspect_mask,
            base_mip_level,
            level_count,
            base_array_layer,
            layer_count,
        } = read
        else {
            unreachable!()
        };
        let expected = [2, 4]
            .into_iter()
            .filter(|bit| bit & aspect_mask != 0)
            .all(|bit| {
                (base_mip_level..base_mip_level + level_count).all(|mip| {
                    (base_array_layer..base_array_layer + layer_count).all(|layer| {
                        writes.iter().any(|range| {
                            let AccessRange::Texture {
                                aspect_mask,
                                base_mip_level,
                                level_count,
                                base_array_layer,
                                layer_count,
                            } = *range
                            else {
                                unreachable!()
                            };
                            aspect_mask & bit != 0
                                && base_mip_level <= mip
                                && mip < base_mip_level + level_count
                                && base_array_layer <= layer
                                && layer < base_array_layer + layer_count
                        })
                    })
                })
            });
        assert_eq!(covers(&writes, read, descriptor), Ok(expected));
        assert_eq!(
            covers_iter(writes.into_iter().rev(), read, descriptor),
            Ok(expected)
        );
    }
}

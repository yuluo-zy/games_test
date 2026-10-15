//! 第三批 rg 收口：设备函数槽与 ash 函数表一致、`validate_read_access` 空操作、PGO 的 insert 语义。
use ash::vk;
use rhapsody::rg::executor::{PassPgoDuration, PassPgoFrames, PassPgoNameId, PassPgoStats};
use rhapsody::rg::pass_api::{original_slots as slot, Command};
use std::mem::{offset_of, size_of};

const BASE: usize = slot::ASH_TABLE_BASE;

#[test]
fn original_device_slots_equal_ash_table_offsets() {
    // 槽 = 0x78 + ash::DeviceFnV1_0 字段偏移（原版内嵌 ash 函数表）。
    let v10 = |off: usize| BASE + off;
    assert_eq!(slot::SET_VIEWPORT, v10(offset_of!(ash::DeviceFnV1_0, cmd_set_viewport)));
    assert_eq!(slot::SET_SCISSOR, v10(offset_of!(ash::DeviceFnV1_0, cmd_set_scissor)));
    assert_eq!(slot::SET_LINE_WIDTH, v10(offset_of!(ash::DeviceFnV1_0, cmd_set_line_width)));
    assert_eq!(slot::SET_STENCIL_REFERENCE, v10(offset_of!(ash::DeviceFnV1_0, cmd_set_stencil_reference)));
    assert_eq!(slot::BIND_DESCRIPTOR_SETS, v10(offset_of!(ash::DeviceFnV1_0, cmd_bind_descriptor_sets)));
    assert_eq!(slot::BIND_INDEX_BUFFER, v10(offset_of!(ash::DeviceFnV1_0, cmd_bind_index_buffer)));
    assert_eq!(slot::BIND_VERTEX_BUFFERS, v10(offset_of!(ash::DeviceFnV1_0, cmd_bind_vertex_buffers)));
    assert_eq!(slot::DRAW, v10(offset_of!(ash::DeviceFnV1_0, cmd_draw)));
    assert_eq!(slot::DRAW_INDEXED, v10(offset_of!(ash::DeviceFnV1_0, cmd_draw_indexed)));
    assert_eq!(slot::DRAW_INDIRECT, v10(offset_of!(ash::DeviceFnV1_0, cmd_draw_indirect)));
    assert_eq!(slot::DRAW_INDEXED_INDIRECT, v10(offset_of!(ash::DeviceFnV1_0, cmd_draw_indexed_indirect)));
    assert_eq!(slot::DISPATCH, v10(offset_of!(ash::DeviceFnV1_0, cmd_dispatch)));
    assert_eq!(slot::DISPATCH_INDIRECT, v10(offset_of!(ash::DeviceFnV1_0, cmd_dispatch_indirect)));
    assert_eq!(slot::COPY_BUFFER, v10(offset_of!(ash::DeviceFnV1_0, cmd_copy_buffer)));
    assert_eq!(slot::FILL_BUFFER, v10(offset_of!(ash::DeviceFnV1_0, cmd_fill_buffer)));
    assert_eq!(slot::CLEAR_COLOR_IMAGE, v10(offset_of!(ash::DeviceFnV1_0, cmd_clear_color_image)));
    assert_eq!(slot::PIPELINE_BARRIER, v10(offset_of!(ash::DeviceFnV1_0, cmd_pipeline_barrier)));
    assert_eq!(slot::PUSH_CONSTANTS, v10(offset_of!(ash::DeviceFnV1_0, cmd_push_constants)));
    assert_eq!(slot::BEGIN_RENDER_PASS, v10(offset_of!(ash::DeviceFnV1_0, cmd_begin_render_pass)));
    assert_eq!(slot::END_RENDER_PASS, v10(offset_of!(ash::DeviceFnV1_0, cmd_end_render_pass)));
    // V1_2 紧跟 V1_0、V1_1。
    let v12 = BASE + size_of::<ash::DeviceFnV1_0>() + size_of::<ash::DeviceFnV1_1>();
    assert_eq!(slot::DRAW_INDIRECT_COUNT, v12 + offset_of!(ash::DeviceFnV1_2, cmd_draw_indirect_count));
    assert_eq!(slot::DRAW_INDEXED_INDIRECT_COUNT, v12 + offset_of!(ash::DeviceFnV1_2, cmd_draw_indexed_indirect_count));
}

#[test]
fn every_command_maps_to_a_distinct_slot_except_none() {
    let b = vk::Buffer::null();
    let cmds = [
        Command::EndRenderPass,
        Command::SetStencilRef(1),
        Command::SetLineWidth(1.0),
        Command::BindIndexBuffer { buffer: b, ty: vk::IndexType::UINT32 },
        Command::Draw { vertex_count: 1, instance_count: 1, first_vertex: 0, first_instance: 0 },
        Command::DrawIndexed { index_count: 1, instance_count: 1, first_index: 0, vertex_offset: 0, first_instance: 0 },
        Command::DrawIndirect { buffer: b, offset: 0, draw_count: 1, stride: 16 },
        Command::DrawIndexedIndirect { buffer: b, offset: 0, draw_count: 1, stride: 20 },
        Command::DispatchGroups { x: 1, y: 1, z: 1 },
        Command::DispatchIndirect { buffer: b, offset: 0 },
    ];
    let mut slots: Vec<usize> = cmds.iter().map(slot::slot).collect();
    slots.sort_unstable();
    slots.dedup();
    assert_eq!(slots.len(), cmds.len());
}

#[test]
fn validate_read_access_is_a_noop_like_the_original_ret() {
    // 原版 Texture/Buffer 两个 `Resource::validate_read_access`（PDB 符号）ICF 折叠到同一个只有 `ret` 的
    // 0x14003bb80；Rust 侧同样不得有副作用也不得 panic，对所有访问码都成立。
    for access in 0..=u8::MAX {
        rhapsody::rg::resource::validate_read_access(access);
    }
}

#[test]
fn pgo_current_is_cleared_but_seed_free_and_extend_matches_insert_semantics() {
    let id = PassPgoNameId::from_name("w3_pass");
    let other = PassPgoNameId::from_name("w3_other");
    let mut frames = PassPgoFrames::default();
    frames.current.extend([
        PassPgoDuration::new(id, 40),
        PassPgoDuration::new(other, 10),
        PassPgoDuration::new(id, 25),
    ]);
    assert_eq!(frames.current.get(id), Some(25), "insert 覆盖");
    assert_eq!(frames.current.total_ns(), 75);
    assert_eq!(frames.current.max_ns(), 40);
    frames.end_frame();
    assert_eq!(frames.previous.get(id), Some(25));
    assert_eq!(frames.previous.total_ns(), 75);
    assert!(frames.current.is_empty());
    assert_eq!((frames.current.total_ns(), frames.current.max_ns()), (0, 0));
    let mut s = PassPgoStats::default();
    s.record(PassPgoDuration::new(id, u64::MAX));
    s.record(PassPgoDuration::new(id, 2));
    assert_eq!(s.total_ns(), 1, "原版是普通 add，回绕而非饱和");
}

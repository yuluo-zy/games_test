140a78aa0: 55 push rbp
140a78aa1: 4157 push r15
140a78aa3: 4156 push r14
140a78aa5: 4155 push r13
140a78aa7: 4154 push r12
140a78aa9: 56 push rsi
140a78aaa: 57 push rdi
140a78aab: 53 push rbx
140a78aac: 4881ecd8020000 sub rsp, 0x2d8
140a78ab3: 488dac2480000000 lea rbp, [rsp + 0x80]
140a78abb: 660f7fbd40020000 movdqa xmmword ptr [rbp + 0x240], xmm7
140a78ac3: 660f7fb530020000 movdqa xmmword ptr [rbp + 0x230], xmm6
140a78acb: 48c78528020000feffffff mov qword ptr [rbp + 0x228], 0xfffffffffffffffe
140a78ad6: 4c89cf mov rdi, r9
140a78ad9: 4c898548010000 mov qword ptr [rbp + 0x148], r8
140a78ae0: 4889d3 mov rbx, rdx
140a78ae3: 4889ce mov rsi, rcx
140a78ae6: 488b05bb38bb02 mov rax, qword ptr [rip + 0x2bb38bb]
140a78aed: 488b00 mov rax, qword ptr [rax]
140a78af0: 4883f803 cmp rax, 3
140a78af4: 0f86ba000000 jbe 0x140a78bb4
140a78afa: 488d0d3fe20502 lea rcx, [rip + 0x205e23f]
140a78b01: e87a52c0ff call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut | toolbar::mei::widgets::ActionHintView::as_inner_mut | system_decorator::decorator_visual_state::DecoratorPersistentVisualState::entities | country_core::resources::walls::decorator::DecoratorId::new | country_core::resources::walls::decorator::DeferredDecoratorOpEvents::events | country_core::resources::world_raster_defs::impl$45::deref_mut | country_core::resources::world_raster_defs::impl$69::deref | country_core::resources::image_formation::ImageFormationConfigs::initial | country_core::resources::rt::RtBvhBuilderGlueResource::new | country_core::systems::collision::world::SubworldIndex::from_usize_unchecked | country_core::systems::ui::resources::ui_state::UiState::widget | country_core::resources::gui::impl$0::deref | country_core::resources::gui::PendingEguiContextResource::into_egui_context_resource | country_core::render::impl$1::deref | country_core::systems::steam::AppSteamIntegration::client | image::traits::impl$0::as_bytes | rhapsody::raw::buffer::impl$0::as_managed_handle | rhapsody::raw::shader::impl$2::deref | rhapsody::raw::shader::impl$3::deref | rhapsody::rg::executor::PassPgoNameId::from_name | rhapsody::raw::allocator::GpuAllocator::inner | gpu_profiler::shared::NanoSecond::from_raw_ns | utils::swizzlers::impl$5::as_vec2 | log::__private_api::loc | country_core::resources::walls::wall_holes::FinalizedHoleStorage::inner | utils::geometry::intersect::Triangle::points_mut | asset::prefab::impl$4::as_ref | country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo> | country_core::asset_libraries::ssbo_library::impl$0::buffer<country_core::geometry::instanced_wall::BrickDrawListSsbo> | parry3d::shape::trimesh::impl$8::qbvh | parry3d::shape::trimesh::impl$8::qbvh | bevy_ptr::dangling_with_align | event_listener::notify::impl$33::into_notification | flume::signal::impl$1::as_ptr | cpp_demangle::ast::impl$133::downcast_to_pointer_to_member<alloc::string::String> | cpp_demangle::ast::impl$93::downcast_to_type<alloc::string::String> | cpp_demangle::ast::impl$127::downcast_to_array_type<alloc::string::String> | motomoto::Reader::new | country_core::asset_libraries::ssbo_library::impl$0::buffer<country_core::resources::color_id::GateColorIdSsbo> | country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
140a78b06: f30f6f00 movdqu xmm0, xmmword ptr [rax]
140a78b0a: 8b4010 mov eax, dword ptr [rax + 0x10]
140a78b0d: 48c785b001000004000000 mov qword ptr [rbp + 0x1b0], 4
140a78b18: 488d0d39e20502 lea rcx, [rip + 0x205e239] ; @142ad6d58 'country_core::startup::clearingassets/meshes/ice_cracks.json'
140a78b1f: 48898db8010000 mov qword ptr [rbp + 0x1b8], rcx
140a78b26: 48c785c00100001f000000 mov qword ptr [rbp + 0x1c0], 0x1f
140a78b31: 488d15c8e10502 lea rdx, [rip + 0x205e1c8]
140a78b38: 488995d0010000 mov qword ptr [rbp + 0x1d0], rdx
140a78b3f: 48c785d801000001000000 mov qword ptr [rbp + 0x1d8], 1
140a78b4a: 48c785e001000008000000 mov qword ptr [rbp + 0x1e0], 8
140a78b55: 48c7858001000000000000 mov qword ptr [rbp + 0x180], 0
140a78b60: 660fefc9 pxor xmm1, xmm1
140a78b64: f30f7f8de8010000 movdqu xmmword ptr [rbp + 0x1e8], xmm1
140a78b6c: 48898d88010000 mov qword ptr [rbp + 0x188], rcx
140a78b73: 48c785900100001f000000 mov qword ptr [rbp + 0x190], 0x1f
140a78b7e: 48c7859801000000000000 mov qword ptr [rbp + 0x198], 0
140a78b89: f30f7f85a0010000 movdqu xmmword ptr [rbp + 0x1a0], xmm0
140a78b91: c785c801000001000000 mov dword ptr [rbp + 0x1c8], 1
140a78b9b: 8985cc010000 mov dword ptr [rbp + 0x1cc], eax
140a78ba1: 488d8db0000000 lea rcx, [rbp + 0xb0]
140a78ba8: 488d9580010000 lea rdx, [rbp + 0x180]
140a78baf: e87c172d00 call 0x140d4a330 ; log::__private_api::impl$2::log
140a78bb4: 4c8badd0020000 mov r13, qword ptr [rbp + 0x2d0]
140a78bbb: 488b06 mov rax, qword ptr [rsi]
140a78bbe: 488b00 mov rax, qword ptr [rax]
140a78bc1: 4883c010 add rax, 0x10
140a78bc5: 48898500020000 mov qword ptr [rbp + 0x200], rax
140a78bcc: 488b07 mov rax, qword ptr [rdi]
140a78bcf: 0fb64058 movzx eax, byte ptr [rax + 0x58]
140a78bd3: 3c04 cmp al, 4
140a78bd5: 4889bda8000000 mov qword ptr [rbp + 0xa8], rdi
140a78bdc: 0f85d3010000 jne 0x140a78db5
140a78be2: 488d158ee10502 lea rdx, [rip + 0x205e18e] ; @142ad6d77 'assets/meshes/ice_cracks.json'
140a78be9: 488d8d80010000 lea rcx, [rbp + 0x180]
140a78bf0: 41b81d000000 mov r8d, 0x1d
140a78bf6: e8b5620900 call 0x140b0eeb0 ; country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
140a78bfb: 31c0 xor eax, eax
140a78bfd: 483b8580010000 cmp rax, qword ptr [rbp + 0x180]
140a78c04: 0f80e20f0000 jo 0x140a79bec
140a78c0a: 488b85f0010000 mov rax, qword ptr [rbp + 0x1f0]
140a78c11: 48898520010000 mov qword ptr [rbp + 0x120], rax
140a78c18: 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
140a78c1f: 0f298510010000 movaps xmmword ptr [rbp + 0x110], xmm0
140a78c26: 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
140a78c2d: 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
140a78c34: 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
140a78c3b: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
140a78c42: 0f108580010000 movups xmm0, xmmword ptr [rbp + 0x180]
140a78c49: f30f6f8d90010000 movdqu xmm1, xmmword ptr [rbp + 0x190]
140a78c51: 0f1095a0010000 movups xmm2, xmmword ptr [rbp + 0x1a0]
140a78c58: 0f109db0010000 movups xmm3, xmmword ptr [rbp + 0x1b0]
140a78c5f: 0f299de0000000 movaps xmmword ptr [rbp + 0xe0], xmm3
140a78c66: 0f2995d0000000 movaps xmmword ptr [rbp + 0xd0], xmm2
140a78c6d: 660f7f8dc0000000 movdqa xmmword ptr [rbp + 0xc0], xmm1
140a78c75: 0f2985b0000000 movaps xmmword ptr [rbp + 0xb0], xmm0
140a78c7c: 488b8548010000 mov rax, qword ptr [rbp + 0x148]
140a78c83: 4c8b08 mov r9, qword ptr [rax]
140a78c86: 4989dc mov r12, rbx
140a78c89: 488b03 mov rax, qword ptr [rbx]
140a78c8c: 4889442420 mov qword ptr [rsp + 0x20], rax
140a78c91: 488d3d18e10502 lea rdi, [rip + 0x205e118] ; @142ad6db0 'ice_cracks/water/ice_cracks.raster.hlsl'
140a78c98: 488d8db0000000 lea rcx, [rbp + 0xb0]
140a78c9f: 41b80a000000 mov r8d, 0xa
140a78ca5: 4889fa mov rdx, rdi
140a78ca8: e8f31f0a00 call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
140a78cad: 4889c6 mov rsi, rax
140a78cb0: 488b85c8020000 mov rax, qword ptr [rbp + 0x2c8]
140a78cb7: 488b00 mov rax, qword ptr [rax]
140a78cba: 488b8dc0020000 mov rcx, qword ptr [rbp + 0x2c0]
140a78cc1: 488b09 mov rcx, qword ptr [rcx]
140a78cc4: 48894c2430 mov qword ptr [rsp + 0x30], rcx
140a78cc9: 4889442428 mov qword ptr [rsp + 0x28], rax
140a78cce: 48c74424200a000000 mov qword ptr [rsp + 0x20], 0xa
140a78cd7: 488d15dce00502 lea rdx, [rip + 0x205e0dc] ; @142ad6dba '/water/ice_cracks.raster.hlsl'
140a78cde: 41b81d000000 mov r8d, 0x1d
140a78ce4: 488b8d00020000 mov rcx, qword ptr [rbp + 0x200]
140a78ceb: 4989f9 mov r9, rdi
140a78cee: e84d710c00 call 0x140b3fe40 ; country_core::startup::load_shader_into_library<ref$<str$> >
140a78cf3: 4889c7 mov rdi, rax
140a78cf6: 488d8d80010000 lea rcx, [rbp + 0x180]
140a78cfd: 4c89ea mov rdx, r13
140a78d00: e89bb92900 call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
140a78d05: 488b9d88010000 mov rbx, qword ptr [rbp + 0x188]
140a78d0c: 4885db test rbx, rbx
140a78d0f: 7507 jne 0x140a78d18
140a78d11: 488b9d90010000 mov rbx, qword ptr [rbp + 0x190]
140a78d18: 4c8bbda0010000 mov r15, qword ptr [rbp + 0x1a0]
140a78d1f: 488b03 mov rax, qword ptr [rbx]
140a78d22: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
140a78d26: 4c29f0 sub rax, r14
140a78d29: 4883f877 cmp rax, 0x77
140a78d2d: 0f869c0d0000 jbe 0x140a79acf
140a78d33: 488b4308 mov rax, qword ptr [rbx + 8]
140a78d37: 488d0d22478600 lea rcx, [rip + 0x864722]
140a78d3e: 4a890c30 mov qword ptr [rax + r14], rcx
140a78d42: 660f6f057653ec01 movdqa xmm0, xmmword ptr [rip + 0x1ec5376]
140a78d4a: f3420f7f443008 movdqu xmmword ptr [rax + r14 + 8], xmm0
140a78d51: 4ac744301800000000 mov qword ptr [rax + r14 + 0x18], 0
140a78d5a: 42c744302000000000 mov dword ptr [rax + r14 + 0x20], 0
140a78d63: 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
140a78d6d: 4a894c3024 mov qword ptr [rax + r14 + 0x24], rcx
140a78d72: 42c744302c0000803f mov dword ptr [rax + r14 + 0x2c], 0x3f800000
140a78d7b: 4a89743038 mov qword ptr [rax + r14 + 0x38], rsi
140a78d80: 4a897c3040 mov qword ptr [rax + r14 + 0x40], rdi
140a78d85: 42c644304800 mov byte ptr [rax + r14 + 0x48], 0
140a78d8b: 4e897c3058 mov qword ptr [rax + r14 + 0x58], r15
140a78d90: 488d0d99bfae00 lea rcx, [rip + 0xaebf99]
140a78d97: 4a894c3068 mov qword ptr [rax + r14 + 0x68], rcx
140a78d9c: 4983c678 add r14, 0x78
140a78da0: 4c897310 mov qword ptr [rbx + 0x10], r14
140a78da4: 488bbda8000000 mov rdi, qword ptr [rbp + 0xa8]
140a78dab: 488b07 mov rax, qword ptr [rdi]
140a78dae: 0fb64058 movzx eax, byte ptr [rax + 0x58]
140a78db2: 4c89e3 mov rbx, r12
140a78db5: 3c01 cmp al, 1
140a78db7: 745c je 0x140a78e15
140a78db9: 0fb6c0 movzx eax, al
140a78dbc: 83f802 cmp eax, 2
140a78dbf: 7536 jne 0x140a78df7
140a78dc1: 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a78dc6: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
140a78dcd: 4889442428 mov qword ptr [rsp + 0x28], rax
140a78dd2: 488b85d8020000 mov rax, qword ptr [rbp + 0x2d8]
140a78dd9: 4889442420 mov qword ptr [rsp + 0x20], rax
140a78dde: 488b8d48010000 mov rcx, qword ptr [rbp + 0x148]
140a78de5: 4889da mov rdx, rbx
140a78de8: 4d89e8 mov r8, r13
140a78deb: 4c8b8dc0020000 mov r9, qword ptr [rbp + 0x2c0]
140a78df2: e829a3ad00 call 0x141553120 ; country_core::startup::glade_themes::flowery::startup_flowery
140a78df7: 488b07 mov rax, qword ptr [rdi]
140a78dfa: 80785801 cmp byte ptr [rax + 0x58], 1
140a78dfe: 0f8580000000 jne 0x140a78e84
140a78e04: b805000000 mov eax, 5
140a78e09: 4c8d0dc6660702 lea r9, [rip + 0x20766c6] ; @142aef4d6 'oldenfloweryautumnwintersnapshotshistory.jsonscreenshot.jpgmeta.jsonassets/gladestatecrates/coun'
140a78e10: e9ce000000 jmp 0x140a78ee3
140a78e15: 488b95d8020000 mov rdx, qword ptr [rbp + 0x2d8]
140a78e1c: 8b421c mov eax, dword ptr [rdx + 0x1c]
140a78e1f: 488b4a10 mov rcx, qword ptr [rdx + 0x10]
140a78e23: 8901 mov dword ptr [rcx], eax
140a78e25: 488b12 mov rdx, qword ptr [rdx]
140a78e28: 4c8b0f mov r9, qword ptr [rdi]
140a78e2b: 488b8548010000 mov rax, qword ptr [rbp + 0x148]
140a78e32: 488b00 mov rax, qword ptr [rax]
140a78e35: 488b0b mov rcx, qword ptr [rbx]
140a78e38: 4c8b85c0020000 mov r8, qword ptr [rbp + 0x2c0]
140a78e3f: 4d8b00 mov r8, qword ptr [r8]
140a78e42: 4c89442438 mov qword ptr [rsp + 0x38], r8
140a78e47: 48894c2430 mov qword ptr [rsp + 0x30], rcx
140a78e4c: 4889442428 mov qword ptr [rsp + 0x28], rax
140a78e51: 4c896c2420 mov qword ptr [rsp + 0x20], r13
140a78e56: 488d75c0 lea rsi, [rbp - 0x40]
140a78e5a: 4889f1 mov rcx, rsi
140a78e5d: 4c8b8500020000 mov r8, qword ptr [rbp + 0x200]
140a78e64: e867abad00 call 0x1415539d0 ; country_core::startup::glade_themes::olden::startup_olden
140a78e69: 4c8d0568df0502 lea r8, [rip + 0x205df68]
140a78e70: 4c89e9 mov rcx, r13
140a78e73: 4889f2 mov rdx, rsi
140a78e76: e8356d8600 call 0x1412dfbb0 ; bevy_ecs::system::commands::Commands::insert_resource<country_core::resources::preplaced_autoclutter::PreplacedAutoClutter>
140a78e7b: 488b07 mov rax, qword ptr [rdi]
140a78e7e: 80785801 cmp byte ptr [rax + 0x58], 1
140a78e82: 7480 je 0x140a78e04
140a78e84: 498b7508 mov rsi, qword ptr [r13 + 8]
140a78e88: 4885f6 test rsi, rsi
140a78e8b: 7504 jne 0x140a78e91
140a78e8d: 498b7510 mov rsi, qword ptr [r13 + 0x10]
140a78e91: 488b06 mov rax, qword ptr [rsi]
140a78e94: 488b7e10 mov rdi, qword ptr [rsi + 0x10]
140a78e98: 4829f8 sub rax, rdi
140a78e9b: 4883f807 cmp rax, 7
140a78e9f: 0f86050c0000 jbe 0x140a79aaa
140a78ea5: 488b4608 mov rax, qword ptr [rsi + 8]
140a78ea9: 488d0d40388600 lea rcx, [rip + 0x863840]
140a78eb0: 48890c38 mov qword ptr [rax + rdi], rcx
140a78eb4: 4883c708 add rdi, 8
140a78eb8: 48897e10 mov qword ptr [rsi + 0x10], rdi
140a78ebc: 488b85a8000000 mov rax, qword ptr [rbp + 0xa8]
140a78ec3: 488b00 mov rax, qword ptr [rax]
140a78ec6: 0fb64058 movzx eax, byte ptr [rax + 0x58]
140a78eca: c1e003 shl eax, 3
140a78ecd: 488d0d5ce80502 lea rcx, [rip + 0x205e85c]
140a78ed4: 4c8b0c08 mov r9, qword ptr [rax + rcx]
140a78ed8: 488d0d79e80502 lea rcx, [rip + 0x205e879]
140a78edf: 488b0408 mov rax, qword ptr [rax + rcx]
140a78ee3: 4889442420 mov qword ptr [rsp + 0x20], rax
140a78ee8: 488d152b660702 lea rdx, [rip + 0x207662b] ; @142aef51a 'assets/gladestatecrates/country-core/src/resources/session.rsloading_background.jpg'
140a78eef: 488db580010000 lea rsi, [rbp + 0x180]
140a78ef6: 41b80c000000 mov r8d, 0xc
140a78efc: 4889f1 mov rcx, rsi
140a78eff: e80c992e00 call 0x140d62810 ; std::path::Path::_join
140a78f04: 4889f1 mov rcx, rsi
140a78f07: e8c4df2f00 call 0x140d76ed0 ; std::sys::os_str::wtf8::Buf::as_mut_slice | alloc::boxed::convert::impl$19::from::impl$0::description
140a78f0c: 4989d0 mov r8, rdx
140a78f0f: 48c74424200a000000 mov qword ptr [rsp + 0x20], 0xa
140a78f18: 4c8d0dd1de0502 lea r9, [rip + 0x205ded1] ; @142ad6df0 'trees.json'
140a78f1f: 488d8db0000000 lea rcx, [rbp + 0xb0]
140a78f26: 4889c2 mov rdx, rax
140a78f29: e8e2982e00 call 0x140d62810 ; std::path::Path::_join
140a78f2e: 488d8db0000000 lea rcx, [rbp + 0xb0]
140a78f35: e896df2f00 call 0x140d76ed0 ; std::sys::os_str::wtf8::Buf::as_mut_slice | alloc::boxed::convert::impl$19::from::impl$0::description
140a78f3a: 4989d0 mov r8, rdx
140a78f3d: 488d8d50010000 lea rcx, [rbp + 0x150]
140a78f44: 4889c2 mov rdx, rax
140a78f47: e8f4012e00 call 0x140d59140 ; std::fs::read_to_string::inner
140a78f4c: 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
140a78f53: 4885d2 test rdx, rdx
140a78f56: 7412 je 0x140a78f6a
140a78f58: 488b8db8000000 mov rcx, qword ptr [rbp + 0xb8]
140a78f5f: 41b801000000 mov r8d, 1
140a78f65: e8b6acb9ff call 0x140613c20 ; 
140a78f6a: 488b8550010000 mov rax, qword ptr [rbp + 0x150]
140a78f71: 48898518020000 mov qword ptr [rbp + 0x218], rax
140a78f78: 48f7d8 neg rax
140a78f7b: 0f80b10b0000 jo 0x140a79b32
140a78f81: 488b9558010000 mov rdx, qword ptr [rbp + 0x158]
140a78f88: 4c8b8560010000 mov r8, qword ptr [rbp + 0x160]
140a78f8f: 488d4d78 lea rcx, [rbp + 0x78]
140a78f93: 48899508020000 mov qword ptr [rbp + 0x208], rdx
140a78f9a: e811d39800 call 0x1414062b0 ; serde_json::de::from_str<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >
140a78f9f: 31c0 xor eax, eax
140a78fa1: 483b4578 cmp rax, qword ptr [rbp + 0x78]
140a78fa5: 0f80c50b0000 jo 0x140a79b70
140a78fab: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
140a78fb2: 48894540 mov qword ptr [rbp + 0x40], rax
140a78fb6: f30f6f4578 movdqu xmm0, xmmword ptr [rbp + 0x78]
140a78fbb: 660f7f4530 movdqa xmmword ptr [rbp + 0x30], xmm0
140a78fc0: 488b9518020000 mov rdx, qword ptr [rbp + 0x218]
140a78fc7: 4885d2 test rdx, rdx
140a78fca: 7412 je 0x140a78fde
140a78fcc: 41b801000000 mov r8d, 1
140a78fd2: 488b8d08020000 mov rcx, qword ptr [rbp + 0x208]
140a78fd9: e842acb9ff call 0x140613c20 ; 
140a78fde: 488bb5f0020000 mov rsi, qword ptr [rbp + 0x2f0]
140a78fe5: 488b9580010000 mov rdx, qword ptr [rbp + 0x180]
140a78fec: 4885d2 test rdx, rdx
140a78fef: 7412 je 0x140a79003
140a78ff1: 488b8d88010000 mov rcx, qword ptr [rbp + 0x188]
140a78ff8: 41b801000000 mov r8d, 1
140a78ffe: e81dacb9ff call 0x140613c20 ; 
140a79003: 48c7852801000000000000 mov qword ptr [rbp + 0x128], 0
140a7900e: 48c7853001000004000000 mov qword ptr [rbp + 0x130], 4
140a79019: 48c7853801000000000000 mov qword ptr [rbp + 0x138], 0
140a79024: 8b461c mov eax, dword ptr [rsi + 0x1c]
140a79027: 488b4e10 mov rcx, qword ptr [rsi + 0x10]
140a7902b: 894574 mov dword ptr [rbp + 0x74], eax
140a7902e: 48894d20 mov qword ptr [rbp + 0x20], rcx
140a79032: 8901 mov dword ptr [rcx], eax
140a79034: 488b06 mov rax, qword ptr [rsi]
140a79037: 488b4808 mov rcx, qword ptr [rax + 8]
140a7903b: 48898d08020000 mov qword ptr [rbp + 0x208], rcx
140a79042: 488b4810 mov rcx, qword ptr [rax + 0x10]
140a79046: 48894558 mov qword ptr [rbp + 0x58], rax
140a7904a: 48c7401000000000 mov qword ptr [rax + 0x10], 0
140a79052: 48898d10020000 mov qword ptr [rbp + 0x210], rcx
140a79059: 4885c9 test rcx, rcx
140a7905c: 7457 je 0x140a790b5
140a7905e: 488bbd10020000 mov rdi, qword ptr [rbp + 0x210]
140a79065: 48f7df neg rdi
140a79068: b901000000 mov ecx, 1
140a7906d: 488bb508020000 mov rsi, qword ptr [rbp + 0x208]
140a79074: eb25 jmp 0x140a7909b
140a79076: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a79080: 488b8d18020000 mov rcx, qword ptr [rbp + 0x218]
140a79087: 488d040f lea rax, [rdi + rcx]
140a7908b: 48ffc0 inc rax
140a7908e: 48ffc1 inc rcx
140a79091: 4883c650 add rsi, 0x50
140a79095: 4883f801 cmp rax, 1
140a79099: 741a je 0x140a790b5
140a7909b: 48898d18020000 mov qword ptr [rbp + 0x218], rcx
140a790a2: 488b06 mov rax, qword ptr [rsi]
140a790a5: f048ff08 lock dec qword ptr [rax]
140a790a9: 75d5 jne 0x140a79080
140a790ab: 4889f1 mov rcx, rsi
140a790ae: e87d5abdff call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
140a790b3: ebcb jmp 0x140a79080
140a790b5: 48895d28 mov qword ptr [rbp + 0x28], rbx
140a790b9: 488b4558 mov rax, qword ptr [rbp + 0x58]
140a790bd: c6401800 mov byte ptr [rax + 0x18], 0
140a790c1: 488b5530 mov rdx, qword ptr [rbp + 0x30]
140a790c5: 488b4538 mov rax, qword ptr [rbp + 0x38]
140a790c9: 488b4d40 mov rcx, qword ptr [rbp + 0x40]
140a790cd: 488d0c89 lea rcx, [rcx + rcx*4]
140a790d1: 48c1e104 shl rcx, 4
140a790d5: 4801c1 add rcx, rax
140a790d8: 48894578 mov qword ptr [rbp + 0x78], rax
140a790dc: 48898580000000 mov qword ptr [rbp + 0x80], rax
140a790e3: 48899588000000 mov qword ptr [rbp + 0x88], rdx
140a790ea: 48898d90000000 mov qword ptr [rbp + 0x90], rcx
140a790f1: 660f6f3587cc0102 movdqa xmm6, xmmword ptr [rip + 0x201cc87] ; @142a95d80 'clearing/tree/bush'
140a790f9: 660f6e3d8fcc0102 movd xmm7, dword ptr [rip + 0x201cc8f] ; @142a95d90 'sh'
140a79101: 488dbdb0000000 lea rdi, [rbp + 0xb0]
140a79108: 488d5d50 lea rbx, [rbp + 0x50]
140a7910c: eb10 jmp 0x140a7911e
140a7910e: 6690 nop 
140a79110: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
140a79117: 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
140a7911e: 4839c8 cmp rax, rcx
140a79121: 0f8449050000 je 0x140a79670
140a79127: 488d4850 lea rcx, [rax + 0x50]
140a7912b: 48898d80000000 mov qword ptr [rbp + 0x80], rcx
140a79132: 488b10 mov rdx, qword ptr [rax]
140a79135: 4889d1 mov rcx, rdx
140a79138: 48f7d9 neg rcx
140a7913b: 0f802f050000 jo 0x140a79670
140a79141: 488995a0000000 mov qword ptr [rbp + 0xa0], rdx
140a79148: 488b4808 mov rcx, qword ptr [rax + 8]
140a7914c: 48898d10020000 mov qword ptr [rbp + 0x210], rcx
140a79153: 4c8b6810 mov r13, qword ptr [rax + 0x10]
140a79157: 488b4818 mov rcx, qword ptr [rax + 0x18]
140a7915b: 48898d98000000 mov qword ptr [rbp + 0x98], rcx
140a79162: 488b4820 mov rcx, qword ptr [rax + 0x20]
140a79166: 48898d40010000 mov qword ptr [rbp + 0x140], rcx
140a7916d: 4c8b7828 mov r15, qword ptr [rax + 0x28]
140a79171: 488b7030 mov rsi, qword ptr [rax + 0x30]
140a79175: 488b4838 mov rcx, qword ptr [rax + 0x38]
140a79179: 48898d78010000 mov qword ptr [rbp + 0x178], rcx
140a79180: 4c8b6040 mov r12, qword ptr [rax + 0x40]
140a79184: 448b7048 mov r14d, dword ptr [rax + 0x48]
140a79188: 4983fd12 cmp r13, 0x12
140a7918c: 752e jne 0x140a791bc
140a7918e: 488b8510020000 mov rax, qword ptr [rbp + 0x210]
140a79195: f30f6f00 movdqu xmm0, xmmword ptr [rax]
140a79199: 660f74c6 pcmpeqb xmm0, xmm6
140a7919d: 0fb74010 movzx eax, word ptr [rax + 0x10]
140a791a1: 660f6ec8 movd xmm1, eax
140a791a5: 660f74cf pcmpeqb xmm1, xmm7
140a791a9: 660fdbc8 pand xmm1, xmm0
140a791ad: 660fd7c1 pmovmskb eax, xmm1
140a791b1: 3dffff0000 cmp eax, 0xffff
140a791b6: 0f8429040000 je 0x140a795e5
140a791bc: 4889b570010000 mov qword ptr [rbp + 0x170], rsi
140a791c3: 488b95d8020000 mov rdx, qword ptr [rbp + 0x2d8]
140a791ca: 8b421c mov eax, dword ptr [rdx + 0x1c]
140a791cd: 488b4a10 mov rcx, qword ptr [rdx + 0x10]
140a791d1: 8901 mov dword ptr [rcx], eax
140a791d3: 488b32 mov rsi, qword ptr [rdx]
140a791d6: 488b85a8000000 mov rax, qword ptr [rbp + 0xa8]
140a791dd: 488b00 mov rax, qword ptr [rax]
140a791e0: 0fb64058 movzx eax, byte ptr [rax + 0x58]
140a791e4: c1e003 shl eax, 3
140a791e7: 488d0d42e50502 lea rcx, [rip + 0x205e542]
140a791ee: 4c8b0c08 mov r9, qword ptr [rax + rcx]
140a791f2: 488d0d5fe50502 lea rcx, [rip + 0x205e55f]
140a791f9: 488b0408 mov rax, qword ptr [rax + rcx]
140a791fd: c6852602000001 mov byte ptr [rbp + 0x226], 1
140a79204: 4889442420 mov qword ptr [rsp + 0x20], rax
140a79209: 41b80c000000 mov r8d, 0xc
140a7920f: 488d8d80010000 lea rcx, [rbp + 0x180]
140a79216: 488d15fd620702 lea rdx, [rip + 0x20762fd] ; @142aef51a 'assets/gladestatecrates/country-core/src/resources/session.rsloading_background.jpg'
140a7921d: e8ee952e00 call 0x140d62810 ; std::path::Path::_join
140a79222: c6852602000001 mov byte ptr [rbp + 0x226], 1
140a79229: 4889f9 mov rcx, rdi
140a7922c: 488b9540010000 mov rdx, qword ptr [rbp + 0x140]
140a79233: 4d89f8 mov r8, r15
140a79236: 4c8d8d80010000 lea r9, [rbp + 0x180]
140a7923d: e89ed68600 call 0x1412e68e0 ; relative_path::RelativePath::to_path<std::path::PathBuf>
140a79242: c6852602000001 mov byte ptr [rbp + 0x226], 1
140a79249: 488d8d50010000 lea rcx, [rbp + 0x150]
140a79250: 4889f2 mov rdx, rsi
140a79253: 4c8b8500020000 mov r8, qword ptr [rbp + 0x200]
140a7925a: 4989f9 mov r9, rdi
140a7925d: e86ed3ebff call 0x1409365d0 ; country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary::load<std::path::PathBuf>
140a79262: c6852502000001 mov byte ptr [rbp + 0x225], 1
140a79269: 488b8d10020000 mov rcx, qword ptr [rbp + 0x210]
140a79270: 4c89ea mov rdx, r13
140a79273: e808cead00 call 0x141556080 ; asset::prefab::PrefabName::new
140a79278: 48894560 mov qword ptr [rbp + 0x60], rax
140a7927c: 48895568 mov qword ptr [rbp + 0x68], rdx
140a79280: 44897570 mov dword ptr [rbp + 0x70], r14d
140a79284: 8b4574 mov eax, dword ptr [rbp + 0x74]
140a79287: 488b4d20 mov rcx, qword ptr [rbp + 0x20]
140a7928b: 8901 mov dword ptr [rcx], eax
140a7928d: 488b8558010000 mov rax, qword ptr [rbp + 0x158]
140a79294: 488b8d60010000 mov rcx, qword ptr [rbp + 0x160]
140a7929b: 488d0c49 lea rcx, [rcx + rcx*2]
140a7929f: 48c1e104 shl rcx, 4
140a792a3: 4801c1 add rcx, rax
140a792a6: 48898580010000 mov qword ptr [rbp + 0x180], rax
140a792ad: 48898d88010000 mov qword ptr [rbp + 0x188], rcx
140a792b4: 488d4560 lea rax, [rbp + 0x60]
140a792b8: 48898590010000 mov qword ptr [rbp + 0x190], rax
140a792bf: 488d4570 lea rax, [rbp + 0x70]
140a792c3: 48898598010000 mov qword ptr [rbp + 0x198], rax
140a792ca: c6852402000001 mov byte ptr [rbp + 0x224], 1
140a792d1: 488b4d58 mov rcx, qword ptr [rbp + 0x58]
140a792d5: 488d9580010000 lea rdx, [rbp + 0x180]
140a792dc: 4c8d05cddb0502 lea r8, [rip + 0x205dbcd]
140a792e3: e8280d8800 call 0x1412fa010 ; alloc::vec::spec_extend::impl$0::spec_extend<country_core::resources::tree_storage::ForestTree,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env$0>,alloc::alloc::Global>
140a792e8: 40b601 mov sil, 1
140a792eb: 488b8570010000 mov rax, qword ptr [rbp + 0x170]
140a792f2: 48f7d8 neg rax
140a792f5: 0f805c020000 jo 0x140a79557
140a792fb: 48c74424200d000000 mov qword ptr [rsp + 0x20], 0xd
140a79304: 4889f9 mov rcx, rdi
140a79307: 488b9578010000 mov rdx, qword ptr [rbp + 0x178]
140a7930e: 4d89e0 mov r8, r12
140a79311: 4c8d0db0db0502 lea r9, [rip + 0x205dbb0] ; @142ad6ec8 'assets/meshes'
140a79318: e8b3d78600 call 0x1412e6ad0 ; relative_path::RelativePath::to_path<ref$<std::path::Path> >
140a7931d: 488d8d80010000 lea rcx, [rbp + 0x180]
140a79324: 4889fa mov rdx, rdi
140a79327: e8346b0900 call 0x140b0fe60 ; country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>
140a7932c: 31c0 xor eax, eax
140a7932e: 483b8580010000 cmp rax, qword ptr [rbp + 0x180]
140a79335: 0f80b9070000 jo 0x140a79af4
140a7933b: 488b85f0010000 mov rax, qword ptr [rbp + 0x1f0]
140a79342: 48898520010000 mov qword ptr [rbp + 0x120], rax
140a79349: 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
140a79350: 0f298510010000 movaps xmmword ptr [rbp + 0x110], xmm0
140a79357: 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
140a7935e: 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
140a79365: 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
140a7936c: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
140a79373: f30f6f8580010000 movdqu xmm0, xmmword ptr [rbp + 0x180]
140a7937b: f30f6f8d90010000 movdqu xmm1, xmmword ptr [rbp + 0x190]
140a79383: 0f1095a0010000 movups xmm2, xmmword ptr [rbp + 0x1a0]
140a7938a: 0f109db0010000 movups xmm3, xmmword ptr [rbp + 0x1b0]
140a79391: 0f299de0000000 movaps xmmword ptr [rbp + 0xe0], xmm3
140a79398: 0f2995d0000000 movaps xmmword ptr [rbp + 0xd0], xmm2
140a7939f: 660f7f8dc0000000 movdqa xmmword ptr [rbp + 0xc0], xmm1
140a793a7: 660f7f85b0000000 movdqa xmmword ptr [rbp + 0xb0], xmm0
140a793af: 4889f9 mov rcx, rdi
140a793b2: e8e9e8f8ff call 0x140a07ca0 ; country_core::render::mesh::Mesh::positions_without_indices
140a793b7: 4989d0 mov r8, rdx
140a793ba: 488d4d08 lea rcx, [rbp + 8]
140a793be: 4889c2 mov rdx, rax
140a793c1: 4c8d0d60d50502 lea r9, [rip + 0x205d560]
140a793c8: e8037f7e00 call 0x1412612d0 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> > | alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::utils::geo::update_curve_mesh::closure_env$0<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> > > > > | alloc::vec::spec_from_iter::impl$0::from_iter<nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> >,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)> > | alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0> >
140a793cd: 4889f9 mov rcx, rdi
140a793d0: e85bf7e0ff call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
140a793d5: 4c8bbd58010000 mov r15, qword ptr [rbp + 0x158]
140a793dc: 488b8560010000 mov rax, qword ptr [rbp + 0x160]
140a793e3: 4c8d2c40 lea r13, [rax + rax*2]
140a793e7: 49c1e504 shl r13, 4
140a793eb: 4d01fd add r13, r15
140a793ee: 488b4d10 mov rcx, qword ptr [rbp + 0x10]
140a793f2: 488b4518 mov rax, qword ptr [rbp + 0x18]
140a793f6: 48898518020000 mov qword ptr [rbp + 0x218], rax
140a793fd: 488d0440 lea rax, [rax + rax*2]
140a79401: 48898d08020000 mov qword ptr [rbp + 0x208], rcx
140a79408: 4c8d3481 lea r14, [rcx + rax*4]
140a7940c: eb06 jmp 0x140a79414
140a7940e: 6690 nop 
140a79410: 4983c730 add r15, 0x30
140a79414: 4d39ef cmp r15, r13
140a79417: 0f84f3000000 je 0x140a79510
140a7941d: 4c89b588010000 mov qword ptr [rbp + 0x188], r14
140a79424: 4c89bd90010000 mov qword ptr [rbp + 0x190], r15
140a7942b: 488d8590010000 lea rax, [rbp + 0x190]
140a79432: 48894550 mov qword ptr [rbp + 0x50], rax
140a79436: 4883bd1802000000 cmp qword ptr [rbp + 0x218], 0
140a7943e: 74d0 je 0x140a79410
140a79440: 488bb508020000 mov rsi, qword ptr [rbp + 0x208]
140a79447: 4d89f4 mov r12, r14
140a7944a: 660f1f440000 nop word ptr [rax + rax]
140a79450: 4989f0 mov r8, rsi
140a79453: 4883c60c add rsi, 0xc
140a79457: 4889b580010000 mov qword ptr [rbp + 0x180], rsi
140a7945e: 4889f9 mov rcx, rdi
140a79461: 4889da mov rdx, rbx
140a79464: e8378ff9ff call 0x140a123a0 ; core::ops::function::impls::impl$3::call_mut<tuple$<ref$<glam::f32::vec3::Vec3> >,country_core::startup::clearing::init_clearing::closure_env$1>
140a79469: 83bdb000000001 cmp dword ptr [rbp + 0xb0], 1
140a79470: 7407 je 0x140a79479
140a79472: 4c39e6 cmp rsi, r12
140a79475: 75d9 jne 0x140a79450
140a79477: eb97 jmp 0x140a79410
140a79479: 488d85b4000000 lea rax, [rbp + 0xb4]
140a79480: f30f6f00 movdqu xmm0, xmmword ptr [rax]
140a79484: 660f7f45f0 movdqa xmmword ptr [rbp - 0x10], xmm0
140a79489: 488bb538010000 mov rsi, qword ptr [rbp + 0x138]
140a79490: 483bb528010000 cmp rsi, qword ptr [rbp + 0x128]
140a79497: 7524 jne 0x140a794bd
140a79499: 48c744242010000000 mov qword ptr [rsp + 0x20], 0x10
140a794a2: 41b801000000 mov r8d, 1
140a794a8: 41b904000000 mov r9d, 4
140a794ae: 488d8d28010000 lea rcx, [rbp + 0x128]
140a794b5: 4889f2 mov rdx, rsi
140a794b8: e8a305e301 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140a794bd: 488b8530010000 mov rax, qword ptr [rbp + 0x130]
140a794c4: 4889f1 mov rcx, rsi
140a794c7: 48c1e104 shl rcx, 4
140a794cb: 660f6f45f0 movdqa xmm0, xmmword ptr [rbp - 0x10]
140a794d0: f30f7f0408 movdqu xmmword ptr [rax + rcx], xmm0
140a794d5: 48ffc6 inc rsi
140a794d8: 4889b538010000 mov qword ptr [rbp + 0x138], rsi
140a794df: 488d8590010000 lea rax, [rbp + 0x190]
140a794e6: 48894550 mov qword ptr [rbp + 0x50], rax
140a794ea: 488bb580010000 mov rsi, qword ptr [rbp + 0x180]
140a794f1: 4c8ba588010000 mov r12, qword ptr [rbp + 0x188]
140a794f8: 4c39e6 cmp rsi, r12
140a794fb: 0f854fffffff jne 0x140a79450
140a79501: e90affffff jmp 0x140a79410
140a79506: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a79510: 488b4508 mov rax, qword ptr [rbp + 8]
140a79514: 4885c0 test rax, rax
140a79517: 741a je 0x140a79533
140a79519: 48c1e002 shl rax, 2
140a7951d: 488d1440 lea rdx, [rax + rax*2]
140a79521: 41b804000000 mov r8d, 4
140a79527: 488b8d08020000 mov rcx, qword ptr [rbp + 0x208]
140a7952e: e8eda6b9ff call 0x140613c20 ; 
140a79533: 488b9570010000 mov rdx, qword ptr [rbp + 0x170]
140a7953a: 4885d2 test rdx, rdx
140a7953d: 0f8418010000 je 0x140a7965b
140a79543: 41b801000000 mov r8d, 1
140a79549: 488b8d78010000 mov rcx, qword ptr [rbp + 0x178]
140a79550: e8cba6b9ff call 0x140613c20 ; 
140a79555: 31f6 xor esi, esi
140a79557: 488b4560 mov rax, qword ptr [rbp + 0x60]
140a7955b: f048ff08 lock dec qword ptr [rax]
140a7955f: 7510 jne 0x140a79571
140a79561: 4088b525020000 mov byte ptr [rbp + 0x225], sil
140a79568: 488d4d60 lea rcx, [rbp + 0x60]
140a7956c: e8bf55bdff call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
140a79571: 488b8550010000 mov rax, qword ptr [rbp + 0x150]
140a79578: 4885c0 test rax, rax
140a7957b: 741a je 0x140a79597
140a7957d: 488b8d58010000 mov rcx, qword ptr [rbp + 0x158]
140a79584: 48c1e004 shl rax, 4
140a79588: 488d1440 lea rdx, [rax + rax*2]
140a7958c: 41b804000000 mov r8d, 4
140a79592: e889a6b9ff call 0x140613c20 ; 
140a79597: 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
140a7959e: 4885d2 test rdx, rdx
140a795a1: 7412 je 0x140a795b5
140a795a3: 41b801000000 mov r8d, 1
140a795a9: 488b8d10020000 mov rcx, qword ptr [rbp + 0x210]
140a795b0: e86ba6b9ff call 0x140613c20 ; 
140a795b5: 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
140a795bc: 4885d2 test rdx, rdx
140a795bf: 7412 je 0x140a795d3
140a795c1: 41b801000000 mov r8d, 1
140a795c7: 488b8d40010000 mov rcx, qword ptr [rbp + 0x140]
140a795ce: e84da6b9ff call 0x140613c20 ; 
140a795d3: 4084f6 test sil, sil
140a795d6: 488bb570010000 mov rsi, qword ptr [rbp + 0x170]
140a795dd: 0f842dfbffff je 0x140a79110
140a795e3: eb4b jmp 0x140a79630
140a795e5: 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
140a795ec: 4885d2 test rdx, rdx
140a795ef: 7412 je 0x140a79603
140a795f1: 41b801000000 mov r8d, 1
140a795f7: 488b8d10020000 mov rcx, qword ptr [rbp + 0x210]
140a795fe: e81da6b9ff call 0x140613c20 ; 
140a79603: 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
140a7960a: 4885d2 test rdx, rdx
140a7960d: 7421 je 0x140a79630
140a7960f: 41b801000000 mov r8d, 1
140a79615: 488b8d40010000 mov rcx, qword ptr [rbp + 0x140]
140a7961c: e8ffa5b9ff call 0x140613c20 ; 
140a79621: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a79630: 488d047500000000 lea rax, [rsi*2]
140a79638: 4885c0 test rax, rax
140a7963b: 0f84cffaffff je 0x140a79110
140a79641: 41b801000000 mov r8d, 1
140a79647: 488b8d78010000 mov rcx, qword ptr [rbp + 0x178]
140a7964e: 4889f2 mov rdx, rsi
140a79651: e8caa5b9ff call 0x140613c20 ; 
140a79656: e9b5faffff jmp 0x140a79110
140a7965b: 31f6 xor esi, esi
140a7965d: 488b4560 mov rax, qword ptr [rbp + 0x60]
140a79661: f048ff08 lock dec qword ptr [rax]
140a79665: 0f84f6feffff je 0x140a79561
140a7966b: e901ffffff jmp 0x140a79571
140a79670: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a79677: 488d7578 lea rsi, [rbp + 0x78]
140a7967b: 4889f1 mov rcx, rsi
140a7967e: e8adfbdfff call 0x140879230 ; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >
140a79683: 4c8ba5d0020000 mov r12, qword ptr [rbp + 0x2d0]
140a7968a: 488b95e0020000 mov rdx, qword ptr [rbp + 0x2e0]
140a79691: 8b421c mov eax, dword ptr [rdx + 0x1c]
140a79694: 488b4a10 mov rcx, qword ptr [rdx + 0x10]
140a79698: 8901 mov dword ptr [rcx], eax
140a7969a: 488b0a mov rcx, qword ptr [rdx]
140a7969d: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a796a4: e8d7dc8400 call 0x1412c7380 ; country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::systems::render_loop::compute_passes::leaf_particles::ParticleEmitterSsbo>
140a796a9: 488bbd38010000 mov rdi, qword ptr [rbp + 0x138]
140a796b0: 4885c0 test rax, rax
140a796b3: 0f8480000000 je 0x140a79739
140a796b9: 4885ff test rdi, rdi
140a796bc: 747b je 0x140a79739
140a796be: 4889f9 mov rcx, rdi
140a796c1: 48c1e104 shl rcx, 4
140a796c5: 483b4818 cmp rcx, qword ptr [rax + 0x18]
140a796c9: 0f875b050000 ja 0x140a79c2a
140a796cf: 4c8b8d30010000 mov r9, qword ptr [rbp + 0x130]
140a796d6: 48c7855001000000000000 mov qword ptr [rbp + 0x150], 0
140a796e1: 48898d58010000 mov qword ptr [rbp + 0x158], rcx
140a796e8: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a796ef: 488d9550010000 lea rdx, [rbp + 0x150]
140a796f6: 4889542428 mov qword ptr [rsp + 0x28], rdx
140a796fb: 48894c2420 mov qword ptr [rsp + 0x20], rcx
140a79700: 48c744243800000000 mov qword ptr [rsp + 0x38], 0
140a79709: 48c744243001000000 mov qword ptr [rsp + 0x30], 1
140a79712: 488d8d80010000 lea rcx, [rbp + 0x180]
140a79719: 488b9500020000 mov rdx, qword ptr [rbp + 0x200]
140a79720: 4989c0 mov r8, rax
140a79723: e878581700 call 0x140beefa0 ; rhapsody::raw::device::GraphicsDevice::update_buffer_bytes
140a79728: 488b8580010000 mov rax, qword ptr [rbp + 0x180]
140a7972f: 4883f80b cmp rax, 0xb
140a79733: 0f85f8040000 jne 0x140a79c31
140a79739: 4881ff4c040000 cmp rdi, 0x44c
140a79740: b84c040000 mov eax, 0x44c
140a79745: 480f42c7 cmovb rax, rdi
140a79749: 4c8b85e8020000 mov r8, qword ptr [rbp + 0x2e8]
140a79750: 418b481c mov ecx, dword ptr [r8 + 0x1c]
140a79754: 498b5010 mov rdx, qword ptr [r8 + 0x10]
140a79758: 890a mov dword ptr [rdx], ecx
140a7975a: 498b08 mov rcx, qword ptr [r8]
140a7975d: 488901 mov qword ptr [rcx], rax
140a79760: 488b85c8020000 mov rax, qword ptr [rbp + 0x2c8]
140a79767: 488b18 mov rbx, qword ptr [rax]
140a7976a: 488b85c0020000 mov rax, qword ptr [rbp + 0x2c0]
140a79771: 4c8b30 mov r14, qword ptr [rax]
140a79774: 488d05e5d60502 lea rax, [rip + 0x205d6e5] ; @142ad6e60 'distant_billboardassets/meshes/distant_billboard.json'
140a7977b: 48894578 mov qword ptr [rbp + 0x78], rax
140a7977f: 48c7858000000011000000 mov qword ptr [rbp + 0x80], 0x11
140a7978a: 4889b550010000 mov qword ptr [rbp + 0x150], rsi
140a79791: 4c8d3df80669ff lea r15, [rip - 0x96f908]
140a79798: 4c89bd58010000 mov qword ptr [rbp + 0x158], r15
140a7979f: 488d05fad40502 lea rax, [rip + 0x205d4fa]
140a797a6: 48898580010000 mov qword ptr [rbp + 0x180], rax
140a797ad: 48c7858801000002000000 mov qword ptr [rbp + 0x188], 2
140a797b8: 488d8550010000 lea rax, [rbp + 0x150]
140a797bf: 48898590010000 mov qword ptr [rbp + 0x190], rax
140a797c6: 48c7859801000001000000 mov qword ptr [rbp + 0x198], 1
140a797d1: 48c785a001000000000000 mov qword ptr [rbp + 0x1a0], 0
140a797dc: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a797e3: 488d8db0000000 lea rcx, [rbp + 0xb0]
140a797ea: 488d9580010000 lea rdx, [rbp + 0x180]
140a797f1: e89a073000 call 0x140d79f90 ; alloc::fmt::format::format_inner
140a797f6: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
140a797fd: 48898518020000 mov qword ptr [rbp + 0x218], rax
140a79804: 488bbdc0000000 mov rdi, qword ptr [rbp + 0xc0]
140a7980b: 4889b550010000 mov qword ptr [rbp + 0x150], rsi
140a79812: 4c89bd58010000 mov qword ptr [rbp + 0x158], r15
140a79819: 488d05a8d40502 lea rax, [rip + 0x205d4a8]
140a79820: 48898580010000 mov qword ptr [rbp + 0x180], rax
140a79827: 48c7858801000002000000 mov qword ptr [rbp + 0x188], 2
140a79832: 488d8550010000 lea rax, [rbp + 0x150]
140a79839: 48898590010000 mov qword ptr [rbp + 0x190], rax
140a79840: 48c7859801000001000000 mov qword ptr [rbp + 0x198], 1
140a7984b: 48c785a001000000000000 mov qword ptr [rbp + 0x1a0], 0
140a79856: 488d4dd8 lea rcx, [rbp - 0x28]
140a7985a: 488d9580010000 lea rdx, [rbp + 0x180]
140a79861: e82a073000 call 0x140d79f90 ; alloc::fmt::format::format_inner
140a79866: 4c89742428 mov qword ptr [rsp + 0x28], r14
140a7986b: 48895c2420 mov qword ptr [rsp + 0x20], rbx
140a79870: 4c8d4dd8 lea r9, [rbp - 0x28]
140a79874: 488b8d00020000 mov rcx, qword ptr [rbp + 0x200]
140a7987b: 488b9518020000 mov rdx, qword ptr [rbp + 0x218]
140a79882: 4989f8 mov r8, rdi
140a79885: e886690c00 call 0x140b40210 ; country_core::startup::load_shader_into_library<alloc::string::String>
140a7988a: 4889c6 mov rsi, rax
140a7988d: 488b8d18020000 mov rcx, qword ptr [rbp + 0x218]
140a79894: 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
140a7989b: 4885d2 test rdx, rdx
140a7989e: 740b je 0x140a798ab
140a798a0: 41b801000000 mov r8d, 1
140a798a6: e875a3b9ff call 0x140613c20 ; 
140a798ab: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a798b2: 488d15b8d50502 lea rdx, [rip + 0x205d5b8] ; @142ad6e71 'assets/meshes/distant_billboard.json'
140a798b9: 488d8d80010000 lea rcx, [rbp + 0x180]
140a798c0: 41b824000000 mov r8d, 0x24
140a798c6: e8e5550900 call 0x140b0eeb0 ; country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
140a798cb: 31c0 xor eax, eax
140a798cd: 483b8580010000 cmp rax, qword ptr [rbp + 0x180]
140a798d4: 0f80d4020000 jo 0x140a79bae
140a798da: 488b85f0010000 mov rax, qword ptr [rbp + 0x1f0]
140a798e1: 48898520010000 mov qword ptr [rbp + 0x120], rax
140a798e8: 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
140a798ef: 0f298510010000 movaps xmmword ptr [rbp + 0x110], xmm0
140a798f6: 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
140a798fd: 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
140a79904: 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
140a7990b: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
140a79912: f30f6f8580010000 movdqu xmm0, xmmword ptr [rbp + 0x180]
140a7991a: f30f6f8d90010000 movdqu xmm1, xmmword ptr [rbp + 0x190]
140a79922: 0f1095a0010000 movups xmm2, xmmword ptr [rbp + 0x1a0]
140a79929: 0f109db0010000 movups xmm3, xmmword ptr [rbp + 0x1b0]
140a79930: 0f299de0000000 movaps xmmword ptr [rbp + 0xe0], xmm3
140a79937: 0f2995d0000000 movaps xmmword ptr [rbp + 0xd0], xmm2
140a7993e: 660f7f8dc0000000 movdqa xmmword ptr [rbp + 0xc0], xmm1
140a79946: 660f7f85b0000000 movdqa xmmword ptr [rbp + 0xb0], xmm0
140a7994e: 488b8548010000 mov rax, qword ptr [rbp + 0x148]
140a79955: 4c8b08 mov r9, qword ptr [rax]
140a79958: 488b4528 mov rax, qword ptr [rbp + 0x28]
140a7995c: 488b00 mov rax, qword ptr [rax]
140a7995f: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a79966: 4889442420 mov qword ptr [rsp + 0x20], rax
140a7996b: 488d15eed40502 lea rdx, [rip + 0x205d4ee] ; @142ad6e60 'distant_billboardassets/meshes/distant_billboard.json'
140a79972: 488d8db0000000 lea rcx, [rbp + 0xb0]
140a79979: 41b811000000 mov r8d, 0x11
140a7997f: e81c130a00 call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
140a79984: 4889c7 mov rdi, rax
140a79987: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a7998e: 488d8d80010000 lea rcx, [rbp + 0x180]
140a79995: 4c89e2 mov rdx, r12
140a79998: e803ad2900 call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
140a7999d: 488b9d88010000 mov rbx, qword ptr [rbp + 0x188]
140a799a4: 4885db test rbx, rbx
140a799a7: 7507 jne 0x140a799b0
140a799a9: 488b9d90010000 mov rbx, qword ptr [rbp + 0x190]
140a799b0: 4c8bbda0010000 mov r15, qword ptr [rbp + 0x1a0]
140a799b7: 488b03 mov rax, qword ptr [rbx]
140a799ba: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
140a799be: 4c29f0 sub rax, r14
140a799c1: 4883f877 cmp rax, 0x77
140a799c5: 0f86b3000000 jbe 0x140a79a7e
140a799cb: 488b4308 mov rax, qword ptr [rbx + 8]
140a799cf: 488d0d8a3a8600 lea rcx, [rip + 0x863a8a]
140a799d6: 4a890c30 mov qword ptr [rax + r14], rcx
140a799da: 0f2805df46ec01 movaps xmm0, xmmword ptr [rip + 0x1ec46df]
140a799e1: 420f11443008 movups xmmword ptr [rax + r14 + 8], xmm0
140a799e7: 4ac744301800000000 mov qword ptr [rax + r14 + 0x18], 0
140a799f0: 42c744302000000000 mov dword ptr [rax + r14 + 0x20], 0
140a799f9: 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
140a79a03: 4a894c3024 mov qword ptr [rax + r14 + 0x24], rcx
140a79a08: 42c744302c0000803f mov dword ptr [rax + r14 + 0x2c], 0x3f800000
140a79a11: 4a897c3038 mov qword ptr [rax + r14 + 0x38], rdi
140a79a16: 4a89743040 mov qword ptr [rax + r14 + 0x40], rsi
140a79a1b: 42c644304800 mov byte ptr [rax + r14 + 0x48], 0
140a79a21: 4e897c3058 mov qword ptr [rax + r14 + 0x58], r15
140a79a26: 488d0d03b3ae00 lea rcx, [rip + 0xaeb303]
140a79a2d: 4a894c3068 mov qword ptr [rax + r14 + 0x68], rcx
140a79a32: 4983c678 add r14, 0x78
140a79a36: 4c897310 mov qword ptr [rbx + 0x10], r14
140a79a3a: 488b9528010000 mov rdx, qword ptr [rbp + 0x128]
140a79a41: 4885d2 test rdx, rdx
140a79a44: 7416 je 0x140a79a5c
140a79a46: 488b8d30010000 mov rcx, qword ptr [rbp + 0x130]
140a79a4d: 48c1e204 shl rdx, 4
140a79a51: 41b804000000 mov r8d, 4
140a79a57: e8c4a1b9ff call 0x140613c20 ; 
140a79a5c: 0f28b530020000 movaps xmm6, xmmword ptr [rbp + 0x230]
140a79a63: 0f28bd40020000 movaps xmm7, xmmword ptr [rbp + 0x240]
140a79a6a: 4881c4d8020000 add rsp, 0x2d8
140a79a71: 5b pop rbx
140a79a72: 5f pop rdi
140a79a73: 5e pop rsi
140a79a74: 415c pop r12
140a79a76: 415d pop r13
140a79a78: 415e pop r14
140a79a7a: 415f pop r15
140a79a7c: 5d pop rbp
140a79a7d: c3 ret 
140a79a7e: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a79a85: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
140a79a8e: 41b878000000 mov r8d, 0x78
140a79a94: 41b901000000 mov r9d, 1
140a79a9a: 4889d9 mov rcx, rbx
140a79a9d: 4c89f2 mov rdx, r14
140a79aa0: e8bbffe201 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140a79aa5: e921ffffff jmp 0x140a799cb
140a79aaa: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
140a79ab3: 41b808000000 mov r8d, 8
140a79ab9: 41b901000000 mov r9d, 1
140a79abf: 4889f1 mov rcx, rsi
140a79ac2: 4889fa mov rdx, rdi
140a79ac5: e896ffe201 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140a79aca: e9d6f3ffff jmp 0x140a78ea5
140a79acf: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
140a79ad8: 41b878000000 mov r8d, 0x78
140a79ade: 41b901000000 mov r9d, 1
140a79ae4: 4889d9 mov rcx, rbx
140a79ae7: 4c89f2 mov rdx, r14
140a79aea: e871ffe201 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140a79aef: e93ff2ffff jmp 0x140a78d33
140a79af4: 488b8588010000 mov rax, qword ptr [rbp + 0x188]
140a79afb: 488985b0000000 mov qword ptr [rbp + 0xb0], rax
140a79b02: 488d05cfd30502 lea rax, [rip + 0x205d3cf]
140a79b09: 4889442420 mov qword ptr [rsp + 0x20], rax
140a79b0e: 488d0d8bce0502 lea rcx, [rip + 0x205ce8b] ; @142ad69a0 'called `Result::unwrap()` on an `Err` value'
140a79b15: 4c8d0d64ce0502 lea r9, [rip + 0x205ce64]
140a79b1c: 4c8d85b0000000 lea r8, [rbp + 0xb0]
140a79b23: ba2b000000 mov edx, 0x2b
140a79b28: e833fce501 call 0x1428d9760 ; core::result::unwrap_failed
140a79b2d: e9c5010000 jmp 0x140a79cf7
140a79b32: 488b8558010000 mov rax, qword ptr [rbp + 0x158]
140a79b39: 488985b0000000 mov qword ptr [rbp + 0xb0], rax
140a79b40: 488d05b9d20502 lea rax, [rip + 0x205d2b9]
140a79b47: 4889442420 mov qword ptr [rsp + 0x20], rax
140a79b4c: 488d0d9dd20502 lea rcx, [rip + 0x205d29d] ; @142ad6df0 'trees.json'
140a79b53: 4c8d0de6cd0502 lea r9, [rip + 0x205cde6]
140a79b5a: 4c8d85b0000000 lea r8, [rbp + 0xb0]
140a79b61: ba0a000000 mov edx, 0xa
140a79b66: e8f5fbe501 call 0x1428d9760 ; core::result::unwrap_failed
140a79b6b: e987010000 jmp 0x140a79cf7
140a79b70: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
140a79b77: 488985b0000000 mov qword ptr [rbp + 0xb0], rax
140a79b7e: 488d05abd20502 lea rax, [rip + 0x205d2ab]
140a79b85: 4889442420 mov qword ptr [rsp + 0x20], rax
140a79b8a: 488d0d87d20502 lea rcx, [rip + 0x205d287] ; @142ad6e18 'deserialize trees'
140a79b91: 4c8d0dc8cd0502 lea r9, [rip + 0x205cdc8]
140a79b98: 4c8d85b0000000 lea r8, [rbp + 0xb0]
140a79b9f: ba11000000 mov edx, 0x11
140a79ba4: e8b7fbe501 call 0x1428d9760 ; core::result::unwrap_failed
140a79ba9: e949010000 jmp 0x140a79cf7
140a79bae: 488b8588010000 mov rax, qword ptr [rbp + 0x188]
140a79bb5: 488985b0000000 mov qword ptr [rbp + 0xb0], rax
140a79bbc: 488d05d5d20502 lea rax, [rip + 0x205d2d5]
140a79bc3: 4889442420 mov qword ptr [rsp + 0x20], rax
140a79bc8: 488d0dd1cd0502 lea rcx, [rip + 0x205cdd1] ; @142ad69a0 'called `Result::unwrap()` on an `Err` value'
140a79bcf: 4c8d0daacd0502 lea r9, [rip + 0x205cdaa]
140a79bd6: 4c8d85b0000000 lea r8, [rbp + 0xb0]
140a79bdd: ba2b000000 mov edx, 0x2b
140a79be2: e879fbe501 call 0x1428d9760 ; core::result::unwrap_failed
140a79be7: e90b010000 jmp 0x140a79cf7
140a79bec: 488b8588010000 mov rax, qword ptr [rbp + 0x188]
140a79bf3: 488985b0000000 mov qword ptr [rbp + 0xb0], rax
140a79bfa: 488d0597d10502 lea rax, [rip + 0x205d197]
140a79c01: 4889442420 mov qword ptr [rsp + 0x20], rax
140a79c06: 488d0d93cd0502 lea rcx, [rip + 0x205cd93] ; @142ad69a0 'called `Result::unwrap()` on an `Err` value'
140a79c0d: 4c8d0d6ccd0502 lea r9, [rip + 0x205cd6c]
140a79c14: 4c8d85b0000000 lea r8, [rbp + 0xb0]
140a79c1b: ba2b000000 mov edx, 0x2b
140a79c20: e83bfbe501 call 0x1428d9760 ; core::result::unwrap_failed
140a79c25: e9cd000000 jmp 0x140a79cf7
140a79c2a: b80b000000 mov eax, 0xb
140a79c2f: eb4a jmp 0x140a79c7b
140a79c31: 488b8dc8010000 mov rcx, qword ptr [rbp + 0x1c8]
140a79c38: 48898df0000000 mov qword ptr [rbp + 0xf0], rcx
140a79c3f: f30f6f8588010000 movdqu xmm0, xmmword ptr [rbp + 0x188]
140a79c47: f30f6f8d98010000 movdqu xmm1, xmmword ptr [rbp + 0x198]
140a79c4f: 0f1095a8010000 movups xmm2, xmmword ptr [rbp + 0x1a8]
140a79c56: 0f109db8010000 movups xmm3, xmmword ptr [rbp + 0x1b8]
140a79c5d: 0f299de0000000 movaps xmmword ptr [rbp + 0xe0], xmm3
140a79c64: 0f2995d0000000 movaps xmmword ptr [rbp + 0xd0], xmm2
140a79c6b: 660f7f8dc0000000 movdqa xmmword ptr [rbp + 0xc0], xmm1
140a79c73: 660f7f85b0000000 movdqa xmmword ptr [rbp + 0xb0], xmm0
140a79c7b: 48898580010000 mov qword ptr [rbp + 0x180], rax
140a79c82: 660f6f85b0000000 movdqa xmm0, xmmword ptr [rbp + 0xb0]
140a79c8a: 660f6f8dc0000000 movdqa xmm1, xmmword ptr [rbp + 0xc0]
140a79c92: 0f2895d0000000 movaps xmm2, xmmword ptr [rbp + 0xd0]
140a79c99: 0f289de0000000 movaps xmm3, xmmword ptr [rbp + 0xe0]
140a79ca0: f30f7f8588010000 movdqu xmmword ptr [rbp + 0x188], xmm0
140a79ca8: f30f7f8d98010000 movdqu xmmword ptr [rbp + 0x198], xmm1
140a79cb0: 0f1195a8010000 movups xmmword ptr [rbp + 0x1a8], xmm2
140a79cb7: 0f119db8010000 movups xmmword ptr [rbp + 0x1b8], xmm3
140a79cbe: 488b85f0000000 mov rax, qword ptr [rbp + 0xf0]
140a79cc5: 488985c8010000 mov qword ptr [rbp + 0x1c8], rax
140a79ccc: 488d0575d10502 lea rax, [rip + 0x205d175]
140a79cd3: 4889442420 mov qword ptr [rsp + 0x20], rax
140a79cd8: 488d0dc1cc0502 lea rcx, [rip + 0x205ccc1] ; @142ad69a0 'called `Result::unwrap()` on an `Err` value'
140a79cdf: 4c8d0deacc0502 lea r9, [rip + 0x205ccea]
140a79ce6: 4c8d8580010000 lea r8, [rbp + 0x180]
140a79ced: ba2b000000 mov edx, 0x2b
140a79cf2: e869fae501 call 0x1428d9760 ; core::result::unwrap_failed
140a79cf7: 0f0b ud2 
140a79cf9: 0f1f8000000000 nop dword ptr [rax]
140a79d00: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a79d05: 55 push rbp
140a79d06: 4157 push r15
140a79d08: 4156 push r14
140a79d0a: 4155 push r13
140a79d0c: 4154 push r12
140a79d0e: 56 push rsi
140a79d0f: 57 push rdi
140a79d10: 53 push rbx
140a79d11: 4883ec68 sub rsp, 0x68
140a79d15: 488daa80000000 lea rbp, [rdx + 0x80]
140a79d1c: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a79d22: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a79d28: 488b9580010000 mov rdx, qword ptr [rbp + 0x180]
140a79d2f: 4885d2 test rdx, rdx
140a79d32: 7412 je 0x140a79d46
140a79d34: 488b8d88010000 mov rcx, qword ptr [rbp + 0x188]
140a79d3b: 41b801000000 mov r8d, 1
140a79d41: e8da9eb9ff call 0x140613c20 ; 
140a79d46: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a79d4b: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a79d50: 4883c468 add rsp, 0x68
140a79d54: 5b pop rbx
140a79d55: 5f pop rdi
140a79d56: 5e pop rsi
140a79d57: 415c pop r12
140a79d59: 415d pop r13
140a79d5b: 415e pop r14
140a79d5d: 415f pop r15
140a79d5f: 5d pop rbp
140a79d60: c3 ret 
140a79d61: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a79d70: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a79d75: 55 push rbp
140a79d76: 4157 push r15
140a79d78: 4156 push r14
140a79d7a: 4155 push r13
140a79d7c: 4154 push r12
140a79d7e: 56 push rsi
140a79d7f: 57 push rdi
140a79d80: 53 push rbx
140a79d81: 4883ec68 sub rsp, 0x68
140a79d85: 488daa80000000 lea rbp, [rdx + 0x80]
140a79d8c: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a79d92: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a79d98: 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
140a79d9f: 4885d2 test rdx, rdx
140a79da2: 7412 je 0x140a79db6
140a79da4: 488b8db8000000 mov rcx, qword ptr [rbp + 0xb8]
140a79dab: 41b801000000 mov r8d, 1
140a79db1: e86a9eb9ff call 0x140613c20 ; 
140a79db6: 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
140a79dbc: 660f6f7c2440 movdqa xmm7, xmmword ptr [rsp + 0x40]
140a79dc2: 4883c468 add rsp, 0x68
140a79dc6: 5b pop rbx
140a79dc7: 5f pop rdi
140a79dc8: 5e pop rsi
140a79dc9: 415c pop r12
140a79dcb: 415d pop r13
140a79dcd: 415e pop r14
140a79dcf: 415f pop r15
140a79dd1: 5d pop rbp
140a79dd2: c3 ret 
140a79dd3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a79de0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a79de5: 55 push rbp
140a79de6: 4157 push r15
140a79de8: 4156 push r14
140a79dea: 4155 push r13
140a79dec: 4154 push r12
140a79dee: 56 push rsi
140a79def: 57 push rdi
140a79df0: 53 push rbx
140a79df1: 4883ec68 sub rsp, 0x68
140a79df5: 488daa80000000 lea rbp, [rdx + 0x80]
140a79dfc: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a79e02: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a79e08: 488d8db0000000 lea rcx, [rbp + 0xb0]
140a79e0f: e8ecdbb6ff call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::option::Option<std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::option::Option<std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error>
140a79e14: 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
140a79e1a: 660f6f7c2440 movdqa xmm7, xmmword ptr [rsp + 0x40]
140a79e20: 4883c468 add rsp, 0x68
140a79e24: 5b pop rbx
140a79e25: 5f pop rdi
140a79e26: 5e pop rsi
140a79e27: 415c pop r12
140a79e29: 415d pop r13
140a79e2b: 415e pop r14
140a79e2d: 415f pop r15
140a79e2f: 5d pop rbp
140a79e30: c3 ret 
140a79e31: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a79e40: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a79e45: 55 push rbp
140a79e46: 4157 push r15
140a79e48: 4156 push r14
140a79e4a: 4155 push r13
140a79e4c: 4154 push r12
140a79e4e: 56 push rsi
140a79e4f: 57 push rdi
140a79e50: 53 push rbx
140a79e51: 4883ec68 sub rsp, 0x68
140a79e55: 488daa80000000 lea rbp, [rdx + 0x80]
140a79e5c: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a79e62: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a79e68: 4883bd1802000000 cmp qword ptr [rbp + 0x218], 0
140a79e70: 7419 je 0x140a79e8b
140a79e72: 41b801000000 mov r8d, 1
140a79e78: 488b8d08020000 mov rcx, qword ptr [rbp + 0x208]
140a79e7f: 488b9518020000 mov rdx, qword ptr [rbp + 0x218]
140a79e86: e8959db9ff call 0x140613c20 ; 
140a79e8b: 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
140a79e91: 660f6f7c2440 movdqa xmm7, xmmword ptr [rsp + 0x40]
140a79e97: 4883c468 add rsp, 0x68
140a79e9b: 5b pop rbx
140a79e9c: 5f pop rdi
140a79e9d: 5e pop rsi
140a79e9e: 415c pop r12
140a79ea0: 415d pop r13
140a79ea2: 415e pop r14
140a79ea4: 415f pop r15
140a79ea6: 5d pop rbp
140a79ea7: c3 ret 
140a79ea8: 0f1f840000000000 nop dword ptr [rax + rax]
140a79eb0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a79eb5: 55 push rbp
140a79eb6: 4157 push r15
140a79eb8: 4156 push r14
140a79eba: 4155 push r13
140a79ebc: 4154 push r12
140a79ebe: 56 push rsi
140a79ebf: 57 push rdi
140a79ec0: 53 push rbx
140a79ec1: 4883ec68 sub rsp, 0x68
140a79ec5: 488daa80000000 lea rbp, [rdx + 0x80]
140a79ecc: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a79ed2: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a79ed8: 0fb69d27020000 movzx ebx, byte ptr [rbp + 0x227]
140a79edf: 488b9528010000 mov rdx, qword ptr [rbp + 0x128]
140a79ee6: 4885d2 test rdx, rdx
140a79ee9: 7416 je 0x140a79f01
140a79eeb: 488b8d30010000 mov rcx, qword ptr [rbp + 0x130]
140a79ef2: 48c1e204 shl rdx, 4
140a79ef6: 41b804000000 mov r8d, 4
140a79efc: e81f9db9ff call 0x140613c20 ; 
140a79f01: f6c301 test bl, 1
140a79f04: 751b jne 0x140a79f21
140a79f06: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a79f0b: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a79f10: 4883c468 add rsp, 0x68
140a79f14: 5b pop rbx
140a79f15: 5f pop rdi
140a79f16: 5e pop rsi
140a79f17: 415c pop r12
140a79f19: 415d pop r13
140a79f1b: 415e pop r14
140a79f1d: 415f pop r15
140a79f1f: 5d pop rbp
140a79f20: c3 ret 
140a79f21: 488d4d30 lea rcx, [rbp + 0x30]
140a79f25: e866ede1ff call 0x140898c90 ; core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::vec::Vec<sentry_types::protocol::attachment::Attachment,alloc::alloc::Global> >
140a79f2a: ebda jmp 0x140a79f06
140a79f2c: 0f1f4000 nop dword ptr [rax]
140a79f30: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a79f35: 55 push rbp
140a79f36: 4157 push r15
140a79f38: 4156 push r14
140a79f3a: 4155 push r13
140a79f3c: 4154 push r12
140a79f3e: 56 push rsi
140a79f3f: 57 push rdi
140a79f40: 53 push rbx
140a79f41: 4883ec68 sub rsp, 0x68
140a79f45: 488daa80000000 lea rbp, [rdx + 0x80]
140a79f4c: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a79f52: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a79f58: 488bbd10020000 mov rdi, qword ptr [rbp + 0x210]
140a79f5f: 488b9d18020000 mov rbx, qword ptr [rbp + 0x218]
140a79f66: 4839fb cmp rbx, rdi
140a79f69: 488bb508020000 mov rsi, qword ptr [rbp + 0x208]
140a79f70: 7536 jne 0x140a79fa8
140a79f72: c6852702000001 mov byte ptr [rbp + 0x227], 1
140a79f79: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a79f7e: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a79f83: 4883c468 add rsp, 0x68
140a79f87: 5b pop rbx
140a79f88: 5f pop rdi
140a79f89: 5e pop rsi
140a79f8a: 415c pop r12
140a79f8c: 415d pop r13
140a79f8e: 415e pop r14
140a79f90: 415f pop r15
140a79f92: 5d pop rbp
140a79f93: c3 ret 
140a79f94: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a79fa0: 48ffc3 inc rbx
140a79fa3: 4839fb cmp rbx, rdi
140a79fa6: 74ca je 0x140a79f72
140a79fa8: 488d0c9b lea rcx, [rbx + rbx*4]
140a79fac: 48c1e104 shl rcx, 4
140a79fb0: 488b040e mov rax, qword ptr [rsi + rcx]
140a79fb4: f048ff08 lock dec qword ptr [rax]
140a79fb8: 75e6 jne 0x140a79fa0
140a79fba: 4801f1 add rcx, rsi
140a79fbd: c6852702000001 mov byte ptr [rbp + 0x227], 1
140a79fc4: e8674bbdff call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
140a79fc9: ebd5 jmp 0x140a79fa0
140a79fcb: 0f1f440000 nop dword ptr [rax + rax]
140a79fd0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a79fd5: 55 push rbp
140a79fd6: 4157 push r15
140a79fd8: 4156 push r14
140a79fda: 4155 push r13
140a79fdc: 4154 push r12
140a79fde: 56 push rsi
140a79fdf: 57 push rdi
140a79fe0: 53 push rbx
140a79fe1: 4883ec68 sub rsp, 0x68
140a79fe5: 488daa80000000 lea rbp, [rdx + 0x80]
140a79fec: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
140a79ff1: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
140a79ff6: 488d4d78 lea rcx, [rbp + 0x78]
140a79ffa: e831f2dfff call 0x140879230 ; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >
140a79fff: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a7a006: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a7a00b: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a7a010: 4883c468 add rsp, 0x68
140a7a014: 5b pop rbx
140a7a015: 5f pop rdi
140a7a016: 5e pop rsi
140a7a017: 415c pop r12
140a7a019: 415d pop r13
140a7a01b: 415e pop r14
140a7a01d: 415f pop r15
140a7a01f: 5d pop rbp
140a7a020: c3 ret 
140a7a021: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a7a030: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7a035: 55 push rbp
140a7a036: 4157 push r15
140a7a038: 4156 push r14
140a7a03a: 4155 push r13
140a7a03c: 4154 push r12
140a7a03e: 56 push rsi
140a7a03f: 57 push rdi
140a7a040: 53 push rbx
140a7a041: 4883ec68 sub rsp, 0x68
140a7a045: 488daa80000000 lea rbp, [rdx + 0x80]
140a7a04c: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a7a052: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a7a058: 488b8d18020000 mov rcx, qword ptr [rbp + 0x218]
140a7a05f: 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
140a7a066: 4885d2 test rdx, rdx
140a7a069: 740b je 0x140a7a076
140a7a06b: 41b801000000 mov r8d, 1
140a7a071: e8aa9bb9ff call 0x140613c20 ; 
140a7a076: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a7a07d: 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
140a7a083: 660f6f7c2440 movdqa xmm7, xmmword ptr [rsp + 0x40]
140a7a089: 4883c468 add rsp, 0x68
140a7a08d: 5b pop rbx
140a7a08e: 5f pop rdi
140a7a08f: 5e pop rsi
140a7a090: 415c pop r12
140a7a092: 415d pop r13
140a7a094: 415e pop r14
140a7a096: 415f pop r15
140a7a098: 5d pop rbp
140a7a099: c3 ret 
140a7a09a: 660f1f440000 nop word ptr [rax + rax]
140a7a0a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7a0a5: 55 push rbp
140a7a0a6: 4157 push r15
140a7a0a8: 4156 push r14
140a7a0aa: 4155 push r13
140a7a0ac: 4154 push r12
140a7a0ae: 56 push rsi
140a7a0af: 57 push rdi
140a7a0b0: 53 push rbx
140a7a0b1: 4883ec68 sub rsp, 0x68
140a7a0b5: 488daa80000000 lea rbp, [rdx + 0x80]
140a7a0bc: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a7a0c2: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a7a0c8: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a7a0cf: 488d8db0000000 lea rcx, [rbp + 0xb0]
140a7a0d6: e8552b2d00 call 0x140d4cc30 ; anyhow::error::impl$6::drop
140a7a0db: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a7a0e2: 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
140a7a0e8: 660f6f7c2440 movdqa xmm7, xmmword ptr [rsp + 0x40]
140a7a0ee: 4883c468 add rsp, 0x68
140a7a0f2: 5b pop rbx
140a7a0f3: 5f pop rdi
140a7a0f4: 5e pop rsi
140a7a0f5: 415c pop r12
140a7a0f7: 415d pop r13
140a7a0f9: 415e pop r14
140a7a0fb: 415f pop r15
140a7a0fd: 5d pop rbp
140a7a0fe: c3 ret 
140a7a0ff: 90 nop 
140a7a100: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7a105: 55 push rbp
140a7a106: 4157 push r15
140a7a108: 4156 push r14
140a7a10a: 4155 push r13
140a7a10c: 4154 push r12
140a7a10e: 56 push rsi
140a7a10f: 57 push rdi
140a7a110: 53 push rbx
140a7a111: 4883ec68 sub rsp, 0x68
140a7a115: 488daa80000000 lea rbp, [rdx + 0x80]
140a7a11c: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a7a122: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a7a128: 0fb69d26020000 movzx ebx, byte ptr [rbp + 0x226]
140a7a12f: 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
140a7a136: 4885d2 test rdx, rdx
140a7a139: 7536 jne 0x140a7a171
140a7a13b: 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
140a7a142: 4885d2 test rdx, rdx
140a7a145: 488bb570010000 mov rsi, qword ptr [rbp + 0x170]
140a7a14c: 7548 jne 0x140a7a196
140a7a14e: 4889f0 mov rax, rsi
140a7a151: 48f7d8 neg rax
140a7a154: 715a jno 0x140a7a1b0
140a7a156: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a7a15b: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a7a160: 4883c468 add rsp, 0x68
140a7a164: 5b pop rbx
140a7a165: 5f pop rdi
140a7a166: 5e pop rsi
140a7a167: 415c pop r12
140a7a169: 415d pop r13
140a7a16b: 415e pop r14
140a7a16d: 415f pop r15
140a7a16f: 5d pop rbp
140a7a170: c3 ret 
140a7a171: 41b801000000 mov r8d, 1
140a7a177: 488b8d10020000 mov rcx, qword ptr [rbp + 0x210]
140a7a17e: e89d9ab9ff call 0x140613c20 ; 
140a7a183: 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
140a7a18a: 4885d2 test rdx, rdx
140a7a18d: 488bb570010000 mov rsi, qword ptr [rbp + 0x170]
140a7a194: 74b8 je 0x140a7a14e
140a7a196: 41b801000000 mov r8d, 1
140a7a19c: 488b8d40010000 mov rcx, qword ptr [rbp + 0x140]
140a7a1a3: e8789ab9ff call 0x140613c20 ; 
140a7a1a8: 4889f0 mov rax, rsi
140a7a1ab: 48f7d8 neg rax
140a7a1ae: 70a6 jo 0x140a7a156
140a7a1b0: f6c301 test bl, 1
140a7a1b3: 7435 je 0x140a7a1ea
140a7a1b5: 4885f6 test rsi, rsi
140a7a1b8: 7415 je 0x140a7a1cf
140a7a1ba: 41b801000000 mov r8d, 1
140a7a1c0: 488b8d78010000 mov rcx, qword ptr [rbp + 0x178]
140a7a1c7: 4889f2 mov rdx, rsi
140a7a1ca: e8519ab9ff call 0x140613c20 ; 
140a7a1cf: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a7a1d4: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a7a1d9: 4883c468 add rsp, 0x68
140a7a1dd: 5b pop rbx
140a7a1de: 5f pop rdi
140a7a1df: 5e pop rsi
140a7a1e0: 415c pop r12
140a7a1e2: 415d pop r13
140a7a1e4: 415e pop r14
140a7a1e6: 415f pop r15
140a7a1e8: 5d pop rbp
140a7a1e9: c3 ret 
140a7a1ea: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a7a1ef: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a7a1f4: 4883c468 add rsp, 0x68
140a7a1f8: 5b pop rbx
140a7a1f9: 5f pop rdi
140a7a1fa: 5e pop rsi
140a7a1fb: 415c pop r12
140a7a1fd: 415d pop r13
140a7a1ff: 415e pop r14
140a7a201: 415f pop r15
140a7a203: 5d pop rbp
140a7a204: c3 ret 
140a7a205: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a7a210: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7a215: 55 push rbp
140a7a216: 4157 push r15
140a7a218: 4156 push r14
140a7a21a: 4155 push r13
140a7a21c: 4154 push r12
140a7a21e: 56 push rsi
140a7a21f: 57 push rdi
140a7a220: 53 push rbx
140a7a221: 4883ec68 sub rsp, 0x68
140a7a225: 488daa80000000 lea rbp, [rdx + 0x80]
140a7a22c: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a7a232: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a7a238: 0fb69d25020000 movzx ebx, byte ptr [rbp + 0x225]
140a7a23f: 488b8550010000 mov rax, qword ptr [rbp + 0x150]
140a7a246: 4885c0 test rax, rax
140a7a249: 741a je 0x140a7a265
140a7a24b: 488b8d58010000 mov rcx, qword ptr [rbp + 0x158]
140a7a252: 48c1e004 shl rax, 4
140a7a256: 488d1440 lea rdx, [rax + rax*2]
140a7a25a: 41b804000000 mov r8d, 4
140a7a260: e8bb99b9ff call 0x140613c20 ; 
140a7a265: 889d26020000 mov byte ptr [rbp + 0x226], bl
140a7a26b: 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
140a7a271: 660f6f7c2440 movdqa xmm7, xmmword ptr [rsp + 0x40]
140a7a277: 4883c468 add rsp, 0x68
140a7a27b: 5b pop rbx
140a7a27c: 5f pop rdi
140a7a27d: 5e pop rsi
140a7a27e: 415c pop r12
140a7a280: 415d pop r13
140a7a282: 415e pop r14
140a7a284: 415f pop r15
140a7a286: 5d pop rbp
140a7a287: c3 ret 
140a7a288: 0f1f840000000000 nop dword ptr [rax + rax]
140a7a290: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7a295: 55 push rbp
140a7a296: 4157 push r15
140a7a298: 4156 push r14
140a7a29a: 4155 push r13
140a7a29c: 4154 push r12
140a7a29e: 56 push rsi
140a7a29f: 57 push rdi
140a7a2a0: 53 push rbx
140a7a2a1: 4883ec68 sub rsp, 0x68
140a7a2a5: 488daa80000000 lea rbp, [rdx + 0x80]
140a7a2ac: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a7a2b2: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a7a2b8: 0fb69d24020000 movzx ebx, byte ptr [rbp + 0x224]
140a7a2bf: 488b4560 mov rax, qword ptr [rbp + 0x60]
140a7a2c3: f048ff08 lock dec qword ptr [rax]
140a7a2c7: 7509 jne 0x140a7a2d2
140a7a2c9: 488d4d60 lea rcx, [rbp + 0x60]
140a7a2cd: e85e48bdff call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
140a7a2d2: 889d25020000 mov byte ptr [rbp + 0x225], bl
140a7a2d8: 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
140a7a2de: 660f6f7c2440 movdqa xmm7, xmmword ptr [rsp + 0x40]
140a7a2e4: 4883c468 add rsp, 0x68
140a7a2e8: 5b pop rbx
140a7a2e9: 5f pop rdi
140a7a2ea: 5e pop rsi
140a7a2eb: 415c pop r12
140a7a2ed: 415d pop r13
140a7a2ef: 415e pop r14
140a7a2f1: 415f pop r15
140a7a2f3: 5d pop rbp
140a7a2f4: c3 ret 
140a7a2f5: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a7a300: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7a305: 55 push rbp
140a7a306: 4157 push r15
140a7a308: 4156 push r14
140a7a30a: 4155 push r13
140a7a30c: 4154 push r12
140a7a30e: 56 push rsi
140a7a30f: 57 push rdi
140a7a310: 53 push rbx
140a7a311: 4883ec68 sub rsp, 0x68
140a7a315: 488daa80000000 lea rbp, [rdx + 0x80]
140a7a31c: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a7a322: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a7a328: 488b9570010000 mov rdx, qword ptr [rbp + 0x170]
140a7a32f: 4885d2 test rdx, rdx
140a7a332: 7412 je 0x140a7a346
140a7a334: 41b801000000 mov r8d, 1
140a7a33a: 488b8d78010000 mov rcx, qword ptr [rbp + 0x178]
140a7a341: e8da98b9ff call 0x140613c20 ; 
140a7a346: c6852402000000 mov byte ptr [rbp + 0x224], 0
140a7a34d: 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
140a7a353: 660f6f7c2440 movdqa xmm7, xmmword ptr [rsp + 0x40]
140a7a359: 4883c468 add rsp, 0x68
140a7a35d: 5b pop rbx
140a7a35e: 5f pop rdi
140a7a35f: 5e pop rsi
140a7a360: 415c pop r12
140a7a362: 415d pop r13
140a7a364: 415e pop r14
140a7a366: 415f pop r15
140a7a368: 5d pop rbp
140a7a369: c3 ret 
140a7a36a: 660f1f440000 nop word ptr [rax + rax]
140a7a370: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7a375: 55 push rbp
140a7a376: 4157 push r15
140a7a378: 4156 push r14
140a7a37a: 4155 push r13
140a7a37c: 4154 push r12
140a7a37e: 56 push rsi
140a7a37f: 57 push rdi
140a7a380: 53 push rbx
140a7a381: 4883ec68 sub rsp, 0x68
140a7a385: 488daa80000000 lea rbp, [rdx + 0x80]
140a7a38c: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a7a392: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a7a398: 488d8db0000000 lea rcx, [rbp + 0xb0]
140a7a39f: e88ce7e0ff call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
140a7a3a4: 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
140a7a3aa: 660f6f7c2440 movdqa xmm7, xmmword ptr [rsp + 0x40]
140a7a3b0: 4883c468 add rsp, 0x68
140a7a3b4: 5b pop rbx
140a7a3b5: 5f pop rdi
140a7a3b6: 5e pop rsi
140a7a3b7: 415c pop r12
140a7a3b9: 415d pop r13
140a7a3bb: 415e pop r14
140a7a3bd: 415f pop r15
140a7a3bf: 5d pop rbp
140a7a3c0: c3 ret 
140a7a3c1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a7a3d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7a3d5: 55 push rbp
140a7a3d6: 4157 push r15
140a7a3d8: 4156 push r14
140a7a3da: 4155 push r13
140a7a3dc: 4154 push r12
140a7a3de: 56 push rsi
140a7a3df: 57 push rdi
140a7a3e0: 53 push rbx
140a7a3e1: 4883ec68 sub rsp, 0x68
140a7a3e5: 488daa80000000 lea rbp, [rdx + 0x80]
140a7a3ec: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a7a3f2: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a7a3f8: 488b4508 mov rax, qword ptr [rbp + 8]
140a7a3fc: 4885c0 test rax, rax
140a7a3ff: 7417 je 0x140a7a418
140a7a401: 488b4d10 mov rcx, qword ptr [rbp + 0x10]
140a7a405: 48c1e002 shl rax, 2
140a7a409: 488d1440 lea rdx, [rax + rax*2]
140a7a40d: 41b804000000 mov r8d, 4
140a7a413: e80898b9ff call 0x140613c20 ; 
140a7a418: 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
140a7a41e: 660f6f7c2440 movdqa xmm7, xmmword ptr [rsp + 0x40]
140a7a424: 4883c468 add rsp, 0x68
140a7a428: 5b pop rbx
140a7a429: 5f pop rdi
140a7a42a: 5e pop rsi
140a7a42b: 415c pop r12
140a7a42d: 415d pop r13
140a7a42f: 415e pop r14
140a7a431: 415f pop r15
140a7a433: 5d pop rbp
140a7a434: c3 ret 
140a7a435: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a7a440: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7a445: 55 push rbp
140a7a446: 4157 push r15
140a7a448: 4156 push r14
140a7a44a: 4155 push r13
140a7a44c: 4154 push r12
140a7a44e: 56 push rsi
140a7a44f: 57 push rdi
140a7a450: 53 push rbx
140a7a451: 4883ec68 sub rsp, 0x68
140a7a455: 488daa80000000 lea rbp, [rdx + 0x80]
140a7a45c: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a7a462: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a7a468: 488d8db0000000 lea rcx, [rbp + 0xb0]
140a7a46f: e8bc272d00 call 0x140d4cc30 ; anyhow::error::impl$6::drop
140a7a474: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a7a479: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a7a47e: 4883c468 add rsp, 0x68
140a7a482: 5b pop rbx
140a7a483: 5f pop rdi
140a7a484: 5e pop rsi
140a7a485: 415c pop r12
140a7a487: 415d pop r13
140a7a489: 415e pop r14
140a7a48b: 415f pop r15
140a7a48d: 5d pop rbp
140a7a48e: c3 ret 
140a7a48f: 90 nop 
140a7a490: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7a495: 55 push rbp
140a7a496: 4157 push r15
140a7a498: 4156 push r14
140a7a49a: 4155 push r13
140a7a49c: 4154 push r12
140a7a49e: 56 push rsi
140a7a49f: 57 push rdi
140a7a4a0: 53 push rbx
140a7a4a1: 4883ec68 sub rsp, 0x68
140a7a4a5: 488daa80000000 lea rbp, [rdx + 0x80]
140a7a4ac: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a7a4b2: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a7a4b8: 488d8db0000000 lea rcx, [rbp + 0xb0]
140a7a4bf: e8dce3b6ff call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error>
140a7a4c4: 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
140a7a4ca: 660f6f7c2440 movdqa xmm7, xmmword ptr [rsp + 0x40]
140a7a4d0: 4883c468 add rsp, 0x68
140a7a4d4: 5b pop rbx
140a7a4d5: 5f pop rdi
140a7a4d6: 5e pop rsi
140a7a4d7: 415c pop r12
140a7a4d9: 415d pop r13
140a7a4db: 415e pop r14
140a7a4dd: 415f pop r15
140a7a4df: 5d pop rbp
140a7a4e0: c3 ret 
140a7a4e1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a7a4f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7a4f5: 55 push rbp
140a7a4f6: 4157 push r15
140a7a4f8: 4156 push r14
140a7a4fa: 4155 push r13
140a7a4fc: 4154 push r12
140a7a4fe: 56 push rsi
140a7a4ff: 57 push rdi
140a7a500: 53 push rbx
140a7a501: 4883ec68 sub rsp, 0x68
140a7a505: 488daa80000000 lea rbp, [rdx + 0x80]
140a7a50c: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a7a512: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a7a518: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a7a51f: 488d8d80010000 lea rcx, [rbp + 0x180]
140a7a526: e8e572d3ff call 0x1407b1810 ; core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> > | core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> > | core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> > | core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> > | core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> > | core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> > | core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> > | core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> > | core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> > | core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> > | core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> > | core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> > | core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> > | core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> >
140a7a52b: c6852702000000 mov byte ptr [rbp + 0x227], 0
140a7a532: 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
140a7a538: 660f6f7c2440 movdqa xmm7, xmmword ptr [rsp + 0x40]
140a7a53e: 4883c468 add rsp, 0x68
140a7a542: 5b pop rbx
140a7a543: 5f pop rdi
140a7a544: 5e pop rsi
140a7a545: 415c pop r12
140a7a547: 415d pop r13
140a7a549: 415e pop r14
140a7a54b: 415f pop r15
140a7a54d: 5d pop rbp
140a7a54e: c3 ret 
140a7a54f: 90 nop 
140a7a550: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7a555: 55 push rbp
140a7a556: 4157 push r15
140a7a558: 4156 push r14
140a7a55a: 4155 push r13
140a7a55c: 4154 push r12
140a7a55e: 56 push rsi
140a7a55f: 57 push rdi
140a7a560: 53 push rbx
140a7a561: 4883ec68 sub rsp, 0x68
140a7a565: 488daa80000000 lea rbp, [rdx + 0x80]
140a7a56c: 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
140a7a572: 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
140a7a578: 488d8db0000000 lea rcx, [rbp + 0xb0]
140a7a57f: e8ac262d00 call 0x140d4cc30 ; anyhow::error::impl$6::drop
140a7a584: 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
140a7a58a: 660f6f7c2440 movdqa xmm7, xmmword ptr [rsp + 0x40]
140a7a590: 4883c468 add rsp, 0x68
140a7a594: 5b pop rbx
140a7a595: 5f pop rdi
140a7a596: 5e pop rsi
140a7a597: 415c pop r12
140a7a599: 415d pop r13
140a7a59b: 415e pop r14
140a7a59d: 415f pop r15
140a7a59f: 5d pop rbp
140a7a5a0: c3 ret 

141bf16c0: 55 push rbp
141bf16c1: 4157 push r15
141bf16c3: 4156 push r14
141bf16c5: 4155 push r13
141bf16c7: 4154 push r12
141bf16c9: 56 push rsi
141bf16ca: 57 push rdi
141bf16cb: 53 push rbx
141bf16cc: 4881ec88020000 sub rsp, 0x288
141bf16d3: 488dac2480000000 lea rbp, [rsp + 0x80]
141bf16db: 440f29bdf0010000 movaps xmmword ptr [rbp + 0x1f0], xmm15
141bf16e3: 440f29b5e0010000 movaps xmmword ptr [rbp + 0x1e0], xmm14
141bf16eb: 440f29add0010000 movaps xmmword ptr [rbp + 0x1d0], xmm13
141bf16f3: 440f29a5c0010000 movaps xmmword ptr [rbp + 0x1c0], xmm12
141bf16fb: 440f299db0010000 movaps xmmword ptr [rbp + 0x1b0], xmm11
141bf1703: 440f2995a0010000 movaps xmmword ptr [rbp + 0x1a0], xmm10
141bf170b: 440f298d90010000 movaps xmmword ptr [rbp + 0x190], xmm9
141bf1713: 440f298580010000 movaps xmmword ptr [rbp + 0x180], xmm8
141bf171b: 0f29bd70010000 movaps xmmword ptr [rbp + 0x170], xmm7
141bf1722: 0f29b560010000 movaps xmmword ptr [rbp + 0x160], xmm6
141bf1729: 48c78558010000feffffff mov qword ptr [rbp + 0x158], 0xfffffffffffffffe
141bf1734: 4c898d30010000 mov qword ptr [rbp + 0x130], r9
141bf173b: 4c898540010000 mov qword ptr [rbp + 0x140], r8
141bf1742: 4889d7 mov rdi, rdx
141bf1745: e8b65202ff call 0x140c16a00 ; puffin::are_scopes_on
141bf174a: 888557010000 mov byte ptr [rbp + 0x157], al
141bf1750: 84c0 test al, al
141bf1752: 745c je 0x141bf17b0
141bf1754: bb30000000 mov ebx, 0x30
141bf1759: b930000000 mov ecx, 0x30
141bf175e: e82dfbffff call 0x141bf1290 ; core::str::impl$0::rfind<ref$<str$> >
141bf1763: 4c8d35febe3001 lea r14, [rip + 0x130befe] ; @142efd668 'forest::manage_tree_prefabs::manage_tree_prefabs::f'
141bf176a: 4883f801 cmp rax, 1
141bf176e: 0f85a7000000 jne 0x141bf181b
141bf1774: 4989d1 mov r9, rdx
141bf1777: 4885d2 test rdx, rdx
141bf177a: 743d je 0x141bf17b9
141bf177c: 4983f930 cmp r9, 0x30
141bf1780: 7335 jae 0x141bf17b7
141bf1782: 488d05dfbe3001 lea rax, [rip + 0x130bedf] ; @142efd668 'forest::manage_tree_prefabs::manage_tree_prefabs::f'
141bf1789: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
141bf178e: 7f29 jg 0x141bf17b9
141bf1790: 488d0589bf3001 lea rax, [rip + 0x130bf89]
141bf1797: 4889442420 mov qword ptr [rsp + 0x20], rax
141bf179c: 488d0dc5be3001 lea rcx, [rip + 0x130bec5] ; @142efd668 'forest::manage_tree_prefabs::manage_tree_prefabs::f'
141bf17a3: ba30000000 mov edx, 0x30
141bf17a8: 4531c0 xor r8d, r8d
141bf17ab: e89081ce00 call 0x1428d9940 ; core::str::slice_error_fail
141bf17b0: 31c0 xor eax, eax
141bf17b2: e9c1010000 jmp 0x141bf1978
141bf17b7: 75d7 jne 0x141bf1790
141bf17b9: 4c89c9 mov rcx, r9
141bf17bc: e8cffaffff call 0x141bf1290 ; core::str::impl$0::rfind<ref$<str$> >
141bf17c1: 4883f801 cmp rax, 1
141bf17c5: 7554 jne 0x141bf181b
141bf17c7: 4989d0 mov r8, rdx
141bf17ca: 4983c002 add r8, 2
141bf17ce: 7439 je 0x141bf1809
141bf17d0: 4983f830 cmp r8, 0x30
141bf17d4: 7331 jae 0x141bf1807
141bf17d6: 488d058bbe3001 lea rax, [rip + 0x130be8b] ; @142efd668 'forest::manage_tree_prefabs::manage_tree_prefabs::f'
141bf17dd: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
141bf17e2: 7f25 jg 0x141bf1809
141bf17e4: 488d054dbf3001 lea rax, [rip + 0x130bf4d]
141bf17eb: 4889442420 mov qword ptr [rsp + 0x20], rax
141bf17f0: 488d0d71be3001 lea rcx, [rip + 0x130be71] ; @142efd668 'forest::manage_tree_prefabs::manage_tree_prefabs::f'
141bf17f7: ba30000000 mov edx, 0x30
141bf17fc: 41b930000000 mov r9d, 0x30
141bf1802: e83981ce00 call 0x1428d9940 ; core::str::slice_error_fail
141bf1807: 75db jne 0x141bf17e4
141bf1809: bb2e000000 mov ebx, 0x2e
141bf180e: 4829d3 sub rbx, rdx
141bf1811: 4c8d3550be3001 lea r14, [rip + 0x130be50] ; @142efd668 'forest::manage_tree_prefabs::manage_tree_prefabs::f'
141bf1818: 4d01c6 add r14, r8
141bf181b: 4c8d0576bf3001 lea r8, [rip + 0x130bf76]
141bf1822: 4c8d3d3fbf3001 lea r15, [rip + 0x130bf3f]
141bf1829: 0f1f8000000000 nop dword ptr [rax]
141bf1830: 410fbe40ff movsx eax, byte ptr [r8 - 1]
141bf1835: 85c0 test eax, eax
141bf1837: 7817 js 0x141bf1850
141bf1839: 49ffc8 dec r8
141bf183c: 83f85c cmp eax, 0x5c
141bf183f: 7563 jne 0x141bf18a4
141bf1841: eb72 jmp 0x141bf18b5
141bf1843: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141bf1850: 410fb648fe movzx ecx, byte ptr [r8 - 2]
141bf1855: 80f9c0 cmp cl, 0xc0
141bf1858: 7d23 jge 0x141bf187d
141bf185a: 410fb650fd movzx edx, byte ptr [r8 - 3]
141bf185f: 80fac0 cmp dl, 0xc0
141bf1862: 7d22 jge 0x141bf1886
141bf1864: 450fb648fc movzx r9d, byte ptr [r8 - 4]
141bf1869: 4983c0fc add r8, -4
141bf186d: 4183e107 and r9d, 7
141bf1871: 41c1e106 shl r9d, 6
141bf1875: 83e23f and edx, 0x3f
141bf1878: 4409ca or edx, r9d
141bf187b: eb10 jmp 0x141bf188d
141bf187d: 4983c0fe add r8, -2
141bf1881: 83e11f and ecx, 0x1f
141bf1884: eb0f jmp 0x141bf1895
141bf1886: 4983c0fd add r8, -3
141bf188a: 83e20f and edx, 0xf
141bf188d: c1e206 shl edx, 6
141bf1890: 83e13f and ecx, 0x3f
141bf1893: 09d1 or ecx, edx
141bf1895: c1e106 shl ecx, 6
141bf1898: 243f and al, 0x3f
141bf189a: 0fb6c0 movzx eax, al
141bf189d: 09c8 or eax, ecx
141bf189f: 83f85c cmp eax, 0x5c
141bf18a2: 7411 je 0x141bf18b5
141bf18a4: 83f82f cmp eax, 0x2f
141bf18a7: 740c je 0x141bf18b5
141bf18a9: 4d39f8 cmp r8, r15
141bf18ac: 7582 jne 0x141bf1830
141bf18ae: be30000000 mov esi, 0x30
141bf18b3: eb4c jmp 0x141bf1901
141bf18b5: 4c8d3dacbe3001 lea r15, [rip + 0x130beac]
141bf18bc: 4d29f8 sub r8, r15
141bf18bf: 49ffc0 inc r8
141bf18c2: 7432 je 0x141bf18f6
141bf18c4: 4983f830 cmp r8, 0x30
141bf18c8: 732a jae 0x141bf18f4
141bf18ca: 43803c38bf cmp byte ptr [r8 + r15], 0xbf
141bf18cf: 7f25 jg 0x141bf18f6
141bf18d1: 488d0528be3001 lea rax, [rip + 0x130be28]
141bf18d8: 4889442420 mov qword ptr [rsp + 0x20], rax
141bf18dd: 488d0d84be3001 lea rcx, [rip + 0x130be84]
141bf18e4: ba30000000 mov edx, 0x30
141bf18e9: 41b930000000 mov r9d, 0x30
141bf18ef: e84c80ce00 call 0x1428d9940 ; core::str::slice_error_fail
141bf18f4: 75db jne 0x141bf18d1
141bf18f6: be30000000 mov esi, 0x30
141bf18fb: 4c29c6 sub rsi, r8
141bf18fe: 4d01c7 add r15, r8
141bf1901: e82a6802ff call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
141bf1906: 4889c1 mov rcx, rax
141bf1909: 488b00 mov rax, qword ptr [rax]
141bf190c: 4883f801 cmp rax, 1
141bf1910: 0f852e0d0000 jne 0x141bf2644
141bf1916: 4883c108 add rcx, 8
141bf191a: 48833900 cmp qword ptr [rcx], 0
141bf191e: 0f858f0d0000 jne 0x141bf26b3
141bf1924: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
141bf192b: 48898d48010000 mov qword ptr [rbp + 0x148], rcx
141bf1932: 4883c108 add rcx, 8
141bf1936: 4889742420 mov qword ptr [rsp + 0x20], rsi
141bf193b: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
141bf1944: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
141bf194d: 4c89f2 mov rdx, r14
141bf1950: 4989d8 mov r8, rbx
141bf1953: 4d89f9 mov r9, r15
141bf1956: e8e55302ff call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
141bf195b: 488b8d48010000 mov rcx, qword ptr [rbp + 0x148]
141bf1962: 48ff01 inc qword ptr [rcx]
141bf1965: 488985a0000000 mov qword ptr [rbp + 0xa0], rax
141bf196c: 48898580000000 mov qword ptr [rbp + 0x80], rax
141bf1973: b801000000 mov eax, 1
141bf1978: 4c8bbdb8020000 mov r15, qword ptr [rbp + 0x2b8]
141bf197f: 488b8da8020000 mov rcx, qword ptr [rbp + 0x2a8]
141bf1986: 48894578 mov qword ptr [rbp + 0x78], rax
141bf198a: 49c7471000000000 mov qword ptr [r15 + 0x10], 0
141bf1992: 488b09 mov rcx, qword ptr [rcx]
141bf1995: e8e6c3a8fe call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut | toolbar::mei::widgets::ActionHintView::as_inner_mut | system_decorator::decorator_visual_state::DecoratorPersistentVisualState::entities | country_core::resources::walls::decorator::DecoratorId::new | country_core::resources::walls::decorator::DeferredDecoratorOpEvents::events | country_core::resources::world_raster_defs::impl$45::deref_mut | country_core::resources::world_raster_defs::impl$69::deref | country_core::resources::image_formation::ImageFormationConfigs::initial | country_core::resources::rt::RtBvhBuilderGlueResource::new | country_core::systems::collision::world::SubworldIndex::from_usize_unchecked | country_core::systems::ui::resources::ui_state::UiState::widget | country_core::resources::gui::impl$0::deref | country_core::resources::gui::PendingEguiContextResource::into_egui_context_resource | country_core::render::impl$1::deref | country_core::systems::steam::AppSteamIntegration::client | image::traits::impl$0::as_bytes | rhapsody::raw::buffer::impl$0::as_managed_handle | rhapsody::raw::shader::impl$2::deref | rhapsody::raw::shader::impl$3::deref | rhapsody::rg::executor::PassPgoNameId::from_name | rhapsody::raw::allocator::GpuAllocator::inner | gpu_profiler::shared::NanoSecond::from_raw_ns | utils::swizzlers::impl$5::as_vec2 | log::__private_api::loc | country_core::resources::walls::wall_holes::FinalizedHoleStorage::inner | utils::geometry::intersect::Triangle::points_mut | asset::prefab::impl$4::as_ref | country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo> | country_core::asset_libraries::ssbo_library::impl$0::buffer<country_core::geometry::instanced_wall::BrickDrawListSsbo> | parry3d::shape::trimesh::impl$8::qbvh | parry3d::shape::trimesh::impl$8::qbvh | bevy_ptr::dangling_with_align | event_listener::notify::impl$33::into_notification | flume::signal::impl$1::as_ptr | cpp_demangle::ast::impl$133::downcast_to_pointer_to_member<alloc::string::String> | cpp_demangle::ast::impl$93::downcast_to_type<alloc::string::String> | cpp_demangle::ast::impl$127::downcast_to_array_type<alloc::string::String> | motomoto::Reader::new | country_core::asset_libraries::ssbo_library::impl$0::buffer<country_core::resources::color_id::GateColorIdSsbo> | country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
141bf199a: 488b95a0020000 mov rdx, qword ptr [rbp + 0x2a0]
141bf19a1: 488d4808 lea rcx, [rax + 8]
141bf19a5: 4c8d4020 lea r8, [rax + 0x20]
141bf19a9: 4c8d4810 lea r9, [rax + 0x10]
141bf19ad: 4989c2 mov r10, rax
141bf19b0: 4983c228 add r10, 0x28
141bf19b4: 80b89802000000 cmp byte ptr [rax + 0x298], 0
141bf19bb: 490f45c8 cmovne rcx, r8
141bf19bf: 4d0f44d1 cmove r10, r9
141bf19c3: 498b02 mov rax, qword ptr [r10]
141bf19c6: 488b09 mov rcx, qword ptr [rcx]
141bf19c9: 48894d40 mov qword ptr [rbp + 0x40], rcx
141bf19cd: 48894548 mov qword ptr [rbp + 0x48], rax
141bf19d1: 488d4db8 lea rcx, [rbp - 0x48]
141bf19d5: e84632effe call 0x140ae4c20 ; country_core::systems::paths::draw_path::PathRastersResources::as_float_grid
141bf19da: 488b85b0020000 mov rax, qword ptr [rbp + 0x2b0]
141bf19e1: 488b08 mov rcx, qword ptr [rax]
141bf19e4: e897c3a8fe call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut | toolbar::mei::widgets::ActionHintView::as_inner_mut | system_decorator::decorator_visual_state::DecoratorPersistentVisualState::entities | country_core::resources::walls::decorator::DecoratorId::new | country_core::resources::walls::decorator::DeferredDecoratorOpEvents::events | country_core::resources::world_raster_defs::impl$45::deref_mut | country_core::resources::world_raster_defs::impl$69::deref | country_core::resources::image_formation::ImageFormationConfigs::initial | country_core::resources::rt::RtBvhBuilderGlueResource::new | country_core::systems::collision::world::SubworldIndex::from_usize_unchecked | country_core::systems::ui::resources::ui_state::UiState::widget | country_core::resources::gui::impl$0::deref | country_core::resources::gui::PendingEguiContextResource::into_egui_context_resource | country_core::render::impl$1::deref | country_core::systems::steam::AppSteamIntegration::client | image::traits::impl$0::as_bytes | rhapsody::raw::buffer::impl$0::as_managed_handle | rhapsody::raw::shader::impl$2::deref | rhapsody::raw::shader::impl$3::deref | rhapsody::rg::executor::PassPgoNameId::from_name | rhapsody::raw::allocator::GpuAllocator::inner | gpu_profiler::shared::NanoSecond::from_raw_ns | utils::swizzlers::impl$5::as_vec2 | log::__private_api::loc | country_core::resources::walls::wall_holes::FinalizedHoleStorage::inner | utils::geometry::intersect::Triangle::points_mut | asset::prefab::impl$4::as_ref | country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo> | country_core::asset_libraries::ssbo_library::impl$0::buffer<country_core::geometry::instanced_wall::BrickDrawListSsbo> | parry3d::shape::trimesh::impl$8::qbvh | parry3d::shape::trimesh::impl$8::qbvh | bevy_ptr::dangling_with_align | event_listener::notify::impl$33::into_notification | flume::signal::impl$1::as_ptr | cpp_demangle::ast::impl$133::downcast_to_pointer_to_member<alloc::string::String> | cpp_demangle::ast::impl$93::downcast_to_type<alloc::string::String> | cpp_demangle::ast::impl$127::downcast_to_array_type<alloc::string::String> | motomoto::Reader::new | country_core::asset_libraries::ssbo_library::impl$0::buffer<country_core::resources::color_id::GateColorIdSsbo> | country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
141bf19e9: 488d4808 lea rcx, [rax + 8]
141bf19ed: 488d5020 lea rdx, [rax + 0x20]
141bf19f1: 4989c0 mov r8, rax
141bf19f4: 4983c028 add r8, 0x28
141bf19f8: 80b89802000000 cmp byte ptr [rax + 0x298], 0
141bf19ff: 480f45ca cmovne rcx, rdx
141bf1a03: 488d4010 lea rax, [rax + 0x10]
141bf1a07: 4c0f44c0 cmove r8, rax
141bf1a0b: 498b00 mov rax, qword ptr [r8]
141bf1a0e: 488b09 mov rcx, qword ptr [rcx]
141bf1a11: 48894d30 mov qword ptr [rbp + 0x30], rcx
141bf1a15: 48894538 mov qword ptr [rbp + 0x38], rax
141bf1a19: 488b0f mov rcx, qword ptr [rdi]
141bf1a1c: 488d15bd584401 lea rdx, [rip + 0x14458bd]
141bf1a23: e8a8686800 call 0x1422782d0 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
141bf1a28: 4889c7 mov rdi, rax
141bf1a2b: 4885c0 test rax, rax
141bf1a2e: 0f840d0d0000 je 0x141bf2741
141bf1a34: 488d05dd594401 lea rax, [rip + 0x14459dd] ; @143037418 'country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo/rustc/05f9846f893b09a1be1fc8560e'
141bf1a3b: 48894508 mov qword ptr [rbp + 8], rax
141bf1a3f: 48c745103f000000 mov qword ptr [rbp + 0x10], 0x3f
141bf1a47: 4889fb mov rbx, rdi
141bf1a4a: 4883c310 add rbx, 0x10
141bf1a4e: 48837f183f cmp qword ptr [rdi + 0x18], 0x3f
141bf1a53: 0f85290c0000 jne 0x141bf2682
141bf1a59: 488b13 mov rdx, qword ptr [rbx]
141bf1a5c: 488d0db5594401 lea rcx, [rip + 0x14459b5] ; @143037418 'country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo/rustc/05f9846f893b09a1be1fc8560e'
141bf1a63: 41b83f000000 mov r8d, 0x3f
141bf1a69: e88221d300 call 0x142923bf0 ; 
141bf1a6e: 85c0 test eax, eax
141bf1a70: 0f850c0c0000 jne 0x141bf2682
141bf1a76: 488b0f mov rcx, qword ptr [rdi]
141bf1a79: 488b4708 mov rax, qword ptr [rdi + 8]
141bf1a7d: ff5018 call qword ptr [rax + 0x18]
141bf1a80: 4885c0 test rax, rax
141bf1a83: 0f84b80c0000 je 0x141bf2741
141bf1a89: 488b9580020000 mov rdx, qword ptr [rbp + 0x280]
141bf1a90: 488b0a mov rcx, qword ptr [rdx]
141bf1a93: 4c8b4208 mov r8, qword ptr [rdx + 8]
141bf1a97: 4c8b09 mov r9, qword ptr [rcx]
141bf1a9a: 31d2 xor edx, edx
141bf1a9c: 4c89ce mov rsi, r9
141bf1a9f: 492b7018 sub rsi, qword ptr [r8 + 0x18]
141bf1aa3: 480f42f2 cmovb rsi, rdx
141bf1aa7: 4d2b4838 sub r9, qword ptr [r8 + 0x38]
141bf1aab: 4c0f42ca cmovb r9, rdx
141bf1aaf: 488d3cb6 lea rdi, [rsi + rsi*4]
141bf1ab3: 48c1e703 shl rdi, 3
141bf1ab7: 49037808 add rdi, qword ptr [r8 + 8]
141bf1abb: 4f8d1c89 lea r11, [r9 + r9*4]
141bf1abf: 49c1e303 shl r11, 3
141bf1ac3: 4d035828 add r11, qword ptr [r8 + 0x28]
141bf1ac7: 4d8b5010 mov r10, qword ptr [r8 + 0x10]
141bf1acb: 4929f2 sub r10, rsi
141bf1ace: 4c0f42d2 cmovb r10, rdx
141bf1ad2: bb08000000 mov ebx, 8
141bf1ad7: 480f42fb cmovb rdi, rbx
141bf1adb: 4b8d3492 lea rsi, [r10 + r10*4]
141bf1adf: 410f96c6 setbe r14b
141bf1ae3: 488d7cf7d8 lea rdi, [rdi + rsi*8 - 0x28]
141bf1ae8: 498b7030 mov rsi, qword ptr [r8 + 0x30]
141bf1aec: 480f46fa cmovbe rdi, rdx
141bf1af0: 4c29ce sub rsi, r9
141bf1af3: 4c8b8d88020000 mov r9, qword ptr [rbp + 0x288]
141bf1afa: 480f42f2 cmovb rsi, rdx
141bf1afe: 4c0f42db cmovb r11, rbx
141bf1b02: 488d1cb6 lea rbx, [rsi + rsi*4]
141bf1b06: 4d8d5cdbd8 lea r11, [r11 + rbx*8 - 0x28]
141bf1b0b: 4c0f46df cmovbe r11, rdi
141bf1b0f: 0f96c3 setbe bl
141bf1b12: 4c01d6 add rsi, r10
141bf1b15: 4c8b10 mov r10, qword ptr [rax]
141bf1b18: 4c899538010000 mov qword ptr [rbp + 0x138], r10
141bf1b1f: 488b4008 mov rax, qword ptr [rax + 8]
141bf1b23: 48898510010000 mov qword ptr [rbp + 0x110], rax
141bf1b2a: 498b4040 mov rax, qword ptr [r8 + 0x40]
141bf1b2e: 4420f3 and bl, r14b
141bf1b31: 889d08010000 mov byte ptr [rbp + 0x108], bl
141bf1b37: 480f44f2 cmove rsi, rdx
141bf1b3b: 4829f0 sub rax, rsi
141bf1b3e: 488901 mov qword ptr [rcx], rax
141bf1b41: 488b8540010000 mov rax, qword ptr [rbp + 0x140]
141bf1b48: 488b00 mov rax, qword ptr [rax]
141bf1b4b: 488b5808 mov rbx, qword ptr [rax + 8]
141bf1b4f: 48898548010000 mov qword ptr [rbp + 0x148], rax
141bf1b56: 488b4010 mov rax, qword ptr [rax + 0x10]
141bf1b5a: 488d0480 lea rax, [rax + rax*4]
141bf1b5e: 48c1e004 shl rax, 4
141bf1b62: 4801d8 add rax, rbx
141bf1b65: 48898500010000 mov qword ptr [rbp + 0x100], rax
141bf1b6c: 498d4304 lea rax, [r11 + 4]
141bf1b70: 48898520010000 mov qword ptr [rbp + 0x120], rax
141bf1b77: 498b01 mov rax, qword ptr [r9]
141bf1b7a: 48898518010000 mov qword ptr [rbp + 0x118], rax
141bf1b81: 440f2805f7cdd300 movaps xmm8, xmmword ptr [rip + 0xd3cdf7]
141bf1b89: f30f1035831ce900 movss xmm6, dword ptr [rip + 0xe91c83]
141bf1b91: f30f103dbf9ae800 movss xmm7, dword ptr [rip + 0xe89abf]
141bf1b99: eb27 jmp 0x141bf1bc2
141bf1b9b: 0f1f440000 nop dword ptr [rax + rax]
141bf1ba0: 4883c350 add rbx, 0x50
141bf1ba4: 498b4708 mov rax, qword ptr [r15 + 8]
141bf1ba8: 4c89e1 mov rcx, r12
141bf1bab: 48c1e104 shl rcx, 4
141bf1baf: 4c893408 mov qword ptr [rax + rcx], r14
141bf1bb3: 4088740808 mov byte ptr [rax + rcx + 8], sil
141bf1bb8: 49ffc4 inc r12
141bf1bbb: 4d896710 mov qword ptr [r15 + 0x10], r12
141bf1bbf: 4989fb mov r11, rdi
141bf1bc2: 483b9d00010000 cmp rbx, qword ptr [rbp + 0x100]
141bf1bc9: 0f8472020000 je 0x141bf1e41
141bf1bcf: 4c89df mov rdi, r11
141bf1bd2: 488d5310 lea rdx, [rbx + 0x10]
141bf1bd6: 488d4d54 lea rcx, [rbp + 0x54]
141bf1bda: e8c135eefe call 0x140ad51a0 ; country_core::resources::placement_data::PlacementDataSsbo::position
141bf1bdf: f3440f104d54 movss xmm9, dword ptr [rbp + 0x54]
141bf1be5: 410f28c1 movaps xmm0, xmm9
141bf1be9: 410f54c0 andps xmm0, xmm8
141bf1bed: 0f2ec6 ucomiss xmm0, xmm6
141bf1bf0: 7713 ja 0x141bf1c05
141bf1bf2: f3440f10555c movss xmm10, dword ptr [rbp + 0x5c]
141bf1bf8: 410f28c2 movaps xmm0, xmm10
141bf1bfc: 410f54c0 andps xmm0, xmm8
141bf1c00: 0f2ec6 ucomiss xmm0, xmm6
141bf1c03: 762b jbe 0x141bf1c30
141bf1c05: c685a800000001 mov byte ptr [rbp + 0xa8], 1
141bf1c0c: 40b601 mov sil, 1
141bf1c0f: 4c8b7340 mov r14, qword ptr [rbx + 0x40]
141bf1c13: 4d8b6710 mov r12, qword ptr [r15 + 0x10]
141bf1c17: 4d3b27 cmp r12, qword ptr [r15]
141bf1c1a: 7584 jne 0x141bf1ba0
141bf1c1c: 4c89f9 mov rcx, r15
141bf1c1f: 488d154abc3001 lea rdx, [rip + 0x130bc4a]
141bf1c26: e8050a97fe call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<toolbar::ui_interaction::UiJustBecameInFocusEvent>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::walls::WallId,ref$<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> > >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemWindow>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<asset::prefab::PrefabName,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<alloc::ffi::c_str::CString,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemRoofShape>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<ref$<std::ffi::os_str::OsStr>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::systems::prefab::DespawnPrefabCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_onboarding::tutorial_completion::TutorialHintComplete>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_color::set_decorator_color_id::SetStairRailingCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<u64,ref$<system_clutter::autoclutter::AutoClutterObject> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<enum2$<core::option::Option<bevy_ecs::entity::Entity> >,bool>,alloc::alloc::Global>
141bf1c2b: e970ffffff jmp 0x141bf1ba0
141bf1c30: c744242000000243 mov dword ptr [rsp + 0x20], 0x43020000
141bf1c38: 488d4db8 lea rcx, [rbp - 0x48]
141bf1c3c: 410f28c9 movaps xmm1, xmm9
141bf1c40: 410f28d2 movaps xmm2, xmm10
141bf1c44: 0f28df movaps xmm3, xmm7
141bf1c47: e8a431e2fe call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
141bf1c4c: 440f28d8 movaps xmm11, xmm0
141bf1c50: c744242000000243 mov dword ptr [rsp + 0x20], 0x43020000
141bf1c58: 488d4dc8 lea rcx, [rbp - 0x38]
141bf1c5c: 410f28c9 movaps xmm1, xmm9
141bf1c60: 410f28d2 movaps xmm2, xmm10
141bf1c64: 0f28df movaps xmm3, xmm7
141bf1c67: e88431e2fe call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
141bf1c6c: 440f28e0 movaps xmm12, xmm0
141bf1c70: c744242000000243 mov dword ptr [rsp + 0x20], 0x43020000
141bf1c78: 488d4dd8 lea rcx, [rbp - 0x28]
141bf1c7c: 410f28c9 movaps xmm1, xmm9
141bf1c80: 410f28d2 movaps xmm2, xmm10
141bf1c84: 0f28df movaps xmm3, xmm7
141bf1c87: e86431e2fe call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
141bf1c8c: 440f28e8 movaps xmm13, xmm0
141bf1c90: c744242000000243 mov dword ptr [rsp + 0x20], 0x43020000
141bf1c98: 488d4de8 lea rcx, [rbp - 0x18]
141bf1c9c: 410f28c9 movaps xmm1, xmm9
141bf1ca0: 410f28d2 movaps xmm2, xmm10
141bf1ca4: 0f28df movaps xmm3, xmm7
141bf1ca7: e84431e2fe call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
141bf1cac: 440f28f0 movaps xmm14, xmm0
141bf1cb0: c744242000000243 mov dword ptr [rsp + 0x20], 0x43020000
141bf1cb8: 488d4d40 lea rcx, [rbp + 0x40]
141bf1cbc: 410f28c9 movaps xmm1, xmm9
141bf1cc0: 410f28d2 movaps xmm2, xmm10
141bf1cc4: 0f28df movaps xmm3, xmm7
141bf1cc7: e82431e2fe call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
141bf1ccc: 440f28f8 movaps xmm15, xmm0
141bf1cd0: c744242000000243 mov dword ptr [rsp + 0x20], 0x43020000
141bf1cd8: 488d4d30 lea rcx, [rbp + 0x30]
141bf1cdc: 410f28c9 movaps xmm1, xmm9
141bf1ce0: 410f28d2 movaps xmm2, xmm10
141bf1ce4: 0f28df movaps xmm3, xmm7
141bf1ce7: e80431e2fe call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
141bf1cec: 0f57c9 xorps xmm1, xmm1
141bf1cef: 440f2ef9 ucomiss xmm15, xmm1
141bf1cf3: 0f87b4000000 ja 0x141bf1dad
141bf1cf9: 0f2ec1 ucomiss xmm0, xmm1
141bf1cfc: 0f87ab000000 ja 0x141bf1dad
141bf1d02: 488b8d38010000 mov rcx, qword ptr [rbp + 0x138]
141bf1d09: 488b9510010000 mov rdx, qword ptr [rbp + 0x110]
141bf1d10: 410f28d1 movaps xmm2, xmm9
141bf1d14: 410f28da movaps xmm3, xmm10
141bf1d18: e8e38adffe call 0x1409ea800 ; country_core::resources::terrain_rock_mask::impl$1::sample_at_world_xz
141bf1d1d: f3450f58dc addss xmm11, xmm12
141bf1d22: f3450f58dd addss xmm11, xmm13
141bf1d27: f3450f58de addss xmm11, xmm14
141bf1d2c: 0f57c0 xorps xmm0, xmm0
141bf1d2f: f3410f5fc3 maxss xmm0, xmm11
141bf1d34: f30f100dc83bd300 movss xmm1, dword ptr [rip + 0xd33bc8]
141bf1d3c: f30f5dc8 minss xmm1, xmm0
141bf1d40: 0f57c0 xorps xmm0, xmm0
141bf1d43: 0f2ec8 ucomiss xmm1, xmm0
141bf1d46: 400f97c6 seta sil
141bf1d4a: 4008c6 or sil, al
141bf1d4d: 0fb68508010000 movzx eax, byte ptr [rbp + 0x108]
141bf1d54: 4008f0 or al, sil
141bf1d57: 4080f601 xor sil, 1
141bf1d5b: 4088b5a8000000 mov byte ptr [rbp + 0xa8], sil
141bf1d62: 3c01 cmp al, 1
141bf1d64: 0f8492000000 je 0x141bf1dfc
141bf1d6a: f60701 test byte ptr [rdi], 1
141bf1d6d: 744c je 0x141bf1dbb
141bf1d6f: 488b8d20010000 mov rcx, qword ptr [rbp + 0x120]
141bf1d76: 410f28c9 movaps xmm1, xmm9
141bf1d7a: 410f28d2 movaps xmm2, xmm10
141bf1d7e: e89d900aff call 0x140c9ae20 ; utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter
141bf1d83: 0f14c1 unpcklps xmm0, xmm1
141bf1d86: 410f28c9 movaps xmm1, xmm9
141bf1d8a: 410f14ca unpcklps xmm1, xmm10
141bf1d8e: 0f5cc1 subps xmm0, xmm1
141bf1d91: 0f59c0 mulps xmm0, xmm0
141bf1d94: f30f16c8 movshdup xmm1, xmm0
141bf1d98: f30f58c8 addss xmm1, xmm0
141bf1d9c: 0f57c0 xorps xmm0, xmm0
141bf1d9f: f30f51c1 sqrtss xmm0, xmm1
141bf1da3: f30f100d9dafe800 movss xmm1, dword ptr [rip + 0xe8af9d]
141bf1dab: eb3d jmp 0x141bf1dea
141bf1dad: c685a800000000 mov byte ptr [rbp + 0xa8], 0
141bf1db4: 31f6 xor esi, esi
141bf1db6: e954feffff jmp 0x141bf1c0f
141bf1dbb: f20f104704 movsd xmm0, qword ptr [rdi + 4]
141bf1dc0: 410f28c9 movaps xmm1, xmm9
141bf1dc4: 410f14ca unpcklps xmm1, xmm10
141bf1dc8: 0f5cc1 subps xmm0, xmm1
141bf1dcb: 0f59c0 mulps xmm0, xmm0
141bf1dce: f30f16c8 movshdup xmm1, xmm0
141bf1dd2: f30f58c8 addss xmm1, xmm0
141bf1dd6: 0f57c0 xorps xmm0, xmm0
141bf1dd9: f30f51c1 sqrtss xmm0, xmm1
141bf1ddd: f30f104f0c movss xmm1, dword ptr [rdi + 0xc]
141bf1de2: f30f580d5eafe800 addss xmm1, dword ptr [rip + 0xe8af5e]
141bf1dea: 0f2ec8 ucomiss xmm1, xmm0
141bf1ded: 400f97c6 seta sil
141bf1df1: 4080f601 xor sil, 1
141bf1df5: 4088b5a8000000 mov byte ptr [rbp + 0xa8], sil
141bf1dfc: 4084f6 test sil, sil
141bf1dff: 0f840afeffff je 0x141bf1c0f
141bf1e05: 488d4df8 lea rcx, [rbp - 8]
141bf1e09: 410f28c9 movaps xmm1, xmm9
141bf1e0d: 410f28d2 movaps xmm2, xmm10
141bf1e11: f30f101d2fafe800 movss xmm3, dword ptr [rip + 0xe8af2f]
141bf1e19: e822570aff call 0x140c97540 ; utils::geometry::circle::Circle2d::new
141bf1e1e: 488b8d18010000 mov rcx, qword ptr [rbp + 0x118]
141bf1e25: 488d55f8 lea rdx, [rbp - 8]
141bf1e29: 4c8d85a8000000 lea r8, [rbp + 0xa8]
141bf1e30: e89b676800 call 0x1422785d0 ; country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<forest::manage_tree_prefabs::manage_tree_prefabs::closure_env$0>
141bf1e35: 0fb6b5a8000000 movzx esi, byte ptr [rbp + 0xa8]
141bf1e3c: e9cefdffff jmp 0x141bf1c0f
141bf1e41: 488b8590020000 mov rax, qword ptr [rbp + 0x290]
141bf1e48: 488b00 mov rax, qword ptr [rax]
141bf1e4b: 8b482c mov ecx, dword ptr [rax + 0x2c]
141bf1e4e: e8ed430aff call 0x140c96240 ; utils::rng::init_rng
141bf1e53: 48894570 mov qword ptr [rbp + 0x70], rax
141bf1e57: 4d8b4708 mov r8, qword ptr [r15 + 8]
141bf1e5b: 498b4710 mov rax, qword ptr [r15 + 0x10]
141bf1e5f: 48c1e004 shl rax, 4
141bf1e63: 4c01c0 add rax, r8
141bf1e66: 48898500010000 mov qword ptr [rbp + 0x100], rax
141bf1e6d: 488b8d78020000 mov rcx, qword ptr [rbp + 0x278]
141bf1e74: 488b01 mov rax, qword ptr [rcx]
141bf1e77: 48898528010000 mov qword ptr [rbp + 0x128], rax
141bf1e7e: 488b4108 mov rax, qword ptr [rcx + 8]
141bf1e82: 48898538010000 mov qword ptr [rbp + 0x138], rax
141bf1e89: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
141bf1e90: 488b00 mov rax, qword ptr [rax]
141bf1e93: 48898598000000 mov qword ptr [rbp + 0x98], rax
141bf1e9a: 488b8540010000 mov rax, qword ptr [rbp + 0x140]
141bf1ea1: 8b481c mov ecx, dword ptr [rax + 0x1c]
141bf1ea4: 898d18010000 mov dword ptr [rbp + 0x118], ecx
141bf1eaa: 488b4010 mov rax, qword ptr [rax + 0x10]
141bf1eae: 48898520010000 mov qword ptr [rbp + 0x120], rax
141bf1eb5: 488b8d70020000 mov rcx, qword ptr [rbp + 0x270]
141bf1ebc: 8b411c mov eax, dword ptr [rcx + 0x1c]
141bf1ebf: 8985fc000000 mov dword ptr [rbp + 0xfc], eax
141bf1ec5: 488b01 mov rax, qword ptr [rcx]
141bf1ec8: 488b4910 mov rcx, qword ptr [rcx + 0x10]
141bf1ecc: 48898d88000000 mov qword ptr [rbp + 0x88], rcx
141bf1ed3: 48898510010000 mov qword ptr [rbp + 0x110], rax
141bf1eda: 4883c020 add rax, 0x20
141bf1ede: 48898590000000 mov qword ptr [rbp + 0x90], rax
141bf1ee5: 31db xor ebx, ebx
141bf1ee7: f3440f1015e839d300 movss xmm10, dword ptr [rip + 0xd339e8]
141bf1ef0: f3440f101d97bcdf00 movss xmm11, dword ptr [rip + 0xdfbc97]
141bf1ef9: f3440f10254e2eed00 movss xmm12, dword ptr [rip + 0xed2e4e]
141bf1f02: f2440f102da53eea00 movsd xmm13, qword ptr [rip + 0xea3ea5]
141bf1f0b: 4c8ba548010000 mov r12, qword ptr [rbp + 0x148]
141bf1f12: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141bf1f20: 4c3b8500010000 cmp r8, qword ptr [rbp + 0x100]
141bf1f27: 0f844c060000 je 0x141bf2579
141bf1f2d: 4c89c6 mov rsi, r8
141bf1f30: 41807c241801 cmp byte ptr [r12 + 0x18], 1
141bf1f36: 7538 jne 0x141bf1f70
141bf1f38: 488d4d70 lea rcx, [rbp + 0x70]
141bf1f3c: e83fd40bff call 0x140caf380 ; fastrand::Rng::f32
141bf1f41: 0f28f8 movaps xmm7, xmm0
141bf1f44: f30f1005b839d300 movss xmm0, dword ptr [rip + 0xd339b8]
141bf1f4c: f30f5cc7 subss xmm0, xmm7
141bf1f50: f3410f59c2 mulss xmm0, xmm10
141bf1f55: f3410f59fb mulss xmm7, xmm11
141bf1f5a: f30f58f8 addss xmm7, xmm0
141bf1f5e: 450f57c0 xorps xmm8, xmm8
141bf1f62: 450f57f6 xorps xmm14, xmm14
141bf1f66: 4c8ba548010000 mov r12, qword ptr [rbp + 0x148]
141bf1f6d: eb2f jmp 0x141bf1f9e
141bf1f6f: 90 nop 
141bf1f70: 498b542410 mov rdx, qword ptr [r12 + 0x10]
141bf1f75: 4839d3 cmp rbx, rdx
141bf1f78: 0f8341070000 jae 0x141bf26bf
141bf1f7e: 498b442408 mov rax, qword ptr [r12 + 8]
141bf1f83: 488d0c9b lea rcx, [rbx + rbx*4]
141bf1f87: 48c1e104 shl rcx, 4
141bf1f8b: f3440f1074081c movss xmm14, dword ptr [rax + rcx + 0x1c]
141bf1f92: 0f57ff xorps xmm7, xmm7
141bf1f95: f3440f10056639d300 movss xmm8, dword ptr [rip + 0xd33966]
141bf1f9e: 4c8d4610 lea r8, [rsi + 0x10]
141bf1fa2: 4c8d7301 lea r14, [rbx + 1]
141bf1fa6: 8b4604 mov eax, dword ptr [rsi + 4]
141bf1fa9: 807e0800 cmp byte ptr [rsi + 8], 0
141bf1fad: 0f84fd000000 je 0x141bf20b0
141bf1fb3: 85c0 test eax, eax
141bf1fb5: 0f84e6010000 je 0x141bf21a1
141bf1fbb: 488b36 mov rsi, qword ptr [rsi]
141bf1fbe: 4889f0 mov rax, rsi
141bf1fc1: 48c1e820 shr rax, 0x20
141bf1fc5: 0f8405070000 je 0x141bf26d0
141bf1fcb: 89f1 mov ecx, esi
141bf1fcd: 4c89f3 mov rbx, r14
141bf1fd0: 488b9538010000 mov rdx, qword ptr [rbp + 0x138]
141bf1fd7: 483b4a10 cmp rcx, qword ptr [rdx + 0x10]
141bf1fdb: 0f833fffffff jae 0x141bf1f20
141bf1fe1: 488b9538010000 mov rdx, qword ptr [rbp + 0x138]
141bf1fe8: 488b5208 mov rdx, qword ptr [rdx + 8]
141bf1fec: 488d0c89 lea rcx, [rcx + rcx*4]
141bf1ff0: 4c89f3 mov rbx, r14
141bf1ff3: 39048a cmp dword ptr [rdx + rcx*4], eax
141bf1ff6: 0f8524ffffff jne 0x141bf1f20
141bf1ffc: 488d048a lea rax, [rdx + rcx*4]
141bf2000: 8b4004 mov eax, dword ptr [rax + 4]
141bf2003: 4c89f3 mov rbx, r14
141bf2006: b9ffffffff mov ecx, 0xffffffff
141bf200b: 4839c8 cmp rax, rcx
141bf200e: 0f840cffffff je 0x141bf1f20
141bf2014: 4c89f3 mov rbx, r14
141bf2017: 488b8d28010000 mov rcx, qword ptr [rbp + 0x128]
141bf201e: 48394138 cmp qword ptr [rcx + 0x38], rax
141bf2022: 0f86f8feffff jbe 0x141bf1f20
141bf2028: 488b8d28010000 mov rcx, qword ptr [rbp + 0x128]
141bf202f: 488b4928 mov rcx, qword ptr [rcx + 0x28]
141bf2033: 89c2 mov edx, eax
141bf2035: c1ea06 shr edx, 6
141bf2038: 488b0cd1 mov rcx, qword ptr [rcx + rdx*8]
141bf203c: 480fa3c1 bt rcx, rax
141bf2040: 4c89f3 mov rbx, r14
141bf2043: 0f83d7feffff jae 0x141bf1f20
141bf2049: 4d89c7 mov r15, r8
141bf204c: 488b8530010000 mov rax, qword ptr [rbp + 0x130]
141bf2053: 488b7808 mov rdi, qword ptr [rax + 8]
141bf2057: 4885ff test rdi, rdi
141bf205a: 750b jne 0x141bf2067
141bf205c: 488b8530010000 mov rax, qword ptr [rbp + 0x130]
141bf2063: 488b7810 mov rdi, qword ptr [rax + 0x10]
141bf2067: 488b07 mov rax, qword ptr [rdi]
141bf206a: 488b5f10 mov rbx, qword ptr [rdi + 0x10]
141bf206e: 4829d8 sub rax, rbx
141bf2071: 4883f817 cmp rax, 0x17
141bf2075: 0f86d9040000 jbe 0x141bf2554
141bf207b: 488b4708 mov rax, qword ptr [rdi + 8]
141bf207f: 488d0d9a686800 lea rcx, [rip + 0x68689a]
141bf2086: 48890c18 mov qword ptr [rax + rbx], rcx
141bf208a: 488d0d5f6c6800 lea rcx, [rip + 0x686c5f]
141bf2091: 48894c1808 mov qword ptr [rax + rbx + 8], rcx
141bf2096: 4889741810 mov qword ptr [rax + rbx + 0x10], rsi
141bf209b: 4883c318 add rbx, 0x18
141bf209f: e9e7000000 jmp 0x141bf218b
141bf20a4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141bf20b0: 4c89f3 mov rbx, r14
141bf20b3: 85c0 test eax, eax
141bf20b5: 0f8465feffff je 0x141bf1f20
141bf20bb: 488b36 mov rsi, qword ptr [rsi]
141bf20be: 4889f0 mov rax, rsi
141bf20c1: 48c1e820 shr rax, 0x20
141bf20c5: 0f8413060000 je 0x141bf26de
141bf20cb: 89f1 mov ecx, esi
141bf20cd: 488b9538010000 mov rdx, qword ptr [rbp + 0x138]
141bf20d4: 483b4a10 cmp rcx, qword ptr [rdx + 0x10]
141bf20d8: 7356 jae 0x141bf2130
141bf20da: 488b9538010000 mov rdx, qword ptr [rbp + 0x138]
141bf20e1: 488b5208 mov rdx, qword ptr [rdx + 8]
141bf20e5: 488d0c89 lea rcx, [rcx + rcx*4]
141bf20e9: 39048a cmp dword ptr [rdx + rcx*4], eax
141bf20ec: 7542 jne 0x141bf2130
141bf20ee: 488d048a lea rax, [rdx + rcx*4]
141bf20f2: 8b4004 mov eax, dword ptr [rax + 4]
141bf20f5: b9ffffffff mov ecx, 0xffffffff
141bf20fa: 4839c8 cmp rax, rcx
141bf20fd: 7431 je 0x141bf2130
141bf20ff: 488b8d28010000 mov rcx, qword ptr [rbp + 0x128]
141bf2106: 48394138 cmp qword ptr [rcx + 0x38], rax
141bf210a: 7624 jbe 0x141bf2130
141bf210c: 488b8d28010000 mov rcx, qword ptr [rbp + 0x128]
141bf2113: 488b4928 mov rcx, qword ptr [rcx + 0x28]
141bf2117: 89c2 mov edx, eax
141bf2119: c1ea06 shr edx, 6
141bf211c: 488b0cd1 mov rcx, qword ptr [rcx + rdx*8]
141bf2120: 480fa3c1 bt rcx, rax
141bf2124: 4c89f3 mov rbx, r14
141bf2127: 0f82f3fdffff jb 0x141bf1f20
141bf212d: 0f1f00 nop dword ptr [rax]
141bf2130: 4d89c7 mov r15, r8
141bf2133: 488b8530010000 mov rax, qword ptr [rbp + 0x130]
141bf213a: 488b7808 mov rdi, qword ptr [rax + 8]
141bf213e: 4885ff test rdi, rdi
141bf2141: 750b jne 0x141bf214e
141bf2143: 488b8530010000 mov rax, qword ptr [rbp + 0x130]
141bf214a: 488b7810 mov rdi, qword ptr [rax + 0x10]
141bf214e: 488b07 mov rax, qword ptr [rdi]
141bf2151: 488b5f10 mov rbx, qword ptr [rdi + 0x10]
141bf2155: 4829d8 sub rax, rbx
141bf2158: 4883f81f cmp rax, 0x1f
141bf215c: 0f8683030000 jbe 0x141bf24e5
141bf2162: 488b4708 mov rax, qword ptr [rdi + 8]
141bf2166: 488d0da3696800 lea rcx, [rip + 0x6869a3]
141bf216d: 48890c18 mov qword ptr [rax + rbx], rcx
141bf2171: 488d0d886d6800 lea rcx, [rip + 0x686d88]
141bf2178: 48894c1808 mov qword ptr [rax + rbx + 8], rcx
141bf217d: 4889741810 mov qword ptr [rax + rbx + 0x10], rsi
141bf2182: c644181800 mov byte ptr [rax + rbx + 0x18], 0
141bf2187: 4883c320 add rbx, 0x20
141bf218b: 48895f10 mov qword ptr [rdi + 0x10], rbx
141bf218f: 4c89f3 mov rbx, r14
141bf2192: 4c8ba548010000 mov r12, qword ptr [rbp + 0x148]
141bf2199: 4d89f8 mov r8, r15
141bf219c: e97ffdffff jmp 0x141bf1f20
141bf21a1: 4c898508010000 mov qword ptr [rbp + 0x108], r8
141bf21a8: 488d4d08 lea rcx, [rbp + 8]
141bf21ac: 488b9530010000 mov rdx, qword ptr [rbp + 0x130]
141bf21b3: e8e82412ff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
141bf21b8: 488b8548010000 mov rax, qword ptr [rbp + 0x148]
141bf21bf: 488b5010 mov rdx, qword ptr [rax + 0x10]
141bf21c3: 4839d3 cmp rbx, rdx
141bf21c6: 0f8331050000 jae 0x141bf26fd
141bf21cc: 488b7d10 mov rdi, qword ptr [rbp + 0x10]
141bf21d0: 4885ff test rdi, rdi
141bf21d3: 7504 jne 0x141bf21d9
141bf21d5: 488b7d18 mov rdi, qword ptr [rbp + 0x18]
141bf21d9: 488b4008 mov rax, qword ptr [rax + 8]
141bf21dd: 488d349b lea rsi, [rbx + rbx*4]
141bf21e1: 48c1e604 shl rsi, 4
141bf21e5: f3440f104c301c movss xmm9, dword ptr [rax + rsi + 0x1c]
141bf21ec: 4c8b6528 mov r12, qword ptr [rbp + 0x28]
141bf21f0: 488b07 mov rax, qword ptr [rdi]
141bf21f3: 4c8b7f10 mov r15, qword ptr [rdi + 0x10]
141bf21f7: 4c29f8 sub rax, r15
141bf21fa: 4883f837 cmp rax, 0x37
141bf21fe: 0f8606030000 jbe 0x141bf250a
141bf2204: 488b4708 mov rax, qword ptr [rdi + 8]
141bf2208: 488d0df1676800 lea rcx, [rip + 0x6867f1]
141bf220f: 4a890c38 mov qword ptr [rax + r15], rcx
141bf2213: 488d0de66c6800 lea rcx, [rip + 0x686ce6]
141bf221a: 4a894c3808 mov qword ptr [rax + r15 + 8], rcx
141bf221f: 4d89e5 mov r13, r12
141bf2222: 4e89643810 mov qword ptr [rax + r15 + 0x10], r12
141bf2227: 4a895c3818 mov qword ptr [rax + r15 + 0x18], rbx
141bf222c: f3460f114c3820 movss dword ptr [rax + r15 + 0x20], xmm9
141bf2233: f3460f11443824 movss dword ptr [rax + r15 + 0x24], xmm8
141bf223a: f3420f117c3828 movss dword ptr [rax + r15 + 0x28], xmm7
141bf2241: 42c644383000 mov byte ptr [rax + r15 + 0x30], 0
141bf2247: 4983c738 add r15, 0x38
141bf224b: 4c897f10 mov qword ptr [rdi + 0x10], r15
141bf224f: 4c8ba548010000 mov r12, qword ptr [rbp + 0x148]
141bf2256: 498b542410 mov rdx, qword ptr [r12 + 0x10]
141bf225b: 4839d3 cmp rbx, rdx
141bf225e: 0f83aa040000 jae 0x141bf270e
141bf2264: 498b442408 mov rax, qword ptr [r12 + 8]
141bf2269: f30f107c3010 movss xmm7, dword ptr [rax + rsi + 0x10]
141bf226f: f3440f104c3014 movss xmm9, dword ptr [rax + rsi + 0x14]
141bf2276: f3440f10443018 movss xmm8, dword ptr [rax + rsi + 0x18]
141bf227d: 410f2efc ucomiss xmm7, xmm12
141bf2281: 0f82ab000000 jb 0x141bf2332
141bf2287: 0f2ef7 ucomiss xmm6, xmm7
141bf228a: 0f86a2000000 jbe 0x141bf2332
141bf2290: 450f2ec4 ucomiss xmm8, xmm12
141bf2294: 0f8298000000 jb 0x141bf2332
141bf229a: 410f2ef0 ucomiss xmm6, xmm8
141bf229e: 0f868e000000 jbe 0x141bf2332
141bf22a4: 488b8d98000000 mov rcx, qword ptr [rbp + 0x98]
141bf22ab: 0f28cf movaps xmm1, xmm7
141bf22ae: 410f28d0 movaps xmm2, xmm8
141bf22b2: e8f94c65ff call 0x141246fb0 ; country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo> | country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo> | country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo> | country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo> | country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo> | country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo> | country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>
141bf22b7: 440f28c8 movaps xmm9, xmm0
141bf22bb: 4d89ec mov r12, r13
141bf22be: f3450f12fe movsldup xmm15, xmm14
141bf22c3: 450f59fd mulps xmm15, xmm13
141bf22c7: 488b07 mov rax, qword ptr [rdi]
141bf22ca: 4c8b7f10 mov r15, qword ptr [rdi + 0x10]
141bf22ce: 4c29f8 sub rax, r15
141bf22d1: 4883f827 cmp rax, 0x27
141bf22d5: 0f8654020000 jbe 0x141bf252f
141bf22db: 0f28c7 movaps xmm0, xmm7
141bf22de: 410f14c1 unpcklps xmm0, xmm9
141bf22e2: 410f58c7 addps xmm0, xmm15
141bf22e6: f3450f58f8 addss xmm15, xmm8
141bf22eb: 488b4708 mov rax, qword ptr [rdi + 8]
141bf22ef: 488d0d0a696800 lea rcx, [rip + 0x68690a]
141bf22f6: 4a890c38 mov qword ptr [rax + r15], rcx
141bf22fa: 488d0dff6b6800 lea rcx, [rip + 0x686bff]
141bf2301: 4a894c3808 mov qword ptr [rax + r15 + 8], rcx
141bf2306: 4e89643810 mov qword ptr [rax + r15 + 0x10], r12
141bf230b: 420f13443818 movlps qword ptr [rax + r15 + 0x18], xmm0
141bf2311: f3460f117c3820 movss dword ptr [rax + r15 + 0x20], xmm15
141bf2318: 42c644382400 mov byte ptr [rax + r15 + 0x24], 0
141bf231e: 4983c728 add r15, 0x28
141bf2322: 4c897f10 mov qword ptr [rdi + 0x10], r15
141bf2326: 4c8ba548010000 mov r12, qword ptr [rbp + 0x148]
141bf232d: 498b542410 mov rdx, qword ptr [r12 + 0x10]
141bf2332: 4839d3 cmp rbx, rdx
141bf2335: 0f83e4030000 jae 0x141bf271f
141bf233b: 498b442408 mov rax, qword ptr [r12 + 8]
141bf2340: 488b0c30 mov rcx, qword ptr [rax + rsi]
141bf2344: 488b7c3008 mov rdi, qword ptr [rax + rsi + 8]
141bf2349: f048ff01 lock inc qword ptr [rcx]
141bf234d: 0f8efa030000 jle 0x141bf274d
141bf2353: 48898d40010000 mov qword ptr [rbp + 0x140], rcx
141bf235a: 48898da8000000 mov qword ptr [rbp + 0xa8], rcx
141bf2361: 4889bdb0000000 mov qword ptr [rbp + 0xb0], rdi
141bf2368: f30f10059435d300 movss xmm0, dword ptr [rip + 0xd33594]
141bf2370: 0f28c8 movaps xmm1, xmm0
141bf2373: e878ea09ff call 0x140c90df0 ; utils::pack_2x16f_uint
141bf2378: 498b542410 mov rdx, qword ptr [r12 + 0x10]
141bf237d: 4839d3 cmp rbx, rdx
141bf2380: 0f83aa030000 jae 0x141bf2730
141bf2386: 498b4c2408 mov rcx, qword ptr [r12 + 8]
141bf238b: 8b543148 mov edx, dword ptr [rcx + rsi + 0x48]
141bf238f: f30f10443120 movss xmm0, dword ptr [rcx + rsi + 0x20]
141bf2395: 448b443124 mov r8d, dword ptr [rcx + rsi + 0x24]
141bf239a: 448b4c312c mov r9d, dword ptr [rcx + rsi + 0x2c]
141bf239f: 4c8b543134 mov r10, qword ptr [rcx + rsi + 0x34]
141bf23a4: 8b4c313c mov ecx, dword ptr [rcx + rsi + 0x3c]
141bf23a8: 448b9dfc000000 mov r11d, dword ptr [rbp + 0xfc]
141bf23af: 4c8bbd88000000 mov r15, qword ptr [rbp + 0x88]
141bf23b6: 45891f mov dword ptr [r15], r11d
141bf23b9: 4c8bbd10010000 mov r15, qword ptr [rbp + 0x110]
141bf23c0: 4d8b5f40 mov r11, qword ptr [r15 + 0x40]
141bf23c4: 4c899df0000000 mov qword ptr [rbp + 0xf0], r11
141bf23cb: 4c8b9d40010000 mov r11, qword ptr [rbp + 0x140]
141bf23d2: 4c899da8000000 mov qword ptr [rbp + 0xa8], r11
141bf23d9: 4889bdb0000000 mov qword ptr [rbp + 0xb0], rdi
141bf23e0: 4c89adb8000000 mov qword ptr [rbp + 0xb8], r13
141bf23e7: f30f11bdc0000000 movss dword ptr [rbp + 0xc0], xmm7
141bf23ef: f3440f118dc4000000 movss dword ptr [rbp + 0xc4], xmm9
141bf23f8: f3440f1185c8000000 movss dword ptr [rbp + 0xc8], xmm8
141bf2401: f3440f11b5cc000000 movss dword ptr [rbp + 0xcc], xmm14
141bf240a: f30f1185d0000000 movss dword ptr [rbp + 0xd0], xmm0
141bf2412: 448985d4000000 mov dword ptr [rbp + 0xd4], r8d
141bf2419: 8995d8000000 mov dword ptr [rbp + 0xd8], edx
141bf241f: 44898ddc000000 mov dword ptr [rbp + 0xdc], r9d
141bf2426: 8985e0000000 mov dword ptr [rbp + 0xe0], eax
141bf242c: 4c8995e4000000 mov qword ptr [rbp + 0xe4], r10
141bf2433: 898dec000000 mov dword ptr [rbp + 0xec], ecx
141bf2439: 498b7f30 mov rdi, qword ptr [r15 + 0x30]
141bf243d: 493b7f20 cmp rdi, qword ptr [r15 + 0x20]
141bf2441: 7513 jne 0x141bf2456
141bf2443: 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
141bf244a: 488d15c7584401 lea rdx, [rip + 0x14458c7]
141bf2451: e8ea1097fe call 0x140563540 ; alloc::raw_vec::RawVec::grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<country_core::resources::time_of_day::TodSequenceEntry,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>
141bf2456: 488b9510010000 mov rdx, qword ptr [rbp + 0x110]
141bf245d: 488b4228 mov rax, qword ptr [rdx + 0x28]
141bf2461: 488d0cbf lea rcx, [rdi + rdi*4]
141bf2465: 48c1e104 shl rcx, 4
141bf2469: 0f1085a8000000 movups xmm0, xmmword ptr [rbp + 0xa8]
141bf2470: 0f108db8000000 movups xmm1, xmmword ptr [rbp + 0xb8]
141bf2477: 0f1095c8000000 movups xmm2, xmmword ptr [rbp + 0xc8]
141bf247e: 0f109dd8000000 movups xmm3, xmmword ptr [rbp + 0xd8]
141bf2485: 0f10a5e8000000 movups xmm4, xmmword ptr [rbp + 0xe8]
141bf248c: 0f11640840 movups xmmword ptr [rax + rcx + 0x40], xmm4
141bf2491: 0f115c0830 movups xmmword ptr [rax + rcx + 0x30], xmm3
141bf2496: 0f11540820 movups xmmword ptr [rax + rcx + 0x20], xmm2
141bf249b: 0f114c0810 movups xmmword ptr [rax + rcx + 0x10], xmm1
141bf24a0: 0f110408 movups xmmword ptr [rax + rcx], xmm0
141bf24a4: 48ffc7 inc rdi
141bf24a7: 48897a30 mov qword ptr [rdx + 0x30], rdi
141bf24ab: 48ff4240 inc qword ptr [rdx + 0x40]
141bf24af: 8b8518010000 mov eax, dword ptr [rbp + 0x118]
141bf24b5: 488b8d20010000 mov rcx, qword ptr [rbp + 0x120]
141bf24bc: 8901 mov dword ptr [rcx], eax
141bf24be: 498b542410 mov rdx, qword ptr [r12 + 0x10]
141bf24c3: 4839d3 cmp rbx, rdx
141bf24c6: 0f8320020000 jae 0x141bf26ec
141bf24cc: 498b442408 mov rax, qword ptr [r12 + 8]
141bf24d1: 4c896c3040 mov qword ptr [rax + rsi + 0x40], r13
141bf24d6: 4c89f3 mov rbx, r14
141bf24d9: 4c8b8508010000 mov r8, qword ptr [rbp + 0x108]
141bf24e0: e93bfaffff jmp 0x141bf1f20
141bf24e5: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
141bf24ee: 41b820000000 mov r8d, 0x20
141bf24f4: 41b901000000 mov r9d, 1
141bf24fa: 4889f9 mov rcx, rdi
141bf24fd: 4889da mov rdx, rbx
141bf2500: e87b81d200 call 0x14291a680 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
141bf2505: e958fcffff jmp 0x141bf2162
141bf250a: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
141bf2513: 41b838000000 mov r8d, 0x38
141bf2519: 41b901000000 mov r9d, 1
141bf251f: 4889f9 mov rcx, rdi
141bf2522: 4c89fa mov rdx, r15
141bf2525: e85681d200 call 0x14291a680 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
141bf252a: e9d5fcffff jmp 0x141bf2204
141bf252f: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
141bf2538: 41b828000000 mov r8d, 0x28
141bf253e: 41b901000000 mov r9d, 1
141bf2544: 4889f9 mov rcx, rdi
141bf2547: 4c89fa mov rdx, r15
141bf254a: e83181d200 call 0x14291a680 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
141bf254f: e987fdffff jmp 0x141bf22db
141bf2554: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
141bf255d: 41b818000000 mov r8d, 0x18
141bf2563: 41b901000000 mov r9d, 1
141bf2569: 4889f9 mov rcx, rdi
141bf256c: 4889da mov rdx, rbx
141bf256f: e80c81d200 call 0x14291a680 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
141bf2574: e902fbffff jmp 0x141bf207b
141bf2579: 8b8518010000 mov eax, dword ptr [rbp + 0x118]
141bf257f: 488b8d20010000 mov rcx, qword ptr [rbp + 0x120]
141bf2586: 8901 mov dword ptr [rcx], eax
141bf2588: 41c644241801 mov byte ptr [r12 + 0x18], 1
141bf258e: 80bd5701000000 cmp byte ptr [rbp + 0x157], 0
141bf2595: 744b je 0x141bf25e2
141bf2597: e8945b02ff call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
141bf259c: 4889c1 mov rcx, rax
141bf259f: 488b00 mov rax, qword ptr [rax]
141bf25a2: 4883f801 cmp rax, 1
141bf25a6: 0f85b2000000 jne 0x141bf265e
141bf25ac: 4883c108 add rcx, 8
141bf25b0: 48833900 cmp qword ptr [rcx], 0
141bf25b4: 0f85f9000000 jne 0x141bf26b3
141bf25ba: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
141bf25c1: 48898d48010000 mov qword ptr [rbp + 0x148], rcx
141bf25c8: 4883c108 add rcx, 8
141bf25cc: 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
141bf25d3: e8184b02ff call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
141bf25d8: 488b8548010000 mov rax, qword ptr [rbp + 0x148]
141bf25df: 48ff00 inc qword ptr [rax]
141bf25e2: 0f28b560010000 movaps xmm6, xmmword ptr [rbp + 0x160]
141bf25e9: 0f28bd70010000 movaps xmm7, xmmword ptr [rbp + 0x170]
141bf25f0: 440f288580010000 movaps xmm8, xmmword ptr [rbp + 0x180]
141bf25f8: 440f288d90010000 movaps xmm9, xmmword ptr [rbp + 0x190]
141bf2600: 440f2895a0010000 movaps xmm10, xmmword ptr [rbp + 0x1a0]
141bf2608: 440f289db0010000 movaps xmm11, xmmword ptr [rbp + 0x1b0]
141bf2610: 440f28a5c0010000 movaps xmm12, xmmword ptr [rbp + 0x1c0]
141bf2618: 440f28add0010000 movaps xmm13, xmmword ptr [rbp + 0x1d0]
141bf2620: 440f28b5e0010000 movaps xmm14, xmmword ptr [rbp + 0x1e0]
141bf2628: 440f28bdf0010000 movaps xmm15, xmmword ptr [rbp + 0x1f0]
141bf2630: 4881c488020000 add rsp, 0x288
141bf2637: 5b pop rbx
141bf2638: 5f pop rdi
141bf2639: 5e pop rsi
141bf263a: 415c pop r12
141bf263c: 415d pop r13
141bf263e: 415e pop r14
141bf2640: 415f pop r15
141bf2642: 5d pop rbp
141bf2643: c3 ret 
141bf2644: 4885c0 test rax, rax
141bf2647: 752d jne 0x141bf2676
141bf2649: 31d2 xor edx, edx
141bf264b: e8107fd200 call 0x14291a560 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
141bf2650: 4889c1 mov rcx, rax
141bf2653: 4885c0 test rax, rax
141bf2656: 0f85bef2ffff jne 0x141bf191a
141bf265c: eb18 jmp 0x141bf2676
141bf265e: 4885c0 test rax, rax
141bf2661: 7513 jne 0x141bf2676
141bf2663: 31d2 xor edx, edx
141bf2665: e8f67ed200 call 0x14291a560 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
141bf266a: 4889c1 mov rcx, rax
141bf266d: 4885c0 test rax, rax
141bf2670: 0f853affffff jne 0x141bf25b0
141bf2676: 488d0d03574401 lea rcx, [rip + 0x1445703]
141bf267d: e84e5ace00 call 0x1428d80d0 ; std::thread::local::panic_access_error
141bf2682: 48c785a800000000000000 mov qword ptr [rbp + 0xa8], 0
141bf268d: 488d05bc4c4401 lea rax, [rip + 0x1444cbc]
141bf2694: 4889442420 mov qword ptr [rsp + 0x20], rax
141bf2699: 488d5508 lea rdx, [rbp + 8]
141bf269d: 4c8d8da8000000 lea r9, [rbp + 0xa8]
141bf26a4: 31c9 xor ecx, ecx
141bf26a6: 4989d8 mov r8, rbx
141bf26a9: e89384cd00 call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
141bf26ae: e99a000000 jmp 0x141bf274d
141bf26b3: 488d0d36574401 lea rcx, [rip + 0x1445736]
141bf26ba: e8916bce00 call 0x1428d9250 ; core::cell::panic_already_borrowed
141bf26bf: 4c8d05eab03001 lea r8, [rip + 0x130b0ea]
141bf26c6: 4889d9 mov rcx, rbx
141bf26c9: e84a6ece00 call 0x1428d9518 ; core::panicking::panic_bounds_check
141bf26ce: eb7d jmp 0x141bf274d
141bf26d0: 488d0df1b03001 lea rcx, [rip + 0x130b0f1]
141bf26d7: e8346cce00 call 0x1428d9310 ; core::option::unwrap_failed
141bf26dc: eb6f jmp 0x141bf274d
141bf26de: 488d0dfbb03001 lea rcx, [rip + 0x130b0fb]
141bf26e5: e8266cce00 call 0x1428d9310 ; core::option::unwrap_failed
141bf26ea: eb61 jmp 0x141bf274d
141bf26ec: 4c8d0565b13001 lea r8, [rip + 0x130b165]
141bf26f3: 4889d9 mov rcx, rbx
141bf26f6: e81d6ece00 call 0x1428d9518 ; core::panicking::panic_bounds_check
141bf26fb: eb50 jmp 0x141bf274d
141bf26fd: 4c8d05f4b03001 lea r8, [rip + 0x130b0f4]
141bf2704: 4889d9 mov rcx, rbx
141bf2707: e80c6ece00 call 0x1428d9518 ; core::panicking::panic_bounds_check
141bf270c: eb3f jmp 0x141bf274d
141bf270e: 4c8d05fbb03001 lea r8, [rip + 0x130b0fb]
141bf2715: 4889d9 mov rcx, rbx
141bf2718: e8fb6dce00 call 0x1428d9518 ; core::panicking::panic_bounds_check
141bf271d: eb2e jmp 0x141bf274d
141bf271f: 4c8d0502b13001 lea r8, [rip + 0x130b102]
141bf2726: 4889d9 mov rcx, rbx
141bf2729: e8ea6dce00 call 0x1428d9518 ; core::panicking::panic_bounds_check
141bf272e: eb1d jmp 0x141bf274d
141bf2730: 4c8d0509b13001 lea r8, [rip + 0x130b109]
141bf2737: 4889d9 mov rcx, rbx
141bf273a: e8d96dce00 call 0x1428d9518 ; core::panicking::panic_bounds_check
141bf273f: eb0c jmp 0x141bf274d
141bf2741: 488d0d50b03001 lea rcx, [rip + 0x130b050]
141bf2748: e8c36bce00 call 0x1428d9310 ; core::option::unwrap_failed
141bf274d: 0f0b ud2 
141bf274f: 90 nop 
141bf2750: 4889542410 mov qword ptr [rsp + 0x10], rdx
141bf2755: 55 push rbp
141bf2756: 4157 push r15
141bf2758: 4156 push r14
141bf275a: 4155 push r13
141bf275c: 4154 push r12
141bf275e: 56 push rsi
141bf275f: 57 push rdi
141bf2760: 53 push rbx
141bf2761: 4881ecd8000000 sub rsp, 0xd8
141bf2768: 488daa80000000 lea rbp, [rdx + 0x80]
141bf276f: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
141bf2775: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
141bf277b: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
141bf2781: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
141bf2787: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
141bf278d: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
141bf2796: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
141bf279f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
141bf27a8: 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
141bf27b0: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
141bf27b8: 488b8548010000 mov rax, qword ptr [rbp + 0x148]
141bf27bf: 48ff00 inc qword ptr [rax]
141bf27c2: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
141bf27ca: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
141bf27d2: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
141bf27db: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
141bf27e4: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
141bf27ed: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
141bf27f3: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
141bf27f9: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
141bf27ff: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
141bf2805: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
141bf280b: 4881c4d8000000 add rsp, 0xd8
141bf2812: 5b pop rbx
141bf2813: 5f pop rdi
141bf2814: 5e pop rsi
141bf2815: 415c pop r12
141bf2817: 415d pop r13
141bf2819: 415e pop r14
141bf281b: 415f pop r15
141bf281d: 5d pop rbp
141bf281e: c3 ret 
141bf281f: 90 nop 
141bf2820: 4889542410 mov qword ptr [rsp + 0x10], rdx
141bf2825: 55 push rbp
141bf2826: 4157 push r15
141bf2828: 4156 push r14
141bf282a: 4155 push r13
141bf282c: 4154 push r12
141bf282e: 56 push rsi
141bf282f: 57 push rdi
141bf2830: 53 push rbx
141bf2831: 4881ecd8000000 sub rsp, 0xd8
141bf2838: 488daa80000000 lea rbp, [rdx + 0x80]
141bf283f: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
141bf2845: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
141bf284b: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
141bf2851: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
141bf2857: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
141bf285d: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
141bf2866: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
141bf286f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
141bf2878: 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
141bf2880: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
141bf2888: 48837d7800 cmp qword ptr [rbp + 0x78], 0
141bf288d: 7423 je 0x141bf28b2
141bf288f: 488d8580000000 lea rax, [rbp + 0x80]
141bf2896: 48894568 mov qword ptr [rbp + 0x68], rax
141bf289a: 488d4568 lea rax, [rbp + 0x68]
141bf289e: 48894560 mov qword ptr [rbp + 0x60], rax
141bf28a2: 488d0df7ad3001 lea rcx, [rip + 0x130adf7]
141bf28a9: 488d5560 lea rdx, [rbp + 0x60]
141bf28ad: e89e676800 call 0x142279050 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
141bf28b2: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
141bf28ba: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
141bf28c2: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
141bf28cb: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
141bf28d4: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
141bf28dd: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
141bf28e3: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
141bf28e9: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
141bf28ef: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
141bf28f5: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
141bf28fb: 4881c4d8000000 add rsp, 0xd8
141bf2902: 5b pop rbx
141bf2903: 5f pop rdi
141bf2904: 5e pop rsi
141bf2905: 415c pop r12
141bf2907: 415d pop r13
141bf2909: 415e pop r14
141bf290b: 415f pop r15
141bf290d: 5d pop rbp
141bf290e: c3 ret 
141bf290f: 90 nop 
141bf2910: 4889542410 mov qword ptr [rsp + 0x10], rdx
141bf2915: 55 push rbp
141bf2916: 4157 push r15
141bf2918: 4156 push r14
141bf291a: 4155 push r13
141bf291c: 4154 push r12
141bf291e: 56 push rsi
141bf291f: 57 push rdi
141bf2920: 53 push rbx
141bf2921: 4881ecd8000000 sub rsp, 0xd8
141bf2928: 488daa80000000 lea rbp, [rdx + 0x80]
141bf292f: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
141bf2935: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
141bf293b: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
141bf2941: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
141bf2947: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
141bf294d: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
141bf2956: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
141bf295f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
141bf2968: 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
141bf2970: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
141bf2978: 488b8548010000 mov rax, qword ptr [rbp + 0x148]
141bf297f: 48ff00 inc qword ptr [rax]
141bf2982: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
141bf298a: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
141bf2992: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
141bf299b: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
141bf29a4: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
141bf29ad: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
141bf29b3: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
141bf29b9: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
141bf29bf: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
141bf29c5: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
141bf29cb: 4881c4d8000000 add rsp, 0xd8
141bf29d2: 5b pop rbx
141bf29d3: 5f pop rdi
141bf29d4: 5e pop rsi
141bf29d5: 415c pop r12
141bf29d7: 415d pop r13
141bf29d9: 415e pop r14
141bf29db: 415f pop r15
141bf29dd: 5d pop rbp
141bf29de: c3 ret 
141bf29df: 90 nop 
141bf29e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
141bf29e5: 55 push rbp
141bf29e6: 4157 push r15
141bf29e8: 4156 push r14
141bf29ea: 4155 push r13
141bf29ec: 4154 push r12
141bf29ee: 56 push rsi
141bf29ef: 57 push rdi
141bf29f0: 53 push rbx
141bf29f1: 4881ecd8000000 sub rsp, 0xd8
141bf29f8: 488daa80000000 lea rbp, [rdx + 0x80]
141bf29ff: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
141bf2a05: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
141bf2a0b: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
141bf2a11: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
141bf2a17: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
141bf2a1d: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
141bf2a26: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
141bf2a2f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
141bf2a38: 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
141bf2a40: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
141bf2a48: 488b8540010000 mov rax, qword ptr [rbp + 0x140]
141bf2a4f: f048ff08 lock dec qword ptr [rax]
141bf2a53: 750c jne 0x141bf2a61
141bf2a55: 488d8da8000000 lea rcx, [rbp + 0xa8]
141bf2a5c: e8cfc0a5fe call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
141bf2a61: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
141bf2a69: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
141bf2a71: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
141bf2a7a: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
141bf2a83: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
141bf2a8c: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
141bf2a92: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
141bf2a98: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
141bf2a9e: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
141bf2aa4: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
141bf2aaa: 4881c4d8000000 add rsp, 0xd8
141bf2ab1: 5b pop rbx
141bf2ab2: 5f pop rdi
141bf2ab3: 5e pop rsi
141bf2ab4: 415c pop r12
141bf2ab6: 415d pop r13
141bf2ab8: 415e pop r14
141bf2aba: 415f pop r15
141bf2abc: 5d pop rbp
141bf2abd: c3 ret 
141bf2abe: 6690 nop 
141bf2ac0: 4889542410 mov qword ptr [rsp + 0x10], rdx
141bf2ac5: 55 push rbp
141bf2ac6: 4157 push r15
141bf2ac8: 4156 push r14
141bf2aca: 4155 push r13
141bf2acc: 4154 push r12
141bf2ace: 56 push rsi
141bf2acf: 57 push rdi
141bf2ad0: 53 push rbx
141bf2ad1: 4881ecd8000000 sub rsp, 0xd8
141bf2ad8: 488daa80000000 lea rbp, [rdx + 0x80]
141bf2adf: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
141bf2ae5: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
141bf2aeb: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
141bf2af1: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
141bf2af7: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
141bf2afd: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
141bf2b06: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
141bf2b0f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
141bf2b18: 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
141bf2b20: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
141bf2b28: 488b8540010000 mov rax, qword ptr [rbp + 0x140]
141bf2b2f: f048ff08 lock dec qword ptr [rax]
141bf2b33: 750c jne 0x141bf2b41
141bf2b35: 488d8da8000000 lea rcx, [rbp + 0xa8]
141bf2b3c: e8efbfa5fe call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
141bf2b41: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
141bf2b49: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
141bf2b51: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
141bf2b5a: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
141bf2b63: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
141bf2b6c: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
141bf2b72: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
141bf2b78: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
141bf2b7e: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
141bf2b84: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
141bf2b8a: 4881c4d8000000 add rsp, 0xd8
141bf2b91: 5b pop rbx
141bf2b92: 5f pop rdi
141bf2b93: 5e pop rsi
141bf2b94: 415c pop r12
141bf2b96: 415d pop r13
141bf2b98: 415e pop r14
141bf2b9a: 415f pop r15
141bf2b9c: 5d pop rbp
141bf2b9d: c3 ret 

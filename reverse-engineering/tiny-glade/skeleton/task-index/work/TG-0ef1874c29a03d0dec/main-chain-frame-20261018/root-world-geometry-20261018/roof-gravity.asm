140a29b10 55 push rbp
140a29b11 4156 push r14
140a29b13 56 push rsi
140a29b14 57 push rdi
140a29b15 53 push rbx
140a29b16 4881ecd0030000 sub rsp, 0x3d0
140a29b1d 488dac2480000000 lea rbp, [rsp + 0x80]
140a29b25 48c78548030000feffffff mov qword ptr [rbp + 0x348], 0xfffffffffffffffe
140a29b30 4889d6 mov rsi, rdx
140a29b33 4889cf mov rdi, rcx
140a29b36 4883ba8800000000 cmp qword ptr [rdx + 0x88], 0
140a29b3e 48bbac41a8556adacf65 movabs rbx, 0x65cfda6a55a841ac
140a29b48 49be5593676e78cc2682 movabs r14, 0x8226cc786e679355
140a29b52 0f849f000000 je 0x140a29bf7
140a29b58 488b4670 mov rax, qword ptr [rsi + 0x70]
140a29b5c 488b4e78 mov rcx, qword ptr [rsi + 0x78]
140a29b60 4889ca mov rdx, rcx
140a29b63 4821da and rdx, rbx
140a29b66 4531c0 xor r8d, r8d
140a29b69 660f6f059f68f201 movdqa xmm0, xmmword ptr [rip + 0x1f2689f]
140a29b71 660f76c9 pcmpeqd xmm1, xmm1
140a29b75 f30f6f1410 movdqu xmm2, xmmword ptr [rax + rdx]
140a29b7a 660f6fda movdqa xmm3, xmm2
140a29b7e 660f74d8 pcmpeqb xmm3, xmm0
140a29b82 66440fd7cb pmovmskb r9d, xmm3
140a29b87 4585c9 test r9d, r9d
140a29b8a 7531 jne 0x140a29bbd
140a29b8c 660f74d1 pcmpeqb xmm2, xmm1
140a29b90 66440fd7ca pmovmskb r9d, xmm2
140a29b95 4585c9 test r9d, r9d
140a29b98 755d jne 0x140a29bf7
140a29b9a 4c01c2 add rdx, r8
140a29b9d 4883c210 add rdx, 0x10
140a29ba1 4983c010 add r8, 0x10
140a29ba5 4821ca and rdx, rcx
140a29ba8 ebcb jmp 0x140a29b75
140a29baa 660f1f440000 nop word ptr [rax + rax]
140a29bb0 458d51ff lea r10d, [r9 - 1]
140a29bb4 664521ca and r10w, r9w
140a29bb8 4589d1 mov r9d, r10d
140a29bbb 74cf je 0x140a29b8c
140a29bbd f3450fbcd1 tzcnt r10d, r9d
140a29bc2 4901d2 add r10, rdx
140a29bc5 4921ca and r10, rcx
140a29bc8 49f7da neg r10
140a29bcb 4f8d1452 lea r10, [r10 + r10*2]
140a29bcf 4e3974d0e8 cmp qword ptr [rax + r10*8 - 0x18], r14
140a29bd4 75da jne 0x140a29bb0
140a29bd6 4a395cd0f0 cmp qword ptr [rax + r10*8 - 0x10], rbx
140a29bdb 75d3 jne 0x140a29bb0
140a29bdd 4a8d04d0 lea rax, [rax + r10*8]
140a29be1 488b58f8 mov rbx, qword ptr [rax - 8]
140a29be5 48399e18020000 cmp qword ptr [rsi + 0x218], rbx
140a29bec 0f870a010000 ja 0x140a29cfc
140a29bf2 e966040000 jmp 0x140a2a05d
140a29bf7 8b8690000000 mov eax, dword ptr [rsi + 0x90]
140a29bfd 3dfdffff3f cmp eax, 0x3ffffffd
140a29c02 0f871a040000 ja 0x140a2a022
140a29c08 8d4801 lea ecx, [rax + 1]
140a29c0b f00fb18e90000000 lock cmpxchg dword ptr [rsi + 0x90], ecx
140a29c13 0f8509040000 jne 0x140a2a022
140a29c19 0fb68698000000 movzx eax, byte ptr [rsi + 0x98]
140a29c20 4883bef000000000 cmp qword ptr [rsi + 0xf0], 0
140a29c28 0f8415040000 je 0x140a2a043
140a29c2e 488b86d8000000 mov rax, qword ptr [rsi + 0xd8]
140a29c35 488b8ee0000000 mov rcx, qword ptr [rsi + 0xe0]
140a29c3c 4889ca mov rdx, rcx
140a29c3f 4821da and rdx, rbx
140a29c42 4531c0 xor r8d, r8d
140a29c45 660f6f05c367f201 movdqa xmm0, xmmword ptr [rip + 0x1f267c3]
140a29c4d 660f76c9 pcmpeqd xmm1, xmm1
140a29c51 f30f6f1410 movdqu xmm2, xmmword ptr [rax + rdx]
140a29c56 660f6fda movdqa xmm3, xmm2
140a29c5a 660f74d8 pcmpeqb xmm3, xmm0
140a29c5e 66440fd7cb pmovmskb r9d, xmm3
140a29c63 4585c9 test r9d, r9d
140a29c66 7535 jne 0x140a29c9d
140a29c68 660f74d1 pcmpeqb xmm2, xmm1
140a29c6c 66440fd7ca pmovmskb r9d, xmm2
140a29c71 4585c9 test r9d, r9d
140a29c74 0f85c9030000 jne 0x140a2a043
140a29c7a 4c01c2 add rdx, r8
140a29c7d 4883c210 add rdx, 0x10
140a29c81 4983c010 add r8, 0x10
140a29c85 4821ca and rdx, rcx
140a29c88 ebc7 jmp 0x140a29c51
140a29c8a 660f1f440000 nop word ptr [rax + rax]
140a29c90 458d51ff lea r10d, [r9 - 1]
140a29c94 664521ca and r10w, r9w
140a29c98 4589d1 mov r9d, r10d
140a29c9b 74cb je 0x140a29c68
140a29c9d f3450fbcd9 tzcnt r11d, r9d
140a29ca2 4901d3 add r11, rdx
140a29ca5 4921cb and r11, rcx
140a29ca8 4d89da mov r10, r11
140a29cab 49f7da neg r10
140a29cae 49c1e303 shl r11, 3
140a29cb2 49c1e207 shl r10, 7
140a29cb6 4d29da sub r10, r11
140a29cb9 4e39b41078ffffff cmp qword ptr [rax + r10 - 0x88], r14
140a29cc1 75cd jne 0x140a29c90
140a29cc3 4a395c1080 cmp qword ptr [rax + r10 - 0x80], rbx
140a29cc8 75c6 jne 0x140a29c90
140a29cca 4c01d0 add rax, r10
140a29ccd 488b58f8 mov rbx, qword ptr [rax - 8]
140a29cd1 baffffffff mov edx, 0xffffffff
140a29cd6 f00fc19690000000 lock xadd dword ptr [rsi + 0x90], edx
140a29cde ffca dec edx
140a29ce0 89d0 mov eax, edx
140a29ce2 25ffffffbf and eax, 0xbfffffff
140a29ce7 f7d8 neg eax
140a29ce9 0f80b8030000 jo 0x140a2a0a7
140a29cef 48399e18020000 cmp qword ptr [rsi + 0x218], rbx
140a29cf6 0f8661030000 jbe 0x140a2a05d
140a29cfc 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140a29d03 488b0cd8 mov rcx, qword ptr [rax + rbx*8]
140a29d07 4885c9 test rcx, rcx
140a29d0a 0f844d030000 je 0x140a2a05d
140a29d10 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
140a29d17 48f7d1 not rcx
140a29d1a 488d0c49 lea rcx, [rcx + rcx*2]
140a29d1e 48c1e105 shl rcx, 5
140a29d22 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a29d28 0f842f030000 je 0x140a2a05d
140a29d2e 4801c8 add rax, rcx
140a29d31 488b4810 mov rcx, qword ptr [rax + 0x10]
140a29d35 488d15bc462002 lea rdx, [rip + 0x22046bc]
140a29d3c e8dff9f3ff call 0x140969720 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
140a29d41 4885c0 test rax, rax
140a29d44 0f84a8030000 je 0x140a2a0f2
140a29d4a 4889c3 mov rbx, rax
140a29d4d 488d0579460702 lea rax, [rip + 0x2074679]
140a29d54 488945c0 mov qword ptr [rbp - 0x40], rax
140a29d58 48c745c835000000 mov qword ptr [rbp - 0x38], 0x35
140a29d60 48837b1835 cmp qword ptr [rbx + 0x18], 0x35
140a29d65 0f85fe020000 jne 0x140a2a069
140a29d6b 488b5310 mov rdx, qword ptr [rbx + 0x10]
140a29d6f 488d0d57460702 lea rcx, [rip + 0x2074657]
140a29d76 41b835000000 mov r8d, 0x35
140a29d7c e86f9eef01 call 0x142923bf0 ; 
140a29d81 85c0 test eax, eax
140a29d83 0f85e0020000 jne 0x140a2a069
140a29d89 488b0b mov rcx, qword ptr [rbx]
140a29d8c 488b4308 mov rax, qword ptr [rbx + 8]
140a29d90 ff5018 call qword ptr [rax + 0x18]
140a29d93 4885c0 test rax, rax
140a29d96 0f8456030000 je 0x140a2a0f2
140a29d9c 4889c3 mov rbx, rax
140a29d9f 4c8d75c0 lea r14, [rbp - 0x40]
140a29da3 4c89f1 mov rcx, r14
140a29da6 4889f2 mov rdx, rsi
140a29da9 e85201c300 call 0x141659f00 ; bevy_ecs::query::state::QueryState::new<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >
140a29dae 4c89f1 mov rcx, r14
140a29db1 4889f2 mov rdx, rsi
140a29db4 e8d7e4c200 call 0x141658290 ; bevy_ecs::query::state::QueryState::update_archetypes_unsafe_world_cell<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >
140a29db9 488b96a0050000 mov rdx, qword ptr [rsi + 0x5a0]
140a29dc0 488b4dc0 mov rcx, qword ptr [rbp - 0x40]
140a29dc4 4839d1 cmp rcx, rdx
140a29dc7 0f85cc020000 jne 0x140a2a099
140a29dcd 8b86b0050000 mov eax, dword ptr [rsi + 0x5b0]
140a29dd3 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
140a29dda 488b8dc0000000 mov rcx, qword ptr [rbp + 0xc0]
140a29de1 488d0c88 lea rcx, [rax + rcx*4]
140a29de5 0fb695c8000000 movzx edx, byte ptr [rbp + 0xc8]
140a29dec 660fefc0 pxor xmm0, xmm0
140a29df0 f30f7f8508030000 movdqu xmmword ptr [rbp + 0x308], xmm0
140a29df8 f30f7f85f8020000 movdqu xmmword ptr [rbp + 0x2f8], xmm0
140a29e00 f30f7f85e8020000 movdqu xmmword ptr [rbp + 0x2e8], xmm0
140a29e08 48c7851803000000000000 mov qword ptr [rbp + 0x318], 0
140a29e13 4c8d86f8000000 lea r8, [rsi + 0xf8]
140a29e1a 4c8d8ea0010000 lea r9, [rsi + 0x1a0]
140a29e21 4889b5a0020000 mov qword ptr [rbp + 0x2a0], rsi
140a29e28 4c898da8020000 mov qword ptr [rbp + 0x2a8], r9
140a29e2f 4c8985b0020000 mov qword ptr [rbp + 0x2b0], r8
140a29e36 4c89b5b8020000 mov qword ptr [rbp + 0x2b8], r14
140a29e3d 488985c0020000 mov qword ptr [rbp + 0x2c0], rax
140a29e44 48898dc8020000 mov qword ptr [rbp + 0x2c8], rcx
140a29e4b 48c785d002000008000000 mov qword ptr [rbp + 0x2d0], 8
140a29e56 48c785d802000000000000 mov qword ptr [rbp + 0x2d8], 0
140a29e61 48c785e002000008000000 mov qword ptr [rbp + 0x2e0], 8
140a29e6c 889520030000 mov byte ptr [rbp + 0x320], dl
140a29e72 8b8528030000 mov eax, dword ptr [rbp + 0x328]
140a29e78 8b8d2b030000 mov ecx, dword ptr [rbp + 0x32b]
140a29e7e 898521030000 mov dword ptr [rbp + 0x321], eax
140a29e84 898d24030000 mov dword ptr [rbp + 0x324], ecx
140a29e8a 48c7855801000000000000 mov qword ptr [rbp + 0x158], 0
140a29e95 48c785f801000000000000 mov qword ptr [rbp + 0x1f8], 0
140a29ea0 48899d98020000 mov qword ptr [rbp + 0x298], rbx
140a29ea7 4c8d0582060a02 lea r8, [rip + 0x20a0682]
140a29eae 488d8d28030000 lea rcx, [rbp + 0x328]
140a29eb5 488d9558010000 lea rdx, [rbp + 0x158]
140a29ebc e81f3a8e00 call 0x14130d8e0 ; alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >
140a29ec1 c6854703000001 mov byte ptr [rbp + 0x347], 1
140a29ec8 488d4dc0 lea rcx, [rbp - 0x40]
140a29ecc e80f6dbaff call 0x1405d0be0 ; core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::components::WoodenGate>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,ref$<country_core::components::AutoClutterConvertableToDecoClutter>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::components::TreeAmbience>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::Ui2dWsExtent>,ref$<country_core::components::Ui2dImage> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::components::ClothlineCloth>,ref$<country_core::components::NormalMap> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::ClothInstanceData> > > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::window_lights::WindowLight>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> > > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> >,ref$<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram> >,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<enum2$<country_core::components::drawable::DrawTopology> > > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<system_onboarding::hint_orchestrator::HintType>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<enum2$<system_onboarding::hint_orchestrator::HintPlacement> >,ref$<system_onboarding::hint_display_system::HintAnimation> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<system_onboarding::hint_orchestrator::HintLayoutMemo>,ref_mut$<system_onboarding::hint_display_system::HintSpring>,ref$<enum2$<system_onboarding::hint_orchestrator::HintPlacement> > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<system_onboarding::hint_orchestrator::HintLayoutMemo>,ref$<system_onboarding::hint_display_system::HintAnimation>,enum2$<core::option::Option<ref$<enum2$<system_onboarding::hint_orchestrator::BindingHintDisplay> > > >,ref$<system_onboarding::hint_display_system::HintSpring>,enum2$<core::option::Option<ref$<system_onboarding::hint_display_system::HintText> > >,enum2$<core::option::Option<ref$<system_onboarding::hint_orchestrator::HintProgressBar> > >,enum2$<core::option::Option<ref$<system_onboarding::hint_orchestrator::InteractionPrompt> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::components::WoodenGate>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::components::WoodenGate>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::AutoClutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::autoclutter::WindowShutter> > >,enum2$<core::option::Option<ref$<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::components::TreeMarkedForDeletion>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::resources::critters::Duck>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Firefly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Duck>,ref$<enum2$<country_core::resources::critters::DuckState> >,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref_mut$<enum2$<country_core::resources::critters::DuckState> >,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier>,ref_mut$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> > >
140a29ed1 c6854703000001 mov byte ptr [rbp + 0x347], 1
140a29ed8 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a29edd 4889742428 mov qword ptr [rsp + 0x28], rsi
140a29ee2 48c74424201b000000 mov qword ptr [rsp + 0x20], 0x1b
140a29eeb 488d15ce0d0a02 lea rdx, [rip + 0x20a0dce]
140a29ef2 4c8d0dd10d0a02 lea r9, [rip + 0x20a0dd1]
140a29ef9 488d8d58010000 lea rcx, [rbp + 0x158]
140a29f00 41b80a000000 mov r8d, 0xa
140a29f06 e89593bd00 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a29f0b c6854703000001 mov byte ptr [rbp + 0x347], 1
140a29f12 488d4dc0 lea rcx, [rbp - 0x40]
140a29f16 488d9558010000 lea rdx, [rbp + 0x158]
140a29f1d 49b82fde0501d73c48c3 movabs r8, 0xc3483cd70105de2f
140a29f27 e8d428be00 call 0x14160c800 ; country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
140a29f2c 488d4e38 lea rcx, [rsi + 0x38]
140a29f30 c6854603000001 mov byte ptr [rbp + 0x346], 1
140a29f37 48ba29f7805b1962c34b movabs rdx, 0x4bc362195b80f729
140a29f41 49b8fbcecb80ce982798 movabs r8, 0x982798ce80cbcefb
140a29f4b e8a0376bff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
140a29f50 4885c0 test rax, rax
140a29f53 0f8484010000 je 0x140a2a0dd
140a29f59 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140a29f60 0f8677010000 jbe 0x140a2a0dd
140a29f66 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140a29f6d 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140a29f71 4885c9 test rcx, rcx
140a29f74 0f8463010000 je 0x140a2a0dd
140a29f7a 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
140a29f81 48f7d1 not rcx
140a29f84 488d0c49 lea rcx, [rcx + rcx*2]
140a29f88 48c1e105 shl rcx, 5
140a29f8c 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a29f92 0f8445010000 je 0x140a2a0dd
140a29f98 80bdf600000008 cmp byte ptr [rbp + 0xf6], 8
140a29f9f 742e je 0x140a29fcf
140a29fa1 4801c8 add rax, rcx
140a29fa4 488b4010 mov rax, qword ptr [rax + 0x10]
140a29fa8 660f6e5828 movd xmm3, dword ptr [rax + 0x28]
140a29fad 488d8dd0000000 lea rcx, [rbp + 0xd0]
140a29fb4 bae8000000 mov edx, 0xe8
140a29fb9 480395f8000000 add rdx, qword ptr [rbp + 0xf8]
140a29fc0 49b82f91af43ce451a60 movabs r8, 0x601a45ce43af912f
140a29fca e8611af4ff call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
140a29fcf 488db558010000 lea rsi, [rbp + 0x158]
140a29fd6 488d55c0 lea rdx, [rbp - 0x40]
140a29fda 41b890010000 mov r8d, 0x190
140a29fe0 4889f1 mov rcx, rsi
140a29fe3 e8189cef01 call 0x142923c00 ; 
140a29fe8 488b8538030000 mov rax, qword ptr [rbp + 0x338]
140a29fef 488945d0 mov qword ptr [rbp - 0x30], rax
140a29ff3 f30f6f8528030000 movdqu xmm0, xmmword ptr [rbp + 0x328]
140a29ffb 660f7f45c0 movdqa xmmword ptr [rbp - 0x40], xmm0
140a2a000 c6854703000000 mov byte ptr [rbp + 0x347], 0
140a2a007 488d55c0 lea rdx, [rbp - 0x40]
140a2a00b 4889f1 mov rcx, rsi
140a2a00e e88d0abe00 call 0x14160aaa0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_with<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$1>
140a2a013 90 nop 
140a2a014 4881c4d0030000 add rsp, 0x3d0
140a2a01b 5b pop rbx
140a2a01c 5f pop rdi
140a2a01d 5e pop rsi
140a2a01e 415e pop r14
140a2a020 5d pop rbp
140a2a021 c3 ret 
140a2a022 488d8e90000000 lea rcx, [rsi + 0x90]
140a2a029 e802eaea01 call 0x1428d8a30 ; std::sys::sync::rwlock::futex::RwLock::read_contended
140a2a02e 0fb68698000000 movzx eax, byte ptr [rsi + 0x98]
140a2a035 4883bef000000000 cmp qword ptr [rsi + 0xf0], 0
140a2a03d 0f85ebfbffff jne 0x140a29c2e
140a2a043 baffffffff mov edx, 0xffffffff
140a2a048 f00fc19690000000 lock xadd dword ptr [rsi + 0x90], edx
140a2a050 ffca dec edx
140a2a052 89d0 mov eax, edx
140a2a054 25ffffffbf and eax, 0xbfffffff
140a2a059 f7d8 neg eax
140a2a05b 7065 jo 0x140a2a0c2
140a2a05d 488d0d2c0c0a02 lea rcx, [rip + 0x20a0c2c]
140a2a064 e8a7f2ea01 call 0x1428d9310 ; core::option::unwrap_failed
140a2a069 48c7855801000000000000 mov qword ptr [rbp + 0x158], 0
140a2a074 4883c310 add rbx, 0x10
140a2a078 488d0501412002 lea rax, [rip + 0x2204101]
140a2a07f 4889442420 mov qword ptr [rsp + 0x20], rax
140a2a084 488d55c0 lea rdx, [rbp - 0x40]
140a2a088 4c8d8d58010000 lea r9, [rbp + 0x158]
140a2a08f 31c9 xor ecx, ecx
140a2a091 4989d8 mov r8, rbx
140a2a094 e8a80aea01 call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
140a2a099 4c8d0548782502 lea r8, [rip + 0x2257848]
140a2a0a0 e85b51ea01 call 0x1428cf200 ; bevy_ecs::query::state::impl$2::validate_world::panic_mismatched
140a2a0a5 eb49 jmp 0x140a2a0f0
140a2a0a7 488d8e90000000 lea rcx, [rsi + 0x90]
140a2a0ae e88decea01 call 0x1428d8d40 ; std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
140a2a0b3 48399e18020000 cmp qword ptr [rsi + 0x218], rbx
140a2a0ba 0f873cfcffff ja 0x140a29cfc
140a2a0c0 eb9b jmp 0x140a2a05d
140a2a0c2 4881c690000000 add rsi, 0x90
140a2a0c9 4889f1 mov rcx, rsi
140a2a0cc e86fecea01 call 0x1428d8d40 ; std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
140a2a0d1 488d0db80b0a02 lea rcx, [rip + 0x20a0bb8]
140a2a0d8 e833f2ea01 call 0x1428d9310 ; core::option::unwrap_failed
140a2a0dd c6854603000001 mov byte ptr [rbp + 0x346], 1
140a2a0e4 488d0dfd0b0a02 lea rcx, [rip + 0x20a0bfd]
140a2a0eb e820f2ea01 call 0x1428d9310 ; core::option::unwrap_failed
140a2a0f0 0f0b ud2 
140a2a0f2 488d0daf0b0a02 lea rcx, [rip + 0x20a0baf]
140a2a0f9 e812f2ea01 call 0x1428d9310 ; core::option::unwrap_failed
140a2a0fe cc int3 
140a2a0ff 90 nop 
140a2a100 4889542410 mov qword ptr [rsp + 0x10], rdx
140a2a105 55 push rbp
140a2a106 4156 push r14
140a2a108 56 push rsi
140a2a109 57 push rdi
140a2a10a 53 push rbx
140a2a10b 4883ec40 sub rsp, 0x40
140a2a10f 488daa80000000 lea rbp, [rdx + 0x80]
140a2a116 488d4dc0 lea rcx, [rbp - 0x40]
140a2a11a e8c16abaff call 0x1405d0be0 ; core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::components::WoodenGate>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,ref$<country_core::components::AutoClutterConvertableToDecoClutter>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::components::TreeAmbience>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::Ui2dWsExtent>,ref$<country_core::components::Ui2dImage> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::components::ClothlineCloth>,ref$<country_core::components::NormalMap> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::ClothInstanceData> > > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::window_lights::WindowLight>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> > > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::vao::Vao> >,ref$<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram> >,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<enum2$<country_core::components::drawable::DrawTopology> > > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DesignatedRenderPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::UiPass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::WireframePass> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::DontCastShadow> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::OcclusionCulled> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<system_onboarding::hint_orchestrator::HintType>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<enum2$<system_onboarding::hint_orchestrator::HintPlacement> >,ref$<system_onboarding::hint_display_system::HintAnimation> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<system_onboarding::hint_orchestrator::HintLayoutMemo>,ref_mut$<system_onboarding::hint_display_system::HintSpring>,ref$<enum2$<system_onboarding::hint_orchestrator::HintPlacement> > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<system_onboarding::hint_orchestrator::HintLayoutMemo>,ref$<system_onboarding::hint_display_system::HintAnimation>,enum2$<core::option::Option<ref$<enum2$<system_onboarding::hint_orchestrator::BindingHintDisplay> > > >,ref$<system_onboarding::hint_display_system::HintSpring>,enum2$<core::option::Option<ref$<system_onboarding::hint_display_system::HintText> > >,enum2$<core::option::Option<ref$<system_onboarding::hint_orchestrator::HintProgressBar> > >,enum2$<core::option::Option<ref$<system_onboarding::hint_orchestrator::InteractionPrompt> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::components::WoodenGate>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::components::WoodenGate>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::AutoClutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::autoclutter::WindowShutter> > >,enum2$<core::option::Option<ref$<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::components::TreeMarkedForDeletion>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<ref$<country_core::resources::critters::Duck>,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Firefly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Duck>,ref$<enum2$<country_core::resources::critters::DuckState> >,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> > > | core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref_mut$<enum2$<country_core::resources::critters::DuckState> >,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier>,ref_mut$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> > >
140a2a11f 90 nop 
140a2a120 4883c440 add rsp, 0x40
140a2a124 5b pop rbx
140a2a125 5f pop rdi
140a2a126 5e pop rsi
140a2a127 415e pop r14
140a2a129 5d pop rbp
140a2a12a c3 ret 
140a2a12b 0f1f440000 nop dword ptr [rax + rax]
140a2a130 4889542410 mov qword ptr [rsp + 0x10], rdx
140a2a135 55 push rbp
140a2a136 4156 push r14
140a2a138 56 push rsi
140a2a139 57 push rdi
140a2a13a 53 push rbx
140a2a13b 4883ec40 sub rsp, 0x40
140a2a13f 488daa80000000 lea rbp, [rdx + 0x80]
140a2a146 c6854603000000 mov byte ptr [rbp + 0x346], 0
140a2a14d 488d4dc0 lea rcx, [rbp - 0x40]
140a2a151 e85acee6ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a2a156 c6854603000000 mov byte ptr [rbp + 0x346], 0
140a2a15d 4883c440 add rsp, 0x40
140a2a161 5b pop rbx
140a2a162 5f pop rdi
140a2a163 5e pop rsi
140a2a164 415e pop r14
140a2a166 5d pop rbp
140a2a167 c3 ret 
140a2a168 0f1f840000000000 nop dword ptr [rax + rax]
140a2a170 4889542410 mov qword ptr [rsp + 0x10], rdx
140a2a175 55 push rbp
140a2a176 4156 push r14
140a2a178 56 push rsi
140a2a179 57 push rdi
140a2a17a 53 push rbx
140a2a17b 4883ec40 sub rsp, 0x40
140a2a17f 488daa80000000 lea rbp, [rdx + 0x80]
140a2a186 80bd4603000000 cmp byte ptr [rbp + 0x346], 0
140a2a18d 741b je 0x140a2a1aa
140a2a18f 488d4dc0 lea rcx, [rbp - 0x40]
140a2a193 e818cee6ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a2a198 c6854703000001 mov byte ptr [rbp + 0x347], 1
140a2a19f 4883c440 add rsp, 0x40
140a2a1a3 5b pop rbx
140a2a1a4 5f pop rdi
140a2a1a5 5e pop rsi
140a2a1a6 415e pop r14
140a2a1a8 5d pop rbp
140a2a1a9 c3 ret 
140a2a1aa c6854703000001 mov byte ptr [rbp + 0x347], 1
140a2a1b1 4883c440 add rsp, 0x40
140a2a1b5 5b pop rbx
140a2a1b6 5f pop rdi
140a2a1b7 5e pop rsi
140a2a1b8 415e pop r14
140a2a1ba 5d pop rbp
140a2a1bb c3 ret 
140a2a1bc 0f1f4000 nop dword ptr [rax]
140a2a1c0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a2a1c5 55 push rbp
140a2a1c6 4156 push r14
140a2a1c8 56 push rsi
140a2a1c9 57 push rdi
140a2a1ca 53 push rbx
140a2a1cb 4883ec40 sub rsp, 0x40
140a2a1cf 488daa80000000 lea rbp, [rdx + 0x80]
140a2a1d6 80bd4703000000 cmp byte ptr [rbp + 0x347], 0
140a2a1dd 7426 je 0x140a2a205
140a2a1df 488b8528030000 mov rax, qword ptr [rbp + 0x328]
140a2a1e6 4885c0 test rax, rax
140a2a1e9 741a je 0x140a2a205
140a2a1eb 488b8d30030000 mov rcx, qword ptr [rbp + 0x330]
140a2a1f2 48c1e002 shl rax, 2
140a2a1f6 488d1480 lea rdx, [rax + rax*4]
140a2a1fa 41b804000000 mov r8d, 4
140a2a200 e81b9abeff call 0x140613c20 ; 
140a2a205 90 nop 
140a2a206 4883c440 add rsp, 0x40
140a2a20a 5b pop rbx
140a2a20b 5f pop rdi
140a2a20c 5e pop rsi
140a2a20d 415e pop r14
140a2a20f 5d pop rbp
140a2a210 c3 ret 

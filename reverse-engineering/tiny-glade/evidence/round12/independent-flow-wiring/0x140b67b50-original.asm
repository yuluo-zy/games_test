140b67b50: push rsi
140b67b51: push rdi
140b67b52: push rbx
140b67b53: sub rsp, 0xc0
140b67b5a: mov rsi, rcx
140b67b5d: add rcx, 0x38
140b67b61: movabs rdx, 0x8e1bd251962ba784
140b67b6b: movabs r8, 0x6a782374c0e6529b
140b67b75: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
140b67b7a: test rax, rax
140b67b7d: je 0x140b67c1e
140b67b83: cmp qword ptr [rsi + 0x218], rdx
140b67b8a: jbe 0x140b67c1e
140b67b90: mov rax, qword ptr [rsi + 0x210]
140b67b97: mov rcx, qword ptr [rax + rdx*8]
140b67b9b: test rcx, rcx
140b67b9e: je 0x140b67c1e
140b67ba0: mov rax, qword ptr [rsi + 0x1e0]
140b67ba7: not rcx
140b67baa: lea rcx, [rcx + rcx*2]
140b67bae: shl rcx, 5
140b67bb2: cmp qword ptr [rax + rcx + 0x20], 0
140b67bb8: je 0x140b67c1e
140b67bba: add rax, rcx
140b67bbd: mov rcx, qword ptr [rax + 0x10]
140b67bc1: mov edx, dword ptr [rsi + 0x5b0]
140b67bc7: mov dword ptr [rax + 0x5c], edx
140b67bca: call 0x1416a1c20 ; bevy_ecs::schedule::schedule::Schedules::entry<bevy_app::main_schedule::Startup>
140b67bcf: mov rdi, rax
140b67bd2: lea rbx, [rsp + 0x50]
140b67bd7: mov rcx, rbx
140b67bda: call 0x14099b8b0 ; bevy_ecs::schedule::config::impl$16::into_configs<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>)> >,void (*)(bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)> >,void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>)> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>)> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> >
140b67bdf: lea rcx, [rsp + 0x30]
140b67be4: mov rdx, rdi
140b67be7: mov r8, rbx
140b67bea: xor r9d, r9d
140b67bed: call 0x14169ed30 ; bevy_ecs::schedule::schedule::ScheduleGraph::process_configs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> >
140b67bf2: mov rdx, qword ptr [rsp + 0x30]
140b67bf7: test rdx, rdx
140b67bfa: je 0x140b67c10
140b67bfc: mov rcx, qword ptr [rsp + 0x38]
140b67c01: shl rdx, 4
140b67c05: mov r8d, 8
140b67c0b: call 0x140613c20 ; 
140b67c10: mov rax, rsi
140b67c13: add rsp, 0xc0
140b67c1a: pop rbx
140b67c1b: pop rdi
140b67c1c: pop rsi
140b67c1d: ret 
140b67c1e: lea rax, [rip + 0x1f8b09b]
140b67c25: mov qword ptr [rsp + 0x30], rax
140b67c2a: mov qword ptr [rsp + 0x38], 0x27
140b67c33: lea rax, [rsp + 0x30]
140b67c38: mov qword ptr [rsp + 0x20], rax
140b67c3d: lea rax, [rip - 0xa5ddb4]
140b67c44: mov qword ptr [rsp + 0x28], rax
140b67c49: lea rax, [rip + 0x1f8b1a0]
140b67c50: mov qword ptr [rsp + 0x50], rax
140b67c55: mov qword ptr [rsp + 0x58], 2
140b67c5e: mov qword ptr [rsp + 0x70], 0
140b67c67: lea rax, [rsp + 0x20]
140b67c6c: mov qword ptr [rsp + 0x60], rax
140b67c71: mov qword ptr [rsp + 0x68], 1
140b67c7a: lea rdx, [rip + 0x1f8b027]
140b67c81: lea rcx, [rsp + 0x50]
140b67c86: call 0x1428d9390 ; core::panicking::panic_fmt
140b67c8b: int3 

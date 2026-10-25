140a71200: push rbp
140a71201: push rsi
140a71202: push rdi
140a71203: sub rsp, 0x1d0
140a7120a: lea rbp, [rsp + 0x80]
140a71212: mov qword ptr [rbp + 0x148], 0xfffffffffffffffe
140a7121d: mov rsi, rcx
140a71220: xorps xmm0, xmm0
140a71223: movups xmmword ptr [rbp - 0x40], xmm0
140a71227: movups xmmword ptr [rbp - 0x28], xmm0
140a7122b: movups xmmword ptr [rbp - 0x10], xmm0
140a7122f: movups xmmword ptr [rbp + 8], xmm0
140a71233: movups xmmword ptr [rbp + 0x20], xmm0
140a71237: mov dword ptr [rbp + 0x30], 0
140a7123e: movups xmmword ptr [rbp + 0x58], xmm0
140a71242: movups xmmword ptr [rbp + 0x70], xmm0
140a71246: movups xmmword ptr [rbp + 0x88], xmm0
140a7124d: movups xmmword ptr [rbp + 0xa0], xmm0
140a71254: movups xmmword ptr [rbp + 0xb8], xmm0
140a7125b: mov dword ptr [rbp + 0xc8], 0
140a71265: mov qword ptr [rbp - 0x60], 0
140a7126d: mov qword ptr [rbp - 0x58], 8
140a71275: mov qword ptr [rbp - 0x50], 0
140a7127d: mov qword ptr [rbp - 0x48], 0x10
140a71285: mov qword ptr [rbp - 0x30], 0x10
140a7128d: mov qword ptr [rbp - 0x18], 0x10
140a71295: mov qword ptr [rbp], 0x10
140a7129d: mov qword ptr [rbp + 0x18], 0x10
140a712a5: movabs rax, 0x8000000000000000
140a712af: mov qword ptr [rbp + 0x38], rax
140a712b3: lea rcx, [rip + 0x2064ece]
140a712ba: mov qword ptr [rbp + 0x40], rcx
140a712be: mov qword ptr [rbp + 0x48], 0x41
140a712c6: mov qword ptr [rbp + 0x50], 0x10
140a712ce: mov qword ptr [rbp + 0x68], 0x10
140a712d6: mov qword ptr [rbp + 0x80], 0x10
140a712e1: mov qword ptr [rbp + 0x98], 0x10
140a712ec: mov qword ptr [rbp + 0xb0], 0x10
140a712f7: mov dword ptr [rbp + 0xd0], 0
140a71301: mov word ptr [rbp + 0xd4], 1
140a7130a: mov qword ptr [rbp + 0x100], rax
140a71311: mov dword ptr [rbp + 0x140], 0
140a7131b: movzx eax, byte ptr [rip + 0x2bc38e7]
140a71322: mov ecx, 0x1a8
140a71327: mov edx, 8
140a7132c: call 0x140613c10
140a71331: test rax, rax
140a71334: je 0x140a7136b
140a71336: mov rdi, rax
140a71339: lea rdx, [rbp - 0x60]
140a7133d: mov r8d, 0x1a8
140a71343: mov rcx, rax
140a71346: call 0x142923c00
140a7134b: lea r8, [rip + 0x2064a0e]
140a71352: mov rcx, rsi
140a71355: mov rdx, rdi
140a71358: call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
140a7135d: mov rax, rsi
140a71360: add rsp, 0x1d0
140a71367: pop rdi
140a71368: pop rsi
140a71369: pop rbp
140a7136a: ret 
140a7136b: mov ecx, 8
140a71370: mov edx, 0x1a8
140a71375: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
140a7137a: ud2 
140a7137c: nop dword ptr [rax]
140a71380: mov qword ptr [rsp + 0x10], rdx
140a71385: push rbp
140a71386: push rsi
140a71387: push rdi
140a71388: sub rsp, 0x20
140a7138c: lea rbp, [rdx + 0x80]
140a71393: lea rcx, [rbp - 0x60]
140a71397: call 0x1405fb160 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)> > >
140a7139c: nop 
140a7139d: add rsp, 0x20
140a713a1: pop rdi
140a713a2: pop rsi
140a713a3: pop rbp
140a713a4: ret 

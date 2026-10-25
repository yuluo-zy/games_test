1417650e0: push rbp
1417650e1: push rsi
1417650e2: push rdi
1417650e3: sub rsp, 0x1d0
1417650ea: lea rbp, [rsp + 0x80]
1417650f2: mov qword ptr [rbp + 0x148], 0xfffffffffffffffe
1417650fd: mov rsi, rcx
141765100: xorps xmm0, xmm0
141765103: movups xmmword ptr [rbp - 0x40], xmm0
141765107: movups xmmword ptr [rbp - 0x28], xmm0
14176510b: movups xmmword ptr [rbp - 0x10], xmm0
14176510f: movups xmmword ptr [rbp + 8], xmm0
141765113: movups xmmword ptr [rbp + 0x20], xmm0
141765117: mov dword ptr [rbp + 0x30], 0
14176511e: movups xmmword ptr [rbp + 0x58], xmm0
141765122: movups xmmword ptr [rbp + 0x70], xmm0
141765126: movups xmmword ptr [rbp + 0x88], xmm0
14176512d: movups xmmword ptr [rbp + 0xa0], xmm0
141765134: movups xmmword ptr [rbp + 0xb8], xmm0
14176513b: mov dword ptr [rbp + 0xc8], 0
141765145: mov qword ptr [rbp - 0x60], 0
14176514d: mov qword ptr [rbp - 0x58], 8
141765155: mov qword ptr [rbp - 0x50], 0
14176515d: mov qword ptr [rbp - 0x48], 0x10
141765165: mov qword ptr [rbp - 0x30], 0x10
14176516d: mov qword ptr [rbp - 0x18], 0x10
141765175: mov qword ptr [rbp], 0x10
14176517d: mov qword ptr [rbp + 0x18], 0x10
141765185: movabs rax, 0x8000000000000000
14176518f: mov qword ptr [rbp + 0x38], rax
141765193: lea rcx, [rip + 0x1571ea4]
14176519a: mov qword ptr [rbp + 0x40], rcx
14176519e: mov qword ptr [rbp + 0x48], 0x23
1417651a6: mov qword ptr [rbp + 0x50], 0x10
1417651ae: mov qword ptr [rbp + 0x68], 0x10
1417651b6: mov qword ptr [rbp + 0x80], 0x10
1417651c1: mov qword ptr [rbp + 0x98], 0x10
1417651cc: mov qword ptr [rbp + 0xb0], 0x10
1417651d7: mov dword ptr [rbp + 0xd0], 0
1417651e1: mov word ptr [rbp + 0xd4], 1
1417651ea: mov qword ptr [rbp + 0x100], rax
1417651f1: mov dword ptr [rbp + 0x140], 0
1417651fb: movzx eax, byte ptr [rip + 0x1ecfa07]
141765202: mov ecx, 0x1a8
141765207: mov edx, 8
14176520c: call 0x140613c10
141765211: test rax, rax
141765214: je 0x14176524b
141765216: mov rdi, rax
141765219: lea rdx, [rbp - 0x60]
14176521d: mov r8d, 0x1a8
141765223: mov rcx, rax
141765226: call 0x142923c00
14176522b: lea r8, [rip + 0x15719b6]
141765232: mov rcx, rsi
141765235: mov rdx, rdi
141765238: call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14176523d: mov rax, rsi
141765240: add rsp, 0x1d0
141765247: pop rdi
141765248: pop rsi
141765249: pop rbp
14176524a: ret 
14176524b: mov ecx, 8
141765250: mov edx, 0x1a8
141765255: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14176525a: ud2 
14176525c: nop dword ptr [rax]
141765260: mov qword ptr [rsp + 0x10], rdx
141765265: push rbp
141765266: push rsi
141765267: push rdi
141765268: sub rsp, 0x20
14176526c: lea rbp, [rdx + 0x80]
141765273: lea rcx, [rbp - 0x60]
141765277: call 0x1405fb160 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)> > >
14176527c: nop 
14176527d: add rsp, 0x20
141765281: pop rdi
141765282: pop rsi
141765283: pop rbp
141765284: ret 

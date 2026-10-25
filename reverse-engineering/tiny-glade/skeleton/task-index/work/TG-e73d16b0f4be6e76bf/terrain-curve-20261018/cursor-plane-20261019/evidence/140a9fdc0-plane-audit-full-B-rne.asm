0000000140a9fdc0: 55                               push rbp
0000000140a9fdc1: 4157                             push r15
0000000140a9fdc3: 4156                             push r14
0000000140a9fdc5: 4155                             push r13
0000000140a9fdc7: 4154                             push r12
0000000140a9fdc9: 56                               push rsi
0000000140a9fdca: 57                               push rdi
0000000140a9fdcb: 53                               push rbx
0000000140a9fdcc: 4881ec28010000                   sub rsp, 0x128
0000000140a9fdd3: 488dac2480000000                 lea rbp, [rsp + 0x80]
0000000140a9fddb: 440f298590000000                 movaps xmmword ptr [rbp + 0x90], xmm8
0000000140a9fde3: 0f29bd80000000                   movaps xmmword ptr [rbp + 0x80], xmm7
0000000140a9fdea: 0f297570                         movaps xmmword ptr [rbp + 0x70], xmm6
0000000140a9fdee: 48c74568feffffff                 mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
0000000140a9fdf6: 488b01                           mov rax, qword ptr [rcx]
0000000140a9fdf9: 83380a                           cmp dword ptr [rax], 0xa
0000000140a9fdfc: 0f856b090000                     jne 0x140aa076d
0000000140a9fe02: 4c894558                         mov qword ptr [rbp + 0x58], r8
0000000140a9fe06: 488b8548010000                   mov rax, qword ptr [rbp + 0x148]
0000000140a9fe0d: 488b18                           mov rbx, qword ptr [rax]
0000000140a9fe10: 4c8b6820                         mov r13, qword ptr [rax + 0x20]
0000000140a9fe14: 4c8b7030                         mov r14, qword ptr [rax + 0x30]
0000000140a9fe18: 448b603c                         mov r12d, dword ptr [rax + 0x3c]
0000000140a9fe1c: 488b4840                         mov rcx, qword ptr [rax + 0x40]
0000000140a9fe20: 48894d60                         mov qword ptr [rbp + 0x60], rcx
0000000140a9fe24: 488b4850                         mov rcx, qword ptr [rax + 0x50]
0000000140a9fe28: 48894d38                         mov qword ptr [rbp + 0x38], rcx
0000000140a9fe2c: 8b405c                           mov eax, dword ptr [rax + 0x5c]
0000000140a9fe2f: 894540                           mov dword ptr [rbp + 0x40], eax
0000000140a9fe32: 498b01                           mov rax, qword ptr [r9]
0000000140a9fe35: 488d4808                         lea rcx, [rax + 8]
0000000140a9fe39: 4c8d4020                         lea r8, [rax + 0x20]
0000000140a9fe3d: 4c8d4810                         lea r9, [rax + 0x10]
0000000140a9fe41: 4c8d5028                         lea r10, [rax + 0x28]
0000000140a9fe45: 80b89802000000                   cmp byte ptr [rax + 0x298], 0
0000000140a9fe4c: 490f45c8                         cmovne rcx, r8
0000000140a9fe50: 4d0f44d1                         cmove r10, r9
0000000140a9fe54: 498b02                           mov rax, qword ptr [r10]
0000000140a9fe57: 488b09                           mov rcx, qword ptr [rcx]
0000000140a9fe5a: 48894db0                         mov qword ptr [rbp - 0x50], rcx
0000000140a9fe5e: 488945b8                         mov qword ptr [rbp - 0x48], rax
0000000140a9fe62: 4c8b3a                           mov r15, qword ptr [rdx]
0000000140a9fe65: 0f57ff                           xorps xmm7, xmm7
0000000140a9fe68: 41f60701                         test byte ptr [r15], 1
0000000140a9fe6c: 0f57d2                           xorps xmm2, xmm2
0000000140a9fe6f: 0f57c9                           xorps xmm1, xmm1
0000000140a9fe72: 740c                             je 0x140a9fe80
0000000140a9fe74: f3410f104f04                     movss xmm1, dword ptr [r15 + 4]
0000000140a9fe7a: f3410f10570c                     movss xmm2, dword ptr [r15 + 0xc]
0000000140a9fe80: 488bb518010000                   mov rsi, qword ptr [rbp + 0x118]
0000000140a9fe87: c744242000000243                 mov dword ptr [rsp + 0x20], 0x43020000
0000000140a9fe8f: 488d4db0                         lea rcx, [rbp - 0x50]
0000000140a9fe93: f30f1035bdb7fd01                 movss xmm6, dword ptr [rip + 0x1fdb7bd]
0000000140a9fe9b: 0f28de                           movaps xmm3, xmm6
0000000140a9fe9e: e84d4ff7ff                       call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> > | utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
0000000140a9fea3: 0f2ec7                           ucomiss xmm0, xmm7
0000000140a9fea6: 761a                             jbe 0x140a9fec2
0000000140a9fea8: 41f60701                         test byte ptr [r15], 1
0000000140a9feac: 7419                             je 0x140a9fec7
0000000140a9feae: f30f1005ba59e801                 movss xmm0, dword ptr [rip + 0x1e859ba]
0000000140a9feb6: 410f2e4708                       ucomiss xmm0, dword ptr [r15 + 8]
0000000140a9febb: 488b36                           mov rsi, qword ptr [rsi]
0000000140a9febe: 730a                             jae 0x140a9feca
0000000140a9fec0: eb4a                             jmp 0x140a9ff0c
0000000140a9fec2: 488b36                           mov rsi, qword ptr [rsi]
0000000140a9fec5: eb45                             jmp 0x140a9ff0c
0000000140a9fec7: 488b36                           mov rsi, qword ptr [rsi]
0000000140a9feca: c745c000000000                   mov dword ptr [rbp - 0x40], 0
0000000140a9fed1: c745d804000000                   mov dword ptr [rbp - 0x28], 4
0000000140a9fed8: c745f000000000                   mov dword ptr [rbp - 0x10], 0
0000000140a9fedf: c7450800000000                   mov dword ptr [rbp + 8], 0
0000000140a9fee6: 488d45c0                         lea rax, [rbp - 0x40]
0000000140a9feea: 4889442428                       mov qword ptr [rsp + 0x28], rax
0000000140a9feef: c644242001                       mov byte ptr [rsp + 0x20], 1
0000000140a9fef4: 488d15e4c30302                   lea rdx, [rip + 0x203c3e4]
0000000140a9fefb: 41b811000000                     mov r8d, 0x11
0000000140a9ff01: 4889f1                           mov rcx, rsi
0000000140a9ff04: 4531c9                           xor r9d, r9d
0000000140a9ff07: e844e0e8ff                       call 0x14092df50 ; country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound
0000000140a9ff0c: c745c000000000                   mov dword ptr [rbp - 0x40], 0
0000000140a9ff13: c745d800000000                   mov dword ptr [rbp - 0x28], 0
0000000140a9ff1a: c745f000000000                   mov dword ptr [rbp - 0x10], 0
0000000140a9ff21: c7450800000000                   mov dword ptr [rbp + 8], 0
0000000140a9ff28: 488d45c0                         lea rax, [rbp - 0x40]
0000000140a9ff2c: 4889442428                       mov qword ptr [rsp + 0x28], rax
0000000140a9ff31: c644242002                       mov byte ptr [rsp + 0x20], 2
0000000140a9ff36: 488d15b3c30302                   lea rdx, [rip + 0x203c3b3]
0000000140a9ff3d: 41b80c000000                     mov r8d, 0xc
0000000140a9ff43: 4889f1                           mov rcx, rsi
0000000140a9ff46: 41b103                           mov r9b, 3
0000000140a9ff49: e802e0e8ff                       call 0x14092df50 ; country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound
0000000140a9ff4e: 4889c6                           mov rsi, rax
0000000140a9ff51: 4889d7                           mov rdi, rdx
0000000140a9ff54: 41f60701                         test byte ptr [r15], 1
0000000140a9ff58: 741b                             je 0x140a9ff75
0000000140a9ff5a: 498b4704                         mov rax, qword ptr [r15 + 4]
0000000140a9ff5e: 48894548                         mov qword ptr [rbp + 0x48], rax
0000000140a9ff62: 418b470c                         mov eax, dword ptr [r15 + 0xc]
0000000140a9ff66: 894550                           mov dword ptr [rbp + 0x50], eax
0000000140a9ff69: f30f104d48                       movss xmm1, dword ptr [rbp + 0x48]
0000000140a9ff6e: f30f105550                       movss xmm2, dword ptr [rbp + 0x50]
0000000140a9ff73: eb15                             jmp 0x140a9ff8a
0000000140a9ff75: c7455000000000                   mov dword ptr [rbp + 0x50], 0
0000000140a9ff7c: 48c7454800000000                 mov qword ptr [rbp + 0x48], 0
0000000140a9ff84: 0f57d2                           xorps xmm2, xmm2
0000000140a9ff87: 0f57c9                           xorps xmm1, xmm1
0000000140a9ff8a: 488d4dc4                         lea rcx, [rbp - 0x3c]
0000000140a9ff8e: f30f101d226df701                 movss xmm3, dword ptr [rip + 0x1f76d22]
0000000140a9ff96: e8a5751f00                       call 0x140c97540 ; utils::geometry::circle::Circle2d::new
0000000140a9ff9b: c745c000000000                   mov dword ptr [rbp - 0x40], 0
0000000140a9ffa2: 488d55c0                         lea rdx, [rbp - 0x40]
0000000140a9ffa6: 4889d9                           mov rcx, rbx
0000000140a9ffa9: e8c2290300                       call 0x140ad2970 ; country_core::resources::glade_border::GladeBorder::is_shape_inside
0000000140a9ffae: 89c3                             mov ebx, eax
0000000140a9ffb0: 84c0                             test al, al
0000000140a9ffb2: 0f8584000000                     jne 0x140aa003c
0000000140a9ffb8: 458926                           mov dword ptr [r14], r12d
0000000140a9ffbb: 4d8b7530                         mov r14, qword ptr [r13 + 0x30]
0000000140a9ffbf: 4d8b6540                         mov r12, qword ptr [r13 + 0x40]
0000000140a9ffc3: 4d3b7520                         cmp r14, qword ptr [r13 + 0x20]
0000000140a9ffc7: 7510                             jne 0x140a9ffd9
0000000140a9ffc9: 498d4d20                         lea rcx, [r13 + 0x20]
0000000140a9ffcd: 488d1574a11902                   lea rdx, [rip + 0x219a174]
0000000140a9ffd4: e89728acff                       call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<enum2$<country_core::resources::window::OsWindowCmd> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemStairsCanDetachFromWall>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<ash::vk::definitions::SwapchainKHR,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::history::FinishInputRecordingCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_clutter::autoclutter::ClutterRegionId,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_water::record_water_history::RecordWaterEditToHistoryCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
0000000140a9ffd9: 498b4528                         mov rax, qword ptr [r13 + 0x28]
0000000140a9ffdd: 4e8924f0                         mov qword ptr [rax + r14*8], r12
0000000140a9ffe1: 49ffc6                           inc r14
0000000140a9ffe4: 4d897530                         mov qword ptr [r13 + 0x30], r14
0000000140a9ffe8: 49ff4540                         inc qword ptr [r13 + 0x40]
0000000140a9ffec: 807e1802                         cmp byte ptr [rsi + 0x18], 2
0000000140a9fff0: 724a                             jb 0x140aa003c
0000000140a9fff2: 48397e38                         cmp qword ptr [rsi + 0x38], rdi
0000000140a9fff6: 7544                             jne 0x140aa003c
0000000140a9fff8: 807e1902                         cmp byte ptr [rsi + 0x19], 2
0000000140a9fffc: 743e                             je 0x140aa003c
0000000140a9fffe: 488b4538                         mov rax, qword ptr [rbp + 0x38]
0000000140aa0002: 8b4d40                           mov ecx, dword ptr [rbp + 0x40]
0000000140aa0005: 8908                             mov dword ptr [rax], ecx
0000000140aa0007: 4c8b6d60                         mov r13, qword ptr [rbp + 0x60]
0000000140aa000b: 4d8b7530                         mov r14, qword ptr [r13 + 0x30]
0000000140aa000f: 4d8b6540                         mov r12, qword ptr [r13 + 0x40]
0000000140aa0013: 4d3b7520                         cmp r14, qword ptr [r13 + 0x20]
0000000140aa0017: 7510                             jne 0x140aa0029
0000000140aa0019: 498d4d20                         lea rcx, [r13 + 0x20]
0000000140aa001d: 488d1524a11902                   lea rdx, [rip + 0x219a124]
0000000140aa0024: e84728acff                       call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<enum2$<country_core::resources::window::OsWindowCmd> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemStairsCanDetachFromWall>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<ash::vk::definitions::SwapchainKHR,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::history::FinishInputRecordingCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_clutter::autoclutter::ClutterRegionId,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_water::record_water_history::RecordWaterEditToHistoryCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
0000000140aa0029: 498b4528                         mov rax, qword ptr [r13 + 0x28]
0000000140aa002d: 4e8924f0                         mov qword ptr [rax + r14*8], r12
0000000140aa0031: 49ffc6                           inc r14
0000000140aa0034: 4d897530                         mov qword ptr [r13 + 0x30], r14
0000000140aa0038: 49ff4540                         inc qword ptr [r13 + 0x40]
0000000140aa003c: b002                             mov al, 2
0000000140aa003e: 807e1802                         cmp byte ptr [rsi + 0x18], 2
0000000140aa0042: 720a                             jb 0x140aa004e
0000000140aa0044: 48397e38                         cmp qword ptr [rsi + 0x38], rdi
0000000140aa0048: 7504                             jne 0x140aa004e
0000000140aa004a: 0fb64619                         movzx eax, byte ptr [rsi + 0x19]
0000000140aa004e: 04fe                             add al, 0xfe
0000000140aa0050: 0fb6c8                           movzx ecx, al
0000000140aa0053: 80f903                           cmp cl, 3
0000000140aa0056: b803000000                       mov eax, 3
0000000140aa005b: 0f42c1                           cmovb eax, ecx
0000000140aa005e: 488b8d50010000                   mov rcx, qword ptr [rbp + 0x150]
0000000140aa0065: 0fb6d0                           movzx edx, al
0000000140aa0068: 448d42ff                         lea r8d, [rdx - 1]
0000000140aa006c: 4183f802                         cmp r8d, 2
0000000140aa0070: 7314                             jae 0x140aa0086
0000000140aa0072: 0fb601                           movzx eax, byte ptr [rcx]
0000000140aa0075: fec8                             dec al
0000000140aa0077: 3c02                             cmp al, 2
0000000140aa0079: 0fb6d3                           movzx edx, bl
0000000140aa007c: b802000000                       mov eax, 2
0000000140aa0081: 0f43c2                           cmovae eax, edx
0000000140aa0084: eb13                             jmp 0x140aa0099
0000000140aa0086: 85d2                             test edx, edx
0000000140aa0088: 740f                             je 0x140aa0099
0000000140aa008a: 0fb601                           movzx eax, byte ptr [rcx]
0000000140aa008d: fec8                             dec al
0000000140aa008f: 31d2                             xor edx, edx
0000000140aa0091: 3c02                             cmp al, 2
0000000140aa0093: 0f92c2                           setb dl
0000000140aa0096: 8d0452                           lea eax, [rdx + rdx*2]
0000000140aa0099: 4c8bb558010000                   mov r14, qword ptr [rbp + 0x158]
0000000140aa00a0: 8801                             mov byte ptr [rcx], al
0000000140aa00a2: 807e1802                         cmp byte ptr [rsi + 0x18], 2
0000000140aa00a6: 7220                             jb 0x140aa00c8
0000000140aa00a8: 48397e38                         cmp qword ptr [rsi + 0x38], rdi
0000000140aa00ac: 751a                             jne 0x140aa00c8
0000000140aa00ae: 807e1903                         cmp byte ptr [rsi + 0x19], 3
0000000140aa00b2: 7514                             jne 0x140aa00c8
0000000140aa00b4: 0f57c0                           xorps xmm0, xmm0
0000000140aa00b7: 41f60701                         test byte ptr [r15], 1
0000000140aa00bb: 7406                             je 0x140aa00c3
0000000140aa00bd: f3410f104708                     movss xmm0, dword ptr [r15 + 8]
0000000140aa00c3: f3410f1106                       movss dword ptr [r14], xmm0
0000000140aa00c8: 488bbd30010000                   mov rdi, qword ptr [rbp + 0x130]
0000000140aa00cf: 488b8d20010000                   mov rcx, qword ptr [rbp + 0x120]
0000000140aa00d6: 0fb6c0                           movzx eax, al
0000000140aa00d9: 488d15b8c30302                   lea rdx, [rip + 0x203c3b8]
0000000140aa00e0: 48630482                         movsxd rax, dword ptr [rdx + rax*4]
0000000140aa00e4: 4801d0                           add rax, rdx
0000000140aa00e7: ffe0                             jmp rax
0000000140aa00e9: 448b691c                         mov r13d, dword ptr [rcx + 0x1c]
0000000140aa00ed: 488b7110                         mov rsi, qword ptr [rcx + 0x10]
0000000140aa00f1: 44892e                           mov dword ptr [rsi], r13d
0000000140aa00f4: 488b39                           mov rdi, qword ptr [rcx]
0000000140aa00f7: 488d5548                         lea rdx, [rbp + 0x48]
0000000140aa00fb: 4889f9                           mov rcx, rdi
0000000140aa00fe: e80d49e8ff                       call 0x140924a10 ; country_core::resources::terrain_editing::TerrainEditingData::new_stroke
0000000140aa0103: 83f801                           cmp eax, 1
0000000140aa0106: 0f854e020000                     jne 0x140aa035a
0000000140aa010c: 895560                           mov dword ptr [rbp + 0x60], edx
0000000140aa010f: f3410f103e                       movss xmm7, dword ptr [r14]
0000000140aa0114: 44892e                           mov dword ptr [rsi], r13d
0000000140aa0117: c70701000000                     mov dword ptr [rdi], 1
0000000140aa011d: f30f117f04                       movss dword ptr [rdi + 4], xmm7
0000000140aa0122: 488b4558                         mov rax, qword ptr [rbp + 0x58]
0000000140aa0126: 488b18                           mov rbx, qword ptr [rax]
0000000140aa0129: 488d15d8e21802                   lea rdx, [rip + 0x218e2d8]
0000000140aa0130: 4889d9                           mov rcx, rbx
0000000140aa0133: e8e895ecff                       call 0x140969720 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
0000000140aa0138: f3440f100523cff201               movss xmm8, dword ptr [rip + 0x1f2cf23]
0000000140aa0141: 41b401                           mov r12b, 1
0000000140aa0144: 4885c0                           test rax, rax
0000000140aa0147: 7469                             je 0x140aa01b2
0000000140aa0149: 4989c6                           mov r14, rax
0000000140aa014c: 488d0508deff01                   lea rax, [rip + 0x1ffde08]
0000000140aa0153: 48894528                         mov qword ptr [rbp + 0x28], rax
0000000140aa0157: 48c745306c000000                 mov qword ptr [rbp + 0x30], 0x6c
0000000140aa015f: 49837e186c                       cmp qword ptr [r14 + 0x18], 0x6c
0000000140aa0164: 0f8554060000                     jne 0x140aa07be
0000000140aa016a: 498b5610                         mov rdx, qword ptr [r14 + 0x10]
0000000140aa016e: 488d0de6ddff01                   lea rcx, [rip + 0x1ffdde6]
0000000140aa0175: 41b86c000000                     mov r8d, 0x6c
0000000140aa017b: e8703ae801                       call 0x142923bf0 ; 
0000000140aa0180: 85c0                             test eax, eax
0000000140aa0182: 0f8536060000                     jne 0x140aa07be
0000000140aa0188: 498b0e                           mov rcx, qword ptr [r14]
0000000140aa018b: 498b4608                         mov rax, qword ptr [r14 + 8]
0000000140aa018f: ff5018                           call qword ptr [rax + 0x18]
0000000140aa0192: 4885c0                           test rax, rax
0000000140aa0195: 741b                             je 0x140aa01b2
0000000140aa0197: 4883780800                       cmp qword ptr [rax + 8], 0
0000000140aa019c: 0f84ad060000                     je 0x140aa084f
0000000140aa01a2: 488b00                           mov rax, qword ptr [rax]
0000000140aa01a5: 837810ff                         cmp dword ptr [rax + 0x10], -1
0000000140aa01a9: 7407                             je 0x140aa01b2
0000000140aa01ab: 450f57c0                         xorps xmm8, xmm8
0000000140aa01af: 4531e4                           xor r12d, r12d
0000000140aa01b2: 488d15efe01802                   lea rdx, [rip + 0x218e0ef]
0000000140aa01b9: 4889d9                           mov rcx, rbx
0000000140aa01bc: e85f95ecff                       call 0x140969720 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
0000000140aa01c1: 4885c0                           test rax, rax
0000000140aa01c4: 0f8430060000                     je 0x140aa07fa
0000000140aa01ca: 4889c3                           mov rbx, rax
0000000140aa01cd: 488d05edd2ff01                   lea rax, [rip + 0x1ffd2ed]
0000000140aa01d4: 48894528                         mov qword ptr [rbp + 0x28], rax
0000000140aa01d8: 48c745303f000000                 mov qword ptr [rbp + 0x30], 0x3f
0000000140aa01e0: 48837b183f                       cmp qword ptr [rbx + 0x18], 0x3f
0000000140aa01e5: 0f85a9050000                     jne 0x140aa0794
0000000140aa01eb: 488b5310                         mov rdx, qword ptr [rbx + 0x10]
0000000140aa01ef: 488d0dcbd2ff01                   lea rcx, [rip + 0x1ffd2cb]
0000000140aa01f6: 41b83f000000                     mov r8d, 0x3f
0000000140aa01fc: e8ef39e801                       call 0x142923bf0 ; 
0000000140aa0201: 85c0                             test eax, eax
0000000140aa0203: 0f858b050000                     jne 0x140aa0794
0000000140aa0209: 488b0b                           mov rcx, qword ptr [rbx]
0000000140aa020c: 488b4308                         mov rax, qword ptr [rbx + 8]
0000000140aa0210: ff5018                           call qword ptr [rax + 0x18]
0000000140aa0213: 4885c0                           test rax, rax
0000000140aa0216: 0f84de050000                     je 0x140aa07fa
0000000140aa021c: 4584e4                           test r12b, r12b
0000000140aa021f: 0f844b010000                     je 0x140aa0370
0000000140aa0225: 4c8bb530010000                   mov r14, qword ptr [rbp + 0x130]
0000000140aa022c: 448b6560                         mov r12d, dword ptr [rbp + 0x60]
0000000140aa0230: e91d020000                       jmp 0x140aa0452
0000000140aa0235: 8b411c                           mov eax, dword ptr [rcx + 0x1c]
0000000140aa0238: 488b5110                         mov rdx, qword ptr [rcx + 0x10]
0000000140aa023c: 8902                             mov dword ptr [rdx], eax
0000000140aa023e: 488b31                           mov rsi, qword ptr [rcx]
0000000140aa0241: 4889f1                           mov rcx, rsi
0000000140aa0244: e8b746e8ff                       call 0x140924900 ; country_core::resources::terrain_editing::TerrainEditingData::get_last_mut_stroke
0000000140aa0249: 4885c0                           test rax, rax
0000000140aa024c: 0f95c1                           setne cl
0000000140aa024f: 84cb                             test bl, cl
0000000140aa0251: 0f8416050000                     je 0x140aa076d
0000000140aa0257: 488d5548                         lea rdx, [rbp + 0x48]
0000000140aa025b: f30f10157556e801                 movss xmm2, dword ptr [rip + 0x1e85675]
0000000140aa0263: 4889c1                           mov rcx, rax
0000000140aa0266: e81576ffff                       call 0x140a97880 ; country_core::resources::terrain_stroke::TerrainStroke::insert_or_morph_last_point
0000000140aa026b: 488b7638                         mov rsi, qword ptr [rsi + 0x38]
0000000140aa026f: 4885f6                           test rsi, rsi
0000000140aa0272: 0f84bf050000                     je 0x140aa0837
0000000140aa0278: ffce                             dec esi
0000000140aa027a: 8b471c                           mov eax, dword ptr [rdi + 0x1c]
0000000140aa027d: 488b4f10                         mov rcx, qword ptr [rdi + 0x10]
0000000140aa0281: 8901                             mov dword ptr [rcx], eax
0000000140aa0283: 488b3f                           mov rdi, qword ptr [rdi]
0000000140aa0286: 488b5f30                         mov rbx, qword ptr [rdi + 0x30]
0000000140aa028a: 4c8b7740                         mov r14, qword ptr [rdi + 0x40]
0000000140aa028e: 483b5f20                         cmp rbx, qword ptr [rdi + 0x20]
0000000140aa0292: 7510                             jne 0x140aa02a4
0000000140aa0294: 488d4f20                         lea rcx, [rdi + 0x20]
0000000140aa0298: 488d15a99e1902                   lea rdx, [rip + 0x2199ea9]
0000000140aa029f: e88c23acff                       call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<toolbar::ui_interaction::UiJustBecameInFocusEvent>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::walls::WallId,ref$<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> > >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemWindow>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<asset::prefab::PrefabName,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<alloc::ffi::c_str::CString,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemRoofShape>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<ref$<std::ffi::os_str::OsStr>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::systems::prefab::DespawnPrefabCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_onboarding::tutorial_completion::TutorialHintComplete>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_color::set_decorator_color_id::SetStairRailingCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<u64,ref$<system_clutter::autoclutter::AutoClutterObject> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<enum2$<core::option::Option<bevy_ecs::entity::Entity> >,bool>,alloc::alloc::Global>
0000000140aa02a4: 488b4728                         mov rax, qword ptr [rdi + 0x28]
0000000140aa02a8: 4889d9                           mov rcx, rbx
0000000140aa02ab: 48c1e104                         shl rcx, 4
0000000140aa02af: 4c893408                         mov qword ptr [rax + rcx], r14
0000000140aa02b3: 89740808                         mov dword ptr [rax + rcx + 8], esi
0000000140aa02b7: 48ffc3                           inc rbx
0000000140aa02ba: 48895f30                         mov qword ptr [rdi + 0x30], rbx
0000000140aa02be: 48ff4740                         inc qword ptr [rdi + 0x40]
0000000140aa02c2: e9a6040000                       jmp 0x140aa076d
0000000140aa02c7: 448b611c                         mov r12d, dword ptr [rcx + 0x1c]
0000000140aa02cb: 4c8b6910                         mov r13, qword ptr [rcx + 0x10]
0000000140aa02cf: 45896500                         mov dword ptr [r13], r12d
0000000140aa02d3: 488b31                           mov rsi, qword ptr [rcx]
0000000140aa02d6: 4889f1                           mov rcx, rsi
0000000140aa02d9: e82246e8ff                       call 0x140924900 ; country_core::resources::terrain_editing::TerrainEditingData::get_last_mut_stroke
0000000140aa02de: 4885c0                           test rax, rax
0000000140aa02e1: 0f8400040000                     je 0x140aa06e7
0000000140aa02e7: 4889c1                           mov rcx, rax
0000000140aa02ea: 488b7810                         mov rdi, qword ptr [rax + 0x10]
0000000140aa02ee: 4531ff                           xor r15d, r15d
0000000140aa02f1: ba0c000000                       mov edx, 0xc
0000000140aa02f6: 4889f8                           mov rax, rdi
0000000140aa02f9: 48f7e2                           mul rdx
0000000140aa02fc: 4889c3                           mov rbx, rax
0000000140aa02ff: 0f80e3040000                     jo 0x140aa07e8
0000000140aa0305: 48b8fcffffffffffff7f             movabs rax, 0x7ffffffffffffffc
0000000140aa030f: 4839c3                           cmp rbx, rax
0000000140aa0312: 0f87d0040000                     ja 0x140aa07e8
0000000140aa0318: f30f107120                       movss xmm6, dword ptr [rcx + 0x20]
0000000140aa031d: 4c8b7108                         mov r14, qword ptr [rcx + 8]
0000000140aa0321: 4885db                           test rbx, rbx
0000000140aa0324: 44896560                         mov dword ptr [rbp + 0x60], r12d
0000000140aa0328: 0f8488020000                     je 0x140aa05b6
0000000140aa032e: 0fb605d448b902                   movzx eax, byte ptr [rip + 0x2b948d4]
0000000140aa0335: 41bf04000000                     mov r15d, 4
0000000140aa033b: ba04000000                       mov edx, 4
0000000140aa0340: 4889d9                           mov rcx, rbx
0000000140aa0343: e8c838b7ff                       call 0x140613c10 ; 
0000000140aa0348: 4885c0                           test rax, rax
0000000140aa034b: 0f8497040000                     je 0x140aa07e8
0000000140aa0351: 48897d58                         mov qword ptr [rbp + 0x58], rdi
0000000140aa0355: e969020000                       jmp 0x140aa05c3
0000000140aa035a: 488d0dcfbf0302                   lea rcx, [rip + 0x203bfcf]
0000000140aa0361: ba25000000                       mov edx, 0x25
0000000140aa0366: e8a50fe2ff                       call 0x1408c1310 ; country_core::systems::backend::flush_screen_log::log_warning_to_screen
0000000140aa036b: e9fd030000                       jmp 0x140aa076d
0000000140aa0370: 488b5008                         mov rdx, qword ptr [rax + 8]
0000000140aa0374: 0f57c0                           xorps xmm0, xmm0
0000000140aa0377: 41f60701                         test byte ptr [r15], 1
0000000140aa037b: 0f57c9                           xorps xmm1, xmm1
0000000140aa037e: 0f57d2                           xorps xmm2, xmm2
0000000140aa0381: 4c8bb530010000                   mov r14, qword ptr [rbp + 0x130]
0000000140aa0388: 448b6560                         mov r12d, dword ptr [rbp + 0x60]
0000000140aa038c: 740c                             je 0x140aa039a
0000000140aa038e: f3410f105704                     movss xmm2, dword ptr [r15 + 4]
0000000140aa0394: f3410f104f0c                     movss xmm1, dword ptr [r15 + 0xc]
0000000140aa039a: f30f5ed6                         divss xmm2, xmm6
0000000140aa039e: f30f5ece                         divss xmm1, xmm6
0000000140aa03a2: f30f101dc654e801                 movss xmm3, dword ptr [rip + 0x1e854c6]
0000000140aa03aa: f30f58d3                         addss xmm2, xmm3
0000000140aa03ae: f30f58cb                         addss xmm1, xmm3
0000000140aa03b2: f30f5fd0                         maxss xmm2, xmm0
0000000140aa03b6: f30f5fc8                         maxss xmm1, xmm0
0000000140aa03ba: f30f101d4255e801                 movss xmm3, dword ptr [rip + 0x1e85542]
0000000140aa03c2: f30f5dd3                         minss xmm2, xmm3
0000000140aa03c6: f30f5dcb                         minss xmm1, xmm3
0000000140aa03ca: f30f101dde38f001                 movss xmm3, dword ptr [rip + 0x1f038de]
0000000140aa03d2: f30f59d3                         mulss xmm2, xmm3
0000000140aa03d6: f34c0f2cc2                       cvttss2si r8, xmm2
0000000140aa03db: 4531c9                           xor r9d, r9d
0000000140aa03de: 0f2ed0                           ucomiss xmm2, xmm0
0000000140aa03e1: 450f42c1                         cmovb r8d, r9d
0000000140aa03e5: f30f59cb                         mulss xmm1, xmm3
0000000140aa03e9: f30f101ddf03f001                 movss xmm3, dword ptr [rip + 0x1f003df]
0000000140aa03f1: 0f2ed3                           ucomiss xmm2, xmm3
0000000140aa03f4: 41baffffffff                     mov r10d, 0xffffffff
0000000140aa03fa: 450f47c2                         cmova r8d, r10d
0000000140aa03fe: f3480f2cc9                       cvttss2si rcx, xmm1
0000000140aa0403: 0f2ec8                           ucomiss xmm1, xmm0
0000000140aa0406: 410f42c9                         cmovb ecx, r9d
0000000140aa040a: 0f2ecb                           ucomiss xmm1, xmm3
0000000140aa040d: 410f47ca                         cmova ecx, r10d
0000000140aa0411: 4181f8ff010000                   cmp r8d, 0x1ff
0000000140aa0418: 41b9ff010000                     mov r9d, 0x1ff
0000000140aa041e: 450f43c1                         cmovae r8d, r9d
0000000140aa0422: 81f9ff010000                     cmp ecx, 0x1ff
0000000140aa0428: 410f43c9                         cmovae ecx, r9d
0000000140aa042c: c1e109                           shl ecx, 9
0000000140aa042f: 4409c1                           or ecx, r8d
0000000140aa0432: c1e905                           shr ecx, 5
0000000140aa0435: 4839ca                           cmp rdx, rcx
0000000140aa0438: 0f8605040000                     jbe 0x140aa0843
0000000140aa043e: 488b00                           mov rax, qword ptr [rax]
0000000140aa0441: 8b0488                           mov eax, dword ptr [rax + rcx*4]
0000000140aa0444: 440fa3c0                         bt eax, r8d
0000000140aa0448: 7208                             jb 0x140aa0452
0000000140aa044a: f30f583d7e54e801                 addss xmm7, dword ptr [rip + 0x1e8547e]
0000000140aa0452: 44892e                           mov dword ptr [rsi], r13d
0000000140aa0455: 448965c0                         mov dword ptr [rbp - 0x40], r12d
0000000140aa0459: 48837f2000                       cmp qword ptr [rdi + 0x20], 0
0000000140aa045e: 0f848f020000                     je 0x140aa06f3
0000000140aa0464: 488bb560010000                   mov rsi, qword ptr [rbp + 0x160]
0000000140aa046b: 488b9d28010000                   mov rbx, qword ptr [rbp + 0x128]
0000000140aa0472: 488d4f28                         lea rcx, [rdi + 0x28]
0000000140aa0476: 488d55c0                         lea rdx, [rbp - 0x40]
0000000140aa047a: e8510ab1ff                       call 0x1405b0ed0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<shader_pipeline::reflection::BindingIndex> > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::resources::walls::decorator::DecoratorId> > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::resources::walls::decorator::DecoratorId> > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<i32> > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<etagere::AllocId> > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<u32> > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::resources::walls::decorator::DecoratorId> > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::resources::walls::decorator::DecoratorId> >
0000000140aa047f: 488b4f08                         mov rcx, qword ptr [rdi + 8]
0000000140aa0483: 488b5710                         mov rdx, qword ptr [rdi + 0x10]
0000000140aa0487: 4989d0                           mov r8, rdx
0000000140aa048a: 4921c0                           and r8, rax
0000000140aa048d: 48c1e839                         shr rax, 0x39
0000000140aa0491: 660f6ec0                         movd xmm0, eax
0000000140aa0495: 660f60c0                         punpcklbw xmm0, xmm0
0000000140aa0499: f20f70c000                       pshuflw xmm0, xmm0, 0
0000000140aa049e: 660f70c000                       pshufd xmm0, xmm0, 0
0000000140aa04a3: 488d41d0                         lea rax, [rcx - 0x30]
0000000140aa04a7: 4531c9                           xor r9d, r9d
0000000140aa04aa: 660f76c9                         pcmpeqd xmm1, xmm1
0000000140aa04ae: f3420f6f1401                     movdqu xmm2, xmmword ptr [rcx + r8]
0000000140aa04b4: 660f6fda                         movdqa xmm3, xmm2
0000000140aa04b8: 660f74d8                         pcmpeqb xmm3, xmm0
0000000140aa04bc: 66440fd7d3                       pmovmskb r10d, xmm3
0000000140aa04c1: 4585d2                           test r10d, r10d
0000000140aa04c4: 7429                             je 0x140aa04ef
0000000140aa04c6: f3450fbcda                       tzcnt r11d, r10d
0000000140aa04cb: 4d01c3                           add r11, r8
0000000140aa04ce: 4921d3                           and r11, rdx
0000000140aa04d1: 49f7db                           neg r11
0000000140aa04d4: 4f8d1c5b                         lea r11, [r11 + r11*2]
0000000140aa04d8: 49c1e304                         shl r11, 4
0000000140aa04dc: 46392418                         cmp dword ptr [rax + r11], r12d
0000000140aa04e0: 742f                             je 0x140aa0511
0000000140aa04e2: 458d5aff                         lea r11d, [r10 - 1]
0000000140aa04e6: 664521d3                         and r11w, r10w
0000000140aa04ea: 4589da                           mov r10d, r11d
0000000140aa04ed: 75d7                             jne 0x140aa04c6
0000000140aa04ef: 660f74d1                         pcmpeqb xmm2, xmm1
0000000140aa04f3: 66440fd7d2                       pmovmskb r10d, xmm2
0000000140aa04f8: 4585d2                           test r10d, r10d
0000000140aa04fb: 0f85f2010000                     jne 0x140aa06f3
0000000140aa0501: 4d01c8                           add r8, r9
0000000140aa0504: 4983c010                         add r8, 0x10
0000000140aa0508: 4983c110                         add r9, 0x10
0000000140aa050c: 4921d0                           and r8, rdx
0000000140aa050f: eb9d                             jmp 0x140aa04ae
0000000140aa0511: 0f28c7                           movaps xmm0, xmm7
0000000140aa0514: f30fc2c703                       cmpunordss xmm0, xmm7
0000000140aa0519: 0f28c8                           movaps xmm1, xmm0
0000000140aa051c: 410f54c8                         andps xmm1, xmm8
0000000140aa0520: f3440f5fc7                       maxss xmm8, xmm7
0000000140aa0525: 410f55c0                         andnps xmm0, xmm8
0000000140aa0529: 0f56c1                           orps xmm0, xmm1
0000000140aa052c: f30f5d05fcf4fd01                 minss xmm0, dword ptr [rip + 0x1fdf4fc]
0000000140aa0534: f3420f114419f8                   movss dword ptr [rcx + r11 - 8], xmm0
0000000140aa053b: 8b431c                           mov eax, dword ptr [rbx + 0x1c]
0000000140aa053e: 488b4b10                         mov rcx, qword ptr [rbx + 0x10]
0000000140aa0542: 8901                             mov dword ptr [rcx], eax
0000000140aa0544: 488b03                           mov rax, qword ptr [rbx]
0000000140aa0547: c680e001000001                   mov byte ptr [rax + 0x1e0], 1
0000000140aa054e: 418b461c                         mov eax, dword ptr [r14 + 0x1c]
0000000140aa0552: 498b4e10                         mov rcx, qword ptr [r14 + 0x10]
0000000140aa0556: 8901                             mov dword ptr [rcx], eax
0000000140aa0558: 498b3e                           mov rdi, qword ptr [r14]
0000000140aa055b: 488b5f30                         mov rbx, qword ptr [rdi + 0x30]
0000000140aa055f: 4c8b7740                         mov r14, qword ptr [rdi + 0x40]
0000000140aa0563: 483b5f20                         cmp rbx, qword ptr [rdi + 0x20]
0000000140aa0567: 7510                             jne 0x140aa0579
0000000140aa0569: 488d4f20                         lea rcx, [rdi + 0x20]
0000000140aa056d: 488d15d49b1902                   lea rdx, [rip + 0x2199bd4]
0000000140aa0574: e8b720acff                       call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<toolbar::ui_interaction::UiJustBecameInFocusEvent>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::walls::WallId,ref$<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> > >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemWindow>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<asset::prefab::PrefabName,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<alloc::ffi::c_str::CString,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemRoofShape>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<ref$<std::ffi::os_str::OsStr>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::systems::prefab::DespawnPrefabCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_onboarding::tutorial_completion::TutorialHintComplete>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_color::set_decorator_color_id::SetStairRailingCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<u64,ref$<system_clutter::autoclutter::AutoClutterObject> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<enum2$<core::option::Option<bevy_ecs::entity::Entity> >,bool>,alloc::alloc::Global>
0000000140aa0579: 488b4728                         mov rax, qword ptr [rdi + 0x28]
0000000140aa057d: 4889d9                           mov rcx, rbx
0000000140aa0580: 48c1e104                         shl rcx, 4
0000000140aa0584: 4c893408                         mov qword ptr [rax + rcx], r14
0000000140aa0588: 4489640808                       mov dword ptr [rax + rcx + 8], r12d
0000000140aa058d: 48ffc3                           inc rbx
0000000140aa0590: 48895f30                         mov qword ptr [rdi + 0x30], rbx
0000000140aa0594: 48ff4740                         inc qword ptr [rdi + 0x40]
0000000140aa0598: 41f60701                         test byte ptr [r15], 1
0000000140aa059c: 0f845d010000                     je 0x140aa06ff
0000000140aa05a2: 418b470c                         mov eax, dword ptr [r15 + 0xc]
0000000140aa05a6: 8945c8                           mov dword ptr [rbp - 0x38], eax
0000000140aa05a9: 498b4704                         mov rax, qword ptr [r15 + 4]
0000000140aa05ad: 488945c0                         mov qword ptr [rbp - 0x40], rax
0000000140aa05b1: e958010000                       jmp 0x140aa070e
0000000140aa05b6: b804000000                       mov eax, 4
0000000140aa05bb: 48c7455800000000                 mov qword ptr [rbp + 0x58], 0
0000000140aa05c3: 4c8bbd10010000                   mov r15, qword ptr [rbp + 0x110]
0000000140aa05ca: 4989c4                           mov r12, rax
0000000140aa05cd: 4889c1                           mov rcx, rax
0000000140aa05d0: 4c89f2                           mov rdx, r14
0000000140aa05d3: 4989d8                           mov r8, rbx
0000000140aa05d6: e82536e801                       call 0x142923c00 ; 
0000000140aa05db: 418b471c                         mov eax, dword ptr [r15 + 0x1c]
0000000140aa05df: 498b4f10                         mov rcx, qword ptr [r15 + 0x10]
0000000140aa05e3: 8901                             mov dword ptr [rcx], eax
0000000140aa05e5: 488b4e38                         mov rcx, qword ptr [rsi + 0x38]
0000000140aa05e9: 4885c9                           test rcx, rcx
0000000140aa05ec: 4c896540                         mov qword ptr [rbp + 0x40], r12
0000000140aa05f0: 48894d38                         mov qword ptr [rbp + 0x38], rcx
0000000140aa05f4: 0f840c020000                     je 0x140aa0806
0000000140aa05fa: 498b07                           mov rax, qword ptr [r15]
0000000140aa05fd: ffc9                             dec ecx
0000000140aa05ff: 488b5020                         mov rdx, qword ptr [rax + 0x20]
0000000140aa0603: 488955e0                         mov qword ptr [rbp - 0x20], rdx
0000000140aa0607: 0f1000                           movups xmm0, xmmword ptr [rax]
0000000140aa060a: 0f104810                         movups xmm1, xmmword ptr [rax + 0x10]
0000000140aa060e: 0f294dd0                         movaps xmmword ptr [rbp - 0x30], xmm1
0000000140aa0612: 0f2945c0                         movaps xmmword ptr [rbp - 0x40], xmm0
0000000140aa0616: 48ba0000000000000080             movabs rdx, 0x8000000000000000
0000000140aa0620: 488910                           mov qword ptr [rax], rdx
0000000140aa0623: 488b5558                         mov rdx, qword ptr [rbp + 0x58]
0000000140aa0627: 48895008                         mov qword ptr [rax + 8], rdx
0000000140aa062b: 4c896010                         mov qword ptr [rax + 0x10], r12
0000000140aa062f: 48897818                         mov qword ptr [rax + 0x18], rdi
0000000140aa0633: 894820                           mov dword ptr [rax + 0x20], ecx
0000000140aa0636: f30f117024                       movss dword ptr [rax + 0x24], xmm6
0000000140aa063b: 48b80200000000000080             movabs rax, 0x8000000000000002
0000000140aa0645: 483945c0                         cmp qword ptr [rbp - 0x40], rax
0000000140aa0649: 0f85c5010000                     jne 0x140aa0814
0000000140aa064f: 488b8538010000                   mov rax, qword ptr [rbp + 0x138]
0000000140aa0656: 8b481c                           mov ecx, dword ptr [rax + 0x1c]
0000000140aa0659: 488b5010                         mov rdx, qword ptr [rax + 0x10]
0000000140aa065d: 890a                             mov dword ptr [rdx], ecx
0000000140aa065f: 488b38                           mov rdi, qword ptr [rax]
0000000140aa0662: 488b5f30                         mov rbx, qword ptr [rdi + 0x30]
0000000140aa0666: 4c8b7740                         mov r14, qword ptr [rdi + 0x40]
0000000140aa066a: 483b5f20                         cmp rbx, qword ptr [rdi + 0x20]
0000000140aa066e: 448b6560                         mov r12d, dword ptr [rbp + 0x60]
0000000140aa0672: 7510                             jne 0x140aa0684
0000000140aa0674: 488d4f20                         lea rcx, [rdi + 0x20]
0000000140aa0678: 488d15c99a1902                   lea rdx, [rip + 0x2199ac9]
0000000140aa067f: e8ac1facff                       call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<toolbar::ui_interaction::UiJustBecameInFocusEvent>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::walls::WallId,ref$<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> > >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemWindow>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<asset::prefab::PrefabName,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<alloc::ffi::c_str::CString,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemRoofShape>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<ref$<std::ffi::os_str::OsStr>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::systems::prefab::DespawnPrefabCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_onboarding::tutorial_completion::TutorialHintComplete>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_color::set_decorator_color_id::SetStairRailingCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<u64,ref$<system_clutter::autoclutter::AutoClutterObject> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<enum2$<core::option::Option<bevy_ecs::entity::Entity> >,bool>,alloc::alloc::Global>
0000000140aa0684: 488b8540010000                   mov rax, qword ptr [rbp + 0x140]
0000000140aa068b: 488b4f28                         mov rcx, qword ptr [rdi + 0x28]
0000000140aa068f: 4889da                           mov rdx, rbx
0000000140aa0692: 48c1e204                         shl rdx, 4
0000000140aa0696: 4c893411                         mov qword ptr [rcx + rdx], r14
0000000140aa069a: c644110800                       mov byte ptr [rcx + rdx + 8], 0
0000000140aa069f: 48ffc3                           inc rbx
0000000140aa06a2: 48895f30                         mov qword ptr [rdi + 0x30], rbx
0000000140aa06a6: 48ff4740                         inc qword ptr [rdi + 0x40]
0000000140aa06aa: 8b481c                           mov ecx, dword ptr [rax + 0x1c]
0000000140aa06ad: 488b5010                         mov rdx, qword ptr [rax + 0x10]
0000000140aa06b1: 890a                             mov dword ptr [rdx], ecx
0000000140aa06b3: 488b38                           mov rdi, qword ptr [rax]
0000000140aa06b6: 488b5f30                         mov rbx, qword ptr [rdi + 0x30]
0000000140aa06ba: 4c8b7740                         mov r14, qword ptr [rdi + 0x40]
0000000140aa06be: 483b5f20                         cmp rbx, qword ptr [rdi + 0x20]
0000000140aa06c2: 7510                             jne 0x140aa06d4
0000000140aa06c4: 488d4f20                         lea rcx, [rdi + 0x20]
0000000140aa06c8: 488d15799a1902                   lea rdx, [rip + 0x2199a79]
0000000140aa06cf: e89c21acff                       call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<enum2$<country_core::resources::window::OsWindowCmd> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemStairsCanDetachFromWall>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<ash::vk::definitions::SwapchainKHR,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::history::FinishInputRecordingCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_clutter::autoclutter::ClutterRegionId,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_water::record_water_history::RecordWaterEditToHistoryCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
0000000140aa06d4: 488b4728                         mov rax, qword ptr [rdi + 0x28]
0000000140aa06d8: 4c8934d8                         mov qword ptr [rax + rbx*8], r14
0000000140aa06dc: 48ffc3                           inc rbx
0000000140aa06df: 48895f30                         mov qword ptr [rdi + 0x30], rbx
0000000140aa06e3: 48ff4740                         inc qword ptr [rdi + 0x40]
0000000140aa06e7: 45896500                         mov dword ptr [r13], r12d
0000000140aa06eb: c70600000000                     mov dword ptr [rsi], 0
0000000140aa06f1: eb7a                             jmp 0x140aa076d
0000000140aa06f3: 488d0d1ebc0302                   lea rcx, [rip + 0x203bc1e]
0000000140aa06fa: e8118ce301                       call 0x1428d9310 ; core::option::unwrap_failed
0000000140aa06ff: c745c800000000                   mov dword ptr [rbp - 0x38], 0
0000000140aa0706: 48c745c000000000                 mov qword ptr [rbp - 0x40], 0
0000000140aa070e: 8b461c                           mov eax, dword ptr [rsi + 0x1c]
0000000140aa0711: 488b4e10                         mov rcx, qword ptr [rsi + 0x10]
0000000140aa0715: 8901                             mov dword ptr [rcx], eax
0000000140aa0717: 488b36                           mov rsi, qword ptr [rsi]
0000000140aa071a: 488b7e30                         mov rdi, qword ptr [rsi + 0x30]
0000000140aa071e: 488b5e40                         mov rbx, qword ptr [rsi + 0x40]
0000000140aa0722: 483b7e20                         cmp rdi, qword ptr [rsi + 0x20]
0000000140aa0726: 7510                             jne 0x140aa0738
0000000140aa0728: 488d4e20                         lea rcx, [rsi + 0x20]
0000000140aa072c: 488d15159a1902                   lea rdx, [rip + 0x2199a15]
0000000140aa0733: e87823acff                       call 0x140562ab0 ; alloc::raw_vec::RawVec::grow_one<indexmap::Bucket<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<theme_server_proto::ActiveEvent,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<toolbar::resouces_and_components::taffy_layout::AnchoredNode,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,ref$<glam::f32::vec3::Vec3>,glam::f32::vec2::Vec2,f32,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,enum2$<country_core::resources::walls::decorator::DecoratorType> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::systems::wall::merge_walls::MergeWallsCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<ash::vk::definitions::DeviceQueueCreateInfo,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::roofs::roof_sounds::PlayRoofSound>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_color::radial_menu::IconInfo,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlonsHole,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>
0000000140aa0738: 488b4628                         mov rax, qword ptr [rsi + 0x28]
0000000140aa073c: 488d0cbf                         lea rcx, [rdi + rdi*4]
0000000140aa0740: 48891cc8                         mov qword ptr [rax + rcx*8], rbx
0000000140aa0744: c744c80805000000                 mov dword ptr [rax + rcx*8 + 8], 5
0000000140aa074c: 488b55c0                         mov rdx, qword ptr [rbp - 0x40]
0000000140aa0750: 488954c80c                       mov qword ptr [rax + rcx*8 + 0xc], rdx
0000000140aa0755: 8b55c8                           mov edx, dword ptr [rbp - 0x38]
0000000140aa0758: 8954c814                         mov dword ptr [rax + rcx*8 + 0x14], edx
0000000140aa075c: f30f117cc818                     movss dword ptr [rax + rcx*8 + 0x18], xmm7
0000000140aa0762: 48ffc7                           inc rdi
0000000140aa0765: 48897e30                         mov qword ptr [rsi + 0x30], rdi
0000000140aa0769: 48ff4640                         inc qword ptr [rsi + 0x40]
0000000140aa076d: 0f287570                         movaps xmm6, xmmword ptr [rbp + 0x70]
0000000140aa0771: 0f28bd80000000                   movaps xmm7, xmmword ptr [rbp + 0x80]
0000000140aa0778: 440f288590000000                 movaps xmm8, xmmword ptr [rbp + 0x90]
0000000140aa0780: 4881c428010000                   add rsp, 0x128
0000000140aa0787: 5b                               pop rbx
0000000140aa0788: 5f                               pop rdi
0000000140aa0789: 5e                               pop rsi
0000000140aa078a: 415c                             pop r12
0000000140aa078c: 415d                             pop r13
0000000140aa078e: 415e                             pop r14
0000000140aa0790: 415f                             pop r15
0000000140aa0792: 5d                               pop rbp
0000000140aa0793: c3                               ret 
0000000140aa0794: 48c745c000000000                 mov qword ptr [rbp - 0x40], 0
0000000140aa079c: 4883c310                         add rbx, 0x10
0000000140aa07a0: 488d05d9d91802                   lea rax, [rip + 0x218d9d9]
0000000140aa07a7: 4889442420                       mov qword ptr [rsp + 0x20], rax
0000000140aa07ac: 488d5528                         lea rdx, [rbp + 0x28]
0000000140aa07b0: 4c8d4dc0                         lea r9, [rbp - 0x40]
0000000140aa07b4: 31c9                             xor ecx, ecx
0000000140aa07b6: 4989d8                           mov r8, rbx
0000000140aa07b9: e883a3e201                       call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
0000000140aa07be: 48c745c000000000                 mov qword ptr [rbp - 0x40], 0
0000000140aa07c6: 4983c610                         add r14, 0x10
0000000140aa07ca: 488d05afd91802                   lea rax, [rip + 0x218d9af]
0000000140aa07d1: 4889442420                       mov qword ptr [rsp + 0x20], rax
0000000140aa07d6: 488d5528                         lea rdx, [rbp + 0x28]
0000000140aa07da: 4c8d4dc0                         lea r9, [rbp - 0x40]
0000000140aa07de: 31c9                             xor ecx, ecx
0000000140aa07e0: 4d89f0                           mov r8, r14
0000000140aa07e3: e859a3e201                       call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
0000000140aa07e8: 4c8d0589831902                   lea r8, [rip + 0x2198389]
0000000140aa07ef: 4c89f9                           mov rcx, r15
0000000140aa07f2: 4889da                           mov rdx, rbx
0000000140aa07f5: e8e987e301                       call 0x1428d8fe3 ; alloc::raw_vec::handle_error
0000000140aa07fa: 488d0dffba0302                   lea rcx, [rip + 0x203baff]
0000000140aa0801: e80a8be301                       call 0x1428d9310 ; core::option::unwrap_failed
0000000140aa0806: 488d0dfbe00002                   lea rcx, [rip + 0x200e0fb]
0000000140aa080d: e8fe8ae301                       call 0x1428d9310 ; core::option::unwrap_failed
0000000140aa0812: eb21                             jmp 0x140aa0835
0000000140aa0814: 488d4dc0                         lea rcx, [rbp - 0x40]
0000000140aa0818: e8c32ef0ff                       call 0x1409a36e0 ; core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > > > | core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > > > | core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > > >
0000000140aa081d: 488d0d31bb0302                   lea rcx, [rip + 0x203bb31]
0000000140aa0824: 4c8d053dbc0302                   lea r8, [rip + 0x203bc3d]
0000000140aa082b: ba0f010000                       mov edx, 0x10f
0000000140aa0830: e8fb8be301                       call 0x1428d9430 ; core::panicking::panic
0000000140aa0835: 0f0b                             ud2 
0000000140aa0837: 488d0dcae00002                   lea rcx, [rip + 0x200e0ca]
0000000140aa083e: e8cd8ae301                       call 0x1428d9310 ; core::option::unwrap_failed
0000000140aa0843: 4c8d0546480202                   lea r8, [rip + 0x2024846]
0000000140aa084a: e8c98ce301                       call 0x1428d9518 ; core::panicking::panic_bounds_check
0000000140aa084f: 4c8d052abc0302                   lea r8, [rip + 0x203bc2a]
0000000140aa0856: 31c9                             xor ecx, ecx
0000000140aa0858: 31d2                             xor edx, edx
0000000140aa085a: e8b98ce301                       call 0x1428d9518 ; core::panicking::panic_bounds_check
0000000140aa085f: cc                               int3 
0000000140aa0860: 4889542410                       mov qword ptr [rsp + 0x10], rdx
0000000140aa0865: 55                               push rbp
0000000140aa0866: 4157                             push r15
0000000140aa0868: 4156                             push r14
0000000140aa086a: 4155                             push r13
0000000140aa086c: 4154                             push r12
0000000140aa086e: 56                               push rsi
0000000140aa086f: 57                               push rdi
0000000140aa0870: 53                               push rbx
0000000140aa0871: 4883ec68                         sub rsp, 0x68
0000000140aa0875: 488daa80000000                   lea rbp, [rdx + 0x80]
0000000140aa087c: 440f29442430                     movaps xmmword ptr [rsp + 0x30], xmm8
0000000140aa0882: 0f297c2440                       movaps xmmword ptr [rsp + 0x40], xmm7
0000000140aa0887: 0f29742450                       movaps xmmword ptr [rsp + 0x50], xmm6
0000000140aa088c: 48837d3800                       cmp qword ptr [rbp + 0x38], 0
0000000140aa0891: 0f95c0                           setne al
0000000140aa0894: 48837d5800                       cmp qword ptr [rbp + 0x58], 0
0000000140aa0899: 0f94c1                           sete cl
0000000140aa089c: 08c1                             or cl, al
0000000140aa089e: 751b                             jne 0x140aa08bb
0000000140aa08a0: 488b4558                         mov rax, qword ptr [rbp + 0x58]
0000000140aa08a4: 48c1e002                         shl rax, 2
0000000140aa08a8: 488d1440                         lea rdx, [rax + rax*2]
0000000140aa08ac: 41b804000000                     mov r8d, 4
0000000140aa08b2: 488b4d40                         mov rcx, qword ptr [rbp + 0x40]
0000000140aa08b6: e86533b7ff                       call 0x140613c20 ; 
0000000140aa08bb: 0f28742450                       movaps xmm6, xmmword ptr [rsp + 0x50]
0000000140aa08c0: 0f287c2440                       movaps xmm7, xmmword ptr [rsp + 0x40]
0000000140aa08c5: 440f28442430                     movaps xmm8, xmmword ptr [rsp + 0x30]
0000000140aa08cb: 4883c468                         add rsp, 0x68
0000000140aa08cf: 5b                               pop rbx
0000000140aa08d0: 5f                               pop rdi
0000000140aa08d1: 5e                               pop rsi
0000000140aa08d2: 415c                             pop r12
0000000140aa08d4: 415d                             pop r13
0000000140aa08d6: 415e                             pop r14
0000000140aa08d8: 415f                             pop r15
0000000140aa08da: 5d                               pop rbp
0000000140aa08db: c3                               ret 

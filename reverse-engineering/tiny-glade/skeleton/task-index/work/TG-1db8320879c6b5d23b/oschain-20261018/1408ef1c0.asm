00000001408ef1c0: 56                               push rsi
00000001408ef1c1: 57                               push rdi
00000001408ef1c2: 53                               push rbx
00000001408ef1c3: 4883ec50                         sub rsp, 0x50
00000001408ef1c7: 4c89ce                           mov rsi, r9
00000001408ef1ca: 488bbc2490000000                 mov rdi, qword ptr [rsp + 0x90]
00000001408ef1d2: 488b01                           mov rax, qword ptr [rcx]
00000001408ef1d5: f30f1008                         movss xmm1, dword ptr [rax]
00000001408ef1d9: f30f105004                       movss xmm2, dword ptr [rax + 4]
00000001408ef1de: 4c8b0a                           mov r9, qword ptr [rdx]
00000001408ef1e1: 498b00                           mov rax, qword ptr [r8]
00000001408ef1e4: 8b08                             mov ecx, dword ptr [rax]
00000001408ef1e6: f3480f2ac1                       cvtsi2ss xmm0, rcx
00000001408ef1eb: 8b4004                           mov eax, dword ptr [rax + 4]
00000001408ef1ee: f3480f2ad8                       cvtsi2ss xmm3, rax
00000001408ef1f3: f30f115c2428                     movss dword ptr [rsp + 0x28], xmm3
00000001408ef1f9: f30f11442420                     movss dword ptr [rsp + 0x20], xmm0
00000001408ef1ff: 488d4c2438                       lea rcx, [rsp + 0x38]
00000001408ef204: e8a7d89800                       call 0x14127cab0 ; utils::geometry::ray::Ray::from_mouse_ss
00000001408ef209: 8b461c                           mov eax, dword ptr [rsi + 0x1c]
00000001408ef20c: 488b4e10                         mov rcx, qword ptr [rsi + 0x10]
00000001408ef210: 8901                             mov dword ptr [rcx], eax
00000001408ef212: 488b06                           mov rax, qword ptr [rsi]
00000001408ef215: 488b4c2448                       mov rcx, qword ptr [rsp + 0x48]
00000001408ef21a: 48894810                         mov qword ptr [rax + 0x10], rcx
00000001408ef21e: 0f10442438                       movups xmm0, xmmword ptr [rsp + 0x38]
00000001408ef223: 0f1100                           movups xmmword ptr [rax], xmm0
00000001408ef226: 488b07                           mov rax, qword ptr [rdi]
00000001408ef229: 488b4f08                         mov rcx, qword ptr [rdi + 8]
00000001408ef22d: 488b10                           mov rdx, qword ptr [rax]
00000001408ef230: 4531c0                           xor r8d, r8d
00000001408ef233: 4989d1                           mov r9, rdx
00000001408ef236: 4c2b4918                         sub r9, qword ptr [rcx + 0x18]
00000001408ef23a: 4d0f42c8                         cmovb r9, r8
00000001408ef23e: 482b5138                         sub rdx, qword ptr [rcx + 0x38]
00000001408ef242: 490f42d0                         cmovb rdx, r8
00000001408ef246: 4c8b5130                         mov r10, qword ptr [rcx + 0x30]
00000001408ef24a: 4929d2                           sub r10, rdx
00000001408ef24d: 4d0f42d0                         cmovb r10, r8
00000001408ef251: 0f96c2                           setbe dl
00000001408ef254: 4c8b5910                         mov r11, qword ptr [rcx + 0x10]
00000001408ef258: 4d29cb                           sub r11, r9
00000001408ef25b: 4d0f42d8                         cmovb r11, r8
00000001408ef25f: 488b4940                         mov rcx, qword ptr [rcx + 0x40]
00000001408ef263: 410f96c0                         setbe r8b
00000001408ef267: 4d01d3                           add r11, r10
00000001408ef26a: 4989c9                           mov r9, rcx
00000001408ef26d: 4d29d9                           sub r9, r11
00000001408ef270: 4c8908                           mov qword ptr [rax], r9
00000001408ef273: 4184d0                           test r8b, dl
00000001408ef276: 744b                             je 0x1408ef2c3
00000001408ef278: 488b842498000000                 mov rax, qword ptr [rsp + 0x98]
00000001408ef280: 488b00                           mov rax, qword ptr [rax]
00000001408ef283: f20f108058010000                 movsd xmm0, qword ptr [rax + 0x158]
00000001408ef28b: 0f59c0                           mulps xmm0, xmm0
00000001408ef28e: f30f16c8                         movshdup xmm1, xmm0
00000001408ef292: f30f58c8                         addss xmm1, xmm0
00000001408ef296: f30f51c9                         sqrtss xmm1, xmm1
00000001408ef29a: 0f57c0                           xorps xmm0, xmm0
00000001408ef29d: 0f2ec8                           ucomiss xmm1, xmm0
00000001408ef2a0: 7724                             ja 0x1408ef2c6
00000001408ef2a2: f20f108850010000                 movsd xmm1, qword ptr [rax + 0x150]
00000001408ef2aa: 0f59c9                           mulps xmm1, xmm1
00000001408ef2ad: f30f16d1                         movshdup xmm2, xmm1
00000001408ef2b1: f30f58d1                         addss xmm2, xmm1
00000001408ef2b5: 0f57c9                           xorps xmm1, xmm1
00000001408ef2b8: f30f51ca                         sqrtss xmm1, xmm2
00000001408ef2bc: 0f2ec8                           ucomiss xmm1, xmm0
00000001408ef2bf: 7705                             ja 0x1408ef2c6
00000001408ef2c1: eb48                             jmp 0x1408ef30b
00000001408ef2c3: 488908                           mov qword ptr [rax], rcx
00000001408ef2c6: 488b8424a0000000                 mov rax, qword ptr [rsp + 0xa0]
00000001408ef2ce: 8b481c                           mov ecx, dword ptr [rax + 0x1c]
00000001408ef2d1: 488b5010                         mov rdx, qword ptr [rax + 0x10]
00000001408ef2d5: 890a                             mov dword ptr [rdx], ecx
00000001408ef2d7: 488b30                           mov rsi, qword ptr [rax]
00000001408ef2da: 488b7e30                         mov rdi, qword ptr [rsi + 0x30]
00000001408ef2de: 488b5e40                         mov rbx, qword ptr [rsi + 0x40]
00000001408ef2e2: 483b7e20                         cmp rdi, qword ptr [rsi + 0x20]
00000001408ef2e6: 7510                             jne 0x1408ef2f8
00000001408ef2e8: 488d4e20                         lea rcx, [rsi + 0x20]
00000001408ef2ec: 488d1555ae3402                   lea rdx, [rip + 0x234ae55]
00000001408ef2f3: e87835c7ff                       call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<enum2$<country_core::resources::window::OsWindowCmd> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemStairsCanDetachFromWall>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<ash::vk::definitions::SwapchainKHR,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::history::FinishInputRecordingCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_clutter::autoclutter::ClutterRegionId,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_water::record_water_history::RecordWaterEditToHistoryCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
00000001408ef2f8: 488b4628                         mov rax, qword ptr [rsi + 0x28]
00000001408ef2fc: 48891cf8                         mov qword ptr [rax + rdi*8], rbx
00000001408ef300: 48ffc7                           inc rdi
00000001408ef303: 48897e30                         mov qword ptr [rsi + 0x30], rdi
00000001408ef307: 48ff4640                         inc qword ptr [rsi + 0x40]
00000001408ef30b: 4883c450                         add rsp, 0x50
00000001408ef30f: 5b                               pop rbx
00000001408ef310: 5f                               pop rdi
00000001408ef311: 5e                               pop rsi
00000001408ef312: c3                               ret 

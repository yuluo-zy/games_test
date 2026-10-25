1409ebf20: 55 push rbp
1409ebf21: 4157 push r15
1409ebf23: 4156 push r14
1409ebf25: 4155 push r13
1409ebf27: 4154 push r12
1409ebf29: 56 push rsi
1409ebf2a: 57 push rdi
1409ebf2b: 53 push rbx
1409ebf2c: 4881ec28020000 sub rsp, 0x228
1409ebf33: 488dac2480000000 lea rbp, [rsp + 0x80]
1409ebf3b: 440f299d90010000 movaps xmmword ptr [rbp + 0x190], xmm11
1409ebf43: 440f299580010000 movaps xmmword ptr [rbp + 0x180], xmm10
1409ebf4b: 440f298d70010000 movaps xmmword ptr [rbp + 0x170], xmm9
1409ebf53: 440f298560010000 movaps xmmword ptr [rbp + 0x160], xmm8
1409ebf5b: 0f29bd50010000 movaps xmmword ptr [rbp + 0x150], xmm7
1409ebf62: 0f29b540010000 movaps xmmword ptr [rbp + 0x140], xmm6
1409ebf69: 48c78538010000feffffff mov qword ptr [rbp + 0x138], 0xfffffffffffffffe
1409ebf74: 803d158ec40200 cmp byte ptr [rip + 0x2c48e15], 0
1409ebf7b: 0f852d040000 jne 0x1409ec3ae
1409ebf81: 4889d7 mov rdi, rdx
1409ebf84: 4889ce mov rsi, rcx
1409ebf87: 0fb6057b8cc402 movzx eax, byte ptr [rip + 0x2c48c7b]
1409ebf8e: b9e0150000 mov ecx, 0x15e0
1409ebf93: ba08000000 mov edx, 8
1409ebf98: e8737cc2ff call 0x140613c10 ; 
1409ebf9d: 4885c0 test rax, rax
1409ebfa0: 0f84fd060000 je 0x1409ec6a3
1409ebfa6: 4889bde0000000 mov qword ptr [rbp + 0xe0], rdi
1409ebfad: 48c785a000000064000000 mov qword ptr [rbp + 0xa0], 0x64
1409ebfb8: 488985a8000000 mov qword ptr [rbp + 0xa8], rax
1409ebfbf: 48c785b000000000000000 mov qword ptr [rbp + 0xb0], 0
1409ebfca: 0fb605388cc402 movzx eax, byte ptr [rip + 0x2c48c38]
1409ebfd1: b960090000 mov ecx, 0x960
1409ebfd6: ba08000000 mov edx, 8
1409ebfdb: e8307cc2ff call 0x140613c10 ; 
1409ebfe0: 4885c0 test rax, rax
1409ebfe3: 0f84d0060000 je 0x1409ec6b9
1409ebfe9: 48c785b800000064000000 mov qword ptr [rbp + 0xb8], 0x64
1409ebff4: 488985c0000000 mov qword ptr [rbp + 0xc0], rax
1409ebffb: 48c785c800000000000000 mov qword ptr [rbp + 0xc8], 0
1409ec006: 4c8b2e mov r13, qword ptr [rsi]
1409ec009: 31c0 xor eax, eax
1409ec00b: f30f10355d98f301 movss xmm6, dword ptr [rip + 0x1f3985d]
1409ec013: f30f103d8d85f401 movss xmm7, dword ptr [rip + 0x1f4858d]
1409ec01b: f3440f1005e098f301 movss xmm8, dword ptr [rip + 0x1f398e0]
1409ec024: f3440f100de7770902 movss xmm9, dword ptr [rip + 0x20977e7]
1409ec02d: 488d1d64920d02 lea rbx, [rip + 0x20d9264] ; 0x142ac5298 'event:/SFX/Water/Amb'
1409ec034: 4c8db5d0000000 lea r14, [rbp + 0xd0]
1409ec03b: 488d356ebb2802 lea rsi, [rip + 0x228bb6e]
1409ec042: eb13 jmp 0x1409ec057
1409ec044: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ec050: 488b85e8000000 mov rax, qword ptr [rbp + 0xe8]
1409ec057: 4883f80a cmp rax, 0xa
1409ec05b: 0f928535010000 setb byte ptr [rbp + 0x135]
1409ec062: 0f83cc020000 jae 0x1409ec334
1409ec068: 450f57d2 xorps xmm10, xmm10
1409ec06c: f34c0f2ad0 cvtsi2ss xmm10, rax
1409ec071: 48ffc0 inc rax
1409ec074: 488985e8000000 mov qword ptr [rbp + 0xe8], rax
1409ec07b: f3440f58d6 addss xmm10, xmm6
1409ec080: f3440f5ed7 divss xmm10, xmm7
1409ec085: 410f28c0 movaps xmm0, xmm8
1409ec089: f3410f5cc2 subss xmm0, xmm10
1409ec08e: f3410f59c1 mulss xmm0, xmm9
1409ec093: f3450f59d1 mulss xmm10, xmm9
1409ec098: f3440f5cd0 subss xmm10, xmm0
1409ec09d: 4531ff xor r15d, r15d
1409ec0a0: eb63 jmp 0x1409ec105
1409ec0a2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ec0b0: 4c8bbdd8000000 mov r15, qword ptr [rbp + 0xd8]
1409ec0b7: 49ffc7 inc r15
1409ec0ba: 410f28c0 movaps xmm0, xmm8
1409ec0be: f3410f5cc3 subss xmm0, xmm11
1409ec0c3: f3410f59c1 mulss xmm0, xmm9
1409ec0c8: f3450f59d9 mulss xmm11, xmm9
1409ec0cd: f3440f5cd8 subss xmm11, xmm0
1409ec0d2: 488b85c0000000 mov rax, qword ptr [rbp + 0xc0]
1409ec0d9: 488d0c7f lea rcx, [rdi + rdi*2]
1409ec0dd: f3440f111cc8 movss dword ptr [rax + rcx*8], xmm11
1409ec0e3: f3440f1154c804 movss dword ptr [rax + rcx*8 + 4], xmm10
1409ec0ea: 48c744c80800000000 mov qword ptr [rax + rcx*8 + 8], 0
1409ec0f3: c744c81000000000 mov dword ptr [rax + rcx*8 + 0x10], 0
1409ec0fb: 48ffc7 inc rdi
1409ec0fe: 4889bdc8000000 mov qword ptr [rbp + 0xc8], rdi
1409ec105: 4983ff09 cmp r15, 9
1409ec109: 0f8741ffffff ja 0x1409ec050
1409ec10f: 450f57db xorps xmm11, xmm11
1409ec113: f34d0f2adf cvtsi2ss xmm11, r15
1409ec118: f3440f58de addss xmm11, xmm6
1409ec11d: f3440f5edf divss xmm11, xmm7
1409ec122: 48c785d000000000000000 mov qword ptr [rbp + 0xd0], 0
1409ec12d: 4c89adf0000000 mov qword ptr [rbp + 0xf0], r13
1409ec134: 48899df8000000 mov qword ptr [rbp + 0xf8], rbx
1409ec13b: 48c7850001000014000000 mov qword ptr [rbp + 0x100], 0x14
1409ec146: 48c7850801000000000000 mov qword ptr [rbp + 0x108], 0
1409ec151: 48c7851001000004000000 mov qword ptr [rbp + 0x110], 4
1409ec15c: 4c89b518010000 mov qword ptr [rbp + 0x118], r14
1409ec163: c6853401000000 mov byte ptr [rbp + 0x134], 0
1409ec16a: 488d8d80000000 lea rcx, [rbp + 0x80]
1409ec171: 488d95f0000000 lea rdx, [rbp + 0xf0]
1409ec178: 4989f0 mov r8, rsi
1409ec17b: e830ee9000 call 0x1412fafb0 ; alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::audio_resources::AudioInstance,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >
1409ec180: 4c8ba5d0000000 mov r12, qword ptr [rbp + 0xd0]
1409ec187: 4d85e4 test r12, r12
1409ec18a: 0f8560020000 jne 0x1409ec3f0
1409ec190: 4c89bdd8000000 mov qword ptr [rbp + 0xd8], r15
1409ec197: 4c8bbd80000000 mov r15, qword ptr [rbp + 0x80]
1409ec19e: 488b8d88000000 mov rcx, qword ptr [rbp + 0x88]
1409ec1a5: 4c89f8 mov rax, r15
1409ec1a8: 48f7d8 neg rax
1409ec1ab: 0f8082020000 jo 0x1409ec433
1409ec1b1: 488bbd90000000 mov rdi, qword ptr [rbp + 0x90]
1409ec1b8: 4c897d18 mov qword ptr [rbp + 0x18], r15
1409ec1bc: 48894d20 mov qword ptr [rbp + 0x20], rcx
1409ec1c0: 48897d28 mov qword ptr [rbp + 0x28], rdi
1409ec1c4: 4885ff test rdi, rdi
1409ec1c7: 0f849d020000 je 0x1409ec46a
1409ec1cd: 833901 cmp dword ptr [rcx], 1
1409ec1d0: 757e jne 0x1409ec250
1409ec1d2: 4c89ee mov rsi, r13
1409ec1d5: 4d89f5 mov r13, r14
1409ec1d8: 4989de mov r14, rbx
1409ec1db: 4989cc mov r12, rcx
1409ec1de: 488d5108 lea rdx, [rcx + 8]
1409ec1e2: 41b904000000 mov r9d, 4
1409ec1e8: 488d8df0000000 lea rcx, [rbp + 0xf0]
1409ec1ef: 4c8d052a6d0e02 lea r8, [rip + 0x20e6d2a] ; 0x142ad2f20 'loop'
1409ec1f6: e865b8a400 call 0x141437a60 ; libfmod::EventInstance::get_parameter_by_name
1409ec1fb: 488b9df0000000 mov rbx, qword ptr [rbp + 0xf0]
1409ec202: 48b80400000000000080 movabs rax, 0x8000000000000004
1409ec20c: 4839c3 cmp rbx, rax
1409ec20f: 740c je 0x1409ec21d
1409ec211: 488d8df0000000 lea rcx, [rbp + 0xf0]
1409ec218: e893640600 call 0x140a526b0 ; core::ptr::drop_in_place<enum2$<libfmod::Error> >
1409ec21d: 48b80400000000000080 movabs rax, 0x8000000000000004
1409ec227: 4839c3 cmp rbx, rax
1409ec22a: 0f94c0 sete al
1409ec22d: 4c89f3 mov rbx, r14
1409ec230: 4d89ee mov r14, r13
1409ec233: 4989f5 mov r13, rsi
1409ec236: 488d3573b92802 lea rsi, [rip + 0x228b973]
1409ec23d: 4c89e1 mov rcx, r12
1409ec240: eb10 jmp 0x1409ec252
1409ec242: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ec250: 31c0 xor eax, eax
1409ec252: 4c89bdf0000000 mov qword ptr [rbp + 0xf0], r15
1409ec259: 48898df8000000 mov qword ptr [rbp + 0xf8], rcx
1409ec260: 4889bd00010000 mov qword ptr [rbp + 0x100], rdi
1409ec267: c7851001000000ca9a3b mov dword ptr [rbp + 0x110], 0x3b9aca00
1409ec271: 48c7851801000000000000 mov qword ptr [rbp + 0x118], 0
1409ec27c: c7852001000032000000 mov dword ptr [rbp + 0x120], 0x32
1409ec286: c6852401000000 mov byte ptr [rbp + 0x124], 0
1409ec28d: 888525010000 mov byte ptr [rbp + 0x125], al
1409ec293: c6852601000000 mov byte ptr [rbp + 0x126], 0
1409ec29a: 488bbdb0000000 mov rdi, qword ptr [rbp + 0xb0]
1409ec2a1: 483bbda0000000 cmp rdi, qword ptr [rbp + 0xa0]
1409ec2a8: 7513 jne 0x1409ec2bd
1409ec2aa: 488d8da0000000 lea rcx, [rbp + 0xa0]
1409ec2b1: 488d1510900d02 lea rdx, [rip + 0x20d9010]
1409ec2b8: e81357c4ff call 0x1406319d0 ; alloc::raw_vec::RawVec::grow_one<enum2$<iced_core::event::Event>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_critters::boids::BoidAux,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<country_core::systems::ui::GlobalId,enum2$<toolbar::mei::core::IsRoot> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<utils::curve::Curve<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<rhapsody::raw::texture::Texture,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlonsDescription,alloc::alloc::Global>
1409ec2bd: 488b85a8000000 mov rax, qword ptr [rbp + 0xa8]
1409ec2c4: 486bcf38 imul rcx, rdi, 0x38
1409ec2c8: 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
1409ec2cf: 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
1409ec2d6: 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
1409ec2dd: 488b9520010000 mov rdx, qword ptr [rbp + 0x120]
1409ec2e4: 4889540830 mov qword ptr [rax + rcx + 0x30], rdx
1409ec2e9: 0f11540820 movups xmmword ptr [rax + rcx + 0x20], xmm2
1409ec2ee: 0f114c0810 movups xmmword ptr [rax + rcx + 0x10], xmm1
1409ec2f3: 0f110408 movups xmmword ptr [rax + rcx], xmm0
1409ec2f7: 48ffc7 inc rdi
1409ec2fa: 4889bdb0000000 mov qword ptr [rbp + 0xb0], rdi
1409ec301: 488bbdc8000000 mov rdi, qword ptr [rbp + 0xc8]
1409ec308: 483bbdb8000000 cmp rdi, qword ptr [rbp + 0xb8]
1409ec30f: 0f859bfdffff jne 0x1409ec0b0
1409ec315: c6853701000001 mov byte ptr [rbp + 0x137], 1
1409ec31c: 488d8db8000000 lea rcx, [rbp + 0xb8]
1409ec323: 488d15b68f0d02 lea rdx, [rip + 0x20d8fb6]
1409ec32a: e8816ab7ff call 0x140562db0 ; alloc::raw_vec::RawVec::grow_one<tuple$<ref$<str$>,enum2$<country_core::resources::gui::SettingsMenuState> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpmc::waker::Entry,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<alloc::vec::Vec<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > >,alloc::alloc::Global>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpmc::waker::Entry,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::systems::save::save_game::DeleteSessionCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::systems::audio::elevation_supports_audio::PlayElevationSupportSoundCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<indexmap::Bucket<country_core::systems::collision::world::SubworldColliderHandle,ordered_float::OrderedFloat<f32> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<alloc::vec::Vec<enum2$<core::option::Option<tuple$<relative_path::
1409ec32f: e97cfdffff jmp 0x1409ec0b0
1409ec334: 488b85b0000000 mov rax, qword ptr [rbp + 0xb0]
1409ec33b: 48898500010000 mov qword ptr [rbp + 0x100], rax
1409ec342: 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
1409ec349: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
1409ec350: c6853701000001 mov byte ptr [rbp + 0x137], 1
1409ec357: 4c8d050a8f0d02 lea r8, [rip + 0x20d8f0a]
1409ec35e: 488d95f0000000 lea rdx, [rbp + 0xf0]
1409ec365: 488b8de0000000 mov rcx, qword ptr [rbp + 0xe0]
1409ec36c: e8ef368f00 call 0x1412dfa60 ; bevy_ecs::system::commands::Commands::insert_resource<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>
1409ec371: 488b85c8000000 mov rax, qword ptr [rbp + 0xc8]
1409ec378: 48898500010000 mov qword ptr [rbp + 0x100], rax
1409ec37f: 0f1085b8000000 movups xmm0, xmmword ptr [rbp + 0xb8]
1409ec386: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
1409ec38d: c6853701000000 mov byte ptr [rbp + 0x137], 0
1409ec394: 4c8d05e58e0d02 lea r8, [rip + 0x20d8ee5]
1409ec39b: 488d95f0000000 lea rdx, [rbp + 0xf0]
1409ec3a2: 488b8de0000000 mov rcx, qword ptr [rbp + 0xe0]
1409ec3a9: e892298f00 call 0x1412ded40 ; bevy_ecs::system::commands::Commands::insert_resource<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
1409ec3ae: 0f28b540010000 movaps xmm6, xmmword ptr [rbp + 0x140]
1409ec3b5: 0f28bd50010000 movaps xmm7, xmmword ptr [rbp + 0x150]
1409ec3bc: 440f288560010000 movaps xmm8, xmmword ptr [rbp + 0x160]
1409ec3c4: 440f288d70010000 movaps xmm9, xmmword ptr [rbp + 0x170]
1409ec3cc: 440f289580010000 movaps xmm10, xmmword ptr [rbp + 0x180]
1409ec3d4: 440f289d90010000 movaps xmm11, xmmword ptr [rbp + 0x190]
1409ec3dc: 4881c428020000 add rsp, 0x228
1409ec3e3: 5b pop rbx
1409ec3e4: 5f pop rdi
1409ec3e5: 5e pop rsi
1409ec3e6: 415c pop r12
1409ec3e8: 415d pop r13
1409ec3ea: 415e pop r14
1409ec3ec: 415f pop r15
1409ec3ee: 5d pop rbp
1409ec3ef: c3 ret 
1409ec3f0: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
1409ec3f7: 488985e8000000 mov qword ptr [rbp + 0xe8], rax
1409ec3fe: 4c8bad90000000 mov r13, qword ptr [rbp + 0x90]
1409ec405: 4d85ed test r13, r13
1409ec408: 7575 jne 0x1409ec47f
1409ec40a: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
1409ec411: 4885c0 test rax, rax
1409ec414: 741a je 0x1409ec430
1409ec416: 48c1e004 shl rax, 4
1409ec41a: 488d1440 lea rdx, [rax + rax*2]
1409ec41e: 41b808000000 mov r8d, 8
1409ec424: 488b8de8000000 mov rcx, qword ptr [rbp + 0xe8]
1409ec42b: e8f077c2ff call 0x140613c20 ; 
1409ec430: 4c89e1 mov rcx, r12
1409ec433: 48898df0000000 mov qword ptr [rbp + 0xf0], rcx
1409ec43a: 488d056f8e0d02 lea rax, [rip + 0x20d8e6f]
1409ec441: 4889442420 mov qword ptr [rsp + 0x20], rax
1409ec446: 488d0ddb890d02 lea rcx, [rip + 0x20d89db] ; 0x142ac4e28 'called `Result::unwrap()` on an `Err` valuecountry_core::systems::audio::water_ambience_audio::water_ambience_audio_system::fcou'
1409ec44d: 4c8d0db4890d02 lea r9, [rip + 0x20d89b4]
1409ec454: 4c8d85f0000000 lea r8, [rbp + 0xf0]
1409ec45b: ba2b000000 mov edx, 0x2b
1409ec460: e8fbd2ee01 call 0x1428d9760 ; core::result::unwrap_failed
1409ec465: e96c020000 jmp 0x1409ec6d6
1409ec46a: 4c8d05976a0e02 lea r8, [rip + 0x20e6a97]
1409ec471: 31c9 xor ecx, ecx
1409ec473: 31d2 xor edx, edx
1409ec475: e89ed0ee01 call 0x1428d9518 ; core::panicking::panic_bounds_check
1409ec47a: e957020000 jmp 0x1409ec6d6
1409ec47f: 4531f6 xor r14d, r14d
1409ec482: 488d75a8 lea rsi, [rbp - 0x58]
1409ec486: 488d7d30 lea rdi, [rbp + 0x30]
1409ec48a: 488d9df0000000 lea rbx, [rbp + 0xf0]
1409ec491: 4c8d3d08920d02 lea r15, [rip + 0x20d9208]
1409ec498: 31c0 xor eax, eax
1409ec49a: 4c89ad98000000 mov qword ptr [rbp + 0x98], r13
1409ec4a1: eb1d jmp 0x1409ec4c0
1409ec4a3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ec4b0: 488b85d8000000 mov rax, qword ptr [rbp + 0xd8]
1409ec4b7: 4c39e8 cmp rax, r13
1409ec4ba: 0f844affffff je 0x1409ec40a
1409ec4c0: 488d0c40 lea rcx, [rax + rax*2]
1409ec4c4: 48c1e104 shl rcx, 4
1409ec4c8: 488b95e8000000 mov rdx, qword ptr [rbp + 0xe8]
1409ec4cf: 4c8d040a lea r8, [rdx + rcx]
1409ec4d3: 48ffc0 inc rax
1409ec4d6: 488985d8000000 mov qword ptr [rbp + 0xd8], rax
1409ec4dd: 488b440a08 mov rax, qword ptr [rdx + rcx + 8]
1409ec4e2: 48833c0a00 cmp qword ptr [rdx + rcx], 0
1409ec4e7: 48c7040a00000000 mov qword ptr [rdx + rcx], 0
1409ec4ef: 0f8434010000 je 0x1409ec629
1409ec4f5: 48894530 mov qword ptr [rbp + 0x30], rax
1409ec4f9: 4c8985e0000000 mov qword ptr [rbp + 0xe0], r8
1409ec500: 450fb64028 movzx r8d, byte ptr [r8 + 0x28]
1409ec505: 4180f802 cmp r8b, 2
1409ec509: 0f84f1000000 je 0x1409ec600
1409ec50f: 4180e001 and r8b, 1
1409ec513: 4889d9 mov rcx, rbx
1409ec516: 4889fa mov rdx, rdi
1409ec519: e822afa400 call 0x141437440 ; libfmod::EventInstance::stop
1409ec51e: 488b95f0000000 mov rdx, qword ptr [rbp + 0xf0]
1409ec525: 48b80400000000000080 movabs rax, 0x8000000000000004
1409ec52f: 4839c2 cmp rdx, rax
1409ec532: 0f84c8000000 je 0x1409ec600
1409ec538: 48b80100000000000080 movabs rax, 0x8000000000000001
1409ec542: 4801d0 add rax, rdx
1409ec545: 48b90400000000000080 movabs rcx, 0x8000000000000004
1409ec54f: 4839ca cmp rdx, rcx
1409ec552: 490f4dc6 cmovge rax, r14
1409ec556: 4883f803 cmp rax, 3
1409ec55a: 0f87a0000000 ja 0x1409ec600
1409ec560: 49630487 movsxd rax, dword ptr [r15 + rax*4]
1409ec564: 4c01f8 add rax, r15
1409ec567: ffe0 jmp rax
1409ec569: 4885d2 test rdx, rdx
1409ec56c: 7412 je 0x1409ec580
1409ec56e: 488b8df8000000 mov rcx, qword ptr [rbp + 0xf8]
1409ec575: 41b801000000 mov r8d, 1
1409ec57b: e8a076c2ff call 0x140613c20 ; 
1409ec580: 488b9508010000 mov rdx, qword ptr [rbp + 0x108]
1409ec587: 4885d2 test rdx, rdx
1409ec58a: 7474 je 0x1409ec600
1409ec58c: 488b8d10010000 mov rcx, qword ptr [rbp + 0x110]
1409ec593: eb5e jmp 0x1409ec5f3
1409ec595: 488b8df8000000 mov rcx, qword ptr [rbp + 0xf8]
1409ec59c: 488b9500010000 mov rdx, qword ptr [rbp + 0x100]
1409ec5a3: c60100 mov byte ptr [rcx], 0
1409ec5a6: 4885d2 test rdx, rdx
1409ec5a9: 7548 jne 0x1409ec5f3
1409ec5ab: eb53 jmp 0x1409ec600
1409ec5ad: 488b95f8000000 mov rdx, qword ptr [rbp + 0xf8]
1409ec5b4: 4885d2 test rdx, rdx
1409ec5b7: 7447 je 0x1409ec600
1409ec5b9: 488b8d00010000 mov rcx, qword ptr [rbp + 0x100]
1409ec5c0: eb31 jmp 0x1409ec5f3
1409ec5c2: 488b95f8000000 mov rdx, qword ptr [rbp + 0xf8]
1409ec5c9: 4885d2 test rdx, rdx
1409ec5cc: 7412 je 0x1409ec5e0
1409ec5ce: 488b8d00010000 mov rcx, qword ptr [rbp + 0x100]
1409ec5d5: 41b801000000 mov r8d, 1
1409ec5db: e84076c2ff call 0x140613c20 ; 
1409ec5e0: 488b9510010000 mov rdx, qword ptr [rbp + 0x110]
1409ec5e7: 4885d2 test rdx, rdx
1409ec5ea: 7414 je 0x1409ec600
1409ec5ec: 488b8d18010000 mov rcx, qword ptr [rbp + 0x118]
1409ec5f3: 41b801000000 mov r8d, 1
1409ec5f9: e82276c2ff call 0x140613c20 ; 
1409ec5fe: 6690 nop 
1409ec600: 4889f1 mov rcx, rsi
1409ec603: 4889fa mov rdx, rdi
1409ec606: e8e5b2a400 call 0x1414378f0 ; libfmod::EventInstance::release
1409ec60b: 48b80400000000000080 movabs rax, 0x8000000000000004
1409ec615: 483945a8 cmp qword ptr [rbp - 0x58], rax
1409ec619: 752f jne 0x1409ec64a
1409ec61b: 4c8bad98000000 mov r13, qword ptr [rbp + 0x98]
1409ec622: 4c8b85e0000000 mov r8, qword ptr [rbp + 0xe0]
1409ec629: 498b5010 mov rdx, qword ptr [r8 + 0x10]
1409ec62d: 4885d2 test rdx, rdx
1409ec630: 0f847afeffff je 0x1409ec4b0
1409ec636: 498b4818 mov rcx, qword ptr [r8 + 0x18]
1409ec63a: 41b801000000 mov r8d, 1
1409ec640: e8db75c2ff call 0x140613c20 ; 
1409ec645: e966feffff jmp 0x1409ec4b0
1409ec64a: 488b45d8 mov rax, qword ptr [rbp - 0x28]
1409ec64e: 48898520010000 mov qword ptr [rbp + 0x120], rax
1409ec655: 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
1409ec659: 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
1409ec65d: 0f1055c8 movups xmm2, xmmword ptr [rbp - 0x38]
1409ec661: 0f299510010000 movaps xmmword ptr [rbp + 0x110], xmm2
1409ec668: 0f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm1
1409ec66f: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
1409ec676: 488d05cbc30a02 lea rax, [rip + 0x20ac3cb]
1409ec67d: 4889442420 mov qword ptr [rsp + 0x20], rax
1409ec682: 488d0db7bd0a02 lea rcx, [rip + 0x20abdb7] ; 0x142a98440 'called `Result::unwrap()` on an `Err` value'
1409ec689: 4c8d0de0bd0a02 lea r9, [rip + 0x20abde0]
1409ec690: 4c8d85f0000000 lea r8, [rbp + 0xf0]
1409ec697: ba2b000000 mov edx, 0x2b
1409ec69c: e8bfd0ee01 call 0x1428d9760 ; core::result::unwrap_failed
1409ec6a1: eb33 jmp 0x1409ec6d6
1409ec6a3: 4c8d058e8b0d02 lea r8, [rip + 0x20d8b8e]
1409ec6aa: b908000000 mov ecx, 8
1409ec6af: bae0150000 mov edx, 0x15e0
1409ec6b4: e82ac9ee01 call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1409ec6b9: c6853601000001 mov byte ptr [rbp + 0x136], 1
1409ec6c0: 4c8d05898b0d02 lea r8, [rip + 0x20d8b89]
1409ec6c7: b908000000 mov ecx, 8
1409ec6cc: ba60090000 mov edx, 0x960
1409ec6d1: e80dc9ee01 call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1409ec6d6: 0f0b ud2 
1409ec6d8: 0f1f840000000000 nop dword ptr [rax + rax]
1409ec6e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ec6e5: 55 push rbp
1409ec6e6: 4157 push r15
1409ec6e8: 4156 push r14
1409ec6ea: 4155 push r13
1409ec6ec: 4154 push r12
1409ec6ee: 56 push rsi
1409ec6ef: 57 push rdi
1409ec6f0: 53 push rbx
1409ec6f1: 4881ec88000000 sub rsp, 0x88
1409ec6f8: 488daa80000000 lea rbp, [rdx + 0x80]
1409ec6ff: 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
1409ec705: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
1409ec70b: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
1409ec711: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
1409ec717: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
1409ec71c: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
1409ec721: 80bd3401000000 cmp byte ptr [rbp + 0x134], 0
1409ec728: 751d jne 0x1409ec747
1409ec72a: 4883bdd000000000 cmp qword ptr [rbp + 0xd0], 0
1409ec732: 7413 je 0x1409ec747
1409ec734: c6853701000001 mov byte ptr [rbp + 0x137], 1
1409ec73b: 488d8dd0000000 lea rcx, [rbp + 0xd0]
1409ec742: e8e9043600 call 0x140d4cc30 ; anyhow::error::impl$6::drop
1409ec747: c6853701000001 mov byte ptr [rbp + 0x137], 1
1409ec74e: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
1409ec753: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
1409ec758: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1409ec75e: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1409ec764: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1409ec76a: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
1409ec770: 4881c488000000 add rsp, 0x88
1409ec777: 5b pop rbx
1409ec778: 5f pop rdi
1409ec779: 5e pop rsi
1409ec77a: 415c pop r12
1409ec77c: 415d pop r13
1409ec77e: 415e pop r14
1409ec780: 415f pop r15
1409ec782: 5d pop rbp
1409ec783: c3 ret 
1409ec784: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ec790: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ec795: 55 push rbp
1409ec796: 4157 push r15
1409ec798: 4156 push r14
1409ec79a: 4155 push r13
1409ec79c: 4154 push r12
1409ec79e: 56 push rsi
1409ec79f: 57 push rdi
1409ec7a0: 53 push rbx
1409ec7a1: 4881ec88000000 sub rsp, 0x88
1409ec7a8: 488daa80000000 lea rbp, [rdx + 0x80]
1409ec7af: 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
1409ec7b5: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
1409ec7bb: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
1409ec7c1: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
1409ec7c7: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
1409ec7cc: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
1409ec7d1: 488b85e0000000 mov rax, qword ptr [rbp + 0xe0]
1409ec7d8: 488b5010 mov rdx, qword ptr [rax + 0x10]
1409ec7dc: 4885d2 test rdx, rdx
1409ec7df: 740f je 0x1409ec7f0
1409ec7e1: 488b4818 mov rcx, qword ptr [rax + 0x18]
1409ec7e5: 41b801000000 mov r8d, 1
1409ec7eb: e83074c2ff call 0x140613c20 ; 
1409ec7f0: 4c8bad98000000 mov r13, qword ptr [rbp + 0x98]
1409ec7f7: 488b85d8000000 mov rax, qword ptr [rbp + 0xd8]
1409ec7fe: 4c39e8 cmp rax, r13
1409ec801: 7536 jne 0x1409ec839
1409ec803: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
1409ec808: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
1409ec80d: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1409ec813: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1409ec819: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1409ec81f: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
1409ec825: 4881c488000000 add rsp, 0x88
1409ec82c: 5b pop rbx
1409ec82d: 5f pop rdi
1409ec82e: 5e pop rsi
1409ec82f: 415c pop r12
1409ec831: 415d pop r13
1409ec833: 415e pop r14
1409ec835: 415f pop r15
1409ec837: 5d pop rbp
1409ec838: c3 ret 
1409ec839: 488d75e0 lea rsi, [rbp - 0x20]
1409ec83d: 488d7d38 lea rdi, [rbp + 0x38]
1409ec841: 488d5d40 lea rbx, [rbp + 0x40]
1409ec845: 4c8d3d648e0d02 lea r15, [rip + 0x20d8e64]
1409ec84c: eb0d jmp 0x1409ec85b
1409ec84e: 6690 nop 
1409ec850: 4c89f0 mov rax, r14
1409ec853: 48ffc0 inc rax
1409ec856: 4c39e8 cmp rax, r13
1409ec859: 74a8 je 0x1409ec803
1409ec85b: 4989c6 mov r14, rax
1409ec85e: 488d0c40 lea rcx, [rax + rax*2]
1409ec862: 48c1e104 shl rcx, 4
1409ec866: 488b95e8000000 mov rdx, qword ptr [rbp + 0xe8]
1409ec86d: 4c8d240a lea r12, [rdx + rcx]
1409ec871: 488b440a08 mov rax, qword ptr [rdx + rcx + 8]
1409ec876: 48833c0a00 cmp qword ptr [rdx + rcx], 0
1409ec87b: 48c7040a00000000 mov qword ptr [rdx + rcx], 0
1409ec883: 0f8402010000 je 0x1409ec98b
1409ec889: 48894538 mov qword ptr [rbp + 0x38], rax
1409ec88d: 450fb6442428 movzx r8d, byte ptr [r12 + 0x28]
1409ec893: 4180f802 cmp r8b, 2
1409ec897: 0f84d3000000 je 0x1409ec970
1409ec89d: 4180e001 and r8b, 1
1409ec8a1: 4889d9 mov rcx, rbx
1409ec8a4: 4889fa mov rdx, rdi
1409ec8a7: e894aba400 call 0x141437440 ; libfmod::EventInstance::stop
1409ec8ac: 488b5540 mov rdx, qword ptr [rbp + 0x40]
1409ec8b0: 48b80400000000000080 movabs rax, 0x8000000000000004
1409ec8ba: 4839c2 cmp rdx, rax
1409ec8bd: 0f84ad000000 je 0x1409ec970
1409ec8c3: 48b80100000000000080 movabs rax, 0x8000000000000001
1409ec8cd: 4801d0 add rax, rdx
1409ec8d0: 48b90400000000000080 movabs rcx, 0x8000000000000004
1409ec8da: 4839ca cmp rdx, rcx
1409ec8dd: b900000000 mov ecx, 0
1409ec8e2: 480f4dc1 cmovge rax, rcx
1409ec8e6: 4883f803 cmp rax, 3
1409ec8ea: 0f8780000000 ja 0x1409ec970
1409ec8f0: 49630487 movsxd rax, dword ptr [r15 + rax*4]
1409ec8f4: 4c01f8 add rax, r15
1409ec8f7: ffe0 jmp rax
1409ec8f9: 4885d2 test rdx, rdx
1409ec8fc: 740f je 0x1409ec90d
1409ec8fe: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
1409ec902: 41b801000000 mov r8d, 1
1409ec908: e81373c2ff call 0x140613c20 ; 
1409ec90d: 488b5558 mov rdx, qword ptr [rbp + 0x58]
1409ec911: 4885d2 test rdx, rdx
1409ec914: 745a je 0x1409ec970
1409ec916: 488b4d60 mov rcx, qword ptr [rbp + 0x60]
1409ec91a: eb46 jmp 0x1409ec962
1409ec91c: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
1409ec920: 488b5550 mov rdx, qword ptr [rbp + 0x50]
1409ec924: c60100 mov byte ptr [rcx], 0
1409ec927: 4885d2 test rdx, rdx
1409ec92a: 7536 jne 0x1409ec962
1409ec92c: eb42 jmp 0x1409ec970
1409ec92e: 488b5548 mov rdx, qword ptr [rbp + 0x48]
1409ec932: 4885d2 test rdx, rdx
1409ec935: 7439 je 0x1409ec970
1409ec937: 488b4d50 mov rcx, qword ptr [rbp + 0x50]
1409ec93b: eb25 jmp 0x1409ec962
1409ec93d: 488b5548 mov rdx, qword ptr [rbp + 0x48]
1409ec941: 4885d2 test rdx, rdx
1409ec944: 740f je 0x1409ec955
1409ec946: 488b4d50 mov rcx, qword ptr [rbp + 0x50]
1409ec94a: 41b801000000 mov r8d, 1
1409ec950: e8cb72c2ff call 0x140613c20 ; 
1409ec955: 488b5560 mov rdx, qword ptr [rbp + 0x60]
1409ec959: 4885d2 test rdx, rdx
1409ec95c: 7412 je 0x1409ec970
1409ec95e: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
1409ec962: 41b801000000 mov r8d, 1
1409ec968: e8b372c2ff call 0x140613c20 ; 
1409ec96d: 0f1f00 nop dword ptr [rax]
1409ec970: 4889f1 mov rcx, rsi
1409ec973: 4889fa mov rdx, rdi
1409ec976: e875afa400 call 0x1414378f0 ; libfmod::EventInstance::release
1409ec97b: 48b80400000000000080 movabs rax, 0x8000000000000004
1409ec985: 483945e0 cmp qword ptr [rbp - 0x20], rax
1409ec989: 7523 jne 0x1409ec9ae
1409ec98b: 498b542410 mov rdx, qword ptr [r12 + 0x10]
1409ec990: 4885d2 test rdx, rdx
1409ec993: 0f84b7feffff je 0x1409ec850
1409ec999: 498b4c2418 mov rcx, qword ptr [r12 + 0x18]
1409ec99e: 41b801000000 mov r8d, 1
1409ec9a4: e87772c2ff call 0x140613c20 ; 
1409ec9a9: e9a2feffff jmp 0x1409ec850
1409ec9ae: 488b4510 mov rax, qword ptr [rbp + 0x10]
1409ec9b2: 48894570 mov qword ptr [rbp + 0x70], rax
1409ec9b6: 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
1409ec9ba: 0f104df0 movups xmm1, xmmword ptr [rbp - 0x10]
1409ec9be: 0f105500 movups xmm2, xmmword ptr [rbp]
1409ec9c2: 0f295560 movaps xmmword ptr [rbp + 0x60], xmm2
1409ec9c6: 0f294d50 movaps xmmword ptr [rbp + 0x50], xmm1
1409ec9ca: 0f294540 movaps xmmword ptr [rbp + 0x40], xmm0
1409ec9ce: 488d0573c00a02 lea rax, [rip + 0x20ac073]
1409ec9d5: 4889442420 mov qword ptr [rsp + 0x20], rax
1409ec9da: 488d0d5fba0a02 lea rcx, [rip + 0x20aba5f] ; 0x142a98440 'called `Result::unwrap()` on an `Err` value'
1409ec9e1: 4c8d0d88ba0a02 lea r9, [rip + 0x20aba88]
1409ec9e8: 4c8d4540 lea r8, [rbp + 0x40]
1409ec9ec: ba2b000000 mov edx, 0x2b
1409ec9f1: e86acdee01 call 0x1428d9760 ; core::result::unwrap_failed
1409ec9f6: cc int3 
1409ec9f7: 660f1f840000000000 nop word ptr [rax + rax]
1409eca00: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409eca05: 55 push rbp
1409eca06: 4157 push r15
1409eca08: 4156 push r14
1409eca0a: 4155 push r13
1409eca0c: 4154 push r12
1409eca0e: 56 push rsi
1409eca0f: 57 push rdi
1409eca10: 53 push rbx
1409eca11: 4881ec88000000 sub rsp, 0x88
1409eca18: 488daa80000000 lea rbp, [rdx + 0x80]
1409eca1f: 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
1409eca25: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
1409eca2b: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
1409eca31: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
1409eca37: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
1409eca3c: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
1409eca41: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
1409eca48: 4885c0 test rax, rax
1409eca4b: 741a je 0x1409eca67
1409eca4d: 48c1e004 shl rax, 4
1409eca51: 488d1440 lea rdx, [rax + rax*2]
1409eca55: 41b808000000 mov r8d, 8
1409eca5b: 488b8de8000000 mov rcx, qword ptr [rbp + 0xe8]
1409eca62: e8b971c2ff call 0x140613c20 ; 
1409eca67: c6853401000001 mov byte ptr [rbp + 0x134], 1
1409eca6e: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
1409eca73: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
1409eca78: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1409eca7e: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1409eca84: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1409eca8a: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
1409eca90: 4881c488000000 add rsp, 0x88
1409eca97: 5b pop rbx
1409eca98: 5f pop rdi
1409eca99: 5e pop rsi
1409eca9a: 415c pop r12
1409eca9c: 415d pop r13
1409eca9e: 415e pop r14
1409ecaa0: 415f pop r15
1409ecaa2: 5d pop rbp
1409ecaa3: c3 ret 
1409ecaa4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ecab0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ecab5: 55 push rbp
1409ecab6: 4157 push r15
1409ecab8: 4156 push r14
1409ecaba: 4155 push r13
1409ecabc: 4154 push r12
1409ecabe: 56 push rsi
1409ecabf: 57 push rdi
1409ecac0: 53 push rbx
1409ecac1: 4881ec88000000 sub rsp, 0x88
1409ecac8: 488daa80000000 lea rbp, [rdx + 0x80]
1409ecacf: 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
1409ecad5: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
1409ecadb: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
1409ecae1: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
1409ecae7: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
1409ecaec: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
1409ecaf1: c6853701000001 mov byte ptr [rbp + 0x137], 1
1409ecaf8: 488d4d18 lea rcx, [rbp + 0x18]
1409ecafc: e83fcceaff call 0x140899740 ; core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::audio_resources::AudioInstance,alloc::alloc::Global> > | core::ptr::drop_in_place<country_core::utils::audio_utils::AudioLoop>
1409ecb01: c6853701000001 mov byte ptr [rbp + 0x137], 1
1409ecb08: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
1409ecb0d: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
1409ecb12: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1409ecb18: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1409ecb1e: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1409ecb24: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
1409ecb2a: 4881c488000000 add rsp, 0x88
1409ecb31: 5b pop rbx
1409ecb32: 5f pop rdi
1409ecb33: 5e pop rsi
1409ecb34: 415c pop r12
1409ecb36: 415d pop r13
1409ecb38: 415e pop r14
1409ecb3a: 415f pop r15
1409ecb3c: 5d pop rbp
1409ecb3d: c3 ret 
1409ecb3e: 6690 nop 
1409ecb40: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ecb45: 55 push rbp
1409ecb46: 4157 push r15
1409ecb48: 4156 push r14
1409ecb4a: 4155 push r13
1409ecb4c: 4154 push r12
1409ecb4e: 56 push rsi
1409ecb4f: 57 push rdi
1409ecb50: 53 push rbx
1409ecb51: 4881ec88000000 sub rsp, 0x88
1409ecb58: 488daa80000000 lea rbp, [rdx + 0x80]
1409ecb5f: 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
1409ecb65: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
1409ecb6b: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
1409ecb71: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
1409ecb77: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
1409ecb7c: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
1409ecb81: c6853701000001 mov byte ptr [rbp + 0x137], 1
1409ecb88: 488d8df0000000 lea rcx, [rbp + 0xf0]
1409ecb8f: e89c003600 call 0x140d4cc30 ; anyhow::error::impl$6::drop
1409ecb94: c6853701000001 mov byte ptr [rbp + 0x137], 1
1409ecb9b: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
1409ecba0: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
1409ecba5: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1409ecbab: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1409ecbb1: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1409ecbb7: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
1409ecbbd: 4881c488000000 add rsp, 0x88
1409ecbc4: 5b pop rbx
1409ecbc5: 5f pop rdi
1409ecbc6: 5e pop rsi
1409ecbc7: 415c pop r12
1409ecbc9: 415d pop r13
1409ecbcb: 415e pop r14
1409ecbcd: 415f pop r15
1409ecbcf: 5d pop rbp
1409ecbd0: c3 ret 
1409ecbd1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ecbe0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ecbe5: 55 push rbp
1409ecbe6: 4157 push r15
1409ecbe8: 4156 push r14
1409ecbea: 4155 push r13
1409ecbec: 4154 push r12
1409ecbee: 56 push rsi
1409ecbef: 57 push rdi
1409ecbf0: 53 push rbx
1409ecbf1: 4881ec88000000 sub rsp, 0x88
1409ecbf8: 488daa80000000 lea rbp, [rdx + 0x80]
1409ecbff: 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
1409ecc05: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
1409ecc0b: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
1409ecc11: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
1409ecc17: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
1409ecc1c: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
1409ecc21: c6853701000001 mov byte ptr [rbp + 0x137], 1
1409ecc28: 488d8df0000000 lea rcx, [rbp + 0xf0]
1409ecc2f: e80ccbeaff call 0x140899740 ; core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::audio_resources::AudioInstance,alloc::alloc::Global> > | core::ptr::drop_in_place<country_core::utils::audio_utils::AudioLoop>
1409ecc34: c6853701000001 mov byte ptr [rbp + 0x137], 1
1409ecc3b: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
1409ecc40: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
1409ecc45: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1409ecc4b: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1409ecc51: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1409ecc57: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
1409ecc5d: 4881c488000000 add rsp, 0x88
1409ecc64: 5b pop rbx
1409ecc65: 5f pop rdi
1409ecc66: 5e pop rsi
1409ecc67: 415c pop r12
1409ecc69: 415d pop r13
1409ecc6b: 415e pop r14
1409ecc6d: 415f pop r15
1409ecc6f: 5d pop rbp
1409ecc70: c3 ret 
1409ecc71: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ecc80: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ecc85: 55 push rbp
1409ecc86: 4157 push r15
1409ecc88: 4156 push r14
1409ecc8a: 4155 push r13
1409ecc8c: 4154 push r12
1409ecc8e: 56 push rsi
1409ecc8f: 57 push rdi
1409ecc90: 53 push rbx
1409ecc91: 4881ec88000000 sub rsp, 0x88
1409ecc98: 488daa80000000 lea rbp, [rdx + 0x80]
1409ecc9f: 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
1409ecca5: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
1409eccab: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
1409eccb1: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
1409eccb7: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
1409eccbc: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
1409eccc1: 80bd3701000000 cmp byte ptr [rbp + 0x137], 0
1409eccc8: 7469 je 0x1409ecd33
1409eccca: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
1409eccd1: 4885c0 test rax, rax
1409eccd4: 741a je 0x1409eccf0
1409eccd6: 488b8dc0000000 mov rcx, qword ptr [rbp + 0xc0]
1409eccdd: 48c1e003 shl rax, 3
1409ecce1: 488d1440 lea rdx, [rax + rax*2]
1409ecce5: 41b808000000 mov r8d, 8
1409ecceb: e8306fc2ff call 0x140613c20 ; 
1409eccf0: 0fb68535010000 movzx eax, byte ptr [rbp + 0x135]
1409eccf7: 888536010000 mov byte ptr [rbp + 0x136], al
1409eccfd: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
1409ecd02: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
1409ecd07: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1409ecd0d: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1409ecd13: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1409ecd19: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
1409ecd1f: 4881c488000000 add rsp, 0x88
1409ecd26: 5b pop rbx
1409ecd27: 5f pop rdi
1409ecd28: 5e pop rsi
1409ecd29: 415c pop r12
1409ecd2b: 415d pop r13
1409ecd2d: 415e pop r14
1409ecd2f: 415f pop r15
1409ecd31: 5d pop rbp
1409ecd32: c3 ret 
1409ecd33: 0fb68535010000 movzx eax, byte ptr [rbp + 0x135]
1409ecd3a: 888536010000 mov byte ptr [rbp + 0x136], al
1409ecd40: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
1409ecd45: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
1409ecd4a: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1409ecd50: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1409ecd56: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1409ecd5c: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
1409ecd62: 4881c488000000 add rsp, 0x88
1409ecd69: 5b pop rbx
1409ecd6a: 5f pop rdi
1409ecd6b: 5e pop rsi
1409ecd6c: 415c pop r12
1409ecd6e: 415d pop r13
1409ecd70: 415e pop r14
1409ecd72: 415f pop r15
1409ecd74: 5d pop rbp
1409ecd75: c3 ret 
1409ecd76: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ecd80: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ecd85: 55 push rbp
1409ecd86: 4157 push r15
1409ecd88: 4156 push r14
1409ecd8a: 4155 push r13
1409ecd8c: 4154 push r12
1409ecd8e: 56 push rsi
1409ecd8f: 57 push rdi
1409ecd90: 53 push rbx
1409ecd91: 4881ec88000000 sub rsp, 0x88
1409ecd98: 488daa80000000 lea rbp, [rdx + 0x80]
1409ecd9f: 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
1409ecda5: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
1409ecdab: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
1409ecdb1: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
1409ecdb7: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
1409ecdbc: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
1409ecdc1: 80bd3601000000 cmp byte ptr [rbp + 0x136], 0
1409ecdc8: 7536 jne 0x1409ece00
1409ecdca: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
1409ecdcf: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
1409ecdd4: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1409ecdda: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1409ecde0: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1409ecde6: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
1409ecdec: 4881c488000000 add rsp, 0x88
1409ecdf3: 5b pop rbx
1409ecdf4: 5f pop rdi
1409ecdf5: 5e pop rsi
1409ecdf6: 415c pop r12
1409ecdf8: 415d pop r13
1409ecdfa: 415e pop r14
1409ecdfc: 415f pop r15
1409ecdfe: 5d pop rbp
1409ecdff: c3 ret 
1409ece00: 488d8da0000000 lea rcx, [rbp + 0xa0]
1409ece07: e8d486eaff call 0x1408954e0 ; core::ptr::drop_in_place<alloc::vec::Vec<country_core::utils::audio_utils::AudioLoop,alloc::alloc::Global> > | core::ptr::drop_in_place<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> | bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>
1409ece0c: ebbc jmp 0x1409ecdca
1409ece0e: 6690 nop 
1409ece10: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ece15: 55 push rbp
1409ece16: 4157 push r15
1409ece18: 4156 push r14
1409ece1a: 4155 push r13
1409ece1c: 4154 push r12
1409ece1e: 56 push rsi
1409ece1f: 57 push rdi
1409ece20: 53 push rbx
1409ece21: 4881ec88000000 sub rsp, 0x88
1409ece28: 488daa80000000 lea rbp, [rdx + 0x80]
1409ece2f: 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
1409ece35: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
1409ece3b: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
1409ece41: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
1409ece47: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
1409ece4c: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
1409ece51: 488d8df0000000 lea rcx, [rbp + 0xf0]
1409ece58: e8d39ce9ff call 0x140886b30 ; core::ptr::drop_in_place<enum2$<libfmod::Error> >
1409ece5d: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
1409ece62: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
1409ece67: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1409ece6d: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1409ece73: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1409ece79: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
1409ece7f: 4881c488000000 add rsp, 0x88
1409ece86: 5b pop rbx
1409ece87: 5f pop rdi
1409ece88: 5e pop rsi
1409ece89: 415c pop r12
1409ece8b: 415d pop r13
1409ece8d: 415e pop r14
1409ece8f: 415f pop r15
1409ece91: 5d pop rbp
1409ece92: c3 ret 

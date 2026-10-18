1409ecea0: 55 push rbp
1409ecea1: 4157 push r15
1409ecea3: 4156 push r14
1409ecea5: 4155 push r13
1409ecea7: 4154 push r12
1409ecea9: 56 push rsi
1409eceaa: 57 push rdi
1409eceab: 53 push rbx
1409eceac: 4881eca8010000 sub rsp, 0x1a8
1409eceb3: 488dac2480000000 lea rbp, [rsp + 0x80]
1409ecebb: 440f29bd10010000 movaps xmmword ptr [rbp + 0x110], xmm15
1409ecec3: 440f29b500010000 movaps xmmword ptr [rbp + 0x100], xmm14
1409ececb: 440f29adf0000000 movaps xmmword ptr [rbp + 0xf0], xmm13
1409eced3: 440f29a5e0000000 movaps xmmword ptr [rbp + 0xe0], xmm12
1409ecedb: 440f299dd0000000 movaps xmmword ptr [rbp + 0xd0], xmm11
1409ecee3: 440f2995c0000000 movaps xmmword ptr [rbp + 0xc0], xmm10
1409eceeb: 440f298db0000000 movaps xmmword ptr [rbp + 0xb0], xmm9
1409ecef3: 440f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm8
1409ecefb: 0f29bd90000000 movaps xmmword ptr [rbp + 0x90], xmm7
1409ecf02: 0f29b580000000 movaps xmmword ptr [rbp + 0x80], xmm6
1409ecf09: 48c74578feffffff mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
1409ecf11: 803d787ec40200 cmp byte ptr [rip + 0x2c47e78], 0
1409ecf18: 0f85c8040000 jne 0x1409ed3e6
1409ecf1e: 4c89c3 mov rbx, r8
1409ecf21: 4889d7 mov rdi, rdx
1409ecf24: 4889ce mov rsi, rcx
1409ecf27: e8d49a2200 call 0x140c16a00 ; puffin::are_scopes_on
1409ecf2c: 84c0 test al, al
1409ecf2e: 884577 mov byte ptr [rbp + 0x77], al
1409ecf31: 7460 je 0x1409ecf93
1409ecf33: 4c8d3d197f0d02 lea r15, [rip + 0x20d7f19] ; 0x142ac4e53 'country_core::systems::audio::water_ambience_audio::water_ambience_audio_system::fcountry_core::systems::audio::trigger_audio::t'
1409ecf3a: 41be4f000000 mov r14d, 0x4f
1409ecf40: ba4f000000 mov edx, 0x4f
1409ecf45: 4c89f9 mov rcx, r15
1409ecf48: e8d3c2ffff call 0x1409e9220 ; core::str::impl$0::rfind<ref$<str$> >
1409ecf4d: 4883f801 cmp rax, 1
1409ecf51: 0f85b2000000 jne 0x1409ed009
1409ecf57: 4989d1 mov r9, rdx
1409ecf5a: 4885d2 test rdx, rdx
1409ecf5d: 743d je 0x1409ecf9c
1409ecf5f: 4983f94f cmp r9, 0x4f
1409ecf63: 7335 jae 0x1409ecf9a
1409ecf65: 488d05e77e0d02 lea rax, [rip + 0x20d7ee7] ; 0x142ac4e53 'country_core::systems::audio::water_ambience_audio::water_ambience_audio_system::fcountry_core::systems::audio::trigger_audio::t'
1409ecf6c: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
1409ecf71: 7f29 jg 0x1409ecf9c
1409ecf73: 488d0546800d02 lea rax, [rip + 0x20d8046]
1409ecf7a: 4889442420 mov qword ptr [rsp + 0x20], rax
1409ecf7f: 488d0dcd7e0d02 lea rcx, [rip + 0x20d7ecd] ; 0x142ac4e53 'country_core::systems::audio::water_ambience_audio::water_ambience_audio_system::fcountry_core::systems::audio::trigger_audio::t'
1409ecf86: ba4f000000 mov edx, 0x4f
1409ecf8b: 4531c0 xor r8d, r8d
1409ecf8e: e8adc9ee01 call 0x1428d9940 ; core::str::slice_error_fail
1409ecf93: 31c0 xor eax, eax
1409ecf95: e9d0010000 jmp 0x1409ed16a
1409ecf9a: 75d7 jne 0x1409ecf73
1409ecf9c: 4c8d3db07e0d02 lea r15, [rip + 0x20d7eb0] ; 0x142ac4e53 'country_core::systems::audio::water_ambience_audio::water_ambience_audio_system::fcountry_core::systems::audio::trigger_audio::t'
1409ecfa3: 4c89f9 mov rcx, r15
1409ecfa6: 4c89ca mov rdx, r9
1409ecfa9: e872c2ffff call 0x1409e9220 ; core::str::impl$0::rfind<ref$<str$> >
1409ecfae: 4883f801 cmp rax, 1
1409ecfb2: 7555 jne 0x1409ed009
1409ecfb4: 4989d0 mov r8, rdx
1409ecfb7: 4983c002 add r8, 2
1409ecfbb: 7439 je 0x1409ecff6
1409ecfbd: 4983f84f cmp r8, 0x4f
1409ecfc1: 7331 jae 0x1409ecff4
1409ecfc3: 488d05897e0d02 lea rax, [rip + 0x20d7e89] ; 0x142ac4e53 'country_core::systems::audio::water_ambience_audio::water_ambience_audio_system::fcountry_core::systems::audio::trigger_audio::t'
1409ecfca: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
1409ecfcf: 7f25 jg 0x1409ecff6
1409ecfd1: 488d0500800d02 lea rax, [rip + 0x20d8000]
1409ecfd8: 4889442420 mov qword ptr [rsp + 0x20], rax
1409ecfdd: 488d0d6f7e0d02 lea rcx, [rip + 0x20d7e6f] ; 0x142ac4e53 'country_core::systems::audio::water_ambience_audio::water_ambience_audio_system::fcountry_core::systems::audio::trigger_audio::t'
1409ecfe4: ba4f000000 mov edx, 0x4f
1409ecfe9: 41b94f000000 mov r9d, 0x4f
1409ecfef: e84cc9ee01 call 0x1428d9940 ; core::str::slice_error_fail
1409ecff4: 75db jne 0x1409ecfd1
1409ecff6: 41be4d000000 mov r14d, 0x4d
1409ecffc: 4929d6 sub r14, rdx
1409ecfff: 4c8d3d4d7e0d02 lea r15, [rip + 0x20d7e4d] ; 0x142ac4e53 'country_core::systems::audio::water_ambience_audio::water_ambience_audio_system::fcountry_core::systems::audio::trigger_audio::t'
1409ed006: 4d01c7 add r15, r8
1409ed009: 488d050e820d02 lea rax, [rip + 0x20d820e]
1409ed010: 4c8d25ca810d02 lea r12, [rip + 0x20d81ca] ; 0x142ac51e1 'crates/country-core/src/systems/audio/water_ambience_audio.rs'
1409ed017: 660f1f840000000000 nop word ptr [rax + rax]
1409ed020: 0fbe48ff movsx ecx, byte ptr [rax - 1]
1409ed024: 85c9 test ecx, ecx
1409ed026: 7818 js 0x1409ed040
1409ed028: 48ffc8 dec rax
1409ed02b: 83f95c cmp ecx, 0x5c
1409ed02e: 7569 jne 0x1409ed099
1409ed030: eb7d jmp 0x1409ed0af
1409ed032: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ed040: 0fb650fe movzx edx, byte ptr [rax - 2]
1409ed044: 80fac0 cmp dl, 0xc0
1409ed047: 7d25 jge 0x1409ed06e
1409ed049: 440fb640fd movzx r8d, byte ptr [rax - 3]
1409ed04e: 4180f8c0 cmp r8b, 0xc0
1409ed052: 7d23 jge 0x1409ed077
1409ed054: 440fb648fc movzx r9d, byte ptr [rax - 4]
1409ed059: 4883c0fc add rax, -4
1409ed05d: 4183e107 and r9d, 7
1409ed061: 41c1e106 shl r9d, 6
1409ed065: 4183e03f and r8d, 0x3f
1409ed069: 4509c8 or r8d, r9d
1409ed06c: eb11 jmp 0x1409ed07f
1409ed06e: 4883c0fe add rax, -2
1409ed072: 83e21f and edx, 0x1f
1409ed075: eb12 jmp 0x1409ed089
1409ed077: 4883c0fd add rax, -3
1409ed07b: 4183e00f and r8d, 0xf
1409ed07f: 41c1e006 shl r8d, 6
1409ed083: 83e23f and edx, 0x3f
1409ed086: 4409c2 or edx, r8d
1409ed089: c1e206 shl edx, 6
1409ed08c: 80e13f and cl, 0x3f
1409ed08f: 0fb6c9 movzx ecx, cl
1409ed092: 09d1 or ecx, edx
1409ed094: 83f95c cmp ecx, 0x5c
1409ed097: 7416 je 0x1409ed0af
1409ed099: 83f92f cmp ecx, 0x2f
1409ed09c: 7411 je 0x1409ed0af
1409ed09e: 4c39e0 cmp rax, r12
1409ed0a1: 0f8579ffffff jne 0x1409ed020
1409ed0a7: 41bd3d000000 mov r13d, 0x3d
1409ed0ad: eb50 jmp 0x1409ed0ff
1409ed0af: 4c8d252b810d02 lea r12, [rip + 0x20d812b] ; 0x142ac51e1 'crates/country-core/src/systems/audio/water_ambience_audio.rs'
1409ed0b6: 4c29e0 sub rax, r12
1409ed0b9: 4989c0 mov r8, rax
1409ed0bc: 49ffc0 inc r8
1409ed0bf: 7432 je 0x1409ed0f3
1409ed0c1: 4983f83d cmp r8, 0x3d
1409ed0c5: 732a jae 0x1409ed0f1
1409ed0c7: 43803c20bf cmp byte ptr [r8 + r12], 0xbf
1409ed0cc: 7f25 jg 0x1409ed0f3
1409ed0ce: 488d05cb7e0d02 lea rax, [rip + 0x20d7ecb]
1409ed0d5: 4889442420 mov qword ptr [rsp + 0x20], rax
1409ed0da: 488d0d00810d02 lea rcx, [rip + 0x20d8100] ; 0x142ac51e1 'crates/country-core/src/systems/audio/water_ambience_audio.rs'
1409ed0e1: ba3d000000 mov edx, 0x3d
1409ed0e6: 41b93d000000 mov r9d, 0x3d
1409ed0ec: e84fc8ee01 call 0x1428d9940 ; core::str::slice_error_fail
1409ed0f1: 75db jne 0x1409ed0ce
1409ed0f3: 41bd3c000000 mov r13d, 0x3c
1409ed0f9: 4929c5 sub r13, rax
1409ed0fc: 4d01c4 add r12, r8
1409ed0ff: e82cb02200 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1409ed104: 4889c1 mov rcx, rax
1409ed107: 488b00 mov rax, qword ptr [rax]
1409ed10a: 4883f801 cmp rax, 1
1409ed10e: 0f8534030000 jne 0x1409ed448
1409ed114: 4883c108 add rcx, 8
1409ed118: 48833900 cmp qword ptr [rcx], 0
1409ed11c: 0f8564030000 jne 0x1409ed486
1409ed122: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1409ed129: 48894d68 mov qword ptr [rbp + 0x68], rcx
1409ed12d: 4883c108 add rcx, 8
1409ed131: 4c896c2420 mov qword ptr [rsp + 0x20], r13
1409ed136: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
1409ed13f: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
1409ed148: 4c89fa mov rdx, r15
1409ed14b: 4d89f0 mov r8, r14
1409ed14e: 4d89e1 mov r9, r12
1409ed151: e8ea9b2200 call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
1409ed156: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
1409ed15a: 48ff01 inc qword ptr [rcx]
1409ed15d: 48894548 mov qword ptr [rbp + 0x48], rax
1409ed161: 48894530 mov qword ptr [rbp + 0x30], rax
1409ed165: b801000000 mov eax, 1
1409ed16a: 48894528 mov qword ptr [rbp + 0x28], rax
1409ed16e: 488b13 mov rdx, qword ptr [rbx]
1409ed171: 488d4d5c lea rcx, [rbp + 0x5c]
1409ed175: e8c69b2a00 call 0x140c96d40 ; utils::camera::CameraMatrices::eye_position
1409ed17a: 488b07 mov rax, qword ptr [rdi]
1409ed17d: 4c8b6008 mov r12, qword ptr [rax + 8]
1409ed181: 4c8b6810 mov r13, qword ptr [rax + 0x10]
1409ed185: 8b461c mov eax, dword ptr [rsi + 0x1c]
1409ed188: 488b4e10 mov rcx, qword ptr [rsi + 0x10]
1409ed18c: 8901 mov dword ptr [rcx], eax
1409ed18e: 488b06 mov rax, qword ptr [rsi]
1409ed191: 4c8b7008 mov r14, qword ptr [rax + 8]
1409ed195: 488b4010 mov rax, qword ptr [rax + 0x10]
1409ed199: 4c39e8 cmp rax, r13
1409ed19c: 4c0f42e8 cmovb r13, rax
1409ed1a0: f30f10455c movss xmm0, dword ptr [rbp + 0x5c]
1409ed1a5: f30f114568 movss dword ptr [rbp + 0x68], xmm0
1409ed1aa: f30f107d60 movss xmm7, dword ptr [rbp + 0x60]
1409ed1af: 31c9 xor ecx, ecx
1409ed1b1: f3440f104564 movss xmm8, dword ptr [rbp + 0x64]
1409ed1b7: 48bb3333333333333333 movabs rbx, 0x3333333333333333
1409ed1c1: f3440f100d3a87f301 movss xmm9, dword ptr [rip + 0x1f3873a]
1409ed1ca: 488d7550 lea rsi, [rbp + 0x50]
1409ed1ce: f3440f101d297b0d02 movss xmm11, dword ptr [rip + 0x20d7b29]
1409ed1d7: f3440f1025407b0d02 movss xmm12, dword ptr [rip + 0x20d7b40]
1409ed1e0: eb19 jmp 0x1409ed1fb
1409ed1e2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ed1f0: 4c89f9 mov rcx, r15
1409ed1f3: e838c20600 call 0x140a59430 ; country_core::utils::audio_utils::AudioLoop::stop
1409ed1f8: 4889f9 mov rcx, rdi
1409ed1fb: 4c39e9 cmp rcx, r13
1409ed1fe: 0f839a010000 jae 0x1409ed39e
1409ed204: 488d7901 lea rdi, [rcx + 1]
1409ed208: 488d0449 lea rax, [rcx + rcx*2]
1409ed20c: 4c6bf938 imul r15, rcx, 0x38
1409ed210: 4d01f7 add r15, r14
1409ed213: 498b4cc408 mov rcx, qword ptr [r12 + rax*8 + 8]
1409ed218: 4885c9 test rcx, rcx
1409ed21b: 74d3 je 0x1409ed1f0
1409ed21d: 498d04c4 lea rax, [r12 + rax*8]
1409ed221: 4889ca mov rdx, rcx
1409ed224: 48d1ea shr rdx, 1
1409ed227: 49b85555555555555555 movabs r8, 0x5555555555555555
1409ed231: 4c21c2 and rdx, r8
1409ed234: 4829d1 sub rcx, rdx
1409ed237: 4889ca mov rdx, rcx
1409ed23a: 4821da and rdx, rbx
1409ed23d: 48c1e902 shr rcx, 2
1409ed241: 4821d9 and rcx, rbx
1409ed244: 4801d1 add rcx, rdx
1409ed247: 4889ca mov rdx, rcx
1409ed24a: 48c1ea04 shr rdx, 4
1409ed24e: 4801ca add rdx, rcx
1409ed251: 48b90f0f0f0f0f0f0f0f movabs rcx, 0xf0f0f0f0f0f0f0f
1409ed25b: 4821ca and rdx, rcx
1409ed25e: 48b90101010101010101 movabs rcx, 0x101010101010101
1409ed268: 480fafd1 imul rdx, rcx
1409ed26c: 48c1ea38 shr rdx, 0x38
1409ed270: 450f57ed xorps xmm13, xmm13
1409ed274: f3440f2aea cvtsi2ss xmm13, edx
1409ed279: 410f28c1 movaps xmm0, xmm9
1409ed27d: f3410f5ec5 divss xmm0, xmm13
1409ed282: f30f7e4810 movq xmm1, qword ptr [rax + 0x10]
1409ed287: 660f61c9 punpcklwd xmm1, xmm1
1409ed28b: 660f72e110 psrad xmm1, 0x10
1409ed290: 0f5bc9 cvtdq2ps xmm1, xmm1
1409ed293: f30f12c0 movsldup xmm0, xmm0
1409ed297: 0f59c1 mulps xmm0, xmm1
1409ed29a: 0f59056f7a0d02 mulps xmm0, xmmword ptr [rip + 0x20d7a6f]
1409ed2a1: f20f1008 movsd xmm1, qword ptr [rax]
1409ed2a5: 0f58c8 addps xmm1, xmm0
1409ed2a8: 0f134df0 movlps qword ptr [rbp - 0x10], xmm1
1409ed2ac: 4889f1 mov rcx, rsi
1409ed2af: 488d55f0 lea rdx, [rbp - 0x10]
1409ed2b3: e808fb2a00 call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
1409ed2b8: f3450f59eb mulss xmm13, xmm11
1409ed2bd: 450f28f5 movaps xmm14, xmm13
1409ed2c1: f3450f59f4 mulss xmm14, xmm12
1409ed2c6: f30f107550 movss xmm6, dword ptr [rbp + 0x50]
1409ed2cb: f3440f105554 movss xmm10, dword ptr [rbp + 0x54]
1409ed2d1: f3440f107d58 movss xmm15, dword ptr [rbp + 0x58]
1409ed2d7: 4c89f9 mov rcx, r15
1409ed2da: e8d1c30600 call 0x140a596b0 ; country_core::utils::audio_utils::AudioLoop::start_if_not_playing
1409ed2df: 498b5710 mov rdx, qword ptr [r15 + 0x10]
1409ed2e3: 498b4f28 mov rcx, qword ptr [r15 + 0x28]
1409ed2e7: 4839d1 cmp rcx, rdx
1409ed2ea: 0f83b0010000 jae 0x1409ed4a0
1409ed2f0: 488d0c49 lea rcx, [rcx + rcx*2]
1409ed2f4: 48c1e104 shl rcx, 4
1409ed2f8: 49034f08 add rcx, qword ptr [r15 + 8]
1409ed2fc: 4889f2 mov rdx, rsi
1409ed2ff: e89caa0600 call 0x140a57da0 ; country_core::resources::audio_resources::impl$10::set_position
1409ed304: 498b5710 mov rdx, qword ptr [r15 + 0x10]
1409ed308: 498b4f28 mov rcx, qword ptr [r15 + 0x28]
1409ed30c: 4839d1 cmp rcx, rdx
1409ed30f: 0f837d010000 jae 0x1409ed492
1409ed315: 498b5708 mov rdx, qword ptr [r15 + 8]
1409ed319: 488d0449 lea rax, [rcx + rcx*2]
1409ed31d: 48c1e004 shl rax, 4
1409ed321: 833c0201 cmp dword ptr [rdx + rax], 1
1409ed325: 4889f9 mov rcx, rdi
1409ed328: 0f85cdfeffff jne 0x1409ed1fb
1409ed32e: f30f5c7568 subss xmm6, dword ptr [rbp + 0x68]
1409ed333: f3440f5cd7 subss xmm10, xmm7
1409ed338: f3450f5cf8 subss xmm15, xmm8
1409ed33d: f30f59f6 mulss xmm6, xmm6
1409ed341: f3450f59d2 mulss xmm10, xmm10
1409ed346: f3440f58d6 addss xmm10, xmm6
1409ed34b: f3450f59ff mulss xmm15, xmm15
1409ed350: f3450f58fa addss xmm15, xmm10
1409ed355: f3450f5efe divss xmm15, xmm14
1409ed35a: f3450f58f9 addss xmm15, xmm9
1409ed35f: 410f28c1 movaps xmm0, xmm9
1409ed363: f3410f5ec7 divss xmm0, xmm15
1409ed368: f3440f59e8 mulss xmm13, xmm0
1409ed36d: f3410f51d5 sqrtss xmm2, xmm13
1409ed372: 4801c2 add rdx, rax
1409ed375: 4883c208 add rdx, 8
1409ed379: 488d4db8 lea rcx, [rbp - 0x48]
1409ed37d: e8be97a400 call 0x141436b40 ; libfmod::EventInstance::set_volume
1409ed382: 48b80400000000000080 movabs rax, 0x8000000000000004
1409ed38c: 483945b8 cmp qword ptr [rbp - 0x48], rax
1409ed390: 0f8518010000 jne 0x1409ed4ae
1409ed396: 4889f9 mov rcx, rdi
1409ed399: e95dfeffff jmp 0x1409ed1fb
1409ed39e: 807d7700 cmp byte ptr [rbp + 0x77], 0
1409ed3a2: 7442 je 0x1409ed3e6
1409ed3a4: e887ad2200 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1409ed3a9: 4889c1 mov rcx, rax
1409ed3ac: 488b00 mov rax, qword ptr [rax]
1409ed3af: 4883f801 cmp rax, 1
1409ed3b3: 0f85a9000000 jne 0x1409ed462
1409ed3b9: 4883c108 add rcx, 8
1409ed3bd: 48833900 cmp qword ptr [rcx], 0
1409ed3c1: 0f85bf000000 jne 0x1409ed486
1409ed3c7: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1409ed3ce: 48894d68 mov qword ptr [rbp + 0x68], rcx
1409ed3d2: 4883c108 add rcx, 8
1409ed3d6: 488b5548 mov rdx, qword ptr [rbp + 0x48]
1409ed3da: e8119d2200 call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
1409ed3df: 488b4568 mov rax, qword ptr [rbp + 0x68]
1409ed3e3: 48ff00 inc qword ptr [rax]
1409ed3e6: 0f28b580000000 movaps xmm6, xmmword ptr [rbp + 0x80]
1409ed3ed: 0f28bd90000000 movaps xmm7, xmmword ptr [rbp + 0x90]
1409ed3f4: 440f2885a0000000 movaps xmm8, xmmword ptr [rbp + 0xa0]
1409ed3fc: 440f288db0000000 movaps xmm9, xmmword ptr [rbp + 0xb0]
1409ed404: 440f2895c0000000 movaps xmm10, xmmword ptr [rbp + 0xc0]
1409ed40c: 440f289dd0000000 movaps xmm11, xmmword ptr [rbp + 0xd0]
1409ed414: 440f28a5e0000000 movaps xmm12, xmmword ptr [rbp + 0xe0]
1409ed41c: 440f28adf0000000 movaps xmm13, xmmword ptr [rbp + 0xf0]
1409ed424: 440f28b500010000 movaps xmm14, xmmword ptr [rbp + 0x100]
1409ed42c: 440f28bd10010000 movaps xmm15, xmmword ptr [rbp + 0x110]
1409ed434: 4881c4a8010000 add rsp, 0x1a8
1409ed43b: 5b pop rbx
1409ed43c: 5f pop rdi
1409ed43d: 5e pop rsi
1409ed43e: 415c pop r12
1409ed440: 415d pop r13
1409ed442: 415e pop r14
1409ed444: 415f pop r15
1409ed446: 5d pop rbp
1409ed447: c3 ret 
1409ed448: 4885c0 test rax, rax
1409ed44b: 752d jne 0x1409ed47a
1409ed44d: 31d2 xor edx, edx
1409ed44f: e87cb3f001 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1409ed454: 4889c1 mov rcx, rax
1409ed457: 4885c0 test rax, rax
1409ed45a: 0f85b8fcffff jne 0x1409ed118
1409ed460: eb18 jmp 0x1409ed47a
1409ed462: 4885c0 test rax, rax
1409ed465: 7513 jne 0x1409ed47a
1409ed467: 31d2 xor edx, edx
1409ed469: e862b3f001 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1409ed46e: 4889c1 mov rcx, rax
1409ed471: 4885c0 test rax, rax
1409ed474: 0f8543ffffff jne 0x1409ed3bd
1409ed47a: 488d0d1fd62402 lea rcx, [rip + 0x224d61f]
1409ed481: e84aacee01 call 0x1428d80d0 ; std::thread::local::panic_access_error
1409ed486: 488d0de3d62402 lea rcx, [rip + 0x224d6e3]
1409ed48d: e8bebdee01 call 0x1428d9250 ; core::cell::panic_already_borrowed
1409ed492: 4c8d058f5a0e02 lea r8, [rip + 0x20e5a8f]
1409ed499: e87ac0ee01 call 0x1428d9518 ; core::panicking::panic_bounds_check
1409ed49e: eb56 jmp 0x1409ed4f6
1409ed4a0: 4c8d05815a0e02 lea r8, [rip + 0x20e5a81]
1409ed4a7: e86cc0ee01 call 0x1428d9518 ; core::panicking::panic_bounds_check
1409ed4ac: eb48 jmp 0x1409ed4f6
1409ed4ae: 488b45e8 mov rax, qword ptr [rbp - 0x18]
1409ed4b2: 48894520 mov qword ptr [rbp + 0x20], rax
1409ed4b6: 0f1045b8 movups xmm0, xmmword ptr [rbp - 0x48]
1409ed4ba: 0f104dc8 movups xmm1, xmmword ptr [rbp - 0x38]
1409ed4be: 0f1055d8 movups xmm2, xmmword ptr [rbp - 0x28]
1409ed4c2: 0f295510 movaps xmmword ptr [rbp + 0x10], xmm2
1409ed4c6: 0f294d00 movaps xmmword ptr [rbp], xmm1
1409ed4ca: 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
1409ed4ce: 488d05ab570e02 lea rax, [rip + 0x20e57ab]
1409ed4d5: 4889442420 mov qword ptr [rsp + 0x20], rax
1409ed4da: 488d0d1f310e02 lea rcx, [rip + 0x20e311f] ; 0x142ad0600 'called `Result::unwrap()` on an `Err` value'
1409ed4e1: 4c8d0dd8300e02 lea r9, [rip + 0x20e30d8]
1409ed4e8: 4c8d45f0 lea r8, [rbp - 0x10]
1409ed4ec: ba2b000000 mov edx, 0x2b
1409ed4f1: e86ac2ee01 call 0x1428d9760 ; core::result::unwrap_failed
1409ed4f6: 0f0b ud2 
1409ed4f8: 0f1f840000000000 nop dword ptr [rax + rax]
1409ed500: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ed505: 55 push rbp
1409ed506: 4157 push r15
1409ed508: 4156 push r14
1409ed50a: 4155 push r13
1409ed50c: 4154 push r12
1409ed50e: 56 push rsi
1409ed50f: 57 push rdi
1409ed510: 53 push rbx
1409ed511: 4881ecd8000000 sub rsp, 0xd8
1409ed518: 488daa80000000 lea rbp, [rdx + 0x80]
1409ed51f: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
1409ed525: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
1409ed52b: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
1409ed531: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
1409ed537: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
1409ed53d: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
1409ed546: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
1409ed54f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
1409ed558: 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
1409ed560: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
1409ed568: 488b4568 mov rax, qword ptr [rbp + 0x68]
1409ed56c: 48ff00 inc qword ptr [rax]
1409ed56f: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
1409ed577: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
1409ed57f: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
1409ed588: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
1409ed591: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
1409ed59a: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
1409ed5a0: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
1409ed5a6: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
1409ed5ac: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
1409ed5b2: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
1409ed5b8: 4881c4d8000000 add rsp, 0xd8
1409ed5bf: 5b pop rbx
1409ed5c0: 5f pop rdi
1409ed5c1: 5e pop rsi
1409ed5c2: 415c pop r12
1409ed5c4: 415d pop r13
1409ed5c6: 415e pop r14
1409ed5c8: 415f pop r15
1409ed5ca: 5d pop rbp
1409ed5cb: c3 ret 
1409ed5cc: 0f1f4000 nop dword ptr [rax]
1409ed5d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ed5d5: 55 push rbp
1409ed5d6: 4157 push r15
1409ed5d8: 4156 push r14
1409ed5da: 4155 push r13
1409ed5dc: 4154 push r12
1409ed5de: 56 push rsi
1409ed5df: 57 push rdi
1409ed5e0: 53 push rbx
1409ed5e1: 4881ecd8000000 sub rsp, 0xd8
1409ed5e8: 488daa80000000 lea rbp, [rdx + 0x80]
1409ed5ef: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
1409ed5f5: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
1409ed5fb: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
1409ed601: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
1409ed607: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
1409ed60d: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
1409ed616: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
1409ed61f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
1409ed628: 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
1409ed630: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
1409ed638: 807d7700 cmp byte ptr [rbp + 0x77], 0
1409ed63c: 7420 je 0x1409ed65e
1409ed63e: 488d4530 lea rax, [rbp + 0x30]
1409ed642: 48894540 mov qword ptr [rbp + 0x40], rax
1409ed646: 488d4540 lea rax, [rbp + 0x40]
1409ed64a: 48894538 mov qword ptr [rbp + 0x38], rax
1409ed64e: 488d0deb780d02 lea rcx, [rip + 0x20d78eb]
1409ed655: 488d5538 lea rdx, [rbp + 0x38]
1409ed659: e8721c9400 call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
1409ed65e: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
1409ed666: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
1409ed66e: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
1409ed677: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
1409ed680: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
1409ed689: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
1409ed68f: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
1409ed695: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
1409ed69b: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
1409ed6a1: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
1409ed6a7: 4881c4d8000000 add rsp, 0xd8
1409ed6ae: 5b pop rbx
1409ed6af: 5f pop rdi
1409ed6b0: 5e pop rsi
1409ed6b1: 415c pop r12
1409ed6b3: 415d pop r13
1409ed6b5: 415e pop r14
1409ed6b7: 415f pop r15
1409ed6b9: 5d pop rbp
1409ed6ba: c3 ret 
1409ed6bb: 0f1f440000 nop dword ptr [rax + rax]
1409ed6c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ed6c5: 55 push rbp
1409ed6c6: 4157 push r15
1409ed6c8: 4156 push r14
1409ed6ca: 4155 push r13
1409ed6cc: 4154 push r12
1409ed6ce: 56 push rsi
1409ed6cf: 57 push rdi
1409ed6d0: 53 push rbx
1409ed6d1: 4881ecd8000000 sub rsp, 0xd8
1409ed6d8: 488daa80000000 lea rbp, [rdx + 0x80]
1409ed6df: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
1409ed6e5: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
1409ed6eb: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
1409ed6f1: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
1409ed6f7: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
1409ed6fd: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
1409ed706: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
1409ed70f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
1409ed718: 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
1409ed720: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
1409ed728: 488b4568 mov rax, qword ptr [rbp + 0x68]
1409ed72c: 48ff00 inc qword ptr [rax]
1409ed72f: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
1409ed737: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
1409ed73f: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
1409ed748: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
1409ed751: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
1409ed75a: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
1409ed760: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
1409ed766: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
1409ed76c: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
1409ed772: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
1409ed778: 4881c4d8000000 add rsp, 0xd8
1409ed77f: 5b pop rbx
1409ed780: 5f pop rdi
1409ed781: 5e pop rsi
1409ed782: 415c pop r12
1409ed784: 415d pop r13
1409ed786: 415e pop r14
1409ed788: 415f pop r15
1409ed78a: 5d pop rbp
1409ed78b: c3 ret 
1409ed78c: 0f1f4000 nop dword ptr [rax]
1409ed790: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ed795: 55 push rbp
1409ed796: 4157 push r15
1409ed798: 4156 push r14
1409ed79a: 4155 push r13
1409ed79c: 4154 push r12
1409ed79e: 56 push rsi
1409ed79f: 57 push rdi
1409ed7a0: 53 push rbx
1409ed7a1: 4881ecd8000000 sub rsp, 0xd8
1409ed7a8: 488daa80000000 lea rbp, [rdx + 0x80]
1409ed7af: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
1409ed7b5: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
1409ed7bb: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
1409ed7c1: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
1409ed7c7: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
1409ed7cd: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
1409ed7d6: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
1409ed7df: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
1409ed7e8: 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
1409ed7f0: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
1409ed7f8: 488d4df0 lea rcx, [rbp - 0x10]
1409ed7fc: e8af4e0600 call 0x140a526b0 ; core::ptr::drop_in_place<enum2$<libfmod::Error> >
1409ed801: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
1409ed809: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
1409ed811: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
1409ed81a: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
1409ed823: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
1409ed82c: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
1409ed832: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
1409ed838: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
1409ed83e: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
1409ed844: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
1409ed84a: 4881c4d8000000 add rsp, 0xd8
1409ed851: 5b pop rbx
1409ed852: 5f pop rdi
1409ed853: 5e pop rsi
1409ed854: 415c pop r12
1409ed856: 415d pop r13
1409ed858: 415e pop r14
1409ed85a: 415f pop r15
1409ed85c: 5d pop rbp
1409ed85d: c3 ret 

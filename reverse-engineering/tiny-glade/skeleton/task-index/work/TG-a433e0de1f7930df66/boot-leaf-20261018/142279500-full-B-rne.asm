142279500: 55 push rbp
142279501: 4157 push r15
142279503: 4156 push r14
142279505: 4155 push r13
142279507: 4154 push r12
142279509: 56 push rsi
14227950a: 57 push rdi
14227950b: 53 push rbx
14227950c: 4881ecb8010000 sub rsp, 0x1b8
142279513: 488dac2480000000 lea rbp, [rsp + 0x80]
14227951b: 440f29a520010000 movaps xmmword ptr [rbp + 0x120], xmm12
142279523: 440f299d10010000 movaps xmmword ptr [rbp + 0x110], xmm11
14227952b: 440f299500010000 movaps xmmword ptr [rbp + 0x100], xmm10
142279533: 440f298df0000000 movaps xmmword ptr [rbp + 0xf0], xmm9
14227953b: 440f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm8
142279543: 0f29bdd0000000 movaps xmmword ptr [rbp + 0xd0], xmm7
14227954a: 0f29b5c0000000 movaps xmmword ptr [rbp + 0xc0], xmm6
142279551: 48c785b8000000feffffff mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
14227955c: 4c89ce mov rsi, r9
14227955f: 4d89c6 mov r14, r8
142279562: 4989d7 mov r15, rdx
142279565: 48898da8000000 mov qword ptr [rbp + 0xa8], rcx
14227956c: 488bbdb0010000 mov rdi, qword ptr [rbp + 0x1b0]
142279573: e888d499fe call 0x140c16a00 ; puffin::are_scopes_on
142279578: 84c0 test al, al
14227957a: 745f je 0x1422795db
14227957c: 4c8d257de9db00 lea r12, [rip + 0xdbe97d] ; @143037f00 'forest::tree_animation::handplaced_tree_animation::f'
142279583: bb31000000 mov ebx, 0x31
142279588: ba31000000 mov edx, 0x31
14227958d: 4c89e1 mov rcx, r12
142279590: e84bfbffff call 0x1422790e0 ; core::str::impl$0::rfind<ref$<str$> >
142279595: 4883f801 cmp rax, 1
142279599: 0f85da000000 jne 0x142279679
14227959f: 4989d1 mov r9, rdx
1422795a2: 4885d2 test rdx, rdx
1422795a5: 7466 je 0x14227960d
1422795a7: 4983f931 cmp r9, 0x31
1422795ab: 735e jae 0x14227960b
1422795ad: 488d054ce9db00 lea rax, [rip + 0xdbe94c] ; @143037f00 'forest::tree_animation::handplaced_tree_animation::f'
1422795b4: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
1422795b9: 7f52 jg 0x14227960d
1422795bb: 488d05f6e9db00 lea rax, [rip + 0xdbe9f6]
1422795c2: 4889442420 mov qword ptr [rsp + 0x20], rax
1422795c7: 488d0d32e9db00 lea rcx, [rip + 0xdbe932] ; @143037f00 'forest::tree_animation::handplaced_tree_animation::f'
1422795ce: ba31000000 mov edx, 0x31
1422795d3: 4531c0 xor r8d, r8d
1422795d6: e865036600 call 0x1428d9940 ; core::str::slice_error_fail
1422795db: 31c0 xor eax, eax
1422795dd: 48894558 mov qword ptr [rbp + 0x58], rax
1422795e1: 498b0e mov rcx, qword ptr [r14]
1422795e4: 8b01 mov eax, dword ptr [rcx]
1422795e6: 8d50fa lea edx, [rax - 6]
1422795e9: 4989c0 mov r8, rax
1422795ec: 4983c0fb add r8, -5
1422795f0: 4531c9 xor r9d, r9d
1422795f3: 83fa0f cmp edx, 0xf
1422795f6: 4d0f42c8 cmovb r9, r8
1422795fa: 488d154feadb00 lea rdx, [rip + 0xdbea4f]
142279601: 4e63048a movsxd r8, dword ptr [rdx + r9*4]
142279605: 4901d0 add r8, rdx
142279608: 41ffe0 jmp r8
14227960b: 75ae jne 0x1422795bb
14227960d: 4c8d25ece8db00 lea r12, [rip + 0xdbe8ec] ; @143037f00 'forest::tree_animation::handplaced_tree_animation::f'
142279614: 4c89e1 mov rcx, r12
142279617: 4c89ca mov rdx, r9
14227961a: e8c1faffff call 0x1422790e0 ; core::str::impl$0::rfind<ref$<str$> >
14227961f: 4883f801 cmp rax, 1
142279623: 7554 jne 0x142279679
142279625: 4989d0 mov r8, rdx
142279628: 4983c002 add r8, 2
14227962c: 7439 je 0x142279667
14227962e: 4983f831 cmp r8, 0x31
142279632: 7331 jae 0x142279665
142279634: 488d05c5e8db00 lea rax, [rip + 0xdbe8c5] ; @143037f00 'forest::tree_animation::handplaced_tree_animation::f'
14227963b: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
142279640: 7f25 jg 0x142279667
142279642: 488d0587e9db00 lea rax, [rip + 0xdbe987]
142279649: 4889442420 mov qword ptr [rsp + 0x20], rax
14227964e: 488d0dabe8db00 lea rcx, [rip + 0xdbe8ab] ; @143037f00 'forest::tree_animation::handplaced_tree_animation::f'
142279655: ba31000000 mov edx, 0x31
14227965a: 41b931000000 mov r9d, 0x31
142279660: e8db026600 call 0x1428d9940 ; core::str::slice_error_fail
142279665: 75db jne 0x142279642
142279667: bb2f000000 mov ebx, 0x2f
14227966c: 4829d3 sub rbx, rdx
14227966f: 4c8d258ae8db00 lea r12, [rip + 0xdbe88a] ; @143037f00 'forest::tree_animation::handplaced_tree_animation::f'
142279676: 4d01c4 add r12, r8
142279679: 4c8d05abe9db00 lea r8, [rip + 0xdbe9ab]
142279680: 4c8d2d79e9db00 lea r13, [rip + 0xdbe979] ; @143038000 'crates/systems/forest/src/tree_animation.rs'
142279687: 4c897d70 mov qword ptr [rbp + 0x70], r15
14227968b: 0f1f440000 nop dword ptr [rax + rax]
142279690: 410fbe40ff movsx eax, byte ptr [r8 - 1]
142279695: 85c0 test eax, eax
142279697: 7817 js 0x1422796b0
142279699: 49ffc8 dec r8
14227969c: 83f85c cmp eax, 0x5c
14227969f: 7563 jne 0x142279704
1422796a1: eb78 jmp 0x14227971b
1422796a3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1422796b0: 410fb648fe movzx ecx, byte ptr [r8 - 2]
1422796b5: 80f9c0 cmp cl, 0xc0
1422796b8: 7d23 jge 0x1422796dd
1422796ba: 410fb650fd movzx edx, byte ptr [r8 - 3]
1422796bf: 80fac0 cmp dl, 0xc0
1422796c2: 7d22 jge 0x1422796e6
1422796c4: 450fb648fc movzx r9d, byte ptr [r8 - 4]
1422796c9: 4983c0fc add r8, -4
1422796cd: 4183e107 and r9d, 7
1422796d1: 41c1e106 shl r9d, 6
1422796d5: 83e23f and edx, 0x3f
1422796d8: 4409ca or edx, r9d
1422796db: eb10 jmp 0x1422796ed
1422796dd: 4983c0fe add r8, -2
1422796e1: 83e11f and ecx, 0x1f
1422796e4: eb0f jmp 0x1422796f5
1422796e6: 4983c0fd add r8, -3
1422796ea: 83e20f and edx, 0xf
1422796ed: c1e206 shl edx, 6
1422796f0: 83e13f and ecx, 0x3f
1422796f3: 09d1 or ecx, edx
1422796f5: c1e106 shl ecx, 6
1422796f8: 243f and al, 0x3f
1422796fa: 0fb6c0 movzx eax, al
1422796fd: 09c8 or eax, ecx
1422796ff: 83f85c cmp eax, 0x5c
142279702: 7417 je 0x14227971b
142279704: 83f82f cmp eax, 0x2f
142279707: 7412 je 0x14227971b
142279709: 4d39e8 cmp r8, r13
14227970c: 7582 jne 0x142279690
14227970e: 4989ff mov r15, rdi
142279711: 4889f7 mov rdi, rsi
142279714: be2b000000 mov esi, 0x2b
142279719: eb52 jmp 0x14227976d
14227971b: 4c8d2ddee8db00 lea r13, [rip + 0xdbe8de] ; @143038000 'crates/systems/forest/src/tree_animation.rs'
142279722: 4d29e8 sub r8, r13
142279725: 49ffc0 inc r8
142279728: 7432 je 0x14227975c
14227972a: 4983f82b cmp r8, 0x2b
14227972e: 732a jae 0x14227975a
142279730: 43803c28bf cmp byte ptr [r8 + r13], 0xbf
142279735: 7f25 jg 0x14227975c
142279737: 488d055ae8db00 lea rax, [rip + 0xdbe85a]
14227973e: 4889442420 mov qword ptr [rsp + 0x20], rax
142279743: 488d0db6e8db00 lea rcx, [rip + 0xdbe8b6] ; @143038000 'crates/systems/forest/src/tree_animation.rs'
14227974a: ba2b000000 mov edx, 0x2b
14227974f: 41b92b000000 mov r9d, 0x2b
142279755: e8e6016600 call 0x1428d9940 ; core::str::slice_error_fail
14227975a: 75db jne 0x142279737
14227975c: 4989ff mov r15, rdi
14227975f: 4889f7 mov rdi, rsi
142279762: be2b000000 mov esi, 0x2b
142279767: 4c29c6 sub rsi, r8
14227976a: 4d01c5 add r13, r8
14227976d: e8bee999fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
142279772: 4889c1 mov rcx, rax
142279775: 488b00 mov rax, qword ptr [rax]
142279778: 4883f801 cmp rax, 1
14227977c: 0f851d070000 jne 0x142279e9f
142279782: 4883c108 add rcx, 8
142279786: 48833900 cmp qword ptr [rcx], 0
14227978a: 0f8533070000 jne 0x142279ec3
142279790: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
142279797: 48898db0000000 mov qword ptr [rbp + 0xb0], rcx
14227979e: 4883c108 add rcx, 8
1422797a2: 4889742420 mov qword ptr [rsp + 0x20], rsi
1422797a7: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
1422797b0: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
1422797b9: 4c89e2 mov rdx, r12
1422797bc: 4989d8 mov r8, rbx
1422797bf: 4d89e9 mov r9, r13
1422797c2: e879d599fe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
1422797c7: 488b8db0000000 mov rcx, qword ptr [rbp + 0xb0]
1422797ce: 48ff01 inc qword ptr [rcx]
1422797d1: 48894560 mov qword ptr [rbp + 0x60], rax
1422797d5: b801000000 mov eax, 1
1422797da: 48894558 mov qword ptr [rbp + 0x58], rax
1422797de: 498b0e mov rcx, qword ptr [r14]
1422797e1: 8b01 mov eax, dword ptr [rcx]
1422797e3: 8d50fa lea edx, [rax - 6]
1422797e6: 4989c0 mov r8, rax
1422797e9: 4983c0fb add r8, -5
1422797ed: 4531c9 xor r9d, r9d
1422797f0: 83fa0f cmp edx, 0xf
1422797f3: 4d0f42c8 cmovb r9, r8
1422797f7: 488d1552e8db00 lea rdx, [rip + 0xdbe852]
1422797fe: 4e63048a movsxd r8, dword ptr [rdx + r9*4]
142279802: 4901d0 add r8, rdx
142279805: 4889fe mov rsi, rdi
142279808: 4c89ff mov rdi, r15
14227980b: 4c8b7d70 mov r15, qword ptr [rbp + 0x70]
14227980f: 41ffe0 jmp r8
142279812: 498b07 mov rax, qword ptr [r15]
142279815: 48894578 mov qword ptr [rbp + 0x78], rax
142279819: f30f104028 movss xmm0, dword ptr [rax + 0x28]
14227981e: 8b471c mov eax, dword ptr [rdi + 0x1c]
142279821: 488b4f10 mov rcx, qword ptr [rdi + 0x10]
142279825: 8901 mov dword ptr [rcx], eax
142279827: 488b17 mov rdx, qword ptr [rdi]
14227982a: f30f5802 addss xmm0, dword ptr [rdx]
14227982e: f30f1102 movss dword ptr [rdx], xmm0
142279832: 4531f6 xor r14d, r14d
142279835: eb38 jmp 0x14227986f
142279837: 83f803 cmp eax, 3
14227983a: 740c je 0x142279848
14227983c: 85c0 test eax, eax
14227983e: 7408 je 0x142279848
142279840: ebd0 jmp 0x142279812
142279842: 80790406 cmp byte ptr [rcx + 4], 6
142279846: 75ca jne 0x142279812
142279848: 498b07 mov rax, qword ptr [r15]
14227984b: 48894578 mov qword ptr [rbp + 0x78], rax
14227984f: f30f104828 movss xmm1, dword ptr [rax + 0x28]
142279854: 8b471c mov eax, dword ptr [rdi + 0x1c]
142279857: 488b4f10 mov rcx, qword ptr [rdi + 0x10]
14227985b: 8901 mov dword ptr [rcx], eax
14227985d: 488b17 mov rdx, qword ptr [rdi]
142279860: f30f1002 movss xmm0, dword ptr [rdx]
142279864: f30f5cc1 subss xmm0, xmm1
142279868: f30f1102 movss dword ptr [rdx], xmm0
14227986c: 41b601 mov r14b, 1
14227986f: 0f57c9 xorps xmm1, xmm1
142279872: f30f5fc8 maxss xmm1, xmm0
142279876: f30f103586c06a00 movss xmm6, dword ptr [rip + 0x6ac086]
14227987e: 0f28c6 movaps xmm0, xmm6
142279881: f30f5dc1 minss xmm0, xmm1
142279885: f30f1102 movss dword ptr [rdx], xmm0
142279889: 0f28c8 movaps xmm1, xmm0
14227988c: f30f59c8 mulss xmm1, xmm0
142279890: f30f58c0 addss xmm0, xmm0
142279894: f30f1015dc377500 movss xmm2, dword ptr [rip + 0x7537dc]
14227989c: f30f5cd0 subss xmm2, xmm0
1422798a0: f30f59d1 mulss xmm2, xmm1
1422798a4: 8901 mov dword ptr [rcx], eax
1422798a6: f30f115204 movss dword ptr [rdx + 4], xmm2
1422798ab: 488b3e mov rdi, qword ptr [rsi]
1422798ae: 488d4d30 lea rcx, [rbp + 0x30]
1422798b2: 4889fa mov rdx, rdi
1422798b5: e886d4a1fe call 0x140c96d40 ; utils::camera::CameraMatrices::eye_position
1422798ba: 488d4d18 lea rcx, [rbp + 0x18]
1422798be: 4889fa mov rdx, rdi
1422798c1: e8bad4a1fe call 0x140c96d80 ; utils::camera::CameraMatrices::eye_direction
1422798c6: 488b85a0010000 mov rax, qword ptr [rbp + 0x1a0]
1422798cd: 488b08 mov rcx, qword ptr [rax]
1422798d0: 31d2 xor edx, edx
1422798d2: e8b93530ff call 0x14157ce90 ; country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary::get_triggers
1422798d7: 488d4df8 lea rcx, [rbp - 8]
1422798db: 4889c2 mov rdx, rax
1422798de: e85d3530ff call 0x14157ce40 ; country_core::asset_libraries::audio_trigger_library::AudioTriggerCollection::boxes
1422798e3: f30f107d20 movss xmm7, dword ptr [rbp + 0x20]
1422798e8: f30f593d88377500 mulss xmm7, dword ptr [rip + 0x753788]
1422798f0: f20f104530 movsd xmm0, qword ptr [rbp + 0x30]
1422798f5: f2440f104518 movsd xmm8, qword ptr [rbp + 0x18]
1422798fb: 440f59053d418b00 mulps xmm8, xmmword ptr [rip + 0x8b413d]
142279903: 440f5905459d8500 mulps xmm8, xmmword ptr [rip + 0x859d45]
14227990b: f30f593dbdbf6a00 mulss xmm7, dword ptr [rip + 0x6abfbd]
142279913: f30f587d38 addss xmm7, dword ptr [rbp + 0x38]
142279918: 440f58c0 addps xmm8, xmm0
14227991c: 488b5df8 mov rbx, qword ptr [rbp - 8]
142279920: 4c8b6500 mov r12, qword ptr [rbp]
142279924: 488b7508 mov rsi, qword ptr [rbp + 8]
142279928: f3440f100db3076e00 movss xmm9, dword ptr [rip + 0x6e07b3]
142279931: f2440f1015864d6b00 movsd xmm10, qword ptr [rip + 0x6b4d86]
14227993a: 440f281dcee4db00 movaps xmm11, xmmword ptr [rip + 0xdbe4ce]
142279942: f2440f1025258a8300 movsd xmm12, qword ptr [rip + 0x838a25]
14227994b: 4c8d7dc8 lea r15, [rbp - 0x38]
14227994f: 4989dd mov r13, rbx
142279952: 4889f2 mov rdx, rsi
142279955: 4c39e3 cmp rbx, r12
142279958: 0f84cb000000 je 0x142279a29
14227995e: 6690 nop 
142279960: 498d5d1c lea rbx, [r13 + 0x1c]
142279964: 418b4d18 mov ecx, dword ptr [r13 + 0x18]
142279968: f6c101 test cl, 1
14227996b: 7513 jne 0x142279980
14227996d: 48ffc2 inc rdx
142279970: 4989dd mov r13, rbx
142279973: 4c39e3 cmp rbx, r12
142279976: 75e8 jne 0x142279960
142279978: e9ac000000 jmp 0x142279a29
14227997d: 0f1f00 nop dword ptr [rax]
142279980: 48895df8 mov qword ptr [rbp - 8], rbx
142279984: 488d7201 lea rsi, [rdx + 1]
142279988: 48897508 mov qword ptr [rbp + 8], rsi
14227998c: 48ff4d10 dec qword ptr [rbp + 0x10]
142279990: e8bb985bfe call 0x140833250 ; country_core::resources::walls::decorator::DecoratorRank::new | country_core::systems::collision::world::impl$61::from | country_core::render::nani::mesh_atlas::MeshAtlasPart::new | country_core::LazyTile::from_index | country_core::resources::walls::decorator_subtype::impl$7::into_decorator_subtype | country_core::resources::history::history_chain::HistoryChainIndex::new | rhapsody::rg::resource::ResourceHandle::raw | country_core::asset_libraries::audio_trigger_library::impl$3::from | font_types::raw::impl$19::from_raw
142279995: 85c0 test eax, eax
142279997: 400f94c7 sete dil
14227999b: 0f848f000000 je 0x142279a30
1422799a1: 410f104500 movups xmm0, xmmword ptr [r13]
1422799a6: 0f294540 movaps xmmword ptr [rbp + 0x40], xmm0
1422799aa: 498b4510 mov rax, qword ptr [r13 + 0x10]
1422799ae: 48894550 mov qword ptr [rbp + 0x50], rax
1422799b2: f30f104540 movss xmm0, dword ptr [rbp + 0x40]
1422799b7: f3410f58c1 addss xmm0, xmm9
1422799bc: f20f104d48 movsd xmm1, qword ptr [rbp + 0x48]
1422799c1: 410f58ca addps xmm1, xmm10
1422799c5: f30f105554 movss xmm2, dword ptr [rbp + 0x54]
1422799ca: f30f58d6 addss xmm2, xmm6
1422799ce: f30f105d44 movss xmm3, dword ptr [rbp + 0x44]
1422799d3: 0f14c3 unpcklps xmm0, xmm3
1422799d6: 0f16c1 movlhps xmm0, xmm1
1422799d9: 410f58c3 addps xmm0, xmm11
1422799dd: 0f294540 movaps xmmword ptr [rbp + 0x40], xmm0
1422799e1: f30f104550 movss xmm0, dword ptr [rbp + 0x50]
1422799e6: 0f14c2 unpcklps xmm0, xmm2
1422799e9: 410f58c4 addps xmm0, xmm12
1422799ed: 0f134550 movlps qword ptr [rbp + 0x50], xmm0
1422799f1: 8b4538 mov eax, dword ptr [rbp + 0x38]
1422799f4: 8945e0 mov dword ptr [rbp - 0x20], eax
1422799f7: 488b4530 mov rax, qword ptr [rbp + 0x30]
1422799fb: 488945d8 mov qword ptr [rbp - 0x28], rax
1422799ff: 440f1345c8 movlps qword ptr [rbp - 0x38], xmm8
142279a04: f30f117dd0 movss dword ptr [rbp - 0x30], xmm7
142279a09: 488d4db8 lea rcx, [rbp - 0x48]
142279a0d: 488d5540 lea rdx, [rbp + 0x40]
142279a11: 4c8d45d8 lea r8, [rbp - 0x28]
142279a15: 4d89f9 mov r9, r15
142279a18: e8536bfcfe call 0x141240570 ; utils::geometry::aabb::Aabb3::intersect_segment
142279a1d: 837db800 cmp dword ptr [rbp - 0x48], 0
142279a21: 0f8428ffffff je 0x14227994f
142279a27: eb07 jmp 0x142279a30
142279a29: 4c896df8 mov qword ptr [rbp - 8], r13
142279a2d: 40b701 mov dil, 1
142279a30: 4489f1 mov ecx, r14d
142279a33: 4020f9 and cl, dil
142279a36: 488b85a8000000 mov rax, qword ptr [rbp + 0xa8]
142279a3d: 488b10 mov rdx, qword ptr [rax]
142279a40: 488b5808 mov rbx, qword ptr [rax + 8]
142279a44: 8b4014 mov eax, dword ptr [rax + 0x14]
142279a47: 8985a8000000 mov dword ptr [rbp + 0xa8], eax
142279a4d: 4c8bb2f8000000 mov r14, qword ptr [rdx + 0xf8]
142279a54: 488b8200010000 mov rax, qword ptr [rdx + 0x100]
142279a5b: 4d8d3c86 lea r15, [r14 + rax*4]
142279a5f: 488995a0000000 mov qword ptr [rbp + 0xa0], rdx
142279a66: 0fb68208010000 movzx eax, byte ptr [rdx + 0x108]
142279a6d: 884570 mov byte ptr [rbp + 0x70], al
142279a70: b808000000 mov eax, 8
142279a75: 48894528 mov qword ptr [rbp + 0x28], rax
142279a79: 48c7859000000000000000 mov qword ptr [rbp + 0x90], 0
142279a84: f30f103d24628000 movss xmm7, dword ptr [rip + 0x806224]
142279a8c: f6d1 not cl
142279a8e: 894d6c mov dword ptr [rbp + 0x6c], ecx
142279a91: 4531ed xor r13d, r13d
142279a94: 4531e4 xor r12d, r12d
142279a97: 48c7859800000000000000 mov qword ptr [rbp + 0x98], 0
142279aa2: c785b000000000000000 mov dword ptr [rbp + 0xb0], 0
142279aac: 807d7000 cmp byte ptr [rbp + 0x70], 0
142279ab0: 746e je 0x142279b20
142279ab2: 4d39e5 cmp r13, r12
142279ab5: 0f854a010000 jne 0x142279c05
142279abb: 0f1f440000 nop dword ptr [rax + rax]
142279ac0: 4d39fe cmp r14, r15
142279ac3: 0f8427030000 je 0x142279df0
142279ac9: 418b0e mov ecx, dword ptr [r14]
142279acc: 4983c604 add r14, 4
142279ad0: 488b83a8010000 mov rax, qword ptr [rbx + 0x1a8]
142279ad7: 488d0cc9 lea rcx, [rcx + rcx*8]
142279adb: 4c8b64c810 mov r12, qword ptr [rax + rcx*8 + 0x10]
142279ae0: 4d85e4 test r12, r12
142279ae3: 74db je 0x142279ac0
142279ae5: 488d0cc8 lea rcx, [rax + rcx*8]
142279ae9: 488b4138 mov rax, qword ptr [rcx + 0x38]
142279aed: 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
142279af4: 488b9220010000 mov rdx, qword ptr [rdx + 0x120]
142279afb: 48395140 cmp qword ptr [rcx + 0x40], rdx
142279aff: 0f868c000000 jbe 0x142279b91
142279b05: 48833cd000 cmp qword ptr [rax + rdx*8], 0
142279b0a: 0f95c2 setne dl
142279b0d: 8995b0000000 mov dword ptr [rbp + 0xb0], edx
142279b13: e983000000 jmp 0x142279b9b
142279b18: 0f1f840000000000 nop dword ptr [rax + rax]
142279b20: 4d39e5 cmp r13, r12
142279b23: 0f8577010000 jne 0x142279ca0
142279b29: 0f1f8000000000 nop dword ptr [rax]
142279b30: 4d39fe cmp r14, r15
142279b33: 0f84b7020000 je 0x142279df0
142279b39: 418b0e mov ecx, dword ptr [r14]
142279b3c: 4983c604 add r14, 4
142279b40: 488b8300010000 mov rax, qword ptr [rbx + 0x100]
142279b47: 488d0c89 lea rcx, [rcx + rcx*4]
142279b4b: 48c1e105 shl rcx, 5
142279b4f: 4c8b640858 mov r12, qword ptr [rax + rcx + 0x58]
142279b54: 4d85e4 test r12, r12
142279b57: 74d7 je 0x142279b30
142279b59: 4801c8 add rax, rcx
142279b5c: 488b8da0000000 mov rcx, qword ptr [rbp + 0xa0]
142279b63: 488b8920010000 mov rcx, qword ptr [rcx + 0x120]
142279b6a: 48398888000000 cmp qword ptr [rax + 0x88], rcx
142279b71: 0f8696000000 jbe 0x142279c0d
142279b77: 488b9080000000 mov rdx, qword ptr [rax + 0x80]
142279b7e: 48833cca00 cmp qword ptr [rdx + rcx*8], 0
142279b83: 0f95c1 setne cl
142279b86: 898db0000000 mov dword ptr [rbp + 0xb0], ecx
142279b8c: e986000000 jmp 0x142279c17
142279b91: c785b000000000000000 mov dword ptr [rbp + 0xb0], 0
142279b9b: 488b4918 mov rcx, qword ptr [rcx + 0x18]
142279b9f: 4c8b85a0000000 mov r8, qword ptr [rbp + 0xa0]
142279ba6: 498b9010010000 mov rdx, qword ptr [r8 + 0x110]
142279bad: 4d8b8018010000 mov r8, qword ptr [r8 + 0x118]
142279bb4: 488b14d0 mov rdx, qword ptr [rax + rdx*8]
142279bb8: 48f7d2 not rdx
142279bbb: 488d1452 lea rdx, [rdx + rdx*2]
142279bbf: 48c1e204 shl rdx, 4
142279bc3: 4c8b4c1110 mov r9, qword ptr [rcx + rdx + 0x10]
142279bc8: 4c898d90000000 mov qword ptr [rbp + 0x90], r9
142279bcf: 488b541128 mov rdx, qword ptr [rcx + rdx + 0x28]
142279bd4: 48899580000000 mov qword ptr [rbp + 0x80], rdx
142279bdb: 4a8b04c0 mov rax, qword ptr [rax + r8*8]
142279bdf: 48f7d0 not rax
142279be2: 488d0440 lea rax, [rax + rax*2]
142279be6: 48c1e004 shl rax, 4
142279bea: 488b540110 mov rdx, qword ptr [rcx + rax + 0x10]
142279bef: 48899598000000 mov qword ptr [rbp + 0x98], rdx
142279bf6: 488b440128 mov rax, qword ptr [rcx + rax + 0x28]
142279bfb: 48898588000000 mov qword ptr [rbp + 0x88], rax
142279c02: 4531ed xor r13d, r13d
142279c05: 4489e9 mov ecx, r13d
142279c08: e9a2000000 jmp 0x142279caf
142279c0d: c785b000000000000000 mov dword ptr [rbp + 0xb0], 0
142279c17: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
142279c1d: 488b93a8010000 mov rdx, qword ptr [rbx + 0x1a8]
142279c24: 488d0cc9 lea rcx, [rcx + rcx*8]
142279c28: 4c8b44ca18 mov r8, qword ptr [rdx + rcx*8 + 0x18]
142279c2d: 488b4cca38 mov rcx, qword ptr [rdx + rcx*8 + 0x38]
142279c32: 4c8b8da0000000 mov r9, qword ptr [rbp + 0xa0]
142279c39: 498b9110010000 mov rdx, qword ptr [r9 + 0x110]
142279c40: 4d8b8918010000 mov r9, qword ptr [r9 + 0x118]
142279c47: 488b14d1 mov rdx, qword ptr [rcx + rdx*8]
142279c4b: 48f7d2 not rdx
142279c4e: 488d1452 lea rdx, [rdx + rdx*2]
142279c52: 48c1e204 shl rdx, 4
142279c56: 4d8b541010 mov r10, qword ptr [r8 + rdx + 0x10]
142279c5b: 4c899590000000 mov qword ptr [rbp + 0x90], r10
142279c62: 498b541028 mov rdx, qword ptr [r8 + rdx + 0x28]
142279c67: 48899580000000 mov qword ptr [rbp + 0x80], rdx
142279c6e: 4a8b0cc9 mov rcx, qword ptr [rcx + r9*8]
142279c72: 48f7d1 not rcx
142279c75: 488d0c49 lea rcx, [rcx + rcx*2]
142279c79: 48c1e104 shl rcx, 4
142279c7d: 498b540810 mov rdx, qword ptr [r8 + rcx + 0x10]
142279c82: 48899598000000 mov qword ptr [rbp + 0x98], rdx
142279c89: 498b4c0828 mov rcx, qword ptr [r8 + rcx + 0x28]
142279c8e: 48898d88000000 mov qword ptr [rbp + 0x88], rcx
142279c95: 488b4050 mov rax, qword ptr [rax + 0x50]
142279c99: 48894528 mov qword ptr [rbp + 0x28], rax
142279c9d: 4531ed xor r13d, r13d
142279ca0: 4c89e8 mov rax, r13
142279ca3: 48c1e004 shl rax, 4
142279ca7: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
142279cab: 8b4c0108 mov ecx, dword ptr [rcx + rax + 8]
142279caf: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
142279cb6: 488d0488 lea rax, [rax + rcx*4]
142279cba: 488d3c49 lea rdi, [rcx + rcx*2]
142279cbe: 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
142279cc5: 488d348a lea rsi, [rdx + rcx*4]
142279cc9: 48c1e104 shl rcx, 4
142279ccd: 48038d90000000 add rcx, qword ptr [rbp + 0x90]
142279cd4: 48c1e704 shl rdi, 4
142279cd8: 4803bd98000000 add rdi, qword ptr [rbp + 0x98]
142279cdf: f685b000000001 test byte ptr [rbp + 0xb0], 1
142279ce6: 7418 je 0x142279d00
142279ce8: 49ffc5 inc r13
142279ceb: 4c8d4104 lea r8, [rcx + 4]
142279cef: 488d5108 lea rdx, [rcx + 8]
142279cf3: f20f104904 movsd xmm1, qword ptr [rcx + 4]
142279cf8: eb1c jmp 0x142279d16
142279cfa: 660f1f440000 nop word ptr [rax + rax]
142279d00: 49ffc5 inc r13
142279d03: 4c8d4104 lea r8, [rcx + 4]
142279d07: 488d5108 lea rdx, [rcx + 8]
142279d0b: f20f104904 movsd xmm1, qword ptr [rcx + 4]
142279d10: 80790c00 cmp byte ptr [rcx + 0xc], 0
142279d14: 7410 je 0x142279d26
142279d16: 4c8b5d78 mov r11, qword ptr [rbp + 0x78]
142279d1a: f3410f104328 movss xmm0, dword ptr [r11 + 0x28]
142279d20: f30f5ec7 divss xmm0, xmm7
142279d24: eb0e jmp 0x142279d34
142279d26: 4c8b5d78 mov r11, qword ptr [rbp + 0x78]
142279d2a: f3410f104328 movss xmm0, dword ptr [r11 + 0x28]
142279d30: f30f5e01 divss xmm0, dword ptr [rcx]
142279d34: 448b8da8000000 mov r9d, dword ptr [rbp + 0xa8]
142279d3b: 448908 mov dword ptr [rax], r9d
142279d3e: f3410f5800 addss xmm0, dword ptr [r8]
142279d43: 0f57d2 xorps xmm2, xmm2
142279d46: f30f5fd0 maxss xmm2, xmm0
142279d4a: 0f28c6 movaps xmm0, xmm6
142279d4d: f30f5dc2 minss xmm0, xmm2
142279d51: f3410f1100 movss dword ptr [r8], xmm0
142279d56: 0f2ec6 ucomiss xmm0, xmm6
142279d59: 410f9bc1 setnp r9b
142279d5d: 410f94c2 sete r10b
142279d61: 4520ca and r10b, r9b
142279d64: f30f104108 movss xmm0, dword ptr [rcx + 8]
142279d69: f3410f105328 movss xmm2, dword ptr [r11 + 0x28]
142279d6f: 4484556c test byte ptr [rbp + 0x6c], r10b
142279d73: 751b jne 0x142279d90
142279d75: f30f5cc2 subss xmm0, xmm2
142279d79: 0f57d2 xorps xmm2, xmm2
142279d7c: f30f5fd0 maxss xmm2, xmm0
142279d80: 0f28c6 movaps xmm0, xmm6
142279d83: f30f5dc2 minss xmm0, xmm2
142279d87: eb0f jmp 0x142279d98
142279d89: 0f1f8000000000 nop dword ptr [rax]
142279d90: f30f58c2 addss xmm0, xmm2
142279d94: f30f5dc6 minss xmm0, xmm6
142279d98: f30f1102 movss dword ptr [rdx], xmm0
142279d9c: 448b8da8000000 mov r9d, dword ptr [rbp + 0xa8]
142279da3: 448908 mov dword ptr [rax], r9d
142279da6: c6410c00 mov byte ptr [rcx + 0xc], 0
142279daa: f3410f1000 movss xmm0, dword ptr [r8]
142279daf: 0f2ec1 ucomiss xmm0, xmm1
142279db2: f30f1012 movss xmm2, dword ptr [rdx]
142279db6: 7511 jne 0x142279dc9
142279db8: 7a0f jp 0x142279dc9
142279dba: f30f16c9 movshdup xmm1, xmm1
142279dbe: 0f2ed1 ucomiss xmm2, xmm1
142279dc1: 7506 jne 0x142279dc9
142279dc3: 0f8be3fcffff jnp 0x142279aac
142279dc9: 0f28de movaps xmm3, xmm6
142279dcc: f30f5cda subss xmm3, xmm2
142279dd0: f30f59db mulss xmm3, xmm3
142279dd4: 0f28ce movaps xmm1, xmm6
142279dd7: f30f5ccb subss xmm1, xmm3
142279ddb: e81070a1fe call 0x140c90df0 ; utils::pack_2x16f_uint
142279de0: 8b8da8000000 mov ecx, dword ptr [rbp + 0xa8]
142279de6: 890e mov dword ptr [rsi], ecx
142279de8: 894720 mov dword ptr [rdi + 0x20], eax
142279deb: e9bcfcffff jmp 0x142279aac
142279df0: 48837d5800 cmp qword ptr [rbp + 0x58], 0
142279df5: 7444 je 0x142279e3b
142279df7: e834e399fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
142279dfc: 4889c1 mov rcx, rax
142279dff: 488b00 mov rax, qword ptr [rax]
142279e02: 4883f801 cmp rax, 1
142279e06: 757d jne 0x142279e85
142279e08: 4883c108 add rcx, 8
142279e0c: 48833900 cmp qword ptr [rcx], 0
142279e10: 0f85ad000000 jne 0x142279ec3
142279e16: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
142279e1d: 48898da8000000 mov qword ptr [rbp + 0xa8], rcx
142279e24: 4883c108 add rcx, 8
142279e28: 488b5560 mov rdx, qword ptr [rbp + 0x60]
142279e2c: e8bfd299fe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
142279e31: 488b85a8000000 mov rax, qword ptr [rbp + 0xa8]
142279e38: 48ff00 inc qword ptr [rax]
142279e3b: 0f28b5c0000000 movaps xmm6, xmmword ptr [rbp + 0xc0]
142279e42: 0f28bdd0000000 movaps xmm7, xmmword ptr [rbp + 0xd0]
142279e49: 440f2885e0000000 movaps xmm8, xmmword ptr [rbp + 0xe0]
142279e51: 440f288df0000000 movaps xmm9, xmmword ptr [rbp + 0xf0]
142279e59: 440f289500010000 movaps xmm10, xmmword ptr [rbp + 0x100]
142279e61: 440f289d10010000 movaps xmm11, xmmword ptr [rbp + 0x110]
142279e69: 440f28a520010000 movaps xmm12, xmmword ptr [rbp + 0x120]
142279e71: 4881c4b8010000 add rsp, 0x1b8
142279e78: 5b pop rbx
142279e79: 5f pop rdi
142279e7a: 5e pop rsi
142279e7b: 415c pop r12
142279e7d: 415d pop r13
142279e7f: 415e pop r14
142279e81: 415f pop r15
142279e83: 5d pop rbp
142279e84: c3 ret 
142279e85: 4885c0 test rax, rax
142279e88: 752d jne 0x142279eb7
142279e8a: 31d2 xor edx, edx
142279e8c: e8cf066a00 call 0x14291a560 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142279e91: 4889c1 mov rcx, rax
142279e94: 4885c0 test rax, rax
142279e97: 0f856fffffff jne 0x142279e0c
142279e9d: eb18 jmp 0x142279eb7
142279e9f: 4885c0 test rax, rax
142279ea2: 7513 jne 0x142279eb7
142279ea4: 31d2 xor edx, edx
142279ea6: e8b5066a00 call 0x14291a560 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142279eab: 4889c1 mov rcx, rax
142279eae: 4885c0 test rax, rax
142279eb1: 0f85cff8ffff jne 0x142279786
142279eb7: 488d0dc2dedb00 lea rcx, [rip + 0xdbdec2]
142279ebe: e80de26500 call 0x1428d80d0 ; std::thread::local::panic_access_error
142279ec3: 488d0d26dfdb00 lea rcx, [rip + 0xdbdf26]
142279eca: e881f36500 call 0x1428d9250 ; core::cell::panic_already_borrowed
142279ecf: cc int3 
142279ed0: 4889542410 mov qword ptr [rsp + 0x10], rdx
142279ed5: 55 push rbp
142279ed6: 4157 push r15
142279ed8: 4156 push r14
142279eda: 4155 push r13
142279edc: 4154 push r12
142279ede: 56 push rsi
142279edf: 57 push rdi
142279ee0: 53 push rbx
142279ee1: 4881eca8000000 sub rsp, 0xa8
142279ee8: 488daa80000000 lea rbp, [rdx + 0x80]
142279eef: 440f29642430 movaps xmmword ptr [rsp + 0x30], xmm12
142279ef5: 440f295c2440 movaps xmmword ptr [rsp + 0x40], xmm11
142279efb: 440f29542450 movaps xmmword ptr [rsp + 0x50], xmm10
142279f01: 440f294c2460 movaps xmmword ptr [rsp + 0x60], xmm9
142279f07: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
142279f0d: 0f29bc2480000000 movaps xmmword ptr [rsp + 0x80], xmm7
142279f15: 0f29b42490000000 movaps xmmword ptr [rsp + 0x90], xmm6
142279f1d: 488b85b0000000 mov rax, qword ptr [rbp + 0xb0]
142279f24: 48ff00 inc qword ptr [rax]
142279f27: 0f28b42490000000 movaps xmm6, xmmword ptr [rsp + 0x90]
142279f2f: 0f28bc2480000000 movaps xmm7, xmmword ptr [rsp + 0x80]
142279f37: 440f28442470 movaps xmm8, xmmword ptr [rsp + 0x70]
142279f3d: 440f284c2460 movaps xmm9, xmmword ptr [rsp + 0x60]
142279f43: 440f28542450 movaps xmm10, xmmword ptr [rsp + 0x50]
142279f49: 440f285c2440 movaps xmm11, xmmword ptr [rsp + 0x40]
142279f4f: 440f28642430 movaps xmm12, xmmword ptr [rsp + 0x30]
142279f55: 4881c4a8000000 add rsp, 0xa8
142279f5c: 5b pop rbx
142279f5d: 5f pop rdi
142279f5e: 5e pop rsi
142279f5f: 415c pop r12
142279f61: 415d pop r13
142279f63: 415e pop r14
142279f65: 415f pop r15
142279f67: 5d pop rbp
142279f68: c3 ret 
142279f69: 0f1f8000000000 nop dword ptr [rax]
142279f70: 4889542410 mov qword ptr [rsp + 0x10], rdx
142279f75: 55 push rbp
142279f76: 4157 push r15
142279f78: 4156 push r14
142279f7a: 4155 push r13
142279f7c: 4154 push r12
142279f7e: 56 push rsi
142279f7f: 57 push rdi
142279f80: 53 push rbx
142279f81: 4881eca8000000 sub rsp, 0xa8
142279f88: 488daa80000000 lea rbp, [rdx + 0x80]
142279f8f: 440f29642430 movaps xmmword ptr [rsp + 0x30], xmm12
142279f95: 440f295c2440 movaps xmmword ptr [rsp + 0x40], xmm11
142279f9b: 440f29542450 movaps xmmword ptr [rsp + 0x50], xmm10
142279fa1: 440f294c2460 movaps xmmword ptr [rsp + 0x60], xmm9
142279fa7: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
142279fad: 0f29bc2480000000 movaps xmmword ptr [rsp + 0x80], xmm7
142279fb5: 0f29b42490000000 movaps xmmword ptr [rsp + 0x90], xmm6
142279fbd: 48837d5800 cmp qword ptr [rbp + 0x58], 0
142279fc2: 7420 je 0x142279fe4
142279fc4: 488d4560 lea rax, [rbp + 0x60]
142279fc8: 488945f0 mov qword ptr [rbp - 0x10], rax
142279fcc: 488d45f0 lea rax, [rbp - 0x10]
142279fd0: 488945e8 mov qword ptr [rbp - 0x18], rax
142279fd4: 488d0d5ddfdb00 lea rcx, [rip + 0xdbdf5d]
142279fdb: 488d55e8 lea rdx, [rbp - 0x18]
142279fdf: e86cf0ffff call 0x142279050 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
142279fe4: 0f28b42490000000 movaps xmm6, xmmword ptr [rsp + 0x90]
142279fec: 0f28bc2480000000 movaps xmm7, xmmword ptr [rsp + 0x80]
142279ff4: 440f28442470 movaps xmm8, xmmword ptr [rsp + 0x70]
142279ffa: 440f284c2460 movaps xmm9, xmmword ptr [rsp + 0x60]
14227a000: 440f28542450 movaps xmm10, xmmword ptr [rsp + 0x50]
14227a006: 440f285c2440 movaps xmm11, xmmword ptr [rsp + 0x40]
14227a00c: 440f28642430 movaps xmm12, xmmword ptr [rsp + 0x30]
14227a012: 4881c4a8000000 add rsp, 0xa8
14227a019: 5b pop rbx
14227a01a: 5f pop rdi
14227a01b: 5e pop rsi
14227a01c: 415c pop r12
14227a01e: 415d pop r13
14227a020: 415e pop r14
14227a022: 415f pop r15
14227a024: 5d pop rbp
14227a025: c3 ret 
14227a026: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
14227a030: 4889542410 mov qword ptr [rsp + 0x10], rdx
14227a035: 55 push rbp
14227a036: 4157 push r15
14227a038: 4156 push r14
14227a03a: 4155 push r13
14227a03c: 4154 push r12
14227a03e: 56 push rsi
14227a03f: 57 push rdi
14227a040: 53 push rbx
14227a041: 4881eca8000000 sub rsp, 0xa8
14227a048: 488daa80000000 lea rbp, [rdx + 0x80]
14227a04f: 440f29642430 movaps xmmword ptr [rsp + 0x30], xmm12
14227a055: 440f295c2440 movaps xmmword ptr [rsp + 0x40], xmm11
14227a05b: 440f29542450 movaps xmmword ptr [rsp + 0x50], xmm10
14227a061: 440f294c2460 movaps xmmword ptr [rsp + 0x60], xmm9
14227a067: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
14227a06d: 0f29bc2480000000 movaps xmmword ptr [rsp + 0x80], xmm7
14227a075: 0f29b42490000000 movaps xmmword ptr [rsp + 0x90], xmm6
14227a07d: 488b85a8000000 mov rax, qword ptr [rbp + 0xa8]
14227a084: 48ff00 inc qword ptr [rax]
14227a087: 0f28b42490000000 movaps xmm6, xmmword ptr [rsp + 0x90]
14227a08f: 0f28bc2480000000 movaps xmm7, xmmword ptr [rsp + 0x80]
14227a097: 440f28442470 movaps xmm8, xmmword ptr [rsp + 0x70]
14227a09d: 440f284c2460 movaps xmm9, xmmword ptr [rsp + 0x60]
14227a0a3: 440f28542450 movaps xmm10, xmmword ptr [rsp + 0x50]
14227a0a9: 440f285c2440 movaps xmm11, xmmword ptr [rsp + 0x40]
14227a0af: 440f28642430 movaps xmm12, xmmword ptr [rsp + 0x30]
14227a0b5: 4881c4a8000000 add rsp, 0xa8
14227a0bc: 5b pop rbx
14227a0bd: 5f pop rdi
14227a0be: 5e pop rsi
14227a0bf: 415c pop r12
14227a0c1: 415d pop r13
14227a0c3: 415e pop r14
14227a0c5: 415f pop r15
14227a0c7: 5d pop rbp
14227a0c8: c3 ret 

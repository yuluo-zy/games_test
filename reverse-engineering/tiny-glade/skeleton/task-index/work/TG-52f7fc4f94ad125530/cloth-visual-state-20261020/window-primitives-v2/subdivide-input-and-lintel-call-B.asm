1407ed659: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ed660: 8b48e4 mov ecx, dword ptr [rax - 0x1c]
1407ed663: e8d88b4a00 call 0x140c96240 ; utils::rng::init_rng
1407ed668: 4889c7 mov rdi, rax
1407ed66b: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ed672: 0fb670fc movzx esi, byte ptr [rax - 4]
1407ed676: 4883fe02 cmp rsi, 2
1407ed67a: 0f85e7010000 jne 0x1407ed867
1407ed680: 48b82f64bd78641d76a0 movabs rax, 0xa0761d6478bd642f
1407ed68a: 4801c7 add rdi, rax
1407ed68d: 4889f8 mov rax, rdi
1407ed690: 48b9db28b4a0d17e03e7 movabs rcx, 0xe7037ed1a0b428db
1407ed69a: 4831c8 xor rax, rcx
1407ed69d: 48f7e7 mul rdi
1407ed6a0: 31d0 xor eax, edx
1407ed6a2: c1e81f shr eax, 0x1f
1407ed6a5: 48898548010000 mov qword ptr [rbp + 0x148], rax
1407ed6ac: 80bd8002000001 cmp byte ptr [rbp + 0x280], 1
1407ed6b3: 0f84c6010000 je 0x1407ed87f
1407ed6b9: be02000000 mov esi, 2
1407ed6be: 450f57c9 xorps xmm9, xmm9
1407ed6c2: bb03000000 mov ebx, 3
1407ed6c7: f3440f10b5d0000000 movss xmm14, dword ptr [rbp + 0xd0]
1407ed6d0: 488b8d70020000 mov rcx, qword ptr [rbp + 0x270]
1407ed6d7: 488d1512db4302 lea rdx, [rip + 0x243db12]
1407ed6de: e87d5b0400 call 0x140833260 ; country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
1407ed6e3: f30f1038 movss xmm7, dword ptr [rax]
1407ed6e7: f3440f105804 movss xmm11, dword ptr [rax + 4]
1407ed6ed: 488b8d68020000 mov rcx, qword ptr [rbp + 0x268]
1407ed6f4: e847d03300 call 0x140b2a740 ; country_core::resources::walls::public_wall_state::PublicWallState::wall_space
1407ed6f9: 898544010000 mov dword ptr [rbp + 0x144], eax
1407ed6ff: 0f28f0 movaps xmm6, xmm0
1407ed702: 450f28c6 movaps xmm8, xmm14
1407ed706: f3440f5905d9c91602 mulss xmm8, dword ptr [rip + 0x216c9d9]
1407ed70f: 8b8544010000 mov eax, dword ptr [rbp + 0x144]
1407ed715: 898510020000 mov dword ptr [rbp + 0x210], eax
1407ed71b: f30f118514020000 movss dword ptr [rbp + 0x214], xmm0
1407ed723: f3410f58f8 addss xmm7, xmm8
1407ed728: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ed72f: 0f28cf movaps xmm1, xmm7
1407ed732: e8e9474a00 call 0x140c91f20 ; utils::wall_space::impl$1::project_unbounded
1407ed737: 0f57c9 xorps xmm1, xmm1
1407ed73a: f30f2ace cvtsi2ss xmm1, esi
1407ed73e: f3440f5905492f1702 mulss xmm8, dword ptr [rip + 0x2172f49]
1407ed747: f3450f58c3 addss xmm8, xmm11
1407ed74c: f3440f5ef1 divss xmm14, xmm1
1407ed751: 48c7851002000000000000 mov qword ptr [rbp + 0x210], 0
1407ed75c: 48899d18020000 mov qword ptr [rbp + 0x218], rbx
1407ed763: 8b8544010000 mov eax, dword ptr [rbp + 0x144]
1407ed769: 898520020000 mov dword ptr [rbp + 0x220], eax
1407ed76f: f30f11b524020000 movss dword ptr [rbp + 0x224], xmm6
1407ed777: f30f118528020000 movss dword ptr [rbp + 0x228], xmm0
1407ed77f: f3440f11852c020000 movss dword ptr [rbp + 0x22c], xmm8
1407ed788: f3440f11b530020000 movss dword ptr [rbp + 0x230], xmm14
1407ed791: 488b8568020000 mov rax, qword ptr [rbp + 0x268]
1407ed798: 48898538020000 mov qword ptr [rbp + 0x238], rax
1407ed79f: c7854002000000000000 mov dword ptr [rbp + 0x240], 0
1407ed7a9: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407ed7b0: 488d9510020000 lea rdx, [rbp + 0x210]
1407ed7b7: 4c8d0572cb2902 lea r8, [rip + 0x229cb72]
1407ed7be: e8dd34a700 call 0x141260ca0 ; alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core:
1407ed7c3: 4c8b85c8010000 mov r8, qword ptr [rbp + 0x1c8]
1407ed7ca: 4c8b8dd0010000 mov r9, qword ptr [rbp + 0x1d0]
1407ed7d1: f3440f114c2448 movss dword ptr [rsp + 0x48], xmm9
1407ed7d8: 488d85e0000000 lea rax, [rbp + 0xe0]
1407ed7df: 4889442438 mov qword ptr [rsp + 0x38], rax
1407ed7e4: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
1407ed7eb: 4889442430 mov qword ptr [rsp + 0x30], rax
1407ed7f0: 8b8590010000 mov eax, dword ptr [rbp + 0x190]
1407ed7f6: 89442428 mov dword ptr [rsp + 0x28], eax
1407ed7fa: 0fb685f8010000 movzx eax, byte ptr [rbp + 0x1f8]
1407ed801: 88442420 mov byte ptr [rsp + 0x20], al
1407ed805: c744245000000000 mov dword ptr [rsp + 0x50], 0
1407ed80d: 488b8d78020000 mov rcx, qword ptr [rbp + 0x278]
1407ed814: 488b9570020000 mov rdx, qword ptr [rbp + 0x270]
1407ed81b: 4c898568020000 mov qword ptr [rbp + 0x268], r8
1407ed822: e869210000 call 0x1407ef990 ; system_decorator::decorator_visual::cottage_wall_window::add_lintels

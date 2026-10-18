140a17880 55 push rbp
140a17881 4157 push r15
140a17883 4156 push r14
140a17885 4155 push r13
140a17887 4154 push r12
140a17889 56 push rsi
140a1788a 57 push rdi
140a1788b 53 push rbx
140a1788c 4881ecf8030000 sub rsp, 0x3f8
140a17893 488dac2480000000 lea rbp, [rsp + 0x80]
140a1789b 0f29bd60030000 movaps xmmword ptr [rbp + 0x360], xmm7
140a178a2 0f29b550030000 movaps xmmword ptr [rbp + 0x350], xmm6
140a178a9 48c78548030000feffffff mov qword ptr [rbp + 0x348], 0xfffffffffffffffe
140a178b4 4c89c7 mov rdi, r8
140a178b7 4989d6 mov r14, rdx
140a178ba 4889cb mov rbx, rcx
140a178bd 4c8d6a38 lea r13, [rdx + 0x38]
140a178c1 48ba28641ac941e9b8c2 movabs rdx, 0xc2b8e941c91a6428
140a178cb 49b8a043dad3d1c6f359 movabs r8, 0x59f3c6d1d3da43a0
140a178d5 4c89e9 mov rcx, r13
140a178d8 e8135e6cff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
140a178dd 4885c0 test rax, rax
140a178e0 0f8412070000 je 0x140a17ff8
140a178e6 49399618020000 cmp qword ptr [r14 + 0x218], rdx
140a178ed 0f8605070000 jbe 0x140a17ff8
140a178f3 498b8610020000 mov rax, qword ptr [r14 + 0x210]
140a178fa 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140a178fe 4885c9 test rcx, rcx
140a17901 0f84f1060000 je 0x140a17ff8
140a17907 498b86e0010000 mov rax, qword ptr [r14 + 0x1e0]
140a1790e 48f7d1 not rcx
140a17911 488d0c49 lea rcx, [rcx + rcx*2]
140a17915 48c1e105 shl rcx, 5
140a17919 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a1791f 0f84d3060000 je 0x140a17ff8
140a17925 4801c8 add rax, rcx
140a17928 488b4010 mov rax, qword ptr [rax + 0x10]
140a1792c 8b30 mov esi, dword ptr [rax]
140a1792e 48baa612238ca9dc4d86 movabs rdx, 0x864ddca98c2312a6
140a17938 49b84e489a9e36e9b027 movabs r8, 0x27b0e9369e9a484e
140a17942 4c89e9 mov rcx, r13
140a17945 e8a65d6cff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
140a1794a 4885c0 test rax, rax
140a1794d 0f84b1060000 je 0x140a18004
140a17953 49399618020000 cmp qword ptr [r14 + 0x218], rdx
140a1795a 0f86a4060000 jbe 0x140a18004
140a17960 498b8610020000 mov rax, qword ptr [r14 + 0x210]
140a17967 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140a1796b 4885c9 test rcx, rcx
140a1796e 0f8490060000 je 0x140a18004
140a17974 498b86e0010000 mov rax, qword ptr [r14 + 0x1e0]
140a1797b 48f7d1 not rcx
140a1797e 488d0c49 lea rcx, [rcx + rcx*2]
140a17982 48c1e105 shl rcx, 5
140a17986 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a1798c 0f8472060000 je 0x140a18004
140a17992 4889bd30030000 mov qword ptr [rbp + 0x330], rdi
140a17999 4801c8 add rax, rcx
140a1799c 4c8b6010 mov r12, qword ptr [rax + 0x10]
140a179a0 48baffa3086aee51a4bb movabs rdx, 0xbba451ee6a08a3ff
140a179aa 49b89df2bcfe158d008d movabs r8, 0x8d008d15febcf29d
140a179b4 4c89e9 mov rcx, r13
140a179b7 e8345d6cff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
140a179bc 4885c0 test rax, rax
140a179bf 0f844b060000 je 0x140a18010
140a179c5 49399618020000 cmp qword ptr [r14 + 0x218], rdx
140a179cc 0f863e060000 jbe 0x140a18010
140a179d2 498b8610020000 mov rax, qword ptr [r14 + 0x210]
140a179d9 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140a179dd 4885c9 test rcx, rcx
140a179e0 0f842a060000 je 0x140a18010
140a179e6 498b86e0010000 mov rax, qword ptr [r14 + 0x1e0]
140a179ed 48f7d1 not rcx
140a179f0 488d0c49 lea rcx, [rcx + rcx*2]
140a179f4 48c1e105 shl rcx, 5
140a179f8 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a179fe 0f840c060000 je 0x140a18010
140a17a04 4801c8 add rax, rcx
140a17a07 83fe13 cmp esi, 0x13
140a17a0a 0f94c1 sete cl
140a17a0d 488b7010 mov rsi, qword ptr [rax + 0x10]
140a17a11 0fb686d0040000 movzx eax, byte ptr [rsi + 0x4d0]
140a17a18 c1e003 shl eax, 3
140a17a1b 488d15260f0b02 lea rdx, [rip + 0x20b0f26]
140a17a22 488b1410 mov rdx, qword ptr [rax + rdx]
140a17a26 4c8d054b0f0b02 lea r8, [rip + 0x20b0f4b]
140a17a2d 4a8b0400 mov rax, qword ptr [rax + r8]
140a17a31 4c8d3c16 lea r15, [rsi + rdx]
140a17a35 f30f10741618 movss xmm6, dword ptr [rsi + rdx + 0x18]
140a17a3b 48898538030000 mov qword ptr [rbp + 0x338], rax
140a17a42 0fb6840628010000 movzx eax, byte ptr [rsi + rax + 0x128]
140a17a4a 888547030000 mov byte ptr [rbp + 0x347], al
140a17a50 8b0416 mov eax, dword ptr [rsi + rdx]
140a17a53 83f802 cmp eax, 2
140a17a56 400f92c7 setb dil
140a17a5a 4020cf and dil, cl
140a17a5d 7473 je 0x140a17ad2
140a17a5f 48ba97aa2d76b9142afc movabs rdx, 0xfc2a14b9762daa97
140a17a69 49b829b6b9a19de85866 movabs r8, 0x6658e89da1b9b629
140a17a73 4c89e9 mov rcx, r13
140a17a76 e8755c6cff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
140a17a7b 4885c0 test rax, rax
140a17a7e 0f8498050000 je 0x140a1801c
140a17a84 49399618020000 cmp qword ptr [r14 + 0x218], rdx
140a17a8b 0f868b050000 jbe 0x140a1801c
140a17a91 498b8610020000 mov rax, qword ptr [r14 + 0x210]
140a17a98 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140a17a9c 4885c9 test rcx, rcx
140a17a9f 0f8477050000 je 0x140a1801c
140a17aa5 498b86e0010000 mov rax, qword ptr [r14 + 0x1e0]
140a17aac 48f7d1 not rcx
140a17aaf 488d0c49 lea rcx, [rcx + rcx*2]
140a17ab3 48c1e105 shl rcx, 5
140a17ab7 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a17abd 0f8459050000 je 0x140a1801c
140a17ac3 4801c8 add rax, rcx
140a17ac6 488b4010 mov rax, qword ptr [rax + 0x10]
140a17aca 8b5004 mov edx, dword ptr [rax + 4]
140a17acd 418b07 mov eax, dword ptr [r15]
140a17ad0 eb05 jmp 0x140a17ad7
140a17ad2 baffffffff mov edx, 0xffffffff
140a17ad7 83c0fe add eax, -2
140a17ada 83f802 cmp eax, 2
140a17add b902000000 mov ecx, 2
140a17ae2 0f42c8 cmovb ecx, eax
140a17ae5 83f902 cmp ecx, 2
140a17ae8 899540030000 mov dword ptr [rbp + 0x340], edx
140a17aee 7411 je 0x140a17b01
140a17af0 83f901 cmp ecx, 1
140a17af3 756c jne 0x140a17b61
140a17af5 b808000000 mov eax, 8
140a17afa b904000000 mov ecx, 4
140a17aff eb0a jmp 0x140a17b0b
140a17b01 b814000000 mov eax, 0x14
140a17b06 b910000000 mov ecx, 0x10
140a17b0b 418b1424 mov edx, dword ptr [r12]
140a17b0f f3480f2ac2 cvtsi2ss xmm0, rdx
140a17b14 418b542404 mov edx, dword ptr [r12 + 4]
140a17b19 f3480f2aca cvtsi2ss xmm1, rdx
140a17b1e f3410f10140f movss xmm2, dword ptr [r15 + rcx]
140a17b24 f30f5ec1 divss xmm0, xmm1
140a17b28 f30f5ed0 divss xmm2, xmm0
140a17b2c f3410f100407 movss xmm0, dword ptr [r15 + rax]
140a17b32 0f14d0 unpcklps xmm2, xmm0
140a17b35 0f5915a4ddf001 mulps xmm2, xmmword ptr [rip + 0x1f0dda4]
140a17b3c f30f10052cddf001 movss xmm0, dword ptr [rip + 0x1f0dd2c]
140a17b44 0f58c2 addps xmm0, xmm2
140a17b47 0f283d92910602 movaps xmm7, xmmword ptr [rip + 0x2069192]
140a17b4e 0f5cfa subps xmm7, xmm2
140a17b51 0fc6f801 shufps xmm7, xmm0, 1
140a17b55 0fc6f8e2 shufps xmm7, xmm0, 0xe2
140a17b59 4531ed xor r13d, r13d
140a17b5c e99d000000 jmp 0x140a17bfe
140a17b61 48ba80db3f3b7610ab4c movabs rdx, 0x4cab10763b3fdb80
140a17b6b 49b84db53475ef22a401 movabs r8, 0x1a422ef7534b54d
140a17b75 4c89e9 mov rcx, r13
140a17b78 e8735b6cff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
140a17b7d 4885c0 test rax, rax
140a17b80 0f84a2040000 je 0x140a18028
140a17b86 49399618020000 cmp qword ptr [r14 + 0x218], rdx
140a17b8d 0f8695040000 jbe 0x140a18028
140a17b93 498b8610020000 mov rax, qword ptr [r14 + 0x210]
140a17b9a 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140a17b9e 4885c9 test rcx, rcx
140a17ba1 0f8481040000 je 0x140a18028
140a17ba7 498b86e0010000 mov rax, qword ptr [r14 + 0x1e0]
140a17bae 48f7d1 not rcx
140a17bb1 488d0c49 lea rcx, [rcx + rcx*2]
140a17bb5 48c1e105 shl rcx, 5
140a17bb9 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a17bbf 0f8463040000 je 0x140a18028
140a17bc5 4801c8 add rax, rcx
140a17bc8 488b4010 mov rax, qword ptr [rax + 0x10]
140a17bcc f20f1038 movsd xmm7, qword ptr [rax]
140a17bd0 0f583d09ddf001 addps xmm7, xmmword ptr [rip + 0x1f0dd09]
140a17bd7 f2410f100424 movsd xmm0, qword ptr [r12]
140a17bdd 660f57c9 xorpd xmm1, xmm1
140a17be1 0f14c1 unpcklps xmm0, xmm1
140a17be4 660f280dc419fd01 movapd xmm1, xmmword ptr [rip + 0x1fd19c4]
140a17bec 660f56c1 orpd xmm0, xmm1
140a17bf0 660f5cc1 subpd xmm0, xmm1
140a17bf4 660f5ac0 cvtpd2ps xmm0, xmm0
140a17bf8 0f5ef8 divps xmm7, xmm0
140a17bfb 41b501 mov r13b, 1
140a17bfe 48895c2430 mov qword ptr [rsp + 0x30], rbx
140a17c03 4c89742428 mov qword ptr [rsp + 0x28], r14
140a17c08 48c744242016000000 mov qword ptr [rsp + 0x20], 0x16
140a17c11 488d15180c0b02 lea rdx, [rip + 0x20b0c18]
140a17c18 4c8d0d2a0c0b02 lea r9, [rip + 0x20b0c2a]
140a17c1f 488d8da0010000 lea rcx, [rbp + 0x1a0]
140a17c26 41b819000000 mov r8d, 0x19
140a17c2c e86fb6be00 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a17c31 80bdd602000008 cmp byte ptr [rbp + 0x2d6], 8
140a17c38 742f je 0x140a17c69
140a17c3a 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a17c41 bae8000000 mov edx, 0xe8
140a17c46 480395d8020000 add rdx, qword ptr [rbp + 0x2d8]
140a17c4d f30f16c7 movshdup xmm0, xmm7
140a17c51 f30f11442420 movss dword ptr [rsp + 0x20], xmm0
140a17c57 49b8219bbb7e1af6d7e6 movabs r8, 0xe6d7f61a7ebb9b21
140a17c61 0f28df movaps xmm3, xmm7
140a17c64 e8d73cf5ff call 0x14096b940 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec2::Vec2>,glam::f32::vec2::Vec2>
140a17c69 488d4d10 lea rcx, [rbp + 0x10]
140a17c6d 488d95a0010000 lea rdx, [rbp + 0x1a0]
140a17c74 41b890010000 mov r8d, 0x190
140a17c7a e881bff001 call 0x142923c00 ; 
140a17c7f 80bd4601000008 cmp byte ptr [rbp + 0x146], 8
140a17c86 7425 je 0x140a17cad
140a17c88 488d8d20010000 lea rcx, [rbp + 0x120]
140a17c8f bae8000000 mov edx, 0xe8
140a17c94 48039548010000 add rdx, qword ptr [rbp + 0x148]
140a17c9b 49b87c22a88cd6035bfd movabs r8, 0xfd5b03d68ca8227c
140a17ca5 4589e9 mov r9d, r13d
140a17ca8 e8a33bf5ff call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
140a17cad 488d8da0010000 lea rcx, [rbp + 0x1a0]
140a17cb4 488d5510 lea rdx, [rbp + 0x10]
140a17cb8 41b890010000 mov r8d, 0x190
140a17cbe e83dbff001 call 0x142923c00 ; 
140a17cc3 80bdd602000008 cmp byte ptr [rbp + 0x2d6], 8
140a17cca 7425 je 0x140a17cf1
140a17ccc 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a17cd3 bae8000000 mov edx, 0xe8
140a17cd8 480395d8020000 add rdx, qword ptr [rbp + 0x2d8]
140a17cdf 49b8bd6912ab5eb68327 movabs r8, 0x2783b65eab1269bd
140a17ce9 0f28de movaps xmm3, xmm6
140a17cec e83f3df5ff call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
140a17cf1 488d4d10 lea rcx, [rbp + 0x10]
140a17cf5 488d95a0010000 lea rdx, [rbp + 0x1a0]
140a17cfc 41b890010000 mov r8d, 0x190
140a17d02 e8f9bef001 call 0x142923c00 ; 
140a17d07 80bd4601000008 cmp byte ptr [rbp + 0x146], 8
140a17d0e 7429 je 0x140a17d39
140a17d10 488d8d20010000 lea rcx, [rbp + 0x120]
140a17d17 bae8000000 mov edx, 0xe8
140a17d1c 48039548010000 add rdx, qword ptr [rbp + 0x148]
140a17d23 49b845cc2d881fbfdf78 movabs r8, 0x78dfbf1f882dcc45
140a17d2d 448b8d40030000 mov r9d, dword ptr [rbp + 0x340]
140a17d34 e83738f5ff call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a17d39 488d8da0010000 lea rcx, [rbp + 0x1a0]
140a17d40 488d5510 lea rdx, [rbp + 0x10]
140a17d44 41b890010000 mov r8d, 0x190
140a17d4a e8b1bef001 call 0x142923c00 ; 
140a17d4f 80bdd602000008 cmp byte ptr [rbp + 0x2d6], 8
140a17d56 7425 je 0x140a17d7d
140a17d58 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a17d5f bae8000000 mov edx, 0xe8
140a17d64 480395d8020000 add rdx, qword ptr [rbp + 0x2d8]
140a17d6b 49b807aac3bea98de477 movabs r8, 0x77e48da9bec3aa07
140a17d75 4189f9 mov r9d, edi
140a17d78 e8d33af5ff call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
140a17d7d 488d4d10 lea rcx, [rbp + 0x10]
140a17d81 488d95a0010000 lea rdx, [rbp + 0x1a0]
140a17d88 41b890010000 mov r8d, 0x190
140a17d8e e86dbef001 call 0x142923c00 ; 
140a17d93 80bd4601000008 cmp byte ptr [rbp + 0x146], 8
140a17d9a 742a je 0x140a17dc6
140a17d9c 488d8d20010000 lea rcx, [rbp + 0x120]
140a17da3 bae8000000 mov edx, 0xe8
140a17da8 48039548010000 add rdx, qword ptr [rbp + 0x148]
140a17daf 49b8d5fd472b85a47f0d movabs r8, 0xd7fa4852b47fdd5
140a17db9 440fb68d47030000 movzx r9d, byte ptr [rbp + 0x347]
140a17dc1 e88a3af5ff call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
140a17dc6 488d8da0010000 lea rcx, [rbp + 0x1a0]
140a17dcd 488d5510 lea rdx, [rbp + 0x10]
140a17dd1 41b890010000 mov r8d, 0x190
140a17dd7 e824bef001 call 0x142923c00 ; 
140a17ddc 80bdd602000008 cmp byte ptr [rbp + 0x2d6], 8
140a17de3 7432 je 0x140a17e17
140a17de5 488b8538030000 mov rax, qword ptr [rbp + 0x338]
140a17dec f30f109c0618010000 movss xmm3, dword ptr [rsi + rax + 0x118]
140a17df5 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a17dfc bae8000000 mov edx, 0xe8
140a17e01 480395d8020000 add rdx, qword ptr [rbp + 0x2d8]
140a17e08 49b88b29abfaccde65b6 movabs r8, 0xb665deccfaab298b
140a17e12 e8193cf5ff call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
140a17e17 488d7d10 lea rdi, [rbp + 0x10]
140a17e1b 488d95a0010000 lea rdx, [rbp + 0x1a0]
140a17e22 41b890010000 mov r8d, 0x190
140a17e28 4889f9 mov rcx, rdi
140a17e2b e8d0bdf001 call 0x142923c00 ; 
140a17e30 488d9da0010000 lea rbx, [rbp + 0x1a0]
140a17e37 49b8b0084bdec65465c2 movabs r8, 0xc26554c6de4b08b0
140a17e41 4889d9 mov rcx, rbx
140a17e44 4889fa mov rdx, rdi
140a17e47 e884cfbe00 call 0x141604dd0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>
140a17e4c 49b8e9e22f4267f9ddd2 movabs r8, 0xd2ddf967422fe2e9
140a17e56 4889f9 mov rcx, rdi
140a17e59 4889da mov rdx, rbx
140a17e5c e85fe3be00 call 0x1416061c0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo>
140a17e61 488b8530030000 mov rax, qword ptr [rbp + 0x330]
140a17e68 488945c0 mov qword ptr [rbp - 0x40], rax
140a17e6c c785a001000002000000 mov dword ptr [rbp + 0x1a0], 2
140a17e76 48895c2420 mov qword ptr [rsp + 0x20], rbx
140a17e7b 488d4dc0 lea rcx, [rbp - 0x40]
140a17e7f 49b8fcfb49a09d576148 movabs r8, 0x4861579da049fbfc
140a17e89 4889fa mov rdx, rdi
140a17e8c 41b119 mov r9b, 0x19
140a17e8f e8ac63edff call 0x1408ee240 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
140a17e94 488d8da0010000 lea rcx, [rbp + 0x1a0]
140a17e9b 488d5510 lea rdx, [rbp + 0x10]
140a17e9f 41b890010000 mov r8d, 0x190
140a17ea5 e856bdf001 call 0x142923c00 ; 
140a17eaa 0f108540020000 movups xmm0, xmmword ptr [rbp + 0x240]
140a17eb1 0f294500 movaps xmmword ptr [rbp], xmm0
140a17eb5 0f108530020000 movups xmm0, xmmword ptr [rbp + 0x230]
140a17ebc 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
140a17ec0 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
140a17ec7 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140a17ecb 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
140a17ed2 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
140a17ed6 0f108500020000 movups xmm0, xmmword ptr [rbp + 0x200]
140a17edd 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
140a17ee1 488b85d8020000 mov rax, qword ptr [rbp + 0x2d8]
140a17ee8 488b8d20030000 mov rcx, qword ptr [rbp + 0x320]
140a17eef 488b95b0010000 mov rdx, qword ptr [rbp + 0x1b0]
140a17ef6 48895520 mov qword ptr [rbp + 0x20], rdx
140a17efa 0f1085a0010000 movups xmm0, xmmword ptr [rbp + 0x1a0]
140a17f01 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
140a17f05 0f1085b8010000 movups xmm0, xmmword ptr [rbp + 0x1b8]
140a17f0c 0f294540 movaps xmmword ptr [rbp + 0x40], xmm0
140a17f10 488b95c8010000 mov rdx, qword ptr [rbp + 0x1c8]
140a17f17 48895550 mov qword ptr [rbp + 0x50], rdx
140a17f1b 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
140a17f22 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
140a17f26 488b95e0010000 mov rdx, qword ptr [rbp + 0x1e0]
140a17f2d 48895538 mov qword ptr [rbp + 0x38], rdx
140a17f31 0f108550020000 movups xmm0, xmmword ptr [rbp + 0x250]
140a17f38 0f108d60020000 movups xmm1, xmmword ptr [rbp + 0x260]
140a17f3f 0f109570020000 movups xmm2, xmmword ptr [rbp + 0x270]
140a17f46 0f109d80020000 movups xmm3, xmmword ptr [rbp + 0x280]
140a17f4d 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
140a17f54 0f118d98000000 movups xmmword ptr [rbp + 0x98], xmm1
140a17f5b 0f1195a8000000 movups xmmword ptr [rbp + 0xa8], xmm2
140a17f62 0f119db8000000 movups xmmword ptr [rbp + 0xb8], xmm3
140a17f69 0f108590020000 movups xmm0, xmmword ptr [rbp + 0x290]
140a17f70 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
140a17f77 0f1085a0020000 movups xmm0, xmmword ptr [rbp + 0x2a0]
140a17f7e 0f1185d8000000 movups xmmword ptr [rbp + 0xd8], xmm0
140a17f85 488b95f8010000 mov rdx, qword ptr [rbp + 0x1f8]
140a17f8c 48895568 mov qword ptr [rbp + 0x68], rdx
140a17f90 0f1085e8010000 movups xmm0, xmmword ptr [rbp + 0x1e8]
140a17f97 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
140a17f9b 48898df0000000 mov qword ptr [rbp + 0xf0], rcx
140a17fa2 488985e8000000 mov qword ptr [rbp + 0xe8], rax
140a17fa9 0f2805c0fe0a02 movaps xmm0, xmmword ptr [rip + 0x20afec0]
140a17fb0 0f294570 movaps xmmword ptr [rbp + 0x70], xmm0
140a17fb4 488d4dc0 lea rcx, [rbp - 0x40]
140a17fb8 488d5510 lea rdx, [rbp + 0x10]
140a17fbc e80f55fcff call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a17fc1 80bdd602000008 cmp byte ptr [rbp + 0x2d6], 8
140a17fc8 740c je 0x140a17fd6
140a17fca 488d8dc0020000 lea rcx, [rbp + 0x2c0]
140a17fd1 e8daa41c00 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a17fd6 0f28b550030000 movaps xmm6, xmmword ptr [rbp + 0x350]
140a17fdd 0f28bd60030000 movaps xmm7, xmmword ptr [rbp + 0x360]
140a17fe4 4881c4f8030000 add rsp, 0x3f8
140a17feb 5b pop rbx
140a17fec 5f pop rdi
140a17fed 5e pop rsi
140a17fee 415c pop r12
140a17ff0 415d pop r13
140a17ff2 415e pop r14
140a17ff4 415f pop r15
140a17ff6 5d pop rbp
140a17ff7 c3 ret 
140a17ff8 488d0db9070b02 lea rcx, [rip + 0x20b07b9]
140a17fff e80c13ec01 call 0x1428d9310 ; core::option::unwrap_failed
140a18004 488d0dc5070b02 lea rcx, [rip + 0x20b07c5]
140a1800b e80013ec01 call 0x1428d9310 ; core::option::unwrap_failed
140a18010 488d0dd1070b02 lea rcx, [rip + 0x20b07d1]
140a18017 e8f412ec01 call 0x1428d9310 ; core::option::unwrap_failed
140a1801c 488d0ddd070b02 lea rcx, [rip + 0x20b07dd]
140a18023 e8e812ec01 call 0x1428d9310 ; core::option::unwrap_failed
140a18028 488d0de9070b02 lea rcx, [rip + 0x20b07e9]
140a1802f e8dc12ec01 call 0x1428d9310 ; core::option::unwrap_failed
140a18034 cc int3 
140a18035 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a18040 4889542410 mov qword ptr [rsp + 0x10], rdx
140a18045 55 push rbp
140a18046 4157 push r15
140a18048 4156 push r14
140a1804a 4155 push r13
140a1804c 4154 push r12
140a1804e 56 push rsi
140a1804f 57 push rdi
140a18050 53 push rbx
140a18051 4883ec58 sub rsp, 0x58
140a18055 488daa80000000 lea rbp, [rdx + 0x80]
140a1805c 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a18061 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a18066 80bdd602000008 cmp byte ptr [rbp + 0x2d6], 8
140a1806d 740c je 0x140a1807b
140a1806f 488d8dc0020000 lea rcx, [rbp + 0x2c0]
140a18076 e835a41c00 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a1807b 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a18080 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a18085 4883c458 add rsp, 0x58
140a18089 5b pop rbx
140a1808a 5f pop rdi
140a1808b 5e pop rsi
140a1808c 415c pop r12
140a1808e 415d pop r13
140a18090 415e pop r14
140a18092 415f pop r15
140a18094 5d pop rbp
140a18095 c3 ret 
140a18096 662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a180a0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a180a5 55 push rbp
140a180a6 4157 push r15
140a180a8 4156 push r14
140a180aa 4155 push r13
140a180ac 4154 push r12
140a180ae 56 push rsi
140a180af 57 push rdi
140a180b0 53 push rbx
140a180b1 4883ec58 sub rsp, 0x58
140a180b5 488daa80000000 lea rbp, [rdx + 0x80]
140a180bc 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a180c1 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a180c6 488d8da0010000 lea rcx, [rbp + 0x1a0]
140a180cd e8deeee7ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a180d2 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a180d7 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a180dc 4883c458 add rsp, 0x58
140a180e0 5b pop rbx
140a180e1 5f pop rdi
140a180e2 5e pop rsi
140a180e3 415c pop r12
140a180e5 415d pop r13
140a180e7 415e pop r14
140a180e9 415f pop r15
140a180eb 5d pop rbp
140a180ec c3 ret 
140a180ed 0f1f00 nop dword ptr [rax]
140a180f0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a180f5 55 push rbp
140a180f6 4157 push r15
140a180f8 4156 push r14
140a180fa 4155 push r13
140a180fc 4154 push r12
140a180fe 56 push rsi
140a180ff 57 push rdi
140a18100 53 push rbx
140a18101 4883ec58 sub rsp, 0x58
140a18105 488daa80000000 lea rbp, [rdx + 0x80]
140a1810c 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a18111 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a18116 488d4d10 lea rcx, [rbp + 0x10]
140a1811a e891eee7ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a1811f 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a18124 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a18129 4883c458 add rsp, 0x58
140a1812d 5b pop rbx
140a1812e 5f pop rdi
140a1812f 5e pop rsi
140a18130 415c pop r12
140a18132 415d pop r13
140a18134 415e pop r14
140a18136 415f pop r15
140a18138 5d pop rbp
140a18139 c3 ret 
140a1813a 660f1f440000 nop word ptr [rax + rax]
140a18140 4889542410 mov qword ptr [rsp + 0x10], rdx
140a18145 55 push rbp
140a18146 4157 push r15
140a18148 4156 push r14
140a1814a 4155 push r13
140a1814c 4154 push r12
140a1814e 56 push rsi
140a1814f 57 push rdi
140a18150 53 push rbx
140a18151 4883ec58 sub rsp, 0x58
140a18155 488daa80000000 lea rbp, [rdx + 0x80]
140a1815c 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a18161 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a18166 488d8da0010000 lea rcx, [rbp + 0x1a0]
140a1816d e83eeee7ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a18172 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a18177 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a1817c 4883c458 add rsp, 0x58
140a18180 5b pop rbx
140a18181 5f pop rdi
140a18182 5e pop rsi
140a18183 415c pop r12
140a18185 415d pop r13
140a18187 415e pop r14
140a18189 415f pop r15
140a1818b 5d pop rbp
140a1818c c3 ret 
140a1818d 0f1f00 nop dword ptr [rax]
140a18190 4889542410 mov qword ptr [rsp + 0x10], rdx
140a18195 55 push rbp
140a18196 4157 push r15
140a18198 4156 push r14
140a1819a 4155 push r13
140a1819c 4154 push r12
140a1819e 56 push rsi
140a1819f 57 push rdi
140a181a0 53 push rbx
140a181a1 4883ec58 sub rsp, 0x58
140a181a5 488daa80000000 lea rbp, [rdx + 0x80]
140a181ac 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a181b1 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a181b6 488d4d10 lea rcx, [rbp + 0x10]
140a181ba e8f1ede7ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a181bf 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a181c4 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a181c9 4883c458 add rsp, 0x58
140a181cd 5b pop rbx
140a181ce 5f pop rdi
140a181cf 5e pop rsi
140a181d0 415c pop r12
140a181d2 415d pop r13
140a181d4 415e pop r14
140a181d6 415f pop r15
140a181d8 5d pop rbp
140a181d9 c3 ret 
140a181da 660f1f440000 nop word ptr [rax + rax]
140a181e0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a181e5 55 push rbp
140a181e6 4157 push r15
140a181e8 4156 push r14
140a181ea 4155 push r13
140a181ec 4154 push r12
140a181ee 56 push rsi
140a181ef 57 push rdi
140a181f0 53 push rbx
140a181f1 4883ec58 sub rsp, 0x58
140a181f5 488daa80000000 lea rbp, [rdx + 0x80]
140a181fc 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a18201 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a18206 488d8da0010000 lea rcx, [rbp + 0x1a0]
140a1820d e89eede7ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a18212 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a18217 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a1821c 4883c458 add rsp, 0x58
140a18220 5b pop rbx
140a18221 5f pop rdi
140a18222 5e pop rsi
140a18223 415c pop r12
140a18225 415d pop r13
140a18227 415e pop r14
140a18229 415f pop r15
140a1822b 5d pop rbp
140a1822c c3 ret 
140a1822d 0f1f00 nop dword ptr [rax]
140a18230 4889542410 mov qword ptr [rsp + 0x10], rdx
140a18235 55 push rbp
140a18236 4157 push r15
140a18238 4156 push r14
140a1823a 4155 push r13
140a1823c 4154 push r12
140a1823e 56 push rsi
140a1823f 57 push rdi
140a18240 53 push rbx
140a18241 4883ec58 sub rsp, 0x58
140a18245 488daa80000000 lea rbp, [rdx + 0x80]
140a1824c 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a18251 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a18256 488d4d10 lea rcx, [rbp + 0x10]
140a1825a e851ede7ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a1825f 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a18264 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a18269 4883c458 add rsp, 0x58
140a1826d 5b pop rbx
140a1826e 5f pop rdi
140a1826f 5e pop rsi
140a18270 415c pop r12
140a18272 415d pop r13
140a18274 415e pop r14
140a18276 415f pop r15
140a18278 5d pop rbp
140a18279 c3 ret 
140a1827a 660f1f440000 nop word ptr [rax + rax]
140a18280 4889542410 mov qword ptr [rsp + 0x10], rdx
140a18285 55 push rbp
140a18286 4157 push r15
140a18288 4156 push r14
140a1828a 4155 push r13
140a1828c 4154 push r12
140a1828e 56 push rsi
140a1828f 57 push rdi
140a18290 53 push rbx
140a18291 4883ec58 sub rsp, 0x58
140a18295 488daa80000000 lea rbp, [rdx + 0x80]
140a1829c 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a182a1 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a182a6 488d8da0010000 lea rcx, [rbp + 0x1a0]
140a182ad e8feece7ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a182b2 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a182b7 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a182bc 4883c458 add rsp, 0x58
140a182c0 5b pop rbx
140a182c1 5f pop rdi
140a182c2 5e pop rsi
140a182c3 415c pop r12
140a182c5 415d pop r13
140a182c7 415e pop r14
140a182c9 415f pop r15
140a182cb 5d pop rbp
140a182cc c3 ret 
140a182cd 0f1f00 nop dword ptr [rax]
140a182d0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a182d5 55 push rbp
140a182d6 4157 push r15
140a182d8 4156 push r14
140a182da 4155 push r13
140a182dc 4154 push r12
140a182de 56 push rsi
140a182df 57 push rdi
140a182e0 53 push rbx
140a182e1 4883ec58 sub rsp, 0x58
140a182e5 488daa80000000 lea rbp, [rdx + 0x80]
140a182ec 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a182f1 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a182f6 488d4d10 lea rcx, [rbp + 0x10]
140a182fa e8b1ece7ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a182ff 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a18304 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a18309 4883c458 add rsp, 0x58
140a1830d 5b pop rbx
140a1830e 5f pop rdi
140a1830f 5e pop rsi
140a18310 415c pop r12
140a18312 415d pop r13
140a18314 415e pop r14
140a18316 415f pop r15
140a18318 5d pop rbp
140a18319 c3 ret 

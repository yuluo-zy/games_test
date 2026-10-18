1409014e0: push rbp
1409014e1: push r15
1409014e3: push r14
1409014e5: push r13
1409014e7: push r12
1409014e9: push rsi
1409014ea: push rdi
1409014eb: push rbx
1409014ec: sub rsp, 0x5d8
1409014f3: lea rbp, [rsp + 0x80]
1409014fb: movaps xmmword ptr [rbp + 0x540], xmm6
140901502: mov qword ptr [rbp + 0x538], 0xfffffffffffffffe
14090150d: mov r13, rdx
140901510: mov qword ptr [rbp + 0x1f8], rcx
140901517: lea r15, [rdx + 0x38]
14090151b: movabs rdx, 0x85c987d23d58d2de
140901525: movabs r8, 0x2ec6ab231f77203b
14090152f: mov rcx, r15
140901532: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
140901537: test rax, rax
14090153a: je 0x140902928
140901540: cmp qword ptr [r13 + 0x218], rdx
140901547: jbe 0x140902928
14090154d: mov rax, qword ptr [r13 + 0x210]
140901554: mov rax, qword ptr [rax + rdx*8]
140901558: test rax, rax
14090155b: je 0x140902928
140901561: mov rdi, qword ptr [r13 + 0x1e0]
140901568: not rax
14090156b: lea rax, [rax + rax*2]
14090156f: shl rax, 5
140901573: cmp qword ptr [rdi + rax + 0x20], 0
140901579: je 0x140902928
14090157f: add rdi, rax
140901582: mov rax, qword ptr [rdi + 0x10]
140901586: mov qword ptr [rbp + 0x1e0], rax
14090158d: mov r14d, dword ptr [r13 + 0x5b0]
140901594: movabs rdx, 0xbb8761bcc7907109
14090159e: movabs r8, 0xdc4ef498cea8673a
1409015a8: mov rcx, r15
1409015ab: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409015b0: test rax, rax
1409015b3: je 0x140902934
1409015b9: cmp qword ptr [r13 + 0x218], rdx
1409015c0: jbe 0x140902934
1409015c6: mov rax, qword ptr [r13 + 0x210]
1409015cd: mov rax, qword ptr [rax + rdx*8]
1409015d1: test rax, rax
1409015d4: je 0x140902934
1409015da: mov rsi, qword ptr [r13 + 0x1e0]
1409015e1: not rax
1409015e4: lea rax, [rax + rax*2]
1409015e8: shl rax, 5
1409015ec: cmp qword ptr [rsi + rax + 0x20], 0
1409015f2: je 0x140902934
1409015f8: add rsi, rax
1409015fb: mov rax, qword ptr [rsi + 0x10]
1409015ff: mov qword ptr [rbp + 0x1e8], rax
140901606: mov eax, dword ptr [r13 + 0x5b0]
14090160d: mov dword ptr [rbp + 0x524], eax
140901613: movabs rdx, 0x86486f3e82eb6bbe
14090161d: movabs r8, 0x4d7091c2f9bfa189
140901627: mov rcx, r15
14090162a: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14090162f: test rax, rax
140901632: je 0x140902940
140901638: cmp qword ptr [r13 + 0x218], rdx
14090163f: jbe 0x140902940
140901645: mov rax, qword ptr [r13 + 0x210]
14090164c: mov rax, qword ptr [rax + rdx*8]
140901650: test rax, rax
140901653: je 0x140902940
140901659: mov rbx, qword ptr [r13 + 0x1e0]
140901660: not rax
140901663: lea rax, [rax + rax*2]
140901667: shl rax, 5
14090166b: cmp qword ptr [rbx + rax + 0x20], 0
140901671: je 0x140902940
140901677: mov dword ptr [rbp + 0x528], r14d
14090167e: mov qword ptr [rbp + 0x1f0], rdi
140901685: add rbx, rax
140901688: mov r12, qword ptr [rbx + 0x10]
14090168c: mov eax, dword ptr [r13 + 0x5b0]
140901693: mov dword ptr [rbp + 0x52c], eax
140901699: movabs rdx, 0x6dc90dcfd23786fa
1409016a3: movabs r8, 0xed952bd348f66c04
1409016ad: mov rcx, r15
1409016b0: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409016b5: test rax, rax
1409016b8: je 0x14090294c
1409016be: cmp qword ptr [r13 + 0x218], rdx
1409016c5: jbe 0x14090294c
1409016cb: mov rax, qword ptr [r13 + 0x210]
1409016d2: mov rax, qword ptr [rax + rdx*8]
1409016d6: test rax, rax
1409016d9: je 0x14090294c
1409016df: mov rdi, qword ptr [r13 + 0x1e0]
1409016e6: not rax
1409016e9: lea rax, [rax + rax*2]
1409016ed: shl rax, 5
1409016f1: cmp qword ptr [rdi + rax + 0x20], 0
1409016f7: je 0x14090294c
1409016fd: add rdi, rax
140901700: mov r14, qword ptr [rdi + 0x10]
140901704: mov eax, dword ptr [r13 + 0x5b0]
14090170b: mov dword ptr [rbp + 0x530], eax
140901711: movabs rdx, 0xcda6af2f6afb130e
14090171b: movabs r8, 0x36e88a927bd70aa
140901725: mov rcx, r15
140901728: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14090172d: test rax, rax
140901730: je 0x140902958
140901736: cmp qword ptr [r13 + 0x218], rdx
14090173d: jbe 0x140902958
140901743: mov rax, qword ptr [r13 + 0x210]
14090174a: mov rcx, qword ptr [rax + rdx*8]
14090174e: test rcx, rcx
140901751: je 0x140902958
140901757: mov rax, qword ptr [r13 + 0x1e0]
14090175e: not rcx
140901761: lea rdx, [rcx + rcx*2]
140901765: shl rdx, 5
140901769: cmp qword ptr [rax + rdx + 0x20], 0
14090176f: je 0x140902958
140901775: mov r15, r13
140901778: mov rcx, qword ptr [rbp + 0x1e0]
14090177f: cmp byte ptr [rcx + 0x29b], 0
140901786: mov r13, qword ptr [rbp + 0x1e8]
14090178d: jne 0x1409017a8
14090178f: cmp byte ptr [r13 + 0x29b], 1
140901797: je 0x1409017a8
140901799: cmp byte ptr [r12 + 0x29b], 0
1409017a2: je 0x140902904
1409017a8: mov rax, qword ptr [rbp + 0x1f0]
1409017af: mov edx, dword ptr [rbp + 0x528]
1409017b5: mov dword ptr [rax + 0x5c], edx
1409017b8: mov byte ptr [rcx + 0x29b], 0
1409017bf: add rcx, 0x98
1409017c6: mov r8d, 0x200
1409017cc: xor edx, edx
1409017ce: call 0x142923c20 ; ?
1409017d3: mov eax, dword ptr [rbp + 0x524]
1409017d9: mov dword ptr [rsi + 0x5c], eax
1409017dc: mov byte ptr [r13 + 0x29b], 0
1409017e4: add r13, 0x98
1409017eb: mov r8d, 0x200
1409017f1: mov rcx, r13
1409017f4: xor edx, edx
1409017f6: call 0x142923c20 ; ?
1409017fb: mov eax, dword ptr [rbp + 0x52c]
140901801: mov dword ptr [rbx + 0x5c], eax
140901804: mov byte ptr [r12 + 0x29b], 0
14090180d: add r12, 0x98
140901814: mov r8d, 0x200
14090181a: mov rcx, r12
14090181d: xor edx, edx
14090181f: call 0x142923c20 ; ?
140901824: mov eax, dword ptr [rbp + 0x530]
14090182a: mov dword ptr [rdi + 0x5c], eax
14090182d: mov byte ptr [r14 + 0x29b], 0
140901835: add r14, 0x98
14090183c: mov r8d, 0x200
140901842: mov rcx, r14
140901845: xor edx, edx
140901847: call 0x142923c20 ; ?
14090184c: mov r13, qword ptr [rbp + 0x1f8]
140901853: mov qword ptr [rsp + 0x30], r13
140901858: mov r12, r15
14090185b: mov qword ptr [rsp + 0x28], r15
140901860: mov qword ptr [rsp + 0x20], 0x26
140901869: lea rdx, [rip + 0x21a86d8]
140901870: lea r9, [rip + 0x21a86ec]
140901877: lea rbx, [rbp + 0x390]
14090187e: mov r8d, 0x1b
140901884: mov rcx, rbx
140901887: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
14090188c: mov byte ptr [rbp + 0x50], 1
140901890: mov dword ptr [rbp + 0x200], 2
14090189a: lea rax, [rbp + 0x200]
1409018a1: mov qword ptr [rsp + 0x20], rax
1409018a6: lea rcx, [rbp + 0x50]
1409018aa: movabs r8, 0xed502fff6dc4021f
1409018b4: mov rdx, rbx
1409018b7: mov r9b, 0x19
1409018ba: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1409018bf: lea rbx, [rbp + 0x200]
1409018c6: lea r14, [rbp + 0x390]
1409018cd: mov r8d, 0x190
1409018d3: mov rcx, rbx
1409018d6: mov rdx, r14
1409018d9: call 0x142923c00 ; ?
1409018de: mov byte ptr [rbp + 0x50], 2
1409018e2: mov dword ptr [rbp + 0x390], 2
1409018ec: mov qword ptr [rsp + 0x20], r14
1409018f1: lea rcx, [rbp + 0x50]
1409018f5: movabs r8, 0x921351e406bfb529
1409018ff: mov rdx, rbx
140901902: mov r9b, 0x19
140901905: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
14090190a: lea rbx, [rbp + 0x390]
140901911: lea r14, [rbp + 0x200]
140901918: mov r8d, 0x190
14090191e: mov rcx, rbx
140901921: mov rdx, r14
140901924: call 0x142923c00 ; ?
140901929: mov byte ptr [rbp + 0x50], 3
14090192d: mov dword ptr [rbp + 0x200], 2
140901937: mov qword ptr [rsp + 0x20], r14
14090193c: lea rcx, [rbp + 0x50]
140901940: movabs r8, 0x898e75329fb493b2
14090194a: mov rdx, rbx
14090194d: mov r9b, 0x19
140901950: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140901955: lea rbx, [rbp + 0x200]
14090195c: lea r14, [rbp + 0x390]
140901963: mov r8d, 0x190
140901969: mov rcx, rbx
14090196c: mov rdx, r14
14090196f: call 0x142923c00 ; ?
140901974: mov byte ptr [rbp + 0x50], 4
140901978: mov dword ptr [rbp + 0x390], 2
140901982: mov qword ptr [rsp + 0x20], r14
140901987: lea rcx, [rbp + 0x50]
14090198b: movabs r8, 0x60ec324b9742b267
140901995: mov rdx, rbx
140901998: mov r9b, 0x19
14090199b: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1409019a0: lea rbx, [rbp + 0x390]
1409019a7: lea r14, [rbp + 0x200]
1409019ae: mov r8d, 0x190
1409019b4: mov rcx, rbx
1409019b7: mov rdx, r14
1409019ba: call 0x142923c00 ; ?
1409019bf: mov byte ptr [rbp + 0x50], 5
1409019c3: mov dword ptr [rbp + 0x200], 2
1409019cd: mov qword ptr [rsp + 0x20], r14
1409019d2: lea rcx, [rbp + 0x50]
1409019d6: movabs r8, 0xb5e1aef5c3a339a4
1409019e0: mov rdx, rbx
1409019e3: mov r9b, 0x32
1409019e6: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1409019eb: lea rcx, [rbp + 0x200]
1409019f2: lea rdx, [rbp + 0x390]
1409019f9: mov r8d, 0x190
1409019ff: call 0x142923c00 ; ?
140901a04: movups xmm0, xmmword ptr [rbp + 0x2a0]
140901a0b: movaps xmmword ptr [rbp + 0x90], xmm0
140901a12: movups xmm0, xmmword ptr [rbp + 0x290]
140901a19: movaps xmmword ptr [rbp + 0x80], xmm0
140901a20: movups xmm0, xmmword ptr [rbp + 0x280]
140901a27: movaps xmmword ptr [rbp + 0x70], xmm0
140901a2b: movups xmm0, xmmword ptr [rbp + 0x270]
140901a32: movaps xmmword ptr [rbp + 0x60], xmm0
140901a36: movups xmm0, xmmword ptr [rbp + 0x260]
140901a3d: movaps xmmword ptr [rbp + 0x50], xmm0
140901a41: mov rax, qword ptr [rbp + 0x338]
140901a48: mov rcx, qword ptr [rbp + 0x380]
140901a4f: mov rdx, qword ptr [rbp + 0x210]
140901a56: mov qword ptr [rbp + 0x3a0], rdx
140901a5d: movups xmm0, xmmword ptr [rbp + 0x200]
140901a64: movaps xmmword ptr [rbp + 0x390], xmm0
140901a6b: movups xmm0, xmmword ptr [rbp + 0x218]
140901a72: movaps xmmword ptr [rbp + 0x3c0], xmm0
140901a79: mov rdx, qword ptr [rbp + 0x228]
140901a80: mov qword ptr [rbp + 0x3d0], rdx
140901a87: movups xmm0, xmmword ptr [rbp + 0x230]
140901a8e: movups xmmword ptr [rbp + 0x3a8], xmm0
140901a95: mov rdx, qword ptr [rbp + 0x240]
140901a9c: mov qword ptr [rbp + 0x3b8], rdx
140901aa3: movups xmm0, xmmword ptr [rbp + 0x2b0]
140901aaa: movups xmm1, xmmword ptr [rbp + 0x2c0]
140901ab1: movups xmm2, xmmword ptr [rbp + 0x2d0]
140901ab8: movups xmm3, xmmword ptr [rbp + 0x2e0]
140901abf: movups xmmword ptr [rbp + 0x408], xmm0
140901ac6: movups xmmword ptr [rbp + 0x418], xmm1
140901acd: movups xmmword ptr [rbp + 0x428], xmm2
140901ad4: movups xmmword ptr [rbp + 0x438], xmm3
140901adb: movups xmm0, xmmword ptr [rbp + 0x2f0]
140901ae2: movups xmmword ptr [rbp + 0x448], xmm0
140901ae9: movups xmm0, xmmword ptr [rbp + 0x300]
140901af0: movups xmmword ptr [rbp + 0x458], xmm0
140901af7: mov rdx, qword ptr [rbp + 0x258]
140901afe: mov qword ptr [rbp + 0x3e8], rdx
140901b05: movups xmm0, xmmword ptr [rbp + 0x248]
140901b0c: movups xmmword ptr [rbp + 0x3d8], xmm0
140901b13: mov qword ptr [rbp + 0x470], rcx
140901b1a: mov qword ptr [rbp + 0x468], rax
140901b21: movaps xmm6, xmmword ptr [rip + 0x21a09d8]
140901b28: movaps xmmword ptr [rbp + 0x3f0], xmm6
140901b2f: lea rcx, [rbp + 0x50]
140901b33: lea rdx, [rbp + 0x390]
140901b3a: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140901b3f: cmp byte ptr [rbp + 0x336], 8
140901b46: je 0x140901b54
140901b48: lea rcx, [rbp + 0x320]
140901b4f: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140901b54: lea rcx, [rbp + 0x50]
140901b58: movabs rbx, 0x20000000200
140901b62: mov rdx, rbx
140901b65: mov r8d, 0x4c
140901b6b: call 0x140bd52a0 ; rhapsody::raw::texture::TextureDesc::new_2d
140901b70: movups xmm0, xmmword ptr [rbp + 0x50]
140901b74: movups xmm1, xmmword ptr [rbp + 0x60]
140901b78: movaps xmmword ptr [rbp + 0x210], xmm1
140901b7f: movaps xmmword ptr [rbp + 0x200], xmm0
140901b86: lea rcx, [rbp + 0x3a0]
140901b8d: lea rdx, [rbp + 0x200]
140901b94: call 0x140bd6ce0 ; rhapsody::rg::resource::impl$4::from
140901b99: lea rax, [rip + 0x21a8419]
140901ba0: mov qword ptr [rbp + 0x390], rax
140901ba7: mov qword ptr [rbp + 0x398], 0xb
140901bb2: lea rdx, [rbp + 0x390]
140901bb9: mov rcx, r13
140901bbc: call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
140901bc1: movups xmm0, xmmword ptr [rbp + 0x50]
140901bc5: movups xmm1, xmmword ptr [rbp + 0x60]
140901bc9: movups xmmword ptr [rbp + 0x30], xmm0
140901bcd: movups xmmword ptr [rbp + 0x40], xmm1
140901bd1: mov dword ptr [rbp + 0x2c], eax
140901bd4: lea rcx, [rbp + 0x50]
140901bd8: mov rdx, rbx
140901bdb: mov r8d, 0x4c
140901be1: call 0x140bd52a0 ; rhapsody::raw::texture::TextureDesc::new_2d
140901be6: movups xmm0, xmmword ptr [rbp + 0x50]
140901bea: movups xmm1, xmmword ptr [rbp + 0x60]
140901bee: movaps xmmword ptr [rbp + 0x210], xmm1
140901bf5: movaps xmmword ptr [rbp + 0x200], xmm0
140901bfc: lea rcx, [rbp + 0x3a0]
140901c03: lea rdx, [rbp + 0x200]
140901c0a: call 0x140bd6ce0 ; rhapsody::rg::resource::impl$4::from
140901c0f: lea rax, [rip + 0x21a83ae]
140901c16: mov qword ptr [rbp + 0x390], rax
140901c1d: mov qword ptr [rbp + 0x398], 0xb
140901c28: lea rbx, [rbp + 0x390]
140901c2f: mov rcx, r13
140901c32: mov rdx, rbx
140901c35: call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
140901c3a: movups xmm0, xmmword ptr [rbp + 0x50]
140901c3e: movups xmm1, xmmword ptr [rbp + 0x60]
140901c42: movups xmmword ptr [rbp + 0xc], xmm0
140901c46: movups xmmword ptr [rbp + 0x1c], xmm1
140901c4a: mov dword ptr [rbp + 8], eax
140901c4d: mov qword ptr [rsp + 0x30], r13
140901c52: mov qword ptr [rsp + 0x28], r12
140901c57: mov qword ptr [rsp + 0x20], 0x23
140901c60: lea rdx, [rip + 0x21a8390]
140901c67: lea r9, [rip + 0x21a83a0]
140901c6e: mov r8d, 0x17
140901c74: mov rcx, r14
140901c77: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140901c7c: mov byte ptr [rbp + 0x50], 9
140901c80: mov dword ptr [rbp + 0x390], 2
140901c8a: mov qword ptr [rsp + 0x20], rbx
140901c8f: lea rbx, [rbp + 0x50]
140901c93: movabs r8, 0x61b08b5e0470c5f2
140901c9d: mov rcx, rbx
140901ca0: mov rdx, r14
140901ca3: mov r9b, 0x19
140901ca6: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140901cab: lea r14, [rbp + 0x390]
140901cb2: lea rdx, [rbp + 0x200]
140901cb9: mov r8d, 0x190
140901cbf: mov rcx, r14
140901cc2: call 0x142923c00 ; ?
140901cc7: mov byte ptr [rbp - 0x1c], 0xa
140901ccb: mov dword ptr [rbp + 0x50], 2
140901cd2: mov qword ptr [rsp + 0x20], rbx
140901cd7: lea rcx, [rbp - 0x1c]
140901cdb: movabs r8, 0x297d1e4cdae8f657
140901ce5: mov rdx, r14
140901ce8: mov r9b, 0x19
140901ceb: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140901cf0: lea r14, [rbp + 0x50]
140901cf4: lea rbx, [rbp + 0x390]
140901cfb: mov r8d, 0x190
140901d01: mov rcx, r14
140901d04: mov rdx, rbx
140901d07: call 0x142923c00 ; ?
140901d0c: mov byte ptr [rbp + 0x200], 0xd
140901d13: mov dword ptr [rbp + 0x390], 2
140901d1d: mov qword ptr [rsp + 0x20], rbx
140901d22: lea rbx, [rbp + 0x200]
140901d29: movabs r8, 0xab24287344aba8c1
140901d33: mov rcx, rbx
140901d36: mov rdx, r14
140901d39: mov r9b, 0x19
140901d3c: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140901d41: lea r14, [rbp + 0x390]
140901d48: lea rdx, [rbp + 0x50]
140901d4c: mov r8d, 0x190
140901d52: mov rcx, r14
140901d55: call 0x142923c00 ; ?
140901d5a: mov byte ptr [rbp + 0x50], 5
140901d5e: mov dword ptr [rbp + 0x200], 2
140901d68: mov qword ptr [rsp + 0x20], rbx
140901d6d: lea rcx, [rbp + 0x50]
140901d71: movabs r8, 0xed502fff6dc4021f
140901d7b: mov rdx, r14
140901d7e: mov r9b, 0x19
140901d81: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140901d86: mov r8d, 0x190
140901d8c: mov rcx, rbx
140901d8f: mov rdx, r14
140901d92: call 0x142923c00 ; ?
140901d97: lea rsi, [rbp + 8]
140901d9b: mov qword ptr [rbp + 0x50], rsi
140901d9f: mov dword ptr [rbp + 0x390], 2
140901da9: mov qword ptr [rsp + 0x20], r14
140901dae: lea rcx, [rbp + 0x50]
140901db2: movabs r8, 0x1ca51a73373208db
140901dbc: mov rdx, rbx
140901dbf: mov r9b, 0x32
140901dc2: call 0x1408edd00 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
140901dc7: lea rcx, [rbp + 0x390]
140901dce: lea rdx, [rbp + 0x200]
140901dd5: mov r8d, 0x190
140901ddb: call 0x142923c00 ; ?
140901de0: movups xmm0, xmmword ptr [rbp + 0x430]
140901de7: movaps xmmword ptr [rbp + 0x90], xmm0
140901dee: movups xmm0, xmmword ptr [rbp + 0x420]
140901df5: movaps xmmword ptr [rbp + 0x80], xmm0
140901dfc: movups xmm0, xmmword ptr [rbp + 0x410]
140901e03: movaps xmmword ptr [rbp + 0x70], xmm0
140901e07: movups xmm0, xmmword ptr [rbp + 0x400]
140901e0e: movaps xmmword ptr [rbp + 0x60], xmm0
140901e12: movups xmm0, xmmword ptr [rbp + 0x3f0]
140901e19: movaps xmmword ptr [rbp + 0x50], xmm0
140901e1d: mov rax, qword ptr [rbp + 0x4c8]
140901e24: mov rcx, qword ptr [rbp + 0x510]
140901e2b: mov rdx, qword ptr [rbp + 0x3a0]
140901e32: mov qword ptr [rbp + 0x210], rdx
140901e39: movups xmm0, xmmword ptr [rbp + 0x390]
140901e40: movaps xmmword ptr [rbp + 0x200], xmm0
140901e47: movups xmm0, xmmword ptr [rbp + 0x3a8]
140901e4e: movaps xmmword ptr [rbp + 0x230], xmm0
140901e55: mov rdx, qword ptr [rbp + 0x3b8]
140901e5c: mov qword ptr [rbp + 0x240], rdx
140901e63: movups xmm0, xmmword ptr [rbp + 0x3c0]
140901e6a: movups xmmword ptr [rbp + 0x218], xmm0
140901e71: mov rdx, qword ptr [rbp + 0x3d0]
140901e78: mov qword ptr [rbp + 0x228], rdx
140901e7f: movups xmm0, xmmword ptr [rbp + 0x440]
140901e86: movups xmm1, xmmword ptr [rbp + 0x450]
140901e8d: movups xmm2, xmmword ptr [rbp + 0x460]
140901e94: movups xmm3, xmmword ptr [rbp + 0x470]
140901e9b: movups xmmword ptr [rbp + 0x278], xmm0
140901ea2: movups xmmword ptr [rbp + 0x288], xmm1
140901ea9: movups xmmword ptr [rbp + 0x298], xmm2
140901eb0: movups xmmword ptr [rbp + 0x2a8], xmm3
140901eb7: movups xmm0, xmmword ptr [rbp + 0x480]
140901ebe: movups xmmword ptr [rbp + 0x2b8], xmm0
140901ec5: movups xmm0, xmmword ptr [rbp + 0x490]
140901ecc: movups xmmword ptr [rbp + 0x2c8], xmm0
140901ed3: mov rdx, qword ptr [rbp + 0x3e8]
140901eda: mov qword ptr [rbp + 0x258], rdx
140901ee1: movups xmm0, xmmword ptr [rbp + 0x3d8]
140901ee8: movups xmmword ptr [rbp + 0x248], xmm0
140901eef: mov qword ptr [rbp + 0x2e0], rcx
140901ef6: mov qword ptr [rbp + 0x2d8], rax
140901efd: movaps xmmword ptr [rbp + 0x260], xmm6
140901f04: lea rcx, [rbp + 0x50]
140901f08: lea rdx, [rbp + 0x200]
140901f0f: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140901f14: cmp byte ptr [rbp + 0x4c6], 8
140901f1b: je 0x140901f29
140901f1d: lea rcx, [rbp + 0x4b0]
140901f24: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140901f29: mov qword ptr [rsp + 0x30], r13
140901f2e: mov qword ptr [rsp + 0x28], r12
140901f33: mov qword ptr [rsp + 0x20], 0x18
140901f3c: lea rdx, [rip + 0x21a808c]
140901f43: lea r9, [rip + 0x21a8095]
140901f4a: lea rcx, [rbp + 0x390]
140901f51: mov r8d, 0x10
140901f57: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140901f5c: cmp byte ptr [rbp + 0x4c6], 8
140901f63: je 0x140901f8a
140901f65: lea rcx, [rbp + 0x4a0]
140901f6c: mov edx, 0xe8
140901f71: add rdx, qword ptr [rbp + 0x4c8]
140901f78: movabs r8, 0xd91ff8372094bc22
140901f82: mov r9b, 1
140901f85: call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
140901f8a: lea rcx, [rbp + 0x200]
140901f91: lea rdx, [rbp + 0x390]
140901f98: mov r8d, 0x190
140901f9e: call 0x142923c00 ; ?
140901fa3: cmp byte ptr [rbp + 0x336], 8
140901faa: je 0x140901fd6
140901fac: lea rcx, [rbp + 0x310]
140901fb3: mov edx, 0xe8
140901fb8: add rdx, qword ptr [rbp + 0x338]
140901fbf: movabs r8, 0xdc022c4f3092a794
140901fc9: movss xmm3, dword ptr [rip + 0x202c71f]
140901fd1: call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
140901fd6: lea rbx, [rbp + 0x390]
140901fdd: lea r14, [rbp + 0x200]
140901fe4: mov r8d, 0x190
140901fea: mov rcx, rbx
140901fed: mov rdx, r14
140901ff0: call 0x142923c00 ; ?
140901ff5: mov qword ptr [rbp + 0x50], rsi
140901ff9: mov dword ptr [rbp + 0x200], 2
140902003: mov qword ptr [rsp + 0x20], r14
140902008: lea rcx, [rbp + 0x50]
14090200c: movabs r8, 0xd82d7a0adb9164c2
140902016: mov rdx, rbx
140902019: mov r9b, 0x19
14090201c: call 0x1408ee240 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
140902021: lea rbx, [rbp + 0x200]
140902028: lea r14, [rbp + 0x390]
14090202f: mov r8d, 0x190
140902035: mov rcx, rbx
140902038: mov rdx, r14
14090203b: call 0x142923c00 ; ?
140902040: lea rdi, [rbp + 0x2c]
140902044: mov qword ptr [rbp + 0x50], rdi
140902048: mov dword ptr [rbp + 0x390], 2
140902052: mov qword ptr [rsp + 0x20], r14
140902057: lea rcx, [rbp + 0x50]
14090205b: movabs r8, 0x1ca51a73373208db
140902065: mov rdx, rbx
140902068: mov r9b, 0x32
14090206b: call 0x1408edd00 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
140902070: lea rcx, [rbp + 0x390]
140902077: lea rdx, [rbp + 0x200]
14090207e: mov r8d, 0x190
140902084: call 0x142923c00 ; ?
140902089: movups xmm0, xmmword ptr [rbp + 0x430]
140902090: movaps xmmword ptr [rbp + 0x90], xmm0
140902097: movups xmm0, xmmword ptr [rbp + 0x420]
14090209e: movaps xmmword ptr [rbp + 0x80], xmm0
1409020a5: movups xmm0, xmmword ptr [rbp + 0x410]
1409020ac: movaps xmmword ptr [rbp + 0x70], xmm0
1409020b0: movups xmm0, xmmword ptr [rbp + 0x400]
1409020b7: movaps xmmword ptr [rbp + 0x60], xmm0
1409020bb: movups xmm0, xmmword ptr [rbp + 0x3f0]
1409020c2: movaps xmmword ptr [rbp + 0x50], xmm0
1409020c6: mov rax, qword ptr [rbp + 0x4c8]
1409020cd: mov rcx, qword ptr [rbp + 0x510]
1409020d4: mov rdx, qword ptr [rbp + 0x3a0]
1409020db: mov qword ptr [rbp + 0x210], rdx
1409020e2: movups xmm0, xmmword ptr [rbp + 0x390]
1409020e9: movaps xmmword ptr [rbp + 0x200], xmm0
1409020f0: movups xmm0, xmmword ptr [rbp + 0x3a8]
1409020f7: movaps xmmword ptr [rbp + 0x230], xmm0
1409020fe: mov rdx, qword ptr [rbp + 0x3b8]
140902105: mov qword ptr [rbp + 0x240], rdx
14090210c: movups xmm0, xmmword ptr [rbp + 0x3c0]
140902113: movups xmmword ptr [rbp + 0x218], xmm0
14090211a: mov rdx, qword ptr [rbp + 0x3d0]
140902121: mov qword ptr [rbp + 0x228], rdx
140902128: movups xmm0, xmmword ptr [rbp + 0x440]
14090212f: movups xmm1, xmmword ptr [rbp + 0x450]
140902136: movups xmm2, xmmword ptr [rbp + 0x460]
14090213d: movups xmm3, xmmword ptr [rbp + 0x470]
140902144: movups xmmword ptr [rbp + 0x278], xmm0
14090214b: movups xmmword ptr [rbp + 0x288], xmm1
140902152: movups xmmword ptr [rbp + 0x298], xmm2
140902159: movups xmmword ptr [rbp + 0x2a8], xmm3
140902160: movups xmm0, xmmword ptr [rbp + 0x480]
140902167: movups xmmword ptr [rbp + 0x2b8], xmm0
14090216e: movups xmm0, xmmword ptr [rbp + 0x490]
140902175: movups xmmword ptr [rbp + 0x2c8], xmm0
14090217c: mov rdx, qword ptr [rbp + 0x3e8]
140902183: mov qword ptr [rbp + 0x258], rdx
14090218a: movups xmm0, xmmword ptr [rbp + 0x3d8]
140902191: movups xmmword ptr [rbp + 0x248], xmm0
140902198: mov qword ptr [rbp + 0x2e0], rcx
14090219f: mov qword ptr [rbp + 0x2d8], rax
1409021a6: movaps xmmword ptr [rbp + 0x260], xmm6
1409021ad: lea rcx, [rbp + 0x50]
1409021b1: lea rdx, [rbp + 0x200]
1409021b8: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
1409021bd: cmp byte ptr [rbp + 0x4c6], 8
1409021c4: je 0x1409021d2
1409021c6: lea rcx, [rbp + 0x4b0]
1409021cd: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1409021d2: mov qword ptr [rsp + 0x30], r13
1409021d7: mov qword ptr [rsp + 0x28], r12
1409021dc: mov qword ptr [rsp + 0x20], 0x18
1409021e5: lea rdx, [rip + 0x21a7de3]
1409021ec: lea r9, [rip + 0x21a7dec]
1409021f3: lea rcx, [rbp + 0x390]
1409021fa: mov r8d, 0x10
140902200: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140902205: cmp byte ptr [rbp + 0x4c6], 8
14090220c: je 0x140902233
14090220e: lea rcx, [rbp + 0x4a0]
140902215: mov edx, 0xe8
14090221a: add rdx, qword ptr [rbp + 0x4c8]
140902221: movabs r8, 0xd91ff8372094bc22
14090222b: xor r9d, r9d
14090222e: call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
140902233: lea rcx, [rbp + 0x200]
14090223a: lea rdx, [rbp + 0x390]
140902241: mov r8d, 0x190
140902247: call 0x142923c00 ; ?
14090224c: cmp byte ptr [rbp + 0x336], 8
140902253: je 0x14090227f
140902255: lea rcx, [rbp + 0x310]
14090225c: mov edx, 0xe8
140902261: add rdx, qword ptr [rbp + 0x338]
140902268: movabs r8, 0xdc022c4f3092a794
140902272: movss xmm3, dword ptr [rip + 0x202c476]
14090227a: call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
14090227f: lea rbx, [rbp + 0x390]
140902286: lea r14, [rbp + 0x200]
14090228d: mov r8d, 0x190
140902293: mov rcx, rbx
140902296: mov rdx, r14
140902299: call 0x142923c00 ; ?
14090229e: mov qword ptr [rbp + 0x50], rdi
1409022a2: mov dword ptr [rbp + 0x200], 2
1409022ac: mov qword ptr [rsp + 0x20], r14
1409022b1: lea rcx, [rbp + 0x50]
1409022b5: movabs r8, 0xd82d7a0adb9164c2
1409022bf: mov rdx, rbx
1409022c2: mov r9b, 0x19
1409022c5: call 0x1408ee240 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
1409022ca: lea rbx, [rbp + 0x200]
1409022d1: lea r14, [rbp + 0x390]
1409022d8: mov r8d, 0x190
1409022de: mov rcx, rbx
1409022e1: mov rdx, r14
1409022e4: call 0x142923c00 ; ?
1409022e9: mov byte ptr [rbp + 0x50], 5
1409022ed: mov dword ptr [rbp + 0x390], 2
1409022f7: mov qword ptr [rsp + 0x20], r14
1409022fc: lea rcx, [rbp + 0x50]
140902300: movabs r8, 0x1ca51a73373208db
14090230a: mov rdx, rbx
14090230d: mov r9b, 0x32
140902310: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140902315: lea rcx, [rbp + 0x390]
14090231c: lea rdx, [rbp + 0x200]
140902323: mov r8d, 0x190
140902329: call 0x142923c00 ; ?
14090232e: movups xmm0, xmmword ptr [rbp + 0x430]
140902335: movaps xmmword ptr [rbp + 0x90], xmm0
14090233c: movups xmm0, xmmword ptr [rbp + 0x420]
140902343: movaps xmmword ptr [rbp + 0x80], xmm0
14090234a: movups xmm0, xmmword ptr [rbp + 0x410]
140902351: movaps xmmword ptr [rbp + 0x70], xmm0
140902355: movups xmm0, xmmword ptr [rbp + 0x400]
14090235c: movaps xmmword ptr [rbp + 0x60], xmm0
140902360: movups xmm0, xmmword ptr [rbp + 0x3f0]
140902367: movaps xmmword ptr [rbp + 0x50], xmm0
14090236b: mov rax, qword ptr [rbp + 0x4c8]
140902372: mov rcx, qword ptr [rbp + 0x510]
140902379: mov rdx, qword ptr [rbp + 0x3a0]
140902380: mov qword ptr [rbp + 0x210], rdx
140902387: movups xmm0, xmmword ptr [rbp + 0x390]
14090238e: movaps xmmword ptr [rbp + 0x200], xmm0
140902395: movups xmm0, xmmword ptr [rbp + 0x3a8]
14090239c: movaps xmmword ptr [rbp + 0x230], xmm0
1409023a3: mov rdx, qword ptr [rbp + 0x3b8]
1409023aa: mov qword ptr [rbp + 0x240], rdx
1409023b1: movups xmm0, xmmword ptr [rbp + 0x3c0]
1409023b8: movups xmmword ptr [rbp + 0x218], xmm0
1409023bf: mov rdx, qword ptr [rbp + 0x3d0]
1409023c6: mov qword ptr [rbp + 0x228], rdx
1409023cd: movups xmm0, xmmword ptr [rbp + 0x440]
1409023d4: movups xmm1, xmmword ptr [rbp + 0x450]
1409023db: movups xmm2, xmmword ptr [rbp + 0x460]
1409023e2: movups xmm3, xmmword ptr [rbp + 0x470]
1409023e9: movups xmmword ptr [rbp + 0x278], xmm0
1409023f0: movups xmmword ptr [rbp + 0x288], xmm1
1409023f7: movups xmmword ptr [rbp + 0x298], xmm2
1409023fe: movups xmmword ptr [rbp + 0x2a8], xmm3
140902405: movups xmm0, xmmword ptr [rbp + 0x480]
14090240c: movups xmmword ptr [rbp + 0x2b8], xmm0
140902413: movups xmm0, xmmword ptr [rbp + 0x490]
14090241a: movups xmmword ptr [rbp + 0x2c8], xmm0
140902421: mov rdx, qword ptr [rbp + 0x3e8]
140902428: mov qword ptr [rbp + 0x258], rdx
14090242f: movups xmm0, xmmword ptr [rbp + 0x3d8]
140902436: movups xmmword ptr [rbp + 0x248], xmm0
14090243d: mov qword ptr [rbp + 0x2e0], rcx
140902444: mov qword ptr [rbp + 0x2d8], rax
14090244b: movaps xmmword ptr [rbp + 0x260], xmm6
140902452: lea rcx, [rbp + 0x50]
140902456: lea r13, [rbp + 0x200]
14090245d: mov rdx, r13
140902460: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140902465: cmp byte ptr [rbp + 0x4c6], 8
14090246c: je 0x14090247a
14090246e: lea rcx, [rbp + 0x4b0]
140902475: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
14090247a: lea r12, [rbp - 0x40]
14090247e: mov rcx, r12
140902481: mov rdi, qword ptr [rbp + 0x1f8]
140902488: mov rdx, rdi
14090248b: mov rsi, r15
14090248e: mov r8, r15
140902491: mov r9b, 1
140902494: call 0x140903350 ; country_core::systems::render_loop::compute_passes::compute_path_masks::blur_path_mask
140902499: lea r15, [rbp - 0x1c]
14090249d: mov rcx, r15
1409024a0: mov rdx, rdi
1409024a3: mov r8, rsi
1409024a6: mov r9b, 2
1409024a9: call 0x140903350 ; country_core::systems::render_loop::compute_passes::compute_path_masks::blur_path_mask
1409024ae: lea r14, [rbp + 0x2c]
1409024b2: mov rcx, r14
1409024b5: mov rdx, rdi
1409024b8: mov r8, rsi
1409024bb: mov r9b, 3
1409024be: call 0x140903350 ; country_core::systems::render_loop::compute_passes::compute_path_masks::blur_path_mask
1409024c3: lea rbx, [rbp + 8]
1409024c7: mov rcx, rbx
1409024ca: mov rdx, rdi
1409024cd: mov r8, rsi
1409024d0: mov r9b, 4
1409024d3: call 0x140903350 ; country_core::systems::render_loop::compute_passes::compute_path_masks::blur_path_mask
1409024d8: mov qword ptr [rsp + 0x30], rdi
1409024dd: mov qword ptr [rsp + 0x28], rsi
1409024e2: mov qword ptr [rsp + 0x20], 0x1f
1409024eb: lea rdx, [rip + 0x21a7a97]
1409024f2: lea r9, [rip + 0x21a7aa1]
1409024f9: lea rsi, [rbp + 0x390]
140902500: mov r8d, 0x11
140902506: mov rcx, rsi
140902509: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
14090250e: mov byte ptr [rbp + 0x50], 0x13
140902512: mov dword ptr [rbp + 0x200], 2
14090251c: mov qword ptr [rsp + 0x20], r13
140902521: lea rcx, [rbp + 0x50]
140902525: movabs r8, 0x6ad63ccd18c8f041
14090252f: mov rdx, rsi
140902532: mov r9b, 0x19
140902535: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
14090253a: lea rsi, [rbp + 0x200]
140902541: lea rdi, [rbp + 0x390]
140902548: mov r8d, 0x190
14090254e: mov rcx, rsi
140902551: mov rdx, rdi
140902554: call 0x142923c00 ; ?
140902559: mov qword ptr [rbp + 0x50], r12
14090255d: mov dword ptr [rbp + 0x390], 2
140902567: mov qword ptr [rsp + 0x20], rdi
14090256c: lea rcx, [rbp + 0x50]
140902570: movabs r8, 0x8d1f9969c90d5348
14090257a: mov rdx, rsi
14090257d: mov r9b, 0x19
140902580: call 0x1408ee240 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
140902585: lea rsi, [rbp + 0x390]
14090258c: lea rdi, [rbp + 0x200]
140902593: mov r8d, 0x190
140902599: mov rcx, rsi
14090259c: mov rdx, rdi
14090259f: call 0x142923c00 ; ?
1409025a4: mov qword ptr [rbp + 0x50], r15
1409025a8: mov dword ptr [rbp + 0x200], 2
1409025b2: mov qword ptr [rsp + 0x20], rdi
1409025b7: lea rcx, [rbp + 0x50]
1409025bb: movabs r8, 0xf92d7b12a65bc8e2
1409025c5: mov rdx, rsi
1409025c8: mov r9b, 0x19
1409025cb: call 0x1408ee240 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
1409025d0: lea rsi, [rbp + 0x200]
1409025d7: lea rdi, [rbp + 0x390]
1409025de: mov r8d, 0x190
1409025e4: mov rcx, rsi
1409025e7: mov rdx, rdi
1409025ea: call 0x142923c00 ; ?
1409025ef: mov qword ptr [rbp + 0x50], r14
1409025f3: mov dword ptr [rbp + 0x390], 2
1409025fd: mov qword ptr [rsp + 0x20], rdi
140902602: lea rcx, [rbp + 0x50]
140902606: movabs r8, 0xa88a3e9b7a2f3451
140902610: mov rdx, rsi
140902613: mov r9b, 0x19
140902616: call 0x1408ee240 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
14090261b: lea rsi, [rbp + 0x390]
140902622: lea rdi, [rbp + 0x200]
140902629: mov r8d, 0x190
14090262f: mov rcx, rsi
140902632: mov rdx, rdi
140902635: call 0x142923c00 ; ?
14090263a: mov qword ptr [rbp + 0x50], rbx
14090263e: mov dword ptr [rbp + 0x200], 2
140902648: mov qword ptr [rsp + 0x20], rdi
14090264d: lea rcx, [rbp + 0x50]
140902651: movabs r8, 0x39b7b6f8247cf130
14090265b: mov rdx, rsi
14090265e: mov r9b, 0x19
140902661: call 0x1408ee240 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
140902666: mov r8d, 0x190
14090266c: mov rcx, rdi
14090266f: mov rdx, rsi
140902672: call 0x142923c00 ; ?
140902677: mov byte ptr [rbp + 0x50], 9
14090267b: mov dword ptr [rbp + 0x390], 2
140902685: mov qword ptr [rsp + 0x20], rsi
14090268a: lea rsi, [rbp + 0x50]
14090268e: movabs r8, 0x61b08b5e0470c5f2
140902698: mov rcx, rsi
14090269b: mov rdx, rdi
14090269e: mov r9b, 0x19
1409026a1: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1409026a6: lea rdi, [rbp + 0x390]
1409026ad: lea rdx, [rbp + 0x200]
1409026b4: mov r8d, 0x190
1409026ba: mov rcx, rdi
1409026bd: call 0x142923c00 ; ?
1409026c2: mov byte ptr [rbp + 0x537], 0xa
1409026c9: mov dword ptr [rbp + 0x50], 2
1409026d0: mov qword ptr [rsp + 0x20], rsi
1409026d5: lea rcx, [rbp + 0x537]
1409026dc: movabs r8, 0x297d1e4cdae8f657
1409026e6: mov rdx, rdi
1409026e9: mov r9b, 0x19
1409026ec: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1409026f1: lea rdi, [rbp + 0x50]
1409026f5: lea rsi, [rbp + 0x390]
1409026fc: mov r8d, 0x190
140902702: mov rcx, rdi
140902705: mov rdx, rsi
140902708: call 0x142923c00 ; ?
14090270d: mov byte ptr [rbp + 0x200], 6
140902714: mov dword ptr [rbp + 0x390], 2
14090271e: mov qword ptr [rsp + 0x20], rsi
140902723: lea rsi, [rbp + 0x200]
14090272a: movabs r8, 0xd7f481303b104abc
140902734: mov rcx, rsi
140902737: mov rdx, rdi
14090273a: mov r9b, 0x32
14090273d: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140902742: lea rdi, [rbp + 0x390]
140902749: lea rdx, [rbp + 0x50]
14090274d: mov r8d, 0x190
140902753: mov rcx, rdi
140902756: call 0x142923c00 ; ?
14090275b: mov byte ptr [rbp + 0x50], 7
14090275f: mov dword ptr [rbp + 0x200], 2
140902769: mov qword ptr [rsp + 0x20], rsi
14090276e: lea rcx, [rbp + 0x50]
140902772: movabs r8, 0x3ec9710e6411d8d1
14090277c: mov rdx, rdi
14090277f: mov r9b, 0x32
140902782: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140902787: lea rcx, [rbp + 0x200]
14090278e: lea rdx, [rbp + 0x390]
140902795: mov r8d, 0x190
14090279b: call 0x142923c00 ; ?
1409027a0: movups xmm0, xmmword ptr [rbp + 0x2a0]
1409027a7: movaps xmmword ptr [rbp + 0x90], xmm0
1409027ae: movups xmm0, xmmword ptr [rbp + 0x290]
1409027b5: movaps xmmword ptr [rbp + 0x80], xmm0
1409027bc: movups xmm0, xmmword ptr [rbp + 0x280]
1409027c3: movaps xmmword ptr [rbp + 0x70], xmm0
1409027c7: movups xmm0, xmmword ptr [rbp + 0x270]
1409027ce: movaps xmmword ptr [rbp + 0x60], xmm0
1409027d2: movups xmm0, xmmword ptr [rbp + 0x260]
1409027d9: movaps xmmword ptr [rbp + 0x50], xmm0
1409027dd: mov rax, qword ptr [rbp + 0x338]
1409027e4: mov rcx, qword ptr [rbp + 0x380]
1409027eb: mov rdx, qword ptr [rbp + 0x210]
1409027f2: mov qword ptr [rbp + 0x3a0], rdx
1409027f9: movups xmm0, xmmword ptr [rbp + 0x200]
140902800: movaps xmmword ptr [rbp + 0x390], xmm0
140902807: movups xmm0, xmmword ptr [rbp + 0x218]
14090280e: movaps xmmword ptr [rbp + 0x3c0], xmm0
140902815: mov rdx, qword ptr [rbp + 0x228]
14090281c: mov qword ptr [rbp + 0x3d0], rdx
140902823: movups xmm0, xmmword ptr [rbp + 0x230]
14090282a: movups xmmword ptr [rbp + 0x3a8], xmm0
140902831: mov rdx, qword ptr [rbp + 0x240]
140902838: mov qword ptr [rbp + 0x3b8], rdx
14090283f: movups xmm0, xmmword ptr [rbp + 0x2b0]
140902846: movups xmm1, xmmword ptr [rbp + 0x2c0]
14090284d: movups xmm2, xmmword ptr [rbp + 0x2d0]
140902854: movups xmm3, xmmword ptr [rbp + 0x2e0]
14090285b: movups xmmword ptr [rbp + 0x408], xmm0
140902862: movups xmmword ptr [rbp + 0x418], xmm1
140902869: movups xmmword ptr [rbp + 0x428], xmm2
140902870: movups xmmword ptr [rbp + 0x438], xmm3
140902877: movups xmm0, xmmword ptr [rbp + 0x2f0]
14090287e: movups xmmword ptr [rbp + 0x448], xmm0
140902885: movups xmm0, xmmword ptr [rbp + 0x300]
14090288c: movups xmmword ptr [rbp + 0x458], xmm0
140902893: mov rdx, qword ptr [rbp + 0x258]
14090289a: mov qword ptr [rbp + 0x3e8], rdx
1409028a1: movups xmm0, xmmword ptr [rbp + 0x248]
1409028a8: movups xmmword ptr [rbp + 0x3d8], xmm0
1409028af: mov qword ptr [rbp + 0x470], rcx
1409028b6: mov qword ptr [rbp + 0x468], rax
1409028bd: movaps xmmword ptr [rbp + 0x3f0], xmm6
1409028c4: lea rcx, [rbp + 0x50]
1409028c8: lea rdx, [rbp + 0x390]
1409028cf: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
1409028d4: cmp byte ptr [rbp + 0x336], 8
1409028db: je 0x1409028e9
1409028dd: lea rcx, [rbp + 0x320]
1409028e4: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1409028e9: movaps xmm6, xmmword ptr [rbp + 0x540]
1409028f0: add rsp, 0x5d8
1409028f7: pop rbx
1409028f8: pop rdi
1409028f9: pop rsi
1409028fa: pop r12
1409028fc: pop r13
1409028fe: pop r14
140902900: pop r15
140902902: pop rbp
140902903: ret 
140902904: add rax, rdx
140902907: mov rax, qword ptr [rax + 0x10]
14090290b: mov rdx, qword ptr [rax + 0x30]
14090290f: or rdx, qword ptr [rax + 0x10]
140902913: jne 0x1409017a8
140902919: cmp byte ptr [r14 + 0x29b], 0
140902921: je 0x1409028e9
140902923: jmp 0x1409017a8
140902928: lea rcx, [rip + 0x21a75a1]
14090292f: call 0x1428d9310 ; core::option::unwrap_failed
140902934: lea rcx, [rip + 0x21a75ad]
14090293b: call 0x1428d9310 ; core::option::unwrap_failed
140902940: lea rcx, [rip + 0x21a75b9]
140902947: call 0x1428d9310 ; core::option::unwrap_failed
14090294c: lea rcx, [rip + 0x21a75c5]
140902953: call 0x1428d9310 ; core::option::unwrap_failed
140902958: lea rcx, [rip + 0x21a75d1]
14090295f: call 0x1428d9310 ; core::option::unwrap_failed
140902964: int3 
140902965: nop word ptr cs:[rax + rax]
140902970: mov qword ptr [rsp + 0x10], rdx
140902975: push rbp
140902976: push r15
140902978: push r14
14090297a: push r13
14090297c: push r12
14090297e: push rsi
14090297f: push rdi
140902980: push rbx
140902981: sub rsp, 0x48
140902985: lea rbp, [rdx + 0x80]
14090298c: movaps xmmword ptr [rsp + 0x30], xmm6
140902991: cmp byte ptr [rbp + 0x336], 8
140902998: je 0x1409029a6
14090299a: lea rcx, [rbp + 0x320]
1409029a1: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1409029a6: movaps xmm6, xmmword ptr [rsp + 0x30]
1409029ab: add rsp, 0x48
1409029af: pop rbx
1409029b0: pop rdi
1409029b1: pop rsi
1409029b2: pop r12
1409029b4: pop r13
1409029b6: pop r14
1409029b8: pop r15
1409029ba: pop rbp
1409029bb: ret 
1409029bc: nop dword ptr [rax]
1409029c0: mov qword ptr [rsp + 0x10], rdx
1409029c5: push rbp
1409029c6: push r15
1409029c8: push r14
1409029ca: push r13
1409029cc: push r12
1409029ce: push rsi
1409029cf: push rdi
1409029d0: push rbx
1409029d1: sub rsp, 0x48
1409029d5: lea rbp, [rdx + 0x80]
1409029dc: movaps xmmword ptr [rsp + 0x30], xmm6
1409029e1: lea rcx, [rbp + 0x200]
1409029e8: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1409029ed: movaps xmm6, xmmword ptr [rsp + 0x30]
1409029f2: add rsp, 0x48
1409029f6: pop rbx
1409029f7: pop rdi
1409029f8: pop rsi
1409029f9: pop r12
1409029fb: pop r13
1409029fd: pop r14
1409029ff: pop r15
140902a01: pop rbp
140902a02: ret 
140902a03: nop word ptr cs:[rax + rax]
140902a10: mov qword ptr [rsp + 0x10], rdx
140902a15: push rbp
140902a16: push r15
140902a18: push r14
140902a1a: push r13
140902a1c: push r12
140902a1e: push rsi
140902a1f: push rdi
140902a20: push rbx
140902a21: sub rsp, 0x48
140902a25: lea rbp, [rdx + 0x80]
140902a2c: movaps xmmword ptr [rsp + 0x30], xmm6
140902a31: lea rcx, [rbp + 0x390]
140902a38: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902a3d: movaps xmm6, xmmword ptr [rsp + 0x30]
140902a42: add rsp, 0x48
140902a46: pop rbx
140902a47: pop rdi
140902a48: pop rsi
140902a49: pop r12
140902a4b: pop r13
140902a4d: pop r14
140902a4f: pop r15
140902a51: pop rbp
140902a52: ret 
140902a53: nop word ptr cs:[rax + rax]
140902a60: mov qword ptr [rsp + 0x10], rdx
140902a65: push rbp
140902a66: push r15
140902a68: push r14
140902a6a: push r13
140902a6c: push r12
140902a6e: push rsi
140902a6f: push rdi
140902a70: push rbx
140902a71: sub rsp, 0x48
140902a75: lea rbp, [rdx + 0x80]
140902a7c: movaps xmmword ptr [rsp + 0x30], xmm6
140902a81: lea rcx, [rbp + 0x50]
140902a85: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902a8a: movaps xmm6, xmmword ptr [rsp + 0x30]
140902a8f: add rsp, 0x48
140902a93: pop rbx
140902a94: pop rdi
140902a95: pop rsi
140902a96: pop r12
140902a98: pop r13
140902a9a: pop r14
140902a9c: pop r15
140902a9e: pop rbp
140902a9f: ret 
140902aa0: mov qword ptr [rsp + 0x10], rdx
140902aa5: push rbp
140902aa6: push r15
140902aa8: push r14
140902aaa: push r13
140902aac: push r12
140902aae: push rsi
140902aaf: push rdi
140902ab0: push rbx
140902ab1: sub rsp, 0x48
140902ab5: lea rbp, [rdx + 0x80]
140902abc: movaps xmmword ptr [rsp + 0x30], xmm6
140902ac1: lea rcx, [rbp + 0x390]
140902ac8: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902acd: movaps xmm6, xmmword ptr [rsp + 0x30]
140902ad2: add rsp, 0x48
140902ad6: pop rbx
140902ad7: pop rdi
140902ad8: pop rsi
140902ad9: pop r12
140902adb: pop r13
140902add: pop r14
140902adf: pop r15
140902ae1: pop rbp
140902ae2: ret 
140902ae3: nop word ptr cs:[rax + rax]
140902af0: mov qword ptr [rsp + 0x10], rdx
140902af5: push rbp
140902af6: push r15
140902af8: push r14
140902afa: push r13
140902afc: push r12
140902afe: push rsi
140902aff: push rdi
140902b00: push rbx
140902b01: sub rsp, 0x48
140902b05: lea rbp, [rdx + 0x80]
140902b0c: movaps xmmword ptr [rsp + 0x30], xmm6
140902b11: lea rcx, [rbp + 0x200]
140902b18: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902b1d: movaps xmm6, xmmword ptr [rsp + 0x30]
140902b22: add rsp, 0x48
140902b26: pop rbx
140902b27: pop rdi
140902b28: pop rsi
140902b29: pop r12
140902b2b: pop r13
140902b2d: pop r14
140902b2f: pop r15
140902b31: pop rbp
140902b32: ret 
140902b33: nop word ptr cs:[rax + rax]
140902b40: mov qword ptr [rsp + 0x10], rdx
140902b45: push rbp
140902b46: push r15
140902b48: push r14
140902b4a: push r13
140902b4c: push r12
140902b4e: push rsi
140902b4f: push rdi
140902b50: push rbx
140902b51: sub rsp, 0x48
140902b55: lea rbp, [rdx + 0x80]
140902b5c: movaps xmmword ptr [rsp + 0x30], xmm6
140902b61: cmp byte ptr [rbp + 0x4c6], 8
140902b68: je 0x140902b76
140902b6a: lea rcx, [rbp + 0x4b0]
140902b71: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140902b76: movaps xmm6, xmmword ptr [rsp + 0x30]
140902b7b: add rsp, 0x48
140902b7f: pop rbx
140902b80: pop rdi
140902b81: pop rsi
140902b82: pop r12
140902b84: pop r13
140902b86: pop r14
140902b88: pop r15
140902b8a: pop rbp
140902b8b: ret 
140902b8c: nop dword ptr [rax]
140902b90: mov qword ptr [rsp + 0x10], rdx
140902b95: push rbp
140902b96: push r15
140902b98: push r14
140902b9a: push r13
140902b9c: push r12
140902b9e: push rsi
140902b9f: push rdi
140902ba0: push rbx
140902ba1: sub rsp, 0x48
140902ba5: lea rbp, [rdx + 0x80]
140902bac: movaps xmmword ptr [rsp + 0x30], xmm6
140902bb1: lea rcx, [rbp + 0x390]
140902bb8: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902bbd: movaps xmm6, xmmword ptr [rsp + 0x30]
140902bc2: add rsp, 0x48
140902bc6: pop rbx
140902bc7: pop rdi
140902bc8: pop rsi
140902bc9: pop r12
140902bcb: pop r13
140902bcd: pop r14
140902bcf: pop r15
140902bd1: pop rbp
140902bd2: ret 
140902bd3: nop word ptr cs:[rax + rax]
140902be0: mov qword ptr [rsp + 0x10], rdx
140902be5: push rbp
140902be6: push r15
140902be8: push r14
140902bea: push r13
140902bec: push r12
140902bee: push rsi
140902bef: push rdi
140902bf0: push rbx
140902bf1: sub rsp, 0x48
140902bf5: lea rbp, [rdx + 0x80]
140902bfc: movaps xmmword ptr [rsp + 0x30], xmm6
140902c01: lea rcx, [rbp + 0x200]
140902c08: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902c0d: movaps xmm6, xmmword ptr [rsp + 0x30]
140902c12: add rsp, 0x48
140902c16: pop rbx
140902c17: pop rdi
140902c18: pop rsi
140902c19: pop r12
140902c1b: pop r13
140902c1d: pop r14
140902c1f: pop r15
140902c21: pop rbp
140902c22: ret 
140902c23: nop word ptr cs:[rax + rax]
140902c30: mov qword ptr [rsp + 0x10], rdx
140902c35: push rbp
140902c36: push r15
140902c38: push r14
140902c3a: push r13
140902c3c: push r12
140902c3e: push rsi
140902c3f: push rdi
140902c40: push rbx
140902c41: sub rsp, 0x48
140902c45: lea rbp, [rdx + 0x80]
140902c4c: movaps xmmword ptr [rsp + 0x30], xmm6
140902c51: lea rcx, [rbp + 0x390]
140902c58: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902c5d: movaps xmm6, xmmword ptr [rsp + 0x30]
140902c62: add rsp, 0x48
140902c66: pop rbx
140902c67: pop rdi
140902c68: pop rsi
140902c69: pop r12
140902c6b: pop r13
140902c6d: pop r14
140902c6f: pop r15
140902c71: pop rbp
140902c72: ret 
140902c73: nop word ptr cs:[rax + rax]
140902c80: mov qword ptr [rsp + 0x10], rdx
140902c85: push rbp
140902c86: push r15
140902c88: push r14
140902c8a: push r13
140902c8c: push r12
140902c8e: push rsi
140902c8f: push rdi
140902c90: push rbx
140902c91: sub rsp, 0x48
140902c95: lea rbp, [rdx + 0x80]
140902c9c: movaps xmmword ptr [rsp + 0x30], xmm6
140902ca1: lea rcx, [rbp + 0x200]
140902ca8: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902cad: movaps xmm6, xmmword ptr [rsp + 0x30]
140902cb2: add rsp, 0x48
140902cb6: pop rbx
140902cb7: pop rdi
140902cb8: pop rsi
140902cb9: pop r12
140902cbb: pop r13
140902cbd: pop r14
140902cbf: pop r15
140902cc1: pop rbp
140902cc2: ret 
140902cc3: nop word ptr cs:[rax + rax]
140902cd0: mov qword ptr [rsp + 0x10], rdx
140902cd5: push rbp
140902cd6: push r15
140902cd8: push r14
140902cda: push r13
140902cdc: push r12
140902cde: push rsi
140902cdf: push rdi
140902ce0: push rbx
140902ce1: sub rsp, 0x48
140902ce5: lea rbp, [rdx + 0x80]
140902cec: movaps xmmword ptr [rsp + 0x30], xmm6
140902cf1: cmp byte ptr [rbp + 0x4c6], 8
140902cf8: je 0x140902d06
140902cfa: lea rcx, [rbp + 0x4b0]
140902d01: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140902d06: movaps xmm6, xmmword ptr [rsp + 0x30]
140902d0b: add rsp, 0x48
140902d0f: pop rbx
140902d10: pop rdi
140902d11: pop rsi
140902d12: pop r12
140902d14: pop r13
140902d16: pop r14
140902d18: pop r15
140902d1a: pop rbp
140902d1b: ret 
140902d1c: nop dword ptr [rax]
140902d20: mov qword ptr [rsp + 0x10], rdx
140902d25: push rbp
140902d26: push r15
140902d28: push r14
140902d2a: push r13
140902d2c: push r12
140902d2e: push rsi
140902d2f: push rdi
140902d30: push rbx
140902d31: sub rsp, 0x48
140902d35: lea rbp, [rdx + 0x80]
140902d3c: movaps xmmword ptr [rsp + 0x30], xmm6
140902d41: lea rcx, [rbp + 0x390]
140902d48: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902d4d: movaps xmm6, xmmword ptr [rsp + 0x30]
140902d52: add rsp, 0x48
140902d56: pop rbx
140902d57: pop rdi
140902d58: pop rsi
140902d59: pop r12
140902d5b: pop r13
140902d5d: pop r14
140902d5f: pop r15
140902d61: pop rbp
140902d62: ret 
140902d63: nop word ptr cs:[rax + rax]
140902d70: mov qword ptr [rsp + 0x10], rdx
140902d75: push rbp
140902d76: push r15
140902d78: push r14
140902d7a: push r13
140902d7c: push r12
140902d7e: push rsi
140902d7f: push rdi
140902d80: push rbx
140902d81: sub rsp, 0x48
140902d85: lea rbp, [rdx + 0x80]
140902d8c: movaps xmmword ptr [rsp + 0x30], xmm6
140902d91: lea rcx, [rbp + 0x200]
140902d98: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902d9d: movaps xmm6, xmmword ptr [rsp + 0x30]
140902da2: add rsp, 0x48
140902da6: pop rbx
140902da7: pop rdi
140902da8: pop rsi
140902da9: pop r12
140902dab: pop r13
140902dad: pop r14
140902daf: pop r15
140902db1: pop rbp
140902db2: ret 
140902db3: nop word ptr cs:[rax + rax]
140902dc0: mov qword ptr [rsp + 0x10], rdx
140902dc5: push rbp
140902dc6: push r15
140902dc8: push r14
140902dca: push r13
140902dcc: push r12
140902dce: push rsi
140902dcf: push rdi
140902dd0: push rbx
140902dd1: sub rsp, 0x48
140902dd5: lea rbp, [rdx + 0x80]
140902ddc: movaps xmmword ptr [rsp + 0x30], xmm6
140902de1: lea rcx, [rbp + 0x390]
140902de8: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902ded: movaps xmm6, xmmword ptr [rsp + 0x30]
140902df2: add rsp, 0x48
140902df6: pop rbx
140902df7: pop rdi
140902df8: pop rsi
140902df9: pop r12
140902dfb: pop r13
140902dfd: pop r14
140902dff: pop r15
140902e01: pop rbp
140902e02: ret 
140902e03: nop word ptr cs:[rax + rax]
140902e10: mov qword ptr [rsp + 0x10], rdx
140902e15: push rbp
140902e16: push r15
140902e18: push r14
140902e1a: push r13
140902e1c: push r12
140902e1e: push rsi
140902e1f: push rdi
140902e20: push rbx
140902e21: sub rsp, 0x48
140902e25: lea rbp, [rdx + 0x80]
140902e2c: movaps xmmword ptr [rsp + 0x30], xmm6
140902e31: lea rcx, [rbp + 0x200]
140902e38: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902e3d: movaps xmm6, xmmword ptr [rsp + 0x30]
140902e42: add rsp, 0x48
140902e46: pop rbx
140902e47: pop rdi
140902e48: pop rsi
140902e49: pop r12
140902e4b: pop r13
140902e4d: pop r14
140902e4f: pop r15
140902e51: pop rbp
140902e52: ret 
140902e53: nop word ptr cs:[rax + rax]
140902e60: mov qword ptr [rsp + 0x10], rdx
140902e65: push rbp
140902e66: push r15
140902e68: push r14
140902e6a: push r13
140902e6c: push r12
140902e6e: push rsi
140902e6f: push rdi
140902e70: push rbx
140902e71: sub rsp, 0x48
140902e75: lea rbp, [rdx + 0x80]
140902e7c: movaps xmmword ptr [rsp + 0x30], xmm6
140902e81: cmp byte ptr [rbp + 0x4c6], 8
140902e88: je 0x140902e96
140902e8a: lea rcx, [rbp + 0x4b0]
140902e91: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140902e96: movaps xmm6, xmmword ptr [rsp + 0x30]
140902e9b: add rsp, 0x48
140902e9f: pop rbx
140902ea0: pop rdi
140902ea1: pop rsi
140902ea2: pop r12
140902ea4: pop r13
140902ea6: pop r14
140902ea8: pop r15
140902eaa: pop rbp
140902eab: ret 
140902eac: nop dword ptr [rax]
140902eb0: mov qword ptr [rsp + 0x10], rdx
140902eb5: push rbp
140902eb6: push r15
140902eb8: push r14
140902eba: push r13
140902ebc: push r12
140902ebe: push rsi
140902ebf: push rdi
140902ec0: push rbx
140902ec1: sub rsp, 0x48
140902ec5: lea rbp, [rdx + 0x80]
140902ecc: movaps xmmword ptr [rsp + 0x30], xmm6
140902ed1: lea rcx, [rbp + 0x390]
140902ed8: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902edd: movaps xmm6, xmmword ptr [rsp + 0x30]
140902ee2: add rsp, 0x48
140902ee6: pop rbx
140902ee7: pop rdi
140902ee8: pop rsi
140902ee9: pop r12
140902eeb: pop r13
140902eed: pop r14
140902eef: pop r15
140902ef1: pop rbp
140902ef2: ret 
140902ef3: nop word ptr cs:[rax + rax]
140902f00: mov qword ptr [rsp + 0x10], rdx
140902f05: push rbp
140902f06: push r15
140902f08: push r14
140902f0a: push r13
140902f0c: push r12
140902f0e: push rsi
140902f0f: push rdi
140902f10: push rbx
140902f11: sub rsp, 0x48
140902f15: lea rbp, [rdx + 0x80]
140902f1c: movaps xmmword ptr [rsp + 0x30], xmm6
140902f21: lea rcx, [rbp + 0x200]
140902f28: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902f2d: movaps xmm6, xmmword ptr [rsp + 0x30]
140902f32: add rsp, 0x48
140902f36: pop rbx
140902f37: pop rdi
140902f38: pop rsi
140902f39: pop r12
140902f3b: pop r13
140902f3d: pop r14
140902f3f: pop r15
140902f41: pop rbp
140902f42: ret 
140902f43: nop word ptr cs:[rax + rax]
140902f50: mov qword ptr [rsp + 0x10], rdx
140902f55: push rbp
140902f56: push r15
140902f58: push r14
140902f5a: push r13
140902f5c: push r12
140902f5e: push rsi
140902f5f: push rdi
140902f60: push rbx
140902f61: sub rsp, 0x48
140902f65: lea rbp, [rdx + 0x80]
140902f6c: movaps xmmword ptr [rsp + 0x30], xmm6
140902f71: lea rcx, [rbp + 0x390]
140902f78: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902f7d: movaps xmm6, xmmword ptr [rsp + 0x30]
140902f82: add rsp, 0x48
140902f86: pop rbx
140902f87: pop rdi
140902f88: pop rsi
140902f89: pop r12
140902f8b: pop r13
140902f8d: pop r14
140902f8f: pop r15
140902f91: pop rbp
140902f92: ret 
140902f93: nop word ptr cs:[rax + rax]
140902fa0: mov qword ptr [rsp + 0x10], rdx
140902fa5: push rbp
140902fa6: push r15
140902fa8: push r14
140902faa: push r13
140902fac: push r12
140902fae: push rsi
140902faf: push rdi
140902fb0: push rbx
140902fb1: sub rsp, 0x48
140902fb5: lea rbp, [rdx + 0x80]
140902fbc: movaps xmmword ptr [rsp + 0x30], xmm6
140902fc1: lea rcx, [rbp + 0x200]
140902fc8: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140902fcd: movaps xmm6, xmmword ptr [rsp + 0x30]
140902fd2: add rsp, 0x48
140902fd6: pop rbx
140902fd7: pop rdi
140902fd8: pop rsi
140902fd9: pop r12
140902fdb: pop r13
140902fdd: pop r14
140902fdf: pop r15
140902fe1: pop rbp
140902fe2: ret 
140902fe3: nop word ptr cs:[rax + rax]
140902ff0: mov qword ptr [rsp + 0x10], rdx
140902ff5: push rbp
140902ff6: push r15
140902ff8: push r14
140902ffa: push r13
140902ffc: push r12
140902ffe: push rsi
140902fff: push rdi
140903000: push rbx
140903001: sub rsp, 0x48
140903005: lea rbp, [rdx + 0x80]
14090300c: movaps xmmword ptr [rsp + 0x30], xmm6
140903011: lea rcx, [rbp + 0x390]
140903018: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14090301d: movaps xmm6, xmmword ptr [rsp + 0x30]
140903022: add rsp, 0x48
140903026: pop rbx
140903027: pop rdi
140903028: pop rsi
140903029: pop r12
14090302b: pop r13
14090302d: pop r14
14090302f: pop r15
140903031: pop rbp
140903032: ret 
140903033: nop word ptr cs:[rax + rax]
140903040: mov qword ptr [rsp + 0x10], rdx
140903045: push rbp
140903046: push r15
140903048: push r14
14090304a: push r13
14090304c: push r12
14090304e: push rsi
14090304f: push rdi
140903050: push rbx
140903051: sub rsp, 0x48
140903055: lea rbp, [rdx + 0x80]
14090305c: movaps xmmword ptr [rsp + 0x30], xmm6
140903061: lea rcx, [rbp + 0x200]
140903068: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14090306d: movaps xmm6, xmmword ptr [rsp + 0x30]
140903072: add rsp, 0x48
140903076: pop rbx
140903077: pop rdi
140903078: pop rsi
140903079: pop r12
14090307b: pop r13
14090307d: pop r14
14090307f: pop r15
140903081: pop rbp
140903082: ret 
140903083: nop word ptr cs:[rax + rax]
140903090: mov qword ptr [rsp + 0x10], rdx
140903095: push rbp
140903096: push r15
140903098: push r14
14090309a: push r13
14090309c: push r12
14090309e: push rsi
14090309f: push rdi
1409030a0: push rbx
1409030a1: sub rsp, 0x48
1409030a5: lea rbp, [rdx + 0x80]
1409030ac: movaps xmmword ptr [rsp + 0x30], xmm6
1409030b1: lea rcx, [rbp + 0x390]
1409030b8: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1409030bd: movaps xmm6, xmmword ptr [rsp + 0x30]
1409030c2: add rsp, 0x48
1409030c6: pop rbx
1409030c7: pop rdi
1409030c8: pop rsi
1409030c9: pop r12
1409030cb: pop r13
1409030cd: pop r14
1409030cf: pop r15
1409030d1: pop rbp
1409030d2: ret 
1409030d3: nop word ptr cs:[rax + rax]
1409030e0: mov qword ptr [rsp + 0x10], rdx
1409030e5: push rbp
1409030e6: push r15
1409030e8: push r14
1409030ea: push r13
1409030ec: push r12
1409030ee: push rsi
1409030ef: push rdi
1409030f0: push rbx
1409030f1: sub rsp, 0x48
1409030f5: lea rbp, [rdx + 0x80]
1409030fc: movaps xmmword ptr [rsp + 0x30], xmm6
140903101: lea rcx, [rbp + 0x50]
140903105: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14090310a: movaps xmm6, xmmword ptr [rsp + 0x30]
14090310f: add rsp, 0x48
140903113: pop rbx
140903114: pop rdi
140903115: pop rsi
140903116: pop r12
140903118: pop r13
14090311a: pop r14
14090311c: pop r15
14090311e: pop rbp
14090311f: ret 
140903120: mov qword ptr [rsp + 0x10], rdx
140903125: push rbp
140903126: push r15
140903128: push r14
14090312a: push r13
14090312c: push r12
14090312e: push rsi
14090312f: push rdi
140903130: push rbx
140903131: sub rsp, 0x48
140903135: lea rbp, [rdx + 0x80]
14090313c: movaps xmmword ptr [rsp + 0x30], xmm6
140903141: lea rcx, [rbp + 0x390]
140903148: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14090314d: movaps xmm6, xmmword ptr [rsp + 0x30]
140903152: add rsp, 0x48
140903156: pop rbx
140903157: pop rdi
140903158: pop rsi
140903159: pop r12
14090315b: pop r13
14090315d: pop r14
14090315f: pop r15
140903161: pop rbp
140903162: ret 
140903163: nop word ptr cs:[rax + rax]
140903170: mov qword ptr [rsp + 0x10], rdx
140903175: push rbp
140903176: push r15
140903178: push r14
14090317a: push r13
14090317c: push r12
14090317e: push rsi
14090317f: push rdi
140903180: push rbx
140903181: sub rsp, 0x48
140903185: lea rbp, [rdx + 0x80]
14090318c: movaps xmmword ptr [rsp + 0x30], xmm6
140903191: cmp byte ptr [rbp + 0x336], 8
140903198: je 0x1409031a6
14090319a: lea rcx, [rbp + 0x320]
1409031a1: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1409031a6: movaps xmm6, xmmword ptr [rsp + 0x30]
1409031ab: add rsp, 0x48
1409031af: pop rbx
1409031b0: pop rdi
1409031b1: pop rsi
1409031b2: pop r12
1409031b4: pop r13
1409031b6: pop r14
1409031b8: pop r15
1409031ba: pop rbp
1409031bb: ret 
1409031bc: nop dword ptr [rax]
1409031c0: mov qword ptr [rsp + 0x10], rdx
1409031c5: push rbp
1409031c6: push r15
1409031c8: push r14
1409031ca: push r13
1409031cc: push r12
1409031ce: push rsi
1409031cf: push rdi
1409031d0: push rbx
1409031d1: sub rsp, 0x48
1409031d5: lea rbp, [rdx + 0x80]
1409031dc: movaps xmmword ptr [rsp + 0x30], xmm6
1409031e1: lea rcx, [rbp + 0x390]
1409031e8: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1409031ed: movaps xmm6, xmmword ptr [rsp + 0x30]
1409031f2: add rsp, 0x48
1409031f6: pop rbx
1409031f7: pop rdi
1409031f8: pop rsi
1409031f9: pop r12
1409031fb: pop r13
1409031fd: pop r14
1409031ff: pop r15
140903201: pop rbp
140903202: ret 
140903203: nop word ptr cs:[rax + rax]
140903210: mov qword ptr [rsp + 0x10], rdx
140903215: push rbp
140903216: push r15
140903218: push r14
14090321a: push r13
14090321c: push r12
14090321e: push rsi
14090321f: push rdi
140903220: push rbx
140903221: sub rsp, 0x48
140903225: lea rbp, [rdx + 0x80]
14090322c: movaps xmmword ptr [rsp + 0x30], xmm6
140903231: lea rcx, [rbp + 0x200]
140903238: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14090323d: movaps xmm6, xmmword ptr [rsp + 0x30]
140903242: add rsp, 0x48
140903246: pop rbx
140903247: pop rdi
140903248: pop rsi
140903249: pop r12
14090324b: pop r13
14090324d: pop r14
14090324f: pop r15
140903251: pop rbp
140903252: ret 
140903253: nop word ptr cs:[rax + rax]
140903260: mov qword ptr [rsp + 0x10], rdx
140903265: push rbp
140903266: push r15
140903268: push r14
14090326a: push r13
14090326c: push r12
14090326e: push rsi
14090326f: push rdi
140903270: push rbx
140903271: sub rsp, 0x48
140903275: lea rbp, [rdx + 0x80]
14090327c: movaps xmmword ptr [rsp + 0x30], xmm6
140903281: lea rcx, [rbp + 0x390]
140903288: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14090328d: movaps xmm6, xmmword ptr [rsp + 0x30]
140903292: add rsp, 0x48
140903296: pop rbx
140903297: pop rdi
140903298: pop rsi
140903299: pop r12
14090329b: pop r13
14090329d: pop r14
14090329f: pop r15
1409032a1: pop rbp
1409032a2: ret 
1409032a3: nop word ptr cs:[rax + rax]
1409032b0: mov qword ptr [rsp + 0x10], rdx
1409032b5: push rbp
1409032b6: push r15
1409032b8: push r14
1409032ba: push r13
1409032bc: push r12
1409032be: push rsi
1409032bf: push rdi
1409032c0: push rbx
1409032c1: sub rsp, 0x48
1409032c5: lea rbp, [rdx + 0x80]
1409032cc: movaps xmmword ptr [rsp + 0x30], xmm6
1409032d1: lea rcx, [rbp + 0x200]
1409032d8: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1409032dd: movaps xmm6, xmmword ptr [rsp + 0x30]
1409032e2: add rsp, 0x48
1409032e6: pop rbx
1409032e7: pop rdi
1409032e8: pop rsi
1409032e9: pop r12
1409032eb: pop r13
1409032ed: pop r14
1409032ef: pop r15
1409032f1: pop rbp
1409032f2: ret 
1409032f3: nop word ptr cs:[rax + rax]
140903300: mov qword ptr [rsp + 0x10], rdx
140903305: push rbp
140903306: push r15
140903308: push r14
14090330a: push r13
14090330c: push r12
14090330e: push rsi
14090330f: push rdi
140903310: push rbx
140903311: sub rsp, 0x48
140903315: lea rbp, [rdx + 0x80]
14090331c: movaps xmmword ptr [rsp + 0x30], xmm6
140903321: lea rcx, [rbp + 0x390]
140903328: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14090332d: movaps xmm6, xmmword ptr [rsp + 0x30]
140903332: add rsp, 0x48
140903336: pop rbx
140903337: pop rdi
140903338: pop rsi
140903339: pop r12
14090333b: pop r13
14090333d: pop r14
14090333f: pop r15
140903341: pop rbp
140903342: ret 

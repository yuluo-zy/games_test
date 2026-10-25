140a58450: push rbp
140a58451: push r15
140a58453: push r14
140a58455: push rsi
140a58456: push rdi
140a58457: push rbx
140a58458: sub rsp, 0x3f8
140a5845f: lea rbp, [rsp + 0x80]
140a58467: movaps xmmword ptr [rbp + 0x360], xmm6
140a5846e: mov qword ptr [rbp + 0x358], 0xfffffffffffffffe
140a58479: mov rsi, rdx
140a5847c: mov rdi, rcx
140a5847f: lea r14, [rdx + 0x38]
140a58483: movabs rdx, 0xc2b8e941c91a6428
140a5848d: movabs r8, 0x59f3c6d1d3da43a0
140a58497: mov rcx, r14
140a5849a: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
140a5849f: test rax, rax
140a584a2: je 0x140a58e43
140a584a8: cmp qword ptr [rsi + 0x218], rdx
140a584af: jbe 0x140a58e43
140a584b5: mov rax, qword ptr [rsi + 0x210]
140a584bc: mov rcx, qword ptr [rax + rdx*8]
140a584c0: test rcx, rcx
140a584c3: je 0x140a58e43
140a584c9: mov rax, qword ptr [rsi + 0x1e0]
140a584d0: not rcx
140a584d3: lea rcx, [rcx + rcx*2]
140a584d7: shl rcx, 5
140a584db: cmp qword ptr [rax + rcx + 0x20], 0
140a584e1: je 0x140a58e43
140a584e7: add rax, rcx
140a584ea: mov rax, qword ptr [rax + 0x10]
140a584ee: mov eax, dword ptr [rax]
140a584f0: lea edx, [rax - 6]
140a584f3: mov r8, rax
140a584f6: add r8, -5
140a584fa: xor ecx, ecx
140a584fc: cmp edx, 0xf
140a584ff: cmovb rcx, r8
140a58503: mov ebx, 0xffffffff
140a58508: cmp rcx, 5
140a5850c: je 0x140a58520
140a5850e: test rcx, rcx
140a58511: jne 0x140a585cc
140a58517: cmp eax, 5
140a5851a: jne 0x140a585cc
140a58520: movabs rdx, 0xc26ebbeefbfc3261
140a5852a: movabs r8, 0x7a483ee28328d5ec
140a58534: mov rcx, r14
140a58537: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
140a5853c: test rax, rax
140a5853f: je 0x140a58e5b
140a58545: cmp qword ptr [rsi + 0x218], rdx
140a5854c: jbe 0x140a58e5b
140a58552: mov rax, qword ptr [rsi + 0x210]
140a58559: mov rcx, qword ptr [rax + rdx*8]
140a5855d: test rcx, rcx
140a58560: je 0x140a58e5b
140a58566: mov rax, qword ptr [rsi + 0x1e0]
140a5856d: not rcx
140a58570: lea rcx, [rcx + rcx*2]
140a58574: shl rcx, 5
140a58578: cmp qword ptr [rax + rcx + 0x20], 0
140a5857e: je 0x140a58e5b
140a58584: add rax, rcx
140a58587: mov rdx, qword ptr [rax + 0x10]
140a5858b: mov rax, qword ptr [rdx + 0x10]
140a5858f: mov rcx, qword ptr [rdx + 0x30]
140a58593: mov r8, rcx
140a58596: or r8, rax
140a58599: je 0x140a585cc
140a5859b: mov r8, rcx
140a5859e: shl r8, 4
140a585a2: mov r9, rax
140a585a5: shl r9, 4
140a585a9: test rax, rax
140a585ac: mov r10, qword ptr [rdx + 8]
140a585b0: mov rdx, qword ptr [rdx + 0x28]
140a585b4: lea r9, [r10 + r9 - 0x10]
140a585b9: cmove r9, rax
140a585bd: test rcx, rcx
140a585c0: lea rax, [rdx + r8 - 0x10]
140a585c5: cmove rax, r9
140a585c9: mov ebx, dword ptr [rax + 8]
140a585cc: movabs rdx, 0x48b321ef779dc301
140a585d6: movabs r8, 0xbd65767194d9ddc8
140a585e0: mov rcx, r14
140a585e3: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
140a585e8: test rax, rax
140a585eb: je 0x140a58e4f
140a585f1: cmp qword ptr [rsi + 0x218], rdx
140a585f8: jbe 0x140a58e4f
140a585fe: mov rax, qword ptr [rsi + 0x210]
140a58605: mov rcx, qword ptr [rax + rdx*8]
140a58609: test rcx, rcx
140a5860c: je 0x140a58e4f
140a58612: mov rax, qword ptr [rsi + 0x1e0]
140a58619: not rcx
140a5861c: lea rcx, [rcx + rcx*2]
140a58620: shl rcx, 5
140a58624: cmp qword ptr [rax + rcx + 0x20], 0
140a5862a: je 0x140a58e4f
140a58630: add rax, rcx
140a58633: mov rax, qword ptr [rax + 0x10]
140a58637: cmp byte ptr [rax + 0xf9], 0
140a5863e: mov eax, 0xffffffff
140a58643: cmovne ebx, eax
140a58646: mov qword ptr [rsp + 0x30], rdi
140a5864b: mov qword ptr [rsp + 0x28], rsi
140a58650: mov qword ptr [rsp + 0x20], 0x31
140a58659: lea rdx, [rip + 0x207a7e0]
140a58660: lea r9, [rip + 0x207a801]
140a58667: lea rcx, [rbp + 0x170]
140a5866e: mov r8d, 0x28
140a58674: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a58679: cmp byte ptr [rbp + 0x2a6], 8
140a58680: je 0x140a586a7
140a58682: lea rcx, [rbp + 0x280]
140a58689: mov edx, 0xe8
140a5868e: add rdx, qword ptr [rbp + 0x2a8]
140a58695: movabs r8, 0xb02feb017e06f117
140a5869f: mov r9d, ebx
140a586a2: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a586a7: lea r14, [rbp - 0x20]
140a586ab: lea r15, [rbp + 0x170]
140a586b2: mov r8d, 0x190
140a586b8: mov rcx, r14
140a586bb: mov rdx, r15
140a586be: call 0x142923c00 ; ?
140a586c3: mov byte ptr [rbp + 0x300], 0xb
140a586ca: mov dword ptr [rbp + 0x170], 2
140a586d4: mov qword ptr [rsp + 0x20], r15
140a586d9: lea rcx, [rbp + 0x300]
140a586e0: movabs r8, 0xe3659c0fbde7bf03
140a586ea: mov rdx, r14
140a586ed: mov r9b, 0x19
140a586f0: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a586f5: lea r14, [rbp + 0x170]
140a586fc: lea r15, [rbp - 0x20]
140a58700: mov r8d, 0x190
140a58706: mov rcx, r14
140a58709: mov rdx, r15
140a5870c: call 0x142923c00 ; ?
140a58711: mov byte ptr [rbp + 0x300], 0xc
140a58718: mov dword ptr [rbp - 0x20], 2
140a5871f: mov qword ptr [rsp + 0x20], r15
140a58724: lea rcx, [rbp + 0x300]
140a5872b: movabs r8, 0x1ca51a73373208db
140a58735: mov rdx, r14
140a58738: mov r9b, 0x32
140a5873b: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a58740: lea rcx, [rbp - 0x20]
140a58744: lea rdx, [rbp + 0x170]
140a5874b: mov r8d, 0x190
140a58751: call 0x142923c00 ; ?
140a58756: movups xmm0, xmmword ptr [rbp + 0x80]
140a5875d: movaps xmmword ptr [rbp + 0x340], xmm0
140a58764: movups xmm0, xmmword ptr [rbp + 0x70]
140a58768: movaps xmmword ptr [rbp + 0x330], xmm0
140a5876f: movups xmm0, xmmword ptr [rbp + 0x60]
140a58773: movaps xmmword ptr [rbp + 0x320], xmm0
140a5877a: movups xmm0, xmmword ptr [rbp + 0x50]
140a5877e: movaps xmmword ptr [rbp + 0x310], xmm0
140a58785: movups xmm0, xmmword ptr [rbp + 0x40]
140a58789: movaps xmmword ptr [rbp + 0x300], xmm0
140a58790: mov rax, qword ptr [rbp + 0x118]
140a58797: mov rcx, qword ptr [rbp + 0x160]
140a5879e: mov rdx, qword ptr [rbp - 0x10]
140a587a2: mov qword ptr [rbp + 0x180], rdx
140a587a9: movups xmm0, xmmword ptr [rbp - 0x20]
140a587ad: movaps xmmword ptr [rbp + 0x170], xmm0
140a587b4: movups xmm0, xmmword ptr [rbp - 8]
140a587b8: movaps xmmword ptr [rbp + 0x1a0], xmm0
140a587bf: mov rdx, qword ptr [rbp + 8]
140a587c3: mov qword ptr [rbp + 0x1b0], rdx
140a587ca: movups xmm0, xmmword ptr [rbp + 0x10]
140a587ce: movups xmmword ptr [rbp + 0x188], xmm0
140a587d5: mov rdx, qword ptr [rbp + 0x20]
140a587d9: mov qword ptr [rbp + 0x198], rdx
140a587e0: movups xmm0, xmmword ptr [rbp + 0x90]
140a587e7: movups xmm1, xmmword ptr [rbp + 0xa0]
140a587ee: movups xmm2, xmmword ptr [rbp + 0xb0]
140a587f5: movups xmm3, xmmword ptr [rbp + 0xc0]
140a587fc: movups xmmword ptr [rbp + 0x1e8], xmm0
140a58803: movups xmmword ptr [rbp + 0x1f8], xmm1
140a5880a: movups xmmword ptr [rbp + 0x208], xmm2
140a58811: movups xmmword ptr [rbp + 0x218], xmm3
140a58818: movups xmm0, xmmword ptr [rbp + 0xd0]
140a5881f: movups xmmword ptr [rbp + 0x228], xmm0
140a58826: movups xmm0, xmmword ptr [rbp + 0xe0]
140a5882d: movups xmmword ptr [rbp + 0x238], xmm0
140a58834: mov rdx, qword ptr [rbp + 0x38]
140a58838: mov qword ptr [rbp + 0x1c8], rdx
140a5883f: movups xmm0, xmmword ptr [rbp + 0x28]
140a58843: movups xmmword ptr [rbp + 0x1b8], xmm0
140a5884a: mov qword ptr [rbp + 0x250], rcx
140a58851: mov qword ptr [rbp + 0x248], rax
140a58858: movaps xmm6, xmmword ptr [rip + 0x2077c21]
140a5885f: movaps xmmword ptr [rbp + 0x1d0], xmm6
140a58866: lea rcx, [rbp + 0x300]
140a5886d: lea rdx, [rbp + 0x170]
140a58874: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a58879: cmp byte ptr [rbp + 0x116], 8
140a58880: je 0x140a5888e
140a58882: lea rcx, [rbp + 0x100]
140a58889: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a5888e: cmp ebx, -1
140a58891: je 0x140a58e2c
140a58897: lea rcx, [rbp + 0x300]
140a5889e: movabs rdx, 0x10000000100
140a588a8: mov r8d, 0x4c
140a588ae: call 0x140bd52a0 ; rhapsody::raw::texture::TextureDesc::new_2d
140a588b3: movups xmm0, xmmword ptr [rbp + 0x300]
140a588ba: movups xmm1, xmmword ptr [rbp + 0x310]
140a588c1: movaps xmmword ptr [rbp - 0x10], xmm1
140a588c5: movaps xmmword ptr [rbp - 0x20], xmm0
140a588c9: lea rcx, [rbp + 0x180]
140a588d0: lea rdx, [rbp - 0x20]
140a588d4: call 0x140bd6ce0 ; rhapsody::rg::resource::impl$4::from
140a588d9: lea rax, [rip + 0x207a5b9]
140a588e0: mov qword ptr [rbp + 0x170], rax
140a588e7: mov qword ptr [rbp + 0x178], 9
140a588f2: lea rdx, [rbp + 0x170]
140a588f9: mov rcx, rdi
140a588fc: call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
140a58901: movups xmm0, xmmword ptr [rbp + 0x300]
140a58908: movups xmm1, xmmword ptr [rbp + 0x310]
140a5890f: movups xmmword ptr [rbp - 0x40], xmm0
140a58913: movups xmmword ptr [rbp - 0x30], xmm1
140a58917: mov dword ptr [rbp - 0x44], eax
140a5891a: mov qword ptr [rsp + 0x30], rdi
140a5891f: mov qword ptr [rsp + 0x28], rsi
140a58924: mov qword ptr [rsp + 0x20], 0x18
140a5892d: lea rdx, [rip + 0x207a56e]
140a58934: lea r9, [rip + 0x207a582]
140a5893b: lea rcx, [rbp + 0x170]
140a58942: mov r8d, 0x1b
140a58948: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a5894d: cmp byte ptr [rbp + 0x2a6], 8
140a58954: je 0x140a5897b
140a58956: lea rcx, [rbp + 0x280]
140a5895d: mov edx, 0xe8
140a58962: add rdx, qword ptr [rbp + 0x2a8]
140a58969: movabs r8, 0xd91ff8372094bc22
140a58973: mov r9b, 1
140a58976: call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
140a5897b: lea rcx, [rbp - 0x20]
140a5897f: lea rdx, [rbp + 0x170]
140a58986: mov r8d, 0x190
140a5898c: call 0x142923c00 ; ?
140a58991: cmp byte ptr [rbp + 0x116], 8
140a58998: je 0x140a589c4
140a5899a: lea rcx, [rbp + 0xf0]
140a589a1: mov edx, 0xe8
140a589a6: add rdx, qword ptr [rbp + 0x118]
140a589ad: movabs r8, 0xdc022c4f3092a794
140a589b7: movss xmm3, dword ptr [rip + 0x1eccf45]
140a589bf: call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
140a589c4: lea rbx, [rbp + 0x170]
140a589cb: lea r14, [rbp - 0x20]
140a589cf: mov r8d, 0x190
140a589d5: mov rcx, rbx
140a589d8: mov rdx, r14
140a589db: call 0x142923c00 ; ?
140a589e0: mov byte ptr [rbp + 0x300], 0xc
140a589e7: mov dword ptr [rbp - 0x20], 2
140a589ee: mov qword ptr [rsp + 0x20], r14
140a589f3: lea rcx, [rbp + 0x300]
140a589fa: movabs r8, 0xd82d7a0adb9164c2
140a58a04: mov rdx, rbx
140a58a07: mov r9b, 0x19
140a58a0a: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a58a0f: lea rbx, [rbp - 0x20]
140a58a13: lea r14, [rbp + 0x170]
140a58a1a: mov r8d, 0x190
140a58a20: mov rcx, rbx
140a58a23: mov rdx, r14
140a58a26: call 0x142923c00 ; ?
140a58a2b: lea r15, [rbp - 0x44]
140a58a2f: mov qword ptr [rbp + 0x300], r15
140a58a36: mov dword ptr [rbp + 0x170], 2
140a58a40: mov qword ptr [rsp + 0x20], r14
140a58a45: lea rcx, [rbp + 0x300]
140a58a4c: movabs r8, 0x1ca51a73373208db
140a58a56: mov rdx, rbx
140a58a59: mov r9b, 0x32
140a58a5c: call 0x1408edd00 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
140a58a61: lea rcx, [rbp + 0x170]
140a58a68: lea rdx, [rbp - 0x20]
140a58a6c: mov r8d, 0x190
140a58a72: call 0x142923c00 ; ?
140a58a77: movups xmm0, xmmword ptr [rbp + 0x210]
140a58a7e: movaps xmmword ptr [rbp + 0x340], xmm0
140a58a85: movups xmm0, xmmword ptr [rbp + 0x200]
140a58a8c: movaps xmmword ptr [rbp + 0x330], xmm0
140a58a93: movups xmm0, xmmword ptr [rbp + 0x1f0]
140a58a9a: movaps xmmword ptr [rbp + 0x320], xmm0
140a58aa1: movups xmm0, xmmword ptr [rbp + 0x1e0]
140a58aa8: movaps xmmword ptr [rbp + 0x310], xmm0
140a58aaf: movups xmm0, xmmword ptr [rbp + 0x1d0]
140a58ab6: movaps xmmword ptr [rbp + 0x300], xmm0
140a58abd: mov rax, qword ptr [rbp + 0x2a8]
140a58ac4: mov rcx, qword ptr [rbp + 0x2f0]
140a58acb: mov rdx, qword ptr [rbp + 0x180]
140a58ad2: mov qword ptr [rbp - 0x10], rdx
140a58ad6: movups xmm0, xmmword ptr [rbp + 0x170]
140a58add: movaps xmmword ptr [rbp - 0x20], xmm0
140a58ae1: movups xmm0, xmmword ptr [rbp + 0x188]
140a58ae8: movaps xmmword ptr [rbp + 0x10], xmm0
140a58aec: mov rdx, qword ptr [rbp + 0x198]
140a58af3: mov qword ptr [rbp + 0x20], rdx
140a58af7: movups xmm0, xmmword ptr [rbp + 0x1a0]
140a58afe: movups xmmword ptr [rbp - 8], xmm0
140a58b02: mov rdx, qword ptr [rbp + 0x1b0]
140a58b09: mov qword ptr [rbp + 8], rdx
140a58b0d: movups xmm0, xmmword ptr [rbp + 0x220]
140a58b14: movups xmm1, xmmword ptr [rbp + 0x230]
140a58b1b: movups xmm2, xmmword ptr [rbp + 0x240]
140a58b22: movups xmm3, xmmword ptr [rbp + 0x250]
140a58b29: movups xmmword ptr [rbp + 0x58], xmm0
140a58b2d: movups xmmword ptr [rbp + 0x68], xmm1
140a58b31: movups xmmword ptr [rbp + 0x78], xmm2
140a58b35: movups xmmword ptr [rbp + 0x88], xmm3
140a58b3c: movups xmm0, xmmword ptr [rbp + 0x260]
140a58b43: movups xmmword ptr [rbp + 0x98], xmm0
140a58b4a: movups xmm0, xmmword ptr [rbp + 0x270]
140a58b51: movups xmmword ptr [rbp + 0xa8], xmm0
140a58b58: mov rdx, qword ptr [rbp + 0x1c8]
140a58b5f: mov qword ptr [rbp + 0x38], rdx
140a58b63: movups xmm0, xmmword ptr [rbp + 0x1b8]
140a58b6a: movups xmmword ptr [rbp + 0x28], xmm0
140a58b6e: mov qword ptr [rbp + 0xc0], rcx
140a58b75: mov qword ptr [rbp + 0xb8], rax
140a58b7c: movaps xmmword ptr [rbp + 0x40], xmm6
140a58b80: lea rcx, [rbp + 0x300]
140a58b87: lea rdx, [rbp - 0x20]
140a58b8b: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a58b90: cmp byte ptr [rbp + 0x2a6], 8
140a58b97: je 0x140a58ba5
140a58b99: lea rcx, [rbp + 0x290]
140a58ba0: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a58ba5: mov qword ptr [rsp + 0x30], rdi
140a58baa: mov qword ptr [rsp + 0x28], rsi
140a58baf: mov qword ptr [rsp + 0x20], 0x18
140a58bb8: lea rdx, [rip + 0x207a2e3]
140a58bbf: lea r9, [rip + 0x207a2f7]
140a58bc6: lea rcx, [rbp + 0x170]
140a58bcd: mov r8d, 0x1b
140a58bd3: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a58bd8: cmp byte ptr [rbp + 0x2a6], 8
140a58bdf: je 0x140a58c06
140a58be1: lea rcx, [rbp + 0x280]
140a58be8: mov edx, 0xe8
140a58bed: add rdx, qword ptr [rbp + 0x2a8]
140a58bf4: movabs r8, 0xd91ff8372094bc22
140a58bfe: xor r9d, r9d
140a58c01: call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
140a58c06: lea rcx, [rbp - 0x20]
140a58c0a: lea rdx, [rbp + 0x170]
140a58c11: mov r8d, 0x190
140a58c17: call 0x142923c00 ; ?
140a58c1c: cmp byte ptr [rbp + 0x116], 8
140a58c23: je 0x140a58c4f
140a58c25: lea rcx, [rbp + 0xf0]
140a58c2c: mov edx, 0xe8
140a58c31: add rdx, qword ptr [rbp + 0x118]
140a58c38: movabs r8, 0xdc022c4f3092a794
140a58c42: movss xmm3, dword ptr [rip + 0x1ecccba]
140a58c4a: call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
140a58c4f: lea rsi, [rbp + 0x170]
140a58c56: lea rdi, [rbp - 0x20]
140a58c5a: mov r8d, 0x190
140a58c60: mov rcx, rsi
140a58c63: mov rdx, rdi
140a58c66: call 0x142923c00 ; ?
140a58c6b: mov qword ptr [rbp + 0x300], r15
140a58c72: mov dword ptr [rbp - 0x20], 2
140a58c79: mov qword ptr [rsp + 0x20], rdi
140a58c7e: lea rcx, [rbp + 0x300]
140a58c85: movabs r8, 0xd82d7a0adb9164c2
140a58c8f: mov rdx, rsi
140a58c92: mov r9b, 0x19
140a58c95: call 0x1408ee240 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
140a58c9a: lea rsi, [rbp - 0x20]
140a58c9e: lea rdi, [rbp + 0x170]
140a58ca5: mov r8d, 0x190
140a58cab: mov rcx, rsi
140a58cae: mov rdx, rdi
140a58cb1: call 0x142923c00 ; ?
140a58cb6: mov byte ptr [rbp + 0x300], 0xc
140a58cbd: mov dword ptr [rbp + 0x170], 2
140a58cc7: mov qword ptr [rsp + 0x20], rdi
140a58ccc: lea rcx, [rbp + 0x300]
140a58cd3: movabs r8, 0x1ca51a73373208db
140a58cdd: mov rdx, rsi
140a58ce0: mov r9b, 0x32
140a58ce3: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a58ce8: lea rcx, [rbp + 0x170]
140a58cef: lea rdx, [rbp - 0x20]
140a58cf3: mov r8d, 0x190
140a58cf9: call 0x142923c00 ; ?
140a58cfe: movups xmm0, xmmword ptr [rbp + 0x210]
140a58d05: movaps xmmword ptr [rbp + 0x340], xmm0
140a58d0c: movups xmm0, xmmword ptr [rbp + 0x200]
140a58d13: movaps xmmword ptr [rbp + 0x330], xmm0
140a58d1a: movups xmm0, xmmword ptr [rbp + 0x1f0]
140a58d21: movaps xmmword ptr [rbp + 0x320], xmm0
140a58d28: movups xmm0, xmmword ptr [rbp + 0x1e0]
140a58d2f: movaps xmmword ptr [rbp + 0x310], xmm0
140a58d36: movups xmm0, xmmword ptr [rbp + 0x1d0]
140a58d3d: movaps xmmword ptr [rbp + 0x300], xmm0
140a58d44: mov rax, qword ptr [rbp + 0x2a8]
140a58d4b: mov rcx, qword ptr [rbp + 0x2f0]
140a58d52: mov rdx, qword ptr [rbp + 0x180]
140a58d59: mov qword ptr [rbp - 0x10], rdx
140a58d5d: movups xmm0, xmmword ptr [rbp + 0x170]
140a58d64: movaps xmmword ptr [rbp - 0x20], xmm0
140a58d68: movups xmm0, xmmword ptr [rbp + 0x188]
140a58d6f: movaps xmmword ptr [rbp + 0x10], xmm0
140a58d73: mov rdx, qword ptr [rbp + 0x198]
140a58d7a: mov qword ptr [rbp + 0x20], rdx
140a58d7e: movups xmm0, xmmword ptr [rbp + 0x1a0]
140a58d85: movups xmmword ptr [rbp - 8], xmm0
140a58d89: mov rdx, qword ptr [rbp + 0x1b0]
140a58d90: mov qword ptr [rbp + 8], rdx
140a58d94: movups xmm0, xmmword ptr [rbp + 0x220]
140a58d9b: movups xmm1, xmmword ptr [rbp + 0x230]
140a58da2: movups xmm2, xmmword ptr [rbp + 0x240]
140a58da9: movups xmm3, xmmword ptr [rbp + 0x250]
140a58db0: movups xmmword ptr [rbp + 0x58], xmm0
140a58db4: movups xmmword ptr [rbp + 0x68], xmm1
140a58db8: movups xmmword ptr [rbp + 0x78], xmm2
140a58dbc: movups xmmword ptr [rbp + 0x88], xmm3
140a58dc3: movups xmm0, xmmword ptr [rbp + 0x260]
140a58dca: movups xmmword ptr [rbp + 0x98], xmm0
140a58dd1: movups xmm0, xmmword ptr [rbp + 0x270]
140a58dd8: movups xmmword ptr [rbp + 0xa8], xmm0
140a58ddf: mov rdx, qword ptr [rbp + 0x1c8]
140a58de6: mov qword ptr [rbp + 0x38], rdx
140a58dea: movups xmm0, xmmword ptr [rbp + 0x1b8]
140a58df1: movups xmmword ptr [rbp + 0x28], xmm0
140a58df5: mov qword ptr [rbp + 0xc0], rcx
140a58dfc: mov qword ptr [rbp + 0xb8], rax
140a58e03: movaps xmmword ptr [rbp + 0x40], xmm6
140a58e07: lea rcx, [rbp + 0x300]
140a58e0e: lea rdx, [rbp - 0x20]
140a58e12: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a58e17: cmp byte ptr [rbp + 0x2a6], 8
140a58e1e: je 0x140a58e2c
140a58e20: lea rcx, [rbp + 0x290]
140a58e27: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a58e2c: movaps xmm6, xmmword ptr [rbp + 0x360]
140a58e33: add rsp, 0x3f8
140a58e3a: pop rbx
140a58e3b: pop rdi
140a58e3c: pop rsi
140a58e3d: pop r14
140a58e3f: pop r15
140a58e41: pop rbp
140a58e42: ret 
140a58e43: lea rcx, [rip + 0x2079fae]
140a58e4a: call 0x1428d9310 ; core::option::unwrap_failed
140a58e4f: lea rcx, [rip + 0x2079fd2]
140a58e56: call 0x1428d9310 ; core::option::unwrap_failed
140a58e5b: lea rcx, [rip + 0x2079fae]
140a58e62: call 0x1428d9310 ; core::option::unwrap_failed
140a58e67: int3 
140a58e68: nop dword ptr [rax + rax]
140a58e70: mov qword ptr [rsp + 0x10], rdx
140a58e75: push rbp
140a58e76: push r15
140a58e78: push r14
140a58e7a: push rsi
140a58e7b: push rdi
140a58e7c: push rbx
140a58e7d: sub rsp, 0x48
140a58e81: lea rbp, [rdx + 0x80]
140a58e88: movaps xmmword ptr [rsp + 0x30], xmm6
140a58e8d: cmp byte ptr [rbp + 0x116], 8
140a58e94: je 0x140a58ea2
140a58e96: lea rcx, [rbp + 0x100]
140a58e9d: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a58ea2: movaps xmm6, xmmword ptr [rsp + 0x30]
140a58ea7: add rsp, 0x48
140a58eab: pop rbx
140a58eac: pop rdi
140a58ead: pop rsi
140a58eae: pop r14
140a58eb0: pop r15
140a58eb2: pop rbp
140a58eb3: ret 
140a58eb4: nop word ptr cs:[rax + rax]
140a58ec0: mov qword ptr [rsp + 0x10], rdx
140a58ec5: push rbp
140a58ec6: push r15
140a58ec8: push r14
140a58eca: push rsi
140a58ecb: push rdi
140a58ecc: push rbx
140a58ecd: sub rsp, 0x48
140a58ed1: lea rbp, [rdx + 0x80]
140a58ed8: movaps xmmword ptr [rsp + 0x30], xmm6
140a58edd: lea rcx, [rbp + 0x170]
140a58ee4: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a58ee9: movaps xmm6, xmmword ptr [rsp + 0x30]
140a58eee: add rsp, 0x48
140a58ef2: pop rbx
140a58ef3: pop rdi
140a58ef4: pop rsi
140a58ef5: pop r14
140a58ef7: pop r15
140a58ef9: pop rbp
140a58efa: ret 
140a58efb: nop dword ptr [rax + rax]
140a58f00: mov qword ptr [rsp + 0x10], rdx
140a58f05: push rbp
140a58f06: push r15
140a58f08: push r14
140a58f0a: push rsi
140a58f0b: push rdi
140a58f0c: push rbx
140a58f0d: sub rsp, 0x48
140a58f11: lea rbp, [rdx + 0x80]
140a58f18: movaps xmmword ptr [rsp + 0x30], xmm6
140a58f1d: lea rcx, [rbp - 0x20]
140a58f21: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a58f26: movaps xmm6, xmmword ptr [rsp + 0x30]
140a58f2b: add rsp, 0x48
140a58f2f: pop rbx
140a58f30: pop rdi
140a58f31: pop rsi
140a58f32: pop r14
140a58f34: pop r15
140a58f36: pop rbp
140a58f37: ret 
140a58f38: nop dword ptr [rax + rax]
140a58f40: mov qword ptr [rsp + 0x10], rdx
140a58f45: push rbp
140a58f46: push r15
140a58f48: push r14
140a58f4a: push rsi
140a58f4b: push rdi
140a58f4c: push rbx
140a58f4d: sub rsp, 0x48
140a58f51: lea rbp, [rdx + 0x80]
140a58f58: movaps xmmword ptr [rsp + 0x30], xmm6
140a58f5d: lea rcx, [rbp + 0x170]
140a58f64: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a58f69: movaps xmm6, xmmword ptr [rsp + 0x30]
140a58f6e: add rsp, 0x48
140a58f72: pop rbx
140a58f73: pop rdi
140a58f74: pop rsi
140a58f75: pop r14
140a58f77: pop r15
140a58f79: pop rbp
140a58f7a: ret 
140a58f7b: nop dword ptr [rax + rax]
140a58f80: mov qword ptr [rsp + 0x10], rdx
140a58f85: push rbp
140a58f86: push r15
140a58f88: push r14
140a58f8a: push rsi
140a58f8b: push rdi
140a58f8c: push rbx
140a58f8d: sub rsp, 0x48
140a58f91: lea rbp, [rdx + 0x80]
140a58f98: movaps xmmword ptr [rsp + 0x30], xmm6
140a58f9d: lea rcx, [rbp - 0x20]
140a58fa1: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a58fa6: movaps xmm6, xmmword ptr [rsp + 0x30]
140a58fab: add rsp, 0x48
140a58faf: pop rbx
140a58fb0: pop rdi
140a58fb1: pop rsi
140a58fb2: pop r14
140a58fb4: pop r15
140a58fb6: pop rbp
140a58fb7: ret 
140a58fb8: nop dword ptr [rax + rax]
140a58fc0: mov qword ptr [rsp + 0x10], rdx
140a58fc5: push rbp
140a58fc6: push r15
140a58fc8: push r14
140a58fca: push rsi
140a58fcb: push rdi
140a58fcc: push rbx
140a58fcd: sub rsp, 0x48
140a58fd1: lea rbp, [rdx + 0x80]
140a58fd8: movaps xmmword ptr [rsp + 0x30], xmm6
140a58fdd: cmp byte ptr [rbp + 0x2a6], 8
140a58fe4: je 0x140a58ff2
140a58fe6: lea rcx, [rbp + 0x290]
140a58fed: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a58ff2: movaps xmm6, xmmword ptr [rsp + 0x30]
140a58ff7: add rsp, 0x48
140a58ffb: pop rbx
140a58ffc: pop rdi
140a58ffd: pop rsi
140a58ffe: pop r14
140a59000: pop r15
140a59002: pop rbp
140a59003: ret 
140a59004: nop word ptr cs:[rax + rax]
140a59010: mov qword ptr [rsp + 0x10], rdx
140a59015: push rbp
140a59016: push r15
140a59018: push r14
140a5901a: push rsi
140a5901b: push rdi
140a5901c: push rbx
140a5901d: sub rsp, 0x48
140a59021: lea rbp, [rdx + 0x80]
140a59028: movaps xmmword ptr [rsp + 0x30], xmm6
140a5902d: lea rcx, [rbp + 0x170]
140a59034: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a59039: movaps xmm6, xmmword ptr [rsp + 0x30]
140a5903e: add rsp, 0x48
140a59042: pop rbx
140a59043: pop rdi
140a59044: pop rsi
140a59045: pop r14
140a59047: pop r15
140a59049: pop rbp
140a5904a: ret 
140a5904b: nop dword ptr [rax + rax]
140a59050: mov qword ptr [rsp + 0x10], rdx
140a59055: push rbp
140a59056: push r15
140a59058: push r14
140a5905a: push rsi
140a5905b: push rdi
140a5905c: push rbx
140a5905d: sub rsp, 0x48
140a59061: lea rbp, [rdx + 0x80]
140a59068: movaps xmmword ptr [rsp + 0x30], xmm6
140a5906d: lea rcx, [rbp - 0x20]
140a59071: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a59076: movaps xmm6, xmmword ptr [rsp + 0x30]
140a5907b: add rsp, 0x48
140a5907f: pop rbx
140a59080: pop rdi
140a59081: pop rsi
140a59082: pop r14
140a59084: pop r15
140a59086: pop rbp
140a59087: ret 
140a59088: nop dword ptr [rax + rax]
140a59090: mov qword ptr [rsp + 0x10], rdx
140a59095: push rbp
140a59096: push r15
140a59098: push r14
140a5909a: push rsi
140a5909b: push rdi
140a5909c: push rbx
140a5909d: sub rsp, 0x48
140a590a1: lea rbp, [rdx + 0x80]
140a590a8: movaps xmmword ptr [rsp + 0x30], xmm6
140a590ad: lea rcx, [rbp + 0x170]
140a590b4: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a590b9: movaps xmm6, xmmword ptr [rsp + 0x30]
140a590be: add rsp, 0x48
140a590c2: pop rbx
140a590c3: pop rdi
140a590c4: pop rsi
140a590c5: pop r14
140a590c7: pop r15
140a590c9: pop rbp
140a590ca: ret 
140a590cb: nop dword ptr [rax + rax]
140a590d0: mov qword ptr [rsp + 0x10], rdx
140a590d5: push rbp
140a590d6: push r15
140a590d8: push r14
140a590da: push rsi
140a590db: push rdi
140a590dc: push rbx
140a590dd: sub rsp, 0x48
140a590e1: lea rbp, [rdx + 0x80]
140a590e8: movaps xmmword ptr [rsp + 0x30], xmm6
140a590ed: lea rcx, [rbp - 0x20]
140a590f1: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a590f6: movaps xmm6, xmmword ptr [rsp + 0x30]
140a590fb: add rsp, 0x48
140a590ff: pop rbx
140a59100: pop rdi
140a59101: pop rsi
140a59102: pop r14
140a59104: pop r15
140a59106: pop rbp
140a59107: ret 
140a59108: nop dword ptr [rax + rax]
140a59110: mov qword ptr [rsp + 0x10], rdx
140a59115: push rbp
140a59116: push r15
140a59118: push r14
140a5911a: push rsi
140a5911b: push rdi
140a5911c: push rbx
140a5911d: sub rsp, 0x48
140a59121: lea rbp, [rdx + 0x80]
140a59128: movaps xmmword ptr [rsp + 0x30], xmm6
140a5912d: cmp byte ptr [rbp + 0x2a6], 8
140a59134: je 0x140a59142
140a59136: lea rcx, [rbp + 0x290]
140a5913d: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a59142: movaps xmm6, xmmword ptr [rsp + 0x30]
140a59147: add rsp, 0x48
140a5914b: pop rbx
140a5914c: pop rdi
140a5914d: pop rsi
140a5914e: pop r14
140a59150: pop r15
140a59152: pop rbp
140a59153: ret 
140a59154: nop word ptr cs:[rax + rax]
140a59160: mov qword ptr [rsp + 0x10], rdx
140a59165: push rbp
140a59166: push r15
140a59168: push r14
140a5916a: push rsi
140a5916b: push rdi
140a5916c: push rbx
140a5916d: sub rsp, 0x48
140a59171: lea rbp, [rdx + 0x80]
140a59178: movaps xmmword ptr [rsp + 0x30], xmm6
140a5917d: lea rcx, [rbp + 0x170]
140a59184: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a59189: movaps xmm6, xmmword ptr [rsp + 0x30]
140a5918e: add rsp, 0x48
140a59192: pop rbx
140a59193: pop rdi
140a59194: pop rsi
140a59195: pop r14
140a59197: pop r15
140a59199: pop rbp
140a5919a: ret 
140a5919b: nop dword ptr [rax + rax]
140a591a0: mov qword ptr [rsp + 0x10], rdx
140a591a5: push rbp
140a591a6: push r15
140a591a8: push r14
140a591aa: push rsi
140a591ab: push rdi
140a591ac: push rbx
140a591ad: sub rsp, 0x48
140a591b1: lea rbp, [rdx + 0x80]
140a591b8: movaps xmmword ptr [rsp + 0x30], xmm6
140a591bd: lea rcx, [rbp - 0x20]
140a591c1: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a591c6: movaps xmm6, xmmword ptr [rsp + 0x30]
140a591cb: add rsp, 0x48
140a591cf: pop rbx
140a591d0: pop rdi
140a591d1: pop rsi
140a591d2: pop r14
140a591d4: pop r15
140a591d6: pop rbp
140a591d7: ret 
140a591d8: nop dword ptr [rax + rax]
140a591e0: mov qword ptr [rsp + 0x10], rdx
140a591e5: push rbp
140a591e6: push r15
140a591e8: push r14
140a591ea: push rsi
140a591eb: push rdi
140a591ec: push rbx
140a591ed: sub rsp, 0x48
140a591f1: lea rbp, [rdx + 0x80]
140a591f8: movaps xmmword ptr [rsp + 0x30], xmm6
140a591fd: lea rcx, [rbp + 0x170]
140a59204: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a59209: movaps xmm6, xmmword ptr [rsp + 0x30]
140a5920e: add rsp, 0x48
140a59212: pop rbx
140a59213: pop rdi
140a59214: pop rsi
140a59215: pop r14
140a59217: pop r15
140a59219: pop rbp
140a5921a: ret 

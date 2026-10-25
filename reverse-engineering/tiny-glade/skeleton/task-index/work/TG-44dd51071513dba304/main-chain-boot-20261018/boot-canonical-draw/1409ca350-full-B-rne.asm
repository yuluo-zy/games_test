1409ca350: 55 push rbp
1409ca351: 4157 push r15
1409ca353: 4156 push r14
1409ca355: 56 push rsi
1409ca356: 57 push rdi
1409ca357: 53 push rbx
1409ca358: 4881ec68020000 sub rsp, 0x268
1409ca35f: 488dac2480000000 lea rbp, [rsp + 0x80]
1409ca367: 0f29bdd0010000 movaps xmmword ptr [rbp + 0x1d0], xmm7
1409ca36e: 0f29b5c0010000 movaps xmmword ptr [rbp + 0x1c0], xmm6
1409ca375: 48c785b8010000feffffff mov qword ptr [rbp + 0x1b8], 0xfffffffffffffffe
1409ca380: 4c89c6 mov rsi, r8
1409ca383: 80b9f000000000 cmp byte ptr [rcx + 0xf0], 0
1409ca38a: 7411 je 0x1409ca39d
1409ca38c: 0fb681a8000000 movzx eax, byte ptr [rcx + 0xa8]
1409ca393: 24f3 and al, 0xf3
1409ca395: 0c04 or al, 4
1409ca397: 8881a8000000 mov byte ptr [rcx + 0xa8], al
1409ca39d: 48898da0010000 mov qword ptr [rbp + 0x1a0], rcx
1409ca3a4: 488d7e38 lea rdi, [rsi + 0x38]
1409ca3a8: c685b701000001 mov byte ptr [rbp + 0x1b7], 1
1409ca3af: 48baffa3086aee51a4bb movabs rdx, 0xbba451ee6a08a3ff
1409ca3b9: 49b89df2bcfe158d008d movabs r8, 0x8d008d15febcf29d
1409ca3c3: 4889f9 mov rcx, rdi
1409ca3c6: e8253371ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
1409ca3cb: 4885c0 test rax, rax
1409ca3ce: 0f84a0040000 je 0x1409ca874
1409ca3d4: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1409ca3db: 0f8693040000 jbe 0x1409ca874
1409ca3e1: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1409ca3e8: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409ca3ec: 4885c9 test rcx, rcx
1409ca3ef: 0f847f040000 je 0x1409ca874
1409ca3f5: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
1409ca3fc: 48f7d1 not rcx
1409ca3ff: 488d0c49 lea rcx, [rcx + rcx*2]
1409ca403: 48c1e105 shl rcx, 5
1409ca407: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409ca40d: 0f8461040000 je 0x1409ca874
1409ca413: 4801c8 add rax, rcx
1409ca416: 4c8b7810 mov r15, qword ptr [rax + 0x10]
1409ca41a: 410fb687d0040000 movzx eax, byte ptr [r15 + 0x4d0]
1409ca422: 488d0dc7680f02 lea rcx, [rip + 0x20f68c7]
1409ca429: 488b04c1 mov rax, qword ptr [rcx + rax*8]
1409ca42d: f3410f10b40704010000 movss xmm6, dword ptr [r15 + rax + 0x104]
1409ca437: c685b701000001 mov byte ptr [rbp + 0x1b7], 1
1409ca43e: 48ba3b4166302619f5d2 movabs rdx, 0xd2f519263066413b
1409ca448: 49b8e45925965fb19251 movabs r8, 0x5192b15f962559e4
1409ca452: 4889f9 mov rcx, rdi
1409ca455: e8963271ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
1409ca45a: 4885c0 test rax, rax
1409ca45d: 0f8426040000 je 0x1409ca889
1409ca463: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1409ca46a: 0f8619040000 jbe 0x1409ca889
1409ca470: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1409ca477: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409ca47b: 4885c9 test rcx, rcx
1409ca47e: 0f8405040000 je 0x1409ca889
1409ca484: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
1409ca48b: 48f7d1 not rcx
1409ca48e: 488d0c49 lea rcx, [rcx + rcx*2]
1409ca492: 48c1e105 shl rcx, 5
1409ca496: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409ca49c: 0f84e7030000 je 0x1409ca889
1409ca4a2: 4801c8 add rax, rcx
1409ca4a5: 488b4010 mov rax, qword ptr [rax + 0x10]
1409ca4a9: f30f107804 movss xmm7, dword ptr [rax + 4]
1409ca4ae: 488d9da8000000 lea rbx, [rbp + 0xa8]
1409ca4b5: 41b8f8000000 mov r8d, 0xf8
1409ca4bb: 4889d9 mov rcx, rbx
1409ca4be: 488b95a0010000 mov rdx, qword ptr [rbp + 0x1a0]
1409ca4c5: e83697f501 call 0x142923c00 ; 
1409ca4ca: 48b86518a091ab399f57 movabs rax, 0x579f39ab91a01865
1409ca4d4: 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
1409ca4db: c745b002000000 mov dword ptr [rbp - 0x50], 2
1409ca4e2: 488d45b0 lea rax, [rbp - 0x50]
1409ca4e6: 4889442420 mov qword ptr [rsp + 0x20], rax
1409ca4eb: 488d8da8010000 lea rcx, [rbp + 0x1a8]
1409ca4f2: 49b88c97666903f55b81 movabs r8, 0x815bf5036966978c
1409ca4fc: 4889da mov rdx, rbx
1409ca4ff: 41b11d mov r9b, 0x1d
1409ca502: e809e1f3ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409ca507: 488d5db0 lea rbx, [rbp - 0x50]
1409ca50b: 4c8db5a8000000 lea r14, [rbp + 0xa8]
1409ca512: 41b8f8000000 mov r8d, 0xf8
1409ca518: 4889d9 mov rcx, rbx
1409ca51b: 4c89f2 mov rdx, r14
1409ca51e: e8dd96f501 call 0x142923c00 ; 
1409ca523: 48b862964d02305b1d52 movabs rax, 0x521d5b30024d9662
1409ca52d: 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
1409ca534: c785a800000002000000 mov dword ptr [rbp + 0xa8], 2
1409ca53e: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409ca543: 488d8da8010000 lea rcx, [rbp + 0x1a8]
1409ca54a: 49b815dea3cf33a87ebb movabs r8, 0xbb7ea833cfa3de15
1409ca554: 4889da mov rdx, rbx
1409ca557: 41b11d mov r9b, 0x1d
1409ca55a: e8b1e0f3ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409ca55f: 488d9da8000000 lea rbx, [rbp + 0xa8]
1409ca566: 4c8d75b0 lea r14, [rbp - 0x50]
1409ca56a: 41b8f8000000 mov r8d, 0xf8
1409ca570: 4889d9 mov rcx, rbx
1409ca573: 4c89f2 mov rdx, r14
1409ca576: e88596f501 call 0x142923c00 ; 
1409ca57b: 48b8d4983f00691d2093 movabs rax, 0x93201d69003f98d4
1409ca585: 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
1409ca58c: c745b002000000 mov dword ptr [rbp - 0x50], 2
1409ca593: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409ca598: 488d8da8010000 lea rcx, [rbp + 0x1a8]
1409ca59f: 49b81e7b818ffc601056 movabs r8, 0x561060fc8f817b1e
1409ca5a9: 4889da mov rdx, rbx
1409ca5ac: 41b11d mov r9b, 0x1d
1409ca5af: e85ce0f3ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409ca5b4: 488d5db0 lea rbx, [rbp - 0x50]
1409ca5b8: 4c8db5a8000000 lea r14, [rbp + 0xa8]
1409ca5bf: 41b8f8000000 mov r8d, 0xf8
1409ca5c5: 4889d9 mov rcx, rbx
1409ca5c8: 4c89f2 mov rdx, r14
1409ca5cb: e83096f501 call 0x142923c00 ; 
1409ca5d0: 48b84d7cb71113b012aa movabs rax, 0xaa12b01311b77c4d
1409ca5da: 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
1409ca5e1: c785a800000002000000 mov dword ptr [rbp + 0xa8], 2
1409ca5eb: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409ca5f0: 488d8da8010000 lea rcx, [rbp + 0x1a8]
1409ca5f7: 49b802e6e599c585e462 movabs r8, 0x62e485c599e5e602
1409ca601: 4889da mov rdx, rbx
1409ca604: 41b11d mov r9b, 0x1d
1409ca607: e804e0f3ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409ca60c: 488d9da8000000 lea rbx, [rbp + 0xa8]
1409ca613: 4c8d75b0 lea r14, [rbp - 0x50]
1409ca617: 41b8f8000000 mov r8d, 0xf8
1409ca61d: 4889d9 mov rcx, rbx
1409ca620: 4c89f2 mov rdx, r14
1409ca623: e8d895f501 call 0x142923c00 ; 
1409ca628: 48b8b7c7f1ac39bb4d00 movabs rax, 0x4dbb39acf1c7b7
1409ca632: 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
1409ca639: c745b002000000 mov dword ptr [rbp - 0x50], 2
1409ca640: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409ca645: 488d8da8010000 lea rcx, [rbp + 0x1a8]
1409ca64c: 49b88cb1422a0801a530 movabs r8, 0x30a501082a42b18c
1409ca656: 4889da mov rdx, rbx
1409ca659: 41b11d mov r9b, 0x1d
1409ca65c: e8afdff3ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409ca661: 488d4db0 lea rcx, [rbp - 0x50]
1409ca665: 488d95a8000000 lea rdx, [rbp + 0xa8]
1409ca66c: 41b8f8000000 mov r8d, 0xf8
1409ca672: e88995f501 call 0x142923c00 ; 
1409ca677: 80bd8e00000008 cmp byte ptr [rbp + 0x8e], 8
1409ca67e: 7422 je 0x1409ca6a2
1409ca680: 488d4d68 lea rcx, [rbp + 0x68]
1409ca684: bae8000000 mov edx, 0xe8
1409ca689: 48039598000000 add rdx, qword ptr [rbp + 0x98]
1409ca690: 49b8694d11bdea3f2acb movabs r8, 0xcb2a3feabd114d69
1409ca69a: 0f28df movaps xmm3, xmm7
1409ca69d: e88e13faff call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
1409ca6a2: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409ca6a9: 488d55b0 lea rdx, [rbp - 0x50]
1409ca6ad: 41b8f8000000 mov r8d, 0xf8
1409ca6b3: e84895f501 call 0x142923c00 ; 
1409ca6b8: 80bd8601000008 cmp byte ptr [rbp + 0x186], 8
1409ca6bf: 7445 je 0x1409ca706
1409ca6c1: f30f5935e70d0e02 mulss xmm6, dword ptr [rip + 0x20e0de7]
1409ca6c9: f30f59359fb1f501 mulss xmm6, dword ptr [rip + 0x1f5b19f]
1409ca6d1: f30f5f357b3c0b02 maxss xmm6, dword ptr [rip + 0x20b3c7b]
1409ca6d9: f30f5935d7c50402 mulss xmm6, dword ptr [rip + 0x204c5d7]
1409ca6e1: 488d8d60010000 lea rcx, [rbp + 0x160]
1409ca6e8: bae8000000 mov edx, 0xe8
1409ca6ed: 48039590010000 add rdx, qword ptr [rbp + 0x190]
1409ca6f4: 49b89df836f25aac0f09 movabs r8, 0x90fac5af236f89d
1409ca6fe: 0f28de movaps xmm3, xmm6
1409ca701: e82a13faff call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
1409ca706: 488d4db0 lea rcx, [rbp - 0x50]
1409ca70a: 488d95a8000000 lea rdx, [rbp + 0xa8]
1409ca711: 41b8f8000000 mov r8d, 0xf8
1409ca717: e8e494f501 call 0x142923c00 ; 
1409ca71c: 80bd8e00000008 cmp byte ptr [rbp + 0x8e], 8
1409ca723: 742b je 0x1409ca750
1409ca725: 4180bfd004000003 cmp byte ptr [r15 + 0x4d0], 3
1409ca72d: 410f94c1 sete r9b
1409ca731: 488d4d68 lea rcx, [rbp + 0x68]
1409ca735: bae8000000 mov edx, 0xe8
1409ca73a: 48039598000000 add rdx, qword ptr [rbp + 0x98]
1409ca741: 49b8e9cb979665895e51 movabs r8, 0x515e89659697cbe9
1409ca74b: e80011faff call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
1409ca750: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409ca757: 488d55b0 lea rdx, [rbp - 0x50]
1409ca75b: 41b8f8000000 mov r8d, 0xf8
1409ca761: e89a94f501 call 0x142923c00 ; 
1409ca766: c685b601000001 mov byte ptr [rbp + 0x1b6], 1
1409ca76d: 48bac01c6c375d384e1b movabs rdx, 0x1b4e385d376c1cc0
1409ca777: 49b8ec37a4e2edd6388c movabs r8, 0x8c38d6ede2a437ec
1409ca781: 4889f9 mov rcx, rdi
1409ca784: e8672f71ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
1409ca789: 4885c0 test rax, rax
1409ca78c: 0f840c010000 je 0x1409ca89e
1409ca792: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1409ca799: 0f86ff000000 jbe 0x1409ca89e
1409ca79f: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1409ca7a6: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409ca7aa: 4885c9 test rcx, rcx
1409ca7ad: 0f84eb000000 je 0x1409ca89e
1409ca7b3: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
1409ca7ba: 48f7d1 not rcx
1409ca7bd: 488d0c49 lea rcx, [rcx + rcx*2]
1409ca7c1: 48c1e105 shl rcx, 5
1409ca7c5: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409ca7cb: 0f84cd000000 je 0x1409ca89e
1409ca7d1: 80bd8601000008 cmp byte ptr [rbp + 0x186], 8
1409ca7d8: 7452 je 0x1409ca82c
1409ca7da: 4801c8 add rax, rcx
1409ca7dd: 488b4010 mov rax, qword ptr [rax + 0x10]
1409ca7e1: f30f104004 movss xmm0, dword ptr [rax + 4]
1409ca7e6: f30f100d46280002 movss xmm1, dword ptr [rip + 0x2002846]
1409ca7ee: f30f59c8 mulss xmm1, xmm0
1409ca7f2: f30f101d0ab1f501 movss xmm3, dword ptr [rip + 0x1f5b10a]
1409ca7fa: f30f5cd8 subss xmm3, xmm0
1409ca7fe: f30f591d66280002 mulss xmm3, dword ptr [rip + 0x2002866]
1409ca806: f30f58d9 addss xmm3, xmm1
1409ca80a: 488d8d60010000 lea rcx, [rbp + 0x160]
1409ca811: bae8000000 mov edx, 0xe8
1409ca816: 48039590010000 add rdx, qword ptr [rbp + 0x190]
1409ca81d: 49b835d366f35044407f movabs r8, 0x7f404450f366d335
1409ca827: e80412faff call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
1409ca82c: 488d75b0 lea rsi, [rbp - 0x50]
1409ca830: 488d95a8000000 lea rdx, [rbp + 0xa8]
1409ca837: 41b8f8000000 mov r8d, 0xf8
1409ca83d: 4889f1 mov rcx, rsi
1409ca840: e8bb93f501 call 0x142923c00 ; 
1409ca845: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca84c: 4889f1 mov rcx, rsi
1409ca84f: 31d2 xor edx, edx
1409ca851: e88aa2f3ff call 0x140904ae0 ; country_core::render::nani::draw_ext::impl$0::nani_draw
1409ca856: 0f28b5c0010000 movaps xmm6, xmmword ptr [rbp + 0x1c0]
1409ca85d: 0f28bdd0010000 movaps xmm7, xmmword ptr [rbp + 0x1d0]
1409ca864: 4881c468020000 add rsp, 0x268
1409ca86b: 5b pop rbx
1409ca86c: 5f pop rdi
1409ca86d: 5e pop rsi
1409ca86e: 415e pop r14
1409ca870: 415f pop r15
1409ca872: 5d pop rbp
1409ca873: c3 ret 
1409ca874: c685b701000001 mov byte ptr [rbp + 0x1b7], 1
1409ca87b: 488d0d56620f02 lea rcx, [rip + 0x20f6256]
1409ca882: e889eaf001 call 0x1428d9310 ; core::option::unwrap_failed
1409ca887: eb28 jmp 0x1409ca8b1
1409ca889: c685b701000001 mov byte ptr [rbp + 0x1b7], 1
1409ca890: 488d0d59620f02 lea rcx, [rip + 0x20f6259]
1409ca897: e874eaf001 call 0x1428d9310 ; core::option::unwrap_failed
1409ca89c: eb13 jmp 0x1409ca8b1
1409ca89e: c685b601000001 mov byte ptr [rbp + 0x1b6], 1
1409ca8a5: 488d0d5c620f02 lea rcx, [rip + 0x20f625c]
1409ca8ac: e85feaf001 call 0x1428d9310 ; core::option::unwrap_failed
1409ca8b1: 0f0b ud2 
1409ca8b3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ca8c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ca8c5: 55 push rbp
1409ca8c6: 4157 push r15
1409ca8c8: 4156 push r14
1409ca8ca: 56 push rsi
1409ca8cb: 57 push rdi
1409ca8cc: 53 push rbx
1409ca8cd: 4883ec48 sub rsp, 0x48
1409ca8d1: 488daa80000000 lea rbp, [rdx + 0x80]
1409ca8d8: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
1409ca8dd: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
1409ca8e2: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca8e9: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409ca8f0: e8cb8cecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409ca8f5: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca8fc: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1409ca901: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1409ca906: 4883c448 add rsp, 0x48
1409ca90a: 5b pop rbx
1409ca90b: 5f pop rdi
1409ca90c: 5e pop rsi
1409ca90d: 415e pop r14
1409ca90f: 415f pop r15
1409ca911: 5d pop rbp
1409ca912: c3 ret 
1409ca913: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ca920: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ca925: 55 push rbp
1409ca926: 4157 push r15
1409ca928: 4156 push r14
1409ca92a: 56 push rsi
1409ca92b: 57 push rdi
1409ca92c: 53 push rbx
1409ca92d: 4883ec48 sub rsp, 0x48
1409ca931: 488daa80000000 lea rbp, [rdx + 0x80]
1409ca938: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
1409ca93d: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
1409ca942: c685b601000000 mov byte ptr [rbp + 0x1b6], 0
1409ca949: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409ca950: e86b8cecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409ca955: c685b601000000 mov byte ptr [rbp + 0x1b6], 0
1409ca95c: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1409ca961: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1409ca966: 4883c448 add rsp, 0x48
1409ca96a: 5b pop rbx
1409ca96b: 5f pop rdi
1409ca96c: 5e pop rsi
1409ca96d: 415e pop r14
1409ca96f: 415f pop r15
1409ca971: 5d pop rbp
1409ca972: c3 ret 
1409ca973: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ca980: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ca985: 55 push rbp
1409ca986: 4157 push r15
1409ca988: 4156 push r14
1409ca98a: 56 push rsi
1409ca98b: 57 push rdi
1409ca98c: 53 push rbx
1409ca98d: 4883ec48 sub rsp, 0x48
1409ca991: 488daa80000000 lea rbp, [rdx + 0x80]
1409ca998: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
1409ca99d: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
1409ca9a2: 80bdb601000000 cmp byte ptr [rbp + 0x1b6], 0
1409ca9a9: 742a je 0x1409ca9d5
1409ca9ab: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409ca9b2: e8098cecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409ca9b7: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca9be: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1409ca9c3: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1409ca9c8: 4883c448 add rsp, 0x48
1409ca9cc: 5b pop rbx
1409ca9cd: 5f pop rdi
1409ca9ce: 5e pop rsi
1409ca9cf: 415e pop r14
1409ca9d1: 415f pop r15
1409ca9d3: 5d pop rbp
1409ca9d4: c3 ret 
1409ca9d5: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca9dc: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1409ca9e1: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1409ca9e6: 4883c448 add rsp, 0x48
1409ca9ea: 5b pop rbx
1409ca9eb: 5f pop rdi
1409ca9ec: 5e pop rsi
1409ca9ed: 415e pop r14
1409ca9ef: 415f pop r15
1409ca9f1: 5d pop rbp
1409ca9f2: c3 ret 
1409ca9f3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409caa00: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409caa05: 55 push rbp
1409caa06: 4157 push r15
1409caa08: 4156 push r14
1409caa0a: 56 push rsi
1409caa0b: 57 push rdi
1409caa0c: 53 push rbx
1409caa0d: 4883ec48 sub rsp, 0x48
1409caa11: 488daa80000000 lea rbp, [rdx + 0x80]
1409caa18: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
1409caa1d: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
1409caa22: 80bdb701000000 cmp byte ptr [rbp + 0x1b7], 0
1409caa29: 7517 jne 0x1409caa42
1409caa2b: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1409caa30: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1409caa35: 4883c448 add rsp, 0x48
1409caa39: 5b pop rbx
1409caa3a: 5f pop rdi
1409caa3b: 5e pop rsi
1409caa3c: 415e pop r14
1409caa3e: 415f pop r15
1409caa40: 5d pop rbp
1409caa41: c3 ret 
1409caa42: 488b8da0010000 mov rcx, qword ptr [rbp + 0x1a0]
1409caa49: e8728becff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409caa4e: ebdb jmp 0x1409caa2b
1409caa50: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409caa55: 55 push rbp
1409caa56: 4157 push r15
1409caa58: 4156 push r14
1409caa5a: 56 push rsi
1409caa5b: 57 push rdi
1409caa5c: 53 push rbx
1409caa5d: 4883ec48 sub rsp, 0x48
1409caa61: 488daa80000000 lea rbp, [rdx + 0x80]
1409caa68: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
1409caa6d: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
1409caa72: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409caa79: 488d4db0 lea rcx, [rbp - 0x50]
1409caa7d: e83e8becff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409caa82: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409caa89: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1409caa8e: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1409caa93: 4883c448 add rsp, 0x48
1409caa97: 5b pop rbx
1409caa98: 5f pop rdi
1409caa99: 5e pop rsi
1409caa9a: 415e pop r14
1409caa9c: 415f pop r15
1409caa9e: 5d pop rbp
1409caa9f: c3 ret 
1409caaa0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409caaa5: 55 push rbp
1409caaa6: 4157 push r15
1409caaa8: 4156 push r14
1409caaaa: 56 push rsi
1409caaab: 57 push rdi
1409caaac: 53 push rbx
1409caaad: 4883ec48 sub rsp, 0x48
1409caab1: 488daa80000000 lea rbp, [rdx + 0x80]
1409caab8: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
1409caabd: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
1409caac2: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409caac9: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409caad0: e8eb8aecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409caad5: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409caadc: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1409caae1: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1409caae6: 4883c448 add rsp, 0x48
1409caaea: 5b pop rbx
1409caaeb: 5f pop rdi
1409caaec: 5e pop rsi
1409caaed: 415e pop r14
1409caaef: 415f pop r15
1409caaf1: 5d pop rbp
1409caaf2: c3 ret 
1409caaf3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409cab00: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409cab05: 55 push rbp
1409cab06: 4157 push r15
1409cab08: 4156 push r14
1409cab0a: 56 push rsi
1409cab0b: 57 push rdi
1409cab0c: 53 push rbx
1409cab0d: 4883ec48 sub rsp, 0x48
1409cab11: 488daa80000000 lea rbp, [rdx + 0x80]
1409cab18: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
1409cab1d: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
1409cab22: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409cab29: 488d4db0 lea rcx, [rbp - 0x50]
1409cab2d: e88e8aecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409cab32: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409cab39: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1409cab3e: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1409cab43: 4883c448 add rsp, 0x48
1409cab47: 5b pop rbx
1409cab48: 5f pop rdi
1409cab49: 5e pop rsi
1409cab4a: 415e pop r14
1409cab4c: 415f pop r15
1409cab4e: 5d pop rbp
1409cab4f: c3 ret 
1409cab50: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409cab55: 55 push rbp
1409cab56: 4157 push r15
1409cab58: 4156 push r14
1409cab5a: 56 push rsi
1409cab5b: 57 push rdi
1409cab5c: 53 push rbx
1409cab5d: 4883ec48 sub rsp, 0x48
1409cab61: 488daa80000000 lea rbp, [rdx + 0x80]
1409cab68: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
1409cab6d: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
1409cab72: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409cab79: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409cab80: e83b8aecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409cab85: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409cab8c: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1409cab91: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1409cab96: 4883c448 add rsp, 0x48
1409cab9a: 5b pop rbx
1409cab9b: 5f pop rdi
1409cab9c: 5e pop rsi
1409cab9d: 415e pop r14
1409cab9f: 415f pop r15
1409caba1: 5d pop rbp
1409caba2: c3 ret 
1409caba3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409cabb0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409cabb5: 55 push rbp
1409cabb6: 4157 push r15
1409cabb8: 4156 push r14
1409cabba: 56 push rsi
1409cabbb: 57 push rdi
1409cabbc: 53 push rbx
1409cabbd: 4883ec48 sub rsp, 0x48
1409cabc1: 488daa80000000 lea rbp, [rdx + 0x80]
1409cabc8: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
1409cabcd: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
1409cabd2: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409cabd9: 488d4db0 lea rcx, [rbp - 0x50]
1409cabdd: e8de89ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409cabe2: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409cabe9: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1409cabee: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1409cabf3: 4883c448 add rsp, 0x48
1409cabf7: 5b pop rbx
1409cabf8: 5f pop rdi
1409cabf9: 5e pop rsi
1409cabfa: 415e pop r14
1409cabfc: 415f pop r15
1409cabfe: 5d pop rbp
1409cabff: c3 ret 
1409cac00: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409cac05: 55 push rbp
1409cac06: 4157 push r15
1409cac08: 4156 push r14
1409cac0a: 56 push rsi
1409cac0b: 57 push rdi
1409cac0c: 53 push rbx
1409cac0d: 4883ec48 sub rsp, 0x48
1409cac11: 488daa80000000 lea rbp, [rdx + 0x80]
1409cac18: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
1409cac1d: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
1409cac22: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409cac29: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409cac30: e88b89ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409cac35: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409cac3c: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1409cac41: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1409cac46: 4883c448 add rsp, 0x48
1409cac4a: 5b pop rbx
1409cac4b: 5f pop rdi
1409cac4c: 5e pop rsi
1409cac4d: 415e pop r14
1409cac4f: 415f pop r15
1409cac51: 5d pop rbp
1409cac52: c3 ret 
1409cac53: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409cac60: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409cac65: 55 push rbp
1409cac66: 4157 push r15
1409cac68: 4156 push r14
1409cac6a: 56 push rsi
1409cac6b: 57 push rdi
1409cac6c: 53 push rbx
1409cac6d: 4883ec48 sub rsp, 0x48
1409cac71: 488daa80000000 lea rbp, [rdx + 0x80]
1409cac78: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
1409cac7d: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
1409cac82: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409cac89: 488d4db0 lea rcx, [rbp - 0x50]
1409cac8d: e82e89ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409cac92: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409cac99: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1409cac9e: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1409caca3: 4883c448 add rsp, 0x48
1409caca7: 5b pop rbx
1409caca8: 5f pop rdi
1409caca9: 5e pop rsi
1409cacaa: 415e pop r14
1409cacac: 415f pop r15
1409cacae: 5d pop rbp
1409cacaf: c3 ret 

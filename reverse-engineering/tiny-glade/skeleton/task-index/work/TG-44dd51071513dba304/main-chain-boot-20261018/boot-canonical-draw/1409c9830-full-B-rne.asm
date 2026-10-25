1409c9830: 55 push rbp
1409c9831: 4157 push r15
1409c9833: 4156 push r14
1409c9835: 56 push rsi
1409c9836: 57 push rdi
1409c9837: 53 push rbx
1409c9838: 4881ec58020000 sub rsp, 0x258
1409c983f: 488dac2480000000 lea rbp, [rsp + 0x80]
1409c9847: 0f29b5c0010000 movaps xmmword ptr [rbp + 0x1c0], xmm6
1409c984e: 48c785b8010000feffffff mov qword ptr [rbp + 0x1b8], 0xfffffffffffffffe
1409c9859: 4c89c6 mov rsi, r8
1409c985c: 80b9f000000000 cmp byte ptr [rcx + 0xf0], 0
1409c9863: 7411 je 0x1409c9876
1409c9865: 0fb681a8000000 movzx eax, byte ptr [rcx + 0xa8]
1409c986c: 24f3 and al, 0xf3
1409c986e: 0c04 or al, 4
1409c9870: 8881a8000000 mov byte ptr [rcx + 0xa8], al
1409c9876: 48898da0010000 mov qword ptr [rbp + 0x1a0], rcx
1409c987d: 488d7e38 lea rdi, [rsi + 0x38]
1409c9881: c685b701000001 mov byte ptr [rbp + 0x1b7], 1
1409c9888: 48ba3b4166302619f5d2 movabs rdx, 0xd2f519263066413b
1409c9892: 49b8e45925965fb19251 movabs r8, 0x5192b15f962559e4
1409c989c: 4889f9 mov rcx, rdi
1409c989f: e84c3e71ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
1409c98a4: 4885c0 test rax, rax
1409c98a7: 0f84a2050000 je 0x1409c9e4f
1409c98ad: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1409c98b4: 0f8695050000 jbe 0x1409c9e4f
1409c98ba: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1409c98c1: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409c98c5: 4885c9 test rcx, rcx
1409c98c8: 0f8481050000 je 0x1409c9e4f
1409c98ce: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
1409c98d5: 48f7d1 not rcx
1409c98d8: 488d0c49 lea rcx, [rcx + rcx*2]
1409c98dc: 48c1e105 shl rcx, 5
1409c98e0: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409c98e6: 0f8463050000 je 0x1409c9e4f
1409c98ec: 4801c8 add rax, rcx
1409c98ef: 488b4010 mov rax, qword ptr [rax + 0x10]
1409c98f3: f30f107004 movss xmm6, dword ptr [rax + 4]
1409c98f8: c685b701000001 mov byte ptr [rbp + 0x1b7], 1
1409c98ff: 48baffa3086aee51a4bb movabs rdx, 0xbba451ee6a08a3ff
1409c9909: 49b89df2bcfe158d008d movabs r8, 0x8d008d15febcf29d
1409c9913: 4889f9 mov rcx, rdi
1409c9916: e8d53d71ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
1409c991b: 4885c0 test rax, rax
1409c991e: 0f8440050000 je 0x1409c9e64
1409c9924: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1409c992b: 0f8633050000 jbe 0x1409c9e64
1409c9931: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1409c9938: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409c993c: 4885c9 test rcx, rcx
1409c993f: 0f841f050000 je 0x1409c9e64
1409c9945: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
1409c994c: 48f7d1 not rcx
1409c994f: 488d0c49 lea rcx, [rcx + rcx*2]
1409c9953: 48c1e105 shl rcx, 5
1409c9957: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409c995d: 0f8401050000 je 0x1409c9e64
1409c9963: 4801c8 add rax, rcx
1409c9966: 4c8b7810 mov r15, qword ptr [rax + 0x10]
1409c996a: 488d9da8000000 lea rbx, [rbp + 0xa8]
1409c9971: 41b8f8000000 mov r8d, 0xf8
1409c9977: 4889d9 mov rcx, rbx
1409c997a: 488b95a0010000 mov rdx, qword ptr [rbp + 0x1a0]
1409c9981: e87aa2f501 call 0x142923c00 ; 
1409c9986: 48b834e0d39b5b61e10c movabs rax, 0xce1615b9bd3e034
1409c9990: 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
1409c9997: c745b002000000 mov dword ptr [rbp - 0x50], 2
1409c999e: 488d45b0 lea rax, [rbp - 0x50]
1409c99a2: 4889442420 mov qword ptr [rsp + 0x20], rax
1409c99a7: 488d8da8010000 lea rcx, [rbp + 0x1a8]
1409c99ae: 49b89b7865828f4d6974 movabs r8, 0x74694d8f8265789b
1409c99b8: 4889da mov rdx, rbx
1409c99bb: 41b11d mov r9b, 0x1d
1409c99be: e84decf3ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409c99c3: 488d5db0 lea rbx, [rbp - 0x50]
1409c99c7: 4c8db5a8000000 lea r14, [rbp + 0xa8]
1409c99ce: 41b8f8000000 mov r8d, 0xf8
1409c99d4: 4889d9 mov rcx, rbx
1409c99d7: 4c89f2 mov rdx, r14
1409c99da: e821a2f501 call 0x142923c00 ; 
1409c99df: 48b8349f569b93e6dc32 movabs rax, 0x32dce6939b569f34
1409c99e9: 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
1409c99f0: c785a800000002000000 mov dword ptr [rbp + 0xa8], 2
1409c99fa: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409c99ff: 488d8da8010000 lea rcx, [rbp + 0x1a8]
1409c9a06: 49b87bb6ad6ddb6820f8 movabs r8, 0xf82068db6dadb67b
1409c9a10: 4889da mov rdx, rbx
1409c9a13: 41b11d mov r9b, 0x1d
1409c9a16: e8f5ebf3ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409c9a1b: 488d9da8000000 lea rbx, [rbp + 0xa8]
1409c9a22: 4c8d75b0 lea r14, [rbp - 0x50]
1409c9a26: 41b8f8000000 mov r8d, 0xf8
1409c9a2c: 4889d9 mov rcx, rbx
1409c9a2f: 4c89f2 mov rdx, r14
1409c9a32: e8c9a1f501 call 0x142923c00 ; 
1409c9a37: 48b8a66295d988a41582 movabs rax, 0x8215a488d99562a6
1409c9a41: 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
1409c9a48: c745b002000000 mov dword ptr [rbp - 0x50], 2
1409c9a4f: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409c9a54: 488d8da8010000 lea rcx, [rbp + 0x1a8]
1409c9a5b: 49b88f262e077f078c67 movabs r8, 0x678c077f072e268f
1409c9a65: 4889da mov rdx, rbx
1409c9a68: 41b11d mov r9b, 0x1d
1409c9a6b: e8a0ebf3ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409c9a70: 488d5db0 lea rbx, [rbp - 0x50]
1409c9a74: 4c8db5a8000000 lea r14, [rbp + 0xa8]
1409c9a7b: 41b8f8000000 mov r8d, 0xf8
1409c9a81: 4889d9 mov rcx, rbx
1409c9a84: 4c89f2 mov rdx, r14
1409c9a87: e874a1f501 call 0x142923c00 ; 
1409c9a8c: 48b8d4983f00691d2093 movabs rax, 0x93201d69003f98d4
1409c9a96: 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
1409c9a9d: c785a800000002000000 mov dword ptr [rbp + 0xa8], 2
1409c9aa7: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409c9aac: 488d8da8010000 lea rcx, [rbp + 0x1a8]
1409c9ab3: 49b81e7b818ffc601056 movabs r8, 0x561060fc8f817b1e
1409c9abd: 4889da mov rdx, rbx
1409c9ac0: 41b11d mov r9b, 0x1d
1409c9ac3: e848ebf3ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409c9ac8: 488d9da8000000 lea rbx, [rbp + 0xa8]
1409c9acf: 4c8d75b0 lea r14, [rbp - 0x50]
1409c9ad3: 41b8f8000000 mov r8d, 0xf8
1409c9ad9: 4889d9 mov rcx, rbx
1409c9adc: 4c89f2 mov rdx, r14
1409c9adf: e81ca1f501 call 0x142923c00 ; 
1409c9ae4: 48b84d7cb71113b012aa movabs rax, 0xaa12b01311b77c4d
1409c9aee: 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
1409c9af5: c745b002000000 mov dword ptr [rbp - 0x50], 2
1409c9afc: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409c9b01: 488d8da8010000 lea rcx, [rbp + 0x1a8]
1409c9b08: 49b802e6e599c585e462 movabs r8, 0x62e485c599e5e602
1409c9b12: 4889da mov rdx, rbx
1409c9b15: 41b11d mov r9b, 0x1d
1409c9b18: e8f3eaf3ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409c9b1d: 488d5db0 lea rbx, [rbp - 0x50]
1409c9b21: 4c8db5a8000000 lea r14, [rbp + 0xa8]
1409c9b28: 41b8f8000000 mov r8d, 0xf8
1409c9b2e: 4889d9 mov rcx, rbx
1409c9b31: 4c89f2 mov rdx, r14
1409c9b34: e8c7a0f501 call 0x142923c00 ; 
1409c9b39: 48b8b7c7f1ac39bb4d00 movabs rax, 0x4dbb39acf1c7b7
1409c9b43: 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
1409c9b4a: c785a800000002000000 mov dword ptr [rbp + 0xa8], 2
1409c9b54: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409c9b59: 488d8da8010000 lea rcx, [rbp + 0x1a8]
1409c9b60: 49b88cb1422a0801a530 movabs r8, 0x30a501082a42b18c
1409c9b6a: 4889da mov rdx, rbx
1409c9b6d: 41b11d mov r9b, 0x1d
1409c9b70: e89beaf3ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409c9b75: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409c9b7c: 488d55b0 lea rdx, [rbp - 0x50]
1409c9b80: 41b8f8000000 mov r8d, 0xf8
1409c9b86: e875a0f501 call 0x142923c00 ; 
1409c9b8b: 80bd8601000008 cmp byte ptr [rbp + 0x186], 8
1409c9b92: 7425 je 0x1409c9bb9
1409c9b94: 488d8d60010000 lea rcx, [rbp + 0x160]
1409c9b9b: bae8000000 mov edx, 0xe8
1409c9ba0: 48039590010000 add rdx, qword ptr [rbp + 0x190]
1409c9ba7: 49b8694d11bdea3f2acb movabs r8, 0xcb2a3feabd114d69
1409c9bb1: 0f28de movaps xmm3, xmm6
1409c9bb4: e8771efaff call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
1409c9bb9: 488d4db0 lea rcx, [rbp - 0x50]
1409c9bbd: 488d95a8000000 lea rdx, [rbp + 0xa8]
1409c9bc4: 41b8f8000000 mov r8d, 0xf8
1409c9bca: e831a0f501 call 0x142923c00 ; 
1409c9bcf: c685b601000001 mov byte ptr [rbp + 0x1b6], 1
1409c9bd6: 48baffa3086aee51a4bb movabs rdx, 0xbba451ee6a08a3ff
1409c9be0: 49b89df2bcfe158d008d movabs r8, 0x8d008d15febcf29d
1409c9bea: 4889f9 mov rcx, rdi
1409c9bed: e8fe3a71ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
1409c9bf2: 4885c0 test rax, rax
1409c9bf5: 0f847e020000 je 0x1409c9e79
1409c9bfb: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1409c9c02: 0f8671020000 jbe 0x1409c9e79
1409c9c08: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1409c9c0f: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409c9c13: 4885c9 test rcx, rcx
1409c9c16: 0f845d020000 je 0x1409c9e79
1409c9c1c: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
1409c9c23: 48f7d1 not rcx
1409c9c26: 488d0c49 lea rcx, [rcx + rcx*2]
1409c9c2a: 48c1e105 shl rcx, 5
1409c9c2e: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409c9c34: 0f843f020000 je 0x1409c9e79
1409c9c3a: 4801c8 add rax, rcx
1409c9c3d: 488b5010 mov rdx, qword ptr [rax + 0x10]
1409c9c41: c685b601000001 mov byte ptr [rbp + 0x1b6], 1
1409c9c48: 488d8da8010000 lea rcx, [rbp + 0x1a8]
1409c9c4f: e8ecd02c00 call 0x140c96d40 ; utils::camera::CameraMatrices::eye_position
1409c9c54: 80bd8e00000008 cmp byte ptr [rbp + 0x8e], 8
1409c9c5b: 7440 je 0x1409c9c9d
1409c9c5d: 488d4d68 lea rcx, [rbp + 0x68]
1409c9c61: bae8000000 mov edx, 0xe8
1409c9c66: 48039598000000 add rdx, qword ptr [rbp + 0x98]
1409c9c6d: 8b85b0010000 mov eax, dword ptr [rbp + 0x1b0]
1409c9c73: 8985b0000000 mov dword ptr [rbp + 0xb0], eax
1409c9c79: 488b85a8010000 mov rax, qword ptr [rbp + 0x1a8]
1409c9c80: 488985a8000000 mov qword ptr [rbp + 0xa8], rax
1409c9c87: 49b890393d870bb3bd4b movabs r8, 0x4bbdb30b873d3990
1409c9c91: 4c8d8da8000000 lea r9, [rbp + 0xa8]
1409c9c98: e8c319faff call 0x14096b660 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec3::Vec3>,glam::f32::vec3::Vec3>
1409c9c9d: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409c9ca4: 488d55b0 lea rdx, [rbp - 0x50]
1409c9ca8: 41b8f8000000 mov r8d, 0xf8
1409c9cae: e84d9ff501 call 0x142923c00 ; 
1409c9cb3: 80bd8601000008 cmp byte ptr [rbp + 0x186], 8
1409c9cba: 742e je 0x1409c9cea
1409c9cbc: 4180bfd004000003 cmp byte ptr [r15 + 0x4d0], 3
1409c9cc4: 410f94c1 sete r9b
1409c9cc8: 488d8d60010000 lea rcx, [rbp + 0x160]
1409c9ccf: bae8000000 mov edx, 0xe8
1409c9cd4: 48039590010000 add rdx, qword ptr [rbp + 0x190]
1409c9cdb: 49b8e9cb979665895e51 movabs r8, 0x515e89659697cbe9
1409c9ce5: e8661bfaff call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
1409c9cea: 488d4db0 lea rcx, [rbp - 0x50]
1409c9cee: 488d95a8000000 lea rdx, [rbp + 0xa8]
1409c9cf5: 41b8f8000000 mov r8d, 0xf8
1409c9cfb: e8009ff501 call 0x142923c00 ; 
1409c9d00: c685b501000001 mov byte ptr [rbp + 0x1b5], 1
1409c9d07: 48bac01c6c375d384e1b movabs rdx, 0x1b4e385d376c1cc0
1409c9d11: 49b8ec37a4e2edd6388c movabs r8, 0x8c38d6ede2a437ec
1409c9d1b: 4889f9 mov rcx, rdi
1409c9d1e: e8cd3971ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
1409c9d23: 4885c0 test rax, rax
1409c9d26: 0f8462010000 je 0x1409c9e8e
1409c9d2c: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1409c9d33: 0f8655010000 jbe 0x1409c9e8e
1409c9d39: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1409c9d40: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409c9d44: 4885c9 test rcx, rcx
1409c9d47: 0f8441010000 je 0x1409c9e8e
1409c9d4d: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
1409c9d54: 48f7d1 not rcx
1409c9d57: 488d0c49 lea rcx, [rcx + rcx*2]
1409c9d5b: 48c1e105 shl rcx, 5
1409c9d5f: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409c9d65: 0f8423010000 je 0x1409c9e8e
1409c9d6b: 80bd8e00000008 cmp byte ptr [rbp + 0x8e], 8
1409c9d72: 744f je 0x1409c9dc3
1409c9d74: 4801c8 add rax, rcx
1409c9d77: 488b4010 mov rax, qword ptr [rax + 0x10]
1409c9d7b: f30f104004 movss xmm0, dword ptr [rax + 4]
1409c9d80: f30f100dac320002 movss xmm1, dword ptr [rip + 0x20032ac]
1409c9d88: f30f59c8 mulss xmm1, xmm0
1409c9d8c: f30f101d70bbf501 movss xmm3, dword ptr [rip + 0x1f5bb70]
1409c9d94: f30f5cd8 subss xmm3, xmm0
1409c9d98: f30f591dcc320002 mulss xmm3, dword ptr [rip + 0x20032cc]
1409c9da0: f30f58d9 addss xmm3, xmm1
1409c9da4: 488d4d68 lea rcx, [rbp + 0x68]
1409c9da8: bae8000000 mov edx, 0xe8
1409c9dad: 48039598000000 add rdx, qword ptr [rbp + 0x98]
1409c9db4: 49b835d366f35044407f movabs r8, 0x7f404450f366d335
1409c9dbe: e86d1cfaff call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
1409c9dc3: 488db5a8000000 lea rsi, [rbp + 0xa8]
1409c9dca: 488d7db0 lea rdi, [rbp - 0x50]
1409c9dce: 41b8f8000000 mov r8d, 0xf8
1409c9dd4: 4889f1 mov rcx, rsi
1409c9dd7: 4889fa mov rdx, rdi
1409c9dda: e8219ef501 call 0x142923c00 ; 
1409c9ddf: c685a80100000e mov byte ptr [rbp + 0x1a8], 0xe
1409c9de6: c745b002000000 mov dword ptr [rbp - 0x50], 2
1409c9ded: 48897c2420 mov qword ptr [rsp + 0x20], rdi
1409c9df2: 488d8da8010000 lea rcx, [rbp + 0x1a8]
1409c9df9: 49b8b9d21286b398e04e movabs r8, 0x4ee098b38612d2b9
1409c9e03: 4889f2 mov rdx, rsi
1409c9e06: 41b11d mov r9b, 0x1d
1409c9e09: e842eef3ff call 0x140908c50 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>
1409c9e0e: 488d75b0 lea rsi, [rbp - 0x50]
1409c9e12: 488d95a8000000 lea rdx, [rbp + 0xa8]
1409c9e19: 41b8f8000000 mov r8d, 0xf8
1409c9e1f: 4889f1 mov rcx, rsi
1409c9e22: e8d99df501 call 0x142923c00 ; 
1409c9e27: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409c9e2e: 4889f1 mov rcx, rsi
1409c9e31: b201 mov dl, 1
1409c9e33: e8a8acf3ff call 0x140904ae0 ; country_core::render::nani::draw_ext::impl$0::nani_draw
1409c9e38: 0f28b5c0010000 movaps xmm6, xmmword ptr [rbp + 0x1c0]
1409c9e3f: 4881c458020000 add rsp, 0x258
1409c9e46: 5b pop rbx
1409c9e47: 5f pop rdi
1409c9e48: 5e pop rsi
1409c9e49: 415e pop r14
1409c9e4b: 415f pop r15
1409c9e4d: 5d pop rbp
1409c9e4e: c3 ret 
1409c9e4f: c685b701000001 mov byte ptr [rbp + 0x1b7], 1
1409c9e56: 488d0d1b6c0f02 lea rcx, [rip + 0x20f6c1b]
1409c9e5d: e8aef4f001 call 0x1428d9310 ; core::option::unwrap_failed
1409c9e62: eb3d jmp 0x1409c9ea1
1409c9e64: c685b701000001 mov byte ptr [rbp + 0x1b7], 1
1409c9e6b: 488d0d1e6c0f02 lea rcx, [rip + 0x20f6c1e]
1409c9e72: e899f4f001 call 0x1428d9310 ; core::option::unwrap_failed
1409c9e77: eb28 jmp 0x1409c9ea1
1409c9e79: c685b601000001 mov byte ptr [rbp + 0x1b6], 1
1409c9e80: 488d0d216c0f02 lea rcx, [rip + 0x20f6c21]
1409c9e87: e884f4f001 call 0x1428d9310 ; core::option::unwrap_failed
1409c9e8c: eb13 jmp 0x1409c9ea1
1409c9e8e: c685b501000001 mov byte ptr [rbp + 0x1b5], 1
1409c9e95: 488d0d246c0f02 lea rcx, [rip + 0x20f6c24]
1409c9e9c: e86ff4f001 call 0x1428d9310 ; core::option::unwrap_failed
1409c9ea1: 0f0b ud2 
1409c9ea3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409c9eb0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409c9eb5: 55 push rbp
1409c9eb6: 4157 push r15
1409c9eb8: 4156 push r14
1409c9eba: 56 push rsi
1409c9ebb: 57 push rdi
1409c9ebc: 53 push rbx
1409c9ebd: 4883ec38 sub rsp, 0x38
1409c9ec1: 488daa80000000 lea rbp, [rdx + 0x80]
1409c9ec8: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409c9ecd: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409c9ed4: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409c9edb: e8e096ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409c9ee0: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409c9ee7: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409c9eec: 4883c438 add rsp, 0x38
1409c9ef0: 5b pop rbx
1409c9ef1: 5f pop rdi
1409c9ef2: 5e pop rsi
1409c9ef3: 415e pop r14
1409c9ef5: 415f pop r15
1409c9ef7: 5d pop rbp
1409c9ef8: c3 ret 
1409c9ef9: 0f1f8000000000 nop dword ptr [rax]
1409c9f00: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409c9f05: 55 push rbp
1409c9f06: 4157 push r15
1409c9f08: 4156 push r14
1409c9f0a: 56 push rsi
1409c9f0b: 57 push rdi
1409c9f0c: 53 push rbx
1409c9f0d: 4883ec38 sub rsp, 0x38
1409c9f11: 488daa80000000 lea rbp, [rdx + 0x80]
1409c9f18: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409c9f1d: c685b501000000 mov byte ptr [rbp + 0x1b5], 0
1409c9f24: 488d4db0 lea rcx, [rbp - 0x50]
1409c9f28: e89396ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409c9f2d: c685b501000000 mov byte ptr [rbp + 0x1b5], 0
1409c9f34: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409c9f39: 4883c438 add rsp, 0x38
1409c9f3d: 5b pop rbx
1409c9f3e: 5f pop rdi
1409c9f3f: 5e pop rsi
1409c9f40: 415e pop r14
1409c9f42: 415f pop r15
1409c9f44: 5d pop rbp
1409c9f45: c3 ret 
1409c9f46: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409c9f50: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409c9f55: 55 push rbp
1409c9f56: 4157 push r15
1409c9f58: 4156 push r14
1409c9f5a: 56 push rsi
1409c9f5b: 57 push rdi
1409c9f5c: 53 push rbx
1409c9f5d: 4883ec38 sub rsp, 0x38
1409c9f61: 488daa80000000 lea rbp, [rdx + 0x80]
1409c9f68: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409c9f6d: 80bdb501000000 cmp byte ptr [rbp + 0x1b5], 0
1409c9f74: 7422 je 0x1409c9f98
1409c9f76: 488d4db0 lea rcx, [rbp - 0x50]
1409c9f7a: e84196ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409c9f7f: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409c9f86: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409c9f8b: 4883c438 add rsp, 0x38
1409c9f8f: 5b pop rbx
1409c9f90: 5f pop rdi
1409c9f91: 5e pop rsi
1409c9f92: 415e pop r14
1409c9f94: 415f pop r15
1409c9f96: 5d pop rbp
1409c9f97: c3 ret 
1409c9f98: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409c9f9f: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409c9fa4: 4883c438 add rsp, 0x38
1409c9fa8: 5b pop rbx
1409c9fa9: 5f pop rdi
1409c9faa: 5e pop rsi
1409c9fab: 415e pop r14
1409c9fad: 415f pop r15
1409c9faf: 5d pop rbp
1409c9fb0: c3 ret 
1409c9fb1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409c9fc0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409c9fc5: 55 push rbp
1409c9fc6: 4157 push r15
1409c9fc8: 4156 push r14
1409c9fca: 56 push rsi
1409c9fcb: 57 push rdi
1409c9fcc: 53 push rbx
1409c9fcd: 4883ec38 sub rsp, 0x38
1409c9fd1: 488daa80000000 lea rbp, [rdx + 0x80]
1409c9fd8: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409c9fdd: 80bdb601000000 cmp byte ptr [rbp + 0x1b6], 0
1409c9fe4: 7422 je 0x1409ca008
1409c9fe6: 488d4db0 lea rcx, [rbp - 0x50]
1409c9fea: e8d195ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409c9fef: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409c9ff6: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409c9ffb: 4883c438 add rsp, 0x38
1409c9fff: 5b pop rbx
1409ca000: 5f pop rdi
1409ca001: 5e pop rsi
1409ca002: 415e pop r14
1409ca004: 415f pop r15
1409ca006: 5d pop rbp
1409ca007: c3 ret 
1409ca008: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca00f: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409ca014: 4883c438 add rsp, 0x38
1409ca018: 5b pop rbx
1409ca019: 5f pop rdi
1409ca01a: 5e pop rsi
1409ca01b: 415e pop r14
1409ca01d: 415f pop r15
1409ca01f: 5d pop rbp
1409ca020: c3 ret 
1409ca021: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ca030: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ca035: 55 push rbp
1409ca036: 4157 push r15
1409ca038: 4156 push r14
1409ca03a: 56 push rsi
1409ca03b: 57 push rdi
1409ca03c: 53 push rbx
1409ca03d: 4883ec38 sub rsp, 0x38
1409ca041: 488daa80000000 lea rbp, [rdx + 0x80]
1409ca048: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409ca04d: 80bdb701000000 cmp byte ptr [rbp + 0x1b7], 0
1409ca054: 7512 jne 0x1409ca068
1409ca056: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409ca05b: 4883c438 add rsp, 0x38
1409ca05f: 5b pop rbx
1409ca060: 5f pop rdi
1409ca061: 5e pop rsi
1409ca062: 415e pop r14
1409ca064: 415f pop r15
1409ca066: 5d pop rbp
1409ca067: c3 ret 
1409ca068: 488b8da0010000 mov rcx, qword ptr [rbp + 0x1a0]
1409ca06f: e84c95ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409ca074: ebe0 jmp 0x1409ca056
1409ca076: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ca080: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ca085: 55 push rbp
1409ca086: 4157 push r15
1409ca088: 4156 push r14
1409ca08a: 56 push rsi
1409ca08b: 57 push rdi
1409ca08c: 53 push rbx
1409ca08d: 4883ec38 sub rsp, 0x38
1409ca091: 488daa80000000 lea rbp, [rdx + 0x80]
1409ca098: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409ca09d: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca0a4: 488d4db0 lea rcx, [rbp - 0x50]
1409ca0a8: e81395ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409ca0ad: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca0b4: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409ca0b9: 4883c438 add rsp, 0x38
1409ca0bd: 5b pop rbx
1409ca0be: 5f pop rdi
1409ca0bf: 5e pop rsi
1409ca0c0: 415e pop r14
1409ca0c2: 415f pop r15
1409ca0c4: 5d pop rbp
1409ca0c5: c3 ret 
1409ca0c6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ca0d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ca0d5: 55 push rbp
1409ca0d6: 4157 push r15
1409ca0d8: 4156 push r14
1409ca0da: 56 push rsi
1409ca0db: 57 push rdi
1409ca0dc: 53 push rbx
1409ca0dd: 4883ec38 sub rsp, 0x38
1409ca0e1: 488daa80000000 lea rbp, [rdx + 0x80]
1409ca0e8: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409ca0ed: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca0f4: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409ca0fb: e8c094ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409ca100: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca107: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409ca10c: 4883c438 add rsp, 0x38
1409ca110: 5b pop rbx
1409ca111: 5f pop rdi
1409ca112: 5e pop rsi
1409ca113: 415e pop r14
1409ca115: 415f pop r15
1409ca117: 5d pop rbp
1409ca118: c3 ret 
1409ca119: 0f1f8000000000 nop dword ptr [rax]
1409ca120: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ca125: 55 push rbp
1409ca126: 4157 push r15
1409ca128: 4156 push r14
1409ca12a: 56 push rsi
1409ca12b: 57 push rdi
1409ca12c: 53 push rbx
1409ca12d: 4883ec38 sub rsp, 0x38
1409ca131: 488daa80000000 lea rbp, [rdx + 0x80]
1409ca138: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409ca13d: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca144: 488d4db0 lea rcx, [rbp - 0x50]
1409ca148: e87394ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409ca14d: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca154: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409ca159: 4883c438 add rsp, 0x38
1409ca15d: 5b pop rbx
1409ca15e: 5f pop rdi
1409ca15f: 5e pop rsi
1409ca160: 415e pop r14
1409ca162: 415f pop r15
1409ca164: 5d pop rbp
1409ca165: c3 ret 
1409ca166: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ca170: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ca175: 55 push rbp
1409ca176: 4157 push r15
1409ca178: 4156 push r14
1409ca17a: 56 push rsi
1409ca17b: 57 push rdi
1409ca17c: 53 push rbx
1409ca17d: 4883ec38 sub rsp, 0x38
1409ca181: 488daa80000000 lea rbp, [rdx + 0x80]
1409ca188: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409ca18d: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca194: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409ca19b: e82094ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409ca1a0: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca1a7: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409ca1ac: 4883c438 add rsp, 0x38
1409ca1b0: 5b pop rbx
1409ca1b1: 5f pop rdi
1409ca1b2: 5e pop rsi
1409ca1b3: 415e pop r14
1409ca1b5: 415f pop r15
1409ca1b7: 5d pop rbp
1409ca1b8: c3 ret 
1409ca1b9: 0f1f8000000000 nop dword ptr [rax]
1409ca1c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ca1c5: 55 push rbp
1409ca1c6: 4157 push r15
1409ca1c8: 4156 push r14
1409ca1ca: 56 push rsi
1409ca1cb: 57 push rdi
1409ca1cc: 53 push rbx
1409ca1cd: 4883ec38 sub rsp, 0x38
1409ca1d1: 488daa80000000 lea rbp, [rdx + 0x80]
1409ca1d8: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409ca1dd: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca1e4: 488d4db0 lea rcx, [rbp - 0x50]
1409ca1e8: e8d393ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409ca1ed: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca1f4: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409ca1f9: 4883c438 add rsp, 0x38
1409ca1fd: 5b pop rbx
1409ca1fe: 5f pop rdi
1409ca1ff: 5e pop rsi
1409ca200: 415e pop r14
1409ca202: 415f pop r15
1409ca204: 5d pop rbp
1409ca205: c3 ret 
1409ca206: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ca210: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ca215: 55 push rbp
1409ca216: 4157 push r15
1409ca218: 4156 push r14
1409ca21a: 56 push rsi
1409ca21b: 57 push rdi
1409ca21c: 53 push rbx
1409ca21d: 4883ec38 sub rsp, 0x38
1409ca221: 488daa80000000 lea rbp, [rdx + 0x80]
1409ca228: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409ca22d: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca234: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409ca23b: e88093ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409ca240: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca247: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409ca24c: 4883c438 add rsp, 0x38
1409ca250: 5b pop rbx
1409ca251: 5f pop rdi
1409ca252: 5e pop rsi
1409ca253: 415e pop r14
1409ca255: 415f pop r15
1409ca257: 5d pop rbp
1409ca258: c3 ret 
1409ca259: 0f1f8000000000 nop dword ptr [rax]
1409ca260: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ca265: 55 push rbp
1409ca266: 4157 push r15
1409ca268: 4156 push r14
1409ca26a: 56 push rsi
1409ca26b: 57 push rdi
1409ca26c: 53 push rbx
1409ca26d: 4883ec38 sub rsp, 0x38
1409ca271: 488daa80000000 lea rbp, [rdx + 0x80]
1409ca278: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409ca27d: c685b601000000 mov byte ptr [rbp + 0x1b6], 0
1409ca284: 488d4db0 lea rcx, [rbp - 0x50]
1409ca288: e83393ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409ca28d: c685b601000000 mov byte ptr [rbp + 0x1b6], 0
1409ca294: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409ca299: 4883c438 add rsp, 0x38
1409ca29d: 5b pop rbx
1409ca29e: 5f pop rdi
1409ca29f: 5e pop rsi
1409ca2a0: 415e pop r14
1409ca2a2: 415f pop r15
1409ca2a4: 5d pop rbp
1409ca2a5: c3 ret 
1409ca2a6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409ca2b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ca2b5: 55 push rbp
1409ca2b6: 4157 push r15
1409ca2b8: 4156 push r14
1409ca2ba: 56 push rsi
1409ca2bb: 57 push rdi
1409ca2bc: 53 push rbx
1409ca2bd: 4883ec38 sub rsp, 0x38
1409ca2c1: 488daa80000000 lea rbp, [rdx + 0x80]
1409ca2c8: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409ca2cd: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca2d4: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409ca2db: e8e092ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409ca2e0: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca2e7: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409ca2ec: 4883c438 add rsp, 0x38
1409ca2f0: 5b pop rbx
1409ca2f1: 5f pop rdi
1409ca2f2: 5e pop rsi
1409ca2f3: 415e pop r14
1409ca2f5: 415f pop r15
1409ca2f7: 5d pop rbp
1409ca2f8: c3 ret 
1409ca2f9: 0f1f8000000000 nop dword ptr [rax]
1409ca300: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409ca305: 55 push rbp
1409ca306: 4157 push r15
1409ca308: 4156 push r14
1409ca30a: 56 push rsi
1409ca30b: 57 push rdi
1409ca30c: 53 push rbx
1409ca30d: 4883ec38 sub rsp, 0x38
1409ca311: 488daa80000000 lea rbp, [rdx + 0x80]
1409ca318: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1409ca31d: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca324: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409ca32b: e89092ecff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409ca330: c685b701000000 mov byte ptr [rbp + 0x1b7], 0
1409ca337: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1409ca33c: 4883c438 add rsp, 0x38
1409ca340: 5b pop rbx
1409ca341: 5f pop rdi
1409ca342: 5e pop rsi
1409ca343: 415e pop r14
1409ca345: 415f pop r15
1409ca347: 5d pop rbp
1409ca348: c3 ret 

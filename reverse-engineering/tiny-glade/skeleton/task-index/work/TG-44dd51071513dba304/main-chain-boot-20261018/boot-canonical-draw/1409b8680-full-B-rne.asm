1409b8680: 55 push rbp
1409b8681: 4157 push r15
1409b8683: 4156 push r14
1409b8685: 4154 push r12
1409b8687: 56 push rsi
1409b8688: 57 push rdi
1409b8689: 53 push rbx
1409b868a: 4881ec40030000 sub rsp, 0x340
1409b8691: 488dac2480000000 lea rbp, [rsp + 0x80]
1409b8699: 48c785b8020000feffffff mov qword ptr [rbp + 0x2b8], 0xfffffffffffffffe
1409b86a4: 4c89c6 mov rsi, r8
1409b86a7: 4889d7 mov rdi, rdx
1409b86aa: 4889ca mov rdx, rcx
1409b86ad: 488d4db0 lea rcx, [rbp - 0x50]
1409b86b1: 41b8f8000000 mov r8d, 0xf8
1409b86b7: e844b5f601 call 0x142923c00 ; 
1409b86bc: 4889bda8000000 mov qword ptr [rbp + 0xa8], rdi
1409b86c3: 4889b5b0000000 mov qword ptr [rbp + 0xb0], rsi
1409b86ca: 488b4708 mov rax, qword ptr [rdi + 8]
1409b86ce: 89c1 mov ecx, eax
1409b86d0: 483b4e10 cmp rcx, qword ptr [rsi + 0x10]
1409b86d4: 0f8322050000 jae 0x1409b8bfc
1409b86da: 488b5608 mov rdx, qword ptr [rsi + 8]
1409b86de: 4989c0 mov r8, rax
1409b86e1: 49c1e820 shr r8, 0x20
1409b86e5: 488d0c89 lea rcx, [rcx + rcx*4]
1409b86e9: 4439048a cmp dword ptr [rdx + rcx*4], r8d
1409b86ed: 0f8509050000 jne 0x1409b8bfc
1409b86f3: 488d0c8a lea rcx, [rdx + rcx*4]
1409b86f7: 837904ff cmp dword ptr [rcx + 4], -1
1409b86fb: 0f84fb040000 je 0x1409b8bfc
1409b8701: 448b610c mov r12d, dword ptr [rcx + 0xc]
1409b8705: 448b7110 mov r14d, dword ptr [rcx + 0x10]
1409b8709: 488d7e38 lea rdi, [rsi + 0x38]
1409b870d: c685b702000001 mov byte ptr [rbp + 0x2b7], 1
1409b8714: 48bae5c4d1de6e5c3a98 movabs rdx, 0x983a5c6eded1c4e5
1409b871e: 49b89ca7c6dc7b709a92 movabs r8, 0x929a707bdcc6a79c
1409b8728: 4889f9 mov rcx, rdi
1409b872b: e870eafcff call 0x1409871a0 ; bevy_ecs::component::Components::get_id | bevy_ecs::component::Components::get_id | bevy_ecs::component::Components::get_id
1409b8730: 4885c0 test rax, rax
1409b8733: 0f84fe040000 je 0x1409b8c37
1409b8739: 4c39a6b0010000 cmp qword ptr [rsi + 0x1b0], r12
1409b8740: 0f86f1040000 jbe 0x1409b8c37
1409b8746: 4f8d3ce4 lea r15, [r12 + r12*8]
1409b874a: 49c1e703 shl r15, 3
1409b874e: 488b8ea8010000 mov rcx, qword ptr [rsi + 0x1a8]
1409b8755: 4c01f9 add rcx, r15
1409b8758: c685b702000001 mov byte ptr [rbp + 0x2b7], 1
1409b875f: 4589f0 mov r8d, r14d
1409b8762: e8e9833400 call 0x140d00b50 ; bevy_ecs::storage::table::Table::get_component
1409b8767: 4889c3 mov rbx, rax
1409b876a: 4885c0 test rax, rax
1409b876d: 0f84c4040000 je 0x1409b8c37
1409b8773: c685b702000001 mov byte ptr [rbp + 0x2b7], 1
1409b877a: 48baf96fea7d565ff62f movabs rdx, 0x2ff65f567dea6ff9
1409b8784: 49b8db4e83a6d6a326bf movabs r8, 0xbf26a3d6a6834edb
1409b878e: 4889f9 mov rcx, rdi
1409b8791: e80aeafcff call 0x1409871a0 ; bevy_ecs::component::Components::get_id | bevy_ecs::component::Components::get_id | bevy_ecs::component::Components::get_id
1409b8796: 4885c0 test rax, rax
1409b8799: 0f84ad040000 je 0x1409b8c4c
1409b879f: 4c39a6b0010000 cmp qword ptr [rsi + 0x1b0], r12
1409b87a6: 0f86a0040000 jbe 0x1409b8c4c
1409b87ac: 4c03bea8010000 add r15, qword ptr [rsi + 0x1a8]
1409b87b3: c685b702000001 mov byte ptr [rbp + 0x2b7], 1
1409b87ba: 4c89f9 mov rcx, r15
1409b87bd: 4589f0 mov r8d, r14d
1409b87c0: e88b833400 call 0x140d00b50 ; bevy_ecs::storage::table::Table::get_component
1409b87c5: 4989c6 mov r14, rax
1409b87c8: 4885c0 test rax, rax
1409b87cb: 0f847b040000 je 0x1409b8c4c
1409b87d1: 488d8db0010000 lea rcx, [rbp + 0x1b0]
1409b87d8: 488d55b0 lea rdx, [rbp - 0x50]
1409b87dc: 41b8f8000000 mov r8d, 0xf8
1409b87e2: e819b4f601 call 0x142923c00 ; 
1409b87e7: 80bd8e02000008 cmp byte ptr [rbp + 0x28e], 8
1409b87ee: 7425 je 0x1409b8815
1409b87f0: 488d8d68020000 lea rcx, [rbp + 0x268]
1409b87f7: bae8000000 mov edx, 0xe8
1409b87fc: 48039598020000 add rdx, qword ptr [rbp + 0x298]
1409b8803: 49b88acb202f9bd58361 movabs r8, 0x6183d59b2f20cb8a
1409b880d: 4531c9 xor r9d, r9d
1409b8810: e83b30fbff call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
1409b8815: 488d8db8000000 lea rcx, [rbp + 0xb8]
1409b881c: 488d95b0010000 lea rdx, [rbp + 0x1b0]
1409b8823: 41b8f8000000 mov r8d, 0xf8
1409b8829: e8d2b3f601 call 0x142923c00 ; 
1409b882e: 80bd9601000008 cmp byte ptr [rbp + 0x196], 8
1409b8835: 7425 je 0x1409b885c
1409b8837: 488d8d70010000 lea rcx, [rbp + 0x170]
1409b883e: bae8000000 mov edx, 0xe8
1409b8843: 480395a0010000 add rdx, qword ptr [rbp + 0x1a0]
1409b884a: 49b85a4397037834445e movabs r8, 0x5e4434780397435a
1409b8854: 4531c9 xor r9d, r9d
1409b8857: e8f42ffbff call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
1409b885c: 4c8dbdb0010000 lea r15, [rbp + 0x1b0]
1409b8863: 4c8da5b8000000 lea r12, [rbp + 0xb8]
1409b886a: 41b8f8000000 mov r8d, 0xf8
1409b8870: 4c89f9 mov rcx, r15
1409b8873: 4c89e2 mov rdx, r12
1409b8876: e885b3f601 call 0x142923c00 ; 
1409b887b: 48b8318c29481f3cc061 movabs rax, 0x61c03c1f48298c31
1409b8885: 488985a8020000 mov qword ptr [rbp + 0x2a8], rax
1409b888c: c785b800000002000000 mov dword ptr [rbp + 0xb8], 2
1409b8896: 4c89642420 mov qword ptr [rsp + 0x20], r12
1409b889b: 488d8da8020000 lea rcx, [rbp + 0x2a8]
1409b88a2: 49b88f761e6f7911193d movabs r8, 0x3d1911796f1e768f
1409b88ac: 4c89fa mov rdx, r15
1409b88af: 41b11d mov r9b, 0x1d
1409b88b2: e859fdf4ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409b88b7: 4c8dbdb8000000 lea r15, [rbp + 0xb8]
1409b88be: 4c8da5b0010000 lea r12, [rbp + 0x1b0]
1409b88c5: 41b8f8000000 mov r8d, 0xf8
1409b88cb: 4c89f9 mov rcx, r15
1409b88ce: 4c89e2 mov rdx, r12
1409b88d1: e82ab3f601 call 0x142923c00 ; 
1409b88d6: 48b89081ccb80d649b1c movabs rax, 0x1c9b640db8cc8190
1409b88e0: 488985a8020000 mov qword ptr [rbp + 0x2a8], rax
1409b88e7: c785b001000002000000 mov dword ptr [rbp + 0x1b0], 2
1409b88f1: 4c89642420 mov qword ptr [rsp + 0x20], r12
1409b88f6: 488d8da8020000 lea rcx, [rbp + 0x2a8]
1409b88fd: 49b88c7c0d309c5f78e1 movabs r8, 0xe1785f9c300d7c8c
1409b8907: 4c89fa mov rdx, r15
1409b890a: 41b11d mov r9b, 0x1d
1409b890d: e8fefcf4ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409b8912: 4c8dbdb0010000 lea r15, [rbp + 0x1b0]
1409b8919: 4c8da5b8000000 lea r12, [rbp + 0xb8]
1409b8920: 41b8f8000000 mov r8d, 0xf8
1409b8926: 4c89f9 mov rcx, r15
1409b8929: 4c89e2 mov rdx, r12
1409b892c: e8cfb2f601 call 0x142923c00 ; 
1409b8931: 48b8456b7d58c9b6b493 movabs rax, 0x93b4b6c9587d6b45
1409b893b: 488985a8020000 mov qword ptr [rbp + 0x2a8], rax
1409b8942: c785b800000002000000 mov dword ptr [rbp + 0xb8], 2
1409b894c: 4c89642420 mov qword ptr [rsp + 0x20], r12
1409b8951: 488d8da8020000 lea rcx, [rbp + 0x2a8]
1409b8958: 49b82e7b642d2ecb0736 movabs r8, 0x3607cb2e2d647b2e
1409b8962: 4c89fa mov rdx, r15
1409b8965: 41b11d mov r9b, 0x1d
1409b8968: e8a3fcf4ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409b896d: 4c8dbdb8000000 lea r15, [rbp + 0xb8]
1409b8974: 4c8da5b0010000 lea r12, [rbp + 0x1b0]
1409b897b: 41b8f8000000 mov r8d, 0xf8
1409b8981: 4c89f9 mov rcx, r15
1409b8984: 4c89e2 mov rdx, r12
1409b8987: e874b2f601 call 0x142923c00 ; 
1409b898c: 48b87374fe6b60e1f092 movabs rax, 0x92f0e1606bfe7473
1409b8996: 488985a8020000 mov qword ptr [rbp + 0x2a8], rax
1409b899d: c785b001000002000000 mov dword ptr [rbp + 0x1b0], 2
1409b89a7: 4c89642420 mov qword ptr [rsp + 0x20], r12
1409b89ac: 488d8da8020000 lea rcx, [rbp + 0x2a8]
1409b89b3: 49b85088ef348f3e45a5 movabs r8, 0xa5453e8f34ef8850
1409b89bd: 4c89fa mov rdx, r15
1409b89c0: 41b11d mov r9b, 0x1d
1409b89c3: e848fcf4ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409b89c8: 488d8db0010000 lea rcx, [rbp + 0x1b0]
1409b89cf: 488d95b8000000 lea rdx, [rbp + 0xb8]
1409b89d6: 41b8f8000000 mov r8d, 0xf8
1409b89dc: e81fb2f601 call 0x142923c00 ; 
1409b89e1: 80bd8e02000008 cmp byte ptr [rbp + 0x28e], 8
1409b89e8: 7425 je 0x1409b8a0f
1409b89ea: 448b0b mov r9d, dword ptr [rbx]
1409b89ed: 488d8d68020000 lea rcx, [rbp + 0x268]
1409b89f4: bae8000000 mov edx, 0xe8
1409b89f9: 48039598020000 add rdx, qword ptr [rbp + 0x298]
1409b8a00: 49b82cc0ecd318d1fa2c movabs r8, 0x2cfad118d3ecc02c
1409b8a0a: e8612bfbff call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
1409b8a0f: 488d8db8000000 lea rcx, [rbp + 0xb8]
1409b8a16: 488d95b0010000 lea rdx, [rbp + 0x1b0]
1409b8a1d: 41b8f8000000 mov r8d, 0xf8
1409b8a23: e8d8b1f601 call 0x142923c00 ; 
1409b8a28: 80bd9601000008 cmp byte ptr [rbp + 0x196], 8
1409b8a2f: 7425 je 0x1409b8a56
1409b8a31: 458b0e mov r9d, dword ptr [r14]
1409b8a34: 488d8d70010000 lea rcx, [rbp + 0x170]
1409b8a3b: bae8000000 mov edx, 0xe8
1409b8a40: 480395a0010000 add rdx, qword ptr [rbp + 0x1a0]
1409b8a47: 49b8a09a1de67fc498bf movabs r8, 0xbf98c47fe61d9aa0
1409b8a51: e81a2bfbff call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
1409b8a56: 488d9db0010000 lea rbx, [rbp + 0x1b0]
1409b8a5d: 4c8db5b8000000 lea r14, [rbp + 0xb8]
1409b8a64: 41b8f8000000 mov r8d, 0xf8
1409b8a6a: 4889d9 mov rcx, rbx
1409b8a6d: 4c89f2 mov rdx, r14
1409b8a70: e88bb1f601 call 0x142923c00 ; 
1409b8a75: 48b88ca5859971ba2fb7 movabs rax, 0xb72fba719985a58c
1409b8a7f: 488985a8020000 mov qword ptr [rbp + 0x2a8], rax
1409b8a86: c785b800000002000000 mov dword ptr [rbp + 0xb8], 2
1409b8a90: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409b8a95: 488d8da8020000 lea rcx, [rbp + 0x2a8]
1409b8a9c: 49b88c55a3d4df32b683 movabs r8, 0x83b632dfd4a3558c
1409b8aa6: 4889da mov rdx, rbx
1409b8aa9: 41b11d mov r9b, 0x1d
1409b8aac: e85ffbf4ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409b8ab1: 488d8db8000000 lea rcx, [rbp + 0xb8]
1409b8ab8: 41b8f8000000 mov r8d, 0xf8
1409b8abe: 4889da mov rdx, rbx
1409b8ac1: e83ab1f601 call 0x142923c00 ; 
1409b8ac6: c685b602000001 mov byte ptr [rbp + 0x2b6], 1
1409b8acd: 48ba9b71d8c6fc0d4374 movabs rdx, 0x74430dfcc6d8719b
1409b8ad7: 49b89c5064e4f79155ca movabs r8, 0xca5591f7e464509c
1409b8ae1: 4889f9 mov rcx, rdi
1409b8ae4: e8074c72ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
1409b8ae9: 4885c0 test rax, rax
1409b8aec: 0f84f5000000 je 0x1409b8be7
1409b8af2: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1409b8af9: 0f86e8000000 jbe 0x1409b8be7
1409b8aff: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1409b8b06: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409b8b0a: 4885c9 test rcx, rcx
1409b8b0d: 0f84d4000000 je 0x1409b8be7
1409b8b13: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
1409b8b1a: 48f7d1 not rcx
1409b8b1d: 488d0c49 lea rcx, [rcx + rcx*2]
1409b8b21: 48c1e105 shl rcx, 5
1409b8b25: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409b8b2b: 0f84b6000000 je 0x1409b8be7
1409b8b31: 4801c8 add rax, rcx
1409b8b34: 488b4010 mov rax, qword ptr [rax + 0x10]
1409b8b38: 488b4808 mov rcx, qword ptr [rax + 8]
1409b8b3c: 488b5010 mov rdx, qword ptr [rax + 0x10]
1409b8b40: 4531c0 xor r8d, r8d
1409b8b43: 48b8a464a932bc7523e5 movabs rax, 0xe52375bc32a964a4
1409b8b4d: 49b9d250c41ed6d4584c movabs r9, 0x4c58d4d61ec450d2
1409b8b57: 660f1f840000000000 nop word ptr [rax + rax]
1409b8b60: 4c39c2 cmp rdx, r8
1409b8b63: 740e je 0x1409b8b73
1409b8b65: 42803c0101 cmp byte ptr [rcx + r8], 1
1409b8b6a: 4d8d4001 lea r8, [r8 + 1]
1409b8b6e: 77f0 ja 0x1409b8b60
1409b8b70: 4c89c8 mov rax, r9
1409b8b73: 488985a8020000 mov qword ptr [rbp + 0x2a8], rax
1409b8b7a: c785b001000002000000 mov dword ptr [rbp + 0x1b0], 2
1409b8b84: 48895c2420 mov qword ptr [rsp + 0x20], rbx
1409b8b89: 488d8da8020000 lea rcx, [rbp + 0x2a8]
1409b8b90: 488d95b8000000 lea rdx, [rbp + 0xb8]
1409b8b97: 49b81388bf286a6484ce movabs r8, 0xce84646a28bf8813
1409b8ba1: 41b11d mov r9b, 0x1d
1409b8ba4: e867faf4ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409b8ba9: 488db5b0010000 lea rsi, [rbp + 0x1b0]
1409b8bb0: 488d95b8000000 lea rdx, [rbp + 0xb8]
1409b8bb7: 41b8f8000000 mov r8d, 0xf8
1409b8bbd: 4889f1 mov rcx, rsi
1409b8bc0: e83bb0f601 call 0x142923c00 ; 
1409b8bc5: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8bcc: 4889f1 mov rcx, rsi
1409b8bcf: e86ce3f4ff call 0x140906f40 ; country_core::render::pass_builder::draw_builder::DrawBuilder::draw
1409b8bd4: 90 nop 
1409b8bd5: 4881c440030000 add rsp, 0x340
1409b8bdc: 5b pop rbx
1409b8bdd: 5f pop rdi
1409b8bde: 5e pop rsi
1409b8bdf: 415c pop r12
1409b8be1: 415e pop r14
1409b8be3: 415f pop r15
1409b8be5: 5d pop rbp
1409b8be6: c3 ret 
1409b8be7: c685b602000001 mov byte ptr [rbp + 0x2b6], 1
1409b8bee: 488d0de3631002 lea rcx, [rip + 0x21063e3]
1409b8bf5: e81607f201 call 0x1428d9310 ; core::option::unwrap_failed
1409b8bfa: eb63 jmp 0x1409b8c5f
1409b8bfc: 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
1409b8c03: c685b702000001 mov byte ptr [rbp + 0x2b7], 1
1409b8c0a: 488d057f631002 lea rax, [rip + 0x210637f]
1409b8c11: 4889442420 mov qword ptr [rsp + 0x20], rax
1409b8c16: 488d0dab571002 lea rcx, [rip + 0x21057ab] ; @142abe3c8 'called `Result::unwrap()` on an `Err` valuecountry_core::systems::render_loop::draw_passes::draw'
1409b8c1d: 4c8d0d84571002 lea r9, [rip + 0x2105784]
1409b8c24: 4c8d85b0010000 lea r8, [rbp + 0x1b0]
1409b8c2b: ba2b000000 mov edx, 0x2b
1409b8c30: e82b0bf201 call 0x1428d9760 ; core::result::unwrap_failed
1409b8c35: eb28 jmp 0x1409b8c5f
1409b8c37: c685b702000001 mov byte ptr [rbp + 0x2b7], 1
1409b8c3e: 488d0d63631002 lea rcx, [rip + 0x2106363]
1409b8c45: e8c606f201 call 0x1428d9310 ; core::option::unwrap_failed
1409b8c4a: eb13 jmp 0x1409b8c5f
1409b8c4c: c685b702000001 mov byte ptr [rbp + 0x2b7], 1
1409b8c53: 488d0d66631002 lea rcx, [rip + 0x2106366]
1409b8c5a: e8b106f201 call 0x1428d9310 ; core::option::unwrap_failed
1409b8c5f: 0f0b ud2 
1409b8c61: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409b8c70: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409b8c75: 55 push rbp
1409b8c76: 4157 push r15
1409b8c78: 4156 push r14
1409b8c7a: 4154 push r12
1409b8c7c: 56 push rsi
1409b8c7d: 57 push rdi
1409b8c7e: 53 push rbx
1409b8c7f: 4883ec30 sub rsp, 0x30
1409b8c83: 488daa80000000 lea rbp, [rdx + 0x80]
1409b8c8a: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8c91: 488d8db0010000 lea rcx, [rbp + 0x1b0]
1409b8c98: e823a9edff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409b8c9d: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8ca4: 4883c430 add rsp, 0x30
1409b8ca8: 5b pop rbx
1409b8ca9: 5f pop rdi
1409b8caa: 5e pop rsi
1409b8cab: 415c pop r12
1409b8cad: 415e pop r14
1409b8caf: 415f pop r15
1409b8cb1: 5d pop rbp
1409b8cb2: c3 ret 
1409b8cb3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409b8cc0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409b8cc5: 55 push rbp
1409b8cc6: 4157 push r15
1409b8cc8: 4156 push r14
1409b8cca: 4154 push r12
1409b8ccc: 56 push rsi
1409b8ccd: 57 push rdi
1409b8cce: 53 push rbx
1409b8ccf: 4883ec30 sub rsp, 0x30
1409b8cd3: 488daa80000000 lea rbp, [rdx + 0x80]
1409b8cda: 80bdb602000000 cmp byte ptr [rbp + 0x2b6], 0
1409b8ce1: 7422 je 0x1409b8d05
1409b8ce3: 488d8db8000000 lea rcx, [rbp + 0xb8]
1409b8cea: e8d1a8edff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409b8cef: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8cf6: 4883c430 add rsp, 0x30
1409b8cfa: 5b pop rbx
1409b8cfb: 5f pop rdi
1409b8cfc: 5e pop rsi
1409b8cfd: 415c pop r12
1409b8cff: 415e pop r14
1409b8d01: 415f pop r15
1409b8d03: 5d pop rbp
1409b8d04: c3 ret 
1409b8d05: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8d0c: 4883c430 add rsp, 0x30
1409b8d10: 5b pop rbx
1409b8d11: 5f pop rdi
1409b8d12: 5e pop rsi
1409b8d13: 415c pop r12
1409b8d15: 415e pop r14
1409b8d17: 415f pop r15
1409b8d19: 5d pop rbp
1409b8d1a: c3 ret 
1409b8d1b: 0f1f440000 nop dword ptr [rax + rax]
1409b8d20: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409b8d25: 55 push rbp
1409b8d26: 4157 push r15
1409b8d28: 4156 push r14
1409b8d2a: 4154 push r12
1409b8d2c: 56 push rsi
1409b8d2d: 57 push rdi
1409b8d2e: 53 push rbx
1409b8d2f: 4883ec30 sub rsp, 0x30
1409b8d33: 488daa80000000 lea rbp, [rdx + 0x80]
1409b8d3a: 80bdb702000000 cmp byte ptr [rbp + 0x2b7], 0
1409b8d41: 750f jne 0x1409b8d52
1409b8d43: 4883c430 add rsp, 0x30
1409b8d47: 5b pop rbx
1409b8d48: 5f pop rdi
1409b8d49: 5e pop rsi
1409b8d4a: 415c pop r12
1409b8d4c: 415e pop r14
1409b8d4e: 415f pop r15
1409b8d50: 5d pop rbp
1409b8d51: c3 ret 
1409b8d52: 488d4db0 lea rcx, [rbp - 0x50]
1409b8d56: e865a8edff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409b8d5b: ebe6 jmp 0x1409b8d43
1409b8d5d: 0f1f00 nop dword ptr [rax]
1409b8d60: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409b8d65: 55 push rbp
1409b8d66: 4157 push r15
1409b8d68: 4156 push r14
1409b8d6a: 4154 push r12
1409b8d6c: 56 push rsi
1409b8d6d: 57 push rdi
1409b8d6e: 53 push rbx
1409b8d6f: 4883ec30 sub rsp, 0x30
1409b8d73: 488daa80000000 lea rbp, [rdx + 0x80]
1409b8d7a: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8d81: 488d8db8000000 lea rcx, [rbp + 0xb8]
1409b8d88: e833a8edff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409b8d8d: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8d94: 4883c430 add rsp, 0x30
1409b8d98: 5b pop rbx
1409b8d99: 5f pop rdi
1409b8d9a: 5e pop rsi
1409b8d9b: 415c pop r12
1409b8d9d: 415e pop r14
1409b8d9f: 415f pop r15
1409b8da1: 5d pop rbp
1409b8da2: c3 ret 
1409b8da3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409b8db0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409b8db5: 55 push rbp
1409b8db6: 4157 push r15
1409b8db8: 4156 push r14
1409b8dba: 4154 push r12
1409b8dbc: 56 push rsi
1409b8dbd: 57 push rdi
1409b8dbe: 53 push rbx
1409b8dbf: 4883ec30 sub rsp, 0x30
1409b8dc3: 488daa80000000 lea rbp, [rdx + 0x80]
1409b8dca: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8dd1: 488d8db0010000 lea rcx, [rbp + 0x1b0]
1409b8dd8: e8e3a7edff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409b8ddd: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8de4: 4883c430 add rsp, 0x30
1409b8de8: 5b pop rbx
1409b8de9: 5f pop rdi
1409b8dea: 5e pop rsi
1409b8deb: 415c pop r12
1409b8ded: 415e pop r14
1409b8def: 415f pop r15
1409b8df1: 5d pop rbp
1409b8df2: c3 ret 
1409b8df3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409b8e00: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409b8e05: 55 push rbp
1409b8e06: 4157 push r15
1409b8e08: 4156 push r14
1409b8e0a: 4154 push r12
1409b8e0c: 56 push rsi
1409b8e0d: 57 push rdi
1409b8e0e: 53 push rbx
1409b8e0f: 4883ec30 sub rsp, 0x30
1409b8e13: 488daa80000000 lea rbp, [rdx + 0x80]
1409b8e1a: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8e21: 488d8db8000000 lea rcx, [rbp + 0xb8]
1409b8e28: e893a7edff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409b8e2d: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8e34: 4883c430 add rsp, 0x30
1409b8e38: 5b pop rbx
1409b8e39: 5f pop rdi
1409b8e3a: 5e pop rsi
1409b8e3b: 415c pop r12
1409b8e3d: 415e pop r14
1409b8e3f: 415f pop r15
1409b8e41: 5d pop rbp
1409b8e42: c3 ret 
1409b8e43: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409b8e50: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409b8e55: 55 push rbp
1409b8e56: 4157 push r15
1409b8e58: 4156 push r14
1409b8e5a: 4154 push r12
1409b8e5c: 56 push rsi
1409b8e5d: 57 push rdi
1409b8e5e: 53 push rbx
1409b8e5f: 4883ec30 sub rsp, 0x30
1409b8e63: 488daa80000000 lea rbp, [rdx + 0x80]
1409b8e6a: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8e71: 488d8db0010000 lea rcx, [rbp + 0x1b0]
1409b8e78: e843a7edff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409b8e7d: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8e84: 4883c430 add rsp, 0x30
1409b8e88: 5b pop rbx
1409b8e89: 5f pop rdi
1409b8e8a: 5e pop rsi
1409b8e8b: 415c pop r12
1409b8e8d: 415e pop r14
1409b8e8f: 415f pop r15
1409b8e91: 5d pop rbp
1409b8e92: c3 ret 
1409b8e93: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409b8ea0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409b8ea5: 55 push rbp
1409b8ea6: 4157 push r15
1409b8ea8: 4156 push r14
1409b8eaa: 4154 push r12
1409b8eac: 56 push rsi
1409b8ead: 57 push rdi
1409b8eae: 53 push rbx
1409b8eaf: 4883ec30 sub rsp, 0x30
1409b8eb3: 488daa80000000 lea rbp, [rdx + 0x80]
1409b8eba: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8ec1: 488d8db8000000 lea rcx, [rbp + 0xb8]
1409b8ec8: e8f3a6edff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409b8ecd: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8ed4: 4883c430 add rsp, 0x30
1409b8ed8: 5b pop rbx
1409b8ed9: 5f pop rdi
1409b8eda: 5e pop rsi
1409b8edb: 415c pop r12
1409b8edd: 415e pop r14
1409b8edf: 415f pop r15
1409b8ee1: 5d pop rbp
1409b8ee2: c3 ret 
1409b8ee3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409b8ef0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409b8ef5: 55 push rbp
1409b8ef6: 4157 push r15
1409b8ef8: 4156 push r14
1409b8efa: 4154 push r12
1409b8efc: 56 push rsi
1409b8efd: 57 push rdi
1409b8efe: 53 push rbx
1409b8eff: 4883ec30 sub rsp, 0x30
1409b8f03: 488daa80000000 lea rbp, [rdx + 0x80]
1409b8f0a: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8f11: 488d8db0010000 lea rcx, [rbp + 0x1b0]
1409b8f18: e8a3a6edff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409b8f1d: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8f24: 4883c430 add rsp, 0x30
1409b8f28: 5b pop rbx
1409b8f29: 5f pop rdi
1409b8f2a: 5e pop rsi
1409b8f2b: 415c pop r12
1409b8f2d: 415e pop r14
1409b8f2f: 415f pop r15
1409b8f31: 5d pop rbp
1409b8f32: c3 ret 
1409b8f33: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409b8f40: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409b8f45: 55 push rbp
1409b8f46: 4157 push r15
1409b8f48: 4156 push r14
1409b8f4a: 4154 push r12
1409b8f4c: 56 push rsi
1409b8f4d: 57 push rdi
1409b8f4e: 53 push rbx
1409b8f4f: 4883ec30 sub rsp, 0x30
1409b8f53: 488daa80000000 lea rbp, [rdx + 0x80]
1409b8f5a: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8f61: 488d8db8000000 lea rcx, [rbp + 0xb8]
1409b8f68: e853a6edff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409b8f6d: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8f74: 4883c430 add rsp, 0x30
1409b8f78: 5b pop rbx
1409b8f79: 5f pop rdi
1409b8f7a: 5e pop rsi
1409b8f7b: 415c pop r12
1409b8f7d: 415e pop r14
1409b8f7f: 415f pop r15
1409b8f81: 5d pop rbp
1409b8f82: c3 ret 
1409b8f83: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409b8f90: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409b8f95: 55 push rbp
1409b8f96: 4157 push r15
1409b8f98: 4156 push r14
1409b8f9a: 4154 push r12
1409b8f9c: 56 push rsi
1409b8f9d: 57 push rdi
1409b8f9e: 53 push rbx
1409b8f9f: 4883ec30 sub rsp, 0x30
1409b8fa3: 488daa80000000 lea rbp, [rdx + 0x80]
1409b8faa: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8fb1: 488d8db0010000 lea rcx, [rbp + 0x1b0]
1409b8fb8: e803a6edff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409b8fbd: c685b702000000 mov byte ptr [rbp + 0x2b7], 0
1409b8fc4: 4883c430 add rsp, 0x30
1409b8fc8: 5b pop rbx
1409b8fc9: 5f pop rdi
1409b8fca: 5e pop rsi
1409b8fcb: 415c pop r12
1409b8fcd: 415e pop r14
1409b8fcf: 415f pop r15
1409b8fd1: 5d pop rbp
1409b8fd2: c3 ret 
1409b8fd3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409b8fe0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409b8fe5: 55 push rbp
1409b8fe6: 4157 push r15
1409b8fe8: 4156 push r14
1409b8fea: 4154 push r12
1409b8fec: 56 push rsi
1409b8fed: 57 push rdi
1409b8fee: 53 push rbx
1409b8fef: 4883ec30 sub rsp, 0x30
1409b8ff3: 488daa80000000 lea rbp, [rdx + 0x80]
1409b8ffa: c685b602000000 mov byte ptr [rbp + 0x2b6], 0
1409b9001: 488d8db8000000 lea rcx, [rbp + 0xb8]
1409b9008: e8b3a5edff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409b900d: c685b602000000 mov byte ptr [rbp + 0x2b6], 0
1409b9014: 4883c430 add rsp, 0x30
1409b9018: 5b pop rbx
1409b9019: 5f pop rdi
1409b901a: 5e pop rsi
1409b901b: 415c pop r12
1409b901d: 415e pop r14
1409b901f: 415f pop r15
1409b9021: 5d pop rbp
1409b9022: c3 ret 

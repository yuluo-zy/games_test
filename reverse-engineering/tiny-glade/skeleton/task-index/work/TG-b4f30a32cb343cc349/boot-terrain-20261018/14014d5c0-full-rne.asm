14014d5c0: push rsi
14014d5c1: sub rsp, 0x50
14014d5c5: test rdx, rdx
14014d5c8: je 0x14014d678
14014d5ce: mov rsi, rdx
14014d5d1: lea rcx, [rdx + 0x38]
14014d5d5: movabs rdx, 0x1e9ac9b6648cf0a6
14014d5df: movabs r8, 0xe6222c4af0fcfd90
14014d5e9: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14014d5ee: test rax, rax
14014d5f1: je 0x14014d65b
14014d5f3: cmp qword ptr [rsi + 0x218], rdx
14014d5fa: jbe 0x14014d65b
14014d5fc: mov rax, qword ptr [rsi + 0x210]
14014d603: mov rax, qword ptr [rax + rdx*8]
14014d607: test rax, rax
14014d60a: je 0x14014d65b
14014d60c: mov rcx, qword ptr [rsi + 0x1e0]
14014d613: not rax
14014d616: lea rax, [rax + rax*2]
14014d61a: shl rax, 5
14014d61e: cmp qword ptr [rcx + rax + 0x20], 0
14014d624: je 0x14014d65b
14014d626: add rcx, rax
14014d629: xor edx, edx
14014d62b: call 0x140cff710 ; bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
14014d630: movups xmm0, xmmword ptr [rax]
14014d633: movups xmm1, xmmword ptr [rax + 0x10]
14014d637: movups xmm2, xmmword ptr [rax + 0x20]
14014d63b: movaps xmmword ptr [rsp + 0x40], xmm2
14014d640: movaps xmmword ptr [rsp + 0x20], xmm0
14014d645: movaps xmmword ptr [rsp + 0x30], xmm1
14014d64a: cmp byte ptr [rsp + 0x46], 5
14014d64f: je 0x14014d65b
14014d651: lea rcx, [rsp + 0x20]
14014d656: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
14014d65b: mov rcx, rsi
14014d65e: call 0x140cf81b0 ; bevy_ecs::world::World::flush_entities
14014d663: mov rcx, rsi
14014d666: call 0x140cf8250 ; bevy_ecs::world::World::flush_components
14014d66b: mov rcx, rsi
14014d66e: add rsp, 0x50
14014d672: pop rsi
14014d673: jmp 0x140cf8200 ; bevy_ecs::world::World::flush_commands
14014d678: nop 
14014d679: add rsp, 0x50
14014d67d: pop rsi
14014d67e: ret 

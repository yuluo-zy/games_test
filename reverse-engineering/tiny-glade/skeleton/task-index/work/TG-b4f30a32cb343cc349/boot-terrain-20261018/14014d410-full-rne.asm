14014d410: push rsi
14014d411: sub rsp, 0x50
14014d415: add qword ptr [r8], 0x30
14014d419: test rdx, rdx
14014d41c: je 0x14014d462
14014d41e: mov rsi, rdx
14014d421: movups xmm0, xmmword ptr [rcx]
14014d424: movups xmm1, xmmword ptr [rcx + 0x10]
14014d428: movups xmm2, xmmword ptr [rcx + 0x20]
14014d42c: movaps xmmword ptr [rsp + 0x40], xmm2
14014d431: movaps xmmword ptr [rsp + 0x30], xmm1
14014d436: movaps xmmword ptr [rsp + 0x20], xmm0
14014d43b: lea rcx, [rsp + 0x20]
14014d440: call 0x1405274d0 ; bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>,tuple$<> >
14014d445: mov rcx, rsi
14014d448: call 0x140cf81b0 ; bevy_ecs::world::World::flush_entities
14014d44d: mov rcx, rsi
14014d450: call 0x140cf8250 ; bevy_ecs::world::World::flush_components
14014d455: mov rcx, rsi
14014d458: add rsp, 0x50
14014d45c: pop rsi
14014d45d: jmp 0x140cf8200 ; bevy_ecs::world::World::flush_commands
14014d462: movups xmm0, xmmword ptr [rcx]
14014d465: movups xmm1, xmmword ptr [rcx + 0x10]
14014d469: movups xmm2, xmmword ptr [rcx + 0x20]
14014d46d: movaps xmmword ptr [rsp + 0x40], xmm2
14014d472: movaps xmmword ptr [rsp + 0x30], xmm1
14014d477: movaps xmmword ptr [rsp + 0x20], xmm0
14014d47c: lea rcx, [rsp + 0x20]
14014d481: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
14014d486: nop 
14014d487: add rsp, 0x50
14014d48b: pop rsi
14014d48c: ret 

1412d8b00: push rsi
1412d8b01: sub rsp, 0x30
1412d8b05: add qword ptr [r8], 0x10
1412d8b09: test rdx, rdx
1412d8b0c: je 0x1412d8b40
1412d8b0e: mov rsi, rdx
1412d8b11: movups xmm0, xmmword ptr [rcx]
1412d8b14: movaps xmmword ptr [rsp + 0x20], xmm0
1412d8b19: lea rcx, [rsp + 0x20]
1412d8b1e: call 0x142058f60 ; bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::resources::rocky_terrain::TerrainEntities>,tuple$<> >
1412d8b23: mov rcx, rsi
1412d8b26: call 0x140cf81b0 ; bevy_ecs::world::World::flush_entities
1412d8b2b: mov rcx, rsi
1412d8b2e: call 0x140cf8250 ; bevy_ecs::world::World::flush_components
1412d8b33: mov rcx, rsi
1412d8b36: add rsp, 0x30
1412d8b3a: pop rsi
1412d8b3b: jmp 0x140cf8200
1412d8b40: nop 
1412d8b41: add rsp, 0x30
1412d8b45: pop rsi
1412d8b46: ret 

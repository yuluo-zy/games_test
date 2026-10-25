1407ec66b: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ec672: 8a58fc mov bl, byte ptr [rax - 4]
1407ec675: 488b95e0000000 mov rdx, qword ptr [rbp + 0xe0]
1407ec67c: 0fb68d80020000 movzx ecx, byte ptr [rbp + 0x280]
1407ec683: e8c86b0400 call 0x140833250 ; country_core::resources::walls::decorator::DecoratorRank::new | country_core::systems::collision::world::impl$61::from | country_core::render::nani::mesh_atlas::MeshAtlasPart::new | country_core::LazyTile::from_index | c
1407ec688: 488bb5e8000000 mov rsi, qword ptr [rbp + 0xe8]
1407ec68f: 89d9 mov ecx, ebx
1407ec691: 89c2 mov edx, eax
1407ec693: e8f8953300 call 0x140b25c90 ; country_core::resources::walls::decorator_subtype::DecoratorLibKey::new | rhapsody::rg::recorder::PassResourceAccessType::new
1407ec698: 888598010000 mov byte ptr [rbp + 0x198], al
1407ec69e: 889599010000 mov byte ptr [rbp + 0x199], dl
1407ec6a4: 4889f1 mov rcx, rsi
1407ec6a7: 488d9598010000 lea rdx, [rbp + 0x198]
1407ec6ae: e8dd08a700 call 0x14125cf90 ; std::collections::hash::map::HashMap::get<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,en
1407ec6b3: 4885c0 test rax, rax
1407ec6b6: 0f8446260000 je 0x1407eed02
1407ec6bc: 0f104020 movups xmm0, xmmword ptr [rax + 0x20]
1407ec6c0: 0f104830 movups xmm1, xmmword ptr [rax + 0x30]
1407ec6c4: 0f298dd0000000 movaps xmmword ptr [rbp + 0xd0], xmm1
1407ec6cb: 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0

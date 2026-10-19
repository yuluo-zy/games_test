1415f5310: 55 push rbp
1415f5311: 56 push rsi
1415f5312: 57 push rdi
1415f5313: 53 push rbx
1415f5314: 4881ec18050000 sub rsp, 0x518
1415f531b: 488dac2480000000 lea rbp, [rsp + 0x80]
1415f5323: 48c78590040000feffffff mov qword ptr [rbp + 0x490], 0xfffffffffffffffe
1415f532e: 4889ce mov rsi, rcx
1415f5331: 0fb605d1f80302 movzx eax, byte ptr [rip + 0x203f8d1]
1415f5338: b9a0020000 mov ecx, 0x2a0
1415f533d: ba08000000 mov edx, 8
1415f5342: e8c9e801ff call 0x140613c10 ; unknown
1415f5347: 4885c0 test rax, rax
1415f534a: 0f841c060000 je 0x1415f596c
1415f5350: 48898588040000 mov qword ptr [rbp + 0x488], rax
1415f5357: 0f57c0 xorps xmm0, xmm0
1415f535a: 0f1185f0010000 movups xmmword ptr [rbp + 0x1f0], xmm0
1415f5361: 0f118508020000 movups xmmword ptr [rbp + 0x208], xmm0
1415f5368: 0f118520020000 movups xmmword ptr [rbp + 0x220], xmm0
1415f536f: 0f118538020000 movups xmmword ptr [rbp + 0x238], xmm0
1415f5376: 0f118550020000 movups xmmword ptr [rbp + 0x250], xmm0
1415f537d: c7856002000000000000 mov dword ptr [rbp + 0x260], 0
1415f5387: 0f118588020000 movups xmmword ptr [rbp + 0x288], xmm0
1415f538e: 0f1185a0020000 movups xmmword ptr [rbp + 0x2a0], xmm0
1415f5395: 0f1185b8020000 movups xmmword ptr [rbp + 0x2b8], xmm0
1415f539c: 0f1185d0020000 movups xmmword ptr [rbp + 0x2d0], xmm0
1415f53a3: 0f1185e8020000 movups xmmword ptr [rbp + 0x2e8], xmm0
1415f53aa: c785f802000000000000 mov dword ptr [rbp + 0x2f8], 0
1415f53b4: 48c785d001000000000000 mov qword ptr [rbp + 0x1d0], 0
1415f53bf: 48c785d801000008000000 mov qword ptr [rbp + 0x1d8], 8
1415f53ca: 48c785e001000000000000 mov qword ptr [rbp + 0x1e0], 0
1415f53d5: 48c785e801000010000000 mov qword ptr [rbp + 0x1e8], 0x10
1415f53e0: 48c7850002000010000000 mov qword ptr [rbp + 0x200], 0x10
1415f53eb: 48c7851802000010000000 mov qword ptr [rbp + 0x218], 0x10
1415f53f6: 48c7853002000010000000 mov qword ptr [rbp + 0x230], 0x10
1415f5401: 48c7854802000010000000 mov qword ptr [rbp + 0x248], 0x10
1415f540c: 48bb0000000000000080 movabs rbx, 0x8000000000000000
1415f5416: 48899d68020000 mov qword ptr [rbp + 0x268], rbx
1415f541d: 488d05b17d6801 lea rax, [rip + 0x1687db1]
1415f5424: 48898570020000 mov qword ptr [rbp + 0x270], rax
1415f542b: 48c785780200003e000000 mov qword ptr [rbp + 0x278], 0x3e
1415f5436: 48c7858002000010000000 mov qword ptr [rbp + 0x280], 0x10
1415f5441: 48c7859802000010000000 mov qword ptr [rbp + 0x298], 0x10
1415f544c: 48c785b002000010000000 mov qword ptr [rbp + 0x2b0], 0x10
1415f5457: 48c785c802000010000000 mov qword ptr [rbp + 0x2c8], 0x10
1415f5462: 48c785e002000010000000 mov qword ptr [rbp + 0x2e0], 0x10
1415f546d: c7850003000000000000 mov dword ptr [rbp + 0x300], 0
1415f5477: 66c785040300000100 mov word ptr [rbp + 0x304], 1
1415f5480: 48899d00040000 mov qword ptr [rbp + 0x400], rbx
1415f5487: c7858004000000000000 mov dword ptr [rbp + 0x480], 0
1415f5491: 0fb60571f70302 movzx eax, byte ptr [rip + 0x203f771]
1415f5498: b9b8020000 mov ecx, 0x2b8
1415f549d: ba08000000 mov edx, 8
1415f54a2: e869e701ff call 0x140613c10 ; unknown
1415f54a7: 4885c0 test rax, rax
1415f54aa: 0f84cb040000 je 0x1415f597b
1415f54b0: 4889c7 mov rdi, rax
1415f54b3: 488d95d0010000 lea rdx, [rbp + 0x1d0]
1415f54ba: 41b8b8020000 mov r8d, 0x2b8
1415f54c0: 4889c1 mov rcx, rax
1415f54c3: e838e73201 call 0x142923c00 ; unknown
1415f54c8: 4c8d0579716801 lea r8, [rip + 0x1687179]
1415f54cf: 488d8df0000000 lea rcx, [rbp + 0xf0]
1415f54d6: 4889fa mov rdx, rdi
1415f54d9: e832f66eff call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
1415f54de: 0f57c0 xorps xmm0, xmm0
1415f54e1: 0f118518020000 movups xmmword ptr [rbp + 0x218], xmm0
1415f54e8: 0f118530020000 movups xmmword ptr [rbp + 0x230], xmm0
1415f54ef: 0f118548020000 movups xmmword ptr [rbp + 0x248], xmm0
1415f54f6: 0f118560020000 movups xmmword ptr [rbp + 0x260], xmm0
1415f54fd: 0f118578020000 movups xmmword ptr [rbp + 0x278], xmm0
1415f5504: c7858802000000000000 mov dword ptr [rbp + 0x288], 0
1415f550e: 0f1185b0020000 movups xmmword ptr [rbp + 0x2b0], xmm0
1415f5515: 0f1185c8020000 movups xmmword ptr [rbp + 0x2c8], xmm0
1415f551c: 0f1185e0020000 movups xmmword ptr [rbp + 0x2e0], xmm0
1415f5523: 0f1185f8020000 movups xmmword ptr [rbp + 0x2f8], xmm0
1415f552a: 0f118510030000 movups xmmword ptr [rbp + 0x310], xmm0
1415f5531: c7852003000000000000 mov dword ptr [rbp + 0x320], 0
1415f553b: 48c785d001000000000000 mov qword ptr [rbp + 0x1d0], 0
1415f5546: 48c785f801000000000000 mov qword ptr [rbp + 0x1f8], 0
1415f5551: 48c7850002000008000000 mov qword ptr [rbp + 0x200], 8
1415f555c: 48c7850802000000000000 mov qword ptr [rbp + 0x208], 0
1415f5567: 48c7851002000010000000 mov qword ptr [rbp + 0x210], 0x10
1415f5572: 48c7852802000010000000 mov qword ptr [rbp + 0x228], 0x10
1415f557d: 48c7854002000010000000 mov qword ptr [rbp + 0x240], 0x10
1415f5588: 48c7855802000010000000 mov qword ptr [rbp + 0x258], 0x10
1415f5593: 48c7857002000010000000 mov qword ptr [rbp + 0x270], 0x10
1415f559e: 48899d90020000 mov qword ptr [rbp + 0x290], rbx
1415f55a5: 488d05f17b6801 lea rax, [rip + 0x1687bf1]
1415f55ac: 48898598020000 mov qword ptr [rbp + 0x298], rax
1415f55b3: 48c785a002000038000000 mov qword ptr [rbp + 0x2a0], 0x38
1415f55be: 48c785a802000010000000 mov qword ptr [rbp + 0x2a8], 0x10
1415f55c9: 48c785c002000010000000 mov qword ptr [rbp + 0x2c0], 0x10
1415f55d4: 48c785d802000010000000 mov qword ptr [rbp + 0x2d8], 0x10
1415f55df: 48c785f002000010000000 mov qword ptr [rbp + 0x2f0], 0x10
1415f55ea: 48c7850803000010000000 mov qword ptr [rbp + 0x308], 0x10
1415f55f5: c7852803000000000000 mov dword ptr [rbp + 0x328], 0
1415f55ff: 66c7852c0300000100 mov word ptr [rbp + 0x32c], 1
1415f5608: c7853003000000000000 mov dword ptr [rbp + 0x330], 0
1415f5612: 0fb605f0f50302 movzx eax, byte ptr [rip + 0x203f5f0]
1415f5619: b968010000 mov ecx, 0x168
1415f561e: ba08000000 mov edx, 8
1415f5623: e8e8e501ff call 0x140613c10 ; unknown
1415f5628: 4885c0 test rax, rax
1415f562b: 0f845b030000 je 0x1415f598c
1415f5631: 4889c7 mov rdi, rax
1415f5634: 488d95d0010000 lea rdx, [rbp + 0x1d0]
1415f563b: 41b868010000 mov r8d, 0x168
1415f5641: 4889c1 mov rcx, rax
1415f5644: e8b7e53201 call 0x142923c00 ; unknown
1415f5649: 4c8d05b0706801 lea r8, [rip + 0x16870b0]
1415f5650: 488d8d80000000 lea rcx, [rbp + 0x80]
1415f5657: 4889fa mov rdx, rdi
1415f565a: e8b1f46eff call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
1415f565f: 488d4d10 lea rcx, [rbp + 0x10]
1415f5663: e8f863a700 call 0x14206ba60 ; bevy_ecs::schedule::config::impl$4::into_configs<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,
1415f5668: 4c8d05291d6f01 lea r8, [rip + 0x16f1d29]
1415f566f: 488d4da0 lea rcx, [rbp - 0x60]
1415f5673: ba01000000 mov edx, 1
1415f5678: e893f46eff call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
1415f567d: 488d8d60010000 lea rcx, [rbp + 0x160]
1415f5684: e88765a700 call 0x14206bc10 ; bevy_ecs::schedule::config::impl$28::into_configs<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system
1415f5689: 488d8dd0010000 lea rcx, [rbp + 0x1d0]
1415f5690: e80b62a700 call 0x14206b8a0 ; bevy_ecs::schedule::config::impl$4::into_configs<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_c
1415f5695: 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
1415f569c: 488b8588040000 mov rax, qword ptr [rbp + 0x488]
1415f56a3: 0f114060 movups xmmword ptr [rax + 0x60], xmm0
1415f56a7: 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
1415f56ae: 0f114050 movups xmmword ptr [rax + 0x50], xmm0
1415f56b2: 0f108530010000 movups xmm0, xmmword ptr [rbp + 0x130]
1415f56b9: 0f114040 movups xmmword ptr [rax + 0x40], xmm0
1415f56bd: 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
1415f56c4: 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
1415f56cb: 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
1415f56d2: 0f109d20010000 movups xmm3, xmmword ptr [rbp + 0x120]
1415f56d9: 0f115830 movups xmmword ptr [rax + 0x30], xmm3
1415f56dd: 0f115020 movups xmmword ptr [rax + 0x20], xmm2
1415f56e1: 0f114810 movups xmmword ptr [rax + 0x10], xmm1
1415f56e5: 0f1100 movups xmmword ptr [rax], xmm0
1415f56e8: 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
1415f56ef: 0f108d90000000 movups xmm1, xmmword ptr [rbp + 0x90]
1415f56f6: 0f1095a0000000 movups xmm2, xmmword ptr [rbp + 0xa0]
1415f56fd: 0f109db0000000 movups xmm3, xmmword ptr [rbp + 0xb0]
1415f5704: 0f114070 movups xmmword ptr [rax + 0x70], xmm0
1415f5708: 0f118880000000 movups xmmword ptr [rax + 0x80], xmm1
1415f570f: 0f119090000000 movups xmmword ptr [rax + 0x90], xmm2
1415f5716: 0f1198a0000000 movups xmmword ptr [rax + 0xa0], xmm3
1415f571d: 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
1415f5724: 0f1180b0000000 movups xmmword ptr [rax + 0xb0], xmm0
1415f572b: 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
1415f5732: 0f1180c0000000 movups xmmword ptr [rax + 0xc0], xmm0
1415f5739: 0f1085e0000000 movups xmm0, xmmword ptr [rbp + 0xe0]
1415f5740: 0f1180d0000000 movups xmmword ptr [rax + 0xd0], xmm0
1415f5747: 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
1415f574b: 0f118040010000 movups xmmword ptr [rax + 0x140], xmm0
1415f5752: 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
1415f5756: 0f118030010000 movups xmmword ptr [rax + 0x130], xmm0
1415f575d: 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
1415f5761: 0f118020010000 movups xmmword ptr [rax + 0x120], xmm0
1415f5768: 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
1415f576c: 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
1415f5770: 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
1415f5774: 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
1415f5778: 0f119810010000 movups xmmword ptr [rax + 0x110], xmm3
1415f577f: 0f119000010000 movups xmmword ptr [rax + 0x100], xmm2
1415f5786: 0f1188f0000000 movups xmmword ptr [rax + 0xf0], xmm1
1415f578d: 0f1180e0000000 movups xmmword ptr [rax + 0xe0], xmm0
1415f5794: 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
1415f5798: 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
1415f579c: 0f1055c0 movups xmm2, xmmword ptr [rbp - 0x40]
1415f57a0: 0f105dd0 movups xmm3, xmmword ptr [rbp - 0x30]
1415f57a4: 0f118050010000 movups xmmword ptr [rax + 0x150], xmm0
1415f57ab: 0f118860010000 movups xmmword ptr [rax + 0x160], xmm1
1415f57b2: 0f119070010000 movups xmmword ptr [rax + 0x170], xmm2
1415f57b9: 0f119880010000 movups xmmword ptr [rax + 0x180], xmm3
1415f57c0: 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
1415f57c4: 0f118090010000 movups xmmword ptr [rax + 0x190], xmm0
1415f57cb: 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
1415f57cf: 0f1180a0010000 movups xmmword ptr [rax + 0x1a0], xmm0
1415f57d6: 0f104500 movups xmm0, xmmword ptr [rbp]
1415f57da: 0f1180b0010000 movups xmmword ptr [rax + 0x1b0], xmm0
1415f57e1: 0f108560010000 movups xmm0, xmmword ptr [rbp + 0x160]
1415f57e8: 0f108d70010000 movups xmm1, xmmword ptr [rbp + 0x170]
1415f57ef: 0f109580010000 movups xmm2, xmmword ptr [rbp + 0x180]
1415f57f6: 0f109d90010000 movups xmm3, xmmword ptr [rbp + 0x190]
1415f57fd: 0f1180c0010000 movups xmmword ptr [rax + 0x1c0], xmm0
1415f5804: 0f1188d0010000 movups xmmword ptr [rax + 0x1d0], xmm1
1415f580b: 0f1190e0010000 movups xmmword ptr [rax + 0x1e0], xmm2
1415f5812: 0f1198f0010000 movups xmmword ptr [rax + 0x1f0], xmm3
1415f5819: 0f1085a0010000 movups xmm0, xmmword ptr [rbp + 0x1a0]
1415f5820: 0f118000020000 movups xmmword ptr [rax + 0x200], xmm0
1415f5827: 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
1415f582e: 0f118020020000 movups xmmword ptr [rax + 0x220], xmm0
1415f5835: 0f1085b0010000 movups xmm0, xmmword ptr [rbp + 0x1b0]
1415f583c: 0f118010020000 movups xmmword ptr [rax + 0x210], xmm0
1415f5843: 0f108530020000 movups xmm0, xmmword ptr [rbp + 0x230]
1415f584a: 0f118090020000 movups xmmword ptr [rax + 0x290], xmm0
1415f5851: 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
1415f5858: 0f118080020000 movups xmmword ptr [rax + 0x280], xmm0
1415f585f: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
1415f5866: 0f118070020000 movups xmmword ptr [rax + 0x270], xmm0
1415f586d: 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
1415f5874: 0f108de0010000 movups xmm1, xmmword ptr [rbp + 0x1e0]
1415f587b: 0f1095f0010000 movups xmm2, xmmword ptr [rbp + 0x1f0]
1415f5882: 0f109d00020000 movups xmm3, xmmword ptr [rbp + 0x200]
1415f5889: 0f119860020000 movups xmmword ptr [rax + 0x260], xmm3
1415f5890: 0f119050020000 movups xmmword ptr [rax + 0x250], xmm2
1415f5897: 0f118030020000 movups xmmword ptr [rax + 0x230], xmm0
1415f589e: 0f118840020000 movups xmmword ptr [rax + 0x240], xmm1
1415f58a5: 48c785d801000006000000 mov qword ptr [rbp + 0x1d8], 6
1415f58b0: 488985e0010000 mov qword ptr [rbp + 0x1e0], rax
1415f58b7: 48c785e801000006000000 mov qword ptr [rbp + 0x1e8], 6
1415f58c2: 48c785f001000000000000 mov qword ptr [rbp + 0x1f0], 0
1415f58cd: 48c785f801000008000000 mov qword ptr [rbp + 0x1f8], 8
1415f58d8: 0f57c0 xorps xmm0, xmm0
1415f58db: 0f118500020000 movups xmmword ptr [rbp + 0x200], xmm0
1415f58e2: 488b8570010000 mov rax, qword ptr [rbp + 0x170]
1415f58e9: 48898520020000 mov qword ptr [rbp + 0x220], rax
1415f58f0: 0f108560010000 movups xmm0, xmmword ptr [rbp + 0x160]
1415f58f7: 0f118510020000 movups xmmword ptr [rbp + 0x210], xmm0
1415f58fe: 48899dd0010000 mov qword ptr [rbp + 0x1d0], rbx
1415f5905: 488d8d08020000 lea rcx, [rbp + 0x208]
1415f590c: e88fa70a00 call 0x1416a00a0 ; bevy_ecs::schedule::schedule::Chain::set_chained_with_config<bevy_ecs::schedule::auto_insert_apply_deferred::IgnoreDeferred>
1415f5911: 0f108530020000 movups xmm0, xmmword ptr [rbp + 0x230]
1415f5918: 0f114660 movups xmmword ptr [rsi + 0x60], xmm0
1415f591c: 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
1415f5923: 0f114650 movups xmmword ptr [rsi + 0x50], xmm0
1415f5927: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
1415f592e: 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
1415f5932: 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
1415f5939: 0f108de0010000 movups xmm1, xmmword ptr [rbp + 0x1e0]
1415f5940: 0f1095f0010000 movups xmm2, xmmword ptr [rbp + 0x1f0]
1415f5947: 0f109d00020000 movups xmm3, xmmword ptr [rbp + 0x200]
1415f594e: 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
1415f5952: 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
1415f5956: 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
1415f595a: 0f1106 movups xmmword ptr [rsi], xmm0
1415f595d: 4889f0 mov rax, rsi
1415f5960: 4881c418050000 add rsp, 0x518
1415f5967: 5b pop rbx
1415f5968: 5f pop rdi
1415f5969: 5e pop rsi
1415f596a: 5d pop rbp
1415f596b: c3 ret 
1415f596c: b908000000 mov ecx, 8
1415f5971: baa0020000 mov edx, 0x2a0
1415f5976: e885362e01 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
1415f597b: b908000000 mov ecx, 8
1415f5980: bab8020000 mov edx, 0x2b8
1415f5985: e876362e01 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
1415f598a: eb0f jmp 0x1415f599b ; unknown
1415f598c: b908000000 mov ecx, 8
1415f5991: ba68010000 mov edx, 0x168
1415f5996: e865362e01 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
1415f599b: 0f0b ud2 
1415f599d: 0f1f00 nop dword ptr [rax]
1415f59a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1415f59a5: 55 push rbp
1415f59a6: 56 push rsi
1415f59a7: 57 push rdi
1415f59a8: 53 push rbx
1415f59a9: 4883ec28 sub rsp, 0x28
1415f59ad: 488daa80000000 lea rbp, [rdx + 0x80]
1415f59b4: 488d8dd0010000 lea rcx, [rbp + 0x1d0]
1415f59bb: e820e500ff call 0x140603ee0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
1415f59c0: 90 nop 
1415f59c1: 4883c428 add rsp, 0x28
1415f59c5: 5b pop rbx
1415f59c6: 5f pop rdi
1415f59c7: 5e pop rsi
1415f59c8: 5d pop rbp
1415f59c9: c3 ret 
1415f59ca: 660f1f440000 nop word ptr [rax + rax]
1415f59d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1415f59d5: 55 push rbp
1415f59d6: 56 push rsi
1415f59d7: 57 push rdi
1415f59d8: 53 push rbx
1415f59d9: 4883ec28 sub rsp, 0x28
1415f59dd: 488daa80000000 lea rbp, [rdx + 0x80]
1415f59e4: baa0020000 mov edx, 0x2a0
1415f59e9: 41b808000000 mov r8d, 8
1415f59ef: 488b8d88040000 mov rcx, qword ptr [rbp + 0x488]
1415f59f6: e825e201ff call 0x140613c20 ; unknown
1415f59fb: 90 nop 
1415f59fc: 4883c428 add rsp, 0x28
1415f5a00: 5b pop rbx
1415f5a01: 5f pop rdi
1415f5a02: 5e pop rsi
1415f5a03: 5d pop rbp
1415f5a04: c3 ret 
1415f5a05: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1415f5a10: 4889542410 mov qword ptr [rsp + 0x10], rdx
1415f5a15: 55 push rbp
1415f5a16: 56 push rsi
1415f5a17: 57 push rdi
1415f5a18: 53 push rbx
1415f5a19: 4883ec28 sub rsp, 0x28
1415f5a1d: 488daa80000000 lea rbp, [rdx + 0x80]
1415f5a24: 488d8dd0010000 lea rcx, [rbp + 0x1d0]
1415f5a2b: e8e014fffe call 0x1405e6f10 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<be
1415f5a30: 90 nop 
1415f5a31: 4883c428 add rsp, 0x28
1415f5a35: 5b pop rbx
1415f5a36: 5f pop rdi
1415f5a37: 5e pop rsi
1415f5a38: 5d pop rbp
1415f5a39: c3 ret 
1415f5a3a: 660f1f440000 nop word ptr [rax + rax]
1415f5a40: 4889542410 mov qword ptr [rsp + 0x10], rdx
1415f5a45: 55 push rbp
1415f5a46: 56 push rsi
1415f5a47: 57 push rdi
1415f5a48: 53 push rbx
1415f5a49: 4883ec28 sub rsp, 0x28
1415f5a4d: 488daa80000000 lea rbp, [rdx + 0x80]
1415f5a54: 488d8df0000000 lea rcx, [rbp + 0xf0]
1415f5a5b: e8203ea3fe call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
1415f5a60: 90 nop 
1415f5a61: 4883c428 add rsp, 0x28
1415f5a65: 5b pop rbx
1415f5a66: 5f pop rdi
1415f5a67: 5e pop rsi
1415f5a68: 5d pop rbp
1415f5a69: c3 ret 
1415f5a6a: 660f1f440000 nop word ptr [rax + rax]
1415f5a70: 4889542410 mov qword ptr [rsp + 0x10], rdx
1415f5a75: 55 push rbp
1415f5a76: 56 push rsi
1415f5a77: 57 push rdi
1415f5a78: 53 push rbx
1415f5a79: 4883ec28 sub rsp, 0x28
1415f5a7d: 488daa80000000 lea rbp, [rdx + 0x80]
1415f5a84: 488d8d80000000 lea rcx, [rbp + 0x80]
1415f5a8b: e8f03da3fe call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
1415f5a90: 90 nop 
1415f5a91: 4883c428 add rsp, 0x28
1415f5a95: 5b pop rbx
1415f5a96: 5f pop rdi
1415f5a97: 5e pop rsi
1415f5a98: 5d pop rbp
1415f5a99: c3 ret 
1415f5a9a: 660f1f440000 nop word ptr [rax + rax]
1415f5aa0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1415f5aa5: 55 push rbp
1415f5aa6: 56 push rsi
1415f5aa7: 57 push rdi
1415f5aa8: 53 push rbx
1415f5aa9: 4883ec28 sub rsp, 0x28
1415f5aad: 488daa80000000 lea rbp, [rdx + 0x80]
1415f5ab4: 488d4d10 lea rcx, [rbp + 0x10]
1415f5ab8: e8c33da3fe call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
1415f5abd: 90 nop 
1415f5abe: 4883c428 add rsp, 0x28
1415f5ac2: 5b pop rbx
1415f5ac3: 5f pop rdi
1415f5ac4: 5e pop rsi
1415f5ac5: 5d pop rbp
1415f5ac6: c3 ret 
1415f5ac7: 660f1f840000000000 nop word ptr [rax + rax]
1415f5ad0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1415f5ad5: 55 push rbp
1415f5ad6: 56 push rsi
1415f5ad7: 57 push rdi
1415f5ad8: 53 push rbx
1415f5ad9: 4883ec28 sub rsp, 0x28
1415f5add: 488daa80000000 lea rbp, [rdx + 0x80]
1415f5ae4: 488d4da0 lea rcx, [rbp - 0x60]
1415f5ae8: e8933da3fe call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
1415f5aed: 90 nop 
1415f5aee: 4883c428 add rsp, 0x28
1415f5af2: 5b pop rbx
1415f5af3: 5f pop rdi
1415f5af4: 5e pop rsi
1415f5af5: 5d pop rbp
1415f5af6: c3 ret 
1415f5af7: 660f1f840000000000 nop word ptr [rax + rax]
1415f5b00: 4889542410 mov qword ptr [rsp + 0x10], rdx
1415f5b05: 55 push rbp
1415f5b06: 56 push rsi
1415f5b07: 57 push rdi
1415f5b08: 53 push rbx
1415f5b09: 4883ec28 sub rsp, 0x28
1415f5b0d: 488daa80000000 lea rbp, [rdx + 0x80]
1415f5b14: 488d8d60010000 lea rcx, [rbp + 0x160]
1415f5b1b: e8603da3fe call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
1415f5b20: 90 nop 
1415f5b21: 4883c428 add rsp, 0x28
1415f5b25: 5b pop rbx
1415f5b26: 5f pop rdi
1415f5b27: 5e pop rsi
1415f5b28: 5d pop rbp
1415f5b29: c3 ret 
1415f5b2a: 660f1f440000 nop word ptr [rax + rax]
1415f5b30: 4889542410 mov qword ptr [rsp + 0x10], rdx
1415f5b35: 55 push rbp
1415f5b36: 56 push rsi
1415f5b37: 57 push rdi
1415f5b38: 53 push rbx
1415f5b39: 4883ec28 sub rsp, 0x28
1415f5b3d: 488daa80000000 lea rbp, [rdx + 0x80]
1415f5b44: 488d8dd0010000 lea rcx, [rbp + 0x1d0]
1415f5b4b: e8303da3fe call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
1415f5b50: 90 nop 
1415f5b51: 4883c428 add rsp, 0x28
1415f5b55: 5b pop rbx
1415f5b56: 5f pop rdi
1415f5b57: 5e pop rsi
1415f5b58: 5d pop rbp
1415f5b59: c3 ret 

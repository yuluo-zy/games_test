0000000140924a10: 4156                             push r14
0000000140924a12: 56                               push rsi
0000000140924a13: 57                               push rdi
0000000140924a14: 53                               push rbx
0000000140924a15: 4881ec88000000                   sub rsp, 0x88
0000000140924a1c: 488b7138                         mov rsi, qword ptr [rcx + 0x38]
0000000140924a20: 4881fefeff3f00                   cmp rsi, 0x3ffffe
0000000140924a27: 7607                             jbe 0x140924a30
0000000140924a29: 31c0                             xor eax, eax
0000000140924a2b: e992000000                       jmp 0x140924ac2
0000000140924a30: 4889d3                           mov rbx, rdx
0000000140924a33: 4889cf                           mov rdi, rcx
0000000140924a36: 4c8d7138                         lea r14, [rcx + 0x38]
0000000140924a3a: 48ffc6                           inc rsi
0000000140924a3d: 498936                           mov qword ptr [r14], rsi
0000000140924a40: 0fb605c201d102                   movzx eax, byte ptr [rip + 0x2d101c2]
0000000140924a47: b90c000000                       mov ecx, 0xc
0000000140924a4c: ba04000000                       mov edx, 4
0000000140924a51: e8baf1ceff                       call 0x140613c10 ; 
0000000140924a56: 4885c0                           test rax, rax
0000000140924a59: 0f84cf000000                     je 0x140924b2e
0000000140924a5f: ffce                             dec esi
0000000140924a61: 8b4b08                           mov ecx, dword ptr [rbx + 8]
0000000140924a64: 894808                           mov dword ptr [rax + 8], ecx
0000000140924a67: 488b0b                           mov rcx, qword ptr [rbx]
0000000140924a6a: 488908                           mov qword ptr [rax], rcx
0000000140924a6d: 48c744246001000000               mov qword ptr [rsp + 0x60], 1
0000000140924a76: 4889442468                       mov qword ptr [rsp + 0x68], rax
0000000140924a7b: 48c744247001000000               mov qword ptr [rsp + 0x70], 1
0000000140924a84: c7442478cdcccc3e                 mov dword ptr [rsp + 0x78], 0x3ecccccd
0000000140924a8c: 8974247c                         mov dword ptr [rsp + 0x7c], esi
0000000140924a90: c784248000000000002040           mov dword ptr [rsp + 0x80], 0x40200000
0000000140924a9b: 4883c708                         add rdi, 8
0000000140924a9f: 488d4c2420                       lea rcx, [rsp + 0x20]
0000000140924aa4: 4c8d4c2460                       lea r9, [rsp + 0x60]
0000000140924aa9: 4889fa                           mov rdx, rdi
0000000140924aac: 4189f0                           mov r8d, esi
0000000140924aaf: e85c22c000                       call 0x141526d10 ; hashbrown::map::HashMap::insert<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global>
0000000140924ab4: 31c0                             xor eax, eax
0000000140924ab6: 483b442420                       cmp rax, qword ptr [rsp + 0x20]
0000000140924abb: 7114                             jno 0x140924ad1
0000000140924abd: b801000000                       mov eax, 1
0000000140924ac2: 89f2                             mov edx, esi
0000000140924ac4: 4881c488000000                   add rsp, 0x88
0000000140924acb: 5b                               pop rbx
0000000140924acc: 5f                               pop rdi
0000000140924acd: 5e                               pop rsi
0000000140924ace: 415e                             pop r14
0000000140924ad0: c3                               ret 
0000000140924ad1: 488d4c2420                       lea rcx, [rsp + 0x20]
0000000140924ad6: e815eeffff                       call 0x1409238f0 ; core::ptr::drop_in_place<enum2$<core::option::Option<country_core::resources::terrain_stroke::TerrainStroke> > >
0000000140924adb: 4c89742450                       mov qword ptr [rsp + 0x50], r14
0000000140924ae0: 488d05d98b4600                   lea rax, [rip + 0x468bd9]
0000000140924ae7: 4889442458                       mov qword ptr [rsp + 0x58], rax
0000000140924aec: 488d05759e1802                   lea rax, [rip + 0x2189e75]
0000000140924af3: 4889442420                       mov qword ptr [rsp + 0x20], rax
0000000140924af8: 48c744242801000000               mov qword ptr [rsp + 0x28], 1
0000000140924b01: 48c744244000000000               mov qword ptr [rsp + 0x40], 0
0000000140924b0a: 488d442450                       lea rax, [rsp + 0x50]
0000000140924b0f: 4889442430                       mov qword ptr [rsp + 0x30], rax
0000000140924b14: 48c744243801000000               mov qword ptr [rsp + 0x38], 1
0000000140924b1d: 488d15549e1802                   lea rdx, [rip + 0x2189e54]
0000000140924b24: 488d4c2420                       lea rcx, [rsp + 0x20]
0000000140924b29: e86248fb01                       call 0x1428d9390 ; core::panicking::panic_fmt
0000000140924b2e: b904000000                       mov ecx, 4
0000000140924b33: ba0c000000                       mov edx, 0xc
0000000140924b38: e8c344fb01                       call 0x1428d9000 ; alloc::alloc::handle_alloc_error
0000000140924b3d: cc                               int3 

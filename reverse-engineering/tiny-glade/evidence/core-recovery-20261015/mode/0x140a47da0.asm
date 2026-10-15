0x140a47da0 push     rsi
0x140a47da1 push     rdi
0x140a47da2 push     rbx
0x140a47da3 sub      rsp, 0x50
0x140a47da7 mov      rsi, rcx
0x140a47daa mov      rcx, qword ptr [rdx + 0x20]
0x140a47dae mov      rax, qword ptr [rdx + 0x28]
0x140a47db2 cmp      rax, rcx
0x140a47db5 jae      0x140a47df6
0x140a47db7 mov      r8, qword ptr [rdx + 0x18]
0x140a47dbb inc      rax
0x140a47dbe neg      rcx
0x140a47dc1 movabs   rdi, 0x100002600
0x140a47dcb nop      dword ptr [rax + rax]
0x140a47dd0 movzx    r9d, byte ptr [r8 + rax - 1]
0x140a47dd6 cmp      r9, 0x22
0x140a47dda ja       0x140a47e44
0x140a47ddc bt       rdi, r9
0x140a47de0 jae      0x140a47e11
0x140a47de2 mov      qword ptr [rdx + 0x28], rax
0x140a47de6 lea      r9, [rcx + rax]
0x140a47dea inc      r9
0x140a47ded inc      rax
0x140a47df0 cmp      r9, 1
0x140a47df4 jne      0x140a47dd0
0x140a47df6 mov      qword ptr [rsp + 0x28], 5
0x140a47dff lea      rax, [rsp + 0x28]
0x140a47e04 mov      rcx, rdx
0x140a47e07 mov      rdx, rax
0x140a47e0a call     0x1428a0920 ; serde_json::de::Deserializer$LT$R$GT$::peek_error::hee6f49fa8612ef7c (.llvm.3546337791550519002) / serde_json::de::Deserializer$LT$R$GT$::peek_error::h3c4b8448a00c9d9a
0x140a47e0f jmp      0x140a47e74 ; 
0x140a47e11 cmp      r9, 0x22
0x140a47e15 jne      0x140a47e44
0x140a47e17 lea      rcx, [rsp + 0x28]
0x140a47e1c call     0x14140d630 ; _$LT$core..marker..PhantomData$LT$T$GT$$u20$as$u20$serde..de..DeserializeSeed$GT$::deserialize::hca537e754bbb67ed (.llvm.2285519678028410232)
0x140a47e21 cmp      byte ptr [rsp + 0x28], 1
0x140a47e26 je       0x140a47e6f
0x140a47e28 movzx    eax, byte ptr [rsp + 0x29]
0x140a47e2d lea      rcx, [rip + 0x2087934]
0x140a47e34 movsxd   rax, dword ptr [rcx + rax*4]
0x140a47e38 add      rax, rcx
0x140a47e3b jmp      rax
0x140a47e3d mov      word ptr [rsi], 0x3800
0x140a47e42 jmp      0x140a47e7b ; 
0x140a47e44 cmp      r9d, 0x7b
0x140a47e48 jne      0x140a47f55
0x140a47e4e dec      byte ptr [rdx + 0x30]
0x140a47e51 je       0x140a4809b
0x140a47e57 mov      qword ptr [rdx + 0x28], rax
0x140a47e5b lea      rcx, [rsp + 0x28]
0x140a47e60 mov      rbx, rdx
0x140a47e63 call     0x14140d630 ; _$LT$core..marker..PhantomData$LT$T$GT$$u20$as$u20$serde..de..DeserializeSeed$GT$::deserialize::hca537e754bbb67ed (.llvm.2285519678028410232)
0x140a47e68 cmp      byte ptr [rsp + 0x28], 0
0x140a47e6d je       0x140a47e86
0x140a47e6f mov      rax, qword ptr [rsp + 0x30]
0x140a47e74 mov      qword ptr [rsi + 8], rax
0x140a47e78 mov      byte ptr [rsi], 1
0x140a47e7b mov      rax, rsi
0x140a47e7e add      rsp, 0x50
0x140a47e82 pop      rbx
0x140a47e83 pop      rdi
0x140a47e84 pop      rsi
0x140a47e85 ret      
0x140a47e86 mov      rcx, rbx
0x140a47e89 mov      r8, qword ptr [rbx + 0x20]
0x140a47e8d mov      rax, qword ptr [rbx + 0x28]
0x140a47e91 cmp      rax, r8
0x140a47e94 jae      0x140a47ecf
0x140a47e96 movzx    edx, byte ptr [rsp + 0x29]
0x140a47e9b mov      r9, qword ptr [rcx + 0x18]
0x140a47e9f inc      rax
0x140a47ea2 neg      r8
0x140a47ea5 movzx    r10d, byte ptr [r9 + rax - 1]
0x140a47eab cmp      r10, 0x3a
0x140a47eaf ja       0x140a480a9
0x140a47eb5 bt       rdi, r10
0x140a47eb9 jae      0x140a47ee4
0x140a47ebb mov      qword ptr [rcx + 0x28], rax
0x140a47ebf lea      r10, [r8 + rax]
0x140a47ec3 inc      r10
0x140a47ec6 inc      rax
0x140a47ec9 cmp      r10, 1
0x140a47ecd jne      0x140a47ea5
0x140a47ecf mov      qword ptr [rsp + 0x28], 3
0x140a47ed8 lea      rdx, [rsp + 0x28]
0x140a47edd call     0x1428a0920 ; serde_json::de::Deserializer$LT$R$GT$::peek_error::hee6f49fa8612ef7c (.llvm.3546337791550519002) / serde_json::de::Deserializer$LT$R$GT$::peek_error::h3c4b8448a00c9d9a
0x140a47ee2 jmp      0x140a47e74 ; 
0x140a47ee4 cmp      r10, 0x3a
0x140a47ee8 jne      0x140a480a9
0x140a47eee mov      qword ptr [rcx + 0x28], rax
0x140a47ef2 mov      eax, edx
0x140a47ef4 lea      rdx, [rip + 0x208788d]
0x140a47efb movsxd   rax, dword ptr [rdx + rax*4]
0x140a47eff add      rax, rdx
0x140a47f02 jmp      rax
0x140a47f04 call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140a47f09 test     rax, rax
0x140a47f0c jne      0x140a47e74
0x140a47f12 mov      al, 0x38
0x140a47f14 jmp      0x140a48005 ; 
0x140a47f19 mov      word ptr [rsi], 0x3f00
0x140a47f1e jmp      0x140a47e7b ; 
0x140a47f23 mov      word ptr [rsi], 0x3c00
0x140a47f28 jmp      0x140a47e7b ; 
0x140a47f2d mov      word ptr [rsi], 0x3a00
0x140a47f32 jmp      0x140a47e7b ; 
0x140a47f37 mov      word ptr [rsi], 0x3b00
0x140a47f3c jmp      0x140a47e7b ; 
0x140a47f41 mov      word ptr [rsi], 0x3900
0x140a47f46 jmp      0x140a47e7b ; 
0x140a47f4b mov      word ptr [rsi], 0x3d00
0x140a47f50 jmp      0x140a47e7b ; 
0x140a47f55 mov      qword ptr [rsp + 0x28], 0xa
0x140a47f5e jmp      0x140a47dff ; 
0x140a47f63 call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140a47f68 test     rax, rax
0x140a47f6b jne      0x140a47e74
0x140a47f71 mov      al, 0x3c
0x140a47f73 jmp      0x140a48005 ; 
0x140a47f78 call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140a47f7d test     rax, rax
0x140a47f80 jne      0x140a47e74
0x140a47f86 mov      al, 0x3a
0x140a47f88 jmp      0x140a48005 ; 
0x140a47f8a call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140a47f8f test     rax, rax
0x140a47f92 jne      0x140a47e74
0x140a47f98 mov      al, 0x3b
0x140a47f9a jmp      0x140a48005 ; 
0x140a47f9c call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140a47fa1 test     rax, rax
0x140a47fa4 jne      0x140a47e74
0x140a47faa mov      al, 0x3f
0x140a47fac jmp      0x140a48005 ; 
0x140a47fae call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140a47fb3 test     rax, rax
0x140a47fb6 jne      0x140a47e74
0x140a47fbc mov      al, 0x39
0x140a47fbe jmp      0x140a48005 ; 
0x140a47fc0 call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140a47fc5 test     rax, rax
0x140a47fc8 jne      0x140a47e74
0x140a47fce mov      al, 0x3d
0x140a47fd0 jmp      0x140a48005 ; 
0x140a47fd2 lea      r8, [rip + 0x2086e47]
0x140a47fd9 lea      rax, [rsp + 0x40]
0x140a47fde mov      r9d, 1
0x140a47fe4 mov      rdx, rcx
0x140a47fe7 mov      rcx, rax
0x140a47fea call     0x14140e700 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::struct_variant::h953439d0fe49767b
0x140a47fef test     byte ptr [rsp + 0x40], 1
0x140a47ff4 je       0x140a48000
0x140a47ff6 mov      rax, qword ptr [rsp + 0x48]
0x140a47ffb jmp      0x140a47e74 ; 
0x140a48000 movzx    eax, byte ptr [rsp + 0x41]
0x140a48005 mov      rcx, rbx
0x140a48008 inc      byte ptr [rbx + 0x30]
0x140a4800b mov      r8, qword ptr [rbx + 0x20]
0x140a4800f mov      rdx, qword ptr [rbx + 0x28]
0x140a48013 cmp      rdx, r8
0x140a48016 jae      0x140a48048
0x140a48018 mov      r9, qword ptr [rcx + 0x18]
0x140a4801c inc      rdx
0x140a4801f neg      r8
0x140a48022 movzx    r10d, byte ptr [r9 + rdx - 1]
0x140a48028 cmp      r10, 0x20
0x140a4802c ja       0x140a48063
0x140a4802e bt       rdi, r10
0x140a48032 jae      0x140a48063
0x140a48034 mov      qword ptr [rcx + 0x28], rdx
0x140a48038 lea      r10, [r8 + rdx]
0x140a4803c inc      r10
0x140a4803f inc      rdx
0x140a48042 cmp      r10, 1
0x140a48046 jne      0x140a48022
0x140a48048 mov      qword ptr [rsp + 0x28], 3
0x140a48051 lea      rdx, [rsp + 0x28]
0x140a48056 call     0x1428a0fa0 ; serde_json::de::Deserializer$LT$R$GT$::error::h89ce4a093855ca44 / serde_json::de::Deserializer$LT$R$GT$::error::ha7ea65e1a49f0611
0x140a4805b mov      qword ptr [rsi + 8], rax
0x140a4805f mov      al, 1
0x140a48061 jmp      0x140a48072 ; 
0x140a48063 cmp      r10d, 0x7d
0x140a48067 jne      0x140a480b7
0x140a48069 mov      qword ptr [rcx + 0x28], rdx
0x140a4806d mov      byte ptr [rsi + 1], al
0x140a48070 xor      eax, eax
0x140a48072 mov      byte ptr [rsi], al
0x140a48074 jmp      0x140a47e7b ; 
0x140a48079 mov      byte ptr [rsp + 0x28], 0xd
0x140a4807e lea      rdx, [rip + 0x2213e5b]
0x140a48085 lea      r8, [rip + 0x2213e64]
0x140a4808c lea      rcx, [rsp + 0x28]
0x140a48091 call     0x1428c9a00 ; _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type::h76868c415965851f
0x140a48096 jmp      0x140a47e74 ; 
0x140a4809b mov      qword ptr [rsp + 0x28], 0x15
0x140a480a4 jmp      0x140a47dff ; 
0x140a480a9 mov      qword ptr [rsp + 0x28], 6
0x140a480b2 jmp      0x140a47ed8 ; 
0x140a480b7 mov      qword ptr [rsp + 0x28], 0xa
0x140a480c0 jmp      0x140a48051 ; 
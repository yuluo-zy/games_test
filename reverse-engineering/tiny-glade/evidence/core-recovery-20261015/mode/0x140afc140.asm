0x140afc140 push     rsi
0x140afc141 push     rdi
0x140afc142 push     rbx
0x140afc143 sub      rsp, 0x60
0x140afc147 mov      rsi, rcx
0x140afc14a mov      rcx, qword ptr [rdx + 0x20]
0x140afc14e mov      rax, qword ptr [rdx + 0x28]
0x140afc152 cmp      rax, rcx
0x140afc155 jae      0x140afc196
0x140afc157 mov      r8, qword ptr [rdx + 0x18]
0x140afc15b inc      rax
0x140afc15e neg      rcx
0x140afc161 movabs   rdi, 0x100002600
0x140afc16b nop      dword ptr [rax + rax]
0x140afc170 movzx    r9d, byte ptr [r8 + rax - 1]
0x140afc176 cmp      r9, 0x22
0x140afc17a ja       0x140afc1dd
0x140afc17c bt       rdi, r9
0x140afc180 jae      0x140afc1b1
0x140afc182 mov      qword ptr [rdx + 0x28], rax
0x140afc186 lea      r9, [rcx + rax]
0x140afc18a inc      r9
0x140afc18d inc      rax
0x140afc190 cmp      r9, 1
0x140afc194 jne      0x140afc170
0x140afc196 mov      qword ptr [rsp + 0x28], 5
0x140afc19f lea      rax, [rsp + 0x28]
0x140afc1a4 mov      rcx, rdx
0x140afc1a7 mov      rdx, rax
0x140afc1aa call     0x1428a0920 ; serde_json::de::Deserializer$LT$R$GT$::peek_error::hee6f49fa8612ef7c (.llvm.3546337791550519002) / serde_json::de::Deserializer$LT$R$GT$::peek_error::h3c4b8448a00c9d9a
0x140afc1af jmp      0x140afc20d ; 
0x140afc1b1 cmp      r9, 0x22
0x140afc1b5 jne      0x140afc1dd
0x140afc1b7 lea      rcx, [rsp + 0x28]
0x140afc1bc call     0x14140d3f0 ; _$LT$core..marker..PhantomData$LT$T$GT$$u20$as$u20$serde..de..DeserializeSeed$GT$::deserialize::had84699031ee31b2 (.llvm.2285519678028410232)
0x140afc1c1 cmp      byte ptr [rsp + 0x28], 1
0x140afc1c6 je       0x140afc208
0x140afc1c8 movzx    eax, byte ptr [rsp + 0x29]
0x140afc1cd test     eax, eax
0x140afc1cf jne      0x140afc2c1
0x140afc1d5 mov      dword ptr [rsi], 0
0x140afc1db jmp      0x140afc217 ; 
0x140afc1dd cmp      r9d, 0x7b
0x140afc1e1 jne      0x140afc42b
0x140afc1e7 dec      byte ptr [rdx + 0x30]
0x140afc1ea je       0x140afc46a
0x140afc1f0 mov      qword ptr [rdx + 0x28], rax
0x140afc1f4 lea      rcx, [rsp + 0x28]
0x140afc1f9 mov      rbx, rdx
0x140afc1fc call     0x14140d3f0 ; _$LT$core..marker..PhantomData$LT$T$GT$$u20$as$u20$serde..de..DeserializeSeed$GT$::deserialize::had84699031ee31b2 (.llvm.2285519678028410232)
0x140afc201 cmp      byte ptr [rsp + 0x28], 0
0x140afc206 je       0x140afc222
0x140afc208 mov      rax, qword ptr [rsp + 0x30]
0x140afc20d mov      qword ptr [rsi + 8], rax
0x140afc211 mov      dword ptr [rsi], 6
0x140afc217 mov      rax, rsi
0x140afc21a add      rsp, 0x60
0x140afc21e pop      rbx
0x140afc21f pop      rdi
0x140afc220 pop      rsi
0x140afc221 ret      
0x140afc222 mov      rax, rbx
0x140afc225 mov      r8, qword ptr [rbx + 0x20]
0x140afc229 mov      rcx, qword ptr [rbx + 0x28]
0x140afc22d cmp      rcx, r8
0x140afc230 jae      0x140afc26b
0x140afc232 movzx    edx, byte ptr [rsp + 0x29]
0x140afc237 mov      r9, qword ptr [rax + 0x18]
0x140afc23b inc      rcx
0x140afc23e neg      r8
0x140afc241 movzx    r10d, byte ptr [r9 + rcx - 1]
0x140afc247 cmp      r10, 0x3a
0x140afc24b ja       0x140afc478
0x140afc251 bt       rdi, r10
0x140afc255 jae      0x140afc283
0x140afc257 mov      qword ptr [rax + 0x28], rcx
0x140afc25b lea      r10, [r8 + rcx]
0x140afc25f inc      r10
0x140afc262 inc      rcx
0x140afc265 cmp      r10, 1
0x140afc269 jne      0x140afc241
0x140afc26b mov      qword ptr [rsp + 0x28], 3
0x140afc274 lea      rdx, [rsp + 0x28]
0x140afc279 mov      rcx, rax
0x140afc27c call     0x1428a0920 ; serde_json::de::Deserializer$LT$R$GT$::peek_error::hee6f49fa8612ef7c (.llvm.3546337791550519002) / serde_json::de::Deserializer$LT$R$GT$::peek_error::h3c4b8448a00c9d9a
0x140afc281 jmp      0x140afc20d ; 
0x140afc283 cmp      r10, 0x3a
0x140afc287 jne      0x140afc478
0x140afc28d mov      qword ptr [rax + 0x28], rcx
0x140afc291 mov      ecx, edx
0x140afc293 lea      rdx, [rip + 0x1fe9eda]
0x140afc29a movsxd   rcx, dword ptr [rdx + rcx*4]
0x140afc29e add      rcx, rdx
0x140afc2a1 jmp      rcx
0x140afc2a3 mov      rcx, rax
0x140afc2a6 call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afc2ab test     rax, rax
0x140afc2ae jne      0x140afc20d
0x140afc2b4 mov      dword ptr [rsp + 0x40], 0
0x140afc2bc jmp      0x140afc34a ; 
0x140afc2c1 dec      eax
0x140afc2c3 lea      rcx, [rip + 0x1fe9e96]
0x140afc2ca movsxd   rax, dword ptr [rcx + rax*4]
0x140afc2ce add      rax, rcx
0x140afc2d1 jmp      rax
0x140afc2d3 mov      byte ptr [rsp + 0x28], 0xd
0x140afc2d8 lea      rdx, [rip + 0x215fc41]
0x140afc2df jmp      0x140afc445 ; 
0x140afc2e4 lea      r8, [rip + 0x1fe98ed]
0x140afc2eb lea      rcx, [rsp + 0x40]
0x140afc2f0 mov      r9d, 1
0x140afc2f6 mov      rdx, rax
0x140afc2f9 call     0x14140e130 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::struct_variant::h907afb50277b597a
0x140afc2fe mov      eax, dword ptr [rsp + 0x40]
0x140afc302 cmp      eax, 6
0x140afc305 jne      0x140afc34a
0x140afc307 jmp      0x140afc3fc ; 
0x140afc30c mov      rcx, rax
0x140afc30f call     0x141412f50 ; _$LT$$RF$mut$u20$serde_json..de..Deserializer$LT$R$GT$$u20$as$u20$serde..de..Deserializer$GT$::deserialize_u64::hdab8276a6ceab783
0x140afc314 and      eax, 1
0x140afc317 lea      eax, [rax*4 + 2]
0x140afc31e jmp      0x140afc3ea ; 
0x140afc323 lea      r8, [rip + 0x1fe98ae]
0x140afc32a lea      rcx, [rsp + 0x40]
0x140afc32f mov      r9d, 1
0x140afc335 mov      rdx, rax
0x140afc338 call     0x14140f510 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::struct_variant::hc64fc7c55c0d9c06
0x140afc33d mov      eax, dword ptr [rsp + 0x40]
0x140afc341 cmp      eax, 6
0x140afc344 je       0x140afc3fc
0x140afc34a mov      eax, dword ptr [rsp + 0x40]
0x140afc34e mov      dword ptr [rsp + 0x50], eax
0x140afc352 mov      rax, qword ptr [rsp + 0x44]
0x140afc357 mov      qword ptr [rsp + 0x54], rax
0x140afc35c mov      eax, dword ptr [rsp + 0x4c]
0x140afc360 mov      rcx, rbx
0x140afc363 inc      byte ptr [rbx + 0x30]
0x140afc366 mov      dword ptr [rsp + 0x5c], eax
0x140afc36a mov      rdx, qword ptr [rbx + 0x20]
0x140afc36e mov      rax, qword ptr [rbx + 0x28]
0x140afc372 cmp      rax, rdx
0x140afc375 jae      0x140afc3a7
0x140afc377 mov      r8, qword ptr [rcx + 0x18]
0x140afc37b inc      rax
0x140afc37e neg      rdx
0x140afc381 movzx    r9d, byte ptr [r8 + rax - 1]
0x140afc387 cmp      r9, 0x20
0x140afc38b ja       0x140afc3bf
0x140afc38d bt       rdi, r9
0x140afc391 jae      0x140afc3bf
0x140afc393 mov      qword ptr [rcx + 0x28], rax
0x140afc397 lea      r9, [rdx + rax]
0x140afc39b inc      r9
0x140afc39e inc      rax
0x140afc3a1 cmp      r9, 1
0x140afc3a5 jne      0x140afc381
0x140afc3a7 mov      qword ptr [rsp + 0x28], 3
0x140afc3b0 lea      rdx, [rsp + 0x28]
0x140afc3b5 call     0x1428a0fa0 ; serde_json::de::Deserializer$LT$R$GT$::error::h89ce4a093855ca44 / serde_json::de::Deserializer$LT$R$GT$::error::ha7ea65e1a49f0611
0x140afc3ba jmp      0x140afc20d ; 
0x140afc3bf cmp      r9d, 0x7d
0x140afc3c3 jne      0x140afc486
0x140afc3c9 mov      qword ptr [rcx + 0x28], rax
0x140afc3cd movaps   xmm0, xmmword ptr [rsp + 0x50]
0x140afc3d2 movups   xmmword ptr [rsi], xmm0
0x140afc3d5 jmp      0x140afc217 ; 
0x140afc3da mov      rcx, rax
0x140afc3dd call     0x141412f50 ; _$LT$$RF$mut$u20$serde_json..de..Deserializer$LT$R$GT$$u20$as$u20$serde..de..Deserializer$GT$::deserialize_u64::hdab8276a6ceab783
0x140afc3e2 and      eax, 1
0x140afc3e5 lea      eax, [rax + rax*4]
0x140afc3e8 inc      eax
0x140afc3ea mov      qword ptr [rsp + 0x48], rdx
0x140afc3ef mov      dword ptr [rsp + 0x40], eax
0x140afc3f3 cmp      eax, 6
0x140afc3f6 jne      0x140afc34a
0x140afc3fc mov      rax, qword ptr [rsp + 0x48]
0x140afc401 jmp      0x140afc20d ; 
0x140afc406 lea      rcx, [rsp + 0x28]
0x140afc40b mov      rdx, rax
0x140afc40e call     0x141412c00 ; _$LT$$RF$mut$u20$serde_json..de..Deserializer$LT$R$GT$$u20$as$u20$serde..de..Deserializer$GT$::deserialize_u32::h63a66fd8c05ae259
0x140afc413 test     byte ptr [rsp + 0x28], 1
0x140afc418 je       0x140afc45b
0x140afc41a mov      rax, qword ptr [rsp + 0x30]
0x140afc41f mov      qword ptr [rsp + 0x48], rax
0x140afc424 mov      eax, 6
0x140afc429 jmp      0x140afc3ef ; 
0x140afc42b mov      qword ptr [rsp + 0x28], 0xa
0x140afc434 jmp      0x140afc19f ; 
0x140afc439 mov      byte ptr [rsp + 0x28], 0xd
0x140afc43e lea      rdx, [rip + 0x215fa9b]
0x140afc445 lea      r8, [rip + 0x215faa4]
0x140afc44c lea      rcx, [rsp + 0x28]
0x140afc451 call     0x1428c9a00 ; _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type::h76868c415965851f
0x140afc456 jmp      0x140afc20d ; 
0x140afc45b mov      eax, dword ptr [rsp + 0x2c]
0x140afc45f mov      dword ptr [rsp + 0x44], eax
0x140afc463 mov      eax, 5
0x140afc468 jmp      0x140afc3ef ; 
0x140afc46a mov      qword ptr [rsp + 0x28], 0x15
0x140afc473 jmp      0x140afc19f ; 
0x140afc478 mov      qword ptr [rsp + 0x28], 6
0x140afc481 jmp      0x140afc274 ; 
0x140afc486 mov      qword ptr [rsp + 0x28], 0xa
0x140afc48f jmp      0x140afc3b0 ; 
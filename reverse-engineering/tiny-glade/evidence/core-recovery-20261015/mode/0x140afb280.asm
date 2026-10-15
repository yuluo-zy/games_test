0x140afb280 push     r14
0x140afb282 push     rsi
0x140afb283 push     rdi
0x140afb284 push     rbx
0x140afb285 sub      rsp, 0x38
0x140afb289 mov      rsi, rcx
0x140afb28c mov      rcx, qword ptr [rdx + 0x20]
0x140afb290 mov      rax, qword ptr [rdx + 0x28]
0x140afb294 cmp      rax, rcx
0x140afb297 jae      0x140afb2d6
0x140afb299 mov      r8, qword ptr [rdx + 0x18]
0x140afb29d inc      rax
0x140afb2a0 neg      rcx
0x140afb2a3 movabs   rdi, 0x100002600
0x140afb2ad nop      dword ptr [rax]
0x140afb2b0 movzx    r9d, byte ptr [r8 + rax - 1]
0x140afb2b6 cmp      r9, 0x22
0x140afb2ba ja       0x140afb32b
0x140afb2bc bt       rdi, r9
0x140afb2c0 jae      0x140afb2f4
0x140afb2c2 mov      qword ptr [rdx + 0x28], rax
0x140afb2c6 lea      r9, [rcx + rax]
0x140afb2ca inc      r9
0x140afb2cd inc      rax
0x140afb2d0 cmp      r9, 1
0x140afb2d4 jne      0x140afb2b0
0x140afb2d6 mov      qword ptr [rsp + 0x20], 5
0x140afb2df lea      rax, [rsp + 0x20]
0x140afb2e4 mov      rcx, rdx
0x140afb2e7 mov      rdx, rax
0x140afb2ea call     0x1428a0920 ; serde_json::de::Deserializer$LT$R$GT$::peek_error::hee6f49fa8612ef7c (.llvm.3546337791550519002) / serde_json::de::Deserializer$LT$R$GT$::peek_error::h3c4b8448a00c9d9a
0x140afb2ef jmp      0x140afb3ed ; 
0x140afb2f4 cmp      r9, 0x22
0x140afb2f8 jne      0x140afb32b
0x140afb2fa lea      rcx, [rsp + 0x20]
0x140afb2ff call     0x14140d150 ; _$LT$core..marker..PhantomData$LT$T$GT$$u20$as$u20$serde..de..DeserializeSeed$GT$::deserialize::ha4cefc7a2161a691 (.llvm.2285519678028410232)
0x140afb304 cmp      byte ptr [rsp + 0x20], 1
0x140afb309 je       0x140afb356
0x140afb30b movzx    eax, byte ptr [rsp + 0x21]
0x140afb310 lea      rcx, [rip + 0x1feaced]
0x140afb317 movsxd   rax, dword ptr [rcx + rax*4]
0x140afb31b add      rax, rcx
0x140afb31e jmp      rax
0x140afb320 mov      byte ptr [rsi + 1], 0
0x140afb324 xor      eax, eax
0x140afb326 jmp      0x140afb3f3 ; 
0x140afb32b cmp      r9d, 0x7b
0x140afb32f jne      0x140afb470
0x140afb335 dec      byte ptr [rdx + 0x30]
0x140afb338 je       0x140afb565
0x140afb33e mov      qword ptr [rdx + 0x28], rax
0x140afb342 lea      rcx, [rsp + 0x20]
0x140afb347 mov      r14, rdx
0x140afb34a call     0x14140d150 ; _$LT$core..marker..PhantomData$LT$T$GT$$u20$as$u20$serde..de..DeserializeSeed$GT$::deserialize::ha4cefc7a2161a691 (.llvm.2285519678028410232)
0x140afb34f cmp      byte ptr [rsp + 0x20], 0
0x140afb354 je       0x140afb360
0x140afb356 mov      rax, qword ptr [rsp + 0x28]
0x140afb35b jmp      0x140afb3ed ; 
0x140afb360 mov      rcx, r14
0x140afb363 mov      rdx, qword ptr [r14 + 0x20]
0x140afb367 mov      rax, qword ptr [r14 + 0x28]
0x140afb36b cmp      rax, rdx
0x140afb36e jae      0x140afb3ad
0x140afb370 movzx    ebx, byte ptr [rsp + 0x21]
0x140afb375 movzx    r8d, bl
0x140afb379 mov      r9, qword ptr [rcx + 0x18]
0x140afb37d inc      rax
0x140afb380 neg      rdx
0x140afb383 movzx    r10d, byte ptr [r9 + rax - 1]
0x140afb389 cmp      r10, 0x3a
0x140afb38d ja       0x140afb573
0x140afb393 bt       rdi, r10
0x140afb397 jae      0x140afb3c2
0x140afb399 mov      qword ptr [rcx + 0x28], rax
0x140afb39d lea      r10, [rdx + rax]
0x140afb3a1 inc      r10
0x140afb3a4 inc      rax
0x140afb3a7 cmp      r10, 1
0x140afb3ab jne      0x140afb383
0x140afb3ad mov      qword ptr [rsp + 0x20], 3
0x140afb3b6 lea      rdx, [rsp + 0x20]
0x140afb3bb call     0x1428a0920 ; serde_json::de::Deserializer$LT$R$GT$::peek_error::hee6f49fa8612ef7c (.llvm.3546337791550519002) / serde_json::de::Deserializer$LT$R$GT$::peek_error::h3c4b8448a00c9d9a
0x140afb3c0 jmp      0x140afb3ed ; 
0x140afb3c2 cmp      r10, 0x3a
0x140afb3c6 jne      0x140afb573
0x140afb3cc mov      qword ptr [rcx + 0x28], rax
0x140afb3d0 mov      eax, r8d
0x140afb3d3 lea      rdx, [rip + 0x1feac82]
0x140afb3da movsxd   rax, dword ptr [rdx + rax*4]
0x140afb3de add      rax, rdx
0x140afb3e1 jmp      rax
0x140afb3e3 call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afb3e8 test     rax, rax
0x140afb3eb je       0x140afb402
0x140afb3ed mov      qword ptr [rsi + 8], rax
0x140afb3f1 mov      al, 1
0x140afb3f3 mov      byte ptr [rsi], al
0x140afb3f5 mov      rax, rsi
0x140afb3f8 add      rsp, 0x38
0x140afb3fc pop      rbx
0x140afb3fd pop      rdi
0x140afb3fe pop      rsi
0x140afb3ff pop      r14
0x140afb401 ret      
0x140afb402 mov      rcx, r14
0x140afb405 inc      byte ptr [r14 + 0x30]
0x140afb409 mov      rdx, qword ptr [r14 + 0x20]
0x140afb40d mov      rax, qword ptr [r14 + 0x28]
0x140afb411 cmp      rax, rdx
0x140afb414 jae      0x140afb446
0x140afb416 mov      r8, qword ptr [rcx + 0x18]
0x140afb41a inc      rax
0x140afb41d neg      rdx
0x140afb420 movzx    r9d, byte ptr [r8 + rax - 1]
0x140afb426 cmp      r9, 0x20
0x140afb42a ja       0x140afb45b
0x140afb42c bt       rdi, r9
0x140afb430 jae      0x140afb45b
0x140afb432 mov      qword ptr [rcx + 0x28], rax
0x140afb436 lea      r9, [rdx + rax]
0x140afb43a inc      r9
0x140afb43d inc      rax
0x140afb440 cmp      r9, 1
0x140afb444 jne      0x140afb420
0x140afb446 mov      qword ptr [rsp + 0x20], 3
0x140afb44f lea      rdx, [rsp + 0x20]
0x140afb454 call     0x1428a0fa0 ; serde_json::de::Deserializer$LT$R$GT$::error::h89ce4a093855ca44 / serde_json::de::Deserializer$LT$R$GT$::error::ha7ea65e1a49f0611
0x140afb459 jmp      0x140afb3ed ; 
0x140afb45b cmp      r9d, 0x7d
0x140afb45f jne      0x140afb581
0x140afb465 mov      qword ptr [rcx + 0x28], rax
0x140afb469 mov      byte ptr [rsi + 1], bl
0x140afb46c xor      eax, eax
0x140afb46e jmp      0x140afb3f3 ; 
0x140afb470 mov      qword ptr [rsp + 0x20], 0xa
0x140afb479 jmp      0x140afb2df ; 
0x140afb47e mov      byte ptr [rsi + 1], 0x10
0x140afb482 xor      eax, eax
0x140afb484 jmp      0x140afb3f3 ; 
0x140afb489 mov      byte ptr [rsi + 1], 0xe
0x140afb48d xor      eax, eax
0x140afb48f jmp      0x140afb3f3 ; 
0x140afb494 mov      byte ptr [rsi + 1], 0xb
0x140afb498 xor      eax, eax
0x140afb49a jmp      0x140afb3f3 ; 
0x140afb49f mov      byte ptr [rsi + 1], 4
0x140afb4a3 xor      eax, eax
0x140afb4a5 jmp      0x140afb3f3 ; 
0x140afb4aa mov      byte ptr [rsi + 1], 0xc
0x140afb4ae xor      eax, eax
0x140afb4b0 jmp      0x140afb3f3 ; 
0x140afb4b5 mov      byte ptr [rsi + 1], 9
0x140afb4b9 xor      eax, eax
0x140afb4bb jmp      0x140afb3f3 ; 
0x140afb4c0 mov      byte ptr [rsi + 1], 2
0x140afb4c4 xor      eax, eax
0x140afb4c6 jmp      0x140afb3f3 ; 
0x140afb4cb mov      byte ptr [rsi + 1], 0xf
0x140afb4cf xor      eax, eax
0x140afb4d1 jmp      0x140afb3f3 ; 
0x140afb4d6 mov      byte ptr [rsi + 1], 3
0x140afb4da xor      eax, eax
0x140afb4dc jmp      0x140afb3f3 ; 
0x140afb4e1 mov      byte ptr [rsi + 1], 7
0x140afb4e5 xor      eax, eax
0x140afb4e7 jmp      0x140afb3f3 ; 
0x140afb4ec mov      byte ptr [rsi + 1], 1
0x140afb4f0 xor      eax, eax
0x140afb4f2 jmp      0x140afb3f3 ; 
0x140afb4f7 mov      byte ptr [rsi + 1], 0x11
0x140afb4fb xor      eax, eax
0x140afb4fd jmp      0x140afb3f3 ; 
0x140afb502 mov      byte ptr [rsi + 1], 0x14
0x140afb506 xor      eax, eax
0x140afb508 jmp      0x140afb3f3 ; 
0x140afb50d mov      byte ptr [rsi + 1], 5
0x140afb511 xor      eax, eax
0x140afb513 jmp      0x140afb3f3 ; 
0x140afb518 mov      byte ptr [rsi + 1], 0x15
0x140afb51c xor      eax, eax
0x140afb51e jmp      0x140afb3f3 ; 
0x140afb523 mov      byte ptr [rsi + 1], 0xd
0x140afb527 xor      eax, eax
0x140afb529 jmp      0x140afb3f3 ; 
0x140afb52e mov      byte ptr [rsi + 1], 6
0x140afb532 xor      eax, eax
0x140afb534 jmp      0x140afb3f3 ; 
0x140afb539 mov      byte ptr [rsi + 1], 0x12
0x140afb53d xor      eax, eax
0x140afb53f jmp      0x140afb3f3 ; 
0x140afb544 mov      byte ptr [rsi + 1], 0xa
0x140afb548 xor      eax, eax
0x140afb54a jmp      0x140afb3f3 ; 
0x140afb54f mov      byte ptr [rsi + 1], 8
0x140afb553 xor      eax, eax
0x140afb555 jmp      0x140afb3f3 ; 
0x140afb55a mov      byte ptr [rsi + 1], 0x13
0x140afb55e xor      eax, eax
0x140afb560 jmp      0x140afb3f3 ; 
0x140afb565 mov      qword ptr [rsp + 0x20], 0x15
0x140afb56e jmp      0x140afb2df ; 
0x140afb573 mov      qword ptr [rsp + 0x20], 6
0x140afb57c jmp      0x140afb3b6 ; 
0x140afb581 mov      qword ptr [rsp + 0x20], 0xa
0x140afb58a jmp      0x140afb44f ; 
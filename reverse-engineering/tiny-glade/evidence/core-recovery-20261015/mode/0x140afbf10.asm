0x140afbf10 push     r14
0x140afbf12 push     rsi
0x140afbf13 push     rdi
0x140afbf14 push     rbx
0x140afbf15 sub      rsp, 0x38
0x140afbf19 mov      rsi, rcx
0x140afbf1c mov      rcx, qword ptr [rdx + 0x20]
0x140afbf20 mov      rax, qword ptr [rdx + 0x28]
0x140afbf24 cmp      rax, rcx
0x140afbf27 jae      0x140afbf66
0x140afbf29 mov      r8, qword ptr [rdx + 0x18]
0x140afbf2d inc      rax
0x140afbf30 neg      rcx
0x140afbf33 movabs   rdi, 0x100002600
0x140afbf3d nop      dword ptr [rax]
0x140afbf40 movzx    r9d, byte ptr [r8 + rax - 1]
0x140afbf46 cmp      r9, 0x22
0x140afbf4a ja       0x140afbfbc
0x140afbf4c bt       rdi, r9
0x140afbf50 jae      0x140afbf84
0x140afbf52 mov      qword ptr [rdx + 0x28], rax
0x140afbf56 lea      r9, [rcx + rax]
0x140afbf5a inc      r9
0x140afbf5d inc      rax
0x140afbf60 cmp      r9, 1
0x140afbf64 jne      0x140afbf40
0x140afbf66 mov      qword ptr [rsp + 0x20], 5
0x140afbf6f lea      rax, [rsp + 0x20]
0x140afbf74 mov      rcx, rdx
0x140afbf77 mov      rdx, rax
0x140afbf7a call     0x1428a0920 ; serde_json::de::Deserializer$LT$R$GT$::peek_error::hee6f49fa8612ef7c (.llvm.3546337791550519002) / serde_json::de::Deserializer$LT$R$GT$::peek_error::h3c4b8448a00c9d9a
0x140afbf7f jmp      0x140afc072 ; 
0x140afbf84 cmp      r9, 0x22
0x140afbf88 jne      0x140afbfbc
0x140afbf8a lea      rcx, [rsp + 0x20]
0x140afbf8f call     0x14140ce70 ; _$LT$core..marker..PhantomData$LT$T$GT$$u20$as$u20$serde..de..DeserializeSeed$GT$::deserialize::h85cd6a90969fd54a (.llvm.2285519678028410232)
0x140afbf94 cmp      byte ptr [rsp + 0x20], 1
0x140afbf99 je       0x140afbfe7
0x140afbf9b movzx    eax, byte ptr [rsp + 0x21]
0x140afbfa0 test     eax, eax
0x140afbfa2 je       0x140afc0e0
0x140afbfa8 cmp      eax, 1
0x140afbfab jne      0x140afc0e8
0x140afbfb1 mov      byte ptr [rsi + 1], 1
0x140afbfb5 xor      eax, eax
0x140afbfb7 jmp      0x140afc078 ; 
0x140afbfbc cmp      r9d, 0x7b
0x140afbfc0 jne      0x140afc104
0x140afbfc6 dec      byte ptr [rdx + 0x30]
0x140afbfc9 je       0x140afc112
0x140afbfcf mov      qword ptr [rdx + 0x28], rax
0x140afbfd3 lea      rcx, [rsp + 0x20]
0x140afbfd8 mov      r14, rdx
0x140afbfdb call     0x14140ce70 ; _$LT$core..marker..PhantomData$LT$T$GT$$u20$as$u20$serde..de..DeserializeSeed$GT$::deserialize::h85cd6a90969fd54a (.llvm.2285519678028410232)
0x140afbfe0 cmp      byte ptr [rsp + 0x20], 0
0x140afbfe5 je       0x140afbff1
0x140afbfe7 mov      rax, qword ptr [rsp + 0x28]
0x140afbfec jmp      0x140afc072 ; 
0x140afbff1 mov      rcx, r14
0x140afbff4 mov      r8, qword ptr [r14 + 0x20]
0x140afbff8 mov      rax, qword ptr [r14 + 0x28]
0x140afbffc cmp      rax, r8
0x140afbfff jae      0x140afc03d
0x140afc001 movzx    ebx, byte ptr [rsp + 0x21]
0x140afc006 movzx    edx, bl
0x140afc009 mov      r9, qword ptr [rcx + 0x18]
0x140afc00d inc      rax
0x140afc010 neg      r8
0x140afc013 movzx    r10d, byte ptr [r9 + rax - 1]
0x140afc019 cmp      r10, 0x3a
0x140afc01d ja       0x140afc120
0x140afc023 bt       rdi, r10
0x140afc027 jae      0x140afc052
0x140afc029 mov      qword ptr [rcx + 0x28], rax
0x140afc02d lea      r10, [r8 + rax]
0x140afc031 inc      r10
0x140afc034 inc      rax
0x140afc037 cmp      r10, 1
0x140afc03b jne      0x140afc013
0x140afc03d mov      qword ptr [rsp + 0x20], 3
0x140afc046 lea      rdx, [rsp + 0x20]
0x140afc04b call     0x1428a0920 ; serde_json::de::Deserializer$LT$R$GT$::peek_error::hee6f49fa8612ef7c (.llvm.3546337791550519002) / serde_json::de::Deserializer$LT$R$GT$::peek_error::h3c4b8448a00c9d9a
0x140afc050 jmp      0x140afc072 ; 
0x140afc052 cmp      r10, 0x3a
0x140afc056 jne      0x140afc120
0x140afc05c mov      qword ptr [rcx + 0x28], rax
0x140afc060 cmp      edx, 2
0x140afc063 je       0x140afc068
0x140afc065 cmp      edx, 1
0x140afc068 call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afc06d test     rax, rax
0x140afc070 je       0x140afc087
0x140afc072 mov      qword ptr [rsi + 8], rax
0x140afc076 mov      al, 1
0x140afc078 mov      byte ptr [rsi], al
0x140afc07a mov      rax, rsi
0x140afc07d add      rsp, 0x38
0x140afc081 pop      rbx
0x140afc082 pop      rdi
0x140afc083 pop      rsi
0x140afc084 pop      r14
0x140afc086 ret      
0x140afc087 mov      rcx, r14
0x140afc08a inc      byte ptr [r14 + 0x30]
0x140afc08e mov      rdx, qword ptr [r14 + 0x20]
0x140afc092 mov      rax, qword ptr [r14 + 0x28]
0x140afc096 cmp      rax, rdx
0x140afc099 jae      0x140afc0cb
0x140afc09b mov      r8, qword ptr [rcx + 0x18]
0x140afc09f inc      rax
0x140afc0a2 neg      rdx
0x140afc0a5 movzx    r9d, byte ptr [r8 + rax - 1]
0x140afc0ab cmp      r9, 0x20
0x140afc0af ja       0x140afc0f0
0x140afc0b1 bt       rdi, r9
0x140afc0b5 jae      0x140afc0f0
0x140afc0b7 mov      qword ptr [rcx + 0x28], rax
0x140afc0bb lea      r9, [rdx + rax]
0x140afc0bf inc      r9
0x140afc0c2 inc      rax
0x140afc0c5 cmp      r9, 1
0x140afc0c9 jne      0x140afc0a5
0x140afc0cb mov      qword ptr [rsp + 0x20], 3
0x140afc0d4 lea      rdx, [rsp + 0x20]
0x140afc0d9 call     0x1428a0fa0 ; serde_json::de::Deserializer$LT$R$GT$::error::h89ce4a093855ca44 / serde_json::de::Deserializer$LT$R$GT$::error::ha7ea65e1a49f0611
0x140afc0de jmp      0x140afc072 ; 
0x140afc0e0 mov      byte ptr [rsi + 1], 0
0x140afc0e4 xor      eax, eax
0x140afc0e6 jmp      0x140afc078 ; 
0x140afc0e8 mov      byte ptr [rsi + 1], 2
0x140afc0ec xor      eax, eax
0x140afc0ee jmp      0x140afc078 ; 
0x140afc0f0 cmp      r9d, 0x7d
0x140afc0f4 jne      0x140afc12e
0x140afc0f6 mov      qword ptr [rcx + 0x28], rax
0x140afc0fa mov      byte ptr [rsi + 1], bl
0x140afc0fd xor      eax, eax
0x140afc0ff jmp      0x140afc078 ; 
0x140afc104 mov      qword ptr [rsp + 0x20], 0xa
0x140afc10d jmp      0x140afbf6f ; 
0x140afc112 mov      qword ptr [rsp + 0x20], 0x15
0x140afc11b jmp      0x140afbf6f ; 
0x140afc120 mov      qword ptr [rsp + 0x20], 6
0x140afc129 jmp      0x140afc046 ; 
0x140afc12e mov      qword ptr [rsp + 0x20], 0xa
0x140afc137 jmp      0x140afc0d4 ; 
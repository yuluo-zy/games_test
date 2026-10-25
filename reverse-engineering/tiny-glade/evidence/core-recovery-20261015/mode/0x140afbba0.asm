0x140afbba0 push     r14
0x140afbba2 push     rsi
0x140afbba3 push     rdi
0x140afbba4 push     rbx
0x140afbba5 sub      rsp, 0x38
0x140afbba9 mov      rsi, rcx
0x140afbbac mov      rcx, qword ptr [rdx + 0x20]
0x140afbbb0 mov      rax, qword ptr [rdx + 0x28]
0x140afbbb4 cmp      rax, rcx
0x140afbbb7 jae      0x140afbbf6
0x140afbbb9 mov      r8, qword ptr [rdx + 0x18]
0x140afbbbd inc      rax
0x140afbbc0 neg      rcx
0x140afbbc3 movabs   rdi, 0x100002600
0x140afbbcd nop      dword ptr [rax]
0x140afbbd0 movzx    r9d, byte ptr [r8 + rax - 1]
0x140afbbd6 cmp      r9, 0x22
0x140afbbda ja       0x140afbc4b
0x140afbbdc bt       rdi, r9
0x140afbbe0 jae      0x140afbc14
0x140afbbe2 mov      qword ptr [rdx + 0x28], rax
0x140afbbe6 lea      r9, [rcx + rax]
0x140afbbea inc      r9
0x140afbbed inc      rax
0x140afbbf0 cmp      r9, 1
0x140afbbf4 jne      0x140afbbd0
0x140afbbf6 mov      qword ptr [rsp + 0x20], 5
0x140afbbff lea      rax, [rsp + 0x20]
0x140afbc04 mov      rcx, rdx
0x140afbc07 mov      rdx, rax
0x140afbc0a call     0x1428a0920 ; serde_json::de::Deserializer$LT$R$GT$::peek_error::hee6f49fa8612ef7c (.llvm.3546337791550519002) / serde_json::de::Deserializer$LT$R$GT$::peek_error::h3c4b8448a00c9d9a
0x140afbc0f jmp      0x140afbd0d ; 
0x140afbc14 cmp      r9, 0x22
0x140afbc18 jne      0x140afbc4b
0x140afbc1a lea      rcx, [rsp + 0x20]
0x140afbc1f call     0x14140d510 ; _$LT$core..marker..PhantomData$LT$T$GT$$u20$as$u20$serde..de..DeserializeSeed$GT$::deserialize::hba6bfcaba1738902 (.llvm.2285519678028410232)
0x140afbc24 cmp      byte ptr [rsp + 0x20], 1
0x140afbc29 je       0x140afbc76
0x140afbc2b movzx    eax, byte ptr [rsp + 0x21]
0x140afbc30 lea      rcx, [rip + 0x1fea4d9]
0x140afbc37 movsxd   rax, dword ptr [rcx + rax*4]
0x140afbc3b add      rax, rcx
0x140afbc3e jmp      rax
0x140afbc40 mov      byte ptr [rsi + 1], 0
0x140afbc44 xor      eax, eax
0x140afbc46 jmp      0x140afbd13 ; 
0x140afbc4b cmp      r9d, 0x7b
0x140afbc4f jne      0x140afbdce
0x140afbc55 dec      byte ptr [rdx + 0x30]
0x140afbc58 je       0x140afbddc
0x140afbc5e mov      qword ptr [rdx + 0x28], rax
0x140afbc62 lea      rcx, [rsp + 0x20]
0x140afbc67 mov      r14, rdx
0x140afbc6a call     0x14140d510 ; _$LT$core..marker..PhantomData$LT$T$GT$$u20$as$u20$serde..de..DeserializeSeed$GT$::deserialize::hba6bfcaba1738902 (.llvm.2285519678028410232)
0x140afbc6f cmp      byte ptr [rsp + 0x20], 0
0x140afbc74 je       0x140afbc80
0x140afbc76 mov      rax, qword ptr [rsp + 0x28]
0x140afbc7b jmp      0x140afbd0d ; 
0x140afbc80 mov      rcx, r14
0x140afbc83 mov      rdx, qword ptr [r14 + 0x20]
0x140afbc87 mov      rax, qword ptr [r14 + 0x28]
0x140afbc8b cmp      rax, rdx
0x140afbc8e jae      0x140afbccd
0x140afbc90 movzx    ebx, byte ptr [rsp + 0x21]
0x140afbc95 movzx    r8d, bl
0x140afbc99 mov      r9, qword ptr [rcx + 0x18]
0x140afbc9d inc      rax
0x140afbca0 neg      rdx
0x140afbca3 movzx    r10d, byte ptr [r9 + rax - 1]
0x140afbca9 cmp      r10, 0x3a
0x140afbcad ja       0x140afbdea
0x140afbcb3 bt       rdi, r10
0x140afbcb7 jae      0x140afbce2
0x140afbcb9 mov      qword ptr [rcx + 0x28], rax
0x140afbcbd lea      r10, [rdx + rax]
0x140afbcc1 inc      r10
0x140afbcc4 inc      rax
0x140afbcc7 cmp      r10, 1
0x140afbccb jne      0x140afbca3
0x140afbccd mov      qword ptr [rsp + 0x20], 3
0x140afbcd6 lea      rdx, [rsp + 0x20]
0x140afbcdb call     0x1428a0920 ; serde_json::de::Deserializer$LT$R$GT$::peek_error::hee6f49fa8612ef7c (.llvm.3546337791550519002) / serde_json::de::Deserializer$LT$R$GT$::peek_error::h3c4b8448a00c9d9a
0x140afbce0 jmp      0x140afbd0d ; 
0x140afbce2 cmp      r10, 0x3a
0x140afbce6 jne      0x140afbdea
0x140afbcec mov      qword ptr [rcx + 0x28], rax
0x140afbcf0 mov      eax, r8d
0x140afbcf3 lea      rdx, [rip + 0x1fea432]
0x140afbcfa movsxd   rax, dword ptr [rdx + rax*4]
0x140afbcfe add      rax, rdx
0x140afbd01 jmp      rax
0x140afbd03 call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afbd08 test     rax, rax
0x140afbd0b je       0x140afbd22
0x140afbd0d mov      qword ptr [rsi + 8], rax
0x140afbd11 mov      al, 1
0x140afbd13 mov      byte ptr [rsi], al
0x140afbd15 mov      rax, rsi
0x140afbd18 add      rsp, 0x38
0x140afbd1c pop      rbx
0x140afbd1d pop      rdi
0x140afbd1e pop      rsi
0x140afbd1f pop      r14
0x140afbd21 ret      
0x140afbd22 mov      rcx, r14
0x140afbd25 inc      byte ptr [r14 + 0x30]
0x140afbd29 mov      rdx, qword ptr [r14 + 0x20]
0x140afbd2d mov      rax, qword ptr [r14 + 0x28]
0x140afbd31 cmp      rax, rdx
0x140afbd34 jae      0x140afbd66
0x140afbd36 mov      r8, qword ptr [rcx + 0x18]
0x140afbd3a inc      rax
0x140afbd3d neg      rdx
0x140afbd40 movzx    r9d, byte ptr [r8 + rax - 1]
0x140afbd46 cmp      r9, 0x20
0x140afbd4a ja       0x140afbd7b
0x140afbd4c bt       rdi, r9
0x140afbd50 jae      0x140afbd7b
0x140afbd52 mov      qword ptr [rcx + 0x28], rax
0x140afbd56 lea      r9, [rdx + rax]
0x140afbd5a inc      r9
0x140afbd5d inc      rax
0x140afbd60 cmp      r9, 1
0x140afbd64 jne      0x140afbd40
0x140afbd66 mov      qword ptr [rsp + 0x20], 3
0x140afbd6f lea      rdx, [rsp + 0x20]
0x140afbd74 call     0x1428a0fa0 ; serde_json::de::Deserializer$LT$R$GT$::error::h89ce4a093855ca44 / serde_json::de::Deserializer$LT$R$GT$::error::ha7ea65e1a49f0611
0x140afbd79 jmp      0x140afbd0d ; 
0x140afbd7b cmp      r9d, 0x7d
0x140afbd7f jne      0x140afbdf8
0x140afbd81 mov      qword ptr [rcx + 0x28], rax
0x140afbd85 mov      byte ptr [rsi + 1], bl
0x140afbd88 xor      eax, eax
0x140afbd8a jmp      0x140afbd13 ; 
0x140afbd8c mov      byte ptr [rsi + 1], 4
0x140afbd90 xor      eax, eax
0x140afbd92 jmp      0x140afbd13 ; 
0x140afbd97 mov      byte ptr [rsi + 1], 2
0x140afbd9b xor      eax, eax
0x140afbd9d jmp      0x140afbd13 ; 
0x140afbda2 mov      byte ptr [rsi + 1], 3
0x140afbda6 xor      eax, eax
0x140afbda8 jmp      0x140afbd13 ; 
0x140afbdad mov      byte ptr [rsi + 1], 1
0x140afbdb1 xor      eax, eax
0x140afbdb3 jmp      0x140afbd13 ; 
0x140afbdb8 mov      byte ptr [rsi + 1], 5
0x140afbdbc xor      eax, eax
0x140afbdbe jmp      0x140afbd13 ; 
0x140afbdc3 mov      byte ptr [rsi + 1], 6
0x140afbdc7 xor      eax, eax
0x140afbdc9 jmp      0x140afbd13 ; 
0x140afbdce mov      qword ptr [rsp + 0x20], 0xa
0x140afbdd7 jmp      0x140afbbff ; 
0x140afbddc mov      qword ptr [rsp + 0x20], 0x15
0x140afbde5 jmp      0x140afbbff ; 
0x140afbdea mov      qword ptr [rsp + 0x20], 6
0x140afbdf3 jmp      0x140afbcd6 ; 
0x140afbdf8 mov      qword ptr [rsp + 0x20], 0xa
0x140afbe01 jmp      0x140afbd6f ; 
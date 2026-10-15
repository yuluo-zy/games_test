0x140afd8d0 push     rsi
0x140afd8d1 push     rdi
0x140afd8d2 push     rbx
0x140afd8d3 sub      rsp, 0x60
0x140afd8d7 mov      rsi, rcx
0x140afd8da mov      rcx, qword ptr [rdx + 0x20]
0x140afd8de mov      rax, qword ptr [rdx + 0x28]
0x140afd8e2 cmp      rax, rcx
0x140afd8e5 jae      0x140afd926
0x140afd8e7 mov      r8, qword ptr [rdx + 0x18]
0x140afd8eb inc      rax
0x140afd8ee neg      rcx
0x140afd8f1 movabs   rdi, 0x100002600
0x140afd8fb nop      dword ptr [rax + rax]
0x140afd900 movzx    r9d, byte ptr [r8 + rax - 1]
0x140afd906 cmp      r9, 0x22
0x140afd90a ja       0x140afd975
0x140afd90c bt       rdi, r9
0x140afd910 jae      0x140afd941
0x140afd912 mov      qword ptr [rdx + 0x28], rax
0x140afd916 lea      r9, [rcx + rax]
0x140afd91a inc      r9
0x140afd91d inc      rax
0x140afd920 cmp      r9, 1
0x140afd924 jne      0x140afd900
0x140afd926 mov      qword ptr [rsp + 0x28], 5
0x140afd92f lea      rax, [rsp + 0x28]
0x140afd934 mov      rcx, rdx
0x140afd937 mov      rdx, rax
0x140afd93a call     0x1428a0920 ; serde_json::de::Deserializer$LT$R$GT$::peek_error::hee6f49fa8612ef7c (.llvm.3546337791550519002) / serde_json::de::Deserializer$LT$R$GT$::peek_error::h3c4b8448a00c9d9a
0x140afd93f jmp      0x140afd9a5 ; 
0x140afd941 cmp      r9, 0x22
0x140afd945 jne      0x140afd975
0x140afd947 lea      rcx, [rsp + 0x28]
0x140afd94c call     0x14140d750 ; _$LT$core..marker..PhantomData$LT$T$GT$$u20$as$u20$serde..de..DeserializeSeed$GT$::deserialize::hd2e62bf5a9208243 (.llvm.2285519678028410232)
0x140afd951 cmp      byte ptr [rsp + 0x28], 1
0x140afd956 je       0x140afd9a0
0x140afd958 movzx    eax, byte ptr [rsp + 0x29]
0x140afd95d lea      rcx, [rip + 0x1fe88fc]
0x140afd964 movsxd   rax, dword ptr [rcx + rax*4]
0x140afd968 add      rax, rcx
0x140afd96b jmp      rax
0x140afd96d mov      dword ptr [rsi], 7
0x140afd973 jmp      0x140afd9af ; 
0x140afd975 cmp      r9d, 0x7b
0x140afd979 jne      0x140afda77
0x140afd97f dec      byte ptr [rdx + 0x30]
0x140afd982 je       0x140afddf8
0x140afd988 mov      qword ptr [rdx + 0x28], rax
0x140afd98c lea      rcx, [rsp + 0x28]
0x140afd991 mov      rbx, rdx
0x140afd994 call     0x14140d750 ; _$LT$core..marker..PhantomData$LT$T$GT$$u20$as$u20$serde..de..DeserializeSeed$GT$::deserialize::hd2e62bf5a9208243 (.llvm.2285519678028410232)
0x140afd999 cmp      byte ptr [rsp + 0x28], 0
0x140afd99e je       0x140afd9ba
0x140afd9a0 mov      rax, qword ptr [rsp + 0x30]
0x140afd9a5 mov      qword ptr [rsi + 8], rax
0x140afd9a9 mov      dword ptr [rsi], 0x15
0x140afd9af mov      rax, rsi
0x140afd9b2 add      rsp, 0x60
0x140afd9b6 pop      rbx
0x140afd9b7 pop      rdi
0x140afd9b8 pop      rsi
0x140afd9b9 ret      
0x140afd9ba mov      rax, rbx
0x140afd9bd mov      r8, qword ptr [rbx + 0x20]
0x140afd9c1 mov      rcx, qword ptr [rbx + 0x28]
0x140afd9c5 cmp      rcx, r8
0x140afd9c8 jae      0x140afda03
0x140afd9ca movzx    edx, byte ptr [rsp + 0x29]
0x140afd9cf mov      r9, qword ptr [rax + 0x18]
0x140afd9d3 inc      rcx
0x140afd9d6 neg      r8
0x140afd9d9 movzx    r10d, byte ptr [r9 + rcx - 1]
0x140afd9df cmp      r10, 0x3a
0x140afd9e3 ja       0x140afde06
0x140afd9e9 bt       rdi, r10
0x140afd9ed jae      0x140afda1b
0x140afd9ef mov      qword ptr [rax + 0x28], rcx
0x140afd9f3 lea      r10, [r8 + rcx]
0x140afd9f7 inc      r10
0x140afd9fa inc      rcx
0x140afd9fd cmp      r10, 1
0x140afda01 jne      0x140afd9d9
0x140afda03 mov      qword ptr [rsp + 0x28], 3
0x140afda0c lea      rdx, [rsp + 0x28]
0x140afda11 mov      rcx, rax
0x140afda14 call     0x1428a0920 ; serde_json::de::Deserializer$LT$R$GT$::peek_error::hee6f49fa8612ef7c (.llvm.3546337791550519002) / serde_json::de::Deserializer$LT$R$GT$::peek_error::h3c4b8448a00c9d9a
0x140afda19 jmp      0x140afd9a5 ; 
0x140afda1b cmp      r10, 0x3a
0x140afda1f jne      0x140afde06
0x140afda25 mov      qword ptr [rax + 0x28], rcx
0x140afda29 mov      ecx, edx
0x140afda2b lea      rdx, [rip + 0x1fe886e]
0x140afda32 movsxd   rcx, dword ptr [rdx + rcx*4]
0x140afda36 add      rcx, rdx
0x140afda39 jmp      rcx
0x140afda3b lea      rcx, [rsp + 0x28]
0x140afda40 mov      rdx, rax
0x140afda43 call     0x140afc140 ; country_core::systems::mode_manager::_::_$LT$impl$u20$serde..de..Deserialize$u20$for$u20$country_core..systems..mode_manager..EditSubject$GT$::deserialize::h32407d4af2c128d5
0x140afda48 cmp      dword ptr [rsp + 0x28], 6
0x140afda4d jne      0x140afdd27
0x140afda53 mov      rax, qword ptr [rsp + 0x30]
0x140afda58 mov      qword ptr [rsp + 0x48], rax
0x140afda5d mov      dword ptr [rsp + 0x40], 0x15
0x140afda65 mov      eax, dword ptr [rsp + 0x40]
0x140afda69 cmp      eax, 0x15
0x140afda6c je       0x140afdd3a
0x140afda72 jmp      0x140afdd5b ; 
0x140afda77 mov      qword ptr [rsp + 0x28], 0xa
0x140afda80 jmp      0x140afd92f ; 
0x140afda85 mov      dword ptr [rsi], 0x13
0x140afda8b jmp      0x140afd9af ; 
0x140afda90 mov      dword ptr [rsi], 0xf
0x140afda96 jmp      0x140afd9af ; 
0x140afda9b mov      dword ptr [rsi], 0x14
0x140afdaa1 jmp      0x140afd9af ; 
0x140afdaa6 mov      dword ptr [rsi], 0x10
0x140afdaac jmp      0x140afd9af ; 
0x140afdab1 mov      dword ptr [rsi], 0xc
0x140afdab7 jmp      0x140afd9af ; 
0x140afdabc mov      dword ptr [rsi], 0xa
0x140afdac2 jmp      0x140afd9af ; 
0x140afdac7 mov      dword ptr [rsi], 0xd
0x140afdacd jmp      0x140afd9af ; 
0x140afdad2 mov      dword ptr [rsi], 0x11
0x140afdad8 jmp      0x140afd9af ; 
0x140afdadd mov      dword ptr [rsi], 0x12
0x140afdae3 jmp      0x140afd9af ; 
0x140afdae8 mov      rcx, rax
0x140afdaeb call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afdaf0 test     rax, rax
0x140afdaf3 jne      0x140afd9a5
0x140afdaf9 mov      dword ptr [rsp + 0x40], 0x13
0x140afdb01 jmp      0x140afdd5b ; 
0x140afdb06 mov      rcx, rax
0x140afdb09 call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afdb0e test     rax, rax
0x140afdb11 jne      0x140afd9a5
0x140afdb17 mov      dword ptr [rsp + 0x40], 0x10
0x140afdb1f jmp      0x140afdd5b ; 
0x140afdb24 lea      rcx, [rsp + 0x28]
0x140afdb29 mov      rdx, rax
0x140afdb2c call     0x140afbba0 ; country_core::systems::mode_manager::_::_$LT$impl$u20$serde..de..Deserialize$u20$for$u20$country_core..systems..mode_manager..EraserLayer$GT$::deserialize::ha36a6bf96a25447a
0x140afdb31 cmp      byte ptr [rsp + 0x28], 0
0x140afdb36 jne      0x140afdcb2
0x140afdb3c movzx    eax, byte ptr [rsp + 0x29]
0x140afdb41 mov      byte ptr [rsp + 0x44], al
0x140afdb45 mov      eax, 9
0x140afdb4a mov      dword ptr [rsp + 0x40], eax
0x140afdb4e cmp      eax, 0x15
0x140afdb51 je       0x140afdd3a
0x140afdb57 jmp      0x140afdd5b ; 
0x140afdb5c mov      rcx, rax
0x140afdb5f call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afdb64 test     rax, rax
0x140afdb67 jne      0x140afd9a5
0x140afdb6d mov      dword ptr [rsp + 0x40], 0x11
0x140afdb75 jmp      0x140afdd5b ; 
0x140afdb7a lea      rcx, [rsp + 0x40]
0x140afdb7f mov      r8d, 8
0x140afdb85 mov      rdx, rax
0x140afdb88 xor      r9d, r9d
0x140afdb8b call     0x14140fae0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::struct_variant::hce3c0e40ef21c7db
0x140afdb90 mov      eax, dword ptr [rsp + 0x40]
0x140afdb94 cmp      eax, 0x15
0x140afdb97 je       0x140afdd3a
0x140afdb9d jmp      0x140afdd5b ; 
0x140afdba2 mov      rcx, rax
0x140afdba5 call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afdbaa test     rax, rax
0x140afdbad jne      0x140afd9a5
0x140afdbb3 mov      dword ptr [rsp + 0x40], 7
0x140afdbbb jmp      0x140afdd5b ; 
0x140afdbc0 mov      rcx, rax
0x140afdbc3 call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afdbc8 test     rax, rax
0x140afdbcb jne      0x140afd9a5
0x140afdbd1 mov      dword ptr [rsp + 0x40], 0x14
0x140afdbd9 jmp      0x140afdd5b ; 
0x140afdbde mov      rcx, rax
0x140afdbe1 call     0x141412f50 ; _$LT$$RF$mut$u20$serde_json..de..Deserializer$LT$R$GT$$u20$as$u20$serde..de..Deserializer$GT$::deserialize_u64::hdab8276a6ceab783
0x140afdbe6 test     al, 1
0x140afdbe8 mov      ecx, 0x15
0x140afdbed mov      eax, 8
0x140afdbf2 cmovne   eax, ecx
0x140afdbf5 mov      qword ptr [rsp + 0x48], rdx
0x140afdbfa mov      dword ptr [rsp + 0x40], eax
0x140afdbfe cmp      eax, 0x15
0x140afdc01 je       0x140afdd3a
0x140afdc07 jmp      0x140afdd5b ; 
0x140afdc0c mov      rcx, rax
0x140afdc0f call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afdc14 test     rax, rax
0x140afdc17 jne      0x140afd9a5
0x140afdc1d mov      dword ptr [rsp + 0x40], 0xc
0x140afdc25 jmp      0x140afdd5b ; 
0x140afdc2a lea      rcx, [rsp + 0x28]
0x140afdc2f mov      rdx, rax
0x140afdc32 call     0x140afbf10 ; country_core::systems::mode_manager::_::_$LT$impl$u20$serde..de..Deserialize$u20$for$u20$country_core..systems..mode_manager..WallShape$GT$::deserialize::hd8a4391c2200619c
0x140afdc37 cmp      byte ptr [rsp + 0x28], 0
0x140afdc3c jne      0x140afdcb2
0x140afdc3e movzx    eax, byte ptr [rsp + 0x29]
0x140afdc43 mov      byte ptr [rsp + 0x44], al
0x140afdc47 mov      eax, 6
0x140afdc4c mov      dword ptr [rsp + 0x40], eax
0x140afdc50 cmp      eax, 0x15
0x140afdc53 je       0x140afdd3a
0x140afdc59 jmp      0x140afdd5b ; 
0x140afdc5e mov      rcx, rax
0x140afdc61 call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afdc66 test     rax, rax
0x140afdc69 jne      0x140afd9a5
0x140afdc6f mov      dword ptr [rsp + 0x40], 0xa
0x140afdc77 jmp      0x140afdd5b ; 
0x140afdc7c mov      rcx, rax
0x140afdc7f call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afdc84 test     rax, rax
0x140afdc87 jne      0x140afd9a5
0x140afdc8d mov      dword ptr [rsp + 0x40], 0x12
0x140afdc95 jmp      0x140afdd5b ; 
0x140afdc9a lea      rcx, [rsp + 0x28]
0x140afdc9f mov      rdx, rax
0x140afdca2 call     0x140a47da0 ; country_core::resources::walls::decorator::_::_$LT$impl$u20$serde..de..Deserialize$u20$for$u20$country_core..resources..walls..decorator..DecoratorType$GT$::deserialize::hcc4d59760429076f
0x140afdca7 cmp      byte ptr [rsp + 0x28], 0
0x140afdcac je       0x140afdd44
0x140afdcb2 mov      rax, qword ptr [rsp + 0x30]
0x140afdcb7 mov      qword ptr [rsp + 0x48], rax
0x140afdcbc mov      eax, 0x15
0x140afdcc1 mov      dword ptr [rsp + 0x40], eax
0x140afdcc5 cmp      eax, 0x15
0x140afdcc8 je       0x140afdd3a
0x140afdcca jmp      0x140afdd5b ; 
0x140afdccf mov      rcx, rax
0x140afdcd2 call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afdcd7 test     rax, rax
0x140afdcda jne      0x140afd9a5
0x140afdce0 mov      dword ptr [rsp + 0x40], 0xf
0x140afdce8 jmp      0x140afdd5b ; 
0x140afdcea mov      rcx, rax
0x140afdced call     0x14140d9a0 ; _$LT$serde_json..de..VariantAccess$LT$R$GT$$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h100b17baf7f461da
0x140afdcf2 test     rax, rax
0x140afdcf5 jne      0x140afd9a5
0x140afdcfb mov      dword ptr [rsp + 0x40], 0xd
0x140afdd03 jmp      0x140afdd5b ; 
0x140afdd05 mov      byte ptr [rsp + 0x28], 0xd
0x140afdd0a lea      rdx, [rip + 0x215e20f]
0x140afdd11 lea      r8, [rip + 0x215e1d8]
0x140afdd18 lea      rcx, [rsp + 0x28]
0x140afdd1d call     0x1428c9a00 ; _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type::h76868c415965851f
0x140afdd22 jmp      0x140afd9a5 ; 
0x140afdd27 movups   xmm0, xmmword ptr [rsp + 0x28]
0x140afdd2c movaps   xmmword ptr [rsp + 0x40], xmm0
0x140afdd31 mov      eax, dword ptr [rsp + 0x40]
0x140afdd35 cmp      eax, 0x15
0x140afdd38 jne      0x140afdd5b
0x140afdd3a mov      rax, qword ptr [rsp + 0x48]
0x140afdd3f jmp      0x140afd9a5 ; 
0x140afdd44 movzx    eax, byte ptr [rsp + 0x29]
0x140afdd49 mov      byte ptr [rsp + 0x44], al
0x140afdd4d mov      eax, 0xb
0x140afdd52 mov      dword ptr [rsp + 0x40], eax
0x140afdd56 cmp      eax, 0x15
0x140afdd59 je       0x140afdd3a
0x140afdd5b mov      eax, dword ptr [rsp + 0x40]
0x140afdd5f mov      dword ptr [rsp + 0x50], eax
0x140afdd63 mov      rax, qword ptr [rsp + 0x44]
0x140afdd68 mov      qword ptr [rsp + 0x54], rax
0x140afdd6d mov      eax, dword ptr [rsp + 0x4c]
0x140afdd71 mov      rcx, rbx
0x140afdd74 inc      byte ptr [rbx + 0x30]
0x140afdd77 mov      dword ptr [rsp + 0x5c], eax
0x140afdd7b mov      rdx, qword ptr [rbx + 0x20]
0x140afdd7f mov      rax, qword ptr [rbx + 0x28]
0x140afdd83 cmp      rax, rdx
0x140afdd86 jae      0x140afddb8
0x140afdd88 mov      r8, qword ptr [rcx + 0x18]
0x140afdd8c inc      rax
0x140afdd8f neg      rdx
0x140afdd92 movzx    r9d, byte ptr [r8 + rax - 1]
0x140afdd98 cmp      r9, 0x20
0x140afdd9c ja       0x140afddd0
0x140afdd9e bt       rdi, r9
0x140afdda2 jae      0x140afddd0
0x140afdda4 mov      qword ptr [rcx + 0x28], rax
0x140afdda8 lea      r9, [rdx + rax]
0x140afddac inc      r9
0x140afddaf inc      rax
0x140afddb2 cmp      r9, 1
0x140afddb6 jne      0x140afdd92
0x140afddb8 mov      qword ptr [rsp + 0x28], 3
0x140afddc1 lea      rdx, [rsp + 0x28]
0x140afddc6 call     0x1428a0fa0 ; serde_json::de::Deserializer$LT$R$GT$::error::h89ce4a093855ca44 / serde_json::de::Deserializer$LT$R$GT$::error::ha7ea65e1a49f0611
0x140afddcb jmp      0x140afd9a5 ; 
0x140afddd0 cmp      r9d, 0x7d
0x140afddd4 jne      0x140afde14
0x140afddd6 mov      qword ptr [rcx + 0x28], rax
0x140afddda movaps   xmm0, xmmword ptr [rsp + 0x50]
0x140afdddf movups   xmmword ptr [rsi], xmm0
0x140afdde2 jmp      0x140afd9af ; 
0x140afdde7 mov      byte ptr [rsp + 0x28], 0xd
0x140afddec lea      rdx, [rip + 0x215e0ed]
0x140afddf3 jmp      0x140afdd11 ; 
0x140afddf8 mov      qword ptr [rsp + 0x28], 0x15
0x140afde01 jmp      0x140afd92f ; 
0x140afde06 mov      qword ptr [rsp + 0x28], 6
0x140afde0f jmp      0x140afda0c ; 
0x140afde14 mov      qword ptr [rsp + 0x28], 0xa
0x140afde1d jmp      0x140afddc1 ; 
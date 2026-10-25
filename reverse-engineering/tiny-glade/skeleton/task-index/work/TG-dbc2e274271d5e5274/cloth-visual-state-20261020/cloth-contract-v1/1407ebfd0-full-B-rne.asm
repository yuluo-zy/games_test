1407ebfd0: 55 push rbp
1407ebfd1: 4157 push r15
1407ebfd3: 4156 push r14
1407ebfd5: 4155 push r13
1407ebfd7: 4154 push r12
1407ebfd9: 56 push rsi
1407ebfda: 57 push rdi
1407ebfdb: 53 push rbx
1407ebfdc: 4881ece8030000 sub rsp, 0x3e8
1407ebfe3: 488dac2480000000 lea rbp, [rsp + 0x80]
1407ebfeb: 440f29bd50030000 movaps xmmword ptr [rbp + 0x350], xmm15
1407ebff3: 440f29b540030000 movaps xmmword ptr [rbp + 0x340], xmm14
1407ebffb: 440f29ad30030000 movaps xmmword ptr [rbp + 0x330], xmm13
1407ec003: 440f29a520030000 movaps xmmword ptr [rbp + 0x320], xmm12
1407ec00b: 440f299d10030000 movaps xmmword ptr [rbp + 0x310], xmm11
1407ec013: 440f299500030000 movaps xmmword ptr [rbp + 0x300], xmm10
1407ec01b: 440f298df0020000 movaps xmmword ptr [rbp + 0x2f0], xmm9
1407ec023: 440f2985e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm8
1407ec02b: 0f29bdd0020000 movaps xmmword ptr [rbp + 0x2d0], xmm7
1407ec032: 0f29b5c0020000 movaps xmmword ptr [rbp + 0x2c0], xmm6
1407ec039: 48c785b8020000feffffff mov qword ptr [rbp + 0x2b8], 0xfffffffffffffffe
1407ec044: 4c89cf mov rdi, r9
1407ec047: 4d89c7 mov r15, r8
1407ec04a: 4889d3 mov rbx, rdx
1407ec04d: 4989ce mov r14, rcx
1407ec050: e8aba94200 call 0x140c16a00 ; puffin::are_scopes_on
1407ec055: 8885a7020000 mov byte ptr [rbp + 0x2a7], al
1407ec05b: 84c0 test al, al
1407ec05d: 0f843a020000 je 0x1407ec29d
1407ec063: 41bc56000000 mov r12d, 0x56
1407ec069: b956000000 mov ecx, 0x56
1407ec06e: e8fdf9ffff call 0x1407eba70 ; core::str::impl$0::rfind<ref$<str$> >
1407ec073: 488d35cee22902 lea rsi, [rip + 0x229e2ce]
1407ec07a: 4883f801 cmp rax, 1
1407ec07e: 0f85a1000000 jne 0x1407ec125
1407ec084: 4989d1 mov r9, rdx
1407ec087: 4885d2 test rdx, rdx
1407ec08a: 7436 je 0x1407ec0c2
1407ec08c: 4983f956 cmp r9, 0x56
1407ec090: 732e jae 0x1407ec0c0
1407ec092: 488d05afe22902 lea rax, [rip + 0x229e2af]
1407ec099: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
1407ec09e: 7f22 jg 0x1407ec0c2
1407ec0a0: 488d0571e32902 lea rax, [rip + 0x229e371]
1407ec0a7: 4889442420 mov qword ptr [rsp + 0x20], rax
1407ec0ac: 488d0d95e22902 lea rcx, [rip + 0x229e295]
1407ec0b3: ba56000000 mov edx, 0x56
1407ec0b8: 4531c0 xor r8d, r8d
1407ec0bb: e880d80e02 call 0x1428d9940 ; core::str::slice_error_fail
1407ec0c0: 75de jne 0x1407ec0a0
1407ec0c2: 4c89c9 mov rcx, r9
1407ec0c5: e8a6f9ffff call 0x1407eba70 ; core::str::impl$0::rfind<ref$<str$> >
1407ec0ca: 4883f801 cmp rax, 1
1407ec0ce: 7555 jne 0x1407ec125
1407ec0d0: 4989d0 mov r8, rdx
1407ec0d3: 4983c002 add r8, 2
1407ec0d7: 7439 je 0x1407ec112
1407ec0d9: 4983f856 cmp r8, 0x56
1407ec0dd: 7331 jae 0x1407ec110
1407ec0df: 488d0562e22902 lea rax, [rip + 0x229e262]
1407ec0e6: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
1407ec0eb: 7f25 jg 0x1407ec112
1407ec0ed: 488d053ce32902 lea rax, [rip + 0x229e33c]
1407ec0f4: 4889442420 mov qword ptr [rsp + 0x20], rax
1407ec0f9: 488d0d48e22902 lea rcx, [rip + 0x229e248]
1407ec100: ba56000000 mov edx, 0x56
1407ec105: 41b956000000 mov r9d, 0x56
1407ec10b: e830d80e02 call 0x1428d9940 ; core::str::slice_error_fail
1407ec110: 75db jne 0x1407ec0ed
1407ec112: 41bc54000000 mov r12d, 0x54
1407ec118: 4929d4 sub r12, rdx
1407ec11b: 488d3526e22902 lea rsi, [rip + 0x229e226]
1407ec122: 4c01c6 add rsi, r8
1407ec125: 4c8d0546e42902 lea r8, [rip + 0x229e446]
1407ec12c: 4c8d2dfbe32902 lea r13, [rip + 0x229e3fb]
1407ec133: 4c89b5a8020000 mov qword ptr [rbp + 0x2a8], r14
1407ec13a: 660f1f440000 nop word ptr [rax + rax]
1407ec140: 410fbe40ff movsx eax, byte ptr [r8 - 1]
1407ec145: 85c0 test eax, eax
1407ec147: 7817 js 0x1407ec160
1407ec149: 49ffc8 dec r8
1407ec14c: 83f85c cmp eax, 0x5c
1407ec14f: 7563 jne 0x1407ec1b4
1407ec151: eb7c jmp 0x1407ec1cf ; unknown
1407ec153: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407ec160: 410fb648fe movzx ecx, byte ptr [r8 - 2]
1407ec165: 80f9c0 cmp cl, 0xc0
1407ec168: 7d23 jge 0x1407ec18d
1407ec16a: 410fb650fd movzx edx, byte ptr [r8 - 3]
1407ec16f: 80fac0 cmp dl, 0xc0
1407ec172: 7d22 jge 0x1407ec196
1407ec174: 450fb648fc movzx r9d, byte ptr [r8 - 4]
1407ec179: 4983c0fc add r8, -4
1407ec17d: 4183e107 and r9d, 7
1407ec181: 41c1e106 shl r9d, 6
1407ec185: 83e23f and edx, 0x3f
1407ec188: 4409ca or edx, r9d
1407ec18b: eb10 jmp 0x1407ec19d ; unknown
1407ec18d: 4983c0fe add r8, -2
1407ec191: 83e11f and ecx, 0x1f
1407ec194: eb0f jmp 0x1407ec1a5 ; unknown
1407ec196: 4983c0fd add r8, -3
1407ec19a: 83e20f and edx, 0xf
1407ec19d: c1e206 shl edx, 6
1407ec1a0: 83e13f and ecx, 0x3f
1407ec1a3: 09d1 or ecx, edx
1407ec1a5: c1e106 shl ecx, 6
1407ec1a8: 243f and al, 0x3f
1407ec1aa: 0fb6c0 movzx eax, al
1407ec1ad: 09c8 or eax, ecx
1407ec1af: 83f85c cmp eax, 0x5c
1407ec1b2: 741b je 0x1407ec1cf
1407ec1b4: 83f82f cmp eax, 0x2f
1407ec1b7: 7416 je 0x1407ec1cf
1407ec1b9: 4d39e8 cmp r8, r13
1407ec1bc: 7582 jne 0x1407ec140
1407ec1be: 4989de mov r14, rbx
1407ec1c1: 4889fb mov rbx, rdi
1407ec1c4: 4c89ff mov rdi, r15
1407ec1c7: 41bf44000000 mov r15d, 0x44
1407ec1cd: eb56 jmp 0x1407ec225 ; unknown
1407ec1cf: 4c8d2d58e32902 lea r13, [rip + 0x229e358]
1407ec1d6: 4d29e8 sub r8, r13
1407ec1d9: 49ffc0 inc r8
1407ec1dc: 7432 je 0x1407ec210
1407ec1de: 4983f844 cmp r8, 0x44
1407ec1e2: 732a jae 0x1407ec20e
1407ec1e4: 43803c28bf cmp byte ptr [r8 + r13], 0xbf
1407ec1e9: 7f25 jg 0x1407ec210
1407ec1eb: 488d0506e22902 lea rax, [rip + 0x229e206]
1407ec1f2: 4889442420 mov qword ptr [rsp + 0x20], rax
1407ec1f7: 488d0d30e32902 lea rcx, [rip + 0x229e330]
1407ec1fe: ba44000000 mov edx, 0x44
1407ec203: 41b944000000 mov r9d, 0x44
1407ec209: e832d70e02 call 0x1428d9940 ; core::str::slice_error_fail
1407ec20e: 75db jne 0x1407ec1eb
1407ec210: 4989de mov r14, rbx
1407ec213: 4889fb mov rbx, rdi
1407ec216: 4c89ff mov rdi, r15
1407ec219: 41bf44000000 mov r15d, 0x44
1407ec21f: 4d29c7 sub r15, r8
1407ec222: 4d01c5 add r13, r8
1407ec225: e806bf4200 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1407ec22a: 4889c1 mov rcx, rax
1407ec22d: 488b00 mov rax, qword ptr [rax]
1407ec230: 4883f801 cmp rax, 1
1407ec234: 0f851b2a0000 jne 0x1407eec55
1407ec23a: 4883c108 add rcx, 8
1407ec23e: 48833900 cmp qword ptr [rcx], 0
1407ec242: 0f85682a0000 jne 0x1407eecb0
1407ec248: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1407ec24f: 48898d78020000 mov qword ptr [rbp + 0x278], rcx
1407ec256: 4883c108 add rcx, 8
1407ec25a: 4c897c2420 mov qword ptr [rsp + 0x20], r15
1407ec25f: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
1407ec268: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
1407ec271: 4889f2 mov rdx, rsi
1407ec274: 4d89e0 mov r8, r12
1407ec277: 4d89e9 mov r9, r13
1407ec27a: e8c1aa4200 call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
1407ec27f: 48894558 mov qword ptr [rbp + 0x58], rax
1407ec283: 488b8578020000 mov rax, qword ptr [rbp + 0x278]
1407ec28a: 48ff00 inc qword ptr [rax]
1407ec28d: 4989ff mov r15, rdi
1407ec290: 4889df mov rdi, rbx
1407ec293: 4c89f3 mov rbx, r14
1407ec296: 4c8bb5a8020000 mov r14, qword ptr [rbp + 0x2a8]
1407ec29d: 4c89f1 mov rcx, r14
1407ec2a0: e8cba73300 call 0x140b26a70 ; country_core::resources::walls::decorator_subtype::impl$8::from
1407ec2a5: 488985e0000000 mov qword ptr [rbp + 0xe0], rax
1407ec2ac: 488995e8000000 mov qword ptr [rbp + 0xe8], rdx
1407ec2b3: 488b13 mov rdx, qword ptr [rbx]
1407ec2b6: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ec2bd: e83eee2200 call 0x140a1b100 ; country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids | country_core::resources::walls::decorator_storage::DecoratorStorage::iter_walls | country_core::resources::tree_storage::TreeStorage::iter
1407ec2c2: 488b8dd8030000 mov rcx, qword ptr [rbp + 0x3d8]
1407ec2c9: 488b95d0030000 mov rdx, qword ptr [rbp + 0x3d0]
1407ec2d0: 4c8bad10020000 mov r13, qword ptr [rbp + 0x210]
1407ec2d7: 4c8b8518020000 mov r8, qword ptr [rbp + 0x218]
1407ec2de: 0fb78528020000 movzx eax, word ptr [rbp + 0x228]
1407ec2e5: 4c8b8d30020000 mov r9, qword ptr [rbp + 0x230]
1407ec2ec: 4c8b17 mov r10, qword ptr [rdi]
1407ec2ef: 4c895518 mov qword ptr [rbp + 0x18], r10
1407ec2f3: 4983c220 add r10, 0x20
1407ec2f7: 4c895520 mov qword ptr [rbp + 0x20], r10
1407ec2fb: 448b521c mov r10d, dword ptr [rdx + 0x1c]
1407ec2ff: 44899594000000 mov dword ptr [rbp + 0x94], r10d
1407ec306: 4c8b12 mov r10, qword ptr [rdx]
1407ec309: 4c895530 mov qword ptr [rbp + 0x30], r10
1407ec30d: 488b5210 mov rdx, qword ptr [rdx + 0x10]
1407ec311: 48895528 mov qword ptr [rbp + 0x28], rdx
1407ec315: 498b17 mov rdx, qword ptr [r15]
1407ec318: 48895538 mov qword ptr [rbp + 0x38], rdx
1407ec31c: 488b09 mov rcx, qword ptr [rcx]
1407ec31f: 48894d50 mov qword ptr [rbp + 0x50], rcx
1407ec323: 440f282dd5db2902 movaps xmm13, xmmword ptr [rip + 0x229dbd5]
1407ec32b: 440f283deddb2902 movaps xmm15, xmmword ptr [rip + 0x229dbed]
1407ec333: 4c8985f0000000 mov qword ptr [rbp + 0xf0], r8
1407ec33a: 4c898db8000000 mov qword ptr [rbp + 0xb8], r9
1407ec341: 4d85c9 test r9, r9
1407ec344: 0f843e270000 je 0x1407eea88
1407ec34a: 6685c0 test ax, ax
1407ec34d: 7421 je 0x1407ec370
1407ec34f: 4d85ed test r13, r13
1407ec352: 0f8430270000 je 0x1407eea88
1407ec358: 8d48ff lea ecx, [rax - 1]
1407ec35b: 21c1 and ecx, eax
1407ec35d: 898d8c010000 mov dword ptr [rbp + 0x18c], ecx
1407ec363: eb4f jmp 0x1407ec3b4 ; unknown
1407ec365: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407ec370: 488b8df0000000 mov rcx, qword ptr [rbp + 0xf0]
1407ec377: 660f1f840000000000 nop word ptr [rax + rax]
1407ec380: 660f6f01 movdqa xmm0, xmmword ptr [rcx]
1407ec384: 660fd7c0 pmovmskb eax, xmm0
1407ec388: 4981c580f6ffff add r13, -0x980
1407ec38f: 4883c110 add rcx, 0x10
1407ec393: 3dffff0000 cmp eax, 0xffff
1407ec398: 74e6 je 0x1407ec380
1407ec39a: 48898df0000000 mov qword ptr [rbp + 0xf0], rcx
1407ec3a1: b9feffffff mov ecx, 0xfffffffe
1407ec3a6: 29c1 sub ecx, eax
1407ec3a8: f7d0 not eax
1407ec3aa: 89c2 mov edx, eax
1407ec3ac: 21ca and edx, ecx
1407ec3ae: 89958c010000 mov dword ptr [rbp + 0x18c], edx
1407ec3b4: f30fbcc0 tzcnt eax, eax
1407ec3b8: 48f7d8 neg rax
1407ec3bb: 4869c098000000 imul rax, rax, 0x98
1407ec3c2: 4a8d1428 lea rdx, [rax + r13]
1407ec3c6: 4881c270ffffff add rdx, -0x90
1407ec3cd: 498b840568ffffff mov rax, qword ptr [r13 + rax - 0x98]
1407ec3d5: 488985f0010000 mov qword ptr [rbp + 0x1f0], rax
1407ec3dc: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ec3e3: e888ec2200 call 0x140a1b070 ; country_core::resources::walls::decorator_storage::Decorators::iter_modified
1407ec3e8: 48ff8db8000000 dec qword ptr [rbp + 0xb8]
1407ec3ef: 4c8ba510020000 mov r12, qword ptr [rbp + 0x210]
1407ec3f6: 4c8bbd18020000 mov r15, qword ptr [rbp + 0x218]
1407ec3fd: 440fb7b528020000 movzx r14d, word ptr [rbp + 0x228]
1407ec405: 488b8530020000 mov rax, qword ptr [rbp + 0x230]
1407ec40c: 48894540 mov qword ptr [rbp + 0x40], rax
1407ec410: 488b8538020000 mov rax, qword ptr [rbp + 0x238]
1407ec417: 48894548 mov qword ptr [rbp + 0x48], rax
1407ec41b: 4883c050 add rax, 0x50
1407ec41f: 48894510 mov qword ptr [rbp + 0x10], rax
1407ec423: 4c89adb0000000 mov qword ptr [rbp + 0xb0], r13
1407ec42a: 48837d4000 cmp qword ptr [rbp + 0x40], 0
1407ec42f: 0f8428260000 je 0x1407eea5d
1407ec435: 664585f6 test r14w, r14w
1407ec439: 7415 je 0x1407ec450
1407ec43b: 418d46ff lea eax, [r14 - 1]
1407ec43f: 4421f0 and eax, r14d
1407ec442: 4d85e4 test r12, r12
1407ec445: 7534 jne 0x1407ec47b
1407ec447: e911260000 jmp 0x1407eea5d ; unknown
1407ec44c: 0f1f4000 nop dword ptr [rax]
1407ec450: 66410f6f07 movdqa xmm0, xmmword ptr [r15]
1407ec455: 66440fd7f0 pmovmskb r14d, xmm0
1407ec45a: 4983c4c0 add r12, -0x40
1407ec45e: 4983c710 add r15, 0x10
1407ec462: 4181feffff0000 cmp r14d, 0xffff
1407ec469: 74e5 je 0x1407ec450
1407ec46b: b9feffffff mov ecx, 0xfffffffe
1407ec470: 4429f1 sub ecx, r14d
1407ec473: 41f7d6 not r14d
1407ec476: 4489f0 mov eax, r14d
1407ec479: 21c8 and eax, ecx
1407ec47b: 4489f1 mov ecx, r14d
1407ec47e: f30fbcc9 tzcnt ecx, ecx
1407ec482: 4189c6 mov r14d, eax
1407ec485: c1e102 shl ecx, 2
1407ec488: 4c89e0 mov rax, r12
1407ec48b: 4829c8 sub rax, rcx
1407ec48e: 8b48fc mov ecx, dword ptr [rax - 4]
1407ec491: 898d10020000 mov dword ptr [rbp + 0x210], ecx
1407ec497: 488b4548 mov rax, qword ptr [rbp + 0x48]
1407ec49b: 4883784800 cmp qword ptr [rax + 0x48], 0
1407ec4a0: 0f8498260000 je 0x1407eeb3e
1407ec4a6: 48898d78020000 mov qword ptr [rbp + 0x278], rcx
1407ec4ad: 488b4d10 mov rcx, qword ptr [rbp + 0x10]
1407ec4b1: 488d9510020000 lea rdx, [rbp + 0x210]
1407ec4b8: e8134adcff call 0x1405b0ed0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<shader_pipeline::reflection::BindingIndex> > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::resources::walls::d
1407ec4bd: 48ff4d40 dec qword ptr [rbp + 0x40]
1407ec4c1: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
1407ec4c5: 488b7130 mov rsi, qword ptr [rcx + 0x30]
1407ec4c9: 488b4938 mov rcx, qword ptr [rcx + 0x38]
1407ec4cd: 4989cb mov r11, rcx
1407ec4d0: 4921c3 and r11, rax
1407ec4d3: 48c1e839 shr rax, 0x39
1407ec4d7: 660f6ec0 movd xmm0, eax
1407ec4db: 660f60c0 punpcklbw xmm0, xmm0
1407ec4df: f20f70c000 pshuflw xmm0, xmm0, 0
1407ec4e4: 660f70c000 pshufd xmm0, xmm0, 0
1407ec4e9: 488d46b0 lea rax, [rsi - 0x50]
1407ec4ed: 4531c0 xor r8d, r8d
1407ec4f0: 488b9578020000 mov rdx, qword ptr [rbp + 0x278]
1407ec4f7: f3420f6f0c1e movdqu xmm1, xmmword ptr [rsi + r11]
1407ec4fd: 660f6fd1 movdqa xmm2, xmm1
1407ec501: 660f74d0 pcmpeqb xmm2, xmm0
1407ec505: 66440fd7ca pmovmskb r9d, xmm2
1407ec50a: 4585c9 test r9d, r9d
1407ec50d: 7431 je 0x1407ec540
1407ec50f: f3450fbcd1 tzcnt r10d, r9d
1407ec514: 4d01da add r10, r11
1407ec517: 4921ca and r10, rcx
1407ec51a: 49f7da neg r10
1407ec51d: 4b8d3c92 lea rdi, [r10 + r10*4]
1407ec521: 48c1e704 shl rdi, 4
1407ec525: 3b1438 cmp edx, dword ptr [rax + rdi]
1407ec528: 7446 je 0x1407ec570
1407ec52a: 458d51ff lea r10d, [r9 - 1]
1407ec52e: 664521ca and r10w, r9w
1407ec532: 4589d1 mov r9d, r10d
1407ec535: 75d8 jne 0x1407ec50f
1407ec537: 660f1f840000000000 nop word ptr [rax + rax]
1407ec540: 660f740d48941302 pcmpeqb xmm1, xmmword ptr [rip + 0x2139448]
1407ec548: 66440fd7c9 pmovmskb r9d, xmm1
1407ec54d: 4585c9 test r9d, r9d
1407ec550: 0f85e8250000 jne 0x1407eeb3e
1407ec556: 4d01c3 add r11, r8
1407ec559: 4983c310 add r11, 0x10
1407ec55d: 4983c010 add r8, 0x10
1407ec561: 4921cb and r11, rcx
1407ec564: eb91 jmp 0x1407ec4f7 ; unknown
1407ec566: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407ec570: 488d8de0000000 lea rcx, [rbp + 0xe0]
1407ec577: e8f4a33300 call 0x140b26970 ; country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
1407ec57c: 888580020000 mov byte ptr [rbp + 0x280], al
1407ec582: 0fb68580020000 movzx eax, byte ptr [rbp + 0x280]
1407ec589: 88858f020000 mov byte ptr [rbp + 0x28f], al
1407ec58f: 3c01 cmp al, 1
1407ec591: 0f8793feffff ja 0x1407ec42a
1407ec597: 488b5d18 mov rbx, qword ptr [rbp + 0x18]
1407ec59b: 48837b1800 cmp qword ptr [rbx + 0x18], 0
1407ec5a0: 0f84a9250000 je 0x1407eeb4f
1407ec5a6: 488d043e lea rax, [rsi + rdi]
1407ec5aa: 48898598020000 mov qword ptr [rbp + 0x298], rax
1407ec5b1: 488d043e lea rax, [rsi + rdi]
1407ec5b5: 4883c0b8 add rax, -0x48
1407ec5b9: 48898570020000 mov qword ptr [rbp + 0x270], rax
1407ec5c0: 488b4d20 mov rcx, qword ptr [rbp + 0x20]
1407ec5c4: 488d95f0010000 lea rdx, [rbp + 0x1f0]
1407ec5cb: e8a047dcff call 0x1405b0d70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> > 
1407ec5d0: 4c8b2b mov r13, qword ptr [rbx]
1407ec5d3: 488b4b08 mov rcx, qword ptr [rbx + 8]
1407ec5d7: 4889ca mov rdx, rcx
1407ec5da: 4821c2 and rdx, rax
1407ec5dd: 48c1e839 shr rax, 0x39
1407ec5e1: 660f6ec0 movd xmm0, eax
1407ec5e5: 660f60c0 punpcklbw xmm0, xmm0
1407ec5e9: f20f70c000 pshuflw xmm0, xmm0, 0
1407ec5ee: 660f70c000 pshufd xmm0, xmm0, 0
1407ec5f3: 498d8558fdffff lea rax, [r13 - 0x2a8]
1407ec5fa: 4c8b85f0010000 mov r8, qword ptr [rbp + 0x1f0]
1407ec601: 4531c9 xor r9d, r9d
1407ec604: f3410f6f4c1500 movdqu xmm1, xmmword ptr [r13 + rdx]
1407ec60b: 660f6fd1 movdqa xmm2, xmm1
1407ec60f: 660f74d0 pcmpeqb xmm2, xmm0
1407ec613: 66440fd7d2 pmovmskb r10d, xmm2
1407ec618: 4585d2 test r10d, r10d
1407ec61b: 7428 je 0x1407ec645
1407ec61d: f3450fbcda tzcnt r11d, r10d
1407ec622: 4901d3 add r11, rdx
1407ec625: 4921cb and r11, rcx
1407ec628: 49f7db neg r11
1407ec62b: 4969fba8020000 imul rdi, r11, 0x2a8
1407ec632: 4c3b0438 cmp r8, qword ptr [rax + rdi]
1407ec636: 7433 je 0x1407ec66b
1407ec638: 458d5aff lea r11d, [r10 - 1]
1407ec63c: 664521d3 and r11w, r10w
1407ec640: 4589da mov r10d, r11d
1407ec643: 75d8 jne 0x1407ec61d
1407ec645: 660f740d43931302 pcmpeqb xmm1, xmmword ptr [rip + 0x2139343]
1407ec64d: 66440fd7d1 pmovmskb r10d, xmm1
1407ec652: 4585d2 test r10d, r10d
1407ec655: 0f85f4240000 jne 0x1407eeb4f
1407ec65b: 4c01ca add rdx, r9
1407ec65e: 4883c210 add rdx, 0x10
1407ec662: 4983c110 add r9, 0x10
1407ec666: 4821ca and rdx, rcx
1407ec669: eb99 jmp 0x1407ec604 ; unknown
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
1407ec6d2: 8b8594000000 mov eax, dword ptr [rbp + 0x94]
1407ec6d8: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
1407ec6dc: 8901 mov dword ptr [rcx], eax
1407ec6de: 488b8df0010000 mov rcx, qword ptr [rbp + 0x1f0]
1407ec6e5: e836fe0c00 call 0x1408bc520 ; country_core::resources::rt::impl$5::from | country_core::resources::walls::decorator_storage::impl$15::from
1407ec6ea: 4989d1 mov r9, rdx
1407ec6ed: 488b8d98020000 mov rcx, qword ptr [rbp + 0x298]
1407ec6f4: 0fb649ff movzx ecx, byte ptr [rcx - 1]
1407ec6f8: 884c2420 mov byte ptr [rsp + 0x20], cl
1407ec6fc: 488b4d30 mov rcx, qword ptr [rbp + 0x30]
1407ec700: 488b9578020000 mov rdx, qword ptr [rbp + 0x278]
1407ec707: 4989c0 mov r8, rax
1407ec70a: e8a1910300 call 0x1408258b0 ; system_decorator::blueprint::DecoratorBlueprints::insert
1407ec70f: 488985b8010000 mov qword ptr [rbp + 0x1b8], rax
1407ec716: 488b9598020000 mov rdx, qword ptr [rbp + 0x298]
1407ec71d: 0fb642fc movzx eax, byte ptr [rdx - 4]
1407ec721: fec8 dec al
1407ec723: 3c03 cmp al, 3
1407ec725: 0f8348260000 jae 0x1407eed73
1407ec72b: 4e8d042f lea r8, [rdi + r13]
1407ec72f: 4a8d0c2f lea rcx, [rdi + r13]
1407ec733: 4881c160fdffff add rcx, -0x2a0
1407ec73a: 48898d68020000 mov qword ptr [rbp + 0x268], rcx
1407ec741: 0fb64aff movzx ecx, byte ptr [rdx - 1]
1407ec745: 888df8010000 mov byte ptr [rbp + 0x1f8], cl
1407ec74b: 8b4ae0 mov ecx, dword ptr [rdx - 0x20]
1407ec74e: 898d90010000 mov dword ptr [rbp + 0x190], ecx
1407ec754: 0fb6c0 movzx eax, al
1407ec757: 488d0ddae62902 lea rcx, [rip + 0x229e6da]
1407ec75e: 48633c81 movsxd rdi, dword ptr [rcx + rax*4]
1407ec762: 4c898500020000 mov qword ptr [rbp + 0x200], r8
1407ec769: 418378b801 cmp dword ptr [r8 - 0x48], 1
1407ec76e: 4c8b8578020000 mov r8, qword ptr [rbp + 0x278]
1407ec775: 0f85a9030000 jne 0x1407ecb24
1407ec77b: 488d8d10010000 lea rcx, [rbp + 0x110]
1407ec782: 488b5538 mov rdx, qword ptr [rbp + 0x38]
1407ec786: 4c8d0d5bde2902 lea r9, [rip + 0x229de5b]
1407ec78d: e88ee22200 call 0x140a1aa20 ; country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
1407ec792: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407ec799: 8378b802 cmp dword ptr [rax - 0x48], 2
1407ec79d: 0f83a9000000 jae 0x1407ec84c
1407ec7a3: 0f288510010000 movaps xmm0, xmmword ptr [rbp + 0x110]
1407ec7aa: 0f28d0 movaps xmm2, xmm0
1407ec7ad: 0f59d0 mulps xmm2, xmm0
1407ec7b0: f30f16ca movshdup xmm1, xmm2
1407ec7b4: f30f58ca addss xmm1, xmm2
1407ec7b8: 0f28da movaps xmm3, xmm2
1407ec7bb: 660f15da unpckhpd xmm3, xmm2
1407ec7bf: f30f58d9 addss xmm3, xmm1
1407ec7c3: 0fc6db00 shufps xmm3, xmm3, 0
1407ec7c7: 0fc6d2ff shufps xmm2, xmm2, 0xff
1407ec7cb: 0f5cd3 subps xmm2, xmm3
1407ec7ce: 0f281ddb8e2902 movaps xmm3, xmmword ptr [rip + 0x2298edb]
1407ec7d5: 0f59d3 mulps xmm2, xmm3
1407ec7d8: 0f28c8 movaps xmm1, xmm0
1407ec7db: 0f59cb mulps xmm1, xmm3
1407ec7de: f30f16d9 movshdup xmm3, xmm1
1407ec7e2: f30f58d9 addss xmm3, xmm1
1407ec7e6: 0f12c9 movhlps xmm1, xmm1
1407ec7e9: f30f58cb addss xmm1, xmm3
1407ec7ed: f30f58c9 addss xmm1, xmm1
1407ec7f1: 0fc6c900 shufps xmm1, xmm1, 0
1407ec7f5: 0f59c8 mulps xmm1, xmm0
1407ec7f8: 0f58ca addps xmm1, xmm2
1407ec7fb: 0f28d0 movaps xmm2, xmm0
1407ec7fe: f30f10253a452902 movss xmm4, dword ptr [rip + 0x229453a]
1407ec806: 0f59d4 mulps xmm2, xmm4
1407ec809: 0f28d8 movaps xmm3, xmm0
1407ec80c: 0fc6d8c9 shufps xmm3, xmm0, 0xc9
1407ec810: 0f59dc mulps xmm3, xmm4
1407ec813: 0fc6d2d2 shufps xmm2, xmm2, 0xd2
1407ec817: 0f5cda subps xmm3, xmm2
1407ec81a: 0f58c0 addps xmm0, xmm0
1407ec81d: 0fc6c0ff shufps xmm0, xmm0, 0xff
1407ec821: 0f59c3 mulps xmm0, xmm3
1407ec824: 0f58c1 addps xmm0, xmm1
1407ec827: f20f108d20010000 movsd xmm1, qword ptr [rbp + 0x120]
1407ec82f: 0f58c8 addps xmm1, xmm0
1407ec832: 0f138d20010000 movlps qword ptr [rbp + 0x120], xmm1
1407ec839: 0f12c0 movhlps xmm0, xmm0
1407ec83c: f30f588528010000 addss xmm0, dword ptr [rbp + 0x128]
1407ec844: f30f118528010000 movss dword ptr [rbp + 0x128], xmm0
1407ec84c: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ec853: 8a58fc mov bl, byte ptr [rax - 4]
1407ec856: 488b95e0000000 mov rdx, qword ptr [rbp + 0xe0]
1407ec85d: 0fb68d80020000 movzx ecx, byte ptr [rbp + 0x280]
1407ec864: e8e7690400 call 0x140833250 ; country_core::resources::walls::decorator::DecoratorRank::new | country_core::systems::collision::world::impl$61::from | country_core::render::nani::mesh_atlas::MeshAtlasPart::new | country_core::LazyTile::from_index | c
1407ec869: 488bb5e8000000 mov rsi, qword ptr [rbp + 0xe8]
1407ec870: 89d9 mov ecx, ebx
1407ec872: 89c2 mov edx, eax
1407ec874: e817943300 call 0x140b25c90 ; country_core::resources::walls::decorator_subtype::DecoratorLibKey::new | rhapsody::rg::recorder::PassResourceAccessType::new
1407ec879: 888598010000 mov byte ptr [rbp + 0x198], al
1407ec87f: 889599010000 mov byte ptr [rbp + 0x199], dl
1407ec885: 48837e1800 cmp qword ptr [rsi + 0x18], 0
1407ec88a: 0f84dc220000 je 0x1407eeb6c
1407ec890: 488d4e20 lea rcx, [rsi + 0x20]
1407ec894: 488d9598010000 lea rdx, [rbp + 0x198]
1407ec89b: e820b5a000 call 0x1411f7dc0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey> > > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$
1407ec8a0: 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
1407ec8a7: 48898da8020000 mov qword ptr [rbp + 0x2a8], rcx
1407ec8ae: 488d0d83e52902 lea rcx, [rip + 0x229e583]
1407ec8b5: 4801cf add rdi, rcx
1407ec8b8: 488b0e mov rcx, qword ptr [rsi]
1407ec8bb: 488b5608 mov rdx, qword ptr [rsi + 8]
1407ec8bf: 4989d0 mov r8, rdx
1407ec8c2: 4921c0 and r8, rax
1407ec8c5: 48c1e839 shr rax, 0x39
1407ec8c9: 660f6ec0 movd xmm0, eax
1407ec8cd: 660f60c0 punpcklbw xmm0, xmm0
1407ec8d1: f20f70c000 pshuflw xmm0, xmm0, 0
1407ec8d6: 660f70c000 pshufd xmm0, xmm0, 0
1407ec8db: 488d41b8 lea rax, [rcx - 0x48]
1407ec8df: 440fb68d98010000 movzx r9d, byte ptr [rbp + 0x198]
1407ec8e7: 440fb69599010000 movzx r10d, byte ptr [rbp + 0x199]
1407ec8ef: 4531db xor r11d, r11d
1407ec8f2: f3420f6f0c01 movdqu xmm1, xmmword ptr [rcx + r8]
1407ec8f8: 660f6fd1 movdqa xmm2, xmm1
1407ec8fc: 660f74d0 pcmpeqb xmm2, xmm0
1407ec900: 660fd7f2 pmovmskb esi, xmm2
1407ec904: 85f6 test esi, esi
1407ec906: 7473 je 0x1407ec97b
1407ec908: 4180f91a cmp r9b, 0x1a
1407ec90c: 4889bd90020000 mov qword ptr [rbp + 0x290], rdi
1407ec913: 752a jne 0x1407ec93f
1407ec915: f30fbcfe tzcnt edi, esi
1407ec919: 4c01c7 add rdi, r8
1407ec91c: 4821d7 and rdi, rdx
1407ec91f: 48f7df neg rdi
1407ec922: 488d1cff lea rbx, [rdi + rdi*8]
1407ec926: 803cd81a cmp byte ptr [rax + rbx*8], 0x1a
1407ec92a: 7476 je 0x1407ec9a2
1407ec92c: 8d7eff lea edi, [rsi - 1]
1407ec92f: 6621f7 and di, si
1407ec932: 89fe mov esi, edi
1407ec934: 488bbd90020000 mov rdi, qword ptr [rbp + 0x290]
1407ec93b: 75d8 jne 0x1407ec915
1407ec93d: eb3c jmp 0x1407ec97b ; unknown
1407ec93f: f30fbcfe tzcnt edi, esi
1407ec943: 4c01c7 add rdi, r8
1407ec946: 4821d7 and rdi, rdx
1407ec949: 48f7df neg rdi
1407ec94c: 488d1cff lea rbx, [rdi + rdi*8]
1407ec950: 440fb62cd8 movzx r13d, byte ptr [rax + rbx*8]
1407ec955: 4180fd1a cmp r13b, 0x1a
1407ec959: 740f je 0x1407ec96a
1407ec95b: 4538e9 cmp r9b, r13b
1407ec95e: 750a jne 0x1407ec96a
1407ec960: 488d1cd8 lea rbx, [rax + rbx*8]
1407ec964: 443a5301 cmp r10b, byte ptr [rbx + 1]
1407ec968: 7438 je 0x1407ec9a2
1407ec96a: 8d7eff lea edi, [rsi - 1]
1407ec96d: 6621f7 and di, si
1407ec970: 89fe mov esi, edi
1407ec972: 488bbd90020000 mov rdi, qword ptr [rbp + 0x290]
1407ec979: 75c4 jne 0x1407ec93f
1407ec97b: 660f740d0d901302 pcmpeqb xmm1, xmmword ptr [rip + 0x213900d]
1407ec983: 660fd7f1 pmovmskb esi, xmm1
1407ec987: 85f6 test esi, esi
1407ec989: 0f85dd210000 jne 0x1407eeb6c
1407ec98f: 4d01d8 add r8, r11
1407ec992: 4983c010 add r8, 0x10
1407ec996: 4983c310 add r11, 0x10
1407ec99a: 4921d0 and r8, rdx
1407ec99d: e950ffffff jmp 0x1407ec8f2 ; unknown
1407ec9a2: 488d04ff lea rax, [rdi + rdi*8]
1407ec9a6: 488b7cc1d8 mov rdi, qword ptr [rcx + rax*8 - 0x28]
1407ec9ab: f048ff07 lock inc qword ptr [rdi]
1407ec9af: 0f8ec2240000 jle 0x1407eee77
1407ec9b5: 4889bd98010000 mov qword ptr [rbp + 0x198], rdi
1407ec9bc: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ec9c3: 408a70fc mov sil, byte ptr [rax - 4]
1407ec9c7: 488b95e0000000 mov rdx, qword ptr [rbp + 0xe0]
1407ec9ce: c685b702000001 mov byte ptr [rbp + 0x2b7], 1
1407ec9d5: 0fb68d80020000 movzx ecx, byte ptr [rbp + 0x280]
1407ec9dc: e86f680400 call 0x140833250 ; country_core::resources::walls::decorator::DecoratorRank::new | country_core::systems::collision::world::impl$61::from | country_core::render::nani::mesh_atlas::MeshAtlasPart::new | country_core::LazyTile::from_index | c
1407ec9e1: 488b9de8000000 mov rbx, qword ptr [rbp + 0xe8]
1407ec9e8: c685b702000001 mov byte ptr [rbp + 0x2b7], 1
1407ec9ef: 89f1 mov ecx, esi
1407ec9f1: 89c2 mov edx, eax
1407ec9f3: e898923300 call 0x140b25c90 ; country_core::resources::walls::decorator_subtype::DecoratorLibKey::new | rhapsody::rg::recorder::PassResourceAccessType::new
1407ec9f8: 888550010000 mov byte ptr [rbp + 0x150], al
1407ec9fe: 889551010000 mov byte ptr [rbp + 0x151], dl
1407eca04: 48837b4800 cmp qword ptr [rbx + 0x48], 0
1407eca09: 0f84ce210000 je 0x1407eebdd
1407eca0f: 488d4b50 lea rcx, [rbx + 0x50]
1407eca13: c685b702000001 mov byte ptr [rbp + 0x2b7], 1
1407eca1a: 488d9550010000 lea rdx, [rbp + 0x150]
1407eca21: e89ab3a000 call 0x1411f7dc0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey> > > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$
1407eca26: 488b4b30 mov rcx, qword ptr [rbx + 0x30]
1407eca2a: 488b5338 mov rdx, qword ptr [rbx + 0x38]
1407eca2e: 4989d0 mov r8, rdx
1407eca31: 4921c0 and r8, rax
1407eca34: 48c1e839 shr rax, 0x39
1407eca38: 660f6ec0 movd xmm0, eax
1407eca3c: 660f60c0 punpcklbw xmm0, xmm0
1407eca40: f20f70c000 pshuflw xmm0, xmm0, 0
1407eca45: 660f70c000 pshufd xmm0, xmm0, 0
1407eca4a: 488d41b8 lea rax, [rcx - 0x48]
1407eca4e: 440fb68d50010000 movzx r9d, byte ptr [rbp + 0x150]
1407eca56: 440fb69551010000 movzx r10d, byte ptr [rbp + 0x151]
1407eca5e: 31db xor ebx, ebx
1407eca60: f3420f6f0c01 movdqu xmm1, xmmword ptr [rcx + r8]
1407eca66: 660f6fd1 movdqa xmm2, xmm1
1407eca6a: 660f74d0 pcmpeqb xmm2, xmm0
1407eca6e: 660fd7f2 pmovmskb esi, xmm2
1407eca72: 85f6 test esi, esi
1407eca74: 0f8481000000 je 0x1407ecafb
1407eca7a: 4180f91a cmp r9b, 0x1a
1407eca7e: 7531 jne 0x1407ecab1
1407eca80: 4989dd mov r13, rbx
1407eca83: f30fbcde tzcnt ebx, esi
1407eca87: 4c01c3 add rbx, r8
1407eca8a: 4821d3 and rbx, rdx
1407eca8d: 48f7db neg rbx
1407eca90: 4c8d1cdb lea r11, [rbx + rbx*8]
1407eca94: 42803cd81a cmp byte ptr [rax + r11*8], 0x1a
1407eca99: 0f848c020000 je 0x1407ecd2b
1407eca9f: 448d5eff lea r11d, [rsi - 1]
1407ecaa3: 664121f3 and r11w, si
1407ecaa7: 4489de mov esi, r11d
1407ecaaa: 4c89eb mov rbx, r13
1407ecaad: 75d1 jne 0x1407eca80
1407ecaaf: eb4a jmp 0x1407ecafb ; unknown
1407ecab1: 48899d08020000 mov qword ptr [rbp + 0x208], rbx
1407ecab8: f30fbcde tzcnt ebx, esi
1407ecabc: 4c01c3 add rbx, r8
1407ecabf: 4821d3 and rbx, rdx
1407ecac2: 48f7db neg rbx
1407ecac5: 4c8d2cdb lea r13, [rbx + rbx*8]
1407ecac9: 460fb61ce8 movzx r11d, byte ptr [rax + r13*8]
1407ecace: 4180fb1a cmp r11b, 0x1a
1407ecad2: 7413 je 0x1407ecae7
1407ecad4: 4538d9 cmp r9b, r11b
1407ecad7: 750e jne 0x1407ecae7
1407ecad9: 4e8d1ce8 lea r11, [rax + r13*8]
1407ecadd: 453a5301 cmp r10b, byte ptr [r11 + 1]
1407ecae1: 0f8444020000 je 0x1407ecd2b
1407ecae7: 448d5eff lea r11d, [rsi - 1]
1407ecaeb: 664121f3 and r11w, si
1407ecaef: 4489de mov esi, r11d
1407ecaf2: 488b9d08020000 mov rbx, qword ptr [rbp + 0x208]
1407ecaf9: 75bd jne 0x1407ecab8
1407ecafb: 660f740d8d8e1302 pcmpeqb xmm1, xmmword ptr [rip + 0x2138e8d]
1407ecb03: 66440fd7d9 pmovmskb r11d, xmm1
1407ecb08: 4585db test r11d, r11d
1407ecb0b: 0f85cc200000 jne 0x1407eebdd
1407ecb11: 4901d8 add r8, rbx
1407ecb14: 4983c010 add r8, 0x10
1407ecb18: 4883c310 add rbx, 0x10
1407ecb1c: 4921d0 and r8, rdx
1407ecb1f: e93cffffff jmp 0x1407eca60 ; unknown
1407ecb24: 807ada02 cmp byte ptr [rdx - 0x26], 2
1407ecb28: 0f8362220000 jae 0x1407eed90
1407ecb2e: 488b8570020000 mov rax, qword ptr [rbp + 0x270]
1407ecb35: f20f1038 movsd xmm7, qword ptr [rax]
1407ecb39: f20f11bd00010000 movsd qword ptr [rbp + 0x100], xmm7
1407ecb41: f30f10b5d0000000 movss xmm6, dword ptr [rbp + 0xd0]
1407ecb49: 488b8d68020000 mov rcx, qword ptr [rbp + 0x268]
1407ecb50: e8ebdb3300 call 0x140b2a740 ; country_core::resources::walls::public_wall_state::PublicWallState::wall_space
1407ecb55: f30f59358bd51602 mulss xmm6, dword ptr [rip + 0x216d58b]
1407ecb5d: 898510020000 mov dword ptr [rbp + 0x210], eax
1407ecb63: f30f118514020000 movss dword ptr [rbp + 0x214], xmm0
1407ecb6b: f30f58b500010000 addss xmm6, dword ptr [rbp + 0x100]
1407ecb73: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ecb7a: 0f28ce movaps xmm1, xmm6
1407ecb7d: e82e534a00 call 0x140c91eb0 ; utils::wall_space::impl$1::project_clamp
1407ecb82: f30f118500010000 movss dword ptr [rbp + 0x100], xmm0
1407ecb8a: 0f13bd78010000 movlps qword ptr [rbp + 0x178], xmm7
1407ecb91: f30f10b5d0000000 movss xmm6, dword ptr [rbp + 0xd0]
1407ecb99: 488b8d68020000 mov rcx, qword ptr [rbp + 0x268]
1407ecba0: e89bdb3300 call 0x140b2a740 ; country_core::resources::walls::public_wall_state::PublicWallState::wall_space
1407ecba5: f30f5935c38c1302 mulss xmm6, dword ptr [rip + 0x2138cc3]
1407ecbad: 898510020000 mov dword ptr [rbp + 0x210], eax
1407ecbb3: f30f118514020000 movss dword ptr [rbp + 0x214], xmm0
1407ecbbb: f30f58b578010000 addss xmm6, dword ptr [rbp + 0x178]
1407ecbc3: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ecbca: 0f28ce movaps xmm1, xmm6
1407ecbcd: e8de524a00 call 0x140c91eb0 ; utils::wall_space::impl$1::project_clamp
1407ecbd2: f30f118578010000 movss dword ptr [rbp + 0x178], xmm0
1407ecbda: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ecbe1: 488d9500010000 lea rdx, [rbp + 0x100]
1407ecbe8: 4c8b8568020000 mov r8, qword ptr [rbp + 0x268]
1407ecbef: e89c544a00 call 0x140c92090 ; utils::wall_space::WallSpaceCoord::to_world_space
1407ecbf4: f3440f108d10020000 movss xmm9, dword ptr [rbp + 0x210]
1407ecbfd: f3440f108518020000 movss xmm8, dword ptr [rbp + 0x218]
1407ecc06: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ecc0d: 488d9578010000 lea rdx, [rbp + 0x178]
1407ecc14: 4c8b8568020000 mov r8, qword ptr [rbp + 0x268]
1407ecc1b: e870544a00 call 0x140c92090 ; utils::wall_space::WallSpaceCoord::to_world_space
1407ecc20: f3440f109510020000 movss xmm10, dword ptr [rbp + 0x210]
1407ecc29: f30f10b518020000 movss xmm6, dword ptr [rbp + 0x218]
1407ecc31: 410f28c1 movaps xmm0, xmm9
1407ecc35: 410f14c0 unpcklps xmm0, xmm8
1407ecc39: 0f2815a08c1302 movaps xmm2, xmmword ptr [rip + 0x2138ca0]
1407ecc40: 0f59c2 mulps xmm0, xmm2
1407ecc43: 410f28ca movaps xmm1, xmm10
1407ecc47: 0f14ce unpcklps xmm1, xmm6
1407ecc4a: 0f59ca mulps xmm1, xmm2
1407ecc4d: 0f58c8 addps xmm1, xmm0
1407ecc50: 0f138d98000000 movlps qword ptr [rbp + 0x98], xmm1
1407ecc57: f30f16d7 movshdup xmm2, xmm7
1407ecc5b: 488d8d98010000 lea rcx, [rbp + 0x198]
1407ecc62: 488d9598000000 lea rdx, [rbp + 0x98]
1407ecc69: e872014b00 call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
1407ecc6e: 410f14f2 unpcklps xmm6, xmm10
1407ecc72: 450f14c1 unpcklps xmm8, xmm9
1407ecc76: 410f5cf0 subps xmm6, xmm8
1407ecc7a: 0f28c6 movaps xmm0, xmm6
1407ecc7d: 0f59c6 mulps xmm0, xmm6
1407ecc80: f30f16c8 movshdup xmm1, xmm0
1407ecc84: f30f58c8 addss xmm1, xmm0
1407ecc88: 0f57c0 xorps xmm0, xmm0
1407ecc8b: f30f51c1 sqrtss xmm0, xmm1
1407ecc8f: f30f100d6d8c1302 movss xmm1, dword ptr [rip + 0x2138c6d]
1407ecc97: f30f5ec8 divss xmm1, xmm0
1407ecc9b: 0f28c1 movaps xmm0, xmm1
1407ecc9e: 0f57054b8c1302 xorps xmm0, xmmword ptr [rip + 0x2138c4b]
1407ecca5: 0f14c1 unpcklps xmm0, xmm1
1407ecca8: 0f59c6 mulps xmm0, xmm6
1407eccab: 0f138550010000 movlps qword ptr [rbp + 0x150], xmm0
1407eccb2: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407eccb9: 488d9550010000 lea rdx, [rbp + 0x150]
1407eccc0: e8fb004b00 call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
1407eccc5: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ecccc: 488d95c0010000 lea rdx, [rbp + 0x1c0]
1407eccd3: e8f8444a00 call 0x140c911d0 ; utils::xz_projective_rotation_bad
1407eccd8: 0f288510020000 movaps xmm0, xmmword ptr [rbp + 0x210]
1407eccdf: 8b85a0010000 mov eax, dword ptr [rbp + 0x1a0]
1407ecce5: 488d8d20010000 lea rcx, [rbp + 0x120]
1407eccec: 894108 mov dword ptr [rcx + 8], eax
1407eccef: 488b8598010000 mov rax, qword ptr [rbp + 0x198]
1407eccf6: 488901 mov qword ptr [rcx], rax
1407eccf9: 0f298510010000 movaps xmmword ptr [rbp + 0x110], xmm0
1407ecd00: 48b80000803f0000803f movabs rax, 0x3f8000003f800000
1407ecd0a: 4889410c mov qword ptr [rcx + 0xc], rax
1407ecd0e: c741140000803f mov dword ptr [rcx + 0x14], 0x3f800000
1407ecd15: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407ecd1c: 8378b802 cmp dword ptr [rax - 0x48], 2
1407ecd20: 0f827dfaffff jb 0x1407ec7a3
1407ecd26: e921fbffff jmp 0x1407ec84c ; unknown
1407ecd2b: 488d04db lea rax, [rbx + rbx*8]
1407ecd2f: 4c8b44c1d8 mov r8, qword ptr [rcx + rax*8 - 0x28]
1407ecd34: f049ff00 lock inc qword ptr [r8]
1407ecd38: 0f8e39210000 jle 0x1407eee77
1407ecd3e: 4c8985c0010000 mov qword ptr [rbp + 0x1c0], r8
1407ecd45: f20f108520010000 movsd xmm0, qword ptr [rbp + 0x120]
1407ecd4d: 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
1407ecd51: f30f108528010000 movss xmm0, dword ptr [rbp + 0x128]
1407ecd59: f30f118564020000 movss dword ptr [rbp + 0x264], xmm0
1407ecd61: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407ecd68: 8378b801 cmp dword ptr [rax - 0x48], 1
1407ecd6c: 0f57c0 xorps xmm0, xmm0
1407ecd6f: 440f28a510010000 movaps xmm12, xmmword ptr [rbp + 0x110]
1407ecd77: 4c8badb0000000 mov r13, qword ptr [rbp + 0xb0]
1407ecd7e: 4c898508020000 mov qword ptr [rbp + 0x208], r8
1407ecd85: 0f87ba000000 ja 0x1407ece45
1407ecd8b: c685a602000001 mov byte ptr [rbp + 0x2a6], 1
1407ecd92: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ecd99: 488d95c0000000 lea rdx, [rbp + 0xc0]
1407ecda0: e8bbb84a00 call 0x140c98660 ; utils::geometry::rectangle::Rectangle3d::extent
1407ecda5: f30f108d18020000 movss xmm1, dword ptr [rbp + 0x218]
1407ecdad: 0f28c1 movaps xmm0, xmm1
1407ecdb0: f30f5905d8381702 mulss xmm0, dword ptr [rip + 0x21738d8]
1407ecdb8: 0fc6c140 shufps xmm0, xmm1, 0x40
1407ecdbc: 0f5905edc32902 mulps xmm0, xmmword ptr [rip + 0x229c3ed]
1407ecdc3: 410f28d4 movaps xmm2, xmm12
1407ecdc7: 410f59d4 mulps xmm2, xmm12
1407ecdcb: f30f16ca movshdup xmm1, xmm2
1407ecdcf: f30f58ca addss xmm1, xmm2
1407ecdd3: 0f28da movaps xmm3, xmm2
1407ecdd6: 660f15da unpckhpd xmm3, xmm2
1407ecdda: f30f58d9 addss xmm3, xmm1
1407ecdde: 0fc6db00 shufps xmm3, xmm3, 0
1407ecde2: 0fc6d2ff shufps xmm2, xmm2, 0xff
1407ecde6: 0f5cd3 subps xmm2, xmm3
1407ecde9: 0f59d0 mulps xmm2, xmm0
1407ecdec: 410f28cc movaps xmm1, xmm12
1407ecdf0: 0f59c8 mulps xmm1, xmm0
1407ecdf3: f30f16d9 movshdup xmm3, xmm1
1407ecdf7: f30f58d9 addss xmm3, xmm1
1407ecdfb: 0f12c9 movhlps xmm1, xmm1
1407ecdfe: f30f58cb addss xmm1, xmm3
1407ece02: f30f58c9 addss xmm1, xmm1
1407ece06: 0fc6c900 shufps xmm1, xmm1, 0
1407ece0a: 410f59cc mulps xmm1, xmm12
1407ece0e: 0f58ca addps xmm1, xmm2
1407ece11: 410f28d4 movaps xmm2, xmm12
1407ece15: 410fc6d4d2 shufps xmm2, xmm12, 0xd2
1407ece1a: 0f59d0 mulps xmm2, xmm0
1407ece1d: 0fc6c0d2 shufps xmm0, xmm0, 0xd2
1407ece21: 410f59c4 mulps xmm0, xmm12
1407ece25: 0f5cd0 subps xmm2, xmm0
1407ece28: 0fc6d2d2 shufps xmm2, xmm2, 0xd2
1407ece2c: 410f28c4 movaps xmm0, xmm12
1407ece30: 410f58c4 addps xmm0, xmm12
1407ece34: 0fc6c0ff shufps xmm0, xmm0, 0xff
1407ece38: 0f59c2 mulps xmm0, xmm2
1407ece3b: 0f58c1 addps xmm0, xmm1
1407ece3e: 4c8b8508020000 mov r8, qword ptr [rbp + 0x208]
1407ece45: 0f284d60 movaps xmm1, xmmword ptr [rbp + 0x60]
1407ece49: 0f58c8 addps xmm1, xmm0
1407ece4c: 0f12c0 movhlps xmm0, xmm0
1407ece4f: f30f588564020000 addss xmm0, dword ptr [rbp + 0x264]
1407ece57: 0f138d10020000 movlps qword ptr [rbp + 0x210], xmm1
1407ece5e: f30f118518020000 movss dword ptr [rbp + 0x218], xmm0
1407ece66: 440f2965e0 movaps xmmword ptr [rbp - 0x20], xmm12
1407ece6b: c685a602000000 mov byte ptr [rbp + 0x2a6], 0
1407ece72: 488d45e0 lea rax, [rbp - 0x20]
1407ece76: 4889442420 mov qword ptr [rsp + 0x20], rax
1407ece7b: 488b8da8020000 mov rcx, qword ptr [rbp + 0x2a8]
1407ece82: 4889fa mov rdx, rdi
1407ece85: 4c8d8d10020000 lea r9, [rbp + 0x210]
1407ece8c: e81f840300 call 0x1408252b0 ; system_decorator::blueprint::DecoratorBlueprint::set_colliders
1407ece91: 488b95a8020000 mov rdx, qword ptr [rbp + 0x2a8]
1407ece98: 488b02 mov rax, qword ptr [rdx]
1407ece9b: 480b4208 or rax, qword ptr [rdx + 8]
1407ece9f: 0f85171e0000 jne 0x1407eecbc
1407ecea5: 48c7420800000000 mov qword ptr [rdx + 8], 0
1407ecead: 48c70201000000 mov qword ptr [rdx], 1
1407eceb4: 0f288510010000 movaps xmm0, xmmword ptr [rbp + 0x110]
1407ecebb: 0f288d20010000 movaps xmm1, xmmword ptr [rbp + 0x120]
1407ecec2: 0f289530010000 movaps xmm2, xmmword ptr [rbp + 0x130]
1407ecec9: 0f295230 movaps xmmword ptr [rdx + 0x30], xmm2
1407ececd: 0f294a20 movaps xmmword ptr [rdx + 0x20], xmm1
1407eced1: 0f294210 movaps xmmword ptr [rdx + 0x10], xmm0
1407eced5: 488b85b8010000 mov rax, qword ptr [rbp + 0x1b8]
1407ecedc: 4805b0000000 add rax, 0xb0
1407ecee2: 48898508020000 mov qword ptr [rbp + 0x208], rax
1407ecee9: 660f6f8510010000 movdqa xmm0, xmmword ptr [rbp + 0x110]
1407ecef1: 660f6f8d20010000 movdqa xmm1, xmmword ptr [rbp + 0x120]
1407ecef9: 660f6f9530010000 movdqa xmm2, xmmword ptr [rbp + 0x130]
1407ecf01: 660f7f85c0010000 movdqa xmmword ptr [rbp + 0x1c0], xmm0
1407ecf09: 660f7f8dd0010000 movdqa xmmword ptr [rbp + 0x1d0], xmm1
1407ecf11: 660f7f95e0010000 movdqa xmmword ptr [rbp + 0x1e0], xmm2
1407ecf19: 488d8513020000 lea rax, [rbp + 0x213]
1407ecf20: c7400800000000 mov dword ptr [rax + 8], 0
1407ecf27: 48c70000000000 mov qword ptr [rax], 0
1407ecf2e: 488b8510020000 mov rax, qword ptr [rbp + 0x210]
1407ecf35: 488b8d17020000 mov rcx, qword ptr [rbp + 0x217]
1407ecf3c: 48898d9f000000 mov qword ptr [rbp + 0x9f], rcx
1407ecf43: 48898598000000 mov qword ptr [rbp + 0x98], rax
1407ecf4a: 488bb2c0000000 mov rsi, qword ptr [rdx + 0xc0]
1407ecf51: 483bb2b0000000 cmp rsi, qword ptr [rdx + 0xb0]
1407ecf58: 488bbd90020000 mov rdi, qword ptr [rbp + 0x290]
1407ecf5f: 7513 jne 0x1407ecf74
1407ecf61: 488b8d08020000 mov rcx, qword ptr [rbp + 0x208]
1407ecf68: 488d15b1d62902 lea rdx, [rip + 0x229d6b1]
1407ecf6f: e83ce40300 call 0x14082b3b0 ; alloc::raw_vec::RawVec::grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
1407ecf74: 4c8b8da8020000 mov r9, qword ptr [rbp + 0x2a8]
1407ecf7b: 498b81b8000000 mov rax, qword ptr [r9 + 0xb8]
1407ecf82: 4869ce60010000 imul rcx, rsi, 0x160
1407ecf89: 48c7040800000000 mov qword ptr [rax + rcx], 0
1407ecf91: 48c744081000000000 mov qword ptr [rax + rcx + 0x10], 0
1407ecf9a: 48c744082000000000 mov qword ptr [rax + rcx + 0x20], 0
1407ecfa3: c744083000000000 mov dword ptr [rax + rcx + 0x30], 0
1407ecfab: 488b9500010000 mov rdx, qword ptr [rbp + 0x100]
1407ecfb2: 4889540834 mov qword ptr [rax + rcx + 0x34], rdx
1407ecfb7: 8b9508010000 mov edx, dword ptr [rbp + 0x108]
1407ecfbd: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407ecfc1: c744084000000000 mov dword ptr [rax + rcx + 0x40], 0
1407ecfc9: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
1407ecfd0: 0f108d20020000 movups xmm1, xmmword ptr [rbp + 0x220]
1407ecfd7: 0f109530020000 movups xmm2, xmmword ptr [rbp + 0x230]
1407ecfde: 0f109d3c020000 movups xmm3, xmmword ptr [rbp + 0x23c]
1407ecfe5: 0f11440844 movups xmmword ptr [rax + rcx + 0x44], xmm0
1407ecfea: 0f114c0854 movups xmmword ptr [rax + rcx + 0x54], xmm1
1407ecfef: 0f11540864 movups xmmword ptr [rax + rcx + 0x64], xmm2
1407ecff4: 0f115c0870 movups xmmword ptr [rax + rcx + 0x70], xmm3
1407ecff9: c784088000000000000000 mov dword ptr [rax + rcx + 0x80], 0
1407ed004: 488b5570 mov rdx, qword ptr [rbp + 0x70]
1407ed008: 4889940884000000 mov qword ptr [rax + rcx + 0x84], rdx
1407ed010: 8b5578 mov edx, dword ptr [rbp + 0x78]
1407ed013: 8994088c000000 mov dword ptr [rax + rcx + 0x8c], edx
1407ed01a: c784089000000000000000 mov dword ptr [rax + rcx + 0x90], 0
1407ed025: 488b557c mov rdx, qword ptr [rbp + 0x7c]
1407ed029: 4889940894000000 mov qword ptr [rax + rcx + 0x94], rdx
1407ed031: 8b9584000000 mov edx, dword ptr [rbp + 0x84]
1407ed037: 8994089c000000 mov dword ptr [rax + rcx + 0x9c], edx
1407ed03e: 4889bc08a0000000 mov qword ptr [rax + rcx + 0xa0], rdi
1407ed046: 48c78408a80000001d000000 mov qword ptr [rax + rcx + 0xa8], 0x1d
1407ed052: 488d15a7d52902 lea rdx, [rip + 0x229d5a7]
1407ed059: 48899408b0000000 mov qword ptr [rax + rcx + 0xb0], rdx
1407ed061: 48c78408b80000001e000000 mov qword ptr [rax + rcx + 0xb8], 0x1e
1407ed06d: 0f2885c0010000 movaps xmm0, xmmword ptr [rbp + 0x1c0]
1407ed074: 0f288dd0010000 movaps xmm1, xmmword ptr [rbp + 0x1d0]
1407ed07b: 0f2895e0010000 movaps xmm2, xmmword ptr [rbp + 0x1e0]
1407ed082: 0f299408e0000000 movaps xmmword ptr [rax + rcx + 0xe0], xmm2
1407ed08a: 0f298c08d0000000 movaps xmmword ptr [rax + rcx + 0xd0], xmm1
1407ed092: 0f298408c0000000 movaps xmmword ptr [rax + rcx + 0xc0], xmm0
1407ed09a: c78408f000000001000000 mov dword ptr [rax + rcx + 0xf0], 1
1407ed0a5: 8b9590010000 mov edx, dword ptr [rbp + 0x190]
1407ed0ab: 899408f4000000 mov dword ptr [rax + rcx + 0xf4], edx
1407ed0b2: c78408f800000000000000 mov dword ptr [rax + rcx + 0xf8], 0
1407ed0bd: 488b9560010000 mov rdx, qword ptr [rbp + 0x160]
1407ed0c4: 488994080c010000 mov qword ptr [rax + rcx + 0x10c], rdx
1407ed0cc: 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
1407ed0d3: 0f118408fc000000 movups xmmword ptr [rax + rcx + 0xfc], xmm0
1407ed0db: c684081401000002 mov byte ptr [rax + rcx + 0x114], 2
1407ed0e3: 0fb6958a000000 movzx edx, byte ptr [rbp + 0x8a]
1407ed0ea: 88940817010000 mov byte ptr [rax + rcx + 0x117], dl
1407ed0f1: 0fb79588000000 movzx edx, word ptr [rbp + 0x88]
1407ed0f8: 6689940815010000 mov word ptr [rax + rcx + 0x115], dx
1407ed100: c784081801000000000000 mov dword ptr [rax + rcx + 0x118], 0
1407ed10b: f30f6f8598010000 movdqu xmm0, xmmword ptr [rbp + 0x198]
1407ed113: 0f108da8010000 movups xmm1, xmmword ptr [rbp + 0x1a8]
1407ed11a: f30f7f84081c010000 movdqu xmmword ptr [rax + rcx + 0x11c], xmm0
1407ed123: 0f118c082c010000 movups xmmword ptr [rax + rcx + 0x12c], xmm1
1407ed12b: c684083c01000002 mov byte ptr [rax + rcx + 0x13c], 2
1407ed133: 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
1407ed13a: 4c8b859f000000 mov r8, qword ptr [rbp + 0x9f]
1407ed141: 4c89840844010000 mov qword ptr [rax + rcx + 0x144], r8
1407ed149: 488994083d010000 mov qword ptr [rax + rcx + 0x13d], rdx
1407ed151: c684084c01000002 mov byte ptr [rax + rcx + 0x14c], 2
1407ed159: 0fb695f8010000 movzx edx, byte ptr [rbp + 0x1f8]
1407ed160: 8894084e010000 mov byte ptr [rax + rcx + 0x14e], dl
1407ed167: 66c784084f0100000009 mov word ptr [rax + rcx + 0x14f], 0x900
1407ed171: 488b9578010000 mov rdx, qword ptr [rbp + 0x178]
1407ed178: 4c8b857f010000 mov r8, qword ptr [rbp + 0x17f]
1407ed17f: 4889940851010000 mov qword ptr [rax + rcx + 0x151], rdx
1407ed187: 4c89840858010000 mov qword ptr [rax + rcx + 0x158], r8
1407ed18f: 48ffc6 inc rsi
1407ed192: 4989b1c0000000 mov qword ptr [r9 + 0xc0], rsi
1407ed199: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407ed1a0: 8378b802 cmp dword ptr [rax - 0x48], 2
1407ed1a4: 0f830b040000 jae 0x1407ed5b5
1407ed1aa: 488b9598020000 mov rdx, qword ptr [rbp + 0x298]
1407ed1b1: 0fb65afc movzx ebx, byte ptr [rdx - 4]
1407ed1b5: 8d43ff lea eax, [rbx - 1]
1407ed1b8: 3c03 cmp al, 3
1407ed1ba: 0f835a1c0000 jae 0x1407eee1a
1407ed1c0: 0fb6c0 movzx eax, al
1407ed1c3: 488d0d7adc2902 lea rcx, [rip + 0x229dc7a]
1407ed1ca: 48633c81 movsxd rdi, dword ptr [rcx + rax*4]
1407ed1ce: 488d8513020000 lea rax, [rbp + 0x213]
1407ed1d5: c7400800000000 mov dword ptr [rax + 8], 0
1407ed1dc: 48c70000000000 mov qword ptr [rax], 0
1407ed1e3: 488d4ae8 lea rcx, [rdx - 0x18]
1407ed1e7: 488d15bad42902 lea rdx, [rip + 0x229d4ba]
1407ed1ee: e8bdc10400 call 0x1408393b0 ; country_core::resources::walls::decorator::DecoratorExtraInfo::as_window | country_core::resources::walls::decorator::DecoratorExtraInfo::as_window_mut
1407ed1f3: 8b00 mov eax, dword ptr [rax]
1407ed1f5: 898590020000 mov dword ptr [rbp + 0x290], eax
1407ed1fb: 488d8d20010000 lea rcx, [rbp + 0x120]
1407ed202: 8b411c mov eax, dword ptr [rcx + 0x1c]
1407ed205: 898560010000 mov dword ptr [rbp + 0x160], eax
1407ed20b: f30f6f410c movdqu xmm0, xmmword ptr [rcx + 0xc]
1407ed210: 660f7f8550010000 movdqa xmmword ptr [rbp + 0x150], xmm0
1407ed218: 488b8510020000 mov rax, qword ptr [rbp + 0x210]
1407ed21f: 488b8d17020000 mov rcx, qword ptr [rbp + 0x217]
1407ed226: 48898598000000 mov qword ptr [rbp + 0x98], rax
1407ed22d: 48898d9f000000 mov qword ptr [rbp + 0x9f], rcx
1407ed234: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
1407ed23b: 488bb0c0000000 mov rsi, qword ptr [rax + 0xc0]
1407ed242: 483bb0b0000000 cmp rsi, qword ptr [rax + 0xb0]
1407ed249: 7513 jne 0x1407ed25e
1407ed24b: 488b8d08020000 mov rcx, qword ptr [rbp + 0x208]
1407ed252: 488d1567d42902 lea rdx, [rip + 0x229d467]
1407ed259: e852e10300 call 0x14082b3b0 ; alloc::raw_vec::RawVec::grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
1407ed25e: 410f28cc movaps xmm1, xmm12
1407ed262: 410f59cc mulps xmm1, xmm12
1407ed266: f30f16c1 movshdup xmm0, xmm1
1407ed26a: f30f58c1 addss xmm0, xmm1
1407ed26e: 0f28d1 movaps xmm2, xmm1
1407ed271: 660f15d1 unpckhpd xmm2, xmm1
1407ed275: f30f58d0 addss xmm2, xmm0
1407ed279: 0fc6d200 shufps xmm2, xmm2, 0
1407ed27d: 0fc6c9ff shufps xmm1, xmm1, 0xff
1407ed281: 0f5cca subps xmm1, xmm2
1407ed284: 0f281535cc2902 movaps xmm2, xmmword ptr [rip + 0x229cc35]
1407ed28b: 0f59ca mulps xmm1, xmm2
1407ed28e: 410f28c4 movaps xmm0, xmm12
1407ed292: 0f59c2 mulps xmm0, xmm2
1407ed295: f30f16d0 movshdup xmm2, xmm0
1407ed299: f30f58d0 addss xmm2, xmm0
1407ed29d: 0f12c0 movhlps xmm0, xmm0
1407ed2a0: f30f58c2 addss xmm0, xmm2
1407ed2a4: f30f58c0 addss xmm0, xmm0
1407ed2a8: 0fc6c000 shufps xmm0, xmm0, 0
1407ed2ac: 410f59c4 mulps xmm0, xmm12
1407ed2b0: 0f58c1 addps xmm0, xmm1
1407ed2b3: 410f28cc movaps xmm1, xmm12
1407ed2b7: f30f101d95102902 movss xmm3, dword ptr [rip + 0x2291095]
1407ed2bf: 0f59cb mulps xmm1, xmm3
1407ed2c2: 410f28d4 movaps xmm2, xmm12
1407ed2c6: 410fc6d4c9 shufps xmm2, xmm12, 0xc9
1407ed2cb: 0f59d3 mulps xmm2, xmm3
1407ed2ce: 0fc6c9d2 shufps xmm1, xmm1, 0xd2
1407ed2d2: 0f5cd1 subps xmm2, xmm1
1407ed2d5: 410f28dc movaps xmm3, xmm12
1407ed2d9: 410f58dc addps xmm3, xmm12
1407ed2dd: 0fc6dbff shufps xmm3, xmm3, 0xff
1407ed2e1: 0f59da mulps xmm3, xmm2
1407ed2e4: 0f58d8 addps xmm3, xmm0
1407ed2e7: 0f284560 movaps xmm0, xmmword ptr [rbp + 0x60]
1407ed2eb: 0f5cc3 subps xmm0, xmm3
1407ed2ee: 0f12db movhlps xmm3, xmm3
1407ed2f1: f30f108d64020000 movss xmm1, dword ptr [rbp + 0x264]
1407ed2f9: f30f5ccb subss xmm1, xmm3
1407ed2fd: 488d0540db2902 lea rax, [rip + 0x229db40]
1407ed304: 4801c7 add rdi, rax
1407ed307: 4869d3e8030000 imul rdx, rbx, 0x3e8
1407ed30e: 48039578020000 add rdx, qword ptr [rbp + 0x278]
1407ed315: 4c8b8da8020000 mov r9, qword ptr [rbp + 0x2a8]
1407ed31c: 498b81b8000000 mov rax, qword ptr [r9 + 0xb8]
1407ed323: 4869ce60010000 imul rcx, rsi, 0x160
1407ed32a: 48c7040800000000 mov qword ptr [rax + rcx], 0
1407ed332: 48c744081001000000 mov qword ptr [rax + rcx + 0x10], 1
1407ed33b: 4889540818 mov qword ptr [rax + rcx + 0x18], rdx
1407ed340: 48c744082000000000 mov qword ptr [rax + rcx + 0x20], 0
1407ed349: c744083000000000 mov dword ptr [rax + rcx + 0x30], 0
1407ed351: 488b9500010000 mov rdx, qword ptr [rbp + 0x100]
1407ed358: 4889540834 mov qword ptr [rax + rcx + 0x34], rdx
1407ed35d: 8b9508010000 mov edx, dword ptr [rbp + 0x108]
1407ed363: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407ed367: c744084000000000 mov dword ptr [rax + rcx + 0x40], 0
1407ed36f: 0f109510020000 movups xmm2, xmmword ptr [rbp + 0x210]
1407ed376: 0f109d20020000 movups xmm3, xmmword ptr [rbp + 0x220]
1407ed37d: 0f10a530020000 movups xmm4, xmmword ptr [rbp + 0x230]
1407ed384: 0f10ad3c020000 movups xmm5, xmmword ptr [rbp + 0x23c]
1407ed38b: 0f11540844 movups xmmword ptr [rax + rcx + 0x44], xmm2
1407ed390: 0f115c0854 movups xmmword ptr [rax + rcx + 0x54], xmm3
1407ed395: 0f11640864 movups xmmword ptr [rax + rcx + 0x64], xmm4
1407ed39a: 0f116c0870 movups xmmword ptr [rax + rcx + 0x70], xmm5
1407ed39f: c784088000000000000000 mov dword ptr [rax + rcx + 0x80], 0
1407ed3aa: 488b5570 mov rdx, qword ptr [rbp + 0x70]
1407ed3ae: 4889940884000000 mov qword ptr [rax + rcx + 0x84], rdx
1407ed3b6: 8b5578 mov edx, dword ptr [rbp + 0x78]
1407ed3b9: 8994088c000000 mov dword ptr [rax + rcx + 0x8c], edx
1407ed3c0: c784089000000000000000 mov dword ptr [rax + rcx + 0x90], 0
1407ed3cb: 488b557c mov rdx, qword ptr [rbp + 0x7c]
1407ed3cf: 4889940894000000 mov qword ptr [rax + rcx + 0x94], rdx
1407ed3d7: 8b9584000000 mov edx, dword ptr [rbp + 0x84]
1407ed3dd: 8994089c000000 mov dword ptr [rax + rcx + 0x9c], edx
1407ed3e4: 4889bc08a0000000 mov qword ptr [rax + rcx + 0xa0], rdi
1407ed3ec: 48c78408a800000023000000 mov qword ptr [rax + rcx + 0xa8], 0x23
1407ed3f8: 488d154d3e4302 lea rdx, [rip + 0x2433e4d]
1407ed3ff: 48899408b0000000 mov qword ptr [rax + rcx + 0xb0], rdx
1407ed407: 48c78408b800000024000000 mov qword ptr [rax + rcx + 0xb8], 0x24
1407ed413: 440f29a408c0000000 movaps xmmword ptr [rax + rcx + 0xc0], xmm12
1407ed41c: 0f138408d0000000 movlps qword ptr [rax + rcx + 0xd0], xmm0
1407ed424: f30f118c08d8000000 movss dword ptr [rax + rcx + 0xd8], xmm1
1407ed42d: 8b9560010000 mov edx, dword ptr [rbp + 0x160]
1407ed433: 899408ec000000 mov dword ptr [rax + rcx + 0xec], edx
1407ed43a: 0f288550010000 movaps xmm0, xmmword ptr [rbp + 0x150]
1407ed441: 0f118408dc000000 movups xmmword ptr [rax + rcx + 0xdc], xmm0
1407ed449: c78408f000000001000000 mov dword ptr [rax + rcx + 0xf0], 1
1407ed454: 8b9590020000 mov edx, dword ptr [rbp + 0x290]
1407ed45a: 899408f4000000 mov dword ptr [rax + rcx + 0xf4], edx
1407ed461: c78408f800000000000000 mov dword ptr [rax + rcx + 0xf8], 0
1407ed46c: 0f108598010000 movups xmm0, xmmword ptr [rbp + 0x198]
1407ed473: 0f118408fc000000 movups xmmword ptr [rax + rcx + 0xfc], xmm0
1407ed47b: 488b95a8010000 mov rdx, qword ptr [rbp + 0x1a8]
1407ed482: 488994080c010000 mov qword ptr [rax + rcx + 0x10c], rdx
1407ed48a: c684081401000002 mov byte ptr [rax + rcx + 0x114], 2
1407ed492: 0fb6958a000000 movzx edx, byte ptr [rbp + 0x8a]
1407ed499: 88940817010000 mov byte ptr [rax + rcx + 0x117], dl
1407ed4a0: 0fb79588000000 movzx edx, word ptr [rbp + 0x88]
1407ed4a7: 6689940815010000 mov word ptr [rax + rcx + 0x115], dx
1407ed4af: c784081801000000000000 mov dword ptr [rax + rcx + 0x118], 0
1407ed4ba: 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
1407ed4c1: 0f108dd0010000 movups xmm1, xmmword ptr [rbp + 0x1d0]
1407ed4c8: 0f118c082c010000 movups xmmword ptr [rax + rcx + 0x12c], xmm1
1407ed4d0: 0f1184081c010000 movups xmmword ptr [rax + rcx + 0x11c], xmm0
1407ed4d8: c684083c01000002 mov byte ptr [rax + rcx + 0x13c], 2
1407ed4e0: 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
1407ed4e7: 4c8b859f000000 mov r8, qword ptr [rbp + 0x9f]
1407ed4ee: 4c89840844010000 mov qword ptr [rax + rcx + 0x144], r8
1407ed4f6: 488994083d010000 mov qword ptr [rax + rcx + 0x13d], rdx
1407ed4fe: c684084c01000002 mov byte ptr [rax + rcx + 0x14c], 2
1407ed506: 0fb695f8010000 movzx edx, byte ptr [rbp + 0x1f8]
1407ed50d: 8894084e010000 mov byte ptr [rax + rcx + 0x14e], dl
1407ed514: 66c784084f0100000009 mov word ptr [rax + rcx + 0x14f], 0x900
1407ed51e: 488b9578010000 mov rdx, qword ptr [rbp + 0x178]
1407ed525: 4c8b857f010000 mov r8, qword ptr [rbp + 0x17f]
1407ed52c: 4889940851010000 mov qword ptr [rax + rcx + 0x151], rdx
1407ed534: 4c89840858010000 mov qword ptr [rax + rcx + 0x158], r8
1407ed53c: 48ffc6 inc rsi
1407ed53f: 4989b1c0000000 mov qword ptr [r9 + 0xc0], rsi
1407ed546: f30f1085d0000000 movss xmm0, dword ptr [rbp + 0xd0]
1407ed54e: f30f118590020000 movss dword ptr [rbp + 0x290], xmm0
1407ed556: f3440f1095dc000000 movss xmm10, dword ptr [rbp + 0xdc]
1407ed55f: f3440f5c95d8000000 subss xmm10, dword ptr [rbp + 0xd8]
1407ed568: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407ed56f: 8378b800 cmp dword ptr [rax - 0x48], 0
1407ed573: 7562 jne 0x1407ed5d7
1407ed575: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ed57c: 0fb640fc movzx eax, byte ptr [rax - 4]
1407ed580: 83f801 cmp eax, 1
1407ed583: 0f84b7140000 je 0x1407eea40
1407ed589: 83f802 cmp eax, 2
1407ed58c: 0f8491140000 je 0x1407eea23
1407ed592: 83f803 cmp eax, 3
1407ed595: 0f85c4180000 jne 0x1407eee5f
1407ed59b: f30f108590020000 movss xmm0, dword ptr [rbp + 0x290]
1407ed5a3: f30f580555062002 addss xmm0, dword ptr [rip + 0x2200655]
1407ed5ab: f30f118590020000 movss dword ptr [rbp + 0x290], xmm0
1407ed5b3: eb22 jmp 0x1407ed5d7 ; unknown
1407ed5b5: f30f1085d0000000 movss xmm0, dword ptr [rbp + 0xd0]
1407ed5bd: f30f118590020000 movss dword ptr [rbp + 0x290], xmm0
1407ed5c5: f3440f1095dc000000 movss xmm10, dword ptr [rbp + 0xdc]
1407ed5ce: f3440f5c95d8000000 subss xmm10, dword ptr [rbp + 0xd8]
1407ed5d7: 488b8d70020000 mov rcx, qword ptr [rbp + 0x270]
1407ed5de: 488d1533d12902 lea rdx, [rip + 0x229d133]
1407ed5e5: e8765c0400 call 0x140833260 ; country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
1407ed5ea: f20f1030 movsd xmm6, qword ptr [rax]
1407ed5ee: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
1407ed5f5: 488bb0d8000000 mov rsi, qword ptr [rax + 0xd8]
1407ed5fc: 483bb0c8000000 cmp rsi, qword ptr [rax + 0xc8]
1407ed603: 751a jne 0x1407ed61f
1407ed605: 488b85b8010000 mov rax, qword ptr [rbp + 0x1b8]
1407ed60c: 488d88c8000000 lea rcx, [rax + 0xc8]
1407ed613: 488d1516d12902 lea rdx, [rip + 0x229d116]
1407ed61a: e8e15fd7ff call 0x140563600 ; alloc::raw_vec::RawVec::grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<glam::f32::vec2::Vec2,utils::curve::curve_u::CurveU<utils::curve::CurveCoor
1407ed61f: 488b95a8020000 mov rdx, qword ptr [rbp + 0x2a8]
1407ed626: 488b82d0000000 mov rax, qword ptr [rdx + 0xd0]
1407ed62d: 488d0cb6 lea rcx, [rsi + rsi*4]
1407ed631: 0f133488 movlps qword ptr [rax + rcx*4], xmm6
1407ed635: f30f108590020000 movss xmm0, dword ptr [rbp + 0x290]
1407ed63d: f30f11448808 movss dword ptr [rax + rcx*4 + 8], xmm0
1407ed643: f3440f1154880c movss dword ptr [rax + rcx*4 + 0xc], xmm10
1407ed64a: c644881000 mov byte ptr [rax + rcx*4 + 0x10], 0
1407ed64f: 48ffc6 inc rsi
1407ed652: 4889b2d8000000 mov qword ptr [rdx + 0xd8], rsi
1407ed659: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ed660: 8b48e4 mov ecx, dword ptr [rax - 0x1c]
1407ed663: e8d88b4a00 call 0x140c96240 ; utils::rng::init_rng
1407ed668: 4889c7 mov rdi, rax
1407ed66b: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ed672: 0fb670fc movzx esi, byte ptr [rax - 4]
1407ed676: 4883fe02 cmp rsi, 2
1407ed67a: 0f85e7010000 jne 0x1407ed867
1407ed680: 48b82f64bd78641d76a0 movabs rax, 0xa0761d6478bd642f
1407ed68a: 4801c7 add rdi, rax
1407ed68d: 4889f8 mov rax, rdi
1407ed690: 48b9db28b4a0d17e03e7 movabs rcx, 0xe7037ed1a0b428db
1407ed69a: 4831c8 xor rax, rcx
1407ed69d: 48f7e7 mul rdi
1407ed6a0: 31d0 xor eax, edx
1407ed6a2: c1e81f shr eax, 0x1f
1407ed6a5: 48898548010000 mov qword ptr [rbp + 0x148], rax
1407ed6ac: 80bd8002000001 cmp byte ptr [rbp + 0x280], 1
1407ed6b3: 0f84c6010000 je 0x1407ed87f
1407ed6b9: be02000000 mov esi, 2
1407ed6be: 450f57c9 xorps xmm9, xmm9
1407ed6c2: bb03000000 mov ebx, 3
1407ed6c7: f3440f10b5d0000000 movss xmm14, dword ptr [rbp + 0xd0]
1407ed6d0: 488b8d70020000 mov rcx, qword ptr [rbp + 0x270]
1407ed6d7: 488d1512db4302 lea rdx, [rip + 0x243db12]
1407ed6de: e87d5b0400 call 0x140833260 ; country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
1407ed6e3: f30f1038 movss xmm7, dword ptr [rax]
1407ed6e7: f3440f105804 movss xmm11, dword ptr [rax + 4]
1407ed6ed: 488b8d68020000 mov rcx, qword ptr [rbp + 0x268]
1407ed6f4: e847d03300 call 0x140b2a740 ; country_core::resources::walls::public_wall_state::PublicWallState::wall_space
1407ed6f9: 898544010000 mov dword ptr [rbp + 0x144], eax
1407ed6ff: 0f28f0 movaps xmm6, xmm0
1407ed702: 450f28c6 movaps xmm8, xmm14
1407ed706: f3440f5905d9c91602 mulss xmm8, dword ptr [rip + 0x216c9d9]
1407ed70f: 8b8544010000 mov eax, dword ptr [rbp + 0x144]
1407ed715: 898510020000 mov dword ptr [rbp + 0x210], eax
1407ed71b: f30f118514020000 movss dword ptr [rbp + 0x214], xmm0
1407ed723: f3410f58f8 addss xmm7, xmm8
1407ed728: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ed72f: 0f28cf movaps xmm1, xmm7
1407ed732: e8e9474a00 call 0x140c91f20 ; utils::wall_space::impl$1::project_unbounded
1407ed737: 0f57c9 xorps xmm1, xmm1
1407ed73a: f30f2ace cvtsi2ss xmm1, esi
1407ed73e: f3440f5905492f1702 mulss xmm8, dword ptr [rip + 0x2172f49]
1407ed747: f3450f58c3 addss xmm8, xmm11
1407ed74c: f3440f5ef1 divss xmm14, xmm1
1407ed751: 48c7851002000000000000 mov qword ptr [rbp + 0x210], 0
1407ed75c: 48899d18020000 mov qword ptr [rbp + 0x218], rbx
1407ed763: 8b8544010000 mov eax, dword ptr [rbp + 0x144]
1407ed769: 898520020000 mov dword ptr [rbp + 0x220], eax
1407ed76f: f30f11b524020000 movss dword ptr [rbp + 0x224], xmm6
1407ed777: f30f118528020000 movss dword ptr [rbp + 0x228], xmm0
1407ed77f: f3440f11852c020000 movss dword ptr [rbp + 0x22c], xmm8
1407ed788: f3440f11b530020000 movss dword ptr [rbp + 0x230], xmm14
1407ed791: 488b8568020000 mov rax, qword ptr [rbp + 0x268]
1407ed798: 48898538020000 mov qword ptr [rbp + 0x238], rax
1407ed79f: c7854002000000000000 mov dword ptr [rbp + 0x240], 0
1407ed7a9: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407ed7b0: 488d9510020000 lea rdx, [rbp + 0x210]
1407ed7b7: 4c8d0572cb2902 lea r8, [rip + 0x229cb72]
1407ed7be: e8dd34a700 call 0x141260ca0 ; alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core:
1407ed7c3: 4c8b85c8010000 mov r8, qword ptr [rbp + 0x1c8]
1407ed7ca: 4c8b8dd0010000 mov r9, qword ptr [rbp + 0x1d0]
1407ed7d1: f3440f114c2448 movss dword ptr [rsp + 0x48], xmm9
1407ed7d8: 488d85e0000000 lea rax, [rbp + 0xe0]
1407ed7df: 4889442438 mov qword ptr [rsp + 0x38], rax
1407ed7e4: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
1407ed7eb: 4889442430 mov qword ptr [rsp + 0x30], rax
1407ed7f0: 8b8590010000 mov eax, dword ptr [rbp + 0x190]
1407ed7f6: 89442428 mov dword ptr [rsp + 0x28], eax
1407ed7fa: 0fb685f8010000 movzx eax, byte ptr [rbp + 0x1f8]
1407ed801: 88442420 mov byte ptr [rsp + 0x20], al
1407ed805: c744245000000000 mov dword ptr [rsp + 0x50], 0
1407ed80d: 488b8d78020000 mov rcx, qword ptr [rbp + 0x278]
1407ed814: 488b9570020000 mov rdx, qword ptr [rbp + 0x270]
1407ed81b: 4c898568020000 mov qword ptr [rbp + 0x268], r8
1407ed822: e869210000 call 0x1407ef990 ; system_decorator::decorator_visual::cottage_wall_window::add_lintels
1407ed827: 488b85c0010000 mov rax, qword ptr [rbp + 0x1c0]
1407ed82e: 4885c0 test rax, rax
1407ed831: 0f8485060000 je 0x1407edebc
1407ed837: 48c1e004 shl rax, 4
1407ed83b: 488d1440 lea rdx, [rax + rax*2]
1407ed83f: 41b810000000 mov r8d, 0x10
1407ed845: 488b8d68020000 mov rcx, qword ptr [rbp + 0x268]
1407ed84c: e8cf63e2ff call 0x140613c20 ; unknown
1407ed851: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407ed858: 8378b802 cmp dword ptr [rax - 0x48], 2
1407ed85c: 0f836b060000 jae 0x1407edecd
1407ed862: e9640a0000 jmp 0x1407ee2cb ; unknown
1407ed867: 80bd8002000001 cmp byte ptr [rbp + 0x280], 1
1407ed86e: 0f85020e0000 jne 0x1407ee676
1407ed874: 48c7854801000000000000 mov qword ptr [rbp + 0x148], 0
1407ed87f: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407ed886: 8378b801 cmp dword ptr [rax - 0x48], 1
1407ed88a: 0f8748060000 ja 0x1407eded8
1407ed890: 0f2885c0000000 movaps xmm0, xmmword ptr [rbp + 0xc0]
1407ed897: 0f288dd0000000 movaps xmm1, xmmword ptr [rbp + 0xd0]
1407ed89e: 0f298d60010000 movaps xmmword ptr [rbp + 0x160], xmm1
1407ed8a5: 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
1407ed8ac: c78564010000ae47213f mov dword ptr [rbp + 0x164], 0x3f2147ae
1407ed8b6: 410f28cc movaps xmm1, xmm12
1407ed8ba: 410f59cc mulps xmm1, xmm12
1407ed8be: f30f16c1 movshdup xmm0, xmm1
1407ed8c2: f30f58c1 addss xmm0, xmm1
1407ed8c6: 0f28d1 movaps xmm2, xmm1
1407ed8c9: 660f15d1 unpckhpd xmm2, xmm1
1407ed8cd: f30f58d0 addss xmm2, xmm0
1407ed8d1: 0fc6d200 shufps xmm2, xmm2, 0
1407ed8d5: 0fc6c9ff shufps xmm1, xmm1, 0xff
1407ed8d9: 0f5cca subps xmm1, xmm2
1407ed8dc: 0f2815ddf71d02 movaps xmm2, xmmword ptr [rip + 0x21df7dd]
1407ed8e3: 0f59ca mulps xmm1, xmm2
1407ed8e6: 410f28c4 movaps xmm0, xmm12
1407ed8ea: 0f59c2 mulps xmm0, xmm2
1407ed8ed: f30f16d0 movshdup xmm2, xmm0
1407ed8f1: f30f58d0 addss xmm2, xmm0
1407ed8f5: 0f12c0 movhlps xmm0, xmm0
1407ed8f8: f30f58c2 addss xmm0, xmm2
1407ed8fc: f30f58c0 addss xmm0, xmm0
1407ed900: 0fc6c000 shufps xmm0, xmm0, 0
1407ed904: 410f59c4 mulps xmm0, xmm12
1407ed908: 0f58c1 addps xmm0, xmm1
1407ed90b: 410f28cc movaps xmm1, xmm12
1407ed90f: f30f101ded7f1302 movss xmm3, dword ptr [rip + 0x2137fed]
1407ed917: 0f59cb mulps xmm1, xmm3
1407ed91a: 410f28d4 movaps xmm2, xmm12
1407ed91e: 410fc6d4c9 shufps xmm2, xmm12, 0xc9
1407ed923: 0f59d3 mulps xmm2, xmm3
1407ed926: 0fc6c9d2 shufps xmm1, xmm1, 0xd2
1407ed92a: 0f5cd1 subps xmm2, xmm1
1407ed92d: 410f28f4 movaps xmm6, xmm12
1407ed931: 410f58f4 addps xmm6, xmm12
1407ed935: 0fc6f6ff shufps xmm6, xmm6, 0xff
1407ed939: 0f59f2 mulps xmm6, xmm2
1407ed93c: 0f58f0 addps xmm6, xmm0
1407ed93f: 0f28fe movaps xmm7, xmm6
1407ed942: 660f15fe unpckhpd xmm7, xmm6
1407ed946: 0f28c7 movaps xmm0, xmm7
1407ed949: f30f59057fc52902 mulss xmm0, dword ptr [rip + 0x229c57f]
1407ed951: f30f5905177f1302 mulss xmm0, dword ptr [rip + 0x2137f17]
1407ed959: f30f588564020000 addss xmm0, dword ptr [rbp + 0x264]
1407ed961: 0f28ce movaps xmm1, xmm6
1407ed964: 0f590d75c52902 mulps xmm1, xmmword ptr [rip + 0x229c575]
1407ed96b: 0f590d6e7f1302 mulps xmm1, xmmword ptr [rip + 0x2137f6e]
1407ed972: 0f584d60 addps xmm1, xmmword ptr [rbp + 0x60]
1407ed976: 0f134d00 movlps qword ptr [rbp], xmm1
1407ed97a: f30f114508 movss dword ptr [rbp + 8], xmm0
1407ed97f: 488d8df0010000 lea rcx, [rbp + 0x1f0]
1407ed986: e845ac89ff call 0x1400885d0 ; minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction | minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction | toolbar::mei::core::impl$2::as_id | to
1407ed98b: 4889c6 mov rsi, rax
1407ed98e: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ed995: 488d9550010000 lea rdx, [rbp + 0x150]
1407ed99c: e8bfac4a00 call 0x140c98660 ; utils::geometry::rectangle::Rectangle3d::extent
1407ed9a1: f30f108514020000 movss xmm0, dword ptr [rbp + 0x214]
1407ed9a9: f30f5805df2c1702 addss xmm0, dword ptr [rip + 0x2172cdf]
1407ed9b1: f30f108d18020000 movss xmm1, dword ptr [rbp + 0x218]
1407ed9b9: f30f580daf7e1302 addss xmm1, dword ptr [rip + 0x2137eaf]
1407ed9c1: f30f109510020000 movss xmm2, dword ptr [rbp + 0x210]
1407ed9c9: 0f14d0 unpcklps xmm2, xmm0
1407ed9cc: 0f58151dc52902 addps xmm2, xmmword ptr [rip + 0x229c51d]
1407ed9d3: 0f1395c0010000 movlps qword ptr [rbp + 0x1c0], xmm2
1407ed9da: f30f118dc8010000 movss dword ptr [rbp + 0x1c8], xmm1
1407ed9e2: f30f117c2428 movss dword ptr [rsp + 0x28], xmm7
1407ed9e8: f30f11742420 movss dword ptr [rsp + 0x20], xmm6
1407ed9ee: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ed9f5: 89f2 mov edx, esi
1407ed9f7: 4989e8 mov r8, rbp
1407ed9fa: 4c8d8dc0010000 lea r9, [rbp + 0x1c0]
1407eda01: e8da8ca900 call 0x1412866e0 ; country_core::resources::walls::dormers::DormerHoleSsbo::cottage
1407eda06: 488b85b8010000 mov rax, qword ptr [rbp + 0x1b8]
1407eda0d: 488db0e0000000 lea rsi, [rax + 0xe0]
1407eda14: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
1407eda1b: 488b98f0000000 mov rbx, qword ptr [rax + 0xf0]
1407eda22: 483b98e0000000 cmp rbx, qword ptr [rax + 0xe0]
1407eda29: 750f jne 0x1407eda3a
1407eda2b: 4889f1 mov rcx, rsi
1407eda2e: 488d1513cd2902 lea rdx, [rip + 0x229cd13]
1407eda35: e8b6db0300 call 0x14082b5f0 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::walls::dormers::DormerHoleSsbo,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::G
1407eda3a: 488b95a8020000 mov rdx, qword ptr [rbp + 0x2a8]
1407eda41: 488b82e8000000 mov rax, qword ptr [rdx + 0xe8]
1407eda48: 488d0c5b lea rcx, [rbx + rbx*2]
1407eda4c: 48c1e104 shl rcx, 4
1407eda50: f30f6f8510020000 movdqu xmm0, xmmword ptr [rbp + 0x210]
1407eda58: f30f6f8d20020000 movdqu xmm1, xmmword ptr [rbp + 0x220]
1407eda60: f30f6f9530020000 movdqu xmm2, xmmword ptr [rbp + 0x230]
1407eda68: f30f7f540820 movdqu xmmword ptr [rax + rcx + 0x20], xmm2
1407eda6e: f30f7f4c0810 movdqu xmmword ptr [rax + rcx + 0x10], xmm1
1407eda74: f30f7f0408 movdqu xmmword ptr [rax + rcx], xmm0
1407eda79: 48ffc3 inc rbx
1407eda7c: 48899af0000000 mov qword ptr [rdx + 0xf0], rbx
1407eda83: 80bd4801000000 cmp byte ptr [rbp + 0x148], 0
1407eda8a: 0f8451010000 je 0x1407edbe1
1407eda90: 488d8d98010000 lea rcx, [rbp + 0x198]
1407eda97: 488d9550010000 lea rdx, [rbp + 0x150]
1407eda9e: e8bdab4a00 call 0x140c98660 ; utils::geometry::rectangle::Rectangle3d::extent
1407edaa3: f3440f108598010000 movss xmm8, dword ptr [rbp + 0x198]
1407edaac: f3440f58057bf51d02 addss xmm8, dword ptr [rip + 0x21df57b]
1407edab5: f3440f118598010000 movss dword ptr [rbp + 0x198], xmm8
1407edabe: 488d8d10020000 lea rcx, [rbp + 0x210]
1407edac5: 488d9550010000 lea rdx, [rbp + 0x150]
1407edacc: e88fab4a00 call 0x140c98660 ; utils::geometry::rectangle::Rectangle3d::extent
1407edad1: f30f108518020000 movss xmm0, dword ptr [rbp + 0x218]
1407edad9: f30f10158f7d1302 movss xmm2, dword ptr [rip + 0x2137d8f]
1407edae1: f30f58c2 addss xmm0, xmm2
1407edae5: 0f28cf movaps xmm1, xmm7
1407edae8: f30f59c8 mulss xmm1, xmm0
1407edaec: f30f59ca mulss xmm1, xmm2
1407edaf0: f30f588d64020000 addss xmm1, dword ptr [rbp + 0x264]
1407edaf8: f30f12c0 movsldup xmm0, xmm0
1407edafc: 0f28d6 movaps xmm2, xmm6
1407edaff: 0f59d0 mulps xmm2, xmm0
1407edb02: 0f5915d77d1302 mulps xmm2, xmmword ptr [rip + 0x2137dd7]
1407edb09: 0f585560 addps xmm2, xmmword ptr [rbp + 0x60]
1407edb0d: 0f1355f0 movlps qword ptr [rbp - 0x10], xmm2
1407edb11: f30f114df8 movss dword ptr [rbp - 8], xmm1
1407edb16: 488d8df0010000 lea rcx, [rbp + 0x1f0]
1407edb1d: e8aeaa89ff call 0x1400885d0 ; minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction | minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction | toolbar::mei::core::impl$2::as_id | to
1407edb22: f30f10859c010000 movss xmm0, dword ptr [rbp + 0x19c]
1407edb2a: f30f58053a161f02 addss xmm0, dword ptr [rip + 0x21f163a]
1407edb32: f3440f1185c0010000 movss dword ptr [rbp + 0x1c0], xmm8
1407edb3b: f30f1185c4010000 movss dword ptr [rbp + 0x1c4], xmm0
1407edb43: c785c80100000000003f mov dword ptr [rbp + 0x1c8], 0x3f000000
1407edb4d: f30f117c2428 movss dword ptr [rsp + 0x28], xmm7
1407edb53: f30f11742420 movss dword ptr [rsp + 0x20], xmm6
1407edb59: 488d8d10020000 lea rcx, [rbp + 0x210]
1407edb60: 89c2 mov edx, eax
1407edb62: 4c8d45f0 lea r8, [rbp - 0x10]
1407edb66: 4c8d8dc0010000 lea r9, [rbp + 0x1c0]
1407edb6d: e86e8ba900 call 0x1412866e0 ; country_core::resources::walls::dormers::DormerHoleSsbo::cottage
1407edb72: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
1407edb79: 488b98f0000000 mov rbx, qword ptr [rax + 0xf0]
1407edb80: 483b98e0000000 cmp rbx, qword ptr [rax + 0xe0]
1407edb87: 750f jne 0x1407edb98
1407edb89: 4889f1 mov rcx, rsi
1407edb8c: 488d15cdcb2902 lea rdx, [rip + 0x229cbcd]
1407edb93: e858da0300 call 0x14082b5f0 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::walls::dormers::DormerHoleSsbo,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::G
1407edb98: 488b95a8020000 mov rdx, qword ptr [rbp + 0x2a8]
1407edb9f: 488b82e8000000 mov rax, qword ptr [rdx + 0xe8]
1407edba6: 488d0c5b lea rcx, [rbx + rbx*2]
1407edbaa: 48c1e104 shl rcx, 4
1407edbae: f30f6f8510020000 movdqu xmm0, xmmword ptr [rbp + 0x210]
1407edbb6: f30f6f8d20020000 movdqu xmm1, xmmword ptr [rbp + 0x220]
1407edbbe: f30f6f9530020000 movdqu xmm2, xmmword ptr [rbp + 0x230]
1407edbc6: f30f7f540820 movdqu xmmword ptr [rax + rcx + 0x20], xmm2
1407edbcc: f30f7f4c0810 movdqu xmmword ptr [rax + rcx + 0x10], xmm1
1407edbd2: f30f7f0408 movdqu xmmword ptr [rax + rcx], xmm0
1407edbd7: 48ffc3 inc rbx
1407edbda: 48899af0000000 mov qword ptr [rdx + 0xf0], rbx
1407edbe1: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407edbe8: 0fb640fc movzx eax, byte ptr [rax - 4]
1407edbec: fec8 dec al
1407edbee: 3c03 cmp al, 3
1407edbf0: 0f833e120000 jae 0x1407eee34
1407edbf6: 0fb6c0 movzx eax, al
1407edbf9: 488d0d50d22902 lea rcx, [rip + 0x229d250]
1407edc00: 48631c81 movsxd rbx, dword ptr [rcx + rax*4]
1407edc04: 660f6f8510010000 movdqa xmm0, xmmword ptr [rbp + 0x110]
1407edc0c: 660f6f8d20010000 movdqa xmm1, xmmword ptr [rbp + 0x120]
1407edc14: 660f6f9530010000 movdqa xmm2, xmmword ptr [rbp + 0x130]
1407edc1c: 660f7f95e0010000 movdqa xmmword ptr [rbp + 0x1e0], xmm2
1407edc24: 660f7f8dd0010000 movdqa xmmword ptr [rbp + 0x1d0], xmm1
1407edc2c: 660f7f85c0010000 movdqa xmmword ptr [rbp + 0x1c0], xmm0
1407edc34: 488d8513020000 lea rax, [rbp + 0x213]
1407edc3b: c7400800000000 mov dword ptr [rax + 8], 0
1407edc42: 48c70000000000 mov qword ptr [rax], 0
1407edc49: 488b8510020000 mov rax, qword ptr [rbp + 0x210]
1407edc50: 488b8d17020000 mov rcx, qword ptr [rbp + 0x217]
1407edc57: 48898d7f010000 mov qword ptr [rbp + 0x17f], rcx
1407edc5e: 48898578010000 mov qword ptr [rbp + 0x178], rax
1407edc65: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
1407edc6c: 488bb0c0000000 mov rsi, qword ptr [rax + 0xc0]
1407edc73: 483bb0b0000000 cmp rsi, qword ptr [rax + 0xb0]
1407edc7a: 7513 jne 0x1407edc8f
1407edc7c: 488b8d08020000 mov rcx, qword ptr [rbp + 0x208]
1407edc83: 488d157ecb2902 lea rdx, [rip + 0x229cb7e]
1407edc8a: e821d70300 call 0x14082b3b0 ; alloc::raw_vec::RawVec::grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
1407edc8f: 488d05bad12902 lea rax, [rip + 0x229d1ba]
1407edc96: 4801c3 add rbx, rax
1407edc99: 4c8b8da8020000 mov r9, qword ptr [rbp + 0x2a8]
1407edca0: 498b81b8000000 mov rax, qword ptr [r9 + 0xb8]
1407edca7: 4869ce60010000 imul rcx, rsi, 0x160
1407edcae: 48c7040800000000 mov qword ptr [rax + rcx], 0
1407edcb6: 48c744081000000000 mov qword ptr [rax + rcx + 0x10], 0
1407edcbf: 48c744082000000000 mov qword ptr [rax + rcx + 0x20], 0
1407edcc8: c744083000000000 mov dword ptr [rax + rcx + 0x30], 0
1407edcd0: 488b5570 mov rdx, qword ptr [rbp + 0x70]
1407edcd4: 4889540834 mov qword ptr [rax + rcx + 0x34], rdx
1407edcd9: 8b5578 mov edx, dword ptr [rbp + 0x78]
1407edcdc: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407edce0: c744084000000000 mov dword ptr [rax + rcx + 0x40], 0
1407edce8: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
1407edcef: 0f108d20020000 movups xmm1, xmmword ptr [rbp + 0x220]
1407edcf6: 0f109530020000 movups xmm2, xmmword ptr [rbp + 0x230]
1407edcfd: 0f109d3c020000 movups xmm3, xmmword ptr [rbp + 0x23c]
1407edd04: 0f11440844 movups xmmword ptr [rax + rcx + 0x44], xmm0
1407edd09: 0f114c0854 movups xmmword ptr [rax + rcx + 0x54], xmm1
1407edd0e: 0f11540864 movups xmmword ptr [rax + rcx + 0x64], xmm2
1407edd13: 0f115c0870 movups xmmword ptr [rax + rcx + 0x70], xmm3
1407edd18: c784088000000000000000 mov dword ptr [rax + rcx + 0x80], 0
1407edd23: 488b557c mov rdx, qword ptr [rbp + 0x7c]
1407edd27: 4889940884000000 mov qword ptr [rax + rcx + 0x84], rdx
1407edd2f: 8b9584000000 mov edx, dword ptr [rbp + 0x84]
1407edd35: 8994088c000000 mov dword ptr [rax + rcx + 0x8c], edx
1407edd3c: c784089000000000000000 mov dword ptr [rax + rcx + 0x90], 0
1407edd47: 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
1407edd4e: 4889940894000000 mov qword ptr [rax + rcx + 0x94], rdx
1407edd56: 8b9590000000 mov edx, dword ptr [rbp + 0x90]
1407edd5c: 8994089c000000 mov dword ptr [rax + rcx + 0x9c], edx
1407edd63: 48899c08a0000000 mov qword ptr [rax + rcx + 0xa0], rbx
1407edd6b: 48c78408a80000002e000000 mov qword ptr [rax + rcx + 0xa8], 0x2e
1407edd77: 488d1582c82902 lea rdx, [rip + 0x229c882]
1407edd7e: 48899408b0000000 mov qword ptr [rax + rcx + 0xb0], rdx
1407edd86: 48c78408b80000001e000000 mov qword ptr [rax + rcx + 0xb8], 0x1e
1407edd92: 0f2885c0010000 movaps xmm0, xmmword ptr [rbp + 0x1c0]
1407edd99: 0f288dd0010000 movaps xmm1, xmmword ptr [rbp + 0x1d0]
1407edda0: 0f2895e0010000 movaps xmm2, xmmword ptr [rbp + 0x1e0]
1407edda7: 0f299408e0000000 movaps xmmword ptr [rax + rcx + 0xe0], xmm2
1407eddaf: 0f298c08d0000000 movaps xmmword ptr [rax + rcx + 0xd0], xmm1
1407eddb7: 0f298408c0000000 movaps xmmword ptr [rax + rcx + 0xc0], xmm0
1407eddbf: c78408f000000001000000 mov dword ptr [rax + rcx + 0xf0], 1
1407eddca: 8b9590010000 mov edx, dword ptr [rbp + 0x190]
1407eddd0: 899408f4000000 mov dword ptr [rax + rcx + 0xf4], edx
1407eddd7: c78408f800000000000000 mov dword ptr [rax + rcx + 0xf8], 0
1407edde2: 488b95a8000000 mov rdx, qword ptr [rbp + 0xa8]
1407edde9: 488994080c010000 mov qword ptr [rax + rcx + 0x10c], rdx
1407eddf1: 0f108598000000 movups xmm0, xmmword ptr [rbp + 0x98]
1407eddf8: 0f118408fc000000 movups xmmword ptr [rax + rcx + 0xfc], xmm0
1407ede00: c684081401000002 mov byte ptr [rax + rcx + 0x114], 2
1407ede08: 0fb695ff000000 movzx edx, byte ptr [rbp + 0xff]
1407ede0f: 88940817010000 mov byte ptr [rax + rcx + 0x117], dl
1407ede16: 0fb795fd000000 movzx edx, word ptr [rbp + 0xfd]
1407ede1d: 6689940815010000 mov word ptr [rax + rcx + 0x115], dx
1407ede25: c784081801000000000000 mov dword ptr [rax + rcx + 0x118], 0
1407ede30: 0f108598010000 movups xmm0, xmmword ptr [rbp + 0x198]
1407ede37: 0f108da8010000 movups xmm1, xmmword ptr [rbp + 0x1a8]
1407ede3e: 0f1184081c010000 movups xmmword ptr [rax + rcx + 0x11c], xmm0
1407ede46: 0f118c082c010000 movups xmmword ptr [rax + rcx + 0x12c], xmm1
1407ede4e: c684083c01000002 mov byte ptr [rax + rcx + 0x13c], 2
1407ede56: 488b9578010000 mov rdx, qword ptr [rbp + 0x178]
1407ede5d: 4c8b857f010000 mov r8, qword ptr [rbp + 0x17f]
1407ede64: 4c89840844010000 mov qword ptr [rax + rcx + 0x144], r8
1407ede6c: 488994083d010000 mov qword ptr [rax + rcx + 0x13d], rdx
1407ede74: c684084c01000002 mov byte ptr [rax + rcx + 0x14c], 2
1407ede7c: 0fb695f8010000 movzx edx, byte ptr [rbp + 0x1f8]
1407ede83: 8894084e010000 mov byte ptr [rax + rcx + 0x14e], dl
1407ede8a: 66c784084f0100000009 mov word ptr [rax + rcx + 0x14f], 0x900
1407ede94: 488b9500010000 mov rdx, qword ptr [rbp + 0x100]
1407ede9b: 4c8b8507010000 mov r8, qword ptr [rbp + 0x107]
1407edea2: 4889940851010000 mov qword ptr [rax + rcx + 0x151], rdx
1407edeaa: 4c89840858010000 mov qword ptr [rax + rcx + 0x158], r8
1407edeb2: 48ffc6 inc rsi
1407edeb5: 4989b1c0000000 mov qword ptr [r9 + 0xc0], rsi
1407edebc: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407edec3: 8378b802 cmp dword ptr [rax - 0x48], 2
1407edec7: 0f82fe030000 jb 0x1407ee2cb
1407edecd: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407eded4: 0fb670fc movzx esi, byte ptr [rax - 4]
1407eded8: 0f288510010000 movaps xmm0, xmmword ptr [rbp + 0x110]
1407ededf: 0f288d20010000 movaps xmm1, xmmword ptr [rbp + 0x120]
1407edee6: 0f289530010000 movaps xmm2, xmmword ptr [rbp + 0x130]
1407edeed: 0f299530020000 movaps xmmword ptr [rbp + 0x230], xmm2
1407edef4: 0f298d20020000 movaps xmmword ptr [rbp + 0x220], xmm1
1407edefb: 0f298510020000 movaps xmmword ptr [rbp + 0x210], xmm0
1407edf02: 488b4550 mov rax, qword ptr [rbp + 0x50]
1407edf06: 4889442420 mov qword ptr [rsp + 0x20], rax
1407edf0b: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407edf12: 89f2 mov edx, esi
1407edf14: 440fb68580020000 movzx r8d, byte ptr [rbp + 0x280]
1407edf1c: 4c8d8d10020000 lea r9, [rbp + 0x210]
1407edf23: e888180000 call 0x1407ef7b0 ; system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
1407edf28: 488b85c0010000 mov rax, qword ptr [rbp + 0x1c0]
1407edf2f: 48898590010000 mov qword ptr [rbp + 0x190], rax
1407edf36: 488b85c8010000 mov rax, qword ptr [rbp + 0x1c8]
1407edf3d: 48898570020000 mov qword ptr [rbp + 0x270], rax
1407edf44: 488b85d0010000 mov rax, qword ptr [rbp + 0x1d0]
1407edf4b: 4885c0 test rax, rax
1407edf4e: 0f8440030000 je 0x1407ee294
1407edf54: 488d0440 lea rax, [rax + rax*2]
1407edf58: 48c1e004 shl rax, 4
1407edf5c: 488b9d70020000 mov rbx, qword ptr [rbp + 0x270]
1407edf63: 4801d8 add rax, rbx
1407edf66: 48898568020000 mov qword ptr [rbp + 0x268], rax
1407edf6d: e96d010000 jmp 0x1407ee0df ; unknown
1407edf72: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407edf80: 4883c330 add rbx, 0x30
1407edf84: 440f14c7 unpcklps xmm8, xmm7
1407edf88: 440f58c6 addps xmm8, xmm6
1407edf8c: 488d05c9ce2902 lea rax, [rip + 0x229cec9]
1407edf93: 4901c5 add r13, rax
1407edf96: 4c8b8da8020000 mov r9, qword ptr [rbp + 0x2a8]
1407edf9d: 498b81b8000000 mov rax, qword ptr [r9 + 0xb8]
1407edfa4: 4869ce60010000 imul rcx, rsi, 0x160
1407edfab: 48c7040800000000 mov qword ptr [rax + rcx], 0
1407edfb3: 48c744081000000000 mov qword ptr [rax + rcx + 0x10], 0
1407edfbc: 48c744082000000000 mov qword ptr [rax + rcx + 0x20], 0
1407edfc5: c744083000000000 mov dword ptr [rax + rcx + 0x30], 0
1407edfcd: c744084000000000 mov dword ptr [rax + rcx + 0x40], 0
1407edfd5: c784088000000000000000 mov dword ptr [rax + rcx + 0x80], 0
1407edfe0: c784089000000000000000 mov dword ptr [rax + rcx + 0x90], 0
1407edfeb: 4c89ac08a0000000 mov qword ptr [rax + rcx + 0xa0], r13
1407edff3: 48c78408a80000002c000000 mov qword ptr [rax + rcx + 0xa8], 0x2c
1407edfff: 488d15fe314302 lea rdx, [rip + 0x24331fe]
1407ee006: 48899408b0000000 mov qword ptr [rax + rcx + 0xb0], rdx
1407ee00e: 48c78408b800000017000000 mov qword ptr [rax + rcx + 0xb8], 0x17
1407ee01a: 440f299c08c0000000 movaps xmmword ptr [rax + rcx + 0xc0], xmm11
1407ee023: 440f138408d0000000 movlps qword ptr [rax + rcx + 0xd0], xmm8
1407ee02c: f3440f118c08d8000000 movss dword ptr [rax + rcx + 0xd8], xmm9
1407ee036: 0f288510020000 movaps xmm0, xmmword ptr [rbp + 0x210]
1407ee03d: 0f118408dc000000 movups xmmword ptr [rax + rcx + 0xdc], xmm0
1407ee045: 8b9520020000 mov edx, dword ptr [rbp + 0x220]
1407ee04b: 899408ec000000 mov dword ptr [rax + rcx + 0xec], edx
1407ee052: c78408f000000000000000 mov dword ptr [rax + rcx + 0xf0], 0
1407ee05d: c78408f800000000000000 mov dword ptr [rax + rcx + 0xf8], 0
1407ee068: c684081401000002 mov byte ptr [rax + rcx + 0x114], 2
1407ee070: c784081801000000000000 mov dword ptr [rax + rcx + 0x118], 0
1407ee07b: c684083c01000002 mov byte ptr [rax + rcx + 0x13c], 2
1407ee083: 488b9598010000 mov rdx, qword ptr [rbp + 0x198]
1407ee08a: 4c8b859f010000 mov r8, qword ptr [rbp + 0x19f]
1407ee091: 4c89840844010000 mov qword ptr [rax + rcx + 0x144], r8
1407ee099: 488994083d010000 mov qword ptr [rax + rcx + 0x13d], rdx
1407ee0a1: c684084c01000002 mov byte ptr [rax + rcx + 0x14c], 2
1407ee0a9: 0fb695f8010000 movzx edx, byte ptr [rbp + 0x1f8]
1407ee0b0: 8894084e010000 mov byte ptr [rax + rcx + 0x14e], dl
1407ee0b7: 66c784084f0100000009 mov word ptr [rax + rcx + 0x14f], 0x900
1407ee0c1: 48ffc6 inc rsi
1407ee0c4: 4989b1c0000000 mov qword ptr [r9 + 0xc0], rsi
1407ee0cb: 483b9d68020000 cmp rbx, qword ptr [rbp + 0x268]
1407ee0d2: 4c8badb0000000 mov r13, qword ptr [rbp + 0xb0]
1407ee0d9: 0f84b5010000 je 0x1407ee294
1407ee0df: 440f281b movaps xmm11, xmmword ptr [rbx]
1407ee0e3: f30f107b14 movss xmm7, dword ptr [rbx + 0x14]
1407ee0e8: f30f583dc88b2202 addss xmm7, dword ptr [rip + 0x2228bc8]
1407ee0f0: 410f28cb movaps xmm1, xmm11
1407ee0f4: 410fc6cbd2 shufps xmm1, xmm11, 0xd2
1407ee0f9: 0f590d10be2902 mulps xmm1, xmmword ptr [rip + 0x229be10]
1407ee100: 410f28c3 movaps xmm0, xmm11
1407ee104: 410fc6c3c9 shufps xmm0, xmm11, 0xc9
1407ee109: 410f59c7 mulps xmm0, xmm15
1407ee10d: 48b82f64bd78641d76a0 movabs rax, 0xa0761d6478bd642f
1407ee117: 4c8d0407 lea r8, [rdi + rax]
1407ee11b: 4c89c0 mov rax, r8
1407ee11e: 48b9db28b4a0d17e03e7 movabs rcx, 0xe7037ed1a0b428db
1407ee128: 4831c8 xor rax, rcx
1407ee12b: 49f7e0 mul r8
1407ee12e: 4889d1 mov rcx, rdx
1407ee131: 4831c1 xor rcx, rax
1407ee134: 4889c8 mov rax, rcx
1407ee137: ba03000000 mov edx, 3
1407ee13c: 48f7e2 mul rdx
1407ee13f: 4885c9 test rcx, rcx
1407ee142: 754b jne 0x1407ee18f
1407ee144: 48b85ec87af1c83aec40 movabs rax, 0x40ec3ac8f17ac85e
1407ee14e: 4801c7 add rdi, rax
1407ee151: 48b92f64bd78641d76a0 movabs rcx, 0xa0761d6478bd642f
1407ee15b: 49b9db28b4a0d17e03e7 movabs r9, 0xe7037ed1a0b428db
1407ee165: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407ee170: 4901c8 add r8, rcx
1407ee173: 4889f8 mov rax, rdi
1407ee176: 4c31c8 xor rax, r9
1407ee179: 48f7e7 mul rdi
1407ee17c: 4801cf add rdi, rcx
1407ee17f: 4831c2 xor rdx, rax
1407ee182: 74ec je 0x1407ee170
1407ee184: 4889d0 mov rax, rdx
1407ee187: b903000000 mov ecx, 3
1407ee18c: 48f7e1 mul rcx
1407ee18f: 4c89c7 mov rdi, r8
1407ee192: 488d431c lea rax, [rbx + 0x1c]
1407ee196: 410f28d3 movaps xmm2, xmm11
1407ee19a: 410f59d3 mulps xmm2, xmm11
1407ee19e: f30f16da movshdup xmm3, xmm2
1407ee1a2: f30f58da addss xmm3, xmm2
1407ee1a6: 0f28e2 movaps xmm4, xmm2
1407ee1a9: 660f15e2 unpckhpd xmm4, xmm2
1407ee1ad: f30f58e3 addss xmm4, xmm3
1407ee1b1: 0fc6e400 shufps xmm4, xmm4, 0
1407ee1b5: 0fc6d2ff shufps xmm2, xmm2, 0xff
1407ee1b9: 0f5cd4 subps xmm2, xmm4
1407ee1bc: 410f59d5 mulps xmm2, xmm13
1407ee1c0: 410f28db movaps xmm3, xmm11
1407ee1c4: 410f59dd mulps xmm3, xmm13
1407ee1c8: f30f16e3 movshdup xmm4, xmm3
1407ee1cc: f30f58e3 addss xmm4, xmm3
1407ee1d0: 0f12db movhlps xmm3, xmm3
1407ee1d3: f30f58dc addss xmm3, xmm4
1407ee1d7: f30f58db addss xmm3, xmm3
1407ee1db: 0fc6db00 shufps xmm3, xmm3, 0
1407ee1df: 410f59db mulps xmm3, xmm11
1407ee1e3: 0f58da addps xmm3, xmm2
1407ee1e6: 0f5cc1 subps xmm0, xmm1
1407ee1e9: 410f28f3 movaps xmm6, xmm11
1407ee1ed: 410f58f3 addps xmm6, xmm11
1407ee1f1: 0fc6f6ff shufps xmm6, xmm6, 0xff
1407ee1f5: 0f59f0 mulps xmm6, xmm0
1407ee1f8: 0f58f3 addps xmm6, xmm3
1407ee1fb: f3440f104310 movss xmm8, dword ptr [rbx + 0x10]
1407ee201: 440f28ce movaps xmm9, xmm6
1407ee205: 66440f15ce unpckhpd xmm9, xmm6
1407ee20a: f3440f584b18 addss xmm9, dword ptr [rbx + 0x18]
1407ee210: 488d0d45cc2902 lea rcx, [rip + 0x229cc45]
1407ee217: 4c632c91 movsxd r13, dword ptr [rcx + rdx*4]
1407ee21b: f30f6f00 movdqu xmm0, xmmword ptr [rax]
1407ee21f: 660f7f8510020000 movdqa xmmword ptr [rbp + 0x210], xmm0
1407ee227: 8b4010 mov eax, dword ptr [rax + 0x10]
1407ee22a: 898520020000 mov dword ptr [rbp + 0x220], eax
1407ee230: 488d85c3010000 lea rax, [rbp + 0x1c3]
1407ee237: c7400800000000 mov dword ptr [rax + 8], 0
1407ee23e: 48c70000000000 mov qword ptr [rax], 0
1407ee245: 488b85c0010000 mov rax, qword ptr [rbp + 0x1c0]
1407ee24c: 488b8dc7010000 mov rcx, qword ptr [rbp + 0x1c7]
1407ee253: 48898d9f010000 mov qword ptr [rbp + 0x19f], rcx
1407ee25a: 48898598010000 mov qword ptr [rbp + 0x198], rax
1407ee261: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
1407ee268: 488bb0c0000000 mov rsi, qword ptr [rax + 0xc0]
1407ee26f: 483bb0b0000000 cmp rsi, qword ptr [rax + 0xb0]
1407ee276: 0f8504fdffff jne 0x1407edf80
1407ee27c: 488b8d08020000 mov rcx, qword ptr [rbp + 0x208]
1407ee283: 488d1536c62902 lea rdx, [rip + 0x229c636]
1407ee28a: e821d10300 call 0x14082b3b0 ; alloc::raw_vec::RawVec::grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
1407ee28f: e9ecfcffff jmp 0x1407edf80 ; unknown
1407ee294: 488b8590010000 mov rax, qword ptr [rbp + 0x190]
1407ee29b: 4885c0 test rax, rax
1407ee29e: 741a je 0x1407ee2ba
1407ee2a0: 48c1e004 shl rax, 4
1407ee2a4: 488d1440 lea rdx, [rax + rax*2]
1407ee2a8: 41b810000000 mov r8d, 0x10
1407ee2ae: 488b8d70020000 mov rcx, qword ptr [rbp + 0x270]
1407ee2b5: e86659e2ff call 0x140613c20 ; unknown
1407ee2ba: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407ee2c1: 8378b801 cmp dword ptr [rax - 0x48], 1
1407ee2c5: 0f875fe1ffff ja 0x1407ec42a
1407ee2cb: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ee2d2: 8b48e4 mov ecx, dword ptr [rax - 0x1c]
1407ee2d5: e8667f4a00 call 0x140c96240 ; utils::rng::init_rng
1407ee2da: 4889c6 mov rsi, rax
1407ee2dd: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ee2e4: 6940e4b6aaaaaa imul eax, dword ptr [rax - 0x1c], 0xaaaaaab6
1407ee2eb: 054b885655 add eax, 0x5556884b
1407ee2f0: 3d55555555 cmp eax, 0x55555555
1407ee2f5: 0f87aa000000 ja 0x1407ee3a5
1407ee2fb: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ee302: 8078fc03 cmp byte ptr [rax - 4], 3
1407ee306: 0f8399000000 jae 0x1407ee3a5
1407ee30c: 488bbdf0010000 mov rdi, qword ptr [rbp + 0x1f0]
1407ee313: 0fb605ef68e402 movzx eax, byte ptr [rip + 0x2e468ef]
1407ee31a: b950000000 mov ecx, 0x50
1407ee31f: ba10000000 mov edx, 0x10
1407ee324: e8e758e2ff call 0x140613c10 ; unknown
1407ee329: 4885c0 test rax, rax
1407ee32c: 0f841c0b0000 je 0x1407eee4e
1407ee332: 4889c6 mov rsi, rax
1407ee335: 0f288510010000 movaps xmm0, xmmword ptr [rbp + 0x110]
1407ee33c: 0f288d20010000 movaps xmm1, xmmword ptr [rbp + 0x120]
1407ee343: 0f289530010000 movaps xmm2, xmmword ptr [rbp + 0x130]
1407ee34a: 0f295020 movaps xmmword ptr [rax + 0x20], xmm2
1407ee34e: 0f294810 movaps xmmword ptr [rax + 0x10], xmm1
1407ee352: 0f2900 movaps xmmword ptr [rax], xmm0
1407ee355: 48897830 mov qword ptr [rax + 0x30], rdi
1407ee359: f30f108590020000 movss xmm0, dword ptr [rbp + 0x290]
1407ee361: f30f114038 movss dword ptr [rax + 0x38], xmm0
1407ee366: f3440f11503c movss dword ptr [rax + 0x3c], xmm10
1407ee36c: 488b8578020000 mov rax, qword ptr [rbp + 0x278]
1407ee373: 894640 mov dword ptr [rsi + 0x40], eax
1407ee376: 488bbda8020000 mov rdi, qword ptr [rbp + 0x2a8]
1407ee37d: 488b8f10010000 mov rcx, qword ptr [rdi + 0x110]
1407ee384: 4885c9 test rcx, rcx
1407ee387: 7410 je 0x1407ee399
1407ee389: ba50000000 mov edx, 0x50
1407ee38e: 41b810000000 mov r8d, 0x10
1407ee394: e88758e2ff call 0x140613c20 ; unknown
1407ee399: 4889b710010000 mov qword ptr [rdi + 0x110], rsi
1407ee3a0: e929030000 jmp 0x1407ee6ce ; unknown
1407ee3a5: 48b82f64bd78641d76a0 movabs rax, 0xa0761d6478bd642f
1407ee3af: 4801c6 add rsi, rax
1407ee3b2: 4889f0 mov rax, rsi
1407ee3b5: 48b9db28b4a0d17e03e7 movabs rcx, 0xe7037ed1a0b428db
1407ee3bf: 4831c8 xor rax, rcx
1407ee3c2: 48f7e6 mul rsi
1407ee3c5: 31d0 xor eax, edx
1407ee3c7: c1e81e shr eax, 0x1e
1407ee3ca: 83f801 cmp eax, 1
1407ee3cd: 0f85fb020000 jne 0x1407ee6ce
1407ee3d3: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ee3da: 0fb650fc movzx edx, byte ptr [rax - 4]
1407ee3de: 0f288510010000 movaps xmm0, xmmword ptr [rbp + 0x110]
1407ee3e5: 0f288d20010000 movaps xmm1, xmmword ptr [rbp + 0x120]
1407ee3ec: 0f289530010000 movaps xmm2, xmmword ptr [rbp + 0x130]
1407ee3f3: 0f299530020000 movaps xmmword ptr [rbp + 0x230], xmm2
1407ee3fa: 0f298d20020000 movaps xmmword ptr [rbp + 0x220], xmm1
1407ee401: 0f298510020000 movaps xmmword ptr [rbp + 0x210], xmm0
1407ee408: 488b4550 mov rax, qword ptr [rbp + 0x50]
1407ee40c: 4889442420 mov qword ptr [rsp + 0x20], rax
1407ee411: 488d8d50010000 lea rcx, [rbp + 0x150]
1407ee418: 440fb68580020000 movzx r8d, byte ptr [rbp + 0x280]
1407ee420: 4c8d8d10020000 lea r9, [rbp + 0x210]
1407ee427: e884130000 call 0x1407ef7b0 ; system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
1407ee42c: 488b8558010000 mov rax, qword ptr [rbp + 0x158]
1407ee433: 48898570020000 mov qword ptr [rbp + 0x270], rax
1407ee43a: 488b8560010000 mov rax, qword ptr [rbp + 0x160]
1407ee441: 4885c0 test rax, rax
1407ee444: 0f8457020000 je 0x1407ee6a1
1407ee44a: 488d0440 lea rax, [rax + rax*2]
1407ee44e: 48c1e004 shl rax, 4
1407ee452: 488b9d70020000 mov rbx, qword ptr [rbp + 0x270]
1407ee459: 4801d8 add rax, rbx
1407ee45c: 48898500020000 mov qword ptr [rbp + 0x200], rax
1407ee463: 488b85b8010000 mov rax, qword ptr [rbp + 0x1b8]
1407ee46a: 4805f8000000 add rax, 0xf8
1407ee470: 48898508020000 mov qword ptr [rbp + 0x208], rax
1407ee477: 31ff xor edi, edi
1407ee479: e989000000 jmp 0x1407ee507 ; unknown
1407ee47e: 6690 nop 
1407ee480: 4883c330 add rbx, 0x30
1407ee484: 48ffc7 inc rdi
1407ee487: 488b95a8020000 mov rdx, qword ptr [rbp + 0x2a8]
1407ee48e: 488b8200010000 mov rax, qword ptr [rdx + 0x100]
1407ee495: 4a8d0cad00000000 lea rcx, [r13*4]
1407ee49d: 4c01e9 add rcx, r13
1407ee4a0: 48c1e104 shl rcx, 4
1407ee4a4: c6040801 mov byte ptr [rax + rcx], 1
1407ee4a8: 4c8b85f8010000 mov r8, qword ptr [rbp + 0x1f8]
1407ee4af: 4c89440808 mov qword ptr [rax + rcx + 8], r8
1407ee4b4: 0f2885c0010000 movaps xmm0, xmmword ptr [rbp + 0x1c0]
1407ee4bb: 0f288dd0010000 movaps xmm1, xmmword ptr [rbp + 0x1d0]
1407ee4c2: 0f2895e0010000 movaps xmm2, xmmword ptr [rbp + 0x1e0]
1407ee4c9: 0f29440810 movaps xmmword ptr [rax + rcx + 0x10], xmm0
1407ee4ce: 0f294c0820 movaps xmmword ptr [rax + rcx + 0x20], xmm1
1407ee4d3: 0f29540830 movaps xmmword ptr [rax + rcx + 0x30], xmm2
1407ee4d8: 4c8b8580020000 mov r8, qword ptr [rbp + 0x280]
1407ee4df: 4c89440840 mov qword ptr [rax + rcx + 0x40], r8
1407ee4e4: 4c8b8568020000 mov r8, qword ptr [rbp + 0x268]
1407ee4eb: 4c89440848 mov qword ptr [rax + rcx + 0x48], r8
1407ee4f0: 49ffc5 inc r13
1407ee4f3: 4c89aa08010000 mov qword ptr [rdx + 0x108], r13
1407ee4fa: 483b9d00020000 cmp rbx, qword ptr [rbp + 0x200]
1407ee501: 0f849a010000 je 0x1407ee6a1
1407ee507: 660f6f03 movdqa xmm0, xmmword ptr [rbx]
1407ee50b: 660f6f4b10 movdqa xmm1, xmmword ptr [rbx + 0x10]
1407ee510: 660f6f5320 movdqa xmm2, xmmword ptr [rbx + 0x20]
1407ee515: 660f7f9530020000 movdqa xmmword ptr [rbp + 0x230], xmm2
1407ee51d: 660f7f8d20020000 movdqa xmmword ptr [rbp + 0x220], xmm1
1407ee525: 660f7f8510020000 movdqa xmmword ptr [rbp + 0x210], xmm0
1407ee52d: 488b8578020000 mov rax, qword ptr [rbp + 0x278]
1407ee534: 8985a8010000 mov dword ptr [rbp + 0x1a8], eax
1407ee53a: 488d0597c32902 lea rax, [rip + 0x229c397]
1407ee541: 48898598010000 mov qword ptr [rbp + 0x198], rax
1407ee548: 48c785a00100000a000000 mov qword ptr [rbp + 0x1a0], 0xa
1407ee553: 4889bdb0010000 mov qword ptr [rbp + 0x1b0], rdi
1407ee55a: 488d8d98010000 lea rcx, [rbp + 0x198]
1407ee561: e80ae5a900 call 0x14128ca70 ; utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$>,usize> >
1407ee566: 48898580020000 mov qword ptr [rbp + 0x280], rax
1407ee56d: 48b82f64bd78641d76a0 movabs rax, 0xa0761d6478bd642f
1407ee577: 4c8d0406 lea r8, [rsi + rax]
1407ee57b: 4c89c0 mov rax, r8
1407ee57e: 48b9db28b4a0d17e03e7 movabs rcx, 0xe7037ed1a0b428db
1407ee588: 4831c8 xor rax, rcx
1407ee58b: 49f7e0 mul r8
1407ee58e: 4889d1 mov rcx, rdx
1407ee591: 4831c1 xor rcx, rax
1407ee594: 4889c8 mov rax, rcx
1407ee597: ba06000000 mov edx, 6
1407ee59c: 48f7e2 mul rdx
1407ee59f: 4801c9 add rcx, rcx
1407ee5a2: 488d0449 lea rax, [rcx + rcx*2]
1407ee5a6: 4883f803 cmp rax, 3
1407ee5aa: 774f ja 0x1407ee5fb
1407ee5ac: 48b85ec87af1c83aec40 movabs rax, 0x40ec3ac8f17ac85e
1407ee5b6: 4801c6 add rsi, rax
1407ee5b9: 48b92f64bd78641d76a0 movabs rcx, 0xa0761d6478bd642f
1407ee5c3: 49b9db28b4a0d17e03e7 movabs r9, 0xe7037ed1a0b428db
1407ee5cd: 0f1f00 nop dword ptr [rax]
1407ee5d0: 4889f0 mov rax, rsi
1407ee5d3: 4c31c8 xor rax, r9
1407ee5d6: 48f7e6 mul rsi
1407ee5d9: 4901c8 add r8, rcx
1407ee5dc: 4831c2 xor rdx, rax
1407ee5df: 488d0412 lea rax, [rdx + rdx]
1407ee5e3: 488d0440 lea rax, [rax + rax*2]
1407ee5e7: 4801ce add rsi, rcx
1407ee5ea: 4883f804 cmp rax, 4
1407ee5ee: 72e0 jb 0x1407ee5d0
1407ee5f0: 4889d0 mov rax, rdx
1407ee5f3: b906000000 mov ecx, 6
1407ee5f8: 48f7e1 mul rcx
1407ee5fb: 4c89c6 mov rsi, r8
1407ee5fe: 488995f8010000 mov qword ptr [rbp + 0x1f8], rdx
1407ee605: 660f6f8510020000 movdqa xmm0, xmmword ptr [rbp + 0x210]
1407ee60d: 660f6f8d20020000 movdqa xmm1, xmmword ptr [rbp + 0x220]
1407ee615: 660f6f9530020000 movdqa xmm2, xmmword ptr [rbp + 0x230]
1407ee61d: 660f7f95e0010000 movdqa xmmword ptr [rbp + 0x1e0], xmm2
1407ee625: 660f7f8dd0010000 movdqa xmmword ptr [rbp + 0x1d0], xmm1
1407ee62d: 660f7f85c0010000 movdqa xmmword ptr [rbp + 0x1c0], xmm0
1407ee635: 488b85f0010000 mov rax, qword ptr [rbp + 0x1f0]
1407ee63c: 48898568020000 mov qword ptr [rbp + 0x268], rax
1407ee643: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
1407ee64a: 4c8ba808010000 mov r13, qword ptr [rax + 0x108]
1407ee651: 4c3ba8f8000000 cmp r13, qword ptr [rax + 0xf8]
1407ee658: 0f8522feffff jne 0x1407ee480
1407ee65e: 488b8d08020000 mov rcx, qword ptr [rbp + 0x208]
1407ee665: 488d157cc22902 lea rdx, [rip + 0x229c27c]
1407ee66c: e88fe3f9ff call 0x14078ca00 ; alloc::raw_vec::RawVec::grow_one<tuple$<glam::f32::affine3a::Affine3A,u32,array$<f32,2> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc
1407ee671: e90afeffff jmp 0x1407ee480 ; unknown
1407ee676: 4080fe03 cmp sil, 3
1407ee67a: 0f856e030000 jne 0x1407ee9ee
1407ee680: 48c7854801000000000000 mov qword ptr [rbp + 0x148], 0
1407ee68b: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407ee692: 8378b802 cmp dword ptr [rax - 0x48], 2
1407ee696: 0f8331f8ffff jae 0x1407edecd
1407ee69c: e92afcffff jmp 0x1407ee2cb ; unknown
1407ee6a1: 488b8550010000 mov rax, qword ptr [rbp + 0x150]
1407ee6a8: 4885c0 test rax, rax
1407ee6ab: 741a je 0x1407ee6c7
1407ee6ad: 48c1e004 shl rax, 4
1407ee6b1: 488d1440 lea rdx, [rax + rax*2]
1407ee6b5: 41b810000000 mov r8d, 0x10
1407ee6bb: 488b8d70020000 mov rcx, qword ptr [rbp + 0x270]
1407ee6c2: e85955e2ff call 0x140613c20 ; unknown
1407ee6c7: 4c8badb0000000 mov r13, qword ptr [rbp + 0xb0]
1407ee6ce: 80bd4801000000 cmp byte ptr [rbp + 0x148], 0
1407ee6d5: 0f844fddffff je 0x1407ec42a
1407ee6db: 488185b8010000f8000000 add qword ptr [rbp + 0x1b8], 0xf8
1407ee6e6: f30f10b5d0000000 movss xmm6, dword ptr [rbp + 0xd0]
1407ee6ee: f30f5935ae152902 mulss xmm6, dword ptr [rip + 0x22915ae]
1407ee6f6: 488b8578020000 mov rax, qword ptr [rbp + 0x278]
1407ee6fd: 898510020000 mov dword ptr [rbp + 0x210], eax
1407ee703: 488d05f6c12902 lea rax, [rip + 0x229c1f6]
1407ee70a: 48898518020000 mov qword ptr [rbp + 0x218], rax
1407ee711: 48c785200200000d000000 mov qword ptr [rbp + 0x220], 0xd
1407ee71c: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ee723: e808e5a900 call 0x14128cc30 ; utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> > >
1407ee728: 48898580020000 mov qword ptr [rbp + 0x280], rax
1407ee72f: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ee736: 8b70e0 mov esi, dword ptr [rax - 0x20]
1407ee739: 488b85f0010000 mov rax, qword ptr [rbp + 0x1f0]
1407ee740: 488985f8010000 mov qword ptr [rbp + 0x1f8], rax
1407ee747: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
1407ee74e: 488bb808010000 mov rdi, qword ptr [rax + 0x108]
1407ee755: 483bb8f8000000 cmp rdi, qword ptr [rax + 0xf8]
1407ee75c: 7513 jne 0x1407ee771
1407ee75e: 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
1407ee765: 488d15a4c12902 lea rdx, [rip + 0x229c1a4]
1407ee76c: e88fe2f9ff call 0x14078ca00 ; alloc::raw_vec::RawVec::grow_one<tuple$<glam::f32::affine3a::Affine3A,u32,array$<f32,2> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc
1407ee771: 0f280568aa2902 movaps xmm0, xmmword ptr [rip + 0x229aa68]
1407ee778: f30f10c6 movss xmm0, xmm6
1407ee77c: 0f28d0 movaps xmm2, xmm0
1407ee77f: 0fc615a9b7290224 shufps xmm2, xmmword ptr [rip + 0x229b7a9], 0x24
1407ee787: 410f28f4 movaps xmm6, xmm12
1407ee78b: 410f59f4 mulps xmm6, xmm12
1407ee78f: f30f16ce movshdup xmm1, xmm6
1407ee793: f30f58ce addss xmm1, xmm6
1407ee797: 0f28de movaps xmm3, xmm6
1407ee79a: 660f15de unpckhpd xmm3, xmm6
1407ee79e: f30f58d9 addss xmm3, xmm1
1407ee7a2: 0fc6db00 shufps xmm3, xmm3, 0
1407ee7a6: 0fc6f6ff shufps xmm6, xmm6, 0xff
1407ee7aa: 0f5cf3 subps xmm6, xmm3
1407ee7ad: 0f59d6 mulps xmm2, xmm6
1407ee7b0: 410f28cc movaps xmm1, xmm12
1407ee7b4: 0f59c8 mulps xmm1, xmm0
1407ee7b7: f30f16d9 movshdup xmm3, xmm1
1407ee7bb: f30f58d9 addss xmm3, xmm1
1407ee7bf: 0f12c9 movhlps xmm1, xmm1
1407ee7c2: f30f58cb addss xmm1, xmm3
1407ee7c6: f30f58c9 addss xmm1, xmm1
1407ee7ca: 0fc6c900 shufps xmm1, xmm1, 0
1407ee7ce: 410f59cc mulps xmm1, xmm12
1407ee7d2: 0f58ca addps xmm1, xmm2
1407ee7d5: 410f28fc movaps xmm7, xmm12
1407ee7d9: 410fc6fcd2 shufps xmm7, xmm12, 0xd2
1407ee7de: 0f28d7 movaps xmm2, xmm7
1407ee7e1: 0f59d0 mulps xmm2, xmm0
1407ee7e4: 0fc6c0d2 shufps xmm0, xmm0, 0xd2
1407ee7e8: 410f59c4 mulps xmm0, xmm12
1407ee7ec: 0f5cd0 subps xmm2, xmm0
1407ee7ef: 0fc6d2d2 shufps xmm2, xmm2, 0xd2
1407ee7f3: 450f28cc movaps xmm9, xmm12
1407ee7f7: 450f58cc addps xmm9, xmm12
1407ee7fb: 450fc6c9ff shufps xmm9, xmm9, 0xff
1407ee800: 410f59d1 mulps xmm2, xmm9
1407ee804: 0f58d1 addps xmm2, xmm1
1407ee807: 0f284560 movaps xmm0, xmmword ptr [rbp + 0x60]
1407ee80b: 0f58c2 addps xmm0, xmm2
1407ee80e: 0f12d2 movhlps xmm2, xmm2
1407ee811: f30f589564020000 addss xmm2, dword ptr [rbp + 0x264]
1407ee819: 488b9da8020000 mov rbx, qword ptr [rbp + 0x2a8]
1407ee820: 488b8300010000 mov rax, qword ptr [rbx + 0x100]
1407ee827: 488d0cbf lea rcx, [rdi + rdi*4]
1407ee82b: 48c1e104 shl rcx, 4
1407ee82f: 66c704080200 mov word ptr [rax + rcx], 2
1407ee835: 89740804 mov dword ptr [rax + rcx + 4], esi
1407ee839: 440f11640810 movups xmmword ptr [rax + rcx + 0x10], xmm12
1407ee83f: 0f13440820 movlps qword ptr [rax + rcx + 0x20], xmm0
1407ee844: f30f11540828 movss dword ptr [rax + rcx + 0x28], xmm2
1407ee84a: 488d9520010000 lea rdx, [rbp + 0x120]
1407ee851: f30f6f420c movdqu xmm0, xmmword ptr [rdx + 0xc]
1407ee856: f30f7f44082c movdqu xmmword ptr [rax + rcx + 0x2c], xmm0
1407ee85c: 8b521c mov edx, dword ptr [rdx + 0x1c]
1407ee85f: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407ee863: 488b9580020000 mov rdx, qword ptr [rbp + 0x280]
1407ee86a: 4889540840 mov qword ptr [rax + rcx + 0x40], rdx
1407ee86f: 488b95f8010000 mov rdx, qword ptr [rbp + 0x1f8]
1407ee876: 4889540848 mov qword ptr [rax + rcx + 0x48], rdx
1407ee87b: 48ffc7 inc rdi
1407ee87e: 4889bb08010000 mov qword ptr [rbx + 0x108], rdi
1407ee885: f3440f1085d0000000 movss xmm8, dword ptr [rbp + 0xd0]
1407ee88e: f3440f5905058f2902 mulss xmm8, dword ptr [rip + 0x2298f05]
1407ee897: 488b8578020000 mov rax, qword ptr [rbp + 0x278]
1407ee89e: 898510020000 mov dword ptr [rbp + 0x210], eax
1407ee8a4: 488d057dc02902 lea rax, [rip + 0x229c07d]
1407ee8ab: 48898518020000 mov qword ptr [rbp + 0x218], rax
1407ee8b2: 48c785200200000c000000 mov qword ptr [rbp + 0x220], 0xc
1407ee8bd: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ee8c4: e867e3a900 call 0x14128cc30 ; utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> > >
1407ee8c9: 48898578020000 mov qword ptr [rbp + 0x278], rax
1407ee8d0: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ee8d7: 8b70e0 mov esi, dword ptr [rax - 0x20]
1407ee8da: 488b85f0010000 mov rax, qword ptr [rbp + 0x1f0]
1407ee8e1: 48898580020000 mov qword ptr [rbp + 0x280], rax
1407ee8e8: 488bbb08010000 mov rdi, qword ptr [rbx + 0x108]
1407ee8ef: 483bbbf8000000 cmp rdi, qword ptr [rbx + 0xf8]
1407ee8f6: 7513 jne 0x1407ee90b
1407ee8f8: 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
1407ee8ff: 488d1532c02902 lea rdx, [rip + 0x229c032]
1407ee906: e8f5e0f9ff call 0x14078ca00 ; alloc::raw_vec::RawVec::grow_one<tuple$<glam::f32::affine3a::Affine3A,u32,array$<f32,2> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc
1407ee90b: 0f2805cea82902 movaps xmm0, xmmword ptr [rip + 0x229a8ce]
1407ee912: f3410f10c0 movss xmm0, xmm8
1407ee917: 0f28c8 movaps xmm1, xmm0
1407ee91a: 0fc60d0eb6290224 shufps xmm1, xmmword ptr [rip + 0x229b60e], 0x24
1407ee922: 0f59f1 mulps xmm6, xmm1
1407ee925: 410f28cc movaps xmm1, xmm12
1407ee929: 0f59c8 mulps xmm1, xmm0
1407ee92c: f30f16d1 movshdup xmm2, xmm1
1407ee930: f30f58d1 addss xmm2, xmm1
1407ee934: 0f12c9 movhlps xmm1, xmm1
1407ee937: f30f58ca addss xmm1, xmm2
1407ee93b: f30f58c9 addss xmm1, xmm1
1407ee93f: 0fc6c900 shufps xmm1, xmm1, 0
1407ee943: 410f59cc mulps xmm1, xmm12
1407ee947: 0f58ce addps xmm1, xmm6
1407ee94a: 0f59f8 mulps xmm7, xmm0
1407ee94d: 0fc6c0d2 shufps xmm0, xmm0, 0xd2
1407ee951: 410f59c4 mulps xmm0, xmm12
1407ee955: 0f5cf8 subps xmm7, xmm0
1407ee958: 0fc6ffd2 shufps xmm7, xmm7, 0xd2
1407ee95c: 440f59cf mulps xmm9, xmm7
1407ee960: 440f58c9 addps xmm9, xmm1
1407ee964: 0f284560 movaps xmm0, xmmword ptr [rbp + 0x60]
1407ee968: 410f58c1 addps xmm0, xmm9
1407ee96c: 450f12c9 movhlps xmm9, xmm9
1407ee970: f30f108d64020000 movss xmm1, dword ptr [rbp + 0x264]
1407ee978: f3410f58c9 addss xmm1, xmm9
1407ee97d: 4c8b85a8020000 mov r8, qword ptr [rbp + 0x2a8]
1407ee984: 498b8000010000 mov rax, qword ptr [r8 + 0x100]
1407ee98b: 488d0cbf lea rcx, [rdi + rdi*4]
1407ee98f: 48c1e104 shl rcx, 4
1407ee993: 66c704080201 mov word ptr [rax + rcx], 0x102
1407ee999: 89740804 mov dword ptr [rax + rcx + 4], esi
1407ee99d: 440f11640810 movups xmmword ptr [rax + rcx + 0x10], xmm12
1407ee9a3: 0f13440820 movlps qword ptr [rax + rcx + 0x20], xmm0
1407ee9a8: f30f114c0828 movss dword ptr [rax + rcx + 0x28], xmm1
1407ee9ae: 488d9520010000 lea rdx, [rbp + 0x120]
1407ee9b5: f30f6f420c movdqu xmm0, xmmword ptr [rdx + 0xc]
1407ee9ba: f30f7f44082c movdqu xmmword ptr [rax + rcx + 0x2c], xmm0
1407ee9c0: 8b521c mov edx, dword ptr [rdx + 0x1c]
1407ee9c3: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407ee9c7: 488b9578020000 mov rdx, qword ptr [rbp + 0x278]
1407ee9ce: 4889540840 mov qword ptr [rax + rcx + 0x40], rdx
1407ee9d3: 488b9580020000 mov rdx, qword ptr [rbp + 0x280]
1407ee9da: 4889540848 mov qword ptr [rax + rcx + 0x48], rdx
1407ee9df: 48ffc7 inc rdi
1407ee9e2: 4989b808010000 mov qword ptr [r8 + 0x108], rdi
1407ee9e9: e93cdaffff jmp 0x1407ec42a ; unknown
1407ee9ee: 450f57c9 xorps xmm9, xmm9
1407ee9f2: 7209 jb 0x1407ee9fd
1407ee9f4: f3440f100d4bf92802 movss xmm9, dword ptr [rip + 0x228f94b]
1407ee9fd: 4084f6 test sil, sil
1407eea00: 0f8469020000 je 0x1407eec6f
1407eea06: f3440f10b5d0000000 movss xmm14, dword ptr [rbp + 0xd0]
1407eea0f: 488d5e01 lea rbx, [rsi + 1]
1407eea13: 48c7854801000000000000 mov qword ptr [rbp + 0x148], 0
1407eea1e: e9adecffff jmp 0x1407ed6d0 ; unknown
1407eea23: f30f108590020000 movss xmm0, dword ptr [rbp + 0x290]
1407eea2b: f30f5805c5cc1f02 addss xmm0, dword ptr [rip + 0x21fccc5]
1407eea33: f30f118590020000 movss dword ptr [rbp + 0x290], xmm0
1407eea3b: e997ebffff jmp 0x1407ed5d7 ; unknown
1407eea40: f30f108590020000 movss xmm0, dword ptr [rbp + 0x290]
1407eea48: f30f580598f21e02 addss xmm0, dword ptr [rip + 0x21ef298]
1407eea50: f30f118590020000 movss dword ptr [rbp + 0x290], xmm0
1407eea58: e97aebffff jmp 0x1407ed5d7 ; unknown
1407eea5d: 8b858c010000 mov eax, dword ptr [rbp + 0x18c]
1407eea63: 4c8b85f0000000 mov r8, qword ptr [rbp + 0xf0]
1407eea6a: 4c8b8db8000000 mov r9, qword ptr [rbp + 0xb8]
1407eea71: 4c8985f0000000 mov qword ptr [rbp + 0xf0], r8
1407eea78: 4c898db8000000 mov qword ptr [rbp + 0xb8], r9
1407eea7f: 4d85c9 test r9, r9
1407eea82: 0f85c2d8ffff jne 0x1407ec34a
1407eea88: 80bda702000000 cmp byte ptr [rbp + 0x2a7], 0
1407eea8f: 488b7558 mov rsi, qword ptr [rbp + 0x58]
1407eea93: 7447 je 0x1407eeadc
1407eea95: e896964200 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1407eea9a: 4889c1 mov rcx, rax
1407eea9d: 488b00 mov rax, qword ptr [rax]
1407eeaa0: 4883f801 cmp rax, 1
1407eeaa4: 0f85e2010000 jne 0x1407eec8c
1407eeaaa: 4883c108 add rcx, 8
1407eeaae: 48833900 cmp qword ptr [rcx], 0
1407eeab2: 0f85f8010000 jne 0x1407eecb0
1407eeab8: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1407eeabf: 48898da8020000 mov qword ptr [rbp + 0x2a8], rcx
1407eeac6: 4883c108 add rcx, 8
1407eeaca: 4889f2 mov rdx, rsi
1407eeacd: e81e864200 call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
1407eead2: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
1407eead9: 48ff00 inc qword ptr [rax]
1407eeadc: 0f28b5c0020000 movaps xmm6, xmmword ptr [rbp + 0x2c0]
1407eeae3: 0f28bdd0020000 movaps xmm7, xmmword ptr [rbp + 0x2d0]
1407eeaea: 440f2885e0020000 movaps xmm8, xmmword ptr [rbp + 0x2e0]
1407eeaf2: 440f288df0020000 movaps xmm9, xmmword ptr [rbp + 0x2f0]
1407eeafa: 440f289500030000 movaps xmm10, xmmword ptr [rbp + 0x300]
1407eeb02: 440f289d10030000 movaps xmm11, xmmword ptr [rbp + 0x310]
1407eeb0a: 440f28a520030000 movaps xmm12, xmmword ptr [rbp + 0x320]
1407eeb12: 440f28ad30030000 movaps xmm13, xmmword ptr [rbp + 0x330]
1407eeb1a: 440f28b540030000 movaps xmm14, xmmword ptr [rbp + 0x340]
1407eeb22: 440f28bd50030000 movaps xmm15, xmmword ptr [rbp + 0x350]
1407eeb2a: 4881c4e8030000 add rsp, 0x3e8
1407eeb31: 5b pop rbx
1407eeb32: 5f pop rdi
1407eeb33: 5e pop rsi
1407eeb34: 415c pop r12
1407eeb36: 415d pop r13
1407eeb38: 415e pop r14
1407eeb3a: 415f pop r15
1407eeb3c: 5d pop rbp
1407eeb3d: c3 ret 
1407eeb3e: 488d0da3274302 lea rcx, [rip + 0x24327a3]
1407eeb45: e8c6a70e02 call 0x1428d9310 ; core::option::unwrap_failed
1407eeb4a: e928030000 jmp 0x1407eee77 ; unknown
1407eeb4f: 488d0de3b52902 lea rcx, [rip + 0x229b5e3]
1407eeb56: 4c8d051bba2902 lea r8, [rip + 0x229ba1b]
1407eeb5d: ba16000000 mov edx, 0x16
1407eeb62: e8c9a70e02 call 0x1428d9330 ; core::option::expect_failed
1407eeb67: e90b030000 jmp 0x1407eee77 ; unknown
1407eeb6c: 488d8598010000 lea rax, [rbp + 0x198]
1407eeb73: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
1407eeb7a: 488d051fceffff lea rax, [rip - 0x31e1]
1407eeb81: 488985c8010000 mov qword ptr [rbp + 0x1c8], rax
1407eeb88: 488d0559b62902 lea rax, [rip + 0x229b659]
1407eeb8f: 48898510020000 mov qword ptr [rbp + 0x210], rax
1407eeb96: 48c7851802000001000000 mov qword ptr [rbp + 0x218], 1
1407eeba1: 48c7853002000000000000 mov qword ptr [rbp + 0x230], 0
1407eebac: 488d85c0010000 lea rax, [rbp + 0x1c0]
1407eebb3: 48898520020000 mov qword ptr [rbp + 0x220], rax
1407eebba: 48c7852802000001000000 mov qword ptr [rbp + 0x228], 1
1407eebc5: 488d152cb62902 lea rdx, [rip + 0x229b62c]
1407eebcc: 488d8d10020000 lea rcx, [rbp + 0x210]
1407eebd3: e8b8a70e02 call 0x1428d9390 ; core::panicking::panic_fmt
1407eebd8: e99a020000 jmp 0x1407eee77 ; unknown
1407eebdd: 488d8550010000 lea rax, [rbp + 0x150]
1407eebe4: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
1407eebeb: 488d05aecdffff lea rax, [rip - 0x3252]
1407eebf2: 488985c8010000 mov qword ptr [rbp + 0x1c8], rax
1407eebf9: 488d05e8b52902 lea rax, [rip + 0x229b5e8]
1407eec00: 48898510020000 mov qword ptr [rbp + 0x210], rax
1407eec07: 48c7851802000001000000 mov qword ptr [rbp + 0x218], 1
1407eec12: 48c7853002000000000000 mov qword ptr [rbp + 0x230], 0
1407eec1d: 488d85c0010000 lea rax, [rbp + 0x1c0]
1407eec24: 48898520020000 mov qword ptr [rbp + 0x220], rax
1407eec2b: 48c7852802000001000000 mov qword ptr [rbp + 0x228], 1
1407eec36: c685b702000001 mov byte ptr [rbp + 0x2b7], 1
1407eec3d: 488d15ccb52902 lea rdx, [rip + 0x229b5cc]
1407eec44: 488d8d10020000 lea rcx, [rbp + 0x210]
1407eec4b: e840a70e02 call 0x1428d9390 ; core::panicking::panic_fmt
1407eec50: e922020000 jmp 0x1407eee77 ; unknown
1407eec55: 4885c0 test rax, rax
1407eec58: 754a jne 0x1407eeca4
1407eec5a: 31d2 xor edx, edx
1407eec5c: e81f391002 call 0x1428f2580 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1407eec61: 4889c1 mov rcx, rax
1407eec64: 4885c0 test rax, rax
1407eec67: 0f85d1d5ffff jne 0x1407ec23e
1407eec6d: eb35 jmp 0x1407eeca4 ; unknown
1407eec6f: 488d0d16c54302 lea rcx, [rip + 0x243c516]
1407eec76: 4c8d055bc54302 lea r8, [rip + 0x243c55b]
1407eec7d: ba23000000 mov edx, 0x23
1407eec82: e8a9a70e02 call 0x1428d9430 ; core::panicking::panic
1407eec87: e9eb010000 jmp 0x1407eee77 ; unknown
1407eec8c: 4885c0 test rax, rax
1407eec8f: 7513 jne 0x1407eeca4
1407eec91: 31d2 xor edx, edx
1407eec93: e8e8381002 call 0x1428f2580 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1407eec98: 4889c1 mov rcx, rax
1407eec9b: 4885c0 test rax, rax
1407eec9e: 0f850afeffff jne 0x1407eeaae
1407eeca4: 488d0d1d274302 lea rcx, [rip + 0x243271d]
1407eecab: e820940e02 call 0x1428d80d0 ; std::thread::local::panic_access_error
1407eecb0: 488d0d81274302 lea rcx, [rip + 0x2432781]
1407eecb7: e894a50e02 call 0x1428d9250 ; core::cell::panic_already_borrowed
1407eecbc: 488d05652a2a02 lea rax, [rip + 0x22a2a65]
1407eecc3: 48898510020000 mov qword ptr [rbp + 0x210], rax
1407eecca: 48c7851802000001000000 mov qword ptr [rbp + 0x218], 1
1407eecd5: 48c7852002000008000000 mov qword ptr [rbp + 0x220], 8
1407eece0: 0f57c0 xorps xmm0, xmm0
1407eece3: 0f118528020000 movups xmmword ptr [rbp + 0x228], xmm0
1407eecea: 488d15472a2a02 lea rdx, [rip + 0x22a2a47]
1407eecf1: 488d8d10020000 lea rcx, [rbp + 0x210]
1407eecf8: e893a60e02 call 0x1428d9390 ; core::panicking::panic_fmt
1407eecfd: e975010000 jmp 0x1407eee77 ; unknown
1407eed02: 488d8598010000 lea rax, [rbp + 0x198]
1407eed09: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
1407eed10: 488d05d9e1a600 lea rax, [rip + 0xa6e1d9]
1407eed17: 488985c8010000 mov qword ptr [rbp + 0x1c8], rax
1407eed1e: 488d055b2f4302 lea rax, [rip + 0x2432f5b]
1407eed25: 48898510020000 mov qword ptr [rbp + 0x210], rax
1407eed2c: 48c7851802000001000000 mov qword ptr [rbp + 0x218], 1
1407eed37: 48c7853002000000000000 mov qword ptr [rbp + 0x230], 0
1407eed42: 488d85c0010000 lea rax, [rbp + 0x1c0]
1407eed49: 48898520020000 mov qword ptr [rbp + 0x220], rax
1407eed50: 48c7852802000001000000 mov qword ptr [rbp + 0x228], 1
1407eed5b: 488d15462f4302 lea rdx, [rip + 0x2432f46]
1407eed62: 488d8d10020000 lea rcx, [rbp + 0x210]
1407eed69: e822a60e02 call 0x1428d9390 ; core::panicking::panic_fmt
1407eed6e: e904010000 jmp 0x1407eee77 ; unknown
1407eed73: 488d0d5eb92902 lea rcx, [rip + 0x229b95e]
1407eed7a: 4c8d0547bc2902 lea r8, [rip + 0x229bc47]
1407eed81: ba28000000 mov edx, 0x28
1407eed86: e8a5a60e02 call 0x1428d9430 ; core::panicking::panic
1407eed8b: e9e7000000 jmp 0x1407eee77 ; unknown
1407eed90: 488d858f020000 lea rax, [rbp + 0x28f]
1407eed97: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
1407eed9e: 488d059bccffff lea rax, [rip - 0x3365]
1407eeda5: 488985c8010000 mov qword ptr [rbp + 0x1c8], rax
1407eedac: 488b8570020000 mov rax, qword ptr [rbp + 0x270]
1407eedb3: 488985d0010000 mov qword ptr [rbp + 0x1d0], rax
1407eedba: 488d05dfd0ffff lea rax, [rip - 0x2f21]
1407eedc1: 488985d8010000 mov qword ptr [rbp + 0x1d8], rax
1407eedc8: 488d05c1bb2902 lea rax, [rip + 0x229bbc1]
1407eedcf: 48898510020000 mov qword ptr [rbp + 0x210], rax
1407eedd6: 48c7851802000002000000 mov qword ptr [rbp + 0x218], 2
1407eede1: 48c7853002000000000000 mov qword ptr [rbp + 0x230], 0
1407eedec: 488d85c0010000 lea rax, [rbp + 0x1c0]
1407eedf3: 48898520020000 mov qword ptr [rbp + 0x220], rax
1407eedfa: 48c7852802000002000000 mov qword ptr [rbp + 0x228], 2
1407eee05: 488d15a4bb2902 lea rdx, [rip + 0x229bba4]
1407eee0c: 488d8d10020000 lea rcx, [rbp + 0x210]
1407eee13: e878a50e02 call 0x1428d9390 ; core::panicking::panic_fmt
1407eee18: eb5d jmp 0x1407eee77 ; unknown
1407eee1a: 488d0db7b82902 lea rcx, [rip + 0x229b8b7]
1407eee21: 4c8d0528bb2902 lea r8, [rip + 0x229bb28]
1407eee28: ba28000000 mov edx, 0x28
1407eee2d: e8fea50e02 call 0x1428d9430 ; core::panicking::panic
1407eee32: eb43 jmp 0x1407eee77 ; unknown
1407eee34: 488d0d9db82902 lea rcx, [rip + 0x229b89d]
1407eee3b: 4c8d05deb92902 lea r8, [rip + 0x229b9de]
1407eee42: ba28000000 mov edx, 0x28
1407eee47: e8e4a50e02 call 0x1428d9430 ; core::panicking::panic
1407eee4c: eb29 jmp 0x1407eee77 ; unknown
1407eee4e: b910000000 mov ecx, 0x10
1407eee53: ba50000000 mov edx, 0x50
1407eee58: e8a3a10e02 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
1407eee5d: eb18 jmp 0x1407eee77 ; unknown
1407eee5f: 488d0d72b82902 lea rcx, [rip + 0x229b872]
1407eee66: 4c8d0593b82902 lea r8, [rip + 0x229b893]
1407eee6d: ba28000000 mov edx, 0x28
1407eee72: e8b9a50e02 call 0x1428d9430 ; core::panicking::panic
1407eee77: 0f0b ud2 
1407eee79: 0f1f8000000000 nop dword ptr [rax]
1407eee80: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407eee85: 55 push rbp
1407eee86: 4157 push r15
1407eee88: 4156 push r14
1407eee8a: 4155 push r13
1407eee8c: 4154 push r12
1407eee8e: 56 push rsi
1407eee8f: 57 push rdi
1407eee90: 53 push rbx
1407eee91: 4881ec08010000 sub rsp, 0x108
1407eee98: 488daa80000000 lea rbp, [rdx + 0x80]
1407eee9f: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407eeea5: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407eeeab: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407eeeb4: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407eeebd: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407eeec6: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407eeecf: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407eeed8: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407eeee1: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407eeee9: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407eeef1: 488b8578020000 mov rax, qword ptr [rbp + 0x278]
1407eeef8: 48ff00 inc qword ptr [rax]
1407eeefb: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407eef03: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407eef0b: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407eef14: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407eef1d: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407eef26: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407eef2f: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407eef38: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407eef41: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407eef47: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407eef4d: 4881c408010000 add rsp, 0x108
1407eef54: 5b pop rbx
1407eef55: 5f pop rdi
1407eef56: 5e pop rsi
1407eef57: 415c pop r12
1407eef59: 415d pop r13
1407eef5b: 415e pop r14
1407eef5d: 415f pop r15
1407eef5f: 5d pop rbp
1407eef60: c3 ret 
1407eef61: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407eef70: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407eef75: 55 push rbp
1407eef76: 4157 push r15
1407eef78: 4156 push r14
1407eef7a: 4155 push r13
1407eef7c: 4154 push r12
1407eef7e: 56 push rsi
1407eef7f: 57 push rdi
1407eef80: 53 push rbx
1407eef81: 4881ec08010000 sub rsp, 0x108
1407eef88: 488daa80000000 lea rbp, [rdx + 0x80]
1407eef8f: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407eef95: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407eef9b: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407eefa4: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407eefad: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407eefb6: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407eefbf: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407eefc8: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407eefd1: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407eefd9: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407eefe1: 80bda702000000 cmp byte ptr [rbp + 0x2a7], 0
1407eefe8: 743a je 0x1407ef024
1407eefea: e841914200 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1407eefef: 4889c6 mov rsi, rax
1407eeff2: 488b00 mov rax, qword ptr [rax]
1407eeff5: 4883f801 cmp rax, 1
1407eeff9: 488b5558 mov rdx, qword ptr [rbp + 0x58]
1407eeffd: 0f8587000000 jne 0x1407ef08a
1407ef003: 4883c608 add rsi, 8
1407ef007: 48833e00 cmp qword ptr [rsi], 0
1407ef00b: 0f85a6000000 jne 0x1407ef0b7
1407ef011: 48c706ffffffff mov qword ptr [rsi], 0xffffffffffffffff
1407ef018: 488d4e08 lea rcx, [rsi + 8]
1407ef01c: e8cf804200 call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
1407ef021: 48ff06 inc qword ptr [rsi]
1407ef024: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407ef02c: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407ef034: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407ef03d: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407ef046: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407ef04f: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407ef058: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407ef061: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407ef06a: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407ef070: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407ef076: 4881c408010000 add rsp, 0x108
1407ef07d: 5b pop rbx
1407ef07e: 5f pop rdi
1407ef07f: 5e pop rsi
1407ef080: 415c pop r12
1407ef082: 415d pop r13
1407ef084: 415e pop r14
1407ef086: 415f pop r15
1407ef088: 5d pop rbp
1407ef089: c3 ret 
1407ef08a: 4885c0 test rax, rax
1407ef08d: 751c jne 0x1407ef0ab
1407ef08f: 4889f1 mov rcx, rsi
1407ef092: 4889d6 mov rsi, rdx
1407ef095: 31d2 xor edx, edx
1407ef097: e8e4341002 call 0x1428f2580 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1407ef09c: 4889f2 mov rdx, rsi
1407ef09f: 4889c6 mov rsi, rax
1407ef0a2: 4885c0 test rax, rax
1407ef0a5: 0f855cffffff jne 0x1407ef007
1407ef0ab: 488d0d16234302 lea rcx, [rip + 0x2432316]
1407ef0b2: e819900e02 call 0x1428d80d0 ; std::thread::local::panic_access_error
1407ef0b7: 488d0d7a234302 lea rcx, [rip + 0x243237a]
1407ef0be: e88da10e02 call 0x1428d9250 ; core::cell::panic_already_borrowed
1407ef0c3: cc int3 
1407ef0c4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407ef0d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407ef0d5: 55 push rbp
1407ef0d6: 4157 push r15
1407ef0d8: 4156 push r14
1407ef0da: 4155 push r13
1407ef0dc: 4154 push r12
1407ef0de: 56 push rsi
1407ef0df: 57 push rdi
1407ef0e0: 53 push rbx
1407ef0e1: 4881ec08010000 sub rsp, 0x108
1407ef0e8: 488daa80000000 lea rbp, [rdx + 0x80]
1407ef0ef: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407ef0f5: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407ef0fb: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407ef104: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407ef10d: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407ef116: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407ef11f: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407ef128: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407ef131: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407ef139: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407ef141: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
1407ef148: 48ff00 inc qword ptr [rax]
1407ef14b: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407ef153: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407ef15b: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407ef164: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407ef16d: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407ef176: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407ef17f: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407ef188: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407ef191: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407ef197: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407ef19d: 4881c408010000 add rsp, 0x108
1407ef1a4: 5b pop rbx
1407ef1a5: 5f pop rdi
1407ef1a6: 5e pop rsi
1407ef1a7: 415c pop r12
1407ef1a9: 415d pop r13
1407ef1ab: 415e pop r14
1407ef1ad: 415f pop r15
1407ef1af: 5d pop rbp
1407ef1b0: c3 ret 
1407ef1b1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407ef1c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407ef1c5: 55 push rbp
1407ef1c6: 4157 push r15
1407ef1c8: 4156 push r14
1407ef1ca: 4155 push r13
1407ef1cc: 4154 push r12
1407ef1ce: 56 push rsi
1407ef1cf: 57 push rdi
1407ef1d0: 53 push rbx
1407ef1d1: 4881ec08010000 sub rsp, 0x108
1407ef1d8: 488daa80000000 lea rbp, [rdx + 0x80]
1407ef1df: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407ef1e5: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407ef1eb: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407ef1f4: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407ef1fd: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407ef206: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407ef20f: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407ef218: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407ef221: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407ef229: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407ef231: 488b85c0010000 mov rax, qword ptr [rbp + 0x1c0]
1407ef238: 4885c0 test rax, rax
1407ef23b: 741a je 0x1407ef257
1407ef23d: 48c1e004 shl rax, 4
1407ef241: 488d1440 lea rdx, [rax + rax*2]
1407ef245: 41b810000000 mov r8d, 0x10
1407ef24b: 488b8d68020000 mov rcx, qword ptr [rbp + 0x268]
1407ef252: e8c949e2ff call 0x140613c20 ; unknown
1407ef257: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407ef25f: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407ef267: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407ef270: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407ef279: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407ef282: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407ef28b: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407ef294: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407ef29d: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407ef2a3: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407ef2a9: 4881c408010000 add rsp, 0x108
1407ef2b0: 5b pop rbx
1407ef2b1: 5f pop rdi
1407ef2b2: 5e pop rsi
1407ef2b3: 415c pop r12
1407ef2b5: 415d pop r13
1407ef2b7: 415e pop r14
1407ef2b9: 415f pop r15
1407ef2bb: 5d pop rbp
1407ef2bc: c3 ret 
1407ef2bd: 0f1f00 nop dword ptr [rax]
1407ef2c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407ef2c5: 55 push rbp
1407ef2c6: 4157 push r15
1407ef2c8: 4156 push r14
1407ef2ca: 4155 push r13
1407ef2cc: 4154 push r12
1407ef2ce: 56 push rsi
1407ef2cf: 57 push rdi
1407ef2d0: 53 push rbx
1407ef2d1: 4881ec08010000 sub rsp, 0x108
1407ef2d8: 488daa80000000 lea rbp, [rdx + 0x80]
1407ef2df: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407ef2e5: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407ef2eb: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407ef2f4: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407ef2fd: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407ef306: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407ef30f: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407ef318: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407ef321: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407ef329: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407ef331: 4883bd9001000000 cmp qword ptr [rbp + 0x190], 0
1407ef339: 7421 je 0x1407ef35c
1407ef33b: 488b8590010000 mov rax, qword ptr [rbp + 0x190]
1407ef342: 48c1e004 shl rax, 4
1407ef346: 488d1440 lea rdx, [rax + rax*2]
1407ef34a: 41b810000000 mov r8d, 0x10
1407ef350: 488b8d70020000 mov rcx, qword ptr [rbp + 0x270]
1407ef357: e8c448e2ff call 0x140613c20 ; unknown
1407ef35c: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407ef364: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407ef36c: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407ef375: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407ef37e: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407ef387: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407ef390: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407ef399: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407ef3a2: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407ef3a8: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407ef3ae: 4881c408010000 add rsp, 0x108
1407ef3b5: 5b pop rbx
1407ef3b6: 5f pop rdi
1407ef3b7: 5e pop rsi
1407ef3b8: 415c pop r12
1407ef3ba: 415d pop r13
1407ef3bc: 415e pop r14
1407ef3be: 415f pop r15
1407ef3c0: 5d pop rbp
1407ef3c1: c3 ret 
1407ef3c2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407ef3d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407ef3d5: 55 push rbp
1407ef3d6: 4157 push r15
1407ef3d8: 4156 push r14
1407ef3da: 4155 push r13
1407ef3dc: 4154 push r12
1407ef3de: 56 push rsi
1407ef3df: 57 push rdi
1407ef3e0: 53 push rbx
1407ef3e1: 4881ec08010000 sub rsp, 0x108
1407ef3e8: 488daa80000000 lea rbp, [rdx + 0x80]
1407ef3ef: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407ef3f5: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407ef3fb: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407ef404: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407ef40d: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407ef416: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407ef41f: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407ef428: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407ef431: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407ef439: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407ef441: 488b8550010000 mov rax, qword ptr [rbp + 0x150]
1407ef448: 4885c0 test rax, rax
1407ef44b: 741a je 0x1407ef467
1407ef44d: 48c1e004 shl rax, 4
1407ef451: 488d1440 lea rdx, [rax + rax*2]
1407ef455: 41b810000000 mov r8d, 0x10
1407ef45b: 488b8d70020000 mov rcx, qword ptr [rbp + 0x270]
1407ef462: e8b947e2ff call 0x140613c20 ; unknown
1407ef467: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407ef46f: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407ef477: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407ef480: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407ef489: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407ef492: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407ef49b: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407ef4a4: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407ef4ad: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407ef4b3: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407ef4b9: 4881c408010000 add rsp, 0x108
1407ef4c0: 5b pop rbx
1407ef4c1: 5f pop rdi
1407ef4c2: 5e pop rsi
1407ef4c3: 415c pop r12
1407ef4c5: 415d pop r13
1407ef4c7: 415e pop r14
1407ef4c9: 415f pop r15
1407ef4cb: 5d pop rbp
1407ef4cc: c3 ret 
1407ef4cd: 0f1f00 nop dword ptr [rax]
1407ef4d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407ef4d5: 55 push rbp
1407ef4d6: 4157 push r15
1407ef4d8: 4156 push r14
1407ef4da: 4155 push r13
1407ef4dc: 4154 push r12
1407ef4de: 56 push rsi
1407ef4df: 57 push rdi
1407ef4e0: 53 push rbx
1407ef4e1: 4881ec08010000 sub rsp, 0x108
1407ef4e8: 488daa80000000 lea rbp, [rdx + 0x80]
1407ef4ef: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407ef4f5: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407ef4fb: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407ef504: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407ef50d: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407ef516: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407ef51f: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407ef528: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407ef531: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407ef539: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407ef541: 0fb69da6020000 movzx ebx, byte ptr [rbp + 0x2a6]
1407ef548: f6c301 test bl, 1
1407ef54b: 0f8485000000 je 0x1407ef5d6
1407ef551: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407ef558: f048ff08 lock dec qword ptr [rax]
1407ef55c: 750c jne 0x1407ef56a
1407ef55e: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407ef565: e8b6d03600 call 0x140b5c620 ; alloc::sync::Arc::drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
1407ef56a: 889db7020000 mov byte ptr [rbp + 0x2b7], bl
1407ef570: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407ef578: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407ef580: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407ef589: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407ef592: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407ef59b: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407ef5a4: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407ef5ad: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407ef5b6: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407ef5bc: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407ef5c2: 4881c408010000 add rsp, 0x108
1407ef5c9: 5b pop rbx
1407ef5ca: 5f pop rdi
1407ef5cb: 5e pop rsi
1407ef5cc: 415c pop r12
1407ef5ce: 415d pop r13
1407ef5d0: 415e pop r14
1407ef5d2: 415f pop r15
1407ef5d4: 5d pop rbp
1407ef5d5: c3 ret 
1407ef5d6: 889db7020000 mov byte ptr [rbp + 0x2b7], bl
1407ef5dc: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407ef5e4: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407ef5ec: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407ef5f5: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407ef5fe: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407ef607: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407ef610: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407ef619: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407ef622: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407ef628: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407ef62e: 4881c408010000 add rsp, 0x108
1407ef635: 5b pop rbx
1407ef636: 5f pop rdi
1407ef637: 5e pop rsi
1407ef638: 415c pop r12
1407ef63a: 415d pop r13
1407ef63c: 415e pop r14
1407ef63e: 415f pop r15
1407ef640: 5d pop rbp
1407ef641: c3 ret 
1407ef642: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407ef650: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407ef655: 55 push rbp
1407ef656: 4157 push r15
1407ef658: 4156 push r14
1407ef65a: 4155 push r13
1407ef65c: 4154 push r12
1407ef65e: 56 push rsi
1407ef65f: 57 push rdi
1407ef660: 53 push rbx
1407ef661: 4881ec08010000 sub rsp, 0x108
1407ef668: 488daa80000000 lea rbp, [rdx + 0x80]
1407ef66f: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407ef675: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407ef67b: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407ef684: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407ef68d: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407ef696: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407ef69f: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407ef6a8: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407ef6b1: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407ef6b9: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407ef6c1: f685b702000001 test byte ptr [rbp + 0x2b7], 1
1407ef6c8: 747f je 0x1407ef749
1407ef6ca: 488b8598010000 mov rax, qword ptr [rbp + 0x198]
1407ef6d1: f048ff08 lock dec qword ptr [rax]
1407ef6d5: 750c jne 0x1407ef6e3
1407ef6d7: 488d8d98010000 lea rcx, [rbp + 0x198]
1407ef6de: e83dcf3600 call 0x140b5c620 ; alloc::sync::Arc::drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
1407ef6e3: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407ef6eb: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407ef6f3: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407ef6fc: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407ef705: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407ef70e: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407ef717: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407ef720: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407ef729: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407ef72f: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407ef735: 4881c408010000 add rsp, 0x108
1407ef73c: 5b pop rbx
1407ef73d: 5f pop rdi
1407ef73e: 5e pop rsi
1407ef73f: 415c pop r12
1407ef741: 415d pop r13
1407ef743: 415e pop r14
1407ef745: 415f pop r15
1407ef747: 5d pop rbp
1407ef748: c3 ret 
1407ef749: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407ef751: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407ef759: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407ef762: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407ef76b: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407ef774: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407ef77d: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407ef786: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407ef78f: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407ef795: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407ef79b: 4881c408010000 add rsp, 0x108
1407ef7a2: 5b pop rbx
1407ef7a3: 5f pop rdi
1407ef7a4: 5e pop rsi
1407ef7a5: 415c pop r12
1407ef7a7: 415d pop r13
1407ef7a9: 415e pop r14
1407ef7ab: 415f pop r15
1407ef7ad: 5d pop rbp
1407ef7ae: c3 ret 

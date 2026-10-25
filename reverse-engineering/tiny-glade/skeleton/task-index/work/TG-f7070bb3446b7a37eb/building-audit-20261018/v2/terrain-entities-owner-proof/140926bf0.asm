0x140926bf0 55 push rbp
0x140926bf1 4157 push r15
0x140926bf3 4156 push r14
0x140926bf5 4155 push r13
0x140926bf7 4154 push r12
0x140926bf9 56 push rsi
0x140926bfa 57 push rdi
0x140926bfb 53 push rbx
0x140926bfc 4881ec98010000 sub rsp, 0x198
0x140926c03 488dac2480000000 lea rbp, [rsp + 0x80]
0x140926c0b 440f298500010000 movaps xmmword ptr [rbp + 0x100], xmm8
0x140926c13 0f29bdf0000000 movaps xmmword ptr [rbp + 0xf0], xmm7
0x140926c1a 0f29b5e0000000 movaps xmmword ptr [rbp + 0xe0], xmm6
0x140926c21 48c785d8000000feffffff mov qword ptr [rbp + 0xd8], 0xfffffffffffffffe
0x140926c2c 4c89cb mov rbx, r9
0x140926c2f 4c89c7 mov rdi, r8
0x140926c32 4989d6 mov r14, rdx
0x140926c35 4989cf mov r15, rcx
0x140926c38 31c9 xor ecx, ecx
0x140926c3a e8e12e7bff call 0x1400d9b20
0x140926c3f 4885c0 test rax, rax
0x140926c42 0f84ce0f0000 je 0x140927c16
0x140926c48 0f1000 movups xmm0, xmmword ptr [rax]
0x140926c4b 48ff00 inc qword ptr [rax]
0x140926c4e 0f103d7bf81902 movups xmm7, xmmword ptr [rip + 0x219f87b]
0x140926c55 0f11bda8000000 movups xmmword ptr [rbp + 0xa8], xmm7
0x140926c5c 440f10055cf81902 movups xmm8, xmmword ptr [rip + 0x219f85c]
0x140926c64 440f118598000000 movups xmmword ptr [rbp + 0x98], xmm8
0x140926c6c 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x140926c73 48c7456000000000 mov qword ptr [rbp + 0x60], 0
0x140926c7b 48c7456804000000 mov qword ptr [rbp + 0x68], 4
0x140926c83 48c7457000000000 mov qword ptr [rbp + 0x70], 0
0x140926c8b 48b80000000000000080 movabs rax, 0x8000000000000000
0x140926c95 48894578 mov qword ptr [rbp + 0x78], rax
0x140926c99 488d354a861802 lea rsi, [rip + 0x218864a]
0x140926ca0 4889b5c8000000 mov qword ptr [rbp + 0xc8], rsi
0x140926ca7 48c785d000000007000000 mov qword ptr [rbp + 0xd0], 7
0x140926cb2 c74424205a000000 mov dword ptr [rsp + 0x20], 0x5a
0x140926cba 488d4d60 lea rcx, [rbp + 0x60]
0x140926cbe 31d2 xor edx, edx
0x140926cc0 41b85a000000 mov r8d, 0x5a
0x140926cc6 41b95a000000 mov r9d, 0x5a
0x140926ccc e82f140000 call 0x140928100
0x140926cd1 488b8d90010000 mov rcx, qword ptr [rbp + 0x190]
0x140926cd8 4c8ba588010000 mov r12, qword ptr [rbp + 0x188]
0x140926cdf 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
0x140926ce6 48894540 mov qword ptr [rbp + 0x40], rax
0x140926cea 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
0x140926cf1 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
0x140926cf5 0f1085b0000000 movups xmm0, xmmword ptr [rbp + 0xb0]
0x140926cfc 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x140926d00 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
0x140926d07 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x140926d0b 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x140926d0f 0f104d70 movups xmm1, xmmword ptr [rbp + 0x70]
0x140926d13 0f109580000000 movups xmm2, xmmword ptr [rbp + 0x80]
0x140926d1a 0f109d90000000 movups xmm3, xmmword ptr [rbp + 0x90]
0x140926d21 0f295d00 movaps xmmword ptr [rbp], xmm3
0x140926d25 0f2955f0 movaps xmmword ptr [rbp - 0x10], xmm2
0x140926d29 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
0x140926d2d 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
0x140926d31 4c8b0f mov r9, qword ptr [rdi]
0x140926d34 4889cf mov rdi, rcx
0x140926d37 498b06 mov rax, qword ptr [r14]
0x140926d3a 48894558 mov qword ptr [rbp + 0x58], rax
0x140926d3e 4889442420 mov qword ptr [rsp + 0x20], rax
0x140926d43 488d15de811802 lea rdx, [rip + 0x21881de]
0x140926d4a 488d4dd0 lea rcx, [rbp - 0x30]
0x140926d4e 41b805000000 mov r8d, 5
0x140926d54 4c894d50 mov qword ptr [rbp + 0x50], r9
0x140926d58 e8433f1f00 call 0x140b1aca0
0x140926d5d 488945b8 mov qword ptr [rbp - 0x48], rax
0x140926d61 498b07 mov rax, qword ptr [r15]
0x140926d64 48894548 mov qword ptr [rbp + 0x48], rax
0x140926d68 488b08 mov rcx, qword ptr [rax]
0x140926d6b 4883c110 add rcx, 0x10
0x140926d6f 498b0424 mov rax, qword ptr [r12]
0x140926d73 488b1b mov rbx, qword ptr [rbx]
0x140926d76 48895c2430 mov qword ptr [rsp + 0x30], rbx
0x140926d7b 488945c8 mov qword ptr [rbp - 0x38], rax
0x140926d7f 4889442428 mov qword ptr [rsp + 0x28], rax
0x140926d84 48c74424200c000000 mov qword ptr [rsp + 0x20], 0xc
0x140926d8d 488d1599811802 lea rdx, [rip + 0x2188199]
0x140926d94 4c8d0db4811802 lea r9, [rip + 0x21881b4]
0x140926d9b 41b822000000 mov r8d, 0x22
0x140926da1 e89a902100 call 0x140b3fe40
0x140926da6 4989c7 mov r15, rax
0x140926da9 488d4d60 lea rcx, [rbp + 0x60]
0x140926dad 4889fa mov rdx, rdi
0x140926db0 e8ebd83e00 call 0x140d146a0
0x140926db5 4c8b6568 mov r12, qword ptr [rbp + 0x68]
0x140926db9 4d85e4 test r12, r12
0x140926dbc 7504 jne 0x140926dc2
0x140926dbe 4c8b6570 mov r12, qword ptr [rbp + 0x70]
0x140926dc2 488b9580000000 mov rdx, qword ptr [rbp + 0x80]
0x140926dc9 498b0424 mov rax, qword ptr [r12]
0x140926dcd 4d8b6c2410 mov r13, qword ptr [r12 + 0x10]
0x140926dd2 4c29e8 sub rax, r13
0x140926dd5 4883f877 cmp rax, 0x77
0x140926dd9 0f86850c0000 jbe 0x140927a64
0x140926ddf 498b442408 mov rax, qword ptr [r12 + 8]
0x140926de4 4c8d35850e9b00 lea r14, [rip + 0x9b0e85]
0x140926deb 4e893428 mov qword ptr [rax + r13], r14
0x140926def 0f2835ca720102 movaps xmm6, xmmword ptr [rip + 0x20172ca]
0x140926df6 420f11742808 movups xmmword ptr [rax + r13 + 8], xmm6
0x140926dfc 4ac744281800000000 mov qword ptr [rax + r13 + 0x18], 0
0x140926e05 42c744282000000000 mov dword ptr [rax + r13 + 0x20], 0
0x140926e0e 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
0x140926e18 4a894c2824 mov qword ptr [rax + r13 + 0x24], rcx
0x140926e1d 42c744282c0000803f mov dword ptr [rax + r13 + 0x2c], 0x3f800000
0x140926e26 488b4db8 mov rcx, qword ptr [rbp - 0x48]
0x140926e2a 4a894c2838 mov qword ptr [rax + r13 + 0x38], rcx
0x140926e2f 4e897c2840 mov qword ptr [rax + r13 + 0x40], r15
0x140926e34 42c644284800 mov byte ptr [rax + r13 + 0x48], 0
0x140926e3a 488955c0 mov qword ptr [rbp - 0x40], rdx
0x140926e3e 4a89542858 mov qword ptr [rax + r13 + 0x58], rdx
0x140926e43 488d0de6dec300 lea rcx, [rip + 0xc3dee6]
0x140926e4a 4a894c2868 mov qword ptr [rax + r13 + 0x68], rcx
0x140926e4f 4983c578 add r13, 0x78
0x140926e53 4d896c2410 mov qword ptr [r12 + 0x10], r13
0x140926e58 803d31dfd00200 cmp byte ptr [rip + 0x2d0df31], 0
0x140926e5f 0f84e6010000 je 0x14092704b
0x140926e65 31c9 xor ecx, ecx
0x140926e67 e8b42c7bff call 0x1400d9b20
0x140926e6c 4885c0 test rax, rax
0x140926e6f 0f84a10d0000 je 0x140927c16
0x140926e75 0f1000 movups xmm0, xmmword ptr [rax]
0x140926e78 48ff00 inc qword ptr [rax]
0x140926e7b 0f11bda8000000 movups xmmword ptr [rbp + 0xa8], xmm7
0x140926e82 440f118598000000 movups xmmword ptr [rbp + 0x98], xmm8
0x140926e8a 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x140926e91 48c7456000000000 mov qword ptr [rbp + 0x60], 0
0x140926e99 48c7456804000000 mov qword ptr [rbp + 0x68], 4
0x140926ea1 48c7457000000000 mov qword ptr [rbp + 0x70], 0
0x140926ea9 48b80000000000000080 movabs rax, 0x8000000000000000
0x140926eb3 48894578 mov qword ptr [rbp + 0x78], rax
0x140926eb7 4889b5c8000000 mov qword ptr [rbp + 0xc8], rsi
0x140926ebe 48c785d000000007000000 mov qword ptr [rbp + 0xd0], 7
0x140926ec9 c74424202d000000 mov dword ptr [rsp + 0x20], 0x2d
0x140926ed1 488d4d60 lea rcx, [rbp + 0x60]
0x140926ed5 31d2 xor edx, edx
0x140926ed7 41b82d000000 mov r8d, 0x2d
0x140926edd 41b92d000000 mov r9d, 0x2d
0x140926ee3 e818120000 call 0x140928100
0x140926ee8 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
0x140926eef 48894540 mov qword ptr [rbp + 0x40], rax
0x140926ef3 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
0x140926efa 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
0x140926efe 0f1085b0000000 movups xmm0, xmmword ptr [rbp + 0xb0]
0x140926f05 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x140926f09 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
0x140926f10 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x140926f14 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x140926f18 0f104d70 movups xmm1, xmmword ptr [rbp + 0x70]
0x140926f1c 0f109580000000 movups xmm2, xmmword ptr [rbp + 0x80]
0x140926f23 0f109d90000000 movups xmm3, xmmword ptr [rbp + 0x90]
0x140926f2a 0f295d00 movaps xmmword ptr [rbp], xmm3
0x140926f2e 0f2955f0 movaps xmmword ptr [rbp - 0x10], xmm2
0x140926f32 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
0x140926f36 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
0x140926f3a 488b4558 mov rax, qword ptr [rbp + 0x58]
0x140926f3e 4889442420 mov qword ptr [rsp + 0x20], rax
0x140926f43 488d1543801802 lea rdx, [rip + 0x2188043]
0x140926f4a 488d4dd0 lea rcx, [rbp - 0x30]
0x140926f4e 41b80f000000 mov r8d, 0xf
0x140926f54 4c8b4d50 mov r9, qword ptr [rbp + 0x50]
0x140926f58 e8433d1f00 call 0x140b1aca0
0x140926f5d 4989c7 mov r15, rax
0x140926f60 488b4548 mov rax, qword ptr [rbp + 0x48]
0x140926f64 488b08 mov rcx, qword ptr [rax]
0x140926f67 4883c110 add rcx, 0x10
0x140926f6b 48895c2430 mov qword ptr [rsp + 0x30], rbx
0x140926f70 488b45c8 mov rax, qword ptr [rbp - 0x38]
0x140926f74 4889442428 mov qword ptr [rsp + 0x28], rax
0x140926f79 48c744242014000000 mov qword ptr [rsp + 0x20], 0x14
0x140926f82 488d1513801802 lea rdx, [rip + 0x2188013]
0x140926f89 4c8d0d2d801802 lea r9, [rip + 0x218802d]
0x140926f90 41b821000000 mov r8d, 0x21
0x140926f96 e8a58e2100 call 0x140b3fe40
0x140926f9b 4989c4 mov r12, rax
0x140926f9e 488d4d60 lea rcx, [rbp + 0x60]
0x140926fa2 4889fa mov rdx, rdi
0x140926fa5 e8f6d63e00 call 0x140d146a0
0x140926faa 4c8b6d68 mov r13, qword ptr [rbp + 0x68]
0x140926fae 4d85ed test r13, r13
0x140926fb1 7504 jne 0x140926fb7
0x140926fb3 4c8b6d70 mov r13, qword ptr [rbp + 0x70]
0x140926fb7 4c8bb580000000 mov r14, qword ptr [rbp + 0x80]
0x140926fbe 498b4500 mov rax, qword ptr [r13]
0x140926fc2 498b7510 mov rsi, qword ptr [r13 + 0x10]
0x140926fc6 4829f0 sub rax, rsi
0x140926fc9 483d87000000 cmp rax, 0x87
0x140926fcf 0f861c0c0000 jbe 0x140927bf1
0x140926fd5 498b4508 mov rax, qword ptr [r13 + 8]
0x140926fd9 488d0d60349b00 lea rcx, [rip + 0x9b3460]
0x140926fe0 48890c30 mov qword ptr [rax + rsi], rcx
0x140926fe4 0f11743008 movups xmmword ptr [rax + rsi + 8], xmm6
0x140926fe9 48c744301800000000 mov qword ptr [rax + rsi + 0x18], 0
0x140926ff2 c744302000000000 mov dword ptr [rax + rsi + 0x20], 0
0x140926ffa 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
0x140927004 48894c3024 mov qword ptr [rax + rsi + 0x24], rcx
0x140927009 c744302c0000803f mov dword ptr [rax + rsi + 0x2c], 0x3f800000
0x140927011 4c897c3038 mov qword ptr [rax + rsi + 0x38], r15
0x140927016 4c89643040 mov qword ptr [rax + rsi + 0x40], r12
0x14092701b c744304810000000 mov dword ptr [rax + rsi + 0x48], 0x10
0x140927023 c644305800 mov byte ptr [rax + rsi + 0x58], 0
0x140927028 4c89743068 mov qword ptr [rax + rsi + 0x68], r14
0x14092702d 488d0dfcdcc300 lea rcx, [rip + 0xc3dcfc]
0x140927034 48894c3078 mov qword ptr [rax + rsi + 0x78], rcx
0x140927039 4881c688000000 add rsi, 0x88
0x140927040 49897510 mov qword ptr [r13 + 0x10], rsi
0x140927044 4c8d35250c9b00 lea r14, [rip + 0x9b0c25]
0x14092704b 488b8580010000 mov rax, qword ptr [rbp + 0x180]
0x140927052 488b4d48 mov rcx, qword ptr [rbp + 0x48]
0x140927056 4c8b01 mov r8, qword ptr [rcx]
0x140927059 4983c010 add r8, 0x10
0x14092705d 4c8b08 mov r9, qword ptr [rax]
0x140927060 488d0d6a7f1802 lea rcx, [rip + 0x2187f6a]
0x140927067 ba1e000000 mov edx, 0x1e
0x14092706c e8df4ef9ff call 0x1408bbf50
0x140927071 a801 test al, 1
0x140927073 0f85a90b0000 jne 0x140927c22
0x140927079 488b7708 mov rsi, qword ptr [rdi + 8]
0x14092707d 4885f6 test rsi, rsi
0x140927080 7504 jne 0x140927086
0x140927082 488b7710 mov rsi, qword ptr [rdi + 0x10]
0x140927086 488b06 mov rax, qword ptr [rsi]
0x140927089 4c8b7e10 mov r15, qword ptr [rsi + 0x10]
0x14092708d 4c29f8 sub rax, r15
0x140927090 4883f80f cmp rax, 0xf
0x140927094 0f86f5090000 jbe 0x140927a8f
0x14092709a 488b4608 mov rax, qword ptr [rsi + 8]
0x14092709e 488d0dfb099b00 lea rcx, [rip + 0x9b09fb]
0x1409270a5 4a890c38 mov qword ptr [rax + r15], rcx
0x1409270a9 4a89543808 mov qword ptr [rax + r15 + 8], rdx
0x1409270ae 4983c710 add r15, 0x10
0x1409270b2 4c897e10 mov qword ptr [rsi + 0x10], r15
0x1409270b6 488b4548 mov rax, qword ptr [rbp + 0x48]
0x1409270ba 488b08 mov rcx, qword ptr [rax]
0x1409270bd 4883c110 add rcx, 0x10
0x1409270c1 48895c2430 mov qword ptr [rsp + 0x30], rbx
0x1409270c6 488b45c8 mov rax, qword ptr [rbp - 0x38]
0x1409270ca 4889442428 mov qword ptr [rsp + 0x28], rax
0x1409270cf 48c74424200d000000 mov qword ptr [rsp + 0x20], 0xd
0x1409270d8 488d15297f1802 lea rdx, [rip + 0x2187f29]
0x1409270df 4c8d0d4a7f1802 lea r9, [rip + 0x2187f4a]
0x1409270e6 41b828000000 mov r8d, 0x28
0x1409270ec e84f8d2100 call 0x140b3fe40
0x1409270f1 4989c7 mov r15, rax
0x1409270f4 488d4d60 lea rcx, [rbp + 0x60]
0x1409270f8 4889fa mov rdx, rdi
0x1409270fb e8a0d53e00 call 0x140d146a0
0x140927100 4c8b6568 mov r12, qword ptr [rbp + 0x68]
0x140927104 4d85e4 test r12, r12
0x140927107 7504 jne 0x14092710d
0x140927109 4c8b6570 mov r12, qword ptr [rbp + 0x70]
0x14092710d 4c8bad80000000 mov r13, qword ptr [rbp + 0x80]
0x140927114 498b0424 mov rax, qword ptr [r12]
0x140927118 498b742410 mov rsi, qword ptr [r12 + 0x10]
0x14092711d 4829f0 sub rax, rsi
0x140927120 4883f877 cmp rax, 0x77
0x140927124 488b55c0 mov rdx, qword ptr [rbp - 0x40]
0x140927128 0f8693090000 jbe 0x140927ac1
0x14092712e 498b442408 mov rax, qword ptr [r12 + 8]
0x140927133 4c893430 mov qword ptr [rax + rsi], r14
0x140927137 0f11743008 movups xmmword ptr [rax + rsi + 8], xmm6
0x14092713c 48c744301800000000 mov qword ptr [rax + rsi + 0x18], 0
0x140927145 c744302000000000 mov dword ptr [rax + rsi + 0x20], 0
0x14092714d 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
0x140927157 48894c3024 mov qword ptr [rax + rsi + 0x24], rcx
0x14092715c c744302c0000803f mov dword ptr [rax + rsi + 0x2c], 0x3f800000
0x140927164 488b4db8 mov rcx, qword ptr [rbp - 0x48]
0x140927168 48894c3038 mov qword ptr [rax + rsi + 0x38], rcx
0x14092716d 4c897c3040 mov qword ptr [rax + rsi + 0x40], r15
0x140927172 c644304800 mov byte ptr [rax + rsi + 0x48], 0
0x140927177 4c896c3058 mov qword ptr [rax + rsi + 0x58], r13
0x14092717c 488d0daddbc300 lea rcx, [rip + 0xc3dbad]
0x140927183 48894c3068 mov qword ptr [rax + rsi + 0x68], rcx
0x140927188 4883c678 add rsi, 0x78
0x14092718c 4989742410 mov qword ptr [r12 + 0x10], rsi
0x140927191 488b7708 mov rsi, qword ptr [rdi + 8]
0x140927195 4885f6 test rsi, rsi
0x140927198 7504 jne 0x14092719e
0x14092719a 488b7710 mov rsi, qword ptr [rdi + 0x10]
0x14092719e 488b06 mov rax, qword ptr [rsi]
0x1409271a1 4c8b7610 mov r14, qword ptr [rsi + 0x10]
0x1409271a5 4c29f0 sub rax, r14
0x1409271a8 4883f817 cmp rax, 0x17
0x1409271ac 0f8638090000 jbe 0x140927aea
0x1409271b2 488b4608 mov rax, qword ptr [rsi + 8]
0x1409271b6 488d0d43199b00 lea rcx, [rip + 0x9b1943]
0x1409271bd 4a890c30 mov qword ptr [rax + r14], rcx
0x1409271c1 4a89543008 mov qword ptr [rax + r14 + 8], rdx
0x1409271c6 4e896c3010 mov qword ptr [rax + r14 + 0x10], r13
0x1409271cb 4983c618 add r14, 0x18
0x1409271cf 4c897610 mov qword ptr [rsi + 0x10], r14
0x1409271d3 488d15637e1802 lea rdx, [rip + 0x2187e63]
0x1409271da 488d4d60 lea rcx, [rbp + 0x60]
0x1409271de 41b81e000000 mov r8d, 0x1e
0x1409271e4 e8c77c1e00 call 0x140b0eeb0
0x1409271e9 31c0 xor eax, eax
0x1409271eb 483b4560 cmp rax, qword ptr [rbp + 0x60]
0x1409271ef 0f805e0a0000 jo 0x140927c53
0x1409271f5 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
0x1409271fc 48894540 mov qword ptr [rbp + 0x40], rax
0x140927200 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
0x140927207 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
0x14092720b 0f1085b0000000 movups xmm0, xmmword ptr [rbp + 0xb0]
0x140927212 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x140927216 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
0x14092721d 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x140927221 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x140927225 0f104d70 movups xmm1, xmmword ptr [rbp + 0x70]
0x140927229 0f109580000000 movups xmm2, xmmword ptr [rbp + 0x80]
0x140927230 0f109d90000000 movups xmm3, xmmword ptr [rbp + 0x90]
0x140927237 0f295d00 movaps xmmword ptr [rbp], xmm3
0x14092723b 0f2955f0 movaps xmmword ptr [rbp - 0x10], xmm2
0x14092723f 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
0x140927243 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
0x140927247 488b4558 mov rax, qword ptr [rbp + 0x58]
0x14092724b 4889442420 mov qword ptr [rsp + 0x20], rax
0x140927250 488d15217e1802 lea rdx, [rip + 0x2187e21]
0x140927257 488d4dd0 lea rcx, [rbp - 0x30]
0x14092725b 41b810000000 mov r8d, 0x10
0x140927261 4c8b4d50 mov r9, qword ptr [rbp + 0x50]
0x140927265 e8363a1f00 call 0x140b1aca0
0x14092726a 4989c7 mov r15, rax
0x14092726d 488b4548 mov rax, qword ptr [rbp + 0x48]
0x140927271 488b08 mov rcx, qword ptr [rax]
0x140927274 4883c110 add rcx, 0x10
0x140927278 48895c2430 mov qword ptr [rsp + 0x30], rbx
0x14092727d 488b45c8 mov rax, qword ptr [rbp - 0x38]
0x140927281 4889442428 mov qword ptr [rsp + 0x28], rax
0x140927286 48c74424200b000000 mov qword ptr [rsp + 0x20], 0xb
0x14092728f 488d15f27d1802 lea rdx, [rip + 0x2187df2]
0x140927296 4c8d0d1c7e1802 lea r9, [rip + 0x2187e1c]
0x14092729d 41b831000000 mov r8d, 0x31
0x1409272a3 e8988b2100 call 0x140b3fe40
0x1409272a8 4989c6 mov r14, rax
0x1409272ab 488d4d60 lea rcx, [rbp + 0x60]
0x1409272af 4889fa mov rdx, rdi
0x1409272b2 e8e9d33e00 call 0x140d146a0
0x1409272b7 4c8b6568 mov r12, qword ptr [rbp + 0x68]
0x1409272bb 4d85e4 test r12, r12
0x1409272be 7504 jne 0x1409272c4
0x1409272c0 4c8b6570 mov r12, qword ptr [rbp + 0x70]
0x1409272c4 4c8bad80000000 mov r13, qword ptr [rbp + 0x80]
0x1409272cb 498b0424 mov rax, qword ptr [r12]
0x1409272cf 498b742410 mov rsi, qword ptr [r12 + 0x10]
0x1409272d4 4829f0 sub rax, rsi
0x1409272d7 483d87000000 cmp rax, 0x87
0x1409272dd 0f8630080000 jbe 0x140927b13
0x1409272e3 498b442408 mov rax, qword ptr [r12 + 8]
0x1409272e8 488d0d01579b00 lea rcx, [rip + 0x9b5701]
0x1409272ef 48890c30 mov qword ptr [rax + rsi], rcx
0x1409272f3 0f11743008 movups xmmword ptr [rax + rsi + 8], xmm6
0x1409272f8 48c744301800000000 mov qword ptr [rax + rsi + 0x18], 0
0x140927301 c744302000000000 mov dword ptr [rax + rsi + 0x20], 0
0x140927309 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
0x140927313 48894c3024 mov qword ptr [rax + rsi + 0x24], rcx
0x140927318 c744302c0000803f mov dword ptr [rax + rsi + 0x2c], 0x3f800000
0x140927320 4c897c3038 mov qword ptr [rax + rsi + 0x38], r15
0x140927325 4c89743040 mov qword ptr [rax + rsi + 0x40], r14
0x14092732a c644304800 mov byte ptr [rax + rsi + 0x48], 0
0x14092732f c644305800 mov byte ptr [rax + rsi + 0x58], 0
0x140927334 4c896c3068 mov qword ptr [rax + rsi + 0x68], r13
0x140927339 488d0df0d9c300 lea rcx, [rip + 0xc3d9f0]
0x140927340 48894c3078 mov qword ptr [rax + rsi + 0x78], rcx
0x140927345 4881c688000000 add rsi, 0x88
0x14092734c 4989742410 mov qword ptr [r12 + 0x10], rsi
0x140927351 488d156c7d1802 lea rdx, [rip + 0x2187d6c]
0x140927358 488d4d60 lea rcx, [rbp + 0x60]
0x14092735c 41b81e000000 mov r8d, 0x1e
0x140927362 e8497b1e00 call 0x140b0eeb0
0x140927367 31c0 xor eax, eax
0x140927369 483b4560 cmp rax, qword ptr [rbp + 0x60]
0x14092736d 0f8015090000 jo 0x140927c88
0x140927373 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
0x14092737a 48894540 mov qword ptr [rbp + 0x40], rax
0x14092737e 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
0x140927385 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
0x140927389 0f1085b0000000 movups xmm0, xmmword ptr [rbp + 0xb0]
0x140927390 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x140927394 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
0x14092739b 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x14092739f 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x1409273a3 0f104d70 movups xmm1, xmmword ptr [rbp + 0x70]
0x1409273a7 0f109580000000 movups xmm2, xmmword ptr [rbp + 0x80]
0x1409273ae 0f109d90000000 movups xmm3, xmmword ptr [rbp + 0x90]
0x1409273b5 0f295d00 movaps xmmword ptr [rbp], xmm3
0x1409273b9 0f2955f0 movaps xmmword ptr [rbp - 0x10], xmm2
0x1409273bd 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
0x1409273c1 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
0x1409273c5 488b4558 mov rax, qword ptr [rbp + 0x58]
0x1409273c9 4889442420 mov qword ptr [rsp + 0x20], rax
0x1409273ce 488d152b7d1802 lea rdx, [rip + 0x2187d2b]
0x1409273d5 488d4dd0 lea rcx, [rbp - 0x30]
0x1409273d9 41b810000000 mov r8d, 0x10
0x1409273df 4c8b4d50 mov r9, qword ptr [rbp + 0x50]
0x1409273e3 e8b8381f00 call 0x140b1aca0
0x1409273e8 4989c7 mov r15, rax
0x1409273eb 488b4548 mov rax, qword ptr [rbp + 0x48]
0x1409273ef 488b08 mov rcx, qword ptr [rax]
0x1409273f2 4883c110 add rcx, 0x10
0x1409273f6 48895c2430 mov qword ptr [rsp + 0x30], rbx
0x1409273fb 488b45c8 mov rax, qword ptr [rbp - 0x38]
0x1409273ff 4889442428 mov qword ptr [rsp + 0x28], rax
0x140927404 48c74424200b000000 mov qword ptr [rsp + 0x20], 0xb
0x14092740d 488d15fc7c1802 lea rdx, [rip + 0x2187cfc]
0x140927414 4c8d0d0d7d1802 lea r9, [rip + 0x2187d0d]
0x14092741b 41b818000000 mov r8d, 0x18
0x140927421 e81a8a2100 call 0x140b3fe40
0x140927426 4989c4 mov r12, rax
0x140927429 488d4d60 lea rcx, [rbp + 0x60]
0x14092742d 4889fa mov rdx, rdi
0x140927430 e86bd23e00 call 0x140d146a0
0x140927435 4c8b6d68 mov r13, qword ptr [rbp + 0x68]
0x140927439 4d85ed test r13, r13
0x14092743c 7504 jne 0x140927442
0x14092743e 4c8b6d70 mov r13, qword ptr [rbp + 0x70]
0x140927442 488bbd80000000 mov rdi, qword ptr [rbp + 0x80]
0x140927449 498b4500 mov rax, qword ptr [r13]
0x14092744d 498b7510 mov rsi, qword ptr [r13 + 0x10]
0x140927451 4829f0 sub rax, rsi
0x140927454 4883f877 cmp rax, 0x77
0x140927458 0f86da060000 jbe 0x140927b38
0x14092745e 498b4508 mov rax, qword ptr [r13 + 8]
0x140927462 488d0d07089b00 lea rcx, [rip + 0x9b0807]
0x140927469 48890c30 mov qword ptr [rax + rsi], rcx
0x14092746d 0f11743008 movups xmmword ptr [rax + rsi + 8], xmm6
0x140927472 48c744301800000000 mov qword ptr [rax + rsi + 0x18], 0
0x14092747b c744302000000000 mov dword ptr [rax + rsi + 0x20], 0
0x140927483 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
0x14092748d 48894c3024 mov qword ptr [rax + rsi + 0x24], rcx
0x140927492 c744302c0000803f mov dword ptr [rax + rsi + 0x2c], 0x3f800000
0x14092749a 4c897c3038 mov qword ptr [rax + rsi + 0x38], r15
0x14092749f 4c89643040 mov qword ptr [rax + rsi + 0x40], r12
0x1409274a4 c644304800 mov byte ptr [rax + rsi + 0x48], 0
0x1409274a9 48897c3058 mov qword ptr [rax + rsi + 0x58], rdi
0x1409274ae 488d0d7bd8c300 lea rcx, [rip + 0xc3d87b]
0x1409274b5 48894c3068 mov qword ptr [rax + rsi + 0x68], rcx
0x1409274ba 4883c678 add rsi, 0x78
0x1409274be 49897510 mov qword ptr [r13 + 0x10], rsi
0x1409274c2 488d156a7c1802 lea rdx, [rip + 0x2187c6a]
0x1409274c9 488d4d60 lea rcx, [rbp + 0x60]
0x1409274cd 41b820000000 mov r8d, 0x20
0x1409274d3 e8d8791e00 call 0x140b0eeb0
0x1409274d8 31c0 xor eax, eax
0x1409274da 483b4560 cmp rax, qword ptr [rbp + 0x60]
0x1409274de 0f80d9070000 jo 0x140927cbd
0x1409274e4 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
0x1409274eb 48894540 mov qword ptr [rbp + 0x40], rax
0x1409274ef 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
0x1409274f6 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
0x1409274fa 0f1085b0000000 movups xmm0, xmmword ptr [rbp + 0xb0]
0x140927501 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x140927505 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
0x14092750c 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x140927510 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x140927514 0f104d70 movups xmm1, xmmword ptr [rbp + 0x70]
0x140927518 0f109580000000 movups xmm2, xmmword ptr [rbp + 0x80]
0x14092751f 0f109d90000000 movups xmm3, xmmword ptr [rbp + 0x90]
0x140927526 0f295d00 movaps xmmword ptr [rbp], xmm3
0x14092752a 0f2955f0 movaps xmmword ptr [rbp - 0x10], xmm2
0x14092752e 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
0x140927532 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
0x140927536 488b4558 mov rax, qword ptr [rbp + 0x58]
0x14092753a 4889442420 mov qword ptr [rsp + 0x20], rax
0x14092753f 488d152a7c1802 lea rdx, [rip + 0x2187c2a]
0x140927546 488d4dd0 lea rcx, [rbp - 0x30]
0x14092754a 41b812000000 mov r8d, 0x12
0x140927550 4c8b4d50 mov r9, qword ptr [rbp + 0x50]
0x140927554 e847371f00 call 0x140b1aca0
0x140927559 4989c7 mov r15, rax
0x14092755c 488d4d60 lea rcx, [rbp + 0x60]
0x140927560 488bbd90010000 mov rdi, qword ptr [rbp + 0x190]
0x140927567 4889fa mov rdx, rdi
0x14092756a e831d13e00 call 0x140d146a0
0x14092756f 4c8b6568 mov r12, qword ptr [rbp + 0x68]
0x140927573 4d85e4 test r12, r12
0x140927576 7504 jne 0x14092757c
0x140927578 4c8b6570 mov r12, qword ptr [rbp + 0x70]
0x14092757c 4c8bad80000000 mov r13, qword ptr [rbp + 0x80]
0x140927583 498b0424 mov rax, qword ptr [r12]
0x140927587 498b742410 mov rsi, qword ptr [r12 + 0x10]
0x14092758c 4829f0 sub rax, rsi
0x14092758f 483d87000000 cmp rax, 0x87
0x140927595 0f86c2050000 jbe 0x140927b5d
0x14092759b 498b442408 mov rax, qword ptr [r12 + 8]
0x1409275a0 488d0d49549b00 lea rcx, [rip + 0x9b5449]
0x1409275a7 48890c30 mov qword ptr [rax + rsi], rcx
0x1409275ab 0f11743008 movups xmmword ptr [rax + rsi + 8], xmm6
0x1409275b0 48c744301800000000 mov qword ptr [rax + rsi + 0x18], 0
0x1409275b9 c744302000000000 mov dword ptr [rax + rsi + 0x20], 0
0x1409275c1 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
0x1409275cb 48894c3024 mov qword ptr [rax + rsi + 0x24], rcx
0x1409275d0 c744302c0000803f mov dword ptr [rax + rsi + 0x2c], 0x3f800000
0x1409275d8 4c897c3038 mov qword ptr [rax + rsi + 0x38], r15
0x1409275dd 4c89743040 mov qword ptr [rax + rsi + 0x40], r14
0x1409275e2 c644304801 mov byte ptr [rax + rsi + 0x48], 1
0x1409275e7 c644305800 mov byte ptr [rax + rsi + 0x58], 0
0x1409275ec 4c896c3068 mov qword ptr [rax + rsi + 0x68], r13
0x1409275f1 488d0d38d7c300 lea rcx, [rip + 0xc3d738]
0x1409275f8 48894c3078 mov qword ptr [rax + rsi + 0x78], rcx
0x1409275fd 4881c688000000 add rsi, 0x88
0x140927604 4989742410 mov qword ptr [r12 + 0x10], rsi
0x140927609 488d15727b1802 lea rdx, [rip + 0x2187b72]
0x140927610 488d4d60 lea rcx, [rbp + 0x60]
0x140927614 41b81a000000 mov r8d, 0x1a
0x14092761a e891781e00 call 0x140b0eeb0
0x14092761f 31c0 xor eax, eax
0x140927621 483b4560 cmp rax, qword ptr [rbp + 0x60]
0x140927625 0f80c7060000 jo 0x140927cf2
0x14092762b 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
0x140927632 48894540 mov qword ptr [rbp + 0x40], rax
0x140927636 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
0x14092763d 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
0x140927641 0f1085b0000000 movups xmm0, xmmword ptr [rbp + 0xb0]
0x140927648 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x14092764c 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
0x140927653 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x140927657 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x14092765b 0f104d70 movups xmm1, xmmword ptr [rbp + 0x70]
0x14092765f 0f109580000000 movups xmm2, xmmword ptr [rbp + 0x80]
0x140927666 0f109d90000000 movups xmm3, xmmword ptr [rbp + 0x90]
0x14092766d 0f295d00 movaps xmmword ptr [rbp], xmm3
0x140927671 0f2955f0 movaps xmmword ptr [rbp - 0x10], xmm2
0x140927675 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
0x140927679 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
0x14092767d 488b4558 mov rax, qword ptr [rbp + 0x58]
0x140927681 4889442420 mov qword ptr [rsp + 0x20], rax
0x140927686 488d152b7b1802 lea rdx, [rip + 0x2187b2b]
0x14092768d 488d4dd0 lea rcx, [rbp - 0x30]
0x140927691 41b807000000 mov r8d, 7
0x140927697 4c8b4d50 mov r9, qword ptr [rbp + 0x50]
0x14092769b e800361f00 call 0x140b1aca0
0x1409276a0 4989c6 mov r14, rax
0x1409276a3 488d15157b1802 lea rdx, [rip + 0x2187b15]
0x1409276aa 41b81d000000 mov r8d, 0x1d
0x1409276b0 4889d9 mov rcx, rbx
0x1409276b3 e8e8030100 call 0x140937aa0
0x1409276b8 a801 test al, 1
0x1409276ba 0f8464060000 je 0x140927d24
0x1409276c0 4989d7 mov r15, rdx
0x1409276c3 488d4d60 lea rcx, [rbp + 0x60]
0x1409276c7 4889fa mov rdx, rdi
0x1409276ca e8d1cf3e00 call 0x140d146a0
0x1409276cf 4c8b6568 mov r12, qword ptr [rbp + 0x68]
0x1409276d3 4d85e4 test r12, r12
0x1409276d6 7504 jne 0x1409276dc
0x1409276d8 4c8b6570 mov r12, qword ptr [rbp + 0x70]
0x1409276dc 4c8bad80000000 mov r13, qword ptr [rbp + 0x80]
0x1409276e3 498b0424 mov rax, qword ptr [r12]
0x1409276e7 498b742410 mov rsi, qword ptr [r12 + 0x10]
0x1409276ec 4829f0 sub rax, rsi
0x1409276ef 483d87000000 cmp rax, 0x87
0x1409276f5 0f8687040000 jbe 0x140927b82
0x1409276fb 498b442408 mov rax, qword ptr [r12 + 8]
0x140927700 488d0df9439b00 lea rcx, [rip + 0x9b43f9]
0x140927707 48890c30 mov qword ptr [rax + rsi], rcx
0x14092770b 0f11743008 movups xmmword ptr [rax + rsi + 8], xmm6
0x140927710 48c744301800000000 mov qword ptr [rax + rsi + 0x18], 0
0x140927719 c744302000000000 mov dword ptr [rax + rsi + 0x20], 0
0x140927721 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
0x14092772b 48894c3024 mov qword ptr [rax + rsi + 0x24], rcx
0x140927730 c744302c0000803f mov dword ptr [rax + rsi + 0x2c], 0x3f800000
0x140927738 4c89743038 mov qword ptr [rax + rsi + 0x38], r14
0x14092773d 4c897c3040 mov qword ptr [rax + rsi + 0x40], r15
0x140927742 c74430486666663f mov dword ptr [rax + rsi + 0x48], 0x3f666666
0x14092774a c644305800 mov byte ptr [rax + rsi + 0x58], 0
0x14092774f 4c896c3068 mov qword ptr [rax + rsi + 0x68], r13
0x140927754 488d0dd5d5c300 lea rcx, [rip + 0xc3d5d5]
0x14092775b 48894c3078 mov qword ptr [rax + rsi + 0x78], rcx
0x140927760 4881c688000000 add rsi, 0x88
0x140927767 4989742410 mov qword ptr [r12 + 0x10], rsi
0x14092776c 488d15857a1802 lea rdx, [rip + 0x2187a85]
0x140927773 488d4d60 lea rcx, [rbp + 0x60]
0x140927777 41b820000000 mov r8d, 0x20
0x14092777d e82e771e00 call 0x140b0eeb0
0x140927782 31c0 xor eax, eax
0x140927784 483b4560 cmp rax, qword ptr [rbp + 0x60]
0x140927788 0f80a2050000 jo 0x140927d30
0x14092778e 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
0x140927795 48894540 mov qword ptr [rbp + 0x40], rax
0x140927799 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
0x1409277a0 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
0x1409277a4 0f1085b0000000 movups xmm0, xmmword ptr [rbp + 0xb0]
0x1409277ab 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x1409277af 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
0x1409277b6 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x1409277ba 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x1409277be 0f104d70 movups xmm1, xmmword ptr [rbp + 0x70]
0x1409277c2 0f109580000000 movups xmm2, xmmword ptr [rbp + 0x80]
0x1409277c9 0f109d90000000 movups xmm3, xmmword ptr [rbp + 0x90]
0x1409277d0 0f295d00 movaps xmmword ptr [rbp], xmm3
0x1409277d4 0f2955f0 movaps xmmword ptr [rbp - 0x10], xmm2
0x1409277d8 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
0x1409277dc 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
0x1409277e0 488b4558 mov rax, qword ptr [rbp + 0x58]
0x1409277e4 4889442420 mov qword ptr [rsp + 0x20], rax
0x1409277e9 488d15407a1802 lea rdx, [rip + 0x2187a40]
0x1409277f0 488d4dd0 lea rcx, [rbp - 0x30]
0x1409277f4 41b80d000000 mov r8d, 0xd
0x1409277fa 4c8b4d50 mov r9, qword ptr [rbp + 0x50]
0x1409277fe e89d341f00 call 0x140b1aca0
0x140927803 4989c6 mov r14, rax
0x140927806 488d15307a1802 lea rdx, [rip + 0x2187a30]
0x14092780d 41b812000000 mov r8d, 0x12
0x140927813 4889d9 mov rcx, rbx
0x140927816 e885020100 call 0x140937aa0
0x14092781b a801 test al, 1
0x14092781d 0f843f050000 je 0x140927d62
0x140927823 4989d7 mov r15, rdx
0x140927826 488d4d60 lea rcx, [rbp + 0x60]
0x14092782a 4889fa mov rdx, rdi
0x14092782d e86ece3e00 call 0x140d146a0
0x140927832 4c8b6568 mov r12, qword ptr [rbp + 0x68]
0x140927836 4d85e4 test r12, r12
0x140927839 7504 jne 0x14092783f
0x14092783b 4c8b6570 mov r12, qword ptr [rbp + 0x70]
0x14092783f 4c8bad80000000 mov r13, qword ptr [rbp + 0x80]
0x140927846 498b0424 mov rax, qword ptr [r12]
0x14092784a 498b742410 mov rsi, qword ptr [r12 + 0x10]
0x14092784f 4829f0 sub rax, rsi
0x140927852 4883f877 cmp rax, 0x77
0x140927856 0f864b030000 jbe 0x140927ba7
0x14092785c 498b442408 mov rax, qword ptr [r12 + 8]
0x140927861 488d0d281f9b00 lea rcx, [rip + 0x9b1f28]
0x140927868 48890c30 mov qword ptr [rax + rsi], rcx
0x14092786c 0f11743008 movups xmmword ptr [rax + rsi + 8], xmm6
0x140927871 48c744301800000000 mov qword ptr [rax + rsi + 0x18], 0
0x14092787a c744302000000000 mov dword ptr [rax + rsi + 0x20], 0
0x140927882 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
0x14092788c 48894c3024 mov qword ptr [rax + rsi + 0x24], rcx
0x140927891 c744302c0000803f mov dword ptr [rax + rsi + 0x2c], 0x3f800000
0x140927899 4c89743038 mov qword ptr [rax + rsi + 0x38], r14
0x14092789e 4c897c3040 mov qword ptr [rax + rsi + 0x40], r15
0x1409278a3 c644304800 mov byte ptr [rax + rsi + 0x48], 0
0x1409278a8 4c896c3058 mov qword ptr [rax + rsi + 0x58], r13
0x1409278ad 488d0d7cd4c300 lea rcx, [rip + 0xc3d47c]
0x1409278b4 48894c3068 mov qword ptr [rax + rsi + 0x68], rcx
0x1409278b9 4883c678 add rsi, 0x78
0x1409278bd 4989742410 mov qword ptr [r12 + 0x10], rsi
0x1409278c2 488d159f791802 lea rdx, [rip + 0x218799f]
0x1409278c9 488d4d60 lea rcx, [rbp + 0x60]
0x1409278cd 41b823000000 mov r8d, 0x23
0x1409278d3 e8d8751e00 call 0x140b0eeb0
0x1409278d8 31c0 xor eax, eax
0x1409278da 483b4560 cmp rax, qword ptr [rbp + 0x60]
0x1409278de 0f808a040000 jo 0x140927d6e
0x1409278e4 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
0x1409278eb 48894540 mov qword ptr [rbp + 0x40], rax
0x1409278ef 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
0x1409278f6 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
0x1409278fa 0f1085b0000000 movups xmm0, xmmword ptr [rbp + 0xb0]
0x140927901 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x140927905 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
0x14092790c 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x140927910 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x140927914 0f104d70 movups xmm1, xmmword ptr [rbp + 0x70]
0x140927918 0f109580000000 movups xmm2, xmmword ptr [rbp + 0x80]
0x14092791f 0f109d90000000 movups xmm3, xmmword ptr [rbp + 0x90]
0x140927926 0f295d00 movaps xmmword ptr [rbp], xmm3
0x14092792a 0f2955f0 movaps xmmword ptr [rbp - 0x10], xmm2
0x14092792e 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
0x140927932 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
0x140927936 488b4558 mov rax, qword ptr [rbp + 0x58]
0x14092793a 4889442420 mov qword ptr [rsp + 0x20], rax
0x14092793f 488d1562791802 lea rdx, [rip + 0x2187962]
0x140927946 488d4dd0 lea rcx, [rbp - 0x30]
0x14092794a 41b810000000 mov r8d, 0x10
0x140927950 4c8b4d50 mov r9, qword ptr [rbp + 0x50]
0x140927954 e847331f00 call 0x140b1aca0
0x140927959 4889c7 mov rdi, rax
0x14092795c 488b4548 mov rax, qword ptr [rbp + 0x48]
0x140927960 488b08 mov rcx, qword ptr [rax]
0x140927963 4883c110 add rcx, 0x10
0x140927967 48895c2430 mov qword ptr [rsp + 0x30], rbx
0x14092796c 488b45c8 mov rax, qword ptr [rbp - 0x38]
0x140927970 4889442428 mov qword ptr [rsp + 0x28], rax
0x140927975 48c744242016000000 mov qword ptr [rsp + 0x20], 0x16
0x14092797e 488d1533791802 lea rdx, [rip + 0x2187933]
0x140927985 4c8d0d48791802 lea r9, [rip + 0x2187948]
0x14092798c 41b81c000000 mov r8d, 0x1c
0x140927992 e8a9842100 call 0x140b3fe40
0x140927997 4889c3 mov rbx, rax
0x14092799a 488d4d60 lea rcx, [rbp + 0x60]
0x14092799e 488b9590010000 mov rdx, qword ptr [rbp + 0x190]
0x1409279a5 e8f6cc3e00 call 0x140d146a0
0x1409279aa 488b7568 mov rsi, qword ptr [rbp + 0x68]
0x1409279ae 4885f6 test rsi, rsi
0x1409279b1 7504 jne 0x1409279b7
0x1409279b3 488b7570 mov rsi, qword ptr [rbp + 0x70]
0x1409279b7 4c8bbd80000000 mov r15, qword ptr [rbp + 0x80]
0x1409279be 488b06 mov rax, qword ptr [rsi]
0x1409279c1 4c8b7610 mov r14, qword ptr [rsi + 0x10]
0x1409279c5 4c29f0 sub rax, r14
0x1409279c8 4883f877 cmp rax, 0x77
0x1409279cc 0f86fa010000 jbe 0x140927bcc
0x1409279d2 488b4608 mov rax, qword ptr [rsi + 8]
0x1409279d6 488d0dc3389b00 lea rcx, [rip + 0x9b38c3]
0x1409279dd 4a890c30 mov qword ptr [rax + r14], rcx
0x1409279e1 420f11743008 movups xmmword ptr [rax + r14 + 8], xmm6
0x1409279e7 4ac744301800000000 mov qword ptr [rax + r14 + 0x18], 0
0x1409279f0 42c744302000000000 mov dword ptr [rax + r14 + 0x20], 0
0x1409279f9 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
0x140927a03 4a894c3024 mov qword ptr [rax + r14 + 0x24], rcx
0x140927a08 42c744302c0000803f mov dword ptr [rax + r14 + 0x2c], 0x3f800000
0x140927a11 4a897c3038 mov qword ptr [rax + r14 + 0x38], rdi
0x140927a16 4a895c3040 mov qword ptr [rax + r14 + 0x40], rbx
0x140927a1b 42c644304800 mov byte ptr [rax + r14 + 0x48], 0
0x140927a21 4e897c3058 mov qword ptr [rax + r14 + 0x58], r15
0x140927a26 488d0d03d3c300 lea rcx, [rip + 0xc3d303]
0x140927a2d 4a894c3068 mov qword ptr [rax + r14 + 0x68], rcx
0x140927a32 4983c678 add r14, 0x78
0x140927a36 4c897610 mov qword ptr [rsi + 0x10], r14
0x140927a3a 0f28b5e0000000 movaps xmm6, xmmword ptr [rbp + 0xe0]
0x140927a41 0f28bdf0000000 movaps xmm7, xmmword ptr [rbp + 0xf0]
0x140927a48 440f288500010000 movaps xmm8, xmmword ptr [rbp + 0x100]
0x140927a50 4881c498010000 add rsp, 0x198
0x140927a57 5b pop rbx
0x140927a58 5f pop rdi
0x140927a59 5e pop rsi
0x140927a5a 415c pop r12
0x140927a5c 415d pop r13
0x140927a5e 415e pop r14
0x140927a60 415f pop r15
0x140927a62 5d pop rbp
0x140927a63 c3 ret 
0x140927a64 48c744242001000000 mov qword ptr [rsp + 0x20], 1
0x140927a6d 41b878000000 mov r8d, 0x78
0x140927a73 41b901000000 mov r9d, 1
0x140927a79 4c89e1 mov rcx, r12
0x140927a7c 4989d6 mov r14, rdx
0x140927a7f 4c89ea mov rdx, r13
0x140927a82 e8d91ff801 call 0x1428a9a60
0x140927a87 4c89f2 mov rdx, r14
0x140927a8a e950f3ffff jmp 0x140926ddf
0x140927a8f 48c744242001000000 mov qword ptr [rsp + 0x20], 1
0x140927a98 41b810000000 mov r8d, 0x10
0x140927a9e 41b901000000 mov r9d, 1
0x140927aa4 4889f1 mov rcx, rsi
0x140927aa7 4889d7 mov rdi, rdx
0x140927aaa 4c89fa mov rdx, r15
0x140927aad e8ae1ff801 call 0x1428a9a60
0x140927ab2 4889fa mov rdx, rdi
0x140927ab5 488bbd90010000 mov rdi, qword ptr [rbp + 0x190]
0x140927abc e9d9f5ffff jmp 0x14092709a
0x140927ac1 48c744242001000000 mov qword ptr [rsp + 0x20], 1
0x140927aca 41b878000000 mov r8d, 0x78
0x140927ad0 41b901000000 mov r9d, 1
0x140927ad6 4c89e1 mov rcx, r12
0x140927ad9 4889f2 mov rdx, rsi
0x140927adc e87f1ff801 call 0x1428a9a60
0x140927ae1 488b55c0 mov rdx, qword ptr [rbp - 0x40]
0x140927ae5 e944f6ffff jmp 0x14092712e
0x140927aea 48c744242001000000 mov qword ptr [rsp + 0x20], 1
0x140927af3 41b818000000 mov r8d, 0x18
0x140927af9 41b901000000 mov r9d, 1
0x140927aff 4889f1 mov rcx, rsi
0x140927b02 4c89f2 mov rdx, r14
0x140927b05 e8561ff801 call 0x1428a9a60
0x140927b0a 488b55c0 mov rdx, qword ptr [rbp - 0x40]
0x140927b0e e99ff6ffff jmp 0x1409271b2
0x140927b13 48c744242001000000 mov qword ptr [rsp + 0x20], 1
0x140927b1c 41b888000000 mov r8d, 0x88
0x140927b22 41b901000000 mov r9d, 1
0x140927b28 4c89e1 mov rcx, r12
0x140927b2b 4889f2 mov rdx, rsi
0x140927b2e e82d1ff801 call 0x1428a9a60
0x140927b33 e9abf7ffff jmp 0x1409272e3
0x140927b38 48c744242001000000 mov qword ptr [rsp + 0x20], 1
0x140927b41 41b878000000 mov r8d, 0x78
0x140927b47 41b901000000 mov r9d, 1
0x140927b4d 4c89e9 mov rcx, r13
0x140927b50 4889f2 mov rdx, rsi
0x140927b53 e8081ff801 call 0x1428a9a60
0x140927b58 e901f9ffff jmp 0x14092745e
0x140927b5d 48c744242001000000 mov qword ptr [rsp + 0x20], 1
0x140927b66 41b888000000 mov r8d, 0x88
0x140927b6c 41b901000000 mov r9d, 1
0x140927b72 4c89e1 mov rcx, r12
0x140927b75 4889f2 mov rdx, rsi
0x140927b78 e8e31ef801 call 0x1428a9a60
0x140927b7d e919faffff jmp 0x14092759b
0x140927b82 48c744242001000000 mov qword ptr [rsp + 0x20], 1
0x140927b8b 41b888000000 mov r8d, 0x88
0x140927b91 41b901000000 mov r9d, 1
0x140927b97 4c89e1 mov rcx, r12
0x140927b9a 4889f2 mov rdx, rsi
0x140927b9d e8be1ef801 call 0x1428a9a60
0x140927ba2 e954fbffff jmp 0x1409276fb
0x140927ba7 48c744242001000000 mov qword ptr [rsp + 0x20], 1
0x140927bb0 41b878000000 mov r8d, 0x78
0x140927bb6 41b901000000 mov r9d, 1
0x140927bbc 4c89e1 mov rcx, r12
0x140927bbf 4889f2 mov rdx, rsi
0x140927bc2 e8991ef801 call 0x1428a9a60
0x140927bc7 e990fcffff jmp 0x14092785c
0x140927bcc 48c744242001000000 mov qword ptr [rsp + 0x20], 1
0x140927bd5 41b878000000 mov r8d, 0x78
0x140927bdb 41b901000000 mov r9d, 1
0x140927be1 4889f1 mov rcx, rsi
0x140927be4 4c89f2 mov rdx, r14
0x140927be7 e8741ef801 call 0x1428a9a60
0x140927bec e9e1fdffff jmp 0x1409279d2
0x140927bf1 48c744242001000000 mov qword ptr [rsp + 0x20], 1
0x140927bfa 41b888000000 mov r8d, 0x88
0x140927c00 41b901000000 mov r9d, 1
0x140927c06 4c89e9 mov rcx, r13
0x140927c09 4889f2 mov rdx, rsi
0x140927c0c e84f1ef801 call 0x1428a9a60
0x140927c11 e9bff3ffff jmp 0x140926fd5
0x140927c16 488d0d832e3102 lea rcx, [rip + 0x2312e83]
0x140927c1d e8ae04fb01 call 0x1428d80d0
0x140927c22 48895560 mov qword ptr [rbp + 0x60], rdx
0x140927c26 488d05c3731802 lea rax, [rip + 0x21873c3]
0x140927c2d 4889442420 mov qword ptr [rsp + 0x20], rax
0x140927c32 488d0dd7681802 lea rcx, [rip + 0x21868d7]
0x140927c39 4c8d0db0681802 lea r9, [rip + 0x21868b0]
0x140927c40 4c8d4560 lea r8, [rbp + 0x60]
0x140927c44 ba2b000000 mov edx, 0x2b
0x140927c49 e8121bfb01 call 0x1428d9760
0x140927c4e e94b010000 jmp 0x140927d9e
0x140927c53 488b4568 mov rax, qword ptr [rbp + 0x68]
0x140927c57 488945d0 mov qword ptr [rbp - 0x30], rax
0x140927c5b 488d05fe731802 lea rax, [rip + 0x21873fe]
0x140927c62 4889442420 mov qword ptr [rsp + 0x20], rax
0x140927c67 488d0da2681802 lea rcx, [rip + 0x21868a2]
0x140927c6e 4c8d0d7b681802 lea r9, [rip + 0x218687b]
0x140927c75 4c8d45d0 lea r8, [rbp - 0x30]
0x140927c79 ba2b000000 mov edx, 0x2b
0x140927c7e e8dd1afb01 call 0x1428d9760
0x140927c83 e916010000 jmp 0x140927d9e
0x140927c88 488b4568 mov rax, qword ptr [rbp + 0x68]
0x140927c8c 488945d0 mov qword ptr [rbp - 0x30], rax
0x140927c90 488d0551741802 lea rax, [rip + 0x2187451]
0x140927c97 4889442420 mov qword ptr [rsp + 0x20], rax
0x140927c9c 488d0d6d681802 lea rcx, [rip + 0x218686d]
0x140927ca3 4c8d0d46681802 lea r9, [rip + 0x2186846]
0x140927caa 4c8d45d0 lea r8, [rbp - 0x30]
0x140927cae ba2b000000 mov edx, 0x2b
0x140927cb3 e8a81afb01 call 0x1428d9760
0x140927cb8 e9e1000000 jmp 0x140927d9e
0x140927cbd 488b4568 mov rax, qword ptr [rbp + 0x68]
0x140927cc1 488945d0 mov qword ptr [rbp - 0x30], rax
0x140927cc5 488d058c741802 lea rax, [rip + 0x218748c]
0x140927ccc 4889442420 mov qword ptr [rsp + 0x20], rax
0x140927cd1 488d0d38681802 lea rcx, [rip + 0x2186838]
0x140927cd8 4c8d0d11681802 lea r9, [rip + 0x2186811]
0x140927cdf 4c8d45d0 lea r8, [rbp - 0x30]
0x140927ce3 ba2b000000 mov edx, 0x2b
0x140927ce8 e8731afb01 call 0x1428d9760
0x140927ced e9ac000000 jmp 0x140927d9e
0x140927cf2 488b4568 mov rax, qword ptr [rbp + 0x68]
0x140927cf6 488945d0 mov qword ptr [rbp - 0x30], rax
0x140927cfa 488d059f741802 lea rax, [rip + 0x218749f]
0x140927d01 4889442420 mov qword ptr [rsp + 0x20], rax
0x140927d06 488d0d03681802 lea rcx, [rip + 0x2186803]
0x140927d0d 4c8d0ddc671802 lea r9, [rip + 0x21867dc]
0x140927d14 4c8d45d0 lea r8, [rbp - 0x30]
0x140927d18 ba2b000000 mov edx, 0x2b
0x140927d1d e83e1afb01 call 0x1428d9760
0x140927d22 eb7a jmp 0x140927d9e
0x140927d24 488d0db5741802 lea rcx, [rip + 0x21874b5]
0x140927d2b e8e015fb01 call 0x1428d9310
0x140927d30 488b4568 mov rax, qword ptr [rbp + 0x68]
0x140927d34 488945d0 mov qword ptr [rbp - 0x30], rax
0x140927d38 488d05d9741802 lea rax, [rip + 0x21874d9]
0x140927d3f 4889442420 mov qword ptr [rsp + 0x20], rax
0x140927d44 488d0dc5671802 lea rcx, [rip + 0x21867c5]
0x140927d4b 4c8d0d9e671802 lea r9, [rip + 0x218679e]
0x140927d52 4c8d45d0 lea r8, [rbp - 0x30]
0x140927d56 ba2b000000 mov edx, 0x2b
0x140927d5b e8001afb01 call 0x1428d9760
0x140927d60 eb3c jmp 0x140927d9e
0x140927d62 488d0de7741802 lea rcx, [rip + 0x21874e7]
0x140927d69 e8a215fb01 call 0x1428d9310
0x140927d6e 488b4568 mov rax, qword ptr [rbp + 0x68]
0x140927d72 488945d0 mov qword ptr [rbp - 0x30], rax
0x140927d76 488d0513751802 lea rax, [rip + 0x2187513]
0x140927d7d 4889442420 mov qword ptr [rsp + 0x20], rax
0x140927d82 488d0d87671802 lea rcx, [rip + 0x2186787]
0x140927d89 4c8d0d60671802 lea r9, [rip + 0x2186760]
0x140927d90 4c8d45d0 lea r8, [rbp - 0x30]
0x140927d94 ba2b000000 mov edx, 0x2b
0x140927d99 e8c219fb01 call 0x1428d9760
0x140927d9e 0f0b ud2 
0x140927da0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140927da5 55 push rbp
0x140927da6 4157 push r15
0x140927da8 4156 push r14
0x140927daa 4155 push r13
0x140927dac 4154 push r12
0x140927dae 56 push rsi
0x140927daf 57 push rdi
0x140927db0 53 push rbx
0x140927db1 4883ec68 sub rsp, 0x68
0x140927db5 488daa80000000 lea rbp, [rdx + 0x80]
0x140927dbc 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x140927dc2 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x140927dc7 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140927dcc 488d4d60 lea rcx, [rbp + 0x60]
0x140927dd0 e85b0df6ff call 0x140888b30
0x140927dd5 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140927dda 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x140927ddf 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x140927de5 4883c468 add rsp, 0x68
0x140927de9 5b pop rbx
0x140927dea 5f pop rdi
0x140927deb 5e pop rsi
0x140927dec 415c pop r12
0x140927dee 415d pop r13
0x140927df0 415e pop r14
0x140927df2 415f pop r15
0x140927df4 5d pop rbp
0x140927df5 c3 ret 
0x140927df6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140927e00 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140927e05 55 push rbp
0x140927e06 4157 push r15
0x140927e08 4156 push r14
0x140927e0a 4155 push r13
0x140927e0c 4154 push r12
0x140927e0e 56 push rsi
0x140927e0f 57 push rdi
0x140927e10 53 push rbx
0x140927e11 4883ec68 sub rsp, 0x68
0x140927e15 488daa80000000 lea rbp, [rdx + 0x80]
0x140927e1c 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x140927e22 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x140927e27 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140927e2c 488d4d60 lea rcx, [rbp + 0x60]
0x140927e30 e8fb4d4200 call 0x140d4cc30
0x140927e35 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140927e3a 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x140927e3f 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x140927e45 4883c468 add rsp, 0x68
0x140927e49 5b pop rbx
0x140927e4a 5f pop rdi
0x140927e4b 5e pop rsi
0x140927e4c 415c pop r12
0x140927e4e 415d pop r13
0x140927e50 415e pop r14
0x140927e52 415f pop r15
0x140927e54 5d pop rbp
0x140927e55 c3 ret 
0x140927e56 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140927e60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140927e65 55 push rbp
0x140927e66 4157 push r15
0x140927e68 4156 push r14
0x140927e6a 4155 push r13
0x140927e6c 4154 push r12
0x140927e6e 56 push rsi
0x140927e6f 57 push rdi
0x140927e70 53 push rbx
0x140927e71 4883ec68 sub rsp, 0x68
0x140927e75 488daa80000000 lea rbp, [rdx + 0x80]
0x140927e7c 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x140927e82 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x140927e87 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140927e8c 488d4dd0 lea rcx, [rbp - 0x30]
0x140927e90 e89b4d4200 call 0x140d4cc30
0x140927e95 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140927e9a 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x140927e9f 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x140927ea5 4883c468 add rsp, 0x68
0x140927ea9 5b pop rbx
0x140927eaa 5f pop rdi
0x140927eab 5e pop rsi
0x140927eac 415c pop r12
0x140927eae 415d pop r13
0x140927eb0 415e pop r14
0x140927eb2 415f pop r15
0x140927eb4 5d pop rbp
0x140927eb5 c3 ret 
0x140927eb6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140927ec0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140927ec5 55 push rbp
0x140927ec6 4157 push r15
0x140927ec8 4156 push r14
0x140927eca 4155 push r13
0x140927ecc 4154 push r12
0x140927ece 56 push rsi
0x140927ecf 57 push rdi
0x140927ed0 53 push rbx
0x140927ed1 4883ec68 sub rsp, 0x68
0x140927ed5 488daa80000000 lea rbp, [rdx + 0x80]
0x140927edc 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x140927ee2 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x140927ee7 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140927eec 488d4d60 lea rcx, [rbp + 0x60]
0x140927ef0 e83b0cf6ff call 0x140888b30
0x140927ef5 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140927efa 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x140927eff 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x140927f05 4883c468 add rsp, 0x68
0x140927f09 5b pop rbx
0x140927f0a 5f pop rdi
0x140927f0b 5e pop rsi
0x140927f0c 415c pop r12
0x140927f0e 415d pop r13
0x140927f10 415e pop r14
0x140927f12 415f pop r15
0x140927f14 5d pop rbp
0x140927f15 c3 ret 
0x140927f16 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140927f20 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140927f25 55 push rbp
0x140927f26 4157 push r15
0x140927f28 4156 push r14
0x140927f2a 4155 push r13
0x140927f2c 4154 push r12
0x140927f2e 56 push rsi
0x140927f2f 57 push rdi
0x140927f30 53 push rbx
0x140927f31 4883ec68 sub rsp, 0x68
0x140927f35 488daa80000000 lea rbp, [rdx + 0x80]
0x140927f3c 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x140927f42 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x140927f47 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140927f4c 488d4dd0 lea rcx, [rbp - 0x30]
0x140927f50 e8db4c4200 call 0x140d4cc30
0x140927f55 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140927f5a 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x140927f5f 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x140927f65 4883c468 add rsp, 0x68
0x140927f69 5b pop rbx
0x140927f6a 5f pop rdi
0x140927f6b 5e pop rsi
0x140927f6c 415c pop r12
0x140927f6e 415d pop r13
0x140927f70 415e pop r14
0x140927f72 415f pop r15
0x140927f74 5d pop rbp
0x140927f75 c3 ret 
0x140927f76 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140927f80 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140927f85 55 push rbp
0x140927f86 4157 push r15
0x140927f88 4156 push r14
0x140927f8a 4155 push r13
0x140927f8c 4154 push r12
0x140927f8e 56 push rsi
0x140927f8f 57 push rdi
0x140927f90 53 push rbx
0x140927f91 4883ec68 sub rsp, 0x68
0x140927f95 488daa80000000 lea rbp, [rdx + 0x80]
0x140927f9c 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x140927fa2 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x140927fa7 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140927fac 488d4dd0 lea rcx, [rbp - 0x30]
0x140927fb0 e87b4c4200 call 0x140d4cc30
0x140927fb5 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140927fba 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x140927fbf 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x140927fc5 4883c468 add rsp, 0x68
0x140927fc9 5b pop rbx
0x140927fca 5f pop rdi
0x140927fcb 5e pop rsi
0x140927fcc 415c pop r12
0x140927fce 415d pop r13
0x140927fd0 415e pop r14
0x140927fd2 415f pop r15
0x140927fd4 5d pop rbp
0x140927fd5 c3 ret 
0x140927fd6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140927fe0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140927fe5 55 push rbp
0x140927fe6 4157 push r15
0x140927fe8 4156 push r14
0x140927fea 4155 push r13
0x140927fec 4154 push r12
0x140927fee 56 push rsi
0x140927fef 57 push rdi
0x140927ff0 53 push rbx
0x140927ff1 4883ec68 sub rsp, 0x68
0x140927ff5 488daa80000000 lea rbp, [rdx + 0x80]
0x140927ffc 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x140928002 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x140928007 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x14092800c 488d4dd0 lea rcx, [rbp - 0x30]
0x140928010 e81b4c4200 call 0x140d4cc30
0x140928015 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14092801a 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14092801f 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x140928025 4883c468 add rsp, 0x68
0x140928029 5b pop rbx
0x14092802a 5f pop rdi
0x14092802b 5e pop rsi
0x14092802c 415c pop r12
0x14092802e 415d pop r13
0x140928030 415e pop r14
0x140928032 415f pop r15
0x140928034 5d pop rbp
0x140928035 c3 ret 
0x140928036 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140928040 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140928045 55 push rbp
0x140928046 4157 push r15
0x140928048 4156 push r14
0x14092804a 4155 push r13
0x14092804c 4154 push r12
0x14092804e 56 push rsi
0x14092804f 57 push rdi
0x140928050 53 push rbx
0x140928051 4883ec68 sub rsp, 0x68
0x140928055 488daa80000000 lea rbp, [rdx + 0x80]
0x14092805c 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x140928062 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x140928067 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x14092806c 488d4dd0 lea rcx, [rbp - 0x30]
0x140928070 e8bb4b4200 call 0x140d4cc30
0x140928075 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14092807a 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14092807f 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x140928085 4883c468 add rsp, 0x68
0x140928089 5b pop rbx
0x14092808a 5f pop rdi
0x14092808b 5e pop rsi
0x14092808c 415c pop r12
0x14092808e 415d pop r13
0x140928090 415e pop r14
0x140928092 415f pop r15
0x140928094 5d pop rbp
0x140928095 c3 ret 
0x140928096 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409280a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409280a5 55 push rbp
0x1409280a6 4157 push r15
0x1409280a8 4156 push r14
0x1409280aa 4155 push r13
0x1409280ac 4154 push r12
0x1409280ae 56 push rsi
0x1409280af 57 push rdi
0x1409280b0 53 push rbx
0x1409280b1 4883ec68 sub rsp, 0x68
0x1409280b5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409280bc 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x1409280c2 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x1409280c7 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1409280cc 488d4dd0 lea rcx, [rbp - 0x30]
0x1409280d0 e85b4b4200 call 0x140d4cc30
0x1409280d5 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409280da 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409280df 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x1409280e5 4883c468 add rsp, 0x68
0x1409280e9 5b pop rbx
0x1409280ea 5f pop rdi
0x1409280eb 5e pop rsi
0x1409280ec 415c pop r12
0x1409280ee 415d pop r13
0x1409280f0 415e pop r14
0x1409280f2 415f pop r15
0x1409280f4 5d pop rbp
0x1409280f5 c3 ret 

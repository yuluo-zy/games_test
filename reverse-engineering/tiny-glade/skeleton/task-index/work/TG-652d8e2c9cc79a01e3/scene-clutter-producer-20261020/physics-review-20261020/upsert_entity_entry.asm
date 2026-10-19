0x140b40b50 55 push rbp
0x140b40b51 4157 push r15
0x140b40b53 4156 push r14
0x140b40b55 4155 push r13
0x140b40b57 4154 push r12
0x140b40b59 56 push rsi
0x140b40b5a 57 push rdi
0x140b40b5b 53 push rbx
0x140b40b5c 4881eca8020000 sub rsp, 0x2a8
0x140b40b63 488dac2480000000 lea rbp, [rsp + 0x80]
0x140b40b6b 440f298510020000 movaps xmmword ptr [rbp + 0x210], xmm8
0x140b40b73 0f29bd00020000 movaps xmmword ptr [rbp + 0x200], xmm7
0x140b40b7a 0f29b5f0010000 movaps xmmword ptr [rbp + 0x1f0], xmm6
0x140b40b81 48c785e8010000feffffff mov qword ptr [rbp + 0x1e8], 0xfffffffffffffffe
0x140b40b8c 4d89cf mov r15, r9
0x140b40b8f 4c89c7 mov rdi, r8
0x140b40b92 4889d3 mov rbx, rdx
0x140b40b95 4889ce mov rsi, rcx
0x140b40b98 488b8590020000 mov rax, qword ptr [rbp + 0x290]
0x140b40b9f 410fb609 movzx ecx, byte ptr [r9]
0x140b40ba3 488d1592e4fa01 lea rdx, [rip + 0x1fae492]
0x140b40baa 448a2411 mov r12b, byte ptr [rcx + rdx]
0x140b40bae 498d4918 lea rcx, [r9 + 0x18]
0x140b40bb2 48898dd0010000 mov qword ptr [rbp + 0x1d0], rcx
0x140b40bb9 4d8b7118 mov r14, qword ptr [r9 + 0x18]
0x140b40bbd 8b4808 mov ecx, dword ptr [rax + 8]
0x140b40bc0 898de8000000 mov dword ptr [rbp + 0xe8], ecx
0x140b40bc6 488b08 mov rcx, qword ptr [rax]
0x140b40bc9 48898de0000000 mov qword ptr [rbp + 0xe0], rcx
0x140b40bd0 0f284010 movaps xmm0, xmmword ptr [rax + 0x10]
0x140b40bd4 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
0x140b40bd8 c685e501000001 mov byte ptr [rbp + 0x1e5], 1
0x140b40bdf 488d8d10010000 lea rcx, [rbp + 0x110]
0x140b40be6 488d95e0000000 lea rdx, [rbp + 0xe0]
0x140b40bed 4c8d4550 lea r8, [rbp + 0x50]
0x140b40bf1 e86a0f1500 call 0x140c91b60
0x140b40bf6 498d5660 lea rdx, [r14 + 0x60]
0x140b40bfa c685e501000001 mov byte ptr [rbp + 0x1e5], 1
0x140b40c01 488d8df0000000 lea rcx, [rbp + 0xf0]
0x140b40c08 4c8d8510010000 lea r8, [rbp + 0x110]
0x140b40c0f e86c0f1500 call 0x140c91b80
0x140b40c14 498b5630 mov rdx, qword ptr [r14 + 0x30]
0x140b40c18 4d8b4638 mov r8, qword ptr [r14 + 0x38]
0x140b40c1c f048ff02 lock inc qword ptr [rdx]
0x140b40c20 0f8e5e080000 jle 0x140b41484
0x140b40c26 c685e501000001 mov byte ptr [rbp + 0x1e5], 1
0x140b40c2d 488d8d10010000 lea rcx, [rbp + 0x110]
0x140b40c34 e867230500 call 0x140b92fa0
0x140b40c39 4488a5e2010000 mov byte ptr [rbp + 0x1e2], r12b
0x140b40c40 450fb6e4 movzx r12d, r12b
0x140b40c44 c6858601000001 mov byte ptr [rbp + 0x186], 1
0x140b40c4b 0f288510010000 movaps xmm0, xmmword ptr [rbp + 0x110]
0x140b40c52 0f288d20010000 movaps xmm1, xmmword ptr [rbp + 0x120]
0x140b40c59 0f289530010000 movaps xmm2, xmmword ptr [rbp + 0x130]
0x140b40c60 0f289d40010000 movaps xmm3, xmmword ptr [rbp + 0x140]
0x140b40c67 0f1145a4 movups xmmword ptr [rbp - 0x5c], xmm0
0x140b40c6b 0f114db4 movups xmmword ptr [rbp - 0x4c], xmm1
0x140b40c6f 0f1155c4 movups xmmword ptr [rbp - 0x3c], xmm2
0x140b40c73 0f115dd4 movups xmmword ptr [rbp - 0x2c], xmm3
0x140b40c77 488b8550010000 mov rax, qword ptr [rbp + 0x150]
0x140b40c7e 488945e4 mov qword ptr [rbp - 0x1c], rax
0x140b40c82 8b8580010000 mov eax, dword ptr [rbp + 0x180]
0x140b40c88 8985d0000000 mov dword ptr [rbp + 0xd0], eax
0x140b40c8e 0fb78584010000 movzx eax, word ptr [rbp + 0x184]
0x140b40c95 668985d4000000 mov word ptr [rbp + 0xd4], ax
0x140b40c9c 0fb68586010000 movzx eax, byte ptr [rbp + 0x186]
0x140b40ca3 8885d6000000 mov byte ptr [rbp + 0xd6], al
0x140b40ca9 488b8587010000 mov rax, qword ptr [rbp + 0x187]
0x140b40cb0 488985d7000000 mov qword ptr [rbp + 0xd7], rax
0x140b40cb7 0fb6858f010000 movzx eax, byte ptr [rbp + 0x18f]
0x140b40cbe 8885df000000 mov byte ptr [rbp + 0xdf], al
0x140b40cc4 0f108574010000 movups xmm0, xmmword ptr [rbp + 0x174]
0x140b40ccb 0f1185c4000000 movups xmmword ptr [rbp + 0xc4], xmm0
0x140b40cd2 440f2885f0000000 movaps xmm8, xmmword ptr [rbp + 0xf0]
0x140b40cda 0f28b500010000 movaps xmm6, xmmword ptr [rbp + 0x100]
0x140b40ce1 f30f10bdf8000000 movss xmm7, dword ptr [rbp + 0xf8]
0x140b40ce9 f30f6f45a4 movdqu xmm0, xmmword ptr [rbp - 0x5c]
0x140b40cee 0f104db4 movups xmm1, xmmword ptr [rbp - 0x4c]
0x140b40cf2 0f1055c4 movups xmm2, xmmword ptr [rbp - 0x3c]
0x140b40cf6 0f105dd4 movups xmm3, xmmword ptr [rbp - 0x2c]
0x140b40cfa 660f7f4560 movdqa xmmword ptr [rbp + 0x60], xmm0
0x140b40cff 0f294d70 movaps xmmword ptr [rbp + 0x70], xmm1
0x140b40d03 0f299580000000 movaps xmmword ptr [rbp + 0x80], xmm2
0x140b40d0a 0f299d90000000 movaps xmmword ptr [rbp + 0x90], xmm3
0x140b40d11 488b45e4 mov rax, qword ptr [rbp - 0x1c]
0x140b40d15 488985a0000000 mov qword ptr [rbp + 0xa0], rax
0x140b40d1c 0f11b5a8000000 movups xmmword ptr [rbp + 0xa8], xmm6
0x140b40d23 f30f11bdc0000000 movss dword ptr [rbp + 0xc0], xmm7
0x140b40d2b 440f1385b8000000 movlps qword ptr [rbp + 0xb8], xmm8
0x140b40d33 4c8b33 mov r14, qword ptr [rbx]
0x140b40d36 4d85f6 test r14, r14
0x140b40d39 0f84ea020000 je 0x140b41029
0x140b40d3f f3410f6f07 movdqu xmm0, xmmword ptr [r15]
0x140b40d44 410f104f10 movups xmm1, xmmword ptr [r15 + 0x10]
0x140b40d49 0f298d20010000 movaps xmmword ptr [rbp + 0x120], xmm1
0x140b40d50 660f7f8510010000 movdqa xmmword ptr [rbp + 0x110], xmm0
0x140b40d58 c685e701000001 mov byte ptr [rbp + 0x1e7], 1
0x140b40d5f c685e601000000 mov byte ptr [rbp + 0x1e6], 0
0x140b40d66 488d9510010000 lea rdx, [rbp + 0x110]
0x140b40d6d 4889f9 mov rcx, rdi
0x140b40d70 e82b81a100 call 0x141558ea0
0x140b40d75 4189c7 mov r15d, eax
0x140b40d78 4189d5 mov r13d, edx
0x140b40d7b 0f284560 movaps xmm0, xmmword ptr [rbp + 0x60]
0x140b40d7f 0f284d70 movaps xmm1, xmmword ptr [rbp + 0x70]
0x140b40d83 0f289590000000 movaps xmm2, xmmword ptr [rbp + 0x90]
0x140b40d8a 0f289da0000000 movaps xmm3, xmmword ptr [rbp + 0xa0]
0x140b40d91 0f298d20010000 movaps xmmword ptr [rbp + 0x120], xmm1
0x140b40d98 0f298510010000 movaps xmmword ptr [rbp + 0x110], xmm0
0x140b40d9f 0f299540010000 movaps xmmword ptr [rbp + 0x140], xmm2
0x140b40da6 0f299d50010000 movaps xmmword ptr [rbp + 0x150], xmm3
0x140b40dad 0f2885b0000000 movaps xmm0, xmmword ptr [rbp + 0xb0]
0x140b40db4 0f298560010000 movaps xmmword ptr [rbp + 0x160], xmm0
0x140b40dbb 0f2885c0000000 movaps xmm0, xmmword ptr [rbp + 0xc0]
0x140b40dc2 0f298570010000 movaps xmmword ptr [rbp + 0x170], xmm0
0x140b40dc9 660f6f85d0000000 movdqa xmm0, xmmword ptr [rbp + 0xd0]
0x140b40dd1 660f7f8580010000 movdqa xmmword ptr [rbp + 0x180], xmm0
0x140b40dd9 4c89f8 mov rax, r15
0x140b40ddc 48c1e020 shl rax, 0x20
0x140b40de0 89d1 mov ecx, edx
0x140b40de2 4809c1 or rcx, rax
0x140b40de5 48898d30010000 mov qword ptr [rbp + 0x130], rcx
0x140b40dec 48c7853801000000000000 mov qword ptr [rbp + 0x138], 0
0x140b40df7 488d4da0 lea rcx, [rbp - 0x60]
0x140b40dfb 488d9510010000 lea rdx, [rbp + 0x110]
0x140b40e02 e8f9210500 call 0x140b93000
0x140b40e07 488b8520010000 mov rax, qword ptr [rbp + 0x120]
0x140b40e0e f048ff08 lock dec qword ptr [rax]
0x140b40e12 750c jne 0x140b40e20
0x140b40e14 488d8d20010000 lea rcx, [rbp + 0x120]
0x140b40e1b e8b0ebb0ff call 0x14064f9d0
0x140b40e20 4489adcc010000 mov dword ptr [rbp + 0x1cc], r13d
0x140b40e27 83bd1001000002 cmp dword ptr [rbp + 0x110], 2
0x140b40e2e 7217 jb 0x140b40e47
0x140b40e30 488b8d18010000 mov rcx, qword ptr [rbp + 0x118]
0x140b40e37 ba2c000000 mov edx, 0x2c
0x140b40e3c 41b804000000 mov r8d, 4
0x140b40e42 e8d92dadff call 0x140613c20
0x140b40e47 4d69e490030000 imul r12, r12, 0x390
0x140b40e4e 42c68427a803000001 mov byte ptr [rdi + r12 + 0x3a8], 1
0x140b40e57 c685e301000001 mov byte ptr [rbp + 0x1e3], 1
0x140b40e5e 488d8d10010000 lea rcx, [rbp + 0x110]
0x140b40e65 488d55a0 lea rdx, [rbp - 0x60]
0x140b40e69 e8f2200500 call 0x140b92f60
0x140b40e6e 4901fc add r12, rdi
0x140b40e71 4983c420 add r12, 0x20
0x140b40e75 498d942488010000 lea rdx, [r12 + 0x188]
0x140b40e7d f30f108510010000 movss xmm0, dword ptr [rbp + 0x110]
0x140b40e85 f30f108d24010000 movss xmm1, dword ptr [rbp + 0x124]
0x140b40e8d f30f1185f0000000 movss dword ptr [rbp + 0xf0], xmm0
0x140b40e95 f30f7e8518010000 movq xmm0, qword ptr [rbp + 0x118]
0x140b40e9d 660fd685f4000000 movq qword ptr [rbp + 0xf4], xmm0
0x140b40ea5 f30f118dfc000000 movss dword ptr [rbp + 0xfc], xmm1
0x140b40ead c685e301000001 mov byte ptr [rbp + 0x1e3], 1
0x140b40eb4 488d8df0000000 lea rcx, [rbp + 0xf0]
0x140b40ebb e820daf9ff call 0x140ade8e0
0x140b40ec0 4c8dad10010000 lea r13, [rbp + 0x110]
0x140b40ec7 488d55a0 lea rdx, [rbp - 0x60]
0x140b40ecb 41b8b0000000 mov r8d, 0xb0
0x140b40ed1 4c89e9 mov rcx, r13
0x140b40ed4 e8272dde01 call 0x142923c00
0x140b40ed9 c685e301000000 mov byte ptr [rbp + 0x1e3], 0
0x140b40ee0 4c89e1 mov rcx, r12
0x140b40ee3 4c89ea mov rdx, r13
0x140b40ee6 e8c51aa100 call 0x1415529b0
0x140b40eeb 4189c4 mov r12d, eax
0x140b40eee 4189d5 mov r13d, edx
0x140b40ef1 488b4b10 mov rcx, qword ptr [rbx + 0x10]
0x140b40ef5 498b06 mov rax, qword ptr [r14]
0x140b40ef8 498b5608 mov rdx, qword ptr [r14 + 8]
0x140b40efc 4989d1 mov r9, rdx
0x140b40eff 4921c9 and r9, rcx
0x140b40f02 f3420f6f0408 movdqu xmm0, xmmword ptr [rax + r9]
0x140b40f08 66440fd7c0 pmovmskb r8d, xmm0
0x140b40f0d 4585c0 test r8d, r8d
0x140b40f10 0f840a050000 je 0x140b41420
0x140b40f16 4c8b5308 mov r10, qword ptr [rbx + 8]
0x140b40f1a f3450fbcc0 tzcnt r8d, r8d
0x140b40f1f 4d01c8 add r8, r9
0x140b40f22 4921d0 and r8, rdx
0x140b40f25 460fb60c00 movzx r9d, byte ptr [rax + r8]
0x140b40f2a 4584c9 test r9b, r9b
0x140b40f2d 0f8913050000 jns 0x140b41446
0x140b40f33 48c1e939 shr rcx, 0x39
0x140b40f37 4d8d58f0 lea r11, [r8 - 0x10]
0x140b40f3b 4921d3 and r11, rdx
0x140b40f3e 42880c00 mov byte ptr [rax + r8], cl
0x140b40f42 42884c1810 mov byte ptr [rax + r11 + 0x10], cl
0x140b40f47 49c1e005 shl r8, 5
0x140b40f4b 4180e101 and r9b, 1
0x140b40f4f 410fb6c9 movzx ecx, r9b
0x140b40f53 49294e10 sub qword ptr [r14 + 0x10], rcx
0x140b40f57 4889c1 mov rcx, rax
0x140b40f5a 4c29c1 sub rcx, r8
0x140b40f5d 4c8951e0 mov qword ptr [rcx - 0x20], r10
0x140b40f61 448979e8 mov dword ptr [rcx - 0x18], r15d
0x140b40f65 8b95cc010000 mov edx, dword ptr [rbp + 0x1cc]
0x140b40f6b 8951ec mov dword ptr [rcx - 0x14], edx
0x140b40f6e 448961f0 mov dword ptr [rcx - 0x10], r12d
0x140b40f72 448969f4 mov dword ptr [rcx - 0xc], r13d
0x140b40f76 440fb68de2010000 movzx r9d, byte ptr [rbp + 0x1e2]
0x140b40f7e 448849f8 mov byte ptr [rcx - 8], r9b
0x140b40f82 49f7d8 neg r8
0x140b40f85 0fb695de010000 movzx edx, byte ptr [rbp + 0x1de]
0x140b40f8c 42885400fb mov byte ptr [rax + r8 - 5], dl
0x140b40f91 0fb785dc010000 movzx eax, word ptr [rbp + 0x1dc]
0x140b40f98 668941f9 mov word ptr [rcx - 7], ax
0x140b40f9c 49ff4618 inc qword ptr [r14 + 0x18]
0x140b40fa0 4881c7b03c0000 add rdi, 0x3cb0
0x140b40fa7 4489a514010000 mov dword ptr [rbp + 0x114], r12d
0x140b40fae 4489ad18010000 mov dword ptr [rbp + 0x118], r13d
0x140b40fb5 44888d1c010000 mov byte ptr [rbp + 0x11c], r9b
0x140b40fbc 0fb785dc010000 movzx eax, word ptr [rbp + 0x1dc]
0x140b40fc3 6689851d010000 mov word ptr [rbp + 0x11d], ax
0x140b40fca 0fb685de010000 movzx eax, byte ptr [rbp + 0x1de]
0x140b40fd1 88851f010000 mov byte ptr [rbp + 0x11f], al
0x140b40fd7 c7851001000000000000 mov dword ptr [rbp + 0x110], 0
0x140b40fe1 c685e301000000 mov byte ptr [rbp + 0x1e3], 0
0x140b40fe8 488d9510010000 lea rdx, [rbp + 0x110]
0x140b40fef 4889f9 mov rcx, rdi
0x140b40ff2 e8092c9100 call 0x141453c00
0x140b40ff7 448926 mov dword ptr [rsi], r12d
0x140b40ffa 44896e04 mov dword ptr [rsi + 4], r13d
0x140b40ffe 0fb685e2010000 movzx eax, byte ptr [rbp + 0x1e2]
0x140b41005 884608 mov byte ptr [rsi + 8], al
0x140b41008 0fb785dc010000 movzx eax, word ptr [rbp + 0x1dc]
0x140b4100f 66894609 mov word ptr [rsi + 9], ax
0x140b41013 0fb685de010000 movzx eax, byte ptr [rbp + 0x1de]
0x140b4101a 88460b mov byte ptr [rsi + 0xb], al
0x140b4101d c7460c00000000 mov dword ptr [rsi + 0xc], 0
0x140b41024 e9f6020000 jmp 0x140b4131f
0x140b41029 4c8b6b08 mov r13, qword ptr [rbx + 8]
0x140b4102d 418b5df0 mov ebx, dword ptr [r13 - 0x10]
0x140b41031 458b75f4 mov r14d, dword ptr [r13 - 0xc]
0x140b41035 410fb645f8 movzx eax, byte ptr [r13 - 8]
0x140b4103a 4869c090030000 imul rax, rax, 0x390
0x140b41041 488d0c07 lea rcx, [rdi + rax]
0x140b41045 4883c120 add rcx, 0x20
0x140b41049 c685e701000001 mov byte ptr [rbp + 0x1e7], 1
0x140b41050 c685e601000001 mov byte ptr [rbp + 0x1e6], 1
0x140b41057 89da mov edx, ebx
0x140b41059 4589f0 mov r8d, r14d
0x140b4105c e8df280500 call 0x140b93940
0x140b41061 4885c0 test rax, rax
0x140b41064 0f84f4030000 je 0x140b4145e
0x140b4106a f30f104044 movss xmm0, dword ptr [rax + 0x44]
0x140b4106f 410f2ec0 ucomiss xmm0, xmm8
0x140b41073 7506 jne 0x140b4107b
0x140b41075 0f8bd1020000 jnp 0x140b4134c
0x140b4107b 4d8d45e8 lea r8, [r13 - 0x18]
0x140b4107f c685e701000001 mov byte ptr [rbp + 0x1e7], 1
0x140b41086 c685e601000001 mov byte ptr [rbp + 0x1e6], 1
0x140b4108d 488d8d10010000 lea rcx, [rbp + 0x110]
0x140b41094 4889fa mov rdx, rdi
0x140b41097 e8f484d9ff call 0x1408d9590
0x140b4109c 488b8548010000 mov rax, qword ptr [rbp + 0x148]
0x140b410a3 f048ff08 lock dec qword ptr [rax]
0x140b410a7 751a jne 0x140b410c3
0x140b410a9 488d8d48010000 lea rcx, [rbp + 0x148]
0x140b410b0 c685e701000001 mov byte ptr [rbp + 0x1e7], 1
0x140b410b7 c685e601000001 mov byte ptr [rbp + 0x1e6], 1
0x140b410be e85db50100 call 0x140b5c620
0x140b410c3 f3410f6f07 movdqu xmm0, xmmword ptr [r15]
0x140b410c8 410f104f10 movups xmm1, xmmword ptr [r15 + 0x10]
0x140b410cd 0f298d20010000 movaps xmmword ptr [rbp + 0x120], xmm1
0x140b410d4 660f7f8510010000 movdqa xmmword ptr [rbp + 0x110], xmm0
0x140b410dc c685e701000001 mov byte ptr [rbp + 0x1e7], 1
0x140b410e3 c685e601000000 mov byte ptr [rbp + 0x1e6], 0
0x140b410ea 488d9510010000 lea rdx, [rbp + 0x110]
0x140b410f1 4889f9 mov rcx, rdi
0x140b410f4 e8a77da100 call 0x141558ea0
0x140b410f9 418945e8 mov dword ptr [r13 - 0x18], eax
0x140b410fd 418955ec mov dword ptr [r13 - 0x14], edx
0x140b41101 0f284560 movaps xmm0, xmmword ptr [rbp + 0x60]
0x140b41105 0f284d70 movaps xmm1, xmmword ptr [rbp + 0x70]
0x140b41109 0f289590000000 movaps xmm2, xmmword ptr [rbp + 0x90]
0x140b41110 0f289da0000000 movaps xmm3, xmmword ptr [rbp + 0xa0]
0x140b41117 0f298d20010000 movaps xmmword ptr [rbp + 0x120], xmm1
0x140b4111e 0f298510010000 movaps xmmword ptr [rbp + 0x110], xmm0
0x140b41125 0f299540010000 movaps xmmword ptr [rbp + 0x140], xmm2
0x140b4112c 0f299d50010000 movaps xmmword ptr [rbp + 0x150], xmm3
0x140b41133 0f2885b0000000 movaps xmm0, xmmword ptr [rbp + 0xb0]
0x140b4113a 0f298560010000 movaps xmmword ptr [rbp + 0x160], xmm0
0x140b41141 0f2885c0000000 movaps xmm0, xmmword ptr [rbp + 0xc0]
0x140b41148 0f298570010000 movaps xmmword ptr [rbp + 0x170], xmm0
0x140b4114f 0f2885d0000000 movaps xmm0, xmmword ptr [rbp + 0xd0]
0x140b41156 0f298580010000 movaps xmmword ptr [rbp + 0x180], xmm0
0x140b4115d 48c1e020 shl rax, 0x20
0x140b41161 89d1 mov ecx, edx
0x140b41163 4809c1 or rcx, rax
0x140b41166 48898d30010000 mov qword ptr [rbp + 0x130], rcx
0x140b4116d 48c7853801000000000000 mov qword ptr [rbp + 0x138], 0
0x140b41178 488d4da0 lea rcx, [rbp - 0x60]
0x140b4117c 488d9510010000 lea rdx, [rbp + 0x110]
0x140b41183 e8781e0500 call 0x140b93000
0x140b41188 488b8520010000 mov rax, qword ptr [rbp + 0x120]
0x140b4118f f048ff08 lock dec qword ptr [rax]
0x140b41193 750c jne 0x140b411a1
0x140b41195 488d8d20010000 lea rcx, [rbp + 0x120]
0x140b4119c e82fe8b0ff call 0x14064f9d0
0x140b411a1 4c8d7f20 lea r15, [rdi + 0x20]
0x140b411a5 83bd1001000002 cmp dword ptr [rbp + 0x110], 2
0x140b411ac 7217 jb 0x140b411c5
0x140b411ae 488b8d18010000 mov rcx, qword ptr [rbp + 0x118]
0x140b411b5 ba2c000000 mov edx, 0x2c
0x140b411ba 41b804000000 mov r8d, 4
0x140b411c0 e85b2aadff call 0x140613c20
0x140b411c5 4d69e490030000 imul r12, r12, 0x390
0x140b411cc 43c684278803000001 mov byte ptr [r15 + r12 + 0x388], 1
0x140b411d5 c685e401000001 mov byte ptr [rbp + 0x1e4], 1
0x140b411dc 488d8d10010000 lea rcx, [rbp + 0x110]
0x140b411e3 488d55a0 lea rdx, [rbp - 0x60]
0x140b411e7 e8741d0500 call 0x140b92f60
0x140b411ec 4d01e7 add r15, r12
0x140b411ef 498d9788010000 lea rdx, [r15 + 0x188]
0x140b411f6 f30f108510010000 movss xmm0, dword ptr [rbp + 0x110]
0x140b411fe f30f108d24010000 movss xmm1, dword ptr [rbp + 0x124]
0x140b41206 f30f1185f0000000 movss dword ptr [rbp + 0xf0], xmm0
0x140b4120e f20f108518010000 movsd xmm0, qword ptr [rbp + 0x118]
0x140b41216 f20f1185f4000000 movsd qword ptr [rbp + 0xf4], xmm0
0x140b4121e f30f118dfc000000 movss dword ptr [rbp + 0xfc], xmm1
0x140b41226 c685e401000001 mov byte ptr [rbp + 0x1e4], 1
0x140b4122d 488d8df0000000 lea rcx, [rbp + 0xf0]
0x140b41234 e8a7d6f9ff call 0x140ade8e0
0x140b41239 4c8da510010000 lea r12, [rbp + 0x110]
0x140b41240 488d55a0 lea rdx, [rbp - 0x60]
0x140b41244 41b8b0000000 mov r8d, 0xb0
0x140b4124a 4c89e1 mov rcx, r12
0x140b4124d e8ae29de01 call 0x142923c00
0x140b41252 c685e401000000 mov byte ptr [rbp + 0x1e4], 0
0x140b41259 4c89f9 mov rcx, r15
0x140b4125c 4c89e2 mov rdx, r12
0x140b4125f e84c17a100 call 0x1415529b0
0x140b41264 4189c7 mov r15d, eax
0x140b41267 4189d4 mov r12d, edx
0x140b4126a 498d45f9 lea rax, [r13 - 7]
0x140b4126e 45897df0 mov dword ptr [r13 - 0x10], r15d
0x140b41272 418955f4 mov dword ptr [r13 - 0xc], edx
0x140b41276 0fb695e2010000 movzx edx, byte ptr [rbp + 0x1e2]
0x140b4127d 418855f8 mov byte ptr [r13 - 8], dl
0x140b41281 0fb68de1010000 movzx ecx, byte ptr [rbp + 0x1e1]
0x140b41288 884802 mov byte ptr [rax + 2], cl
0x140b4128b 0fb78ddf010000 movzx ecx, word ptr [rbp + 0x1df]
0x140b41292 668908 mov word ptr [rax], cx
0x140b41295 4881c7b03c0000 add rdi, 0x3cb0
0x140b4129c 4489bd14010000 mov dword ptr [rbp + 0x114], r15d
0x140b412a3 4489a518010000 mov dword ptr [rbp + 0x118], r12d
0x140b412aa 88951c010000 mov byte ptr [rbp + 0x11c], dl
0x140b412b0 0fb785df010000 movzx eax, word ptr [rbp + 0x1df]
0x140b412b7 6689851d010000 mov word ptr [rbp + 0x11d], ax
0x140b412be 0fb685e1010000 movzx eax, byte ptr [rbp + 0x1e1]
0x140b412c5 88851f010000 mov byte ptr [rbp + 0x11f], al
0x140b412cb c7851001000000000000 mov dword ptr [rbp + 0x110], 0
0x140b412d5 c685e401000000 mov byte ptr [rbp + 0x1e4], 0
0x140b412dc 488d9510010000 lea rdx, [rbp + 0x110]
0x140b412e3 4889f9 mov rcx, rdi
0x140b412e6 e815299100 call 0x141453c00
0x140b412eb 44893e mov dword ptr [rsi], r15d
0x140b412ee 44896604 mov dword ptr [rsi + 4], r12d
0x140b412f2 0fb685e2010000 movzx eax, byte ptr [rbp + 0x1e2]
0x140b412f9 884608 mov byte ptr [rsi + 8], al
0x140b412fc 0fb785df010000 movzx eax, word ptr [rbp + 0x1df]
0x140b41303 66894609 mov word ptr [rsi + 9], ax
0x140b41307 0fb685e1010000 movzx eax, byte ptr [rbp + 0x1e1]
0x140b4130e 88460b mov byte ptr [rsi + 0xb], al
0x140b41311 c7460c01000000 mov dword ptr [rsi + 0xc], 1
0x140b41318 895e10 mov dword ptr [rsi + 0x10], ebx
0x140b4131b 44897614 mov dword ptr [rsi + 0x14], r14d
0x140b4131f 4889f0 mov rax, rsi
0x140b41322 0f28b5f0010000 movaps xmm6, xmmword ptr [rbp + 0x1f0]
0x140b41329 0f28bd00020000 movaps xmm7, xmmword ptr [rbp + 0x200]
0x140b41330 440f288510020000 movaps xmm8, xmmword ptr [rbp + 0x210]
0x140b41338 4881c4a8020000 add rsp, 0x2a8
0x140b4133f 5b pop rbx
0x140b41340 5f pop rdi
0x140b41341 5e pop rsi
0x140b41342 415c pop r12
0x140b41344 415d pop r13
0x140b41346 415e pop r14
0x140b41348 415f pop r15
0x140b4134a 5d pop rbp
0x140b4134b c3 ret
0x140b4134c f3410f16c0 movshdup xmm0, xmm8
0x140b41351 f30f104848 movss xmm1, dword ptr [rax + 0x48]
0x140b41356 0f2ec8 ucomiss xmm1, xmm0
0x140b41359 0f851cfdffff jne 0x140b4107b
0x140b4135f 0f8a16fdffff jp 0x140b4107b
0x140b41365 f30f10404c movss xmm0, dword ptr [rax + 0x4c]
0x140b4136a 0f2ec7 ucomiss xmm0, xmm7
0x140b4136d 0f8508fdffff jne 0x140b4107b
0x140b41373 0f8a02fdffff jp 0x140b4107b
0x140b41379 0f104034 movups xmm0, xmmword ptr [rax + 0x34]
0x140b4137d 0fc2c600 cmpeqps xmm0, xmm6
0x140b41381 0f50c8 movmskps ecx, xmm0
0x140b41384 83f90f cmp ecx, 0xf
0x140b41387 0f85eefcffff jne 0x140b4107b
0x140b4138d 4c8b00 mov r8, qword ptr [rax]
0x140b41390 4c89c2 mov rdx, r8
0x140b41393 48c1ea20 shr rdx, 0x20
0x140b41397 83ca01 or edx, 1
0x140b4139a c685e701000001 mov byte ptr [rbp + 0x1e7], 1
0x140b413a1 c685e601000001 mov byte ptr [rbp + 0x1e6], 1
0x140b413a8 4889f9 mov rcx, rdi
0x140b413ab e820c8d8ff call 0x1408cdbd0
0x140b413b0 c685e701000001 mov byte ptr [rbp + 0x1e7], 1
0x140b413b7 c685e601000001 mov byte ptr [rbp + 0x1e6], 1
0x140b413be 4889c1 mov rcx, rax
0x140b413c1 4c89fa mov rdx, r15
0x140b413c4 e867d6d8ff call 0x1408cea30
0x140b413c9 84c0 test al, al
0x140b413cb 0f84aafcffff je 0x140b4107b
0x140b413d1 4983c5f0 add r13, -0x10
0x140b413d5 418b4508 mov eax, dword ptr [r13 + 8]
0x140b413d9 894608 mov dword ptr [rsi + 8], eax
0x140b413dc 498b4500 mov rax, qword ptr [r13]
0x140b413e0 488906 mov qword ptr [rsi], rax
0x140b413e3 c7460c02000000 mov dword ptr [rsi + 0xc], 2
0x140b413ea 498b4500 mov rax, qword ptr [r13]
0x140b413ee 48894610 mov qword ptr [rsi + 0x10], rax
0x140b413f2 c685e501000001 mov byte ptr [rbp + 0x1e5], 1
0x140b413f9 488d4d60 lea rcx, [rbp + 0x60]
0x140b413fd e83e8aabff call 0x1405f9e40
0x140b41402 488b8dd0010000 mov rcx, qword ptr [rbp + 0x1d0]
0x140b41409 488b01 mov rax, qword ptr [rcx]
0x140b4140c f048ff08 lock dec qword ptr [rax]
0x140b41410 0f8509ffffff jne 0x140b4131f
0x140b41416 e805b20100 call 0x140b5c620
0x140b4141b e9fffeffff jmp 0x140b4131f
0x140b41420 41ba10000000 mov r10d, 0x10
0x140b41426 4d01d1 add r9, r10
0x140b41429 4921d1 and r9, rdx
0x140b4142c f3420f6f0408 movdqu xmm0, xmmword ptr [rax + r9]
0x140b41432 66440fd7c0 pmovmskb r8d, xmm0
0x140b41437 4983c210 add r10, 0x10
0x140b4143b 4585c0 test r8d, r8d
0x140b4143e 0f85d2faffff jne 0x140b40f16
0x140b41444 ebe0 jmp 0x140b41426
0x140b41446 660f6f00 movdqa xmm0, xmmword ptr [rax]
0x140b4144a 66440fd7c0 pmovmskb r8d, xmm0
0x140b4144f f3450fbcc0 tzcnt r8d, r8d
0x140b41454 460fb60c00 movzx r9d, byte ptr [rax + r8]
0x140b41459 e9d5faffff jmp 0x140b40f33
0x140b4145e c685e701000001 mov byte ptr [rbp + 0x1e7], 1
0x140b41465 c685e601000001 mov byte ptr [rbp + 0x1e6], 1
0x140b4146c 488d0ddd28f601 lea rcx, [rip + 0x1f628dd]
0x140b41473 4c8d051e29f601 lea r8, [rip + 0x1f6291e]
0x140b4147a ba42000000 mov edx, 0x42
0x140b4147f e8ac7ed901 call 0x1428d9330
0x140b41484 0f0b ud2
0x140b41486 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140b41490 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b41495 55 push rbp
0x140b41496 4157 push r15
0x140b41498 4156 push r14
0x140b4149a 4155 push r13
0x140b4149c 4154 push r12
0x140b4149e 56 push rsi
0x140b4149f 57 push rdi
0x140b414a0 53 push rbx
0x140b414a1 4883ec58 sub rsp, 0x58
0x140b414a5 488daa80000000 lea rbp, [rdx + 0x80]
0x140b414ac 440f29442420 movaps xmmword ptr [rsp + 0x20], xmm8
0x140b414b2 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
0x140b414b7 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b414bc 83bd1001000002 cmp dword ptr [rbp + 0x110], 2
0x140b414c3 7217 jb 0x140b414dc
0x140b414c5 488b8d18010000 mov rcx, qword ptr [rbp + 0x118]
0x140b414cc ba2c000000 mov edx, 0x2c
0x140b414d1 41b804000000 mov r8d, 4
0x140b414d7 e84427adff call 0x140613c20
0x140b414dc c685e401000001 mov byte ptr [rbp + 0x1e4], 1
0x140b414e3 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b414e8 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b414ed 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140b414f3 4883c458 add rsp, 0x58
0x140b414f7 5b pop rbx
0x140b414f8 5f pop rdi
0x140b414f9 5e pop rsi
0x140b414fa 415c pop r12
0x140b414fc 415d pop r13
0x140b414fe 415e pop r14
0x140b41500 415f pop r15
0x140b41502 5d pop rbp
0x140b41503 c3 ret
0x140b41504 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140b41510 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b41515 55 push rbp
0x140b41516 4157 push r15
0x140b41518 4156 push r14
0x140b4151a 4155 push r13
0x140b4151c 4154 push r12
0x140b4151e 56 push rsi
0x140b4151f 57 push rdi
0x140b41520 53 push rbx
0x140b41521 4883ec58 sub rsp, 0x58
0x140b41525 488daa80000000 lea rbp, [rdx + 0x80]
0x140b4152c 440f29442420 movaps xmmword ptr [rsp + 0x20], xmm8
0x140b41532 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
0x140b41537 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b4153c 80bde401000000 cmp byte ptr [rbp + 0x1e4], 0
0x140b41543 7438 je 0x140b4157d
0x140b41545 488d4da0 lea rcx, [rbp - 0x60]
0x140b41549 e8522fabff call 0x1405f44a0
0x140b4154e c685e701000000 mov byte ptr [rbp + 0x1e7], 0
0x140b41555 c685e601000000 mov byte ptr [rbp + 0x1e6], 0
0x140b4155c 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b41561 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b41566 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140b4156c 4883c458 add rsp, 0x58
0x140b41570 5b pop rbx
0x140b41571 5f pop rdi
0x140b41572 5e pop rsi
0x140b41573 415c pop r12
0x140b41575 415d pop r13
0x140b41577 415e pop r14
0x140b41579 415f pop r15
0x140b4157b 5d pop rbp
0x140b4157c c3 ret
0x140b4157d c685e701000000 mov byte ptr [rbp + 0x1e7], 0
0x140b41584 c685e601000000 mov byte ptr [rbp + 0x1e6], 0
0x140b4158b 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b41590 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b41595 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140b4159b 4883c458 add rsp, 0x58
0x140b4159f 5b pop rbx
0x140b415a0 5f pop rdi
0x140b415a1 5e pop rsi
0x140b415a2 415c pop r12
0x140b415a4 415d pop r13
0x140b415a6 415e pop r14
0x140b415a8 415f pop r15
0x140b415aa 5d pop rbp
0x140b415ab c3 ret
0x140b415ac 0f1f4000 nop dword ptr [rax]
0x140b415b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b415b5 55 push rbp
0x140b415b6 4157 push r15
0x140b415b8 4156 push r14
0x140b415ba 4155 push r13
0x140b415bc 4154 push r12
0x140b415be 56 push rsi
0x140b415bf 57 push rdi
0x140b415c0 53 push rbx
0x140b415c1 4883ec58 sub rsp, 0x58
0x140b415c5 488daa80000000 lea rbp, [rdx + 0x80]
0x140b415cc 440f29442420 movaps xmmword ptr [rsp + 0x20], xmm8
0x140b415d2 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
0x140b415d7 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b415dc 83bd1001000002 cmp dword ptr [rbp + 0x110], 2
0x140b415e3 7217 jb 0x140b415fc
0x140b415e5 488b8d18010000 mov rcx, qword ptr [rbp + 0x118]
0x140b415ec ba2c000000 mov edx, 0x2c
0x140b415f1 41b804000000 mov r8d, 4
0x140b415f7 e82426adff call 0x140613c20
0x140b415fc c685e301000001 mov byte ptr [rbp + 0x1e3], 1
0x140b41603 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b41608 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b4160d 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140b41613 4883c458 add rsp, 0x58
0x140b41617 5b pop rbx
0x140b41618 5f pop rdi
0x140b41619 5e pop rsi
0x140b4161a 415c pop r12
0x140b4161c 415d pop r13
0x140b4161e 415e pop r14
0x140b41620 415f pop r15
0x140b41622 5d pop rbp
0x140b41623 c3 ret
0x140b41624 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140b41630 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b41635 55 push rbp
0x140b41636 4157 push r15
0x140b41638 4156 push r14
0x140b4163a 4155 push r13
0x140b4163c 4154 push r12
0x140b4163e 56 push rsi
0x140b4163f 57 push rdi
0x140b41640 53 push rbx
0x140b41641 4883ec58 sub rsp, 0x58
0x140b41645 488daa80000000 lea rbp, [rdx + 0x80]
0x140b4164c 440f29442420 movaps xmmword ptr [rsp + 0x20], xmm8
0x140b41652 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
0x140b41657 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b4165c 80bde301000000 cmp byte ptr [rbp + 0x1e3], 0
0x140b41663 7438 je 0x140b4169d
0x140b41665 488d4da0 lea rcx, [rbp - 0x60]
0x140b41669 e8322eabff call 0x1405f44a0
0x140b4166e c685e701000000 mov byte ptr [rbp + 0x1e7], 0
0x140b41675 c685e601000000 mov byte ptr [rbp + 0x1e6], 0
0x140b4167c 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b41681 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b41686 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140b4168c 4883c458 add rsp, 0x58
0x140b41690 5b pop rbx
0x140b41691 5f pop rdi
0x140b41692 5e pop rsi
0x140b41693 415c pop r12
0x140b41695 415d pop r13
0x140b41697 415e pop r14
0x140b41699 415f pop r15
0x140b4169b 5d pop rbp
0x140b4169c c3 ret
0x140b4169d c685e701000000 mov byte ptr [rbp + 0x1e7], 0
0x140b416a4 c685e601000000 mov byte ptr [rbp + 0x1e6], 0
0x140b416ab 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b416b0 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b416b5 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140b416bb 4883c458 add rsp, 0x58
0x140b416bf 5b pop rbx
0x140b416c0 5f pop rdi
0x140b416c1 5e pop rsi
0x140b416c2 415c pop r12
0x140b416c4 415d pop r13
0x140b416c6 415e pop r14
0x140b416c8 415f pop r15
0x140b416ca 5d pop rbp
0x140b416cb c3 ret
0x140b416cc 0f1f4000 nop dword ptr [rax]
0x140b416d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b416d5 55 push rbp
0x140b416d6 4157 push r15
0x140b416d8 4156 push r14
0x140b416da 4155 push r13
0x140b416dc 4154 push r12
0x140b416de 56 push rsi
0x140b416df 57 push rdi
0x140b416e0 53 push rbx
0x140b416e1 4883ec58 sub rsp, 0x58
0x140b416e5 488daa80000000 lea rbp, [rdx + 0x80]
0x140b416ec 440f29442420 movaps xmmword ptr [rsp + 0x20], xmm8
0x140b416f2 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
0x140b416f7 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b416fc 0fb69de6010000 movzx ebx, byte ptr [rbp + 0x1e6]
0x140b41703 80bde701000000 cmp byte ptr [rbp + 0x1e7], 0
0x140b4170a 7433 je 0x140b4173f
0x140b4170c 488d4d60 lea rcx, [rbp + 0x60]
0x140b41710 e82b87abff call 0x1405f9e40
0x140b41715 80e301 and bl, 1
0x140b41718 889de5010000 mov byte ptr [rbp + 0x1e5], bl
0x140b4171e 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b41723 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b41728 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140b4172e 4883c458 add rsp, 0x58
0x140b41732 5b pop rbx
0x140b41733 5f pop rdi
0x140b41734 5e pop rsi
0x140b41735 415c pop r12
0x140b41737 415d pop r13
0x140b41739 415e pop r14
0x140b4173b 415f pop r15
0x140b4173d 5d pop rbp
0x140b4173e c3 ret
0x140b4173f 80e301 and bl, 1
0x140b41742 889de5010000 mov byte ptr [rbp + 0x1e5], bl
0x140b41748 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b4174d 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b41752 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140b41758 4883c458 add rsp, 0x58
0x140b4175c 5b pop rbx
0x140b4175d 5f pop rdi
0x140b4175e 5e pop rsi
0x140b4175f 415c pop r12
0x140b41761 415d pop r13
0x140b41763 415e pop r14
0x140b41765 415f pop r15
0x140b41767 5d pop rbp
0x140b41768 c3 ret
0x140b41769 0f1f8000000000 nop dword ptr [rax]
0x140b41770 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b41775 55 push rbp
0x140b41776 4157 push r15
0x140b41778 4156 push r14
0x140b4177a 4155 push r13
0x140b4177c 4154 push r12
0x140b4177e 56 push rsi
0x140b4177f 57 push rdi
0x140b41780 53 push rbx
0x140b41781 4883ec58 sub rsp, 0x58
0x140b41785 488daa80000000 lea rbp, [rdx + 0x80]
0x140b4178c 440f29442420 movaps xmmword ptr [rsp + 0x20], xmm8
0x140b41792 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
0x140b41797 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b4179c 80bde501000000 cmp byte ptr [rbp + 0x1e5], 0
0x140b417a3 741c je 0x140b417c1
0x140b417a5 488b85d0010000 mov rax, qword ptr [rbp + 0x1d0]
0x140b417ac 488b00 mov rax, qword ptr [rax]
0x140b417af f048ff08 lock dec qword ptr [rax]
0x140b417b3 750c jne 0x140b417c1
0x140b417b5 488b8dd0010000 mov rcx, qword ptr [rbp + 0x1d0]
0x140b417bc e85fae0100 call 0x140b5c620
0x140b417c1 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b417c6 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b417cb 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140b417d1 4883c458 add rsp, 0x58
0x140b417d5 5b pop rbx
0x140b417d6 5f pop rdi
0x140b417d7 5e pop rsi
0x140b417d8 415c pop r12
0x140b417da 415d pop r13
0x140b417dc 415e pop r14
0x140b417de 415f pop r15
0x140b417e0 5d pop rbp
0x140b417e1 c3 ret
0x140b417e2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140b417f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b417f5 55 push rbp
0x140b417f6 4157 push r15
0x140b417f8 4156 push r14
0x140b417fa 4155 push r13
0x140b417fc 4154 push r12
0x140b417fe 56 push rsi
0x140b417ff 57 push rdi
0x140b41800 53 push rbx
0x140b41801 4883ec58 sub rsp, 0x58
0x140b41805 488daa80000000 lea rbp, [rdx + 0x80]
0x140b4180c 440f29442420 movaps xmmword ptr [rsp + 0x20], xmm8
0x140b41812 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
0x140b41817 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b4181c 488d8d10010000 lea rcx, [rbp + 0x110]
0x140b41823 e81886abff call 0x1405f9e40
0x140b41828 c685e701000000 mov byte ptr [rbp + 0x1e7], 0
0x140b4182f c685e601000000 mov byte ptr [rbp + 0x1e6], 0
0x140b41836 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b4183b 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b41840 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140b41846 4883c458 add rsp, 0x58
0x140b4184a 5b pop rbx
0x140b4184b 5f pop rdi
0x140b4184c 5e pop rsi
0x140b4184d 415c pop r12
0x140b4184f 415d pop r13
0x140b41851 415e pop r14
0x140b41853 415f pop r15
0x140b41855 5d pop rbp
0x140b41856 c3 ret
0x140b41857 660f1f840000000000 nop word ptr [rax + rax]
0x140b41860 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b41865 55 push rbp
0x140b41866 4157 push r15
0x140b41868 4156 push r14
0x140b4186a 4155 push r13
0x140b4186c 4154 push r12
0x140b4186e 56 push rsi
0x140b4186f 57 push rdi
0x140b41870 53 push rbx
0x140b41871 4883ec58 sub rsp, 0x58
0x140b41875 488daa80000000 lea rbp, [rdx + 0x80]
0x140b4187c 440f29442420 movaps xmmword ptr [rsp + 0x20], xmm8
0x140b41882 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
0x140b41887 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x140b4188c 488d8d10010000 lea rcx, [rbp + 0x110]
0x140b41893 e8a885abff call 0x1405f9e40
0x140b41898 c685e701000000 mov byte ptr [rbp + 0x1e7], 0
0x140b4189f c685e601000000 mov byte ptr [rbp + 0x1e6], 0
0x140b418a6 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b418ab 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b418b0 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140b418b6 4883c458 add rsp, 0x58
0x140b418ba 5b pop rbx
0x140b418bb 5f pop rdi
0x140b418bc 5e pop rsi
0x140b418bd 415c pop r12
0x140b418bf 415d pop r13
0x140b418c1 415e pop r14
0x140b418c3 415f pop r15
0x140b418c5 5d pop rbp
0x140b418c6 c3 ret

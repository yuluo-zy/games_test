14143fb50: push rbp
14143fb51: push r15
14143fb53: push r14
14143fb55: push r12
14143fb57: push rsi
14143fb58: push rdi
14143fb59: push rbx
14143fb5a: sub rsp, 0xe0
14143fb61: lea rbp, [rsp + 0x80]
14143fb69: mov qword ptr [rbp + 0x58], 0xfffffffffffffffe
14143fb71: mov rsi, r9
14143fb74: mov edi, r8d
14143fb77: mov ebx, edx
14143fb79: mov rax, qword ptr [rcx]
14143fb7c: lea rdx, [rip + 0x181f701]
14143fb83: movsxd rax, dword ptr [rdx + rax*4]
14143fb87: add rax, rdx
14143fb8a: jmp rax
14143fb8c: test bl, bl
14143fb8e: je 0x1414400f5
14143fb94: test dil, dil
14143fb97: je 0x141440785
14143fb9d: mov rax, qword ptr [rcx + 8]
14143fba1: lock inc qword ptr [rax]
14143fba5: jle 0x141441b81
14143fbab: mov qword ptr [rbp + 0x30], rax
14143fbaf: mov qword ptr [rbp + 0x38], 0
14143fbb7: mov dword ptr [rbp + 0x40], 2
14143fbbe: mov byte ptr [rbp + 0x44], 1
14143fbc2: jmp 0x1414407d2
14143fbc7: mov rax, qword ptr [rcx + 8]
14143fbcb: lock inc qword ptr [rax]
14143fbcf: jle 0x141441b81
14143fbd5: xor dil, 1
14143fbd9: movzx ecx, dil
14143fbdd: mov dword ptr [rbp + 0x30], ecx
14143fbe0: mov dword ptr [rbp + 0x34], 0x3e4ccccd
14143fbe7: mov qword ptr [rbp + 0x38], rax
14143fbeb: mov dword ptr [rbp + 0x40], 0
14143fbf2: mov byte ptr [rbp + 0x44], bl
14143fbf5: mov byte ptr [rbp + 0x45], 0
14143fbf9: mov byte ptr [rbp + 0x28], 0xa
14143fbfd: mov rdi, qword ptr [rsi + 0x10]
14143fc01: cmp rdi, qword ptr [rsi]
14143fc04: jne 0x141440b61
14143fc0a: lea rdx, [rip + 0x181f4bf]
14143fc11: mov rcx, rsi
14143fc14: call 0x1405629f0
14143fc19: jmp 0x141440b61
14143fc1e: mov rax, qword ptr [rcx + 8]
14143fc22: lock inc qword ptr [rax]
14143fc26: jle 0x141441b81
14143fc2c: xor dil, 1
14143fc30: movzx ecx, dil
14143fc34: mov dword ptr [rbp + 0x30], ecx
14143fc37: mov dword ptr [rbp + 0x34], 0x3dcccccd
14143fc3e: mov qword ptr [rbp + 0x38], rax
14143fc42: mov dword ptr [rbp + 0x40], 0
14143fc49: mov byte ptr [rbp + 0x44], bl
14143fc4c: mov byte ptr [rbp + 0x45], 0
14143fc50: mov byte ptr [rbp + 0x28], 9
14143fc54: mov rdi, qword ptr [rsi + 0x10]
14143fc58: cmp rdi, qword ptr [rsi]
14143fc5b: jne 0x141440b61
14143fc61: lea rdx, [rip + 0x181f468]
14143fc68: mov rcx, rsi
14143fc6b: call 0x1405629f0
14143fc70: jmp 0x141440b61
14143fc75: mov r15, qword ptr [rcx + 8]
14143fc79: lea r14, [r15 + 0x10]
14143fc7d: mov eax, 1
14143fc82: lock xadd qword ptr [r15], rax
14143fc87: test bl, bl
14143fc89: je 0x14143ffd5
14143fc8f: test dil, dil
14143fc92: je 0x1414401bb
14143fc98: test rax, rax
14143fc9b: js 0x141441b81
14143fca1: mov qword ptr [rbp + 0x30], r15
14143fca5: mov qword ptr [rbp + 0x28], r14
14143fca9: mov byte ptr [rbp + 0x40], 1
14143fcad: mov qword ptr [rbp + 0x38], 0
14143fcb5: mov word ptr [rbp + 0x41], 0x100
14143fcbb: jmp 0x1414401f3
14143fcc0: mov rax, qword ptr [rcx + 8]
14143fcc4: lock inc qword ptr [rax]
14143fcc8: jle 0x141441b81
14143fcce: xor dil, 1
14143fcd2: movzx ecx, dil
14143fcd6: mov dword ptr [rbp + 0x30], ecx
14143fcd9: mov dword ptr [rbp + 0x34], 0x3e4ccccd
14143fce0: mov qword ptr [rbp + 0x38], rax
14143fce4: mov dword ptr [rbp + 0x40], 0
14143fceb: mov byte ptr [rbp + 0x44], bl
14143fcee: mov byte ptr [rbp + 0x45], 0
14143fcf2: mov byte ptr [rbp + 0x28], 6
14143fcf6: mov rdi, qword ptr [rsi + 0x10]
14143fcfa: cmp rdi, qword ptr [rsi]
14143fcfd: jne 0x141440b61
14143fd03: lea rdx, [rip + 0x181f3c6]
14143fd0a: mov rcx, rsi
14143fd0d: call 0x1405629f0
14143fd12: jmp 0x141440b61
14143fd17: mov r15, qword ptr [rcx + 8]
14143fd1b: lea r14, [r15 + 0x10]
14143fd1f: mov eax, 1
14143fd24: lock xadd qword ptr [r15], rax
14143fd29: test bl, bl
14143fd2b: je 0x141440006
14143fd31: test dil, dil
14143fd34: je 0x1414403a9
14143fd3a: test rax, rax
14143fd3d: js 0x141441b81
14143fd43: mov qword ptr [rbp + 0x30], r15
14143fd47: mov qword ptr [rbp + 0x28], r14
14143fd4b: mov byte ptr [rbp + 0x40], 1
14143fd4f: mov qword ptr [rbp + 0x38], 0
14143fd57: mov word ptr [rbp + 0x41], 0x100
14143fd5d: jmp 0x1414403e1
14143fd62: mov r15, qword ptr [rcx + 8]
14143fd66: lea r14, [r15 + 0x10]
14143fd6a: mov eax, 1
14143fd6f: lock xadd qword ptr [r15], rax
14143fd74: test bl, bl
14143fd76: je 0x141440037
14143fd7c: test dil, dil
14143fd7f: je 0x141440597
14143fd85: test rax, rax
14143fd88: js 0x141441b81
14143fd8e: mov qword ptr [rbp + 0x30], r15
14143fd92: mov qword ptr [rbp + 0x28], r14
14143fd96: mov byte ptr [rbp + 0x40], 1
14143fd9a: mov qword ptr [rbp + 0x38], 0
14143fda2: mov word ptr [rbp + 0x41], 0x100
14143fda8: jmp 0x1414405cf
14143fdad: mov byte ptr [rbp - 9], bl
14143fdb0: mov byte ptr [rbp - 8], dil
14143fdb4: mov rdi, qword ptr [rcx + 8]
14143fdb8: mov r14, qword ptr [rcx + 0x10]
14143fdbc: movzx eax, byte ptr [rip + 0x21f4e46]
14143fdc3: mov ecx, 0x10
14143fdc8: mov edx, 8
14143fdcd: call 0x140613c10
14143fdd2: test bl, bl
14143fdd4: je 0x141440068
14143fdda: test rax, rax
14143fddd: je 0x141441b20
14143fde3: lea rcx, [rip + 0x16745de]
14143fdea: jmp 0x141440078
14143fdef: mov r15, qword ptr [rcx + 8]
14143fdf3: lea r14, [r15 + 0x10]
14143fdf7: mov eax, 1
14143fdfc: lock xadd qword ptr [r15], rax
14143fe01: test bl, bl
14143fe03: je 0x141440128
14143fe09: test dil, dil
14143fe0c: je 0x14144079e
14143fe12: test rax, rax
14143fe15: js 0x141441b81
14143fe1b: mov qword ptr [rbp + 0x30], r15
14143fe1f: mov qword ptr [rbp + 0x28], r14
14143fe23: mov byte ptr [rbp + 0x40], 1
14143fe27: mov qword ptr [rbp + 0x38], 0
14143fe2f: mov word ptr [rbp + 0x41], 0x100
14143fe35: jmp 0x141440802
14143fe3a: mov rdx, qword ptr [rcx + 8]
14143fe3e: lea r8, [rdx + 0x10]
14143fe42: mov eax, 1
14143fe47: lock xadd qword ptr [rdx], rax
14143fe4c: test bl, bl
14143fe4e: je 0x141440159
14143fe54: test dil, dil
14143fe57: je 0x1414409b4
14143fe5d: test rax, rax
14143fe60: js 0x141441b81
14143fe66: mov qword ptr [rbp - 0x30], rdx
14143fe6a: mov qword ptr [rbp - 0x38], r8
14143fe6e: mov byte ptr [rbp - 0x20], 1
14143fe72: mov qword ptr [rbp - 0x28], 0
14143fe7a: mov word ptr [rbp - 0x1f], 0x100
14143fe80: jmp 0x1414409e0
14143fe85: mov rdx, qword ptr [rcx + 8]
14143fe89: lea r8, [rdx + 0x10]
14143fe8d: mov eax, 1
14143fe92: lock xadd qword ptr [rdx], rax
14143fe97: test bl, bl
14143fe99: je 0x14144018a
14143fe9f: test dil, dil
14143fea2: je 0x141440a8f
14143fea8: test rax, rax
14143feab: js 0x141441b81
14143feb1: mov qword ptr [rbp - 0x50], rdx
14143feb5: mov qword ptr [rbp - 0x58], r8
14143feb9: mov byte ptr [rbp - 0x40], 1
14143febd: mov qword ptr [rbp - 0x48], 0
14143fec5: mov word ptr [rbp - 0x3f], 0x100
14143fecb: jmp 0x141440abb
14143fed0: mov rax, qword ptr [rcx + 8]
14143fed4: lock inc qword ptr [rax]
14143fed8: jle 0x141441b81
14143fede: xor dil, 1
14143fee2: movzx ecx, dil
14143fee6: mov dword ptr [rbp + 0x30], ecx
14143fee9: mov dword ptr [rbp + 0x34], 0x3e4ccccd
14143fef0: mov qword ptr [rbp + 0x38], rax
14143fef4: mov dword ptr [rbp + 0x40], 0
14143fefb: mov byte ptr [rbp + 0x44], bl
14143fefe: mov byte ptr [rbp + 0x45], 0
14143ff02: mov byte ptr [rbp + 0x28], 7
14143ff06: mov rdi, qword ptr [rsi + 0x10]
14143ff0a: cmp rdi, qword ptr [rsi]
14143ff0d: jne 0x141440b61
14143ff13: lea rdx, [rip + 0x181f1b6]
14143ff1a: mov rcx, rsi
14143ff1d: call 0x1405629f0
14143ff22: jmp 0x141440b61
14143ff27: mov rax, qword ptr [rcx + 8]
14143ff2b: lock inc qword ptr [rax]
14143ff2f: jle 0x141441b81
14143ff35: xor dil, 1
14143ff39: movzx ecx, dil
14143ff3d: mov dword ptr [rbp + 0x30], ecx
14143ff40: mov dword ptr [rbp + 0x34], 0x3e4ccccd
14143ff47: mov qword ptr [rbp + 0x38], rax
14143ff4b: mov dword ptr [rbp + 0x40], 0
14143ff52: mov byte ptr [rbp + 0x44], bl
14143ff55: mov byte ptr [rbp + 0x45], 0
14143ff59: mov byte ptr [rbp + 0x28], 5
14143ff5d: mov rdi, qword ptr [rsi + 0x10]
14143ff61: cmp rdi, qword ptr [rsi]
14143ff64: jne 0x141440b61
14143ff6a: lea rdx, [rip + 0x181f15f]
14143ff71: mov rcx, rsi
14143ff74: call 0x1405629f0
14143ff79: jmp 0x141440b61
14143ff7e: mov rax, qword ptr [rcx + 8]
14143ff82: lock inc qword ptr [rax]
14143ff86: jle 0x141441b81
14143ff8c: xor dil, 1
14143ff90: movzx ecx, dil
14143ff94: mov dword ptr [rbp + 0x30], ecx
14143ff97: mov dword ptr [rbp + 0x34], 0x3dcccccd
14143ff9e: mov qword ptr [rbp + 0x38], rax
14143ffa2: mov dword ptr [rbp + 0x40], 0
14143ffa9: mov byte ptr [rbp + 0x44], bl
14143ffac: mov byte ptr [rbp + 0x45], 0
14143ffb0: mov byte ptr [rbp + 0x28], 8
14143ffb4: mov rdi, qword ptr [rsi + 0x10]
14143ffb8: cmp rdi, qword ptr [rsi]
14143ffbb: jne 0x141440b61
14143ffc1: lea rdx, [rip + 0x181f108]
14143ffc8: mov rcx, rsi
14143ffcb: call 0x1405629f0
14143ffd0: jmp 0x141440b61
14143ffd5: test dil, dil
14143ffd8: je 0x1414401d8
14143ffde: test rax, rax
14143ffe1: js 0x141441b81
14143ffe7: mov qword ptr [rbp + 0x30], r15
14143ffeb: mov qword ptr [rbp + 0x28], r14
14143ffef: mov qword ptr [rbp + 0x38], 0
14143fff7: mov word ptr [rbp + 0x40], 0
14143fffd: mov byte ptr [rbp + 0x42], 1
141440001: jmp 0x1414401f3
141440006: test dil, dil
141440009: je 0x1414403c6
14144000f: test rax, rax
141440012: js 0x141441b81
141440018: mov qword ptr [rbp + 0x30], r15
14144001c: mov qword ptr [rbp + 0x28], r14
141440020: mov qword ptr [rbp + 0x38], 0
141440028: mov word ptr [rbp + 0x40], 0
14144002e: mov byte ptr [rbp + 0x42], 1
141440032: jmp 0x1414403e1
141440037: test dil, dil
14144003a: je 0x1414405b4
141440040: test rax, rax
141440043: js 0x141441b81
141440049: mov qword ptr [rbp + 0x30], r15
14144004d: mov qword ptr [rbp + 0x28], r14
141440051: mov qword ptr [rbp + 0x38], 0
141440059: mov word ptr [rbp + 0x40], 0
14144005f: mov byte ptr [rbp + 0x42], 1
141440063: jmp 0x1414405cf
141440068: test rax, rax
14144006b: je 0x141441b20
141440071: lea rcx, [rip + 0x1674318]
141440078: shl r14, 4
14144007c: add r14, rdi
14144007f: mov qword ptr [rax], rdi
141440082: mov qword ptr [rax + 8], r14
141440086: mov qword ptr [rbp + 0x28], rax
14144008a: mov qword ptr [rbp + 0x30], rcx
14144008e: lea rax, [rbp - 9]
141440092: mov qword ptr [rbp + 0x38], rax
141440096: lea rax, [rbp - 8]
14144009a: mov qword ptr [rbp + 0x40], rax
14144009e: lea r8, [rip + 0x1673c73]
1414400a5: mov rcx, rbp
1414400a8: lea rdx, [rbp + 0x28]
1414400ac: call 0x14130e950
1414400b1: movups xmm0, xmmword ptr [rbp]
1414400b5: mov rax, qword ptr [rbp + 0x10]
1414400b9: movzx ecx, byte ptr [rbp - 9]
1414400bd: movups xmmword ptr [rbp + 0x30], xmm0
1414400c1: mov qword ptr [rbp + 0x40], 0
1414400c9: mov qword ptr [rbp + 0x48], rax
1414400cd: mov byte ptr [rbp + 0x50], cl
1414400d0: mov byte ptr [rbp + 0x28], 4
1414400d4: mov rdi, qword ptr [rsi + 0x10]
1414400d8: cmp rdi, qword ptr [rsi]
1414400db: jne 0x141440b61
1414400e1: lea rdx, [rip + 0x181efe8]
1414400e8: mov rcx, rsi
1414400eb: call 0x1405629f0
1414400f0: jmp 0x141440b61
1414400f5: test dil, dil
1414400f8: je 0x1414407bb
1414400fe: mov rax, qword ptr [rcx + 8]
141440102: lock inc qword ptr [rax]
141440106: jle 0x141441b81
14144010c: mov qword ptr [rbp + 0x30], rax
141440110: mov qword ptr [rbp + 0x38], 0
141440118: mov dword ptr [rbp + 0x40], 2
14144011f: mov byte ptr [rbp + 0x44], 0
141440123: jmp 0x1414407d2
141440128: test dil, dil
14144012b: je 0x1414407e7
141440131: test rax, rax
141440134: js 0x141441b81
14144013a: mov qword ptr [rbp + 0x30], r15
14144013e: mov qword ptr [rbp + 0x28], r14
141440142: mov qword ptr [rbp + 0x38], 0
14144014a: mov word ptr [rbp + 0x40], 0
141440150: mov byte ptr [rbp + 0x42], 1
141440154: jmp 0x141440802
141440159: test dil, dil
14144015c: je 0x1414409cb
141440162: test rax, rax
141440165: js 0x141441b81
14144016b: mov qword ptr [rbp - 0x30], rdx
14144016f: mov qword ptr [rbp - 0x38], r8
141440173: mov qword ptr [rbp - 0x28], 0
14144017b: mov word ptr [rbp - 0x20], 0
141440181: mov byte ptr [rbp - 0x1e], 1
141440185: jmp 0x1414409e0
14144018a: test dil, dil
14144018d: je 0x141440aa6
141440193: test rax, rax
141440196: js 0x141441b81
14144019c: mov qword ptr [rbp - 0x50], rdx
1414401a0: mov qword ptr [rbp - 0x58], r8
1414401a4: mov qword ptr [rbp - 0x48], 0
1414401ac: mov word ptr [rbp - 0x40], 0
1414401b2: mov byte ptr [rbp - 0x3e], 1
1414401b6: jmp 0x141440abb
1414401bb: test rax, rax
1414401be: js 0x141441b81
1414401c4: lea rcx, [rbp + 0x28]
1414401c8: mov rdx, r15
1414401cb: mov r8, r14
1414401ce: mov r9b, 1
1414401d1: call 0x141700620
1414401d6: jmp 0x1414401f3
1414401d8: test rax, rax
1414401db: js 0x141441b81
1414401e1: lea rcx, [rbp + 0x28]
1414401e5: mov rdx, r15
1414401e8: mov r8, r14
1414401eb: mov r9b, 1
1414401ee: call 0x141700430
1414401f3: mov rax, qword ptr [rbp + 0x28]
1414401f7: mov rcx, qword ptr [rbp + 0x30]
1414401fb: mov rdx, qword ptr [rbp + 0x38]
1414401ff: mov qword ptr [rbp + 0x15], rdx
141440203: movzx edx, byte ptr [rbp + 0x40]
141440207: mov byte ptr [rbp + 0x1d], dl
14144020a: movzx edx, byte ptr [rbp + 0x41]
14144020e: mov byte ptr [rbp + 0x1e], dl
141440211: movzx edx, byte ptr [rbp + 0x42]
141440215: mov byte ptr [rbp + 0x1f], dl
141440218: mov edx, dword ptr [rbp + 0x43]
14144021b: mov dword ptr [rbp + 0x20], edx
14144021e: movzx edx, byte ptr [rbp + 0x47]
141440222: mov byte ptr [rbp + 0x24], dl
141440225: mov qword ptr [rbp + 5], rax
141440229: mov qword ptr [rbp + 0xd], rcx
14144022d: mov word ptr [rbp + 0x28], 0x100
141440233: mov byte ptr [rbp + 0x2a], 3
141440237: mov rax, qword ptr [rbp + 0x1d]
14144023b: mov qword ptr [rbp + 0x48], rax
14144023f: mov rax, qword ptr [rbp + 0x10]
141440243: mov qword ptr [rbp + 0x3b], rax
141440247: mov eax, dword ptr [rbp + 0x18]
14144024a: mov dword ptr [rbp + 0x43], eax
14144024d: movzx eax, byte ptr [rbp + 0x1c]
141440251: mov byte ptr [rbp + 0x47], al
141440254: movzx eax, byte ptr [rbp + 0x1d]
141440258: mov byte ptr [rbp + 0x48], al
14144025b: movzx eax, byte ptr [rbp + 0x1e]
14144025f: mov byte ptr [rbp + 0x49], al
141440262: movzx eax, byte ptr [rbp + 0x1f]
141440266: mov byte ptr [rbp + 0x4a], al
141440269: movups xmm0, xmmword ptr [rbp]
14144026d: movups xmmword ptr [rbp + 0x2b], xmm0
141440271: mov r12, qword ptr [rsi + 0x10]
141440275: cmp r12, qword ptr [rsi]
141440278: jne 0x141440289
14144027a: lea rdx, [rip + 0x181ee4f]
141440281: mov rcx, rsi
141440284: call 0x1405629f0
141440289: mov rax, qword ptr [rsi + 8]
14144028d: lea rcx, [r12 + r12*2]
141440291: shl rcx, 4
141440295: movups xmm0, xmmword ptr [rbp + 0x28]
141440299: movups xmm1, xmmword ptr [rbp + 0x38]
14144029d: movups xmm2, xmmword ptr [rbp + 0x48]
1414402a1: movups xmmword ptr [rax + rcx + 0x20], xmm2
1414402a6: movups xmmword ptr [rax + rcx + 0x10], xmm1
1414402ab: movups xmmword ptr [rax + rcx], xmm0
1414402af: inc r12
1414402b2: mov qword ptr [rsi + 0x10], r12
1414402b6: mov r12, qword ptr [r15 + 0x18]
1414402ba: mov rax, qword ptr [r15 + 0x20]
1414402be: lock inc qword ptr [r12]
1414402c3: jle 0x141441b81
1414402c9: movss xmm0, dword ptr [r15 + 0x28]
1414402cf: mov ecx, dword ptr [r15 + 0x10]
1414402d3: movss xmm1, dword ptr [r15 + 0x14]
1414402d9: mov qword ptr [rbp + 0x28], 1
1414402e1: mov qword ptr [rbp + 0x30], 1
1414402e9: mov dword ptr [rbp + 0x38], ecx
1414402ec: movss dword ptr [rbp + 0x3c], xmm1
1414402f1: mov qword ptr [rbp + 0x40], r12
1414402f5: mov qword ptr [rbp + 0x48], rax
1414402f9: movss dword ptr [rbp + 0x50], xmm0
1414402fe: mov byte ptr [rbp + 0x54], 0
141440302: movzx eax, byte ptr [rip + 0x21f4900]
141440309: mov ecx, 0x30
14144030e: mov edx, 8
141440313: call 0x140613c10
141440318: test rax, rax
14144031b: je 0x141441b2f
141440321: movups xmm0, xmmword ptr [rbp + 0x28]
141440325: movups xmm1, xmmword ptr [rbp + 0x38]
141440329: movups xmm2, xmmword ptr [rbp + 0x48]
14144032d: movups xmmword ptr [rax + 0x20], xmm2
141440331: movups xmmword ptr [rax + 0x10], xmm1
141440335: movups xmmword ptr [rax], xmm0
141440338: mov qword ptr [rbp - 8], rax
14144033c: mov ecx, 1
141440341: lock xadd qword ptr [rax], rcx
141440346: test bl, bl
141440348: je 0x14144037b
14144034a: test dil, dil
14144034d: je 0x141440ba0
141440353: test rcx, rcx
141440356: js 0x141441b81
14144035c: mov qword ptr [rbp + 0x30], rax
141440360: mov qword ptr [rbp + 0x28], r14
141440364: mov byte ptr [rbp + 0x40], 1
141440368: mov qword ptr [rbp + 0x38], 0
141440370: mov word ptr [rbp + 0x41], 0
141440376: jmp 0x141440bd8
14144037b: test dil, dil
14144037e: je 0x141440bbd
141440384: test rcx, rcx
141440387: js 0x141441b81
14144038d: mov qword ptr [rbp + 0x30], rax
141440391: mov qword ptr [rbp + 0x28], r14
141440395: mov qword ptr [rbp + 0x38], 0
14144039d: mov dword ptr [rbp + 0x3f], 0
1414403a4: jmp 0x141440bd8
1414403a9: test rax, rax
1414403ac: js 0x141441b81
1414403b2: lea rcx, [rbp + 0x28]
1414403b6: mov rdx, r15
1414403b9: mov r8, r14
1414403bc: mov r9b, 1
1414403bf: call 0x141700620
1414403c4: jmp 0x1414403e1
1414403c6: test rax, rax
1414403c9: js 0x141441b81
1414403cf: lea rcx, [rbp + 0x28]
1414403d3: mov rdx, r15
1414403d6: mov r8, r14
1414403d9: mov r9b, 1
1414403dc: call 0x141700430
1414403e1: mov rax, qword ptr [rbp + 0x28]
1414403e5: mov rcx, qword ptr [rbp + 0x30]
1414403e9: mov rdx, qword ptr [rbp + 0x38]
1414403ed: mov qword ptr [rbp + 0x15], rdx
1414403f1: movzx edx, byte ptr [rbp + 0x40]
1414403f5: mov byte ptr [rbp + 0x1d], dl
1414403f8: movzx edx, byte ptr [rbp + 0x41]
1414403fc: mov byte ptr [rbp + 0x1e], dl
1414403ff: movzx edx, byte ptr [rbp + 0x42]
141440403: mov byte ptr [rbp + 0x1f], dl
141440406: mov edx, dword ptr [rbp + 0x43]
141440409: mov dword ptr [rbp + 0x20], edx
14144040c: movzx edx, byte ptr [rbp + 0x47]
141440410: mov byte ptr [rbp + 0x24], dl
141440413: mov qword ptr [rbp + 5], rax
141440417: mov qword ptr [rbp + 0xd], rcx
14144041b: mov word ptr [rbp + 0x28], 0x100
141440421: mov byte ptr [rbp + 0x2a], 1
141440425: mov rax, qword ptr [rbp + 0x1d]
141440429: mov qword ptr [rbp + 0x48], rax
14144042d: mov rax, qword ptr [rbp + 0x10]
141440431: mov qword ptr [rbp + 0x3b], rax
141440435: mov eax, dword ptr [rbp + 0x18]
141440438: mov dword ptr [rbp + 0x43], eax
14144043b: movzx eax, byte ptr [rbp + 0x1c]
14144043f: mov byte ptr [rbp + 0x47], al
141440442: movzx eax, byte ptr [rbp + 0x1d]
141440446: mov byte ptr [rbp + 0x48], al
141440449: movzx eax, byte ptr [rbp + 0x1e]
14144044d: mov byte ptr [rbp + 0x49], al
141440450: movzx eax, byte ptr [rbp + 0x1f]
141440454: mov byte ptr [rbp + 0x4a], al
141440457: movups xmm0, xmmword ptr [rbp]
14144045b: movups xmmword ptr [rbp + 0x2b], xmm0
14144045f: mov r12, qword ptr [rsi + 0x10]
141440463: cmp r12, qword ptr [rsi]
141440466: jne 0x141440477
141440468: lea rdx, [rip + 0x181ec61]
14144046f: mov rcx, rsi
141440472: call 0x1405629f0
141440477: mov rax, qword ptr [rsi + 8]
14144047b: lea rcx, [r12 + r12*2]
14144047f: shl rcx, 4
141440483: movups xmm0, xmmword ptr [rbp + 0x28]
141440487: movups xmm1, xmmword ptr [rbp + 0x38]
14144048b: movups xmm2, xmmword ptr [rbp + 0x48]
14144048f: movups xmmword ptr [rax + rcx + 0x20], xmm2
141440494: movups xmmword ptr [rax + rcx + 0x10], xmm1
141440499: movups xmmword ptr [rax + rcx], xmm0
14144049d: inc r12
1414404a0: mov qword ptr [rsi + 0x10], r12
1414404a4: mov r12, qword ptr [r15 + 0x18]
1414404a8: mov rax, qword ptr [r15 + 0x20]
1414404ac: lock inc qword ptr [r12]
1414404b1: jle 0x141441b81
1414404b7: movss xmm0, dword ptr [r15 + 0x28]
1414404bd: mov ecx, dword ptr [r15 + 0x10]
1414404c1: movss xmm1, dword ptr [r15 + 0x14]
1414404c7: mov qword ptr [rbp + 0x28], 1
1414404cf: mov qword ptr [rbp + 0x30], 1
1414404d7: mov dword ptr [rbp + 0x38], ecx
1414404da: movss dword ptr [rbp + 0x3c], xmm1
1414404df: mov qword ptr [rbp + 0x40], r12
1414404e3: mov qword ptr [rbp + 0x48], rax
1414404e7: movss dword ptr [rbp + 0x50], xmm0
1414404ec: mov byte ptr [rbp + 0x54], 0
1414404f0: movzx eax, byte ptr [rip + 0x21f4712]
1414404f7: mov ecx, 0x30
1414404fc: mov edx, 8
141440501: call 0x140613c10
141440506: test rax, rax
141440509: je 0x141441b44
14144050f: movups xmm0, xmmword ptr [rbp + 0x28]
141440513: movups xmm1, xmmword ptr [rbp + 0x38]
141440517: movups xmm2, xmmword ptr [rbp + 0x48]
14144051b: movups xmmword ptr [rax + 0x20], xmm2
14144051f: movups xmmword ptr [rax + 0x10], xmm1
141440523: movups xmmword ptr [rax], xmm0
141440526: mov qword ptr [rbp - 8], rax
14144052a: mov ecx, 1
14144052f: lock xadd qword ptr [rax], rcx
141440534: test bl, bl
141440536: je 0x141440569
141440538: test dil, dil
14144053b: je 0x141440d1c
141440541: test rcx, rcx
141440544: js 0x141441b81
14144054a: mov qword ptr [rbp + 0x30], rax
14144054e: mov qword ptr [rbp + 0x28], r14
141440552: mov byte ptr [rbp + 0x40], 1
141440556: mov qword ptr [rbp + 0x38], 0
14144055e: mov word ptr [rbp + 0x41], 0
141440564: jmp 0x141440d54
141440569: test dil, dil
14144056c: je 0x141440d39
141440572: test rcx, rcx
141440575: js 0x141441b81
14144057b: mov qword ptr [rbp + 0x30], rax
14144057f: mov qword ptr [rbp + 0x28], r14
141440583: mov qword ptr [rbp + 0x38], 0
14144058b: mov dword ptr [rbp + 0x3f], 0
141440592: jmp 0x141440d54
141440597: test rax, rax
14144059a: js 0x141441b81
1414405a0: lea rcx, [rbp + 0x28]
1414405a4: mov rdx, r15
1414405a7: mov r8, r14
1414405aa: mov r9b, 1
1414405ad: call 0x141700620
1414405b2: jmp 0x1414405cf
1414405b4: test rax, rax
1414405b7: js 0x141441b81
1414405bd: lea rcx, [rbp + 0x28]
1414405c1: mov rdx, r15
1414405c4: mov r8, r14
1414405c7: mov r9b, 1
1414405ca: call 0x141700430
1414405cf: mov rax, qword ptr [rbp + 0x28]
1414405d3: mov rcx, qword ptr [rbp + 0x30]
1414405d7: mov rdx, qword ptr [rbp + 0x38]
1414405db: mov qword ptr [rbp + 0x15], rdx
1414405df: movzx edx, byte ptr [rbp + 0x40]
1414405e3: mov byte ptr [rbp + 0x1d], dl
1414405e6: movzx edx, byte ptr [rbp + 0x41]
1414405ea: mov byte ptr [rbp + 0x1e], dl
1414405ed: movzx edx, byte ptr [rbp + 0x42]
1414405f1: mov byte ptr [rbp + 0x1f], dl
1414405f4: mov edx, dword ptr [rbp + 0x43]
1414405f7: mov dword ptr [rbp + 0x20], edx
1414405fa: movzx edx, byte ptr [rbp + 0x47]
1414405fe: mov byte ptr [rbp + 0x24], dl
141440601: mov qword ptr [rbp + 5], rax
141440605: mov qword ptr [rbp + 0xd], rcx
141440609: mov word ptr [rbp + 0x28], 0x100
14144060f: mov byte ptr [rbp + 0x2a], 2
141440613: mov rax, qword ptr [rbp + 0x1d]
141440617: mov qword ptr [rbp + 0x48], rax
14144061b: mov rax, qword ptr [rbp + 0x10]
14144061f: mov qword ptr [rbp + 0x3b], rax
141440623: mov eax, dword ptr [rbp + 0x18]
141440626: mov dword ptr [rbp + 0x43], eax
141440629: movzx eax, byte ptr [rbp + 0x1c]
14144062d: mov byte ptr [rbp + 0x47], al
141440630: movzx eax, byte ptr [rbp + 0x1d]
141440634: mov byte ptr [rbp + 0x48], al
141440637: movzx eax, byte ptr [rbp + 0x1e]
14144063b: mov byte ptr [rbp + 0x49], al
14144063e: movzx eax, byte ptr [rbp + 0x1f]
141440642: mov byte ptr [rbp + 0x4a], al
141440645: movups xmm0, xmmword ptr [rbp]
141440649: movups xmmword ptr [rbp + 0x2b], xmm0
14144064d: mov r12, qword ptr [rsi + 0x10]
141440651: cmp r12, qword ptr [rsi]
141440654: jne 0x141440665
141440656: lea rdx, [rip + 0x181ea73]
14144065d: mov rcx, rsi
141440660: call 0x1405629f0
141440665: mov rax, qword ptr [rsi + 8]
141440669: lea rcx, [r12 + r12*2]
14144066d: shl rcx, 4
141440671: movups xmm0, xmmword ptr [rbp + 0x28]
141440675: movups xmm1, xmmword ptr [rbp + 0x38]
141440679: movups xmm2, xmmword ptr [rbp + 0x48]
14144067d: movups xmmword ptr [rax + rcx + 0x20], xmm2
141440682: movups xmmword ptr [rax + rcx + 0x10], xmm1
141440687: movups xmmword ptr [rax + rcx], xmm0
14144068b: inc r12
14144068e: mov qword ptr [rsi + 0x10], r12
141440692: mov r12, qword ptr [r15 + 0x18]
141440696: mov rax, qword ptr [r15 + 0x20]
14144069a: lock inc qword ptr [r12]
14144069f: jle 0x141441b81
1414406a5: movss xmm0, dword ptr [r15 + 0x28]
1414406ab: mov ecx, dword ptr [r15 + 0x10]
1414406af: movss xmm1, dword ptr [r15 + 0x14]
1414406b5: mov qword ptr [rbp + 0x28], 1
1414406bd: mov qword ptr [rbp + 0x30], 1
1414406c5: mov dword ptr [rbp + 0x38], ecx
1414406c8: movss dword ptr [rbp + 0x3c], xmm1
1414406cd: mov qword ptr [rbp + 0x40], r12
1414406d1: mov qword ptr [rbp + 0x48], rax
1414406d5: movss dword ptr [rbp + 0x50], xmm0
1414406da: mov byte ptr [rbp + 0x54], 0
1414406de: movzx eax, byte ptr [rip + 0x21f4524]
1414406e5: mov ecx, 0x30
1414406ea: mov edx, 8
1414406ef: call 0x140613c10
1414406f4: test rax, rax
1414406f7: je 0x141441b59
1414406fd: movups xmm0, xmmword ptr [rbp + 0x28]
141440701: movups xmm1, xmmword ptr [rbp + 0x38]
141440705: movups xmm2, xmmword ptr [rbp + 0x48]
141440709: movups xmmword ptr [rax + 0x20], xmm2
14144070d: movups xmmword ptr [rax + 0x10], xmm1
141440711: movups xmmword ptr [rax], xmm0
141440714: mov qword ptr [rbp - 8], rax
141440718: mov ecx, 1
14144071d: lock xadd qword ptr [rax], rcx
141440722: test bl, bl
141440724: je 0x141440757
141440726: test dil, dil
141440729: je 0x141440e94
14144072f: test rcx, rcx
141440732: js 0x141441b81
141440738: mov qword ptr [rbp + 0x30], rax
14144073c: mov qword ptr [rbp + 0x28], r14
141440740: mov byte ptr [rbp + 0x40], 1
141440744: mov qword ptr [rbp + 0x38], 0
14144074c: mov word ptr [rbp + 0x41], 0
141440752: jmp 0x141440ecc
141440757: test dil, dil
14144075a: je 0x141440eb1
141440760: test rcx, rcx
141440763: js 0x141441b81
141440769: mov qword ptr [rbp + 0x30], rax
14144076d: mov qword ptr [rbp + 0x28], r14
141440771: mov qword ptr [rbp + 0x38], 0
141440779: mov dword ptr [rbp + 0x3f], 0
141440780: jmp 0x141440ecc
141440785: mov rdx, qword ptr [rcx + 8]
141440789: lock inc qword ptr [rdx]
14144078d: jle 0x141441b81
141440793: lea rcx, [rbp + 0x30]
141440797: call 0x140995040
14144079c: jmp 0x1414407d2
14144079e: test rax, rax
1414407a1: js 0x141441b81
1414407a7: lea rcx, [rbp + 0x28]
1414407ab: mov rdx, r15
1414407ae: mov r8, r14
1414407b1: mov r9b, 1
1414407b4: call 0x141700620
1414407b9: jmp 0x141440802
1414407bb: mov rdx, qword ptr [rcx + 8]
1414407bf: lock inc qword ptr [rdx]
1414407c3: jle 0x141441b81
1414407c9: lea rcx, [rbp + 0x30]
1414407cd: call 0x140994f30
1414407d2: mov byte ptr [rbp + 0x28], 3
1414407d6: lea rdx, [rbp + 0x28]
1414407da: mov rcx, rsi
1414407dd: call 0x14143d220
1414407e2: jmp 0x141440b8e
1414407e7: test rax, rax
1414407ea: js 0x141441b81
1414407f0: lea rcx, [rbp + 0x28]
1414407f4: mov rdx, r15
1414407f7: mov r8, r14
1414407fa: mov r9b, 1
1414407fd: call 0x141700430
141440802: mov rax, qword ptr [rbp + 0x28]
141440806: mov rcx, qword ptr [rbp + 0x30]
14144080a: mov rdx, qword ptr [rbp + 0x38]
14144080e: mov qword ptr [rbp + 0x15], rdx
141440812: movzx edx, byte ptr [rbp + 0x40]
141440816: mov byte ptr [rbp + 0x1d], dl
141440819: movzx edx, byte ptr [rbp + 0x41]
14144081d: mov byte ptr [rbp + 0x1e], dl
141440820: movzx edx, byte ptr [rbp + 0x42]
141440824: mov byte ptr [rbp + 0x1f], dl
141440827: mov edx, dword ptr [rbp + 0x43]
14144082a: mov dword ptr [rbp + 0x20], edx
14144082d: movzx edx, byte ptr [rbp + 0x47]
141440831: mov byte ptr [rbp + 0x24], dl
141440834: mov qword ptr [rbp + 5], rax
141440838: mov qword ptr [rbp + 0xd], rcx
14144083c: mov word ptr [rbp + 0x28], 0
141440842: mov rax, qword ptr [rbp + 0x1d]
141440846: mov qword ptr [rbp + 0x48], rax
14144084a: mov eax, dword ptr [rbp + 0x10]
14144084d: mov dword ptr [rbp + 0x3b], eax
141440850: movzx eax, byte ptr [rbp + 0x14]
141440854: mov byte ptr [rbp + 0x3f], al
141440857: mov rax, qword ptr [rbp + 0x15]
14144085b: mov qword ptr [rbp + 0x40], rax
14144085f: movzx eax, byte ptr [rbp + 0x1d]
141440863: mov byte ptr [rbp + 0x48], al
141440866: movzx eax, byte ptr [rbp + 0x1e]
14144086a: mov byte ptr [rbp + 0x49], al
14144086d: movzx eax, byte ptr [rbp + 0x1f]
141440871: mov byte ptr [rbp + 0x4a], al
141440874: movups xmm0, xmmword ptr [rbp]
141440878: movups xmmword ptr [rbp + 0x2b], xmm0
14144087c: mov r12, qword ptr [rsi + 0x10]
141440880: cmp r12, qword ptr [rsi]
141440883: jne 0x141440894
141440885: lea rdx, [rip + 0x181e844]
14144088c: mov rcx, rsi
14144088f: call 0x1405629f0
141440894: mov rax, qword ptr [rsi + 8]
141440898: lea rcx, [r12 + r12*2]
14144089c: shl rcx, 4
1414408a0: movups xmm0, xmmword ptr [rbp + 0x28]
1414408a4: movups xmm1, xmmword ptr [rbp + 0x38]
1414408a8: movups xmm2, xmmword ptr [rbp + 0x48]
1414408ac: movups xmmword ptr [rax + rcx + 0x20], xmm2
1414408b1: movups xmmword ptr [rax + rcx + 0x10], xmm1
1414408b6: movups xmmword ptr [rax + rcx], xmm0
1414408ba: inc r12
1414408bd: mov qword ptr [rsi + 0x10], r12
1414408c1: mov r12, qword ptr [r15 + 0x18]
1414408c5: mov rax, qword ptr [r15 + 0x20]
1414408c9: lock inc qword ptr [r12]
1414408ce: jle 0x141441b81
1414408d4: movss xmm0, dword ptr [r15 + 0x28]
1414408da: mov ecx, dword ptr [r15 + 0x10]
1414408de: movss xmm1, dword ptr [r15 + 0x14]
1414408e4: mov qword ptr [rbp + 0x28], 1
1414408ec: mov qword ptr [rbp + 0x30], 1
1414408f4: mov dword ptr [rbp + 0x38], ecx
1414408f7: movss dword ptr [rbp + 0x3c], xmm1
1414408fc: mov qword ptr [rbp + 0x40], r12
141440900: mov qword ptr [rbp + 0x48], rax
141440904: movss dword ptr [rbp + 0x50], xmm0
141440909: mov byte ptr [rbp + 0x54], 0
14144090d: movzx eax, byte ptr [rip + 0x21f42f5]
141440914: mov ecx, 0x30
141440919: mov edx, 8
14144091e: call 0x140613c10
141440923: test rax, rax
141440926: je 0x141441b6e
14144092c: movups xmm0, xmmword ptr [rbp + 0x28]
141440930: movups xmm1, xmmword ptr [rbp + 0x38]
141440934: movups xmm2, xmmword ptr [rbp + 0x48]
141440938: movups xmmword ptr [rax + 0x20], xmm2
14144093c: movups xmmword ptr [rax + 0x10], xmm1
141440940: movups xmmword ptr [rax], xmm0
141440943: mov qword ptr [rbp - 8], rax
141440947: mov ecx, 1
14144094c: lock xadd qword ptr [rax], rcx
141440951: test bl, bl
141440953: je 0x141440986
141440955: test dil, dil
141440958: je 0x141441010
14144095e: test rcx, rcx
141440961: js 0x141441b81
141440967: mov qword ptr [rbp + 0x30], rax
14144096b: mov qword ptr [rbp + 0x28], r14
14144096f: mov byte ptr [rbp + 0x40], 1
141440973: mov qword ptr [rbp + 0x38], 0
14144097b: mov word ptr [rbp + 0x41], 0
141440981: jmp 0x141441048
141440986: test dil, dil
141440989: je 0x14144102d
14144098f: test rcx, rcx
141440992: js 0x141441b81
141440998: mov qword ptr [rbp + 0x30], rax
14144099c: mov qword ptr [rbp + 0x28], r14
1414409a0: mov qword ptr [rbp + 0x38], 0
1414409a8: mov dword ptr [rbp + 0x3f], 0
1414409af: jmp 0x141441048
1414409b4: test rax, rax
1414409b7: js 0x141441b81
1414409bd: lea rcx, [rbp - 0x38]
1414409c1: mov r9b, 1
1414409c4: call 0x141700620
1414409c9: jmp 0x1414409e0
1414409cb: test rax, rax
1414409ce: js 0x141441b81
1414409d4: lea rcx, [rbp - 0x38]
1414409d8: mov r9b, 1
1414409db: call 0x141700430
1414409e0: mov rax, qword ptr [rbp - 0x38]
1414409e4: mov rcx, qword ptr [rbp - 0x30]
1414409e8: mov rdx, qword ptr [rbp - 0x28]
1414409ec: mov qword ptr [rbp + 0x17], rdx
1414409f0: movzx edx, byte ptr [rbp - 0x20]
1414409f4: mov byte ptr [rbp + 0x1f], dl
1414409f7: movzx edx, byte ptr [rbp - 0x1f]
1414409fb: mov byte ptr [rbp + 0x20], dl
1414409fe: movzx edx, byte ptr [rbp - 0x1e]
141440a02: mov byte ptr [rbp + 0x21], dl
141440a05: mov edx, dword ptr [rbp - 0x1d]
141440a08: mov dword ptr [rbp + 0x22], edx
141440a0b: movzx edx, byte ptr [rbp - 0x19]
141440a0f: mov byte ptr [rbp + 0x26], dl
141440a12: mov qword ptr [rbp + 7], rax
141440a16: mov qword ptr [rbp + 0xf], rcx
141440a1a: mov byte ptr [rbp + 0x28], 1
141440a1e: mov rax, qword ptr [rbp + 0x1f]
141440a22: mov qword ptr [rbp + 0x48], rax
141440a26: mov eax, dword ptr [rbp + 0x10]
141440a29: mov dword ptr [rbp + 0x39], eax
141440a2c: movzx eax, word ptr [rbp + 0x14]
141440a30: mov word ptr [rbp + 0x3d], ax
141440a34: movzx eax, byte ptr [rbp + 0x16]
141440a38: mov byte ptr [rbp + 0x3f], al
141440a3b: mov rax, qword ptr [rbp + 0x17]
141440a3f: mov qword ptr [rbp + 0x40], rax
141440a43: movzx eax, byte ptr [rbp + 0x1f]
141440a47: mov byte ptr [rbp + 0x48], al
141440a4a: mov eax, dword ptr [rbp]
141440a4d: mov dword ptr [rbp + 0x29], eax
141440a50: movzx eax, word ptr [rbp + 4]
141440a54: mov word ptr [rbp + 0x2d], ax
141440a58: movzx eax, byte ptr [rbp + 6]
141440a5c: mov byte ptr [rbp + 0x2f], al
141440a5f: mov rax, qword ptr [rbp + 7]
141440a63: mov qword ptr [rbp + 0x30], rax
141440a67: movzx eax, byte ptr [rbp + 0xf]
141440a6b: mov byte ptr [rbp + 0x38], al
141440a6e: mov rdi, qword ptr [rsi + 0x10]
141440a72: cmp rdi, qword ptr [rsi]
141440a75: jne 0x141440b61
141440a7b: lea rdx, [rip + 0x181e64e]
141440a82: mov rcx, rsi
141440a85: call 0x1405629f0
141440a8a: jmp 0x141440b61
141440a8f: test rax, rax
141440a92: js 0x141441b81
141440a98: lea rcx, [rbp - 0x58]
141440a9c: mov r9b, 1
141440a9f: call 0x141700620
141440aa4: jmp 0x141440abb
141440aa6: test rax, rax
141440aa9: js 0x141441b81
141440aaf: lea rcx, [rbp - 0x58]
141440ab3: mov r9b, 1
141440ab6: call 0x141700430
141440abb: mov rax, qword ptr [rbp - 0x58]
141440abf: mov rcx, qword ptr [rbp - 0x50]
141440ac3: mov rdx, qword ptr [rbp - 0x48]
141440ac7: mov qword ptr [rbp + 0x17], rdx
141440acb: movzx edx, byte ptr [rbp - 0x40]
141440acf: mov byte ptr [rbp + 0x1f], dl
141440ad2: movzx edx, byte ptr [rbp - 0x3f]
141440ad6: mov byte ptr [rbp + 0x20], dl
141440ad9: movzx edx, byte ptr [rbp - 0x3e]
141440add: mov byte ptr [rbp + 0x21], dl
141440ae0: mov edx, dword ptr [rbp - 0x3d]
141440ae3: mov dword ptr [rbp + 0x22], edx
141440ae6: movzx edx, byte ptr [rbp - 0x39]
141440aea: mov byte ptr [rbp + 0x26], dl
141440aed: mov qword ptr [rbp + 7], rax
141440af1: mov qword ptr [rbp + 0xf], rcx
141440af5: mov byte ptr [rbp + 0x28], 2
141440af9: mov rax, qword ptr [rbp + 0x1f]
141440afd: mov qword ptr [rbp + 0x48], rax
141440b01: mov eax, dword ptr [rbp + 0x10]
141440b04: mov dword ptr [rbp + 0x39], eax
141440b07: movzx eax, word ptr [rbp + 0x14]
141440b0b: mov word ptr [rbp + 0x3d], ax
141440b0f: movzx eax, byte ptr [rbp + 0x16]
141440b13: mov byte ptr [rbp + 0x3f], al
141440b16: mov rax, qword ptr [rbp + 0x17]
141440b1a: mov qword ptr [rbp + 0x40], rax
141440b1e: movzx eax, byte ptr [rbp + 0x1f]
141440b22: mov byte ptr [rbp + 0x48], al
141440b25: mov eax, dword ptr [rbp]
141440b28: mov dword ptr [rbp + 0x29], eax
141440b2b: movzx eax, word ptr [rbp + 4]
141440b2f: mov word ptr [rbp + 0x2d], ax
141440b33: movzx eax, byte ptr [rbp + 6]
141440b37: mov byte ptr [rbp + 0x2f], al
141440b3a: mov rax, qword ptr [rbp + 7]
141440b3e: mov qword ptr [rbp + 0x30], rax
141440b42: movzx eax, byte ptr [rbp + 0xf]
141440b46: mov byte ptr [rbp + 0x38], al
141440b49: mov rdi, qword ptr [rsi + 0x10]
141440b4d: cmp rdi, qword ptr [rsi]
141440b50: jne 0x141440b61
141440b52: lea rdx, [rip + 0x181e577]
141440b59: mov rcx, rsi
141440b5c: call 0x1405629f0
141440b61: mov rax, qword ptr [rsi + 8]
141440b65: lea rcx, [rdi + rdi*2]
141440b69: shl rcx, 4
141440b6d: movups xmm0, xmmword ptr [rbp + 0x28]
141440b71: movups xmm1, xmmword ptr [rbp + 0x38]
141440b75: movups xmm2, xmmword ptr [rbp + 0x48]
141440b79: movups xmmword ptr [rax + rcx + 0x20], xmm2
141440b7e: movups xmmword ptr [rax + rcx + 0x10], xmm1
141440b83: movups xmmword ptr [rax + rcx], xmm0
141440b87: inc rdi
141440b8a: mov qword ptr [rsi + 0x10], rdi
141440b8e: add rsp, 0xe0
141440b95: pop rbx
141440b96: pop rdi
141440b97: pop rsi
141440b98: pop r12
141440b9a: pop r14
141440b9c: pop r15
141440b9e: pop rbp
141440b9f: ret 
141440ba0: test rcx, rcx
141440ba3: js 0x141441b81
141440ba9: lea rcx, [rbp + 0x28]
141440bad: mov rdx, rax
141440bb0: mov r8, r14
141440bb3: xor r9d, r9d
141440bb6: call 0x141700620
141440bbb: jmp 0x141440bd8
141440bbd: test rcx, rcx
141440bc0: js 0x141441b81
141440bc6: lea rcx, [rbp + 0x28]
141440bca: mov rdx, rax
141440bcd: mov r8, r14
141440bd0: xor r9d, r9d
141440bd3: call 0x141700430
141440bd8: mov rax, qword ptr [rbp + 0x28]
141440bdc: mov rcx, qword ptr [rbp + 0x30]
141440be0: mov rdx, qword ptr [rbp + 0x38]
141440be4: mov qword ptr [rbp + 0x15], rdx
141440be8: movzx edx, byte ptr [rbp + 0x40]
141440bec: mov byte ptr [rbp + 0x1d], dl
141440bef: movzx edx, word ptr [rbp + 0x41]
141440bf3: mov word ptr [rbp + 0x1e], dx
141440bf7: mov edx, dword ptr [rbp + 0x43]
141440bfa: mov dword ptr [rbp + 0x20], edx
141440bfd: movzx edx, byte ptr [rbp + 0x47]
141440c01: mov byte ptr [rbp + 0x24], dl
141440c04: mov qword ptr [rbp + 5], rax
141440c08: mov qword ptr [rbp + 0xd], rcx
141440c0c: mov word ptr [rbp + 0x28], 0x100
141440c12: mov byte ptr [rbp + 0x2a], 1
141440c16: mov rax, qword ptr [rbp + 0x1d]
141440c1a: mov qword ptr [rbp + 0x48], rax
141440c1e: mov eax, dword ptr [rbp + 0x10]
141440c21: mov dword ptr [rbp + 0x3b], eax
141440c24: movzx eax, byte ptr [rbp + 0x14]
141440c28: mov byte ptr [rbp + 0x3f], al
141440c2b: mov rax, qword ptr [rbp + 0x15]
141440c2f: mov qword ptr [rbp + 0x40], rax
141440c33: movzx eax, byte ptr [rbp + 0x1d]
141440c37: mov byte ptr [rbp + 0x48], al
141440c3a: movzx eax, word ptr [rbp + 0x1e]
141440c3e: mov word ptr [rbp + 0x49], ax
141440c42: mov eax, dword ptr [rbp]
141440c45: mov dword ptr [rbp + 0x2b], eax
141440c48: movzx eax, byte ptr [rbp + 4]
141440c4c: mov byte ptr [rbp + 0x2f], al
141440c4f: mov rax, qword ptr [rbp + 5]
141440c53: mov qword ptr [rbp + 0x30], rax
141440c57: movzx eax, word ptr [rbp + 0xd]
141440c5b: mov word ptr [rbp + 0x38], ax
141440c5f: movzx eax, byte ptr [rbp + 0xf]
141440c63: mov byte ptr [rbp + 0x3a], al
141440c66: mov r15, qword ptr [rsi + 0x10]
141440c6a: cmp r15, qword ptr [rsi]
141440c6d: jne 0x141440c7e
141440c6f: lea rdx, [rip + 0x181e45a]
141440c76: mov rcx, rsi
141440c79: call 0x1405629f0
141440c7e: mov rax, qword ptr [rsi + 8]
141440c82: lea rcx, [r15 + r15*2]
141440c86: shl rcx, 4
141440c8a: movups xmm0, xmmword ptr [rbp + 0x28]
141440c8e: movups xmm1, xmmword ptr [rbp + 0x38]
141440c92: movups xmm2, xmmword ptr [rbp + 0x48]
141440c96: movups xmmword ptr [rax + rcx + 0x20], xmm2
141440c9b: movups xmmword ptr [rax + rcx + 0x10], xmm1
141440ca0: movups xmmword ptr [rax + rcx], xmm0
141440ca4: inc r15
141440ca7: mov qword ptr [rsi + 0x10], r15
141440cab: mov rdx, qword ptr [rbp - 8]
141440caf: mov eax, 1
141440cb4: lock xadd qword ptr [rdx], rax
141440cb9: test bl, bl
141440cbb: je 0x141440cee
141440cbd: test dil, dil
141440cc0: je 0x14144118c
141440cc6: test rax, rax
141440cc9: js 0x141441b81
141440ccf: mov qword ptr [rbp + 0x30], rdx
141440cd3: mov qword ptr [rbp + 0x28], r14
141440cd7: mov byte ptr [rbp + 0x40], 1
141440cdb: mov qword ptr [rbp + 0x38], 0
141440ce3: mov word ptr [rbp + 0x41], 0
141440ce9: jmp 0x1414411be
141440cee: test dil, dil
141440cf1: je 0x1414411a6
141440cf7: test rax, rax
141440cfa: js 0x141441b81
141440d00: mov qword ptr [rbp + 0x30], rdx
141440d04: mov qword ptr [rbp + 0x28], r14
141440d08: mov qword ptr [rbp + 0x38], 0
141440d10: mov dword ptr [rbp + 0x3f], 0
141440d17: jmp 0x1414411be
141440d1c: test rcx, rcx
141440d1f: js 0x141441b81
141440d25: lea rcx, [rbp + 0x28]
141440d29: mov rdx, rax
141440d2c: mov r8, r14
141440d2f: xor r9d, r9d
141440d32: call 0x141700620
141440d37: jmp 0x141440d54
141440d39: test rcx, rcx
141440d3c: js 0x141441b81
141440d42: lea rcx, [rbp + 0x28]
141440d46: mov rdx, rax
141440d49: mov r8, r14
141440d4c: xor r9d, r9d
141440d4f: call 0x141700430
141440d54: mov rax, qword ptr [rbp + 0x28]
141440d58: mov rcx, qword ptr [rbp + 0x30]
141440d5c: mov rdx, qword ptr [rbp + 0x38]
141440d60: mov qword ptr [rbp + 0x15], rdx
141440d64: movzx edx, byte ptr [rbp + 0x40]
141440d68: mov byte ptr [rbp + 0x1d], dl
141440d6b: movzx edx, word ptr [rbp + 0x41]
141440d6f: mov word ptr [rbp + 0x1e], dx
141440d73: mov edx, dword ptr [rbp + 0x43]
141440d76: mov dword ptr [rbp + 0x20], edx
141440d79: movzx edx, byte ptr [rbp + 0x47]
141440d7d: mov byte ptr [rbp + 0x24], dl
141440d80: mov qword ptr [rbp + 5], rax
141440d84: mov qword ptr [rbp + 0xd], rcx
141440d88: mov word ptr [rbp + 0x28], 0
141440d8e: mov rax, qword ptr [rbp + 0x1d]
141440d92: mov qword ptr [rbp + 0x48], rax
141440d96: mov eax, dword ptr [rbp + 0x10]
141440d99: mov dword ptr [rbp + 0x3b], eax
141440d9c: movzx eax, byte ptr [rbp + 0x14]
141440da0: mov byte ptr [rbp + 0x3f], al
141440da3: mov rax, qword ptr [rbp + 0x15]
141440da7: mov qword ptr [rbp + 0x40], rax
141440dab: movzx eax, byte ptr [rbp + 0x1d]
141440daf: mov byte ptr [rbp + 0x48], al
141440db2: movzx eax, word ptr [rbp + 0x1e]
141440db6: mov word ptr [rbp + 0x49], ax
141440dba: mov eax, dword ptr [rbp]
141440dbd: mov dword ptr [rbp + 0x2b], eax
141440dc0: movzx eax, byte ptr [rbp + 4]
141440dc4: mov byte ptr [rbp + 0x2f], al
141440dc7: mov rax, qword ptr [rbp + 5]
141440dcb: mov qword ptr [rbp + 0x30], rax
141440dcf: movzx eax, word ptr [rbp + 0xd]
141440dd3: mov word ptr [rbp + 0x38], ax
141440dd7: movzx eax, byte ptr [rbp + 0xf]
141440ddb: mov byte ptr [rbp + 0x3a], al
141440dde: mov r15, qword ptr [rsi + 0x10]
141440de2: cmp r15, qword ptr [rsi]
141440de5: jne 0x141440df6
141440de7: lea rdx, [rip + 0x181e2e2]
141440dee: mov rcx, rsi
141440df1: call 0x1405629f0
141440df6: mov rax, qword ptr [rsi + 8]
141440dfa: lea rcx, [r15 + r15*2]
141440dfe: shl rcx, 4
141440e02: movups xmm0, xmmword ptr [rbp + 0x28]
141440e06: movups xmm1, xmmword ptr [rbp + 0x38]
141440e0a: movups xmm2, xmmword ptr [rbp + 0x48]
141440e0e: movups xmmword ptr [rax + rcx + 0x20], xmm2
141440e13: movups xmmword ptr [rax + rcx + 0x10], xmm1
141440e18: movups xmmword ptr [rax + rcx], xmm0
141440e1c: inc r15
141440e1f: mov qword ptr [rsi + 0x10], r15
141440e23: mov rdx, qword ptr [rbp - 8]
141440e27: mov eax, 1
141440e2c: lock xadd qword ptr [rdx], rax
141440e31: test bl, bl
141440e33: je 0x141440e66
141440e35: test dil, dil
141440e38: je 0x141441302
141440e3e: test rax, rax
141440e41: js 0x141441b81
141440e47: mov qword ptr [rbp + 0x30], rdx
141440e4b: mov qword ptr [rbp + 0x28], r14
141440e4f: mov byte ptr [rbp + 0x40], 1
141440e53: mov qword ptr [rbp + 0x38], 0
141440e5b: mov word ptr [rbp + 0x41], 0
141440e61: jmp 0x141441334
141440e66: test dil, dil
141440e69: je 0x14144131c
141440e6f: test rax, rax
141440e72: js 0x141441b81
141440e78: mov qword ptr [rbp + 0x30], rdx
141440e7c: mov qword ptr [rbp + 0x28], r14
141440e80: mov qword ptr [rbp + 0x38], 0
141440e88: mov dword ptr [rbp + 0x3f], 0
141440e8f: jmp 0x141441334
141440e94: test rcx, rcx
141440e97: js 0x141441b81
141440e9d: lea rcx, [rbp + 0x28]
141440ea1: mov rdx, rax
141440ea4: mov r8, r14
141440ea7: xor r9d, r9d
141440eaa: call 0x141700620
141440eaf: jmp 0x141440ecc
141440eb1: test rcx, rcx
141440eb4: js 0x141441b81
141440eba: lea rcx, [rbp + 0x28]
141440ebe: mov rdx, rax
141440ec1: mov r8, r14
141440ec4: xor r9d, r9d
141440ec7: call 0x141700430
141440ecc: mov rax, qword ptr [rbp + 0x28]
141440ed0: mov rcx, qword ptr [rbp + 0x30]
141440ed4: mov rdx, qword ptr [rbp + 0x38]
141440ed8: mov qword ptr [rbp + 0x15], rdx
141440edc: movzx edx, byte ptr [rbp + 0x40]
141440ee0: mov byte ptr [rbp + 0x1d], dl
141440ee3: movzx edx, word ptr [rbp + 0x41]
141440ee7: mov word ptr [rbp + 0x1e], dx
141440eeb: mov edx, dword ptr [rbp + 0x43]
141440eee: mov dword ptr [rbp + 0x20], edx
141440ef1: movzx edx, byte ptr [rbp + 0x47]
141440ef5: mov byte ptr [rbp + 0x24], dl
141440ef8: mov qword ptr [rbp + 5], rax
141440efc: mov qword ptr [rbp + 0xd], rcx
141440f00: mov word ptr [rbp + 0x28], 0x100
141440f06: mov byte ptr [rbp + 0x2a], 1
141440f0a: mov rax, qword ptr [rbp + 0x1d]
141440f0e: mov qword ptr [rbp + 0x48], rax
141440f12: mov eax, dword ptr [rbp + 0x10]
141440f15: mov dword ptr [rbp + 0x3b], eax
141440f18: movzx eax, byte ptr [rbp + 0x14]
141440f1c: mov byte ptr [rbp + 0x3f], al
141440f1f: mov rax, qword ptr [rbp + 0x15]
141440f23: mov qword ptr [rbp + 0x40], rax
141440f27: movzx eax, byte ptr [rbp + 0x1d]
141440f2b: mov byte ptr [rbp + 0x48], al
141440f2e: movzx eax, word ptr [rbp + 0x1e]
141440f32: mov word ptr [rbp + 0x49], ax
141440f36: mov eax, dword ptr [rbp]
141440f39: mov dword ptr [rbp + 0x2b], eax
141440f3c: movzx eax, byte ptr [rbp + 4]
141440f40: mov byte ptr [rbp + 0x2f], al
141440f43: mov rax, qword ptr [rbp + 5]
141440f47: mov qword ptr [rbp + 0x30], rax
141440f4b: movzx eax, word ptr [rbp + 0xd]
141440f4f: mov word ptr [rbp + 0x38], ax
141440f53: movzx eax, byte ptr [rbp + 0xf]
141440f57: mov byte ptr [rbp + 0x3a], al
141440f5a: mov r15, qword ptr [rsi + 0x10]
141440f5e: cmp r15, qword ptr [rsi]
141440f61: jne 0x141440f72
141440f63: lea rdx, [rip + 0x181e166]
141440f6a: mov rcx, rsi
141440f6d: call 0x1405629f0
141440f72: mov rax, qword ptr [rsi + 8]
141440f76: lea rcx, [r15 + r15*2]
141440f7a: shl rcx, 4
141440f7e: movups xmm0, xmmword ptr [rbp + 0x28]
141440f82: movups xmm1, xmmword ptr [rbp + 0x38]
141440f86: movups xmm2, xmmword ptr [rbp + 0x48]
141440f8a: movups xmmword ptr [rax + rcx + 0x20], xmm2
141440f8f: movups xmmword ptr [rax + rcx + 0x10], xmm1
141440f94: movups xmmword ptr [rax + rcx], xmm0
141440f98: inc r15
141440f9b: mov qword ptr [rsi + 0x10], r15
141440f9f: mov rdx, qword ptr [rbp - 8]
141440fa3: mov eax, 1
141440fa8: lock xadd qword ptr [rdx], rax
141440fad: test bl, bl
141440faf: je 0x141440fe2
141440fb1: test dil, dil
141440fb4: je 0x141441478
141440fba: test rax, rax
141440fbd: js 0x141441b81
141440fc3: mov qword ptr [rbp + 0x30], rdx
141440fc7: mov qword ptr [rbp + 0x28], r14
141440fcb: mov byte ptr [rbp + 0x40], 1
141440fcf: mov qword ptr [rbp + 0x38], 0
141440fd7: mov word ptr [rbp + 0x41], 0
141440fdd: jmp 0x1414414aa
141440fe2: test dil, dil
141440fe5: je 0x141441492
141440feb: test rax, rax
141440fee: js 0x141441b81
141440ff4: mov qword ptr [rbp + 0x30], rdx
141440ff8: mov qword ptr [rbp + 0x28], r14
141440ffc: mov qword ptr [rbp + 0x38], 0
141441004: mov dword ptr [rbp + 0x3f], 0
14144100b: jmp 0x1414414aa
141441010: test rcx, rcx
141441013: js 0x141441b81
141441019: lea rcx, [rbp + 0x28]
14144101d: mov rdx, rax
141441020: mov r8, r14
141441023: xor r9d, r9d
141441026: call 0x141700620
14144102b: jmp 0x141441048
14144102d: test rcx, rcx
141441030: js 0x141441b81
141441036: lea rcx, [rbp + 0x28]
14144103a: mov rdx, rax
14144103d: mov r8, r14
141441040: xor r9d, r9d
141441043: call 0x141700430
141441048: mov rax, qword ptr [rbp + 0x28]
14144104c: mov rcx, qword ptr [rbp + 0x30]
141441050: mov rdx, qword ptr [rbp + 0x38]
141441054: mov qword ptr [rbp + 0x15], rdx
141441058: movzx edx, byte ptr [rbp + 0x40]
14144105c: mov byte ptr [rbp + 0x1d], dl
14144105f: movzx edx, word ptr [rbp + 0x41]
141441063: mov word ptr [rbp + 0x1e], dx
141441067: mov edx, dword ptr [rbp + 0x43]
14144106a: mov dword ptr [rbp + 0x20], edx
14144106d: movzx edx, byte ptr [rbp + 0x47]
141441071: mov byte ptr [rbp + 0x24], dl
141441074: mov qword ptr [rbp + 5], rax
141441078: mov qword ptr [rbp + 0xd], rcx
14144107c: mov word ptr [rbp + 0x28], 0x100
141441082: mov byte ptr [rbp + 0x2a], 1
141441086: mov rax, qword ptr [rbp + 0x1d]
14144108a: mov qword ptr [rbp + 0x48], rax
14144108e: mov eax, dword ptr [rbp + 0x10]
141441091: mov dword ptr [rbp + 0x3b], eax
141441094: movzx eax, byte ptr [rbp + 0x14]
141441098: mov byte ptr [rbp + 0x3f], al
14144109b: mov rax, qword ptr [rbp + 0x15]
14144109f: mov qword ptr [rbp + 0x40], rax
1414410a3: movzx eax, byte ptr [rbp + 0x1d]
1414410a7: mov byte ptr [rbp + 0x48], al
1414410aa: movzx eax, word ptr [rbp + 0x1e]
1414410ae: mov word ptr [rbp + 0x49], ax
1414410b2: mov eax, dword ptr [rbp]
1414410b5: mov dword ptr [rbp + 0x2b], eax
1414410b8: movzx eax, byte ptr [rbp + 4]
1414410bc: mov byte ptr [rbp + 0x2f], al
1414410bf: mov rax, qword ptr [rbp + 5]
1414410c3: mov qword ptr [rbp + 0x30], rax
1414410c7: movzx eax, word ptr [rbp + 0xd]
1414410cb: mov word ptr [rbp + 0x38], ax
1414410cf: movzx eax, byte ptr [rbp + 0xf]
1414410d3: mov byte ptr [rbp + 0x3a], al
1414410d6: mov r15, qword ptr [rsi + 0x10]
1414410da: cmp r15, qword ptr [rsi]
1414410dd: jne 0x1414410ee
1414410df: lea rdx, [rip + 0x181dfea]
1414410e6: mov rcx, rsi
1414410e9: call 0x1405629f0
1414410ee: mov rax, qword ptr [rsi + 8]
1414410f2: lea rcx, [r15 + r15*2]
1414410f6: shl rcx, 4
1414410fa: movups xmm0, xmmword ptr [rbp + 0x28]
1414410fe: movups xmm1, xmmword ptr [rbp + 0x38]
141441102: movups xmm2, xmmword ptr [rbp + 0x48]
141441106: movups xmmword ptr [rax + rcx + 0x20], xmm2
14144110b: movups xmmword ptr [rax + rcx + 0x10], xmm1
141441110: movups xmmword ptr [rax + rcx], xmm0
141441114: inc r15
141441117: mov qword ptr [rsi + 0x10], r15
14144111b: mov rdx, qword ptr [rbp - 8]
14144111f: mov eax, 1
141441124: lock xadd qword ptr [rdx], rax
141441129: test bl, bl
14144112b: je 0x14144115e
14144112d: test dil, dil
141441130: je 0x1414415ea
141441136: test rax, rax
141441139: js 0x141441b81
14144113f: mov qword ptr [rbp + 0x30], rdx
141441143: mov qword ptr [rbp + 0x28], r14
141441147: mov byte ptr [rbp + 0x40], 1
14144114b: mov qword ptr [rbp + 0x38], 0
141441153: mov word ptr [rbp + 0x41], 0
141441159: jmp 0x14144161c
14144115e: test dil, dil
141441161: je 0x141441604
141441167: test rax, rax
14144116a: js 0x141441b81
141441170: mov qword ptr [rbp + 0x30], rdx
141441174: mov qword ptr [rbp + 0x28], r14
141441178: mov qword ptr [rbp + 0x38], 0
141441180: mov dword ptr [rbp + 0x3f], 0
141441187: jmp 0x14144161c
14144118c: test rax, rax
14144118f: js 0x141441b81
141441195: lea rcx, [rbp + 0x28]
141441199: mov r8, r14
14144119c: xor r9d, r9d
14144119f: call 0x141700620
1414411a4: jmp 0x1414411be
1414411a6: test rax, rax
1414411a9: js 0x141441b81
1414411af: lea rcx, [rbp + 0x28]
1414411b3: mov r8, r14
1414411b6: xor r9d, r9d
1414411b9: call 0x141700430
1414411be: mov rax, qword ptr [rbp + 0x28]
1414411c2: mov rcx, qword ptr [rbp + 0x30]
1414411c6: mov rdx, qword ptr [rbp + 0x38]
1414411ca: mov qword ptr [rbp + 0x15], rdx
1414411ce: movzx edx, byte ptr [rbp + 0x40]
1414411d2: mov byte ptr [rbp + 0x1d], dl
1414411d5: movzx edx, word ptr [rbp + 0x41]
1414411d9: mov word ptr [rbp + 0x1e], dx
1414411dd: mov edx, dword ptr [rbp + 0x43]
1414411e0: mov dword ptr [rbp + 0x20], edx
1414411e3: movzx edx, byte ptr [rbp + 0x47]
1414411e7: mov byte ptr [rbp + 0x24], dl
1414411ea: mov qword ptr [rbp + 5], rax
1414411ee: mov qword ptr [rbp + 0xd], rcx
1414411f2: mov word ptr [rbp + 0x28], 0x100
1414411f8: mov byte ptr [rbp + 0x2a], 2
1414411fc: mov rax, qword ptr [rbp + 0x1d]
141441200: mov qword ptr [rbp + 0x48], rax
141441204: mov eax, dword ptr [rbp + 0x10]
141441207: mov dword ptr [rbp + 0x3b], eax
14144120a: movzx eax, byte ptr [rbp + 0x14]
14144120e: mov byte ptr [rbp + 0x3f], al
141441211: mov rax, qword ptr [rbp + 0x15]
141441215: mov qword ptr [rbp + 0x40], rax
141441219: movzx eax, byte ptr [rbp + 0x1d]
14144121d: mov byte ptr [rbp + 0x48], al
141441220: movzx eax, word ptr [rbp + 0x1e]
141441224: mov word ptr [rbp + 0x49], ax
141441228: mov eax, dword ptr [rbp]
14144122b: mov dword ptr [rbp + 0x2b], eax
14144122e: movzx eax, byte ptr [rbp + 4]
141441232: mov byte ptr [rbp + 0x2f], al
141441235: mov rax, qword ptr [rbp + 5]
141441239: mov qword ptr [rbp + 0x30], rax
14144123d: movzx eax, word ptr [rbp + 0xd]
141441241: mov word ptr [rbp + 0x38], ax
141441245: movzx eax, byte ptr [rbp + 0xf]
141441249: mov byte ptr [rbp + 0x3a], al
14144124c: mov r15, qword ptr [rsi + 0x10]
141441250: cmp r15, qword ptr [rsi]
141441253: jne 0x141441264
141441255: lea rdx, [rip + 0x181de74]
14144125c: mov rcx, rsi
14144125f: call 0x1405629f0
141441264: mov rax, qword ptr [rsi + 8]
141441268: lea rcx, [r15 + r15*2]
14144126c: shl rcx, 4
141441270: movups xmm0, xmmword ptr [rbp + 0x28]
141441274: movups xmm1, xmmword ptr [rbp + 0x38]
141441278: movups xmm2, xmmword ptr [rbp + 0x48]
14144127c: movups xmmword ptr [rax + rcx + 0x20], xmm2
141441281: movups xmmword ptr [rax + rcx + 0x10], xmm1
141441286: movups xmmword ptr [rax + rcx], xmm0
14144128a: inc r15
14144128d: mov qword ptr [rsi + 0x10], r15
141441291: mov rdx, qword ptr [rbp - 8]
141441295: mov eax, 1
14144129a: lock xadd qword ptr [rdx], rax
14144129f: test bl, bl
1414412a1: je 0x1414412d4
1414412a3: test dil, dil
1414412a6: je 0x141441760
1414412ac: test rax, rax
1414412af: js 0x141441b81
1414412b5: mov qword ptr [rbp + 0x30], rdx
1414412b9: mov qword ptr [rbp + 0x28], r14
1414412bd: mov byte ptr [rbp + 0x40], 1
1414412c1: mov qword ptr [rbp + 0x38], 0
1414412c9: mov word ptr [rbp + 0x41], 0
1414412cf: jmp 0x141441792
1414412d4: test dil, dil
1414412d7: je 0x14144177a
1414412dd: test rax, rax
1414412e0: js 0x141441b81
1414412e6: mov qword ptr [rbp + 0x30], rdx
1414412ea: mov qword ptr [rbp + 0x28], r14
1414412ee: mov qword ptr [rbp + 0x38], 0
1414412f6: mov dword ptr [rbp + 0x3f], 0
1414412fd: jmp 0x141441792
141441302: test rax, rax
141441305: js 0x141441b81
14144130b: lea rcx, [rbp + 0x28]
14144130f: mov r8, r14
141441312: xor r9d, r9d
141441315: call 0x141700620
14144131a: jmp 0x141441334
14144131c: test rax, rax
14144131f: js 0x141441b81
141441325: lea rcx, [rbp + 0x28]
141441329: mov r8, r14
14144132c: xor r9d, r9d
14144132f: call 0x141700430
141441334: mov rax, qword ptr [rbp + 0x28]
141441338: mov rcx, qword ptr [rbp + 0x30]
14144133c: mov rdx, qword ptr [rbp + 0x38]
141441340: mov qword ptr [rbp + 0x15], rdx
141441344: movzx edx, byte ptr [rbp + 0x40]
141441348: mov byte ptr [rbp + 0x1d], dl
14144134b: movzx edx, word ptr [rbp + 0x41]
14144134f: mov word ptr [rbp + 0x1e], dx
141441353: mov edx, dword ptr [rbp + 0x43]
141441356: mov dword ptr [rbp + 0x20], edx
141441359: movzx edx, byte ptr [rbp + 0x47]
14144135d: mov byte ptr [rbp + 0x24], dl
141441360: mov qword ptr [rbp + 5], rax
141441364: mov qword ptr [rbp + 0xd], rcx
141441368: mov word ptr [rbp + 0x28], 0x100
14144136e: mov byte ptr [rbp + 0x2a], 2
141441372: mov rax, qword ptr [rbp + 0x1d]
141441376: mov qword ptr [rbp + 0x48], rax
14144137a: mov eax, dword ptr [rbp + 0x10]
14144137d: mov dword ptr [rbp + 0x3b], eax
141441380: movzx eax, byte ptr [rbp + 0x14]
141441384: mov byte ptr [rbp + 0x3f], al
141441387: mov rax, qword ptr [rbp + 0x15]
14144138b: mov qword ptr [rbp + 0x40], rax
14144138f: movzx eax, byte ptr [rbp + 0x1d]
141441393: mov byte ptr [rbp + 0x48], al
141441396: movzx eax, word ptr [rbp + 0x1e]
14144139a: mov word ptr [rbp + 0x49], ax
14144139e: mov eax, dword ptr [rbp]
1414413a1: mov dword ptr [rbp + 0x2b], eax
1414413a4: movzx eax, byte ptr [rbp + 4]
1414413a8: mov byte ptr [rbp + 0x2f], al
1414413ab: mov rax, qword ptr [rbp + 5]
1414413af: mov qword ptr [rbp + 0x30], rax
1414413b3: movzx eax, word ptr [rbp + 0xd]
1414413b7: mov word ptr [rbp + 0x38], ax
1414413bb: movzx eax, byte ptr [rbp + 0xf]
1414413bf: mov byte ptr [rbp + 0x3a], al
1414413c2: mov r15, qword ptr [rsi + 0x10]
1414413c6: cmp r15, qword ptr [rsi]
1414413c9: jne 0x1414413da
1414413cb: lea rdx, [rip + 0x181dcfe]
1414413d2: mov rcx, rsi
1414413d5: call 0x1405629f0
1414413da: mov rax, qword ptr [rsi + 8]
1414413de: lea rcx, [r15 + r15*2]
1414413e2: shl rcx, 4
1414413e6: movups xmm0, xmmword ptr [rbp + 0x28]
1414413ea: movups xmm1, xmmword ptr [rbp + 0x38]
1414413ee: movups xmm2, xmmword ptr [rbp + 0x48]
1414413f2: movups xmmword ptr [rax + rcx + 0x20], xmm2
1414413f7: movups xmmword ptr [rax + rcx + 0x10], xmm1
1414413fc: movups xmmword ptr [rax + rcx], xmm0
141441400: inc r15
141441403: mov qword ptr [rsi + 0x10], r15
141441407: mov rdx, qword ptr [rbp - 8]
14144140b: mov eax, 1
141441410: lock xadd qword ptr [rdx], rax
141441415: test bl, bl
141441417: je 0x14144144a
141441419: test dil, dil
14144141c: je 0x14144183d
141441422: test rax, rax
141441425: js 0x141441b81
14144142b: mov qword ptr [rbp + 0x30], rdx
14144142f: mov qword ptr [rbp + 0x28], r14
141441433: mov byte ptr [rbp + 0x40], 1
141441437: mov qword ptr [rbp + 0x38], 0
14144143f: mov word ptr [rbp + 0x41], 0
141441445: jmp 0x14144186f
14144144a: test dil, dil
14144144d: je 0x141441857
141441453: test rax, rax
141441456: js 0x141441b81
14144145c: mov qword ptr [rbp + 0x30], rdx
141441460: mov qword ptr [rbp + 0x28], r14
141441464: mov qword ptr [rbp + 0x38], 0
14144146c: mov dword ptr [rbp + 0x3f], 0
141441473: jmp 0x14144186f
141441478: test rax, rax
14144147b: js 0x141441b81
141441481: lea rcx, [rbp + 0x28]
141441485: mov r8, r14
141441488: xor r9d, r9d
14144148b: call 0x141700620
141441490: jmp 0x1414414aa
141441492: test rax, rax
141441495: js 0x141441b81
14144149b: lea rcx, [rbp + 0x28]
14144149f: mov r8, r14
1414414a2: xor r9d, r9d
1414414a5: call 0x141700430
1414414aa: mov rax, qword ptr [rbp + 0x28]
1414414ae: mov rcx, qword ptr [rbp + 0x30]
1414414b2: mov rdx, qword ptr [rbp + 0x38]
1414414b6: mov qword ptr [rbp + 0x15], rdx
1414414ba: movzx edx, byte ptr [rbp + 0x40]
1414414be: mov byte ptr [rbp + 0x1d], dl
1414414c1: movzx edx, word ptr [rbp + 0x41]
1414414c5: mov word ptr [rbp + 0x1e], dx
1414414c9: mov edx, dword ptr [rbp + 0x43]
1414414cc: mov dword ptr [rbp + 0x20], edx
1414414cf: movzx edx, byte ptr [rbp + 0x47]
1414414d3: mov byte ptr [rbp + 0x24], dl
1414414d6: mov qword ptr [rbp + 5], rax
1414414da: mov qword ptr [rbp + 0xd], rcx
1414414de: mov word ptr [rbp + 0x28], 0
1414414e4: mov rax, qword ptr [rbp + 0x1d]
1414414e8: mov qword ptr [rbp + 0x48], rax
1414414ec: mov eax, dword ptr [rbp + 0x10]
1414414ef: mov dword ptr [rbp + 0x3b], eax
1414414f2: movzx eax, byte ptr [rbp + 0x14]
1414414f6: mov byte ptr [rbp + 0x3f], al
1414414f9: mov rax, qword ptr [rbp + 0x15]
1414414fd: mov qword ptr [rbp + 0x40], rax
141441501: movzx eax, byte ptr [rbp + 0x1d]
141441505: mov byte ptr [rbp + 0x48], al
141441508: movzx eax, word ptr [rbp + 0x1e]
14144150c: mov word ptr [rbp + 0x49], ax
141441510: mov eax, dword ptr [rbp]
141441513: mov dword ptr [rbp + 0x2b], eax
141441516: movzx eax, byte ptr [rbp + 4]
14144151a: mov byte ptr [rbp + 0x2f], al
14144151d: mov rax, qword ptr [rbp + 5]
141441521: mov qword ptr [rbp + 0x30], rax
141441525: movzx eax, word ptr [rbp + 0xd]
141441529: mov word ptr [rbp + 0x38], ax
14144152d: movzx eax, byte ptr [rbp + 0xf]
141441531: mov byte ptr [rbp + 0x3a], al
141441534: mov r15, qword ptr [rsi + 0x10]
141441538: cmp r15, qword ptr [rsi]
14144153b: jne 0x14144154c
14144153d: lea rdx, [rip + 0x181db8c]
141441544: mov rcx, rsi
141441547: call 0x1405629f0
14144154c: mov rax, qword ptr [rsi + 8]
141441550: lea rcx, [r15 + r15*2]
141441554: shl rcx, 4
141441558: movups xmm0, xmmword ptr [rbp + 0x28]
14144155c: movups xmm1, xmmword ptr [rbp + 0x38]
141441560: movups xmm2, xmmword ptr [rbp + 0x48]
141441564: movups xmmword ptr [rax + rcx + 0x20], xmm2
141441569: movups xmmword ptr [rax + rcx + 0x10], xmm1
14144156e: movups xmmword ptr [rax + rcx], xmm0
141441572: inc r15
141441575: mov qword ptr [rsi + 0x10], r15
141441579: mov rdx, qword ptr [rbp - 8]
14144157d: mov eax, 1
141441582: lock xadd qword ptr [rdx], rax
141441587: test bl, bl
141441589: je 0x1414415bc
14144158b: test dil, dil
14144158e: je 0x14144191e
141441594: test rax, rax
141441597: js 0x141441b81
14144159d: mov qword ptr [rbp + 0x30], rdx
1414415a1: mov qword ptr [rbp + 0x28], r14
1414415a5: mov byte ptr [rbp + 0x40], 1
1414415a9: mov qword ptr [rbp + 0x38], 0
1414415b1: mov word ptr [rbp + 0x41], 0
1414415b7: jmp 0x141441950
1414415bc: test dil, dil
1414415bf: je 0x141441938
1414415c5: test rax, rax
1414415c8: js 0x141441b81
1414415ce: mov qword ptr [rbp + 0x30], rdx
1414415d2: mov qword ptr [rbp + 0x28], r14
1414415d6: mov qword ptr [rbp + 0x38], 0
1414415de: mov dword ptr [rbp + 0x3f], 0
1414415e5: jmp 0x141441950
1414415ea: test rax, rax
1414415ed: js 0x141441b81
1414415f3: lea rcx, [rbp + 0x28]
1414415f7: mov r8, r14
1414415fa: xor r9d, r9d
1414415fd: call 0x141700620
141441602: jmp 0x14144161c
141441604: test rax, rax
141441607: js 0x141441b81
14144160d: lea rcx, [rbp + 0x28]
141441611: mov r8, r14
141441614: xor r9d, r9d
141441617: call 0x141700430
14144161c: mov rax, qword ptr [rbp + 0x28]
141441620: mov rcx, qword ptr [rbp + 0x30]
141441624: mov rdx, qword ptr [rbp + 0x38]
141441628: mov qword ptr [rbp + 0x15], rdx
14144162c: movzx edx, byte ptr [rbp + 0x40]
141441630: mov byte ptr [rbp + 0x1d], dl
141441633: movzx edx, word ptr [rbp + 0x41]
141441637: mov word ptr [rbp + 0x1e], dx
14144163b: mov edx, dword ptr [rbp + 0x43]
14144163e: mov dword ptr [rbp + 0x20], edx
141441641: movzx edx, byte ptr [rbp + 0x47]
141441645: mov byte ptr [rbp + 0x24], dl
141441648: mov qword ptr [rbp + 5], rax
14144164c: mov qword ptr [rbp + 0xd], rcx
141441650: mov word ptr [rbp + 0x28], 0x100
141441656: mov byte ptr [rbp + 0x2a], 2
14144165a: mov rax, qword ptr [rbp + 0x1d]
14144165e: mov qword ptr [rbp + 0x48], rax
141441662: mov eax, dword ptr [rbp + 0x10]
141441665: mov dword ptr [rbp + 0x3b], eax
141441668: movzx eax, byte ptr [rbp + 0x14]
14144166c: mov byte ptr [rbp + 0x3f], al
14144166f: mov rax, qword ptr [rbp + 0x15]
141441673: mov qword ptr [rbp + 0x40], rax
141441677: movzx eax, byte ptr [rbp + 0x1d]
14144167b: mov byte ptr [rbp + 0x48], al
14144167e: movzx eax, word ptr [rbp + 0x1e]
141441682: mov word ptr [rbp + 0x49], ax
141441686: mov eax, dword ptr [rbp]
141441689: mov dword ptr [rbp + 0x2b], eax
14144168c: movzx eax, byte ptr [rbp + 4]
141441690: mov byte ptr [rbp + 0x2f], al
141441693: mov rax, qword ptr [rbp + 5]
141441697: mov qword ptr [rbp + 0x30], rax
14144169b: movzx eax, word ptr [rbp + 0xd]
14144169f: mov word ptr [rbp + 0x38], ax
1414416a3: movzx eax, byte ptr [rbp + 0xf]
1414416a7: mov byte ptr [rbp + 0x3a], al
1414416aa: mov r15, qword ptr [rsi + 0x10]
1414416ae: cmp r15, qword ptr [rsi]
1414416b1: jne 0x1414416c2
1414416b3: lea rdx, [rip + 0x181da16]
1414416ba: mov rcx, rsi
1414416bd: call 0x1405629f0
1414416c2: mov rax, qword ptr [rsi + 8]
1414416c6: lea rcx, [r15 + r15*2]
1414416ca: shl rcx, 4
1414416ce: movups xmm0, xmmword ptr [rbp + 0x28]
1414416d2: movups xmm1, xmmword ptr [rbp + 0x38]
1414416d6: movups xmm2, xmmword ptr [rbp + 0x48]
1414416da: movups xmmword ptr [rax + rcx + 0x20], xmm2
1414416df: movups xmmword ptr [rax + rcx + 0x10], xmm1
1414416e4: movups xmmword ptr [rax + rcx], xmm0
1414416e8: inc r15
1414416eb: mov qword ptr [rsi + 0x10], r15
1414416ef: mov rdx, qword ptr [rbp - 8]
1414416f3: mov eax, 1
1414416f8: lock xadd qword ptr [rdx], rax
1414416fd: test bl, bl
1414416ff: je 0x141441732
141441701: test dil, dil
141441704: je 0x1414419ff
14144170a: test rax, rax
14144170d: js 0x141441b81
141441713: mov qword ptr [rbp + 0x30], rdx
141441717: mov qword ptr [rbp + 0x28], r14
14144171b: mov byte ptr [rbp + 0x40], 1
14144171f: mov qword ptr [rbp + 0x38], 0
141441727: mov word ptr [rbp + 0x41], 0
14144172d: jmp 0x141441a31
141441732: test dil, dil
141441735: je 0x141441a19
14144173b: test rax, rax
14144173e: js 0x141441b81
141441744: mov qword ptr [rbp + 0x30], rdx
141441748: mov qword ptr [rbp + 0x28], r14
14144174c: mov qword ptr [rbp + 0x38], 0
141441754: mov dword ptr [rbp + 0x3f], 0
14144175b: jmp 0x141441a31
141441760: test rax, rax
141441763: js 0x141441b81
141441769: lea rcx, [rbp + 0x28]
14144176d: mov r8, r14
141441770: xor r9d, r9d
141441773: call 0x141700620
141441778: jmp 0x141441792
14144177a: test rax, rax
14144177d: js 0x141441b81
141441783: lea rcx, [rbp + 0x28]
141441787: mov r8, r14
14144178a: xor r9d, r9d
14144178d: call 0x141700430
141441792: mov rax, qword ptr [rbp + 0x28]
141441796: mov rcx, qword ptr [rbp + 0x30]
14144179a: mov rdx, qword ptr [rbp + 0x38]
14144179e: mov qword ptr [rbp + 0x15], rdx
1414417a2: movzx edx, byte ptr [rbp + 0x40]
1414417a6: mov byte ptr [rbp + 0x1d], dl
1414417a9: movzx edx, word ptr [rbp + 0x41]
1414417ad: mov word ptr [rbp + 0x1e], dx
1414417b1: mov edx, dword ptr [rbp + 0x43]
1414417b4: mov dword ptr [rbp + 0x20], edx
1414417b7: movzx edx, byte ptr [rbp + 0x47]
1414417bb: mov byte ptr [rbp + 0x24], dl
1414417be: mov qword ptr [rbp + 5], rax
1414417c2: mov qword ptr [rbp + 0xd], rcx
1414417c6: mov word ptr [rbp + 0x28], 0
1414417cc: mov rax, qword ptr [rbp + 0x1d]
1414417d0: mov qword ptr [rbp + 0x48], rax
1414417d4: mov eax, dword ptr [rbp + 0x10]
1414417d7: mov dword ptr [rbp + 0x3b], eax
1414417da: movzx eax, byte ptr [rbp + 0x14]
1414417de: mov byte ptr [rbp + 0x3f], al
1414417e1: mov rax, qword ptr [rbp + 0x15]
1414417e5: mov qword ptr [rbp + 0x40], rax
1414417e9: movzx eax, byte ptr [rbp + 0x1d]
1414417ed: mov byte ptr [rbp + 0x48], al
1414417f0: movzx eax, word ptr [rbp + 0x1e]
1414417f4: mov word ptr [rbp + 0x49], ax
1414417f8: mov eax, dword ptr [rbp]
1414417fb: mov dword ptr [rbp + 0x2b], eax
1414417fe: movzx eax, byte ptr [rbp + 4]
141441802: mov byte ptr [rbp + 0x2f], al
141441805: mov rax, qword ptr [rbp + 5]
141441809: mov qword ptr [rbp + 0x30], rax
14144180d: movzx eax, word ptr [rbp + 0xd]
141441811: mov word ptr [rbp + 0x38], ax
141441815: movzx eax, byte ptr [rbp + 0xf]
141441819: mov byte ptr [rbp + 0x3a], al
14144181c: mov rdi, qword ptr [rsi + 0x10]
141441820: cmp rdi, qword ptr [rsi]
141441823: jne 0x141441ad7
141441829: lea rdx, [rip + 0x181d8a0]
141441830: mov rcx, rsi
141441833: call 0x1405629f0
141441838: jmp 0x141441ad7
14144183d: test rax, rax
141441840: js 0x141441b81
141441846: lea rcx, [rbp + 0x28]
14144184a: mov r8, r14
14144184d: xor r9d, r9d
141441850: call 0x141700620
141441855: jmp 0x14144186f
141441857: test rax, rax
14144185a: js 0x141441b81
141441860: lea rcx, [rbp + 0x28]
141441864: mov r8, r14
141441867: xor r9d, r9d
14144186a: call 0x141700430
14144186f: mov rax, qword ptr [rbp + 0x28]
141441873: mov rcx, qword ptr [rbp + 0x30]
141441877: mov rdx, qword ptr [rbp + 0x38]
14144187b: mov qword ptr [rbp + 0x15], rdx
14144187f: movzx edx, byte ptr [rbp + 0x40]
141441883: mov byte ptr [rbp + 0x1d], dl
141441886: movzx edx, word ptr [rbp + 0x41]
14144188a: mov word ptr [rbp + 0x1e], dx
14144188e: mov edx, dword ptr [rbp + 0x43]
141441891: mov dword ptr [rbp + 0x20], edx
141441894: movzx edx, byte ptr [rbp + 0x47]
141441898: mov byte ptr [rbp + 0x24], dl
14144189b: mov qword ptr [rbp + 5], rax
14144189f: mov qword ptr [rbp + 0xd], rcx
1414418a3: mov word ptr [rbp + 0x28], 0x100
1414418a9: mov byte ptr [rbp + 0x2a], 3
1414418ad: mov rax, qword ptr [rbp + 0x1d]
1414418b1: mov qword ptr [rbp + 0x48], rax
1414418b5: mov eax, dword ptr [rbp + 0x10]
1414418b8: mov dword ptr [rbp + 0x3b], eax
1414418bb: movzx eax, byte ptr [rbp + 0x14]
1414418bf: mov byte ptr [rbp + 0x3f], al
1414418c2: mov rax, qword ptr [rbp + 0x15]
1414418c6: mov qword ptr [rbp + 0x40], rax
1414418ca: movzx eax, byte ptr [rbp + 0x1d]
1414418ce: mov byte ptr [rbp + 0x48], al
1414418d1: movzx eax, word ptr [rbp + 0x1e]
1414418d5: mov word ptr [rbp + 0x49], ax
1414418d9: mov eax, dword ptr [rbp]
1414418dc: mov dword ptr [rbp + 0x2b], eax
1414418df: movzx eax, byte ptr [rbp + 4]
1414418e3: mov byte ptr [rbp + 0x2f], al
1414418e6: mov rax, qword ptr [rbp + 5]
1414418ea: mov qword ptr [rbp + 0x30], rax
1414418ee: movzx eax, word ptr [rbp + 0xd]
1414418f2: mov word ptr [rbp + 0x38], ax
1414418f6: movzx eax, byte ptr [rbp + 0xf]
1414418fa: mov byte ptr [rbp + 0x3a], al
1414418fd: mov rdi, qword ptr [rsi + 0x10]
141441901: cmp rdi, qword ptr [rsi]
141441904: jne 0x141441ad7
14144190a: lea rdx, [rip + 0x181d7bf]
141441911: mov rcx, rsi
141441914: call 0x1405629f0
141441919: jmp 0x141441ad7
14144191e: test rax, rax
141441921: js 0x141441b81
141441927: lea rcx, [rbp + 0x28]
14144192b: mov r8, r14
14144192e: xor r9d, r9d
141441931: call 0x141700620
141441936: jmp 0x141441950
141441938: test rax, rax
14144193b: js 0x141441b81
141441941: lea rcx, [rbp + 0x28]
141441945: mov r8, r14
141441948: xor r9d, r9d
14144194b: call 0x141700430
141441950: mov rax, qword ptr [rbp + 0x28]
141441954: mov rcx, qword ptr [rbp + 0x30]
141441958: mov rdx, qword ptr [rbp + 0x38]
14144195c: mov qword ptr [rbp + 0x15], rdx
141441960: movzx edx, byte ptr [rbp + 0x40]
141441964: mov byte ptr [rbp + 0x1d], dl
141441967: movzx edx, word ptr [rbp + 0x41]
14144196b: mov word ptr [rbp + 0x1e], dx
14144196f: mov edx, dword ptr [rbp + 0x43]
141441972: mov dword ptr [rbp + 0x20], edx
141441975: movzx edx, byte ptr [rbp + 0x47]
141441979: mov byte ptr [rbp + 0x24], dl
14144197c: mov qword ptr [rbp + 5], rax
141441980: mov qword ptr [rbp + 0xd], rcx
141441984: mov word ptr [rbp + 0x28], 0x100
14144198a: mov byte ptr [rbp + 0x2a], 3
14144198e: mov rax, qword ptr [rbp + 0x1d]
141441992: mov qword ptr [rbp + 0x48], rax
141441996: mov eax, dword ptr [rbp + 0x10]
141441999: mov dword ptr [rbp + 0x3b], eax
14144199c: movzx eax, byte ptr [rbp + 0x14]
1414419a0: mov byte ptr [rbp + 0x3f], al
1414419a3: mov rax, qword ptr [rbp + 0x15]
1414419a7: mov qword ptr [rbp + 0x40], rax
1414419ab: movzx eax, byte ptr [rbp + 0x1d]
1414419af: mov byte ptr [rbp + 0x48], al
1414419b2: movzx eax, word ptr [rbp + 0x1e]
1414419b6: mov word ptr [rbp + 0x49], ax
1414419ba: mov eax, dword ptr [rbp]
1414419bd: mov dword ptr [rbp + 0x2b], eax
1414419c0: movzx eax, byte ptr [rbp + 4]
1414419c4: mov byte ptr [rbp + 0x2f], al
1414419c7: mov rax, qword ptr [rbp + 5]
1414419cb: mov qword ptr [rbp + 0x30], rax
1414419cf: movzx eax, word ptr [rbp + 0xd]
1414419d3: mov word ptr [rbp + 0x38], ax
1414419d7: movzx eax, byte ptr [rbp + 0xf]
1414419db: mov byte ptr [rbp + 0x3a], al
1414419de: mov rdi, qword ptr [rsi + 0x10]
1414419e2: cmp rdi, qword ptr [rsi]
1414419e5: jne 0x141441ad7
1414419eb: lea rdx, [rip + 0x181d6de]
1414419f2: mov rcx, rsi
1414419f5: call 0x1405629f0
1414419fa: jmp 0x141441ad7
1414419ff: test rax, rax
141441a02: js 0x141441b81
141441a08: lea rcx, [rbp + 0x28]
141441a0c: mov r8, r14
141441a0f: xor r9d, r9d
141441a12: call 0x141700620
141441a17: jmp 0x141441a31
141441a19: test rax, rax
141441a1c: js 0x141441b81
141441a22: lea rcx, [rbp + 0x28]
141441a26: mov r8, r14
141441a29: xor r9d, r9d
141441a2c: call 0x141700430
141441a31: mov rax, qword ptr [rbp + 0x28]
141441a35: mov rcx, qword ptr [rbp + 0x30]
141441a39: mov rdx, qword ptr [rbp + 0x38]
141441a3d: mov qword ptr [rbp + 0x15], rdx
141441a41: movzx edx, byte ptr [rbp + 0x40]
141441a45: mov byte ptr [rbp + 0x1d], dl
141441a48: movzx edx, word ptr [rbp + 0x41]
141441a4c: mov word ptr [rbp + 0x1e], dx
141441a50: mov edx, dword ptr [rbp + 0x43]
141441a53: mov dword ptr [rbp + 0x20], edx
141441a56: movzx edx, byte ptr [rbp + 0x47]
141441a5a: mov byte ptr [rbp + 0x24], dl
141441a5d: mov qword ptr [rbp + 5], rax
141441a61: mov qword ptr [rbp + 0xd], rcx
141441a65: mov word ptr [rbp + 0x28], 0x100
141441a6b: mov byte ptr [rbp + 0x2a], 3
141441a6f: mov rax, qword ptr [rbp + 0x1d]
141441a73: mov qword ptr [rbp + 0x48], rax
141441a77: mov eax, dword ptr [rbp + 0x10]
141441a7a: mov dword ptr [rbp + 0x3b], eax
141441a7d: movzx eax, byte ptr [rbp + 0x14]
141441a81: mov byte ptr [rbp + 0x3f], al
141441a84: mov rax, qword ptr [rbp + 0x15]
141441a88: mov qword ptr [rbp + 0x40], rax
141441a8c: movzx eax, byte ptr [rbp + 0x1d]
141441a90: mov byte ptr [rbp + 0x48], al
141441a93: movzx eax, word ptr [rbp + 0x1e]
141441a97: mov word ptr [rbp + 0x49], ax
141441a9b: mov eax, dword ptr [rbp]
141441a9e: mov dword ptr [rbp + 0x2b], eax
141441aa1: movzx eax, byte ptr [rbp + 4]
141441aa5: mov byte ptr [rbp + 0x2f], al
141441aa8: mov rax, qword ptr [rbp + 5]
141441aac: mov qword ptr [rbp + 0x30], rax
141441ab0: movzx eax, word ptr [rbp + 0xd]
141441ab4: mov word ptr [rbp + 0x38], ax
141441ab8: movzx eax, byte ptr [rbp + 0xf]
141441abc: mov byte ptr [rbp + 0x3a], al
141441abf: mov rdi, qword ptr [rsi + 0x10]
141441ac3: cmp rdi, qword ptr [rsi]
141441ac6: jne 0x141441ad7
141441ac8: lea rdx, [rip + 0x181d601]
141441acf: mov rcx, rsi
141441ad2: call 0x1405629f0
141441ad7: mov rax, qword ptr [rsi + 8]
141441adb: lea rcx, [rdi + rdi*2]
141441adf: shl rcx, 4
141441ae3: movups xmm0, xmmword ptr [rbp + 0x28]
141441ae7: movups xmm1, xmmword ptr [rbp + 0x38]
141441aeb: movups xmm2, xmmword ptr [rbp + 0x48]
141441aef: movups xmmword ptr [rax + rcx + 0x20], xmm2
141441af4: movups xmmword ptr [rax + rcx + 0x10], xmm1
141441af9: movups xmmword ptr [rax + rcx], xmm0
141441afd: inc rdi
141441b00: mov qword ptr [rsi + 0x10], rdi
141441b04: mov rax, qword ptr [rbp - 8]
141441b08: lock dec qword ptr [rax]
141441b0c: jne 0x141440b8e
141441b12: lea rcx, [rbp - 8]
141441b16: call 0x140b5cfc0
141441b1b: jmp 0x141440b8e
141441b20: mov ecx, 8
141441b25: mov edx, 0x10
141441b2a: call 0x1428d9000
141441b2f: mov qword ptr [rbp - 0x18], r12
141441b33: mov ecx, 8
141441b38: mov edx, 0x30
141441b3d: call 0x1428d9000
141441b42: jmp 0x141441b81
141441b44: mov qword ptr [rbp - 0x18], r12
141441b48: mov ecx, 8
141441b4d: mov edx, 0x30
141441b52: call 0x1428d9000
141441b57: jmp 0x141441b81
141441b59: mov qword ptr [rbp - 0x18], r12
141441b5d: mov ecx, 8
141441b62: mov edx, 0x30
141441b67: call 0x1428d9000
141441b6c: jmp 0x141441b81
141441b6e: mov qword ptr [rbp - 0x18], r12
141441b72: mov ecx, 8
141441b77: mov edx, 0x30
141441b7c: call 0x1428d9000
141441b81: ud2 
141441b83: lea rcx, [rip + 0x181d4a7]
141441b8a: lea r8, [rip + 0x181d557]
141441b91: mov edx, 0x28
141441b96: call 0x1428d9430
141441b9b: int3 
141441b9c: nop dword ptr [rax]
141441ba0: mov qword ptr [rsp + 0x10], rdx
141441ba5: push rbp
141441ba6: push r15
141441ba8: push r14
141441baa: push r12
141441bac: push rsi
141441bad: push rdi
141441bae: push rbx
141441baf: sub rsp, 0x20
141441bb3: lea rbp, [rdx + 0x80]
141441bba: mov rax, qword ptr [rbp - 0x18]
141441bbe: lock dec qword ptr [rax]
141441bc2: jne 0x141441bcd
141441bc4: lea rcx, [rbp + 0x40]
141441bc8: call 0x140b5d130
141441bcd: nop 
141441bce: add rsp, 0x20
141441bd2: pop rbx
141441bd3: pop rdi
141441bd4: pop rsi
141441bd5: pop r12
141441bd7: pop r14
141441bd9: pop r15
141441bdb: pop rbp
141441bdc: ret 
141441bdd: nop dword ptr [rax]
141441be0: mov qword ptr [rsp + 0x10], rdx
141441be5: push rbp
141441be6: push r15
141441be8: push r14
141441bea: push r12
141441bec: push rsi
141441bed: push rdi
141441bee: push rbx
141441bef: sub rsp, 0x20
141441bf3: lea rbp, [rdx + 0x80]
141441bfa: mov rax, qword ptr [rbp - 8]
141441bfe: lock dec qword ptr [rax]
141441c02: jne 0x141441c0d
141441c04: lea rcx, [rbp - 8]
141441c08: call 0x140b5cfc0
141441c0d: nop 
141441c0e: add rsp, 0x20
141441c12: pop rbx
141441c13: pop rdi
141441c14: pop rsi
141441c15: pop r12
141441c17: pop r14
141441c19: pop r15
141441c1b: pop rbp
141441c1c: ret 
141441c1d: nop dword ptr [rax]
141441c20: mov qword ptr [rsp + 0x10], rdx
141441c25: push rbp
141441c26: push r15
141441c28: push r14
141441c2a: push r12
141441c2c: push rsi
141441c2d: push rdi
141441c2e: push rbx
141441c2f: sub rsp, 0x20
141441c33: lea rbp, [rdx + 0x80]
141441c3a: lea rcx, [rbp + 0x28]
141441c3e: call 0x14143b7e0
141441c43: nop 
141441c44: add rsp, 0x20
141441c48: pop rbx
141441c49: pop rdi
141441c4a: pop rsi
141441c4b: pop r12
141441c4d: pop r14
141441c4f: pop r15
141441c51: pop rbp
141441c52: ret 
141441c53: nop word ptr cs:[rax + rax]
141441c60: mov qword ptr [rsp + 0x10], rdx
141441c65: push rbp
141441c66: push r15
141441c68: push r14
141441c6a: push r12
141441c6c: push rsi
141441c6d: push rdi
141441c6e: push rbx
141441c6f: sub rsp, 0x20
141441c73: lea rbp, [rdx + 0x80]
141441c7a: mov rax, qword ptr [rbp - 0x18]
141441c7e: lock dec qword ptr [rax]
141441c82: jne 0x141441c8d
141441c84: lea rcx, [rbp + 0x40]
141441c88: call 0x140b5d130
141441c8d: nop 
141441c8e: add rsp, 0x20
141441c92: pop rbx
141441c93: pop rdi
141441c94: pop rsi
141441c95: pop r12
141441c97: pop r14
141441c99: pop r15
141441c9b: pop rbp
141441c9c: ret 
141441c9d: nop dword ptr [rax]
141441ca0: mov qword ptr [rsp + 0x10], rdx
141441ca5: push rbp
141441ca6: push r15
141441ca8: push r14
141441caa: push r12
141441cac: push rsi
141441cad: push rdi
141441cae: push rbx
141441caf: sub rsp, 0x20
141441cb3: lea rbp, [rdx + 0x80]
141441cba: mov rax, qword ptr [rbp - 8]
141441cbe: lock dec qword ptr [rax]
141441cc2: jne 0x141441ccd
141441cc4: lea rcx, [rbp - 8]
141441cc8: call 0x140b5cfc0
141441ccd: nop 
141441cce: add rsp, 0x20
141441cd2: pop rbx
141441cd3: pop rdi
141441cd4: pop rsi
141441cd5: pop r12
141441cd7: pop r14
141441cd9: pop r15
141441cdb: pop rbp
141441cdc: ret 
141441cdd: nop dword ptr [rax]
141441ce0: mov qword ptr [rsp + 0x10], rdx
141441ce5: push rbp
141441ce6: push r15
141441ce8: push r14
141441cea: push r12
141441cec: push rsi
141441ced: push rdi
141441cee: push rbx
141441cef: sub rsp, 0x20
141441cf3: lea rbp, [rdx + 0x80]
141441cfa: lea rcx, [rbp + 0x28]
141441cfe: call 0x14143b7e0
141441d03: nop 
141441d04: add rsp, 0x20
141441d08: pop rbx
141441d09: pop rdi
141441d0a: pop rsi
141441d0b: pop r12
141441d0d: pop r14
141441d0f: pop r15
141441d11: pop rbp
141441d12: ret 
141441d13: nop word ptr cs:[rax + rax]
141441d20: mov qword ptr [rsp + 0x10], rdx
141441d25: push rbp
141441d26: push r15
141441d28: push r14
141441d2a: push r12
141441d2c: push rsi
141441d2d: push rdi
141441d2e: push rbx
141441d2f: sub rsp, 0x20
141441d33: lea rbp, [rdx + 0x80]
141441d3a: mov rax, qword ptr [rbp - 0x18]
141441d3e: lock dec qword ptr [rax]
141441d42: jne 0x141441d4d
141441d44: lea rcx, [rbp + 0x40]
141441d48: call 0x140b5d130
141441d4d: nop 
141441d4e: add rsp, 0x20
141441d52: pop rbx
141441d53: pop rdi
141441d54: pop rsi
141441d55: pop r12
141441d57: pop r14
141441d59: pop r15
141441d5b: pop rbp
141441d5c: ret 
141441d5d: nop dword ptr [rax]
141441d60: mov qword ptr [rsp + 0x10], rdx
141441d65: push rbp
141441d66: push r15
141441d68: push r14
141441d6a: push r12
141441d6c: push rsi
141441d6d: push rdi
141441d6e: push rbx
141441d6f: sub rsp, 0x20
141441d73: lea rbp, [rdx + 0x80]
141441d7a: mov rax, qword ptr [rbp - 8]
141441d7e: lock dec qword ptr [rax]
141441d82: jne 0x141441d8d
141441d84: lea rcx, [rbp - 8]
141441d88: call 0x140b5cfc0
141441d8d: nop 
141441d8e: add rsp, 0x20
141441d92: pop rbx
141441d93: pop rdi
141441d94: pop rsi
141441d95: pop r12
141441d97: pop r14
141441d99: pop r15
141441d9b: pop rbp
141441d9c: ret 
141441d9d: nop dword ptr [rax]
141441da0: mov qword ptr [rsp + 0x10], rdx
141441da5: push rbp
141441da6: push r15
141441da8: push r14
141441daa: push r12
141441dac: push rsi
141441dad: push rdi
141441dae: push rbx
141441daf: sub rsp, 0x20
141441db3: lea rbp, [rdx + 0x80]
141441dba: lea rcx, [rbp + 0x28]
141441dbe: call 0x14143b7e0
141441dc3: nop 
141441dc4: add rsp, 0x20
141441dc8: pop rbx
141441dc9: pop rdi
141441dca: pop rsi
141441dcb: pop r12
141441dcd: pop r14
141441dcf: pop r15
141441dd1: pop rbp
141441dd2: ret 
141441dd3: nop word ptr cs:[rax + rax]
141441de0: mov qword ptr [rsp + 0x10], rdx
141441de5: push rbp
141441de6: push r15
141441de8: push r14
141441dea: push r12
141441dec: push rsi
141441ded: push rdi
141441dee: push rbx
141441def: sub rsp, 0x20
141441df3: lea rbp, [rdx + 0x80]
141441dfa: mov rax, qword ptr [rbp - 0x18]
141441dfe: lock dec qword ptr [rax]
141441e02: jne 0x141441e0d
141441e04: lea rcx, [rbp + 0x40]
141441e08: call 0x140b5d130
141441e0d: nop 
141441e0e: add rsp, 0x20
141441e12: pop rbx
141441e13: pop rdi
141441e14: pop rsi
141441e15: pop r12
141441e17: pop r14
141441e19: pop r15
141441e1b: pop rbp
141441e1c: ret 
141441e1d: nop dword ptr [rax]
141441e20: mov qword ptr [rsp + 0x10], rdx
141441e25: push rbp
141441e26: push r15
141441e28: push r14
141441e2a: push r12
141441e2c: push rsi
141441e2d: push rdi
141441e2e: push rbx
141441e2f: sub rsp, 0x20
141441e33: lea rbp, [rdx + 0x80]
141441e3a: mov rax, qword ptr [rbp - 8]
141441e3e: lock dec qword ptr [rax]
141441e42: jne 0x141441e4d
141441e44: lea rcx, [rbp - 8]
141441e48: call 0x140b5cfc0
141441e4d: nop 
141441e4e: add rsp, 0x20
141441e52: pop rbx
141441e53: pop rdi
141441e54: pop rsi
141441e55: pop r12
141441e57: pop r14
141441e59: pop r15
141441e5b: pop rbp
141441e5c: ret 
141441e5d: nop dword ptr [rax]
141441e60: mov qword ptr [rsp + 0x10], rdx
141441e65: push rbp
141441e66: push r15
141441e68: push r14
141441e6a: push r12
141441e6c: push rsi
141441e6d: push rdi
141441e6e: push rbx
141441e6f: sub rsp, 0x20
141441e73: lea rbp, [rdx + 0x80]
141441e7a: lea rcx, [rbp + 0x28]
141441e7e: call 0x14143b7e0
141441e83: nop 
141441e84: add rsp, 0x20
141441e88: pop rbx
141441e89: pop rdi
141441e8a: pop rsi
141441e8b: pop r12
141441e8d: pop r14
141441e8f: pop r15
141441e91: pop rbp
141441e92: ret 
141441e93: nop word ptr cs:[rax + rax]
141441ea0: mov qword ptr [rsp + 0x10], rdx
141441ea5: push rbp
141441ea6: push r15
141441ea8: push r14
141441eaa: push r12
141441eac: push rsi
141441ead: push rdi
141441eae: push rbx
141441eaf: sub rsp, 0x20
141441eb3: lea rbp, [rdx + 0x80]
141441eba: lea rcx, [rbp + 0x28]
141441ebe: call 0x14143b7e0
141441ec3: nop 
141441ec4: add rsp, 0x20
141441ec8: pop rbx
141441ec9: pop rdi
141441eca: pop rsi
141441ecb: pop r12
141441ecd: pop r14
141441ecf: pop r15
141441ed1: pop rbp
141441ed2: ret 
141441ed3: nop word ptr cs:[rax + rax]
141441ee0: mov qword ptr [rsp + 0x10], rdx
141441ee5: push rbp
141441ee6: push r15
141441ee8: push r14
141441eea: push r12
141441eec: push rsi
141441eed: push rdi
141441eee: push rbx
141441eef: sub rsp, 0x20
141441ef3: lea rbp, [rdx + 0x80]
141441efa: lea rcx, [rbp + 0x28]
141441efe: call 0x14143b7e0
141441f03: nop 
141441f04: add rsp, 0x20
141441f08: pop rbx
141441f09: pop rdi
141441f0a: pop rsi
141441f0b: pop r12
141441f0d: pop r14
141441f0f: pop r15
141441f11: pop rbp
141441f12: ret 
141441f13: nop word ptr cs:[rax + rax]
141441f20: mov qword ptr [rsp + 0x10], rdx
141441f25: push rbp
141441f26: push r15
141441f28: push r14
141441f2a: push r12
141441f2c: push rsi
141441f2d: push rdi
141441f2e: push rbx
141441f2f: sub rsp, 0x20
141441f33: lea rbp, [rdx + 0x80]
141441f3a: lea rcx, [rbp + 0x28]
141441f3e: call 0x14143b7e0
141441f43: nop 
141441f44: add rsp, 0x20
141441f48: pop rbx
141441f49: pop rdi
141441f4a: pop rsi
141441f4b: pop r12
141441f4d: pop r14
141441f4f: pop r15
141441f51: pop rbp
141441f52: ret 
141441f53: nop word ptr cs:[rax + rax]
141441f60: mov qword ptr [rsp + 0x10], rdx
141441f65: push rbp
141441f66: push r15
141441f68: push r14
141441f6a: push r12
141441f6c: push rsi
141441f6d: push rdi
141441f6e: push rbx
141441f6f: sub rsp, 0x20
141441f73: lea rbp, [rdx + 0x80]
141441f7a: lea rcx, [rbp + 0x28]
141441f7e: call 0x14143b7e0
141441f83: nop 
141441f84: add rsp, 0x20
141441f88: pop rbx
141441f89: pop rdi
141441f8a: pop rsi
141441f8b: pop r12
141441f8d: pop r14
141441f8f: pop r15
141441f91: pop rbp
141441f92: ret 
141441f93: nop word ptr cs:[rax + rax]
141441fa0: mov qword ptr [rsp + 0x10], rdx
141441fa5: push rbp
141441fa6: push r15
141441fa8: push r14
141441faa: push r12
141441fac: push rsi
141441fad: push rdi
141441fae: push rbx
141441faf: sub rsp, 0x20
141441fb3: lea rbp, [rdx + 0x80]
141441fba: lea rcx, [rbp + 0x28]
141441fbe: call 0x14143b7e0
141441fc3: nop 
141441fc4: add rsp, 0x20
141441fc8: pop rbx
141441fc9: pop rdi
141441fca: pop rsi
141441fcb: pop r12
141441fcd: pop r14
141441fcf: pop r15
141441fd1: pop rbp
141441fd2: ret 
141441fd3: nop word ptr cs:[rax + rax]
141441fe0: mov qword ptr [rsp + 0x10], rdx
141441fe5: push rbp
141441fe6: push r15
141441fe8: push r14
141441fea: push r12
141441fec: push rsi
141441fed: push rdi
141441fee: push rbx
141441fef: sub rsp, 0x20
141441ff3: lea rbp, [rdx + 0x80]
141441ffa: lea rcx, [rbp + 0x28]
141441ffe: call 0x14143b7e0
141442003: nop 
141442004: add rsp, 0x20
141442008: pop rbx
141442009: pop rdi
14144200a: pop rsi
14144200b: pop r12
14144200d: pop r14
14144200f: pop r15
141442011: pop rbp
141442012: ret 
141442013: nop word ptr cs:[rax + rax]
141442020: mov qword ptr [rsp + 0x10], rdx
141442025: push rbp
141442026: push r15
141442028: push r14
14144202a: push r12
14144202c: push rsi
14144202d: push rdi
14144202e: push rbx
14144202f: sub rsp, 0x20
141442033: lea rbp, [rdx + 0x80]
14144203a: lea rcx, [rbp + 0x28]
14144203e: call 0x14143b7e0
141442043: nop 
141442044: add rsp, 0x20
141442048: pop rbx
141442049: pop rdi
14144204a: pop rsi
14144204b: pop r12
14144204d: pop r14
14144204f: pop r15
141442051: pop rbp
141442052: ret 
141442053: nop word ptr cs:[rax + rax]
141442060: mov qword ptr [rsp + 0x10], rdx
141442065: push rbp
141442066: push r15
141442068: push r14
14144206a: push r12
14144206c: push rsi
14144206d: push rdi
14144206e: push rbx
14144206f: sub rsp, 0x20
141442073: lea rbp, [rdx + 0x80]
14144207a: lea rcx, [rbp + 0x28]
14144207e: call 0x14143b7e0
141442083: nop 
141442084: add rsp, 0x20
141442088: pop rbx
141442089: pop rdi
14144208a: pop rsi
14144208b: pop r12
14144208d: pop r14
14144208f: pop r15
141442091: pop rbp
141442092: ret 
141442093: nop word ptr cs:[rax + rax]
1414420a0: mov qword ptr [rsp + 0x10], rdx
1414420a5: push rbp
1414420a6: push r15
1414420a8: push r14
1414420aa: push r12
1414420ac: push rsi
1414420ad: push rdi
1414420ae: push rbx
1414420af: sub rsp, 0x20
1414420b3: lea rbp, [rdx + 0x80]
1414420ba: lea rcx, [rbp + 0x28]
1414420be: call 0x14143b7e0
1414420c3: nop 
1414420c4: add rsp, 0x20
1414420c8: pop rbx
1414420c9: pop rdi
1414420ca: pop rsi
1414420cb: pop r12
1414420cd: pop r14
1414420cf: pop r15
1414420d1: pop rbp
1414420d2: ret 
1414420d3: nop word ptr cs:[rax + rax]
1414420e0: mov qword ptr [rsp + 0x10], rdx
1414420e5: push rbp
1414420e6: push r15
1414420e8: push r14
1414420ea: push r12
1414420ec: push rsi
1414420ed: push rdi
1414420ee: push rbx
1414420ef: sub rsp, 0x20
1414420f3: lea rbp, [rdx + 0x80]
1414420fa: lea rcx, [rbp + 0x28]
1414420fe: call 0x14143b7e0
141442103: nop 
141442104: add rsp, 0x20
141442108: pop rbx
141442109: pop rdi
14144210a: pop rsi
14144210b: pop r12
14144210d: pop r14
14144210f: pop r15
141442111: pop rbp
141442112: ret 
141442113: nop word ptr cs:[rax + rax]
141442120: mov qword ptr [rsp + 0x10], rdx
141442125: push rbp
141442126: push r15
141442128: push r14
14144212a: push r12
14144212c: push rsi
14144212d: push rdi
14144212e: push rbx
14144212f: sub rsp, 0x20
141442133: lea rbp, [rdx + 0x80]
14144213a: lea rcx, [rbp + 0x28]
14144213e: call 0x14143b7e0
141442143: nop 
141442144: add rsp, 0x20
141442148: pop rbx
141442149: pop rdi
14144214a: pop rsi
14144214b: pop r12
14144214d: pop r14
14144214f: pop r15
141442151: pop rbp
141442152: ret 
141442153: nop word ptr cs:[rax + rax]
141442160: mov qword ptr [rsp + 0x10], rdx
141442165: push rbp
141442166: push r15
141442168: push r14
14144216a: push r12
14144216c: push rsi
14144216d: push rdi
14144216e: push rbx
14144216f: sub rsp, 0x20
141442173: lea rbp, [rdx + 0x80]
14144217a: lea rcx, [rbp + 0x28]
14144217e: call 0x14143b7e0
141442183: nop 
141442184: add rsp, 0x20
141442188: pop rbx
141442189: pop rdi
14144218a: pop rsi
14144218b: pop r12
14144218d: pop r14
14144218f: pop r15
141442191: pop rbp
141442192: ret 
141442193: nop word ptr cs:[rax + rax]
1414421a0: mov qword ptr [rsp + 0x10], rdx
1414421a5: push rbp
1414421a6: push r15
1414421a8: push r14
1414421aa: push r12
1414421ac: push rsi
1414421ad: push rdi
1414421ae: push rbx
1414421af: sub rsp, 0x20
1414421b3: lea rbp, [rdx + 0x80]
1414421ba: lea rcx, [rbp + 0x28]
1414421be: call 0x14143b7e0
1414421c3: nop 
1414421c4: add rsp, 0x20
1414421c8: pop rbx
1414421c9: pop rdi
1414421ca: pop rsi
1414421cb: pop r12
1414421cd: pop r14
1414421cf: pop r15
1414421d1: pop rbp
1414421d2: ret 
1414421d3: nop word ptr cs:[rax + rax]
1414421e0: mov qword ptr [rsp + 0x10], rdx
1414421e5: push rbp
1414421e6: push r15
1414421e8: push r14
1414421ea: push r12
1414421ec: push rsi
1414421ed: push rdi
1414421ee: push rbx
1414421ef: sub rsp, 0x20
1414421f3: lea rbp, [rdx + 0x80]
1414421fa: lea rcx, [rbp + 0x28]
1414421fe: call 0x14143b7e0
141442203: nop 
141442204: add rsp, 0x20
141442208: pop rbx
141442209: pop rdi
14144220a: pop rsi
14144220b: pop r12
14144220d: pop r14
14144220f: pop r15
141442211: pop rbp
141442212: ret 
141442213: nop word ptr cs:[rax + rax]
141442220: mov qword ptr [rsp + 0x10], rdx
141442225: push rbp
141442226: push r15
141442228: push r14
14144222a: push r12
14144222c: push rsi
14144222d: push rdi
14144222e: push rbx
14144222f: sub rsp, 0x20
141442233: lea rbp, [rdx + 0x80]
14144223a: lea rcx, [rbp + 0x28]
14144223e: call 0x14143b7e0
141442243: nop 
141442244: add rsp, 0x20
141442248: pop rbx
141442249: pop rdi
14144224a: pop rsi
14144224b: pop r12
14144224d: pop r14
14144224f: pop r15
141442251: pop rbp
141442252: ret 
141442253: nop word ptr cs:[rax + rax]
141442260: mov qword ptr [rsp + 0x10], rdx
141442265: push rbp
141442266: push r15
141442268: push r14
14144226a: push r12
14144226c: push rsi
14144226d: push rdi
14144226e: push rbx
14144226f: sub rsp, 0x20
141442273: lea rbp, [rdx + 0x80]
14144227a: lea rcx, [rbp + 0x28]
14144227e: call 0x14143b7e0
141442283: nop 
141442284: add rsp, 0x20
141442288: pop rbx
141442289: pop rdi
14144228a: pop rsi
14144228b: pop r12
14144228d: pop r14
14144228f: pop r15
141442291: pop rbp
141442292: ret 
141442293: nop word ptr cs:[rax + rax]
1414422a0: mov qword ptr [rsp + 0x10], rdx
1414422a5: push rbp
1414422a6: push r15
1414422a8: push r14
1414422aa: push r12
1414422ac: push rsi
1414422ad: push rdi
1414422ae: push rbx
1414422af: sub rsp, 0x20
1414422b3: lea rbp, [rdx + 0x80]
1414422ba: lea rcx, [rbp + 0x28]
1414422be: call 0x14143b7e0
1414422c3: nop 
1414422c4: add rsp, 0x20
1414422c8: pop rbx
1414422c9: pop rdi
1414422ca: pop rsi
1414422cb: pop r12
1414422cd: pop r14
1414422cf: pop r15
1414422d1: pop rbp
1414422d2: ret 
1414422d3: nop word ptr cs:[rax + rax]
1414422e0: mov qword ptr [rsp + 0x10], rdx
1414422e5: push rbp
1414422e6: push r15
1414422e8: push r14
1414422ea: push r12
1414422ec: push rsi
1414422ed: push rdi
1414422ee: push rbx
1414422ef: sub rsp, 0x20
1414422f3: lea rbp, [rdx + 0x80]
1414422fa: lea rcx, [rbp + 0x28]
1414422fe: call 0x14143b7e0
141442303: nop 
141442304: add rsp, 0x20
141442308: pop rbx
141442309: pop rdi
14144230a: pop rsi
14144230b: pop r12
14144230d: pop r14
14144230f: pop r15
141442311: pop rbp
141442312: ret 
141442313: nop word ptr cs:[rax + rax]
141442320: mov qword ptr [rsp + 0x10], rdx
141442325: push rbp
141442326: push r15
141442328: push r14
14144232a: push r12
14144232c: push rsi
14144232d: push rdi
14144232e: push rbx
14144232f: sub rsp, 0x20
141442333: lea rbp, [rdx + 0x80]
14144233a: lea rcx, [rbp + 0x28]
14144233e: call 0x14143b7e0
141442343: nop 
141442344: add rsp, 0x20
141442348: pop rbx
141442349: pop rdi
14144234a: pop rsi
14144234b: pop r12
14144234d: pop r14
14144234f: pop r15
141442351: pop rbp
141442352: ret 
141442353: nop word ptr cs:[rax + rax]
141442360: mov qword ptr [rsp + 0x10], rdx
141442365: push rbp
141442366: push r15
141442368: push r14
14144236a: push r12
14144236c: push rsi
14144236d: push rdi
14144236e: push rbx
14144236f: sub rsp, 0x20
141442373: lea rbp, [rdx + 0x80]
14144237a: lea rcx, [rbp + 0x28]
14144237e: call 0x14143b7e0
141442383: nop 
141442384: add rsp, 0x20
141442388: pop rbx
141442389: pop rdi
14144238a: pop rsi
14144238b: pop r12
14144238d: pop r14
14144238f: pop r15
141442391: pop rbp
141442392: ret 
141442393: nop word ptr cs:[rax + rax]
1414423a0: mov qword ptr [rsp + 0x10], rdx
1414423a5: push rbp
1414423a6: push r15
1414423a8: push r14
1414423aa: push r12
1414423ac: push rsi
1414423ad: push rdi
1414423ae: push rbx
1414423af: sub rsp, 0x20
1414423b3: lea rbp, [rdx + 0x80]
1414423ba: lea rcx, [rbp + 0x28]
1414423be: call 0x14143b7e0
1414423c3: nop 
1414423c4: add rsp, 0x20
1414423c8: pop rbx
1414423c9: pop rdi
1414423ca: pop rsi
1414423cb: pop r12
1414423cd: pop r14
1414423cf: pop r15
1414423d1: pop rbp
1414423d2: ret 

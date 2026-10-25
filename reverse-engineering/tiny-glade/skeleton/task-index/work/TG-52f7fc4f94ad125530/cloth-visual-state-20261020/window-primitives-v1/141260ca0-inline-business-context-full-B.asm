141260ca0: 55 push rbp
141260ca1: 4157 push r15
141260ca3: 4156 push r14
141260ca5: 4155 push r13
141260ca7: 4154 push r12
141260ca9: 56 push rsi
141260caa: 57 push rdi
141260cab: 53 push rbx
141260cac: 4881ec98010000 sub rsp, 0x198
141260cb3: 488dac2480000000 lea rbp, [rsp + 0x80]
141260cbb: 440f29ad00010000 movaps xmmword ptr [rbp + 0x100], xmm13
141260cc3: 440f29a5f0000000 movaps xmmword ptr [rbp + 0xf0], xmm12
141260ccb: 440f299de0000000 movaps xmmword ptr [rbp + 0xe0], xmm11
141260cd3: 440f2995d0000000 movaps xmmword ptr [rbp + 0xd0], xmm10
141260cdb: 440f298dc0000000 movaps xmmword ptr [rbp + 0xc0], xmm9
141260ce3: 440f2985b0000000 movaps xmmword ptr [rbp + 0xb0], xmm8
141260ceb: 0f29bda0000000 movaps xmmword ptr [rbp + 0xa0], xmm7
141260cf2: 0f29b590000000 movaps xmmword ptr [rbp + 0x90], xmm6
141260cf9: 48c78588000000feffffff mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
141260d04: 4d89c5 mov r13, r8
141260d07: 4889d7 mov rdi, rdx
141260d0a: 4889ce mov rsi, rcx
141260d0d: 488d4db0 lea rcx, [rbp - 0x50]
141260d11: e83ac70300 call 0x14129d450
141260d16: f645b001 test byte ptr [rbp - 0x50], 1
141260d1a: 0f8434010000 je 0x141260e54
141260d20: f30f104dbc movss xmm1, dword ptr [rbp - 0x44]
141260d25: f30f1045c8 movss xmm0, dword ptr [rbp - 0x38]
141260d2a: 0f28f9 movaps xmm7, xmm1
141260d2d: f30f58f8 addss xmm7, xmm0
141260d31: f30f593d374b6c01 mulss xmm7, dword ptr [rip + 0x16c4b37]
141260d39: 488d4d00 lea rcx, [rbp]
141260d3d: f30f5cc1 subss xmm0, xmm1
141260d41: 0f28c8 movaps xmm1, xmm0
141260d44: f30f59c8 mulss xmm1, xmm0
141260d48: 0f5705a14b6c01 xorps xmm0, xmmword ptr [rip + 0x16c4ba1]
141260d4f: f20f105db4 movsd xmm3, qword ptr [rbp - 0x4c]
141260d54: f20f1055c0 movsd xmm2, qword ptr [rbp - 0x40]
141260d59: 440f28c3 movaps xmm8, xmm3
141260d5d: 440f58c2 addps xmm8, xmm2
141260d61: 440f5905774b6c01 mulps xmm8, xmmword ptr [rip + 0x16c4b77]
141260d69: 0f5cd3 subps xmm2, xmm3
141260d6c: 0f28da movaps xmm3, xmm2
141260d6f: 0f59da mulps xmm3, xmm2
141260d72: f30f16e3 movshdup xmm4, xmm3
141260d76: f30f58e3 addss xmm4, xmm3
141260d7a: f30f58e1 addss xmm4, xmm1
141260d7e: 0f57db xorps xmm3, xmm3
141260d81: f30f51dc sqrtss xmm3, xmm4
141260d85: f30f1035774b6c01 movss xmm6, dword ptr [rip + 0x16c4b77]
141260d8d: 0f28ce movaps xmm1, xmm6
141260d90: f30f5ecb divss xmm1, xmm3
141260d94: f30f59d1 mulss xmm2, xmm1
141260d98: f30f59c8 mulss xmm1, xmm0
141260d9c: e82f02a3ff call 0x140c90fd0
141260da1: 440f134510 movlps qword ptr [rbp + 0x10], xmm8
141260da6: f30f117d18 movss dword ptr [rbp + 0x18], xmm7
141260dab: 488b058eb0d601 mov rax, qword ptr [rip + 0x1d6b08e]
141260db2: 4889451c mov qword ptr [rbp + 0x1c], rax
141260db6: 8b158cb0d601 mov edx, dword ptr [rip + 0x1d6b08c]
141260dbc: 488b4708 mov rax, qword ptr [rdi + 8]
141260dc0: 4531ff xor r15d, r15d
141260dc3: 482b07 sub rax, qword ptr [rdi]
141260dc6: 490f42c7 cmovb rax, r15
141260dca: 4889c1 mov rcx, rax
141260dcd: 4883e901 sub rcx, 1
141260dd1: 490f42cf cmovb rcx, r15
141260dd5: 837f3000 cmp dword ptr [rdi + 0x30], 0
141260dd9: 895524 mov dword ptr [rbp + 0x24], edx
141260ddc: 480f45c8 cmovne rcx, rax
141260de0: 48ffc1 inc rcx
141260de3: 48c7c0ffffffff mov rax, 0xffffffffffffffff
141260dea: 480f45c1 cmovne rax, rcx
141260dee: 4883f805 cmp rax, 5
141260df2: 41bc04000000 mov r12d, 4
141260df8: 4c0f43e0 cmovae r12, rax
141260dfc: b930000000 mov ecx, 0x30
141260e01: 4c89e0 mov rax, r12
141260e04: 48f7e1 mul rcx
141260e07: 4989c6 mov r14, rax
141260e0a: 0f8083020000 jo 0x141261093
141260e10: 48b8f0ffffffffffff7f movabs rax, 0x7ffffffffffffff0
141260e1a: 4939c6 cmp r14, rax
141260e1d: 0f8770020000 ja 0x141261093
141260e23: 4d85f6 test r14, r14
141260e26: 48897568 mov qword ptr [rbp + 0x68], rsi
141260e2a: 7444 je 0x141260e70
141260e2c: 0fb605d63d3d02 movzx eax, byte ptr [rip + 0x23d3dd6]
141260e33: 41bf10000000 mov r15d, 0x10
141260e39: ba10000000 mov edx, 0x10
141260e3e: 4c89f1 mov rcx, r14
141260e41: e8ca2d3bff call 0x140613c10
141260e46: 4885c0 test rax, rax
141260e49: 0f8444020000 je 0x141261093
141260e4f: 4889c3 mov rbx, rax
141260e52: eb24 jmp 0x141260e78
141260e54: 48c70600000000 mov qword ptr [rsi], 0
141260e5b: 48c7460810000000 mov qword ptr [rsi + 8], 0x10
141260e63: 48c7461000000000 mov qword ptr [rsi + 0x10], 0
141260e6b: e9ce010000 jmp 0x14126103e
141260e70: bb10000000 mov ebx, 0x10
141260e75: 4531e4 xor r12d, r12d
141260e78: 488d4500 lea rax, [rbp]
141260e7c: 0f2800 movaps xmm0, xmmword ptr [rax]
141260e7f: 0f284810 movaps xmm1, xmmword ptr [rax + 0x10]
141260e83: 0f285020 movaps xmm2, xmmword ptr [rax + 0x20]
141260e87: 0f295320 movaps xmmword ptr [rbx + 0x20], xmm2
141260e8b: 0f294b10 movaps xmmword ptr [rbx + 0x10], xmm1
141260e8f: 0f2903 movaps xmmword ptr [rbx], xmm0
141260e92: 4c896570 mov qword ptr [rbp + 0x70], r12
141260e96: 48895d78 mov qword ptr [rbp + 0x78], rbx
141260e9a: 48c7858000000001000000 mov qword ptr [rbp + 0x80], 1
141260ea5: 0f104740 movups xmm0, xmmword ptr [rdi + 0x40]
141260ea9: 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
141260ead: 0f1007 movups xmm0, xmmword ptr [rdi]
141260eb0: 0f104f10 movups xmm1, xmmword ptr [rdi + 0x10]
141260eb4: 0f105720 movups xmm2, xmmword ptr [rdi + 0x20]
141260eb8: 0f105f30 movups xmm3, xmmword ptr [rdi + 0x30]
141260ebc: 0f295d20 movaps xmmword ptr [rbp + 0x20], xmm3
141260ec0: 0f295510 movaps xmmword ptr [rbp + 0x10], xmm2
141260ec4: 0f294d00 movaps xmmword ptr [rbp], xmm1
141260ec8: 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
141260ecc: 488d7dc0 lea rdi, [rbp - 0x40]
141260ed0: 41be01000000 mov r14d, 1
141260ed6: be30000000 mov esi, 0x30
141260edb: 4c8d7d4c lea r15, [rbp + 0x4c]
141260edf: 4c8d65f0 lea r12, [rbp - 0x10]
141260ee3: 0f283d064a6c01 movaps xmm7, xmmword ptr [rip + 0x16c4a06]
141260eea: f3440f10057d496c01 movss xmm8, dword ptr [rip + 0x16c497d]
141260ef3: 440f280de5496c01 movaps xmm9, xmmword ptr [rip + 0x16c49e5]
141260efb: 4531ed xor r13d, r13d
141260efe: eb27 jmp 0x141260f27
141260f00: 0f2807 movaps xmm0, xmmword ptr [rdi]
141260f03: 0f284f10 movaps xmm1, xmmword ptr [rdi + 0x10]
141260f07: 0f285720 movaps xmm2, xmmword ptr [rdi + 0x20]
141260f0b: 0f29543320 movaps xmmword ptr [rbx + rsi + 0x20], xmm2
141260f10: 0f294c3310 movaps xmmword ptr [rbx + rsi + 0x10], xmm1
141260f15: 0f290433 movaps xmmword ptr [rbx + rsi], xmm0
141260f19: 49ffc6 inc r14
141260f1c: 4c89b580000000 mov qword ptr [rbp + 0x80], r14
141260f23: 4883c630 add rsi, 0x30
141260f27: 4c89f9 mov rcx, r15
141260f2a: 4c89e2 mov rdx, r12
141260f2d: e81ec50300 call 0x14129d450
141260f32: 837d4c01 cmp dword ptr [rbp + 0x4c], 1
141260f36: 0f85ec000000 jne 0x141261028
141260f3c: f3440f105558 movss xmm10, dword ptr [rbp + 0x58]
141260f42: f2440f105d50 movsd xmm11, qword ptr [rbp + 0x50]
141260f48: f2440f10655c movsd xmm12, qword ptr [rbp + 0x5c]
141260f4e: f3440f106d64 movss xmm13, dword ptr [rbp + 0x64]
141260f54: 410f28cd movaps xmm1, xmm13
141260f58: f3410f5cca subss xmm1, xmm10
141260f5d: 410f28d4 movaps xmm2, xmm12
141260f61: 410f5cd3 subps xmm2, xmm11
141260f65: 0f28c2 movaps xmm0, xmm2
141260f68: 0f59c2 mulps xmm0, xmm2
141260f6b: f30f16d8 movshdup xmm3, xmm0
141260f6f: f30f58d8 addss xmm3, xmm0
141260f73: 0f28c1 movaps xmm0, xmm1
141260f76: f30f59c1 mulss xmm0, xmm1
141260f7a: f30f58c3 addss xmm0, xmm3
141260f7e: f30f51c0 sqrtss xmm0, xmm0
141260f82: 0f28de movaps xmm3, xmm6
141260f85: f30f5ed8 divss xmm3, xmm0
141260f89: f30f59d3 mulss xmm2, xmm3
141260f8d: 0f57cf xorps xmm1, xmm7
141260f90: f30f59cb mulss xmm1, xmm3
141260f94: 4889f9 mov rcx, rdi
141260f97: e83400a3ff call 0x140c90fd0
141260f9c: f3450f58d5 addss xmm10, xmm13
141260fa1: f3450f59d0 mulss xmm10, xmm8
141260fa6: 450f58dc addps xmm11, xmm12
141260faa: 450f59d9 mulps xmm11, xmm9
141260fae: 440f135dd0 movlps qword ptr [rbp - 0x30], xmm11
141260fb3: f3440f1155d8 movss dword ptr [rbp - 0x28], xmm10
141260fb9: 8b0589aed601 mov eax, dword ptr [rip + 0x1d6ae89]
141260fbf: 894724 mov dword ptr [rdi + 0x24], eax
141260fc2: 488b0577aed601 mov rax, qword ptr [rip + 0x1d6ae77]
141260fc9: 4889471c mov qword ptr [rdi + 0x1c], rax
141260fcd: 4c3b7570 cmp r14, qword ptr [rbp + 0x70]
141260fd1: 0f8529ffffff jne 0x141260f00
141260fd7: 488b45f8 mov rax, qword ptr [rbp - 8]
141260fdb: 482b45f0 sub rax, qword ptr [rbp - 0x10]
141260fdf: 490f42c5 cmovb rax, r13
141260fe3: 4989c0 mov r8, rax
141260fe6: 4983e801 sub r8, 1
141260fea: 4d0f42c5 cmovb r8, r13
141260fee: 837d2000 cmp dword ptr [rbp + 0x20], 0
141260ff2: 4c0f45c0 cmovne r8, rax
141260ff6: 49ffc0 inc r8
141260ff9: 48c7c0ffffffff mov rax, 0xffffffffffffffff
141261000: 4c0f44c0 cmove r8, rax
141261004: 48c744242030000000 mov qword ptr [rsp + 0x20], 0x30
14126100d: 41b910000000 mov r9d, 0x10
141261013: 488d4d70 lea rcx, [rbp + 0x70]
141261017: 4c89f2 mov rdx, r14
14126101a: e8016f6401 call 0x1428a7f20
14126101f: 488b5d78 mov rbx, qword ptr [rbp + 0x78]
141261023: e9d8feffff jmp 0x141260f00
141261028: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
14126102f: 488b7568 mov rsi, qword ptr [rbp + 0x68]
141261033: 48894610 mov qword ptr [rsi + 0x10], rax
141261037: 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
14126103b: 0f1106 movups xmmword ptr [rsi], xmm0
14126103e: 4889f0 mov rax, rsi
141261041: 0f28b590000000 movaps xmm6, xmmword ptr [rbp + 0x90]
141261048: 0f28bda0000000 movaps xmm7, xmmword ptr [rbp + 0xa0]
14126104f: 440f2885b0000000 movaps xmm8, xmmword ptr [rbp + 0xb0]
141261057: 440f288dc0000000 movaps xmm9, xmmword ptr [rbp + 0xc0]
14126105f: 440f2895d0000000 movaps xmm10, xmmword ptr [rbp + 0xd0]
141261067: 440f289de0000000 movaps xmm11, xmmword ptr [rbp + 0xe0]
14126106f: 440f28a5f0000000 movaps xmm12, xmmword ptr [rbp + 0xf0]
141261077: 440f28ad00010000 movaps xmm13, xmmword ptr [rbp + 0x100]
14126107f: 4881c498010000 add rsp, 0x198
141261086: 5b pop rbx
141261087: 5f pop rdi
141261088: 5e pop rsi
141261089: 415c pop r12
14126108b: 415d pop r13
14126108d: 415e pop r14
14126108f: 415f pop r15
141261091: 5d pop rbp
141261092: c3 ret 
141261093: 4c89f9 mov rcx, r15
141261096: 4c89f2 mov rdx, r14
141261099: 4d89e8 mov r8, r13
14126109c: e8427f6701 call 0x1428d8fe3
1412610a1: cc int3 
1412610a2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1412610b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1412610b5: 55 push rbp
1412610b6: 4157 push r15
1412610b8: 4156 push r14
1412610ba: 4155 push r13
1412610bc: 4154 push r12
1412610be: 56 push rsi
1412610bf: 57 push rdi
1412610c0: 53 push rbx
1412610c1: 4881eca8000000 sub rsp, 0xa8
1412610c8: 488daa80000000 lea rbp, [rdx + 0x80]
1412610cf: 440f296c2420 movaps xmmword ptr [rsp + 0x20], xmm13
1412610d5: 440f29642430 movaps xmmword ptr [rsp + 0x30], xmm12
1412610db: 440f295c2440 movaps xmmword ptr [rsp + 0x40], xmm11
1412610e1: 440f29542450 movaps xmmword ptr [rsp + 0x50], xmm10
1412610e7: 440f294c2460 movaps xmmword ptr [rsp + 0x60], xmm9
1412610ed: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412610f3: 0f29bc2480000000 movaps xmmword ptr [rsp + 0x80], xmm7
1412610fb: 0f29b42490000000 movaps xmmword ptr [rsp + 0x90], xmm6
141261103: 488b4570 mov rax, qword ptr [rbp + 0x70]
141261107: 4885c0 test rax, rax
14126110a: 7417 je 0x141261123
14126110c: 488b4d78 mov rcx, qword ptr [rbp + 0x78]
141261110: 48c1e004 shl rax, 4
141261114: 488d1440 lea rdx, [rax + rax*2]
141261118: 41b810000000 mov r8d, 0x10
14126111e: e8fd2a3bff call 0x140613c20
141261123: 0f28b42490000000 movaps xmm6, xmmword ptr [rsp + 0x90]
14126112b: 0f28bc2480000000 movaps xmm7, xmmword ptr [rsp + 0x80]
141261133: 440f28442470 movaps xmm8, xmmword ptr [rsp + 0x70]
141261139: 440f284c2460 movaps xmm9, xmmword ptr [rsp + 0x60]
14126113f: 440f28542450 movaps xmm10, xmmword ptr [rsp + 0x50]
141261145: 440f285c2440 movaps xmm11, xmmword ptr [rsp + 0x40]
14126114b: 440f28642430 movaps xmm12, xmmword ptr [rsp + 0x30]
141261151: 440f286c2420 movaps xmm13, xmmword ptr [rsp + 0x20]
141261157: 4881c4a8000000 add rsp, 0xa8
14126115e: 5b pop rbx
14126115f: 5f pop rdi
141261160: 5e pop rsi
141261161: 415c pop r12
141261163: 415d pop r13
141261165: 415e pop r14
141261167: 415f pop r15
141261169: 5d pop rbp
14126116a: c3 ret 

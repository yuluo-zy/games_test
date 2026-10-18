141452870: push rbp
141452871: push rsi
141452872: push rdi
141452873: sub rsp, 0x1f0
14145287a: lea rbp, [rsp + 0x80]
141452882: mov qword ptr [rbp + 0x168], 0xfffffffffffffffe
14145288d: mov rdi, rdx
141452890: mov rsi, rcx
141452893: movups xmm0, xmmword ptr [rdx + 0x60]
141452897: movaps xmmword ptr [rbp], xmm0
14145289b: movups xmm0, xmmword ptr [rdx + 0x50]
14145289f: movaps xmmword ptr [rbp - 0x10], xmm0
1414528a3: movups xmm0, xmmword ptr [rdx + 0x40]
1414528a7: movaps xmmword ptr [rbp - 0x20], xmm0
1414528ab: movups xmm0, xmmword ptr [rdx]
1414528ae: movups xmm1, xmmword ptr [rdx + 0x10]
1414528b2: movups xmm2, xmmword ptr [rdx + 0x20]
1414528b6: movups xmm3, xmmword ptr [rdx + 0x30]
1414528ba: movaps xmmword ptr [rbp - 0x30], xmm3
1414528be: movaps xmmword ptr [rbp - 0x40], xmm2
1414528c2: movaps xmmword ptr [rbp - 0x50], xmm1
1414528c6: movaps xmmword ptr [rbp - 0x60], xmm0
1414528ca: movzx eax, byte ptr [rip + 0x21e2338]
1414528d1: mov ecx, 0x150
1414528d6: mov edx, 8
1414528db: call 0x140613c10
1414528e0: mov qword ptr [rbp + 0x160], rax
1414528e7: test rax, rax
1414528ea: je 0x141452b04
1414528f0: movups xmm0, xmmword ptr [rdi + 0x60]
1414528f4: movaps xmmword ptr [rbp + 0xe0], xmm0
1414528fb: movups xmm0, xmmword ptr [rdi + 0x50]
1414528ff: movaps xmmword ptr [rbp + 0xd0], xmm0
141452906: movups xmm0, xmmword ptr [rdi + 0x40]
14145290a: movaps xmmword ptr [rbp + 0xc0], xmm0
141452911: movups xmm0, xmmword ptr [rdi]
141452914: movups xmm1, xmmword ptr [rdi + 0x10]
141452918: movups xmm2, xmmword ptr [rdi + 0x20]
14145291c: movups xmm3, xmmword ptr [rdi + 0x30]
141452920: movaps xmmword ptr [rbp + 0xb0], xmm3
141452927: movaps xmmword ptr [rbp + 0xa0], xmm2
14145292e: movaps xmmword ptr [rbp + 0x90], xmm1
141452935: movaps xmmword ptr [rbp + 0x80], xmm0
14145293c: lea rcx, [rbp + 0x10]
141452940: call 0x141736760
141452945: lea rcx, [rbp + 0xf0]
14145294c: call 0x140839b80
141452951: movaps xmm0, xmmword ptr [rbp + 0xe0]
141452958: mov rax, qword ptr [rbp + 0x160]
14145295f: movups xmmword ptr [rax + 0x60], xmm0
141452963: movaps xmm0, xmmword ptr [rbp + 0xd0]
14145296a: movups xmmword ptr [rax + 0x50], xmm0
14145296e: movaps xmm0, xmmword ptr [rbp + 0xc0]
141452975: movups xmmword ptr [rax + 0x40], xmm0
141452979: movaps xmm0, xmmword ptr [rbp + 0x80]
141452980: movaps xmm1, xmmword ptr [rbp + 0x90]
141452987: movaps xmm2, xmmword ptr [rbp + 0xa0]
14145298e: movaps xmm3, xmmword ptr [rbp + 0xb0]
141452995: movups xmmword ptr [rax + 0x30], xmm3
141452999: movups xmmword ptr [rax + 0x20], xmm2
14145299d: movups xmmword ptr [rax + 0x10], xmm1
1414529a1: movups xmmword ptr [rax], xmm0
1414529a4: movups xmm0, xmmword ptr [rbp + 0x10]
1414529a8: movups xmm1, xmmword ptr [rbp + 0x20]
1414529ac: movups xmm2, xmmword ptr [rbp + 0x30]
1414529b0: movups xmm3, xmmword ptr [rbp + 0x40]
1414529b4: movups xmmword ptr [rax + 0x70], xmm0
1414529b8: movups xmmword ptr [rax + 0x80], xmm1
1414529bf: movups xmmword ptr [rax + 0x90], xmm2
1414529c6: movups xmmword ptr [rax + 0xa0], xmm3
1414529cd: movups xmm0, xmmword ptr [rbp + 0x50]
1414529d1: movups xmmword ptr [rax + 0xb0], xmm0
1414529d8: movups xmm0, xmmword ptr [rbp + 0x60]
1414529dc: movups xmmword ptr [rax + 0xc0], xmm0
1414529e3: movups xmm0, xmmword ptr [rbp + 0x70]
1414529e7: movups xmmword ptr [rax + 0xd0], xmm0
1414529ee: movups xmm0, xmmword ptr [rbp + 0xf0]
1414529f5: movups xmm1, xmmword ptr [rbp + 0x100]
1414529fc: movups xmm2, xmmword ptr [rbp + 0x110]
141452a03: movups xmm3, xmmword ptr [rbp + 0x120]
141452a0a: movups xmmword ptr [rax + 0xe0], xmm0
141452a11: movups xmmword ptr [rax + 0xf0], xmm1
141452a18: movups xmmword ptr [rax + 0x100], xmm2
141452a1f: movups xmmword ptr [rax + 0x110], xmm3
141452a26: movups xmm0, xmmword ptr [rbp + 0x130]
141452a2d: movups xmmword ptr [rax + 0x120], xmm0
141452a34: movups xmm0, xmmword ptr [rbp + 0x140]
141452a3b: movups xmmword ptr [rax + 0x130], xmm0
141452a42: movups xmm0, xmmword ptr [rbp + 0x150]
141452a49: movups xmmword ptr [rax + 0x140], xmm0
141452a50: mov qword ptr [rbp + 0xf8], 3
141452a5b: mov qword ptr [rbp + 0x100], rax
141452a62: mov qword ptr [rbp + 0x108], 3
141452a6d: mov qword ptr [rbp + 0x110], 0
141452a78: mov qword ptr [rbp + 0x118], 8
141452a83: lea rcx, [rbp + 0x128]
141452a8a: xorps xmm0, xmm0
141452a8d: movups xmmword ptr [rbp + 0x120], xmm0
141452a94: movabs rax, 0x8000000000000000
141452a9e: mov qword ptr [rbp + 0xf0], rax
141452aa5: call 0x140d2fa20
141452aaa: movups xmm0, xmmword ptr [rbp + 0x150]
141452ab1: movups xmmword ptr [rsi + 0x60], xmm0
141452ab5: movups xmm0, xmmword ptr [rbp + 0x140]
141452abc: movups xmmword ptr [rsi + 0x50], xmm0
141452ac0: movups xmm0, xmmword ptr [rbp + 0x130]
141452ac7: movups xmmword ptr [rsi + 0x40], xmm0
141452acb: movups xmm0, xmmword ptr [rbp + 0xf0]
141452ad2: movups xmm1, xmmword ptr [rbp + 0x100]
141452ad9: movups xmm2, xmmword ptr [rbp + 0x110]
141452ae0: movups xmm3, xmmword ptr [rbp + 0x120]
141452ae7: movups xmmword ptr [rsi + 0x30], xmm3
141452aeb: movups xmmword ptr [rsi + 0x20], xmm2
141452aef: movups xmmword ptr [rsi + 0x10], xmm1
141452af3: movups xmmword ptr [rsi], xmm0
141452af6: mov rax, rsi
141452af9: add rsp, 0x1f0
141452b00: pop rdi
141452b01: pop rsi
141452b02: pop rbp
141452b03: ret 
141452b04: mov ecx, 8
141452b09: mov edx, 0x150
141452b0e: call 0x1428d9000
141452b13: ud2 
141452b15: nop word ptr cs:[rax + rax]
141452b20: mov qword ptr [rsp + 0x10], rdx
141452b25: push rbp
141452b26: push rsi
141452b27: push rdi
141452b28: sub rsp, 0x20
141452b2c: lea rbp, [rdx + 0x80]
141452b33: lea rcx, [rbp + 0x80]
141452b3a: call 0x140029880
141452b3f: mov edx, 0x150
141452b44: mov r8d, 8
141452b4a: mov rcx, qword ptr [rbp + 0x160]
141452b51: call 0x140613c20
141452b56: nop 
141452b57: add rsp, 0x20
141452b5b: pop rdi
141452b5c: pop rsi
141452b5d: pop rbp
141452b5e: ret 
141452b5f: nop 
141452b60: mov qword ptr [rsp + 0x10], rdx
141452b65: push rbp
141452b66: push rsi
141452b67: push rdi
141452b68: sub rsp, 0x20
141452b6c: lea rbp, [rdx + 0x80]
141452b73: lea rcx, [rbp + 0x10]
141452b77: call 0x140029880
141452b7c: nop 
141452b7d: add rsp, 0x20
141452b81: pop rdi
141452b82: pop rsi
141452b83: pop rbp
141452b84: ret 
141452b85: nop word ptr cs:[rax + rax]
141452b90: mov qword ptr [rsp + 0x10], rdx
141452b95: push rbp
141452b96: push rsi
141452b97: push rdi
141452b98: sub rsp, 0x20
141452b9c: lea rbp, [rdx + 0x80]
141452ba3: cmp qword ptr [rbp + 0x160], 0
141452bab: je 0x141452bb5
141452bad: add rsp, 0x20
141452bb1: pop rdi
141452bb2: pop rsi
141452bb3: pop rbp
141452bb4: ret 
141452bb5: lea rcx, [rbp - 0x60]
141452bb9: call 0x140029880
141452bbe: jmp 0x141452bad
141452bc0: mov qword ptr [rsp + 0x10], rdx
141452bc5: push rbp
141452bc6: push rsi
141452bc7: push rdi
141452bc8: sub rsp, 0x20
141452bcc: lea rbp, [rdx + 0x80]
141452bd3: lea rcx, [rbp + 0xf0]
141452bda: call 0x140029880
141452bdf: nop 
141452be0: add rsp, 0x20
141452be4: pop rdi
141452be5: pop rsi
141452be6: pop rbp
141452be7: ret 

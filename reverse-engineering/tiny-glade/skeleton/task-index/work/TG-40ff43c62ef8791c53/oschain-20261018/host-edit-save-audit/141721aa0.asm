141721aa0: push rbp
141721aa1: push r15
141721aa3: push r14
141721aa5: push r13
141721aa7: push r12
141721aa9: push rsi
141721aaa: push rdi
141721aab: push rbx
141721aac: mov eax, 0x1838
141721ab1: call 0x14289aa40
141721ab6: sub rsp, rax
141721ab9: lea rbp, [rsp + 0x80]
141721ac1: movaps xmmword ptr [rbp + 0x17a0], xmm7
141721ac8: movaps xmmword ptr [rbp + 0x1790], xmm6
141721acf: and rsp, 0xffffffffffffff80
141721ad3: mov rbx, rsp
141721ad6: mov qword ptr [rbx + 0x1800], rbp
141721add: mov qword ptr [rbp + 0x1788], 0xfffffffffffffffe
141721ae8: mov r12, r9
141721aeb: mov rax, qword ptr [rcx]
141721aee: lea r9, [rax - 3]
141721af2: cmp r9, 8
141721af6: mov r10d, 5
141721afc: cmovb r10, r9
141721b00: mov r15, rdx
141721b03: lea rdx, [rip + 0x15a98f6]
141721b0a: movsxd r9, dword ptr [rdx + r10*4]
141721b0e: add r9, rdx
141721b11: jmp r9
141721b14: movups xmm0, xmmword ptr [rcx + 0x30]
141721b18: movaps xmmword ptr [rbx + 0x6f0], xmm0
141721b1f: movups xmm0, xmmword ptr [rcx + 8]
141721b23: movups xmm1, xmmword ptr [rcx + 0x18]
141721b27: movaps xmmword ptr [rbx + 0x1a0], xmm1
141721b2e: movaps xmmword ptr [rbx + 0x190], xmm0
141721b35: mov rax, qword ptr [rcx + 0x28]
141721b39: mov qword ptr [rbx + 0xb8], rax
141721b40: mov r14, qword ptr [rip + 0x1f0a861]
141721b47: mov rax, qword ptr [r14]
141721b4a: cmp rax, 3
141721b4e: jb 0x141721c3d
141721b54: lea rax, [rbx + 0x6f0]
141721b5b: mov qword ptr [rbx + 0x200], rax
141721b62: lea rax, [rip + 0xad37]
141721b69: mov qword ptr [rbx + 0x208], rax
141721b70: mov byte ptr [rbx + 0x7f], 1
141721b74: lea rcx, [rip + 0x15a8f45]
141721b7b: call 0x14067dd80
141721b80: movups xmm0, xmmword ptr [rax]
141721b83: mov eax, dword ptr [rax + 0x10]
141721b86: mov qword ptr [rbx + 0x730], 3
141721b91: lea rcx, [rip + 0x15a89c8]
141721b98: mov qword ptr [rbx + 0x738], rcx
141721b9f: mov qword ptr [rbx + 0x740], 0x28
141721baa: lea rdx, [rip + 0x15a8eef]
141721bb1: mov qword ptr [rbx + 0x750], rdx
141721bb8: mov qword ptr [rbx + 0x758], 2
141721bc3: lea rdx, [rbx + 0x200]
141721bca: mov qword ptr [rbx + 0x760], rdx
141721bd1: mov qword ptr [rbx + 0x768], 1
141721bdc: mov qword ptr [rbx + 0x770], 0
141721be7: mov qword ptr [rbx + 0x700], 0
141721bf2: mov qword ptr [rbx + 0x708], rcx
141721bf9: mov qword ptr [rbx + 0x710], 0x28
141721c04: mov qword ptr [rbx + 0x718], 0
141721c0f: movups xmmword ptr [rbx + 0x720], xmm0
141721c16: mov dword ptr [rbx + 0x748], 1
141721c20: mov dword ptr [rbx + 0x74c], eax
141721c26: mov byte ptr [rbx + 0x7f], 1
141721c2a: lea rcx, [rbx + 0x93]
141721c31: lea rdx, [rbx + 0x700]
141721c38: call 0x140d4a330
141721c3d: mov byte ptr [rbx + 0x7f], 1
141721c41: call 0x140c16a00
141721c46: test al, al
141721c48: je 0x1417224dd
141721c4e: lea r8, [rip + 0x15a8856]
141721c55: lea rsi, [rip + 0x15a881c]
141721c5c: nop dword ptr [rax]
141721c60: movsx eax, byte ptr [r8 - 1]
141721c65: test eax, eax
141721c67: js 0x141721c80
141721c69: dec r8
141721c6c: cmp eax, 0x5c
141721c6f: jne 0x141721cd8
141721c71: jmp 0x1417226e0
141721c76: nop word ptr cs:[rax + rax]
141721c80: movzx ecx, byte ptr [r8 - 2]
141721c85: cmp cl, 0xc0
141721c88: jge 0x141721cad
141721c8a: movzx edx, byte ptr [r8 - 3]
141721c8f: cmp dl, 0xc0
141721c92: jge 0x141721cb6
141721c94: movzx r9d, byte ptr [r8 - 4]
141721c99: add r8, -4
141721c9d: and r9d, 7
141721ca1: shl r9d, 6
141721ca5: and edx, 0x3f
141721ca8: or edx, r9d
141721cab: jmp 0x141721cbd
141721cad: add r8, -2
141721cb1: and ecx, 0x1f
141721cb4: jmp 0x141721cc5
141721cb6: add r8, -3
141721cba: and edx, 0xf
141721cbd: shl edx, 6
141721cc0: and ecx, 0x3f
141721cc3: or ecx, edx
141721cc5: shl ecx, 6
141721cc8: and al, 0x3f
141721cca: movzx eax, al
141721ccd: or eax, ecx
141721ccf: cmp eax, 0x5c
141721cd2: je 0x1417226e0
141721cd8: cmp eax, 0x2f
141721cdb: je 0x1417226e0
141721ce1: cmp r8, rsi
141721ce4: jne 0x141721c60
141721cea: mov edi, 0x33
141721cef: jmp 0x14172429b
141721cf4: mov qword ptr [rbx + 0x80], r8
141721cfb: mov eax, dword ptr [rcx + 0x10]
141721cfe: mov dword ptr [rbx + 0x1d8], eax
141721d04: mov rax, qword ptr [rcx + 8]
141721d08: mov qword ptr [rbx + 0x1d0], rax
141721d0f: call 0x140c16a00
141721d14: test al, al
141721d16: je 0x141722384
141721d1c: lea r8, [rip + 0x15a8788]
141721d23: lea rsi, [rip + 0x15a874e]
141721d2a: nop word ptr [rax + rax]
141721d30: movsx eax, byte ptr [r8 - 1]
141721d35: test eax, eax
141721d37: js 0x141721d50
141721d39: dec r8
141721d3c: cmp eax, 0x5c
141721d3f: jne 0x141721da8
141721d41: jmp 0x1417224e4
141721d46: nop word ptr cs:[rax + rax]
141721d50: movzx ecx, byte ptr [r8 - 2]
141721d55: cmp cl, 0xc0
141721d58: jge 0x141721d7d
141721d5a: movzx edx, byte ptr [r8 - 3]
141721d5f: cmp dl, 0xc0
141721d62: jge 0x141721d86
141721d64: movzx r9d, byte ptr [r8 - 4]
141721d69: add r8, -4
141721d6d: and r9d, 7
141721d71: shl r9d, 6
141721d75: and edx, 0x3f
141721d78: or edx, r9d
141721d7b: jmp 0x141721d8d
141721d7d: add r8, -2
141721d81: and ecx, 0x1f
141721d84: jmp 0x141721d95
141721d86: add r8, -3
141721d8a: and edx, 0xf
141721d8d: shl edx, 6
141721d90: and ecx, 0x3f
141721d93: or ecx, edx
141721d95: shl ecx, 6
141721d98: and al, 0x3f
141721d9a: movzx eax, al
141721d9d: or eax, ecx
141721d9f: cmp eax, 0x5c
141721da2: je 0x1417224e4
141721da8: cmp eax, 0x2f
141721dab: je 0x1417224e4
141721db1: cmp r8, rsi
141721db4: jne 0x141721d30
141721dba: mov edi, 0x33
141721dbf: jmp 0x141722745
141721dc4: mov r14, r15
141721dc7: mov r15, qword ptr [rcx + 0x20]
141721dcb: mov rax, qword ptr [rcx + 0x18]
141721dcf: mov qword ptr [rbx + 0x150], rax
141721dd6: movups xmm0, xmmword ptr [rcx + 8]
141721dda: movaps xmmword ptr [rbx + 0x140], xmm0
141721de1: mov byte ptr [rbx + 0x8f], 1
141721de8: call 0x140c16a00
141721ded: test al, al
141721def: je 0x1417223a3
141721df5: lea r8, [rip + 0x15a86af]
141721dfc: lea rsi, [rip + 0x15a8675]
141721e03: nop word ptr cs:[rax + rax]
141721e10: movsx eax, byte ptr [r8 - 1]
141721e15: test eax, eax
141721e17: js 0x141721e30
141721e19: dec r8
141721e1c: cmp eax, 0x5c
141721e1f: jne 0x141721e88
141721e21: jmp 0x14172253e
141721e26: nop word ptr cs:[rax + rax]
141721e30: movzx ecx, byte ptr [r8 - 2]
141721e35: cmp cl, 0xc0
141721e38: jge 0x141721e5d
141721e3a: movzx edx, byte ptr [r8 - 3]
141721e3f: cmp dl, 0xc0
141721e42: jge 0x141721e66
141721e44: movzx r9d, byte ptr [r8 - 4]
141721e49: add r8, -4
141721e4d: and r9d, 7
141721e51: shl r9d, 6
141721e55: and edx, 0x3f
141721e58: or edx, r9d
141721e5b: jmp 0x141721e6d
141721e5d: add r8, -2
141721e61: and ecx, 0x1f
141721e64: jmp 0x141721e75
141721e66: add r8, -3
141721e6a: and edx, 0xf
141721e6d: shl edx, 6
141721e70: and ecx, 0x3f
141721e73: or ecx, edx
141721e75: shl ecx, 6
141721e78: and al, 0x3f
141721e7a: movzx eax, al
141721e7d: or eax, ecx
141721e7f: cmp eax, 0x5c
141721e82: je 0x14172253e
141721e88: cmp eax, 0x2f
141721e8b: je 0x14172253e
141721e91: cmp r8, rsi
141721e94: jne 0x141721e10
141721e9a: mov edi, 0x33
141721e9f: jmp 0x141722f04
141721ea4: movzx eax, byte ptr [rcx + 0x18]
141721ea8: mov byte ptr [rbx + 0xc6], al
141721eae: mov rax, qword ptr [rcx + 8]
141721eb2: mov qword ptr [rbx + 0x98], rax
141721eb9: mov rax, qword ptr [rcx + 0x10]
141721ebd: mov qword ptr [rbx + 0x80], rax
141721ec4: call 0x140c16a00
141721ec9: test al, al
141721ecb: je 0x1417223aa
141721ed1: lea r8, [rip + 0x15a85d3]
141721ed8: lea rsi, [rip + 0x15a8599]
141721edf: nop 
141721ee0: movsx eax, byte ptr [r8 - 1]
141721ee5: test eax, eax
141721ee7: js 0x141721f00
141721ee9: dec r8
141721eec: cmp eax, 0x5c
141721eef: jne 0x141721f58
141721ef1: jmp 0x141722595
141721ef6: nop word ptr cs:[rax + rax]
141721f00: movzx ecx, byte ptr [r8 - 2]
141721f05: cmp cl, 0xc0
141721f08: jge 0x141721f2d
141721f0a: movzx edx, byte ptr [r8 - 3]
141721f0f: cmp dl, 0xc0
141721f12: jge 0x141721f36
141721f14: movzx r9d, byte ptr [r8 - 4]
141721f19: add r8, -4
141721f1d: and r9d, 7
141721f21: shl r9d, 6
141721f25: and edx, 0x3f
141721f28: or edx, r9d
141721f2b: jmp 0x141721f3d
141721f2d: add r8, -2
141721f31: and ecx, 0x1f
141721f34: jmp 0x141721f45
141721f36: add r8, -3
141721f3a: and edx, 0xf
141721f3d: shl edx, 6
141721f40: and ecx, 0x3f
141721f43: or ecx, edx
141721f45: shl ecx, 6
141721f48: and al, 0x3f
141721f4a: movzx eax, al
141721f4d: or eax, ecx
141721f4f: cmp eax, 0x5c
141721f52: je 0x141722595
141721f58: cmp eax, 0x2f
141721f5b: je 0x141722595
141721f61: cmp r8, rsi
141721f64: jne 0x141721ee0
141721f6a: mov edi, 0x33
141721f6f: jmp 0x141723079
141721f74: movups xmm0, xmmword ptr [rcx + 8]
141721f78: movups xmmword ptr [rbx + 0x480], xmm0
141721f7f: call 0x140c16a00
141721f84: mov r14d, eax
141721f87: test al, al
141721f89: je 0x1417224b7
141721f8f: lea r8, [rip + 0x15a8515]
141721f96: lea rsi, [rip + 0x15a84db]
141721f9d: nop dword ptr [rax]
141721fa0: movsx eax, byte ptr [r8 - 1]
141721fa5: test eax, eax
141721fa7: js 0x141721fc0
141721fa9: dec r8
141721fac: cmp eax, 0x5c
141721faf: jne 0x141722018
141721fb1: jmp 0x1417225e5
141721fb6: nop word ptr cs:[rax + rax]
141721fc0: movzx ecx, byte ptr [r8 - 2]
141721fc5: cmp cl, 0xc0
141721fc8: jge 0x141721fed
141721fca: movzx edx, byte ptr [r8 - 3]
141721fcf: cmp dl, 0xc0
141721fd2: jge 0x141721ff6
141721fd4: movzx r9d, byte ptr [r8 - 4]
141721fd9: add r8, -4
141721fdd: and r9d, 7
141721fe1: shl r9d, 6
141721fe5: and edx, 0x3f
141721fe8: or edx, r9d
141721feb: jmp 0x141721ffd
141721fed: add r8, -2
141721ff1: and ecx, 0x1f
141721ff4: jmp 0x141722005
141721ff6: add r8, -3
141721ffa: and edx, 0xf
141721ffd: shl edx, 6
141722000: and ecx, 0x3f
141722003: or ecx, edx
141722005: shl ecx, 6
141722008: and al, 0x3f
14172200a: movzx eax, al
14172200d: or eax, ecx
14172200f: cmp eax, 0x5c
141722012: je 0x1417225e5
141722018: cmp eax, 0x2f
14172201b: je 0x1417225e5
141722021: cmp r8, rsi
141722024: jne 0x141721fa0
14172202a: mov edi, 0x33
14172202f: jmp 0x1417233ce
141722034: mov rax, qword ptr [rip + 0x1f0a36d]
14172203b: mov rax, qword ptr [rax]
14172203e: cmp rax, 2
141722042: jbe 0x1417220fe
141722048: lea rcx, [rip + 0x15a8ac9]
14172204f: call 0x14067dd80
141722054: movups xmm0, xmmword ptr [rax]
141722057: mov eax, dword ptr [rax + 0x10]
14172205a: mov qword ptr [rbx + 0x730], 3
141722065: lea rcx, [rip + 0x15a84f4]
14172206c: mov qword ptr [rbx + 0x738], rcx
141722073: mov qword ptr [rbx + 0x740], 0x28
14172207e: lea rdx, [rip + 0x15a8a83]
141722085: mov qword ptr [rbx + 0x750], rdx
14172208c: mov qword ptr [rbx + 0x758], 1
141722097: mov qword ptr [rbx + 0x760], 8
1417220a2: mov qword ptr [rbx + 0x700], 0
1417220ad: xorps xmm1, xmm1
1417220b0: movups xmmword ptr [rbx + 0x768], xmm1
1417220b7: mov qword ptr [rbx + 0x708], rcx
1417220be: mov qword ptr [rbx + 0x710], 0x28
1417220c9: mov qword ptr [rbx + 0x718], 0
1417220d4: movups xmmword ptr [rbx + 0x720], xmm0
1417220db: mov dword ptr [rbx + 0x748], 1
1417220e5: mov dword ptr [rbx + 0x74c], eax
1417220eb: lea rcx, [rbx + 0x93]
1417220f2: lea rdx, [rbx + 0x700]
1417220f9: call 0x140d4a330
1417220fe: call 0x140c16a00
141722103: mov r14d, eax
141722106: test al, al
141722108: je 0x14172239c
14172210e: lea r8, [rip + 0x15a8396]
141722115: lea rsi, [rip + 0x15a835c]
14172211c: nop dword ptr [rax]
141722120: movsx eax, byte ptr [r8 - 1]
141722125: test eax, eax
141722127: js 0x141722140
141722129: dec r8
14172212c: cmp eax, 0x5c
14172212f: jne 0x141722198
141722131: jmp 0x141722511
141722136: nop word ptr cs:[rax + rax]
141722140: movzx ecx, byte ptr [r8 - 2]
141722145: cmp cl, 0xc0
141722148: jge 0x14172216d
14172214a: movzx edx, byte ptr [r8 - 3]
14172214f: cmp dl, 0xc0
141722152: jge 0x141722176
141722154: movzx r9d, byte ptr [r8 - 4]
141722159: add r8, -4
14172215d: and r9d, 7
141722161: shl r9d, 6
141722165: and edx, 0x3f
141722168: or edx, r9d
14172216b: jmp 0x14172217d
14172216d: add r8, -2
141722171: and ecx, 0x1f
141722174: jmp 0x141722185
141722176: add r8, -3
14172217a: and edx, 0xf
14172217d: shl edx, 6
141722180: and ecx, 0x3f
141722183: or ecx, edx
141722185: shl ecx, 6
141722188: and al, 0x3f
14172218a: movzx eax, al
14172218d: or eax, ecx
14172218f: cmp eax, 0x5c
141722192: je 0x141722511
141722198: cmp eax, 0x2f
14172219b: je 0x141722511
1417221a1: cmp r8, rsi
1417221a4: jne 0x141722120
1417221aa: mov edi, 0x33
1417221af: jmp 0x141722c9c
1417221b4: movups xmm0, xmmword ptr [rcx + 0x30]
1417221b8: movaps xmmword ptr [rbx + 0x120], xmm0
1417221bf: mov rdx, qword ptr [rcx + 0x10]
1417221c3: mov qword ptr [rbx + 0x98], rdx
1417221ca: mov rdx, qword ptr [rcx + 0x18]
1417221ce: mov qword ptr [rbx + 0xa8], rdx
1417221d5: movdqu xmm0, xmmword ptr [rcx + 0x20]
1417221da: movdqa xmmword ptr [rbx + 0x170], xmm0
1417221e2: mov rcx, qword ptr [rcx + 8]
1417221e6: mov qword ptr [rbx + 0xd8], rax
1417221ed: mov qword ptr [rbx + 0xe0], rcx
1417221f4: mov byte ptr [rbx + 0x7e], 1
1417221f8: call 0x140c16a00
1417221fd: test al, al
1417221ff: je 0x1417224be
141722205: lea r8, [rip + 0x15a829f]
14172220c: lea rsi, [rip + 0x15a8265]
141722213: nop word ptr cs:[rax + rax]
141722220: movsx eax, byte ptr [r8 - 1]
141722225: test eax, eax
141722227: js 0x141722240
141722229: dec r8
14172222c: cmp eax, 0x5c
14172222f: jne 0x141722298
141722231: jmp 0x141722635
141722236: nop word ptr cs:[rax + rax]
141722240: movzx ecx, byte ptr [r8 - 2]
141722245: cmp cl, 0xc0
141722248: jge 0x14172226d
14172224a: movzx edx, byte ptr [r8 - 3]
14172224f: cmp dl, 0xc0
141722252: jge 0x141722276
141722254: movzx r9d, byte ptr [r8 - 4]
141722259: add r8, -4
14172225d: and r9d, 7
141722261: shl r9d, 6
141722265: and edx, 0x3f
141722268: or edx, r9d
14172226b: jmp 0x14172227d
14172226d: add r8, -2
141722271: and ecx, 0x1f
141722274: jmp 0x141722285
141722276: add r8, -3
14172227a: and edx, 0xf
14172227d: shl edx, 6
141722280: and ecx, 0x3f
141722283: or ecx, edx
141722285: shl ecx, 6
141722288: and al, 0x3f
14172228a: movzx eax, al
14172228d: or eax, ecx
14172228f: cmp eax, 0x5c
141722292: je 0x141722635
141722298: cmp eax, 0x2f
14172229b: je 0x141722635
1417222a1: cmp r8, rsi
1417222a4: jne 0x141722220
1417222aa: mov edi, 0x33
1417222af: jmp 0x141723587
1417222b4: mov rax, qword ptr [rcx + 8]
1417222b8: mov qword ptr [rbx + 0x80], rax
1417222bf: mov rax, qword ptr [rcx + 0x10]
1417222c3: mov qword ptr [rbx + 0xf0], rax
1417222ca: mov byte ptr [rbx + 0x90], 1
1417222d1: call 0x140c16a00
1417222d6: mov r14d, eax
1417222d9: test al, al
1417222db: je 0x1417224d6
1417222e1: lea r8, [rip + 0x15a81c3]
1417222e8: lea rsi, [rip + 0x15a8189]
1417222ef: nop 
1417222f0: movsx eax, byte ptr [r8 - 1]
1417222f5: test eax, eax
1417222f7: js 0x141722310
1417222f9: dec r8
1417222fc: cmp eax, 0x5c
1417222ff: jne 0x141722368
141722301: jmp 0x141722689
141722306: nop word ptr cs:[rax + rax]
141722310: movzx ecx, byte ptr [r8 - 2]
141722315: cmp cl, 0xc0
141722318: jge 0x14172233d
14172231a: movzx edx, byte ptr [r8 - 3]
14172231f: cmp dl, 0xc0
141722322: jge 0x141722346
141722324: movzx r9d, byte ptr [r8 - 4]
141722329: add r8, -4
14172232d: and r9d, 7
141722331: shl r9d, 6
141722335: and edx, 0x3f
141722338: or edx, r9d
14172233b: jmp 0x14172234d
14172233d: add r8, -2
141722341: and ecx, 0x1f
141722344: jmp 0x141722355
141722346: add r8, -3
14172234a: and edx, 0xf
14172234d: shl edx, 6
141722350: and ecx, 0x3f
141722353: or ecx, edx
141722355: shl ecx, 6
141722358: and al, 0x3f
14172235a: movzx eax, al
14172235d: or eax, ecx
14172235f: cmp eax, 0x5c
141722362: je 0x141722689
141722368: cmp eax, 0x2f
14172236b: je 0x141722689
141722371: cmp r8, rsi
141722374: jne 0x1417222f0
14172237a: mov edi, 0x33
14172237f: jmp 0x141723f57
141722384: xor eax, eax
141722386: mov qword ptr [rbx + 0x120], rax
14172238d: cmp dword ptr [r15], 3
141722391: jne 0x1417227cd
141722397: jmp 0x14172648a
14172239c: xor eax, eax
14172239e: jmp 0x141722d16
1417223a3: xor eax, eax
1417223a5: jmp 0x141722f9b
1417223aa: xor eax, eax
1417223ac: mov qword ptr [rbx + 0x480], rax
1417223b3: cmp dword ptr [r15], 3
1417223b7: jne 0x14172311a
1417223bd: mov rax, qword ptr [rip + 0x1f09fe4]
1417223c4: mov rax, qword ptr [rax]
1417223c7: cmp rax, 2
1417223cb: jb 0x14172328e
1417223d1: lea rax, [rbx + 0xc6]
1417223d8: mov qword ptr [rbx + 0x200], rax
1417223df: lea rax, [rip + 0xa53a]
1417223e6: mov qword ptr [rbx + 0x208], rax
1417223ed: lea rcx, [rip + 0x15a8864]
1417223f4: call 0x14067dd80
1417223f9: movups xmm0, xmmword ptr [rax]
1417223fc: mov eax, dword ptr [rax + 0x10]
1417223ff: mov qword ptr [rbx + 0x730], 2
14172240a: lea rcx, [rip + 0x15a814f]
141722411: mov qword ptr [rbx + 0x738], rcx
141722418: mov qword ptr [rbx + 0x740], 0x28
141722423: lea rdx, [rip + 0x15a880e]
14172242a: mov qword ptr [rbx + 0x750], rdx
141722431: mov qword ptr [rbx + 0x758], 2
14172243c: lea rdx, [rbx + 0x200]
141722443: mov qword ptr [rbx + 0x760], rdx
14172244a: mov qword ptr [rbx + 0x768], 1
141722455: mov qword ptr [rbx + 0x770], 0
141722460: mov qword ptr [rbx + 0x700], 0
14172246b: mov qword ptr [rbx + 0x708], rcx
141722472: mov qword ptr [rbx + 0x710], 0x28
14172247d: mov qword ptr [rbx + 0x718], 0
141722488: movups xmmword ptr [rbx + 0x720], xmm0
14172248f: mov dword ptr [rbx + 0x748], 1
141722499: mov dword ptr [rbx + 0x74c], eax
14172249f: lea rcx, [rbx + 0x93]
1417224a6: lea rdx, [rbx + 0x700]
1417224ad: call 0x140d4a330
1417224b2: jmp 0x14172328e
1417224b7: xor eax, eax
1417224b9: jmp 0x141723461
1417224be: xor eax, eax
1417224c0: mov qword ptr [rbx + 0x190], rax
1417224c7: cmp dword ptr [r15], 3
1417224cb: jne 0x14172362c
1417224d1: jmp 0x14172374d
1417224d6: xor eax, eax
1417224d8: jmp 0x141723ff1
1417224dd: xor eax, eax
1417224df: jmp 0x14172432f
1417224e4: lea rsi, [rip + 0x15a7f8d]
1417224eb: sub r8, rsi
1417224ee: inc r8
1417224f1: je 0x14172273a
1417224f7: cmp r8, 0x33
1417224fb: jae 0x141722734
141722501: cmp byte ptr [r8 + rsi], 0xbf
141722506: jg 0x14172273a
14172250c: jmp 0x141727425
141722511: lea rsi, [rip + 0x15a7f60]
141722518: sub r8, rsi
14172251b: inc r8
14172251e: je 0x141722c91
141722524: cmp r8, 0x33
141722528: jae 0x141722c8b
14172252e: cmp byte ptr [r8 + rsi], 0xbf
141722533: jg 0x141722c91
141722539: jmp 0x141727425
14172253e: lea rsi, [rip + 0x15a7f33]
141722545: sub r8, rsi
141722548: inc r8
14172254b: je 0x141722ef9
141722551: cmp r8, 0x33
141722555: jae 0x141722ef3
14172255b: cmp byte ptr [r8 + rsi], 0xbf
141722560: jg 0x141722ef9
141722566: mov byte ptr [rbx + 0x8f], 1
14172256d: lea rax, [rip + 0x15a7e9c]
141722574: mov qword ptr [rsp + 0x20], rax
141722579: lea rcx, [rip + 0x15a7ef8]
141722580: mov edx, 0x33
141722585: mov r9d, 0x33
14172258b: call 0x1428d9940
141722590: jmp 0x14172746a
141722595: lea rsi, [rip + 0x15a7edc]
14172259c: sub r8, rsi
14172259f: inc r8
1417225a2: je 0x14172306e
1417225a8: cmp r8, 0x33
1417225ac: jae 0x141723068
1417225b2: cmp byte ptr [r8 + rsi], 0xbf
1417225b7: jg 0x14172306e
1417225bd: lea rax, [rip + 0x15a7e4c]
1417225c4: mov qword ptr [rsp + 0x20], rax
1417225c9: lea rcx, [rip + 0x15a7ea8]
1417225d0: mov edx, 0x33
1417225d5: mov r9d, 0x33
1417225db: call 0x1428d9940
1417225e0: jmp 0x14172746a
1417225e5: lea rsi, [rip + 0x15a7e8c]
1417225ec: sub r8, rsi
1417225ef: inc r8
1417225f2: je 0x1417233c3
1417225f8: cmp r8, 0x33
1417225fc: jae 0x1417233bd
141722602: cmp byte ptr [r8 + rsi], 0xbf
141722607: jg 0x1417233c3
14172260d: lea rax, [rip + 0x15a7dfc]
141722614: mov qword ptr [rsp + 0x20], rax
141722619: lea rcx, [rip + 0x15a7e58]
141722620: mov edx, 0x33
141722625: mov r9d, 0x33
14172262b: call 0x1428d9940
141722630: jmp 0x14172746a
141722635: lea rsi, [rip + 0x15a7e3c]
14172263c: sub r8, rsi
14172263f: inc r8
141722642: je 0x14172357c
141722648: cmp r8, 0x33
14172264c: jae 0x141723576
141722652: cmp byte ptr [r8 + rsi], 0xbf
141722657: jg 0x14172357c
14172265d: mov byte ptr [rbx + 0x7e], 1
141722661: lea rax, [rip + 0x15a7da8]
141722668: mov qword ptr [rsp + 0x20], rax
14172266d: lea rcx, [rip + 0x15a7e04]
141722674: mov edx, 0x33
141722679: mov r9d, 0x33
14172267f: call 0x1428d9940
141722684: jmp 0x14172746a
141722689: lea rsi, [rip + 0x15a7de8]
141722690: sub r8, rsi
141722693: inc r8
141722696: je 0x141723f4c
14172269c: cmp r8, 0x33
1417226a0: jae 0x141723f46
1417226a6: cmp byte ptr [r8 + rsi], 0xbf
1417226ab: jg 0x141723f4c
1417226b1: mov byte ptr [rbx + 0x90], 1
1417226b8: lea rax, [rip + 0x15a7d51]
1417226bf: mov qword ptr [rsp + 0x20], rax
1417226c4: lea rcx, [rip + 0x15a7dad]
1417226cb: mov edx, 0x33
1417226d0: mov r9d, 0x33
1417226d6: call 0x1428d9940
1417226db: jmp 0x14172746a
1417226e0: lea rsi, [rip + 0x15a7d91]
1417226e7: sub r8, rsi
1417226ea: inc r8
1417226ed: je 0x141724290
1417226f3: cmp r8, 0x33
1417226f7: jae 0x14172428a
1417226fd: cmp byte ptr [r8 + rsi], 0xbf
141722702: jg 0x141724290
141722708: mov byte ptr [rbx + 0x7f], 1
14172270c: lea rax, [rip + 0x15a7cfd]
141722713: mov qword ptr [rsp + 0x20], rax
141722718: lea rcx, [rip + 0x15a7d59]
14172271f: mov edx, 0x33
141722724: mov r9d, 0x33
14172272a: call 0x1428d9940
14172272f: jmp 0x14172746a
141722734: jne 0x141727425
14172273a: mov edi, 0x33
14172273f: sub rdi, r8
141722742: add rsi, r8
141722745: call 0x140c18130
14172274a: mov rcx, rax
14172274d: mov rax, qword ptr [rax]
141722750: cmp rax, 1
141722754: jne 0x141726c47
14172275a: add rcx, 8
14172275e: cmp qword ptr [rcx], 0
141722762: jne 0x141726e07
141722768: mov qword ptr [rcx], 0xffffffffffffffff
14172276f: mov qword ptr [rbx + 0x98], rcx
141722776: add rcx, 8
14172277a: mov qword ptr [rsp + 0x20], rdi
14172277f: mov qword ptr [rsp + 0x30], 0
141722788: mov qword ptr [rsp + 0x28], 1
141722791: lea rdx, [rip + 0x15a84d8]
141722798: mov r8d, 0x18
14172279e: mov r9, rsi
1417227a1: call 0x140c16d40
1417227a6: mov rcx, qword ptr [rbx + 0x98]
1417227ad: inc qword ptr [rcx]
1417227b0: mov qword ptr [rbx + 0x128], rax
1417227b7: mov eax, 1
1417227bc: mov qword ptr [rbx + 0x120], rax
1417227c3: cmp dword ptr [r15], 3
1417227c7: je 0x14172648a
1417227cd: lea rsi, [r15 + 0x298]
1417227d4: lea rcx, [rbx + 0x140]
1417227db: mov rdx, rsi
1417227de: call 0x140b46180
1417227e3: lea rcx, [rbx + 0x700]
1417227ea: mov rdx, rsi
1417227ed: call 0x140b46000
1417227f2: lea rcx, [rbx + 0x700]
1417227f9: call 0x140d76ed0
1417227fe: mov r8, rdx
141722801: mov qword ptr [rsp + 0x20], 0xc
14172280a: lea r9, [rip + 0x15a7e97]
141722811: lea rcx, [rbx + 0x15a0]
141722818: mov rdx, rax
14172281b: call 0x140d62810
141722820: mov r12, r15
141722823: mov rdx, qword ptr [rbx + 0x700]
14172282a: test rdx, rdx
14172282d: je 0x141722841
14172282f: mov rcx, qword ptr [rbx + 0x708]
141722836: mov r8d, 1
14172283c: call 0x140613c20
141722841: mov byte ptr [rbx + 0xb6], 1
141722848: lea rcx, [rbx + 0x140]
14172284f: call 0x140d76ed0
141722854: mov r14, rax
141722857: mov r15, rdx
14172285a: lea rax, [rbx + 0x1d8]
141722861: lea rcx, [rip + 0x15a7e58]
141722868: mov qword ptr [rbx + 0x200], rcx
14172286f: lea rcx, [rip - 0x16189e6]
141722876: mov qword ptr [rbx + 0x208], rcx
14172287d: mov qword ptr [rbx + 0x210], rax
141722884: lea rax, [rip - 0x99532b]
14172288b: mov qword ptr [rbx + 0x218], rax
141722892: lea rdi, [rip + 0x15a7e3f]
141722899: mov qword ptr [rbx + 0x700], rdi
1417228a0: mov qword ptr [rbx + 0x708], 3
1417228ab: lea r13, [rbx + 0x200]
1417228b2: mov qword ptr [rbx + 0x710], r13
1417228b9: mov qword ptr [rbx + 0x718], 2
1417228c4: mov qword ptr [rbx + 0x720], 0
1417228cf: mov byte ptr [rbx + 0xb6], 1
1417228d6: lea rcx, [rbx + 0xd8]
1417228dd: lea rdx, [rbx + 0x700]
1417228e4: call 0x140d79f90
1417228e9: mov rax, qword ptr [rbx + 0xd8]
1417228f0: mov qword ptr [rbx + 0x98], rax
1417228f7: mov r9, qword ptr [rbx + 0xe0]
1417228fe: mov rax, qword ptr [rbx + 0xe8]
141722905: mov qword ptr [rsp + 0x20], rax
14172290a: lea rsi, [rbx + 0x1340]
141722911: mov rcx, rsi
141722914: mov rdx, r14
141722917: mov r8, r15
14172291a: mov qword ptr [rbx + 0xa8], r9
141722921: call 0x140d62810
141722926: mov rdx, qword ptr [rbx + 0x98]
14172292d: test rdx, rdx
141722930: mov r15, r12
141722933: je 0x141722947
141722935: mov r8d, 1
14172293b: mov rcx, qword ptr [rbx + 0xa8]
141722942: call 0x140613c20
141722947: mov byte ptr [rbx + 0x7c], 1
14172294b: mov rcx, r15
14172294e: call 0x14171f7f0
141722953: mov r14d, eax
141722956: test al, al
141722958: je 0x1417229f7
14172295e: movups xmm0, xmmword ptr [rbx + 0x15a0]
141722965: movups xmm1, xmmword ptr [rbx + 0x15b0]
14172296c: movaps xmmword ptr [rbx + 0x710], xmm1
141722973: movaps xmmword ptr [rbx + 0x700], xmm0
14172297a: lea rcx, [rbx + 0x700]
141722981: call 0x140d76ed0
141722986: mov r15, rax
141722989: mov r12, rdx
14172298c: lea rcx, [rbx + 0x1340]
141722993: call 0x140d76ed0
141722998: mov r9, rdx
14172299b: mov rcx, r15
14172299e: mov rdx, r12
1417229a1: mov r8, rax
1417229a4: call 0x140d6b070
1417229a9: mov r15, rax
1417229ac: mov rax, qword ptr [rbx + 0x700]
1417229b3: test rax, rax
1417229b6: je 0x1417229d3
1417229b8: mov rcx, qword ptr [rbx + 0x708]
1417229bf: mov r8d, 1
1417229c5: mov r12, rdx
1417229c8: mov rdx, rax
1417229cb: call 0x140613c20
1417229d0: mov rdx, r12
1417229d3: mov byte ptr [rbx + 0x7c], 0
1417229d7: lea r8, [rbx + 0x1340]
1417229de: mov rcx, r15
1417229e1: call 0x1415a4f40
1417229e6: test rax, rax
1417229e9: je 0x141725849
1417229ef: mov r15, rdx
1417229f2: jmp 0x141722b50
1417229f7: mov rax, qword ptr [rip + 0x1f099aa]
1417229fe: mov rax, qword ptr [rax]
141722a01: test rax, rax
141722a04: je 0x141722a96
141722a0a: lea rax, [rip + 0x15a7d1f]
141722a11: mov qword ptr [rbx + 0x700], rax
141722a18: mov qword ptr [rbx + 0x708], 1
141722a23: mov qword ptr [rbx + 0x710], 8
141722a2e: xorps xmm0, xmm0
141722a31: movups xmmword ptr [rbx + 0x718], xmm0
141722a38: mov byte ptr [rbx + 0x7c], 1
141722a3c: lea rcx, [rip + 0x15a7cfd]
141722a43: call 0x14067dd80
141722a48: lea rcx, [rip + 0x15a7b11]
141722a4f: mov qword ptr [rbx + 0x200], rcx
141722a56: mov qword ptr [rbx + 0x208], 0x28
141722a61: mov qword ptr [rbx + 0x210], rcx
141722a68: mov qword ptr [rbx + 0x218], 0x28
141722a73: mov qword ptr [rbx + 0x220], rax
141722a7a: mov byte ptr [rbx + 0x7c], 1
141722a7e: lea rcx, [rbx + 0x700]
141722a85: lea r8, [rbx + 0x200]
141722a8c: mov edx, 1
141722a91: call 0x1404b1430
141722a96: mov rax, r15
141722a99: mov r15, qword ptr [r15 + 0x38]
141722a9d: mov r12, qword ptr [rax + 0x40]
141722aa1: mov byte ptr [rbx + 0x7c], 1
141722aa5: lea rcx, [rbx + 0x1340]
141722aac: call 0x140d76ed0
141722ab1: mov byte ptr [rbx + 0x7c], 1
141722ab5: mov rcx, rax
141722ab8: mov r8, r15
141722abb: mov r9, r12
141722abe: call 0x140d59430
141722ac3: mov r15, rax
141722ac6: test rax, rax
141722ac9: je 0x141725844
141722acf: mov qword ptr [rbx + 0x480], r15
141722ad6: mov byte ptr [rbx + 0xca], 1
141722add: lea rcx, [rbx + 0x700]
141722ae4: call 0x140d56700
141722ae9: movups xmm0, xmmword ptr [rbx + 0x700]
141722af0: movups xmm1, xmmword ptr [rbx + 0x710]
141722af7: movups xmm2, xmmword ptr [rbx + 0x720]
141722afe: movaps xmmword ptr [rbx + 0x220], xmm2
141722b05: movaps xmmword ptr [rbx + 0x210], xmm1
141722b0c: movaps xmmword ptr [rbx + 0x200], xmm0
141722b13: lea rax, [rip + 0x15a7c3e]
141722b1a: mov qword ptr [rbx + 0x700], rax
141722b21: mov qword ptr [rbx + 0x708], 0x24
141722b2c: mov qword ptr [rbx + 0x710], r15
141722b33: mov byte ptr [rbx + 0xca], 0
141722b3a: lea rcx, [rbx + 0x700]
141722b41: lea rdx, [rbx + 0x200]
141722b48: call 0x1428a8e80
141722b4d: mov r15, rax
141722b50: mov rdx, qword ptr [rbx + 0x1340]
141722b57: test rdx, rdx
141722b5a: je 0x141722b6e
141722b5c: mov rcx, qword ptr [rbx + 0x1348]
141722b63: mov r8d, 1
141722b69: call 0x140613c20
141722b6e: test r14b, r14b
141722b71: jne 0x141722b91
141722b73: mov rdx, qword ptr [rbx + 0x15a0]
141722b7a: test rdx, rdx
141722b7d: je 0x141722b91
141722b7f: mov rcx, qword ptr [rbx + 0x15a8]
141722b86: mov r8d, 1
141722b8c: call 0x140613c20
141722b91: mov rdx, qword ptr [rbx + 0x140]
141722b98: test rdx, rdx
141722b9b: je 0x141722baf
141722b9d: mov rcx, qword ptr [rbx + 0x148]
141722ba4: mov r8d, 1
141722baa: call 0x140613c20
141722baf: test r15, r15
141722bb2: je 0x141726483
141722bb8: mov qword ptr [rbx + 0x1340], r15
141722bbf: mov rax, qword ptr [rip + 0x1f097e2]
141722bc6: mov rax, qword ptr [rax]
141722bc9: test rax, rax
141722bcc: je 0x141722c7a
141722bd2: mov qword ptr [rbx + 0x480], rsi
141722bd9: lea rax, [rip - 0x9d5fc0]
141722be0: mov qword ptr [rbx + 0x488], rax
141722be7: lea rax, [rip + 0x15a80c2]
141722bee: mov qword ptr [rbx + 0x700], rax
141722bf5: mov qword ptr [rbx + 0x708], 1
141722c00: mov qword ptr [rbx + 0x720], 0
141722c0b: lea rax, [rbx + 0x480]
141722c12: mov qword ptr [rbx + 0x710], rax
141722c19: mov qword ptr [rbx + 0x718], 1
141722c24: lea rcx, [rip + 0x15a8095]
141722c2b: call 0x14067dd80
141722c30: lea rcx, [rip + 0x15a7929]
141722c37: mov qword ptr [rbx + 0x200], rcx
141722c3e: mov qword ptr [rbx + 0x208], 0x28
141722c49: mov qword ptr [rbx + 0x210], rcx
141722c50: mov qword ptr [rbx + 0x218], 0x28
141722c5b: mov qword ptr [rbx + 0x220], rax
141722c62: lea rcx, [rbx + 0x700]
141722c69: lea r8, [rbx + 0x200]
141722c70: mov edx, 1
141722c75: call 0x1404b1430
141722c7a: lea rcx, [rbx + 0x1340]
141722c81: call 0x140d4cc30
141722c86: jmp 0x141726483
141722c8b: jne 0x141727425
141722c91: mov edi, 0x33
141722c96: sub rdi, r8
141722c99: add rsi, r8
141722c9c: call 0x140c18130
141722ca1: mov rcx, rax
141722ca4: mov rax, qword ptr [rax]
141722ca7: cmp rax, 1
141722cab: jne 0x141726c61
141722cb1: add rcx, 8
141722cb5: cmp qword ptr [rcx], 0
141722cb9: jne 0x141726e07
141722cbf: mov qword ptr [rcx], 0xffffffffffffffff
141722cc6: mov qword ptr [rbx + 0x80], rcx
141722ccd: add rcx, 8
141722cd1: mov qword ptr [rsp + 0x20], rdi
141722cd6: mov qword ptr [rsp + 0x30], 0
141722cdf: mov qword ptr [rsp + 0x28], 1
141722ce8: lea rdx, [rip + 0x15a7dff]
141722cef: mov r8d, 0x17
141722cf5: mov r9, rsi
141722cf8: call 0x140c16d40
141722cfd: mov rsi, rax
141722d00: mov rax, qword ptr [rbx + 0x80]
141722d07: inc qword ptr [rax]
141722d0a: mov qword ptr [rbx + 0x488], rsi
141722d11: mov eax, 1
141722d16: mov qword ptr [rbx + 0x480], rax
141722d1d: mov rax, qword ptr [r15]
141722d20: mov qword ptr [r15], 3
141722d27: cmp rax, 3
141722d2b: je 0x141722ea8
141722d31: mov byte ptr [rbx + 0x80], r14b
141722d38: add r15, 8
141722d3c: mov qword ptr [rbx + 0x700], rax
141722d43: lea rcx, [rbx + 0x708]
141722d4a: mov r8d, 0x2a0
141722d50: mov rdx, r15
141722d53: call 0x142923c00
141722d58: mov byte ptr [rbx + 0xc9], 1
141722d5f: lea rcx, [rbx + 0x700]
141722d66: call 0x14171f7f0
141722d6b: test al, al
141722d6d: je 0x141722d80
141722d6f: lea rcx, [rbx + 0x710]
141722d76: call 0x14088ed10
141722d7b: jmp 0x141722e06
141722d80: movups xmm0, xmmword ptr [rbx + 0x780]
141722d87: movaps xmmword ptr [rbx + 0x270], xmm0
141722d8e: movups xmm0, xmmword ptr [rbx + 0x770]
141722d95: movaps xmmword ptr [rbx + 0x260], xmm0
141722d9c: movups xmm0, xmmword ptr [rbx + 0x760]
141722da3: movaps xmmword ptr [rbx + 0x250], xmm0
141722daa: movups xmm0, xmmword ptr [rbx + 0x750]
141722db1: movaps xmmword ptr [rbx + 0x240], xmm0
141722db8: movups xmm0, xmmword ptr [rbx + 0x710]
141722dbf: movups xmm1, xmmword ptr [rbx + 0x720]
141722dc6: movups xmm2, xmmword ptr [rbx + 0x730]
141722dcd: movups xmm3, xmmword ptr [rbx + 0x740]
141722dd4: movaps xmmword ptr [rbx + 0x230], xmm3
141722ddb: movaps xmmword ptr [rbx + 0x220], xmm2
141722de2: movaps xmmword ptr [rbx + 0x210], xmm1
141722de9: movaps xmmword ptr [rbx + 0x200], xmm0
141722df0: mov byte ptr [rbx + 0xc9], 0
141722df7: lea rdx, [rbx + 0x200]
141722dfe: mov rcx, r12
141722e01: call 0x14171ff80
141722e06: mov r9, qword ptr [rbx + 0x7b0]
141722e0d: test r9, r9
141722e10: je 0x141722e47
141722e12: lea rax, [r9 + 1]
141722e16: mov ecx, 0x18
141722e1b: mul rcx
141722e1e: add rax, 0xf
141722e22: and rax, 0xfffffffffffffff0
141722e26: add r9, rax
141722e29: add r9, 0x11
141722e2d: je 0x141722e47
141722e2f: mov rcx, qword ptr [rbx + 0x7a8]
141722e36: sub rcx, rax
141722e39: mov r8d, 0x10
141722e3f: mov rdx, r9
141722e42: call 0x140613c20
141722e47: mov rdx, qword ptr [rbx + 0x790]
141722e4e: test rdx, rdx
141722e51: je 0x141722e69
141722e53: mov rcx, qword ptr [rbx + 0x798]
141722e5a: shl rdx, 3
141722e5e: mov r8d, 8
141722e64: call 0x140613c20
141722e69: lea rcx, [rbx + 0x7d8]
141722e70: call 0x14087cfb0
141722e75: lea rcx, [rbx + 0x700]
141722e7c: call 0x140878420
141722e81: lea rcx, [rbx + 0x968]
141722e88: lea rdx, [rbx + 0x988]
141722e8f: mov r8d, 0x20
141722e95: mov r9d, 0x10
141722e9b: call 0x140052a30
141722ea0: movzx r14d, byte ptr [rbx + 0x80]
141722ea8: test r14b, r14b
141722eab: je 0x141726c0b
141722eb1: call 0x140c18130
141722eb6: mov rcx, rax
141722eb9: mov rax, qword ptr [rax]
141722ebc: cmp rax, 1
141722ec0: jne 0x141726c7b
141722ec6: add rcx, 8
141722eca: cmp qword ptr [rcx], 0
141722ece: jne 0x141726e07
141722ed4: mov qword ptr [rcx], 0xffffffffffffffff
141722edb: mov qword ptr [rbx + 0x80], rcx
141722ee2: add rcx, 8
141722ee6: mov rdx, rsi
141722ee9: call 0x140c170f0
141722eee: jmp 0x141726c01
141722ef3: jne 0x141722566
141722ef9: mov edi, 0x33
141722efe: sub rdi, r8
141722f01: add rsi, r8
141722f04: call 0x140c18130
141722f09: mov rcx, rax
141722f0c: mov rax, qword ptr [rax]
141722f0f: test rax, rax
141722f12: je 0x141722f23
141722f14: cmp eax, 1
141722f17: jne 0x141727328
141722f1d: add rcx, 8
141722f21: jmp 0x141722f3d
141722f23: mov byte ptr [rbx + 0x8f], 1
141722f2a: xor edx, edx
141722f2c: call 0x1428f87d0
141722f31: mov rcx, rax
141722f34: test rax, rax
141722f37: je 0x141727328
141722f3d: cmp qword ptr [rcx], 0
141722f41: jne 0x141726f71
141722f47: mov qword ptr [rcx], 0xffffffffffffffff
141722f4e: mov qword ptr [rbx + 0x80], rcx
141722f55: add rcx, 8
141722f59: mov qword ptr [rsp + 0x20], rdi
141722f5e: mov qword ptr [rsp + 0x30], 0
141722f67: mov qword ptr [rsp + 0x28], 1
141722f70: lea rdx, [rip + 0x15a7bb9]
141722f77: mov r8d, 0x1a
141722f7d: mov r9, rsi
141722f80: call 0x140c16d40
141722f85: mov rcx, qword ptr [rbx + 0x80]
141722f8c: inc qword ptr [rcx]
141722f8f: mov qword ptr [rbx + 0xe0], rax
141722f96: mov eax, 1
141722f9b: mov qword ptr [rbx + 0xd8], rax
141722fa2: mov rax, qword ptr [r14]
141722fa5: xor r13d, r13d
141722fa8: cmp rax, 3
141722fac: sete byte ptr [rbx + 0xd0]
141722fb3: cmovne r13, r14
141722fb7: cmp eax, 3
141722fba: je 0x141727148
141722fc0: mov rax, qword ptr [rbx + 0x150]
141722fc7: mov qword ptr [rbx + 0x15b0], rax
141722fce: movaps xmm0, xmmword ptr [rbx + 0x140]
141722fd5: movaps xmmword ptr [rbx + 0x15a0], xmm0
141722fdc: mov byte ptr [rbx + 0x7d], 1
141722fe0: call 0x140c16a00
141722fe5: test al, al
141722fe7: je 0x141723061
141722fe9: mov byte ptr [rbx + 0x7d], 1
141722fed: lea rdi, [rip + 0x15a72d0]
141722ff4: mov esi, 0x48
141722ff9: mov edx, 0x48
141722ffe: mov rcx, rdi
141723001: call 0x14171e730
141723006: cmp rax, 1
14172300a: jne 0x1417264f1
141723010: mov r9, rdx
141723013: test rdx, rdx
141723016: je 0x141725c1f
14172301c: cmp r9, 0x48
141723020: jae 0x141725c19
141723026: lea rax, [rip + 0x15a7297]
14172302d: cmp byte ptr [r9 + rax], 0xbf
141723032: jg 0x141725c1f
141723038: mov byte ptr [rbx + 0x7d], 1
14172303c: lea rax, [rip + 0x15a73ed]
141723043: mov qword ptr [rsp + 0x20], rax
141723048: lea rcx, [rip + 0x15a7275]
14172304f: mov edx, 0x48
141723054: xor r8d, r8d
141723057: call 0x1428d9940
14172305c: jmp 0x14172746a
141723061: xor eax, eax
141723063: jmp 0x141726669
141723068: jne 0x1417225bd
14172306e: mov edi, 0x33
141723073: sub rdi, r8
141723076: add rsi, r8
141723079: call 0x140c18130
14172307e: mov rcx, rax
141723081: mov rax, qword ptr [rax]
141723084: test rax, rax
141723087: je 0x141723098
141723089: cmp eax, 1
14172308c: jne 0x141727340
141723092: add rcx, 8
141723096: jmp 0x1417230ab
141723098: xor edx, edx
14172309a: call 0x1428f87d0
14172309f: mov rcx, rax
1417230a2: test rax, rax
1417230a5: je 0x141727340
1417230ab: cmp qword ptr [rcx], 0
1417230af: jne 0x141726f89
1417230b5: mov qword ptr [rcx], 0xffffffffffffffff
1417230bc: mov qword ptr [rbx + 0xa8], rcx
1417230c3: add rcx, 8
1417230c7: mov qword ptr [rsp + 0x20], rdi
1417230cc: mov qword ptr [rsp + 0x30], 0
1417230d5: mov qword ptr [rsp + 0x28], 1
1417230de: lea rdx, [rip + 0x15a7ad3]
1417230e5: mov r8d, 0x1b
1417230eb: mov r9, rsi
1417230ee: call 0x140c16d40
1417230f3: mov rcx, qword ptr [rbx + 0xa8]
1417230fa: inc qword ptr [rcx]
1417230fd: mov qword ptr [rbx + 0x488], rax
141723104: mov eax, 1
141723109: mov qword ptr [rbx + 0x480], rax
141723110: cmp dword ptr [r15], 3
141723114: je 0x1417223bd
14172311a: lea rcx, [rbx + 0x700]
141723121: mov rdx, qword ptr [rbx + 0x98]
141723128: mov rax, qword ptr [rbx + 0x80]
14172312f: call qword ptr [rax + 0x18]
141723132: mov rcx, qword ptr [rbx + 0x700]
141723139: mov rax, rcx
14172313c: neg rax
14172313f: jo 0x1417273a4
141723145: movzx edx, byte ptr [rbx + 0xc6]
14172314c: lea eax, [rdx - 3]
14172314f: xorps xmm0, xmm0
141723152: cmp al, 3
141723154: mov qword ptr [rbx + 0xa8], rcx
14172315b: jb 0x14172322b
141723161: xorps xmm1, xmm1
141723164: mov r8, qword ptr [rbx + 0x708]
14172316b: mov rsi, qword ptr [rbx + 0x710]
141723172: ucomiss xmm0, xmm1
141723175: jae 0x14172324a
14172317b: movzx eax, dl
14172317e: lea rax, [rax + rax*4]
141723182: mov rcx, r15
141723185: lea r15, [r15 + rax*8]
141723189: add r15, 0xd8
141723190: mov rdx, qword ptr [rcx + rax*8 + 0xd8]
141723198: mov rax, rdx
14172319b: neg rax
14172319e: jno 0x14172331a
1417231a4: movss dword ptr [rbx + 0xd0], xmm1
1417231ac: mov qword ptr [rbx + 0xb8], r8
1417231b3: call 0x140d635e0
1417231b8: mov rdi, rax
1417231bb: mov r14d, edx
1417231be: movss xmm0, dword ptr [rbx + 0xd0]
1417231c6: call 0x14171eb50
1417231cb: mov r9d, edx
1417231ce: mov rcx, rdi
1417231d1: mov edx, r14d
1417231d4: mov r8, rax
1417231d7: call 0x140d63a20
1417231dc: mov r9, qword ptr [r15]
1417231df: mov rcx, r9
1417231e2: neg rcx
1417231e5: jo 0x141723205
1417231e7: jae 0x141723205
1417231e9: mov rcx, qword ptr [r15 + 8]
1417231ed: mov r8d, 1
1417231f3: mov edi, edx
1417231f5: mov rdx, r9
1417231f8: mov r14, rax
1417231fb: call 0x140613c20
141723200: mov edx, edi
141723202: mov rax, r14
141723205: mov rcx, qword ptr [rbx + 0xa8]
14172320c: mov qword ptr [r15], rcx
14172320f: mov rcx, qword ptr [rbx + 0xb8]
141723216: mov qword ptr [r15 + 8], rcx
14172321a: mov qword ptr [r15 + 0x10], rsi
14172321e: mov qword ptr [r15 + 0x18], rax
141723222: mov dword ptr [r15 + 0x20], edx
141723226: jmp 0x141723343
14172322b: movss xmm1, dword ptr [rip + 0x12a9ec5]
141723233: mov r8, qword ptr [rbx + 0x708]
14172323a: mov rsi, qword ptr [rbx + 0x710]
141723241: ucomiss xmm0, xmm1
141723244: jb 0x14172317b
14172324a: movss dword ptr [rbx + 0xd0], xmm1
141723252: mov rcx, r15
141723255: mov qword ptr [rbx + 0xb8], r8
14172325c: mov r9, rsi
14172325f: call 0x14171ece0
141723264: add r15, 0x10
141723268: mov rcx, r15
14172326b: call 0x1409193d0
141723270: mov rdx, qword ptr [rbx + 0xa8]
141723277: test rdx, rdx
14172327a: je 0x14172328e
14172327c: mov r8d, 1
141723282: mov rcx, qword ptr [rbx + 0xb8]
141723289: call 0x140613c20
14172328e: cmp qword ptr [rbx + 0x480], 0
141723296: je 0x1417232fc
141723298: call 0x140c18130
14172329d: mov rcx, rax
1417232a0: mov rax, qword ptr [rax]
1417232a3: test rax, rax
1417232a6: je 0x1417232b7
1417232a8: cmp eax, 1
1417232ab: jne 0x141727417
1417232b1: add rcx, 8
1417232b5: jmp 0x1417232ca
1417232b7: xor edx, edx
1417232b9: call 0x1428f87d0
1417232be: mov rcx, rax
1417232c1: test rax, rax
1417232c4: je 0x141727417
1417232ca: cmp qword ptr [rcx], 0
1417232ce: jne 0x1417270a1
1417232d4: mov qword ptr [rcx], 0xffffffffffffffff
1417232db: mov qword ptr [rbx + 0xa8], rcx
1417232e2: add rcx, 8
1417232e6: mov rdx, qword ptr [rbx + 0x488]
1417232ed: call 0x140c170f0
1417232f2: mov rax, qword ptr [rbx + 0xa8]
1417232f9: inc qword ptr [rax]
1417232fc: mov rax, qword ptr [rbx + 0x80]
141723303: mov rax, qword ptr [rax]
141723306: test rax, rax
141723309: je 0x141723394
14172330f: mov rcx, qword ptr [rbx + 0x98]
141723316: call rax
141723318: jmp 0x141723394
14172331a: jae 0x141723331
14172331c: mov rcx, qword ptr [r15 + 8]
141723320: mov rdi, r8
141723323: mov r8d, 1
141723329: call 0x140613c20
14172332e: mov r8, rdi
141723331: mov rax, qword ptr [rbx + 0xa8]
141723338: mov qword ptr [r15], rax
14172333b: mov qword ptr [r15 + 8], r8
14172333f: mov qword ptr [r15 + 0x10], rsi
141723343: cmp qword ptr [rbx + 0x480], 0
14172334b: je 0x14172337c
14172334d: lea rax, [rbx + 0x488]
141723354: mov qword ptr [rbx + 0x200], rax
14172335b: lea rax, [rbx + 0x200]
141723362: mov qword ptr [rbx + 0x700], rax
141723369: lea rcx, [rip + 0x15a7040]
141723370: lea rdx, [rbx + 0x700]
141723377: call 0x14132f2d0
14172337c: mov rax, qword ptr [rbx + 0x80]
141723383: mov rax, qword ptr [rax]
141723386: test rax, rax
141723389: je 0x141723394
14172338b: mov rcx, qword ptr [rbx + 0x98]
141723392: call rax
141723394: mov rax, qword ptr [rbx + 0x80]
14172339b: mov rdx, qword ptr [rax + 8]
14172339f: test rdx, rdx
1417233a2: je 0x141726c0b
1417233a8: mov r8, qword ptr [rax + 0x10]
1417233ac: mov rcx, qword ptr [rbx + 0x98]
1417233b3: call 0x140613c20
1417233b8: jmp 0x141726c0b
1417233bd: jne 0x14172260d
1417233c3: mov edi, 0x33
1417233c8: sub rdi, r8
1417233cb: add rsi, r8
1417233ce: call 0x140c18130
1417233d3: mov rcx, rax
1417233d6: mov rax, qword ptr [rax]
1417233d9: test rax, rax
1417233dc: je 0x1417233ed
1417233de: cmp eax, 1
1417233e1: jne 0x141727351
1417233e7: add rcx, 8
1417233eb: jmp 0x141723400
1417233ed: xor edx, edx
1417233ef: call 0x1428f87d0
1417233f4: mov rcx, rax
1417233f7: test rax, rax
1417233fa: je 0x141727351
141723400: cmp qword ptr [rcx], 0
141723404: jne 0x141726f9a
14172340a: mov qword ptr [rcx], 0xffffffffffffffff
141723411: mov qword ptr [rbx + 0x80], rcx
141723418: add rcx, 8
14172341c: mov qword ptr [rsp + 0x20], rdi
141723421: mov qword ptr [rsp + 0x30], 0
14172342a: mov qword ptr [rsp + 0x28], 1
141723433: lea rdx, [rip + 0x15a79d6]
14172343a: mov r8d, 0x13
141723440: mov r9, rsi
141723443: call 0x140c16d40
141723448: mov rsi, rax
14172344b: mov rax, qword ptr [rbx + 0x80]
141723452: inc qword ptr [rax]
141723455: mov qword ptr [rbx + 0x208], rsi
14172345c: mov eax, 1
141723461: mov byte ptr [rbx + 0x80], r14b
141723468: mov qword ptr [rbx + 0x200], rax
14172346f: cmp dword ptr [r15], 3
141723473: je 0x14172718e
141723479: mov rcx, r15
14172347c: call 0x14171f7f0
141723481: test al, al
141723483: je 0x141723489
141723485: xor edx, edx
141723487: jmp 0x1417234c6
141723489: lea rax, [rip + 0x15a79b8]
141723490: mov qword ptr [rbx + 0x700], rax
141723497: mov qword ptr [rbx + 0x708], 1
1417234a2: mov qword ptr [rbx + 0x710], 8
1417234ad: xorps xmm0, xmm0
1417234b0: movups xmmword ptr [rbx + 0x718], xmm0
1417234b7: lea rcx, [rbx + 0x700]
1417234be: call 0x1428a8020
1417234c3: mov rdx, rax
1417234c6: lea rcx, [rbx + 0x480]
1417234cd: call 0x1408da1b0
1417234d2: mov qword ptr [rbx + 0x700], rax
1417234d9: mov qword ptr [rbx + 0x708], rdx
1417234e0: test rax, rax
1417234e3: sete al
1417234e6: test rdx, rdx
1417234e9: sete cl
1417234ec: or cl, al
1417234ee: jne 0x1417234fc
1417234f0: lea rcx, [rbx + 0x708]
1417234f7: call 0x140d4cc30
1417234fc: cmp byte ptr [rbx + 0x80], 0
141723503: je 0x141723565
141723505: call 0x140c18130
14172350a: mov rcx, rax
14172350d: mov rax, qword ptr [rax]
141723510: test rax, rax
141723513: je 0x141723524
141723515: cmp eax, 1
141723518: jne 0x1417273e2
14172351e: add rcx, 8
141723522: jmp 0x141723537
141723524: xor edx, edx
141723526: call 0x1428f87d0
14172352b: mov rcx, rax
14172352e: test rax, rax
141723531: je 0x1417273e2
141723537: cmp qword ptr [rcx], 0
14172353b: jne 0x141726fed
141723541: mov qword ptr [rcx], 0xffffffffffffffff
141723548: mov qword ptr [rbx + 0x80], rcx
14172354f: add rcx, 8
141723553: mov rdx, rsi
141723556: call 0x140c170f0
14172355b: mov rax, qword ptr [rbx + 0x80]
141723562: inc qword ptr [rax]
141723565: lea rcx, [rbx + 0x480]
14172356c: call 0x140877b30
141723571: jmp 0x141726c0b
141723576: jne 0x14172265d
14172357c: mov edi, 0x33
141723581: sub rdi, r8
141723584: add rsi, r8
141723587: call 0x140c18130
14172358c: mov rcx, rax
14172358f: mov rax, qword ptr [rax]
141723592: test rax, rax
141723595: je 0x1417235a6
141723597: cmp eax, 1
14172359a: jne 0x141727362
1417235a0: add rcx, 8
1417235a4: jmp 0x1417235bd
1417235a6: mov byte ptr [rbx + 0x7e], 1
1417235aa: xor edx, edx
1417235ac: call 0x1428f87d0
1417235b1: mov rcx, rax
1417235b4: test rax, rax
1417235b7: je 0x141727362
1417235bd: cmp qword ptr [rcx], 0
1417235c1: jne 0x141726fab
1417235c7: mov qword ptr [rcx], 0xffffffffffffffff
1417235ce: mov qword ptr [rbx + 0x80], rcx
1417235d5: add rcx, 8
1417235d9: mov qword ptr [rsp + 0x20], rdi
1417235de: mov qword ptr [rsp + 0x30], 0
1417235e7: mov qword ptr [rsp + 0x28], 1
1417235f0: lea rdx, [rip + 0x15a76e1]
1417235f7: mov r8d, 0x1a
1417235fd: mov r9, rsi
141723600: call 0x140c16d40
141723605: mov rcx, qword ptr [rbx + 0x80]
14172360c: inc qword ptr [rcx]
14172360f: mov qword ptr [rbx + 0x198], rax
141723616: mov eax, 1
14172361b: mov qword ptr [rbx + 0x190], rax
141723622: cmp dword ptr [r15], 3
141723626: je 0x14172374d
14172362c: movdqu xmm0, xmmword ptr [r15 + 0x298]
141723635: pcmpeqb xmm0, xmmword ptr [rbx + 0x120]
14172363d: pmovmskb eax, xmm0
141723641: cmp eax, 0xffff
141723646: jne 0x14172374d
14172364c: lea rdx, [r15 + 0x298]
141723653: mov rax, qword ptr [rbx + 0x98]
14172365a: mov qword ptr [rbx + 0x1340], rax
141723661: mov rax, qword ptr [rbx + 0xa8]
141723668: mov qword ptr [rbx + 0x1348], rax
14172366f: movdqa xmm0, xmmword ptr [rbx + 0x170]
141723677: movdqu xmmword ptr [rbx + 0x1350], xmm0
14172367f: lea rcx, [rbx + 0x700]
141723686: call 0x140b46000
14172368b: lea rcx, [rbx + 0x700]
141723692: call 0x140d76ed0
141723697: mov r8, rdx
14172369a: mov qword ptr [rsp + 0x20], 0xc
1417236a3: lea r9, [rip + 0x15a6ffe]
1417236aa: lea rcx, [rbx + 0x480]
1417236b1: mov rdx, rax
1417236b4: call 0x140d62810
1417236b9: mov rdx, qword ptr [rbx + 0x700]
1417236c0: test rdx, rdx
1417236c3: je 0x1417236d7
1417236c5: mov rcx, qword ptr [rbx + 0x708]
1417236cc: mov r8d, 1
1417236d2: call 0x140613c20
1417236d7: mov byte ptr [rbx + 0x95], 1
1417236de: mov rcx, r15
1417236e1: call 0x14171f7f0
1417236e6: test al, al
1417236e8: je 0x141723dbf
1417236ee: movdqu xmm0, xmmword ptr [rbx + 0x480]
1417236f6: movups xmm1, xmmword ptr [rbx + 0x490]
1417236fd: movaps xmmword ptr [rbx + 0x710], xmm1
141723704: movdqa xmmword ptr [rbx + 0x700], xmm0
14172370c: mov byte ptr [rbx + 0x95], 0
141723713: lea rcx, [rbx + 0x700]
14172371a: lea rdx, [rbx + 0x1340]
141723721: call 0x140914ae0
141723726: mov byte ptr [rbx + 0x95], 0
14172372d: lea r8, [rbx + 0x1340]
141723734: mov rcx, rax
141723737: call 0x1415a4f40
14172373c: test rax, rax
14172373f: je 0x141725cb2
141723745: mov rsi, rdx
141723748: jmp 0x141725cb4
14172374d: mov byte ptr [rbx + 0x97], 1
141723754: lea rcx, [rbx + 0x700]
14172375b: lea rdx, [rbx + 0x120]
141723762: call 0x140b46000
141723767: lea rcx, [rbx + 0x700]
14172376e: call 0x140d76ed0
141723773: mov r8, rdx
141723776: mov qword ptr [rsp + 0x20], 0xc
14172377f: lea r9, [rip + 0x15a6f22]
141723786: lea rcx, [rbx + 0x100]
14172378d: mov rdx, rax
141723790: call 0x140d62810
141723795: mov rdx, qword ptr [rbx + 0x700]
14172379c: test rdx, rdx
14172379f: je 0x1417237b3
1417237a1: mov rcx, qword ptr [rbx + 0x708]
1417237a8: mov r8d, 1
1417237ae: call 0x140613c20
1417237b3: mov cl, 1
1417237b5: xor eax, eax
1417237b7: lock cmpxchg byte ptr [r12], cl
1417237bd: jne 0x141726f0e
1417237c3: mov byte ptr [rbx + 0x8d], 1
1417237ca: mov byte ptr [rbx + 0x8e], 1
1417237d1: mov rcx, r12
1417237d4: call 0x140c81c70
1417237d9: mov rcx, qword ptr [r12 + 0x10]
1417237de: mov qword ptr [rbx + 0xb8], r12
1417237e5: mov rax, qword ptr [r12 + 0x18]
1417237ea: shl rax, 7
1417237ee: add rax, rcx
1417237f1: mov qword ptr [rbx + 0x80], rax
1417237f8: lea r13, [rbx + 0x711]
1417237ff: nop 
141723800: cmp rcx, qword ptr [rbx + 0x80]
141723807: je 0x141723b72
14172380d: mov rsi, rcx
141723810: mov byte ptr [rbx + 0x92], 1
141723817: call 0x140d76ed0
14172381c: mov r15, rax
14172381f: mov r12, rdx
141723822: mov byte ptr [rbx + 0x92], 1
141723829: lea rcx, [rbx + 0x100]
141723830: call 0x140d76ed0
141723835: mov rdi, rax
141723838: mov r14, rdx
14172383b: mov byte ptr [rbx + 0x92], 1
141723842: lea rcx, [rbx + 0x1340]
141723849: mov rdx, r15
14172384c: mov r8, r12
14172384f: call 0x140d62c00
141723854: mov byte ptr [rbx + 0x92], 1
14172385b: lea rcx, [rbx + 0x480]
141723862: mov rdx, rdi
141723865: mov r8, r14
141723868: call 0x140d62c00
14172386d: mov rdi, qword ptr [rbx + 0x1348]
141723874: mov r14, qword ptr [rbx + 0x488]
14172387b: cmp rdi, r14
14172387e: jne 0x1417238d0
141723880: movzx r15d, byte ptr [rbx + 0x1350]
141723888: movzx eax, byte ptr [rbx + 0x1378]
14172388f: cmp al, byte ptr [rbx + 0x4b8]
141723895: jne 0x1417238d8
141723897: cmp byte ptr [rbx + 0x1379], 2
14172389e: jne 0x1417238d8
1417238a0: cmp byte ptr [rbx + 0x4b9], 2
1417238a7: jne 0x1417238d8
1417238a9: cmp r15b, 6
1417238ad: jne 0x141723a25
1417238b3: cmp byte ptr [rbx + 0x490], 2
1417238ba: ja 0x141723a36
1417238c0: mov cl, 6
1417238c2: mov rax, qword ptr [rbx + 0x1340]
1417238c9: jmp 0x141723913
1417238cb: nop dword ptr [rax + rax]
1417238d0: movzx r15d, byte ptr [rbx + 0x1350]
1417238d8: mov rax, qword ptr [rbx + 0x1340]
1417238df: mov cl, 6
1417238e1: cmp r15b, 6
1417238e5: je 0x141723913
1417238e7: lea rdx, [rbx + 0x1351]
1417238ee: mov rcx, qword ptr [rdx + 0x1f]
1417238f2: mov qword ptr [rbx + 0x15f], rcx
1417238f9: movdqu xmm0, xmmword ptr [rdx]
1417238fd: movups xmm1, xmmword ptr [rdx + 0x10]
141723901: movaps xmmword ptr [rbx + 0x150], xmm1
141723908: movdqa xmmword ptr [rbx + 0x140], xmm0
141723910: mov ecx, r15d
141723913: movzx edx, byte ptr [rbx + 0x137a]
14172391a: movzx r8d, word ptr [rbx + 0x1378]
141723922: mov qword ptr [rbx + 0x200], rax
141723929: mov qword ptr [rbx + 0x208], rdi
141723930: mov byte ptr [rbx + 0x210], cl
141723936: mov rax, qword ptr [rbx + 0x15f]
14172393d: lea rcx, [rbx + 0x211]
141723944: mov qword ptr [rcx + 0x1f], rax
141723948: movaps xmm0, xmmword ptr [rbx + 0x140]
14172394f: movaps xmm1, xmmword ptr [rbx + 0x150]
141723956: movups xmmword ptr [rcx + 0x10], xmm1
14172395a: movups xmmword ptr [rcx], xmm0
14172395d: mov word ptr [rbx + 0x238], r8w
141723965: mov byte ptr [rbx + 0x23a], dl
14172396b: movzx eax, byte ptr [rbx + 0x490]
141723972: cmp al, 6
141723974: je 0x14172399d
141723976: lea rdx, [rbx + 0x491]
14172397d: mov rcx, qword ptr [rdx + 0x1f]
141723981: mov qword ptr [rbx + 0x15bf], rcx
141723988: movups xmm0, xmmword ptr [rdx]
14172398b: movups xmm1, xmmword ptr [rdx + 0x10]
14172398f: movaps xmmword ptr [rbx + 0x15b0], xmm1
141723996: movaps xmmword ptr [rbx + 0x15a0], xmm0
14172399d: mov rcx, qword ptr [rbx + 0x480]
1417239a4: movzx edx, byte ptr [rbx + 0x4ba]
1417239ab: movzx r8d, word ptr [rbx + 0x4b8]
1417239b3: mov qword ptr [rbx + 0x700], rcx
1417239ba: mov qword ptr [rbx + 0x708], r14
1417239c1: mov byte ptr [rbx + 0x710], al
1417239c7: mov rax, qword ptr [rbx + 0x15bf]
1417239ce: mov qword ptr [r13 + 0x1f], rax
1417239d2: movdqa xmm0, xmmword ptr [rbx + 0x15a0]
1417239da: movaps xmm1, xmmword ptr [rbx + 0x15b0]
1417239e1: movups xmmword ptr [r13 + 0x10], xmm1
1417239e6: movdqu xmmword ptr [r13], xmm0
1417239ec: mov word ptr [rbx + 0x738], r8w
1417239f4: mov byte ptr [rbx + 0x73a], dl
1417239fa: mov byte ptr [rbx + 0x92], 1
141723a01: lea rcx, [rbx + 0x200]
141723a08: lea rdx, [rbx + 0x700]
141723a0f: call 0x1416f6360
141723a14: lea rcx, [rsi + 0x80]
141723a1b: test al, al
141723a1d: je 0x141723800
141723a23: jmp 0x141723a72
141723a25: movzx eax, byte ptr [rbx + 0x490]
141723a2c: cmp al, 6
141723a2e: jne 0x141723a56
141723a30: cmp r15b, 3
141723a34: jb 0x141723a66
141723a36: mov rcx, qword ptr [rbx + 0x1340]
141723a3d: mov rdx, qword ptr [rbx + 0x480]
141723a44: mov r8, rdi
141723a47: call 0x142923bf0
141723a4c: test eax, eax
141723a4e: jne 0x1417238d8
141723a54: jmp 0x141723a72
141723a56: cmp r15b, 3
141723a5a: setb cl
141723a5d: cmp al, 3
141723a5f: setb al
141723a62: xor al, cl
141723a64: je 0x141723a36
141723a66: mov rax, qword ptr [rbx + 0x1340]
141723a6d: jmp 0x1417238e7
141723a72: mov rax, qword ptr [rbx + 0x98]
141723a79: mov qword ptr [rbx + 0x700], rax
141723a80: mov rax, qword ptr [rbx + 0xa8]
141723a87: mov qword ptr [rbx + 0x708], rax
141723a8e: movdqa xmm0, xmmword ptr [rbx + 0x170]
141723a96: movdqu xmmword ptr [rbx + 0x710], xmm0
141723a9e: mov rdi, qword ptr [rsi + 0x28]
141723aa2: mov rsi, qword ptr [rsi + 0x30]
141723aa6: lea rcx, [rbx + 0x700]
141723aad: call 0x140d76ed0
141723ab2: mov rcx, rax
141723ab5: mov r8, rdi
141723ab8: mov r9, rsi
141723abb: call 0x140d59430
141723ac0: mov rsi, rax
141723ac3: mov rdx, qword ptr [rbx + 0x700]
141723aca: test rdx, rdx
141723acd: je 0x141723ae1
141723acf: mov rcx, qword ptr [rbx + 0x708]
141723ad6: mov r8d, 1
141723adc: call 0x140613c20
141723ae1: test rsi, rsi
141723ae4: je 0x141723d27
141723aea: mov qword ptr [rbx + 0x480], rsi
141723af1: mov byte ptr [rbx + 0xcd], 1
141723af8: lea rcx, [rbx + 0x700]
141723aff: call 0x140d56700
141723b04: movdqu xmm0, xmmword ptr [rbx + 0x700]
141723b0c: movups xmm1, xmmword ptr [rbx + 0x710]
141723b13: movups xmm2, xmmword ptr [rbx + 0x720]
141723b1a: movaps xmmword ptr [rbx + 0x220], xmm2
141723b21: movaps xmmword ptr [rbx + 0x210], xmm1
141723b28: movdqa xmmword ptr [rbx + 0x200], xmm0
141723b30: lea rax, [rip + 0x15a71df]
141723b37: mov qword ptr [rbx + 0x700], rax
141723b3e: mov qword ptr [rbx + 0x708], 0x1f
141723b49: mov qword ptr [rbx + 0x710], rsi
141723b50: mov byte ptr [rbx + 0xcd], 0
141723b57: lea rcx, [rbx + 0x700]
141723b5e: lea rdx, [rbx + 0x200]
141723b65: call 0x1428a8e80
141723b6a: mov rdx, rax
141723b6d: jmp 0x141723d29
141723b72: mov byte ptr [rbx + 0x8d], 1
141723b79: mov byte ptr [rbx + 0x8e], 1
141723b80: mov rsi, qword ptr [rbx + 0xb8]
141723b87: mov rcx, rsi
141723b8a: call 0x140c81e10
141723b8f: xor ecx, ecx
141723b91: mov al, 1
141723b93: lock cmpxchg byte ptr [rsi], cl
141723b97: jne 0x1417270e4
141723b9d: movups xmm0, xmmword ptr [rbx + 0x100]
141723ba4: movups xmm1, xmmword ptr [rbx + 0x110]
141723bab: movaps xmmword ptr [rbx + 0x210], xmm1
141723bb2: movaps xmmword ptr [rbx + 0x200], xmm0
141723bb9: mov rax, qword ptr [rbx + 0x98]
141723bc0: mov qword ptr [rbx + 0x700], rax
141723bc7: mov rax, qword ptr [rbx + 0xa8]
141723bce: mov qword ptr [rbx + 0x708], rax
141723bd5: movdqa xmm0, xmmword ptr [rbx + 0x170]
141723bdd: movdqu xmmword ptr [rbx + 0x710], xmm0
141723be5: mov byte ptr [rbx + 0x8d], 0
141723bec: mov byte ptr [rbx + 0x8e], 0
141723bf3: lea rcx, [rbx + 0x200]
141723bfa: lea rdx, [rbx + 0x700]
141723c01: call 0x1403107e0
141723c06: mov rsi, rdx
141723c09: test rdx, rdx
141723c0c: setne cl
141723c0f: and cl, al
141723c11: cmp cl, 1
141723c14: jne 0x141723c9f
141723c1a: mov qword ptr [rbx + 0x480], rsi
141723c21: mov byte ptr [rbx + 0xcc], 1
141723c28: lea rcx, [rbx + 0x700]
141723c2f: call 0x140d56700
141723c34: movdqu xmm0, xmmword ptr [rbx + 0x700]
141723c3c: movups xmm1, xmmword ptr [rbx + 0x710]
141723c43: movups xmm2, xmmword ptr [rbx + 0x720]
141723c4a: movaps xmmword ptr [rbx + 0x220], xmm2
141723c51: movaps xmmword ptr [rbx + 0x210], xmm1
141723c58: movdqa xmmword ptr [rbx + 0x200], xmm0
141723c60: lea rax, [rip + 0x15a708b]
141723c67: mov qword ptr [rbx + 0x700], rax
141723c6e: mov qword ptr [rbx + 0x708], 0x24
141723c79: mov qword ptr [rbx + 0x710], rsi
141723c80: mov byte ptr [rbx + 0xcc], 0
141723c87: lea rcx, [rbx + 0x700]
141723c8e: lea rdx, [rbx + 0x200]
141723c95: call 0x1428a8e80
141723c9a: mov rdx, rax
141723c9d: jmp 0x141723ca1
141723c9f: xor edx, edx
141723ca1: mov byte ptr [rbx + 0x8d], 0
141723ca8: mov byte ptr [rbx + 0x8e], 0
141723caf: lea rcx, [rbx + 0xd8]
141723cb6: call 0x1408da1b0
141723cbb: mov qword ptr [rbx + 0x700], rax
141723cc2: mov qword ptr [rbx + 0x708], rdx
141723cc9: test rax, rax
141723ccc: sete al
141723ccf: test rdx, rdx
141723cd2: sete cl
141723cd5: or cl, al
141723cd7: jne 0x141723cf3
141723cd9: lea rcx, [rbx + 0x708]
141723ce0: mov byte ptr [rbx + 0x8d], 0
141723ce7: mov byte ptr [rbx + 0x8e], 0
141723cee: call 0x140d4cc30
141723cf3: cmp qword ptr [rbx + 0x190], 0
141723cfb: je 0x141723f2e
141723d01: call 0x140c18130
141723d06: mov rcx, rax
141723d09: mov rax, qword ptr [rax]
141723d0c: test rax, rax
141723d0f: je 0x141723ee5
141723d15: cmp eax, 1
141723d18: jne 0x14172745a
141723d1e: add rcx, 8
141723d22: jmp 0x141723efc
141723d27: xor edx, edx
141723d29: mov byte ptr [rbx + 0x92], 0
141723d30: lea rcx, [rbx + 0xd8]
141723d37: call 0x1408da1b0
141723d3c: mov qword ptr [rbx + 0x700], rax
141723d43: mov qword ptr [rbx + 0x708], rdx
141723d4a: test rax, rax
141723d4d: sete al
141723d50: test rdx, rdx
141723d53: sete cl
141723d56: or cl, al
141723d58: jne 0x141723d6d
141723d5a: lea rcx, [rbx + 0x708]
141723d61: mov byte ptr [rbx + 0x92], 0
141723d68: call 0x140d4cc30
141723d6d: mov byte ptr [rbx + 0x8d], 1
141723d74: mov byte ptr [rbx + 0x8e], 0
141723d7b: mov rsi, qword ptr [rbx + 0xb8]
141723d82: mov rcx, rsi
141723d85: call 0x140c81e10
141723d8a: xor ecx, ecx
141723d8c: mov al, 1
141723d8e: lock cmpxchg byte ptr [rsi], cl
141723d92: jne 0x1417270c7
141723d98: mov rdx, qword ptr [rbx + 0x100]
141723d9f: test rdx, rdx
141723da2: je 0x141725d19
141723da8: mov rcx, qword ptr [rbx + 0x108]
141723daf: mov r8d, 1
141723db5: call 0x140613c20
141723dba: jmp 0x141725d19
141723dbf: mov rax, qword ptr [rip + 0x1f085e2]
141723dc6: mov rax, qword ptr [rax]
141723dc9: test rax, rax
141723dcc: je 0x141723e66
141723dd2: lea rax, [rip + 0x15a6957]
141723dd9: mov qword ptr [rbx + 0x700], rax
141723de0: mov qword ptr [rbx + 0x708], 1
141723deb: mov qword ptr [rbx + 0x710], 8
141723df6: pxor xmm0, xmm0
141723dfa: movdqu xmmword ptr [rbx + 0x718], xmm0
141723e02: mov byte ptr [rbx + 0x95], 1
141723e09: lea rcx, [rip + 0x15a6a70]
141723e10: call 0x14067dd80
141723e15: lea rcx, [rip + 0x15a6744]
141723e1c: mov qword ptr [rbx + 0x200], rcx
141723e23: mov qword ptr [rbx + 0x208], 0x28
141723e2e: mov qword ptr [rbx + 0x210], rcx
141723e35: mov qword ptr [rbx + 0x218], 0x28
141723e40: mov qword ptr [rbx + 0x220], rax
141723e47: mov byte ptr [rbx + 0x95], 1
141723e4e: lea rcx, [rbx + 0x700]
141723e55: lea r8, [rbx + 0x200]
141723e5c: mov edx, 1
141723e61: call 0x1404b1430
141723e66: mov rsi, qword ptr [r15 + 0x38]
141723e6a: mov rdi, qword ptr [r15 + 0x40]
141723e6e: mov byte ptr [rbx + 0x95], 1
141723e75: lea rcx, [rbx + 0x1340]
141723e7c: call 0x140d76ed0
141723e81: mov byte ptr [rbx + 0x95], 1
141723e88: mov rcx, rax
141723e8b: mov r8, rsi
141723e8e: mov r9, rdi
141723e91: call 0x140d59430
141723e96: mov byte ptr [rbx + 0x95], 1
141723e9d: lea rdx, [rip + 0x15a68b4]
141723ea4: mov r8d, 0x24
141723eaa: mov rcx, rax
141723ead: call 0x1415a5fe0
141723eb2: mov rsi, rax
141723eb5: test rax, rax
141723eb8: je 0x141725c94
141723ebe: mov rdx, qword ptr [rbx + 0x480]
141723ec5: test rdx, rdx
141723ec8: je 0x141725cb4
141723ece: mov rcx, qword ptr [rbx + 0x488]
141723ed5: mov r8d, 1
141723edb: call 0x140613c20
141723ee0: jmp 0x141725cb4
141723ee5: mov byte ptr [rbx + 0x7e], 0
141723ee9: xor edx, edx
141723eeb: call 0x1428f87d0
141723ef0: mov rcx, rax
141723ef3: test rax, rax
141723ef6: je 0x14172745a
141723efc: cmp qword ptr [rcx], 0
141723f00: jne 0x141727101
141723f06: mov qword ptr [rcx], 0xffffffffffffffff
141723f0d: mov qword ptr [rbx + 0x80], rcx
141723f14: add rcx, 8
141723f18: mov rdx, qword ptr [rbx + 0x198]
141723f1f: call 0x140c170f0
141723f24: mov rax, qword ptr [rbx + 0x80]
141723f2b: inc qword ptr [rax]
141723f2e: mov byte ptr [rbx + 0xc5], 0
141723f35: lea rcx, [rbx + 0xd8]
141723f3c: call 0x140877b30
141723f41: jmp 0x141726c0b
141723f46: jne 0x1417226b1
141723f4c: mov edi, 0x33
141723f51: sub rdi, r8
141723f54: add rsi, r8
141723f57: call 0x140c18130
141723f5c: mov rcx, rax
141723f5f: mov rax, qword ptr [rax]
141723f62: test rax, rax
141723f65: je 0x141723f76
141723f67: cmp eax, 1
141723f6a: jne 0x141727377
141723f70: add rcx, 8
141723f74: jmp 0x141723f90
141723f76: mov byte ptr [rbx + 0x90], 1
141723f7d: xor edx, edx
141723f7f: call 0x1428f87d0
141723f84: mov rcx, rax
141723f87: test rax, rax
141723f8a: je 0x141727377
141723f90: cmp qword ptr [rcx], 0
141723f94: jne 0x141726fc0
141723f9a: mov qword ptr [rcx], 0xffffffffffffffff
141723fa1: mov qword ptr [rbx + 0x98], rcx
141723fa8: add rcx, 8
141723fac: mov qword ptr [rsp + 0x20], rdi
141723fb1: mov qword ptr [rsp + 0x30], 0
141723fba: mov qword ptr [rsp + 0x28], 1
141723fc3: lea rdx, [rip + 0x15a6d6b]
141723fca: mov r8d, 0x1d
141723fd0: mov r9, rsi
141723fd3: call 0x140c16d40
141723fd8: mov rdi, rax
141723fdb: mov rax, qword ptr [rbx + 0x98]
141723fe2: inc qword ptr [rax]
141723fe5: mov qword ptr [rbx + 0xe0], rdi
141723fec: mov eax, 1
141723ff1: mov byte ptr [rbx + 0x98], r14b
141723ff8: mov qword ptr [rbx + 0xd8], rax
141723fff: mov rax, qword ptr [r15]
141724002: cmp rax, 3
141724006: sete byte ptr [rbx + 0xa8]
14172400d: cmp eax, 3
141724010: je 0x1417271d4
141724016: movups xmm0, xmmword ptr [r15 + 0x298]
14172401e: movaps xmmword ptr [rbx + 0x190], xmm0
141724025: lea rcx, [rbx + 0x700]
14172402c: mov rdx, qword ptr [rbx + 0x80]
141724033: call 0x1417a1d00
141724038: mov rdx, qword ptr [rbx + 0x700]
14172403f: mov rcx, qword ptr [rbx + 0x708]
141724046: mov rax, rdx
141724049: neg rax
14172404c: jo 0x141725a6a
141724052: mov qword ptr [rbx + 0xb8], rdx
141724059: mov r14, qword ptr [rbx + 0x710]
141724060: mov qword ptr [rbx + 0xd0], rcx
141724067: mov qword ptr [rbx + 0x700], rcx
14172406e: mov qword ptr [rbx + 0x708], r14
141724075: lea rcx, [rbx + 0x700]
14172407c: call 0x14145b1a0
141724081: mov rsi, rax
141724084: lea rcx, [rbx + 0x100]
14172408b: lea rdx, [rbx + 0x190]
141724092: call 0x140b46000
141724097: lea rcx, [rbx + 0x100]
14172409e: call 0x140d76ed0
1417240a3: mov r8, rdx
1417240a6: mov qword ptr [rsp + 0x20], 9
1417240af: lea r9, [rip + 0x15a6e72]
1417240b6: lea rcx, [rbx + 0x140]
1417240bd: mov rdx, rax
1417240c0: call 0x140d62810
1417240c5: mov qword ptr [rbx + 0x1340], rsi
1417240cc: lea rax, [rbx + 0x1340]
1417240d3: mov qword ptr [rbx + 0x200], rax
1417240da: lea rax, [rip - 0x996a21]
1417240e1: mov qword ptr [rbx + 0x208], rax
1417240e8: lea rax, [rip + 0x13b3d79]
1417240ef: mov qword ptr [rbx + 0x210], rax
1417240f6: lea rax, [rip - 0x161a26d]
1417240fd: mov qword ptr [rbx + 0x218], rax
141724104: lea rax, [rip + 0x13b3d75]
14172410b: mov qword ptr [rbx + 0x700], rax
141724112: mov qword ptr [rbx + 0x708], 2
14172411d: lea rax, [rbx + 0x200]
141724124: mov qword ptr [rbx + 0x710], rax
14172412b: mov qword ptr [rbx + 0x718], 2
141724136: mov qword ptr [rbx + 0x720], 0
141724141: lea rcx, [rbx + 0x480]
141724148: lea rdx, [rbx + 0x700]
14172414f: call 0x140d79f90
141724154: mov rdx, qword ptr [rbx + 0x488]
14172415b: mov r8, qword ptr [rbx + 0x490]
141724162: lea rcx, [rbx + 0x15a0]
141724169: lea r9, [rbx + 0x140]
141724170: mov qword ptr [rbx + 0x1c8], rdx
141724177: call 0x1412e6780
14172417c: mov rdx, qword ptr [rbx + 0x480]
141724183: test rdx, rdx
141724186: je 0x14172419a
141724188: mov r8d, 1
14172418e: mov rcx, qword ptr [rbx + 0x1c8]
141724195: call 0x140613c20
14172419a: mov byte ptr [rbx + 0x91], 1
1417241a1: lea rcx, [rbx + 0x15a0]
1417241a8: call 0x140d76ed0
1417241ad: mov byte ptr [rbx + 0x91], 1
1417241b4: mov rcx, rax
1417241b7: call 0x140d62cf0
1417241bc: test al, al
1417241be: je 0x1417241c8
1417241c0: xor r14d, r14d
1417241c3: jmp 0x1417259e3
1417241c8: mov byte ptr [rbx + 0x700], 1
1417241cf: mov byte ptr [rbx + 0x91], 1
1417241d6: lea rcx, [rbx + 0x140]
1417241dd: call 0x140d76ed0
1417241e2: mov r8, rdx
1417241e5: mov byte ptr [rbx + 0x91], 1
1417241ec: lea rcx, [rbx + 0x700]
1417241f3: mov rdx, rax
1417241f6: call 0x140d5a150
1417241fb: mov r15, rax
1417241fe: test rax, rax
141724201: je 0x14172599e
141724207: mov qword ptr [rbx + 0x480], r15
14172420e: mov byte ptr [rbx + 0xce], 1
141724215: lea rcx, [rbx + 0x700]
14172421c: call 0x140d56700
141724221: movups xmm0, xmmword ptr [rbx + 0x700]
141724228: movups xmm1, xmmword ptr [rbx + 0x710]
14172422f: movups xmm2, xmmword ptr [rbx + 0x720]
141724236: movaps xmmword ptr [rbx + 0x220], xmm2
14172423d: movaps xmmword ptr [rbx + 0x210], xmm1
141724244: movaps xmmword ptr [rbx + 0x200], xmm0
14172424b: lea rax, [rip + 0x15a6da4]
141724252: mov qword ptr [rbx + 0x700], rax
141724259: mov qword ptr [rbx + 0x708], 0x18
141724264: mov qword ptr [rbx + 0x710], r15
14172426b: mov byte ptr [rbx + 0xce], 0
141724272: lea rcx, [rbx + 0x700]
141724279: lea rdx, [rbx + 0x200]
141724280: call 0x1428a8e80
141724285: jmp 0x1417259da
14172428a: jne 0x141722708
141724290: mov edi, 0x33
141724295: sub rdi, r8
141724298: add rsi, r8
14172429b: call 0x140c18130
1417242a0: mov rcx, rax
1417242a3: mov rax, qword ptr [rax]
1417242a6: test rax, rax
1417242a9: je 0x1417242ba
1417242ab: cmp eax, 1
1417242ae: jne 0x14172738f
1417242b4: add rcx, 8
1417242b8: jmp 0x1417242d1
1417242ba: mov byte ptr [rbx + 0x7f], 1
1417242be: xor edx, edx
1417242c0: call 0x1428f87d0
1417242c5: mov rcx, rax
1417242c8: test rax, rax
1417242cb: je 0x14172738f
1417242d1: cmp qword ptr [rcx], 0
1417242d5: jne 0x141726fd8
1417242db: mov qword ptr [rcx], 0xffffffffffffffff
1417242e2: mov qword ptr [rbx + 0x80], rcx
1417242e9: add rcx, 8
1417242ed: mov qword ptr [rsp + 0x20], rdi
1417242f2: mov qword ptr [rsp + 0x30], 0
1417242fb: mov qword ptr [rsp + 0x28], 1
141724304: lea rdx, [rip + 0x15a67cd]
14172430b: mov r8d, 0x16
141724311: mov r9, rsi
141724314: call 0x140c16d40
141724319: mov rcx, qword ptr [rbx + 0x80]
141724320: inc qword ptr [rcx]
141724323: mov qword ptr [rbx + 0x1f8], rax
14172432a: mov eax, 1
14172432f: mov qword ptr [rbx + 0x1f0], rax
141724336: movaps xmm0, xmmword ptr [rbx + 0x190]
14172433d: movaps xmm1, xmmword ptr [rbx + 0x1a0]
141724344: movaps xmmword ptr [rbx + 0x110], xmm1
14172434b: movaps xmmword ptr [rbx + 0x100], xmm0
141724352: movaps xmm0, xmmword ptr [rbx + 0x6f0]
141724359: movaps xmmword ptr [rbx + 0x1e0], xmm0
141724360: mov rax, qword ptr [r15]
141724363: mov qword ptr [r15], 3
14172436a: cmp rax, 3
14172436e: je 0x14172457e
141724374: lea rdx, [r15 + 8]
141724378: mov qword ptr [rbx + 0x700], rax
14172437f: lea rcx, [rbx + 0x708]
141724386: mov r8d, 0x2a0
14172438c: call 0x142923c00
141724391: mov byte ptr [rbx + 0xb1], 1
141724398: lea rcx, [rbx + 0x700]
14172439f: call 0x14171f7f0
1417243a4: test al, al
1417243a6: je 0x1417243b9
1417243a8: lea rcx, [rbx + 0x710]
1417243af: call 0x14088ed10
1417243b4: jmp 0x1417244dd
1417243b9: mov rax, qword ptr [r14]
1417243bc: test rax, rax
1417243bf: je 0x141724457
1417243c5: lea rax, [rip + 0x15a6b34]
1417243cc: mov qword ptr [rbx + 0x200], rax
1417243d3: mov qword ptr [rbx + 0x208], 1
1417243de: mov qword ptr [rbx + 0x210], 8
1417243e9: xorps xmm0, xmm0
1417243ec: movups xmmword ptr [rbx + 0x218], xmm0
1417243f3: mov byte ptr [rbx + 0xb1], 1
1417243fa: lea rcx, [rip + 0x15a6b0f]
141724401: call 0x14067dd80
141724406: lea rcx, [rip + 0x15a6153]
14172440d: mov qword ptr [rbx + 0x480], rcx
141724414: mov qword ptr [rbx + 0x488], 0x28
14172441f: mov qword ptr [rbx + 0x490], rcx
141724426: mov qword ptr [rbx + 0x498], 0x28
141724431: mov qword ptr [rbx + 0x4a0], rax
141724438: mov byte ptr [rbx + 0xb1], 1
14172443f: lea rcx, [rbx + 0x200]
141724446: lea r8, [rbx + 0x480]
14172444d: mov edx, 1
141724452: call 0x1404b1430
141724457: movups xmm0, xmmword ptr [rbx + 0x780]
14172445e: movaps xmmword ptr [rbx + 0x270], xmm0
141724465: movups xmm0, xmmword ptr [rbx + 0x770]
14172446c: movaps xmmword ptr [rbx + 0x260], xmm0
141724473: movups xmm0, xmmword ptr [rbx + 0x760]
14172447a: movaps xmmword ptr [rbx + 0x250], xmm0
141724481: movups xmm0, xmmword ptr [rbx + 0x750]
141724488: movaps xmmword ptr [rbx + 0x240], xmm0
14172448f: movups xmm0, xmmword ptr [rbx + 0x710]
141724496: movups xmm1, xmmword ptr [rbx + 0x720]
14172449d: movups xmm2, xmmword ptr [rbx + 0x730]
1417244a4: movups xmm3, xmmword ptr [rbx + 0x740]
1417244ab: movaps xmmword ptr [rbx + 0x230], xmm3
1417244b2: movaps xmmword ptr [rbx + 0x220], xmm2
1417244b9: movaps xmmword ptr [rbx + 0x210], xmm1
1417244c0: movaps xmmword ptr [rbx + 0x200], xmm0
1417244c7: mov byte ptr [rbx + 0xb1], 0
1417244ce: lea rdx, [rbx + 0x200]
1417244d5: mov rcx, r12
1417244d8: call 0x14171ff80
1417244dd: mov r9, qword ptr [rbx + 0x7b0]
1417244e4: test r9, r9
1417244e7: je 0x14172451e
1417244e9: lea rax, [r9 + 1]
1417244ed: mov ecx, 0x18
1417244f2: mul rcx
1417244f5: add rax, 0xf
1417244f9: and rax, 0xfffffffffffffff0
1417244fd: add r9, rax
141724500: add r9, 0x11
141724504: je 0x14172451e
141724506: mov rcx, qword ptr [rbx + 0x7a8]
14172450d: sub rcx, rax
141724510: mov r8d, 0x10
141724516: mov rdx, r9
141724519: call 0x140613c20
14172451e: mov rdx, qword ptr [rbx + 0x790]
141724525: test rdx, rdx
141724528: je 0x141724540
14172452a: mov rcx, qword ptr [rbx + 0x798]
141724531: shl rdx, 3
141724535: mov r8d, 8
14172453b: call 0x140613c20
141724540: lea rcx, [rbx + 0x7d8]
141724547: call 0x14087cfb0
14172454c: lea rcx, [rbx + 0x700]
141724553: call 0x140878420
141724558: lea rcx, [rbx + 0x968]
14172455f: lea rdx, [rbx + 0x988]
141724566: mov byte ptr [rbx + 0xa6], 1
14172456d: mov r8d, 0x20
141724573: mov r9d, 0x10
141724579: call 0x140052a30
14172457e: mov qword ptr [rbx + 0xd0], r15
141724585: mov qword ptr [rbx + 0x120], 0
141724590: mov qword ptr [rbx + 0x128], 8
14172459b: mov qword ptr [rbx + 0x130], 0
1417245a6: mov byte ptr [rbx + 0xa4], 1
1417245ad: mov byte ptr [rbx + 0xa5], 1
1417245b4: lea rcx, [rbx + 0x140]
1417245bb: lea r15, [rbx + 0x1e0]
1417245c2: mov rdx, r15
1417245c5: call 0x140b46000
1417245ca: lea rcx, [rbx + 0x140]
1417245d1: call 0x140d76ed0
1417245d6: mov r8, rdx
1417245d9: mov qword ptr [rsp + 0x20], 9
1417245e2: lea r9, [rip + 0x15a693f]
1417245e9: lea rcx, [rbx + 0x480]
1417245f0: mov rdx, rax
1417245f3: call 0x140d62810
1417245f8: lea rdi, [rbx + 0x480]
1417245ff: mov rcx, rdi
141724602: call 0x140d76ed0
141724607: mov r8, rdx
14172460a: lea rcx, [rbx + 0x700]
141724611: mov rdx, rax
141724614: call 0x140d68f80
141724619: cmp dword ptr [rbx + 0x700], 2
141724620: jne 0x141724649
141724622: mov rax, qword ptr [rbx + 0x708]
141724629: mov qword ptr [rbx + 0x208], rax
141724630: mov qword ptr [rbx + 0x200], 2
14172463b: mov rdx, qword ptr [rbx + 0x480]
141724642: test rdx, rdx
141724645: jne 0x14172466e
141724647: jmp 0x141724680
141724649: lea rcx, [rbx + 0x200]
141724650: lea rdx, [rbx + 0x700]
141724657: mov r8d, 0x270
14172465d: call 0x142923c00
141724662: mov rdx, qword ptr [rbx + 0x480]
141724669: test rdx, rdx
14172466c: je 0x141724680
14172466e: mov rcx, qword ptr [rbx + 0x488]
141724675: mov r8d, 1
14172467b: call 0x140613c20
141724680: mov rsi, qword ptr [rbx + 0x200]
141724687: cmp rsi, 2
14172468b: jne 0x1417247c9
141724691: lea rax, [rbx + 0x208]
141724698: mov qword ptr [rbx + 0x1340], rax
14172469f: mov rax, qword ptr [r14]
1417246a2: test rax, rax
1417246a5: je 0x141724807
1417246ab: mov qword ptr [rbx + 0x480], r15
1417246b2: lea rax, [rip + 0x81e7]
1417246b9: mov qword ptr [rbx + 0x488], rax
1417246c0: lea rax, [rbx + 0x1340]
1417246c7: mov qword ptr [rbx + 0x490], rax
1417246ce: lea rax, [rip - 0x15af765]
1417246d5: mov qword ptr [rbx + 0x498], rax
1417246dc: lea rcx, [rip + 0x15a68dd]
1417246e3: call 0x14067dd80
1417246e8: movups xmm0, xmmword ptr [rax]
1417246eb: mov eax, dword ptr [rax + 0x10]
1417246ee: mov qword ptr [rbx + 0x730], 1
1417246f9: lea rcx, [rip + 0x15a5e60]
141724700: mov qword ptr [rbx + 0x738], rcx
141724707: mov qword ptr [rbx + 0x740], 0x28
141724712: lea rdx, [rip + 0x15a6887]
141724719: mov qword ptr [rbx + 0x750], rdx
141724720: mov qword ptr [rbx + 0x758], 2
14172472b: mov qword ptr [rbx + 0x760], rdi
141724732: mov qword ptr [rbx + 0x768], 2
14172473d: mov qword ptr [rbx + 0x770], 0
141724748: mov qword ptr [rbx + 0x700], 0
141724753: mov qword ptr [rbx + 0x708], rcx
14172475a: mov qword ptr [rbx + 0x710], 0x28
141724765: mov qword ptr [rbx + 0x718], 0
141724770: movups xmmword ptr [rbx + 0x720], xmm0
141724777: mov dword ptr [rbx + 0x748], 1
141724781: mov dword ptr [rbx + 0x74c], eax
141724787: lea rcx, [rbx + 0x93]
14172478e: lea rdx, [rbx + 0x700]
141724795: call 0x140d4a330
14172479a: mov rsi, qword ptr [rbx + 0x200]
1417247a1: mov r14, qword ptr [rbx + 0x208]
1417247a8: lea rdx, [rbx + 0x210]
1417247af: lea rcx, [rbx + 0x480]
1417247b6: mov r8d, 0x260
1417247bc: call 0x142923c00
1417247c1: cmp rsi, 2
1417247c5: jne 0x1417247e9
1417247c7: jmp 0x141724827
1417247c9: mov r14, qword ptr [rbx + 0x208]
1417247d0: lea rdx, [rbx + 0x210]
1417247d7: lea rcx, [rbx + 0x480]
1417247de: mov r8d, 0x260
1417247e4: call 0x142923c00
1417247e9: lea rcx, [rbx + 0x200]
1417247f0: lea rdx, [rbx + 0x480]
1417247f7: mov r8d, 0x260
1417247fd: call 0x142923c00
141724802: jmp 0x1417248a0
141724807: mov r14, qword ptr [rbx + 0x208]
14172480e: lea rdx, [rbx + 0x210]
141724815: lea rcx, [rbx + 0x480]
14172481c: mov r8d, 0x260
141724822: call 0x142923c00
141724827: mov eax, r14d
14172482a: and eax, 3
14172482d: mov esi, 2
141724832: cmp eax, 1
141724835: jne 0x1417248a0
141724837: lea rax, [r14 - 1]
14172483b: mov qword ptr [rbx + 0xa8], rax
141724842: mov rax, qword ptr [r14 - 1]
141724846: mov qword ptr [rbx + 0x80], rax
14172484d: mov rax, qword ptr [r14 + 7]
141724851: mov qword ptr [rbx + 0x98], rax
141724858: mov rax, qword ptr [rax]
14172485b: test rax, rax
14172485e: je 0x141724869
141724860: mov rcx, qword ptr [rbx + 0x80]
141724867: call rax
141724869: mov rcx, qword ptr [rbx + 0x80]
141724870: mov rax, qword ptr [rbx + 0x98]
141724877: mov rdx, qword ptr [rax + 8]
14172487b: test rdx, rdx
14172487e: je 0x141724889
141724880: mov r8, qword ptr [rax + 0x10]
141724884: call 0x140613c20
141724889: mov edx, 0x18
14172488e: mov r8d, 8
141724894: mov rcx, qword ptr [rbx + 0xa8]
14172489b: call 0x140613c20
1417248a0: mov qword ptr [rbx + 0x10a8], r14
1417248a7: lea rcx, [rbx + 0x10b0]
1417248ae: lea r14, [rbx + 0x200]
1417248b5: mov r8d, 0x260
1417248bb: mov rdx, r14
1417248be: call 0x142923c00
1417248c3: lea rcx, [rbx + 0xbc8]
1417248ca: lea rdx, [rbx + 0x200]
1417248d1: mov r8d, 0x268
1417248d7: call 0x142923c00
1417248dc: lea rcx, [rbx + 0xe38]
1417248e3: lea r12, [rbx + 0x480]
1417248ea: mov r8d, 0x268
1417248f0: mov rdx, r12
1417248f3: call 0x142923c00
1417248f8: lea rcx, [rbx + 0x708]
1417248ff: lea rdx, [rbx + 0x200]
141724906: mov r8d, 0x258
14172490c: call 0x142923c00
141724911: lea rcx, [rbx + 0x968]
141724918: lea rdx, [rbx + 0x480]
14172491f: mov r8d, 0x258
141724925: call 0x142923c00
14172492a: mov qword ptr [rbx + 0x700], 0
141724935: mov qword ptr [rbx + 0x960], 0
141724940: lea r13, [rbx + 0xbc0]
141724947: mov qword ptr [rbx + 0xbc0], 2
141724952: mov qword ptr [rbx + 0xe30], 2
14172495d: lea r15, [rbx + 0x10a0]
141724964: mov qword ptr [rbx + 0x10a0], rsi
14172496b: mov edi, 2
141724970: lea rsi, [rbx + 0x1340]
141724977: jmp 0x14172499e
141724979: nop dword ptr [rax]
141724980: mov edx, 0x18
141724985: mov r8d, 8
14172498b: mov rcx, qword ptr [rbx + 0xa8]
141724992: call 0x140613c20
141724997: mov rdi, qword ptr [rbx + 0xbc0]
14172499e: cmp rdi, 3
1417249a2: jne 0x1417249d3
1417249a4: jmp 0x141724eba
1417249a9: nop dword ptr [rax]
1417249b0: mov rdi, qword ptr [rbx + 0x80]
1417249b7: mov qword ptr [rbx + 0xbc0], rdi
1417249be: mov r8d, 0x268
1417249c4: lea rcx, [rbx + 0xbc8]
1417249cb: mov rdx, r14
1417249ce: call 0x142923c00
1417249d3: cmp rdi, 2
1417249d7: je 0x141724b00
1417249dd: mov rcx, r12
1417249e0: mov rdx, r13
1417249e3: call 0x140d59e80
1417249e8: test byte ptr [rbx + 0x480], 1
1417249ef: jne 0x141724a32
1417249f1: mov rax, qword ptr [rbx + 0xbc0]
1417249f8: test rax, rax
1417249fb: je 0x141724a0e
1417249fd: cmp eax, 2
141724a00: je 0x141724a27
141724a02: lea rcx, [rbx + 0xbc8]
141724a09: call 0x140d685b0
141724a0e: mov rax, qword ptr [rbx + 0xbd0]
141724a15: lock dec qword ptr [rax]
141724a19: jne 0x141724a27
141724a1b: lea rcx, [rbx + 0xbd0]
141724a22: call 0x140d53bc0
141724a27: mov qword ptr [rbx + 0xbc0], 2
141724a32: mov r8d, 0x260
141724a38: mov rcx, rsi
141724a3b: mov rdx, r12
141724a3e: call 0x142923c00
141724a43: mov rax, qword ptr [rbx + 0x1340]
141724a4a: test al, 1
141724a4c: jne 0x141724bc0
141724a52: test rax, rax
141724a55: je 0x141724b00
141724a5b: mov rax, qword ptr [rbx + 0x1348]
141724a62: test rax, rax
141724a65: je 0x141724a82
141724a67: lock dec qword ptr [rax]
141724a6b: jne 0x141724b00
141724a71: lea rcx, [rbx + 0x1348]
141724a78: call 0x140d53bc0
141724a7d: jmp 0x141724b00
141724a82: mov rax, qword ptr [rbx + 0x1350]
141724a89: mov ecx, eax
141724a8b: and ecx, 3
141724a8e: cmp ecx, 1
141724a91: jne 0x141724b00
141724a93: lea rcx, [rax - 1]
141724a97: mov qword ptr [rbx + 0xa8], rcx
141724a9e: mov rcx, qword ptr [rax - 1]
141724aa2: mov qword ptr [rbx + 0x80], rcx
141724aa9: mov rax, qword ptr [rax + 7]
141724aad: mov qword ptr [rbx + 0x98], rax
141724ab4: mov rax, qword ptr [rax]
141724ab7: test rax, rax
141724aba: je 0x141724ac5
141724abc: mov rcx, qword ptr [rbx + 0x80]
141724ac3: call rax
141724ac5: mov rcx, qword ptr [rbx + 0x80]
141724acc: mov rax, qword ptr [rbx + 0x98]
141724ad3: mov rdx, qword ptr [rax + 8]
141724ad7: test rdx, rdx
141724ada: je 0x141724ae5
141724adc: mov r8, qword ptr [rax + 0x10]
141724ae0: call 0x140613c20
141724ae5: mov edx, 0x18
141724aea: mov r8d, 8
141724af0: mov rcx, qword ptr [rbx + 0xa8]
141724af7: call 0x140613c20
141724afc: nop dword ptr [rax]
141724b00: mov rax, qword ptr [rbx + 0x10a0]
141724b07: mov rcx, rsi
141724b0a: cmp rax, 3
141724b0e: je 0x141724b2f
141724b10: mov qword ptr [rbx + 0x1340], rax
141724b17: mov r8d, 0x268
141724b1d: mov rcx, r14
141724b20: lea rdx, [rbx + 0x10a8]
141724b27: call 0x142923c00
141724b2c: mov rcx, r15
141724b2f: mov qword ptr [rcx], 2
141724b36: mov rax, qword ptr [rbx + 0x1340]
141724b3d: cmp rax, 2
141724b41: je 0x141724b90
141724b43: mov qword ptr [rbx + 0x80], rax
141724b4a: mov rax, qword ptr [rbx + 0xbc0]
141724b51: test rax, rax
141724b54: je 0x141724b6b
141724b56: cmp eax, 2
141724b59: je 0x1417249b0
141724b5f: lea rcx, [rbx + 0xbc8]
141724b66: call 0x140d685b0
141724b6b: mov rax, qword ptr [rbx + 0xbd0]
141724b72: lock dec qword ptr [rax]
141724b76: jne 0x1417249b0
141724b7c: lea rcx, [rbx + 0xbd0]
141724b83: call 0x140d53bc0
141724b88: jmp 0x1417249b0
141724b8d: nop dword ptr [rax]
141724b90: cmp dword ptr [rbx + 0xe30], 2
141724b97: jne 0x141724be0
141724b99: mov qword ptr [rbx + 0x15a0], 0
141724ba4: mov rax, qword ptr [rbx + 0x15a0]
141724bab: test al, 1
141724bad: jne 0x141724c5d
141724bb3: jmp 0x141724e89
141724bb8: nop dword ptr [rax + rax]
141724bc0: mov r8d, 0x260
141724bc6: lea rcx, [rbx + 0x15a0]
141724bcd: mov rdx, rsi
141724bd0: jmp 0x141724c49
141724bd2: nop word ptr cs:[rax + rax]
141724be0: mov rcx, r12
141724be3: lea rdx, [rbx + 0xe30]
141724bea: call 0x140d59e80
141724bef: test byte ptr [rbx + 0x480], 1
141724bf6: jne 0x141724c39
141724bf8: mov rax, qword ptr [rbx + 0xe30]
141724bff: test rax, rax
141724c02: je 0x141724c15
141724c04: cmp eax, 2
141724c07: je 0x141724c2e
141724c09: lea rcx, [rbx + 0xe38]
141724c10: call 0x140d685b0
141724c15: mov rax, qword ptr [rbx + 0xe40]
141724c1c: lock dec qword ptr [rax]
141724c20: jne 0x141724c2e
141724c22: lea rcx, [rbx + 0xe40]
141724c29: call 0x140d53bc0
141724c2e: mov qword ptr [rbx + 0xe30], 2
141724c39: mov r8d, 0x260
141724c3f: lea rcx, [rbx + 0x15a0]
141724c46: mov rdx, r12
141724c49: call 0x142923c00
141724c4e: mov rax, qword ptr [rbx + 0x15a0]
141724c55: test al, 1
141724c57: je 0x141724e89
141724c5d: mov r14, r12
141724c60: mov r12, r13
141724c63: mov r13, r15
141724c66: mov rdi, qword ptr [rbx + 0x15a8]
141724c6d: test rdi, rdi
141724c70: jne 0x141724cf6
141724c76: mov rax, qword ptr [rbx + 0x15b0]
141724c7d: mov ecx, eax
141724c7f: and ecx, 3
141724c82: cmp ecx, 1
141724c85: mov r15, r13
141724c88: mov r13, r12
141724c8b: mov r12, r14
141724c8e: lea r14, [rbx + 0x200]
141724c95: jne 0x141724997
141724c9b: lea rcx, [rax - 1]
141724c9f: mov qword ptr [rbx + 0xa8], rcx
141724ca6: mov rcx, qword ptr [rax - 1]
141724caa: mov qword ptr [rbx + 0x80], rcx
141724cb1: mov rax, qword ptr [rax + 7]
141724cb5: mov qword ptr [rbx + 0x98], rax
141724cbc: mov rax, qword ptr [rax]
141724cbf: test rax, rax
141724cc2: je 0x141724ccd
141724cc4: mov rcx, qword ptr [rbx + 0x80]
141724ccb: call rax
141724ccd: mov rcx, qword ptr [rbx + 0x80]
141724cd4: mov rax, qword ptr [rbx + 0x98]
141724cdb: mov rdx, qword ptr [rax + 8]
141724cdf: test rdx, rdx
141724ce2: je 0x141724980
141724ce8: mov r8, qword ptr [rax + 0x10]
141724cec: call 0x140613c20
141724cf1: jmp 0x141724980
141724cf6: mov r8d, 0x250
141724cfc: lea rcx, [rbx + 0x208]
141724d03: lea rdx, [rbx + 0x15b0]
141724d0a: call 0x142923c00
141724d0f: mov qword ptr [rbx + 0x200], rdi
141724d16: mov rdi, r14
141724d19: mov rcx, r14
141724d1c: lea rdx, [rbx + 0x200]
141724d23: call 0x140d59f10
141724d28: mov byte ptr [rbx + 0xb5], 1
141724d2f: mov rcx, rdi
141724d32: call 0x140d76ed0
141724d37: mov r15, r13
141724d3a: mov r13, r12
141724d3d: mov byte ptr [rbx + 0xb5], 1
141724d44: mov rcx, rax
141724d47: call 0x140d627a0
141724d4c: test rax, rax
141724d4f: je 0x141724d8f
141724d51: mov r8, rdx
141724d54: mov byte ptr [rbx + 0xb5], 1
141724d5b: mov rcx, rsi
141724d5e: mov rdx, rax
141724d61: call 0x140d58e70
141724d66: test byte ptr [rbx + 0x1340], 1
141724d6d: jne 0x141724d8f
141724d6f: cmp qword ptr [rbx + 0x1350], 8
141724d77: jne 0x141724d8f
141724d79: mov rax, qword ptr [rbx + 0x1348]
141724d80: movabs rcx, 0x746f687370616e73
141724d8a: cmp qword ptr [rax], rcx
141724d8d: je 0x141724dd8
141724d8f: mov rdx, qword ptr [rbx + 0x480]
141724d96: test rdx, rdx
141724d99: je 0x141724dad
141724d9b: mov rcx, qword ptr [rbx + 0x488]
141724da2: mov r8d, 1
141724da8: call 0x140613c20
141724dad: mov rax, qword ptr [rbx + 0x200]
141724db4: lock dec qword ptr [rax]
141724db8: mov r12, r14
141724dbb: lea r14, [rbx + 0x200]
141724dc2: jne 0x141724dcc
141724dc4: mov rcx, r14
141724dc7: call 0x140d53bc0
141724dcc: mov rdi, qword ptr [rbx + 0xbc0]
141724dd3: jmp 0x14172499e
141724dd8: mov rax, qword ptr [rbx + 0x480]
141724ddf: mov qword ptr [rbx + 0x80], rax
141724de6: mov rax, qword ptr [rbx + 0x488]
141724ded: mov qword ptr [rbx + 0x98], rax
141724df4: mov r12, r14
141724df7: movups xmm0, xmmword ptr [r14 + 0x10]
141724dfc: movaps xmmword ptr [rbx + 0x1340], xmm0
141724e03: mov rdi, qword ptr [rbx + 0x130]
141724e0a: cmp rdi, qword ptr [rbx + 0x120]
141724e11: jne 0x141724e26
141724e13: lea rcx, [rbx + 0x120]
141724e1a: lea rdx, [rip + 0x15a613f]
141724e21: call 0x140562c30
141724e26: mov rax, qword ptr [rbx + 0x128]
141724e2d: mov rcx, rdi
141724e30: shl rcx, 5
141724e34: mov rdx, qword ptr [rbx + 0x80]
141724e3b: mov qword ptr [rax + rcx], rdx
141724e3f: mov rdx, qword ptr [rbx + 0x98]
141724e46: mov qword ptr [rax + rcx + 8], rdx
141724e4b: movaps xmm0, xmmword ptr [rbx + 0x1340]
141724e52: movups xmmword ptr [rax + rcx + 0x10], xmm0
141724e57: inc rdi
141724e5a: mov qword ptr [rbx + 0x130], rdi
141724e61: mov rax, qword ptr [rbx + 0x200]
141724e68: lock dec qword ptr [rax]
141724e6c: lea r14, [rbx + 0x200]
141724e73: jne 0x141724e7d
141724e75: mov rcx, r14
141724e78: call 0x140d53bc0
141724e7d: mov rdi, qword ptr [rbx + 0xbc0]
141724e84: jmp 0x14172499e
141724e89: test rax, rax
141724e8c: je 0x141724eba
141724e8e: mov rax, qword ptr [rbx + 0x15a8]
141724e95: test rax, rax
141724e98: je 0x141724eae
141724e9a: lock dec qword ptr [rax]
141724e9e: jne 0x141724eba
141724ea0: lea rcx, [rbx + 0x15a8]
141724ea7: call 0x140d53bc0
141724eac: jmp 0x141724eba
141724eae: lea rcx, [rbx + 0x15b0]
141724eb5: call 0x1405e7a00
141724eba: lea rcx, [rbx + 0x700]
141724ec1: call 0x1408802e0
141724ec6: mov rdx, qword ptr [rbx + 0x140]
141724ecd: test rdx, rdx
141724ed0: je 0x141724ee4
141724ed2: mov rcx, qword ptr [rbx + 0x148]
141724ed9: mov r8d, 1
141724edf: call 0x140613c20
141724ee4: movaps xmm0, xmmword ptr [rbx + 0x1e0]
141724eeb: movaps xmmword ptr [rbx + 0x140], xmm0
141724ef2: mov qword ptr [rbx + 0x15a0], 0
141724efd: mov qword ptr [rbx + 0x15a8], 1
141724f08: xorps xmm0, xmm0
141724f0b: movups xmmword ptr [rbx + 0x15b0], xmm0
141724f12: mov byte ptr [rbx + 0xb2], 1
141724f19: call 0x140d635e0
141724f1e: mov r15, qword ptr [rbx + 0xb8]
141724f25: mov r8, qword ptr [r15 + 0x48]
141724f29: mov r9d, dword ptr [r15 + 0xc0]
141724f30: lea rax, [r15 + 0xc4]
141724f37: lea rcx, [r15 + 0x94]
141724f3e: mov edx, dword ptr [r15 + 0xe8]
141724f45: mov r10d, dword ptr [r15 + 0xec]
141724f4c: mov r11, qword ptr [r15 + 8]
141724f50: mov rsi, qword ptr [r15 + 0x10]
141724f54: lea rsi, [rsi + rsi*4]
141724f58: lea rsi, [r11 + rsi*8]
141724f5c: movups xmm0, xmmword ptr [r15 + 0x20]
141724f61: lea rdi, [r15 + 0x30]
141724f65: xor r14d, r14d
141724f68: cmp r14, qword ptr [r15 + 0x30]
141724f6c: cmovo rdi, r14
141724f70: lea r14, [r15 + 0x50]
141724f74: mov byte ptr [rbx + 0xb2], 1
141724f7b: mov qword ptr [rsp + 0x68], r14
141724f80: mov qword ptr [rsp + 0x60], rdi
141724f85: movups xmmword ptr [rsp + 0x50], xmm0
141724f8a: mov qword ptr [rsp + 0x48], rsi
141724f8f: mov qword ptr [rsp + 0x40], r11
141724f94: mov dword ptr [rsp + 0x38], r10d
141724f99: mov dword ptr [rsp + 0x30], edx
141724f9d: mov qword ptr [rsp + 0x28], rcx
141724fa2: mov qword ptr [rsp + 0x20], rax
141724fa7: lea rcx, [rbx + 0x700]
141724fae: lea rdx, [rbx + 0x15a0]
141724fb5: call 0x141750c90
141724fba: mov rdi, qword ptr [rbx + 0x700]
141724fc1: mov rsi, qword ptr [rbx + 0x708]
141724fc8: mov rax, rdi
141724fcb: neg rax
141724fce: jo 0x14172721a
141724fd4: mov rax, qword ptr [rbx + 0x740]
141724fdb: mov qword ptr [rbx + 0x1380], rax
141724fe2: movups xmm0, xmmword ptr [rbx + 0x710]
141724fe9: movups xmm1, xmmword ptr [rbx + 0x720]
141724ff0: movups xmm2, xmmword ptr [rbx + 0x730]
141724ff7: movups xmmword ptr [rbx + 0x1370], xmm2
141724ffe: movups xmmword ptr [rbx + 0x1360], xmm1
141725005: movups xmmword ptr [rbx + 0x1350], xmm0
14172500c: mov qword ptr [rbx + 0x1340], rdi
141725013: mov qword ptr [rbx + 0x1348], rsi
14172501a: mov rax, qword ptr [rbx + 0x15b0]
141725021: mov qword ptr [rbx + 0x210], rax
141725028: movups xmm0, xmmword ptr [rbx + 0x15a0]
14172502f: movaps xmmword ptr [rbx + 0x200], xmm0
141725036: lea rcx, [rbx + 0x700]
14172503d: lea rdx, [rbx + 0x140]
141725044: call 0x140b46000
141725049: mov byte ptr [rbx + 0xc7], 1
141725050: lea rcx, [rbx + 0x700]
141725057: call 0x140d76ed0
14172505c: mov r9, rdx
14172505f: mov byte ptr [rbx + 0xc7], 0
141725066: lea rcx, [rip + 0x15a563b]
14172506d: mov qword ptr [rsp + 0x20], rcx
141725072: mov qword ptr [rsp + 0x28], 0xc
14172507b: lea rcx, [rbx + 0x480]
141725082: lea rdx, [rbx + 0x200]
141725089: mov r8, rax
14172508c: call 0x1409184d0
141725091: lea r14, [rbx + 0x1350]
141725098: mov rdx, qword ptr [rbx + 0x700]
14172509f: test rdx, rdx
1417250a2: je 0x1417250b6
1417250a4: mov rcx, qword ptr [rbx + 0x708]
1417250ab: mov r8d, 1
1417250b1: call 0x140613c20
1417250b6: mov rax, qword ptr [r14 + 0x30]
1417250ba: mov qword ptr [rbx + 0x730], rax
1417250c1: movups xmm0, xmmword ptr [r14]
1417250c5: movups xmm1, xmmword ptr [r14 + 0x10]
1417250ca: movups xmm2, xmmword ptr [r14 + 0x20]
1417250cf: movaps xmmword ptr [rbx + 0x720], xmm2
1417250d6: movaps xmmword ptr [rbx + 0x710], xmm1
1417250dd: movaps xmmword ptr [rbx + 0x700], xmm0
1417250e4: movups xmm0, xmmword ptr [rbx + 0x480]
1417250eb: movups xmm1, xmmword ptr [rbx + 0x490]
1417250f2: movups xmm2, xmmword ptr [rbx + 0x4a0]
1417250f9: movups xmm3, xmmword ptr [rbx + 0x4b0]
141725100: movups xmmword ptr [rbx + 0x738], xmm0
141725107: movups xmmword ptr [rbx + 0x748], xmm1
14172510e: movups xmmword ptr [rbx + 0x758], xmm2
141725115: movups xmmword ptr [rbx + 0x768], xmm3
14172511c: movups xmm0, xmmword ptr [rbx + 0x4c0]
141725123: movups xmmword ptr [rbx + 0x778], xmm0
14172512a: movups xmm0, xmmword ptr [rbx + 0x4d0]
141725131: movups xmmword ptr [rbx + 0x788], xmm0
141725138: movups xmm0, xmmword ptr [rbx + 0x4e0]
14172513f: movups xmmword ptr [rbx + 0x798], xmm0
141725146: movups xmm0, xmmword ptr [rbx + 0x4f0]
14172514d: movups xmmword ptr [rbx + 0x7a8], xmm0
141725154: lea rcx, [rbx + 0x200]
14172515b: lea rdx, [rbx + 0x700]
141725162: mov r8d, 0xb8
141725168: call 0x142923c00
14172516d: mov qword ptr [rbx + 0x1340], rdi
141725174: mov qword ptr [rbx + 0x1348], rsi
14172517b: movups xmm0, xmmword ptr [rbx + 0x200]
141725182: movups xmm1, xmmword ptr [rbx + 0x210]
141725189: movups xmm2, xmmword ptr [rbx + 0x220]
141725190: movups xmmword ptr [rbx + 0x1350], xmm0
141725197: movups xmmword ptr [rbx + 0x1360], xmm1
14172519e: movups xmmword ptr [rbx + 0x1370], xmm2
1417251a5: mov rax, qword ptr [rbx + 0x230]
1417251ac: mov qword ptr [rbx + 0x1380], rax
1417251b3: movups xmm0, xmmword ptr [rbx + 0x2a8]
1417251ba: movaps xmmword ptr [rbx + 0x4f0], xmm0
1417251c1: movups xmm0, xmmword ptr [rbx + 0x298]
1417251c8: movaps xmmword ptr [rbx + 0x4e0], xmm0
1417251cf: movups xmm0, xmmword ptr [rbx + 0x288]
1417251d6: movaps xmmword ptr [rbx + 0x4d0], xmm0
1417251dd: movups xmm0, xmmword ptr [rbx + 0x278]
1417251e4: movaps xmmword ptr [rbx + 0x4c0], xmm0
1417251eb: movups xmm0, xmmword ptr [rbx + 0x268]
1417251f2: movaps xmmword ptr [rbx + 0x4b0], xmm0
1417251f9: movups xmm0, xmmword ptr [rbx + 0x258]
141725200: movaps xmmword ptr [rbx + 0x4a0], xmm0
141725207: movups xmm0, xmmword ptr [rbx + 0x248]
14172520e: movaps xmmword ptr [rbx + 0x490], xmm0
141725215: movups xmm0, xmmword ptr [rbx + 0x238]
14172521c: movaps xmmword ptr [rbx + 0x480], xmm0
141725223: mov rsi, qword ptr [rbx + 0x110]
14172522a: mov qword ptr [rbx + 0x180], rsi
141725231: movaps xmm0, xmmword ptr [rbx + 0x100]
141725238: movaps xmmword ptr [rbx + 0x170], xmm0
14172523f: mov r13, qword ptr [rbx + 0x178]
141725246: call 0x140d4d740
14172524b: mov rdi, rax
14172524e: test byte ptr [rax], 1
141725251: je 0x141725264
141725253: mov rcx, rdi
141725256: add rcx, 8
14172525a: mov rax, qword ptr [rdi + 8]
14172525e: mov rdx, qword ptr [rdi + 0x10]
141725262: jmp 0x141725286
141725264: mov byte ptr [rbx + 0xb3], 1
14172526b: call 0x140d64e00
141725270: mov rcx, rdi
141725273: add rcx, 8
141725277: mov qword ptr [rdi], 1
14172527e: mov qword ptr [rdi + 8], rax
141725282: mov qword ptr [rdi + 0x10], rdx
141725286: lea r8, [rax + 1]
14172528a: mov qword ptr [rcx], r8
14172528d: movups xmm6, xmmword ptr [rip + 0x15a4d1c]
141725294: movaps xmmword ptr [rbx + 0x710], xmm6
14172529b: movups xmm7, xmmword ptr [rip + 0x15a4cfe]
1417252a2: movaps xmmword ptr [rbx + 0x700], xmm7
1417252a9: mov qword ptr [rbx + 0x720], rax
1417252b0: mov qword ptr [rbx + 0x728], rdx
1417252b7: test rsi, rsi
1417252ba: jne 0x141726e37
1417252c0: movaps xmm0, xmmword ptr [rbx + 0x700]
1417252c7: movaps xmm1, xmmword ptr [rbx + 0x710]
1417252ce: mov rax, qword ptr [rbx + 0x720]
1417252d5: mov qword ptr [rbx + 0x160], rax
1417252dc: mov rax, qword ptr [rbx + 0x728]
1417252e3: mov qword ptr [rbx + 0x168], rax
1417252ea: movaps xmmword ptr [rbx + 0x150], xmm1
1417252f1: movaps xmmword ptr [rbx + 0x140], xmm0
1417252f8: mov rax, qword ptr [rbx + 0xb8]
1417252ff: mov r14, qword ptr [rax + 0x20]
141725303: mov rsi, qword ptr [rax + 0x28]
141725307: call 0x140d4d740
14172530c: mov rdi, rax
14172530f: test byte ptr [rax], 1
141725312: je 0x141725325
141725314: mov rcx, rdi
141725317: add rcx, 8
14172531b: mov rax, qword ptr [rdi + 8]
14172531f: mov rdx, qword ptr [rdi + 0x10]
141725323: jmp 0x141725347
141725325: mov byte ptr [rbx + 0xa7], 1
14172532c: call 0x140d64e00
141725331: mov rcx, rdi
141725334: add rcx, 8
141725338: mov qword ptr [rdi], 1
14172533f: mov qword ptr [rdi + 8], rax
141725343: mov qword ptr [rdi + 0x10], rdx
141725347: lea r8, [rax + 1]
14172534b: mov qword ptr [rcx], r8
14172534e: movaps xmmword ptr [rbx + 0x710], xmm6
141725355: movaps xmmword ptr [rbx + 0x700], xmm7
14172535c: mov qword ptr [rbx + 0x720], rax
141725363: mov qword ptr [rbx + 0x728], rdx
14172536a: test rsi, rsi
14172536d: jne 0x141726ea0
141725373: movaps xmm0, xmmword ptr [rbx + 0x700]
14172537a: movaps xmm1, xmmword ptr [rbx + 0x710]
141725381: mov rax, qword ptr [rbx + 0x720]
141725388: mov qword ptr [rbx + 0x15c0], rax
14172538f: mov rax, qword ptr [rbx + 0x728]
141725396: mov qword ptr [rbx + 0x15c8], rax
14172539d: movaps xmmword ptr [rbx + 0x15b0], xmm1
1417253a4: movaps xmmword ptr [rbx + 0x15a0], xmm0
1417253ab: mov qword ptr [rbx + 0x880], 1
1417253b6: mov qword ptr [rbx + 0x888], 1
1417253c1: mov byte ptr [rbx + 0x890], 0
1417253c8: xorps xmm0, xmm0
1417253cb: movaps xmmword ptr [rbx + 0x700], xmm0
1417253d2: movaps xmmword ptr [rbx + 0x780], xmm0
1417253d9: mov word ptr [rbx + 0x800], 0
1417253e2: mov qword ptr [rbx + 0x808], 0
1417253ed: mov qword ptr [rbx + 0x810], 8
1417253f8: movups xmmword ptr [rbx + 0x818], xmm0
1417253ff: mov qword ptr [rbx + 0x828], 8
14172540a: mov qword ptr [rbx + 0x830], 0
141725415: mov byte ptr [rbx + 0x838], 1
14172541c: movzx eax, byte ptr [rip + 0x1f0f7e6]
141725423: mov ecx, 0x200
141725428: mov edx, 0x80
14172542d: call 0x140613c10
141725432: test rax, rax
141725435: je 0x1417272b7
14172543b: mov rsi, rax
14172543e: lea rdx, [rbx + 0x700]
141725445: mov r8d, 0x200
14172544b: mov rcx, rax
14172544e: call 0x142923c00
141725453: mov qword ptr [rbx + 0x1b8], 1
14172545e: mov qword ptr [rbx + 0x1c0], rsi
141725465: mov qword ptr [rbx + 0xf0], 1
141725470: mov qword ptr [rbx + 0xf8], rsi
141725477: movabs rdi, 0x8000000000000000
141725481: mov qword ptr [rbx + 0x710], rdi
141725488: mov qword ptr [rbx + 0x700], 0
141725493: mov byte ptr [rbx + 0x728], 0
14172549a: movzx eax, byte ptr [rip + 0x1f0f768]
1417254a1: mov ecx, 0xb
1417254a6: mov edx, 1
1417254ab: call 0x140613c10
1417254b0: test rax, rax
1417254b3: mov r15, qword ptr [rbx + 0xd0]
1417254ba: mov qword ptr [rbx + 0x80], rax
1417254c1: je 0x1417272cb
1417254c7: mov r9, rax
1417254ca: movabs rax, 0x746f687370616e73
1417254d4: mov qword ptr [r9], rax
1417254d7: mov dword ptr [r9 + 7], 0x63672074
1417254df: mov qword ptr [rbx + 0xd8], 0xb
1417254ea: mov qword ptr [rbx + 0xe0], r9
1417254f1: mov qword ptr [rbx + 0xe8], 0xb
1417254fc: lea rcx, [rbx + 0x200]
141725503: lea rdx, [rbx + 0x700]
14172550a: lea r8, [rbx + 0xd8]
141725511: call 0x140d558f0
141725516: mov rax, qword ptr [rbx + 0x180]
14172551d: mov qword ptr [rbx + 0x720], rax
141725524: movaps xmm0, xmmword ptr [rbx + 0x170]
14172552b: movups xmmword ptr [rbx + 0x710], xmm0
141725532: movups xmm0, xmmword ptr [rbx + 0x120]
141725539: movups xmmword ptr [rbx + 0x728], xmm0
141725540: mov rax, qword ptr [rbx + 0x130]
141725547: mov qword ptr [rbx + 0x738], rax
14172554e: movaps xmm0, xmmword ptr [rbx + 0x15a0]
141725555: movaps xmm1, xmmword ptr [rbx + 0x15b0]
14172555c: movaps xmm2, xmmword ptr [rbx + 0x15c0]
141725563: movups xmmword ptr [rbx + 0x740], xmm0
14172556a: movups xmmword ptr [rbx + 0x750], xmm1
141725571: movups xmmword ptr [rbx + 0x760], xmm2
141725578: mov qword ptr [rbx + 0x700], 1
141725583: mov qword ptr [rbx + 0x708], rsi
14172558a: mov byte ptr [rbx + 0x96], 0
141725591: lea rcx, [rbx + 0xd8]
141725598: lea rdx, [rbx + 0x200]
14172559f: lea r8, [rbx + 0x700]
1417255a6: call 0x140a6e0b0
1417255ab: mov r14, qword ptr [rbx + 0xd8]
1417255b2: test r14, r14
1417255b5: je 0x1417255fc
1417255b7: mov rcx, qword ptr [rbx + 0xe8]
1417255be: call qword ptr [rip + 0x1a6ce2c]
1417255c4: lock dec qword ptr [r14]
1417255c8: jne 0x1417255d6
1417255ca: lea rcx, [rbx + 0xd8]
1417255d1: call 0x140d53b60
1417255d6: mov rax, qword ptr [rbx + 0xe0]
1417255dd: lock dec qword ptr [rax]
1417255e1: jne 0x141725676
1417255e7: lea rcx, [rbx + 0xe0]
1417255ee: mov byte ptr [rbx + 0x96], 0
1417255f5: call 0x140d53db0
1417255fa: jmp 0x141725676
1417255fc: mov rax, qword ptr [rbx + 0xe0]
141725603: mov ecx, eax
141725605: and ecx, 3
141725608: cmp ecx, 1
14172560b: jne 0x141725676
14172560d: lea rcx, [rax - 1]
141725611: mov qword ptr [rbx + 0xa8], rcx
141725618: mov rcx, qword ptr [rax - 1]
14172561c: mov qword ptr [rbx + 0x80], rcx
141725623: mov rax, qword ptr [rax + 7]
141725627: mov qword ptr [rbx + 0x98], rax
14172562e: mov rax, qword ptr [rax]
141725631: test rax, rax
141725634: je 0x14172563f
141725636: mov rcx, qword ptr [rbx + 0x80]
14172563d: call rax
14172563f: mov rcx, qword ptr [rbx + 0x80]
141725646: mov rax, qword ptr [rbx + 0x98]
14172564d: mov rdx, qword ptr [rax + 8]
141725651: test rdx, rdx
141725654: je 0x14172565f
141725656: mov r8, qword ptr [rax + 0x10]
14172565a: call 0x140613c20
14172565f: mov edx, 0x18
141725664: mov r8d, 8
14172566a: mov rcx, qword ptr [rbx + 0xa8]
141725671: call 0x140613c20
141725676: mov qword ptr [r15], 1
14172567d: mov qword ptr [r15 + 8], rsi
141725681: movaps xmm0, xmmword ptr [rbx + 0x480]
141725688: movaps xmm1, xmmword ptr [rbx + 0x490]
14172568f: movaps xmm2, xmmword ptr [rbx + 0x4a0]
141725696: movaps xmm3, xmmword ptr [rbx + 0x4b0]
14172569d: movups xmmword ptr [r15 + 0x10], xmm0
1417256a2: movups xmmword ptr [r15 + 0x20], xmm1
1417256a7: movups xmmword ptr [r15 + 0x30], xmm2
1417256ac: movups xmmword ptr [r15 + 0x40], xmm3
1417256b1: movaps xmm0, xmmword ptr [rbx + 0x4c0]
1417256b8: movups xmmword ptr [r15 + 0x50], xmm0
1417256bd: movaps xmm0, xmmword ptr [rbx + 0x4d0]
1417256c4: movups xmmword ptr [r15 + 0x60], xmm0
1417256c9: movaps xmm0, xmmword ptr [rbx + 0x4e0]
1417256d0: movups xmmword ptr [r15 + 0x70], xmm0
1417256d5: movaps xmm0, xmmword ptr [rbx + 0x4f0]
1417256dc: movups xmmword ptr [r15 + 0x80], xmm0
1417256e4: movups xmm0, xmmword ptr [rbx + 0x1340]
1417256eb: movups xmm1, xmmword ptr [rbx + 0x1350]
1417256f2: movups xmm2, xmmword ptr [rbx + 0x1360]
1417256f9: movups xmm3, xmmword ptr [rbx + 0x1370]
141725700: movups xmmword ptr [r15 + 0x90], xmm0
141725708: movups xmmword ptr [r15 + 0xa0], xmm1
141725710: movups xmmword ptr [r15 + 0xb0], xmm2
141725718: movups xmmword ptr [r15 + 0xc0], xmm3
141725720: mov rax, qword ptr [rbx + 0x1380]
141725727: mov qword ptr [r15 + 0xd0], rax
14172572e: mov qword ptr [r15 + 0xd8], rdi
141725735: mov qword ptr [r15 + 0x100], rdi
14172573c: mov qword ptr [r15 + 0x128], rdi
141725743: mov qword ptr [r15 + 0x150], rdi
14172574a: mov qword ptr [r15 + 0x178], rdi
141725751: mov qword ptr [r15 + 0x1a0], rdi
141725758: mov qword ptr [r15 + 0x1c8], rdi
14172575f: mov qword ptr [r15 + 0x1f0], rdi
141725766: mov qword ptr [r15 + 0x218], rdi
14172576d: mov qword ptr [r15 + 0x240], rdi
141725774: movaps xmm0, xmmword ptr [rbx + 0x140]
14172577b: movaps xmm1, xmmword ptr [rbx + 0x150]
141725782: movaps xmm2, xmmword ptr [rbx + 0x160]
141725789: movups xmmword ptr [r15 + 0x288], xmm2
141725791: movups xmmword ptr [r15 + 0x278], xmm1
141725799: movups xmmword ptr [r15 + 0x268], xmm0
1417257a1: movaps xmm0, xmmword ptr [rbx + 0x1e0]
1417257a8: movups xmmword ptr [r15 + 0x298], xmm0
1417257b0: mov rcx, qword ptr [rbx + 0xb8]
1417257b7: call 0x14088eb40
1417257bc: mov edx, 0xf0
1417257c1: mov r8d, 8
1417257c7: mov rcx, qword ptr [rbx + 0xb8]
1417257ce: call 0x140613c20
1417257d3: cmp qword ptr [rbx + 0x1f0], 0
1417257db: je 0x141726c0b
1417257e1: call 0x140c18130
1417257e6: mov rcx, rax
1417257e9: mov rax, qword ptr [rax]
1417257ec: test rax, rax
1417257ef: je 0x141725800
1417257f1: cmp eax, 1
1417257f4: jne 0x1417272e6
1417257fa: add rcx, 8
1417257fe: jmp 0x141725817
141725800: mov byte ptr [rbx + 0x7f], 0
141725804: xor edx, edx
141725806: call 0x1428f87d0
14172580b: mov rcx, rax
14172580e: test rax, rax
141725811: je 0x1417272e6
141725817: cmp qword ptr [rcx], 0
14172581b: jne 0x141726f2f
141725821: mov qword ptr [rcx], 0xffffffffffffffff
141725828: mov qword ptr [rbx + 0x80], rcx
14172582f: add rcx, 8
141725833: mov rdx, qword ptr [rbx + 0x1f8]
14172583a: call 0x140c170f0
14172583f: jmp 0x141726c01
141725844: mov r15b, 1
141725847: jmp 0x14172584c
141725849: xor r15d, r15d
14172584c: test byte ptr [rbx + 0x1d0], 1
141725853: mov dword ptr [rbx + 0xb8], r15d
14172585a: je 0x141725e4b
141725860: mov eax, dword ptr [rbx + 0x1d4]
141725866: mov dword ptr [rbx + 0x480], eax
14172586c: mov byte ptr [rbx + 0x7c], r15b
141725870: lea rcx, [rbx + 0x140]
141725877: call 0x140d76ed0
14172587c: mov rsi, rax
14172587f: mov r14, rdx
141725882: lea rax, [rip + 0x15a4e37]
141725889: mov qword ptr [rbx + 0x200], rax
141725890: lea rax, [rip - 0x161ba07]
141725897: mov qword ptr [rbx + 0x208], rax
14172589e: lea rax, [rbx + 0x480]
1417258a5: mov qword ptr [rbx + 0x210], rax
1417258ac: lea rax, [rip - 0x998353]
1417258b3: mov qword ptr [rbx + 0x218], rax
1417258ba: mov qword ptr [rbx + 0x700], rdi
1417258c1: mov qword ptr [rbx + 0x708], 3
1417258cc: mov qword ptr [rbx + 0x710], r13
1417258d3: mov qword ptr [rbx + 0x718], 2
1417258de: mov qword ptr [rbx + 0x720], 0
1417258e9: mov byte ptr [rbx + 0x7c], r15b
1417258ed: lea rcx, [rbx + 0x190]
1417258f4: lea rdx, [rbx + 0x700]
1417258fb: call 0x140d79f90
141725900: movups xmm0, xmmword ptr [rbx + 0x190]
141725907: movaps xmmword ptr [rbx + 0x700], xmm0
14172590e: mov rax, qword ptr [rbx + 0x1a0]
141725915: mov qword ptr [rbx + 0x710], rax
14172591c: mov byte ptr [rbx + 0x7c], r15b
141725920: lea rcx, [rbx + 0x200]
141725927: lea r9, [rbx + 0x700]
14172592e: mov rdx, rsi
141725931: mov r8, r14
141725934: call 0x140b0cca0
141725939: mov byte ptr [rbx + 0xc4], 1
141725940: lea rcx, [rbx + 0x200]
141725947: call 0x140d76ed0
14172594c: mov byte ptr [rbx + 0xc4], 1
141725953: mov rcx, rax
141725956: call 0x140d62cf0
14172595b: test al, al
14172595d: je 0x141725e26
141725963: movups xmm0, xmmword ptr [rbx + 0x200]
14172596a: movups xmm1, xmmword ptr [rbx + 0x210]
141725971: movaps xmmword ptr [rbx + 0x710], xmm1
141725978: movaps xmmword ptr [rbx + 0x700], xmm0
14172597f: mov byte ptr [rbx + 0xc4], 0
141725986: lea rdx, [rbx + 0x700]
14172598d: mov rcx, qword ptr [rbx + 0x80]
141725994: call 0x141527a70
141725999: jmp 0x141725e44
14172599e: mov byte ptr [rbx + 0x91], 1
1417259a5: lea rcx, [rbx + 0x700]
1417259ac: lea rdx, [rbx + 0x140]
1417259b3: call 0x14096b1c0
1417259b8: movzx eax, byte ptr [rbx + 0x718]
1417259bf: mov rcx, qword ptr [rbx + 0x700]
1417259c6: cmp al, 2
1417259c8: jne 0x141725da3
1417259ce: mov byte ptr [rbx + 0x91], 1
1417259d5: call 0x1428a9590
1417259da: mov rsi, rax
1417259dd: mov r14d, 1
1417259e3: mov r15, qword ptr [rbx + 0xb8]
1417259ea: mov rdx, qword ptr [rbx + 0x15a0]
1417259f1: test rdx, rdx
1417259f4: je 0x141725a08
1417259f6: mov rcx, qword ptr [rbx + 0x15a8]
1417259fd: mov r8d, 1
141725a03: call 0x140613c20
141725a08: mov rdx, qword ptr [rbx + 0x140]
141725a0f: test rdx, rdx
141725a12: je 0x141725a26
141725a14: mov rcx, qword ptr [rbx + 0x148]
141725a1b: mov r8d, 1
141725a21: call 0x140613c20
141725a26: mov rdx, qword ptr [rbx + 0x100]
141725a2d: test rdx, rdx
141725a30: je 0x141725a44
141725a32: mov rcx, qword ptr [rbx + 0x108]
141725a39: mov r8d, 1
141725a3f: call 0x140613c20
141725a44: test r15, r15
141725a47: je 0x141725a5e
141725a49: mov r8d, 1
141725a4f: mov rcx, qword ptr [rbx + 0xd0]
141725a56: mov rdx, r15
141725a59: call 0x140613c20
141725a5e: mov rcx, rsi
141725a61: test r14, r14
141725a64: je 0x141726d71
141725a6a: mov qword ptr [rbx + 0x480], rcx
141725a71: mov rax, qword ptr [rip + 0x1f06930]
141725a78: mov rax, qword ptr [rax]
141725a7b: test rax, rax
141725a7e: je 0x141725b65
141725a84: lea rax, [rbx + 0x480]
141725a8b: mov qword ptr [rbx + 0x200], rax
141725a92: lea rax, [rip - 0x9d8e79]
141725a99: mov qword ptr [rbx + 0x208], rax
141725aa0: lea rcx, [rip + 0x15a52e1]
141725aa7: call 0x14067dd80
141725aac: movups xmm0, xmmword ptr [rax]
141725aaf: mov eax, dword ptr [rax + 0x10]
141725ab2: mov qword ptr [rbx + 0x730], 1
141725abd: lea rcx, [rip + 0x15a4a9c]
141725ac4: mov qword ptr [rbx + 0x738], rcx
141725acb: mov qword ptr [rbx + 0x740], 0x28
141725ad6: lea rdx, [rip + 0x15a529b]
141725add: mov qword ptr [rbx + 0x750], rdx
141725ae4: mov qword ptr [rbx + 0x758], 1
141725aef: lea rdx, [rbx + 0x200]
141725af6: mov qword ptr [rbx + 0x760], rdx
141725afd: mov qword ptr [rbx + 0x768], 1
141725b08: mov qword ptr [rbx + 0x770], 0
141725b13: mov qword ptr [rbx + 0x700], 0
141725b1e: mov qword ptr [rbx + 0x708], rcx
141725b25: mov qword ptr [rbx + 0x710], 0x28
141725b30: mov qword ptr [rbx + 0x718], 0
141725b3b: movups xmmword ptr [rbx + 0x720], xmm0
141725b42: mov dword ptr [rbx + 0x748], 1
141725b4c: mov dword ptr [rbx + 0x74c], eax
141725b52: lea rcx, [rbx + 0x93]
141725b59: lea rdx, [rbx + 0x700]
141725b60: call 0x140d4a330
141725b65: mov rax, qword ptr [rbx + 0xf0]
141725b6c: mov qword ptr [rax + 0x10], 1
141725b74: lea rcx, [rbx + 0x480]
141725b7b: call 0x140d4cc30
141725b80: cmp byte ptr [rbx + 0x98], 0
141725b87: je 0x141725bf0
141725b89: call 0x140c18130
141725b8e: mov rcx, rax
141725b91: mov rax, qword ptr [rax]
141725b94: test rax, rax
141725b97: je 0x141725ba8
141725b99: cmp eax, 1
141725b9c: jne 0x141727402
141725ba2: add rcx, 8
141725ba6: jmp 0x141725bc2
141725ba8: mov byte ptr [rbx + 0x90], 0
141725baf: xor edx, edx
141725bb1: call 0x1428f87d0
141725bb6: mov rcx, rax
141725bb9: test rax, rax
141725bbc: je 0x141727402
141725bc2: cmp qword ptr [rcx], 0
141725bc6: jne 0x141727013
141725bcc: mov qword ptr [rcx], 0xffffffffffffffff
141725bd3: mov qword ptr [rbx + 0x98], rcx
141725bda: add rcx, 8
141725bde: mov rdx, rdi
141725be1: call 0x140c170f0
141725be6: mov rax, qword ptr [rbx + 0x98]
141725bed: inc qword ptr [rax]
141725bf0: mov rax, qword ptr [rbx + 0xf0]
141725bf7: lock dec qword ptr [rax]
141725bfb: jne 0x141726c0b
141725c01: mov byte ptr [rbx + 0xcf], 0
141725c08: lea rcx, [rbx + 0xf0]
141725c0f: call 0x1402e5d80
141725c14: jmp 0x141726c0b
141725c19: jne 0x141723038
141725c1f: mov byte ptr [rbx + 0x7d], 1
141725c23: lea rdi, [rip + 0x15a469a]
141725c2a: mov rcx, rdi
141725c2d: mov rdx, r9
141725c30: call 0x14171e730
141725c35: cmp rax, 1
141725c39: jne 0x1417264f1
141725c3f: mov r8, rdx
141725c42: add r8, 2
141725c46: je 0x1417264df
141725c4c: cmp r8, 0x48
141725c50: jae 0x1417264d9
141725c56: lea rax, [rip + 0x15a4667]
141725c5d: cmp byte ptr [r8 + rax], 0xbf
141725c62: jg 0x1417264df
141725c68: mov byte ptr [rbx + 0x7d], 1
141725c6c: lea rax, [rip + 0x15a47d5]
141725c73: mov qword ptr [rsp + 0x20], rax
141725c78: lea rcx, [rip + 0x15a4645]
141725c7f: mov edx, 0x48
141725c84: mov r9d, 0x48
141725c8a: call 0x1428d9940
141725c8f: jmp 0x14172746a
141725c94: mov rdx, qword ptr [rbx + 0x480]
141725c9b: test rdx, rdx
141725c9e: je 0x141725cb2
141725ca0: mov rcx, qword ptr [rbx + 0x488]
141725ca7: mov r8d, 1
141725cad: call 0x140613c20
141725cb2: xor esi, esi
141725cb4: mov rdx, qword ptr [rbx + 0x1340]
141725cbb: test rdx, rdx
141725cbe: je 0x141725cd2
141725cc0: mov rcx, qword ptr [rbx + 0x1348]
141725cc7: mov r8d, 1
141725ccd: call 0x140613c20
141725cd2: mov byte ptr [rbx + 0x97], 0
141725cd9: lea rcx, [rbx + 0xd8]
141725ce0: mov rdx, rsi
141725ce3: call 0x1408da1b0
141725ce8: mov qword ptr [rbx + 0x700], rax
141725cef: mov qword ptr [rbx + 0x708], rdx
141725cf6: test rax, rax
141725cf9: sete al
141725cfc: test rdx, rdx
141725cff: sete cl
141725d02: or cl, al
141725d04: jne 0x141725d19
141725d06: lea rcx, [rbx + 0x708]
141725d0d: mov byte ptr [rbx + 0x97], 0
141725d14: call 0x140d4cc30
141725d19: cmp qword ptr [rbx + 0x190], 0
141725d21: je 0x141725d8b
141725d23: call 0x140c18130
141725d28: mov rcx, rax
141725d2b: mov rax, qword ptr [rax]
141725d2e: test rax, rax
141725d31: je 0x141725d42
141725d33: cmp eax, 1
141725d36: jne 0x141727448
141725d3c: add rcx, 8
141725d40: jmp 0x141725d59
141725d42: mov byte ptr [rbx + 0x7e], 0
141725d46: xor edx, edx
141725d48: call 0x1428f87d0
141725d4d: mov rcx, rax
141725d50: test rax, rax
141725d53: je 0x141727448
141725d59: cmp qword ptr [rcx], 0
141725d5d: jne 0x1417270b2
141725d63: mov qword ptr [rcx], 0xffffffffffffffff
141725d6a: mov qword ptr [rbx + 0x80], rcx
141725d71: add rcx, 8
141725d75: mov rdx, qword ptr [rbx + 0x198]
141725d7c: call 0x140c170f0
141725d81: mov rax, qword ptr [rbx + 0x80]
141725d88: inc qword ptr [rax]
141725d8b: mov byte ptr [rbx + 0xc5], 0
141725d92: lea rcx, [rbx + 0xd8]
141725d99: call 0x140877b30
141725d9e: jmp 0x141726c0b
141725da3: movups xmm0, xmmword ptr [rbx + 0x708]
141725daa: movups xmmword ptr [rbx + 0x1348], xmm0
141725db1: mov edx, dword ptr [rbx + 0x719]
141725db7: mov dword ptr [rbx + 0x1359], edx
141725dbd: mov edx, dword ptr [rbx + 0x71c]
141725dc3: mov dword ptr [rbx + 0x135c], edx
141725dc9: mov qword ptr [rbx + 0x1340], rcx
141725dd0: mov byte ptr [rbx + 0x1358], al
141725dd6: mov byte ptr [rbx + 0xb7], 1
141725ddd: lea rcx, [rbx + 0x1340]
141725de4: mov rdx, qword ptr [rbx + 0xd0]
141725deb: mov r8, r14
141725dee: call 0x14096b030
141725df3: test rax, rax
141725df6: je 0x141726c9f
141725dfc: mov byte ptr [rbx + 0xb7], 1
141725e03: mov rcx, rax
141725e06: call 0x1428a9590
141725e0b: mov rsi, rax
141725e0e: mov byte ptr [rbx + 0x91], 1
141725e15: lea rcx, [rbx + 0x1340]
141725e1c: call 0x1405ecc10
141725e21: jmp 0x1417259dd
141725e26: mov rdx, qword ptr [rbx + 0x200]
141725e2d: test rdx, rdx
141725e30: je 0x141725e44
141725e32: mov rcx, qword ptr [rbx + 0x208]
141725e39: mov r8d, 1
141725e3f: call 0x140613c20
141725e44: mov r15d, dword ptr [rbx + 0xb8]
141725e4b: mov byte ptr [rbx + 0x7c], r15b
141725e4f: lea rcx, [rbx + 0x140]
141725e56: call 0x140d76ed0
141725e5b: mov r8, rdx
141725e5e: mov byte ptr [rbx + 0x7c], r15b
141725e62: lea rcx, [rbx + 0x200]
141725e69: mov rdx, rax
141725e6c: call 0x140d68f80
141725e71: mov rdi, qword ptr [rbx + 0x200]
141725e78: mov rsi, qword ptr [rbx + 0x208]
141725e7f: cmp rdi, 2
141725e83: jne 0x141725feb
141725e89: mov qword ptr [rbx + 0x480], rsi
141725e90: mov byte ptr [rbx + 0xcb], 1
141725e97: lea rcx, [rbx + 0x700]
141725e9e: call 0x140d56700
141725ea3: movups xmm0, xmmword ptr [rbx + 0x700]
141725eaa: movups xmm1, xmmword ptr [rbx + 0x710]
141725eb1: movups xmm2, xmmword ptr [rbx + 0x720]
141725eb8: movaps xmmword ptr [rbx + 0x220], xmm2
141725ebf: movaps xmmword ptr [rbx + 0x210], xmm1
141725ec6: movaps xmmword ptr [rbx + 0x200], xmm0
141725ecd: lea rax, [rip + 0x13bd7a8]
141725ed4: mov qword ptr [rbx + 0x700], rax
141725edb: mov qword ptr [rbx + 0x708], 0x23
141725ee6: mov qword ptr [rbx + 0x710], rsi
141725eed: mov byte ptr [rbx + 0xcb], 0
141725ef4: lea rcx, [rbx + 0x700]
141725efb: lea rdx, [rbx + 0x200]
141725f02: call 0x1428a8e80
141725f07: mov qword ptr [rbx + 0x100], rax
141725f0e: mov rax, qword ptr [rip + 0x1f06493]
141725f15: mov rax, qword ptr [rax]
141725f18: test rax, rax
141725f1b: je 0x141725fd0
141725f21: lea rax, [rbx + 0x100]
141725f28: mov qword ptr [rbx + 0x480], rax
141725f2f: lea rax, [rip + 0x69aa]
141725f36: mov qword ptr [rbx + 0x488], rax
141725f3d: lea rax, [rip + 0x15a4914]
141725f44: mov qword ptr [rbx + 0x700], rax
141725f4b: mov qword ptr [rbx + 0x708], 1
141725f56: mov qword ptr [rbx + 0x720], 0
141725f61: lea rax, [rbx + 0x480]
141725f68: mov qword ptr [rbx + 0x710], rax
141725f6f: mov qword ptr [rbx + 0x718], 1
141725f7a: lea rcx, [rip + 0x15a48e7]
141725f81: call 0x14067dd80
141725f86: lea rcx, [rip + 0x15a45d3]
141725f8d: mov qword ptr [rbx + 0x200], rcx
141725f94: mov qword ptr [rbx + 0x208], 0x28
141725f9f: mov qword ptr [rbx + 0x210], rcx
141725fa6: mov qword ptr [rbx + 0x218], 0x28
141725fb1: mov qword ptr [rbx + 0x220], rax
141725fb8: lea rcx, [rbx + 0x700]
141725fbf: lea r8, [rbx + 0x200]
141725fc6: mov edx, 1
141725fcb: call 0x1404b1430
141725fd0: mov esi, dword ptr [rbx + 0xb8]
141725fd6: mov byte ptr [rbx + 0x7c], sil
141725fda: lea rcx, [rbx + 0x100]
141725fe1: call 0x140d4cc30
141725fe6: jmp 0x141726424
141725feb: lea rdx, [rbx + 0x210]
141725ff2: lea rcx, [rbx + 0xbd0]
141725ff9: mov r8d, 0x260
141725fff: call 0x142923c00
141726004: mov qword ptr [rbx + 0x700], 0
14172600f: mov qword ptr [rbx + 0x960], 0
14172601a: mov qword ptr [rbx + 0xbc0], rdi
141726021: mov qword ptr [rbx + 0xbc8], rsi
141726028: mov byte ptr [rbx + 0x7c], r15b
14172602c: lea r8, [rip + 0x15a4095]
141726033: lea rcx, [rbx + 0x100]
14172603a: lea rdx, [rbx + 0x700]
141726041: call 0x14130b810
141726046: mov rax, qword ptr [rbx + 0x80]
14172604d: mov r13, qword ptr [rax + 0x18]
141726051: add r13, 4
141726055: mov rdx, qword ptr [rbx + 0x110]
14172605c: cmp rdx, r13
14172605f: jbe 0x14172640e
141726065: mov rcx, qword ptr [rbx + 0x108]
14172606c: lea rax, [rbx + 0x93]
141726073: mov qword ptr [rbx + 0x700], rax
14172607a: cmp rdx, 2
14172607e: jae 0x141727116
141726084: lea r12, [rbx + 0x208]
14172608b: mov rdi, qword ptr [rip + 0x1f06316]
141726092: lea rsi, [rbx + 0x200]
141726099: lea r14, [rip + 0x15a44c0]
1417260a0: lea r15, [rbx + 0x700]
1417260a7: mov rax, qword ptr [rbx + 0x110]
1417260ae: cmp rax, r13
1417260b1: jbe 0x14172640e
1417260b7: dec rax
1417260ba: mov qword ptr [rbx + 0x110], rax
1417260c1: mov rcx, qword ptr [rbx + 0x108]
1417260c8: lea rdx, [rax + rax*2]
1417260cc: shl rdx, 4
1417260d0: mov rax, qword ptr [rcx + rdx]
1417260d4: movups xmm0, xmmword ptr [rcx + rdx + 8]
1417260d9: movaps xmmword ptr [rbx + 0x700], xmm0
1417260e0: movups xmm0, xmmword ptr [rcx + rdx + 0x18]
1417260e5: movaps xmmword ptr [rbx + 0x710], xmm0
1417260ec: mov rcx, qword ptr [rcx + rdx + 0x28]
1417260f1: mov qword ptr [rbx + 0x720], rcx
1417260f8: mov rcx, rax
1417260fb: neg rcx
1417260fe: jo 0x141727137
141726104: mov qword ptr [rbx + 0x200], rax
14172610b: mov rax, qword ptr [rbx + 0x720]
141726112: mov qword ptr [r12 + 0x20], rax
141726117: movaps xmm0, xmmword ptr [rbx + 0x700]
14172611e: movaps xmm1, xmmword ptr [rbx + 0x710]
141726125: movups xmmword ptr [r12 + 0x10], xmm1
14172612b: movups xmmword ptr [r12], xmm0
141726130: mov rax, qword ptr [rdi]
141726133: cmp rax, 3
141726137: jb 0x141726231
14172613d: mov rcx, rsi
141726140: call 0x140d76ed0
141726145: mov rcx, rax
141726148: call 0x140d61e00
14172614d: mov qword ptr [rbx + 0x480], rax
141726154: mov qword ptr [rbx + 0x488], rdx
14172615b: lea rax, [rbx + 0x480]
141726162: mov qword ptr [rbx + 0x170], rax
141726169: lea rax, [rip - 0x74f0]
141726170: mov qword ptr [rbx + 0x178], rax
141726177: lea rcx, [rip + 0x15a464a]
14172617e: call 0x14067dd80
141726183: movups xmm0, xmmword ptr [rax]
141726186: mov eax, dword ptr [rax + 0x10]
141726189: mov qword ptr [rbx + 0x730], 3
141726194: mov qword ptr [rbx + 0x738], r14
14172619b: mov qword ptr [rbx + 0x740], 0x28
1417261a6: lea rcx, [rip + 0x15a460b]
1417261ad: mov qword ptr [rbx + 0x750], rcx
1417261b4: mov qword ptr [rbx + 0x758], 1
1417261bf: lea rcx, [rbx + 0x170]
1417261c6: mov qword ptr [rbx + 0x760], rcx
1417261cd: mov qword ptr [rbx + 0x768], 1
1417261d8: mov qword ptr [rbx + 0x770], 0
1417261e3: mov qword ptr [rbx + 0x700], 0
1417261ee: mov qword ptr [rbx + 0x708], r14
1417261f5: mov qword ptr [rbx + 0x710], 0x28
141726200: mov qword ptr [rbx + 0x718], 0
14172620b: movups xmmword ptr [rbx + 0x720], xmm0
141726212: mov dword ptr [rbx + 0x748], 1
14172621c: mov dword ptr [rbx + 0x74c], eax
141726222: lea rcx, [rbx + 0x93]
141726229: mov rdx, r15
14172622c: call 0x140d4a330
141726231: mov rcx, rsi
141726234: call 0x140d76ed0
141726239: mov rcx, rax
14172623c: call 0x140d69590
141726241: test rax, rax
141726244: je 0x1417263e7
14172624a: mov qword ptr [rbx + 0xf0], rax
141726251: mov rcx, qword ptr [rdi]
141726254: test rcx, rcx
141726257: je 0x141726374
14172625d: mov rcx, rsi
141726260: call 0x140d76ed0
141726265: mov rcx, rax
141726268: call 0x140d61e00
14172626d: mov qword ptr [rbx + 0x170], rax
141726274: mov qword ptr [rbx + 0x178], rdx
14172627b: lea rax, [rbx + 0x170]
141726282: mov qword ptr [rbx + 0x480], rax
141726289: lea rax, [rip - 0x7610]
141726290: mov qword ptr [rbx + 0x488], rax
141726297: lea rax, [rbx + 0xf0]
14172629e: mov qword ptr [rbx + 0x490], rax
1417262a5: lea rax, [rip - 0x9cb5cc]
1417262ac: mov qword ptr [rbx + 0x498], rax
1417262b3: lea rcx, [rip + 0x15a4566]
1417262ba: call 0x14067dd80
1417262bf: movups xmm0, xmmword ptr [rax]
1417262c2: mov eax, dword ptr [rax + 0x10]
1417262c5: mov qword ptr [rbx + 0x730], 1
1417262d0: mov qword ptr [rbx + 0x738], r14
1417262d7: mov qword ptr [rbx + 0x740], 0x28
1417262e2: lea rcx, [rip + 0x15a4517]
1417262e9: mov qword ptr [rbx + 0x750], rcx
1417262f0: mov qword ptr [rbx + 0x758], 2
1417262fb: lea rcx, [rbx + 0x480]
141726302: mov qword ptr [rbx + 0x760], rcx
141726309: mov qword ptr [rbx + 0x768], 2
141726314: mov qword ptr [rbx + 0x770], 0
14172631f: mov qword ptr [rbx + 0x700], 0
14172632a: mov qword ptr [rbx + 0x708], r14
141726331: mov qword ptr [rbx + 0x710], 0x28
14172633c: mov qword ptr [rbx + 0x718], 0
141726347: movups xmmword ptr [rbx + 0x720], xmm0
14172634e: mov dword ptr [rbx + 0x748], 1
141726358: mov dword ptr [rbx + 0x74c], eax
14172635e: lea rcx, [rbx + 0x93]
141726365: mov rdx, r15
141726368: call 0x140d4a330
14172636d: mov rax, qword ptr [rbx + 0xf0]
141726374: mov ecx, eax
141726376: and ecx, 3
141726379: cmp ecx, 1
14172637c: jne 0x1417263e7
14172637e: lea rcx, [rax - 1]
141726382: mov qword ptr [rbx + 0xa8], rcx
141726389: mov rcx, qword ptr [rax - 1]
14172638d: mov qword ptr [rbx + 0x80], rcx
141726394: mov rax, qword ptr [rax + 7]
141726398: mov qword ptr [rbx + 0x98], rax
14172639f: mov rax, qword ptr [rax]
1417263a2: test rax, rax
1417263a5: je 0x1417263b0
1417263a7: mov rcx, qword ptr [rbx + 0x80]
1417263ae: call rax
1417263b0: mov rcx, qword ptr [rbx + 0x80]
1417263b7: mov rax, qword ptr [rbx + 0x98]
1417263be: mov rdx, qword ptr [rax + 8]
1417263c2: test rdx, rdx
1417263c5: je 0x1417263d0
1417263c7: mov r8, qword ptr [rax + 0x10]
1417263cb: call 0x140613c20
1417263d0: mov edx, 0x18
1417263d5: mov r8d, 8
1417263db: mov rcx, qword ptr [rbx + 0xa8]
1417263e2: call 0x140613c20
1417263e7: mov rdx, qword ptr [rbx + 0x200]
1417263ee: test rdx, rdx
1417263f1: je 0x1417260a7
1417263f7: mov rcx, qword ptr [rbx + 0x208]
1417263fe: mov r8d, 1
141726404: call 0x140613c20
141726409: jmp 0x1417260a7
14172640e: mov esi, dword ptr [rbx + 0xb8]
141726414: mov byte ptr [rbx + 0x7c], sil
141726418: lea rcx, [rbx + 0x100]
14172641f: call 0x140877160
141726424: mov rdx, qword ptr [rbx + 0x1340]
14172642b: test rdx, rdx
14172642e: je 0x141726442
141726430: mov rcx, qword ptr [rbx + 0x1348]
141726437: mov r8d, 1
14172643d: call 0x140613c20
141726442: test sil, sil
141726445: je 0x141726465
141726447: mov rdx, qword ptr [rbx + 0x15a0]
14172644e: test rdx, rdx
141726451: je 0x141726465
141726453: mov rcx, qword ptr [rbx + 0x15a8]
14172645a: mov r8d, 1
141726460: call 0x140613c20
141726465: mov rdx, qword ptr [rbx + 0x140]
14172646c: test rdx, rdx
14172646f: je 0x141726483
141726471: mov rcx, qword ptr [rbx + 0x148]
141726478: mov r8d, 1
14172647e: call 0x140613c20
141726483: mov rax, qword ptr [rbx + 0x120]
14172648a: test rax, rax
14172648d: je 0x141726c0b
141726493: call 0x140c18130
141726498: mov rcx, rax
14172649b: mov rax, qword ptr [rax]
14172649e: cmp rax, 1
1417264a2: jne 0x141726c2d
1417264a8: add rcx, 8
1417264ac: cmp qword ptr [rcx], 0
1417264b0: jne 0x141726e07
1417264b6: mov qword ptr [rcx], 0xffffffffffffffff
1417264bd: mov qword ptr [rbx + 0x80], rcx
1417264c4: add rcx, 8
1417264c8: mov rdx, qword ptr [rbx + 0x128]
1417264cf: call 0x140c170f0
1417264d4: jmp 0x141726c01
1417264d9: jne 0x141725c68
1417264df: mov esi, 0x46
1417264e4: sub rsi, rdx
1417264e7: lea rdi, [rip + 0x15a3dd6]
1417264ee: add rdi, r8
1417264f1: lea r8, [rip + 0x15a3fb3]
1417264f8: lea r14, [rip + 0x15a3f79]
1417264ff: nop 
141726500: movsx eax, byte ptr [r8 - 1]
141726505: test eax, eax
141726507: js 0x141726520
141726509: dec r8
14172650c: cmp eax, 0x5c
14172650f: jne 0x141726574
141726511: jmp 0x141726586
141726513: nop word ptr cs:[rax + rax]
141726520: movzx ecx, byte ptr [r8 - 2]
141726525: cmp cl, 0xc0
141726528: jge 0x14172654d
14172652a: movzx edx, byte ptr [r8 - 3]
14172652f: cmp dl, 0xc0
141726532: jge 0x141726556
141726534: movzx r9d, byte ptr [r8 - 4]
141726539: add r8, -4
14172653d: and r9d, 7
141726541: shl r9d, 6
141726545: and edx, 0x3f
141726548: or edx, r9d
14172654b: jmp 0x14172655d
14172654d: add r8, -2
141726551: and ecx, 0x1f
141726554: jmp 0x141726565
141726556: add r8, -3
14172655a: and edx, 0xf
14172655d: shl edx, 6
141726560: and ecx, 0x3f
141726563: or ecx, edx
141726565: shl ecx, 6
141726568: and al, 0x3f
14172656a: movzx eax, al
14172656d: or eax, ecx
14172656f: cmp eax, 0x5c
141726572: je 0x141726586
141726574: cmp eax, 0x2f
141726577: je 0x141726586
141726579: cmp r8, r14
14172657c: jne 0x141726500
14172657e: mov r12d, 0x33
141726584: jmp 0x1417265dc
141726586: lea r14, [rip + 0x15a3eeb]
14172658d: sub r8, r14
141726590: inc r8
141726593: je 0x1417265d0
141726595: cmp r8, 0x33
141726599: jae 0x1417265ce
14172659b: cmp byte ptr [r8 + r14], 0xbf
1417265a0: jg 0x1417265d0
1417265a2: mov byte ptr [rbx + 0x7d], 1
1417265a6: lea rax, [rip + 0x15a3e63]
1417265ad: mov qword ptr [rsp + 0x20], rax
1417265b2: lea rcx, [rip + 0x15a3ebf]
1417265b9: mov edx, 0x33
1417265be: mov r9d, 0x33
1417265c4: call 0x1428d9940
1417265c9: jmp 0x14172746a
1417265ce: jne 0x1417265a2
1417265d0: mov r12d, 0x33
1417265d6: sub r12, r8
1417265d9: add r14, r8
1417265dc: call 0x140c18130
1417265e1: mov rcx, rax
1417265e4: mov rax, qword ptr [rax]
1417265e7: test rax, rax
1417265ea: je 0x1417265fb
1417265ec: cmp eax, 1
1417265ef: jne 0x1417273f0
1417265f5: add rcx, 8
1417265f9: jmp 0x141726612
1417265fb: mov byte ptr [rbx + 0x7d], 1
1417265ff: xor edx, edx
141726601: call 0x1428f87d0
141726606: mov rcx, rax
141726609: test rax, rax
14172660c: je 0x1417273f0
141726612: cmp qword ptr [rcx], 0
141726616: jne 0x141726ffe
14172661c: mov qword ptr [rcx], 0xffffffffffffffff
141726623: mov qword ptr [rbx + 0x80], rcx
14172662a: add rcx, 8
14172662e: mov qword ptr [rsp + 0x20], r12
141726633: mov qword ptr [rsp + 0x30], 0
14172663c: mov qword ptr [rsp + 0x28], 1
141726645: mov rdx, rdi
141726648: mov r8, rsi
14172664b: mov r9, r14
14172664e: call 0x140c16d40
141726653: mov rcx, qword ptr [rbx + 0x80]
14172665a: inc qword ptr [rcx]
14172665d: mov qword ptr [rbx + 0x198], rax
141726664: mov eax, 1
141726669: mov qword ptr [rbx + 0x190], rax
141726670: mov byte ptr [rbx + 0x94], 1
141726677: call 0x140d635e0
14172667c: lea rdi, [r13 + 0x90]
141726683: cmp qword ptr [r13 + 0xa0], r15
14172668a: jb 0x141726e13
141726690: jbe 0x14172669f
141726692: mov rax, qword ptr [r13 + 0x98]
141726699: mov rax, qword ptr [rax + r15*8]
14172669d: jmp 0x1417266ad
14172669f: mov rax, qword ptr [r13 + 0x40]
1417266a3: add rax, -4
1417266a7: jae 0x14172702b
1417266ad: lea rcx, [r13 + 0x10]
1417266b1: mov qword ptr [r13 + 0x48], rax
1417266b5: mov qword ptr [rbx + 0xb8], rcx
1417266bc: mov qword ptr [rbx + 0x1b8], rcx
1417266c3: mov qword ptr [rbx + 0xa8], r13
1417266ca: mov qword ptr [r13 + 0xa0], r15
1417266d1: mov rdx, qword ptr [rbx + 0x15a0]
1417266d8: mov rcx, qword ptr [rbx + 0x15a8]
1417266df: mov rax, qword ptr [rbx + 0x15b0]
1417266e6: lea rax, [rax + rax*4]
1417266ea: lea rax, [rcx + rax*8]
1417266ee: mov qword ptr [rbx + 0x1340], rcx
1417266f5: mov qword ptr [rbx + 0x1348], rcx
1417266fc: mov qword ptr [rbx + 0x1350], rdx
141726703: mov qword ptr [rbx + 0x1358], rax
14172670a: lea rsi, [rbx + 0x208]
141726711: lea r15, [rbx + 0x200]
141726718: mov r13, qword ptr [rip + 0x1f05c89]
14172671f: lea r14, [rip + 0x15a3e3a]
141726726: xorps xmm6, xmm6
141726729: lea r12, [rbx + 0xf0]
141726730: cmp rcx, rax
141726733: je 0x14172697f
141726739: nop dword ptr [rax]
141726740: lea r8, [rcx + 0x28]
141726744: mov qword ptr [rbx + 0x1348], r8
14172674b: mov rdx, qword ptr [rcx]
14172674e: movups xmm0, xmmword ptr [rcx + 0x18]
141726752: movaps xmmword ptr [rbx + 0x490], xmm0
141726759: movups xmm0, xmmword ptr [rcx + 8]
14172675d: movaps xmmword ptr [rbx + 0x480], xmm0
141726764: mov rcx, rdx
141726767: neg rcx
14172676a: jo 0x141726982
141726770: mov qword ptr [rbx + 0x200], rdx
141726777: movaps xmm0, xmmword ptr [rbx + 0x480]
14172677e: movaps xmm1, xmmword ptr [rbx + 0x490]
141726785: movups xmmword ptr [rsi + 0x10], xmm1
141726789: movups xmmword ptr [rsi], xmm0
14172678c: lea rcx, [rbx + 0x1b8]
141726793: mov rdx, r15
141726796: mov r8, rdi
141726799: call 0x140a439b0
14172679e: test rax, rax
1417267a1: je 0x1417268a0
1417267a7: mov qword ptr [rbx + 0xf0], rax
1417267ae: mov rax, qword ptr [r13]
1417267b2: test rax, rax
1417267b5: je 0x14172688e
1417267bb: mov qword ptr [rbx + 0x100], r12
1417267c2: lea rax, [rip - 0x9d9ba9]
1417267c9: mov qword ptr [rbx + 0x108], rax
1417267d0: lea rcx, [rip + 0x15a3de9]
1417267d7: call 0x14067dd80
1417267dc: movups xmm0, xmmword ptr [rax]
1417267df: mov eax, dword ptr [rax + 0x10]
1417267e2: mov qword ptr [rbx + 0x730], 1
1417267ed: mov qword ptr [rbx + 0x738], r14
1417267f4: mov qword ptr [rbx + 0x740], 0x28
1417267ff: lea rcx, [rip + 0x15a3daa]
141726806: mov qword ptr [rbx + 0x750], rcx
14172680d: mov qword ptr [rbx + 0x758], 1
141726818: lea rcx, [rbx + 0x100]
14172681f: mov qword ptr [rbx + 0x760], rcx
141726826: mov qword ptr [rbx + 0x768], 1
141726831: mov qword ptr [rbx + 0x770], 0
14172683c: mov qword ptr [rbx + 0x700], 0
141726847: mov qword ptr [rbx + 0x708], r14
14172684e: mov qword ptr [rbx + 0x710], 0x28
141726859: mov qword ptr [rbx + 0x718], 0
141726864: movups xmmword ptr [rbx + 0x720], xmm0
14172686b: mov dword ptr [rbx + 0x748], 1
141726875: mov dword ptr [rbx + 0x74c], eax
14172687b: lea rcx, [rbx + 0x93]
141726882: lea rdx, [rbx + 0x700]
141726889: call 0x140d4a330
14172688e: mov rcx, r12
141726891: call 0x140d4cc30
141726896: jmp 0x141726960
14172689b: nop dword ptr [rax + rax]
1417268a0: mov rax, qword ptr [r13]
1417268a4: cmp rax, 5
1417268a8: jb 0x141726960
1417268ae: lea rcx, [rip + 0x15a3c93]
1417268b5: call 0x14067dd80
1417268ba: movups xmm0, xmmword ptr [rax]
1417268bd: mov eax, dword ptr [rax + 0x10]
1417268c0: mov qword ptr [rbx + 0x730], 5
1417268cb: mov qword ptr [rbx + 0x738], r14
1417268d2: mov qword ptr [rbx + 0x740], 0x28
1417268dd: lea rcx, [rip + 0x15a3c54]
1417268e4: mov qword ptr [rbx + 0x750], rcx
1417268eb: mov qword ptr [rbx + 0x758], 1
1417268f6: mov qword ptr [rbx + 0x760], 8
141726901: mov qword ptr [rbx + 0x700], 0
14172690c: lea rcx, [rbx + 0x768]
141726913: movups xmmword ptr [rcx], xmm6
141726916: mov qword ptr [rbx + 0x708], r14
14172691d: mov qword ptr [rbx + 0x710], 0x28
141726928: mov qword ptr [rbx + 0x718], 0
141726933: movups xmmword ptr [rbx + 0x720], xmm0
14172693a: mov dword ptr [rbx + 0x748], 1
141726944: mov dword ptr [rbx + 0x74c], eax
14172694a: lea rcx, [rbx + 0x93]
141726951: lea rdx, [rbx + 0x700]
141726958: call 0x140d4a330
14172695d: nop dword ptr [rax]
141726960: mov rcx, r15
141726963: call 0x14088bae0
141726968: mov rcx, qword ptr [rbx + 0x1348]
14172696f: mov rax, qword ptr [rbx + 0x1358]
141726976: cmp rcx, rax
141726979: jne 0x141726740
14172697f: mov r8, rcx
141726982: sub rax, r8
141726985: shr rax, 3
141726989: movabs rcx, 0xcccccccccccccccd
141726993: imul rcx, rax
141726997: mov qword ptr [rbx + 0x98], rcx
14172699e: mov qword ptr [rbx + 0x80], 0
1417269a9: mov qword ptr [rbx + 0x1c8], r8
1417269b0: mov rcx, r8
1417269b3: mov r12, qword ptr [rbx + 0xa8]
1417269ba: nop word ptr [rax + rax]
1417269c0: mov rax, qword ptr [rbx + 0x80]
1417269c7: cmp qword ptr [rbx + 0x98], rax
1417269ce: je 0x1417269e8
1417269d0: inc rax
1417269d3: mov qword ptr [rbx + 0x80], rax
1417269da: lea rsi, [rcx + 0x28]
1417269de: call 0x14088bae0
1417269e3: mov rcx, rsi
1417269e6: jmp 0x1417269c0
1417269e8: mov rax, qword ptr [rbx + 0x1350]
1417269ef: test rax, rax
1417269f2: je 0x141726a0e
1417269f4: mov rcx, qword ptr [rbx + 0x1340]
1417269fb: shl rax, 3
1417269ff: lea rdx, [rax + rax*4]
141726a03: mov r8d, 8
141726a09: call 0x140613c20
141726a0e: mov rsi, qword ptr [rbx + 0x1b8]
141726a15: mov rdi, qword ptr [rsi + 0x38]
141726a19: mov r14, rdi
141726a1c: add r14, 4
141726a20: mov r8, 0xffffffffffffffff
141726a27: cmovae r8, r14
141726a2b: mov rax, qword ptr [rsi + 0x20]
141726a2f: mov rdx, qword ptr [rsi + 0x30]
141726a33: cmp r8, rax
141726a36: jbe 0x141726a47
141726a38: sub r8, rdx
141726a3b: sub rax, rdx
141726a3e: cmp rax, r8
141726a41: jb 0x141727069
141726a47: mov r8, rdi
141726a4a: mov r15, qword ptr [rsi + 0x28]
141726a4e: sub r8, rdx
141726a51: jbe 0x141726a67
141726a53: add rdx, r15
141726a56: mov rcx, rdx
141726a59: xor edx, edx
141726a5b: call 0x142923c20
141726a60: mov qword ptr [rsi + 0x30], rdi
141726a64: mov rdx, rdi
141726a67: mov dword ptr [r15 + rdi], 0x7d7d5d0a
141726a6f: cmp r14, rdx
141726a72: jbe 0x141726a78
141726a74: mov qword ptr [rsi + 0x30], r14
141726a78: mov qword ptr [rsi + 0x38], r14
141726a7c: mov byte ptr [rbx + 0x94], 0
141726a83: mov rcx, rsi
141726a86: mov rdx, rdi
141726a89: mov r8, r14
141726a8c: call 0x14091a450
141726a91: mov rax, qword ptr [rbx + 0x1b8]
141726a98: mov rax, qword ptr [rax + 0x38]
141726a9c: mov rdx, qword ptr [r12 + 0x30]
141726aa1: mov r8, qword ptr [r12 + 0x38]
141726aa6: mov rcx, qword ptr [r12 + 0x40]
141726aab: mov qword ptr [r12 + 0x30], 0
141726ab4: mov qword ptr [r12 + 0x38], 1
141726abd: xorps xmm0, xmm0
141726ac0: movups xmmword ptr [r12 + 0x40], xmm0
141726ac6: cmp rcx, rax
141726ac9: jb 0x141726ee3
141726acf: mov qword ptr [r12 + 0x30], rdx
141726ad4: mov qword ptr [r12 + 0x38], r8
141726ad9: mov qword ptr [r12 + 0x40], rax
141726ade: mov qword ptr [r12 + 0x48], 0
141726ae7: mov rcx, rax
141726aea: shr rcx, 0xc
141726aee: and eax, 0xfff
141726af3: cmp rax, 1
141726af7: sbb rcx, -1
141726afb: mov rax, qword ptr [r12 + 0x78]
141726b00: mov rdx, rax
141726b03: shr rdx, 3
141726b07: cmp rdx, rcx
141726b0a: jbe 0x141726b18
141726b0c: and eax, 7
141726b0f: lea rax, [rax + rcx*8]
141726b13: mov qword ptr [r12 + 0x78], rax
141726b18: cmp qword ptr [rbx + 0x190], 0
141726b20: je 0x141726b8a
141726b22: call 0x140c18130
141726b27: mov rcx, rax
141726b2a: mov rax, qword ptr [rax]
141726b2d: test rax, rax
141726b30: je 0x141726b41
141726b32: cmp eax, 1
141726b35: jne 0x1417272fb
141726b3b: add rcx, 8
141726b3f: jmp 0x141726b58
141726b41: mov byte ptr [rbx + 0x7d], 0
141726b45: xor edx, edx
141726b47: call 0x1428f87d0
141726b4c: mov rcx, rax
141726b4f: test rax, rax
141726b52: je 0x1417272fb
141726b58: cmp qword ptr [rcx], 0
141726b5c: jne 0x141726f44
141726b62: mov qword ptr [rcx], 0xffffffffffffffff
141726b69: mov qword ptr [rbx + 0x80], rcx
141726b70: add rcx, 8
141726b74: mov rdx, qword ptr [rbx + 0x198]
141726b7b: call 0x140c170f0
141726b80: mov rax, qword ptr [rbx + 0x80]
141726b87: inc qword ptr [rax]
141726b8a: mov rcx, qword ptr [rbx + 0xb8]
141726b91: call 0x1409193d0
141726b96: cmp qword ptr [rbx + 0xd8], 0
141726b9e: je 0x141726c0b
141726ba0: call 0x140c18130
141726ba5: mov rcx, rax
141726ba8: mov rax, qword ptr [rax]
141726bab: test rax, rax
141726bae: je 0x141726bbf
141726bb0: cmp eax, 1
141726bb3: jne 0x141727310
141726bb9: add rcx, 8
141726bbd: jmp 0x141726bd9
141726bbf: mov byte ptr [rbx + 0x8f], 0
141726bc6: xor edx, edx
141726bc8: call 0x1428f87d0
141726bcd: mov rcx, rax
141726bd0: test rax, rax
141726bd3: je 0x141727310
141726bd9: cmp qword ptr [rcx], 0
141726bdd: jne 0x141726f59
141726be3: mov qword ptr [rcx], 0xffffffffffffffff
141726bea: mov qword ptr [rbx + 0x80], rcx
141726bf1: add rcx, 8
141726bf5: mov rdx, qword ptr [rbx + 0xe0]
141726bfc: call 0x140c170f0
141726c01: mov rax, qword ptr [rbx + 0x80]
141726c08: inc qword ptr [rax]
141726c0b: movaps xmm6, xmmword ptr [rbp + 0x1790]
141726c12: movaps xmm7, xmmword ptr [rbp + 0x17a0]
141726c19: lea rsp, [rbp + 0x17b8]
141726c20: pop rbx
141726c21: pop rdi
141726c22: pop rsi
141726c23: pop r12
141726c25: pop r13
141726c27: pop r14
141726c29: pop r15
141726c2b: pop rbp
141726c2c: ret 
141726c2d: test rax, rax
141726c30: jne 0x141726c93
141726c32: xor edx, edx
141726c34: call 0x1428f87d0
141726c39: mov rcx, rax
141726c3c: test rax, rax
141726c3f: jne 0x1417264ac
141726c45: jmp 0x141726c93
141726c47: test rax, rax
141726c4a: jne 0x141726c93
141726c4c: xor edx, edx
141726c4e: call 0x1428f87d0
141726c53: mov rcx, rax
141726c56: test rax, rax
141726c59: jne 0x14172275e
141726c5f: jmp 0x141726c93
141726c61: test rax, rax
141726c64: jne 0x141726c93
141726c66: xor edx, edx
141726c68: call 0x1428f87d0
141726c6d: mov rcx, rax
141726c70: test rax, rax
141726c73: jne 0x141722cb5
141726c79: jmp 0x141726c93
141726c7b: test rax, rax
141726c7e: jne 0x141726c93
141726c80: xor edx, edx
141726c82: call 0x1428f87d0
141726c87: mov rcx, rax
141726c8a: test rax, rax
141726c8d: jne 0x141722eca
141726c93: lea rcx, [rip + 0x1513e06]
141726c9a: call 0x1428d80d0
141726c9f: movups xmm0, xmmword ptr [rbx + 0x1340]
141726ca6: movups xmm1, xmmword ptr [rbx + 0x1350]
141726cad: movaps xmmword ptr [rbx + 0x490], xmm1
141726cb4: movaps xmmword ptr [rbx + 0x480], xmm0
141726cbb: movups xmm0, xmmword ptr [rbx + 0x15a0]
141726cc2: movups xmm1, xmmword ptr [rbx + 0x15b0]
141726cc9: movaps xmmword ptr [rbx + 0x710], xmm1
141726cd0: movaps xmmword ptr [rbx + 0x700], xmm0
141726cd7: mov byte ptr [rbx + 0xb7], 0
141726cde: lea rcx, [rbx + 0x200]
141726ce5: lea rdx, [rbx + 0x480]
141726cec: lea r8, [rbx + 0x700]
141726cf3: call 0x14096b270
141726cf8: movzx eax, byte ptr [rbx + 0x220]
141726cff: mov rcx, qword ptr [rbx + 0x200]
141726d06: cmp al, 2
141726d08: jne 0x141726d8e
141726d0e: call qword ptr [rip + 0x1a6b6dc]
141726d14: mov rdx, qword ptr [rbx + 0x140]
141726d1b: test rdx, rdx
141726d1e: je 0x141726d32
141726d20: mov rcx, qword ptr [rbx + 0x148]
141726d27: mov r8d, 1
141726d2d: call 0x140613c20
141726d32: mov rdx, qword ptr [rbx + 0x100]
141726d39: test rdx, rdx
141726d3c: mov r14, qword ptr [rbx + 0xb8]
141726d43: je 0x141726d57
141726d45: mov rcx, qword ptr [rbx + 0x108]
141726d4c: mov r8d, 1
141726d52: call 0x140613c20
141726d57: test r14, r14
141726d5a: je 0x141726d71
141726d5c: mov r8d, 1
141726d62: mov rcx, qword ptr [rbx + 0xd0]
141726d69: mov rdx, r14
141726d6c: call 0x140613c20
141726d71: mov rax, qword ptr [rbx + 0xf0]
141726d78: mov qword ptr [rax + 0x10], rsi
141726d7c: cmp byte ptr [rbx + 0x98], 0
141726d83: jne 0x141725b89
141726d89: jmp 0x141725bf0
141726d8e: mov rdx, qword ptr [rbx + 0x218]
141726d95: mov qword ptr [rbx + 0x718], rdx
141726d9c: movups xmm0, xmmword ptr [rbx + 0x208]
141726da3: movups xmmword ptr [rbx + 0x708], xmm0
141726daa: mov edx, dword ptr [rbx + 0x221]
141726db0: mov dword ptr [rbx + 0x721], edx
141726db6: mov edx, dword ptr [rbx + 0x224]
141726dbc: mov dword ptr [rbx + 0x724], edx
141726dc2: mov qword ptr [rbx + 0x700], rcx
141726dc9: mov byte ptr [rbx + 0x720], al
141726dcf: mov byte ptr [rbx + 0xb7], 0
141726dd6: lea rcx, [rbx + 0x700]
141726ddd: call 0x1428a9680
141726de2: mov rsi, rax
141726de5: mov r14d, 1
141726deb: mov r15, qword ptr [rbx + 0xb8]
141726df2: mov rdx, qword ptr [rbx + 0x140]
141726df9: test rdx, rdx
141726dfc: jne 0x141725a14
141726e02: jmp 0x141725a26
141726e07: lea rcx, [rip + 0x1513d62]
141726e0e: call 0x1428d9250
141726e13: mov byte ptr [rbx + 0x94], 1
141726e1a: lea rcx, [rip + 0x15a368a]
141726e21: lea r8, [rip + 0x15a36d8]
141726e28: mov edx, 0x51
141726e2d: call 0x1428d9430
141726e32: jmp 0x14172746a
141726e37: lea r8, [rbx + 0x720]
141726e3e: lea rcx, [rbx + 0x700]
141726e45: mov rdx, rsi
141726e48: mov r9b, 1
141726e4b: call 0x1428b2e40
141726e50: xor edi, edi
141726e52: lea r14, [rbx + 0x200]
141726e59: lea r15, [rbx + 0x700]
141726e60: mov r12, rdi
141726e63: shl r12, 5
141726e67: add r12, r13
141726e6a: mov rcx, r14
141726e6d: mov rdx, r12
141726e70: lea r8, [rip + 0x13be289]
141726e77: call 0x1412f81e0
141726e7c: movzx eax, byte ptr [r12 + 0x18]
141726e82: mov byte ptr [rbx + 0x218], al
141726e88: mov rcx, r15
141726e8b: mov rdx, r14
141726e8e: call 0x141527a70
141726e93: inc rdi
141726e96: cmp rdi, rsi
141726e99: jne 0x141726e60
141726e9b: jmp 0x1417252c0
141726ea0: lea r8, [rbx + 0x720]
141726ea7: lea rcx, [rbx + 0x700]
141726eae: mov rdx, rsi
141726eb1: mov r9b, 1
141726eb4: call 0x1428aa7a0
141726eb9: xor r15d, r15d
141726ebc: lea rdi, [rbx + 0x700]
141726ec3: mov rax, r15
141726ec6: shl rax, 4
141726eca: mov rdx, qword ptr [r14 + rax]
141726ece: mov rcx, rdi
141726ed1: call 0x1402ca2f0
141726ed6: inc r15
141726ed9: cmp r15, rsi
141726edc: jne 0x141726ec3
141726ede: jmp 0x141725373
141726ee3: mov qword ptr [rbx + 0x98], r8
141726eea: mov qword ptr [rbx + 0x80], rdx
141726ef1: lea rcx, [rip + 0x1386150]
141726ef8: lea r8, [rip + 0x1386179]
141726eff: mov edx, 0x2d
141726f04: call 0x1428d9430
141726f09: jmp 0x14172746a
141726f0e: mov byte ptr [rbx + 0x8d], 1
141726f15: mov byte ptr [rbx + 0x8e], 1
141726f1c: mov rcx, r12
141726f1f: mov r8d, 0x3b9aca00
141726f25: call 0x1428cd210
141726f2a: jmp 0x1417237c3
141726f2f: mov byte ptr [rbx + 0x7f], 0
141726f33: lea rcx, [rip + 0x1513c36]
141726f3a: call 0x1428d9250
141726f3f: jmp 0x14172746a
141726f44: mov byte ptr [rbx + 0x7d], 0
141726f48: lea rcx, [rip + 0x1513c21]
141726f4f: call 0x1428d9250
141726f54: jmp 0x14172746a
141726f59: mov byte ptr [rbx + 0x8f], 0
141726f60: lea rcx, [rip + 0x1513c09]
141726f67: call 0x1428d9250
141726f6c: jmp 0x14172746a
141726f71: mov byte ptr [rbx + 0x8f], 1
141726f78: lea rcx, [rip + 0x1513bf1]
141726f7f: call 0x1428d9250
141726f84: jmp 0x14172746a
141726f89: lea rcx, [rip + 0x1513be0]
141726f90: call 0x1428d9250
141726f95: jmp 0x14172746a
141726f9a: lea rcx, [rip + 0x1513bcf]
141726fa1: call 0x1428d9250
141726fa6: jmp 0x14172746a
141726fab: mov byte ptr [rbx + 0x7e], 1
141726faf: lea rcx, [rip + 0x1513bba]
141726fb6: call 0x1428d9250
141726fbb: jmp 0x14172746a
141726fc0: mov byte ptr [rbx + 0x90], 1
141726fc7: lea rcx, [rip + 0x1513ba2]
141726fce: call 0x1428d9250
141726fd3: jmp 0x14172746a
141726fd8: mov byte ptr [rbx + 0x7f], 1
141726fdc: lea rcx, [rip + 0x1513b8d]
141726fe3: call 0x1428d9250
141726fe8: jmp 0x14172746a
141726fed: lea rcx, [rip + 0x1513b7c]
141726ff4: call 0x1428d9250
141726ff9: jmp 0x14172746a
141726ffe: mov byte ptr [rbx + 0x7d], 1
141727002: lea rcx, [rip + 0x1513b67]
141727009: call 0x1428d9250
14172700e: jmp 0x14172746a
141727013: mov byte ptr [rbx + 0x90], 0
14172701a: lea rcx, [rip + 0x1513b4f]
141727021: call 0x1428d9250
141727026: jmp 0x14172746a
14172702b: lea rax, [rip + 0x155face]
141727032: mov qword ptr [rbx + 0x700], rax
141727039: lea rax, [rip + 0x1385ff0]
141727040: mov qword ptr [rsp + 0x20], rax
141727045: lea rcx, [rip + 0x1384fc4]
14172704c: lea r9, [rip + 0x1384f9d]
141727053: lea r8, [rbx + 0x700]
14172705a: mov edx, 0x2b
14172705f: call 0x1428d9760
141727064: jmp 0x14172746a
141727069: lea rcx, [rsi + 0x20]
14172706d: mov byte ptr [rbx + 0x94], 0
141727074: mov qword ptr [rsp + 0x20], 1
14172707d: mov r9d, 1
141727083: call 0x1428a9a60
141727088: mov rdx, qword ptr [rsi + 0x30]
14172708c: mov r8, rdi
14172708f: mov r15, qword ptr [rsi + 0x28]
141727093: sub r8, rdx
141727096: ja 0x141726a53
14172709c: jmp 0x141726a67
1417270a1: lea rcx, [rip + 0x1513ac8]
1417270a8: call 0x1428d9250
1417270ad: jmp 0x14172746a
1417270b2: mov byte ptr [rbx + 0x7e], 0
1417270b6: lea rcx, [rip + 0x1513ab3]
1417270bd: call 0x1428d9250
1417270c2: jmp 0x14172746a
1417270c7: mov byte ptr [rbx + 0x8d], 1
1417270ce: mov byte ptr [rbx + 0x8e], 0
1417270d5: mov rcx, rsi
1417270d8: xor edx, edx
1417270da: call 0x1428cdb90
1417270df: jmp 0x141723d98
1417270e4: mov byte ptr [rbx + 0x8d], 1
1417270eb: mov byte ptr [rbx + 0x8e], 1
1417270f2: mov rcx, rsi
1417270f5: xor edx, edx
1417270f7: call 0x1428cdb90
1417270fc: jmp 0x141723b9d
141727101: mov byte ptr [rbx + 0x7e], 0
141727105: lea rcx, [rip + 0x1513a64]
14172710c: call 0x1428d9250
141727111: jmp 0x14172746a
141727116: cmp rdx, 0x15
14172711a: jae 0x14172746c
141727120: lea r9, [rbx + 0x700]
141727127: mov r8d, 1
14172712d: call 0x140fe41e0
141727132: jmp 0x141726084
141727137: lea rcx, [rip + 0x15a3642]
14172713e: call 0x1428d9310
141727143: jmp 0x14172746a
141727148: lea rax, [rip + 0x15a3a41]
14172714f: mov qword ptr [rbx + 0x700], rax
141727156: mov qword ptr [rbx + 0x708], 1
141727161: mov qword ptr [rbx + 0x710], 8
14172716c: xorps xmm0, xmm0
14172716f: movups xmmword ptr [rbx + 0x718], xmm0
141727176: lea rdx, [rip + 0x15a3a23]
14172717d: lea rcx, [rbx + 0x700]
141727184: call 0x1428d9390
141727189: jmp 0x14172746a
14172718e: lea rax, [rip + 0x15a3d03]
141727195: mov qword ptr [rbx + 0x700], rax
14172719c: mov qword ptr [rbx + 0x708], 1
1417271a7: mov qword ptr [rbx + 0x710], 8
1417271b2: xorps xmm0, xmm0
1417271b5: movups xmmword ptr [rbx + 0x718], xmm0
1417271bc: lea rdx, [rip + 0x15a3ce5]
1417271c3: lea rcx, [rbx + 0x700]
1417271ca: call 0x1428d9390
1417271cf: jmp 0x14172746a
1417271d4: lea rax, [rip + 0x15a3c0d]
1417271db: mov qword ptr [rbx + 0x700], rax
1417271e2: mov qword ptr [rbx + 0x708], 1
1417271ed: mov qword ptr [rbx + 0x710], 8
1417271f8: xorps xmm0, xmm0
1417271fb: movups xmmword ptr [rbx + 0x718], xmm0
141727202: lea rdx, [rip + 0x15a3bef]
141727209: lea rcx, [rbx + 0x700]
141727210: call 0x1428d9390
141727215: jmp 0x14172746a
14172721a: lea rax, [rip + 0x15a3db7]
141727221: mov qword ptr [rbx + 0x480], rax
141727228: mov qword ptr [rbx + 0x488], 0x1e
141727233: mov qword ptr [rbx + 0x490], rsi
14172723a: mov qword ptr [rbx + 0x200], 3
141727245: mov byte ptr [rbx + 0xb2], 1
14172724c: lea rcx, [rbx + 0x480]
141727253: lea rdx, [rbx + 0x200]
14172725a: call 0x1428a8a20
14172725f: mov rsi, rax
141727262: mov rdx, qword ptr [rbx + 0x15a0]
141727269: test rdx, rdx
14172726c: je 0x141727280
14172726e: mov rcx, qword ptr [rbx + 0x15a8]
141727275: mov r8d, 1
14172727b: call 0x140613c20
141727280: mov qword ptr [rbx + 0x700], rsi
141727287: lea rax, [rip + 0x15a3cba]
14172728e: mov qword ptr [rsp + 0x20], rax
141727293: lea rcx, [rip + 0x15a3c97]
14172729a: lea r9, [rip + 0x15a2f27]
1417272a1: lea r8, [rbx + 0x700]
1417272a8: mov edx, 0x16
1417272ad: call 0x1428d9760
1417272b2: jmp 0x14172746a
1417272b7: mov ecx, 0x80
1417272bc: mov edx, 0x200
1417272c1: call 0x1428d9000
1417272c6: jmp 0x14172746a
1417272cb: lea r8, [rip + 0x15a2fae]
1417272d2: mov ecx, 1
1417272d7: mov edx, 0xb
1417272dc: call 0x1428d8fe3
1417272e1: jmp 0x14172746a
1417272e6: mov byte ptr [rbx + 0x7f], 0
1417272ea: lea rcx, [rip + 0x15137af]
1417272f1: call 0x1428d80d0
1417272f6: jmp 0x14172746a
1417272fb: mov byte ptr [rbx + 0x7d], 0
1417272ff: lea rcx, [rip + 0x151379a]
141727306: call 0x1428d80d0
14172730b: jmp 0x14172746a
141727310: mov byte ptr [rbx + 0x8f], 0
141727317: lea rcx, [rip + 0x1513782]
14172731e: call 0x1428d80d0
141727323: jmp 0x14172746a
141727328: mov byte ptr [rbx + 0x8f], 1
14172732f: lea rcx, [rip + 0x151376a]
141727336: call 0x1428d80d0
14172733b: jmp 0x14172746a
141727340: lea rcx, [rip + 0x1513759]
141727347: call 0x1428d80d0
14172734c: jmp 0x14172746a
141727351: lea rcx, [rip + 0x1513748]
141727358: call 0x1428d80d0
14172735d: jmp 0x14172746a
141727362: mov byte ptr [rbx + 0x7e], 1
141727366: lea rcx, [rip + 0x1513733]
14172736d: call 0x1428d80d0
141727372: jmp 0x14172746a
141727377: mov byte ptr [rbx + 0x90], 1
14172737e: lea rcx, [rip + 0x151371b]
141727385: call 0x1428d80d0
14172738a: jmp 0x14172746a
14172738f: mov byte ptr [rbx + 0x7f], 1
141727393: lea rcx, [rip + 0x1513706]
14172739a: call 0x1428d80d0
14172739f: jmp 0x14172746a
1417273a4: mov rax, qword ptr [rbx + 0x708]
1417273ab: mov qword ptr [rbx + 0x200], rax
1417273b2: lea rax, [rip + 0x15a381f]
1417273b9: mov qword ptr [rsp + 0x20], rax
1417273be: lea rcx, [rip + 0x15a2e43]
1417273c5: lea r9, [rip + 0x15a2e1c]
1417273cc: lea r8, [rbx + 0x200]
1417273d3: mov edx, 0x2b
1417273d8: call 0x1428d9760
1417273dd: jmp 0x14172746a
1417273e2: lea rcx, [rip + 0x15136b7]
1417273e9: call 0x1428d80d0
1417273ee: jmp 0x14172746a
1417273f0: mov byte ptr [rbx + 0x7d], 1
1417273f4: lea rcx, [rip + 0x15136a5]
1417273fb: call 0x1428d80d0
141727400: jmp 0x14172746a
141727402: mov byte ptr [rbx + 0x90], 0
141727409: lea rcx, [rip + 0x1513690]
141727410: call 0x1428d80d0
141727415: jmp 0x14172746a
141727417: lea rcx, [rip + 0x1513682]
14172741e: call 0x1428d80d0
141727423: jmp 0x14172746a
141727425: lea rax, [rip + 0x15a2fe4]
14172742c: mov qword ptr [rsp + 0x20], rax
141727431: lea rcx, [rip + 0x15a3040]
141727438: mov edx, 0x33
14172743d: mov r9d, 0x33
141727443: call 0x1428d9940
141727448: mov byte ptr [rbx + 0x7e], 0
14172744c: lea rcx, [rip + 0x151364d]
141727453: call 0x1428d80d0
141727458: jmp 0x14172746a
14172745a: mov byte ptr [rbx + 0x7e], 0
14172745e: lea rcx, [rip + 0x151363b]
141727465: call 0x1428d80d0
14172746a: ud2 
14172746c: lea r8, [rbx + 0x700]
141727473: call 0x140b4c000
141727478: jmp 0x141726084
14172747d: nop dword ptr [rax]
141727480: mov qword ptr [rsp + 0x10], rdx
141727485: push rbp
141727486: push r15
141727488: push r14
14172748a: push r13
14172748c: push r12
14172748e: push rsi
14172748f: push rdi
141727490: push rbx
141727491: sub rsp, 0x98
141727498: lea rbp, [rdx + 0x80]
14172749f: movaps xmmword ptr [rsp + 0x70], xmm7
1417274a4: movaps xmmword ptr [rsp + 0x80], xmm6
1417274ac: and rdx, 0xffffffffffffff80
1417274b0: mov rbx, rdx
1417274b3: movzx esi, byte ptr [rbx + 0xa6]
1417274ba: mov rcx, qword ptr [rbx + 0xb8]
1417274c1: call 0x14146cb40
1417274c6: mov byte ptr [rbx + 0xc8], sil
1417274cd: movaps xmm6, xmmword ptr [rsp + 0x80]
1417274d5: movaps xmm7, xmmword ptr [rsp + 0x70]
1417274da: add rsp, 0x98
1417274e1: pop rbx
1417274e2: pop rdi
1417274e3: pop rsi
1417274e4: pop r12
1417274e6: pop r13
1417274e8: pop r14
1417274ea: pop r15
1417274ec: pop rbp
1417274ed: ret 
1417274ee: nop 
1417274f0: mov qword ptr [rsp + 0x10], rdx
1417274f5: push rbp
1417274f6: push r15
1417274f8: push r14
1417274fa: push r13
1417274fc: push r12
1417274fe: push rsi
1417274ff: push rdi
141727500: push rbx
141727501: sub rsp, 0x98
141727508: lea rbp, [rdx + 0x80]
14172750f: movaps xmmword ptr [rsp + 0x70], xmm7
141727514: movaps xmmword ptr [rsp + 0x80], xmm6
14172751c: and rdx, 0xffffffffffffff80
141727520: mov rbx, rdx
141727523: cmp byte ptr [rbx + 0xb1], 0
14172752a: je 0x141727538
14172752c: lea rcx, [rbx + 0x710]
141727533: call 0x14088ed10
141727538: lea rcx, [rbx + 0x790]
14172753f: call 0x140890ff0
141727544: lea rcx, [rbx + 0x7d8]
14172754b: call 0x14087cfb0
141727550: lea rcx, [rbx + 0x700]
141727557: call 0x140878420
14172755c: lea rcx, [rbx + 0x968]
141727563: lea rdx, [rbx + 0x988]
14172756a: mov r8d, 0x20
141727570: mov r9d, 0x10
141727576: call 0x140052a30
14172757b: mov byte ptr [rbx + 0xa6], 1
141727582: movaps xmm6, xmmword ptr [rsp + 0x80]
14172758a: movaps xmm7, xmmword ptr [rsp + 0x70]
14172758f: add rsp, 0x98
141727596: pop rbx
141727597: pop rdi
141727598: pop rsi
141727599: pop r12
14172759b: pop r13
14172759d: pop r14
14172759f: pop r15
1417275a1: pop rbp
1417275a2: ret 
1417275a3: nop word ptr cs:[rax + rax]
1417275b0: mov qword ptr [rsp + 0x10], rdx
1417275b5: push rbp
1417275b6: push r15
1417275b8: push r14
1417275ba: push r13
1417275bc: push r12
1417275be: push rsi
1417275bf: push rdi
1417275c0: push rbx
1417275c1: sub rsp, 0x98
1417275c8: lea rbp, [rdx + 0x80]
1417275cf: movaps xmmword ptr [rsp + 0x70], xmm7
1417275d4: movaps xmmword ptr [rsp + 0x80], xmm6
1417275dc: and rdx, 0xffffffffffffff80
1417275e0: mov rbx, rdx
1417275e3: mov rdx, qword ptr [rbx + 0x140]
1417275ea: test rdx, rdx
1417275ed: je 0x141727601
1417275ef: mov rcx, qword ptr [rbx + 0x148]
1417275f6: mov r8d, 1
1417275fc: call 0x140613c20
141727601: mov byte ptr [rbx + 0xa4], 1
141727608: mov byte ptr [rbx + 0xa5], 1
14172760f: movaps xmm6, xmmword ptr [rsp + 0x80]
141727617: movaps xmm7, xmmword ptr [rsp + 0x70]
14172761c: add rsp, 0x98
141727623: pop rbx
141727624: pop rdi
141727625: pop rsi
141727626: pop r12
141727628: pop r13
14172762a: pop r14
14172762c: pop r15
14172762e: pop rbp
14172762f: ret 
141727630: mov qword ptr [rsp + 0x10], rdx
141727635: push rbp
141727636: push r15
141727638: push r14
14172763a: push r13
14172763c: push r12
14172763e: push rsi
14172763f: push rdi
141727640: push rbx
141727641: sub rsp, 0x98
141727648: lea rbp, [rdx + 0x80]
14172764f: movaps xmmword ptr [rsp + 0x70], xmm7
141727654: movaps xmmword ptr [rsp + 0x80], xmm6
14172765c: and rdx, 0xffffffffffffff80
141727660: mov rbx, rdx
141727663: mov rdx, qword ptr [rbx + 0x480]
14172766a: test rdx, rdx
14172766d: je 0x141727681
14172766f: mov rcx, qword ptr [rbx + 0x488]
141727676: mov r8d, 1
14172767c: call 0x140613c20
141727681: movaps xmm6, xmmword ptr [rsp + 0x80]
141727689: movaps xmm7, xmmword ptr [rsp + 0x70]
14172768e: add rsp, 0x98
141727695: pop rbx
141727696: pop rdi
141727697: pop rsi
141727698: pop r12
14172769a: pop r13
14172769c: pop r14
14172769e: pop r15
1417276a0: pop rbp
1417276a1: ret 
1417276a2: nop word ptr cs:[rax + rax]
1417276b0: mov qword ptr [rsp + 0x10], rdx
1417276b5: push rbp
1417276b6: push r15
1417276b8: push r14
1417276ba: push r13
1417276bc: push r12
1417276be: push rsi
1417276bf: push rdi
1417276c0: push rbx
1417276c1: sub rsp, 0x98
1417276c8: lea rbp, [rdx + 0x80]
1417276cf: movaps xmmword ptr [rsp + 0x70], xmm7
1417276d4: movaps xmmword ptr [rsp + 0x80], xmm6
1417276dc: and rdx, 0xffffffffffffff80
1417276e0: mov rbx, rdx
1417276e3: mov rcx, qword ptr [rbx + 0x80]
1417276ea: mov rax, qword ptr [rbx + 0x98]
1417276f1: mov rdx, qword ptr [rax + 8]
1417276f5: test rdx, rdx
1417276f8: je 0x14172770a
1417276fa: mov rax, qword ptr [rbx + 0x98]
141727701: mov r8, qword ptr [rax + 0x10]
141727705: call 0x140613c20
14172770a: mov edx, 0x18
14172770f: mov r8d, 8
141727715: mov rcx, qword ptr [rbx + 0xa8]
14172771c: call 0x140613c20
141727721: movaps xmm6, xmmword ptr [rsp + 0x80]
141727729: movaps xmm7, xmmword ptr [rsp + 0x70]
14172772e: add rsp, 0x98
141727735: pop rbx
141727736: pop rdi
141727737: pop rsi
141727738: pop r12
14172773a: pop r13
14172773c: pop r14
14172773e: pop r15
141727740: pop rbp
141727741: ret 
141727742: nop word ptr cs:[rax + rax]
141727750: mov qword ptr [rsp + 0x10], rdx
141727755: push rbp
141727756: push r15
141727758: push r14
14172775a: push r13
14172775c: push r12
14172775e: push rsi
14172775f: push rdi
141727760: push rbx
141727761: sub rsp, 0x98
141727768: lea rbp, [rdx + 0x80]
14172776f: movaps xmmword ptr [rsp + 0x70], xmm7
141727774: movaps xmmword ptr [rsp + 0x80], xmm6
14172777c: and rdx, 0xffffffffffffff80
141727780: mov rbx, rdx
141727783: mov rax, qword ptr [rbx + 0xbd0]
14172778a: lock dec qword ptr [rax]
14172778e: jne 0x14172779c
141727790: lea rcx, [rbx + 0xbd0]
141727797: call 0x140d53bc0
14172779c: movaps xmm6, xmmword ptr [rsp + 0x80]
1417277a4: movaps xmm7, xmmword ptr [rsp + 0x70]
1417277a9: add rsp, 0x98
1417277b0: pop rbx
1417277b1: pop rdi
1417277b2: pop rsi
1417277b3: pop r12
1417277b5: pop r13
1417277b7: pop r14
1417277b9: pop r15
1417277bb: pop rbp
1417277bc: ret 
1417277bd: nop dword ptr [rax]
1417277c0: mov qword ptr [rsp + 0x10], rdx
1417277c5: push rbp
1417277c6: push r15
1417277c8: push r14
1417277ca: push r13
1417277cc: push r12
1417277ce: push rsi
1417277cf: push rdi
1417277d0: push rbx
1417277d1: sub rsp, 0x98
1417277d8: lea rbp, [rdx + 0x80]
1417277df: movaps xmmword ptr [rsp + 0x70], xmm7
1417277e4: movaps xmmword ptr [rsp + 0x80], xmm6
1417277ec: and rdx, 0xffffffffffffff80
1417277f0: mov rbx, rdx
1417277f3: mov qword ptr [rbx + 0xbc0], 2
1417277fe: lea rcx, [rbx + 0x480]
141727805: call 0x14171e300
14172780a: movaps xmm6, xmmword ptr [rsp + 0x80]
141727812: movaps xmm7, xmmword ptr [rsp + 0x70]
141727817: add rsp, 0x98
14172781e: pop rbx
14172781f: pop rdi
141727820: pop rsi
141727821: pop r12
141727823: pop r13
141727825: pop r14
141727827: pop r15
141727829: pop rbp
14172782a: ret 
14172782b: nop dword ptr [rax + rax]
141727830: mov qword ptr [rsp + 0x10], rdx
141727835: push rbp
141727836: push r15
141727838: push r14
14172783a: push r13
14172783c: push r12
14172783e: push rsi
14172783f: push rdi
141727840: push rbx
141727841: sub rsp, 0x98
141727848: lea rbp, [rdx + 0x80]
14172784f: movaps xmmword ptr [rsp + 0x70], xmm7
141727854: movaps xmmword ptr [rsp + 0x80], xmm6
14172785c: and rdx, 0xffffffffffffff80
141727860: mov rbx, rdx
141727863: mov rcx, qword ptr [rbx + 0x80]
14172786a: mov rax, qword ptr [rbx + 0x98]
141727871: mov rdx, qword ptr [rax + 8]
141727875: test rdx, rdx
141727878: je 0x14172788a
14172787a: mov rax, qword ptr [rbx + 0x98]
141727881: mov r8, qword ptr [rax + 0x10]
141727885: call 0x140613c20
14172788a: mov edx, 0x18
14172788f: mov r8d, 8
141727895: mov rcx, qword ptr [rbx + 0xa8]
14172789c: call 0x140613c20
1417278a1: movaps xmm6, xmmword ptr [rsp + 0x80]
1417278a9: movaps xmm7, xmmword ptr [rsp + 0x70]
1417278ae: add rsp, 0x98
1417278b5: pop rbx
1417278b6: pop rdi
1417278b7: pop rsi
1417278b8: pop r12
1417278ba: pop r13
1417278bc: pop r14
1417278be: pop r15
1417278c0: pop rbp
1417278c1: ret 
1417278c2: nop word ptr cs:[rax + rax]
1417278d0: mov qword ptr [rsp + 0x10], rdx
1417278d5: push rbp
1417278d6: push r15
1417278d8: push r14
1417278da: push r13
1417278dc: push r12
1417278de: push rsi
1417278df: push rdi
1417278e0: push rbx
1417278e1: sub rsp, 0x98
1417278e8: lea rbp, [rdx + 0x80]
1417278ef: movaps xmmword ptr [rsp + 0x70], xmm7
1417278f4: movaps xmmword ptr [rsp + 0x80], xmm6
1417278fc: and rdx, 0xffffffffffffff80
141727900: mov rbx, rdx
141727903: mov rax, qword ptr [rbx + 0xe40]
14172790a: lock dec qword ptr [rax]
14172790e: jne 0x14172791c
141727910: lea rcx, [rbx + 0xe40]
141727917: call 0x140d53bc0
14172791c: movaps xmm6, xmmword ptr [rsp + 0x80]
141727924: movaps xmm7, xmmword ptr [rsp + 0x70]
141727929: add rsp, 0x98
141727930: pop rbx
141727931: pop rdi
141727932: pop rsi
141727933: pop r12
141727935: pop r13
141727937: pop r14
141727939: pop r15
14172793b: pop rbp
14172793c: ret 
14172793d: nop dword ptr [rax]
141727940: mov qword ptr [rsp + 0x10], rdx
141727945: push rbp
141727946: push r15
141727948: push r14
14172794a: push r13
14172794c: push r12
14172794e: push rsi
14172794f: push rdi
141727950: push rbx
141727951: sub rsp, 0x98
141727958: lea rbp, [rdx + 0x80]
14172795f: movaps xmmword ptr [rsp + 0x70], xmm7
141727964: movaps xmmword ptr [rsp + 0x80], xmm6
14172796c: and rdx, 0xffffffffffffff80
141727970: mov rbx, rdx
141727973: mov qword ptr [rbx + 0xe30], 2
14172797e: lea rcx, [rbx + 0x480]
141727985: call 0x14171e300
14172798a: movaps xmm6, xmmword ptr [rsp + 0x80]
141727992: movaps xmm7, xmmword ptr [rsp + 0x70]
141727997: add rsp, 0x98
14172799e: pop rbx
14172799f: pop rdi
1417279a0: pop rsi
1417279a1: pop r12
1417279a3: pop r13
1417279a5: pop r14
1417279a7: pop r15
1417279a9: pop rbp
1417279aa: ret 
1417279ab: nop dword ptr [rax + rax]
1417279b0: mov qword ptr [rsp + 0x10], rdx
1417279b5: push rbp
1417279b6: push r15
1417279b8: push r14
1417279ba: push r13
1417279bc: push r12
1417279be: push rsi
1417279bf: push rdi
1417279c0: push rbx
1417279c1: sub rsp, 0x98
1417279c8: lea rbp, [rdx + 0x80]
1417279cf: movaps xmmword ptr [rsp + 0x70], xmm7
1417279d4: movaps xmmword ptr [rsp + 0x80], xmm6
1417279dc: and rdx, 0xffffffffffffff80
1417279e0: mov rbx, rdx
1417279e3: mov rax, qword ptr [rbx + 0xbd0]
1417279ea: lock dec qword ptr [rax]
1417279ee: jne 0x1417279fc
1417279f0: lea rcx, [rbx + 0xbd0]
1417279f7: call 0x140d53bc0
1417279fc: movaps xmm6, xmmword ptr [rsp + 0x80]
141727a04: movaps xmm7, xmmword ptr [rsp + 0x70]
141727a09: add rsp, 0x98
141727a10: pop rbx
141727a11: pop rdi
141727a12: pop rsi
141727a13: pop r12
141727a15: pop r13
141727a17: pop r14
141727a19: pop r15
141727a1b: pop rbp
141727a1c: ret 
141727a1d: nop dword ptr [rax]
141727a20: mov qword ptr [rsp + 0x10], rdx
141727a25: push rbp
141727a26: push r15
141727a28: push r14
141727a2a: push r13
141727a2c: push r12
141727a2e: push rsi
141727a2f: push rdi
141727a30: push rbx
141727a31: sub rsp, 0x98
141727a38: lea rbp, [rdx + 0x80]
141727a3f: movaps xmmword ptr [rsp + 0x70], xmm7
141727a44: movaps xmmword ptr [rsp + 0x80], xmm6
141727a4c: and rdx, 0xffffffffffffff80
141727a50: mov rbx, rdx
141727a53: mov rax, qword ptr [rbx + 0x80]
141727a5a: mov qword ptr [rbx + 0xbc0], rax
141727a61: lea rdx, [rbx + 0x200]
141727a68: mov r8d, 0x268
141727a6e: lea rcx, [rbx + 0xbc8]
141727a75: call 0x142923c00
141727a7a: movaps xmm6, xmmword ptr [rsp + 0x80]
141727a82: movaps xmm7, xmmword ptr [rsp + 0x70]
141727a87: add rsp, 0x98
141727a8e: pop rbx
141727a8f: pop rdi
141727a90: pop rsi
141727a91: pop r12
141727a93: pop r13
141727a95: pop r14
141727a97: pop r15
141727a99: pop rbp
141727a9a: ret 
141727a9b: nop dword ptr [rax + rax]
141727aa0: mov qword ptr [rsp + 0x10], rdx
141727aa5: push rbp
141727aa6: push r15
141727aa8: push r14
141727aaa: push r13
141727aac: push r12
141727aae: push rsi
141727aaf: push rdi
141727ab0: push rbx
141727ab1: sub rsp, 0x98
141727ab8: lea rbp, [rdx + 0x80]
141727abf: movaps xmmword ptr [rsp + 0x70], xmm7
141727ac4: movaps xmmword ptr [rsp + 0x80], xmm6
141727acc: and rdx, 0xffffffffffffff80
141727ad0: mov rbx, rdx
141727ad3: mov rcx, qword ptr [rbx + 0x80]
141727ada: mov rax, qword ptr [rbx + 0x98]
141727ae1: mov rdx, qword ptr [rax + 8]
141727ae5: test rdx, rdx
141727ae8: je 0x141727afa
141727aea: mov rax, qword ptr [rbx + 0x98]
141727af1: mov r8, qword ptr [rax + 0x10]
141727af5: call 0x140613c20
141727afa: mov edx, 0x18
141727aff: mov r8d, 8
141727b05: mov rcx, qword ptr [rbx + 0xa8]
141727b0c: call 0x140613c20
141727b11: movaps xmm6, xmmword ptr [rsp + 0x80]
141727b19: movaps xmm7, xmmword ptr [rsp + 0x70]
141727b1e: add rsp, 0x98
141727b25: pop rbx
141727b26: pop rdi
141727b27: pop rsi
141727b28: pop r12
141727b2a: pop r13
141727b2c: pop r14
141727b2e: pop r15
141727b30: pop rbp
141727b31: ret 
141727b32: nop word ptr cs:[rax + rax]
141727b40: mov qword ptr [rsp + 0x10], rdx
141727b45: push rbp
141727b46: push r15
141727b48: push r14
141727b4a: push r13
141727b4c: push r12
141727b4e: push rsi
141727b4f: push rdi
141727b50: push rbx
141727b51: sub rsp, 0x98
141727b58: lea rbp, [rdx + 0x80]
141727b5f: movaps xmmword ptr [rsp + 0x70], xmm7
141727b64: movaps xmmword ptr [rsp + 0x80], xmm6
141727b6c: and rdx, 0xffffffffffffff80
141727b70: mov rbx, rdx
141727b73: lea rcx, [rbx + 0x700]
141727b7a: call 0x1408802e0
141727b7f: movaps xmm6, xmmword ptr [rsp + 0x80]
141727b87: movaps xmm7, xmmword ptr [rsp + 0x70]
141727b8c: add rsp, 0x98
141727b93: pop rbx
141727b94: pop rdi
141727b95: pop rsi
141727b96: pop r12
141727b98: pop r13
141727b9a: pop r14
141727b9c: pop r15
141727b9e: pop rbp
141727b9f: ret 
141727ba0: mov qword ptr [rsp + 0x10], rdx
141727ba5: push rbp
141727ba6: push r15
141727ba8: push r14
141727baa: push r13
141727bac: push r12
141727bae: push rsi
141727baf: push rdi
141727bb0: push rbx
141727bb1: sub rsp, 0x98
141727bb8: lea rbp, [rdx + 0x80]
141727bbf: movaps xmmword ptr [rsp + 0x70], xmm7
141727bc4: movaps xmmword ptr [rsp + 0x80], xmm6
141727bcc: and rdx, 0xffffffffffffff80
141727bd0: mov rbx, rdx
141727bd3: lea rcx, [rbx + 0x1340]
141727bda: call 0x140890ff0
141727bdf: mov byte ptr [rbx + 0xb2], 0
141727be6: movaps xmm6, xmmword ptr [rsp + 0x80]
141727bee: movaps xmm7, xmmword ptr [rsp + 0x70]
141727bf3: add rsp, 0x98
141727bfa: pop rbx
141727bfb: pop rdi
141727bfc: pop rsi
141727bfd: pop r12
141727bff: pop r13
141727c01: pop r14
141727c03: pop r15
141727c05: pop rbp
141727c06: ret 
141727c07: nop word ptr [rax + rax]
141727c10: mov qword ptr [rsp + 0x10], rdx
141727c15: push rbp
141727c16: push r15
141727c18: push r14
141727c1a: push r13
141727c1c: push r12
141727c1e: push rsi
141727c1f: push rdi
141727c20: push rbx
141727c21: sub rsp, 0x98
141727c28: lea rbp, [rdx + 0x80]
141727c2f: movaps xmmword ptr [rsp + 0x70], xmm7
141727c34: movaps xmmword ptr [rsp + 0x80], xmm6
141727c3c: and rdx, 0xffffffffffffff80
141727c40: mov rbx, rdx
141727c43: mov rdx, qword ptr [rbx + 0x200]
141727c4a: test rdx, rdx
141727c4d: je 0x141727c61
141727c4f: mov rcx, qword ptr [rbx + 0x208]
141727c56: mov r8d, 1
141727c5c: call 0x140613c20
141727c61: movaps xmm6, xmmword ptr [rsp + 0x80]
141727c69: movaps xmm7, xmmword ptr [rsp + 0x70]
141727c6e: add rsp, 0x98
141727c75: pop rbx
141727c76: pop rdi
141727c77: pop rsi
141727c78: pop r12
141727c7a: pop r13
141727c7c: pop r14
141727c7e: pop r15
141727c80: pop rbp
141727c81: ret 
141727c82: nop word ptr cs:[rax + rax]
141727c90: mov qword ptr [rsp + 0x10], rdx
141727c95: push rbp
141727c96: push r15
141727c98: push r14
141727c9a: push r13
141727c9c: push r12
141727c9e: push rsi
141727c9f: push rdi
141727ca0: push rbx
141727ca1: sub rsp, 0x98
141727ca8: lea rbp, [rdx + 0x80]
141727caf: movaps xmmword ptr [rsp + 0x70], xmm7
141727cb4: movaps xmmword ptr [rsp + 0x80], xmm6
141727cbc: and rdx, 0xffffffffffffff80
141727cc0: mov rbx, rdx
141727cc3: cmp byte ptr [rbx + 0xc7], 0
141727cca: je 0x141727cea
141727ccc: mov rdx, qword ptr [rbx + 0x200]
141727cd3: test rdx, rdx
141727cd6: je 0x141727cea
141727cd8: mov rcx, qword ptr [rbx + 0x208]
141727cdf: mov r8d, 1
141727ce5: call 0x140613c20
141727cea: mov rdx, qword ptr [rbx + 0x700]
141727cf1: test rdx, rdx
141727cf4: je 0x141727d08
141727cf6: mov rcx, qword ptr [rbx + 0x708]
141727cfd: mov r8d, 1
141727d03: call 0x140613c20
141727d08: movaps xmm6, xmmword ptr [rsp + 0x80]
141727d10: movaps xmm7, xmmword ptr [rsp + 0x70]
141727d15: add rsp, 0x98
141727d1c: pop rbx
141727d1d: pop rdi
141727d1e: pop rsi
141727d1f: pop r12
141727d21: pop r13
141727d23: pop r14
141727d25: pop r15
141727d27: pop rbp
141727d28: ret 
141727d29: nop dword ptr [rax]
141727d30: mov qword ptr [rsp + 0x10], rdx
141727d35: push rbp
141727d36: push r15
141727d38: push r14
141727d3a: push r13
141727d3c: push r12
141727d3e: push rsi
141727d3f: push rdi
141727d40: push rbx
141727d41: sub rsp, 0x98
141727d48: lea rbp, [rdx + 0x80]
141727d4f: movaps xmmword ptr [rsp + 0x70], xmm7
141727d54: movaps xmmword ptr [rsp + 0x80], xmm6
141727d5c: and rdx, 0xffffffffffffff80
141727d60: mov rbx, rdx
141727d63: cmp byte ptr [rbx + 0xb2], 0
141727d6a: je 0x141727d8a
141727d6c: mov rdx, qword ptr [rbx + 0x15a0]
141727d73: test rdx, rdx
141727d76: je 0x141727d8a
141727d78: mov rcx, qword ptr [rbx + 0x15a8]
141727d7f: mov r8d, 1
141727d85: call 0x140613c20
141727d8a: mov byte ptr [rbx + 0xa4], 1
141727d91: mov byte ptr [rbx + 0xa5], 1
141727d98: movaps xmm6, xmmword ptr [rsp + 0x80]
141727da0: movaps xmm7, xmmword ptr [rsp + 0x70]
141727da5: add rsp, 0x98
141727dac: pop rbx
141727dad: pop rdi
141727dae: pop rsi
141727daf: pop r12
141727db1: pop r13
141727db3: pop r14
141727db5: pop r15
141727db7: pop rbp
141727db8: ret 
141727db9: nop dword ptr [rax]
141727dc0: mov qword ptr [rsp + 0x10], rdx
141727dc5: push rbp
141727dc6: push r15
141727dc8: push r14
141727dca: push r13
141727dcc: push r12
141727dce: push rsi
141727dcf: push rdi
141727dd0: push rbx
141727dd1: sub rsp, 0x98
141727dd8: lea rbp, [rdx + 0x80]
141727ddf: movaps xmmword ptr [rsp + 0x70], xmm7
141727de4: movaps xmmword ptr [rsp + 0x80], xmm6
141727dec: and rdx, 0xffffffffffffff80
141727df0: mov rbx, rdx
141727df3: mov byte ptr [rbx + 0xa4], 1
141727dfa: mov byte ptr [rbx + 0xa5], 1
141727e01: lea rcx, [rbx + 0x700]
141727e08: call 0x140d4cc30
141727e0d: mov byte ptr [rbx + 0xa4], 1
141727e14: mov byte ptr [rbx + 0xa5], 1
141727e1b: movaps xmm6, xmmword ptr [rsp + 0x80]
141727e23: movaps xmm7, xmmword ptr [rsp + 0x70]
141727e28: add rsp, 0x98
141727e2f: pop rbx
141727e30: pop rdi
141727e31: pop rsi
141727e32: pop r12
141727e34: pop r13
141727e36: pop r14
141727e38: pop r15
141727e3a: pop rbp
141727e3b: ret 
141727e3c: nop dword ptr [rax]
141727e40: mov qword ptr [rsp + 0x10], rdx
141727e45: push rbp
141727e46: push r15
141727e48: push r14
141727e4a: push r13
141727e4c: push r12
141727e4e: push rsi
141727e4f: push rdi
141727e50: push rbx
141727e51: sub rsp, 0x98
141727e58: lea rbp, [rdx + 0x80]
141727e5f: movaps xmmword ptr [rsp + 0x70], xmm7
141727e64: movaps xmmword ptr [rsp + 0x80], xmm6
141727e6c: and rdx, 0xffffffffffffff80
141727e70: mov rbx, rdx
141727e73: lea rcx, [rbx + 0x480]
141727e7a: call 0x14088ed10
141727e7f: lea rcx, [rbx + 0x1340]
141727e86: call 0x140890ff0
141727e8b: movzx eax, byte ptr [rbx + 0x80]
141727e92: mov byte ptr [rbx + 0xa4], al
141727e98: mov byte ptr [rbx + 0xa5], 0
141727e9f: movaps xmm6, xmmword ptr [rsp + 0x80]
141727ea7: movaps xmm7, xmmword ptr [rsp + 0x70]
141727eac: add rsp, 0x98
141727eb3: pop rbx
141727eb4: pop rdi
141727eb5: pop rsi
141727eb6: pop r12
141727eb8: pop r13
141727eba: pop r14
141727ebc: pop r15
141727ebe: pop rbp
141727ebf: ret 
141727ec0: mov qword ptr [rsp + 0x10], rdx
141727ec5: push rbp
141727ec6: push r15
141727ec8: push r14
141727eca: push r13
141727ecc: push r12
141727ece: push rsi
141727ecf: push rdi
141727ed0: push rbx
141727ed1: sub rsp, 0x98
141727ed8: lea rbp, [rdx + 0x80]
141727edf: movaps xmmword ptr [rsp + 0x70], xmm7
141727ee4: movaps xmmword ptr [rsp + 0x80], xmm6
141727eec: and rdx, 0xffffffffffffff80
141727ef0: mov rbx, rdx
141727ef3: mov byte ptr [rbx + 0xb4], 1
141727efa: lea rcx, [rbx + 0x700]
141727f01: call 0x1408800b0
141727f06: mov byte ptr [rbx + 0xb4], 1
141727f0d: movaps xmm6, xmmword ptr [rsp + 0x80]
141727f15: movaps xmm7, xmmword ptr [rsp + 0x70]
141727f1a: add rsp, 0x98
141727f21: pop rbx
141727f22: pop rdi
141727f23: pop rsi
141727f24: pop r12
141727f26: pop r13
141727f28: pop r14
141727f2a: pop r15
141727f2c: pop rbp
141727f2d: ret 
141727f2e: nop 
141727f30: mov qword ptr [rsp + 0x10], rdx
141727f35: push rbp
141727f36: push r15
141727f38: push r14
141727f3a: push r13
141727f3c: push r12
141727f3e: push rsi
141727f3f: push rdi
141727f40: push rbx
141727f41: sub rsp, 0x98
141727f48: lea rbp, [rdx + 0x80]
141727f4f: movaps xmmword ptr [rsp + 0x70], xmm7
141727f54: movaps xmmword ptr [rsp + 0x80], xmm6
141727f5c: and rdx, 0xffffffffffffff80
141727f60: mov rbx, rdx
141727f63: mov rax, qword ptr [rbx + 0xe0]
141727f6a: lock dec qword ptr [rax]
141727f6e: jne 0x141727f83
141727f70: lea rcx, [rbx + 0xe0]
141727f77: mov byte ptr [rbx + 0x96], 0
141727f7e: call 0x140d53db0
141727f83: mov byte ptr [rbx + 0x96], 0
141727f8a: movaps xmm6, xmmword ptr [rsp + 0x80]
141727f92: movaps xmm7, xmmword ptr [rsp + 0x70]
141727f97: add rsp, 0x98
141727f9e: pop rbx
141727f9f: pop rdi
141727fa0: pop rsi
141727fa1: pop r12
141727fa3: pop r13
141727fa5: pop r14
141727fa7: pop r15
141727fa9: pop rbp
141727faa: ret 
141727fab: nop dword ptr [rax + rax]
141727fb0: mov qword ptr [rsp + 0x10], rdx
141727fb5: push rbp
141727fb6: push r15
141727fb8: push r14
141727fba: push r13
141727fbc: push r12
141727fbe: push rsi
141727fbf: push rdi
141727fc0: push rbx
141727fc1: sub rsp, 0x98
141727fc8: lea rbp, [rdx + 0x80]
141727fcf: movaps xmmword ptr [rsp + 0x70], xmm7
141727fd4: movaps xmmword ptr [rsp + 0x80], xmm6
141727fdc: and rdx, 0xffffffffffffff80
141727fe0: mov rbx, rdx
141727fe3: mov rcx, qword ptr [rbx + 0x80]
141727fea: mov rax, qword ptr [rbx + 0x98]
141727ff1: mov rdx, qword ptr [rax + 8]
141727ff5: test rdx, rdx
141727ff8: je 0x14172800a
141727ffa: mov rax, qword ptr [rbx + 0x98]
141728001: mov r8, qword ptr [rax + 0x10]
141728005: call 0x140613c20
14172800a: mov edx, 0x18
14172800f: mov r8d, 8
141728015: mov rcx, qword ptr [rbx + 0xa8]
14172801c: call 0x140613c20
141728021: mov byte ptr [rbx + 0x96], 0
141728028: movaps xmm6, xmmword ptr [rsp + 0x80]
141728030: movaps xmm7, xmmword ptr [rsp + 0x70]
141728035: add rsp, 0x98
14172803c: pop rbx
14172803d: pop rdi
14172803e: pop rsi
14172803f: pop r12
141728041: pop r13
141728043: pop r14
141728045: pop r15
141728047: pop rbp
141728048: ret 
141728049: nop dword ptr [rax]
141728050: mov qword ptr [rsp + 0x10], rdx
141728055: push rbp
141728056: push r15
141728058: push r14
14172805a: push r13
14172805c: push r12
14172805e: push rsi
14172805f: push rdi
141728060: push rbx
141728061: sub rsp, 0x98
141728068: lea rbp, [rdx + 0x80]
14172806f: movaps xmmword ptr [rsp + 0x70], xmm7
141728074: movaps xmmword ptr [rsp + 0x80], xmm6
14172807c: and rdx, 0xffffffffffffff80
141728080: mov rbx, rdx
141728083: mov edx, 0xf0
141728088: mov r8d, 8
14172808e: mov rcx, qword ptr [rbx + 0xb8]
141728095: call 0x140613c20
14172809a: mov byte ptr [rbx + 0xc8], 0
1417280a1: movaps xmm6, xmmword ptr [rsp + 0x80]
1417280a9: movaps xmm7, xmmword ptr [rsp + 0x70]
1417280ae: add rsp, 0x98
1417280b5: pop rbx
1417280b6: pop rdi
1417280b7: pop rsi
1417280b8: pop r12
1417280ba: pop r13
1417280bc: pop r14
1417280be: pop r15
1417280c0: pop rbp
1417280c1: ret 
1417280c2: nop word ptr cs:[rax + rax]
1417280d0: mov qword ptr [rsp + 0x10], rdx
1417280d5: push rbp
1417280d6: push r15
1417280d8: push r14
1417280da: push r13
1417280dc: push r12
1417280de: push rsi
1417280df: push rdi
1417280e0: push rbx
1417280e1: sub rsp, 0x98
1417280e8: lea rbp, [rdx + 0x80]
1417280ef: movaps xmmword ptr [rsp + 0x70], xmm7
1417280f4: movaps xmmword ptr [rsp + 0x80], xmm6
1417280fc: and rdx, 0xffffffffffffff80
141728100: mov rbx, rdx
141728103: cmp qword ptr [rbx + 0x80], 0
14172810b: jne 0x14172815b
14172810d: mov rdx, qword ptr [rbx + 0x710]
141728114: lea rax, [rdx*2]
14172811c: test rax, rax
14172811f: je 0x141728133
141728121: mov rcx, qword ptr [rbx + 0x718]
141728128: mov r8d, 1
14172812e: call 0x140613c20
141728133: mov byte ptr [rbx + 0x96], 1
14172813a: movaps xmm6, xmmword ptr [rsp + 0x80]
141728142: movaps xmm7, xmmword ptr [rsp + 0x70]
141728147: add rsp, 0x98
14172814e: pop rbx
14172814f: pop rdi
141728150: pop rsi
141728151: pop r12
141728153: pop r13
141728155: pop r14
141728157: pop r15
141728159: pop rbp
14172815a: ret 
14172815b: mov byte ptr [rbx + 0x96], 1
141728162: movaps xmm6, xmmword ptr [rsp + 0x80]
14172816a: movaps xmm7, xmmword ptr [rsp + 0x70]
14172816f: add rsp, 0x98
141728176: pop rbx
141728177: pop rdi
141728178: pop rsi
141728179: pop r12
14172817b: pop r13
14172817d: pop r14
14172817f: pop r15
141728181: pop rbp
141728182: ret 
141728183: nop word ptr cs:[rax + rax]
141728190: mov qword ptr [rsp + 0x10], rdx
141728195: push rbp
141728196: push r15
141728198: push r14
14172819a: push r13
14172819c: push r12
14172819e: push rsi
14172819f: push rdi
1417281a0: push rbx
1417281a1: sub rsp, 0x98
1417281a8: lea rbp, [rdx + 0x80]
1417281af: movaps xmmword ptr [rsp + 0x70], xmm7
1417281b4: movaps xmmword ptr [rsp + 0x80], xmm6
1417281bc: and rdx, 0xffffffffffffff80
1417281c0: mov rbx, rdx
1417281c3: movzx esi, byte ptr [rbx + 0x96]
1417281ca: lea rcx, [rbx + 0xf0]
1417281d1: call 0x140878420
1417281d6: test sil, 1
1417281da: je 0x141728210
1417281dc: lea rcx, [rbx + 0x1b8]
1417281e3: call 0x1408778c0
1417281e8: mov byte ptr [rbx + 0xb4], sil
1417281ef: movaps xmm6, xmmword ptr [rsp + 0x80]
1417281f7: movaps xmm7, xmmword ptr [rsp + 0x70]
1417281fc: add rsp, 0x98
141728203: pop rbx
141728204: pop rdi
141728205: pop rsi
141728206: pop r12
141728208: pop r13
14172820a: pop r14
14172820c: pop r15
14172820e: pop rbp
14172820f: ret 
141728210: mov byte ptr [rbx + 0xb4], sil
141728217: movaps xmm6, xmmword ptr [rsp + 0x80]
14172821f: movaps xmm7, xmmword ptr [rsp + 0x70]
141728224: add rsp, 0x98
14172822b: pop rbx
14172822c: pop rdi
14172822d: pop rsi
14172822e: pop r12
141728230: pop r13
141728232: pop r14
141728234: pop r15
141728236: pop rbp
141728237: ret 
141728238: nop dword ptr [rax + rax]
141728240: mov qword ptr [rsp + 0x10], rdx
141728245: push rbp
141728246: push r15
141728248: push r14
14172824a: push r13
14172824c: push r12
14172824e: push rsi
14172824f: push rdi
141728250: push rbx
141728251: sub rsp, 0x98
141728258: lea rbp, [rdx + 0x80]
14172825f: movaps xmmword ptr [rsp + 0x70], xmm7
141728264: movaps xmmword ptr [rsp + 0x80], xmm6
14172826c: and rdx, 0xffffffffffffff80
141728270: mov rbx, rdx
141728273: movzx esi, byte ptr [rbx + 0xb4]
14172827a: test sil, 1
14172827e: je 0x1417282b4
141728280: lea rcx, [rbx + 0x15a0]
141728287: call 0x140149f50
14172828c: mov byte ptr [rbx + 0xa7], sil
141728293: movaps xmm6, xmmword ptr [rsp + 0x80]
14172829b: movaps xmm7, xmmword ptr [rsp + 0x70]
1417282a0: add rsp, 0x98
1417282a7: pop rbx
1417282a8: pop rdi
1417282a9: pop rsi
1417282aa: pop r12
1417282ac: pop r13
1417282ae: pop r14
1417282b0: pop r15
1417282b2: pop rbp
1417282b3: ret 
1417282b4: mov byte ptr [rbx + 0xa7], sil
1417282bb: movaps xmm6, xmmword ptr [rsp + 0x80]
1417282c3: movaps xmm7, xmmword ptr [rsp + 0x70]
1417282c8: add rsp, 0x98
1417282cf: pop rbx
1417282d0: pop rdi
1417282d1: pop rsi
1417282d2: pop r12
1417282d4: pop r13
1417282d6: pop r14
1417282d8: pop r15
1417282da: pop rbp
1417282db: ret 
1417282dc: nop dword ptr [rax]
1417282e0: mov qword ptr [rsp + 0x10], rdx
1417282e5: push rbp
1417282e6: push r15
1417282e8: push r14
1417282ea: push r13
1417282ec: push r12
1417282ee: push rsi
1417282ef: push rdi
1417282f0: push rbx
1417282f1: sub rsp, 0x98
1417282f8: lea rbp, [rdx + 0x80]
1417282ff: movaps xmmword ptr [rsp + 0x70], xmm7
141728304: movaps xmmword ptr [rsp + 0x80], xmm6
14172830c: and rdx, 0xffffffffffffff80
141728310: mov rbx, rdx
141728313: movzx esi, byte ptr [rbx + 0xa7]
14172831a: lea rdx, [rbx + 0x160]
141728321: lea rcx, [rbx + 0x140]
141728328: mov r8d, 0x20
14172832e: mov r9d, 0x10
141728334: call 0x140052a30
141728339: mov byte ptr [rbx + 0xb3], sil
141728340: movaps xmm6, xmmword ptr [rsp + 0x80]
141728348: movaps xmm7, xmmword ptr [rsp + 0x70]
14172834d: add rsp, 0x98
141728354: pop rbx
141728355: pop rdi
141728356: pop rsi
141728357: pop r12
141728359: pop r13
14172835b: pop r14
14172835d: pop r15
14172835f: pop rbp
141728360: ret 
141728361: nop word ptr cs:[rax + rax]
141728370: mov qword ptr [rsp + 0x10], rdx
141728375: push rbp
141728376: push r15
141728378: push r14
14172837a: push r13
14172837c: push r12
14172837e: push rsi
14172837f: push rdi
141728380: push rbx
141728381: sub rsp, 0x98
141728388: lea rbp, [rdx + 0x80]
14172838f: movaps xmmword ptr [rsp + 0x70], xmm7
141728394: movaps xmmword ptr [rsp + 0x80], xmm6
14172839c: and rdx, 0xffffffffffffff80
1417283a0: mov rbx, rdx
1417283a3: movzx eax, byte ptr [rbx + 0xb3]
1417283aa: mov byte ptr [rbx + 0x80], al
1417283b0: test al, 1
1417283b2: je 0x1417283e1
1417283b4: lea rcx, [rbx + 0x170]
1417283bb: call 0x1405d8fc0
1417283c0: movaps xmm6, xmmword ptr [rsp + 0x80]
1417283c8: movaps xmm7, xmmword ptr [rsp + 0x70]
1417283cd: add rsp, 0x98
1417283d4: pop rbx
1417283d5: pop rdi
1417283d6: pop rsi
1417283d7: pop r12
1417283d9: pop r13
1417283db: pop r14
1417283dd: pop r15
1417283df: pop rbp
1417283e0: ret 
1417283e1: movaps xmm6, xmmword ptr [rsp + 0x80]
1417283e9: movaps xmm7, xmmword ptr [rsp + 0x70]
1417283ee: add rsp, 0x98
1417283f5: pop rbx
1417283f6: pop rdi
1417283f7: pop rsi
1417283f8: pop r12
1417283fa: pop r13
1417283fc: pop r14
1417283fe: pop r15
141728400: pop rbp
141728401: ret 
141728402: nop word ptr cs:[rax + rax]
141728410: mov qword ptr [rsp + 0x10], rdx
141728415: push rbp
141728416: push r15
141728418: push r14
14172841a: push r13
14172841c: push r12
14172841e: push rsi
14172841f: push rdi
141728420: push rbx
141728421: sub rsp, 0x98
141728428: lea rbp, [rdx + 0x80]
14172842f: movaps xmmword ptr [rsp + 0x70], xmm7
141728434: movaps xmmword ptr [rsp + 0x80], xmm6
14172843c: and rdx, 0xffffffffffffff80
141728440: mov rbx, rdx
141728443: mov rax, qword ptr [rbx + 0x200]
14172844a: lock dec qword ptr [rax]
14172844e: jne 0x14172845c
141728450: lea rcx, [rbx + 0x200]
141728457: call 0x140d53bc0
14172845c: movaps xmm6, xmmword ptr [rsp + 0x80]
141728464: movaps xmm7, xmmword ptr [rsp + 0x70]
141728469: add rsp, 0x98
141728470: pop rbx
141728471: pop rdi
141728472: pop rsi
141728473: pop r12
141728475: pop r13
141728477: pop r14
141728479: pop r15
14172847b: pop rbp
14172847c: ret 
14172847d: nop dword ptr [rax]
141728480: mov qword ptr [rsp + 0x10], rdx
141728485: push rbp
141728486: push r15
141728488: push r14
14172848a: push r13
14172848c: push r12
14172848e: push rsi
14172848f: push rdi
141728490: push rbx
141728491: sub rsp, 0x98
141728498: lea rbp, [rdx + 0x80]
14172849f: movaps xmmword ptr [rsp + 0x70], xmm7
1417284a4: movaps xmmword ptr [rsp + 0x80], xmm6
1417284ac: and rdx, 0xffffffffffffff80
1417284b0: mov rbx, rdx
1417284b3: cmp qword ptr [rbx + 0x80], 0
1417284bb: je 0x1417284d6
1417284bd: mov r8d, 1
1417284c3: mov rcx, qword ptr [rbx + 0x98]
1417284ca: mov rdx, qword ptr [rbx + 0x80]
1417284d1: call 0x140613c20
1417284d6: mov byte ptr [rbx + 0xb5], 0
1417284dd: movaps xmm6, xmmword ptr [rsp + 0x80]
1417284e5: movaps xmm7, xmmword ptr [rsp + 0x70]
1417284ea: add rsp, 0x98
1417284f1: pop rbx
1417284f2: pop rdi
1417284f3: pop rsi
1417284f4: pop r12
1417284f6: pop r13
1417284f8: pop r14
1417284fa: pop r15
1417284fc: pop rbp
1417284fd: ret 
1417284fe: nop 
141728500: mov qword ptr [rsp + 0x10], rdx
141728505: push rbp
141728506: push r15
141728508: push r14
14172850a: push r13
14172850c: push r12
14172850e: push rsi
14172850f: push rdi
141728510: push rbx
141728511: sub rsp, 0x98
141728518: lea rbp, [rdx + 0x80]
14172851f: movaps xmmword ptr [rsp + 0x70], xmm7
141728524: movaps xmmword ptr [rsp + 0x80], xmm6
14172852c: and rdx, 0xffffffffffffff80
141728530: mov rbx, rdx
141728533: cmp byte ptr [rbx + 0xb5], 0
14172853a: je 0x14172857b
14172853c: mov rdx, qword ptr [rbx + 0x480]
141728543: test rdx, rdx
141728546: je 0x14172855a
141728548: mov rcx, qword ptr [rbx + 0x488]
14172854f: mov r8d, 1
141728555: call 0x140613c20
14172855a: movaps xmm6, xmmword ptr [rsp + 0x80]
141728562: movaps xmm7, xmmword ptr [rsp + 0x70]
141728567: add rsp, 0x98
14172856e: pop rbx
14172856f: pop rdi
141728570: pop rsi
141728571: pop r12
141728573: pop r13
141728575: pop r14
141728577: pop r15
141728579: pop rbp
14172857a: ret 
14172857b: movaps xmm6, xmmword ptr [rsp + 0x80]
141728583: movaps xmm7, xmmword ptr [rsp + 0x70]
141728588: add rsp, 0x98
14172858f: pop rbx
141728590: pop rdi
141728591: pop rsi
141728592: pop r12
141728594: pop r13
141728596: pop r14
141728598: pop r15
14172859a: pop rbp
14172859b: ret 
14172859c: nop dword ptr [rax]
1417285a0: mov qword ptr [rsp + 0x10], rdx
1417285a5: push rbp
1417285a6: push r15
1417285a8: push r14
1417285aa: push r13
1417285ac: push r12
1417285ae: push rsi
1417285af: push rdi
1417285b0: push rbx
1417285b1: sub rsp, 0x98
1417285b8: lea rbp, [rdx + 0x80]
1417285bf: movaps xmmword ptr [rsp + 0x70], xmm7
1417285c4: movaps xmmword ptr [rsp + 0x80], xmm6
1417285cc: and rdx, 0xffffffffffffff80
1417285d0: mov rbx, rdx
1417285d3: movzx esi, byte ptr [rbx + 0xa5]
1417285da: test byte ptr [rbx + 0xa4], 1
1417285e1: je 0x14172861b
1417285e3: lea rcx, [rbx + 0x120]
1417285ea: call 0x1405d8fc0
1417285ef: and sil, 1
1417285f3: mov byte ptr [rbx + 0xa6], sil
1417285fa: movaps xmm6, xmmword ptr [rsp + 0x80]
141728602: movaps xmm7, xmmword ptr [rsp + 0x70]
141728607: add rsp, 0x98
14172860e: pop rbx
14172860f: pop rdi
141728610: pop rsi
141728611: pop r12
141728613: pop r13
141728615: pop r14
141728617: pop r15
141728619: pop rbp
14172861a: ret 
14172861b: and sil, 1
14172861f: mov byte ptr [rbx + 0xa6], sil
141728626: movaps xmm6, xmmword ptr [rsp + 0x80]
14172862e: movaps xmm7, xmmword ptr [rsp + 0x70]
141728633: add rsp, 0x98
14172863a: pop rbx
14172863b: pop rdi
14172863c: pop rsi
14172863d: pop r12
14172863f: pop r13
141728641: pop r14
141728643: pop r15
141728645: pop rbp
141728646: ret 
141728647: nop word ptr [rax + rax]
141728650: mov qword ptr [rsp + 0x10], rdx
141728655: push rbp
141728656: push r15
141728658: push r14
14172865a: push r13
14172865c: push r12
14172865e: push rsi
14172865f: push rdi
141728660: push rbx
141728661: sub rsp, 0x98
141728668: lea rbp, [rdx + 0x80]
14172866f: movaps xmmword ptr [rsp + 0x70], xmm7
141728674: movaps xmmword ptr [rsp + 0x80], xmm6
14172867c: and rdx, 0xffffffffffffff80
141728680: mov rbx, rdx
141728683: cmp byte ptr [rbx + 0xc8], 0
14172868a: je 0x141728698
14172868c: lea rcx, [rbx + 0x100]
141728693: call 0x1405d8fc0
141728698: movaps xmm6, xmmword ptr [rsp + 0x80]
1417286a0: movaps xmm7, xmmword ptr [rsp + 0x70]
1417286a5: add rsp, 0x98
1417286ac: pop rbx
1417286ad: pop rdi
1417286ae: pop rsi
1417286af: pop r12
1417286b1: pop r13
1417286b3: pop r14
1417286b5: pop r15
1417286b7: pop rbp
1417286b8: ret 
1417286b9: nop dword ptr [rax]
1417286c0: mov qword ptr [rsp + 0x10], rdx
1417286c5: push rbp
1417286c6: push r15
1417286c8: push r14
1417286ca: push r13
1417286cc: push r12
1417286ce: push rsi
1417286cf: push rdi
1417286d0: push rbx
1417286d1: sub rsp, 0x98
1417286d8: lea rbp, [rdx + 0x80]
1417286df: movaps xmmword ptr [rsp + 0x70], xmm7
1417286e4: movaps xmmword ptr [rsp + 0x80], xmm6
1417286ec: and rdx, 0xffffffffffffff80
1417286f0: mov rbx, rdx
1417286f3: mov rax, qword ptr [rbx + 0x80]
1417286fa: inc qword ptr [rax]
1417286fd: mov byte ptr [rbx + 0x7f], 1
141728701: movaps xmm6, xmmword ptr [rsp + 0x80]
141728709: movaps xmm7, xmmword ptr [rsp + 0x70]
14172870e: add rsp, 0x98
141728715: pop rbx
141728716: pop rdi
141728717: pop rsi
141728718: pop r12
14172871a: pop r13
14172871c: pop r14
14172871e: pop r15
141728720: pop rbp
141728721: ret 
141728722: nop word ptr cs:[rax + rax]
141728730: mov qword ptr [rsp + 0x10], rdx
141728735: push rbp
141728736: push r15
141728738: push r14
14172873a: push r13
14172873c: push r12
14172873e: push rsi
14172873f: push rdi
141728740: push rbx
141728741: sub rsp, 0x98
141728748: lea rbp, [rdx + 0x80]
14172874f: movaps xmmword ptr [rsp + 0x70], xmm7
141728754: movaps xmmword ptr [rsp + 0x80], xmm6
14172875c: and rdx, 0xffffffffffffff80
141728760: mov rbx, rdx
141728763: cmp qword ptr [rbx + 0x1f0], 0
14172876b: je 0x14172879c
14172876d: lea rax, [rbx + 0x1f8]
141728774: mov qword ptr [rbx + 0x470], rax
14172877b: lea rax, [rbx + 0x470]
141728782: mov qword ptr [rbx + 0x478], rax
141728789: lea rcx, [rip + 0x15a1c20]
141728790: lea rdx, [rbx + 0x478]
141728797: call 0x14132f2d0
14172879c: mov byte ptr [rbx + 0x7f], 0
1417287a0: movaps xmm6, xmmword ptr [rsp + 0x80]
1417287a8: movaps xmm7, xmmword ptr [rsp + 0x70]
1417287ad: add rsp, 0x98
1417287b4: pop rbx
1417287b5: pop rdi
1417287b6: pop rsi
1417287b7: pop r12
1417287b9: pop r13
1417287bb: pop r14
1417287bd: pop r15
1417287bf: pop rbp
1417287c0: ret 
1417287c1: nop word ptr cs:[rax + rax]
1417287d0: mov qword ptr [rsp + 0x10], rdx
1417287d5: push rbp
1417287d6: push r15
1417287d8: push r14
1417287da: push r13
1417287dc: push r12
1417287de: push rsi
1417287df: push rdi
1417287e0: push rbx
1417287e1: sub rsp, 0x98
1417287e8: lea rbp, [rdx + 0x80]
1417287ef: movaps xmmword ptr [rsp + 0x70], xmm7
1417287f4: movaps xmmword ptr [rsp + 0x80], xmm6
1417287fc: and rdx, 0xffffffffffffff80
141728800: mov rbx, rdx
141728803: mov rax, qword ptr [rbx + 0x80]
14172880a: inc qword ptr [rax]
14172880d: mov byte ptr [rbx + 0x7f], 0
141728811: movaps xmm6, xmmword ptr [rsp + 0x80]
141728819: movaps xmm7, xmmword ptr [rsp + 0x70]
14172881e: add rsp, 0x98
141728825: pop rbx
141728826: pop rdi
141728827: pop rsi
141728828: pop r12
14172882a: pop r13
14172882c: pop r14
14172882e: pop r15
141728830: pop rbp
141728831: ret 
141728832: nop word ptr cs:[rax + rax]
141728840: mov qword ptr [rsp + 0x10], rdx
141728845: push rbp
141728846: push r15
141728848: push r14
14172884a: push r13
14172884c: push r12
14172884e: push rsi
14172884f: push rdi
141728850: push rbx
141728851: sub rsp, 0x98
141728858: lea rbp, [rdx + 0x80]
14172885f: movaps xmmword ptr [rsp + 0x70], xmm7
141728864: movaps xmmword ptr [rsp + 0x80], xmm6
14172886c: and rdx, 0xffffffffffffff80
141728870: mov rbx, rdx
141728873: cmp byte ptr [rbx + 0x7f], 0
141728877: jne 0x14172889a
141728879: movaps xmm6, xmmword ptr [rsp + 0x80]
141728881: movaps xmm7, xmmword ptr [rsp + 0x70]
141728886: add rsp, 0x98
14172888d: pop rbx
14172888e: pop rdi
14172888f: pop rsi
141728890: pop r12
141728892: pop r13
141728894: pop r14
141728896: pop r15
141728898: pop rbp
141728899: ret 
14172889a: mov rcx, qword ptr [rbx + 0xb8]
1417288a1: call 0x14146cb40
1417288a6: lea rcx, [rbx + 0x190]
1417288ad: call 0x1405d8fc0
1417288b2: jmp 0x141728879
1417288b4: nop word ptr cs:[rax + rax]
1417288c0: mov qword ptr [rsp + 0x10], rdx
1417288c5: push rbp
1417288c6: push r15
1417288c8: push r14
1417288ca: push r13
1417288cc: push r12
1417288ce: push rsi
1417288cf: push rdi
1417288d0: push rbx
1417288d1: sub rsp, 0x98
1417288d8: lea rbp, [rdx + 0x80]
1417288df: movaps xmmword ptr [rsp + 0x70], xmm7
1417288e4: movaps xmmword ptr [rsp + 0x80], xmm6
1417288ec: and rdx, 0xffffffffffffff80
1417288f0: mov rbx, rdx
1417288f3: mov rax, qword ptr [rbx + 0x80]
1417288fa: inc qword ptr [rax]
1417288fd: movaps xmm6, xmmword ptr [rsp + 0x80]
141728905: movaps xmm7, xmmword ptr [rsp + 0x70]
14172890a: add rsp, 0x98
141728911: pop rbx
141728912: pop rdi
141728913: pop rsi
141728914: pop r12
141728916: pop r13
141728918: pop r14
14172891a: pop r15
14172891c: pop rbp
14172891d: ret 
14172891e: nop 
141728920: mov qword ptr [rsp + 0x10], rdx
141728925: push rbp
141728926: push r15
141728928: push r14
14172892a: push r13
14172892c: push r12
14172892e: push rsi
14172892f: push rdi
141728930: push rbx
141728931: sub rsp, 0x98
141728938: lea rbp, [rdx + 0x80]
14172893f: movaps xmmword ptr [rsp + 0x70], xmm7
141728944: movaps xmmword ptr [rsp + 0x80], xmm6
14172894c: and rdx, 0xffffffffffffff80
141728950: mov rbx, rdx
141728953: cmp byte ptr [rbx + 0x80], 0
14172895a: je 0x14172898b
14172895c: lea rax, [rbx + 0x488]
141728963: mov qword ptr [rbx + 0x15a0], rax
14172896a: lea rax, [rbx + 0x15a0]
141728971: mov qword ptr [rbx + 0x1340], rax
141728978: lea rcx, [rip + 0x15a1a31]
14172897f: lea rdx, [rbx + 0x1340]
141728986: call 0x14132f2d0
14172898b: movaps xmm6, xmmword ptr [rsp + 0x80]
141728993: movaps xmm7, xmmword ptr [rsp + 0x70]
141728998: add rsp, 0x98
14172899f: pop rbx
1417289a0: pop rdi
1417289a1: pop rsi
1417289a2: pop r12
1417289a4: pop r13
1417289a6: pop r14
1417289a8: pop r15
1417289aa: pop rbp
1417289ab: ret 
1417289ac: nop dword ptr [rax]
1417289b0: mov qword ptr [rsp + 0x10], rdx
1417289b5: push rbp
1417289b6: push r15
1417289b8: push r14
1417289ba: push r13
1417289bc: push r12
1417289be: push rsi
1417289bf: push rdi
1417289c0: push rbx
1417289c1: sub rsp, 0x98
1417289c8: lea rbp, [rdx + 0x80]
1417289cf: movaps xmmword ptr [rsp + 0x70], xmm7
1417289d4: movaps xmmword ptr [rsp + 0x80], xmm6
1417289dc: and rdx, 0xffffffffffffff80
1417289e0: mov rbx, rdx
1417289e3: lea rcx, [rbx + 0x790]
1417289ea: call 0x140890ff0
1417289ef: lea rcx, [rbx + 0x7d8]
1417289f6: call 0x14087cfb0
1417289fb: lea rcx, [rbx + 0x700]
141728a02: call 0x140878420
141728a07: movaps xmm6, xmmword ptr [rsp + 0x80]
141728a0f: movaps xmm7, xmmword ptr [rsp + 0x70]
141728a14: add rsp, 0x98
141728a1b: pop rbx
141728a1c: pop rdi
141728a1d: pop rsi
141728a1e: pop r12
141728a20: pop r13
141728a22: pop r14
141728a24: pop r15
141728a26: pop rbp
141728a27: ret 
141728a28: nop dword ptr [rax + rax]
141728a30: mov qword ptr [rsp + 0x10], rdx
141728a35: push rbp
141728a36: push r15
141728a38: push r14
141728a3a: push r13
141728a3c: push r12
141728a3e: push rsi
141728a3f: push rdi
141728a40: push rbx
141728a41: sub rsp, 0x98
141728a48: lea rbp, [rdx + 0x80]
141728a4f: movaps xmmword ptr [rsp + 0x70], xmm7
141728a54: movaps xmmword ptr [rsp + 0x80], xmm6
141728a5c: and rdx, 0xffffffffffffff80
141728a60: mov rbx, rdx
141728a63: lea rcx, [rbx + 0x968]
141728a6a: lea rdx, [rbx + 0x988]
141728a71: mov r8d, 0x20
141728a77: mov r9d, 0x10
141728a7d: call 0x140052a30
141728a82: movaps xmm6, xmmword ptr [rsp + 0x80]
141728a8a: movaps xmm7, xmmword ptr [rsp + 0x70]
141728a8f: add rsp, 0x98
141728a96: pop rbx
141728a97: pop rdi
141728a98: pop rsi
141728a99: pop r12
141728a9b: pop r13
141728a9d: pop r14
141728a9f: pop r15
141728aa1: pop rbp
141728aa2: ret 
141728aa3: nop word ptr cs:[rax + rax]
141728ab0: mov qword ptr [rsp + 0x10], rdx
141728ab5: push rbp
141728ab6: push r15
141728ab8: push r14
141728aba: push r13
141728abc: push r12
141728abe: push rsi
141728abf: push rdi
141728ac0: push rbx
141728ac1: sub rsp, 0x98
141728ac8: lea rbp, [rdx + 0x80]
141728acf: movaps xmmword ptr [rsp + 0x70], xmm7
141728ad4: movaps xmmword ptr [rsp + 0x80], xmm6
141728adc: and rdx, 0xffffffffffffff80
141728ae0: mov rbx, rdx
141728ae3: mov rax, qword ptr [rbx + 0x80]
141728aea: inc qword ptr [rax]
141728aed: movaps xmm6, xmmword ptr [rsp + 0x80]
141728af5: movaps xmm7, xmmword ptr [rsp + 0x70]
141728afa: add rsp, 0x98
141728b01: pop rbx
141728b02: pop rdi
141728b03: pop rsi
141728b04: pop r12
141728b06: pop r13
141728b08: pop r14
141728b0a: pop r15
141728b0c: pop rbp
141728b0d: ret 
141728b0e: nop 
141728b10: mov qword ptr [rsp + 0x10], rdx
141728b15: push rbp
141728b16: push r15
141728b18: push r14
141728b1a: push r13
141728b1c: push r12
141728b1e: push rsi
141728b1f: push rdi
141728b20: push rbx
141728b21: sub rsp, 0x98
141728b28: lea rbp, [rdx + 0x80]
141728b2f: movaps xmmword ptr [rsp + 0x70], xmm7
141728b34: movaps xmmword ptr [rsp + 0x80], xmm6
141728b3c: and rdx, 0xffffffffffffff80
141728b40: mov rbx, rdx
141728b43: cmp byte ptr [rbx + 0xc9], 0
141728b4a: je 0x141728b58
141728b4c: lea rcx, [rbx + 0x710]
141728b53: call 0x14088ed10
141728b58: lea rcx, [rbx + 0x790]
141728b5f: call 0x140890ff0
141728b64: lea rcx, [rbx + 0x7d8]
141728b6b: call 0x14087cfb0
141728b70: lea rcx, [rbx + 0x700]
141728b77: call 0x140878420
141728b7c: lea rcx, [rbx + 0x968]
141728b83: lea rdx, [rbx + 0x988]
141728b8a: mov r8d, 0x20
141728b90: mov r9d, 0x10
141728b96: call 0x140052a30
141728b9b: movaps xmm6, xmmword ptr [rsp + 0x80]
141728ba3: movaps xmm7, xmmword ptr [rsp + 0x70]
141728ba8: add rsp, 0x98
141728baf: pop rbx
141728bb0: pop rdi
141728bb1: pop rsi
141728bb2: pop r12
141728bb4: pop r13
141728bb6: pop r14
141728bb8: pop r15
141728bba: pop rbp
141728bbb: ret 
141728bbc: nop dword ptr [rax]
141728bc0: mov qword ptr [rsp + 0x10], rdx
141728bc5: push rbp
141728bc6: push r15
141728bc8: push r14
141728bca: push r13
141728bcc: push r12
141728bce: push rsi
141728bcf: push rdi
141728bd0: push rbx
141728bd1: sub rsp, 0x98
141728bd8: lea rbp, [rdx + 0x80]
141728bdf: movaps xmmword ptr [rsp + 0x70], xmm7
141728be4: movaps xmmword ptr [rsp + 0x80], xmm6
141728bec: and rdx, 0xffffffffffffff80
141728bf0: mov rbx, rdx
141728bf3: mov rax, qword ptr [rbx + 0x80]
141728bfa: inc qword ptr [rax]
141728bfd: mov byte ptr [rbx + 0x8f], 1
141728c04: movaps xmm6, xmmword ptr [rsp + 0x80]
141728c0c: movaps xmm7, xmmword ptr [rsp + 0x70]
141728c11: add rsp, 0x98
141728c18: pop rbx
141728c19: pop rdi
141728c1a: pop rsi
141728c1b: pop r12
141728c1d: pop r13
141728c1f: pop r14
141728c21: pop r15
141728c23: pop rbp
141728c24: ret 
141728c25: nop word ptr cs:[rax + rax]
141728c30: mov qword ptr [rsp + 0x10], rdx
141728c35: push rbp
141728c36: push r15
141728c38: push r14
141728c3a: push r13
141728c3c: push r12
141728c3e: push rsi
141728c3f: push rdi
141728c40: push rbx
141728c41: sub rsp, 0x98
141728c48: lea rbp, [rdx + 0x80]
141728c4f: movaps xmmword ptr [rsp + 0x70], xmm7
141728c54: movaps xmmword ptr [rsp + 0x80], xmm6
141728c5c: and rdx, 0xffffffffffffff80
141728c60: mov rbx, rdx
141728c63: cmp qword ptr [rbx + 0xd8], 0
141728c6b: je 0x141728c9c
141728c6d: lea rax, [rbx + 0xe0]
141728c74: mov qword ptr [rbx + 0x120], rax
141728c7b: lea rax, [rbx + 0x120]
141728c82: mov qword ptr [rbx + 0x170], rax
141728c89: lea rcx, [rip + 0x15a1720]
141728c90: lea rdx, [rbx + 0x170]
141728c97: call 0x14132f2d0
141728c9c: movzx eax, byte ptr [rbx + 0xd0]
141728ca3: mov byte ptr [rbx + 0x8f], al
141728ca9: movaps xmm6, xmmword ptr [rsp + 0x80]
141728cb1: movaps xmm7, xmmword ptr [rsp + 0x70]
141728cb6: add rsp, 0x98
141728cbd: pop rbx
141728cbe: pop rdi
141728cbf: pop rsi
141728cc0: pop r12
141728cc2: pop r13
141728cc4: pop r14
141728cc6: pop r15
141728cc8: pop rbp
141728cc9: ret 
141728cca: nop word ptr [rax + rax]
141728cd0: mov qword ptr [rsp + 0x10], rdx
141728cd5: push rbp
141728cd6: push r15
141728cd8: push r14
141728cda: push r13
141728cdc: push r12
141728cde: push rsi
141728cdf: push rdi
141728ce0: push rbx
141728ce1: sub rsp, 0x98
141728ce8: lea rbp, [rdx + 0x80]
141728cef: movaps xmmword ptr [rsp + 0x70], xmm7
141728cf4: movaps xmmword ptr [rsp + 0x80], xmm6
141728cfc: and rdx, 0xffffffffffffff80
141728d00: mov rbx, rdx
141728d03: mov rax, qword ptr [rbx + 0x80]
141728d0a: inc qword ptr [rax]
141728d0d: mov byte ptr [rbx + 0x7d], 1
141728d11: movaps xmm6, xmmword ptr [rsp + 0x80]
141728d19: movaps xmm7, xmmword ptr [rsp + 0x70]
141728d1e: add rsp, 0x98
141728d25: pop rbx
141728d26: pop rdi
141728d27: pop rsi
141728d28: pop r12
141728d2a: pop r13
141728d2c: pop r14
141728d2e: pop r15
141728d30: pop rbp
141728d31: ret 
141728d32: nop word ptr cs:[rax + rax]
141728d40: mov qword ptr [rsp + 0x10], rdx
141728d45: push rbp
141728d46: push r15
141728d48: push r14
141728d4a: push r13
141728d4c: push r12
141728d4e: push rsi
141728d4f: push rdi
141728d50: push rbx
141728d51: sub rsp, 0x98
141728d58: lea rbp, [rdx + 0x80]
141728d5f: movaps xmmword ptr [rsp + 0x70], xmm7
141728d64: movaps xmmword ptr [rsp + 0x80], xmm6
141728d6c: and rdx, 0xffffffffffffff80
141728d70: mov rbx, rdx
141728d73: movzx esi, byte ptr [rbx + 0x94]
141728d7a: cmp qword ptr [rbx + 0x190], 0
141728d82: je 0x141728db3
141728d84: lea rax, [rbx + 0x198]
141728d8b: mov qword ptr [rbx + 0x120], rax
141728d92: lea rax, [rbx + 0x120]
141728d99: mov qword ptr [rbx + 0x170], rax
141728da0: lea rcx, [rip + 0x15a1609]
141728da7: lea rdx, [rbx + 0x170]
141728dae: call 0x14132f2d0
141728db3: and sil, 1
141728db7: mov byte ptr [rbx + 0x7d], sil
141728dbb: movaps xmm6, xmmword ptr [rsp + 0x80]
141728dc3: movaps xmm7, xmmword ptr [rsp + 0x70]
141728dc8: add rsp, 0x98
141728dcf: pop rbx
141728dd0: pop rdi
141728dd1: pop rsi
141728dd2: pop r12
141728dd4: pop r13
141728dd6: pop r14
141728dd8: pop r15
141728dda: pop rbp
141728ddb: ret 
141728ddc: nop dword ptr [rax]
141728de0: mov qword ptr [rsp + 0x10], rdx
141728de5: push rbp
141728de6: push r15
141728de8: push r14
141728dea: push r13
141728dec: push r12
141728dee: push rsi
141728def: push rdi
141728df0: push rbx
141728df1: sub rsp, 0x98
141728df8: lea rbp, [rdx + 0x80]
141728dff: movaps xmmword ptr [rsp + 0x70], xmm7
141728e04: movaps xmmword ptr [rsp + 0x80], xmm6
141728e0c: and rdx, 0xffffffffffffff80
141728e10: mov rbx, rdx
141728e13: lea rcx, [rbx + 0x1340]
141728e1a: call 0x140876ca0
141728e1f: mov byte ptr [rbx + 0x94], 0
141728e26: movaps xmm6, xmmword ptr [rsp + 0x80]
141728e2e: movaps xmm7, xmmword ptr [rsp + 0x70]
141728e33: add rsp, 0x98
141728e3a: pop rbx
141728e3b: pop rdi
141728e3c: pop rsi
141728e3d: pop r12
141728e3f: pop r13
141728e41: pop r14
141728e43: pop r15
141728e45: pop rbp
141728e46: ret 
141728e47: nop word ptr [rax + rax]
141728e50: mov qword ptr [rsp + 0x10], rdx
141728e55: push rbp
141728e56: push r15
141728e58: push r14
141728e5a: push r13
141728e5c: push r12
141728e5e: push rsi
141728e5f: push rdi
141728e60: push rbx
141728e61: sub rsp, 0x98
141728e68: lea rbp, [rdx + 0x80]
141728e6f: movaps xmmword ptr [rsp + 0x70], xmm7
141728e74: movaps xmmword ptr [rsp + 0x80], xmm6
141728e7c: and rdx, 0xffffffffffffff80
141728e80: mov rbx, rdx
141728e83: mov rax, qword ptr [rbx + 0x1350]
141728e8a: test rax, rax
141728e8d: je 0x141728ea9
141728e8f: mov rcx, qword ptr [rbx + 0x1340]
141728e96: shl rax, 3
141728e9a: lea rdx, [rax + rax*4]
141728e9e: mov r8d, 8
141728ea4: call 0x140613c20
141728ea9: mov byte ptr [rbx + 0x94], 0
141728eb0: movaps xmm6, xmmword ptr [rsp + 0x80]
141728eb8: movaps xmm7, xmmword ptr [rsp + 0x70]
141728ebd: add rsp, 0x98
141728ec4: pop rbx
141728ec5: pop rdi
141728ec6: pop rsi
141728ec7: pop r12
141728ec9: pop r13
141728ecb: pop r14
141728ecd: pop r15
141728ecf: pop rbp
141728ed0: ret 
141728ed1: nop word ptr cs:[rax + rax]
141728ee0: mov qword ptr [rsp + 0x10], rdx
141728ee5: push rbp
141728ee6: push r15
141728ee8: push r14
141728eea: push r13
141728eec: push r12
141728eee: push rsi
141728eef: push rdi
141728ef0: push rbx
141728ef1: sub rsp, 0x98
141728ef8: lea rbp, [rdx + 0x80]
141728eff: movaps xmmword ptr [rsp + 0x70], xmm7
141728f04: movaps xmmword ptr [rsp + 0x80], xmm6
141728f0c: and rdx, 0xffffffffffffff80
141728f10: mov rbx, rdx
141728f13: mov rdi, qword ptr [rbx + 0x98]
141728f1a: mov r14, qword ptr [rbx + 0x80]
141728f21: cmp rdi, r14
141728f24: mov rsi, qword ptr [rbx + 0x1c8]
141728f2b: je 0x141728f42
141728f2d: lea rax, [r14 + r14*4]
141728f31: lea rcx, [rsi + rax*8]
141728f35: call 0x14088bae0
141728f3a: inc r14
141728f3d: cmp r14, rdi
141728f40: jne 0x141728f2d
141728f42: movaps xmm6, xmmword ptr [rsp + 0x80]
141728f4a: movaps xmm7, xmmword ptr [rsp + 0x70]
141728f4f: add rsp, 0x98
141728f56: pop rbx
141728f57: pop rdi
141728f58: pop rsi
141728f59: pop r12
141728f5b: pop r13
141728f5d: pop r14
141728f5f: pop r15
141728f61: pop rbp
141728f62: ret 
141728f63: nop word ptr cs:[rax + rax]
141728f70: mov qword ptr [rsp + 0x10], rdx
141728f75: push rbp
141728f76: push r15
141728f78: push r14
141728f7a: push r13
141728f7c: push r12
141728f7e: push rsi
141728f7f: push rdi
141728f80: push rbx
141728f81: sub rsp, 0x98
141728f88: lea rbp, [rdx + 0x80]
141728f8f: movaps xmmword ptr [rsp + 0x70], xmm7
141728f94: movaps xmmword ptr [rsp + 0x80], xmm6
141728f9c: and rdx, 0xffffffffffffff80
141728fa0: mov rbx, rdx
141728fa3: cmp qword ptr [rbx + 0x80], 0
141728fab: je 0x141728fc6
141728fad: mov r8d, 1
141728fb3: mov rcx, qword ptr [rbx + 0x98]
141728fba: mov rdx, qword ptr [rbx + 0x80]
141728fc1: call 0x140613c20
141728fc6: mov byte ptr [rbx + 0x94], 0
141728fcd: movaps xmm6, xmmword ptr [rsp + 0x80]
141728fd5: movaps xmm7, xmmword ptr [rsp + 0x70]
141728fda: add rsp, 0x98
141728fe1: pop rbx
141728fe2: pop rdi
141728fe3: pop rsi
141728fe4: pop r12
141728fe6: pop r13
141728fe8: pop r14
141728fea: pop r15
141728fec: pop rbp
141728fed: ret 
141728fee: nop 
141728ff0: mov qword ptr [rsp + 0x10], rdx
141728ff5: push rbp
141728ff6: push r15
141728ff8: push r14
141728ffa: push r13
141728ffc: push r12
141728ffe: push rsi
141728fff: push rdi
141729000: push rbx
141729001: sub rsp, 0x98
141729008: lea rbp, [rdx + 0x80]
14172900f: movaps xmmword ptr [rsp + 0x70], xmm7
141729014: movaps xmmword ptr [rsp + 0x80], xmm6
14172901c: and rdx, 0xffffffffffffff80
141729020: mov rbx, rdx
141729023: mov rax, qword ptr [rbx + 0x80]
14172902a: inc qword ptr [rax]
14172902d: mov byte ptr [rbx + 0x7d], 0
141729031: movaps xmm6, xmmword ptr [rsp + 0x80]
141729039: movaps xmm7, xmmword ptr [rsp + 0x70]
14172903e: add rsp, 0x98
141729045: pop rbx
141729046: pop rdi
141729047: pop rsi
141729048: pop r12
14172904a: pop r13
14172904c: pop r14
14172904e: pop r15
141729050: pop rbp
141729051: ret 
141729052: nop word ptr cs:[rax + rax]
141729060: mov qword ptr [rsp + 0x10], rdx
141729065: push rbp
141729066: push r15
141729068: push r14
14172906a: push r13
14172906c: push r12
14172906e: push rsi
14172906f: push rdi
141729070: push rbx
141729071: sub rsp, 0x98
141729078: lea rbp, [rdx + 0x80]
14172907f: movaps xmmword ptr [rsp + 0x70], xmm7
141729084: movaps xmmword ptr [rsp + 0x80], xmm6
14172908c: and rdx, 0xffffffffffffff80
141729090: mov rbx, rdx
141729093: lea rcx, [rbx + 0x200]
14172909a: call 0x14088bae0
14172909f: movaps xmm6, xmmword ptr [rsp + 0x80]
1417290a7: movaps xmm7, xmmword ptr [rsp + 0x70]
1417290ac: add rsp, 0x98
1417290b3: pop rbx
1417290b4: pop rdi
1417290b5: pop rsi
1417290b6: pop r12
1417290b8: pop r13
1417290ba: pop r14
1417290bc: pop r15
1417290be: pop rbp
1417290bf: ret 
1417290c0: mov qword ptr [rsp + 0x10], rdx
1417290c5: push rbp
1417290c6: push r15
1417290c8: push r14
1417290ca: push r13
1417290cc: push r12
1417290ce: push rsi
1417290cf: push rdi
1417290d0: push rbx
1417290d1: sub rsp, 0x98
1417290d8: lea rbp, [rdx + 0x80]
1417290df: movaps xmmword ptr [rsp + 0x70], xmm7
1417290e4: movaps xmmword ptr [rsp + 0x80], xmm6
1417290ec: and rdx, 0xffffffffffffff80
1417290f0: mov rbx, rdx
1417290f3: lea rcx, [rbx + 0xf0]
1417290fa: call 0x140d4cc30
1417290ff: movaps xmm6, xmmword ptr [rsp + 0x80]
141729107: movaps xmm7, xmmword ptr [rsp + 0x70]
14172910c: add rsp, 0x98
141729113: pop rbx
141729114: pop rdi
141729115: pop rsi
141729116: pop r12
141729118: pop r13
14172911a: pop r14
14172911c: pop r15
14172911e: pop rbp
14172911f: ret 
141729120: mov qword ptr [rsp + 0x10], rdx
141729125: push rbp
141729126: push r15
141729128: push r14
14172912a: push r13
14172912c: push r12
14172912e: push rsi
14172912f: push rdi
141729130: push rbx
141729131: sub rsp, 0x98
141729138: lea rbp, [rdx + 0x80]
14172913f: movaps xmmword ptr [rsp + 0x70], xmm7
141729144: movaps xmmword ptr [rsp + 0x80], xmm6
14172914c: and rdx, 0xffffffffffffff80
141729150: mov rbx, rdx
141729153: cmp byte ptr [rbx + 0x7d], 0
141729157: je 0x141729165
141729159: lea rcx, [rbx + 0x15a0]
141729160: call 0x140894d60
141729165: movaps xmm6, xmmword ptr [rsp + 0x80]
14172916d: movaps xmm7, xmmword ptr [rsp + 0x70]
141729172: add rsp, 0x98
141729179: pop rbx
14172917a: pop rdi
14172917b: pop rsi
14172917c: pop r12
14172917e: pop r13
141729180: pop r14
141729182: pop r15
141729184: pop rbp
141729185: ret 
141729186: nop word ptr cs:[rax + rax]
141729190: mov qword ptr [rsp + 0x10], rdx
141729195: push rbp
141729196: push r15
141729198: push r14
14172919a: push r13
14172919c: push r12
14172919e: push rsi
14172919f: push rdi
1417291a0: push rbx
1417291a1: sub rsp, 0x98
1417291a8: lea rbp, [rdx + 0x80]
1417291af: movaps xmmword ptr [rsp + 0x70], xmm7
1417291b4: movaps xmmword ptr [rsp + 0x80], xmm6
1417291bc: and rdx, 0xffffffffffffff80
1417291c0: mov rbx, rdx
1417291c3: mov rax, qword ptr [rbx + 0x80]
1417291ca: inc qword ptr [rax]
1417291cd: mov byte ptr [rbx + 0x8f], 0
1417291d4: movaps xmm6, xmmword ptr [rsp + 0x80]
1417291dc: movaps xmm7, xmmword ptr [rsp + 0x70]
1417291e1: add rsp, 0x98
1417291e8: pop rbx
1417291e9: pop rdi
1417291ea: pop rsi
1417291eb: pop r12
1417291ed: pop r13
1417291ef: pop r14
1417291f1: pop r15
1417291f3: pop rbp
1417291f4: ret 
1417291f5: nop word ptr cs:[rax + rax]
141729200: mov qword ptr [rsp + 0x10], rdx
141729205: push rbp
141729206: push r15
141729208: push r14
14172920a: push r13
14172920c: push r12
14172920e: push rsi
14172920f: push rdi
141729210: push rbx
141729211: sub rsp, 0x98
141729218: lea rbp, [rdx + 0x80]
14172921f: movaps xmmword ptr [rsp + 0x70], xmm7
141729224: movaps xmmword ptr [rsp + 0x80], xmm6
14172922c: and rdx, 0xffffffffffffff80
141729230: mov rbx, rdx
141729233: cmp byte ptr [rbx + 0x8f], 0
14172923a: jne 0x14172925d
14172923c: movaps xmm6, xmmword ptr [rsp + 0x80]
141729244: movaps xmm7, xmmword ptr [rsp + 0x70]
141729249: add rsp, 0x98
141729250: pop rbx
141729251: pop rdi
141729252: pop rsi
141729253: pop r12
141729255: pop r13
141729257: pop r14
141729259: pop r15
14172925b: pop rbp
14172925c: ret 
14172925d: lea rcx, [rbx + 0x140]
141729264: call 0x140894d60
141729269: jmp 0x14172923c
14172926b: nop dword ptr [rax + rax]
141729270: mov qword ptr [rsp + 0x10], rdx
141729275: push rbp
141729276: push r15
141729278: push r14
14172927a: push r13
14172927c: push r12
14172927e: push rsi
14172927f: push rdi
141729280: push rbx
141729281: sub rsp, 0x98
141729288: lea rbp, [rdx + 0x80]
14172928f: movaps xmmword ptr [rsp + 0x70], xmm7
141729294: movaps xmmword ptr [rsp + 0x80], xmm6
14172929c: and rdx, 0xffffffffffffff80
1417292a0: mov rbx, rdx
1417292a3: mov rcx, qword ptr [rbx + 0x98]
1417292aa: mov rdx, qword ptr [rbx + 0x80]
1417292b1: call 0x14002e900
1417292b6: movaps xmm6, xmmword ptr [rsp + 0x80]
1417292be: movaps xmm7, xmmword ptr [rsp + 0x70]
1417292c3: add rsp, 0x98
1417292ca: pop rbx
1417292cb: pop rdi
1417292cc: pop rsi
1417292cd: pop r12
1417292cf: pop r13
1417292d1: pop r14
1417292d3: pop r15
1417292d5: pop rbp
1417292d6: ret 
1417292d7: nop word ptr [rax + rax]
1417292e0: mov qword ptr [rsp + 0x10], rdx
1417292e5: push rbp
1417292e6: push r15
1417292e8: push r14
1417292ea: push r13
1417292ec: push r12
1417292ee: push rsi
1417292ef: push rdi
1417292f0: push rbx
1417292f1: sub rsp, 0x98
1417292f8: lea rbp, [rdx + 0x80]
1417292ff: movaps xmmword ptr [rsp + 0x70], xmm7
141729304: movaps xmmword ptr [rsp + 0x80], xmm6
14172930c: and rdx, 0xffffffffffffff80
141729310: mov rbx, rdx
141729313: mov rax, qword ptr [rbx + 0xa8]
14172931a: inc qword ptr [rax]
14172931d: movaps xmm6, xmmword ptr [rsp + 0x80]
141729325: movaps xmm7, xmmword ptr [rsp + 0x70]
14172932a: add rsp, 0x98
141729331: pop rbx
141729332: pop rdi
141729333: pop rsi
141729334: pop r12
141729336: pop r13
141729338: pop r14
14172933a: pop r15
14172933c: pop rbp
14172933d: ret 
14172933e: nop 
141729340: mov qword ptr [rsp + 0x10], rdx
141729345: push rbp
141729346: push r15
141729348: push r14
14172934a: push r13
14172934c: push r12
14172934e: push rsi
14172934f: push rdi
141729350: push rbx
141729351: sub rsp, 0x98
141729358: lea rbp, [rdx + 0x80]
14172935f: movaps xmmword ptr [rsp + 0x70], xmm7
141729364: movaps xmmword ptr [rsp + 0x80], xmm6
14172936c: and rdx, 0xffffffffffffff80
141729370: mov rbx, rdx
141729373: cmp qword ptr [rbx + 0x480], 0
14172937b: je 0x1417293ac
14172937d: lea rax, [rbx + 0x488]
141729384: mov qword ptr [rbx + 0x15a0], rax
14172938b: lea rax, [rbx + 0x15a0]
141729392: mov qword ptr [rbx + 0x1340], rax
141729399: lea rcx, [rip + 0x15a1010]
1417293a0: lea rdx, [rbx + 0x1340]
1417293a7: call 0x14132f2d0
1417293ac: movaps xmm6, xmmword ptr [rsp + 0x80]
1417293b4: movaps xmm7, xmmword ptr [rsp + 0x70]
1417293b9: add rsp, 0x98
1417293c0: pop rbx
1417293c1: pop rdi
1417293c2: pop rsi
1417293c3: pop r12
1417293c5: pop r13
1417293c7: pop r14
1417293c9: pop r15
1417293cb: pop rbp
1417293cc: ret 
1417293cd: nop dword ptr [rax]
1417293d0: mov qword ptr [rsp + 0x10], rdx
1417293d5: push rbp
1417293d6: push r15
1417293d8: push r14
1417293da: push r13
1417293dc: push r12
1417293de: push rsi
1417293df: push rdi
1417293e0: push rbx
1417293e1: sub rsp, 0x98
1417293e8: lea rbp, [rdx + 0x80]
1417293ef: movaps xmmword ptr [rsp + 0x70], xmm7
1417293f4: movaps xmmword ptr [rsp + 0x80], xmm6
1417293fc: and rdx, 0xffffffffffffff80
141729400: mov rbx, rdx
141729403: lea rcx, [rbx + 0x200]
14172940a: call 0x1405e88a0
14172940f: movaps xmm6, xmmword ptr [rsp + 0x80]
141729417: movaps xmm7, xmmword ptr [rsp + 0x70]
14172941c: add rsp, 0x98
141729423: pop rbx
141729424: pop rdi
141729425: pop rsi
141729426: pop r12
141729428: pop r13
14172942a: pop r14
14172942c: pop r15
14172942e: pop rbp
14172942f: ret 
141729430: mov qword ptr [rsp + 0x10], rdx
141729435: push rbp
141729436: push r15
141729438: push r14
14172943a: push r13
14172943c: push r12
14172943e: push rsi
14172943f: push rdi
141729440: push rbx
141729441: sub rsp, 0x98
141729448: lea rbp, [rdx + 0x80]
14172944f: movaps xmmword ptr [rsp + 0x70], xmm7
141729454: movaps xmmword ptr [rsp + 0x80], xmm6
14172945c: and rdx, 0xffffffffffffff80
141729460: mov rbx, rdx
141729463: cmp qword ptr [rbx + 0xa8], 0
14172946b: je 0x141729486
14172946d: mov r8d, 1
141729473: mov rcx, qword ptr [rbx + 0xb8]
14172947a: mov rdx, qword ptr [rbx + 0xa8]
141729481: call 0x140613c20
141729486: movaps xmm6, xmmword ptr [rsp + 0x80]
14172948e: movaps xmm7, xmmword ptr [rsp + 0x70]
141729493: add rsp, 0x98
14172949a: pop rbx
14172949b: pop rdi
14172949c: pop rsi
14172949d: pop r12
14172949f: pop r13
1417294a1: pop r14
1417294a3: pop r15
1417294a5: pop rbp
1417294a6: ret 
1417294a7: nop word ptr [rax + rax]
1417294b0: mov qword ptr [rsp + 0x10], rdx
1417294b5: push rbp
1417294b6: push r15
1417294b8: push r14
1417294ba: push r13
1417294bc: push r12
1417294be: push rsi
1417294bf: push rdi
1417294c0: push rbx
1417294c1: sub rsp, 0x98
1417294c8: lea rbp, [rdx + 0x80]
1417294cf: movaps xmmword ptr [rsp + 0x70], xmm7
1417294d4: movaps xmmword ptr [rsp + 0x80], xmm6
1417294dc: and rdx, 0xffffffffffffff80
1417294e0: mov rbx, rdx
1417294e3: mov rax, qword ptr [rbx + 0x80]
1417294ea: mov rdx, qword ptr [rax + 8]
1417294ee: test rdx, rdx
1417294f1: je 0x14172950a
1417294f3: mov rax, qword ptr [rbx + 0x80]
1417294fa: mov r8, qword ptr [rax + 0x10]
1417294fe: mov rcx, qword ptr [rbx + 0x98]
141729505: call 0x140613c20
14172950a: movaps xmm6, xmmword ptr [rsp + 0x80]
141729512: movaps xmm7, xmmword ptr [rsp + 0x70]
141729517: add rsp, 0x98
14172951e: pop rbx
14172951f: pop rdi
141729520: pop rsi
141729521: pop r12
141729523: pop r13
141729525: pop r14
141729527: pop r15
141729529: pop rbp
14172952a: ret 
14172952b: nop dword ptr [rax + rax]
141729530: mov qword ptr [rsp + 0x10], rdx
141729535: push rbp
141729536: push r15
141729538: push r14
14172953a: push r13
14172953c: push r12
14172953e: push rsi
14172953f: push rdi
141729540: push rbx
141729541: sub rsp, 0x98
141729548: lea rbp, [rdx + 0x80]
14172954f: movaps xmmword ptr [rsp + 0x70], xmm7
141729554: movaps xmmword ptr [rsp + 0x80], xmm6
14172955c: and rdx, 0xffffffffffffff80
141729560: mov rbx, rdx
141729563: mov rax, qword ptr [rbx + 0xa8]
14172956a: inc qword ptr [rax]
14172956d: movaps xmm6, xmmword ptr [rsp + 0x80]
141729575: movaps xmm7, xmmword ptr [rsp + 0x70]
14172957a: add rsp, 0x98
141729581: pop rbx
141729582: pop rdi
141729583: pop rsi
141729584: pop r12
141729586: pop r13
141729588: pop r14
14172958a: pop r15
14172958c: pop rbp
14172958d: ret 
14172958e: nop 
141729590: mov qword ptr [rsp + 0x10], rdx
141729595: push rbp
141729596: push r15
141729598: push r14
14172959a: push r13
14172959c: push r12
14172959e: push rsi
14172959f: push rdi
1417295a0: push rbx
1417295a1: sub rsp, 0x98
1417295a8: lea rbp, [rdx + 0x80]
1417295af: movaps xmmword ptr [rsp + 0x70], xmm7
1417295b4: movaps xmmword ptr [rsp + 0x80], xmm6
1417295bc: and rdx, 0xffffffffffffff80
1417295c0: mov rbx, rdx
1417295c3: xorps xmm0, xmm0
1417295c6: ucomiss xmm0, dword ptr [rbx + 0xd0]
1417295cd: jb 0x141729613
1417295cf: cmp qword ptr [rbx + 0xa8], 0
1417295d7: je 0x1417295f2
1417295d9: mov r8d, 1
1417295df: mov rcx, qword ptr [rbx + 0xb8]
1417295e6: mov rdx, qword ptr [rbx + 0xa8]
1417295ed: call 0x140613c20
1417295f2: movaps xmm6, xmmword ptr [rsp + 0x80]
1417295fa: movaps xmm7, xmmword ptr [rsp + 0x70]
1417295ff: add rsp, 0x98
141729606: pop rbx
141729607: pop rdi
141729608: pop rsi
141729609: pop r12
14172960b: pop r13
14172960d: pop r14
14172960f: pop r15
141729611: pop rbp
141729612: ret 
141729613: movaps xmm6, xmmword ptr [rsp + 0x80]
14172961b: movaps xmm7, xmmword ptr [rsp + 0x70]
141729620: add rsp, 0x98
141729627: pop rbx
141729628: pop rdi
141729629: pop rsi
14172962a: pop r12
14172962c: pop r13
14172962e: pop r14
141729630: pop r15
141729632: pop rbp
141729633: ret 
141729634: nop word ptr cs:[rax + rax]
141729640: mov qword ptr [rsp + 0x10], rdx
141729645: push rbp
141729646: push r15
141729648: push r14
14172964a: push r13
14172964c: push r12
14172964e: push rsi
14172964f: push rdi
141729650: push rbx
141729651: sub rsp, 0x98
141729658: lea rbp, [rdx + 0x80]
14172965f: movaps xmmword ptr [rsp + 0x70], xmm7
141729664: movaps xmmword ptr [rsp + 0x80], xmm6
14172966c: and rdx, 0xffffffffffffff80
141729670: mov rbx, rdx
141729673: mov rax, qword ptr [rbx + 0x80]
14172967a: mov rdx, qword ptr [rax + 8]
14172967e: test rdx, rdx
141729681: je 0x14172969a
141729683: mov rax, qword ptr [rbx + 0x80]
14172968a: mov r8, qword ptr [rax + 0x10]
14172968e: mov rcx, qword ptr [rbx + 0x98]
141729695: call 0x140613c20
14172969a: movaps xmm6, xmmword ptr [rsp + 0x80]
1417296a2: movaps xmm7, xmmword ptr [rsp + 0x70]
1417296a7: add rsp, 0x98
1417296ae: pop rbx
1417296af: pop rdi
1417296b0: pop rsi
1417296b1: pop r12
1417296b3: pop r13
1417296b5: pop r14
1417296b7: pop r15
1417296b9: pop rbp
1417296ba: ret 
1417296bb: nop dword ptr [rax + rax]
1417296c0: mov qword ptr [rsp + 0x10], rdx
1417296c5: push rbp
1417296c6: push r15
1417296c8: push r14
1417296ca: push r13
1417296cc: push r12
1417296ce: push rsi
1417296cf: push rdi
1417296d0: push rbx
1417296d1: sub rsp, 0x98
1417296d8: lea rbp, [rdx + 0x80]
1417296df: movaps xmmword ptr [rsp + 0x70], xmm7
1417296e4: movaps xmmword ptr [rsp + 0x80], xmm6
1417296ec: and rdx, 0xffffffffffffff80
1417296f0: mov rbx, rdx
1417296f3: mov rax, qword ptr [rbx + 0x98]
1417296fa: inc qword ptr [rax]
1417296fd: movaps xmm6, xmmword ptr [rsp + 0x80]
141729705: movaps xmm7, xmmword ptr [rsp + 0x70]
14172970a: add rsp, 0x98
141729711: pop rbx
141729712: pop rdi
141729713: pop rsi
141729714: pop r12
141729716: pop r13
141729718: pop r14
14172971a: pop r15
14172971c: pop rbp
14172971d: ret 
14172971e: nop 
141729720: mov qword ptr [rsp + 0x10], rdx
141729725: push rbp
141729726: push r15
141729728: push r14
14172972a: push r13
14172972c: push r12
14172972e: push rsi
14172972f: push rdi
141729730: push rbx
141729731: sub rsp, 0x98
141729738: lea rbp, [rdx + 0x80]
14172973f: movaps xmmword ptr [rsp + 0x70], xmm7
141729744: movaps xmmword ptr [rsp + 0x80], xmm6
14172974c: and rdx, 0xffffffffffffff80
141729750: mov rbx, rdx
141729753: cmp qword ptr [rbx + 0x120], 0
14172975b: je 0x14172978c
14172975d: lea rax, [rbx + 0x128]
141729764: mov qword ptr [rbx + 0x1e0], rax
14172976b: lea rax, [rbx + 0x1e0]
141729772: mov qword ptr [rbx + 0x1b8], rax
141729779: lea rcx, [rip + 0x15a0c30]
141729780: lea rdx, [rbx + 0x1b8]
141729787: call 0x14132f2d0
14172978c: movaps xmm6, xmmword ptr [rsp + 0x80]
141729794: movaps xmm7, xmmword ptr [rsp + 0x70]
141729799: add rsp, 0x98
1417297a0: pop rbx
1417297a1: pop rdi
1417297a2: pop rsi
1417297a3: pop r12
1417297a5: pop r13
1417297a7: pop r14
1417297a9: pop r15
1417297ab: pop rbp
1417297ac: ret 
1417297ad: nop dword ptr [rax]
1417297b0: mov qword ptr [rsp + 0x10], rdx
1417297b5: push rbp
1417297b6: push r15
1417297b8: push r14
1417297ba: push r13
1417297bc: push r12
1417297be: push rsi
1417297bf: push rdi
1417297c0: push rbx
1417297c1: sub rsp, 0x98
1417297c8: lea rbp, [rdx + 0x80]
1417297cf: movaps xmmword ptr [rsp + 0x70], xmm7
1417297d4: movaps xmmword ptr [rsp + 0x80], xmm6
1417297dc: and rdx, 0xffffffffffffff80
1417297e0: mov rbx, rdx
1417297e3: mov rdx, qword ptr [rbx + 0x140]
1417297ea: test rdx, rdx
1417297ed: je 0x141729801
1417297ef: mov rcx, qword ptr [rbx + 0x148]
1417297f6: mov r8d, 1
1417297fc: call 0x140613c20
141729801: movaps xmm6, xmmword ptr [rsp + 0x80]
141729809: movaps xmm7, xmmword ptr [rsp + 0x70]
14172980e: add rsp, 0x98
141729815: pop rbx
141729816: pop rdi
141729817: pop rsi
141729818: pop r12
14172981a: pop r13
14172981c: pop r14
14172981e: pop r15
141729820: pop rbp
141729821: ret 
141729822: nop word ptr cs:[rax + rax]
141729830: mov qword ptr [rsp + 0x10], rdx
141729835: push rbp
141729836: push r15
141729838: push r14
14172983a: push r13
14172983c: push r12
14172983e: push rsi
14172983f: push rdi
141729840: push rbx
141729841: sub rsp, 0x98
141729848: lea rbp, [rdx + 0x80]
14172984f: movaps xmmword ptr [rsp + 0x70], xmm7
141729854: movaps xmmword ptr [rsp + 0x80], xmm6
14172985c: and rdx, 0xffffffffffffff80
141729860: mov rbx, rdx
141729863: mov rdx, qword ptr [rbx + 0x700]
14172986a: test rdx, rdx
14172986d: je 0x141729881
14172986f: mov rcx, qword ptr [rbx + 0x708]
141729876: mov r8d, 1
14172987c: call 0x140613c20
141729881: movaps xmm6, xmmword ptr [rsp + 0x80]
141729889: movaps xmm7, xmmword ptr [rsp + 0x70]
14172988e: add rsp, 0x98
141729895: pop rbx
141729896: pop rdi
141729897: pop rsi
141729898: pop r12
14172989a: pop r13
14172989c: pop r14
14172989e: pop r15
1417298a0: pop rbp
1417298a1: ret 
1417298a2: nop word ptr cs:[rax + rax]
1417298b0: mov qword ptr [rsp + 0x10], rdx
1417298b5: push rbp
1417298b6: push r15
1417298b8: push r14
1417298ba: push r13
1417298bc: push r12
1417298be: push rsi
1417298bf: push rdi
1417298c0: push rbx
1417298c1: sub rsp, 0x98
1417298c8: lea rbp, [rdx + 0x80]
1417298cf: movaps xmmword ptr [rsp + 0x70], xmm7
1417298d4: movaps xmmword ptr [rsp + 0x80], xmm6
1417298dc: and rdx, 0xffffffffffffff80
1417298e0: mov rbx, rdx
1417298e3: cmp qword ptr [rbx + 0x98], 0
1417298eb: je 0x141729906
1417298ed: mov r8d, 1
1417298f3: mov rcx, qword ptr [rbx + 0xa8]
1417298fa: mov rdx, qword ptr [rbx + 0x98]
141729901: call 0x140613c20
141729906: mov byte ptr [rbx + 0xb6], 1
14172990d: movaps xmm6, xmmword ptr [rsp + 0x80]
141729915: movaps xmm7, xmmword ptr [rsp + 0x70]
14172991a: add rsp, 0x98
141729921: pop rbx
141729922: pop rdi
141729923: pop rsi
141729924: pop r12
141729926: pop r13
141729928: pop r14
14172992a: pop r15
14172992c: pop rbp
14172992d: ret 
14172992e: nop 
141729930: mov qword ptr [rsp + 0x10], rdx
141729935: push rbp
141729936: push r15
141729938: push r14
14172993a: push r13
14172993c: push r12
14172993e: push rsi
14172993f: push rdi
141729940: push rbx
141729941: sub rsp, 0x98
141729948: lea rbp, [rdx + 0x80]
14172994f: movaps xmmword ptr [rsp + 0x70], xmm7
141729954: movaps xmmword ptr [rsp + 0x80], xmm6
14172995c: and rdx, 0xffffffffffffff80
141729960: mov rbx, rdx
141729963: movzx esi, byte ptr [rbx + 0x7c]
141729967: mov rdx, qword ptr [rbx + 0x1340]
14172996e: test rdx, rdx
141729971: je 0x141729985
141729973: mov rcx, qword ptr [rbx + 0x1348]
14172997a: mov r8d, 1
141729980: call 0x140613c20
141729985: mov byte ptr [rbx + 0xb6], sil
14172998c: movaps xmm6, xmmword ptr [rsp + 0x80]
141729994: movaps xmm7, xmmword ptr [rsp + 0x70]
141729999: add rsp, 0x98
1417299a0: pop rbx
1417299a1: pop rdi
1417299a2: pop rsi
1417299a3: pop r12
1417299a5: pop r13
1417299a7: pop r14
1417299a9: pop r15
1417299ab: pop rbp
1417299ac: ret 
1417299ad: nop dword ptr [rax]
1417299b0: mov qword ptr [rsp + 0x10], rdx
1417299b5: push rbp
1417299b6: push r15
1417299b8: push r14
1417299ba: push r13
1417299bc: push r12
1417299be: push rsi
1417299bf: push rdi
1417299c0: push rbx
1417299c1: sub rsp, 0x98
1417299c8: lea rbp, [rdx + 0x80]
1417299cf: movaps xmmword ptr [rsp + 0x70], xmm7
1417299d4: movaps xmmword ptr [rsp + 0x80], xmm6
1417299dc: and rdx, 0xffffffffffffff80
1417299e0: mov rbx, rdx
1417299e3: mov rdx, qword ptr [rbx + 0x700]
1417299ea: test rdx, rdx
1417299ed: je 0x141729a01
1417299ef: mov rcx, qword ptr [rbx + 0x708]
1417299f6: mov r8d, 1
1417299fc: call 0x140613c20
141729a01: mov byte ptr [rbx + 0x7c], 0
141729a05: movaps xmm6, xmmword ptr [rsp + 0x80]
141729a0d: movaps xmm7, xmmword ptr [rsp + 0x70]
141729a12: add rsp, 0x98
141729a19: pop rbx
141729a1a: pop rdi
141729a1b: pop rsi
141729a1c: pop r12
141729a1e: pop r13
141729a20: pop r14
141729a22: pop r15
141729a24: pop rbp
141729a25: ret 
141729a26: nop word ptr cs:[rax + rax]
141729a30: mov qword ptr [rsp + 0x10], rdx
141729a35: push rbp
141729a36: push r15
141729a38: push r14
141729a3a: push r13
141729a3c: push r12
141729a3e: push rsi
141729a3f: push rdi
141729a40: push rbx
141729a41: sub rsp, 0x98
141729a48: lea rbp, [rdx + 0x80]
141729a4f: movaps xmmword ptr [rsp + 0x70], xmm7
141729a54: movaps xmmword ptr [rsp + 0x80], xmm6
141729a5c: and rdx, 0xffffffffffffff80
141729a60: mov rbx, rdx
141729a63: cmp byte ptr [rbx + 0xca], 0
141729a6a: je 0x141729a7c
141729a6c: mov byte ptr [rbx + 0x7c], 1
141729a70: lea rcx, [rbx + 0x480]
141729a77: call 0x1405e7a00
141729a7c: mov byte ptr [rbx + 0x7c], 1
141729a80: movaps xmm6, xmmword ptr [rsp + 0x80]
141729a88: movaps xmm7, xmmword ptr [rsp + 0x70]
141729a8d: add rsp, 0x98
141729a94: pop rbx
141729a95: pop rdi
141729a96: pop rsi
141729a97: pop r12
141729a99: pop r13
141729a9b: pop r14
141729a9d: pop r15
141729a9f: pop rbp
141729aa0: ret 
141729aa1: nop word ptr cs:[rax + rax]
141729ab0: mov qword ptr [rsp + 0x10], rdx
141729ab5: push rbp
141729ab6: push r15
141729ab8: push r14
141729aba: push r13
141729abc: push r12
141729abe: push rsi
141729abf: push rdi
141729ac0: push rbx
141729ac1: sub rsp, 0x98
141729ac8: lea rbp, [rdx + 0x80]
141729acf: movaps xmmword ptr [rsp + 0x70], xmm7
141729ad4: movaps xmmword ptr [rsp + 0x80], xmm6
141729adc: and rdx, 0xffffffffffffff80
141729ae0: mov rbx, rdx
141729ae3: cmp byte ptr [rbx + 0xc4], 0
141729aea: je 0x141729b34
141729aec: mov rdx, qword ptr [rbx + 0x200]
141729af3: test rdx, rdx
141729af6: je 0x141729b0a
141729af8: mov rcx, qword ptr [rbx + 0x208]
141729aff: mov r8d, 1
141729b05: call 0x140613c20
141729b0a: mov eax, dword ptr [rbx + 0xb8]
141729b10: mov byte ptr [rbx + 0x7c], al
141729b13: movaps xmm6, xmmword ptr [rsp + 0x80]
141729b1b: movaps xmm7, xmmword ptr [rsp + 0x70]
141729b20: add rsp, 0x98
141729b27: pop rbx
141729b28: pop rdi
141729b29: pop rsi
141729b2a: pop r12
141729b2c: pop r13
141729b2e: pop r14
141729b30: pop r15
141729b32: pop rbp
141729b33: ret 
141729b34: mov eax, dword ptr [rbx + 0xb8]
141729b3a: mov byte ptr [rbx + 0x7c], al
141729b3d: movaps xmm6, xmmword ptr [rsp + 0x80]
141729b45: movaps xmm7, xmmword ptr [rsp + 0x70]
141729b4a: add rsp, 0x98
141729b51: pop rbx
141729b52: pop rdi
141729b53: pop rsi
141729b54: pop r12
141729b56: pop r13
141729b58: pop r14
141729b5a: pop r15
141729b5c: pop rbp
141729b5d: ret 
141729b5e: nop 
141729b60: mov qword ptr [rsp + 0x10], rdx
141729b65: push rbp
141729b66: push r15
141729b68: push r14
141729b6a: push r13
141729b6c: push r12
141729b6e: push rsi
141729b6f: push rdi
141729b70: push rbx
141729b71: sub rsp, 0x98
141729b78: lea rbp, [rdx + 0x80]
141729b7f: movaps xmmword ptr [rsp + 0x70], xmm7
141729b84: movaps xmmword ptr [rsp + 0x80], xmm6
141729b8c: and rdx, 0xffffffffffffff80
141729b90: mov rbx, rdx
141729b93: cmp byte ptr [rbx + 0xcb], 0
141729b9a: je 0x141729bb1
141729b9c: mov eax, dword ptr [rbx + 0xb8]
141729ba2: mov byte ptr [rbx + 0x7c], al
141729ba5: lea rcx, [rbx + 0x480]
141729bac: call 0x1405e7a00
141729bb1: mov eax, dword ptr [rbx + 0xb8]
141729bb7: mov byte ptr [rbx + 0x7c], al
141729bba: movaps xmm6, xmmword ptr [rsp + 0x80]
141729bc2: movaps xmm7, xmmword ptr [rsp + 0x70]
141729bc7: add rsp, 0x98
141729bce: pop rbx
141729bcf: pop rdi
141729bd0: pop rsi
141729bd1: pop r12
141729bd3: pop r13
141729bd5: pop r14
141729bd7: pop r15
141729bd9: pop rbp
141729bda: ret 
141729bdb: nop dword ptr [rax + rax]
141729be0: mov qword ptr [rsp + 0x10], rdx
141729be5: push rbp
141729be6: push r15
141729be8: push r14
141729bea: push r13
141729bec: push r12
141729bee: push rsi
141729bef: push rdi
141729bf0: push rbx
141729bf1: sub rsp, 0x98
141729bf8: lea rbp, [rdx + 0x80]
141729bff: movaps xmmword ptr [rsp + 0x70], xmm7
141729c04: movaps xmmword ptr [rsp + 0x80], xmm6
141729c0c: and rdx, 0xffffffffffffff80
141729c10: mov rbx, rdx
141729c13: lea rcx, [rbx + 0x100]
141729c1a: call 0x140877160
141729c1f: mov eax, dword ptr [rbx + 0xb8]
141729c25: mov byte ptr [rbx + 0x7c], al
141729c28: movaps xmm6, xmmword ptr [rsp + 0x80]
141729c30: movaps xmm7, xmmword ptr [rsp + 0x70]
141729c35: add rsp, 0x98
141729c3c: pop rbx
141729c3d: pop rdi
141729c3e: pop rsi
141729c3f: pop r12
141729c41: pop r13
141729c43: pop r14
141729c45: pop r15
141729c47: pop rbp
141729c48: ret 
141729c49: nop dword ptr [rax]
141729c50: mov qword ptr [rsp + 0x10], rdx
141729c55: push rbp
141729c56: push r15
141729c58: push r14
141729c5a: push r13
141729c5c: push r12
141729c5e: push rsi
141729c5f: push rdi
141729c60: push rbx
141729c61: sub rsp, 0x98
141729c68: lea rbp, [rdx + 0x80]
141729c6f: movaps xmmword ptr [rsp + 0x70], xmm7
141729c74: movaps xmmword ptr [rsp + 0x80], xmm6
141729c7c: and rdx, 0xffffffffffffff80
141729c80: mov rbx, rdx
141729c83: mov rdx, qword ptr [rbx + 0x200]
141729c8a: test rdx, rdx
141729c8d: je 0x141729ca1
141729c8f: mov rcx, qword ptr [rbx + 0x208]
141729c96: mov r8d, 1
141729c9c: call 0x140613c20
141729ca1: movaps xmm6, xmmword ptr [rsp + 0x80]
141729ca9: movaps xmm7, xmmword ptr [rsp + 0x70]
141729cae: add rsp, 0x98
141729cb5: pop rbx
141729cb6: pop rdi
141729cb7: pop rsi
141729cb8: pop r12
141729cba: pop r13
141729cbc: pop r14
141729cbe: pop r15
141729cc0: pop rbp
141729cc1: ret 
141729cc2: nop word ptr cs:[rax + rax]
141729cd0: mov qword ptr [rsp + 0x10], rdx
141729cd5: push rbp
141729cd6: push r15
141729cd8: push r14
141729cda: push r13
141729cdc: push r12
141729cde: push rsi
141729cdf: push rdi
141729ce0: push rbx
141729ce1: sub rsp, 0x98
141729ce8: lea rbp, [rdx + 0x80]
141729cef: movaps xmmword ptr [rsp + 0x70], xmm7
141729cf4: movaps xmmword ptr [rsp + 0x80], xmm6
141729cfc: and rdx, 0xffffffffffffff80
141729d00: mov rbx, rdx
141729d03: lea rcx, [rbx + 0xf0]
141729d0a: call 0x1405e7a00
141729d0f: movaps xmm6, xmmword ptr [rsp + 0x80]
141729d17: movaps xmm7, xmmword ptr [rsp + 0x70]
141729d1c: add rsp, 0x98
141729d23: pop rbx
141729d24: pop rdi
141729d25: pop rsi
141729d26: pop r12
141729d28: pop r13
141729d2a: pop r14
141729d2c: pop r15
141729d2e: pop rbp
141729d2f: ret 
141729d30: mov qword ptr [rsp + 0x10], rdx
141729d35: push rbp
141729d36: push r15
141729d38: push r14
141729d3a: push r13
141729d3c: push r12
141729d3e: push rsi
141729d3f: push rdi
141729d40: push rbx
141729d41: sub rsp, 0x98
141729d48: lea rbp, [rdx + 0x80]
141729d4f: movaps xmmword ptr [rsp + 0x70], xmm7
141729d54: movaps xmmword ptr [rsp + 0x80], xmm6
141729d5c: and rdx, 0xffffffffffffff80
141729d60: mov rbx, rdx
141729d63: mov rcx, qword ptr [rbx + 0x80]
141729d6a: mov rax, qword ptr [rbx + 0x98]
141729d71: mov rdx, qword ptr [rax + 8]
141729d75: test rdx, rdx
141729d78: je 0x141729d8a
141729d7a: mov rax, qword ptr [rbx + 0x98]
141729d81: mov r8, qword ptr [rax + 0x10]
141729d85: call 0x140613c20
141729d8a: mov edx, 0x18
141729d8f: mov r8d, 8
141729d95: mov rcx, qword ptr [rbx + 0xa8]
141729d9c: call 0x140613c20
141729da1: movaps xmm6, xmmword ptr [rsp + 0x80]
141729da9: movaps xmm7, xmmword ptr [rsp + 0x70]
141729dae: add rsp, 0x98
141729db5: pop rbx
141729db6: pop rdi
141729db7: pop rsi
141729db8: pop r12
141729dba: pop r13
141729dbc: pop r14
141729dbe: pop r15
141729dc0: pop rbp
141729dc1: ret 
141729dc2: nop word ptr cs:[rax + rax]
141729dd0: mov qword ptr [rsp + 0x10], rdx
141729dd5: push rbp
141729dd6: push r15
141729dd8: push r14
141729dda: push r13
141729ddc: push r12
141729dde: push rsi
141729ddf: push rdi
141729de0: push rbx
141729de1: sub rsp, 0x98
141729de8: lea rbp, [rdx + 0x80]
141729def: movaps xmmword ptr [rsp + 0x70], xmm7
141729df4: movaps xmmword ptr [rsp + 0x80], xmm6
141729dfc: and rdx, 0xffffffffffffff80
141729e00: mov rbx, rdx
141729e03: lea rcx, [rbx + 0x100]
141729e0a: call 0x140d4cc30
141729e0f: mov eax, dword ptr [rbx + 0xb8]
141729e15: mov byte ptr [rbx + 0x7c], al
141729e18: movaps xmm6, xmmword ptr [rsp + 0x80]
141729e20: movaps xmm7, xmmword ptr [rsp + 0x70]
141729e25: add rsp, 0x98
141729e2c: pop rbx
141729e2d: pop rdi
141729e2e: pop rsi
141729e2f: pop r12
141729e31: pop r13
141729e33: pop r14
141729e35: pop r15
141729e37: pop rbp
141729e38: ret 
141729e39: nop dword ptr [rax]
141729e40: mov qword ptr [rsp + 0x10], rdx
141729e45: push rbp
141729e46: push r15
141729e48: push r14
141729e4a: push r13
141729e4c: push r12
141729e4e: push rsi
141729e4f: push rdi
141729e50: push rbx
141729e51: sub rsp, 0x98
141729e58: lea rbp, [rdx + 0x80]
141729e5f: movaps xmmword ptr [rsp + 0x70], xmm7
141729e64: movaps xmmword ptr [rsp + 0x80], xmm6
141729e6c: and rdx, 0xffffffffffffff80
141729e70: mov rbx, rdx
141729e73: test byte ptr [rbx + 0xb6], 1
141729e7a: je 0x141729ebb
141729e7c: mov rdx, qword ptr [rbx + 0x15a0]
141729e83: test rdx, rdx
141729e86: je 0x141729e9a
141729e88: mov rcx, qword ptr [rbx + 0x15a8]
141729e8f: mov r8d, 1
141729e95: call 0x140613c20
141729e9a: movaps xmm6, xmmword ptr [rsp + 0x80]
141729ea2: movaps xmm7, xmmword ptr [rsp + 0x70]
141729ea7: add rsp, 0x98
141729eae: pop rbx
141729eaf: pop rdi
141729eb0: pop rsi
141729eb1: pop r12
141729eb3: pop r13
141729eb5: pop r14
141729eb7: pop r15
141729eb9: pop rbp
141729eba: ret 
141729ebb: movaps xmm6, xmmword ptr [rsp + 0x80]
141729ec3: movaps xmm7, xmmword ptr [rsp + 0x70]
141729ec8: add rsp, 0x98
141729ecf: pop rbx
141729ed0: pop rdi
141729ed1: pop rsi
141729ed2: pop r12
141729ed4: pop r13
141729ed6: pop r14
141729ed8: pop r15
141729eda: pop rbp
141729edb: ret 
141729edc: nop dword ptr [rax]
141729ee0: mov qword ptr [rsp + 0x10], rdx
141729ee5: push rbp
141729ee6: push r15
141729ee8: push r14
141729eea: push r13
141729eec: push r12
141729eee: push rsi
141729eef: push rdi
141729ef0: push rbx
141729ef1: sub rsp, 0x98
141729ef8: lea rbp, [rdx + 0x80]
141729eff: movaps xmmword ptr [rsp + 0x70], xmm7
141729f04: movaps xmmword ptr [rsp + 0x80], xmm6
141729f0c: and rdx, 0xffffffffffffff80
141729f10: mov rbx, rdx
141729f13: lea rcx, [rbx + 0x1340]
141729f1a: call 0x140d4cc30
141729f1f: movaps xmm6, xmmword ptr [rsp + 0x80]
141729f27: movaps xmm7, xmmword ptr [rsp + 0x70]
141729f2c: add rsp, 0x98
141729f33: pop rbx
141729f34: pop rdi
141729f35: pop rsi
141729f36: pop r12
141729f38: pop r13
141729f3a: pop r14
141729f3c: pop r15
141729f3e: pop rbp
141729f3f: ret 
141729f40: mov qword ptr [rsp + 0x10], rdx
141729f45: push rbp
141729f46: push r15
141729f48: push r14
141729f4a: push r13
141729f4c: push r12
141729f4e: push rsi
141729f4f: push rdi
141729f50: push rbx
141729f51: sub rsp, 0x98
141729f58: lea rbp, [rdx + 0x80]
141729f5f: movaps xmmword ptr [rsp + 0x70], xmm7
141729f64: movaps xmmword ptr [rsp + 0x80], xmm6
141729f6c: and rdx, 0xffffffffffffff80
141729f70: mov rbx, rdx
141729f73: mov rax, qword ptr [rbx + 0x80]
141729f7a: inc qword ptr [rax]
141729f7d: movaps xmm6, xmmword ptr [rsp + 0x80]
141729f85: movaps xmm7, xmmword ptr [rsp + 0x70]
141729f8a: add rsp, 0x98
141729f91: pop rbx
141729f92: pop rdi
141729f93: pop rsi
141729f94: pop r12
141729f96: pop r13
141729f98: pop r14
141729f9a: pop r15
141729f9c: pop rbp
141729f9d: ret 
141729f9e: nop 
141729fa0: mov qword ptr [rsp + 0x10], rdx
141729fa5: push rbp
141729fa6: push r15
141729fa8: push r14
141729faa: push r13
141729fac: push r12
141729fae: push rsi
141729faf: push rdi
141729fb0: push rbx
141729fb1: sub rsp, 0x98
141729fb8: lea rbp, [rdx + 0x80]
141729fbf: movaps xmmword ptr [rsp + 0x70], xmm7
141729fc4: movaps xmmword ptr [rsp + 0x80], xmm6
141729fcc: and rdx, 0xffffffffffffff80
141729fd0: mov rbx, rdx
141729fd3: movzx esi, byte ptr [rbx + 0x7e]
141729fd7: lea rcx, [rbx + 0xd8]
141729fde: call 0x140877b30
141729fe3: mov byte ptr [rbx + 0xc5], sil
141729fea: movaps xmm6, xmmword ptr [rsp + 0x80]
141729ff2: movaps xmm7, xmmword ptr [rsp + 0x70]
141729ff7: add rsp, 0x98
141729ffe: pop rbx
141729fff: pop rdi
14172a000: pop rsi
14172a001: pop r12
14172a003: pop r13
14172a005: pop r14
14172a007: pop r15
14172a009: pop rbp
14172a00a: ret 
14172a00b: nop dword ptr [rax + rax]
14172a010: mov qword ptr [rsp + 0x10], rdx
14172a015: push rbp
14172a016: push r15
14172a018: push r14
14172a01a: push r13
14172a01c: push r12
14172a01e: push rsi
14172a01f: push rdi
14172a020: push rbx
14172a021: sub rsp, 0x98
14172a028: lea rbp, [rdx + 0x80]
14172a02f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a034: movaps xmmword ptr [rsp + 0x80], xmm6
14172a03c: and rdx, 0xffffffffffffff80
14172a040: mov rbx, rdx
14172a043: mov rax, qword ptr [rbx + 0x80]
14172a04a: inc qword ptr [rax]
14172a04d: mov byte ptr [rbx + 0x7e], 1
14172a051: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a059: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a05e: add rsp, 0x98
14172a065: pop rbx
14172a066: pop rdi
14172a067: pop rsi
14172a068: pop r12
14172a06a: pop r13
14172a06c: pop r14
14172a06e: pop r15
14172a070: pop rbp
14172a071: ret 
14172a072: nop word ptr cs:[rax + rax]
14172a080: mov qword ptr [rsp + 0x10], rdx
14172a085: push rbp
14172a086: push r15
14172a088: push r14
14172a08a: push r13
14172a08c: push r12
14172a08e: push rsi
14172a08f: push rdi
14172a090: push rbx
14172a091: sub rsp, 0x98
14172a098: lea rbp, [rdx + 0x80]
14172a09f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a0a4: movaps xmmword ptr [rsp + 0x80], xmm6
14172a0ac: and rdx, 0xffffffffffffff80
14172a0b0: mov rbx, rdx
14172a0b3: movzx esi, byte ptr [rbx + 0x97]
14172a0ba: cmp qword ptr [rbx + 0x190], 0
14172a0c2: je 0x14172a0f3
14172a0c4: lea rax, [rbx + 0x198]
14172a0cb: mov qword ptr [rbx + 0x1b8], rax
14172a0d2: lea rax, [rbx + 0x1b8]
14172a0d9: mov qword ptr [rbx + 0xf0], rax
14172a0e0: lea rcx, [rip + 0x15a02c9]
14172a0e7: lea rdx, [rbx + 0xf0]
14172a0ee: call 0x14132f2d0
14172a0f3: and sil, 1
14172a0f7: mov byte ptr [rbx + 0x7e], sil
14172a0fb: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a103: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a108: add rsp, 0x98
14172a10f: pop rbx
14172a110: pop rdi
14172a111: pop rsi
14172a112: pop r12
14172a114: pop r13
14172a116: pop r14
14172a118: pop r15
14172a11a: pop rbp
14172a11b: ret 
14172a11c: nop dword ptr [rax]
14172a120: mov qword ptr [rsp + 0x10], rdx
14172a125: push rbp
14172a126: push r15
14172a128: push r14
14172a12a: push r13
14172a12c: push r12
14172a12e: push rsi
14172a12f: push rdi
14172a130: push rbx
14172a131: sub rsp, 0x98
14172a138: lea rbp, [rdx + 0x80]
14172a13f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a144: movaps xmmword ptr [rsp + 0x80], xmm6
14172a14c: and rdx, 0xffffffffffffff80
14172a150: mov rbx, rdx
14172a153: mov rdx, qword ptr [rbx + 0x1340]
14172a15a: test rdx, rdx
14172a15d: je 0x14172a171
14172a15f: mov rcx, qword ptr [rbx + 0x1348]
14172a166: mov r8d, 1
14172a16c: call 0x140613c20
14172a171: mov byte ptr [rbx + 0x97], 0
14172a178: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a180: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a185: add rsp, 0x98
14172a18c: pop rbx
14172a18d: pop rdi
14172a18e: pop rsi
14172a18f: pop r12
14172a191: pop r13
14172a193: pop r14
14172a195: pop r15
14172a197: pop rbp
14172a198: ret 
14172a199: nop dword ptr [rax]
14172a1a0: mov qword ptr [rsp + 0x10], rdx
14172a1a5: push rbp
14172a1a6: push r15
14172a1a8: push r14
14172a1aa: push r13
14172a1ac: push r12
14172a1ae: push rsi
14172a1af: push rdi
14172a1b0: push rbx
14172a1b1: sub rsp, 0x98
14172a1b8: lea rbp, [rdx + 0x80]
14172a1bf: movaps xmmword ptr [rsp + 0x70], xmm7
14172a1c4: movaps xmmword ptr [rsp + 0x80], xmm6
14172a1cc: and rdx, 0xffffffffffffff80
14172a1d0: mov rbx, rdx
14172a1d3: mov rdx, qword ptr [rbx + 0x700]
14172a1da: test rdx, rdx
14172a1dd: je 0x14172a1f1
14172a1df: mov rcx, qword ptr [rbx + 0x708]
14172a1e6: mov r8d, 1
14172a1ec: call 0x140613c20
14172a1f1: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a1f9: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a1fe: add rsp, 0x98
14172a205: pop rbx
14172a206: pop rdi
14172a207: pop rsi
14172a208: pop r12
14172a20a: pop r13
14172a20c: pop r14
14172a20e: pop r15
14172a210: pop rbp
14172a211: ret 
14172a212: nop word ptr cs:[rax + rax]
14172a220: mov qword ptr [rsp + 0x10], rdx
14172a225: push rbp
14172a226: push r15
14172a228: push r14
14172a22a: push r13
14172a22c: push r12
14172a22e: push rsi
14172a22f: push rdi
14172a230: push rbx
14172a231: sub rsp, 0x98
14172a238: lea rbp, [rdx + 0x80]
14172a23f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a244: movaps xmmword ptr [rsp + 0x80], xmm6
14172a24c: and rdx, 0xffffffffffffff80
14172a250: mov rbx, rdx
14172a253: cmp byte ptr [rbx + 0x95], 0
14172a25a: je 0x14172a29b
14172a25c: mov rdx, qword ptr [rbx + 0x480]
14172a263: test rdx, rdx
14172a266: je 0x14172a27a
14172a268: mov rcx, qword ptr [rbx + 0x488]
14172a26f: mov r8d, 1
14172a275: call 0x140613c20
14172a27a: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a282: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a287: add rsp, 0x98
14172a28e: pop rbx
14172a28f: pop rdi
14172a290: pop rsi
14172a291: pop r12
14172a293: pop r13
14172a295: pop r14
14172a297: pop r15
14172a299: pop rbp
14172a29a: ret 
14172a29b: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a2a3: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a2a8: add rsp, 0x98
14172a2af: pop rbx
14172a2b0: pop rdi
14172a2b1: pop rsi
14172a2b2: pop r12
14172a2b4: pop r13
14172a2b6: pop r14
14172a2b8: pop r15
14172a2ba: pop rbp
14172a2bb: ret 
14172a2bc: nop dword ptr [rax]
14172a2c0: mov qword ptr [rsp + 0x10], rdx
14172a2c5: push rbp
14172a2c6: push r15
14172a2c8: push r14
14172a2ca: push r13
14172a2cc: push r12
14172a2ce: push rsi
14172a2cf: push rdi
14172a2d0: push rbx
14172a2d1: sub rsp, 0x98
14172a2d8: lea rbp, [rdx + 0x80]
14172a2df: movaps xmmword ptr [rsp + 0x70], xmm7
14172a2e4: movaps xmmword ptr [rsp + 0x80], xmm6
14172a2ec: and rdx, 0xffffffffffffff80
14172a2f0: mov rbx, rdx
14172a2f3: mov rax, qword ptr [rbx + 0x80]
14172a2fa: inc qword ptr [rax]
14172a2fd: mov byte ptr [rbx + 0x7e], 0
14172a301: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a309: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a30e: add rsp, 0x98
14172a315: pop rbx
14172a316: pop rdi
14172a317: pop rsi
14172a318: pop r12
14172a31a: pop r13
14172a31c: pop r14
14172a31e: pop r15
14172a320: pop rbp
14172a321: ret 
14172a322: nop word ptr cs:[rax + rax]
14172a330: mov qword ptr [rsp + 0x10], rdx
14172a335: push rbp
14172a336: push r15
14172a338: push r14
14172a33a: push r13
14172a33c: push r12
14172a33e: push rsi
14172a33f: push rdi
14172a340: push rbx
14172a341: sub rsp, 0x98
14172a348: lea rbp, [rdx + 0x80]
14172a34f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a354: movaps xmmword ptr [rsp + 0x80], xmm6
14172a35c: and rdx, 0xffffffffffffff80
14172a360: mov rbx, rdx
14172a363: mov rdx, qword ptr [rbx + 0x700]
14172a36a: test rdx, rdx
14172a36d: je 0x14172a381
14172a36f: mov rcx, qword ptr [rbx + 0x708]
14172a376: mov r8d, 1
14172a37c: call 0x140613c20
14172a381: mov byte ptr [rbx + 0x97], 1
14172a388: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a390: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a395: add rsp, 0x98
14172a39c: pop rbx
14172a39d: pop rdi
14172a39e: pop rsi
14172a39f: pop r12
14172a3a1: pop r13
14172a3a3: pop r14
14172a3a5: pop r15
14172a3a7: pop rbp
14172a3a8: ret 
14172a3a9: nop dword ptr [rax]
14172a3b0: mov qword ptr [rsp + 0x10], rdx
14172a3b5: push rbp
14172a3b6: push r15
14172a3b8: push r14
14172a3ba: push r13
14172a3bc: push r12
14172a3be: push rsi
14172a3bf: push rdi
14172a3c0: push rbx
14172a3c1: sub rsp, 0x98
14172a3c8: lea rbp, [rdx + 0x80]
14172a3cf: movaps xmmword ptr [rsp + 0x70], xmm7
14172a3d4: movaps xmmword ptr [rsp + 0x80], xmm6
14172a3dc: and rdx, 0xffffffffffffff80
14172a3e0: mov rbx, rdx
14172a3e3: movzx esi, byte ptr [rbx + 0x92]
14172a3ea: mov rcx, qword ptr [rbx + 0xb8]
14172a3f1: call 0x140161c30
14172a3f6: mov byte ptr [rbx + 0x8d], 1
14172a3fd: mov byte ptr [rbx + 0x8e], sil
14172a404: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a40c: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a411: add rsp, 0x98
14172a418: pop rbx
14172a419: pop rdi
14172a41a: pop rsi
14172a41b: pop r12
14172a41d: pop r13
14172a41f: pop r14
14172a421: pop r15
14172a423: pop rbp
14172a424: ret 
14172a425: nop word ptr cs:[rax + rax]
14172a430: mov qword ptr [rsp + 0x10], rdx
14172a435: push rbp
14172a436: push r15
14172a438: push r14
14172a43a: push r13
14172a43c: push r12
14172a43e: push rsi
14172a43f: push rdi
14172a440: push rbx
14172a441: sub rsp, 0x98
14172a448: lea rbp, [rdx + 0x80]
14172a44f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a454: movaps xmmword ptr [rsp + 0x80], xmm6
14172a45c: and rdx, 0xffffffffffffff80
14172a460: mov rbx, rdx
14172a463: cmp byte ptr [rbx + 0xcc], 0
14172a46a: je 0x14172a486
14172a46c: mov byte ptr [rbx + 0x8d], 0
14172a473: mov byte ptr [rbx + 0x8e], 0
14172a47a: lea rcx, [rbx + 0x480]
14172a481: call 0x1405e7a00
14172a486: mov byte ptr [rbx + 0x8d], 0
14172a48d: mov byte ptr [rbx + 0x8e], 0
14172a494: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a49c: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a4a1: add rsp, 0x98
14172a4a8: pop rbx
14172a4a9: pop rdi
14172a4aa: pop rsi
14172a4ab: pop r12
14172a4ad: pop r13
14172a4af: pop r14
14172a4b1: pop r15
14172a4b3: pop rbp
14172a4b4: ret 
14172a4b5: nop word ptr cs:[rax + rax]
14172a4c0: mov qword ptr [rsp + 0x10], rdx
14172a4c5: push rbp
14172a4c6: push r15
14172a4c8: push r14
14172a4ca: push r13
14172a4cc: push r12
14172a4ce: push rsi
14172a4cf: push rdi
14172a4d0: push rbx
14172a4d1: sub rsp, 0x98
14172a4d8: lea rbp, [rdx + 0x80]
14172a4df: movaps xmmword ptr [rsp + 0x70], xmm7
14172a4e4: movaps xmmword ptr [rsp + 0x80], xmm6
14172a4ec: and rdx, 0xffffffffffffff80
14172a4f0: mov rbx, rdx
14172a4f3: mov rax, qword ptr [rbx + 0x80]
14172a4fa: inc qword ptr [rax]
14172a4fd: mov byte ptr [rbx + 0x7e], 0
14172a501: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a509: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a50e: add rsp, 0x98
14172a515: pop rbx
14172a516: pop rdi
14172a517: pop rsi
14172a518: pop r12
14172a51a: pop r13
14172a51c: pop r14
14172a51e: pop r15
14172a520: pop rbp
14172a521: ret 
14172a522: nop word ptr cs:[rax + rax]
14172a530: mov qword ptr [rsp + 0x10], rdx
14172a535: push rbp
14172a536: push r15
14172a538: push r14
14172a53a: push r13
14172a53c: push r12
14172a53e: push rsi
14172a53f: push rdi
14172a540: push rbx
14172a541: sub rsp, 0x98
14172a548: lea rbp, [rdx + 0x80]
14172a54f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a554: movaps xmmword ptr [rsp + 0x80], xmm6
14172a55c: and rdx, 0xffffffffffffff80
14172a560: mov rbx, rdx
14172a563: mov rdx, qword ptr [rbx + 0x700]
14172a56a: test rdx, rdx
14172a56d: je 0x14172a581
14172a56f: mov rcx, qword ptr [rbx + 0x708]
14172a576: mov r8d, 1
14172a57c: call 0x140613c20
14172a581: mov byte ptr [rbx + 0x92], 0
14172a588: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a590: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a595: add rsp, 0x98
14172a59c: pop rbx
14172a59d: pop rdi
14172a59e: pop rsi
14172a59f: pop r12
14172a5a1: pop r13
14172a5a3: pop r14
14172a5a5: pop r15
14172a5a7: pop rbp
14172a5a8: ret 
14172a5a9: nop dword ptr [rax]
14172a5b0: mov qword ptr [rsp + 0x10], rdx
14172a5b5: push rbp
14172a5b6: push r15
14172a5b8: push r14
14172a5ba: push r13
14172a5bc: push r12
14172a5be: push rsi
14172a5bf: push rdi
14172a5c0: push rbx
14172a5c1: sub rsp, 0x98
14172a5c8: lea rbp, [rdx + 0x80]
14172a5cf: movaps xmmword ptr [rsp + 0x70], xmm7
14172a5d4: movaps xmmword ptr [rsp + 0x80], xmm6
14172a5dc: and rdx, 0xffffffffffffff80
14172a5e0: mov rbx, rdx
14172a5e3: cmp byte ptr [rbx + 0xcd], 0
14172a5ea: je 0x14172a5ff
14172a5ec: mov byte ptr [rbx + 0x92], 0
14172a5f3: lea rcx, [rbx + 0x480]
14172a5fa: call 0x1405e7a00
14172a5ff: mov byte ptr [rbx + 0x92], 0
14172a606: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a60e: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a613: add rsp, 0x98
14172a61a: pop rbx
14172a61b: pop rdi
14172a61c: pop rsi
14172a61d: pop r12
14172a61f: pop r13
14172a621: pop r14
14172a623: pop r15
14172a625: pop rbp
14172a626: ret 
14172a627: nop word ptr [rax + rax]
14172a630: mov qword ptr [rsp + 0x10], rdx
14172a635: push rbp
14172a636: push r15
14172a638: push r14
14172a63a: push r13
14172a63c: push r12
14172a63e: push rsi
14172a63f: push rdi
14172a640: push rbx
14172a641: sub rsp, 0x98
14172a648: lea rbp, [rdx + 0x80]
14172a64f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a654: movaps xmmword ptr [rsp + 0x80], xmm6
14172a65c: and rdx, 0xffffffffffffff80
14172a660: mov rbx, rdx
14172a663: movzx esi, byte ptr [rbx + 0x8e]
14172a66a: cmp byte ptr [rbx + 0x8d], 0
14172a671: je 0x14172a6bd
14172a673: mov rdx, qword ptr [rbx + 0x100]
14172a67a: test rdx, rdx
14172a67d: je 0x14172a691
14172a67f: mov rcx, qword ptr [rbx + 0x108]
14172a686: mov r8d, 1
14172a68c: call 0x140613c20
14172a691: and sil, 1
14172a695: mov byte ptr [rbx + 0x97], sil
14172a69c: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a6a4: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a6a9: add rsp, 0x98
14172a6b0: pop rbx
14172a6b1: pop rdi
14172a6b2: pop rsi
14172a6b3: pop r12
14172a6b5: pop r13
14172a6b7: pop r14
14172a6b9: pop r15
14172a6bb: pop rbp
14172a6bc: ret 
14172a6bd: and sil, 1
14172a6c1: mov byte ptr [rbx + 0x97], sil
14172a6c8: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a6d0: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a6d5: add rsp, 0x98
14172a6dc: pop rbx
14172a6dd: pop rdi
14172a6de: pop rsi
14172a6df: pop r12
14172a6e1: pop r13
14172a6e3: pop r14
14172a6e5: pop r15
14172a6e7: pop rbp
14172a6e8: ret 
14172a6e9: nop dword ptr [rax]
14172a6f0: mov qword ptr [rsp + 0x10], rdx
14172a6f5: push rbp
14172a6f6: push r15
14172a6f8: push r14
14172a6fa: push r13
14172a6fc: push r12
14172a6fe: push rsi
14172a6ff: push rdi
14172a700: push rbx
14172a701: sub rsp, 0x98
14172a708: lea rbp, [rdx + 0x80]
14172a70f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a714: movaps xmmword ptr [rsp + 0x80], xmm6
14172a71c: and rdx, 0xffffffffffffff80
14172a720: mov rbx, rdx
14172a723: cmp byte ptr [rbx + 0xc5], 0
14172a72a: je 0x14172a74f
14172a72c: cmp qword ptr [rbx + 0x98], 0
14172a734: je 0x14172a74f
14172a736: mov r8d, 1
14172a73c: mov rcx, qword ptr [rbx + 0xa8]
14172a743: mov rdx, qword ptr [rbx + 0x98]
14172a74a: call 0x140613c20
14172a74f: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a757: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a75c: add rsp, 0x98
14172a763: pop rbx
14172a764: pop rdi
14172a765: pop rsi
14172a766: pop r12
14172a768: pop r13
14172a76a: pop r14
14172a76c: pop r15
14172a76e: pop rbp
14172a76f: ret 
14172a770: mov qword ptr [rsp + 0x10], rdx
14172a775: push rbp
14172a776: push r15
14172a778: push r14
14172a77a: push r13
14172a77c: push r12
14172a77e: push rsi
14172a77f: push rdi
14172a780: push rbx
14172a781: sub rsp, 0x98
14172a788: lea rbp, [rdx + 0x80]
14172a78f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a794: movaps xmmword ptr [rsp + 0x80], xmm6
14172a79c: and rdx, 0xffffffffffffff80
14172a7a0: mov rbx, rdx
14172a7a3: movzx esi, byte ptr [rbx + 0x90]
14172a7aa: mov rax, qword ptr [rbx + 0xf0]
14172a7b1: lock dec qword ptr [rax]
14172a7b5: jne 0x14172a7c3
14172a7b7: lea rcx, [rbx + 0xf0]
14172a7be: call 0x1402e5d80
14172a7c3: and sil, 1
14172a7c7: mov byte ptr [rbx + 0xcf], sil
14172a7ce: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a7d6: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a7db: add rsp, 0x98
14172a7e2: pop rbx
14172a7e3: pop rdi
14172a7e4: pop rsi
14172a7e5: pop r12
14172a7e7: pop r13
14172a7e9: pop r14
14172a7eb: pop r15
14172a7ed: pop rbp
14172a7ee: ret 
14172a7ef: nop 
14172a7f0: mov qword ptr [rsp + 0x10], rdx
14172a7f5: push rbp
14172a7f6: push r15
14172a7f8: push r14
14172a7fa: push r13
14172a7fc: push r12
14172a7fe: push rsi
14172a7ff: push rdi
14172a800: push rbx
14172a801: sub rsp, 0x98
14172a808: lea rbp, [rdx + 0x80]
14172a80f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a814: movaps xmmword ptr [rsp + 0x80], xmm6
14172a81c: and rdx, 0xffffffffffffff80
14172a820: mov rbx, rdx
14172a823: mov rax, qword ptr [rbx + 0x98]
14172a82a: inc qword ptr [rax]
14172a82d: mov byte ptr [rbx + 0x90], 1
14172a834: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a83c: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a841: add rsp, 0x98
14172a848: pop rbx
14172a849: pop rdi
14172a84a: pop rsi
14172a84b: pop r12
14172a84d: pop r13
14172a84f: pop r14
14172a851: pop r15
14172a853: pop rbp
14172a854: ret 
14172a855: nop word ptr cs:[rax + rax]
14172a860: mov qword ptr [rsp + 0x10], rdx
14172a865: push rbp
14172a866: push r15
14172a868: push r14
14172a86a: push r13
14172a86c: push r12
14172a86e: push rsi
14172a86f: push rdi
14172a870: push rbx
14172a871: sub rsp, 0x98
14172a878: lea rbp, [rdx + 0x80]
14172a87f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a884: movaps xmmword ptr [rsp + 0x80], xmm6
14172a88c: and rdx, 0xffffffffffffff80
14172a890: mov rbx, rdx
14172a893: cmp byte ptr [rbx + 0x98], 0
14172a89a: je 0x14172a8cb
14172a89c: lea rax, [rbx + 0xe0]
14172a8a3: mov qword ptr [rbx + 0x120], rax
14172a8aa: lea rax, [rbx + 0x120]
14172a8b1: mov qword ptr [rbx + 0x170], rax
14172a8b8: lea rcx, [rip + 0x159faf1]
14172a8bf: lea rdx, [rbx + 0x170]
14172a8c6: call 0x14132f2d0
14172a8cb: movzx eax, byte ptr [rbx + 0xa8]
14172a8d2: mov byte ptr [rbx + 0x90], al
14172a8d8: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a8e0: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a8e5: add rsp, 0x98
14172a8ec: pop rbx
14172a8ed: pop rdi
14172a8ee: pop rsi
14172a8ef: pop r12
14172a8f1: pop r13
14172a8f3: pop r14
14172a8f5: pop r15
14172a8f7: pop rbp
14172a8f8: ret 
14172a8f9: nop dword ptr [rax]
14172a900: mov qword ptr [rsp + 0x10], rdx
14172a905: push rbp
14172a906: push r15
14172a908: push r14
14172a90a: push r13
14172a90c: push r12
14172a90e: push rsi
14172a90f: push rdi
14172a910: push rbx
14172a911: sub rsp, 0x98
14172a918: lea rbp, [rdx + 0x80]
14172a91f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a924: movaps xmmword ptr [rsp + 0x80], xmm6
14172a92c: and rdx, 0xffffffffffffff80
14172a930: mov rbx, rdx
14172a933: cmp qword ptr [rbx + 0xb8], 0
14172a93b: je 0x14172a956
14172a93d: mov r8d, 1
14172a943: mov rcx, qword ptr [rbx + 0xd0]
14172a94a: mov rdx, qword ptr [rbx + 0xb8]
14172a951: call 0x140613c20
14172a956: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a95e: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a963: add rsp, 0x98
14172a96a: pop rbx
14172a96b: pop rdi
14172a96c: pop rsi
14172a96d: pop r12
14172a96f: pop r13
14172a971: pop r14
14172a973: pop r15
14172a975: pop rbp
14172a976: ret 
14172a977: nop word ptr [rax + rax]
14172a980: mov qword ptr [rsp + 0x10], rdx
14172a985: push rbp
14172a986: push r15
14172a988: push r14
14172a98a: push r13
14172a98c: push r12
14172a98e: push rsi
14172a98f: push rdi
14172a990: push rbx
14172a991: sub rsp, 0x98
14172a998: lea rbp, [rdx + 0x80]
14172a99f: movaps xmmword ptr [rsp + 0x70], xmm7
14172a9a4: movaps xmmword ptr [rsp + 0x80], xmm6
14172a9ac: and rdx, 0xffffffffffffff80
14172a9b0: mov rbx, rdx
14172a9b3: mov rdx, qword ptr [rbx + 0x100]
14172a9ba: test rdx, rdx
14172a9bd: je 0x14172a9d1
14172a9bf: mov rcx, qword ptr [rbx + 0x108]
14172a9c6: mov r8d, 1
14172a9cc: call 0x140613c20
14172a9d1: movaps xmm6, xmmword ptr [rsp + 0x80]
14172a9d9: movaps xmm7, xmmword ptr [rsp + 0x70]
14172a9de: add rsp, 0x98
14172a9e5: pop rbx
14172a9e6: pop rdi
14172a9e7: pop rsi
14172a9e8: pop r12
14172a9ea: pop r13
14172a9ec: pop r14
14172a9ee: pop r15
14172a9f0: pop rbp
14172a9f1: ret 
14172a9f2: nop word ptr cs:[rax + rax]
14172aa00: mov qword ptr [rsp + 0x10], rdx
14172aa05: push rbp
14172aa06: push r15
14172aa08: push r14
14172aa0a: push r13
14172aa0c: push r12
14172aa0e: push rsi
14172aa0f: push rdi
14172aa10: push rbx
14172aa11: sub rsp, 0x98
14172aa18: lea rbp, [rdx + 0x80]
14172aa1f: movaps xmmword ptr [rsp + 0x70], xmm7
14172aa24: movaps xmmword ptr [rsp + 0x80], xmm6
14172aa2c: and rdx, 0xffffffffffffff80
14172aa30: mov rbx, rdx
14172aa33: mov rdx, qword ptr [rbx + 0x140]
14172aa3a: test rdx, rdx
14172aa3d: je 0x14172aa51
14172aa3f: mov rcx, qword ptr [rbx + 0x148]
14172aa46: mov r8d, 1
14172aa4c: call 0x140613c20
14172aa51: movaps xmm6, xmmword ptr [rsp + 0x80]
14172aa59: movaps xmm7, xmmword ptr [rsp + 0x70]
14172aa5e: add rsp, 0x98
14172aa65: pop rbx
14172aa66: pop rdi
14172aa67: pop rsi
14172aa68: pop r12
14172aa6a: pop r13
14172aa6c: pop r14
14172aa6e: pop r15
14172aa70: pop rbp
14172aa71: ret 
14172aa72: nop word ptr cs:[rax + rax]
14172aa80: mov qword ptr [rsp + 0x10], rdx
14172aa85: push rbp
14172aa86: push r15
14172aa88: push r14
14172aa8a: push r13
14172aa8c: push r12
14172aa8e: push rsi
14172aa8f: push rdi
14172aa90: push rbx
14172aa91: sub rsp, 0x98
14172aa98: lea rbp, [rdx + 0x80]
14172aa9f: movaps xmmword ptr [rsp + 0x70], xmm7
14172aaa4: movaps xmmword ptr [rsp + 0x80], xmm6
14172aaac: and rdx, 0xffffffffffffff80
14172aab0: mov rbx, rdx
14172aab3: mov rdx, qword ptr [rbx + 0x480]
14172aaba: test rdx, rdx
14172aabd: je 0x14172aad1
14172aabf: mov r8d, 1
14172aac5: mov rcx, qword ptr [rbx + 0x1c8]
14172aacc: call 0x140613c20
14172aad1: movaps xmm6, xmmword ptr [rsp + 0x80]
14172aad9: movaps xmm7, xmmword ptr [rsp + 0x70]
14172aade: add rsp, 0x98
14172aae5: pop rbx
14172aae6: pop rdi
14172aae7: pop rsi
14172aae8: pop r12
14172aaea: pop r13
14172aaec: pop r14
14172aaee: pop r15
14172aaf0: pop rbp
14172aaf1: ret 
14172aaf2: nop word ptr cs:[rax + rax]
14172ab00: mov qword ptr [rsp + 0x10], rdx
14172ab05: push rbp
14172ab06: push r15
14172ab08: push r14
14172ab0a: push r13
14172ab0c: push r12
14172ab0e: push rsi
14172ab0f: push rdi
14172ab10: push rbx
14172ab11: sub rsp, 0x98
14172ab18: lea rbp, [rdx + 0x80]
14172ab1f: movaps xmmword ptr [rsp + 0x70], xmm7
14172ab24: movaps xmmword ptr [rsp + 0x80], xmm6
14172ab2c: and rdx, 0xffffffffffffff80
14172ab30: mov rbx, rdx
14172ab33: cmp byte ptr [rbx + 0xce], 0
14172ab3a: je 0x14172ab4f
14172ab3c: mov byte ptr [rbx + 0x91], 1
14172ab43: lea rcx, [rbx + 0x480]
14172ab4a: call 0x1405e7a00
14172ab4f: mov byte ptr [rbx + 0x91], 1
14172ab56: movaps xmm6, xmmword ptr [rsp + 0x80]
14172ab5e: movaps xmm7, xmmword ptr [rsp + 0x70]
14172ab63: add rsp, 0x98
14172ab6a: pop rbx
14172ab6b: pop rdi
14172ab6c: pop rsi
14172ab6d: pop r12
14172ab6f: pop r13
14172ab71: pop r14
14172ab73: pop r15
14172ab75: pop rbp
14172ab76: ret 
14172ab77: nop word ptr [rax + rax]
14172ab80: mov qword ptr [rsp + 0x10], rdx
14172ab85: push rbp
14172ab86: push r15
14172ab88: push r14
14172ab8a: push r13
14172ab8c: push r12
14172ab8e: push rsi
14172ab8f: push rdi
14172ab90: push rbx
14172ab91: sub rsp, 0x98
14172ab98: lea rbp, [rdx + 0x80]
14172ab9f: movaps xmmword ptr [rsp + 0x70], xmm7
14172aba4: movaps xmmword ptr [rsp + 0x80], xmm6
14172abac: and rdx, 0xffffffffffffff80
14172abb0: mov rbx, rdx
14172abb3: movzx esi, byte ptr [rbx + 0xb7]
14172abba: test sil, 1
14172abbe: je 0x14172abf4
14172abc0: lea rcx, [rbx + 0x1340]
14172abc7: call 0x1405ecc10
14172abcc: mov byte ptr [rbx + 0x91], sil
14172abd3: movaps xmm6, xmmword ptr [rsp + 0x80]
14172abdb: movaps xmm7, xmmword ptr [rsp + 0x70]
14172abe0: add rsp, 0x98
14172abe7: pop rbx
14172abe8: pop rdi
14172abe9: pop rsi
14172abea: pop r12
14172abec: pop r13
14172abee: pop r14
14172abf0: pop r15
14172abf2: pop rbp
14172abf3: ret 
14172abf4: mov byte ptr [rbx + 0x91], sil
14172abfb: movaps xmm6, xmmword ptr [rsp + 0x80]
14172ac03: movaps xmm7, xmmword ptr [rsp + 0x70]
14172ac08: add rsp, 0x98
14172ac0f: pop rbx
14172ac10: pop rdi
14172ac11: pop rsi
14172ac12: pop r12
14172ac14: pop r13
14172ac16: pop r14
14172ac18: pop r15
14172ac1a: pop rbp
14172ac1b: ret 
14172ac1c: nop dword ptr [rax]
14172ac20: mov qword ptr [rsp + 0x10], rdx
14172ac25: push rbp
14172ac26: push r15
14172ac28: push r14
14172ac2a: push r13
14172ac2c: push r12
14172ac2e: push rsi
14172ac2f: push rdi
14172ac30: push rbx
14172ac31: sub rsp, 0x98
14172ac38: lea rbp, [rdx + 0x80]
14172ac3f: movaps xmmword ptr [rsp + 0x70], xmm7
14172ac44: movaps xmmword ptr [rsp + 0x80], xmm6
14172ac4c: and rdx, 0xffffffffffffff80
14172ac50: mov rbx, rdx
14172ac53: test byte ptr [rbx + 0x91], 1
14172ac5a: je 0x14172ac9b
14172ac5c: mov rdx, qword ptr [rbx + 0x15a0]
14172ac63: test rdx, rdx
14172ac66: je 0x14172ac7a
14172ac68: mov rcx, qword ptr [rbx + 0x15a8]
14172ac6f: mov r8d, 1
14172ac75: call 0x140613c20
14172ac7a: movaps xmm6, xmmword ptr [rsp + 0x80]
14172ac82: movaps xmm7, xmmword ptr [rsp + 0x70]
14172ac87: add rsp, 0x98
14172ac8e: pop rbx
14172ac8f: pop rdi
14172ac90: pop rsi
14172ac91: pop r12
14172ac93: pop r13
14172ac95: pop r14
14172ac97: pop r15
14172ac99: pop rbp
14172ac9a: ret 
14172ac9b: movaps xmm6, xmmword ptr [rsp + 0x80]
14172aca3: movaps xmm7, xmmword ptr [rsp + 0x70]
14172aca8: add rsp, 0x98
14172acaf: pop rbx
14172acb0: pop rdi
14172acb1: pop rsi
14172acb2: pop r12
14172acb4: pop r13
14172acb6: pop r14
14172acb8: pop r15
14172acba: pop rbp
14172acbb: ret 
14172acbc: nop dword ptr [rax]
14172acc0: mov qword ptr [rsp + 0x10], rdx
14172acc5: push rbp
14172acc6: push r15
14172acc8: push r14
14172acca: push r13
14172accc: push r12
14172acce: push rsi
14172accf: push rdi
14172acd0: push rbx
14172acd1: sub rsp, 0x98
14172acd8: lea rbp, [rdx + 0x80]
14172acdf: movaps xmmword ptr [rsp + 0x70], xmm7
14172ace4: movaps xmmword ptr [rsp + 0x80], xmm6
14172acec: and rdx, 0xffffffffffffff80
14172acf0: mov rbx, rdx
14172acf3: mov rax, qword ptr [rbx + 0x98]
14172acfa: inc qword ptr [rax]
14172acfd: mov byte ptr [rbx + 0x90], 0
14172ad04: movaps xmm6, xmmword ptr [rsp + 0x80]
14172ad0c: movaps xmm7, xmmword ptr [rsp + 0x70]
14172ad11: add rsp, 0x98
14172ad18: pop rbx
14172ad19: pop rdi
14172ad1a: pop rsi
14172ad1b: pop r12
14172ad1d: pop r13
14172ad1f: pop r14
14172ad21: pop r15
14172ad23: pop rbp
14172ad24: ret 
14172ad25: nop word ptr cs:[rax + rax]
14172ad30: mov qword ptr [rsp + 0x10], rdx
14172ad35: push rbp
14172ad36: push r15
14172ad38: push r14
14172ad3a: push r13
14172ad3c: push r12
14172ad3e: push rsi
14172ad3f: push rdi
14172ad40: push rbx
14172ad41: sub rsp, 0x98
14172ad48: lea rbp, [rdx + 0x80]
14172ad4f: movaps xmmword ptr [rsp + 0x70], xmm7
14172ad54: movaps xmmword ptr [rsp + 0x80], xmm6
14172ad5c: and rdx, 0xffffffffffffff80
14172ad60: mov rbx, rdx
14172ad63: lea rcx, [rbx + 0x480]
14172ad6a: call 0x140d4cc30
14172ad6f: movaps xmm6, xmmword ptr [rsp + 0x80]
14172ad77: movaps xmm7, xmmword ptr [rsp + 0x70]
14172ad7c: add rsp, 0x98
14172ad83: pop rbx
14172ad84: pop rdi
14172ad85: pop rsi
14172ad86: pop r12
14172ad88: pop r13
14172ad8a: pop r14
14172ad8c: pop r15
14172ad8e: pop rbp
14172ad8f: ret 
14172ad90: mov qword ptr [rsp + 0x10], rdx
14172ad95: push rbp
14172ad96: push r15
14172ad98: push r14
14172ad9a: push r13
14172ad9c: push r12
14172ad9e: push rsi
14172ad9f: push rdi
14172ada0: push rbx
14172ada1: sub rsp, 0x98
14172ada8: lea rbp, [rdx + 0x80]
14172adaf: movaps xmmword ptr [rsp + 0x70], xmm7
14172adb4: movaps xmmword ptr [rsp + 0x80], xmm6
14172adbc: and rdx, 0xffffffffffffff80
14172adc0: mov rbx, rdx
14172adc3: cmp byte ptr [rbx + 0xcf], 0
14172adca: jne 0x14172aded
14172adcc: movaps xmm6, xmmword ptr [rsp + 0x80]
14172add4: movaps xmm7, xmmword ptr [rsp + 0x70]
14172add9: add rsp, 0x98
14172ade0: pop rbx
14172ade1: pop rdi
14172ade2: pop rsi
14172ade3: pop r12
14172ade5: pop r13
14172ade7: pop r14
14172ade9: pop r15
14172adeb: pop rbp
14172adec: ret 
14172aded: mov rcx, qword ptr [rbx + 0x80]
14172adf4: call 0x140a3e7a0
14172adf9: jmp 0x14172adcc
14172adfb: nop dword ptr [rax + rax]
14172ae00: mov qword ptr [rsp + 0x10], rdx
14172ae05: push rbp
14172ae06: push r15
14172ae08: push r14
14172ae0a: push r13
14172ae0c: push r12
14172ae0e: push rsi
14172ae0f: push rdi
14172ae10: push rbx
14172ae11: sub rsp, 0x98
14172ae18: lea rbp, [rdx + 0x80]
14172ae1f: movaps xmmword ptr [rsp + 0x70], xmm7
14172ae24: movaps xmmword ptr [rsp + 0x80], xmm6
14172ae2c: and rdx, 0xffffffffffffff80
14172ae30: mov rbx, rdx
14172ae33: lea rcx, [rbx + 0x480]
14172ae3a: call 0x140877b30
14172ae3f: movaps xmm6, xmmword ptr [rsp + 0x80]
14172ae47: movaps xmm7, xmmword ptr [rsp + 0x70]
14172ae4c: add rsp, 0x98
14172ae53: pop rbx
14172ae54: pop rdi
14172ae55: pop rsi
14172ae56: pop r12
14172ae58: pop r13
14172ae5a: pop r14
14172ae5c: pop r15
14172ae5e: pop rbp
14172ae5f: ret 
14172ae60: mov qword ptr [rsp + 0x10], rdx
14172ae65: push rbp
14172ae66: push r15
14172ae68: push r14
14172ae6a: push r13
14172ae6c: push r12
14172ae6e: push rsi
14172ae6f: push rdi
14172ae70: push rbx
14172ae71: sub rsp, 0x98
14172ae78: lea rbp, [rdx + 0x80]
14172ae7f: movaps xmmword ptr [rsp + 0x70], xmm7
14172ae84: movaps xmmword ptr [rsp + 0x80], xmm6
14172ae8c: and rdx, 0xffffffffffffff80
14172ae90: mov rbx, rdx
14172ae93: mov rax, qword ptr [rbx + 0x80]
14172ae9a: inc qword ptr [rax]
14172ae9d: movaps xmm6, xmmword ptr [rsp + 0x80]
14172aea5: movaps xmm7, xmmword ptr [rsp + 0x70]
14172aeaa: add rsp, 0x98
14172aeb1: pop rbx
14172aeb2: pop rdi
14172aeb3: pop rsi
14172aeb4: pop r12
14172aeb6: pop r13
14172aeb8: pop r14
14172aeba: pop r15
14172aebc: pop rbp
14172aebd: ret 
14172aebe: nop 
14172aec0: mov qword ptr [rsp + 0x10], rdx
14172aec5: push rbp
14172aec6: push r15
14172aec8: push r14
14172aeca: push r13
14172aecc: push r12
14172aece: push rsi
14172aecf: push rdi
14172aed0: push rbx
14172aed1: sub rsp, 0x98
14172aed8: lea rbp, [rdx + 0x80]
14172aedf: movaps xmmword ptr [rsp + 0x70], xmm7
14172aee4: movaps xmmword ptr [rsp + 0x80], xmm6
14172aeec: and rdx, 0xffffffffffffff80
14172aef0: mov rbx, rdx
14172aef3: cmp byte ptr [rbx + 0x80], 0
14172aefa: je 0x14172af2b
14172aefc: lea rax, [rbx + 0x208]
14172af03: mov qword ptr [rbx + 0x15a0], rax
14172af0a: lea rax, [rbx + 0x15a0]
14172af11: mov qword ptr [rbx + 0x1340], rax
14172af18: lea rcx, [rip + 0x159f491]
14172af1f: lea rdx, [rbx + 0x1340]
14172af26: call 0x14132f2d0
14172af2b: movaps xmm6, xmmword ptr [rsp + 0x80]
14172af33: movaps xmm7, xmmword ptr [rsp + 0x70]
14172af38: add rsp, 0x98
14172af3f: pop rbx
14172af40: pop rdi
14172af41: pop rsi
14172af42: pop r12
14172af44: pop r13
14172af46: pop r14
14172af48: pop r15
14172af4a: pop rbp
14172af4b: ret 
14172af4c: nop dword ptr [rax]
14172af50: mov qword ptr [rsp + 0x10], rdx
14172af55: push rbp
14172af56: push r15
14172af58: push r14
14172af5a: push r13
14172af5c: push r12
14172af5e: push rsi
14172af5f: push rdi
14172af60: push rbx
14172af61: sub rsp, 0x98
14172af68: lea rbp, [rdx + 0x80]
14172af6f: movaps xmmword ptr [rsp + 0x70], xmm7
14172af74: movaps xmmword ptr [rsp + 0x80], xmm6
14172af7c: and rdx, 0xffffffffffffff80
14172af80: mov rbx, rdx
14172af83: mov rax, qword ptr [rbx + 0x80]
14172af8a: inc qword ptr [rax]
14172af8d: movaps xmm6, xmmword ptr [rsp + 0x80]
14172af95: movaps xmm7, xmmword ptr [rsp + 0x70]
14172af9a: add rsp, 0x98
14172afa1: pop rbx
14172afa2: pop rdi
14172afa3: pop rsi
14172afa4: pop r12
14172afa6: pop r13
14172afa8: pop r14
14172afaa: pop r15
14172afac: pop rbp
14172afad: ret 
14172afae: nop 
14172afb0: mov qword ptr [rsp + 0x10], rdx
14172afb5: push rbp
14172afb6: push r15
14172afb8: push r14
14172afba: push r13
14172afbc: push r12
14172afbe: push rsi
14172afbf: push rdi
14172afc0: push rbx
14172afc1: sub rsp, 0x98
14172afc8: lea rbp, [rdx + 0x80]
14172afcf: movaps xmmword ptr [rsp + 0x70], xmm7
14172afd4: movaps xmmword ptr [rsp + 0x80], xmm6
14172afdc: and rdx, 0xffffffffffffff80
14172afe0: mov rbx, rdx
14172afe3: lea rcx, [rbx + 0x790]
14172afea: call 0x140890ff0
14172afef: lea rcx, [rbx + 0x7d8]
14172aff6: call 0x14087cfb0
14172affb: lea rcx, [rbx + 0x700]
14172b002: call 0x140878420
14172b007: movaps xmm6, xmmword ptr [rsp + 0x80]
14172b00f: movaps xmm7, xmmword ptr [rsp + 0x70]
14172b014: add rsp, 0x98
14172b01b: pop rbx
14172b01c: pop rdi
14172b01d: pop rsi
14172b01e: pop r12
14172b020: pop r13
14172b022: pop r14
14172b024: pop r15
14172b026: pop rbp
14172b027: ret 
14172b028: nop dword ptr [rax + rax]
14172b030: mov qword ptr [rsp + 0x10], rdx
14172b035: push rbp
14172b036: push r15
14172b038: push r14
14172b03a: push r13
14172b03c: push r12
14172b03e: push rsi
14172b03f: push rdi
14172b040: push rbx
14172b041: sub rsp, 0x98
14172b048: lea rbp, [rdx + 0x80]
14172b04f: movaps xmmword ptr [rsp + 0x70], xmm7
14172b054: movaps xmmword ptr [rsp + 0x80], xmm6
14172b05c: and rdx, 0xffffffffffffff80
14172b060: mov rbx, rdx
14172b063: lea rcx, [rbx + 0x968]
14172b06a: lea rdx, [rbx + 0x988]
14172b071: mov r8d, 0x20
14172b077: mov r9d, 0x10
14172b07d: call 0x140052a30
14172b082: mov byte ptr [rbx + 0xa6], 1
14172b089: movaps xmm6, xmmword ptr [rsp + 0x80]
14172b091: movaps xmm7, xmmword ptr [rsp + 0x70]
14172b096: add rsp, 0x98
14172b09d: pop rbx
14172b09e: pop rdi
14172b09f: pop rsi
14172b0a0: pop r12
14172b0a2: pop r13
14172b0a4: pop r14
14172b0a6: pop r15
14172b0a8: pop rbp
14172b0a9: ret 
14172b0aa: nop word ptr [rax + rax]
14172b0b0: mov qword ptr [rsp + 0x10], rdx
14172b0b5: push rbp
14172b0b6: push r15
14172b0b8: push r14
14172b0ba: push r13
14172b0bc: push r12
14172b0be: push rsi
14172b0bf: push rdi
14172b0c0: push rbx
14172b0c1: sub rsp, 0x98
14172b0c8: lea rbp, [rdx + 0x80]
14172b0cf: movaps xmmword ptr [rsp + 0x70], xmm7
14172b0d4: movaps xmmword ptr [rsp + 0x80], xmm6
14172b0dc: and rdx, 0xffffffffffffff80
14172b0e0: mov rbx, rdx
14172b0e3: lea rcx, [rbx + 0x200]
14172b0ea: call 0x14171e690
14172b0ef: movaps xmm6, xmmword ptr [rsp + 0x80]
14172b0f7: movaps xmm7, xmmword ptr [rsp + 0x70]
14172b0fc: add rsp, 0x98
14172b103: pop rbx
14172b104: pop rdi
14172b105: pop rsi
14172b106: pop r12
14172b108: pop r13
14172b10a: pop r14
14172b10c: pop r15
14172b10e: pop rbp
14172b10f: ret 
14172b110: mov qword ptr [rsp + 0x10], rdx
14172b115: push rbp
14172b116: push r15
14172b118: push r14
14172b11a: push r13
14172b11c: push r12
14172b11e: push rsi
14172b11f: push rdi
14172b120: push rbx
14172b121: sub rsp, 0x98
14172b128: lea rbp, [rdx + 0x80]
14172b12f: movaps xmmword ptr [rsp + 0x70], xmm7
14172b134: movaps xmmword ptr [rsp + 0x80], xmm6
14172b13c: and rdx, 0xffffffffffffff80
14172b140: mov rbx, rdx
14172b143: mov byte ptr [rbx + 0xb3], 1
14172b14a: lea rcx, [rbx + 0x700]
14172b151: mov r8d, 0x20
14172b157: mov r9d, 0x10
14172b15d: lea rdx, [rbx + 0x720]
14172b164: call 0x140052a30
14172b169: mov byte ptr [rbx + 0xb3], 1
14172b170: movaps xmm6, xmmword ptr [rsp + 0x80]
14172b178: movaps xmm7, xmmword ptr [rsp + 0x70]
14172b17d: add rsp, 0x98
14172b184: pop rbx
14172b185: pop rdi
14172b186: pop rsi
14172b187: pop r12
14172b189: pop r13
14172b18b: pop r14
14172b18d: pop r15
14172b18f: pop rbp
14172b190: ret 
14172b191: nop word ptr cs:[rax + rax]
14172b1a0: mov qword ptr [rsp + 0x10], rdx
14172b1a5: push rbp
14172b1a6: push r15
14172b1a8: push r14
14172b1aa: push r13
14172b1ac: push r12
14172b1ae: push rsi
14172b1af: push rdi
14172b1b0: push rbx
14172b1b1: sub rsp, 0x98
14172b1b8: lea rbp, [rdx + 0x80]
14172b1bf: movaps xmmword ptr [rsp + 0x70], xmm7
14172b1c4: movaps xmmword ptr [rsp + 0x80], xmm6
14172b1cc: and rdx, 0xffffffffffffff80
14172b1d0: mov rbx, rdx
14172b1d3: mov byte ptr [rbx + 0xa7], 1
14172b1da: lea rcx, [rbx + 0x700]
14172b1e1: call 0x140149f50
14172b1e6: mov byte ptr [rbx + 0xa7], 1
14172b1ed: movaps xmm6, xmmword ptr [rsp + 0x80]
14172b1f5: movaps xmm7, xmmword ptr [rsp + 0x70]
14172b1fa: add rsp, 0x98
14172b201: pop rbx
14172b202: pop rdi
14172b203: pop rsi
14172b204: pop r12
14172b206: pop r13
14172b208: pop r14
14172b20a: pop r15
14172b20c: pop rbp
14172b20d: ret 
14172b20e: nop 
14172b210: mov qword ptr [rsp + 0x10], rdx
14172b215: push rbp
14172b216: push r15
14172b218: push r14
14172b21a: push r13
14172b21c: push r12
14172b21e: push rsi
14172b21f: push rdi
14172b220: push rbx
14172b221: sub rsp, 0x98
14172b228: lea rbp, [rdx + 0x80]
14172b22f: movaps xmmword ptr [rsp + 0x70], xmm7
14172b234: movaps xmmword ptr [rsp + 0x80], xmm6
14172b23c: and rdx, 0xffffffffffffff80
14172b240: mov rbx, rdx
14172b243: mov byte ptr [rbx + 0x94], 1
14172b24a: lea rcx, [rbx + 0x700]
14172b251: call 0x1405e7a00
14172b256: mov byte ptr [rbx + 0x94], 1
14172b25d: movaps xmm6, xmmword ptr [rsp + 0x80]
14172b265: movaps xmm7, xmmword ptr [rsp + 0x70]
14172b26a: add rsp, 0x98
14172b271: pop rbx
14172b272: pop rdi
14172b273: pop rsi
14172b274: pop r12
14172b276: pop r13
14172b278: pop r14
14172b27a: pop r15
14172b27c: pop rbp
14172b27d: ret 

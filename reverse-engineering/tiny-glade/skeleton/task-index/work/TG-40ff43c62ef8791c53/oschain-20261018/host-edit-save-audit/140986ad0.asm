140986ad0: push rbp
140986ad1: push rsi
140986ad2: push rdi
140986ad3: push rbx
140986ad4: sub rsp, 0x318
140986adb: lea rbp, [rsp + 0x80]
140986ae3: mov qword ptr [rbp + 0x290], 0xfffffffffffffffe
140986aee: mov rsi, rcx
140986af1: movzx eax, byte ptr [rip + 0x2cae111]
140986af8: mov ecx, 0x1c0
140986afd: mov edx, 8
140986b02: call 0x140613c10
140986b07: test rax, rax
140986b0a: je 0x140986fcd
140986b10: mov qword ptr [rbp + 0x280], rax
140986b17: movzx eax, byte ptr [rip + 0x2cae0eb]
140986b1e: mov ecx, 0xe0
140986b23: mov edx, 8
140986b28: call 0x140613c10
140986b2d: test rax, rax
140986b30: je 0x140986fdc
140986b36: mov qword ptr [rbp + 0x288], rax
140986b3d: xorps xmm0, xmm0
140986b40: movups xmmword ptr [rbp + 0x160], xmm0
140986b47: movups xmmword ptr [rbp + 0x178], xmm0
140986b4e: movups xmmword ptr [rbp + 0x190], xmm0
140986b55: movups xmmword ptr [rbp + 0x1a8], xmm0
140986b5c: movups xmmword ptr [rbp + 0x1c0], xmm0
140986b63: mov dword ptr [rbp + 0x1d0], 0
140986b6d: movups xmmword ptr [rbp + 0x1f8], xmm0
140986b74: movups xmmword ptr [rbp + 0x210], xmm0
140986b7b: movups xmmword ptr [rbp + 0x228], xmm0
140986b82: movups xmmword ptr [rbp + 0x240], xmm0
140986b89: movups xmmword ptr [rbp + 0x258], xmm0
140986b90: mov dword ptr [rbp + 0x268], 0
140986b9a: mov qword ptr [rbp + 0xf8], 0
140986ba5: mov qword ptr [rbp + 0x140], 0
140986bb0: mov qword ptr [rbp + 0x148], 8
140986bbb: mov qword ptr [rbp + 0x150], 0
140986bc6: mov qword ptr [rbp + 0x158], 0x10
140986bd1: mov qword ptr [rbp + 0x170], 0x10
140986bdc: mov qword ptr [rbp + 0x188], 0x10
140986be7: mov qword ptr [rbp + 0x1a0], 0x10
140986bf2: mov qword ptr [rbp + 0x1b8], 0x10
140986bfd: movabs rbx, 0x8000000000000000
140986c07: mov qword ptr [rbp + 0x1d8], rbx
140986c0e: lea rax, [rip + 0x2131e14]
140986c15: mov qword ptr [rbp + 0x1e0], rax
140986c1c: mov qword ptr [rbp + 0x1e8], 0x67
140986c27: mov qword ptr [rbp + 0x1f0], 0x10
140986c32: mov qword ptr [rbp + 0x208], 0x10
140986c3d: mov qword ptr [rbp + 0x220], 0x10
140986c48: mov qword ptr [rbp + 0x238], 0x10
140986c53: mov qword ptr [rbp + 0x250], 0x10
140986c5e: mov dword ptr [rbp + 0x270], 0
140986c68: mov word ptr [rbp + 0x274], 1
140986c71: mov dword ptr [rbp + 0x278], 0
140986c7b: movzx eax, byte ptr [rip + 0x2cadf87]
140986c82: mov ecx, 0x188
140986c87: mov edx, 8
140986c8c: call 0x140613c10
140986c91: test rax, rax
140986c94: je 0x140986fed
140986c9a: mov rdi, rax
140986c9d: lea rdx, [rbp + 0xf8]
140986ca4: mov r8d, 0x188
140986caa: mov rcx, rax
140986cad: call 0x142923c00
140986cb2: lea r8, [rip + 0x2131be7]
140986cb9: lea rcx, [rbp + 0x88]
140986cc0: mov rdx, rdi
140986cc3: call 0x140ce4b10
140986cc8: lea rcx, [rbp + 0xf8]
140986ccf: call 0x140b64720
140986cd4: movups xmm0, xmmword ptr [rbp + 0xe8]
140986cdb: mov rax, qword ptr [rbp + 0x288]
140986ce2: movups xmmword ptr [rax + 0x60], xmm0
140986ce6: movups xmm0, xmmword ptr [rbp + 0xd8]
140986ced: movups xmmword ptr [rax + 0x50], xmm0
140986cf1: movups xmm0, xmmword ptr [rbp + 0xc8]
140986cf8: movups xmmword ptr [rax + 0x40], xmm0
140986cfc: movups xmm0, xmmword ptr [rbp + 0x88]
140986d03: movups xmm1, xmmword ptr [rbp + 0x98]
140986d0a: movups xmm2, xmmword ptr [rbp + 0xa8]
140986d11: movups xmm3, xmmword ptr [rbp + 0xb8]
140986d18: movups xmmword ptr [rax + 0x30], xmm3
140986d1c: movups xmmword ptr [rax + 0x20], xmm2
140986d20: movups xmmword ptr [rax + 0x10], xmm1
140986d24: movups xmmword ptr [rax], xmm0
140986d27: movups xmm0, xmmword ptr [rbp + 0xf8]
140986d2e: movups xmm1, xmmword ptr [rbp + 0x108]
140986d35: movups xmm2, xmmword ptr [rbp + 0x118]
140986d3c: movups xmm3, xmmword ptr [rbp + 0x128]
140986d43: movups xmmword ptr [rax + 0x70], xmm0
140986d47: movups xmmword ptr [rax + 0x80], xmm1
140986d4e: movups xmmword ptr [rax + 0x90], xmm2
140986d55: movups xmmword ptr [rax + 0xa0], xmm3
140986d5c: movups xmm0, xmmword ptr [rbp + 0x138]
140986d63: movups xmmword ptr [rax + 0xb0], xmm0
140986d6a: movups xmm0, xmmword ptr [rbp + 0x148]
140986d71: movups xmmword ptr [rax + 0xc0], xmm0
140986d78: movups xmm0, xmmword ptr [rbp + 0x158]
140986d7f: movups xmmword ptr [rax + 0xd0], xmm0
140986d86: mov qword ptr [rbp + 0x20], 2
140986d8e: mov qword ptr [rbp + 0x28], rax
140986d92: mov qword ptr [rbp + 0x30], 2
140986d9a: mov qword ptr [rbp + 0x38], 0
140986da2: mov qword ptr [rbp + 0x40], 8
140986daa: xorps xmm0, xmm0
140986dad: movups xmmword ptr [rbp + 0x48], xmm0
140986db1: mov qword ptr [rbp + 0x18], rbx
140986db5: lea rcx, [rbp - 0x58]
140986db9: call 0x14168b890
140986dbe: lea rcx, [rbp + 0x88]
140986dc5: call 0x140b15dc0
140986dca: lea rcx, [rbp + 0xf8]
140986dd1: call 0x141764a40
140986dd6: movups xmm0, xmmword ptr [rbp + 0x78]
140986dda: mov rax, qword ptr [rbp + 0x280]
140986de1: movups xmmword ptr [rax + 0x60], xmm0
140986de5: movups xmm0, xmmword ptr [rbp + 0x68]
140986de9: movups xmmword ptr [rax + 0x50], xmm0
140986ded: movups xmm0, xmmword ptr [rbp + 0x58]
140986df1: movups xmmword ptr [rax + 0x40], xmm0
140986df5: movups xmm0, xmmword ptr [rbp + 0x18]
140986df9: movups xmm1, xmmword ptr [rbp + 0x28]
140986dfd: movups xmm2, xmmword ptr [rbp + 0x38]
140986e01: movups xmm3, xmmword ptr [rbp + 0x48]
140986e05: movups xmmword ptr [rax + 0x30], xmm3
140986e09: movups xmmword ptr [rax + 0x20], xmm2
140986e0d: movups xmmword ptr [rax + 0x10], xmm1
140986e11: movups xmmword ptr [rax], xmm0
140986e14: movups xmm0, xmmword ptr [rbp - 0x58]
140986e18: movups xmm1, xmmword ptr [rbp - 0x48]
140986e1c: movups xmm2, xmmword ptr [rbp - 0x38]
140986e20: movups xmm3, xmmword ptr [rbp - 0x28]
140986e24: movups xmmword ptr [rax + 0x70], xmm0
140986e28: movups xmmword ptr [rax + 0x80], xmm1
140986e2f: movups xmmword ptr [rax + 0x90], xmm2
140986e36: movups xmmword ptr [rax + 0xa0], xmm3
140986e3d: movups xmm0, xmmword ptr [rbp - 0x18]
140986e41: movups xmmword ptr [rax + 0xb0], xmm0
140986e48: movups xmm0, xmmword ptr [rbp - 8]
140986e4c: movups xmmword ptr [rax + 0xc0], xmm0
140986e53: movups xmm0, xmmword ptr [rbp + 8]
140986e57: movups xmmword ptr [rax + 0xd0], xmm0
140986e5e: movups xmm0, xmmword ptr [rbp + 0xe8]
140986e65: movups xmmword ptr [rax + 0x140], xmm0
140986e6c: movups xmm0, xmmword ptr [rbp + 0xd8]
140986e73: movups xmmword ptr [rax + 0x130], xmm0
140986e7a: movups xmm0, xmmword ptr [rbp + 0xc8]
140986e81: movups xmmword ptr [rax + 0x120], xmm0
140986e88: movups xmm0, xmmword ptr [rbp + 0x88]
140986e8f: movups xmm1, xmmword ptr [rbp + 0x98]
140986e96: movups xmm2, xmmword ptr [rbp + 0xa8]
140986e9d: movups xmm3, xmmword ptr [rbp + 0xb8]
140986ea4: movups xmmword ptr [rax + 0x110], xmm3
140986eab: movups xmmword ptr [rax + 0x100], xmm2
140986eb2: movups xmmword ptr [rax + 0xf0], xmm1
140986eb9: movups xmmword ptr [rax + 0xe0], xmm0
140986ec0: movups xmm0, xmmword ptr [rbp + 0xf8]
140986ec7: movups xmm1, xmmword ptr [rbp + 0x108]
140986ece: movups xmm2, xmmword ptr [rbp + 0x118]
140986ed5: movups xmm3, xmmword ptr [rbp + 0x128]
140986edc: movups xmmword ptr [rax + 0x150], xmm0
140986ee3: movups xmmword ptr [rax + 0x160], xmm1
140986eea: movups xmmword ptr [rax + 0x170], xmm2
140986ef1: movups xmmword ptr [rax + 0x180], xmm3
140986ef8: movups xmm0, xmmword ptr [rbp + 0x138]
140986eff: movups xmmword ptr [rax + 0x190], xmm0
140986f06: movups xmm0, xmmword ptr [rbp + 0x148]
140986f0d: movups xmmword ptr [rax + 0x1a0], xmm0
140986f14: movups xmm0, xmmword ptr [rbp + 0x158]
140986f1b: movups xmmword ptr [rax + 0x1b0], xmm0
140986f22: mov qword ptr [rbp + 0x100], 4
140986f2d: mov qword ptr [rbp + 0x108], rax
140986f34: mov qword ptr [rbp + 0x110], 4
140986f3f: mov qword ptr [rbp + 0x118], 0
140986f4a: mov qword ptr [rbp + 0x120], 8
140986f55: xorps xmm0, xmm0
140986f58: movups xmmword ptr [rbp + 0x128], xmm0
140986f5f: mov qword ptr [rbp + 0xf8], rbx
140986f66: lea rcx, [rbp + 0x130]
140986f6d: call 0x1416a00a0
140986f72: movups xmm0, xmmword ptr [rbp + 0x158]
140986f79: movups xmmword ptr [rsi + 0x60], xmm0
140986f7d: movups xmm0, xmmword ptr [rbp + 0x148]
140986f84: movups xmmword ptr [rsi + 0x50], xmm0
140986f88: movups xmm0, xmmword ptr [rbp + 0x138]
140986f8f: movups xmmword ptr [rsi + 0x40], xmm0
140986f93: movups xmm0, xmmword ptr [rbp + 0xf8]
140986f9a: movups xmm1, xmmword ptr [rbp + 0x108]
140986fa1: movups xmm2, xmmword ptr [rbp + 0x118]
140986fa8: movups xmm3, xmmword ptr [rbp + 0x128]
140986faf: movups xmmword ptr [rsi + 0x30], xmm3
140986fb3: movups xmmword ptr [rsi + 0x20], xmm2
140986fb7: movups xmmword ptr [rsi + 0x10], xmm1
140986fbb: movups xmmword ptr [rsi], xmm0
140986fbe: mov rax, rsi
140986fc1: add rsp, 0x318
140986fc8: pop rbx
140986fc9: pop rdi
140986fca: pop rsi
140986fcb: pop rbp
140986fcc: ret 
140986fcd: mov ecx, 8
140986fd2: mov edx, 0x1c0
140986fd7: call 0x1428d9000
140986fdc: mov ecx, 8
140986fe1: mov edx, 0xe0
140986fe6: call 0x1428d9000
140986feb: jmp 0x140986ffc
140986fed: mov ecx, 8
140986ff2: mov edx, 0x188
140986ff7: call 0x1428d9000
140986ffc: ud2 
140986ffe: nop 
140987000: mov qword ptr [rsp + 0x10], rdx
140987005: push rbp
140987006: push rsi
140987007: push rdi
140987008: push rbx
140987009: sub rsp, 0x28
14098700d: lea rbp, [rdx + 0x80]
140987014: lea rcx, [rbp + 0xf8]
14098701b: call 0x1405d0e50
140987020: nop 
140987021: add rsp, 0x28
140987025: pop rbx
140987026: pop rdi
140987027: pop rsi
140987028: pop rbp
140987029: ret 
14098702a: nop word ptr [rax + rax]
140987030: mov qword ptr [rsp + 0x10], rdx
140987035: push rbp
140987036: push rsi
140987037: push rdi
140987038: push rbx
140987039: sub rsp, 0x28
14098703d: lea rbp, [rdx + 0x80]
140987044: mov edx, 0xe0
140987049: mov r8d, 8
14098704f: mov rcx, qword ptr [rbp + 0x288]
140987056: call 0x140613c20
14098705b: nop 
14098705c: add rsp, 0x28
140987060: pop rbx
140987061: pop rdi
140987062: pop rsi
140987063: pop rbp
140987064: ret 
140987065: nop word ptr cs:[rax + rax]
140987070: mov qword ptr [rsp + 0x10], rdx
140987075: push rbp
140987076: push rsi
140987077: push rdi
140987078: push rbx
140987079: sub rsp, 0x28
14098707d: lea rbp, [rdx + 0x80]
140987084: lea rcx, [rbp + 0x88]
14098708b: call 0x140029880
140987090: nop 
140987091: add rsp, 0x28
140987095: pop rbx
140987096: pop rdi
140987097: pop rsi
140987098: pop rbp
140987099: ret 
14098709a: nop word ptr [rax + rax]
1409870a0: mov qword ptr [rsp + 0x10], rdx
1409870a5: push rbp
1409870a6: push rsi
1409870a7: push rdi
1409870a8: push rbx
1409870a9: sub rsp, 0x28
1409870ad: lea rbp, [rdx + 0x80]
1409870b4: mov edx, 0x1c0
1409870b9: mov r8d, 8
1409870bf: mov rcx, qword ptr [rbp + 0x280]
1409870c6: call 0x140613c20
1409870cb: nop 
1409870cc: add rsp, 0x28
1409870d0: pop rbx
1409870d1: pop rdi
1409870d2: pop rsi
1409870d3: pop rbp
1409870d4: ret 
1409870d5: nop word ptr cs:[rax + rax]
1409870e0: mov qword ptr [rsp + 0x10], rdx
1409870e5: push rbp
1409870e6: push rsi
1409870e7: push rdi
1409870e8: push rbx
1409870e9: sub rsp, 0x28
1409870ed: lea rbp, [rdx + 0x80]
1409870f4: lea rcx, [rbp + 0x18]
1409870f8: call 0x140029880
1409870fd: nop 
1409870fe: add rsp, 0x28
140987102: pop rbx
140987103: pop rdi
140987104: pop rsi
140987105: pop rbp
140987106: ret 
140987107: nop word ptr [rax + rax]
140987110: mov qword ptr [rsp + 0x10], rdx
140987115: push rbp
140987116: push rsi
140987117: push rdi
140987118: push rbx
140987119: sub rsp, 0x28
14098711d: lea rbp, [rdx + 0x80]
140987124: lea rcx, [rbp - 0x58]
140987128: call 0x140029880
14098712d: nop 
14098712e: add rsp, 0x28
140987132: pop rbx
140987133: pop rdi
140987134: pop rsi
140987135: pop rbp
140987136: ret 
140987137: nop word ptr [rax + rax]
140987140: mov qword ptr [rsp + 0x10], rdx
140987145: push rbp
140987146: push rsi
140987147: push rdi
140987148: push rbx
140987149: sub rsp, 0x28
14098714d: lea rbp, [rdx + 0x80]
140987154: lea rcx, [rbp + 0x88]
14098715b: call 0x140029880
140987160: nop 
140987161: add rsp, 0x28
140987165: pop rbx
140987166: pop rdi
140987167: pop rsi
140987168: pop rbp
140987169: ret 
14098716a: nop word ptr [rax + rax]
140987170: mov qword ptr [rsp + 0x10], rdx
140987175: push rbp
140987176: push rsi
140987177: push rdi
140987178: push rbx
140987179: sub rsp, 0x28
14098717d: lea rbp, [rdx + 0x80]
140987184: lea rcx, [rbp + 0xf8]
14098718b: call 0x140029880
140987190: nop 
140987191: add rsp, 0x28
140987195: pop rbx
140987196: pop rdi
140987197: pop rsi
140987198: pop rbp
140987199: ret 

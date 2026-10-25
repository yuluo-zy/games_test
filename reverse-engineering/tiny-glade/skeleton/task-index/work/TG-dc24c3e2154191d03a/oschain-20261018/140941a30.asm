140941a30: push rbp
140941a31: push r15
140941a33: push r14
140941a35: push r13
140941a37: push r12
140941a39: push rsi
140941a3a: push rdi
140941a3b: push rbx
140941a3c: sub rsp, 0x1e8
140941a43: lea rbp, [rsp + 0x80]
140941a4b: movaps xmmword ptr [rbp + 0x150], xmm6
140941a52: mov qword ptr [rbp + 0x148], 0xfffffffffffffffe
140941a5d: mov rax, qword ptr [rbp + 0x1d0]
140941a64: mov r10d, dword ptr [r9 + 0x1c]
140941a68: mov r11, qword ptr [r9 + 0x10]
140941a6c: mov dword ptr [r11], r10d
140941a6f: mov rbx, qword ptr [r9]
140941a72: mov r9d, dword ptr [rcx + 0x1c]
140941a76: mov dword ptr [rbp + 0xb4], r9d
140941a7d: mov r9, qword ptr [rcx]
140941a80: mov qword ptr [rbp + 0x98], r9
140941a87: mov rcx, qword ptr [rcx + 0x10]
140941a8b: mov qword ptr [rbp + 0x38], rcx
140941a8f: mov ecx, dword ptr [rdx + 0x1c]
140941a92: mov dword ptr [rbp + 0xb8], ecx
140941a98: mov rcx, qword ptr [rdx]
140941a9b: mov qword ptr [rbp + 0x140], rcx
140941aa2: mov rcx, qword ptr [rdx + 0x10]
140941aa6: mov qword ptr [rbp + 0x40], rcx
140941aaa: mov ecx, dword ptr [r8 + 0x1c]
140941aae: mov dword ptr [rbp + 0xbc], ecx
140941ab4: mov rcx, qword ptr [r8]
140941ab7: mov qword ptr [rbp + 0xa0], rcx
140941abe: mov rcx, qword ptr [r8 + 0x10]
140941ac2: mov qword ptr [rbp + 0x50], rcx
140941ac6: lea rdi, [rbx + 0x40]
140941aca: mov r13, qword ptr [rax]
140941acd: mov r12, qword ptr [rax + 8]
140941ad1: xorps xmm6, xmm6
140941ad4: xor r14d, r14d
140941ad7: mov qword ptr [rbp + 0x48], rdi
140941adb: mov qword ptr [rbp + 0x58], r13
140941adf: mov qword ptr [rbp + 0xa8], rbx
140941ae6: jmp 0x140941b4a
140941ae8: nop dword ptr [rax + rax]
140941af0: movups xmm0, xmmword ptr [rbp + 0xc0]
140941af7: movups xmm1, xmmword ptr [rbp + 0xd0]
140941afe: mov rax, qword ptr [rbp + 0x140]
140941b05: movups xmmword ptr [rax + 0x10], xmm1
140941b09: movups xmmword ptr [rax], xmm0
140941b0c: mov esi, dword ptr [rbp + 0xbc]
140941b12: mov r13, qword ptr [rbp + 0x50]
140941b16: mov dword ptr [r13], esi
140941b1a: add r15, 0x30
140941b1e: mov rbx, qword ptr [rbp + 0xa0]
140941b25: mov rcx, rbx
140941b28: mov rdx, r15
140941b2b: xor r8d, r8d
140941b2e: call 0x140911520
140941b33: mov dword ptr [r13], esi
140941b37: mov rcx, rbx
140941b3a: call 0x140911b50
140941b3f: mov rbx, rdi
140941b42: mov rdi, qword ptr [rbp + 0x48]
140941b46: mov r13, qword ptr [rbp + 0x58]
140941b4a: mov rax, qword ptr [r13]
140941b4e: mov rcx, rax
140941b51: sub rcx, qword ptr [r12 + 0x18]
140941b56: cmovb rcx, r14
140941b5a: sub rax, qword ptr [r12 + 0x38]
140941b5f: cmovb rax, r14
140941b63: lea r15, [rcx + rcx*2]
140941b67: shl r15, 4
140941b6b: add r15, qword ptr [r12 + 8]
140941b70: mov r10, qword ptr [r12 + 0x10]
140941b75: mov r11, r10
140941b78: sub r11, rcx
140941b7b: cmovb r11, r14
140941b7f: mov r8, qword ptr [r12 + 0x30]
140941b84: mov rsi, r8
140941b87: sub rsi, rax
140941b8a: cmovb rsi, r14
140941b8e: mov r9, qword ptr [r12 + 0x28]
140941b93: mov rdx, qword ptr [r12 + 0x40]
140941b98: add rsi, r11
140941b9b: sub rdx, rsi
140941b9e: mov qword ptr [r13], rdx
140941ba2: cmp r10, rcx
140941ba5: mov r10d, 8
140941bab: cmovb r15, r10
140941baf: ja 0x140941bcc
140941bb1: lea rcx, [rax + rax*2]
140941bb5: shl rcx, 4
140941bb9: add r9, rcx
140941bbc: cmp r8, rax
140941bbf: cmovb r9, r10
140941bc3: mov r15, r9
140941bc6: jbe 0x140941f4a
140941bcc: inc rdx
140941bcf: mov qword ptr [r13], rdx
140941bd3: cmp r14, qword ptr [r15]
140941bd6: jno 0x140941c80
140941bdc: cmp r14, qword ptr [rdi]
140941bdf: jno 0x140941b4a
140941be5: mov rax, qword ptr [rbp + 0x98]
140941bec: movups xmm0, xmmword ptr [rax]
140941bef: movaps xmmword ptr [rbp - 0x60], xmm0
140941bf3: lea rcx, [rbp + 0x60]
140941bf7: mov rsi, qword ptr [rbp + 0x140]
140941bfe: mov rdx, rsi
140941c01: lea r8, [rip + 0x2170f98]
140941c08: call 0x1412f7fa0
140941c0d: movzx eax, byte ptr [rsi + 0x18]
140941c11: mov byte ptr [rbp + 0x78], al
140941c14: lea rcx, [rbp - 0x38]
140941c18: mov rdx, qword ptr [rbp + 0xa0]
140941c1f: call 0x140911710
140941c24: movaps xmm0, xmmword ptr [rbp - 0x60]
140941c28: lea rcx, [rbp + 0xe0]
140941c2f: movups xmmword ptr [rcx], xmm0
140941c32: movups xmm0, xmmword ptr [rbp + 0x60]
140941c36: movups xmm1, xmmword ptr [rbp + 0x70]
140941c3a: movaps xmmword ptr [rbp + 0xc0], xmm0
140941c41: movaps xmmword ptr [rbp + 0xd0], xmm1
140941c48: mov eax, dword ptr [rbp - 0x18]
140941c4b: mov dword ptr [rcx + 0x30], eax
140941c4e: movups xmm0, xmmword ptr [rbp - 0x38]
140941c52: movups xmm1, xmmword ptr [rbp - 0x28]
140941c56: movups xmmword ptr [rcx + 0x20], xmm1
140941c5a: movups xmmword ptr [rcx + 0x10], xmm0
140941c5e: movzx r8d, byte ptr [r15 + 8]
140941c63: mov rcx, rbx
140941c66: lea rdx, [rbp + 0xc0]
140941c6d: call 0x1409494c0
140941c72: jmp 0x140941b4a
140941c77: nop word ptr [rax + rax]
140941c80: mov qword ptr [rbp + 0x30], r15
140941c84: lea r13, [r15 + 0x20]
140941c88: mov qword ptr [rbp + 0x28], r13
140941c8c: mov rax, qword ptr [rip + 0x2cea715]
140941c93: mov rax, qword ptr [rax]
140941c96: cmp rax, 2
140941c9a: jbe 0x140941d54
140941ca0: lea rcx, [rip + 0x21709b9]
140941ca7: call 0x14067dd80
140941cac: movups xmm0, xmmword ptr [rax]
140941caf: mov eax, dword ptr [rax + 0x10]
140941cb2: mov qword ptr [rbp + 0xf0], 3
140941cbd: lea rdx, [rip + 0x21709b4]
140941cc4: mov qword ptr [rbp + 0xf8], rdx
140941ccb: mov qword ptr [rbp + 0x100], 0x3a
140941cd6: lea rcx, [rip + 0x2170973]
140941cdd: mov qword ptr [rbp + 0x110], rcx
140941ce4: mov qword ptr [rbp + 0x118], 1
140941cef: mov qword ptr [rbp + 0x120], 8
140941cfa: mov qword ptr [rbp + 0xc0], 0
140941d05: lea rcx, [rbp + 0xe0]
140941d0c: movups xmmword ptr [rcx + 0x48], xmm6
140941d10: mov qword ptr [rbp + 0xc8], rdx
140941d17: mov qword ptr [rbp + 0xd0], 0x3a
140941d22: mov qword ptr [rbp + 0xd8], 0
140941d2d: movups xmmword ptr [rbp + 0xe0], xmm0
140941d34: mov dword ptr [rbp + 0x108], 1
140941d3e: mov dword ptr [rbp + 0x10c], eax
140941d44: lea rcx, [rbp - 0x38]
140941d48: lea rdx, [rbp + 0xc0]
140941d4f: call 0x140d4a330
140941d54: mov rcx, r15
140941d57: call 0x140d76ed0
140941d5c: mov r8, rdx
140941d5f: lea rcx, [rbp - 0x38]
140941d63: mov rdx, rax
140941d66: call 0x1409499c0
140941d6b: cmp r14, qword ptr [rbp - 0x38]
140941d6f: jo 0x140942038
140941d75: movups xmm0, xmmword ptr [rbp + 0x18]
140941d79: movaps xmmword ptr [rbp + 0x110], xmm0
140941d80: movups xmm0, xmmword ptr [rbp + 8]
140941d84: movaps xmmword ptr [rbp + 0x100], xmm0
140941d8b: movups xmm0, xmmword ptr [rbp - 0x38]
140941d8f: movups xmm1, xmmword ptr [rbp - 0x28]
140941d93: movups xmm2, xmmword ptr [rbp - 0x18]
140941d97: movups xmm3, xmmword ptr [rbp - 8]
140941d9b: movaps xmmword ptr [rbp + 0xf0], xmm3
140941da2: movaps xmmword ptr [rbp + 0xe0], xmm2
140941da9: movaps xmmword ptr [rbp + 0xd0], xmm1
140941db0: movaps xmmword ptr [rbp + 0xc0], xmm0
140941db7: mov rcx, rbx
140941dba: call 0x140602160
140941dbf: movaps xmm0, xmmword ptr [rbp + 0x110]
140941dc6: mov r8, qword ptr [rbp + 0xa8]
140941dcd: movups xmmword ptr [r8 + 0x50], xmm0
140941dd2: movaps xmm0, xmmword ptr [rbp + 0x100]
140941dd9: movups xmmword ptr [r8 + 0x40], xmm0
140941dde: movaps xmm0, xmmword ptr [rbp + 0xc0]
140941de5: movaps xmm1, xmmword ptr [rbp + 0xd0]
140941dec: movaps xmm2, xmmword ptr [rbp + 0xe0]
140941df3: movaps xmm3, xmmword ptr [rbp + 0xf0]
140941dfa: movups xmmword ptr [r8 + 0x30], xmm3
140941dff: movups xmmword ptr [r8 + 0x20], xmm2
140941e04: movups xmmword ptr [r8 + 0x10], xmm1
140941e09: movups xmmword ptr [r8], xmm0
140941e0d: mov rcx, qword ptr [r8 + 0x18]
140941e11: test rcx, rcx
140941e14: je 0x140942126
140941e1a: dec rcx
140941e1d: mov rax, rcx
140941e20: sub rax, qword ptr [r13]
140941e24: jb 0x140941fc1
140941e2a: movabs rcx, 0x8000000000000002
140941e34: cmp qword ptr [rdi], rcx
140941e37: jne 0x140941f65
140941e3d: mov rcx, qword ptr [r8]
140941e40: mov rsi, qword ptr [r8 + 8]
140941e44: add rax, qword ptr [r8 + 0x10]
140941e48: cmp rax, rcx
140941e4b: mov edx, 0
140941e50: cmovae rdx, rcx
140941e54: sub rax, rdx
140941e57: lea rbx, [rax + rax*2]
140941e5b: shl rbx, 5
140941e5f: lea r15, [rsi + rbx]
140941e63: mov rax, qword ptr [rsi + rbx + 0x58]
140941e68: dec rax
140941e6b: mov rdi, r8
140941e6e: mov qword ptr [r8 + 0x48], rax
140941e72: movups xmm0, xmmword ptr [rsi + rbx + 0x20]
140941e77: movaps xmmword ptr [rbp + 0xc0], xmm0
140941e7e: mov eax, dword ptr [rbp + 0xb4]
140941e84: mov rcx, qword ptr [rbp + 0x38]
140941e88: mov dword ptr [rcx], eax
140941e8a: movaps xmm0, xmmword ptr [rbp + 0xc0]
140941e91: mov rax, qword ptr [rbp + 0x98]
140941e98: movups xmmword ptr [rax], xmm0
140941e9b: lea rcx, [rbp + 0xc0]
140941ea2: mov rdx, r15
140941ea5: lea r8, [rip + 0x2170cf4]
140941eac: call 0x1412f7fa0
140941eb1: movzx eax, byte ptr [rsi + rbx + 0x18]
140941eb6: mov byte ptr [rbp + 0xd8], al
140941ebc: mov eax, dword ptr [rbp + 0xb8]
140941ec2: mov rcx, qword ptr [rbp + 0x40]
140941ec6: mov dword ptr [rcx], eax
140941ec8: mov rax, qword ptr [rbp + 0x140]
140941ecf: mov r13, qword ptr [rax + 8]
140941ed3: mov rbx, qword ptr [rax + 0x10]
140941ed7: test rbx, rbx
140941eda: je 0x140941f20
140941edc: lea rsi, [r13 + 8]
140941ee0: jmp 0x140941ef9
140941ee2: nop word ptr cs:[rax + rax]
140941ef0: add rsi, 0x20
140941ef4: dec rbx
140941ef7: je 0x140941f20
140941ef9: mov rdx, qword ptr [rsi - 8]
140941efd: test rdx, rdx
140941f00: je 0x140941ef0
140941f02: mov rcx, qword ptr [rsi]
140941f05: shl rdx, 4
140941f09: mov r8d, 8
140941f0f: call 0x140613c20
140941f14: jmp 0x140941ef0
140941f16: nop word ptr cs:[rax + rax]
140941f20: mov rax, qword ptr [rbp + 0x140]
140941f27: mov rdx, qword ptr [rax]
140941f2a: test rdx, rdx
140941f2d: je 0x140941af0
140941f33: shl rdx, 5
140941f37: mov r8d, 8
140941f3d: mov rcx, r13
140941f40: call 0x140613c20
140941f45: jmp 0x140941af0
140941f4a: movaps xmm6, xmmword ptr [rbp + 0x150]
140941f51: add rsp, 0x1e8
140941f58: pop rbx
140941f59: pop rdi
140941f5a: pop rsi
140941f5b: pop r12
140941f5d: pop r13
140941f5f: pop r14
140941f61: pop r15
140941f63: pop rbp
140941f64: ret 
140941f65: mov qword ptr [rbp - 0x38], rdi
140941f69: lea rax, [rip + 0xd360]
140941f70: mov qword ptr [rbp - 0x30], rax
140941f74: lea rax, [rip + 0x21716dd]
140941f7b: mov qword ptr [rbp + 0xc0], rax
140941f82: mov qword ptr [rbp + 0xc8], 1
140941f8d: mov qword ptr [rbp + 0xe0], 0
140941f98: lea rax, [rbp - 0x38]
140941f9c: mov qword ptr [rbp + 0xd0], rax
140941fa3: mov qword ptr [rbp + 0xd8], 1
140941fae: lea rdx, [rip + 0x21716b3]
140941fb5: lea rcx, [rbp + 0xc0]
140941fbc: call 0x1428d9390
140941fc1: mov qword ptr [rbp + 0x60], rcx
140941fc5: lea rax, [rbp + 0x28]
140941fc9: mov qword ptr [rbp - 0x38], rax
140941fcd: lea rax, [rip - 0x45d44]
140941fd4: mov qword ptr [rbp - 0x30], rax
140941fd8: lea rax, [rbp + 0x60]
140941fdc: mov qword ptr [rbp - 0x28], rax
140941fe0: lea rax, [rip + 0x44b6d9]
140941fe7: mov qword ptr [rbp - 0x20], rax
140941feb: lea rax, [rip + 0x21707be]
140941ff2: mov qword ptr [rbp + 0xc0], rax
140941ff9: mov qword ptr [rbp + 0xc8], 3
140942004: mov qword ptr [rbp + 0xe0], 0
14094200f: lea rax, [rbp - 0x38]
140942013: mov qword ptr [rbp + 0xd0], rax
14094201a: mov qword ptr [rbp + 0xd8], 2
140942025: lea rdx, [rip + 0x21707b4]
14094202c: lea rcx, [rbp + 0xc0]
140942033: call 0x1428d9390
140942038: mov rax, qword ptr [rbp - 0x30]
14094203c: mov qword ptr [rbp + 0x90], rax
140942043: lea rax, [rbp + 0x30]
140942047: mov qword ptr [rbp - 0x60], rax
14094204b: lea rax, [rip - 0x913762]
140942052: mov qword ptr [rbp - 0x58], rax
140942056: lea rax, [rbp + 0x90]
14094205d: mov qword ptr [rbp - 0x50], rax
140942061: lea rax, [rip + 0x40aba8]
140942068: mov qword ptr [rbp - 0x48], rax
14094206c: mov qword ptr [rbp + 0xc0], 2
140942077: mov qword ptr [rbp + 0xd0], 2
140942082: mov qword ptr [rbp + 0xe0], 0
14094208d: mov qword ptr [rbp + 0xe8], 0x20
140942098: mov byte ptr [rbp + 0xf0], 3
14094209f: mov qword ptr [rbp + 0xf8], 2
1409420aa: mov qword ptr [rbp + 0x108], 2
1409420b5: mov qword ptr [rbp + 0x118], 1
1409420c0: movabs rax, 0x400000020
1409420ca: mov qword ptr [rbp + 0x120], rax
1409420d1: mov byte ptr [rbp + 0x128], 3
1409420d8: lea rax, [rip + 0x2170601]
1409420df: mov qword ptr [rbp + 0x60], rax
1409420e3: mov qword ptr [rbp + 0x68], 2
1409420eb: lea rax, [rbp + 0xc0]
1409420f2: mov qword ptr [rbp + 0x80], rax
1409420f9: mov qword ptr [rbp + 0x88], 2
140942104: lea rax, [rbp - 0x60]
140942108: mov qword ptr [rbp + 0x70], rax
14094210c: mov qword ptr [rbp + 0x78], 2
140942114: lea rdx, [rip + 0x21705e5]
14094211b: lea rcx, [rbp + 0x60]
14094211f: call 0x1428d9390
140942124: ud2 
140942126: lea rax, [rip + 0x2170613]
14094212d: mov qword ptr [rbp + 0xc0], rax
140942134: mov qword ptr [rbp + 0xc8], 1
14094213f: mov qword ptr [rbp + 0xd0], 8
14094214a: xorps xmm0, xmm0
14094214d: movups xmmword ptr [rbp + 0xd8], xmm0
140942154: lea rdx, [rip + 0x21705f5]
14094215b: lea rcx, [rbp + 0xc0]
140942162: call 0x1428d9390
140942167: int3 
140942168: nop dword ptr [rax + rax]
140942170: mov qword ptr [rsp + 0x10], rdx
140942175: push rbp
140942176: push r15
140942178: push r14
14094217a: push r13
14094217c: push r12
14094217e: push rsi
14094217f: push rdi
140942180: push rbx
140942181: sub rsp, 0x38
140942185: lea rbp, [rdx + 0x80]
14094218c: movaps xmmword ptr [rsp + 0x20], xmm6
140942191: movaps xmm0, xmmword ptr [rbp + 0x110]
140942198: mov rax, qword ptr [rbp + 0xa8]
14094219f: movups xmmword ptr [rax + 0x50], xmm0
1409421a3: movaps xmm0, xmmword ptr [rbp + 0x100]
1409421aa: movups xmmword ptr [rax + 0x40], xmm0
1409421ae: movaps xmm0, xmmword ptr [rbp + 0xc0]
1409421b5: movaps xmm1, xmmword ptr [rbp + 0xd0]
1409421bc: movaps xmm2, xmmword ptr [rbp + 0xe0]
1409421c3: movaps xmm3, xmmword ptr [rbp + 0xf0]
1409421ca: movups xmmword ptr [rax + 0x30], xmm3
1409421ce: movups xmmword ptr [rax + 0x20], xmm2
1409421d2: movups xmmword ptr [rax + 0x10], xmm1
1409421d6: movups xmmword ptr [rax], xmm0
1409421d9: movaps xmm6, xmmword ptr [rsp + 0x20]
1409421de: add rsp, 0x38
1409421e2: pop rbx
1409421e3: pop rdi
1409421e4: pop rsi
1409421e5: pop r12
1409421e7: pop r13
1409421e9: pop r14
1409421eb: pop r15
1409421ed: pop rbp
1409421ee: ret 
1409421ef: nop 
1409421f0: mov qword ptr [rsp + 0x10], rdx
1409421f5: push rbp
1409421f6: push r15
1409421f8: push r14
1409421fa: push r13
1409421fc: push r12
1409421fe: push rsi
1409421ff: push rdi
140942200: push rbx
140942201: sub rsp, 0x38
140942205: lea rbp, [rdx + 0x80]
14094220c: movaps xmmword ptr [rsp + 0x20], xmm6
140942211: lea rcx, [rbp + 0x60]
140942215: call 0x14014b1c0
14094221a: movaps xmm6, xmmword ptr [rsp + 0x20]
14094221f: add rsp, 0x38
140942223: pop rbx
140942224: pop rdi
140942225: pop rsi
140942226: pop r12
140942228: pop r13
14094222a: pop r14
14094222c: pop r15
14094222e: pop rbp
14094222f: ret 
140942230: mov qword ptr [rsp + 0x10], rdx
140942235: push rbp
140942236: push r15
140942238: push r14
14094223a: push r13
14094223c: push r12
14094223e: push rsi
14094223f: push rdi
140942240: push rbx
140942241: sub rsp, 0x38
140942245: lea rbp, [rdx + 0x80]
14094224c: movaps xmmword ptr [rsp + 0x20], xmm6
140942251: lea rcx, [rbp + 0x90]
140942258: call 0x140d4cc30
14094225d: movaps xmm6, xmmword ptr [rsp + 0x20]
140942262: add rsp, 0x38
140942266: pop rbx
140942267: pop rdi
140942268: pop rsi
140942269: pop r12
14094226b: pop r13
14094226d: pop r14
14094226f: pop r15
140942271: pop rbp
140942272: ret 

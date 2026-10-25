1416fea60: push rbp
1416fea61: push rsi
1416fea62: push rdi
1416fea63: push rbx
1416fea64: sub rsp, 0x3a8
1416fea6b: lea rbp, [rsp + 0x80]
1416fea73: mov qword ptr [rbp + 0x320], 0xfffffffffffffffe
1416fea7e: mov rsi, rcx
1416fea81: xorps xmm0, xmm0
1416fea84: movups xmmword ptr [rbp + 0x30], xmm0
1416fea88: movups xmmword ptr [rbp + 0x48], xmm0
1416fea8c: movups xmmword ptr [rbp + 0x60], xmm0
1416fea90: movups xmmword ptr [rbp + 0x78], xmm0
1416fea94: movups xmmword ptr [rbp + 0x90], xmm0
1416fea9b: mov dword ptr [rbp + 0xa0], 0
1416feaa5: movups xmmword ptr [rbp + 0xc8], xmm0
1416feaac: movups xmmword ptr [rbp + 0xe0], xmm0
1416feab3: movups xmmword ptr [rbp + 0xf8], xmm0
1416feaba: movups xmmword ptr [rbp + 0x110], xmm0
1416feac1: movups xmmword ptr [rbp + 0x128], xmm0
1416feac8: mov dword ptr [rbp + 0x138], 0
1416fead2: mov qword ptr [rbp + 0x10], 0
1416feada: mov qword ptr [rbp + 0x18], 8
1416feae2: mov qword ptr [rbp + 0x20], 0
1416feaea: mov qword ptr [rbp + 0x28], 0x10
1416feaf2: mov qword ptr [rbp + 0x40], 0x10
1416feafa: mov qword ptr [rbp + 0x58], 0x10
1416feb02: mov qword ptr [rbp + 0x70], 0x10
1416feb0a: mov qword ptr [rbp + 0x88], 0x10
1416feb15: movabs rax, 0x8000000000000000
1416feb1f: mov qword ptr [rbp + 0xa8], rax
1416feb26: lea rcx, [rip + 0x15943f0]
1416feb2d: mov qword ptr [rbp + 0xb0], rcx
1416feb34: mov qword ptr [rbp + 0xb8], 0x36
1416feb3f: mov qword ptr [rbp + 0xc0], 0x10
1416feb4a: mov qword ptr [rbp + 0xd8], 0x10
1416feb55: mov qword ptr [rbp + 0xf0], 0x10
1416feb60: mov qword ptr [rbp + 0x108], 0x10
1416feb6b: mov qword ptr [rbp + 0x120], 0x10
1416feb76: mov dword ptr [rbp + 0x140], 0
1416feb80: mov word ptr [rbp + 0x144], 1
1416feb89: mov qword ptr [rbp + 0x240], rax
1416feb90: mov dword ptr [rbp + 0x318], 0
1416feb9a: movzx eax, byte ptr [rip + 0x1f36068]
1416feba1: mov ecx, 0x310
1416feba6: mov edx, 8
1416febab: call 0x140613c10
1416febb0: test rax, rax
1416febb3: je 0x1416febfa
1416febb5: mov rdi, rax
1416febb8: lea rdx, [rbp + 0x10]
1416febbc: mov r8d, 0x310
1416febc2: mov rcx, rax
1416febc5: call 0x142923c00
1416febca: lea r8, [rip + 0x1593faf]
1416febd1: lea rbx, [rbp - 0x60]
1416febd5: mov rcx, rbx
1416febd8: mov rdx, rdi
1416febdb: call 0x140ce4b10
1416febe0: mov rcx, rsi
1416febe3: mov rdx, rbx
1416febe6: call 0x14144f660
1416febeb: mov rax, rsi
1416febee: add rsp, 0x3a8
1416febf5: pop rbx
1416febf6: pop rdi
1416febf7: pop rsi
1416febf8: pop rbp
1416febf9: ret 
1416febfa: mov ecx, 8
1416febff: mov edx, 0x310
1416fec04: call 0x1428d9000
1416fec09: ud2 
1416fec0b: nop dword ptr [rax + rax]
1416fec10: mov qword ptr [rsp + 0x10], rdx
1416fec15: push rbp
1416fec16: push rsi
1416fec17: push rdi
1416fec18: push rbx
1416fec19: sub rsp, 0x28
1416fec1d: lea rbp, [rdx + 0x80]
1416fec24: lea rcx, [rbp + 0x10]
1416fec28: call 0x1405cf3b0
1416fec2d: nop 
1416fec2e: add rsp, 0x28
1416fec32: pop rbx
1416fec33: pop rdi
1416fec34: pop rsi
1416fec35: pop rbp
1416fec36: ret 
1416fec37: int3 
1416fec38: int3 
1416fec39: int3 
1416fec3a: int3 

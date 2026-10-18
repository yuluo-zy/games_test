1412c4e80: push rbp
1412c4e81: push rsi
1412c4e82: sub rsp, 0xe8
1412c4e89: lea rbp, [rsp + 0x80]
1412c4e91: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c4e99: mov rsi, rcx
1412c4e9c: lea rcx, [rbp - 0x58]
1412c4ea0: xor r8d, r8d
1412c4ea3: call 0x14161ad70
1412c4ea8: movzx eax, byte ptr [rip + 0x236fd5a]
1412c4eaf: mov ecx, 0x38
1412c4eb4: mov edx, 8
1412c4eb9: call 0x140613c10
1412c4ebe: test rax, rax
1412c4ec1: je 0x1412c4f7f
1412c4ec7: mov rcx, qword ptr [rbp - 0x28]
1412c4ecb: mov qword ptr [rax + 0x30], rcx
1412c4ecf: movups xmm0, xmmword ptr [rbp - 0x58]
1412c4ed3: movups xmm1, xmmword ptr [rbp - 0x48]
1412c4ed7: movups xmm2, xmmword ptr [rbp - 0x38]
1412c4edb: movups xmmword ptr [rax + 0x20], xmm2
1412c4edf: movups xmmword ptr [rax + 0x10], xmm1
1412c4ee3: movups xmmword ptr [rax], xmm0
1412c4ee6: lea rcx, [rip + 0x17d86dc]
1412c4eed: mov qword ptr [rbp + 0x10], rcx
1412c4ef1: mov qword ptr [rbp + 0x18], 0x49
1412c4ef9: mov qword ptr [rbp + 0x20], rax
1412c4efd: lea rax, [rip + 0x19682bc]
1412c4f04: mov qword ptr [rbp + 0x28], rax
1412c4f08: mov qword ptr [rbp + 0x40], 0
1412c4f10: mov byte ptr [rbp + 0x56], 5
1412c4f14: mov qword ptr [rbp + 0x30], rcx
1412c4f18: mov qword ptr [rbp + 0x38], 0x49
1412c4f20: movzx eax, byte ptr [rip + 0x236fce2]
1412c4f27: mov ecx, 0x40
1412c4f2c: mov edx, 8
1412c4f31: call 0x140613c10
1412c4f36: test rax, rax
1412c4f39: je 0x1412c4f90
1412c4f3b: movups xmm0, xmmword ptr [rbp + 0x20]
1412c4f3f: movups xmm1, xmmword ptr [rbp + 0x30]
1412c4f43: movups xmm2, xmmword ptr [rbp + 0x40]
1412c4f47: movups xmm3, xmmword ptr [rbp + 0x50]
1412c4f4b: movups xmmword ptr [rax + 0x30], xmm3
1412c4f4f: movups xmmword ptr [rax + 0x20], xmm2
1412c4f53: movups xmmword ptr [rax + 0x10], xmm1
1412c4f57: movups xmmword ptr [rax], xmm0
1412c4f5a: movabs rdx, 0xb004e39a49a347c5
1412c4f64: mov rcx, rsi
1412c4f67: mov r8, rax
1412c4f6a: call 0x140969bf0
1412c4f6f: cmp rax, 1
1412c4f73: je 0x1412c4fa1
1412c4f75: add rsp, 0xe8
1412c4f7c: pop rsi
1412c4f7d: pop rbp
1412c4f7e: ret 
1412c4f7f: mov ecx, 8
1412c4f84: mov edx, 0x38
1412c4f89: call 0x1428d9000
1412c4f8e: jmp 0x1412c4f9f
1412c4f90: mov ecx, 8
1412c4f95: mov edx, 0x40
1412c4f9a: call 0x1428d9000
1412c4f9f: ud2 
1412c4fa1: mov rax, rdx
1412c4fa4: add rdx, 0x10
1412c4fa8: mov rsi, rdx
1412c4fab: cmp qword ptr [rax + 0x18], 0x49
1412c4fb0: jne 0x1412c502f
1412c4fb2: mov rdx, qword ptr [rsi]
1412c4fb5: lea rcx, [rip + 0x17d860d]
1412c4fbc: mov r8d, 0x49
1412c4fc2: call 0x142923bf0
1412c4fc7: test eax, eax
1412c4fc9: jne 0x1412c502f
1412c4fcb: lea rax, [rip + 0x196889e]
1412c4fd2: mov qword ptr [rbp - 0x20], rax
1412c4fd6: lea rax, [rip - 0xd9173d]
1412c4fdd: mov qword ptr [rbp - 0x18], rax
1412c4fe1: lea rax, [rbp + 0x10]
1412c4fe5: mov qword ptr [rbp - 0x10], rax
1412c4fe9: lea rax, [rip - 0x11bb160]
1412c4ff0: mov qword ptr [rbp - 8], rax
1412c4ff4: lea rax, [rip + 0x196862d]
1412c4ffb: mov qword ptr [rbp + 0x20], rax
1412c4fff: mov qword ptr [rbp + 0x28], 3
1412c5007: mov qword ptr [rbp + 0x40], 0
1412c500f: lea rax, [rbp - 0x20]
1412c5013: mov qword ptr [rbp + 0x30], rax
1412c5017: mov qword ptr [rbp + 0x38], 2
1412c501f: lea rdx, [rip + 0x1968632]
1412c5026: lea rcx, [rbp + 0x20]
1412c502a: call 0x1428d9390
1412c502f: lea rax, [rip + 0x196883a]
1412c5036: mov qword ptr [rbp + 0x20], rax
1412c503a: lea rax, [rip - 0xd917a1]
1412c5041: mov qword ptr [rbp + 0x28], rax
1412c5045: mov qword ptr [rbp + 0x30], rsi
1412c5049: lea rax, [rip - 0x11bb1c0]
1412c5050: mov qword ptr [rbp + 0x38], rax
1412c5054: lea rcx, [rbp + 0x10]
1412c5058: mov qword ptr [rbp + 0x40], rcx
1412c505c: mov qword ptr [rbp + 0x48], rax
1412c5060: lea rax, [rip + 0x1968501]
1412c5067: mov qword ptr [rbp - 0x20], rax
1412c506b: mov qword ptr [rbp - 0x18], 3
1412c5073: mov qword ptr [rbp], 0
1412c507b: lea rax, [rbp + 0x20]
1412c507f: mov qword ptr [rbp - 0x10], rax
1412c5083: mov qword ptr [rbp - 8], 3
1412c508b: lea rdx, [rip + 0x196853e]
1412c5092: lea rcx, [rbp - 0x20]
1412c5096: call 0x1428d9390
1412c509b: int3 
1412c509c: nop dword ptr [rax]
1412c50a0: mov qword ptr [rsp + 0x10], rdx
1412c50a5: push rbp
1412c50a6: push rsi
1412c50a7: sub rsp, 0x28
1412c50ab: lea rbp, [rdx + 0x80]
1412c50b2: lea rcx, [rbp - 0x48]
1412c50b6: call 0x140be24b0
1412c50bb: nop 
1412c50bc: add rsp, 0x28
1412c50c0: pop rsi
1412c50c1: pop rbp
1412c50c2: ret 
1412c50c3: nop word ptr cs:[rax + rax]
1412c50d0: mov qword ptr [rsp + 0x10], rdx
1412c50d5: push rbp
1412c50d6: push rsi
1412c50d7: sub rsp, 0x28
1412c50db: lea rbp, [rdx + 0x80]
1412c50e2: lea rcx, [rbp + 0x20]
1412c50e6: call 0x1400c04b0
1412c50eb: nop 
1412c50ec: add rsp, 0x28
1412c50f0: pop rsi
1412c50f1: pop rbp
1412c50f2: ret 

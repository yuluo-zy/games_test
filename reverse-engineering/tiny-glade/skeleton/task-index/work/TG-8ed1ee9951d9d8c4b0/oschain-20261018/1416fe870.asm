1416fe870: push rbp
1416fe871: push rsi
1416fe872: push rdi
1416fe873: push rbx
1416fe874: sub rsp, 0x218
1416fe87b: lea rbp, [rsp + 0x80]
1416fe883: mov qword ptr [rbp + 0x190], 0xfffffffffffffffe
1416fe88e: mov rsi, rcx
1416fe891: xorps xmm0, xmm0
1416fe894: movups xmmword ptr [rbp + 0x70], xmm0
1416fe898: movups xmmword ptr [rbp + 0x88], xmm0
1416fe89f: movups xmmword ptr [rbp + 0xa0], xmm0
1416fe8a6: movups xmmword ptr [rbp + 0xb8], xmm0
1416fe8ad: movups xmmword ptr [rbp + 0xd0], xmm0
1416fe8b4: mov dword ptr [rbp + 0xe0], 0
1416fe8be: movups xmmword ptr [rbp + 0x108], xmm0
1416fe8c5: movups xmmword ptr [rbp + 0x120], xmm0
1416fe8cc: movups xmmword ptr [rbp + 0x138], xmm0
1416fe8d3: movups xmmword ptr [rbp + 0x150], xmm0
1416fe8da: movups xmmword ptr [rbp + 0x168], xmm0
1416fe8e1: mov dword ptr [rbp + 0x178], 0
1416fe8eb: mov qword ptr [rbp + 0x18], 0
1416fe8f3: mov qword ptr [rbp + 0x50], 0
1416fe8fb: mov qword ptr [rbp + 0x58], 8
1416fe903: mov qword ptr [rbp + 0x60], 0
1416fe90b: mov qword ptr [rbp + 0x68], 0x10
1416fe913: mov qword ptr [rbp + 0x80], 0x10
1416fe91e: mov qword ptr [rbp + 0x98], 0x10
1416fe929: mov qword ptr [rbp + 0xb0], 0x10
1416fe934: mov qword ptr [rbp + 0xc8], 0x10
1416fe93f: movabs rax, 0x8000000000000000
1416fe949: mov qword ptr [rbp + 0xe8], rax
1416fe950: lea rax, [rip + 0x1594526]
1416fe957: mov qword ptr [rbp + 0xf0], rax
1416fe95e: mov qword ptr [rbp + 0xf8], 0x33
1416fe969: mov qword ptr [rbp + 0x100], 0x10
1416fe974: mov qword ptr [rbp + 0x118], 0x10
1416fe97f: mov qword ptr [rbp + 0x130], 0x10
1416fe98a: mov qword ptr [rbp + 0x148], 0x10
1416fe995: mov qword ptr [rbp + 0x160], 0x10
1416fe9a0: mov dword ptr [rbp + 0x180], 0
1416fe9aa: mov word ptr [rbp + 0x184], 1
1416fe9b3: mov dword ptr [rbp + 0x188], 0
1416fe9bd: movzx eax, byte ptr [rip + 0x1f36245]
1416fe9c4: mov ecx, 0x178
1416fe9c9: mov edx, 8
1416fe9ce: call 0x140613c10
1416fe9d3: test rax, rax
1416fe9d6: je 0x1416fea1d
1416fe9d8: mov rdi, rax
1416fe9db: lea rdx, [rbp + 0x18]
1416fe9df: mov r8d, 0x178
1416fe9e5: mov rcx, rax
1416fe9e8: call 0x142923c00
1416fe9ed: lea r8, [rip + 0x1594244]
1416fe9f4: lea rbx, [rbp - 0x58]
1416fe9f8: mov rcx, rbx
1416fe9fb: mov rdx, rdi
1416fe9fe: call 0x140ce4b10
1416fea03: mov rcx, rsi
1416fea06: mov rdx, rbx
1416fea09: call 0x14144f660
1416fea0e: mov rax, rsi
1416fea11: add rsp, 0x218
1416fea18: pop rbx
1416fea19: pop rdi
1416fea1a: pop rsi
1416fea1b: pop rbp
1416fea1c: ret 
1416fea1d: mov ecx, 8
1416fea22: mov edx, 0x178
1416fea27: call 0x1428d9000
1416fea2c: ud2 
1416fea2e: nop 
1416fea30: mov qword ptr [rsp + 0x10], rdx
1416fea35: push rbp
1416fea36: push rsi
1416fea37: push rdi
1416fea38: push rbx
1416fea39: sub rsp, 0x28
1416fea3d: lea rbp, [rdx + 0x80]
1416fea44: lea rcx, [rbp + 0x18]
1416fea48: call 0x1405e9af0
1416fea4d: nop 
1416fea4e: add rsp, 0x28
1416fea52: pop rbx
1416fea53: pop rdi
1416fea54: pop rsi
1416fea55: pop rbp
1416fea56: ret 
1416fea57: int3 
1416fea58: int3 
1416fea59: int3 
1416fea5a: int3 
1416fea5b: int3 
1416fea5c: int3 
1416fea5d: int3 
1416fea5e: int3 

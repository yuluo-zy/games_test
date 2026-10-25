1403429a0: push rbp
1403429a1: push rsi
1403429a2: push rdi
1403429a3: push rbx
1403429a4: sub rsp, 0x208
1403429ab: lea rbp, [rsp + 0x80]
1403429b3: mov qword ptr [rbp + 0x180], 0xfffffffffffffffe
1403429be: mov ebx, edx
1403429c0: mov rsi, rcx
1403429c3: xorps xmm0, xmm0
1403429c6: movups xmmword ptr [rbp + 0x60], xmm0
1403429ca: movups xmmword ptr [rbp + 0x78], xmm0
1403429ce: movups xmmword ptr [rbp + 0x90], xmm0
1403429d5: movups xmmword ptr [rbp + 0xa8], xmm0
1403429dc: movups xmmword ptr [rbp + 0xc0], xmm0
1403429e3: mov dword ptr [rbp + 0xd0], 0
1403429ed: movups xmmword ptr [rbp + 0xf8], xmm0
1403429f4: movups xmmword ptr [rbp + 0x110], xmm0
1403429fb: movups xmmword ptr [rbp + 0x128], xmm0
140342a02: movups xmmword ptr [rbp + 0x140], xmm0
140342a09: movups xmmword ptr [rbp + 0x158], xmm0
140342a10: mov dword ptr [rbp + 0x168], 0
140342a1a: mov qword ptr [rbp + 0x18], 0
140342a22: mov qword ptr [rbp + 0x40], 0
140342a2a: mov qword ptr [rbp + 0x48], 8
140342a32: mov qword ptr [rbp + 0x50], 0
140342a3a: mov qword ptr [rbp + 0x58], 0x10
140342a42: mov qword ptr [rbp + 0x70], 0x10
140342a4a: mov qword ptr [rbp + 0x88], 0x10
140342a55: mov qword ptr [rbp + 0xa0], 0x10
140342a60: mov qword ptr [rbp + 0xb8], 0x10
140342a6b: movabs rax, 0x8000000000000000
140342a75: mov qword ptr [rbp + 0xd8], rax
140342a7c: lea rax, [rip + 0x268037a]
140342a83: mov qword ptr [rbp + 0xe0], rax
140342a8a: mov qword ptr [rbp + 0xe8], 0x35
140342a95: mov qword ptr [rbp + 0xf0], 0x10
140342aa0: mov qword ptr [rbp + 0x108], 0x10
140342aab: mov qword ptr [rbp + 0x120], 0x10
140342ab6: mov qword ptr [rbp + 0x138], 0x10
140342ac1: mov qword ptr [rbp + 0x150], 0x10
140342acc: mov dword ptr [rbp + 0x170], 0
140342ad6: mov word ptr [rbp + 0x174], 1
140342adf: mov dword ptr [rbp + 0x178], 0
140342ae9: movzx eax, byte ptr [rip + 0x32f2119]
140342af0: mov ecx, 0x168
140342af5: mov edx, 8
140342afa: call 0x140613c10
140342aff: test rax, rax
140342b02: je 0x140342b9f
140342b08: mov rdi, rax
140342b0b: lea rdx, [rbp + 0x18]
140342b0f: mov r8d, 0x168
140342b15: mov rcx, rax
140342b18: call 0x142923c00
140342b1d: lea r8, [rip + 0x26801d4]
140342b24: lea rcx, [rbp - 0x58]
140342b28: mov rdx, rdi
140342b2b: call 0x140ce4b10
140342b30: mov byte ptr [rbp + 0x18], bl
140342b33: mov rcx, qword ptr [rip + 0x32e9836]
140342b3a: lea r8, [rip + 0x25e8457]
140342b41: lea rdx, [rbp + 0x18]
140342b45: call 0x140364d20
140342b4a: mov r8, rdx
140342b4d: lea rcx, [rbp - 0x58]
140342b51: mov rdx, rax
140342b54: call 0x1400d68c0
140342b59: movups xmm0, xmmword ptr [rbp + 8]
140342b5d: movups xmmword ptr [rsi + 0x60], xmm0
140342b61: movups xmm0, xmmword ptr [rbp - 8]
140342b65: movups xmmword ptr [rsi + 0x50], xmm0
140342b69: movups xmm0, xmmword ptr [rbp - 0x18]
140342b6d: movups xmmword ptr [rsi + 0x40], xmm0
140342b71: movups xmm0, xmmword ptr [rbp - 0x58]
140342b75: movups xmm1, xmmword ptr [rbp - 0x48]
140342b79: movups xmm2, xmmword ptr [rbp - 0x38]
140342b7d: movups xmm3, xmmword ptr [rbp - 0x28]
140342b81: movups xmmword ptr [rsi + 0x30], xmm3
140342b85: movups xmmword ptr [rsi + 0x20], xmm2
140342b89: movups xmmword ptr [rsi + 0x10], xmm1
140342b8d: movups xmmword ptr [rsi], xmm0
140342b90: mov rax, rsi
140342b93: add rsp, 0x208
140342b9a: pop rbx
140342b9b: pop rdi
140342b9c: pop rsi
140342b9d: pop rbp
140342b9e: ret 
140342b9f: mov ecx, 8
140342ba4: mov edx, 0x168
140342ba9: call 0x1428d9000
140342bae: ud2 
140342bb0: mov qword ptr [rsp + 0x10], rdx
140342bb5: push rbp
140342bb6: push rsi
140342bb7: push rdi
140342bb8: push rbx
140342bb9: sub rsp, 0x28
140342bbd: lea rbp, [rdx + 0x80]
140342bc4: lea rcx, [rbp + 0x18]
140342bc8: call 0x1405e6f10
140342bcd: nop 
140342bce: add rsp, 0x28
140342bd2: pop rbx
140342bd3: pop rdi
140342bd4: pop rsi
140342bd5: pop rbp
140342bd6: ret 
140342bd7: nop word ptr [rax + rax]
140342be0: mov qword ptr [rsp + 0x10], rdx
140342be5: push rbp
140342be6: push rsi
140342be7: push rdi
140342be8: push rbx
140342be9: sub rsp, 0x28
140342bed: lea rbp, [rdx + 0x80]
140342bf4: lea rcx, [rbp - 0x58]
140342bf8: call 0x140029880
140342bfd: nop 
140342bfe: add rsp, 0x28
140342c02: pop rbx
140342c03: pop rdi
140342c04: pop rsi
140342c05: pop rbp
140342c06: ret 

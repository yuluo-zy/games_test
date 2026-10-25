141750ab0: push rbp
141750ab1: push rsi
141750ab2: push rdi
141750ab3: push rbx
141750ab4: sub rsp, 0x658
141750abb: lea rbp, [rsp + 0x80]
141750ac3: mov qword ptr [rbp + 0x5d0], 0xfffffffffffffffe
141750ace: mov rsi, rcx
141750ad1: xorps xmm0, xmm0
141750ad4: movups xmmword ptr [rbp + 0x38], xmm0
141750ad8: movups xmmword ptr [rbp + 0x50], xmm0
141750adc: movups xmmword ptr [rbp + 0x68], xmm0
141750ae0: movups xmmword ptr [rbp + 0x80], xmm0
141750ae7: movups xmmword ptr [rbp + 0x98], xmm0
141750aee: mov dword ptr [rbp + 0xa8], 0
141750af8: movups xmmword ptr [rbp + 0xd0], xmm0
141750aff: movups xmmword ptr [rbp + 0xe8], xmm0
141750b06: movups xmmword ptr [rbp + 0x100], xmm0
141750b0d: movups xmmword ptr [rbp + 0x118], xmm0
141750b14: movups xmmword ptr [rbp + 0x130], xmm0
141750b1b: mov dword ptr [rbp + 0x140], 0
141750b25: mov qword ptr [rbp + 0x18], 0
141750b2d: mov qword ptr [rbp + 0x20], 8
141750b35: mov qword ptr [rbp + 0x28], 0
141750b3d: mov qword ptr [rbp + 0x30], 0x10
141750b45: mov qword ptr [rbp + 0x48], 0x10
141750b4d: mov qword ptr [rbp + 0x60], 0x10
141750b55: mov qword ptr [rbp + 0x78], 0x10
141750b5d: mov qword ptr [rbp + 0x90], 0x10
141750b68: movabs rax, 0x8000000000000000
141750b72: mov qword ptr [rbp + 0xb0], rax
141750b79: lea rcx, [rip + 0x15830fd]
141750b80: mov qword ptr [rbp + 0xb8], rcx
141750b87: mov qword ptr [rbp + 0xc0], 0x31
141750b92: mov qword ptr [rbp + 0xc8], 0x10
141750b9d: mov qword ptr [rbp + 0xe0], 0x10
141750ba8: mov qword ptr [rbp + 0xf8], 0x10
141750bb3: mov qword ptr [rbp + 0x110], 0x10
141750bbe: mov qword ptr [rbp + 0x128], 0x10
141750bc9: mov dword ptr [rbp + 0x148], 0
141750bd3: mov word ptr [rbp + 0x14c], 1
141750bdc: mov qword ptr [rbp + 0x240], rax
141750be3: mov dword ptr [rbp + 0x5c8], 0
141750bed: movzx eax, byte ptr [rip + 0x1ee4015]
141750bf4: mov ecx, 0x5b8
141750bf9: mov edx, 8
141750bfe: call 0x140613c10
141750c03: test rax, rax
141750c06: je 0x141750c4d
141750c08: mov rdi, rax
141750c0b: lea rdx, [rbp + 0x18]
141750c0f: mov r8d, 0x5b8
141750c15: mov rcx, rax
141750c18: call 0x142923c00
141750c1d: lea r8, [rip + 0x1582d94]
141750c24: lea rbx, [rbp - 0x58]
141750c28: mov rcx, rbx
141750c2b: mov rdx, rdi
141750c2e: call 0x140ce4b10
141750c33: mov rcx, rsi
141750c36: mov rdx, rbx
141750c39: call 0x14144fae0
141750c3e: mov rax, rsi
141750c41: add rsp, 0x658
141750c48: pop rbx
141750c49: pop rdi
141750c4a: pop rsi
141750c4b: pop rbp
141750c4c: ret 
141750c4d: mov ecx, 8
141750c52: mov edx, 0x5b8
141750c57: call 0x1428d9000
141750c5c: ud2 
141750c5e: nop 
141750c60: mov qword ptr [rsp + 0x10], rdx
141750c65: push rbp
141750c66: push rsi
141750c67: push rdi
141750c68: push rbx
141750c69: sub rsp, 0x28
141750c6d: lea rbp, [rdx + 0x80]
141750c74: lea rcx, [rbp + 0x18]
141750c78: call 0x140886850
141750c7d: nop 
141750c7e: add rsp, 0x28
141750c82: pop rbx
141750c83: pop rdi
141750c84: pop rsi
141750c85: pop rbp
141750c86: ret 

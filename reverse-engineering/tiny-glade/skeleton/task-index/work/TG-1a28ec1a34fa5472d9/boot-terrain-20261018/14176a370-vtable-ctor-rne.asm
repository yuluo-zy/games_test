14176a370: push rbp
14176a371: push rsi
14176a372: push rdi
14176a373: sub rsp, 0x1e0
14176a37a: lea rbp, [rsp + 0x80]
14176a382: mov qword ptr [rbp + 0x158], 0xfffffffffffffffe
14176a38d: mov rsi, rcx
14176a390: xorps xmm0, xmm0
14176a393: movups xmmword ptr [rbp - 0x38], xmm0
14176a397: movups xmmword ptr [rbp - 0x20], xmm0
14176a39b: movups xmmword ptr [rbp - 8], xmm0
14176a39f: movups xmmword ptr [rbp + 0x10], xmm0
14176a3a3: movups xmmword ptr [rbp + 0x28], xmm0
14176a3a7: mov dword ptr [rbp + 0x38], 0
14176a3ae: movups xmmword ptr [rbp + 0x60], xmm0
14176a3b2: movups xmmword ptr [rbp + 0x78], xmm0
14176a3b6: movups xmmword ptr [rbp + 0x90], xmm0
14176a3bd: movups xmmword ptr [rbp + 0xa8], xmm0
14176a3c4: movups xmmword ptr [rbp + 0xc0], xmm0
14176a3cb: mov dword ptr [rbp + 0xd0], 0
14176a3d5: mov qword ptr [rbp - 0x58], 0
14176a3dd: mov qword ptr [rbp - 0x50], 8
14176a3e5: mov qword ptr [rbp - 0x48], 0
14176a3ed: mov qword ptr [rbp - 0x40], 0x10
14176a3f5: mov qword ptr [rbp - 0x28], 0x10
14176a3fd: mov qword ptr [rbp - 0x10], 0x10
14176a405: mov qword ptr [rbp + 8], 0x10
14176a40d: mov qword ptr [rbp + 0x20], 0x10
14176a415: movabs rax, 0x8000000000000000
14176a41f: mov qword ptr [rbp + 0x40], rax
14176a423: lea rcx, [rip + 0x156d987]
14176a42a: mov qword ptr [rbp + 0x48], rcx
14176a42e: mov qword ptr [rbp + 0x50], 0x37
14176a436: mov qword ptr [rbp + 0x58], 0x10
14176a43e: mov qword ptr [rbp + 0x70], 0x10
14176a446: mov qword ptr [rbp + 0x88], 0x10
14176a451: mov qword ptr [rbp + 0xa0], 0x10
14176a45c: mov qword ptr [rbp + 0xb8], 0x10
14176a467: mov dword ptr [rbp + 0xd8], 0
14176a471: mov word ptr [rbp + 0xdc], 1
14176a47a: mov qword ptr [rbp + 0x110], rax
14176a481: mov dword ptr [rbp + 0x150], 0
14176a48b: movzx eax, byte ptr [rip + 0x1eca777]
14176a492: mov ecx, 0x1b0
14176a497: mov edx, 8
14176a49c: call 0x140613c10
14176a4a1: test rax, rax
14176a4a4: je 0x14176a4db
14176a4a6: mov rdi, rax
14176a4a9: lea rdx, [rbp - 0x58]
14176a4ad: mov r8d, 0x1b0
14176a4b3: mov rcx, rax
14176a4b6: call 0x142923c00
14176a4bb: lea r8, [rip + 0x156d7de]
14176a4c2: mov rcx, rsi
14176a4c5: mov rdx, rdi
14176a4c8: call 0x140ce4b10
14176a4cd: mov rax, rsi
14176a4d0: add rsp, 0x1e0
14176a4d7: pop rdi
14176a4d8: pop rsi
14176a4d9: pop rbp
14176a4da: ret 
14176a4db: mov ecx, 8
14176a4e0: mov edx, 0x1b0
14176a4e5: call 0x1428d9000
14176a4ea: ud2 
14176a4ec: nop dword ptr [rax]
14176a4f0: mov qword ptr [rsp + 0x10], rdx
14176a4f5: push rbp
14176a4f6: push rsi
14176a4f7: push rdi
14176a4f8: sub rsp, 0x20
14176a4fc: lea rbp, [rdx + 0x80]
14176a503: lea rcx, [rbp - 0x58]
14176a507: call 0x140893990
14176a50c: nop 
14176a50d: add rsp, 0x20
14176a511: pop rdi
14176a512: pop rsi
14176a513: pop rbp
14176a514: ret 

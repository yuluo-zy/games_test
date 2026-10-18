140945ed0: push rsi
140945ed1: sub rsp, 0x90
140945ed8: mov rsi, rcx
140945edb: lea rcx, [rsp + 0x28]
140945ee0: lea r8, [rip + 0x216c551]
140945ee7: xor edx, edx
140945ee9: call 0x14130f350
140945eee: mov qword ptr [rsp + 0x40], 0
140945ef7: mov qword ptr [rsp + 0x48], 8
140945f00: mov qword ptr [rsp + 0x50], 0
140945f09: movups xmm0, xmmword ptr [rip + 0x216c5e0]
140945f10: movups xmmword ptr [rsp + 0x58], xmm0
140945f15: movups xmm0, xmmword ptr [rip + 0x216c5e4]
140945f1c: movups xmmword ptr [rsp + 0x68], xmm0
140945f21: movabs rax, 0x8000000000000000
140945f2b: mov qword ptr [rsp + 0x20], rax
140945f30: lea r8, [rip + 0x216cbd9]
140945f37: lea rdx, [rsp + 0x20]
140945f3c: mov rcx, rsi
140945f3f: call 0x140b68190
140945f44: mov rax, rsi
140945f47: add rsp, 0x90
140945f4e: pop rsi
140945f4f: ret 

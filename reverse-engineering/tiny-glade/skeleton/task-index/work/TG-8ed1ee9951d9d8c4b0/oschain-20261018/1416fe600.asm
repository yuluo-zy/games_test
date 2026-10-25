1416fe600: push rbp
1416fe601: push rsi
1416fe602: push rdi
1416fe603: push rbx
1416fe604: sub rsp, 0x208
1416fe60b: lea rbp, [rsp + 0x80]
1416fe613: mov qword ptr [rbp + 0x180], 0xfffffffffffffffe
1416fe61e: mov ebx, edx
1416fe620: mov rsi, rcx
1416fe623: xorps xmm0, xmm0
1416fe626: movups xmmword ptr [rbp + 0x60], xmm0
1416fe62a: movups xmmword ptr [rbp + 0x78], xmm0
1416fe62e: movups xmmword ptr [rbp + 0x90], xmm0
1416fe635: movups xmmword ptr [rbp + 0xa8], xmm0
1416fe63c: movups xmmword ptr [rbp + 0xc0], xmm0
1416fe643: mov dword ptr [rbp + 0xd0], 0
1416fe64d: movups xmmword ptr [rbp + 0xf8], xmm0
1416fe654: movups xmmword ptr [rbp + 0x110], xmm0
1416fe65b: movups xmmword ptr [rbp + 0x128], xmm0
1416fe662: movups xmmword ptr [rbp + 0x140], xmm0
1416fe669: movups xmmword ptr [rbp + 0x158], xmm0
1416fe670: mov dword ptr [rbp + 0x168], 0
1416fe67a: mov qword ptr [rbp + 0x18], 0
1416fe682: mov qword ptr [rbp + 0x40], 0
1416fe68a: mov qword ptr [rbp + 0x48], 8
1416fe692: mov qword ptr [rbp + 0x50], 0
1416fe69a: mov qword ptr [rbp + 0x58], 0x10
1416fe6a2: mov qword ptr [rbp + 0x70], 0x10
1416fe6aa: mov qword ptr [rbp + 0x88], 0x10
1416fe6b5: mov qword ptr [rbp + 0xa0], 0x10
1416fe6c0: mov qword ptr [rbp + 0xb8], 0x10
1416fe6cb: movabs rax, 0x8000000000000000
1416fe6d5: mov qword ptr [rbp + 0xd8], rax
1416fe6dc: lea rax, [rip + 0x159480a]
1416fe6e3: mov qword ptr [rbp + 0xe0], rax
1416fe6ea: mov qword ptr [rbp + 0xe8], 0x30
1416fe6f5: mov qword ptr [rbp + 0xf0], 0x10
1416fe700: mov qword ptr [rbp + 0x108], 0x10
1416fe70b: mov qword ptr [rbp + 0x120], 0x10
1416fe716: mov qword ptr [rbp + 0x138], 0x10
1416fe721: mov qword ptr [rbp + 0x150], 0x10
1416fe72c: mov dword ptr [rbp + 0x170], 0
1416fe736: mov word ptr [rbp + 0x174], 1
1416fe73f: mov dword ptr [rbp + 0x178], 0
1416fe749: movzx eax, byte ptr [rip + 0x1f364b9]
1416fe750: mov ecx, 0x168
1416fe755: mov edx, 8
1416fe75a: call 0x140613c10
1416fe75f: test rax, rax
1416fe762: je 0x1416fe7ff
1416fe768: mov rdi, rax
1416fe76b: lea rdx, [rbp + 0x18]
1416fe76f: mov r8d, 0x168
1416fe775: mov rcx, rax
1416fe778: call 0x142923c00
1416fe77d: lea r8, [rip + 0x159456c]
1416fe784: lea rcx, [rbp - 0x58]
1416fe788: mov rdx, rdi
1416fe78b: call 0x140ce4b10
1416fe790: mov byte ptr [rbp + 0x18], bl
1416fe793: mov rcx, qword ptr [rip + 0x1f2dbd6]
1416fe79a: lea r8, [rip + 0x15615bf]
1416fe7a1: lea rdx, [rbp + 0x18]
1416fe7a5: call 0x140b218a0
1416fe7aa: mov r8, rdx
1416fe7ad: lea rcx, [rbp - 0x58]
1416fe7b1: mov rdx, rax
1416fe7b4: call 0x141453000
1416fe7b9: movups xmm0, xmmword ptr [rbp + 8]
1416fe7bd: movups xmmword ptr [rsi + 0x60], xmm0
1416fe7c1: movups xmm0, xmmword ptr [rbp - 8]
1416fe7c5: movups xmmword ptr [rsi + 0x50], xmm0
1416fe7c9: movups xmm0, xmmword ptr [rbp - 0x18]
1416fe7cd: movups xmmword ptr [rsi + 0x40], xmm0
1416fe7d1: movups xmm0, xmmword ptr [rbp - 0x58]
1416fe7d5: movups xmm1, xmmword ptr [rbp - 0x48]
1416fe7d9: movups xmm2, xmmword ptr [rbp - 0x38]
1416fe7dd: movups xmm3, xmmword ptr [rbp - 0x28]
1416fe7e1: movups xmmword ptr [rsi + 0x30], xmm3
1416fe7e5: movups xmmword ptr [rsi + 0x20], xmm2
1416fe7e9: movups xmmword ptr [rsi + 0x10], xmm1
1416fe7ed: movups xmmword ptr [rsi], xmm0
1416fe7f0: mov rax, rsi
1416fe7f3: add rsp, 0x208
1416fe7fa: pop rbx
1416fe7fb: pop rdi
1416fe7fc: pop rsi
1416fe7fd: pop rbp
1416fe7fe: ret 
1416fe7ff: mov ecx, 8
1416fe804: mov edx, 0x168
1416fe809: call 0x1428d9000
1416fe80e: ud2 
1416fe810: mov qword ptr [rsp + 0x10], rdx
1416fe815: push rbp
1416fe816: push rsi
1416fe817: push rdi
1416fe818: push rbx
1416fe819: sub rsp, 0x28
1416fe81d: lea rbp, [rdx + 0x80]
1416fe824: lea rcx, [rbp + 0x18]
1416fe828: call 0x1405e6f10
1416fe82d: nop 
1416fe82e: add rsp, 0x28
1416fe832: pop rbx
1416fe833: pop rdi
1416fe834: pop rsi
1416fe835: pop rbp
1416fe836: ret 
1416fe837: nop word ptr [rax + rax]
1416fe840: mov qword ptr [rsp + 0x10], rdx
1416fe845: push rbp
1416fe846: push rsi
1416fe847: push rdi
1416fe848: push rbx
1416fe849: sub rsp, 0x28
1416fe84d: lea rbp, [rdx + 0x80]
1416fe854: lea rcx, [rbp - 0x58]
1416fe858: call 0x140029880
1416fe85d: nop 
1416fe85e: add rsp, 0x28
1416fe862: pop rbx
1416fe863: pop rdi
1416fe864: pop rsi
1416fe865: pop rbp
1416fe866: ret 
1416fe867: int3 
1416fe868: int3 
1416fe869: int3 
1416fe86a: int3 
1416fe86b: int3 

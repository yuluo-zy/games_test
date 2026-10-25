14161a760: push rbp
14161a761: push r15
14161a763: push r14
14161a765: push r12
14161a767: push rsi
14161a768: push rdi
14161a769: push rbx
14161a76a: sub rsp, 0x100
14161a771: lea rbp, [rsp + 0x80]
14161a779: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
14161a781: mov rsi, rcx
14161a784: test r8, r8
14161a787: sete al
14161a78a: cmp r9, 0x2bf3
14161a791: setb cl
14161a794: or cl, al
14161a796: je 0x14161a837
14161a79c: mov r14, r9
14161a79f: mov rbx, r8
14161a7a2: mov rdi, rdx
14161a7a5: lea r15, [rbp + 0x10]
14161a7a9: mov edx, 0x83d60
14161a7ae: mov rcx, r15
14161a7b1: mov r8d, 0x22
14161a7b7: mov r9b, 1
14161a7ba: call 0x140bd4de0
14161a7bf: lea r12, [rbp + 0x68]
14161a7c3: mov r8d, 4
14161a7c9: mov rcx, r12
14161a7cc: mov rdx, r15
14161a7cf: call 0x140bd4f20
14161a7d4: shl r14, 4
14161a7d8: lea rax, [r14 + r14*2]
14161a7dc: mov qword ptr [rsp + 0x30], rax
14161a7e1: mov qword ptr [rsp + 0x28], rbx
14161a7e6: mov qword ptr [rsp + 0x20], r12
14161a7eb: lea r8, [rip + 0x1665959]
14161a7f2: lea rcx, [rbp - 0x40]
14161a7f6: mov r9d, 0xc
14161a7fc: mov rdx, rdi
14161a7ff: call 0x1416a2dc0
14161a804: cmp dword ptr [rbp - 0x40], 0xb
14161a808: jne 0x14161a898
14161a80e: mov rax, qword ptr [rbp - 0x18]
14161a812: mov qword ptr [rsi + 0x20], rax
14161a816: movups xmm0, xmmword ptr [rbp - 0x38]
14161a81a: movups xmm1, xmmword ptr [rbp - 0x28]
14161a81e: movups xmmword ptr [rsi + 0x10], xmm1
14161a822: movups xmmword ptr [rsi], xmm0
14161a825: add rsp, 0x100
14161a82c: pop rbx
14161a82d: pop rdi
14161a82e: pop rsi
14161a82f: pop r12
14161a831: pop r14
14161a833: pop r15
14161a835: pop rbp
14161a836: ret 
14161a837: lea rax, [rip + 0x1484028]
14161a83e: mov qword ptr [rbp - 0x40], rax
14161a842: mov qword ptr [rbp - 0x38], 0x37
14161a84a: lea rax, [rbp - 0x40]
14161a84e: mov qword ptr [rbp + 0x68], rax
14161a852: lea rax, [rip - 0x15109c9]
14161a859: mov qword ptr [rbp + 0x70], rax
14161a85d: lea rax, [rip + 0x1664f64]
14161a864: mov qword ptr [rbp + 0x10], rax
14161a868: mov qword ptr [rbp + 0x18], 1
14161a870: mov qword ptr [rbp + 0x30], 0
14161a878: lea rax, [rbp + 0x68]
14161a87c: mov qword ptr [rbp + 0x20], rax
14161a880: mov qword ptr [rbp + 0x28], 1
14161a888: lea rdx, [rip + 0x1664f49]
14161a88f: lea rcx, [rbp + 0x10]
14161a893: call 0x1428d9390
14161a898: movups xmm0, xmmword ptr [rbp]
14161a89c: movaps xmmword ptr [rbp + 0x50], xmm0
14161a8a0: movups xmm0, xmmword ptr [rbp - 0x40]
14161a8a4: movups xmm1, xmmword ptr [rbp - 0x30]
14161a8a8: movups xmm2, xmmword ptr [rbp - 0x20]
14161a8ac: movups xmm3, xmmword ptr [rbp - 0x10]
14161a8b0: movaps xmmword ptr [rbp + 0x40], xmm3
14161a8b4: movaps xmmword ptr [rbp + 0x30], xmm2
14161a8b8: movaps xmmword ptr [rbp + 0x20], xmm1
14161a8bc: movaps xmmword ptr [rbp + 0x10], xmm0
14161a8c0: lea rax, [rip + 0x16658e1]
14161a8c7: mov qword ptr [rsp + 0x20], rax
14161a8cc: lea rcx, [rip + 0x1664215]
14161a8d3: lea r9, [rip + 0x16641ee]
14161a8da: lea r8, [rbp + 0x10]
14161a8de: mov edx, 0x2b
14161a8e3: call 0x1428d9760
14161a8e8: ud2 
14161a8ea: nop word ptr [rax + rax]
14161a8f0: mov qword ptr [rsp + 0x10], rdx
14161a8f5: push rbp
14161a8f6: push r15
14161a8f8: push r14
14161a8fa: push r12
14161a8fc: push rsi
14161a8fd: push rdi
14161a8fe: push rbx
14161a8ff: sub rsp, 0x40
14161a903: lea rbp, [rdx + 0x80]
14161a90a: lea rcx, [rbp + 0x10]
14161a90e: call 0x1407cea70
14161a913: nop 
14161a914: add rsp, 0x40
14161a918: pop rbx
14161a919: pop rdi
14161a91a: pop rsi
14161a91b: pop r12
14161a91d: pop r14
14161a91f: pop r15
14161a921: pop rbp
14161a922: ret 

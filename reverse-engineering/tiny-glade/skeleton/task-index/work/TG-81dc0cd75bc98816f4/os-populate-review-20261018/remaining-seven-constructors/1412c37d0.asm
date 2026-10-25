1412c37d0: push rbp
1412c37d1: push rsi
1412c37d2: sub rsp, 0xd8
1412c37d9: lea rbp, [rsp + 0x80]
1412c37e1: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c37e9: mov rsi, rcx
1412c37ec: lea rcx, [rbp - 0x58]
1412c37f0: xor r8d, r8d
1412c37f3: call 0x141612cb0
1412c37f8: movzx eax, byte ptr [rip + 0x237140a]
1412c37ff: mov ecx, 0x28
1412c3804: mov edx, 8
1412c3809: call 0x140613c10
1412c380e: test rax, rax
1412c3811: je 0x1412c38c7
1412c3817: mov rcx, qword ptr [rbp - 0x38]
1412c381b: mov qword ptr [rax + 0x20], rcx
1412c381f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c3823: movups xmm1, xmmword ptr [rbp - 0x48]
1412c3827: movups xmmword ptr [rax + 0x10], xmm1
1412c382b: movups xmmword ptr [rax], xmm0
1412c382e: lea rcx, [rip + 0x17db38f]
1412c3835: mov qword ptr [rbp], rcx
1412c3839: mov qword ptr [rbp + 8], 0x4f
1412c3841: mov qword ptr [rbp + 0x10], rax
1412c3845: lea rax, [rip + 0x196977c]
1412c384c: mov qword ptr [rbp + 0x18], rax
1412c3850: mov qword ptr [rbp + 0x30], 0
1412c3858: mov byte ptr [rbp + 0x46], 5
1412c385c: mov qword ptr [rbp + 0x20], rcx
1412c3860: mov qword ptr [rbp + 0x28], 0x4f
1412c3868: movzx eax, byte ptr [rip + 0x237139a]
1412c386f: mov ecx, 0x40
1412c3874: mov edx, 8
1412c3879: call 0x140613c10
1412c387e: test rax, rax
1412c3881: je 0x1412c38d8
1412c3883: movups xmm0, xmmword ptr [rbp + 0x10]
1412c3887: movups xmm1, xmmword ptr [rbp + 0x20]
1412c388b: movups xmm2, xmmword ptr [rbp + 0x30]
1412c388f: movups xmm3, xmmword ptr [rbp + 0x40]
1412c3893: movups xmmword ptr [rax + 0x30], xmm3
1412c3897: movups xmmword ptr [rax + 0x20], xmm2
1412c389b: movups xmmword ptr [rax + 0x10], xmm1
1412c389f: movups xmmword ptr [rax], xmm0
1412c38a2: movabs rdx, 0xc405372d6efc32d3
1412c38ac: mov rcx, rsi
1412c38af: mov r8, rax
1412c38b2: call 0x140969bf0
1412c38b7: cmp rax, 1
1412c38bb: je 0x1412c38e9
1412c38bd: add rsp, 0xd8
1412c38c4: pop rsi
1412c38c5: pop rbp
1412c38c6: ret 
1412c38c7: mov ecx, 8
1412c38cc: mov edx, 0x28
1412c38d1: call 0x1428d9000
1412c38d6: jmp 0x1412c38e7
1412c38d8: mov ecx, 8
1412c38dd: mov edx, 0x40
1412c38e2: call 0x1428d9000
1412c38e7: ud2 
1412c38e9: mov rax, rdx
1412c38ec: add rdx, 0x10
1412c38f0: mov rsi, rdx
1412c38f3: cmp qword ptr [rax + 0x18], 0x4f
1412c38f8: jne 0x1412c3976
1412c38fa: mov rdx, qword ptr [rsi]
1412c38fd: lea rcx, [rip + 0x17db2c0]
1412c3904: mov r8d, 0x4f
1412c390a: call 0x142923bf0
1412c390f: test eax, eax
1412c3911: jne 0x1412c3976
1412c3913: lea rax, [rip + 0x1969f7e]
1412c391a: mov qword ptr [rbp - 0x30], rax
1412c391e: lea rax, [rip - 0xd90085]
1412c3925: mov qword ptr [rbp - 0x28], rax
1412c3929: mov rax, rbp
1412c392c: mov qword ptr [rbp - 0x20], rax
1412c3930: lea rax, [rip - 0x11b9aa7]
1412c3937: mov qword ptr [rbp - 0x18], rax
1412c393b: lea rax, [rip + 0x1969ce6]
1412c3942: mov qword ptr [rbp + 0x10], rax
1412c3946: mov qword ptr [rbp + 0x18], 3
1412c394e: mov qword ptr [rbp + 0x30], 0
1412c3956: lea rax, [rbp - 0x30]
1412c395a: mov qword ptr [rbp + 0x20], rax
1412c395e: mov qword ptr [rbp + 0x28], 2
1412c3966: lea rdx, [rip + 0x1969ceb]
1412c396d: lea rcx, [rbp + 0x10]
1412c3971: call 0x1428d9390
1412c3976: lea rax, [rip + 0x1969f1b]
1412c397d: mov qword ptr [rbp + 0x10], rax
1412c3981: lea rax, [rip - 0xd900e8]
1412c3988: mov qword ptr [rbp + 0x18], rax
1412c398c: mov qword ptr [rbp + 0x20], rsi
1412c3990: lea rax, [rip - 0x11b9b07]
1412c3997: mov qword ptr [rbp + 0x28], rax
1412c399b: mov rcx, rbp
1412c399e: mov qword ptr [rbp + 0x30], rcx
1412c39a2: mov qword ptr [rbp + 0x38], rax
1412c39a6: lea rax, [rip + 0x1969bbb]
1412c39ad: mov qword ptr [rbp - 0x30], rax
1412c39b1: mov qword ptr [rbp - 0x28], 3
1412c39b9: mov qword ptr [rbp - 0x10], 0
1412c39c1: lea rax, [rbp + 0x10]
1412c39c5: mov qword ptr [rbp - 0x20], rax
1412c39c9: mov qword ptr [rbp - 0x18], 3
1412c39d1: lea rdx, [rip + 0x1969bf8]
1412c39d8: lea rcx, [rbp - 0x30]
1412c39dc: call 0x1428d9390
1412c39e1: int3 
1412c39e2: nop word ptr cs:[rax + rax]
1412c39f0: mov qword ptr [rsp + 0x10], rdx
1412c39f5: push rbp
1412c39f6: push rsi
1412c39f7: sub rsp, 0x28
1412c39fb: lea rbp, [rdx + 0x80]
1412c3a02: lea rcx, [rbp - 0x58]
1412c3a06: call 0x140be24b0
1412c3a0b: nop 
1412c3a0c: add rsp, 0x28
1412c3a10: pop rsi
1412c3a11: pop rbp
1412c3a12: ret 
1412c3a13: nop word ptr cs:[rax + rax]
1412c3a20: mov qword ptr [rsp + 0x10], rdx
1412c3a25: push rbp
1412c3a26: push rsi
1412c3a27: sub rsp, 0x28
1412c3a2b: lea rbp, [rdx + 0x80]
1412c3a32: lea rcx, [rbp + 0x10]
1412c3a36: call 0x1400c04b0
1412c3a3b: nop 
1412c3a3c: add rsp, 0x28
1412c3a40: pop rsi
1412c3a41: pop rbp
1412c3a42: ret 

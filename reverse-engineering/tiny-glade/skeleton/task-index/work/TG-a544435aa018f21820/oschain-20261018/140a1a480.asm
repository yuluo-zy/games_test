140a1a480: push r15
140a1a482: push r14
140a1a484: push r13
140a1a486: push r12
140a1a488: push rsi
140a1a489: push rdi
140a1a48a: push rbx
140a1a48b: sub rsp, 0x70
140a1a48f: mov r12, qword ptr [rsp + 0xd8]
140a1a497: movabs r13, 0x8000000000000000
140a1a4a1: mov r15, qword ptr [r12]
140a1a4a5: lea rax, [r13 + 3]
140a1a4a9: cmp qword ptr [r15 + 0x40], rax
140a1a4ad: je 0x140a1a925
140a1a4b3: mov rbx, r8
140a1a4b6: mov rdi, rdx
140a1a4b9: mov rsi, rcx
140a1a4bc: mov rcx, qword ptr [r9]
140a1a4bf: lea rdx, [rip + 0x2213e82]
140a1a4c6: call 0x140969720
140a1a4cb: test rax, rax
140a1a4ce: je 0x140a1a962
140a1a4d4: mov r14, rax
140a1a4d7: lea rax, [rip + 0x2083998]
140a1a4de: mov qword ptr [rsp + 0x60], rax
140a1a4e3: mov qword ptr [rsp + 0x68], 0x47
140a1a4ec: cmp qword ptr [r14 + 0x18], 0x47
140a1a4f1: jne 0x140a1a935
140a1a4f7: mov rdx, qword ptr [r14 + 0x10]
140a1a4fb: lea rcx, [rip + 0x2083974]
140a1a502: mov r8d, 0x47
140a1a508: call 0x142923bf0
140a1a50d: test eax, eax
140a1a50f: jne 0x140a1a935
140a1a515: mov rcx, qword ptr [r14]
140a1a518: mov rax, qword ptr [r14 + 8]
140a1a51c: call qword ptr [rax + 0x18]
140a1a51f: test rax, rax
140a1a522: je 0x140a1a962
140a1a528: cmp qword ptr [rax + 8], 0
140a1a52d: je 0x140a1a96e
140a1a533: mov rcx, qword ptr [rsp + 0xd0]
140a1a53b: mov rdx, qword ptr [rax]
140a1a53e: movups xmm0, xmmword ptr [rdx]
140a1a541: movaps xmm1, xmm0
140a1a544: shufps xmm1, xmm0, 0xff
140a1a548: movd edx, xmm1
140a1a54c: and edx, 0x7fffffff
140a1a552: cmp edx, 0x7f7fffff
140a1a558: jg 0x140a1a56b
140a1a55a: mov edx, dword ptr [rcx + 0x1c]
140a1a55d: mov r8, qword ptr [rcx + 0x10]
140a1a561: mov dword ptr [r8], edx
140a1a564: mov rdx, qword ptr [rcx]
140a1a567: movss dword ptr [rdx], xmm1
140a1a56b: movd edx, xmm0
140a1a56f: movshdup xmm1, xmm0
140a1a573: movd r8d, xmm1
140a1a578: movdqa xmm1, xmm0
140a1a57c: punpckhqdq xmm1, xmm0
140a1a580: movd r9d, xmm1
140a1a585: and edx, 0x7fffffff
140a1a58b: cmp edx, 0x7f800000
140a1a591: setl dl
140a1a594: and r8d, 0x7fffffff
140a1a59b: cmp r8d, 0x7f800000
140a1a5a2: setl r8b
140a1a5a6: and r8b, dl
140a1a5a9: and r9d, 0x7fffffff
140a1a5b0: cmp r9d, 0x7f800000
140a1a5b7: setl dl
140a1a5ba: and dl, r8b
140a1a5bd: cmp dl, 1
140a1a5c0: jne 0x140a1a5d9
140a1a5c2: mov edx, dword ptr [rcx + 0x1c]
140a1a5c5: mov r8, qword ptr [rcx + 0x10]
140a1a5c9: mov dword ptr [r8], edx
140a1a5cc: mov rdx, qword ptr [rcx]
140a1a5cf: movlpd qword ptr [rdx + 8], xmm0
140a1a5d4: movd dword ptr [rdx + 0x10], xmm1
140a1a5d9: mov rdx, qword ptr [rax + 8]
140a1a5dd: cmp rdx, 2
140a1a5e1: jb 0x140a1a97e
140a1a5e7: mov r8, qword ptr [rax]
140a1a5ea: movdqu xmm0, xmmword ptr [r8 + 0x10]
140a1a5f0: movd r8d, xmm0
140a1a5f5: movshdup xmm1, xmm0
140a1a5f9: movd r9d, xmm1
140a1a5fe: movdqa xmm1, xmm0
140a1a602: punpckhqdq xmm1, xmm0
140a1a606: movd r10d, xmm1
140a1a60b: and r8d, 0x7fffffff
140a1a612: cmp r8d, 0x7f800000
140a1a619: setl r8b
140a1a61d: and r9d, 0x7fffffff
140a1a624: cmp r9d, 0x7f800000
140a1a62b: setl r9b
140a1a62f: and r9b, r8b
140a1a632: and r10d, 0x7fffffff
140a1a639: cmp r10d, 0x7f800000
140a1a640: setl r8b
140a1a644: test r9b, r8b
140a1a647: je 0x140a1a664
140a1a649: mov edx, dword ptr [rcx + 0x1c]
140a1a64c: mov r8, qword ptr [rcx + 0x10]
140a1a650: mov dword ptr [r8], edx
140a1a653: mov rdx, qword ptr [rcx]
140a1a656: movlpd qword ptr [rdx + 0x14], xmm0
140a1a65b: movd dword ptr [rdx + 0x1c], xmm1
140a1a660: mov rdx, qword ptr [rax + 8]
140a1a664: cmp rdx, 3
140a1a668: jb 0x140a1a98f
140a1a66e: mov r8, qword ptr [rax]
140a1a671: movdqu xmm0, xmmword ptr [r8 + 0x20]
140a1a677: movd r8d, xmm0
140a1a67c: movshdup xmm1, xmm0
140a1a680: movd r9d, xmm1
140a1a685: movdqa xmm1, xmm0
140a1a689: punpckhqdq xmm1, xmm0
140a1a68d: movd r10d, xmm1
140a1a692: and r8d, 0x7fffffff
140a1a699: cmp r8d, 0x7f800000
140a1a6a0: setl r8b
140a1a6a4: and r9d, 0x7fffffff
140a1a6ab: cmp r9d, 0x7f800000
140a1a6b2: setl r9b
140a1a6b6: and r9b, r8b
140a1a6b9: and r10d, 0x7fffffff
140a1a6c0: cmp r10d, 0x7f800000
140a1a6c7: setl r8b
140a1a6cb: test r9b, r8b
140a1a6ce: je 0x140a1a6eb
140a1a6d0: mov edx, dword ptr [rcx + 0x1c]
140a1a6d3: mov r8, qword ptr [rcx + 0x10]
140a1a6d7: mov dword ptr [r8], edx
140a1a6da: mov rdx, qword ptr [rcx]
140a1a6dd: movlpd qword ptr [rdx + 0x20], xmm0
140a1a6e2: movd dword ptr [rdx + 0x28], xmm1
140a1a6e7: mov rdx, qword ptr [rax + 8]
140a1a6eb: cmp rdx, 4
140a1a6ef: jb 0x140a1a9a0
140a1a6f5: mov rax, qword ptr [rax]
140a1a6f8: movdqu xmm0, xmmword ptr [rax + 0x30]
140a1a6fd: movd eax, xmm0
140a1a701: movshdup xmm1, xmm0
140a1a705: movd edx, xmm1
140a1a709: movdqa xmm1, xmm0
140a1a70d: punpckhqdq xmm1, xmm0
140a1a711: movd r8d, xmm1
140a1a716: and eax, 0x7fffffff
140a1a71b: cmp eax, 0x7f800000
140a1a720: setl al
140a1a723: and edx, 0x7fffffff
140a1a729: cmp edx, 0x7f800000
140a1a72f: setl dl
140a1a732: and dl, al
140a1a734: and r8d, 0x7fffffff
140a1a73b: cmp r8d, 0x7f800000
140a1a742: setl al
140a1a745: test dl, al
140a1a747: je 0x140a1a75f
140a1a749: mov eax, dword ptr [rcx + 0x1c]
140a1a74c: mov rdx, qword ptr [rcx + 0x10]
140a1a750: mov dword ptr [rdx], eax
140a1a752: mov rax, qword ptr [rcx]
140a1a755: movlpd qword ptr [rax + 0x2c], xmm0
140a1a75a: movd dword ptr [rax + 0x34], xmm1
140a1a75f: xor r13, qword ptr [r15 + 0x40]
140a1a763: cmp r13, 5
140a1a767: mov eax, 1
140a1a76c: cmovb rax, r13
140a1a770: lea rdx, [rip + 0x20af451]
140a1a777: movsxd rax, dword ptr [rdx + rax*4]
140a1a77b: add rax, rdx
140a1a77e: jmp rax
140a1a780: mov eax, dword ptr [r12 + 0x1c]
140a1a785: mov rdx, qword ptr [r12 + 0x10]
140a1a78a: mov dword ptr [rdx], eax
140a1a78c: mov rax, qword ptr [r15 + 0x38]
140a1a790: test rax, rax
140a1a793: je 0x140a1a9b1
140a1a799: mov rdx, qword ptr [r15 + 0x30]
140a1a79d: add rax, rdx
140a1a7a0: dec rax
140a1a7a3: mov rdx, qword ptr [r15 + 0x20]
140a1a7a7: mov r8, qword ptr [r15 + 0x28]
140a1a7ab: xor r9d, r9d
140a1a7ae: cmp rax, rdx
140a1a7b1: cmovae r9, rdx
140a1a7b5: sub rax, r9
140a1a7b8: imul rdx, rax, 0xd0
140a1a7bf: mov rax, qword ptr [rcx]
140a1a7c2: movss xmm0, dword ptr [rax]
140a1a7c6: movss dword ptr [r8 + rdx + 0x48], xmm0
140a1a7cd: mov r9, qword ptr [rax + 8]
140a1a7d1: mov r10d, dword ptr [rax + 0x10]
140a1a7d5: mov dword ptr [r8 + rdx + 0x54], r10d
140a1a7da: mov qword ptr [r8 + rdx + 0x4c], r9
140a1a7df: mov r9, qword ptr [rax + 0x14]
140a1a7e3: mov r10d, dword ptr [rax + 0x1c]
140a1a7e7: mov dword ptr [r8 + rdx + 0x60], r10d
140a1a7ec: mov qword ptr [r8 + rdx + 0x58], r9
140a1a7f1: jmp 0x140a1a892
140a1a7f6: mov rax, qword ptr [r15 + 0x48]
140a1a7fa: cmp qword ptr [r15 + 0x38], rax
140a1a7fe: jbe 0x140a1a9d5
140a1a804: add rax, qword ptr [r15 + 0x30]
140a1a808: mov rdx, qword ptr [r15 + 0x20]
140a1a80c: mov r8, qword ptr [r15 + 0x28]
140a1a810: xor r9d, r9d
140a1a813: cmp rax, rdx
140a1a816: cmovae r9, rdx
140a1a81a: sub rax, r9
140a1a81d: imul rdx, rax, 0xd0
140a1a824: movss xmm0, dword ptr [r8 + rdx + 0x48]
140a1a82b: mov r9d, dword ptr [rcx + 0x1c]
140a1a82f: mov r10, qword ptr [rcx + 0x10]
140a1a833: mov dword ptr [r10], r9d
140a1a836: mov rax, qword ptr [rcx]
140a1a839: movss dword ptr [rax], xmm0
140a1a83d: mov r11d, dword ptr [r8 + rdx + 0x54]
140a1a842: mov dword ptr [rsp + 0x38], r11d
140a1a847: mov r11, qword ptr [r8 + rdx + 0x4c]
140a1a84c: mov qword ptr [rsp + 0x30], r11
140a1a851: mov dword ptr [r10], r9d
140a1a854: mov r11d, dword ptr [rsp + 0x38]
140a1a859: mov dword ptr [rax + 0x10], r11d
140a1a85d: mov r11, qword ptr [rsp + 0x30]
140a1a862: mov qword ptr [rax + 8], r11
140a1a866: mov r11d, dword ptr [r8 + rdx + 0x60]
140a1a86b: mov dword ptr [rsp + 0x38], r11d
140a1a870: mov rdx, qword ptr [r8 + rdx + 0x58]
140a1a875: mov qword ptr [rsp + 0x30], rdx
140a1a87a: mov dword ptr [r10], r9d
140a1a87d: mov edx, dword ptr [rsp + 0x38]
140a1a881: mov dword ptr [rax + 0x1c], edx
140a1a884: mov rdx, qword ptr [rsp + 0x30]
140a1a889: mov qword ptr [rax + 0x14], rdx
140a1a88d: jmp 0x140a1a892
140a1a88f: mov rax, qword ptr [rcx]
140a1a892: mov rdx, qword ptr [rbx]
140a1a895: mov r8d, dword ptr [rdx]
140a1a898: xorps xmm1, xmm1
140a1a89b: cvtsi2ss xmm1, r8
140a1a8a0: mov r8, qword ptr [rdi]
140a1a8a3: movss xmm0, dword ptr [r8]
140a1a8a8: mov edx, dword ptr [rdx + 4]
140a1a8ab: divss xmm0, xmm1
140a1a8af: xorps xmm1, xmm1
140a1a8b2: cvtsi2ss xmm1, rdx
140a1a8b7: movss xmm2, dword ptr [r8 + 4]
140a1a8bd: divss xmm2, xmm1
140a1a8c1: mov rdx, qword ptr [rsi]
140a1a8c4: addss xmm0, xmm0
140a1a8c8: addss xmm2, xmm2
140a1a8cc: movss xmm1, dword ptr [rip + 0x1f3f810]
140a1a8d4: addss xmm0, xmm1
140a1a8d8: addss xmm2, xmm1
140a1a8dc: shufps xmm0, xmm0, 0
140a1a8e0: mulps xmm0, xmmword ptr [rdx + 0x40]
140a1a8e4: shufps xmm2, xmm2, 0
140a1a8e8: mulps xmm2, xmmword ptr [rdx + 0x50]
140a1a8ec: subps xmm0, xmm2
140a1a8ef: movss xmm1, dword ptr [rax]
140a1a8f3: shufps xmm1, xmm1, 0
140a1a8f7: mulps xmm1, xmmword ptr [rdx + 0x60]
140a1a8fb: addps xmm1, xmm0
140a1a8fe: addps xmm1, xmmword ptr [rdx + 0x70]
140a1a902: movaps xmm0, xmm1
140a1a905: shufps xmm0, xmm1, 0xff
140a1a909: movhlps xmm1, xmm1
140a1a90c: divss xmm1, xmm0
140a1a910: mov edx, dword ptr [rcx + 0x1c]
140a1a913: mov rcx, qword ptr [rcx + 0x10]
140a1a917: mov dword ptr [rcx], edx
140a1a919: xorps xmm1, xmmword ptr [rip + 0x1f0afd0]
140a1a920: movss dword ptr [rax + 4], xmm1
140a1a925: add rsp, 0x70
140a1a929: pop rbx
140a1a92a: pop rdi
140a1a92b: pop rsi
140a1a92c: pop r12
140a1a92e: pop r13
140a1a930: pop r14
140a1a932: pop r15
140a1a934: ret 
140a1a935: mov qword ptr [rsp + 0x30], 0
140a1a93e: add r14, 0x10
140a1a942: lea rax, [rip + 0x2213837]
140a1a949: mov qword ptr [rsp + 0x20], rax
140a1a94e: lea rdx, [rsp + 0x60]
140a1a953: lea r9, [rsp + 0x30]
140a1a958: xor ecx, ecx
140a1a95a: mov r8, r14
140a1a95d: call 0x1428cab41
140a1a962: lea rcx, [rip + 0x20ae36f]
140a1a969: call 0x1428d9310
140a1a96e: lea r8, [rip + 0x20ae37b]
140a1a975: xor ecx, ecx
140a1a977: xor edx, edx
140a1a979: call 0x1428d9518
140a1a97e: lea r8, [rip + 0x20ae383]
140a1a985: mov ecx, 1
140a1a98a: call 0x1428d9518
140a1a98f: lea r8, [rip + 0x20ae38a]
140a1a996: mov ecx, 2
140a1a99b: call 0x1428d9518
140a1a9a0: lea r8, [rip + 0x20ae391]
140a1a9a7: mov ecx, 3
140a1a9ac: call 0x1428d9518
140a1a9b1: lea rcx, [rip + 0x20ae398]
140a1a9b8: call 0x1428d9310
140a1a9bd: lea rcx, [rip + 0x20ae3bc]
140a1a9c4: lea r8, [rip + 0x20ae3dd]
140a1a9cb: mov edx, 0x28
140a1a9d0: call 0x1428d9430
140a1a9d5: lea rcx, [rip + 0x2098848]
140a1a9dc: lea r8, [rip + 0x20ae385]
140a1a9e3: mov edx, 0x14
140a1a9e8: call 0x1428d9330
140a1a9ed: int3 

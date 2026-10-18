142283640: push rbp
142283641: push r15
142283643: push r14
142283645: push r13
142283647: push r12
142283649: push rsi
14228364a: push rdi
14228364b: push rbx
14228364c: sub rsp, 0x118
142283653: lea rbp, [rsp + 0x80]
14228365b: mov qword ptr [rbp + 0x90], 0xfffffffffffffffe
142283666: mov qword ptr [rbp + 0x80], rdx
14228366d: mov qword ptr [rbp + 0x78], rcx
142283671: mov rdi, qword ptr [r9]
142283674: mov rdx, qword ptr [r9 + 8]
142283678: mov r11, qword ptr [rdi]
14228367b: xor r10d, r10d
14228367e: mov rcx, r11
142283681: sub rcx, qword ptr [rdx + 0x18]
142283685: cmovb rcx, r10
142283689: sub r11, qword ptr [rdx + 0x38]
14228368d: cmovb r11, r10
142283691: mov rax, rcx
142283694: shl rax, 4
142283698: add rax, qword ptr [rdx + 8]
14228369c: mov rbx, qword ptr [rdx + 0x10]
1422836a0: sub rbx, rcx
1422836a3: cmovb rbx, r10
1422836a7: mov r9, qword ptr [rbp + 0x100]
1422836ae: mov esi, 8
1422836b3: cmovb rax, rsi
1422836b7: mov rcx, r11
1422836ba: shl rcx, 4
1422836be: add rcx, qword ptr [rdx + 0x28]
1422836c2: mov r14, qword ptr [rdx + 0x30]
1422836c6: sub r14, r11
1422836c9: cmovb r14, r10
1422836cd: cmovb rcx, rsi
1422836d1: mov rdx, qword ptr [rdx + 0x40]
1422836d5: sub rdx, r14
1422836d8: sub rdx, rbx
1422836db: mov r10, rbx
1422836de: mov qword ptr [rdi], rdx
1422836e1: mov rdx, rdi
1422836e4: shl r10, 4
1422836e8: add r10, rax
1422836eb: shl r14, 4
1422836ef: add r14, rcx
1422836f2: mov edi, dword ptr [r8 + 0x1c]
1422836f6: mov r11, qword ptr [r8]
1422836f9: mov r8, qword ptr [r8 + 0x10]
1422836fd: mov esi, dword ptr [r9 + 0x1c]
142283701: mov dword ptr [rbp + 0x88], esi
142283707: mov rbx, qword ptr [r9]
14228370a: mov r9, qword ptr [r9 + 0x10]
14228370e: lea rsi, [rbx + 0x20]
142283712: mov qword ptr [rbp + 0x70], rsi
142283716: mov qword ptr [rbp + 0x50], rdx
14228371a: mov qword ptr [rbp + 0x58], r14
14228371e: mov dword ptr [rbp + 0x8c], edi
142283724: mov qword ptr [rbp + 0x60], r11
142283728: mov qword ptr [rbp + 0x68], r9
14228372c: jmp 0x14228373a
14228372e: nop 
142283730: mov byte ptr [r11], 1
142283734: mov rcx, r13
142283737: mov rax, r12
14228373a: test rax, rax
14228373d: je 0x142283760
14228373f: cmp rax, r10
142283742: je 0x142283760
142283744: lea r12, [rax + 0x10]
142283748: mov r13, rcx
14228374b: inc qword ptr [rdx]
14228374e: cmp byte ptr [rax + 8], 0
142283752: mov dword ptr [r8], edi
142283755: je 0x142283730
142283757: jmp 0x142283788
142283759: nop dword ptr [rax]
142283760: test rcx, rcx
142283763: je 0x1422837fa
142283769: cmp rcx, r14
14228376c: je 0x1422837fa
142283772: lea r13, [rcx + 0x10]
142283776: xor r12d, r12d
142283779: mov rax, rcx
14228377c: inc qword ptr [rdx]
14228377f: cmp byte ptr [rax + 8], 0
142283783: mov dword ptr [r8], edi
142283786: je 0x142283730
142283788: mov byte ptr [r11 + 1], 1
14228378d: mov eax, dword ptr [rbp + 0x88]
142283793: mov dword ptr [r9], eax
142283796: mov r15, qword ptr [rbx + 0x30]
14228379a: mov rsi, qword ptr [rbx + 0x40]
14228379e: cmp r15, qword ptr [rbx + 0x20]
1422837a2: jne 0x1422837d6
1422837a4: mov rcx, qword ptr [rbp + 0x70]
1422837a8: lea rdx, [rip + 0xdb4d21]
1422837af: mov r14, r10
1422837b2: mov rdi, r8
1422837b5: call 0x140562630
1422837ba: mov r9, qword ptr [rbp + 0x68]
1422837be: mov r8, rdi
1422837c1: mov r11, qword ptr [rbp + 0x60]
1422837c5: mov edi, dword ptr [rbp + 0x8c]
1422837cb: mov r10, r14
1422837ce: mov r14, qword ptr [rbp + 0x58]
1422837d2: mov rdx, qword ptr [rbp + 0x50]
1422837d6: mov rax, qword ptr [rbx + 0x28]
1422837da: mov rcx, r15
1422837dd: shl rcx, 4
1422837e1: mov qword ptr [rax + rcx], rsi
1422837e5: mov byte ptr [rax + rcx + 8], 4
1422837ea: inc r15
1422837ed: mov qword ptr [rbx + 0x30], r15
1422837f1: inc qword ptr [rbx + 0x40]
1422837f5: jmp 0x142283734
1422837fa: mov rax, qword ptr [rbp + 0x78]
1422837fe: mov rax, qword ptr [rax]
142283801: cmp dword ptr [rax], 0xa
142283804: jne 0x142283927
14228380a: mov rax, qword ptr [rbp + 0x80]
142283811: mov rax, qword ptr [rax]
142283814: cmp byte ptr [rax + 0x1fa], 0
14228381b: jne 0x142283927
142283821: cmp byte ptr [r11], 1
142283825: jne 0x142283927
14228382b: mov rax, qword ptr [rbp + 0x110]
142283832: mov rax, qword ptr [rax]
142283835: cmp qword ptr [rax], 0
142283839: je 0x142283927
14228383f: cmp byte ptr [r11 + 1], 0
142283844: jne 0x142283927
14228384a: lea rcx, [rbp - 0x48]
14228384e: call 0x142285270
142283853: mov word ptr [rbp - 0x18], 0x400
142283859: mov dword ptr [rbp - 0x60], 6
142283860: mov rdx, qword ptr [rbp + 0x108]
142283867: mov eax, dword ptr [rdx + 0x1c]
14228386a: mov rcx, qword ptr [rdx + 0x10]
14228386e: mov dword ptr [rcx], eax
142283870: mov rsi, qword ptr [rdx]
142283873: mov rax, qword ptr [rsi + 0x40]
142283877: mov qword ptr [rbp + 0x40], rax
14228387b: mov rax, qword ptr [rbp - 0x20]
14228387f: mov qword ptr [rbp + 0x30], rax
142283883: movzx eax, word ptr [rbp - 0x18]
142283887: mov word ptr [rbp + 0x38], ax
14228388b: mov eax, dword ptr [rbp - 0x16]
14228388e: mov dword ptr [rbp + 0x3a], eax
142283891: movzx eax, word ptr [rbp - 0x12]
142283895: mov word ptr [rbp + 0x3e], ax
142283899: mov eax, dword ptr [rbp - 0x60]
14228389c: mov rcx, qword ptr [rbp - 0x5c]
1422838a0: mov edx, dword ptr [rbp - 0x54]
1422838a3: movups xmm0, xmmword ptr [rbp - 0x50]
1422838a7: movups xmm1, xmmword ptr [rbp - 0x40]
1422838ab: movups xmm2, xmmword ptr [rbp - 0x30]
1422838af: movaps xmmword ptr [rbp + 0x20], xmm2
1422838b3: movaps xmmword ptr [rbp + 0x10], xmm1
1422838b7: movaps xmmword ptr [rbp], xmm0
1422838bb: mov dword ptr [rbp - 0x10], eax
1422838be: mov qword ptr [rbp - 0xc], rcx
1422838c2: mov dword ptr [rbp - 4], edx
1422838c5: mov rdi, qword ptr [rsi + 0x30]
1422838c9: cmp rdi, qword ptr [rsi + 0x20]
1422838cd: jne 0x1422838df
1422838cf: lea rcx, [rsi + 0x20]
1422838d3: lea rdx, [rip + 0xdb4bf6]
1422838da: call 0x140562f30
1422838df: mov rax, qword ptr [rsi + 0x28]
1422838e3: imul rcx, rdi, 0x58
1422838e7: movaps xmm0, xmmword ptr [rbp - 0x10]
1422838eb: movaps xmm1, xmmword ptr [rbp]
1422838ef: movaps xmm2, xmmword ptr [rbp + 0x10]
1422838f3: movaps xmm3, xmmword ptr [rbp + 0x20]
1422838f7: movaps xmm4, xmmword ptr [rbp + 0x30]
1422838fb: mov rdx, qword ptr [rbp + 0x40]
1422838ff: mov qword ptr [rax + rcx + 0x50], rdx
142283904: movups xmmword ptr [rax + rcx + 0x40], xmm4
142283909: movups xmmword ptr [rax + rcx + 0x30], xmm3
14228390e: movups xmmword ptr [rax + rcx + 0x20], xmm2
142283913: movups xmmword ptr [rax + rcx + 0x10], xmm1
142283918: movups xmmword ptr [rax + rcx], xmm0
14228391c: inc rdi
14228391f: mov qword ptr [rsi + 0x30], rdi
142283923: inc qword ptr [rsi + 0x40]
142283927: add rsp, 0x118
14228392e: pop rbx
14228392f: pop rdi
142283930: pop rsi
142283931: pop r12
142283933: pop r13
142283935: pop r14
142283937: pop r15
142283939: pop rbp
14228393a: ret 
14228393b: nop dword ptr [rax + rax]
142283940: mov qword ptr [rsp + 0x10], rdx
142283945: push rbp
142283946: push r15
142283948: push r14
14228394a: push r13
14228394c: push r12
14228394e: push rsi
14228394f: push rdi
142283950: push rbx
142283951: sub rsp, 0x28
142283955: lea rbp, [rdx + 0x80]
14228395c: lea rcx, [rbp + 8]
142283960: call 0x140eaaa50
142283965: nop 
142283966: add rsp, 0x28
14228396a: pop rbx
14228396b: pop rdi
14228396c: pop rsi
14228396d: pop r12
14228396f: pop r13
142283971: pop r14
142283973: pop r15
142283975: pop rbp
142283976: ret 

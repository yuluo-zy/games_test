1409423d0: push rbp
1409423d1: push r15
1409423d3: push r14
1409423d5: push r13
1409423d7: push r12
1409423d9: push rsi
1409423da: push rdi
1409423db: push rbx
1409423dc: sub rsp, 0x208
1409423e3: lea rbp, [rsp + 0x80]
1409423eb: mov qword ptr [rbp + 0x180], 0xfffffffffffffffe
1409423f6: mov rax, qword ptr [rbp + 0x218]
1409423fd: mov r10, qword ptr [rbp + 0x210]
140942404: mov r11d, dword ptr [r10 + 0x1c]
140942408: mov rsi, qword ptr [r10 + 0x10]
14094240c: mov dword ptr [rsi], r11d
14094240f: mov rsi, qword ptr [rax]
140942412: mov r14, qword ptr [rax + 8]
140942416: mov rax, qword ptr [rsi]
140942419: xor ebx, ebx
14094241b: mov r11, rax
14094241e: sub r11, qword ptr [r14 + 0x18]
140942422: cmovb r11, rbx
140942426: sub rax, qword ptr [r14 + 0x38]
14094242a: cmovb rax, rbx
14094242e: mov rdi, qword ptr [r14 + 0x30]
140942432: sub rdi, rax
140942435: cmovb rdi, rbx
140942439: seta r15b
14094243d: mov r12, qword ptr [r14 + 0x10]
140942441: sub r12, r11
140942444: cmovb r12, rbx
140942448: seta r11b
14094244c: add r12, rdi
14094244f: mov rax, qword ptr [rbp + 0x208]
140942456: or r11b, r15b
140942459: mov rdi, qword ptr [rbp + 0x200]
140942460: cmovne r12, rbx
140942464: mov rbx, qword ptr [rbp + 0x1f8]
14094246b: mov r13, qword ptr [r10]
14094246e: mov r10, qword ptr [r14 + 0x40]
140942472: sub r10, r12
140942475: mov qword ptr [rsi], r10
140942478: mov r10, qword ptr [r13 + 0x40]
14094247c: movabs rsi, 0x8000000000000000
140942486: xor rsi, r10
140942489: cmp rsi, 5
14094248d: mov r14d, 1
140942493: cmovb r14, rsi
140942497: lea rsi, [rip + 0x21707c6]
14094249e: movsxd r15, dword ptr [rsi + r14*4]
1409424a2: add r15, rsi
1409424a5: mov r14, qword ptr [rbp + 0x1f0]
1409424ac: jmp r15
1409424af: mov rsi, qword ptr [rbp + 0x220]
1409424b6: mov r15, qword ptr [rax]
1409424b9: movups xmm0, xmmword ptr [r15 + 0x58]
1409424be: movupd xmm1, xmmword ptr [r15 + 0x68]
1409424c4: movups xmm2, xmmword ptr [r15 + 0x78]
1409424c9: movups xmm3, xmmword ptr [r15 + 0x88]
1409424d1: movaps xmmword ptr [rbp + 0xe0], xmm0
1409424d8: movapd xmmword ptr [rbp + 0xf0], xmm1
1409424e0: movaps xmmword ptr [rbp + 0x100], xmm2
1409424e7: movaps xmmword ptr [rbp + 0x110], xmm3
1409424ee: movups xmm0, xmmword ptr [r15 + 0x93]
1409424f6: movups xmmword ptr [rbp + 0x11b], xmm0
1409424fd: movzx eax, byte ptr [r15 + 0xa3]
140942505: mov r12d, dword ptr [r15 + 0xac]
14094250c: mov dword ptr [rbp + 0x170], r12d
140942513: mov r15, qword ptr [r15 + 0xa4]
14094251a: mov qword ptr [rbp + 0x168], r15
140942521: cmp qword ptr [rsi], 0
140942525: setne sil
140942529: or r11b, sil
14094252c: jne 0x140942536
14094252e: test al, 1
140942530: je 0x1409426ba
140942536: mov r15, rcx
140942539: mov rax, qword ptr [rdx]
14094253c: movups xmm0, xmmword ptr [rax]
14094253f: movaps xmmword ptr [rbp], xmm0
140942543: mov rsi, qword ptr [r8]
140942546: lea r8, [rip + 0x2170653]
14094254d: lea rcx, [rbp + 0x130]
140942554: mov rdx, rsi
140942557: mov r12, r9
14094255a: call 0x1412f7fa0
14094255f: movzx eax, byte ptr [rsi + 0x18]
140942563: mov byte ptr [rbp + 0x148], al
140942569: mov rdx, qword ptr [r12]
14094256d: lea rcx, [rbp + 0x10]
140942571: call 0x140911710
140942576: movaps xmm0, xmmword ptr [rbp]
14094257a: movaps xmmword ptr [rbp - 0x40], xmm0
14094257e: movupd xmm1, xmmword ptr [rbp + 0x130]
140942586: movups xmm2, xmmword ptr [rbp + 0x140]
14094258d: movapd xmmword ptr [rbp - 0x60], xmm1
140942592: movaps xmmword ptr [rbp - 0x50], xmm2
140942596: movups xmm3, xmmword ptr [rbp + 0x10]
14094259a: movups xmm4, xmmword ptr [rbp + 0x20]
14094259e: movaps xmmword ptr [rbp - 0x30], xmm3
1409425a2: movaps xmmword ptr [rbp - 0x20], xmm4
1409425a6: mov eax, dword ptr [rbp + 0x30]
1409425a9: mov dword ptr [rbp - 0x10], eax
1409425ac: mov rax, qword ptr [rbp - 0x10]
1409425b0: mov qword ptr [rbp + 0x60], rax
1409425b4: movaps xmmword ptr [rbp + 0x50], xmm4
1409425b8: movaps xmmword ptr [rbp + 0x40], xmm3
1409425bc: movaps xmmword ptr [rbp + 0x30], xmm0
1409425c0: movaps xmmword ptr [rbp + 0x20], xmm2
1409425c4: movapd xmmword ptr [rbp + 0x10], xmm1
1409425c9: mov qword ptr [rbp + 0x68], 0
1409425d1: lea r8, [rip + 0x2170d68]
1409425d8: lea rcx, [rbp + 0x130]
1409425df: lea rdx, [rbp + 0x10]
1409425e3: call 0x141301640
1409425e8: mov rax, qword ptr [rbp + 0x130]
1409425ef: mov qword ptr [rbp + 0x160], rax
1409425f6: mov rax, qword ptr [rbp + 0x138]
1409425fd: mov qword ptr [rbp + 0x158], rax
140942604: mov rax, qword ptr [rbp + 0x140]
14094260b: mov qword ptr [rbp + 0x150], rax
140942612: mov rcx, r13
140942615: mov qword ptr [rbp + 0x178], r13
14094261c: call 0x1405d7290
140942621: mov rax, qword ptr [rbp + 0x178]
140942628: lea rcx, [rax + 0x20]
14094262c: call 0x1405d4b50
140942631: mov r13, qword ptr [rbp + 0x178]
140942638: mov rax, qword ptr [rbp + 0x160]
14094263f: mov qword ptr [r13], rax
140942643: mov rax, qword ptr [rbp + 0x158]
14094264a: mov qword ptr [r13 + 8], rax
14094264e: mov qword ptr [r13 + 0x10], 0
140942656: mov rax, qword ptr [rbp + 0x150]
14094265d: mov qword ptr [r13 + 0x18], rax
140942661: mov qword ptr [r13 + 0x20], 0
140942669: mov qword ptr [r13 + 0x28], 8
140942671: xorps xmm0, xmm0
140942674: movups xmmword ptr [r13 + 0x30], xmm0
140942679: movups xmmword ptr [rbp + 0x11b], xmm0
140942680: movaps xmmword ptr [rbp + 0x110], xmm0
140942687: movaps xmmword ptr [rbp + 0x100], xmm0
14094268e: movaps xmmword ptr [rbp + 0xf0], xmm0
140942695: movaps xmmword ptr [rbp + 0xe0], xmm0
14094269c: mov qword ptr [rbp + 0x168], 0
1409426a7: mov dword ptr [rbp + 0x170], 0
1409426b1: mov r10, qword ptr [r13 + 0x40]
1409426b5: xor eax, eax
1409426b7: mov rcx, r15
1409426ba: mov r9, qword ptr [rbx]
1409426bd: mov rsi, qword ptr [r14]
1409426c0: movzx r11d, byte ptr [rsi + 0xf4]
1409426c8: xorpd xmm1, xmm1
1409426cc: movupd xmmword ptr [rbp + 0x64], xmm1
1409426d1: movupd xmmword ptr [rbp + 0x58], xmm1
1409426d6: movupd xmmword ptr [rbp + 0x20], xmm1
1409426db: mov rdx, qword ptr [rdi]
1409426de: mov rcx, qword ptr [rcx]
1409426e1: movss xmm0, dword ptr [rcx + 0x28]
1409426e6: movabs rcx, 0x8000000000000005
1409426f0: cmp r10, rcx
1409426f3: setl cl
1409426f6: movabs r8, 0x8000000000000001
140942700: cmp r10, r8
140942703: setne r8b
140942707: and r8b, cl
14094270a: movups xmm2, xmmword ptr [rbp + 0x11b]
140942711: movups xmmword ptr [rbp + 0xb3], xmm2
140942718: movaps xmm2, xmmword ptr [rbp + 0xe0]
14094271f: movaps xmm3, xmmword ptr [rbp + 0xf0]
140942726: movaps xmm4, xmmword ptr [rbp + 0x100]
14094272d: movaps xmm5, xmmword ptr [rbp + 0x110]
140942734: movups xmmword ptr [rbp + 0xa8], xmm5
14094273b: movups xmmword ptr [rbp + 0x98], xmm4
140942742: movups xmmword ptr [rbp + 0x88], xmm3
140942749: movups xmmword ptr [rbp + 0x78], xmm2
14094274d: mov rcx, qword ptr [rbp + 0x168]
140942754: mov qword ptr [rbp + 0xc4], rcx
14094275b: mov ecx, dword ptr [rbp + 0x170]
140942761: mov dword ptr [rbp + 0xcc], ecx
140942767: mov qword ptr [rbp + 0x10], 0
14094276f: mov qword ptr [rbp + 0x18], 4
140942777: mov qword ptr [rbp + 0x30], 4
14094277f: mov qword ptr [rbp + 0x38], 0
140942787: movsd xmm2, qword ptr [rsi + 0xe8]
14094278f: movsd qword ptr [rbp + 0x40], xmm2
140942794: mov ecx, dword ptr [rsi + 0xf0]
14094279a: mov dword ptr [rbp + 0x48], ecx
14094279d: mov byte ptr [rbp + 0x4c], r11b
1409427a1: movsd xmm2, qword ptr [r9]
1409427a6: unpcklps xmm2, xmm1
1409427a9: movapd xmm1, xmmword ptr [rip + 0x20a6dff]
1409427b1: orpd xmm2, xmm1
1409427b5: subpd xmm2, xmm1
1409427b9: cvtpd2ps xmm1, xmm2
1409427bd: movsd xmm2, qword ptr [rdx]
1409427c1: movaps xmm3, xmmword ptr [rip + 0x20d4a28]
1409427c8: mulps xmm3, xmm1
1409427cb: addps xmm3, xmm2
1409427ce: movshdup xmm2, xmm1
1409427d2: divps xmm3, xmm2
1409427d5: movlpd qword ptr [rbp + 0x50], xmm1
1409427da: movlps qword ptr [rbp + 0xd0], xmm3
1409427e1: movss dword ptr [rbp + 0xd8], xmm0
1409427e9: mov byte ptr [rbp + 0xc3], al
1409427ef: lea rdx, [rbp + 0x10]
1409427f3: mov rcx, r13
1409427f6: call 0x140949190
1409427fb: jmp 0x1409429ce
140942800: mov ecx, dword ptr [rax + 0x1c]
140942803: mov rdx, qword ptr [rax + 0x10]
140942807: mov dword ptr [rdx], ecx
140942809: mov rax, qword ptr [rax]
14094280c: xorps xmm0, xmm0
14094280f: movups xmmword ptr [rax + 0xa0], xmm0
140942816: movups xmmword ptr [rax + 0x90], xmm0
14094281d: movups xmmword ptr [rax + 0x80], xmm0
140942824: movups xmmword ptr [rax + 0x70], xmm0
140942828: movups xmmword ptr [rax + 0x60], xmm0
14094282c: movups xmmword ptr [rax + 0x50], xmm0
140942830: movups xmmword ptr [rax + 0x40], xmm0
140942834: movups xmmword ptr [rax + 0x30], xmm0
140942838: movups xmmword ptr [rax + 0x20], xmm0
14094283c: movups xmmword ptr [rax + 0x10], xmm0
140942840: movups xmmword ptr [rax], xmm0
140942843: jmp 0x1409429ce
140942848: mov ecx, dword ptr [rax + 0x1c]
14094284b: mov rdx, qword ptr [rax + 0x10]
14094284f: mov dword ptr [rdx], ecx
140942851: mov rdx, qword ptr [r13 + 0x48]
140942855: cmp qword ptr [r13 + 0x38], rdx
140942859: jbe 0x1409429e2
14094285f: add rdx, qword ptr [r13 + 0x30]
140942863: mov rax, qword ptr [rax]
140942866: mov r9, qword ptr [r13 + 0x20]
14094286a: mov rcx, qword ptr [r13 + 0x28]
14094286e: xor r8d, r8d
140942871: cmp rdx, r9
140942874: mov r10d, 0
14094287a: cmovae r10, r9
14094287e: sub rdx, r10
140942881: imul rdx, rdx, 0xd0
140942888: movups xmm0, xmmword ptr [rcx + rdx + 0x68]
14094288d: movups xmm1, xmmword ptr [rcx + rdx + 0x78]
140942892: movups xmm2, xmmword ptr [rcx + rdx + 0x88]
14094289a: movups xmm3, xmmword ptr [rcx + rdx + 0x98]
1409428a2: movups xmm4, xmmword ptr [rcx + rdx + 0xa8]
1409428aa: mov r9, qword ptr [rcx + rdx + 0xb8]
1409428b2: mov qword ptr [rax + 0xa8], r9
1409428b9: movups xmmword ptr [rax + 0x98], xmm4
1409428c0: movups xmmword ptr [rax + 0x88], xmm3
1409428c7: movups xmmword ptr [rax + 0x78], xmm2
1409428cb: movups xmmword ptr [rax + 0x68], xmm1
1409428cf: movups xmmword ptr [rax + 0x58], xmm0
1409428d3: movzx r9d, byte ptr [rcx + rdx + 0x3c]
1409428d9: mov r10d, dword ptr [r14 + 0x1c]
1409428dd: mov r11, qword ptr [r14]
1409428e0: mov rsi, qword ptr [r14 + 0x10]
1409428e4: movsd xmm0, qword ptr [rcx + rdx + 0x30]
1409428ea: mov r14d, dword ptr [rcx + rdx + 0x38]
1409428ef: mov dword ptr [rsi], r10d
1409428f2: movsd qword ptr [r11 + 0xe8], xmm0
1409428fb: mov dword ptr [r11 + 0xf0], r14d
140942902: mov byte ptr [r11 + 0xf4], r9b
140942909: movsd xmm0, qword ptr [rcx + rdx + 0x40]
14094290f: cvttss2si r9, xmm0
140942914: xorps xmm1, xmm1
140942917: ucomiss xmm0, xmm1
14094291a: cmovb r9d, r8d
14094291e: mov r10d, dword ptr [rbx + 0x1c]
140942922: movss xmm2, dword ptr [rip + 0x205dea6]
14094292a: ucomiss xmm0, xmm2
14094292d: mov r11d, 0xffffffff
140942933: cmova r9d, r11d
140942937: mov rsi, qword ptr [rbx]
14094293a: mov rbx, qword ptr [rbx + 0x10]
14094293e: movd xmm3, r9d
140942943: movshdup xmm0, xmm0
140942947: cvttss2si r9, xmm0
14094294c: ucomiss xmm0, xmm1
14094294f: cmovb r9d, r8d
140942953: ucomiss xmm0, xmm2
140942956: cmova r9d, r11d
14094295a: movd xmm0, r9d
14094295f: punpckldq xmm3, xmm0
140942963: mov dword ptr [rbx], r10d
140942966: movq qword ptr [rsi], xmm3
14094296a: movsd xmm0, qword ptr [rax + 0x60]
14094296f: mov r8d, dword ptr [rdi + 0x1c]
140942973: mov r9, qword ptr [rdi]
140942976: mov r10, qword ptr [rdi + 0x10]
14094297a: xorps xmm1, xmm1
14094297d: punpckldq xmm3, xmm1
140942981: movdqa xmm1, xmmword ptr [rip + 0x20a6c27]
140942989: por xmm3, xmm1
14094298d: subpd xmm3, xmm1
140942991: cvtpd2ps xmm1, xmm3
140942995: movshdup xmm2, xmm1
140942999: movss xmm3, dword ptr [rcx + rdx + 0x44]
14094299f: movaps xmm4, xmm2
1409429a2: divss xmm4, xmm3
1409429a6: movsldup xmm3, xmm4
1409429aa: mulps xmm3, xmm0
1409429ad: movlps qword ptr [rax + 0x60], xmm3
1409429b1: movsd xmm0, qword ptr [rcx + rdx + 0xc0]
1409429ba: mulps xmm1, xmmword ptr [rip + 0x1fe2f1f]
1409429c1: mulps xmm0, xmm2
1409429c4: addps xmm1, xmm0
1409429c7: mov dword ptr [r10], r8d
1409429ca: movlps qword ptr [r9], xmm1
1409429ce: add rsp, 0x208
1409429d5: pop rbx
1409429d6: pop rdi
1409429d7: pop rsi
1409429d8: pop r12
1409429da: pop r13
1409429dc: pop r14
1409429de: pop r15
1409429e0: pop rbp
1409429e1: ret 
1409429e2: lea rcx, [rip + 0x217083b]
1409429e9: lea r8, [rip + 0x216fe48]
1409429f0: mov edx, 0x14
1409429f5: call 0x1428d9330
1409429fa: int3 
1409429fb: nop dword ptr [rax + rax]
140942a00: mov qword ptr [rsp + 0x10], rdx
140942a05: push rbp
140942a06: push r15
140942a08: push r14
140942a0a: push r13
140942a0c: push r12
140942a0e: push rsi
140942a0f: push rdi
140942a10: push rbx
140942a11: sub rsp, 0x28
140942a15: lea rbp, [rdx + 0x80]
140942a1c: lea rcx, [rbp + 0x130]
140942a23: call 0x14014b1c0
140942a28: nop 
140942a29: add rsp, 0x28
140942a2d: pop rbx
140942a2e: pop rdi
140942a2f: pop rsi
140942a30: pop r12
140942a32: pop r13
140942a34: pop r14
140942a36: pop r15
140942a38: pop rbp
140942a39: ret 
140942a3a: nop word ptr [rax + rax]
140942a40: mov qword ptr [rsp + 0x10], rdx
140942a45: push rbp
140942a46: push r15
140942a48: push r14
140942a4a: push r13
140942a4c: push r12
140942a4e: push rsi
140942a4f: push rdi
140942a50: push rbx
140942a51: sub rsp, 0x28
140942a55: lea rbp, [rdx + 0x80]
140942a5c: mov rax, qword ptr [rbp + 0x178]
140942a63: lea rcx, [rax + 0x20]
140942a67: call 0x1405d4b50
140942a6c: nop 
140942a6d: add rsp, 0x28
140942a71: pop rbx
140942a72: pop rdi
140942a73: pop rsi
140942a74: pop r12
140942a76: pop r13
140942a78: pop r14
140942a7a: pop r15
140942a7c: pop rbp
140942a7d: ret 
140942a7e: nop 
140942a80: mov qword ptr [rsp + 0x10], rdx
140942a85: push rbp
140942a86: push r15
140942a88: push r14
140942a8a: push r13
140942a8c: push r12
140942a8e: push rsi
140942a8f: push rdi
140942a90: push rbx
140942a91: sub rsp, 0x28
140942a95: lea rbp, [rdx + 0x80]
140942a9c: mov rax, qword ptr [rbp + 0x178]
140942aa3: mov rcx, qword ptr [rbp + 0x160]
140942aaa: mov qword ptr [rax], rcx
140942aad: mov rcx, qword ptr [rbp + 0x158]
140942ab4: mov qword ptr [rax + 8], rcx
140942ab8: mov qword ptr [rax + 0x10], 0
140942ac0: mov rcx, qword ptr [rbp + 0x150]
140942ac7: mov qword ptr [rax + 0x18], rcx
140942acb: mov qword ptr [rax + 0x20], 0
140942ad3: mov qword ptr [rax + 0x28], 8
140942adb: xorps xmm0, xmm0
140942ade: movups xmmword ptr [rax + 0x30], xmm0
140942ae2: add rsp, 0x28
140942ae6: pop rbx
140942ae7: pop rdi
140942ae8: pop rsi
140942ae9: pop r12
140942aeb: pop r13
140942aed: pop r14
140942aef: pop r15
140942af1: pop rbp
140942af2: ret 

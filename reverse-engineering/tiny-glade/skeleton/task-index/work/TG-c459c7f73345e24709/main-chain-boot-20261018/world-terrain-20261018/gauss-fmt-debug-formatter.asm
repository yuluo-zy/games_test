core::fmt::Formatter::pad
140d86280: push rbp
140d86281: push r15
140d86283: push r14
140d86285: push r13
140d86287: push r12
140d86289: push rsi
140d8628a: push rdi
140d8628b: push rbx
140d8628c: sub rsp, 0x28
140d86290: lea rbp, [rsp + 0x20]
140d86295: mov rsi, r8
140d86298: mov rdi, rdx
140d8629b: mov rax, qword ptr [rcx]
140d8629e: mov rdx, qword ptr [rcx + 0x10]
140d862a2: mov r8, rax
140d862a5: or r8, rdx
140d862a8: je 0x140d865fa
140d862ae: test dl, 1
140d862b1: je 0x140d8634b
140d862b7: mov r9, qword ptr [rcx + 0x18]
140d862bb: lea r8, [rdi + rsi]
140d862bf: xor edx, edx
140d862c1: test r9, r9
140d862c4: je 0x140d86311
140d862c6: mov r10, rdi
140d862c9: jmp 0x140d862e5
140d862cb: nop dword ptr [rax + rax]
140d862d0: lea r11, [r10 + 1]
140d862d4: mov rbx, r11
140d862d7: sub rbx, r10
140d862da: add rdx, rbx
140d862dd: mov r10, r11
140d862e0: dec r9
140d862e3: je 0x140d86314
140d862e5: cmp r10, r8
140d862e8: je 0x140d8634b
140d862ea: movzx r11d, byte ptr [r10]
140d862ee: test r11b, r11b
140d862f1: jns 0x140d862d0
140d862f3: cmp r11b, 0xe0
140d862f7: jb 0x140d86305
140d862f9: cmp r11b, 0xf0
140d862fd: jb 0x140d8630b
140d862ff: lea r11, [r10 + 4]
140d86303: jmp 0x140d862d4
140d86305: lea r11, [r10 + 2]
140d86309: jmp 0x140d862d4
140d8630b: lea r11, [r10 + 3]
140d8630f: jmp 0x140d862d4
140d86311: mov r11, rdi
140d86314: cmp r11, r8
140d86317: je 0x140d8634b
140d86319: movzx r8d, byte ptr [r11]
140d8631d: test r8b, r8b
140d86320: jns 0x140d86326
140d86322: cmp r8b, 0xe0
140d86326: test rdx, rdx
140d86329: je 0x140d8633d
140d8632b: cmp rdx, rsi
140d8632e: jae 0x140d8633b
140d86330: cmp byte ptr [rdi + rdx], 0xc0
140d86334: jge 0x140d8633d
140d86336: xor r8d, r8d
140d86339: jmp 0x140d86340
140d8633b: jne 0x140d86336
140d8633d: mov r8, rdi
140d86340: test r8, r8
140d86343: cmovne rsi, rdx
140d86347: cmovne rdi, r8
140d8634b: test rax, rax
140d8634e: je 0x140d865fa
140d86354: mov r15, qword ptr [rcx + 8]
140d86358: cmp rsi, 0x20
140d8635c: jae 0x140d86372
140d8635e: test rsi, rsi
140d86361: je 0x140d86388
140d86363: cmp rsi, 4
140d86367: jae 0x140d8638f
140d86369: xor eax, eax
140d8636b: xor edx, edx
140d8636d: jmp 0x140d865be
140d86372: mov rbx, rcx
140d86375: mov rcx, rdi
140d86378: mov rdx, rsi
140d8637b: call 0x140d89170 ; core::str::count::do_count_chars
140d86380: mov rcx, rbx
140d86383: jmp 0x140d865d4
140d86388: xor eax, eax
140d8638a: jmp 0x140d865d4
140d8638f: mov edx, esi
140d86391: and edx, 0x1c
140d86394: movzx eax, word ptr [rdi]
140d86397: movd xmm0, eax
140d8639b: movzx eax, word ptr [rdi + 2]
140d8639f: movd xmm1, eax
140d863a3: movdqa xmm2, xmmword ptr [rip + 0x1dc5955]
140d863ab: pcmpgtb xmm0, xmm2
140d863af: punpcklbw xmm0, xmm0
140d863b3: pshuflw xmm0, xmm0, 0xd4
140d863b8: pshufd xmm0, xmm0, 0xd4
140d863bd: movdqa xmm3, xmmword ptr [rip + 0x1d5a93b]
140d863c5: pand xmm0, xmm3
140d863c9: pcmpgtb xmm1, xmm2
140d863cd: punpcklbw xmm1, xmm1
140d863d1: pshuflw xmm1, xmm1, 0xd4
140d863d6: pshufd xmm1, xmm1, 0xd4
140d863db: pand xmm1, xmm3
140d863df: cmp rdx, 4
140d863e3: je 0x140d865aa
140d863e9: movzx eax, word ptr [rdi + 4]
140d863ed: movd xmm4, eax
140d863f1: movzx eax, word ptr [rdi + 6]
140d863f5: movd xmm5, eax
140d863f9: pcmpgtb xmm4, xmm2
140d863fd: punpcklbw xmm4, xmm4
140d86401: pshuflw xmm4, xmm4, 0xd4
140d86406: pshufd xmm4, xmm4, 0xd4
140d8640b: pand xmm4, xmm3
140d8640f: pcmpgtb xmm5, xmm2
140d86413: punpcklbw xmm5, xmm5
140d86417: pshuflw xmm5, xmm5, 0xd4
140d8641c: pshufd xmm5, xmm5, 0xd4
140d86421: pand xmm5, xmm3
140d86425: paddq xmm0, xmm4
140d86429: paddq xmm1, xmm5
140d8642d: cmp edx, 8
140d86430: je 0x140d865aa
140d86436: movzx eax, word ptr [rdi + 8]
140d8643a: movd xmm4, eax
140d8643e: movzx eax, word ptr [rdi + 0xa]
140d86442: movd xmm5, eax
140d86446: pcmpgtb xmm4, xmm2
140d8644a: punpcklbw xmm4, xmm4
140d8644e: pshuflw xmm4, xmm4, 0xd4
140d86453: pshufd xmm4, xmm4, 0xd4
140d86458: pand xmm4, xmm3
140d8645c: pcmpgtb xmm5, xmm2
140d86460: punpcklbw xmm5, xmm5
140d86464: pshuflw xmm5, xmm5, 0xd4
140d86469: pshufd xmm5, xmm5, 0xd4
140d8646e: pand xmm5, xmm3
140d86472: paddq xmm0, xmm4
140d86476: paddq xmm1, xmm5
140d8647a: cmp edx, 0xc
140d8647d: je 0x140d865aa
140d86483: movzx eax, word ptr [rdi + 0xc]
140d86487: movd xmm4, eax
140d8648b: movzx eax, word ptr [rdi + 0xe]
140d8648f: movd xmm5, eax
140d86493: pcmpgtb xmm4, xmm2
140d86497: punpcklbw xmm4, xmm4
140d8649b: pshuflw xmm4, xmm4, 0xd4
140d864a0: pshufd xmm4, xmm4, 0xd4
140d864a5: pand xmm4, xmm3
140d864a9: pcmpgtb xmm5, xmm2
140d864ad: punpcklbw xmm5, xmm5
140d864b1: pshuflw xmm5, xmm5, 0xd4
140d864b6: pshufd xmm5, xmm5, 0xd4
140d864bb: pand xmm5, xmm3
140d864bf: paddq xmm0, xmm4
140d864c3: paddq xmm1, xmm5
140d864c7: cmp edx, 0x10
140d864ca: je 0x140d865aa
140d864d0: movzx eax, word ptr [rdi + 0x10]
140d864d4: movd xmm4, eax
140d864d8: movzx eax, word ptr [rdi + 0x12]
140d864dc: movd xmm5, eax
140d864e0: pcmpgtb xmm4, xmm2
140d864e4: punpcklbw xmm4, xmm4
140d864e8: pshuflw xmm4, xmm4, 0xd4
140d864ed: pshufd xmm4, xmm4, 0xd4
140d864f2: pand xmm4, xmm3
140d864f6: pcmpgtb xmm5, xmm2
140d864fa: punpcklbw xmm5, xmm5
140d864fe: pshuflw xmm5, xmm5, 0xd4
140d86503: pshufd xmm5, xmm5, 0xd4
140d86508: pand xmm5, xmm3
140d8650c: paddq xmm0, xmm4
140d86510: paddq xmm1, xmm5
140d86514: cmp edx, 0x14
140d86517: je 0x140d865aa
140d8651d: movzx eax, word ptr [rdi + 0x14]
140d86521: movd xmm4, eax
140d86525: movzx eax, word ptr [rdi + 0x16]
140d86529: movd xmm5, eax
140d8652d: pcmpgtb xmm4, xmm2
140d86531: punpcklbw xmm4, xmm4
140d86535: pshuflw xmm4, xmm4, 0xd4
140d8653a: pshufd xmm4, xmm4, 0xd4
140d8653f: pand xmm4, xmm3
140d86543: pcmpgtb xmm5, xmm2
140d86547: punpcklbw xmm5, xmm5
140d8654b: pshuflw xmm5, xmm5, 0xd4
140d86550: pshufd xmm5, xmm5, 0xd4
140d86555: pand xmm5, xmm3
140d86559: paddq xmm0, xmm4
140d8655d: paddq xmm1, xmm5
140d86561: cmp edx, 0x18
140d86564: je 0x140d865aa
140d86566: movzx eax, word ptr [rdi + 0x18]
140d8656a: movd xmm4, eax
140d8656e: movzx eax, word ptr [rdi + 0x1a]
140d86572: movd xmm5, eax
140d86576: pcmpgtb xmm4, xmm2
140d8657a: punpcklbw xmm4, xmm4
140d8657e: pshuflw xmm4, xmm4, 0xd4
140d86583: pshufd xmm4, xmm4, 0xd4
140d86588: pand xmm4, xmm3
140d8658c: pcmpgtb xmm5, xmm2
140d86590: punpcklbw xmm5, xmm5
140d86594: pshuflw xmm2, xmm5, 0xd4
140d86599: pshufd xmm2, xmm2, 0xd4
140d8659e: pand xmm2, xmm3
140d865a2: paddq xmm0, xmm4
140d865a6: paddq xmm1, xmm2
140d865aa: paddq xmm0, xmm1
140d865ae: pshufd xmm1, xmm0, 0xee
140d865b3: paddq xmm1, xmm0
140d865b7: movq rax, xmm1
140d865bc: jmp 0x140d865cf
140d865be: xor r8d, r8d
140d865c1: cmp byte ptr [rdi + rdx], 0xc0
140d865c5: setge r8b
140d865c9: add rax, r8
140d865cc: inc rdx
140d865cf: cmp rsi, rdx
140d865d2: jne 0x140d865be
140d865d4: sub r15, rax
140d865d7: jbe 0x140d865fa
140d865d9: mov eax, dword ptr [rcx + 0x28]
140d865dc: xor edx, edx
140d865de: cmp al, 3
140d865e0: cmovne edx, eax
140d865e3: movzx r13d, dl
140d865e7: test r13b, r13b
140d865ea: je 0x140d8662e
140d865ec: cmp r13d, 1
140d865f0: jne 0x140d86622
140d865f2: mov r13, r15
140d865f5: xor r15d, r15d
140d865f8: jmp 0x140d8662e
140d865fa: mov rax, qword ptr [rcx + 0x30]
140d865fe: mov rcx, qword ptr [rcx + 0x38]
140d86602: mov r9, qword ptr [rcx + 0x18]
140d86606: mov rcx, rax
140d86609: mov rdx, rdi
140d8660c: mov r8, rsi
140d8660f: add rsp, 0x28
140d86613: pop rbx
140d86614: pop rdi
140d86615: pop rsi
140d86616: pop r12
140d86618: pop r13
140d8661a: pop r14
140d8661c: pop r15
140d8661e: pop rbp
140d8661f: jmp r9
140d86622: mov r13, r15
140d86625: shr r13, 1
140d86628: inc r15
140d8662b: shr r15, 1
140d8662e: mov rbx, qword ptr [rcx + 0x30]
140d86632: mov r12, qword ptr [rcx + 0x38]
140d86636: mov r14d, dword ptr [rcx + 0x20]
140d8663a: inc r13
140d8663d: nop dword ptr [rax]
140d86640: mov rcx, rbx
140d86643: dec r13
140d86646: je 0x140d86658
140d86648: mov edx, r14d
140d8664b: call qword ptr [r12 + 0x20]
140d86650: test al, al
140d86652: je 0x140d86640
140d86654: mov al, 1
140d86656: jmp 0x140d866a7
140d86658: mov rdx, rdi
140d8665b: mov r8, rsi
140d8665e: call qword ptr [r12 + 0x18]
140d86663: mov ecx, eax
140d86665: mov al, 1
140d86667: test cl, cl
140d86669: jne 0x140d866a7
140d8666b: mov rdi, r15
140d8666e: neg rdi
140d86671: mov rsi, 0xffffffffffffffff
140d86678: nop dword ptr [rax + rax]
140d86680: lea rax, [rdi + rsi]
140d86684: cmp rax, -1
140d86688: je 0x140d8669e
140d8668a: mov rcx, rbx
140d8668d: mov edx, r14d
140d86690: call qword ptr [r12 + 0x20]
140d86695: inc rsi
140d86698: test al, al
140d8669a: je 0x140d86680
140d8669c: jmp 0x140d866a1
140d8669e: mov rsi, r15
140d866a1: cmp rsi, r15
140d866a4: setb al
140d866a7: add rsp, 0x28
140d866ab: pop rbx
140d866ac: pop rdi
140d866ad: pop rsi
140d866ae: pop r12
140d866b0: pop r13
140d866b2: pop r14
140d866b4: pop r15
140d866b6: pop rbp
140d866b7: ret 

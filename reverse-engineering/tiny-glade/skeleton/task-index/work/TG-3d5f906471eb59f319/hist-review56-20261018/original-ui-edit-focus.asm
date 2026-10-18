1409771f0: push r15
1409771f2: push r14
1409771f4: push r13
1409771f6: push r12
1409771f8: push rsi
1409771f9: push rdi
1409771fa: push rbp
1409771fb: push rbx
1409771fc: sub rsp, 0x1d8
140977203: movaps xmmword ptr [rsp + 0x1c0], xmm8
14097720c: movaps xmmword ptr [rsp + 0x1b0], xmm7
140977214: movaps xmmword ptr [rsp + 0x1a0], xmm6
14097721c: mov rbx, r9
14097721f: mov r9, qword ptr [rsp + 0x290]
140977227: mov r11, qword ptr [rsp + 0x268]
14097722f: mov r13, qword ptr [rsp + 0x240]
140977237: mov rax, qword ptr [rsp + 0x288]
14097723f: mov rdi, qword ptr [rax]
140977242: mov r12, qword ptr [rax + 0x20]
140977246: mov rbp, qword ptr [r9 + 0x20]
14097724a: mov r14, qword ptr [rcx]
14097724d: mov ecx, dword ptr [r14]
140977250: cmp ecx, 5
140977253: ja 0x140977429
140977259: mov r9, qword ptr [rsp + 0x280]
140977261: mov eax, dword ptr [r14 + 4]
140977265: mov r10d, 6
14097726b: bt r10d, ecx
14097726f: jb 0x1409772d9
140977271: mov r10d, 0x18
140977277: bt r10d, ecx
14097727b: jae 0x140977426
140977281: mov rcx, qword ptr [r9]
140977284: xor r9d, r9d
140977287: cmp r9, qword ptr [rcx + 0x30]
14097728b: jno 0x140977426
140977291: mov rcx, qword ptr [rsp + 0x270]
140977299: mov rcx, qword ptr [rcx]
14097729c: mov rsi, rdx
14097729f: mov edx, eax
1409772a1: mov r15, r8
1409772a4: mov r13, r11
1409772a7: call 0x140a1c930
1409772ac: mov r11, r13
1409772af: mov r13, qword ptr [rsp + 0x240]
1409772b7: mov r8, r15
1409772ba: mov rdx, rsi
1409772bd: test rax, rax
1409772c0: jne 0x140977426
1409772c6: mov eax, dword ptr [r13 + 0x1c]
1409772ca: mov rcx, qword ptr [r13 + 0x10]
1409772ce: mov dword ptr [rcx], eax
1409772d0: mov rsi, qword ptr [r13]
1409772d4: jmp 0x1409773b9
1409772d9: mov rsi, qword ptr [r14 + 8]
1409772dd: mov r15, qword ptr [r11]
1409772e0: mov qword ptr [rsp + 0xc0], rsi
1409772e8: cmp qword ptr [r15 + 0x18], 0
1409772ed: je 0x1409773a5
1409772f3: mov qword ptr [rsp + 0x50], r8
1409772f8: mov qword ptr [rsp + 0x40], rdx
1409772fd: lea rcx, [r15 + 0x20]
140977301: lea rdx, [rsp + 0xc0]
140977309: call 0x1405b0d70
14097730e: mov rcx, qword ptr [r15]
140977311: mov r9, qword ptr [r15 + 8]
140977315: mov r10, r9
140977318: and r10, rax
14097731b: shr rax, 0x39
14097731f: movd xmm0, eax
140977323: punpcklbw xmm0, xmm0
140977327: pshuflw xmm0, xmm0, 0
14097732c: pshufd xmm0, xmm0, 0
140977331: lea rax, [rcx - 0x2a8]
140977338: xor r13d, r13d
14097733b: pcmpeqd xmm1, xmm1
14097733f: mov r8, qword ptr [rsp + 0x50]
140977344: movdqu xmm2, xmmword ptr [rcx + r10]
14097734a: movdqa xmm3, xmm2
14097734e: pcmpeqb xmm3, xmm0
140977352: pmovmskb r11d, xmm3
140977357: test r11d, r11d
14097735a: je 0x140977389
14097735c: tzcnt edx, r11d
140977361: add rdx, r10
140977364: and rdx, r9
140977367: neg rdx
14097736a: imul r15, rdx, 0x2a8
140977371: cmp qword ptr [rax + r15], rsi
140977375: mov rdx, qword ptr [rsp + 0x40]
14097737a: je 0x1409773f0
14097737c: lea edx, [r11 - 1]
140977380: and dx, r11w
140977384: mov r11d, edx
140977387: jne 0x14097735c
140977389: pcmpeqb xmm2, xmm1
14097738d: pmovmskb edx, xmm2
140977391: test edx, edx
140977393: jne 0x1409773a5
140977395: add r10, r13
140977398: add r10, 0x10
14097739c: add r13, 0x10
1409773a0: and r10, r9
1409773a3: jmp 0x140977344
1409773a5: mov rdx, qword ptr [rsp + 0x240]
1409773ad: mov eax, dword ptr [rdx + 0x1c]
1409773b0: mov rcx, qword ptr [rdx + 0x10]
1409773b4: mov dword ptr [rcx], eax
1409773b6: mov rsi, qword ptr [rdx]
1409773b9: mov rdi, qword ptr [rsi + 0x30]
1409773bd: mov rbx, qword ptr [rsi + 0x40]
1409773c1: cmp rdi, qword ptr [rsi + 0x20]
1409773c5: jne 0x1409773d7
1409773c7: lea rcx, [rsi + 0x20]
1409773cb: lea rdx, [rip + 0x22c2d76]
1409773d2: call 0x140562db0
1409773d7: mov rax, qword ptr [rsi + 0x28]
1409773db: lea rcx, [rdi + rdi*2]
1409773df: mov dword ptr [rax + rcx*8], 0
1409773e6: mov qword ptr [rax + rcx*8 + 0x10], rbx
1409773eb: jmp 0x140977937
1409773f0: cmp qword ptr [rcx + r15 - 0x290], 2
1409773f9: jb 0x140977e54
1409773ff: movss xmm0, dword ptr [rcx + r15 - 0x270]
140977409: pxor xmm1, xmm1
14097740d: ucomiss xmm0, xmm1
140977410: jbe 0x140977e54
140977416: mov r13, qword ptr [rsp + 0x240]
14097741e: mov r11, qword ptr [rsp + 0x268]
140977426: mov ecx, dword ptr [r14]
140977429: cmp ecx, 0xa
14097742c: sete byte ptr [rbp]
140977430: mov rsi, qword ptr [rsp + 0x248]
140977438: mov rax, qword ptr [r8]
14097743b: cmp qword ptr [rax + 0x10], 0
140977440: jne 0x14097745b
140977442: mov rax, qword ptr [rbx]
140977445: mov ecx, dword ptr [rax + 0x90]
14097744b: lea r8, [rip + 0x213ff3a]
140977452: movsxd rcx, dword ptr [r8 + rcx*4]
140977456: add rcx, r8
140977459: jmp rcx
14097745b: mov eax, dword ptr [r14]
14097745e: cmp eax, 6
140977461: jae 0x1409776ae
140977467: mov qword ptr [rsp + 0x50], rdi
14097746c: mov rsi, r11
14097746f: mov r15, qword ptr [rsp + 0x258]
140977477: mov rcx, qword ptr [rdx]
14097747a: mov qword ptr [rsp + 0xc0], 0
140977486: mov qword ptr [rsp + 0xc8], 8
140977492: mov qword ptr [rsp + 0xd0], 0
14097749e: mov dword ptr [rsp + 0xd8], 0
1409774a9: mov dword ptr [rsp + 0xf0], 0
1409774b4: mov dword ptr [rsp + 0x108], 0
1409774bf: mov dword ptr [rsp + 0x120], 0
1409774ca: mov byte ptr [rsp + 0x138], 0
1409774d2: mov byte ptr [rsp + 0x148], 5
1409774da: mov dword ptr [rsp + 0x14c], 0x7f7fffff
1409774e5: mov word ptr [rsp + 0x168], 0x100
1409774ef: lea rdx, [rip + 0x213f4ea]
1409774f6: lea r9, [rsp + 0xc0]
1409774fe: mov r8d, 0x11
140977504: call 0x1409236d0
140977509: mov rdi, rax
14097750c: mov rax, qword ptr [r15]
14097750f: mov rcx, qword ptr [rax + 0x10]
140977513: mov qword ptr [rsp + 0x70], rcx
140977518: movups xmm0, xmmword ptr [rax]
14097751b: movaps xmmword ptr [rsp + 0x60], xmm0
140977520: mov rax, qword ptr [rbx]
140977523: mov ecx, dword ptr [rax + 8]
140977526: lea r8, [rip + 0x213fe9b]
14097752d: movsxd rcx, dword ptr [r8 + rcx*4]
140977531: add rcx, r8
140977534: jmp rcx
140977536: cmp byte ptr [rdi + 0x18], 2
14097753a: jb 0x140977640
140977540: movzx eax, byte ptr [rdi + 0x19]
140977544: cmp al, 1
140977546: ja 0x140977640
14097754c: test al, 1
14097754e: je 0x140977640
140977554: cmp qword ptr [rdi + 0x38], rdx
140977558: jne 0x140977640
14097755e: cmp dword ptr [r14], 6
140977562: jae 0x140977942
140977568: mov rax, qword ptr [rsp + 0x260]
140977570: mov rdx, qword ptr [rax]
140977573: mov dword ptr [rsp + 0x30], 0xfb
14097757b: mov dword ptr [rsp + 0x28], 0x7f7fffff
140977583: lea rcx, [rsp + 0xc0]
14097758b: lea r8, [rsp + 0x60]
140977590: xor r9d, r9d
140977593: call 0x1408cdc60
140977598: cmp dword ptr [rsp + 0xf4], 2
1409775a0: je 0x1409775f5
1409775a2: mov rax, qword ptr [rsp + 0xd0]
1409775aa: mov qword ptr [rsp + 0x190], rax
1409775b2: movups xmm0, xmmword ptr [rsp + 0xc0]
1409775ba: movaps xmmword ptr [rsp + 0x180], xmm0
1409775c2: mov rcx, qword ptr [rsp + 0x270]
1409775ca: mov rcx, qword ptr [rcx]
1409775cd: movaps xmmword ptr [rsp + 0x90], xmm0
1409775d5: mov qword ptr [rsp + 0xa0], rax
1409775dd: movzx eax, byte ptr [rsp + 0x90]
1409775e5: lea rdx, [rip + 0x213fe4c]
1409775ec: movsxd rax, dword ptr [rdx + rax*4]
1409775f0: add rax, rdx
1409775f3: jmp rax
1409775f5: mov eax, dword ptr [r13 + 0x1c]
1409775f9: mov rcx, qword ptr [r13 + 0x10]
1409775fd: mov dword ptr [rcx], eax
1409775ff: mov rsi, qword ptr [r13]
140977603: mov rdi, qword ptr [rsi + 0x30]
140977607: mov rbx, qword ptr [rsi + 0x40]
14097760b: cmp rdi, qword ptr [rsi + 0x20]
14097760f: jne 0x140977621
140977611: lea rcx, [rsi + 0x20]
140977615: lea rdx, [rip + 0x22c2b2c]
14097761c: call 0x140562db0
140977621: mov rax, qword ptr [rsi + 0x28]
140977625: lea rcx, [rdi + rdi*2]
140977629: mov dword ptr [rax + rcx*8], 0
140977630: mov qword ptr [rax + rcx*8 + 0x10], rbx
140977635: inc rdi
140977638: mov qword ptr [rsi + 0x30], rdi
14097763c: inc qword ptr [rsi + 0x40]
140977640: mov eax, dword ptr [r14]
140977643: dec eax
140977645: cmp eax, 2
140977648: jae 0x140977942
14097764e: mov rax, qword ptr [rsp + 0x250]
140977656: mov rdi, qword ptr [r14 + 8]
14097765a: mov ecx, dword ptr [rax + 0x1c]
14097765d: mov rdx, qword ptr [rax + 0x10]
140977661: mov dword ptr [rdx], ecx
140977663: mov rsi, qword ptr [rax]
140977666: mov rbx, qword ptr [rsi + 0x30]
14097766a: mov r14, qword ptr [rsi + 0x40]
14097766e: cmp rbx, qword ptr [rsi + 0x20]
140977672: jne 0x140977684
140977674: lea rcx, [rsi + 0x20]
140977678: lea rdx, [rip + 0x22c2ac9]
14097767f: call 0x140562db0
140977684: mov rax, qword ptr [rsi + 0x28]
140977688: lea rcx, [rbx + rbx*2]
14097768c: mov qword ptr [rax + rcx*8], r14
140977690: mov qword ptr [rax + rcx*8 + 8], rdi
140977695: mov dword ptr [rax + rcx*8 + 0x10], 0x3f800000
14097769d: mov byte ptr [rax + rcx*8 + 0x14], 2
1409776a2: inc rbx
1409776a5: mov qword ptr [rsi + 0x30], rbx
1409776a9: jmp 0x14097793e
1409776ae: mov rcx, qword ptr [rbx]
1409776b1: mov r8d, dword ptr [rcx + 8]
1409776b5: cmp eax, 0xa
1409776b8: jne 0x1409776e1
1409776ba: lea r9, [rip + 0x213fcdf]
1409776c1: movsxd r10, dword ptr [r9 + r8*4]
1409776c5: add r10, r9
1409776c8: jmp r10
1409776cb: xorps xmm0, xmm0
1409776ce: ucomiss xmm0, dword ptr [rcx + 0xc]
1409776d2: jb 0x1409778a4
1409776d8: test byte ptr [rdi], 1
1409776db: jne 0x140977467
1409776e1: lea rdx, [rip + 0x213fccc]
1409776e8: movsxd r8, dword ptr [rdx + r8*4]
1409776ec: add r8, rdx
1409776ef: jmp r8
1409776f2: cmp eax, 0x13
1409776f5: jne 0x1409778ba
1409776fb: jmp 0x140977942
140977700: xorps xmm0, xmm0
140977703: ucomiss xmm0, dword ptr [rax + 0xc]
140977707: jb 0x140977536
14097770d: mov qword ptr [rsp + 0x40], rdx
140977712: mov rax, qword ptr [rsp + 0x260]
14097771a: mov rdx, qword ptr [rax]
14097771d: mov dword ptr [rsp + 0x30], 0x3f43
140977725: mov dword ptr [rsp + 0x28], 0x7f7fffff
14097772d: xor r15d, r15d
140977730: lea rcx, [rsp + 0x90]
140977738: lea r8, [rsp + 0x60]
14097773d: xor r9d, r9d
140977740: call 0x1408ce180
140977745: movzx eax, byte ptr [rsp + 0x90]
14097774d: cmp eax, 0x11
140977750: jne 0x1409777d0
140977752: movss xmm0, dword ptr [rip + 0x1fae226]
14097775a: xor ebx, ebx
14097775c: mov rdx, qword ptr [rsp + 0x40]
140977761: jmp 0x1409779f5
140977766: xorps xmm0, xmm0
140977769: ucomiss xmm0, dword ptr [rax + 0x94]
140977770: jb 0x14097745b
140977776: mov eax, dword ptr [r13 + 0x1c]
14097777a: mov rcx, qword ptr [r13 + 0x10]
14097777e: mov dword ptr [rcx], eax
140977780: mov rdi, qword ptr [r13]
140977784: mov rbx, qword ptr [rdi + 0x30]
140977788: mov r15, qword ptr [rdi + 0x40]
14097778c: cmp rbx, qword ptr [rdi + 0x20]
140977790: jne 0x1409777a2
140977792: lea rcx, [rdi + 0x20]
140977796: lea rdx, [rip + 0x22c29ab]
14097779d: call 0x140562db0
1409777a2: mov rax, qword ptr [rdi + 0x28]
1409777a6: lea rcx, [rbx + rbx*2]
1409777aa: mov dword ptr [rax + rcx*8], 0
1409777b1: mov qword ptr [rax + rcx*8 + 0x10], r15
1409777b6: inc rbx
1409777b9: mov qword ptr [rdi + 0x30], rbx
1409777bd: inc qword ptr [rdi + 0x40]
1409777c1: cmp dword ptr [r14], 6
1409777c5: jae 0x140977905
1409777cb: jmp 0x140977942
1409777d0: mov r11, qword ptr [rsp + 0x98]
1409777d8: movss xmm7, dword ptr [rsp + 0xa8]
1409777e1: movss xmm6, dword ptr [rsp + 0xac]
1409777ea: movss xmm8, dword ptr [rsp + 0xb0]
1409777f4: cmp al, 0xc
1409777f6: jne 0x14097796f
1409777fc: mov rax, qword ptr [rsp + 0x278]
140977804: mov ecx, dword ptr [rax + 0x1c]
140977807: mov rdx, qword ptr [rax + 0x10]
14097780b: mov dword ptr [rdx], ecx
14097780d: mov rbx, qword ptr [rax]
140977810: mov r15, qword ptr [rbx + 0x30]
140977814: mov r13, qword ptr [rbx + 0x40]
140977818: cmp r15, qword ptr [rbx + 0x20]
14097781c: jne 0x140977838
14097781e: lea rcx, [rbx + 0x20]
140977822: lea rdx, [rip + 0x22c291f]
140977829: mov qword ptr [rsp + 0x48], r11
14097782e: call 0x140562630
140977833: mov r11, qword ptr [rsp + 0x48]
140977838: mov rax, qword ptr [rbx + 0x28]
14097783c: mov rcx, r15
14097783f: shl rcx, 4
140977843: mov qword ptr [rax + rcx], r13
140977847: mov qword ptr [rax + rcx + 8], r11
14097784c: inc r15
14097784f: mov qword ptr [rbx + 0x30], r15
140977853: inc qword ptr [rbx + 0x40]
140977857: movaps xmm0, xmmword ptr [rsp + 0x60]
14097785c: subss xmm6, dword ptr [rsp + 0x64]
140977862: mulss xmm6, xmm6
140977866: unpcklps xmm7, xmm8
14097786a: movss xmm1, dword ptr [rsp + 0x68]
140977870: movlhps xmm1, xmm0
140977873: shufps xmm1, xmm0, 0xe2
140977877: subps xmm7, xmm1
14097787a: mulps xmm7, xmm7
14097787d: addss xmm6, xmm7
140977881: movshdup xmm0, xmm7
140977885: addss xmm0, xmm6
140977889: sqrtss xmm0, xmm0
14097788d: xor ebx, ebx
14097788f: mov r13, qword ptr [rsp + 0x240]
140977897: mov rdx, qword ptr [rsp + 0x40]
14097789c: xor r15d, r15d
14097789f: jmp 0x1409779f5
1409778a4: cmp eax, 0x13
1409778a7: je 0x140977942
1409778ad: xorps xmm0, xmm0
1409778b0: ucomiss xmm0, dword ptr [rcx + 0xc]
1409778b4: jb 0x140977942
1409778ba: mov eax, dword ptr [r13 + 0x1c]
1409778be: mov rcx, qword ptr [r13 + 0x10]
1409778c2: mov dword ptr [rcx], eax
1409778c4: mov rdi, qword ptr [r13]
1409778c8: mov rbx, qword ptr [rdi + 0x30]
1409778cc: mov r14, qword ptr [rdi + 0x40]
1409778d0: cmp rbx, qword ptr [rdi + 0x20]
1409778d4: jne 0x1409778e6
1409778d6: lea rcx, [rdi + 0x20]
1409778da: lea rdx, [rip + 0x22c2867]
1409778e1: call 0x140562db0
1409778e6: mov rax, qword ptr [rdi + 0x28]
1409778ea: lea rcx, [rbx + rbx*2]
1409778ee: mov dword ptr [rax + rcx*8], 0
1409778f5: mov qword ptr [rax + rcx*8 + 0x10], r14
1409778fa: inc rbx
1409778fd: mov qword ptr [rdi + 0x30], rbx
140977901: inc qword ptr [rdi + 0x40]
140977905: mov eax, dword ptr [rsi + 0x1c]
140977908: mov rcx, qword ptr [rsi + 0x10]
14097790c: mov dword ptr [rcx], eax
14097790e: mov rsi, qword ptr [rsi]
140977911: mov rdi, qword ptr [rsi + 0x30]
140977915: mov rbx, qword ptr [rsi + 0x40]
140977919: cmp rdi, qword ptr [rsi + 0x20]
14097791d: jne 0x14097792f
14097791f: lea rcx, [rsi + 0x20]
140977923: lea rdx, [rip + 0x22c281e]
14097792a: call 0x140562870
14097792f: mov rax, qword ptr [rsi + 0x28]
140977933: mov qword ptr [rax + rdi*8], rbx
140977937: inc rdi
14097793a: mov qword ptr [rsi + 0x30], rdi
14097793e: inc qword ptr [rsi + 0x40]
140977942: movaps xmm6, xmmword ptr [rsp + 0x1a0]
14097794a: movaps xmm7, xmmword ptr [rsp + 0x1b0]
140977952: movaps xmm8, xmmword ptr [rsp + 0x1c0]
14097795b: add rsp, 0x1d8
140977962: pop rbx
140977963: pop rbp
140977964: pop rdi
140977965: pop rsi
140977966: pop r12
140977968: pop r13
14097796a: pop r14
14097796c: pop r15
14097796e: ret 
14097796f: mov r8, qword ptr [rsp + 0xa0]
140977977: mov byte ptr [rsp + 0x78], al
14097797b: mov ecx, dword ptr [rsp + 0x91]
140977982: mov edx, dword ptr [rsp + 0x94]
140977989: mov dword ptr [rsp + 0x7c], edx
14097798d: mov dword ptr [rsp + 0x79], ecx
140977991: mov qword ptr [rsp + 0x80], r11
140977999: mov qword ptr [rsp + 0x88], r8
1409779a1: xor ebx, ebx
1409779a3: mov eax, eax
1409779a5: lea rcx, [rip + 0x213fa30]
1409779ac: movsxd rax, dword ptr [rcx + rax*4]
1409779b0: add rax, rcx
1409779b3: mov rdx, qword ptr [rsp + 0x40]
1409779b8: jmp rax
1409779ba: mov ebx, 1
1409779bf: subss xmm6, dword ptr [rsp + 0x64]
1409779c5: movaps xmm0, xmmword ptr [rsp + 0x60]
1409779ca: mulss xmm6, xmm6
1409779ce: unpcklps xmm7, xmm8
1409779d2: movss xmm1, dword ptr [rsp + 0x68]
1409779d8: movlhps xmm1, xmm0
1409779db: shufps xmm1, xmm0, 0xe2
1409779df: subps xmm7, xmm1
1409779e2: mulps xmm7, xmm7
1409779e5: addss xmm6, xmm7
1409779e9: movshdup xmm0, xmm7
1409779ed: addss xmm0, xmm6
1409779f1: sqrtss xmm0, xmm0
1409779f5: mov r9d, dword ptr [r14]
1409779f8: mov eax, dword ptr [r14 + 4]
1409779fc: mov rcx, qword ptr [r14 + 8]
140977a00: cmp r9d, 0xa
140977a04: je 0x140977a17
140977a06: lea r8d, [r9 - 0x15]
140977a0a: mov r15d, r9d
140977a0d: cmp r8d, -0x10
140977a11: ja 0x140977536
140977a17: test byte ptr [r12], 1
140977a1c: je 0x140977a39
140977a1e: movss xmm1, dword ptr [r12 + 8]
140977a25: movd xmm3, dword ptr [r12 + 0xc]
140977a2c: movd xmm2, dword ptr [r12 + 4]
140977a33: punpckldq xmm2, xmm3
140977a37: jmp 0x140977a40
140977a39: pxor xmm2, xmm2
140977a3d: xorps xmm1, xmm1
140977a40: movaps xmm3, xmmword ptr [rsp + 0x60]
140977a45: subss xmm1, dword ptr [rsp + 0x64]
140977a4b: mulss xmm1, xmm1
140977a4f: movss xmm4, dword ptr [rsp + 0x68]
140977a55: movlhps xmm4, xmm3
140977a58: shufps xmm4, xmm3, 0xe2
140977a5c: subps xmm2, xmm4
140977a5f: mulps xmm2, xmm2
140977a62: addss xmm1, xmm2
140977a66: movshdup xmm2, xmm2
140977a6a: addss xmm2, xmm1
140977a6e: xorps xmm1, xmm1
140977a71: sqrtss xmm1, xmm2
140977a75: ucomiss xmm0, xmm1
140977a78: jbe 0x140977a8e
140977a7a: mov r8, qword ptr [rsp + 0x50]
140977a7f: cmp dword ptr [r8], 1
140977a83: jne 0x140977a8e
140977a85: mov r10d, dword ptr [r8 + 8]
140977a89: mov ebx, 5
140977a8e: cmp r15d, ebx
140977a91: mov qword ptr [rsp + 0x50], rdi
140977a96: jne 0x140977b4d
140977a9c: mov r9d, r15d
140977a9f: lea r8, [rip + 0x213f97a]
140977aa6: movsxd r9, dword ptr [r8 + r9*4]
140977aaa: add r9, r8
140977aad: jmp r9
140977ab0: cmp rcx, r11
140977ab3: je 0x140977ac3
140977ab5: jmp 0x140977b4d
140977aba: cmp eax, r10d
140977abd: jne 0x140977b4d
140977ac3: xor edi, edi
140977ac5: mov eax, dword ptr [r13 + 0x1c]
140977ac9: mov rcx, qword ptr [r13 + 0x10]
140977acd: mov dword ptr [rcx], eax
140977acf: mov rbx, qword ptr [r13]
140977ad3: mov r15, qword ptr [rbx + 0x30]
140977ad7: mov r12, qword ptr [rbx + 0x40]
140977adb: cmp r15, qword ptr [rbx + 0x20]
140977adf: jne 0x140977af6
140977ae1: lea rcx, [rbx + 0x20]
140977ae5: lea rdx, [rip + 0x22c265c]
140977aec: call 0x140562db0
140977af1: mov rdx, qword ptr [rsp + 0x40]
140977af6: mov rax, qword ptr [rbx + 0x28]
140977afa: lea rcx, [r15 + r15*2]
140977afe: mov dword ptr [rax + rcx*8], edi
140977b01: mov r8, qword ptr [rsp + 0xc0]
140977b09: mov qword ptr [rax + rcx*8 + 4], r8
140977b0e: mov r8d, dword ptr [rsp + 0xc8]
140977b16: mov dword ptr [rax + rcx*8 + 0xc], r8d
140977b1b: mov qword ptr [rax + rcx*8 + 0x10], r12
140977b20: inc r15
140977b23: mov qword ptr [rbx + 0x30], r15
140977b27: inc qword ptr [rbx + 0x40]
140977b2b: mov rdi, qword ptr [rsp + 0x50]
140977b30: jmp 0x140977536
140977b35: cmp eax, r10d
140977b38: jne 0x140977b4d
140977b3a: xor eax, eax
140977b3c: cmp byte ptr [rbp], 0
140977b40: mov edi, 0xa
140977b45: cmove edi, eax
140977b48: jmp 0x140977ac5
140977b4d: lea eax, [rbx - 1]
140977b50: cmp eax, 1
140977b53: mov qword ptr [rsp + 0x48], r11
140977b58: ja 0x140977c5a
140977b5e: mov rdi, qword ptr [rsi]
140977b61: mov qword ptr [rsp + 0xc0], r11
140977b69: cmp qword ptr [rdi + 0x18], 0
140977b6e: je 0x140977c22
140977b74: mov r12, r11
140977b77: mov esi, r10d
140977b7a: lea rcx, [rdi + 0x20]
140977b7e: lea rdx, [rsp + 0xc0]
140977b86: call 0x1405b0d70
140977b8b: mov rcx, qword ptr [rdi]
140977b8e: mov rdi, qword ptr [rdi + 8]
140977b92: mov r8, rdi
140977b95: and r8, rax
140977b98: shr rax, 0x39
140977b9c: movd xmm0, eax
140977ba0: punpcklbw xmm0, xmm0
140977ba4: pshuflw xmm0, xmm0, 0
140977ba9: pshufd xmm0, xmm0, 0
140977bae: lea rax, [rcx - 0x2a8]
140977bb5: xor r9d, r9d
140977bb8: pcmpeqd xmm1, xmm1
140977bbc: mov rdx, qword ptr [rsp + 0x40]
140977bc1: movdqu xmm2, xmmword ptr [rcx + r8]
140977bc7: movdqa xmm3, xmm2
140977bcb: pcmpeqb xmm3, xmm0
140977bcf: pmovmskb r10d, xmm3
140977bd4: test r10d, r10d
140977bd7: je 0x140977c01
140977bd9: tzcnt r11d, r10d
140977bde: add r11, r8
140977be1: and r11, rdi
140977be4: neg r11
140977be7: imul r11, r11, 0x2a8
140977bee: cmp qword ptr [rax + r11], r12
140977bf2: je 0x140977c2c
140977bf4: lea r11d, [r10 - 1]
140977bf8: and r11w, r10w
140977bfc: mov r10d, r11d
140977bff: jne 0x140977bd9
140977c01: pcmpeqb xmm2, xmm1
140977c05: pmovmskb r10d, xmm2
140977c0a: test r10d, r10d
140977c0d: mov r10d, esi
140977c10: jne 0x140977c22
140977c12: add r8, r9
140977c15: add r8, 0x10
140977c19: add r9, 0x10
140977c1d: and r8, rdi
140977c20: jmp 0x140977bc1
140977c22: xor edi, edi
140977c24: cmp r15d, 5
140977c28: je 0x140977c66
140977c2a: jmp 0x140977c9e
140977c2c: cmp qword ptr [rcx + r11 - 0x290], 2
140977c35: jb 0x140977e6c
140977c3b: movss xmm0, dword ptr [rcx + r11 - 0x270]
140977c45: pxor xmm1, xmm1
140977c49: ucomiss xmm0, xmm1
140977c4c: jbe 0x140977e6c
140977c52: mov rdx, qword ptr [rsp + 0x40]
140977c57: mov r10d, esi
140977c5a: cmp r15d, 5
140977c5e: jne 0x140977c9c
140977c60: mov edi, ebx
140977c62: test ebx, ebx
140977c64: jne 0x140977c74
140977c66: xor ebx, ebx
140977c68: cmp byte ptr [rbp], 0
140977c6c: mov edi, 0xa
140977c71: cmove edi, ebx
140977c74: mov eax, dword ptr [r13 + 0x1c]
140977c78: mov rcx, qword ptr [r13 + 0x10]
140977c7c: mov dword ptr [rcx], eax
140977c7e: mov rbp, r13
140977c81: mov r15, qword ptr [r13]
140977c85: mov r12, qword ptr [r15 + 0x30]
140977c89: mov r13, qword ptr [r15 + 0x40]
140977c8d: cmp r12, qword ptr [r15 + 0x20]
140977c91: jne 0x140977cde
140977c93: mov esi, r10d
140977c96: lea rcx, [r15 + 0x20]
140977c9a: jmp 0x140977cc6
140977c9c: mov edi, ebx
140977c9e: mov eax, dword ptr [r13 + 0x1c]
140977ca2: mov rcx, qword ptr [r13 + 0x10]
140977ca6: mov dword ptr [rcx], eax
140977ca8: mov rbp, r13
140977cab: mov r15, qword ptr [r13]
140977caf: mov r12, qword ptr [r15 + 0x30]
140977cb3: mov r13, qword ptr [r15 + 0x40]
140977cb7: cmp r12, qword ptr [r15 + 0x20]
140977cbb: jne 0x140977cdc
140977cbd: mov esi, r10d
140977cc0: lea rcx, [r15 + 0x20]
140977cc4: mov ebx, edi
140977cc6: lea rdx, [rip + 0x22c247b]
140977ccd: call 0x140562db0
140977cd2: mov rdx, qword ptr [rsp + 0x40]
140977cd7: mov r10d, esi
140977cda: jmp 0x140977cde
140977cdc: mov ebx, edi
140977cde: mov rax, qword ptr [r15 + 0x28]
140977ce2: lea rcx, [r12 + r12*2]
140977ce6: mov dword ptr [rax + rcx*8], edi
140977ce9: mov dword ptr [rax + rcx*8 + 4], r10d
140977cee: mov r8, qword ptr [rsp + 0x48]
140977cf3: mov qword ptr [rax + rcx*8 + 8], r8
140977cf8: mov qword ptr [rax + rcx*8 + 0x10], r13
140977cfd: inc r12
140977d00: mov qword ptr [r15 + 0x30], r12
140977d04: inc qword ptr [r15 + 0x40]
140977d08: cmp ebx, 5
140977d0b: mov r13, rbp
140977d0e: mov rdi, qword ptr [rsp + 0x50]
140977d13: jne 0x140977536
140977d19: mov r8, qword ptr [rsp + 0x290]
140977d21: mov eax, dword ptr [r8 + 0x1c]
140977d25: mov rcx, qword ptr [r8 + 0x10]
140977d29: mov dword ptr [rcx], eax
140977d2b: mov rdi, qword ptr [r8]
140977d2e: mov rbx, qword ptr [rdi + 0x30]
140977d32: mov r15, qword ptr [rdi + 0x40]
140977d36: cmp rbx, qword ptr [rdi + 0x20]
140977d3a: jne 0x140977d51
140977d3c: lea rcx, [rdi + 0x20]
140977d40: lea rdx, [rip + 0x22c2401]
140977d47: call 0x140562630
140977d4c: mov rdx, qword ptr [rsp + 0x40]
140977d51: mov rax, qword ptr [rdi + 0x28]
140977d55: mov rcx, rbx
140977d58: shl rcx, 4
140977d5c: mov qword ptr [rax + rcx], r15
140977d60: mov byte ptr [rax + rcx + 8], 2
140977d65: inc rbx
140977d68: mov qword ptr [rdi + 0x30], rbx
140977d6c: inc qword ptr [rdi + 0x40]
140977d70: mov rdi, qword ptr [rsp + 0x50]
140977d75: jmp 0x140977536
140977d7a: mov ebx, 4
140977d7f: mov r10d, dword ptr [rsp + 0x7c]
140977d84: jmp 0x1409779bf
140977d89: mov rax, qword ptr [rsp + 0x270]
140977d91: mov rcx, qword ptr [rax]
140977d94: mov r9d, dword ptr [rsp + 0x7c]
140977d99: mov qword ptr [rsp + 0x48], r11
140977d9e: mov rdx, r11
140977da1: mov dword ptr [rsp + 0x5c], r9d
140977da6: call 0x140a1c5f0
140977dab: test rax, rax
140977dae: je 0x140977df6
140977db0: movzx eax, byte ptr [rax + 0x46]
140977db4: add al, 0xc8
140977db6: cmp al, 8
140977db8: movzx ecx, al
140977dbb: mov eax, 6
140977dc0: cmovb eax, ecx
140977dc3: cmp al, 7
140977dc5: mov rdx, qword ptr [rsp + 0x40]
140977dca: je 0x140977dfd
140977dcc: movzx eax, al
140977dcf: cmp eax, 6
140977dd2: mov r10d, dword ptr [rsp + 0x5c]
140977dd7: mov r11, qword ptr [rsp + 0x48]
140977ddc: jne 0x1409779bf
140977de2: mov ebx, 3
140977de7: jmp 0x1409779bf
140977dec: mov ebx, 2
140977df1: jmp 0x1409779bf
140977df6: mov rdx, qword ptr [rsp + 0x40]
140977dfb: jmp 0x140977e07
140977dfd: mov ebx, 4
140977e02: mov r10d, dword ptr [rsp + 0x5c]
140977e07: mov r11, qword ptr [rsp + 0x48]
140977e0c: jmp 0x1409779bf
140977e11: mov r9d, dword ptr [rsp + 0x94]
140977e19: mov rdx, qword ptr [rsp + 0x98]
140977e21: mov r8, qword ptr [rsp + 0xa0]
140977e29: call 0x140a1c5f0
140977e2e: test rax, rax
140977e31: je 0x1409775f5
140977e37: movzx eax, byte ptr [rax + 0x46]
140977e3b: not al
140977e3d: test al, 0x38
140977e3f: sete cl
140977e42: test al, 0x3e
140977e44: setne al
140977e47: test cl, al
140977e49: jne 0x1409775f5
140977e4f: jmp 0x140977640
140977e54: lea rcx, [rip + 0x2162338]
140977e5b: lea r8, [rip + 0x213eb66]
140977e62: mov edx, 0x32
140977e67: call 0x1428d9430
140977e6c: lea rcx, [rip + 0x2162320]
140977e73: lea r8, [rip + 0x213eb7e]
140977e7a: mov edx, 0x32
140977e7f: call 0x1428d9430
140977e84: lea rax, [rsp + 0x78]
140977e89: mov qword ptr [rsp + 0x170], rax
140977e91: lea rax, [rip + 0x9808]
140977e98: mov qword ptr [rsp + 0x178], rax
140977ea0: lea rax, [rip + 0x213ebc9]
140977ea7: mov qword ptr [rsp + 0xc0], rax
140977eaf: mov qword ptr [rsp + 0xc8], 2
140977ebb: mov qword ptr [rsp + 0xe0], 0
140977ec7: lea rax, [rsp + 0x170]
140977ecf: jmp 0x140977f19
140977ed1: lea rax, [rsp + 0x90]
140977ed9: mov qword ptr [rsp + 0x78], rax
140977ede: lea rax, [rip + 0x97bb]
140977ee5: mov qword ptr [rsp + 0x80], rax
140977eed: lea rax, [rip + 0x213eb7c]
140977ef4: mov qword ptr [rsp + 0xc0], rax
140977efc: mov qword ptr [rsp + 0xc8], 2
140977f08: mov qword ptr [rsp + 0xe0], 0
140977f14: lea rax, [rsp + 0x78]
140977f19: mov qword ptr [rsp + 0xd0], rax
140977f21: mov qword ptr [rsp + 0xd8], 1
140977f2d: lea rdx, [rip + 0x213eb5c]
140977f34: lea rcx, [rsp + 0xc0]
140977f3c: call 0x1428d9390
140977f41: ud2 

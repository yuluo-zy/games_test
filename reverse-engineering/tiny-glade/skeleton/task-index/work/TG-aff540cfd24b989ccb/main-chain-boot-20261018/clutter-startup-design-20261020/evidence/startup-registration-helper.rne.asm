140ed9480: 55 push rbp
140ed9481: 4157 push r15
140ed9483: 4156 push r14
140ed9485: 4155 push r13
140ed9487: 4154 push r12
140ed9489: 56 push rsi
140ed948a: 57 push rdi
140ed948b: 53 push rbx
140ed948c: 4881ecd8070000 sub rsp, 0x7d8
140ed9493: 488dac2480000000 lea rbp, [rsp + 0x80]
140ed949b: 66440f7f8540070000 movdqa xmmword ptr [rbp + 0x740], xmm8
140ed94a4: 660f7fbd30070000 movdqa xmmword ptr [rbp + 0x730], xmm7
140ed94ac: 0f29b520070000 movaps xmmword ptr [rbp + 0x720], xmm6
140ed94b3: 48c78518070000feffffff mov qword ptr [rbp + 0x718], 0xfffffffffffffffe
140ed94be: 4889ce mov rsi, rcx
140ed94c1: 488b0db02e7502 mov rcx, qword ptr [rip + 0x2752eb0]
140ed94c8: 4c8d05e9200502 lea r8, [rip + 0x20520e9]
140ed94cf: 488d9517070000 lea rdx, [rbp + 0x717]
140ed94d6: e8359c0000 call 0x140ee3110
140ed94db: 4889c7 mov rdi, rax
140ed94de: 4889d3 mov rbx, rdx
140ed94e1: 4889c1 mov rcx, rax
140ed94e4: ff5228 call qword ptr [rdx + 0x28]
140ed94e7: 4889c1 mov rcx, rax
140ed94ea: ff5218 call qword ptr [rdx + 0x18]
140ed94ed: 48b8f1de68ef5cfec09d movabs rax, 0x9dc0fe5cef68def1
140ed94f7: 4831d0 xor rax, rdx
140ed94fa: 48b97713d038e6212845 movabs rcx, 0x452821e638d01377
140ed9504: 4831f9 xor rcx, rdi
140ed9507: 48f7e1 mul rcx
140ed950a: 4831c2 xor rdx, rax
140ed950d: 4889d0 mov rax, rdx
140ed9510: 48c1e839 shr rax, 0x39
140ed9514: 488b0e mov rcx, qword ptr [rsi]
140ed9517: 4889b508070000 mov qword ptr [rbp + 0x708], rsi
140ed951e: 4c8b6608 mov r12, qword ptr [rsi + 8]
140ed9522: 4d89e5 mov r13, r12
140ed9525: 488995f8060000 mov qword ptr [rbp + 0x6f8], rdx
140ed952c: 4921d5 and r13, rdx
140ed952f: 660f6ec0 movd xmm0, eax
140ed9533: 660f60c0 punpcklbw xmm0, xmm0
140ed9537: f20f70c000 pshuflw xmm0, xmm0, 0
140ed953c: 660f70f000 pshufd xmm6, xmm0, 0
140ed9541: 488db1c0fcffff lea rsi, [rcx - 0x340]
140ed9548: 31c0 xor eax, eax
140ed954a: 660f76ff pcmpeqd xmm7, xmm7
140ed954e: 48898d00070000 mov qword ptr [rbp + 0x700], rcx
140ed9555: 488985f0060000 mov qword ptr [rbp + 0x6f0], rax
140ed955c: f3460f6f0429 movdqu xmm8, xmmword ptr [rcx + r13]
140ed9562: 66410f6fc0 movdqa xmm0, xmm8
140ed9567: 660f74c6 pcmpeqb xmm0, xmm6
140ed956b: 66440fd7f0 pmovmskb r14d, xmm0
140ed9570: 4585f6 test r14d, r14d
140ed9573: 7445 je 0x140ed95ba
140ed9575: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
140ed9580: f3410fbcc6 tzcnt eax, r14d
140ed9585: 4c01e8 add rax, r13
140ed9588: 4c21e0 and rax, r12
140ed958b: 48f7d8 neg rax
140ed958e: 4c69f840030000 imul r15, rax, 0x340
140ed9595: 4e8b043e mov r8, qword ptr [rsi + r15]
140ed9599: 4e8b4c3e08 mov r9, qword ptr [rsi + r15 + 8]
140ed959e: 4889f9 mov rcx, rdi
140ed95a1: 4889da mov rdx, rbx
140ed95a4: e8b721e3ff call 0x140d0b760
140ed95a9: 84c0 test al, al
140ed95ab: 7540 jne 0x140ed95ed
140ed95ad: 418d46ff lea eax, [r14 - 1]
140ed95b1: 664421f0 and ax, r14w
140ed95b5: 4189c6 mov r14d, eax
140ed95b8: 75c6 jne 0x140ed9580
140ed95ba: 66440f74c7 pcmpeqb xmm8, xmm7
140ed95bf: 66410fd7c0 pmovmskb eax, xmm8
140ed95c4: 85c0 test eax, eax
140ed95c6: 488b85f0060000 mov rax, qword ptr [rbp + 0x6f0]
140ed95cd: 0f85a2000000 jne 0x140ed9675
140ed95d3: 4901c5 add r13, rax
140ed95d6: 4983c510 add r13, 0x10
140ed95da: 4883c010 add rax, 0x10
140ed95de: 4d21e5 and r13, r12
140ed95e1: 488b8d00070000 mov rcx, qword ptr [rbp + 0x700]
140ed95e8: e968ffffff jmp 0x140ed9555
140ed95ed: 488b9d00070000 mov rbx, qword ptr [rbp + 0x700]
140ed95f4: 4c01fb add rbx, r15
140ed95f7: 488bb508070000 mov rsi, qword ptr [rbp + 0x708]
140ed95fe: 4881c3d0fcffff add rbx, -0x330
140ed9605: 488d7da8 lea rdi, [rbp - 0x58]
140ed9609: 4889f9 mov rcx, rdi
140ed960c: e8ff8ddd00 call 0x141cb2410
140ed9611: 488d8de8020000 lea rcx, [rbp + 0x2e8]
140ed9618: 4889da mov rdx, rbx
140ed961b: 4989f8 mov r8, rdi
140ed961e: 4531c9 xor r9d, r9d
140ed9621: e89ae8ffff call 0x140ed7ec0
140ed9626: 488b95e8020000 mov rdx, qword ptr [rbp + 0x2e8]
140ed962d: 4885d2 test rdx, rdx
140ed9630: 7416 je 0x140ed9648
140ed9632: 488b8df0020000 mov rcx, qword ptr [rbp + 0x2f0]
140ed9639: 48c1e204 shl rdx, 4
140ed963d: 41b808000000 mov r8d, 8
140ed9643: e8d8a573ff call 0x140613c20
140ed9648: 4889f0 mov rax, rsi
140ed964b: 0f28b520070000 movaps xmm6, xmmword ptr [rbp + 0x720]
140ed9652: 0f28bd30070000 movaps xmm7, xmmword ptr [rbp + 0x730]
140ed9659: 440f288540070000 movaps xmm8, xmmword ptr [rbp + 0x740]
140ed9661: 4881c4d8070000 add rsp, 0x7d8
140ed9668: 5b pop rbx
140ed9669: 5f pop rdi
140ed966a: 5e pop rsi
140ed966b: 415c pop r12
140ed966d: 415d pop r13
140ed966f: 415e pop r14
140ed9671: 415f pop r15
140ed9673: 5d pop rbp
140ed9674: c3 ret 
140ed9675: 488b0dfc2c7502 mov rcx, qword ptr [rip + 0x2752cfc]
140ed967c: 4c8d05351f0502 lea r8, [rip + 0x2051f35]
140ed9683: 488d9517070000 lea rdx, [rbp + 0x717]
140ed968a: e8819a0000 call 0x140ee3110
140ed968f: 4889c6 mov rsi, rax
140ed9692: 4989d4 mov r12, rdx
140ed9695: 488d4da8 lea rcx, [rbp - 0x58]
140ed9699: e8a27ee5ff call 0x140d31540
140ed969e: 48c7851806000000000000 mov qword ptr [rbp + 0x618], 0
140ed96a9: 48c7852006000008000000 mov qword ptr [rbp + 0x620], 8
140ed96b4: 660fefc0 pxor xmm0, xmm0
140ed96b8: f30f7f8528060000 movdqu xmmword ptr [rbp + 0x628], xmm0
140ed96c0: 48c7853806000008000000 mov qword ptr [rbp + 0x638], 8
140ed96cb: f30f7f8540060000 movdqu xmmword ptr [rbp + 0x640], xmm0
140ed96d3: 48c7855006000008000000 mov qword ptr [rbp + 0x650], 8
140ed96de: f30f7f8558060000 movdqu xmmword ptr [rbp + 0x658], xmm0
140ed96e6: 48c7856806000008000000 mov qword ptr [rbp + 0x668], 8
140ed96f1: f30f7f8570060000 movdqu xmmword ptr [rbp + 0x670], xmm0
140ed96f9: 48c7858006000008000000 mov qword ptr [rbp + 0x680], 8
140ed9704: f30f7f8588060000 movdqu xmmword ptr [rbp + 0x688], xmm0
140ed970c: 48c7859806000008000000 mov qword ptr [rbp + 0x698], 8
140ed9717: f30f7f85a0060000 movdqu xmmword ptr [rbp + 0x6a0], xmm0
140ed971f: 48c785b006000008000000 mov qword ptr [rbp + 0x6b0], 8
140ed972a: f30f7f85b8060000 movdqu xmmword ptr [rbp + 0x6b8], xmm0
140ed9732: 48c785c806000008000000 mov qword ptr [rbp + 0x6c8], 8
140ed973d: f30f7f85d0060000 movdqu xmmword ptr [rbp + 0x6d0], xmm0
140ed9745: 48c785e006000008000000 mov qword ptr [rbp + 0x6e0], 8
140ed9750: 48c785e806000000000000 mov qword ptr [rbp + 0x6e8], 0
140ed975b: b102 mov cl, 2
140ed975d: e8fe5ee5ff call 0x140d2f660
140ed9762: 4989c5 mov r13, rax
140ed9765: 4989d7 mov r15, rdx
140ed9768: 4889b5f0050000 mov qword ptr [rbp + 0x5f0], rsi
140ed976f: 4c89a5f8050000 mov qword ptr [rbp + 0x5f8], r12
140ed9776: 4c8da5e8020000 lea r12, [rbp + 0x2e8]
140ed977d: 488d55a8 lea rdx, [rbp - 0x58]
140ed9781: 41b830020000 mov r8d, 0x230
140ed9787: 4c89e1 mov rcx, r12
140ed978a: e871a4a401 call 0x142923c00
140ed978f: 488d8d18050000 lea rcx, [rbp + 0x518]
140ed9796: 488d9518060000 lea rdx, [rbp + 0x618]
140ed979d: 41b8d8000000 mov r8d, 0xd8
140ed97a3: e858a4a401 call 0x142923c00
140ed97a8: 4c89ad00060000 mov qword ptr [rbp + 0x600], r13
140ed97af: 4c89bd08060000 mov qword ptr [rbp + 0x608], r15
140ed97b6: c6851006000000 mov byte ptr [rbp + 0x610], 0
140ed97bd: 48ba0101010001000000 movabs rdx, 0x100010101
140ed97c7: 4c89e1 mov rcx, r12
140ed97ca: e87162e5ff call 0x140d2fa40
140ed97cf: 488d4db8 lea rcx, [rbp - 0x48]
140ed97d3: 488d95e8020000 lea rdx, [rbp + 0x2e8]
140ed97da: 41b830030000 mov r8d, 0x330
140ed97e0: e81ba4a401 call 0x142923c00
140ed97e5: 48897da8 mov qword ptr [rbp - 0x58], rdi
140ed97e9: 48895db0 mov qword ptr [rbp - 0x50], rbx
140ed97ed: 488bb508070000 mov rsi, qword ptr [rbp + 0x708]
140ed97f4: 4c8d4e20 lea r9, [rsi + 0x20]
140ed97f8: 4c8d45a8 lea r8, [rbp - 0x58]
140ed97fc: 4889f1 mov rcx, rsi
140ed97ff: 488b95f8060000 mov rdx, qword ptr [rbp + 0x6f8]
140ed9806: e8e50a0000 call 0x140eda2f0
140ed980b: 4889c3 mov rbx, rax
140ed980e: e9ebfdffff jmp 0x140ed95fe
140ed9813: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140ed9820: 4889542410 mov qword ptr [rsp + 0x10], rdx
140ed9825: 55 push rbp
140ed9826: 4157 push r15
140ed9828: 4156 push r14
140ed982a: 4155 push r13
140ed982c: 4154 push r12
140ed982e: 56 push rsi
140ed982f: 57 push rdi
140ed9830: 53 push rbx
140ed9831: 4883ec58 sub rsp, 0x58
140ed9835: 488daa80000000 lea rbp, [rdx + 0x80]
140ed983c: 66440f7f442420 movdqa xmmword ptr [rsp + 0x20], xmm8
140ed9843: 660f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm7
140ed9849: 660f7f742440 movdqa xmmword ptr [rsp + 0x40], xmm6
140ed984f: 488d8d18060000 lea rcx, [rbp + 0x618]
140ed9856: e895f771ff call 0x1405f8ff0
140ed985b: 488d4da8 lea rcx, [rbp - 0x58]
140ed985f: e84cb6ffff call 0x140ed4eb0
140ed9864: 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140ed9869: 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140ed986e: 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
140ed9874: 4883c458 add rsp, 0x58
140ed9878: 5b pop rbx
140ed9879: 5f pop rdi
140ed987a: 5e pop rsi
140ed987b: 415c pop r12
140ed987d: 415d pop r13
140ed987f: 415e pop r14
140ed9881: 415f pop r15
140ed9883: 5d pop rbp
140ed9884: c3 ret 
140ed9885: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
140ed9890: 4889542410 mov qword ptr [rsp + 0x10], rdx
140ed9895: 55 push rbp
140ed9896: 4157 push r15
140ed9898: 4156 push r14
140ed989a: 4155 push r13
140ed989c: 4154 push r12
140ed989e: 56 push rsi
140ed989f: 57 push rdi
140ed98a0: 53 push rbx
140ed98a1: 4883ec58 sub rsp, 0x58
140ed98a5: 488daa80000000 lea rbp, [rdx + 0x80]
140ed98ac: 66440f7f442420 movdqa xmmword ptr [rsp + 0x20], xmm8
140ed98b3: 660f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm7
140ed98b9: 660f7f742440 movdqa xmmword ptr [rsp + 0x40], xmm6
140ed98bf: 488d8de8020000 lea rcx, [rbp + 0x2e8]
140ed98c6: e825b4ffff call 0x140ed4cf0
140ed98cb: 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140ed98d0: 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140ed98d5: 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
140ed98db: 4883c458 add rsp, 0x58
140ed98df: 5b pop rbx
140ed98e0: 5f pop rdi
140ed98e1: 5e pop rsi
140ed98e2: 415c pop r12
140ed98e4: 415d pop r13
140ed98e6: 415e pop r14
140ed98e8: 415f pop r15
140ed98ea: 5d pop rbp
140ed98eb: c3 ret 

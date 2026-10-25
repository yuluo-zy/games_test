141620320: push rbp
141620321: push r15
141620323: push r14
141620325: push r12
141620327: push rsi
141620328: push rdi
141620329: push rbx
14162032a: sub rsp, 0x130
141620331: lea rbp, [rsp + 0x80]
141620339: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
141620344: mov rdi, rdx
141620347: mov rsi, rcx
14162034a: test r8, r8
14162034d: je 0x141620389
14162034f: mov rbx, r9
141620352: mov rax, r9
141620355: and rax, 3
141620359: mov qword ptr [rbp - 0x40], rax
14162035d: jne 0x1416205ae
141620363: mov r15, r8
141620366: mov rax, r8
141620369: and rax, 3
14162036d: mov qword ptr [rbp - 0x40], rax
141620371: jne 0x1416205bf
141620377: cmp rbx, 0x8004
14162037e: jae 0x14162065a
141620384: xor r12d, r12d
141620387: jmp 0x141620403
141620389: movzx eax, byte ptr [rip + 0x2014879]
141620390: mov ecx, 0x8000
141620395: mov edx, 4
14162039a: call 0x140613c10
14162039f: test rax, rax
1416203a2: je 0x1416206c4
1416203a8: mov r15, rax
1416203ab: mov eax, 0x1c
1416203b0: xorps xmm0, xmm0
1416203b3: nop word ptr cs:[rax + rax]
1416203c0: movups xmmword ptr [r15 + rax*4 - 0x70], xmm0
1416203c6: movups xmmword ptr [r15 + rax*4 - 0x60], xmm0
1416203cc: movups xmmword ptr [r15 + rax*4 - 0x50], xmm0
1416203d2: movups xmmword ptr [r15 + rax*4 - 0x40], xmm0
1416203d8: movups xmmword ptr [r15 + rax*4 - 0x30], xmm0
1416203de: movups xmmword ptr [r15 + rax*4 - 0x20], xmm0
1416203e4: movups xmmword ptr [r15 + rax*4 - 0x10], xmm0
1416203ea: movups xmmword ptr [r15 + rax*4], xmm0
1416203ef: add rax, 0x20
1416203f3: cmp rax, 0x201c
1416203f9: jne 0x1416203c0
1416203fb: mov r12b, 1
1416203fe: mov ebx, 0x8000
141620403: mov qword ptr [rbp + 0x98], r15
14162040a: mov byte ptr [rbp + 0xa7], r12b
141620411: lea rcx, [rbp + 0x10]
141620415: mov edx, 0x8000
14162041a: mov r8d, 0x22
141620420: mov r9b, 4
141620423: call 0x140bd4de0
141620428: mov qword ptr [rbp + 0x98], r15
14162042f: mov byte ptr [rbp + 0xa7], r12b
141620436: lea r14, [rbp + 0x60]
14162043a: lea rdx, [rbp + 0x10]
14162043e: mov r8d, 4
141620444: mov rcx, r14
141620447: call 0x140bd4f20
14162044c: mov qword ptr [rbp + 0x98], r15
141620453: mov byte ptr [rbp + 0xa7], r12b
14162045a: mov qword ptr [rsp + 0x30], rbx
14162045f: mov qword ptr [rsp + 0x28], r15
141620464: mov qword ptr [rsp + 0x20], r14
141620469: lea r8, [rip + 0x165fc24]
141620470: lea rcx, [rbp - 0x40]
141620474: mov r9d, 0xf
14162047a: mov rdx, rdi
14162047d: call 0x1416a2dc0
141620482: cmp dword ptr [rbp - 0x40], 0xb
141620486: jne 0x1416205e9
14162048c: mov rax, qword ptr [rbp - 0x18]
141620490: mov qword ptr [rbp + 0x80], rax
141620497: movups xmm0, xmmword ptr [rbp - 0x38]
14162049b: movups xmm1, xmmword ptr [rbp - 0x28]
14162049f: movaps xmmword ptr [rbp + 0x70], xmm1
1416204a3: movaps xmmword ptr [rbp + 0x60], xmm0
1416204a7: test r12b, r12b
1416204aa: je 0x1416204bf
1416204ac: mov edx, 0x8000
1416204b1: mov r8d, 4
1416204b7: mov rcx, r15
1416204ba: call 0x140613c20
1416204bf: lea rcx, [rbp + 0x60]
1416204c3: mov rdx, rdi
1416204c6: call 0x140bd4cd0
1416204cb: test rax, rax
1416204ce: je 0x141620649
1416204d4: cmp rdx, qword ptr [rbp + 0x78]
1416204d8: jb 0x141620533
1416204da: mov rcx, rdx
1416204dd: and rcx, 3
1416204e1: mov qword ptr [rbp - 0x40], rcx
1416204e5: jne 0x141620550
1416204e7: mov rcx, rax
1416204ea: and rcx, 3
1416204ee: mov qword ptr [rbp - 0x40], rcx
1416204f2: jne 0x14162057f
1416204f8: mov rcx, qword ptr [rbp + 0x80]
1416204ff: mov qword ptr [rsi + 0x30], rcx
141620503: movaps xmm0, xmmword ptr [rbp + 0x60]
141620507: movaps xmm1, xmmword ptr [rbp + 0x70]
14162050b: movups xmmword ptr [rsi + 0x20], xmm1
14162050f: movups xmmword ptr [rsi + 0x10], xmm0
141620513: shr rdx, 2
141620517: mov qword ptr [rsi], rax
14162051a: mov qword ptr [rsi + 8], rdx
14162051e: mov rax, rsi
141620521: add rsp, 0x130
141620528: pop rbx
141620529: pop rdi
14162052a: pop rsi
14162052b: pop r12
14162052d: pop r14
14162052f: pop r15
141620531: pop rbp
141620532: ret 
141620533: lea rcx, [rip + 0x165fcb6]
14162053a: lea r8, [rip + 0x165fce7]
141620541: mov edx, 0x37
141620546: call 0x1428d9430
14162054b: jmp 0x1416206e1
141620550: mov qword ptr [rbp + 0x10], 0
141620558: lea rax, [rip + 0x165fce9]
14162055f: mov qword ptr [rsp + 0x20], rax
141620564: lea r8, [rip + 0x165fcd5]
14162056b: lea rdx, [rbp - 0x40]
14162056f: lea r9, [rbp + 0x10]
141620573: xor ecx, ecx
141620575: call 0x1428d9596
14162057a: jmp 0x1416206e1
14162057f: mov qword ptr [rbp + 0x10], 0
141620587: lea rax, [rip + 0x165fcd2]
14162058e: mov qword ptr [rsp + 0x20], rax
141620593: lea r8, [rip + 0x165fca6]
14162059a: lea rdx, [rbp - 0x40]
14162059e: lea r9, [rbp + 0x10]
1416205a2: xor ecx, ecx
1416205a4: call 0x1428d9596
1416205a9: jmp 0x1416206e1
1416205ae: mov qword ptr [rbp + 0x10], 0
1416205b6: lea rax, [rip + 0x16a998b]
1416205bd: jmp 0x1416205ce
1416205bf: mov qword ptr [rbp + 0x10], 0
1416205c7: lea rax, [rip + 0x16a9992]
1416205ce: mov qword ptr [rsp + 0x20], rax
1416205d3: lea r8, [rip + 0x16a9966]
1416205da: lea rdx, [rbp - 0x40]
1416205de: lea r9, [rbp + 0x10]
1416205e2: xor ecx, ecx
1416205e4: call 0x1428d9596
1416205e9: mov dword ptr [rbp + 0x94], r12d
1416205f0: mov qword ptr [rbp - 0x48], r15
1416205f4: movups xmm0, xmmword ptr [rbp]
1416205f8: movaps xmmword ptr [rbp + 0x50], xmm0
1416205fc: movups xmm0, xmmword ptr [rbp - 0x40]
141620600: movups xmm1, xmmword ptr [rbp - 0x30]
141620604: movups xmm2, xmmword ptr [rbp - 0x20]
141620608: movups xmm3, xmmword ptr [rbp - 0x10]
14162060c: movaps xmmword ptr [rbp + 0x40], xmm3
141620610: movaps xmmword ptr [rbp + 0x30], xmm2
141620614: movaps xmmword ptr [rbp + 0x20], xmm1
141620618: movaps xmmword ptr [rbp + 0x10], xmm0
14162061c: lea rax, [rip + 0x165fb85]
141620623: mov qword ptr [rsp + 0x20], rax
141620628: lea rcx, [rip + 0x165e4b9]
14162062f: lea r9, [rip + 0x165e492]
141620636: lea r8, [rbp + 0x10]
14162063a: mov edx, 0x2b
14162063f: call 0x1428d9760
141620644: jmp 0x1416206e1
141620649: lea rcx, [rip + 0x165fb88]
141620650: call 0x1428d9310
141620655: jmp 0x1416206e1
14162065a: lea rax, [rip + 0x147ce60]
141620661: mov qword ptr [rbp - 0x40], rax
141620665: mov qword ptr [rbp - 0x38], 0x3f
14162066d: lea rax, [rbp - 0x40]
141620671: mov qword ptr [rbp + 0x60], rax
141620675: lea rax, [rip - 0x15167ec]
14162067c: mov qword ptr [rbp + 0x68], rax
141620680: lea rax, [rip + 0x165f141]
141620687: mov qword ptr [rbp + 0x10], rax
14162068b: mov qword ptr [rbp + 0x18], 1
141620693: mov qword ptr [rbp + 0x30], 0
14162069b: lea rax, [rbp + 0x60]
14162069f: mov qword ptr [rbp + 0x20], rax
1416206a3: mov qword ptr [rbp + 0x28], 1
1416206ab: mov byte ptr [rbp + 0xa7], 0
1416206b2: lea rdx, [rip + 0x165f11f]
1416206b9: lea rcx, [rbp + 0x10]
1416206bd: call 0x1428d9390
1416206c2: jmp 0x1416206e1
1416206c4: mov byte ptr [rbp + 0xa7], 0
1416206cb: lea r8, [rip + 0x165e3de]
1416206d2: mov ecx, 4
1416206d7: mov edx, 0x8000
1416206dc: call 0x1428d8fe3
1416206e1: ud2 
1416206e3: nop word ptr cs:[rax + rax]
1416206f0: mov qword ptr [rsp + 0x10], rdx
1416206f5: push rbp
1416206f6: push r15
1416206f8: push r14
1416206fa: push r12
1416206fc: push rsi
1416206fd: push rdi
1416206fe: push rbx
1416206ff: sub rsp, 0x40
141620703: lea rbp, [rdx + 0x80]
14162070a: mov rsi, qword ptr [rbp - 0x48]
14162070e: mov qword ptr [rbp + 0x98], rsi
141620715: mov edi, dword ptr [rbp + 0x94]
14162071b: mov byte ptr [rbp + 0xa7], dil
141620722: lea rcx, [rbp + 0x10]
141620726: call 0x1407cea70
14162072b: mov qword ptr [rbp + 0x98], rsi
141620732: mov byte ptr [rbp + 0xa7], dil
141620739: add rsp, 0x40
14162073d: pop rbx
14162073e: pop rdi
14162073f: pop rsi
141620740: pop r12
141620742: pop r14
141620744: pop r15
141620746: pop rbp
141620747: ret 
141620748: nop dword ptr [rax + rax]
141620750: mov qword ptr [rsp + 0x10], rdx
141620755: push rbp
141620756: push r15
141620758: push r14
14162075a: push r12
14162075c: push rsi
14162075d: push rdi
14162075e: push rbx
14162075f: sub rsp, 0x40
141620763: lea rbp, [rdx + 0x80]
14162076a: test byte ptr [rbp + 0xa7], 1
141620771: je 0x14162078a
141620773: mov rcx, qword ptr [rbp + 0x98]
14162077a: mov edx, 0x8000
14162077f: mov r8d, 4
141620785: call 0x140613c20
14162078a: nop 
14162078b: add rsp, 0x40
14162078f: pop rbx
141620790: pop rdi
141620791: pop rsi
141620792: pop r12
141620794: pop r14
141620796: pop r15
141620798: pop rbp
141620799: ret 
14162079a: nop word ptr [rax + rax]
1416207a0: mov qword ptr [rsp + 0x10], rdx
1416207a5: push rbp
1416207a6: push r15
1416207a8: push r14
1416207aa: push r12
1416207ac: push rsi
1416207ad: push rdi
1416207ae: push rbx
1416207af: sub rsp, 0x40
1416207b3: lea rbp, [rdx + 0x80]
1416207ba: lea rcx, [rbp + 0x60]
1416207be: call 0x140be24b0
1416207c3: nop 
1416207c4: add rsp, 0x40
1416207c8: pop rbx
1416207c9: pop rdi
1416207ca: pop rsi
1416207cb: pop r12
1416207cd: pop r14
1416207cf: pop r15
1416207d1: pop rbp
1416207d2: ret 

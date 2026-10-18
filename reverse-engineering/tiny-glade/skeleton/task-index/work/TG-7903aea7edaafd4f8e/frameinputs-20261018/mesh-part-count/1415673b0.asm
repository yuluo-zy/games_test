1415673b0: push rbp
1415673b1: push r15
1415673b3: push r14
1415673b5: push r13
1415673b7: push r12
1415673b9: push rsi
1415673ba: push rdi
1415673bb: push rbx
1415673bc: sub rsp, 0x148
1415673c3: lea rbp, [rsp + 0x80]
1415673cb: mov qword ptr [rbp + 0xc0], 0xfffffffffffffffe
1415673d6: mov qword ptr [rbp + 0xb0], r8
1415673dd: mov r15, r9
1415673e0: shl r15, 4
1415673e4: xor r13d, r13d
1415673e7: mov rax, r9
1415673ea: shr rax, 0x3c
1415673ee: jne 0x14156782e
1415673f4: movabs rdi, 0x7ffffffffffffffc
1415673fe: cmp r15, rdi
141567401: ja 0x14156782e
141567407: mov r14, r9
14156740a: mov rbx, rdx
14156740d: mov rsi, rcx
141567410: test r15, r15
141567413: je 0x14156745c
141567415: movzx eax, byte ptr [rip + 0x20cd7ed]
14156741c: mov r13d, 4
141567422: mov edx, 4
141567427: mov rcx, r15
14156742a: call 0x140613c10
14156742f: test rax, rax
141567432: je 0x14156782e
141567438: mov r12, rax
14156743b: mov r13, r14
14156743e: cmp r14, 2
141567442: jb 0x14156746b
141567444: lea r8, [r15 - 0x10]
141567448: mov rcx, r12
14156744b: xor edx, edx
14156744d: call 0x142923c20
141567452: lea rax, [r12 + r15]
141567456: add rax, -0x10
14156745a: jmp 0x141567473
14156745c: mov r12d, 4
141567462: xor r13d, r13d
141567465: cmp r14, 2
141567469: jae 0x141567444
14156746b: mov rax, r12
14156746e: test r14, r14
141567471: je 0x141567479
141567473: xorps xmm0, xmm0
141567476: movups xmmword ptr [rax], xmm0
141567479: mov qword ptr [rbp + 0x50], r13
14156747d: mov qword ptr [rbp + 0x58], r12
141567481: mov qword ptr [rbp + 0x60], r14
141567485: lea rcx, [rbp - 0x10]
141567489: lea r8, [rbp + 0x50]
14156748d: mov rdx, rbx
141567490: call 0x141691810
141567495: xorps xmm0, xmm0
141567498: movaps xmmword ptr [rbp + 0x60], xmm0
14156749c: movaps xmmword ptr [rbp + 0x50], xmm0
1415674a0: mov qword ptr [rbp + 0x70], 0
1415674a8: xor r12d, r12d
1415674ab: mov ecx, 0x30
1415674b0: mov rax, r14
1415674b3: mul rcx
1415674b6: mov r15, rax
1415674b9: jo 0x141567840
1415674bf: cmp r15, rdi
1415674c2: ja 0x141567840
1415674c8: test r15, r15
1415674cb: je 0x141567515
1415674cd: movzx eax, byte ptr [rip + 0x20cd735]
1415674d4: mov r12d, 4
1415674da: mov edx, 4
1415674df: mov rcx, r15
1415674e2: call 0x140613c10
1415674e7: test rax, rax
1415674ea: je 0x141567840
1415674f0: mov rcx, r14
1415674f3: cmp r14, 2
1415674f7: jb 0x141567522
1415674f9: lea r9, [r14 - 1]
1415674fd: lea r8, [r14 - 2]
141567501: mov edx, r9d
141567504: and edx, 7
141567507: cmp r8, 7
14156750b: jae 0x141567533
14156750d: mov r8, rax
141567510: jmp 0x14156765f
141567515: mov eax, 4
14156751a: xor ecx, ecx
14156751c: cmp r14, 2
141567520: jae 0x1415674f9
141567522: mov r8, rax
141567525: test r14, r14
141567528: jne 0x141567690
14156752e: jmp 0x1415676b7
141567533: and r9, 0xfffffffffffffff8
141567537: movsd xmm0, qword ptr [rip + 0x15797e1]
14156753f: xorps xmm1, xmm1
141567542: mov r8, rax
141567545: nop word ptr cs:[rax + rax]
141567550: movsd qword ptr [r8], xmm0
141567555: movups xmmword ptr [r8 + 8], xmm1
14156755a: movups xmmword ptr [r8 + 0x18], xmm1
14156755f: mov qword ptr [r8 + 0x28], 0
141567567: movsd qword ptr [r8 + 0x30], xmm0
14156756d: movups xmmword ptr [r8 + 0x38], xmm1
141567572: movups xmmword ptr [r8 + 0x48], xmm1
141567577: mov qword ptr [r8 + 0x58], 0
14156757f: movsd qword ptr [r8 + 0x60], xmm0
141567585: movups xmmword ptr [r8 + 0x68], xmm1
14156758a: movups xmmword ptr [r8 + 0x78], xmm1
14156758f: mov qword ptr [r8 + 0x88], 0
14156759a: movsd qword ptr [r8 + 0x90], xmm0
1415675a3: movups xmmword ptr [r8 + 0x98], xmm1
1415675ab: movups xmmword ptr [r8 + 0xa8], xmm1
1415675b3: mov qword ptr [r8 + 0xb8], 0
1415675be: movsd qword ptr [r8 + 0xc0], xmm0
1415675c7: movups xmmword ptr [r8 + 0xc8], xmm1
1415675cf: movups xmmword ptr [r8 + 0xd8], xmm1
1415675d7: mov qword ptr [r8 + 0xe8], 0
1415675e2: movsd qword ptr [r8 + 0xf0], xmm0
1415675eb: movups xmmword ptr [r8 + 0xf8], xmm1
1415675f3: movups xmmword ptr [r8 + 0x108], xmm1
1415675fb: mov qword ptr [r8 + 0x118], 0
141567606: movsd qword ptr [r8 + 0x120], xmm0
14156760f: movups xmmword ptr [r8 + 0x128], xmm1
141567617: movups xmmword ptr [r8 + 0x138], xmm1
14156761f: mov qword ptr [r8 + 0x148], 0
14156762a: movsd qword ptr [r8 + 0x150], xmm0
141567633: movups xmmword ptr [r8 + 0x158], xmm1
14156763b: movups xmmword ptr [r8 + 0x168], xmm1
141567643: mov qword ptr [r8 + 0x178], 0
14156764e: add r8, 0x180
141567655: add r9, -8
141567659: jne 0x141567550
14156765f: test rdx, rdx
141567662: je 0x141567690
141567664: movsd xmm0, qword ptr [rip + 0x15796b4]
14156766c: xorps xmm1, xmm1
14156766f: nop 
141567670: movsd qword ptr [r8], xmm0
141567675: movups xmmword ptr [r8 + 8], xmm1
14156767a: movups xmmword ptr [r8 + 0x18], xmm1
14156767f: mov qword ptr [r8 + 0x28], 0
141567687: add r8, 0x30
14156768b: dec rdx
14156768e: jne 0x141567670
141567690: movsd xmm0, qword ptr [rip + 0x1579688]
141567698: movsd qword ptr [r8], xmm0
14156769d: movaps xmm0, xmmword ptr [rbp + 0x50]
1415676a1: movaps xmm1, xmmword ptr [rbp + 0x60]
1415676a5: movups xmmword ptr [r8 + 8], xmm0
1415676aa: movups xmmword ptr [r8 + 0x18], xmm1
1415676af: mov rdx, qword ptr [rbp + 0x70]
1415676b3: mov qword ptr [r8 + 0x28], rdx
1415676b7: mov qword ptr [rbp + 0x38], rcx
1415676bb: mov qword ptr [rbp + 0x40], rax
1415676bf: mov qword ptr [rbp + 0x48], r14
1415676c3: lea rcx, [rbp - 0x58]
1415676c7: lea r8, [rbp + 0x38]
1415676cb: mov rdx, rbx
1415676ce: call 0x141690360
1415676d3: movups xmm0, xmmword ptr [rbp - 0x58]
1415676d7: movups xmm1, xmmword ptr [rbp - 0x48]
1415676db: movups xmm2, xmmword ptr [rbp - 0x38]
1415676df: movups xmm3, xmmword ptr [rbp - 0x28]
1415676e3: movups xmmword ptr [rbp + 0x68], xmm0
1415676e7: movups xmmword ptr [rbp + 0x78], xmm1
1415676eb: movups xmmword ptr [rbp + 0x88], xmm2
1415676f2: movups xmmword ptr [rbp + 0x98], xmm3
1415676f9: mov rax, qword ptr [rbp - 0x18]
1415676fd: mov qword ptr [rbp + 0xa8], rax
141567704: mov byte ptr [rbp + 0x5e], 5
141567708: movzx eax, byte ptr [rip + 0x20cd4fa]
14156770f: mov ecx, 0x60
141567714: mov edx, 8
141567719: call 0x140613c10
14156771e: test rax, rax
141567721: je 0x141567854
141567727: movups xmm0, xmmword ptr [rbp + 0xa0]
14156772e: movups xmmword ptr [rax + 0x50], xmm0
141567732: movups xmm0, xmmword ptr [rbp + 0x90]
141567739: movups xmmword ptr [rax + 0x40], xmm0
14156773d: movups xmm0, xmmword ptr [rbp + 0x50]
141567741: movups xmm1, xmmword ptr [rbp + 0x60]
141567745: movups xmm2, xmmword ptr [rbp + 0x70]
141567749: movups xmm3, xmmword ptr [rbp + 0x80]
141567750: movups xmmword ptr [rax + 0x30], xmm3
141567754: movups xmmword ptr [rax + 0x20], xmm2
141567758: movups xmmword ptr [rax + 0x10], xmm1
14156775c: mov qword ptr [rbp + 0xb8], rax
141567763: movups xmmword ptr [rax], xmm0
141567766: xor ecx, ecx
141567768: call 0x1400d9b20
14156776d: test rax, rax
141567770: je 0x141567865
141567776: movups xmm0, xmmword ptr [rax]
141567779: inc qword ptr [rax]
14156777c: mov rax, qword ptr [rbp + 0x30]
141567780: mov qword ptr [rsi + 0x88], rax
141567787: movups xmm1, xmmword ptr [rbp - 0x10]
14156778b: movups xmm2, xmmword ptr [rbp]
14156778f: movups xmm3, xmmword ptr [rbp + 0x10]
141567793: movups xmm4, xmmword ptr [rbp + 0x20]
141567797: movups xmmword ptr [rsi + 0x78], xmm4
14156779b: movups xmmword ptr [rsi + 0x68], xmm3
14156779f: movups xmmword ptr [rsi + 0x58], xmm2
1415677a3: movups xmmword ptr [rsi + 0x48], xmm1
1415677a7: mov qword ptr [rsi], 0
1415677ae: mov qword ptr [rsi + 8], 4
1415677b6: mov qword ptr [rsi + 0x10], 0
1415677be: movups xmm1, xmmword ptr [rip + 0x156124b]
1415677c5: movups xmmword ptr [rsi + 0x18], xmm1
1415677c9: movups xmm1, xmmword ptr [rip + 0x1561250]
1415677d0: movups xmmword ptr [rsi + 0x28], xmm1
1415677d4: movups xmmword ptr [rsi + 0x38], xmm0
1415677d8: mov rax, qword ptr [rbp + 0xb8]
1415677df: mov qword ptr [rsi + 0x90], rax
1415677e6: lea rax, [rip + 0x170efcb]
1415677ed: mov qword ptr [rsi + 0x98], rax
1415677f4: mov qword ptr [rsi + 0xa0], 0
1415677ff: mov rax, qword ptr [rbp + 0xb0]
141567806: mov qword ptr [rsi + 0xa8], rax
14156780d: mov dword ptr [rsi + 0xb0], 0
141567817: mov rax, rsi
14156781a: add rsp, 0x148
141567821: pop rbx
141567822: pop rdi
141567823: pop rsi
141567824: pop r12
141567826: pop r13
141567828: pop r14
14156782a: pop r15
14156782c: pop rbp
14156782d: ret 
14156782e: lea r8, [rip + 0x170e873]
141567835: mov rcx, r13
141567838: mov rdx, r15
14156783b: call 0x1428d8fe3
141567840: lea r8, [rip + 0x170e879]
141567847: mov rcx, r12
14156784a: mov rdx, r15
14156784d: call 0x1428d8fe3
141567852: jmp 0x141567871
141567854: mov ecx, 8
141567859: mov edx, 0x60
14156785e: call 0x1428d9000
141567863: jmp 0x141567871
141567865: lea rcx, [rip + 0x16d3234]
14156786c: call 0x1428d80d0
141567871: ud2 
141567873: nop word ptr cs:[rax + rax]
141567880: mov qword ptr [rsp + 0x10], rdx
141567885: push rbp
141567886: push r15
141567888: push r14
14156788a: push r13
14156788c: push r12
14156788e: push rsi
14156788f: push rdi
141567890: push rbx
141567891: sub rsp, 0x28
141567895: lea rbp, [rdx + 0x80]
14156789c: lea rcx, [rbp - 0x10]
1415678a0: call 0x14087d360
1415678a5: nop 
1415678a6: add rsp, 0x28
1415678aa: pop rbx
1415678ab: pop rdi
1415678ac: pop rsi
1415678ad: pop r12
1415678af: pop r13
1415678b1: pop r14
1415678b3: pop r15
1415678b5: pop rbp
1415678b6: ret 
1415678b7: nop word ptr [rax + rax]
1415678c0: mov qword ptr [rsp + 0x10], rdx
1415678c5: push rbp
1415678c6: push r15
1415678c8: push r14
1415678ca: push r13
1415678cc: push r12
1415678ce: push rsi
1415678cf: push rdi
1415678d0: push rbx
1415678d1: sub rsp, 0x28
1415678d5: lea rbp, [rdx + 0x80]
1415678dc: lea rcx, [rbp + 0x50]
1415678e0: call 0x14087e820
1415678e5: nop 
1415678e6: add rsp, 0x28
1415678ea: pop rbx
1415678eb: pop rdi
1415678ec: pop rsi
1415678ed: pop r12
1415678ef: pop r13
1415678f1: pop r14
1415678f3: pop r15
1415678f5: pop rbp
1415678f6: ret 
1415678f7: nop word ptr [rax + rax]
141567900: mov qword ptr [rsp + 0x10], rdx
141567905: push rbp
141567906: push r15
141567908: push r14
14156790a: push r13
14156790c: push r12
14156790e: push rsi
14156790f: push rdi
141567910: push rbx
141567911: sub rsp, 0x28
141567915: lea rbp, [rdx + 0x80]
14156791c: lea rdx, [rip + 0x170ee95]
141567923: mov rcx, qword ptr [rbp + 0xb8]
14156792a: call 0x14002e900
14156792f: nop 
141567930: add rsp, 0x28
141567934: pop rbx
141567935: pop rdi
141567936: pop rsi
141567937: pop r12
141567939: pop r13
14156793b: pop r14
14156793d: pop r15
14156793f: pop rbp
141567940: ret 

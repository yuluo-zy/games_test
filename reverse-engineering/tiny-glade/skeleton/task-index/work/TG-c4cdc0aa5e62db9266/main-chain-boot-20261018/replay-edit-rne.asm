140af6170: push rbp
140af6171: push r15
140af6173: push r14
140af6175: push r13
140af6177: push r12
140af6179: push rsi
140af617a: push rdi
140af617b: push rbx
140af617c: sub rsp, 0x208
140af6183: lea rbp, [rsp + 0x80]
140af618b: mov qword ptr [rbp + 0x180], 0xfffffffffffffffe
140af6196: mov qword ptr [rbp + 0x150], r9
140af619d: mov edi, r8d
140af61a0: mov ebx, edx
140af61a2: mov rsi, rcx
140af61a5: mov qword ptr [rbp - 0x38], 0
140af61ad: mov qword ptr [rbp - 0x30], 8
140af61b5: mov qword ptr [rbp - 0x28], 0
140af61bd: mov r15, qword ptr [rcx + 8]
140af61c1: mov rax, qword ptr [rcx + 0x10]
140af61c5: lea rax, [rax + rax*2]
140af61c9: lea rax, [r15 + rax*8]
140af61cd: mov qword ptr [rbp + 0x80], rax
140af61d4: lea r14, [rbp - 0x38]
140af61d8: nop dword ptr [rax + rax]
140af61e0: cmp r15, qword ptr [rbp + 0x80]
140af61e7: mov qword ptr [rbp - 0x40], r15
140af61eb: je 0x140af6203
140af61ed: mov rcx, r15
140af61f0: add r15, 0x18
140af61f4: mov edx, ebx
140af61f6: mov r8d, edi
140af61f9: mov r9, r14
140af61fc: call 0x14143fb50
140af6201: jmp 0x140af61e0
140af6203: mov rax, qword ptr [rbp - 0x28]
140af6207: mov qword ptr [rbp + 0x120], rax
140af620e: movdqu xmm0, xmmword ptr [rbp - 0x38]
140af6213: movdqa xmmword ptr [rbp + 0x110], xmm0
140af621b: mov rax, qword ptr [rsi + 0x18]
140af621f: lock inc qword ptr [rax]
140af6223: jle 0x140af6ea5
140af6229: mov rdx, qword ptr [rsi + 0x20]
140af622d: lock inc qword ptr [rdx]
140af6231: jle 0x140af6ea5
140af6237: mov byte ptr [rbp + 0x17b], bl
140af623d: mov qword ptr [rbp + 0x148], rax
140af6244: mov qword ptr [rbp + 0x158], rdx
140af624b: test bl, bl
140af624d: je 0x140af62bb
140af624f: mov rax, qword ptr [rbp + 0x120]
140af6256: cmp rax, 2
140af625a: jb 0x140af62bb
140af625c: mov rcx, qword ptr [rbp + 0x118]
140af6263: lea rdx, [rax + rax*2]
140af6267: shr rax, 1
140af626a: shl rdx, 4
140af626e: add rdx, rcx
140af6271: add rdx, -0x10
140af6275: add rcx, 0x20
140af6279: nop dword ptr [rax]
140af6280: movups xmm0, xmmword ptr [rcx - 0x20]
140af6284: movups xmm1, xmmword ptr [rdx - 0x20]
140af6288: movups xmmword ptr [rcx - 0x20], xmm1
140af628c: movups xmmword ptr [rdx - 0x20], xmm0
140af6290: movups xmm0, xmmword ptr [rcx - 0x10]
140af6294: movups xmm1, xmmword ptr [rdx - 0x10]
140af6298: movups xmmword ptr [rcx - 0x10], xmm1
140af629c: movups xmmword ptr [rdx - 0x10], xmm0
140af62a0: movdqu xmm0, xmmword ptr [rcx]
140af62a4: movups xmm1, xmmword ptr [rdx]
140af62a7: movups xmmword ptr [rcx], xmm1
140af62aa: movdqu xmmword ptr [rdx], xmm0
140af62ae: add rcx, 0x30
140af62b2: add rdx, -0x30
140af62b6: dec rax
140af62b9: jne 0x140af6280
140af62bb: mov qword ptr [rbp + 0xe8], 0
140af62c6: mov qword ptr [rbp + 0xf0], 8
140af62d1: mov qword ptr [rbp + 0xf8], 0
140af62dc: movzx eax, byte ptr [rip + 0x2b3e926]
140af62e3: mov ecx, 0x40
140af62e8: mov edx, 8
140af62ed: call 0x140613c10
140af62f2: test rax, rax
140af62f5: je 0x140af6e81
140af62fb: mov qword ptr [rbp + 0x130], 8
140af6306: mov qword ptr [rbp + 0x138], rax
140af630d: mov qword ptr [rbp + 0x140], 0
140af6318: xor r13d, r13d
140af631b: lea r15, [rip + 0x1fef10e]
140af6322: jmp 0x140af6381
140af6324: nop word ptr cs:[rax + rax]
140af6330: mov rax, qword ptr [rbp + 0xf0]
140af6337: lea rcx, [rsi + rsi*2]
140af633b: mov rdx, qword ptr [rbp + 0x168]
140af6342: mov qword ptr [rax + rcx*8], rdx
140af6346: mov rdx, qword ptr [rbp + 0x160]
140af634d: mov qword ptr [rax + rcx*8 + 8], rdx
140af6352: mov qword ptr [rax + rcx*8 + 0x10], r14
140af6357: inc rsi
140af635a: mov qword ptr [rbp + 0xf8], rsi
140af6361: mov rax, qword ptr [rbp + 0x138]
140af6368: mov rcx, qword ptr [rbp + 0x140]
140af636f: test rcx, rcx
140af6372: lea rdx, [rax + rcx*8 - 8]
140af6377: cmove rdx, rcx
140af637b: mov r13, qword ptr [rdx]
140af637e: inc r13
140af6381: mov rcx, qword ptr [rbp + 0x120]
140af6388: cmp r13, rcx
140af638b: jae 0x140af6711
140af6391: mov qword ptr [rbp + 0x140], 0
140af639c: cmp qword ptr [rbp + 0x130], 0
140af63a4: jne 0x140af63ce
140af63a6: mov byte ptr [rbp + 0x17f], 1
140af63ad: mov byte ptr [rbp + 0x17e], 1
140af63b4: lea rcx, [rbp + 0x130]
140af63bb: lea rdx, [rip + 0x1feef56]
140af63c2: call 0x140b81df0
140af63c7: mov rax, qword ptr [rbp + 0x138]
140af63ce: mov qword ptr [rax], r13
140af63d1: mov qword ptr [rbp + 0x140], 1
140af63dc: mov rsi, qword ptr [rbp + 0x118]
140af63e3: mov rax, qword ptr [rbp + 0x120]
140af63ea: lea rdi, [rax + rax*2]
140af63ee: shl rdi, 4
140af63f2: add rdi, rsi
140af63f5: inc r13
140af63f8: mov r14d, 1
140af63fe: mov ebx, 8
140af6403: mov r12d, 8
140af6409: xor edx, edx
140af640b: jmp 0x140af6434
140af640d: nop dword ptr [rax]
140af6410: add rsi, 0x30
140af6414: mov rdx, qword ptr [rbp + 0x168]
140af641b: mov qword ptr [r13 + r14*8], rdx
140af6420: inc rdx
140af6423: inc r14
140af6426: mov qword ptr [rbp + 0x140], r14
140af642d: add rbx, 8
140af6431: xor r13d, r13d
140af6434: sub r12, 1
140af6438: jb 0x140af664a
140af643e: test r13, r13
140af6441: jne 0x140af65c7
140af6447: cmp rsi, rdi
140af644a: je 0x140af664a
140af6450: mov qword ptr [rbp + 0x168], rdx
140af6457: mov r13, qword ptr [rbp + 0x138]
140af645e: test r14, r14
140af6461: je 0x140af6610
140af6467: mov rax, qword ptr [rbp + 0x118]
140af646e: mov rdx, qword ptr [rbp + 0x120]
140af6475: xor r8d, r8d
140af6478: jmp 0x140af649d
140af647a: test byte ptr [rsi], 7
140af647d: jne 0x140af6660
140af6483: nop word ptr cs:[rax + rax]
140af6490: add r8, 8
140af6494: cmp rbx, r8
140af6497: je 0x140af6610
140af649d: mov rcx, qword ptr [r13 + r8]
140af64a2: cmp rdx, rcx
140af64a5: jbe 0x140af6e41
140af64ab: lea rcx, [rcx + rcx*2]
140af64af: shl rcx, 4
140af64b3: movzx r9d, byte ptr [rax + rcx]
140af64b8: cmp r9, 0xa
140af64bc: ja 0x140af6660
140af64c2: add rcx, rax
140af64c5: movsxd r9, dword ptr [r15 + r9*4]
140af64c9: add r9, r15
140af64cc: jmp r9
140af64cf: movzx r9d, byte ptr [rsi]
140af64d3: test r9d, r9d
140af64d6: je 0x140af659b
140af64dc: cmp r9d, 4
140af64e0: je 0x140af6490
140af64e2: cmp r9d, 8
140af64e6: je 0x140af6490
140af64e8: jmp 0x140af6660
140af64ed: cmp byte ptr [rsi], 6
140af64f0: je 0x140af6490
140af64f2: jmp 0x140af6660
140af64f7: cmp byte ptr [rsi], 4
140af64fa: mov rcx, rsi
140af64fd: je 0x140af6525
140af64ff: jmp 0x140af6660
140af6504: cmp byte ptr [rsi], 7
140af6507: je 0x140af6490
140af6509: jmp 0x140af6660
140af650e: movzx r9d, byte ptr [rsi]
140af6512: test r9d, r9d
140af6515: je 0x140af6490
140af651b: cmp r9d, 5
140af651f: jne 0x140af6660
140af6525: cmp qword ptr [rcx + 0x20], 1
140af652a: jne 0x140af6660
140af6530: mov r9, qword ptr [rcx + 0x18]
140af6534: mov r10, qword ptr [rcx + 8]
140af6538: mov rcx, qword ptr [rcx + 0x10]
140af653c: cmp r9, r10
140af653f: mov r11d, 0
140af6545: cmovae r11, r10
140af6549: sub r9, r11
140af654c: lea r9, [r9 + r9*2]
140af6550: cmp qword ptr [rcx + r9*8 + 0x10], 1
140af6556: jne 0x140af6660
140af655c: lea rcx, [rcx + r9*8]
140af6560: mov rcx, qword ptr [rcx + 8]
140af6564: mov rcx, qword ptr [rcx + 8]
140af6568: movabs r9, 0x8000000000000002
140af6572: cmp qword ptr [rcx + 0x10], r9
140af6576: je 0x140af6490
140af657c: jmp 0x140af6660
140af6581: movzx ecx, byte ptr [rsi]
140af6584: cmp ecx, 7
140af6587: je 0x140af6490
140af658d: cmp ecx, 0xa
140af6590: je 0x140af6490
140af6596: jmp 0x140af6660
140af659b: movzx r9d, byte ptr [rcx + 1]
140af65a0: cmp r9b, byte ptr [rsi + 1]
140af65a4: jne 0x140af6490
140af65aa: test r9b, r9b
140af65ad: je 0x140af6660
140af65b3: movzx r9d, byte ptr [rsi + 2]
140af65b8: cmp byte ptr [rcx + 2], r9b
140af65bc: jne 0x140af6490
140af65c2: jmp 0x140af6660
140af65c7: mov rax, rdi
140af65ca: sub rax, rsi
140af65cd: shr rax, 4
140af65d1: movabs rcx, 0xaaaaaaaaaaaaaaab
140af65db: imul rax, rcx
140af65df: cmp rax, r13
140af65e2: jbe 0x140af664a
140af65e4: lea rax, [r13*2]
140af65ec: add rax, r13
140af65ef: shl rax, 4
140af65f3: add rsi, rax
140af65f6: add rdx, r13
140af65f9: mov qword ptr [rbp + 0x168], rdx
140af6600: mov r13, qword ptr [rbp + 0x138]
140af6607: test r14, r14
140af660a: jne 0x140af6467
140af6610: cmp r14, qword ptr [rbp + 0x130]
140af6617: jne 0x140af6410
140af661d: mov byte ptr [rbp + 0x17f], 1
140af6624: mov byte ptr [rbp + 0x17e], 1
140af662b: lea rcx, [rbp + 0x130]
140af6632: lea rdx, [rip + 0x1feed0f]
140af6639: call 0x140b81df0
140af663e: mov r13, qword ptr [rbp + 0x138]
140af6645: jmp 0x140af6410
140af664a: mov r13, qword ptr [rbp + 0x138]
140af6651: nop word ptr cs:[rax + rax]
140af6660: lea rsi, [r14*8]
140af6668: mov rax, r14
140af666b: shr rax, 0x3d
140af666f: jne 0x140af6e5d
140af6675: mov edi, 0
140af667a: movabs rax, 0x7ffffffffffffff8
140af6684: cmp rsi, rax
140af6687: ja 0x140af6e5f
140af668d: test rsi, rsi
140af6690: je 0x140af66c0
140af6692: movzx eax, byte ptr [rip + 0x2b3e570]
140af6699: mov edi, 8
140af669e: mov edx, 8
140af66a3: mov rcx, rsi
140af66a6: call 0x140613c10
140af66ab: test rax, rax
140af66ae: je 0x140af6e5f
140af66b4: mov qword ptr [rbp + 0x168], r14
140af66bb: jmp 0x140af66d0
140af66bd: nop dword ptr [rax]
140af66c0: mov eax, 8
140af66c5: mov qword ptr [rbp + 0x168], 0
140af66d0: mov qword ptr [rbp + 0x160], rax
140af66d7: mov rcx, rax
140af66da: mov rdx, r13
140af66dd: mov r8, rsi
140af66e0: call 0x142923c00
140af66e5: mov rsi, qword ptr [rbp + 0xf8]
140af66ec: cmp rsi, qword ptr [rbp + 0xe8]
140af66f3: jne 0x140af6330
140af66f9: lea rcx, [rbp + 0xe8]
140af6700: lea rdx, [rip + 0x1feec29]
140af6707: call 0x1406ae3b0
140af670c: jmp 0x140af6330
140af6711: mov qword ptr [rbp + 0x120], 0
140af671c: mov rax, qword ptr [rbp + 0x118]
140af6723: lea rdx, [rcx + rcx*2]
140af6727: shl rdx, 4
140af672b: add rdx, rax
140af672e: mov qword ptr [rbp - 0x20], rax
140af6732: mov qword ptr [rbp - 0x18], rdx
140af6736: lea rax, [rbp + 0x110]
140af673d: mov qword ptr [rbp - 0x10], rax
140af6741: mov qword ptr [rbp - 8], rcx
140af6745: mov qword ptr [rbp], 0
140af674d: mov byte ptr [rbp + 0x17f], 1
140af6754: mov byte ptr [rbp + 0x17e], 1
140af675b: lea r8, [rip + 0x1fee92e]
140af6762: lea rsi, [rbp - 0x58]
140af6766: lea rdx, [rbp - 0x20]
140af676a: mov rcx, rsi
140af676d: call 0x141307390
140af6772: mov rax, qword ptr [rbp + 0x148]
140af6779: movzx ecx, byte ptr [rbp + 0x17b]
140af6780: mov rdx, qword ptr [rax + 0x10]
140af6784: mov r8, qword ptr [rax + 0x18]
140af6788: add r8, rdx
140af678b: inc r8
140af678e: movdqa xmm0, xmmword ptr [rdx]
140af6792: pmovmskb r9d, xmm0
140af6797: not r9d
140af679a: mov word ptr [rbp + 0x58], r9w
140af679f: mov qword ptr [rbp + 0x40], rdx
140af67a3: add rdx, 0x10
140af67a7: mov qword ptr [rbp + 0x48], rdx
140af67ab: mov qword ptr [rbp + 0x50], r8
140af67af: mov rax, qword ptr [rax + 0x28]
140af67b3: mov qword ptr [rbp + 0x60], rax
140af67b7: mov qword ptr [rbp - 0x20], 0
140af67bf: mov qword ptr [rbp + 0x10], 0
140af67c7: mov byte ptr [rbp + 0x68], cl
140af67ca: mov byte ptr [rbp + 0x17a], 1
140af67d1: mov byte ptr [rbp + 0x179], 1
140af67d8: lea rcx, [rbp - 0x20]
140af67dc: call 0x140a96d20
140af67e1: mov qword ptr [rbp + 0x88], rax
140af67e8: mov qword ptr [rbp + 0x90], rdx
140af67ef: mov rax, qword ptr [rbp + 0xf0]
140af67f6: mov rcx, qword ptr [rbp + 0xf8]
140af67fd: lea rcx, [rcx + rcx*2]
140af6801: lea rcx, [rax + rcx*8]
140af6805: mov qword ptr [rbp - 0x20], rax
140af6809: mov qword ptr [rbp - 0x18], rcx
140af680d: mov qword ptr [rbp - 0x10], rsi
140af6811: lea rax, [rbp + 0x17b]
140af6818: mov qword ptr [rbp - 8], rax
140af681c: lea rax, [rbp + 0x88]
140af6823: mov qword ptr [rbp], rax
140af6827: mov byte ptr [rbp + 0x175], 1
140af682e: mov byte ptr [rbp + 0x174], 1
140af6835: lea r8, [rip + 0x1fee854]
140af683c: lea rcx, [rbp + 0xd0]
140af6843: lea rdx, [rbp - 0x20]
140af6847: call 0x1412ffd50
140af684c: mov rax, qword ptr [rbp + 0x148]
140af6853: cmp qword ptr [rax + 0x28], 0
140af6858: je 0x140af68c8
140af685a: mov rsi, qword ptr [rbp + 0xe0]
140af6861: test rsi, rsi
140af6864: sete cl
140af6867: shl rsi, 6
140af686b: add rsi, qword ptr [rbp + 0xd8]
140af6872: cmp rsi, 0x40
140af6876: sete dl
140af6879: or dl, cl
140af687b: je 0x140af6a31
140af6881: mov qword ptr [rbp + 0xa8], 0
140af688c: mov qword ptr [rbp + 0xb0], 8
140af6897: mov qword ptr [rbp + 0xb8], 0
140af68a2: mov qword ptr [rbp + 0x70], 0
140af68aa: mov qword ptr [rbp + 0xa0], rax
140af68b1: mov rax, qword ptr [rbp + 0x158]
140af68b8: cmp qword ptr [rax + 0x40], 0
140af68bd: jne 0x140af6991
140af68c3: jmp 0x140af696f
140af68c8: mov rcx, qword ptr [rbp + 0x158]
140af68cf: mov rdx, qword ptr [rcx + 0x40]
140af68d3: test rdx, rdx
140af68d6: jne 0x140af68e3
140af68d8: cmp qword ptr [rcx + 0x58], 0
140af68dd: je 0x140af6e37
140af68e3: mov rsi, qword ptr [rbp + 0xe0]
140af68ea: test rsi, rsi
140af68ed: sete r8b
140af68f1: shl rsi, 6
140af68f5: add rsi, qword ptr [rbp + 0xd8]
140af68fc: cmp rsi, 0x40
140af6900: sete r9b
140af6904: or r9b, r8b
140af6907: mov bl, 1
140af6909: je 0x140af6a6c
140af690f: mov qword ptr [rbp + 0xa8], 0
140af691a: mov qword ptr [rbp + 0xb0], 8
140af6925: mov qword ptr [rbp + 0xb8], 0
140af6930: mov qword ptr [rbp + 0x70], 0
140af6938: mov qword ptr [rbp + 0x98], rax
140af693f: lock dec qword ptr [rax]
140af6943: jne 0x140af6958
140af6945: mov byte ptr [rbp + 0x173], 1
140af694c: lea rcx, [rbp + 0x98]
140af6953: call 0x140b5cb60
140af6958: xor eax, eax
140af695a: mov qword ptr [rbp + 0xa0], rax
140af6961: mov rax, qword ptr [rbp + 0x158]
140af6968: cmp qword ptr [rax + 0x40], 0
140af696d: jne 0x140af6991
140af696f: cmp qword ptr [rax + 0x58], 0
140af6974: mov qword ptr [rbp + 0x98], rax
140af697b: jne 0x140af6991
140af697d: lock dec qword ptr [rax]
140af6981: jne 0x140af698f
140af6983: lea rcx, [rbp + 0x98]
140af698a: call 0x140b5c420
140af698f: xor eax, eax
140af6991: movups xmm0, xmmword ptr [rbp + 0xa8]
140af6998: movaps xmmword ptr [rbp - 0x20], xmm0
140af699c: mov rcx, qword ptr [rbp + 0xb8]
140af69a3: mov qword ptr [rbp - 0x10], rcx
140af69a7: movzx ecx, byte ptr [rbp + 0x17b]
140af69ae: mov byte ptr [rbp + 0x18], cl
140af69b1: movdqu xmm0, xmmword ptr [rbp + 0x70]
140af69b6: movdqu xmmword ptr [rbp - 8], xmm0
140af69bb: mov rcx, qword ptr [rbp + 0xa0]
140af69c2: mov qword ptr [rbp + 8], rcx
140af69c6: mov qword ptr [rbp + 0x10], rax
140af69ca: mov rsi, qword ptr [rbp + 0xe0]
140af69d1: cmp rsi, qword ptr [rbp + 0xd0]
140af69d8: jne 0x140af69ed
140af69da: lea rdx, [rip + 0x1fee91f]
140af69e1: lea rcx, [rbp + 0xd0]
140af69e8: call 0x1405627b0
140af69ed: mov rax, qword ptr [rbp + 0xd8]
140af69f4: mov rcx, rsi
140af69f7: shl rcx, 6
140af69fb: movdqa xmm0, xmmword ptr [rbp - 0x20]
140af6a00: movaps xmm1, xmmword ptr [rbp - 0x10]
140af6a04: movaps xmm2, xmmword ptr [rbp]
140af6a08: movaps xmm3, xmmword ptr [rbp + 0x10]
140af6a0c: movups xmmword ptr [rax + rcx + 0x30], xmm3
140af6a11: movups xmmword ptr [rax + rcx + 0x20], xmm2
140af6a16: movups xmmword ptr [rax + rcx + 0x10], xmm1
140af6a1b: movdqu xmmword ptr [rax + rcx], xmm0
140af6a20: inc rsi
140af6a23: mov qword ptr [rbp + 0xe0], rsi
140af6a2a: xor r14d, r14d
140af6a2d: xor ebx, ebx
140af6a2f: jmp 0x140af6aac
140af6a31: mov rcx, qword ptr [rsi - 0x18]
140af6a35: mov qword ptr [rsi - 0x18], rax
140af6a39: mov qword ptr [rbp - 0x20], rcx
140af6a3d: test rcx, rcx
140af6a40: je 0x140af6a5f
140af6a42: lock dec qword ptr [rcx]
140af6a46: jne 0x140af6a5f
140af6a48: mov byte ptr [rbp + 0x17d], 1
140af6a4f: mov byte ptr [rbp + 0x17c], 0
140af6a56: lea rcx, [rbp - 0x20]
140af6a5a: call 0x140b5cb60
140af6a5f: mov rcx, qword ptr [rbp + 0x158]
140af6a66: mov rdx, qword ptr [rcx + 0x40]
140af6a6a: xor ebx, ebx
140af6a6c: test rdx, rdx
140af6a6f: jne 0x140af6a7c
140af6a71: cmp qword ptr [rcx + 0x58], 0
140af6a76: je 0x140af6e2f
140af6a7c: mov rax, qword ptr [rsi - 0x10]
140af6a80: mov qword ptr [rsi - 0x10], rcx
140af6a84: mov qword ptr [rbp - 0x20], rax
140af6a88: test rax, rax
140af6a8b: je 0x140af6aa9
140af6a8d: lock dec qword ptr [rax]
140af6a91: jne 0x140af6aa9
140af6a93: mov byte ptr [rbp + 0x17d], 0
140af6a9a: mov byte ptr [rbp + 0x17c], bl
140af6aa0: lea rcx, [rbp - 0x20]
140af6aa4: call 0x140b5c420
140af6aa9: xor r14d, r14d
140af6aac: mov rax, qword ptr [rbp + 0xd0]
140af6ab3: mov qword ptr [rbp + 0x108], rax
140af6aba: mov rax, qword ptr [rbp + 0xd8]
140af6ac1: mov qword ptr [rbp + 0x100], rax
140af6ac8: mov rdi, qword ptr [rbp + 0xe0]
140af6acf: mov rax, qword ptr [rbp + 0x88]
140af6ad6: lock dec qword ptr [rax]
140af6ada: jne 0x140af6af5
140af6adc: mov byte ptr [rbp + 0x17a], r14b
140af6ae3: mov byte ptr [rbp + 0x179], bl
140af6ae9: lea rcx, [rbp + 0x88]
140af6af0: call 0x140b5cd70
140af6af5: mov byte ptr [rbp + 0x17f], r14b
140af6afc: mov byte ptr [rbp + 0x17e], bl
140af6b02: lea rcx, [rbp - 0x58]
140af6b06: call 0x14087d990
140af6b0b: mov rdx, qword ptr [rbp + 0x130]
140af6b12: test rdx, rdx
140af6b15: je 0x140af6b2d
140af6b17: mov rcx, qword ptr [rbp + 0x138]
140af6b1e: shl rdx, 3
140af6b22: mov r8d, 8
140af6b28: call 0x140613c20
140af6b2d: mov rsi, qword ptr [rbp + 0xf0]
140af6b34: mov r15, qword ptr [rbp + 0xf8]
140af6b3b: test r15, r15
140af6b3e: je 0x140af6b78
140af6b40: lea r12, [rsi + 8]
140af6b44: jmp 0x140af6b59
140af6b46: nop word ptr cs:[rax + rax]
140af6b50: add r12, 0x18
140af6b54: dec r15
140af6b57: je 0x140af6b78
140af6b59: mov rdx, qword ptr [r12 - 8]
140af6b5e: test rdx, rdx
140af6b61: je 0x140af6b50
140af6b63: mov rcx, qword ptr [r12]
140af6b67: shl rdx, 3
140af6b6b: mov r8d, 8
140af6b71: call 0x140613c20
140af6b76: jmp 0x140af6b50
140af6b78: mov rax, qword ptr [rbp + 0xe8]
140af6b7f: test rax, rax
140af6b82: je 0x140af6b9a
140af6b84: shl rax, 3
140af6b88: lea rdx, [rax + rax*2]
140af6b8c: mov r8d, 8
140af6b92: mov rcx, rsi
140af6b95: call 0x140613c20
140af6b9a: test r14b, r14b
140af6b9d: je 0x140af6bbe
140af6b9f: mov rax, qword ptr [rbp + 0x158]
140af6ba6: lock dec qword ptr [rax]
140af6baa: jne 0x140af6bbe
140af6bac: mov byte ptr [rbp + 0x178], bl
140af6bb2: lea rcx, [rbp + 0x158]
140af6bb9: call 0x140b5c420
140af6bbe: test bl, bl
140af6bc0: je 0x140af6bdb
140af6bc2: mov rax, qword ptr [rbp + 0x148]
140af6bc9: lock dec qword ptr [rax]
140af6bcd: jne 0x140af6bdb
140af6bcf: lea rcx, [rbp + 0x148]
140af6bd6: call 0x140b5cb60
140af6bdb: mov rcx, qword ptr [rbp + 0x118]
140af6be2: mov rax, qword ptr [rbp + 0x120]
140af6be9: mov qword ptr [rbp + 0x160], rax
140af6bf0: mov qword ptr [rbp + 0x168], 0
140af6bfb: mov qword ptr [rbp + 0xc8], rcx
140af6c02: nop word ptr cs:[rax + rax]
140af6c10: mov rax, qword ptr [rbp + 0x168]
140af6c17: cmp qword ptr [rbp + 0x160], rax
140af6c1e: je 0x140af6c38
140af6c20: inc rax
140af6c23: mov qword ptr [rbp + 0x168], rax
140af6c2a: lea rsi, [rcx + 0x30]
140af6c2e: call 0x140893b10
140af6c33: mov rcx, rsi
140af6c36: jmp 0x140af6c10
140af6c38: mov rax, qword ptr [rbp + 0x110]
140af6c3f: test rax, rax
140af6c42: je 0x140af6c5e
140af6c44: shl rax, 4
140af6c48: lea rdx, [rax + rax*2]
140af6c4c: mov r8d, 8
140af6c52: mov rcx, qword ptr [rbp + 0xc8]
140af6c59: call 0x140613c20
140af6c5e: mov rsi, rdi
140af6c61: shl rsi, 6
140af6c65: mov rcx, qword ptr [rbp + 0x100]
140af6c6c: add rsi, rcx
140af6c6f: mov qword ptr [rbp + 0xa8], rcx
140af6c76: mov qword ptr [rbp + 0xb0], rcx
140af6c7d: mov rax, qword ptr [rbp + 0x108]
140af6c84: mov qword ptr [rbp + 0xb8], rax
140af6c8b: mov qword ptr [rbp + 0xc0], rsi
140af6c92: mov rdx, rcx
140af6c95: test rdi, rdi
140af6c98: je 0x140af6d9f
140af6c9e: mov rax, qword ptr [rbp + 0x150]
140af6ca5: mov ebx, dword ptr [rax + 0x1c]
140af6ca8: mov rdi, qword ptr [rax]
140af6cab: mov r14, qword ptr [rax + 0x10]
140af6caf: lea r15, [rbp - 0x18]
140af6cb3: mov rax, qword ptr [rbp + 0x100]
140af6cba: lea rdx, [rax + 0x40]
140af6cbe: xor r12d, r12d
140af6cc1: jmp 0x140af6d23
140af6cc3: nop word ptr cs:[rax + rax]
140af6cd0: add rax, qword ptr [rdi + 0x10]
140af6cd4: cmp rax, rcx
140af6cd7: cmovb rcx, r12
140af6cdb: sub rax, rcx
140af6cde: mov rcx, qword ptr [rdi + 8]
140af6ce2: shl rax, 6
140af6ce6: movdqu xmm0, xmmword ptr [rbp - 0x20]
140af6ceb: movups xmm1, xmmword ptr [rbp - 0x10]
140af6cef: movups xmm2, xmmword ptr [rbp]
140af6cf3: movups xmm3, xmmword ptr [rbp + 0x10]
140af6cf7: movups xmmword ptr [rcx + rax + 0x30], xmm3
140af6cfc: movups xmmword ptr [rcx + rax + 0x20], xmm2
140af6d01: movups xmmword ptr [rcx + rax + 0x10], xmm1
140af6d06: movdqu xmmword ptr [rcx + rax], xmm0
140af6d0b: inc qword ptr [rdi + 0x18]
140af6d0f: mov rdx, qword ptr [rbp + 0x150]
140af6d16: add rdx, 0x40
140af6d1a: add r13, 0x40
140af6d1e: cmp r13, rsi
140af6d21: je 0x140af6d95
140af6d23: mov rax, qword ptr [rdx - 0x40]
140af6d27: mov rcx, rax
140af6d2a: neg rcx
140af6d2d: jo 0x140af6d98
140af6d2f: mov qword ptr [rbp + 0x150], rdx
140af6d36: lea r13, [rdx - 0x40]
140af6d3a: mov rcx, qword ptr [r13 + 0x38]
140af6d3e: mov qword ptr [r15 + 0x30], rcx
140af6d42: movups xmm0, xmmword ptr [r13 + 8]
140af6d47: movups xmm1, xmmword ptr [r13 + 0x18]
140af6d4c: movups xmm2, xmmword ptr [r13 + 0x28]
140af6d51: movups xmmword ptr [r15 + 0x20], xmm2
140af6d56: movups xmmword ptr [r15 + 0x10], xmm1
140af6d5b: movups xmmword ptr [r15], xmm0
140af6d5f: mov dword ptr [r14], ebx
140af6d62: mov qword ptr [rbp - 0x20], rax
140af6d66: inc qword ptr [rdi + 0x20]
140af6d6a: mov rcx, qword ptr [rdi]
140af6d6d: mov rax, qword ptr [rdi + 0x18]
140af6d71: cmp rax, rcx
140af6d74: jne 0x140af6cd0
140af6d7a: mov rcx, rdi
140af6d7d: lea rdx, [rip + 0x1fcea5c]
140af6d84: call 0x140b5aa20
140af6d89: mov rcx, qword ptr [rdi]
140af6d8c: mov rax, qword ptr [rdi + 0x18]
140af6d90: jmp 0x140af6cd0
140af6d95: mov rdx, rsi
140af6d98: mov qword ptr [rbp + 0xb0], rdx
140af6d9f: sub rsi, rdx
140af6da2: shr rsi, 6
140af6da6: mov qword ptr [rbp + 0x160], rsi
140af6dad: mov qword ptr [rbp + 0x168], 0
140af6db8: mov qword ptr [rbp + 0x150], rdx
140af6dbf: mov rcx, rdx
140af6dc2: nop word ptr cs:[rax + rax]
140af6dd0: mov rax, qword ptr [rbp + 0x168]
140af6dd7: cmp qword ptr [rbp + 0x160], rax
140af6dde: je 0x140af6df8
140af6de0: inc rax
140af6de3: mov qword ptr [rbp + 0x168], rax
140af6dea: lea rsi, [rcx + 0x40]
140af6dee: call 0x140894e90
140af6df3: mov rcx, rsi
140af6df6: jmp 0x140af6dd0
140af6df8: mov rdx, qword ptr [rbp + 0x108]
140af6dff: test rdx, rdx
140af6e02: je 0x140af6e1a
140af6e04: shl rdx, 6
140af6e08: mov r8d, 8
140af6e0e: mov rcx, qword ptr [rbp + 0x100]
140af6e15: call 0x140613c20
140af6e1a: nop 
140af6e1b: add rsp, 0x208
140af6e22: pop rbx
140af6e23: pop rdi
140af6e24: pop rsi
140af6e25: pop r12
140af6e27: pop r13
140af6e29: pop r14
140af6e2b: pop r15
140af6e2d: pop rbp
140af6e2e: ret 
140af6e2f: mov r14b, 1
140af6e32: jmp 0x140af6aac
140af6e37: mov r14b, 1
140af6e3a: mov bl, 1
140af6e3c: jmp 0x140af6aac
140af6e41: mov byte ptr [rbp + 0x17f], 1
140af6e48: mov byte ptr [rbp + 0x17e], 1
140af6e4f: lea r8, [rip + 0x1fee50a]
140af6e56: call 0x1428d9518
140af6e5b: jmp 0x140af6ea5
140af6e5d: xor edi, edi
140af6e5f: mov byte ptr [rbp + 0x17f], 1
140af6e66: mov byte ptr [rbp + 0x17e], 1
140af6e6d: lea r8, [rip + 0x2141d04]
140af6e74: mov rcx, rdi
140af6e77: mov rdx, rsi
140af6e7a: call 0x1428d8fe3
140af6e7f: jmp 0x140af6ea5
140af6e81: mov byte ptr [rbp + 0x177], 1
140af6e88: mov byte ptr [rbp + 0x176], 1
140af6e8f: lea r8, [rip + 0x1fee452]
140af6e96: mov ecx, 8
140af6e9b: mov edx, 0x40
140af6ea0: call 0x1428d8fe3
140af6ea5: ud2 
140af6ea7: nop word ptr [rax + rax]
140af6eb0: mov qword ptr [rsp + 0x10], rdx
140af6eb5: push rbp
140af6eb6: push r15
140af6eb8: push r14
140af6eba: push r13
140af6ebc: push r12
140af6ebe: push rsi
140af6ebf: push rdi
140af6ec0: push rbx
140af6ec1: sub rsp, 0x28
140af6ec5: lea rbp, [rdx + 0x80]
140af6ecc: mov rax, qword ptr [rbp + 0xa0]
140af6ed3: test rax, rax
140af6ed6: je 0x140af6eea
140af6ed8: lock dec qword ptr [rax]
140af6edc: jne 0x140af6eea
140af6ede: lea rcx, [rbp + 0xa0]
140af6ee5: call 0x140b5cb60
140af6eea: mov byte ptr [rbp + 0x173], 0
140af6ef1: add rsp, 0x28
140af6ef5: pop rbx
140af6ef6: pop rdi
140af6ef7: pop rsi
140af6ef8: pop r12
140af6efa: pop r13
140af6efc: pop r14
140af6efe: pop r15
140af6f00: pop rbp
140af6f01: ret 
140af6f02: nop word ptr cs:[rax + rax]
140af6f10: mov qword ptr [rsp + 0x10], rdx
140af6f15: push rbp
140af6f16: push r15
140af6f18: push r14
140af6f1a: push r13
140af6f1c: push r12
140af6f1e: push rsi
140af6f1f: push rdi
140af6f20: push rbx
140af6f21: sub rsp, 0x28
140af6f25: lea rbp, [rdx + 0x80]
140af6f2c: movzx ebx, byte ptr [rbp + 0x173]
140af6f33: mov rax, qword ptr [rbp + 0x70]
140af6f37: test rax, rax
140af6f3a: je 0x140af6f4b
140af6f3c: lock dec qword ptr [rax]
140af6f40: jne 0x140af6f4b
140af6f42: lea rcx, [rbp + 0x70]
140af6f46: call 0x140b5cd70
140af6f4b: lea rcx, [rbp + 0xa8]
140af6f52: call 0x1408772e0
140af6f57: mov byte ptr [rbp + 0x17d], bl
140af6f5d: mov byte ptr [rbp + 0x17c], 0
140af6f64: add rsp, 0x28
140af6f68: pop rbx
140af6f69: pop rdi
140af6f6a: pop rsi
140af6f6b: pop r12
140af6f6d: pop r13
140af6f6f: pop r14
140af6f71: pop r15
140af6f73: pop rbp
140af6f74: ret 
140af6f75: nop word ptr cs:[rax + rax]
140af6f80: mov qword ptr [rsp + 0x10], rdx
140af6f85: push rbp
140af6f86: push r15
140af6f88: push r14
140af6f8a: push r13
140af6f8c: push r12
140af6f8e: push rsi
140af6f8f: push rdi
140af6f90: push rbx
140af6f91: sub rsp, 0x28
140af6f95: lea rbp, [rdx + 0x80]
140af6f9c: mov byte ptr [rbp + 0x17d], 0
140af6fa3: mov byte ptr [rbp + 0x17c], 0
140af6faa: lea rcx, [rbp - 0x20]
140af6fae: call 0x140894e90
140af6fb3: mov byte ptr [rbp + 0x17d], 0
140af6fba: mov byte ptr [rbp + 0x17c], 0
140af6fc1: add rsp, 0x28
140af6fc5: pop rbx
140af6fc6: pop rdi
140af6fc7: pop rsi
140af6fc8: pop r12
140af6fca: pop r13
140af6fcc: pop r14
140af6fce: pop r15
140af6fd0: pop rbp
140af6fd1: ret 
140af6fd2: nop word ptr cs:[rax + rax]
140af6fe0: mov qword ptr [rsp + 0x10], rdx
140af6fe5: push rbp
140af6fe6: push r15
140af6fe8: push r14
140af6fea: push r13
140af6fec: push r12
140af6fee: push rsi
140af6fef: push rdi
140af6ff0: push rbx
140af6ff1: sub rsp, 0x28
140af6ff5: lea rbp, [rdx + 0x80]
140af6ffc: movzx ebx, byte ptr [rbp + 0x17c]
140af7003: movzx esi, byte ptr [rbp + 0x17d]
140af700a: lea rcx, [rbp + 0xd0]
140af7011: call 0x140878160
140af7016: mov byte ptr [rbp + 0x175], sil
140af701d: mov byte ptr [rbp + 0x174], bl
140af7023: add rsp, 0x28
140af7027: pop rbx
140af7028: pop rdi
140af7029: pop rsi
140af702a: pop r12
140af702c: pop r13
140af702e: pop r14
140af7030: pop r15
140af7032: pop rbp
140af7033: ret 
140af7034: nop word ptr cs:[rax + rax]
140af7040: mov qword ptr [rsp + 0x10], rdx
140af7045: push rbp
140af7046: push r15
140af7048: push r14
140af704a: push r13
140af704c: push r12
140af704e: push rsi
140af704f: push rdi
140af7050: push rbx
140af7051: sub rsp, 0x28
140af7055: lea rbp, [rdx + 0x80]
140af705c: movzx ebx, byte ptr [rbp + 0x174]
140af7063: movzx esi, byte ptr [rbp + 0x175]
140af706a: mov rax, qword ptr [rbp + 0x88]
140af7071: lock dec qword ptr [rax]
140af7075: jne 0x140af7083
140af7077: lea rcx, [rbp + 0x88]
140af707e: call 0x140b5cd70
140af7083: mov byte ptr [rbp + 0x17a], sil
140af708a: mov byte ptr [rbp + 0x179], bl
140af7090: add rsp, 0x28
140af7094: pop rbx
140af7095: pop rdi
140af7096: pop rsi
140af7097: pop r12
140af7099: pop r13
140af709b: pop r14
140af709d: pop r15
140af709f: pop rbp
140af70a0: ret 
140af70a1: nop word ptr cs:[rax + rax]
140af70b0: mov qword ptr [rsp + 0x10], rdx
140af70b5: push rbp
140af70b6: push r15
140af70b8: push r14
140af70ba: push r13
140af70bc: push r12
140af70be: push rsi
140af70bf: push rdi
140af70c0: push rbx
140af70c1: sub rsp, 0x28
140af70c5: lea rbp, [rdx + 0x80]
140af70cc: movzx ebx, byte ptr [rbp + 0x179]
140af70d3: movzx esi, byte ptr [rbp + 0x17a]
140af70da: lea rcx, [rbp - 0x58]
140af70de: call 0x14087d990
140af70e3: mov byte ptr [rbp + 0x17f], sil
140af70ea: mov byte ptr [rbp + 0x17e], bl
140af70f0: add rsp, 0x28
140af70f4: pop rbx
140af70f5: pop rdi
140af70f6: pop rsi
140af70f7: pop r12
140af70f9: pop r13
140af70fb: pop r14
140af70fd: pop r15
140af70ff: pop rbp
140af7100: ret 
140af7101: nop word ptr cs:[rax + rax]
140af7110: mov qword ptr [rsp + 0x10], rdx
140af7115: push rbp
140af7116: push r15
140af7118: push r14
140af711a: push r13
140af711c: push r12
140af711e: push rsi
140af711f: push rdi
140af7120: push rbx
140af7121: sub rsp, 0x28
140af7125: lea rbp, [rdx + 0x80]
140af712c: mov rax, qword ptr [rbp + 0x150]
140af7133: mov qword ptr [rbp + 0xb0], rax
140af713a: lea rcx, [rbp - 0x20]
140af713e: call 0x140894e90
140af7143: nop 
140af7144: add rsp, 0x28
140af7148: pop rbx
140af7149: pop rdi
140af714a: pop rsi
140af714b: pop r12
140af714d: pop r13
140af714f: pop r14
140af7151: pop r15
140af7153: pop rbp
140af7154: ret 
140af7155: nop word ptr cs:[rax + rax]
140af7160: mov qword ptr [rsp + 0x10], rdx
140af7165: push rbp
140af7166: push r15
140af7168: push r14
140af716a: push r13
140af716c: push r12
140af716e: push rsi
140af716f: push rdi
140af7170: push rbx
140af7171: sub rsp, 0x28
140af7175: lea rbp, [rdx + 0x80]
140af717c: lea rcx, [rbp + 0xa8]
140af7183: call 0x14087c820
140af7188: nop 
140af7189: add rsp, 0x28
140af718d: pop rbx
140af718e: pop rdi
140af718f: pop rsi
140af7190: pop r12
140af7192: pop r13
140af7194: pop r14
140af7196: pop r15
140af7198: pop rbp
140af7199: ret 
140af719a: nop word ptr [rax + rax]
140af71a0: mov qword ptr [rsp + 0x10], rdx
140af71a5: push rbp
140af71a6: push r15
140af71a8: push r14
140af71aa: push r13
140af71ac: push r12
140af71ae: push rsi
140af71af: push rdi
140af71b0: push rbx
140af71b1: sub rsp, 0x28
140af71b5: lea rbp, [rdx + 0x80]
140af71bc: cmp qword ptr [rbp + 0x168], 0
140af71c4: je 0x140af71e3
140af71c6: mov rdx, qword ptr [rbp + 0x168]
140af71cd: shl rdx, 3
140af71d1: mov r8d, 8
140af71d7: mov rcx, qword ptr [rbp + 0x160]
140af71de: call 0x140613c20
140af71e3: mov byte ptr [rbp + 0x17f], 1
140af71ea: mov byte ptr [rbp + 0x17e], 1
140af71f1: add rsp, 0x28
140af71f5: pop rbx
140af71f6: pop rdi
140af71f7: pop rsi
140af71f8: pop r12
140af71fa: pop r13
140af71fc: pop r14
140af71fe: pop r15
140af7200: pop rbp
140af7201: ret 
140af7202: nop word ptr cs:[rax + rax]
140af7210: mov qword ptr [rsp + 0x10], rdx
140af7215: push rbp
140af7216: push r15
140af7218: push r14
140af721a: push r13
140af721c: push r12
140af721e: push rsi
140af721f: push rdi
140af7220: push rbx
140af7221: sub rsp, 0x28
140af7225: lea rbp, [rdx + 0x80]
140af722c: mov rsi, qword ptr [rbp + 0x160]
140af7233: mov rbx, qword ptr [rbp + 0x168]
140af723a: cmp rsi, rbx
140af723d: mov rdi, qword ptr [rbp + 0x150]
140af7244: je 0x140af7267
140af7246: nop word ptr cs:[rax + rax]
140af7250: mov rcx, rbx
140af7253: shl rcx, 6
140af7257: add rcx, rdi
140af725a: call 0x140894e90
140af725f: inc rbx
140af7262: cmp rbx, rsi
140af7265: jne 0x140af7250
140af7267: add rsp, 0x28
140af726b: pop rbx
140af726c: pop rdi
140af726d: pop rsi
140af726e: pop r12
140af7270: pop r13
140af7272: pop r14
140af7274: pop r15
140af7276: pop rbp
140af7277: ret 
140af7278: nop dword ptr [rax + rax]
140af7280: mov qword ptr [rsp + 0x10], rdx
140af7285: push rbp
140af7286: push r15
140af7288: push r14
140af728a: push r13
140af728c: push r12
140af728e: push rsi
140af728f: push rdi
140af7290: push rbx
140af7291: sub rsp, 0x28
140af7295: lea rbp, [rdx + 0x80]
140af729c: cmp qword ptr [rbp + 0x108], 0
140af72a4: je 0x140af72c3
140af72a6: mov rdx, qword ptr [rbp + 0x108]
140af72ad: shl rdx, 6
140af72b1: mov r8d, 8
140af72b7: mov rcx, qword ptr [rbp + 0x100]
140af72be: call 0x140613c20
140af72c3: nop 
140af72c4: add rsp, 0x28
140af72c8: pop rbx
140af72c9: pop rdi
140af72ca: pop rsi
140af72cb: pop r12
140af72cd: pop r13
140af72cf: pop r14
140af72d1: pop r15
140af72d3: pop rbp
140af72d4: ret 
140af72d5: nop word ptr cs:[rax + rax]
140af72e0: mov qword ptr [rsp + 0x10], rdx
140af72e5: push rbp
140af72e6: push r15
140af72e8: push r14
140af72ea: push r13
140af72ec: push r12
140af72ee: push rsi
140af72ef: push rdi
140af72f0: push rbx
140af72f1: sub rsp, 0x28
140af72f5: lea rbp, [rdx + 0x80]
140af72fc: mov rdi, qword ptr [rbp + 0x160]
140af7303: mov rbx, qword ptr [rbp + 0x168]
140af730a: cmp rdi, rbx
140af730d: mov rsi, qword ptr [rbp + 0xc8]
140af7314: je 0x140af7338
140af7316: nop word ptr cs:[rax + rax]
140af7320: lea rcx, [rbx + rbx*2]
140af7324: shl rcx, 4
140af7328: add rcx, rsi
140af732b: call 0x140893b10
140af7330: inc rbx
140af7333: cmp rbx, rdi
140af7336: jne 0x140af7320
140af7338: add rsp, 0x28
140af733c: pop rbx
140af733d: pop rdi
140af733e: pop rsi
140af733f: pop r12
140af7341: pop r13
140af7343: pop r14
140af7345: pop r15
140af7347: pop rbp
140af7348: ret 
140af7349: nop dword ptr [rax]
140af7350: mov qword ptr [rsp + 0x10], rdx
140af7355: push rbp
140af7356: push r15
140af7358: push r14
140af735a: push r13
140af735c: push r12
140af735e: push rsi
140af735f: push rdi
140af7360: push rbx
140af7361: sub rsp, 0x28
140af7365: lea rbp, [rdx + 0x80]
140af736c: mov rax, qword ptr [rbp + 0x110]
140af7373: test rax, rax
140af7376: je 0x140af7392
140af7378: shl rax, 4
140af737c: lea rdx, [rax + rax*2]
140af7380: mov r8d, 8
140af7386: mov rcx, qword ptr [rbp + 0xc8]
140af738d: call 0x140613c20
140af7392: nop 
140af7393: add rsp, 0x28
140af7397: pop rbx
140af7398: pop rdi
140af7399: pop rsi
140af739a: pop r12
140af739c: pop r13
140af739e: pop r14
140af73a0: pop r15
140af73a2: pop rbp
140af73a3: ret 
140af73a4: nop word ptr cs:[rax + rax]
140af73b0: mov qword ptr [rsp + 0x10], rdx
140af73b5: push rbp
140af73b6: push r15
140af73b8: push r14
140af73ba: push r13
140af73bc: push r12
140af73be: push rsi
140af73bf: push rdi
140af73c0: push rbx
140af73c1: sub rsp, 0x28
140af73c5: lea rbp, [rdx + 0x80]
140af73cc: movzx ebx, byte ptr [rbp + 0x17e]
140af73d3: movzx esi, byte ptr [rbp + 0x17f]
140af73da: mov rdx, qword ptr [rbp + 0x130]
140af73e1: test rdx, rdx
140af73e4: je 0x140af73fc
140af73e6: mov rcx, qword ptr [rbp + 0x138]
140af73ed: shl rdx, 3
140af73f1: mov r8d, 8
140af73f7: call 0x140613c20
140af73fc: mov byte ptr [rbp + 0x177], sil
140af7403: mov byte ptr [rbp + 0x176], bl
140af7409: add rsp, 0x28
140af740d: pop rbx
140af740e: pop rdi
140af740f: pop rsi
140af7410: pop r12
140af7412: pop r13
140af7414: pop r14
140af7416: pop r15
140af7418: pop rbp
140af7419: ret 
140af741a: nop word ptr [rax + rax]
140af7420: mov qword ptr [rsp + 0x10], rdx
140af7425: push rbp
140af7426: push r15
140af7428: push r14
140af742a: push r13
140af742c: push r12
140af742e: push rsi
140af742f: push rdi
140af7430: push rbx
140af7431: sub rsp, 0x28
140af7435: lea rbp, [rdx + 0x80]
140af743c: movzx ebx, byte ptr [rbp + 0x176]
140af7443: movzx esi, byte ptr [rbp + 0x177]
140af744a: lea rcx, [rbp + 0xe8]
140af7451: call 0x1405fdeb0
140af7456: test sil, 1
140af745a: je 0x140af748c
140af745c: mov rax, qword ptr [rbp + 0x158]
140af7463: lock dec qword ptr [rax]
140af7467: jne 0x140af7475
140af7469: lea rcx, [rbp + 0x158]
140af7470: call 0x140b5c420
140af7475: mov byte ptr [rbp + 0x178], bl
140af747b: add rsp, 0x28
140af747f: pop rbx
140af7480: pop rdi
140af7481: pop rsi
140af7482: pop r12
140af7484: pop r13
140af7486: pop r14
140af7488: pop r15
140af748a: pop rbp
140af748b: ret 
140af748c: mov byte ptr [rbp + 0x178], bl
140af7492: add rsp, 0x28
140af7496: pop rbx
140af7497: pop rdi
140af7498: pop rsi
140af7499: pop r12
140af749b: pop r13
140af749d: pop r14
140af749f: pop r15
140af74a1: pop rbp
140af74a2: ret 
140af74a3: nop word ptr cs:[rax + rax]
140af74b0: mov qword ptr [rsp + 0x10], rdx
140af74b5: push rbp
140af74b6: push r15
140af74b8: push r14
140af74ba: push r13
140af74bc: push r12
140af74be: push rsi
140af74bf: push rdi
140af74c0: push rbx
140af74c1: sub rsp, 0x28
140af74c5: lea rbp, [rdx + 0x80]
140af74cc: test byte ptr [rbp + 0x178], 1
140af74d3: je 0x140af7500
140af74d5: mov rax, qword ptr [rbp + 0x148]
140af74dc: lock dec qword ptr [rax]
140af74e0: jne 0x140af74ee
140af74e2: lea rcx, [rbp + 0x148]
140af74e9: call 0x140b5cb60
140af74ee: nop 
140af74ef: add rsp, 0x28
140af74f3: pop rbx
140af74f4: pop rdi
140af74f5: pop rsi
140af74f6: pop r12
140af74f8: pop r13
140af74fa: pop r14
140af74fc: pop r15
140af74fe: pop rbp
140af74ff: ret 
140af7500: add rsp, 0x28
140af7504: pop rbx
140af7505: pop rdi
140af7506: pop rsi
140af7507: pop r12
140af7509: pop r13
140af750b: pop r14
140af750d: pop r15
140af750f: pop rbp
140af7510: ret 
140af7511: nop word ptr cs:[rax + rax]
140af7520: mov qword ptr [rsp + 0x10], rdx
140af7525: push rbp
140af7526: push r15
140af7528: push r14
140af752a: push r13
140af752c: push r12
140af752e: push rsi
140af752f: push rdi
140af7530: push rbx
140af7531: sub rsp, 0x28
140af7535: lea rbp, [rdx + 0x80]
140af753c: lea rcx, [rbp + 0x110]
140af7543: call 0x1408772e0
140af7548: nop 
140af7549: add rsp, 0x28
140af754d: pop rbx
140af754e: pop rdi
140af754f: pop rsi
140af7550: pop r12
140af7552: pop r13
140af7554: pop r14
140af7556: pop r15
140af7558: pop rbp
140af7559: ret 
140af755a: nop word ptr [rax + rax]
140af7560: mov qword ptr [rsp + 0x10], rdx
140af7565: push rbp
140af7566: push r15
140af7568: push r14
140af756a: push r13
140af756c: push r12
140af756e: push rsi
140af756f: push rdi
140af7570: push rbx
140af7571: sub rsp, 0x28
140af7575: lea rbp, [rdx + 0x80]
140af757c: mov rax, qword ptr [rbp - 0x40]
140af7580: cmp rax, qword ptr [rbp + 0x80]
140af7587: jne 0x140af759a
140af7589: add rsp, 0x28
140af758d: pop rbx
140af758e: pop rdi
140af758f: pop rsi
140af7590: pop r12
140af7592: pop r13
140af7594: pop r14
140af7596: pop r15
140af7598: pop rbp
140af7599: ret 
140af759a: lea rcx, [rbp - 0x38]
140af759e: call 0x1408772e0
140af75a3: jmp 0x140af7589

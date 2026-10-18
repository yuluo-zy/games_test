1408320e0: push r15
1408320e2: push r14
1408320e4: push r13
1408320e6: push r12
1408320e8: push rsi
1408320e9: push rdi
1408320ea: push rbp
1408320eb: push rbx
1408320ec: sub rsp, 0x28
1408320f0: mov rbx, r8
1408320f3: mov rdi, rdx
1408320f6: mov rsi, rcx
1408320f9: xor ebp, ebp
1408320fb: lea rax, [r8 - 4]
1408320ff: cmp rax, 8
140832103: ja 0x14083232c
140832109: lea rcx, [rip + 0x22602a4]
140832110: movsxd rdx, dword ptr [rcx + rax*4]
140832114: add rdx, rcx
140832117: xor r15d, r15d
14083211a: xor eax, eax
14083211c: jmp rdx
14083211e: cmp dword ptr [rdi], 0x68746170
140832124: je 0x1408325e0
14083212a: lea rdx, [rip + 0x22601a2]
140832131: mov rcx, rdi
140832134: mov r8, rbx
140832137: call 0x142923bf0
14083213c: test eax, eax
14083213e: je 0x140832643
140832144: mov al, 1
140832146: xor ebp, ebp
140832148: xor r15d, r15d
14083214b: cmp rbx, 5
14083214f: sete r14b
140832153: je 0x14083233b
140832159: jmp 0x140832351
14083215e: movabs rax, 0x6769725f65766f6d
140832168: xor rax, qword ptr [rdi]
14083216b: movzx ecx, word ptr [rdi + 8]
14083216f: xor rcx, 0x7468
140832176: or rcx, rax
140832179: je 0x1408325b3
14083217f: movabs rax, 0x6f7461726f636564
140832189: xor rax, qword ptr [rdi]
14083218c: movzx ecx, word ptr [rdi + 8]
140832190: xor rcx, 0x7372
140832197: or rcx, rax
14083219a: je 0x140832616
1408321a0: mov r15b, 1
1408321a3: xor ebp, ebp
1408321a5: xor eax, eax
1408321a7: cmp rbx, 5
1408321ab: sete r14b
1408321af: je 0x14083233b
1408321b5: jmp 0x140832351
1408321ba: mov eax, 0x73617265
1408321bf: xor eax, dword ptr [rdi]
1408321c1: movzx ecx, word ptr [rdi + 4]
1408321c5: xor ecx, 0x7265
1408321cb: or ecx, eax
1408321cd: je 0x1408325bc
1408321d3: mov eax, 0x64726167
1408321d8: xor eax, dword ptr [rdi]
1408321da: movzx ecx, word ptr [rdi + 4]
1408321de: xor ecx, 0x6e65
1408321e4: or ecx, eax
1408321e6: je 0x14083261f
1408321ec: xor ebp, ebp
1408321ee: jmp 0x14083232c
1408321f3: movabs rax, 0x6c5f657461746f72
1408321fd: xor rax, qword ptr [rdi]
140832200: movabs rcx, 0x7466656c5f657461
14083220a: xor rcx, qword ptr [rdi + 3]
14083220e: or rcx, rax
140832211: je 0x1408325c5
140832217: movabs rax, 0x5f666f5f656d6974
140832221: xor rax, qword ptr [rdi]
140832224: movabs rcx, 0x7961645f666f5f65
14083222e: xor rcx, qword ptr [rdi + 3]
140832232: or rcx, rax
140832235: je 0x140832628
14083223b: lea rdx, [rip + 0x2260121]
140832242: mov rcx, rdi
140832245: mov r8, rbx
140832248: call 0x142923bf0
14083224d: test eax, eax
14083224f: je 0x14083275e
140832255: lea rdx, [rip + 0x2260112]
14083225c: mov rcx, rdi
14083225f: mov r8, rbx
140832262: call 0x142923bf0
140832267: test eax, eax
140832269: jne 0x140832755
14083226f: mov byte ptr [rsi + 1], 0x23
140832273: jmp 0x1408327a3
140832278: movabs rax, 0x726f665f65766f6d
140832282: xor rax, qword ptr [rdi]
140832285: mov ecx, dword ptr [rdi + 8]
140832288: xor rcx, 0x64726177
14083228f: or rcx, rax
140832292: je 0x1408325ce
140832298: movabs rax, 0x725f657461746f72
1408322a2: xor rax, qword ptr [rdi]
1408322a5: mov ecx, dword ptr [rdi + 8]
1408322a8: xor rcx, 0x74686769
1408322af: or rcx, rax
1408322b2: je 0x140832631
1408322b8: movabs rax, 0x6f725f6573756f6d
1408322c2: xor rax, qword ptr [rdi]
1408322c5: mov ecx, dword ptr [rdi + 8]
1408322c8: xor rcx, 0x65746174
1408322cf: or rcx, rax
1408322d2: jne 0x140832755
1408322d8: mov byte ptr [rsi + 1], 0x24
1408322dc: jmp 0x1408327a3
1408322e1: mov eax, 0x65766f6d
1408322e6: xor eax, dword ptr [rdi]
1408322e8: mov ecx, 0x70755f65
1408322ed: xor ecx, dword ptr [rdi + 3]
1408322f0: or ecx, eax
1408322f2: je 0x1408325d7
1408322f8: mov eax, 0x6d6f6f7a
1408322fd: xor eax, dword ptr [rdi]
1408322ff: mov ecx, 0x6e695f6d
140832304: xor ecx, dword ptr [rdi + 3]
140832307: or ecx, eax
140832309: je 0x14083263a
14083230f: lea rdx, [rip + 0x225ffb0]
140832316: mov rcx, rdi
140832319: mov r8, rbx
14083231c: call 0x142923bf0
140832321: test eax, eax
140832323: je 0x140832764
140832329: mov bpl, 1
14083232c: xor r15d, r15d
14083232f: xor eax, eax
140832331: cmp rbx, 5
140832335: sete r14b
140832339: jne 0x140832351
14083233b: mov ecx, 0x65746177
140832340: xor ecx, dword ptr [rdi]
140832342: movzx edx, byte ptr [rdi + 4]
140832346: xor edx, 0x72
140832349: or edx, ecx
14083234b: je 0x1408325aa
140832351: test al, al
140832353: je 0x140832389
140832355: lea rdx, [rip + 0x225ff94]
14083235c: mov rcx, rdi
14083235f: mov r8, rbx
140832362: call 0x142923bf0
140832367: test eax, eax
140832369: je 0x1408325fb
14083236f: lea rdx, [rip + 0x225ff7e]
140832376: mov rcx, rdi
140832379: mov r8, rbx
14083237c: call 0x142923bf0
140832381: test eax, eax
140832383: je 0x14083265e
140832389: test r15b, r15b
14083238c: je 0x1408324d4
140832392: lea rdx, [rip + 0x225ff5f]
140832399: mov rcx, rdi
14083239c: mov r8, rbx
14083239f: call 0x142923bf0
1408323a4: test eax, eax
1408323a6: je 0x14083260d
1408323ac: mov r12b, 1
1408323af: xor r15d, r15d
1408323b2: cmp rbx, 0x10
1408323b6: je 0x1408324e0
1408323bc: jmp 0x1408324fb
1408323c1: movabs rax, 0x74756f5f6d6f6f7a
1408323cb: cmp qword ptr [rdi], rax
1408323ce: je 0x1408325e9
1408323d4: movabs rax, 0x676e69646c697562
1408323de: cmp qword ptr [rdi], rax
1408323e1: je 0x14083264c
1408323e7: mov r15b, 1
1408323ea: xor ebp, ebp
1408323ec: xor r14d, r14d
1408323ef: xor r12d, r12d
1408323f2: cmp rbx, 0x10
1408323f6: je 0x1408324e0
1408323fc: jmp 0x1408324fb
140832401: movabs rax, 0x6361625f65766f6d
14083240b: xor rax, qword ptr [rdi]
14083240e: movzx ecx, byte ptr [rdi + 8]
140832412: xor rcx, 0x6b
140832416: or rcx, rax
140832419: je 0x1408325f2
14083241f: movabs rax, 0x66656c5f65766f6d
140832429: xor rax, qword ptr [rdi]
14083242c: movzx ecx, byte ptr [rdi + 8]
140832430: xor rcx, 0x74
140832434: or rcx, rax
140832437: je 0x140832655
14083243d: movabs rax, 0x776f645f65766f6d
140832447: xor rax, qword ptr [rdi]
14083244a: movzx ecx, byte ptr [rdi + 8]
14083244e: xor rcx, 0x6e
140832452: or rcx, rax
140832455: je 0x14083276a
14083245b: movabs rax, 0x7a696d6f74737563
140832465: xor rax, qword ptr [rdi]
140832468: movzx ecx, byte ptr [rdi + 8]
14083246c: xor rcx, 0x65
140832470: or rcx, rax
140832473: je 0x14083277c
140832479: movabs rax, 0x646f6d6f746f6870
140832483: xor rax, qword ptr [rdi]
140832486: movzx ecx, byte ptr [rdi + 8]
14083248a: xor rcx, 0x65
14083248e: or rcx, rax
140832491: je 0x140832794
140832497: lea rdx, [rip + 0x225fe97]
14083249e: mov rcx, rdi
1408324a1: mov r8, rbx
1408324a4: call 0x142923bf0
1408324a9: test eax, eax
1408324ab: je 0x14083279a
1408324b1: lea rdx, [rip + 0x225fecd]
1408324b8: mov rcx, rdi
1408324bb: mov r8, rbx
1408324be: call 0x142923bf0
1408324c3: test eax, eax
1408324c5: jne 0x14083271d
1408324cb: mov byte ptr [rsi + 1], 0x25
1408324cf: jmp 0x1408327a3
1408324d4: xor r15d, r15d
1408324d7: xor r12d, r12d
1408324da: cmp rbx, 0x10
1408324de: jne 0x1408324fb
1408324e0: movdqu xmm0, xmmword ptr [rdi]
1408324e4: pcmpeqb xmm0, xmmword ptr [rip + 0x2193554]
1408324ec: pmovmskb eax, xmm0
1408324f0: cmp eax, 0xffff
1408324f5: je 0x140832604
1408324fb: test r12b, r12b
1408324fe: je 0x14083251a
140832500: lea rdx, [rip + 0x225fe0b]
140832507: mov rcx, rdi
14083250a: mov r8, rbx
14083250d: call 0x142923bf0
140832512: test eax, eax
140832514: je 0x140832667
14083251a: test r15b, r15b
14083251d: je 0x140832546
14083251f: lea rdx, [rip + 0x225fdf6]
140832526: mov rcx, rdi
140832529: mov r8, rbx
14083252c: call 0x142923bf0
140832531: test eax, eax
140832533: je 0x140832714
140832539: mov r13b, 1
14083253c: test r12b, r12b
14083253f: jne 0x140832584
140832541: jmp 0x140832673
140832546: cmp rbx, 0x11
14083254a: jne 0x14083257b
14083254c: movdqu xmm0, xmmword ptr [rdi]
140832550: movzx eax, byte ptr [rdi + 0x10]
140832554: movd xmm1, eax
140832558: pcmpeqb xmm0, xmmword ptr [rip + 0x21934d0]
140832560: pcmpeqb xmm1, xmmword ptr [rip + 0x2105978]
140832568: pand xmm1, xmm0
14083256c: pmovmskb eax, xmm1
140832570: cmp eax, 0xffff
140832575: je 0x140832776
14083257b: test r12b, r12b
14083257e: je 0x140832670
140832584: lea rdx, [rip + 0x225fdb3]
14083258b: mov rcx, rdi
14083258e: mov r8, rbx
140832591: call 0x142923bf0
140832596: mov r13d, r15d
140832599: test eax, eax
14083259b: jne 0x140832673
1408325a1: mov byte ptr [rsi + 1], 0x1d
1408325a5: jmp 0x1408327a3
1408325aa: mov byte ptr [rsi + 1], 0x12
1408325ae: jmp 0x1408327a3
1408325b3: mov byte ptr [rsi + 1], 3
1408325b7: jmp 0x1408327a3
1408325bc: mov byte ptr [rsi + 1], 0xe
1408325c0: jmp 0x1408327a3
1408325c5: mov byte ptr [rsi + 1], 6
1408325c9: jmp 0x1408327a3
1408325ce: mov byte ptr [rsi + 1], 0
1408325d2: jmp 0x1408327a3
1408325d7: mov byte ptr [rsi + 1], 4
1408325db: jmp 0x1408327a3
1408325e0: mov byte ptr [rsi + 1], 0xb
1408325e4: jmp 0x1408327a3
1408325e9: mov byte ptr [rsi + 1], 9
1408325ed: jmp 0x1408327a3
1408325f2: mov byte ptr [rsi + 1], 1
1408325f6: jmp 0x1408327a3
1408325fb: mov byte ptr [rsi + 1], 0x15
1408325ff: jmp 0x1408327a3
140832604: mov byte ptr [rsi + 1], 0x18
140832608: jmp 0x1408327a3
14083260d: mov byte ptr [rsi + 1], 0x17
140832611: jmp 0x1408327a3
140832616: mov byte ptr [rsi + 1], 0xc
14083261a: jmp 0x1408327a3
14083261f: mov byte ptr [rsi + 1], 0x10
140832623: jmp 0x1408327a3
140832628: mov byte ptr [rsi + 1], 0x14
14083262c: jmp 0x1408327a3
140832631: mov byte ptr [rsi + 1], 7
140832635: jmp 0x1408327a3
14083263a: mov byte ptr [rsi + 1], 8
14083263e: jmp 0x1408327a3
140832643: mov byte ptr [rsi + 1], 0x11
140832647: jmp 0x1408327a3
14083264c: mov byte ptr [rsi + 1], 0xa
140832650: jmp 0x1408327a3
140832655: mov byte ptr [rsi + 1], 2
140832659: jmp 0x1408327a3
14083265e: mov byte ptr [rsi + 1], 0x16
140832662: jmp 0x1408327a3
140832667: mov byte ptr [rsi + 1], 0x19
14083266b: jmp 0x1408327a3
140832670: xor r13d, r13d
140832673: test bpl, bpl
140832676: je 0x140832692
140832678: lea rdx, [rip + 0x225fcc9]
14083267f: mov rcx, rdi
140832682: mov r8, rbx
140832685: call 0x142923bf0
14083268a: test eax, eax
14083268c: je 0x140832770
140832692: test r13b, r13b
140832695: je 0x1408326b1
140832697: lea rdx, [rip + 0x225fcb1]
14083269e: mov rcx, rdi
1408326a1: mov r8, rbx
1408326a4: call 0x142923bf0
1408326a9: test eax, eax
1408326ab: je 0x140832782
1408326b1: test bpl, bpl
1408326b4: je 0x1408326d0
1408326b6: lea rdx, [rip + 0x225fc9a]
1408326bd: mov rcx, rdi
1408326c0: mov r8, rbx
1408326c3: call 0x142923bf0
1408326c8: test eax, eax
1408326ca: je 0x140832788
1408326d0: test r14b, r14b
1408326d3: je 0x1408326ef
1408326d5: lea rdx, [rip + 0x225fc82]
1408326dc: mov rcx, rdi
1408326df: mov r8, rbx
1408326e2: call 0x142923bf0
1408326e7: test eax, eax
1408326e9: je 0x14083278e
1408326ef: cmp rbx, 0x10
1408326f3: jne 0x14083271d
1408326f5: lea rdx, [rip + 0x225fc92]
1408326fc: mov rcx, rdi
1408326ff: mov r8, rbx
140832702: call 0x142923bf0
140832707: test eax, eax
140832709: jne 0x140832755
14083270b: mov byte ptr [rsi + 1], 0x26
14083270f: jmp 0x1408327a3
140832714: mov byte ptr [rsi + 1], 0x1a
140832718: jmp 0x1408327a3
14083271d: cmp rbx, 0x13
140832721: jne 0x140832755
140832723: movdqu xmm0, xmmword ptr [rdi]
140832727: movdqu xmm1, xmmword ptr [rdi + 3]
14083272c: pcmpeqb xmm1, xmmword ptr [rip + 0x21932dc]
140832734: pcmpeqb xmm0, xmmword ptr [rip + 0x21932e4]
14083273c: pand xmm0, xmm1
140832740: pmovmskb ecx, xmm0
140832744: lea rax, [rsi + 1]
140832748: cmp ecx, 0xffff
14083274e: je 0x1408327a0
140832750: mov byte ptr [rax], 0x28
140832753: jmp 0x1408327a3
140832755: lea rax, [rsi + 1]
140832759: mov byte ptr [rax], 0x28
14083275c: jmp 0x1408327a3
14083275e: mov byte ptr [rsi + 1], 0x22
140832762: jmp 0x1408327a3
140832764: mov byte ptr [rsi + 1], 0xf
140832768: jmp 0x1408327a3
14083276a: mov byte ptr [rsi + 1], 5
14083276e: jmp 0x1408327a3
140832770: mov byte ptr [rsi + 1], 0x1e
140832774: jmp 0x1408327a3
140832776: mov byte ptr [rsi + 1], 0x1b
14083277a: jmp 0x1408327a3
14083277c: mov byte ptr [rsi + 1], 0xd
140832780: jmp 0x1408327a3
140832782: mov byte ptr [rsi + 1], 0x1f
140832786: jmp 0x1408327a3
140832788: mov byte ptr [rsi + 1], 0x20
14083278c: jmp 0x1408327a3
14083278e: mov byte ptr [rsi + 1], 0x21
140832792: jmp 0x1408327a3
140832794: mov byte ptr [rsi + 1], 0x13
140832798: jmp 0x1408327a3
14083279a: mov byte ptr [rsi + 1], 0x1c
14083279e: jmp 0x1408327a3
1408327a0: mov byte ptr [rax], 0x27
1408327a3: mov byte ptr [rsi], 0x32
1408327a6: mov rax, rsi
1408327a9: add rsp, 0x28
1408327ad: pop rbx
1408327ae: pop rbp
1408327af: pop rdi
1408327b0: pop rsi
1408327b1: pop r12
1408327b3: pop r13
1408327b5: pop r14
1408327b7: pop r15
1408327b9: ret 

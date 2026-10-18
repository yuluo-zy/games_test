
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405ba5f0: 55                          	push	rbp
1405ba5f1: 41 57                       	push	r15
1405ba5f3: 41 56                       	push	r14
1405ba5f5: 41 55                       	push	r13
1405ba5f7: 41 54                       	push	r12
1405ba5f9: 56                          	push	rsi
1405ba5fa: 57                          	push	rdi
1405ba5fb: 53                          	push	rbx
1405ba5fc: 48 81 ec 18 04 00 00        	sub	rsp, 0x418
1405ba603: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1405ba60b: 48 c7 85 90 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x390], -0x2
1405ba616: 4c 89 85 78 03 00 00        	mov	qword ptr [rbp + 0x378], r8
1405ba61d: 48 89 d3                    	mov	rbx, rdx
1405ba620: 48 89 ce                    	mov	rsi, rcx
1405ba623: 48 8b 95 00 04 00 00        	mov	rdx, qword ptr [rbp + 0x400]
1405ba62a: 48 8b 85 08 04 00 00        	mov	rax, qword ptr [rbp + 0x408]
1405ba631: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
1405ba635: 48 89 8d f0 01 00 00        	mov	qword ptr [rbp + 0x1f0], rcx
1405ba63c: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1405ba63f: 0f 29 85 e0 01 00 00        	movaps	xmmword ptr [rbp + 0x1e0], xmm0
1405ba646: 8b 02                       	mov	eax, dword ptr [rdx]
1405ba648: 85 c0                       	test	eax, eax
1405ba64a: 0f 84 fb 06 00 00           	je	0x1405bad4b <.text+0x5b9d4b>
1405ba650: 4d 89 ce                    	mov	r14, r9
1405ba653: 48 8b bd e0 01 00 00        	mov	rdi, qword ptr [rbp + 0x1e0]
1405ba65a: 48 85 ff                    	test	rdi, rdi
1405ba65d: 74 05                       	je	0x1405ba664 <.text+0x5b9664>
1405ba65f: 83 c8 02                    	or	eax, 0x2
1405ba662: 89 02                       	mov	dword ptr [rdx], eax
1405ba664: 48 85 ff                    	test	rdi, rdi
1405ba667: 41 0f 95 c0                 	setne	r8b
1405ba66b: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
1405ba66f: e8 5c b8 61 00              	call	0x140bd5ed0 <.text+0xbd4ed0>
1405ba674: 83 bd 94 00 00 00 00        	cmp	dword ptr [rbp + 0x94], 0x0
1405ba67b: 0f 84 e2 06 00 00           	je	0x1405bad63 <.text+0x5b9d63>
1405ba681: 83 bd 98 00 00 00 00        	cmp	dword ptr [rbp + 0x98], 0x0
1405ba688: 0f 84 ed 06 00 00           	je	0x1405bad7b <.text+0x5b9d7b>
1405ba68e: 83 bd 9c 00 00 00 00        	cmp	dword ptr [rbp + 0x9c], 0x0
1405ba695: 0f 84 f8 06 00 00           	je	0x1405bad93 <.text+0x5b9d93>
1405ba69b: 48 8b 8b 40 06 00 00        	mov	rcx, qword ptr [rbx + 0x640]
1405ba6a2: 48 8d 55 78                 	lea	rdx, [rbp + 0x78]
1405ba6a6: 4c 8d 8d 70 02 00 00        	lea	r9, [rbp + 0x270]
1405ba6ad: 45 31 c0                    	xor	r8d, r8d
1405ba6b0: ff 93 a0 01 00 00           	call	qword ptr [rbx + 0x1a0]
1405ba6b6: 85 c0                       	test	eax, eax
1405ba6b8: 0f 85 16 07 00 00           	jne	0x1405badd4 <.text+0x5b9dd4>
1405ba6be: 4c 8b a5 70 02 00 00        	mov	r12, qword ptr [rbp + 0x270]
1405ba6c5: 48 89 d9                    	mov	rcx, rbx
1405ba6c8: e8 93 e2 63 00              	call	0x140bf8960 <.text+0xbf7960>
1405ba6cd: 48 85 c0                    	test	rax, rax
1405ba6d0: 0f 84 a9 00 00 00           	je	0x1405ba77f <.text+0x5b977f>
1405ba6d6: 49 89 c5                    	mov	r13, rax
1405ba6d9: 4d 89 f0                    	mov	r8, r14
1405ba6dc: 48 89 b5 80 03 00 00        	mov	qword ptr [rbp + 0x380], rsi
1405ba6e3: 4c 89 e6                    	mov	rsi, r12
1405ba6e6: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
1405ba6ed: 48 8b 95 78 03 00 00        	mov	rdx, qword ptr [rbp + 0x378]
1405ba6f4: e8 d7 f0 7b 00              	call	0x140d797d0 <.text+0xd787d0>
1405ba6f9: 31 c0                       	xor	eax, eax
1405ba6fb: 48 3b 85 d0 00 00 00        	cmp	rax, qword ptr [rbp + 0xd0]
1405ba702: 0f 81 3a 07 00 00           	jno	0x1405bae42 <.text+0x5b9e42>
1405ba708: 4c 8b a5 d8 00 00 00        	mov	r12, qword ptr [rbp + 0xd8]
1405ba70f: 4c 8b bd e0 00 00 00        	mov	r15, qword ptr [rbp + 0xe0]
1405ba716: c7 85 70 02 00 00 00 be 9c 3b       	mov	dword ptr [rbp + 0x270], 0x3b9cbe00
1405ba720: 48 c7 85 78 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x278], 0x0
1405ba72b: c7 85 80 02 00 00 0a 00 00 00       	mov	dword ptr [rbp + 0x280], 0xa
1405ba735: 48 89 b5 88 02 00 00        	mov	qword ptr [rbp + 0x288], rsi
1405ba73c: 4c 89 a5 90 02 00 00        	mov	qword ptr [rbp + 0x290], r12
1405ba743: 49 8b 4d 40                 	mov	rcx, qword ptr [r13 + 0x40]
1405ba747: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
1405ba74e: 41 ff 55 00                 	call	qword ptr [r13]
1405ba752: 85 c0                       	test	eax, eax
1405ba754: 0f 85 31 07 00 00           	jne	0x1405bae8b <.text+0x5b9e8b>
1405ba75a: 41 c6 04 24 00              	mov	byte ptr [r12], 0x0
1405ba75f: 4d 85 ff                    	test	r15, r15
1405ba762: 4c 89 e1                    	mov	rcx, r12
1405ba765: 49 89 f4                    	mov	r12, rsi
1405ba768: 48 8b b5 80 03 00 00        	mov	rsi, qword ptr [rbp + 0x380]
1405ba76f: 74 0e                       	je	0x1405ba77f <.text+0x5b977f>
1405ba771: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405ba777: 4c 89 fa                    	mov	rdx, r15
1405ba77a: e8 a1 94 05 00              	call	0x140613c20 <.text+0x612c20>
1405ba77f: 4d 89 f7                    	mov	r15, r14
1405ba782: 48 8b 8b 40 06 00 00        	mov	rcx, qword ptr [rbx + 0x640]
1405ba789: 4c 8d 85 50 03 00 00        	lea	r8, [rbp + 0x350]
1405ba790: 4c 89 e2                    	mov	rdx, r12
1405ba793: ff 93 f0 00 00 00           	call	qword ptr [rbx + 0xf0]
1405ba799: 4c 8b ab 78 06 00 00        	mov	r13, qword ptr [rbx + 0x678]
1405ba7a0: 4d 8d 75 10                 	lea	r14, [r13 + 0x10]
1405ba7a4: b1 01                       	mov	cl, 0x1
1405ba7a6: 31 c0                       	xor	eax, eax
1405ba7a8: f0                          	lock
1405ba7a9: 41 0f b0 4d 10              	cmpxchg	byte ptr [r13 + 0x10], cl
1405ba7ae: 0f 85 f7 05 00 00           	jne	0x1405badab <.text+0x5b9dab>
1405ba7b4: 4c 89 b5 68 03 00 00        	mov	qword ptr [rbp + 0x368], r14
1405ba7bb: 4c 89 f1                    	mov	rcx, r14
1405ba7be: e8 ad 74 6c 00              	call	0x140c81c70 <.text+0xc80c70>
1405ba7c3: 49 83 c5 18                 	add	r13, 0x18
1405ba7c7: 48 8b 85 78 03 00 00        	mov	rax, qword ptr [rbp + 0x378]
1405ba7ce: 48 89 85 e0 00 00 00        	mov	qword ptr [rbp + 0xe0], rax
1405ba7d5: 4d 89 fe                    	mov	r14, r15
1405ba7d8: 4c 89 bd e8 00 00 00        	mov	qword ptr [rbp + 0xe8], r15
1405ba7df: 0f 10 85 50 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x350]
1405ba7e6: 0f 11 85 f0 00 00 00        	movups	xmmword ptr [rbp + 0xf0], xmm0
1405ba7ed: 48 8b 85 60 03 00 00        	mov	rax, qword ptr [rbp + 0x360]
1405ba7f4: 48 89 85 00 01 00 00        	mov	qword ptr [rbp + 0x100], rax
1405ba7fb: 66 c7 85 08 01 00 00 00 01  	mov	word ptr [rbp + 0x108], 0x100
1405ba804: 48 c7 85 d0 00 00 00 02 00 00 00    	mov	qword ptr [rbp + 0xd0], 0x2
1405ba80f: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1405ba816: 4c 8d 85 d0 00 00 00        	lea	r8, [rbp + 0xd0]
1405ba81d: 4c 89 ea                    	mov	rdx, r13
1405ba820: e8 1b 32 65 00              	call	0x140c0da40 <.text+0xc0ca40>
1405ba825: 48 89 bd 68 02 00 00        	mov	qword ptr [rbp + 0x268], rdi
1405ba82c: 80 bd b4 02 00 00 02        	cmp	byte ptr [rbp + 0x2b4], 0x2
1405ba833: 48 8b 85 70 02 00 00        	mov	rax, qword ptr [rbp + 0x270]
1405ba83a: 48 89 85 80 03 00 00        	mov	qword ptr [rbp + 0x380], rax
1405ba841: 4c 8b ad 88 02 00 00        	mov	r13, qword ptr [rbp + 0x288]
1405ba848: 0f 85 9a 00 00 00           	jne	0x1405ba8e8 <.text+0x5b98e8>
1405ba84e: 48 8b 85 78 02 00 00        	mov	rax, qword ptr [rbp + 0x278]
1405ba855: 48 89 85 70 03 00 00        	mov	qword ptr [rbp + 0x370], rax
1405ba85c: 4c 8b bd 80 02 00 00        	mov	r15, qword ptr [rbp + 0x280]
1405ba863: 0f b6 05 9f a3 07 03        	movzx	eax, byte ptr [rip + 0x307a39f] # 0x143634c09
1405ba86a: bf 0f 00 00 00              	mov	edi, 0xf
1405ba86f: b9 0f 00 00 00              	mov	ecx, 0xf
1405ba874: ba 01 00 00 00              	mov	edx, 0x1
1405ba879: e8 92 93 05 00              	call	0x140613c10 <.text+0x612c10>
1405ba87e: 48 85 c0                    	test	rax, rax
1405ba881: 0f 84 45 06 00 00           	je	0x1405baecc <.text+0x5b9ecc>
1405ba887: 48 b9 20 74 65 78 74 75 72 65       	movabs	rcx, 0x6572757478657420
1405ba891: 48 89 48 07                 	mov	qword ptr [rax + 0x7], rcx
1405ba895: 48 b9 47 70 75 4f 6e 6c 79 20       	movabs	rcx, 0x20796c6e4f757047
1405ba89f: 48 89 08                    	mov	qword ptr [rax], rcx
1405ba8a2: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
1405ba8a9: 48 89 8d a0 01 00 00        	mov	qword ptr [rbp + 0x1a0], rcx
1405ba8b0: 0f 10 85 50 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x350]
1405ba8b7: 0f 29 85 90 01 00 00        	movaps	xmmword ptr [rbp + 0x190], xmm0
1405ba8be: 48 8b 8d 80 03 00 00        	mov	rcx, qword ptr [rbp + 0x380]
1405ba8c5: 48 83 f9 0b                 	cmp	rcx, 0xb
1405ba8c9: 0f 85 94 03 00 00           	jne	0x1405bac63 <.text+0x5b9c63>
1405ba8cf: 4c 89 e9                    	mov	rcx, r13
1405ba8d2: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1405ba8d9: 48 89 95 80 03 00 00        	mov	qword ptr [rbp + 0x380], rdx
1405ba8e0: 41 bd 0f 00 00 00           	mov	r13d, 0xf
1405ba8e6: eb 38                       	jmp	0x1405ba920 <.text+0x5b9920>
1405ba8e8: 4c 8b bd 78 02 00 00        	mov	r15, qword ptr [rbp + 0x278]
1405ba8ef: 48 8b 8d 80 02 00 00        	mov	rcx, qword ptr [rbp + 0x280]
1405ba8f6: 48 8b 85 90 02 00 00        	mov	rax, qword ptr [rbp + 0x290]
1405ba8fd: 48 8b bd 98 02 00 00        	mov	rdi, qword ptr [rbp + 0x298]
1405ba904: 0f 10 85 a0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2a0]
1405ba90b: 0f 29 85 90 01 00 00        	movaps	xmmword ptr [rbp + 0x190], xmm0
1405ba912: 48 8b 95 b0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2b0]
1405ba919: 48 89 95 a0 01 00 00        	mov	qword ptr [rbp + 0x1a0], rdx
1405ba920: 48 8b 95 a0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1a0]
1405ba927: 48 89 95 10 02 00 00        	mov	qword ptr [rbp + 0x210], rdx
1405ba92e: 0f 28 85 90 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x190]
1405ba935: 0f 29 85 00 02 00 00        	movaps	xmmword ptr [rbp + 0x200], xmm0
1405ba93c: 4c 8b 85 80 03 00 00        	mov	r8, qword ptr [rbp + 0x380]
1405ba943: 4c 89 85 20 02 00 00        	mov	qword ptr [rbp + 0x220], r8
1405ba94a: 4c 89 bd 28 02 00 00        	mov	qword ptr [rbp + 0x228], r15
1405ba951: 48 89 8d 30 02 00 00        	mov	qword ptr [rbp + 0x230], rcx
1405ba958: 0f 11 85 50 02 00 00        	movups	xmmword ptr [rbp + 0x250], xmm0
1405ba95f: 48 89 95 60 02 00 00        	mov	qword ptr [rbp + 0x260], rdx
1405ba966: 4c 89 ad 38 02 00 00        	mov	qword ptr [rbp + 0x238], r13
1405ba96d: 48 89 85 40 02 00 00        	mov	qword ptr [rbp + 0x240], rax
1405ba974: 48 89 bd 48 02 00 00        	mov	qword ptr [rbp + 0x248], rdi
1405ba97b: c6 85 8f 03 00 00 01        	mov	byte ptr [rbp + 0x38f], 0x1
1405ba982: 48 8b bd 68 03 00 00        	mov	rdi, qword ptr [rbp + 0x368]
1405ba989: 48 89 f9                    	mov	rcx, rdi
1405ba98c: e8 7f 74 6c 00              	call	0x140c81e10 <.text+0xc80e10>
1405ba991: 31 c9                       	xor	ecx, ecx
1405ba993: b0 01                       	mov	al, 0x1
1405ba995: f0                          	lock
1405ba996: 0f b0 0f                    	cmpxchg	byte ptr [rdi], cl
1405ba999: 0f 85 1f 04 00 00           	jne	0x1405badbe <.text+0x5b9dbe>
1405ba99f: c6 85 8f 03 00 00 01        	mov	byte ptr [rbp + 0x38f], 0x1
1405ba9a6: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
1405ba9ad: e8 fe 62 11 00              	call	0x1406d0cb0 <.text+0x6cfcb0>
1405ba9b2: 49 89 c5                    	mov	r13, rax
1405ba9b5: c6 85 8f 03 00 00 01        	mov	byte ptr [rbp + 0x38f], 0x1
1405ba9bc: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
1405ba9c3: e8 e8 be 0c 00              	call	0x1406868b0 <.text+0x6858b0>
1405ba9c8: 48 8b 8b 40 06 00 00        	mov	rcx, qword ptr [rbx + 0x640]
1405ba9cf: 4c 89 e2                    	mov	rdx, r12
1405ba9d2: 4d 89 e8                    	mov	r8, r13
1405ba9d5: 49 89 c1                    	mov	r9, rax
1405ba9d8: ff 93 e0 00 00 00           	call	qword ptr [rbx + 0xe0]
1405ba9de: 85 c0                       	test	eax, eax
1405ba9e0: 48 8b 95 00 04 00 00        	mov	rdx, qword ptr [rbp + 0x400]
1405ba9e7: 0f 85 18 04 00 00           	jne	0x1405bae05 <.text+0x5b9e05>
1405ba9ed: 48 8d 43 78                 	lea	rax, [rbx + 0x78]
1405ba9f1: 4c 8b ab 48 06 00 00        	mov	r13, qword ptr [rbx + 0x648]
1405ba9f8: 48 8b 8d 60 02 00 00        	mov	rcx, qword ptr [rbp + 0x260]
1405ba9ff: 48 89 8d d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], rcx
1405baa06: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
1405baa0d: 0f 10 8d 30 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x230]
1405baa14: 0f 10 95 40 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x240]
1405baa1b: 0f 10 9d 50 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x250]
1405baa22: 0f 29 9d c0 01 00 00        	movaps	xmmword ptr [rbp + 0x1c0], xmm3
1405baa29: 0f 29 95 b0 01 00 00        	movaps	xmmword ptr [rbp + 0x1b0], xmm2
1405baa30: 0f 29 8d a0 01 00 00        	movaps	xmmword ptr [rbp + 0x1a0], xmm1
1405baa37: 0f 29 85 90 01 00 00        	movaps	xmmword ptr [rbp + 0x190], xmm0
1405baa3e: 0f 10 02                    	movups	xmm0, xmmword ptr [rdx]
1405baa41: 0f 10 4a 10                 	movups	xmm1, xmmword ptr [rdx + 0x10]
1405baa45: 0f 29 8d 10 02 00 00        	movaps	xmmword ptr [rbp + 0x210], xmm1
1405baa4c: 0f 29 85 00 02 00 00        	movaps	xmmword ptr [rbp + 0x200], xmm0
1405baa53: c6 85 8f 03 00 00 00        	mov	byte ptr [rbp + 0x38f], 0x0
1405baa5a: 48 8d 8d 00 02 00 00        	lea	rcx, [rbp + 0x200]
1405baa61: 48 89 4c 24 30              	mov	qword ptr [rsp + 0x30], rcx
1405baa66: 48 8d 8d 90 01 00 00        	lea	rcx, [rbp + 0x190]
1405baa6d: 48 89 4c 24 28              	mov	qword ptr [rsp + 0x28], rcx
1405baa72: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405baa77: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1405baa7e: 48 8b 95 78 03 00 00        	mov	rdx, qword ptr [rbp + 0x378]
1405baa85: 4d 89 f0                    	mov	r8, r14
1405baa88: 4d 89 e1                    	mov	r9, r12
1405baa8b: e8 40 18 62 00              	call	0x140bdc2d0 <.text+0xbdb2d0>
1405baa90: 0f b6 bd 2c 03 00 00        	movzx	edi, byte ptr [rbp + 0x32c]
1405baa97: 40 80 ff 03                 	cmp	dil, 0x3
1405baa9b: 75 5e                       	jne	0x1405baafb <.text+0x5b9afb>
1405baa9d: 0f 10 85 b0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2b0]
1405baaa4: 0f 29 85 10 01 00 00        	movaps	xmmword ptr [rbp + 0x110], xmm0
1405baaab: 0f 10 8d 70 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x270]
1405baab2: 0f 10 95 80 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x280]
1405baab9: 0f 10 9d 90 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x290]
1405baac0: 0f 10 a5 a0 02 00 00        	movups	xmm4, xmmword ptr [rbp + 0x2a0]
1405baac7: 0f 29 a5 00 01 00 00        	movaps	xmmword ptr [rbp + 0x100], xmm4
1405baace: 0f 29 9d f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm3
1405baad5: 0f 29 95 e0 00 00 00        	movaps	xmmword ptr [rbp + 0xe0], xmm2
1405baadc: 0f 29 8d d0 00 00 00        	movaps	xmmword ptr [rbp + 0xd0], xmm1
1405baae3: 0f 11 46 40                 	movups	xmmword ptr [rsi + 0x40], xmm0
1405baae7: 0f 11 66 30                 	movups	xmmword ptr [rsi + 0x30], xmm4
1405baaeb: 0f 11 5e 20                 	movups	xmmword ptr [rsi + 0x20], xmm3
1405baaef: 0f 11 56 10                 	movups	xmmword ptr [rsi + 0x10], xmm2
1405baaf3: 0f 11 0e                    	movups	xmmword ptr [rsi], xmm1
1405baaf6: e9 39 02 00 00              	jmp	0x1405bad34 <.text+0x5b9d34>
1405baafb: 4c 8d b5 d0 00 00 00        	lea	r14, [rbp + 0xd0]
1405bab02: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
1405bab09: 41 b8 bc 00 00 00           	mov	r8d, 0xbc
1405bab0f: 4c 89 f1                    	mov	rcx, r14
1405bab12: e8 e9 90 36 02              	call	0x142923c00 <.text+0x2922c00>
1405bab17: 0f b7 85 2d 03 00 00        	movzx	eax, word ptr [rbp + 0x32d]
1405bab1e: 66 89 45 75                 	mov	word ptr [rbp + 0x75], ax
1405bab22: 0f b6 85 2f 03 00 00        	movzx	eax, byte ptr [rbp + 0x32f]
1405bab29: 88 45 77                    	mov	byte ptr [rbp + 0x77], al
1405bab2c: 4c 8d 7d b8                 	lea	r15, [rbp - 0x48]
1405bab30: 41 b8 bc 00 00 00           	mov	r8d, 0xbc
1405bab36: 4c 89 f9                    	mov	rcx, r15
1405bab39: 4c 89 f2                    	mov	rdx, r14
1405bab3c: e8 bf 90 36 02              	call	0x142923c00 <.text+0x2922c00>
1405bab41: 40 88 7d 74                 	mov	byte ptr [rbp + 0x74], dil
1405bab45: c6 85 8f 03 00 00 00        	mov	byte ptr [rbp + 0x38f], 0x0
1405bab4c: 4c 89 e9                    	mov	rcx, r13
1405bab4f: 4c 89 fa                    	mov	rdx, r15
1405bab52: e8 f9 11 62 00              	call	0x140bdbd50 <.text+0xbdad50>
1405bab57: 8b 48 0c                    	mov	ecx, dword ptr [rax + 0xc]
1405bab5a: 41 0f b7 95 50 0a 00 00     	movzx	edx, word ptr [r13 + 0xa50]
1405bab62: 4d 8b 45 00                 	mov	r8, qword ptr [r13]
1405bab66: 4c 89 85 38 03 00 00        	mov	qword ptr [rbp + 0x338], r8
1405bab6d: 49 89 c0                    	mov	r8, rax
1405bab70: 49 83 c0 10                 	add	r8, 0x10
1405bab74: 48 89 85 40 03 00 00        	mov	qword ptr [rbp + 0x340], rax
1405bab7b: 89 8d 48 03 00 00           	mov	dword ptr [rbp + 0x348], ecx
1405bab81: 66 89 95 4c 03 00 00        	mov	word ptr [rbp + 0x34c], dx
1405bab88: c6 85 4e 03 00 00 00        	mov	byte ptr [rbp + 0x34e], 0x0
1405bab8f: 4c 89 85 30 03 00 00        	mov	qword ptr [rbp + 0x330], r8
1405bab96: 48 83 bd 68 02 00 00 00     	cmp	qword ptr [rbp + 0x268], 0x0
1405bab9e: 48 8b bd 00 04 00 00        	mov	rdi, qword ptr [rbp + 0x400]
1405baba5: 74 4f                       	je	0x1405babf6 <.text+0x5b9bf6>
1405baba7: 0f 10 07                    	movups	xmm0, xmmword ptr [rdi]
1405babaa: 0f 10 4f 10                 	movups	xmm1, xmmword ptr [rdi + 0x10]
1405babae: 0f 29 8d e0 00 00 00        	movaps	xmmword ptr [rbp + 0xe0], xmm1
1405babb5: 0f 29 85 d0 00 00 00        	movaps	xmmword ptr [rbp + 0xd0], xmm0
1405babbc: 48 8d 85 30 03 00 00        	lea	rax, [rbp + 0x330]
1405babc3: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405babc8: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1405babcf: 4c 8d 85 d0 00 00 00        	lea	r8, [rbp + 0xd0]
1405babd6: 4c 8d 8d e0 01 00 00        	lea	r9, [rbp + 0x1e0]
1405babdd: 48 89 da                    	mov	rdx, rbx
1405babe0: e8 8b 36 64 00              	call	0x140bfe270 <.text+0xbfd270>
1405babe5: 48 8b 85 70 02 00 00        	mov	rax, qword ptr [rbp + 0x270]
1405babec: 48 83 f8 0b                 	cmp	rax, 0xb
1405babf0: 0f 85 f1 00 00 00           	jne	0x1405bace7 <.text+0x5b9ce7>
1405babf6: 48 8b 85 48 03 00 00        	mov	rax, qword ptr [rbp + 0x348]
1405babfd: 48 89 85 80 02 00 00        	mov	qword ptr [rbp + 0x280], rax
1405bac04: 0f 10 85 38 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x338]
1405bac0b: 0f 29 85 70 02 00 00        	movaps	xmmword ptr [rbp + 0x270], xmm0
1405bac12: 0f 10 0f                    	movups	xmm1, xmmword ptr [rdi]
1405bac15: 0f 10 57 10                 	movups	xmm2, xmmword ptr [rdi + 0x10]
1405bac19: 0f 11 95 98 02 00 00        	movups	xmmword ptr [rbp + 0x298], xmm2
1405bac20: 0f 11 8d 88 02 00 00        	movups	xmmword ptr [rbp + 0x288], xmm1
1405bac27: 0f 11 46 08                 	movups	xmmword ptr [rsi + 0x8], xmm0
1405bac2b: 48 8b 85 a0 02 00 00        	mov	rax, qword ptr [rbp + 0x2a0]
1405bac32: 48 89 46 38                 	mov	qword ptr [rsi + 0x38], rax
1405bac36: 0f 28 85 90 02 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x290]
1405bac3d: 0f 11 46 28                 	movups	xmmword ptr [rsi + 0x28], xmm0
1405bac41: 48 8b 85 80 02 00 00        	mov	rax, qword ptr [rbp + 0x280]
1405bac48: 48 89 46 18                 	mov	qword ptr [rsi + 0x18], rax
1405bac4c: 48 8b 85 88 02 00 00        	mov	rax, qword ptr [rbp + 0x288]
1405bac53: 48 89 46 20                 	mov	qword ptr [rsi + 0x20], rax
1405bac57: 48 c7 06 0b 00 00 00        	mov	qword ptr [rsi], 0xb
1405bac5e: e9 d1 00 00 00              	jmp	0x1405bad34 <.text+0x5b9d34>
1405bac63: 48 89 ca                    	mov	rdx, rcx
1405bac66: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
1405bac6d: 48 89 8d 10 02 00 00        	mov	qword ptr [rbp + 0x210], rcx
1405bac74: 0f 10 85 50 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x350]
1405bac7b: 0f 29 85 00 02 00 00        	movaps	xmmword ptr [rbp + 0x200], xmm0
1405bac82: 48 8b 8d 70 03 00 00        	mov	rcx, qword ptr [rbp + 0x370]
1405bac89: 48 89 4e 08                 	mov	qword ptr [rsi + 0x8], rcx
1405bac8d: 4c 89 7e 10                 	mov	qword ptr [rsi + 0x10], r15
1405bac91: 4c 89 6e 18                 	mov	qword ptr [rsi + 0x18], r13
1405bac95: 0f 10 85 50 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x350]
1405bac9c: 0f 11 46 38                 	movups	xmmword ptr [rsi + 0x38], xmm0
1405baca0: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
1405baca7: 48 89 4e 48                 	mov	qword ptr [rsi + 0x48], rcx
1405bacab: 48 89 16                    	mov	qword ptr [rsi], rdx
1405bacae: 48 c7 46 20 0f 00 00 00     	mov	qword ptr [rsi + 0x20], 0xf
1405bacb6: 48 89 46 28                 	mov	qword ptr [rsi + 0x28], rax
1405bacba: 48 c7 46 30 0f 00 00 00     	mov	qword ptr [rsi + 0x30], 0xf
1405bacc2: 48 8b bd 68 03 00 00        	mov	rdi, qword ptr [rbp + 0x368]
1405bacc9: 48 89 f9                    	mov	rcx, rdi
1405baccc: e8 3f 71 6c 00              	call	0x140c81e10 <.text+0xc80e10>
1405bacd1: 31 c9                       	xor	ecx, ecx
1405bacd3: b0 01                       	mov	al, 0x1
1405bacd5: f0                          	lock
1405bacd6: 0f b0 0f                    	cmpxchg	byte ptr [rdi], cl
1405bacd9: 74 59                       	je	0x1405bad34 <.text+0x5b9d34>
1405bacdb: 48 89 f9                    	mov	rcx, rdi
1405bacde: 31 d2                       	xor	edx, edx
1405bace0: e8 ab 2e 31 02              	call	0x1428cdb90 <.text+0x28ccb90>
1405bace5: eb 4d                       	jmp	0x1405bad34 <.text+0x5b9d34>
1405bace7: 48 8b 8d b8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2b8]
1405bacee: 48 89 4e 48                 	mov	qword ptr [rsi + 0x48], rcx
1405bacf2: 0f 10 85 78 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x278]
1405bacf9: 0f 10 8d 88 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x288]
1405bad00: 0f 10 95 98 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x298]
1405bad07: 0f 10 9d a8 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x2a8]
1405bad0e: 0f 11 5e 38                 	movups	xmmword ptr [rsi + 0x38], xmm3
1405bad12: 0f 11 56 28                 	movups	xmmword ptr [rsi + 0x28], xmm2
1405bad16: 0f 11 4e 18                 	movups	xmmword ptr [rsi + 0x18], xmm1
1405bad1a: 0f 11 46 08                 	movups	xmmword ptr [rsi + 0x8], xmm0
1405bad1e: 48 89 06                    	mov	qword ptr [rsi], rax
1405bad21: c6 85 8f 03 00 00 00        	mov	byte ptr [rbp + 0x38f], 0x0
1405bad28: 48 8d 8d 38 03 00 00        	lea	rcx, [rbp + 0x338]
1405bad2f: e8 7c 77 62 00              	call	0x140be24b0 <.text+0xbe14b0>
1405bad34: 48 89 f0                    	mov	rax, rsi
1405bad37: 48 81 c4 18 04 00 00        	add	rsp, 0x418
1405bad3e: 5b                          	pop	rbx
1405bad3f: 5f                          	pop	rdi
1405bad40: 5e                          	pop	rsi
1405bad41: 41 5c                       	pop	r12
1405bad43: 41 5d                       	pop	r13
1405bad45: 41 5e                       	pop	r14
1405bad47: 41 5f                       	pop	r15
1405bad49: 5d                          	pop	rbp
1405bad4a: c3                          	ret
1405bad4b: 48 8d 0d 70 9b 44 02        	lea	rcx, [rip + 0x2449b70]  # 0x142a048c2
1405bad52: 4c 8d 05 a7 9b 44 02        	lea	r8, [rip + 0x2449ba7]   # 0x142a04900
1405bad59: ba 3c 00 00 00              	mov	edx, 0x3c
1405bad5e: e8 cd e6 31 02              	call	0x1428d9430 <.text+0x28d8430>
1405bad63: 48 8d 0d ae 9b 44 02        	lea	rcx, [rip + 0x2449bae]  # 0x142a04918
1405bad6a: 4c 8d 05 d7 9b 44 02        	lea	r8, [rip + 0x2449bd7]   # 0x142a04948
1405bad71: ba 2e 00 00 00              	mov	edx, 0x2e
1405bad76: e8 b5 e6 31 02              	call	0x1428d9430 <.text+0x28d8430>
1405bad7b: 48 8d 0d de 9b 44 02        	lea	rcx, [rip + 0x2449bde]  # 0x142a04960
1405bad82: 4c 8d 05 07 9c 44 02        	lea	r8, [rip + 0x2449c07]   # 0x142a04990
1405bad89: ba 2f 00 00 00              	mov	edx, 0x2f
1405bad8e: e8 9d e6 31 02              	call	0x1428d9430 <.text+0x28d8430>
1405bad93: 48 8d 0d 0e 9c 44 02        	lea	rcx, [rip + 0x2449c0e]  # 0x142a049a8
1405bad9a: 4c 8d 05 37 9c 44 02        	lea	r8, [rip + 0x2449c37]   # 0x142a049d8
1405bada1: ba 2e 00 00 00              	mov	edx, 0x2e
1405bada6: e8 85 e6 31 02              	call	0x1428d9430 <.text+0x28d8430>
1405badab: 4c 89 f1                    	mov	rcx, r14
1405badae: 41 b8 00 ca 9a 3b           	mov	r8d, 0x3b9aca00
1405badb4: e8 57 24 31 02              	call	0x1428cd210 <.text+0x28cc210>
1405badb9: e9 f6 f9 ff ff              	jmp	0x1405ba7b4 <.text+0x5b97b4>
1405badbe: c6 85 8f 03 00 00 01        	mov	byte ptr [rbp + 0x38f], 0x1
1405badc5: 48 89 f9                    	mov	rcx, rdi
1405badc8: 31 d2                       	xor	edx, edx
1405badca: e8 c1 2d 31 02              	call	0x1428cdb90 <.text+0x28ccb90>
1405badcf: e9 cb fb ff ff              	jmp	0x1405ba99f <.text+0x5b999f>
1405badd4: 89 85 70 02 00 00           	mov	dword ptr [rbp + 0x270], eax
1405badda: 48 8d 05 1f 9c 44 02        	lea	rax, [rip + 0x2449c1f]  # 0x142a04a00
1405bade1: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405bade6: 48 8d 0d 03 9c 44 02        	lea	rcx, [rip + 0x2449c03]  # 0x142a049f0
1405baded: 4c 8d 0d f4 73 44 02        	lea	r9, [rip + 0x24473f4]   # 0x142a021e8
1405badf4: 4c 8d 85 70 02 00 00        	lea	r8, [rbp + 0x270]
1405badfb: ba 0c 00 00 00              	mov	edx, 0xc
1405bae00: e8 5b e9 31 02              	call	0x1428d9760 <.text+0x28d8760>
1405bae05: 89 85 70 02 00 00           	mov	dword ptr [rbp + 0x270], eax
1405bae0b: c6 85 8f 03 00 00 01        	mov	byte ptr [rbp + 0x38f], 0x1
1405bae12: 48 8d 05 67 9c 44 02        	lea	rax, [rip + 0x2449c67]  # 0x142a04a80
1405bae19: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405bae1e: 48 8d 0d 43 9c 44 02        	lea	rcx, [rip + 0x2449c43]  # 0x142a04a68
1405bae25: 4c 8d 0d bc 73 44 02        	lea	r9, [rip + 0x24473bc]   # 0x142a021e8
1405bae2c: 4c 8d 85 70 02 00 00        	lea	r8, [rbp + 0x270]
1405bae33: ba 11 00 00 00              	mov	edx, 0x11
1405bae38: e8 23 e9 31 02              	call	0x1428d9760 <.text+0x28d8760>
1405bae3d: e9 a7 00 00 00              	jmp	0x1405baee9 <.text+0x5b9ee9>
1405bae42: 0f 10 85 d0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xd0]
1405bae49: 0f 10 8d e0 00 00 00        	movups	xmm1, xmmword ptr [rbp + 0xe0]
1405bae50: 0f 29 8d 80 02 00 00        	movaps	xmmword ptr [rbp + 0x280], xmm1
1405bae57: 0f 29 85 70 02 00 00        	movaps	xmmword ptr [rbp + 0x270], xmm0
1405bae5e: 48 8d 05 b3 9b 44 02        	lea	rax, [rip + 0x2449bb3]  # 0x142a04a18
1405bae65: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405bae6a: 48 8d 0d b7 73 44 02        	lea	rcx, [rip + 0x24473b7]  # 0x142a02228
1405bae71: 4c 8d 0d 90 73 44 02        	lea	r9, [rip + 0x2447390]   # 0x142a02208
1405bae78: 4c 8d 85 70 02 00 00        	lea	r8, [rbp + 0x270]
1405bae7f: ba 2b 00 00 00              	mov	edx, 0x2b
1405bae84: e8 d7 e8 31 02              	call	0x1428d9760 <.text+0x28d8760>
1405bae89: eb 5e                       	jmp	0x1405baee9 <.text+0x5b9ee9>
1405bae8b: 4c 89 a5 80 03 00 00        	mov	qword ptr [rbp + 0x380], r12
1405bae92: 4c 89 bd 78 03 00 00        	mov	qword ptr [rbp + 0x378], r15
1405bae99: 89 85 d0 00 00 00           	mov	dword ptr [rbp + 0xd0], eax
1405bae9f: 48 8d 05 aa 9b 44 02        	lea	rax, [rip + 0x2449baa]  # 0x142a04a50
1405baea6: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405baeab: 48 8d 0d 7e 9b 44 02        	lea	rcx, [rip + 0x2449b7e]  # 0x142a04a30
1405baeb2: 4c 8d 0d 2f 73 44 02        	lea	r9, [rip + 0x244732f]   # 0x142a021e8
1405baeb9: 4c 8d 85 d0 00 00 00        	lea	r8, [rbp + 0xd0]
1405baec0: ba 1b 00 00 00              	mov	edx, 0x1b
1405baec5: e8 96 e8 31 02              	call	0x1428d9760 <.text+0x28d8760>
1405baeca: eb 1d                       	jmp	0x1405baee9 <.text+0x5b9ee9>
1405baecc: 4c 89 bd 78 03 00 00        	mov	qword ptr [rbp + 0x378], r15
1405baed3: 4c 8d 05 ee 73 44 02        	lea	r8, [rip + 0x24473ee]   # 0x142a022c8
1405baeda: b9 01 00 00 00              	mov	ecx, 0x1
1405baedf: ba 0f 00 00 00              	mov	edx, 0xf
1405baee4: e8 fa e0 31 02              	call	0x1428d8fe3 <.text+0x28d7fe3>
1405baee9: 0f 0b                       	ud2
1405baeeb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1405baef0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405baef5: 55                          	push	rbp
1405baef6: 41 57                       	push	r15
1405baef8: 41 56                       	push	r14
1405baefa: 41 55                       	push	r13
1405baefc: 41 54                       	push	r12
1405baefe: 56                          	push	rsi
1405baeff: 57                          	push	rdi
1405baf00: 53                          	push	rbx
1405baf01: 48 83 ec 38                 	sub	rsp, 0x38
1405baf05: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405baf0c: 48 8b 95 70 02 00 00        	mov	rdx, qword ptr [rbp + 0x270]
1405baf13: 48 85 d2                    	test	rdx, rdx
1405baf16: 74 12                       	je	0x1405baf2a <.text+0x5b9f2a>
1405baf18: 48 8b 8d 78 02 00 00        	mov	rcx, qword ptr [rbp + 0x278]
1405baf1f: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405baf25: e8 f6 8c 05 00              	call	0x140613c20 <.text+0x612c20>
1405baf2a: 90                          	nop
1405baf2b: 48 83 c4 38                 	add	rsp, 0x38
1405baf2f: 5b                          	pop	rbx
1405baf30: 5f                          	pop	rdi
1405baf31: 5e                          	pop	rsi
1405baf32: 41 5c                       	pop	r12
1405baf34: 41 5d                       	pop	r13
1405baf36: 41 5e                       	pop	r14
1405baf38: 41 5f                       	pop	r15
1405baf3a: 5d                          	pop	rbp
1405baf3b: c3                          	ret
1405baf3c: 0f 1f 40 00                 	nop	dword ptr [rax]
1405baf40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405baf45: 55                          	push	rbp
1405baf46: 41 57                       	push	r15
1405baf48: 41 56                       	push	r14
1405baf4a: 41 55                       	push	r13
1405baf4c: 41 54                       	push	r12
1405baf4e: 56                          	push	rsi
1405baf4f: 57                          	push	rdi
1405baf50: 53                          	push	rbx
1405baf51: 48 83 ec 38                 	sub	rsp, 0x38
1405baf55: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405baf5c: 48 8b 85 80 03 00 00        	mov	rax, qword ptr [rbp + 0x380]
1405baf63: c6 00 00                    	mov	byte ptr [rax], 0x0
1405baf66: 48 83 bd 78 03 00 00 00     	cmp	qword ptr [rbp + 0x378], 0x0
1405baf6e: 74 19                       	je	0x1405baf89 <.text+0x5b9f89>
1405baf70: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405baf76: 48 8b 8d 80 03 00 00        	mov	rcx, qword ptr [rbp + 0x380]
1405baf7d: 48 8b 95 78 03 00 00        	mov	rdx, qword ptr [rbp + 0x378]
1405baf84: e8 97 8c 05 00              	call	0x140613c20 <.text+0x612c20>
1405baf89: 90                          	nop
1405baf8a: 48 83 c4 38                 	add	rsp, 0x38
1405baf8e: 5b                          	pop	rbx
1405baf8f: 5f                          	pop	rdi
1405baf90: 5e                          	pop	rsi
1405baf91: 41 5c                       	pop	r12
1405baf93: 41 5d                       	pop	r13
1405baf95: 41 5e                       	pop	r14
1405baf97: 41 5f                       	pop	r15
1405baf99: 5d                          	pop	rbp
1405baf9a: c3                          	ret
1405baf9b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1405bafa0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405bafa5: 55                          	push	rbp
1405bafa6: 41 57                       	push	r15
1405bafa8: 41 56                       	push	r14
1405bafaa: 41 55                       	push	r13
1405bafac: 41 54                       	push	r12
1405bafae: 56                          	push	rsi
1405bafaf: 57                          	push	rdi
1405bafb0: 53                          	push	rbx
1405bafb1: 48 83 ec 38                 	sub	rsp, 0x38
1405bafb5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405bafbc: 48 83 bd 80 03 00 00 05     	cmp	qword ptr [rbp + 0x380], 0x5
1405bafc4: 77 35                       	ja	0x1405baffb <.text+0x5b9ffb>
1405bafc6: b8 32 00 00 00              	mov	eax, 0x32
1405bafcb: 48 8b 8d 80 03 00 00        	mov	rcx, qword ptr [rbp + 0x380]
1405bafd2: 48 0f a3 c8                 	bt	rax, rcx
1405bafd6: 73 23                       	jae	0x1405baffb <.text+0x5b9ffb>
1405bafd8: 48 83 bd 70 03 00 00 00     	cmp	qword ptr [rbp + 0x370], 0x0
1405bafe0: 74 19                       	je	0x1405baffb <.text+0x5b9ffb>
1405bafe2: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405bafe8: 48 8b 8d 78 03 00 00        	mov	rcx, qword ptr [rbp + 0x378]
1405bafef: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1405baff6: e8 25 8c 05 00              	call	0x140613c20 <.text+0x612c20>
1405baffb: 90                          	nop
1405baffc: 48 83 c4 38                 	add	rsp, 0x38
1405bb000: 5b                          	pop	rbx
1405bb001: 5f                          	pop	rdi
1405bb002: 5e                          	pop	rsi
1405bb003: 41 5c                       	pop	r12
1405bb005: 41 5d                       	pop	r13
1405bb007: 41 5e                       	pop	r14
1405bb009: 41 5f                       	pop	r15
1405bb00b: 5d                          	pop	rbp
1405bb00c: c3                          	ret
1405bb00d: 0f 1f 00                    	nop	dword ptr [rax]
1405bb010: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405bb015: 55                          	push	rbp
1405bb016: 41 57                       	push	r15
1405bb018: 41 56                       	push	r14
1405bb01a: 41 55                       	push	r13
1405bb01c: 41 54                       	push	r12
1405bb01e: 56                          	push	rsi
1405bb01f: 57                          	push	rdi
1405bb020: 53                          	push	rbx
1405bb021: 48 83 ec 38                 	sub	rsp, 0x38
1405bb025: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405bb02c: 48 8d 8d 38 03 00 00        	lea	rcx, [rbp + 0x338]
1405bb033: e8 78 74 62 00              	call	0x140be24b0 <.text+0xbe14b0>
1405bb038: c6 85 8f 03 00 00 00        	mov	byte ptr [rbp + 0x38f], 0x0
1405bb03f: 48 83 c4 38                 	add	rsp, 0x38
1405bb043: 5b                          	pop	rbx
1405bb044: 5f                          	pop	rdi
1405bb045: 5e                          	pop	rsi
1405bb046: 41 5c                       	pop	r12
1405bb048: 41 5d                       	pop	r13
1405bb04a: 41 5e                       	pop	r14
1405bb04c: 41 5f                       	pop	r15
1405bb04e: 5d                          	pop	rbp
1405bb04f: c3                          	ret
1405bb050: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405bb055: 55                          	push	rbp
1405bb056: 41 57                       	push	r15
1405bb058: 41 56                       	push	r14
1405bb05a: 41 55                       	push	r13
1405bb05c: 41 54                       	push	r12
1405bb05e: 56                          	push	rsi
1405bb05f: 57                          	push	rdi
1405bb060: 53                          	push	rbx
1405bb061: 48 83 ec 38                 	sub	rsp, 0x38
1405bb065: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405bb06c: 80 bd 8f 03 00 00 00        	cmp	byte ptr [rbp + 0x38f], 0x0
1405bb073: 75 11                       	jne	0x1405bb086 <.text+0x5ba086>
1405bb075: 48 83 c4 38                 	add	rsp, 0x38
1405bb079: 5b                          	pop	rbx
1405bb07a: 5f                          	pop	rdi
1405bb07b: 5e                          	pop	rsi
1405bb07c: 41 5c                       	pop	r12
1405bb07e: 41 5d                       	pop	r13
1405bb080: 41 5e                       	pop	r14
1405bb082: 41 5f                       	pop	r15
1405bb084: 5d                          	pop	rbp
1405bb085: c3                          	ret
1405bb086: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
1405bb08d: e8 6e 45 03 00              	call	0x1405ef600 <.text+0x5ee600>
1405bb092: eb e1                       	jmp	0x1405bb075 <.text+0x5ba075>
1405bb094: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1405bb0a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405bb0a5: 55                          	push	rbp
1405bb0a6: 41 57                       	push	r15
1405bb0a8: 41 56                       	push	r14
1405bb0aa: 41 55                       	push	r13
1405bb0ac: 41 54                       	push	r12
1405bb0ae: 56                          	push	rsi
1405bb0af: 57                          	push	rdi
1405bb0b0: 53                          	push	rbx
1405bb0b1: 48 83 ec 38                 	sub	rsp, 0x38
1405bb0b5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405bb0bc: 48 8b 8d 68 03 00 00        	mov	rcx, qword ptr [rbp + 0x368]
1405bb0c3: e8 68 6b ba ff              	call	0x140161c30 <.text+0x160c30>
1405bb0c8: 90                          	nop
1405bb0c9: 48 83 c4 38                 	add	rsp, 0x38
1405bb0cd: 5b                          	pop	rbx
1405bb0ce: 5f                          	pop	rdi
1405bb0cf: 5e                          	pop	rsi
1405bb0d0: 41 5c                       	pop	r12
1405bb0d2: 41 5d                       	pop	r13
1405bb0d4: 41 5e                       	pop	r14
1405bb0d6: 41 5f                       	pop	r15
1405bb0d8: 5d                          	pop	rbp
1405bb0d9: c3                          	ret

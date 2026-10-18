
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14059d450: 55                          	push	rbp
14059d451: 41 57                       	push	r15
14059d453: 41 56                       	push	r14
14059d455: 41 55                       	push	r13
14059d457: 41 54                       	push	r12
14059d459: 56                          	push	rsi
14059d45a: 57                          	push	rdi
14059d45b: 53                          	push	rbx
14059d45c: b8 b8 37 00 00              	mov	eax, 0x37b8
14059d461: e8 da d5 2f 02              	call	0x14289aa40 <.text+0x2899a40>
14059d466: 48 29 c4                    	sub	rsp, rax
14059d469: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14059d471: 44 0f 29 bd 20 37 00 00     	movaps	xmmword ptr [rbp + 0x3720], xmm15
14059d479: 44 0f 29 b5 10 37 00 00     	movaps	xmmword ptr [rbp + 0x3710], xmm14
14059d481: 44 0f 29 ad 00 37 00 00     	movaps	xmmword ptr [rbp + 0x3700], xmm13
14059d489: 44 0f 29 a5 f0 36 00 00     	movaps	xmmword ptr [rbp + 0x36f0], xmm12
14059d491: 44 0f 29 9d e0 36 00 00     	movaps	xmmword ptr [rbp + 0x36e0], xmm11
14059d499: 44 0f 29 95 d0 36 00 00     	movaps	xmmword ptr [rbp + 0x36d0], xmm10
14059d4a1: 44 0f 29 8d c0 36 00 00     	movaps	xmmword ptr [rbp + 0x36c0], xmm9
14059d4a9: 44 0f 29 85 b0 36 00 00     	movaps	xmmword ptr [rbp + 0x36b0], xmm8
14059d4b1: 0f 29 bd a0 36 00 00        	movaps	xmmword ptr [rbp + 0x36a0], xmm7
14059d4b8: 0f 29 b5 90 36 00 00        	movaps	xmmword ptr [rbp + 0x3690], xmm6
14059d4bf: 48 83 e4 80                 	and	rsp, -0x80
14059d4c3: 48 89 e3                    	mov	rbx, rsp
14059d4c6: 48 89 ab 00 37 00 00        	mov	qword ptr [rbx + 0x3700], rbp
14059d4cd: 48 c7 85 88 36 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x3688], -0x2
14059d4d8: 4c 89 8b c8 00 00 00        	mov	qword ptr [rbx + 0xc8], r9
14059d4df: 48 8b 85 a0 37 00 00        	mov	rax, qword ptr [rbp + 0x37a0]
14059d4e6: 0f 10 85 d8 37 00 00        	movups	xmm0, xmmword ptr [rbp + 0x37d8]
14059d4ed: 0f 10 8d c8 37 00 00        	movups	xmm1, xmmword ptr [rbp + 0x37c8]
14059d4f4: 48 89 8b c0 02 00 00        	mov	qword ptr [rbx + 0x2c0], rcx
14059d4fb: 48 89 93 c8 02 00 00        	mov	qword ptr [rbx + 0x2c8], rdx
14059d502: 4c 89 83 40 01 00 00        	mov	qword ptr [rbx + 0x140], r8
14059d509: 0f 11 8b 28 03 00 00        	movups	xmmword ptr [rbx + 0x328], xmm1
14059d510: 0f 11 83 60 01 00 00        	movups	xmmword ptr [rbx + 0x160], xmm0
14059d517: 66 0f 10 80 e0 0a 00 00     	movupd	xmm0, xmmword ptr [rax + 0xae0]
14059d51f: 0f 10 88 f0 0a 00 00        	movups	xmm1, xmmword ptr [rax + 0xaf0]
14059d526: 0f 29 8b 60 03 00 00        	movaps	xmmword ptr [rbx + 0x360], xmm1
14059d52d: 66 0f 29 83 50 03 00 00     	movapd	xmmword ptr [rbx + 0x350], xmm0
14059d535: 48 8b 88 00 0b 00 00        	mov	rcx, qword ptr [rax + 0xb00]
14059d53c: 48 89 8b b8 02 00 00        	mov	qword ptr [rbx + 0x2b8], rcx
14059d543: 48 8b 88 08 0b 00 00        	mov	rcx, qword ptr [rax + 0xb08]
14059d54a: 48 89 8b b0 02 00 00        	mov	qword ptr [rbx + 0x2b0], rcx
14059d551: 48 c7 83 00 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb00], 0x1
14059d55c: 48 c7 83 08 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x1
14059d567: 48 8d 8b 10 0b 00 00        	lea	rcx, [rbx + 0xb10]
14059d56e: 41 b8 e0 0a 00 00           	mov	r8d, 0xae0
14059d574: 48 89 c2                    	mov	rdx, rax
14059d577: e8 84 66 38 02              	call	0x142923c00 <.text+0x2922c00>
14059d57c: 0f b6 05 86 76 09 03        	movzx	eax, byte ptr [rip + 0x3097686] # 0x143634c09
14059d583: b9 f0 0a 00 00              	mov	ecx, 0xaf0
14059d588: ba 08 00 00 00              	mov	edx, 0x8
14059d58d: e8 7e 66 07 00              	call	0x140613c10 <.text+0x612c10>
14059d592: 48 85 c0                    	test	rax, rax
14059d595: 0f 84 8b 94 00 00           	je	0x1405a6a26 <.text+0x5a5a26>
14059d59b: 48 89 c6                    	mov	rsi, rax
14059d59e: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
14059d5a5: 41 b8 f0 0a 00 00           	mov	r8d, 0xaf0
14059d5ab: 48 89 c1                    	mov	rcx, rax
14059d5ae: e8 4d 66 38 02              	call	0x142923c00 <.text+0x2922c00>
14059d5b3: 48 89 b3 38 01 00 00        	mov	qword ptr [rbx + 0x138], rsi
14059d5ba: 80 be e8 0a 00 00 01        	cmp	byte ptr [rsi + 0xae8], 0x1
14059d5c1: 75 79                       	jne	0x14059d63c <.text+0x59c63c>
14059d5c3: 48 83 c6 10                 	add	rsi, 0x10
14059d5c7: c6 83 bc 00 00 00 01        	mov	byte ptr [rbx + 0xbc], 0x1
14059d5ce: c6 83 bd 00 00 00 01        	mov	byte ptr [rbx + 0xbd], 0x1
14059d5d5: c6 83 be 00 00 00 01        	mov	byte ptr [rbx + 0xbe], 0x1
14059d5dc: c6 83 bf 00 00 00 01        	mov	byte ptr [rbx + 0xbf], 0x1
14059d5e3: c6 83 c0 00 00 00 01        	mov	byte ptr [rbx + 0xc0], 0x1
14059d5ea: c6 83 c1 00 00 00 01        	mov	byte ptr [rbx + 0xc1], 0x1
14059d5f1: 48 89 f1                    	mov	rcx, rsi
14059d5f4: e8 57 b3 65 00              	call	0x140bf8950 <.text+0xbf7950>
14059d5f9: 48 83 c0 68                 	add	rax, 0x68
14059d5fd: c6 83 bc 00 00 00 01        	mov	byte ptr [rbx + 0xbc], 0x1
14059d604: c6 83 bd 00 00 00 01        	mov	byte ptr [rbx + 0xbd], 0x1
14059d60b: c6 83 be 00 00 00 01        	mov	byte ptr [rbx + 0xbe], 0x1
14059d612: c6 83 bf 00 00 00 01        	mov	byte ptr [rbx + 0xbf], 0x1
14059d619: c6 83 c0 00 00 00 01        	mov	byte ptr [rbx + 0xc0], 0x1
14059d620: c6 83 c1 00 00 00 01        	mov	byte ptr [rbx + 0xc1], 0x1
14059d627: 48 89 c1                    	mov	rcx, rax
14059d62a: e8 b1 2b 66 00              	call	0x140c001e0 <.text+0xbff1e0>
14059d62f: 84 c0                       	test	al, al
14059d631: 74 13                       	je	0x14059d646 <.text+0x59c646>
14059d633: 48 8b 05 8e ea 08 03        	mov	rax, qword ptr [rip + 0x308ea8e] # 0x14362c0c8
14059d63a: eb 07                       	jmp	0x14059d643 <.text+0x59c643>
14059d63c: 48 8b 05 8d ea 08 03        	mov	rax, qword ptr [rip + 0x308ea8d] # 0x14362c0d0
14059d643: c6 00 01                    	mov	byte ptr [rax], 0x1
14059d646: 48 8b 8b 38 01 00 00        	mov	rcx, qword ptr [rbx + 0x138]
14059d64d: 48 83 c1 10                 	add	rcx, 0x10
14059d651: c6 83 bc 00 00 00 01        	mov	byte ptr [rbx + 0xbc], 0x1
14059d658: c6 83 bd 00 00 00 01        	mov	byte ptr [rbx + 0xbd], 0x1
14059d65f: c6 83 be 00 00 00 01        	mov	byte ptr [rbx + 0xbe], 0x1
14059d666: c6 83 bf 00 00 00 01        	mov	byte ptr [rbx + 0xbf], 0x1
14059d66d: c6 83 c0 00 00 00 01        	mov	byte ptr [rbx + 0xc0], 0x1
14059d674: c6 83 c1 00 00 00 01        	mov	byte ptr [rbx + 0xc1], 0x1
14059d67b: e8 20 c2 fe ff              	call	0x1405898a0 <.text+0x5888a0>
14059d680: 48 8b 83 40 01 00 00        	mov	rax, qword ptr [rbx + 0x140]
14059d687: 48 8b 48 18                 	mov	rcx, qword ptr [rax + 0x18]
14059d68b: 0f 57 f6                    	xorps	xmm6, xmm6
14059d68e: 0f 29 b3 40 04 00 00        	movaps	xmmword ptr [rbx + 0x440], xmm6
14059d695: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
14059d69c: ff 15 de 51 bf 02           	call	qword ptr [rip + 0x2bf51de] # 0x143192880
14059d6a2: 85 c0                       	test	eax, eax
14059d6a4: 0f 84 76 8e 00 00           	je	0x1405a6520 <.text+0x5a5520>
14059d6aa: 8b bb 44 04 00 00           	mov	edi, dword ptr [rbx + 0x444]
14059d6b0: 8b 83 48 04 00 00           	mov	eax, dword ptr [rbx + 0x448]
14059d6b6: 2b 83 40 04 00 00           	sub	eax, dword ptr [rbx + 0x440]
14059d6bc: 44 8b b3 4c 04 00 00        	mov	r14d, dword ptr [rbx + 0x44c]
14059d6c3: 0f 57 c0                    	xorps	xmm0, xmm0
14059d6c6: f2 48 0f 2a c0              	cvtsi2sd	xmm0, rax
14059d6cb: c6 83 bc 00 00 00 01        	mov	byte ptr [rbx + 0xbc], 0x1
14059d6d2: c6 83 bd 00 00 00 01        	mov	byte ptr [rbx + 0xbd], 0x1
14059d6d9: c6 83 be 00 00 00 01        	mov	byte ptr [rbx + 0xbe], 0x1
14059d6e0: c6 83 bf 00 00 00 01        	mov	byte ptr [rbx + 0xbf], 0x1
14059d6e7: c6 83 c0 00 00 00 01        	mov	byte ptr [rbx + 0xc0], 0x1
14059d6ee: c6 83 c1 00 00 00 01        	mov	byte ptr [rbx + 0xc1], 0x1
14059d6f5: e8 f6 a0 0f 00              	call	0x1406977f0 <.text+0x6967f0>
14059d6fa: 89 c6                       	mov	esi, eax
14059d6fc: 41 29 fe                    	sub	r14d, edi
14059d6ff: 0f 57 c0                    	xorps	xmm0, xmm0
14059d702: f2 49 0f 2a c6              	cvtsi2sd	xmm0, r14
14059d707: c6 83 bc 00 00 00 01        	mov	byte ptr [rbx + 0xbc], 0x1
14059d70e: c6 83 bd 00 00 00 01        	mov	byte ptr [rbx + 0xbd], 0x1
14059d715: c6 83 be 00 00 00 01        	mov	byte ptr [rbx + 0xbe], 0x1
14059d71c: c6 83 bf 00 00 00 01        	mov	byte ptr [rbx + 0xbf], 0x1
14059d723: c6 83 c0 00 00 00 01        	mov	byte ptr [rbx + 0xc0], 0x1
14059d72a: c6 83 c1 00 00 00 01        	mov	byte ptr [rbx + 0xc1], 0x1
14059d731: e8 ba a0 0f 00              	call	0x1406977f0 <.text+0x6967f0>
14059d736: 48 c1 e0 20                 	shl	rax, 0x20
14059d73a: 89 f1                       	mov	ecx, esi
14059d73c: 48 09 c1                    	or	rcx, rax
14059d73f: c6 83 bc 00 00 00 01        	mov	byte ptr [rbx + 0xbc], 0x1
14059d746: c6 83 bd 00 00 00 01        	mov	byte ptr [rbx + 0xbd], 0x1
14059d74d: c6 83 be 00 00 00 01        	mov	byte ptr [rbx + 0xbe], 0x1
14059d754: c6 83 bf 00 00 00 01        	mov	byte ptr [rbx + 0xbf], 0x1
14059d75b: c6 83 c0 00 00 00 01        	mov	byte ptr [rbx + 0xc0], 0x1
14059d762: c6 83 c1 00 00 00 01        	mov	byte ptr [rbx + 0xc1], 0x1
14059d769: e8 82 c4 3a 00              	call	0x140949bf0 <.text+0x948bf0>
14059d76e: 89 83 48 01 00 00           	mov	dword ptr [rbx + 0x148], eax
14059d774: 89 93 4c 01 00 00           	mov	dword ptr [rbx + 0x14c], edx
14059d77a: 48 8b 8d b0 37 00 00        	mov	rcx, qword ptr [rbp + 0x37b0]
14059d781: 0f b6 81 f4 00 00 00        	movzx	eax, byte ptr [rcx + 0xf4]
14059d788: f3 0f 10 81 ec 00 00 00     	movss	xmm0, dword ptr [rcx + 0xec]
14059d790: 89 83 08 0b 00 00           	mov	dword ptr [rbx + 0xb08], eax
14059d796: f3 0f 11 83 0c 0b 00 00     	movss	dword ptr [rbx + 0xb0c], xmm0
14059d79e: 49 bd 00 00 00 00 00 00 00 80       	movabs	r13, -0x8000000000000000
14059d7a8: 4c 89 ab 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], r13
14059d7af: c6 83 bc 00 00 00 01        	mov	byte ptr [rbx + 0xbc], 0x1
14059d7b6: c6 83 bd 00 00 00 01        	mov	byte ptr [rbx + 0xbd], 0x1
14059d7bd: c6 83 be 00 00 00 01        	mov	byte ptr [rbx + 0xbe], 0x1
14059d7c4: c6 83 bf 00 00 00 01        	mov	byte ptr [rbx + 0xbf], 0x1
14059d7cb: c6 83 c0 00 00 00 01        	mov	byte ptr [rbx + 0xc0], 0x1
14059d7d2: c6 83 c1 00 00 00 01        	mov	byte ptr [rbx + 0xc1], 0x1
14059d7d9: 48 8d b3 40 04 00 00        	lea	rsi, [rbx + 0x440]
14059d7e0: 48 8d 93 60 01 00 00        	lea	rdx, [rbx + 0x160]
14059d7e7: 4c 8d 83 00 0b 00 00        	lea	r8, [rbx + 0xb00]
14059d7ee: 48 89 f1                    	mov	rcx, rsi
14059d7f1: e8 1a 82 e2 ff              	call	0x1403c5a10 <.text+0x3c4a10>
14059d7f6: 48 b8 07 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffff9
14059d800: 48 39 83 40 04 00 00        	cmp	qword ptr [rbx + 0x440], rax
14059d807: 0f 85 2d 92 00 00           	jne	0x1405a6a3a <.text+0x5a5a3a>
14059d80d: e8 2e ff 7a 00              	call	0x140d4d740 <.text+0xd4c740>
14059d812: 48 89 c7                    	mov	rdi, rax
14059d815: f6 00 01                    	test	byte ptr [rax], 0x1
14059d818: 74 11                       	je	0x14059d82b <.text+0x59c82b>
14059d81a: 48 89 f9                    	mov	rcx, rdi
14059d81d: 48 83 c1 08                 	add	rcx, 0x8
14059d821: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
14059d825: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
14059d829: eb 45                       	jmp	0x14059d870 <.text+0x59c870>
14059d82b: c6 83 bc 00 00 00 01        	mov	byte ptr [rbx + 0xbc], 0x1
14059d832: c6 83 bd 00 00 00 01        	mov	byte ptr [rbx + 0xbd], 0x1
14059d839: c6 83 be 00 00 00 01        	mov	byte ptr [rbx + 0xbe], 0x1
14059d840: c6 83 bf 00 00 00 01        	mov	byte ptr [rbx + 0xbf], 0x1
14059d847: c6 83 c0 00 00 00 01        	mov	byte ptr [rbx + 0xc0], 0x1
14059d84e: c6 83 c1 00 00 00 01        	mov	byte ptr [rbx + 0xc1], 0x1
14059d855: e8 a6 75 7c 00              	call	0x140d64e00 <.text+0xd63e00>
14059d85a: 48 89 f9                    	mov	rcx, rdi
14059d85d: 48 83 c1 08                 	add	rcx, 0x8
14059d861: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
14059d868: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
14059d86c: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
14059d870: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
14059d874: 4c 89 01                    	mov	qword ptr [rcx], r8
14059d877: 0f 10 35 32 0e 46 02        	movups	xmm6, xmmword ptr [rip + 0x2460e32] # 0x1429fe6b0
14059d87e: 0f 29 b3 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm6
14059d885: 0f 10 3d 14 0e 46 02        	movups	xmm7, xmmword ptr [rip + 0x2460e14] # 0x1429fe6a0
14059d88c: 0f 29 bb 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm7
14059d893: 48 89 83 20 0b 00 00        	mov	qword ptr [rbx + 0xb20], rax
14059d89a: 48 89 93 28 0b 00 00        	mov	qword ptr [rbx + 0xb28], rdx
14059d8a1: e8 9a fe 7a 00              	call	0x140d4d740 <.text+0xd4c740>
14059d8a6: 48 89 c7                    	mov	rdi, rax
14059d8a9: f6 00 01                    	test	byte ptr [rax], 0x1
14059d8ac: 74 11                       	je	0x14059d8bf <.text+0x59c8bf>
14059d8ae: 48 89 f9                    	mov	rcx, rdi
14059d8b1: 48 83 c1 08                 	add	rcx, 0x8
14059d8b5: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
14059d8b9: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
14059d8bd: eb 1b                       	jmp	0x14059d8da <.text+0x59c8da>
14059d8bf: e8 3c 75 7c 00              	call	0x140d64e00 <.text+0xd63e00>
14059d8c4: 48 89 f9                    	mov	rcx, rdi
14059d8c7: 48 83 c1 08                 	add	rcx, 0x8
14059d8cb: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
14059d8d2: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
14059d8d6: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
14059d8da: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
14059d8de: 4c 89 01                    	mov	qword ptr [rcx], r8
14059d8e1: 0f 28 83 00 0b 00 00        	movaps	xmm0, xmmword ptr [rbx + 0xb00]
14059d8e8: 0f 28 8b 10 0b 00 00        	movaps	xmm1, xmmword ptr [rbx + 0xb10]
14059d8ef: 0f 28 93 20 0b 00 00        	movaps	xmm2, xmmword ptr [rbx + 0xb20]
14059d8f6: 0f 29 93 c0 0a 00 00        	movaps	xmmword ptr [rbx + 0xac0], xmm2
14059d8fd: 0f 29 8b b0 0a 00 00        	movaps	xmmword ptr [rbx + 0xab0], xmm1
14059d904: 0f 29 83 a0 0a 00 00        	movaps	xmmword ptr [rbx + 0xaa0], xmm0
14059d90b: 0f 29 bb d0 0a 00 00        	movaps	xmmword ptr [rbx + 0xad0], xmm7
14059d912: 0f 29 b3 e0 0a 00 00        	movaps	xmmword ptr [rbx + 0xae0], xmm6
14059d919: 48 89 83 f0 0a 00 00        	mov	qword ptr [rbx + 0xaf0], rax
14059d920: 48 89 93 f8 0a 00 00        	mov	qword ptr [rbx + 0xaf8], rdx
14059d927: 48 8b 95 b8 37 00 00        	mov	rdx, qword ptr [rbp + 0x37b8]
14059d92e: 48 8b 02                    	mov	rax, qword ptr [rdx]
14059d931: f0                          	lock
14059d932: 48 ff 00                    	inc	qword ptr [rax]
14059d935: 0f 8e 05 97 00 00           	jle	0x1405a7040 <.text+0x5a6040>
14059d93b: 48 8b 4a 08                 	mov	rcx, qword ptr [rdx + 0x8]
14059d93f: f0                          	lock
14059d940: 48 ff 01                    	inc	qword ptr [rcx]
14059d943: 0f 8e f7 96 00 00           	jle	0x1405a7040 <.text+0x5a6040>
14059d949: 48 8b 52 10                 	mov	rdx, qword ptr [rdx + 0x10]
14059d94d: f0                          	lock
14059d94e: 48 ff 02                    	inc	qword ptr [rdx]
14059d951: 0f 8e e9 96 00 00           	jle	0x1405a7040 <.text+0x5a6040>
14059d957: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
14059d95e: 48 89 8b 08 0b 00 00        	mov	qword ptr [rbx + 0xb08], rcx
14059d965: 48 89 93 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], rdx
14059d96c: 31 c9                       	xor	ecx, ecx
14059d96e: e8 ad c1 b3 ff              	call	0x1400d9b20 <.text+0xd8b20>
14059d973: 48 85 c0                    	test	rax, rax
14059d976: 0f 84 60 91 00 00           	je	0x1405a6adc <.text+0x5a5adc>
14059d97c: 0f 57 c0                    	xorps	xmm0, xmm0
14059d97f: 0f 11 83 08 21 00 00        	movups	xmmword ptr [rbx + 0x2108], xmm0
14059d986: 0f 10 08                    	movups	xmm1, xmmword ptr [rax]
14059d989: 48 ff 00                    	inc	qword ptr [rax]
14059d98c: 48 8b 83 10 0b 00 00        	mov	rax, qword ptr [rbx + 0xb10]
14059d993: 48 89 83 50 21 00 00        	mov	qword ptr [rbx + 0x2150], rax
14059d99a: 0f 10 93 00 0b 00 00        	movups	xmm2, xmmword ptr [rbx + 0xb00]
14059d9a1: 0f 11 93 40 21 00 00        	movups	xmmword ptr [rbx + 0x2140], xmm2
14059d9a8: 0f 10 15 41 d7 39 02        	movups	xmm2, xmmword ptr [rip + 0x239d741] # 0x14293b0f0
14059d9af: 0f 11 93 58 21 00 00        	movups	xmmword ptr [rbx + 0x2158], xmm2
14059d9b6: 0f 10 15 43 d7 39 02        	movups	xmm2, xmmword ptr [rip + 0x239d743] # 0x14293b100
14059d9bd: 0f 11 93 68 21 00 00        	movups	xmmword ptr [rbx + 0x2168], xmm2
14059d9c4: 0f 11 8b 78 21 00 00        	movups	xmmword ptr [rbx + 0x2178], xmm1
14059d9cb: 48 c7 83 e0 20 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x20e0], 0x0
14059d9d6: 48 c7 83 e8 20 00 00 08 00 00 00    	mov	qword ptr [rbx + 0x20e8], 0x8
14059d9e1: 0f 11 83 f0 20 00 00        	movups	xmmword ptr [rbx + 0x20f0], xmm0
14059d9e8: 48 c7 83 00 21 00 00 08 00 00 00    	mov	qword ptr [rbx + 0x2100], 0x8
14059d9f3: 48 c7 83 18 21 00 00 08 00 00 00    	mov	qword ptr [rbx + 0x2118], 0x8
14059d9fe: 0f 11 83 20 21 00 00        	movups	xmmword ptr [rbx + 0x2120], xmm0
14059da05: 48 c7 83 30 21 00 00 08 00 00 00    	mov	qword ptr [rbx + 0x2130], 0x8
14059da10: 48 c7 83 38 21 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x2138], 0x0
14059da1b: 48 8b 05 86 e9 08 03        	mov	rax, qword ptr [rip + 0x308e986] # 0x14362c3a8
14059da22: 48 8b 00                    	mov	rax, qword ptr [rax]
14059da25: 48 83 f8 03                 	cmp	rax, 0x3
14059da29: 0f 82 3c 01 00 00           	jb	0x14059db6b <.text+0x59cb6b>
14059da2f: 48 8d 05 52 1d 46 02        	lea	rax, [rip + 0x2461d52]  # 0x1429ff788
14059da36: 48 89 83 40 04 00 00        	mov	qword ptr [rbx + 0x440], rax
14059da3d: 48 8d 05 4c c4 b6 ff        	lea	rax, [rip - 0x493bb4]   # 0x140109e90 <.text+0x108e90>
14059da44: 48 89 83 48 04 00 00        	mov	qword ptr [rbx + 0x448], rax
14059da4b: c6 83 dc 00 00 00 01        	mov	byte ptr [rbx + 0xdc], 0x1
14059da52: c6 83 dd 00 00 00 01        	mov	byte ptr [rbx + 0xdd], 0x1
14059da59: c6 83 de 00 00 00 01        	mov	byte ptr [rbx + 0xde], 0x1
14059da60: c6 83 df 00 00 00 01        	mov	byte ptr [rbx + 0xdf], 0x1
14059da67: c6 83 e0 00 00 00 01        	mov	byte ptr [rbx + 0xe0], 0x1
14059da6e: c6 83 e1 00 00 00 01        	mov	byte ptr [rbx + 0xe1], 0x1
14059da75: c6 83 e2 00 00 00 01        	mov	byte ptr [rbx + 0xe2], 0x1
14059da7c: 48 8d 0d 35 1d 46 02        	lea	rcx, [rip + 0x2461d35]  # 0x1429ff7b8
14059da83: e8 f8 02 0e 00              	call	0x14067dd80 <.text+0x67cd80>
14059da88: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14059da8b: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
14059da8e: 48 c7 83 30 0b 00 00 03 00 00 00    	mov	qword ptr [rbx + 0xb30], 0x3
14059da99: 48 8d 0d 50 19 46 02        	lea	rcx, [rip + 0x2461950]  # 0x1429ff3f0
14059daa0: 48 89 8b 38 0b 00 00        	mov	qword ptr [rbx + 0xb38], rcx
14059daa7: 48 c7 83 40 0b 00 00 14 00 00 00    	mov	qword ptr [rbx + 0xb40], 0x14
14059dab2: 48 8d 15 ef 1c 46 02        	lea	rdx, [rip + 0x2461cef]  # 0x1429ff7a8
14059dab9: 48 89 93 50 0b 00 00        	mov	qword ptr [rbx + 0xb50], rdx
14059dac0: 48 c7 83 58 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb58], 0x1
14059dacb: 48 89 b3 60 0b 00 00        	mov	qword ptr [rbx + 0xb60], rsi
14059dad2: 48 c7 83 68 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb68], 0x1
14059dadd: 48 c7 83 70 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb70], 0x0
14059dae8: 48 c7 83 00 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb00], 0x0
14059daf3: 48 89 8b 08 0b 00 00        	mov	qword ptr [rbx + 0xb08], rcx
14059dafa: 48 c7 83 10 0b 00 00 14 00 00 00    	mov	qword ptr [rbx + 0xb10], 0x14
14059db05: 48 c7 83 18 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb18], 0x0
14059db10: 0f 11 83 20 0b 00 00        	movups	xmmword ptr [rbx + 0xb20], xmm0
14059db17: c7 83 48 0b 00 00 01 00 00 00       	mov	dword ptr [rbx + 0xb48], 0x1
14059db21: 89 83 4c 0b 00 00           	mov	dword ptr [rbx + 0xb4c], eax
14059db27: c6 83 dc 00 00 00 01        	mov	byte ptr [rbx + 0xdc], 0x1
14059db2e: c6 83 dd 00 00 00 01        	mov	byte ptr [rbx + 0xdd], 0x1
14059db35: c6 83 de 00 00 00 01        	mov	byte ptr [rbx + 0xde], 0x1
14059db3c: c6 83 df 00 00 00 01        	mov	byte ptr [rbx + 0xdf], 0x1
14059db43: c6 83 e0 00 00 00 01        	mov	byte ptr [rbx + 0xe0], 0x1
14059db4a: c6 83 e1 00 00 00 01        	mov	byte ptr [rbx + 0xe1], 0x1
14059db51: c6 83 e2 00 00 00 01        	mov	byte ptr [rbx + 0xe2], 0x1
14059db58: 48 8d 8b 70 03 00 00        	lea	rcx, [rbx + 0x370]
14059db5f: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
14059db66: e8 c5 c7 7a 00              	call	0x140d4a330 <.text+0xd49330>
14059db6b: c6 83 dc 00 00 00 01        	mov	byte ptr [rbx + 0xdc], 0x1
14059db72: c6 83 dd 00 00 00 01        	mov	byte ptr [rbx + 0xdd], 0x1
14059db79: c6 83 de 00 00 00 01        	mov	byte ptr [rbx + 0xde], 0x1
14059db80: c6 83 df 00 00 00 01        	mov	byte ptr [rbx + 0xdf], 0x1
14059db87: c6 83 e0 00 00 00 01        	mov	byte ptr [rbx + 0xe0], 0x1
14059db8e: c6 83 e1 00 00 00 01        	mov	byte ptr [rbx + 0xe1], 0x1
14059db95: c6 83 e2 00 00 00 01        	mov	byte ptr [rbx + 0xe2], 0x1
14059db9c: 48 8d 8b 48 01 00 00        	lea	rcx, [rbx + 0x148]
14059dba3: e8 78 c0 3a 00              	call	0x140949c20 <.text+0x948c20>
14059dba8: c6 83 dc 00 00 00 01        	mov	byte ptr [rbx + 0xdc], 0x1
14059dbaf: c6 83 dd 00 00 00 01        	mov	byte ptr [rbx + 0xdd], 0x1
14059dbb6: c6 83 de 00 00 00 01        	mov	byte ptr [rbx + 0xde], 0x1
14059dbbd: c6 83 df 00 00 00 01        	mov	byte ptr [rbx + 0xdf], 0x1
14059dbc4: c6 83 e0 00 00 00 01        	mov	byte ptr [rbx + 0xe0], 0x1
14059dbcb: c6 83 e1 00 00 00 01        	mov	byte ptr [rbx + 0xe1], 0x1
14059dbd2: c6 83 e2 00 00 00 01        	mov	byte ptr [rbx + 0xe2], 0x1
14059dbd9: 48 8d 8b 20 32 00 00        	lea	rcx, [rbx + 0x3220]
14059dbe0: 0f 28 c8                    	movaps	xmm1, xmm0
14059dbe3: e8 88 10 37 00              	call	0x14090ec70 <.text+0x90dc70>
14059dbe8: 0f b6 05 1a 70 09 03        	movzx	eax, byte ptr [rip + 0x309701a] # 0x143634c09
14059dbef: b9 b0 00 00 00              	mov	ecx, 0xb0
14059dbf4: ba 08 00 00 00              	mov	edx, 0x8
14059dbf9: e8 12 60 07 00              	call	0x140613c10 <.text+0x612c10>
14059dbfe: 48 85 c0                    	test	rax, rax
14059dc01: 0f 84 e6 8e 00 00           	je	0x1405a6aed <.text+0x5a5aed>
14059dc07: 48 c7 00 01 00 00 00        	mov	qword ptr [rax], 0x1
14059dc0e: 48 c7 40 08 01 00 00 00     	mov	qword ptr [rax + 0x8], 0x1
14059dc16: 48 c7 40 10 00 00 00 00     	mov	qword ptr [rax + 0x10], 0x0
14059dc1e: c6 40 18 00                 	mov	byte ptr [rax + 0x18], 0x0
14059dc22: 4c 89 68 20                 	mov	qword ptr [rax + 0x20], r13
14059dc26: c6 80 a0 00 00 00 02        	mov	byte ptr [rax + 0xa0], 0x2
14059dc2d: c6 80 a8 00 00 00 00        	mov	byte ptr [rax + 0xa8], 0x0
14059dc34: 48 89 83 58 01 00 00        	mov	qword ptr [rbx + 0x158], rax
14059dc3b: 48 8b 85 c0 37 00 00        	mov	rax, qword ptr [rbp + 0x37c0]
14059dc42: 80 b8 47 01 00 00 01        	cmp	byte ptr [rax + 0x147], 0x1
14059dc49: 75 4b                       	jne	0x14059dc96 <.text+0x59cc96>
14059dc4b: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059dc52: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059dc59: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059dc60: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059dc67: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059dc6e: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059dc75: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059dc7c: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059dc83: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059dc8a: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059dc91: e8 9a a4 d8 ff              	call	0x140328130 <.text+0x327130>
14059dc96: 48 c7 83 40 04 00 00 01 00 00 00    	mov	qword ptr [rbx + 0x440], 0x1
14059dca1: 48 c7 83 48 04 00 00 ff ff ff ff    	mov	qword ptr [rbx + 0x448], -0x1
14059dcac: 48 c7 83 50 04 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x450], 0x0
14059dcb7: 48 c7 83 60 04 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x460], 0x0
14059dcc2: 48 c7 83 70 04 00 00 01 00 00 00    	mov	qword ptr [rbx + 0x470], 0x1
14059dccd: 48 c7 83 78 04 00 00 04 00 00 00    	mov	qword ptr [rbx + 0x478], 0x4
14059dcd8: c7 83 80 04 00 00 00 00 80 3e       	mov	dword ptr [rbx + 0x480], 0x3e800000
14059dce2: 48 c7 83 88 04 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x488], 0x0
14059dced: 48 c7 83 98 04 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x498], 0x0
14059dcf8: 48 c7 83 a8 04 00 00 01 00 00 00    	mov	qword ptr [rbx + 0x4a8], 0x1
14059dd03: 48 c7 83 b0 04 00 00 04 00 00 00    	mov	qword ptr [rbx + 0x4b0], 0x4
14059dd0e: c7 83 b8 04 00 00 00 00 80 3e       	mov	dword ptr [rbx + 0x4b8], 0x3e800000
14059dd18: 48 c7 83 c0 04 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x4c0], 0x0
14059dd23: 48 c7 83 d0 04 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x4d0], 0x0
14059dd2e: 48 c7 83 e0 04 00 00 01 00 00 00    	mov	qword ptr [rbx + 0x4e0], 0x1
14059dd39: 48 c7 83 e8 04 00 00 ff ff ff ff    	mov	qword ptr [rbx + 0x4e8], -0x1
14059dd44: c7 83 f0 04 00 00 00 00 80 3f       	mov	dword ptr [rbx + 0x4f0], 0x3f800000
14059dd4e: c6 83 10 01 00 00 01        	mov	byte ptr [rbx + 0x110], 0x1
14059dd55: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059dd5c: e8 8f 62 b7 ff              	call	0x140113ff0 <.text+0x112ff0>
14059dd61: 3c 01                       	cmp	al, 0x1
14059dd63: 0f 87 d0 8d 00 00           	ja	0x1405a6b39 <.text+0x5a5b39>
14059dd69: 48 8d b3 00 0b 00 00        	lea	rsi, [rbx + 0xb00]
14059dd70: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
14059dd77: 41 b8 b8 00 00 00           	mov	r8d, 0xb8
14059dd7d: 48 89 f1                    	mov	rcx, rsi
14059dd80: e8 7b 5e 38 02              	call	0x142923c00 <.text+0x2922c00>
14059dd85: c6 83 10 01 00 00 00        	mov	byte ptr [rbx + 0x110], 0x0
14059dd8c: 4c 8d 05 3d 1a 46 02        	lea	r8, [rip + 0x2461a3d]   # 0x1429ff7d0
14059dd93: 48 89 f1                    	mov	rcx, rsi
14059dd96: 48 8b b3 c8 00 00 00        	mov	rsi, qword ptr [rbx + 0xc8]
14059dd9d: 48 89 f2                    	mov	rdx, rsi
14059dda0: e8 6b ea b4 ff              	call	0x1400ec810 <.text+0xeb810>
14059dda5: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059ddac: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059ddb3: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059ddba: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059ddc1: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059ddc8: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059ddcf: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059ddd6: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059dddd: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059dde4: 48 89 f1                    	mov	rcx, rsi
14059dde7: e8 04 62 b7 ff              	call	0x140113ff0 <.text+0x112ff0>
14059ddec: 3c 01                       	cmp	al, 0x1
14059ddee: 0f 87 92 8d 00 00           	ja	0x1405a6b86 <.text+0x5a5b86>
14059ddf4: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059ddfb: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059de02: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059de09: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059de10: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059de17: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059de1e: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059de25: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059de2c: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059de33: 48 8d 15 ae 19 46 02        	lea	rdx, [rip + 0x24619ae]  # 0x1429ff7e8
14059de3a: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059de41: e8 1a d8 d8 ff              	call	0x14032b660 <.text+0x32a660>
14059de46: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059de4d: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059de54: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059de5b: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059de62: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059de69: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059de70: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059de77: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059de7e: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059de85: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059de8c: e8 5f 61 b7 ff              	call	0x140113ff0 <.text+0x112ff0>
14059de91: 3c 01                       	cmp	al, 0x1
14059de93: 0f 87 72 8d 00 00           	ja	0x1405a6c0b <.text+0x5a5c0b>
14059de99: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059dea0: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059dea7: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059deae: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059deb5: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059debc: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059dec3: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059deca: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059ded1: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059ded8: 4c 8d 0d 21 19 46 02        	lea	r9, [rip + 0x2461921]   # 0x1429ff800
14059dedf: 31 c9                       	xor	ecx, ecx
14059dee1: 31 d2                       	xor	edx, edx
14059dee3: 4c 8b 83 c8 00 00 00        	mov	r8, qword ptr [rbx + 0xc8]
14059deea: e8 51 22 01 00              	call	0x1405b0140 <.text+0x5af140>
14059deef: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059def6: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059defd: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059df04: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059df0b: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059df12: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059df19: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059df20: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059df27: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059df2e: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059df35: e8 b6 60 b7 ff              	call	0x140113ff0 <.text+0x112ff0>
14059df3a: 3c 01                       	cmp	al, 0x1
14059df3c: 0f 87 4e 8d 00 00           	ja	0x1405a6c90 <.text+0x5a5c90>
14059df42: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059df49: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059df50: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059df57: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059df5e: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059df65: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059df6c: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059df73: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059df7a: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059df81: 48 8d 15 90 18 46 02        	lea	rdx, [rip + 0x2461890]  # 0x1429ff818
14059df88: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059df8f: e8 dc c6 d6 ff              	call	0x14030a670 <.text+0x309670>
14059df94: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059df9b: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059dfa2: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059dfa9: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059dfb0: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059dfb7: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059dfbe: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059dfc5: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059dfcc: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059dfd3: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059dfda: e8 e1 78 d8 ff              	call	0x1403258c0 <.text+0x3248c0>
14059dfdf: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059dfe6: 48 8d 48 38                 	lea	rcx, [rax + 0x38]
14059dfea: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059dff1: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059dff8: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059dfff: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e006: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e00d: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e014: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e01b: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e022: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e029: 48 ba a1 bf 76 93 bb 3e 38 2e       	movabs	rdx, 0x2e383ebb9376bfa1
14059e033: 49 b8 3c 83 7c c1 e2 47 b8 96       	movabs	r8, -0x6947b81d3e837cc4
14059e03d: 48 89 8b e8 00 00 00        	mov	qword ptr [rbx + 0xe8], rcx
14059e044: e8 a7 f6 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
14059e049: 48 85 c0                    	test	rax, rax
14059e04c: 74 48                       	je	0x14059e096 <.text+0x59d096>
14059e04e: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e055: 48 39 90 18 02 00 00        	cmp	qword ptr [rax + 0x218], rdx
14059e05c: 76 38                       	jbe	0x14059e096 <.text+0x59d096>
14059e05e: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e065: 48 8b 80 10 02 00 00        	mov	rax, qword ptr [rax + 0x210]
14059e06c: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
14059e070: 48 85 c0                    	test	rax, rax
14059e073: 74 21                       	je	0x14059e096 <.text+0x59d096>
14059e075: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e07c: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
14059e083: 48 f7 d0                    	not	rax
14059e086: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14059e08a: 48 c1 e0 05                 	shl	rax, 0x5
14059e08e: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
14059e094: 75 4b                       	jne	0x14059e0e1 <.text+0x59d0e1>
14059e096: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e09d: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e0a4: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e0ab: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e0b2: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e0b9: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e0c0: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e0c7: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e0ce: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e0d5: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e0dc: e8 ff 8d fc ff              	call	0x140566ee0 <.text+0x565ee0>
14059e0e1: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e0e8: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e0ef: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e0f6: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e0fd: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e104: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e10b: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e112: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e119: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e120: 48 ba 87 9e dd 27 a2 04 f7 47       	movabs	rdx, 0x47f704a227dd9e87
14059e12a: 49 b8 94 c7 97 b0 75 52 48 c1       	movabs	r8, -0x3eb7ad8a4f68386c
14059e134: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
14059e13b: e8 b0 f5 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
14059e140: 48 85 c0                    	test	rax, rax
14059e143: 74 48                       	je	0x14059e18d <.text+0x59d18d>
14059e145: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e14c: 48 39 90 18 02 00 00        	cmp	qword ptr [rax + 0x218], rdx
14059e153: 76 38                       	jbe	0x14059e18d <.text+0x59d18d>
14059e155: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e15c: 48 8b 80 10 02 00 00        	mov	rax, qword ptr [rax + 0x210]
14059e163: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
14059e167: 48 85 c0                    	test	rax, rax
14059e16a: 74 21                       	je	0x14059e18d <.text+0x59d18d>
14059e16c: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e173: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
14059e17a: 48 f7 d0                    	not	rax
14059e17d: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14059e181: 48 c1 e0 05                 	shl	rax, 0x5
14059e185: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
14059e18b: 75 4b                       	jne	0x14059e1d8 <.text+0x59d1d8>
14059e18d: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e194: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e19b: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e1a2: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e1a9: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e1b0: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e1b7: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e1be: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e1c5: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e1cc: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e1d3: e8 48 d7 fc ff              	call	0x14056b920 <.text+0x56a920>
14059e1d8: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e1df: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e1e6: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e1ed: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e1f4: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e1fb: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e202: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e209: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e210: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e217: 48 ba d0 0b 3b f9 98 5f 21 b5       	movabs	rdx, -0x4adea06706c4f430
14059e221: 49 b8 bb e6 4e 6b b4 29 28 a5       	movabs	r8, -0x5ad7d64b94b11945
14059e22b: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
14059e232: e8 b9 f4 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
14059e237: 48 85 c0                    	test	rax, rax
14059e23a: 74 48                       	je	0x14059e284 <.text+0x59d284>
14059e23c: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e243: 48 39 90 18 02 00 00        	cmp	qword ptr [rax + 0x218], rdx
14059e24a: 76 38                       	jbe	0x14059e284 <.text+0x59d284>
14059e24c: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e253: 48 8b 80 10 02 00 00        	mov	rax, qword ptr [rax + 0x210]
14059e25a: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
14059e25e: 48 85 c0                    	test	rax, rax
14059e261: 74 21                       	je	0x14059e284 <.text+0x59d284>
14059e263: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e26a: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
14059e271: 48 f7 d0                    	not	rax
14059e274: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14059e278: 48 c1 e0 05                 	shl	rax, 0x5
14059e27c: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
14059e282: 75 4b                       	jne	0x14059e2cf <.text+0x59d2cf>
14059e284: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e28b: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e292: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e299: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e2a0: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e2a7: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e2ae: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e2b5: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e2bc: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e2c3: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e2ca: e8 e1 74 fe ff              	call	0x1405857b0 <.text+0x5847b0>
14059e2cf: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e2d6: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e2dd: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e2e4: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e2eb: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e2f2: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e2f9: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e300: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e307: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e30e: 48 ba cb 64 e7 ae 73 16 7c bb       	movabs	rdx, -0x4483e98c51189b35
14059e318: 49 b8 d6 e2 45 39 38 ee 0a 25       	movabs	r8, 0x250aee383945e2d6
14059e322: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
14059e329: e8 c2 f3 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
14059e32e: 48 85 c0                    	test	rax, rax
14059e331: 74 48                       	je	0x14059e37b <.text+0x59d37b>
14059e333: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e33a: 48 39 90 18 02 00 00        	cmp	qword ptr [rax + 0x218], rdx
14059e341: 76 38                       	jbe	0x14059e37b <.text+0x59d37b>
14059e343: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e34a: 48 8b 80 10 02 00 00        	mov	rax, qword ptr [rax + 0x210]
14059e351: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
14059e355: 48 85 c0                    	test	rax, rax
14059e358: 74 21                       	je	0x14059e37b <.text+0x59d37b>
14059e35a: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e361: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
14059e368: 48 f7 d0                    	not	rax
14059e36b: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14059e36f: 48 c1 e0 05                 	shl	rax, 0x5
14059e373: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
14059e379: 75 4b                       	jne	0x14059e3c6 <.text+0x59d3c6>
14059e37b: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e382: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e389: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e390: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e397: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e39e: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e3a5: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e3ac: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e3b3: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e3ba: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e3c1: e8 da a3 fc ff              	call	0x1405687a0 <.text+0x5677a0>
14059e3c6: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e3cd: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e3d4: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e3db: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e3e2: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e3e9: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e3f0: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e3f7: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e3fe: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e405: 48 ba 1c 42 08 a4 38 2a 8e 99       	movabs	rdx, -0x6671d5c75bf7bde4
14059e40f: 49 b8 68 1d 0c 95 80 8e 87 f8       	movabs	r8, -0x778717f6af3e298
14059e419: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
14059e420: e8 cb f2 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
14059e425: 48 85 c0                    	test	rax, rax
14059e428: 74 48                       	je	0x14059e472 <.text+0x59d472>
14059e42a: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e431: 48 39 90 18 02 00 00        	cmp	qword ptr [rax + 0x218], rdx
14059e438: 76 38                       	jbe	0x14059e472 <.text+0x59d472>
14059e43a: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e441: 48 8b 80 10 02 00 00        	mov	rax, qword ptr [rax + 0x210]
14059e448: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
14059e44c: 48 85 c0                    	test	rax, rax
14059e44f: 74 21                       	je	0x14059e472 <.text+0x59d472>
14059e451: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e458: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
14059e45f: 48 f7 d0                    	not	rax
14059e462: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14059e466: 48 c1 e0 05                 	shl	rax, 0x5
14059e46a: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
14059e470: 75 4b                       	jne	0x14059e4bd <.text+0x59d4bd>
14059e472: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e479: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e480: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e487: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e48e: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e495: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e49c: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e4a3: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e4aa: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e4b1: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e4b8: e8 23 ed fc ff              	call	0x14056d1e0 <.text+0x56c1e0>
14059e4bd: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e4c4: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e4cb: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e4d2: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e4d9: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e4e0: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e4e7: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e4ee: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e4f5: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e4fc: 48 ba 24 4f 41 93 6e f0 06 52       	movabs	rdx, 0x5206f06e93414f24
14059e506: 49 b8 64 19 73 6f f7 e4 f4 39       	movabs	r8, 0x39f4e4f76f731964
14059e510: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
14059e517: e8 d4 f1 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
14059e51c: 48 85 c0                    	test	rax, rax
14059e51f: 74 48                       	je	0x14059e569 <.text+0x59d569>
14059e521: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e528: 48 39 90 18 02 00 00        	cmp	qword ptr [rax + 0x218], rdx
14059e52f: 76 38                       	jbe	0x14059e569 <.text+0x59d569>
14059e531: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e538: 48 8b 80 10 02 00 00        	mov	rax, qword ptr [rax + 0x210]
14059e53f: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
14059e543: 48 85 c0                    	test	rax, rax
14059e546: 74 21                       	je	0x14059e569 <.text+0x59d569>
14059e548: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e54f: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
14059e556: 48 f7 d0                    	not	rax
14059e559: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14059e55d: 48 c1 e0 05                 	shl	rax, 0x5
14059e561: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
14059e567: 75 4b                       	jne	0x14059e5b4 <.text+0x59d5b4>
14059e569: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e570: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e577: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e57e: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e585: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e58c: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e593: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e59a: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e5a1: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e5a8: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e5af: e8 3c 30 fd ff              	call	0x1405715f0 <.text+0x5705f0>
14059e5b4: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e5bb: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e5c2: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e5c9: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e5d0: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e5d7: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e5de: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e5e5: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e5ec: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e5f3: 48 ba 68 1e fa c6 44 6b 47 7e       	movabs	rdx, 0x7e476b44c6fa1e68
14059e5fd: 49 b8 3e 1f ab 41 4e 17 da 0a       	movabs	r8, 0xada174e41ab1f3e
14059e607: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
14059e60e: e8 dd f0 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
14059e613: 48 85 c0                    	test	rax, rax
14059e616: 74 48                       	je	0x14059e660 <.text+0x59d660>
14059e618: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e61f: 48 39 90 18 02 00 00        	cmp	qword ptr [rax + 0x218], rdx
14059e626: 76 38                       	jbe	0x14059e660 <.text+0x59d660>
14059e628: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e62f: 48 8b 80 10 02 00 00        	mov	rax, qword ptr [rax + 0x210]
14059e636: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
14059e63a: 48 85 c0                    	test	rax, rax
14059e63d: 74 21                       	je	0x14059e660 <.text+0x59d660>
14059e63f: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e646: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
14059e64d: 48 f7 d0                    	not	rax
14059e650: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14059e654: 48 c1 e0 05                 	shl	rax, 0x5
14059e658: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
14059e65e: 75 4b                       	jne	0x14059e6ab <.text+0x59d6ab>
14059e660: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e667: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e66e: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e675: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e67c: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e683: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e68a: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e691: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e698: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e69f: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e6a6: e8 05 48 fd ff              	call	0x140572eb0 <.text+0x571eb0>
14059e6ab: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e6b2: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e6b9: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e6c0: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e6c7: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e6ce: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e6d5: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e6dc: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e6e3: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e6ea: 48 ba 30 e8 5a 61 8d f4 c7 50       	movabs	rdx, 0x50c7f48d615ae830
14059e6f4: 49 b8 63 43 dc 2b 33 52 05 36       	movabs	r8, 0x360552332bdc4363
14059e6fe: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
14059e705: e8 e6 ef b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
14059e70a: 48 85 c0                    	test	rax, rax
14059e70d: 74 48                       	je	0x14059e757 <.text+0x59d757>
14059e70f: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e716: 48 39 90 18 02 00 00        	cmp	qword ptr [rax + 0x218], rdx
14059e71d: 76 38                       	jbe	0x14059e757 <.text+0x59d757>
14059e71f: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e726: 48 8b 80 10 02 00 00        	mov	rax, qword ptr [rax + 0x210]
14059e72d: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
14059e731: 48 85 c0                    	test	rax, rax
14059e734: 74 21                       	je	0x14059e757 <.text+0x59d757>
14059e736: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e73d: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
14059e744: 48 f7 d0                    	not	rax
14059e747: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14059e74b: 48 c1 e0 05                 	shl	rax, 0x5
14059e74f: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
14059e755: 75 4b                       	jne	0x14059e7a2 <.text+0x59d7a2>
14059e757: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e75e: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e765: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e76c: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e773: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e77a: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e781: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e788: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e78f: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e796: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e79d: e8 ee 4a fe ff              	call	0x140583290 <.text+0x582290>
14059e7a2: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e7a9: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e7b0: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e7b7: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e7be: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e7c5: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e7cc: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e7d3: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e7da: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e7e1: 48 ba 97 e7 5e ca 10 f7 ee 76       	movabs	rdx, 0x76eef710ca5ee797
14059e7eb: 49 b8 13 96 f3 b0 69 51 4a 80       	movabs	r8, -0x7fb5ae964f0c69ed
14059e7f5: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
14059e7fc: e8 ef ee b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
14059e801: 48 85 c0                    	test	rax, rax
14059e804: 74 48                       	je	0x14059e84e <.text+0x59d84e>
14059e806: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e80d: 48 39 90 18 02 00 00        	cmp	qword ptr [rax + 0x218], rdx
14059e814: 76 38                       	jbe	0x14059e84e <.text+0x59d84e>
14059e816: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e81d: 48 8b 80 10 02 00 00        	mov	rax, qword ptr [rax + 0x210]
14059e824: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
14059e828: 48 85 c0                    	test	rax, rax
14059e82b: 74 21                       	je	0x14059e84e <.text+0x59d84e>
14059e82d: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e834: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
14059e83b: 48 f7 d0                    	not	rax
14059e83e: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14059e842: 48 c1 e0 05                 	shl	rax, 0x5
14059e846: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
14059e84c: 75 4b                       	jne	0x14059e899 <.text+0x59d899>
14059e84e: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e855: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e85c: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e863: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e86a: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e871: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e878: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e87f: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e886: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e88d: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e894: e8 e7 a2 fd ff              	call	0x140578b80 <.text+0x577b80>
14059e899: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e8a0: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e8a7: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e8ae: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e8b5: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e8bc: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e8c3: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e8ca: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e8d1: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e8d8: 48 ba 1c d6 a9 4e 06 3d f9 d8       	movabs	rdx, -0x2706c2f9b15629e4
14059e8e2: 49 b8 2f 83 d9 66 70 61 47 64       	movabs	r8, 0x6447617066d9832f
14059e8ec: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
14059e8f3: e8 f8 ed b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
14059e8f8: 48 85 c0                    	test	rax, rax
14059e8fb: 74 48                       	je	0x14059e945 <.text+0x59d945>
14059e8fd: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e904: 48 39 90 18 02 00 00        	cmp	qword ptr [rax + 0x218], rdx
14059e90b: 76 38                       	jbe	0x14059e945 <.text+0x59d945>
14059e90d: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e914: 48 8b 80 10 02 00 00        	mov	rax, qword ptr [rax + 0x210]
14059e91b: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
14059e91f: 48 85 c0                    	test	rax, rax
14059e922: 74 21                       	je	0x14059e945 <.text+0x59d945>
14059e924: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e92b: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
14059e932: 48 f7 d0                    	not	rax
14059e935: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14059e939: 48 c1 e0 05                 	shl	rax, 0x5
14059e93d: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
14059e943: 75 4b                       	jne	0x14059e990 <.text+0x59d990>
14059e945: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e94c: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e953: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e95a: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e961: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e968: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e96f: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e976: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e97d: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e984: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059e98b: e8 50 4b fd ff              	call	0x1405734e0 <.text+0x5724e0>
14059e990: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059e997: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059e99e: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059e9a5: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059e9ac: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059e9b3: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059e9ba: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059e9c1: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059e9c8: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059e9cf: 48 ba 2f cb 1f f2 24 2c 69 53       	movabs	rdx, 0x53692c24f21fcb2f
14059e9d9: 49 b8 6b 07 48 ef 13 dd 74 bc       	movabs	r8, -0x438b22ec10b7f895
14059e9e3: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
14059e9ea: e8 01 ed b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
14059e9ef: 48 85 c0                    	test	rax, rax
14059e9f2: 74 48                       	je	0x14059ea3c <.text+0x59da3c>
14059e9f4: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059e9fb: 48 39 90 18 02 00 00        	cmp	qword ptr [rax + 0x218], rdx
14059ea02: 76 38                       	jbe	0x14059ea3c <.text+0x59da3c>
14059ea04: 48 8b 83 c8 00 00 00        	mov	rax, qword ptr [rbx + 0xc8]
14059ea0b: 48 8b 80 10 02 00 00        	mov	rax, qword ptr [rax + 0x210]
14059ea12: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
14059ea16: 48 85 c0                    	test	rax, rax
14059ea19: 74 21                       	je	0x14059ea3c <.text+0x59da3c>
14059ea1b: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059ea22: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
14059ea29: 48 f7 d0                    	not	rax
14059ea2c: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14059ea30: 48 c1 e0 05                 	shl	rax, 0x5
14059ea34: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
14059ea3a: 75 4b                       	jne	0x14059ea87 <.text+0x59da87>
14059ea3c: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059ea43: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059ea4a: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059ea51: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059ea58: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059ea5f: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059ea66: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059ea6d: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059ea74: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059ea7b: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059ea82: e8 39 25 fd ff              	call	0x140570fc0 <.text+0x56ffc0>
14059ea87: 48 8b 85 b0 37 00 00        	mov	rax, qword ptr [rbp + 0x37b0]
14059ea8e: 4c 8d b0 e0 00 00 00        	lea	r14, [rax + 0xe0]
14059ea95: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059ea9c: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059eaa3: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059eaaa: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059eab1: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059eab8: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059eabf: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059eac6: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059eacd: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059ead4: 4c 89 f1                    	mov	rcx, r14
14059ead7: e8 64 f4 36 00              	call	0x14090df40 <.text+0x90cf40>
14059eadc: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059eae3: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059eaea: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059eaf1: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059eaf8: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059eaff: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059eb06: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059eb0d: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059eb14: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059eb1b: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
14059eb22: 89 c2                       	mov	edx, eax
14059eb24: e8 c7 b5 31 00              	call	0x1408ba0f0 <.text+0x8b90f0>
14059eb29: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059eb30: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059eb37: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059eb3e: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059eb45: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059eb4c: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059eb53: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059eb5a: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059eb61: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059eb68: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
14059eb6f: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
14059eb76: e8 e5 31 b9 ff              	call	0x140131d60 <.text+0x130d60>
14059eb7b: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059eb82: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059eb89: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059eb90: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059eb97: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059eb9e: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059eba5: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059ebac: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059ebb3: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059ebba: 48 89 c1                    	mov	rcx, rax
14059ebbd: e8 fe 6b 3a 00              	call	0x1409457c0 <.text+0x9447c0>
14059ebc2: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059ebc9: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059ebd0: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059ebd7: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059ebde: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059ebe5: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059ebec: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059ebf3: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059ebfa: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059ec01: 48 89 c1                    	mov	rcx, rax
14059ec04: e8 07 52 3a 00              	call	0x140943e10 <.text+0x942e10>
14059ec09: 48 8b 93 38 01 00 00        	mov	rdx, qword ptr [rbx + 0x138]
14059ec10: 48 83 c2 10                 	add	rdx, 0x10
14059ec14: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059ec1b: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059ec22: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059ec29: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059ec30: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059ec37: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059ec3e: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059ec45: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059ec4c: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059ec53: 48 89 c1                    	mov	rcx, rax
14059ec56: 4c 8b 85 b0 37 00 00        	mov	r8, qword ptr [rbp + 0x37b0]
14059ec5d: e8 2e 15 3a 00              	call	0x140940190 <.text+0x93f190>
14059ec62: 48 89 c6                    	mov	rsi, rax
14059ec65: 48 8b 93 38 01 00 00        	mov	rdx, qword ptr [rbx + 0x138]
14059ec6c: 48 83 c2 10                 	add	rdx, 0x10
14059ec70: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059ec77: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059ec7e: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059ec85: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059ec8c: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059ec93: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059ec9a: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059eca1: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059eca8: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059ecaf: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
14059ecb6: e8 b5 38 66 00              	call	0x140c02570 <.text+0xc01570>
14059ecbb: 0f 10 83 40 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb40]
14059ecc2: 0f 11 83 87 04 00 00        	movups	xmmword ptr [rbx + 0x487], xmm0
14059ecc9: 0f 10 83 00 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb00]
14059ecd0: 0f 10 8b 10 0b 00 00        	movups	xmm1, xmmword ptr [rbx + 0xb10]
14059ecd7: 0f 10 93 20 0b 00 00        	movups	xmm2, xmmword ptr [rbx + 0xb20]
14059ecde: 0f 10 9b 30 0b 00 00        	movups	xmm3, xmmword ptr [rbx + 0xb30]
14059ece5: 0f 11 9b 77 04 00 00        	movups	xmmword ptr [rbx + 0x477], xmm3
14059ecec: 0f 11 93 67 04 00 00        	movups	xmmword ptr [rbx + 0x467], xmm2
14059ecf3: 0f 11 8b 57 04 00 00        	movups	xmmword ptr [rbx + 0x457], xmm1
14059ecfa: 0f 11 83 47 04 00 00        	movups	xmmword ptr [rbx + 0x447], xmm0
14059ed01: c6 83 00 0b 00 00 00        	mov	byte ptr [rbx + 0xb00], 0x0
14059ed08: 0f 10 83 40 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x440]
14059ed0f: 0f 11 83 01 0b 00 00        	movups	xmmword ptr [rbx + 0xb01], xmm0
14059ed16: 0f 10 83 50 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x450]
14059ed1d: 0f 11 83 11 0b 00 00        	movups	xmmword ptr [rbx + 0xb11], xmm0
14059ed24: 0f 10 83 60 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x460]
14059ed2b: 0f 11 83 21 0b 00 00        	movups	xmmword ptr [rbx + 0xb21], xmm0
14059ed32: 0f 10 83 70 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x470]
14059ed39: 0f 11 83 31 0b 00 00        	movups	xmmword ptr [rbx + 0xb31], xmm0
14059ed40: 0f 10 83 80 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x480]
14059ed47: 0f 11 83 41 0b 00 00        	movups	xmmword ptr [rbx + 0xb41], xmm0
14059ed4e: 48 8b 83 8f 04 00 00        	mov	rax, qword ptr [rbx + 0x48f]
14059ed55: 48 89 83 50 0b 00 00        	mov	qword ptr [rbx + 0xb50], rax
14059ed5c: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059ed63: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059ed6a: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059ed71: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059ed78: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059ed7f: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059ed86: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059ed8d: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059ed94: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059ed9b: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
14059eda2: 48 89 f1                    	mov	rcx, rsi
14059eda5: e8 96 0a b8 ff              	call	0x14011f840 <.text+0x11e840>
14059edaa: 48 8b 93 38 01 00 00        	mov	rdx, qword ptr [rbx + 0x138]
14059edb1: f0                          	lock
14059edb2: 48 ff 02                    	inc	qword ptr [rdx]
14059edb5: 0f 8e 85 82 00 00           	jle	0x1405a7040 <.text+0x5a6040>
14059edbb: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059edc2: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059edc9: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059edd0: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059edd7: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059edde: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059ede5: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
14059edec: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059edf3: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059edfa: 48 89 c1                    	mov	rcx, rax
14059edfd: e8 8e dd b7 ff              	call	0x14011cb90 <.text+0x11bb90>
14059ee02: 48 89 c6                    	mov	rsi, rax
14059ee05: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059ee0c: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059ee13: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059ee1a: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059ee21: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059ee28: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059ee2f: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059ee36: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059ee3d: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059ee44: 48 8b 8b b8 02 00 00        	mov	rcx, qword ptr [rbx + 0x2b8]
14059ee4b: 48 8b 93 b0 02 00 00        	mov	rdx, qword ptr [rbx + 0x2b0]
14059ee52: e8 59 8e 3e 00              	call	0x140987cb0 <.text+0x986cb0>
14059ee57: 49 89 d0                    	mov	r8, rdx
14059ee5a: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059ee61: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059ee68: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059ee6f: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059ee76: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059ee7d: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059ee84: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059ee8b: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059ee92: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059ee99: 48 89 f1                    	mov	rcx, rsi
14059ee9c: 48 89 c2                    	mov	rdx, rax
14059ee9f: e8 5c 4f b8 ff              	call	0x140123e00 <.text+0x122e00>
14059eea4: 48 89 c6                    	mov	rsi, rax
14059eea7: 44 0f 28 05 51 ff 45 02     	movaps	xmm8, xmmword ptr [rip + 0x245ff51] # 0x1429fee00
14059eeaf: 44 0f 29 83 70 04 00 00     	movaps	xmmword ptr [rbx + 0x470], xmm8
14059eeb7: 44 0f 28 0d 31 ff 45 02     	movaps	xmm9, xmmword ptr [rip + 0x245ff31] # 0x1429fedf0
14059eebf: 44 0f 29 8b 60 04 00 00     	movaps	xmmword ptr [rbx + 0x460], xmm9
14059eec7: 44 0f 28 15 11 ff 45 02     	movaps	xmm10, xmmword ptr [rip + 0x245ff11] # 0x1429fede0
14059eecf: 44 0f 29 93 50 04 00 00     	movaps	xmmword ptr [rbx + 0x450], xmm10
14059eed7: 44 0f 28 1d f1 fe 45 02     	movaps	xmm11, xmmword ptr [rip + 0x245fef1] # 0x1429fedd0
14059eedf: 44 0f 29 9b 40 04 00 00     	movaps	xmmword ptr [rbx + 0x440], xmm11
14059eee7: 44 0f 29 9b 80 04 00 00     	movaps	xmmword ptr [rbx + 0x480], xmm11
14059eeef: 44 0f 29 93 90 04 00 00     	movaps	xmmword ptr [rbx + 0x490], xmm10
14059eef7: 44 0f 29 8b a0 04 00 00     	movaps	xmmword ptr [rbx + 0x4a0], xmm9
14059eeff: 44 0f 29 83 b0 04 00 00     	movaps	xmmword ptr [rbx + 0x4b0], xmm8
14059ef07: 44 0f 29 9b c0 04 00 00     	movaps	xmmword ptr [rbx + 0x4c0], xmm11
14059ef0f: 44 0f 29 93 d0 04 00 00     	movaps	xmmword ptr [rbx + 0x4d0], xmm10
14059ef17: 44 0f 29 8b e0 04 00 00     	movaps	xmmword ptr [rbx + 0x4e0], xmm9
14059ef1f: 44 0f 29 83 f0 04 00 00     	movaps	xmmword ptr [rbx + 0x4f0], xmm8
14059ef27: 44 0f 29 83 30 05 00 00     	movaps	xmmword ptr [rbx + 0x530], xmm8
14059ef2f: 44 0f 29 8b 20 05 00 00     	movaps	xmmword ptr [rbx + 0x520], xmm9
14059ef37: 44 0f 29 93 10 05 00 00     	movaps	xmmword ptr [rbx + 0x510], xmm10
14059ef3f: 44 0f 29 9b 00 05 00 00     	movaps	xmmword ptr [rbx + 0x500], xmm11
14059ef47: c7 83 08 0c 00 00 00 00 00 00       	mov	dword ptr [rbx + 0xc08], 0x0
14059ef51: 48 8d bb 00 0b 00 00        	lea	rdi, [rbx + 0xb00]
14059ef58: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
14059ef5f: 41 b8 00 01 00 00           	mov	r8d, 0x100
14059ef65: 48 89 f9                    	mov	rcx, rdi
14059ef68: e8 93 4c 38 02              	call	0x142923c00 <.text+0x2922c00>
14059ef6d: 48 c7 83 00 0c 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xc00], 0x0
14059ef78: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059ef7f: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059ef86: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059ef8d: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059ef94: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059ef9b: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059efa2: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059efa9: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059efb0: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059efb7: 48 89 f1                    	mov	rcx, rsi
14059efba: 48 89 fa                    	mov	rdx, rdi
14059efbd: e8 fe 6c b9 ff              	call	0x140135cc0 <.text+0x134cc0>
14059efc2: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059efc9: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059efd0: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059efd7: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059efde: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059efe5: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059efec: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059eff3: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059effa: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f001: 48 89 c1                    	mov	rcx, rax
14059f004: e8 b7 50 3a 00              	call	0x1409440c0 <.text+0x9430c0>
14059f009: 48 89 c6                    	mov	rsi, rax
14059f00c: 48 8b 85 b8 37 00 00        	mov	rax, qword ptr [rbp + 0x37b8]
14059f013: 48 8b 08                    	mov	rcx, qword ptr [rax]
14059f016: f0                          	lock
14059f017: 48 ff 01                    	inc	qword ptr [rcx]
14059f01a: 0f 8e 20 80 00 00           	jle	0x1405a7040 <.text+0x5a6040>
14059f020: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f027: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f02e: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f035: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f03c: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059f043: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f04a: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f051: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f058: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f05f: e8 1c ed 0d 00              	call	0x14067dd80 <.text+0x67cd80>
14059f064: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f06b: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f072: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f079: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f080: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059f087: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f08e: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f095: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f09c: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f0a3: 48 89 f1                    	mov	rcx, rsi
14059f0a6: 48 89 c2                    	mov	rdx, rax
14059f0a9: e8 92 8d b8 ff              	call	0x140127e40 <.text+0x126e40>
14059f0ae: 48 89 c6                    	mov	rsi, rax
14059f0b1: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f0b8: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f0bf: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f0c6: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f0cd: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059f0d4: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f0db: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f0e2: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f0e9: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f0f0: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
14059f0f7: e8 e4 0b 3a 00              	call	0x14093fce0 <.text+0x93ece0>
14059f0fc: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f103: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f10a: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f111: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f118: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059f11f: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f126: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f12d: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f134: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f13b: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
14059f142: 48 89 f1                    	mov	rcx, rsi
14059f145: e8 c6 98 b8 ff              	call	0x140128a10 <.text+0x127a10>
14059f14a: 48 89 c6                    	mov	rsi, rax
14059f14d: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f154: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f15b: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f162: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f169: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059f170: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f177: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f17e: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f185: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f18c: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
14059f193: 48 8b 95 c0 37 00 00        	mov	rdx, qword ptr [rbp + 0x37c0]
14059f19a: e8 f1 41 db ff              	call	0x140353390 <.text+0x352390>
14059f19f: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f1a6: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f1ad: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f1b4: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f1bb: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
14059f1c2: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f1c9: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f1d0: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f1d7: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f1de: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
14059f1e5: 4c 8d 83 20 32 00 00        	lea	r8, [rbx + 0x3220]
14059f1ec: 48 89 f1                    	mov	rcx, rsi
14059f1ef: e8 8c 3d 3a 00              	call	0x140942f80 <.text+0x941f80>
14059f1f4: 48 89 c6                    	mov	rsi, rax
14059f1f7: 48 8d bb 00 0b 00 00        	lea	rdi, [rbx + 0xb00]
14059f1fe: 48 8d 93 20 32 00 00        	lea	rdx, [rbx + 0x3220]
14059f205: 41 b8 e0 04 00 00           	mov	r8d, 0x4e0
14059f20b: 48 89 f9                    	mov	rcx, rdi
14059f20e: e8 ed 49 38 02              	call	0x142923c00 <.text+0x2922c00>
14059f213: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f21a: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f221: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f228: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f22f: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f236: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f23d: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f244: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f24b: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f252: 48 89 f1                    	mov	rcx, rsi
14059f255: 48 89 fa                    	mov	rdx, rdi
14059f258: e8 b3 42 b9 ff              	call	0x140133510 <.text+0x132510>
14059f25d: 48 89 c6                    	mov	rsi, rax
14059f260: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f267: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f26e: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f275: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f27c: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f283: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f28a: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f291: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f298: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f29f: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
14059f2a6: e8 45 f5 30 00              	call	0x1408ae7f0 <.text+0x8ad7f0>
14059f2ab: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f2b2: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f2b9: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f2c0: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f2c7: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f2ce: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f2d5: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f2dc: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f2e3: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f2ea: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
14059f2f1: 48 89 f1                    	mov	rcx, rsi
14059f2f4: e8 37 39 b8 ff              	call	0x140122c30 <.text+0x121c30>
14059f2f9: 48 89 c6                    	mov	rsi, rax
14059f2fc: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f303: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f30a: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f311: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f318: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f31f: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f326: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f32d: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f334: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f33b: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
14059f342: e8 69 12 2d 00              	call	0x1408705b0 <.text+0x86f5b0>
14059f347: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f34e: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f355: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f35c: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f363: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f36a: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f371: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f378: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f37f: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f386: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
14059f38d: 48 89 f1                    	mov	rcx, rsi
14059f390: e8 fb 06 b9 ff              	call	0x14012fa90 <.text+0x12ea90>
14059f395: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f39c: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f3a3: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f3aa: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f3b1: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f3b8: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f3bf: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f3c6: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f3cd: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f3d4: 48 89 c1                    	mov	rcx, rax
14059f3d7: 31 d2                       	xor	edx, edx
14059f3d9: e8 92 dd b7 ff              	call	0x14011d170 <.text+0x11c170>
14059f3de: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f3e5: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f3ec: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f3f3: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f3fa: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f401: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f408: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f40f: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f416: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f41d: 0f 57 c9                    	xorps	xmm1, xmm1
14059f420: 0f 57 d2                    	xorps	xmm2, xmm2
14059f423: 48 89 c1                    	mov	rcx, rax
14059f426: e8 f5 90 b9 ff              	call	0x140138520 <.text+0x137520>
14059f42b: 48 89 c6                    	mov	rsi, rax
14059f42e: 48 8b 93 38 01 00 00        	mov	rdx, qword ptr [rbx + 0x138]
14059f435: 48 83 c2 10                 	add	rdx, 0x10
14059f439: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f440: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f447: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f44e: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f455: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f45c: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f463: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f46a: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f471: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f478: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
14059f47f: e8 4c 23 3f 00              	call	0x1409917d0 <.text+0x9907d0>
14059f484: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f48b: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f492: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f499: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f4a0: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f4a7: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f4ae: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f4b5: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f4bc: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f4c3: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
14059f4ca: 48 89 f1                    	mov	rcx, rsi
14059f4cd: e8 fe 64 b7 ff              	call	0x1401159d0 <.text+0x1149d0>
14059f4d2: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f4d9: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f4e0: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f4e7: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f4ee: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f4f5: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f4fc: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f503: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f50a: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f511: 48 89 c1                    	mov	rcx, rax
14059f514: e8 97 41 ef ff              	call	0x1404936b0 <.text+0x4926b0>
14059f519: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f520: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f527: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f52e: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f535: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f53c: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f543: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f54a: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f551: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f558: 48 89 c1                    	mov	rcx, rax
14059f55b: e8 c0 3b ef ff              	call	0x140493120 <.text+0x492120>
14059f560: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f567: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f56e: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f575: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f57c: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f583: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f58a: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f591: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f598: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f59f: 48 89 c1                    	mov	rcx, rax
14059f5a2: e8 a9 69 3a 00              	call	0x140945f50 <.text+0x944f50>
14059f5a7: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f5ae: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f5b5: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f5bc: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f5c3: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f5ca: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f5d1: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f5d8: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f5df: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f5e6: 48 89 c1                    	mov	rcx, rax
14059f5e9: e8 32 64 3a 00              	call	0x140945a20 <.text+0x944a20>
14059f5ee: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f5f5: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f5fc: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f603: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f60a: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f611: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f618: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f61f: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f626: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f62d: 48 89 c1                    	mov	rcx, rax
14059f630: e8 bb 5f 3a 00              	call	0x1409455f0 <.text+0x9445f0>
14059f635: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f63c: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f643: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f64a: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f651: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f658: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f65f: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f666: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f66d: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f674: 48 89 c1                    	mov	rcx, rax
14059f677: 48 8b 95 b0 37 00 00        	mov	rdx, qword ptr [rbp + 0x37b0]
14059f67e: e8 fd 35 3a 00              	call	0x140942c80 <.text+0x941c80>
14059f683: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f68a: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f691: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f698: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f69f: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f6a6: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f6ad: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f6b4: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f6bb: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f6c2: 48 89 c1                    	mov	rcx, rax
14059f6c5: e8 96 3f 3a 00              	call	0x140943660 <.text+0x942660>
14059f6ca: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f6d1: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f6d8: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f6df: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f6e6: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f6ed: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f6f4: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f6fb: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f702: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f709: 48 89 c1                    	mov	rcx, rax
14059f70c: e8 8f 41 3a 00              	call	0x1409438a0 <.text+0x9428a0>
14059f711: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f718: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f71f: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f726: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f72d: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f734: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f73b: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f742: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f749: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f750: 48 89 c1                    	mov	rcx, rax
14059f753: e8 e8 47 3a 00              	call	0x140943f40 <.text+0x942f40>
14059f758: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f75f: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f766: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f76d: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f774: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f77b: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f782: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f789: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f790: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f797: 48 89 c1                    	mov	rcx, rax
14059f79a: e8 d1 48 3a 00              	call	0x140944070 <.text+0x943070>
14059f79f: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f7a6: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f7ad: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f7b4: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f7bb: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f7c2: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f7c9: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f7d0: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f7d7: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f7de: 48 89 c1                    	mov	rcx, rax
14059f7e1: e8 9a 4a 3a 00              	call	0x140944280 <.text+0x943280>
14059f7e6: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f7ed: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f7f4: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f7fb: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f802: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f809: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f810: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f817: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f81e: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f825: 48 89 c1                    	mov	rcx, rax
14059f828: e8 73 19 3a 00              	call	0x1409411a0 <.text+0x9401a0>
14059f82d: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f834: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f83b: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f842: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f849: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f850: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f857: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f85e: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f865: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f86c: 48 89 c1                    	mov	rcx, rax
14059f86f: e8 dc 1b 3a 00              	call	0x140941450 <.text+0x940450>
14059f874: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f87b: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f882: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f889: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f890: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f897: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f89e: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f8a5: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f8ac: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f8b3: 48 89 c1                    	mov	rcx, rax
14059f8b6: e8 f5 4a 3a 00              	call	0x1409443b0 <.text+0x9433b0>
14059f8bb: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f8c2: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f8c9: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f8d0: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f8d7: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f8de: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f8e5: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f8ec: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f8f3: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f8fa: 48 89 c1                    	mov	rcx, rax
14059f8fd: e8 de 4e 3a 00              	call	0x1409447e0 <.text+0x9437e0>
14059f902: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f909: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f910: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f917: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f91e: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f925: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f92c: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f933: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f93a: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f941: 48 89 c1                    	mov	rcx, rax
14059f944: e8 67 03 09 00              	call	0x14062fcb0 <.text+0x62ecb0>
14059f949: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f950: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f957: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f95e: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f965: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f96c: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f973: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f97a: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f981: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f988: 48 89 c1                    	mov	rcx, rax
14059f98b: e8 d0 62 3a 00              	call	0x140945c60 <.text+0x944c60>
14059f990: 48 89 c6                    	mov	rsi, rax
14059f993: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059f99a: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059f9a1: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059f9a8: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059f9af: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059f9b6: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059f9bd: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059f9c4: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059f9cb: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059f9d2: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
14059f9d9: e8 32 9c e2 ff              	call	0x1403c9610 <.text+0x3c8610>
14059f9de: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
14059f9e5: e8 06 1f b1 ff              	call	0x1400b18f0 <.text+0xb08f0>
14059f9ea: 0f 10 83 f0 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x25f0]
14059f9f1: 0f 29 83 60 0b 00 00        	movaps	xmmword ptr [rbx + 0xb60], xmm0
14059f9f8: 0f 10 83 e0 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x25e0]
14059f9ff: 0f 29 83 50 0b 00 00        	movaps	xmmword ptr [rbx + 0xb50], xmm0
14059fa06: 0f 10 83 d0 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x25d0]
14059fa0d: 0f 29 83 40 0b 00 00        	movaps	xmmword ptr [rbx + 0xb40], xmm0
14059fa14: 0f 10 83 90 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2590]
14059fa1b: 0f 10 8b a0 25 00 00        	movups	xmm1, xmmword ptr [rbx + 0x25a0]
14059fa22: 0f 10 93 b0 25 00 00        	movups	xmm2, xmmword ptr [rbx + 0x25b0]
14059fa29: 0f 10 9b c0 25 00 00        	movups	xmm3, xmmword ptr [rbx + 0x25c0]
14059fa30: 0f 29 9b 30 0b 00 00        	movaps	xmmword ptr [rbx + 0xb30], xmm3
14059fa37: 0f 29 93 20 0b 00 00        	movaps	xmmword ptr [rbx + 0xb20], xmm2
14059fa3e: 0f 29 8b 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm1
14059fa45: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
14059fa4c: 0f 10 83 40 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x440]
14059fa53: 0f 10 8b 50 04 00 00        	movups	xmm1, xmmword ptr [rbx + 0x450]
14059fa5a: 0f 10 93 60 04 00 00        	movups	xmm2, xmmword ptr [rbx + 0x460]
14059fa61: 0f 10 9b 70 04 00 00        	movups	xmm3, xmmword ptr [rbx + 0x470]
14059fa68: 0f 29 83 70 0b 00 00        	movaps	xmmword ptr [rbx + 0xb70], xmm0
14059fa6f: 0f 29 8b 80 0b 00 00        	movaps	xmmword ptr [rbx + 0xb80], xmm1
14059fa76: 0f 29 93 90 0b 00 00        	movaps	xmmword ptr [rbx + 0xb90], xmm2
14059fa7d: 0f 29 9b a0 0b 00 00        	movaps	xmmword ptr [rbx + 0xba0], xmm3
14059fa84: 0f 10 83 80 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x480]
14059fa8b: 0f 29 83 b0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbb0], xmm0
14059fa92: 0f 10 83 90 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x490]
14059fa99: 0f 29 83 c0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbc0], xmm0
14059faa0: 0f 10 83 a0 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x4a0]
14059faa7: 0f 29 83 d0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbd0], xmm0
14059faae: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059fab5: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059fabc: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059fac3: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059faca: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059fad1: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059fad8: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059fadf: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059fae6: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059faed: 4c 8d 0d 3c fd 45 02        	lea	r9, [rip + 0x245fd3c]   # 0x1429ff830
14059faf4: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
14059fafb: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
14059fb02: 41 b0 0a                    	mov	r8b, 0xa
14059fb05: e8 16 6a b3 ff              	call	0x1400d6520 <.text+0xd5520>
14059fb0a: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059fb11: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059fb18: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059fb1f: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059fb26: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059fb2d: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059fb34: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059fb3b: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059fb42: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059fb49: 48 8d 8b e8 24 00 00        	lea	rcx, [rbx + 0x24e8]
14059fb50: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
14059fb57: e8 74 3c b3 ff              	call	0x1400d37d0 <.text+0xd27d0>
14059fb5c: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
14059fb63: e8 48 8e f9 ff              	call	0x1405389b0 <.text+0x5379b0>
14059fb68: c6 83 40 04 00 00 16        	mov	byte ptr [rbx + 0x440], 0x16
14059fb6f: 48 8b 0d fa c7 08 03        	mov	rcx, qword ptr [rip + 0x308c7fa] # 0x14362c370
14059fb76: 4c 8d 05 1b b4 38 02        	lea	r8, [rip + 0x238b41b]   # 0x14292af98
14059fb7d: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
14059fb84: e8 97 51 dc ff              	call	0x140364d20 <.text+0x363d20>
14059fb89: 49 89 d0                    	mov	r8, rdx
14059fb8c: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
14059fb93: 48 89 c2                    	mov	rdx, rax
14059fb96: e8 25 6d b3 ff              	call	0x1400d68c0 <.text+0xd58c0>
14059fb9b: 0f 10 83 60 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb60]
14059fba2: 0f 29 83 d0 03 00 00        	movaps	xmmword ptr [rbx + 0x3d0], xmm0
14059fba9: 0f 10 83 50 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb50]
14059fbb0: 0f 29 83 c0 03 00 00        	movaps	xmmword ptr [rbx + 0x3c0], xmm0
14059fbb7: 0f 10 83 40 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb40]
14059fbbe: 0f 29 83 b0 03 00 00        	movaps	xmmword ptr [rbx + 0x3b0], xmm0
14059fbc5: 0f 10 83 00 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb00]
14059fbcc: 0f 10 8b 10 0b 00 00        	movups	xmm1, xmmword ptr [rbx + 0xb10]
14059fbd3: 0f 10 93 20 0b 00 00        	movups	xmm2, xmmword ptr [rbx + 0xb20]
14059fbda: 0f 10 9b 30 0b 00 00        	movups	xmm3, xmmword ptr [rbx + 0xb30]
14059fbe1: 0f 29 9b a0 03 00 00        	movaps	xmmword ptr [rbx + 0x3a0], xmm3
14059fbe8: 0f 29 93 90 03 00 00        	movaps	xmmword ptr [rbx + 0x390], xmm2
14059fbef: 0f 29 8b 80 03 00 00        	movaps	xmmword ptr [rbx + 0x380], xmm1
14059fbf6: 0f 29 83 70 03 00 00        	movaps	xmmword ptr [rbx + 0x370], xmm0
14059fbfd: 4c 8d 05 44 fc 45 02        	lea	r8, [rip + 0x245fc44]   # 0x1429ff848
14059fc04: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
14059fc0b: b2 24                       	mov	dl, 0x24
14059fc0d: e8 0e c6 d2 ff              	call	0x1402cc220 <.text+0x2cb220>
14059fc12: 48 8d 8b f0 1f 00 00        	lea	rcx, [rbx + 0x1ff0]
14059fc19: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
14059fc20: e8 ab 3b b3 ff              	call	0x1400d37d0 <.text+0xd27d0>
14059fc25: 4c 8d 05 34 fc 45 02        	lea	r8, [rip + 0x245fc34]   # 0x1429ff860
14059fc2c: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
14059fc33: b2 24                       	mov	dl, 0x24
14059fc35: e8 c6 47 f8 ff              	call	0x140524400 <.text+0x523400>
14059fc3a: 48 8d 8b 30 22 00 00        	lea	rcx, [rbx + 0x2230]
14059fc41: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
14059fc48: e8 83 3b b3 ff              	call	0x1400d37d0 <.text+0xd27d0>
14059fc4d: 4c 8d 05 24 fc 45 02        	lea	r8, [rip + 0x245fc24]   # 0x1429ff878
14059fc54: 48 8d 8b f0 2b 00 00        	lea	rcx, [rbx + 0x2bf0]
14059fc5b: b2 24                       	mov	dl, 0x24
14059fc5d: e8 3e 2d da ff              	call	0x1403429a0 <.text+0x3419a0>
14059fc62: 4c 8d 05 27 fc 45 02        	lea	r8, [rip + 0x245fc27]   # 0x1429ff890
14059fc69: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
14059fc70: b2 2c                       	mov	dl, 0x2c
14059fc72: e8 b9 7d e2 ff              	call	0x1403c7a30 <.text+0x3c6a30>
14059fc77: 4c 8d 05 2a fc 45 02        	lea	r8, [rip + 0x245fc2a]   # 0x1429ff8a8
14059fc7e: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
14059fc85: b2 0e                       	mov	dl, 0xe
14059fc87: e8 74 07 02 00              	call	0x1405c0400 <.text+0x5bf400>
14059fc8c: 0f 10 83 48 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2548]
14059fc93: 0f 29 83 60 0b 00 00        	movaps	xmmword ptr [rbx + 0xb60], xmm0
14059fc9a: 0f 10 83 38 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2538]
14059fca1: 0f 29 83 50 0b 00 00        	movaps	xmmword ptr [rbx + 0xb50], xmm0
14059fca8: 0f 10 83 28 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2528]
14059fcaf: 0f 29 83 40 0b 00 00        	movaps	xmmword ptr [rbx + 0xb40], xmm0
14059fcb6: 0f 10 83 e8 24 00 00        	movups	xmm0, xmmword ptr [rbx + 0x24e8]
14059fcbd: 0f 10 8b f8 24 00 00        	movups	xmm1, xmmword ptr [rbx + 0x24f8]
14059fcc4: 0f 10 93 08 25 00 00        	movups	xmm2, xmmword ptr [rbx + 0x2508]
14059fccb: 0f 10 9b 18 25 00 00        	movups	xmm3, xmmword ptr [rbx + 0x2518]
14059fcd2: 0f 29 9b 30 0b 00 00        	movaps	xmmword ptr [rbx + 0xb30], xmm3
14059fcd9: 0f 29 93 20 0b 00 00        	movaps	xmmword ptr [rbx + 0xb20], xmm2
14059fce0: 0f 29 8b 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm1
14059fce7: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
14059fcee: 0f 28 83 70 03 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x370]
14059fcf5: 0f 28 8b 80 03 00 00        	movaps	xmm1, xmmword ptr [rbx + 0x380]
14059fcfc: 0f 28 93 90 03 00 00        	movaps	xmm2, xmmword ptr [rbx + 0x390]
14059fd03: 0f 28 9b a0 03 00 00        	movaps	xmm3, xmmword ptr [rbx + 0x3a0]
14059fd0a: 0f 29 83 70 0b 00 00        	movaps	xmmword ptr [rbx + 0xb70], xmm0
14059fd11: 0f 29 8b 80 0b 00 00        	movaps	xmmword ptr [rbx + 0xb80], xmm1
14059fd18: 0f 29 93 90 0b 00 00        	movaps	xmmword ptr [rbx + 0xb90], xmm2
14059fd1f: 0f 29 9b a0 0b 00 00        	movaps	xmmword ptr [rbx + 0xba0], xmm3
14059fd26: 0f 28 83 b0 03 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x3b0]
14059fd2d: 0f 29 83 b0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbb0], xmm0
14059fd34: 0f 28 83 c0 03 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x3c0]
14059fd3b: 0f 29 83 c0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbc0], xmm0
14059fd42: 0f 28 83 d0 03 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x3d0]
14059fd49: 0f 29 83 d0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbd0], xmm0
14059fd50: 0f 10 83 50 20 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2050]
14059fd57: 0f 29 83 40 0c 00 00        	movaps	xmmword ptr [rbx + 0xc40], xmm0
14059fd5e: 0f 10 83 40 20 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2040]
14059fd65: 0f 29 83 30 0c 00 00        	movaps	xmmword ptr [rbx + 0xc30], xmm0
14059fd6c: 0f 10 83 30 20 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2030]
14059fd73: 0f 29 83 20 0c 00 00        	movaps	xmmword ptr [rbx + 0xc20], xmm0
14059fd7a: 0f 10 83 f0 1f 00 00        	movups	xmm0, xmmword ptr [rbx + 0x1ff0]
14059fd81: 0f 10 8b 00 20 00 00        	movups	xmm1, xmmword ptr [rbx + 0x2000]
14059fd88: 0f 10 93 10 20 00 00        	movups	xmm2, xmmword ptr [rbx + 0x2010]
14059fd8f: 0f 10 9b 20 20 00 00        	movups	xmm3, xmmword ptr [rbx + 0x2020]
14059fd96: 0f 29 9b 10 0c 00 00        	movaps	xmmword ptr [rbx + 0xc10], xmm3
14059fd9d: 0f 29 93 00 0c 00 00        	movaps	xmmword ptr [rbx + 0xc00], xmm2
14059fda4: 0f 29 8b f0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbf0], xmm1
14059fdab: 0f 29 83 e0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbe0], xmm0
14059fdb2: 0f 10 83 30 22 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2230]
14059fdb9: 0f 10 8b 40 22 00 00        	movups	xmm1, xmmword ptr [rbx + 0x2240]
14059fdc0: 0f 10 93 50 22 00 00        	movups	xmm2, xmmword ptr [rbx + 0x2250]
14059fdc7: 0f 10 9b 60 22 00 00        	movups	xmm3, xmmword ptr [rbx + 0x2260]
14059fdce: 0f 29 83 50 0c 00 00        	movaps	xmmword ptr [rbx + 0xc50], xmm0
14059fdd5: 0f 29 8b 60 0c 00 00        	movaps	xmmword ptr [rbx + 0xc60], xmm1
14059fddc: 0f 29 93 70 0c 00 00        	movaps	xmmword ptr [rbx + 0xc70], xmm2
14059fde3: 0f 29 9b 80 0c 00 00        	movaps	xmmword ptr [rbx + 0xc80], xmm3
14059fdea: 0f 10 83 70 22 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2270]
14059fdf1: 0f 29 83 90 0c 00 00        	movaps	xmmword ptr [rbx + 0xc90], xmm0
14059fdf8: 0f 10 83 80 22 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2280]
14059fdff: 0f 29 83 a0 0c 00 00        	movaps	xmmword ptr [rbx + 0xca0], xmm0
14059fe06: 0f 10 83 90 22 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2290]
14059fe0d: 0f 29 83 b0 0c 00 00        	movaps	xmmword ptr [rbx + 0xcb0], xmm0
14059fe14: 0f 10 83 50 2c 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2c50]
14059fe1b: 0f 29 83 20 0d 00 00        	movaps	xmmword ptr [rbx + 0xd20], xmm0
14059fe22: 0f 10 83 40 2c 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2c40]
14059fe29: 0f 29 83 10 0d 00 00        	movaps	xmmword ptr [rbx + 0xd10], xmm0
14059fe30: 0f 10 83 30 2c 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2c30]
14059fe37: 0f 29 83 00 0d 00 00        	movaps	xmmword ptr [rbx + 0xd00], xmm0
14059fe3e: 0f 10 83 f0 2b 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2bf0]
14059fe45: 0f 10 8b 00 2c 00 00        	movups	xmm1, xmmword ptr [rbx + 0x2c00]
14059fe4c: 0f 10 93 10 2c 00 00        	movups	xmm2, xmmword ptr [rbx + 0x2c10]
14059fe53: 0f 10 9b 20 2c 00 00        	movups	xmm3, xmmword ptr [rbx + 0x2c20]
14059fe5a: 0f 29 9b f0 0c 00 00        	movaps	xmmword ptr [rbx + 0xcf0], xmm3
14059fe61: 0f 29 93 e0 0c 00 00        	movaps	xmmword ptr [rbx + 0xce0], xmm2
14059fe68: 0f 29 8b d0 0c 00 00        	movaps	xmmword ptr [rbx + 0xcd0], xmm1
14059fe6f: 0f 29 83 c0 0c 00 00        	movaps	xmmword ptr [rbx + 0xcc0], xmm0
14059fe76: 0f 10 83 f0 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x25f0]
14059fe7d: 0f 29 83 90 0d 00 00        	movaps	xmmword ptr [rbx + 0xd90], xmm0
14059fe84: 0f 10 83 e0 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x25e0]
14059fe8b: 0f 29 83 80 0d 00 00        	movaps	xmmword ptr [rbx + 0xd80], xmm0
14059fe92: 0f 10 83 d0 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x25d0]
14059fe99: 0f 29 83 70 0d 00 00        	movaps	xmmword ptr [rbx + 0xd70], xmm0
14059fea0: 0f 10 83 90 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2590]
14059fea7: 0f 10 8b a0 25 00 00        	movups	xmm1, xmmword ptr [rbx + 0x25a0]
14059feae: 0f 10 93 b0 25 00 00        	movups	xmm2, xmmword ptr [rbx + 0x25b0]
14059feb5: 0f 10 9b c0 25 00 00        	movups	xmm3, xmmword ptr [rbx + 0x25c0]
14059febc: 0f 29 9b 60 0d 00 00        	movaps	xmmword ptr [rbx + 0xd60], xmm3
14059fec3: 0f 29 93 50 0d 00 00        	movaps	xmmword ptr [rbx + 0xd50], xmm2
14059feca: 0f 29 8b 40 0d 00 00        	movaps	xmmword ptr [rbx + 0xd40], xmm1
14059fed1: 0f 29 83 30 0d 00 00        	movaps	xmmword ptr [rbx + 0xd30], xmm0
14059fed8: 0f 10 83 a0 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x4a0]
14059fedf: 0f 29 83 00 0e 00 00        	movaps	xmmword ptr [rbx + 0xe00], xmm0
14059fee6: 0f 10 83 90 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x490]
14059feed: 0f 29 83 f0 0d 00 00        	movaps	xmmword ptr [rbx + 0xdf0], xmm0
14059fef4: 0f 10 83 80 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x480]
14059fefb: 0f 29 83 e0 0d 00 00        	movaps	xmmword ptr [rbx + 0xde0], xmm0
14059ff02: 0f 10 83 40 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x440]
14059ff09: 0f 10 8b 50 04 00 00        	movups	xmm1, xmmword ptr [rbx + 0x450]
14059ff10: 0f 10 93 60 04 00 00        	movups	xmm2, xmmword ptr [rbx + 0x460]
14059ff17: 0f 10 9b 70 04 00 00        	movups	xmm3, xmmword ptr [rbx + 0x470]
14059ff1e: 0f 29 9b d0 0d 00 00        	movaps	xmmword ptr [rbx + 0xdd0], xmm3
14059ff25: 0f 29 93 c0 0d 00 00        	movaps	xmmword ptr [rbx + 0xdc0], xmm2
14059ff2c: 0f 29 8b b0 0d 00 00        	movaps	xmmword ptr [rbx + 0xdb0], xmm1
14059ff33: 0f 29 83 a0 0d 00 00        	movaps	xmmword ptr [rbx + 0xda0], xmm0
14059ff3a: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059ff41: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059ff48: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059ff4f: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059ff56: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059ff5d: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059ff64: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059ff6b: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059ff72: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059ff79: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
14059ff80: 48 89 f1                    	mov	rcx, rsi
14059ff83: e8 08 75 d8 ff              	call	0x140327490 <.text+0x326490>
14059ff88: 48 89 f1                    	mov	rcx, rsi
14059ff8b: 48 83 c1 38                 	add	rcx, 0x38
14059ff8f: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
14059ff96: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
14059ff9d: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
14059ffa4: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
14059ffab: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
14059ffb2: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
14059ffb9: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
14059ffc0: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
14059ffc7: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
14059ffce: 48 ba b2 0e 27 45 55 12 79 13       	movabs	rdx, 0x1379125545270eb2
14059ffd8: 49 b8 7b 58 2a c9 c0 26 ae 4e       	movabs	r8, 0x4eae26c0c92a587b
14059ffe2: e8 09 d7 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
14059ffe7: 48 85 c0                    	test	rax, rax
14059ffea: 74 33                       	je	0x1405a001f <.text+0x59f01f>
14059ffec: 48 39 96 18 02 00 00        	cmp	qword ptr [rsi + 0x218], rdx
14059fff3: 76 2a                       	jbe	0x1405a001f <.text+0x59f01f>
14059fff5: 48 8b 86 10 02 00 00        	mov	rax, qword ptr [rsi + 0x210]
14059fffc: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a0000: 48 85 c0                    	test	rax, rax
1405a0003: 74 1a                       	je	0x1405a001f <.text+0x59f01f>
1405a0005: 48 8b 8e e0 01 00 00        	mov	rcx, qword ptr [rsi + 0x1e0]
1405a000c: 48 f7 d0                    	not	rax
1405a000f: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a0013: 48 c1 e0 05                 	shl	rax, 0x5
1405a0017: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a001d: 75 47                       	jne	0x1405a0066 <.text+0x59f066>
1405a001f: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0026: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a002d: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0034: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a003b: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0042: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0049: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0050: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0057: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a005e: 48 89 f1                    	mov	rcx, rsi
1405a0061: e8 aa d7 fc ff              	call	0x14056d810 <.text+0x56c810>
1405a0066: 48 8b 8b 58 01 00 00        	mov	rcx, qword ptr [rbx + 0x158]
1405a006d: f0                          	lock
1405a006e: 48 ff 01                    	inc	qword ptr [rcx]
1405a0071: 0f 8e c9 6f 00 00           	jle	0x1405a7040 <.text+0x5a6040>
1405a0077: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a007e: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0085: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a008c: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0093: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a009a: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a00a1: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a00a8: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a00af: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a00b6: e8 c5 dc 0d 00              	call	0x14067dd80 <.text+0x67cd80>
1405a00bb: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a00c2: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a00c9: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a00d0: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a00d7: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a00de: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a00e5: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a00ec: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a00f3: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a00fa: 48 89 f1                    	mov	rcx, rsi
1405a00fd: 48 89 c2                    	mov	rdx, rax
1405a0100: e8 9b 89 b9 ff              	call	0x140138aa0 <.text+0x137aa0>
1405a0105: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a010c: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0113: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a011a: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0121: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0128: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a012f: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0136: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a013d: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0144: 48 89 c1                    	mov	rcx, rax
1405a0147: e8 a4 04 09 00              	call	0x1406305f0 <.text+0x62f5f0>
1405a014c: 49 89 c7                    	mov	r15, rax
1405a014f: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0156: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a015d: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0164: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a016b: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0172: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0179: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0180: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0187: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a018e: 4c 8d 05 2b f7 45 02        	lea	r8, [rip + 0x245f72b]   # 0x1429ff8c0
1405a0195: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a019c: b2 07                       	mov	dl, 0x7
1405a019e: e8 6d 35 dc ff              	call	0x140363710 <.text+0x362710>
1405a01a3: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a01aa: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a01b1: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a01b8: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a01bf: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a01c6: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a01cd: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a01d4: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a01db: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a01e2: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a01e9: 4c 89 f9                    	mov	rcx, r15
1405a01ec: e8 ff 74 d8 ff              	call	0x1403276f0 <.text+0x3266f0>
1405a01f1: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a01f8: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a01ff: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0206: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a020d: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0214: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a021b: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0222: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0229: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0230: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a0237: e8 74 6e dd ff              	call	0x1403770b0 <.text+0x3760b0>
1405a023c: c6 83 90 25 00 00 21        	mov	byte ptr [rbx + 0x2590], 0x21
1405a0243: 48 8b 0d 26 c1 08 03        	mov	rcx, qword ptr [rip + 0x308c126] # 0x14362c370
1405a024a: 4c 8d 05 47 ad 38 02        	lea	r8, [rip + 0x238ad47]   # 0x14292af98
1405a0251: 48 8d 93 90 25 00 00        	lea	rdx, [rbx + 0x2590]
1405a0258: e8 c3 4a dc ff              	call	0x140364d20 <.text+0x363d20>
1405a025d: 49 89 d0                    	mov	r8, rdx
1405a0260: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a0267: 48 89 c2                    	mov	rdx, rax
1405a026a: e8 51 66 b3 ff              	call	0x1400d68c0 <.text+0xd58c0>
1405a026f: 0f 10 83 60 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb60]
1405a0276: 0f 29 83 a0 04 00 00        	movaps	xmmword ptr [rbx + 0x4a0], xmm0
1405a027d: 0f 10 83 50 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb50]
1405a0284: 0f 29 83 90 04 00 00        	movaps	xmmword ptr [rbx + 0x490], xmm0
1405a028b: 0f 10 83 40 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb40]
1405a0292: 0f 29 83 80 04 00 00        	movaps	xmmword ptr [rbx + 0x480], xmm0
1405a0299: 0f 10 83 00 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb00]
1405a02a0: 0f 10 8b 10 0b 00 00        	movups	xmm1, xmmword ptr [rbx + 0xb10]
1405a02a7: 0f 10 93 20 0b 00 00        	movups	xmm2, xmmword ptr [rbx + 0xb20]
1405a02ae: 0f 10 9b 30 0b 00 00        	movups	xmm3, xmmword ptr [rbx + 0xb30]
1405a02b5: 0f 29 9b 70 04 00 00        	movaps	xmmword ptr [rbx + 0x470], xmm3
1405a02bc: 0f 29 93 60 04 00 00        	movaps	xmmword ptr [rbx + 0x460], xmm2
1405a02c3: 0f 29 8b 50 04 00 00        	movaps	xmmword ptr [rbx + 0x450], xmm1
1405a02ca: 0f 29 83 40 04 00 00        	movaps	xmmword ptr [rbx + 0x440], xmm0
1405a02d1: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a02d8: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a02df: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a02e6: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a02ed: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a02f4: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a02fb: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0302: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0309: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0310: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a0317: 4c 89 f9                    	mov	rcx, r15
1405a031a: e8 d1 73 d8 ff              	call	0x1403276f0 <.text+0x3266f0>
1405a031f: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0326: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a032d: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0334: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a033b: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0342: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0349: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0350: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0357: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a035e: 48 8d 8b f0 2b 00 00        	lea	rcx, [rbx + 0x2bf0]
1405a0365: e8 66 b9 bc ff              	call	0x14016bcd0 <.text+0x16acd0>
1405a036a: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405a0371: e8 6a 6b 01 00              	call	0x1405b6ee0 <.text+0x5b5ee0>
1405a0376: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a037d: e8 fe 7e d4 ff              	call	0x1402e8280 <.text+0x2e7280>
1405a0382: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a0389: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a0390: e8 bb 2f b3 ff              	call	0x1400d3350 <.text+0xd2350>
1405a0395: 0f 10 83 f0 2b 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2bf0]
1405a039c: 0f 10 8b 00 2c 00 00        	movups	xmm1, xmmword ptr [rbx + 0x2c00]
1405a03a3: 0f 10 93 10 2c 00 00        	movups	xmm2, xmmword ptr [rbx + 0x2c10]
1405a03aa: 0f 10 9b 20 2c 00 00        	movups	xmm3, xmmword ptr [rbx + 0x2c20]
1405a03b1: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
1405a03b8: 0f 29 8b 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm1
1405a03bf: 0f 29 93 20 0b 00 00        	movaps	xmmword ptr [rbx + 0xb20], xmm2
1405a03c6: 0f 29 9b 30 0b 00 00        	movaps	xmmword ptr [rbx + 0xb30], xmm3
1405a03cd: 0f 10 83 30 2c 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2c30]
1405a03d4: 0f 29 83 40 0b 00 00        	movaps	xmmword ptr [rbx + 0xb40], xmm0
1405a03db: 0f 10 83 40 2c 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2c40]
1405a03e2: 0f 29 83 50 0b 00 00        	movaps	xmmword ptr [rbx + 0xb50], xmm0
1405a03e9: 0f 10 83 50 2c 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2c50]
1405a03f0: 0f 29 83 60 0b 00 00        	movaps	xmmword ptr [rbx + 0xb60], xmm0
1405a03f7: 0f 10 83 90 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2590]
1405a03fe: 0f 10 8b a0 25 00 00        	movups	xmm1, xmmword ptr [rbx + 0x25a0]
1405a0405: 0f 10 93 b0 25 00 00        	movups	xmm2, xmmword ptr [rbx + 0x25b0]
1405a040c: 0f 10 9b c0 25 00 00        	movups	xmm3, xmmword ptr [rbx + 0x25c0]
1405a0413: 0f 29 83 70 0b 00 00        	movaps	xmmword ptr [rbx + 0xb70], xmm0
1405a041a: 0f 29 8b 80 0b 00 00        	movaps	xmmword ptr [rbx + 0xb80], xmm1
1405a0421: 0f 29 93 90 0b 00 00        	movaps	xmmword ptr [rbx + 0xb90], xmm2
1405a0428: 0f 29 9b a0 0b 00 00        	movaps	xmmword ptr [rbx + 0xba0], xmm3
1405a042f: 0f 10 83 d0 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x25d0]
1405a0436: 0f 29 83 b0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbb0], xmm0
1405a043d: 0f 10 83 e0 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x25e0]
1405a0444: 0f 29 83 c0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbc0], xmm0
1405a044b: 0f 10 83 f0 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x25f0]
1405a0452: 0f 29 83 d0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbd0], xmm0
1405a0459: 0f 10 83 a0 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x4a0]
1405a0460: 0f 29 83 40 0c 00 00        	movaps	xmmword ptr [rbx + 0xc40], xmm0
1405a0467: 0f 10 83 90 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x490]
1405a046e: 0f 29 83 30 0c 00 00        	movaps	xmmword ptr [rbx + 0xc30], xmm0
1405a0475: 0f 10 83 80 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x480]
1405a047c: 0f 29 83 20 0c 00 00        	movaps	xmmword ptr [rbx + 0xc20], xmm0
1405a0483: 0f 10 83 40 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x440]
1405a048a: 0f 10 8b 50 04 00 00        	movups	xmm1, xmmword ptr [rbx + 0x450]
1405a0491: 0f 10 93 60 04 00 00        	movups	xmm2, xmmword ptr [rbx + 0x460]
1405a0498: 0f 10 9b 70 04 00 00        	movups	xmm3, xmmword ptr [rbx + 0x470]
1405a049f: 0f 29 9b 10 0c 00 00        	movaps	xmmword ptr [rbx + 0xc10], xmm3
1405a04a6: 0f 29 93 00 0c 00 00        	movaps	xmmword ptr [rbx + 0xc00], xmm2
1405a04ad: 0f 29 8b f0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbf0], xmm1
1405a04b4: 0f 29 83 e0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbe0], xmm0
1405a04bb: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a04c2: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a04c9: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a04d0: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a04d7: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a04de: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a04e5: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a04ec: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a04f3: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a04fa: 4c 8d 0d d7 f3 45 02        	lea	r9, [rip + 0x245f3d7]   # 0x1429ff8d8
1405a0501: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a0508: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a050f: 41 b0 27                    	mov	r8b, 0x27
1405a0512: e8 29 0b 07 00              	call	0x140611040 <.text+0x610040>
1405a0517: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a051e: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0525: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a052c: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0533: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a053a: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0541: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0548: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a054f: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0556: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a055d: 4c 89 f9                    	mov	rcx, r15
1405a0560: e8 8b 71 d8 ff              	call	0x1403276f0 <.text+0x3266f0>
1405a0565: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a056c: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0573: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a057a: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0581: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0588: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a058f: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0596: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a059d: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a05a4: 4c 89 f9                    	mov	rcx, r15
1405a05a7: e8 c4 77 d8 ff              	call	0x140327d70 <.text+0x326d70>
1405a05ac: 4d 89 fc                    	mov	r12, r15
1405a05af: 49 83 c4 38                 	add	r12, 0x38
1405a05b3: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a05ba: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a05c1: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a05c8: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a05cf: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a05d6: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a05dd: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a05e4: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a05eb: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a05f2: 48 ba 3d 4e 90 7e 6e a6 8d 0b       	movabs	rdx, 0xb8da66e7e904e3d
1405a05fc: 49 b8 45 17 a4 91 18 5d be 2f       	movabs	r8, 0x2fbe5d1891a41745
1405a0606: 4c 89 e1                    	mov	rcx, r12
1405a0609: e8 e2 d0 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a060e: 48 85 c0                    	test	rax, rax
1405a0611: 74 33                       	je	0x1405a0646 <.text+0x59f646>
1405a0613: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a061a: 76 2a                       	jbe	0x1405a0646 <.text+0x59f646>
1405a061c: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a0623: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a0627: 48 85 c0                    	test	rax, rax
1405a062a: 74 1a                       	je	0x1405a0646 <.text+0x59f646>
1405a062c: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a0633: 48 f7 d0                    	not	rax
1405a0636: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a063a: 48 c1 e0 05                 	shl	rax, 0x5
1405a063e: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a0644: 75 47                       	jne	0x1405a068d <.text+0x59f68d>
1405a0646: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a064d: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0654: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a065b: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0662: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0669: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0670: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0677: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a067e: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0685: 4c 89 f9                    	mov	rcx, r15
1405a0688: e8 e3 7a fc ff              	call	0x140568170 <.text+0x567170>
1405a068d: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0694: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a069b: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a06a2: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a06a9: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a06b0: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a06b7: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a06be: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a06c5: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a06cc: 48 ba 0d 06 51 76 b7 40 bf 75       	movabs	rdx, 0x75bf40b77651060d
1405a06d6: 49 b8 10 92 a1 09 d8 b9 6a c8       	movabs	r8, -0x37954627f65e6df0
1405a06e0: 4c 89 e1                    	mov	rcx, r12
1405a06e3: e8 08 d0 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a06e8: 48 85 c0                    	test	rax, rax
1405a06eb: 74 33                       	je	0x1405a0720 <.text+0x59f720>
1405a06ed: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a06f4: 76 2a                       	jbe	0x1405a0720 <.text+0x59f720>
1405a06f6: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a06fd: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a0701: 48 85 c0                    	test	rax, rax
1405a0704: 74 1a                       	je	0x1405a0720 <.text+0x59f720>
1405a0706: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a070d: 48 f7 d0                    	not	rax
1405a0710: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a0714: 48 c1 e0 05                 	shl	rax, 0x5
1405a0718: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a071e: 75 47                       	jne	0x1405a0767 <.text+0x59f767>
1405a0720: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0727: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a072e: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0735: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a073c: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0743: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a074a: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0751: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0758: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a075f: 4c 89 f9                    	mov	rcx, r15
1405a0762: e8 99 8c fc ff              	call	0x140569400 <.text+0x568400>
1405a0767: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a076e: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0775: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a077c: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0783: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a078a: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0791: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0798: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a079f: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a07a6: 48 ba 5f 04 ba ac cf 82 dd bb       	movabs	rdx, -0x44227d305345fba1
1405a07b0: 49 b8 36 88 49 49 a8 7b f2 ca       	movabs	r8, -0x350d8457b6b677ca
1405a07ba: 4c 89 e1                    	mov	rcx, r12
1405a07bd: e8 2e cf b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a07c2: 48 85 c0                    	test	rax, rax
1405a07c5: 74 33                       	je	0x1405a07fa <.text+0x59f7fa>
1405a07c7: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a07ce: 76 2a                       	jbe	0x1405a07fa <.text+0x59f7fa>
1405a07d0: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a07d7: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a07db: 48 85 c0                    	test	rax, rax
1405a07de: 74 1a                       	je	0x1405a07fa <.text+0x59f7fa>
1405a07e0: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a07e7: 48 f7 d0                    	not	rax
1405a07ea: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a07ee: 48 c1 e0 05                 	shl	rax, 0x5
1405a07f2: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a07f8: 75 47                       	jne	0x1405a0841 <.text+0x59f841>
1405a07fa: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0801: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0808: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a080f: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0816: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a081d: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0824: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a082b: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0832: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0839: 4c 89 f9                    	mov	rcx, r15
1405a083c: e8 3f 5a fc ff              	call	0x140566280 <.text+0x565280>
1405a0841: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0848: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a084f: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0856: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a085d: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0864: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a086b: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0872: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0879: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0880: 48 ba 4f cc 30 77 91 03 5d 1c       	movabs	rdx, 0x1c5d03917730cc4f
1405a088a: 49 b8 6c da ba 18 fb e4 29 3d       	movabs	r8, 0x3d29e4fb18bada6c
1405a0894: 4c 89 e1                    	mov	rcx, r12
1405a0897: e8 54 ce b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a089c: 48 85 c0                    	test	rax, rax
1405a089f: 74 33                       	je	0x1405a08d4 <.text+0x59f8d4>
1405a08a1: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a08a8: 76 2a                       	jbe	0x1405a08d4 <.text+0x59f8d4>
1405a08aa: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a08b1: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a08b5: 48 85 c0                    	test	rax, rax
1405a08b8: 74 1a                       	je	0x1405a08d4 <.text+0x59f8d4>
1405a08ba: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a08c1: 48 f7 d0                    	not	rax
1405a08c4: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a08c8: 48 c1 e0 05                 	shl	rax, 0x5
1405a08cc: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a08d2: 75 47                       	jne	0x1405a091b <.text+0x59f91b>
1405a08d4: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a08db: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a08e2: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a08e9: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a08f0: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a08f7: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a08fe: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0905: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a090c: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0913: 4c 89 f9                    	mov	rcx, r15
1405a0916: e8 35 df fd ff              	call	0x14057e850 <.text+0x57d850>
1405a091b: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0922: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0929: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0930: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0937: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a093e: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0945: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a094c: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0953: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a095a: 48 ba f9 e1 c9 a4 84 46 cb 6e       	movabs	rdx, 0x6ecb4684a4c9e1f9
1405a0964: 49 b8 dc 1d c8 ba 1a e2 e2 fa       	movabs	r8, -0x51d1de54537e224
1405a096e: 4c 89 e1                    	mov	rcx, r12
1405a0971: e8 7a cd b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a0976: 48 85 c0                    	test	rax, rax
1405a0979: 74 33                       	je	0x1405a09ae <.text+0x59f9ae>
1405a097b: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a0982: 76 2a                       	jbe	0x1405a09ae <.text+0x59f9ae>
1405a0984: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a098b: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a098f: 48 85 c0                    	test	rax, rax
1405a0992: 74 1a                       	je	0x1405a09ae <.text+0x59f9ae>
1405a0994: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a099b: 48 f7 d0                    	not	rax
1405a099e: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a09a2: 48 c1 e0 05                 	shl	rax, 0x5
1405a09a6: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a09ac: 75 47                       	jne	0x1405a09f5 <.text+0x59f9f5>
1405a09ae: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a09b5: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a09bc: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a09c3: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a09ca: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a09d1: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a09d8: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a09df: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a09e6: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a09ed: 4c 89 f9                    	mov	rcx, r15
1405a09f0: e8 0b 50 fd ff              	call	0x140575a00 <.text+0x574a00>
1405a09f5: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a09fc: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0a03: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0a0a: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0a11: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0a18: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0a1f: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0a26: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0a2d: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0a34: 48 ba c0 d6 8d 67 90 c5 c2 da       	movabs	rdx, -0x253d3a6f98722940
1405a0a3e: 49 b8 59 4d ae 33 67 eb 30 5d       	movabs	r8, 0x5d30eb6733ae4d59
1405a0a48: 4c 89 e1                    	mov	rcx, r12
1405a0a4b: e8 a0 cc b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a0a50: 48 85 c0                    	test	rax, rax
1405a0a53: 74 33                       	je	0x1405a0a88 <.text+0x59fa88>
1405a0a55: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a0a5c: 76 2a                       	jbe	0x1405a0a88 <.text+0x59fa88>
1405a0a5e: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a0a65: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a0a69: 48 85 c0                    	test	rax, rax
1405a0a6c: 74 1a                       	je	0x1405a0a88 <.text+0x59fa88>
1405a0a6e: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a0a75: 48 f7 d0                    	not	rax
1405a0a78: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a0a7c: 48 c1 e0 05                 	shl	rax, 0x5
1405a0a80: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a0a86: 75 47                       	jne	0x1405a0acf <.text+0x59facf>
1405a0a88: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0a8f: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0a96: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0a9d: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0aa4: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0aab: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0ab2: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0ab9: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0ac0: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0ac7: 4c 89 f9                    	mov	rcx, r15
1405a0aca: e8 51 3a fe ff              	call	0x140584520 <.text+0x583520>
1405a0acf: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0ad6: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0add: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0ae4: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0aeb: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0af2: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0af9: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0b00: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0b07: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0b0e: 48 ba 66 79 0e 3a cd b2 b1 25       	movabs	rdx, 0x25b1b2cd3a0e7966
1405a0b18: 49 b8 4e 41 f6 43 ba 86 21 10       	movabs	r8, 0x102186ba43f6414e
1405a0b22: 4c 89 e1                    	mov	rcx, r12
1405a0b25: e8 c6 cb b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a0b2a: 48 85 c0                    	test	rax, rax
1405a0b2d: 74 33                       	je	0x1405a0b62 <.text+0x59fb62>
1405a0b2f: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a0b36: 76 2a                       	jbe	0x1405a0b62 <.text+0x59fb62>
1405a0b38: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a0b3f: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a0b43: 48 85 c0                    	test	rax, rax
1405a0b46: 74 1a                       	je	0x1405a0b62 <.text+0x59fb62>
1405a0b48: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a0b4f: 48 f7 d0                    	not	rax
1405a0b52: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a0b56: 48 c1 e0 05                 	shl	rax, 0x5
1405a0b5a: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a0b60: 75 47                       	jne	0x1405a0ba9 <.text+0x59fba9>
1405a0b62: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0b69: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0b70: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0b77: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0b7e: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0b85: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0b8c: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0b93: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0b9a: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0ba1: 4c 89 f9                    	mov	rcx, r15
1405a0ba4: e8 e7 c3 fd ff              	call	0x14057cf90 <.text+0x57bf90>
1405a0ba9: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0bb0: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0bb7: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0bbe: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0bc5: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0bcc: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0bd3: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0bda: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0be1: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0be8: 48 ba 98 67 2b ce f9 b6 e1 1b       	movabs	rdx, 0x1be1b6f9ce2b6798
1405a0bf2: 49 b8 93 7b 40 fb 2b 95 06 d6       	movabs	r8, -0x29f96ad404bf846d
1405a0bfc: 4c 89 e1                    	mov	rcx, r12
1405a0bff: e8 ec ca b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a0c04: 48 85 c0                    	test	rax, rax
1405a0c07: 74 33                       	je	0x1405a0c3c <.text+0x59fc3c>
1405a0c09: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a0c10: 76 2a                       	jbe	0x1405a0c3c <.text+0x59fc3c>
1405a0c12: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a0c19: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a0c1d: 48 85 c0                    	test	rax, rax
1405a0c20: 74 1a                       	je	0x1405a0c3c <.text+0x59fc3c>
1405a0c22: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a0c29: 48 f7 d0                    	not	rax
1405a0c2c: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a0c30: 48 c1 e0 05                 	shl	rax, 0x5
1405a0c34: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a0c3a: 75 47                       	jne	0x1405a0c83 <.text+0x59fc83>
1405a0c3c: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0c43: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0c4a: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0c51: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0c58: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0c5f: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0c66: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0c6d: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0c74: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0c7b: 4c 89 f9                    	mov	rcx, r15
1405a0c7e: e8 9d 0f fd ff              	call	0x140571c20 <.text+0x570c20>
1405a0c83: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0c8a: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0c91: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0c98: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0c9f: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0ca6: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0cad: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0cb4: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0cbb: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0cc2: 48 ba 79 a4 46 ed 75 a1 31 cf       	movabs	rdx, -0x30ce5e8a12b95b87
1405a0ccc: 49 b8 e1 39 d1 d1 ad ed d3 7e       	movabs	r8, 0x7ed3edadd1d139e1
1405a0cd6: 4c 89 e1                    	mov	rcx, r12
1405a0cd9: e8 12 ca b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a0cde: 48 85 c0                    	test	rax, rax
1405a0ce1: 74 33                       	je	0x1405a0d16 <.text+0x59fd16>
1405a0ce3: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a0cea: 76 2a                       	jbe	0x1405a0d16 <.text+0x59fd16>
1405a0cec: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a0cf3: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a0cf7: 48 85 c0                    	test	rax, rax
1405a0cfa: 74 1a                       	je	0x1405a0d16 <.text+0x59fd16>
1405a0cfc: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a0d03: 48 f7 d0                    	not	rax
1405a0d06: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a0d0a: 48 c1 e0 05                 	shl	rax, 0x5
1405a0d0e: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a0d14: 75 47                       	jne	0x1405a0d5d <.text+0x59fd5d>
1405a0d16: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0d1d: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0d24: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0d2b: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0d32: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0d39: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0d40: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0d47: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0d4e: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0d55: 4c 89 f9                    	mov	rcx, r15
1405a0d58: e8 b3 56 fe ff              	call	0x140586410 <.text+0x585410>
1405a0d5d: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0d64: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0d6b: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0d72: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0d79: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0d80: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0d87: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0d8e: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0d95: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0d9c: 48 ba 98 f4 c7 95 7f d6 94 23       	movabs	rdx, 0x2394d67f95c7f498
1405a0da6: 49 b8 c6 9c a7 8a 25 29 01 80       	movabs	r8, -0x7ffed6da7558633a
1405a0db0: 4c 89 e1                    	mov	rcx, r12
1405a0db3: e8 38 c9 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a0db8: 48 85 c0                    	test	rax, rax
1405a0dbb: 74 33                       	je	0x1405a0df0 <.text+0x59fdf0>
1405a0dbd: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a0dc4: 76 2a                       	jbe	0x1405a0df0 <.text+0x59fdf0>
1405a0dc6: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a0dcd: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a0dd1: 48 85 c0                    	test	rax, rax
1405a0dd4: 74 1a                       	je	0x1405a0df0 <.text+0x59fdf0>
1405a0dd6: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a0ddd: 48 f7 d0                    	not	rax
1405a0de0: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a0de4: 48 c1 e0 05                 	shl	rax, 0x5
1405a0de8: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a0dee: 75 47                       	jne	0x1405a0e37 <.text+0x59fe37>
1405a0df0: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0df7: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0dfe: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0e05: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0e0c: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0e13: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0e1a: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0e21: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0e28: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0e2f: 4c 89 f9                    	mov	rcx, r15
1405a0e32: e8 29 92 fc ff              	call	0x14056a060 <.text+0x569060>
1405a0e37: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0e3e: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0e45: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0e4c: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0e53: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0e5a: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0e61: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0e68: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0e6f: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0e76: 48 ba 4f 6c 2b 7f 9a 6c de d1       	movabs	rdx, -0x2e21936580d493b1
1405a0e80: 49 b8 1e a5 2b ea 8f d1 73 f0       	movabs	r8, -0xf8c2e7015d45ae2
1405a0e8a: 4c 89 e1                    	mov	rcx, r12
1405a0e8d: e8 5e c8 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a0e92: 48 85 c0                    	test	rax, rax
1405a0e95: 74 33                       	je	0x1405a0eca <.text+0x59feca>
1405a0e97: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a0e9e: 76 2a                       	jbe	0x1405a0eca <.text+0x59feca>
1405a0ea0: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a0ea7: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a0eab: 48 85 c0                    	test	rax, rax
1405a0eae: 74 1a                       	je	0x1405a0eca <.text+0x59feca>
1405a0eb0: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a0eb7: 48 f7 d0                    	not	rax
1405a0eba: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a0ebe: 48 c1 e0 05                 	shl	rax, 0x5
1405a0ec2: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a0ec8: 75 47                       	jne	0x1405a0f11 <.text+0x59ff11>
1405a0eca: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0ed1: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0ed8: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0edf: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0ee6: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0eed: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0ef4: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0efb: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0f02: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0f09: 4c 89 f9                    	mov	rcx, r15
1405a0f0c: e8 cf 4e fe ff              	call	0x140585de0 <.text+0x584de0>
1405a0f11: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0f18: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0f1f: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0f26: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0f2d: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0f34: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0f3b: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0f42: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0f49: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0f50: 48 ba 80 0d 28 0f 49 d2 f0 b4       	movabs	rdx, -0x4b0f2db6f0d7f280
1405a0f5a: 49 b8 fe 02 26 f1 29 93 44 f8       	movabs	r8, -0x7bb6cd60ed9fd02
1405a0f64: 4c 89 e1                    	mov	rcx, r12
1405a0f67: e8 84 c7 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a0f6c: 48 85 c0                    	test	rax, rax
1405a0f6f: 74 33                       	je	0x1405a0fa4 <.text+0x59ffa4>
1405a0f71: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a0f78: 76 2a                       	jbe	0x1405a0fa4 <.text+0x59ffa4>
1405a0f7a: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a0f81: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a0f85: 48 85 c0                    	test	rax, rax
1405a0f88: 74 1a                       	je	0x1405a0fa4 <.text+0x59ffa4>
1405a0f8a: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a0f91: 48 f7 d0                    	not	rax
1405a0f94: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a0f98: 48 c1 e0 05                 	shl	rax, 0x5
1405a0f9c: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a0fa2: 75 47                       	jne	0x1405a0feb <.text+0x59ffeb>
1405a0fa4: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0fab: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0fb2: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a0fb9: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0fc0: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a0fc7: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a0fce: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a0fd5: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a0fdc: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a0fe3: 4c 89 f9                    	mov	rcx, r15
1405a0fe6: e8 c5 58 fc ff              	call	0x1405668b0 <.text+0x5658b0>
1405a0feb: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a0ff2: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0ff9: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1000: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1007: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a100e: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1015: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a101c: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1023: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a102a: 48 ba a0 1c bf 3b bb 4f 55 0a       	movabs	rdx, 0xa554fbb3bbf1ca0
1405a1034: 49 b8 9d 8e bb 86 a9 93 b9 29       	movabs	r8, 0x29b993a986bb8e9d
1405a103e: 4c 89 e1                    	mov	rcx, r12
1405a1041: e8 aa c6 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a1046: 48 85 c0                    	test	rax, rax
1405a1049: 74 33                       	je	0x1405a107e <.text+0x5a007e>
1405a104b: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a1052: 76 2a                       	jbe	0x1405a107e <.text+0x5a007e>
1405a1054: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a105b: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a105f: 48 85 c0                    	test	rax, rax
1405a1062: 74 1a                       	je	0x1405a107e <.text+0x5a007e>
1405a1064: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a106b: 48 f7 d0                    	not	rax
1405a106e: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a1072: 48 c1 e0 05                 	shl	rax, 0x5
1405a1076: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a107c: 75 47                       	jne	0x1405a10c5 <.text+0x5a00c5>
1405a107e: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1085: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a108c: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1093: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a109a: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a10a1: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a10a8: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a10af: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a10b6: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a10bd: 4c 89 f9                    	mov	rcx, r15
1405a10c0: e8 1b 87 fd ff              	call	0x1405797e0 <.text+0x5787e0>
1405a10c5: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a10cc: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a10d3: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a10da: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a10e1: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a10e8: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a10ef: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a10f6: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a10fd: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1104: 48 ba 59 d0 38 a8 52 66 5e db       	movabs	rdx, -0x24a199ad57c72fa7
1405a110e: 49 b8 f1 b9 65 22 60 ea 0a 82       	movabs	r8, -0x7df5159fdd9a460f
1405a1118: 4c 89 e1                    	mov	rcx, r12
1405a111b: e8 d0 c5 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a1120: 48 85 c0                    	test	rax, rax
1405a1123: 74 33                       	je	0x1405a1158 <.text+0x5a0158>
1405a1125: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a112c: 76 2a                       	jbe	0x1405a1158 <.text+0x5a0158>
1405a112e: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a1135: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a1139: 48 85 c0                    	test	rax, rax
1405a113c: 74 1a                       	je	0x1405a1158 <.text+0x5a0158>
1405a113e: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a1145: 48 f7 d0                    	not	rax
1405a1148: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a114c: 48 c1 e0 05                 	shl	rax, 0x5
1405a1150: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a1156: 75 47                       	jne	0x1405a119f <.text+0x5a019f>
1405a1158: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a115f: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1166: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a116d: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1174: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a117b: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1182: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1189: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1190: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1197: 4c 89 f9                    	mov	rcx, r15
1405a119a: e8 51 ca fd ff              	call	0x14057dbf0 <.text+0x57cbf0>
1405a119f: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a11a6: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a11ad: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a11b4: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a11bb: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a11c2: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a11c9: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a11d0: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a11d7: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a11de: 48 ba 9b 40 f7 8f c5 fe 04 3a       	movabs	rdx, 0x3a04fec58ff7409b
1405a11e8: 49 b8 a2 03 3e cb c1 89 f6 cc       	movabs	r8, -0x3309763e34c1fc5e
1405a11f2: 4c 89 e1                    	mov	rcx, r12
1405a11f5: e8 f6 c4 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a11fa: 48 85 c0                    	test	rax, rax
1405a11fd: 74 33                       	je	0x1405a1232 <.text+0x5a0232>
1405a11ff: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a1206: 76 2a                       	jbe	0x1405a1232 <.text+0x5a0232>
1405a1208: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a120f: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a1213: 48 85 c0                    	test	rax, rax
1405a1216: 74 1a                       	je	0x1405a1232 <.text+0x5a0232>
1405a1218: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a121f: 48 f7 d0                    	not	rax
1405a1222: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a1226: 48 c1 e0 05                 	shl	rax, 0x5
1405a122a: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a1230: 75 47                       	jne	0x1405a1279 <.text+0x5a0279>
1405a1232: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1239: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1240: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1247: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a124e: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1255: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a125c: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1263: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a126a: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1271: 4c 89 f9                    	mov	rcx, r15
1405a1274: e8 57 07 fe ff              	call	0x1405819d0 <.text+0x5809d0>
1405a1279: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1280: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1287: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a128e: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1295: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a129c: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a12a3: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a12aa: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a12b1: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a12b8: 48 ba 39 e7 3b 51 94 6e 02 2c       	movabs	rdx, 0x2c026e94513be739
1405a12c2: 49 b8 87 69 4c 4f b0 89 54 a0       	movabs	r8, -0x5fab764fb0b39679
1405a12cc: 4c 89 e1                    	mov	rcx, r12
1405a12cf: e8 1c c4 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a12d4: 48 85 c0                    	test	rax, rax
1405a12d7: 74 33                       	je	0x1405a130c <.text+0x5a030c>
1405a12d9: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a12e0: 76 2a                       	jbe	0x1405a130c <.text+0x5a030c>
1405a12e2: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a12e9: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a12ed: 48 85 c0                    	test	rax, rax
1405a12f0: 74 1a                       	je	0x1405a130c <.text+0x5a030c>
1405a12f2: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a12f9: 48 f7 d0                    	not	rax
1405a12fc: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a1300: 48 c1 e0 05                 	shl	rax, 0x5
1405a1304: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a130a: 75 47                       	jne	0x1405a1353 <.text+0x5a0353>
1405a130c: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1313: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a131a: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1321: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1328: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a132f: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1336: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a133d: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1344: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a134b: 4c 89 f9                    	mov	rcx, r15
1405a134e: e8 6d c2 fd ff              	call	0x14057d5c0 <.text+0x57c5c0>
1405a1353: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a135a: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1361: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1368: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a136f: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1376: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a137d: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1384: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a138b: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1392: 48 ba d8 3d 6e f3 06 09 77 3b       	movabs	rdx, 0x3b770906f36e3dd8
1405a139c: 49 b8 ee 18 ce 4b 0c 87 ce 1b       	movabs	r8, 0x1bce870c4bce18ee
1405a13a6: 4c 89 e1                    	mov	rcx, r12
1405a13a9: e8 42 c3 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a13ae: 48 85 c0                    	test	rax, rax
1405a13b1: 74 33                       	je	0x1405a13e6 <.text+0x5a03e6>
1405a13b3: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
1405a13ba: 76 2a                       	jbe	0x1405a13e6 <.text+0x5a03e6>
1405a13bc: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
1405a13c3: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a13c7: 48 85 c0                    	test	rax, rax
1405a13ca: 74 1a                       	je	0x1405a13e6 <.text+0x5a03e6>
1405a13cc: 49 8b 8f e0 01 00 00        	mov	rcx, qword ptr [r15 + 0x1e0]
1405a13d3: 48 f7 d0                    	not	rax
1405a13d6: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a13da: 48 c1 e0 05                 	shl	rax, 0x5
1405a13de: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a13e4: 75 47                       	jne	0x1405a142d <.text+0x5a042d>
1405a13e6: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a13ed: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a13f4: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a13fb: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1402: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1409: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1410: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1417: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a141e: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1425: 4c 89 f9                    	mov	rcx, r15
1405a1428: e8 53 da fd ff              	call	0x14057ee80 <.text+0x57de80>
1405a142d: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1434: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a143b: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1442: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1449: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1450: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1457: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a145e: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1465: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a146c: 4c 89 f9                    	mov	rcx, r15
1405a146f: b2 02                       	mov	dl, 0x2
1405a1471: e8 4a 81 b8 ff              	call	0x1401295c0 <.text+0x1285c0>
1405a1476: 48 89 c6                    	mov	rsi, rax
1405a1479: 48 c7 83 40 04 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x440], 0x0
1405a1484: 0f 57 c0                    	xorps	xmm0, xmm0
1405a1487: 0f 11 83 50 04 00 00        	movups	xmmword ptr [rbx + 0x450], xmm0
1405a148e: 48 c7 83 48 04 00 00 08 00 00 00    	mov	qword ptr [rbx + 0x448], 0x8
1405a1499: e8 a2 c2 7a 00              	call	0x140d4d740 <.text+0xd4c740>
1405a149e: 48 89 c7                    	mov	rdi, rax
1405a14a1: f6 00 01                    	test	byte ptr [rax], 0x1
1405a14a4: 74 11                       	je	0x1405a14b7 <.text+0x5a04b7>
1405a14a6: 48 89 f9                    	mov	rcx, rdi
1405a14a9: 48 83 c1 08                 	add	rcx, 0x8
1405a14ad: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1405a14b1: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1405a14b5: eb 1b                       	jmp	0x1405a14d2 <.text+0x5a04d2>
1405a14b7: e8 44 39 7c 00              	call	0x140d64e00 <.text+0xd63e00>
1405a14bc: 48 89 f9                    	mov	rcx, rdi
1405a14bf: 48 83 c1 08                 	add	rcx, 0x8
1405a14c3: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
1405a14ca: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1405a14ce: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
1405a14d2: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1405a14d6: 4c 89 01                    	mov	qword ptr [rcx], r8
1405a14d9: 0f 10 83 40 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x440]
1405a14e0: 0f 10 8b 50 04 00 00        	movups	xmm1, xmmword ptr [rbx + 0x450]
1405a14e7: 0f 29 8b 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm1
1405a14ee: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
1405a14f5: 0f 29 bb 20 0b 00 00        	movaps	xmmword ptr [rbx + 0xb20], xmm7
1405a14fc: 0f 29 b3 30 0b 00 00        	movaps	xmmword ptr [rbx + 0xb30], xmm6
1405a1503: 48 89 83 40 0b 00 00        	mov	qword ptr [rbx + 0xb40], rax
1405a150a: 48 89 93 48 0b 00 00        	mov	qword ptr [rbx + 0xb48], rdx
1405a1511: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1518: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a151f: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1526: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a152d: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1534: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a153b: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1542: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1549: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1550: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a1557: 48 89 f1                    	mov	rcx, rsi
1405a155a: e8 d1 70 b7 ff              	call	0x140118630 <.text+0x117630>
1405a155f: 48 89 c6                    	mov	rsi, rax
1405a1562: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1569: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1570: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1577: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a157e: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1585: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a158c: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1593: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a159a: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a15a1: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a15a8: e8 83 55 4e 00              	call	0x140a86b30 <.text+0xa85b30>
1405a15ad: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a15b4: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a15bb: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a15c2: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a15c9: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a15d0: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a15d7: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a15de: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a15e5: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a15ec: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a15f3: 48 89 f1                    	mov	rcx, rsi
1405a15f6: e8 f5 6a b7 ff              	call	0x1401180f0 <.text+0x1170f0>
1405a15fb: 48 89 c6                    	mov	rsi, rax
1405a15fe: e8 3d c1 7a 00              	call	0x140d4d740 <.text+0xd4c740>
1405a1603: 48 89 c7                    	mov	rdi, rax
1405a1606: f6 00 01                    	test	byte ptr [rax], 0x1
1405a1609: 74 11                       	je	0x1405a161c <.text+0x5a061c>
1405a160b: 48 89 f9                    	mov	rcx, rdi
1405a160e: 48 83 c1 08                 	add	rcx, 0x8
1405a1612: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1405a1616: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1405a161a: eb 5a                       	jmp	0x1405a1676 <.text+0x5a0676>
1405a161c: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1623: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a162a: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1631: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1638: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a163f: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1646: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a164d: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1654: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a165b: e8 a0 37 7c 00              	call	0x140d64e00 <.text+0xd63e00>
1405a1660: 48 89 f9                    	mov	rcx, rdi
1405a1663: 48 83 c1 08                 	add	rcx, 0x8
1405a1667: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
1405a166e: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1405a1672: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
1405a1676: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1405a167a: 4c 89 01                    	mov	qword ptr [rcx], r8
1405a167d: 0f 29 b3 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm6
1405a1684: 0f 29 bb 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm7
1405a168b: 48 89 83 20 0b 00 00        	mov	qword ptr [rbx + 0xb20], rax
1405a1692: 48 89 93 28 0b 00 00        	mov	qword ptr [rbx + 0xb28], rdx
1405a1699: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a16a0: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a16a7: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a16ae: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a16b5: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a16bc: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a16c3: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a16ca: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a16d1: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a16d8: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a16df: 48 89 f1                    	mov	rcx, rsi
1405a16e2: e8 89 fa b8 ff              	call	0x140131170 <.text+0x130170>
1405a16e7: 48 89 c6                    	mov	rsi, rax
1405a16ea: e8 51 c0 7a 00              	call	0x140d4d740 <.text+0xd4c740>
1405a16ef: 48 89 c7                    	mov	rdi, rax
1405a16f2: f6 00 01                    	test	byte ptr [rax], 0x1
1405a16f5: 74 11                       	je	0x1405a1708 <.text+0x5a0708>
1405a16f7: 48 89 f9                    	mov	rcx, rdi
1405a16fa: 48 83 c1 08                 	add	rcx, 0x8
1405a16fe: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1405a1702: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1405a1706: eb 5a                       	jmp	0x1405a1762 <.text+0x5a0762>
1405a1708: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a170f: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1716: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a171d: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1724: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a172b: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1732: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1739: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1740: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1747: e8 b4 36 7c 00              	call	0x140d64e00 <.text+0xd63e00>
1405a174c: 48 89 f9                    	mov	rcx, rdi
1405a174f: 48 83 c1 08                 	add	rcx, 0x8
1405a1753: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
1405a175a: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1405a175e: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
1405a1762: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1405a1766: 4c 89 01                    	mov	qword ptr [rcx], r8
1405a1769: 0f 29 b3 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm6
1405a1770: 0f 29 bb 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm7
1405a1777: 48 89 83 20 0b 00 00        	mov	qword ptr [rbx + 0xb20], rax
1405a177e: 48 89 93 28 0b 00 00        	mov	qword ptr [rbx + 0xb28], rdx
1405a1785: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a178c: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1793: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a179a: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a17a1: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a17a8: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a17af: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a17b6: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a17bd: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a17c4: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a17cb: 48 89 f1                    	mov	rcx, rsi
1405a17ce: e8 6d da b7 ff              	call	0x14011f240 <.text+0x11e240>
1405a17d3: 48 89 c6                    	mov	rsi, rax
1405a17d6: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a17dd: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a17e4: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a17eb: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a17f2: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a17f9: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1800: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1807: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a180e: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1815: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a181c: e8 2f 74 55 00              	call	0x140af8c50 <.text+0xaf7c50>
1405a1821: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1828: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a182f: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1836: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a183d: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1844: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a184b: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1852: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1859: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1860: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a1867: 48 89 f1                    	mov	rcx, rsi
1405a186a: e8 81 e0 b9 ff              	call	0x14013f8f0 <.text+0x13e8f0>
1405a186f: 48 89 c6                    	mov	rsi, rax
1405a1872: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1879: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1880: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1887: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a188e: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1895: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a189c: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a18a3: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a18aa: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a18b1: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a18b8: e8 83 09 32 00              	call	0x1408c2240 <.text+0x8c1240>
1405a18bd: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a18c4: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a18cb: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a18d2: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a18d9: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a18e0: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a18e7: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a18ee: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a18f5: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a18fc: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a1903: 48 89 f1                    	mov	rcx, rsi
1405a1906: e8 a5 bd b7 ff              	call	0x14011d6b0 <.text+0x11c6b0>
1405a190b: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1912: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1919: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1920: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1927: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a192e: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1935: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a193c: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1943: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a194a: 48 89 c1                    	mov	rcx, rax
1405a194d: 31 d2                       	xor	edx, edx
1405a194f: e8 bc af b8 ff              	call	0x14012c910 <.text+0x12b910>
1405a1954: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a195b: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1962: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1969: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1970: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1977: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a197e: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1985: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a198c: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1993: 48 89 c1                    	mov	rcx, rax
1405a1996: 31 d2                       	xor	edx, edx
1405a1998: e8 83 9b b7 ff              	call	0x14011b520 <.text+0x11a520>
1405a199d: 48 89 c6                    	mov	rsi, rax
1405a19a0: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a19a7: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a19ae: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a19b5: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a19bc: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a19c3: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a19ca: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a19d1: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a19d8: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a19df: e8 dc f4 f0 ff              	call	0x1404b0ec0 <.text+0x4afec0>
1405a19e4: 41 89 d0                    	mov	r8d, edx
1405a19e7: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a19ee: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a19f5: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a19fc: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1a03: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1a0a: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1a11: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1a18: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1a1f: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1a26: 24 01                       	and	al, 0x1
1405a1a28: 41 80 e0 01                 	and	r8b, 0x1
1405a1a2c: 48 89 f1                    	mov	rcx, rsi
1405a1a2f: 89 c2                       	mov	edx, eax
1405a1a31: e8 ba d2 b7 ff              	call	0x14011ecf0 <.text+0x11dcf0>
1405a1a36: 48 c7 83 05 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb05], 0x0
1405a1a41: 48 c7 83 00 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb00], 0x0
1405a1a4c: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1a53: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1a5a: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1a61: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1a68: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1a6f: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1a76: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1a7d: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1a84: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1a8b: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a1a92: 48 89 c1                    	mov	rcx, rax
1405a1a95: e8 c6 ce b8 ff              	call	0x14012e960 <.text+0x12d960>
1405a1a9a: 48 89 c6                    	mov	rsi, rax
1405a1a9d: e8 9e bc 7a 00              	call	0x140d4d740 <.text+0xd4c740>
1405a1aa2: 48 89 c7                    	mov	rdi, rax
1405a1aa5: f6 00 01                    	test	byte ptr [rax], 0x1
1405a1aa8: 74 11                       	je	0x1405a1abb <.text+0x5a0abb>
1405a1aaa: 48 89 f9                    	mov	rcx, rdi
1405a1aad: 48 83 c1 08                 	add	rcx, 0x8
1405a1ab1: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1405a1ab5: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1405a1ab9: eb 5a                       	jmp	0x1405a1b15 <.text+0x5a0b15>
1405a1abb: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1ac2: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1ac9: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1ad0: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1ad7: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1ade: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1ae5: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1aec: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1af3: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1afa: e8 01 33 7c 00              	call	0x140d64e00 <.text+0xd63e00>
1405a1aff: 48 89 f9                    	mov	rcx, rdi
1405a1b02: 48 83 c1 08                 	add	rcx, 0x8
1405a1b06: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
1405a1b0d: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1405a1b11: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
1405a1b15: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1405a1b19: 4c 89 01                    	mov	qword ptr [rcx], r8
1405a1b1c: 0f 29 b3 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm6
1405a1b23: 0f 29 bb 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm7
1405a1b2a: 48 89 83 20 0b 00 00        	mov	qword ptr [rbx + 0xb20], rax
1405a1b31: 48 89 93 28 0b 00 00        	mov	qword ptr [rbx + 0xb28], rdx
1405a1b38: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1b3f: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1b46: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1b4d: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1b54: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1b5b: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1b62: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1b69: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1b70: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1b77: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a1b7e: 48 89 f1                    	mov	rcx, rsi
1405a1b81: e8 6a 57 b8 ff              	call	0x1401272f0 <.text+0x1262f0>
1405a1b86: 48 89 c6                    	mov	rsi, rax
1405a1b89: e8 b2 bb 7a 00              	call	0x140d4d740 <.text+0xd4c740>
1405a1b8e: 48 89 c7                    	mov	rdi, rax
1405a1b91: f6 00 01                    	test	byte ptr [rax], 0x1
1405a1b94: 74 11                       	je	0x1405a1ba7 <.text+0x5a0ba7>
1405a1b96: 48 89 f9                    	mov	rcx, rdi
1405a1b99: 48 83 c1 08                 	add	rcx, 0x8
1405a1b9d: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1405a1ba1: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1405a1ba5: eb 5a                       	jmp	0x1405a1c01 <.text+0x5a0c01>
1405a1ba7: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1bae: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1bb5: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1bbc: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1bc3: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1bca: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1bd1: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1bd8: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1bdf: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1be6: e8 15 32 7c 00              	call	0x140d64e00 <.text+0xd63e00>
1405a1beb: 48 89 f9                    	mov	rcx, rdi
1405a1bee: 48 83 c1 08                 	add	rcx, 0x8
1405a1bf2: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
1405a1bf9: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1405a1bfd: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
1405a1c01: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1405a1c05: 4c 89 01                    	mov	qword ptr [rcx], r8
1405a1c08: 0f 29 b3 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm6
1405a1c0f: 0f 29 bb 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm7
1405a1c16: 48 89 83 20 0b 00 00        	mov	qword ptr [rbx + 0xb20], rax
1405a1c1d: 48 89 93 28 0b 00 00        	mov	qword ptr [rbx + 0xb28], rdx
1405a1c24: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1c2b: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1c32: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1c39: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1c40: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1c47: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1c4e: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1c55: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1c5c: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1c63: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a1c6a: 48 89 f1                    	mov	rcx, rsi
1405a1c6d: e8 3e 90 b8 ff              	call	0x14012acb0 <.text+0x129cb0>
1405a1c72: 48 89 c6                    	mov	rsi, rax
1405a1c75: e8 c6 ba 7a 00              	call	0x140d4d740 <.text+0xd4c740>
1405a1c7a: 48 89 c7                    	mov	rdi, rax
1405a1c7d: f6 00 01                    	test	byte ptr [rax], 0x1
1405a1c80: 74 11                       	je	0x1405a1c93 <.text+0x5a0c93>
1405a1c82: 48 89 f9                    	mov	rcx, rdi
1405a1c85: 48 83 c1 08                 	add	rcx, 0x8
1405a1c89: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1405a1c8d: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1405a1c91: eb 5a                       	jmp	0x1405a1ced <.text+0x5a0ced>
1405a1c93: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1c9a: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1ca1: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1ca8: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1caf: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1cb6: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1cbd: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1cc4: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1ccb: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1cd2: e8 29 31 7c 00              	call	0x140d64e00 <.text+0xd63e00>
1405a1cd7: 48 89 f9                    	mov	rcx, rdi
1405a1cda: 48 83 c1 08                 	add	rcx, 0x8
1405a1cde: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
1405a1ce5: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1405a1ce9: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
1405a1ced: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1405a1cf1: 4c 89 01                    	mov	qword ptr [rcx], r8
1405a1cf4: 0f 29 b3 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm6
1405a1cfb: 0f 29 bb 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm7
1405a1d02: 48 89 83 20 0b 00 00        	mov	qword ptr [rbx + 0xb20], rax
1405a1d09: 48 89 93 28 0b 00 00        	mov	qword ptr [rbx + 0xb28], rdx
1405a1d10: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1d17: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1d1e: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1d25: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1d2c: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1d33: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1d3a: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1d41: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1d48: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1d4f: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a1d56: 48 89 f1                    	mov	rcx, rsi
1405a1d59: e8 32 d7 b8 ff              	call	0x14012f490 <.text+0x12e490>
1405a1d5e: 48 89 c6                    	mov	rsi, rax
1405a1d61: e8 da b9 7a 00              	call	0x140d4d740 <.text+0xd4c740>
1405a1d66: 48 89 c7                    	mov	rdi, rax
1405a1d69: f6 00 01                    	test	byte ptr [rax], 0x1
1405a1d6c: 74 11                       	je	0x1405a1d7f <.text+0x5a0d7f>
1405a1d6e: 48 89 f9                    	mov	rcx, rdi
1405a1d71: 48 83 c1 08                 	add	rcx, 0x8
1405a1d75: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1405a1d79: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1405a1d7d: eb 5a                       	jmp	0x1405a1dd9 <.text+0x5a0dd9>
1405a1d7f: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1d86: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1d8d: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1d94: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1d9b: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1da2: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1da9: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1db0: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1db7: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1dbe: e8 3d 30 7c 00              	call	0x140d64e00 <.text+0xd63e00>
1405a1dc3: 48 89 f9                    	mov	rcx, rdi
1405a1dc6: 48 83 c1 08                 	add	rcx, 0x8
1405a1dca: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
1405a1dd1: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1405a1dd5: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
1405a1dd9: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1405a1ddd: 4c 89 01                    	mov	qword ptr [rcx], r8
1405a1de0: 0f 29 b3 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm6
1405a1de7: 0f 29 bb 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm7
1405a1dee: 48 89 83 20 0b 00 00        	mov	qword ptr [rbx + 0xb20], rax
1405a1df5: 48 89 93 28 0b 00 00        	mov	qword ptr [rbx + 0xb28], rdx
1405a1dfc: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1e03: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1e0a: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1e11: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1e18: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1e1f: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1e26: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1e2d: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1e34: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1e3b: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a1e42: 48 89 f1                    	mov	rcx, rsi
1405a1e45: e8 76 9f b8 ff              	call	0x14012bdc0 <.text+0x12adc0>
1405a1e4a: 48 89 c6                    	mov	rsi, rax
1405a1e4d: e8 ee b8 7a 00              	call	0x140d4d740 <.text+0xd4c740>
1405a1e52: 48 89 c7                    	mov	rdi, rax
1405a1e55: f6 00 01                    	test	byte ptr [rax], 0x1
1405a1e58: 74 11                       	je	0x1405a1e6b <.text+0x5a0e6b>
1405a1e5a: 48 89 f9                    	mov	rcx, rdi
1405a1e5d: 48 83 c1 08                 	add	rcx, 0x8
1405a1e61: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1405a1e65: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1405a1e69: eb 5a                       	jmp	0x1405a1ec5 <.text+0x5a0ec5>
1405a1e6b: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1e72: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1e79: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1e80: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1e87: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1e8e: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1e95: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1e9c: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1ea3: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1eaa: e8 51 2f 7c 00              	call	0x140d64e00 <.text+0xd63e00>
1405a1eaf: 48 89 f9                    	mov	rcx, rdi
1405a1eb2: 48 83 c1 08                 	add	rcx, 0x8
1405a1eb6: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
1405a1ebd: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1405a1ec1: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
1405a1ec5: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1405a1ec9: 4c 89 01                    	mov	qword ptr [rcx], r8
1405a1ecc: 0f 29 b3 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm6
1405a1ed3: 0f 29 bb 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm7
1405a1eda: 48 89 83 20 0b 00 00        	mov	qword ptr [rbx + 0xb20], rax
1405a1ee1: 48 89 93 28 0b 00 00        	mov	qword ptr [rbx + 0xb28], rdx
1405a1ee8: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1eef: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1ef6: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1efd: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1f04: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1f0b: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1f12: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1f19: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1f20: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1f27: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a1f2e: 48 89 f1                    	mov	rcx, rsi
1405a1f31: e8 da 30 b8 ff              	call	0x140125010 <.text+0x124010>
1405a1f36: 48 89 c6                    	mov	rsi, rax
1405a1f39: e8 02 b8 7a 00              	call	0x140d4d740 <.text+0xd4c740>
1405a1f3e: 48 89 c7                    	mov	rdi, rax
1405a1f41: f6 00 01                    	test	byte ptr [rax], 0x1
1405a1f44: 74 11                       	je	0x1405a1f57 <.text+0x5a0f57>
1405a1f46: 48 89 f9                    	mov	rcx, rdi
1405a1f49: 48 83 c1 08                 	add	rcx, 0x8
1405a1f4d: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1405a1f51: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1405a1f55: eb 5a                       	jmp	0x1405a1fb1 <.text+0x5a0fb1>
1405a1f57: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a1f5e: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a1f65: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a1f6c: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a1f73: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a1f7a: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a1f81: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a1f88: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a1f8f: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a1f96: e8 65 2e 7c 00              	call	0x140d64e00 <.text+0xd63e00>
1405a1f9b: 48 89 f9                    	mov	rcx, rdi
1405a1f9e: 48 83 c1 08                 	add	rcx, 0x8
1405a1fa2: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
1405a1fa9: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1405a1fad: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
1405a1fb1: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1405a1fb5: 4c 89 01                    	mov	qword ptr [rcx], r8
1405a1fb8: 0f 29 b3 50 04 00 00        	movaps	xmmword ptr [rbx + 0x450], xmm6
1405a1fbf: 0f 29 bb 40 04 00 00        	movaps	xmmword ptr [rbx + 0x440], xmm7
1405a1fc6: 48 89 83 60 04 00 00        	mov	qword ptr [rbx + 0x460], rax
1405a1fcd: 48 89 93 68 04 00 00        	mov	qword ptr [rbx + 0x468], rdx
1405a1fd4: e8 67 b7 7a 00              	call	0x140d4d740 <.text+0xd4c740>
1405a1fd9: 48 89 c7                    	mov	rdi, rax
1405a1fdc: f6 00 01                    	test	byte ptr [rax], 0x1
1405a1fdf: 74 11                       	je	0x1405a1ff2 <.text+0x5a0ff2>
1405a1fe1: 48 89 f9                    	mov	rcx, rdi
1405a1fe4: 48 83 c1 08                 	add	rcx, 0x8
1405a1fe8: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1405a1fec: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1405a1ff0: eb 1b                       	jmp	0x1405a200d <.text+0x5a100d>
1405a1ff2: e8 09 2e 7c 00              	call	0x140d64e00 <.text+0xd63e00>
1405a1ff7: 48 89 f9                    	mov	rcx, rdi
1405a1ffa: 48 83 c1 08                 	add	rcx, 0x8
1405a1ffe: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
1405a2005: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1405a2009: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
1405a200d: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1405a2011: 4c 89 01                    	mov	qword ptr [rcx], r8
1405a2014: 0f 28 83 40 04 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x440]
1405a201b: 0f 28 8b 50 04 00 00        	movaps	xmm1, xmmword ptr [rbx + 0x450]
1405a2022: 0f 28 93 60 04 00 00        	movaps	xmm2, xmmword ptr [rbx + 0x460]
1405a2029: 0f 29 93 20 0b 00 00        	movaps	xmmword ptr [rbx + 0xb20], xmm2
1405a2030: 0f 29 8b 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm1
1405a2037: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
1405a203e: 0f 29 bb 30 0b 00 00        	movaps	xmmword ptr [rbx + 0xb30], xmm7
1405a2045: 0f 29 b3 40 0b 00 00        	movaps	xmmword ptr [rbx + 0xb40], xmm6
1405a204c: 48 89 83 50 0b 00 00        	mov	qword ptr [rbx + 0xb50], rax
1405a2053: 48 89 93 58 0b 00 00        	mov	qword ptr [rbx + 0xb58], rdx
1405a205a: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a2061: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a2068: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a206f: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a2076: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a207d: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a2084: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a208b: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a2092: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a2099: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a20a0: 48 89 f1                    	mov	rcx, rsi
1405a20a3: e8 a8 dd b7 ff              	call	0x14011fe50 <.text+0x11ee50>
1405a20a8: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a20af: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a20b6: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a20bd: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a20c4: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a20cb: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a20d2: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a20d9: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a20e0: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a20e7: 48 89 c1                    	mov	rcx, rax
1405a20ea: e8 81 f7 0a 00              	call	0x140651870 <.text+0x650870>
1405a20ef: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a20f6: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a20fd: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a2104: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a210b: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a2112: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a2119: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a2120: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a2127: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a212e: 48 89 c1                    	mov	rcx, rax
1405a2131: e8 ea e8 08 00              	call	0x140630a20 <.text+0x62fa20>
1405a2136: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a213d: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a2144: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a214b: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a2152: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a2159: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a2160: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a2167: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a216e: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a2175: 48 89 c1                    	mov	rcx, rax
1405a2178: e8 d3 eb 0a 00              	call	0x140650d50 <.text+0x64fd50>
1405a217d: 8b 93 48 01 00 00           	mov	edx, dword ptr [rbx + 0x148]
1405a2183: 44 8b 83 4c 01 00 00        	mov	r8d, dword ptr [rbx + 0x14c]
1405a218a: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a2191: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a2198: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a219f: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a21a6: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a21ad: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a21b4: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a21bb: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a21c2: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a21c9: 48 89 c1                    	mov	rcx, rax
1405a21cc: e8 af eb 39 00              	call	0x140940d80 <.text+0x93fd80>
1405a21d1: 48 89 c6                    	mov	rsi, rax
1405a21d4: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a21db: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a21e2: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a21e9: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a21f0: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a21f7: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a21fe: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a2205: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a220c: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a2213: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a221a: 48 8b 95 b0 37 00 00        	mov	rdx, qword ptr [rbp + 0x37b0]
1405a2221: e8 ea 12 ff ff              	call	0x140593510 <.text+0x592510>
1405a2226: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a222d: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a2234: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a223b: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a2242: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a2249: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a2250: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a2257: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a225e: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a2265: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a226c: 48 89 f1                    	mov	rcx, rsi
1405a226f: e8 9c 75 b7 ff              	call	0x140119810 <.text+0x118810>
1405a2274: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a227b: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a2282: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a2289: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a2290: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a2297: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a229e: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a22a5: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a22ac: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a22b3: 48 ba 1b 6a 49 55 f3 1b 60 e3       	movabs	rdx, -0x1c9fe40caab695e5
1405a22bd: 49 b8 e3 3b f2 ce 6c 54 cc 8a       	movabs	r8, -0x7533ab93310dc41d
1405a22c7: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
1405a22ce: e8 1d b4 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a22d3: 48 85 c0                    	test	rax, rax
1405a22d6: 4c 8b 83 c8 00 00 00        	mov	r8, qword ptr [rbx + 0xc8]
1405a22dd: 0f 84 ba 01 00 00           	je	0x1405a249d <.text+0x5a149d>
1405a22e3: 49 39 90 18 02 00 00        	cmp	qword ptr [r8 + 0x218], rdx
1405a22ea: 0f 86 ad 01 00 00           	jbe	0x1405a249d <.text+0x5a149d>
1405a22f0: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
1405a22f7: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1405a22fb: 48 85 c9                    	test	rcx, rcx
1405a22fe: 0f 84 99 01 00 00           	je	0x1405a249d <.text+0x5a149d>
1405a2304: 49 8b 80 e0 01 00 00        	mov	rax, qword ptr [r8 + 0x1e0]
1405a230b: 48 f7 d1                    	not	rcx
1405a230e: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1405a2312: 48 c1 e1 05                 	shl	rcx, 0x5
1405a2316: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1405a231c: 0f 84 7b 01 00 00           	je	0x1405a249d <.text+0x5a149d>
1405a2322: 48 01 c8                    	add	rax, rcx
1405a2325: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
1405a2329: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a2330: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a2337: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a233e: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a2345: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a234c: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a2353: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a235a: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a2361: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a2368: e8 13 ba 0d 00              	call	0x14067dd80 <.text+0x67cd80>
1405a236d: 48 89 c6                    	mov	rsi, rax
1405a2370: e8 fb 11 38 02              	call	0x142923570 <.text+0x2922570>
1405a2375: 48 8b 0e                    	mov	rcx, qword ptr [rsi]
1405a2378: f0                          	lock
1405a2379: 48 ff 01                    	inc	qword ptr [rcx]
1405a237c: 0f 8e be 4c 00 00           	jle	0x1405a7040 <.text+0x5a6040>
1405a2382: 48 89 8b 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rcx
1405a2389: 48 89 83 08 0b 00 00        	mov	qword ptr [rbx + 0xb08], rax
1405a2390: 48 89 c1                    	mov	rcx, rax
1405a2393: e8 58 11 38 02              	call	0x1429234f0 <.text+0x29224f0>
1405a2398: 89 c6                       	mov	esi, eax
1405a239a: 48 8b 83 00 0b 00 00        	mov	rax, qword ptr [rbx + 0xb00]
1405a23a1: f0                          	lock
1405a23a2: 48 ff 08                    	dec	qword ptr [rax]
1405a23a5: 75 4b                       	jne	0x1405a23f2 <.text+0x5a13f2>
1405a23a7: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a23ae: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a23b5: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a23bc: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a23c3: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a23ca: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a23d1: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a23d8: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a23df: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a23e6: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a23ed: e8 5e 2f 5e 00              	call	0x140b85350 <.text+0xb84350>
1405a23f2: 40 84 f6                    	test	sil, sil
1405a23f5: 0f 84 a2 00 00 00           	je	0x1405a249d <.text+0x5a149d>
1405a23fb: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a2402: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a2409: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a2410: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a2417: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a241e: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a2425: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a242c: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a2433: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a243a: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a2441: e8 6a 6a 3b 00              	call	0x140958eb0 <.text+0x957eb0>
1405a2446: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a244d: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a2454: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a245b: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a2462: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a2469: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a2470: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a2477: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a247e: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a2485: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a248c: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a2493: e8 e8 93 b8 ff              	call	0x14012b880 <.text+0x12a880>
1405a2498: e9 9d 00 00 00              	jmp	0x1405a253a <.text+0x5a153a>
1405a249d: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a24a4: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a24ab: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a24b2: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a24b9: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a24c0: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a24c7: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a24ce: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a24d5: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a24dc: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a24e3: e8 08 6a 3b 00              	call	0x140958ef0 <.text+0x957ef0>
1405a24e8: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a24ef: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a24f6: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a24fd: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a2504: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a250b: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a2512: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a2519: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a2520: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a2527: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a252e: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a2535: e8 46 93 b8 ff              	call	0x14012b880 <.text+0x12a880>
1405a253a: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a2541: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a2548: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a254f: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a2556: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a255d: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a2564: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a256b: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a2572: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a2579: 48 8d 8b 48 01 00 00        	lea	rcx, [rbx + 0x148]
1405a2580: e8 4b 60 ae ff              	call	0x1400885d0 <.text+0x875d0>
1405a2585: 48 89 c6                    	mov	rsi, rax
1405a2588: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a258f: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a2596: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a259d: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a25a4: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a25ab: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a25b2: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a25b9: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a25c0: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a25c7: 48 8d 8b 48 01 00 00        	lea	rcx, [rbx + 0x148]
1405a25ce: e8 fd 5f ae ff              	call	0x1400885d0 <.text+0x875d0>
1405a25d3: 48 89 c7                    	mov	rdi, rax
1405a25d6: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a25dd: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a25e4: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a25eb: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a25f2: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a25f9: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a2600: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a2607: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a260e: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a2615: 48 8d 8b 48 01 00 00        	lea	rcx, [rbx + 0x148]
1405a261c: e8 af 5f ae ff              	call	0x1400885d0 <.text+0x875d0>
1405a2621: 49 89 c7                    	mov	r15, rax
1405a2624: 48 8b 85 b0 37 00 00        	mov	rax, qword ptr [rbp + 0x37b0]
1405a262b: 80 b8 88 00 00 00 00        	cmp	byte ptr [rax + 0x88], 0x0
1405a2632: f3 0f 10 05 ca 32 38 02     	movss	xmm0, dword ptr [rip + 0x23832ca] # 0x142925904
1405a263a: 75 0f                       	jne	0x1405a264b <.text+0x5a164b>
1405a263c: 44 0f 28 e0                 	movaps	xmm12, xmm0
1405a2640: 80 b8 ab 00 00 00 01        	cmp	byte ptr [rax + 0xab], 0x1
1405a2647: 74 14                       	je	0x1405a265d <.text+0x5a165d>
1405a2649: eb 5d                       	jmp	0x1405a26a8 <.text+0x5a16a8>
1405a264b: f3 44 0f 10 a0 8c 00 00 00  	movss	xmm12, dword ptr [rax + 0x8c]
1405a2654: 80 b8 ab 00 00 00 01        	cmp	byte ptr [rax + 0xab], 0x1
1405a265b: 75 4b                       	jne	0x1405a26a8 <.text+0x5a16a8>
1405a265d: 48 8d 88 c3 00 00 00        	lea	rcx, [rax + 0xc3]
1405a2664: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a266b: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a2672: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a2679: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a2680: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a2687: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a268e: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a2695: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a269c: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a26a3: e8 08 65 3c 00              	call	0x140968bb0 <.text+0x967bb0>
1405a26a8: 48 89 b3 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rsi
1405a26af: 48 89 bb 08 0b 00 00        	mov	qword ptr [rbx + 0xb08], rdi
1405a26b6: 4c 89 bb 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], r15
1405a26bd: c6 83 20 0b 00 00 00        	mov	byte ptr [rbx + 0xb20], 0x0
1405a26c4: f3 44 0f 11 a3 18 0b 00 00  	movss	dword ptr [rbx + 0xb18], xmm12
1405a26cd: f3 0f 11 83 1c 0b 00 00     	movss	dword ptr [rbx + 0xb1c], xmm0
1405a26d5: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a26dc: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a26e3: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a26ea: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a26f1: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a26f8: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a26ff: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a2706: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a270d: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a2714: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a271b: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a2722: e8 f9 bc b8 ff              	call	0x14012e420 <.text+0x12d420>
1405a2727: 48 8b 85 c0 37 00 00        	mov	rax, qword ptr [rbp + 0x37c0]
1405a272e: 0f b6 80 41 01 00 00        	movzx	eax, byte ptr [rax + 0x141]
1405a2735: 34 01                       	xor	al, 0x1
1405a2737: 88 83 28 0b 00 00           	mov	byte ptr [rbx + 0xb28], al
1405a273d: 48 c7 83 00 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb00], 0x0
1405a2748: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a274f: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a2756: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a275d: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a2764: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a276b: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a2772: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a2779: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a2780: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a2787: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a278e: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a2795: e8 56 ae b9 ff              	call	0x14013d5f0 <.text+0x13c5f0>
1405a279a: 48 8b 83 c0 02 00 00        	mov	rax, qword ptr [rbx + 0x2c0]
1405a27a1: 48 8b 93 c8 02 00 00        	mov	rdx, qword ptr [rbx + 0x2c8]
1405a27a8: 48 85 c0                    	test	rax, rax
1405a27ab: 74 5f                       	je	0x1405a280c <.text+0x5a180c>
1405a27ad: 83 f8 01                    	cmp	eax, 0x1
1405a27b0: 0f 85 a9 00 00 00           	jne	0x1405a285f <.text+0x5a185f>
1405a27b6: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a27bd: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a27c4: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a27cb: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a27d2: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a27d9: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a27e0: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a27e7: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a27ee: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a27f5: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a27fc: 41 b9 00 ca 9a 3b           	mov	r9d, 0x3b9aca00
1405a2802: e8 29 19 cc ff              	call	0x140264130 <.text+0x263130>
1405a2807: e9 a4 00 00 00              	jmp	0x1405a28b0 <.text+0x5a18b0>
1405a280c: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a2813: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a281a: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a2821: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a2828: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a282f: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a2836: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a283d: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a2844: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a284b: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a2852: 41 b9 00 ca 9a 3b           	mov	r9d, 0x3b9aca00
1405a2858: e8 e3 c0 f4 ff              	call	0x1404ee940 <.text+0x4ed940>
1405a285d: eb 51                       	jmp	0x1405a28b0 <.text+0x5a18b0>
1405a285f: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a2866: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a286d: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a2874: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a287b: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a2882: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a2889: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a2890: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a2897: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a289e: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a28a5: 41 b9 00 ca 9a 3b           	mov	r9d, 0x3b9aca00
1405a28ab: e8 c0 0c f7 ff              	call	0x140513570 <.text+0x512570>
1405a28b0: 48 8b 83 00 0b 00 00        	mov	rax, qword ptr [rbx + 0xb00]
1405a28b7: 48 83 f8 02                 	cmp	rax, 0x2
1405a28bb: 0f 84 54 44 00 00           	je	0x1405a6d15 <.text+0x5a5d15>
1405a28c1: 0f 10 83 08 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb08]
1405a28c8: 0f 10 8b 18 0b 00 00        	movups	xmm1, xmmword ptr [rbx + 0xb18]
1405a28cf: 0f 11 8b 18 03 00 00        	movups	xmmword ptr [rbx + 0x318], xmm1
1405a28d6: 0f 11 83 08 03 00 00        	movups	xmmword ptr [rbx + 0x308], xmm0
1405a28dd: 48 89 83 00 03 00 00        	mov	qword ptr [rbx + 0x300], rax
1405a28e4: 48 8b 93 20 03 00 00        	mov	rdx, qword ptr [rbx + 0x320]
1405a28eb: 48 85 d2                    	test	rdx, rdx
1405a28ee: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a28f5: 74 0a                       	je	0x1405a2901 <.text+0x5a1901>
1405a28f7: f0                          	lock
1405a28f8: 48 ff 02                    	inc	qword ptr [rdx]
1405a28fb: 0f 8e 3f 47 00 00           	jle	0x1405a7040 <.text+0x5a6040>
1405a2901: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a2908: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a290f: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a2916: c6 83 b7 00 00 00 01        	mov	byte ptr [rbx + 0xb7], 0x1
1405a291d: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a2924: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a292b: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a2932: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a2939: e8 d2 24 b7 ff              	call	0x140114e10 <.text+0x113e10>
1405a293e: 0f 10 83 00 03 00 00        	movups	xmm0, xmmword ptr [rbx + 0x300]
1405a2945: 0f 10 8b 10 03 00 00        	movups	xmm1, xmmword ptr [rbx + 0x310]
1405a294c: 0f 29 8b 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm1
1405a2953: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
1405a295a: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a2961: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a2968: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a296f: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a2976: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a297d: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a2984: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a298b: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a2992: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a2999: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a29a0: e8 fb 88 b8 ff              	call	0x14012b2a0 <.text+0x12a2a0>
1405a29a5: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a29ac: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a29b3: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a29ba: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a29c1: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a29c8: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a29cf: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a29d6: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a29dd: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a29e4: e8 c7 54 d8 ff              	call	0x140327eb0 <.text+0x326eb0>
1405a29e9: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a29f0: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a29f7: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a29fe: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a2a05: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a2a0c: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a2a13: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a2a1a: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a2a21: 48 8d 15 e0 ce 45 02        	lea	rdx, [rip + 0x245cee0]  # 0x1429ff908
1405a2a28: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a2a2f: e8 0c 09 b7 ff              	call	0x140113340 <.text+0x112340>
1405a2a34: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a2a3b: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a2a42: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a2a49: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a2a50: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a2a57: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a2a5e: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a2a65: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a2a6c: 48 8d 15 ad ce 45 02        	lea	rdx, [rip + 0x245cead]  # 0x1429ff920
1405a2a73: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a2a7a: e8 61 06 b7 ff              	call	0x1401130e0 <.text+0x1120e0>
1405a2a7f: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a2a86: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a2a8d: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a2a94: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a2a9b: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a2aa2: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a2aa9: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a2ab0: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a2ab7: 48 8d 15 7a ce 45 02        	lea	rdx, [rip + 0x245ce7a]  # 0x1429ff938
1405a2abe: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a2ac5: e8 46 07 b7 ff              	call	0x140113210 <.text+0x112210>
1405a2aca: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a2ad1: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a2ad8: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a2adf: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a2ae6: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a2aed: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a2af4: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a2afb: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a2b02: 48 8d 15 47 ce 45 02        	lea	rdx, [rip + 0x245ce47]  # 0x1429ff950
1405a2b09: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a2b10: e8 9b 04 b7 ff              	call	0x140112fb0 <.text+0x111fb0>
1405a2b15: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a2b1c: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a2b23: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a2b2a: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a2b31: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a2b38: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a2b3f: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a2b46: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a2b4d: 4c 89 f1                    	mov	rcx, r14
1405a2b50: e8 eb b3 36 00              	call	0x14090df40 <.text+0x90cf40>
1405a2b55: 40 88 c6                    	mov	sil, al
1405a2b58: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a2b5f: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a2b66: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a2b6d: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a2b74: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a2b7b: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a2b82: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a2b89: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a2b90: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a2b97: e8 54 14 b7 ff              	call	0x140113ff0 <.text+0x112ff0>
1405a2b9c: 3c 01                       	cmp	al, 0x1
1405a2b9e: 0f 87 e0 41 00 00           	ja	0x1405a6d84 <.text+0x5a5d84>
1405a2ba4: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a2bab: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a2bb2: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a2bb9: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a2bc0: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a2bc7: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a2bce: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a2bd5: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a2bdc: 4c 8d 05 85 cd 45 02        	lea	r8, [rip + 0x245cd85]   # 0x1429ff968
1405a2be3: 89 f1                       	mov	ecx, esi
1405a2be5: 48 8b 93 c8 00 00 00        	mov	rdx, qword ptr [rbx + 0xc8]
1405a2bec: e8 df 83 d8 ff              	call	0x14032afd0 <.text+0x329fd0>
1405a2bf1: 48 8b 85 b0 37 00 00        	mov	rax, qword ptr [rbp + 0x37b0]
1405a2bf8: 0f b6 80 e4 00 00 00        	movzx	eax, byte ptr [rax + 0xe4]
1405a2bff: 88 83 07 01 00 00           	mov	byte ptr [rbx + 0x107], al
1405a2c05: 31 c0                       	xor	eax, eax
1405a2c07: 48 8b 8d c0 37 00 00        	mov	rcx, qword ptr [rbp + 0x37c0]
1405a2c0e: 48 3b 81 c8 00 00 00        	cmp	rax, qword ptr [rcx + 0xc8]
1405a2c15: 0f 80 cc 04 00 00           	jo	0x1405a30e7 <.text+0x5a20e7>
1405a2c1b: 48 81 c1 c8 00 00 00        	add	rcx, 0xc8
1405a2c22: 48 89 8b 38 03 00 00        	mov	qword ptr [rbx + 0x338], rcx
1405a2c29: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a2c30: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a2c37: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a2c3e: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a2c45: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a2c4c: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a2c53: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a2c5a: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a2c61: e8 6a 42 7d 00              	call	0x140d76ed0 <.text+0xd75ed0>
1405a2c66: 49 89 d0                    	mov	r8, rdx
1405a2c69: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a2c70: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a2c77: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a2c7e: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a2c85: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a2c8c: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a2c93: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a2c9a: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a2ca1: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a2ca8: 48 89 c2                    	mov	rdx, rax
1405a2cab: e8 90 64 7b 00              	call	0x140d59140 <.text+0xd58140>
1405a2cb0: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a2cb7: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a2cbe: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a2cc5: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a2ccc: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a2cd3: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a2cda: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a2ce1: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a2ce8: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405a2cef: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a2cf6: 4c 8d 83 38 03 00 00        	lea	r8, [rbx + 0x338]
1405a2cfd: e8 9e f1 d6 ff              	call	0x140311ea0 <.text+0x310ea0>
1405a2d02: 48 8b 83 90 25 00 00        	mov	rax, qword ptr [rbx + 0x2590]
1405a2d09: 4c 8b 83 98 25 00 00        	mov	r8, qword ptr [rbx + 0x2598]
1405a2d10: 48 89 83 30 01 00 00        	mov	qword ptr [rbx + 0x130], rax
1405a2d17: 48 f7 d8                    	neg	rax
1405a2d1a: 0f 80 52 41 00 00           	jo	0x1405a6e72 <.text+0x5a5e72>
1405a2d20: 4c 8b 8b a0 25 00 00        	mov	r9, qword ptr [rbx + 0x25a0]
1405a2d27: 48 c7 83 10 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb10], 0x0
1405a2d32: 48 c7 83 00 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb00], 0x1
1405a2d3d: 48 c7 83 08 0b 00 00 80 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x80
1405a2d48: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a2d4f: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a2d56: 4c 89 83 28 01 00 00        	mov	qword ptr [rbx + 0x128], r8
1405a2d5d: e8 8e 06 f8 ff              	call	0x1405233f0 <.text+0x5223f0>
1405a2d62: 0f b6 83 40 04 00 00        	movzx	eax, byte ptr [rbx + 0x440]
1405a2d69: 3c 32                       	cmp	al, 0x32
1405a2d6b: 0f 85 e2 03 00 00           	jne	0x1405a3153 <.text+0x5a2153>
1405a2d71: 4c 8b ab 48 04 00 00        	mov	r13, qword ptr [rbx + 0x448]
1405a2d78: 48 8b b3 50 04 00 00        	mov	rsi, qword ptr [rbx + 0x450]
1405a2d7f: 0f 10 83 58 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x458]
1405a2d86: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
1405a2d8d: 48 8b 93 30 01 00 00        	mov	rdx, qword ptr [rbx + 0x130]
1405a2d94: 48 85 d2                    	test	rdx, rdx
1405a2d97: 74 12                       	je	0x1405a2dab <.text+0x5a1dab>
1405a2d99: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a2d9f: 48 8b 8b 28 01 00 00        	mov	rcx, qword ptr [rbx + 0x128]
1405a2da6: e8 75 0e 07 00              	call	0x140613c20 <.text+0x612c20>
1405a2dab: 4c 89 e8                    	mov	rax, r13
1405a2dae: 48 f7 d8                    	neg	rax
1405a2db1: 0f 80 b8 40 00 00           	jo	0x1405a6e6f <.text+0x5a5e6f>
1405a2db7: 4c 89 ab 30 22 00 00        	mov	qword ptr [rbx + 0x2230], r13
1405a2dbe: 48 89 b3 38 22 00 00        	mov	qword ptr [rbx + 0x2238], rsi
1405a2dc5: 0f 28 83 00 0b 00 00        	movaps	xmm0, xmmword ptr [rbx + 0xb00]
1405a2dcc: 0f 11 83 40 22 00 00        	movups	xmmword ptr [rbx + 0x2240], xmm0
1405a2dd3: 4c 8d bb 98 25 00 00        	lea	r15, [rbx + 0x2598]
1405a2dda: 45 31 e4                    	xor	r12d, r12d
1405a2ddd: 49 bd 14 00 00 00 00 00 00 80       	movabs	r13, -0x7fffffffffffffec
1405a2de7: 49 be 00 00 00 00 00 00 00 80       	movabs	r14, -0x8000000000000000
1405a2df1: 45 0f 57 e4                 	xorps	xmm12, xmm12
1405a2df5: 48 8d b3 00 0b 00 00        	lea	rsi, [rbx + 0xb00]
1405a2dfc: 48 8d bb 90 25 00 00        	lea	rdi, [rbx + 0x2590]
1405a2e03: 45 0f 57 ed                 	xorps	xmm13, xmm13
1405a2e07: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1405a2e10: 48 8b 83 48 22 00 00        	mov	rax, qword ptr [rbx + 0x2248]
1405a2e17: 48 85 c0                    	test	rax, rax
1405a2e1a: 0f 84 7c 02 00 00           	je	0x1405a309c <.text+0x5a209c>
1405a2e20: 4c 8b 8b 40 22 00 00        	mov	r9, qword ptr [rbx + 0x2240]
1405a2e27: 4d 8d 41 01                 	lea	r8, [r9 + 0x1]
1405a2e2b: 48 8b 93 30 22 00 00        	mov	rdx, qword ptr [rbx + 0x2230]
1405a2e32: 48 8b 8b 38 22 00 00        	mov	rcx, qword ptr [rbx + 0x2238]
1405a2e39: 49 39 d0                    	cmp	r8, rdx
1405a2e3c: 49 89 d0                    	mov	r8, rdx
1405a2e3f: 4d 0f 42 c4                 	cmovb	r8, r12
1405a2e43: 49 f7 d8                    	neg	r8
1405a2e46: 4d 01 c8                    	add	r8, r9
1405a2e49: 49 ff c0                    	inc	r8
1405a2e4c: 4c 89 83 40 22 00 00        	mov	qword ptr [rbx + 0x2240], r8
1405a2e53: 4c 8d 50 ff                 	lea	r10, [rax - 0x1]
1405a2e57: 4c 89 93 48 22 00 00        	mov	qword ptr [rbx + 0x2248], r10
1405a2e5e: 4d 6b d1 38                 	imul	r10, r9, 0x38
1405a2e62: 4e 8b 0c 11                 	mov	r9, qword ptr [rcx + r10]
1405a2e66: 42 0f 10 44 11 08           	movups	xmm0, xmmword ptr [rcx + r10 + 0x8]
1405a2e6c: 0f 29 83 f0 1f 00 00        	movaps	xmmword ptr [rbx + 0x1ff0], xmm0
1405a2e73: 42 0f 10 44 11 18           	movups	xmm0, xmmword ptr [rcx + r10 + 0x18]
1405a2e79: 42 0f 10 4c 11 28           	movups	xmm1, xmmword ptr [rcx + r10 + 0x28]
1405a2e7f: 0f 29 83 f0 2b 00 00        	movaps	xmmword ptr [rbx + 0x2bf0], xmm0
1405a2e86: 0f 29 8b 00 2c 00 00        	movaps	xmmword ptr [rbx + 0x2c00], xmm1
1405a2e8d: 4d 39 e9                    	cmp	r9, r13
1405a2e90: 0f 84 06 02 00 00           	je	0x1405a309c <.text+0x5a209c>
1405a2e96: 4c 89 8b 90 25 00 00        	mov	qword ptr [rbx + 0x2590], r9
1405a2e9d: 0f 28 83 f0 1f 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x1ff0]
1405a2ea4: 41 0f 11 07                 	movups	xmmword ptr [r15], xmm0
1405a2ea8: 0f 28 83 f0 2b 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x2bf0]
1405a2eaf: 0f 28 8b 00 2c 00 00        	movaps	xmm1, xmmword ptr [rbx + 0x2c00]
1405a2eb6: 41 0f 11 4f 20              	movups	xmmword ptr [r15 + 0x20], xmm1
1405a2ebb: 41 0f 11 47 10              	movups	xmmword ptr [r15 + 0x10], xmm0
1405a2ec0: 4d 89 ca                    	mov	r10, r9
1405a2ec3: 4d 31 f2                    	xor	r10, r14
1405a2ec6: 49 83 fa 06                 	cmp	r10, 0x6
1405a2eca: 0f 84 80 00 00 00           	je	0x1405a2f50 <.text+0x5a1f50>
1405a2ed0: 49 83 fa 07                 	cmp	r10, 0x7
1405a2ed4: 0f 85 9c 0a 00 00           	jne	0x1405a3976 <.text+0x5a2976>
1405a2eda: 48 8d 83 10 0b 00 00        	lea	rax, [rbx + 0xb10]
1405a2ee1: 44 0f 11 28                 	movups	xmmword ptr [rax], xmm13
1405a2ee5: 48 8d 83 30 0b 00 00        	lea	rax, [rbx + 0xb30]
1405a2eec: 44 0f 11 28                 	movups	xmmword ptr [rax], xmm13
1405a2ef0: 48 c7 83 00 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb00], 0x0
1405a2efb: 48 c7 83 08 0b 00 00 08 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x8
1405a2f06: 48 c7 83 20 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb20], 0x0
1405a2f11: 48 c7 83 28 0b 00 00 08 00 00 00    	mov	qword ptr [rbx + 0xb28], 0x8
1405a2f1c: 48 b8 03 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffffd
1405a2f26: 48 89 83 40 0b 00 00        	mov	qword ptr [rbx + 0xb40], rax
1405a2f2d: c6 83 d9 00 00 00 01        	mov	byte ptr [rbx + 0xd9], 0x1
1405a2f34: c6 83 da 00 00 00 01        	mov	byte ptr [rbx + 0xda], 0x1
1405a2f3b: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a2f42: 48 89 f2                    	mov	rdx, rsi
1405a2f45: e8 c6 49 b9 ff              	call	0x140137910 <.text+0x136910>
1405a2f4a: e9 b3 00 00 00              	jmp	0x1405a3002 <.text+0x5a2002>
1405a2f4f: 90                          	nop
1405a2f50: f3 0f 10 83 98 25 00 00     	movss	xmm0, dword ptr [rbx + 0x2598]
1405a2f58: 44 0f 2e e0                 	ucomiss	xmm12, xmm0
1405a2f5c: 0f 87 4f 36 00 00           	ja	0x1405a65b1 <.text+0x5a55b1>
1405a2f62: 66 41 0f 7e c1              	movd	r9d, xmm0
1405a2f67: 45 89 c8                    	mov	r8d, r9d
1405a2f6a: 41 c1 e8 17                 	shr	r8d, 0x17
1405a2f6e: 41 0f b6 c8                 	movzx	ecx, r8b
1405a2f72: 31 c0                       	xor	eax, eax
1405a2f74: 66 83 f9 60                 	cmp	cx, 0x60
1405a2f78: 72 51                       	jb	0x1405a2fcb <.text+0x5a1fcb>
1405a2f7a: 44 89 ca                    	mov	edx, r9d
1405a2f7d: 81 e2 ff ff 7f 00           	and	edx, 0x7fffff
1405a2f83: 81 ca 00 00 80 00           	or	edx, 0x800000
1405a2f89: 66 83 f9 7f                 	cmp	cx, 0x7f
1405a2f8d: 0f 83 90 00 00 00           	jae	0x1405a3023 <.text+0x5a2023>
1405a2f93: 89 d0                       	mov	eax, edx
1405a2f95: 41 8d 48 2a                 	lea	ecx, [r8 + 0x2a]
1405a2f99: 48 d3 e0                    	shl	rax, cl
1405a2f9c: b9 00 ca 9a 3b              	mov	ecx, 0x3b9aca00
1405a2fa1: 48 f7 e1                    	mul	rcx
1405a2fa4: 48 89 c1                    	mov	rcx, rax
1405a2fa7: 48 f7 d9                    	neg	rcx
1405a2faa: 0f 91 c1                    	setno	cl
1405a2fad: 48 c1 e8 3f                 	shr	rax, 0x3f
1405a2fb1: 41 89 d0                    	mov	r8d, edx
1405a2fb4: 41 08 c8                    	or	r8b, cl
1405a2fb7: 41 20 c0                    	and	r8b, al
1405a2fba: 41 0f b6 c0                 	movzx	eax, r8b
1405a2fbe: 01 d0                       	add	eax, edx
1405a2fc0: 3d 00 ca 9a 3b              	cmp	eax, 0x3b9aca00
1405a2fc5: 0f 84 e6 35 00 00           	je	0x1405a65b1 <.text+0x5a55b1>
1405a2fcb: 31 d2                       	xor	edx, edx
1405a2fcd: 48 89 93 08 0b 00 00        	mov	qword ptr [rbx + 0xb08], rdx
1405a2fd4: 89 83 10 0b 00 00           	mov	dword ptr [rbx + 0xb10], eax
1405a2fda: 48 c7 83 00 0b 00 00 02 00 00 00    	mov	qword ptr [rbx + 0xb00], 0x2
1405a2fe5: c6 83 d9 00 00 00 01        	mov	byte ptr [rbx + 0xd9], 0x1
1405a2fec: c6 83 da 00 00 00 01        	mov	byte ptr [rbx + 0xda], 0x1
1405a2ff3: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a2ffa: 48 89 f2                    	mov	rdx, rsi
1405a2ffd: e8 6e 21 b9 ff              	call	0x140135170 <.text+0x134170>
1405a3002: 4c 3b a3 90 25 00 00        	cmp	r12, qword ptr [rbx + 0x2590]
1405a3009: 0f 80 01 fe ff ff           	jo	0x1405a2e10 <.text+0x5a1e10>
1405a300f: c6 83 f4 00 00 00 01        	mov	byte ptr [rbx + 0xf4], 0x1
1405a3016: 48 89 f9                    	mov	rcx, rdi
1405a3019: e8 a2 fc fe ff              	call	0x140592cc0 <.text+0x591cc0>
1405a301e: e9 ed fd ff ff              	jmp	0x1405a2e10 <.text+0x5a1e10>
1405a3023: 0f b7 c9                    	movzx	ecx, cx
1405a3026: 81 f9 96 00 00 00           	cmp	ecx, 0x96
1405a302c: 73 54                       	jae	0x1405a3082 <.text+0x5a2082>
1405a302e: 41 8d 40 01                 	lea	eax, [r8 + 0x1]
1405a3032: b1 16                       	mov	cl, 0x16
1405a3034: 44 28 c1                    	sub	cl, r8b
1405a3037: d3 ea                       	shr	edx, cl
1405a3039: 89 c1                       	mov	ecx, eax
1405a303b: 41 d3 e1                    	shl	r9d, cl
1405a303e: 41 81 e1 ff ff 7f 00        	and	r9d, 0x7fffff
1405a3045: 49 69 c1 00 ca 9a 3b        	imul	rax, r9, 0x3b9aca00
1405a304c: 48 89 c1                    	mov	rcx, rax
1405a304f: 48 c1 e9 17                 	shr	rcx, 0x17
1405a3053: 41 89 c0                    	mov	r8d, eax
1405a3056: 41 81 e0 00 fe 7f 00        	and	r8d, 0x7ffe00
1405a305d: 41 81 f8 00 00 40 00        	cmp	r8d, 0x400000
1405a3064: 41 0f 95 c0                 	setne	r8b
1405a3068: c1 e8 16                    	shr	eax, 0x16
1405a306b: 83 e0 01                    	and	eax, 0x1
1405a306e: 41 89 c9                    	mov	r9d, ecx
1405a3071: 45 08 c1                    	or	r9b, r8b
1405a3074: 41 20 c1                    	and	r9b, al
1405a3077: 41 0f b6 c1                 	movzx	eax, r9b
1405a307b: 01 c8                       	add	eax, ecx
1405a307d: e9 4b ff ff ff              	jmp	0x1405a2fcd <.text+0x5a1fcd>
1405a3082: 81 f9 bf 00 00 00           	cmp	ecx, 0xbf
1405a3088: 0f 83 6f 3f 00 00           	jae	0x1405a6ffd <.text+0x5a5ffd>
1405a308e: 89 d2                       	mov	edx, edx
1405a3090: 41 8d 48 2a                 	lea	ecx, [r8 + 0x2a]
1405a3094: 48 d3 e2                    	shl	rdx, cl
1405a3097: e9 31 ff ff ff              	jmp	0x1405a2fcd <.text+0x5a1fcd>
1405a309c: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a30a3: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a30aa: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a30b1: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a30b8: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a30bf: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a30c6: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a30cd: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a30d4: 48 8d 8b 30 22 00 00        	lea	rcx, [rbx + 0x2230]
1405a30db: e8 10 fc 02 00              	call	0x1405d2cf0 <.text+0x5d1cf0>
1405a30e0: 48 8b 8d c0 37 00 00        	mov	rcx, qword ptr [rbp + 0x37c0]
1405a30e7: 31 c0                       	xor	eax, eax
1405a30e9: 48 3b 81 a8 00 00 00        	cmp	rax, qword ptr [rcx + 0xa8]
1405a30f0: 71 4e                       	jno	0x1405a3140 <.text+0x5a2140>
1405a30f2: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a30f9: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a3100: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a3107: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a310e: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a3115: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a311c: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a3123: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a312a: 40 b7 01                    	mov	dil, 0x1
1405a312d: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a3134: 31 d2                       	xor	edx, edx
1405a3136: e8 75 d8 b7 ff              	call	0x1401209b0 <.text+0x11f9b0>
1405a313b: e9 f7 0c 00 00              	jmp	0x1405a3e37 <.text+0x5a2e37>
1405a3140: 80 79 20 00                 	cmp	byte ptr [rcx + 0x20], 0x0
1405a3144: 0f 84 98 00 00 00           	je	0x1405a31e2 <.text+0x5a21e2>
1405a314a: 48 8b 79 28                 	mov	rdi, qword ptr [rcx + 0x28]
1405a314e: e9 91 00 00 00              	jmp	0x1405a31e4 <.text+0x5a21e4>
1405a3153: 8b 8b 41 04 00 00           	mov	ecx, dword ptr [rbx + 0x441]
1405a3159: 8b 93 44 04 00 00           	mov	edx, dword ptr [rbx + 0x444]
1405a315f: 89 93 04 0b 00 00           	mov	dword ptr [rbx + 0xb04], edx
1405a3165: 89 8b 01 0b 00 00           	mov	dword ptr [rbx + 0xb01], ecx
1405a316b: 0f 10 83 48 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x448]
1405a3172: 0f 10 8b 58 04 00 00        	movups	xmm1, xmmword ptr [rbx + 0x458]
1405a3179: 0f 29 8b f0 2b 00 00        	movaps	xmmword ptr [rbx + 0x2bf0], xmm1
1405a3180: 0f 10 93 68 04 00 00        	movups	xmm2, xmmword ptr [rbx + 0x468]
1405a3187: 0f 11 93 28 0b 00 00        	movups	xmmword ptr [rbx + 0xb28], xmm2
1405a318e: 0f 10 93 78 04 00 00        	movups	xmm2, xmmword ptr [rbx + 0x478]
1405a3195: 0f 11 93 38 0b 00 00        	movups	xmmword ptr [rbx + 0xb38], xmm2
1405a319c: 0f 10 93 88 04 00 00        	movups	xmm2, xmmword ptr [rbx + 0x488]
1405a31a3: 0f 11 93 48 0b 00 00        	movups	xmmword ptr [rbx + 0xb48], xmm2
1405a31aa: 0f 11 83 08 0b 00 00        	movups	xmmword ptr [rbx + 0xb08], xmm0
1405a31b1: 0f 11 8b 18 0b 00 00        	movups	xmmword ptr [rbx + 0xb18], xmm1
1405a31b8: 88 83 00 0b 00 00           	mov	byte ptr [rbx + 0xb00], al
1405a31be: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a31c5: e8 36 0c 30 02              	call	0x1428a3e00 <.text+0x28a2e00>
1405a31ca: 48 89 c6                    	mov	rsi, rax
1405a31cd: 48 8b 93 30 01 00 00        	mov	rdx, qword ptr [rbx + 0x130]
1405a31d4: 48 85 d2                    	test	rdx, rdx
1405a31d7: 0f 85 bc fb ff ff           	jne	0x1405a2d99 <.text+0x5a1d99>
1405a31dd: e9 c9 fb ff ff              	jmp	0x1405a2dab <.text+0x5a1dab>
1405a31e2: 31 ff                       	xor	edi, edi
1405a31e4: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a31eb: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a31f2: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a31f9: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a3200: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a3207: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a320e: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a3215: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a321c: 48 8d 8b d0 02 00 00        	lea	rcx, [rbx + 0x2d0]
1405a3223: e8 f8 31 37 00              	call	0x140916420 <.text+0x915420>
1405a3228: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a322f: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a3236: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a323d: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a3244: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a324b: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a3252: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a3259: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a3260: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a3267: 48 8d 93 d0 02 00 00        	lea	rdx, [rbx + 0x2d0]
1405a326e: e8 8d 2d 5a 00              	call	0x140b46000 <.text+0xb45000>
1405a3273: c6 83 40 04 00 00 01        	mov	byte ptr [rbx + 0x440], 0x1
1405a327a: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a3281: e8 4a 3c 7d 00              	call	0x140d76ed0 <.text+0xd75ed0>
1405a3286: 49 89 d0                    	mov	r8, rdx
1405a3289: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a3290: 48 89 c2                    	mov	rdx, rax
1405a3293: e8 b8 6e 7b 00              	call	0x140d5a150 <.text+0xd59150>
1405a3298: 48 89 c6                    	mov	rsi, rax
1405a329b: 48 8b 93 00 0b 00 00        	mov	rdx, qword ptr [rbx + 0xb00]
1405a32a2: 48 85 d2                    	test	rdx, rdx
1405a32a5: 74 12                       	je	0x1405a32b9 <.text+0x5a22b9>
1405a32a7: 48 8b 8b 08 0b 00 00        	mov	rcx, qword ptr [rbx + 0xb08]
1405a32ae: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a32b4: e8 67 09 07 00              	call	0x140613c20 <.text+0x612c20>
1405a32b9: 48 85 f6                    	test	rsi, rsi
1405a32bc: 0f 85 e7 3b 00 00           	jne	0x1405a6ea9 <.text+0x5a5ea9>
1405a32c2: 0f 10 83 d0 02 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2d0]
1405a32c9: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
1405a32d0: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a32d7: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a32de: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a32e5: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a32ec: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a32f3: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a32fa: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a3301: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a3308: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a330f: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a3316: e8 05 36 5a 00              	call	0x140b46920 <.text+0xb45920>
1405a331b: 48 8b 85 c0 37 00 00        	mov	rax, qword ptr [rbp + 0x37c0]
1405a3322: 48 8d b0 a8 00 00 00        	lea	rsi, [rax + 0xa8]
1405a3329: 48 c7 83 f8 2b 00 00 0a 00 00 00    	mov	qword ptr [rbx + 0x2bf8], 0xa
1405a3334: 48 b8 05 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffffb
1405a333e: 48 89 83 f0 2b 00 00        	mov	qword ptr [rbx + 0x2bf0], rax
1405a3345: 48 b8 01 00 00 00 00 00 00 80       	movabs	rax, -0x7fffffffffffffff
1405a334f: 48 89 83 90 25 00 00        	mov	qword ptr [rbx + 0x2590], rax
1405a3356: 48 89 f1                    	mov	rcx, rsi
1405a3359: e8 72 3b 7d 00              	call	0x140d76ed0 <.text+0xd75ed0>
1405a335e: 49 89 d0                    	mov	r8, rdx
1405a3361: 48 c7 44 24 20 0b 00 00 00  	mov	qword ptr [rsp + 0x20], 0xb
1405a336a: 4c 8d 0d 7f c6 45 02        	lea	r9, [rip + 0x245c67f]   # 0x1429ff9f0
1405a3371: 48 8d 8b 30 22 00 00        	lea	rcx, [rbx + 0x2230]
1405a3378: 48 89 c2                    	mov	rdx, rax
1405a337b: e8 90 f4 7b 00              	call	0x140d62810 <.text+0xd61810>
1405a3380: 0f 10 83 30 22 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2230]
1405a3387: 0f 10 8b 40 22 00 00        	movups	xmm1, xmmword ptr [rbx + 0x2240]
1405a338e: 0f 11 8b 88 0b 00 00        	movups	xmmword ptr [rbx + 0xb88], xmm1
1405a3395: 0f 11 83 78 0b 00 00        	movups	xmmword ptr [rbx + 0xb78], xmm0
1405a339c: 0f 10 83 f0 2b 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2bf0]
1405a33a3: 0f 10 8b 00 2c 00 00        	movups	xmm1, xmmword ptr [rbx + 0x2c00]
1405a33aa: 0f 10 93 10 2c 00 00        	movups	xmm2, xmmword ptr [rbx + 0x2c10]
1405a33b1: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
1405a33b8: 0f 29 8b 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm1
1405a33bf: 0f 29 93 20 0b 00 00        	movaps	xmmword ptr [rbx + 0xb20], xmm2
1405a33c6: 48 8b 83 20 2c 00 00        	mov	rax, qword ptr [rbx + 0x2c20]
1405a33cd: 48 89 83 30 0b 00 00        	mov	qword ptr [rbx + 0xb30], rax
1405a33d4: 0f 10 83 90 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2590]
1405a33db: 0f 10 8b a0 25 00 00        	movups	xmm1, xmmword ptr [rbx + 0x25a0]
1405a33e2: 0f 10 93 b0 25 00 00        	movups	xmm2, xmmword ptr [rbx + 0x25b0]
1405a33e9: 0f 11 83 38 0b 00 00        	movups	xmmword ptr [rbx + 0xb38], xmm0
1405a33f0: 0f 11 8b 48 0b 00 00        	movups	xmmword ptr [rbx + 0xb48], xmm1
1405a33f7: 0f 11 93 58 0b 00 00        	movups	xmmword ptr [rbx + 0xb58], xmm2
1405a33fe: 48 8b 83 c0 25 00 00        	mov	rax, qword ptr [rbx + 0x25c0]
1405a3405: 48 89 83 68 0b 00 00        	mov	qword ptr [rbx + 0xb68], rax
1405a340c: 48 b8 08 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffff8
1405a3416: 48 89 83 70 0b 00 00        	mov	qword ptr [rbx + 0xb70], rax
1405a341d: 48 89 bb 98 0b 00 00        	mov	qword ptr [rbx + 0xb98], rdi
1405a3424: c6 83 1e 01 00 00 00        	mov	byte ptr [rbx + 0x11e], 0x0
1405a342b: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405a3432: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a3439: 4c 8d 83 00 0b 00 00        	lea	r8, [rbx + 0xb00]
1405a3440: e8 0b 1d e1 ff              	call	0x1403b5150 <.text+0x3b4150>
1405a3445: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a344c: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a3453: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a345a: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a3461: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a3468: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a346f: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a3476: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a347d: 48 8d 93 90 25 00 00        	lea	rdx, [rbx + 0x2590]
1405a3484: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a348b: e8 20 6f b7 ff              	call	0x14011a3b0 <.text+0x1193b0>
1405a3490: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a3497: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a349e: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a34a5: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a34ac: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a34b3: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a34ba: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a34c1: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a34c8: 48 89 f1                    	mov	rcx, rsi
1405a34cb: e8 00 3a 7d 00              	call	0x140d76ed0 <.text+0xd75ed0>
1405a34d0: 49 89 d0                    	mov	r8, rdx
1405a34d3: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a34da: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a34e1: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a34e8: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a34ef: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a34f6: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a34fd: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a3504: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a350b: 48 c7 44 24 20 09 00 00 00  	mov	qword ptr [rsp + 0x20], 0x9
1405a3514: 4c 8d 0d e0 c4 45 02        	lea	r9, [rip + 0x245c4e0]   # 0x1429ff9fb
1405a351b: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405a3522: 48 89 c2                    	mov	rdx, rax
1405a3525: e8 e6 f2 7b 00              	call	0x140d62810 <.text+0xd61810>
1405a352a: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a3531: 48 8d 93 d0 02 00 00        	lea	rdx, [rbx + 0x2d0]
1405a3538: e8 c3 2a 5a 00              	call	0x140b46000 <.text+0xb45000>
1405a353d: c6 83 11 01 00 00 01        	mov	byte ptr [rbx + 0x111], 0x1
1405a3544: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a354b: e8 80 39 7d 00              	call	0x140d76ed0 <.text+0xd75ed0>
1405a3550: 49 89 d0                    	mov	r8, rdx
1405a3553: c6 83 11 01 00 00 01        	mov	byte ptr [rbx + 0x111], 0x1
1405a355a: 48 c7 44 24 20 0c 00 00 00  	mov	qword ptr [rsp + 0x20], 0xc
1405a3563: 4c 8d 0d 9a c4 45 02        	lea	r9, [rip + 0x245c49a]   # 0x1429ffa04
1405a356a: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a3571: 48 89 c2                    	mov	rdx, rax
1405a3574: e8 97 f2 7b 00              	call	0x140d62810 <.text+0xd61810>
1405a3579: c6 83 11 01 00 00 00        	mov	byte ptr [rbx + 0x111], 0x0
1405a3580: 48 8d bb 90 25 00 00        	lea	rdi, [rbx + 0x2590]
1405a3587: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a358e: 48 89 f9                    	mov	rcx, rdi
1405a3591: e8 4a d2 d6 ff              	call	0x1403107e0 <.text+0x30f7e0>
1405a3596: a8 01                       	test	al, 0x1
1405a3598: 0f 85 42 39 00 00           	jne	0x1405a6ee0 <.text+0x5a5ee0>
1405a359e: 48 8b 93 00 0b 00 00        	mov	rdx, qword ptr [rbx + 0xb00]
1405a35a5: 48 85 d2                    	test	rdx, rdx
1405a35a8: 74 12                       	je	0x1405a35bc <.text+0x5a25bc>
1405a35aa: 48 8b 8b 08 0b 00 00        	mov	rcx, qword ptr [rbx + 0xb08]
1405a35b1: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a35b7: e8 64 06 07 00              	call	0x140613c20 <.text+0x612c20>
1405a35bc: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a35c3: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a35ca: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a35d1: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a35d8: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a35df: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a35e6: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a35ed: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a35f4: 48 89 f1                    	mov	rcx, rsi
1405a35f7: e8 d4 38 7d 00              	call	0x140d76ed0 <.text+0xd75ed0>
1405a35fc: 49 89 d0                    	mov	r8, rdx
1405a35ff: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a3606: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a360d: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a3614: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a361b: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a3622: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a3629: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a3630: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a3637: 48 c7 44 24 20 09 00 00 00  	mov	qword ptr [rsp + 0x20], 0x9
1405a3640: 4c 8d 0d e1 c3 45 02        	lea	r9, [rip + 0x245c3e1]   # 0x1429ffa28
1405a3647: 48 8d 8b f0 1f 00 00        	lea	rcx, [rbx + 0x1ff0]
1405a364e: 48 89 c2                    	mov	rdx, rax
1405a3651: e8 ba f1 7b 00              	call	0x140d62810 <.text+0xd61810>
1405a3656: c6 83 d3 00 00 00 01        	mov	byte ptr [rbx + 0xd3], 0x1
1405a365d: 48 8d 8b f0 1f 00 00        	lea	rcx, [rbx + 0x1ff0]
1405a3664: e8 67 38 7d 00              	call	0x140d76ed0 <.text+0xd75ed0>
1405a3669: c6 83 d3 00 00 00 01        	mov	byte ptr [rbx + 0xd3], 0x1
1405a3670: 48 89 c1                    	mov	rcx, rax
1405a3673: e8 98 f7 7b 00              	call	0x140d62e10 <.text+0xd61e10>
1405a3678: 84 c0                       	test	al, al
1405a367a: 0f 84 82 00 00 00           	je	0x1405a3702 <.text+0x5a2702>
1405a3680: c6 83 d3 00 00 00 01        	mov	byte ptr [rbx + 0xd3], 0x1
1405a3687: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a368e: 48 8d 93 d0 02 00 00        	lea	rdx, [rbx + 0x2d0]
1405a3695: e8 66 29 5a 00              	call	0x140b46000 <.text+0xb45000>
1405a369a: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a36a1: e8 2a 38 7d 00              	call	0x140d76ed0 <.text+0xd75ed0>
1405a36a6: 49 89 d0                    	mov	r8, rdx
1405a36a9: 48 c7 44 24 20 09 00 00 00  	mov	qword ptr [rsp + 0x20], 0x9
1405a36b2: 4c 8d 0d 6f c3 45 02        	lea	r9, [rip + 0x245c36f]   # 0x1429ffa28
1405a36b9: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a36c0: 48 89 c2                    	mov	rdx, rax
1405a36c3: e8 48 f1 7b 00              	call	0x140d62810 <.text+0xd61810>
1405a36c8: 48 8d 8b f0 1f 00 00        	lea	rcx, [rbx + 0x1ff0]
1405a36cf: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a36d6: e8 25 21 db ff              	call	0x140355800 <.text+0x354800>
1405a36db: 48 85 c0                    	test	rax, rax
1405a36de: 0f 85 2a 39 00 00           	jne	0x1405a700e <.text+0x5a600e>
1405a36e4: 48 8b 93 00 0b 00 00        	mov	rdx, qword ptr [rbx + 0xb00]
1405a36eb: 48 85 d2                    	test	rdx, rdx
1405a36ee: 74 12                       	je	0x1405a3702 <.text+0x5a2702>
1405a36f0: 48 8b 8b 08 0b 00 00        	mov	rcx, qword ptr [rbx + 0xb08]
1405a36f7: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a36fd: e8 1e 05 07 00              	call	0x140613c20 <.text+0x612c20>
1405a3702: c6 83 d3 00 00 00 01        	mov	byte ptr [rbx + 0xd3], 0x1
1405a3709: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a3710: 48 8d 93 d0 02 00 00        	lea	rdx, [rbx + 0x2d0]
1405a3717: e8 e4 28 5a 00              	call	0x140b46000 <.text+0xb45000>
1405a371c: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a3723: e8 a8 37 7d 00              	call	0x140d76ed0 <.text+0xd75ed0>
1405a3728: 49 89 d0                    	mov	r8, rdx
1405a372b: 48 c7 44 24 20 0c 00 00 00  	mov	qword ptr [rsp + 0x20], 0xc
1405a3734: 4c 8d 0d 15 c3 45 02        	lea	r9, [rip + 0x245c315]   # 0x1429ffa50
1405a373b: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a3742: 48 89 c2                    	mov	rdx, rax
1405a3745: e8 c6 f0 7b 00              	call	0x140d62810 <.text+0xd61810>
1405a374a: 4c 8d b3 40 04 00 00        	lea	r14, [rbx + 0x440]
1405a3751: 4c 89 f1                    	mov	rcx, r14
1405a3754: e8 77 37 7d 00              	call	0x140d76ed0 <.text+0xd75ed0>
1405a3759: 4c 8d 05 fc c2 45 02        	lea	r8, [rip + 0x245c2fc]   # 0x1429ffa5c
1405a3760: 41 b9 05 00 00 00           	mov	r9d, 0x5
1405a3766: 48 89 c1                    	mov	rcx, rax
1405a3769: e8 c2 5c 7b 00              	call	0x140d59430 <.text+0xd58430>
1405a376e: 49 89 c7                    	mov	r15, rax
1405a3771: 48 8b 93 40 04 00 00        	mov	rdx, qword ptr [rbx + 0x440]
1405a3778: 48 85 d2                    	test	rdx, rdx
1405a377b: 74 12                       	je	0x1405a378f <.text+0x5a278f>
1405a377d: 48 8b 8b 48 04 00 00        	mov	rcx, qword ptr [rbx + 0x448]
1405a3784: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a378a: e8 91 04 07 00              	call	0x140613c20 <.text+0x612c20>
1405a378f: 4d 85 ff                    	test	r15, r15
1405a3792: 0f 85 7f 37 00 00           	jne	0x1405a6f17 <.text+0x5a5f17>
1405a3798: 48 8b 93 00 0b 00 00        	mov	rdx, qword ptr [rbx + 0xb00]
1405a379f: 48 85 d2                    	test	rdx, rdx
1405a37a2: 74 12                       	je	0x1405a37b6 <.text+0x5a27b6>
1405a37a4: 48 8b 8b 08 0b 00 00        	mov	rcx, qword ptr [rbx + 0xb08]
1405a37ab: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a37b1: e8 6a 04 07 00              	call	0x140613c20 <.text+0x612c20>
1405a37b6: c6 83 d3 00 00 00 01        	mov	byte ptr [rbx + 0xd3], 0x1
1405a37bd: 48 89 f1                    	mov	rcx, rsi
1405a37c0: e8 0b 37 7d 00              	call	0x140d76ed0 <.text+0xd75ed0>
1405a37c5: 49 89 d0                    	mov	r8, rdx
1405a37c8: c6 83 d3 00 00 00 01        	mov	byte ptr [rbx + 0xd3], 0x1
1405a37cf: 48 c7 44 24 20 0c 00 00 00  	mov	qword ptr [rsp + 0x20], 0xc
1405a37d8: 4c 8d 0d a1 c2 45 02        	lea	r9, [rip + 0x245c2a1]   # 0x1429ffa80
1405a37df: 48 8d 8b 30 22 00 00        	lea	rcx, [rbx + 0x2230]
1405a37e6: 48 89 c2                    	mov	rdx, rax
1405a37e9: e8 22 f0 7b 00              	call	0x140d62810 <.text+0xd61810>
1405a37ee: c6 83 c7 00 00 00 01        	mov	byte ptr [rbx + 0xc7], 0x1
1405a37f5: 48 8d 8b 30 22 00 00        	lea	rcx, [rbx + 0x2230]
1405a37fc: e8 cf 36 7d 00              	call	0x140d76ed0 <.text+0xd75ed0>
1405a3801: 49 89 d0                    	mov	r8, rdx
1405a3804: c6 83 c7 00 00 00 01        	mov	byte ptr [rbx + 0xc7], 0x1
1405a380b: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a3812: 48 89 c2                    	mov	rdx, rax
1405a3815: e8 26 59 7b 00              	call	0x140d59140 <.text+0xd58140>
1405a381a: c6 83 c7 00 00 00 01        	mov	byte ptr [rbx + 0xc7], 0x1
1405a3821: 48 8d 8b 70 03 00 00        	lea	rcx, [rbx + 0x370]
1405a3828: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a382f: 4c 8d 83 30 22 00 00        	lea	r8, [rbx + 0x2230]
1405a3836: e8 05 e3 d6 ff              	call	0x140311b40 <.text+0x310b40>
1405a383b: 48 8b 8b 70 03 00 00        	mov	rcx, qword ptr [rbx + 0x370]
1405a3842: 4c 8b 83 78 03 00 00        	mov	r8, qword ptr [rbx + 0x378]
1405a3849: 48 89 c8                    	mov	rax, rcx
1405a384c: 48 f7 d8                    	neg	rax
1405a384f: 0f 80 13 03 00 00           	jo	0x1405a3b68 <.text+0x5a2b68>
1405a3855: 48 89 8b 30 01 00 00        	mov	qword ptr [rbx + 0x130], rcx
1405a385c: 4c 8b 8b 80 03 00 00        	mov	r9, qword ptr [rbx + 0x380]
1405a3863: 48 c7 83 a0 25 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x25a0], 0x0
1405a386e: 48 c7 83 90 25 00 00 01 00 00 00    	mov	qword ptr [rbx + 0x2590], 0x1
1405a3879: 48 c7 83 98 25 00 00 80 00 00 00    	mov	qword ptr [rbx + 0x2598], 0x80
1405a3884: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a388b: 48 8d 93 90 25 00 00        	lea	rdx, [rbx + 0x2590]
1405a3892: 4c 89 83 28 01 00 00        	mov	qword ptr [rbx + 0x128], r8
1405a3899: e8 32 f2 f7 ff              	call	0x140522ad0 <.text+0x521ad0>
1405a389e: 4c 8b bb 00 0b 00 00        	mov	r15, qword ptr [rbx + 0xb00]
1405a38a5: 48 8b b3 08 0b 00 00        	mov	rsi, qword ptr [rbx + 0xb08]
1405a38ac: 4c 89 f8                    	mov	rax, r15
1405a38af: 48 f7 d8                    	neg	rax
1405a38b2: 0f 10 83 10 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb10]
1405a38b9: 0f 29 83 40 04 00 00        	movaps	xmmword ptr [rbx + 0x440], xmm0
1405a38c0: 0f 10 83 20 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb20]
1405a38c7: 0f 29 83 50 04 00 00        	movaps	xmmword ptr [rbx + 0x450], xmm0
1405a38ce: 0f 10 83 30 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb30]
1405a38d5: 0f 29 83 60 04 00 00        	movaps	xmmword ptr [rbx + 0x460], xmm0
1405a38dc: 0f 10 83 40 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb40]
1405a38e3: 0f 29 83 70 04 00 00        	movaps	xmmword ptr [rbx + 0x470], xmm0
1405a38ea: 0f 10 83 50 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb50]
1405a38f1: 0f 29 83 80 04 00 00        	movaps	xmmword ptr [rbx + 0x480], xmm0
1405a38f8: 0f 81 d7 01 00 00           	jno	0x1405a3ad5 <.text+0x5a2ad5>
1405a38fe: 48 89 b3 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rsi
1405a3905: 0f 28 83 40 04 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x440]
1405a390c: 0f 28 8b 50 04 00 00        	movaps	xmm1, xmmword ptr [rbx + 0x450]
1405a3913: 0f 28 93 60 04 00 00        	movaps	xmm2, xmmword ptr [rbx + 0x460]
1405a391a: 0f 28 9b 70 04 00 00        	movaps	xmm3, xmmword ptr [rbx + 0x470]
1405a3921: 0f 11 83 08 0b 00 00        	movups	xmmword ptr [rbx + 0xb08], xmm0
1405a3928: 0f 11 8b 18 0b 00 00        	movups	xmmword ptr [rbx + 0xb18], xmm1
1405a392f: 0f 11 93 28 0b 00 00        	movups	xmmword ptr [rbx + 0xb28], xmm2
1405a3936: 0f 11 9b 38 0b 00 00        	movups	xmmword ptr [rbx + 0xb38], xmm3
1405a393d: 0f 28 83 80 04 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x480]
1405a3944: 0f 11 83 48 0b 00 00        	movups	xmmword ptr [rbx + 0xb48], xmm0
1405a394b: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a3952: e8 a9 04 30 02              	call	0x1428a3e00 <.text+0x28a2e00>
1405a3957: 48 89 c6                    	mov	rsi, rax
1405a395a: 48 8b 93 30 01 00 00        	mov	rdx, qword ptr [rbx + 0x130]
1405a3961: 48 85 d2                    	test	rdx, rdx
1405a3964: 4c 8b a5 b0 37 00 00        	mov	r12, qword ptr [rbp + 0x37b0]
1405a396b: 0f 85 d6 01 00 00           	jne	0x1405a3b47 <.text+0x5a2b47>
1405a3971: e9 e3 01 00 00              	jmp	0x1405a3b59 <.text+0x5a2b59>
1405a3976: 4d 85 d2                    	test	r10, r10
1405a3979: 0f 85 c2 2a 00 00           	jne	0x1405a6441 <.text+0x5a5441>
1405a397f: 0f 28 83 f0 1f 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x1ff0]
1405a3986: 0f 29 83 e0 1f 00 00        	movaps	xmmword ptr [rbx + 0x1fe0], xmm0
1405a398d: 0f b6 b3 b0 25 00 00        	movzx	esi, byte ptr [rbx + 0x25b0]
1405a3994: 48 8b 83 a8 25 00 00        	mov	rax, qword ptr [rbx + 0x25a8]
1405a399b: 48 89 83 70 03 00 00        	mov	qword ptr [rbx + 0x370], rax
1405a39a2: 48 85 c0                    	test	rax, rax
1405a39a5: 0f 85 e2 2b 00 00           	jne	0x1405a658d <.text+0x5a558d>
1405a39ab: c6 83 e3 00 00 00 01        	mov	byte ptr [rbx + 0xe3], 0x1
1405a39b2: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a39b9: 48 8d 93 e0 1f 00 00        	lea	rdx, [rbx + 0x1fe0]
1405a39c0: e8 5b 2f 5a 00              	call	0x140b46920 <.text+0xb45920>
1405a39c5: c6 83 e3 00 00 00 01        	mov	byte ptr [rbx + 0xe3], 0x1
1405a39cc: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a39d3: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a39da: 41 89 f0                    	mov	r8d, esi
1405a39dd: e8 3e 30 5a 00              	call	0x140b46a20 <.text+0xb45a20>
1405a39e2: 0f 10 83 30 22 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2230]
1405a39e9: 0f 10 8b 40 22 00 00        	movups	xmm1, xmmword ptr [rbx + 0x2240]
1405a39f0: 0f 29 8b 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm1
1405a39f7: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
1405a39fe: 48 8b b3 50 04 00 00        	mov	rsi, qword ptr [rbx + 0x450]
1405a3a05: 48 3b b3 40 04 00 00        	cmp	rsi, qword ptr [rbx + 0x440]
1405a3a0c: 75 13                       	jne	0x1405a3a21 <.text+0x5a2a21>
1405a3a0e: 48 8d 15 93 90 42 02        	lea	rdx, [rip + 0x2429093]  # 0x1429ccaa8
1405a3a15: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a3a1c: e8 0f f2 fb ff              	call	0x140562c30 <.text+0x561c30>
1405a3a21: 48 8b 83 48 04 00 00        	mov	rax, qword ptr [rbx + 0x448]
1405a3a28: 48 89 f1                    	mov	rcx, rsi
1405a3a2b: 48 c1 e1 05                 	shl	rcx, 0x5
1405a3a2f: 0f 28 83 00 0b 00 00        	movaps	xmm0, xmmword ptr [rbx + 0xb00]
1405a3a36: 0f 28 8b 10 0b 00 00        	movaps	xmm1, xmmword ptr [rbx + 0xb10]
1405a3a3d: 0f 11 4c 08 10              	movups	xmmword ptr [rax + rcx + 0x10], xmm1
1405a3a42: 0f 11 04 08                 	movups	xmmword ptr [rax + rcx], xmm0
1405a3a46: 48 ff c6                    	inc	rsi
1405a3a49: 48 89 b3 50 04 00 00        	mov	qword ptr [rbx + 0x450], rsi
1405a3a50: 48 8d b3 00 0b 00 00        	lea	rsi, [rbx + 0xb00]
1405a3a57: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a3a5e: 41 b8 a0 00 00 00           	mov	r8d, 0xa0
1405a3a64: 48 89 f1                    	mov	rcx, rsi
1405a3a67: e8 94 01 38 02              	call	0x142923c00 <.text+0x2922c00>
1405a3a6c: c6 83 e3 00 00 00 00        	mov	byte ptr [rbx + 0xe3], 0x0
1405a3a73: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a3a7a: 48 89 f2                    	mov	rdx, rsi
1405a3a7d: e8 2e 69 b7 ff              	call	0x14011a3b0 <.text+0x1193b0>
1405a3a82: 48 8b 83 70 03 00 00        	mov	rax, qword ptr [rbx + 0x370]
1405a3a89: 48 85 c0                    	test	rax, rax
1405a3a8c: 74 20                       	je	0x1405a3aae <.text+0x5a2aae>
1405a3a8e: f0                          	lock
1405a3a8f: 48 ff 08                    	dec	qword ptr [rax]
1405a3a92: 75 1a                       	jne	0x1405a3aae <.text+0x5a2aae>
1405a3a94: c6 83 d9 00 00 00 00        	mov	byte ptr [rbx + 0xd9], 0x0
1405a3a9b: c6 83 da 00 00 00 01        	mov	byte ptr [rbx + 0xda], 0x1
1405a3aa2: 48 8d 8b 70 03 00 00        	lea	rcx, [rbx + 0x370]
1405a3aa9: e8 d2 22 d4 ff              	call	0x1402e5d80 <.text+0x2e4d80>
1405a3aae: 31 c0                       	xor	eax, eax
1405a3ab0: 48 3b 83 90 25 00 00        	cmp	rax, qword ptr [rbx + 0x2590]
1405a3ab7: 0f 80 23 f6 ff ff           	jo	0x1405a30e0 <.text+0x5a20e0>
1405a3abd: c6 83 f4 00 00 00 00        	mov	byte ptr [rbx + 0xf4], 0x0
1405a3ac4: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405a3acb: e8 f0 f1 fe ff              	call	0x140592cc0 <.text+0x591cc0>
1405a3ad0: e9 0b f6 ff ff              	jmp	0x1405a30e0 <.text+0x5a20e0>
1405a3ad5: 48 8d 93 60 0b 00 00        	lea	rdx, [rbx + 0xb60]
1405a3adc: 48 8d 8b 40 2c 00 00        	lea	rcx, [rbx + 0x2c40]
1405a3ae3: 41 b8 c8 00 00 00           	mov	r8d, 0xc8
1405a3ae9: e8 12 01 38 02              	call	0x142923c00 <.text+0x2922c00>
1405a3aee: 0f 28 83 40 04 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x440]
1405a3af5: 0f 28 8b 50 04 00 00        	movaps	xmm1, xmmword ptr [rbx + 0x450]
1405a3afc: 0f 28 93 60 04 00 00        	movaps	xmm2, xmmword ptr [rbx + 0x460]
1405a3b03: 0f 28 9b 70 04 00 00        	movaps	xmm3, xmmword ptr [rbx + 0x470]
1405a3b0a: 0f 29 83 f0 2b 00 00        	movaps	xmmword ptr [rbx + 0x2bf0], xmm0
1405a3b11: 0f 29 8b 00 2c 00 00        	movaps	xmmword ptr [rbx + 0x2c00], xmm1
1405a3b18: 0f 29 93 10 2c 00 00        	movaps	xmmword ptr [rbx + 0x2c10], xmm2
1405a3b1f: 0f 29 9b 20 2c 00 00        	movaps	xmmword ptr [rbx + 0x2c20], xmm3
1405a3b26: 0f 28 83 80 04 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x480]
1405a3b2d: 0f 29 83 30 2c 00 00        	movaps	xmmword ptr [rbx + 0x2c30], xmm0
1405a3b34: 48 8b 93 30 01 00 00        	mov	rdx, qword ptr [rbx + 0x130]
1405a3b3b: 48 85 d2                    	test	rdx, rdx
1405a3b3e: 4c 8b a5 b0 37 00 00        	mov	r12, qword ptr [rbp + 0x37b0]
1405a3b45: 74 12                       	je	0x1405a3b59 <.text+0x5a2b59>
1405a3b47: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a3b4d: 48 8b 8b 28 01 00 00        	mov	rcx, qword ptr [rbx + 0x128]
1405a3b54: e8 c7 00 07 00              	call	0x140613c20 <.text+0x612c20>
1405a3b59: 4c 89 f8                    	mov	rax, r15
1405a3b5c: 48 f7 d8                    	neg	rax
1405a3b5f: 0f 81 0c 01 00 00           	jno	0x1405a3c71 <.text+0x5a2c71>
1405a3b65: 49 89 f0                    	mov	r8, rsi
1405a3b68: 4c 89 83 90 25 00 00        	mov	qword ptr [rbx + 0x2590], r8
1405a3b6f: 48 8b 05 32 88 08 03        	mov	rax, qword ptr [rip + 0x3088832] # 0x14362c3a8
1405a3b76: 48 8b 00                    	mov	rax, qword ptr [rax]
1405a3b79: 48 83 f8 02                 	cmp	rax, 0x2
1405a3b7d: 0f 82 d3 00 00 00           	jb	0x1405a3c56 <.text+0x5a2c56>
1405a3b83: 48 89 bb 40 04 00 00        	mov	qword ptr [rbx + 0x440], rdi
1405a3b8a: 48 8d 05 8f 90 7a 00        	lea	rax, [rip + 0x7a908f]   # 0x140d4cc20 <.text+0xd4bc20>
1405a3b91: 48 89 83 48 04 00 00        	mov	qword ptr [rbx + 0x448], rax
1405a3b98: 48 8d 0d 21 bf 45 02        	lea	rcx, [rip + 0x245bf21]  # 0x1429ffac0
1405a3b9f: e8 dc a1 0d 00              	call	0x14067dd80 <.text+0x67cd80>
1405a3ba4: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1405a3ba7: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
1405a3baa: 48 c7 83 30 0b 00 00 02 00 00 00    	mov	qword ptr [rbx + 0xb30], 0x2
1405a3bb5: 48 8d 0d 34 b8 45 02        	lea	rcx, [rip + 0x245b834]  # 0x1429ff3f0
1405a3bbc: 48 89 8b 38 0b 00 00        	mov	qword ptr [rbx + 0xb38], rcx
1405a3bc3: 48 c7 83 40 0b 00 00 14 00 00 00    	mov	qword ptr [rbx + 0xb40], 0x14
1405a3bce: 48 8d 15 db be 45 02        	lea	rdx, [rip + 0x245bedb]  # 0x1429ffab0
1405a3bd5: 48 89 93 50 0b 00 00        	mov	qword ptr [rbx + 0xb50], rdx
1405a3bdc: 48 c7 83 58 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb58], 0x1
1405a3be7: 4c 89 b3 60 0b 00 00        	mov	qword ptr [rbx + 0xb60], r14
1405a3bee: 48 c7 83 68 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb68], 0x1
1405a3bf9: 48 c7 83 70 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb70], 0x0
1405a3c04: 48 c7 83 00 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb00], 0x0
1405a3c0f: 48 89 8b 08 0b 00 00        	mov	qword ptr [rbx + 0xb08], rcx
1405a3c16: 48 c7 83 10 0b 00 00 14 00 00 00    	mov	qword ptr [rbx + 0xb10], 0x14
1405a3c21: 48 c7 83 18 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb18], 0x0
1405a3c2c: 0f 11 83 20 0b 00 00        	movups	xmmword ptr [rbx + 0xb20], xmm0
1405a3c33: c7 83 48 0b 00 00 01 00 00 00       	mov	dword ptr [rbx + 0xb48], 0x1
1405a3c3d: 89 83 4c 0b 00 00           	mov	dword ptr [rbx + 0xb4c], eax
1405a3c43: 48 8d 8b 70 03 00 00        	lea	rcx, [rbx + 0x370]
1405a3c4a: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a3c51: e8 da 66 7a 00              	call	0x140d4a330 <.text+0xd49330>
1405a3c56: c6 83 c7 00 00 00 01        	mov	byte ptr [rbx + 0xc7], 0x1
1405a3c5d: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405a3c64: e8 c7 8f 7a 00              	call	0x140d4cc30 <.text+0xd4bc30>
1405a3c69: 40 b6 01                    	mov	sil, 0x1
1405a3c6c: e9 a5 00 00 00              	jmp	0x1405a3d16 <.text+0x5a2d16>
1405a3c71: 4c 89 bb 90 25 00 00        	mov	qword ptr [rbx + 0x2590], r15
1405a3c78: 48 89 b3 98 25 00 00        	mov	qword ptr [rbx + 0x2598], rsi
1405a3c7f: 48 8d 8b a0 25 00 00        	lea	rcx, [rbx + 0x25a0]
1405a3c86: 48 8d 93 f0 2b 00 00        	lea	rdx, [rbx + 0x2bf0]
1405a3c8d: 41 b8 18 01 00 00           	mov	r8d, 0x118
1405a3c93: e8 68 ff 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a3c98: 48 8d b3 40 04 00 00        	lea	rsi, [rbx + 0x440]
1405a3c9f: 41 b8 28 01 00 00           	mov	r8d, 0x128
1405a3ca5: 48 89 f1                    	mov	rcx, rsi
1405a3ca8: 4c 89 e2                    	mov	rdx, r12
1405a3cab: e8 50 ff 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a3cb0: 0f 10 83 c8 2c 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2cc8]
1405a3cb7: 0f 11 83 28 05 00 00        	movups	xmmword ptr [rbx + 0x528], xmm0
1405a3cbe: 48 8d bb 00 0b 00 00        	lea	rdi, [rbx + 0xb00]
1405a3cc5: 41 b8 28 01 00 00           	mov	r8d, 0x128
1405a3ccb: 48 89 f9                    	mov	rcx, rdi
1405a3cce: 48 89 f2                    	mov	rdx, rsi
1405a3cd1: e8 2a ff 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a3cd6: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a3cdd: 48 89 fa                    	mov	rdx, rdi
1405a3ce0: e8 2b 5b b7 ff              	call	0x140119810 <.text+0x118810>
1405a3ce5: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405a3cec: e8 8f 9e 05 00              	call	0x1405fdb80 <.text+0x5fcb80>
1405a3cf1: 48 8b 93 58 26 00 00        	mov	rdx, qword ptr [rbx + 0x2658]
1405a3cf8: 48 89 d0                    	mov	rax, rdx
1405a3cfb: 48 f7 d8                    	neg	rax
1405a3cfe: 70 14                       	jo	0x1405a3d14 <.text+0x5a2d14>
1405a3d00: 73 12                       	jae	0x1405a3d14 <.text+0x5a2d14>
1405a3d02: 48 8b 8b 60 26 00 00        	mov	rcx, qword ptr [rbx + 0x2660]
1405a3d09: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a3d0f: e8 0c ff 06 00              	call	0x140613c20 <.text+0x612c20>
1405a3d14: 31 f6                       	xor	esi, esi
1405a3d16: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a3d1d: 40 88 b3 c7 00 00 00        	mov	byte ptr [rbx + 0xc7], sil
1405a3d24: b2 01                       	mov	dl, 0x1
1405a3d26: e8 85 cc b7 ff              	call	0x1401209b0 <.text+0x11f9b0>
1405a3d2b: 40 88 b3 c7 00 00 00        	mov	byte ptr [rbx + 0xc7], sil
1405a3d32: 4c 8d 05 9f bd 45 02        	lea	r8, [rip + 0x245bd9f]   # 0x1429ffad8
1405a3d39: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a3d40: b2 01                       	mov	dl, 0x1
1405a3d42: e8 59 e8 ba ff              	call	0x1401525a0 <.text+0x1515a0>
1405a3d47: 89 b3 30 01 00 00           	mov	dword ptr [rbx + 0x130], esi
1405a3d4d: 48 8b 0d 1c 86 08 03        	mov	rcx, qword ptr [rip + 0x308861c] # 0x14362c370
1405a3d54: 4c 8d 05 0d 7b 3c 02        	lea	r8, [rip + 0x23c7b0d]   # 0x14296b868
1405a3d5b: 48 8d 93 70 03 00 00        	lea	rdx, [rbx + 0x370]
1405a3d62: e8 b9 0f dc ff              	call	0x140364d20 <.text+0x363d20>
1405a3d67: 49 89 d0                    	mov	r8, rdx
1405a3d6a: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a3d71: 48 89 c2                    	mov	rdx, rax
1405a3d74: e8 37 2a b3 ff              	call	0x1400d67b0 <.text+0xd57b0>
1405a3d79: 0f 10 83 60 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb60]
1405a3d80: 0f 29 83 a0 04 00 00        	movaps	xmmword ptr [rbx + 0x4a0], xmm0
1405a3d87: 0f 10 83 50 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb50]
1405a3d8e: 0f 29 83 90 04 00 00        	movaps	xmmword ptr [rbx + 0x490], xmm0
1405a3d95: 0f 10 83 40 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb40]
1405a3d9c: 0f 29 83 80 04 00 00        	movaps	xmmword ptr [rbx + 0x480], xmm0
1405a3da3: 0f 10 83 00 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb00]
1405a3daa: 0f 10 8b 10 0b 00 00        	movups	xmm1, xmmword ptr [rbx + 0xb10]
1405a3db1: 0f 10 93 20 0b 00 00        	movups	xmm2, xmmword ptr [rbx + 0xb20]
1405a3db8: 0f 10 9b 30 0b 00 00        	movups	xmm3, xmmword ptr [rbx + 0xb30]
1405a3dbf: 0f 29 9b 70 04 00 00        	movaps	xmmword ptr [rbx + 0x470], xmm3
1405a3dc6: 0f 29 93 60 04 00 00        	movaps	xmmword ptr [rbx + 0x460], xmm2
1405a3dcd: 0f 29 8b 50 04 00 00        	movaps	xmmword ptr [rbx + 0x450], xmm1
1405a3dd4: 0f 29 83 40 04 00 00        	movaps	xmmword ptr [rbx + 0x440], xmm0
1405a3ddb: 8b bb 30 01 00 00           	mov	edi, dword ptr [rbx + 0x130]
1405a3de1: 40 88 bb c7 00 00 00        	mov	byte ptr [rbx + 0xc7], dil
1405a3de8: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a3def: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a3df6: e8 f5 38 d8 ff              	call	0x1403276f0 <.text+0x3266f0>
1405a3dfb: 48 8b 93 30 22 00 00        	mov	rdx, qword ptr [rbx + 0x2230]
1405a3e02: 48 85 d2                    	test	rdx, rdx
1405a3e05: 74 12                       	je	0x1405a3e19 <.text+0x5a2e19>
1405a3e07: 48 8b 8b 38 22 00 00        	mov	rcx, qword ptr [rbx + 0x2238]
1405a3e0e: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a3e14: e8 07 fe 06 00              	call	0x140613c20 <.text+0x612c20>
1405a3e19: 48 8b 93 f0 1f 00 00        	mov	rdx, qword ptr [rbx + 0x1ff0]
1405a3e20: 48 85 d2                    	test	rdx, rdx
1405a3e23: 74 12                       	je	0x1405a3e37 <.text+0x5a2e37>
1405a3e25: 48 8b 8b f8 1f 00 00        	mov	rcx, qword ptr [rbx + 0x1ff8]
1405a3e2c: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a3e32: e8 e9 fd 06 00              	call	0x140613c20 <.text+0x612c20>
1405a3e37: 0f 28 83 f0 0a 00 00        	movaps	xmm0, xmmword ptr [rbx + 0xaf0]
1405a3e3e: 0f 29 83 50 0b 00 00        	movaps	xmmword ptr [rbx + 0xb50], xmm0
1405a3e45: 0f 28 83 e0 0a 00 00        	movaps	xmm0, xmmword ptr [rbx + 0xae0]
1405a3e4c: 0f 29 83 40 0b 00 00        	movaps	xmmword ptr [rbx + 0xb40], xmm0
1405a3e53: 0f 28 83 a0 0a 00 00        	movaps	xmm0, xmmword ptr [rbx + 0xaa0]
1405a3e5a: 0f 28 8b b0 0a 00 00        	movaps	xmm1, xmmword ptr [rbx + 0xab0]
1405a3e61: 0f 28 93 c0 0a 00 00        	movaps	xmm2, xmmword ptr [rbx + 0xac0]
1405a3e68: 0f 28 9b d0 0a 00 00        	movaps	xmm3, xmmword ptr [rbx + 0xad0]
1405a3e6f: 0f 29 9b 30 0b 00 00        	movaps	xmmword ptr [rbx + 0xb30], xmm3
1405a3e76: 0f 29 93 20 0b 00 00        	movaps	xmmword ptr [rbx + 0xb20], xmm2
1405a3e7d: 0f 29 8b 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm1
1405a3e84: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
1405a3e8b: e8 b0 98 7a 00              	call	0x140d4d740 <.text+0xd4c740>
1405a3e90: 48 89 c6                    	mov	rsi, rax
1405a3e93: f6 00 01                    	test	byte ptr [rax], 0x1
1405a3e96: 89 bb 30 01 00 00           	mov	dword ptr [rbx + 0x130], edi
1405a3e9c: 74 19                       	je	0x1405a3eb7 <.text+0x5a2eb7>
1405a3e9e: 48 89 f1                    	mov	rcx, rsi
1405a3ea1: 48 83 c1 08                 	add	rcx, 0x8
1405a3ea5: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
1405a3ea9: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1405a3ead: 44 0f b6 ab 07 01 00 00     	movzx	r13d, byte ptr [rbx + 0x107]
1405a3eb5: eb 22                       	jmp	0x1405a3ed9 <.text+0x5a2ed9>
1405a3eb7: e8 44 0f 7c 00              	call	0x140d64e00 <.text+0xd63e00>
1405a3ebc: 44 8a ab 07 01 00 00        	mov	r13b, byte ptr [rbx + 0x107]
1405a3ec3: 48 89 f1                    	mov	rcx, rsi
1405a3ec6: 48 83 c1 08                 	add	rcx, 0x8
1405a3eca: 48 c7 06 01 00 00 00        	mov	qword ptr [rsi], 0x1
1405a3ed1: 48 89 46 08                 	mov	qword ptr [rsi + 0x8], rax
1405a3ed5: 48 89 56 10                 	mov	qword ptr [rsi + 0x10], rdx
1405a3ed9: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1405a3edd: 4c 89 01                    	mov	qword ptr [rcx], r8
1405a3ee0: 0f 28 83 50 0b 00 00        	movaps	xmm0, xmmword ptr [rbx + 0xb50]
1405a3ee7: 0f 11 83 f0 21 00 00        	movups	xmmword ptr [rbx + 0x21f0], xmm0
1405a3eee: 0f 28 83 40 0b 00 00        	movaps	xmm0, xmmword ptr [rbx + 0xb40]
1405a3ef5: 0f 11 83 e0 21 00 00        	movups	xmmword ptr [rbx + 0x21e0], xmm0
1405a3efc: 0f 28 83 00 0b 00 00        	movaps	xmm0, xmmword ptr [rbx + 0xb00]
1405a3f03: 0f 28 8b 10 0b 00 00        	movaps	xmm1, xmmword ptr [rbx + 0xb10]
1405a3f0a: 0f 28 93 20 0b 00 00        	movaps	xmm2, xmmword ptr [rbx + 0xb20]
1405a3f11: 0f 28 9b 30 0b 00 00        	movaps	xmm3, xmmword ptr [rbx + 0xb30]
1405a3f18: 0f 11 9b d0 21 00 00        	movups	xmmword ptr [rbx + 0x21d0], xmm3
1405a3f1f: 0f 11 93 c0 21 00 00        	movups	xmmword ptr [rbx + 0x21c0], xmm2
1405a3f26: 0f 11 8b b0 21 00 00        	movups	xmmword ptr [rbx + 0x21b0], xmm1
1405a3f2d: 0f 11 83 a0 21 00 00        	movups	xmmword ptr [rbx + 0x21a0], xmm0
1405a3f34: 0f 11 bb 00 22 00 00        	movups	xmmword ptr [rbx + 0x2200], xmm7
1405a3f3b: 0f 11 b3 10 22 00 00        	movups	xmmword ptr [rbx + 0x2210], xmm6
1405a3f42: 48 89 83 20 22 00 00        	mov	qword ptr [rbx + 0x2220], rax
1405a3f49: 48 89 93 28 22 00 00        	mov	qword ptr [rbx + 0x2228], rdx
1405a3f50: 48 c7 83 88 21 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x2188], 0x0
1405a3f5b: 48 c7 83 90 21 00 00 08 00 00 00    	mov	qword ptr [rbx + 0x2190], 0x8
1405a3f66: 48 c7 83 98 21 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x2198], 0x0
1405a3f71: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1405a3f7b: 48 89 83 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], rax
1405a3f82: 48 c7 83 00 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb00], 0x0
1405a3f8d: c6 83 28 0b 00 00 00        	mov	byte ptr [rbx + 0xb28], 0x0
1405a3f94: 0f b6 05 6e 0c 09 03        	movzx	eax, byte ptr [rip + 0x3090c6e] # 0x143634c09
1405a3f9b: b9 12 00 00 00              	mov	ecx, 0x12
1405a3fa0: ba 01 00 00 00              	mov	edx, 0x1
1405a3fa5: e8 66 fc 06 00              	call	0x140613c10 <.text+0x612c10>
1405a3faa: 48 85 c0                    	test	rax, rax
1405a3fad: 48 89 83 28 01 00 00        	mov	qword ptr [rbx + 0x128], rax
1405a3fb4: 0f 84 48 2e 00 00           	je	0x1405a6e02 <.text+0x5a5e02>
1405a3fba: 0f 10 05 2f bb 45 02        	movups	xmm0, xmmword ptr [rip + 0x245bb2f] # 0x1429ffaf0
1405a3fc1: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
1405a3fc4: 66 c7 40 10 65 72           	mov	word ptr [rax + 0x10], 0x7265
1405a3fca: 48 c7 83 90 25 00 00 12 00 00 00    	mov	qword ptr [rbx + 0x2590], 0x12
1405a3fd5: 48 89 83 98 25 00 00        	mov	qword ptr [rbx + 0x2598], rax
1405a3fdc: 48 c7 83 a0 25 00 00 12 00 00 00    	mov	qword ptr [rbx + 0x25a0], 0x12
1405a3fe7: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a3fee: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a3ff5: 4c 8d b3 90 25 00 00        	lea	r14, [rbx + 0x2590]
1405a3ffc: 4d 89 f0                    	mov	r8, r14
1405a3fff: e8 ec 18 7b 00              	call	0x140d558f0 <.text+0xd548f0>
1405a4004: 4c 8b 83 58 01 00 00        	mov	r8, qword ptr [rbx + 0x158]
1405a400b: c6 83 d4 00 00 00 01        	mov	byte ptr [rbx + 0xd4], 0x1
1405a4012: c6 83 d5 00 00 00 01        	mov	byte ptr [rbx + 0xd5], 0x1
1405a4019: c6 83 d6 00 00 00 01        	mov	byte ptr [rbx + 0xd6], 0x1
1405a4020: c6 83 d7 00 00 00 00        	mov	byte ptr [rbx + 0xd7], 0x0
1405a4027: c6 83 d8 00 00 00 01        	mov	byte ptr [rbx + 0xd8], 0x1
1405a402e: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a4035: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a403c: e8 4f 6e ce ff              	call	0x14028ae90 <.text+0x289e90>
1405a4041: 48 83 bb 00 0b 00 00 00     	cmp	qword ptr [rbx + 0xb00], 0x0
1405a4049: 0f 84 ce 2d 00 00           	je	0x1405a6e1d <.text+0x5a5e1d>
1405a404f: 48 8b 83 10 0b 00 00        	mov	rax, qword ptr [rbx + 0xb10]
1405a4056: 48 89 83 a0 02 00 00        	mov	qword ptr [rbx + 0x2a0], rax
1405a405d: 0f 10 83 00 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb00]
1405a4064: 0f 29 83 90 02 00 00        	movaps	xmmword ptr [rbx + 0x290], xmm0
1405a406b: 48 8b 85 c0 37 00 00        	mov	rax, qword ptr [rbp + 0x37c0]
1405a4072: 80 b8 4a 01 00 00 00        	cmp	byte ptr [rax + 0x14a], 0x0
1405a4079: 74 21                       	je	0x1405a409c <.text+0x5a309c>
1405a407b: c6 83 e4 00 00 00 01        	mov	byte ptr [rbx + 0xe4], 0x1
1405a4082: c6 83 e5 00 00 00 01        	mov	byte ptr [rbx + 0xe5], 0x1
1405a4089: c6 83 e6 00 00 00 01        	mov	byte ptr [rbx + 0xe6], 0x1
1405a4090: c6 83 e7 00 00 00 01        	mov	byte ptr [rbx + 0xe7], 0x1
1405a4097: e8 44 1b dc ff              	call	0x140365be0 <.text+0x364be0>
1405a409c: 48 c7 83 80 0c 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xc80], 0x1
1405a40a7: 48 c7 83 88 0c 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xc88], 0x1
1405a40b2: c6 83 90 0c 00 00 00        	mov	byte ptr [rbx + 0xc90], 0x0
1405a40b9: 0f 57 c0                    	xorps	xmm0, xmm0
1405a40bc: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
1405a40c3: 0f 29 83 80 0b 00 00        	movaps	xmmword ptr [rbx + 0xb80], xmm0
1405a40ca: 66 c7 83 00 0c 00 00 00 00  	mov	word ptr [rbx + 0xc00], 0x0
1405a40d3: 48 c7 83 08 0c 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xc08], 0x0
1405a40de: 48 c7 83 10 0c 00 00 08 00 00 00    	mov	qword ptr [rbx + 0xc10], 0x8
1405a40e9: 0f 11 83 18 0c 00 00        	movups	xmmword ptr [rbx + 0xc18], xmm0
1405a40f0: 48 c7 83 28 0c 00 00 08 00 00 00    	mov	qword ptr [rbx + 0xc28], 0x8
1405a40fb: 48 c7 83 30 0c 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xc30], 0x0
1405a4106: c6 83 38 0c 00 00 01        	mov	byte ptr [rbx + 0xc38], 0x1
1405a410d: 0f b6 05 f5 0a 09 03        	movzx	eax, byte ptr [rip + 0x3090af5] # 0x143634c09
1405a4114: b9 00 02 00 00              	mov	ecx, 0x200
1405a4119: ba 80 00 00 00              	mov	edx, 0x80
1405a411e: e8 ed fa 06 00              	call	0x140613c10 <.text+0x612c10>
1405a4123: 48 85 c0                    	test	rax, rax
1405a4126: 0f 84 2f 2d 00 00           	je	0x1405a6e5b <.text+0x5a5e5b>
1405a412c: 48 89 c6                    	mov	rsi, rax
1405a412f: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a4136: 41 b8 00 02 00 00           	mov	r8d, 0x200
1405a413c: 48 89 c1                    	mov	rcx, rax
1405a413f: e8 bc fa 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a4144: 48 c7 83 e0 02 00 00 01 00 00 00    	mov	qword ptr [rbx + 0x2e0], 0x1
1405a414f: 48 89 b3 e8 02 00 00        	mov	qword ptr [rbx + 0x2e8], rsi
1405a4156: 48 c7 83 f0 02 00 00 01 00 00 00    	mov	qword ptr [rbx + 0x2f0], 0x1
1405a4161: 48 89 b3 f8 02 00 00        	mov	qword ptr [rbx + 0x2f8], rsi
1405a4168: 45 84 ed                    	test	r13b, r13b
1405a416b: 0f 84 80 02 00 00           	je	0x1405a43f1 <.text+0x5a33f1>
1405a4171: c6 83 c2 00 00 00 01        	mov	byte ptr [rbx + 0xc2], 0x1
1405a4178: c6 83 c3 00 00 00 01        	mov	byte ptr [rbx + 0xc3], 0x1
1405a417f: c6 83 c4 00 00 00 01        	mov	byte ptr [rbx + 0xc4], 0x1
1405a4186: c6 83 c5 00 00 00 01        	mov	byte ptr [rbx + 0xc5], 0x1
1405a418d: c6 83 c6 00 00 00 01        	mov	byte ptr [rbx + 0xc6], 0x1
1405a4194: 48 ba a3 8b f6 7e 77 b5 d7 ae       	movabs	rdx, -0x51284a888109745d
1405a419e: 49 b8 72 2b 69 b0 99 a1 34 58       	movabs	r8, 0x5834a199b0692b72
1405a41a8: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
1405a41af: e8 3c 95 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a41b4: 48 85 c0                    	test	rax, rax
1405a41b7: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a41be: 0f 84 2d 24 00 00           	je	0x1405a65f1 <.text+0x5a55f1>
1405a41c4: 48 39 91 18 02 00 00        	cmp	qword ptr [rcx + 0x218], rdx
1405a41cb: 0f 86 20 24 00 00           	jbe	0x1405a65f1 <.text+0x5a55f1>
1405a41d1: 48 8b 81 10 02 00 00        	mov	rax, qword ptr [rcx + 0x210]
1405a41d8: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a41dc: 48 85 c0                    	test	rax, rax
1405a41df: 0f 84 0c 24 00 00           	je	0x1405a65f1 <.text+0x5a55f1>
1405a41e5: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
1405a41ec: 48 f7 d0                    	not	rax
1405a41ef: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a41f3: 48 c1 e0 05                 	shl	rax, 0x5
1405a41f7: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a41fd: 0f 84 ee 23 00 00           	je	0x1405a65f1 <.text+0x5a55f1>
1405a4203: 48 01 c1                    	add	rcx, rax
1405a4206: c6 83 c2 00 00 00 01        	mov	byte ptr [rbx + 0xc2], 0x1
1405a420d: c6 83 c3 00 00 00 01        	mov	byte ptr [rbx + 0xc3], 0x1
1405a4214: c6 83 c4 00 00 00 01        	mov	byte ptr [rbx + 0xc4], 0x1
1405a421b: c6 83 c5 00 00 00 01        	mov	byte ptr [rbx + 0xc5], 0x1
1405a4222: c6 83 c6 00 00 00 01        	mov	byte ptr [rbx + 0xc6], 0x1
1405a4229: 31 d2                       	xor	edx, edx
1405a422b: e8 e0 b4 75 00              	call	0x140cff710 <.text+0xcfe710>
1405a4230: 48 8b 08                    	mov	rcx, qword ptr [rax]
1405a4233: c6 83 c2 00 00 00 01        	mov	byte ptr [rbx + 0xc2], 0x1
1405a423a: c6 83 c3 00 00 00 01        	mov	byte ptr [rbx + 0xc3], 0x1
1405a4241: c6 83 c4 00 00 00 01        	mov	byte ptr [rbx + 0xc4], 0x1
1405a4248: c6 83 c5 00 00 00 01        	mov	byte ptr [rbx + 0xc5], 0x1
1405a424f: c6 83 c6 00 00 00 01        	mov	byte ptr [rbx + 0xc6], 0x1
1405a4256: e8 25 9b 0d 00              	call	0x14067dd80 <.text+0x67cd80>
1405a425b: c6 83 c2 00 00 00 01        	mov	byte ptr [rbx + 0xc2], 0x1
1405a4262: c6 83 c3 00 00 00 01        	mov	byte ptr [rbx + 0xc3], 0x1
1405a4269: c6 83 c4 00 00 00 01        	mov	byte ptr [rbx + 0xc4], 0x1
1405a4270: c6 83 c5 00 00 00 01        	mov	byte ptr [rbx + 0xc5], 0x1
1405a4277: c6 83 c6 00 00 00 01        	mov	byte ptr [rbx + 0xc6], 0x1
1405a427e: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a4285: 48 89 c2                    	mov	rdx, rax
1405a4288: e8 63 87 b9 ff              	call	0x14013c9f0 <.text+0x13b9f0>
1405a428d: c6 83 c2 00 00 00 01        	mov	byte ptr [rbx + 0xc2], 0x1
1405a4294: c6 83 c3 00 00 00 01        	mov	byte ptr [rbx + 0xc3], 0x1
1405a429b: c6 83 c4 00 00 00 01        	mov	byte ptr [rbx + 0xc4], 0x1
1405a42a2: c6 83 c5 00 00 00 01        	mov	byte ptr [rbx + 0xc5], 0x1
1405a42a9: c6 83 c6 00 00 00 01        	mov	byte ptr [rbx + 0xc6], 0x1
1405a42b0: 48 ba 1c d6 a9 4e 06 3d f9 d8       	movabs	rdx, -0x2706c2f9b15629e4
1405a42ba: 49 b8 2f 83 d9 66 70 61 47 64       	movabs	r8, 0x6447617066d9832f
1405a42c4: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
1405a42cb: e8 20 94 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a42d0: 48 85 c0                    	test	rax, rax
1405a42d3: 4c 8b 83 c8 00 00 00        	mov	r8, qword ptr [rbx + 0xc8]
1405a42da: 0f 84 cf 23 00 00           	je	0x1405a66af <.text+0x5a56af>
1405a42e0: 49 39 90 18 02 00 00        	cmp	qword ptr [r8 + 0x218], rdx
1405a42e7: 0f 86 c2 23 00 00           	jbe	0x1405a66af <.text+0x5a56af>
1405a42ed: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
1405a42f4: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1405a42f8: 48 85 c9                    	test	rcx, rcx
1405a42fb: 0f 84 ae 23 00 00           	je	0x1405a66af <.text+0x5a56af>
1405a4301: 49 8b 80 e0 01 00 00        	mov	rax, qword ptr [r8 + 0x1e0]
1405a4308: 48 f7 d1                    	not	rcx
1405a430b: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1405a430f: 48 c1 e1 05                 	shl	rcx, 0x5
1405a4313: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1405a4319: 0f 84 90 23 00 00           	je	0x1405a66af <.text+0x5a56af>
1405a431f: 48 01 c8                    	add	rax, rcx
1405a4322: 48 8b 70 10                 	mov	rsi, qword ptr [rax + 0x10]
1405a4326: 41 8b 88 b0 05 00 00        	mov	ecx, dword ptr [r8 + 0x5b0]
1405a432d: 89 48 5c                    	mov	dword ptr [rax + 0x5c], ecx
1405a4330: 48 8b 7e 30                 	mov	rdi, qword ptr [rsi + 0x30]
1405a4334: 4c 8b 7e 40                 	mov	r15, qword ptr [rsi + 0x40]
1405a4338: 48 3b 7e 20                 	cmp	rdi, qword ptr [rsi + 0x20]
1405a433c: 75 33                       	jne	0x1405a4371 <.text+0x5a3371>
1405a433e: 48 8d 4e 20                 	lea	rcx, [rsi + 0x20]
1405a4342: c6 83 c2 00 00 00 01        	mov	byte ptr [rbx + 0xc2], 0x1
1405a4349: c6 83 c3 00 00 00 01        	mov	byte ptr [rbx + 0xc3], 0x1
1405a4350: c6 83 c4 00 00 00 01        	mov	byte ptr [rbx + 0xc4], 0x1
1405a4357: c6 83 c5 00 00 00 01        	mov	byte ptr [rbx + 0xc5], 0x1
1405a435e: c6 83 c6 00 00 00 01        	mov	byte ptr [rbx + 0xc6], 0x1
1405a4365: 48 8d 15 bc ed 44 02        	lea	rdx, [rip + 0x244edbc]  # 0x1429f3128
1405a436c: e8 ff e4 fb ff              	call	0x140562870 <.text+0x561870>
1405a4371: 48 8b 46 28                 	mov	rax, qword ptr [rsi + 0x28]
1405a4375: 4c 89 3c f8                 	mov	qword ptr [rax + 8*rdi], r15
1405a4379: 48 ff c7                    	inc	rdi
1405a437c: 48 89 7e 30                 	mov	qword ptr [rsi + 0x30], rdi
1405a4380: 48 ff 46 40                 	inc	qword ptr [rsi + 0x40]
1405a4384: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a438b: 41 b8 58 06 00 00           	mov	r8d, 0x658
1405a4391: 48 8b 93 c8 00 00 00        	mov	rdx, qword ptr [rbx + 0xc8]
1405a4398: e8 63 f8 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a439d: 0f 10 83 f0 02 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2f0]
1405a43a4: 0f 11 83 80 02 00 00        	movups	xmmword ptr [rbx + 0x280], xmm0
1405a43ab: 0f 10 83 28 03 00 00        	movups	xmm0, xmmword ptr [rbx + 0x328]
1405a43b2: 0f 11 83 e8 24 00 00        	movups	xmmword ptr [rbx + 0x24e8], xmm0
1405a43b9: 48 8b 83 60 01 00 00        	mov	rax, qword ptr [rbx + 0x160]
1405a43c0: 48 83 f8 02                 	cmp	rax, 0x2
1405a43c4: 0f 84 fb 1b 00 00           	je	0x1405a5fc5 <.text+0x5a4fc5>
1405a43ca: 83 f8 01                    	cmp	eax, 0x1
1405a43cd: 0f 85 09 1c 00 00           	jne	0x1405a5fdc <.text+0x5a4fdc>
1405a43d3: 48 8b 8b 68 01 00 00        	mov	rcx, qword ptr [rbx + 0x168]
1405a43da: f0                          	lock
1405a43db: 48 ff 81 80 01 00 00        	inc	qword ptr [rcx + 0x180]
1405a43e2: 0f 8f 09 1c 00 00           	jg	0x1405a5ff1 <.text+0x5a4ff1>
1405a43e8: b9 07 00 00 00              	mov	ecx, 0x7
1405a43ed: cd 29                       	int	0x29
1405a43ef: 0f 0b                       	ud2
1405a43f1: 4c 8b bb 38 01 00 00        	mov	r15, qword ptr [rbx + 0x138]
1405a43f8: f0                          	lock
1405a43f9: 49 ff 07                    	inc	qword ptr [r15]
1405a43fc: 0f 8e 3e 2c 00 00           	jle	0x1405a7040 <.text+0x5a6040>
1405a4402: 4c 89 bb 30 22 00 00        	mov	qword ptr [rbx + 0x2230], r15
1405a4409: c6 83 1f 01 00 00 01        	mov	byte ptr [rbx + 0x11f], 0x1
1405a4410: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a4417: e8 f4 93 70 00              	call	0x140cad810 <.text+0xcac810>
1405a441c: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4423: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a442a: e8 a1 1a 3a 00              	call	0x140945ed0 <.text+0x944ed0>
1405a442f: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4436: 48 ba a6 12 23 8c a9 dc 4d 86       	movabs	rdx, -0x79b2235673dced5a
1405a4440: 49 b8 4e 48 9a 9e 36 e9 b0 27       	movabs	r8, 0x27b0e9369e9a484e
1405a444a: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
1405a4451: e8 9a 92 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a4456: 48 85 c0                    	test	rax, rax
1405a4459: 4c 8b 83 c8 00 00 00        	mov	r8, qword ptr [rbx + 0xc8]
1405a4460: 0f 84 bf 21 00 00           	je	0x1405a6625 <.text+0x5a5625>
1405a4466: 49 39 90 18 02 00 00        	cmp	qword ptr [r8 + 0x218], rdx
1405a446d: 0f 86 b2 21 00 00           	jbe	0x1405a6625 <.text+0x5a5625>
1405a4473: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
1405a447a: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1405a447e: 48 85 c9                    	test	rcx, rcx
1405a4481: 0f 84 9e 21 00 00           	je	0x1405a6625 <.text+0x5a5625>
1405a4487: 49 8b 80 e0 01 00 00        	mov	rax, qword ptr [r8 + 0x1e0]
1405a448e: 48 f7 d1                    	not	rcx
1405a4491: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1405a4495: 48 c1 e1 05                 	shl	rcx, 0x5
1405a4499: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1405a449f: 0f 84 80 21 00 00           	je	0x1405a6625 <.text+0x5a5625>
1405a44a5: 48 01 c8                    	add	rax, rcx
1405a44a8: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
1405a44ac: 44 8b 20                    	mov	r12d, dword ptr [rax]
1405a44af: 44 8b 68 04                 	mov	r13d, dword ptr [rax + 0x4]
1405a44b3: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a44ba: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a44c1: e8 2a fb b6 ff              	call	0x140113ff0 <.text+0x112ff0>
1405a44c6: 3c 01                       	cmp	al, 0x1
1405a44c8: 0f 87 80 2a 00 00           	ja	0x1405a6f4e <.text+0x5a5f4e>
1405a44ce: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a44d5: 48 8d 15 a4 b6 45 02        	lea	rdx, [rip + 0x245b6a4]  # 0x1429ffb80
1405a44dc: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a44e3: e8 08 6f d8 ff              	call	0x14032b3f0 <.text+0x32a3f0>
1405a44e8: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a44ef: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a44f6: e8 f5 fa b6 ff              	call	0x140113ff0 <.text+0x112ff0>
1405a44fb: 3c 01                       	cmp	al, 0x1
1405a44fd: 0f 87 98 2a 00 00           	ja	0x1405a6f9b <.text+0x5a5f9b>
1405a4503: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a450a: 4c 8d 0d 87 b6 45 02        	lea	r9, [rip + 0x245b687]   # 0x1429ffb98
1405a4511: 4c 8d 83 40 04 00 00        	lea	r8, [rbx + 0x440]
1405a4518: b1 01                       	mov	cl, 0x1
1405a451a: b2 01                       	mov	dl, 0x1
1405a451c: e8 2f 8f b2 ff              	call	0x1400cd450 <.text+0xcc450>
1405a4521: 48 8d bb 78 04 00 00        	lea	rdi, [rbx + 0x478]
1405a4528: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a452f: 48 ba 5f 04 ba ac cf 82 dd bb       	movabs	rdx, -0x44227d305345fba1
1405a4539: 49 b8 36 88 49 49 a8 7b f2 ca       	movabs	r8, -0x350d8457b6b677ca
1405a4543: 48 89 f9                    	mov	rcx, rdi
1405a4546: e8 a5 91 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a454b: 48 85 c0                    	test	rax, rax
1405a454e: 74 33                       	je	0x1405a4583 <.text+0x5a3583>
1405a4550: 48 39 93 58 06 00 00        	cmp	qword ptr [rbx + 0x658], rdx
1405a4557: 76 2a                       	jbe	0x1405a4583 <.text+0x5a3583>
1405a4559: 48 8b 83 50 06 00 00        	mov	rax, qword ptr [rbx + 0x650]
1405a4560: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a4564: 48 85 c0                    	test	rax, rax
1405a4567: 74 1a                       	je	0x1405a4583 <.text+0x5a3583>
1405a4569: 48 8b 8b 20 06 00 00        	mov	rcx, qword ptr [rbx + 0x620]
1405a4570: 48 f7 d0                    	not	rax
1405a4573: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a4577: 48 c1 e0 05                 	shl	rax, 0x5
1405a457b: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a4581: 75 13                       	jne	0x1405a4596 <.text+0x5a3596>
1405a4583: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a458a: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a4591: e8 ea 1c fc ff              	call	0x140566280 <.text+0x565280>
1405a4596: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a459d: 48 ba 4f cc 30 77 91 03 5d 1c       	movabs	rdx, 0x1c5d03917730cc4f
1405a45a7: 49 b8 6c da ba 18 fb e4 29 3d       	movabs	r8, 0x3d29e4fb18bada6c
1405a45b1: 48 89 f9                    	mov	rcx, rdi
1405a45b4: e8 37 91 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a45b9: 48 85 c0                    	test	rax, rax
1405a45bc: 74 33                       	je	0x1405a45f1 <.text+0x5a35f1>
1405a45be: 48 39 93 58 06 00 00        	cmp	qword ptr [rbx + 0x658], rdx
1405a45c5: 76 2a                       	jbe	0x1405a45f1 <.text+0x5a35f1>
1405a45c7: 48 8b 83 50 06 00 00        	mov	rax, qword ptr [rbx + 0x650]
1405a45ce: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a45d2: 48 85 c0                    	test	rax, rax
1405a45d5: 74 1a                       	je	0x1405a45f1 <.text+0x5a35f1>
1405a45d7: 48 8b 8b 20 06 00 00        	mov	rcx, qword ptr [rbx + 0x620]
1405a45de: 48 f7 d0                    	not	rax
1405a45e1: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a45e5: 48 c1 e0 05                 	shl	rax, 0x5
1405a45e9: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a45ef: 75 13                       	jne	0x1405a4604 <.text+0x5a3604>
1405a45f1: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a45f8: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a45ff: e8 4c a2 fd ff              	call	0x14057e850 <.text+0x57d850>
1405a4604: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a460b: 48 ba f9 e1 c9 a4 84 46 cb 6e       	movabs	rdx, 0x6ecb4684a4c9e1f9
1405a4615: 49 b8 dc 1d c8 ba 1a e2 e2 fa       	movabs	r8, -0x51d1de54537e224
1405a461f: 48 89 f9                    	mov	rcx, rdi
1405a4622: e8 c9 90 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a4627: 48 85 c0                    	test	rax, rax
1405a462a: 74 33                       	je	0x1405a465f <.text+0x5a365f>
1405a462c: 48 39 93 58 06 00 00        	cmp	qword ptr [rbx + 0x658], rdx
1405a4633: 76 2a                       	jbe	0x1405a465f <.text+0x5a365f>
1405a4635: 48 8b 83 50 06 00 00        	mov	rax, qword ptr [rbx + 0x650]
1405a463c: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a4640: 48 85 c0                    	test	rax, rax
1405a4643: 74 1a                       	je	0x1405a465f <.text+0x5a365f>
1405a4645: 48 8b 8b 20 06 00 00        	mov	rcx, qword ptr [rbx + 0x620]
1405a464c: 48 f7 d0                    	not	rax
1405a464f: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a4653: 48 c1 e0 05                 	shl	rax, 0x5
1405a4657: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a465d: 75 13                       	jne	0x1405a4672 <.text+0x5a3672>
1405a465f: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4666: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a466d: e8 8e 13 fd ff              	call	0x140575a00 <.text+0x574a00>
1405a4672: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4679: 48 ba c0 d6 8d 67 90 c5 c2 da       	movabs	rdx, -0x253d3a6f98722940
1405a4683: 49 b8 59 4d ae 33 67 eb 30 5d       	movabs	r8, 0x5d30eb6733ae4d59
1405a468d: 48 89 f9                    	mov	rcx, rdi
1405a4690: e8 5b 90 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a4695: 48 85 c0                    	test	rax, rax
1405a4698: 74 33                       	je	0x1405a46cd <.text+0x5a36cd>
1405a469a: 48 39 93 58 06 00 00        	cmp	qword ptr [rbx + 0x658], rdx
1405a46a1: 76 2a                       	jbe	0x1405a46cd <.text+0x5a36cd>
1405a46a3: 48 8b 83 50 06 00 00        	mov	rax, qword ptr [rbx + 0x650]
1405a46aa: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a46ae: 48 85 c0                    	test	rax, rax
1405a46b1: 74 1a                       	je	0x1405a46cd <.text+0x5a36cd>
1405a46b3: 48 8b 8b 20 06 00 00        	mov	rcx, qword ptr [rbx + 0x620]
1405a46ba: 48 f7 d0                    	not	rax
1405a46bd: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a46c1: 48 c1 e0 05                 	shl	rax, 0x5
1405a46c5: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a46cb: 75 13                       	jne	0x1405a46e0 <.text+0x5a36e0>
1405a46cd: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a46d4: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a46db: e8 40 fe fd ff              	call	0x140584520 <.text+0x583520>
1405a46e0: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a46e7: 48 ba a0 1c bf 3b bb 4f 55 0a       	movabs	rdx, 0xa554fbb3bbf1ca0
1405a46f1: 49 b8 9d 8e bb 86 a9 93 b9 29       	movabs	r8, 0x29b993a986bb8e9d
1405a46fb: 48 89 f9                    	mov	rcx, rdi
1405a46fe: e8 ed 8f b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a4703: 48 85 c0                    	test	rax, rax
1405a4706: 74 33                       	je	0x1405a473b <.text+0x5a373b>
1405a4708: 48 39 93 58 06 00 00        	cmp	qword ptr [rbx + 0x658], rdx
1405a470f: 76 2a                       	jbe	0x1405a473b <.text+0x5a373b>
1405a4711: 48 8b 83 50 06 00 00        	mov	rax, qword ptr [rbx + 0x650]
1405a4718: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a471c: 48 85 c0                    	test	rax, rax
1405a471f: 74 1a                       	je	0x1405a473b <.text+0x5a373b>
1405a4721: 48 8b 8b 20 06 00 00        	mov	rcx, qword ptr [rbx + 0x620]
1405a4728: 48 f7 d0                    	not	rax
1405a472b: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a472f: 48 c1 e0 05                 	shl	rax, 0x5
1405a4733: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a4739: 75 13                       	jne	0x1405a474e <.text+0x5a374e>
1405a473b: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4742: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a4749: e8 92 50 fd ff              	call	0x1405797e0 <.text+0x5787e0>
1405a474e: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4755: 48 ba cb 64 e7 ae 73 16 7c bb       	movabs	rdx, -0x4483e98c51189b35
1405a475f: 49 b8 d6 e2 45 39 38 ee 0a 25       	movabs	r8, 0x250aee383945e2d6
1405a4769: 48 89 f9                    	mov	rcx, rdi
1405a476c: e8 7f 8f b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a4771: 48 85 c0                    	test	rax, rax
1405a4774: 74 33                       	je	0x1405a47a9 <.text+0x5a37a9>
1405a4776: 48 39 93 58 06 00 00        	cmp	qword ptr [rbx + 0x658], rdx
1405a477d: 76 2a                       	jbe	0x1405a47a9 <.text+0x5a37a9>
1405a477f: 48 8b 83 50 06 00 00        	mov	rax, qword ptr [rbx + 0x650]
1405a4786: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a478a: 48 85 c0                    	test	rax, rax
1405a478d: 74 1a                       	je	0x1405a47a9 <.text+0x5a37a9>
1405a478f: 48 8b 8b 20 06 00 00        	mov	rcx, qword ptr [rbx + 0x620]
1405a4796: 48 f7 d0                    	not	rax
1405a4799: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a479d: 48 c1 e0 05                 	shl	rax, 0x5
1405a47a1: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a47a7: 75 13                       	jne	0x1405a47bc <.text+0x5a37bc>
1405a47a9: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a47b0: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a47b7: e8 e4 3f fc ff              	call	0x1405687a0 <.text+0x5677a0>
1405a47bc: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a47c3: 48 ba 51 15 c9 59 aa d6 25 08       	movabs	rdx, 0x825d6aa59c91551
1405a47cd: 49 b8 b6 d5 be e6 cf af 73 e0       	movabs	r8, -0x1f8c503019412a4a
1405a47d7: 48 89 f9                    	mov	rcx, rdi
1405a47da: e8 11 8f b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a47df: 48 85 c0                    	test	rax, rax
1405a47e2: 74 33                       	je	0x1405a4817 <.text+0x5a3817>
1405a47e4: 48 39 93 58 06 00 00        	cmp	qword ptr [rbx + 0x658], rdx
1405a47eb: 76 2a                       	jbe	0x1405a4817 <.text+0x5a3817>
1405a47ed: 48 8b 83 50 06 00 00        	mov	rax, qword ptr [rbx + 0x650]
1405a47f4: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a47f8: 48 85 c0                    	test	rax, rax
1405a47fb: 74 1a                       	je	0x1405a4817 <.text+0x5a3817>
1405a47fd: 48 8b 8b 20 06 00 00        	mov	rcx, qword ptr [rbx + 0x620]
1405a4804: 48 f7 d0                    	not	rax
1405a4807: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a480b: 48 c1 e0 05                 	shl	rax, 0x5
1405a480f: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a4815: 75 13                       	jne	0x1405a482a <.text+0x5a382a>
1405a4817: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a481e: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a4825: e8 c6 30 fd ff              	call	0x1405778f0 <.text+0x5768f0>
1405a482a: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4831: 48 ba 36 26 d5 31 c3 1a 3f 92       	movabs	rdx, -0x6dc0e53cce2ad9ca
1405a483b: 49 b8 f6 2c 93 ff ea c9 14 41       	movabs	r8, 0x4114c9eaff932cf6
1405a4845: 48 89 f9                    	mov	rcx, rdi
1405a4848: e8 a3 8e b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a484d: 48 85 c0                    	test	rax, rax
1405a4850: 74 33                       	je	0x1405a4885 <.text+0x5a3885>
1405a4852: 48 39 93 58 06 00 00        	cmp	qword ptr [rbx + 0x658], rdx
1405a4859: 76 2a                       	jbe	0x1405a4885 <.text+0x5a3885>
1405a485b: 48 8b 83 50 06 00 00        	mov	rax, qword ptr [rbx + 0x650]
1405a4862: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a4866: 48 85 c0                    	test	rax, rax
1405a4869: 74 1a                       	je	0x1405a4885 <.text+0x5a3885>
1405a486b: 48 8b 8b 20 06 00 00        	mov	rcx, qword ptr [rbx + 0x620]
1405a4872: 48 f7 d0                    	not	rax
1405a4875: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a4879: 48 c1 e0 05                 	shl	rax, 0x5
1405a487d: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a4883: 75 13                       	jne	0x1405a4898 <.text+0x5a3898>
1405a4885: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a488c: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a4893: e8 b8 d9 fc ff              	call	0x140572250 <.text+0x571250>
1405a4898: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a489f: 48 ba a1 bf 76 93 bb 3e 38 2e       	movabs	rdx, 0x2e383ebb9376bfa1
1405a48a9: 49 b8 3c 83 7c c1 e2 47 b8 96       	movabs	r8, -0x6947b81d3e837cc4
1405a48b3: 48 89 f9                    	mov	rcx, rdi
1405a48b6: e8 35 8e b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a48bb: 48 85 c0                    	test	rax, rax
1405a48be: 74 33                       	je	0x1405a48f3 <.text+0x5a38f3>
1405a48c0: 48 39 93 58 06 00 00        	cmp	qword ptr [rbx + 0x658], rdx
1405a48c7: 76 2a                       	jbe	0x1405a48f3 <.text+0x5a38f3>
1405a48c9: 48 8b 83 50 06 00 00        	mov	rax, qword ptr [rbx + 0x650]
1405a48d0: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a48d4: 48 85 c0                    	test	rax, rax
1405a48d7: 74 1a                       	je	0x1405a48f3 <.text+0x5a38f3>
1405a48d9: 48 8b 8b 20 06 00 00        	mov	rcx, qword ptr [rbx + 0x620]
1405a48e0: 48 f7 d0                    	not	rax
1405a48e3: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a48e7: 48 c1 e0 05                 	shl	rax, 0x5
1405a48eb: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a48f1: 75 13                       	jne	0x1405a4906 <.text+0x5a3906>
1405a48f3: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a48fa: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a4901: e8 da 25 fc ff              	call	0x140566ee0 <.text+0x565ee0>
1405a4906: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a490d: 48 ba ea 1a 85 f9 b7 27 ec 09       	movabs	rdx, 0x9ec27b7f9851aea
1405a4917: 49 b8 3e 88 e6 b9 89 f0 73 1e       	movabs	r8, 0x1e73f089b9e6883e
1405a4921: 48 89 f9                    	mov	rcx, rdi
1405a4924: e8 c7 8d b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a4929: 48 85 c0                    	test	rax, rax
1405a492c: 74 33                       	je	0x1405a4961 <.text+0x5a3961>
1405a492e: 48 39 93 58 06 00 00        	cmp	qword ptr [rbx + 0x658], rdx
1405a4935: 76 2a                       	jbe	0x1405a4961 <.text+0x5a3961>
1405a4937: 48 8b 83 50 06 00 00        	mov	rax, qword ptr [rbx + 0x650]
1405a493e: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a4942: 48 85 c0                    	test	rax, rax
1405a4945: 74 1a                       	je	0x1405a4961 <.text+0x5a3961>
1405a4947: 48 8b 8b 20 06 00 00        	mov	rcx, qword ptr [rbx + 0x620]
1405a494e: 48 f7 d0                    	not	rax
1405a4951: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a4955: 48 c1 e0 05                 	shl	rax, 0x5
1405a4959: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a495f: 75 13                       	jne	0x1405a4974 <.text+0x5a3974>
1405a4961: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4968: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a496f: e8 3c 48 fd ff              	call	0x1405791b0 <.text+0x5781b0>
1405a4974: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a497b: 48 ba 12 23 b8 7f fe 3c 53 fc       	movabs	rdx, -0x3acc3018047dcee
1405a4985: 49 b8 77 b2 ed 3d a0 a6 13 2e       	movabs	r8, 0x2e13a6a03dedb277
1405a498f: 48 89 f9                    	mov	rcx, rdi
1405a4992: e8 59 8d b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a4997: 48 85 c0                    	test	rax, rax
1405a499a: 74 33                       	je	0x1405a49cf <.text+0x5a39cf>
1405a499c: 48 39 93 58 06 00 00        	cmp	qword ptr [rbx + 0x658], rdx
1405a49a3: 76 2a                       	jbe	0x1405a49cf <.text+0x5a39cf>
1405a49a5: 48 8b 83 50 06 00 00        	mov	rax, qword ptr [rbx + 0x650]
1405a49ac: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1405a49b0: 48 85 c0                    	test	rax, rax
1405a49b3: 74 1a                       	je	0x1405a49cf <.text+0x5a39cf>
1405a49b5: 48 8b 8b 20 06 00 00        	mov	rcx, qword ptr [rbx + 0x620]
1405a49bc: 48 f7 d0                    	not	rax
1405a49bf: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1405a49c3: 48 c1 e0 05                 	shl	rax, 0x5
1405a49c7: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1405a49cd: 75 13                       	jne	0x1405a49e2 <.text+0x5a39e2>
1405a49cf: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a49d6: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a49dd: e8 7e b9 fc ff              	call	0x140570360 <.text+0x56f360>
1405a49e2: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a49e9: 48 ba 8b 04 7b e6 61 26 c0 a4       	movabs	rdx, -0x5b3fd99e1984fb75
1405a49f3: 49 b8 ec 67 2a aa e6 61 e7 ea       	movabs	r8, -0x15189e1955d59814
1405a49fd: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
1405a4a04: e8 e7 8c b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a4a09: 48 85 c0                    	test	rax, rax
1405a4a0c: 4c 8b 83 c8 00 00 00        	mov	r8, qword ptr [rbx + 0xc8]
1405a4a13: 0f 84 ca 1c 00 00           	je	0x1405a66e3 <.text+0x5a56e3>
1405a4a19: 49 39 90 18 02 00 00        	cmp	qword ptr [r8 + 0x218], rdx
1405a4a20: 0f 86 bd 1c 00 00           	jbe	0x1405a66e3 <.text+0x5a56e3>
1405a4a26: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
1405a4a2d: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1405a4a31: 48 85 c9                    	test	rcx, rcx
1405a4a34: 0f 84 a9 1c 00 00           	je	0x1405a66e3 <.text+0x5a56e3>
1405a4a3a: 49 8b 80 e0 01 00 00        	mov	rax, qword ptr [r8 + 0x1e0]
1405a4a41: 48 f7 d1                    	not	rcx
1405a4a44: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1405a4a48: 48 c1 e1 05                 	shl	rcx, 0x5
1405a4a4c: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1405a4a52: 0f 84 8b 1c 00 00           	je	0x1405a66e3 <.text+0x5a56e3>
1405a4a58: 48 01 c8                    	add	rax, rcx
1405a4a5b: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
1405a4a5f: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
1405a4a63: 0f b6 50 20                 	movzx	edx, byte ptr [rax + 0x20]
1405a4a67: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1405a4a6a: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
1405a4a71: 48 89 8b 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], rcx
1405a4a78: 88 93 20 0b 00 00           	mov	byte ptr [rbx + 0xb20], dl
1405a4a7e: f2 0f 10 40 18              	movsd	xmm0, qword ptr [rax + 0x18]
1405a4a83: f2 0f 11 83 18 0b 00 00     	movsd	qword ptr [rbx + 0xb18], xmm0
1405a4a8b: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4a92: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a4a99: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a4aa0: e8 7b 99 b8 ff              	call	0x14012e420 <.text+0x12d420>
1405a4aa5: 48 89 c6                    	mov	rsi, rax
1405a4aa8: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4aaf: 48 ba 80 ee bc 5f 6a 38 35 33       	movabs	rdx, 0x3335386a5fbcee80
1405a4ab9: 49 b8 60 ec 65 1a f3 59 2a 8f       	movabs	r8, -0x70d5a60ce59a13a0
1405a4ac3: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
1405a4aca: e8 21 8c b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a4acf: 48 85 c0                    	test	rax, rax
1405a4ad2: 4c 8b 83 c8 00 00 00        	mov	r8, qword ptr [rbx + 0xc8]
1405a4ad9: 0f 84 8e 1c 00 00           	je	0x1405a676d <.text+0x5a576d>
1405a4adf: 49 39 90 18 02 00 00        	cmp	qword ptr [r8 + 0x218], rdx
1405a4ae6: 0f 86 81 1c 00 00           	jbe	0x1405a676d <.text+0x5a576d>
1405a4aec: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
1405a4af3: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1405a4af7: 48 85 c9                    	test	rcx, rcx
1405a4afa: 0f 84 6d 1c 00 00           	je	0x1405a676d <.text+0x5a576d>
1405a4b00: 49 8b 80 e0 01 00 00        	mov	rax, qword ptr [r8 + 0x1e0]
1405a4b07: 48 f7 d1                    	not	rcx
1405a4b0a: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1405a4b0e: 48 c1 e1 05                 	shl	rcx, 0x5
1405a4b12: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1405a4b18: 0f 84 4f 1c 00 00           	je	0x1405a676d <.text+0x5a576d>
1405a4b1e: 48 01 c8                    	add	rax, rcx
1405a4b21: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
1405a4b25: 0f b6 48 28                 	movzx	ecx, byte ptr [rax + 0x28]
1405a4b29: 88 8b 28 0b 00 00           	mov	byte ptr [rbx + 0xb28], cl
1405a4b2f: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1405a4b32: 0f 10 48 10                 	movups	xmm1, xmmword ptr [rax + 0x10]
1405a4b36: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
1405a4b3d: 0f 29 8b 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm1
1405a4b44: 48 8b 40 20                 	mov	rax, qword ptr [rax + 0x20]
1405a4b48: 48 89 83 20 0b 00 00        	mov	qword ptr [rbx + 0xb20], rax
1405a4b4f: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4b56: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a4b5d: 48 89 f1                    	mov	rcx, rsi
1405a4b60: e8 8b 8a b9 ff              	call	0x14013d5f0 <.text+0x13c5f0>
1405a4b65: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4b6c: 48 89 c1                    	mov	rcx, rax
1405a4b6f: 44 89 e2                    	mov	edx, r12d
1405a4b72: 45 89 e8                    	mov	r8d, r13d
1405a4b75: e8 e6 b8 b7 ff              	call	0x140120460 <.text+0x11f460>
1405a4b7a: 48 89 c6                    	mov	rsi, rax
1405a4b7d: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4b84: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a4b8b: e8 70 01 5c 00              	call	0x140b64d00 <.text+0xb63d00>
1405a4b90: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4b97: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a4b9e: 48 89 f1                    	mov	rcx, rsi
1405a4ba1: e8 1a 78 b8 ff              	call	0x14012c3c0 <.text+0x12b3c0>
1405a4ba6: 48 89 c6                    	mov	rsi, rax
1405a4ba9: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4bb0: 48 ba 40 7c d4 87 e7 56 7b 3d       	movabs	rdx, 0x3d7b56e787d47c40
1405a4bba: 49 b8 c7 bf 43 8d 0a b1 b0 67       	movabs	r8, 0x67b0b10a8d43bfc7
1405a4bc4: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
1405a4bcb: e8 20 8b b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a4bd0: 48 85 c0                    	test	rax, rax
1405a4bd3: 4c 8b 83 c8 00 00 00        	mov	r8, qword ptr [rbx + 0xc8]
1405a4bda: 0f 84 17 1c 00 00           	je	0x1405a67f7 <.text+0x5a57f7>
1405a4be0: 49 39 90 18 02 00 00        	cmp	qword ptr [r8 + 0x218], rdx
1405a4be7: 0f 86 0a 1c 00 00           	jbe	0x1405a67f7 <.text+0x5a57f7>
1405a4bed: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
1405a4bf4: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1405a4bf8: 48 85 c9                    	test	rcx, rcx
1405a4bfb: 0f 84 f6 1b 00 00           	je	0x1405a67f7 <.text+0x5a57f7>
1405a4c01: 49 8b 80 e0 01 00 00        	mov	rax, qword ptr [r8 + 0x1e0]
1405a4c08: 48 f7 d1                    	not	rcx
1405a4c0b: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1405a4c0f: 48 c1 e1 05                 	shl	rcx, 0x5
1405a4c13: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1405a4c19: 0f 84 d8 1b 00 00           	je	0x1405a67f7 <.text+0x5a57f7>
1405a4c1f: 48 01 c8                    	add	rax, rcx
1405a4c22: 48 8b 50 10                 	mov	rdx, qword ptr [rax + 0x10]
1405a4c26: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4c2d: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a4c34: e8 d7 e8 fe ff              	call	0x140593510 <.text+0x592510>
1405a4c39: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4c40: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a4c47: 48 89 f1                    	mov	rcx, rsi
1405a4c4a: e8 c1 4b b7 ff              	call	0x140119810 <.text+0x118810>
1405a4c4f: 48 89 c6                    	mov	rsi, rax
1405a4c52: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4c59: 48 ba 1c d5 52 f0 a9 85 f9 fd       	movabs	rdx, -0x2067a560fad2ae4
1405a4c63: 49 b8 c6 b8 1d 62 b5 d9 c2 d7       	movabs	r8, -0x283d264a9de2473a
1405a4c6d: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
1405a4c74: e8 77 8a b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a4c79: 48 85 c0                    	test	rax, rax
1405a4c7c: 4c 8b 83 c8 00 00 00        	mov	r8, qword ptr [rbx + 0xc8]
1405a4c83: 0f 84 f8 1b 00 00           	je	0x1405a6881 <.text+0x5a5881>
1405a4c89: 49 39 90 18 02 00 00        	cmp	qword ptr [r8 + 0x218], rdx
1405a4c90: 0f 86 eb 1b 00 00           	jbe	0x1405a6881 <.text+0x5a5881>
1405a4c96: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
1405a4c9d: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1405a4ca1: 48 85 c9                    	test	rcx, rcx
1405a4ca4: 0f 84 d7 1b 00 00           	je	0x1405a6881 <.text+0x5a5881>
1405a4caa: 49 8b 80 e0 01 00 00        	mov	rax, qword ptr [r8 + 0x1e0]
1405a4cb1: 48 f7 d1                    	not	rcx
1405a4cb4: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1405a4cb8: 48 c1 e1 05                 	shl	rcx, 0x5
1405a4cbc: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1405a4cc2: 0f 84 b9 1b 00 00           	je	0x1405a6881 <.text+0x5a5881>
1405a4cc8: 48 01 c8                    	add	rax, rcx
1405a4ccb: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
1405a4ccf: 48 8b 10                    	mov	rdx, qword ptr [rax]
1405a4cd2: 48 85 d2                    	test	rdx, rdx
1405a4cd5: 74 0a                       	je	0x1405a4ce1 <.text+0x5a3ce1>
1405a4cd7: f0                          	lock
1405a4cd8: 48 ff 02                    	inc	qword ptr [rdx]
1405a4cdb: 0f 8e 5f 23 00 00           	jle	0x1405a7040 <.text+0x5a6040>
1405a4ce1: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4ce8: 48 89 f1                    	mov	rcx, rsi
1405a4ceb: e8 20 01 b7 ff              	call	0x140114e10 <.text+0x113e10>
1405a4cf0: 48 89 c6                    	mov	rsi, rax
1405a4cf3: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4cfa: 48 ba 44 ee a4 09 e0 bd b8 08       	movabs	rdx, 0x8b8bde009a4ee44
1405a4d04: 49 b8 dd f9 bd 7b 2b 57 d7 2a       	movabs	r8, 0x2ad7572b7bbdf9dd
1405a4d0e: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
1405a4d15: e8 d6 89 b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a4d1a: 48 85 c0                    	test	rax, rax
1405a4d1d: 4c 8b 83 c8 00 00 00        	mov	r8, qword ptr [rbx + 0xc8]
1405a4d24: 0f 84 e1 1b 00 00           	je	0x1405a690b <.text+0x5a590b>
1405a4d2a: 49 39 90 18 02 00 00        	cmp	qword ptr [r8 + 0x218], rdx
1405a4d31: 0f 86 d4 1b 00 00           	jbe	0x1405a690b <.text+0x5a590b>
1405a4d37: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
1405a4d3e: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1405a4d42: 48 85 c9                    	test	rcx, rcx
1405a4d45: 0f 84 c0 1b 00 00           	je	0x1405a690b <.text+0x5a590b>
1405a4d4b: 49 8b 80 e0 01 00 00        	mov	rax, qword ptr [r8 + 0x1e0]
1405a4d52: 48 f7 d1                    	not	rcx
1405a4d55: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1405a4d59: 48 c1 e1 05                 	shl	rcx, 0x5
1405a4d5d: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1405a4d63: 0f 84 a2 1b 00 00           	je	0x1405a690b <.text+0x5a590b>
1405a4d69: 48 01 c8                    	add	rax, rcx
1405a4d6c: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
1405a4d70: 0f b6 10                    	movzx	edx, byte ptr [rax]
1405a4d73: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4d7a: 48 89 f1                    	mov	rcx, rsi
1405a4d7d: e8 4e 54 b9 ff              	call	0x14013a1d0 <.text+0x1391d0>
1405a4d82: 48 89 c6                    	mov	rsi, rax
1405a4d85: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4d8c: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a4d93: 31 d2                       	xor	edx, edx
1405a4d95: e8 56 53 31 00              	call	0x1408ba0f0 <.text+0x8b90f0>
1405a4d9a: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4da1: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a4da8: 48 89 f1                    	mov	rcx, rsi
1405a4dab: e8 b0 cf b8 ff              	call	0x140131d60 <.text+0x130d60>
1405a4db0: 48 89 c6                    	mov	rsi, rax
1405a4db3: 44 0f 29 83 c0 25 00 00     	movaps	xmmword ptr [rbx + 0x25c0], xmm8
1405a4dbb: 44 0f 29 8b b0 25 00 00     	movaps	xmmword ptr [rbx + 0x25b0], xmm9
1405a4dc3: 44 0f 29 93 a0 25 00 00     	movaps	xmmword ptr [rbx + 0x25a0], xmm10
1405a4dcb: 44 0f 29 9b 90 25 00 00     	movaps	xmmword ptr [rbx + 0x2590], xmm11
1405a4dd3: 44 0f 29 9b d0 25 00 00     	movaps	xmmword ptr [rbx + 0x25d0], xmm11
1405a4ddb: 44 0f 29 93 e0 25 00 00     	movaps	xmmword ptr [rbx + 0x25e0], xmm10
1405a4de3: 44 0f 29 8b f0 25 00 00     	movaps	xmmword ptr [rbx + 0x25f0], xmm9
1405a4deb: 44 0f 29 83 00 26 00 00     	movaps	xmmword ptr [rbx + 0x2600], xmm8
1405a4df3: 44 0f 29 9b 10 26 00 00     	movaps	xmmword ptr [rbx + 0x2610], xmm11
1405a4dfb: 44 0f 29 93 20 26 00 00     	movaps	xmmword ptr [rbx + 0x2620], xmm10
1405a4e03: 44 0f 29 8b 30 26 00 00     	movaps	xmmword ptr [rbx + 0x2630], xmm9
1405a4e0b: 44 0f 29 83 40 26 00 00     	movaps	xmmword ptr [rbx + 0x2640], xmm8
1405a4e13: 44 0f 29 9b 50 26 00 00     	movaps	xmmword ptr [rbx + 0x2650], xmm11
1405a4e1b: 44 0f 29 93 60 26 00 00     	movaps	xmmword ptr [rbx + 0x2660], xmm10
1405a4e23: 44 0f 29 8b 70 26 00 00     	movaps	xmmword ptr [rbx + 0x2670], xmm9
1405a4e2b: 44 0f 29 83 80 26 00 00     	movaps	xmmword ptr [rbx + 0x2680], xmm8
1405a4e33: c7 83 08 0c 00 00 00 00 00 00       	mov	dword ptr [rbx + 0xc08], 0x0
1405a4e3d: 48 8d bb 00 0b 00 00        	lea	rdi, [rbx + 0xb00]
1405a4e44: 48 8d 93 90 25 00 00        	lea	rdx, [rbx + 0x2590]
1405a4e4b: 41 b8 00 01 00 00           	mov	r8d, 0x100
1405a4e51: 48 89 f9                    	mov	rcx, rdi
1405a4e54: e8 a7 ed 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a4e59: 48 c7 83 00 0c 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xc00], 0x0
1405a4e64: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4e6b: 48 89 f1                    	mov	rcx, rsi
1405a4e6e: 48 89 fa                    	mov	rdx, rdi
1405a4e71: e8 4a 0e b9 ff              	call	0x140135cc0 <.text+0x134cc0>
1405a4e76: 48 89 c6                    	mov	rsi, rax
1405a4e79: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4e80: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a4e87: e8 d4 1d 39 00              	call	0x140936c60 <.text+0x935c60>
1405a4e8c: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4e93: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a4e9a: 48 89 f1                    	mov	rcx, rsi
1405a4e9d: e8 3e 58 b8 ff              	call	0x14012a6e0 <.text+0x1296e0>
1405a4ea2: 48 89 c7                    	mov	rdi, rax
1405a4ea5: 49 8d 77 10                 	lea	rsi, [r15 + 0x10]
1405a4ea9: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4eb0: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a4eb7: 48 89 f2                    	mov	rdx, rsi
1405a4eba: e8 51 6c 39 00              	call	0x14093bb10 <.text+0x93ab10>
1405a4ebf: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4ec6: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a4ecd: 48 89 f9                    	mov	rcx, rdi
1405a4ed0: e8 4b 1e b8 ff              	call	0x140126d20 <.text+0x125d20>
1405a4ed5: 48 89 c7                    	mov	rdi, rax
1405a4ed8: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4edf: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a4ee6: e8 35 7a 38 00              	call	0x14092c920 <.text+0x92b920>
1405a4eeb: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4ef2: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a4ef9: 48 89 f9                    	mov	rcx, rdi
1405a4efc: e8 4f 18 b8 ff              	call	0x140126750 <.text+0x125750>
1405a4f01: 48 89 c7                    	mov	rdi, rax
1405a4f04: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4f0b: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a4f12: e8 49 9c 37 00              	call	0x14091eb60 <.text+0x91db60>
1405a4f17: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4f1e: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a4f25: 48 89 f9                    	mov	rcx, rdi
1405a4f28: e8 e3 42 b7 ff              	call	0x140119210 <.text+0x118210>
1405a4f2d: 48 89 c7                    	mov	rdi, rax
1405a4f30: 48 c7 83 a0 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xba0], 0x0
1405a4f3b: c7 83 a8 0b 00 00 00 00 00 00       	mov	dword ptr [rbx + 0xba8], 0x0
1405a4f45: 48 c7 83 b0 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xbb0], 0x0
1405a4f50: c7 83 b8 0b 00 00 00 00 00 00       	mov	dword ptr [rbx + 0xbb8], 0x0
1405a4f5a: 48 c7 83 c0 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xbc0], 0x0
1405a4f65: c7 83 c8 0b 00 00 00 00 00 00       	mov	dword ptr [rbx + 0xbc8], 0x0
1405a4f6f: 0f 57 f6                    	xorps	xmm6, xmm6
1405a4f72: 0f 29 b3 e0 0b 00 00        	movaps	xmmword ptr [rbx + 0xbe0], xmm6
1405a4f79: 48 8d 8b f0 0b 00 00        	lea	rcx, [rbx + 0xbf0]
1405a4f80: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4f87: e8 04 a5 b4 ff              	call	0x1400ef490 <.text+0xee490>
1405a4f8c: c7 83 ac 0b 00 00 00 00 00 00       	mov	dword ptr [rbx + 0xbac], 0x0
1405a4f96: c7 83 bc 0b 00 00 00 00 00 00       	mov	dword ptr [rbx + 0xbbc], 0x0
1405a4fa0: 0f 11 b3 cc 0b 00 00        	movups	xmmword ptr [rbx + 0xbcc], xmm6
1405a4fa7: c7 83 dc 0b 00 00 00 00 00 00       	mov	dword ptr [rbx + 0xbdc], 0x0
1405a4fb1: 0f 29 b3 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm6
1405a4fb8: 0f 29 b3 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm6
1405a4fbf: 0f 29 b3 20 0b 00 00        	movaps	xmmword ptr [rbx + 0xb20], xmm6
1405a4fc6: 0f 29 b3 30 0b 00 00        	movaps	xmmword ptr [rbx + 0xb30], xmm6
1405a4fcd: 0f 29 b3 40 0b 00 00        	movaps	xmmword ptr [rbx + 0xb40], xmm6
1405a4fd4: 0f 29 b3 50 0b 00 00        	movaps	xmmword ptr [rbx + 0xb50], xmm6
1405a4fdb: 0f 29 b3 60 0b 00 00        	movaps	xmmword ptr [rbx + 0xb60], xmm6
1405a4fe2: 0f 29 b3 70 0b 00 00        	movaps	xmmword ptr [rbx + 0xb70], xmm6
1405a4fe9: 0f 29 b3 80 0b 00 00        	movaps	xmmword ptr [rbx + 0xb80], xmm6
1405a4ff0: 0f 29 b3 90 0b 00 00        	movaps	xmmword ptr [rbx + 0xb90], xmm6
1405a4ff7: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a4ffe: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a5005: 48 89 f9                    	mov	rcx, rdi
1405a5008: e8 43 84 b8 ff              	call	0x14012d450 <.text+0x12c450>
1405a500d: 48 89 c7                    	mov	rdi, rax
1405a5010: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a5017: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a501e: 31 d2                       	xor	edx, edx
1405a5020: e8 6b 54 31 00              	call	0x1408ba490 <.text+0x8b9490>
1405a5025: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a502c: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a5033: 48 89 f9                    	mov	rcx, rdi
1405a5036: e8 e5 33 b8 ff              	call	0x140128420 <.text+0x127420>
1405a503b: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a5042: 48 89 c1                    	mov	rcx, rax
1405a5045: e8 56 0f b7 ff              	call	0x140115fa0 <.text+0x114fa0>
1405a504a: 48 89 c7                    	mov	rdi, rax
1405a504d: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a5054: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a505b: 31 d2                       	xor	edx, edx
1405a505d: e8 ce 5a 31 00              	call	0x1408bab30 <.text+0x8b9b30>
1405a5062: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a5069: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a5070: 48 89 f9                    	mov	rcx, rdi
1405a5073: e8 68 1d b9 ff              	call	0x140136de0 <.text+0x135de0>
1405a5078: 48 89 c7                    	mov	rdi, rax
1405a507b: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a5082: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a5089: e8 a2 ef 37 00              	call	0x140924030 <.text+0x923030>
1405a508e: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a5095: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a509c: 48 89 f9                    	mov	rcx, rdi
1405a509f: e8 4c c4 b7 ff              	call	0x1401214f0 <.text+0x1204f0>
1405a50a4: 48 89 c7                    	mov	rdi, rax
1405a50a7: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a50ae: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a50b5: 48 89 f2                    	mov	rdx, rsi
1405a50b8: e8 b3 d4 65 00              	call	0x140c02570 <.text+0xc01570>
1405a50bd: 0f 10 83 40 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb40]
1405a50c4: 0f 11 83 d7 25 00 00        	movups	xmmword ptr [rbx + 0x25d7], xmm0
1405a50cb: 0f 10 83 00 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb00]
1405a50d2: 0f 10 8b 10 0b 00 00        	movups	xmm1, xmmword ptr [rbx + 0xb10]
1405a50d9: 0f 10 93 20 0b 00 00        	movups	xmm2, xmmword ptr [rbx + 0xb20]
1405a50e0: 0f 10 9b 30 0b 00 00        	movups	xmm3, xmmword ptr [rbx + 0xb30]
1405a50e7: 0f 11 9b c7 25 00 00        	movups	xmmword ptr [rbx + 0x25c7], xmm3
1405a50ee: 0f 11 93 b7 25 00 00        	movups	xmmword ptr [rbx + 0x25b7], xmm2
1405a50f5: 0f 11 8b a7 25 00 00        	movups	xmmword ptr [rbx + 0x25a7], xmm1
1405a50fc: 0f 11 83 97 25 00 00        	movups	xmmword ptr [rbx + 0x2597], xmm0
1405a5103: c6 83 00 0b 00 00 00        	mov	byte ptr [rbx + 0xb00], 0x0
1405a510a: 0f 10 83 90 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2590]
1405a5111: 0f 11 83 01 0b 00 00        	movups	xmmword ptr [rbx + 0xb01], xmm0
1405a5118: 0f 10 83 a0 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x25a0]
1405a511f: 0f 11 83 11 0b 00 00        	movups	xmmword ptr [rbx + 0xb11], xmm0
1405a5126: 0f 10 83 b0 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x25b0]
1405a512d: 0f 11 83 21 0b 00 00        	movups	xmmword ptr [rbx + 0xb21], xmm0
1405a5134: 0f 10 83 c0 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x25c0]
1405a513b: 0f 11 83 31 0b 00 00        	movups	xmmword ptr [rbx + 0xb31], xmm0
1405a5142: 0f 10 83 d0 25 00 00        	movups	xmm0, xmmword ptr [rbx + 0x25d0]
1405a5149: 0f 11 83 41 0b 00 00        	movups	xmmword ptr [rbx + 0xb41], xmm0
1405a5150: 48 8b 83 df 25 00 00        	mov	rax, qword ptr [rbx + 0x25df]
1405a5157: 48 89 83 50 0b 00 00        	mov	qword ptr [rbx + 0xb50], rax
1405a515e: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a5165: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a516c: 48 89 f9                    	mov	rcx, rdi
1405a516f: e8 cc a6 b7 ff              	call	0x14011f840 <.text+0x11e840>
1405a5174: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a517b: 48 89 c1                    	mov	rcx, rax
1405a517e: 4c 89 fa                    	mov	rdx, r15
1405a5181: e8 0a 7a b7 ff              	call	0x14011cb90 <.text+0x11bb90>
1405a5186: 48 89 c6                    	mov	rsi, rax
1405a5189: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5190: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a5197: e8 94 ee 37 00              	call	0x140924030 <.text+0x923030>
1405a519c: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a51a3: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a51aa: 48 89 f1                    	mov	rcx, rsi
1405a51ad: e8 3e c3 b7 ff              	call	0x1401214f0 <.text+0x1204f0>
1405a51b2: 48 89 c6                    	mov	rsi, rax
1405a51b5: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a51bc: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a51c3: e8 38 25 2c 00              	call	0x140867700 <.text+0x866700>
1405a51c8: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a51cf: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a51d6: 48 89 f1                    	mov	rcx, rsi
1405a51d9: e8 12 e6 b7 ff              	call	0x1401237f0 <.text+0x1227f0>
1405a51de: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a51e5: 48 89 c1                    	mov	rcx, rax
1405a51e8: e8 b3 0d b7 ff              	call	0x140115fa0 <.text+0x114fa0>
1405a51ed: 48 c7 83 00 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb00], 0x0
1405a51f8: 48 c7 83 08 0b 00 00 04 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x4
1405a5203: 48 c7 83 10 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb10], 0x0
1405a520e: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5215: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a521c: 48 89 c1                    	mov	rcx, rax
1405a521f: e8 fc 2c b9 ff              	call	0x140137f20 <.text+0x136f20>
1405a5224: 48 89 c6                    	mov	rsi, rax
1405a5227: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a522e: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a5235: e8 b6 95 30 00              	call	0x1408ae7f0 <.text+0x8ad7f0>
1405a523a: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5241: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a5248: 48 89 f1                    	mov	rcx, rsi
1405a524b: e8 e0 d9 b7 ff              	call	0x140122c30 <.text+0x121c30>
1405a5250: 48 c7 83 00 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb00], 0x0
1405a525b: 48 c7 83 10 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb10], 0x0
1405a5266: c7 83 28 0b 00 00 00 ca 9a 3b       	mov	dword ptr [rbx + 0xb28], 0x3b9aca00
1405a5270: c6 83 30 0b 00 00 00        	mov	byte ptr [rbx + 0xb30], 0x0
1405a5277: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a527e: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a5285: 48 89 c1                    	mov	rcx, rax
1405a5288: e8 43 8f b7 ff              	call	0x14011e1d0 <.text+0x11d1d0>
1405a528d: 48 89 83 28 01 00 00        	mov	qword ptr [rbx + 0x128], rax
1405a5294: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a529b: e8 c0 a7 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a52a0: 89 83 50 01 00 00           	mov	dword ptr [rbx + 0x150], eax
1405a52a6: f3 0f 11 83 6c 02 00 00     	movss	dword ptr [rbx + 0x26c], xmm0
1405a52ae: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a52b5: e8 a6 a7 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a52ba: 89 83 68 02 00 00           	mov	dword ptr [rbx + 0x268], eax
1405a52c0: f3 0f 11 83 64 02 00 00     	movss	dword ptr [rbx + 0x264], xmm0
1405a52c8: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a52cf: e8 8c a7 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a52d4: 89 83 60 02 00 00           	mov	dword ptr [rbx + 0x260], eax
1405a52da: f3 0f 11 83 5c 02 00 00     	movss	dword ptr [rbx + 0x25c], xmm0
1405a52e2: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a52e9: e8 72 a7 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a52ee: 89 83 58 02 00 00           	mov	dword ptr [rbx + 0x258], eax
1405a52f4: f3 0f 11 83 54 02 00 00     	movss	dword ptr [rbx + 0x254], xmm0
1405a52fc: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5303: e8 58 a7 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5308: 89 83 50 02 00 00           	mov	dword ptr [rbx + 0x250], eax
1405a530e: f3 0f 11 83 4c 02 00 00     	movss	dword ptr [rbx + 0x24c], xmm0
1405a5316: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a531d: e8 3e a7 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5322: 89 83 48 02 00 00           	mov	dword ptr [rbx + 0x248], eax
1405a5328: f3 0f 11 83 44 02 00 00     	movss	dword ptr [rbx + 0x244], xmm0
1405a5330: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5337: e8 24 a7 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a533c: 89 83 40 02 00 00           	mov	dword ptr [rbx + 0x240], eax
1405a5342: f3 0f 11 83 3c 02 00 00     	movss	dword ptr [rbx + 0x23c], xmm0
1405a534a: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5351: e8 0a a7 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5356: 89 83 38 02 00 00           	mov	dword ptr [rbx + 0x238], eax
1405a535c: f3 0f 11 83 34 02 00 00     	movss	dword ptr [rbx + 0x234], xmm0
1405a5364: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a536b: e8 f0 a6 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5370: 89 83 30 02 00 00           	mov	dword ptr [rbx + 0x230], eax
1405a5376: f3 0f 11 83 2c 02 00 00     	movss	dword ptr [rbx + 0x22c], xmm0
1405a537e: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5385: e8 d6 a6 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a538a: 89 83 28 02 00 00           	mov	dword ptr [rbx + 0x228], eax
1405a5390: f3 0f 11 83 24 02 00 00     	movss	dword ptr [rbx + 0x224], xmm0
1405a5398: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a539f: e8 bc a6 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a53a4: 89 83 20 02 00 00           	mov	dword ptr [rbx + 0x220], eax
1405a53aa: f3 0f 11 83 1c 02 00 00     	movss	dword ptr [rbx + 0x21c], xmm0
1405a53b2: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a53b9: e8 a2 a6 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a53be: 89 83 18 02 00 00           	mov	dword ptr [rbx + 0x218], eax
1405a53c4: f3 0f 11 83 14 02 00 00     	movss	dword ptr [rbx + 0x214], xmm0
1405a53cc: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a53d3: e8 88 a6 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a53d8: 89 83 10 02 00 00           	mov	dword ptr [rbx + 0x210], eax
1405a53de: f3 0f 11 83 0c 02 00 00     	movss	dword ptr [rbx + 0x20c], xmm0
1405a53e6: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a53ed: e8 6e a6 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a53f2: 89 83 08 02 00 00           	mov	dword ptr [rbx + 0x208], eax
1405a53f8: f3 0f 11 83 04 02 00 00     	movss	dword ptr [rbx + 0x204], xmm0
1405a5400: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5407: e8 54 a6 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a540c: 89 83 00 02 00 00           	mov	dword ptr [rbx + 0x200], eax
1405a5412: f3 0f 11 83 fc 01 00 00     	movss	dword ptr [rbx + 0x1fc], xmm0
1405a541a: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5421: e8 3a a6 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5426: 89 83 f8 01 00 00           	mov	dword ptr [rbx + 0x1f8], eax
1405a542c: f3 0f 11 83 f4 01 00 00     	movss	dword ptr [rbx + 0x1f4], xmm0
1405a5434: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a543b: e8 20 a6 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5440: 89 83 f0 01 00 00           	mov	dword ptr [rbx + 0x1f0], eax
1405a5446: f3 0f 11 83 ec 01 00 00     	movss	dword ptr [rbx + 0x1ec], xmm0
1405a544e: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5455: e8 06 a6 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a545a: 89 83 e8 01 00 00           	mov	dword ptr [rbx + 0x1e8], eax
1405a5460: f3 0f 11 83 e4 01 00 00     	movss	dword ptr [rbx + 0x1e4], xmm0
1405a5468: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a546f: e8 ec a5 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5474: 89 83 e0 01 00 00           	mov	dword ptr [rbx + 0x1e0], eax
1405a547a: f3 0f 11 83 dc 01 00 00     	movss	dword ptr [rbx + 0x1dc], xmm0
1405a5482: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5489: e8 d2 a5 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a548e: 89 83 d8 01 00 00           	mov	dword ptr [rbx + 0x1d8], eax
1405a5494: f3 0f 11 83 d4 01 00 00     	movss	dword ptr [rbx + 0x1d4], xmm0
1405a549c: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a54a3: e8 b8 a5 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a54a8: 89 83 d0 01 00 00           	mov	dword ptr [rbx + 0x1d0], eax
1405a54ae: f3 0f 11 83 cc 01 00 00     	movss	dword ptr [rbx + 0x1cc], xmm0
1405a54b6: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a54bd: e8 9e a5 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a54c2: 89 83 c8 01 00 00           	mov	dword ptr [rbx + 0x1c8], eax
1405a54c8: f3 0f 11 83 c4 01 00 00     	movss	dword ptr [rbx + 0x1c4], xmm0
1405a54d0: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a54d7: e8 84 a5 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a54dc: 89 83 c0 01 00 00           	mov	dword ptr [rbx + 0x1c0], eax
1405a54e2: f3 0f 11 83 bc 01 00 00     	movss	dword ptr [rbx + 0x1bc], xmm0
1405a54ea: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a54f1: e8 6a a5 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a54f6: 89 83 b8 01 00 00           	mov	dword ptr [rbx + 0x1b8], eax
1405a54fc: f3 0f 11 83 b4 01 00 00     	movss	dword ptr [rbx + 0x1b4], xmm0
1405a5504: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a550b: e8 50 a5 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5510: 89 83 b0 01 00 00           	mov	dword ptr [rbx + 0x1b0], eax
1405a5516: f3 0f 11 83 ac 01 00 00     	movss	dword ptr [rbx + 0x1ac], xmm0
1405a551e: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5525: e8 36 a5 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a552a: 89 83 a8 01 00 00           	mov	dword ptr [rbx + 0x1a8], eax
1405a5530: f3 0f 11 83 a4 01 00 00     	movss	dword ptr [rbx + 0x1a4], xmm0
1405a5538: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a553f: e8 1c a5 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5544: 89 83 a0 01 00 00           	mov	dword ptr [rbx + 0x1a0], eax
1405a554a: f3 0f 11 83 9c 01 00 00     	movss	dword ptr [rbx + 0x19c], xmm0
1405a5552: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5559: e8 02 a5 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a555e: 89 83 98 01 00 00           	mov	dword ptr [rbx + 0x198], eax
1405a5564: f3 0f 11 83 94 01 00 00     	movss	dword ptr [rbx + 0x194], xmm0
1405a556c: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5573: e8 e8 a4 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5578: 89 83 90 01 00 00           	mov	dword ptr [rbx + 0x190], eax
1405a557e: f3 0f 11 83 8c 01 00 00     	movss	dword ptr [rbx + 0x18c], xmm0
1405a5586: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a558d: e8 ce a4 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5592: 89 83 88 01 00 00           	mov	dword ptr [rbx + 0x188], eax
1405a5598: 44 0f 28 f8                 	movaps	xmm15, xmm0
1405a559c: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a55a3: e8 b8 a4 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a55a8: 89 83 84 01 00 00           	mov	dword ptr [rbx + 0x184], eax
1405a55ae: 0f 28 f0                    	movaps	xmm6, xmm0
1405a55b1: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a55b8: e8 a3 a4 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a55bd: 89 83 80 01 00 00           	mov	dword ptr [rbx + 0x180], eax
1405a55c3: 0f 28 f8                    	movaps	xmm7, xmm0
1405a55c6: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a55cd: e8 8e a4 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a55d2: 89 83 7c 01 00 00           	mov	dword ptr [rbx + 0x17c], eax
1405a55d8: 44 0f 28 c0                 	movaps	xmm8, xmm0
1405a55dc: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a55e3: e8 78 a4 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a55e8: 41 89 c7                    	mov	r15d, eax
1405a55eb: 44 0f 28 c8                 	movaps	xmm9, xmm0
1405a55ef: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a55f6: e8 65 a4 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a55fb: 41 89 c4                    	mov	r12d, eax
1405a55fe: 44 0f 28 d0                 	movaps	xmm10, xmm0
1405a5602: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5609: e8 52 a4 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a560e: 89 c6                       	mov	esi, eax
1405a5610: 44 0f 28 d8                 	movaps	xmm11, xmm0
1405a5614: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a561b: e8 40 a4 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5620: 41 89 c5                    	mov	r13d, eax
1405a5623: 44 0f 28 e0                 	movaps	xmm12, xmm0
1405a5627: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a562e: e8 2d a4 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5633: 89 c7                       	mov	edi, eax
1405a5635: 44 0f 28 e8                 	movaps	xmm13, xmm0
1405a5639: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5640: e8 1b a4 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5645: 41 89 c6                    	mov	r14d, eax
1405a5648: 44 0f 28 f0                 	movaps	xmm14, xmm0
1405a564c: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5653: e8 08 a4 39 00              	call	0x14093fa60 <.text+0x93ea60>
1405a5658: 0f 57 c9                    	xorps	xmm1, xmm1
1405a565b: 0f 11 8b 60 0c 00 00        	movups	xmmword ptr [rbx + 0xc60], xmm1
1405a5662: 0f 11 8b 50 0c 00 00        	movups	xmmword ptr [rbx + 0xc50], xmm1
1405a5669: 0f 11 8b 40 0c 00 00        	movups	xmmword ptr [rbx + 0xc40], xmm1
1405a5670: 8b 8b 50 01 00 00           	mov	ecx, dword ptr [rbx + 0x150]
1405a5676: 89 8b 00 0b 00 00           	mov	dword ptr [rbx + 0xb00], ecx
1405a567c: f3 0f 10 8b 6c 02 00 00     	movss	xmm1, dword ptr [rbx + 0x26c]
1405a5684: f3 0f 11 8b 04 0b 00 00     	movss	dword ptr [rbx + 0xb04], xmm1
1405a568c: 8b 8b 68 02 00 00           	mov	ecx, dword ptr [rbx + 0x268]
1405a5692: 89 8b 08 0b 00 00           	mov	dword ptr [rbx + 0xb08], ecx
1405a5698: f3 0f 10 8b 64 02 00 00     	movss	xmm1, dword ptr [rbx + 0x264]
1405a56a0: f3 0f 11 8b 0c 0b 00 00     	movss	dword ptr [rbx + 0xb0c], xmm1
1405a56a8: 8b 8b 60 02 00 00           	mov	ecx, dword ptr [rbx + 0x260]
1405a56ae: 89 8b 10 0b 00 00           	mov	dword ptr [rbx + 0xb10], ecx
1405a56b4: f3 0f 10 8b 5c 02 00 00     	movss	xmm1, dword ptr [rbx + 0x25c]
1405a56bc: f3 0f 11 8b 14 0b 00 00     	movss	dword ptr [rbx + 0xb14], xmm1
1405a56c4: 8b 8b 58 02 00 00           	mov	ecx, dword ptr [rbx + 0x258]
1405a56ca: 89 8b 18 0b 00 00           	mov	dword ptr [rbx + 0xb18], ecx
1405a56d0: f3 0f 10 8b 54 02 00 00     	movss	xmm1, dword ptr [rbx + 0x254]
1405a56d8: f3 0f 11 8b 1c 0b 00 00     	movss	dword ptr [rbx + 0xb1c], xmm1
1405a56e0: 8b 8b 50 02 00 00           	mov	ecx, dword ptr [rbx + 0x250]
1405a56e6: 89 8b 20 0b 00 00           	mov	dword ptr [rbx + 0xb20], ecx
1405a56ec: f3 0f 10 8b 4c 02 00 00     	movss	xmm1, dword ptr [rbx + 0x24c]
1405a56f4: f3 0f 11 8b 24 0b 00 00     	movss	dword ptr [rbx + 0xb24], xmm1
1405a56fc: 8b 8b 48 02 00 00           	mov	ecx, dword ptr [rbx + 0x248]
1405a5702: 89 8b 28 0b 00 00           	mov	dword ptr [rbx + 0xb28], ecx
1405a5708: f3 0f 10 8b 44 02 00 00     	movss	xmm1, dword ptr [rbx + 0x244]
1405a5710: f3 0f 11 8b 2c 0b 00 00     	movss	dword ptr [rbx + 0xb2c], xmm1
1405a5718: 8b 8b 40 02 00 00           	mov	ecx, dword ptr [rbx + 0x240]
1405a571e: 89 8b 30 0b 00 00           	mov	dword ptr [rbx + 0xb30], ecx
1405a5724: f3 0f 10 8b 3c 02 00 00     	movss	xmm1, dword ptr [rbx + 0x23c]
1405a572c: f3 0f 11 8b 34 0b 00 00     	movss	dword ptr [rbx + 0xb34], xmm1
1405a5734: 8b 8b 38 02 00 00           	mov	ecx, dword ptr [rbx + 0x238]
1405a573a: 89 8b 38 0b 00 00           	mov	dword ptr [rbx + 0xb38], ecx
1405a5740: f3 0f 10 8b 34 02 00 00     	movss	xmm1, dword ptr [rbx + 0x234]
1405a5748: f3 0f 11 8b 3c 0b 00 00     	movss	dword ptr [rbx + 0xb3c], xmm1
1405a5750: 8b 8b 30 02 00 00           	mov	ecx, dword ptr [rbx + 0x230]
1405a5756: 89 8b 40 0b 00 00           	mov	dword ptr [rbx + 0xb40], ecx
1405a575c: f3 0f 10 8b 2c 02 00 00     	movss	xmm1, dword ptr [rbx + 0x22c]
1405a5764: f3 0f 11 8b 44 0b 00 00     	movss	dword ptr [rbx + 0xb44], xmm1
1405a576c: 8b 8b 28 02 00 00           	mov	ecx, dword ptr [rbx + 0x228]
1405a5772: 89 8b 48 0b 00 00           	mov	dword ptr [rbx + 0xb48], ecx
1405a5778: f3 0f 10 8b 24 02 00 00     	movss	xmm1, dword ptr [rbx + 0x224]
1405a5780: f3 0f 11 8b 4c 0b 00 00     	movss	dword ptr [rbx + 0xb4c], xmm1
1405a5788: 8b 8b 20 02 00 00           	mov	ecx, dword ptr [rbx + 0x220]
1405a578e: 89 8b 50 0b 00 00           	mov	dword ptr [rbx + 0xb50], ecx
1405a5794: f3 0f 10 8b 1c 02 00 00     	movss	xmm1, dword ptr [rbx + 0x21c]
1405a579c: f3 0f 11 8b 54 0b 00 00     	movss	dword ptr [rbx + 0xb54], xmm1
1405a57a4: 8b 8b 18 02 00 00           	mov	ecx, dword ptr [rbx + 0x218]
1405a57aa: 89 8b 58 0b 00 00           	mov	dword ptr [rbx + 0xb58], ecx
1405a57b0: f3 0f 10 8b 14 02 00 00     	movss	xmm1, dword ptr [rbx + 0x214]
1405a57b8: f3 0f 11 8b 5c 0b 00 00     	movss	dword ptr [rbx + 0xb5c], xmm1
1405a57c0: 8b 8b 10 02 00 00           	mov	ecx, dword ptr [rbx + 0x210]
1405a57c6: 89 8b 60 0b 00 00           	mov	dword ptr [rbx + 0xb60], ecx
1405a57cc: f3 0f 10 8b 0c 02 00 00     	movss	xmm1, dword ptr [rbx + 0x20c]
1405a57d4: f3 0f 11 8b 64 0b 00 00     	movss	dword ptr [rbx + 0xb64], xmm1
1405a57dc: 8b 8b 08 02 00 00           	mov	ecx, dword ptr [rbx + 0x208]
1405a57e2: 89 8b 68 0b 00 00           	mov	dword ptr [rbx + 0xb68], ecx
1405a57e8: f3 0f 10 8b 04 02 00 00     	movss	xmm1, dword ptr [rbx + 0x204]
1405a57f0: f3 0f 11 8b 6c 0b 00 00     	movss	dword ptr [rbx + 0xb6c], xmm1
1405a57f8: 8b 8b 00 02 00 00           	mov	ecx, dword ptr [rbx + 0x200]
1405a57fe: 89 8b 70 0b 00 00           	mov	dword ptr [rbx + 0xb70], ecx
1405a5804: f3 0f 10 8b fc 01 00 00     	movss	xmm1, dword ptr [rbx + 0x1fc]
1405a580c: f3 0f 11 8b 74 0b 00 00     	movss	dword ptr [rbx + 0xb74], xmm1
1405a5814: 8b 8b f8 01 00 00           	mov	ecx, dword ptr [rbx + 0x1f8]
1405a581a: 89 8b 78 0b 00 00           	mov	dword ptr [rbx + 0xb78], ecx
1405a5820: f3 0f 10 8b f4 01 00 00     	movss	xmm1, dword ptr [rbx + 0x1f4]
1405a5828: f3 0f 11 8b 7c 0b 00 00     	movss	dword ptr [rbx + 0xb7c], xmm1
1405a5830: 8b 8b f0 01 00 00           	mov	ecx, dword ptr [rbx + 0x1f0]
1405a5836: 89 8b 80 0b 00 00           	mov	dword ptr [rbx + 0xb80], ecx
1405a583c: f3 0f 10 8b ec 01 00 00     	movss	xmm1, dword ptr [rbx + 0x1ec]
1405a5844: f3 0f 11 8b 84 0b 00 00     	movss	dword ptr [rbx + 0xb84], xmm1
1405a584c: 8b 8b e8 01 00 00           	mov	ecx, dword ptr [rbx + 0x1e8]
1405a5852: 89 8b 88 0b 00 00           	mov	dword ptr [rbx + 0xb88], ecx
1405a5858: f3 0f 10 8b e4 01 00 00     	movss	xmm1, dword ptr [rbx + 0x1e4]
1405a5860: f3 0f 11 8b 8c 0b 00 00     	movss	dword ptr [rbx + 0xb8c], xmm1
1405a5868: 8b 8b e0 01 00 00           	mov	ecx, dword ptr [rbx + 0x1e0]
1405a586e: 89 8b 90 0b 00 00           	mov	dword ptr [rbx + 0xb90], ecx
1405a5874: f3 0f 10 8b dc 01 00 00     	movss	xmm1, dword ptr [rbx + 0x1dc]
1405a587c: f3 0f 11 8b 94 0b 00 00     	movss	dword ptr [rbx + 0xb94], xmm1
1405a5884: 8b 8b d8 01 00 00           	mov	ecx, dword ptr [rbx + 0x1d8]
1405a588a: 89 8b 98 0b 00 00           	mov	dword ptr [rbx + 0xb98], ecx
1405a5890: f3 0f 10 8b d4 01 00 00     	movss	xmm1, dword ptr [rbx + 0x1d4]
1405a5898: f3 0f 11 8b 9c 0b 00 00     	movss	dword ptr [rbx + 0xb9c], xmm1
1405a58a0: 8b 8b d0 01 00 00           	mov	ecx, dword ptr [rbx + 0x1d0]
1405a58a6: 89 8b a0 0b 00 00           	mov	dword ptr [rbx + 0xba0], ecx
1405a58ac: f3 0f 10 8b cc 01 00 00     	movss	xmm1, dword ptr [rbx + 0x1cc]
1405a58b4: f3 0f 11 8b a4 0b 00 00     	movss	dword ptr [rbx + 0xba4], xmm1
1405a58bc: 8b 8b c8 01 00 00           	mov	ecx, dword ptr [rbx + 0x1c8]
1405a58c2: 89 8b a8 0b 00 00           	mov	dword ptr [rbx + 0xba8], ecx
1405a58c8: f3 0f 10 8b c4 01 00 00     	movss	xmm1, dword ptr [rbx + 0x1c4]
1405a58d0: f3 0f 11 8b ac 0b 00 00     	movss	dword ptr [rbx + 0xbac], xmm1
1405a58d8: 8b 8b c0 01 00 00           	mov	ecx, dword ptr [rbx + 0x1c0]
1405a58de: 89 8b b0 0b 00 00           	mov	dword ptr [rbx + 0xbb0], ecx
1405a58e4: f3 0f 10 8b bc 01 00 00     	movss	xmm1, dword ptr [rbx + 0x1bc]
1405a58ec: f3 0f 11 8b b4 0b 00 00     	movss	dword ptr [rbx + 0xbb4], xmm1
1405a58f4: 8b 8b b8 01 00 00           	mov	ecx, dword ptr [rbx + 0x1b8]
1405a58fa: 89 8b b8 0b 00 00           	mov	dword ptr [rbx + 0xbb8], ecx
1405a5900: f3 0f 10 8b b4 01 00 00     	movss	xmm1, dword ptr [rbx + 0x1b4]
1405a5908: f3 0f 11 8b bc 0b 00 00     	movss	dword ptr [rbx + 0xbbc], xmm1
1405a5910: 8b 8b b0 01 00 00           	mov	ecx, dword ptr [rbx + 0x1b0]
1405a5916: 89 8b c0 0b 00 00           	mov	dword ptr [rbx + 0xbc0], ecx
1405a591c: f3 0f 10 8b ac 01 00 00     	movss	xmm1, dword ptr [rbx + 0x1ac]
1405a5924: f3 0f 11 8b c4 0b 00 00     	movss	dword ptr [rbx + 0xbc4], xmm1
1405a592c: 8b 8b a8 01 00 00           	mov	ecx, dword ptr [rbx + 0x1a8]
1405a5932: 89 8b c8 0b 00 00           	mov	dword ptr [rbx + 0xbc8], ecx
1405a5938: f3 0f 10 8b a4 01 00 00     	movss	xmm1, dword ptr [rbx + 0x1a4]
1405a5940: f3 0f 11 8b cc 0b 00 00     	movss	dword ptr [rbx + 0xbcc], xmm1
1405a5948: 8b 8b a0 01 00 00           	mov	ecx, dword ptr [rbx + 0x1a0]
1405a594e: 89 8b d0 0b 00 00           	mov	dword ptr [rbx + 0xbd0], ecx
1405a5954: f3 0f 10 8b 9c 01 00 00     	movss	xmm1, dword ptr [rbx + 0x19c]
1405a595c: f3 0f 11 8b d4 0b 00 00     	movss	dword ptr [rbx + 0xbd4], xmm1
1405a5964: 8b 8b 98 01 00 00           	mov	ecx, dword ptr [rbx + 0x198]
1405a596a: 89 8b d8 0b 00 00           	mov	dword ptr [rbx + 0xbd8], ecx
1405a5970: f3 0f 10 8b 94 01 00 00     	movss	xmm1, dword ptr [rbx + 0x194]
1405a5978: f3 0f 11 8b dc 0b 00 00     	movss	dword ptr [rbx + 0xbdc], xmm1
1405a5980: 8b 8b 90 01 00 00           	mov	ecx, dword ptr [rbx + 0x190]
1405a5986: 89 8b e0 0b 00 00           	mov	dword ptr [rbx + 0xbe0], ecx
1405a598c: f3 0f 10 8b 8c 01 00 00     	movss	xmm1, dword ptr [rbx + 0x18c]
1405a5994: f3 0f 11 8b e4 0b 00 00     	movss	dword ptr [rbx + 0xbe4], xmm1
1405a599c: 8b 8b 88 01 00 00           	mov	ecx, dword ptr [rbx + 0x188]
1405a59a2: 89 8b e8 0b 00 00           	mov	dword ptr [rbx + 0xbe8], ecx
1405a59a8: f3 44 0f 11 bb ec 0b 00 00  	movss	dword ptr [rbx + 0xbec], xmm15
1405a59b1: 8b 8b 84 01 00 00           	mov	ecx, dword ptr [rbx + 0x184]
1405a59b7: 89 8b f0 0b 00 00           	mov	dword ptr [rbx + 0xbf0], ecx
1405a59bd: f3 0f 11 b3 f4 0b 00 00     	movss	dword ptr [rbx + 0xbf4], xmm6
1405a59c5: 8b 8b 80 01 00 00           	mov	ecx, dword ptr [rbx + 0x180]
1405a59cb: 89 8b f8 0b 00 00           	mov	dword ptr [rbx + 0xbf8], ecx
1405a59d1: f3 0f 11 bb fc 0b 00 00     	movss	dword ptr [rbx + 0xbfc], xmm7
1405a59d9: 8b 8b 7c 01 00 00           	mov	ecx, dword ptr [rbx + 0x17c]
1405a59df: 89 8b 00 0c 00 00           	mov	dword ptr [rbx + 0xc00], ecx
1405a59e5: f3 44 0f 11 83 04 0c 00 00  	movss	dword ptr [rbx + 0xc04], xmm8
1405a59ee: 44 89 bb 08 0c 00 00        	mov	dword ptr [rbx + 0xc08], r15d
1405a59f5: f3 44 0f 11 8b 0c 0c 00 00  	movss	dword ptr [rbx + 0xc0c], xmm9
1405a59fe: 44 89 a3 10 0c 00 00        	mov	dword ptr [rbx + 0xc10], r12d
1405a5a05: f3 44 0f 11 93 14 0c 00 00  	movss	dword ptr [rbx + 0xc14], xmm10
1405a5a0e: 89 b3 18 0c 00 00           	mov	dword ptr [rbx + 0xc18], esi
1405a5a14: f3 44 0f 11 9b 1c 0c 00 00  	movss	dword ptr [rbx + 0xc1c], xmm11
1405a5a1d: 44 89 ab 20 0c 00 00        	mov	dword ptr [rbx + 0xc20], r13d
1405a5a24: f3 44 0f 11 a3 24 0c 00 00  	movss	dword ptr [rbx + 0xc24], xmm12
1405a5a2d: 89 bb 28 0c 00 00           	mov	dword ptr [rbx + 0xc28], edi
1405a5a33: f3 44 0f 11 ab 2c 0c 00 00  	movss	dword ptr [rbx + 0xc2c], xmm13
1405a5a3c: 44 89 b3 30 0c 00 00        	mov	dword ptr [rbx + 0xc30], r14d
1405a5a43: f3 44 0f 11 b3 34 0c 00 00  	movss	dword ptr [rbx + 0xc34], xmm14
1405a5a4c: 89 83 38 0c 00 00           	mov	dword ptr [rbx + 0xc38], eax
1405a5a52: f3 0f 11 83 3c 0c 00 00     	movss	dword ptr [rbx + 0xc3c], xmm0
1405a5a5a: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5a61: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a5a68: 48 8b 8b 28 01 00 00        	mov	rcx, qword ptr [rbx + 0x128]
1405a5a6f: e8 7c 1e b8 ff              	call	0x1401278f0 <.text+0x1268f0>
1405a5a74: 48 89 c6                    	mov	rsi, rax
1405a5a77: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5a7e: 48 ba 8a bf e6 7d 0e 98 39 2b       	movabs	rdx, 0x2b39980e7de6bf8a
1405a5a88: 49 b8 c5 56 0d d2 bf aa d9 52       	movabs	r8, 0x52d9aabfd20d56c5
1405a5a92: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
1405a5a99: e8 52 7c b3 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1405a5a9e: 48 85 c0                    	test	rax, rax
1405a5aa1: 4c 8b 83 c8 00 00 00        	mov	r8, qword ptr [rbx + 0xc8]
1405a5aa8: 0f 84 e7 0e 00 00           	je	0x1405a6995 <.text+0x5a5995>
1405a5aae: 49 39 90 18 02 00 00        	cmp	qword ptr [r8 + 0x218], rdx
1405a5ab5: 0f 86 da 0e 00 00           	jbe	0x1405a6995 <.text+0x5a5995>
1405a5abb: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
1405a5ac2: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1405a5ac6: 48 85 c9                    	test	rcx, rcx
1405a5ac9: 0f 84 c6 0e 00 00           	je	0x1405a6995 <.text+0x5a5995>
1405a5acf: 49 8b 80 e0 01 00 00        	mov	rax, qword ptr [r8 + 0x1e0]
1405a5ad6: 48 f7 d1                    	not	rcx
1405a5ad9: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1405a5add: 48 c1 e1 05                 	shl	rcx, 0x5
1405a5ae1: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1405a5ae7: 0f 84 a8 0e 00 00           	je	0x1405a6995 <.text+0x5a5995>
1405a5aed: 48 01 c8                    	add	rax, rcx
1405a5af0: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
1405a5af4: 48 8b 08                    	mov	rcx, qword ptr [rax]
1405a5af7: 8b 50 08                    	mov	edx, dword ptr [rax + 0x8]
1405a5afa: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
1405a5afe: 8b 40 18                    	mov	eax, dword ptr [rax + 0x18]
1405a5b01: 48 89 8b 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rcx
1405a5b08: 89 93 08 0b 00 00           	mov	dword ptr [rbx + 0xb08], edx
1405a5b0e: 4c 89 83 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], r8
1405a5b15: 89 83 18 0b 00 00           	mov	dword ptr [rbx + 0xb18], eax
1405a5b1b: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5b22: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a5b29: 48 89 f1                    	mov	rcx, rsi
1405a5b2c: e8 4f 5d b8 ff              	call	0x14012b880 <.text+0x12a880>
1405a5b31: 48 89 c6                    	mov	rsi, rax
1405a5b34: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5b3b: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a5b42: e8 49 17 01 00              	call	0x1405b7290 <.text+0x5b6290>
1405a5b47: 0f 10 83 60 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb60]
1405a5b4e: 0f 29 83 f0 25 00 00        	movaps	xmmword ptr [rbx + 0x25f0], xmm0
1405a5b55: 0f 10 83 50 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb50]
1405a5b5c: 0f 29 83 e0 25 00 00        	movaps	xmmword ptr [rbx + 0x25e0], xmm0
1405a5b63: 0f 10 83 40 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb40]
1405a5b6a: 0f 29 83 d0 25 00 00        	movaps	xmmword ptr [rbx + 0x25d0], xmm0
1405a5b71: 0f 10 83 00 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb00]
1405a5b78: 0f 10 8b 10 0b 00 00        	movups	xmm1, xmmword ptr [rbx + 0xb10]
1405a5b7f: 0f 10 93 20 0b 00 00        	movups	xmm2, xmmword ptr [rbx + 0xb20]
1405a5b86: 0f 10 9b 30 0b 00 00        	movups	xmm3, xmmword ptr [rbx + 0xb30]
1405a5b8d: 0f 29 9b c0 25 00 00        	movaps	xmmword ptr [rbx + 0x25c0], xmm3
1405a5b94: 0f 29 93 b0 25 00 00        	movaps	xmmword ptr [rbx + 0x25b0], xmm2
1405a5b9b: 0f 29 8b a0 25 00 00        	movaps	xmmword ptr [rbx + 0x25a0], xmm1
1405a5ba2: 0f 29 83 90 25 00 00        	movaps	xmmword ptr [rbx + 0x2590], xmm0
1405a5ba9: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a5bb0: 48 8d 93 90 25 00 00        	lea	rdx, [rbx + 0x2590]
1405a5bb7: 48 89 f1                    	mov	rcx, rsi
1405a5bba: e8 d1 0c d8 ff              	call	0x140326890 <.text+0x325890>
1405a5bbf: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405a5bc6: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a5bcd: 41 b8 58 06 00 00           	mov	r8d, 0x658
1405a5bd3: e8 28 e0 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a5bd8: c6 83 12 01 00 00 01        	mov	byte ptr [rbx + 0x112], 0x1
1405a5bdf: c6 83 13 01 00 00 01        	mov	byte ptr [rbx + 0x113], 0x1
1405a5be6: c6 83 14 01 00 00 01        	mov	byte ptr [rbx + 0x114], 0x1
1405a5bed: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a5bf4: e8 d7 03 70 00              	call	0x140ca5fd0 <.text+0xca4fd0>
1405a5bf9: 48 8d 8b f0 2b 00 00        	lea	rcx, [rbx + 0x2bf0]
1405a5c00: 41 b8 28 06 00 00           	mov	r8d, 0x628
1405a5c06: 48 8b b3 c8 00 00 00        	mov	rsi, qword ptr [rbx + 0xc8]
1405a5c0d: 48 89 f2                    	mov	rdx, rsi
1405a5c10: e8 eb df 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a5c15: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a5c1c: 41 b8 28 06 00 00           	mov	r8d, 0x628
1405a5c22: 48 89 f1                    	mov	rcx, rsi
1405a5c25: e8 d6 df 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a5c2a: 48 8b 83 38 01 00 00        	mov	rax, qword ptr [rbx + 0x138]
1405a5c31: f0                          	lock
1405a5c32: 48 ff 00                    	inc	qword ptr [rax]
1405a5c35: 48 b9 00 00 00 00 00 00 00 80       	movabs	rcx, -0x8000000000000000
1405a5c3f: 0f 8e fb 13 00 00           	jle	0x1405a7040 <.text+0x5a6040>
1405a5c45: 48 89 83 f0 1f 00 00        	mov	qword ptr [rbx + 0x1ff0], rax
1405a5c4c: 48 89 8b 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], rcx
1405a5c53: 48 c7 83 00 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb00], 0x0
1405a5c5e: c6 83 28 0b 00 00 00        	mov	byte ptr [rbx + 0xb28], 0x0
1405a5c65: 0f b6 05 9d ef 08 03        	movzx	eax, byte ptr [rip + 0x308ef9d] # 0x143634c09
1405a5c6c: b9 0a 00 00 00              	mov	ecx, 0xa
1405a5c71: ba 01 00 00 00              	mov	edx, 0x1
1405a5c76: e8 95 df 06 00              	call	0x140613c10 <.text+0x612c10>
1405a5c7b: 48 85 c0                    	test	rax, rax
1405a5c7e: 48 89 83 e8 00 00 00        	mov	qword ptr [rbx + 0xe8], rax
1405a5c85: 0f 84 5a 13 00 00           	je	0x1405a6fe5 <.text+0x5a5fe5>
1405a5c8b: 48 b9 61 73 79 6e 63 20 6c 6f       	movabs	rcx, 0x6f6c20636e797361
1405a5c95: 48 89 08                    	mov	qword ptr [rax], rcx
1405a5c98: 66 c7 40 08 61 64           	mov	word ptr [rax + 0x8], 0x6461
1405a5c9e: 48 c7 83 30 22 00 00 0a 00 00 00    	mov	qword ptr [rbx + 0x2230], 0xa
1405a5ca9: 48 89 83 38 22 00 00        	mov	qword ptr [rbx + 0x2238], rax
1405a5cb0: 48 c7 83 40 22 00 00 0a 00 00 00    	mov	qword ptr [rbx + 0x2240], 0xa
1405a5cbb: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a5cc2: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a5cc9: 4c 8d 83 30 22 00 00        	lea	r8, [rbx + 0x2230]
1405a5cd0: e8 1b fc 7a 00              	call	0x140d558f0 <.text+0xd548f0>
1405a5cd5: 44 8a ab 07 01 00 00        	mov	r13b, byte ptr [rbx + 0x107]
1405a5cdc: 48 8b 83 f0 1f 00 00        	mov	rax, qword ptr [rbx + 0x1ff0]
1405a5ce3: 48 89 83 38 11 00 00        	mov	qword ptr [rbx + 0x1138], rax
1405a5cea: 48 8d 8b 10 0b 00 00        	lea	rcx, [rbx + 0xb10]
1405a5cf1: 48 8d 93 f0 2b 00 00        	lea	rdx, [rbx + 0x2bf0]
1405a5cf8: 41 b8 28 06 00 00           	mov	r8d, 0x628
1405a5cfe: e8 fd de 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a5d03: 0f 10 83 e0 02 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2e0]
1405a5d0a: 0f 11 83 00 0b 00 00        	movups	xmmword ptr [rbx + 0xb00], xmm0
1405a5d11: c6 83 db 00 00 00 00        	mov	byte ptr [rbx + 0xdb], 0x0
1405a5d18: 48 8d b3 30 22 00 00        	lea	rsi, [rbx + 0x2230]
1405a5d1f: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a5d26: 4c 8d 83 00 0b 00 00        	lea	r8, [rbx + 0xb00]
1405a5d2d: 48 89 f1                    	mov	rcx, rsi
1405a5d30: e8 4b 49 ce ff              	call	0x14028a680 <.text+0x289680>
1405a5d35: 48 8b bb 30 22 00 00        	mov	rdi, qword ptr [rbx + 0x2230]
1405a5d3c: 48 85 ff                    	test	rdi, rdi
1405a5d3f: 74 45                       	je	0x1405a5d86 <.text+0x5a4d86>
1405a5d41: 48 8b 8b 40 22 00 00        	mov	rcx, qword ptr [rbx + 0x2240]
1405a5d48: ff 15 a2 c6 be 02           	call	qword ptr [rip + 0x2bec6a2] # 0x1431923f0
1405a5d4e: f0                          	lock
1405a5d4f: 48 ff 0f                    	dec	qword ptr [rdi]
1405a5d52: 75 0c                       	jne	0x1405a5d60 <.text+0x5a4d60>
1405a5d54: 48 8d 8b 30 22 00 00        	lea	rcx, [rbx + 0x2230]
1405a5d5b: e8 00 de 7a 00              	call	0x140d53b60 <.text+0xd52b60>
1405a5d60: 48 8b 83 38 22 00 00        	mov	rax, qword ptr [rbx + 0x2238]
1405a5d67: f0                          	lock
1405a5d68: 48 ff 08                    	dec	qword ptr [rax]
1405a5d6b: 0f 85 8f 00 00 00           	jne	0x1405a5e00 <.text+0x5a4e00>
1405a5d71: 48 8d 8b 38 22 00 00        	lea	rcx, [rbx + 0x2238]
1405a5d78: c6 83 db 00 00 00 00        	mov	byte ptr [rbx + 0xdb], 0x0
1405a5d7f: e8 2c e0 7a 00              	call	0x140d53db0 <.text+0xd52db0>
1405a5d84: eb 7a                       	jmp	0x1405a5e00 <.text+0x5a4e00>
1405a5d86: 48 8b 83 38 22 00 00        	mov	rax, qword ptr [rbx + 0x2238]
1405a5d8d: 89 c1                       	mov	ecx, eax
1405a5d8f: 83 e1 03                    	and	ecx, 0x3
1405a5d92: 83 f9 01                    	cmp	ecx, 0x1
1405a5d95: 75 69                       	jne	0x1405a5e00 <.text+0x5a4e00>
1405a5d97: 48 8d 48 ff                 	lea	rcx, [rax - 0x1]
1405a5d9b: 48 89 8b 50 01 00 00        	mov	qword ptr [rbx + 0x150], rcx
1405a5da2: 48 8b 48 ff                 	mov	rcx, qword ptr [rax - 0x1]
1405a5da6: 48 89 8b e8 00 00 00        	mov	qword ptr [rbx + 0xe8], rcx
1405a5dad: 48 8b 40 07                 	mov	rax, qword ptr [rax + 0x7]
1405a5db1: 48 89 83 28 01 00 00        	mov	qword ptr [rbx + 0x128], rax
1405a5db8: 48 8b 00                    	mov	rax, qword ptr [rax]
1405a5dbb: 48 85 c0                    	test	rax, rax
1405a5dbe: 74 09                       	je	0x1405a5dc9 <.text+0x5a4dc9>
1405a5dc0: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
1405a5dc7: ff d0                       	call	rax
1405a5dc9: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
1405a5dd0: 48 8b 83 28 01 00 00        	mov	rax, qword ptr [rbx + 0x128]
1405a5dd7: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
1405a5ddb: 48 85 d2                    	test	rdx, rdx
1405a5dde: 74 09                       	je	0x1405a5de9 <.text+0x5a4de9>
1405a5de0: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
1405a5de4: e8 37 de 06 00              	call	0x140613c20 <.text+0x612c20>
1405a5de9: ba 18 00 00 00              	mov	edx, 0x18
1405a5dee: 41 b8 08 00 00 00           	mov	r8d, 0x8
1405a5df4: 48 8b 8b 50 01 00 00        	mov	rcx, qword ptr [rbx + 0x150]
1405a5dfb: e8 20 de 06 00              	call	0x140613c20 <.text+0x612c20>
1405a5e00: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a5e07: 48 8d 93 90 25 00 00        	lea	rdx, [rbx + 0x2590]
1405a5e0e: 41 b8 58 06 00 00           	mov	r8d, 0x658
1405a5e14: e8 e7 dd 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a5e19: 0f 10 83 f0 02 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2f0]
1405a5e20: 0f 11 83 40 03 00 00        	movups	xmmword ptr [rbx + 0x340], xmm0
1405a5e27: 0f 10 83 28 03 00 00        	movups	xmm0, xmmword ptr [rbx + 0x328]
1405a5e2e: 0f 11 83 70 02 00 00        	movups	xmmword ptr [rbx + 0x270], xmm0
1405a5e35: 0f 10 83 60 01 00 00        	movups	xmm0, xmmword ptr [rbx + 0x160]
1405a5e3c: 0f 11 83 80 02 00 00        	movups	xmmword ptr [rbx + 0x280], xmm0
1405a5e43: 48 8b 83 40 01 00 00        	mov	rax, qword ptr [rbx + 0x140]
1405a5e4a: 48 89 83 70 01 00 00        	mov	qword ptr [rbx + 0x170], rax
1405a5e51: 48 8d bb e8 24 00 00        	lea	rdi, [rbx + 0x24e8]
1405a5e58: 48 8d 93 88 21 00 00        	lea	rdx, [rbx + 0x2188]
1405a5e5f: 41 b8 a8 00 00 00           	mov	r8d, 0xa8
1405a5e65: 48 89 f9                    	mov	rcx, rdi
1405a5e68: e8 93 dd 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a5e6d: 48 8b 93 38 01 00 00        	mov	rdx, qword ptr [rbx + 0x138]
1405a5e74: 48 83 c2 10                 	add	rdx, 0x10
1405a5e78: c6 83 05 01 00 00 01        	mov	byte ptr [rbx + 0x105], 0x1
1405a5e7f: 48 8d 8b 70 03 00 00        	lea	rcx, [rbx + 0x370]
1405a5e86: e8 45 44 64 00              	call	0x140bea2d0 <.text+0xbe92d0>
1405a5e8b: 48 8d 8b f0 1f 00 00        	lea	rcx, [rbx + 0x1ff0]
1405a5e92: e8 39 f9 63 00              	call	0x140be57d0 <.text+0xbe47d0>
1405a5e97: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a5e9e: e8 2d f9 63 00              	call	0x140be57d0 <.text+0xbe47d0>
1405a5ea3: 4c 8d b3 70 03 00 00        	lea	r14, [rbx + 0x370]
1405a5eaa: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
1405a5eb0: 48 89 f1                    	mov	rcx, rsi
1405a5eb3: 4c 89 f2                    	mov	rdx, r14
1405a5eb6: e8 45 dd 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a5ebb: 48 8d 8b 00 23 00 00        	lea	rcx, [rbx + 0x2300]
1405a5ec2: 4c 8d bb f0 1f 00 00        	lea	r15, [rbx + 0x1ff0]
1405a5ec9: 41 b8 f0 00 00 00           	mov	r8d, 0xf0
1405a5ecf: 4c 89 fa                    	mov	rdx, r15
1405a5ed2: e8 29 dd 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a5ed7: 48 8d 8b f0 23 00 00        	lea	rcx, [rbx + 0x23f0]
1405a5ede: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a5ee5: 41 b8 f0 00 00 00           	mov	r8d, 0xf0
1405a5eeb: e8 10 dd 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a5ef0: 0f 28 83 50 03 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x350]
1405a5ef7: 0f 28 8b 60 03 00 00        	movaps	xmm1, xmmword ptr [rbx + 0x360]
1405a5efe: 0f 29 8b 80 03 00 00        	movaps	xmmword ptr [rbx + 0x380], xmm1
1405a5f05: 0f 29 83 70 03 00 00        	movaps	xmmword ptr [rbx + 0x370], xmm0
1405a5f0c: 4c 8b a5 a8 37 00 00        	mov	r12, qword ptr [rbp + 0x37a8]
1405a5f13: 48 8d 93 e0 20 00 00        	lea	rdx, [rbx + 0x20e0]
1405a5f1a: 41 b8 a8 00 00 00           	mov	r8d, 0xa8
1405a5f20: 4c 89 f9                    	mov	rcx, r15
1405a5f23: e8 d8 dc 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a5f28: 4c 8b 83 40 03 00 00        	mov	r8, qword ptr [rbx + 0x340]
1405a5f2f: 4c 8b 8b 48 03 00 00        	mov	r9, qword ptr [rbx + 0x348]
1405a5f36: 0f 10 83 70 02 00 00        	movups	xmm0, xmmword ptr [rbx + 0x270]
1405a5f3d: 0f 10 8b 80 02 00 00        	movups	xmm1, xmmword ptr [rbx + 0x280]
1405a5f44: 48 8b 83 70 01 00 00        	mov	rax, qword ptr [rbx + 0x170]
1405a5f4b: c6 83 05 01 00 00 00        	mov	byte ptr [rbx + 0x105], 0x0
1405a5f52: 4c 89 7c 24 68              	mov	qword ptr [rsp + 0x68], r15
1405a5f57: 4c 89 64 24 60              	mov	qword ptr [rsp + 0x60], r12
1405a5f5c: 4c 89 74 24 58              	mov	qword ptr [rsp + 0x58], r14
1405a5f61: 48 89 74 24 50              	mov	qword ptr [rsp + 0x50], rsi
1405a5f66: 48 89 7c 24 48              	mov	qword ptr [rsp + 0x48], rdi
1405a5f6b: 48 89 44 24 40              	mov	qword ptr [rsp + 0x40], rax
1405a5f70: 0f 11 4c 24 30              	movups	xmmword ptr [rsp + 0x30], xmm1
1405a5f75: 0f 11 44 24 20              	movups	xmmword ptr [rsp + 0x20], xmm0
1405a5f7a: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a5f81: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a5f88: e8 73 21 f3 ff              	call	0x1404d8100 <.text+0x4d7100>
1405a5f8d: c6 83 f5 00 00 00 00        	mov	byte ptr [rbx + 0xf5], 0x0
1405a5f94: c6 83 f6 00 00 00 00        	mov	byte ptr [rbx + 0xf6], 0x0
1405a5f9b: c6 83 f7 00 00 00 00        	mov	byte ptr [rbx + 0xf7], 0x0
1405a5fa2: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a5fa9: e8 a2 22 f3 ff              	call	0x1404d8250 <.text+0x4d7250>
1405a5fae: b0 01                       	mov	al, 0x1
1405a5fb0: 89 83 28 01 00 00           	mov	dword ptr [rbx + 0x128], eax
1405a5fb6: c7 83 e8 00 00 00 00 00 00 00       	mov	dword ptr [rbx + 0xe8], 0x0
1405a5fc0: e9 f7 01 00 00              	jmp	0x1405a61bc <.text+0x5a51bc>
1405a5fc5: 48 8b 8b 68 01 00 00        	mov	rcx, qword ptr [rbx + 0x168]
1405a5fcc: f0                          	lock
1405a5fcd: 48 ff 41 70                 	inc	qword ptr [rcx + 0x70]
1405a5fd1: 7f 1e                       	jg	0x1405a5ff1 <.text+0x5a4ff1>
1405a5fd3: b9 07 00 00 00              	mov	ecx, 0x7
1405a5fd8: cd 29                       	int	0x29
1405a5fda: 0f 0b                       	ud2
1405a5fdc: 48 8b 8b 68 01 00 00        	mov	rcx, qword ptr [rbx + 0x168]
1405a5fe3: f0                          	lock
1405a5fe4: 48 ff 81 00 02 00 00        	inc	qword ptr [rcx + 0x200]
1405a5feb: 0f 8e 51 10 00 00           	jle	0x1405a7042 <.text+0x5a6042>
1405a5ff1: 48 89 83 70 03 00 00        	mov	qword ptr [rbx + 0x370], rax
1405a5ff8: 48 89 8b 78 03 00 00        	mov	qword ptr [rbx + 0x378], rcx
1405a5fff: 48 8b 83 40 01 00 00        	mov	rax, qword ptr [rbx + 0x140]
1405a6006: f0                          	lock
1405a6007: 48 ff 00                    	inc	qword ptr [rax]
1405a600a: 0f 8e 30 10 00 00           	jle	0x1405a7040 <.text+0x5a6040>
1405a6010: 48 89 83 70 02 00 00        	mov	qword ptr [rbx + 0x270], rax
1405a6017: 48 8d b3 f0 1f 00 00        	lea	rsi, [rbx + 0x1ff0]
1405a601e: 48 8d 93 88 21 00 00        	lea	rdx, [rbx + 0x2188]
1405a6025: 41 b8 a8 00 00 00           	mov	r8d, 0xa8
1405a602b: 48 89 f1                    	mov	rcx, rsi
1405a602e: e8 cd db 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a6033: 48 8b 93 38 01 00 00        	mov	rdx, qword ptr [rbx + 0x138]
1405a603a: 48 83 c2 10                 	add	rdx, 0x10
1405a603e: c6 83 06 01 00 00 01        	mov	byte ptr [rbx + 0x106], 0x1
1405a6045: 48 8d 8b 30 22 00 00        	lea	rcx, [rbx + 0x2230]
1405a604c: e8 7f 42 64 00              	call	0x140bea2d0 <.text+0xbe92d0>
1405a6051: 48 8d 8b f0 2b 00 00        	lea	rcx, [rbx + 0x2bf0]
1405a6058: e8 73 f7 63 00              	call	0x140be57d0 <.text+0xbe47d0>
1405a605d: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6064: e8 67 f7 63 00              	call	0x140be57d0 <.text+0xbe47d0>
1405a6069: 48 8d bb 30 22 00 00        	lea	rdi, [rbx + 0x2230]
1405a6070: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
1405a6076: 4c 89 f1                    	mov	rcx, r14
1405a6079: 48 89 fa                    	mov	rdx, rdi
1405a607c: e8 7f db 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a6081: 48 8d 8b 60 26 00 00        	lea	rcx, [rbx + 0x2660]
1405a6088: 4c 8d bb f0 2b 00 00        	lea	r15, [rbx + 0x2bf0]
1405a608f: 41 b8 f0 00 00 00           	mov	r8d, 0xf0
1405a6095: 4c 89 fa                    	mov	rdx, r15
1405a6098: e8 63 db 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a609d: 48 8d 8b 50 27 00 00        	lea	rcx, [rbx + 0x2750]
1405a60a4: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a60ab: 41 b8 f0 00 00 00           	mov	r8d, 0xf0
1405a60b1: e8 4a db 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a60b6: 0f 28 83 50 03 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x350]
1405a60bd: 0f 28 8b 60 03 00 00        	movaps	xmm1, xmmword ptr [rbx + 0x360]
1405a60c4: 0f 29 8b 40 22 00 00        	movaps	xmmword ptr [rbx + 0x2240], xmm1
1405a60cb: 0f 29 83 30 22 00 00        	movaps	xmmword ptr [rbx + 0x2230], xmm0
1405a60d2: 4c 8b a5 a8 37 00 00        	mov	r12, qword ptr [rbp + 0x37a8]
1405a60d9: 48 8d 93 e0 20 00 00        	lea	rdx, [rbx + 0x20e0]
1405a60e0: 41 b8 a8 00 00 00           	mov	r8d, 0xa8
1405a60e6: 4c 89 f9                    	mov	rcx, r15
1405a60e9: e8 12 db 37 02              	call	0x142923c00 <.text+0x2922c00>
1405a60ee: 4c 8b 83 80 02 00 00        	mov	r8, qword ptr [rbx + 0x280]
1405a60f5: 4c 8b 8b 88 02 00 00        	mov	r9, qword ptr [rbx + 0x288]
1405a60fc: 0f 10 83 e8 24 00 00        	movups	xmm0, xmmword ptr [rbx + 0x24e8]
1405a6103: 0f 10 8b 70 03 00 00        	movups	xmm1, xmmword ptr [rbx + 0x370]
1405a610a: 48 8b 83 70 02 00 00        	mov	rax, qword ptr [rbx + 0x270]
1405a6111: c6 83 06 01 00 00 00        	mov	byte ptr [rbx + 0x106], 0x0
1405a6118: 4c 89 7c 24 68              	mov	qword ptr [rsp + 0x68], r15
1405a611d: 4c 89 64 24 60              	mov	qword ptr [rsp + 0x60], r12
1405a6122: 48 89 7c 24 58              	mov	qword ptr [rsp + 0x58], rdi
1405a6127: 4c 89 74 24 50              	mov	qword ptr [rsp + 0x50], r14
1405a612c: 48 89 74 24 48              	mov	qword ptr [rsp + 0x48], rsi
1405a6131: 48 89 44 24 40              	mov	qword ptr [rsp + 0x40], rax
1405a6136: 0f 11 4c 24 30              	movups	xmmword ptr [rsp + 0x30], xmm1
1405a613b: 0f 11 44 24 20              	movups	xmmword ptr [rsp + 0x20], xmm0
1405a6140: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6147: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a614e: e8 ad 1f f3 ff              	call	0x1404d8100 <.text+0x4d7100>
1405a6153: c6 83 c2 00 00 00 00        	mov	byte ptr [rbx + 0xc2], 0x0
1405a615a: c6 83 c3 00 00 00 00        	mov	byte ptr [rbx + 0xc3], 0x0
1405a6161: c6 83 c4 00 00 00 01        	mov	byte ptr [rbx + 0xc4], 0x1
1405a6168: c6 83 c5 00 00 00 01        	mov	byte ptr [rbx + 0xc5], 0x1
1405a616f: c6 83 c6 00 00 00 00        	mov	byte ptr [rbx + 0xc6], 0x0
1405a6176: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a617d: e8 ce 20 f3 ff              	call	0x1404d8250 <.text+0x4d7250>
1405a6182: c6 83 e4 00 00 00 00        	mov	byte ptr [rbx + 0xe4], 0x0
1405a6189: c6 83 e5 00 00 00 00        	mov	byte ptr [rbx + 0xe5], 0x0
1405a6190: c6 83 e6 00 00 00 01        	mov	byte ptr [rbx + 0xe6], 0x1
1405a6197: c6 83 e7 00 00 00 00        	mov	byte ptr [rbx + 0xe7], 0x0
1405a619e: 48 8d 8b e0 02 00 00        	lea	rcx, [rbx + 0x2e0]
1405a61a5: e8 b6 bb 05 00              	call	0x140601d60 <.text+0x600d60>
1405a61aa: b0 01                       	mov	al, 0x1
1405a61ac: 89 83 e8 00 00 00           	mov	dword ptr [rbx + 0xe8], eax
1405a61b2: c7 83 28 01 00 00 00 00 00 00       	mov	dword ptr [rbx + 0x128], 0x0
1405a61bc: 48 8b 8b a0 02 00 00        	mov	rcx, qword ptr [rbx + 0x2a0]
1405a61c3: ff 15 27 c2 be 02           	call	qword ptr [rip + 0x2bec227] # 0x1431923f0
1405a61c9: 48 8b 83 90 02 00 00        	mov	rax, qword ptr [rbx + 0x290]
1405a61d0: f0                          	lock
1405a61d1: 48 ff 08                    	dec	qword ptr [rax]
1405a61d4: 75 0c                       	jne	0x1405a61e2 <.text+0x5a51e2>
1405a61d6: 48 8d 8b 90 02 00 00        	lea	rcx, [rbx + 0x290]
1405a61dd: e8 7e d9 7a 00              	call	0x140d53b60 <.text+0xd52b60>
1405a61e2: 48 8b 83 98 02 00 00        	mov	rax, qword ptr [rbx + 0x298]
1405a61e9: f0                          	lock
1405a61ea: 48 ff 08                    	dec	qword ptr [rax]
1405a61ed: 8b bb e8 00 00 00           	mov	edi, dword ptr [rbx + 0xe8]
1405a61f3: 44 8b b3 28 01 00 00        	mov	r14d, dword ptr [rbx + 0x128]
1405a61fa: 75 2f                       	jne	0x1405a622b <.text+0x5a522b>
1405a61fc: 48 8d 8b 98 02 00 00        	lea	rcx, [rbx + 0x298]
1405a6203: c6 83 d4 00 00 00 00        	mov	byte ptr [rbx + 0xd4], 0x0
1405a620a: 44 88 b3 d5 00 00 00        	mov	byte ptr [rbx + 0xd5], r14b
1405a6211: 40 88 bb d6 00 00 00        	mov	byte ptr [rbx + 0xd6], dil
1405a6218: c6 83 d7 00 00 00 00        	mov	byte ptr [rbx + 0xd7], 0x0
1405a621f: c6 83 d8 00 00 00 00        	mov	byte ptr [rbx + 0xd8], 0x0
1405a6226: e8 85 db 7a 00              	call	0x140d53db0 <.text+0xd52db0>
1405a622b: 48 8b 83 20 03 00 00        	mov	rax, qword ptr [rbx + 0x320]
1405a6232: 48 85 c0                    	test	rax, rax
1405a6235: 8b b3 30 01 00 00           	mov	esi, dword ptr [rbx + 0x130]
1405a623b: 74 51                       	je	0x1405a628e <.text+0x5a528e>
1405a623d: f0                          	lock
1405a623e: 48 ff 08                    	dec	qword ptr [rax]
1405a6241: 75 4b                       	jne	0x1405a628e <.text+0x5a528e>
1405a6243: 44 88 b3 aa 00 00 00        	mov	byte ptr [rbx + 0xaa], r14b
1405a624a: 40 88 b3 ab 00 00 00        	mov	byte ptr [rbx + 0xab], sil
1405a6251: 40 88 bb ac 00 00 00        	mov	byte ptr [rbx + 0xac], dil
1405a6258: c6 83 ad 00 00 00 00        	mov	byte ptr [rbx + 0xad], 0x0
1405a625f: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a6266: c6 83 af 00 00 00 00        	mov	byte ptr [rbx + 0xaf], 0x0
1405a626d: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a6274: c6 83 b1 00 00 00 00        	mov	byte ptr [rbx + 0xb1], 0x0
1405a627b: c6 83 b2 00 00 00 00        	mov	byte ptr [rbx + 0xb2], 0x0
1405a6282: 48 8d 8b 20 03 00 00        	lea	rcx, [rbx + 0x320]
1405a6289: e8 52 16 0f 00              	call	0x1406978e0 <.text+0x6968e0>
1405a628e: 48 8b 83 38 01 00 00        	mov	rax, qword ptr [rbx + 0x138]
1405a6295: f0                          	lock
1405a6296: 48 ff 08                    	dec	qword ptr [rax]
1405a6299: 75 36                       	jne	0x1405a62d1 <.text+0x5a52d1>
1405a629b: 44 88 b3 f8 00 00 00        	mov	byte ptr [rbx + 0xf8], r14b
1405a62a2: 40 88 b3 f9 00 00 00        	mov	byte ptr [rbx + 0xf9], sil
1405a62a9: 40 88 bb fa 00 00 00        	mov	byte ptr [rbx + 0xfa], dil
1405a62b0: c6 83 fb 00 00 00 00        	mov	byte ptr [rbx + 0xfb], 0x0
1405a62b7: c6 83 fc 00 00 00 00        	mov	byte ptr [rbx + 0xfc], 0x0
1405a62be: c6 83 fd 00 00 00 00        	mov	byte ptr [rbx + 0xfd], 0x0
1405a62c5: 48 8d 8b 38 01 00 00        	lea	rcx, [rbx + 0x138]
1405a62cc: e8 bf 5b 5b 00              	call	0x140b5be90 <.text+0xb5ae90>
1405a62d1: 45 84 ed                    	test	r13b, r13b
1405a62d4: 74 2f                       	je	0x1405a6305 <.text+0x5a5305>
1405a62d6: 40 88 bb 15 01 00 00        	mov	byte ptr [rbx + 0x115], dil
1405a62dd: 44 88 b3 16 01 00 00        	mov	byte ptr [rbx + 0x116], r14b
1405a62e4: 40 88 b3 17 01 00 00        	mov	byte ptr [rbx + 0x117], sil
1405a62eb: c6 83 18 01 00 00 00        	mov	byte ptr [rbx + 0x118], 0x0
1405a62f2: c6 83 19 01 00 00 00        	mov	byte ptr [rbx + 0x119], 0x0
1405a62f9: 48 8d 8b 60 01 00 00        	lea	rcx, [rbx + 0x160]
1405a6300: e8 6b 05 06 00              	call	0x140606870 <.text+0x605870>
1405a6305: 40 88 bb 20 01 00 00        	mov	byte ptr [rbx + 0x120], dil
1405a630c: 44 88 b3 21 01 00 00        	mov	byte ptr [rbx + 0x121], r14b
1405a6313: 40 88 b3 22 01 00 00        	mov	byte ptr [rbx + 0x122], sil
1405a631a: c6 83 23 01 00 00 00        	mov	byte ptr [rbx + 0x123], 0x0
1405a6321: 48 8b 8d c0 37 00 00        	mov	rcx, qword ptr [rbp + 0x37c0]
1405a6328: e8 53 86 04 00              	call	0x1405ee980 <.text+0x5ed980>
1405a632d: 40 88 bb 24 01 00 00        	mov	byte ptr [rbx + 0x124], dil
1405a6334: 44 88 b3 25 01 00 00        	mov	byte ptr [rbx + 0x125], r14b
1405a633b: 40 88 b3 26 01 00 00        	mov	byte ptr [rbx + 0x126], sil
1405a6342: c6 83 27 01 00 00 00        	mov	byte ptr [rbx + 0x127], 0x0
1405a6349: 48 8b 8d b8 37 00 00        	mov	rcx, qword ptr [rbp + 0x37b8]
1405a6350: e8 6b b5 04 00              	call	0x1405f18c0 <.text+0x5f08c0>
1405a6355: 40 84 f6                    	test	sil, sil
1405a6358: 48 8b 8d b0 37 00 00        	mov	rcx, qword ptr [rbp + 0x37b0]
1405a635f: 74 3c                       	je	0x1405a639d <.text+0x5a539d>
1405a6361: e8 1a 78 05 00              	call	0x1405fdb80 <.text+0x5fcb80>
1405a6366: 48 8b 8d b0 37 00 00        	mov	rcx, qword ptr [rbp + 0x37b0]
1405a636d: 48 8b 91 c8 00 00 00        	mov	rdx, qword ptr [rcx + 0xc8]
1405a6374: 48 89 d0                    	mov	rax, rdx
1405a6377: 48 f7 d8                    	neg	rax
1405a637a: 8b bb e8 00 00 00           	mov	edi, dword ptr [rbx + 0xe8]
1405a6380: 44 8b b3 28 01 00 00        	mov	r14d, dword ptr [rbx + 0x128]
1405a6387: 70 14                       	jo	0x1405a639d <.text+0x5a539d>
1405a6389: 73 12                       	jae	0x1405a639d <.text+0x5a539d>
1405a638b: 48 8b 89 d0 00 00 00        	mov	rcx, qword ptr [rcx + 0xd0]
1405a6392: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a6398: e8 83 d8 06 00              	call	0x140613c20 <.text+0x612c20>
1405a639d: 45 84 f6                    	test	r14b, r14b
1405a63a0: 74 13                       	je	0x1405a63b5 <.text+0x5a53b5>
1405a63a2: 40 88 bb 1d 01 00 00        	mov	byte ptr [rbx + 0x11d], dil
1405a63a9: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a63b0: e8 1b 09 04 00              	call	0x1405e6cd0 <.text+0x5e5cd0>
1405a63b5: 45 84 ed                    	test	r13b, r13b
1405a63b8: 74 19                       	je	0x1405a63d3 <.text+0x5a53d3>
1405a63ba: 48 8b 83 40 01 00 00        	mov	rax, qword ptr [rbx + 0x140]
1405a63c1: f0                          	lock
1405a63c2: 48 ff 08                    	dec	qword ptr [rax]
1405a63c5: 75 0c                       	jne	0x1405a63d3 <.text+0x5a53d3>
1405a63c7: 48 8d 8b 40 01 00 00        	lea	rcx, [rbx + 0x140]
1405a63ce: e8 6d b4 08 00              	call	0x140631840 <.text+0x630840>
1405a63d3: 48 8d 8b c0 02 00 00        	lea	rcx, [rbx + 0x2c0]
1405a63da: e8 a1 ce 02 00              	call	0x1405d3280 <.text+0x5d2280>
1405a63df: 0f 28 b5 90 36 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x3690]
1405a63e6: 0f 28 bd a0 36 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x36a0]
1405a63ed: 44 0f 28 85 b0 36 00 00     	movaps	xmm8, xmmword ptr [rbp + 0x36b0]
1405a63f5: 44 0f 28 8d c0 36 00 00     	movaps	xmm9, xmmword ptr [rbp + 0x36c0]
1405a63fd: 44 0f 28 95 d0 36 00 00     	movaps	xmm10, xmmword ptr [rbp + 0x36d0]
1405a6405: 44 0f 28 9d e0 36 00 00     	movaps	xmm11, xmmword ptr [rbp + 0x36e0]
1405a640d: 44 0f 28 a5 f0 36 00 00     	movaps	xmm12, xmmword ptr [rbp + 0x36f0]
1405a6415: 44 0f 28 ad 00 37 00 00     	movaps	xmm13, xmmword ptr [rbp + 0x3700]
1405a641d: 44 0f 28 b5 10 37 00 00     	movaps	xmm14, xmmword ptr [rbp + 0x3710]
1405a6425: 44 0f 28 bd 20 37 00 00     	movaps	xmm15, xmmword ptr [rbp + 0x3720]
1405a642d: 48 8d a5 38 37 00 00        	lea	rsp, [rbp + 0x3738]
1405a6434: 5b                          	pop	rbx
1405a6435: 5f                          	pop	rdi
1405a6436: 5e                          	pop	rsi
1405a6437: 41 5c                       	pop	r12
1405a6439: 41 5d                       	pop	r13
1405a643b: 41 5e                       	pop	r14
1405a643d: 41 5f                       	pop	r15
1405a643f: 5d                          	pop	rbp
1405a6440: c3                          	ret
1405a6441: 0f 28 83 f0 1f 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x1ff0]
1405a6448: 0f 29 83 40 04 00 00        	movaps	xmmword ptr [rbx + 0x440], xmm0
1405a644f: 0f 28 83 f0 2b 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x2bf0]
1405a6456: 0f 28 8b 00 2c 00 00        	movaps	xmm1, xmmword ptr [rbx + 0x2c00]
1405a645d: 0f 29 83 00 0b 00 00        	movaps	xmmword ptr [rbx + 0xb00], xmm0
1405a6464: 0f 29 8b 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm1
1405a646b: 49 ff c8                    	dec	r8
1405a646e: 4c 01 c2                    	add	rdx, r8
1405a6471: 49 0f 43 d0                 	cmovae	rdx, r8
1405a6475: 48 89 93 40 22 00 00        	mov	qword ptr [rbx + 0x2240], rdx
1405a647c: 48 89 83 48 22 00 00        	mov	qword ptr [rbx + 0x2248], rax
1405a6483: 48 6b c2 38                 	imul	rax, rdx, 0x38
1405a6487: 4c 89 0c 01                 	mov	qword ptr [rcx + rax], r9
1405a648b: 0f 28 83 40 04 00 00        	movaps	xmm0, xmmword ptr [rbx + 0x440]
1405a6492: 0f 11 44 01 08              	movups	xmmword ptr [rcx + rax + 0x8], xmm0
1405a6497: 0f 28 83 00 0b 00 00        	movaps	xmm0, xmmword ptr [rbx + 0xb00]
1405a649e: 0f 28 8b 10 0b 00 00        	movaps	xmm1, xmmword ptr [rbx + 0xb10]
1405a64a5: 0f 11 4c 01 28              	movups	xmmword ptr [rcx + rax + 0x28], xmm1
1405a64aa: 0f 11 44 01 18              	movups	xmmword ptr [rcx + rax + 0x18], xmm0
1405a64af: 0f 10 83 30 22 00 00        	movups	xmm0, xmmword ptr [rbx + 0x2230]
1405a64b6: 48 8b 83 40 22 00 00        	mov	rax, qword ptr [rbx + 0x2240]
1405a64bd: 48 89 83 50 04 00 00        	mov	qword ptr [rbx + 0x450], rax
1405a64c4: 48 8b 83 48 22 00 00        	mov	rax, qword ptr [rbx + 0x2248]
1405a64cb: 48 89 83 58 04 00 00        	mov	qword ptr [rbx + 0x458], rax
1405a64d2: 0f 29 83 40 04 00 00        	movaps	xmmword ptr [rbx + 0x440], xmm0
1405a64d9: c6 83 d9 00 00 00 00        	mov	byte ptr [rbx + 0xd9], 0x0
1405a64e0: c6 83 da 00 00 00 00        	mov	byte ptr [rbx + 0xda], 0x0
1405a64e7: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a64ee: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a64f5: e8 f6 8a 37 00              	call	0x14091eff0 <.text+0x91dff0>
1405a64fa: c6 83 d9 00 00 00 00        	mov	byte ptr [rbx + 0xd9], 0x0
1405a6501: c6 83 da 00 00 00 00        	mov	byte ptr [rbx + 0xda], 0x0
1405a6508: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a650f: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405a6516: e8 15 21 b7 ff              	call	0x140118630 <.text+0x117630>
1405a651b: e9 c0 cb ff ff              	jmp	0x1405a30e0 <.text+0x5a20e0>
1405a6520: 48 8d 05 81 86 45 02        	lea	rax, [rip + 0x2458681]  # 0x1429feba8
1405a6527: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a652e: 48 c7 83 08 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x1
1405a6539: 48 c7 83 10 0b 00 00 08 00 00 00    	mov	qword ptr [rbx + 0xb10], 0x8
1405a6544: 0f 11 b3 18 0b 00 00        	movups	xmmword ptr [rbx + 0xb18], xmm6
1405a654b: c6 83 bc 00 00 00 01        	mov	byte ptr [rbx + 0xbc], 0x1
1405a6552: c6 83 bd 00 00 00 01        	mov	byte ptr [rbx + 0xbd], 0x1
1405a6559: c6 83 be 00 00 00 01        	mov	byte ptr [rbx + 0xbe], 0x1
1405a6560: c6 83 bf 00 00 00 01        	mov	byte ptr [rbx + 0xbf], 0x1
1405a6567: c6 83 c0 00 00 00 01        	mov	byte ptr [rbx + 0xc0], 0x1
1405a656e: c6 83 c1 00 00 00 01        	mov	byte ptr [rbx + 0xc1], 0x1
1405a6575: 48 8d 15 9c 86 45 02        	lea	rdx, [rip + 0x245869c]  # 0x1429fec18
1405a657c: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6583: e8 08 2e 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a6588: e9 b3 0a 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a658d: c6 83 e3 00 00 00 01        	mov	byte ptr [rbx + 0xe3], 0x1
1405a6594: 48 8d 0d fd 93 45 02        	lea	rcx, [rip + 0x24593fd]  # 0x1429ff998
1405a659b: 4c 8d 05 1e 94 45 02        	lea	r8, [rip + 0x245941e]   # 0x1429ff9c0
1405a65a2: ba 21 00 00 00              	mov	edx, 0x21
1405a65a7: e8 84 2e 33 02              	call	0x1428d9430 <.text+0x28d8430>
1405a65ac: e9 8f 0a 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a65b1: b8 3b 00 00 00              	mov	eax, 0x3b
1405a65b6: 48 8d 0d 43 82 45 02        	lea	rcx, [rip + 0x2458243]  # 0x1429fe800
1405a65bd: 48 89 8b 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rcx
1405a65c4: 48 89 83 08 0b 00 00        	mov	qword ptr [rbx + 0xb08], rax
1405a65cb: c6 83 d9 00 00 00 01        	mov	byte ptr [rbx + 0xd9], 0x1
1405a65d2: c6 83 da 00 00 00 01        	mov	byte ptr [rbx + 0xda], 0x1
1405a65d9: 48 8d 15 f0 82 45 02        	lea	rdx, [rip + 0x24582f0]  # 0x1429fe8d0
1405a65e0: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a65e7: e8 a4 7a 32 02              	call	0x1428ce090 <.text+0x28cd090>
1405a65ec: e9 4f 0a 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a65f1: c6 83 c2 00 00 00 01        	mov	byte ptr [rbx + 0xc2], 0x1
1405a65f8: c6 83 c3 00 00 00 01        	mov	byte ptr [rbx + 0xc3], 0x1
1405a65ff: c6 83 c4 00 00 00 01        	mov	byte ptr [rbx + 0xc4], 0x1
1405a6606: c6 83 c5 00 00 00 01        	mov	byte ptr [rbx + 0xc5], 0x1
1405a660d: c6 83 c6 00 00 00 01        	mov	byte ptr [rbx + 0xc6], 0x1
1405a6614: 48 8d 0d 1d 95 45 02        	lea	rcx, [rip + 0x245951d]  # 0x1429ffb38
1405a661b: e8 f0 2c 33 02              	call	0x1428d9310 <.text+0x28d8310>
1405a6620: e9 1b 0a 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6625: 48 8d 05 74 8b 45 02        	lea	rax, [rip + 0x2458b74]  # 0x1429ff1a0
1405a662c: 48 89 83 90 25 00 00        	mov	qword ptr [rbx + 0x2590], rax
1405a6633: 48 c7 83 98 25 00 00 2b 00 00 00    	mov	qword ptr [rbx + 0x2598], 0x2b
1405a663e: 4c 89 b3 f0 2b 00 00        	mov	qword ptr [rbx + 0x2bf0], r14
1405a6645: 48 8d 05 44 38 b6 ff        	lea	rax, [rip - 0x49c7bc]   # 0x140109e90 <.text+0x108e90>
1405a664c: 48 89 83 f8 2b 00 00        	mov	qword ptr [rbx + 0x2bf8], rax
1405a6653: 48 8d 05 26 8b 45 02        	lea	rax, [rip + 0x2458b26]  # 0x1429ff180
1405a665a: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a6661: 48 c7 83 08 0b 00 00 02 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x2
1405a666c: 48 c7 83 20 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb20], 0x0
1405a6677: 48 8d 83 f0 2b 00 00        	lea	rax, [rbx + 0x2bf0]
1405a667e: 48 89 83 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], rax
1405a6685: 48 c7 83 18 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb18], 0x1
1405a6690: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a6697: 48 8d 15 ca 94 45 02        	lea	rdx, [rip + 0x24594ca]  # 0x1429ffb68
1405a669e: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a66a5: e8 e6 2c 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a66aa: e9 91 09 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a66af: c6 83 c2 00 00 00 01        	mov	byte ptr [rbx + 0xc2], 0x1
1405a66b6: c6 83 c3 00 00 00 01        	mov	byte ptr [rbx + 0xc3], 0x1
1405a66bd: c6 83 c4 00 00 00 01        	mov	byte ptr [rbx + 0xc4], 0x1
1405a66c4: c6 83 c5 00 00 00 01        	mov	byte ptr [rbx + 0xc5], 0x1
1405a66cb: c6 83 c6 00 00 00 01        	mov	byte ptr [rbx + 0xc6], 0x1
1405a66d2: 48 8d 0d 77 94 45 02        	lea	rcx, [rip + 0x2459477]  # 0x1429ffb50
1405a66d9: e8 32 2c 33 02              	call	0x1428d9310 <.text+0x28d8310>
1405a66de: e9 5d 09 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a66e3: 48 8d 05 56 89 45 02        	lea	rax, [rip + 0x2458956]  # 0x1429ff040
1405a66ea: 48 89 83 90 25 00 00        	mov	qword ptr [rbx + 0x2590], rax
1405a66f1: 48 c7 83 98 25 00 00 36 00 00 00    	mov	qword ptr [rbx + 0x2598], 0x36
1405a66fc: 4c 89 b3 f0 2b 00 00        	mov	qword ptr [rbx + 0x2bf0], r14
1405a6703: 48 8d 05 86 37 b6 ff        	lea	rax, [rip - 0x49c87a]   # 0x140109e90 <.text+0x108e90>
1405a670a: 48 89 83 f8 2b 00 00        	mov	qword ptr [rbx + 0x2bf8], rax
1405a6711: 48 8d 05 68 8a 45 02        	lea	rax, [rip + 0x2458a68]  # 0x1429ff180
1405a6718: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a671f: 48 c7 83 08 0b 00 00 02 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x2
1405a672a: 48 c7 83 20 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb20], 0x0
1405a6735: 48 8d 83 f0 2b 00 00        	lea	rax, [rbx + 0x2bf0]
1405a673c: 48 89 83 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], rax
1405a6743: 48 c7 83 18 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb18], 0x1
1405a674e: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a6755: 48 8d 15 54 94 45 02        	lea	rdx, [rip + 0x2459454]  # 0x1429ffbb0
1405a675c: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6763: e8 28 2c 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a6768: e9 d3 08 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a676d: 48 8d 05 bc 8a 45 02        	lea	rax, [rip + 0x2458abc]  # 0x1429ff230
1405a6774: 48 89 83 90 25 00 00        	mov	qword ptr [rbx + 0x2590], rax
1405a677b: 48 c7 83 98 25 00 00 40 00 00 00    	mov	qword ptr [rbx + 0x2598], 0x40
1405a6786: 4c 89 b3 f0 2b 00 00        	mov	qword ptr [rbx + 0x2bf0], r14
1405a678d: 48 8d 05 fc 36 b6 ff        	lea	rax, [rip - 0x49c904]   # 0x140109e90 <.text+0x108e90>
1405a6794: 48 89 83 f8 2b 00 00        	mov	qword ptr [rbx + 0x2bf8], rax
1405a679b: 48 8d 05 de 89 45 02        	lea	rax, [rip + 0x24589de]  # 0x1429ff180
1405a67a2: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a67a9: 48 c7 83 08 0b 00 00 02 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x2
1405a67b4: 48 c7 83 20 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb20], 0x0
1405a67bf: 48 8d 83 f0 2b 00 00        	lea	rax, [rbx + 0x2bf0]
1405a67c6: 48 89 83 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], rax
1405a67cd: 48 c7 83 18 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb18], 0x1
1405a67d8: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a67df: 48 8d 15 e2 93 45 02        	lea	rdx, [rip + 0x24593e2]  # 0x1429ffbc8
1405a67e6: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a67ed: e8 9e 2b 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a67f2: e9 49 08 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a67f7: 48 8d 05 9d 8a 45 02        	lea	rax, [rip + 0x2458a9d]  # 0x1429ff29b
1405a67fe: 48 89 83 90 25 00 00        	mov	qword ptr [rbx + 0x2590], rax
1405a6805: 48 c7 83 98 25 00 00 25 00 00 00    	mov	qword ptr [rbx + 0x2598], 0x25
1405a6810: 4c 89 b3 f0 2b 00 00        	mov	qword ptr [rbx + 0x2bf0], r14
1405a6817: 48 8d 05 72 36 b6 ff        	lea	rax, [rip - 0x49c98e]   # 0x140109e90 <.text+0x108e90>
1405a681e: 48 89 83 f8 2b 00 00        	mov	qword ptr [rbx + 0x2bf8], rax
1405a6825: 48 8d 05 54 89 45 02        	lea	rax, [rip + 0x2458954]  # 0x1429ff180
1405a682c: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a6833: 48 c7 83 08 0b 00 00 02 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x2
1405a683e: 48 c7 83 20 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb20], 0x0
1405a6849: 48 8d 83 f0 2b 00 00        	lea	rax, [rbx + 0x2bf0]
1405a6850: 48 89 83 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], rax
1405a6857: 48 c7 83 18 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb18], 0x1
1405a6862: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a6869: 48 8d 15 70 93 45 02        	lea	rdx, [rip + 0x2459370]  # 0x1429ffbe0
1405a6870: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6877: e8 14 2b 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a687c: e9 bf 07 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6881: 48 8d 05 43 89 45 02        	lea	rax, [rip + 0x2458943]  # 0x1429ff1cb
1405a6888: 48 89 83 90 25 00 00        	mov	qword ptr [rbx + 0x2590], rax
1405a688f: 48 c7 83 98 25 00 00 33 00 00 00    	mov	qword ptr [rbx + 0x2598], 0x33
1405a689a: 4c 89 b3 f0 2b 00 00        	mov	qword ptr [rbx + 0x2bf0], r14
1405a68a1: 48 8d 05 e8 35 b6 ff        	lea	rax, [rip - 0x49ca18]   # 0x140109e90 <.text+0x108e90>
1405a68a8: 48 89 83 f8 2b 00 00        	mov	qword ptr [rbx + 0x2bf8], rax
1405a68af: 48 8d 05 ca 88 45 02        	lea	rax, [rip + 0x24588ca]  # 0x1429ff180
1405a68b6: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a68bd: 48 c7 83 08 0b 00 00 02 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x2
1405a68c8: 48 c7 83 20 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb20], 0x0
1405a68d3: 48 8d 83 f0 2b 00 00        	lea	rax, [rbx + 0x2bf0]
1405a68da: 48 89 83 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], rax
1405a68e1: 48 c7 83 18 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb18], 0x1
1405a68ec: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a68f3: 48 8d 15 fe 92 45 02        	lea	rdx, [rip + 0x24592fe]  # 0x1429ffbf8
1405a68fa: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6901: e8 8a 2a 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a6906: e9 35 07 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a690b: 48 8d 05 ec 88 45 02        	lea	rax, [rip + 0x24588ec]  # 0x1429ff1fe
1405a6912: 48 89 83 90 25 00 00        	mov	qword ptr [rbx + 0x2590], rax
1405a6919: 48 c7 83 98 25 00 00 32 00 00 00    	mov	qword ptr [rbx + 0x2598], 0x32
1405a6924: 4c 89 b3 f0 2b 00 00        	mov	qword ptr [rbx + 0x2bf0], r14
1405a692b: 48 8d 05 5e 35 b6 ff        	lea	rax, [rip - 0x49caa2]   # 0x140109e90 <.text+0x108e90>
1405a6932: 48 89 83 f8 2b 00 00        	mov	qword ptr [rbx + 0x2bf8], rax
1405a6939: 48 8d 05 40 88 45 02        	lea	rax, [rip + 0x2458840]  # 0x1429ff180
1405a6940: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a6947: 48 c7 83 08 0b 00 00 02 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x2
1405a6952: 48 c7 83 20 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb20], 0x0
1405a695d: 48 8d 83 f0 2b 00 00        	lea	rax, [rbx + 0x2bf0]
1405a6964: 48 89 83 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], rax
1405a696b: 48 c7 83 18 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb18], 0x1
1405a6976: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a697d: 48 8d 15 8c 92 45 02        	lea	rdx, [rip + 0x245928c]  # 0x1429ffc10
1405a6984: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a698b: e8 00 2a 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a6990: e9 ab 06 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6995: 48 8d 05 d4 88 45 02        	lea	rax, [rip + 0x24588d4]  # 0x1429ff270
1405a699c: 48 89 83 90 25 00 00        	mov	qword ptr [rbx + 0x2590], rax
1405a69a3: 48 c7 83 98 25 00 00 2b 00 00 00    	mov	qword ptr [rbx + 0x2598], 0x2b
1405a69ae: 48 8d 83 90 25 00 00        	lea	rax, [rbx + 0x2590]
1405a69b5: 48 89 83 f0 2b 00 00        	mov	qword ptr [rbx + 0x2bf0], rax
1405a69bc: 48 8d 05 cd 34 b6 ff        	lea	rax, [rip - 0x49cb33]   # 0x140109e90 <.text+0x108e90>
1405a69c3: 48 89 83 f8 2b 00 00        	mov	qword ptr [rbx + 0x2bf8], rax
1405a69ca: 48 8d 05 af 87 45 02        	lea	rax, [rip + 0x24587af]  # 0x1429ff180
1405a69d1: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a69d8: 48 c7 83 08 0b 00 00 02 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x2
1405a69e3: 48 c7 83 20 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb20], 0x0
1405a69ee: 48 8d 83 f0 2b 00 00        	lea	rax, [rbx + 0x2bf0]
1405a69f5: 48 89 83 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], rax
1405a69fc: 48 c7 83 18 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb18], 0x1
1405a6a07: c6 83 b3 00 00 00 00        	mov	byte ptr [rbx + 0xb3], 0x0
1405a6a0e: 48 8d 15 13 92 45 02        	lea	rdx, [rip + 0x2459213]  # 0x1429ffc28
1405a6a15: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6a1c: e8 6f 29 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a6a21: e9 1a 06 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6a26: b9 08 00 00 00              	mov	ecx, 0x8
1405a6a2b: ba f0 0a 00 00              	mov	edx, 0xaf0
1405a6a30: e8 cb 25 33 02              	call	0x1428d9000 <.text+0x28d8000>
1405a6a35: e9 06 06 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6a3a: 48 8b 83 b0 04 00 00        	mov	rax, qword ptr [rbx + 0x4b0]
1405a6a41: 48 89 83 70 0b 00 00        	mov	qword ptr [rbx + 0xb70], rax
1405a6a48: 0f 10 83 a0 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x4a0]
1405a6a4f: 0f 29 83 60 0b 00 00        	movaps	xmmword ptr [rbx + 0xb60], xmm0
1405a6a56: 0f 10 83 90 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x490]
1405a6a5d: 0f 29 83 50 0b 00 00        	movaps	xmmword ptr [rbx + 0xb50], xmm0
1405a6a64: 0f 10 83 80 04 00 00        	movups	xmm0, xmmword ptr [rbx + 0x480]
1405a6a6b: 0f 29 83 40 0b 00 00        	movaps	xmmword ptr [rbx + 0xb40], xmm0
1405a6a72: 66 0f 10 83 40 04 00 00     	movupd	xmm0, xmmword ptr [rbx + 0x440]
1405a6a7a: 0f 10 8b 50 04 00 00        	movups	xmm1, xmmword ptr [rbx + 0x450]
1405a6a81: 0f 10 93 60 04 00 00        	movups	xmm2, xmmword ptr [rbx + 0x460]
1405a6a88: 0f 10 9b 70 04 00 00        	movups	xmm3, xmmword ptr [rbx + 0x470]
1405a6a8f: 0f 29 9b 30 0b 00 00        	movaps	xmmword ptr [rbx + 0xb30], xmm3
1405a6a96: 0f 29 93 20 0b 00 00        	movaps	xmmword ptr [rbx + 0xb20], xmm2
1405a6a9d: 0f 29 8b 10 0b 00 00        	movaps	xmmword ptr [rbx + 0xb10], xmm1
1405a6aa4: 66 0f 29 83 00 0b 00 00     	movapd	xmmword ptr [rbx + 0xb00], xmm0
1405a6aac: 48 8d 05 a5 8c 45 02        	lea	rax, [rip + 0x2458ca5]  # 0x1429ff758
1405a6ab3: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405a6ab8: 48 8d 0d f1 7e 45 02        	lea	rcx, [rip + 0x2457ef1]  # 0x1429fe9b0
1405a6abf: 4c 8d 0d 5a 7f 45 02        	lea	r9, [rip + 0x2457f5a]   # 0x1429fea20
1405a6ac6: 4c 8d 83 00 0b 00 00        	lea	r8, [rbx + 0xb00]
1405a6acd: ba 2b 00 00 00              	mov	edx, 0x2b
1405a6ad2: e8 89 2c 33 02              	call	0x1428d9760 <.text+0x28d8760>
1405a6ad7: e9 64 05 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6adc: 48 8d 0d 2d bf 38 02        	lea	rcx, [rip + 0x238bf2d]  # 0x142932a10
1405a6ae3: e8 e8 15 33 02              	call	0x1428d80d0 <.text+0x28d70d0>
1405a6ae8: e9 53 05 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6aed: c6 83 08 01 00 00 01        	mov	byte ptr [rbx + 0x108], 0x1
1405a6af4: c6 83 09 01 00 00 01        	mov	byte ptr [rbx + 0x109], 0x1
1405a6afb: c6 83 0a 01 00 00 01        	mov	byte ptr [rbx + 0x10a], 0x1
1405a6b02: c6 83 0b 01 00 00 01        	mov	byte ptr [rbx + 0x10b], 0x1
1405a6b09: c6 83 0c 01 00 00 01        	mov	byte ptr [rbx + 0x10c], 0x1
1405a6b10: c6 83 0d 01 00 00 01        	mov	byte ptr [rbx + 0x10d], 0x1
1405a6b17: c6 83 0e 01 00 00 01        	mov	byte ptr [rbx + 0x10e], 0x1
1405a6b1e: c6 83 0f 01 00 00 01        	mov	byte ptr [rbx + 0x10f], 0x1
1405a6b25: b9 08 00 00 00              	mov	ecx, 0x8
1405a6b2a: ba b0 00 00 00              	mov	edx, 0xb0
1405a6b2f: e8 cc 24 33 02              	call	0x1428d9000 <.text+0x28d8000>
1405a6b34: e9 07 05 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6b39: 48 8d 05 d0 9c 3a 02        	lea	rax, [rip + 0x23a9cd0]  # 0x142950810
1405a6b40: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a6b47: 48 c7 83 08 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x1
1405a6b52: 48 c7 83 10 0b 00 00 08 00 00 00    	mov	qword ptr [rbx + 0xb10], 0x8
1405a6b5d: 0f 57 c0                    	xorps	xmm0, xmm0
1405a6b60: 0f 11 83 18 0b 00 00        	movups	xmmword ptr [rbx + 0xb18], xmm0
1405a6b67: c6 83 10 01 00 00 01        	mov	byte ptr [rbx + 0x110], 0x1
1405a6b6e: 48 8d 15 5b 8c 45 02        	lea	rdx, [rip + 0x2458c5b]  # 0x1429ff7d0
1405a6b75: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6b7c: e8 0f 28 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a6b81: e9 ba 04 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6b86: 48 8d 05 83 9c 3a 02        	lea	rax, [rip + 0x23a9c83]  # 0x142950810
1405a6b8d: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a6b94: 48 c7 83 08 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x1
1405a6b9f: 48 c7 83 10 0b 00 00 08 00 00 00    	mov	qword ptr [rbx + 0xb10], 0x8
1405a6baa: 0f 57 c0                    	xorps	xmm0, xmm0
1405a6bad: 0f 11 83 18 0b 00 00        	movups	xmmword ptr [rbx + 0xb18], xmm0
1405a6bb4: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a6bbb: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a6bc2: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a6bc9: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a6bd0: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
1405a6bd7: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a6bde: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
1405a6be5: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a6bec: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a6bf3: 48 8d 15 ee 8b 45 02        	lea	rdx, [rip + 0x2458bee]  # 0x1429ff7e8
1405a6bfa: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6c01: e8 8a 27 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a6c06: e9 35 04 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6c0b: 48 8d 05 fe 9b 3a 02        	lea	rax, [rip + 0x23a9bfe]  # 0x142950810
1405a6c12: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a6c19: 48 c7 83 08 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x1
1405a6c24: 48 c7 83 10 0b 00 00 08 00 00 00    	mov	qword ptr [rbx + 0xb10], 0x8
1405a6c2f: 0f 57 c0                    	xorps	xmm0, xmm0
1405a6c32: 0f 11 83 18 0b 00 00        	movups	xmmword ptr [rbx + 0xb18], xmm0
1405a6c39: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a6c40: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a6c47: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a6c4e: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a6c55: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
1405a6c5c: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a6c63: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
1405a6c6a: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a6c71: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a6c78: 48 8d 15 81 8b 45 02        	lea	rdx, [rip + 0x2458b81]  # 0x1429ff800
1405a6c7f: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6c86: e8 05 27 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a6c8b: e9 b0 03 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6c90: 48 8d 05 79 9b 3a 02        	lea	rax, [rip + 0x23a9b79]  # 0x142950810
1405a6c97: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a6c9e: 48 c7 83 08 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x1
1405a6ca9: 48 c7 83 10 0b 00 00 08 00 00 00    	mov	qword ptr [rbx + 0xb10], 0x8
1405a6cb4: 0f 57 c0                    	xorps	xmm0, xmm0
1405a6cb7: 0f 11 83 18 0b 00 00        	movups	xmmword ptr [rbx + 0xb18], xmm0
1405a6cbe: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a6cc5: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a6ccc: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a6cd3: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a6cda: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
1405a6ce1: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a6ce8: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
1405a6cef: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a6cf6: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a6cfd: 48 8d 15 14 8b 45 02        	lea	rdx, [rip + 0x2458b14]  # 0x1429ff818
1405a6d04: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6d0b: e8 80 26 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a6d10: e9 2b 03 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6d15: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a6d1c: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a6d23: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a6d2a: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a6d31: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a6d38: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a6d3f: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a6d46: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a6d4d: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a6d54: 48 8d 05 95 8b 45 02        	lea	rax, [rip + 0x2458b95]  # 0x1429ff8f0
1405a6d5b: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405a6d60: 48 8d 0d 49 7c 45 02        	lea	rcx, [rip + 0x2457c49]  # 0x1429fe9b0
1405a6d67: 4c 8d 0d 02 7c 45 02        	lea	r9, [rip + 0x2457c02]   # 0x1429fe970
1405a6d6e: 4c 8d 83 70 03 00 00        	lea	r8, [rbx + 0x370]
1405a6d75: ba 2b 00 00 00              	mov	edx, 0x2b
1405a6d7a: e8 e1 29 33 02              	call	0x1428d9760 <.text+0x28d8760>
1405a6d7f: e9 bc 02 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6d84: 48 8d 05 85 9a 3a 02        	lea	rax, [rip + 0x23a9a85]  # 0x142950810
1405a6d8b: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a6d92: 48 c7 83 08 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x1
1405a6d9d: 48 c7 83 10 0b 00 00 08 00 00 00    	mov	qword ptr [rbx + 0xb10], 0x8
1405a6da8: 0f 57 c0                    	xorps	xmm0, xmm0
1405a6dab: 0f 11 83 18 0b 00 00        	movups	xmmword ptr [rbx + 0xb18], xmm0
1405a6db2: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a6db9: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a6dc0: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a6dc7: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a6dce: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a6dd5: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a6ddc: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a6de3: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a6dea: 48 8d 15 77 8b 45 02        	lea	rdx, [rip + 0x2458b77]  # 0x1429ff968
1405a6df1: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6df8: e8 93 25 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a6dfd: e9 3e 02 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6e02: 4c 8d 05 87 7c 45 02        	lea	r8, [rip + 0x2457c87]   # 0x1429fea90
1405a6e09: b9 01 00 00 00              	mov	ecx, 0x1
1405a6e0e: ba 12 00 00 00              	mov	edx, 0x12
1405a6e13: e8 cb 21 33 02              	call	0x1428d8fe3 <.text+0x28d7fe3>
1405a6e18: e9 23 02 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6e1d: 48 8b 83 08 0b 00 00        	mov	rax, qword ptr [rbx + 0xb08]
1405a6e24: 48 89 83 40 04 00 00        	mov	qword ptr [rbx + 0x440], rax
1405a6e2b: 48 8d 05 ee 8c 45 02        	lea	rax, [rip + 0x2458cee]  # 0x1429ffb20
1405a6e32: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405a6e37: 48 8d 0d c4 8c 45 02        	lea	rcx, [rip + 0x2458cc4]  # 0x1429ffb02
1405a6e3e: 4c 8d 0d cb 7a 45 02        	lea	r9, [rip + 0x2457acb]   # 0x1429fe910
1405a6e45: 4c 8d 83 40 04 00 00        	lea	r8, [rbx + 0x440]
1405a6e4c: ba 1a 00 00 00              	mov	edx, 0x1a
1405a6e51: e8 0a 29 33 02              	call	0x1428d9760 <.text+0x28d8760>
1405a6e56: e9 e5 01 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6e5b: b9 80 00 00 00              	mov	ecx, 0x80
1405a6e60: ba 00 02 00 00              	mov	edx, 0x200
1405a6e65: e8 96 21 33 02              	call	0x1428d9000 <.text+0x28d8000>
1405a6e6a: e9 d1 01 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6e6f: 49 89 f0                    	mov	r8, rsi
1405a6e72: 4c 89 83 40 04 00 00        	mov	qword ptr [rbx + 0x440], r8
1405a6e79: 48 8d 05 00 8b 45 02        	lea	rax, [rip + 0x2458b00]  # 0x1429ff980
1405a6e80: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405a6e85: 48 8d 0d 24 7b 45 02        	lea	rcx, [rip + 0x2457b24]  # 0x1429fe9b0
1405a6e8c: 4c 8d 0d fd 7a 45 02        	lea	r9, [rip + 0x2457afd]   # 0x1429fe990
1405a6e93: 4c 8d 83 40 04 00 00        	lea	r8, [rbx + 0x440]
1405a6e9a: ba 2b 00 00 00              	mov	edx, 0x2b
1405a6e9f: e8 bc 28 33 02              	call	0x1428d9760 <.text+0x28d8760>
1405a6ea4: e9 97 01 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6ea9: 48 89 b3 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rsi
1405a6eb0: 48 8d 05 21 8b 45 02        	lea	rax, [rip + 0x2458b21]  # 0x1429ff9d8
1405a6eb7: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405a6ebc: 48 8d 0d ed 7a 45 02        	lea	rcx, [rip + 0x2457aed]  # 0x1429fe9b0
1405a6ec3: 4c 8d 0d 46 7a 45 02        	lea	r9, [rip + 0x2457a46]   # 0x1429fe910
1405a6eca: 4c 8d 83 00 0b 00 00        	lea	r8, [rbx + 0xb00]
1405a6ed1: ba 2b 00 00 00              	mov	edx, 0x2b
1405a6ed6: e8 85 28 33 02              	call	0x1428d9760 <.text+0x28d8760>
1405a6edb: e9 60 01 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6ee0: 48 89 93 40 04 00 00        	mov	qword ptr [rbx + 0x440], rdx
1405a6ee7: 48 8d 05 22 8b 45 02        	lea	rax, [rip + 0x2458b22]  # 0x1429ffa10
1405a6eee: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405a6ef3: 48 8d 0d b6 7a 45 02        	lea	rcx, [rip + 0x2457ab6]  # 0x1429fe9b0
1405a6efa: 4c 8d 0d 0f 7a 45 02        	lea	r9, [rip + 0x2457a0f]   # 0x1429fe910
1405a6f01: 4c 8d 83 40 04 00 00        	lea	r8, [rbx + 0x440]
1405a6f08: ba 2b 00 00 00              	mov	edx, 0x2b
1405a6f0d: e8 4e 28 33 02              	call	0x1428d9760 <.text+0x28d8760>
1405a6f12: e9 29 01 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6f17: 4c 89 bb 40 04 00 00        	mov	qword ptr [rbx + 0x440], r15
1405a6f1e: 48 8d 05 43 8b 45 02        	lea	rax, [rip + 0x2458b43]  # 0x1429ffa68
1405a6f25: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405a6f2a: 48 8d 0d 7f 7a 45 02        	lea	rcx, [rip + 0x2457a7f]  # 0x1429fe9b0
1405a6f31: 4c 8d 0d d8 79 45 02        	lea	r9, [rip + 0x24579d8]   # 0x1429fe910
1405a6f38: 4c 8d 83 40 04 00 00        	lea	r8, [rbx + 0x440]
1405a6f3f: ba 2b 00 00 00              	mov	edx, 0x2b
1405a6f44: e8 17 28 33 02              	call	0x1428d9760 <.text+0x28d8760>
1405a6f49: e9 f2 00 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6f4e: 48 8d 05 bb 98 3a 02        	lea	rax, [rip + 0x23a98bb]  # 0x142950810
1405a6f55: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a6f5c: 48 c7 83 08 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x1
1405a6f67: 48 c7 83 10 0b 00 00 08 00 00 00    	mov	qword ptr [rbx + 0xb10], 0x8
1405a6f72: 0f 57 c0                    	xorps	xmm0, xmm0
1405a6f75: 0f 11 83 18 0b 00 00        	movups	xmmword ptr [rbx + 0xb18], xmm0
1405a6f7c: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a6f83: 48 8d 15 f6 8b 45 02        	lea	rdx, [rip + 0x2458bf6]  # 0x1429ffb80
1405a6f8a: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6f91: e8 fa 23 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a6f96: e9 a5 00 00 00              	jmp	0x1405a7040 <.text+0x5a6040>
1405a6f9b: 48 8d 05 6e 98 3a 02        	lea	rax, [rip + 0x23a986e]  # 0x142950810
1405a6fa2: 48 89 83 00 0b 00 00        	mov	qword ptr [rbx + 0xb00], rax
1405a6fa9: 48 c7 83 08 0b 00 00 01 00 00 00    	mov	qword ptr [rbx + 0xb08], 0x1
1405a6fb4: 48 c7 83 10 0b 00 00 08 00 00 00    	mov	qword ptr [rbx + 0xb10], 0x8
1405a6fbf: 0f 57 c0                    	xorps	xmm0, xmm0
1405a6fc2: 0f 11 83 18 0b 00 00        	movups	xmmword ptr [rbx + 0xb18], xmm0
1405a6fc9: c6 83 b3 00 00 00 01        	mov	byte ptr [rbx + 0xb3], 0x1
1405a6fd0: 48 8d 15 c1 8b 45 02        	lea	rdx, [rip + 0x2458bc1]  # 0x1429ffb98
1405a6fd7: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a6fde: e8 ad 23 33 02              	call	0x1428d9390 <.text+0x28d8390>
1405a6fe3: eb 5b                       	jmp	0x1405a7040 <.text+0x5a6040>
1405a6fe5: 4c 8d 05 a4 7a 45 02        	lea	r8, [rip + 0x2457aa4]   # 0x1429fea90
1405a6fec: b9 01 00 00 00              	mov	ecx, 0x1
1405a6ff1: ba 0a 00 00 00              	mov	edx, 0xa
1405a6ff6: e8 e8 1f 33 02              	call	0x1428d8fe3 <.text+0x28d7fe3>
1405a6ffb: eb 43                       	jmp	0x1405a7040 <.text+0x5a6040>
1405a6ffd: b8 48 00 00 00              	mov	eax, 0x48
1405a7002: 48 8d 0d 32 78 45 02        	lea	rcx, [rip + 0x2457832]  # 0x1429fe83b
1405a7009: e9 af f5 ff ff              	jmp	0x1405a65bd <.text+0x5a55bd>
1405a700e: 48 89 83 40 04 00 00        	mov	qword ptr [rbx + 0x440], rax
1405a7015: 48 8d 05 1c 8a 45 02        	lea	rax, [rip + 0x2458a1c]  # 0x1429ffa38
1405a701c: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1405a7021: 48 8d 0d 88 79 45 02        	lea	rcx, [rip + 0x2457988]  # 0x1429fe9b0
1405a7028: 4c 8d 0d 61 79 45 02        	lea	r9, [rip + 0x2457961]   # 0x1429fe990
1405a702f: 4c 8d 83 40 04 00 00        	lea	r8, [rbx + 0x440]
1405a7036: ba 2b 00 00 00              	mov	edx, 0x2b
1405a703b: e8 20 27 33 02              	call	0x1428d9760 <.text+0x28d8760>
1405a7040: 0f 0b                       	ud2
1405a7042: b9 07 00 00 00              	mov	ecx, 0x7
1405a7047: cd 29                       	int	0x29
1405a7049: 0f 0b                       	ud2
1405a704b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1405a7050: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a7055: 55                          	push	rbp
1405a7056: 41 57                       	push	r15
1405a7058: 41 56                       	push	r14
1405a705a: 41 55                       	push	r13
1405a705c: 41 54                       	push	r12
1405a705e: 56                          	push	rsi
1405a705f: 57                          	push	rdi
1405a7060: 53                          	push	rbx
1405a7061: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a7068: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a706f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a7075: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a707e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a7087: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a7090: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a7099: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a70a2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a70ab: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a70b4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a70bc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a70c4: 48 83 e2 80                 	and	rdx, -0x80
1405a70c8: 48 89 d3                    	mov	rbx, rdx
1405a70cb: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a70d2: e8 e9 ef e0 ff              	call	0x1403b60c0 <.text+0x3b50c0>
1405a70d7: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a70df: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a70e7: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a70f0: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a70f9: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a7102: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a710b: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a7114: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a711d: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a7126: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a712c: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a7133: 5b                          	pop	rbx
1405a7134: 5f                          	pop	rdi
1405a7135: 5e                          	pop	rsi
1405a7136: 41 5c                       	pop	r12
1405a7138: 41 5d                       	pop	r13
1405a713a: 41 5e                       	pop	r14
1405a713c: 41 5f                       	pop	r15
1405a713e: 5d                          	pop	rbp
1405a713f: c3                          	ret
1405a7140: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a7145: 55                          	push	rbp
1405a7146: 41 57                       	push	r15
1405a7148: 41 56                       	push	r14
1405a714a: 41 55                       	push	r13
1405a714c: 41 54                       	push	r12
1405a714e: 56                          	push	rsi
1405a714f: 57                          	push	rdi
1405a7150: 53                          	push	rbx
1405a7151: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a7158: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a715f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a7165: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a716e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a7177: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a7180: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a7189: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a7192: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a719b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a71a4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a71ac: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a71b4: 48 83 e2 80                 	and	rdx, -0x80
1405a71b8: 48 89 d3                    	mov	rbx, rdx
1405a71bb: c6 83 e3 00 00 00 00        	mov	byte ptr [rbx + 0xe3], 0x0
1405a71c2: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a71c9: e8 a2 28 05 00              	call	0x1405f9a70 <.text+0x5f8a70>
1405a71ce: c6 83 e3 00 00 00 00        	mov	byte ptr [rbx + 0xe3], 0x0
1405a71d5: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a71dd: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a71e5: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a71ee: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a71f7: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a7200: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a7209: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a7212: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a721b: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a7224: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a722a: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a7231: 5b                          	pop	rbx
1405a7232: 5f                          	pop	rdi
1405a7233: 5e                          	pop	rsi
1405a7234: 41 5c                       	pop	r12
1405a7236: 41 5d                       	pop	r13
1405a7238: 41 5e                       	pop	r14
1405a723a: 41 5f                       	pop	r15
1405a723c: 5d                          	pop	rbp
1405a723d: c3                          	ret
1405a723e: 66 90                       	nop
1405a7240: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a7245: 55                          	push	rbp
1405a7246: 41 57                       	push	r15
1405a7248: 41 56                       	push	r14
1405a724a: 41 55                       	push	r13
1405a724c: 41 54                       	push	r12
1405a724e: 56                          	push	rsi
1405a724f: 57                          	push	rdi
1405a7250: 53                          	push	rbx
1405a7251: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a7258: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a725f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a7265: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a726e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a7277: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a7280: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a7289: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a7292: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a729b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a72a4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a72ac: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a72b4: 48 83 e2 80                 	and	rdx, -0x80
1405a72b8: 48 89 d3                    	mov	rbx, rdx
1405a72bb: 48 8b 8b e8 00 00 00        	mov	rcx, qword ptr [rbx + 0xe8]
1405a72c2: 48 8b 83 28 01 00 00        	mov	rax, qword ptr [rbx + 0x128]
1405a72c9: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
1405a72cd: 48 85 d2                    	test	rdx, rdx
1405a72d0: 74 10                       	je	0x1405a72e2 <.text+0x5a62e2>
1405a72d2: 48 8b 83 28 01 00 00        	mov	rax, qword ptr [rbx + 0x128]
1405a72d9: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
1405a72dd: e8 3e c9 06 00              	call	0x140613c20 <.text+0x612c20>
1405a72e2: ba 18 00 00 00              	mov	edx, 0x18
1405a72e7: 41 b8 08 00 00 00           	mov	r8d, 0x8
1405a72ed: 48 8b 8b 50 01 00 00        	mov	rcx, qword ptr [rbx + 0x150]
1405a72f4: e8 27 c9 06 00              	call	0x140613c20 <.text+0x612c20>
1405a72f9: c6 83 db 00 00 00 00        	mov	byte ptr [rbx + 0xdb], 0x0
1405a7300: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a7308: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a7310: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a7319: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a7322: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a732b: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a7334: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a733d: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a7346: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a734f: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a7355: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a735c: 5b                          	pop	rbx
1405a735d: 5f                          	pop	rdi
1405a735e: 5e                          	pop	rsi
1405a735f: 41 5c                       	pop	r12
1405a7361: 41 5d                       	pop	r13
1405a7363: 41 5e                       	pop	r14
1405a7365: 41 5f                       	pop	r15
1405a7367: 5d                          	pop	rbp
1405a7368: c3                          	ret
1405a7369: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1405a7370: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a7375: 55                          	push	rbp
1405a7376: 41 57                       	push	r15
1405a7378: 41 56                       	push	r14
1405a737a: 41 55                       	push	r13
1405a737c: 41 54                       	push	r12
1405a737e: 56                          	push	rsi
1405a737f: 57                          	push	rdi
1405a7380: 53                          	push	rbx
1405a7381: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a7388: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a738f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a7395: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a739e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a73a7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a73b0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a73b9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a73c2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a73cb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a73d4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a73dc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a73e4: 48 83 e2 80                 	and	rdx, -0x80
1405a73e8: 48 89 d3                    	mov	rbx, rdx
1405a73eb: 48 8b 93 58 26 00 00        	mov	rdx, qword ptr [rbx + 0x2658]
1405a73f2: 48 8d 04 55 00 00 00 00     	lea	rax, [2*rdx]
1405a73fa: 48 85 c0                    	test	rax, rax
1405a73fd: 74 12                       	je	0x1405a7411 <.text+0x5a6411>
1405a73ff: 48 8b 8b 60 26 00 00        	mov	rcx, qword ptr [rbx + 0x2660]
1405a7406: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a740c: e8 0f c8 06 00              	call	0x140613c20 <.text+0x612c20>
1405a7411: c6 83 c7 00 00 00 00        	mov	byte ptr [rbx + 0xc7], 0x0
1405a7418: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a7420: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a7428: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a7431: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a743a: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a7443: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a744c: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a7455: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a745e: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a7467: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a746d: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a7474: 5b                          	pop	rbx
1405a7475: 5f                          	pop	rdi
1405a7476: 5e                          	pop	rsi
1405a7477: 41 5c                       	pop	r12
1405a7479: 41 5d                       	pop	r13
1405a747b: 41 5e                       	pop	r14
1405a747d: 41 5f                       	pop	r15
1405a747f: 5d                          	pop	rbp
1405a7480: c3                          	ret
1405a7481: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1405a7490: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a7495: 55                          	push	rbp
1405a7496: 41 57                       	push	r15
1405a7498: 41 56                       	push	r14
1405a749a: 41 55                       	push	r13
1405a749c: 41 54                       	push	r12
1405a749e: 56                          	push	rsi
1405a749f: 57                          	push	rdi
1405a74a0: 53                          	push	rbx
1405a74a1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a74a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a74af: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a74b5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a74be: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a74c7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a74d0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a74d9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a74e2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a74eb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a74f4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a74fc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a7504: 48 83 e2 80                 	and	rdx, -0x80
1405a7508: 48 89 d3                    	mov	rbx, rdx
1405a750b: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405a7512: e8 39 af 04 00              	call	0x1405f2450 <.text+0x5f1450>
1405a7517: c6 83 c7 00 00 00 00        	mov	byte ptr [rbx + 0xc7], 0x0
1405a751e: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a7526: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a752e: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a7537: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a7540: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a7549: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a7552: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a755b: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a7564: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a756d: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a7573: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a757a: 5b                          	pop	rbx
1405a757b: 5f                          	pop	rdi
1405a757c: 5e                          	pop	rsi
1405a757d: 41 5c                       	pop	r12
1405a757f: 41 5d                       	pop	r13
1405a7581: 41 5e                       	pop	r14
1405a7583: 41 5f                       	pop	r15
1405a7585: 5d                          	pop	rbp
1405a7586: c3                          	ret
1405a7587: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1405a7590: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a7595: 55                          	push	rbp
1405a7596: 41 57                       	push	r15
1405a7598: 41 56                       	push	r14
1405a759a: 41 55                       	push	r13
1405a759c: 41 54                       	push	r12
1405a759e: 56                          	push	rsi
1405a759f: 57                          	push	rdi
1405a75a0: 53                          	push	rbx
1405a75a1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a75a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a75af: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a75b5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a75be: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a75c7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a75d0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a75d9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a75e2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a75eb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a75f4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a75fc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a7604: 48 83 e2 80                 	and	rdx, -0x80
1405a7608: 48 89 d3                    	mov	rbx, rdx
1405a760b: 48 8b 83 38 22 00 00        	mov	rax, qword ptr [rbx + 0x2238]
1405a7612: f0                          	lock
1405a7613: 48 ff 08                    	dec	qword ptr [rax]
1405a7616: 75 13                       	jne	0x1405a762b <.text+0x5a662b>
1405a7618: 48 8d 8b 38 22 00 00        	lea	rcx, [rbx + 0x2238]
1405a761f: c6 83 db 00 00 00 00        	mov	byte ptr [rbx + 0xdb], 0x0
1405a7626: e8 85 c7 7a 00              	call	0x140d53db0 <.text+0xd52db0>
1405a762b: c6 83 db 00 00 00 00        	mov	byte ptr [rbx + 0xdb], 0x0
1405a7632: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a763a: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a7642: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a764b: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a7654: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a765d: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a7666: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a766f: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a7678: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a7681: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a7687: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a768e: 5b                          	pop	rbx
1405a768f: 5f                          	pop	rdi
1405a7690: 5e                          	pop	rsi
1405a7691: 41 5c                       	pop	r12
1405a7693: 41 5d                       	pop	r13
1405a7695: 41 5e                       	pop	r14
1405a7697: 41 5f                       	pop	r15
1405a7699: 5d                          	pop	rbp
1405a769a: c3                          	ret
1405a769b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1405a76a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a76a5: 55                          	push	rbp
1405a76a6: 41 57                       	push	r15
1405a76a8: 41 56                       	push	r14
1405a76aa: 41 55                       	push	r13
1405a76ac: 41 54                       	push	r12
1405a76ae: 56                          	push	rsi
1405a76af: 57                          	push	rdi
1405a76b0: 53                          	push	rbx
1405a76b1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a76b8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a76bf: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a76c5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a76ce: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a76d7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a76e0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a76e9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a76f2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a76fb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a7704: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a770c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a7714: 48 83 e2 80                 	and	rdx, -0x80
1405a7718: 48 89 d3                    	mov	rbx, rdx
1405a771b: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a7722: e8 09 55 7a 00              	call	0x140d4cc30 <.text+0xd4bc30>
1405a7727: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a772f: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a7737: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a7740: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a7749: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a7752: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a775b: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a7764: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a776d: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a7776: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a777c: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a7783: 5b                          	pop	rbx
1405a7784: 5f                          	pop	rdi
1405a7785: 5e                          	pop	rsi
1405a7786: 41 5c                       	pop	r12
1405a7788: 41 5d                       	pop	r13
1405a778a: 41 5e                       	pop	r14
1405a778c: 41 5f                       	pop	r15
1405a778e: 5d                          	pop	rbp
1405a778f: c3                          	ret
1405a7790: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a7795: 55                          	push	rbp
1405a7796: 41 57                       	push	r15
1405a7798: 41 56                       	push	r14
1405a779a: 41 55                       	push	r13
1405a779c: 41 54                       	push	r12
1405a779e: 56                          	push	rsi
1405a779f: 57                          	push	rdi
1405a77a0: 53                          	push	rbx
1405a77a1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a77a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a77af: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a77b5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a77be: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a77c7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a77d0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a77d9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a77e2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a77eb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a77f4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a77fc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a7804: 48 83 e2 80                 	and	rdx, -0x80
1405a7808: 48 89 d3                    	mov	rbx, rdx
1405a780b: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405a7812: e8 19 54 7a 00              	call	0x140d4cc30 <.text+0xd4bc30>
1405a7817: c6 83 c7 00 00 00 01        	mov	byte ptr [rbx + 0xc7], 0x1
1405a781e: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a7826: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a782e: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a7837: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a7840: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a7849: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a7852: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a785b: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a7864: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a786d: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a7873: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a787a: 5b                          	pop	rbx
1405a787b: 5f                          	pop	rdi
1405a787c: 5e                          	pop	rsi
1405a787d: 41 5c                       	pop	r12
1405a787f: 41 5d                       	pop	r13
1405a7881: 41 5e                       	pop	r14
1405a7883: 41 5f                       	pop	r15
1405a7885: 5d                          	pop	rbp
1405a7886: c3                          	ret
1405a7887: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1405a7890: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a7895: 55                          	push	rbp
1405a7896: 41 57                       	push	r15
1405a7898: 41 56                       	push	r14
1405a789a: 41 55                       	push	r13
1405a789c: 41 54                       	push	r12
1405a789e: 56                          	push	rsi
1405a789f: 57                          	push	rdi
1405a78a0: 53                          	push	rbx
1405a78a1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a78a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a78af: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a78b5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a78be: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a78c7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a78d0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a78d9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a78e2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a78eb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a78f4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a78fc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a7904: 48 83 e2 80                 	and	rdx, -0x80
1405a7908: 48 89 d3                    	mov	rbx, rdx
1405a790b: 48 83 bb 30 01 00 00 00     	cmp	qword ptr [rbx + 0x130], 0x0
1405a7913: 74 19                       	je	0x1405a792e <.text+0x5a692e>
1405a7915: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a791b: 48 8b 8b 28 01 00 00        	mov	rcx, qword ptr [rbx + 0x128]
1405a7922: 48 8b 93 30 01 00 00        	mov	rdx, qword ptr [rbx + 0x130]
1405a7929: e8 f2 c2 06 00              	call	0x140613c20 <.text+0x612c20>
1405a792e: c6 83 c7 00 00 00 01        	mov	byte ptr [rbx + 0xc7], 0x1
1405a7935: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a793d: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a7945: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a794e: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a7957: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a7960: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a7969: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a7972: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a797b: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a7984: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a798a: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a7991: 5b                          	pop	rbx
1405a7992: 5f                          	pop	rdi
1405a7993: 5e                          	pop	rsi
1405a7994: 41 5c                       	pop	r12
1405a7996: 41 5d                       	pop	r13
1405a7998: 41 5e                       	pop	r14
1405a799a: 41 5f                       	pop	r15
1405a799c: 5d                          	pop	rbp
1405a799d: c3                          	ret
1405a799e: 66 90                       	nop
1405a79a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a79a5: 55                          	push	rbp
1405a79a6: 41 57                       	push	r15
1405a79a8: 41 56                       	push	r14
1405a79aa: 41 55                       	push	r13
1405a79ac: 41 54                       	push	r12
1405a79ae: 56                          	push	rsi
1405a79af: 57                          	push	rdi
1405a79b0: 53                          	push	rbx
1405a79b1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a79b8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a79bf: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a79c5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a79ce: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a79d7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a79e0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a79e9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a79f2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a79fb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a7a04: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a7a0c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a7a14: 48 83 e2 80                 	and	rdx, -0x80
1405a7a18: 48 89 d3                    	mov	rbx, rdx
1405a7a1b: 48 8d 8b f0 1f 00 00        	lea	rcx, [rbx + 0x1ff0]
1405a7a22: e8 49 b8 04 00              	call	0x1405f3270 <.text+0x5f2270>
1405a7a27: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a7a2f: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a7a37: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a7a40: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a7a49: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a7a52: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a7a5b: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a7a64: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a7a6d: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a7a76: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a7a7c: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a7a83: 5b                          	pop	rbx
1405a7a84: 5f                          	pop	rdi
1405a7a85: 5e                          	pop	rsi
1405a7a86: 41 5c                       	pop	r12
1405a7a88: 41 5d                       	pop	r13
1405a7a8a: 41 5e                       	pop	r14
1405a7a8c: 41 5f                       	pop	r15
1405a7a8e: 5d                          	pop	rbp
1405a7a8f: c3                          	ret
1405a7a90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a7a95: 55                          	push	rbp
1405a7a96: 41 57                       	push	r15
1405a7a98: 41 56                       	push	r14
1405a7a9a: 41 55                       	push	r13
1405a7a9c: 41 54                       	push	r12
1405a7a9e: 56                          	push	rsi
1405a7a9f: 57                          	push	rdi
1405a7aa0: 53                          	push	rbx
1405a7aa1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a7aa8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a7aaf: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a7ab5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a7abe: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a7ac7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a7ad0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a7ad9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a7ae2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a7aeb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a7af4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a7afc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a7b04: 48 83 e2 80                 	and	rdx, -0x80
1405a7b08: 48 89 d3                    	mov	rbx, rdx
1405a7b0b: 48 8b 85 b0 37 00 00        	mov	rax, qword ptr [rbp + 0x37b0]
1405a7b12: 48 8b 90 c8 00 00 00        	mov	rdx, qword ptr [rax + 0xc8]
1405a7b19: 48 8d 04 55 00 00 00 00     	lea	rax, [2*rdx]
1405a7b21: 48 85 c0                    	test	rax, rax
1405a7b24: 74 19                       	je	0x1405a7b3f <.text+0x5a6b3f>
1405a7b26: 48 8b 85 b0 37 00 00        	mov	rax, qword ptr [rbp + 0x37b0]
1405a7b2d: 48 8b 88 d0 00 00 00        	mov	rcx, qword ptr [rax + 0xd0]
1405a7b34: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a7b3a: e8 e1 c0 06 00              	call	0x140613c20 <.text+0x612c20>
1405a7b3f: 8b 83 e8 00 00 00           	mov	eax, dword ptr [rbx + 0xe8]
1405a7b45: 88 83 1a 01 00 00           	mov	byte ptr [rbx + 0x11a], al
1405a7b4b: 8b 83 28 01 00 00           	mov	eax, dword ptr [rbx + 0x128]
1405a7b51: 88 83 1b 01 00 00           	mov	byte ptr [rbx + 0x11b], al
1405a7b57: c6 83 1c 01 00 00 00        	mov	byte ptr [rbx + 0x11c], 0x0
1405a7b5e: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a7b66: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a7b6e: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a7b77: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a7b80: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a7b89: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a7b92: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a7b9b: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a7ba4: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a7bad: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a7bb3: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a7bba: 5b                          	pop	rbx
1405a7bbb: 5f                          	pop	rdi
1405a7bbc: 5e                          	pop	rsi
1405a7bbd: 41 5c                       	pop	r12
1405a7bbf: 41 5d                       	pop	r13
1405a7bc1: 41 5e                       	pop	r14
1405a7bc3: 41 5f                       	pop	r15
1405a7bc5: 5d                          	pop	rbp
1405a7bc6: c3                          	ret
1405a7bc7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1405a7bd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a7bd5: 55                          	push	rbp
1405a7bd6: 41 57                       	push	r15
1405a7bd8: 41 56                       	push	r14
1405a7bda: 41 55                       	push	r13
1405a7bdc: 41 54                       	push	r12
1405a7bde: 56                          	push	rsi
1405a7bdf: 57                          	push	rdi
1405a7be0: 53                          	push	rbx
1405a7be1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a7be8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a7bef: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a7bf5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a7bfe: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a7c07: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a7c10: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a7c19: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a7c22: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a7c2b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a7c34: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a7c3c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a7c44: 48 83 e2 80                 	and	rdx, -0x80
1405a7c48: 48 89 d3                    	mov	rbx, rdx
1405a7c4b: 48 8b 83 98 02 00 00        	mov	rax, qword ptr [rbx + 0x298]
1405a7c52: f0                          	lock
1405a7c53: 48 ff 08                    	dec	qword ptr [rax]
1405a7c56: 75 39                       	jne	0x1405a7c91 <.text+0x5a6c91>
1405a7c58: 48 8d 8b 98 02 00 00        	lea	rcx, [rbx + 0x298]
1405a7c5f: c6 83 d4 00 00 00 00        	mov	byte ptr [rbx + 0xd4], 0x0
1405a7c66: 8b 83 28 01 00 00           	mov	eax, dword ptr [rbx + 0x128]
1405a7c6c: 88 83 d5 00 00 00           	mov	byte ptr [rbx + 0xd5], al
1405a7c72: 8b 83 e8 00 00 00           	mov	eax, dword ptr [rbx + 0xe8]
1405a7c78: 88 83 d6 00 00 00           	mov	byte ptr [rbx + 0xd6], al
1405a7c7e: c6 83 d7 00 00 00 00        	mov	byte ptr [rbx + 0xd7], 0x0
1405a7c85: c6 83 d8 00 00 00 00        	mov	byte ptr [rbx + 0xd8], 0x0
1405a7c8c: e8 1f c1 7a 00              	call	0x140d53db0 <.text+0xd52db0>
1405a7c91: c6 83 d4 00 00 00 00        	mov	byte ptr [rbx + 0xd4], 0x0
1405a7c98: 8b 83 28 01 00 00           	mov	eax, dword ptr [rbx + 0x128]
1405a7c9e: 88 83 d5 00 00 00           	mov	byte ptr [rbx + 0xd5], al
1405a7ca4: 8b 83 e8 00 00 00           	mov	eax, dword ptr [rbx + 0xe8]
1405a7caa: 88 83 d6 00 00 00           	mov	byte ptr [rbx + 0xd6], al
1405a7cb0: c6 83 d7 00 00 00 00        	mov	byte ptr [rbx + 0xd7], 0x0
1405a7cb7: c6 83 d8 00 00 00 00        	mov	byte ptr [rbx + 0xd8], 0x0
1405a7cbe: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a7cc6: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a7cce: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a7cd7: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a7ce0: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a7ce9: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a7cf2: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a7cfb: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a7d04: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a7d0d: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a7d13: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a7d1a: 5b                          	pop	rbx
1405a7d1b: 5f                          	pop	rdi
1405a7d1c: 5e                          	pop	rsi
1405a7d1d: 41 5c                       	pop	r12
1405a7d1f: 41 5d                       	pop	r13
1405a7d21: 41 5e                       	pop	r14
1405a7d23: 41 5f                       	pop	r15
1405a7d25: 5d                          	pop	rbp
1405a7d26: c3                          	ret
1405a7d27: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1405a7d30: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a7d35: 55                          	push	rbp
1405a7d36: 41 57                       	push	r15
1405a7d38: 41 56                       	push	r14
1405a7d3a: 41 55                       	push	r13
1405a7d3c: 41 54                       	push	r12
1405a7d3e: 56                          	push	rsi
1405a7d3f: 57                          	push	rdi
1405a7d40: 53                          	push	rbx
1405a7d41: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a7d48: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a7d4f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a7d55: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a7d5e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a7d67: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a7d70: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a7d79: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a7d82: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a7d8b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a7d94: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a7d9c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a7da4: 48 83 e2 80                 	and	rdx, -0x80
1405a7da8: 48 89 d3                    	mov	rbx, rdx
1405a7dab: 48 8d 8b f0 2b 00 00        	lea	rcx, [rbx + 0x2bf0]
1405a7db2: e8 b9 b4 04 00              	call	0x1405f3270 <.text+0x5f2270>
1405a7db7: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a7dbf: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a7dc7: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a7dd0: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a7dd9: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a7de2: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a7deb: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a7df4: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a7dfd: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a7e06: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a7e0c: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a7e13: 5b                          	pop	rbx
1405a7e14: 5f                          	pop	rdi
1405a7e15: 5e                          	pop	rsi
1405a7e16: 41 5c                       	pop	r12
1405a7e18: 41 5d                       	pop	r13
1405a7e1a: 41 5e                       	pop	r14
1405a7e1c: 41 5f                       	pop	r15
1405a7e1e: 5d                          	pop	rbp
1405a7e1f: c3                          	ret
1405a7e20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a7e25: 55                          	push	rbp
1405a7e26: 41 57                       	push	r15
1405a7e28: 41 56                       	push	r14
1405a7e2a: 41 55                       	push	r13
1405a7e2c: 41 54                       	push	r12
1405a7e2e: 56                          	push	rsi
1405a7e2f: 57                          	push	rdi
1405a7e30: 53                          	push	rbx
1405a7e31: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a7e38: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a7e3f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a7e45: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a7e4e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a7e57: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a7e60: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a7e69: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a7e72: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a7e7b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a7e84: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a7e8c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a7e94: 48 83 e2 80                 	and	rdx, -0x80
1405a7e98: 48 89 d3                    	mov	rbx, rdx
1405a7e9b: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a7ea2: e8 59 fb 03 00              	call	0x1405e7a00 <.text+0x5e6a00>
1405a7ea7: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a7eaf: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a7eb7: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a7ec0: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a7ec9: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a7ed2: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a7edb: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a7ee4: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a7eed: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a7ef6: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a7efc: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a7f03: 5b                          	pop	rbx
1405a7f04: 5f                          	pop	rdi
1405a7f05: 5e                          	pop	rsi
1405a7f06: 41 5c                       	pop	r12
1405a7f08: 41 5d                       	pop	r13
1405a7f0a: 41 5e                       	pop	r14
1405a7f0c: 41 5f                       	pop	r15
1405a7f0e: 5d                          	pop	rbp
1405a7f0f: c3                          	ret
1405a7f10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a7f15: 55                          	push	rbp
1405a7f16: 41 57                       	push	r15
1405a7f18: 41 56                       	push	r14
1405a7f1a: 41 55                       	push	r13
1405a7f1c: 41 54                       	push	r12
1405a7f1e: 56                          	push	rsi
1405a7f1f: 57                          	push	rdi
1405a7f20: 53                          	push	rbx
1405a7f21: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a7f28: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a7f2f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a7f35: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a7f3e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a7f47: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a7f50: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a7f59: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a7f62: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a7f6b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a7f74: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a7f7c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a7f84: 48 83 e2 80                 	and	rdx, -0x80
1405a7f88: 48 89 d3                    	mov	rbx, rdx
1405a7f8b: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a7f92: e8 d9 e7 04 00              	call	0x1405f6770 <.text+0x5f5770>
1405a7f97: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a7f9e: 8b 83 30 01 00 00           	mov	eax, dword ptr [rbx + 0x130]
1405a7fa4: 88 83 b5 00 00 00           	mov	byte ptr [rbx + 0xb5], al
1405a7faa: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a7fb1: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a7fb8: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a7fbf: c6 83 b9 00 00 00 00        	mov	byte ptr [rbx + 0xb9], 0x0
1405a7fc6: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a7fcd: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a7fd4: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a7fdc: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a7fe4: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a7fed: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a7ff6: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a7fff: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a8008: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a8011: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a801a: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a8023: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a8029: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a8030: 5b                          	pop	rbx
1405a8031: 5f                          	pop	rdi
1405a8032: 5e                          	pop	rsi
1405a8033: 41 5c                       	pop	r12
1405a8035: 41 5d                       	pop	r13
1405a8037: 41 5e                       	pop	r14
1405a8039: 41 5f                       	pop	r15
1405a803b: 5d                          	pop	rbp
1405a803c: c3                          	ret
1405a803d: 0f 1f 00                    	nop	dword ptr [rax]
1405a8040: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a8045: 55                          	push	rbp
1405a8046: 41 57                       	push	r15
1405a8048: 41 56                       	push	r14
1405a804a: 41 55                       	push	r13
1405a804c: 41 54                       	push	r12
1405a804e: 56                          	push	rsi
1405a804f: 57                          	push	rdi
1405a8050: 53                          	push	rbx
1405a8051: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a8058: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a805f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a8065: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a806e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a8077: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a8080: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a8089: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a8092: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a809b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a80a4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a80ac: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a80b4: 48 83 e2 80                 	and	rdx, -0x80
1405a80b8: 48 89 d3                    	mov	rbx, rdx
1405a80bb: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a80c2: e8 39 f9 03 00              	call	0x1405e7a00 <.text+0x5e6a00>
1405a80c7: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a80cf: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a80d7: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a80e0: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a80e9: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a80f2: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a80fb: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a8104: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a810d: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a8116: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a811c: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a8123: 5b                          	pop	rbx
1405a8124: 5f                          	pop	rdi
1405a8125: 5e                          	pop	rsi
1405a8126: 41 5c                       	pop	r12
1405a8128: 41 5d                       	pop	r13
1405a812a: 41 5e                       	pop	r14
1405a812c: 41 5f                       	pop	r15
1405a812e: 5d                          	pop	rbp
1405a812f: c3                          	ret
1405a8130: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a8135: 55                          	push	rbp
1405a8136: 41 57                       	push	r15
1405a8138: 41 56                       	push	r14
1405a813a: 41 55                       	push	r13
1405a813c: 41 54                       	push	r12
1405a813e: 56                          	push	rsi
1405a813f: 57                          	push	rdi
1405a8140: 53                          	push	rbx
1405a8141: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a8148: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a814f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a8155: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a815e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a8167: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a8170: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a8179: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a8182: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a818b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a8194: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a819c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a81a4: 48 83 e2 80                 	and	rdx, -0x80
1405a81a8: 48 89 d3                    	mov	rbx, rdx
1405a81ab: 48 8b 93 90 25 00 00        	mov	rdx, qword ptr [rbx + 0x2590]
1405a81b2: 48 85 d2                    	test	rdx, rdx
1405a81b5: 74 12                       	je	0x1405a81c9 <.text+0x5a71c9>
1405a81b7: 48 8b 8b 98 25 00 00        	mov	rcx, qword ptr [rbx + 0x2598]
1405a81be: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a81c4: e8 57 ba 06 00              	call	0x140613c20 <.text+0x612c20>
1405a81c9: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a81d0: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a81d7: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a81de: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a81e5: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a81ec: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a81f3: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a81fa: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a8201: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a8209: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a8211: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a821a: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a8223: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a822c: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a8235: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a823e: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a8247: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a8250: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a8256: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a825d: 5b                          	pop	rbx
1405a825e: 5f                          	pop	rdi
1405a825f: 5e                          	pop	rsi
1405a8260: 41 5c                       	pop	r12
1405a8262: 41 5d                       	pop	r13
1405a8264: 41 5e                       	pop	r14
1405a8266: 41 5f                       	pop	r15
1405a8268: 5d                          	pop	rbp
1405a8269: c3                          	ret
1405a826a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1405a8270: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a8275: 55                          	push	rbp
1405a8276: 41 57                       	push	r15
1405a8278: 41 56                       	push	r14
1405a827a: 41 55                       	push	r13
1405a827c: 41 54                       	push	r12
1405a827e: 56                          	push	rsi
1405a827f: 57                          	push	rdi
1405a8280: 53                          	push	rbx
1405a8281: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a8288: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a828f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a8295: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a829e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a82a7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a82b0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a82b9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a82c2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a82cb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a82d4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a82dc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a82e4: 48 83 e2 80                 	and	rdx, -0x80
1405a82e8: 48 89 d3                    	mov	rbx, rdx
1405a82eb: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a82f2: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a82f9: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a8300: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a8307: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a830e: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a8315: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a831c: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a8323: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a832a: e8 d1 f6 03 00              	call	0x1405e7a00 <.text+0x5e6a00>
1405a832f: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a8336: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a833d: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a8344: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a834b: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a8352: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a8359: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a8360: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a8367: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a836f: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a8377: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a8380: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a8389: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a8392: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a839b: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a83a4: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a83ad: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a83b6: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a83bc: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a83c3: 5b                          	pop	rbx
1405a83c4: 5f                          	pop	rdi
1405a83c5: 5e                          	pop	rsi
1405a83c6: 41 5c                       	pop	r12
1405a83c8: 41 5d                       	pop	r13
1405a83ca: 41 5e                       	pop	r14
1405a83cc: 41 5f                       	pop	r15
1405a83ce: 5d                          	pop	rbp
1405a83cf: c3                          	ret
1405a83d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a83d5: 55                          	push	rbp
1405a83d6: 41 57                       	push	r15
1405a83d8: 41 56                       	push	r14
1405a83da: 41 55                       	push	r13
1405a83dc: 41 54                       	push	r12
1405a83de: 56                          	push	rsi
1405a83df: 57                          	push	rdi
1405a83e0: 53                          	push	rbx
1405a83e1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a83e8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a83ef: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a83f5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a83fe: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a8407: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a8410: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a8419: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a8422: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a842b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a8434: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a843c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a8444: 48 83 e2 80                 	and	rdx, -0x80
1405a8448: 48 89 d3                    	mov	rbx, rdx
1405a844b: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a8452: 41 b8 20 00 00 00           	mov	r8d, 0x20
1405a8458: 41 b9 10 00 00 00           	mov	r9d, 0x10
1405a845e: 48 8d 93 60 04 00 00        	lea	rdx, [rbx + 0x460]
1405a8465: e8 36 9b aa ff              	call	0x140051fa0 <.text+0x50fa0>
1405a846a: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a8471: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a8478: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a847f: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a8486: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a848d: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a8494: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a849b: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a84a2: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a84a9: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a84b1: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a84b9: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a84c2: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a84cb: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a84d4: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a84dd: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a84e6: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a84ef: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a84f8: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a84fe: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a8505: 5b                          	pop	rbx
1405a8506: 5f                          	pop	rdi
1405a8507: 5e                          	pop	rsi
1405a8508: 41 5c                       	pop	r12
1405a850a: 41 5d                       	pop	r13
1405a850c: 41 5e                       	pop	r14
1405a850e: 41 5f                       	pop	r15
1405a8510: 5d                          	pop	rbp
1405a8511: c3                          	ret
1405a8512: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1405a8520: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a8525: 55                          	push	rbp
1405a8526: 41 57                       	push	r15
1405a8528: 41 56                       	push	r14
1405a852a: 41 55                       	push	r13
1405a852c: 41 54                       	push	r12
1405a852e: 56                          	push	rsi
1405a852f: 57                          	push	rdi
1405a8530: 53                          	push	rbx
1405a8531: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a8538: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a853f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a8545: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a854e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a8557: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a8560: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a8569: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a8572: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a857b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a8584: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a858c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a8594: 48 83 e2 80                 	and	rdx, -0x80
1405a8598: 48 89 d3                    	mov	rbx, rdx
1405a859b: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a85a2: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a85a9: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a85b0: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a85b7: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a85be: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a85c5: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a85cc: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a85d3: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a85da: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a85e1: e8 0a a7 02 00              	call	0x1405d2cf0 <.text+0x5d1cf0>
1405a85e6: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a85ed: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a85f4: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a85fb: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a8602: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a8609: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a8610: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a8617: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a861e: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a8625: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a862d: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a8635: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a863e: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a8647: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a8650: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a8659: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a8662: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a866b: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a8674: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a867a: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a8681: 5b                          	pop	rbx
1405a8682: 5f                          	pop	rdi
1405a8683: 5e                          	pop	rsi
1405a8684: 41 5c                       	pop	r12
1405a8686: 41 5d                       	pop	r13
1405a8688: 41 5e                       	pop	r14
1405a868a: 41 5f                       	pop	r15
1405a868c: 5d                          	pop	rbp
1405a868d: c3                          	ret
1405a868e: 66 90                       	nop
1405a8690: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a8695: 55                          	push	rbp
1405a8696: 41 57                       	push	r15
1405a8698: 41 56                       	push	r14
1405a869a: 41 55                       	push	r13
1405a869c: 41 54                       	push	r12
1405a869e: 56                          	push	rsi
1405a869f: 57                          	push	rdi
1405a86a0: 53                          	push	rbx
1405a86a1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a86a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a86af: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a86b5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a86be: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a86c7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a86d0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a86d9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a86e2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a86eb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a86f4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a86fc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a8704: 48 83 e2 80                 	and	rdx, -0x80
1405a8708: 48 89 d3                    	mov	rbx, rdx
1405a870b: c6 83 bc 00 00 00 01        	mov	byte ptr [rbx + 0xbc], 0x1
1405a8712: c6 83 bd 00 00 00 01        	mov	byte ptr [rbx + 0xbd], 0x1
1405a8719: c6 83 be 00 00 00 01        	mov	byte ptr [rbx + 0xbe], 0x1
1405a8720: c6 83 bf 00 00 00 01        	mov	byte ptr [rbx + 0xbf], 0x1
1405a8727: c6 83 c0 00 00 00 01        	mov	byte ptr [rbx + 0xc0], 0x1
1405a872e: c6 83 c1 00 00 00 01        	mov	byte ptr [rbx + 0xc1], 0x1
1405a8735: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a873c: 41 b8 38 00 00 00           	mov	r8d, 0x38
1405a8742: 41 b9 10 00 00 00           	mov	r9d, 0x10
1405a8748: 48 8d 93 20 0b 00 00        	lea	rdx, [rbx + 0xb20]
1405a874f: e8 0c a9 aa ff              	call	0x140053060 <.text+0x52060>
1405a8754: c6 83 bc 00 00 00 01        	mov	byte ptr [rbx + 0xbc], 0x1
1405a875b: c6 83 bd 00 00 00 01        	mov	byte ptr [rbx + 0xbd], 0x1
1405a8762: c6 83 be 00 00 00 01        	mov	byte ptr [rbx + 0xbe], 0x1
1405a8769: c6 83 bf 00 00 00 01        	mov	byte ptr [rbx + 0xbf], 0x1
1405a8770: c6 83 c0 00 00 00 01        	mov	byte ptr [rbx + 0xc0], 0x1
1405a8777: c6 83 c1 00 00 00 01        	mov	byte ptr [rbx + 0xc1], 0x1
1405a877e: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a8786: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a878e: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a8797: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a87a0: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a87a9: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a87b2: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a87bb: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a87c4: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a87cd: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a87d3: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a87da: 5b                          	pop	rbx
1405a87db: 5f                          	pop	rdi
1405a87dc: 5e                          	pop	rsi
1405a87dd: 41 5c                       	pop	r12
1405a87df: 41 5d                       	pop	r13
1405a87e1: 41 5e                       	pop	r14
1405a87e3: 41 5f                       	pop	r15
1405a87e5: 5d                          	pop	rbp
1405a87e6: c3                          	ret
1405a87e7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1405a87f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a87f5: 55                          	push	rbp
1405a87f6: 41 57                       	push	r15
1405a87f8: 41 56                       	push	r14
1405a87fa: 41 55                       	push	r13
1405a87fc: 41 54                       	push	r12
1405a87fe: 56                          	push	rsi
1405a87ff: 57                          	push	rdi
1405a8800: 53                          	push	rbx
1405a8801: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a8808: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a880f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a8815: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a881e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a8827: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a8830: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a8839: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a8842: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a884b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a8854: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a885c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a8864: 48 83 e2 80                 	and	rdx, -0x80
1405a8868: 48 89 d3                    	mov	rbx, rdx
1405a886b: 48 83 bb 30 01 00 00 00     	cmp	qword ptr [rbx + 0x130], 0x0
1405a8873: 74 19                       	je	0x1405a888e <.text+0x5a788e>
1405a8875: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a887b: 48 8b 8b 28 01 00 00        	mov	rcx, qword ptr [rbx + 0x128]
1405a8882: 48 8b 93 30 01 00 00        	mov	rdx, qword ptr [rbx + 0x130]
1405a8889: e8 92 b3 06 00              	call	0x140613c20 <.text+0x612c20>
1405a888e: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a8895: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a889c: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a88a3: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a88aa: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a88b1: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a88b8: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a88bf: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a88c6: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a88ce: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a88d6: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a88df: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a88e8: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a88f1: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a88fa: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a8903: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a890c: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a8915: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a891b: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a8922: 5b                          	pop	rbx
1405a8923: 5f                          	pop	rdi
1405a8924: 5e                          	pop	rsi
1405a8925: 41 5c                       	pop	r12
1405a8927: 41 5d                       	pop	r13
1405a8929: 41 5e                       	pop	r14
1405a892b: 41 5f                       	pop	r15
1405a892d: 5d                          	pop	rbp
1405a892e: c3                          	ret
1405a892f: 90                          	nop
1405a8930: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a8935: 55                          	push	rbp
1405a8936: 41 57                       	push	r15
1405a8938: 41 56                       	push	r14
1405a893a: 41 55                       	push	r13
1405a893c: 41 54                       	push	r12
1405a893e: 56                          	push	rsi
1405a893f: 57                          	push	rdi
1405a8940: 53                          	push	rbx
1405a8941: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a8948: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a894f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a8955: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a895e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a8967: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a8970: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a8979: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a8982: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a898b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a8994: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a899c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a89a4: 48 83 e2 80                 	and	rdx, -0x80
1405a89a8: 48 89 d3                    	mov	rbx, rdx
1405a89ab: c6 83 05 01 00 00 01        	mov	byte ptr [rbx + 0x105], 0x1
1405a89b2: 48 8d 8b 70 03 00 00        	lea	rcx, [rbx + 0x370]
1405a89b9: e8 e2 a4 04 00              	call	0x1405f2ea0 <.text+0x5f1ea0>
1405a89be: c6 83 05 01 00 00 01        	mov	byte ptr [rbx + 0x105], 0x1
1405a89c5: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a89cd: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a89d5: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a89de: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a89e7: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a89f0: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a89f9: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a8a02: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a8a0b: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a8a14: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a8a1a: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a8a21: 5b                          	pop	rbx
1405a8a22: 5f                          	pop	rdi
1405a8a23: 5e                          	pop	rsi
1405a8a24: 41 5c                       	pop	r12
1405a8a26: 41 5d                       	pop	r13
1405a8a28: 41 5e                       	pop	r14
1405a8a2a: 41 5f                       	pop	r15
1405a8a2c: 5d                          	pop	rbp
1405a8a2d: c3                          	ret
1405a8a2e: 66 90                       	nop
1405a8a30: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a8a35: 55                          	push	rbp
1405a8a36: 41 57                       	push	r15
1405a8a38: 41 56                       	push	r14
1405a8a3a: 41 55                       	push	r13
1405a8a3c: 41 54                       	push	r12
1405a8a3e: 56                          	push	rsi
1405a8a3f: 57                          	push	rdi
1405a8a40: 53                          	push	rbx
1405a8a41: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a8a48: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a8a4f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a8a55: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a8a5e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a8a67: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a8a70: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a8a79: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a8a82: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a8a8b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a8a94: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a8a9c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a8aa4: 48 83 e2 80                 	and	rdx, -0x80
1405a8aa8: 48 89 d3                    	mov	rbx, rdx
1405a8aab: 48 83 bb e8 00 00 00 00     	cmp	qword ptr [rbx + 0xe8], 0x0
1405a8ab3: 0f 85 96 00 00 00           	jne	0x1405a8b4f <.text+0x5a7b4f>
1405a8ab9: 48 8b 93 10 0b 00 00        	mov	rdx, qword ptr [rbx + 0xb10]
1405a8ac0: 48 8d 04 55 00 00 00 00     	lea	rax, [2*rdx]
1405a8ac8: 48 85 c0                    	test	rax, rax
1405a8acb: 74 12                       	je	0x1405a8adf <.text+0x5a7adf>
1405a8acd: 48 8b 8b 18 0b 00 00        	mov	rcx, qword ptr [rbx + 0xb18]
1405a8ad4: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a8ada: e8 41 b1 06 00              	call	0x140613c20 <.text+0x612c20>
1405a8adf: c6 83 db 00 00 00 01        	mov	byte ptr [rbx + 0xdb], 0x1
1405a8ae6: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a8aee: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a8af6: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a8aff: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a8b08: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a8b11: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a8b1a: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a8b23: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a8b2c: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a8b35: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a8b3b: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a8b42: 5b                          	pop	rbx
1405a8b43: 5f                          	pop	rdi
1405a8b44: 5e                          	pop	rsi
1405a8b45: 41 5c                       	pop	r12
1405a8b47: 41 5d                       	pop	r13
1405a8b49: 41 5e                       	pop	r14
1405a8b4b: 41 5f                       	pop	r15
1405a8b4d: 5d                          	pop	rbp
1405a8b4e: c3                          	ret
1405a8b4f: c6 83 db 00 00 00 01        	mov	byte ptr [rbx + 0xdb], 0x1
1405a8b56: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a8b5e: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a8b66: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a8b6f: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a8b78: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a8b81: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a8b8a: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a8b93: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a8b9c: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a8ba5: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a8bab: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a8bb2: 5b                          	pop	rbx
1405a8bb3: 5f                          	pop	rdi
1405a8bb4: 5e                          	pop	rsi
1405a8bb5: 41 5c                       	pop	r12
1405a8bb7: 41 5d                       	pop	r13
1405a8bb9: 41 5e                       	pop	r14
1405a8bbb: 41 5f                       	pop	r15
1405a8bbd: 5d                          	pop	rbp
1405a8bbe: c3                          	ret
1405a8bbf: 90                          	nop
1405a8bc0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a8bc5: 55                          	push	rbp
1405a8bc6: 41 57                       	push	r15
1405a8bc8: 41 56                       	push	r14
1405a8bca: 41 55                       	push	r13
1405a8bcc: 41 54                       	push	r12
1405a8bce: 56                          	push	rsi
1405a8bcf: 57                          	push	rdi
1405a8bd0: 53                          	push	rbx
1405a8bd1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a8bd8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a8bdf: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a8be5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a8bee: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a8bf7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a8c00: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a8c09: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a8c12: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a8c1b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a8c24: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a8c2c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a8c34: 48 83 e2 80                 	and	rdx, -0x80
1405a8c38: 48 89 d3                    	mov	rbx, rdx
1405a8c3b: c6 83 06 01 00 00 01        	mov	byte ptr [rbx + 0x106], 0x1
1405a8c42: 48 8d 8b 30 22 00 00        	lea	rcx, [rbx + 0x2230]
1405a8c49: e8 52 a2 04 00              	call	0x1405f2ea0 <.text+0x5f1ea0>
1405a8c4e: c6 83 06 01 00 00 01        	mov	byte ptr [rbx + 0x106], 0x1
1405a8c55: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a8c5d: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a8c65: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a8c6e: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a8c77: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a8c80: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a8c89: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a8c92: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a8c9b: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a8ca4: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a8caa: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a8cb1: 5b                          	pop	rbx
1405a8cb2: 5f                          	pop	rdi
1405a8cb3: 5e                          	pop	rsi
1405a8cb4: 41 5c                       	pop	r12
1405a8cb6: 41 5d                       	pop	r13
1405a8cb8: 41 5e                       	pop	r14
1405a8cba: 41 5f                       	pop	r15
1405a8cbc: 5d                          	pop	rbp
1405a8cbd: c3                          	ret
1405a8cbe: 66 90                       	nop
1405a8cc0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a8cc5: 55                          	push	rbp
1405a8cc6: 41 57                       	push	r15
1405a8cc8: 41 56                       	push	r14
1405a8cca: 41 55                       	push	r13
1405a8ccc: 41 54                       	push	r12
1405a8cce: 56                          	push	rsi
1405a8ccf: 57                          	push	rdi
1405a8cd0: 53                          	push	rbx
1405a8cd1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a8cd8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a8cdf: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a8ce5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a8cee: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a8cf7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a8d00: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a8d09: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a8d12: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a8d1b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a8d24: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a8d2c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a8d34: 48 83 e2 80                 	and	rdx, -0x80
1405a8d38: 48 89 d3                    	mov	rbx, rdx
1405a8d3b: 8b b3 30 01 00 00           	mov	esi, dword ptr [rbx + 0x130]
1405a8d41: 40 88 b3 c7 00 00 00        	mov	byte ptr [rbx + 0xc7], sil
1405a8d48: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a8d4f: e8 2c 0b a8 ff              	call	0x140029880 <.text+0x28880>
1405a8d54: 40 88 b3 c7 00 00 00        	mov	byte ptr [rbx + 0xc7], sil
1405a8d5b: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a8d63: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a8d6b: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a8d74: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a8d7d: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a8d86: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a8d8f: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a8d98: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a8da1: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a8daa: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a8db0: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a8db7: 5b                          	pop	rbx
1405a8db8: 5f                          	pop	rdi
1405a8db9: 5e                          	pop	rsi
1405a8dba: 41 5c                       	pop	r12
1405a8dbc: 41 5d                       	pop	r13
1405a8dbe: 41 5e                       	pop	r14
1405a8dc0: 41 5f                       	pop	r15
1405a8dc2: 5d                          	pop	rbp
1405a8dc3: c3                          	ret
1405a8dc4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1405a8dd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a8dd5: 55                          	push	rbp
1405a8dd6: 41 57                       	push	r15
1405a8dd8: 41 56                       	push	r14
1405a8dda: 41 55                       	push	r13
1405a8ddc: 41 54                       	push	r12
1405a8dde: 56                          	push	rsi
1405a8ddf: 57                          	push	rdi
1405a8de0: 53                          	push	rbx
1405a8de1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a8de8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a8def: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a8df5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a8dfe: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a8e07: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a8e10: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a8e19: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a8e22: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a8e2b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a8e34: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a8e3c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a8e44: 48 83 e2 80                 	and	rdx, -0x80
1405a8e48: 48 89 d3                    	mov	rbx, rdx
1405a8e4b: 48 8b 93 40 04 00 00        	mov	rdx, qword ptr [rbx + 0x440]
1405a8e52: 48 85 d2                    	test	rdx, rdx
1405a8e55: 74 12                       	je	0x1405a8e69 <.text+0x5a7e69>
1405a8e57: 48 8b 8b 48 04 00 00        	mov	rcx, qword ptr [rbx + 0x448]
1405a8e5e: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a8e64: e8 b7 ad 06 00              	call	0x140613c20 <.text+0x612c20>
1405a8e69: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a8e71: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a8e79: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a8e82: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a8e8b: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a8e94: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a8e9d: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a8ea6: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a8eaf: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a8eb8: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a8ebe: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a8ec5: 5b                          	pop	rbx
1405a8ec6: 5f                          	pop	rdi
1405a8ec7: 5e                          	pop	rsi
1405a8ec8: 41 5c                       	pop	r12
1405a8eca: 41 5d                       	pop	r13
1405a8ecc: 41 5e                       	pop	r14
1405a8ece: 41 5f                       	pop	r15
1405a8ed0: 5d                          	pop	rbp
1405a8ed1: c3                          	ret
1405a8ed2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1405a8ee0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a8ee5: 55                          	push	rbp
1405a8ee6: 41 57                       	push	r15
1405a8ee8: 41 56                       	push	r14
1405a8eea: 41 55                       	push	r13
1405a8eec: 41 54                       	push	r12
1405a8eee: 56                          	push	rsi
1405a8eef: 57                          	push	rdi
1405a8ef0: 53                          	push	rbx
1405a8ef1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a8ef8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a8eff: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a8f05: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a8f0e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a8f17: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a8f20: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a8f29: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a8f32: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a8f3b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a8f44: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a8f4c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a8f54: 48 83 e2 80                 	and	rdx, -0x80
1405a8f58: 48 89 d3                    	mov	rbx, rdx
1405a8f5b: c6 83 e4 00 00 00 01        	mov	byte ptr [rbx + 0xe4], 0x1
1405a8f62: c6 83 e5 00 00 00 01        	mov	byte ptr [rbx + 0xe5], 0x1
1405a8f69: c6 83 e6 00 00 00 01        	mov	byte ptr [rbx + 0xe6], 0x1
1405a8f70: c6 83 e7 00 00 00 01        	mov	byte ptr [rbx + 0xe7], 0x1
1405a8f77: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a8f7e: e8 bd f1 02 00              	call	0x1405d8140 <.text+0x5d7140>
1405a8f83: c6 83 e4 00 00 00 01        	mov	byte ptr [rbx + 0xe4], 0x1
1405a8f8a: c6 83 e5 00 00 00 01        	mov	byte ptr [rbx + 0xe5], 0x1
1405a8f91: c6 83 e6 00 00 00 01        	mov	byte ptr [rbx + 0xe6], 0x1
1405a8f98: c6 83 e7 00 00 00 01        	mov	byte ptr [rbx + 0xe7], 0x1
1405a8f9f: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a8fa7: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a8faf: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a8fb8: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a8fc1: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a8fca: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a8fd3: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a8fdc: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a8fe5: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a8fee: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a8ff4: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a8ffb: 5b                          	pop	rbx
1405a8ffc: 5f                          	pop	rdi
1405a8ffd: 5e                          	pop	rsi
1405a8ffe: 41 5c                       	pop	r12
1405a9000: 41 5d                       	pop	r13
1405a9002: 41 5e                       	pop	r14
1405a9004: 41 5f                       	pop	r15
1405a9006: 5d                          	pop	rbp
1405a9007: c3                          	ret
1405a9008: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1405a9010: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a9015: 55                          	push	rbp
1405a9016: 41 57                       	push	r15
1405a9018: 41 56                       	push	r14
1405a901a: 41 55                       	push	r13
1405a901c: 41 54                       	push	r12
1405a901e: 56                          	push	rsi
1405a901f: 57                          	push	rdi
1405a9020: 53                          	push	rbx
1405a9021: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a9028: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a902f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a9035: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a903e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a9047: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a9050: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a9059: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a9062: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a906b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a9074: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a907c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a9084: 48 83 e2 80                 	and	rdx, -0x80
1405a9088: 48 89 d3                    	mov	rbx, rdx
1405a908b: c6 83 d4 00 00 00 01        	mov	byte ptr [rbx + 0xd4], 0x1
1405a9092: c6 83 d5 00 00 00 01        	mov	byte ptr [rbx + 0xd5], 0x1
1405a9099: c6 83 d6 00 00 00 01        	mov	byte ptr [rbx + 0xd6], 0x1
1405a90a0: c6 83 d7 00 00 00 00        	mov	byte ptr [rbx + 0xd7], 0x0
1405a90a7: c6 83 d8 00 00 00 01        	mov	byte ptr [rbx + 0xd8], 0x1
1405a90ae: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a90b5: e8 46 e9 03 00              	call	0x1405e7a00 <.text+0x5e6a00>
1405a90ba: c6 83 d4 00 00 00 01        	mov	byte ptr [rbx + 0xd4], 0x1
1405a90c1: c6 83 d5 00 00 00 01        	mov	byte ptr [rbx + 0xd5], 0x1
1405a90c8: c6 83 d6 00 00 00 01        	mov	byte ptr [rbx + 0xd6], 0x1
1405a90cf: c6 83 d7 00 00 00 00        	mov	byte ptr [rbx + 0xd7], 0x0
1405a90d6: c6 83 d8 00 00 00 01        	mov	byte ptr [rbx + 0xd8], 0x1
1405a90dd: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a90e5: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a90ed: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a90f6: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a90ff: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a9108: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a9111: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a911a: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a9123: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a912c: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a9132: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a9139: 5b                          	pop	rbx
1405a913a: 5f                          	pop	rdi
1405a913b: 5e                          	pop	rsi
1405a913c: 41 5c                       	pop	r12
1405a913e: 41 5d                       	pop	r13
1405a9140: 41 5e                       	pop	r14
1405a9142: 41 5f                       	pop	r15
1405a9144: 5d                          	pop	rbp
1405a9145: c3                          	ret
1405a9146: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1405a9150: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a9155: 55                          	push	rbp
1405a9156: 41 57                       	push	r15
1405a9158: 41 56                       	push	r14
1405a915a: 41 55                       	push	r13
1405a915c: 41 54                       	push	r12
1405a915e: 56                          	push	rsi
1405a915f: 57                          	push	rdi
1405a9160: 53                          	push	rbx
1405a9161: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a9168: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a916f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a9175: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a917e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a9187: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a9190: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a9199: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a91a2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a91ab: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a91b4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a91bc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a91c4: 48 83 e2 80                 	and	rdx, -0x80
1405a91c8: 48 89 d3                    	mov	rbx, rdx
1405a91cb: 48 8b 93 00 0b 00 00        	mov	rdx, qword ptr [rbx + 0xb00]
1405a91d2: 48 85 d2                    	test	rdx, rdx
1405a91d5: 74 12                       	je	0x1405a91e9 <.text+0x5a81e9>
1405a91d7: 48 8b 8b 08 0b 00 00        	mov	rcx, qword ptr [rbx + 0xb08]
1405a91de: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a91e4: e8 37 aa 06 00              	call	0x140613c20 <.text+0x612c20>
1405a91e9: c6 83 d3 00 00 00 01        	mov	byte ptr [rbx + 0xd3], 0x1
1405a91f0: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a91f8: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a9200: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a9209: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a9212: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a921b: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a9224: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a922d: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a9236: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a923f: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a9245: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a924c: 5b                          	pop	rbx
1405a924d: 5f                          	pop	rdi
1405a924e: 5e                          	pop	rsi
1405a924f: 41 5c                       	pop	r12
1405a9251: 41 5d                       	pop	r13
1405a9253: 41 5e                       	pop	r14
1405a9255: 41 5f                       	pop	r15
1405a9257: 5d                          	pop	rbp
1405a9258: c3                          	ret
1405a9259: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1405a9260: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a9265: 55                          	push	rbp
1405a9266: 41 57                       	push	r15
1405a9268: 41 56                       	push	r14
1405a926a: 41 55                       	push	r13
1405a926c: 41 54                       	push	r12
1405a926e: 56                          	push	rsi
1405a926f: 57                          	push	rdi
1405a9270: 53                          	push	rbx
1405a9271: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a9278: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a927f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a9285: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a928e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a9297: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a92a0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a92a9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a92b2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a92bb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a92c4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a92cc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a92d4: 48 83 e2 80                 	and	rdx, -0x80
1405a92d8: 48 89 d3                    	mov	rbx, rdx
1405a92db: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405a92e2: e8 d9 99 fe ff              	call	0x140592cc0 <.text+0x591cc0>
1405a92e7: 48 8d 8b f0 2b 00 00        	lea	rcx, [rbx + 0x2bf0]
1405a92ee: e8 cd 99 fe ff              	call	0x140592cc0 <.text+0x591cc0>
1405a92f3: c6 83 1e 01 00 00 01        	mov	byte ptr [rbx + 0x11e], 0x1
1405a92fa: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a9302: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a930a: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a9313: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a931c: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a9325: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a932e: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a9337: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a9340: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a9349: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a934f: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a9356: 5b                          	pop	rbx
1405a9357: 5f                          	pop	rdi
1405a9358: 5e                          	pop	rsi
1405a9359: 41 5c                       	pop	r12
1405a935b: 41 5d                       	pop	r13
1405a935d: 41 5e                       	pop	r14
1405a935f: 41 5f                       	pop	r15
1405a9361: 5d                          	pop	rbp
1405a9362: c3                          	ret
1405a9363: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1405a9370: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a9375: 55                          	push	rbp
1405a9376: 41 57                       	push	r15
1405a9378: 41 56                       	push	r14
1405a937a: 41 55                       	push	r13
1405a937c: 41 54                       	push	r12
1405a937e: 56                          	push	rsi
1405a937f: 57                          	push	rdi
1405a9380: 53                          	push	rbx
1405a9381: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a9388: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a938f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a9395: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a939e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a93a7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a93b0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a93b9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a93c2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a93cb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a93d4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a93dc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a93e4: 48 83 e2 80                 	and	rdx, -0x80
1405a93e8: 48 89 d3                    	mov	rbx, rdx
1405a93eb: 48 8b 93 00 0b 00 00        	mov	rdx, qword ptr [rbx + 0xb00]
1405a93f2: 48 85 d2                    	test	rdx, rdx
1405a93f5: 74 12                       	je	0x1405a9409 <.text+0x5a8409>
1405a93f7: 48 8b 8b 08 0b 00 00        	mov	rcx, qword ptr [rbx + 0xb08]
1405a93fe: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a9404: e8 17 a8 06 00              	call	0x140613c20 <.text+0x612c20>
1405a9409: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a9410: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a9417: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a941e: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a9425: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a942c: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a9433: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a943a: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a9441: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a9449: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a9451: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a945a: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a9463: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a946c: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a9475: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a947e: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a9487: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a9490: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a9496: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a949d: 5b                          	pop	rbx
1405a949e: 5f                          	pop	rdi
1405a949f: 5e                          	pop	rsi
1405a94a0: 41 5c                       	pop	r12
1405a94a2: 41 5d                       	pop	r13
1405a94a4: 41 5e                       	pop	r14
1405a94a6: 41 5f                       	pop	r15
1405a94a8: 5d                          	pop	rbp
1405a94a9: c3                          	ret
1405a94aa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1405a94b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a94b5: 55                          	push	rbp
1405a94b6: 41 57                       	push	r15
1405a94b8: 41 56                       	push	r14
1405a94ba: 41 55                       	push	r13
1405a94bc: 41 54                       	push	r12
1405a94be: 56                          	push	rsi
1405a94bf: 57                          	push	rdi
1405a94c0: 53                          	push	rbx
1405a94c1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a94c8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a94cf: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a94d5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a94de: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a94e7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a94f0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a94f9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a9502: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a950b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a9514: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a951c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a9524: 48 83 e2 80                 	and	rdx, -0x80
1405a9528: 48 89 d3                    	mov	rbx, rdx
1405a952b: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a9532: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a9539: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a9540: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a9547: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a954e: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a9555: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a955c: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a9563: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a956a: e8 c1 36 7a 00              	call	0x140d4cc30 <.text+0xd4bc30>
1405a956f: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a9576: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a957d: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a9584: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a958b: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a9592: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a9599: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a95a0: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a95a7: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a95af: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a95b7: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a95c0: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a95c9: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a95d2: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a95db: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a95e4: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a95ed: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a95f6: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a95fc: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a9603: 5b                          	pop	rbx
1405a9604: 5f                          	pop	rdi
1405a9605: 5e                          	pop	rsi
1405a9606: 41 5c                       	pop	r12
1405a9608: 41 5d                       	pop	r13
1405a960a: 41 5e                       	pop	r14
1405a960c: 41 5f                       	pop	r15
1405a960e: 5d                          	pop	rbp
1405a960f: c3                          	ret
1405a9610: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a9615: 55                          	push	rbp
1405a9616: 41 57                       	push	r15
1405a9618: 41 56                       	push	r14
1405a961a: 41 55                       	push	r13
1405a961c: 41 54                       	push	r12
1405a961e: 56                          	push	rsi
1405a961f: 57                          	push	rdi
1405a9620: 53                          	push	rbx
1405a9621: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a9628: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a962f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a9635: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a963e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a9647: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a9650: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a9659: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a9662: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a966b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a9674: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a967c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a9684: 48 83 e2 80                 	and	rdx, -0x80
1405a9688: 48 89 d3                    	mov	rbx, rdx
1405a968b: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405a9692: e8 e9 01 a8 ff              	call	0x140029880 <.text+0x28880>
1405a9697: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a969f: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a96a7: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a96b0: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a96b9: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a96c2: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a96cb: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a96d4: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a96dd: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a96e6: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a96ec: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a96f3: 5b                          	pop	rbx
1405a96f4: 5f                          	pop	rdi
1405a96f5: 5e                          	pop	rsi
1405a96f6: 41 5c                       	pop	r12
1405a96f8: 41 5d                       	pop	r13
1405a96fa: 41 5e                       	pop	r14
1405a96fc: 41 5f                       	pop	r15
1405a96fe: 5d                          	pop	rbp
1405a96ff: c3                          	ret
1405a9700: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a9705: 55                          	push	rbp
1405a9706: 41 57                       	push	r15
1405a9708: 41 56                       	push	r14
1405a970a: 41 55                       	push	r13
1405a970c: 41 54                       	push	r12
1405a970e: 56                          	push	rsi
1405a970f: 57                          	push	rdi
1405a9710: 53                          	push	rbx
1405a9711: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a9718: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a971f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a9725: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a972e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a9737: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a9740: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a9749: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a9752: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a975b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a9764: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a976c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a9774: 48 83 e2 80                 	and	rdx, -0x80
1405a9778: 48 89 d3                    	mov	rbx, rdx
1405a977b: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405a9782: e8 f9 00 a8 ff              	call	0x140029880 <.text+0x28880>
1405a9787: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a978e: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a9795: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a979c: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a97a3: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a97aa: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a97b1: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a97b8: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a97bf: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a97c6: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a97ce: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a97d6: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a97df: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a97e8: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a97f1: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a97fa: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a9803: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a980c: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a9815: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a981b: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a9822: 5b                          	pop	rbx
1405a9823: 5f                          	pop	rdi
1405a9824: 5e                          	pop	rsi
1405a9825: 41 5c                       	pop	r12
1405a9827: 41 5d                       	pop	r13
1405a9829: 41 5e                       	pop	r14
1405a982b: 41 5f                       	pop	r15
1405a982d: 5d                          	pop	rbp
1405a982e: c3                          	ret
1405a982f: 90                          	nop
1405a9830: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a9835: 55                          	push	rbp
1405a9836: 41 57                       	push	r15
1405a9838: 41 56                       	push	r14
1405a983a: 41 55                       	push	r13
1405a983c: 41 54                       	push	r12
1405a983e: 56                          	push	rsi
1405a983f: 57                          	push	rdi
1405a9840: 53                          	push	rbx
1405a9841: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a9848: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a984f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a9855: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a985e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a9867: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a9870: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a9879: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a9882: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a988b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a9894: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a989c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a98a4: 48 83 e2 80                 	and	rdx, -0x80
1405a98a8: 48 89 d3                    	mov	rbx, rdx
1405a98ab: c6 83 bc 00 00 00 01        	mov	byte ptr [rbx + 0xbc], 0x1
1405a98b2: c6 83 bd 00 00 00 01        	mov	byte ptr [rbx + 0xbd], 0x1
1405a98b9: c6 83 be 00 00 00 01        	mov	byte ptr [rbx + 0xbe], 0x1
1405a98c0: c6 83 bf 00 00 00 01        	mov	byte ptr [rbx + 0xbf], 0x1
1405a98c7: c6 83 c0 00 00 00 01        	mov	byte ptr [rbx + 0xc0], 0x1
1405a98ce: c6 83 c1 00 00 00 01        	mov	byte ptr [rbx + 0xc1], 0x1
1405a98d5: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a98dc: e8 9f f1 05 00              	call	0x140608a80 <.text+0x607a80>
1405a98e1: c6 83 bc 00 00 00 01        	mov	byte ptr [rbx + 0xbc], 0x1
1405a98e8: c6 83 bd 00 00 00 01        	mov	byte ptr [rbx + 0xbd], 0x1
1405a98ef: c6 83 be 00 00 00 01        	mov	byte ptr [rbx + 0xbe], 0x1
1405a98f6: c6 83 bf 00 00 00 01        	mov	byte ptr [rbx + 0xbf], 0x1
1405a98fd: c6 83 c0 00 00 00 01        	mov	byte ptr [rbx + 0xc0], 0x1
1405a9904: c6 83 c1 00 00 00 01        	mov	byte ptr [rbx + 0xc1], 0x1
1405a990b: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a9913: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a991b: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a9924: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a992d: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a9936: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a993f: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a9948: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a9951: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a995a: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a9960: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a9967: 5b                          	pop	rbx
1405a9968: 5f                          	pop	rdi
1405a9969: 5e                          	pop	rsi
1405a996a: 41 5c                       	pop	r12
1405a996c: 41 5d                       	pop	r13
1405a996e: 41 5e                       	pop	r14
1405a9970: 41 5f                       	pop	r15
1405a9972: 5d                          	pop	rbp
1405a9973: c3                          	ret
1405a9974: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1405a9980: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a9985: 55                          	push	rbp
1405a9986: 41 57                       	push	r15
1405a9988: 41 56                       	push	r14
1405a998a: 41 55                       	push	r13
1405a998c: 41 54                       	push	r12
1405a998e: 56                          	push	rsi
1405a998f: 57                          	push	rdi
1405a9990: 53                          	push	rbx
1405a9991: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a9998: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a999f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a99a5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a99ae: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a99b7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a99c0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a99c9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a99d2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a99db: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a99e4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a99ec: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a99f4: 48 83 e2 80                 	and	rdx, -0x80
1405a99f8: 48 89 d3                    	mov	rbx, rdx
1405a99fb: c6 83 f8 00 00 00 01        	mov	byte ptr [rbx + 0xf8], 0x1
1405a9a02: c6 83 f9 00 00 00 01        	mov	byte ptr [rbx + 0xf9], 0x1
1405a9a09: c6 83 fa 00 00 00 01        	mov	byte ptr [rbx + 0xfa], 0x1
1405a9a10: c6 83 fb 00 00 00 01        	mov	byte ptr [rbx + 0xfb], 0x1
1405a9a17: c6 83 fc 00 00 00 01        	mov	byte ptr [rbx + 0xfc], 0x1
1405a9a1e: c6 83 fd 00 00 00 01        	mov	byte ptr [rbx + 0xfd], 0x1
1405a9a25: 48 8d 8b 10 0b 00 00        	lea	rcx, [rbx + 0xb10]
1405a9a2c: e8 9f 8d 04 00              	call	0x1405f27d0 <.text+0x5f17d0>
1405a9a31: c6 83 f8 00 00 00 01        	mov	byte ptr [rbx + 0xf8], 0x1
1405a9a38: c6 83 f9 00 00 00 01        	mov	byte ptr [rbx + 0xf9], 0x1
1405a9a3f: c6 83 fa 00 00 00 01        	mov	byte ptr [rbx + 0xfa], 0x1
1405a9a46: c6 83 fb 00 00 00 01        	mov	byte ptr [rbx + 0xfb], 0x1
1405a9a4d: c6 83 fc 00 00 00 01        	mov	byte ptr [rbx + 0xfc], 0x1
1405a9a54: c6 83 fd 00 00 00 01        	mov	byte ptr [rbx + 0xfd], 0x1
1405a9a5b: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a9a63: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a9a6b: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a9a74: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a9a7d: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a9a86: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a9a8f: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a9a98: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a9aa1: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a9aaa: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a9ab0: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a9ab7: 5b                          	pop	rbx
1405a9ab8: 5f                          	pop	rdi
1405a9ab9: 5e                          	pop	rsi
1405a9aba: 41 5c                       	pop	r12
1405a9abc: 41 5d                       	pop	r13
1405a9abe: 41 5e                       	pop	r14
1405a9ac0: 41 5f                       	pop	r15
1405a9ac2: 5d                          	pop	rbp
1405a9ac3: c3                          	ret
1405a9ac4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1405a9ad0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a9ad5: 55                          	push	rbp
1405a9ad6: 41 57                       	push	r15
1405a9ad8: 41 56                       	push	r14
1405a9ada: 41 55                       	push	r13
1405a9adc: 41 54                       	push	r12
1405a9ade: 56                          	push	rsi
1405a9adf: 57                          	push	rdi
1405a9ae0: 53                          	push	rbx
1405a9ae1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a9ae8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a9aef: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a9af5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a9afe: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a9b07: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a9b10: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a9b19: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a9b22: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a9b2b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a9b34: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a9b3c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a9b44: 48 83 e2 80                 	and	rdx, -0x80
1405a9b48: 48 89 d3                    	mov	rbx, rdx
1405a9b4b: 80 bb 11 01 00 00 00        	cmp	byte ptr [rbx + 0x111], 0x0
1405a9b52: 0f 84 87 00 00 00           	je	0x1405a9bdf <.text+0x5a8bdf>
1405a9b58: 48 8b 93 90 25 00 00        	mov	rdx, qword ptr [rbx + 0x2590]
1405a9b5f: 48 85 d2                    	test	rdx, rdx
1405a9b62: 74 12                       	je	0x1405a9b76 <.text+0x5a8b76>
1405a9b64: 48 8b 8b 98 25 00 00        	mov	rcx, qword ptr [rbx + 0x2598]
1405a9b6b: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405a9b71: e8 aa a0 06 00              	call	0x140613c20 <.text+0x612c20>
1405a9b76: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a9b7e: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a9b86: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a9b8f: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a9b98: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a9ba1: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a9baa: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a9bb3: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a9bbc: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a9bc5: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a9bcb: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a9bd2: 5b                          	pop	rbx
1405a9bd3: 5f                          	pop	rdi
1405a9bd4: 5e                          	pop	rsi
1405a9bd5: 41 5c                       	pop	r12
1405a9bd7: 41 5d                       	pop	r13
1405a9bd9: 41 5e                       	pop	r14
1405a9bdb: 41 5f                       	pop	r15
1405a9bdd: 5d                          	pop	rbp
1405a9bde: c3                          	ret
1405a9bdf: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a9be7: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a9bef: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a9bf8: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a9c01: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a9c0a: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a9c13: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a9c1c: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a9c25: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a9c2e: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a9c34: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a9c3b: 5b                          	pop	rbx
1405a9c3c: 5f                          	pop	rdi
1405a9c3d: 5e                          	pop	rsi
1405a9c3e: 41 5c                       	pop	r12
1405a9c40: 41 5d                       	pop	r13
1405a9c42: 41 5e                       	pop	r14
1405a9c44: 41 5f                       	pop	r15
1405a9c46: 5d                          	pop	rbp
1405a9c47: c3                          	ret
1405a9c48: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1405a9c50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a9c55: 55                          	push	rbp
1405a9c56: 41 57                       	push	r15
1405a9c58: 41 56                       	push	r14
1405a9c5a: 41 55                       	push	r13
1405a9c5c: 41 54                       	push	r12
1405a9c5e: 56                          	push	rsi
1405a9c5f: 57                          	push	rdi
1405a9c60: 53                          	push	rbx
1405a9c61: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a9c68: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a9c6f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a9c75: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a9c7e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a9c87: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a9c90: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a9c99: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a9ca2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a9cab: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a9cb4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a9cbc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a9cc4: 48 83 e2 80                 	and	rdx, -0x80
1405a9cc8: 48 89 d3                    	mov	rbx, rdx
1405a9ccb: 80 bb 1e 01 00 00 00        	cmp	byte ptr [rbx + 0x11e], 0x0
1405a9cd2: 0f 84 ad 00 00 00           	je	0x1405a9d85 <.text+0x5a8d85>
1405a9cd8: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a9cdf: e8 8c fd 04 00              	call	0x1405f9a70 <.text+0x5f8a70>
1405a9ce4: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a9ceb: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a9cf2: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a9cf9: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a9d00: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a9d07: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a9d0e: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a9d15: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a9d1c: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a9d24: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a9d2c: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a9d35: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a9d3e: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a9d47: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a9d50: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a9d59: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a9d62: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a9d6b: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a9d71: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a9d78: 5b                          	pop	rbx
1405a9d79: 5f                          	pop	rdi
1405a9d7a: 5e                          	pop	rsi
1405a9d7b: 41 5c                       	pop	r12
1405a9d7d: 41 5d                       	pop	r13
1405a9d7f: 41 5e                       	pop	r14
1405a9d81: 41 5f                       	pop	r15
1405a9d83: 5d                          	pop	rbp
1405a9d84: c3                          	ret
1405a9d85: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405a9d8c: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405a9d93: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405a9d9a: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405a9da1: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405a9da8: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405a9daf: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405a9db6: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405a9dbd: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a9dc5: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a9dcd: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a9dd6: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a9ddf: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a9de8: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a9df1: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a9dfa: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a9e03: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a9e0c: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a9e12: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a9e19: 5b                          	pop	rbx
1405a9e1a: 5f                          	pop	rdi
1405a9e1b: 5e                          	pop	rsi
1405a9e1c: 41 5c                       	pop	r12
1405a9e1e: 41 5d                       	pop	r13
1405a9e20: 41 5e                       	pop	r14
1405a9e22: 41 5f                       	pop	r15
1405a9e24: 5d                          	pop	rbp
1405a9e25: c3                          	ret
1405a9e26: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1405a9e30: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a9e35: 55                          	push	rbp
1405a9e36: 41 57                       	push	r15
1405a9e38: 41 56                       	push	r14
1405a9e3a: 41 55                       	push	r13
1405a9e3c: 41 54                       	push	r12
1405a9e3e: 56                          	push	rsi
1405a9e3f: 57                          	push	rdi
1405a9e40: 53                          	push	rbx
1405a9e41: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a9e48: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a9e4f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405a9e55: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405a9e5e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405a9e67: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405a9e70: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405a9e79: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405a9e82: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405a9e8b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405a9e94: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405a9e9c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405a9ea4: 48 83 e2 80                 	and	rdx, -0x80
1405a9ea8: 48 89 d3                    	mov	rbx, rdx
1405a9eab: 0f b6 b3 db 00 00 00        	movzx	esi, byte ptr [rbx + 0xdb]
1405a9eb2: 40 f6 c6 01                 	test	sil, 0x1
1405a9eb6: 0f 84 97 00 00 00           	je	0x1405a9f53 <.text+0x5a8f53>
1405a9ebc: 48 8b 83 f0 1f 00 00        	mov	rax, qword ptr [rbx + 0x1ff0]
1405a9ec3: f0                          	lock
1405a9ec4: 48 ff 08                    	dec	qword ptr [rax]
1405a9ec7: 75 0c                       	jne	0x1405a9ed5 <.text+0x5a8ed5>
1405a9ec9: 48 8d 8b f0 1f 00 00        	lea	rcx, [rbx + 0x1ff0]
1405a9ed0: e8 bb 1f 5b 00              	call	0x140b5be90 <.text+0xb5ae90>
1405a9ed5: 40 88 b3 f5 00 00 00        	mov	byte ptr [rbx + 0xf5], sil
1405a9edc: c6 83 f6 00 00 00 01        	mov	byte ptr [rbx + 0xf6], 0x1
1405a9ee3: c6 83 f7 00 00 00 01        	mov	byte ptr [rbx + 0xf7], 0x1
1405a9eea: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a9ef2: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a9efa: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a9f03: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a9f0c: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a9f15: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a9f1e: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a9f27: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a9f30: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a9f39: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a9f3f: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a9f46: 5b                          	pop	rbx
1405a9f47: 5f                          	pop	rdi
1405a9f48: 5e                          	pop	rsi
1405a9f49: 41 5c                       	pop	r12
1405a9f4b: 41 5d                       	pop	r13
1405a9f4d: 41 5e                       	pop	r14
1405a9f4f: 41 5f                       	pop	r15
1405a9f51: 5d                          	pop	rbp
1405a9f52: c3                          	ret
1405a9f53: 40 88 b3 f5 00 00 00        	mov	byte ptr [rbx + 0xf5], sil
1405a9f5a: c6 83 f6 00 00 00 01        	mov	byte ptr [rbx + 0xf6], 0x1
1405a9f61: c6 83 f7 00 00 00 01        	mov	byte ptr [rbx + 0xf7], 0x1
1405a9f68: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405a9f70: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405a9f78: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405a9f81: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405a9f8a: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405a9f93: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405a9f9c: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405a9fa5: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405a9fae: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405a9fb7: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405a9fbd: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405a9fc4: 5b                          	pop	rbx
1405a9fc5: 5f                          	pop	rdi
1405a9fc6: 5e                          	pop	rsi
1405a9fc7: 41 5c                       	pop	r12
1405a9fc9: 41 5d                       	pop	r13
1405a9fcb: 41 5e                       	pop	r14
1405a9fcd: 41 5f                       	pop	r15
1405a9fcf: 5d                          	pop	rbp
1405a9fd0: c3                          	ret
1405a9fd1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1405a9fe0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405a9fe5: 55                          	push	rbp
1405a9fe6: 41 57                       	push	r15
1405a9fe8: 41 56                       	push	r14
1405a9fea: 41 55                       	push	r13
1405a9fec: 41 54                       	push	r12
1405a9fee: 56                          	push	rsi
1405a9fef: 57                          	push	rdi
1405a9ff0: 53                          	push	rbx
1405a9ff1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405a9ff8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405a9fff: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405aa005: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405aa00e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405aa017: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405aa020: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405aa029: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405aa032: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405aa03b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405aa044: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405aa04c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405aa054: 48 83 e2 80                 	and	rdx, -0x80
1405aa058: 48 89 d3                    	mov	rbx, rdx
1405aa05b: 0f b6 b3 05 01 00 00        	movzx	esi, byte ptr [rbx + 0x105]
1405aa062: 40 f6 c6 01                 	test	sil, 0x1
1405aa066: 0f 84 d3 00 00 00           	je	0x1405aa13f <.text+0x5a913f>
1405aa06c: 48 8d 8b e8 24 00 00        	lea	rcx, [rbx + 0x24e8]
1405aa073: e8 b8 d5 04 00              	call	0x1405f7630 <.text+0x5f6630>
1405aa078: 48 8b 83 70 01 00 00        	mov	rax, qword ptr [rbx + 0x170]
1405aa07f: f0                          	lock
1405aa080: 48 ff 08                    	dec	qword ptr [rax]
1405aa083: 75 0c                       	jne	0x1405aa091 <.text+0x5a9091>
1405aa085: 48 8d 8b 70 01 00 00        	lea	rcx, [rbx + 0x170]
1405aa08c: e8 af 77 08 00              	call	0x140631840 <.text+0x630840>
1405aa091: 48 8d 8b 80 02 00 00        	lea	rcx, [rbx + 0x280]
1405aa098: e8 d3 c7 05 00              	call	0x140606870 <.text+0x605870>
1405aa09d: 48 8d 8b 70 02 00 00        	lea	rcx, [rbx + 0x270]
1405aa0a4: e8 17 02 06 00              	call	0x14060a2c0 <.text+0x6092c0>
1405aa0a9: 48 8d 8b 40 03 00 00        	lea	rcx, [rbx + 0x340]
1405aa0b0: e8 cb 8b 05 00              	call	0x140602c80 <.text+0x601c80>
1405aa0b5: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405aa0bc: e8 0f cc 03 00              	call	0x1405e6cd0 <.text+0x5e5cd0>
1405aa0c1: c6 83 f5 00 00 00 00        	mov	byte ptr [rbx + 0xf5], 0x0
1405aa0c8: c6 83 f6 00 00 00 00        	mov	byte ptr [rbx + 0xf6], 0x0
1405aa0cf: 40 88 b3 f7 00 00 00        	mov	byte ptr [rbx + 0xf7], sil
1405aa0d6: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aa0de: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aa0e6: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aa0ef: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aa0f8: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aa101: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aa10a: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aa113: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aa11c: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aa125: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aa12b: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aa132: 5b                          	pop	rbx
1405aa133: 5f                          	pop	rdi
1405aa134: 5e                          	pop	rsi
1405aa135: 41 5c                       	pop	r12
1405aa137: 41 5d                       	pop	r13
1405aa139: 41 5e                       	pop	r14
1405aa13b: 41 5f                       	pop	r15
1405aa13d: 5d                          	pop	rbp
1405aa13e: c3                          	ret
1405aa13f: c6 83 f5 00 00 00 00        	mov	byte ptr [rbx + 0xf5], 0x0
1405aa146: c6 83 f6 00 00 00 00        	mov	byte ptr [rbx + 0xf6], 0x0
1405aa14d: 40 88 b3 f7 00 00 00        	mov	byte ptr [rbx + 0xf7], sil
1405aa154: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aa15c: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aa164: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aa16d: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aa176: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aa17f: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aa188: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aa191: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aa19a: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aa1a3: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aa1a9: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aa1b0: 5b                          	pop	rbx
1405aa1b1: 5f                          	pop	rdi
1405aa1b2: 5e                          	pop	rsi
1405aa1b3: 41 5c                       	pop	r12
1405aa1b5: 41 5d                       	pop	r13
1405aa1b7: 41 5e                       	pop	r14
1405aa1b9: 41 5f                       	pop	r15
1405aa1bb: 5d                          	pop	rbp
1405aa1bc: c3                          	ret
1405aa1bd: 0f 1f 00                    	nop	dword ptr [rax]
1405aa1c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405aa1c5: 55                          	push	rbp
1405aa1c6: 41 57                       	push	r15
1405aa1c8: 41 56                       	push	r14
1405aa1ca: 41 55                       	push	r13
1405aa1cc: 41 54                       	push	r12
1405aa1ce: 56                          	push	rsi
1405aa1cf: 57                          	push	rdi
1405aa1d0: 53                          	push	rbx
1405aa1d1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405aa1d8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405aa1df: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405aa1e5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405aa1ee: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405aa1f7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405aa200: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405aa209: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405aa212: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405aa21b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405aa224: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405aa22c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405aa234: 48 83 e2 80                 	and	rdx, -0x80
1405aa238: 48 89 d3                    	mov	rbx, rdx
1405aa23b: 0f b6 83 06 01 00 00        	movzx	eax, byte ptr [rbx + 0x106]
1405aa242: 88 83 e8 00 00 00           	mov	byte ptr [rbx + 0xe8], al
1405aa248: a8 01                       	test	al, 0x1
1405aa24a: 0f 84 8e 00 00 00           	je	0x1405aa2de <.text+0x5a92de>
1405aa250: 48 8d 8b f0 1f 00 00        	lea	rcx, [rbx + 0x1ff0]
1405aa257: e8 d4 d3 04 00              	call	0x1405f7630 <.text+0x5f6630>
1405aa25c: 48 8b 83 70 02 00 00        	mov	rax, qword ptr [rbx + 0x270]
1405aa263: f0                          	lock
1405aa264: 48 ff 08                    	dec	qword ptr [rax]
1405aa267: 75 0c                       	jne	0x1405aa275 <.text+0x5a9275>
1405aa269: 48 8d 8b 70 02 00 00        	lea	rcx, [rbx + 0x270]
1405aa270: e8 cb 75 08 00              	call	0x140631840 <.text+0x630840>
1405aa275: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aa27d: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aa285: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aa28e: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aa297: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aa2a0: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aa2a9: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aa2b2: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aa2bb: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aa2c4: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aa2ca: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aa2d1: 5b                          	pop	rbx
1405aa2d2: 5f                          	pop	rdi
1405aa2d3: 5e                          	pop	rsi
1405aa2d4: 41 5c                       	pop	r12
1405aa2d6: 41 5d                       	pop	r13
1405aa2d8: 41 5e                       	pop	r14
1405aa2da: 41 5f                       	pop	r15
1405aa2dc: 5d                          	pop	rbp
1405aa2dd: c3                          	ret
1405aa2de: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aa2e6: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aa2ee: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aa2f7: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aa300: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aa309: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aa312: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aa31b: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aa324: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aa32d: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aa333: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aa33a: 5b                          	pop	rbx
1405aa33b: 5f                          	pop	rdi
1405aa33c: 5e                          	pop	rsi
1405aa33d: 41 5c                       	pop	r12
1405aa33f: 41 5d                       	pop	r13
1405aa341: 41 5e                       	pop	r14
1405aa343: 41 5f                       	pop	r15
1405aa345: 5d                          	pop	rbp
1405aa346: c3                          	ret
1405aa347: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1405aa350: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405aa355: 55                          	push	rbp
1405aa356: 41 57                       	push	r15
1405aa358: 41 56                       	push	r14
1405aa35a: 41 55                       	push	r13
1405aa35c: 41 54                       	push	r12
1405aa35e: 56                          	push	rsi
1405aa35f: 57                          	push	rdi
1405aa360: 53                          	push	rbx
1405aa361: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405aa368: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405aa36f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405aa375: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405aa37e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405aa387: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405aa390: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405aa399: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405aa3a2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405aa3ab: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405aa3b4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405aa3bc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405aa3c4: 48 83 e2 80                 	and	rdx, -0x80
1405aa3c8: 48 89 d3                    	mov	rbx, rdx
1405aa3cb: f6 83 e8 00 00 00 01        	test	byte ptr [rbx + 0xe8], 0x1
1405aa3d2: 74 75                       	je	0x1405aa449 <.text+0x5a9449>
1405aa3d4: 48 8d 8b 70 03 00 00        	lea	rcx, [rbx + 0x370]
1405aa3db: e8 90 c4 05 00              	call	0x140606870 <.text+0x605870>
1405aa3e0: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aa3e8: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aa3f0: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aa3f9: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aa402: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aa40b: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aa414: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aa41d: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aa426: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aa42f: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aa435: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aa43c: 5b                          	pop	rbx
1405aa43d: 5f                          	pop	rdi
1405aa43e: 5e                          	pop	rsi
1405aa43f: 41 5c                       	pop	r12
1405aa441: 41 5d                       	pop	r13
1405aa443: 41 5e                       	pop	r14
1405aa445: 41 5f                       	pop	r15
1405aa447: 5d                          	pop	rbp
1405aa448: c3                          	ret
1405aa449: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aa451: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aa459: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aa462: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aa46b: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aa474: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aa47d: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aa486: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aa48f: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aa498: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aa49e: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aa4a5: 5b                          	pop	rbx
1405aa4a6: 5f                          	pop	rdi
1405aa4a7: 5e                          	pop	rsi
1405aa4a8: 41 5c                       	pop	r12
1405aa4aa: 41 5d                       	pop	r13
1405aa4ac: 41 5e                       	pop	r14
1405aa4ae: 41 5f                       	pop	r15
1405aa4b0: 5d                          	pop	rbp
1405aa4b1: c3                          	ret
1405aa4b2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1405aa4c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405aa4c5: 55                          	push	rbp
1405aa4c6: 41 57                       	push	r15
1405aa4c8: 41 56                       	push	r14
1405aa4ca: 41 55                       	push	r13
1405aa4cc: 41 54                       	push	r12
1405aa4ce: 56                          	push	rsi
1405aa4cf: 57                          	push	rdi
1405aa4d0: 53                          	push	rbx
1405aa4d1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405aa4d8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405aa4df: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405aa4e5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405aa4ee: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405aa4f7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405aa500: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405aa509: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405aa512: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405aa51b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405aa524: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405aa52c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405aa534: 48 83 e2 80                 	and	rdx, -0x80
1405aa538: 48 89 d3                    	mov	rbx, rdx
1405aa53b: f6 83 e8 00 00 00 01        	test	byte ptr [rbx + 0xe8], 0x1
1405aa542: 0f 84 b6 00 00 00           	je	0x1405aa5fe <.text+0x5a95fe>
1405aa548: 48 8d 8b e8 24 00 00        	lea	rcx, [rbx + 0x24e8]
1405aa54f: e8 6c fd 05 00              	call	0x14060a2c0 <.text+0x6092c0>
1405aa554: 48 8d 8b 80 02 00 00        	lea	rcx, [rbx + 0x280]
1405aa55b: e8 20 87 05 00              	call	0x140602c80 <.text+0x601c80>
1405aa560: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405aa567: e8 64 c7 03 00              	call	0x1405e6cd0 <.text+0x5e5cd0>
1405aa56c: c6 83 c2 00 00 00 00        	mov	byte ptr [rbx + 0xc2], 0x0
1405aa573: c6 83 c3 00 00 00 00        	mov	byte ptr [rbx + 0xc3], 0x0
1405aa57a: c6 83 c4 00 00 00 01        	mov	byte ptr [rbx + 0xc4], 0x1
1405aa581: c6 83 c5 00 00 00 01        	mov	byte ptr [rbx + 0xc5], 0x1
1405aa588: 0f b6 83 e8 00 00 00        	movzx	eax, byte ptr [rbx + 0xe8]
1405aa58f: 88 83 c6 00 00 00           	mov	byte ptr [rbx + 0xc6], al
1405aa595: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aa59d: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aa5a5: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aa5ae: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aa5b7: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aa5c0: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aa5c9: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aa5d2: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aa5db: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aa5e4: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aa5ea: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aa5f1: 5b                          	pop	rbx
1405aa5f2: 5f                          	pop	rdi
1405aa5f3: 5e                          	pop	rsi
1405aa5f4: 41 5c                       	pop	r12
1405aa5f6: 41 5d                       	pop	r13
1405aa5f8: 41 5e                       	pop	r14
1405aa5fa: 41 5f                       	pop	r15
1405aa5fc: 5d                          	pop	rbp
1405aa5fd: c3                          	ret
1405aa5fe: c6 83 c2 00 00 00 00        	mov	byte ptr [rbx + 0xc2], 0x0
1405aa605: c6 83 c3 00 00 00 00        	mov	byte ptr [rbx + 0xc3], 0x0
1405aa60c: c6 83 c4 00 00 00 01        	mov	byte ptr [rbx + 0xc4], 0x1
1405aa613: c6 83 c5 00 00 00 01        	mov	byte ptr [rbx + 0xc5], 0x1
1405aa61a: 0f b6 83 e8 00 00 00        	movzx	eax, byte ptr [rbx + 0xe8]
1405aa621: 88 83 c6 00 00 00           	mov	byte ptr [rbx + 0xc6], al
1405aa627: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aa62f: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aa637: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aa640: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aa649: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aa652: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aa65b: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aa664: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aa66d: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aa676: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aa67c: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aa683: 5b                          	pop	rbx
1405aa684: 5f                          	pop	rdi
1405aa685: 5e                          	pop	rsi
1405aa686: 41 5c                       	pop	r12
1405aa688: 41 5d                       	pop	r13
1405aa68a: 41 5e                       	pop	r14
1405aa68c: 41 5f                       	pop	r15
1405aa68e: 5d                          	pop	rbp
1405aa68f: c3                          	ret
1405aa690: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405aa695: 55                          	push	rbp
1405aa696: 41 57                       	push	r15
1405aa698: 41 56                       	push	r14
1405aa69a: 41 55                       	push	r13
1405aa69c: 41 54                       	push	r12
1405aa69e: 56                          	push	rsi
1405aa69f: 57                          	push	rdi
1405aa6a0: 53                          	push	rbx
1405aa6a1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405aa6a8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405aa6af: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405aa6b5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405aa6be: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405aa6c7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405aa6d0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405aa6d9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405aa6e2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405aa6eb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405aa6f4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405aa6fc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405aa704: 48 83 e2 80                 	and	rdx, -0x80
1405aa708: 48 89 d3                    	mov	rbx, rdx
1405aa70b: 48 83 bb 28 01 00 00 00     	cmp	qword ptr [rbx + 0x128], 0x0
1405aa713: 0f 85 b2 00 00 00           	jne	0x1405aa7cb <.text+0x5a97cb>
1405aa719: 48 8b 93 10 0b 00 00        	mov	rdx, qword ptr [rbx + 0xb10]
1405aa720: 48 8d 04 55 00 00 00 00     	lea	rax, [2*rdx]
1405aa728: 48 85 c0                    	test	rax, rax
1405aa72b: 74 12                       	je	0x1405aa73f <.text+0x5a973f>
1405aa72d: 48 8b 8b 18 0b 00 00        	mov	rcx, qword ptr [rbx + 0xb18]
1405aa734: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405aa73a: e8 e1 94 06 00              	call	0x140613c20 <.text+0x612c20>
1405aa73f: c6 83 d4 00 00 00 01        	mov	byte ptr [rbx + 0xd4], 0x1
1405aa746: c6 83 d5 00 00 00 01        	mov	byte ptr [rbx + 0xd5], 0x1
1405aa74d: c6 83 d6 00 00 00 01        	mov	byte ptr [rbx + 0xd6], 0x1
1405aa754: c6 83 d7 00 00 00 01        	mov	byte ptr [rbx + 0xd7], 0x1
1405aa75b: c6 83 d8 00 00 00 01        	mov	byte ptr [rbx + 0xd8], 0x1
1405aa762: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aa76a: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aa772: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aa77b: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aa784: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aa78d: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aa796: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aa79f: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aa7a8: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aa7b1: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aa7b7: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aa7be: 5b                          	pop	rbx
1405aa7bf: 5f                          	pop	rdi
1405aa7c0: 5e                          	pop	rsi
1405aa7c1: 41 5c                       	pop	r12
1405aa7c3: 41 5d                       	pop	r13
1405aa7c5: 41 5e                       	pop	r14
1405aa7c7: 41 5f                       	pop	r15
1405aa7c9: 5d                          	pop	rbp
1405aa7ca: c3                          	ret
1405aa7cb: c6 83 d4 00 00 00 01        	mov	byte ptr [rbx + 0xd4], 0x1
1405aa7d2: c6 83 d5 00 00 00 01        	mov	byte ptr [rbx + 0xd5], 0x1
1405aa7d9: c6 83 d6 00 00 00 01        	mov	byte ptr [rbx + 0xd6], 0x1
1405aa7e0: c6 83 d7 00 00 00 01        	mov	byte ptr [rbx + 0xd7], 0x1
1405aa7e7: c6 83 d8 00 00 00 01        	mov	byte ptr [rbx + 0xd8], 0x1
1405aa7ee: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aa7f6: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aa7fe: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aa807: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aa810: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aa819: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aa822: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aa82b: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aa834: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aa83d: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aa843: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aa84a: 5b                          	pop	rbx
1405aa84b: 5f                          	pop	rdi
1405aa84c: 5e                          	pop	rsi
1405aa84d: 41 5c                       	pop	r12
1405aa84f: 41 5d                       	pop	r13
1405aa851: 41 5e                       	pop	r14
1405aa853: 41 5f                       	pop	r15
1405aa855: 5d                          	pop	rbp
1405aa856: c3                          	ret
1405aa857: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1405aa860: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405aa865: 55                          	push	rbp
1405aa866: 41 57                       	push	r15
1405aa868: 41 56                       	push	r14
1405aa86a: 41 55                       	push	r13
1405aa86c: 41 54                       	push	r12
1405aa86e: 56                          	push	rsi
1405aa86f: 57                          	push	rdi
1405aa870: 53                          	push	rbx
1405aa871: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405aa878: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405aa87f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405aa885: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405aa88e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405aa897: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405aa8a0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405aa8a9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405aa8b2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405aa8bb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405aa8c4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405aa8cc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405aa8d4: 48 83 e2 80                 	and	rdx, -0x80
1405aa8d8: 48 89 d3                    	mov	rbx, rdx
1405aa8db: 48 8b 93 00 0b 00 00        	mov	rdx, qword ptr [rbx + 0xb00]
1405aa8e2: 48 85 d2                    	test	rdx, rdx
1405aa8e5: 74 12                       	je	0x1405aa8f9 <.text+0x5a98f9>
1405aa8e7: 48 8b 8b 08 0b 00 00        	mov	rcx, qword ptr [rbx + 0xb08]
1405aa8ee: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405aa8f4: e8 27 93 06 00              	call	0x140613c20 <.text+0x612c20>
1405aa8f9: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405aa900: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405aa907: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405aa90e: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405aa915: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405aa91c: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405aa923: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405aa92a: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405aa931: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aa939: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aa941: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aa94a: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aa953: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aa95c: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aa965: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aa96e: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aa977: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aa980: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aa986: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aa98d: 5b                          	pop	rbx
1405aa98e: 5f                          	pop	rdi
1405aa98f: 5e                          	pop	rsi
1405aa990: 41 5c                       	pop	r12
1405aa992: 41 5d                       	pop	r13
1405aa994: 41 5e                       	pop	r14
1405aa996: 41 5f                       	pop	r15
1405aa998: 5d                          	pop	rbp
1405aa999: c3                          	ret
1405aa99a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1405aa9a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405aa9a5: 55                          	push	rbp
1405aa9a6: 41 57                       	push	r15
1405aa9a8: 41 56                       	push	r14
1405aa9aa: 41 55                       	push	r13
1405aa9ac: 41 54                       	push	r12
1405aa9ae: 56                          	push	rsi
1405aa9af: 57                          	push	rdi
1405aa9b0: 53                          	push	rbx
1405aa9b1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405aa9b8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405aa9bf: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405aa9c5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405aa9ce: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405aa9d7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405aa9e0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405aa9e9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405aa9f2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405aa9fb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405aaa04: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405aaa0c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405aaa14: 48 83 e2 80                 	and	rdx, -0x80
1405aaa18: 48 89 d3                    	mov	rbx, rdx
1405aaa1b: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405aaa22: e8 59 ee a7 ff              	call	0x140029880 <.text+0x28880>
1405aaa27: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aaa2f: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aaa37: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aaa40: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aaa49: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aaa52: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aaa5b: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aaa64: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aaa6d: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aaa76: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aaa7c: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aaa83: 5b                          	pop	rbx
1405aaa84: 5f                          	pop	rdi
1405aaa85: 5e                          	pop	rsi
1405aaa86: 41 5c                       	pop	r12
1405aaa88: 41 5d                       	pop	r13
1405aaa8a: 41 5e                       	pop	r14
1405aaa8c: 41 5f                       	pop	r15
1405aaa8e: 5d                          	pop	rbp
1405aaa8f: c3                          	ret
1405aaa90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405aaa95: 55                          	push	rbp
1405aaa96: 41 57                       	push	r15
1405aaa98: 41 56                       	push	r14
1405aaa9a: 41 55                       	push	r13
1405aaa9c: 41 54                       	push	r12
1405aaa9e: 56                          	push	rsi
1405aaa9f: 57                          	push	rdi
1405aaaa0: 53                          	push	rbx
1405aaaa1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405aaaa8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405aaaaf: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405aaab5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405aaabe: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405aaac7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405aaad0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405aaad9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405aaae2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405aaaeb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405aaaf4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405aaafc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405aab04: 48 83 e2 80                 	and	rdx, -0x80
1405aab08: 48 89 d3                    	mov	rbx, rdx
1405aab0b: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405aab12: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405aab19: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405aab20: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405aab27: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405aab2e: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405aab35: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405aab3c: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405aab43: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405aab4a: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405aab51: e8 2a ed a7 ff              	call	0x140029880 <.text+0x28880>
1405aab56: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405aab5d: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405aab64: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405aab6b: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405aab72: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405aab79: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405aab80: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405aab87: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405aab8e: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405aab95: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aab9d: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aaba5: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aabae: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aabb7: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aabc0: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aabc9: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aabd2: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aabdb: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aabe4: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aabea: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aabf1: 5b                          	pop	rbx
1405aabf2: 5f                          	pop	rdi
1405aabf3: 5e                          	pop	rsi
1405aabf4: 41 5c                       	pop	r12
1405aabf6: 41 5d                       	pop	r13
1405aabf8: 41 5e                       	pop	r14
1405aabfa: 41 5f                       	pop	r15
1405aabfc: 5d                          	pop	rbp
1405aabfd: c3                          	ret
1405aabfe: 66 90                       	nop
1405aac00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405aac05: 55                          	push	rbp
1405aac06: 41 57                       	push	r15
1405aac08: 41 56                       	push	r14
1405aac0a: 41 55                       	push	r13
1405aac0c: 41 54                       	push	r12
1405aac0e: 56                          	push	rsi
1405aac0f: 57                          	push	rdi
1405aac10: 53                          	push	rbx
1405aac11: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405aac18: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405aac1f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405aac25: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405aac2e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405aac37: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405aac40: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405aac49: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405aac52: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405aac5b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405aac64: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405aac6c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405aac74: 48 83 e2 80                 	and	rdx, -0x80
1405aac78: 48 89 d3                    	mov	rbx, rdx
1405aac7b: 48 8d 8b f0 2b 00 00        	lea	rcx, [rbx + 0x2bf0]
1405aac82: e8 f9 eb a7 ff              	call	0x140029880 <.text+0x28880>
1405aac87: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aac8f: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aac97: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aaca0: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aaca9: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aacb2: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aacbb: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aacc4: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aaccd: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aacd6: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aacdc: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aace3: 5b                          	pop	rbx
1405aace4: 5f                          	pop	rdi
1405aace5: 5e                          	pop	rsi
1405aace6: 41 5c                       	pop	r12
1405aace8: 41 5d                       	pop	r13
1405aacea: 41 5e                       	pop	r14
1405aacec: 41 5f                       	pop	r15
1405aacee: 5d                          	pop	rbp
1405aacef: c3                          	ret
1405aacf0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405aacf5: 55                          	push	rbp
1405aacf6: 41 57                       	push	r15
1405aacf8: 41 56                       	push	r14
1405aacfa: 41 55                       	push	r13
1405aacfc: 41 54                       	push	r12
1405aacfe: 56                          	push	rsi
1405aacff: 57                          	push	rdi
1405aad00: 53                          	push	rbx
1405aad01: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405aad08: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405aad0f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405aad15: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405aad1e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405aad27: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405aad30: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405aad39: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405aad42: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405aad4b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405aad54: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405aad5c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405aad64: 48 83 e2 80                 	and	rdx, -0x80
1405aad68: 48 89 d3                    	mov	rbx, rdx
1405aad6b: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405aad72: e8 09 eb a7 ff              	call	0x140029880 <.text+0x28880>
1405aad77: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aad7f: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aad87: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aad90: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aad99: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aada2: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aadab: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aadb4: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aadbd: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aadc6: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aadcc: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aadd3: 5b                          	pop	rbx
1405aadd4: 5f                          	pop	rdi
1405aadd5: 5e                          	pop	rsi
1405aadd6: 41 5c                       	pop	r12
1405aadd8: 41 5d                       	pop	r13
1405aadda: 41 5e                       	pop	r14
1405aaddc: 41 5f                       	pop	r15
1405aadde: 5d                          	pop	rbp
1405aaddf: c3                          	ret
1405aade0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405aade5: 55                          	push	rbp
1405aade6: 41 57                       	push	r15
1405aade8: 41 56                       	push	r14
1405aadea: 41 55                       	push	r13
1405aadec: 41 54                       	push	r12
1405aadee: 56                          	push	rsi
1405aadef: 57                          	push	rdi
1405aadf0: 53                          	push	rbx
1405aadf1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405aadf8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405aadff: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405aae05: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405aae0e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405aae17: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405aae20: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405aae29: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405aae32: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405aae3b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405aae44: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405aae4c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405aae54: 48 83 e2 80                 	and	rdx, -0x80
1405aae58: 48 89 d3                    	mov	rbx, rdx
1405aae5b: c6 83 fe 00 00 00 01        	mov	byte ptr [rbx + 0xfe], 0x1
1405aae62: c6 83 ff 00 00 00 01        	mov	byte ptr [rbx + 0xff], 0x1
1405aae69: c6 83 00 01 00 00 01        	mov	byte ptr [rbx + 0x100], 0x1
1405aae70: c6 83 01 01 00 00 01        	mov	byte ptr [rbx + 0x101], 0x1
1405aae77: c6 83 02 01 00 00 01        	mov	byte ptr [rbx + 0x102], 0x1
1405aae7e: c6 83 03 01 00 00 01        	mov	byte ptr [rbx + 0x103], 0x1
1405aae85: c6 83 04 01 00 00 01        	mov	byte ptr [rbx + 0x104], 0x1
1405aae8c: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405aae93: e8 28 6a 04 00              	call	0x1405f18c0 <.text+0x5f08c0>
1405aae98: c6 83 fe 00 00 00 01        	mov	byte ptr [rbx + 0xfe], 0x1
1405aae9f: c6 83 ff 00 00 00 01        	mov	byte ptr [rbx + 0xff], 0x1
1405aaea6: c6 83 00 01 00 00 01        	mov	byte ptr [rbx + 0x100], 0x1
1405aaead: c6 83 01 01 00 00 01        	mov	byte ptr [rbx + 0x101], 0x1
1405aaeb4: c6 83 02 01 00 00 01        	mov	byte ptr [rbx + 0x102], 0x1
1405aaebb: c6 83 03 01 00 00 01        	mov	byte ptr [rbx + 0x103], 0x1
1405aaec2: c6 83 04 01 00 00 01        	mov	byte ptr [rbx + 0x104], 0x1
1405aaec9: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aaed1: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aaed9: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aaee2: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aaeeb: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aaef4: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aaefd: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aaf06: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aaf0f: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aaf18: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aaf1e: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aaf25: 5b                          	pop	rbx
1405aaf26: 5f                          	pop	rdi
1405aaf27: 5e                          	pop	rsi
1405aaf28: 41 5c                       	pop	r12
1405aaf2a: 41 5d                       	pop	r13
1405aaf2c: 41 5e                       	pop	r14
1405aaf2e: 41 5f                       	pop	r15
1405aaf30: 5d                          	pop	rbp
1405aaf31: c3                          	ret
1405aaf32: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1405aaf40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405aaf45: 55                          	push	rbp
1405aaf46: 41 57                       	push	r15
1405aaf48: 41 56                       	push	r14
1405aaf4a: 41 55                       	push	r13
1405aaf4c: 41 54                       	push	r12
1405aaf4e: 56                          	push	rsi
1405aaf4f: 57                          	push	rdi
1405aaf50: 53                          	push	rbx
1405aaf51: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405aaf58: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405aaf5f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405aaf65: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405aaf6e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405aaf77: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405aaf80: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405aaf89: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405aaf92: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405aaf9b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405aafa4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405aafac: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405aafb4: 48 83 e2 80                 	and	rdx, -0x80
1405aafb8: 48 89 d3                    	mov	rbx, rdx
1405aafbb: 48 8b 93 00 0b 00 00        	mov	rdx, qword ptr [rbx + 0xb00]
1405aafc2: 48 85 d2                    	test	rdx, rdx
1405aafc5: 74 12                       	je	0x1405aafd9 <.text+0x5a9fd9>
1405aafc7: 48 8b 8b 08 0b 00 00        	mov	rcx, qword ptr [rbx + 0xb08]
1405aafce: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405aafd4: e8 47 8c 06 00              	call	0x140613c20 <.text+0x612c20>
1405aafd9: c6 83 d3 00 00 00 01        	mov	byte ptr [rbx + 0xd3], 0x1
1405aafe0: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405aafe8: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405aaff0: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aaff9: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ab002: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ab00b: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ab014: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ab01d: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ab026: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ab02f: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ab035: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ab03c: 5b                          	pop	rbx
1405ab03d: 5f                          	pop	rdi
1405ab03e: 5e                          	pop	rsi
1405ab03f: 41 5c                       	pop	r12
1405ab041: 41 5d                       	pop	r13
1405ab043: 41 5e                       	pop	r14
1405ab045: 41 5f                       	pop	r15
1405ab047: 5d                          	pop	rbp
1405ab048: c3                          	ret
1405ab049: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1405ab050: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ab055: 55                          	push	rbp
1405ab056: 41 57                       	push	r15
1405ab058: 41 56                       	push	r14
1405ab05a: 41 55                       	push	r13
1405ab05c: 41 54                       	push	r12
1405ab05e: 56                          	push	rsi
1405ab05f: 57                          	push	rdi
1405ab060: 53                          	push	rbx
1405ab061: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ab068: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ab06f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ab075: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ab07e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ab087: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ab090: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ab099: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ab0a2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ab0ab: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ab0b4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ab0bc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ab0c4: 48 83 e2 80                 	and	rdx, -0x80
1405ab0c8: 48 89 d3                    	mov	rbx, rdx
1405ab0cb: 48 8d 8b f0 2b 00 00        	lea	rcx, [rbx + 0x2bf0]
1405ab0d2: e8 a9 e7 a7 ff              	call	0x140029880 <.text+0x28880>
1405ab0d7: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405ab0de: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405ab0e5: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405ab0ec: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405ab0f3: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405ab0fa: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405ab101: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405ab108: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405ab10f: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405ab116: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ab11e: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ab126: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ab12f: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ab138: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ab141: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ab14a: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ab153: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ab15c: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ab165: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ab16b: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ab172: 5b                          	pop	rbx
1405ab173: 5f                          	pop	rdi
1405ab174: 5e                          	pop	rsi
1405ab175: 41 5c                       	pop	r12
1405ab177: 41 5d                       	pop	r13
1405ab179: 41 5e                       	pop	r14
1405ab17b: 41 5f                       	pop	r15
1405ab17d: 5d                          	pop	rbp
1405ab17e: c3                          	ret
1405ab17f: 90                          	nop
1405ab180: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ab185: 55                          	push	rbp
1405ab186: 41 57                       	push	r15
1405ab188: 41 56                       	push	r14
1405ab18a: 41 55                       	push	r13
1405ab18c: 41 54                       	push	r12
1405ab18e: 56                          	push	rsi
1405ab18f: 57                          	push	rdi
1405ab190: 53                          	push	rbx
1405ab191: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ab198: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ab19f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ab1a5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ab1ae: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ab1b7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ab1c0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ab1c9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ab1d2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ab1db: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ab1e4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ab1ec: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ab1f4: 48 83 e2 80                 	and	rdx, -0x80
1405ab1f8: 48 89 d3                    	mov	rbx, rdx
1405ab1fb: 48 8d 8b 30 22 00 00        	lea	rcx, [rbx + 0x2230]
1405ab202: e8 79 e6 a7 ff              	call	0x140029880 <.text+0x28880>
1405ab207: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ab20f: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ab217: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ab220: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ab229: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ab232: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ab23b: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ab244: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ab24d: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ab256: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ab25c: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ab263: 5b                          	pop	rbx
1405ab264: 5f                          	pop	rdi
1405ab265: 5e                          	pop	rsi
1405ab266: 41 5c                       	pop	r12
1405ab268: 41 5d                       	pop	r13
1405ab26a: 41 5e                       	pop	r14
1405ab26c: 41 5f                       	pop	r15
1405ab26e: 5d                          	pop	rbp
1405ab26f: c3                          	ret
1405ab270: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ab275: 55                          	push	rbp
1405ab276: 41 57                       	push	r15
1405ab278: 41 56                       	push	r14
1405ab27a: 41 55                       	push	r13
1405ab27c: 41 54                       	push	r12
1405ab27e: 56                          	push	rsi
1405ab27f: 57                          	push	rdi
1405ab280: 53                          	push	rbx
1405ab281: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ab288: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ab28f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ab295: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ab29e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ab2a7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ab2b0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ab2b9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ab2c2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ab2cb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ab2d4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ab2dc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ab2e4: 48 83 e2 80                 	and	rdx, -0x80
1405ab2e8: 48 89 d3                    	mov	rbx, rdx
1405ab2eb: 80 bb 10 01 00 00 00        	cmp	byte ptr [rbx + 0x110], 0x0
1405ab2f2: 74 4b                       	je	0x1405ab33f <.text+0x5aa33f>
1405ab2f4: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405ab2fb: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405ab302: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405ab309: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405ab310: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
1405ab317: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405ab31e: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
1405ab325: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405ab32c: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405ab333: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405ab33a: e8 a1 c6 04 00              	call	0x1405f79e0 <.text+0x5f69e0>
1405ab33f: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405ab346: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405ab34d: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405ab354: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405ab35b: c6 83 ae 00 00 00 01        	mov	byte ptr [rbx + 0xae], 0x1
1405ab362: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405ab369: c6 83 b0 00 00 00 01        	mov	byte ptr [rbx + 0xb0], 0x1
1405ab370: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405ab377: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405ab37e: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ab386: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ab38e: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ab397: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ab3a0: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ab3a9: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ab3b2: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ab3bb: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ab3c4: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ab3cd: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ab3d3: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ab3da: 5b                          	pop	rbx
1405ab3db: 5f                          	pop	rdi
1405ab3dc: 5e                          	pop	rsi
1405ab3dd: 41 5c                       	pop	r12
1405ab3df: 41 5d                       	pop	r13
1405ab3e1: 41 5e                       	pop	r14
1405ab3e3: 41 5f                       	pop	r15
1405ab3e5: 5d                          	pop	rbp
1405ab3e6: c3                          	ret
1405ab3e7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1405ab3f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ab3f5: 55                          	push	rbp
1405ab3f6: 41 57                       	push	r15
1405ab3f8: 41 56                       	push	r14
1405ab3fa: 41 55                       	push	r13
1405ab3fc: 41 54                       	push	r12
1405ab3fe: 56                          	push	rsi
1405ab3ff: 57                          	push	rdi
1405ab400: 53                          	push	rbx
1405ab401: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ab408: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ab40f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ab415: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ab41e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ab427: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ab430: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ab439: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ab442: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ab44b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ab454: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ab45c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ab464: 48 83 e2 80                 	and	rdx, -0x80
1405ab468: 48 89 d3                    	mov	rbx, rdx
1405ab46b: 0f b6 b3 f7 00 00 00        	movzx	esi, byte ptr [rbx + 0xf7]
1405ab472: 0f b6 bb f6 00 00 00        	movzx	edi, byte ptr [rbx + 0xf6]
1405ab479: 44 0f b6 b3 f5 00 00 00     	movzx	r14d, byte ptr [rbx + 0xf5]
1405ab481: 41 f6 c6 01                 	test	r14b, 0x1
1405ab485: 0f 84 8a 00 00 00           	je	0x1405ab515 <.text+0x5aa515>
1405ab48b: 48 8d 8b f0 2b 00 00        	lea	rcx, [rbx + 0x2bf0]
1405ab492: e8 c9 d6 03 00              	call	0x1405e8b60 <.text+0x5e7b60>
1405ab497: 44 88 b3 12 01 00 00        	mov	byte ptr [rbx + 0x112], r14b
1405ab49e: 40 88 bb 13 01 00 00        	mov	byte ptr [rbx + 0x113], dil
1405ab4a5: 40 88 b3 14 01 00 00        	mov	byte ptr [rbx + 0x114], sil
1405ab4ac: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ab4b4: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ab4bc: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ab4c5: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ab4ce: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ab4d7: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ab4e0: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ab4e9: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ab4f2: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ab4fb: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ab501: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ab508: 5b                          	pop	rbx
1405ab509: 5f                          	pop	rdi
1405ab50a: 5e                          	pop	rsi
1405ab50b: 41 5c                       	pop	r12
1405ab50d: 41 5d                       	pop	r13
1405ab50f: 41 5e                       	pop	r14
1405ab511: 41 5f                       	pop	r15
1405ab513: 5d                          	pop	rbp
1405ab514: c3                          	ret
1405ab515: 44 88 b3 12 01 00 00        	mov	byte ptr [rbx + 0x112], r14b
1405ab51c: 40 88 bb 13 01 00 00        	mov	byte ptr [rbx + 0x113], dil
1405ab523: 40 88 b3 14 01 00 00        	mov	byte ptr [rbx + 0x114], sil
1405ab52a: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ab532: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ab53a: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ab543: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ab54c: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ab555: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ab55e: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ab567: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ab570: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ab579: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ab57f: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ab586: 5b                          	pop	rbx
1405ab587: 5f                          	pop	rdi
1405ab588: 5e                          	pop	rsi
1405ab589: 41 5c                       	pop	r12
1405ab58b: 41 5d                       	pop	r13
1405ab58d: 41 5e                       	pop	r14
1405ab58f: 41 5f                       	pop	r15
1405ab591: 5d                          	pop	rbp
1405ab592: c3                          	ret
1405ab593: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1405ab5a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ab5a5: 55                          	push	rbp
1405ab5a6: 41 57                       	push	r15
1405ab5a8: 41 56                       	push	r14
1405ab5aa: 41 55                       	push	r13
1405ab5ac: 41 54                       	push	r12
1405ab5ae: 56                          	push	rsi
1405ab5af: 57                          	push	rdi
1405ab5b0: 53                          	push	rbx
1405ab5b1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ab5b8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ab5bf: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ab5c5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ab5ce: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ab5d7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ab5e0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ab5e9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ab5f2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ab5fb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ab604: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ab60c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ab614: 48 83 e2 80                 	and	rdx, -0x80
1405ab618: 48 89 d3                    	mov	rbx, rdx
1405ab61b: 0f b6 b3 14 01 00 00        	movzx	esi, byte ptr [rbx + 0x114]
1405ab622: 44 0f b6 b3 13 01 00 00     	movzx	r14d, byte ptr [rbx + 0x113]
1405ab62a: 0f b6 bb 12 01 00 00        	movzx	edi, byte ptr [rbx + 0x112]
1405ab631: 41 f6 c6 01                 	test	r14b, 0x1
1405ab635: 0f 84 98 00 00 00           	je	0x1405ab6d3 <.text+0x5aa6d3>
1405ab63b: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405ab642: e8 89 b6 03 00              	call	0x1405e6cd0 <.text+0x5e5cd0>
1405ab647: 44 88 b3 c2 00 00 00        	mov	byte ptr [rbx + 0xc2], r14b
1405ab64e: c6 83 c3 00 00 00 01        	mov	byte ptr [rbx + 0xc3], 0x1
1405ab655: 44 88 b3 c4 00 00 00        	mov	byte ptr [rbx + 0xc4], r14b
1405ab65c: 40 88 bb c5 00 00 00        	mov	byte ptr [rbx + 0xc5], dil
1405ab663: 40 88 b3 c6 00 00 00        	mov	byte ptr [rbx + 0xc6], sil
1405ab66a: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ab672: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ab67a: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ab683: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ab68c: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ab695: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ab69e: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ab6a7: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ab6b0: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ab6b9: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ab6bf: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ab6c6: 5b                          	pop	rbx
1405ab6c7: 5f                          	pop	rdi
1405ab6c8: 5e                          	pop	rsi
1405ab6c9: 41 5c                       	pop	r12
1405ab6cb: 41 5d                       	pop	r13
1405ab6cd: 41 5e                       	pop	r14
1405ab6cf: 41 5f                       	pop	r15
1405ab6d1: 5d                          	pop	rbp
1405ab6d2: c3                          	ret
1405ab6d3: 44 88 b3 c2 00 00 00        	mov	byte ptr [rbx + 0xc2], r14b
1405ab6da: c6 83 c3 00 00 00 01        	mov	byte ptr [rbx + 0xc3], 0x1
1405ab6e1: 44 88 b3 c4 00 00 00        	mov	byte ptr [rbx + 0xc4], r14b
1405ab6e8: 40 88 bb c5 00 00 00        	mov	byte ptr [rbx + 0xc5], dil
1405ab6ef: 40 88 b3 c6 00 00 00        	mov	byte ptr [rbx + 0xc6], sil
1405ab6f6: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ab6fe: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ab706: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ab70f: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ab718: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ab721: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ab72a: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ab733: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ab73c: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ab745: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ab74b: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ab752: 5b                          	pop	rbx
1405ab753: 5f                          	pop	rdi
1405ab754: 5e                          	pop	rsi
1405ab755: 41 5c                       	pop	r12
1405ab757: 41 5d                       	pop	r13
1405ab759: 41 5e                       	pop	r14
1405ab75b: 41 5f                       	pop	r15
1405ab75d: 5d                          	pop	rbp
1405ab75e: c3                          	ret
1405ab75f: 90                          	nop
1405ab760: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ab765: 55                          	push	rbp
1405ab766: 41 57                       	push	r15
1405ab768: 41 56                       	push	r14
1405ab76a: 41 55                       	push	r13
1405ab76c: 41 54                       	push	r12
1405ab76e: 56                          	push	rsi
1405ab76f: 57                          	push	rdi
1405ab770: 53                          	push	rbx
1405ab771: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ab778: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ab77f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ab785: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ab78e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ab797: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ab7a0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ab7a9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ab7b2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ab7bb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ab7c4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ab7cc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ab7d4: 48 83 e2 80                 	and	rdx, -0x80
1405ab7d8: 48 89 d3                    	mov	rbx, rdx
1405ab7db: 48 8d 8b f0 1f 00 00        	lea	rcx, [rbx + 0x1ff0]
1405ab7e2: e8 99 e0 a7 ff              	call	0x140029880 <.text+0x28880>
1405ab7e7: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ab7ef: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ab7f7: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ab800: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ab809: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ab812: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ab81b: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ab824: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ab82d: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ab836: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ab83c: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ab843: 5b                          	pop	rbx
1405ab844: 5f                          	pop	rdi
1405ab845: 5e                          	pop	rsi
1405ab846: 41 5c                       	pop	r12
1405ab848: 41 5d                       	pop	r13
1405ab84a: 41 5e                       	pop	r14
1405ab84c: 41 5f                       	pop	r15
1405ab84e: 5d                          	pop	rbp
1405ab84f: c3                          	ret
1405ab850: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ab855: 55                          	push	rbp
1405ab856: 41 57                       	push	r15
1405ab858: 41 56                       	push	r14
1405ab85a: 41 55                       	push	r13
1405ab85c: 41 54                       	push	r12
1405ab85e: 56                          	push	rsi
1405ab85f: 57                          	push	rdi
1405ab860: 53                          	push	rbx
1405ab861: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ab868: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ab86f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ab875: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ab87e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ab887: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ab890: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ab899: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ab8a2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ab8ab: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ab8b4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ab8bc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ab8c4: 48 83 e2 80                 	and	rdx, -0x80
1405ab8c8: 48 89 d3                    	mov	rbx, rdx
1405ab8cb: 0f b6 b3 c7 00 00 00        	movzx	esi, byte ptr [rbx + 0xc7]
1405ab8d2: 48 8b 93 30 22 00 00        	mov	rdx, qword ptr [rbx + 0x2230]
1405ab8d9: 48 85 d2                    	test	rdx, rdx
1405ab8dc: 74 12                       	je	0x1405ab8f0 <.text+0x5aa8f0>
1405ab8de: 48 8b 8b 38 22 00 00        	mov	rcx, qword ptr [rbx + 0x2238]
1405ab8e5: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405ab8eb: e8 30 83 06 00              	call	0x140613c20 <.text+0x612c20>
1405ab8f0: 40 88 b3 d3 00 00 00        	mov	byte ptr [rbx + 0xd3], sil
1405ab8f7: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ab8ff: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ab907: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ab910: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ab919: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ab922: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ab92b: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ab934: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ab93d: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ab946: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ab94c: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ab953: 5b                          	pop	rbx
1405ab954: 5f                          	pop	rdi
1405ab955: 5e                          	pop	rsi
1405ab956: 41 5c                       	pop	r12
1405ab958: 41 5d                       	pop	r13
1405ab95a: 41 5e                       	pop	r14
1405ab95c: 41 5f                       	pop	r15
1405ab95e: 5d                          	pop	rbp
1405ab95f: c3                          	ret
1405ab960: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ab965: 55                          	push	rbp
1405ab966: 41 57                       	push	r15
1405ab968: 41 56                       	push	r14
1405ab96a: 41 55                       	push	r13
1405ab96c: 41 54                       	push	r12
1405ab96e: 56                          	push	rsi
1405ab96f: 57                          	push	rdi
1405ab970: 53                          	push	rbx
1405ab971: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ab978: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ab97f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ab985: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ab98e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ab997: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ab9a0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ab9a9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ab9b2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ab9bb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ab9c4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ab9cc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ab9d4: 48 83 e2 80                 	and	rdx, -0x80
1405ab9d8: 48 89 d3                    	mov	rbx, rdx
1405ab9db: 48 8d 8b 70 03 00 00        	lea	rcx, [rbx + 0x370]
1405ab9e2: e8 99 de a7 ff              	call	0x140029880 <.text+0x28880>
1405ab9e7: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ab9ef: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ab9f7: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405aba00: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405aba09: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aba12: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405aba1b: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405aba24: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405aba2d: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405aba36: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405aba3c: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405aba43: 5b                          	pop	rbx
1405aba44: 5f                          	pop	rdi
1405aba45: 5e                          	pop	rsi
1405aba46: 41 5c                       	pop	r12
1405aba48: 41 5d                       	pop	r13
1405aba4a: 41 5e                       	pop	r14
1405aba4c: 41 5f                       	pop	r15
1405aba4e: 5d                          	pop	rbp
1405aba4f: c3                          	ret
1405aba50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405aba55: 55                          	push	rbp
1405aba56: 41 57                       	push	r15
1405aba58: 41 56                       	push	r14
1405aba5a: 41 55                       	push	r13
1405aba5c: 41 54                       	push	r12
1405aba5e: 56                          	push	rsi
1405aba5f: 57                          	push	rdi
1405aba60: 53                          	push	rbx
1405aba61: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405aba68: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405aba6f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405aba75: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405aba7e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405aba87: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405aba90: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405aba99: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405abaa2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405abaab: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405abab4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ababc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405abac4: 48 83 e2 80                 	and	rdx, -0x80
1405abac8: 48 89 d3                    	mov	rbx, rdx
1405abacb: 48 8d 8b e8 24 00 00        	lea	rcx, [rbx + 0x24e8]
1405abad2: e8 a9 dd a7 ff              	call	0x140029880 <.text+0x28880>
1405abad7: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405abade: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405abae5: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405abaec: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405abaf3: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405abafa: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405abb01: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405abb08: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405abb0f: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405abb16: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405abb1e: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405abb26: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405abb2f: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405abb38: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405abb41: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405abb4a: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405abb53: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405abb5c: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405abb65: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405abb6b: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405abb72: 5b                          	pop	rbx
1405abb73: 5f                          	pop	rdi
1405abb74: 5e                          	pop	rsi
1405abb75: 41 5c                       	pop	r12
1405abb77: 41 5d                       	pop	r13
1405abb79: 41 5e                       	pop	r14
1405abb7b: 41 5f                       	pop	r15
1405abb7d: 5d                          	pop	rbp
1405abb7e: c3                          	ret
1405abb7f: 90                          	nop
1405abb80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405abb85: 55                          	push	rbp
1405abb86: 41 57                       	push	r15
1405abb88: 41 56                       	push	r14
1405abb8a: 41 55                       	push	r13
1405abb8c: 41 54                       	push	r12
1405abb8e: 56                          	push	rsi
1405abb8f: 57                          	push	rdi
1405abb90: 53                          	push	rbx
1405abb91: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405abb98: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405abb9f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405abba5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405abbae: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405abbb7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405abbc0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405abbc9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405abbd2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405abbdb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405abbe4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405abbec: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405abbf4: 48 83 e2 80                 	and	rdx, -0x80
1405abbf8: 48 89 d3                    	mov	rbx, rdx
1405abbfb: 0f b6 b3 d3 00 00 00        	movzx	esi, byte ptr [rbx + 0xd3]
1405abc02: 48 8b 93 f0 1f 00 00        	mov	rdx, qword ptr [rbx + 0x1ff0]
1405abc09: 48 85 d2                    	test	rdx, rdx
1405abc0c: 74 12                       	je	0x1405abc20 <.text+0x5aac20>
1405abc0e: 48 8b 8b f8 1f 00 00        	mov	rcx, qword ptr [rbx + 0x1ff8]
1405abc15: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405abc1b: e8 00 80 06 00              	call	0x140613c20 <.text+0x612c20>
1405abc20: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405abc27: 40 88 b3 b5 00 00 00        	mov	byte ptr [rbx + 0xb5], sil
1405abc2e: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405abc35: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405abc3c: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405abc43: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405abc4a: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405abc51: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405abc58: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405abc60: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405abc68: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405abc71: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405abc7a: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405abc83: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405abc8c: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405abc95: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405abc9e: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405abca7: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405abcad: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405abcb4: 5b                          	pop	rbx
1405abcb5: 5f                          	pop	rdi
1405abcb6: 5e                          	pop	rsi
1405abcb7: 41 5c                       	pop	r12
1405abcb9: 41 5d                       	pop	r13
1405abcbb: 41 5e                       	pop	r14
1405abcbd: 41 5f                       	pop	r15
1405abcbf: 5d                          	pop	rbp
1405abcc0: c3                          	ret
1405abcc1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1405abcd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405abcd5: 55                          	push	rbp
1405abcd6: 41 57                       	push	r15
1405abcd8: 41 56                       	push	r14
1405abcda: 41 55                       	push	r13
1405abcdc: 41 54                       	push	r12
1405abcde: 56                          	push	rsi
1405abcdf: 57                          	push	rdi
1405abce0: 53                          	push	rbx
1405abce1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405abce8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405abcef: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405abcf5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405abcfe: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405abd07: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405abd10: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405abd19: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405abd22: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405abd2b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405abd34: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405abd3c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405abd44: 48 83 e2 80                 	and	rdx, -0x80
1405abd48: 48 89 d3                    	mov	rbx, rdx
1405abd4b: 0f b6 b3 e3 00 00 00        	movzx	esi, byte ptr [rbx + 0xe3]
1405abd52: 48 8b 83 70 03 00 00        	mov	rax, qword ptr [rbx + 0x370]
1405abd59: 48 85 c0                    	test	rax, rax
1405abd5c: 74 12                       	je	0x1405abd70 <.text+0x5aad70>
1405abd5e: f0                          	lock
1405abd5f: 48 ff 08                    	dec	qword ptr [rax]
1405abd62: 75 0c                       	jne	0x1405abd70 <.text+0x5aad70>
1405abd64: 48 8d 8b 70 03 00 00        	lea	rcx, [rbx + 0x370]
1405abd6b: e8 10 a0 d3 ff              	call	0x1402e5d80 <.text+0x2e4d80>
1405abd70: 40 80 e6 01                 	and	sil, 0x1
1405abd74: 40 88 b3 d9 00 00 00        	mov	byte ptr [rbx + 0xd9], sil
1405abd7b: c6 83 da 00 00 00 01        	mov	byte ptr [rbx + 0xda], 0x1
1405abd82: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405abd8a: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405abd92: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405abd9b: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405abda4: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405abdad: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405abdb6: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405abdbf: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405abdc8: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405abdd1: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405abdd7: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405abdde: 5b                          	pop	rbx
1405abddf: 5f                          	pop	rdi
1405abde0: 5e                          	pop	rsi
1405abde1: 41 5c                       	pop	r12
1405abde3: 41 5d                       	pop	r13
1405abde5: 41 5e                       	pop	r14
1405abde7: 41 5f                       	pop	r15
1405abde9: 5d                          	pop	rbp
1405abdea: c3                          	ret
1405abdeb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1405abdf0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405abdf5: 55                          	push	rbp
1405abdf6: 41 57                       	push	r15
1405abdf8: 41 56                       	push	r14
1405abdfa: 41 55                       	push	r13
1405abdfc: 41 54                       	push	r12
1405abdfe: 56                          	push	rsi
1405abdff: 57                          	push	rdi
1405abe00: 53                          	push	rbx
1405abe01: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405abe08: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405abe0f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405abe15: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405abe1e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405abe27: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405abe30: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405abe39: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405abe42: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405abe4b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405abe54: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405abe5c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405abe64: 48 83 e2 80                 	and	rdx, -0x80
1405abe68: 48 89 d3                    	mov	rbx, rdx
1405abe6b: 0f b6 b3 d9 00 00 00        	movzx	esi, byte ptr [rbx + 0xd9]
1405abe72: 80 bb da 00 00 00 00        	cmp	byte ptr [rbx + 0xda], 0x0
1405abe79: 0f 84 8f 00 00 00           	je	0x1405abf0e <.text+0x5aaf0e>
1405abe7f: 31 c0                       	xor	eax, eax
1405abe81: 48 3b 83 90 25 00 00        	cmp	rax, qword ptr [rbx + 0x2590]
1405abe88: 0f 80 f4 00 00 00           	jo	0x1405abf82 <.text+0x5aaf82>
1405abe8e: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
1405abe95: e8 26 6e fe ff              	call	0x140592cc0 <.text+0x591cc0>
1405abe9a: 40 80 e6 01                 	and	sil, 0x1
1405abe9e: 40 88 b3 f4 00 00 00        	mov	byte ptr [rbx + 0xf4], sil
1405abea5: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405abead: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405abeb5: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405abebe: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405abec7: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405abed0: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405abed9: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405abee2: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405abeeb: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405abef4: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405abefa: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405abf01: 5b                          	pop	rbx
1405abf02: 5f                          	pop	rdi
1405abf03: 5e                          	pop	rsi
1405abf04: 41 5c                       	pop	r12
1405abf06: 41 5d                       	pop	r13
1405abf08: 41 5e                       	pop	r14
1405abf0a: 41 5f                       	pop	r15
1405abf0c: 5d                          	pop	rbp
1405abf0d: c3                          	ret
1405abf0e: 40 80 e6 01                 	and	sil, 0x1
1405abf12: 40 88 b3 f4 00 00 00        	mov	byte ptr [rbx + 0xf4], sil
1405abf19: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405abf21: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405abf29: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405abf32: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405abf3b: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405abf44: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405abf4d: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405abf56: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405abf5f: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405abf68: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405abf6e: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405abf75: 5b                          	pop	rbx
1405abf76: 5f                          	pop	rdi
1405abf77: 5e                          	pop	rsi
1405abf78: 41 5c                       	pop	r12
1405abf7a: 41 5d                       	pop	r13
1405abf7c: 41 5e                       	pop	r14
1405abf7e: 41 5f                       	pop	r15
1405abf80: 5d                          	pop	rbp
1405abf81: c3                          	ret
1405abf82: 40 80 e6 01                 	and	sil, 0x1
1405abf86: 40 88 b3 f4 00 00 00        	mov	byte ptr [rbx + 0xf4], sil
1405abf8d: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405abf95: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405abf9d: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405abfa6: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405abfaf: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405abfb8: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405abfc1: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405abfca: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405abfd3: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405abfdc: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405abfe2: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405abfe9: 5b                          	pop	rbx
1405abfea: 5f                          	pop	rdi
1405abfeb: 5e                          	pop	rsi
1405abfec: 41 5c                       	pop	r12
1405abfee: 41 5d                       	pop	r13
1405abff0: 41 5e                       	pop	r14
1405abff2: 41 5f                       	pop	r15
1405abff4: 5d                          	pop	rbp
1405abff5: c3                          	ret
1405abff6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1405ac000: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ac005: 55                          	push	rbp
1405ac006: 41 57                       	push	r15
1405ac008: 41 56                       	push	r14
1405ac00a: 41 55                       	push	r13
1405ac00c: 41 54                       	push	r12
1405ac00e: 56                          	push	rsi
1405ac00f: 57                          	push	rdi
1405ac010: 53                          	push	rbx
1405ac011: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ac018: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ac01f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ac025: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ac02e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ac037: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ac040: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ac049: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ac052: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ac05b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ac064: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ac06c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ac074: 48 83 e2 80                 	and	rdx, -0x80
1405ac078: 48 89 d3                    	mov	rbx, rdx
1405ac07b: 80 bb f4 00 00 00 00        	cmp	byte ptr [rbx + 0xf4], 0x0
1405ac082: 0f 84 ad 00 00 00           	je	0x1405ac135 <.text+0x5ab135>
1405ac088: 48 8d 8b 30 22 00 00        	lea	rcx, [rbx + 0x2230]
1405ac08f: e8 5c 6c 02 00              	call	0x1405d2cf0 <.text+0x5d1cf0>
1405ac094: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405ac09b: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405ac0a2: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405ac0a9: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405ac0b0: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405ac0b7: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405ac0be: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405ac0c5: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405ac0cc: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ac0d4: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ac0dc: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ac0e5: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ac0ee: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ac0f7: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ac100: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ac109: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ac112: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ac11b: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ac121: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ac128: 5b                          	pop	rbx
1405ac129: 5f                          	pop	rdi
1405ac12a: 5e                          	pop	rsi
1405ac12b: 41 5c                       	pop	r12
1405ac12d: 41 5d                       	pop	r13
1405ac12f: 41 5e                       	pop	r14
1405ac131: 41 5f                       	pop	r15
1405ac133: 5d                          	pop	rbp
1405ac134: c3                          	ret
1405ac135: c6 83 b4 00 00 00 01        	mov	byte ptr [rbx + 0xb4], 0x1
1405ac13c: c6 83 b5 00 00 00 01        	mov	byte ptr [rbx + 0xb5], 0x1
1405ac143: c6 83 b6 00 00 00 01        	mov	byte ptr [rbx + 0xb6], 0x1
1405ac14a: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405ac151: c6 83 b8 00 00 00 01        	mov	byte ptr [rbx + 0xb8], 0x1
1405ac158: c6 83 b9 00 00 00 01        	mov	byte ptr [rbx + 0xb9], 0x1
1405ac15f: c6 83 ba 00 00 00 01        	mov	byte ptr [rbx + 0xba], 0x1
1405ac166: c6 83 bb 00 00 00 01        	mov	byte ptr [rbx + 0xbb], 0x1
1405ac16d: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ac175: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ac17d: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ac186: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ac18f: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ac198: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ac1a1: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ac1aa: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ac1b3: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ac1bc: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ac1c2: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ac1c9: 5b                          	pop	rbx
1405ac1ca: 5f                          	pop	rdi
1405ac1cb: 5e                          	pop	rsi
1405ac1cc: 41 5c                       	pop	r12
1405ac1ce: 41 5d                       	pop	r13
1405ac1d0: 41 5e                       	pop	r14
1405ac1d2: 41 5f                       	pop	r15
1405ac1d4: 5d                          	pop	rbp
1405ac1d5: c3                          	ret
1405ac1d6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1405ac1e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ac1e5: 55                          	push	rbp
1405ac1e6: 41 57                       	push	r15
1405ac1e8: 41 56                       	push	r14
1405ac1ea: 41 55                       	push	r13
1405ac1ec: 41 54                       	push	r12
1405ac1ee: 56                          	push	rsi
1405ac1ef: 57                          	push	rdi
1405ac1f0: 53                          	push	rbx
1405ac1f1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ac1f8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ac1ff: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ac205: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ac20e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ac217: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ac220: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ac229: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ac232: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ac23b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ac244: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ac24c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ac254: 48 83 e2 80                 	and	rdx, -0x80
1405ac258: 48 89 d3                    	mov	rbx, rdx
1405ac25b: 0f b6 b3 b3 00 00 00        	movzx	esi, byte ptr [rbx + 0xb3]
1405ac262: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405ac269: e8 62 aa 03 00              	call	0x1405e6cd0 <.text+0x5e5cd0>
1405ac26e: 40 88 b3 1f 01 00 00        	mov	byte ptr [rbx + 0x11f], sil
1405ac275: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ac27d: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ac285: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ac28e: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ac297: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ac2a0: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ac2a9: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ac2b2: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ac2bb: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ac2c4: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ac2ca: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ac2d1: 5b                          	pop	rbx
1405ac2d2: 5f                          	pop	rdi
1405ac2d3: 5e                          	pop	rsi
1405ac2d4: 41 5c                       	pop	r12
1405ac2d6: 41 5d                       	pop	r13
1405ac2d8: 41 5e                       	pop	r14
1405ac2da: 41 5f                       	pop	r15
1405ac2dc: 5d                          	pop	rbp
1405ac2dd: c3                          	ret
1405ac2de: 66 90                       	nop
1405ac2e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ac2e5: 55                          	push	rbp
1405ac2e6: 41 57                       	push	r15
1405ac2e8: 41 56                       	push	r14
1405ac2ea: 41 55                       	push	r13
1405ac2ec: 41 54                       	push	r12
1405ac2ee: 56                          	push	rsi
1405ac2ef: 57                          	push	rdi
1405ac2f0: 53                          	push	rbx
1405ac2f1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ac2f8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ac2ff: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ac305: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ac30e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ac317: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ac320: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ac329: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ac332: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ac33b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ac344: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ac34c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ac354: 48 83 e2 80                 	and	rdx, -0x80
1405ac358: 48 89 d3                    	mov	rbx, rdx
1405ac35b: 80 bb 1f 01 00 00 00        	cmp	byte ptr [rbx + 0x11f], 0x0
1405ac362: 74 3c                       	je	0x1405ac3a0 <.text+0x5ab3a0>
1405ac364: 48 8b 83 30 22 00 00        	mov	rax, qword ptr [rbx + 0x2230]
1405ac36b: f0                          	lock
1405ac36c: 48 ff 08                    	dec	qword ptr [rax]
1405ac36f: 75 2f                       	jne	0x1405ac3a0 <.text+0x5ab3a0>
1405ac371: c6 83 c2 00 00 00 01        	mov	byte ptr [rbx + 0xc2], 0x1
1405ac378: c6 83 c3 00 00 00 01        	mov	byte ptr [rbx + 0xc3], 0x1
1405ac37f: c6 83 c4 00 00 00 01        	mov	byte ptr [rbx + 0xc4], 0x1
1405ac386: c6 83 c5 00 00 00 01        	mov	byte ptr [rbx + 0xc5], 0x1
1405ac38d: c6 83 c6 00 00 00 01        	mov	byte ptr [rbx + 0xc6], 0x1
1405ac394: 48 8d 8b 30 22 00 00        	lea	rcx, [rbx + 0x2230]
1405ac39b: e8 f0 fa 5a 00              	call	0x140b5be90 <.text+0xb5ae90>
1405ac3a0: c6 83 c2 00 00 00 01        	mov	byte ptr [rbx + 0xc2], 0x1
1405ac3a7: c6 83 c3 00 00 00 01        	mov	byte ptr [rbx + 0xc3], 0x1
1405ac3ae: c6 83 c4 00 00 00 01        	mov	byte ptr [rbx + 0xc4], 0x1
1405ac3b5: c6 83 c5 00 00 00 01        	mov	byte ptr [rbx + 0xc5], 0x1
1405ac3bc: c6 83 c6 00 00 00 01        	mov	byte ptr [rbx + 0xc6], 0x1
1405ac3c3: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ac3cb: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ac3d3: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ac3dc: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ac3e5: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ac3ee: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ac3f7: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ac400: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ac409: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ac412: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ac418: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ac41f: 5b                          	pop	rbx
1405ac420: 5f                          	pop	rdi
1405ac421: 5e                          	pop	rsi
1405ac422: 41 5c                       	pop	r12
1405ac424: 41 5d                       	pop	r13
1405ac426: 41 5e                       	pop	r14
1405ac428: 41 5f                       	pop	r15
1405ac42a: 5d                          	pop	rbp
1405ac42b: c3                          	ret
1405ac42c: 0f 1f 40 00                 	nop	dword ptr [rax]
1405ac430: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ac435: 55                          	push	rbp
1405ac436: 41 57                       	push	r15
1405ac438: 41 56                       	push	r14
1405ac43a: 41 55                       	push	r13
1405ac43c: 41 54                       	push	r12
1405ac43e: 56                          	push	rsi
1405ac43f: 57                          	push	rdi
1405ac440: 53                          	push	rbx
1405ac441: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ac448: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ac44f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ac455: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ac45e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ac467: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ac470: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ac479: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ac482: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ac48b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ac494: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ac49c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ac4a4: 48 83 e2 80                 	and	rdx, -0x80
1405ac4a8: 48 89 d3                    	mov	rbx, rdx
1405ac4ab: 0f b6 b3 c6 00 00 00        	movzx	esi, byte ptr [rbx + 0xc6]
1405ac4b2: 44 0f b6 a3 c5 00 00 00     	movzx	r12d, byte ptr [rbx + 0xc5]
1405ac4ba: 0f b6 bb c4 00 00 00        	movzx	edi, byte ptr [rbx + 0xc4]
1405ac4c1: 44 0f b6 b3 c3 00 00 00     	movzx	r14d, byte ptr [rbx + 0xc3]
1405ac4c9: 44 0f b6 bb c2 00 00 00     	movzx	r15d, byte ptr [rbx + 0xc2]
1405ac4d1: 41 f6 c7 01                 	test	r15b, 0x1
1405ac4d5: 74 0c                       	je	0x1405ac4e3 <.text+0x5ab4e3>
1405ac4d7: 48 8d 8b f0 02 00 00        	lea	rcx, [rbx + 0x2f0]
1405ac4de: e8 9d 67 05 00              	call	0x140602c80 <.text+0x601c80>
1405ac4e3: 41 f6 c4 01                 	test	r12b, 0x1
1405ac4e7: 0f 84 91 00 00 00           	je	0x1405ac57e <.text+0x5ab57e>
1405ac4ed: 48 8d 8b e0 02 00 00        	lea	rcx, [rbx + 0x2e0]
1405ac4f4: e8 67 58 05 00              	call	0x140601d60 <.text+0x600d60>
1405ac4f9: 44 88 bb e4 00 00 00        	mov	byte ptr [rbx + 0xe4], r15b
1405ac500: 44 88 b3 e5 00 00 00        	mov	byte ptr [rbx + 0xe5], r14b
1405ac507: 40 88 bb e6 00 00 00        	mov	byte ptr [rbx + 0xe6], dil
1405ac50e: 40 88 b3 e7 00 00 00        	mov	byte ptr [rbx + 0xe7], sil
1405ac515: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ac51d: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ac525: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ac52e: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ac537: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ac540: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ac549: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ac552: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ac55b: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ac564: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ac56a: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ac571: 5b                          	pop	rbx
1405ac572: 5f                          	pop	rdi
1405ac573: 5e                          	pop	rsi
1405ac574: 41 5c                       	pop	r12
1405ac576: 41 5d                       	pop	r13
1405ac578: 41 5e                       	pop	r14
1405ac57a: 41 5f                       	pop	r15
1405ac57c: 5d                          	pop	rbp
1405ac57d: c3                          	ret
1405ac57e: 44 88 bb e4 00 00 00        	mov	byte ptr [rbx + 0xe4], r15b
1405ac585: 44 88 b3 e5 00 00 00        	mov	byte ptr [rbx + 0xe5], r14b
1405ac58c: 40 88 bb e6 00 00 00        	mov	byte ptr [rbx + 0xe6], dil
1405ac593: 40 88 b3 e7 00 00 00        	mov	byte ptr [rbx + 0xe7], sil
1405ac59a: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ac5a2: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ac5aa: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ac5b3: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ac5bc: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ac5c5: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ac5ce: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ac5d7: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ac5e0: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ac5e9: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ac5ef: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ac5f6: 5b                          	pop	rbx
1405ac5f7: 5f                          	pop	rdi
1405ac5f8: 5e                          	pop	rsi
1405ac5f9: 41 5c                       	pop	r12
1405ac5fb: 41 5d                       	pop	r13
1405ac5fd: 41 5e                       	pop	r14
1405ac5ff: 41 5f                       	pop	r15
1405ac601: 5d                          	pop	rbp
1405ac602: c3                          	ret
1405ac603: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1405ac610: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ac615: 55                          	push	rbp
1405ac616: 41 57                       	push	r15
1405ac618: 41 56                       	push	r14
1405ac61a: 41 55                       	push	r13
1405ac61c: 41 54                       	push	r12
1405ac61e: 56                          	push	rsi
1405ac61f: 57                          	push	rdi
1405ac620: 53                          	push	rbx
1405ac621: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ac628: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ac62f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ac635: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ac63e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ac647: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ac650: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ac659: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ac662: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ac66b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ac674: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ac67c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ac684: 48 83 e2 80                 	and	rdx, -0x80
1405ac688: 48 89 d3                    	mov	rbx, rdx
1405ac68b: 0f b6 b3 e7 00 00 00        	movzx	esi, byte ptr [rbx + 0xe7]
1405ac692: 0f b6 bb e6 00 00 00        	movzx	edi, byte ptr [rbx + 0xe6]
1405ac699: 44 0f b6 b3 e5 00 00 00     	movzx	r14d, byte ptr [rbx + 0xe5]
1405ac6a1: 44 0f b6 bb e4 00 00 00     	movzx	r15d, byte ptr [rbx + 0xe4]
1405ac6a9: 48 8d 8b 90 02 00 00        	lea	rcx, [rbx + 0x290]
1405ac6b0: e8 6b 95 04 00              	call	0x1405f5c20 <.text+0x5f4c20>
1405ac6b5: 44 88 bb d4 00 00 00        	mov	byte ptr [rbx + 0xd4], r15b
1405ac6bc: 44 88 b3 d5 00 00 00        	mov	byte ptr [rbx + 0xd5], r14b
1405ac6c3: 40 88 bb d6 00 00 00        	mov	byte ptr [rbx + 0xd6], dil
1405ac6ca: c6 83 d7 00 00 00 00        	mov	byte ptr [rbx + 0xd7], 0x0
1405ac6d1: 40 88 b3 d8 00 00 00        	mov	byte ptr [rbx + 0xd8], sil
1405ac6d8: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ac6e0: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ac6e8: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ac6f1: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ac6fa: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ac703: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ac70c: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ac715: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ac71e: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ac727: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ac72d: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ac734: 5b                          	pop	rbx
1405ac735: 5f                          	pop	rdi
1405ac736: 5e                          	pop	rsi
1405ac737: 41 5c                       	pop	r12
1405ac739: 41 5d                       	pop	r13
1405ac73b: 41 5e                       	pop	r14
1405ac73d: 41 5f                       	pop	r15
1405ac73f: 5d                          	pop	rbp
1405ac740: c3                          	ret
1405ac741: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1405ac750: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ac755: 55                          	push	rbp
1405ac756: 41 57                       	push	r15
1405ac758: 41 56                       	push	r14
1405ac75a: 41 55                       	push	r13
1405ac75c: 41 54                       	push	r12
1405ac75e: 56                          	push	rsi
1405ac75f: 57                          	push	rdi
1405ac760: 53                          	push	rbx
1405ac761: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ac768: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ac76f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ac775: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ac77e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ac787: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ac790: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ac799: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ac7a2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ac7ab: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ac7b4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ac7bc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ac7c4: 48 83 e2 80                 	and	rdx, -0x80
1405ac7c8: 48 89 d3                    	mov	rbx, rdx
1405ac7cb: 0f b6 b3 d8 00 00 00        	movzx	esi, byte ptr [rbx + 0xd8]
1405ac7d2: 44 0f b6 b3 d7 00 00 00     	movzx	r14d, byte ptr [rbx + 0xd7]
1405ac7da: 44 0f b6 bb d6 00 00 00     	movzx	r15d, byte ptr [rbx + 0xd6]
1405ac7e2: 44 0f b6 a3 d5 00 00 00     	movzx	r12d, byte ptr [rbx + 0xd5]
1405ac7ea: 0f b6 bb d4 00 00 00        	movzx	edi, byte ptr [rbx + 0xd4]
1405ac7f1: 40 f6 c7 01                 	test	dil, 0x1
1405ac7f5: 0f 84 b6 00 00 00           	je	0x1405ac8b1 <.text+0x5ab8b1>
1405ac7fb: 48 8d 8b 88 21 00 00        	lea	rcx, [rbx + 0x2188]
1405ac802: e8 29 ae 04 00              	call	0x1405f7630 <.text+0x5f6630>
1405ac807: 44 88 a3 b4 00 00 00        	mov	byte ptr [rbx + 0xb4], r12b
1405ac80e: 8b 83 30 01 00 00           	mov	eax, dword ptr [rbx + 0x130]
1405ac814: 88 83 b5 00 00 00           	mov	byte ptr [rbx + 0xb5], al
1405ac81a: 44 88 bb b6 00 00 00        	mov	byte ptr [rbx + 0xb6], r15b
1405ac821: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405ac828: 41 80 e6 01                 	and	r14b, 0x1
1405ac82c: 44 88 b3 b8 00 00 00        	mov	byte ptr [rbx + 0xb8], r14b
1405ac833: c6 83 b9 00 00 00 00        	mov	byte ptr [rbx + 0xb9], 0x0
1405ac83a: 40 88 bb ba 00 00 00        	mov	byte ptr [rbx + 0xba], dil
1405ac841: 40 88 b3 bb 00 00 00        	mov	byte ptr [rbx + 0xbb], sil
1405ac848: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ac850: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ac858: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ac861: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ac86a: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ac873: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ac87c: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ac885: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ac88e: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ac897: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ac89d: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ac8a4: 5b                          	pop	rbx
1405ac8a5: 5f                          	pop	rdi
1405ac8a6: 5e                          	pop	rsi
1405ac8a7: 41 5c                       	pop	r12
1405ac8a9: 41 5d                       	pop	r13
1405ac8ab: 41 5e                       	pop	r14
1405ac8ad: 41 5f                       	pop	r15
1405ac8af: 5d                          	pop	rbp
1405ac8b0: c3                          	ret
1405ac8b1: 44 88 a3 b4 00 00 00        	mov	byte ptr [rbx + 0xb4], r12b
1405ac8b8: 8b 83 30 01 00 00           	mov	eax, dword ptr [rbx + 0x130]
1405ac8be: 88 83 b5 00 00 00           	mov	byte ptr [rbx + 0xb5], al
1405ac8c4: 44 88 bb b6 00 00 00        	mov	byte ptr [rbx + 0xb6], r15b
1405ac8cb: c6 83 b7 00 00 00 00        	mov	byte ptr [rbx + 0xb7], 0x0
1405ac8d2: 41 80 e6 01                 	and	r14b, 0x1
1405ac8d6: 44 88 b3 b8 00 00 00        	mov	byte ptr [rbx + 0xb8], r14b
1405ac8dd: c6 83 b9 00 00 00 00        	mov	byte ptr [rbx + 0xb9], 0x0
1405ac8e4: 40 88 bb ba 00 00 00        	mov	byte ptr [rbx + 0xba], dil
1405ac8eb: 40 88 b3 bb 00 00 00        	mov	byte ptr [rbx + 0xbb], sil
1405ac8f2: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ac8fa: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ac902: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ac90b: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ac914: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ac91d: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ac926: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ac92f: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ac938: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ac941: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ac947: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ac94e: 5b                          	pop	rbx
1405ac94f: 5f                          	pop	rdi
1405ac950: 5e                          	pop	rsi
1405ac951: 41 5c                       	pop	r12
1405ac953: 41 5d                       	pop	r13
1405ac955: 41 5e                       	pop	r14
1405ac957: 41 5f                       	pop	r15
1405ac959: 5d                          	pop	rbp
1405ac95a: c3                          	ret
1405ac95b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1405ac960: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ac965: 55                          	push	rbp
1405ac966: 41 57                       	push	r15
1405ac968: 41 56                       	push	r14
1405ac96a: 41 55                       	push	r13
1405ac96c: 41 54                       	push	r12
1405ac96e: 56                          	push	rsi
1405ac96f: 57                          	push	rdi
1405ac970: 53                          	push	rbx
1405ac971: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ac978: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ac97f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ac985: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ac98e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ac997: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ac9a0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ac9a9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ac9b2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ac9bb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ac9c4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ac9cc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ac9d4: 48 83 e2 80                 	and	rdx, -0x80
1405ac9d8: 48 89 d3                    	mov	rbx, rdx
1405ac9db: 0f b6 83 bb 00 00 00        	movzx	eax, byte ptr [rbx + 0xbb]
1405ac9e2: 88 83 e8 00 00 00           	mov	byte ptr [rbx + 0xe8], al
1405ac9e8: 0f b6 83 ba 00 00 00        	movzx	eax, byte ptr [rbx + 0xba]
1405ac9ef: 88 83 30 01 00 00           	mov	byte ptr [rbx + 0x130], al
1405ac9f5: 44 0f b6 b3 b9 00 00 00     	movzx	r14d, byte ptr [rbx + 0xb9]
1405ac9fd: 44 0f b6 bb b8 00 00 00     	movzx	r15d, byte ptr [rbx + 0xb8]
1405aca05: 0f b6 bb b7 00 00 00        	movzx	edi, byte ptr [rbx + 0xb7]
1405aca0c: 44 0f b6 a3 b6 00 00 00     	movzx	r12d, byte ptr [rbx + 0xb6]
1405aca14: 44 0f b6 ab b5 00 00 00     	movzx	r13d, byte ptr [rbx + 0xb5]
1405aca1c: 0f b6 b3 b4 00 00 00        	movzx	esi, byte ptr [rbx + 0xb4]
1405aca23: 48 8b 83 20 03 00 00        	mov	rax, qword ptr [rbx + 0x320]
1405aca2a: 48 85 c0                    	test	rax, rax
1405aca2d: 74 12                       	je	0x1405aca41 <.text+0x5aba41>
1405aca2f: f0                          	lock
1405aca30: 48 ff 08                    	dec	qword ptr [rax]
1405aca33: 75 0c                       	jne	0x1405aca41 <.text+0x5aba41>
1405aca35: 48 8d 8b 20 03 00 00        	lea	rcx, [rbx + 0x320]
1405aca3c: e8 9f ae 0e 00              	call	0x1406978e0 <.text+0x6968e0>
1405aca41: 40 f6 c7 01                 	test	dil, 0x1
1405aca45: 0f 84 c8 00 00 00           	je	0x1405acb13 <.text+0x5abb13>
1405aca4b: 48 8d 8b 00 03 00 00        	lea	rcx, [rbx + 0x300]
1405aca52: e8 89 bd 05 00              	call	0x1406087e0 <.text+0x6077e0>
1405aca57: 40 88 b3 aa 00 00 00        	mov	byte ptr [rbx + 0xaa], sil
1405aca5e: 44 88 ab ab 00 00 00        	mov	byte ptr [rbx + 0xab], r13b
1405aca65: 44 88 a3 ac 00 00 00        	mov	byte ptr [rbx + 0xac], r12b
1405aca6c: 41 80 e7 01                 	and	r15b, 0x1
1405aca70: 44 88 bb ad 00 00 00        	mov	byte ptr [rbx + 0xad], r15b
1405aca77: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405aca7e: 41 80 e6 01                 	and	r14b, 0x1
1405aca82: 44 88 b3 af 00 00 00        	mov	byte ptr [rbx + 0xaf], r14b
1405aca89: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405aca90: 0f b6 83 30 01 00 00        	movzx	eax, byte ptr [rbx + 0x130]
1405aca97: 88 83 b1 00 00 00           	mov	byte ptr [rbx + 0xb1], al
1405aca9d: 0f b6 83 e8 00 00 00        	movzx	eax, byte ptr [rbx + 0xe8]
1405acaa4: 88 83 b2 00 00 00           	mov	byte ptr [rbx + 0xb2], al
1405acaaa: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405acab2: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405acaba: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405acac3: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405acacc: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405acad5: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405acade: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405acae7: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405acaf0: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405acaf9: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405acaff: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405acb06: 5b                          	pop	rbx
1405acb07: 5f                          	pop	rdi
1405acb08: 5e                          	pop	rsi
1405acb09: 41 5c                       	pop	r12
1405acb0b: 41 5d                       	pop	r13
1405acb0d: 41 5e                       	pop	r14
1405acb0f: 41 5f                       	pop	r15
1405acb11: 5d                          	pop	rbp
1405acb12: c3                          	ret
1405acb13: 40 88 b3 aa 00 00 00        	mov	byte ptr [rbx + 0xaa], sil
1405acb1a: 44 88 ab ab 00 00 00        	mov	byte ptr [rbx + 0xab], r13b
1405acb21: 44 88 a3 ac 00 00 00        	mov	byte ptr [rbx + 0xac], r12b
1405acb28: 41 80 e7 01                 	and	r15b, 0x1
1405acb2c: 44 88 bb ad 00 00 00        	mov	byte ptr [rbx + 0xad], r15b
1405acb33: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405acb3a: 41 80 e6 01                 	and	r14b, 0x1
1405acb3e: 44 88 b3 af 00 00 00        	mov	byte ptr [rbx + 0xaf], r14b
1405acb45: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405acb4c: 0f b6 83 30 01 00 00        	movzx	eax, byte ptr [rbx + 0x130]
1405acb53: 88 83 b1 00 00 00           	mov	byte ptr [rbx + 0xb1], al
1405acb59: 0f b6 83 e8 00 00 00        	movzx	eax, byte ptr [rbx + 0xe8]
1405acb60: 88 83 b2 00 00 00           	mov	byte ptr [rbx + 0xb2], al
1405acb66: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405acb6e: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405acb76: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405acb7f: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405acb88: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405acb91: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405acb9a: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405acba3: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405acbac: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405acbb5: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405acbbb: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405acbc2: 5b                          	pop	rbx
1405acbc3: 5f                          	pop	rdi
1405acbc4: 5e                          	pop	rsi
1405acbc5: 41 5c                       	pop	r12
1405acbc7: 41 5d                       	pop	r13
1405acbc9: 41 5e                       	pop	r14
1405acbcb: 41 5f                       	pop	r15
1405acbcd: 5d                          	pop	rbp
1405acbce: c3                          	ret
1405acbcf: 90                          	nop
1405acbd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405acbd5: 55                          	push	rbp
1405acbd6: 41 57                       	push	r15
1405acbd8: 41 56                       	push	r14
1405acbda: 41 55                       	push	r13
1405acbdc: 41 54                       	push	r12
1405acbde: 56                          	push	rsi
1405acbdf: 57                          	push	rdi
1405acbe0: 53                          	push	rbx
1405acbe1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405acbe8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405acbef: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405acbf5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405acbfe: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405acc07: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405acc10: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405acc19: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405acc22: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405acc2b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405acc34: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405acc3c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405acc44: 48 83 e2 80                 	and	rdx, -0x80
1405acc48: 48 89 d3                    	mov	rbx, rdx
1405acc4b: 0f b6 8b b2 00 00 00        	movzx	ecx, byte ptr [rbx + 0xb2]
1405acc52: 0f b6 93 b1 00 00 00        	movzx	edx, byte ptr [rbx + 0xb1]
1405acc59: 44 0f b6 b3 b0 00 00 00     	movzx	r14d, byte ptr [rbx + 0xb0]
1405acc61: 44 0f b6 bb af 00 00 00     	movzx	r15d, byte ptr [rbx + 0xaf]
1405acc69: 44 0f b6 a3 ae 00 00 00     	movzx	r12d, byte ptr [rbx + 0xae]
1405acc71: 44 0f b6 ab ac 00 00 00     	movzx	r13d, byte ptr [rbx + 0xac]
1405acc79: 0f b6 b3 ab 00 00 00        	movzx	esi, byte ptr [rbx + 0xab]
1405acc80: 0f b6 bb aa 00 00 00        	movzx	edi, byte ptr [rbx + 0xaa]
1405acc87: 80 bb ad 00 00 00 00        	cmp	byte ptr [rbx + 0xad], 0x0
1405acc8e: 0f 84 de 00 00 00           	je	0x1405acd72 <.text+0x5abd72>
1405acc94: 48 8b 83 58 01 00 00        	mov	rax, qword ptr [rbx + 0x158]
1405acc9b: f0                          	lock
1405acc9c: 48 ff 08                    	dec	qword ptr [rax]
1405acc9f: 75 26                       	jne	0x1405accc7 <.text+0x5abcc7>
1405acca1: 88 8b e8 00 00 00           	mov	byte ptr [rbx + 0xe8], cl
1405acca7: 48 8d 8b 58 01 00 00        	lea	rcx, [rbx + 0x158]
1405accae: 88 93 30 01 00 00           	mov	byte ptr [rbx + 0x130], dl
1405accb4: e8 e7 fa 5a 00              	call	0x140b5c7a0 <.text+0xb5b7a0>
1405accb9: 0f b6 93 30 01 00 00        	movzx	edx, byte ptr [rbx + 0x130]
1405accc0: 0f b6 8b e8 00 00 00        	movzx	ecx, byte ptr [rbx + 0xe8]
1405accc7: 40 88 bb 08 01 00 00        	mov	byte ptr [rbx + 0x108], dil
1405accce: 40 88 b3 09 01 00 00        	mov	byte ptr [rbx + 0x109], sil
1405accd5: 44 88 ab 0a 01 00 00        	mov	byte ptr [rbx + 0x10a], r13b
1405accdc: 41 80 e4 01                 	and	r12b, 0x1
1405acce0: 44 88 a3 0b 01 00 00        	mov	byte ptr [rbx + 0x10b], r12b
1405acce7: 41 80 e7 01                 	and	r15b, 0x1
1405acceb: 44 88 bb 0c 01 00 00        	mov	byte ptr [rbx + 0x10c], r15b
1405accf2: 41 80 e6 01                 	and	r14b, 0x1
1405accf6: 44 88 b3 0d 01 00 00        	mov	byte ptr [rbx + 0x10d], r14b
1405accfd: 88 93 0e 01 00 00           	mov	byte ptr [rbx + 0x10e], dl
1405acd03: 88 8b 0f 01 00 00           	mov	byte ptr [rbx + 0x10f], cl
1405acd09: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405acd11: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405acd19: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405acd22: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405acd2b: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405acd34: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405acd3d: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405acd46: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405acd4f: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405acd58: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405acd5e: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405acd65: 5b                          	pop	rbx
1405acd66: 5f                          	pop	rdi
1405acd67: 5e                          	pop	rsi
1405acd68: 41 5c                       	pop	r12
1405acd6a: 41 5d                       	pop	r13
1405acd6c: 41 5e                       	pop	r14
1405acd6e: 41 5f                       	pop	r15
1405acd70: 5d                          	pop	rbp
1405acd71: c3                          	ret
1405acd72: 40 88 bb 08 01 00 00        	mov	byte ptr [rbx + 0x108], dil
1405acd79: 40 88 b3 09 01 00 00        	mov	byte ptr [rbx + 0x109], sil
1405acd80: 44 88 ab 0a 01 00 00        	mov	byte ptr [rbx + 0x10a], r13b
1405acd87: 41 80 e4 01                 	and	r12b, 0x1
1405acd8b: 44 88 a3 0b 01 00 00        	mov	byte ptr [rbx + 0x10b], r12b
1405acd92: 41 80 e7 01                 	and	r15b, 0x1
1405acd96: 44 88 bb 0c 01 00 00        	mov	byte ptr [rbx + 0x10c], r15b
1405acd9d: 41 80 e6 01                 	and	r14b, 0x1
1405acda1: 44 88 b3 0d 01 00 00        	mov	byte ptr [rbx + 0x10d], r14b
1405acda8: 88 93 0e 01 00 00           	mov	byte ptr [rbx + 0x10e], dl
1405acdae: 88 8b 0f 01 00 00           	mov	byte ptr [rbx + 0x10f], cl
1405acdb4: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405acdbc: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405acdc4: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405acdcd: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405acdd6: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405acddf: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405acde8: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405acdf1: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405acdfa: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ace03: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ace09: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ace10: 5b                          	pop	rbx
1405ace11: 5f                          	pop	rdi
1405ace12: 5e                          	pop	rsi
1405ace13: 41 5c                       	pop	r12
1405ace15: 41 5d                       	pop	r13
1405ace17: 41 5e                       	pop	r14
1405ace19: 41 5f                       	pop	r15
1405ace1b: 5d                          	pop	rbp
1405ace1c: c3                          	ret
1405ace1d: 0f 1f 00                    	nop	dword ptr [rax]
1405ace20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ace25: 55                          	push	rbp
1405ace26: 41 57                       	push	r15
1405ace28: 41 56                       	push	r14
1405ace2a: 41 55                       	push	r13
1405ace2c: 41 54                       	push	r12
1405ace2e: 56                          	push	rsi
1405ace2f: 57                          	push	rdi
1405ace30: 53                          	push	rbx
1405ace31: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ace38: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ace3f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ace45: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ace4e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ace57: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ace60: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ace69: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ace72: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ace7b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ace84: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ace8c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ace94: 48 83 e2 80                 	and	rdx, -0x80
1405ace98: 48 89 d3                    	mov	rbx, rdx
1405ace9b: 0f b6 83 0f 01 00 00        	movzx	eax, byte ptr [rbx + 0x10f]
1405acea2: 88 83 e8 00 00 00           	mov	byte ptr [rbx + 0xe8], al
1405acea8: 0f b6 bb 0e 01 00 00        	movzx	edi, byte ptr [rbx + 0x10e]
1405aceaf: 44 0f b6 b3 0d 01 00 00     	movzx	r14d, byte ptr [rbx + 0x10d]
1405aceb7: 44 0f b6 bb 0c 01 00 00     	movzx	r15d, byte ptr [rbx + 0x10c]
1405acebf: 44 0f b6 a3 0a 01 00 00     	movzx	r12d, byte ptr [rbx + 0x10a]
1405acec7: 44 0f b6 ab 09 01 00 00     	movzx	r13d, byte ptr [rbx + 0x109]
1405acecf: 0f b6 b3 08 01 00 00        	movzx	esi, byte ptr [rbx + 0x108]
1405aced6: 80 bb 0b 01 00 00 00        	cmp	byte ptr [rbx + 0x10b], 0x0
1405acedd: 0f 84 b4 00 00 00           	je	0x1405acf97 <.text+0x5abf97>
1405acee3: 48 8d 8b 20 32 00 00        	lea	rcx, [rbx + 0x3220]
1405aceea: e8 f1 cd 04 00              	call	0x1405f9ce0 <.text+0x5f8ce0>
1405aceef: 40 88 b3 dc 00 00 00        	mov	byte ptr [rbx + 0xdc], sil
1405acef6: 44 88 ab dd 00 00 00        	mov	byte ptr [rbx + 0xdd], r13b
1405acefd: 44 88 a3 de 00 00 00        	mov	byte ptr [rbx + 0xde], r12b
1405acf04: 41 80 e7 01                 	and	r15b, 0x1
1405acf08: 44 88 bb df 00 00 00        	mov	byte ptr [rbx + 0xdf], r15b
1405acf0f: 41 80 e6 01                 	and	r14b, 0x1
1405acf13: 44 88 b3 e0 00 00 00        	mov	byte ptr [rbx + 0xe0], r14b
1405acf1a: 40 88 bb e1 00 00 00        	mov	byte ptr [rbx + 0xe1], dil
1405acf21: 0f b6 83 e8 00 00 00        	movzx	eax, byte ptr [rbx + 0xe8]
1405acf28: 88 83 e2 00 00 00           	mov	byte ptr [rbx + 0xe2], al
1405acf2e: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405acf36: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405acf3e: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405acf47: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405acf50: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405acf59: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405acf62: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405acf6b: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405acf74: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405acf7d: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405acf83: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405acf8a: 5b                          	pop	rbx
1405acf8b: 5f                          	pop	rdi
1405acf8c: 5e                          	pop	rsi
1405acf8d: 41 5c                       	pop	r12
1405acf8f: 41 5d                       	pop	r13
1405acf91: 41 5e                       	pop	r14
1405acf93: 41 5f                       	pop	r15
1405acf95: 5d                          	pop	rbp
1405acf96: c3                          	ret
1405acf97: 40 88 b3 dc 00 00 00        	mov	byte ptr [rbx + 0xdc], sil
1405acf9e: 44 88 ab dd 00 00 00        	mov	byte ptr [rbx + 0xdd], r13b
1405acfa5: 44 88 a3 de 00 00 00        	mov	byte ptr [rbx + 0xde], r12b
1405acfac: 41 80 e7 01                 	and	r15b, 0x1
1405acfb0: 44 88 bb df 00 00 00        	mov	byte ptr [rbx + 0xdf], r15b
1405acfb7: 41 80 e6 01                 	and	r14b, 0x1
1405acfbb: 44 88 b3 e0 00 00 00        	mov	byte ptr [rbx + 0xe0], r14b
1405acfc2: 40 88 bb e1 00 00 00        	mov	byte ptr [rbx + 0xe1], dil
1405acfc9: 0f b6 83 e8 00 00 00        	movzx	eax, byte ptr [rbx + 0xe8]
1405acfd0: 88 83 e2 00 00 00           	mov	byte ptr [rbx + 0xe2], al
1405acfd6: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405acfde: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405acfe6: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405acfef: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405acff8: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ad001: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ad00a: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ad013: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ad01c: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ad025: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ad02b: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ad032: 5b                          	pop	rbx
1405ad033: 5f                          	pop	rdi
1405ad034: 5e                          	pop	rsi
1405ad035: 41 5c                       	pop	r12
1405ad037: 41 5d                       	pop	r13
1405ad039: 41 5e                       	pop	r14
1405ad03b: 41 5f                       	pop	r15
1405ad03d: 5d                          	pop	rbp
1405ad03e: c3                          	ret
1405ad03f: 90                          	nop
1405ad040: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ad045: 55                          	push	rbp
1405ad046: 41 57                       	push	r15
1405ad048: 41 56                       	push	r14
1405ad04a: 41 55                       	push	r13
1405ad04c: 41 54                       	push	r12
1405ad04e: 56                          	push	rsi
1405ad04f: 57                          	push	rdi
1405ad050: 53                          	push	rbx
1405ad051: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ad058: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ad05f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ad065: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ad06e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ad077: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ad080: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ad089: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ad092: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ad09b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ad0a4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ad0ac: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ad0b4: 48 83 e2 80                 	and	rdx, -0x80
1405ad0b8: 48 89 d3                    	mov	rbx, rdx
1405ad0bb: 0f b6 b3 e2 00 00 00        	movzx	esi, byte ptr [rbx + 0xe2]
1405ad0c2: 0f b6 83 e1 00 00 00        	movzx	eax, byte ptr [rbx + 0xe1]
1405ad0c9: 88 83 e8 00 00 00           	mov	byte ptr [rbx + 0xe8], al
1405ad0cf: 44 0f b6 b3 e0 00 00 00     	movzx	r14d, byte ptr [rbx + 0xe0]
1405ad0d7: 44 0f b6 bb df 00 00 00     	movzx	r15d, byte ptr [rbx + 0xdf]
1405ad0df: 44 0f b6 a3 de 00 00 00     	movzx	r12d, byte ptr [rbx + 0xde]
1405ad0e7: 44 0f b6 ab dd 00 00 00     	movzx	r13d, byte ptr [rbx + 0xdd]
1405ad0ef: 0f b6 bb dc 00 00 00        	movzx	edi, byte ptr [rbx + 0xdc]
1405ad0f6: 40 f6 c6 01                 	test	sil, 0x1
1405ad0fa: 0f 84 b4 00 00 00           	je	0x1405ad1b4 <.text+0x5ac1b4>
1405ad100: 48 8d 8b e0 20 00 00        	lea	rcx, [rbx + 0x20e0]
1405ad107: e8 e4 66 04 00              	call	0x1405f37f0 <.text+0x5f27f0>
1405ad10c: 40 88 bb fe 00 00 00        	mov	byte ptr [rbx + 0xfe], dil
1405ad113: 44 88 ab ff 00 00 00        	mov	byte ptr [rbx + 0xff], r13b
1405ad11a: 44 88 a3 00 01 00 00        	mov	byte ptr [rbx + 0x100], r12b
1405ad121: 41 80 e7 01                 	and	r15b, 0x1
1405ad125: 44 88 bb 01 01 00 00        	mov	byte ptr [rbx + 0x101], r15b
1405ad12c: 41 80 e6 01                 	and	r14b, 0x1
1405ad130: 44 88 b3 02 01 00 00        	mov	byte ptr [rbx + 0x102], r14b
1405ad137: 0f b6 83 e8 00 00 00        	movzx	eax, byte ptr [rbx + 0xe8]
1405ad13e: 88 83 03 01 00 00           	mov	byte ptr [rbx + 0x103], al
1405ad144: 40 88 b3 04 01 00 00        	mov	byte ptr [rbx + 0x104], sil
1405ad14b: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ad153: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ad15b: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ad164: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ad16d: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ad176: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ad17f: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ad188: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ad191: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ad19a: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ad1a0: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ad1a7: 5b                          	pop	rbx
1405ad1a8: 5f                          	pop	rdi
1405ad1a9: 5e                          	pop	rsi
1405ad1aa: 41 5c                       	pop	r12
1405ad1ac: 41 5d                       	pop	r13
1405ad1ae: 41 5e                       	pop	r14
1405ad1b0: 41 5f                       	pop	r15
1405ad1b2: 5d                          	pop	rbp
1405ad1b3: c3                          	ret
1405ad1b4: 40 88 bb fe 00 00 00        	mov	byte ptr [rbx + 0xfe], dil
1405ad1bb: 44 88 ab ff 00 00 00        	mov	byte ptr [rbx + 0xff], r13b
1405ad1c2: 44 88 a3 00 01 00 00        	mov	byte ptr [rbx + 0x100], r12b
1405ad1c9: 41 80 e7 01                 	and	r15b, 0x1
1405ad1cd: 44 88 bb 01 01 00 00        	mov	byte ptr [rbx + 0x101], r15b
1405ad1d4: 41 80 e6 01                 	and	r14b, 0x1
1405ad1d8: 44 88 b3 02 01 00 00        	mov	byte ptr [rbx + 0x102], r14b
1405ad1df: 0f b6 83 e8 00 00 00        	movzx	eax, byte ptr [rbx + 0xe8]
1405ad1e6: 88 83 03 01 00 00           	mov	byte ptr [rbx + 0x103], al
1405ad1ec: 40 88 b3 04 01 00 00        	mov	byte ptr [rbx + 0x104], sil
1405ad1f3: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ad1fb: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ad203: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ad20c: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ad215: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ad21e: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ad227: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ad230: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ad239: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ad242: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ad248: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ad24f: 5b                          	pop	rbx
1405ad250: 5f                          	pop	rdi
1405ad251: 5e                          	pop	rsi
1405ad252: 41 5c                       	pop	r12
1405ad254: 41 5d                       	pop	r13
1405ad256: 41 5e                       	pop	r14
1405ad258: 41 5f                       	pop	r15
1405ad25a: 5d                          	pop	rbp
1405ad25b: c3                          	ret
1405ad25c: 0f 1f 40 00                 	nop	dword ptr [rax]
1405ad260: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ad265: 55                          	push	rbp
1405ad266: 41 57                       	push	r15
1405ad268: 41 56                       	push	r14
1405ad26a: 41 55                       	push	r13
1405ad26c: 41 54                       	push	r12
1405ad26e: 56                          	push	rsi
1405ad26f: 57                          	push	rdi
1405ad270: 53                          	push	rbx
1405ad271: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ad278: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ad27f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ad285: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ad28e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ad297: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ad2a0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ad2a9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ad2b2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ad2bb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ad2c4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ad2cc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ad2d4: 48 83 e2 80                 	and	rdx, -0x80
1405ad2d8: 48 89 d3                    	mov	rbx, rdx
1405ad2db: 0f b6 b3 04 01 00 00        	movzx	esi, byte ptr [rbx + 0x104]
1405ad2e2: 0f b6 bb 03 01 00 00        	movzx	edi, byte ptr [rbx + 0x103]
1405ad2e9: 44 0f b6 b3 02 01 00 00     	movzx	r14d, byte ptr [rbx + 0x102]
1405ad2f1: 44 0f b6 bb 00 01 00 00     	movzx	r15d, byte ptr [rbx + 0x100]
1405ad2f9: 44 0f b6 a3 ff 00 00 00     	movzx	r12d, byte ptr [rbx + 0xff]
1405ad301: 44 0f b6 ab fe 00 00 00     	movzx	r13d, byte ptr [rbx + 0xfe]
1405ad309: 80 bb 01 01 00 00 00        	cmp	byte ptr [rbx + 0x101], 0x0
1405ad310: 0f 84 a3 00 00 00           	je	0x1405ad3b9 <.text+0x5ac3b9>
1405ad316: 48 8d 8b a0 0a 00 00        	lea	rcx, [rbx + 0xaa0]
1405ad31d: e8 4e 94 04 00              	call	0x1405f6770 <.text+0x5f5770>
1405ad322: 44 88 ab bc 00 00 00        	mov	byte ptr [rbx + 0xbc], r13b
1405ad329: 44 88 a3 bd 00 00 00        	mov	byte ptr [rbx + 0xbd], r12b
1405ad330: 44 88 bb be 00 00 00        	mov	byte ptr [rbx + 0xbe], r15b
1405ad337: 41 80 e6 01                 	and	r14b, 0x1
1405ad33b: 44 88 b3 bf 00 00 00        	mov	byte ptr [rbx + 0xbf], r14b
1405ad342: 40 88 bb c0 00 00 00        	mov	byte ptr [rbx + 0xc0], dil
1405ad349: 40 88 b3 c1 00 00 00        	mov	byte ptr [rbx + 0xc1], sil
1405ad350: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ad358: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ad360: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ad369: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ad372: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ad37b: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ad384: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ad38d: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ad396: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ad39f: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ad3a5: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ad3ac: 5b                          	pop	rbx
1405ad3ad: 5f                          	pop	rdi
1405ad3ae: 5e                          	pop	rsi
1405ad3af: 41 5c                       	pop	r12
1405ad3b1: 41 5d                       	pop	r13
1405ad3b3: 41 5e                       	pop	r14
1405ad3b5: 41 5f                       	pop	r15
1405ad3b7: 5d                          	pop	rbp
1405ad3b8: c3                          	ret
1405ad3b9: 44 88 ab bc 00 00 00        	mov	byte ptr [rbx + 0xbc], r13b
1405ad3c0: 44 88 a3 bd 00 00 00        	mov	byte ptr [rbx + 0xbd], r12b
1405ad3c7: 44 88 bb be 00 00 00        	mov	byte ptr [rbx + 0xbe], r15b
1405ad3ce: 41 80 e6 01                 	and	r14b, 0x1
1405ad3d2: 44 88 b3 bf 00 00 00        	mov	byte ptr [rbx + 0xbf], r14b
1405ad3d9: 40 88 bb c0 00 00 00        	mov	byte ptr [rbx + 0xc0], dil
1405ad3e0: 40 88 b3 c1 00 00 00        	mov	byte ptr [rbx + 0xc1], sil
1405ad3e7: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ad3ef: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ad3f7: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ad400: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ad409: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ad412: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ad41b: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ad424: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ad42d: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ad436: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ad43c: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ad443: 5b                          	pop	rbx
1405ad444: 5f                          	pop	rdi
1405ad445: 5e                          	pop	rsi
1405ad446: 41 5c                       	pop	r12
1405ad448: 41 5d                       	pop	r13
1405ad44a: 41 5e                       	pop	r14
1405ad44c: 41 5f                       	pop	r15
1405ad44e: 5d                          	pop	rbp
1405ad44f: c3                          	ret
1405ad450: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ad455: 55                          	push	rbp
1405ad456: 41 57                       	push	r15
1405ad458: 41 56                       	push	r14
1405ad45a: 41 55                       	push	r13
1405ad45c: 41 54                       	push	r12
1405ad45e: 56                          	push	rsi
1405ad45f: 57                          	push	rdi
1405ad460: 53                          	push	rbx
1405ad461: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ad468: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ad46f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ad475: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ad47e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ad487: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ad490: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ad499: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ad4a2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ad4ab: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ad4b4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ad4bc: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ad4c4: 48 83 e2 80                 	and	rdx, -0x80
1405ad4c8: 48 89 d3                    	mov	rbx, rdx
1405ad4cb: 0f b6 b3 c1 00 00 00        	movzx	esi, byte ptr [rbx + 0xc1]
1405ad4d2: 0f b6 bb c0 00 00 00        	movzx	edi, byte ptr [rbx + 0xc0]
1405ad4d9: 44 0f b6 b3 bf 00 00 00     	movzx	r14d, byte ptr [rbx + 0xbf]
1405ad4e1: 44 0f b6 bb be 00 00 00     	movzx	r15d, byte ptr [rbx + 0xbe]
1405ad4e9: 44 0f b6 a3 bd 00 00 00     	movzx	r12d, byte ptr [rbx + 0xbd]
1405ad4f1: 44 0f b6 ab bc 00 00 00     	movzx	r13d, byte ptr [rbx + 0xbc]
1405ad4f9: 48 8b 83 38 01 00 00        	mov	rax, qword ptr [rbx + 0x138]
1405ad500: f0                          	lock
1405ad501: 48 ff 08                    	dec	qword ptr [rax]
1405ad504: 75 0c                       	jne	0x1405ad512 <.text+0x5ac512>
1405ad506: 48 8d 8b 38 01 00 00        	lea	rcx, [rbx + 0x138]
1405ad50d: e8 7e e9 5a 00              	call	0x140b5be90 <.text+0xb5ae90>
1405ad512: 44 88 ab f8 00 00 00        	mov	byte ptr [rbx + 0xf8], r13b
1405ad519: 44 88 a3 f9 00 00 00        	mov	byte ptr [rbx + 0xf9], r12b
1405ad520: 44 88 bb fa 00 00 00        	mov	byte ptr [rbx + 0xfa], r15b
1405ad527: 41 80 e6 01                 	and	r14b, 0x1
1405ad52b: 44 88 b3 fb 00 00 00        	mov	byte ptr [rbx + 0xfb], r14b
1405ad532: 40 88 bb fc 00 00 00        	mov	byte ptr [rbx + 0xfc], dil
1405ad539: 40 88 b3 fd 00 00 00        	mov	byte ptr [rbx + 0xfd], sil
1405ad540: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ad548: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ad550: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ad559: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ad562: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ad56b: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ad574: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ad57d: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ad586: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ad58f: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ad595: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ad59c: 5b                          	pop	rbx
1405ad59d: 5f                          	pop	rdi
1405ad59e: 5e                          	pop	rsi
1405ad59f: 41 5c                       	pop	r12
1405ad5a1: 41 5d                       	pop	r13
1405ad5a3: 41 5e                       	pop	r14
1405ad5a5: 41 5f                       	pop	r15
1405ad5a7: 5d                          	pop	rbp
1405ad5a8: c3                          	ret
1405ad5a9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1405ad5b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ad5b5: 55                          	push	rbp
1405ad5b6: 41 57                       	push	r15
1405ad5b8: 41 56                       	push	r14
1405ad5ba: 41 55                       	push	r13
1405ad5bc: 41 54                       	push	r12
1405ad5be: 56                          	push	rsi
1405ad5bf: 57                          	push	rdi
1405ad5c0: 53                          	push	rbx
1405ad5c1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ad5c8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ad5cf: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ad5d5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ad5de: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ad5e7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ad5f0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ad5f9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ad602: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ad60b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ad614: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ad61c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ad624: 48 83 e2 80                 	and	rdx, -0x80
1405ad628: 48 89 d3                    	mov	rbx, rdx
1405ad62b: 0f b6 b3 fd 00 00 00        	movzx	esi, byte ptr [rbx + 0xfd]
1405ad632: 0f b6 bb fc 00 00 00        	movzx	edi, byte ptr [rbx + 0xfc]
1405ad639: 44 0f b6 bb fa 00 00 00     	movzx	r15d, byte ptr [rbx + 0xfa]
1405ad641: 44 0f b6 b3 f9 00 00 00     	movzx	r14d, byte ptr [rbx + 0xf9]
1405ad649: 44 0f b6 a3 f8 00 00 00     	movzx	r12d, byte ptr [rbx + 0xf8]
1405ad651: 80 bb fb 00 00 00 00        	cmp	byte ptr [rbx + 0xfb], 0x0
1405ad658: 0f 84 bf 00 00 00           	je	0x1405ad71d <.text+0x5ac71d>
1405ad65e: 48 8b 8b b8 02 00 00        	mov	rcx, qword ptr [rbx + 0x2b8]
1405ad665: 48 8b 93 b0 02 00 00        	mov	rdx, qword ptr [rbx + 0x2b0]
1405ad66c: e8 ff 52 fe ff              	call	0x140592970 <.text+0x591970>
1405ad671: 40 f6 c6 01                 	test	sil, 0x1
1405ad675: 0f 85 ac 00 00 00           	jne	0x1405ad727 <.text+0x5ac727>
1405ad67b: 41 f6 c7 01                 	test	r15b, 0x1
1405ad67f: 0f 84 b8 00 00 00           	je	0x1405ad73d <.text+0x5ac73d>
1405ad685: 48 8d 8b 60 01 00 00        	lea	rcx, [rbx + 0x160]
1405ad68c: e8 df 91 05 00              	call	0x140606870 <.text+0x605870>
1405ad691: 44 88 bb 15 01 00 00        	mov	byte ptr [rbx + 0x115], r15b
1405ad698: 44 88 a3 16 01 00 00        	mov	byte ptr [rbx + 0x116], r12b
1405ad69f: 44 88 b3 17 01 00 00        	mov	byte ptr [rbx + 0x117], r14b
1405ad6a6: 40 88 bb 18 01 00 00        	mov	byte ptr [rbx + 0x118], dil
1405ad6ad: 40 88 b3 19 01 00 00        	mov	byte ptr [rbx + 0x119], sil
1405ad6b4: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ad6bc: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ad6c4: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ad6cd: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ad6d6: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ad6df: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ad6e8: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ad6f1: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ad6fa: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ad703: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ad709: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ad710: 5b                          	pop	rbx
1405ad711: 5f                          	pop	rdi
1405ad712: 5e                          	pop	rsi
1405ad713: 41 5c                       	pop	r12
1405ad715: 41 5d                       	pop	r13
1405ad717: 41 5e                       	pop	r14
1405ad719: 41 5f                       	pop	r15
1405ad71b: 5d                          	pop	rbp
1405ad71c: c3                          	ret
1405ad71d: 40 f6 c6 01                 	test	sil, 0x1
1405ad721: 0f 84 54 ff ff ff           	je	0x1405ad67b <.text+0x5ac67b>
1405ad727: 48 8d 8b 50 03 00 00        	lea	rcx, [rbx + 0x350]
1405ad72e: e8 7d 4d 63 00              	call	0x140be24b0 <.text+0xbe14b0>
1405ad733: 41 f6 c7 01                 	test	r15b, 0x1
1405ad737: 0f 85 48 ff ff ff           	jne	0x1405ad685 <.text+0x5ac685>
1405ad73d: 44 88 bb 15 01 00 00        	mov	byte ptr [rbx + 0x115], r15b
1405ad744: 44 88 a3 16 01 00 00        	mov	byte ptr [rbx + 0x116], r12b
1405ad74b: 44 88 b3 17 01 00 00        	mov	byte ptr [rbx + 0x117], r14b
1405ad752: 40 88 bb 18 01 00 00        	mov	byte ptr [rbx + 0x118], dil
1405ad759: 40 88 b3 19 01 00 00        	mov	byte ptr [rbx + 0x119], sil
1405ad760: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ad768: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ad770: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ad779: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ad782: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ad78b: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ad794: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ad79d: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ad7a6: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ad7af: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ad7b5: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ad7bc: 5b                          	pop	rbx
1405ad7bd: 5f                          	pop	rdi
1405ad7be: 5e                          	pop	rsi
1405ad7bf: 41 5c                       	pop	r12
1405ad7c1: 41 5d                       	pop	r13
1405ad7c3: 41 5e                       	pop	r14
1405ad7c5: 41 5f                       	pop	r15
1405ad7c7: 5d                          	pop	rbp
1405ad7c8: c3                          	ret
1405ad7c9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1405ad7d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ad7d5: 55                          	push	rbp
1405ad7d6: 41 57                       	push	r15
1405ad7d8: 41 56                       	push	r14
1405ad7da: 41 55                       	push	r13
1405ad7dc: 41 54                       	push	r12
1405ad7de: 56                          	push	rsi
1405ad7df: 57                          	push	rdi
1405ad7e0: 53                          	push	rbx
1405ad7e1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ad7e8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ad7ef: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ad7f5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ad7fe: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ad807: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ad810: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ad819: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ad822: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ad82b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ad834: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ad83c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ad844: 48 83 e2 80                 	and	rdx, -0x80
1405ad848: 48 89 d3                    	mov	rbx, rdx
1405ad84b: 0f b6 b3 19 01 00 00        	movzx	esi, byte ptr [rbx + 0x119]
1405ad852: 0f b6 bb 17 01 00 00        	movzx	edi, byte ptr [rbx + 0x117]
1405ad859: 44 0f b6 b3 16 01 00 00     	movzx	r14d, byte ptr [rbx + 0x116]
1405ad861: 44 0f b6 bb 15 01 00 00     	movzx	r15d, byte ptr [rbx + 0x115]
1405ad869: f6 83 18 01 00 00 01        	test	byte ptr [rbx + 0x118], 0x1
1405ad870: 74 0c                       	je	0x1405ad87e <.text+0x5ac87e>
1405ad872: 48 8d 8b 28 03 00 00        	lea	rcx, [rbx + 0x328]
1405ad879: e8 42 ca 05 00              	call	0x14060a2c0 <.text+0x6092c0>
1405ad87e: 48 8b 8d c0 37 00 00        	mov	rcx, qword ptr [rbp + 0x37c0]
1405ad885: e8 f6 10 04 00              	call	0x1405ee980 <.text+0x5ed980>
1405ad88a: 44 88 bb 20 01 00 00        	mov	byte ptr [rbx + 0x120], r15b
1405ad891: 44 88 b3 21 01 00 00        	mov	byte ptr [rbx + 0x121], r14b
1405ad898: 40 88 bb 22 01 00 00        	mov	byte ptr [rbx + 0x122], dil
1405ad89f: 40 88 b3 23 01 00 00        	mov	byte ptr [rbx + 0x123], sil
1405ad8a6: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ad8ae: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ad8b6: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ad8bf: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ad8c8: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ad8d1: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ad8da: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ad8e3: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ad8ec: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ad8f5: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ad8fb: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ad902: 5b                          	pop	rbx
1405ad903: 5f                          	pop	rdi
1405ad904: 5e                          	pop	rsi
1405ad905: 41 5c                       	pop	r12
1405ad907: 41 5d                       	pop	r13
1405ad909: 41 5e                       	pop	r14
1405ad90b: 41 5f                       	pop	r15
1405ad90d: 5d                          	pop	rbp
1405ad90e: c3                          	ret
1405ad90f: 90                          	nop
1405ad910: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ad915: 55                          	push	rbp
1405ad916: 41 57                       	push	r15
1405ad918: 41 56                       	push	r14
1405ad91a: 41 55                       	push	r13
1405ad91c: 41 54                       	push	r12
1405ad91e: 56                          	push	rsi
1405ad91f: 57                          	push	rdi
1405ad920: 53                          	push	rbx
1405ad921: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ad928: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ad92f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ad935: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ad93e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ad947: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ad950: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ad959: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ad962: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ad96b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ad974: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ad97c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ad984: 48 83 e2 80                 	and	rdx, -0x80
1405ad988: 48 89 d3                    	mov	rbx, rdx
1405ad98b: 0f b6 b3 23 01 00 00        	movzx	esi, byte ptr [rbx + 0x123]
1405ad992: 0f b6 bb 22 01 00 00        	movzx	edi, byte ptr [rbx + 0x122]
1405ad999: 44 0f b6 b3 21 01 00 00     	movzx	r14d, byte ptr [rbx + 0x121]
1405ad9a1: 44 0f b6 bb 20 01 00 00     	movzx	r15d, byte ptr [rbx + 0x120]
1405ad9a9: 48 8b 8d b8 37 00 00        	mov	rcx, qword ptr [rbp + 0x37b8]
1405ad9b0: e8 0b 3f 04 00              	call	0x1405f18c0 <.text+0x5f08c0>
1405ad9b5: 44 88 bb 24 01 00 00        	mov	byte ptr [rbx + 0x124], r15b
1405ad9bc: 44 88 b3 25 01 00 00        	mov	byte ptr [rbx + 0x125], r14b
1405ad9c3: 40 88 bb 26 01 00 00        	mov	byte ptr [rbx + 0x126], dil
1405ad9ca: 40 88 b3 27 01 00 00        	mov	byte ptr [rbx + 0x127], sil
1405ad9d1: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ad9d9: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ad9e1: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ad9ea: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ad9f3: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ad9fc: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ada05: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ada0e: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ada17: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ada20: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ada26: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ada2d: 5b                          	pop	rbx
1405ada2e: 5f                          	pop	rdi
1405ada2f: 5e                          	pop	rsi
1405ada30: 41 5c                       	pop	r12
1405ada32: 41 5d                       	pop	r13
1405ada34: 41 5e                       	pop	r14
1405ada36: 41 5f                       	pop	r15
1405ada38: 5d                          	pop	rbp
1405ada39: c3                          	ret
1405ada3a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1405ada40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405ada45: 55                          	push	rbp
1405ada46: 41 57                       	push	r15
1405ada48: 41 56                       	push	r14
1405ada4a: 41 55                       	push	r13
1405ada4c: 41 54                       	push	r12
1405ada4e: 56                          	push	rsi
1405ada4f: 57                          	push	rdi
1405ada50: 53                          	push	rbx
1405ada51: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405ada58: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405ada5f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405ada65: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405ada6e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405ada77: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405ada80: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405ada89: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405ada92: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405ada9b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405adaa4: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405adaac: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405adab4: 48 83 e2 80                 	and	rdx, -0x80
1405adab8: 48 89 d3                    	mov	rbx, rdx
1405adabb: 0f b6 b3 27 01 00 00        	movzx	esi, byte ptr [rbx + 0x127]
1405adac2: 0f b6 bb 25 01 00 00        	movzx	edi, byte ptr [rbx + 0x125]
1405adac9: 44 0f b6 b3 24 01 00 00     	movzx	r14d, byte ptr [rbx + 0x124]
1405adad1: f6 83 26 01 00 00 01        	test	byte ptr [rbx + 0x126], 0x1
1405adad8: 0f 84 8a 00 00 00           	je	0x1405adb68 <.text+0x5acb68>
1405adade: 48 8b 8d b0 37 00 00        	mov	rcx, qword ptr [rbp + 0x37b0]
1405adae5: e8 66 49 04 00              	call	0x1405f2450 <.text+0x5f1450>
1405adaea: 44 88 b3 1a 01 00 00        	mov	byte ptr [rbx + 0x11a], r14b
1405adaf1: 40 88 bb 1b 01 00 00        	mov	byte ptr [rbx + 0x11b], dil
1405adaf8: 40 88 b3 1c 01 00 00        	mov	byte ptr [rbx + 0x11c], sil
1405adaff: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405adb07: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405adb0f: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405adb18: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405adb21: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405adb2a: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405adb33: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405adb3c: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405adb45: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405adb4e: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405adb54: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405adb5b: 5b                          	pop	rbx
1405adb5c: 5f                          	pop	rdi
1405adb5d: 5e                          	pop	rsi
1405adb5e: 41 5c                       	pop	r12
1405adb60: 41 5d                       	pop	r13
1405adb62: 41 5e                       	pop	r14
1405adb64: 41 5f                       	pop	r15
1405adb66: 5d                          	pop	rbp
1405adb67: c3                          	ret
1405adb68: 44 88 b3 1a 01 00 00        	mov	byte ptr [rbx + 0x11a], r14b
1405adb6f: 40 88 bb 1b 01 00 00        	mov	byte ptr [rbx + 0x11b], dil
1405adb76: 40 88 b3 1c 01 00 00        	mov	byte ptr [rbx + 0x11c], sil
1405adb7d: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405adb85: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405adb8d: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405adb96: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405adb9f: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405adba8: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405adbb1: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405adbba: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405adbc3: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405adbcc: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405adbd2: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405adbd9: 5b                          	pop	rbx
1405adbda: 5f                          	pop	rdi
1405adbdb: 5e                          	pop	rsi
1405adbdc: 41 5c                       	pop	r12
1405adbde: 41 5d                       	pop	r13
1405adbe0: 41 5e                       	pop	r14
1405adbe2: 41 5f                       	pop	r15
1405adbe4: 5d                          	pop	rbp
1405adbe5: c3                          	ret
1405adbe6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1405adbf0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405adbf5: 55                          	push	rbp
1405adbf6: 41 57                       	push	r15
1405adbf8: 41 56                       	push	r14
1405adbfa: 41 55                       	push	r13
1405adbfc: 41 54                       	push	r12
1405adbfe: 56                          	push	rsi
1405adbff: 57                          	push	rdi
1405adc00: 53                          	push	rbx
1405adc01: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405adc08: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405adc0f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405adc15: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405adc1e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405adc27: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405adc30: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405adc39: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405adc42: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405adc4b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405adc54: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405adc5c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405adc64: 48 83 e2 80                 	and	rdx, -0x80
1405adc68: 48 89 d3                    	mov	rbx, rdx
1405adc6b: 0f b6 bb 1b 01 00 00        	movzx	edi, byte ptr [rbx + 0x11b]
1405adc72: 0f b6 b3 1a 01 00 00        	movzx	esi, byte ptr [rbx + 0x11a]
1405adc79: 48 8b 85 a8 37 00 00        	mov	rax, qword ptr [rbp + 0x37a8]
1405adc80: 48 85 c0                    	test	rax, rax
1405adc83: 0f 95 c1                    	setne	cl
1405adc86: 84 8b 1c 01 00 00           	test	byte ptr [rbx + 0x11c], cl
1405adc8c: 74 12                       	je	0x1405adca0 <.text+0x5acca0>
1405adc8e: f0                          	lock
1405adc8f: 48 ff 08                    	dec	qword ptr [rax]
1405adc92: 75 0c                       	jne	0x1405adca0 <.text+0x5acca0>
1405adc94: 48 8d 8d a8 37 00 00        	lea	rcx, [rbp + 0x37a8]
1405adc9b: e8 b0 76 5d 00              	call	0x140b85350 <.text+0xb84350>
1405adca0: 40 f6 c7 01                 	test	dil, 0x1
1405adca4: 74 7c                       	je	0x1405add22 <.text+0x5acd22>
1405adca6: 48 8b 8b c8 00 00 00        	mov	rcx, qword ptr [rbx + 0xc8]
1405adcad: e8 1e 90 03 00              	call	0x1405e6cd0 <.text+0x5e5cd0>
1405adcb2: 40 88 b3 1d 01 00 00        	mov	byte ptr [rbx + 0x11d], sil
1405adcb9: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405adcc1: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405adcc9: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405adcd2: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405adcdb: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405adce4: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405adced: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405adcf6: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405adcff: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405add08: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405add0e: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405add15: 5b                          	pop	rbx
1405add16: 5f                          	pop	rdi
1405add17: 5e                          	pop	rsi
1405add18: 41 5c                       	pop	r12
1405add1a: 41 5d                       	pop	r13
1405add1c: 41 5e                       	pop	r14
1405add1e: 41 5f                       	pop	r15
1405add20: 5d                          	pop	rbp
1405add21: c3                          	ret
1405add22: 40 88 b3 1d 01 00 00        	mov	byte ptr [rbx + 0x11d], sil
1405add29: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405add31: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405add39: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405add42: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405add4b: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405add54: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405add5d: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405add66: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405add6f: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405add78: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405add7e: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405add85: 5b                          	pop	rbx
1405add86: 5f                          	pop	rdi
1405add87: 5e                          	pop	rsi
1405add88: 41 5c                       	pop	r12
1405add8a: 41 5d                       	pop	r13
1405add8c: 41 5e                       	pop	r14
1405add8e: 41 5f                       	pop	r15
1405add90: 5d                          	pop	rbp
1405add91: c3                          	ret
1405add92: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1405adda0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405adda5: 55                          	push	rbp
1405adda6: 41 57                       	push	r15
1405adda8: 41 56                       	push	r14
1405addaa: 41 55                       	push	r13
1405addac: 41 54                       	push	r12
1405addae: 56                          	push	rsi
1405addaf: 57                          	push	rdi
1405addb0: 53                          	push	rbx
1405addb1: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405addb8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405addbf: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405addc5: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405addce: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405addd7: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405adde0: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405adde9: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405addf2: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405addfb: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405ade04: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405ade0c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405ade14: 48 83 e2 80                 	and	rdx, -0x80
1405ade18: 48 89 d3                    	mov	rbx, rdx
1405ade1b: f6 83 1d 01 00 00 01        	test	byte ptr [rbx + 0x11d], 0x1
1405ade22: 0f 84 82 00 00 00           	je	0x1405adeaa <.text+0x5aceaa>
1405ade28: 48 8b 83 40 01 00 00        	mov	rax, qword ptr [rbx + 0x140]
1405ade2f: f0                          	lock
1405ade30: 48 ff 08                    	dec	qword ptr [rax]
1405ade33: 75 0c                       	jne	0x1405ade41 <.text+0x5ace41>
1405ade35: 48 8d 8b 40 01 00 00        	lea	rcx, [rbx + 0x140]
1405ade3c: e8 ff 39 08 00              	call	0x140631840 <.text+0x630840>
1405ade41: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405ade49: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405ade51: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405ade5a: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405ade63: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405ade6c: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405ade75: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405ade7e: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405ade87: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405ade90: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405ade96: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ade9d: 5b                          	pop	rbx
1405ade9e: 5f                          	pop	rdi
1405ade9f: 5e                          	pop	rsi
1405adea0: 41 5c                       	pop	r12
1405adea2: 41 5d                       	pop	r13
1405adea4: 41 5e                       	pop	r14
1405adea6: 41 5f                       	pop	r15
1405adea8: 5d                          	pop	rbp
1405adea9: c3                          	ret
1405adeaa: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405adeb2: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405adeba: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405adec3: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405adecc: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405aded5: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405adede: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405adee7: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405adef0: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405adef9: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405adeff: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405adf06: 5b                          	pop	rbx
1405adf07: 5f                          	pop	rdi
1405adf08: 5e                          	pop	rsi
1405adf09: 41 5c                       	pop	r12
1405adf0b: 41 5d                       	pop	r13
1405adf0d: 41 5e                       	pop	r14
1405adf0f: 41 5f                       	pop	r15
1405adf11: 5d                          	pop	rbp
1405adf12: c3                          	ret
1405adf13: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1405adf20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405adf25: 55                          	push	rbp
1405adf26: 41 57                       	push	r15
1405adf28: 41 56                       	push	r14
1405adf2a: 41 55                       	push	r13
1405adf2c: 41 54                       	push	r12
1405adf2e: 56                          	push	rsi
1405adf2f: 57                          	push	rdi
1405adf30: 53                          	push	rbx
1405adf31: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
1405adf38: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405adf3f: 44 0f 29 7c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm15
1405adf45: 44 0f 29 b4 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm14
1405adf4e: 44 0f 29 ac 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm13
1405adf57: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
1405adf60: 44 0f 29 9c 24 b0 00 00 00  	movaps	xmmword ptr [rsp + 0xb0], xmm11
1405adf69: 44 0f 29 94 24 c0 00 00 00  	movaps	xmmword ptr [rsp + 0xc0], xmm10
1405adf72: 44 0f 29 8c 24 d0 00 00 00  	movaps	xmmword ptr [rsp + 0xd0], xmm9
1405adf7b: 44 0f 29 84 24 e0 00 00 00  	movaps	xmmword ptr [rsp + 0xe0], xmm8
1405adf84: 0f 29 bc 24 f0 00 00 00     	movaps	xmmword ptr [rsp + 0xf0], xmm7
1405adf8c: 0f 29 b4 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm6
1405adf94: 48 83 e2 80                 	and	rdx, -0x80
1405adf98: 48 89 d3                    	mov	rbx, rdx
1405adf9b: 48 8d 8b c0 02 00 00        	lea	rcx, [rbx + 0x2c0]
1405adfa2: e8 d9 52 02 00              	call	0x1405d3280 <.text+0x5d2280>
1405adfa7: 0f 28 b4 24 00 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x100]
1405adfaf: 0f 28 bc 24 f0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xf0]
1405adfb7: 44 0f 28 84 24 e0 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0xe0]
1405adfc0: 44 0f 28 8c 24 d0 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0xd0]
1405adfc9: 44 0f 28 94 24 c0 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0xc0]
1405adfd2: 44 0f 28 9c 24 b0 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0xb0]
1405adfdb: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
1405adfe4: 44 0f 28 ac 24 90 00 00 00  	movaps	xmm13, xmmword ptr [rsp + 0x90]
1405adfed: 44 0f 28 b4 24 80 00 00 00  	movaps	xmm14, xmmword ptr [rsp + 0x80]
1405adff6: 44 0f 28 7c 24 70           	movaps	xmm15, xmmword ptr [rsp + 0x70]
1405adffc: 48 81 c4 18 01 00 00        	add	rsp, 0x118
1405ae003: 5b                          	pop	rbx
1405ae004: 5f                          	pop	rdi
1405ae005: 5e                          	pop	rsi
1405ae006: 41 5c                       	pop	r12
1405ae008: 41 5d                       	pop	r13
1405ae00a: 41 5e                       	pop	r14
1405ae00c: 41 5f                       	pop	r15
1405ae00e: 5d                          	pop	rbp
1405ae00f: c3                          	ret


ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415673b0: 55                          	push	rbp
1415673b1: 41 57                       	push	r15
1415673b3: 41 56                       	push	r14
1415673b5: 41 55                       	push	r13
1415673b7: 41 54                       	push	r12
1415673b9: 56                          	push	rsi
1415673ba: 57                          	push	rdi
1415673bb: 53                          	push	rbx
1415673bc: 48 81 ec 48 01 00 00        	sub	rsp, 0x148
1415673c3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1415673cb: 48 c7 85 c0 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xc0], -0x2
1415673d6: 4c 89 85 b0 00 00 00        	mov	qword ptr [rbp + 0xb0], r8
1415673dd: 4d 89 cf                    	mov	r15, r9
1415673e0: 49 c1 e7 04                 	shl	r15, 0x4
1415673e4: 45 31 ed                    	xor	r13d, r13d
1415673e7: 4c 89 c8                    	mov	rax, r9
1415673ea: 48 c1 e8 3c                 	shr	rax, 0x3c
1415673ee: 0f 85 3a 04 00 00           	jne	0x14156782e <.text+0x156682e>
1415673f4: 48 bf fc ff ff ff ff ff ff 7f       	movabs	rdi, 0x7ffffffffffffffc
1415673fe: 49 39 ff                    	cmp	r15, rdi
141567401: 0f 87 27 04 00 00           	ja	0x14156782e <.text+0x156682e>
141567407: 4d 89 ce                    	mov	r14, r9
14156740a: 48 89 d3                    	mov	rbx, rdx
14156740d: 48 89 ce                    	mov	rsi, rcx
141567410: 4d 85 ff                    	test	r15, r15
141567413: 74 47                       	je	0x14156745c <.text+0x156645c>
141567415: 0f b6 05 ed d7 0c 02        	movzx	eax, byte ptr [rip + 0x20cd7ed] # 0x143634c09
14156741c: 41 bd 04 00 00 00           	mov	r13d, 0x4
141567422: ba 04 00 00 00              	mov	edx, 0x4
141567427: 4c 89 f9                    	mov	rcx, r15
14156742a: e8 e1 c7 0a ff              	call	0x140613c10 <.text+0x612c10>
14156742f: 48 85 c0                    	test	rax, rax
141567432: 0f 84 f6 03 00 00           	je	0x14156782e <.text+0x156682e>
141567438: 49 89 c4                    	mov	r12, rax
14156743b: 4d 89 f5                    	mov	r13, r14
14156743e: 49 83 fe 02                 	cmp	r14, 0x2
141567442: 72 27                       	jb	0x14156746b <.text+0x156646b>
141567444: 4d 8d 47 f0                 	lea	r8, [r15 - 0x10]
141567448: 4c 89 e1                    	mov	rcx, r12
14156744b: 31 d2                       	xor	edx, edx
14156744d: e8 ce c7 3b 01              	call	0x142923c20 <.text+0x2922c20>
141567452: 4b 8d 04 3c                 	lea	rax, [r12 + r15]
141567456: 48 83 c0 f0                 	add	rax, -0x10
14156745a: eb 17                       	jmp	0x141567473 <.text+0x1566473>
14156745c: 41 bc 04 00 00 00           	mov	r12d, 0x4
141567462: 45 31 ed                    	xor	r13d, r13d
141567465: 49 83 fe 02                 	cmp	r14, 0x2
141567469: 73 d9                       	jae	0x141567444 <.text+0x1566444>
14156746b: 4c 89 e0                    	mov	rax, r12
14156746e: 4d 85 f6                    	test	r14, r14
141567471: 74 06                       	je	0x141567479 <.text+0x1566479>
141567473: 0f 57 c0                    	xorps	xmm0, xmm0
141567476: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
141567479: 4c 89 6d 50                 	mov	qword ptr [rbp + 0x50], r13
14156747d: 4c 89 65 58                 	mov	qword ptr [rbp + 0x58], r12
141567481: 4c 89 75 60                 	mov	qword ptr [rbp + 0x60], r14
141567485: 48 8d 4d f0                 	lea	rcx, [rbp - 0x10]
141567489: 4c 8d 45 50                 	lea	r8, [rbp + 0x50]
14156748d: 48 89 da                    	mov	rdx, rbx
141567490: e8 7b a3 12 00              	call	0x141691810 <.text+0x1690810>
141567495: 0f 57 c0                    	xorps	xmm0, xmm0
141567498: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
14156749c: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
1415674a0: 48 c7 45 70 00 00 00 00     	mov	qword ptr [rbp + 0x70], 0x0
1415674a8: 45 31 e4                    	xor	r12d, r12d
1415674ab: b9 30 00 00 00              	mov	ecx, 0x30
1415674b0: 4c 89 f0                    	mov	rax, r14
1415674b3: 48 f7 e1                    	mul	rcx
1415674b6: 49 89 c7                    	mov	r15, rax
1415674b9: 0f 80 81 03 00 00           	jo	0x141567840 <.text+0x1566840>
1415674bf: 49 39 ff                    	cmp	r15, rdi
1415674c2: 0f 87 78 03 00 00           	ja	0x141567840 <.text+0x1566840>
1415674c8: 4d 85 ff                    	test	r15, r15
1415674cb: 74 48                       	je	0x141567515 <.text+0x1566515>
1415674cd: 0f b6 05 35 d7 0c 02        	movzx	eax, byte ptr [rip + 0x20cd735] # 0x143634c09
1415674d4: 41 bc 04 00 00 00           	mov	r12d, 0x4
1415674da: ba 04 00 00 00              	mov	edx, 0x4
1415674df: 4c 89 f9                    	mov	rcx, r15
1415674e2: e8 29 c7 0a ff              	call	0x140613c10 <.text+0x612c10>
1415674e7: 48 85 c0                    	test	rax, rax
1415674ea: 0f 84 50 03 00 00           	je	0x141567840 <.text+0x1566840>
1415674f0: 4c 89 f1                    	mov	rcx, r14
1415674f3: 49 83 fe 02                 	cmp	r14, 0x2
1415674f7: 72 29                       	jb	0x141567522 <.text+0x1566522>
1415674f9: 4d 8d 4e ff                 	lea	r9, [r14 - 0x1]
1415674fd: 4d 8d 46 fe                 	lea	r8, [r14 - 0x2]
141567501: 44 89 ca                    	mov	edx, r9d
141567504: 83 e2 07                    	and	edx, 0x7
141567507: 49 83 f8 07                 	cmp	r8, 0x7
14156750b: 73 26                       	jae	0x141567533 <.text+0x1566533>
14156750d: 49 89 c0                    	mov	r8, rax
141567510: e9 4a 01 00 00              	jmp	0x14156765f <.text+0x156665f>
141567515: b8 04 00 00 00              	mov	eax, 0x4
14156751a: 31 c9                       	xor	ecx, ecx
14156751c: 49 83 fe 02                 	cmp	r14, 0x2
141567520: 73 d7                       	jae	0x1415674f9 <.text+0x15664f9>
141567522: 49 89 c0                    	mov	r8, rax
141567525: 4d 85 f6                    	test	r14, r14
141567528: 0f 85 62 01 00 00           	jne	0x141567690 <.text+0x1566690>
14156752e: e9 84 01 00 00              	jmp	0x1415676b7 <.text+0x15666b7>
141567533: 49 83 e1 f8                 	and	r9, -0x8
141567537: f2 0f 10 05 e1 97 57 01     	movsd	xmm0, qword ptr [rip + 0x15797e1] # 0x142ae0d20
14156753f: 0f 57 c9                    	xorps	xmm1, xmm1
141567542: 49 89 c0                    	mov	r8, rax
141567545: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
141567550: f2 41 0f 11 00              	movsd	qword ptr [r8], xmm0
141567555: 41 0f 11 48 08              	movups	xmmword ptr [r8 + 0x8], xmm1
14156755a: 41 0f 11 48 18              	movups	xmmword ptr [r8 + 0x18], xmm1
14156755f: 49 c7 40 28 00 00 00 00     	mov	qword ptr [r8 + 0x28], 0x0
141567567: f2 41 0f 11 40 30           	movsd	qword ptr [r8 + 0x30], xmm0
14156756d: 41 0f 11 48 38              	movups	xmmword ptr [r8 + 0x38], xmm1
141567572: 41 0f 11 48 48              	movups	xmmword ptr [r8 + 0x48], xmm1
141567577: 49 c7 40 58 00 00 00 00     	mov	qword ptr [r8 + 0x58], 0x0
14156757f: f2 41 0f 11 40 60           	movsd	qword ptr [r8 + 0x60], xmm0
141567585: 41 0f 11 48 68              	movups	xmmword ptr [r8 + 0x68], xmm1
14156758a: 41 0f 11 48 78              	movups	xmmword ptr [r8 + 0x78], xmm1
14156758f: 49 c7 80 88 00 00 00 00 00 00 00    	mov	qword ptr [r8 + 0x88], 0x0
14156759a: f2 41 0f 11 80 90 00 00 00  	movsd	qword ptr [r8 + 0x90], xmm0
1415675a3: 41 0f 11 88 98 00 00 00     	movups	xmmword ptr [r8 + 0x98], xmm1
1415675ab: 41 0f 11 88 a8 00 00 00     	movups	xmmword ptr [r8 + 0xa8], xmm1
1415675b3: 49 c7 80 b8 00 00 00 00 00 00 00    	mov	qword ptr [r8 + 0xb8], 0x0
1415675be: f2 41 0f 11 80 c0 00 00 00  	movsd	qword ptr [r8 + 0xc0], xmm0
1415675c7: 41 0f 11 88 c8 00 00 00     	movups	xmmword ptr [r8 + 0xc8], xmm1
1415675cf: 41 0f 11 88 d8 00 00 00     	movups	xmmword ptr [r8 + 0xd8], xmm1
1415675d7: 49 c7 80 e8 00 00 00 00 00 00 00    	mov	qword ptr [r8 + 0xe8], 0x0
1415675e2: f2 41 0f 11 80 f0 00 00 00  	movsd	qword ptr [r8 + 0xf0], xmm0
1415675eb: 41 0f 11 88 f8 00 00 00     	movups	xmmword ptr [r8 + 0xf8], xmm1
1415675f3: 41 0f 11 88 08 01 00 00     	movups	xmmword ptr [r8 + 0x108], xmm1
1415675fb: 49 c7 80 18 01 00 00 00 00 00 00    	mov	qword ptr [r8 + 0x118], 0x0
141567606: f2 41 0f 11 80 20 01 00 00  	movsd	qword ptr [r8 + 0x120], xmm0
14156760f: 41 0f 11 88 28 01 00 00     	movups	xmmword ptr [r8 + 0x128], xmm1
141567617: 41 0f 11 88 38 01 00 00     	movups	xmmword ptr [r8 + 0x138], xmm1
14156761f: 49 c7 80 48 01 00 00 00 00 00 00    	mov	qword ptr [r8 + 0x148], 0x0
14156762a: f2 41 0f 11 80 50 01 00 00  	movsd	qword ptr [r8 + 0x150], xmm0
141567633: 41 0f 11 88 58 01 00 00     	movups	xmmword ptr [r8 + 0x158], xmm1
14156763b: 41 0f 11 88 68 01 00 00     	movups	xmmword ptr [r8 + 0x168], xmm1
141567643: 49 c7 80 78 01 00 00 00 00 00 00    	mov	qword ptr [r8 + 0x178], 0x0
14156764e: 49 81 c0 80 01 00 00        	add	r8, 0x180
141567655: 49 83 c1 f8                 	add	r9, -0x8
141567659: 0f 85 f1 fe ff ff           	jne	0x141567550 <.text+0x1566550>
14156765f: 48 85 d2                    	test	rdx, rdx
141567662: 74 2c                       	je	0x141567690 <.text+0x1566690>
141567664: f2 0f 10 05 b4 96 57 01     	movsd	xmm0, qword ptr [rip + 0x15796b4] # 0x142ae0d20
14156766c: 0f 57 c9                    	xorps	xmm1, xmm1
14156766f: 90                          	nop
141567670: f2 41 0f 11 00              	movsd	qword ptr [r8], xmm0
141567675: 41 0f 11 48 08              	movups	xmmword ptr [r8 + 0x8], xmm1
14156767a: 41 0f 11 48 18              	movups	xmmword ptr [r8 + 0x18], xmm1
14156767f: 49 c7 40 28 00 00 00 00     	mov	qword ptr [r8 + 0x28], 0x0
141567687: 49 83 c0 30                 	add	r8, 0x30
14156768b: 48 ff ca                    	dec	rdx
14156768e: 75 e0                       	jne	0x141567670 <.text+0x1566670>
141567690: f2 0f 10 05 88 96 57 01     	movsd	xmm0, qword ptr [rip + 0x1579688] # 0x142ae0d20
141567698: f2 41 0f 11 00              	movsd	qword ptr [r8], xmm0
14156769d: 0f 28 45 50                 	movaps	xmm0, xmmword ptr [rbp + 0x50]
1415676a1: 0f 28 4d 60                 	movaps	xmm1, xmmword ptr [rbp + 0x60]
1415676a5: 41 0f 11 40 08              	movups	xmmword ptr [r8 + 0x8], xmm0
1415676aa: 41 0f 11 48 18              	movups	xmmword ptr [r8 + 0x18], xmm1
1415676af: 48 8b 55 70                 	mov	rdx, qword ptr [rbp + 0x70]
1415676b3: 49 89 50 28                 	mov	qword ptr [r8 + 0x28], rdx
1415676b7: 48 89 4d 38                 	mov	qword ptr [rbp + 0x38], rcx
1415676bb: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
1415676bf: 4c 89 75 48                 	mov	qword ptr [rbp + 0x48], r14
1415676c3: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1415676c7: 4c 8d 45 38                 	lea	r8, [rbp + 0x38]
1415676cb: 48 89 da                    	mov	rdx, rbx
1415676ce: e8 8d 8c 12 00              	call	0x141690360 <.text+0x168f360>
1415676d3: 0f 10 45 a8                 	movups	xmm0, xmmword ptr [rbp - 0x58]
1415676d7: 0f 10 4d b8                 	movups	xmm1, xmmword ptr [rbp - 0x48]
1415676db: 0f 10 55 c8                 	movups	xmm2, xmmword ptr [rbp - 0x38]
1415676df: 0f 10 5d d8                 	movups	xmm3, xmmword ptr [rbp - 0x28]
1415676e3: 0f 11 45 68                 	movups	xmmword ptr [rbp + 0x68], xmm0
1415676e7: 0f 11 4d 78                 	movups	xmmword ptr [rbp + 0x78], xmm1
1415676eb: 0f 11 95 88 00 00 00        	movups	xmmword ptr [rbp + 0x88], xmm2
1415676f2: 0f 11 9d 98 00 00 00        	movups	xmmword ptr [rbp + 0x98], xmm3
1415676f9: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1415676fd: 48 89 85 a8 00 00 00        	mov	qword ptr [rbp + 0xa8], rax
141567704: c6 45 5e 05                 	mov	byte ptr [rbp + 0x5e], 0x5
141567708: 0f b6 05 fa d4 0c 02        	movzx	eax, byte ptr [rip + 0x20cd4fa] # 0x143634c09
14156770f: b9 60 00 00 00              	mov	ecx, 0x60
141567714: ba 08 00 00 00              	mov	edx, 0x8
141567719: e8 f2 c4 0a ff              	call	0x140613c10 <.text+0x612c10>
14156771e: 48 85 c0                    	test	rax, rax
141567721: 0f 84 2d 01 00 00           	je	0x141567854 <.text+0x1566854>
141567727: 0f 10 85 a0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xa0]
14156772e: 0f 11 40 50                 	movups	xmmword ptr [rax + 0x50], xmm0
141567732: 0f 10 85 90 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x90]
141567739: 0f 11 40 40                 	movups	xmmword ptr [rax + 0x40], xmm0
14156773d: 0f 10 45 50                 	movups	xmm0, xmmword ptr [rbp + 0x50]
141567741: 0f 10 4d 60                 	movups	xmm1, xmmword ptr [rbp + 0x60]
141567745: 0f 10 55 70                 	movups	xmm2, xmmword ptr [rbp + 0x70]
141567749: 0f 10 9d 80 00 00 00        	movups	xmm3, xmmword ptr [rbp + 0x80]
141567750: 0f 11 58 30                 	movups	xmmword ptr [rax + 0x30], xmm3
141567754: 0f 11 50 20                 	movups	xmmword ptr [rax + 0x20], xmm2
141567758: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
14156775c: 48 89 85 b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rax
141567763: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
141567766: 31 c9                       	xor	ecx, ecx
141567768: e8 b3 23 b7 fe              	call	0x1400d9b20 <.text+0xd8b20>
14156776d: 48 85 c0                    	test	rax, rax
141567770: 0f 84 ef 00 00 00           	je	0x141567865 <.text+0x1566865>
141567776: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
141567779: 48 ff 00                    	inc	qword ptr [rax]
14156777c: 48 8b 45 30                 	mov	rax, qword ptr [rbp + 0x30]
141567780: 48 89 86 88 00 00 00        	mov	qword ptr [rsi + 0x88], rax
141567787: 0f 10 4d f0                 	movups	xmm1, xmmword ptr [rbp - 0x10]
14156778b: 0f 10 55 00                 	movups	xmm2, xmmword ptr [rbp]
14156778f: 0f 10 5d 10                 	movups	xmm3, xmmword ptr [rbp + 0x10]
141567793: 0f 10 65 20                 	movups	xmm4, xmmword ptr [rbp + 0x20]
141567797: 0f 11 66 78                 	movups	xmmword ptr [rsi + 0x78], xmm4
14156779b: 0f 11 5e 68                 	movups	xmmword ptr [rsi + 0x68], xmm3
14156779f: 0f 11 56 58                 	movups	xmmword ptr [rsi + 0x58], xmm2
1415677a3: 0f 11 4e 48                 	movups	xmmword ptr [rsi + 0x48], xmm1
1415677a7: 48 c7 06 00 00 00 00        	mov	qword ptr [rsi], 0x0
1415677ae: 48 c7 46 08 04 00 00 00     	mov	qword ptr [rsi + 0x8], 0x4
1415677b6: 48 c7 46 10 00 00 00 00     	mov	qword ptr [rsi + 0x10], 0x0
1415677be: 0f 10 0d 4b 12 56 01        	movups	xmm1, xmmword ptr [rip + 0x156124b] # 0x142ac8a10
1415677c5: 0f 11 4e 18                 	movups	xmmword ptr [rsi + 0x18], xmm1
1415677c9: 0f 10 0d 50 12 56 01        	movups	xmm1, xmmword ptr [rip + 0x1561250] # 0x142ac8a20
1415677d0: 0f 11 4e 28                 	movups	xmmword ptr [rsi + 0x28], xmm1
1415677d4: 0f 11 46 38                 	movups	xmmword ptr [rsi + 0x38], xmm0
1415677d8: 48 8b 85 b8 00 00 00        	mov	rax, qword ptr [rbp + 0xb8]
1415677df: 48 89 86 90 00 00 00        	mov	qword ptr [rsi + 0x90], rax
1415677e6: 48 8d 05 cb ef 70 01        	lea	rax, [rip + 0x170efcb]  # 0x142c767b8
1415677ed: 48 89 86 98 00 00 00        	mov	qword ptr [rsi + 0x98], rax
1415677f4: 48 c7 86 a0 00 00 00 00 00 00 00    	mov	qword ptr [rsi + 0xa0], 0x0
1415677ff: 48 8b 85 b0 00 00 00        	mov	rax, qword ptr [rbp + 0xb0]
141567806: 48 89 86 a8 00 00 00        	mov	qword ptr [rsi + 0xa8], rax
14156780d: c7 86 b0 00 00 00 00 00 00 00       	mov	dword ptr [rsi + 0xb0], 0x0
141567817: 48 89 f0                    	mov	rax, rsi
14156781a: 48 81 c4 48 01 00 00        	add	rsp, 0x148
141567821: 5b                          	pop	rbx
141567822: 5f                          	pop	rdi
141567823: 5e                          	pop	rsi
141567824: 41 5c                       	pop	r12
141567826: 41 5d                       	pop	r13
141567828: 41 5e                       	pop	r14
14156782a: 41 5f                       	pop	r15
14156782c: 5d                          	pop	rbp
14156782d: c3                          	ret
14156782e: 4c 8d 05 73 e8 70 01        	lea	r8, [rip + 0x170e873]   # 0x142c760a8
141567835: 4c 89 e9                    	mov	rcx, r13
141567838: 4c 89 fa                    	mov	rdx, r15
14156783b: e8 a3 17 37 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
141567840: 4c 8d 05 79 e8 70 01        	lea	r8, [rip + 0x170e879]   # 0x142c760c0
141567847: 4c 89 e1                    	mov	rcx, r12
14156784a: 4c 89 fa                    	mov	rdx, r15
14156784d: e8 91 17 37 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
141567852: eb 1d                       	jmp	0x141567871 <.text+0x1566871>
141567854: b9 08 00 00 00              	mov	ecx, 0x8
141567859: ba 60 00 00 00              	mov	edx, 0x60
14156785e: e8 9d 17 37 01              	call	0x1428d9000 <.text+0x28d8000>
141567863: eb 0c                       	jmp	0x141567871 <.text+0x1566871>
141567865: 48 8d 0d 34 32 6d 01        	lea	rcx, [rip + 0x16d3234]  # 0x142c3aaa0
14156786c: e8 5f 08 37 01              	call	0x1428d80d0 <.text+0x28d70d0>
141567871: 0f 0b                       	ud2
141567873: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
141567880: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141567885: 55                          	push	rbp
141567886: 41 57                       	push	r15
141567888: 41 56                       	push	r14
14156788a: 41 55                       	push	r13
14156788c: 41 54                       	push	r12
14156788e: 56                          	push	rsi
14156788f: 57                          	push	rdi
141567890: 53                          	push	rbx
141567891: 48 83 ec 28                 	sub	rsp, 0x28
141567895: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14156789c: 48 8d 4d f0                 	lea	rcx, [rbp - 0x10]
1415678a0: e8 bb 5a 31 ff              	call	0x14087d360 <.text+0x87c360>
1415678a5: 90                          	nop
1415678a6: 48 83 c4 28                 	add	rsp, 0x28
1415678aa: 5b                          	pop	rbx
1415678ab: 5f                          	pop	rdi
1415678ac: 5e                          	pop	rsi
1415678ad: 41 5c                       	pop	r12
1415678af: 41 5d                       	pop	r13
1415678b1: 41 5e                       	pop	r14
1415678b3: 41 5f                       	pop	r15
1415678b5: 5d                          	pop	rbp
1415678b6: c3                          	ret
1415678b7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1415678c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1415678c5: 55                          	push	rbp
1415678c6: 41 57                       	push	r15
1415678c8: 41 56                       	push	r14
1415678ca: 41 55                       	push	r13
1415678cc: 41 54                       	push	r12
1415678ce: 56                          	push	rsi
1415678cf: 57                          	push	rdi
1415678d0: 53                          	push	rbx
1415678d1: 48 83 ec 28                 	sub	rsp, 0x28
1415678d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1415678dc: 48 8d 4d 50                 	lea	rcx, [rbp + 0x50]
1415678e0: e8 3b 6f 31 ff              	call	0x14087e820 <.text+0x87d820>
1415678e5: 90                          	nop
1415678e6: 48 83 c4 28                 	add	rsp, 0x28
1415678ea: 5b                          	pop	rbx
1415678eb: 5f                          	pop	rdi
1415678ec: 5e                          	pop	rsi
1415678ed: 41 5c                       	pop	r12
1415678ef: 41 5d                       	pop	r13
1415678f1: 41 5e                       	pop	r14
1415678f3: 41 5f                       	pop	r15
1415678f5: 5d                          	pop	rbp
1415678f6: c3                          	ret
1415678f7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
141567900: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141567905: 55                          	push	rbp
141567906: 41 57                       	push	r15
141567908: 41 56                       	push	r14
14156790a: 41 55                       	push	r13
14156790c: 41 54                       	push	r12
14156790e: 56                          	push	rsi
14156790f: 57                          	push	rdi
141567910: 53                          	push	rbx
141567911: 48 83 ec 28                 	sub	rsp, 0x28
141567915: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14156791c: 48 8d 15 95 ee 70 01        	lea	rdx, [rip + 0x170ee95]  # 0x142c767b8
141567923: 48 8b 8d b8 00 00 00        	mov	rcx, qword ptr [rbp + 0xb8]
14156792a: e8 d1 6f ac fe              	call	0x14002e900 <.text+0x2d900>
14156792f: 90                          	nop
141567930: 48 83 c4 28                 	add	rsp, 0x28
141567934: 5b                          	pop	rbx
141567935: 5f                          	pop	rdi
141567936: 5e                          	pop	rsi
141567937: 41 5c                       	pop	r12
141567939: 41 5d                       	pop	r13
14156793b: 41 5e                       	pop	r14
14156793d: 41 5f                       	pop	r15
14156793f: 5d                          	pop	rbp
141567940: c3                          	ret

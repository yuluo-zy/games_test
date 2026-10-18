
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409d20b0: 55                          	push	rbp
1409d20b1: 41 57                       	push	r15
1409d20b3: 41 56                       	push	r14
1409d20b5: 41 55                       	push	r13
1409d20b7: 41 54                       	push	r12
1409d20b9: 56                          	push	rsi
1409d20ba: 57                          	push	rdi
1409d20bb: 53                          	push	rbx
1409d20bc: 48 81 ec 08 0e 00 00        	sub	rsp, 0xe08
1409d20c3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1409d20cb: 0f 29 bd 70 0d 00 00        	movaps	xmmword ptr [rbp + 0xd70], xmm7
1409d20d2: 0f 29 b5 60 0d 00 00        	movaps	xmmword ptr [rbp + 0xd60], xmm6
1409d20d9: 48 c7 85 58 0d 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xd58], -0x2
1409d20e4: 4c 89 8d 40 0d 00 00        	mov	qword ptr [rbp + 0xd40], r9
1409d20eb: 4c 89 85 20 0d 00 00        	mov	qword ptr [rbp + 0xd20], r8
1409d20f2: 48 89 d3                    	mov	rbx, rdx
1409d20f5: 48 89 cf                    	mov	rdi, rcx
1409d20f8: 48 8d 72 38                 	lea	rsi, [rdx + 0x38]
1409d20fc: 48 ba 82 a3 af b9 57 c5 ca 78       	movabs	rdx, 0x78cac557b9afa382
1409d2106: 49 b8 55 a0 83 7a 1d 80 99 41       	movabs	r8, 0x4199801d7a83a055
1409d2110: 48 89 f1                    	mov	rcx, rsi
1409d2113: e8 d8 b5 70 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1409d2118: 48 85 c0                    	test	rax, rax
1409d211b: 0f 84 69 0e 00 00           	je	0x1409d2f8a <.text+0x9d1f8a>
1409d2121: 48 39 93 18 02 00 00        	cmp	qword ptr [rbx + 0x218], rdx
1409d2128: 0f 86 5c 0e 00 00           	jbe	0x1409d2f8a <.text+0x9d1f8a>
1409d212e: 48 8b 83 10 02 00 00        	mov	rax, qword ptr [rbx + 0x210]
1409d2135: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1409d2139: 48 85 c9                    	test	rcx, rcx
1409d213c: 0f 84 48 0e 00 00           	je	0x1409d2f8a <.text+0x9d1f8a>
1409d2142: 48 8b 83 e0 01 00 00        	mov	rax, qword ptr [rbx + 0x1e0]
1409d2149: 48 f7 d1                    	not	rcx
1409d214c: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1409d2150: 48 c1 e1 05                 	shl	rcx, 0x5
1409d2154: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1409d215a: 0f 84 2a 0e 00 00           	je	0x1409d2f8a <.text+0x9d1f8a>
1409d2160: 48 01 c8                    	add	rax, rcx
1409d2163: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
1409d2167: 48 89 85 30 0d 00 00        	mov	qword ptr [rbp + 0xd30], rax
1409d216e: 48 ba 29 f7 80 5b 19 62 c3 4b       	movabs	rdx, 0x4bc362195b80f729
1409d2178: 49 b8 fb ce cb 80 ce 98 27 98       	movabs	r8, -0x67d867317f343105
1409d2182: 48 89 f1                    	mov	rcx, rsi
1409d2185: e8 66 b5 70 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1409d218a: 48 85 c0                    	test	rax, rax
1409d218d: 0f 84 03 0e 00 00           	je	0x1409d2f96 <.text+0x9d1f96>
1409d2193: 48 39 93 18 02 00 00        	cmp	qword ptr [rbx + 0x218], rdx
1409d219a: 0f 86 f6 0d 00 00           	jbe	0x1409d2f96 <.text+0x9d1f96>
1409d21a0: 48 8b 83 10 02 00 00        	mov	rax, qword ptr [rbx + 0x210]
1409d21a7: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1409d21ab: 48 85 c9                    	test	rcx, rcx
1409d21ae: 0f 84 e2 0d 00 00           	je	0x1409d2f96 <.text+0x9d1f96>
1409d21b4: 48 8b 83 e0 01 00 00        	mov	rax, qword ptr [rbx + 0x1e0]
1409d21bb: 48 f7 d1                    	not	rcx
1409d21be: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1409d21c2: 48 c1 e1 05                 	shl	rcx, 0x5
1409d21c6: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1409d21cc: 0f 84 c4 0d 00 00           	je	0x1409d2f96 <.text+0x9d1f96>
1409d21d2: 48 01 c8                    	add	rax, rcx
1409d21d5: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
1409d21d9: 48 8b 08                    	mov	rcx, qword ptr [rax]
1409d21dc: 48 85 c9                    	test	rcx, rcx
1409d21df: 78 07                       	js	0x1409d21e8 <.text+0x9d11e8>
1409d21e1: f3 48 0f 2a f9              	cvtsi2ss	xmm7, rcx
1409d21e6: eb 15                       	jmp	0x1409d21fd <.text+0x9d11fd>
1409d21e8: 48 89 ca                    	mov	rdx, rcx
1409d21eb: 48 d1 ea                    	shr	rdx
1409d21ee: 83 e1 01                    	and	ecx, 0x1
1409d21f1: 48 09 d1                    	or	rcx, rdx
1409d21f4: f3 48 0f 2a f9              	cvtsi2ss	xmm7, rcx
1409d21f9: f3 0f 58 ff                 	addss	xmm7, xmm7
1409d21fd: f3 0f 2a 70 08              	cvtsi2ss	xmm6, dword ptr [rax + 0x8]
1409d2202: 48 8b 85 20 0d 00 00        	mov	rax, qword ptr [rbp + 0xd20]
1409d2209: 4c 8d 70 24                 	lea	r14, [rax + 0x24]
1409d220d: 48 8d 48 28                 	lea	rcx, [rax + 0x28]
1409d2211: e8 7a 35 20 00              	call	0x140bd5790 <.text+0xbd4790>
1409d2216: c7 85 98 06 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x698], 0x0
1409d2220: 48 8b 8d 40 0d 00 00        	mov	rcx, qword ptr [rbp + 0xd40]
1409d2227: 48 89 8d a0 06 00 00        	mov	qword ptr [rbp + 0x6a0], rcx
1409d222e: 48 c7 85 78 0b 00 00 01 00 00 00    	mov	qword ptr [rbp + 0xb78], 0x1
1409d2239: 4c 89 b5 f8 0c 00 00        	mov	qword ptr [rbp + 0xcf8], r14
1409d2240: 4c 89 b5 80 0b 00 00        	mov	qword ptr [rbp + 0xb80], r14
1409d2247: 48 c7 85 88 0b 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xb88], 0x0
1409d2252: 48 89 7c 24 40              	mov	qword ptr [rsp + 0x40], rdi
1409d2257: 48 89 9d 28 0d 00 00        	mov	qword ptr [rbp + 0xd28], rbx
1409d225e: 48 89 5c 24 38              	mov	qword ptr [rsp + 0x38], rbx
1409d2263: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1409d2268: 4c 8d b5 78 0b 00 00        	lea	r14, [rbp + 0xb78]
1409d226f: 4c 89 74 24 28              	mov	qword ptr [rsp + 0x28], r14
1409d2274: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1409d227d: 48 8d 15 0c f6 0e 02        	lea	rdx, [rip + 0x20ef60c]  # 0x142ac1890
1409d2284: 48 8d 4d c8                 	lea	rcx, [rbp - 0x38]
1409d2288: 4c 8d bd 98 06 00 00        	lea	r15, [rbp + 0x698]
1409d228f: 41 b8 17 00 00 00           	mov	r8d, 0x17
1409d2295: 4d 89 f9                    	mov	r9, r15
1409d2298: e8 d3 56 e9 ff              	call	0x140867970 <.text+0x866970>
1409d229d: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d22a4: 48 ba 55 93 67 6e 78 cc 26 82       	movabs	rdx, -0x7dd9338791986cab
1409d22ae: 49 b8 ac 41 a8 55 6a da cf 65       	movabs	r8, 0x65cfda6a55a841ac
1409d22b8: 48 89 f1                    	mov	rcx, rsi
1409d22bb: e8 30 b4 70 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1409d22c0: 48 85 c0                    	test	rax, rax
1409d22c3: 4c 8b 85 28 0d 00 00        	mov	r8, qword ptr [rbp + 0xd28]
1409d22ca: 0f 84 d2 0c 00 00           	je	0x1409d2fa2 <.text+0x9d1fa2>
1409d22d0: 49 39 90 18 02 00 00        	cmp	qword ptr [r8 + 0x218], rdx
1409d22d7: 0f 86 c5 0c 00 00           	jbe	0x1409d2fa2 <.text+0x9d1fa2>
1409d22dd: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
1409d22e4: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1409d22e8: 48 85 c9                    	test	rcx, rcx
1409d22eb: 0f 84 b1 0c 00 00           	je	0x1409d2fa2 <.text+0x9d1fa2>
1409d22f1: 49 8b 80 e0 01 00 00        	mov	rax, qword ptr [r8 + 0x1e0]
1409d22f8: 48 f7 d1                    	not	rcx
1409d22fb: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1409d22ff: 48 c1 e1 05                 	shl	rcx, 0x5
1409d2303: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1409d2309: 0f 84 93 0c 00 00           	je	0x1409d2fa2 <.text+0x9d1fa2>
1409d230f: 48 01 c8                    	add	rax, rcx
1409d2312: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
1409d2316: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d231d: 48 8d 15 0c bf 25 02        	lea	rdx, [rip + 0x225bf0c]  # 0x142c2e230
1409d2324: e8 f7 73 f9 ff              	call	0x140969720 <.text+0x968720>
1409d2329: 48 89 c6                    	mov	rsi, rax
1409d232c: 48 85 c0                    	test	rax, rax
1409d232f: 0f 84 82 0c 00 00           	je	0x1409d2fb7 <.text+0x9d1fb7>
1409d2335: 48 8d 05 c2 af 0c 02        	lea	rax, [rip + 0x20cafc2]  # 0x142a9d2fe
1409d233c: 48 89 85 78 0b 00 00        	mov	qword ptr [rbp + 0xb78], rax
1409d2343: 48 c7 85 80 0b 00 00 35 00 00 00    	mov	qword ptr [rbp + 0xb80], 0x35
1409d234e: 48 89 f7                    	mov	rdi, rsi
1409d2351: 48 83 c7 10                 	add	rdi, 0x10
1409d2355: 48 83 7e 18 35              	cmp	qword ptr [rsi + 0x18], 0x35
1409d235a: 0f 85 91 0b 00 00           	jne	0x1409d2ef1 <.text+0x9d1ef1>
1409d2360: 48 8b 17                    	mov	rdx, qword ptr [rdi]
1409d2363: 48 8d 0d 94 af 0c 02        	lea	rcx, [rip + 0x20caf94]  # 0x142a9d2fe
1409d236a: 41 b8 35 00 00 00           	mov	r8d, 0x35
1409d2370: e8 7b 18 f5 01              	call	0x142923bf0 <.text+0x2922bf0>
1409d2375: 85 c0                       	test	eax, eax
1409d2377: 0f 85 74 0b 00 00           	jne	0x1409d2ef1 <.text+0x9d1ef1>
1409d237d: 48 8b 0e                    	mov	rcx, qword ptr [rsi]
1409d2380: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
1409d2384: 48 8b 40 18                 	mov	rax, qword ptr [rax + 0x18]
1409d2388: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d238f: ff d0                       	call	rax
1409d2391: 48 85 c0                    	test	rax, rax
1409d2394: 0f 84 1d 0c 00 00           	je	0x1409d2fb7 <.text+0x9d1fb7>
1409d239a: 48 8b 95 30 0e 00 00        	mov	rdx, qword ptr [rbp + 0xe30]
1409d23a1: 4c 8b 85 10 0e 00 00        	mov	r8, qword ptr [rbp + 0xe10]
1409d23a8: 0f b6 8d f8 0d 00 00        	movzx	ecx, byte ptr [rbp + 0xdf8]
1409d23af: f3 0f 5e 35 89 52 fd 01     	divss	xmm6, dword ptr [rip + 0x1fd5289] # 0x1429a7640
1409d23b7: f3 0f 58 f7                 	addss	xmm6, xmm7
1409d23bb: 48 8b 00                    	mov	rax, qword ptr [rax]
1409d23be: 48 89 85 18 0d 00 00        	mov	qword ptr [rbp + 0xd18], rax
1409d23c5: 48 c7 85 c8 0c 00 00 05 00 00 00    	mov	qword ptr [rbp + 0xcc8], 0x5
1409d23d0: c7 85 d0 0c 00 00 01 00 00 00       	mov	dword ptr [rbp + 0xcd0], 0x1
1409d23da: c6 85 d4 0c 00 00 00        	mov	byte ptr [rbp + 0xcd4], 0x0
1409d23e1: c7 85 d8 0c 00 00 03 00 00 00       	mov	dword ptr [rbp + 0xcd8], 0x3
1409d23eb: c6 85 dc 0c 00 00 00        	mov	byte ptr [rbp + 0xcdc], 0x0
1409d23f2: c7 85 e0 0c 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xce0], 0x0
1409d23fc: c6 85 e4 0c 00 00 00        	mov	byte ptr [rbp + 0xce4], 0x0
1409d2403: c7 85 e8 0c 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xce8], 0x0
1409d240d: c6 85 ec 0c 00 00 01        	mov	byte ptr [rbp + 0xcec], 0x1
1409d2414: c7 85 f0 0c 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xcf0], 0x2
1409d241e: c6 85 f4 0c 00 00 00        	mov	byte ptr [rbp + 0xcf4], 0x0
1409d2425: 0f b6 c1                    	movzx	eax, cl
1409d2428: 89 85 3c 0d 00 00           	mov	dword ptr [rbp + 0xd3c], eax
1409d242e: 48 83 85 40 0d 00 00 04     	add	qword ptr [rbp + 0xd40], 0x4
1409d2436: 49 8d 40 24                 	lea	rax, [r8 + 0x24]
1409d243a: 48 89 85 00 0d 00 00        	mov	qword ptr [rbp + 0xd00], rax
1409d2441: 49 8d 40 48                 	lea	rax, [r8 + 0x48]
1409d2445: 48 89 85 08 0d 00 00        	mov	qword ptr [rbp + 0xd08], rax
1409d244c: 48 8d 42 18                 	lea	rax, [rdx + 0x18]
1409d2450: 48 89 85 10 0d 00 00        	mov	qword ptr [rbp + 0xd10], rax
1409d2457: 0f 57 ff                    	xorps	xmm7, xmm7
1409d245a: 48 8d b5 80 0a 00 00        	lea	rsi, [rbp + 0xa80]
1409d2461: 31 c0                       	xor	eax, eax
1409d2463: 48 8b 95 30 0d 00 00        	mov	rdx, qword ptr [rbp + 0xd30]
1409d246a: 48 83 f8 05                 	cmp	rax, 0x5
1409d246e: 0f 84 33 0a 00 00           	je	0x1409d2ea7 <.text+0x9d1ea7>
1409d2474: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1409d2480: 4c 8d 60 01                 	lea	r12, [rax + 0x1]
1409d2484: 4c 89 a5 c0 0c 00 00        	mov	qword ptr [rbp + 0xcc0], r12
1409d248b: 44 8b ac c5 d0 0c 00 00     	mov	r13d, dword ptr [rbp + 8*rax + 0xcd0]
1409d2493: 49 83 fd 05                 	cmp	r13, 0x5
1409d2497: 0f 83 8f 0a 00 00           	jae	0x1409d2f2c <.text+0x9d1f2c>
1409d249d: 44 89 e9                    	mov	ecx, r13d
1409d24a0: c1 e1 04                    	shl	ecx, 0x4
1409d24a3: f6 04 0a 01                 	test	byte ptr [rdx + rcx], 0x1
1409d24a7: 0f 84 9e 0a 00 00           	je	0x1409d2f4b <.text+0x9d1f4b>
1409d24ad: 48 8b 7c 0a 08              	mov	rdi, qword ptr [rdx + rcx + 0x8]
1409d24b2: 80 bc c5 d4 0c 00 00 00     	cmp	byte ptr [rbp + 8*rax + 0xcd4], 0x0
1409d24ba: 0f 84 a0 00 00 00           	je	0x1409d2560 <.text+0x9d1560>
1409d24c0: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d24c7: e8 84 42 21 00              	call	0x140be6750 <.text+0xbe5750>
1409d24cc: 89 c3                       	mov	ebx, eax
1409d24ce: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d24d5: 31 c9                       	xor	ecx, ecx
1409d24d7: e8 44 76 70 ff              	call	0x1400d9b20 <.text+0xd8b20>
1409d24dc: 48 85 c0                    	test	rax, rax
1409d24df: 0f 84 90 0a 00 00           	je	0x1409d2f75 <.text+0x9d1f75>
1409d24e5: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1409d24e8: 48 ff 00                    	inc	qword ptr [rax]
1409d24eb: 0f 10 0d 3e 42 0c 02        	movups	xmm1, xmmword ptr [rip + 0x20c423e] # 0x142a96730
1409d24f2: 48 8d 85 c0 06 00 00        	lea	rax, [rbp + 0x6c0]
1409d24f9: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
1409d24fd: 0f 10 0d 1c 42 0c 02        	movups	xmm1, xmmword ptr [rip + 0x20c421c] # 0x142a96720
1409d2504: 0f 11 08                    	movups	xmmword ptr [rax], xmm1
1409d2507: 41 80 cd 80                 	or	r13b, -0x80
1409d250b: 0f 10 8d 98 06 00 00        	movups	xmm1, xmmword ptr [rbp + 0x698]
1409d2512: 0f 10 95 a8 06 00 00        	movups	xmm2, xmmword ptr [rbp + 0x6a8]
1409d2519: 0f 10 9d b8 06 00 00        	movups	xmm3, xmmword ptr [rbp + 0x6b8]
1409d2520: 0f 10 a5 c8 06 00 00        	movups	xmm4, xmmword ptr [rbp + 0x6c8]
1409d2527: 0f 29 8d 70 0c 00 00        	movaps	xmmword ptr [rbp + 0xc70], xmm1
1409d252e: 0f 29 95 80 0c 00 00        	movaps	xmmword ptr [rbp + 0xc80], xmm2
1409d2535: 0f 29 9d 90 0c 00 00        	movaps	xmmword ptr [rbp + 0xc90], xmm3
1409d253c: 0f 29 a5 a0 0c 00 00        	movaps	xmmword ptr [rbp + 0xca0], xmm4
1409d2543: 48 8b 85 d8 06 00 00        	mov	rax, qword ptr [rbp + 0x6d8]
1409d254a: 48 89 85 b0 0c 00 00        	mov	qword ptr [rbp + 0xcb0], rax
1409d2551: 89 d8                       	mov	eax, ebx
1409d2553: c1 e8 18                    	shr	eax, 0x18
1409d2556: 24 9f                       	and	al, -0x61
1409d2558: 0c 20                       	or	al, 0x20
1409d255a: e9 93 00 00 00              	jmp	0x1409d25f2 <.text+0x9d15f2>
1409d255f: 90                          	nop
1409d2560: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2567: e8 e4 41 21 00              	call	0x140be6750 <.text+0xbe5750>
1409d256c: 89 c3                       	mov	ebx, eax
1409d256e: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2575: 31 c9                       	xor	ecx, ecx
1409d2577: e8 a4 75 70 ff              	call	0x1400d9b20 <.text+0xd8b20>
1409d257c: 48 85 c0                    	test	rax, rax
1409d257f: 0f 84 db 09 00 00           	je	0x1409d2f60 <.text+0x9d1f60>
1409d2585: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1409d2588: 48 ff 00                    	inc	qword ptr [rax]
1409d258b: 0f 10 0d 9e 41 0c 02        	movups	xmm1, xmmword ptr [rip + 0x20c419e] # 0x142a96730
1409d2592: 48 8d 85 c0 06 00 00        	lea	rax, [rbp + 0x6c0]
1409d2599: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
1409d259d: 0f 10 0d 7c 41 0c 02        	movups	xmm1, xmmword ptr [rip + 0x20c417c] # 0x142a96720
1409d25a4: 0f 11 08                    	movups	xmmword ptr [rax], xmm1
1409d25a7: 0f 10 8d 98 06 00 00        	movups	xmm1, xmmword ptr [rbp + 0x698]
1409d25ae: 0f 10 95 a8 06 00 00        	movups	xmm2, xmmword ptr [rbp + 0x6a8]
1409d25b5: 0f 10 9d b8 06 00 00        	movups	xmm3, xmmword ptr [rbp + 0x6b8]
1409d25bc: 0f 10 a5 c8 06 00 00        	movups	xmm4, xmmword ptr [rbp + 0x6c8]
1409d25c3: 0f 29 8d 70 0c 00 00        	movaps	xmmword ptr [rbp + 0xc70], xmm1
1409d25ca: 0f 29 95 80 0c 00 00        	movaps	xmmword ptr [rbp + 0xc80], xmm2
1409d25d1: 0f 29 9d 90 0c 00 00        	movaps	xmmword ptr [rbp + 0xc90], xmm3
1409d25d8: 0f 29 a5 a0 0c 00 00        	movaps	xmmword ptr [rbp + 0xca0], xmm4
1409d25df: 48 8b 85 d8 06 00 00        	mov	rax, qword ptr [rbp + 0x6d8]
1409d25e6: 48 89 85 b0 0c 00 00        	mov	qword ptr [rbp + 0xcb0], rax
1409d25ed: 89 d8                       	mov	eax, ebx
1409d25ef: c1 e8 18                    	shr	eax, 0x18
1409d25f2: 89 d9                       	mov	ecx, ebx
1409d25f4: 80 e1 bf                    	and	cl, -0x41
1409d25f7: c1 eb 08                    	shr	ebx, 0x8
1409d25fa: 48 c7 85 78 0b 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xb78], 0x0
1409d2605: 48 c7 85 88 0b 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xb88], 0x0
1409d2610: 48 c7 85 90 0b 00 00 04 00 00 00    	mov	qword ptr [rbp + 0xb90], 0x4
1409d261b: 4c 8d 85 98 0b 00 00        	lea	r8, [rbp + 0xb98]
1409d2622: 41 0f 11 38                 	movups	xmmword ptr [r8], xmm7
1409d2626: 48 c7 85 a8 0b 00 00 01 00 00 00    	mov	qword ptr [rbp + 0xba8], 0x1
1409d2631: 48 c7 85 b0 0b 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xbb0], 0x0
1409d263c: 48 ba 03 00 00 00 01 00 00 00       	movabs	rdx, 0x100000003
1409d2646: 48 89 95 b8 0b 00 00        	mov	qword ptr [rbp + 0xbb8], rdx
1409d264d: 48 8b 95 b0 0c 00 00        	mov	rdx, qword ptr [rbp + 0xcb0]
1409d2654: 49 89 50 68                 	mov	qword ptr [r8 + 0x68], rdx
1409d2658: 0f 28 8d 70 0c 00 00        	movaps	xmm1, xmmword ptr [rbp + 0xc70]
1409d265f: 0f 28 95 80 0c 00 00        	movaps	xmm2, xmmword ptr [rbp + 0xc80]
1409d2666: 0f 28 9d 90 0c 00 00        	movaps	xmm3, xmmword ptr [rbp + 0xc90]
1409d266d: 0f 28 a5 a0 0c 00 00        	movaps	xmm4, xmmword ptr [rbp + 0xca0]
1409d2674: 41 0f 11 60 58              	movups	xmmword ptr [r8 + 0x58], xmm4
1409d2679: 41 0f 11 58 48              	movups	xmmword ptr [r8 + 0x48], xmm3
1409d267e: 41 0f 11 50 38              	movups	xmmword ptr [r8 + 0x38], xmm2
1409d2683: 41 0f 11 48 28              	movups	xmmword ptr [r8 + 0x28], xmm1
1409d2688: 0f 11 85 08 0c 00 00        	movups	xmmword ptr [rbp + 0xc08], xmm0
1409d268f: 48 89 bd 18 0c 00 00        	mov	qword ptr [rbp + 0xc18], rdi
1409d2696: 88 8d 20 0c 00 00           	mov	byte ptr [rbp + 0xc20], cl
1409d269c: 66 89 9d 21 0c 00 00        	mov	word ptr [rbp + 0xc21], bx
1409d26a3: 88 85 23 0c 00 00           	mov	byte ptr [rbp + 0xc23], al
1409d26a9: c7 85 24 0c 00 00 00 00 80 3f       	mov	dword ptr [rbp + 0xc24], 0x3f800000
1409d26b3: c6 85 28 0c 00 00 00        	mov	byte ptr [rbp + 0xc28], 0x0
1409d26ba: 44 88 ad 29 0c 00 00        	mov	byte ptr [rbp + 0xc29], r13b
1409d26c1: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d26c8: 4c 89 f9                    	mov	rcx, r15
1409d26cb: 4c 89 f2                    	mov	rdx, r14
1409d26ce: 4c 8d 45 c8                 	lea	r8, [rbp - 0x38]
1409d26d2: 45 31 c9                    	xor	r9d, r9d
1409d26d5: e8 06 35 f3 ff              	call	0x140905be0 <.text+0x904be0>
1409d26da: 80 bd 76 07 00 00 08        	cmp	byte ptr [rbp + 0x776], 0x8
1409d26e1: 74 2c                       	je	0x1409d270f <.text+0x9d170f>
1409d26e3: 48 8b 95 80 07 00 00        	mov	rdx, qword ptr [rbp + 0x780]
1409d26ea: b8 e8 00 00 00              	mov	eax, 0xe8
1409d26ef: 48 01 c2                    	add	rdx, rax
1409d26f2: 48 8d 8d 50 07 00 00        	lea	rcx, [rbp + 0x750]
1409d26f9: 49 b8 27 a0 52 34 94 83 88 57       	movabs	r8, 0x578883943452a027
1409d2703: 44 8b 8d 3c 0d 00 00        	mov	r9d, dword ptr [rbp + 0xd3c]
1409d270a: e8 61 8e f9 ff              	call	0x14096b570 <.text+0x96a570>
1409d270f: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2715: 4c 89 f1                    	mov	rcx, r14
1409d2718: 4c 89 fa                    	mov	rdx, r15
1409d271b: e8 e0 14 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2720: 48 8b 85 20 0d 00 00        	mov	rax, qword ptr [rbp + 0xd20]
1409d2727: 48 89 85 48 0d 00 00        	mov	qword ptr [rbp + 0xd48], rax
1409d272e: c7 85 98 06 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x698], 0x2
1409d2738: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1409d273d: 48 8d bd 48 0d 00 00        	lea	rdi, [rbp + 0xd48]
1409d2744: 48 89 f9                    	mov	rcx, rdi
1409d2747: 4c 89 f2                    	mov	rdx, r14
1409d274a: 49 b8 36 0b ea 2d 0d 4a aa 41       	movabs	r8, 0x41aa4a0d2dea0b36
1409d2754: 41 b1 1d                    	mov	r9b, 0x1d
1409d2757: e8 a4 c0 f1 ff              	call	0x1408ee800 <.text+0x8ed800>
1409d275c: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2762: 4c 89 f9                    	mov	rcx, r15
1409d2765: 4c 89 f2                    	mov	rdx, r14
1409d2768: e8 93 14 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d276d: 48 8b 85 f8 0c 00 00        	mov	rax, qword ptr [rbp + 0xcf8]
1409d2774: 48 89 85 48 0d 00 00        	mov	qword ptr [rbp + 0xd48], rax
1409d277b: c7 85 78 0b 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xb78], 0x2
1409d2785: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1409d278a: 48 89 f9                    	mov	rcx, rdi
1409d278d: 4c 89 fa                    	mov	rdx, r15
1409d2790: 49 b8 fc fb 49 a0 9d 57 61 48       	movabs	r8, 0x4861579da049fbfc
1409d279a: 41 b1 1d                    	mov	r9b, 0x1d
1409d279d: e8 5e c0 f1 ff              	call	0x1408ee800 <.text+0x8ed800>
1409d27a2: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d27a8: 4c 89 f1                    	mov	rcx, r14
1409d27ab: 4c 89 fa                    	mov	rdx, r15
1409d27ae: e8 4d 14 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d27b3: 48 8b 85 08 0e 00 00        	mov	rax, qword ptr [rbp + 0xe08]
1409d27ba: 48 89 85 48 0d 00 00        	mov	qword ptr [rbp + 0xd48], rax
1409d27c1: c7 85 98 06 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x698], 0x2
1409d27cb: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1409d27d0: 48 89 f9                    	mov	rcx, rdi
1409d27d3: 4c 89 f2                    	mov	rdx, r14
1409d27d6: 49 b8 30 82 02 28 3a 3c 33 2a       	movabs	r8, 0x2a333c3a28028230
1409d27e0: 41 b1 1d                    	mov	r9b, 0x1d
1409d27e3: e8 18 c0 f1 ff              	call	0x1408ee800 <.text+0x8ed800>
1409d27e8: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d27ee: 4c 89 f9                    	mov	rcx, r15
1409d27f1: 4c 89 f2                    	mov	rdx, r14
1409d27f4: e8 07 14 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d27f9: 48 8b 85 f0 0d 00 00        	mov	rax, qword ptr [rbp + 0xdf0]
1409d2800: 48 89 85 48 0d 00 00        	mov	qword ptr [rbp + 0xd48], rax
1409d2807: c7 85 78 0b 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xb78], 0x2
1409d2811: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1409d2816: 48 89 f9                    	mov	rcx, rdi
1409d2819: 4c 89 fa                    	mov	rdx, r15
1409d281c: 49 b8 d0 09 6c 82 85 51 17 04       	movabs	r8, 0x4175185826c09d0
1409d2826: 41 b1 1d                    	mov	r9b, 0x1d
1409d2829: e8 d2 bf f1 ff              	call	0x1408ee800 <.text+0x8ed800>
1409d282e: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2834: 4c 89 f1                    	mov	rcx, r14
1409d2837: 4c 89 fa                    	mov	rdx, r15
1409d283a: e8 c1 13 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d283f: 48 b8 65 18 a0 91 ab 39 9f 57       	movabs	rax, 0x579f39ab91a01865
1409d2849: 48 89 85 48 0d 00 00        	mov	qword ptr [rbp + 0xd48], rax
1409d2850: c7 85 98 06 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x698], 0x2
1409d285a: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1409d285f: 48 89 f9                    	mov	rcx, rdi
1409d2862: 4c 89 f2                    	mov	rdx, r14
1409d2865: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
1409d286f: 41 b1 1d                    	mov	r9b, 0x1d
1409d2872: e8 99 5d f3 ff              	call	0x140908610 <.text+0x907610>
1409d2877: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d287d: 4c 89 f9                    	mov	rcx, r15
1409d2880: 4c 89 f2                    	mov	rdx, r14
1409d2883: e8 78 13 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2888: 48 8b 85 00 0e 00 00        	mov	rax, qword ptr [rbp + 0xe00]
1409d288f: 48 89 85 48 0d 00 00        	mov	qword ptr [rbp + 0xd48], rax
1409d2896: c7 85 78 0b 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xb78], 0x2
1409d28a0: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1409d28a5: 48 89 f9                    	mov	rcx, rdi
1409d28a8: 4c 89 fa                    	mov	rdx, r15
1409d28ab: 49 b8 f9 9a 39 64 bb e2 03 e0       	movabs	r8, -0x1ffc1d449bc66507
1409d28b5: 41 b1 1d                    	mov	r9b, 0x1d
1409d28b8: e8 43 bf f1 ff              	call	0x1408ee800 <.text+0x8ed800>
1409d28bd: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d28c3: 4c 89 f1                    	mov	rcx, r14
1409d28c6: 4c 89 fa                    	mov	rdx, r15
1409d28c9: e8 32 13 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d28ce: c6 85 48 0d 00 00 08        	mov	byte ptr [rbp + 0xd48], 0x8
1409d28d5: c7 85 98 06 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x698], 0x2
1409d28df: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1409d28e4: 48 89 f9                    	mov	rcx, rdi
1409d28e7: 4c 89 f2                    	mov	rdx, r14
1409d28ea: 49 b8 3f ca 8d ac 13 01 16 77       	movabs	r8, 0x77160113ac8dca3f
1409d28f4: 41 b1 1d                    	mov	r9b, 0x1d
1409d28f7: e8 54 63 f3 ff              	call	0x140908c50 <.text+0x907c50>
1409d28fc: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2902: 4c 89 f9                    	mov	rcx, r15
1409d2905: 4c 89 f2                    	mov	rdx, r14
1409d2908: e8 f3 12 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d290d: c6 85 48 0d 00 00 0c        	mov	byte ptr [rbp + 0xd48], 0xc
1409d2914: c7 85 78 0b 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xb78], 0x2
1409d291e: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1409d2923: 48 89 f9                    	mov	rcx, rdi
1409d2926: 4c 89 fa                    	mov	rdx, r15
1409d2929: 49 b8 f4 b0 60 70 5c 0a 87 8f       	movabs	r8, -0x7078f5a38f9f4f0c
1409d2933: 41 b1 1d                    	mov	r9b, 0x1d
1409d2936: e8 15 63 f3 ff              	call	0x140908c50 <.text+0x907c50>
1409d293b: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2941: 48 89 f1                    	mov	rcx, rsi
1409d2944: 4c 89 fa                    	mov	rdx, r15
1409d2947: e8 b4 12 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d294c: 80 bd 5e 0b 00 00 08        	cmp	byte ptr [rbp + 0xb5e], 0x8
1409d2953: 74 28                       	je	0x1409d297d <.text+0x9d197d>
1409d2955: 48 8b 95 68 0b 00 00        	mov	rdx, qword ptr [rbp + 0xb68]
1409d295c: b8 e8 00 00 00              	mov	eax, 0xe8
1409d2961: 48 01 c2                    	add	rdx, rax
1409d2964: 48 8d 8d 38 0b 00 00        	lea	rcx, [rbp + 0xb38]
1409d296b: 49 b8 4c a5 59 1f 44 98 8a 41       	movabs	r8, 0x418a98441f59a54c
1409d2975: 41 b1 01                    	mov	r9b, 0x1
1409d2978: e8 d3 8e f9 ff              	call	0x14096b850 <.text+0x96a850>
1409d297d: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2983: 4c 89 f9                    	mov	rcx, r15
1409d2986: 48 89 f2                    	mov	rdx, rsi
1409d2989: e8 72 12 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d298e: c6 85 48 0d 00 00 11        	mov	byte ptr [rbp + 0xd48], 0x11
1409d2995: c7 85 78 0b 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xb78], 0x2
1409d299f: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1409d29a4: 48 89 f9                    	mov	rcx, rdi
1409d29a7: 4c 89 fa                    	mov	rdx, r15
1409d29aa: 49 b8 a4 1a 84 c9 4d 1f 65 11       	movabs	r8, 0x11651f4dc9841aa4
1409d29b4: 41 b1 1d                    	mov	r9b, 0x1d
1409d29b7: e8 94 62 f3 ff              	call	0x140908c50 <.text+0x907c50>
1409d29bc: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d29c2: 4c 89 f1                    	mov	rcx, r14
1409d29c5: 4c 89 fa                    	mov	rdx, r15
1409d29c8: e8 33 12 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d29cd: 48 8b 85 20 0e 00 00        	mov	rax, qword ptr [rbp + 0xe20]
1409d29d4: 48 89 85 80 0a 00 00        	mov	qword ptr [rbp + 0xa80], rax
1409d29db: c7 85 98 06 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x698], 0x2
1409d29e5: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1409d29ea: 48 89 f1                    	mov	rcx, rsi
1409d29ed: 4c 89 f2                    	mov	rdx, r14
1409d29f0: 49 b8 25 6d d6 56 7e 43 41 99       	movabs	r8, -0x66bebc81a92992db
1409d29fa: 41 b1 1d                    	mov	r9b, 0x1d
1409d29fd: e8 fe bd f1 ff              	call	0x1408ee800 <.text+0x8ed800>
1409d2a02: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2a08: 4c 89 f9                    	mov	rcx, r15
1409d2a0b: 4c 89 f2                    	mov	rdx, r14
1409d2a0e: e8 ed 11 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2a13: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2a1a: 48 8d 8d a8 04 00 00        	lea	rcx, [rbp + 0x4a8]
1409d2a21: 4c 89 fa                    	mov	rdx, r15
1409d2a24: 49 b8 84 93 9f 2e ef 85 54 1e       	movabs	r8, 0x1e5485ef2e9f9384
1409d2a2e: 0f 28 de                    	movaps	xmm3, xmm6
1409d2a31: e8 ea 6f c5 00              	call	0x141629a20 <.text+0x1628a20>
1409d2a36: 80 bd 86 05 00 00 08        	cmp	byte ptr [rbp + 0x586], 0x8
1409d2a3d: 74 28                       	je	0x1409d2a67 <.text+0x9d1a67>
1409d2a3f: 48 8b 95 90 05 00 00        	mov	rdx, qword ptr [rbp + 0x590]
1409d2a46: b8 e8 00 00 00              	mov	eax, 0xe8
1409d2a4b: 48 01 c2                    	add	rdx, rax
1409d2a4e: 48 8d 8d 60 05 00 00        	lea	rcx, [rbp + 0x560]
1409d2a55: 49 b8 f5 51 86 b0 3f 67 ab c5       	movabs	r8, -0x3a5498c04f79ae0b
1409d2a5f: 45 31 c9                    	xor	r9d, r9d
1409d2a62: e8 09 8b f9 ff              	call	0x14096b570 <.text+0x96a570>
1409d2a67: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2a6d: 4c 89 f9                    	mov	rcx, r15
1409d2a70: 48 8d 95 a8 04 00 00        	lea	rdx, [rbp + 0x4a8]
1409d2a77: e8 84 11 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2a7c: c6 85 56 0d 00 00 01        	mov	byte ptr [rbp + 0xd56], 0x1
1409d2a83: 48 8b 8d 40 0d 00 00        	mov	rcx, qword ptr [rbp + 0xd40]
1409d2a8a: e8 01 2d 20 00              	call	0x140bd5790 <.text+0xbd4790>
1409d2a8f: c6 85 56 0d 00 00 00        	mov	byte ptr [rbp + 0xd56], 0x0
1409d2a96: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
1409d2a9d: 4c 89 fa                    	mov	rdx, r15
1409d2aa0: 49 b8 71 49 ea 8e 31 b9 1b 53       	movabs	r8, 0x531bb9318eea4971
1409d2aaa: 49 89 c1                    	mov	r9, rax
1409d2aad: e8 6e 72 c5 00              	call	0x141629d20 <.text+0x1628d20>
1409d2ab2: 80 bd 8e 04 00 00 08        	cmp	byte ptr [rbp + 0x48e], 0x8
1409d2ab9: 74 2c                       	je	0x1409d2ae7 <.text+0x9d1ae7>
1409d2abb: 48 8b 95 98 04 00 00        	mov	rdx, qword ptr [rbp + 0x498]
1409d2ac2: b8 e8 00 00 00              	mov	eax, 0xe8
1409d2ac7: 48 01 c2                    	add	rdx, rax
1409d2aca: 48 8d 8d 68 04 00 00        	lea	rcx, [rbp + 0x468]
1409d2ad1: 49 b8 9f f6 8f 40 9e 32 88 9b       	movabs	r8, -0x6477cd61bf700961
1409d2adb: 4c 8b 8d 18 0d 00 00        	mov	r9, qword ptr [rbp + 0xd18]
1409d2ae2: e8 89 8a f9 ff              	call	0x14096b570 <.text+0x96a570>
1409d2ae7: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2aed: 4c 89 f9                    	mov	rcx, r15
1409d2af0: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
1409d2af7: e8 04 11 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2afc: 48 8b 85 18 0e 00 00        	mov	rax, qword ptr [rbp + 0xe18]
1409d2b03: 48 89 85 80 0a 00 00        	mov	qword ptr [rbp + 0xa80], rax
1409d2b0a: c7 85 78 0b 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xb78], 0x2
1409d2b14: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1409d2b19: 48 89 f1                    	mov	rcx, rsi
1409d2b1c: 4c 89 fa                    	mov	rdx, r15
1409d2b1f: 49 b8 d7 74 57 02 af 6c 3e ba       	movabs	r8, -0x45c19350fda88b29
1409d2b29: 41 b1 1d                    	mov	r9b, 0x1d
1409d2b2c: e8 cf bc f1 ff              	call	0x1408ee800 <.text+0x8ed800>
1409d2b31: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2b37: 4c 89 f1                    	mov	rcx, r14
1409d2b3a: 4c 89 fa                    	mov	rdx, r15
1409d2b3d: e8 be 10 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2b42: 48 8b 85 10 0e 00 00        	mov	rax, qword ptr [rbp + 0xe10]
1409d2b49: 48 89 85 80 0a 00 00        	mov	qword ptr [rbp + 0xa80], rax
1409d2b50: c7 85 98 06 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x698], 0x2
1409d2b5a: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1409d2b5f: 48 89 f1                    	mov	rcx, rsi
1409d2b62: 4c 89 f2                    	mov	rdx, r14
1409d2b65: 49 b8 aa 9d 74 97 9f a5 8d fe       	movabs	r8, -0x1725a60688b6256
1409d2b6f: 41 b1 1d                    	mov	r9b, 0x1d
1409d2b72: e8 89 bc f1 ff              	call	0x1408ee800 <.text+0x8ed800>
1409d2b77: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2b7d: 4c 89 f9                    	mov	rcx, r15
1409d2b80: 4c 89 f2                    	mov	rdx, r14
1409d2b83: e8 78 10 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2b88: 48 8b 85 00 0d 00 00        	mov	rax, qword ptr [rbp + 0xd00]
1409d2b8f: 48 89 85 80 0a 00 00        	mov	qword ptr [rbp + 0xa80], rax
1409d2b96: c7 85 78 0b 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xb78], 0x2
1409d2ba0: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1409d2ba5: 48 89 f1                    	mov	rcx, rsi
1409d2ba8: 4c 89 fa                    	mov	rdx, r15
1409d2bab: 49 b8 f1 9d a2 2b 99 dd 36 0d       	movabs	r8, 0xd36dd992ba29df1
1409d2bb5: 41 b1 1d                    	mov	r9b, 0x1d
1409d2bb8: e8 43 bc f1 ff              	call	0x1408ee800 <.text+0x8ed800>
1409d2bbd: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2bc3: 4c 89 f1                    	mov	rcx, r14
1409d2bc6: 4c 89 fa                    	mov	rdx, r15
1409d2bc9: e8 32 10 f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2bce: 48 8b 85 08 0d 00 00        	mov	rax, qword ptr [rbp + 0xd08]
1409d2bd5: 48 89 85 80 0a 00 00        	mov	qword ptr [rbp + 0xa80], rax
1409d2bdc: c7 85 98 06 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x698], 0x2
1409d2be6: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1409d2beb: 48 89 f1                    	mov	rcx, rsi
1409d2bee: 4c 89 f2                    	mov	rdx, r14
1409d2bf1: 49 b8 f0 4d f1 36 d2 8b 1b 4b       	movabs	r8, 0x4b1b8bd236f14df0
1409d2bfb: 41 b1 1d                    	mov	r9b, 0x1d
1409d2bfe: e8 fd bb f1 ff              	call	0x1408ee800 <.text+0x8ed800>
1409d2c03: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2c09: 4c 89 f9                    	mov	rcx, r15
1409d2c0c: 4c 89 f2                    	mov	rdx, r14
1409d2c0f: e8 ec 0f f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2c14: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2c1b: 4c 89 f1                    	mov	rcx, r14
1409d2c1e: 4c 89 fa                    	mov	rdx, r15
1409d2c21: 49 b8 c3 15 a9 0b 0c d8 5a bc       	movabs	r8, -0x43a527f3f456ea3d
1409d2c2b: e8 60 de c5 00              	call	0x141630a90 <.text+0x162fa90>
1409d2c30: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2c37: 4c 89 f9                    	mov	rcx, r15
1409d2c3a: 4c 89 f2                    	mov	rdx, r14
1409d2c3d: 49 b8 ad 2a 99 20 eb 33 6b 0c       	movabs	r8, 0xc6b33eb20992aad
1409d2c47: e8 44 7b c5 00              	call	0x14162a790 <.text+0x1629790>
1409d2c4c: 48 8b 85 28 0e 00 00        	mov	rax, qword ptr [rbp + 0xe28]
1409d2c53: 48 89 85 80 0a 00 00        	mov	qword ptr [rbp + 0xa80], rax
1409d2c5a: c7 85 78 0b 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xb78], 0x2
1409d2c64: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1409d2c69: 48 89 f1                    	mov	rcx, rsi
1409d2c6c: 4c 89 fa                    	mov	rdx, r15
1409d2c6f: 49 b8 fe 6f 11 26 70 34 67 5a       	movabs	r8, 0x5a67347026116ffe
1409d2c79: 41 b1 1d                    	mov	r9b, 0x1d
1409d2c7c: e8 7f bb f1 ff              	call	0x1408ee800 <.text+0x8ed800>
1409d2c81: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2c87: 4c 89 f1                    	mov	rcx, r14
1409d2c8a: 4c 89 fa                    	mov	rdx, r15
1409d2c8d: e8 6e 0f f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2c92: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2c99: 4c 89 f9                    	mov	rcx, r15
1409d2c9c: 4c 89 f2                    	mov	rdx, r14
1409d2c9f: 49 b8 e1 c8 87 90 28 ac d0 2b       	movabs	r8, 0x2bd0ac289087c8e1
1409d2ca9: 4c 8b 8d 30 0e 00 00        	mov	r9, qword ptr [rbp + 0xe30]
1409d2cb0: e8 9b 34 f3 ff              	call	0x140906150 <.text+0x905150>
1409d2cb5: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2cbc: 4c 89 f1                    	mov	rcx, r14
1409d2cbf: 4c 89 fa                    	mov	rdx, r15
1409d2cc2: 49 b8 a0 06 54 8d ce 01 9f 12       	movabs	r8, 0x129f01ce8d5406a0
1409d2ccc: 4c 8b 8d 10 0d 00 00        	mov	r9, qword ptr [rbp + 0xd10]
1409d2cd3: e8 78 34 f3 ff              	call	0x140906150 <.text+0x905150>
1409d2cd8: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2cdf: 4c 89 f9                    	mov	rcx, r15
1409d2ce2: 4c 89 f2                    	mov	rdx, r14
1409d2ce5: e8 06 5a c5 00              	call	0x1416286f0 <.text+0x16276f0>
1409d2cea: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2cf1: 4c 89 f1                    	mov	rcx, r14
1409d2cf4: 4c 89 fa                    	mov	rdx, r15
1409d2cf7: 4c 8b 85 28 0d 00 00        	mov	r8, qword ptr [rbp + 0xd28]
1409d2cfe: e8 cd 5c c5 00              	call	0x1416289d0 <.text+0x16279d0>
1409d2d03: c6 85 80 0a 00 00 09        	mov	byte ptr [rbp + 0xa80], 0x9
1409d2d0a: c7 85 98 06 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x698], 0x2
1409d2d14: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1409d2d19: 48 89 f1                    	mov	rcx, rsi
1409d2d1c: 4c 89 f2                    	mov	rdx, r14
1409d2d1f: 49 b8 f2 c5 70 04 5e 8b b0 61       	movabs	r8, 0x61b08b5e0470c5f2
1409d2d29: 41 b1 1d                    	mov	r9b, 0x1d
1409d2d2c: e8 1f 5f f3 ff              	call	0x140908c50 <.text+0x907c50>
1409d2d31: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2d37: 4c 89 f9                    	mov	rcx, r15
1409d2d3a: 4c 89 f2                    	mov	rdx, r14
1409d2d3d: e8 be 0e f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2d42: c6 85 a8 04 00 00 0a        	mov	byte ptr [rbp + 0x4a8], 0xa
1409d2d49: c7 85 80 0a 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xa80], 0x2
1409d2d53: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409d2d58: 48 8d 8d a8 04 00 00        	lea	rcx, [rbp + 0x4a8]
1409d2d5f: 4c 89 fa                    	mov	rdx, r15
1409d2d62: 49 b8 57 f6 e8 da 4c 1e 7d 29       	movabs	r8, 0x297d1e4cdae8f657
1409d2d6c: 41 b1 1d                    	mov	r9b, 0x1d
1409d2d6f: e8 dc 5e f3 ff              	call	0x140908c50 <.text+0x907c50>
1409d2d74: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409d2d7a: 48 8d bd a0 05 00 00        	lea	rdi, [rbp + 0x5a0]
1409d2d81: 48 89 f9                    	mov	rcx, rdi
1409d2d84: 4c 89 fa                    	mov	rdx, r15
1409d2d87: e8 74 0e f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2d8c: c6 85 78 0b 00 00 00        	mov	byte ptr [rbp + 0xb78], 0x0
1409d2d93: c7 85 98 06 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x698], 0x2
1409d2d9d: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1409d2da2: 4c 89 f1                    	mov	rcx, r14
1409d2da5: 48 89 fa                    	mov	rdx, rdi
1409d2da8: 49 b8 45 41 25 6a 8a 1a 9d b7       	movabs	r8, -0x4862e57595dabebb
1409d2db2: 41 b1 1d                    	mov	r9b, 0x1d
1409d2db5: e8 96 5e f3 ff              	call	0x140908c50 <.text+0x907c50>
1409d2dba: 48 8b bd a0 05 00 00        	mov	rdi, qword ptr [rbp + 0x5a0]
1409d2dc1: 41 b8 a0 00 00 00           	mov	r8d, 0xa0
1409d2dc7: 48 8d 8d a0 06 00 00        	lea	rcx, [rbp + 0x6a0]
1409d2dce: 48 8d 9d a8 05 00 00        	lea	rbx, [rbp + 0x5a8]
1409d2dd5: 48 89 da                    	mov	rdx, rbx
1409d2dd8: e8 23 0e f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2ddd: 0f b6 85 48 06 00 00        	movzx	eax, byte ptr [rbp + 0x648]
1409d2de4: 0f b6 8d 49 06 00 00        	movzx	ecx, byte ptr [rbp + 0x649]
1409d2deb: 0f b6 95 4a 06 00 00        	movzx	edx, byte ptr [rbp + 0x64a]
1409d2df2: 44 0f b6 85 4b 06 00 00     	movzx	r8d, byte ptr [rbp + 0x64b]
1409d2dfa: 0f 10 83 e0 00 00 00        	movups	xmm0, xmmword ptr [rbx + 0xe0]
1409d2e01: 4c 8d 8d c0 06 00 00        	lea	r9, [rbp + 0x6c0]
1409d2e08: 41 0f 11 81 c0 00 00 00     	movups	xmmword ptr [r9 + 0xc0], xmm0
1409d2e10: 0f 10 83 a4 00 00 00        	movups	xmm0, xmmword ptr [rbx + 0xa4]
1409d2e17: 0f 10 8b b4 00 00 00        	movups	xmm1, xmmword ptr [rbx + 0xb4]
1409d2e1e: 0f 10 93 c4 00 00 00        	movups	xmm2, xmmword ptr [rbx + 0xc4]
1409d2e25: 0f 10 9b d4 00 00 00        	movups	xmm3, xmmword ptr [rbx + 0xd4]
1409d2e2c: 41 0f 11 99 b4 00 00 00     	movups	xmmword ptr [r9 + 0xb4], xmm3
1409d2e34: 41 0f 11 91 a4 00 00 00     	movups	xmmword ptr [r9 + 0xa4], xmm2
1409d2e3c: 41 0f 11 89 94 00 00 00     	movups	xmmword ptr [r9 + 0x94], xmm1
1409d2e44: 41 0f 11 81 84 00 00 00     	movups	xmmword ptr [r9 + 0x84], xmm0
1409d2e4c: 24 7f                       	and	al, 0x7f
1409d2e4e: 80 c9 01                    	or	cl, 0x1
1409d2e51: 80 e2 03                    	and	dl, 0x3
1409d2e54: 80 ca e8                    	or	dl, -0x18
1409d2e57: 41 80 e0 e0                 	and	r8b, -0x20
1409d2e5b: 41 80 c8 10                 	or	r8b, 0x10
1409d2e5f: 48 89 bd 98 06 00 00        	mov	qword ptr [rbp + 0x698], rdi
1409d2e66: 88 85 40 07 00 00           	mov	byte ptr [rbp + 0x740], al
1409d2e6c: 88 8d 41 07 00 00           	mov	byte ptr [rbp + 0x741], cl
1409d2e72: 88 95 42 07 00 00           	mov	byte ptr [rbp + 0x742], dl
1409d2e78: 44 88 85 43 07 00 00        	mov	byte ptr [rbp + 0x743], r8b
1409d2e7f: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2e86: ba 03 00 00 00              	mov	edx, 0x3
1409d2e8b: 4c 89 f9                    	mov	rcx, r15
1409d2e8e: e8 fd 3b f3 ff              	call	0x140906a90 <.text+0x905a90>
1409d2e93: 4c 89 e0                    	mov	rax, r12
1409d2e96: 48 8b 95 30 0d 00 00        	mov	rdx, qword ptr [rbp + 0xd30]
1409d2e9d: 48 83 f8 05                 	cmp	rax, 0x5
1409d2ea1: 0f 85 d9 f5 ff ff           	jne	0x1409d2480 <.text+0x9d1480>
1409d2ea7: 48 8d b5 98 06 00 00        	lea	rsi, [rbp + 0x698]
1409d2eae: 48 8d 55 c8                 	lea	rdx, [rbp - 0x38]
1409d2eb2: 41 b8 e8 03 00 00           	mov	r8d, 0x3e8
1409d2eb8: 48 89 f1                    	mov	rcx, rsi
1409d2ebb: e8 40 0d f5 01              	call	0x142923c00 <.text+0x2922c00>
1409d2ec0: c6 85 57 0d 00 00 00        	mov	byte ptr [rbp + 0xd57], 0x0
1409d2ec7: 48 89 f1                    	mov	rcx, rsi
1409d2eca: e8 a1 54 e9 ff              	call	0x140868370 <.text+0x867370>
1409d2ecf: 0f 28 b5 60 0d 00 00        	movaps	xmm6, xmmword ptr [rbp + 0xd60]
1409d2ed6: 0f 28 bd 70 0d 00 00        	movaps	xmm7, xmmword ptr [rbp + 0xd70]
1409d2edd: 48 81 c4 08 0e 00 00        	add	rsp, 0xe08
1409d2ee4: 5b                          	pop	rbx
1409d2ee5: 5f                          	pop	rdi
1409d2ee6: 5e                          	pop	rsi
1409d2ee7: 41 5c                       	pop	r12
1409d2ee9: 41 5d                       	pop	r13
1409d2eeb: 41 5e                       	pop	r14
1409d2eed: 41 5f                       	pop	r15
1409d2eef: 5d                          	pop	rbp
1409d2ef0: c3                          	ret
1409d2ef1: 48 c7 85 98 06 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x698], 0x0
1409d2efc: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2f03: 48 8d 05 76 b2 25 02        	lea	rax, [rip + 0x225b276]  # 0x142c2e180
1409d2f0a: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1409d2f0f: 48 8d 95 78 0b 00 00        	lea	rdx, [rbp + 0xb78]
1409d2f16: 4c 8d 8d 98 06 00 00        	lea	r9, [rbp + 0x698]
1409d2f1d: 31 c9                       	xor	ecx, ecx
1409d2f1f: 49 89 f8                    	mov	r8, rdi
1409d2f22: e8 1a 7c ef 01              	call	0x1428cab41 <.text+0x28c9b41>
1409d2f27: e9 9e 00 00 00              	jmp	0x1409d2fca <.text+0x9d1fca>
1409d2f2c: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2f33: 4c 8d 05 9e e9 0e 02        	lea	r8, [rip + 0x20ee99e]   # 0x142ac18d8
1409d2f3a: b9 05 00 00 00              	mov	ecx, 0x5
1409d2f3f: ba 05 00 00 00              	mov	edx, 0x5
1409d2f44: e8 cf 65 f0 01              	call	0x1428d9518 <.text+0x28d8518>
1409d2f49: eb 7f                       	jmp	0x1409d2fca <.text+0x9d1fca>
1409d2f4b: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2f52: 48 8d 0d 97 e9 0e 02        	lea	rcx, [rip + 0x20ee997]  # 0x142ac18f0
1409d2f59: e8 b2 63 f0 01              	call	0x1428d9310 <.text+0x28d8310>
1409d2f5e: eb 6a                       	jmp	0x1409d2fca <.text+0x9d1fca>
1409d2f60: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2f67: 48 8d 0d 32 7b 26 02        	lea	rcx, [rip + 0x2267b32]  # 0x142c3aaa0
1409d2f6e: e8 5d 51 f0 01              	call	0x1428d80d0 <.text+0x28d70d0>
1409d2f73: eb 55                       	jmp	0x1409d2fca <.text+0x9d1fca>
1409d2f75: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2f7c: 48 8d 0d 1d 7b 26 02        	lea	rcx, [rip + 0x2267b1d]  # 0x142c3aaa0
1409d2f83: e8 48 51 f0 01              	call	0x1428d80d0 <.text+0x28d70d0>
1409d2f88: eb 40                       	jmp	0x1409d2fca <.text+0x9d1fca>
1409d2f8a: 48 8d 0d cf e8 0e 02        	lea	rcx, [rip + 0x20ee8cf]  # 0x142ac1860
1409d2f91: e8 7a 63 f0 01              	call	0x1428d9310 <.text+0x28d8310>
1409d2f96: 48 8d 0d db e8 0e 02        	lea	rcx, [rip + 0x20ee8db]  # 0x142ac1878
1409d2f9d: e8 6e 63 f0 01              	call	0x1428d9310 <.text+0x28d8310>
1409d2fa2: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2fa9: 48 8d 0d f8 e8 0e 02        	lea	rcx, [rip + 0x20ee8f8]  # 0x142ac18a8
1409d2fb0: e8 5b 63 f0 01              	call	0x1428d9310 <.text+0x28d8310>
1409d2fb5: eb 13                       	jmp	0x1409d2fca <.text+0x9d1fca>
1409d2fb7: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d2fbe: 48 8d 0d fb e8 0e 02        	lea	rcx, [rip + 0x20ee8fb]  # 0x142ac18c0
1409d2fc5: e8 46 63 f0 01              	call	0x1428d9310 <.text+0x28d8310>
1409d2fca: 0f 0b                       	ud2
1409d2fcc: 0f 1f 40 00                 	nop	dword ptr [rax]
1409d2fd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d2fd5: 55                          	push	rbp
1409d2fd6: 41 57                       	push	r15
1409d2fd8: 41 56                       	push	r14
1409d2fda: 41 55                       	push	r13
1409d2fdc: 41 54                       	push	r12
1409d2fde: 56                          	push	rsi
1409d2fdf: 57                          	push	rdi
1409d2fe0: 53                          	push	rbx
1409d2fe1: 48 83 ec 68                 	sub	rsp, 0x68
1409d2fe5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d2fec: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d2ff1: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d2ff6: 80 bd 56 0d 00 00 00        	cmp	byte ptr [rbp + 0xd56], 0x0
1409d2ffd: 74 2e                       	je	0x1409d302d <.text+0x9d202d>
1409d2fff: 48 8d 8d 98 06 00 00        	lea	rcx, [rbp + 0x698]
1409d3006: e8 b5 05 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d300b: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3012: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3017: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d301c: 48 83 c4 68                 	add	rsp, 0x68
1409d3020: 5b                          	pop	rbx
1409d3021: 5f                          	pop	rdi
1409d3022: 5e                          	pop	rsi
1409d3023: 41 5c                       	pop	r12
1409d3025: 41 5d                       	pop	r13
1409d3027: 41 5e                       	pop	r14
1409d3029: 41 5f                       	pop	r15
1409d302b: 5d                          	pop	rbp
1409d302c: c3                          	ret
1409d302d: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3034: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3039: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d303e: 48 83 c4 68                 	add	rsp, 0x68
1409d3042: 5b                          	pop	rbx
1409d3043: 5f                          	pop	rdi
1409d3044: 5e                          	pop	rsi
1409d3045: 41 5c                       	pop	r12
1409d3047: 41 5d                       	pop	r13
1409d3049: 41 5e                       	pop	r14
1409d304b: 41 5f                       	pop	r15
1409d304d: 5d                          	pop	rbp
1409d304e: c3                          	ret
1409d304f: 90                          	nop
1409d3050: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3055: 55                          	push	rbp
1409d3056: 41 57                       	push	r15
1409d3058: 41 56                       	push	r14
1409d305a: 41 55                       	push	r13
1409d305c: 41 54                       	push	r12
1409d305e: 56                          	push	rsi
1409d305f: 57                          	push	rdi
1409d3060: 53                          	push	rbx
1409d3061: 48 83 ec 68                 	sub	rsp, 0x68
1409d3065: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d306c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3071: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3076: 80 bd 57 0d 00 00 00        	cmp	byte ptr [rbp + 0xd57], 0x0
1409d307d: 75 1b                       	jne	0x1409d309a <.text+0x9d209a>
1409d307f: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3084: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d3089: 48 83 c4 68                 	add	rsp, 0x68
1409d308d: 5b                          	pop	rbx
1409d308e: 5f                          	pop	rdi
1409d308f: 5e                          	pop	rsi
1409d3090: 41 5c                       	pop	r12
1409d3092: 41 5d                       	pop	r13
1409d3094: 41 5e                       	pop	r14
1409d3096: 41 5f                       	pop	r15
1409d3098: 5d                          	pop	rbp
1409d3099: c3                          	ret
1409d309a: 48 8d 4d c8                 	lea	rcx, [rbp - 0x38]
1409d309e: e8 1d f3 eb ff              	call	0x1408923c0 <.text+0x8913c0>
1409d30a3: eb da                       	jmp	0x1409d307f <.text+0x9d207f>
1409d30a5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1409d30b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d30b5: 55                          	push	rbp
1409d30b6: 41 57                       	push	r15
1409d30b8: 41 56                       	push	r14
1409d30ba: 41 55                       	push	r13
1409d30bc: 41 54                       	push	r12
1409d30be: 56                          	push	rsi
1409d30bf: 57                          	push	rdi
1409d30c0: 53                          	push	rbx
1409d30c1: 48 83 ec 68                 	sub	rsp, 0x68
1409d30c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d30cc: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d30d1: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d30d6: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d30dd: 48 8d 8d 98 06 00 00        	lea	rcx, [rbp + 0x698]
1409d30e4: e8 d7 04 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d30e9: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d30f0: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d30f5: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d30fa: 48 83 c4 68                 	add	rsp, 0x68
1409d30fe: 5b                          	pop	rbx
1409d30ff: 5f                          	pop	rdi
1409d3100: 5e                          	pop	rsi
1409d3101: 41 5c                       	pop	r12
1409d3103: 41 5d                       	pop	r13
1409d3105: 41 5e                       	pop	r14
1409d3107: 41 5f                       	pop	r15
1409d3109: 5d                          	pop	rbp
1409d310a: c3                          	ret
1409d310b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3110: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3115: 55                          	push	rbp
1409d3116: 41 57                       	push	r15
1409d3118: 41 56                       	push	r14
1409d311a: 41 55                       	push	r13
1409d311c: 41 54                       	push	r12
1409d311e: 56                          	push	rsi
1409d311f: 57                          	push	rdi
1409d3120: 53                          	push	rbx
1409d3121: 48 83 ec 68                 	sub	rsp, 0x68
1409d3125: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d312c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3131: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3136: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d313d: 48 8d 8d 78 0b 00 00        	lea	rcx, [rbp + 0xb78]
1409d3144: e8 77 04 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d3149: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3150: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3155: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d315a: 48 83 c4 68                 	add	rsp, 0x68
1409d315e: 5b                          	pop	rbx
1409d315f: 5f                          	pop	rdi
1409d3160: 5e                          	pop	rsi
1409d3161: 41 5c                       	pop	r12
1409d3163: 41 5d                       	pop	r13
1409d3165: 41 5e                       	pop	r14
1409d3167: 41 5f                       	pop	r15
1409d3169: 5d                          	pop	rbp
1409d316a: c3                          	ret
1409d316b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3170: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3175: 55                          	push	rbp
1409d3176: 41 57                       	push	r15
1409d3178: 41 56                       	push	r14
1409d317a: 41 55                       	push	r13
1409d317c: 41 54                       	push	r12
1409d317e: 56                          	push	rsi
1409d317f: 57                          	push	rdi
1409d3180: 53                          	push	rbx
1409d3181: 48 83 ec 68                 	sub	rsp, 0x68
1409d3185: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d318c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3191: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3196: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d319d: 48 8d 8d 98 06 00 00        	lea	rcx, [rbp + 0x698]
1409d31a4: e8 17 04 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d31a9: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d31b0: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d31b5: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d31ba: 48 83 c4 68                 	add	rsp, 0x68
1409d31be: 5b                          	pop	rbx
1409d31bf: 5f                          	pop	rdi
1409d31c0: 5e                          	pop	rsi
1409d31c1: 41 5c                       	pop	r12
1409d31c3: 41 5d                       	pop	r13
1409d31c5: 41 5e                       	pop	r14
1409d31c7: 41 5f                       	pop	r15
1409d31c9: 5d                          	pop	rbp
1409d31ca: c3                          	ret
1409d31cb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d31d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d31d5: 55                          	push	rbp
1409d31d6: 41 57                       	push	r15
1409d31d8: 41 56                       	push	r14
1409d31da: 41 55                       	push	r13
1409d31dc: 41 54                       	push	r12
1409d31de: 56                          	push	rsi
1409d31df: 57                          	push	rdi
1409d31e0: 53                          	push	rbx
1409d31e1: 48 83 ec 68                 	sub	rsp, 0x68
1409d31e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d31ec: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d31f1: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d31f6: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d31fd: 48 8d 8d 78 0b 00 00        	lea	rcx, [rbp + 0xb78]
1409d3204: e8 b7 03 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d3209: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3210: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3215: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d321a: 48 83 c4 68                 	add	rsp, 0x68
1409d321e: 5b                          	pop	rbx
1409d321f: 5f                          	pop	rdi
1409d3220: 5e                          	pop	rsi
1409d3221: 41 5c                       	pop	r12
1409d3223: 41 5d                       	pop	r13
1409d3225: 41 5e                       	pop	r14
1409d3227: 41 5f                       	pop	r15
1409d3229: 5d                          	pop	rbp
1409d322a: c3                          	ret
1409d322b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3230: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3235: 55                          	push	rbp
1409d3236: 41 57                       	push	r15
1409d3238: 41 56                       	push	r14
1409d323a: 41 55                       	push	r13
1409d323c: 41 54                       	push	r12
1409d323e: 56                          	push	rsi
1409d323f: 57                          	push	rdi
1409d3240: 53                          	push	rbx
1409d3241: 48 83 ec 68                 	sub	rsp, 0x68
1409d3245: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d324c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3251: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3256: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d325d: 48 8d 8d 98 06 00 00        	lea	rcx, [rbp + 0x698]
1409d3264: e8 57 03 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d3269: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3270: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3275: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d327a: 48 83 c4 68                 	add	rsp, 0x68
1409d327e: 5b                          	pop	rbx
1409d327f: 5f                          	pop	rdi
1409d3280: 5e                          	pop	rsi
1409d3281: 41 5c                       	pop	r12
1409d3283: 41 5d                       	pop	r13
1409d3285: 41 5e                       	pop	r14
1409d3287: 41 5f                       	pop	r15
1409d3289: 5d                          	pop	rbp
1409d328a: c3                          	ret
1409d328b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3290: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3295: 55                          	push	rbp
1409d3296: 41 57                       	push	r15
1409d3298: 41 56                       	push	r14
1409d329a: 41 55                       	push	r13
1409d329c: 41 54                       	push	r12
1409d329e: 56                          	push	rsi
1409d329f: 57                          	push	rdi
1409d32a0: 53                          	push	rbx
1409d32a1: 48 83 ec 68                 	sub	rsp, 0x68
1409d32a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d32ac: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d32b1: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d32b6: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d32bd: 48 8d 8d 78 0b 00 00        	lea	rcx, [rbp + 0xb78]
1409d32c4: e8 f7 02 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d32c9: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d32d0: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d32d5: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d32da: 48 83 c4 68                 	add	rsp, 0x68
1409d32de: 5b                          	pop	rbx
1409d32df: 5f                          	pop	rdi
1409d32e0: 5e                          	pop	rsi
1409d32e1: 41 5c                       	pop	r12
1409d32e3: 41 5d                       	pop	r13
1409d32e5: 41 5e                       	pop	r14
1409d32e7: 41 5f                       	pop	r15
1409d32e9: 5d                          	pop	rbp
1409d32ea: c3                          	ret
1409d32eb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d32f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d32f5: 55                          	push	rbp
1409d32f6: 41 57                       	push	r15
1409d32f8: 41 56                       	push	r14
1409d32fa: 41 55                       	push	r13
1409d32fc: 41 54                       	push	r12
1409d32fe: 56                          	push	rsi
1409d32ff: 57                          	push	rdi
1409d3300: 53                          	push	rbx
1409d3301: 48 83 ec 68                 	sub	rsp, 0x68
1409d3305: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d330c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3311: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3316: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d331d: 48 8d 8d 98 06 00 00        	lea	rcx, [rbp + 0x698]
1409d3324: e8 97 02 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d3329: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3330: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3335: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d333a: 48 83 c4 68                 	add	rsp, 0x68
1409d333e: 5b                          	pop	rbx
1409d333f: 5f                          	pop	rdi
1409d3340: 5e                          	pop	rsi
1409d3341: 41 5c                       	pop	r12
1409d3343: 41 5d                       	pop	r13
1409d3345: 41 5e                       	pop	r14
1409d3347: 41 5f                       	pop	r15
1409d3349: 5d                          	pop	rbp
1409d334a: c3                          	ret
1409d334b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3350: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3355: 55                          	push	rbp
1409d3356: 41 57                       	push	r15
1409d3358: 41 56                       	push	r14
1409d335a: 41 55                       	push	r13
1409d335c: 41 54                       	push	r12
1409d335e: 56                          	push	rsi
1409d335f: 57                          	push	rdi
1409d3360: 53                          	push	rbx
1409d3361: 48 83 ec 68                 	sub	rsp, 0x68
1409d3365: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d336c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3371: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3376: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d337d: 48 8d 8d 78 0b 00 00        	lea	rcx, [rbp + 0xb78]
1409d3384: e8 37 02 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d3389: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3390: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3395: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d339a: 48 83 c4 68                 	add	rsp, 0x68
1409d339e: 5b                          	pop	rbx
1409d339f: 5f                          	pop	rdi
1409d33a0: 5e                          	pop	rsi
1409d33a1: 41 5c                       	pop	r12
1409d33a3: 41 5d                       	pop	r13
1409d33a5: 41 5e                       	pop	r14
1409d33a7: 41 5f                       	pop	r15
1409d33a9: 5d                          	pop	rbp
1409d33aa: c3                          	ret
1409d33ab: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d33b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d33b5: 55                          	push	rbp
1409d33b6: 41 57                       	push	r15
1409d33b8: 41 56                       	push	r14
1409d33ba: 41 55                       	push	r13
1409d33bc: 41 54                       	push	r12
1409d33be: 56                          	push	rsi
1409d33bf: 57                          	push	rdi
1409d33c0: 53                          	push	rbx
1409d33c1: 48 83 ec 68                 	sub	rsp, 0x68
1409d33c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d33cc: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d33d1: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d33d6: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d33dd: 48 8d 8d 98 06 00 00        	lea	rcx, [rbp + 0x698]
1409d33e4: e8 d7 01 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d33e9: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d33f0: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d33f5: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d33fa: 48 83 c4 68                 	add	rsp, 0x68
1409d33fe: 5b                          	pop	rbx
1409d33ff: 5f                          	pop	rdi
1409d3400: 5e                          	pop	rsi
1409d3401: 41 5c                       	pop	r12
1409d3403: 41 5d                       	pop	r13
1409d3405: 41 5e                       	pop	r14
1409d3407: 41 5f                       	pop	r15
1409d3409: 5d                          	pop	rbp
1409d340a: c3                          	ret
1409d340b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3410: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3415: 55                          	push	rbp
1409d3416: 41 57                       	push	r15
1409d3418: 41 56                       	push	r14
1409d341a: 41 55                       	push	r13
1409d341c: 41 54                       	push	r12
1409d341e: 56                          	push	rsi
1409d341f: 57                          	push	rdi
1409d3420: 53                          	push	rbx
1409d3421: 48 83 ec 68                 	sub	rsp, 0x68
1409d3425: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d342c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3431: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3436: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d343d: 48 8d 8d 80 0a 00 00        	lea	rcx, [rbp + 0xa80]
1409d3444: e8 77 01 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d3449: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3450: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3455: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d345a: 48 83 c4 68                 	add	rsp, 0x68
1409d345e: 5b                          	pop	rbx
1409d345f: 5f                          	pop	rdi
1409d3460: 5e                          	pop	rsi
1409d3461: 41 5c                       	pop	r12
1409d3463: 41 5d                       	pop	r13
1409d3465: 41 5e                       	pop	r14
1409d3467: 41 5f                       	pop	r15
1409d3469: 5d                          	pop	rbp
1409d346a: c3                          	ret
1409d346b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3470: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3475: 55                          	push	rbp
1409d3476: 41 57                       	push	r15
1409d3478: 41 56                       	push	r14
1409d347a: 41 55                       	push	r13
1409d347c: 41 54                       	push	r12
1409d347e: 56                          	push	rsi
1409d347f: 57                          	push	rdi
1409d3480: 53                          	push	rbx
1409d3481: 48 83 ec 68                 	sub	rsp, 0x68
1409d3485: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d348c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3491: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3496: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d349d: 48 8d 8d 98 06 00 00        	lea	rcx, [rbp + 0x698]
1409d34a4: e8 17 01 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d34a9: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d34b0: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d34b5: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d34ba: 48 83 c4 68                 	add	rsp, 0x68
1409d34be: 5b                          	pop	rbx
1409d34bf: 5f                          	pop	rdi
1409d34c0: 5e                          	pop	rsi
1409d34c1: 41 5c                       	pop	r12
1409d34c3: 41 5d                       	pop	r13
1409d34c5: 41 5e                       	pop	r14
1409d34c7: 41 5f                       	pop	r15
1409d34c9: 5d                          	pop	rbp
1409d34ca: c3                          	ret
1409d34cb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d34d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d34d5: 55                          	push	rbp
1409d34d6: 41 57                       	push	r15
1409d34d8: 41 56                       	push	r14
1409d34da: 41 55                       	push	r13
1409d34dc: 41 54                       	push	r12
1409d34de: 56                          	push	rsi
1409d34df: 57                          	push	rdi
1409d34e0: 53                          	push	rbx
1409d34e1: 48 83 ec 68                 	sub	rsp, 0x68
1409d34e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d34ec: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d34f1: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d34f6: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d34fd: 48 8d 8d 78 0b 00 00        	lea	rcx, [rbp + 0xb78]
1409d3504: e8 b7 00 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d3509: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3510: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3515: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d351a: 48 83 c4 68                 	add	rsp, 0x68
1409d351e: 5b                          	pop	rbx
1409d351f: 5f                          	pop	rdi
1409d3520: 5e                          	pop	rsi
1409d3521: 41 5c                       	pop	r12
1409d3523: 41 5d                       	pop	r13
1409d3525: 41 5e                       	pop	r14
1409d3527: 41 5f                       	pop	r15
1409d3529: 5d                          	pop	rbp
1409d352a: c3                          	ret
1409d352b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3530: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3535: 55                          	push	rbp
1409d3536: 41 57                       	push	r15
1409d3538: 41 56                       	push	r14
1409d353a: 41 55                       	push	r13
1409d353c: 41 54                       	push	r12
1409d353e: 56                          	push	rsi
1409d353f: 57                          	push	rdi
1409d3540: 53                          	push	rbx
1409d3541: 48 83 ec 68                 	sub	rsp, 0x68
1409d3545: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d354c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3551: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3556: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d355d: 48 8d 8d a8 04 00 00        	lea	rcx, [rbp + 0x4a8]
1409d3564: e8 57 00 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409d3569: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3570: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3575: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d357a: 48 83 c4 68                 	add	rsp, 0x68
1409d357e: 5b                          	pop	rbx
1409d357f: 5f                          	pop	rdi
1409d3580: 5e                          	pop	rsi
1409d3581: 41 5c                       	pop	r12
1409d3583: 41 5d                       	pop	r13
1409d3585: 41 5e                       	pop	r14
1409d3587: 41 5f                       	pop	r15
1409d3589: 5d                          	pop	rbp
1409d358a: c3                          	ret
1409d358b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3590: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3595: 55                          	push	rbp
1409d3596: 41 57                       	push	r15
1409d3598: 41 56                       	push	r14
1409d359a: 41 55                       	push	r13
1409d359c: 41 54                       	push	r12
1409d359e: 56                          	push	rsi
1409d359f: 57                          	push	rdi
1409d35a0: 53                          	push	rbx
1409d35a1: 48 83 ec 68                 	sub	rsp, 0x68
1409d35a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d35ac: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d35b1: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d35b6: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d35bd: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
1409d35c4: e8 f7 ff eb ff              	call	0x1408935c0 <.text+0x8925c0>
1409d35c9: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d35d0: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d35d5: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d35da: 48 83 c4 68                 	add	rsp, 0x68
1409d35de: 5b                          	pop	rbx
1409d35df: 5f                          	pop	rdi
1409d35e0: 5e                          	pop	rsi
1409d35e1: 41 5c                       	pop	r12
1409d35e3: 41 5d                       	pop	r13
1409d35e5: 41 5e                       	pop	r14
1409d35e7: 41 5f                       	pop	r15
1409d35e9: 5d                          	pop	rbp
1409d35ea: c3                          	ret
1409d35eb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d35f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d35f5: 55                          	push	rbp
1409d35f6: 41 57                       	push	r15
1409d35f8: 41 56                       	push	r14
1409d35fa: 41 55                       	push	r13
1409d35fc: 41 54                       	push	r12
1409d35fe: 56                          	push	rsi
1409d35ff: 57                          	push	rdi
1409d3600: 53                          	push	rbx
1409d3601: 48 83 ec 68                 	sub	rsp, 0x68
1409d3605: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d360c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3611: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3616: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d361d: 48 8d 8d 98 06 00 00        	lea	rcx, [rbp + 0x698]
1409d3624: e8 97 ff eb ff              	call	0x1408935c0 <.text+0x8925c0>
1409d3629: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3630: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3635: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d363a: 48 83 c4 68                 	add	rsp, 0x68
1409d363e: 5b                          	pop	rbx
1409d363f: 5f                          	pop	rdi
1409d3640: 5e                          	pop	rsi
1409d3641: 41 5c                       	pop	r12
1409d3643: 41 5d                       	pop	r13
1409d3645: 41 5e                       	pop	r14
1409d3647: 41 5f                       	pop	r15
1409d3649: 5d                          	pop	rbp
1409d364a: c3                          	ret
1409d364b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3650: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3655: 55                          	push	rbp
1409d3656: 41 57                       	push	r15
1409d3658: 41 56                       	push	r14
1409d365a: 41 55                       	push	r13
1409d365c: 41 54                       	push	r12
1409d365e: 56                          	push	rsi
1409d365f: 57                          	push	rdi
1409d3660: 53                          	push	rbx
1409d3661: 48 83 ec 68                 	sub	rsp, 0x68
1409d3665: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d366c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3671: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3676: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d367d: 48 8d 8d 78 0b 00 00        	lea	rcx, [rbp + 0xb78]
1409d3684: e8 37 ff eb ff              	call	0x1408935c0 <.text+0x8925c0>
1409d3689: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3690: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3695: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d369a: 48 83 c4 68                 	add	rsp, 0x68
1409d369e: 5b                          	pop	rbx
1409d369f: 5f                          	pop	rdi
1409d36a0: 5e                          	pop	rsi
1409d36a1: 41 5c                       	pop	r12
1409d36a3: 41 5d                       	pop	r13
1409d36a5: 41 5e                       	pop	r14
1409d36a7: 41 5f                       	pop	r15
1409d36a9: 5d                          	pop	rbp
1409d36aa: c3                          	ret
1409d36ab: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d36b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d36b5: 55                          	push	rbp
1409d36b6: 41 57                       	push	r15
1409d36b8: 41 56                       	push	r14
1409d36ba: 41 55                       	push	r13
1409d36bc: 41 54                       	push	r12
1409d36be: 56                          	push	rsi
1409d36bf: 57                          	push	rdi
1409d36c0: 53                          	push	rbx
1409d36c1: 48 83 ec 68                 	sub	rsp, 0x68
1409d36c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d36cc: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d36d1: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d36d6: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d36dd: 48 8d 8d 98 06 00 00        	lea	rcx, [rbp + 0x698]
1409d36e4: e8 d7 fe eb ff              	call	0x1408935c0 <.text+0x8925c0>
1409d36e9: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d36f0: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d36f5: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d36fa: 48 83 c4 68                 	add	rsp, 0x68
1409d36fe: 5b                          	pop	rbx
1409d36ff: 5f                          	pop	rdi
1409d3700: 5e                          	pop	rsi
1409d3701: 41 5c                       	pop	r12
1409d3703: 41 5d                       	pop	r13
1409d3705: 41 5e                       	pop	r14
1409d3707: 41 5f                       	pop	r15
1409d3709: 5d                          	pop	rbp
1409d370a: c3                          	ret
1409d370b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3710: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3715: 55                          	push	rbp
1409d3716: 41 57                       	push	r15
1409d3718: 41 56                       	push	r14
1409d371a: 41 55                       	push	r13
1409d371c: 41 54                       	push	r12
1409d371e: 56                          	push	rsi
1409d371f: 57                          	push	rdi
1409d3720: 53                          	push	rbx
1409d3721: 48 83 ec 68                 	sub	rsp, 0x68
1409d3725: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d372c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3731: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3736: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d373d: 48 8d 8d 78 0b 00 00        	lea	rcx, [rbp + 0xb78]
1409d3744: e8 77 fe eb ff              	call	0x1408935c0 <.text+0x8925c0>
1409d3749: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3750: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3755: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d375a: 48 83 c4 68                 	add	rsp, 0x68
1409d375e: 5b                          	pop	rbx
1409d375f: 5f                          	pop	rdi
1409d3760: 5e                          	pop	rsi
1409d3761: 41 5c                       	pop	r12
1409d3763: 41 5d                       	pop	r13
1409d3765: 41 5e                       	pop	r14
1409d3767: 41 5f                       	pop	r15
1409d3769: 5d                          	pop	rbp
1409d376a: c3                          	ret
1409d376b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3770: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3775: 55                          	push	rbp
1409d3776: 41 57                       	push	r15
1409d3778: 41 56                       	push	r14
1409d377a: 41 55                       	push	r13
1409d377c: 41 54                       	push	r12
1409d377e: 56                          	push	rsi
1409d377f: 57                          	push	rdi
1409d3780: 53                          	push	rbx
1409d3781: 48 83 ec 68                 	sub	rsp, 0x68
1409d3785: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d378c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3791: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3796: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d379d: 48 8d 8d 98 06 00 00        	lea	rcx, [rbp + 0x698]
1409d37a4: e8 17 fe eb ff              	call	0x1408935c0 <.text+0x8925c0>
1409d37a9: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d37b0: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d37b5: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d37ba: 48 83 c4 68                 	add	rsp, 0x68
1409d37be: 5b                          	pop	rbx
1409d37bf: 5f                          	pop	rdi
1409d37c0: 5e                          	pop	rsi
1409d37c1: 41 5c                       	pop	r12
1409d37c3: 41 5d                       	pop	r13
1409d37c5: 41 5e                       	pop	r14
1409d37c7: 41 5f                       	pop	r15
1409d37c9: 5d                          	pop	rbp
1409d37ca: c3                          	ret
1409d37cb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d37d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d37d5: 55                          	push	rbp
1409d37d6: 41 57                       	push	r15
1409d37d8: 41 56                       	push	r14
1409d37da: 41 55                       	push	r13
1409d37dc: 41 54                       	push	r12
1409d37de: 56                          	push	rsi
1409d37df: 57                          	push	rdi
1409d37e0: 53                          	push	rbx
1409d37e1: 48 83 ec 68                 	sub	rsp, 0x68
1409d37e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d37ec: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d37f1: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d37f6: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d37fd: 48 8d 8d 78 0b 00 00        	lea	rcx, [rbp + 0xb78]
1409d3804: e8 b7 fd eb ff              	call	0x1408935c0 <.text+0x8925c0>
1409d3809: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3810: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3815: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d381a: 48 83 c4 68                 	add	rsp, 0x68
1409d381e: 5b                          	pop	rbx
1409d381f: 5f                          	pop	rdi
1409d3820: 5e                          	pop	rsi
1409d3821: 41 5c                       	pop	r12
1409d3823: 41 5d                       	pop	r13
1409d3825: 41 5e                       	pop	r14
1409d3827: 41 5f                       	pop	r15
1409d3829: 5d                          	pop	rbp
1409d382a: c3                          	ret
1409d382b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3830: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3835: 55                          	push	rbp
1409d3836: 41 57                       	push	r15
1409d3838: 41 56                       	push	r14
1409d383a: 41 55                       	push	r13
1409d383c: 41 54                       	push	r12
1409d383e: 56                          	push	rsi
1409d383f: 57                          	push	rdi
1409d3840: 53                          	push	rbx
1409d3841: 48 83 ec 68                 	sub	rsp, 0x68
1409d3845: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d384c: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d3851: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d3856: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d385d: 48 8d 8d 98 06 00 00        	lea	rcx, [rbp + 0x698]
1409d3864: e8 57 fd eb ff              	call	0x1408935c0 <.text+0x8925c0>
1409d3869: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d3870: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d3875: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d387a: 48 83 c4 68                 	add	rsp, 0x68
1409d387e: 5b                          	pop	rbx
1409d387f: 5f                          	pop	rdi
1409d3880: 5e                          	pop	rsi
1409d3881: 41 5c                       	pop	r12
1409d3883: 41 5d                       	pop	r13
1409d3885: 41 5e                       	pop	r14
1409d3887: 41 5f                       	pop	r15
1409d3889: 5d                          	pop	rbp
1409d388a: c3                          	ret
1409d388b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409d3890: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409d3895: 55                          	push	rbp
1409d3896: 41 57                       	push	r15
1409d3898: 41 56                       	push	r14
1409d389a: 41 55                       	push	r13
1409d389c: 41 54                       	push	r12
1409d389e: 56                          	push	rsi
1409d389f: 57                          	push	rdi
1409d38a0: 53                          	push	rbx
1409d38a1: 48 83 ec 68                 	sub	rsp, 0x68
1409d38a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409d38ac: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
1409d38b1: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
1409d38b6: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d38bd: 48 8d 8d a0 05 00 00        	lea	rcx, [rbp + 0x5a0]
1409d38c4: e8 f7 fc eb ff              	call	0x1408935c0 <.text+0x8925c0>
1409d38c9: c6 85 57 0d 00 00 01        	mov	byte ptr [rbp + 0xd57], 0x1
1409d38d0: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
1409d38d5: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
1409d38da: 48 83 c4 68                 	add	rsp, 0x68
1409d38de: 5b                          	pop	rbx
1409d38df: 5f                          	pop	rdi
1409d38e0: 5e                          	pop	rsi
1409d38e1: 41 5c                       	pop	r12
1409d38e3: 41 5d                       	pop	r13
1409d38e5: 41 5e                       	pop	r14
1409d38e7: 41 5f                       	pop	r15
1409d38e9: 5d                          	pop	rbp
1409d38ea: c3                          	ret

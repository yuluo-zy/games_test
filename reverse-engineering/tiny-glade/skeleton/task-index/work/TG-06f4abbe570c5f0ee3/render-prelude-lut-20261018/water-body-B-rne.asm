
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a713b0: 55                          	push	rbp
140a713b1: 56                          	push	rsi
140a713b2: 57                          	push	rdi
140a713b3: 48 81 ec 40 05 00 00        	sub	rsp, 0x540
140a713ba: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a713c2: 48 c7 85 b8 04 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x4b8], -0x2
140a713cd: 48 8d 7d b0                 	lea	rdi, [rbp - 0x50]
140a713d1: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a713d7: 48 89 f9                    	mov	rcx, rdi
140a713da: e8 21 28 eb 01              	call	0x142923c00 <.text+0x2922c00>
140a713df: c6 85 d0 02 00 00 18        	mov	byte ptr [rbp + 0x2d0], 0x18
140a713e6: c7 85 40 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x140], 0x2
140a713f0: 48 8d 85 40 01 00 00        	lea	rax, [rbp + 0x140]
140a713f7: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140a713fc: 48 8d b5 d0 02 00 00        	lea	rsi, [rbp + 0x2d0]
140a71403: 49 b8 b2 92 98 e6 3a 9a 21 88       	movabs	r8, -0x77de65c519676d4e
140a7140d: 48 89 f1                    	mov	rcx, rsi
140a71410: 48 89 fa                    	mov	rdx, rdi
140a71413: 41 b1 32                    	mov	r9b, 0x32
140a71416: e8 45 3e e9 ff              	call	0x140905260 <.text+0x904260>
140a7141b: 48 8d bd 40 01 00 00        	lea	rdi, [rbp + 0x140]
140a71422: 48 8d 55 b0                 	lea	rdx, [rbp - 0x50]
140a71426: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a7142c: 48 89 f9                    	mov	rcx, rdi
140a7142f: e8 cc 27 eb 01              	call	0x142923c00 <.text+0x2922c00>
140a71434: 48 b8 af 0f 9f 9d 7a 5f c9 6e       	movabs	rax, 0x6ec95f7a9d9f0faf
140a7143e: 48 89 85 60 04 00 00        	mov	qword ptr [rbp + 0x460], rax
140a71445: c7 85 d0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2d0], 0x2
140a7144f: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
140a71454: 48 8d 8d 60 04 00 00        	lea	rcx, [rbp + 0x460]
140a7145b: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
140a71465: 48 89 fa                    	mov	rdx, rdi
140a71468: 41 b1 19                    	mov	r9b, 0x19
140a7146b: e8 60 42 e9 ff              	call	0x1409056d0 <.text+0x9046d0>
140a71470: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
140a71477: 48 8d 95 40 01 00 00        	lea	rdx, [rbp + 0x140]
140a7147e: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a71484: e8 77 27 eb 01              	call	0x142923c00 <.text+0x2922c00>
140a71489: 0f 10 85 70 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x370]
140a71490: 0f 29 85 a0 04 00 00        	movaps	xmmword ptr [rbp + 0x4a0], xmm0
140a71497: 0f 10 85 60 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x360]
140a7149e: 0f 29 85 90 04 00 00        	movaps	xmmword ptr [rbp + 0x490], xmm0
140a714a5: 0f 10 85 50 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x350]
140a714ac: 0f 29 85 80 04 00 00        	movaps	xmmword ptr [rbp + 0x480], xmm0
140a714b3: 0f 10 85 40 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x340]
140a714ba: 0f 29 85 70 04 00 00        	movaps	xmmword ptr [rbp + 0x470], xmm0
140a714c1: 0f 10 85 30 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x330]
140a714c8: 0f 29 85 60 04 00 00        	movaps	xmmword ptr [rbp + 0x460], xmm0
140a714cf: 48 8b 85 08 04 00 00        	mov	rax, qword ptr [rbp + 0x408]
140a714d6: 48 8b 8d 50 04 00 00        	mov	rcx, qword ptr [rbp + 0x450]
140a714dd: 48 8b 95 e0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e0]
140a714e4: 48 89 95 50 01 00 00        	mov	qword ptr [rbp + 0x150], rdx
140a714eb: 0f 10 85 d0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2d0]
140a714f2: 0f 29 85 40 01 00 00        	movaps	xmmword ptr [rbp + 0x140], xmm0
140a714f9: 0f 10 85 e8 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2e8]
140a71500: 0f 29 85 70 01 00 00        	movaps	xmmword ptr [rbp + 0x170], xmm0
140a71507: 48 8b 95 f8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2f8]
140a7150e: 48 89 95 80 01 00 00        	mov	qword ptr [rbp + 0x180], rdx
140a71515: 0f 10 85 00 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x300]
140a7151c: 0f 11 85 58 01 00 00        	movups	xmmword ptr [rbp + 0x158], xmm0
140a71523: 48 8b 95 10 03 00 00        	mov	rdx, qword ptr [rbp + 0x310]
140a7152a: 48 89 95 68 01 00 00        	mov	qword ptr [rbp + 0x168], rdx
140a71531: 0f 10 85 80 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x380]
140a71538: 0f 10 8d 90 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x390]
140a7153f: 0f 10 95 a0 03 00 00        	movups	xmm2, xmmword ptr [rbp + 0x3a0]
140a71546: 0f 10 9d b0 03 00 00        	movups	xmm3, xmmword ptr [rbp + 0x3b0]
140a7154d: 0f 11 85 b8 01 00 00        	movups	xmmword ptr [rbp + 0x1b8], xmm0
140a71554: 0f 11 8d c8 01 00 00        	movups	xmmword ptr [rbp + 0x1c8], xmm1
140a7155b: 0f 11 95 d8 01 00 00        	movups	xmmword ptr [rbp + 0x1d8], xmm2
140a71562: 0f 11 9d e8 01 00 00        	movups	xmmword ptr [rbp + 0x1e8], xmm3
140a71569: 0f 10 85 c0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3c0]
140a71570: 0f 11 85 f8 01 00 00        	movups	xmmword ptr [rbp + 0x1f8], xmm0
140a71577: 0f 10 85 d0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3d0]
140a7157e: 0f 11 85 08 02 00 00        	movups	xmmword ptr [rbp + 0x208], xmm0
140a71585: 48 8b 95 28 03 00 00        	mov	rdx, qword ptr [rbp + 0x328]
140a7158c: 48 89 95 98 01 00 00        	mov	qword ptr [rbp + 0x198], rdx
140a71593: 0f 10 85 18 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x318]
140a7159a: 0f 11 85 88 01 00 00        	movups	xmmword ptr [rbp + 0x188], xmm0
140a715a1: 48 89 8d 20 02 00 00        	mov	qword ptr [rbp + 0x220], rcx
140a715a8: 48 89 85 18 02 00 00        	mov	qword ptr [rbp + 0x218], rax
140a715af: 0f 28 05 ca ee 05 02        	movaps	xmm0, xmmword ptr [rip + 0x205eeca] # 0x142ad0480
140a715b6: 0f 29 85 a0 01 00 00        	movaps	xmmword ptr [rbp + 0x1a0], xmm0
140a715bd: 48 8d 8d 60 04 00 00        	lea	rcx, [rbp + 0x460]
140a715c4: 48 8d 95 40 01 00 00        	lea	rdx, [rbp + 0x140]
140a715cb: e8 00 bf f6 ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a715d0: 80 bd 06 04 00 00 08        	cmp	byte ptr [rbp + 0x406], 0x8
140a715d7: 74 0c                       	je	0x140a715e5 <.text+0xa705e5>
140a715d9: 48 8d 8d f0 03 00 00        	lea	rcx, [rbp + 0x3f0]
140a715e0: e8 cb 0e 17 00              	call	0x140be24b0 <.text+0xbe14b0>
140a715e5: 90                          	nop
140a715e6: 48 81 c4 40 05 00 00        	add	rsp, 0x540
140a715ed: 5f                          	pop	rdi
140a715ee: 5e                          	pop	rsi
140a715ef: 5d                          	pop	rbp
140a715f0: c3                          	ret
140a715f1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140a71600: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a71605: 55                          	push	rbp
140a71606: 56                          	push	rsi
140a71607: 57                          	push	rdi
140a71608: 48 83 ec 30                 	sub	rsp, 0x30
140a7160c: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a71613: 80 bd 06 04 00 00 08        	cmp	byte ptr [rbp + 0x406], 0x8
140a7161a: 74 0c                       	je	0x140a71628 <.text+0xa70628>
140a7161c: 48 8d 8d f0 03 00 00        	lea	rcx, [rbp + 0x3f0]
140a71623: e8 88 0e 17 00              	call	0x140be24b0 <.text+0xbe14b0>
140a71628: 90                          	nop
140a71629: 48 83 c4 30                 	add	rsp, 0x30
140a7162d: 5f                          	pop	rdi
140a7162e: 5e                          	pop	rsi
140a7162f: 5d                          	pop	rbp
140a71630: c3                          	ret
140a71631: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140a71640: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a71645: 55                          	push	rbp
140a71646: 56                          	push	rsi
140a71647: 57                          	push	rdi
140a71648: 48 83 ec 30                 	sub	rsp, 0x30
140a7164c: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a71653: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
140a71657: e8 54 59 e2 ff              	call	0x140896fb0 <.text+0x895fb0>
140a7165c: 90                          	nop
140a7165d: 48 83 c4 30                 	add	rsp, 0x30
140a71661: 5f                          	pop	rdi
140a71662: 5e                          	pop	rsi
140a71663: 5d                          	pop	rbp
140a71664: c3                          	ret
140a71665: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
140a71670: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a71675: 55                          	push	rbp
140a71676: 56                          	push	rsi
140a71677: 57                          	push	rdi
140a71678: 48 83 ec 30                 	sub	rsp, 0x30
140a7167c: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a71683: 48 8d 8d 40 01 00 00        	lea	rcx, [rbp + 0x140]
140a7168a: e8 21 59 e2 ff              	call	0x140896fb0 <.text+0x895fb0>
140a7168f: 90                          	nop
140a71690: 48 83 c4 30                 	add	rsp, 0x30
140a71694: 5f                          	pop	rdi
140a71695: 5e                          	pop	rsi
140a71696: 5d                          	pop	rbp
140a71697: c3                          	ret

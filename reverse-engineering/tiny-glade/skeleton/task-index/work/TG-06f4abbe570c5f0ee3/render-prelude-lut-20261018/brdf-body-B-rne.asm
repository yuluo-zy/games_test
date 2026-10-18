
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a71730: 55                          	push	rbp
140a71731: 56                          	push	rsi
140a71732: 48 81 ec 98 04 00 00        	sub	rsp, 0x498
140a71739: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a71741: 48 c7 85 10 04 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x410], -0x2
140a7174c: 48 8d b5 40 01 00 00        	lea	rsi, [rbp + 0x140]
140a71753: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a71759: 48 89 f1                    	mov	rcx, rsi
140a7175c: e8 9f 24 eb 01              	call	0x142923c00 <.text+0x2922c00>
140a71761: c6 85 20 03 00 00 00        	mov	byte ptr [rbp + 0x320], 0x0
140a71768: c7 45 b0 02 00 00 00        	mov	dword ptr [rbp - 0x50], 0x2
140a7176f: 48 8d 45 b0                 	lea	rax, [rbp - 0x50]
140a71773: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140a71778: 48 8d 8d 20 03 00 00        	lea	rcx, [rbp + 0x320]
140a7177f: 49 b8 b2 92 98 e6 3a 9a 21 88       	movabs	r8, -0x77de65c519676d4e
140a71789: 48 89 f2                    	mov	rdx, rsi
140a7178c: 41 b1 32                    	mov	r9b, 0x32
140a7178f: e8 cc 3a e9 ff              	call	0x140905260 <.text+0x904260>
140a71794: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
140a71798: 48 8d 95 40 01 00 00        	lea	rdx, [rbp + 0x140]
140a7179f: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a717a5: e8 56 24 eb 01              	call	0x142923c00 <.text+0x2922c00>
140a717aa: 0f 10 85 e0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1e0]
140a717b1: 0f 29 85 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm0
140a717b8: 0f 10 85 d0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d0]
140a717bf: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a717c6: 0f 10 85 c0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1c0]
140a717cd: 0f 29 85 f0 02 00 00        	movaps	xmmword ptr [rbp + 0x2f0], xmm0
140a717d4: 0f 10 85 b0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1b0]
140a717db: 0f 29 85 e0 02 00 00        	movaps	xmmword ptr [rbp + 0x2e0], xmm0
140a717e2: 0f 10 85 a0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1a0]
140a717e9: 0f 29 85 d0 02 00 00        	movaps	xmmword ptr [rbp + 0x2d0], xmm0
140a717f0: 48 8b 85 e8 00 00 00        	mov	rax, qword ptr [rbp + 0xe8]
140a717f7: 48 8b 8d 30 01 00 00        	mov	rcx, qword ptr [rbp + 0x130]
140a717fe: 48 8b 95 50 01 00 00        	mov	rdx, qword ptr [rbp + 0x150]
140a71805: 48 89 95 30 03 00 00        	mov	qword ptr [rbp + 0x330], rdx
140a7180c: 0f 10 85 40 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x140]
140a71813: 0f 29 85 20 03 00 00        	movaps	xmmword ptr [rbp + 0x320], xmm0
140a7181a: 0f 10 85 58 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x158]
140a71821: 0f 29 85 50 03 00 00        	movaps	xmmword ptr [rbp + 0x350], xmm0
140a71828: 48 8b 95 68 01 00 00        	mov	rdx, qword ptr [rbp + 0x168]
140a7182f: 48 89 95 60 03 00 00        	mov	qword ptr [rbp + 0x360], rdx
140a71836: 0f 10 85 70 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x170]
140a7183d: 0f 11 85 38 03 00 00        	movups	xmmword ptr [rbp + 0x338], xmm0
140a71844: 48 8b 95 80 01 00 00        	mov	rdx, qword ptr [rbp + 0x180]
140a7184b: 48 89 95 48 03 00 00        	mov	qword ptr [rbp + 0x348], rdx
140a71852: 0f 10 85 f0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1f0]
140a71859: 0f 10 8d 00 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x200]
140a71860: 0f 10 95 10 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x210]
140a71867: 0f 10 9d 20 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x220]
140a7186e: 0f 11 85 98 03 00 00        	movups	xmmword ptr [rbp + 0x398], xmm0
140a71875: 0f 11 8d a8 03 00 00        	movups	xmmword ptr [rbp + 0x3a8], xmm1
140a7187c: 0f 11 95 b8 03 00 00        	movups	xmmword ptr [rbp + 0x3b8], xmm2
140a71883: 0f 11 9d c8 03 00 00        	movups	xmmword ptr [rbp + 0x3c8], xmm3
140a7188a: 0f 10 85 30 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x230]
140a71891: 0f 11 85 d8 03 00 00        	movups	xmmword ptr [rbp + 0x3d8], xmm0
140a71898: 0f 10 85 40 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x240]
140a7189f: 0f 11 85 e8 03 00 00        	movups	xmmword ptr [rbp + 0x3e8], xmm0
140a718a6: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
140a718ad: 48 89 95 78 03 00 00        	mov	qword ptr [rbp + 0x378], rdx
140a718b4: 0f 10 85 88 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x188]
140a718bb: 0f 11 85 68 03 00 00        	movups	xmmword ptr [rbp + 0x368], xmm0
140a718c2: 48 89 8d 00 04 00 00        	mov	qword ptr [rbp + 0x400], rcx
140a718c9: 48 89 85 f8 03 00 00        	mov	qword ptr [rbp + 0x3f8], rax
140a718d0: 0f 28 05 09 3d 06 02        	movaps	xmm0, xmmword ptr [rip + 0x2063d09] # 0x142ad55e0
140a718d7: 0f 29 85 80 03 00 00        	movaps	xmmword ptr [rbp + 0x380], xmm0
140a718de: 48 8d 8d d0 02 00 00        	lea	rcx, [rbp + 0x2d0]
140a718e5: 48 8d 95 20 03 00 00        	lea	rdx, [rbp + 0x320]
140a718ec: e8 df bb f6 ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a718f1: 80 bd e6 00 00 00 08        	cmp	byte ptr [rbp + 0xe6], 0x8
140a718f8: 74 0c                       	je	0x140a71906 <.text+0xa70906>
140a718fa: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
140a71901: e8 aa 0b 17 00              	call	0x140be24b0 <.text+0xbe14b0>
140a71906: 90                          	nop
140a71907: 48 81 c4 98 04 00 00        	add	rsp, 0x498
140a7190e: 5e                          	pop	rsi
140a7190f: 5d                          	pop	rbp
140a71910: c3                          	ret
140a71911: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140a71920: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a71925: 55                          	push	rbp
140a71926: 56                          	push	rsi
140a71927: 48 83 ec 28                 	sub	rsp, 0x28
140a7192b: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a71932: 80 bd e6 00 00 00 08        	cmp	byte ptr [rbp + 0xe6], 0x8
140a71939: 74 0c                       	je	0x140a71947 <.text+0xa70947>
140a7193b: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
140a71942: e8 69 0b 17 00              	call	0x140be24b0 <.text+0xbe14b0>
140a71947: 90                          	nop
140a71948: 48 83 c4 28                 	add	rsp, 0x28
140a7194c: 5e                          	pop	rsi
140a7194d: 5d                          	pop	rbp
140a7194e: c3                          	ret
140a7194f: 90                          	nop
140a71950: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a71955: 55                          	push	rbp
140a71956: 56                          	push	rsi
140a71957: 48 83 ec 28                 	sub	rsp, 0x28
140a7195b: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a71962: 48 8d 8d 40 01 00 00        	lea	rcx, [rbp + 0x140]
140a71969: e8 42 56 e2 ff              	call	0x140896fb0 <.text+0x895fb0>
140a7196e: 90                          	nop
140a7196f: 48 83 c4 28                 	add	rsp, 0x28
140a71973: 5e                          	pop	rsi
140a71974: 5d                          	pop	rbp
140a71975: c3                          	ret


ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a22140: 55                          	push	rbp
140a22141: 41 57                       	push	r15
140a22143: 41 56                       	push	r14
140a22145: 56                          	push	rsi
140a22146: 57                          	push	rdi
140a22147: 53                          	push	rbx
140a22148: 48 81 ec f8 03 00 00        	sub	rsp, 0x3f8
140a2214f: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a22157: 0f 29 b5 60 03 00 00        	movaps	xmmword ptr [rbp + 0x360], xmm6
140a2215e: 48 c7 85 58 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x358], -0x2
140a22169: 48 89 d6                    	mov	rsi, rdx
140a2216c: 48 89 cf                    	mov	rdi, rcx
140a2216f: 48 8d 4a 38                 	lea	rcx, [rdx + 0x38]
140a22173: 48 ba 9a 25 aa 88 62 fa 9f 97       	movabs	rdx, -0x6860059d7755da66
140a2217d: 49 b8 9d 44 ed 46 5b 1d 79 09       	movabs	r8, 0x9791d5b46ed449d
140a22187: e8 64 b5 6b ff              	call	0x1400dd6f0 <.text+0xdc6f0>
140a2218c: 48 85 c0                    	test	rax, rax
140a2218f: 0f 84 af 06 00 00           	je	0x140a22844 <.text+0xa21844>
140a22195: 48 39 96 18 02 00 00        	cmp	qword ptr [rsi + 0x218], rdx
140a2219c: 0f 86 a2 06 00 00           	jbe	0x140a22844 <.text+0xa21844>
140a221a2: 48 8b 86 10 02 00 00        	mov	rax, qword ptr [rsi + 0x210]
140a221a9: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
140a221ad: 48 85 c9                    	test	rcx, rcx
140a221b0: 0f 84 8e 06 00 00           	je	0x140a22844 <.text+0xa21844>
140a221b6: 48 8b 86 e0 01 00 00        	mov	rax, qword ptr [rsi + 0x1e0]
140a221bd: 48 f7 d1                    	not	rcx
140a221c0: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
140a221c4: 48 c1 e1 05                 	shl	rcx, 0x5
140a221c8: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
140a221ce: 0f 84 70 06 00 00           	je	0x140a22844 <.text+0xa21844>
140a221d4: 48 01 c8                    	add	rax, rcx
140a221d7: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
140a221db: 8b 96 b0 05 00 00           	mov	edx, dword ptr [rsi + 0x5b0]
140a221e1: 80 b9 9b 02 00 00 00        	cmp	byte ptr [rcx + 0x29b], 0x0
140a221e8: 0f 84 3f 06 00 00           	je	0x140a2282d <.text+0xa2182d>
140a221ee: 89 50 5c                    	mov	dword ptr [rax + 0x5c], edx
140a221f1: c6 81 9b 02 00 00 00        	mov	byte ptr [rcx + 0x29b], 0x0
140a221f8: 48 81 c1 98 00 00 00        	add	rcx, 0x98
140a221ff: 41 b8 00 02 00 00           	mov	r8d, 0x200
140a22205: 31 d2                       	xor	edx, edx
140a22207: e8 14 1a f0 01              	call	0x142923c20 <.text+0x2922c20>
140a2220c: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a22213: 48 ba 00 02 00 00 00 02 00 00       	movabs	rdx, 0x20000000200
140a2221d: 41 b8 61 00 00 00           	mov	r8d, 0x61
140a22223: e8 78 30 1b 00              	call	0x140bd52a0 <.text+0xbd42a0>
140a22228: 0f 10 85 00 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x300]
140a2222f: 0f 10 8d 10 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x310]
140a22236: 0f 29 8d 80 01 00 00        	movaps	xmmword ptr [rbp + 0x180], xmm1
140a2223d: 0f 29 85 70 01 00 00        	movaps	xmmword ptr [rbp + 0x170], xmm0
140a22244: 48 8d 4d f0                 	lea	rcx, [rbp - 0x10]
140a22248: 48 8d 95 70 01 00 00        	lea	rdx, [rbp + 0x170]
140a2224f: e8 8c 4a 1b 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
140a22254: 48 8d 05 cd 76 0a 02        	lea	rax, [rip + 0x20a76cd]  # 0x142ac9928
140a2225b: 48 89 45 e0                 	mov	qword ptr [rbp - 0x20], rax
140a2225f: 48 c7 45 e8 10 00 00 00     	mov	qword ptr [rbp - 0x18], 0x10
140a22267: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a2226b: 48 89 f9                    	mov	rcx, rdi
140a2226e: e8 5d 72 1c 00              	call	0x140be94d0 <.text+0xbe84d0>
140a22273: 0f 10 85 00 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x300]
140a2227a: 0f 10 8d 10 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x310]
140a22281: 0f 11 45 c0                 	movups	xmmword ptr [rbp - 0x40], xmm0
140a22285: 0f 11 4d d0                 	movups	xmmword ptr [rbp - 0x30], xmm1
140a22289: 89 45 bc                    	mov	dword ptr [rbp - 0x44], eax
140a2228c: 48 89 7c 24 30              	mov	qword ptr [rsp + 0x30], rdi
140a22291: 48 89 74 24 28              	mov	qword ptr [rsp + 0x28], rsi
140a22296: 48 c7 44 24 20 18 00 00 00  	mov	qword ptr [rsp + 0x20], 0x18
140a2229f: 48 8d 15 92 76 0a 02        	lea	rdx, [rip + 0x20a7692]  # 0x142ac9938
140a222a6: 4c 8d 0d 9b 76 0a 02        	lea	r9, [rip + 0x20a769b]   # 0x142ac9948
140a222ad: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a222b1: 41 b8 10 00 00 00           	mov	r8d, 0x10
140a222b7: e8 e4 0f be 00              	call	0x1416032a0 <.text+0x16022a0>
140a222bc: 80 bd 16 01 00 00 08        	cmp	byte ptr [rbp + 0x116], 0x8
140a222c3: 74 25                       	je	0x140a222ea <.text+0xa212ea>
140a222c5: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a222cc: ba e8 00 00 00              	mov	edx, 0xe8
140a222d1: 48 03 95 18 01 00 00        	add	rdx, qword ptr [rbp + 0x118]
140a222d8: 49 b8 22 bc 94 20 37 f8 1f d9       	movabs	r8, -0x26e007c8df6b43de
140a222e2: 41 b1 01                    	mov	r9b, 0x1
140a222e5: e8 66 95 f4 ff              	call	0x14096b850 <.text+0x96a850>
140a222ea: 48 8d 9d 70 01 00 00        	lea	rbx, [rbp + 0x170]
140a222f1: 4c 8d 75 e0                 	lea	r14, [rbp - 0x20]
140a222f5: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a222fb: 48 89 d9                    	mov	rcx, rbx
140a222fe: 4c 89 f2                    	mov	rdx, r14
140a22301: e8 fa 18 f0 01              	call	0x142923c00 <.text+0x2922c00>
140a22306: c6 85 00 03 00 00 14        	mov	byte ptr [rbp + 0x300], 0x14
140a2230d: c7 45 e0 02 00 00 00        	mov	dword ptr [rbp - 0x20], 0x2
140a22314: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
140a22319: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a22320: 49 b8 41 f0 c8 18 cd 3c d6 6a       	movabs	r8, 0x6ad63ccd18c8f041
140a2232a: 48 89 da                    	mov	rdx, rbx
140a2232d: 41 b1 19                    	mov	r9b, 0x19
140a22330: e8 2b 2f ee ff              	call	0x140905260 <.text+0x904260>
140a22335: 48 8d 5d e0                 	lea	rbx, [rbp - 0x20]
140a22339: 4c 8d b5 70 01 00 00        	lea	r14, [rbp + 0x170]
140a22340: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a22346: 48 89 d9                    	mov	rcx, rbx
140a22349: 4c 89 f2                    	mov	rdx, r14
140a2234c: e8 af 18 f0 01              	call	0x142923c00 <.text+0x2922c00>
140a22351: 4c 8d 7d bc                 	lea	r15, [rbp - 0x44]
140a22355: 4c 89 bd 00 03 00 00        	mov	qword ptr [rbp + 0x300], r15
140a2235c: c7 85 70 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x170], 0x2
140a22366: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
140a2236b: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a22372: 49 b8 68 07 7d b5 a7 c3 f7 e8       	movabs	r8, -0x17083c584a82f898
140a2237c: 48 89 da                    	mov	rdx, rbx
140a2237f: 41 b1 32                    	mov	r9b, 0x32
140a22382: e8 79 b9 ec ff              	call	0x1408edd00 <.text+0x8ecd00>
140a22387: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a2238e: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a22392: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a22398: e8 63 18 f0 01              	call	0x142923c00 <.text+0x2922c00>
140a2239d: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a223a4: 74 2a                       	je	0x140a223d0 <.text+0xa213d0>
140a223a6: 48 8d 8d 80 02 00 00        	lea	rcx, [rbp + 0x280]
140a223ad: ba e8 00 00 00              	mov	edx, 0xe8
140a223b2: 48 03 95 a8 02 00 00        	add	rdx, qword ptr [rbp + 0x2a8]
140a223b9: 49 b8 94 a7 92 30 4f 2c 02 dc       	movabs	r8, -0x23fdd3b0cf6d586c
140a223c3: f3 0f 10 1d bd 35 f0 01     	movss	xmm3, dword ptr [rip + 0x1f035bd] # 0x142925988
140a223cb: e8 60 96 f4 ff              	call	0x14096ba30 <.text+0x96aa30>
140a223d0: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a223d4: 48 8d 95 70 01 00 00        	lea	rdx, [rbp + 0x170]
140a223db: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a223e1: e8 1a 18 f0 01              	call	0x142923c00 <.text+0x2922c00>
140a223e6: 80 bd 16 01 00 00 08        	cmp	byte ptr [rbp + 0x116], 0x8
140a223ed: 74 28                       	je	0x140a22417 <.text+0xa21417>
140a223ef: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a223f6: ba e8 00 00 00              	mov	edx, 0xe8
140a223fb: 48 03 95 18 01 00 00        	add	rdx, qword ptr [rbp + 0x118]
140a22402: 49 b8 28 ae 4f 9d 5b 6c d7 4a       	movabs	r8, 0x4ad76c5b9d4fae28
140a2240c: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a22412: e8 59 91 f4 ff              	call	0x14096b570 <.text+0x96a570>
140a22417: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a2241e: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a22422: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a22428: e8 d3 17 f0 01              	call	0x142923c00 <.text+0x2922c00>
140a2242d: 0f 10 85 10 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x210]
140a22434: 0f 29 85 40 03 00 00        	movaps	xmmword ptr [rbp + 0x340], xmm0
140a2243b: 0f 10 85 00 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x200]
140a22442: 0f 29 85 30 03 00 00        	movaps	xmmword ptr [rbp + 0x330], xmm0
140a22449: 0f 10 85 f0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1f0]
140a22450: 0f 29 85 20 03 00 00        	movaps	xmmword ptr [rbp + 0x320], xmm0
140a22457: 0f 10 85 e0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1e0]
140a2245e: 0f 29 85 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm0
140a22465: 0f 10 85 d0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d0]
140a2246c: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a22473: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
140a2247a: 48 8b 8d f0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f0]
140a22481: 48 8b 95 80 01 00 00        	mov	rdx, qword ptr [rbp + 0x180]
140a22488: 48 89 55 f0                 	mov	qword ptr [rbp - 0x10], rdx
140a2248c: 0f 10 85 70 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x170]
140a22493: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
140a22497: 0f 10 85 88 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x188]
140a2249e: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
140a224a2: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
140a224a9: 48 89 55 20                 	mov	qword ptr [rbp + 0x20], rdx
140a224ad: 0f 10 85 a0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1a0]
140a224b4: 0f 11 45 f8                 	movups	xmmword ptr [rbp - 0x8], xmm0
140a224b8: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
140a224bf: 48 89 55 08                 	mov	qword ptr [rbp + 0x8], rdx
140a224c3: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
140a224ca: 0f 10 8d 30 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x230]
140a224d1: 0f 10 95 40 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x240]
140a224d8: 0f 10 9d 50 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x250]
140a224df: 0f 11 45 58                 	movups	xmmword ptr [rbp + 0x58], xmm0
140a224e3: 0f 11 4d 68                 	movups	xmmword ptr [rbp + 0x68], xmm1
140a224e7: 0f 11 55 78                 	movups	xmmword ptr [rbp + 0x78], xmm2
140a224eb: 0f 11 9d 88 00 00 00        	movups	xmmword ptr [rbp + 0x88], xmm3
140a224f2: 0f 10 85 60 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x260]
140a224f9: 0f 11 85 98 00 00 00        	movups	xmmword ptr [rbp + 0x98], xmm0
140a22500: 0f 10 85 70 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x270]
140a22507: 0f 11 85 a8 00 00 00        	movups	xmmword ptr [rbp + 0xa8], xmm0
140a2250e: 48 8b 95 c8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1c8]
140a22515: 48 89 55 38                 	mov	qword ptr [rbp + 0x38], rdx
140a22519: 0f 10 85 b8 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1b8]
140a22520: 0f 11 45 28                 	movups	xmmword ptr [rbp + 0x28], xmm0
140a22524: 48 89 8d c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rcx
140a2252b: 48 89 85 b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rax
140a22532: 0f 28 35 c7 ff 07 02        	movaps	xmm6, xmmword ptr [rip + 0x207ffc7] # 0x142aa2500
140a22539: 0f 29 75 40                 	movaps	xmmword ptr [rbp + 0x40], xmm6
140a2253d: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a22544: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a22548: e8 83 af fb ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a2254d: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a22554: 74 0c                       	je	0x140a22562 <.text+0xa21562>
140a22556: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a2255d: e8 4e ff 1b 00              	call	0x140be24b0 <.text+0xbe14b0>
140a22562: 48 89 7c 24 30              	mov	qword ptr [rsp + 0x30], rdi
140a22567: 48 89 74 24 28              	mov	qword ptr [rsp + 0x28], rsi
140a2256c: 48 c7 44 24 20 18 00 00 00  	mov	qword ptr [rsp + 0x20], 0x18
140a22575: 48 8d 15 bc 73 0a 02        	lea	rdx, [rip + 0x20a73bc]  # 0x142ac9938
140a2257c: 4c 8d 0d c5 73 0a 02        	lea	r9, [rip + 0x20a73c5]   # 0x142ac9948
140a22583: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a22587: 41 b8 10 00 00 00           	mov	r8d, 0x10
140a2258d: e8 0e 0d be 00              	call	0x1416032a0 <.text+0x16022a0>
140a22592: 80 bd 16 01 00 00 08        	cmp	byte ptr [rbp + 0x116], 0x8
140a22599: 74 25                       	je	0x140a225c0 <.text+0xa215c0>
140a2259b: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a225a2: ba e8 00 00 00              	mov	edx, 0xe8
140a225a7: 48 03 95 18 01 00 00        	add	rdx, qword ptr [rbp + 0x118]
140a225ae: 49 b8 22 bc 94 20 37 f8 1f d9       	movabs	r8, -0x26e007c8df6b43de
140a225b8: 45 31 c9                    	xor	r9d, r9d
140a225bb: e8 90 92 f4 ff              	call	0x14096b850 <.text+0x96a850>
140a225c0: 48 8d b5 70 01 00 00        	lea	rsi, [rbp + 0x170]
140a225c7: 48 8d 7d e0                 	lea	rdi, [rbp - 0x20]
140a225cb: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a225d1: 48 89 f1                    	mov	rcx, rsi
140a225d4: 48 89 fa                    	mov	rdx, rdi
140a225d7: e8 24 16 f0 01              	call	0x142923c00 <.text+0x2922c00>
140a225dc: 4c 89 bd 00 03 00 00        	mov	qword ptr [rbp + 0x300], r15
140a225e3: c7 45 e0 02 00 00 00        	mov	dword ptr [rbp - 0x20], 0x2
140a225ea: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a225ef: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a225f6: 49 b8 41 f0 c8 18 cd 3c d6 6a       	movabs	r8, 0x6ad63ccd18c8f041
140a22600: 48 89 f2                    	mov	rdx, rsi
140a22603: 41 b1 19                    	mov	r9b, 0x19
140a22606: e8 35 bc ec ff              	call	0x1408ee240 <.text+0x8ed240>
140a2260b: 48 8d 75 e0                 	lea	rsi, [rbp - 0x20]
140a2260f: 48 8d bd 70 01 00 00        	lea	rdi, [rbp + 0x170]
140a22616: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a2261c: 48 89 f1                    	mov	rcx, rsi
140a2261f: 48 89 fa                    	mov	rdx, rdi
140a22622: e8 d9 15 f0 01              	call	0x142923c00 <.text+0x2922c00>
140a22627: c6 85 00 03 00 00 14        	mov	byte ptr [rbp + 0x300], 0x14
140a2262e: c7 85 70 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x170], 0x2
140a22638: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a2263d: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a22644: 49 b8 68 07 7d b5 a7 c3 f7 e8       	movabs	r8, -0x17083c584a82f898
140a2264e: 48 89 f2                    	mov	rdx, rsi
140a22651: 41 b1 32                    	mov	r9b, 0x32
140a22654: e8 07 2c ee ff              	call	0x140905260 <.text+0x904260>
140a22659: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a22660: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a22664: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a2266a: e8 91 15 f0 01              	call	0x142923c00 <.text+0x2922c00>
140a2266f: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a22676: 74 2a                       	je	0x140a226a2 <.text+0xa216a2>
140a22678: 48 8d 8d 80 02 00 00        	lea	rcx, [rbp + 0x280]
140a2267f: ba e8 00 00 00              	mov	edx, 0xe8
140a22684: 48 03 95 a8 02 00 00        	add	rdx, qword ptr [rbp + 0x2a8]
140a2268b: 49 b8 94 a7 92 30 4f 2c 02 dc       	movabs	r8, -0x23fdd3b0cf6d586c
140a22695: f3 0f 10 1d eb 32 f0 01     	movss	xmm3, dword ptr [rip + 0x1f032eb] # 0x142925988
140a2269d: e8 8e 93 f4 ff              	call	0x14096ba30 <.text+0x96aa30>
140a226a2: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a226a6: 48 8d 95 70 01 00 00        	lea	rdx, [rbp + 0x170]
140a226ad: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a226b3: e8 48 15 f0 01              	call	0x142923c00 <.text+0x2922c00>
140a226b8: 80 bd 16 01 00 00 08        	cmp	byte ptr [rbp + 0x116], 0x8
140a226bf: 74 28                       	je	0x140a226e9 <.text+0xa216e9>
140a226c1: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a226c8: ba e8 00 00 00              	mov	edx, 0xe8
140a226cd: 48 03 95 18 01 00 00        	add	rdx, qword ptr [rbp + 0x118]
140a226d4: 49 b8 28 ae 4f 9d 5b 6c d7 4a       	movabs	r8, 0x4ad76c5b9d4fae28
140a226de: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a226e4: e8 87 8e f4 ff              	call	0x14096b570 <.text+0x96a570>
140a226e9: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a226f0: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a226f4: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a226fa: e8 01 15 f0 01              	call	0x142923c00 <.text+0x2922c00>
140a226ff: 0f 10 85 10 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x210]
140a22706: 0f 29 85 40 03 00 00        	movaps	xmmword ptr [rbp + 0x340], xmm0
140a2270d: 0f 10 85 00 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x200]
140a22714: 0f 29 85 30 03 00 00        	movaps	xmmword ptr [rbp + 0x330], xmm0
140a2271b: 0f 10 85 f0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1f0]
140a22722: 0f 29 85 20 03 00 00        	movaps	xmmword ptr [rbp + 0x320], xmm0
140a22729: 0f 10 85 e0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1e0]
140a22730: 0f 29 85 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm0
140a22737: 0f 10 85 d0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d0]
140a2273e: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a22745: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
140a2274c: 48 8b 8d f0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f0]
140a22753: 48 8b 95 80 01 00 00        	mov	rdx, qword ptr [rbp + 0x180]
140a2275a: 48 89 55 f0                 	mov	qword ptr [rbp - 0x10], rdx
140a2275e: 0f 10 85 70 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x170]
140a22765: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
140a22769: 0f 10 85 88 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x188]
140a22770: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
140a22774: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
140a2277b: 48 89 55 20                 	mov	qword ptr [rbp + 0x20], rdx
140a2277f: 0f 10 85 a0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1a0]
140a22786: 0f 11 45 f8                 	movups	xmmword ptr [rbp - 0x8], xmm0
140a2278a: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
140a22791: 48 89 55 08                 	mov	qword ptr [rbp + 0x8], rdx
140a22795: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
140a2279c: 0f 10 8d 30 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x230]
140a227a3: 0f 10 95 40 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x240]
140a227aa: 0f 10 9d 50 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x250]
140a227b1: 0f 11 45 58                 	movups	xmmword ptr [rbp + 0x58], xmm0
140a227b5: 0f 11 4d 68                 	movups	xmmword ptr [rbp + 0x68], xmm1
140a227b9: 0f 11 55 78                 	movups	xmmword ptr [rbp + 0x78], xmm2
140a227bd: 0f 11 9d 88 00 00 00        	movups	xmmword ptr [rbp + 0x88], xmm3
140a227c4: 0f 10 85 60 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x260]
140a227cb: 0f 11 85 98 00 00 00        	movups	xmmword ptr [rbp + 0x98], xmm0
140a227d2: 0f 10 85 70 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x270]
140a227d9: 0f 11 85 a8 00 00 00        	movups	xmmword ptr [rbp + 0xa8], xmm0
140a227e0: 48 8b 95 c8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1c8]
140a227e7: 48 89 55 38                 	mov	qword ptr [rbp + 0x38], rdx
140a227eb: 0f 10 85 b8 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1b8]
140a227f2: 0f 11 45 28                 	movups	xmmword ptr [rbp + 0x28], xmm0
140a227f6: 48 89 8d c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rcx
140a227fd: 48 89 85 b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rax
140a22804: 0f 29 75 40                 	movaps	xmmword ptr [rbp + 0x40], xmm6
140a22808: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a2280f: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a22813: e8 b8 ac fb ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a22818: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a2281f: 74 0c                       	je	0x140a2282d <.text+0xa2182d>
140a22821: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a22828: e8 83 fc 1b 00              	call	0x140be24b0 <.text+0xbe14b0>
140a2282d: 0f 28 b5 60 03 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x360]
140a22834: 48 81 c4 f8 03 00 00        	add	rsp, 0x3f8
140a2283b: 5b                          	pop	rbx
140a2283c: 5f                          	pop	rdi
140a2283d: 5e                          	pop	rsi
140a2283e: 41 5e                       	pop	r14
140a22840: 41 5f                       	pop	r15
140a22842: 5d                          	pop	rbp
140a22843: c3                          	ret
140a22844: 48 8d 0d c5 70 0a 02        	lea	rcx, [rip + 0x20a70c5]  # 0x142ac9910
140a2284b: e8 c0 6a eb 01              	call	0x1428d9310 <.text+0x28d8310>
140a22850: cc                          	int3
140a22851: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140a22860: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a22865: 55                          	push	rbp
140a22866: 41 57                       	push	r15
140a22868: 41 56                       	push	r14
140a2286a: 56                          	push	rsi
140a2286b: 57                          	push	rdi
140a2286c: 53                          	push	rbx
140a2286d: 48 83 ec 48                 	sub	rsp, 0x48
140a22871: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a22878: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a2287d: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a22884: 74 0c                       	je	0x140a22892 <.text+0xa21892>
140a22886: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a2288d: e8 1e fc 1b 00              	call	0x140be24b0 <.text+0xbe14b0>
140a22892: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a22897: 48 83 c4 48                 	add	rsp, 0x48
140a2289b: 5b                          	pop	rbx
140a2289c: 5f                          	pop	rdi
140a2289d: 5e                          	pop	rsi
140a2289e: 41 5e                       	pop	r14
140a228a0: 41 5f                       	pop	r15
140a228a2: 5d                          	pop	rbp
140a228a3: c3                          	ret
140a228a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
140a228b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a228b5: 55                          	push	rbp
140a228b6: 41 57                       	push	r15
140a228b8: 41 56                       	push	r14
140a228ba: 56                          	push	rsi
140a228bb: 57                          	push	rdi
140a228bc: 53                          	push	rbx
140a228bd: 48 83 ec 48                 	sub	rsp, 0x48
140a228c1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a228c8: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a228cd: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a228d1: e8 da 46 e7 ff              	call	0x140896fb0 <.text+0x895fb0>
140a228d6: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a228db: 48 83 c4 48                 	add	rsp, 0x48
140a228df: 5b                          	pop	rbx
140a228e0: 5f                          	pop	rdi
140a228e1: 5e                          	pop	rsi
140a228e2: 41 5e                       	pop	r14
140a228e4: 41 5f                       	pop	r15
140a228e6: 5d                          	pop	rbp
140a228e7: c3                          	ret
140a228e8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a228f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a228f5: 55                          	push	rbp
140a228f6: 41 57                       	push	r15
140a228f8: 41 56                       	push	r14
140a228fa: 56                          	push	rsi
140a228fb: 57                          	push	rdi
140a228fc: 53                          	push	rbx
140a228fd: 48 83 ec 48                 	sub	rsp, 0x48
140a22901: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a22908: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a2290d: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a22914: e8 97 46 e7 ff              	call	0x140896fb0 <.text+0x895fb0>
140a22919: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a2291e: 48 83 c4 48                 	add	rsp, 0x48
140a22922: 5b                          	pop	rbx
140a22923: 5f                          	pop	rdi
140a22924: 5e                          	pop	rsi
140a22925: 41 5e                       	pop	r14
140a22927: 41 5f                       	pop	r15
140a22929: 5d                          	pop	rbp
140a2292a: c3                          	ret
140a2292b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140a22930: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a22935: 55                          	push	rbp
140a22936: 41 57                       	push	r15
140a22938: 41 56                       	push	r14
140a2293a: 56                          	push	rsi
140a2293b: 57                          	push	rdi
140a2293c: 53                          	push	rbx
140a2293d: 48 83 ec 48                 	sub	rsp, 0x48
140a22941: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a22948: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a2294d: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a22951: e8 5a 46 e7 ff              	call	0x140896fb0 <.text+0x895fb0>
140a22956: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a2295b: 48 83 c4 48                 	add	rsp, 0x48
140a2295f: 5b                          	pop	rbx
140a22960: 5f                          	pop	rdi
140a22961: 5e                          	pop	rsi
140a22962: 41 5e                       	pop	r14
140a22964: 41 5f                       	pop	r15
140a22966: 5d                          	pop	rbp
140a22967: c3                          	ret
140a22968: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a22970: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a22975: 55                          	push	rbp
140a22976: 41 57                       	push	r15
140a22978: 41 56                       	push	r14
140a2297a: 56                          	push	rsi
140a2297b: 57                          	push	rdi
140a2297c: 53                          	push	rbx
140a2297d: 48 83 ec 48                 	sub	rsp, 0x48
140a22981: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a22988: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a2298d: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a22994: e8 17 46 e7 ff              	call	0x140896fb0 <.text+0x895fb0>
140a22999: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a2299e: 48 83 c4 48                 	add	rsp, 0x48
140a229a2: 5b                          	pop	rbx
140a229a3: 5f                          	pop	rdi
140a229a4: 5e                          	pop	rsi
140a229a5: 41 5e                       	pop	r14
140a229a7: 41 5f                       	pop	r15
140a229a9: 5d                          	pop	rbp
140a229aa: c3                          	ret
140a229ab: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140a229b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a229b5: 55                          	push	rbp
140a229b6: 41 57                       	push	r15
140a229b8: 41 56                       	push	r14
140a229ba: 56                          	push	rsi
140a229bb: 57                          	push	rdi
140a229bc: 53                          	push	rbx
140a229bd: 48 83 ec 48                 	sub	rsp, 0x48
140a229c1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a229c8: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a229cd: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a229d1: e8 da 45 e7 ff              	call	0x140896fb0 <.text+0x895fb0>
140a229d6: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a229db: 48 83 c4 48                 	add	rsp, 0x48
140a229df: 5b                          	pop	rbx
140a229e0: 5f                          	pop	rdi
140a229e1: 5e                          	pop	rsi
140a229e2: 41 5e                       	pop	r14
140a229e4: 41 5f                       	pop	r15
140a229e6: 5d                          	pop	rbp
140a229e7: c3                          	ret
140a229e8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a229f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a229f5: 55                          	push	rbp
140a229f6: 41 57                       	push	r15
140a229f8: 41 56                       	push	r14
140a229fa: 56                          	push	rsi
140a229fb: 57                          	push	rdi
140a229fc: 53                          	push	rbx
140a229fd: 48 83 ec 48                 	sub	rsp, 0x48
140a22a01: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a22a08: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a22a0d: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a22a14: 74 0c                       	je	0x140a22a22 <.text+0xa21a22>
140a22a16: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a22a1d: e8 8e fa 1b 00              	call	0x140be24b0 <.text+0xbe14b0>
140a22a22: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a22a27: 48 83 c4 48                 	add	rsp, 0x48
140a22a2b: 5b                          	pop	rbx
140a22a2c: 5f                          	pop	rdi
140a22a2d: 5e                          	pop	rsi
140a22a2e: 41 5e                       	pop	r14
140a22a30: 41 5f                       	pop	r15
140a22a32: 5d                          	pop	rbp
140a22a33: c3                          	ret
140a22a34: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
140a22a40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a22a45: 55                          	push	rbp
140a22a46: 41 57                       	push	r15
140a22a48: 41 56                       	push	r14
140a22a4a: 56                          	push	rsi
140a22a4b: 57                          	push	rdi
140a22a4c: 53                          	push	rbx
140a22a4d: 48 83 ec 48                 	sub	rsp, 0x48
140a22a51: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a22a58: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a22a5d: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a22a61: e8 4a 45 e7 ff              	call	0x140896fb0 <.text+0x895fb0>
140a22a66: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a22a6b: 48 83 c4 48                 	add	rsp, 0x48
140a22a6f: 5b                          	pop	rbx
140a22a70: 5f                          	pop	rdi
140a22a71: 5e                          	pop	rsi
140a22a72: 41 5e                       	pop	r14
140a22a74: 41 5f                       	pop	r15
140a22a76: 5d                          	pop	rbp
140a22a77: c3                          	ret
140a22a78: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a22a80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a22a85: 55                          	push	rbp
140a22a86: 41 57                       	push	r15
140a22a88: 41 56                       	push	r14
140a22a8a: 56                          	push	rsi
140a22a8b: 57                          	push	rdi
140a22a8c: 53                          	push	rbx
140a22a8d: 48 83 ec 48                 	sub	rsp, 0x48
140a22a91: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a22a98: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a22a9d: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a22aa4: e8 07 45 e7 ff              	call	0x140896fb0 <.text+0x895fb0>
140a22aa9: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a22aae: 48 83 c4 48                 	add	rsp, 0x48
140a22ab2: 5b                          	pop	rbx
140a22ab3: 5f                          	pop	rdi
140a22ab4: 5e                          	pop	rsi
140a22ab5: 41 5e                       	pop	r14
140a22ab7: 41 5f                       	pop	r15
140a22ab9: 5d                          	pop	rbp
140a22aba: c3                          	ret
140a22abb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140a22ac0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a22ac5: 55                          	push	rbp
140a22ac6: 41 57                       	push	r15
140a22ac8: 41 56                       	push	r14
140a22aca: 56                          	push	rsi
140a22acb: 57                          	push	rdi
140a22acc: 53                          	push	rbx
140a22acd: 48 83 ec 48                 	sub	rsp, 0x48
140a22ad1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a22ad8: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a22add: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a22ae1: e8 ca 44 e7 ff              	call	0x140896fb0 <.text+0x895fb0>
140a22ae6: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a22aeb: 48 83 c4 48                 	add	rsp, 0x48
140a22aef: 5b                          	pop	rbx
140a22af0: 5f                          	pop	rdi
140a22af1: 5e                          	pop	rsi
140a22af2: 41 5e                       	pop	r14
140a22af4: 41 5f                       	pop	r15
140a22af6: 5d                          	pop	rbp
140a22af7: c3                          	ret
140a22af8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a22b00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a22b05: 55                          	push	rbp
140a22b06: 41 57                       	push	r15
140a22b08: 41 56                       	push	r14
140a22b0a: 56                          	push	rsi
140a22b0b: 57                          	push	rdi
140a22b0c: 53                          	push	rbx
140a22b0d: 48 83 ec 48                 	sub	rsp, 0x48
140a22b11: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a22b18: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a22b1d: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a22b24: e8 87 44 e7 ff              	call	0x140896fb0 <.text+0x895fb0>
140a22b29: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a22b2e: 48 83 c4 48                 	add	rsp, 0x48
140a22b32: 5b                          	pop	rbx
140a22b33: 5f                          	pop	rdi
140a22b34: 5e                          	pop	rsi
140a22b35: 41 5e                       	pop	r14
140a22b37: 41 5f                       	pop	r15
140a22b39: 5d                          	pop	rbp
140a22b3a: c3                          	ret
140a22b3b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140a22b40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a22b45: 55                          	push	rbp
140a22b46: 41 57                       	push	r15
140a22b48: 41 56                       	push	r14
140a22b4a: 56                          	push	rsi
140a22b4b: 57                          	push	rdi
140a22b4c: 53                          	push	rbx
140a22b4d: 48 83 ec 48                 	sub	rsp, 0x48
140a22b51: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a22b58: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a22b5d: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a22b61: e8 4a 44 e7 ff              	call	0x140896fb0 <.text+0x895fb0>
140a22b66: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a22b6b: 48 83 c4 48                 	add	rsp, 0x48
140a22b6f: 5b                          	pop	rbx
140a22b70: 5f                          	pop	rdi
140a22b71: 5e                          	pop	rsi
140a22b72: 41 5e                       	pop	r14
140a22b74: 41 5f                       	pop	r15
140a22b76: 5d                          	pop	rbp
140a22b77: c3                          	ret

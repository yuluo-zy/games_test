
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a12680: 55                          	push	rbp
140a12681: 41 57                       	push	r15
140a12683: 41 56                       	push	r14
140a12685: 56                          	push	rsi
140a12686: 57                          	push	rdi
140a12687: 53                          	push	rbx
140a12688: 48 81 ec f8 03 00 00        	sub	rsp, 0x3f8
140a1268f: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a12697: 0f 29 b5 60 03 00 00        	movaps	xmmword ptr [rbp + 0x360], xmm6
140a1269e: 48 c7 85 58 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x358], -0x2
140a126a9: 48 89 d6                    	mov	rsi, rdx
140a126ac: 48 89 cf                    	mov	rdi, rcx
140a126af: 48 8d 4a 38                 	lea	rcx, [rdx + 0x38]
140a126b3: 48 ba ee a0 86 62 93 82 d1 1a       	movabs	rdx, 0x1ad182936286a0ee
140a126bd: 49 b8 6f 7e 12 f9 b6 e3 04 ac       	movabs	r8, -0x53fb1c4906ed8191
140a126c7: e8 24 b0 6c ff              	call	0x1400dd6f0 <.text+0xdc6f0>
140a126cc: 48 85 c0                    	test	rax, rax
140a126cf: 0f 84 45 0c 00 00           	je	0x140a1331a <.text+0xa1231a>
140a126d5: 48 39 96 18 02 00 00        	cmp	qword ptr [rsi + 0x218], rdx
140a126dc: 0f 86 38 0c 00 00           	jbe	0x140a1331a <.text+0xa1231a>
140a126e2: 48 8b 86 10 02 00 00        	mov	rax, qword ptr [rsi + 0x210]
140a126e9: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
140a126ed: 48 85 c9                    	test	rcx, rcx
140a126f0: 0f 84 24 0c 00 00           	je	0x140a1331a <.text+0xa1231a>
140a126f6: 48 8b 86 e0 01 00 00        	mov	rax, qword ptr [rsi + 0x1e0]
140a126fd: 48 f7 d1                    	not	rcx
140a12700: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
140a12704: 48 c1 e1 05                 	shl	rcx, 0x5
140a12708: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
140a1270e: 0f 84 06 0c 00 00           	je	0x140a1331a <.text+0xa1231a>
140a12714: 48 01 c8                    	add	rax, rcx
140a12717: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
140a1271b: 8b 96 b0 05 00 00           	mov	edx, dword ptr [rsi + 0x5b0]
140a12721: 80 b9 9b 02 00 00 00        	cmp	byte ptr [rcx + 0x29b], 0x0
140a12728: 0f 84 d5 0b 00 00           	je	0x140a13303 <.text+0xa12303>
140a1272e: 89 50 5c                    	mov	dword ptr [rax + 0x5c], edx
140a12731: c6 81 9b 02 00 00 00        	mov	byte ptr [rcx + 0x29b], 0x0
140a12738: 48 81 c1 98 00 00 00        	add	rcx, 0x98
140a1273f: 41 b8 00 02 00 00           	mov	r8d, 0x200
140a12745: 31 d2                       	xor	edx, edx
140a12747: e8 d4 14 f1 01              	call	0x142923c20 <.text+0x2922c20>
140a1274c: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a12753: 48 ba 00 02 00 00 00 02 00 00       	movabs	rdx, 0x20000000200
140a1275d: 41 b8 61 00 00 00           	mov	r8d, 0x61
140a12763: e8 38 2b 1c 00              	call	0x140bd52a0 <.text+0xbd42a0>
140a12768: 0f 10 85 00 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x300]
140a1276f: 0f 10 8d 10 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x310]
140a12776: 0f 29 8d 80 01 00 00        	movaps	xmmword ptr [rbp + 0x180], xmm1
140a1277d: 0f 29 85 70 01 00 00        	movaps	xmmword ptr [rbp + 0x170], xmm0
140a12784: 48 8d 4d f0                 	lea	rcx, [rbp - 0x10]
140a12788: 48 8d 95 70 01 00 00        	lea	rdx, [rbp + 0x170]
140a1278f: e8 4c 45 1c 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
140a12794: 48 8d 05 2d 55 0b 02        	lea	rax, [rip + 0x20b552d]  # 0x142ac7cc8
140a1279b: 48 89 45 e0                 	mov	qword ptr [rbp - 0x20], rax
140a1279f: 48 c7 45 e8 0f 00 00 00     	mov	qword ptr [rbp - 0x18], 0xf
140a127a7: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a127ab: 48 89 f9                    	mov	rcx, rdi
140a127ae: e8 1d 6d 1d 00              	call	0x140be94d0 <.text+0xbe84d0>
140a127b3: 0f 10 85 00 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x300]
140a127ba: 0f 10 8d 10 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x310]
140a127c1: 0f 11 45 c0                 	movups	xmmword ptr [rbp - 0x40], xmm0
140a127c5: 0f 11 4d d0                 	movups	xmmword ptr [rbp - 0x30], xmm1
140a127c9: 89 45 bc                    	mov	dword ptr [rbp - 0x44], eax
140a127cc: 48 89 7c 24 30              	mov	qword ptr [rsp + 0x30], rdi
140a127d1: 48 89 74 24 28              	mov	qword ptr [rsp + 0x28], rsi
140a127d6: 48 c7 44 24 20 18 00 00 00  	mov	qword ptr [rsp + 0x20], 0x18
140a127df: 48 8d 15 f1 54 0b 02        	lea	rdx, [rip + 0x20b54f1]  # 0x142ac7cd7
140a127e6: 4c 8d 0d f9 54 0b 02        	lea	r9, [rip + 0x20b54f9]   # 0x142ac7ce6
140a127ed: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a127f1: 41 b8 0f 00 00 00           	mov	r8d, 0xf
140a127f7: e8 a4 0a bf 00              	call	0x1416032a0 <.text+0x16022a0>
140a127fc: 80 bd 16 01 00 00 08        	cmp	byte ptr [rbp + 0x116], 0x8
140a12803: 74 25                       	je	0x140a1282a <.text+0xa1182a>
140a12805: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a1280c: ba e8 00 00 00              	mov	edx, 0xe8
140a12811: 48 03 95 18 01 00 00        	add	rdx, qword ptr [rbp + 0x118]
140a12818: 49 b8 22 bc 94 20 37 f8 1f d9       	movabs	r8, -0x26e007c8df6b43de
140a12822: 41 b1 01                    	mov	r9b, 0x1
140a12825: e8 26 90 f5 ff              	call	0x14096b850 <.text+0x96a850>
140a1282a: 48 8d 9d 70 01 00 00        	lea	rbx, [rbp + 0x170]
140a12831: 4c 8d 75 e0                 	lea	r14, [rbp - 0x20]
140a12835: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a1283b: 48 89 d9                    	mov	rcx, rbx
140a1283e: 4c 89 f2                    	mov	rdx, r14
140a12841: e8 ba 13 f1 01              	call	0x142923c00 <.text+0x2922c00>
140a12846: c6 85 00 03 00 00 13        	mov	byte ptr [rbp + 0x300], 0x13
140a1284d: c7 45 e0 02 00 00 00        	mov	dword ptr [rbp - 0x20], 0x2
140a12854: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
140a12859: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a12860: 49 b8 41 f0 c8 18 cd 3c d6 6a       	movabs	r8, 0x6ad63ccd18c8f041
140a1286a: 48 89 da                    	mov	rdx, rbx
140a1286d: 41 b1 19                    	mov	r9b, 0x19
140a12870: e8 eb 29 ef ff              	call	0x140905260 <.text+0x904260>
140a12875: 48 8d 5d e0                 	lea	rbx, [rbp - 0x20]
140a12879: 4c 8d b5 70 01 00 00        	lea	r14, [rbp + 0x170]
140a12880: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a12886: 48 89 d9                    	mov	rcx, rbx
140a12889: 4c 89 f2                    	mov	rdx, r14
140a1288c: e8 6f 13 f1 01              	call	0x142923c00 <.text+0x2922c00>
140a12891: 4c 8d 7d bc                 	lea	r15, [rbp - 0x44]
140a12895: 4c 89 bd 00 03 00 00        	mov	qword ptr [rbp + 0x300], r15
140a1289c: c7 85 70 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x170], 0x2
140a128a6: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
140a128ab: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a128b2: 49 b8 68 07 7d b5 a7 c3 f7 e8       	movabs	r8, -0x17083c584a82f898
140a128bc: 48 89 da                    	mov	rdx, rbx
140a128bf: 41 b1 32                    	mov	r9b, 0x32
140a128c2: e8 39 b4 ed ff              	call	0x1408edd00 <.text+0x8ecd00>
140a128c7: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a128ce: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a128d2: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a128d8: e8 23 13 f1 01              	call	0x142923c00 <.text+0x2922c00>
140a128dd: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a128e4: 74 2a                       	je	0x140a12910 <.text+0xa11910>
140a128e6: 48 8d 8d 80 02 00 00        	lea	rcx, [rbp + 0x280]
140a128ed: ba e8 00 00 00              	mov	edx, 0xe8
140a128f2: 48 03 95 a8 02 00 00        	add	rdx, qword ptr [rbp + 0x2a8]
140a128f9: 49 b8 94 a7 92 30 4f 2c 02 dc       	movabs	r8, -0x23fdd3b0cf6d586c
140a12903: f3 0f 10 1d 9d 1c f2 01     	movss	xmm3, dword ptr [rip + 0x1f21c9d] # 0x1429345a8
140a1290b: e8 20 91 f5 ff              	call	0x14096ba30 <.text+0x96aa30>
140a12910: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a12914: 48 8d 95 70 01 00 00        	lea	rdx, [rbp + 0x170]
140a1291b: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a12921: e8 da 12 f1 01              	call	0x142923c00 <.text+0x2922c00>
140a12926: 80 bd 16 01 00 00 08        	cmp	byte ptr [rbp + 0x116], 0x8
140a1292d: 74 28                       	je	0x140a12957 <.text+0xa11957>
140a1292f: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a12936: ba e8 00 00 00              	mov	edx, 0xe8
140a1293b: 48 03 95 18 01 00 00        	add	rdx, qword ptr [rbp + 0x118]
140a12942: 49 b8 28 ae 4f 9d 5b 6c d7 4a       	movabs	r8, 0x4ad76c5b9d4fae28
140a1294c: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a12952: e8 19 8c f5 ff              	call	0x14096b570 <.text+0x96a570>
140a12957: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a1295e: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a12962: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a12968: e8 93 12 f1 01              	call	0x142923c00 <.text+0x2922c00>
140a1296d: 0f 10 85 10 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x210]
140a12974: 0f 29 85 40 03 00 00        	movaps	xmmword ptr [rbp + 0x340], xmm0
140a1297b: 0f 10 85 00 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x200]
140a12982: 0f 29 85 30 03 00 00        	movaps	xmmword ptr [rbp + 0x330], xmm0
140a12989: 0f 10 85 f0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1f0]
140a12990: 0f 29 85 20 03 00 00        	movaps	xmmword ptr [rbp + 0x320], xmm0
140a12997: 0f 10 85 e0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1e0]
140a1299e: 0f 29 85 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm0
140a129a5: 0f 10 85 d0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d0]
140a129ac: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a129b3: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
140a129ba: 48 8b 8d f0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f0]
140a129c1: 48 8b 95 80 01 00 00        	mov	rdx, qword ptr [rbp + 0x180]
140a129c8: 48 89 55 f0                 	mov	qword ptr [rbp - 0x10], rdx
140a129cc: 0f 10 85 70 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x170]
140a129d3: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
140a129d7: 0f 10 85 88 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x188]
140a129de: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
140a129e2: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
140a129e9: 48 89 55 20                 	mov	qword ptr [rbp + 0x20], rdx
140a129ed: 0f 10 85 a0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1a0]
140a129f4: 0f 11 45 f8                 	movups	xmmword ptr [rbp - 0x8], xmm0
140a129f8: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
140a129ff: 48 89 55 08                 	mov	qword ptr [rbp + 0x8], rdx
140a12a03: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
140a12a0a: 0f 10 8d 30 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x230]
140a12a11: 0f 10 95 40 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x240]
140a12a18: 0f 10 9d 50 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x250]
140a12a1f: 0f 11 45 58                 	movups	xmmword ptr [rbp + 0x58], xmm0
140a12a23: 0f 11 4d 68                 	movups	xmmword ptr [rbp + 0x68], xmm1
140a12a27: 0f 11 55 78                 	movups	xmmword ptr [rbp + 0x78], xmm2
140a12a2b: 0f 11 9d 88 00 00 00        	movups	xmmword ptr [rbp + 0x88], xmm3
140a12a32: 0f 10 85 60 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x260]
140a12a39: 0f 11 85 98 00 00 00        	movups	xmmword ptr [rbp + 0x98], xmm0
140a12a40: 0f 10 85 70 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x270]
140a12a47: 0f 11 85 a8 00 00 00        	movups	xmmword ptr [rbp + 0xa8], xmm0
140a12a4e: 48 8b 95 c8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1c8]
140a12a55: 48 89 55 38                 	mov	qword ptr [rbp + 0x38], rdx
140a12a59: 0f 10 85 b8 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1b8]
140a12a60: 0f 11 45 28                 	movups	xmmword ptr [rbp + 0x28], xmm0
140a12a64: 48 89 8d c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rcx
140a12a6b: 48 89 85 b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rax
140a12a72: 0f 28 35 87 fa 08 02        	movaps	xmm6, xmmword ptr [rip + 0x208fa87] # 0x142aa2500
140a12a79: 0f 29 75 40                 	movaps	xmmword ptr [rbp + 0x40], xmm6
140a12a7d: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a12a84: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a12a88: e8 43 aa fc ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a12a8d: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a12a94: 74 0c                       	je	0x140a12aa2 <.text+0xa11aa2>
140a12a96: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a12a9d: e8 0e fa 1c 00              	call	0x140be24b0 <.text+0xbe14b0>
140a12aa2: 48 89 7c 24 30              	mov	qword ptr [rsp + 0x30], rdi
140a12aa7: 48 89 74 24 28              	mov	qword ptr [rsp + 0x28], rsi
140a12aac: 48 c7 44 24 20 18 00 00 00  	mov	qword ptr [rsp + 0x20], 0x18
140a12ab5: 48 8d 15 1b 52 0b 02        	lea	rdx, [rip + 0x20b521b]  # 0x142ac7cd7
140a12abc: 4c 8d 0d 23 52 0b 02        	lea	r9, [rip + 0x20b5223]   # 0x142ac7ce6
140a12ac3: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a12ac7: 41 b8 0f 00 00 00           	mov	r8d, 0xf
140a12acd: e8 ce 07 bf 00              	call	0x1416032a0 <.text+0x16022a0>
140a12ad2: 80 bd 16 01 00 00 08        	cmp	byte ptr [rbp + 0x116], 0x8
140a12ad9: 74 25                       	je	0x140a12b00 <.text+0xa11b00>
140a12adb: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a12ae2: ba e8 00 00 00              	mov	edx, 0xe8
140a12ae7: 48 03 95 18 01 00 00        	add	rdx, qword ptr [rbp + 0x118]
140a12aee: 49 b8 22 bc 94 20 37 f8 1f d9       	movabs	r8, -0x26e007c8df6b43de
140a12af8: 45 31 c9                    	xor	r9d, r9d
140a12afb: e8 50 8d f5 ff              	call	0x14096b850 <.text+0x96a850>
140a12b00: 48 8d 9d 70 01 00 00        	lea	rbx, [rbp + 0x170]
140a12b07: 4c 8d 75 e0                 	lea	r14, [rbp - 0x20]
140a12b0b: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a12b11: 48 89 d9                    	mov	rcx, rbx
140a12b14: 4c 89 f2                    	mov	rdx, r14
140a12b17: e8 e4 10 f1 01              	call	0x142923c00 <.text+0x2922c00>
140a12b1c: 4c 89 bd 00 03 00 00        	mov	qword ptr [rbp + 0x300], r15
140a12b23: c7 45 e0 02 00 00 00        	mov	dword ptr [rbp - 0x20], 0x2
140a12b2a: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
140a12b2f: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a12b36: 49 b8 41 f0 c8 18 cd 3c d6 6a       	movabs	r8, 0x6ad63ccd18c8f041
140a12b40: 48 89 da                    	mov	rdx, rbx
140a12b43: 41 b1 19                    	mov	r9b, 0x19
140a12b46: e8 f5 b6 ed ff              	call	0x1408ee240 <.text+0x8ed240>
140a12b4b: 48 8d 5d e0                 	lea	rbx, [rbp - 0x20]
140a12b4f: 4c 8d b5 70 01 00 00        	lea	r14, [rbp + 0x170]
140a12b56: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a12b5c: 48 89 d9                    	mov	rcx, rbx
140a12b5f: 4c 89 f2                    	mov	rdx, r14
140a12b62: e8 99 10 f1 01              	call	0x142923c00 <.text+0x2922c00>
140a12b67: c6 85 00 03 00 00 13        	mov	byte ptr [rbp + 0x300], 0x13
140a12b6e: c7 85 70 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x170], 0x2
140a12b78: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
140a12b7d: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a12b84: 49 b8 68 07 7d b5 a7 c3 f7 e8       	movabs	r8, -0x17083c584a82f898
140a12b8e: 48 89 da                    	mov	rdx, rbx
140a12b91: 41 b1 32                    	mov	r9b, 0x32
140a12b94: e8 c7 26 ef ff              	call	0x140905260 <.text+0x904260>
140a12b99: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a12ba0: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a12ba4: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a12baa: e8 51 10 f1 01              	call	0x142923c00 <.text+0x2922c00>
140a12baf: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a12bb6: 74 2a                       	je	0x140a12be2 <.text+0xa11be2>
140a12bb8: 48 8d 8d 80 02 00 00        	lea	rcx, [rbp + 0x280]
140a12bbf: ba e8 00 00 00              	mov	edx, 0xe8
140a12bc4: 48 03 95 a8 02 00 00        	add	rdx, qword ptr [rbp + 0x2a8]
140a12bcb: 49 b8 94 a7 92 30 4f 2c 02 dc       	movabs	r8, -0x23fdd3b0cf6d586c
140a12bd5: f3 0f 10 1d cb 19 f2 01     	movss	xmm3, dword ptr [rip + 0x1f219cb] # 0x1429345a8
140a12bdd: e8 4e 8e f5 ff              	call	0x14096ba30 <.text+0x96aa30>
140a12be2: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a12be6: 48 8d 95 70 01 00 00        	lea	rdx, [rbp + 0x170]
140a12bed: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a12bf3: e8 08 10 f1 01              	call	0x142923c00 <.text+0x2922c00>
140a12bf8: 80 bd 16 01 00 00 08        	cmp	byte ptr [rbp + 0x116], 0x8
140a12bff: 74 28                       	je	0x140a12c29 <.text+0xa11c29>
140a12c01: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a12c08: ba e8 00 00 00              	mov	edx, 0xe8
140a12c0d: 48 03 95 18 01 00 00        	add	rdx, qword ptr [rbp + 0x118]
140a12c14: 49 b8 28 ae 4f 9d 5b 6c d7 4a       	movabs	r8, 0x4ad76c5b9d4fae28
140a12c1e: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a12c24: e8 47 89 f5 ff              	call	0x14096b570 <.text+0x96a570>
140a12c29: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a12c30: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a12c34: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a12c3a: e8 c1 0f f1 01              	call	0x142923c00 <.text+0x2922c00>
140a12c3f: 0f 10 85 10 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x210]
140a12c46: 0f 29 85 40 03 00 00        	movaps	xmmword ptr [rbp + 0x340], xmm0
140a12c4d: 0f 10 85 00 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x200]
140a12c54: 0f 29 85 30 03 00 00        	movaps	xmmword ptr [rbp + 0x330], xmm0
140a12c5b: 0f 10 85 f0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1f0]
140a12c62: 0f 29 85 20 03 00 00        	movaps	xmmword ptr [rbp + 0x320], xmm0
140a12c69: 0f 10 85 e0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1e0]
140a12c70: 0f 29 85 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm0
140a12c77: 0f 10 85 d0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d0]
140a12c7e: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a12c85: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
140a12c8c: 48 8b 8d f0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f0]
140a12c93: 48 8b 95 80 01 00 00        	mov	rdx, qword ptr [rbp + 0x180]
140a12c9a: 48 89 55 f0                 	mov	qword ptr [rbp - 0x10], rdx
140a12c9e: 0f 10 85 70 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x170]
140a12ca5: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
140a12ca9: 0f 10 85 88 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x188]
140a12cb0: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
140a12cb4: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
140a12cbb: 48 89 55 20                 	mov	qword ptr [rbp + 0x20], rdx
140a12cbf: 0f 10 85 a0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1a0]
140a12cc6: 0f 11 45 f8                 	movups	xmmword ptr [rbp - 0x8], xmm0
140a12cca: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
140a12cd1: 48 89 55 08                 	mov	qword ptr [rbp + 0x8], rdx
140a12cd5: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
140a12cdc: 0f 10 8d 30 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x230]
140a12ce3: 0f 10 95 40 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x240]
140a12cea: 0f 10 9d 50 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x250]
140a12cf1: 0f 11 45 58                 	movups	xmmword ptr [rbp + 0x58], xmm0
140a12cf5: 0f 11 4d 68                 	movups	xmmword ptr [rbp + 0x68], xmm1
140a12cf9: 0f 11 55 78                 	movups	xmmword ptr [rbp + 0x78], xmm2
140a12cfd: 0f 11 9d 88 00 00 00        	movups	xmmword ptr [rbp + 0x88], xmm3
140a12d04: 0f 10 85 60 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x260]
140a12d0b: 0f 11 85 98 00 00 00        	movups	xmmword ptr [rbp + 0x98], xmm0
140a12d12: 0f 10 85 70 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x270]
140a12d19: 0f 11 85 a8 00 00 00        	movups	xmmword ptr [rbp + 0xa8], xmm0
140a12d20: 48 8b 95 c8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1c8]
140a12d27: 48 89 55 38                 	mov	qword ptr [rbp + 0x38], rdx
140a12d2b: 0f 10 85 b8 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1b8]
140a12d32: 0f 11 45 28                 	movups	xmmword ptr [rbp + 0x28], xmm0
140a12d36: 48 89 8d c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rcx
140a12d3d: 48 89 85 b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rax
140a12d44: 0f 29 75 40                 	movaps	xmmword ptr [rbp + 0x40], xmm6
140a12d48: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a12d4f: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a12d53: e8 78 a7 fc ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a12d58: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a12d5f: 74 0c                       	je	0x140a12d6d <.text+0xa11d6d>
140a12d61: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a12d68: e8 43 f7 1c 00              	call	0x140be24b0 <.text+0xbe14b0>
140a12d6d: 48 89 7c 24 30              	mov	qword ptr [rsp + 0x30], rdi
140a12d72: 48 89 74 24 28              	mov	qword ptr [rsp + 0x28], rsi
140a12d77: 48 c7 44 24 20 18 00 00 00  	mov	qword ptr [rsp + 0x20], 0x18
140a12d80: 48 8d 15 50 4f 0b 02        	lea	rdx, [rip + 0x20b4f50]  # 0x142ac7cd7
140a12d87: 4c 8d 0d 58 4f 0b 02        	lea	r9, [rip + 0x20b4f58]   # 0x142ac7ce6
140a12d8e: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a12d92: 41 b8 0f 00 00 00           	mov	r8d, 0xf
140a12d98: e8 03 05 bf 00              	call	0x1416032a0 <.text+0x16022a0>
140a12d9d: 80 bd 16 01 00 00 08        	cmp	byte ptr [rbp + 0x116], 0x8
140a12da4: 74 25                       	je	0x140a12dcb <.text+0xa11dcb>
140a12da6: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a12dad: ba e8 00 00 00              	mov	edx, 0xe8
140a12db2: 48 03 95 18 01 00 00        	add	rdx, qword ptr [rbp + 0x118]
140a12db9: 49 b8 22 bc 94 20 37 f8 1f d9       	movabs	r8, -0x26e007c8df6b43de
140a12dc3: 41 b1 01                    	mov	r9b, 0x1
140a12dc6: e8 85 8a f5 ff              	call	0x14096b850 <.text+0x96a850>
140a12dcb: 48 8d 9d 70 01 00 00        	lea	rbx, [rbp + 0x170]
140a12dd2: 4c 8d 75 e0                 	lea	r14, [rbp - 0x20]
140a12dd6: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a12ddc: 48 89 d9                    	mov	rcx, rbx
140a12ddf: 4c 89 f2                    	mov	rdx, r14
140a12de2: e8 19 0e f1 01              	call	0x142923c00 <.text+0x2922c00>
140a12de7: c6 85 00 03 00 00 13        	mov	byte ptr [rbp + 0x300], 0x13
140a12dee: c7 45 e0 02 00 00 00        	mov	dword ptr [rbp - 0x20], 0x2
140a12df5: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
140a12dfa: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a12e01: 49 b8 41 f0 c8 18 cd 3c d6 6a       	movabs	r8, 0x6ad63ccd18c8f041
140a12e0b: 48 89 da                    	mov	rdx, rbx
140a12e0e: 41 b1 19                    	mov	r9b, 0x19
140a12e11: e8 4a 24 ef ff              	call	0x140905260 <.text+0x904260>
140a12e16: 48 8d 5d e0                 	lea	rbx, [rbp - 0x20]
140a12e1a: 4c 8d b5 70 01 00 00        	lea	r14, [rbp + 0x170]
140a12e21: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a12e27: 48 89 d9                    	mov	rcx, rbx
140a12e2a: 4c 89 f2                    	mov	rdx, r14
140a12e2d: e8 ce 0d f1 01              	call	0x142923c00 <.text+0x2922c00>
140a12e32: 4c 89 bd 00 03 00 00        	mov	qword ptr [rbp + 0x300], r15
140a12e39: c7 85 70 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x170], 0x2
140a12e43: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
140a12e48: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a12e4f: 49 b8 68 07 7d b5 a7 c3 f7 e8       	movabs	r8, -0x17083c584a82f898
140a12e59: 48 89 da                    	mov	rdx, rbx
140a12e5c: 41 b1 32                    	mov	r9b, 0x32
140a12e5f: e8 9c ae ed ff              	call	0x1408edd00 <.text+0x8ecd00>
140a12e64: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a12e6b: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a12e6f: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a12e75: e8 86 0d f1 01              	call	0x142923c00 <.text+0x2922c00>
140a12e7a: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a12e81: 74 2a                       	je	0x140a12ead <.text+0xa11ead>
140a12e83: 48 8d 8d 80 02 00 00        	lea	rcx, [rbp + 0x280]
140a12e8a: ba e8 00 00 00              	mov	edx, 0xe8
140a12e8f: 48 03 95 a8 02 00 00        	add	rdx, qword ptr [rbp + 0x2a8]
140a12e96: 49 b8 94 a7 92 30 4f 2c 02 dc       	movabs	r8, -0x23fdd3b0cf6d586c
140a12ea0: f3 0f 10 1d 8c a1 fb 01     	movss	xmm3, dword ptr [rip + 0x1fba18c] # 0x1429cd034
140a12ea8: e8 83 8b f5 ff              	call	0x14096ba30 <.text+0x96aa30>
140a12ead: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a12eb1: 48 8d 95 70 01 00 00        	lea	rdx, [rbp + 0x170]
140a12eb8: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a12ebe: e8 3d 0d f1 01              	call	0x142923c00 <.text+0x2922c00>
140a12ec3: 80 bd 16 01 00 00 08        	cmp	byte ptr [rbp + 0x116], 0x8
140a12eca: 74 28                       	je	0x140a12ef4 <.text+0xa11ef4>
140a12ecc: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a12ed3: ba e8 00 00 00              	mov	edx, 0xe8
140a12ed8: 48 03 95 18 01 00 00        	add	rdx, qword ptr [rbp + 0x118]
140a12edf: 49 b8 28 ae 4f 9d 5b 6c d7 4a       	movabs	r8, 0x4ad76c5b9d4fae28
140a12ee9: 41 b9 02 00 00 00           	mov	r9d, 0x2
140a12eef: e8 7c 86 f5 ff              	call	0x14096b570 <.text+0x96a570>
140a12ef4: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a12efb: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a12eff: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a12f05: e8 f6 0c f1 01              	call	0x142923c00 <.text+0x2922c00>
140a12f0a: 0f 10 85 10 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x210]
140a12f11: 0f 29 85 40 03 00 00        	movaps	xmmword ptr [rbp + 0x340], xmm0
140a12f18: 0f 10 85 00 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x200]
140a12f1f: 0f 29 85 30 03 00 00        	movaps	xmmword ptr [rbp + 0x330], xmm0
140a12f26: 0f 10 85 f0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1f0]
140a12f2d: 0f 29 85 20 03 00 00        	movaps	xmmword ptr [rbp + 0x320], xmm0
140a12f34: 0f 10 85 e0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1e0]
140a12f3b: 0f 29 85 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm0
140a12f42: 0f 10 85 d0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d0]
140a12f49: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a12f50: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
140a12f57: 48 8b 8d f0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f0]
140a12f5e: 48 8b 95 80 01 00 00        	mov	rdx, qword ptr [rbp + 0x180]
140a12f65: 48 89 55 f0                 	mov	qword ptr [rbp - 0x10], rdx
140a12f69: 0f 10 85 70 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x170]
140a12f70: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
140a12f74: 0f 10 85 88 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x188]
140a12f7b: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
140a12f7f: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
140a12f86: 48 89 55 20                 	mov	qword ptr [rbp + 0x20], rdx
140a12f8a: 0f 10 85 a0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1a0]
140a12f91: 0f 11 45 f8                 	movups	xmmword ptr [rbp - 0x8], xmm0
140a12f95: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
140a12f9c: 48 89 55 08                 	mov	qword ptr [rbp + 0x8], rdx
140a12fa0: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
140a12fa7: 0f 10 8d 30 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x230]
140a12fae: 0f 10 95 40 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x240]
140a12fb5: 0f 10 9d 50 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x250]
140a12fbc: 0f 11 45 58                 	movups	xmmword ptr [rbp + 0x58], xmm0
140a12fc0: 0f 11 4d 68                 	movups	xmmword ptr [rbp + 0x68], xmm1
140a12fc4: 0f 11 55 78                 	movups	xmmword ptr [rbp + 0x78], xmm2
140a12fc8: 0f 11 9d 88 00 00 00        	movups	xmmword ptr [rbp + 0x88], xmm3
140a12fcf: 0f 10 85 60 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x260]
140a12fd6: 0f 11 85 98 00 00 00        	movups	xmmword ptr [rbp + 0x98], xmm0
140a12fdd: 0f 10 85 70 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x270]
140a12fe4: 0f 11 85 a8 00 00 00        	movups	xmmword ptr [rbp + 0xa8], xmm0
140a12feb: 48 8b 95 c8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1c8]
140a12ff2: 48 89 55 38                 	mov	qword ptr [rbp + 0x38], rdx
140a12ff6: 0f 10 85 b8 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1b8]
140a12ffd: 0f 11 45 28                 	movups	xmmword ptr [rbp + 0x28], xmm0
140a13001: 48 89 8d c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rcx
140a13008: 48 89 85 b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rax
140a1300f: 0f 29 75 40                 	movaps	xmmword ptr [rbp + 0x40], xmm6
140a13013: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a1301a: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a1301e: e8 ad a4 fc ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a13023: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a1302a: 74 0c                       	je	0x140a13038 <.text+0xa12038>
140a1302c: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a13033: e8 78 f4 1c 00              	call	0x140be24b0 <.text+0xbe14b0>
140a13038: 48 89 7c 24 30              	mov	qword ptr [rsp + 0x30], rdi
140a1303d: 48 89 74 24 28              	mov	qword ptr [rsp + 0x28], rsi
140a13042: 48 c7 44 24 20 18 00 00 00  	mov	qword ptr [rsp + 0x20], 0x18
140a1304b: 48 8d 15 85 4c 0b 02        	lea	rdx, [rip + 0x20b4c85]  # 0x142ac7cd7
140a13052: 4c 8d 0d 8d 4c 0b 02        	lea	r9, [rip + 0x20b4c8d]   # 0x142ac7ce6
140a13059: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a1305d: 41 b8 0f 00 00 00           	mov	r8d, 0xf
140a13063: e8 38 02 bf 00              	call	0x1416032a0 <.text+0x16022a0>
140a13068: 80 bd 16 01 00 00 08        	cmp	byte ptr [rbp + 0x116], 0x8
140a1306f: 74 25                       	je	0x140a13096 <.text+0xa12096>
140a13071: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a13078: ba e8 00 00 00              	mov	edx, 0xe8
140a1307d: 48 03 95 18 01 00 00        	add	rdx, qword ptr [rbp + 0x118]
140a13084: 49 b8 22 bc 94 20 37 f8 1f d9       	movabs	r8, -0x26e007c8df6b43de
140a1308e: 45 31 c9                    	xor	r9d, r9d
140a13091: e8 ba 87 f5 ff              	call	0x14096b850 <.text+0x96a850>
140a13096: 48 8d b5 70 01 00 00        	lea	rsi, [rbp + 0x170]
140a1309d: 48 8d 7d e0                 	lea	rdi, [rbp - 0x20]
140a130a1: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a130a7: 48 89 f1                    	mov	rcx, rsi
140a130aa: 48 89 fa                    	mov	rdx, rdi
140a130ad: e8 4e 0b f1 01              	call	0x142923c00 <.text+0x2922c00>
140a130b2: 4c 89 bd 00 03 00 00        	mov	qword ptr [rbp + 0x300], r15
140a130b9: c7 45 e0 02 00 00 00        	mov	dword ptr [rbp - 0x20], 0x2
140a130c0: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a130c5: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a130cc: 49 b8 41 f0 c8 18 cd 3c d6 6a       	movabs	r8, 0x6ad63ccd18c8f041
140a130d6: 48 89 f2                    	mov	rdx, rsi
140a130d9: 41 b1 19                    	mov	r9b, 0x19
140a130dc: e8 5f b1 ed ff              	call	0x1408ee240 <.text+0x8ed240>
140a130e1: 48 8d 75 e0                 	lea	rsi, [rbp - 0x20]
140a130e5: 48 8d bd 70 01 00 00        	lea	rdi, [rbp + 0x170]
140a130ec: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a130f2: 48 89 f1                    	mov	rcx, rsi
140a130f5: 48 89 fa                    	mov	rdx, rdi
140a130f8: e8 03 0b f1 01              	call	0x142923c00 <.text+0x2922c00>
140a130fd: c6 85 00 03 00 00 13        	mov	byte ptr [rbp + 0x300], 0x13
140a13104: c7 85 70 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x170], 0x2
140a1310e: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a13113: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a1311a: 49 b8 68 07 7d b5 a7 c3 f7 e8       	movabs	r8, -0x17083c584a82f898
140a13124: 48 89 f2                    	mov	rdx, rsi
140a13127: 41 b1 32                    	mov	r9b, 0x32
140a1312a: e8 31 21 ef ff              	call	0x140905260 <.text+0x904260>
140a1312f: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a13136: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a1313a: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a13140: e8 bb 0a f1 01              	call	0x142923c00 <.text+0x2922c00>
140a13145: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a1314c: 74 2a                       	je	0x140a13178 <.text+0xa12178>
140a1314e: 48 8d 8d 80 02 00 00        	lea	rcx, [rbp + 0x280]
140a13155: ba e8 00 00 00              	mov	edx, 0xe8
140a1315a: 48 03 95 a8 02 00 00        	add	rdx, qword ptr [rbp + 0x2a8]
140a13161: 49 b8 94 a7 92 30 4f 2c 02 dc       	movabs	r8, -0x23fdd3b0cf6d586c
140a1316b: f3 0f 10 1d c1 9e fb 01     	movss	xmm3, dword ptr [rip + 0x1fb9ec1] # 0x1429cd034
140a13173: e8 b8 88 f5 ff              	call	0x14096ba30 <.text+0x96aa30>
140a13178: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a1317c: 48 8d 95 70 01 00 00        	lea	rdx, [rbp + 0x170]
140a13183: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a13189: e8 72 0a f1 01              	call	0x142923c00 <.text+0x2922c00>
140a1318e: 80 bd 16 01 00 00 08        	cmp	byte ptr [rbp + 0x116], 0x8
140a13195: 74 28                       	je	0x140a131bf <.text+0xa121bf>
140a13197: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a1319e: ba e8 00 00 00              	mov	edx, 0xe8
140a131a3: 48 03 95 18 01 00 00        	add	rdx, qword ptr [rbp + 0x118]
140a131aa: 49 b8 28 ae 4f 9d 5b 6c d7 4a       	movabs	r8, 0x4ad76c5b9d4fae28
140a131b4: 41 b9 02 00 00 00           	mov	r9d, 0x2
140a131ba: e8 b1 83 f5 ff              	call	0x14096b570 <.text+0x96a570>
140a131bf: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a131c6: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a131ca: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a131d0: e8 2b 0a f1 01              	call	0x142923c00 <.text+0x2922c00>
140a131d5: 0f 10 85 10 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x210]
140a131dc: 0f 29 85 40 03 00 00        	movaps	xmmword ptr [rbp + 0x340], xmm0
140a131e3: 0f 10 85 00 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x200]
140a131ea: 0f 29 85 30 03 00 00        	movaps	xmmword ptr [rbp + 0x330], xmm0
140a131f1: 0f 10 85 f0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1f0]
140a131f8: 0f 29 85 20 03 00 00        	movaps	xmmword ptr [rbp + 0x320], xmm0
140a131ff: 0f 10 85 e0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1e0]
140a13206: 0f 29 85 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm0
140a1320d: 0f 10 85 d0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d0]
140a13214: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a1321b: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
140a13222: 48 8b 8d f0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f0]
140a13229: 48 8b 95 80 01 00 00        	mov	rdx, qword ptr [rbp + 0x180]
140a13230: 48 89 55 f0                 	mov	qword ptr [rbp - 0x10], rdx
140a13234: 0f 10 85 70 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x170]
140a1323b: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
140a1323f: 0f 10 85 88 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x188]
140a13246: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
140a1324a: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
140a13251: 48 89 55 20                 	mov	qword ptr [rbp + 0x20], rdx
140a13255: 0f 10 85 a0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1a0]
140a1325c: 0f 11 45 f8                 	movups	xmmword ptr [rbp - 0x8], xmm0
140a13260: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
140a13267: 48 89 55 08                 	mov	qword ptr [rbp + 0x8], rdx
140a1326b: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
140a13272: 0f 10 8d 30 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x230]
140a13279: 0f 10 95 40 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x240]
140a13280: 0f 10 9d 50 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x250]
140a13287: 0f 11 45 58                 	movups	xmmword ptr [rbp + 0x58], xmm0
140a1328b: 0f 11 4d 68                 	movups	xmmword ptr [rbp + 0x68], xmm1
140a1328f: 0f 11 55 78                 	movups	xmmword ptr [rbp + 0x78], xmm2
140a13293: 0f 11 9d 88 00 00 00        	movups	xmmword ptr [rbp + 0x88], xmm3
140a1329a: 0f 10 85 60 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x260]
140a132a1: 0f 11 85 98 00 00 00        	movups	xmmword ptr [rbp + 0x98], xmm0
140a132a8: 0f 10 85 70 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x270]
140a132af: 0f 11 85 a8 00 00 00        	movups	xmmword ptr [rbp + 0xa8], xmm0
140a132b6: 48 8b 95 c8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1c8]
140a132bd: 48 89 55 38                 	mov	qword ptr [rbp + 0x38], rdx
140a132c1: 0f 10 85 b8 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1b8]
140a132c8: 0f 11 45 28                 	movups	xmmword ptr [rbp + 0x28], xmm0
140a132cc: 48 89 8d c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rcx
140a132d3: 48 89 85 b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rax
140a132da: 0f 29 75 40                 	movaps	xmmword ptr [rbp + 0x40], xmm6
140a132de: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a132e5: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
140a132e9: e8 e2 a1 fc ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a132ee: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a132f5: 74 0c                       	je	0x140a13303 <.text+0xa12303>
140a132f7: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a132fe: e8 ad f1 1c 00              	call	0x140be24b0 <.text+0xbe14b0>
140a13303: 0f 28 b5 60 03 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x360]
140a1330a: 48 81 c4 f8 03 00 00        	add	rsp, 0x3f8
140a13311: 5b                          	pop	rbx
140a13312: 5f                          	pop	rdi
140a13313: 5e                          	pop	rsi
140a13314: 41 5e                       	pop	r14
140a13316: 41 5f                       	pop	r15
140a13318: 5d                          	pop	rbp
140a13319: c3                          	ret
140a1331a: 48 8d 0d 8f 49 0b 02        	lea	rcx, [rip + 0x20b498f]  # 0x142ac7cb0
140a13321: e8 ea 5f ec 01              	call	0x1428d9310 <.text+0x28d8310>
140a13326: cc                          	int3
140a13327: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140a13330: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13335: 55                          	push	rbp
140a13336: 41 57                       	push	r15
140a13338: 41 56                       	push	r14
140a1333a: 56                          	push	rsi
140a1333b: 57                          	push	rdi
140a1333c: 53                          	push	rbx
140a1333d: 48 83 ec 48                 	sub	rsp, 0x48
140a13341: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13348: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1334d: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a13354: 74 0c                       	je	0x140a13362 <.text+0xa12362>
140a13356: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a1335d: e8 4e f1 1c 00              	call	0x140be24b0 <.text+0xbe14b0>
140a13362: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a13367: 48 83 c4 48                 	add	rsp, 0x48
140a1336b: 5b                          	pop	rbx
140a1336c: 5f                          	pop	rdi
140a1336d: 5e                          	pop	rsi
140a1336e: 41 5e                       	pop	r14
140a13370: 41 5f                       	pop	r15
140a13372: 5d                          	pop	rbp
140a13373: c3                          	ret
140a13374: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
140a13380: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13385: 55                          	push	rbp
140a13386: 41 57                       	push	r15
140a13388: 41 56                       	push	r14
140a1338a: 56                          	push	rsi
140a1338b: 57                          	push	rdi
140a1338c: 53                          	push	rbx
140a1338d: 48 83 ec 48                 	sub	rsp, 0x48
140a13391: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13398: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1339d: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a133a1: e8 0a 3c e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a133a6: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a133ab: 48 83 c4 48                 	add	rsp, 0x48
140a133af: 5b                          	pop	rbx
140a133b0: 5f                          	pop	rdi
140a133b1: 5e                          	pop	rsi
140a133b2: 41 5e                       	pop	r14
140a133b4: 41 5f                       	pop	r15
140a133b6: 5d                          	pop	rbp
140a133b7: c3                          	ret
140a133b8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a133c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a133c5: 55                          	push	rbp
140a133c6: 41 57                       	push	r15
140a133c8: 41 56                       	push	r14
140a133ca: 56                          	push	rsi
140a133cb: 57                          	push	rdi
140a133cc: 53                          	push	rbx
140a133cd: 48 83 ec 48                 	sub	rsp, 0x48
140a133d1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a133d8: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a133dd: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a133e4: e8 c7 3b e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a133e9: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a133ee: 48 83 c4 48                 	add	rsp, 0x48
140a133f2: 5b                          	pop	rbx
140a133f3: 5f                          	pop	rdi
140a133f4: 5e                          	pop	rsi
140a133f5: 41 5e                       	pop	r14
140a133f7: 41 5f                       	pop	r15
140a133f9: 5d                          	pop	rbp
140a133fa: c3                          	ret
140a133fb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140a13400: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13405: 55                          	push	rbp
140a13406: 41 57                       	push	r15
140a13408: 41 56                       	push	r14
140a1340a: 56                          	push	rsi
140a1340b: 57                          	push	rdi
140a1340c: 53                          	push	rbx
140a1340d: 48 83 ec 48                 	sub	rsp, 0x48
140a13411: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13418: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1341d: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a13421: e8 8a 3b e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a13426: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a1342b: 48 83 c4 48                 	add	rsp, 0x48
140a1342f: 5b                          	pop	rbx
140a13430: 5f                          	pop	rdi
140a13431: 5e                          	pop	rsi
140a13432: 41 5e                       	pop	r14
140a13434: 41 5f                       	pop	r15
140a13436: 5d                          	pop	rbp
140a13437: c3                          	ret
140a13438: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a13440: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13445: 55                          	push	rbp
140a13446: 41 57                       	push	r15
140a13448: 41 56                       	push	r14
140a1344a: 56                          	push	rsi
140a1344b: 57                          	push	rdi
140a1344c: 53                          	push	rbx
140a1344d: 48 83 ec 48                 	sub	rsp, 0x48
140a13451: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13458: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1345d: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a13464: e8 47 3b e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a13469: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a1346e: 48 83 c4 48                 	add	rsp, 0x48
140a13472: 5b                          	pop	rbx
140a13473: 5f                          	pop	rdi
140a13474: 5e                          	pop	rsi
140a13475: 41 5e                       	pop	r14
140a13477: 41 5f                       	pop	r15
140a13479: 5d                          	pop	rbp
140a1347a: c3                          	ret
140a1347b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140a13480: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13485: 55                          	push	rbp
140a13486: 41 57                       	push	r15
140a13488: 41 56                       	push	r14
140a1348a: 56                          	push	rsi
140a1348b: 57                          	push	rdi
140a1348c: 53                          	push	rbx
140a1348d: 48 83 ec 48                 	sub	rsp, 0x48
140a13491: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13498: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1349d: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a134a1: e8 0a 3b e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a134a6: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a134ab: 48 83 c4 48                 	add	rsp, 0x48
140a134af: 5b                          	pop	rbx
140a134b0: 5f                          	pop	rdi
140a134b1: 5e                          	pop	rsi
140a134b2: 41 5e                       	pop	r14
140a134b4: 41 5f                       	pop	r15
140a134b6: 5d                          	pop	rbp
140a134b7: c3                          	ret
140a134b8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a134c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a134c5: 55                          	push	rbp
140a134c6: 41 57                       	push	r15
140a134c8: 41 56                       	push	r14
140a134ca: 56                          	push	rsi
140a134cb: 57                          	push	rdi
140a134cc: 53                          	push	rbx
140a134cd: 48 83 ec 48                 	sub	rsp, 0x48
140a134d1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a134d8: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a134dd: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a134e4: 74 0c                       	je	0x140a134f2 <.text+0xa124f2>
140a134e6: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a134ed: e8 be ef 1c 00              	call	0x140be24b0 <.text+0xbe14b0>
140a134f2: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a134f7: 48 83 c4 48                 	add	rsp, 0x48
140a134fb: 5b                          	pop	rbx
140a134fc: 5f                          	pop	rdi
140a134fd: 5e                          	pop	rsi
140a134fe: 41 5e                       	pop	r14
140a13500: 41 5f                       	pop	r15
140a13502: 5d                          	pop	rbp
140a13503: c3                          	ret
140a13504: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
140a13510: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13515: 55                          	push	rbp
140a13516: 41 57                       	push	r15
140a13518: 41 56                       	push	r14
140a1351a: 56                          	push	rsi
140a1351b: 57                          	push	rdi
140a1351c: 53                          	push	rbx
140a1351d: 48 83 ec 48                 	sub	rsp, 0x48
140a13521: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13528: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1352d: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a13531: e8 7a 3a e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a13536: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a1353b: 48 83 c4 48                 	add	rsp, 0x48
140a1353f: 5b                          	pop	rbx
140a13540: 5f                          	pop	rdi
140a13541: 5e                          	pop	rsi
140a13542: 41 5e                       	pop	r14
140a13544: 41 5f                       	pop	r15
140a13546: 5d                          	pop	rbp
140a13547: c3                          	ret
140a13548: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a13550: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13555: 55                          	push	rbp
140a13556: 41 57                       	push	r15
140a13558: 41 56                       	push	r14
140a1355a: 56                          	push	rsi
140a1355b: 57                          	push	rdi
140a1355c: 53                          	push	rbx
140a1355d: 48 83 ec 48                 	sub	rsp, 0x48
140a13561: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13568: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1356d: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a13574: e8 37 3a e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a13579: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a1357e: 48 83 c4 48                 	add	rsp, 0x48
140a13582: 5b                          	pop	rbx
140a13583: 5f                          	pop	rdi
140a13584: 5e                          	pop	rsi
140a13585: 41 5e                       	pop	r14
140a13587: 41 5f                       	pop	r15
140a13589: 5d                          	pop	rbp
140a1358a: c3                          	ret
140a1358b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140a13590: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13595: 55                          	push	rbp
140a13596: 41 57                       	push	r15
140a13598: 41 56                       	push	r14
140a1359a: 56                          	push	rsi
140a1359b: 57                          	push	rdi
140a1359c: 53                          	push	rbx
140a1359d: 48 83 ec 48                 	sub	rsp, 0x48
140a135a1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a135a8: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a135ad: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a135b1: e8 fa 39 e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a135b6: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a135bb: 48 83 c4 48                 	add	rsp, 0x48
140a135bf: 5b                          	pop	rbx
140a135c0: 5f                          	pop	rdi
140a135c1: 5e                          	pop	rsi
140a135c2: 41 5e                       	pop	r14
140a135c4: 41 5f                       	pop	r15
140a135c6: 5d                          	pop	rbp
140a135c7: c3                          	ret
140a135c8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a135d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a135d5: 55                          	push	rbp
140a135d6: 41 57                       	push	r15
140a135d8: 41 56                       	push	r14
140a135da: 56                          	push	rsi
140a135db: 57                          	push	rdi
140a135dc: 53                          	push	rbx
140a135dd: 48 83 ec 48                 	sub	rsp, 0x48
140a135e1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a135e8: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a135ed: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a135f4: e8 b7 39 e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a135f9: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a135fe: 48 83 c4 48                 	add	rsp, 0x48
140a13602: 5b                          	pop	rbx
140a13603: 5f                          	pop	rdi
140a13604: 5e                          	pop	rsi
140a13605: 41 5e                       	pop	r14
140a13607: 41 5f                       	pop	r15
140a13609: 5d                          	pop	rbp
140a1360a: c3                          	ret
140a1360b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140a13610: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13615: 55                          	push	rbp
140a13616: 41 57                       	push	r15
140a13618: 41 56                       	push	r14
140a1361a: 56                          	push	rsi
140a1361b: 57                          	push	rdi
140a1361c: 53                          	push	rbx
140a1361d: 48 83 ec 48                 	sub	rsp, 0x48
140a13621: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13628: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1362d: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a13631: e8 7a 39 e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a13636: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a1363b: 48 83 c4 48                 	add	rsp, 0x48
140a1363f: 5b                          	pop	rbx
140a13640: 5f                          	pop	rdi
140a13641: 5e                          	pop	rsi
140a13642: 41 5e                       	pop	r14
140a13644: 41 5f                       	pop	r15
140a13646: 5d                          	pop	rbp
140a13647: c3                          	ret
140a13648: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a13650: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13655: 55                          	push	rbp
140a13656: 41 57                       	push	r15
140a13658: 41 56                       	push	r14
140a1365a: 56                          	push	rsi
140a1365b: 57                          	push	rdi
140a1365c: 53                          	push	rbx
140a1365d: 48 83 ec 48                 	sub	rsp, 0x48
140a13661: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13668: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1366d: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a13674: 74 0c                       	je	0x140a13682 <.text+0xa12682>
140a13676: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a1367d: e8 2e ee 1c 00              	call	0x140be24b0 <.text+0xbe14b0>
140a13682: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a13687: 48 83 c4 48                 	add	rsp, 0x48
140a1368b: 5b                          	pop	rbx
140a1368c: 5f                          	pop	rdi
140a1368d: 5e                          	pop	rsi
140a1368e: 41 5e                       	pop	r14
140a13690: 41 5f                       	pop	r15
140a13692: 5d                          	pop	rbp
140a13693: c3                          	ret
140a13694: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
140a136a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a136a5: 55                          	push	rbp
140a136a6: 41 57                       	push	r15
140a136a8: 41 56                       	push	r14
140a136aa: 56                          	push	rsi
140a136ab: 57                          	push	rdi
140a136ac: 53                          	push	rbx
140a136ad: 48 83 ec 48                 	sub	rsp, 0x48
140a136b1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a136b8: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a136bd: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a136c1: e8 ea 38 e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a136c6: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a136cb: 48 83 c4 48                 	add	rsp, 0x48
140a136cf: 5b                          	pop	rbx
140a136d0: 5f                          	pop	rdi
140a136d1: 5e                          	pop	rsi
140a136d2: 41 5e                       	pop	r14
140a136d4: 41 5f                       	pop	r15
140a136d6: 5d                          	pop	rbp
140a136d7: c3                          	ret
140a136d8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a136e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a136e5: 55                          	push	rbp
140a136e6: 41 57                       	push	r15
140a136e8: 41 56                       	push	r14
140a136ea: 56                          	push	rsi
140a136eb: 57                          	push	rdi
140a136ec: 53                          	push	rbx
140a136ed: 48 83 ec 48                 	sub	rsp, 0x48
140a136f1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a136f8: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a136fd: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a13704: e8 a7 38 e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a13709: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a1370e: 48 83 c4 48                 	add	rsp, 0x48
140a13712: 5b                          	pop	rbx
140a13713: 5f                          	pop	rdi
140a13714: 5e                          	pop	rsi
140a13715: 41 5e                       	pop	r14
140a13717: 41 5f                       	pop	r15
140a13719: 5d                          	pop	rbp
140a1371a: c3                          	ret
140a1371b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140a13720: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13725: 55                          	push	rbp
140a13726: 41 57                       	push	r15
140a13728: 41 56                       	push	r14
140a1372a: 56                          	push	rsi
140a1372b: 57                          	push	rdi
140a1372c: 53                          	push	rbx
140a1372d: 48 83 ec 48                 	sub	rsp, 0x48
140a13731: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13738: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1373d: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a13741: e8 6a 38 e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a13746: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a1374b: 48 83 c4 48                 	add	rsp, 0x48
140a1374f: 5b                          	pop	rbx
140a13750: 5f                          	pop	rdi
140a13751: 5e                          	pop	rsi
140a13752: 41 5e                       	pop	r14
140a13754: 41 5f                       	pop	r15
140a13756: 5d                          	pop	rbp
140a13757: c3                          	ret
140a13758: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a13760: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13765: 55                          	push	rbp
140a13766: 41 57                       	push	r15
140a13768: 41 56                       	push	r14
140a1376a: 56                          	push	rsi
140a1376b: 57                          	push	rdi
140a1376c: 53                          	push	rbx
140a1376d: 48 83 ec 48                 	sub	rsp, 0x48
140a13771: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13778: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1377d: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a13784: e8 27 38 e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a13789: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a1378e: 48 83 c4 48                 	add	rsp, 0x48
140a13792: 5b                          	pop	rbx
140a13793: 5f                          	pop	rdi
140a13794: 5e                          	pop	rsi
140a13795: 41 5e                       	pop	r14
140a13797: 41 5f                       	pop	r15
140a13799: 5d                          	pop	rbp
140a1379a: c3                          	ret
140a1379b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140a137a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a137a5: 55                          	push	rbp
140a137a6: 41 57                       	push	r15
140a137a8: 41 56                       	push	r14
140a137aa: 56                          	push	rsi
140a137ab: 57                          	push	rdi
140a137ac: 53                          	push	rbx
140a137ad: 48 83 ec 48                 	sub	rsp, 0x48
140a137b1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a137b8: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a137bd: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a137c1: e8 ea 37 e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a137c6: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a137cb: 48 83 c4 48                 	add	rsp, 0x48
140a137cf: 5b                          	pop	rbx
140a137d0: 5f                          	pop	rdi
140a137d1: 5e                          	pop	rsi
140a137d2: 41 5e                       	pop	r14
140a137d4: 41 5f                       	pop	r15
140a137d6: 5d                          	pop	rbp
140a137d7: c3                          	ret
140a137d8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a137e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a137e5: 55                          	push	rbp
140a137e6: 41 57                       	push	r15
140a137e8: 41 56                       	push	r14
140a137ea: 56                          	push	rsi
140a137eb: 57                          	push	rdi
140a137ec: 53                          	push	rbx
140a137ed: 48 83 ec 48                 	sub	rsp, 0x48
140a137f1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a137f8: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a137fd: 80 bd a6 02 00 00 08        	cmp	byte ptr [rbp + 0x2a6], 0x8
140a13804: 74 0c                       	je	0x140a13812 <.text+0xa12812>
140a13806: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a1380d: e8 9e ec 1c 00              	call	0x140be24b0 <.text+0xbe14b0>
140a13812: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a13817: 48 83 c4 48                 	add	rsp, 0x48
140a1381b: 5b                          	pop	rbx
140a1381c: 5f                          	pop	rdi
140a1381d: 5e                          	pop	rsi
140a1381e: 41 5e                       	pop	r14
140a13820: 41 5f                       	pop	r15
140a13822: 5d                          	pop	rbp
140a13823: c3                          	ret
140a13824: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
140a13830: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13835: 55                          	push	rbp
140a13836: 41 57                       	push	r15
140a13838: 41 56                       	push	r14
140a1383a: 56                          	push	rsi
140a1383b: 57                          	push	rdi
140a1383c: 53                          	push	rbx
140a1383d: 48 83 ec 48                 	sub	rsp, 0x48
140a13841: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13848: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1384d: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a13851: e8 5a 37 e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a13856: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a1385b: 48 83 c4 48                 	add	rsp, 0x48
140a1385f: 5b                          	pop	rbx
140a13860: 5f                          	pop	rdi
140a13861: 5e                          	pop	rsi
140a13862: 41 5e                       	pop	r14
140a13864: 41 5f                       	pop	r15
140a13866: 5d                          	pop	rbp
140a13867: c3                          	ret
140a13868: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a13870: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13875: 55                          	push	rbp
140a13876: 41 57                       	push	r15
140a13878: 41 56                       	push	r14
140a1387a: 56                          	push	rsi
140a1387b: 57                          	push	rdi
140a1387c: 53                          	push	rbx
140a1387d: 48 83 ec 48                 	sub	rsp, 0x48
140a13881: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13888: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1388d: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a13894: e8 17 37 e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a13899: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a1389e: 48 83 c4 48                 	add	rsp, 0x48
140a138a2: 5b                          	pop	rbx
140a138a3: 5f                          	pop	rdi
140a138a4: 5e                          	pop	rsi
140a138a5: 41 5e                       	pop	r14
140a138a7: 41 5f                       	pop	r15
140a138a9: 5d                          	pop	rbp
140a138aa: c3                          	ret
140a138ab: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140a138b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a138b5: 55                          	push	rbp
140a138b6: 41 57                       	push	r15
140a138b8: 41 56                       	push	r14
140a138ba: 56                          	push	rsi
140a138bb: 57                          	push	rdi
140a138bc: 53                          	push	rbx
140a138bd: 48 83 ec 48                 	sub	rsp, 0x48
140a138c1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a138c8: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a138cd: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a138d1: e8 da 36 e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a138d6: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a138db: 48 83 c4 48                 	add	rsp, 0x48
140a138df: 5b                          	pop	rbx
140a138e0: 5f                          	pop	rdi
140a138e1: 5e                          	pop	rsi
140a138e2: 41 5e                       	pop	r14
140a138e4: 41 5f                       	pop	r15
140a138e6: 5d                          	pop	rbp
140a138e7: c3                          	ret
140a138e8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140a138f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a138f5: 55                          	push	rbp
140a138f6: 41 57                       	push	r15
140a138f8: 41 56                       	push	r14
140a138fa: 56                          	push	rsi
140a138fb: 57                          	push	rdi
140a138fc: 53                          	push	rbx
140a138fd: 48 83 ec 48                 	sub	rsp, 0x48
140a13901: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13908: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1390d: 48 8d 8d 70 01 00 00        	lea	rcx, [rbp + 0x170]
140a13914: e8 97 36 e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a13919: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a1391e: 48 83 c4 48                 	add	rsp, 0x48
140a13922: 5b                          	pop	rbx
140a13923: 5f                          	pop	rdi
140a13924: 5e                          	pop	rsi
140a13925: 41 5e                       	pop	r14
140a13927: 41 5f                       	pop	r15
140a13929: 5d                          	pop	rbp
140a1392a: c3                          	ret
140a1392b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140a13930: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a13935: 55                          	push	rbp
140a13936: 41 57                       	push	r15
140a13938: 41 56                       	push	r14
140a1393a: 56                          	push	rsi
140a1393b: 57                          	push	rdi
140a1393c: 53                          	push	rbx
140a1393d: 48 83 ec 48                 	sub	rsp, 0x48
140a13941: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a13948: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a1394d: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a13951: e8 5a 36 e8 ff              	call	0x140896fb0 <.text+0x895fb0>
140a13956: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a1395b: 48 83 c4 48                 	add	rsp, 0x48
140a1395f: 5b                          	pop	rbx
140a13960: 5f                          	pop	rdi
140a13961: 5e                          	pop	rsi
140a13962: 41 5e                       	pop	r14
140a13964: 41 5f                       	pop	r15
140a13966: 5d                          	pop	rbp
140a13967: c3                          	ret


ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1413fb240: 55                          	push	rbp
1413fb241: 56                          	push	rsi
1413fb242: 48 81 ec 68 02 00 00        	sub	rsp, 0x268
1413fb249: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1413fb251: 48 c7 85 e0 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x1e0], -0x2
1413fb25c: 48 89 ce                    	mov	rsi, rcx
1413fb25f: 0f b6 05 a3 99 23 02        	movzx	eax, byte ptr [rip + 0x22399a3] # 0x143634c09
1413fb266: b9 30 02 00 00              	mov	ecx, 0x230
1413fb26b: ba 08 00 00 00              	mov	edx, 0x8
1413fb270: e8 9b 89 21 ff              	call	0x140613c10 <.text+0x612c10>
1413fb275: 48 89 85 d8 01 00 00        	mov	qword ptr [rbp + 0x1d8], rax
1413fb27c: 48 85 c0                    	test	rax, rax
1413fb27f: 0f 84 29 02 00 00           	je	0x1413fb4ae <.text+0x13fa4ae>
1413fb285: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
1413fb28c: e8 ef 04 00 00              	call	0x1413fb780 <.text+0x13fa780>
1413fb291: 48 8d 8d f8 00 00 00        	lea	rcx, [rbp + 0xf8]
1413fb298: e8 43 42 3a 00              	call	0x14179f4e0 <.text+0x179e4e0>
1413fb29d: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
1413fb2a4: e8 67 fe 25 00              	call	0x14165b110 <.text+0x165a110>
1413fb2a9: 48 8d 4d 18                 	lea	rcx, [rbp + 0x18]
1413fb2ad: e8 de 14 63 ff              	call	0x140a2c790 <.text+0xa2b790>
1413fb2b2: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1413fb2b6: e8 f5 32 4b ff              	call	0x1408ae5b0 <.text+0x8ad5b0>
1413fb2bb: 0f 10 85 c8 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1c8]
1413fb2c2: 48 8b 85 d8 01 00 00        	mov	rax, qword ptr [rbp + 0x1d8]
1413fb2c9: 0f 11 40 60                 	movups	xmmword ptr [rax + 0x60], xmm0
1413fb2cd: 0f 10 85 b8 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1b8]
1413fb2d4: 0f 11 40 50                 	movups	xmmword ptr [rax + 0x50], xmm0
1413fb2d8: 0f 10 85 a8 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1a8]
1413fb2df: 0f 11 40 40                 	movups	xmmword ptr [rax + 0x40], xmm0
1413fb2e3: 0f 10 85 68 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x168]
1413fb2ea: 0f 10 8d 78 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x178]
1413fb2f1: 0f 10 95 88 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x188]
1413fb2f8: 0f 10 9d 98 01 00 00        	movups	xmm3, xmmword ptr [rbp + 0x198]
1413fb2ff: 0f 11 58 30                 	movups	xmmword ptr [rax + 0x30], xmm3
1413fb303: 0f 11 50 20                 	movups	xmmword ptr [rax + 0x20], xmm2
1413fb307: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
1413fb30b: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
1413fb30e: 0f 10 85 f8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xf8]
1413fb315: 0f 10 8d 08 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x108]
1413fb31c: 0f 10 95 18 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x118]
1413fb323: 0f 10 9d 28 01 00 00        	movups	xmm3, xmmword ptr [rbp + 0x128]
1413fb32a: 0f 11 40 70                 	movups	xmmword ptr [rax + 0x70], xmm0
1413fb32e: 0f 11 88 80 00 00 00        	movups	xmmword ptr [rax + 0x80], xmm1
1413fb335: 0f 11 90 90 00 00 00        	movups	xmmword ptr [rax + 0x90], xmm2
1413fb33c: 0f 11 98 a0 00 00 00        	movups	xmmword ptr [rax + 0xa0], xmm3
1413fb343: 0f 10 85 38 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x138]
1413fb34a: 0f 11 80 b0 00 00 00        	movups	xmmword ptr [rax + 0xb0], xmm0
1413fb351: 0f 10 85 48 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x148]
1413fb358: 0f 11 80 c0 00 00 00        	movups	xmmword ptr [rax + 0xc0], xmm0
1413fb35f: 0f 10 85 58 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x158]
1413fb366: 0f 11 80 d0 00 00 00        	movups	xmmword ptr [rax + 0xd0], xmm0
1413fb36d: 0f 10 85 e8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xe8]
1413fb374: 0f 11 80 40 01 00 00        	movups	xmmword ptr [rax + 0x140], xmm0
1413fb37b: 0f 10 85 d8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xd8]
1413fb382: 0f 11 80 30 01 00 00        	movups	xmmword ptr [rax + 0x130], xmm0
1413fb389: 0f 10 85 c8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xc8]
1413fb390: 0f 11 80 20 01 00 00        	movups	xmmword ptr [rax + 0x120], xmm0
1413fb397: 0f 10 85 88 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x88]
1413fb39e: 0f 10 8d 98 00 00 00        	movups	xmm1, xmmword ptr [rbp + 0x98]
1413fb3a5: 0f 10 95 a8 00 00 00        	movups	xmm2, xmmword ptr [rbp + 0xa8]
1413fb3ac: 0f 10 9d b8 00 00 00        	movups	xmm3, xmmword ptr [rbp + 0xb8]
1413fb3b3: 0f 11 98 10 01 00 00        	movups	xmmword ptr [rax + 0x110], xmm3
1413fb3ba: 0f 11 90 00 01 00 00        	movups	xmmword ptr [rax + 0x100], xmm2
1413fb3c1: 0f 11 88 f0 00 00 00        	movups	xmmword ptr [rax + 0xf0], xmm1
1413fb3c8: 0f 11 80 e0 00 00 00        	movups	xmmword ptr [rax + 0xe0], xmm0
1413fb3cf: 0f 10 45 18                 	movups	xmm0, xmmword ptr [rbp + 0x18]
1413fb3d3: 0f 10 4d 28                 	movups	xmm1, xmmword ptr [rbp + 0x28]
1413fb3d7: 0f 10 55 38                 	movups	xmm2, xmmword ptr [rbp + 0x38]
1413fb3db: 0f 10 5d 48                 	movups	xmm3, xmmword ptr [rbp + 0x48]
1413fb3df: 0f 11 80 50 01 00 00        	movups	xmmword ptr [rax + 0x150], xmm0
1413fb3e6: 0f 11 88 60 01 00 00        	movups	xmmword ptr [rax + 0x160], xmm1
1413fb3ed: 0f 11 90 70 01 00 00        	movups	xmmword ptr [rax + 0x170], xmm2
1413fb3f4: 0f 11 98 80 01 00 00        	movups	xmmword ptr [rax + 0x180], xmm3
1413fb3fb: 0f 10 45 58                 	movups	xmm0, xmmword ptr [rbp + 0x58]
1413fb3ff: 0f 11 80 90 01 00 00        	movups	xmmword ptr [rax + 0x190], xmm0
1413fb406: 0f 10 45 68                 	movups	xmm0, xmmword ptr [rbp + 0x68]
1413fb40a: 0f 11 80 a0 01 00 00        	movups	xmmword ptr [rax + 0x1a0], xmm0
1413fb411: 0f 10 45 78                 	movups	xmm0, xmmword ptr [rbp + 0x78]
1413fb415: 0f 11 80 b0 01 00 00        	movups	xmmword ptr [rax + 0x1b0], xmm0
1413fb41c: 0f 10 45 a8                 	movups	xmm0, xmmword ptr [rbp - 0x58]
1413fb420: 0f 10 4d b8                 	movups	xmm1, xmmword ptr [rbp - 0x48]
1413fb424: 0f 10 55 c8                 	movups	xmm2, xmmword ptr [rbp - 0x38]
1413fb428: 0f 10 5d d8                 	movups	xmm3, xmmword ptr [rbp - 0x28]
1413fb42c: 0f 11 80 c0 01 00 00        	movups	xmmword ptr [rax + 0x1c0], xmm0
1413fb433: 0f 11 88 d0 01 00 00        	movups	xmmword ptr [rax + 0x1d0], xmm1
1413fb43a: 0f 11 90 e0 01 00 00        	movups	xmmword ptr [rax + 0x1e0], xmm2
1413fb441: 0f 11 98 f0 01 00 00        	movups	xmmword ptr [rax + 0x1f0], xmm3
1413fb448: 0f 10 45 e8                 	movups	xmm0, xmmword ptr [rbp - 0x18]
1413fb44c: 0f 11 80 00 02 00 00        	movups	xmmword ptr [rax + 0x200], xmm0
1413fb453: 0f 10 45 08                 	movups	xmm0, xmmword ptr [rbp + 0x8]
1413fb457: 0f 11 80 20 02 00 00        	movups	xmmword ptr [rax + 0x220], xmm0
1413fb45e: 0f 10 45 f8                 	movups	xmm0, xmmword ptr [rbp - 0x8]
1413fb462: 0f 11 80 10 02 00 00        	movups	xmmword ptr [rax + 0x210], xmm0
1413fb469: 48 c7 46 08 05 00 00 00     	mov	qword ptr [rsi + 0x8], 0x5
1413fb471: 48 89 46 10                 	mov	qword ptr [rsi + 0x10], rax
1413fb475: 48 c7 46 18 05 00 00 00     	mov	qword ptr [rsi + 0x18], 0x5
1413fb47d: 48 c7 46 20 00 00 00 00     	mov	qword ptr [rsi + 0x20], 0x0
1413fb485: 48 c7 46 28 08 00 00 00     	mov	qword ptr [rsi + 0x28], 0x8
1413fb48d: 0f 57 c0                    	xorps	xmm0, xmm0
1413fb490: 0f 11 46 30                 	movups	xmmword ptr [rsi + 0x30], xmm0
1413fb494: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1413fb49e: 48 89 06                    	mov	qword ptr [rsi], rax
1413fb4a1: 48 89 f0                    	mov	rax, rsi
1413fb4a4: 48 81 c4 68 02 00 00        	add	rsp, 0x268
1413fb4ab: 5e                          	pop	rsi
1413fb4ac: 5d                          	pop	rbp
1413fb4ad: c3                          	ret
1413fb4ae: b9 08 00 00 00              	mov	ecx, 0x8
1413fb4b3: ba 30 02 00 00              	mov	edx, 0x230
1413fb4b8: e8 43 db 4d 01              	call	0x1428d9000 <.text+0x28d8000>
1413fb4bd: cc                          	int3
1413fb4be: 66 90                       	nop
1413fb4c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1413fb4c5: 55                          	push	rbp
1413fb4c6: 56                          	push	rsi
1413fb4c7: 48 83 ec 28                 	sub	rsp, 0x28
1413fb4cb: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1413fb4d2: ba 30 02 00 00              	mov	edx, 0x230
1413fb4d7: 41 b8 08 00 00 00           	mov	r8d, 0x8
1413fb4dd: 48 8b 8d d8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1d8]
1413fb4e4: e8 37 87 21 ff              	call	0x140613c20 <.text+0x612c20>
1413fb4e9: 90                          	nop
1413fb4ea: 48 83 c4 28                 	add	rsp, 0x28
1413fb4ee: 5e                          	pop	rsi
1413fb4ef: 5d                          	pop	rbp
1413fb4f0: c3                          	ret
1413fb4f1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1413fb500: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1413fb505: 55                          	push	rbp
1413fb506: 56                          	push	rsi
1413fb507: 48 83 ec 28                 	sub	rsp, 0x28
1413fb50b: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1413fb512: 48 8d 8d 68 01 00 00        	lea	rcx, [rbp + 0x168]
1413fb519: e8 62 e3 c2 fe              	call	0x140029880 <.text+0x28880>
1413fb51e: 90                          	nop
1413fb51f: 48 83 c4 28                 	add	rsp, 0x28
1413fb523: 5e                          	pop	rsi
1413fb524: 5d                          	pop	rbp
1413fb525: c3                          	ret
1413fb526: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1413fb530: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1413fb535: 55                          	push	rbp
1413fb536: 56                          	push	rsi
1413fb537: 48 83 ec 28                 	sub	rsp, 0x28
1413fb53b: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1413fb542: 48 8d 8d f8 00 00 00        	lea	rcx, [rbp + 0xf8]
1413fb549: e8 32 e3 c2 fe              	call	0x140029880 <.text+0x28880>
1413fb54e: 90                          	nop
1413fb54f: 48 83 c4 28                 	add	rsp, 0x28
1413fb553: 5e                          	pop	rsi
1413fb554: 5d                          	pop	rbp
1413fb555: c3                          	ret
1413fb556: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1413fb560: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1413fb565: 55                          	push	rbp
1413fb566: 56                          	push	rsi
1413fb567: 48 83 ec 28                 	sub	rsp, 0x28
1413fb56b: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1413fb572: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
1413fb579: e8 02 e3 c2 fe              	call	0x140029880 <.text+0x28880>
1413fb57e: 90                          	nop
1413fb57f: 48 83 c4 28                 	add	rsp, 0x28
1413fb583: 5e                          	pop	rsi
1413fb584: 5d                          	pop	rbp
1413fb585: c3                          	ret
1413fb586: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1413fb590: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1413fb595: 55                          	push	rbp
1413fb596: 56                          	push	rsi
1413fb597: 48 83 ec 28                 	sub	rsp, 0x28
1413fb59b: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1413fb5a2: 48 8d 4d 18                 	lea	rcx, [rbp + 0x18]
1413fb5a6: e8 d5 e2 c2 fe              	call	0x140029880 <.text+0x28880>
1413fb5ab: 90                          	nop
1413fb5ac: 48 83 c4 28                 	add	rsp, 0x28
1413fb5b0: 5e                          	pop	rsi
1413fb5b1: 5d                          	pop	rbp
1413fb5b2: c3                          	ret

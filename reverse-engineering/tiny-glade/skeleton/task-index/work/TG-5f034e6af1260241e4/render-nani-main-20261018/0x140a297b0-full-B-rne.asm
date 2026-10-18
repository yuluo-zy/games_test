
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a297b0: 41 57                       	push	r15
140a297b2: 41 56                       	push	r14
140a297b4: 41 55                       	push	r13
140a297b6: 41 54                       	push	r12
140a297b8: 56                          	push	rsi
140a297b9: 57                          	push	rdi
140a297ba: 55                          	push	rbp
140a297bb: 53                          	push	rbx
140a297bc: 48 81 ec 58 03 00 00        	sub	rsp, 0x358
140a297c3: 44 89 cb                    	mov	ebx, r9d
140a297c6: 49 89 d6                    	mov	r14, rdx
140a297c9: 49 89 cf                    	mov	r15, rcx
140a297cc: 4c 8b ac 24 d0 03 00 00     	mov	r13, qword ptr [rsp + 0x3d0]
140a297d4: 49 8b b8 50 14 00 00        	mov	rdi, qword ptr [r8 + 0x1450]
140a297db: 48 89 4c 24 30              	mov	qword ptr [rsp + 0x30], rcx
140a297e0: 48 89 54 24 28              	mov	qword ptr [rsp + 0x28], rdx
140a297e5: 48 c7 44 24 20 27 00 00 00  	mov	qword ptr [rsp + 0x20], 0x27
140a297ee: 48 8d 15 f2 12 0a 02        	lea	rdx, [rip + 0x20a12f2]  # 0x142acaae7
140a297f5: 4c 8d 0d 09 13 0a 02        	lea	r9, [rip + 0x20a1309]   # 0x142acab05
140a297fc: 4c 8d 64 24 38              	lea	r12, [rsp + 0x38]
140a29801: 41 b8 1e 00 00 00           	mov	r8d, 0x1e
140a29807: 4c 89 e1                    	mov	rcx, r12
140a2980a: e8 91 9a bd 00              	call	0x1416032a0 <.text+0x16022a0>
140a2980f: 48 8d ac 24 c8 01 00 00     	lea	rbp, [rsp + 0x1c8]
140a29817: 49 b8 74 44 1c 32 11 a8 61 86       	movabs	r8, -0x799e57eecde3bb8c
140a29821: 48 89 e9                    	mov	rcx, rbp
140a29824: 4c 89 e2                    	mov	rdx, r12
140a29827: e8 d4 d0 bd 00              	call	0x141606900 <.text+0x1605900>
140a2982c: 48 8d 74 24 38              	lea	rsi, [rsp + 0x38]
140a29831: 49 bc 48 72 ad 18 d3 61 b8 aa       	movabs	r12, -0x55479e2ce7528db8
140a2983b: 48 89 f1                    	mov	rcx, rsi
140a2983e: 48 89 ea                    	mov	rdx, rbp
140a29841: 4d 89 e0                    	mov	r8, r12
140a29844: 4d 89 e9                    	mov	r9, r13
140a29847: e8 14 45 f4 ff              	call	0x14096dd60 <.text+0x96cd60>
140a2984c: 4c 8d ac 24 c8 01 00 00     	lea	r13, [rsp + 0x1c8]
140a29854: 49 b8 90 69 1f 97 40 44 67 c3       	movabs	r8, -0x3c98bbbf68e09670
140a2985e: 4c 89 e9                    	mov	rcx, r13
140a29861: 48 89 f2                    	mov	rdx, rsi
140a29864: 4c 8b 8c 24 d8 03 00 00     	mov	r9, qword ptr [rsp + 0x3d8]
140a2986c: e8 ef 44 f4 ff              	call	0x14096dd60 <.text+0x96cd60>
140a29871: 88 5c 24 40                 	mov	byte ptr [rsp + 0x40], bl
140a29875: 0f b6 ac 24 c0 03 00 00     	movzx	ebp, byte ptr [rsp + 0x3c0]
140a2987d: 40 88 6c 24 41              	mov	byte ptr [rsp + 0x41], bpl
140a29882: 48 89 7c 24 38              	mov	qword ptr [rsp + 0x38], rdi
140a29887: 4c 8d 44 24 38              	lea	r8, [rsp + 0x38]
140a2988c: 4c 89 e9                    	mov	rcx, r13
140a2988f: 4c 8b ac 24 c8 03 00 00     	mov	r13, qword ptr [rsp + 0x3c8]
140a29897: 4c 89 ea                    	mov	rdx, r13
140a2989a: e8 b1 1f be 00              	call	0x14160b850 <.text+0x160a850>
140a2989f: 4c 89 7c 24 30              	mov	qword ptr [rsp + 0x30], r15
140a298a4: 4c 89 74 24 28              	mov	qword ptr [rsp + 0x28], r14
140a298a9: 48 c7 44 24 20 21 00 00 00  	mov	qword ptr [rsp + 0x20], 0x21
140a298b2: 48 8d 15 73 12 0a 02        	lea	rdx, [rip + 0x20a1273]  # 0x142acab2c
140a298b9: 4c 8d 0d 89 12 0a 02        	lea	r9, [rip + 0x20a1289]   # 0x142acab49
140a298c0: 48 8d 74 24 38              	lea	rsi, [rsp + 0x38]
140a298c5: 41 b8 1d 00 00 00           	mov	r8d, 0x1d
140a298cb: 48 89 f1                    	mov	rcx, rsi
140a298ce: e8 cd 99 bd 00              	call	0x1416032a0 <.text+0x16022a0>
140a298d3: 4c 8d b4 24 c8 01 00 00     	lea	r14, [rsp + 0x1c8]
140a298db: 4c 89 f1                    	mov	rcx, r14
140a298de: 48 89 f2                    	mov	rdx, rsi
140a298e1: 4d 89 e0                    	mov	r8, r12
140a298e4: 4c 8b 8c 24 d8 03 00 00     	mov	r9, qword ptr [rsp + 0x3d8]
140a298ec: e8 6f 44 f4 ff              	call	0x14096dd60 <.text+0x96cd60>
140a298f1: 88 5c 24 40                 	mov	byte ptr [rsp + 0x40], bl
140a298f5: 40 88 6c 24 41              	mov	byte ptr [rsp + 0x41], bpl
140a298fa: 48 89 7c 24 38              	mov	qword ptr [rsp + 0x38], rdi
140a298ff: 4c 8d 44 24 38              	lea	r8, [rsp + 0x38]
140a29904: 4c 89 f1                    	mov	rcx, r14
140a29907: 4c 89 ea                    	mov	rdx, r13
140a2990a: e8 71 1c be 00              	call	0x14160b580 <.text+0x160a580>
140a2990f: 90                          	nop
140a29910: 48 81 c4 58 03 00 00        	add	rsp, 0x358
140a29917: 5b                          	pop	rbx
140a29918: 5d                          	pop	rbp
140a29919: 5f                          	pop	rdi
140a2991a: 5e                          	pop	rsi
140a2991b: 41 5c                       	pop	r12
140a2991d: 41 5d                       	pop	r13
140a2991f: 41 5e                       	pop	r14
140a29921: 41 5f                       	pop	r15
140a29923: c3                          	ret

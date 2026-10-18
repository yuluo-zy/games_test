
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405b93b0: 55                          	push	rbp
1405b93b1: 41 57                       	push	r15
1405b93b3: 41 56                       	push	r14
1405b93b5: 41 55                       	push	r13
1405b93b7: 41 54                       	push	r12
1405b93b9: 56                          	push	rsi
1405b93ba: 57                          	push	rdi
1405b93bb: 53                          	push	rbx
1405b93bc: 48 81 ec 68 01 00 00        	sub	rsp, 0x168
1405b93c3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1405b93cb: 48 c7 85 e0 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xe0], -0x2
1405b93d6: 4d 89 cf                    	mov	r15, r9
1405b93d9: 4d 89 c6                    	mov	r14, r8
1405b93dc: 48 89 d7                    	mov	rdi, rdx
1405b93df: 48 89 ce                    	mov	rsi, rcx
1405b93e2: 4c 8b ad 50 01 00 00        	mov	r13, qword ptr [rbp + 0x150]
1405b93e9: 0f 10 85 58 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x158]
1405b93f0: 0f 11 85 d0 00 00 00        	movups	xmmword ptr [rbp + 0xd0], xmm0
1405b93f7: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
1405b93fe: e8 9d bd 61 00              	call	0x140bd51a0 <.text+0xbd41a0>
1405b9403: 48 85 c0                    	test	rax, rax
1405b9406: 74 05                       	je	0x1405b940d <.text+0x5b840d>
1405b9408: 41 80 4d 08 02              	or	byte ptr [r13 + 0x8], 0x2
1405b940d: 48 8d 5f 78                 	lea	rbx, [rdi + 0x78]
1405b9411: 48 89 f9                    	mov	rcx, rdi
1405b9414: e8 47 f5 63 00              	call	0x140bf8960 <.text+0xbf7960>
1405b9419: 4c 8b 8f 48 06 00 00        	mov	r9, qword ptr [rdi + 0x648]
1405b9420: 48 8b 8f 78 06 00 00        	mov	rcx, qword ptr [rdi + 0x678]
1405b9427: 48 83 c1 10                 	add	rcx, 0x10
1405b942b: 41 0f 10 45 00              	movups	xmm0, xmmword ptr [r13]
1405b9430: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
1405b9434: 4c 89 7c 24 38              	mov	qword ptr [rsp + 0x38], r15
1405b9439: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
1405b943e: 4c 8d 65 20                 	lea	r12, [rbp + 0x20]
1405b9442: 4c 89 64 24 28              	mov	qword ptr [rsp + 0x28], r12
1405b9447: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
1405b944c: 48 8d 4d 50                 	lea	rcx, [rbp + 0x50]
1405b9450: 48 89 da                    	mov	rdx, rbx
1405b9453: 49 89 c0                    	mov	r8, rax
1405b9456: e8 a5 4d 63 00              	call	0x140bee200 <.text+0xbed200>
1405b945b: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
1405b945f: 0f 10 45 58                 	movups	xmm0, xmmword ptr [rbp + 0x58]
1405b9463: 0f 29 85 a0 00 00 00        	movaps	xmmword ptr [rbp + 0xa0], xmm0
1405b946a: 0f 10 45 68                 	movups	xmm0, xmmword ptr [rbp + 0x68]
1405b946e: 0f 29 85 b0 00 00 00        	movaps	xmmword ptr [rbp + 0xb0], xmm0
1405b9475: 0f 10 45 78                 	movups	xmm0, xmmword ptr [rbp + 0x78]
1405b9479: 0f 29 85 c0 00 00 00        	movaps	xmmword ptr [rbp + 0xc0], xmm0
1405b9480: 4c 8b b5 88 00 00 00        	mov	r14, qword ptr [rbp + 0x88]
1405b9487: 4c 8b bd 90 00 00 00        	mov	r15, qword ptr [rbp + 0x90]
1405b948e: 48 83 f8 0b                 	cmp	rax, 0xb
1405b9492: 0f 85 9c 00 00 00           	jne	0x1405b9534 <.text+0x5b8534>
1405b9498: 0f 28 85 a0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xa0]
1405b949f: 0f 28 8d b0 00 00 00        	movaps	xmm1, xmmword ptr [rbp + 0xb0]
1405b94a6: 0f 28 95 c0 00 00 00        	movaps	xmm2, xmmword ptr [rbp + 0xc0]
1405b94ad: 0f 29 55 40                 	movaps	xmmword ptr [rbp + 0x40], xmm2
1405b94b1: 0f 29 4d 30                 	movaps	xmmword ptr [rbp + 0x30], xmm1
1405b94b5: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
1405b94b9: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
1405b94c0: e8 db bc 61 00              	call	0x140bd51a0 <.text+0xbd41a0>
1405b94c5: 48 85 c0                    	test	rax, rax
1405b94c8: 74 49                       	je	0x1405b9513 <.text+0x5b8513>
1405b94ca: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
1405b94d1: e8 ca bc 61 00              	call	0x140bd51a0 <.text+0xbd41a0>
1405b94d6: 49 8b 55 00                 	mov	rdx, qword ptr [r13]
1405b94da: 48 39 d0                    	cmp	rax, rdx
1405b94dd: 0f 87 4f 02 00 00           	ja	0x1405b9732 <.text+0x5b8732>
1405b94e3: 4d 85 f6                    	test	r14, r14
1405b94e6: 0f 84 96 00 00 00           	je	0x1405b9582 <.text+0x5b8582>
1405b94ec: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
1405b94f3: e8 a8 bc 61 00              	call	0x140bd51a0 <.text+0xbd41a0>
1405b94f8: 4c 39 f8                    	cmp	rax, r15
1405b94fb: 0f 87 4b 02 00 00           	ja	0x1405b974c <.text+0x5b874c>
1405b9501: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
1405b9508: 4c 89 f2                    	mov	rdx, r14
1405b950b: 49 89 c0                    	mov	r8, rax
1405b950e: e8 9d bc 61 00              	call	0x140bd51b0 <.text+0xbd41b0>
1405b9513: 48 8b 45 48                 	mov	rax, qword ptr [rbp + 0x48]
1405b9517: 48 89 46 28                 	mov	qword ptr [rsi + 0x28], rax
1405b951b: 0f 10 45 28                 	movups	xmm0, xmmword ptr [rbp + 0x28]
1405b951f: 0f 10 4d 38                 	movups	xmm1, xmmword ptr [rbp + 0x38]
1405b9523: 0f 11 4e 18                 	movups	xmmword ptr [rsi + 0x18], xmm1
1405b9527: 0f 11 46 08                 	movups	xmmword ptr [rsi + 0x8], xmm0
1405b952b: 48 c7 06 0b 00 00 00        	mov	qword ptr [rsi], 0xb
1405b9532: eb 37                       	jmp	0x1405b956b <.text+0x5b856b>
1405b9534: 48 8b 8d 98 00 00 00        	mov	rcx, qword ptr [rbp + 0x98]
1405b953b: 0f 28 85 a0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xa0]
1405b9542: 0f 28 8d b0 00 00 00        	movaps	xmm1, xmmword ptr [rbp + 0xb0]
1405b9549: 0f 28 95 c0 00 00 00        	movaps	xmm2, xmmword ptr [rbp + 0xc0]
1405b9550: 0f 11 56 28                 	movups	xmmword ptr [rsi + 0x28], xmm2
1405b9554: 0f 11 4e 18                 	movups	xmmword ptr [rsi + 0x18], xmm1
1405b9558: 0f 11 46 08                 	movups	xmmword ptr [rsi + 0x8], xmm0
1405b955c: 48 89 06                    	mov	qword ptr [rsi], rax
1405b955f: 4c 89 76 38                 	mov	qword ptr [rsi + 0x38], r14
1405b9563: 4c 89 7e 40                 	mov	qword ptr [rsi + 0x40], r15
1405b9567: 48 89 4e 48                 	mov	qword ptr [rsi + 0x48], rcx
1405b956b: 48 89 f0                    	mov	rax, rsi
1405b956e: 48 81 c4 68 01 00 00        	add	rsp, 0x168
1405b9575: 5b                          	pop	rbx
1405b9576: 5f                          	pop	rdi
1405b9577: 5e                          	pop	rsi
1405b9578: 41 5c                       	pop	r12
1405b957a: 41 5d                       	pop	r13
1405b957c: 41 5e                       	pop	r14
1405b957e: 41 5f                       	pop	r15
1405b9580: 5d                          	pop	rbp
1405b9581: c3                          	ret
1405b9582: 4c 8d 75 c0                 	lea	r14, [rbp - 0x40]
1405b9586: 4c 89 f1                    	mov	rcx, r14
1405b9589: 41 b8 01 00 00 00           	mov	r8d, 0x1
1405b958f: e8 0c b9 61 00              	call	0x140bd4ea0 <.text+0xbd3ea0>
1405b9594: 48 89 f9                    	mov	rcx, rdi
1405b9597: e8 c4 f3 63 00              	call	0x140bf8960 <.text+0xbf7960>
1405b959c: 4c 8b 8f 48 06 00 00        	mov	r9, qword ptr [rdi + 0x648]
1405b95a3: 48 8b 8f 78 06 00 00        	mov	rcx, qword ptr [rdi + 0x678]
1405b95aa: 48 83 c1 10                 	add	rcx, 0x10
1405b95ae: 48 8d 15 63 b2 44 02        	lea	rdx, [rip + 0x244b263]  # 0x142a04818
1405b95b5: 48 89 54 24 30              	mov	qword ptr [rsp + 0x30], rdx
1405b95ba: 4c 89 74 24 28              	mov	qword ptr [rsp + 0x28], r14
1405b95bf: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
1405b95c4: 48 c7 44 24 38 13 00 00 00  	mov	qword ptr [rsp + 0x38], 0x13
1405b95cd: 4c 8d 75 50                 	lea	r14, [rbp + 0x50]
1405b95d1: 4c 89 f1                    	mov	rcx, r14
1405b95d4: 48 89 da                    	mov	rdx, rbx
1405b95d7: 49 89 c0                    	mov	r8, rax
1405b95da: e8 21 4c 63 00              	call	0x140bee200 <.text+0xbed200>
1405b95df: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
1405b95e3: 0f 10 45 58                 	movups	xmm0, xmmword ptr [rbp + 0x58]
1405b95e7: 0f 29 85 a0 00 00 00        	movaps	xmmword ptr [rbp + 0xa0], xmm0
1405b95ee: 0f 10 45 68                 	movups	xmm0, xmmword ptr [rbp + 0x68]
1405b95f2: 0f 29 85 b0 00 00 00        	movaps	xmmword ptr [rbp + 0xb0], xmm0
1405b95f9: 0f 10 45 78                 	movups	xmm0, xmmword ptr [rbp + 0x78]
1405b95fd: 0f 29 85 c0 00 00 00        	movaps	xmmword ptr [rbp + 0xc0], xmm0
1405b9604: 48 8b 9d 88 00 00 00        	mov	rbx, qword ptr [rbp + 0x88]
1405b960b: 4c 8b bd 90 00 00 00        	mov	r15, qword ptr [rbp + 0x90]
1405b9612: 48 83 f8 0b                 	cmp	rax, 0xb
1405b9616: 0f 85 9b 00 00 00           	jne	0x1405b96b7 <.text+0x5b86b7>
1405b961c: 0f 28 85 a0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xa0]
1405b9623: 0f 28 8d b0 00 00 00        	movaps	xmm1, xmmword ptr [rbp + 0xb0]
1405b962a: 0f 28 95 c0 00 00 00        	movaps	xmm2, xmmword ptr [rbp + 0xc0]
1405b9631: 0f 29 55 70                 	movaps	xmmword ptr [rbp + 0x70], xmm2
1405b9635: 0f 29 4d 60                 	movaps	xmmword ptr [rbp + 0x60], xmm1
1405b9639: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
1405b963d: 48 85 db                    	test	rbx, rbx
1405b9640: 0f 84 2e 01 00 00           	je	0x1405b9774 <.text+0x5b8774>
1405b9646: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
1405b964d: e8 4e bb 61 00              	call	0x140bd51a0 <.text+0xbd41a0>
1405b9652: 4c 39 f8                    	cmp	rax, r15
1405b9655: 0f 87 05 01 00 00           	ja	0x1405b9760 <.text+0x5b8760>
1405b965b: 4c 8d bd d0 00 00 00        	lea	r15, [rbp + 0xd0]
1405b9662: 4c 89 f9                    	mov	rcx, r15
1405b9665: 48 89 da                    	mov	rdx, rbx
1405b9668: 49 89 c0                    	mov	r8, rax
1405b966b: e8 40 bb 61 00              	call	0x140bd51b0 <.text+0xbd41b0>
1405b9670: 4c 89 b5 a0 00 00 00        	mov	qword ptr [rbp + 0xa0], r14
1405b9677: 48 89 bd a8 00 00 00        	mov	qword ptr [rbp + 0xa8], rdi
1405b967e: 4c 89 a5 b0 00 00 00        	mov	qword ptr [rbp + 0xb0], r12
1405b9685: 4c 89 bd b8 00 00 00        	mov	qword ptr [rbp + 0xb8], r15
1405b968c: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
1405b9690: 4c 8d 85 a0 00 00 00        	lea	r8, [rbp + 0xa0]
1405b9697: 48 89 fa                    	mov	rdx, rdi
1405b969a: e8 a1 b1 64 00              	call	0x140c04840 <.text+0xc03840>
1405b969f: 48 8b 45 d0                 	mov	rax, qword ptr [rbp - 0x30]
1405b96a3: 48 83 f8 0b                 	cmp	rax, 0xb
1405b96a7: 75 47                       	jne	0x1405b96f0 <.text+0x5b86f0>
1405b96a9: 48 8d 4d 58                 	lea	rcx, [rbp + 0x58]
1405b96ad: e8 fe 8d 62 00              	call	0x140be24b0 <.text+0xbe14b0>
1405b96b2: e9 5c fe ff ff              	jmp	0x1405b9513 <.text+0x5b8513>
1405b96b7: 48 8b 8d 98 00 00 00        	mov	rcx, qword ptr [rbp + 0x98]
1405b96be: 0f 28 85 a0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xa0]
1405b96c5: 0f 28 8d b0 00 00 00        	movaps	xmm1, xmmword ptr [rbp + 0xb0]
1405b96cc: 0f 28 95 c0 00 00 00        	movaps	xmm2, xmmword ptr [rbp + 0xc0]
1405b96d3: 0f 11 56 28                 	movups	xmmword ptr [rsi + 0x28], xmm2
1405b96d7: 0f 11 4e 18                 	movups	xmmword ptr [rsi + 0x18], xmm1
1405b96db: 0f 11 46 08                 	movups	xmmword ptr [rsi + 0x8], xmm0
1405b96df: 48 89 06                    	mov	qword ptr [rsi], rax
1405b96e2: 48 89 5e 38                 	mov	qword ptr [rsi + 0x38], rbx
1405b96e6: 4c 89 7e 40                 	mov	qword ptr [rsi + 0x40], r15
1405b96ea: 48 89 4e 48                 	mov	qword ptr [rsi + 0x48], rcx
1405b96ee: eb 34                       	jmp	0x1405b9724 <.text+0x5b8724>
1405b96f0: 48 8b 4d 18                 	mov	rcx, qword ptr [rbp + 0x18]
1405b96f4: 48 89 4e 48                 	mov	qword ptr [rsi + 0x48], rcx
1405b96f8: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1405b96fc: 0f 10 4d e8                 	movups	xmm1, xmmword ptr [rbp - 0x18]
1405b9700: 0f 10 55 f8                 	movups	xmm2, xmmword ptr [rbp - 0x8]
1405b9704: 0f 10 5d 08                 	movups	xmm3, xmmword ptr [rbp + 0x8]
1405b9708: 0f 11 5e 38                 	movups	xmmword ptr [rsi + 0x38], xmm3
1405b970c: 0f 11 56 28                 	movups	xmmword ptr [rsi + 0x28], xmm2
1405b9710: 0f 11 4e 18                 	movups	xmmword ptr [rsi + 0x18], xmm1
1405b9714: 0f 11 46 08                 	movups	xmmword ptr [rsi + 0x8], xmm0
1405b9718: 48 89 06                    	mov	qword ptr [rsi], rax
1405b971b: 48 8d 4d 58                 	lea	rcx, [rbp + 0x58]
1405b971f: e8 8c 8d 62 00              	call	0x140be24b0 <.text+0xbe14b0>
1405b9724: 48 8d 4d 28                 	lea	rcx, [rbp + 0x28]
1405b9728: e8 83 8d 62 00              	call	0x140be24b0 <.text+0xbe14b0>
1405b972d: e9 39 fe ff ff              	jmp	0x1405b956b <.text+0x5b856b>
1405b9732: 48 8d 0d 21 b0 44 02        	lea	rcx, [rip + 0x244b021]  # 0x142a0475a
1405b9739: 4c 8d 05 a8 b0 44 02        	lea	r8, [rip + 0x244b0a8]   # 0x142a047e8
1405b9740: ba 38 00 00 00              	mov	edx, 0x38
1405b9745: e8 e6 fc 31 02              	call	0x1428d9430 <.text+0x28d8430>
1405b974a: eb 40                       	jmp	0x1405b978c <.text+0x5b878c>
1405b974c: 4c 8d 05 ad b0 44 02        	lea	r8, [rip + 0x244b0ad]   # 0x142a04800
1405b9753: 48 89 c1                    	mov	rcx, rax
1405b9756: 4c 89 fa                    	mov	rdx, r15
1405b9759: e8 e2 00 32 02              	call	0x1428d9840 <.text+0x28d8840>
1405b975e: eb 2c                       	jmp	0x1405b978c <.text+0x5b878c>
1405b9760: 4c 8d 05 f1 b0 44 02        	lea	r8, [rip + 0x244b0f1]   # 0x142a04858
1405b9767: 48 89 c1                    	mov	rcx, rax
1405b976a: 4c 89 fa                    	mov	rdx, r15
1405b976d: e8 ce 00 32 02              	call	0x1428d9840 <.text+0x28d8840>
1405b9772: eb 18                       	jmp	0x1405b978c <.text+0x5b878c>
1405b9774: 48 8d 0d b0 b0 44 02        	lea	rcx, [rip + 0x244b0b0]  # 0x142a0482b
1405b977b: 4c 8d 05 be b0 44 02        	lea	r8, [rip + 0x244b0be]   # 0x142a04840
1405b9782: ba 14 00 00 00              	mov	edx, 0x14
1405b9787: e8 a4 fb 31 02              	call	0x1428d9330 <.text+0x28d8330>
1405b978c: 0f 0b                       	ud2
1405b978e: 66 90                       	nop
1405b9790: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405b9795: 55                          	push	rbp
1405b9796: 41 57                       	push	r15
1405b9798: 41 56                       	push	r14
1405b979a: 41 55                       	push	r13
1405b979c: 41 54                       	push	r12
1405b979e: 56                          	push	rsi
1405b979f: 57                          	push	rdi
1405b97a0: 53                          	push	rbx
1405b97a1: 48 83 ec 48                 	sub	rsp, 0x48
1405b97a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405b97ac: 48 8d 4d 58                 	lea	rcx, [rbp + 0x58]
1405b97b0: e8 fb 8c 62 00              	call	0x140be24b0 <.text+0xbe14b0>
1405b97b5: 90                          	nop
1405b97b6: 48 83 c4 48                 	add	rsp, 0x48
1405b97ba: 5b                          	pop	rbx
1405b97bb: 5f                          	pop	rdi
1405b97bc: 5e                          	pop	rsi
1405b97bd: 41 5c                       	pop	r12
1405b97bf: 41 5d                       	pop	r13
1405b97c1: 41 5e                       	pop	r14
1405b97c3: 41 5f                       	pop	r15
1405b97c5: 5d                          	pop	rbp
1405b97c6: c3                          	ret
1405b97c7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1405b97d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1405b97d5: 55                          	push	rbp
1405b97d6: 41 57                       	push	r15
1405b97d8: 41 56                       	push	r14
1405b97da: 41 55                       	push	r13
1405b97dc: 41 54                       	push	r12
1405b97de: 56                          	push	rsi
1405b97df: 57                          	push	rdi
1405b97e0: 53                          	push	rbx
1405b97e1: 48 83 ec 48                 	sub	rsp, 0x48
1405b97e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1405b97ec: 48 8d 4d 28                 	lea	rcx, [rbp + 0x28]
1405b97f0: e8 bb 8c 62 00              	call	0x140be24b0 <.text+0xbe14b0>
1405b97f5: 90                          	nop
1405b97f6: 48 83 c4 48                 	add	rsp, 0x48
1405b97fa: 5b                          	pop	rbx
1405b97fb: 5f                          	pop	rdi
1405b97fc: 5e                          	pop	rsi
1405b97fd: 41 5c                       	pop	r12
1405b97ff: 41 5d                       	pop	r13
1405b9801: 41 5e                       	pop	r14
1405b9803: 41 5f                       	pop	r15
1405b9805: 5d                          	pop	rbp
1405b9806: c3                          	ret

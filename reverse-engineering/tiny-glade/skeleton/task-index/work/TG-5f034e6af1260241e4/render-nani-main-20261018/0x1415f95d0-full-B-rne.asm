
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415f95d0: 55                          	push	rbp
1415f95d1: 41 57                       	push	r15
1415f95d3: 41 56                       	push	r14
1415f95d5: 41 55                       	push	r13
1415f95d7: 41 54                       	push	r12
1415f95d9: 56                          	push	rsi
1415f95da: 57                          	push	rdi
1415f95db: 53                          	push	rbx
1415f95dc: 48 83 ec 68                 	sub	rsp, 0x68
1415f95e0: 48 8d 6c 24 60              	lea	rbp, [rsp + 0x60]
1415f95e5: 48 c7 45 00 fe ff ff ff     	mov	qword ptr [rbp], -0x2
1415f95ed: 48 89 55 e0                 	mov	qword ptr [rbp - 0x20], rdx
1415f95f1: 4c 89 45 e8                 	mov	qword ptr [rbp - 0x18], r8
1415f95f5: 48 8b 01                    	mov	rax, qword ptr [rcx]
1415f95f8: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1415f95fc: 48 8b 41 08                 	mov	rax, qword ptr [rcx + 0x8]
1415f9600: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1415f9604: 48 8b 41 10                 	mov	rax, qword ptr [rcx + 0x10]
1415f9608: 48 85 c0                    	test	rax, rax
1415f960b: 74 75                       	je	0x1415f9682 <.text+0x15f8682>
1415f960d: 48 8b 75 f0                 	mov	rsi, qword ptr [rbp - 0x10]
1415f9611: 4c 8d 34 c6                 	lea	r14, [rsi + 8*rax]
1415f9615: 44 0f b6 79 1c              	movzx	r15d, byte ptr [rcx + 0x1c]
1415f961a: 44 8b 61 18                 	mov	r12d, dword ptr [rcx + 0x18]
1415f961e: 48 8d 7d e0                 	lea	rdi, [rbp - 0x20]
1415f9622: 48 8d 5d c4                 	lea	rbx, [rbp - 0x3c]
1415f9626: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1415f9630: 8b 06                       	mov	eax, dword ptr [rsi]
1415f9632: 44 8b 6e 04                 	mov	r13d, dword ptr [rsi + 0x4]
1415f9636: 44 89 7d d0                 	mov	dword ptr [rbp - 0x30], r15d
1415f963a: 89 45 d4                    	mov	dword ptr [rbp - 0x2c], eax
1415f963d: 44 89 6d d8                 	mov	dword ptr [rbp - 0x28], r13d
1415f9641: 44 89 65 dc                 	mov	dword ptr [rbp - 0x24], r12d
1415f9645: 48 8b 4d e0                 	mov	rcx, qword ptr [rbp - 0x20]
1415f9649: 48 8b 55 e8                 	mov	rdx, qword ptr [rbp - 0x18]
1415f964d: 41 b9 10 00 00 00           	mov	r9d, 0x10
1415f9653: 4c 8d 45 d0                 	lea	r8, [rbp - 0x30]
1415f9657: e8 f4 e3 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415f965c: 44 89 6d c4                 	mov	dword ptr [rbp - 0x3c], r13d
1415f9660: 48 b8 01 00 00 00 01 00 00 00       	movabs	rax, 0x100000001
1415f966a: 48 89 45 c8                 	mov	qword ptr [rbp - 0x38], rax
1415f966e: 48 89 f9                    	mov	rcx, rdi
1415f9671: 48 89 da                    	mov	rdx, rbx
1415f9674: e8 27 59 37 ff              	call	0x14096efa0 <.text+0x96dfa0>
1415f9679: 48 83 c6 08                 	add	rsi, 0x8
1415f967d: 4c 39 f6                    	cmp	rsi, r14
1415f9680: 75 ae                       	jne	0x1415f9630 <.text+0x15f8630>
1415f9682: 48 8b 55 f8                 	mov	rdx, qword ptr [rbp - 0x8]
1415f9686: 48 85 d2                    	test	rdx, rdx
1415f9689: 74 13                       	je	0x1415f969e <.text+0x15f869e>
1415f968b: 48 c1 e2 03                 	shl	rdx, 0x3
1415f968f: 41 b8 04 00 00 00           	mov	r8d, 0x4
1415f9695: 48 8b 4d f0                 	mov	rcx, qword ptr [rbp - 0x10]
1415f9699: e8 82 a5 01 ff              	call	0x140613c20 <.text+0x612c20>
1415f969e: 90                          	nop
1415f969f: 48 83 c4 68                 	add	rsp, 0x68
1415f96a3: 5b                          	pop	rbx
1415f96a4: 5f                          	pop	rdi
1415f96a5: 5e                          	pop	rsi
1415f96a6: 41 5c                       	pop	r12
1415f96a8: 41 5d                       	pop	r13
1415f96aa: 41 5e                       	pop	r14
1415f96ac: 41 5f                       	pop	r15
1415f96ae: 5d                          	pop	rbp
1415f96af: c3                          	ret
1415f96b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1415f96b5: 55                          	push	rbp
1415f96b6: 41 57                       	push	r15
1415f96b8: 41 56                       	push	r14
1415f96ba: 41 55                       	push	r13
1415f96bc: 41 54                       	push	r12
1415f96be: 56                          	push	rsi
1415f96bf: 57                          	push	rdi
1415f96c0: 53                          	push	rbx
1415f96c1: 48 83 ec 28                 	sub	rsp, 0x28
1415f96c5: 48 8d 6a 60                 	lea	rbp, [rdx + 0x60]
1415f96c9: 48 83 7d f8 00              	cmp	qword ptr [rbp - 0x8], 0x0
1415f96ce: 74 17                       	je	0x1415f96e7 <.text+0x15f86e7>
1415f96d0: 48 8b 55 f8                 	mov	rdx, qword ptr [rbp - 0x8]
1415f96d4: 48 c1 e2 03                 	shl	rdx, 0x3
1415f96d8: 41 b8 04 00 00 00           	mov	r8d, 0x4
1415f96de: 48 8b 4d f0                 	mov	rcx, qword ptr [rbp - 0x10]
1415f96e2: e8 39 a5 01 ff              	call	0x140613c20 <.text+0x612c20>
1415f96e7: 90                          	nop
1415f96e8: 48 83 c4 28                 	add	rsp, 0x28
1415f96ec: 5b                          	pop	rbx
1415f96ed: 5f                          	pop	rdi
1415f96ee: 5e                          	pop	rsi
1415f96ef: 41 5c                       	pop	r12
1415f96f1: 41 5d                       	pop	r13
1415f96f3: 41 5e                       	pop	r14
1415f96f5: 41 5f                       	pop	r15
1415f96f7: 5d                          	pop	rbp
1415f96f8: c3                          	ret

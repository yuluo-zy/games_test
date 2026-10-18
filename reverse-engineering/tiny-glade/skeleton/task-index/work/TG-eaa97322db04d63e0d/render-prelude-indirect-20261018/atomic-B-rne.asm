
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416148e0: 55                          	push	rbp
1416148e1: 41 57                       	push	r15
1416148e3: 41 56                       	push	r14
1416148e5: 41 54                       	push	r12
1416148e7: 56                          	push	rsi
1416148e8: 57                          	push	rdi
1416148e9: 53                          	push	rbx
1416148ea: 48 81 ec 00 01 00 00        	sub	rsp, 0x100
1416148f1: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1416148f9: 48 c7 45 78 fe ff ff ff     	mov	qword ptr [rbp + 0x78], -0x2
141614901: 4c 89 c3                    	mov	rbx, r8
141614904: 48 89 d7                    	mov	rdi, rdx
141614907: 48 89 ce                    	mov	rsi, rcx
14161490a: 4d 85 c0                    	test	r8, r8
14161490d: 0f 84 8c 00 00 00           	je	0x14161499f <.text+0x161399f>
141614913: 4d 89 ce                    	mov	r14, r9
141614916: 4c 89 c8                    	mov	rax, r9
141614919: 48 83 e0 03                 	and	rax, 0x3
14161491d: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
141614921: 0f 85 04 01 00 00           	jne	0x141614a2b <.text+0x1613a2b>
141614927: 48 89 d8                    	mov	rax, rbx
14161492a: 48 83 e0 03                 	and	rax, 0x3
14161492e: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
141614932: 0f 85 04 01 00 00           	jne	0x141614a3c <.text+0x1613a3c>
141614938: 49 83 fe 0c                 	cmp	r14, 0xc
14161493c: 72 64                       	jb	0x1416149a2 <.text+0x16139a2>
14161493e: 48 8d 05 17 a4 48 01        	lea	rax, [rip + 0x148a417]  # 0x142a9ed5c
141614945: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
141614949: 48 c7 45 c8 48 00 00 00     	mov	qword ptr [rbp - 0x38], 0x48
141614951: 48 8d 45 c0                 	lea	rax, [rbp - 0x40]
141614955: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
141614959: 48 8d 05 30 55 af fe        	lea	rax, [rip - 0x150aad0]  # 0x140109e90 <.text+0x108e90>
141614960: 48 89 45 70                 	mov	qword ptr [rbp + 0x70], rax
141614964: 48 8d 05 5d ae 66 01        	lea	rax, [rip + 0x166ae5d]  # 0x142c7f7c8
14161496b: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
14161496f: 48 c7 45 18 01 00 00 00     	mov	qword ptr [rbp + 0x18], 0x1
141614977: 48 c7 45 30 00 00 00 00     	mov	qword ptr [rbp + 0x30], 0x0
14161497f: 48 8d 45 68                 	lea	rax, [rbp + 0x68]
141614983: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
141614987: 48 c7 45 28 01 00 00 00     	mov	qword ptr [rbp + 0x28], 0x1
14161498f: 48 8d 15 42 ae 66 01        	lea	rdx, [rip + 0x166ae42]  # 0x142c7f7d8
141614996: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14161499a: e8 f1 49 2c 01              	call	0x1428d9390 <.text+0x28d8390>
14161499f: 45 31 f6                    	xor	r14d, r14d
1416149a2: 4c 8d 7d 10                 	lea	r15, [rbp + 0x10]
1416149a6: ba 08 00 00 00              	mov	edx, 0x8
1416149ab: 4c 89 f9                    	mov	rcx, r15
1416149ae: 41 b8 22 01 00 00           	mov	r8d, 0x122
1416149b4: 41 b1 01                    	mov	r9b, 0x1
1416149b7: e8 24 04 5c ff              	call	0x140bd4de0 <.text+0xbd3de0>
1416149bc: 4c 8d 65 68                 	lea	r12, [rbp + 0x68]
1416149c0: 41 b8 04 00 00 00           	mov	r8d, 0x4
1416149c6: 4c 89 e1                    	mov	rcx, r12
1416149c9: 4c 89 fa                    	mov	rdx, r15
1416149cc: e8 4f 05 5c ff              	call	0x140bd4f20 <.text+0xbd3f20>
1416149d1: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
1416149d6: 48 89 5c 24 28              	mov	qword ptr [rsp + 0x28], rbx
1416149db: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
1416149e0: 4c 8d 05 fd b5 66 01        	lea	r8, [rip + 0x166b5fd]   # 0x142c7ffe4
1416149e7: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1416149eb: 41 b9 13 00 00 00           	mov	r9d, 0x13
1416149f1: 48 89 fa                    	mov	rdx, rdi
1416149f4: e8 c7 e3 08 00              	call	0x1416a2dc0 <.text+0x16a1dc0>
1416149f9: 83 7d c0 0b                 	cmp	dword ptr [rbp - 0x40], 0xb
1416149fd: 75 67                       	jne	0x141614a66 <.text+0x1613a66>
1416149ff: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
141614a03: 48 89 46 20                 	mov	qword ptr [rsi + 0x20], rax
141614a07: 0f 10 45 c8                 	movups	xmm0, xmmword ptr [rbp - 0x38]
141614a0b: 0f 10 4d d8                 	movups	xmm1, xmmword ptr [rbp - 0x28]
141614a0f: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
141614a13: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
141614a16: 48 89 f0                    	mov	rax, rsi
141614a19: 48 81 c4 00 01 00 00        	add	rsp, 0x100
141614a20: 5b                          	pop	rbx
141614a21: 5f                          	pop	rdi
141614a22: 5e                          	pop	rsi
141614a23: 41 5c                       	pop	r12
141614a25: 41 5e                       	pop	r14
141614a27: 41 5f                       	pop	r15
141614a29: 5d                          	pop	rbp
141614a2a: c3                          	ret
141614a2b: 48 c7 45 10 00 00 00 00     	mov	qword ptr [rbp + 0x10], 0x0
141614a33: 48 8d 05 0e 55 6b 01        	lea	rax, [rip + 0x16b550e]  # 0x142cc9f48
141614a3a: eb 0f                       	jmp	0x141614a4b <.text+0x1613a4b>
141614a3c: 48 c7 45 10 00 00 00 00     	mov	qword ptr [rbp + 0x10], 0x0
141614a44: 48 8d 05 15 55 6b 01        	lea	rax, [rip + 0x16b5515]  # 0x142cc9f60
141614a4b: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141614a50: 4c 8d 05 e9 54 6b 01        	lea	r8, [rip + 0x16b54e9]   # 0x142cc9f40
141614a57: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
141614a5b: 4c 8d 4d 10                 	lea	r9, [rbp + 0x10]
141614a5f: 31 c9                       	xor	ecx, ecx
141614a61: e8 30 4b 2c 01              	call	0x1428d9596 <.text+0x28d8596>
141614a66: 0f 10 45 00                 	movups	xmm0, xmmword ptr [rbp]
141614a6a: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
141614a6e: 0f 10 45 c0                 	movups	xmm0, xmmword ptr [rbp - 0x40]
141614a72: 0f 10 4d d0                 	movups	xmm1, xmmword ptr [rbp - 0x30]
141614a76: 0f 10 55 e0                 	movups	xmm2, xmmword ptr [rbp - 0x20]
141614a7a: 0f 10 5d f0                 	movups	xmm3, xmmword ptr [rbp - 0x10]
141614a7e: 0f 29 5d 40                 	movaps	xmmword ptr [rbp + 0x40], xmm3
141614a82: 0f 29 55 30                 	movaps	xmmword ptr [rbp + 0x30], xmm2
141614a86: 0f 29 4d 20                 	movaps	xmmword ptr [rbp + 0x20], xmm1
141614a8a: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
141614a8e: 48 8d 05 13 b7 66 01        	lea	rax, [rip + 0x166b713]  # 0x142c801a8
141614a95: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141614a9a: 48 8d 0d 47 a0 66 01        	lea	rcx, [rip + 0x166a047]  # 0x142c7eae8
141614aa1: 4c 8d 0d 20 a0 66 01        	lea	r9, [rip + 0x166a020]   # 0x142c7eac8
141614aa8: 4c 8d 45 10                 	lea	r8, [rbp + 0x10]
141614aac: ba 2b 00 00 00              	mov	edx, 0x2b
141614ab1: e8 aa 4c 2c 01              	call	0x1428d9760 <.text+0x28d8760>
141614ab6: 0f 0b                       	ud2
141614ab8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
141614ac0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141614ac5: 55                          	push	rbp
141614ac6: 41 57                       	push	r15
141614ac8: 41 56                       	push	r14
141614aca: 41 54                       	push	r12
141614acc: 56                          	push	rsi
141614acd: 57                          	push	rdi
141614ace: 53                          	push	rbx
141614acf: 48 83 ec 40                 	sub	rsp, 0x40
141614ad3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141614ada: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
141614ade: e8 8d 9f 1b ff              	call	0x1407cea70 <.text+0x7cda70>
141614ae3: 90                          	nop
141614ae4: 48 83 c4 40                 	add	rsp, 0x40
141614ae8: 5b                          	pop	rbx
141614ae9: 5f                          	pop	rdi
141614aea: 5e                          	pop	rsi
141614aeb: 41 5c                       	pop	r12
141614aed: 41 5e                       	pop	r14
141614aef: 41 5f                       	pop	r15
141614af1: 5d                          	pop	rbp
141614af2: c3                          	ret

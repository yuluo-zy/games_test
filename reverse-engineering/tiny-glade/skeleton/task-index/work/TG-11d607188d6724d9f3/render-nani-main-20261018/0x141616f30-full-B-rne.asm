
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141616f30: 55                          	push	rbp
141616f31: 41 57                       	push	r15
141616f33: 41 56                       	push	r14
141616f35: 41 54                       	push	r12
141616f37: 56                          	push	rsi
141616f38: 57                          	push	rdi
141616f39: 53                          	push	rbx
141616f3a: 48 81 ec 00 01 00 00        	sub	rsp, 0x100
141616f41: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
141616f49: 48 c7 45 78 fe ff ff ff     	mov	qword ptr [rbp + 0x78], -0x2
141616f51: 4d 85 c0                    	test	r8, r8
141616f54: 0f 84 9b 00 00 00           	je	0x141616ff5 <.text+0x1615ff5>
141616f5a: 4d 89 ce                    	mov	r14, r9
141616f5d: 4c 89 c3                    	mov	rbx, r8
141616f60: 48 89 d7                    	mov	rdi, rdx
141616f63: 48 89 ce                    	mov	rsi, rcx
141616f66: 49 c1 e6 04                 	shl	r14, 0x4
141616f6a: 4c 8d 7d c0                 	lea	r15, [rbp - 0x40]
141616f6e: 4c 89 f9                    	mov	rcx, r15
141616f71: 4c 89 f2                    	mov	rdx, r14
141616f74: 41 b8 22 00 02 00           	mov	r8d, 0x20022
141616f7a: 41 b1 01                    	mov	r9b, 0x1
141616f7d: e8 5e de 5b ff              	call	0x140bd4de0 <.text+0xbd3de0>
141616f82: 4c 8d 65 68                 	lea	r12, [rbp + 0x68]
141616f86: 41 b8 04 00 00 00           	mov	r8d, 0x4
141616f8c: 4c 89 e1                    	mov	rcx, r12
141616f8f: 4c 89 fa                    	mov	rdx, r15
141616f92: e8 89 df 5b ff              	call	0x140bd4f20 <.text+0xbd3f20>
141616f97: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
141616f9c: 48 89 5c 24 28              	mov	qword ptr [rsp + 0x28], rbx
141616fa1: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
141616fa6: 4c 8d 05 5c 8a 66 01        	lea	r8, [rip + 0x1668a5c]   # 0x142c7fa09
141616fad: 48 8d 4d 18                 	lea	rcx, [rbp + 0x18]
141616fb1: 41 b9 0b 00 00 00           	mov	r9d, 0xb
141616fb7: 48 89 fa                    	mov	rdx, rdi
141616fba: e8 01 be 08 00              	call	0x1416a2dc0 <.text+0x16a1dc0>
141616fbf: 83 7d 18 0b                 	cmp	dword ptr [rbp + 0x18], 0xb
141616fc3: 0f 85 a3 00 00 00           	jne	0x14161706c <.text+0x161606c>
141616fc9: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
141616fcd: 48 89 46 20                 	mov	qword ptr [rsi + 0x20], rax
141616fd1: 0f 10 45 20                 	movups	xmm0, xmmword ptr [rbp + 0x20]
141616fd5: 0f 10 4d 30                 	movups	xmm1, xmmword ptr [rbp + 0x30]
141616fd9: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
141616fdd: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
141616fe0: 48 89 f0                    	mov	rax, rsi
141616fe3: 48 81 c4 00 01 00 00        	add	rsp, 0x100
141616fea: 5b                          	pop	rbx
141616feb: 5f                          	pop	rdi
141616fec: 5e                          	pop	rsi
141616fed: 41 5c                       	pop	r12
141616fef: 41 5e                       	pop	r14
141616ff1: 41 5f                       	pop	r15
141616ff3: 5d                          	pop	rbp
141616ff4: c3                          	ret
141616ff5: 48 8d 05 55 73 48 01        	lea	rax, [rip + 0x1487355]  # 0x142a9e351
141616ffc: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
141617000: 48 c7 45 70 30 00 00 00     	mov	qword ptr [rbp + 0x70], 0x30
141617008: 48 8d 05 09 8a 66 01        	lea	rax, [rip + 0x1668a09]  # 0x142c7fa18
14161700f: 48 89 45 18                 	mov	qword ptr [rbp + 0x18], rax
141617013: 48 8d 05 86 c8 f1 fe        	lea	rax, [rip - 0x10e377a]  # 0x1405338a0 <.text+0x5328a0>
14161701a: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
14161701e: 48 8d 45 68                 	lea	rax, [rbp + 0x68]
141617022: 48 89 45 28                 	mov	qword ptr [rbp + 0x28], rax
141617026: 48 8d 05 63 2e af fe        	lea	rax, [rip - 0x150d19d]  # 0x140109e90 <.text+0x108e90>
14161702d: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
141617031: 48 8d 05 f8 86 66 01        	lea	rax, [rip + 0x16686f8]  # 0x142c7f730
141617038: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
14161703c: 48 c7 45 c8 03 00 00 00     	mov	qword ptr [rbp - 0x38], 0x3
141617044: 48 c7 45 e0 00 00 00 00     	mov	qword ptr [rbp - 0x20], 0x0
14161704c: 48 8d 45 18                 	lea	rax, [rbp + 0x18]
141617050: 48 89 45 d0                 	mov	qword ptr [rbp - 0x30], rax
141617054: 48 c7 45 d8 02 00 00 00     	mov	qword ptr [rbp - 0x28], 0x2
14161705c: 48 8d 15 25 87 66 01        	lea	rdx, [rip + 0x1668725]  # 0x142c7f788
141617063: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
141617067: e8 24 23 2c 01              	call	0x1428d9390 <.text+0x28d8390>
14161706c: 0f 10 45 58                 	movups	xmm0, xmmword ptr [rbp + 0x58]
141617070: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
141617074: 0f 10 45 18                 	movups	xmm0, xmmword ptr [rbp + 0x18]
141617078: 0f 10 4d 28                 	movups	xmm1, xmmword ptr [rbp + 0x28]
14161707c: 0f 10 55 38                 	movups	xmm2, xmmword ptr [rbp + 0x38]
141617080: 0f 10 5d 48                 	movups	xmm3, xmmword ptr [rbp + 0x48]
141617084: 0f 29 5d f0                 	movaps	xmmword ptr [rbp - 0x10], xmm3
141617088: 0f 29 55 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm2
14161708c: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
141617090: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
141617094: 48 8d 05 0d 91 66 01        	lea	rax, [rip + 0x166910d]  # 0x142c801a8
14161709b: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1416170a0: 48 8d 0d 41 7a 66 01        	lea	rcx, [rip + 0x1667a41]  # 0x142c7eae8
1416170a7: 4c 8d 0d 1a 7a 66 01        	lea	r9, [rip + 0x1667a1a]   # 0x142c7eac8
1416170ae: 4c 8d 45 c0                 	lea	r8, [rbp - 0x40]
1416170b2: ba 2b 00 00 00              	mov	edx, 0x2b
1416170b7: e8 a4 26 2c 01              	call	0x1428d9760 <.text+0x28d8760>
1416170bc: 0f 0b                       	ud2
1416170be: 66 90                       	nop
1416170c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1416170c5: 55                          	push	rbp
1416170c6: 41 57                       	push	r15
1416170c8: 41 56                       	push	r14
1416170ca: 41 54                       	push	r12
1416170cc: 56                          	push	rsi
1416170cd: 57                          	push	rdi
1416170ce: 53                          	push	rbx
1416170cf: 48 83 ec 40                 	sub	rsp, 0x40
1416170d3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1416170da: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1416170de: e8 8d 79 1b ff              	call	0x1407cea70 <.text+0x7cda70>
1416170e3: 90                          	nop
1416170e4: 48 83 c4 40                 	add	rsp, 0x40
1416170e8: 5b                          	pop	rbx
1416170e9: 5f                          	pop	rdi
1416170ea: 5e                          	pop	rsi
1416170eb: 41 5c                       	pop	r12
1416170ed: 41 5e                       	pop	r14
1416170ef: 41 5f                       	pop	r15
1416170f1: 5d                          	pop	rbp
1416170f2: c3                          	ret


D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142339f10: 55                          	push	rbp
142339f11: 41 56                       	push	r14
142339f13: 56                          	push	rsi
142339f14: 57                          	push	rdi
142339f15: 53                          	push	rbx
142339f16: 48 81 ec b0 01 00 00        	sub	rsp, 0x1b0
142339f1d: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
142339f25: 0f 29 bd 20 01 00 00        	movaps	xmmword ptr [rbp + 0x120], xmm7
142339f2c: 0f 29 b5 10 01 00 00        	movaps	xmmword ptr [rbp + 0x110], xmm6
142339f33: 48 c7 85 08 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x108], -0x2
142339f3e: 4c 89 cb                    	mov	rbx, r9
142339f41: 4d 89 c6                    	mov	r14, r8
142339f44: 48 89 d7                    	mov	rdi, rdx
142339f47: 48 89 ce                    	mov	rsi, rcx
142339f4a: 0f b6 05 b8 ac 2f 01        	movzx	eax, byte ptr [rip + 0x12facb8] # 0x143634c09
142339f51: b9 08 00 00 00              	mov	ecx, 0x8
142339f56: ba 08 00 00 00              	mov	edx, 0x8
142339f5b: e8 b0 9c 2d fe              	call	0x140613c10 <.text+0x612c10>
142339f60: 48 89 85 00 01 00 00        	mov	qword ptr [rbp + 0x100], rax
142339f67: 48 85 c0                    	test	rax, rax
142339f6a: 0f 84 84 01 00 00           	je	0x14233a0f4 <.text+0x23390f4>
142339f70: 48 b8 01 00 00 00 00 00 00 80       	movabs	rax, -0x7fffffffffffffff
142339f7a: 48 89 45 b0                 	mov	qword ptr [rbp - 0x50], rax
142339f7e: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
142339f82: 48 8d 55 b0                 	lea	rdx, [rbp - 0x50]
142339f86: e8 25 3a 56 fe              	call	0x14089d9b0 <.text+0x89c9b0>
142339f8b: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
142339f8f: 48 8d 55 40                 	lea	rdx, [rbp + 0x40]
142339f93: 0f 57 d2                    	xorps	xmm2, xmm2
142339f96: e8 f5 3c 56 fe              	call	0x14089dc90 <.text+0x89cc90>
142339f9b: 48 8b 85 80 01 00 00        	mov	rax, qword ptr [rbp + 0x180]
142339fa2: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142339fa7: 48 8d 8d d8 00 00 00        	lea	rcx, [rbp + 0xd8]
142339fae: 48 8d 55 b0                 	lea	rdx, [rbp - 0x50]
142339fb2: 4d 89 f0                    	mov	r8, r14
142339fb5: 49 89 d9                    	mov	r9, rbx
142339fb8: e8 93 3d 56 fe              	call	0x14089dd50 <.text+0x89cd50>
142339fbd: f3 0f 10 37                 	movss	xmm6, dword ptr [rdi]
142339fc1: f2 0f 10 7f 04              	movsd	xmm7, qword ptr [rdi + 0x4]
142339fc6: 0f b6 05 3c ac 2f 01        	movzx	eax, byte ptr [rip + 0x12fac3c] # 0x143634c09
142339fcd: b9 0a 00 00 00              	mov	ecx, 0xa
142339fd2: ba 01 00 00 00              	mov	edx, 0x1
142339fd7: e8 34 9c 2d fe              	call	0x140613c10 <.text+0x612c10>
142339fdc: 48 85 c0                    	test	rax, rax
142339fdf: 0f 84 1e 01 00 00           	je	0x14233a103 <.text+0x2339103>
142339fe5: 48 b9 69 63 6f 6e 5f 63 72 6f       	movabs	rcx, 0x6f72635f6e6f6369
142339fef: 48 89 08                    	mov	qword ptr [rax], rcx
142339ff2: 66 c7 40 08 73 73           	mov	word ptr [rax + 0x8], 0x7373
142339ff8: 0f 16 f7                    	movlhps	xmm6, xmm7              # xmm6 = xmm6[0],xmm7[0]
142339ffb: 0f c6 f7 58                 	shufps	xmm6, xmm7, 0x58        # xmm6 = xmm6[0,2],xmm7[1,1]
142339fff: 0f 29 75 40                 	movaps	xmmword ptr [rbp + 0x40], xmm6
14233a003: 48 c7 45 50 0a 00 00 00     	mov	qword ptr [rbp + 0x50], 0xa
14233a00b: 48 89 45 58                 	mov	qword ptr [rbp + 0x58], rax
14233a00f: 48 c7 45 60 0a 00 00 00     	mov	qword ptr [rbp + 0x60], 0xa
14233a017: c7 45 68 00 00 00 00        	mov	dword ptr [rbp + 0x68], 0x0
14233a01e: 4c 8d 05 4b 3d d1 00        	lea	r8, [rip + 0xd13d4b]    # 0x14304dd70
14233a025: 48 8d 8d d8 00 00 00        	lea	rcx, [rbp + 0xd8]
14233a02c: 48 8d 55 40                 	lea	rdx, [rbp + 0x40]
14233a030: e8 eb 5c ca fe              	call	0x140fdfd20 <.text+0xfded20>
14233a035: 48 8b bd e0 00 00 00        	mov	rdi, qword ptr [rbp + 0xe0]
14233a03c: 48 85 ff                    	test	rdi, rdi
14233a03f: 75 07                       	jne	0x14233a048 <.text+0x2339048>
14233a041: 48 8b bd e8 00 00 00        	mov	rdi, qword ptr [rbp + 0xe8]
14233a048: 4c 8b b5 f8 00 00 00        	mov	r14, qword ptr [rbp + 0xf8]
14233a04f: 48 8b 07                    	mov	rax, qword ptr [rdi]
14233a052: 48 8b 5f 10                 	mov	rbx, qword ptr [rdi + 0x10]
14233a056: 48 29 d8                    	sub	rax, rbx
14233a059: 48 83 f8 1f                 	cmp	rax, 0x1f
14233a05d: 76 70                       	jbe	0x14233a0cf <.text+0x23390cf>
14233a05f: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
14233a063: 48 8d 0d 16 53 ca fe        	lea	rcx, [rip - 0x135acea]  # 0x140fdf380 <.text+0xfde380>
14233a06a: 48 89 0c 18                 	mov	qword ptr [rax + rbx], rcx
14233a06e: 48 8d 0d eb 57 b1 ff        	lea	rcx, [rip - 0x4ea815]   # 0x141e4f860 <.text+0x1e4e860>
14233a075: 48 89 4c 18 08              	mov	qword ptr [rax + rbx + 0x8], rcx
14233a07a: 4c 89 74 18 10              	mov	qword ptr [rax + rbx + 0x10], r14
14233a07f: c6 44 18 18 00              	mov	byte ptr [rax + rbx + 0x18], 0x0
14233a084: 48 83 c3 20                 	add	rbx, 0x20
14233a088: 48 89 5f 10                 	mov	qword ptr [rdi + 0x10], rbx
14233a08c: 48 8b 85 f8 00 00 00        	mov	rax, qword ptr [rbp + 0xf8]
14233a093: 48 8b 8d 00 01 00 00        	mov	rcx, qword ptr [rbp + 0x100]
14233a09a: 48 89 01                    	mov	qword ptr [rcx], rax
14233a09d: 48 c7 06 01 00 00 00        	mov	qword ptr [rsi], 0x1
14233a0a4: 48 89 4e 08                 	mov	qword ptr [rsi + 0x8], rcx
14233a0a8: 48 c7 46 10 01 00 00 00     	mov	qword ptr [rsi + 0x10], 0x1
14233a0b0: 48 89 f0                    	mov	rax, rsi
14233a0b3: 0f 28 b5 10 01 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x110]
14233a0ba: 0f 28 bd 20 01 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x120]
14233a0c1: 48 81 c4 b0 01 00 00        	add	rsp, 0x1b0
14233a0c8: 5b                          	pop	rbx
14233a0c9: 5f                          	pop	rdi
14233a0ca: 5e                          	pop	rsi
14233a0cb: 41 5e                       	pop	r14
14233a0cd: 5d                          	pop	rbp
14233a0ce: c3                          	ret
14233a0cf: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
14233a0d8: 41 b8 20 00 00 00           	mov	r8d, 0x20
14233a0de: 41 b9 01 00 00 00           	mov	r9d, 0x1
14233a0e4: 48 89 f9                    	mov	rcx, rdi
14233a0e7: 48 89 da                    	mov	rdx, rbx
14233a0ea: e8 e1 b7 56 00              	call	0x1428a58d0 <.text+0x28a48d0>
14233a0ef: e9 6b ff ff ff              	jmp	0x14233a05f <.text+0x233905f>
14233a0f4: b9 08 00 00 00              	mov	ecx, 0x8
14233a0f9: ba 08 00 00 00              	mov	edx, 0x8
14233a0fe: e8 fd ee 59 00              	call	0x1428d9000 <.text+0x28d8000>
14233a103: 4c 8d 05 1e 27 c5 00        	lea	r8, [rip + 0xc5271e]    # 0x142f8c828
14233a10a: b9 01 00 00 00              	mov	ecx, 0x1
14233a10f: ba 0a 00 00 00              	mov	edx, 0xa
14233a114: e8 ca ee 59 00              	call	0x1428d8fe3 <.text+0x28d7fe3>
14233a119: 0f 0b                       	ud2
14233a11b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
14233a120: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14233a125: 55                          	push	rbp
14233a126: 41 56                       	push	r14
14233a128: 56                          	push	rsi
14233a129: 57                          	push	rdi
14233a12a: 53                          	push	rbx
14233a12b: 48 83 ec 50                 	sub	rsp, 0x50
14233a12f: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14233a136: 0f 29 7c 24 20              	movaps	xmmword ptr [rsp + 0x20], xmm7
14233a13b: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
14233a140: ba 08 00 00 00              	mov	edx, 0x8
14233a145: 41 b8 08 00 00 00           	mov	r8d, 0x8
14233a14b: 48 8b 8d 00 01 00 00        	mov	rcx, qword ptr [rbp + 0x100]
14233a152: e8 c9 9a 2d fe              	call	0x140613c20 <.text+0x612c20>
14233a157: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
14233a15c: 0f 28 7c 24 20              	movaps	xmm7, xmmword ptr [rsp + 0x20]
14233a161: 48 83 c4 50                 	add	rsp, 0x50
14233a165: 5b                          	pop	rbx
14233a166: 5f                          	pop	rdi
14233a167: 5e                          	pop	rsi
14233a168: 41 5e                       	pop	r14
14233a16a: 5d                          	pop	rbp
14233a16b: c3                          	ret

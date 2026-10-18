
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141610820: 55                          	push	rbp
141610821: 41 57                       	push	r15
141610823: 41 56                       	push	r14
141610825: 41 54                       	push	r12
141610827: 56                          	push	rsi
141610828: 57                          	push	rdi
141610829: 53                          	push	rbx
14161082a: 48 81 ec 20 01 00 00        	sub	rsp, 0x120
141610831: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
141610839: 48 c7 85 98 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x98], -0x2
141610844: 48 89 d7                    	mov	rdi, rdx
141610847: 48 89 ce                    	mov	rsi, rcx
14161084a: 4d 85 c0                    	test	r8, r8
14161084d: 74 37                       	je	0x141610886 <.text+0x160f886>
14161084f: 4c 89 cb                    	mov	rbx, r9
141610852: 4c 89 c8                    	mov	rax, r9
141610855: 48 83 e0 0f                 	and	rax, 0xf
141610859: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
14161085d: 0f 85 1c 01 00 00           	jne	0x14161097f <.text+0x160f97f>
141610863: 4d 89 c7                    	mov	r15, r8
141610866: 4c 89 c0                    	mov	rax, r8
141610869: 48 83 e0 03                 	and	rax, 0x3
14161086d: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
141610871: 0f 85 19 01 00 00           	jne	0x141610990 <.text+0x160f990>
141610877: 48 83 fb 20                 	cmp	rbx, 0x20
14161087b: 0f 83 99 01 00 00           	jae	0x141610a1a <.text+0x160fa1a>
141610881: 45 31 e4                    	xor	r12d, r12d
141610884: eb 30                       	jmp	0x1416108b6 <.text+0x160f8b6>
141610886: 0f b6 05 7c 43 02 02        	movzx	eax, byte ptr [rip + 0x202437c] # 0x143634c09
14161088d: bb 10 00 00 00              	mov	ebx, 0x10
141610892: b9 10 00 00 00              	mov	ecx, 0x10
141610897: ba 04 00 00 00              	mov	edx, 0x4
14161089c: e8 6f 33 00 ff              	call	0x140613c10 <.text+0x612c10>
1416108a1: 48 85 c0                    	test	rax, rax
1416108a4: 0f 84 da 01 00 00           	je	0x141610a84 <.text+0x160fa84>
1416108aa: 49 89 c7                    	mov	r15, rax
1416108ad: 0f 57 c0                    	xorps	xmm0, xmm0
1416108b0: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
1416108b3: 41 b4 01                    	mov	r12b, 0x1
1416108b6: 4c 89 bd 88 00 00 00        	mov	qword ptr [rbp + 0x88], r15
1416108bd: 44 88 a5 97 00 00 00        	mov	byte ptr [rbp + 0x97], r12b
1416108c4: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
1416108c8: ba 10 00 00 00              	mov	edx, 0x10
1416108cd: 41 b8 22 01 00 00           	mov	r8d, 0x122
1416108d3: 41 b1 01                    	mov	r9b, 0x1
1416108d6: e8 05 45 5c ff              	call	0x140bd4de0 <.text+0xbd3de0>
1416108db: 4c 89 bd 88 00 00 00        	mov	qword ptr [rbp + 0x88], r15
1416108e2: 44 88 a5 97 00 00 00        	mov	byte ptr [rbp + 0x97], r12b
1416108e9: 4c 8d 75 68                 	lea	r14, [rbp + 0x68]
1416108ed: 48 8d 55 10                 	lea	rdx, [rbp + 0x10]
1416108f1: 41 b8 04 00 00 00           	mov	r8d, 0x4
1416108f7: 4c 89 f1                    	mov	rcx, r14
1416108fa: e8 21 46 5c ff              	call	0x140bd4f20 <.text+0xbd3f20>
1416108ff: 4c 89 bd 88 00 00 00        	mov	qword ptr [rbp + 0x88], r15
141610906: 44 88 a5 97 00 00 00        	mov	byte ptr [rbp + 0x97], r12b
14161090d: 48 89 5c 24 30              	mov	qword ptr [rsp + 0x30], rbx
141610912: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
141610917: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
14161091c: 4c 8d 05 9e f3 66 01        	lea	r8, [rip + 0x166f39e]   # 0x142c7fcc1
141610923: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
141610927: 41 b9 12 00 00 00           	mov	r9d, 0x12
14161092d: 48 89 fa                    	mov	rdx, rdi
141610930: e8 8b 24 09 00              	call	0x1416a2dc0 <.text+0x16a1dc0>
141610935: 83 7d c0 0b                 	cmp	dword ptr [rbp - 0x40], 0xb
141610939: 75 7f                       	jne	0x1416109ba <.text+0x160f9ba>
14161093b: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
14161093f: 48 89 46 20                 	mov	qword ptr [rsi + 0x20], rax
141610943: 0f 10 45 c8                 	movups	xmm0, xmmword ptr [rbp - 0x38]
141610947: 0f 10 4d d8                 	movups	xmm1, xmmword ptr [rbp - 0x28]
14161094b: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
14161094f: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
141610952: 45 84 e4                    	test	r12b, r12b
141610955: 74 13                       	je	0x14161096a <.text+0x160f96a>
141610957: ba 10 00 00 00              	mov	edx, 0x10
14161095c: 41 b8 04 00 00 00           	mov	r8d, 0x4
141610962: 4c 89 f9                    	mov	rcx, r15
141610965: e8 b6 32 00 ff              	call	0x140613c20 <.text+0x612c20>
14161096a: 48 89 f0                    	mov	rax, rsi
14161096d: 48 81 c4 20 01 00 00        	add	rsp, 0x120
141610974: 5b                          	pop	rbx
141610975: 5f                          	pop	rdi
141610976: 5e                          	pop	rsi
141610977: 41 5c                       	pop	r12
141610979: 41 5e                       	pop	r14
14161097b: 41 5f                       	pop	r15
14161097d: 5d                          	pop	rbp
14161097e: c3                          	ret
14161097f: 48 c7 45 10 00 00 00 00     	mov	qword ptr [rbp + 0x10], 0x0
141610987: 48 8d 05 ba 95 6b 01        	lea	rax, [rip + 0x16b95ba]  # 0x142cc9f48
14161098e: eb 0f                       	jmp	0x14161099f <.text+0x160f99f>
141610990: 48 c7 45 10 00 00 00 00     	mov	qword ptr [rbp + 0x10], 0x0
141610998: 48 8d 05 c1 95 6b 01        	lea	rax, [rip + 0x16b95c1]  # 0x142cc9f60
14161099f: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1416109a4: 4c 8d 05 95 95 6b 01        	lea	r8, [rip + 0x16b9595]   # 0x142cc9f40
1416109ab: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
1416109af: 4c 8d 4d 10                 	lea	r9, [rbp + 0x10]
1416109b3: 31 c9                       	xor	ecx, ecx
1416109b5: e8 dc 8b 2c 01              	call	0x1428d9596 <.text+0x28d8596>
1416109ba: 44 89 a5 84 00 00 00        	mov	dword ptr [rbp + 0x84], r12d
1416109c1: 4c 89 7d 78                 	mov	qword ptr [rbp + 0x78], r15
1416109c5: 0f 10 45 00                 	movups	xmm0, xmmword ptr [rbp]
1416109c9: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
1416109cd: 0f 10 45 c0                 	movups	xmm0, xmmword ptr [rbp - 0x40]
1416109d1: 0f 10 4d d0                 	movups	xmm1, xmmword ptr [rbp - 0x30]
1416109d5: 0f 10 55 e0                 	movups	xmm2, xmmword ptr [rbp - 0x20]
1416109d9: 0f 10 5d f0                 	movups	xmm3, xmmword ptr [rbp - 0x10]
1416109dd: 0f 29 5d 40                 	movaps	xmmword ptr [rbp + 0x40], xmm3
1416109e1: 0f 29 55 30                 	movaps	xmmword ptr [rbp + 0x30], xmm2
1416109e5: 0f 29 4d 20                 	movaps	xmmword ptr [rbp + 0x20], xmm1
1416109e9: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
1416109ed: 48 8d 05 b4 f7 66 01        	lea	rax, [rip + 0x166f7b4]  # 0x142c801a8
1416109f4: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1416109f9: 48 8d 0d e8 e0 66 01        	lea	rcx, [rip + 0x166e0e8]  # 0x142c7eae8
141610a00: 4c 8d 0d c1 e0 66 01        	lea	r9, [rip + 0x166e0c1]   # 0x142c7eac8
141610a07: 4c 8d 45 10                 	lea	r8, [rbp + 0x10]
141610a0b: ba 2b 00 00 00              	mov	edx, 0x2b
141610a10: e8 4b 8d 2c 01              	call	0x1428d9760 <.text+0x28d8760>
141610a15: e9 87 00 00 00              	jmp	0x141610aa1 <.text+0x160faa1>
141610a1a: 48 8d 05 41 d7 48 01        	lea	rax, [rip + 0x148d741]  # 0x142a9e162
141610a21: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
141610a25: 48 c7 45 c8 47 00 00 00     	mov	qword ptr [rbp - 0x38], 0x47
141610a2d: 48 8d 45 c0                 	lea	rax, [rbp - 0x40]
141610a31: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
141610a35: 48 8d 05 54 94 af fe        	lea	rax, [rip - 0x1506bac]  # 0x140109e90 <.text+0x108e90>
141610a3c: 48 89 45 70                 	mov	qword ptr [rbp + 0x70], rax
141610a40: 48 8d 05 81 ed 66 01        	lea	rax, [rip + 0x166ed81]  # 0x142c7f7c8
141610a47: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
141610a4b: 48 c7 45 18 01 00 00 00     	mov	qword ptr [rbp + 0x18], 0x1
141610a53: 48 c7 45 30 00 00 00 00     	mov	qword ptr [rbp + 0x30], 0x0
141610a5b: 48 8d 45 68                 	lea	rax, [rbp + 0x68]
141610a5f: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
141610a63: 48 c7 45 28 01 00 00 00     	mov	qword ptr [rbp + 0x28], 0x1
141610a6b: c6 85 97 00 00 00 00        	mov	byte ptr [rbp + 0x97], 0x0
141610a72: 48 8d 15 5f ed 66 01        	lea	rdx, [rip + 0x166ed5f]  # 0x142c7f7d8
141610a79: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
141610a7d: e8 0e 89 2c 01              	call	0x1428d9390 <.text+0x28d8390>
141610a82: eb 1d                       	jmp	0x141610aa1 <.text+0x160faa1>
141610a84: c6 85 97 00 00 00 00        	mov	byte ptr [rbp + 0x97], 0x0
141610a8b: 4c 8d 05 1e e0 66 01        	lea	r8, [rip + 0x166e01e]   # 0x142c7eab0
141610a92: b9 04 00 00 00              	mov	ecx, 0x4
141610a97: ba 10 00 00 00              	mov	edx, 0x10
141610a9c: e8 42 85 2c 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
141610aa1: 0f 0b                       	ud2
141610aa3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
141610ab0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141610ab5: 55                          	push	rbp
141610ab6: 41 57                       	push	r15
141610ab8: 41 56                       	push	r14
141610aba: 41 54                       	push	r12
141610abc: 56                          	push	rsi
141610abd: 57                          	push	rdi
141610abe: 53                          	push	rbx
141610abf: 48 83 ec 40                 	sub	rsp, 0x40
141610ac3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141610aca: 48 8b 75 78                 	mov	rsi, qword ptr [rbp + 0x78]
141610ace: 48 89 b5 88 00 00 00        	mov	qword ptr [rbp + 0x88], rsi
141610ad5: 8b bd 84 00 00 00           	mov	edi, dword ptr [rbp + 0x84]
141610adb: 40 88 bd 97 00 00 00        	mov	byte ptr [rbp + 0x97], dil
141610ae2: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
141610ae6: e8 85 df 1b ff              	call	0x1407cea70 <.text+0x7cda70>
141610aeb: 48 89 b5 88 00 00 00        	mov	qword ptr [rbp + 0x88], rsi
141610af2: 40 88 bd 97 00 00 00        	mov	byte ptr [rbp + 0x97], dil
141610af9: 48 83 c4 40                 	add	rsp, 0x40
141610afd: 5b                          	pop	rbx
141610afe: 5f                          	pop	rdi
141610aff: 5e                          	pop	rsi
141610b00: 41 5c                       	pop	r12
141610b02: 41 5e                       	pop	r14
141610b04: 41 5f                       	pop	r15
141610b06: 5d                          	pop	rbp
141610b07: c3                          	ret
141610b08: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
141610b10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141610b15: 55                          	push	rbp
141610b16: 41 57                       	push	r15
141610b18: 41 56                       	push	r14
141610b1a: 41 54                       	push	r12
141610b1c: 56                          	push	rsi
141610b1d: 57                          	push	rdi
141610b1e: 53                          	push	rbx
141610b1f: 48 83 ec 40                 	sub	rsp, 0x40
141610b23: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141610b2a: f6 85 97 00 00 00 01        	test	byte ptr [rbp + 0x97], 0x1
141610b31: 74 17                       	je	0x141610b4a <.text+0x160fb4a>
141610b33: 48 8b 8d 88 00 00 00        	mov	rcx, qword ptr [rbp + 0x88]
141610b3a: ba 10 00 00 00              	mov	edx, 0x10
141610b3f: 41 b8 04 00 00 00           	mov	r8d, 0x4
141610b45: e8 d6 30 00 ff              	call	0x140613c20 <.text+0x612c20>
141610b4a: 90                          	nop
141610b4b: 48 83 c4 40                 	add	rsp, 0x40
141610b4f: 5b                          	pop	rbx
141610b50: 5f                          	pop	rdi
141610b51: 5e                          	pop	rsi
141610b52: 41 5c                       	pop	r12
141610b54: 41 5e                       	pop	r14
141610b56: 41 5f                       	pop	r15
141610b58: 5d                          	pop	rbp
141610b59: c3                          	ret

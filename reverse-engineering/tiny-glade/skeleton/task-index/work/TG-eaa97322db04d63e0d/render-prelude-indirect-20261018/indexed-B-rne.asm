
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141614d30: 55                          	push	rbp
141614d31: 41 57                       	push	r15
141614d33: 41 56                       	push	r14
141614d35: 41 54                       	push	r12
141614d37: 56                          	push	rsi
141614d38: 57                          	push	rdi
141614d39: 53                          	push	rbx
141614d3a: 48 81 ec 20 01 00 00        	sub	rsp, 0x120
141614d41: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
141614d49: 48 c7 85 98 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x98], -0x2
141614d54: 48 89 d7                    	mov	rdi, rdx
141614d57: 48 89 ce                    	mov	rsi, rcx
141614d5a: 4d 85 c0                    	test	r8, r8
141614d5d: 74 59                       	je	0x141614db8 <.text+0x1613db8>
141614d5f: 48 b9 cd cc cc cc cc cc cc cc       	movabs	rcx, -0x3333333333333333
141614d69: 4c 89 c8                    	mov	rax, r9
141614d6c: 48 f7 e1                    	mul	rcx
141614d6f: 48 89 d3                    	mov	rbx, rdx
141614d72: 48 c1 eb 04                 	shr	rbx, 0x4
141614d76: 48 8d 04 9d 00 00 00 00     	lea	rax, [4*rbx]
141614d7e: 48 8d 04 80                 	lea	rax, [rax + 4*rax]
141614d82: 4c 89 c9                    	mov	rcx, r9
141614d85: 48 29 c1                    	sub	rcx, rax
141614d88: 48 89 4d c0                 	mov	qword ptr [rbp - 0x40], rcx
141614d8c: 0f 85 31 01 00 00           	jne	0x141614ec3 <.text+0x1613ec3>
141614d92: 4d 89 c7                    	mov	r15, r8
141614d95: 4c 89 c0                    	mov	rax, r8
141614d98: 48 83 e0 03                 	and	rax, 0x3
141614d9c: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
141614da0: 0f 85 2e 01 00 00           	jne	0x141614ed4 <.text+0x1613ed4>
141614da6: 49 81 f9 f4 01 00 00        	cmp	r9, 0x1f4
141614dad: 0f 83 ab 01 00 00           	jae	0x141614f5e <.text+0x1613f5e>
141614db3: 45 31 e4                    	xor	r12d, r12d
141614db6: eb 3a                       	jmp	0x141614df2 <.text+0x1613df2>
141614db8: 0f b6 05 4a fe 01 02        	movzx	eax, byte ptr [rip + 0x201fe4a] # 0x143634c09
141614dbf: b9 e0 01 00 00              	mov	ecx, 0x1e0
141614dc4: ba 04 00 00 00              	mov	edx, 0x4
141614dc9: e8 42 ee ff fe              	call	0x140613c10 <.text+0x612c10>
141614dce: 48 85 c0                    	test	rax, rax
141614dd1: 0f 84 f1 01 00 00           	je	0x141614fc8 <.text+0x1613fc8>
141614dd7: 49 89 c7                    	mov	r15, rax
141614dda: 41 b8 e0 01 00 00           	mov	r8d, 0x1e0
141614de0: 48 89 c1                    	mov	rcx, rax
141614de3: 31 d2                       	xor	edx, edx
141614de5: e8 36 ee 30 01              	call	0x142923c20 <.text+0x2922c20>
141614dea: 41 b4 01                    	mov	r12b, 0x1
141614ded: bb 18 00 00 00              	mov	ebx, 0x18
141614df2: 4c 89 bd 88 00 00 00        	mov	qword ptr [rbp + 0x88], r15
141614df9: 44 88 a5 97 00 00 00        	mov	byte ptr [rbp + 0x97], r12b
141614e00: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
141614e04: ba e0 01 00 00              	mov	edx, 0x1e0
141614e09: 41 b8 22 01 00 00           	mov	r8d, 0x122
141614e0f: 41 b1 01                    	mov	r9b, 0x1
141614e12: e8 c9 ff 5b ff              	call	0x140bd4de0 <.text+0xbd3de0>
141614e17: 4c 89 bd 88 00 00 00        	mov	qword ptr [rbp + 0x88], r15
141614e1e: 44 88 a5 97 00 00 00        	mov	byte ptr [rbp + 0x97], r12b
141614e25: 4c 8d 75 68                 	lea	r14, [rbp + 0x68]
141614e29: 48 8d 55 10                 	lea	rdx, [rbp + 0x10]
141614e2d: 41 b8 04 00 00 00           	mov	r8d, 0x4
141614e33: 4c 89 f1                    	mov	rcx, r14
141614e36: e8 e5 00 5c ff              	call	0x140bd4f20 <.text+0xbd3f20>
141614e3b: 48 c1 e3 02                 	shl	rbx, 0x2
141614e3f: 48 8d 04 9b                 	lea	rax, [rbx + 4*rbx]
141614e43: 4c 89 bd 88 00 00 00        	mov	qword ptr [rbp + 0x88], r15
141614e4a: 44 88 a5 97 00 00 00        	mov	byte ptr [rbp + 0x97], r12b
141614e51: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
141614e56: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
141614e5b: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
141614e60: 4c 8d 05 19 ad 66 01        	lea	r8, [rip + 0x166ad19]   # 0x142c7fb80
141614e67: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
141614e6b: 41 b9 19 00 00 00           	mov	r9d, 0x19
141614e71: 48 89 fa                    	mov	rdx, rdi
141614e74: e8 47 df 08 00              	call	0x1416a2dc0 <.text+0x16a1dc0>
141614e79: 83 7d c0 0b                 	cmp	dword ptr [rbp - 0x40], 0xb
141614e7d: 75 7f                       	jne	0x141614efe <.text+0x1613efe>
141614e7f: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
141614e83: 48 89 46 20                 	mov	qword ptr [rsi + 0x20], rax
141614e87: 0f 10 45 c8                 	movups	xmm0, xmmword ptr [rbp - 0x38]
141614e8b: 0f 10 4d d8                 	movups	xmm1, xmmword ptr [rbp - 0x28]
141614e8f: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
141614e93: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
141614e96: 45 84 e4                    	test	r12b, r12b
141614e99: 74 13                       	je	0x141614eae <.text+0x1613eae>
141614e9b: ba e0 01 00 00              	mov	edx, 0x1e0
141614ea0: 41 b8 04 00 00 00           	mov	r8d, 0x4
141614ea6: 4c 89 f9                    	mov	rcx, r15
141614ea9: e8 72 ed ff fe              	call	0x140613c20 <.text+0x612c20>
141614eae: 48 89 f0                    	mov	rax, rsi
141614eb1: 48 81 c4 20 01 00 00        	add	rsp, 0x120
141614eb8: 5b                          	pop	rbx
141614eb9: 5f                          	pop	rdi
141614eba: 5e                          	pop	rsi
141614ebb: 41 5c                       	pop	r12
141614ebd: 41 5e                       	pop	r14
141614ebf: 41 5f                       	pop	r15
141614ec1: 5d                          	pop	rbp
141614ec2: c3                          	ret
141614ec3: 48 c7 45 10 00 00 00 00     	mov	qword ptr [rbp + 0x10], 0x0
141614ecb: 48 8d 05 76 50 6b 01        	lea	rax, [rip + 0x16b5076]  # 0x142cc9f48
141614ed2: eb 0f                       	jmp	0x141614ee3 <.text+0x1613ee3>
141614ed4: 48 c7 45 10 00 00 00 00     	mov	qword ptr [rbp + 0x10], 0x0
141614edc: 48 8d 05 7d 50 6b 01        	lea	rax, [rip + 0x16b507d]  # 0x142cc9f60
141614ee3: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141614ee8: 4c 8d 05 51 50 6b 01        	lea	r8, [rip + 0x16b5051]   # 0x142cc9f40
141614eef: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
141614ef3: 4c 8d 4d 10                 	lea	r9, [rbp + 0x10]
141614ef7: 31 c9                       	xor	ecx, ecx
141614ef9: e8 98 46 2c 01              	call	0x1428d9596 <.text+0x28d8596>
141614efe: 44 89 a5 84 00 00 00        	mov	dword ptr [rbp + 0x84], r12d
141614f05: 4c 89 7d 78                 	mov	qword ptr [rbp + 0x78], r15
141614f09: 0f 10 45 00                 	movups	xmm0, xmmword ptr [rbp]
141614f0d: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
141614f11: 0f 10 45 c0                 	movups	xmm0, xmmword ptr [rbp - 0x40]
141614f15: 0f 10 4d d0                 	movups	xmm1, xmmword ptr [rbp - 0x30]
141614f19: 0f 10 55 e0                 	movups	xmm2, xmmword ptr [rbp - 0x20]
141614f1d: 0f 10 5d f0                 	movups	xmm3, xmmword ptr [rbp - 0x10]
141614f21: 0f 29 5d 40                 	movaps	xmmword ptr [rbp + 0x40], xmm3
141614f25: 0f 29 55 30                 	movaps	xmmword ptr [rbp + 0x30], xmm2
141614f29: 0f 29 4d 20                 	movaps	xmmword ptr [rbp + 0x20], xmm1
141614f2d: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
141614f31: 48 8d 05 70 b2 66 01        	lea	rax, [rip + 0x166b270]  # 0x142c801a8
141614f38: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141614f3d: 48 8d 0d a4 9b 66 01        	lea	rcx, [rip + 0x1669ba4]  # 0x142c7eae8
141614f44: 4c 8d 0d 7d 9b 66 01        	lea	r9, [rip + 0x1669b7d]   # 0x142c7eac8
141614f4b: 4c 8d 45 10                 	lea	r8, [rbp + 0x10]
141614f4f: ba 2b 00 00 00              	mov	edx, 0x2b
141614f54: e8 07 48 2c 01              	call	0x1428d9760 <.text+0x28d8760>
141614f59: e9 87 00 00 00              	jmp	0x141614fe5 <.text+0x1613fe5>
141614f5e: 48 8d 05 9e 93 48 01        	lea	rax, [rip + 0x148939e]  # 0x142a9e303
141614f65: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
141614f69: 48 c7 45 c8 4e 00 00 00     	mov	qword ptr [rbp - 0x38], 0x4e
141614f71: 48 8d 45 c0                 	lea	rax, [rbp - 0x40]
141614f75: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
141614f79: 48 8d 05 10 4f af fe        	lea	rax, [rip - 0x150b0f0]  # 0x140109e90 <.text+0x108e90>
141614f80: 48 89 45 70                 	mov	qword ptr [rbp + 0x70], rax
141614f84: 48 8d 05 3d a8 66 01        	lea	rax, [rip + 0x166a83d]  # 0x142c7f7c8
141614f8b: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
141614f8f: 48 c7 45 18 01 00 00 00     	mov	qword ptr [rbp + 0x18], 0x1
141614f97: 48 c7 45 30 00 00 00 00     	mov	qword ptr [rbp + 0x30], 0x0
141614f9f: 48 8d 45 68                 	lea	rax, [rbp + 0x68]
141614fa3: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
141614fa7: 48 c7 45 28 01 00 00 00     	mov	qword ptr [rbp + 0x28], 0x1
141614faf: c6 85 97 00 00 00 00        	mov	byte ptr [rbp + 0x97], 0x0
141614fb6: 48 8d 15 1b a8 66 01        	lea	rdx, [rip + 0x166a81b]  # 0x142c7f7d8
141614fbd: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
141614fc1: e8 ca 43 2c 01              	call	0x1428d9390 <.text+0x28d8390>
141614fc6: eb 1d                       	jmp	0x141614fe5 <.text+0x1613fe5>
141614fc8: c6 85 97 00 00 00 00        	mov	byte ptr [rbp + 0x97], 0x0
141614fcf: 4c 8d 05 da 9a 66 01        	lea	r8, [rip + 0x1669ada]   # 0x142c7eab0
141614fd6: b9 04 00 00 00              	mov	ecx, 0x4
141614fdb: ba e0 01 00 00              	mov	edx, 0x1e0
141614fe0: e8 fe 3f 2c 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
141614fe5: 0f 0b                       	ud2
141614fe7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
141614ff0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141614ff5: 55                          	push	rbp
141614ff6: 41 57                       	push	r15
141614ff8: 41 56                       	push	r14
141614ffa: 41 54                       	push	r12
141614ffc: 56                          	push	rsi
141614ffd: 57                          	push	rdi
141614ffe: 53                          	push	rbx
141614fff: 48 83 ec 40                 	sub	rsp, 0x40
141615003: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14161500a: 48 8b 75 78                 	mov	rsi, qword ptr [rbp + 0x78]
14161500e: 48 89 b5 88 00 00 00        	mov	qword ptr [rbp + 0x88], rsi
141615015: 8b bd 84 00 00 00           	mov	edi, dword ptr [rbp + 0x84]
14161501b: 40 88 bd 97 00 00 00        	mov	byte ptr [rbp + 0x97], dil
141615022: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
141615026: e8 45 9a 1b ff              	call	0x1407cea70 <.text+0x7cda70>
14161502b: 48 89 b5 88 00 00 00        	mov	qword ptr [rbp + 0x88], rsi
141615032: 40 88 bd 97 00 00 00        	mov	byte ptr [rbp + 0x97], dil
141615039: 48 83 c4 40                 	add	rsp, 0x40
14161503d: 5b                          	pop	rbx
14161503e: 5f                          	pop	rdi
14161503f: 5e                          	pop	rsi
141615040: 41 5c                       	pop	r12
141615042: 41 5e                       	pop	r14
141615044: 41 5f                       	pop	r15
141615046: 5d                          	pop	rbp
141615047: c3                          	ret
141615048: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
141615050: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141615055: 55                          	push	rbp
141615056: 41 57                       	push	r15
141615058: 41 56                       	push	r14
14161505a: 41 54                       	push	r12
14161505c: 56                          	push	rsi
14161505d: 57                          	push	rdi
14161505e: 53                          	push	rbx
14161505f: 48 83 ec 40                 	sub	rsp, 0x40
141615063: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14161506a: f6 85 97 00 00 00 01        	test	byte ptr [rbp + 0x97], 0x1
141615071: 74 17                       	je	0x14161508a <.text+0x161408a>
141615073: 48 8b 8d 88 00 00 00        	mov	rcx, qword ptr [rbp + 0x88]
14161507a: ba e0 01 00 00              	mov	edx, 0x1e0
14161507f: 41 b8 04 00 00 00           	mov	r8d, 0x4
141615085: e8 96 eb ff fe              	call	0x140613c20 <.text+0x612c20>
14161508a: 90                          	nop
14161508b: 48 83 c4 40                 	add	rsp, 0x40
14161508f: 5b                          	pop	rbx
141615090: 5f                          	pop	rdi
141615091: 5e                          	pop	rsi
141615092: 41 5c                       	pop	r12
141615094: 41 5e                       	pop	r14
141615096: 41 5f                       	pop	r15
141615098: 5d                          	pop	rbp
141615099: c3                          	ret


ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142314b70: 55                          	push	rbp
142314b71: 56                          	push	rsi
142314b72: 57                          	push	rdi
142314b73: 53                          	push	rbx
142314b74: 48 81 ec 68 05 00 00        	sub	rsp, 0x568
142314b7b: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
142314b83: 0f 29 bd d0 04 00 00        	movaps	xmmword ptr [rbp + 0x4d0], xmm7
142314b8a: 0f 29 b5 c0 04 00 00        	movaps	xmmword ptr [rbp + 0x4c0], xmm6
142314b91: 48 c7 85 b8 04 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x4b8], -0x2
142314b9c: 4c 89 c7                    	mov	rdi, r8
142314b9f: 48 89 d6                    	mov	rsi, rdx
142314ba2: 48 89 cb                    	mov	rbx, rcx
142314ba5: 49 8b 01                    	mov	rax, qword ptr [r9]
142314ba8: f3 0f 10 70 28              	movss	xmm6, dword ptr [rax + 0x28]
142314bad: f3 0f 10 05 0b a6 6c 00     	movss	xmm0, dword ptr [rip + 0x6ca60b] # 0x1429df1c0
142314bb5: f3 0f 59 c6                 	mulss	xmm0, xmm6
142314bb9: e8 02 f3 60 00              	call	0x142923ec0 <.text+0x2922ec0>
142314bbe: f3 0f 10 3d 3e 0d 61 00     	movss	xmm7, dword ptr [rip + 0x610d3e] # 0x142925904
142314bc6: 0f 28 cf                    	movaps	xmm1, xmm7
142314bc9: f3 0f 5c c8                 	subss	xmm1, xmm0
142314bcd: f3 0f 11 8d b4 04 00 00     	movss	dword ptr [rbp + 0x4b4], xmm1
142314bd5: f3 0f 59 35 73 6a 76 00     	mulss	xmm6, dword ptr [rip + 0x766a73] # 0x142a7b650
142314bdd: 0f 28 c6                    	movaps	xmm0, xmm6
142314be0: e8 db f2 60 00              	call	0x142923ec0 <.text+0x2922ec0>
142314be5: f3 0f 5c f8                 	subss	xmm7, xmm0
142314be9: f3 0f 11 bd b0 04 00 00     	movss	dword ptr [rbp + 0x4b0], xmm7
142314bf1: 48 8b 03                    	mov	rax, qword ptr [rbx]
142314bf4: 48 8b 4b 08                 	mov	rcx, qword ptr [rbx + 0x8]
142314bf8: 48 8d 91 a0 01 00 00        	lea	rdx, [rcx + 0x1a0]
142314bff: 4c 8d 81 f8 00 00 00        	lea	r8, [rcx + 0xf8]
142314c06: 4c 8b 88 f8 00 00 00        	mov	r9, qword ptr [rax + 0xf8]
142314c0d: 4c 8b 90 00 01 00 00        	mov	r10, qword ptr [rax + 0x100]
142314c14: 4f 8d 14 91                 	lea	r10, [r9 + 4*r10]
142314c18: 44 0f b6 98 08 01 00 00     	movzx	r11d, byte ptr [rax + 0x108]
142314c20: 0f 57 c0                    	xorps	xmm0, xmm0
142314c23: 0f 11 85 48 02 00 00        	movups	xmmword ptr [rbp + 0x248], xmm0
142314c2a: 0f 11 85 38 02 00 00        	movups	xmmword ptr [rbp + 0x238], xmm0
142314c31: 48 c7 85 58 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x258], 0x0
142314c3c: 48 c7 85 d0 01 00 00 20 00 00 00    	mov	qword ptr [rbp + 0x1d0], 0x20
142314c47: 48 8d 9d 30 05 00 00        	lea	rbx, [rbp + 0x530]
142314c4e: 48 89 9d d8 01 00 00        	mov	qword ptr [rbp + 0x1d8], rbx
142314c55: 48 8d 9d b0 04 00 00        	lea	rbx, [rbp + 0x4b0]
142314c5c: 48 89 9d e0 01 00 00        	mov	qword ptr [rbp + 0x1e0], rbx
142314c63: 48 8d 9d b4 04 00 00        	lea	rbx, [rbp + 0x4b4]
142314c6a: 48 89 9d e8 01 00 00        	mov	qword ptr [rbp + 0x1e8], rbx
142314c71: 48 89 8d f0 01 00 00        	mov	qword ptr [rbp + 0x1f0], rcx
142314c78: 48 89 95 f8 01 00 00        	mov	qword ptr [rbp + 0x1f8], rdx
142314c7f: 4c 89 85 00 02 00 00        	mov	qword ptr [rbp + 0x200], r8
142314c86: 48 89 85 08 02 00 00        	mov	qword ptr [rbp + 0x208], rax
142314c8d: 4c 89 8d 10 02 00 00        	mov	qword ptr [rbp + 0x210], r9
142314c94: 4c 89 95 18 02 00 00        	mov	qword ptr [rbp + 0x218], r10
142314c9b: 48 c7 85 20 02 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x220], 0x8
142314ca6: 48 c7 85 28 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x228], 0x0
142314cb1: 48 c7 85 30 02 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x230], 0x8
142314cbc: 44 88 9d 60 02 00 00        	mov	byte ptr [rbp + 0x260], r11b
142314cc3: 4c 8d 05 66 5c d3 00        	lea	r8, [rip + 0xd35c66]    # 0x14304a930
142314cca: 48 8d 8d 98 04 00 00        	lea	rcx, [rbp + 0x498]
142314cd1: 48 8d 95 d0 01 00 00        	lea	rdx, [rbp + 0x1d0]
142314cd8: e8 e3 ab aa ff              	call	0x141dbf8c0 <.text+0x1dbe8c0>
142314cdd: 48 8b 0e                    	mov	rcx, qword ptr [rsi]
142314ce0: 48 8d 15 f9 55 d3 00        	lea	rdx, [rip + 0xd355f9]   # 0x14304a2e0
142314ce7: e8 94 9a ff ff              	call	0x14230e780 <.text+0x230d780>
142314cec: 48 89 c6                    	mov	rsi, rax
142314cef: 48 85 c0                    	test	rax, rax
142314cf2: 0f 84 dd 02 00 00           	je	0x142314fd5 <.text+0x2313fd5>
142314cf8: 48 8d 05 a1 56 d3 00        	lea	rax, [rip + 0xd356a1]   # 0x14304a3a0
142314cff: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
142314d03: 48 c7 45 c8 30 00 00 00     	mov	qword ptr [rbp - 0x38], 0x30
142314d0b: 48 89 f3                    	mov	rbx, rsi
142314d0e: 48 83 c3 10                 	add	rbx, 0x10
142314d12: 48 83 7e 18 30              	cmp	qword ptr [rsi + 0x18], 0x30
142314d17: 0f 85 8a 02 00 00           	jne	0x142314fa7 <.text+0x2313fa7>
142314d1d: 48 8b 13                    	mov	rdx, qword ptr [rbx]
142314d20: 48 8d 0d 79 56 d3 00        	lea	rcx, [rip + 0xd35679]   # 0x14304a3a0
142314d27: 41 b8 30 00 00 00           	mov	r8d, 0x30
142314d2d: e8 be ee 60 00              	call	0x142923bf0 <.text+0x2922bf0>
142314d32: 85 c0                       	test	eax, eax
142314d34: 0f 85 6d 02 00 00           	jne	0x142314fa7 <.text+0x2313fa7>
142314d3a: 48 8b 0e                    	mov	rcx, qword ptr [rsi]
142314d3d: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
142314d41: ff 50 18                    	call	qword ptr [rax + 0x18]
142314d44: 48 89 c3                    	mov	rbx, rax
142314d47: 48 85 c0                    	test	rax, rax
142314d4a: 0f 84 85 02 00 00           	je	0x142314fd5 <.text+0x2313fd5>
142314d50: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
142314d54: 41 b8 00 02 00 00           	mov	r8d, 0x200
142314d5a: 31 d2                       	xor	edx, edx
142314d5c: e8 bf ee 60 00              	call	0x142923c20 <.text+0x2922c20>
142314d61: 48 8b b5 a0 04 00 00        	mov	rsi, qword ptr [rbp + 0x4a0]
142314d68: 48 8b 8d a8 04 00 00        	mov	rcx, qword ptr [rbp + 0x4a8]
142314d6f: 89 8d c0 01 00 00           	mov	dword ptr [rbp + 0x1c0], ecx
142314d75: 48 c7 85 c4 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1c4], 0x0
142314d80: c7 85 cc 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x1cc], 0x0
142314d8a: 48 83 f9 20                 	cmp	rcx, 0x20
142314d8e: b8 20 00 00 00              	mov	eax, 0x20
142314d93: 48 0f 42 c1                 	cmovb	rax, rcx
142314d97: 48 85 c9                    	test	rcx, rcx
142314d9a: 74 59                       	je	0x142314df5 <.text+0x2313df5>
142314d9c: 48 83 f9 01                 	cmp	rcx, 0x1
142314da0: 75 04                       	jne	0x142314da6 <.text+0x2313da6>
142314da2: 31 c9                       	xor	ecx, ecx
142314da4: eb 3e                       	jmp	0x142314de4 <.text+0x2313de4>
142314da6: 89 c2                       	mov	edx, eax
142314da8: 83 e2 3e                    	and	edx, 0x3e
142314dab: 41 b8 10 00 00 00           	mov	r8d, 0x10
142314db1: 31 c9                       	xor	ecx, ecx
142314db3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
142314dc0: 42 0f 10 44 06 f0           	movups	xmm0, xmmword ptr [rsi + r8 - 0x10]
142314dc6: 42 0f 11 44 05 b0           	movups	xmmword ptr [rbp + r8 - 0x50], xmm0
142314dcc: 48 83 c1 02                 	add	rcx, 0x2
142314dd0: 42 0f 10 04 06              	movups	xmm0, xmmword ptr [rsi + r8]
142314dd5: 42 0f 11 44 05 c0           	movups	xmmword ptr [rbp + r8 - 0x40], xmm0
142314ddb: 49 83 c0 20                 	add	r8, 0x20
142314ddf: 48 39 ca                    	cmp	rdx, rcx
142314de2: 75 dc                       	jne	0x142314dc0 <.text+0x2313dc0>
142314de4: a8 01                       	test	al, 0x1
142314de6: 74 0d                       	je	0x142314df5 <.text+0x2313df5>
142314de8: 48 c1 e1 04                 	shl	rcx, 0x4
142314dec: 0f 10 04 0e                 	movups	xmm0, xmmword ptr [rsi + rcx]
142314df0: 0f 11 44 0d c0              	movups	xmmword ptr [rbp + rcx - 0x40], xmm0
142314df5: 48 8b 0f                    	mov	rcx, qword ptr [rdi]
142314df8: e8 a3 2e 67 fe              	call	0x140987ca0 <.text+0x986ca0>
142314dfd: 48 89 c7                    	mov	rdi, rax
142314e00: 48 8d 8d d0 01 00 00        	lea	rcx, [rbp + 0x1d0]
142314e07: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
142314e0b: 41 b8 10 02 00 00           	mov	r8d, 0x210
142314e11: e8 ea ed 60 00              	call	0x142923c00 <.text+0x2922c00>
142314e16: 48 81 7b 18 0f 02 00 00     	cmp	qword ptr [rbx + 0x18], 0x20f
142314e1e: 0f 86 4b 01 00 00           	jbe	0x142314f6f <.text+0x2313f6f>
142314e24: 48 c7 85 88 04 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x488], 0x0
142314e2f: 48 c7 85 90 04 00 00 10 02 00 00    	mov	qword ptr [rbp + 0x490], 0x210
142314e3a: 48 8d 85 88 04 00 00        	lea	rax, [rbp + 0x488]
142314e41: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
142314e46: 48 c7 44 24 38 00 00 00 00  	mov	qword ptr [rsp + 0x38], 0x0
142314e4f: 48 c7 44 24 30 01 00 00 00  	mov	qword ptr [rsp + 0x30], 0x1
142314e58: 48 c7 44 24 20 10 02 00 00  	mov	qword ptr [rsp + 0x20], 0x210
142314e61: 48 8d 8d e0 03 00 00        	lea	rcx, [rbp + 0x3e0]
142314e68: 4c 8d 8d d0 01 00 00        	lea	r9, [rbp + 0x1d0]
142314e6f: 48 89 fa                    	mov	rdx, rdi
142314e72: 49 89 d8                    	mov	r8, rbx
142314e75: e8 26 a1 8d fe              	call	0x140beefa0 <.text+0xbedfa0>
142314e7a: 83 bd e0 03 00 00 0b        	cmp	dword ptr [rbp + 0x3e0], 0xb
142314e81: 0f 84 e8 00 00 00           	je	0x142314f6f <.text+0x2313f6f>
142314e87: 0f 10 85 e0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3e0]
142314e8e: 0f 10 8d f0 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x3f0]
142314e95: 0f 10 95 00 04 00 00        	movups	xmm2, xmmword ptr [rbp + 0x400]
142314e9c: 0f 10 9d 10 04 00 00        	movups	xmm3, xmmword ptr [rbp + 0x410]
142314ea3: 0f 29 85 30 04 00 00        	movaps	xmmword ptr [rbp + 0x430], xmm0
142314eaa: 0f 10 85 20 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x420]
142314eb1: 0f 29 85 70 04 00 00        	movaps	xmmword ptr [rbp + 0x470], xmm0
142314eb8: 0f 29 9d 60 04 00 00        	movaps	xmmword ptr [rbp + 0x460], xmm3
142314ebf: 0f 29 95 50 04 00 00        	movaps	xmmword ptr [rbp + 0x450], xmm2
142314ec6: 0f 29 8d 40 04 00 00        	movaps	xmmword ptr [rbp + 0x440], xmm1
142314ecd: 48 8b 85 30 04 00 00        	mov	rax, qword ptr [rbp + 0x430]
142314ed4: 48 8d 48 f5                 	lea	rcx, [rax - 0xb]
142314ed8: 48 83 f9 02                 	cmp	rcx, 0x2
142314edc: 0f 82 8d 00 00 00           	jb	0x142314f6f <.text+0x2313f6f>
142314ee2: 48 8d 50 f7                 	lea	rdx, [rax - 0x9]
142314ee6: 4c 8d 40 f8                 	lea	r8, [rax - 0x8]
142314eea: 31 c9                       	xor	ecx, ecx
142314eec: 48 83 fa 02                 	cmp	rdx, 0x2
142314ef0: 49 0f 42 c8                 	cmovb	rcx, r8
142314ef4: 48 85 c9                    	test	rcx, rcx
142314ef7: 74 14                       	je	0x142314f0d <.text+0x2313f0d>
142314ef9: 48 83 f9 01                 	cmp	rcx, 0x1
142314efd: 75 52                       	jne	0x142314f51 <.text+0x2313f51>
142314eff: 48 8d 8d 38 04 00 00        	lea	rcx, [rbp + 0x438]
142314f06: e8 25 36 57 fe              	call	0x140888530 <.text+0x887530>
142314f0b: eb 62                       	jmp	0x142314f6f <.text+0x2313f6f>
142314f0d: 48 83 f8 05                 	cmp	rax, 0x5
142314f11: 77 29                       	ja	0x142314f3c <.text+0x2313f3c>
142314f13: b9 32 00 00 00              	mov	ecx, 0x32
142314f18: 48 0f a3 c1                 	bt	rcx, rax
142314f1c: 73 1e                       	jae	0x142314f3c <.text+0x2313f3c>
142314f1e: 48 8b 95 38 04 00 00        	mov	rdx, qword ptr [rbp + 0x438]
142314f25: 48 85 d2                    	test	rdx, rdx
142314f28: 74 12                       	je	0x142314f3c <.text+0x2313f3c>
142314f2a: 48 8b 8d 40 04 00 00        	mov	rcx, qword ptr [rbp + 0x440]
142314f31: 41 b8 01 00 00 00           	mov	r8d, 0x1
142314f37: e8 e4 ec 2f fe              	call	0x140613c20 <.text+0x612c20>
142314f3c: 48 8b 95 50 04 00 00        	mov	rdx, qword ptr [rbp + 0x450]
142314f43: 48 85 d2                    	test	rdx, rdx
142314f46: 74 27                       	je	0x142314f6f <.text+0x2313f6f>
142314f48: 48 8b 8d 58 04 00 00        	mov	rcx, qword ptr [rbp + 0x458]
142314f4f: eb 13                       	jmp	0x142314f64 <.text+0x2313f64>
142314f51: 48 8b 95 38 04 00 00        	mov	rdx, qword ptr [rbp + 0x438]
142314f58: 48 85 d2                    	test	rdx, rdx
142314f5b: 74 12                       	je	0x142314f6f <.text+0x2313f6f>
142314f5d: 48 8b 8d 40 04 00 00        	mov	rcx, qword ptr [rbp + 0x440]
142314f64: 41 b8 01 00 00 00           	mov	r8d, 0x1
142314f6a: e8 b1 ec 2f fe              	call	0x140613c20 <.text+0x612c20>
142314f6f: 48 8b 95 98 04 00 00        	mov	rdx, qword ptr [rbp + 0x498]
142314f76: 48 85 d2                    	test	rdx, rdx
142314f79: 74 12                       	je	0x142314f8d <.text+0x2313f8d>
142314f7b: 48 c1 e2 04                 	shl	rdx, 0x4
142314f7f: 41 b8 04 00 00 00           	mov	r8d, 0x4
142314f85: 48 89 f1                    	mov	rcx, rsi
142314f88: e8 93 ec 2f fe              	call	0x140613c20 <.text+0x612c20>
142314f8d: 0f 28 b5 c0 04 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x4c0]
142314f94: 0f 28 bd d0 04 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x4d0]
142314f9b: 48 81 c4 68 05 00 00        	add	rsp, 0x568
142314fa2: 5b                          	pop	rbx
142314fa3: 5f                          	pop	rdi
142314fa4: 5e                          	pop	rsi
142314fa5: 5d                          	pop	rbp
142314fa6: c3                          	ret
142314fa7: 48 c7 85 d0 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1d0], 0x0
142314fb2: 48 8d 05 07 53 d3 00        	lea	rax, [rip + 0xd35307]   # 0x14304a2c0
142314fb9: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142314fbe: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
142314fc2: 4c 8d 8d d0 01 00 00        	lea	r9, [rbp + 0x1d0]
142314fc9: 31 c9                       	xor	ecx, ecx
142314fcb: 49 89 d8                    	mov	r8, rbx
142314fce: e8 6e 5b 5b 00              	call	0x1428cab41 <.text+0x28c9b41>
142314fd3: eb 0c                       	jmp	0x142314fe1 <.text+0x2313fe1>
142314fd5: 48 8d 0d 24 5d d3 00        	lea	rcx, [rip + 0xd35d24]   # 0x14304ad00
142314fdc: e8 2f 43 5c 00              	call	0x1428d9310 <.text+0x28d8310>
142314fe1: 0f 0b                       	ud2
142314fe3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
142314ff0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142314ff5: 55                          	push	rbp
142314ff6: 56                          	push	rsi
142314ff7: 57                          	push	rdi
142314ff8: 53                          	push	rbx
142314ff9: 48 83 ec 68                 	sub	rsp, 0x68
142314ffd: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
142315004: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
142315009: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14231500e: 48 8b 95 98 04 00 00        	mov	rdx, qword ptr [rbp + 0x498]
142315015: 48 85 d2                    	test	rdx, rdx
142315018: 74 16                       	je	0x142315030 <.text+0x2314030>
14231501a: 48 8b 8d a0 04 00 00        	mov	rcx, qword ptr [rbp + 0x4a0]
142315021: 48 c1 e2 04                 	shl	rdx, 0x4
142315025: 41 b8 04 00 00 00           	mov	r8d, 0x4
14231502b: e8 f0 eb 2f fe              	call	0x140613c20 <.text+0x612c20>
142315030: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
142315035: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
14231503a: 48 83 c4 68                 	add	rsp, 0x68
14231503e: 5b                          	pop	rbx
14231503f: 5f                          	pop	rdi
142315040: 5e                          	pop	rsi
142315041: 5d                          	pop	rbp
142315042: c3                          	ret


D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141ea8c60: 55                          	push	rbp
141ea8c61: 41 57                       	push	r15
141ea8c63: 41 56                       	push	r14
141ea8c65: 41 54                       	push	r12
141ea8c67: 56                          	push	rsi
141ea8c68: 57                          	push	rdi
141ea8c69: 53                          	push	rbx
141ea8c6a: 48 81 ec 90 00 00 00        	sub	rsp, 0x90
141ea8c71: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
141ea8c79: 48 c7 45 08 fe ff ff ff     	mov	qword ptr [rbp + 0x8], -0x2
141ea8c81: 4c 89 ce                    	mov	rsi, r9
141ea8c84: 44 89 c3                    	mov	ebx, r8d
141ea8c87: 41 89 d6                    	mov	r14d, edx
141ea8c8a: 48 89 cf                    	mov	rdi, rcx
141ea8c8d: 89 55 d0                    	mov	dword ptr [rbp - 0x30], edx
141ea8c90: 44 89 45 d4                 	mov	dword ptr [rbp - 0x2c], r8d
141ea8c94: 4d 8b 39                    	mov	r15, qword ptr [r9]
141ea8c97: 49 83 7f 18 00              	cmp	qword ptr [r15 + 0x18], 0x0
141ea8c9c: 0f 84 1b 03 00 00           	je	0x141ea8fbd <.text+0x1ea7fbd>
141ea8ca2: 49 8d 4f 20                 	lea	rcx, [r15 + 0x20]
141ea8ca6: 48 8d 55 d0                 	lea	rdx, [rbp - 0x30]
141ea8caa: e8 e1 85 0f ff              	call	0x140fa1290 <.text+0xfa0290>
141ea8caf: 49 8b 0f                    	mov	rcx, qword ptr [r15]
141ea8cb2: 49 8b 57 08                 	mov	rdx, qword ptr [r15 + 0x8]
141ea8cb6: 49 89 d0                    	mov	r8, rdx
141ea8cb9: 49 21 c0                    	and	r8, rax
141ea8cbc: 48 c1 e8 39                 	shr	rax, 0x39
141ea8cc0: 66 0f 6e c0                 	movd	xmm0, eax
141ea8cc4: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141ea8cc8: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141ea8ccd: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
141ea8cd2: 48 8d 41 f0                 	lea	rax, [rcx - 0x10]
141ea8cd6: 45 31 c9                    	xor	r9d, r9d
141ea8cd9: f3 0f 7e 4d d0              	movq	xmm1, qword ptr [rbp - 0x30]
141ea8cde: 66 0f 76 d2                 	pcmpeqd	xmm2, xmm2
141ea8ce2: f3 42 0f 6f 1c 01           	movdqu	xmm3, xmmword ptr [rcx + r8]
141ea8ce8: 66 0f 6f e3                 	movdqa	xmm4, xmm3
141ea8cec: 66 0f 74 e0                 	pcmpeqb	xmm4, xmm0
141ea8cf0: 66 44 0f d7 d4              	pmovmskb	r10d, xmm4
141ea8cf5: 45 85 d2                    	test	r10d, r10d
141ea8cf8: 74 46                       	je	0x141ea8d40 <.text+0x1ea7d40>
141ea8cfa: f3 45 0f bc da              	tzcnt	r11d, r10d
141ea8cff: 4d 01 c3                    	add	r11, r8
141ea8d02: 49 21 d3                    	and	r11, rdx
141ea8d05: 4d 89 df                    	mov	r15, r11
141ea8d08: 49 c1 e7 04                 	shl	r15, 0x4
141ea8d0c: 49 89 c4                    	mov	r12, rax
141ea8d0f: 4d 29 fc                    	sub	r12, r15
141ea8d12: f3 41 0f 7e 24 24           	movq	xmm4, qword ptr [r12]
141ea8d18: 66 0f 76 e1                 	pcmpeqd	xmm4, xmm1
141ea8d1c: 66 0f 70 e4 50              	pshufd	xmm4, xmm4, 0x50        # xmm4 = xmm4[0,0,1,1]
141ea8d21: 66 44 0f 50 fc              	movmskpd	r15d, xmm4
141ea8d26: 41 83 ff 03                 	cmp	r15d, 0x3
141ea8d2a: 74 36                       	je	0x141ea8d62 <.text+0x1ea7d62>
141ea8d2c: 45 8d 5a ff                 	lea	r11d, [r10 - 0x1]
141ea8d30: 66 45 21 d3                 	and	r11w, r10w
141ea8d34: 45 89 da                    	mov	r10d, r11d
141ea8d37: 75 c1                       	jne	0x141ea8cfa <.text+0x1ea7cfa>
141ea8d39: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
141ea8d40: 66 0f 74 da                 	pcmpeqb	xmm3, xmm2
141ea8d44: 66 44 0f d7 d3              	pmovmskb	r10d, xmm3
141ea8d49: 45 85 d2                    	test	r10d, r10d
141ea8d4c: 0f 85 6b 02 00 00           	jne	0x141ea8fbd <.text+0x1ea7fbd>
141ea8d52: 4d 01 c8                    	add	r8, r9
141ea8d55: 49 83 c0 10                 	add	r8, 0x10
141ea8d59: 49 83 c1 10                 	add	r9, 0x10
141ea8d5d: 49 21 d0                    	and	r8, rdx
141ea8d60: eb 80                       	jmp	0x141ea8ce2 <.text+0x1ea7ce2>
141ea8d62: 49 f7 db                    	neg	r11
141ea8d65: 49 c1 e3 04                 	shl	r11, 0x4
141ea8d69: 48 8b 56 08                 	mov	rdx, qword ptr [rsi + 0x8]
141ea8d6d: 4c 8b 4a 08                 	mov	r9, qword ptr [rdx + 0x8]
141ea8d71: 4a 8b 44 19 f8              	mov	rax, qword ptr [rcx + r11 - 0x8]
141ea8d76: 89 c1                       	mov	ecx, eax
141ea8d78: 49 3b 49 10                 	cmp	rcx, qword ptr [r9 + 0x10]
141ea8d7c: 0f 83 3b 02 00 00           	jae	0x141ea8fbd <.text+0x1ea7fbd>
141ea8d82: 4d 8b 41 08                 	mov	r8, qword ptr [r9 + 0x8]
141ea8d86: 49 89 c2                    	mov	r10, rax
141ea8d89: 49 c1 ea 20                 	shr	r10, 0x20
141ea8d8d: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
141ea8d91: 45 39 14 88                 	cmp	dword ptr [r8 + 4*rcx], r10d
141ea8d95: 0f 85 22 02 00 00           	jne	0x141ea8fbd <.text+0x1ea7fbd>
141ea8d9b: 49 8d 0c 88                 	lea	rcx, [r8 + 4*rcx]
141ea8d9f: 44 8b 41 04                 	mov	r8d, dword ptr [rcx + 0x4]
141ea8da3: 41 ba ff ff ff ff           	mov	r10d, 0xffffffff
141ea8da9: 4d 39 d0                    	cmp	r8, r10
141ea8dac: 0f 84 0b 02 00 00           	je	0x141ea8fbd <.text+0x1ea7fbd>
141ea8db2: 48 8b 12                    	mov	rdx, qword ptr [rdx]
141ea8db5: 4c 39 42 38                 	cmp	qword ptr [rdx + 0x38], r8
141ea8db9: 0f 86 fe 01 00 00           	jbe	0x141ea8fbd <.text+0x1ea7fbd>
141ea8dbf: 4c 8b 52 28                 	mov	r10, qword ptr [rdx + 0x28]
141ea8dc3: 45 89 c3                    	mov	r11d, r8d
141ea8dc6: 41 c1 eb 06                 	shr	r11d, 0x6
141ea8dca: 4f 8b 14 da                 	mov	r10, qword ptr [r10 + 8*r11]
141ea8dce: 4d 0f a3 c2                 	bt	r10, r8
141ea8dd2: 0f 83 e5 01 00 00           	jae	0x141ea8fbd <.text+0x1ea7fbd>
141ea8dd8: 4d 39 81 08 01 00 00        	cmp	qword ptr [r9 + 0x108], r8
141ea8ddf: 76 11                       	jbe	0x141ea8df2 <.text+0x1ea7df2>
141ea8de1: 4f 8d 04 80                 	lea	r8, [r8 + 4*r8]
141ea8de5: 49 c1 e0 05                 	shl	r8, 0x5
141ea8de9: 4d 03 81 00 01 00 00        	add	r8, qword ptr [r9 + 0x100]
141ea8df0: eb 03                       	jmp	0x141ea8df5 <.text+0x1ea7df5>
141ea8df2: 45 31 c0                    	xor	r8d, r8d
141ea8df5: 4c 8b ba 10 01 00 00        	mov	r15, qword ptr [rdx + 0x110]
141ea8dfc: 4d 8b 90 88 00 00 00        	mov	r10, qword ptr [r8 + 0x88]
141ea8e03: 41 b3 01                    	mov	r11b, 0x1
141ea8e06: 4d 39 fa                    	cmp	r10, r15
141ea8e09: 76 50                       	jbe	0x141ea8e5b <.text+0x1ea7e5b>
141ea8e0b: 4d 8b a0 80 00 00 00        	mov	r12, qword ptr [r8 + 0x80]
141ea8e12: 4b 83 3c fc 00              	cmp	qword ptr [r12 + 8*r15], 0x0
141ea8e17: 74 42                       	je	0x141ea8e5b <.text+0x1ea7e5b>
141ea8e19: 44 8b 59 0c                 	mov	r11d, dword ptr [rcx + 0xc]
141ea8e1d: 4f 8d 1c db                 	lea	r11, [r11 + 8*r11]
141ea8e21: 49 c1 e3 03                 	shl	r11, 0x3
141ea8e25: 4d 03 99 a8 01 00 00        	add	r11, qword ptr [r9 + 0x1a8]
141ea8e2c: 4d 39 7b 40                 	cmp	qword ptr [r11 + 0x40], r15
141ea8e30: 76 26                       	jbe	0x141ea8e58 <.text+0x1ea7e58>
141ea8e32: 4d 8b 4b 38                 	mov	r9, qword ptr [r11 + 0x38]
141ea8e36: 4f 8b 0c f9                 	mov	r9, qword ptr [r9 + 8*r15]
141ea8e3a: 4d 85 c9                    	test	r9, r9
141ea8e3d: 74 19                       	je	0x141ea8e58 <.text+0x1ea7e58>
141ea8e3f: 4d 8b 5b 18                 	mov	r11, qword ptr [r11 + 0x18]
141ea8e43: 49 f7 d1                    	not	r9
141ea8e46: 4f 8d 0c 49                 	lea	r9, [r9 + 2*r9]
141ea8e4a: 49 c1 e1 04                 	shl	r9, 0x4
141ea8e4e: 4f 8b 4c 0b 10              	mov	r9, qword ptr [r11 + r9 + 0x10]
141ea8e53: 45 31 db                    	xor	r11d, r11d
141ea8e56: eb 06                       	jmp	0x141ea8e5e <.text+0x1ea7e5e>
141ea8e58: 45 31 db                    	xor	r11d, r11d
141ea8e5b: 45 31 c9                    	xor	r9d, r9d
141ea8e5e: 48 8b 92 18 01 00 00        	mov	rdx, qword ptr [rdx + 0x118]
141ea8e65: 49 39 d2                    	cmp	r10, rdx
141ea8e68: 76 0c                       	jbe	0x141ea8e76 <.text+0x1ea7e76>
141ea8e6a: 4d 8b 80 80 00 00 00        	mov	r8, qword ptr [r8 + 0x80]
141ea8e71: 49 83 3c d0 00              	cmp	qword ptr [r8 + 8*rdx], 0x0
141ea8e76: 4d 85 c9                    	test	r9, r9
141ea8e79: 0f 94 c2                    	sete	dl
141ea8e7c: 41 08 d3                    	or	r11b, dl
141ea8e7f: 0f 85 38 01 00 00           	jne	0x141ea8fbd <.text+0x1ea7fbd>
141ea8e85: 8b 51 10                    	mov	edx, dword ptr [rcx + 0x10]
141ea8e88: 48 c1 e2 04                 	shl	rdx, 0x4
141ea8e8c: 4c 8b 46 10                 	mov	r8, qword ptr [rsi + 0x10]
141ea8e90: 48 8b 4e 18                 	mov	rcx, qword ptr [rsi + 0x18]
141ea8e94: f2 41 0f 10 14 11           	movsd	xmm2, qword ptr [r9 + rdx]
141ea8e9a: f2 41 0f 10 00              	movsd	xmm0, qword ptr [r8]
141ea8e9f: 0f 5c d0                    	subps	xmm2, xmm0
141ea8ea2: 0f 28 c2                    	movaps	xmm0, xmm2
141ea8ea5: 0f 59 c2                    	mulps	xmm0, xmm2
141ea8ea8: f3 0f 16 c8                 	movshdup	xmm1, xmm0              # xmm1 = xmm0[1,1,3,3]
141ea8eac: f3 0f 58 c8                 	addss	xmm1, xmm0
141ea8eb0: 0f 57 c0                    	xorps	xmm0, xmm0
141ea8eb3: f3 0f 51 c1                 	sqrtss	xmm0, xmm1
141ea8eb7: f3 0f 10 0d 45 ca a7 00     	movss	xmm1, dword ptr [rip + 0xa7ca45] # 0x142925904
141ea8ebf: 0f 28 d9                    	movaps	xmm3, xmm1
141ea8ec2: f3 0f 5e d8                 	divss	xmm3, xmm0
141ea8ec6: 66 0f 7e da                 	movd	edx, xmm3
141ea8eca: 85 d2                       	test	edx, edx
141ea8ecc: 41 0f 99 c0                 	setns	r8b
141ea8ed0: 41 89 d1                    	mov	r9d, edx
141ea8ed3: 41 81 e1 ff ff ff 7f        	and	r9d, 0x7fffffff
141ea8eda: 41 81 c1 00 00 80 ff        	add	r9d, 0xff800000
141ea8ee1: 41 81 f9 00 00 00 7f        	cmp	r9d, 0x7f000000
141ea8ee8: 41 0f 92 c1                 	setb	r9b
141ea8eec: 45 20 c1                    	and	r9b, r8b
141ea8eef: ff ca                       	dec	edx
141ea8ef1: 81 fa ff ff 7f 00           	cmp	edx, 0x7fffff
141ea8ef7: 0f 92 c2                    	setb	dl
141ea8efa: 44 08 ca                    	or	dl, r9b
141ea8efd: f3 0f 12 db                 	movsldup	xmm3, xmm3              # xmm3 = xmm3[0,0,2,2]
141ea8f01: 0f 59 da                    	mulps	xmm3, xmm2
141ea8f04: 0f b6 d2                    	movzx	edx, dl
141ea8f07: 66 0f 6e d2                 	movd	xmm2, edx
141ea8f0b: 66 0f 70 d2 50              	pshufd	xmm2, xmm2, 0x50        # xmm2 = xmm2[0,0,1,1]
141ea8f10: 66 0f 72 f2 1f              	pslld	xmm2, 0x1f
141ea8f15: 66 0f 72 e2 1f              	psrad	xmm2, 0x1f
141ea8f1a: 66 0f db d3                 	pand	xmm2, xmm3
141ea8f1e: f2 0f 10 19                 	movsd	xmm3, qword ptr [rcx]
141ea8f22: 0f 28 e3                    	movaps	xmm4, xmm3
141ea8f25: 0f 59 e3                    	mulps	xmm4, xmm3
141ea8f28: f3 0f 16 ec                 	movshdup	xmm5, xmm4              # xmm5 = xmm4[1,1,3,3]
141ea8f2c: f3 0f 58 ec                 	addss	xmm5, xmm4
141ea8f30: 0f 57 e4                    	xorps	xmm4, xmm4
141ea8f33: f3 0f 51 e5                 	sqrtss	xmm4, xmm5
141ea8f37: f3 0f 5e cc                 	divss	xmm1, xmm4
141ea8f3b: 66 0f 7e c9                 	movd	ecx, xmm1
141ea8f3f: 85 c9                       	test	ecx, ecx
141ea8f41: 0f 99 c2                    	setns	dl
141ea8f44: 41 89 c8                    	mov	r8d, ecx
141ea8f47: 41 81 e0 ff ff ff 7f        	and	r8d, 0x7fffffff
141ea8f4e: 41 81 c0 00 00 80 ff        	add	r8d, 0xff800000
141ea8f55: 41 81 f8 00 00 00 7f        	cmp	r8d, 0x7f000000
141ea8f5c: 41 0f 92 c0                 	setb	r8b
141ea8f60: 41 20 d0                    	and	r8b, dl
141ea8f63: ff c9                       	dec	ecx
141ea8f65: 81 f9 ff ff 7f 00           	cmp	ecx, 0x7fffff
141ea8f6b: 0f 92 c1                    	setb	cl
141ea8f6e: 44 08 c1                    	or	cl, r8b
141ea8f71: f3 0f 12 c9                 	movsldup	xmm1, xmm1              # xmm1 = xmm1[0,0,2,2]
141ea8f75: 0f 59 cb                    	mulps	xmm1, xmm3
141ea8f78: 0f b6 c9                    	movzx	ecx, cl
141ea8f7b: 66 0f 6e d9                 	movd	xmm3, ecx
141ea8f7f: 66 0f 70 db 50              	pshufd	xmm3, xmm3, 0x50        # xmm3 = xmm3[0,0,1,1]
141ea8f84: 66 0f 72 f3 1f              	pslld	xmm3, 0x1f
141ea8f89: 66 0f 72 e3 1f              	psrad	xmm3, 0x1f
141ea8f8e: 66 0f db d9                 	pand	xmm3, xmm1
141ea8f92: 0f 59 da                    	mulps	xmm3, xmm2
141ea8f95: f3 0f 16 cb                 	movshdup	xmm1, xmm3              # xmm1 = xmm3[1,1,3,3]
141ea8f99: f3 0f 58 cb                 	addss	xmm1, xmm3
141ea8f9d: 0f 57 d2                    	xorps	xmm2, xmm2
141ea8fa0: 0f 2e d1                    	ucomiss	xmm2, xmm1
141ea8fa3: 73 18                       	jae	0x141ea8fbd <.text+0x1ea7fbd>
141ea8fa5: 48 8b 4e 20                 	mov	rcx, qword ptr [rsi + 0x20]
141ea8fa9: f3 0f 10 09                 	movss	xmm1, dword ptr [rcx]
141ea8fad: 0f 2e c8                    	ucomiss	xmm1, xmm0
141ea8fb0: 76 0b                       	jbe	0x141ea8fbd <.text+0x1ea7fbd>
141ea8fb2: 48 8b 56 28                 	mov	rdx, qword ptr [rsi + 0x28]
141ea8fb6: 48 89 02                    	mov	qword ptr [rdx], rax
141ea8fb9: f3 0f 11 01                 	movss	dword ptr [rcx], xmm0
141ea8fbd: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
141ea8fc1: 48 89 fa                    	mov	rdx, rdi
141ea8fc4: 45 89 f0                    	mov	r8d, r14d
141ea8fc7: 41 89 d9                    	mov	r9d, ebx
141ea8fca: e8 81 e9 13 ff              	call	0x140fe7950 <.text+0xfe6950>
141ea8fcf: 83 7d b0 04                 	cmp	dword ptr [rbp - 0x50], 0x4
141ea8fd3: 75 6e                       	jne	0x141ea9043 <.text+0x1ea8043>
141ea8fd5: 48 8b 45 b8                 	mov	rax, qword ptr [rbp - 0x48]
141ea8fd9: 48 89 45 00                 	mov	qword ptr [rbp], rax
141ea8fdd: 48 8b 45 c0                 	mov	rax, qword ptr [rbp - 0x40]
141ea8fe1: 48 8b 5d c8                 	mov	rbx, qword ptr [rbp - 0x38]
141ea8fe5: 48 c1 e3 03                 	shl	rbx, 0x3
141ea8fe9: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
141ea8fed: 0f 1f 00                    	nop	dword ptr [rax]
141ea8ff0: 48 85 db                    	test	rbx, rbx
141ea8ff3: 74 1e                       	je	0x141ea9013 <.text+0x1ea8013>
141ea8ff5: 4c 8d 70 08                 	lea	r14, [rax + 0x8]
141ea8ff9: 8b 10                       	mov	edx, dword ptr [rax]
141ea8ffb: 44 8b 40 04                 	mov	r8d, dword ptr [rax + 0x4]
141ea8fff: 48 83 c3 f8                 	add	rbx, -0x8
141ea9003: 48 89 f9                    	mov	rcx, rdi
141ea9006: 49 89 f1                    	mov	r9, rsi
141ea9009: e8 52 fc ff ff              	call	0x141ea8c60 <.text+0x1ea7c60>
141ea900e: 4c 89 f0                    	mov	rax, r14
141ea9011: eb dd                       	jmp	0x141ea8ff0 <.text+0x1ea7ff0>
141ea9013: 48 8b 55 00                 	mov	rdx, qword ptr [rbp]
141ea9017: 48 85 d2                    	test	rdx, rdx
141ea901a: 74 13                       	je	0x141ea902f <.text+0x1ea802f>
141ea901c: 48 c1 e2 03                 	shl	rdx, 0x3
141ea9020: 41 b8 04 00 00 00           	mov	r8d, 0x4
141ea9026: 48 8b 4d f8                 	mov	rcx, qword ptr [rbp - 0x8]
141ea902a: e8 f1 ab 76 fe              	call	0x140613c20 <.text+0x612c20>
141ea902f: 31 c0                       	xor	eax, eax
141ea9031: 48 81 c4 90 00 00 00        	add	rsp, 0x90
141ea9038: 5b                          	pop	rbx
141ea9039: 5f                          	pop	rdi
141ea903a: 5e                          	pop	rsi
141ea903b: 41 5c                       	pop	r12
141ea903d: 41 5e                       	pop	r14
141ea903f: 41 5f                       	pop	r15
141ea9041: 5d                          	pop	rbp
141ea9042: c3                          	ret
141ea9043: 0f 10 45 b0                 	movups	xmm0, xmmword ptr [rbp - 0x50]
141ea9047: 0f 10 4d c0                 	movups	xmm1, xmmword ptr [rbp - 0x40]
141ea904b: 0f 29 4d e0                 	movaps	xmmword ptr [rbp - 0x20], xmm1
141ea904f: 0f 29 45 d0                 	movaps	xmmword ptr [rbp - 0x30], xmm0
141ea9053: 48 8d 05 fe e2 0e 01        	lea	rax, [rip + 0x10ee2fe]  # 0x142f97358
141ea905a: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141ea905f: 48 8d 0d a2 de 0e 01        	lea	rcx, [rip + 0x10edea2]  # 0x142f96f08
141ea9066: 4c 8d 0d 7b de 0e 01        	lea	r9, [rip + 0x10ede7b]   # 0x142f96ee8
141ea906d: 4c 8d 45 d0                 	lea	r8, [rbp - 0x30]
141ea9071: ba 2b 00 00 00              	mov	edx, 0x2b
141ea9076: e8 e5 06 a3 00              	call	0x1428d9760 <.text+0x28d8760>
141ea907b: cc                          	int3
141ea907c: 0f 1f 40 00                 	nop	dword ptr [rax]
141ea9080: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141ea9085: 55                          	push	rbp
141ea9086: 41 57                       	push	r15
141ea9088: 41 56                       	push	r14
141ea908a: 41 54                       	push	r12
141ea908c: 56                          	push	rsi
141ea908d: 57                          	push	rdi
141ea908e: 53                          	push	rbx
141ea908f: 48 83 ec 30                 	sub	rsp, 0x30
141ea9093: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141ea909a: 48 83 7d 00 00              	cmp	qword ptr [rbp], 0x0
141ea909f: 74 17                       	je	0x141ea90b8 <.text+0x1ea80b8>
141ea90a1: 48 8b 55 00                 	mov	rdx, qword ptr [rbp]
141ea90a5: 48 c1 e2 03                 	shl	rdx, 0x3
141ea90a9: 41 b8 04 00 00 00           	mov	r8d, 0x4
141ea90af: 48 8b 4d f8                 	mov	rcx, qword ptr [rbp - 0x8]
141ea90b3: e8 68 ab 76 fe              	call	0x140613c20 <.text+0x612c20>
141ea90b8: 90                          	nop
141ea90b9: 48 83 c4 30                 	add	rsp, 0x30
141ea90bd: 5b                          	pop	rbx
141ea90be: 5f                          	pop	rdi
141ea90bf: 5e                          	pop	rsi
141ea90c0: 41 5c                       	pop	r12
141ea90c2: 41 5e                       	pop	r14
141ea90c4: 41 5f                       	pop	r15
141ea90c6: 5d                          	pop	rbp
141ea90c7: c3                          	ret

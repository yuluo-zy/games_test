
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409c3a80: 55                          	push	rbp
1409c3a81: 41 57                       	push	r15
1409c3a83: 41 56                       	push	r14
1409c3a85: 41 55                       	push	r13
1409c3a87: 41 54                       	push	r12
1409c3a89: 56                          	push	rsi
1409c3a8a: 57                          	push	rdi
1409c3a8b: 53                          	push	rbx
1409c3a8c: 48 81 ec 48 0a 00 00        	sub	rsp, 0xa48
1409c3a93: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1409c3a9b: 48 c7 85 c0 09 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x9c0], -0x2
1409c3aa6: 4c 89 cb                    	mov	rbx, r9
1409c3aa9: 4c 89 c6                    	mov	rsi, r8
1409c3aac: 49 89 d7                    	mov	r15, rdx
1409c3aaf: 48 89 cf                    	mov	rdi, rcx
1409c3ab2: 49 8d 49 04                 	lea	rcx, [r9 + 0x4]
1409c3ab6: e8 d5 1c 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409c3abb: c7 85 b8 03 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x3b8], 0x0
1409c3ac5: 48 89 b5 c0 03 00 00        	mov	qword ptr [rbp + 0x3c0], rsi
1409c3acc: 48 c7 85 98 08 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x898], 0x1
1409c3ad7: 48 89 9d 98 09 00 00        	mov	qword ptr [rbp + 0x998], rbx
1409c3ade: 48 89 9d a0 08 00 00        	mov	qword ptr [rbp + 0x8a0], rbx
1409c3ae5: 48 c7 85 a8 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8a8], 0x0
1409c3af0: 48 89 7c 24 40              	mov	qword ptr [rsp + 0x40], rdi
1409c3af5: 4c 89 7c 24 38              	mov	qword ptr [rsp + 0x38], r15
1409c3afa: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1409c3aff: 48 8d 85 98 08 00 00        	lea	rax, [rbp + 0x898]
1409c3b06: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1409c3b0b: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1409c3b14: 48 8d 15 ab c3 0f 02        	lea	rdx, [rip + 0x20fc3ab]  # 0x142abfec6
1409c3b1b: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
1409c3b1f: 4c 8d 8d b8 03 00 00        	lea	r9, [rbp + 0x3b8]
1409c3b26: 41 b8 14 00 00 00           	mov	r8d, 0x14
1409c3b2c: e8 3f 3e ea ff              	call	0x140867970 <.text+0x866970>
1409c3b31: 4c 89 bd b0 09 00 00        	mov	qword ptr [rbp + 0x9b0], r15
1409c3b38: 49 8d 4f 38                 	lea	rcx, [r15 + 0x38]
1409c3b3c: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c3b43: 48 ba 29 f7 80 5b 19 62 c3 4b       	movabs	rdx, 0x4bc362195b80f729
1409c3b4d: 49 b8 fb ce cb 80 ce 98 27 98       	movabs	r8, -0x67d867317f343105
1409c3b57: e8 94 9b 71 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1409c3b5c: 48 85 c0                    	test	rax, rax
1409c3b5f: 48 8b 8d b0 09 00 00        	mov	rcx, qword ptr [rbp + 0x9b0]
1409c3b66: 0f 84 ef 03 00 00           	je	0x1409c3f5b <.text+0x9c2f5b>
1409c3b6c: 48 39 91 18 02 00 00        	cmp	qword ptr [rcx + 0x218], rdx
1409c3b73: 0f 86 e2 03 00 00           	jbe	0x1409c3f5b <.text+0x9c2f5b>
1409c3b79: 48 8b 81 10 02 00 00        	mov	rax, qword ptr [rcx + 0x210]
1409c3b80: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1409c3b84: 48 85 c0                    	test	rax, rax
1409c3b87: 0f 84 ce 03 00 00           	je	0x1409c3f5b <.text+0x9c2f5b>
1409c3b8d: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
1409c3b94: 48 f7 d0                    	not	rax
1409c3b97: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
1409c3b9b: 48 c1 e2 05                 	shl	rdx, 0x5
1409c3b9f: 48 83 7c 11 20 00           	cmp	qword ptr [rcx + rdx + 0x20], 0x0
1409c3ba5: 0f 84 b0 03 00 00           	je	0x1409c3f5b <.text+0x9c2f5b>
1409c3bab: 48 8b 85 48 0a 00 00        	mov	rax, qword ptr [rbp + 0xa48]
1409c3bb2: 48 01 d1                    	add	rcx, rdx
1409c3bb5: 48 8b 49 10                 	mov	rcx, qword ptr [rcx + 0x10]
1409c3bb9: 48 8b 11                    	mov	rdx, qword ptr [rcx]
1409c3bbc: 48 85 d2                    	test	rdx, rdx
1409c3bbf: 78 07                       	js	0x1409c3bc8 <.text+0x9c2bc8>
1409c3bc1: f3 48 0f 2a c2              	cvtsi2ss	xmm0, rdx
1409c3bc6: eb 15                       	jmp	0x1409c3bdd <.text+0x9c2bdd>
1409c3bc8: 49 89 d0                    	mov	r8, rdx
1409c3bcb: 49 d1 e8                    	shr	r8
1409c3bce: 83 e2 01                    	and	edx, 0x1
1409c3bd1: 4c 09 c2                    	or	rdx, r8
1409c3bd4: f3 48 0f 2a c2              	cvtsi2ss	xmm0, rdx
1409c3bd9: f3 0f 58 c0                 	addss	xmm0, xmm0
1409c3bdd: f3 0f 2a 49 08              	cvtsi2ss	xmm1, dword ptr [rcx + 0x8]
1409c3be2: f3 0f 5e 0d 56 3a fe 01     	divss	xmm1, dword ptr [rip + 0x1fe3a56] # 0x1429a7640
1409c3bea: f3 0f 58 c8                 	addss	xmm1, xmm0
1409c3bee: f3 0f 11 8d a0 09 00 00     	movss	dword ptr [rbp + 0x9a0], xmm1
1409c3bf6: c6 85 a4 09 00 00 00        	mov	byte ptr [rbp + 0x9a4], 0x0
1409c3bfd: 4c 8b b8 88 01 00 00        	mov	r15, qword ptr [rax + 0x188]
1409c3c04: 48 8b 80 90 01 00 00        	mov	rax, qword ptr [rax + 0x190]
1409c3c0b: 48 c1 e0 07                 	shl	rax, 0x7
1409c3c0f: 4c 01 f8                    	add	rax, r15
1409c3c12: 48 89 85 90 09 00 00        	mov	qword ptr [rbp + 0x990], rax
1409c3c19: 4c 8d a5 a8 07 00 00        	lea	r12, [rbp + 0x7a8]
1409c3c20: 4c 8d ad c0 03 00 00        	lea	r13, [rbp + 0x3c0]
1409c3c27: 4c 8d b5 a8 09 00 00        	lea	r14, [rbp + 0x9a8]
1409c3c2e: 4c 3b bd 90 09 00 00        	cmp	r15, qword ptr [rbp + 0x990]
1409c3c35: 0f 84 ce 02 00 00           	je	0x1409c3f09 <.text+0x9c2f09>
1409c3c3b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409c3c40: 49 8b 5f 50                 	mov	rbx, qword ptr [r15 + 0x50]
1409c3c44: 49 8b 7f 58                 	mov	rdi, qword ptr [r15 + 0x58]
1409c3c48: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c3c4f: e8 fc 2a 22 00              	call	0x140be6750 <.text+0xbe5750>
1409c3c54: 89 c6                       	mov	esi, eax
1409c3c56: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c3c5d: 31 c9                       	xor	ecx, ecx
1409c3c5f: e8 bc 5e 71 ff              	call	0x1400d9b20 <.text+0xd8b20>
1409c3c64: 48 85 c0                    	test	rax, rax
1409c3c67: 0f 84 d9 02 00 00           	je	0x1409c3f46 <.text+0x9c2f46>
1409c3c6d: 89 b5 40 09 00 00           	mov	dword ptr [rbp + 0x940], esi
1409c3c73: 66 c7 85 48 09 00 00 00 00  	mov	word ptr [rbp + 0x948], 0x0
1409c3c7c: c7 85 44 09 00 00 00 00 80 3f       	mov	dword ptr [rbp + 0x944], 0x3f800000
1409c3c86: 48 c7 85 98 08 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x898], 0x1
1409c3c91: 48 89 9d a0 08 00 00        	mov	qword ptr [rbp + 0x8a0], rbx
1409c3c98: 48 89 bd 38 09 00 00        	mov	qword ptr [rbp + 0x938], rdi
1409c3c9f: 48 c7 85 a8 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8a8], 0x0
1409c3caa: 48 c7 85 b0 08 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x8b0], 0x4
1409c3cb5: 48 c7 85 b8 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8b8], 0x0
1409c3cc0: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1409c3cc3: 48 ff 00                    	inc	qword ptr [rax]
1409c3cc6: 0f 10 0d 63 2a 0d 02        	movups	xmm1, xmmword ptr [rip + 0x20d2a63] # 0x142a96730
1409c3ccd: 48 8d 85 08 09 00 00        	lea	rax, [rbp + 0x908]
1409c3cd4: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
1409c3cd8: 0f 10 0d 41 2a 0d 02        	movups	xmm1, xmmword ptr [rip + 0x20d2a41] # 0x142a96720
1409c3cdf: 0f 11 08                    	movups	xmmword ptr [rax], xmm1
1409c3ce2: 0f 11 85 28 09 00 00        	movups	xmmword ptr [rbp + 0x928], xmm0
1409c3ce9: 48 b8 03 00 00 00 01 00 00 00       	movabs	rax, 0x100000003
1409c3cf3: 48 89 85 d8 08 00 00        	mov	qword ptr [rbp + 0x8d8], rax
1409c3cfa: 48 c7 85 c0 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8c0], 0x0
1409c3d05: 48 c7 85 c8 08 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x8c8], 0x1
1409c3d10: 48 c7 85 d0 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8d0], 0x0
1409c3d1b: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c3d22: 48 8d 8d b8 03 00 00        	lea	rcx, [rbp + 0x3b8]
1409c3d29: 48 8d 95 98 08 00 00        	lea	rdx, [rbp + 0x898]
1409c3d30: 4c 8d 45 d0                 	lea	r8, [rbp - 0x30]
1409c3d34: 45 31 c9                    	xor	r9d, r9d
1409c3d37: e8 a4 1e f4 ff              	call	0x140905be0 <.text+0x904be0>
1409c3d3c: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c3d43: 48 8d 8d 98 08 00 00        	lea	rcx, [rbp + 0x898]
1409c3d4a: 48 8d 95 b8 03 00 00        	lea	rdx, [rbp + 0x3b8]
1409c3d51: 4c 8d 85 a0 09 00 00        	lea	r8, [rbp + 0x9a0]
1409c3d58: 4d 89 f9                    	mov	r9, r15
1409c3d5b: e8 00 77 f4 ff              	call	0x14090b460 <.text+0x90a460>
1409c3d60: 48 8b 85 98 09 00 00        	mov	rax, qword ptr [rbp + 0x998]
1409c3d67: 48 89 85 a8 09 00 00        	mov	qword ptr [rbp + 0x9a8], rax
1409c3d6e: c7 85 b8 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x3b8], 0x2
1409c3d78: 48 8d 9d b8 03 00 00        	lea	rbx, [rbp + 0x3b8]
1409c3d7f: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1409c3d84: 4c 89 f7                    	mov	rdi, r14
1409c3d87: 4c 89 f1                    	mov	rcx, r14
1409c3d8a: 48 8d b5 98 08 00 00        	lea	rsi, [rbp + 0x898]
1409c3d91: 48 89 f2                    	mov	rdx, rsi
1409c3d94: 49 b8 fc fb 49 a0 9d 57 61 48       	movabs	r8, 0x4861579da049fbfc
1409c3d9e: 41 b1 1d                    	mov	r9b, 0x1d
1409c3da1: e8 5a aa f2 ff              	call	0x1408ee800 <.text+0x8ed800>
1409c3da6: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409c3dac: 48 89 d9                    	mov	rcx, rbx
1409c3daf: 48 89 f2                    	mov	rdx, rsi
1409c3db2: e8 49 fe f5 01              	call	0x142923c00 <.text+0x2922c00>
1409c3db7: 48 8b 85 30 0a 00 00        	mov	rax, qword ptr [rbp + 0xa30]
1409c3dbe: 48 89 85 a8 09 00 00        	mov	qword ptr [rbp + 0x9a8], rax
1409c3dc5: c7 85 98 08 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x898], 0x2
1409c3dcf: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c3dd4: 48 89 f9                    	mov	rcx, rdi
1409c3dd7: 48 89 da                    	mov	rdx, rbx
1409c3dda: 49 b8 30 82 02 28 3a 3c 33 2a       	movabs	r8, 0x2a333c3a28028230
1409c3de4: 41 b1 1d                    	mov	r9b, 0x1d
1409c3de7: e8 14 aa f2 ff              	call	0x1408ee800 <.text+0x8ed800>
1409c3dec: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409c3df2: 48 8d bd a0 07 00 00        	lea	rdi, [rbp + 0x7a0]
1409c3df9: 48 89 f9                    	mov	rcx, rdi
1409c3dfc: 48 89 da                    	mov	rdx, rbx
1409c3dff: e8 fc fd f5 01              	call	0x142923c00 <.text+0x2922c00>
1409c3e04: 48 8b 85 38 0a 00 00        	mov	rax, qword ptr [rbp + 0xa38]
1409c3e0b: 48 89 85 98 08 00 00        	mov	qword ptr [rbp + 0x898], rax
1409c3e12: c7 85 b8 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x3b8], 0x2
1409c3e1c: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1409c3e21: 48 89 f1                    	mov	rcx, rsi
1409c3e24: 48 89 fa                    	mov	rdx, rdi
1409c3e27: 49 b8 d0 09 6c 82 85 51 17 04       	movabs	r8, 0x4175185826c09d0
1409c3e31: 41 b1 1d                    	mov	r9b, 0x1d
1409c3e34: e8 c7 a9 f2 ff              	call	0x1408ee800 <.text+0x8ed800>
1409c3e39: 48 8b b5 a0 07 00 00        	mov	rsi, qword ptr [rbp + 0x7a0]
1409c3e40: 41 b8 a0 00 00 00           	mov	r8d, 0xa0
1409c3e46: 4c 89 e9                    	mov	rcx, r13
1409c3e49: 4c 89 e2                    	mov	rdx, r12
1409c3e4c: e8 af fd f5 01              	call	0x142923c00 <.text+0x2922c00>
1409c3e51: 0f b6 85 48 08 00 00        	movzx	eax, byte ptr [rbp + 0x848]
1409c3e58: 0f b7 8d 49 08 00 00        	movzx	ecx, word ptr [rbp + 0x849]
1409c3e5f: 0f b6 95 4b 08 00 00        	movzx	edx, byte ptr [rbp + 0x84b]
1409c3e66: 41 0f 10 84 24 a4 00 00 00  	movups	xmm0, xmmword ptr [r12 + 0xa4]
1409c3e6f: 41 0f 10 8c 24 b4 00 00 00  	movups	xmm1, xmmword ptr [r12 + 0xb4]
1409c3e78: 41 0f 10 94 24 c4 00 00 00  	movups	xmm2, xmmword ptr [r12 + 0xc4]
1409c3e81: 41 0f 10 9c 24 d4 00 00 00  	movups	xmm3, xmmword ptr [r12 + 0xd4]
1409c3e8a: 41 0f 11 85 a4 00 00 00     	movups	xmmword ptr [r13 + 0xa4], xmm0
1409c3e92: 41 0f 11 8d b4 00 00 00     	movups	xmmword ptr [r13 + 0xb4], xmm1
1409c3e9a: 41 0f 11 95 c4 00 00 00     	movups	xmmword ptr [r13 + 0xc4], xmm2
1409c3ea2: 41 0f 11 9d d4 00 00 00     	movups	xmmword ptr [r13 + 0xd4], xmm3
1409c3eaa: 41 0f 10 84 24 e0 00 00 00  	movups	xmm0, xmmword ptr [r12 + 0xe0]
1409c3eb3: 41 0f 11 85 e0 00 00 00     	movups	xmmword ptr [r13 + 0xe0], xmm0
1409c3ebb: 24 bf                       	and	al, -0x41
1409c3ebd: 80 e2 9f                    	and	dl, -0x61
1409c3ec0: 80 ca 40                    	or	dl, 0x40
1409c3ec3: 4d 8b 4f 70                 	mov	r9, qword ptr [r15 + 0x70]
1409c3ec7: 48 89 b5 b8 03 00 00        	mov	qword ptr [rbp + 0x3b8], rsi
1409c3ece: 88 85 60 04 00 00           	mov	byte ptr [rbp + 0x460], al
1409c3ed4: 66 89 8d 61 04 00 00        	mov	word ptr [rbp + 0x461], cx
1409c3edb: 88 95 63 04 00 00           	mov	byte ptr [rbp + 0x463], dl
1409c3ee1: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c3ee8: 48 89 d9                    	mov	rcx, rbx
1409c3eeb: 4c 89 fa                    	mov	rdx, r15
1409c3eee: 4c 8b 85 b0 09 00 00        	mov	r8, qword ptr [rbp + 0x9b0]
1409c3ef5: 41 ff d1                    	call	r9
1409c3ef8: 49 83 ef 80                 	sub	r15, -0x80
1409c3efc: 4c 3b bd 90 09 00 00        	cmp	r15, qword ptr [rbp + 0x990]
1409c3f03: 0f 85 37 fd ff ff           	jne	0x1409c3c40 <.text+0x9c2c40>
1409c3f09: 48 8d b5 b8 03 00 00        	lea	rsi, [rbp + 0x3b8]
1409c3f10: 48 8d 55 d0                 	lea	rdx, [rbp - 0x30]
1409c3f14: 41 b8 e8 03 00 00           	mov	r8d, 0x3e8
1409c3f1a: 48 89 f1                    	mov	rcx, rsi
1409c3f1d: e8 de fc f5 01              	call	0x142923c00 <.text+0x2922c00>
1409c3f22: c6 85 bf 09 00 00 00        	mov	byte ptr [rbp + 0x9bf], 0x0
1409c3f29: 48 89 f1                    	mov	rcx, rsi
1409c3f2c: e8 3f 44 ea ff              	call	0x140868370 <.text+0x867370>
1409c3f31: 90                          	nop
1409c3f32: 48 81 c4 48 0a 00 00        	add	rsp, 0xa48
1409c3f39: 5b                          	pop	rbx
1409c3f3a: 5f                          	pop	rdi
1409c3f3b: 5e                          	pop	rsi
1409c3f3c: 41 5c                       	pop	r12
1409c3f3e: 41 5d                       	pop	r13
1409c3f40: 41 5e                       	pop	r14
1409c3f42: 41 5f                       	pop	r15
1409c3f44: 5d                          	pop	rbp
1409c3f45: c3                          	ret
1409c3f46: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c3f4d: 48 8d 0d 4c 6b 27 02        	lea	rcx, [rip + 0x2276b4c]  # 0x142c3aaa0
1409c3f54: e8 77 41 f1 01              	call	0x1428d80d0 <.text+0x28d70d0>
1409c3f59: eb 13                       	jmp	0x1409c3f6e <.text+0x9c2f6e>
1409c3f5b: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c3f62: 48 8d 0d 6f 1c 0e 02        	lea	rcx, [rip + 0x20e1c6f]  # 0x142aa5bd8
1409c3f69: e8 a2 53 f1 01              	call	0x1428d9310 <.text+0x28d8310>
1409c3f6e: 0f 0b                       	ud2
1409c3f70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3f75: 55                          	push	rbp
1409c3f76: 41 57                       	push	r15
1409c3f78: 41 56                       	push	r14
1409c3f7a: 41 55                       	push	r13
1409c3f7c: 41 54                       	push	r12
1409c3f7e: 56                          	push	rsi
1409c3f7f: 57                          	push	rdi
1409c3f80: 53                          	push	rbx
1409c3f81: 48 83 ec 48                 	sub	rsp, 0x48
1409c3f85: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c3f8c: 80 bd bf 09 00 00 00        	cmp	byte ptr [rbp + 0x9bf], 0x0
1409c3f93: 75 11                       	jne	0x1409c3fa6 <.text+0x9c2fa6>
1409c3f95: 48 83 c4 48                 	add	rsp, 0x48
1409c3f99: 5b                          	pop	rbx
1409c3f9a: 5f                          	pop	rdi
1409c3f9b: 5e                          	pop	rsi
1409c3f9c: 41 5c                       	pop	r12
1409c3f9e: 41 5d                       	pop	r13
1409c3fa0: 41 5e                       	pop	r14
1409c3fa2: 41 5f                       	pop	r15
1409c3fa4: 5d                          	pop	rbp
1409c3fa5: c3                          	ret
1409c3fa6: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
1409c3faa: e8 11 e4 ec ff              	call	0x1408923c0 <.text+0x8913c0>
1409c3faf: eb e4                       	jmp	0x1409c3f95 <.text+0x9c2f95>
1409c3fb1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1409c3fc0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3fc5: 55                          	push	rbp
1409c3fc6: 41 57                       	push	r15
1409c3fc8: 41 56                       	push	r14
1409c3fca: 41 55                       	push	r13
1409c3fcc: 41 54                       	push	r12
1409c3fce: 56                          	push	rsi
1409c3fcf: 57                          	push	rdi
1409c3fd0: 53                          	push	rbx
1409c3fd1: 48 83 ec 48                 	sub	rsp, 0x48
1409c3fd5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c3fdc: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c3fe3: 48 8d 8d 98 08 00 00        	lea	rcx, [rbp + 0x898]
1409c3fea: e8 d1 f5 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409c3fef: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c3ff6: 48 83 c4 48                 	add	rsp, 0x48
1409c3ffa: 5b                          	pop	rbx
1409c3ffb: 5f                          	pop	rdi
1409c3ffc: 5e                          	pop	rsi
1409c3ffd: 41 5c                       	pop	r12
1409c3fff: 41 5d                       	pop	r13
1409c4001: 41 5e                       	pop	r14
1409c4003: 41 5f                       	pop	r15
1409c4005: 5d                          	pop	rbp
1409c4006: c3                          	ret
1409c4007: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c4010: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c4015: 55                          	push	rbp
1409c4016: 41 57                       	push	r15
1409c4018: 41 56                       	push	r14
1409c401a: 41 55                       	push	r13
1409c401c: 41 54                       	push	r12
1409c401e: 56                          	push	rsi
1409c401f: 57                          	push	rdi
1409c4020: 53                          	push	rbx
1409c4021: 48 83 ec 48                 	sub	rsp, 0x48
1409c4025: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c402c: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c4033: 48 8d 8d b8 03 00 00        	lea	rcx, [rbp + 0x3b8]
1409c403a: e8 81 f5 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409c403f: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c4046: 48 83 c4 48                 	add	rsp, 0x48
1409c404a: 5b                          	pop	rbx
1409c404b: 5f                          	pop	rdi
1409c404c: 5e                          	pop	rsi
1409c404d: 41 5c                       	pop	r12
1409c404f: 41 5d                       	pop	r13
1409c4051: 41 5e                       	pop	r14
1409c4053: 41 5f                       	pop	r15
1409c4055: 5d                          	pop	rbp
1409c4056: c3                          	ret
1409c4057: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c4060: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c4065: 55                          	push	rbp
1409c4066: 41 57                       	push	r15
1409c4068: 41 56                       	push	r14
1409c406a: 41 55                       	push	r13
1409c406c: 41 54                       	push	r12
1409c406e: 56                          	push	rsi
1409c406f: 57                          	push	rdi
1409c4070: 53                          	push	rbx
1409c4071: 48 83 ec 48                 	sub	rsp, 0x48
1409c4075: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c407c: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c4083: 48 8d 8d a0 07 00 00        	lea	rcx, [rbp + 0x7a0]
1409c408a: e8 31 f5 ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409c408f: c6 85 bf 09 00 00 01        	mov	byte ptr [rbp + 0x9bf], 0x1
1409c4096: 48 83 c4 48                 	add	rsp, 0x48
1409c409a: 5b                          	pop	rbx
1409c409b: 5f                          	pop	rdi
1409c409c: 5e                          	pop	rsi
1409c409d: 41 5c                       	pop	r12
1409c409f: 41 5d                       	pop	r13
1409c40a1: 41 5e                       	pop	r14
1409c40a3: 41 5f                       	pop	r15
1409c40a5: 5d                          	pop	rbp
1409c40a6: c3                          	ret

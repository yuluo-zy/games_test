
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141226cc0: 4c 39 e0                    	cmpq	%r12, %rax
141226cc3: 0f 84 98 00 00 00           	je	0x141226d61 <.text+0x1225d61>
141226cc9: 4c 8d 68 14                 	leaq	0x14(%rax), %r13
141226ccd: 80 78 11 00                 	cmpb	$0x0, 0x11(%rax)
141226cd1: 4c 89 e8                    	movq	%r13, %rax
141226cd4: 75 ea                       	jne	0x141226cc0 <.text+0x1225cc0>
141226cd6: 4d 8d 75 ec                 	leaq	-0x14(%r13), %r14
141226cda: 4c 89 f1                    	movq	%r14, %rcx
141226cdd: e8 de 93 01 00              	callq	0x1412400c0 <.text+0x123f0c0>
141226ce2: 0f 28 f0                    	movaps	%xmm0, %xmm6
141226ce5: 0f 28 f9                    	movaps	%xmm1, %xmm7
141226ce8: 4c 89 f1                    	movq	%r14, %rcx
141226ceb: e8 00 94 01 00              	callq	0x1412400f0 <.text+0x123f0f0>
141226cf0: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
141226cf4: 44 0f 28 c9                 	movaps	%xmm1, %xmm9
141226cf8: 48 89 da                    	movq	%rbx, %rdx
141226cfb: 48 3b 9d 70 01 00 00        	cmpq	0x170(%rbp), %rbx
141226d02: 75 2b                       	jne	0x141226d2f <.text+0x1225d2f>
141226d04: 48 c7 44 24 20 10 00 00 00  	movq	$0x10, 0x20(%rsp)
141226d0d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141226d13: 41 b9 04 00 00 00           	movl	$0x4, %r9d
141226d19: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
141226d20: e8 bb 10 68 01              	callq	0x1428a7de0 <.text+0x28a6de0>
141226d25: 48 8b bd 78 01 00 00        	movq	0x178(%rbp), %rdi
141226d2c: 48 89 da                    	movq	%rbx, %rdx
141226d2f: 48 89 d0                    	movq	%rdx, %rax
141226d32: 48 c1 e0 04                 	shlq	$0x4, %rax
141226d36: f3 0f 11 34 07              	movss	%xmm6, (%rdi,%rax)
141226d3b: f3 0f 11 7c 07 04           	movss	%xmm7, 0x4(%rdi,%rax)
141226d41: f3 44 0f 11 44 07 08        	movss	%xmm8, 0x8(%rdi,%rax)
141226d48: f3 44 0f 11 4c 07 0c        	movss	%xmm9, 0xc(%rdi,%rax)
141226d4f: 48 ff c2                    	incq	%rdx
141226d52: 48 89 d3                    	movq	%rdx, %rbx
141226d55: 48 89 95 80 01 00 00        	movq	%rdx, 0x180(%rbp)
141226d5c: e9 51 ff ff ff              	jmp	0x141226cb2 <.text+0x1225cb2>
141226d61: 83 bd 98 03 00 00 02        	cmpl	$0x2, 0x398(%rbp)
141226d68: 75 16                       	jne	0x141226d80 <.text+0x1225d80>
141226d6a: 4c 8b 8d 70 01 00 00        	movq	0x170(%rbp), %r9
141226d71: 4c 8b 95 78 01 00 00        	movq	0x178(%rbp), %r10
141226d78: 49 89 dd                    	movq	%rbx, %r13
141226d7b: e9 c8 00 00 00              	jmp	0x141226e48 <.text+0x1225e48>
141226d80: 4c 8b 8d 70 01 00 00        	movq	0x170(%rbp), %r9
141226d87: 4c 8b 95 78 01 00 00        	movq	0x178(%rbp), %r10
141226d8e: 48 85 db                    	testq	%rbx, %rbx
141226d91: 0f 84 a9 00 00 00           	je	0x141226e40 <.text+0x1225e40>
141226d97: 31 c0                       	xorl	%eax, %eax
141226d99: f3 0f 10 05 63 eb 6f 01     	movss	0x16feb63(%rip), %xmm0  # 0x142925904
141226da1: f3 0f 10 0d 3b 33 73 01     	movss	0x173333b(%rip), %xmm1  # 0x14295a0e4
141226da9: 0f 57 d2                    	xorps	%xmm2, %xmm2
141226dac: 4c 89 d2                    	movq	%r10, %rdx
141226daf: 48 89 d9                    	movq	%rbx, %rcx
141226db2: eb 35                       	jmp	0x141226de9 <.text+0x1225de9>
141226db4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141226dc0: 49 89 c8                    	movq	%rcx, %r8
141226dc3: 49 c1 e0 04                 	shlq	$0x4, %r8
141226dc7: f3 43 0f 11 1c 02           	movss	%xmm3, (%r10,%r8)
141226dcd: 43 0f 13 64 02 04           	movlps	%xmm4, 0x4(%r10,%r8)
141226dd3: f3 43 0f 11 6c 02 0c        	movss	%xmm5, 0xc(%r10,%r8)
141226dda: 48 ff c1                    	incq	%rcx
141226ddd: 48 ff c0                    	incq	%rax
141226de0: 48 83 c2 10                 	addq	$0x10, %rdx
141226de4: 48 39 c3                    	cmpq	%rax, %rbx
141226de7: 74 5c                       	je	0x141226e45 <.text+0x1225e45>
141226de9: f3 0f 10 1a                 	movss	(%rdx), %xmm3
141226ded: f2 0f 10 62 04              	movsd	0x4(%rdx), %xmm4
141226df2: f3 0f 16 f4                 	movshdup	%xmm4, %xmm6            # xmm6 = xmm4[1,1,3,3]
141226df6: f3 0f 10 6a 0c              	movss	0xc(%rdx), %xmm5
141226dfb: 0f 28 fb                    	movaps	%xmm3, %xmm7
141226dfe: f3 0f 58 fe                 	addss	%xmm6, %xmm7
141226e02: f3 0f 58 f8                 	addss	%xmm0, %xmm7
141226e06: 41 0f 2e fb                 	ucomiss	%xmm11, %xmm7
141226e0a: 76 14                       	jbe	0x141226e20 <.text+0x1225e20>
141226e0c: 4c 39 c9                    	cmpq	%r9, %rcx
141226e0f: 0f 83 3a 21 00 00           	jae	0x141228f4f <.text+0x1227f4f>
141226e15: f3 41 0f 5c db              	subss	%xmm11, %xmm3
141226e1a: eb a4                       	jmp	0x141226dc0 <.text+0x1225dc0>
141226e1c: 0f 1f 40 00                 	nopl	(%rax)
141226e20: 0f 28 fb                    	movaps	%xmm3, %xmm7
141226e23: f3 0f 5c fe                 	subss	%xmm6, %xmm7
141226e27: f3 0f 58 f9                 	addss	%xmm1, %xmm7
141226e2b: 0f 2e d7                    	ucomiss	%xmm7, %xmm2
141226e2e: 76 ad                       	jbe	0x141226ddd <.text+0x1225ddd>
141226e30: 4c 39 c9                    	cmpq	%r9, %rcx
141226e33: 0f 83 50 21 00 00           	jae	0x141228f89 <.text+0x1227f89>
141226e39: f3 41 0f 58 db              	addss	%xmm11, %xmm3
141226e3e: eb 80                       	jmp	0x141226dc0 <.text+0x1225dc0>
141226e40: 45 31 ed                    	xorl	%r13d, %r13d
141226e43: eb 03                       	jmp	0x141226e48 <.text+0x1225e48>
141226e45: 49 89 cd                    	movq	%rcx, %r13

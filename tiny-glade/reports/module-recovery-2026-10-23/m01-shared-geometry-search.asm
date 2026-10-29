
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140ad2ce0: 41 57                       	pushq	%r15
140ad2ce2: 41 56                       	pushq	%r14
140ad2ce4: 41 55                       	pushq	%r13
140ad2ce6: 41 54                       	pushq	%r12
140ad2ce8: 56                          	pushq	%rsi
140ad2ce9: 57                          	pushq	%rdi
140ad2cea: 55                          	pushq	%rbp
140ad2ceb: 53                          	pushq	%rbx
140ad2cec: 48 81 ec f8 00 00 00        	subq	$0xf8, %rsp
140ad2cf3: 0f 29 b4 24 e0 00 00 00     	movaps	%xmm6, 0xe0(%rsp)
140ad2cfb: 4c 89 cf                    	movq	%r9, %rdi
140ad2cfe: 48 89 d3                    	movq	%rdx, %rbx
140ad2d01: 48 89 ce                    	movq	%rcx, %rsi
140ad2d04: 45 8b 30                    	movl	(%r8), %r14d
140ad2d07: 4d 8d 68 04                 	leaq	0x4(%r8), %r13
140ad2d0b: 49 8b 40 14                 	movq	0x14(%r8), %rax
140ad2d0f: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140ad2d14: 41 0f 10 40 04              	movups	0x4(%r8), %xmm0
140ad2d19: 0f 29 44 24 50              	movaps	%xmm0, 0x50(%rsp)
140ad2d1e: 48 8d 6c 24 74              	leaq	0x74(%rsp), %rbp
140ad2d23: 41 0f b6 01                 	movzbl	(%r9), %eax
140ad2d27: 48 83 c7 04                 	addq	$0x4, %rdi
140ad2d2b: 41 f6 c6 01                 	testb	$0x1, %r14b
140ad2d2f: 0f 84 87 01 00 00           	je	0x140ad2ebc <.text+0xad1ebc>
140ad2d35: a8 01                       	testb	$0x1, %al
140ad2d37: 0f 84 ec 02 00 00           	je	0x140ad3029 <.text+0xad2029>
140ad2d3d: 49 8b 45 10                 	movq	0x10(%r13), %rax
140ad2d41: 48 89 84 24 b0 00 00 00     	movq	%rax, 0xb0(%rsp)
140ad2d49: 41 0f 10 45 00              	movups	(%r13), %xmm0
140ad2d4e: 0f 29 84 24 a0 00 00 00     	movaps	%xmm0, 0xa0(%rsp)
140ad2d56: 48 8b 47 10                 	movq	0x10(%rdi), %rax
140ad2d5a: 48 89 84 24 d0 00 00 00     	movq	%rax, 0xd0(%rsp)
140ad2d62: 0f 10 07                    	movups	(%rdi), %xmm0
140ad2d65: 0f 29 84 24 c0 00 00 00     	movaps	%xmm0, 0xc0(%rsp)
140ad2d6d: 48 8d 4c 24 70              	leaq	0x70(%rsp), %rcx
140ad2d72: 48 8d 94 24 a0 00 00 00     	leaq	0xa0(%rsp), %rdx
140ad2d7a: 4c 8d 84 24 c0 00 00 00     	leaq	0xc0(%rsp), %r8
140ad2d82: 0f 57 db                    	xorps	%xmm3, %xmm3
140ad2d85: e8 a6 99 1c 00              	callq	0x140c9c730 <.text+0xc9b730>
140ad2d8a: 48 8b 84 24 80 00 00 00     	movq	0x80(%rsp), %rax
140ad2d92: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140ad2d97: 0f 10 44 24 70              	movups	0x70(%rsp), %xmm0
140ad2d9c: 0f 29 44 24 30              	movaps	%xmm0, 0x30(%rsp)
140ad2da1: c7 44 24 70 01 00 00 00     	movl	$0x1, 0x70(%rsp)
140ad2da9: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140ad2dad: 0f 11 45 00                 	movups	%xmm0, (%rbp)
140ad2db1: 48 8d 54 24 70              	leaq	0x70(%rsp), %rdx
140ad2db6: 48 89 d9                    	movq	%rbx, %rcx
140ad2db9: e8 b2 fb ff ff              	callq	0x140ad2970 <.text+0xad1970>
140ad2dbe: 84 c0                       	testb	%al, %al
140ad2dc0: 0f 84 2f 02 00 00           	je	0x140ad2ff5 <.text+0xad1ff5>
140ad2dc6: 48 89 74 24 28              	movq	%rsi, 0x28(%rsp)
140ad2dcb: 0f 28 44 24 30              	movaps	0x30(%rsp), %xmm0
140ad2dd0: 0f 29 44 24 50              	movaps	%xmm0, 0x50(%rsp)
140ad2dd5: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
140ad2dda: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140ad2ddf: be 01 00 00 00              	movl	$0x1, %esi
140ad2de4: f3 0f 10 35 44 cc fa 01     	movss	0x1facc44(%rip), %xmm6  # 0x142a7fa30
140ad2dec: 4c 8d 74 24 70              	leaq	0x70(%rsp), %r14
140ad2df1: 4c 8d bc 24 a0 00 00 00     	leaq	0xa0(%rsp), %r15
140ad2df9: 4c 8d a4 24 c0 00 00 00     	leaq	0xc0(%rsp), %r12
140ad2e01: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140ad2e10: 0f 57 db                    	xorps	%xmm3, %xmm3
140ad2e13: f3 48 0f 2a de              	cvtsi2ss	%rsi, %xmm3
140ad2e18: f3 0f 5e de                 	divss	%xmm6, %xmm3
140ad2e1c: 49 8b 45 10                 	movq	0x10(%r13), %rax
140ad2e20: 48 89 84 24 b0 00 00 00     	movq	%rax, 0xb0(%rsp)
140ad2e28: 41 0f 10 45 00              	movups	(%r13), %xmm0
140ad2e2d: 0f 29 84 24 a0 00 00 00     	movaps	%xmm0, 0xa0(%rsp)
140ad2e35: 48 8b 47 10                 	movq	0x10(%rdi), %rax
140ad2e39: 48 89 84 24 d0 00 00 00     	movq	%rax, 0xd0(%rsp)
140ad2e41: 0f 10 07                    	movups	(%rdi), %xmm0
140ad2e44: 0f 29 84 24 c0 00 00 00     	movaps	%xmm0, 0xc0(%rsp)
140ad2e4c: 4c 89 f1                    	movq	%r14, %rcx
140ad2e4f: 4c 89 fa                    	movq	%r15, %rdx
140ad2e52: 4d 89 e0                    	movq	%r12, %r8
140ad2e55: e8 d6 98 1c 00              	callq	0x140c9c730 <.text+0xc9b730>
140ad2e5a: 48 8b 84 24 80 00 00 00     	movq	0x80(%rsp), %rax
140ad2e62: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140ad2e67: 0f 10 44 24 70              	movups	0x70(%rsp), %xmm0
140ad2e6c: 0f 29 44 24 30              	movaps	%xmm0, 0x30(%rsp)
140ad2e71: c7 44 24 70 01 00 00 00     	movl	$0x1, 0x70(%rsp)
140ad2e79: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140ad2e7d: 0f 11 45 00                 	movups	%xmm0, (%rbp)
140ad2e81: 48 89 d9                    	movq	%rbx, %rcx
140ad2e84: 4c 89 f2                    	movq	%r14, %rdx
140ad2e87: e8 e4 fa ff ff              	callq	0x140ad2970 <.text+0xad1970>
140ad2e8c: 84 c0                       	testb	%al, %al
140ad2e8e: 74 21                       	je	0x140ad2eb1 <.text+0xad1eb1>
140ad2e90: 48 ff c6                    	incq	%rsi
140ad2e93: 0f 28 44 24 30              	movaps	0x30(%rsp), %xmm0
140ad2e98: 0f 29 44 24 50              	movaps	%xmm0, 0x50(%rsp)
140ad2e9d: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
140ad2ea2: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140ad2ea7: 48 83 fe 14                 	cmpq	$0x14, %rsi
140ad2eab: 0f 85 5f ff ff ff           	jne	0x140ad2e10 <.text+0xad1e10>
140ad2eb1: 41 be 01 00 00 00           	movl	$0x1, %r14d
140ad2eb7: e9 34 01 00 00              	jmp	0x140ad2ff0 <.text+0xad1ff0>
140ad2ebc: a8 01                       	testb	$0x1, %al
140ad2ebe: 0f 85 65 01 00 00           	jne	0x140ad3029 <.text+0xad2029>
140ad2ec4: 41 0f 10 45 00              	movups	(%r13), %xmm0
140ad2ec9: 0f 29 84 24 a0 00 00 00     	movaps	%xmm0, 0xa0(%rsp)
140ad2ed1: 0f 10 07                    	movups	(%rdi), %xmm0
140ad2ed4: 0f 29 84 24 c0 00 00 00     	movaps	%xmm0, 0xc0(%rsp)
140ad2edc: 48 8d 4c 24 70              	leaq	0x70(%rsp), %rcx
140ad2ee1: 48 8d 94 24 a0 00 00 00     	leaq	0xa0(%rsp), %rdx
140ad2ee9: 4c 8d 84 24 c0 00 00 00     	leaq	0xc0(%rsp), %r8
140ad2ef1: 0f 57 db                    	xorps	%xmm3, %xmm3
140ad2ef4: e8 77 52 1c 00              	callq	0x140c98170 <.text+0xc97170>
140ad2ef9: 0f 10 44 24 70              	movups	0x70(%rsp), %xmm0
140ad2efe: 0f 29 44 24 30              	movaps	%xmm0, 0x30(%rsp)
140ad2f03: c7 44 24 70 00 00 00 00     	movl	$0x0, 0x70(%rsp)
140ad2f0b: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
140ad2f10: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140ad2f14: 0f 11 45 00                 	movups	%xmm0, (%rbp)
140ad2f18: 48 8d 54 24 70              	leaq	0x70(%rsp), %rdx
140ad2f1d: 48 89 d9                    	movq	%rbx, %rcx
140ad2f20: e8 4b fa ff ff              	callq	0x140ad2970 <.text+0xad1970>
140ad2f25: 84 c0                       	testb	%al, %al
140ad2f27: 0f 84 c8 00 00 00           	je	0x140ad2ff5 <.text+0xad1ff5>
140ad2f2d: 48 89 74 24 28              	movq	%rsi, 0x28(%rsp)
140ad2f32: 0f 28 44 24 30              	movaps	0x30(%rsp), %xmm0
140ad2f37: 0f 29 44 24 50              	movaps	%xmm0, 0x50(%rsp)
140ad2f3c: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
140ad2f41: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140ad2f46: be 01 00 00 00              	movl	$0x1, %esi
140ad2f4b: f3 0f 10 35 dd ca fa 01     	movss	0x1facadd(%rip), %xmm6  # 0x142a7fa30
140ad2f53: 4c 8d 74 24 70              	leaq	0x70(%rsp), %r14
140ad2f58: 4c 8d bc 24 a0 00 00 00     	leaq	0xa0(%rsp), %r15
140ad2f60: 4c 8d a4 24 c0 00 00 00     	leaq	0xc0(%rsp), %r12
140ad2f68: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140ad2f70: 0f 57 db                    	xorps	%xmm3, %xmm3
140ad2f73: f3 48 0f 2a de              	cvtsi2ss	%rsi, %xmm3
140ad2f78: f3 0f 5e de                 	divss	%xmm6, %xmm3
140ad2f7c: 41 0f 10 45 00              	movups	(%r13), %xmm0
140ad2f81: 0f 29 84 24 a0 00 00 00     	movaps	%xmm0, 0xa0(%rsp)
140ad2f89: 0f 10 07                    	movups	(%rdi), %xmm0
140ad2f8c: 0f 29 84 24 c0 00 00 00     	movaps	%xmm0, 0xc0(%rsp)
140ad2f94: 4c 89 f1                    	movq	%r14, %rcx
140ad2f97: 4c 89 fa                    	movq	%r15, %rdx
140ad2f9a: 4d 89 e0                    	movq	%r12, %r8
140ad2f9d: e8 ce 51 1c 00              	callq	0x140c98170 <.text+0xc97170>
140ad2fa2: 0f 10 44 24 70              	movups	0x70(%rsp), %xmm0
140ad2fa7: 0f 29 44 24 30              	movaps	%xmm0, 0x30(%rsp)
140ad2fac: c7 44 24 70 00 00 00 00     	movl	$0x0, 0x70(%rsp)
140ad2fb4: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
140ad2fb9: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140ad2fbd: 0f 11 45 00                 	movups	%xmm0, (%rbp)
140ad2fc1: 48 89 d9                    	movq	%rbx, %rcx
140ad2fc4: 4c 89 f2                    	movq	%r14, %rdx
140ad2fc7: e8 a4 f9 ff ff              	callq	0x140ad2970 <.text+0xad1970>
140ad2fcc: 84 c0                       	testb	%al, %al
140ad2fce: 74 1d                       	je	0x140ad2fed <.text+0xad1fed>
140ad2fd0: 48 ff c6                    	incq	%rsi
140ad2fd3: 0f 28 44 24 30              	movaps	0x30(%rsp), %xmm0
140ad2fd8: 0f 29 44 24 50              	movaps	%xmm0, 0x50(%rsp)
140ad2fdd: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
140ad2fe2: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140ad2fe7: 48 83 fe 14                 	cmpq	$0x14, %rsi
140ad2feb: 75 83                       	jne	0x140ad2f70 <.text+0xad1f70>
140ad2fed: 45 31 f6                    	xorl	%r14d, %r14d
140ad2ff0: 48 8b 74 24 28              	movq	0x28(%rsp), %rsi
140ad2ff5: 44 89 36                    	movl	%r14d, (%rsi)
140ad2ff8: 0f 28 44 24 50              	movaps	0x50(%rsp), %xmm0
140ad2ffd: 0f 11 46 04                 	movups	%xmm0, 0x4(%rsi)
140ad3001: 48 8b 44 24 60              	movq	0x60(%rsp), %rax
140ad3006: 48 89 46 14                 	movq	%rax, 0x14(%rsi)
140ad300a: 48 89 f0                    	movq	%rsi, %rax
140ad300d: 0f 28 b4 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm6
140ad3015: 48 81 c4 f8 00 00 00        	addq	$0xf8, %rsp
140ad301c: 5b                          	popq	%rbx
140ad301d: 5d                          	popq	%rbp
140ad301e: 5f                          	popq	%rdi
140ad301f: 5e                          	popq	%rsi
140ad3020: 41 5c                       	popq	%r12
140ad3022: 41 5d                       	popq	%r13
140ad3024: 41 5e                       	popq	%r14
140ad3026: 41 5f                       	popq	%r15
140ad3028: c3                          	retq
140ad3029: 48 8d 05 d8 e3 00 02        	leaq	0x200e3d8(%rip), %rax   # 0x142ae1408
140ad3030: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
140ad3035: 48 c7 44 24 78 01 00 00 00  	movq	$0x1, 0x78(%rsp)
140ad303e: 48 8d 44 24 20              	leaq	0x20(%rsp), %rax
140ad3043: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
140ad304b: 0f 57 c0                    	xorps	%xmm0, %xmm0
140ad304e: 0f 11 84 24 88 00 00 00     	movups	%xmm0, 0x88(%rsp)
140ad3056: 48 8d 15 f3 e3 00 02        	leaq	0x200e3f3(%rip), %rdx   # 0x142ae1450
140ad305d: 48 8d 4c 24 70              	leaq	0x70(%rsp), %rcx
140ad3062: e8 29 63 e0 01              	callq	0x1428d9390 <.text+0x28d8390>

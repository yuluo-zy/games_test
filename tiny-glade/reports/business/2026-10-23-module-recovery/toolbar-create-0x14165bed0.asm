
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14165bed0: 41 57                       	pushq	%r15
14165bed2: 41 56                       	pushq	%r14
14165bed4: 41 55                       	pushq	%r13
14165bed6: 41 54                       	pushq	%r12
14165bed8: 56                          	pushq	%rsi
14165bed9: 57                          	pushq	%rdi
14165beda: 55                          	pushq	%rbp
14165bedb: 53                          	pushq	%rbx
14165bedc: 48 81 ec f8 01 00 00        	subq	$0x1f8, %rsp            # imm = 0x1F8
14165bee3: 44 0f 29 84 24 e0 01 00 00  	movaps	%xmm8, 0x1e0(%rsp)
14165beec: 0f 29 bc 24 d0 01 00 00     	movaps	%xmm7, 0x1d0(%rsp)
14165bef4: 0f 29 b4 24 c0 01 00 00     	movaps	%xmm6, 0x1c0(%rsp)
14165befc: 49 89 d4                    	movq	%rdx, %r12
14165beff: 48 8b 84 24 68 02 00 00     	movq	0x268(%rsp), %rax
14165bf07: 48 8b 68 20                 	movq	0x20(%rax), %rbp
14165bf0b: 48 8b 50 30                 	movq	0x30(%rax), %rdx
14165bf0f: 44 8b 50 3c                 	movl	0x3c(%rax), %r10d
14165bf13: 48 8b 09                    	movq	(%rcx), %rcx
14165bf16: 83 39 06                    	cmpl	$0x6, (%rcx)
14165bf19: 0f 85 90 05 00 00           	jne	0x14165c4af <.text+0x165b4af>
14165bf1f: 80 79 04 01                 	cmpb	$0x1, 0x4(%rcx)
14165bf23: 0f 85 86 05 00 00           	jne	0x14165c4af <.text+0x165b4af>
14165bf29: 4c 89 8c 24 f8 00 00 00     	movq	%r9, 0xf8(%rsp)
14165bf31: 44 89 54 24 54              	movl	%r10d, 0x54(%rsp)
14165bf36: 48 89 94 24 90 00 00 00     	movq	%rdx, 0x90(%rsp)
14165bf3e: 4c 89 44 24 28              	movq	%r8, 0x28(%rsp)
14165bf43: 48 8b 9c 24 88 02 00 00     	movq	0x288(%rsp), %rbx
14165bf4b: 48 8b 8c 24 70 02 00 00     	movq	0x270(%rsp), %rcx
14165bf53: 48 8b 94 24 90 02 00 00     	movq	0x290(%rsp), %rdx
14165bf5b: 4c 8b 28                    	movq	(%rax), %r13
14165bf5e: 48 8b 41 20                 	movq	0x20(%rcx), %rax
14165bf62: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
14165bf6a: 48 8b 43 20                 	movq	0x20(%rbx), %rax
14165bf6e: 48 89 84 24 d0 00 00 00     	movq	%rax, 0xd0(%rsp)
14165bf76: 48 8b 43 30                 	movq	0x30(%rbx), %rax
14165bf7a: 48 89 84 24 00 01 00 00     	movq	%rax, 0x100(%rsp)
14165bf82: 8b 43 3c                    	movl	0x3c(%rbx), %eax
14165bf85: 89 84 24 b0 00 00 00        	movl	%eax, 0xb0(%rsp)
14165bf8c: 48 8b 43 40                 	movq	0x40(%rbx), %rax
14165bf90: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
14165bf95: 48 8b 32                    	movq	(%rdx), %rsi
14165bf98: 48 8b 42 20                 	movq	0x20(%rdx), %rax
14165bf9c: 48 89 84 24 98 00 00 00     	movq	%rax, 0x98(%rsp)
14165bfa4: 48 8b 7a 30                 	movq	0x30(%rdx), %rdi
14165bfa8: 8b 42 3c                    	movl	0x3c(%rdx), %eax
14165bfab: 89 84 24 ac 00 00 00        	movl	%eax, 0xac(%rsp)
14165bfb2: 48 8b 42 40                 	movq	0x40(%rdx), %rax
14165bfb6: 48 89 84 24 e8 00 00 00     	movq	%rax, 0xe8(%rsp)
14165bfbe: 48 8b 42 50                 	movq	0x50(%rdx), %rax
14165bfc2: 48 89 84 24 d8 00 00 00     	movq	%rax, 0xd8(%rsp)
14165bfca: 8b 42 5c                    	movl	0x5c(%rdx), %eax
14165bfcd: 89 84 24 a0 00 00 00        	movl	%eax, 0xa0(%rsp)
14165bfd4: 48 8b 09                    	movq	(%rcx), %rcx
14165bfd7: 48 c7 84 24 10 01 00 00 00 00 00 00 	movq	$0x0, 0x110(%rsp)
14165bfe3: 48 c7 84 24 18 01 00 00 08 00 00 00 	movq	$0x8, 0x118(%rsp)
14165bfef: 48 c7 84 24 20 01 00 00 00 00 00 00 	movq	$0x0, 0x120(%rsp)
14165bffb: c7 84 24 28 01 00 00 00 00 00 00    	movl	$0x0, 0x128(%rsp)
14165c006: c7 84 24 40 01 00 00 00 00 00 00    	movl	$0x0, 0x140(%rsp)
14165c011: c7 84 24 58 01 00 00 00 00 00 00    	movl	$0x0, 0x158(%rsp)
14165c01c: c7 84 24 70 01 00 00 00 00 00 00    	movl	$0x0, 0x170(%rsp)
14165c027: c6 84 24 88 01 00 00 00     	movb	$0x0, 0x188(%rsp)
14165c02f: c6 84 24 98 01 00 00 05     	movb	$0x5, 0x198(%rsp)
14165c037: c7 84 24 9c 01 00 00 ff ff 7f 7f    	movl	$0x7f7fffff, 0x19c(%rsp) # imm = 0x7F7FFFFF
14165c042: 66 c7 84 24 b8 01 00 00 00 01       	movw	$0x100, 0x1b8(%rsp) # imm = 0x100
14165c04c: 48 8b 43 50                 	movq	0x50(%rbx), %rax
14165c050: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
14165c055: 8b 43 5c                    	movl	0x5c(%rbx), %eax
14165c058: 89 44 24 24                 	movl	%eax, 0x24(%rsp)
14165c05c: 48 8b 43 60                 	movq	0x60(%rbx), %rax
14165c060: 48 89 84 24 f0 00 00 00     	movq	%rax, 0xf0(%rsp)
14165c068: 48 8b 43 70                 	movq	0x70(%rbx), %rax
14165c06c: 48 89 84 24 e0 00 00 00     	movq	%rax, 0xe0(%rsp)
14165c074: 8b 43 7c                    	movl	0x7c(%rbx), %eax
14165c077: 89 84 24 a4 00 00 00        	movl	%eax, 0xa4(%rsp)
14165c07e: 48 8b 83 80 00 00 00        	movq	0x80(%rbx), %rax
14165c085: 48 89 84 24 c0 00 00 00     	movq	%rax, 0xc0(%rsp)
14165c08d: 4c 8b b3 90 00 00 00        	movq	0x90(%rbx), %r14
14165c094: 8b 83 9c 00 00 00           	movl	0x9c(%rbx), %eax
14165c09a: 89 84 24 a8 00 00 00        	movl	%eax, 0xa8(%rsp)
14165c0a1: 48 8b 83 a0 00 00 00        	movq	0xa0(%rbx), %rax
14165c0a8: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
14165c0ad: 48 8b 83 b0 00 00 00        	movq	0xb0(%rbx), %rax
14165c0b4: 48 89 84 24 c8 00 00 00     	movq	%rax, 0xc8(%rsp)
14165c0bc: 8b 83 bc 00 00 00           	movl	0xbc(%rbx), %eax
14165c0c2: 89 44 24 6c                 	movl	%eax, 0x6c(%rsp)
14165c0c6: 48 8d 15 2c 61 62 01        	leaq	0x162612c(%rip), %rdx   # 0x142c821f9
14165c0cd: 4c 8d 8c 24 10 01 00 00     	leaq	0x110(%rsp), %r9
14165c0d5: 41 b8 12 00 00 00           	movl	$0x12, %r8d
14165c0db: e8 f0 75 2c ff              	callq	0x1409236d0 <.text+0x9226d0>
14165c0e0: 49 89 c1                    	movq	%rax, %r9
14165c0e3: 49 89 d2                    	movq	%rdx, %r10
14165c0e6: f3 41 0f 10 45 3c           	movss	0x3c(%r13), %xmm0
14165c0ec: 0f 2e 05 d9 57 37 01        	ucomiss	0x13757d9(%rip), %xmm0  # 0x1429d18cc
14165c0f3: 4c 89 ac 24 88 00 00 00     	movq	%r13, 0x88(%rsp)
14165c0fb: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
14165c100: 0f 82 a8 00 00 00           	jb	0x14165c1ae <.text+0x165b1ae>
14165c106: 48 89 f3                    	movq	%rsi, %rbx
14165c109: 48 8b 84 24 98 02 00 00     	movq	0x298(%rsp), %rax
14165c111: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14165c115: 0f 29 84 24 10 01 00 00     	movaps	%xmm0, 0x110(%rsp)
14165c11d: 8b 48 1c                    	movl	0x1c(%rax), %ecx
14165c120: 48 8b 50 10                 	movq	0x10(%rax), %rdx
14165c124: 89 0a                       	movl	%ecx, (%rdx)
14165c126: 4c 8b 38                    	movq	(%rax), %r15
14165c129: 4d 8b 6f 30                 	movq	0x30(%r15), %r13
14165c12d: 49 8b 77 40                 	movq	0x40(%r15), %rsi
14165c131: 4d 3b 6f 20                 	cmpq	0x20(%r15), %r13
14165c135: 75 31                       	jne	0x14165c168 <.text+0x165b168>
14165c137: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
14165c13b: 48 8d 15 06 e0 5d 01        	leaq	0x15de006(%rip), %rdx   # 0x142c3a148
14165c142: 4c 89 74 24 48              	movq	%r14, 0x48(%rsp)
14165c147: 4d 89 e6                    	movq	%r12, %r14
14165c14a: 49 89 fc                    	movq	%rdi, %r12
14165c14d: 4c 89 d7                    	movq	%r10, %rdi
14165c150: e8 5b 69 f0 fe              	callq	0x140562ab0 <.text+0x561ab0>
14165c155: 49 89 fa                    	movq	%rdi, %r10
14165c158: 4c 89 e7                    	movq	%r12, %rdi
14165c15b: 4d 89 f4                    	movq	%r14, %r12
14165c15e: 4c 8b 74 24 48              	movq	0x48(%rsp), %r14
14165c163: 4c 8b 4c 24 60              	movq	0x60(%rsp), %r9
14165c168: 49 8b 47 28                 	movq	0x28(%r15), %rax
14165c16c: 4a 8d 0c ad 00 00 00 00     	leaq	(,%r13,4), %rcx
14165c174: 4c 01 e9                    	addq	%r13, %rcx
14165c177: 48 89 34 c8                 	movq	%rsi, (%rax,%rcx,8)
14165c17b: c7 44 c8 08 00 00 00 00     	movl	$0x0, 0x8(%rax,%rcx,8)
14165c183: 0f 28 84 24 10 01 00 00     	movaps	0x110(%rsp), %xmm0
14165c18b: 0f 11 44 c8 0c              	movups	%xmm0, 0xc(%rax,%rcx,8)
14165c190: 49 ff c5                    	incq	%r13
14165c193: 4d 89 6f 30                 	movq	%r13, 0x30(%r15)
14165c197: 49 ff 47 40                 	incq	0x40(%r15)
14165c19b: 4c 8b ac 24 88 00 00 00     	movq	0x88(%rsp), %r13
14165c1a3: 48 89 de                    	movq	%rbx, %rsi
14165c1a6: 48 8b 9c 24 88 02 00 00     	movq	0x288(%rsp), %rbx
14165c1ae: 4c 8b 9c 24 78 02 00 00     	movq	0x278(%rsp), %r11
14165c1b6: 41 b7 02                    	movb	$0x2, %r15b
14165c1b9: 41 80 79 18 02              	cmpb	$0x2, 0x18(%r9)
14165c1be: 0f 82 5f 02 00 00           	jb	0x14165c423 <.text+0x165b423>
14165c1c4: 4d 39 51 38                 	cmpq	%r10, 0x38(%r9)
14165c1c8: 0f 85 c6 00 00 00           	jne	0x14165c294 <.text+0x165b294>
14165c1ce: 41 80 79 19 03              	cmpb	$0x3, 0x19(%r9)
14165c1d3: 0f 85 bb 00 00 00           	jne	0x14165c294 <.text+0x165b294>
14165c1d9: 48 89 74 24 48              	movq	%rsi, 0x48(%rsp)
14165c1de: 41 8b 4b 1c                 	movl	0x1c(%r11), %ecx
14165c1e2: 49 8b 53 10                 	movq	0x10(%r11), %rdx
14165c1e6: 89 0a                       	movl	%ecx, (%rdx)
14165c1e8: 4d 8b 03                    	movq	(%r11), %r8
14165c1eb: 41 c6 40 0c 00              	movb	$0x0, 0xc(%r8)
14165c1f0: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
14165c1f5: 48 8b 00                    	movq	(%rax), %rax
14165c1f8: f2 0f 10 00                 	movsd	(%rax), %xmm0
14165c1fc: f2 41 0f 11 40 04           	movsd	%xmm0, 0x4(%r8)
14165c202: 89 0a                       	movl	%ecx, (%rdx)
14165c204: 41 c7 00 00 00 00 00        	movl	$0x0, (%r8)
14165c20b: 48 8b 84 24 c8 00 00 00     	movq	0xc8(%rsp), %rax
14165c213: 8b 4c 24 6c                 	movl	0x6c(%rsp), %ecx
14165c217: 89 08                       	movl	%ecx, (%rax)
14165c219: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
14165c21e: 48 8b 71 30                 	movq	0x30(%rcx), %rsi
14165c222: 4c 8b 69 40                 	movq	0x40(%rcx), %r13
14165c226: 48 3b 71 20                 	cmpq	0x20(%rcx), %rsi
14165c22a: 75 3d                       	jne	0x14165c269 <.text+0x165b269>
14165c22c: 48 8b 44 24 58              	movq	0x58(%rsp), %rax
14165c231: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14165c235: 48 8d 15 0c df 5d 01        	leaq	0x15ddf0c(%rip), %rdx   # 0x142c3a148
14165c23c: 4c 89 64 24 28              	movq	%r12, 0x28(%rsp)
14165c241: 49 89 fc                    	movq	%rdi, %r12
14165c244: 4c 89 d7                    	movq	%r10, %rdi
14165c247: e8 24 66 f0 fe              	callq	0x140562870 <.text+0x561870>
14165c24c: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
14165c251: 4c 8b 9c 24 78 02 00 00     	movq	0x278(%rsp), %r11
14165c259: 49 89 fa                    	movq	%rdi, %r10
14165c25c: 4c 89 e7                    	movq	%r12, %rdi
14165c25f: 4c 8b 64 24 28              	movq	0x28(%rsp), %r12
14165c264: 4c 8b 4c 24 60              	movq	0x60(%rsp), %r9
14165c269: 48 8b 41 28                 	movq	0x28(%rcx), %rax
14165c26d: 4c 89 2c f0                 	movq	%r13, (%rax,%rsi,8)
14165c271: 48 ff c6                    	incq	%rsi
14165c274: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
14165c278: 48 ff 41 40                 	incq	0x40(%rcx)
14165c27c: 41 80 79 18 02              	cmpb	$0x2, 0x18(%r9)
14165c281: 4c 8b ac 24 88 00 00 00     	movq	0x88(%rsp), %r13
14165c289: 48 8b 74 24 48              	movq	0x48(%rsp), %rsi
14165c28e: 0f 82 8f 01 00 00           	jb	0x14165c423 <.text+0x165b423>
14165c294: 4d 39 51 38                 	cmpq	%r10, 0x38(%r9)
14165c298: 0f 85 cd 00 00 00           	jne	0x14165c36b <.text+0x165b36b>
14165c29e: 41 80 79 19 04              	cmpb	$0x4, 0x19(%r9)
14165c2a3: 0f 85 c2 00 00 00           	jne	0x14165c36b <.text+0x165b36b>
14165c2a9: 48 89 74 24 48              	movq	%rsi, 0x48(%rsp)
14165c2ae: 48 89 7c 24 28              	movq	%rdi, 0x28(%rsp)
14165c2b3: 49 8b 04 24                 	movq	(%r12), %rax
14165c2b7: f2 0f 10 80 40 01 00 00     	movsd	0x140(%rax), %xmm0
14165c2bf: 0f 59 c0                    	mulps	%xmm0, %xmm0
14165c2c2: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
14165c2c6: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14165c2ca: 0f 57 c0                    	xorps	%xmm0, %xmm0
14165c2cd: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
14165c2d1: 41 8b 43 1c                 	movl	0x1c(%r11), %eax
14165c2d5: 49 8b 4b 10                 	movq	0x10(%r11), %rcx
14165c2d9: 89 01                       	movl	%eax, (%rcx)
14165c2db: 49 8b 13                    	movq	(%r11), %rdx
14165c2de: f3 0f 58 02                 	addss	(%rdx), %xmm0
14165c2e2: f3 0f 11 02                 	movss	%xmm0, (%rdx)
14165c2e6: 0f 2e 05 a7 83 2d 01        	ucomiss	0x12d83a7(%rip), %xmm0  # 0x142934694
14165c2ed: 76 06                       	jbe	0x14165c2f5 <.text+0x165b2f5>
14165c2ef: 89 01                       	movl	%eax, (%rcx)
14165c2f1: c6 42 0c 01                 	movb	$0x1, 0xc(%rdx)
14165c2f5: 48 8b 84 24 c8 00 00 00     	movq	0xc8(%rsp), %rax
14165c2fd: 8b 4c 24 6c                 	movl	0x6c(%rsp), %ecx
14165c301: 89 08                       	movl	%ecx, (%rax)
14165c303: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
14165c308: 48 8b 71 30                 	movq	0x30(%rcx), %rsi
14165c30c: 48 8b 79 40                 	movq	0x40(%rcx), %rdi
14165c310: 48 3b 71 20                 	cmpq	0x20(%rcx), %rsi
14165c314: 75 2d                       	jne	0x14165c343 <.text+0x165b343>
14165c316: 48 8b 44 24 58              	movq	0x58(%rsp), %rax
14165c31b: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14165c31f: 48 8d 15 22 de 5d 01        	leaq	0x15dde22(%rip), %rdx   # 0x142c3a148
14165c326: 4d 89 d4                    	movq	%r10, %r12
14165c329: e8 42 65 f0 fe              	callq	0x140562870 <.text+0x561870>
14165c32e: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
14165c333: 4c 8b 9c 24 78 02 00 00     	movq	0x278(%rsp), %r11
14165c33b: 4d 89 e2                    	movq	%r12, %r10
14165c33e: 4c 8b 4c 24 60              	movq	0x60(%rsp), %r9
14165c343: 48 8b 41 28                 	movq	0x28(%rcx), %rax
14165c347: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
14165c34b: 48 ff c6                    	incq	%rsi
14165c34e: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
14165c352: 48 ff 41 40                 	incq	0x40(%rcx)
14165c356: 41 80 79 18 02              	cmpb	$0x2, 0x18(%r9)
14165c35b: 48 8b 7c 24 28              	movq	0x28(%rsp), %rdi
14165c360: 48 8b 74 24 48              	movq	0x48(%rsp), %rsi
14165c365: 0f 82 b8 00 00 00           	jb	0x14165c423 <.text+0x165b423>
14165c36b: 4d 39 51 38                 	cmpq	%r10, 0x38(%r9)
14165c36f: 0f 85 a3 00 00 00           	jne	0x14165c418 <.text+0x165b418>
14165c375: 41 80 79 19 01              	cmpb	$0x1, 0x19(%r9)
14165c37a: 0f 87 98 00 00 00           	ja	0x14165c418 <.text+0x165b418>
14165c380: 48 89 eb                    	movq	%rbp, %rbx
14165c383: 48 89 f5                    	movq	%rsi, %rbp
14165c386: 49 89 fc                    	movq	%rdi, %r12
14165c389: 49 8b 03                    	movq	(%r11), %rax
14165c38c: f2 0f 10 70 04              	movsd	0x4(%rax), %xmm6
14165c391: 8b 84 24 a8 00 00 00        	movl	0xa8(%rsp), %eax
14165c398: 41 89 06                    	movl	%eax, (%r14)
14165c39b: 48 8b 94 24 c0 00 00 00     	movq	0xc0(%rsp), %rdx
14165c3a3: 48 8b 7a 30                 	movq	0x30(%rdx), %rdi
14165c3a7: 48 8b 72 40                 	movq	0x40(%rdx), %rsi
14165c3ab: 48 3b 7a 20                 	cmpq	0x20(%rdx), %rdi
14165c3af: 75 2b                       	jne	0x14165c3dc <.text+0x165b3dc>
14165c3b1: 48 8d 4a 20                 	leaq	0x20(%rdx), %rcx
14165c3b5: 48 8d 15 8c dd 5d 01        	leaq	0x15ddd8c(%rip), %rdx   # 0x142c3a148
14165c3bc: 4d 89 d6                    	movq	%r10, %r14
14165c3bf: e8 ec 69 f0 fe              	callq	0x140562db0 <.text+0x561db0>
14165c3c4: 4c 8b 9c 24 78 02 00 00     	movq	0x278(%rsp), %r11
14165c3cc: 48 8b 94 24 c0 00 00 00     	movq	0xc0(%rsp), %rdx
14165c3d4: 4d 89 f2                    	movq	%r14, %r10
14165c3d7: 4c 8b 4c 24 60              	movq	0x60(%rsp), %r9
14165c3dc: 48 8b 42 28                 	movq	0x28(%rdx), %rax
14165c3e0: 48 8d 0c 7f                 	leaq	(%rdi,%rdi,2), %rcx
14165c3e4: 48 89 34 c8                 	movq	%rsi, (%rax,%rcx,8)
14165c3e8: c7 44 c8 08 02 00 00 00     	movl	$0x2, 0x8(%rax,%rcx,8)
14165c3f0: 0f 13 74 c8 0c              	movlps	%xmm6, 0xc(%rax,%rcx,8)
14165c3f5: 48 ff c7                    	incq	%rdi
14165c3f8: 48 89 7a 30                 	movq	%rdi, 0x30(%rdx)
14165c3fc: 48 ff 42 40                 	incq	0x40(%rdx)
14165c400: 41 80 79 18 02              	cmpb	$0x2, 0x18(%r9)
14165c405: 4c 89 e7                    	movq	%r12, %rdi
14165c408: 48 89 ee                    	movq	%rbp, %rsi
14165c40b: 48 89 dd                    	movq	%rbx, %rbp
14165c40e: 48 8b 9c 24 88 02 00 00     	movq	0x288(%rsp), %rbx
14165c416: 72 0b                       	jb	0x14165c423 <.text+0x165b423>
14165c418: 4d 39 51 38                 	cmpq	%r10, 0x38(%r9)
14165c41c: 75 05                       	jne	0x14165c423 <.text+0x165b423>
14165c41e: 45 0f b6 79 19              	movzbl	0x19(%r9), %r15d
14165c423: 41 80 c7 fe                 	addb	$-0x2, %r15b
14165c427: 41 0f b6 cf                 	movzbl	%r15b, %ecx
14165c42b: 80 f9 03                    	cmpb	$0x3, %cl
14165c42e: b8 03 00 00 00              	movl	$0x3, %eax
14165c433: 0f 42 c1                    	cmovbl	%ecx, %eax
14165c436: 3c 02                       	cmpb	$0x2, %al
14165c438: 4c 89 94 24 08 01 00 00     	movq	%r10, 0x108(%rsp)
14165c440: 73 7c                       	jae	0x14165c4be <.text+0x165b4be>
14165c442: f3 41 0f 10 45 30           	movss	0x30(%r13), %xmm0
14165c448: f3 41 0f 10 4d 38           	movss	0x38(%r13), %xmm1
14165c44e: 48 8b 84 24 90 00 00 00     	movq	0x90(%rsp), %rax
14165c456: 8b 4c 24 54                 	movl	0x54(%rsp), %ecx
14165c45a: 89 08                       	movl	%ecx, (%rax)
14165c45c: f3 0f 11 45 10              	movss	%xmm0, 0x10(%rbp)
14165c461: f3 0f 11 4d 14              	movss	%xmm1, 0x14(%rbp)
14165c466: f3 41 0f 10 45 34           	movss	0x34(%r13), %xmm0
14165c46c: 89 08                       	movl	%ecx, (%rax)
14165c46e: f3 0f 11 45 20              	movss	%xmm0, 0x20(%rbp)
14165c473: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14165c477: 0f 29 44 24 30              	movaps	%xmm0, 0x30(%rsp)
14165c47c: 4c 8d bc 24 10 01 00 00     	leaq	0x110(%rsp), %r15
14165c484: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
14165c489: 41 b4 01                    	movb	$0x1, %r12b
14165c48c: 4c 89 f9                    	movq	%r15, %rcx
14165c48f: 41 b0 01                    	movb	$0x1, %r8b
14165c492: e8 a9 08 00 00              	callq	0x14165cd40 <.text+0x165bd40>
14165c497: 48 8b 8c 24 80 00 00 00     	movq	0x80(%rsp), %rcx
14165c49f: 4c 89 fa                    	movq	%r15, %rdx
14165c4a2: e8 d9 03 e1 ff              	callq	0x14146c880 <.text+0x146b880>
14165c4a7: 0f 57 ff                    	xorps	%xmm7, %xmm7
14165c4aa: e9 29 03 00 00              	jmp	0x14165c7d8 <.text+0x165b7d8>
14165c4af: 44 89 12                    	movl	%r10d, (%rdx)
14165c4b2: c7 45 00 00 00 00 00        	movl	$0x0, (%rbp)
14165c4b9: e9 54 08 00 00              	jmp	0x14165cd12 <.text+0x165bd12>
14165c4be: 0f b6 c0                    	movzbl	%al, %eax
14165c4c1: 83 f8 02                    	cmpl	$0x2, %eax
14165c4c4: 0f 85 e5 00 00 00           	jne	0x14165c5af <.text+0x165b5af>
14165c4ca: 49 8b 03                    	movq	(%r11), %rax
14165c4cd: 0f 57 ff                    	xorps	%xmm7, %xmm7
14165c4d0: 41 b4 01                    	movb	$0x1, %r12b
14165c4d3: 80 78 0c 00                 	cmpb	$0x0, 0xc(%rax)
14165c4d7: 41 b0 01                    	movb	$0x1, %r8b
14165c4da: 0f 84 98 00 00 00           	je	0x14165c578 <.text+0x165b578>
14165c4e0: 48 8b 8c 24 f8 00 00 00     	movq	0xf8(%rsp), %rcx
14165c4e8: 48 8b 09                    	movq	(%rcx), %rcx
14165c4eb: f3 41 0f 10 45 34           	movss	0x34(%r13), %xmm0
14165c4f1: f3 0f 5c 41 04              	subss	0x4(%rcx), %xmm0
14165c4f6: f3 0f 5e 41 10              	divss	0x10(%rcx), %xmm0
14165c4fb: f2 0f 10 4d 10              	movsd	0x10(%rbp), %xmm1
14165c500: f3 0f 10 11                 	movss	(%rcx), %xmm2
14165c504: f3 0f 10 59 08              	movss	0x8(%rcx), %xmm3
14165c509: 0f 14 d3                    	unpcklps	%xmm3, %xmm2            # xmm2 = xmm2[0],xmm3[0],xmm2[1],xmm3[1]
14165c50c: f3 0f 10 59 0c              	movss	0xc(%rcx), %xmm3
14165c511: f3 0f 10 61 14              	movss	0x14(%rcx), %xmm4
14165c516: 0f 14 dc                    	unpcklps	%xmm4, %xmm3            # xmm3 = xmm3[0],xmm4[0],xmm3[1],xmm4[1]
14165c519: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
14165c51d: 0f 59 c3                    	mulps	%xmm3, %xmm0
14165c520: 0f 58 c2                    	addps	%xmm2, %xmm0
14165c523: 0f 5c c8                    	subps	%xmm0, %xmm1
14165c526: 0f 59 c9                    	mulps	%xmm1, %xmm1
14165c529: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14165c52d: f3 0f 58 c1                 	addss	%xmm1, %xmm0
14165c531: f3 0f 51 c0                 	sqrtss	%xmm0, %xmm0
14165c535: f3 0f 10 0d 43 03 42 01     	movss	0x1420343(%rip), %xmm1  # 0x142a7c880
14165c53d: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
14165c541: f3 0f 10 05 bb 80 2d 01     	movss	0x12d80bb(%rip), %xmm0  # 0x142934604
14165c549: f3 0f 5d c1                 	minss	%xmm1, %xmm0
14165c54d: 0f 28 f8                    	movaps	%xmm0, %xmm7
14165c550: f3 0f 5c 7d 18              	subss	0x18(%rbp), %xmm7
14165c555: 48 8b 8c 24 90 00 00 00     	movq	0x90(%rsp), %rcx
14165c55d: 8b 54 24 54                 	movl	0x54(%rsp), %edx
14165c561: 89 11                       	movl	%edx, (%rcx)
14165c563: f3 0f 11 45 18              	movss	%xmm0, 0x18(%rbp)
14165c568: 0f 54 3d 11 24 2d 01        	andps	0x12d2411(%rip), %xmm7  # 0x14292e980
14165c56f: 44 0f b6 40 0c              	movzbl	0xc(%rax), %r8d
14165c574: 41 80 f0 01                 	xorb	$0x1, %r8b
14165c578: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14165c57c: 0f 29 44 24 30              	movaps	%xmm0, 0x30(%rsp)
14165c581: 41 80 e0 01                 	andb	$0x1, %r8b
14165c585: 4c 8d bc 24 10 01 00 00     	leaq	0x110(%rsp), %r15
14165c58d: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
14165c592: 4c 89 f9                    	movq	%r15, %rcx
14165c595: e8 a6 07 00 00              	callq	0x14165cd40 <.text+0x165bd40>
14165c59a: 48 8b 8c 24 80 00 00 00     	movq	0x80(%rsp), %rcx
14165c5a2: 4c 89 fa                    	movq	%r15, %rdx
14165c5a5: e8 d6 02 e1 ff              	callq	0x14146c880 <.text+0x146b880>
14165c5aa: e9 29 02 00 00              	jmp	0x14165c7d8 <.text+0x165b7d8>
14165c5af: 49 8b 03                    	movq	(%r11), %rax
14165c5b2: 41 b4 01                    	movb	$0x1, %r12b
14165c5b5: 0f 57 ff                    	xorps	%xmm7, %xmm7
14165c5b8: 80 78 0c 01                 	cmpb	$0x1, 0xc(%rax)
14165c5bc: 0f 84 16 02 00 00           	je	0x14165c7d8 <.text+0x165b7d8>
14165c5c2: 48 89 7c 24 28              	movq	%rdi, 0x28(%rsp)
14165c5c7: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14165c5cb: 0f 11 84 24 14 01 00 00     	movups	%xmm0, 0x114(%rsp)
14165c5d3: c7 84 24 10 01 00 00 00 00 00 00    	movl	$0x0, 0x110(%rsp)
14165c5de: 48 8d 94 24 10 01 00 00     	leaq	0x110(%rsp), %rdx
14165c5e6: 48 89 f1                    	movq	%rsi, %rcx
14165c5e9: e8 82 63 47 ff              	callq	0x140ad2970 <.text+0xad1970>
14165c5ee: 84 c0                       	testb	%al, %al
14165c5f0: 74 10                       	je	0x14165c602 <.text+0x165b602>
14165c5f2: 48 8d 7d 10                 	leaq	0x10(%rbp), %rdi
14165c5f6: 41 80 7d 2c 02              	cmpb	$0x2, 0x2c(%r13)
14165c5fb: 75 62                       	jne	0x14165c65f <.text+0x165b65f>
14165c5fd: 45 31 f6                    	xorl	%r14d, %r14d
14165c600: eb 67                       	jmp	0x14165c669 <.text+0x165b669>
14165c602: 49 89 f6                    	movq	%rsi, %r14
14165c605: 48 8b 84 24 d8 00 00 00     	movq	0xd8(%rsp), %rax
14165c60d: 8b 8c 24 a0 00 00 00        	movl	0xa0(%rsp), %ecx
14165c614: 89 08                       	movl	%ecx, (%rax)
14165c616: 4c 8b bc 24 e8 00 00 00     	movq	0xe8(%rsp), %r15
14165c61e: 49 8b 77 30                 	movq	0x30(%r15), %rsi
14165c622: 49 8b 7f 40                 	movq	0x40(%r15), %rdi
14165c626: 49 3b 77 20                 	cmpq	0x20(%r15), %rsi
14165c62a: 75 10                       	jne	0x14165c63c <.text+0x165b63c>
14165c62c: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
14165c630: 48 8d 15 11 db 5d 01        	leaq	0x15ddb11(%rip), %rdx   # 0x142c3a148
14165c637: e8 34 62 f0 fe              	callq	0x140562870 <.text+0x561870>
14165c63c: 49 8b 47 28                 	movq	0x28(%r15), %rax
14165c640: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
14165c644: 48 ff c6                    	incq	%rsi
14165c647: 49 89 77 30                 	movq	%rsi, 0x30(%r15)
14165c64b: 49 ff 47 40                 	incq	0x40(%r15)
14165c64f: 45 31 e4                    	xorl	%r12d, %r12d
14165c652: 48 8b 7c 24 28              	movq	0x28(%rsp), %rdi
14165c657: 4c 89 f6                    	movq	%r14, %rsi
14165c65a: e9 79 01 00 00              	jmp	0x14165c7d8 <.text+0x165b7d8>
14165c65f: 49 8b 55 20                 	movq	0x20(%r13), %rdx
14165c663: 41 be 01 00 00 00           	movl	$0x1, %r14d
14165c669: 48 89 74 24 48              	movq	%rsi, 0x48(%rsp)
14165c66e: f3 41 0f 10 75 40           	movss	0x40(%r13), %xmm6
14165c674: f3 45 0f 10 45 3c           	movss	0x3c(%r13), %xmm8
14165c67a: 0f 10 07                    	movups	(%rdi), %xmm0
14165c67d: 0f 29 84 24 10 01 00 00     	movaps	%xmm0, 0x110(%rsp)
14165c685: 48 8b 84 24 e0 00 00 00     	movq	0xe0(%rsp), %rax
14165c68d: 8b 8c 24 a4 00 00 00        	movl	0xa4(%rsp), %ecx
14165c694: 89 08                       	movl	%ecx, (%rax)
14165c696: 48 8b b4 24 f0 00 00 00     	movq	0xf0(%rsp), %rsi
14165c69e: 4c 8b 6e 30                 	movq	0x30(%rsi), %r13
14165c6a2: 4c 8b 7e 40                 	movq	0x40(%rsi), %r15
14165c6a6: 4c 3b 6e 20                 	cmpq	0x20(%rsi), %r13
14165c6aa: 75 20                       	jne	0x14165c6cc <.text+0x165b6cc>
14165c6ac: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
14165c6b0: 48 89 94 24 80 00 00 00     	movq	%rdx, 0x80(%rsp)
14165c6b8: 48 8d 15 89 da 5d 01        	leaq	0x15dda89(%rip), %rdx   # 0x142c3a148
14165c6bf: e8 2c 63 f0 fe              	callq	0x1405629f0 <.text+0x5619f0>
14165c6c4: 48 8b 94 24 80 00 00 00     	movq	0x80(%rsp), %rdx
14165c6cc: 48 8b 46 28                 	movq	0x28(%rsi), %rax
14165c6d0: 4a 8d 0c 6d 00 00 00 00     	leaq	(,%r13,2), %rcx
14165c6d8: 4c 01 e9                    	addq	%r13, %rcx
14165c6db: 48 c1 e1 04                 	shlq	$0x4, %rcx
14165c6df: 4c 89 34 08                 	movq	%r14, (%rax,%rcx)
14165c6e3: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
14165c6e8: 0f 28 84 24 10 01 00 00     	movaps	0x110(%rsp), %xmm0
14165c6f0: 0f 11 44 08 10              	movups	%xmm0, 0x10(%rax,%rcx)
14165c6f5: f3 0f 11 74 08 20           	movss	%xmm6, 0x20(%rax,%rcx)
14165c6fb: f3 44 0f 11 44 08 24        	movss	%xmm8, 0x24(%rax,%rcx)
14165c702: 4c 89 7c 08 28              	movq	%r15, 0x28(%rax,%rcx)
14165c707: 49 ff c5                    	incq	%r13
14165c70a: 4c 89 6e 30                 	movq	%r13, 0x30(%rsi)
14165c70e: 48 ff 46 40                 	incq	0x40(%rsi)
14165c712: 4c 8b ac 24 88 00 00 00     	movq	0x88(%rsp), %r13
14165c71a: 41 8b 45 38                 	movl	0x38(%r13), %eax
14165c71e: 89 44 24 3f                 	movl	%eax, 0x3f(%rsp)
14165c722: 49 8b 45 30                 	movq	0x30(%r13), %rax
14165c726: 48 89 44 24 37              	movq	%rax, 0x37(%rsp)
14165c72b: 48 8d 8c 24 14 01 00 00     	leaq	0x114(%rsp), %rcx
14165c733: 0f 10 07                    	movups	(%rdi), %xmm0
14165c736: 0f 11 84 24 14 01 00 00     	movups	%xmm0, 0x114(%rsp)
14165c73e: c7 84 24 10 01 00 00 00 00 00 00    	movl	$0x0, 0x110(%rsp)
14165c749: e8 32 ae 63 ff              	callq	0x140c97580 <.text+0xc96580>
14165c74e: 0f 28 d0                    	movaps	%xmm0, %xmm2
14165c751: f3 0f 10 05 97 52 62 01     	movss	0x1625297(%rip), %xmm0  # 0x142c819f0
14165c759: f3 0f 10 0d 93 52 62 01     	movss	0x1625293(%rip), %xmm1  # 0x142c819f4
14165c761: e8 3a 3d 63 ff              	callq	0x140c904a0 <.text+0xc8f4a0>
14165c766: 0f 28 f0                    	movaps	%xmm0, %xmm6
14165c769: 48 8b 44 24 70              	movq	0x70(%rsp), %rax
14165c76e: 8b 4c 24 24                 	movl	0x24(%rsp), %ecx
14165c772: 89 08                       	movl	%ecx, (%rax)
14165c774: 48 8b 74 24 78              	movq	0x78(%rsp), %rsi
14165c779: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
14165c77d: 4c 8b 76 40                 	movq	0x40(%rsi), %r14
14165c781: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
14165c785: 75 10                       	jne	0x14165c797 <.text+0x165b797>
14165c787: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
14165c78b: 48 8d 15 b6 d9 5d 01        	leaq	0x15dd9b6(%rip), %rdx   # 0x142c3a148
14165c792: e8 99 64 f0 fe              	callq	0x140562c30 <.text+0x561c30>
14165c797: 48 8b 46 28                 	movq	0x28(%rsi), %rax
14165c79b: 48 89 f9                    	movq	%rdi, %rcx
14165c79e: 48 c1 e1 05                 	shlq	$0x5, %rcx
14165c7a2: c6 04 08 07                 	movb	$0x7, (%rax,%rcx)
14165c7a6: 0f 10 44 24 30              	movups	0x30(%rsp), %xmm0
14165c7ab: 0f 11 44 08 01              	movups	%xmm0, 0x1(%rax,%rcx)
14165c7b0: 8b 54 24 3f                 	movl	0x3f(%rsp), %edx
14165c7b4: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
14165c7b8: f3 0f 11 74 08 14           	movss	%xmm6, 0x14(%rax,%rcx)
14165c7be: 4c 89 74 08 18              	movq	%r14, 0x18(%rax,%rcx)
14165c7c3: 48 ff c7                    	incq	%rdi
14165c7c6: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
14165c7ca: 48 ff 46 40                 	incq	0x40(%rsi)
14165c7ce: 48 8b 7c 24 28              	movq	0x28(%rsp), %rdi
14165c7d3: 48 8b 74 24 48              	movq	0x48(%rsp), %rsi
14165c7d8: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14165c7dc: 0f 11 84 24 14 01 00 00     	movups	%xmm0, 0x114(%rsp)
14165c7e4: c7 84 24 10 01 00 00 00 00 00 00    	movl	$0x0, 0x110(%rsp)
14165c7ef: 48 8d 94 24 10 01 00 00     	leaq	0x110(%rsp), %rdx
14165c7f7: 48 89 f1                    	movq	%rsi, %rcx
14165c7fa: e8 71 61 47 ff              	callq	0x140ad2970 <.text+0xad1970>
14165c7ff: 84 c0                       	testb	%al, %al
14165c801: 75 52                       	jne	0x14165c855 <.text+0x165b855>
14165c803: 8b 84 24 ac 00 00 00        	movl	0xac(%rsp), %eax
14165c80a: 89 07                       	movl	%eax, (%rdi)
14165c80c: 48 8b 8c 24 98 00 00 00     	movq	0x98(%rsp), %rcx
14165c814: 48 8b 71 30                 	movq	0x30(%rcx), %rsi
14165c818: 48 8b 79 40                 	movq	0x40(%rcx), %rdi
14165c81c: 48 3b 71 20                 	cmpq	0x20(%rcx), %rsi
14165c820: 75 20                       	jne	0x14165c842 <.text+0x165b842>
14165c822: 48 8b 84 24 98 00 00 00     	movq	0x98(%rsp), %rax
14165c82a: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14165c82e: 48 8d 15 13 d9 5d 01        	leaq	0x15dd913(%rip), %rdx   # 0x142c3a148
14165c835: e8 36 60 f0 fe              	callq	0x140562870 <.text+0x561870>
14165c83a: 48 8b 8c 24 98 00 00 00     	movq	0x98(%rsp), %rcx
14165c842: 48 8b 41 28                 	movq	0x28(%rcx), %rax
14165c846: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
14165c84a: 48 ff c6                    	incq	%rsi
14165c84d: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
14165c851: 48 ff 41 40                 	incq	0x40(%rcx)
14165c855: 48 8b bc 24 80 02 00 00     	movq	0x280(%rsp), %rdi
14165c85d: 45 84 e4                    	testb	%r12b, %r12b
14165c860: 0f 84 1e 01 00 00           	je	0x14165c984 <.text+0x165b984>
14165c866: 4c 8b 75 10                 	movq	0x10(%rbp), %r14
14165c86a: f3 0f 10 75 18              	movss	0x18(%rbp), %xmm6
14165c86f: 44 8b 7d 1c                 	movl	0x1c(%rbp), %r15d
14165c873: f3 0f 58 35 d1 41 42 01     	addss	0x14241d1(%rip), %xmm6  # 0x142a80a4c
14165c87b: 41 8b 45 2c                 	movl	0x2c(%r13), %eax
14165c87f: 45 31 e4                    	xorl	%r12d, %r12d
14165c882: 3c 02                       	cmpb	$0x2, %al
14165c884: 44 0f 45 e0                 	cmovnel	%eax, %r12d
14165c888: 8b 43 1c                    	movl	0x1c(%rbx), %eax
14165c88b: 48 8b 4b 10                 	movq	0x10(%rbx), %rcx
14165c88f: 89 01                       	movl	%eax, (%rcx)
14165c891: 48 8b 1b                    	movq	(%rbx), %rbx
14165c894: 4c 8b 6b 30                 	movq	0x30(%rbx), %r13
14165c898: 48 8b 73 40                 	movq	0x40(%rbx), %rsi
14165c89c: 4c 3b 6b 20                 	cmpq	0x20(%rbx), %r13
14165c8a0: 75 10                       	jne	0x14165c8b2 <.text+0x165b8b2>
14165c8a2: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
14165c8a6: 48 8d 15 9b d8 5d 01        	leaq	0x15dd89b(%rip), %rdx   # 0x142c3a148
14165c8ad: e8 fe 61 f0 fe              	callq	0x140562ab0 <.text+0x561ab0>
14165c8b2: 48 8b 43 28                 	movq	0x28(%rbx), %rax
14165c8b6: 4a 8d 0c ad 00 00 00 00     	leaq	(,%r13,4), %rcx
14165c8be: 4c 01 e9                    	addq	%r13, %rcx
14165c8c1: c7 04 c8 00 00 00 00        	movl	$0x0, (%rax,%rcx,8)
14165c8c8: 4c 89 74 c8 04              	movq	%r14, 0x4(%rax,%rcx,8)
14165c8cd: f3 0f 11 74 c8 0c           	movss	%xmm6, 0xc(%rax,%rcx,8)
14165c8d3: 44 89 7c c8 10              	movl	%r15d, 0x10(%rax,%rcx,8)
14165c8d8: 44 88 64 c8 1c              	movb	%r12b, 0x1c(%rax,%rcx,8)
14165c8dd: 48 89 74 c8 20              	movq	%rsi, 0x20(%rax,%rcx,8)
14165c8e2: 49 ff c5                    	incq	%r13
14165c8e5: 4c 89 6b 30                 	movq	%r13, 0x30(%rbx)
14165c8e9: 48 ff 43 40                 	incq	0x40(%rbx)
14165c8ed: 4c 8b ac 24 88 00 00 00     	movq	0x88(%rsp), %r13
14165c8f5: f3 45 0f 10 45 3c           	movss	0x3c(%r13), %xmm8
14165c8fb: 48 8b 84 24 00 01 00 00     	movq	0x100(%rsp), %rax
14165c903: 8b 8c 24 b0 00 00 00        	movl	0xb0(%rsp), %ecx
14165c90a: 89 08                       	movl	%ecx, (%rax)
14165c90c: 48 8b 84 24 d0 00 00 00     	movq	0xd0(%rsp), %rax
14165c914: 48 8b 58 30                 	movq	0x30(%rax), %rbx
14165c918: 4c 8b 60 40                 	movq	0x40(%rax), %r12
14165c91c: 48 3b 58 20                 	cmpq	0x20(%rax), %rbx
14165c920: 75 18                       	jne	0x14165c93a <.text+0x165b93a>
14165c922: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14165c926: 48 8d 15 1b d8 5d 01        	leaq	0x15dd81b(%rip), %rdx   # 0x142c3a148
14165c92d: e8 be 60 f0 fe              	callq	0x1405629f0 <.text+0x5619f0>
14165c932: 48 8b 84 24 d0 00 00 00     	movq	0xd0(%rsp), %rax
14165c93a: 48 89 c2                    	movq	%rax, %rdx
14165c93d: 48 8b 40 28                 	movq	0x28(%rax), %rax
14165c941: 48 8d 0c 5b                 	leaq	(%rbx,%rbx,2), %rcx
14165c945: 48 c1 e1 04                 	shlq	$0x4, %rcx
14165c949: c7 04 08 00 00 00 00        	movl	$0x0, (%rax,%rcx)
14165c950: 4c 89 74 08 04              	movq	%r14, 0x4(%rax,%rcx)
14165c955: f3 0f 11 74 08 0c           	movss	%xmm6, 0xc(%rax,%rcx)
14165c95b: 44 89 7c 08 10              	movl	%r15d, 0x10(%rax,%rcx)
14165c960: f3 44 0f 11 44 08 1c        	movss	%xmm8, 0x1c(%rax,%rcx)
14165c967: c7 44 08 20 00 00 80 40     	movl	$0x40800000, 0x20(%rax,%rcx) # imm = 0x40800000
14165c96f: c6 44 08 24 00              	movb	$0x0, 0x24(%rax,%rcx)
14165c974: 4c 89 64 08 28              	movq	%r12, 0x28(%rax,%rcx)
14165c979: 48 ff c3                    	incq	%rbx
14165c97c: 48 89 5a 30                 	movq	%rbx, 0x30(%rdx)
14165c980: 48 ff 42 40                 	incq	0x40(%rdx)
14165c984: 48 8b 17                    	movq	(%rdi), %rdx
14165c987: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
14165c98b: 4c 8b 02                    	movq	(%rdx), %r8
14165c98e: 31 c0                       	xorl	%eax, %eax
14165c990: 4d 89 c1                    	movq	%r8, %r9
14165c993: 4c 2b 49 18                 	subq	0x18(%rcx), %r9
14165c997: 4c 0f 42 c8                 	cmovbq	%rax, %r9
14165c99b: 4c 2b 41 38                 	subq	0x38(%rcx), %r8
14165c99f: 4c 0f 42 c0                 	cmovbq	%rax, %r8
14165c9a3: 4c 8b 51 30                 	movq	0x30(%rcx), %r10
14165c9a7: 4d 29 c2                    	subq	%r8, %r10
14165c9aa: 4c 0f 42 d0                 	cmovbq	%rax, %r10
14165c9ae: 41 0f 96 c0                 	setbe	%r8b
14165c9b2: 4c 8b 59 10                 	movq	0x10(%rcx), %r11
14165c9b6: 4d 29 cb                    	subq	%r9, %r11
14165c9b9: 4c 0f 42 d8                 	cmovbq	%rax, %r11
14165c9bd: 4c 8b 49 40                 	movq	0x40(%rcx), %r9
14165c9c1: 0f 96 c1                    	setbe	%cl
14165c9c4: 4d 01 d3                    	addq	%r10, %r11
14165c9c7: 44 20 c1                    	andb	%r8b, %cl
14165c9ca: 4c 0f 44 d8                 	cmoveq	%rax, %r11
14165c9ce: 4d 29 d9                    	subq	%r11, %r9
14165c9d1: 4c 89 0a                    	movq	%r9, (%rdx)
14165c9d4: 48 8b 54 24 60              	movq	0x60(%rsp), %rdx
14165c9d9: 80 7a 18 02                 	cmpb	$0x2, 0x18(%rdx)
14165c9dd: 72 18                       	jb	0x14165c9f7 <.text+0x165b9f7>
14165c9df: 48 8b 84 24 08 01 00 00     	movq	0x108(%rsp), %rax
14165c9e7: 48 39 42 38                 	cmpq	%rax, 0x38(%rdx)
14165c9eb: 75 08                       	jne	0x14165c9f5 <.text+0x165b9f5>
14165c9ed: 0f b6 42 19                 	movzbl	0x19(%rdx), %eax
14165c9f1: 04 fe                       	addb	$-0x2, %al
14165c9f3: eb 02                       	jmp	0x14165c9f7 <.text+0x165b9f7>
14165c9f5: 31 c0                       	xorl	%eax, %eax
14165c9f7: 48 8d 75 10                 	leaq	0x10(%rbp), %rsi
14165c9fb: 0f b6 c0                    	movzbl	%al, %eax
14165c9fe: 3c 03                       	cmpb	$0x3, %al
14165ca00: ba 03 00 00 00              	movl	$0x3, %edx
14165ca05: 0f 42 d0                    	cmovbl	%eax, %edx
14165ca08: 0f b6 c2                    	movzbl	%dl, %eax
14165ca0b: 48 8d 15 aa 5c 62 01        	leaq	0x1625caa(%rip), %rdx   # 0x142c826bc
14165ca12: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14165ca16: 48 01 d0                    	addq	%rdx, %rax
14165ca19: ff e0                       	jmpq	*%rax
14165ca1b: 84 c9                       	testb	%cl, %cl
14165ca1d: 0f 85 ef 02 00 00           	jne	0x14165cd12 <.text+0x165bd12>
14165ca23: 48 8b 9c 24 60 02 00 00     	movq	0x260(%rsp), %rbx
14165ca2b: f3 41 0f 10 55 3c           	movss	0x3c(%r13), %xmm2
14165ca31: 48 8d 8c 24 b4 00 00 00     	leaq	0xb4(%rsp), %rcx
14165ca39: 48 89 f2                    	movq	%rsi, %rdx
14165ca3c: e8 9f 03 64 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
14165ca41: f3 0f 10 94 24 b4 00 00 00  	movss	0xb4(%rsp), %xmm2
14165ca4a: f3 0f 10 84 24 b8 00 00 00  	movss	0xb8(%rsp), %xmm0
14165ca53: f3 0f 10 8c 24 bc 00 00 00  	movss	0xbc(%rsp), %xmm1
14165ca5c: f6 45 00 01                 	testb	$0x1, (%rbp)
14165ca60: 0f 85 74 01 00 00           	jne	0x14165cbda <.text+0x165bbda>
14165ca66: 0f 28 da                    	movaps	%xmm2, %xmm3
14165ca69: 0f 28 e0                    	movaps	%xmm0, %xmm4
14165ca6c: 0f 28 e9                    	movaps	%xmm1, %xmm5
14165ca6f: e9 75 01 00 00              	jmp	0x14165cbe9 <.text+0x165bbe9>
14165ca74: 41 8b 45 38                 	movl	0x38(%r13), %eax
14165ca78: 89 44 24 3f                 	movl	%eax, 0x3f(%rsp)
14165ca7c: 49 8b 45 30                 	movq	0x30(%r13), %rax
14165ca80: 48 89 44 24 37              	movq	%rax, 0x37(%rsp)
14165ca85: 48 8d 8c 24 14 01 00 00     	leaq	0x114(%rsp), %rcx
14165ca8d: 0f 10 06                    	movups	(%rsi), %xmm0
14165ca90: 0f 11 84 24 14 01 00 00     	movups	%xmm0, 0x114(%rsp)
14165ca98: c7 84 24 10 01 00 00 00 00 00 00    	movl	$0x0, 0x110(%rsp)
14165caa3: e8 d8 aa 63 ff              	callq	0x140c97580 <.text+0xc96580>
14165caa8: 0f 28 d0                    	movaps	%xmm0, %xmm2
14165caab: f3 0f 10 05 3d 4f 62 01     	movss	0x1624f3d(%rip), %xmm0  # 0x142c819f0
14165cab3: f3 0f 10 0d 39 4f 62 01     	movss	0x1624f39(%rip), %xmm1  # 0x142c819f4
14165cabb: e8 e0 39 63 ff              	callq	0x140c904a0 <.text+0xc8f4a0>
14165cac0: 0f 28 f0                    	movaps	%xmm0, %xmm6
14165cac3: 48 8b 44 24 70              	movq	0x70(%rsp), %rax
14165cac8: 8b 4c 24 24                 	movl	0x24(%rsp), %ecx
14165cacc: 89 08                       	movl	%ecx, (%rax)
14165cace: 48 8b 5c 24 78              	movq	0x78(%rsp), %rbx
14165cad3: 48 8b 73 30                 	movq	0x30(%rbx), %rsi
14165cad7: 48 8b 7b 40                 	movq	0x40(%rbx), %rdi
14165cadb: 48 3b 73 20                 	cmpq	0x20(%rbx), %rsi
14165cadf: 75 10                       	jne	0x14165caf1 <.text+0x165baf1>
14165cae1: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
14165cae5: 48 8d 15 5c d6 5d 01        	leaq	0x15dd65c(%rip), %rdx   # 0x142c3a148
14165caec: e8 3f 61 f0 fe              	callq	0x140562c30 <.text+0x561c30>
14165caf1: 48 8b 43 28                 	movq	0x28(%rbx), %rax
14165caf5: 48 89 f1                    	movq	%rsi, %rcx
14165caf8: 48 c1 e1 05                 	shlq	$0x5, %rcx
14165cafc: c6 04 08 02                 	movb	$0x2, (%rax,%rcx)
14165cb00: 0f 10 44 24 30              	movups	0x30(%rsp), %xmm0
14165cb05: 0f 11 44 08 01              	movups	%xmm0, 0x1(%rax,%rcx)
14165cb0a: 8b 54 24 3f                 	movl	0x3f(%rsp), %edx
14165cb0e: e9 a8 00 00 00              	jmp	0x14165cbbb <.text+0x165bbbb>
14165cb13: 84 c9                       	testb	%cl, %cl
14165cb15: 0f 85 f7 01 00 00           	jne	0x14165cd12 <.text+0x165bd12>
14165cb1b: 41 8b 45 38                 	movl	0x38(%r13), %eax
14165cb1f: 89 44 24 38                 	movl	%eax, 0x38(%rsp)
14165cb23: 49 8b 45 30                 	movq	0x30(%r13), %rax
14165cb27: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14165cb2c: 48 8d 8c 24 14 01 00 00     	leaq	0x114(%rsp), %rcx
14165cb34: 0f 10 06                    	movups	(%rsi), %xmm0
14165cb37: 0f 11 84 24 14 01 00 00     	movups	%xmm0, 0x114(%rsp)
14165cb3f: c7 84 24 10 01 00 00 00 00 00 00    	movl	$0x0, 0x110(%rsp)
14165cb4a: e8 31 aa 63 ff              	callq	0x140c97580 <.text+0xc96580>
14165cb4f: 0f 28 d0                    	movaps	%xmm0, %xmm2
14165cb52: f3 0f 10 05 96 4e 62 01     	movss	0x1624e96(%rip), %xmm0  # 0x142c819f0
14165cb5a: f3 0f 10 0d 92 4e 62 01     	movss	0x1624e92(%rip), %xmm1  # 0x142c819f4
14165cb62: e8 39 39 63 ff              	callq	0x140c904a0 <.text+0xc8f4a0>
14165cb67: 0f 28 f0                    	movaps	%xmm0, %xmm6
14165cb6a: 48 8b 44 24 70              	movq	0x70(%rsp), %rax
14165cb6f: 8b 4c 24 24                 	movl	0x24(%rsp), %ecx
14165cb73: 89 08                       	movl	%ecx, (%rax)
14165cb75: 48 8b 5c 24 78              	movq	0x78(%rsp), %rbx
14165cb7a: 48 8b 73 30                 	movq	0x30(%rbx), %rsi
14165cb7e: 48 8b 7b 40                 	movq	0x40(%rbx), %rdi
14165cb82: 48 3b 73 20                 	cmpq	0x20(%rbx), %rsi
14165cb86: 75 10                       	jne	0x14165cb98 <.text+0x165bb98>
14165cb88: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
14165cb8c: 48 8d 15 b5 d5 5d 01        	leaq	0x15dd5b5(%rip), %rdx   # 0x142c3a148
14165cb93: e8 98 60 f0 fe              	callq	0x140562c30 <.text+0x561c30>
14165cb98: 48 8b 43 28                 	movq	0x28(%rbx), %rax
14165cb9c: 48 89 f1                    	movq	%rsi, %rcx
14165cb9f: 48 c1 e1 05                 	shlq	$0x5, %rcx
14165cba3: c6 04 08 00                 	movb	$0x0, (%rax,%rcx)
14165cba7: f3 0f 11 7c 08 04           	movss	%xmm7, 0x4(%rax,%rcx)
14165cbad: 48 8b 54 24 30              	movq	0x30(%rsp), %rdx
14165cbb2: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
14165cbb7: 8b 54 24 38                 	movl	0x38(%rsp), %edx
14165cbbb: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
14165cbbf: f3 0f 11 74 08 14           	movss	%xmm6, 0x14(%rax,%rcx)
14165cbc5: 48 89 7c 08 18              	movq	%rdi, 0x18(%rax,%rcx)
14165cbca: 48 ff c6                    	incq	%rsi
14165cbcd: 48 89 73 30                 	movq	%rsi, 0x30(%rbx)
14165cbd1: 48 ff 43 40                 	incq	0x40(%rbx)
14165cbd5: e9 38 01 00 00              	jmp	0x14165cd12 <.text+0x165bd12>
14165cbda: f3 0f 10 5d 04              	movss	0x4(%rbp), %xmm3
14165cbdf: f3 0f 10 65 08              	movss	0x8(%rbp), %xmm4
14165cbe4: f3 0f 10 6d 0c              	movss	0xc(%rbp), %xmm5
14165cbe9: 48 8d 7d 04                 	leaq	0x4(%rbp), %rdi
14165cbed: f3 0f 5c d3                 	subss	%xmm3, %xmm2
14165cbf1: f3 0f 5c c4                 	subss	%xmm4, %xmm0
14165cbf5: f3 0f 5c cd                 	subss	%xmm5, %xmm1
14165cbf9: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
14165cbfd: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
14165cc01: f3 0f 58 c2                 	addss	%xmm2, %xmm0
14165cc05: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
14165cc09: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14165cc0d: f3 0f 51 f9                 	sqrtss	%xmm1, %xmm7
14165cc11: 48 8b 03                    	movq	(%rbx), %rax
14165cc14: f3 0f 5e 78 28              	divss	0x28(%rax), %xmm7
14165cc19: 0f 57 c0                    	xorps	%xmm0, %xmm0
14165cc1c: f3 41 0f c2 45 3c 00        	cmpeqss	0x3c(%r13), %xmm0
14165cc23: 66 0f 7e c3                 	movd	%xmm0, %ebx
14165cc27: 83 e3 01                    	andl	$0x1, %ebx
14165cc2a: 49 8b 45 30                 	movq	0x30(%r13), %rax
14165cc2e: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14165cc33: 41 8b 45 38                 	movl	0x38(%r13), %eax
14165cc37: 89 44 24 38                 	movl	%eax, 0x38(%rsp)
14165cc3b: 48 8d 8c 24 14 01 00 00     	leaq	0x114(%rsp), %rcx
14165cc43: 0f 10 06                    	movups	(%rsi), %xmm0
14165cc46: 0f 11 84 24 14 01 00 00     	movups	%xmm0, 0x114(%rsp)
14165cc4e: c7 84 24 10 01 00 00 00 00 00 00    	movl	$0x0, 0x110(%rsp)
14165cc59: e8 22 a9 63 ff              	callq	0x140c97580 <.text+0xc96580>
14165cc5e: 0f 28 d0                    	movaps	%xmm0, %xmm2
14165cc61: f3 0f 10 05 87 4d 62 01     	movss	0x1624d87(%rip), %xmm0  # 0x142c819f0
14165cc69: f3 0f 10 0d 83 4d 62 01     	movss	0x1624d83(%rip), %xmm1  # 0x142c819f4
14165cc71: e8 2a 38 63 ff              	callq	0x140c904a0 <.text+0xc8f4a0>
14165cc76: 0f 28 f0                    	movaps	%xmm0, %xmm6
14165cc79: 48 8b 44 24 70              	movq	0x70(%rsp), %rax
14165cc7e: 8b 4c 24 24                 	movl	0x24(%rsp), %ecx
14165cc82: 89 08                       	movl	%ecx, (%rax)
14165cc84: 4c 8b 7c 24 78              	movq	0x78(%rsp), %r15
14165cc89: 49 8b 77 30                 	movq	0x30(%r15), %rsi
14165cc8d: 4d 8b 77 40                 	movq	0x40(%r15), %r14
14165cc91: 49 3b 77 20                 	cmpq	0x20(%r15), %rsi
14165cc95: 75 10                       	jne	0x14165cca7 <.text+0x165bca7>
14165cc97: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
14165cc9b: 48 8d 15 a6 d4 5d 01        	leaq	0x15dd4a6(%rip), %rdx   # 0x142c3a148
14165cca2: e8 89 5f f0 fe              	callq	0x140562c30 <.text+0x561c30>
14165cca7: 49 8b 47 28                 	movq	0x28(%r15), %rax
14165ccab: 48 89 f1                    	movq	%rsi, %rcx
14165ccae: 48 c1 e1 05                 	shlq	$0x5, %rcx
14165ccb2: c6 04 08 03                 	movb	$0x3, (%rax,%rcx)
14165ccb6: 88 5c 08 01                 	movb	%bl, 0x1(%rax,%rcx)
14165ccba: f3 0f 11 7c 08 04           	movss	%xmm7, 0x4(%rax,%rcx)
14165ccc0: 48 8b 54 24 30              	movq	0x30(%rsp), %rdx
14165ccc5: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
14165ccca: 8b 54 24 38                 	movl	0x38(%rsp), %edx
14165ccce: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
14165ccd2: f3 0f 11 74 08 14           	movss	%xmm6, 0x14(%rax,%rcx)
14165ccd8: 4c 89 74 08 18              	movq	%r14, 0x18(%rax,%rcx)
14165ccdd: 48 ff c6                    	incq	%rsi
14165cce0: 49 89 77 30                 	movq	%rsi, 0x30(%r15)
14165cce4: 49 ff 47 40                 	incq	0x40(%r15)
14165cce8: 48 8b 84 24 90 00 00 00     	movq	0x90(%rsp), %rax
14165ccf0: 8b 4c 24 54                 	movl	0x54(%rsp), %ecx
14165ccf4: 89 08                       	movl	%ecx, (%rax)
14165ccf6: c7 45 00 01 00 00 00        	movl	$0x1, (%rbp)
14165ccfd: 8b 84 24 bc 00 00 00        	movl	0xbc(%rsp), %eax
14165cd04: 89 47 08                    	movl	%eax, 0x8(%rdi)
14165cd07: 48 8b 84 24 b4 00 00 00     	movq	0xb4(%rsp), %rax
14165cd0f: 48 89 07                    	movq	%rax, (%rdi)
14165cd12: 0f 28 b4 24 c0 01 00 00     	movaps	0x1c0(%rsp), %xmm6
14165cd1a: 0f 28 bc 24 d0 01 00 00     	movaps	0x1d0(%rsp), %xmm7
14165cd22: 44 0f 28 84 24 e0 01 00 00  	movaps	0x1e0(%rsp), %xmm8
14165cd2b: 48 81 c4 f8 01 00 00        	addq	$0x1f8, %rsp            # imm = 0x1F8
14165cd32: 5b                          	popq	%rbx
14165cd33: 5d                          	popq	%rbp
14165cd34: 5f                          	popq	%rdi
14165cd35: 5e                          	popq	%rsi
14165cd36: 41 5c                       	popq	%r12
14165cd38: 41 5d                       	popq	%r13
14165cd3a: 41 5e                       	popq	%r14
14165cd3c: 41 5f                       	popq	%r15
14165cd3e: c3                          	retq

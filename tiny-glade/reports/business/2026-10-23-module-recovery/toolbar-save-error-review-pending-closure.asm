
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14130f900: 55                          	pushq	%rbp
14130f901: 41 56                       	pushq	%r14
14130f903: 56                          	pushq	%rsi
14130f904: 57                          	pushq	%rdi
14130f905: 53                          	pushq	%rbx
14130f906: 48 81 ec f0 00 00 00        	subq	$0xf0, %rsp
14130f90d: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14130f915: 48 c7 45 68 fe ff ff ff     	movq	$-0x2, 0x68(%rbp)
14130f91d: 48 89 d7                    	movq	%rdx, %rdi
14130f920: 48 89 ce                    	movq	%rcx, %rsi
14130f923: 48 89 d1                    	movq	%rdx, %rcx
14130f926: e8 05 a9 60 ff              	callq	0x14091a230 <.text+0x919230>
14130f92b: 89 c3                       	movl	%eax, %ebx
14130f92d: 84 c0                       	testb	%al, %al
14130f92f: 0f 84 9e 00 00 00           	je	0x14130f9d3 <.text+0x130e9d3>
14130f935: 48 8b 05 6c ca 31 02        	movq	0x231ca6c(%rip), %rax   # 0x14362c3a8
14130f93c: 48 8b 00                    	movq	(%rax), %rax
14130f93f: 48 85 c0                    	testq	%rax, %rax
14130f942: 0f 84 2a 01 00 00           	je	0x14130fa72 <.text+0x130ea72>
14130f948: 48 8d 0d a1 95 92 01        	leaq	0x19295a1(%rip), %rcx   # 0x142c38ef0
14130f94f: e8 2c e4 36 ff              	callq	0x14067dd80 <.text+0x67cd80>
14130f954: 0f 10 00                    	movups	(%rax), %xmm0
14130f957: 8b 40 10                    	movl	0x10(%rax), %eax
14130f95a: 48 c7 45 10 01 00 00 00     	movq	$0x1, 0x10(%rbp)
14130f962: 48 8d 0d 9f 95 92 01        	leaq	0x192959f(%rip), %rcx   # 0x142c38f08
14130f969: 48 89 4d 18                 	movq	%rcx, 0x18(%rbp)
14130f96d: 48 c7 45 20 28 00 00 00     	movq	$0x28, 0x20(%rbp)
14130f975: 48 8d 15 64 95 92 01        	leaq	0x1929564(%rip), %rdx   # 0x142c38ee0
14130f97c: 48 89 55 30                 	movq	%rdx, 0x30(%rbp)
14130f980: 48 c7 45 38 01 00 00 00     	movq	$0x1, 0x38(%rbp)
14130f988: 48 c7 45 40 08 00 00 00     	movq	$0x8, 0x40(%rbp)
14130f990: 48 c7 45 e0 00 00 00 00     	movq	$0x0, -0x20(%rbp)
14130f998: 0f 57 c9                    	xorps	%xmm1, %xmm1
14130f99b: 0f 11 4d 48                 	movups	%xmm1, 0x48(%rbp)
14130f99f: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
14130f9a3: 48 c7 45 f0 28 00 00 00     	movq	$0x28, -0x10(%rbp)
14130f9ab: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
14130f9b3: 0f 11 45 00                 	movups	%xmm0, (%rbp)
14130f9b7: c7 45 28 01 00 00 00        	movl	$0x1, 0x28(%rbp)
14130f9be: 89 45 2c                    	movl	%eax, 0x2c(%rbp)
14130f9c1: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
14130f9c5: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
14130f9c9: e8 62 a9 a3 ff              	callq	0x140d4a330 <.text+0xd49330>
14130f9ce: e9 9f 00 00 00              	jmp	0x14130fa72 <.text+0x130ea72>
14130f9d3: 48 83 7f 78 00              	cmpq	$0x0, 0x78(%rdi)
14130f9d8: 0f 84 94 00 00 00           	je	0x14130fa72 <.text+0x130ea72>
14130f9de: 4c 8d 77 78                 	leaq	0x78(%rdi), %r14
14130f9e2: 48 89 f9                    	movq	%rdi, %rcx
14130f9e5: e8 e6 74 a6 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
14130f9ea: 49 89 d0                    	movq	%rdx, %r8
14130f9ed: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
14130f9f1: 48 89 c2                    	movq	%rax, %rdx
14130f9f4: e8 07 21 a5 ff              	callq	0x140d61b00 <.text+0xd60b00>
14130f9f9: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
14130f9fd: 4c 89 f2                    	movq	%r14, %rdx
14130fa00: e8 8b 0a 41 00              	callq	0x141720490 <.text+0x171f490>
14130fa05: 0f 10 45 c0                 	movups	-0x40(%rbp), %xmm0
14130fa09: 0f 10 4d d0                 	movups	-0x30(%rbp), %xmm1
14130fa0d: 0f 29 4d f0                 	movaps	%xmm1, -0x10(%rbp)
14130fa11: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
14130fa15: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
14130fa19: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
14130fa1d: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
14130fa21: 0f 29 4d 10                 	movaps	%xmm1, 0x10(%rbp)
14130fa25: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
14130fa29: 48 3b 3e                    	cmpq	(%rsi), %rdi
14130fa2c: 75 0f                       	jne	0x14130fa3d <.text+0x130ea3d>
14130fa2e: 48 8d 15 5b 94 92 01        	leaq	0x192945b(%rip), %rdx   # 0x142c38e90
14130fa35: 48 89 f1                    	movq	%rsi, %rcx
14130fa38: e8 73 2d 25 ff              	callq	0x1405627b0 <.text+0x5617b0>
14130fa3d: 48 8b 46 08                 	movq	0x8(%rsi), %rax
14130fa41: 48 89 f9                    	movq	%rdi, %rcx
14130fa44: 48 c1 e1 06                 	shlq	$0x6, %rcx
14130fa48: 0f 28 45 e0                 	movaps	-0x20(%rbp), %xmm0
14130fa4c: 0f 28 4d f0                 	movaps	-0x10(%rbp), %xmm1
14130fa50: 0f 28 55 00                 	movaps	(%rbp), %xmm2
14130fa54: 0f 28 5d 10                 	movaps	0x10(%rbp), %xmm3
14130fa58: 0f 11 5c 08 30              	movups	%xmm3, 0x30(%rax,%rcx)
14130fa5d: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
14130fa62: 0f 11 4c 08 10              	movups	%xmm1, 0x10(%rax,%rcx)
14130fa67: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
14130fa6b: 48 ff c7                    	incq	%rdi
14130fa6e: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
14130fa72: 80 f3 01                    	xorb	$0x1, %bl
14130fa75: 89 d8                       	movl	%ebx, %eax
14130fa77: 48 81 c4 f0 00 00 00        	addq	$0xf0, %rsp
14130fa7e: 5b                          	popq	%rbx
14130fa7f: 5f                          	popq	%rdi
14130fa80: 5e                          	popq	%rsi
14130fa81: 41 5e                       	popq	%r14
14130fa83: 5d                          	popq	%rbp
14130fa84: c3                          	retq
14130fa85: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14130fa90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14130fa95: 55                          	pushq	%rbp
14130fa96: 41 56                       	pushq	%r14
14130fa98: 56                          	pushq	%rsi
14130fa99: 57                          	pushq	%rdi
14130fa9a: 53                          	pushq	%rbx
14130fa9b: 48 83 ec 20                 	subq	$0x20, %rsp
14130fa9f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14130faa6: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
14130faaa: 48 85 d2                    	testq	%rdx, %rdx
14130faad: 74 0f                       	je	0x14130fabe <.text+0x130eabe>
14130faaf: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
14130fab3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14130fab9: e8 62 41 30 ff              	callq	0x140613c20 <.text+0x612c20>
14130fabe: 90                          	nop
14130fabf: 48 83 c4 20                 	addq	$0x20, %rsp
14130fac3: 5b                          	popq	%rbx
14130fac4: 5f                          	popq	%rdi
14130fac5: 5e                          	popq	%rsi
14130fac6: 41 5e                       	popq	%r14
14130fac8: 5d                          	popq	%rbp
14130fac9: c3                          	retq
14130faca: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14130fad0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14130fad5: 55                          	pushq	%rbp
14130fad6: 41 56                       	pushq	%r14
14130fad8: 56                          	pushq	%rsi
14130fad9: 57                          	pushq	%rdi
14130fada: 53                          	pushq	%rbx
14130fadb: 48 83 ec 20                 	subq	$0x20, %rsp
14130fadf: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14130fae6: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
14130faea: e8 81 0f 86 ff              	callq	0x140b70a70 <.text+0xb6fa70>
14130faef: 90                          	nop
14130faf0: 48 83 c4 20                 	addq	$0x20, %rsp
14130faf4: 5b                          	popq	%rbx
14130faf5: 5f                          	popq	%rdi
14130faf6: 5e                          	popq	%rsi
14130faf7: 41 5e                       	popq	%r14
14130faf9: 5d                          	popq	%rbp
14130fafa: c3                          	retq

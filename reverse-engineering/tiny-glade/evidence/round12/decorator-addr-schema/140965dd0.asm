
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140965dd0: 55                          	pushq	%rbp
140965dd1: 41 57                       	pushq	%r15
140965dd3: 41 56                       	pushq	%r14
140965dd5: 41 55                       	pushq	%r13
140965dd7: 41 54                       	pushq	%r12
140965dd9: 56                          	pushq	%rsi
140965dda: 57                          	pushq	%rdi
140965ddb: 53                          	pushq	%rbx
140965ddc: 48 81 ec 68 01 00 00        	subq	$0x168, %rsp            # imm = 0x168
140965de3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140965deb: 0f 29 b5 d0 00 00 00        	movaps	%xmm6, 0xd0(%rbp)
140965df2: 48 c7 85 c8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xc8(%rbp)
140965dfd: 48 89 d7                    	movq	%rdx, %rdi
140965e00: 49 89 ce                    	movq	%rcx, %r14
140965e03: 48 89 d1                    	movq	%rdx, %rcx
140965e06: e8 a5 22 22 00              	callq	0x140b880b0 <.text+0xb870b0>
140965e0b: 48 81 fa 92 24 00 00        	cmpq	$0x2492, %rdx           # imm = 0x2492
140965e12: be 92 24 00 00              	movl	$0x2492, %esi           # imm = 0x2492
140965e17: 48 0f 42 f2                 	cmovbq	%rdx, %rsi
140965e1b: 31 c9                       	xorl	%ecx, %ecx
140965e1d: a8 01                       	testb	$0x1, %al
140965e1f: b8 00 00 00 00              	movl	$0x0, %eax
140965e24: 48 0f 45 c6                 	cmovneq	%rsi, %rax
140965e28: 48 85 c0                    	testq	%rax, %rax
140965e2b: 4c 89 b5 a8 00 00 00        	movq	%r14, 0xa8(%rbp)
140965e32: 74 26                       	je	0x140965e5a <.text+0x964e5a>
140965e34: 48 6b d8 70                 	imulq	$0x70, %rax, %rbx
140965e38: 0f b6 05 ca ed cc 02        	movzbl	0x2ccedca(%rip), %eax   # 0x143634c09
140965e3f: ba 08 00 00 00              	movl	$0x8, %edx
140965e44: 48 89 d9                    	movq	%rbx, %rcx
140965e47: e8 c4 dd ca ff              	callq	0x140613c10 <.text+0x612c10>
140965e4c: 48 85 c0                    	testq	%rax, %rax
140965e4f: 0f 84 c0 01 00 00           	je	0x140966015 <.text+0x965015>
140965e55: 48 89 f1                    	movq	%rsi, %rcx
140965e58: eb 05                       	jmp	0x140965e5f <.text+0x964e5f>
140965e5a: b8 08 00 00 00              	movl	$0x8, %eax
140965e5f: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
140965e66: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
140965e6d: 48 c7 85 c0 00 00 00 00 00 00 00    	movq	$0x0, 0xc0(%rbp)
140965e78: 4c 8d 6d 19                 	leaq	0x19(%rbp), %r13
140965e7c: 48 8d 75 b8                 	leaq	-0x48(%rbp), %rsi
140965e80: 48 8d 5d a8                 	leaq	-0x58(%rbp), %rbx
140965e84: 4c 8d 75 18                 	leaq	0x18(%rbp), %r14
140965e88: 4c 8d a5 b0 00 00 00        	leaq	0xb0(%rbp), %r12
140965e8f: eb 68                       	jmp	0x140965ef9 <.text+0x964ef9>
140965e91: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140965ea0: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
140965ea7: 49 6b cf 70                 	imulq	$0x70, %r15, %rcx
140965eab: 0f 10 45 38                 	movups	0x38(%rbp), %xmm0
140965eaf: 0f 10 4d 48                 	movups	0x48(%rbp), %xmm1
140965eb3: 0f 10 55 58                 	movups	0x58(%rbp), %xmm2
140965eb7: 0f 10 5d 68                 	movups	0x68(%rbp), %xmm3
140965ebb: 0f 10 65 78                 	movups	0x78(%rbp), %xmm4
140965ebf: 0f 10 ad 88 00 00 00        	movups	0x88(%rbp), %xmm5
140965ec6: 0f 10 b5 98 00 00 00        	movups	0x98(%rbp), %xmm6
140965ecd: 0f 11 74 08 60              	movups	%xmm6, 0x60(%rax,%rcx)
140965ed2: 0f 11 6c 08 50              	movups	%xmm5, 0x50(%rax,%rcx)
140965ed7: 0f 11 64 08 40              	movups	%xmm4, 0x40(%rax,%rcx)
140965edc: 0f 11 5c 08 30              	movups	%xmm3, 0x30(%rax,%rcx)
140965ee1: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
140965ee6: 0f 11 4c 08 10              	movups	%xmm1, 0x10(%rax,%rcx)
140965eeb: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
140965eef: 49 ff c7                    	incq	%r15
140965ef2: 4c 89 bd c0 00 00 00        	movq	%r15, 0xc0(%rbp)
140965ef9: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140965efd: 48 3b 47 18                 	cmpq	0x18(%rdi), %rax
140965f01: 0f 84 ae 00 00 00           	je	0x140965fb5 <.text+0x964fb5>
140965f07: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
140965f0b: 48 89 4f 08                 	movq	%rcx, 0x8(%rdi)
140965f0f: 0f b6 08                    	movzbl	(%rax), %ecx
140965f12: 80 f9 06                    	cmpb	$0x6, %cl
140965f15: 0f 84 9a 00 00 00           	je	0x140965fb5 <.text+0x964fb5>
140965f1b: 88 4d 18                    	movb	%cl, 0x18(%rbp)
140965f1e: 0f 10 40 01                 	movups	0x1(%rax), %xmm0
140965f22: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
140965f26: 41 0f 11 4d 0f              	movups	%xmm1, 0xf(%r13)
140965f2b: 41 0f 11 45 00              	movups	%xmm0, (%r13)
140965f30: 41 b8 02 00 00 00           	movl	$0x2, %r8d
140965f36: 48 89 d9                    	movq	%rbx, %rcx
140965f39: 4c 89 f2                    	movq	%r14, %rdx
140965f3c: e8 4f 7a c3 00              	callq	0x14159d990 <.text+0x159c990>
140965f41: 48 8b 4d a8                 	movq	-0x58(%rbp), %rcx
140965f45: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
140965f49: 48 83 f9 03                 	cmpq	$0x3, %rcx
140965f4d: 0f 84 80 00 00 00           	je	0x140965fd3 <.text+0x964fd3>
140965f53: 0f 10 46 50                 	movups	0x50(%rsi), %xmm0
140965f57: 48 8d 55 48                 	leaq	0x48(%rbp), %rdx
140965f5b: 0f 11 42 50                 	movups	%xmm0, 0x50(%rdx)
140965f5f: 0f 10 46 40                 	movups	0x40(%rsi), %xmm0
140965f63: 0f 11 42 40                 	movups	%xmm0, 0x40(%rdx)
140965f67: 0f 10 06                    	movups	(%rsi), %xmm0
140965f6a: 0f 10 4e 10                 	movups	0x10(%rsi), %xmm1
140965f6e: 0f 10 56 20                 	movups	0x20(%rsi), %xmm2
140965f72: 0f 10 5e 30                 	movups	0x30(%rsi), %xmm3
140965f76: 0f 11 5a 30                 	movups	%xmm3, 0x30(%rdx)
140965f7a: 0f 11 52 20                 	movups	%xmm2, 0x20(%rdx)
140965f7e: 0f 11 4a 10                 	movups	%xmm1, 0x10(%rdx)
140965f82: 0f 11 02                    	movups	%xmm0, (%rdx)
140965f85: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
140965f89: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140965f8d: 4c 8b bd c0 00 00 00        	movq	0xc0(%rbp), %r15
140965f94: 4c 3b bd b0 00 00 00        	cmpq	0xb0(%rbp), %r15
140965f9b: 0f 85 ff fe ff ff           	jne	0x140965ea0 <.text+0x964ea0>
140965fa1: 4c 89 e1                    	movq	%r12, %rcx
140965fa4: 48 8d 15 f5 f3 14 02        	leaq	0x214f3f5(%rip), %rdx   # 0x142ab53a0
140965fab: e8 00 d1 bf ff              	callq	0x1405630b0 <.text+0x5620b0>
140965fb0: e9 eb fe ff ff              	jmp	0x140965ea0 <.text+0x964ea0>
140965fb5: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140965fbc: 48 8b b5 a8 00 00 00        	movq	0xa8(%rbp), %rsi
140965fc3: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
140965fc7: 0f 10 85 b0 00 00 00        	movups	0xb0(%rbp), %xmm0
140965fce: 0f 11 06                    	movups	%xmm0, (%rsi)
140965fd1: eb 24                       	jmp	0x140965ff7 <.text+0x964ff7>
140965fd3: 48 8b b5 a8 00 00 00        	movq	0xa8(%rbp), %rsi
140965fda: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
140965fde: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140965fe8: 48 89 06                    	movq	%rax, (%rsi)
140965feb: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
140965ff2: e8 c9 c8 f1 ff              	callq	0x1408828c0 <.text+0x8818c0>
140965ff7: 48 89 f0                    	movq	%rsi, %rax
140965ffa: 0f 28 b5 d0 00 00 00        	movaps	0xd0(%rbp), %xmm6
140966001: 48 81 c4 68 01 00 00        	addq	$0x168, %rsp            # imm = 0x168
140966008: 5b                          	popq	%rbx
140966009: 5f                          	popq	%rdi
14096600a: 5e                          	popq	%rsi
14096600b: 41 5c                       	popq	%r12
14096600d: 41 5d                       	popq	%r13
14096600f: 41 5e                       	popq	%r14
140966011: 41 5f                       	popq	%r15
140966013: 5d                          	popq	%rbp
140966014: c3                          	retq
140966015: 4c 8d 05 6c f3 14 02        	leaq	0x214f36c(%rip), %r8    # 0x142ab5388
14096601c: b9 08 00 00 00              	movl	$0x8, %ecx
140966021: 48 89 da                    	movq	%rbx, %rdx
140966024: e8 ba 2f f7 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140966029: cc                          	int3
14096602a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140966030: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140966035: 55                          	pushq	%rbp
140966036: 41 57                       	pushq	%r15
140966038: 41 56                       	pushq	%r14
14096603a: 41 55                       	pushq	%r13
14096603c: 41 54                       	pushq	%r12
14096603e: 56                          	pushq	%rsi
14096603f: 57                          	pushq	%rdi
140966040: 53                          	pushq	%rbx
140966041: 48 83 ec 38                 	subq	$0x38, %rsp
140966045: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14096604c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
140966051: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
140966058: e8 63 c8 f1 ff              	callq	0x1408828c0 <.text+0x8818c0>
14096605d: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
140966062: 48 83 c4 38                 	addq	$0x38, %rsp
140966066: 5b                          	popq	%rbx
140966067: 5f                          	popq	%rdi
140966068: 5e                          	popq	%rsi
140966069: 41 5c                       	popq	%r12
14096606b: 41 5d                       	popq	%r13
14096606d: 41 5e                       	popq	%r14
14096606f: 41 5f                       	popq	%r15
140966071: 5d                          	popq	%rbp
140966072: c3                          	retq
140966073: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140966080: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140966085: 55                          	pushq	%rbp
140966086: 41 57                       	pushq	%r15
140966088: 41 56                       	pushq	%r14
14096608a: 41 55                       	pushq	%r13
14096608c: 41 54                       	pushq	%r12
14096608e: 56                          	pushq	%rsi
14096608f: 57                          	pushq	%rdi
140966090: 53                          	pushq	%rbx
140966091: 48 83 ec 38                 	subq	$0x38, %rsp
140966095: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14096609c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1409660a1: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1409660a5: e8 56 30 f3 ff              	callq	0x140899100 <.text+0x898100>
1409660aa: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1409660af: 48 83 c4 38                 	addq	$0x38, %rsp
1409660b3: 5b                          	popq	%rbx
1409660b4: 5f                          	popq	%rdi
1409660b5: 5e                          	popq	%rsi
1409660b6: 41 5c                       	popq	%r12
1409660b8: 41 5d                       	popq	%r13
1409660ba: 41 5e                       	popq	%r14
1409660bc: 41 5f                       	popq	%r15
1409660be: 5d                          	popq	%rbp
1409660bf: c3                          	retq

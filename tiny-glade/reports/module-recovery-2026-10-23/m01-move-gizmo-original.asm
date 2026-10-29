
B版本内容/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14097d280: 55                          	pushq	%rbp
14097d281: 41 57                       	pushq	%r15
14097d283: 41 56                       	pushq	%r14
14097d285: 41 55                       	pushq	%r13
14097d287: 41 54                       	pushq	%r12
14097d289: 56                          	pushq	%rsi
14097d28a: 57                          	pushq	%rdi
14097d28b: 53                          	pushq	%rbx
14097d28c: 48 81 ec 38 02 00 00        	subq	$0x238, %rsp            # imm = 0x238
14097d293: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14097d29b: 0f 29 bd a0 01 00 00        	movaps	%xmm7, 0x1a0(%rbp)
14097d2a2: 0f 29 b5 90 01 00 00        	movaps	%xmm6, 0x190(%rbp)
14097d2a9: 48 c7 85 88 01 00 00 fe ff ff ff    	movq	$-0x2, 0x188(%rbp)
14097d2b4: 4d 89 ce                    	movq	%r9, %r14
14097d2b7: 4c 89 c3                    	movq	%r8, %rbx
14097d2ba: 48 89 d6                    	movq	%rdx, %rsi
14097d2bd: 4c 8b a5 28 02 00 00        	movq	0x228(%rbp), %r12
14097d2c4: f3 41 0f 10 44 24 30        	movss	0x30(%r12), %xmm0
14097d2cb: f3 0f 58 05 15 ce fd 01     	addss	0x1fdce15(%rip), %xmm0  # 0x14295a0e8
14097d2d3: f3 0f 5e 05 89 fd 04 02     	divss	0x204fd89(%rip), %xmm0  # 0x1429cd064
14097d2db: 48 89 cf                    	movq	%rcx, %rdi
14097d2de: 0f 57 c9                    	xorps	%xmm1, %xmm1
14097d2e1: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
14097d2e5: f3 0f 10 35 17 86 fa 01     	movss	0x1fa8617(%rip), %xmm6  # 0x142925904
14097d2ed: 0f 28 c6                    	movaps	%xmm6, %xmm0
14097d2f0: f3 0f 5d c1                 	minss	%xmm1, %xmm0
14097d2f4: 0f 28 c8                    	movaps	%xmm0, %xmm1
14097d2f7: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
14097d2fb: f3 0f 58 c0                 	addss	%xmm0, %xmm0
14097d2ff: f3 0f 10 1d 71 fd 04 02     	movss	0x204fd71(%rip), %xmm3  # 0x1429cd078
14097d307: f3 0f 5c d8                 	subss	%xmm0, %xmm3
14097d30b: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
14097d30f: 0f 28 fe                    	movaps	%xmm6, %xmm7
14097d312: f3 0f 5c fb                 	subss	%xmm3, %xmm7
14097d316: f3 0f 10 15 52 85 fa 01     	movss	0x1fa8552(%rip), %xmm2  # 0x142925870
14097d31e: f3 0f 59 fa                 	mulss	%xmm2, %xmm7
14097d322: f3 0f 58 fb                 	addss	%xmm3, %xmm7
14097d326: f3 0f 10 0d a2 85 fa 01     	movss	0x1fa85a2(%rip), %xmm1  # 0x1429258d0
14097d32e: f3 0f 59 cf                 	mulss	%xmm7, %xmm1
14097d332: 8b 41 08                    	movl	0x8(%rcx), %eax
14097d335: 89 85 a8 00 00 00           	movl	%eax, 0xa8(%rbp)
14097d33b: 48 8b 01                    	movq	(%rcx), %rax
14097d33e: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
14097d345: 0f 28 02                    	movaps	(%rdx), %xmm0
14097d348: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
14097d34f: 48 b8 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
14097d359: 48 89 85 ac 00 00 00        	movq	%rax, 0xac(%rbp)
14097d360: c7 85 b4 00 00 00 00 00 80 3f       	movl	$0x3f800000, 0xb4(%rbp) # imm = 0x3F800000
14097d36a: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
14097d371: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14097d376: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
14097d37a: 0f 28 d9                    	movaps	%xmm1, %xmm3
14097d37d: e8 5e cd 15 00              	callq	0x140ada0e0 <.text+0xad90e0>
14097d382: 48 8b 03                    	movq	(%rbx), %rax
14097d385: 8b 58 10                    	movl	0x10(%rax), %ebx
14097d388: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
14097d38f: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14097d393: 48 8d 05 c6 42 00 00        	leaq	0x42c6(%rip), %rax      # 0x140981660 <.text+0x980660>
14097d39a: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14097d39e: 48 8d 05 8b 99 13 02        	leaq	0x213998b(%rip), %rax   # 0x142ab6d30
14097d3a5: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14097d3ac: 48 c7 85 98 00 00 00 01 00 00 00    	movq	$0x1, 0x98(%rbp)
14097d3b7: 48 8d 45 48                 	leaq	0x48(%rbp), %rax
14097d3bb: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
14097d3c2: 48 c7 85 a8 00 00 00 01 00 00 00    	movq	$0x1, 0xa8(%rbp)
14097d3cd: 48 c7 85 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbp)
14097d3d8: c6 85 87 01 00 00 01        	movb	$0x1, 0x187(%rbp)
14097d3df: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14097d3e3: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
14097d3ea: e8 a1 cb 3f 00              	callq	0x140d79f90 <.text+0xd78f90>
14097d3ef: 31 c0                       	xorl	%eax, %eax
14097d3f1: 83 fb 03                    	cmpl	$0x3, %ebx
14097d3f4: 0f 93 c0                    	setae	%al
14097d3f7: 8d 0c 00                    	leal	(%rax,%rax), %ecx
14097d3fa: 8d 14 40                    	leal	(%rax,%rax,2), %edx
14097d3fd: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
14097d401: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
14097d405: 4c 8b 45 c0                 	movq	-0x40(%rbp), %r8
14097d409: 4c 89 45 20                 	movq	%r8, 0x20(%rbp)
14097d40d: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
14097d411: 0f 11 45 4f                 	movups	%xmm0, 0x4f(%rbp)
14097d415: 4c 8b 45 08                 	movq	0x8(%rbp), %r8
14097d419: 4c 89 45 5f                 	movq	%r8, 0x5f(%rbp)
14097d41d: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
14097d421: 0f 11 85 19 01 00 00        	movups	%xmm0, 0x119(%rbp)
14097d428: 4c 8b 45 57                 	movq	0x57(%rbp), %r8
14097d42c: 4c 89 85 28 01 00 00        	movq	%r8, 0x128(%rbp)
14097d433: 4c 8b 45 5f                 	movq	0x5f(%rbp), %r8
14097d437: 4c 89 85 30 01 00 00        	movq	%r8, 0x130(%rbp)
14097d43e: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
14097d449: 48 c7 85 98 00 00 00 08 00 00 00    	movq	$0x8, 0x98(%rbp)
14097d454: 48 c7 85 a0 00 00 00 00 00 00 00    	movq	$0x0, 0xa0(%rbp)
14097d45f: 89 85 a8 00 00 00           	movl	%eax, 0xa8(%rbp)
14097d465: 48 c7 85 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbp)
14097d470: 89 8d c0 00 00 00           	movl	%ecx, 0xc0(%rbp)
14097d476: 89 95 d8 00 00 00           	movl	%edx, 0xd8(%rbp)
14097d47c: 89 95 f0 00 00 00           	movl	%edx, 0xf0(%rbp)
14097d482: c6 85 08 01 00 00 00        	movb	$0x0, 0x108(%rbp)
14097d489: c6 85 18 01 00 00 02        	movb	$0x2, 0x118(%rbp)
14097d490: 66 c7 85 38 01 00 00 00 01  	movw	$0x100, 0x138(%rbp)     # imm = 0x100
14097d499: c6 85 87 01 00 00 00        	movb	$0x0, 0x187(%rbp)
14097d4a0: 4c 8d 7d 10                 	leaq	0x10(%rbp), %r15
14097d4a4: 4c 8d ad 90 00 00 00        	leaq	0x90(%rbp), %r13
14097d4ab: 4c 89 f1                    	movq	%r14, %rcx
14097d4ae: 4c 89 fa                    	movq	%r15, %rdx
14097d4b1: 4d 89 e8                    	movq	%r13, %r8
14097d4b4: e8 37 5d fa ff              	callq	0x1409231f0 <.text+0x9221f0>
14097d4b9: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14097d4c0: 48 89 95 58 01 00 00        	movq	%rdx, 0x158(%rbp)
14097d4c7: f3 41 0f 10 84 24 80 02 00 00       	movss	0x280(%r12), %xmm0
14097d4d1: 0f 57 c9                    	xorps	%xmm1, %xmm1
14097d4d4: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
14097d4d7: 0f 86 2e 02 00 00           	jbe	0x14097d70b <.text+0x97c70b>
14097d4dd: f3 0f 10 95 30 02 00 00     	movss	0x230(%rbp), %xmm2
14097d4e5: f3 0f 10 07                 	movss	(%rdi), %xmm0
14097d4e9: f3 0f 10 4f 08              	movss	0x8(%rdi), %xmm1
14097d4ee: f3 0f 11 85 90 00 00 00     	movss	%xmm0, 0x90(%rbp)
14097d4f6: f3 0f 11 8d 94 00 00 00     	movss	%xmm1, 0x94(%rbp)
14097d4fe: c6 85 87 01 00 00 00        	movb	$0x0, 0x187(%rbp)
14097d505: 4c 8d a5 48 01 00 00        	leaq	0x148(%rbp), %r12
14097d50c: 4c 8d b5 90 00 00 00        	leaq	0x90(%rbp), %r14
14097d513: 4c 89 e1                    	movq	%r12, %rcx
14097d516: 4c 89 f2                    	movq	%r14, %rdx
14097d519: e8 c2 f8 31 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14097d51e: 8b 47 08                    	movl	0x8(%rdi), %eax
14097d521: 89 45 18                    	movl	%eax, 0x18(%rbp)
14097d524: 48 8b 07                    	movq	(%rdi), %rax
14097d527: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
14097d52b: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
14097d532: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14097d539: 48 8d 05 20 41 00 00        	leaq	0x4120(%rip), %rax      # 0x140981660 <.text+0x980660>
14097d540: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
14097d547: 48 89 bd a0 00 00 00        	movq	%rdi, 0xa0(%rbp)
14097d54e: 48 8d 05 ab e4 3c 00        	leaq	0x3ce4ab(%rip), %rax    # 0x140d4ba00 <.text+0xd4aa00>
14097d555: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
14097d55c: 4c 89 a5 b0 00 00 00        	movq	%r12, 0xb0(%rbp)
14097d563: 48 8d 05 66 e2 3c 00        	leaq	0x3ce266(%rip), %rax    # 0x140d4b7d0 <.text+0xd4a7d0>
14097d56a: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
14097d571: 4c 89 bd c0 00 00 00        	movq	%r15, 0xc0(%rbp)
14097d578: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
14097d57f: 48 8d 05 ca 97 13 02        	leaq	0x21397ca(%rip), %rax   # 0x142ab6d50
14097d586: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14097d58a: 48 c7 45 50 04 00 00 00     	movq	$0x4, 0x50(%rbp)
14097d592: 4c 89 75 58                 	movq	%r14, 0x58(%rbp)
14097d596: 48 c7 45 60 04 00 00 00     	movq	$0x4, 0x60(%rbp)
14097d59e: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
14097d5a6: c6 85 87 01 00 00 00        	movb	$0x0, 0x187(%rbp)
14097d5ad: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
14097d5b1: 48 8d 55 48                 	leaq	0x48(%rbp), %rdx
14097d5b5: e8 d6 c9 3f 00              	callq	0x140d79f90 <.text+0xd78f90>
14097d5ba: 4c 8b 7d e0                 	movq	-0x20(%rbp), %r15
14097d5be: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
14097d5c2: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
14097d5c9: 4c 8b 75 f0                 	movq	-0x10(%rbp), %r14
14097d5cd: 8b 85 50 01 00 00           	movl	0x150(%rbp), %eax
14097d5d3: 89 85 98 00 00 00           	movl	%eax, 0x98(%rbp)
14097d5d9: 48 8b 85 48 01 00 00        	movq	0x148(%rbp), %rax
14097d5e0: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14097d5e7: 48 8b 07                    	movq	(%rdi), %rax
14097d5ea: 48 89 85 9c 00 00 00        	movq	%rax, 0x9c(%rbp)
14097d5f1: 8b 47 08                    	movl	0x8(%rdi), %eax
14097d5f4: 89 85 a4 00 00 00           	movl	%eax, 0xa4(%rbp)
14097d5fa: 0f b6 05 08 76 cb 02        	movzbl	0x2cb7608(%rip), %eax   # 0x143634c09
14097d601: b9 28 00 00 00              	movl	$0x28, %ecx
14097d606: ba 08 00 00 00              	movl	$0x8, %edx
14097d60b: e8 00 66 c9 ff              	callq	0x140613c10 <.text+0x612c10>
14097d610: 48 85 c0                    	testq	%rax, %rax
14097d613: 4c 89 bd 78 01 00 00        	movq	%r15, 0x178(%rbp)
14097d61a: 0f 84 59 04 00 00           	je	0x14097da79 <.text+0x97ca79>
14097d620: 49 89 c4                    	movq	%rax, %r12
14097d623: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
14097d62a: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
14097d632: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
14097d639: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
14097d63d: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
14097d644: 49 89 44 24 20              	movq	%rax, 0x20(%r12)
14097d649: 4c 89 a5 90 00 00 00        	movq	%r12, 0x90(%rbp)
14097d650: 48 8d 1d 39 97 13 02        	leaq	0x2139739(%rip), %rbx   # 0x142ab6d90
14097d657: 48 89 9d 98 00 00 00        	movq	%rbx, 0x98(%rbp)
14097d65e: 4d 85 f6                    	testq	%r14, %r14
14097d661: 0f 88 23 04 00 00           	js	0x14097da8a <.text+0x97ca8a>
14097d667: 74 36                       	je	0x14097d69f <.text+0x97c69f>
14097d669: 4c 89 a5 68 01 00 00        	movq	%r12, 0x168(%rbp)
14097d670: 0f b6 05 92 75 cb 02        	movzbl	0x2cb7592(%rip), %eax   # 0x143634c09
14097d677: 41 bc 01 00 00 00           	movl	$0x1, %r12d
14097d67d: ba 01 00 00 00              	movl	$0x1, %edx
14097d682: 4c 89 f1                    	movq	%r14, %rcx
14097d685: e8 86 65 c9 ff              	callq	0x140613c10 <.text+0x612c10>
14097d68a: 48 85 c0                    	testq	%rax, %rax
14097d68d: 0f 84 01 04 00 00           	je	0x14097da94 <.text+0x97ca94>
14097d693: 49 89 c7                    	movq	%rax, %r15
14097d696: 4c 8b a5 68 01 00 00        	movq	0x168(%rbp), %r12
14097d69d: eb 06                       	jmp	0x14097d6a5 <.text+0x97c6a5>
14097d69f: 41 bf 01 00 00 00           	movl	$0x1, %r15d
14097d6a5: 4c 89 f9                    	movq	%r15, %rcx
14097d6a8: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
14097d6af: 4d 89 f0                    	movq	%r14, %r8
14097d6b2: e8 49 65 fa 01              	callq	0x142923c00 <.text+0x2922c00>
14097d6b7: 4c 89 b5 90 00 00 00        	movq	%r14, 0x90(%rbp)
14097d6be: 4c 89 bd 98 00 00 00        	movq	%r15, 0x98(%rbp)
14097d6c5: 4c 89 b5 a0 00 00 00        	movq	%r14, 0xa0(%rbp)
14097d6cc: 4c 89 a5 a8 00 00 00        	movq	%r12, 0xa8(%rbp)
14097d6d3: 48 89 9d b0 00 00 00        	movq	%rbx, 0xb0(%rbp)
14097d6da: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
14097d6e1: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
14097d6e8: e8 93 f1 ae 00              	callq	0x14146c880 <.text+0x146b880>
14097d6ed: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
14097d6f4: 48 85 d2                    	testq	%rdx, %rdx
14097d6f7: 74 12                       	je	0x14097d70b <.text+0x97c70b>
14097d6f9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14097d6ff: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14097d706: e8 15 65 c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097d70b: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
14097d712: 0f b6 41 18                 	movzbl	0x18(%rcx), %eax
14097d716: 85 c0                       	testl	%eax, %eax
14097d718: 74 35                       	je	0x14097d74f <.text+0x97c74f>
14097d71a: 83 f8 01                    	cmpl	$0x1, %eax
14097d71d: 75 15                       	jne	0x14097d734 <.text+0x97c734>
14097d71f: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14097d726: 48 39 41 38                 	cmpq	%rax, 0x38(%rcx)
14097d72a: 75 23                       	jne	0x14097d74f <.text+0x97c74f>
14097d72c: 80 79 19 00                 	cmpb	$0x0, 0x19(%rcx)
14097d730: 75 15                       	jne	0x14097d747 <.text+0x97c747>
14097d732: eb 1b                       	jmp	0x14097d74f <.text+0x97c74f>
14097d734: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14097d73b: 48 39 41 38                 	cmpq	%rax, 0x38(%rcx)
14097d73f: 75 0e                       	jne	0x14097d74f <.text+0x97c74f>
14097d741: 80 79 19 02                 	cmpb	$0x2, 0x19(%rcx)
14097d745: 74 08                       	je	0x14097d74f <.text+0x97c74f>
14097d747: f3 0f 10 35 d9 f9 04 02     	movss	0x204f9d9(%rip), %xmm6  # 0x1429cd128
14097d74f: f3 0f 59 fe                 	mulss	%xmm6, %xmm7
14097d753: f3 0f 11 bd 64 01 00 00     	movss	%xmm7, 0x164(%rbp)
14097d75b: c6 45 48 00                 	movb	$0x0, 0x48(%rbp)
14097d75f: 48 8d 05 2a 95 13 02        	leaq	0x213952a(%rip), %rax   # 0x142ab6c90
14097d766: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14097d76a: 48 c7 45 58 0a 00 00 00     	movq	$0xa, 0x58(%rbp)
14097d772: c6 45 60 01                 	movb	$0x1, 0x60(%rbp)
14097d776: 48 8d 05 43 96 13 02        	leaq	0x2139643(%rip), %rax   # 0x142ab6dc0
14097d77d: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14097d781: 48 c7 45 70 12 00 00 00     	movq	$0x12, 0x70(%rbp)
14097d789: 48 c7 85 80 00 00 00 02 00 00 00    	movq	$0x2, 0x80(%rbp)
14097d794: 31 c0                       	xorl	%eax, %eax
14097d796: eb 0c                       	jmp	0x14097d7a4 <.text+0x97c7a4>
14097d798: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14097d7a0: 48 8b 45 40                 	movq	0x40(%rbp), %rax
14097d7a4: 48 83 f8 02                 	cmpq	$0x2, %rax
14097d7a8: 0f 84 74 02 00 00           	je	0x14097da22 <.text+0x97ca22>
14097d7ae: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
14097d7b2: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
14097d7b6: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14097d7ba: 0f b6 5c c5 48              	movzbl	0x48(%rbp,%rax,8), %ebx
14097d7bf: 80 fb 02                    	cmpb	$0x2, %bl
14097d7c2: 0f 84 5a 02 00 00           	je	0x14097da22 <.text+0x97ca22>
14097d7c8: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
14097d7cc: 48 8d 04 c5 48 00 00 00     	leaq	0x48(,%rax,8), %rax
14097d7d4: 48 01 e8                    	addq	%rbp, %rax
14097d7d7: 0f 10 40 08                 	movups	0x8(%rax), %xmm0
14097d7db: 0f 11 85 48 01 00 00        	movups	%xmm0, 0x148(%rbp)
14097d7e2: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
14097d7e9: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14097d7f0: 48 8d 05 69 3e 00 00        	leaq	0x3e69(%rip), %rax      # 0x140981660 <.text+0x980660>
14097d7f7: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
14097d7fe: 48 89 bd a0 00 00 00        	movq	%rdi, 0xa0(%rbp)
14097d805: 48 8d 05 f4 e1 3c 00        	leaq	0x3ce1f4(%rip), %rax    # 0x140d4ba00 <.text+0xd4aa00>
14097d80c: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
14097d813: 48 8d 85 48 01 00 00        	leaq	0x148(%rbp), %rax
14097d81a: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
14097d821: 48 8d 05 68 c6 78 ff        	leaq	-0x873998(%rip), %rax   # 0x140109e90 <.text+0x108e90>
14097d828: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
14097d82f: 48 8d 85 64 01 00 00        	leaq	0x164(%rbp), %rax
14097d836: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14097d83d: 48 8d 05 cc f0 40 00        	leaq	0x40f0cc(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
14097d844: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
14097d84b: 48 89 b5 d0 00 00 00        	movq	%rsi, 0xd0(%rbp)
14097d852: 48 8d 05 37 d7 3c 00        	leaq	0x3cd737(%rip), %rax    # 0x140d4af90 <.text+0xd49f90>
14097d859: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
14097d860: 48 8d 05 71 95 13 02        	leaq	0x2139571(%rip), %rax   # 0x142ab6dd8
14097d867: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
14097d86b: 48 c7 45 18 05 00 00 00     	movq	$0x5, 0x18(%rbp)
14097d873: 4c 89 6d 20                 	movq	%r13, 0x20(%rbp)
14097d877: 48 c7 45 28 05 00 00 00     	movq	$0x5, 0x28(%rbp)
14097d87f: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
14097d887: c6 85 87 01 00 00 00        	movb	$0x0, 0x187(%rbp)
14097d88e: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
14097d892: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
14097d896: e8 f5 c6 3f 00              	callq	0x140d79f90 <.text+0xd78f90>
14097d89b: 4d 89 ef                    	movq	%r13, %r15
14097d89e: 48 8b 45 c8                 	movq	-0x38(%rbp), %rax
14097d8a2: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14097d8a9: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
14097d8ad: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
14097d8b4: 4c 8b 75 d8                 	movq	-0x28(%rbp), %r14
14097d8b8: 4c 8b a5 48 01 00 00        	movq	0x148(%rbp), %r12
14097d8bf: 48 89 f1                    	movq	%rsi, %rcx
14097d8c2: 48 8b b5 50 01 00 00        	movq	0x150(%rbp), %rsi
14097d8c9: f3 0f 10 b5 64 01 00 00     	movss	0x164(%rbp), %xmm6
14097d8d1: 48 8b 07                    	movq	(%rdi), %rax
14097d8d4: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14097d8db: 8b 47 08                    	movl	0x8(%rdi), %eax
14097d8de: 89 85 98 00 00 00           	movl	%eax, 0x98(%rbp)
14097d8e4: 49 89 cd                    	movq	%rcx, %r13
14097d8e7: 0f 28 39                    	movaps	(%rcx), %xmm7
14097d8ea: 0f b6 05 18 73 cb 02        	movzbl	0x2cb7318(%rip), %eax   # 0x143634c09
14097d8f1: b9 50 00 00 00              	movl	$0x50, %ecx
14097d8f6: ba 10 00 00 00              	movl	$0x10, %edx
14097d8fb: e8 10 63 c9 ff              	callq	0x140613c10 <.text+0x612c10>
14097d900: 48 85 c0                    	testq	%rax, %rax
14097d903: 0f 84 49 01 00 00           	je	0x14097da52 <.text+0x97ca52>
14097d909: 48 89 c1                    	movq	%rax, %rcx
14097d90c: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
14097d913: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
14097d91b: 4c 89 60 10                 	movq	%r12, 0x10(%rax)
14097d91f: 48 89 70 18                 	movq	%rsi, 0x18(%rax)
14097d923: 0f 29 78 20                 	movaps	%xmm7, 0x20(%rax)
14097d927: f3 0f 11 70 30              	movss	%xmm6, 0x30(%rax)
14097d92c: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
14097d933: 48 89 41 34                 	movq	%rax, 0x34(%rcx)
14097d937: 8b 85 98 00 00 00           	movl	0x98(%rbp), %eax
14097d93d: 89 41 3c                    	movl	%eax, 0x3c(%rcx)
14097d940: 88 59 40                    	movb	%bl, 0x40(%rcx)
14097d943: 48 89 8d 90 00 00 00        	movq	%rcx, 0x90(%rbp)
14097d94a: 48 8d 05 d7 94 13 02        	leaq	0x21394d7(%rip), %rax   # 0x142ab6e28
14097d951: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
14097d958: 4d 85 f6                    	testq	%r14, %r14
14097d95b: 48 89 8d 68 01 00 00        	movq	%rcx, 0x168(%rbp)
14097d962: 0f 88 fb 00 00 00           	js	0x14097da63 <.text+0x97ca63>
14097d968: 4c 89 ee                    	movq	%r13, %rsi
14097d96b: 74 33                       	je	0x14097d9a0 <.text+0x97c9a0>
14097d96d: 0f b6 05 95 72 cb 02        	movzbl	0x2cb7295(%rip), %eax   # 0x143634c09
14097d974: bb 01 00 00 00              	movl	$0x1, %ebx
14097d979: ba 01 00 00 00              	movl	$0x1, %edx
14097d97e: 4c 89 f1                    	movq	%r14, %rcx
14097d981: e8 8a 62 c9 ff              	callq	0x140613c10 <.text+0x612c10>
14097d986: 48 85 c0                    	testq	%rax, %rax
14097d989: 0f 84 d6 00 00 00           	je	0x14097da65 <.text+0x97ca65>
14097d98f: 49 89 c4                    	movq	%rax, %r12
14097d992: eb 12                       	jmp	0x14097d9a6 <.text+0x97c9a6>
14097d994: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14097d9a0: 41 bc 01 00 00 00           	movl	$0x1, %r12d
14097d9a6: 4d 89 fd                    	movq	%r15, %r13
14097d9a9: 4c 89 e1                    	movq	%r12, %rcx
14097d9ac: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
14097d9b3: 4d 89 f0                    	movq	%r14, %r8
14097d9b6: e8 45 62 fa 01              	callq	0x142923c00 <.text+0x2922c00>
14097d9bb: 4c 89 b5 90 00 00 00        	movq	%r14, 0x90(%rbp)
14097d9c2: 4c 89 a5 98 00 00 00        	movq	%r12, 0x98(%rbp)
14097d9c9: 4c 89 b5 a0 00 00 00        	movq	%r14, 0xa0(%rbp)
14097d9d0: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14097d9d7: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
14097d9de: 48 8d 05 43 94 13 02        	leaq	0x2139443(%rip), %rax   # 0x142ab6e28
14097d9e5: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
14097d9ec: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
14097d9f3: 4c 89 fa                    	movq	%r15, %rdx
14097d9f6: e8 85 ee ae 00              	callq	0x14146c880 <.text+0x146b880>
14097d9fb: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
14097da02: 48 85 d2                    	testq	%rdx, %rdx
14097da05: 0f 84 95 fd ff ff           	je	0x14097d7a0 <.text+0x97c7a0>
14097da0b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14097da11: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14097da18: e8 03 62 c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097da1d: e9 7e fd ff ff              	jmp	0x14097d7a0 <.text+0x97c7a0>
14097da22: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
14097da29: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
14097da30: 0f 28 b5 90 01 00 00        	movaps	0x190(%rbp), %xmm6
14097da37: 0f 28 bd a0 01 00 00        	movaps	0x1a0(%rbp), %xmm7
14097da3e: 48 81 c4 38 02 00 00        	addq	$0x238, %rsp            # imm = 0x238
14097da45: 5b                          	popq	%rbx
14097da46: 5f                          	popq	%rdi
14097da47: 5e                          	popq	%rsi
14097da48: 41 5c                       	popq	%r12
14097da4a: 41 5d                       	popq	%r13
14097da4c: 41 5e                       	popq	%r14
14097da4e: 41 5f                       	popq	%r15
14097da50: 5d                          	popq	%rbp
14097da51: c3                          	retq
14097da52: b9 10 00 00 00              	movl	$0x10, %ecx
14097da57: ba 50 00 00 00              	movl	$0x50, %edx
14097da5c: e8 9f b5 f5 01              	callq	0x1428d9000 <.text+0x28d8000>
14097da61: eb 43                       	jmp	0x14097daa6 <.text+0x97caa6>
14097da63: 31 db                       	xorl	%ebx, %ebx
14097da65: 4c 8d 05 b4 b4 11 02        	leaq	0x211b4b4(%rip), %r8    # 0x142a98f20
14097da6c: 48 89 d9                    	movq	%rbx, %rcx
14097da6f: 4c 89 f2                    	movq	%r14, %rdx
14097da72: e8 6c b5 f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14097da77: eb 2d                       	jmp	0x14097daa6 <.text+0x97caa6>
14097da79: b9 08 00 00 00              	movl	$0x8, %ecx
14097da7e: ba 28 00 00 00              	movl	$0x28, %edx
14097da83: e8 78 b5 f5 01              	callq	0x1428d9000 <.text+0x28d8000>
14097da88: eb 1c                       	jmp	0x14097daa6 <.text+0x97caa6>
14097da8a: 4c 89 a5 68 01 00 00        	movq	%r12, 0x168(%rbp)
14097da91: 45 31 e4                    	xorl	%r12d, %r12d
14097da94: 4c 8d 05 85 b4 11 02        	leaq	0x211b485(%rip), %r8    # 0x142a98f20
14097da9b: 4c 89 e1                    	movq	%r12, %rcx
14097da9e: 4c 89 f2                    	movq	%r14, %rdx
14097daa1: e8 3d b5 f5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14097daa6: 0f 0b                       	ud2
14097daa8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14097dab0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097dab5: 55                          	pushq	%rbp
14097dab6: 41 57                       	pushq	%r15
14097dab8: 41 56                       	pushq	%r14
14097daba: 41 55                       	pushq	%r13
14097dabc: 41 54                       	pushq	%r12
14097dabe: 56                          	pushq	%rsi
14097dabf: 57                          	pushq	%rdi
14097dac0: 53                          	pushq	%rbx
14097dac1: 48 83 ec 48                 	subq	$0x48, %rsp
14097dac5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097dacc: 0f 29 7c 24 20              	movaps	%xmm7, 0x20(%rsp)
14097dad1: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
14097dad6: 48 83 bd 78 01 00 00 00     	cmpq	$0x0, 0x178(%rbp)
14097dade: 74 19                       	je	0x14097daf9 <.text+0x97caf9>
14097dae0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14097dae6: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14097daed: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
14097daf4: e8 27 61 c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097daf9: c6 85 87 01 00 00 00        	movb	$0x0, 0x187(%rbp)
14097db00: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14097db05: 0f 28 7c 24 20              	movaps	0x20(%rsp), %xmm7
14097db0a: 48 83 c4 48                 	addq	$0x48, %rsp
14097db0e: 5b                          	popq	%rbx
14097db0f: 5f                          	popq	%rdi
14097db10: 5e                          	popq	%rsi
14097db11: 41 5c                       	popq	%r12
14097db13: 41 5d                       	popq	%r13
14097db15: 41 5e                       	popq	%r14
14097db17: 41 5f                       	popq	%r15
14097db19: 5d                          	popq	%rbp
14097db1a: c3                          	retq
14097db1b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14097db20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097db25: 55                          	pushq	%rbp
14097db26: 41 57                       	pushq	%r15
14097db28: 41 56                       	pushq	%r14
14097db2a: 41 55                       	pushq	%r13
14097db2c: 41 54                       	pushq	%r12
14097db2e: 56                          	pushq	%rsi
14097db2f: 57                          	pushq	%rdi
14097db30: 53                          	pushq	%rbx
14097db31: 48 83 ec 48                 	subq	$0x48, %rsp
14097db35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097db3c: 0f 29 7c 24 20              	movaps	%xmm7, 0x20(%rsp)
14097db41: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
14097db46: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14097db4d: f0                          	lock
14097db4e: 48 ff 08                    	decq	(%rax)
14097db51: 75 0c                       	jne	0x14097db5f <.text+0x97cb5f>
14097db53: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
14097db5a: e8 71 1e cd ff              	callq	0x14064f9d0 <.text+0x64e9d0>
14097db5f: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14097db64: 0f 28 7c 24 20              	movaps	0x20(%rsp), %xmm7
14097db69: 48 83 c4 48                 	addq	$0x48, %rsp
14097db6d: 5b                          	popq	%rbx
14097db6e: 5f                          	popq	%rdi
14097db6f: 5e                          	popq	%rsi
14097db70: 41 5c                       	popq	%r12
14097db72: 41 5d                       	popq	%r13
14097db74: 41 5e                       	popq	%r14
14097db76: 41 5f                       	popq	%r15
14097db78: 5d                          	popq	%rbp
14097db79: c3                          	retq
14097db7a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14097db80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097db85: 55                          	pushq	%rbp
14097db86: 41 57                       	pushq	%r15
14097db88: 41 56                       	pushq	%r14
14097db8a: 41 55                       	pushq	%r13
14097db8c: 41 54                       	pushq	%r12
14097db8e: 56                          	pushq	%rsi
14097db8f: 57                          	pushq	%rdi
14097db90: 53                          	pushq	%rbx
14097db91: 48 83 ec 48                 	subq	$0x48, %rsp
14097db95: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097db9c: 0f 29 7c 24 20              	movaps	%xmm7, 0x20(%rsp)
14097dba1: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
14097dba6: 48 83 bd 78 01 00 00 00     	cmpq	$0x0, 0x178(%rbp)
14097dbae: 74 19                       	je	0x14097dbc9 <.text+0x97cbc9>
14097dbb0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14097dbb6: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14097dbbd: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
14097dbc4: e8 57 60 c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097dbc9: c6 85 87 01 00 00 00        	movb	$0x0, 0x187(%rbp)
14097dbd0: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14097dbd5: 0f 28 7c 24 20              	movaps	0x20(%rsp), %xmm7
14097dbda: 48 83 c4 48                 	addq	$0x48, %rsp
14097dbde: 5b                          	popq	%rbx
14097dbdf: 5f                          	popq	%rdi
14097dbe0: 5e                          	popq	%rsi
14097dbe1: 41 5c                       	popq	%r12
14097dbe3: 41 5d                       	popq	%r13
14097dbe5: 41 5e                       	popq	%r14
14097dbe7: 41 5f                       	popq	%r15
14097dbe9: 5d                          	popq	%rbp
14097dbea: c3                          	retq
14097dbeb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14097dbf0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097dbf5: 55                          	pushq	%rbp
14097dbf6: 41 57                       	pushq	%r15
14097dbf8: 41 56                       	pushq	%r14
14097dbfa: 41 55                       	pushq	%r13
14097dbfc: 41 54                       	pushq	%r12
14097dbfe: 56                          	pushq	%rsi
14097dbff: 57                          	pushq	%rdi
14097dc00: 53                          	pushq	%rbx
14097dc01: 48 83 ec 48                 	subq	$0x48, %rsp
14097dc05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097dc0c: 0f 29 7c 24 20              	movaps	%xmm7, 0x20(%rsp)
14097dc11: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
14097dc16: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14097dc1d: f0                          	lock
14097dc1e: 48 ff 08                    	decq	(%rax)
14097dc21: 75 0c                       	jne	0x14097dc2f <.text+0x97cc2f>
14097dc23: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
14097dc2a: e8 a1 1d cd ff              	callq	0x14064f9d0 <.text+0x64e9d0>
14097dc2f: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14097dc34: 0f 28 7c 24 20              	movaps	0x20(%rsp), %xmm7
14097dc39: 48 83 c4 48                 	addq	$0x48, %rsp
14097dc3d: 5b                          	popq	%rbx
14097dc3e: 5f                          	popq	%rdi
14097dc3f: 5e                          	popq	%rsi
14097dc40: 41 5c                       	popq	%r12
14097dc42: 41 5d                       	popq	%r13
14097dc44: 41 5e                       	popq	%r14
14097dc46: 41 5f                       	popq	%r15
14097dc48: 5d                          	popq	%rbp
14097dc49: c3                          	retq
14097dc4a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14097dc50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14097dc55: 55                          	pushq	%rbp
14097dc56: 41 57                       	pushq	%r15
14097dc58: 41 56                       	pushq	%r14
14097dc5a: 41 55                       	pushq	%r13
14097dc5c: 41 54                       	pushq	%r12
14097dc5e: 56                          	pushq	%rsi
14097dc5f: 57                          	pushq	%rdi
14097dc60: 53                          	pushq	%rbx
14097dc61: 48 83 ec 48                 	subq	$0x48, %rsp
14097dc65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14097dc6c: 0f 29 7c 24 20              	movaps	%xmm7, 0x20(%rsp)
14097dc71: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
14097dc76: 80 bd 87 01 00 00 00        	cmpb	$0x0, 0x187(%rbp)
14097dc7d: 74 20                       	je	0x14097dc9f <.text+0x97cc9f>
14097dc7f: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14097dc83: 48 85 c0                    	testq	%rax, %rax
14097dc86: 74 17                       	je	0x14097dc9f <.text+0x97cc9f>
14097dc88: 48 8b 4d 00                 	movq	(%rbp), %rcx
14097dc8c: 48 c1 e0 02                 	shlq	$0x2, %rax
14097dc90: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
14097dc94: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14097dc9a: e8 81 5f c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097dc9f: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14097dca4: 0f 28 7c 24 20              	movaps	0x20(%rsp), %xmm7
14097dca9: 48 83 c4 48                 	addq	$0x48, %rsp
14097dcad: 5b                          	popq	%rbx
14097dcae: 5f                          	popq	%rdi
14097dcaf: 5e                          	popq	%rsi
14097dcb0: 41 5c                       	popq	%r12
14097dcb2: 41 5d                       	popq	%r13
14097dcb4: 41 5e                       	popq	%r14
14097dcb6: 41 5f                       	popq	%r15
14097dcb8: 5d                          	popq	%rbp
14097dcb9: c3                          	retq


B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141c4f400: 41 57                       	pushq	%r15
141c4f402: 41 56                       	pushq	%r14
141c4f404: 41 55                       	pushq	%r13
141c4f406: 41 54                       	pushq	%r12
141c4f408: 56                          	pushq	%rsi
141c4f409: 57                          	pushq	%rdi
141c4f40a: 55                          	pushq	%rbp
141c4f40b: 53                          	pushq	%rbx
141c4f40c: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
141c4f413: 44 0f 29 44 24 70           	movaps	%xmm8, 0x70(%rsp)
141c4f419: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
141c4f41e: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
141c4f423: 4c 8b 01                    	movq	(%rcx), %r8
141c4f426: 48 8b 41 08                 	movq	0x8(%rcx), %rax
141c4f42a: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
141c4f42f: 44 8b 51 14                 	movl	0x14(%rcx), %r10d
141c4f433: 4d 8b b0 f8 00 00 00        	movq	0xf8(%r8), %r14
141c4f43a: 49 8b 80 00 01 00 00        	movq	0x100(%r8), %rax
141c4f441: 4d 8d 3c 86                 	leaq	(%r14,%rax,4), %r15
141c4f445: 4c 8b 1a                    	movq	(%rdx), %r11
141c4f448: 4c 89 44 24 30              	movq	%r8, 0x30(%rsp)
141c4f44d: 41 80 b8 08 01 00 00 00     	cmpb	$0x0, 0x108(%r8)
141c4f455: 44 89 54 24 2c              	movl	%r10d, 0x2c(%rsp)
141c4f45a: 4c 89 5c 24 48              	movq	%r11, 0x48(%rsp)
141c4f45f: 0f 84 4b 01 00 00           	je	0x141c4f5b0 <.text+0x1c4e5b0>
141c4f465: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
141c4f46e: f3 0f 10 35 62 64 cd 00     	movss	0xcd6462(%rip), %xmm6   # 0x1429258d8
141c4f476: 0f 28 3d 73 64 cd 00        	movaps	0xcd6473(%rip), %xmm7   # 0x1429258f0
141c4f47d: f3 44 0f 10 05 7e 64 cd 00  	movss	0xcd647e(%rip), %xmm8   # 0x142925904
141c4f486: 31 ff                       	xorl	%edi, %edi
141c4f488: 45 31 ed                    	xorl	%r13d, %r13d
141c4f48b: 31 db                       	xorl	%ebx, %ebx
141c4f48d: eb 57                       	jmp	0x141c4f4e6 <.text+0x1c4e4e6>
141c4f48f: 90                          	nop
141c4f490: 4b 8d 2c 64                 	leaq	(%r12,%r12,2), %rbp
141c4f494: 48 ff c3                    	incq	%rbx
141c4f497: f3 0f 58 04 af              	addss	(%rdi,%rbp,4), %xmm0
141c4f49c: 0f 57 d2                    	xorps	%xmm2, %xmm2
141c4f49f: f3 0f 5f d0                 	maxss	%xmm0, %xmm2
141c4f4a3: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
141c4f4a7: f3 0f 5d ca                 	minss	%xmm2, %xmm1
141c4f4ab: f3 0f 11 0c af              	movss	%xmm1, (%rdi,%rbp,4)
141c4f4b0: 48 8d 0d 05 50 2c 01        	leaq	0x12c5005(%rip), %rcx   # 0x142f144bc
141c4f4b7: e8 34 6a 04 ff              	callq	0x140c95ef0 <.text+0xc94ef0>
141c4f4bc: 44 8b 54 24 2c              	movl	0x2c(%rsp), %r10d
141c4f4c1: 46 89 14 a6                 	movl	%r10d, (%rsi,%r12,4)
141c4f4c5: f3 0f 11 44 af 04           	movss	%xmm0, 0x4(%rdi,%rbp,4)
141c4f4cb: 4c 8b 5c 24 48              	movq	0x48(%rsp), %r11
141c4f4d0: f3 41 0f 10 43 58           	movss	0x58(%r11), %xmm0
141c4f4d6: 46 89 14 a6                 	movl	%r10d, (%rsi,%r12,4)
141c4f4da: f3 0f 58 44 af 08           	addss	0x8(%rdi,%rbp,4), %xmm0
141c4f4e0: f3 0f 11 44 af 08           	movss	%xmm0, 0x8(%rdi,%rbp,4)
141c4f4e6: 4c 39 eb                    	cmpq	%r13, %rbx
141c4f4e9: 0f 85 9d 00 00 00           	jne	0x141c4f58c <.text+0x1c4e58c>
141c4f4ef: 4d 39 fe                    	cmpq	%r15, %r14
141c4f4f2: 0f 84 65 02 00 00           	je	0x141c4f75d <.text+0x1c4e75d>
141c4f4f8: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
141c4f4fd: 48 8b 80 a8 01 00 00        	movq	0x1a8(%rax), %rax
141c4f504: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141c4f510: 41 8b 0e                    	movl	(%r14), %ecx
141c4f513: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
141c4f517: 4c 8b 6c c8 10              	movq	0x10(%rax,%rcx,8), %r13
141c4f51c: 4d 85 ed                    	testq	%r13, %r13
141c4f51f: 75 0f                       	jne	0x141c4f530 <.text+0x1c4e530>
141c4f521: 49 83 c6 04                 	addq	$0x4, %r14
141c4f525: 4d 39 fe                    	cmpq	%r15, %r14
141c4f528: 75 e6                       	jne	0x141c4f510 <.text+0x1c4e510>
141c4f52a: e9 2e 02 00 00              	jmp	0x141c4f75d <.text+0x1c4e75d>
141c4f52f: 90                          	nop
141c4f530: 48 8d 14 c8                 	leaq	(%rax,%rcx,8), %rdx
141c4f534: 4c 8b 42 18                 	movq	0x18(%rdx), %r8
141c4f538: 48 8b 42 38                 	movq	0x38(%rdx), %rax
141c4f53c: 48 8b 4c 24 30              	movq	0x30(%rsp), %rcx
141c4f541: 4c 8b 89 10 01 00 00        	movq	0x110(%rcx), %r9
141c4f548: 48 8b 89 18 01 00 00        	movq	0x118(%rcx), %rcx
141c4f54f: 4e 8b 0c c8                 	movq	(%rax,%r9,8), %r9
141c4f553: 49 f7 d1                    	notq	%r9
141c4f556: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
141c4f55a: 49 c1 e1 04                 	shlq	$0x4, %r9
141c4f55e: 4b 8b 7c 08 10              	movq	0x10(%r8,%r9), %rdi
141c4f563: 4b 8b 74 08 28              	movq	0x28(%r8,%r9), %rsi
141c4f568: 49 83 c6 04                 	addq	$0x4, %r14
141c4f56c: 48 39 4a 40                 	cmpq	%rcx, 0x40(%rdx)
141c4f570: 76 0f                       	jbe	0x141c4f581 <.text+0x1c4e581>
141c4f572: 48 83 3c c8 00              	cmpq	$0x0, (%rax,%rcx,8)
141c4f577: 0f 95 c0                    	setne	%al
141c4f57a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141c4f57f: eb 09                       	jmp	0x141c4f58a <.text+0x1c4e58a>
141c4f581: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
141c4f58a: 31 db                       	xorl	%ebx, %ebx
141c4f58c: 41 89 dc                    	movl	%ebx, %r12d
141c4f58f: f3 41 0f 10 43 58           	movss	0x58(%r11), %xmm0
141c4f595: f3 0f 5e c6                 	divss	%xmm6, %xmm0
141c4f599: 46 89 14 a6                 	movl	%r10d, (%rsi,%r12,4)
141c4f59d: f6 44 24 20 01              	testb	$0x1, 0x20(%rsp)
141c4f5a2: 0f 84 e8 fe ff ff           	je	0x141c4f490 <.text+0x1c4e490>
141c4f5a8: 0f 57 c7                    	xorps	%xmm7, %xmm0
141c4f5ab: e9 e0 fe ff ff              	jmp	0x141c4f490 <.text+0x1c4e490>
141c4f5b0: b8 08 00 00 00              	movl	$0x8, %eax
141c4f5b5: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
141c4f5ba: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
141c4f5c3: f3 0f 10 35 0d 63 cd 00     	movss	0xcd630d(%rip), %xmm6   # 0x1429258d8
141c4f5cb: 0f 28 3d 1e 63 cd 00        	movaps	0xcd631e(%rip), %xmm7   # 0x1429258f0
141c4f5d2: f3 44 0f 10 05 29 63 cd 00  	movss	0xcd6329(%rip), %xmm8   # 0x142925904
141c4f5db: 31 f6                       	xorl	%esi, %esi
141c4f5dd: 45 31 ed                    	xorl	%r13d, %r13d
141c4f5e0: 31 db                       	xorl	%ebx, %ebx
141c4f5e2: eb 62                       	jmp	0x141c4f646 <.text+0x1c4e646>
141c4f5e4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141c4f5f0: 4b 8d 2c 64                 	leaq	(%r12,%r12,2), %rbp
141c4f5f4: 48 ff c3                    	incq	%rbx
141c4f5f7: f3 0f 58 04 ae              	addss	(%rsi,%rbp,4), %xmm0
141c4f5fc: 0f 57 d2                    	xorps	%xmm2, %xmm2
141c4f5ff: f3 0f 5f d0                 	maxss	%xmm0, %xmm2
141c4f603: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
141c4f607: f3 0f 5d ca                 	minss	%xmm2, %xmm1
141c4f60b: f3 0f 11 0c ae              	movss	%xmm1, (%rsi,%rbp,4)
141c4f610: 48 8d 0d a5 4e 2c 01        	leaq	0x12c4ea5(%rip), %rcx   # 0x142f144bc
141c4f617: e8 d4 68 04 ff              	callq	0x140c95ef0 <.text+0xc94ef0>
141c4f61c: 44 8b 54 24 2c              	movl	0x2c(%rsp), %r10d
141c4f621: 46 89 14 a7                 	movl	%r10d, (%rdi,%r12,4)
141c4f625: f3 0f 11 44 ae 04           	movss	%xmm0, 0x4(%rsi,%rbp,4)
141c4f62b: 4c 8b 5c 24 48              	movq	0x48(%rsp), %r11
141c4f630: f3 41 0f 10 43 58           	movss	0x58(%r11), %xmm0
141c4f636: 46 89 14 a7                 	movl	%r10d, (%rdi,%r12,4)
141c4f63a: f3 0f 58 44 ae 08           	addss	0x8(%rsi,%rbp,4), %xmm0
141c4f640: f3 0f 11 44 ae 08           	movss	%xmm0, 0x8(%rsi,%rbp,4)
141c4f646: 4c 39 eb                    	cmpq	%r13, %rbx
141c4f649: 0f 85 dc 00 00 00           	jne	0x141c4f72b <.text+0x1c4e72b>
141c4f64f: 4d 39 fe                    	cmpq	%r15, %r14
141c4f652: 0f 84 05 01 00 00           	je	0x141c4f75d <.text+0x1c4e75d>
141c4f658: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
141c4f65d: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
141c4f664: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141c4f670: 41 8b 0e                    	movl	(%r14), %ecx
141c4f673: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
141c4f677: 48 c1 e1 05                 	shlq	$0x5, %rcx
141c4f67b: 4c 8b 6c 08 58              	movq	0x58(%rax,%rcx), %r13
141c4f680: 4d 85 ed                    	testq	%r13, %r13
141c4f683: 75 1b                       	jne	0x141c4f6a0 <.text+0x1c4e6a0>
141c4f685: 49 83 c6 04                 	addq	$0x4, %r14
141c4f689: 4d 39 fe                    	cmpq	%r15, %r14
141c4f68c: 75 e2                       	jne	0x141c4f670 <.text+0x1c4e670>
141c4f68e: e9 ca 00 00 00              	jmp	0x141c4f75d <.text+0x1c4e75d>
141c4f693: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141c4f6a0: 48 01 c8                    	addq	%rcx, %rax
141c4f6a3: 48 8b 4c 24 30              	movq	0x30(%rsp), %rcx
141c4f6a8: 48 8b 89 18 01 00 00        	movq	0x118(%rcx), %rcx
141c4f6af: 48 39 88 88 00 00 00        	cmpq	%rcx, 0x88(%rax)
141c4f6b6: 76 16                       	jbe	0x141c4f6ce <.text+0x1c4e6ce>
141c4f6b8: 48 8b 90 80 00 00 00        	movq	0x80(%rax), %rdx
141c4f6bf: 48 83 3c ca 00              	cmpq	$0x0, (%rdx,%rcx,8)
141c4f6c4: 0f 95 c1                    	setne	%cl
141c4f6c7: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
141c4f6cc: eb 09                       	jmp	0x141c4f6d7 <.text+0x1c4e6d7>
141c4f6ce: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
141c4f6d7: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
141c4f6dd: 48 8b 54 24 38              	movq	0x38(%rsp), %rdx
141c4f6e2: 48 8b 92 a8 01 00 00        	movq	0x1a8(%rdx), %rdx
141c4f6e9: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
141c4f6ed: 4c 8b 44 ca 18              	movq	0x18(%rdx,%rcx,8), %r8
141c4f6f2: 48 8b 4c ca 38              	movq	0x38(%rdx,%rcx,8), %rcx
141c4f6f7: 48 8b 54 24 30              	movq	0x30(%rsp), %rdx
141c4f6fc: 48 8b 92 10 01 00 00        	movq	0x110(%rdx), %rdx
141c4f703: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
141c4f707: 48 f7 d1                    	notq	%rcx
141c4f70a: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
141c4f70e: 48 c1 e1 04                 	shlq	$0x4, %rcx
141c4f712: 49 8b 74 08 10              	movq	0x10(%r8,%rcx), %rsi
141c4f717: 49 8b 7c 08 28              	movq	0x28(%r8,%rcx), %rdi
141c4f71c: 49 83 c6 04                 	addq	$0x4, %r14
141c4f720: 48 8b 40 50                 	movq	0x50(%rax), %rax
141c4f724: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
141c4f729: 31 db                       	xorl	%ebx, %ebx
141c4f72b: 48 89 d8                    	movq	%rbx, %rax
141c4f72e: 48 c1 e0 04                 	shlq	$0x4, %rax
141c4f732: 48 8b 4c 24 40              	movq	0x40(%rsp), %rcx
141c4f737: 44 8b 64 01 08              	movl	0x8(%rcx,%rax), %r12d
141c4f73c: f3 41 0f 10 43 58           	movss	0x58(%r11), %xmm0
141c4f742: f3 0f 5e c6                 	divss	%xmm6, %xmm0
141c4f746: 46 89 14 a7                 	movl	%r10d, (%rdi,%r12,4)
141c4f74a: f6 44 24 20 01              	testb	$0x1, 0x20(%rsp)
141c4f74f: 0f 84 9b fe ff ff           	je	0x141c4f5f0 <.text+0x1c4e5f0>
141c4f755: 0f 57 c7                    	xorps	%xmm7, %xmm0
141c4f758: e9 93 fe ff ff              	jmp	0x141c4f5f0 <.text+0x1c4e5f0>
141c4f75d: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
141c4f762: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
141c4f767: 44 0f 28 44 24 70           	movaps	0x70(%rsp), %xmm8
141c4f76d: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
141c4f774: 5b                          	popq	%rbx
141c4f775: 5d                          	popq	%rbp
141c4f776: 5f                          	popq	%rdi
141c4f777: 5e                          	popq	%rsi
141c4f778: 41 5c                       	popq	%r12
141c4f77a: 41 5d                       	popq	%r13
141c4f77c: 41 5e                       	popq	%r14
141c4f77e: 41 5f                       	popq	%r15
141c4f780: c3                          	retq
141c4f781: cc                          	int3
141c4f782: cc                          	int3
141c4f783: cc                          	int3
141c4f784: cc                          	int3
141c4f785: cc                          	int3
141c4f786: cc                          	int3
141c4f787: cc                          	int3
141c4f788: cc                          	int3
141c4f789: cc                          	int3
141c4f78a: cc                          	int3
141c4f78b: cc                          	int3
141c4f78c: cc                          	int3
141c4f78d: cc                          	int3
141c4f78e: cc                          	int3
141c4f78f: cc                          	int3


D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408b3510: 41 57                       	pushq	%r15
1408b3512: 41 56                       	pushq	%r14
1408b3514: 41 55                       	pushq	%r13
1408b3516: 41 54                       	pushq	%r12
1408b3518: 56                          	pushq	%rsi
1408b3519: 57                          	pushq	%rdi
1408b351a: 55                          	pushq	%rbp
1408b351b: 53                          	pushq	%rbx
1408b351c: 48 83 ec 58                 	subq	$0x58, %rsp
1408b3520: 48 8b 01                    	movq	(%rcx), %rax
1408b3523: 48 8b 59 08                 	movq	0x8(%rcx), %rbx
1408b3527: 4c 8b b0 f8 00 00 00        	movq	0xf8(%rax), %r14
1408b352e: 48 8b 88 00 01 00 00        	movq	0x100(%rax), %rcx
1408b3535: 4d 8d 3c 8e                 	leaq	(%r14,%rcx,4), %r15
1408b3539: 48 8b 0a                    	movq	(%rdx), %rcx
1408b353c: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
1408b3541: 48 8b 4a 08                 	movq	0x8(%rdx), %rcx
1408b3545: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1408b354a: 48 8b 6a 18                 	movq	0x18(%rdx), %rbp
1408b354e: 48 8b 72 10                 	movq	0x10(%rdx), %rsi
1408b3552: 80 b8 08 01 00 00 00        	cmpb	$0x0, 0x108(%rax)
1408b3559: 0f 84 90 00 00 00           	je	0x1408b35ef <.text+0x8b25ef>
1408b355f: 41 bc 08 00 00 00           	movl	$0x8, %r12d
1408b3565: 31 ff                       	xorl	%edi, %edi
1408b3567: 45 31 ed                    	xorl	%r13d, %r13d
1408b356a: 49 39 fd                    	cmpq	%rdi, %r13
1408b356d: 75 31                       	jne	0x1408b35a0 <.text+0x8b25a0>
1408b356f: 90                          	nop
1408b3570: 4d 39 fe                    	cmpq	%r15, %r14
1408b3573: 0f 84 08 01 00 00           	je	0x1408b3681 <.text+0x8b2681>
1408b3579: 41 8b 0e                    	movl	(%r14), %ecx
1408b357c: 49 83 c6 04                 	addq	$0x4, %r14
1408b3580: 48 8b 83 a8 01 00 00        	movq	0x1a8(%rbx), %rax
1408b3587: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1408b358b: 48 8b 7c c8 10              	movq	0x10(%rax,%rcx,8), %rdi
1408b3590: 48 85 ff                    	testq	%rdi, %rdi
1408b3593: 74 db                       	je	0x1408b3570 <.text+0x8b2570>
1408b3595: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
1408b3599: 4c 8b 60 08                 	movq	0x8(%rax), %r12
1408b359d: 45 31 ed                    	xorl	%r13d, %r13d
1408b35a0: 4b 8b 04 ec                 	movq	(%r12,%r13,8), %rax
1408b35a4: 48 89 c1                    	movq	%rax, %rcx
1408b35a7: 48 c1 e9 20                 	shrq	$0x20, %rcx
1408b35ab: 0f 84 d0 00 00 00           	je	0x1408b3681 <.text+0x8b2681>
1408b35b1: 49 ff c5                    	incq	%r13
1408b35b4: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
1408b35b9: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
1408b35be: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1408b35c3: 48 8b 44 24 20              	movq	0x20(%rsp), %rax
1408b35c8: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1408b35cd: 48 89 74 24 40              	movq	%rsi, 0x40(%rsp)
1408b35d2: 48 89 6c 24 48              	movq	%rbp, 0x48(%rsp)
1408b35d7: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1408b35dc: 48 8d 15 8d bb 1e 02        	leaq	0x21ebb8d(%rip), %rdx   # 0x142a9f170
1408b35e3: e8 88 11 46 00              	callq	0x140d14770 <.text+0xd13770>
1408b35e8: 49 39 fd                    	cmpq	%rdi, %r13
1408b35eb: 74 83                       	je	0x1408b3570 <.text+0x8b2570>
1408b35ed: eb b1                       	jmp	0x1408b35a0 <.text+0x8b25a0>
1408b35ef: 41 bc 08 00 00 00           	movl	$0x8, %r12d
1408b35f5: 31 ff                       	xorl	%edi, %edi
1408b35f7: 45 31 ed                    	xorl	%r13d, %r13d
1408b35fa: 49 39 fd                    	cmpq	%rdi, %r13
1408b35fd: 75 30                       	jne	0x1408b362f <.text+0x8b262f>
1408b35ff: 90                          	nop
1408b3600: 4d 39 fe                    	cmpq	%r15, %r14
1408b3603: 74 7c                       	je	0x1408b3681 <.text+0x8b2681>
1408b3605: 41 8b 0e                    	movl	(%r14), %ecx
1408b3608: 49 83 c6 04                 	addq	$0x4, %r14
1408b360c: 48 8b 83 00 01 00 00        	movq	0x100(%rbx), %rax
1408b3613: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1408b3617: 48 c1 e1 05                 	shlq	$0x5, %rcx
1408b361b: 48 8b 7c 08 58              	movq	0x58(%rax,%rcx), %rdi
1408b3620: 48 85 ff                    	testq	%rdi, %rdi
1408b3623: 74 db                       	je	0x1408b3600 <.text+0x8b2600>
1408b3625: 48 01 c8                    	addq	%rcx, %rax
1408b3628: 4c 8b 60 50                 	movq	0x50(%rax), %r12
1408b362c: 45 31 ed                    	xorl	%r13d, %r13d
1408b362f: 4c 89 e8                    	movq	%r13, %rax
1408b3632: 48 c1 e0 04                 	shlq	$0x4, %rax
1408b3636: 49 8b 04 04                 	movq	(%r12,%rax), %rax
1408b363a: 48 89 c1                    	movq	%rax, %rcx
1408b363d: 48 c1 e9 20                 	shrq	$0x20, %rcx
1408b3641: 74 3e                       	je	0x1408b3681 <.text+0x8b2681>
1408b3643: 49 ff c5                    	incq	%r13
1408b3646: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
1408b364b: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
1408b3650: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1408b3655: 48 8b 44 24 20              	movq	0x20(%rsp), %rax
1408b365a: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1408b365f: 48 89 74 24 40              	movq	%rsi, 0x40(%rsp)
1408b3664: 48 89 6c 24 48              	movq	%rbp, 0x48(%rsp)
1408b3669: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1408b366e: 48 8d 15 fb ba 1e 02        	leaq	0x21ebafb(%rip), %rdx   # 0x142a9f170
1408b3675: e8 f6 10 46 00              	callq	0x140d14770 <.text+0xd13770>
1408b367a: 49 39 fd                    	cmpq	%rdi, %r13
1408b367d: 74 81                       	je	0x1408b3600 <.text+0x8b2600>
1408b367f: eb ae                       	jmp	0x1408b362f <.text+0x8b262f>
1408b3681: 48 83 c4 58                 	addq	$0x58, %rsp
1408b3685: 5b                          	popq	%rbx
1408b3686: 5d                          	popq	%rbp
1408b3687: 5f                          	popq	%rdi
1408b3688: 5e                          	popq	%rsi
1408b3689: 41 5c                       	popq	%r12
1408b368b: 41 5d                       	popq	%r13
1408b368d: 41 5e                       	popq	%r14
1408b368f: 41 5f                       	popq	%r15
1408b3691: c3                          	retq

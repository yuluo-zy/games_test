
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141410450: 55                          	pushq	%rbp
141410451: 41 56                       	pushq	%r14
141410453: 56                          	pushq	%rsi
141410454: 57                          	pushq	%rdi
141410455: 53                          	pushq	%rbx
141410456: 48 83 ec 50                 	subq	$0x50, %rsp
14141045a: 48 8d 6c 24 50              	leaq	0x50(%rsp), %rbp
14141045f: 48 c7 45 f8 fe ff ff ff     	movq	$-0x2, -0x8(%rbp)
141410467: 48 89 d7                    	movq	%rdx, %rdi
14141046a: 48 89 ce                    	movq	%rcx, %rsi
14141046d: 48 8d 5a 18                 	leaq	0x18(%rdx), %rbx
141410471: f6 42 60 01                 	testb	$0x1, 0x60(%rdx)
141410475: 74 10                       	je	0x141410487 <.text+0x140f487>
141410477: 0f b6 47 61                 	movzbl	0x61(%rdi), %eax
14141047b: 0f b6 c0                    	movzbl	%al, %eax
14141047e: 3c 20                       	cmpb	$0x20, %al
141410480: 76 76                       	jbe	0x1414104f8 <.text+0x140f4f8>
141410482: e9 8a 00 00 00              	jmp	0x141410511 <.text+0x140f511>
141410487: 48 8b 4f 28                 	movq	0x28(%rdi), %rcx
14141048b: 48 39 4f 30                 	cmpq	%rcx, 0x30(%rdi)
14141048f: 75 31                       	jne	0x1414104c2 <.text+0x140f4c2>
141410491: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141410495: 48 89 da                    	movq	%rbx, %rdx
141410498: e8 a3 07 69 ff              	callq	0x140aa0c40 <.text+0xa9fc40>
14141049d: 0f b6 45 d0                 	movzbl	-0x30(%rbp), %eax
1414104a1: 3c 02                       	cmpb	$0x2, %al
1414104a3: 0f 84 68 01 00 00           	je	0x141410611 <.text+0x140f611>
1414104a9: a8 01                       	testb	$0x1, %al
1414104ab: 0f 85 7b 01 00 00           	jne	0x14141062c <.text+0x140f62c>
1414104b1: 0f b6 45 d1                 	movzbl	-0x2f(%rbp), %eax
1414104b5: 48 8b 4f 50                 	movq	0x50(%rdi), %rcx
1414104b9: 48 ff c1                    	incq	%rcx
1414104bc: 3c 0a                       	cmpb	$0xa, %al
1414104be: 74 1c                       	je	0x1414104dc <.text+0x140f4dc>
1414104c0: eb 24                       	jmp	0x1414104e6 <.text+0x140f4e6>
1414104c2: 48 8b 47 18                 	movq	0x18(%rdi), %rax
1414104c6: 0f b6 04 08                 	movzbl	(%rax,%rcx), %eax
1414104ca: 48 ff c1                    	incq	%rcx
1414104cd: 48 89 4f 28                 	movq	%rcx, 0x28(%rdi)
1414104d1: 48 8b 4f 50                 	movq	0x50(%rdi), %rcx
1414104d5: 48 ff c1                    	incq	%rcx
1414104d8: 3c 0a                       	cmpb	$0xa, %al
1414104da: 75 0a                       	jne	0x1414104e6 <.text+0x140f4e6>
1414104dc: 48 01 4f 58                 	addq	%rcx, 0x58(%rdi)
1414104e0: 48 ff 47 48                 	incq	0x48(%rdi)
1414104e4: 31 c9                       	xorl	%ecx, %ecx
1414104e6: 48 89 4f 50                 	movq	%rcx, 0x50(%rdi)
1414104ea: c6 47 60 01                 	movb	$0x1, 0x60(%rdi)
1414104ee: 88 47 61                    	movb	%al, 0x61(%rdi)
1414104f1: 0f b6 c0                    	movzbl	%al, %eax
1414104f4: 3c 20                       	cmpb	$0x20, %al
1414104f6: 77 19                       	ja	0x141410511 <.text+0x140f511>
1414104f8: 48 b9 00 26 00 00 01 00 00 00       	movabsq	$0x100002600, %rcx # imm = 0x100002600
141410502: 48 0f a3 c1                 	btq	%rax, %rcx
141410506: 73 09                       	jae	0x141410511 <.text+0x140f511>
141410508: c6 47 60 00                 	movb	$0x0, 0x60(%rdi)
14141050c: e9 76 ff ff ff              	jmp	0x141410487 <.text+0x140f487>
141410511: 83 f8 5b                    	cmpl	$0x5b, %eax
141410514: 0f 85 1d 01 00 00           	jne	0x141410637 <.text+0x140f637>
14141051a: fe 4f 68                    	decb	0x68(%rdi)
14141051d: 0f 84 2c 01 00 00           	je	0x14141064f <.text+0x140f64f>
141410523: c6 47 60 00                 	movb	$0x0, 0x60(%rdi)
141410527: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
14141052b: 48 89 fa                    	movq	%rdi, %rdx
14141052e: 41 b0 01                    	movb	$0x1, %r8b
141410531: e8 9a 6f 55 ff              	callq	0x1409674d0 <.text+0x9664d0>
141410536: fe 47 68                    	incb	0x68(%rdi)
141410539: 48 89 f9                    	movq	%rdi, %rcx
14141053c: e8 bf 45 ff ff              	callq	0x141404b00 <.text+0x1403b00>
141410541: 49 89 c6                    	movq	%rax, %r14
141410544: 48 8b 55 d0                 	movq	-0x30(%rbp), %rdx
141410548: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
14141054c: 48 89 d0                    	movq	%rdx, %rax
14141054f: 48 f7 d8                    	negq	%rax
141410552: 70 22                       	jo	0x141410576 <.text+0x140f576>
141410554: 4d 85 f6                    	testq	%r14, %r14
141410557: 75 59                       	jne	0x1414105b2 <.text+0x140f5b2>
141410559: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
14141055d: 48 89 16                    	movq	%rdx, (%rsi)
141410560: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
141410564: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
141410568: 48 89 f0                    	movq	%rsi, %rax
14141056b: 48 83 c4 50                 	addq	$0x50, %rsp
14141056f: 5b                          	popq	%rbx
141410570: 5f                          	popq	%rdi
141410571: 5e                          	popq	%rsi
141410572: 41 5e                       	popq	%r14
141410574: 5d                          	popq	%rbp
141410575: c3                          	retq
141410576: 4d 85 f6                    	testq	%r14, %r14
141410579: 74 78                       	je	0x1414105f3 <.text+0x140f5f3>
14141057b: 48 89 cb                    	movq	%rcx, %rbx
14141057e: 49 8b 06                    	movq	(%r14), %rax
141410581: 48 83 f8 01                 	cmpq	$0x1, %rax
141410585: 4c 89 75 e8                 	movq	%r14, -0x18(%rbp)
141410589: 74 45                       	je	0x1414105d0 <.text+0x140f5d0>
14141058b: 48 85 c0                    	testq	%rax, %rax
14141058e: 75 4c                       	jne	0x1414105dc <.text+0x140f5dc>
141410590: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
141410594: 48 8b 50 10                 	movq	0x10(%rax), %rdx
141410598: 48 85 d2                    	testq	%rdx, %rdx
14141059b: 74 3f                       	je	0x1414105dc <.text+0x140f5dc>
14141059d: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
1414105a1: 48 8b 48 08                 	movq	0x8(%rax), %rcx
1414105a5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414105ab: e8 70 36 20 ff              	callq	0x140613c20 <.text+0x612c20>
1414105b0: eb 2a                       	jmp	0x1414105dc <.text+0x140f5dc>
1414105b2: 48 85 d2                    	testq	%rdx, %rdx
1414105b5: 75 05                       	jne	0x1414105bc <.text+0x140f5bc>
1414105b7: 4c 89 f1                    	movq	%r14, %rcx
1414105ba: eb 37                       	jmp	0x1414105f3 <.text+0x140f5f3>
1414105bc: 48 c1 e2 04                 	shlq	$0x4, %rdx
1414105c0: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1414105c6: e8 55 36 20 ff              	callq	0x140613c20 <.text+0x612c20>
1414105cb: 4c 89 f1                    	movq	%r14, %rcx
1414105ce: eb 23                       	jmp	0x1414105f3 <.text+0x140f5f3>
1414105d0: 4c 89 f1                    	movq	%r14, %rcx
1414105d3: 48 83 c1 08                 	addq	$0x8, %rcx
1414105d7: e8 24 74 1d ff              	callq	0x1405e7a00 <.text+0x5e6a00>
1414105dc: ba 28 00 00 00              	movl	$0x28, %edx
1414105e1: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1414105e7: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
1414105eb: e8 30 36 20 ff              	callq	0x140613c20 <.text+0x612c20>
1414105f0: 48 89 d9                    	movq	%rbx, %rcx
1414105f3: 48 89 fa                    	movq	%rdi, %rdx
1414105f6: e8 a5 94 4e 01              	callq	0x1428f9aa0 <.text+0x28f8aa0>
1414105fb: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1414105ff: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141410609: 48 89 06                    	movq	%rax, (%rsi)
14141060c: e9 57 ff ff ff              	jmp	0x141410568 <.text+0x140f568>
141410611: 48 8b 57 48                 	movq	0x48(%rdi), %rdx
141410615: 4c 8b 47 50                 	movq	0x50(%rdi), %r8
141410619: 48 c7 45 d0 05 00 00 00     	movq	$0x5, -0x30(%rbp)
141410621: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141410625: e8 d6 91 4b 01              	callq	0x1428c9800 <.text+0x28c8800>
14141062a: eb cf                       	jmp	0x1414105fb <.text+0x140f5fb>
14141062c: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
141410630: e8 6b 92 4b 01              	callq	0x1428c98a0 <.text+0x28c88a0>
141410635: eb c4                       	jmp	0x1414105fb <.text+0x140f5fb>
141410637: 4c 8d 05 52 b9 84 01        	leaq	0x184b952(%rip), %r8    # 0x142c5bf90
14141063e: 48 8d 55 f7                 	leaq	-0x9(%rbp), %rdx
141410642: 48 89 f9                    	movq	%rdi, %rcx
141410645: e8 b6 88 4e 01              	callq	0x1428f8f00 <.text+0x28f7f00>
14141064a: 48 89 c1                    	movq	%rax, %rcx
14141064d: eb a4                       	jmp	0x1414105f3 <.text+0x140f5f3>
14141064f: 48 8b 57 48                 	movq	0x48(%rdi), %rdx
141410653: 4c 8b 47 50                 	movq	0x50(%rdi), %r8
141410657: 48 c7 45 d0 15 00 00 00     	movq	$0x15, -0x30(%rbp)
14141065f: eb c0                       	jmp	0x141410621 <.text+0x140f621>

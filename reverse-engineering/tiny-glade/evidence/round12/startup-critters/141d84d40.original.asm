
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141d84d40: 55                          	pushq	%rbp
141d84d41: 41 57                       	pushq	%r15
141d84d43: 41 56                       	pushq	%r14
141d84d45: 41 55                       	pushq	%r13
141d84d47: 41 54                       	pushq	%r12
141d84d49: 56                          	pushq	%rsi
141d84d4a: 57                          	pushq	%rdi
141d84d4b: 53                          	pushq	%rbx
141d84d4c: 48 81 ec 88 02 00 00        	subq	$0x288, %rsp            # imm = 0x288
141d84d53: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141d84d5b: 44 0f 29 bd f0 01 00 00     	movaps	%xmm15, 0x1f0(%rbp)
141d84d63: 44 0f 29 b5 e0 01 00 00     	movaps	%xmm14, 0x1e0(%rbp)
141d84d6b: 44 0f 29 ad d0 01 00 00     	movaps	%xmm13, 0x1d0(%rbp)
141d84d73: 44 0f 29 a5 c0 01 00 00     	movaps	%xmm12, 0x1c0(%rbp)
141d84d7b: 44 0f 29 9d b0 01 00 00     	movaps	%xmm11, 0x1b0(%rbp)
141d84d83: 44 0f 29 95 a0 01 00 00     	movaps	%xmm10, 0x1a0(%rbp)
141d84d8b: 44 0f 29 8d 90 01 00 00     	movaps	%xmm9, 0x190(%rbp)
141d84d93: 44 0f 29 85 80 01 00 00     	movaps	%xmm8, 0x180(%rbp)
141d84d9b: 0f 29 bd 70 01 00 00        	movaps	%xmm7, 0x170(%rbp)
141d84da2: 0f 29 b5 60 01 00 00        	movaps	%xmm6, 0x160(%rbp)
141d84da9: 48 c7 85 58 01 00 00 fe ff ff ff    	movq	$-0x2, 0x158(%rbp)
141d84db4: 4d 89 ce                    	movq	%r9, %r14
141d84db7: 4d 89 c7                    	movq	%r8, %r15
141d84dba: 48 89 d6                    	movq	%rdx, %rsi
141d84dbd: 49 89 cc                    	movq	%rcx, %r12
141d84dc0: e8 3b 1c e9 fe              	callq	0x140c16a00 <.text+0xc15a00>
141d84dc5: 84 c0                       	testb	%al, %al
141d84dc7: 74 63                       	je	0x141d84e2c <.text+0x1d83e2c>
141d84dc9: 4c 89 b5 18 01 00 00        	movq	%r14, 0x118(%rbp)
141d84dd0: bb 33 00 00 00              	movl	$0x33, %ebx
141d84dd5: b9 33 00 00 00              	movl	$0x33, %ecx
141d84dda: e8 31 fb ff ff              	callq	0x141d84910 <.text+0x1d83910>
141d84ddf: 4c 8d 35 8a 30 1e 01        	leaq	0x11e308a(%rip), %r14   # 0x142f67e70
141d84de6: 48 83 f8 01                 	cmpq	$0x1, %rax
141d84dea: 0f 85 a8 00 00 00           	jne	0x141d84e98 <.text+0x1d83e98>
141d84df0: 49 89 d1                    	movq	%rdx, %r9
141d84df3: 48 85 d2                    	testq	%rdx, %rdx
141d84df6: 74 3e                       	je	0x141d84e36 <.text+0x1d83e36>
141d84df8: 49 83 f9 33                 	cmpq	$0x33, %r9
141d84dfc: 73 36                       	jae	0x141d84e34 <.text+0x1d83e34>
141d84dfe: 48 8d 05 6b 30 1e 01        	leaq	0x11e306b(%rip), %rax   # 0x142f67e70
141d84e05: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
141d84e0a: 7f 2a                       	jg	0x141d84e36 <.text+0x1d83e36>
141d84e0c: 48 8d 05 15 31 1e 01        	leaq	0x11e3115(%rip), %rax   # 0x142f67f28
141d84e13: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141d84e18: 48 8d 0d 51 30 1e 01        	leaq	0x11e3051(%rip), %rcx   # 0x142f67e70
141d84e1f: ba 33 00 00 00              	movl	$0x33, %edx
141d84e24: 45 31 c0                    	xorl	%r8d, %r8d
141d84e27: e8 14 4b b5 00              	callq	0x1428d9940 <.text+0x28d8940>
141d84e2c: 45 31 ed                    	xorl	%r13d, %r13d
141d84e2f: e9 d0 01 00 00              	jmp	0x141d85004 <.text+0x1d84004>
141d84e34: 75 d6                       	jne	0x141d84e0c <.text+0x1d83e0c>
141d84e36: 4c 89 c9                    	movq	%r9, %rcx
141d84e39: e8 d2 fa ff ff              	callq	0x141d84910 <.text+0x1d83910>
141d84e3e: 48 83 f8 01                 	cmpq	$0x1, %rax
141d84e42: 75 54                       	jne	0x141d84e98 <.text+0x1d83e98>
141d84e44: 49 89 d0                    	movq	%rdx, %r8
141d84e47: 49 83 c0 02                 	addq	$0x2, %r8
141d84e4b: 74 39                       	je	0x141d84e86 <.text+0x1d83e86>
141d84e4d: 49 83 f8 33                 	cmpq	$0x33, %r8
141d84e51: 73 31                       	jae	0x141d84e84 <.text+0x1d83e84>
141d84e53: 48 8d 05 16 30 1e 01        	leaq	0x11e3016(%rip), %rax   # 0x142f67e70
141d84e5a: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
141d84e5f: 7f 25                       	jg	0x141d84e86 <.text+0x1d83e86>
141d84e61: 48 8d 05 d8 30 1e 01        	leaq	0x11e30d8(%rip), %rax   # 0x142f67f40
141d84e68: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141d84e6d: 48 8d 0d fc 2f 1e 01        	leaq	0x11e2ffc(%rip), %rcx   # 0x142f67e70
141d84e74: ba 33 00 00 00              	movl	$0x33, %edx
141d84e79: 41 b9 33 00 00 00           	movl	$0x33, %r9d
141d84e7f: e8 bc 4a b5 00              	callq	0x1428d9940 <.text+0x28d8940>
141d84e84: 75 db                       	jne	0x141d84e61 <.text+0x1d83e61>
141d84e86: bb 31 00 00 00              	movl	$0x31, %ebx
141d84e8b: 48 29 d3                    	subq	%rdx, %rbx
141d84e8e: 4c 8d 35 db 2f 1e 01        	leaq	0x11e2fdb(%rip), %r14   # 0x142f67e70
141d84e95: 4d 01 c6                    	addq	%r8, %r14
141d84e98: 4c 8d 05 f9 30 1e 01        	leaq	0x11e30f9(%rip), %r8    # 0x142f67f98
141d84e9f: 4c 8d 2d ca 30 1e 01        	leaq	0x11e30ca(%rip), %r13   # 0x142f67f70
141d84ea6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141d84eb0: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
141d84eb5: 85 c0                       	testl	%eax, %eax
141d84eb7: 78 17                       	js	0x141d84ed0 <.text+0x1d83ed0>
141d84eb9: 49 ff c8                    	decq	%r8
141d84ebc: 83 f8 5c                    	cmpl	$0x5c, %eax
141d84ebf: 75 63                       	jne	0x141d84f24 <.text+0x1d83f24>
141d84ec1: eb 76                       	jmp	0x141d84f39 <.text+0x1d83f39>
141d84ec3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141d84ed0: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
141d84ed5: 80 f9 c0                    	cmpb	$-0x40, %cl
141d84ed8: 7d 23                       	jge	0x141d84efd <.text+0x1d83efd>
141d84eda: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
141d84edf: 80 fa c0                    	cmpb	$-0x40, %dl
141d84ee2: 7d 22                       	jge	0x141d84f06 <.text+0x1d83f06>
141d84ee4: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
141d84ee9: 49 83 c0 fc                 	addq	$-0x4, %r8
141d84eed: 41 83 e1 07                 	andl	$0x7, %r9d
141d84ef1: 41 c1 e1 06                 	shll	$0x6, %r9d
141d84ef5: 83 e2 3f                    	andl	$0x3f, %edx
141d84ef8: 44 09 ca                    	orl	%r9d, %edx
141d84efb: eb 10                       	jmp	0x141d84f0d <.text+0x1d83f0d>
141d84efd: 49 83 c0 fe                 	addq	$-0x2, %r8
141d84f01: 83 e1 1f                    	andl	$0x1f, %ecx
141d84f04: eb 0f                       	jmp	0x141d84f15 <.text+0x1d83f15>
141d84f06: 49 83 c0 fd                 	addq	$-0x3, %r8
141d84f0a: 83 e2 0f                    	andl	$0xf, %edx
141d84f0d: c1 e2 06                    	shll	$0x6, %edx
141d84f10: 83 e1 3f                    	andl	$0x3f, %ecx
141d84f13: 09 d1                       	orl	%edx, %ecx
141d84f15: c1 e1 06                    	shll	$0x6, %ecx
141d84f18: 24 3f                       	andb	$0x3f, %al
141d84f1a: 0f b6 c0                    	movzbl	%al, %eax
141d84f1d: 09 c8                       	orl	%ecx, %eax
141d84f1f: 83 f8 5c                    	cmpl	$0x5c, %eax
141d84f22: 74 15                       	je	0x141d84f39 <.text+0x1d83f39>
141d84f24: 83 f8 2f                    	cmpl	$0x2f, %eax
141d84f27: 74 10                       	je	0x141d84f39 <.text+0x1d83f39>
141d84f29: 4d 39 e8                    	cmpq	%r13, %r8
141d84f2c: 75 82                       	jne	0x141d84eb0 <.text+0x1d83eb0>
141d84f2e: 4c 89 ff                    	movq	%r15, %rdi
141d84f31: 41 bf 28 00 00 00           	movl	$0x28, %r15d
141d84f37: eb 50                       	jmp	0x141d84f89 <.text+0x1d83f89>
141d84f39: 4c 8d 2d 30 30 1e 01        	leaq	0x11e3030(%rip), %r13   # 0x142f67f70
141d84f40: 4d 29 e8                    	subq	%r13, %r8
141d84f43: 49 ff c0                    	incq	%r8
141d84f46: 74 32                       	je	0x141d84f7a <.text+0x1d83f7a>
141d84f48: 49 83 f8 28                 	cmpq	$0x28, %r8
141d84f4c: 73 2a                       	jae	0x141d84f78 <.text+0x1d83f78>
141d84f4e: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
141d84f53: 7f 25                       	jg	0x141d84f7a <.text+0x1d83f7a>
141d84f55: 48 8d 05 ac 2f 1e 01        	leaq	0x11e2fac(%rip), %rax   # 0x142f67f08
141d84f5c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141d84f61: 48 8d 0d 08 30 1e 01        	leaq	0x11e3008(%rip), %rcx   # 0x142f67f70
141d84f68: ba 28 00 00 00              	movl	$0x28, %edx
141d84f6d: 41 b9 28 00 00 00           	movl	$0x28, %r9d
141d84f73: e8 c8 49 b5 00              	callq	0x1428d9940 <.text+0x28d8940>
141d84f78: 75 db                       	jne	0x141d84f55 <.text+0x1d83f55>
141d84f7a: 4c 89 ff                    	movq	%r15, %rdi
141d84f7d: 41 bf 28 00 00 00           	movl	$0x28, %r15d
141d84f83: 4d 29 c7                    	subq	%r8, %r15
141d84f86: 4d 01 c5                    	addq	%r8, %r13
141d84f89: e8 a2 31 e9 fe              	callq	0x140c18130 <.text+0xc17130>
141d84f8e: 48 89 c1                    	movq	%rax, %rcx
141d84f91: 48 8b 00                    	movq	(%rax), %rax
141d84f94: 48 83 f8 01                 	cmpq	$0x1, %rax
141d84f98: 0f 85 a8 0c 00 00           	jne	0x141d85c46 <.text+0x1d84c46>
141d84f9e: 48 83 c1 08                 	addq	$0x8, %rcx
141d84fa2: 48 83 39 00                 	cmpq	$0x0, (%rcx)
141d84fa6: 0f 85 be 0c 00 00           	jne	0x141d85c6a <.text+0x1d84c6a>
141d84fac: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
141d84fb3: 48 89 8d 48 01 00 00        	movq	%rcx, 0x148(%rbp)
141d84fba: 48 83 c1 08                 	addq	$0x8, %rcx
141d84fbe: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
141d84fc3: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
141d84fcc: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
141d84fd5: 4c 89 f2                    	movq	%r14, %rdx
141d84fd8: 49 89 d8                    	movq	%rbx, %r8
141d84fdb: 4d 89 e9                    	movq	%r13, %r9
141d84fde: e8 5d 1d e9 fe              	callq	0x140c16d40 <.text+0xc15d40>
141d84fe3: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
141d84fea: 48 ff 01                    	incq	(%rcx)
141d84fed: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
141d84ff4: 41 bd 01 00 00 00           	movl	$0x1, %r13d
141d84ffa: 49 89 ff                    	movq	%rdi, %r15
141d84ffd: 4c 8b b5 18 01 00 00        	movq	0x118(%rbp), %r14
141d85004: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
141d8500b: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
141d85012: 4c 89 ad b8 00 00 00        	movq	%r13, 0xb8(%rbp)
141d85019: 8b 56 1c                    	movl	0x1c(%rsi), %edx
141d8501c: 4c 8b 46 10                 	movq	0x10(%rsi), %r8
141d85020: 41 89 10                    	movl	%edx, (%r8)
141d85023: 48 8b 16                    	movq	(%rsi), %rdx
141d85026: 48 89 95 10 01 00 00        	movq	%rdx, 0x110(%rbp)
141d8502d: 48 8b 00                    	movq	(%rax), %rax
141d85030: f3 44 0f 10 48 28           	movss	0x28(%rax), %xmm9
141d85036: 48 8b 09                    	movq	(%rcx), %rcx
141d85039: 4c 89 ad 28 01 00 00        	movq	%r13, 0x128(%rbp)
141d85040: 48 8d 15 a1 52 2c 01        	leaq	0x12c52a1(%rip), %rdx   # 0x14304a2e8
141d85047: e8 34 97 58 00              	callq	0x14230e780 <.text+0x230d780>
141d8504c: 48 89 c7                    	movq	%rax, %rdi
141d8504f: 48 85 c0                    	testq	%rax, %rax
141d85052: 0f 84 50 0c 00 00           	je	0x141d85ca8 <.text+0x1d84ca8>
141d85058: 48 8d 05 71 53 2c 01        	leaq	0x12c5371(%rip), %rax   # 0x14304a3d0
141d8505f: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
141d85066: 48 c7 85 e0 00 00 00 3f 00 00 00    	movq	$0x3f, 0xe0(%rbp)
141d85071: 48 89 fb                    	movq	%rdi, %rbx
141d85074: 48 83 c3 10                 	addq	$0x10, %rbx
141d85078: 48 83 7f 18 3f              	cmpq	$0x3f, 0x18(%rdi)
141d8507d: 0f 85 f3 0b 00 00           	jne	0x141d85c76 <.text+0x1d84c76>
141d85083: 48 8b 13                    	movq	(%rbx), %rdx
141d85086: 48 8d 0d 43 53 2c 01        	leaq	0x12c5343(%rip), %rcx   # 0x14304a3d0
141d8508d: 41 b8 3f 00 00 00           	movl	$0x3f, %r8d
141d85093: e8 58 eb b9 00              	callq	0x142923bf0 <.text+0x2922bf0>
141d85098: 85 c0                       	testl	%eax, %eax
141d8509a: 0f 85 d6 0b 00 00           	jne	0x141d85c76 <.text+0x1d84c76>
141d850a0: 48 8b 0f                    	movq	(%rdi), %rcx
141d850a3: 48 8b 47 08                 	movq	0x8(%rdi), %rax
141d850a7: 48 8b 40 18                 	movq	0x18(%rax), %rax
141d850ab: 4c 89 ad 28 01 00 00        	movq	%r13, 0x128(%rbp)
141d850b2: ff d0                       	callq	*%rax
141d850b4: 48 85 c0                    	testq	%rax, %rax
141d850b7: 0f 84 eb 0b 00 00           	je	0x141d85ca8 <.text+0x1d84ca8>
141d850bd: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
141d850c4: 48 8b 10                    	movq	(%rax), %rdx
141d850c7: 48 89 95 a8 00 00 00        	movq	%rdx, 0xa8(%rbp)
141d850ce: 48 8b 40 08                 	movq	0x8(%rax), %rax
141d850d2: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
141d850d9: 48 8b 09                    	movq	(%rcx), %rcx
141d850dc: 4c 89 ad 28 01 00 00        	movq	%r13, 0x128(%rbp)
141d850e3: e8 98 8c 8f fe              	callq	0x14067dd80 <.text+0x67cd80>
141d850e8: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
141d850ec: 48 8d 50 20                 	leaq	0x20(%rax), %rdx
141d850f0: 4c 8d 40 10                 	leaq	0x10(%rax), %r8
141d850f4: 49 89 c1                    	movq	%rax, %r9
141d850f7: 49 83 c1 28                 	addq	$0x28, %r9
141d850fb: 80 b8 98 02 00 00 00        	cmpb	$0x0, 0x298(%rax)
141d85102: 48 0f 45 ca                 	cmovneq	%rdx, %rcx
141d85106: 4d 0f 44 c8                 	cmoveq	%r8, %r9
141d8510a: 49 8b 01                    	movq	(%r9), %rax
141d8510d: 48 8b 09                    	movq	(%rcx), %rcx
141d85110: 48 89 8d 80 00 00 00        	movq	%rcx, 0x80(%rbp)
141d85117: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
141d8511e: 49 8b 17                    	movq	(%r15), %rdx
141d85121: 4c 89 ad 28 01 00 00        	movq	%r13, 0x128(%rbp)
141d85128: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
141d8512c: e8 1f d5 c8 fe              	callq	0x140a12650 <.text+0xa11650>
141d85131: f3 44 0f 10 55 c0           	movss	-0x40(%rbp), %xmm10
141d85137: f3 0f 10 75 c8              	movss	-0x38(%rbp), %xmm6
141d8513c: 49 8b 06                    	movq	(%r14), %rax
141d8513f: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
141d85143: 48 8b 90 f8 00 00 00        	movq	0xf8(%rax), %rdx
141d8514a: 4c 8b 80 00 01 00 00        	movq	0x100(%rax), %r8
141d85151: 4e 8d 04 82                 	leaq	(%rdx,%r8,4), %r8
141d85155: 44 0f b6 88 08 01 00 00     	movzbl	0x108(%rax), %r9d
141d8515d: 0f 57 c0                    	xorps	%xmm0, %xmm0
141d85160: 0f 11 45 18                 	movups	%xmm0, 0x18(%rbp)
141d85164: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
141d85168: 48 c7 45 28 00 00 00 00     	movq	$0x0, 0x28(%rbp)
141d85170: 4c 8d 91 f8 00 00 00        	leaq	0xf8(%rcx), %r10
141d85177: 48 89 4d c0                 	movq	%rcx, -0x40(%rbp)
141d8517b: 48 81 c1 a0 01 00 00        	addq	$0x1a0, %rcx            # imm = 0x1A0
141d85182: 48 89 4d c8                 	movq	%rcx, -0x38(%rbp)
141d85186: 4c 89 55 d0                 	movq	%r10, -0x30(%rbp)
141d8518a: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
141d8518e: 48 89 55 e0                 	movq	%rdx, -0x20(%rbp)
141d85192: 4c 89 45 e8                 	movq	%r8, -0x18(%rbp)
141d85196: 48 c7 45 f0 08 00 00 00     	movq	$0x8, -0x10(%rbp)
141d8519e: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
141d851a6: 48 c7 45 00 08 00 00 00     	movq	$0x8, (%rbp)
141d851ae: 44 88 4d 30                 	movb	%r9b, 0x30(%rbp)
141d851b2: 4c 89 ad 28 01 00 00        	movq	%r13, 0x128(%rbp)
141d851b9: 4c 8d 05 98 2c 1e 01        	leaq	0x11e2c98(%rip), %r8    # 0x142f67e58
141d851c0: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
141d851c7: 48 8d 55 c0                 	leaq	-0x40(%rbp), %rdx
141d851cb: e8 d0 a9 03 00              	callq	0x141dbfba0 <.text+0x1dbeba0>
141d851d0: 4d 8b 04 24                 	movq	(%r12), %r8
141d851d4: 49 8b 54 24 08              	movq	0x8(%r12), %rdx
141d851d9: 45 8b 4c 24 10              	movl	0x10(%r12), %r9d
141d851de: 41 8b 44 24 14              	movl	0x14(%r12), %eax
141d851e3: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
141d851e7: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
141d851eb: 48 89 95 f8 00 00 00        	movq	%rdx, 0xf8(%rbp)
141d851f2: 4c 89 85 20 01 00 00        	movq	%r8, 0x120(%rbp)
141d851f9: e8 82 98 58 00              	callq	0x14230ea80 <.text+0x230da80>
141d851fe: 48 8b 85 88 02 00 00        	movq	0x288(%rbp), %rax
141d85205: 48 8b 7d e0                 	movq	-0x20(%rbp), %rdi
141d85209: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
141d8520d: 48 8b 4d 00                 	movq	(%rbp), %rcx
141d85211: 48 89 8d d0 00 00 00        	movq	%rcx, 0xd0(%rbp)
141d85218: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
141d8521c: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
141d85223: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
141d85227: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
141d8522e: 8b 4d 2c                    	movl	0x2c(%rbp), %ecx
141d85231: 89 8d 54 01 00 00           	movl	%ecx, 0x154(%rbp)
141d85237: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
141d8523b: 48 89 8d 30 01 00 00        	movq	%rcx, 0x130(%rbp)
141d85242: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
141d85246: 48 89 8d 38 01 00 00        	movq	%rcx, 0x138(%rbp)
141d8524d: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
141d85251: 48 89 8d 40 01 00 00        	movq	%rcx, 0x140(%rbp)
141d85258: 4c 8b 65 68                 	movq	0x68(%rbp), %r12
141d8525c: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
141d85260: 48 8b 00                    	movq	(%rax), %rax
141d85263: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
141d8526a: f3 44 0f 10 1d fd 05 ba 00  	movss	0xba05fd(%rip), %xmm11  # 0x142925870
141d85273: f3 45 0f 59 d9              	mulss	%xmm9, %xmm11
141d85278: 48 8b b5 e8 00 00 00        	movq	0xe8(%rbp), %rsi
141d8527f: 44 0f 14 d6                 	unpcklps	%xmm6, %xmm10           # xmm10 = xmm10[0],xmm6[0],xmm10[1],xmm6[1]
141d85283: 48 c1 e6 03                 	shlq	$0x3, %rsi
141d85287: f3 44 0f 10 25 e8 7d c4 00  	movss	0xc47de8(%rip), %xmm12  # 0x1429cd078
141d85290: f3 44 0f 10 2d fb f3 ba 00  	movss	0xbaf3fb(%rip), %xmm13  # 0x142934694
141d85299: 45 0f 57 f6                 	xorps	%xmm14, %xmm14
141d8529d: f3 0f 10 35 5f 06 ba 00     	movss	0xba065f(%rip), %xmm6   # 0x142925904
141d852a5: f3 44 0f 10 3d e6 f3 ba 00  	movss	0xbaf3e6(%rip), %xmm15  # 0x142934694
141d852ae: f3 0f 10 3d ba 05 ba 00     	movss	0xba05ba(%rip), %xmm7   # 0x142925870
141d852b6: 0f b6 45 78                 	movzbl	0x78(%rbp), %eax
141d852ba: 88 85 52 01 00 00           	movb	%al, 0x152(%rbp)
141d852c0: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
141d852c7: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
141d852ce: 48 89 95 a0 00 00 00        	movq	%rdx, 0xa0(%rbp)
141d852d5: eb 4e                       	jmp	0x141d85325 <.text+0x1d84325>
141d852d7: f3 0f 10 05 85 7d c4 00     	movss	0xc47d85(%rip), %xmm0   # 0x1429cd064
141d852df: 8b 85 54 01 00 00           	movl	0x154(%rbp), %eax
141d852e5: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
141d852ec: 89 01                       	movl	%eax, (%rcx)
141d852ee: 45 89 7d 00                 	movl	%r15d, (%r13)
141d852f2: f3 41 0f 11 45 04           	movss	%xmm0, 0x4(%r13)
141d852f8: f3 45 0f 59 c1              	mulss	%xmm9, %xmm8
141d852fd: f3 45 0f 58 45 14           	addss	0x14(%r13), %xmm8
141d85303: 89 01                       	movl	%eax, (%rcx)
141d85305: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
141d85309: 0f 28 ce                    	movaps	%xmm6, %xmm1
141d8530c: e8 ff eb b9 00              	callq	0x142923f10 <.text+0x2922f10>
141d85311: f3 41 0f 11 45 14           	movss	%xmm0, 0x14(%r13)
141d85317: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
141d8531e: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
141d85325: f6 85 52 01 00 00 01        	testb	$0x1, 0x152(%rbp)
141d8532c: 74 42                       	je	0x141d85370 <.text+0x1d84370>
141d8532e: 4c 39 e1                    	cmpq	%r12, %rcx
141d85331: 0f 85 62 02 00 00           	jne	0x141d85599 <.text+0x1d84599>
141d85337: 48 39 d7                    	cmpq	%rdx, %rdi
141d8533a: 0f 84 13 08 00 00           	je	0x141d85b53 <.text+0x1d84b53>
141d85340: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
141d85347: 48 8b 80 a8 01 00 00        	movq	0x1a8(%rax), %rax
141d8534e: 66 90                       	nop
141d85350: 8b 0f                       	movl	(%rdi), %ecx
141d85352: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
141d85356: 4c 8b 64 c8 10              	movq	0x10(%rax,%rcx,8), %r12
141d8535b: 4d 85 e4                    	testq	%r12, %r12
141d8535e: 75 52                       	jne	0x141d853b2 <.text+0x1d843b2>
141d85360: 48 83 c7 04                 	addq	$0x4, %rdi
141d85364: 48 39 d7                    	cmpq	%rdx, %rdi
141d85367: 75 e7                       	jne	0x141d85350 <.text+0x1d84350>
141d85369: e9 e5 07 00 00              	jmp	0x141d85b53 <.text+0x1d84b53>
141d8536e: 66 90                       	nop
141d85370: 4c 39 e1                    	cmpq	%r12, %rcx
141d85373: 0f 85 c3 02 00 00           	jne	0x141d8563c <.text+0x1d8463c>
141d85379: 48 39 d7                    	cmpq	%rdx, %rdi
141d8537c: 0f 84 d1 07 00 00           	je	0x141d85b53 <.text+0x1d84b53>
141d85382: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
141d85389: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
141d85390: 8b 0f                       	movl	(%rdi), %ecx
141d85392: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
141d85396: 48 c1 e1 05                 	shlq	$0x5, %rcx
141d8539a: 4c 8b 64 08 58              	movq	0x58(%rax,%rcx), %r12
141d8539f: 4d 85 e4                    	testq	%r12, %r12
141d853a2: 75 57                       	jne	0x141d853fb <.text+0x1d843fb>
141d853a4: 48 83 c7 04                 	addq	$0x4, %rdi
141d853a8: 48 39 d7                    	cmpq	%rdx, %rdi
141d853ab: 75 e3                       	jne	0x141d85390 <.text+0x1d84390>
141d853ad: e9 a1 07 00 00              	jmp	0x141d85b53 <.text+0x1d84b53>
141d853b2: 48 8d 0c c8                 	leaq	(%rax,%rcx,8), %rcx
141d853b6: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
141d853bd: 48 8b 90 10 01 00 00        	movq	0x110(%rax), %rdx
141d853c4: 48 8b 41 40                 	movq	0x40(%rcx), %rax
141d853c8: 48 39 d0                    	cmpq	%rdx, %rax
141d853cb: 0f 86 8b 00 00 00           	jbe	0x141d8545c <.text+0x1d8445c>
141d853d1: 4c 8b 41 38                 	movq	0x38(%rcx), %r8
141d853d5: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
141d853d9: 48 85 d2                    	testq	%rdx, %rdx
141d853dc: 74 7e                       	je	0x141d8545c <.text+0x1d8445c>
141d853de: 4c 8b 41 18                 	movq	0x18(%rcx), %r8
141d853e2: 48 f7 d2                    	notq	%rdx
141d853e5: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
141d853e9: 48 c1 e2 04                 	shlq	$0x4, %rdx
141d853ed: 49 8b 54 10 10              	movq	0x10(%r8,%rdx), %rdx
141d853f2: 48 89 95 30 01 00 00        	movq	%rdx, 0x130(%rbp)
141d853f9: eb 6c                       	jmp	0x141d85467 <.text+0x1d84467>
141d853fb: 48 01 c8                    	addq	%rcx, %rax
141d853fe: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
141d85404: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
141d8540b: 4c 8b 8a a8 01 00 00        	movq	0x1a8(%rdx), %r9
141d85412: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
141d85416: 4d 8d 04 c9                 	leaq	(%r9,%rcx,8), %r8
141d8541a: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
141d85421: 48 8b 92 10 01 00 00        	movq	0x110(%rdx), %rdx
141d85428: 49 8b 4c c9 40              	movq	0x40(%r9,%rcx,8), %rcx
141d8542d: 48 39 d1                    	cmpq	%rdx, %rcx
141d85430: 76 7a                       	jbe	0x141d854ac <.text+0x1d844ac>
141d85432: 4d 8b 48 38                 	movq	0x38(%r8), %r9
141d85436: 49 8b 14 d1                 	movq	(%r9,%rdx,8), %rdx
141d8543a: 48 85 d2                    	testq	%rdx, %rdx
141d8543d: 74 6d                       	je	0x141d854ac <.text+0x1d844ac>
141d8543f: 4d 8b 48 18                 	movq	0x18(%r8), %r9
141d85443: 48 f7 d2                    	notq	%rdx
141d85446: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
141d8544a: 48 c1 e2 04                 	shlq	$0x4, %rdx
141d8544e: 49 8b 54 11 10              	movq	0x10(%r9,%rdx), %rdx
141d85453: 48 89 95 30 01 00 00        	movq	%rdx, 0x130(%rbp)
141d8545a: eb 5b                       	jmp	0x141d854b7 <.text+0x1d844b7>
141d8545c: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
141d85467: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
141d8546e: 4c 8b 82 18 01 00 00        	movq	0x118(%rdx), %r8
141d85475: 48 8b 51 38                 	movq	0x38(%rcx), %rdx
141d85479: 4c 39 c0                    	cmpq	%r8, %rax
141d8547c: 0f 86 94 00 00 00           	jbe	0x141d85516 <.text+0x1d84516>
141d85482: 4e 8b 04 c2                 	movq	(%rdx,%r8,8), %r8
141d85486: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
141d8548a: 4d 85 c0                    	testq	%r8, %r8
141d8548d: 0f 84 87 00 00 00           	je	0x141d8551a <.text+0x1d8451a>
141d85493: 49 f7 d0                    	notq	%r8
141d85496: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
141d8549a: 49 c1 e0 04                 	shlq	$0x4, %r8
141d8549e: 4e 8b 44 01 10              	movq	0x10(%rcx,%r8), %r8
141d854a3: 4c 89 85 38 01 00 00        	movq	%r8, 0x138(%rbp)
141d854aa: eb 79                       	jmp	0x141d85525 <.text+0x1d84525>
141d854ac: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
141d854b7: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
141d854be: 4c 8b 8a 18 01 00 00        	movq	0x118(%rdx), %r9
141d854c5: 49 8b 50 38                 	movq	0x38(%r8), %rdx
141d854c9: 4c 39 c9                    	cmpq	%r9, %rcx
141d854cc: 0f 86 d5 00 00 00           	jbe	0x141d855a7 <.text+0x1d845a7>
141d854d2: 4e 8b 0c ca                 	movq	(%rdx,%r9,8), %r9
141d854d6: 4d 8b 40 18                 	movq	0x18(%r8), %r8
141d854da: 4d 85 c9                    	testq	%r9, %r9
141d854dd: 0f 84 c8 00 00 00           	je	0x141d855ab <.text+0x1d845ab>
141d854e3: 49 f7 d1                    	notq	%r9
141d854e6: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
141d854ea: 49 c1 e1 04                 	shlq	$0x4, %r9
141d854ee: 4f 8b 4c 08 10              	movq	0x10(%r8,%r9), %r9
141d854f3: 4c 89 8d 38 01 00 00        	movq	%r9, 0x138(%rbp)
141d854fa: 4c 8b 8d 20 01 00 00        	movq	0x120(%rbp), %r9
141d85501: 4d 8b 89 30 01 00 00        	movq	0x130(%r9), %r9
141d85508: 4c 39 c9                    	cmpq	%r9, %rcx
141d8550b: 0f 87 b8 00 00 00           	ja	0x141d855c9 <.text+0x1d845c9>
141d85511: e9 d5 00 00 00              	jmp	0x141d855eb <.text+0x1d845eb>
141d85516: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
141d8551a: 48 c7 85 38 01 00 00 00 00 00 00    	movq	$0x0, 0x138(%rbp)
141d85525: 48 83 c7 04                 	addq	$0x4, %rdi
141d85529: 4c 8b 85 20 01 00 00        	movq	0x120(%rbp), %r8
141d85530: 4d 8b 88 20 01 00 00        	movq	0x120(%r8), %r9
141d85537: 4d 8b 80 30 01 00 00        	movq	0x130(%r8), %r8
141d8553e: 4e 8b 0c ca                 	movq	(%rdx,%r9,8), %r9
141d85542: 49 f7 d1                    	notq	%r9
141d85545: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
141d85549: 49 c1 e1 04                 	shlq	$0x4, %r9
141d8554d: 4e 8b 54 09 10              	movq	0x10(%rcx,%r9), %r10
141d85552: 4c 89 95 00 01 00 00        	movq	%r10, 0x100(%rbp)
141d85559: 4e 8b 4c 09 28              	movq	0x28(%rcx,%r9), %r9
141d8555e: 4c 89 8d 08 01 00 00        	movq	%r9, 0x108(%rbp)
141d85565: 4c 39 c0                    	cmpq	%r8, %rax
141d85568: 76 22                       	jbe	0x141d8558c <.text+0x1d8458c>
141d8556a: 4a 8b 04 c2                 	movq	(%rdx,%r8,8), %rax
141d8556e: 48 85 c0                    	testq	%rax, %rax
141d85571: 74 19                       	je	0x141d8558c <.text+0x1d8458c>
141d85573: 48 f7 d0                    	notq	%rax
141d85576: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
141d8557a: 48 c1 e0 04                 	shlq	$0x4, %rax
141d8557e: 48 8b 44 01 10              	movq	0x10(%rcx,%rax), %rax
141d85583: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
141d8558a: eb 0b                       	jmp	0x141d85597 <.text+0x1d84597>
141d8558c: 48 c7 85 40 01 00 00 00 00 00 00    	movq	$0x0, 0x140(%rbp)
141d85597: 31 c9                       	xorl	%ecx, %ecx
141d85599: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
141d855a0: 89 cb                       	movl	%ecx, %ebx
141d855a2: e9 ae 00 00 00              	jmp	0x141d85655 <.text+0x1d84655>
141d855a7: 4d 8b 40 18                 	movq	0x18(%r8), %r8
141d855ab: 48 c7 85 38 01 00 00 00 00 00 00    	movq	$0x0, 0x138(%rbp)
141d855b6: 4c 8b 8d 20 01 00 00        	movq	0x120(%rbp), %r9
141d855bd: 4d 8b 89 30 01 00 00        	movq	0x130(%r9), %r9
141d855c4: 4c 39 c9                    	cmpq	%r9, %rcx
141d855c7: 76 22                       	jbe	0x141d855eb <.text+0x1d845eb>
141d855c9: 4a 8b 0c ca                 	movq	(%rdx,%r9,8), %rcx
141d855cd: 48 85 c9                    	testq	%rcx, %rcx
141d855d0: 74 19                       	je	0x141d855eb <.text+0x1d845eb>
141d855d2: 48 f7 d1                    	notq	%rcx
141d855d5: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
141d855d9: 48 c1 e1 04                 	shlq	$0x4, %rcx
141d855dd: 49 8b 4c 08 10              	movq	0x10(%r8,%rcx), %rcx
141d855e2: 48 89 8d 40 01 00 00        	movq	%rcx, 0x140(%rbp)
141d855e9: eb 0b                       	jmp	0x141d855f6 <.text+0x1d845f6>
141d855eb: 48 c7 85 40 01 00 00 00 00 00 00    	movq	$0x0, 0x140(%rbp)
141d855f6: 48 83 c7 04                 	addq	$0x4, %rdi
141d855fa: 48 8b 8d 20 01 00 00        	movq	0x120(%rbp), %rcx
141d85601: 48 8b 89 20 01 00 00        	movq	0x120(%rcx), %rcx
141d85608: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
141d8560c: 48 f7 d1                    	notq	%rcx
141d8560f: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
141d85613: 48 c1 e1 04                 	shlq	$0x4, %rcx
141d85617: 49 8b 54 08 10              	movq	0x10(%r8,%rcx), %rdx
141d8561c: 48 89 95 00 01 00 00        	movq	%rdx, 0x100(%rbp)
141d85623: 49 8b 4c 08 28              	movq	0x28(%r8,%rcx), %rcx
141d85628: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
141d8562f: 48 8b 40 50                 	movq	0x50(%rax), %rax
141d85633: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
141d8563a: 31 c9                       	xorl	%ecx, %ecx
141d8563c: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
141d85643: 48 89 c8                    	movq	%rcx, %rax
141d85646: 48 c1 e0 04                 	shlq	$0x4, %rax
141d8564a: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
141d85651: 8b 5c 01 08                 	movl	0x8(%rcx,%rax), %ebx
141d85655: 4c 8d 34 5b                 	leaq	(%rbx,%rbx,2), %r14
141d85659: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
141d85660: 4e 8d 3c b0                 	leaq	(%rax,%r14,4), %r15
141d85664: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
141d8566b: 4e 8d 2c f0                 	leaq	(%rax,%r14,8), %r13
141d8566f: 49 c1 e6 04                 	shlq	$0x4, %r14
141d85673: 4c 03 b5 30 01 00 00        	addq	0x130(%rbp), %r14
141d8567a: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
141d85681: 48 8d 04 98                 	leaq	(%rax,%rbx,4), %rax
141d85685: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
141d8568c: 48 c1 e3 04                 	shlq	$0x4, %rbx
141d85690: 48 03 9d 40 01 00 00        	addq	0x140(%rbp), %rbx
141d85697: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
141d8569e: e8 8d bb b8 fe              	callq	0x140911230 <.text+0x910230>
141d856a3: 3c 03                       	cmpb	$0x3, %al
141d856a5: 75 49                       	jne	0x141d856f0 <.text+0x1d846f0>
141d856a7: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
141d856ab: 48 8b 95 c8 00 00 00        	movq	0xc8(%rbp), %rdx
141d856b2: e8 19 c7 b8 fe              	callq	0x140911dd0 <.text+0x910dd0>
141d856b7: f3 0f 10 4d c0              	movss	-0x40(%rbp), %xmm1
141d856bc: f3 0f 10 45 c8              	movss	-0x38(%rbp), %xmm0
141d856c1: 0f 14 c8                    	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
141d856c4: f3 41 0f 10 46 10           	movss	0x10(%r14), %xmm0
141d856ca: f3 41 0f 10 56 18           	movss	0x18(%r14), %xmm2
141d856d0: 0f 14 c2                    	unpcklps	%xmm2, %xmm0            # xmm0 = xmm0[0],xmm2[0],xmm0[1],xmm2[1]
141d856d3: 0f 5c c8                    	subps	%xmm0, %xmm1
141d856d6: 0f 59 c9                    	mulps	%xmm1, %xmm1
141d856d9: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
141d856dd: f3 0f 58 d1                 	addss	%xmm1, %xmm2
141d856e1: 0f 57 c9                    	xorps	%xmm1, %xmm1
141d856e4: f3 0f 51 ca                 	sqrtss	%xmm2, %xmm1
141d856e8: 44 0f 2e e1                 	ucomiss	%xmm1, %xmm12
141d856ec: eb 27                       	jmp	0x141d85715 <.text+0x1d84715>
141d856ee: 66 90                       	nop
141d856f0: f3 41 0f 10 46 10           	movss	0x10(%r14), %xmm0
141d856f6: f3 41 0f 10 4e 18           	movss	0x18(%r14), %xmm1
141d856fc: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
141d856ff: 0f 28 c8                    	movaps	%xmm0, %xmm1
141d85702: 41 0f 5c ca                 	subps	%xmm10, %xmm1
141d85706: 0f 59 c9                    	mulps	%xmm1, %xmm1
141d85709: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
141d8570d: f3 0f 58 d1                 	addss	%xmm1, %xmm2
141d85711: 44 0f 2e ea                 	ucomiss	%xmm2, %xmm13
141d85715: 41 0f 97 c0                 	seta	%r8b
141d85719: 48 ff 85 f0 00 00 00        	incq	0xf0(%rbp)
141d85720: 31 c9                       	xorl	%ecx, %ecx
141d85722: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
141d85730: 48 89 c8                    	movq	%rcx, %rax
141d85733: 48 39 ce                    	cmpq	%rcx, %rsi
141d85736: 74 27                       	je	0x141d8575f <.text+0x1d8475f>
141d85738: 48 8b 8d 18 01 00 00        	movq	0x118(%rbp), %rcx
141d8573f: f2 0f 10 0c 01              	movsd	(%rcx,%rax), %xmm1
141d85744: 0f 28 d0                    	movaps	%xmm0, %xmm2
141d85747: 0f 5c d1                    	subps	%xmm1, %xmm2
141d8574a: 0f 59 d2                    	mulps	%xmm2, %xmm2
141d8574d: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
141d85751: f3 0f 58 ca                 	addss	%xmm2, %xmm1
141d85755: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
141d85759: 44 0f 2e e9                 	ucomiss	%xmm1, %xmm13
141d8575d: 76 d1                       	jbe	0x141d85730 <.text+0x1d84730>
141d8575f: 48 39 c6                    	cmpq	%rax, %rsi
141d85762: 0f 95 c0                    	setne	%al
141d85765: f3 41 0f 10 4d 10           	movss	0x10(%r13), %xmm1
141d8576b: f3 41 0f 5c cb              	subss	%xmm11, %xmm1
141d85770: f3 41 0f 5f ce              	maxss	%xmm14, %xmm1
141d85775: 8b 8d 54 01 00 00           	movl	0x154(%rbp), %ecx
141d8577b: 48 8b 95 48 01 00 00        	movq	0x148(%rbp), %rdx
141d85782: 89 0a                       	movl	%ecx, (%rdx)
141d85784: f3 41 0f 11 4d 10           	movss	%xmm1, 0x10(%r13)
141d8578a: 41 08 c0                    	orb	%al, %r8b
141d8578d: 44 88 85 53 01 00 00        	movb	%r8b, 0x153(%rbp)
141d85794: 41 80 f8 01                 	cmpb	$0x1, %r8b
141d85798: 75 1a                       	jne	0x141d857b4 <.text+0x1d847b4>
141d8579a: 8b 85 54 01 00 00           	movl	0x154(%rbp), %eax
141d857a0: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
141d857a7: 89 01                       	movl	%eax, (%rcx)
141d857a9: 41 c7 45 10 00 00 80 3f     	movl	$0x3f800000, 0x10(%r13) # imm = 0x3F800000
141d857b1: 0f 28 ce                    	movaps	%xmm6, %xmm1
141d857b4: f3 41 0f 10 45 0c           	movss	0xc(%r13), %xmm0
141d857ba: 41 0f 2e c6                 	ucomiss	%xmm14, %xmm0
141d857be: 0f 86 9c 00 00 00           	jbe	0x141d85860 <.text+0x1d84860>
141d857c4: 45 0f 2e 77 04              	ucomiss	0x4(%r15), %xmm14
141d857c9: 0f 82 91 00 00 00           	jb	0x141d85860 <.text+0x1d84860>
141d857cf: 0f 28 d7                    	movaps	%xmm7, %xmm2
141d857d2: f3 41 0f 5c 55 14           	subss	0x14(%r13), %xmm2
141d857d8: 0f 54 15 a1 91 ba 00        	andps	0xba91a1(%rip), %xmm2   # 0x14292e980
141d857df: 0f 2e 15 82 78 c4 00        	ucomiss	0xc47882(%rip), %xmm2   # 0x1429cd068
141d857e6: 72 7e                       	jb	0x141d85866 <.text+0x1d84866>
141d857e8: f3 0f 10 1d b4 a4 cf 00     	movss	0xcfa4b4(%rip), %xmm3   # 0x142a7fca4
141d857f0: 0f 2e da                    	ucomiss	%xmm2, %xmm3
141d857f3: 76 71                       	jbe	0x141d85866 <.text+0x1d84866>
141d857f5: 8b 85 54 01 00 00           	movl	0x154(%rbp), %eax
141d857fb: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
141d85802: 89 01                       	movl	%eax, (%rcx)
141d85804: f3 41 0f 10 45 0c           	movss	0xc(%r13), %xmm0
141d8580a: f3 41 0f 5c c1              	subss	%xmm9, %xmm0
141d8580f: f3 41 0f 11 45 0c           	movss	%xmm0, 0xc(%r13)
141d85815: 41 b7 01                    	movb	$0x1, %r15b
141d85818: 44 0f 2e f0                 	ucomiss	%xmm0, %xmm14
141d8581c: 73 59                       	jae	0x141d85877 <.text+0x1d84877>
141d8581e: f3 41 0f 10 4d 10           	movss	0x10(%r13), %xmm1
141d85824: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
141d85828: f3 44 0f c2 c1 01           	cmpltss	%xmm1, %xmm8
141d8582e: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
141d85832: 41 0f 54 c7                 	andps	%xmm15, %xmm0
141d85836: 44 0f 55 c6                 	andnps	%xmm6, %xmm8
141d8583a: 44 0f 56 c0                 	orps	%xmm0, %xmm8
141d8583e: 41 8b 45 00                 	movl	(%r13), %eax
141d85842: 48 8d 0d bf 27 1e 01        	leaq	0x11e27bf(%rip), %rcx   # 0x142f68008
141d85849: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
141d8584d: 48 01 c8                    	addq	%rcx, %rax
141d85850: ff e0                       	jmpq	*%rax
141d85852: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
141d85860: 41 0f 2e c6                 	ucomiss	%xmm14, %xmm0
141d85864: 76 5c                       	jbe	0x141d858c2 <.text+0x1d848c2>
141d85866: 44 0f 2e f0                 	ucomiss	%xmm0, %xmm14
141d8586a: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
141d8586e: 0f 82 e1 00 00 00           	jb	0x141d85955 <.text+0x1d84955>
141d85874: 45 31 ff                    	xorl	%r15d, %r15d
141d85877: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
141d8587e: e8 5d 02 8a ff              	callq	0x141625ae0 <.text+0x1624ae0>
141d85883: f3 0f 59 05 45 00 ba 00     	mulss	0xba0045(%rip), %xmm0   # 0x1429258d0
141d8588b: f3 0f 58 c7                 	addss	%xmm7, %xmm0
141d8588f: 0f 57 05 5a 00 ba 00        	xorps	0xba005a(%rip), %xmm0   # 0x1429258f0
141d85896: 8b 85 54 01 00 00           	movl	0x154(%rbp), %eax
141d8589c: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
141d858a3: 89 01                       	movl	%eax, (%rcx)
141d858a5: f3 41 0f 11 45 0c           	movss	%xmm0, 0xc(%r13)
141d858ab: f3 41 0f 10 4d 10           	movss	0x10(%r13), %xmm1
141d858b1: 45 84 ff                    	testb	%r15b, %r15b
141d858b4: 0f 84 97 00 00 00           	je	0x141d85951 <.text+0x1d84951>
141d858ba: 0f 28 c6                    	movaps	%xmm6, %xmm0
141d858bd: e9 93 00 00 00              	jmp	0x141d85955 <.text+0x1d84955>
141d858c2: 8b 85 54 01 00 00           	movl	0x154(%rbp), %eax
141d858c8: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
141d858cf: 89 01                       	movl	%eax, (%rcx)
141d858d1: f3 41 0f 10 45 0c           	movss	0xc(%r13), %xmm0
141d858d7: f3 41 0f 58 c1              	addss	%xmm9, %xmm0
141d858dc: f3 41 0f 11 45 0c           	movss	%xmm0, 0xc(%r13)
141d858e2: 41 0f 2e c6                 	ucomiss	%xmm14, %xmm0
141d858e6: 0f 86 02 01 00 00           	jbe	0x141d859ee <.text+0x1d849ee>
141d858ec: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
141d858f3: e8 e8 01 8a ff              	callq	0x141625ae0 <.text+0x1624ae0>
141d858f8: f3 0f 59 05 d8 ff b9 00     	mulss	0xb9ffd8(%rip), %xmm0   # 0x1429258d8
141d85900: f3 0f 58 05 60 77 c4 00     	addss	0xc47760(%rip), %xmm0   # 0x1429cd068
141d85908: 8b 85 54 01 00 00           	movl	0x154(%rbp), %eax
141d8590e: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
141d85915: 89 01                       	movl	%eax, (%rcx)
141d85917: f3 41 0f 11 45 0c           	movss	%xmm0, 0xc(%r13)
141d8591d: f3 41 0f 10 4d 10           	movss	0x10(%r13), %xmm1
141d85923: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
141d85927: f3 44 0f c2 c1 01           	cmpltss	%xmm1, %xmm8
141d8592d: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
141d85931: 41 0f 54 c7                 	andps	%xmm15, %xmm0
141d85935: 45 0f 55 c4                 	andnps	%xmm12, %xmm8
141d85939: 44 0f 56 c0                 	orps	%xmm0, %xmm8
141d8593d: 41 8b 45 00                 	movl	(%r13), %eax
141d85941: 48 8d 0d c0 26 1e 01        	leaq	0x11e26c0(%rip), %rcx   # 0x142f68008
141d85948: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
141d8594c: 48 01 c8                    	addq	%rcx, %rax
141d8594f: ff e0                       	jmpq	*%rax
141d85951: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
141d85955: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
141d85959: f3 44 0f c2 c1 01           	cmpltss	%xmm1, %xmm8
141d8595f: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
141d85963: 41 0f 54 d7                 	andps	%xmm15, %xmm2
141d85967: 44 0f 55 c0                 	andnps	%xmm0, %xmm8
141d8596b: 44 0f 56 c2                 	orps	%xmm2, %xmm8
141d8596f: 41 8b 45 00                 	movl	(%r13), %eax
141d85973: 48 8d 0d 8e 26 1e 01        	leaq	0x11e268e(%rip), %rcx   # 0x142f68008
141d8597a: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
141d8597e: 48 01 c8                    	addq	%rcx, %rax
141d85981: ff e0                       	jmpq	*%rax
141d85983: f3 41 0f 10 45 14           	movss	0x14(%r13), %xmm0
141d85989: 0f 2e f8                    	ucomiss	%xmm0, %xmm7
141d8598c: 72 33                       	jb	0x141d859c1 <.text+0x1d849c1>
141d8598e: 0f 28 cf                    	movaps	%xmm7, %xmm1
141d85991: f3 0f 5c c8                 	subss	%xmm0, %xmm1
141d85995: f3 0f 59 0d e7 ff b9 00     	mulss	0xb9ffe7(%rip), %xmm1   # 0x142925984
141d8599d: f3 41 0f 5e c9              	divss	%xmm9, %xmm1
141d859a2: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
141d859a6: f3 41 0f c2 c0 03           	cmpunordss	%xmm8, %xmm0
141d859ac: 0f 28 d0                    	movaps	%xmm0, %xmm2
141d859af: 0f 54 d1                    	andps	%xmm1, %xmm2
141d859b2: f3 41 0f 5d c8              	minss	%xmm8, %xmm1
141d859b7: 0f 55 c1                    	andnps	%xmm1, %xmm0
141d859ba: 0f 56 c2                    	orps	%xmm2, %xmm0
141d859bd: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
141d859c1: f3 41 0f 10 45 04           	movss	0x4(%r13), %xmm0
141d859c7: f3 41 0f 5c c1              	subss	%xmm9, %xmm0
141d859cc: 31 c0                       	xorl	%eax, %eax
141d859ce: 41 0f 2e c6                 	ucomiss	%xmm14, %xmm0
141d859d2: 0f 96 c0                    	setbe	%al
141d859d5: 44 8d 3c 40                 	leal	(%rax,%rax,2), %r15d
141d859d9: 80 bd 53 01 00 00 00        	cmpb	$0x0, 0x153(%rbp)
141d859e0: b8 03 00 00 00              	movl	$0x3, %eax
141d859e5: 44 0f 45 f8                 	cmovnel	%eax, %r15d
141d859e9: e9 f1 f8 ff ff              	jmp	0x141d852df <.text+0x1d842df>
141d859ee: f3 41 0f 10 4d 10           	movss	0x10(%r13), %xmm1
141d859f4: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
141d859f8: f3 44 0f c2 c1 01           	cmpltss	%xmm1, %xmm8
141d859fe: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
141d85a02: 41 0f 54 c7                 	andps	%xmm15, %xmm0
141d85a06: 45 0f 55 c4                 	andnps	%xmm12, %xmm8
141d85a0a: 44 0f 56 c0                 	orps	%xmm0, %xmm8
141d85a0e: 41 8b 45 00                 	movl	(%r13), %eax
141d85a12: 48 8d 0d ef 25 1e 01        	leaq	0x11e25ef(%rip), %rcx   # 0x142f68008
141d85a19: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
141d85a1d: 48 01 c8                    	addq	%rcx, %rax
141d85a20: ff e0                       	jmpq	*%rax
141d85a22: 41 bf 02 00 00 00           	movl	$0x2, %r15d
141d85a28: 41 0f 2e ce                 	ucomiss	%xmm14, %xmm1
141d85a2c: 0f 85 a5 f8 ff ff           	jne	0x141d852d7 <.text+0x1d842d7>
141d85a32: 0f 8a 9f f8 ff ff           	jp	0x141d852d7 <.text+0x1d842d7>
141d85a38: 83 3b 00                    	cmpl	$0x0, (%rbx)
141d85a3b: 0f 84 96 f8 ff ff           	je	0x141d852d7 <.text+0x1d842d7>
141d85a41: f3 41 0f 10 56 10           	movss	0x10(%r14), %xmm2
141d85a47: f3 41 0f 10 5e 18           	movss	0x18(%r14), %xmm3
141d85a4d: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
141d85a54: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
141d85a5b: e8 a0 4d c6 fe              	callq	0x1409ea800 <.text+0x9e9800>
141d85a60: 84 c0                       	testb	%al, %al
141d85a62: 0f 85 6f f8 ff ff           	jne	0x141d852d7 <.text+0x1d842d7>
141d85a68: f3 41 0f 10 4e 10           	movss	0x10(%r14), %xmm1
141d85a6e: f3 41 0f 10 56 18           	movss	0x18(%r14), %xmm2
141d85a74: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
141d85a7c: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
141d85a83: f3 0f 10 1d cd 5b cf 00     	movss	0xcf5bcd(%rip), %xmm3   # 0x142a7b658
141d85a8b: e8 60 f3 c8 fe              	callq	0x140a14df0 <.text+0xa13df0>
141d85a90: f3 0f 10 0d dc ab bd 00     	movss	0xbdabdc(%rip), %xmm1   # 0x142960674
141d85a98: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
141d85a9b: 0f 86 36 f8 ff ff           	jbe	0x141d852d7 <.text+0x1d842d7>
141d85aa1: 41 bf 01 00 00 00           	movl	$0x1, %r15d
141d85aa7: f3 0f 10 05 59 fe b9 00     	movss	0xb9fe59(%rip), %xmm0   # 0x142925908
141d85aaf: 41 0f 2e 46 14              	ucomiss	0x14(%r14), %xmm0
141d85ab4: 72 67                       	jb	0x141d85b1d <.text+0x1d84b1d>
141d85ab6: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
141d85abd: e8 1e 00 8a ff              	callq	0x141625ae0 <.text+0x1624ae0>
141d85ac2: f3 0f 59 05 de ea ba 00     	mulss	0xbaeade(%rip), %xmm0   # 0x1429345a8
141d85aca: f3 0f 58 05 b6 fe b9 00     	addss	0xb9feb6(%rip), %xmm0   # 0x142925988
141d85ad2: 45 31 ff                    	xorl	%r15d, %r15d
141d85ad5: e9 05 f8 ff ff              	jmp	0x141d852df <.text+0x1d842df>
141d85ada: f3 41 0f 10 45 04           	movss	0x4(%r13), %xmm0
141d85ae0: f3 41 0f 5c c1              	subss	%xmm9, %xmm0
141d85ae5: f6 03 01                    	testb	$0x1, (%rbx)
141d85ae8: 74 56                       	je	0x141d85b40 <.text+0x1d84b40>
141d85aea: 0f 28 d0                    	movaps	%xmm0, %xmm2
141d85aed: 41 0f c2 d6 02              	cmpleps	%xmm14, %xmm2
141d85af2: 41 0f c2 ce 00              	cmpeqps	%xmm14, %xmm1
141d85af7: 0f 54 ca                    	andps	%xmm2, %xmm1
141d85afa: 66 41 0f 7e cf              	movd	%xmm1, %r15d
141d85aff: 41 83 c7 02                 	addl	$0x2, %r15d
141d85b03: e9 d7 f7 ff ff              	jmp	0x141d852df <.text+0x1d842df>
141d85b08: f3 41 0f 10 46 14           	movss	0x14(%r14), %xmm0
141d85b0e: 41 bf 03 00 00 00           	movl	$0x3, %r15d
141d85b14: 0f 2e 05 b5 fd b9 00        	ucomiss	0xb9fdb5(%rip), %xmm0   # 0x1429258d0
141d85b1b: 73 05                       	jae	0x141d85b22 <.text+0x1d84b22>
141d85b1d: e9 bd f7 ff ff              	jmp	0x141d852df <.text+0x1d842df>
141d85b22: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
141d85b29: e8 b2 ff 89 ff              	callq	0x141625ae0 <.text+0x1624ae0>
141d85b2e: f3 0f 10 0d 52 fe b9 00     	movss	0xb9fe52(%rip), %xmm1   # 0x142925988
141d85b36: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
141d85b3a: f3 0f 58 c1                 	addss	%xmm1, %xmm0
141d85b3e: eb 08                       	jmp	0x141d85b48 <.text+0x1d84b48>
141d85b40: f3 0f 5f 05 40 fe b9 00     	maxss	0xb9fe40(%rip), %xmm0   # 0x142925988
141d85b48: 41 bf 02 00 00 00           	movl	$0x2, %r15d
141d85b4e: e9 8c f7 ff ff              	jmp	0x141d852df <.text+0x1d842df>
141d85b53: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
141d85b5a: 48 85 d2                    	testq	%rdx, %rdx
141d85b5d: 74 16                       	je	0x141d85b75 <.text+0x1d84b75>
141d85b5f: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
141d85b66: 48 c1 e2 03                 	shlq	$0x3, %rdx
141d85b6a: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141d85b70: e8 ab e0 88 fe              	callq	0x140613c20 <.text+0x612c20>
141d85b75: 48 83 bd b8 00 00 00 00     	cmpq	$0x0, 0xb8(%rbp)
141d85b7d: 74 4b                       	je	0x141d85bca <.text+0x1d84bca>
141d85b7f: e8 ac 25 e9 fe              	callq	0x140c18130 <.text+0xc17130>
141d85b84: 48 89 c1                    	movq	%rax, %rcx
141d85b87: 48 8b 00                    	movq	(%rax), %rax
141d85b8a: 48 83 f8 01                 	cmpq	$0x1, %rax
141d85b8e: 0f 85 98 00 00 00           	jne	0x141d85c2c <.text+0x1d84c2c>
141d85b94: 48 83 c1 08                 	addq	$0x8, %rcx
141d85b98: 48 83 39 00                 	cmpq	$0x0, (%rcx)
141d85b9c: 0f 85 c8 00 00 00           	jne	0x141d85c6a <.text+0x1d84c6a>
141d85ba2: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
141d85ba9: 48 89 8d 18 01 00 00        	movq	%rcx, 0x118(%rbp)
141d85bb0: 48 83 c1 08                 	addq	$0x8, %rcx
141d85bb4: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
141d85bbb: e8 30 15 e9 fe              	callq	0x140c170f0 <.text+0xc160f0>
141d85bc0: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
141d85bc7: 48 ff 00                    	incq	(%rax)
141d85bca: 0f 28 b5 60 01 00 00        	movaps	0x160(%rbp), %xmm6
141d85bd1: 0f 28 bd 70 01 00 00        	movaps	0x170(%rbp), %xmm7
141d85bd8: 44 0f 28 85 80 01 00 00     	movaps	0x180(%rbp), %xmm8
141d85be0: 44 0f 28 8d 90 01 00 00     	movaps	0x190(%rbp), %xmm9
141d85be8: 44 0f 28 95 a0 01 00 00     	movaps	0x1a0(%rbp), %xmm10
141d85bf0: 44 0f 28 9d b0 01 00 00     	movaps	0x1b0(%rbp), %xmm11
141d85bf8: 44 0f 28 a5 c0 01 00 00     	movaps	0x1c0(%rbp), %xmm12
141d85c00: 44 0f 28 ad d0 01 00 00     	movaps	0x1d0(%rbp), %xmm13
141d85c08: 44 0f 28 b5 e0 01 00 00     	movaps	0x1e0(%rbp), %xmm14
141d85c10: 44 0f 28 bd f0 01 00 00     	movaps	0x1f0(%rbp), %xmm15
141d85c18: 48 81 c4 88 02 00 00        	addq	$0x288, %rsp            # imm = 0x288
141d85c1f: 5b                          	popq	%rbx
141d85c20: 5f                          	popq	%rdi
141d85c21: 5e                          	popq	%rsi
141d85c22: 41 5c                       	popq	%r12
141d85c24: 41 5d                       	popq	%r13
141d85c26: 41 5e                       	popq	%r14
141d85c28: 41 5f                       	popq	%r15
141d85c2a: 5d                          	popq	%rbp
141d85c2b: c3                          	retq
141d85c2c: 48 85 c0                    	testq	%rax, %rax
141d85c2f: 75 2d                       	jne	0x141d85c5e <.text+0x1d84c5e>
141d85c31: 31 d2                       	xorl	%edx, %edx
141d85c33: e8 f8 53 b9 00              	callq	0x14291b030 <.text+0x291a030>
141d85c38: 48 89 c1                    	movq	%rax, %rcx
141d85c3b: 48 85 c0                    	testq	%rax, %rax
141d85c3e: 0f 85 54 ff ff ff           	jne	0x141d85b98 <.text+0x1d84b98>
141d85c44: eb 18                       	jmp	0x141d85c5e <.text+0x1d84c5e>
141d85c46: 48 85 c0                    	testq	%rax, %rax
141d85c49: 75 13                       	jne	0x141d85c5e <.text+0x1d84c5e>
141d85c4b: 31 d2                       	xorl	%edx, %edx
141d85c4d: e8 de 53 b9 00              	callq	0x14291b030 <.text+0x291a030>
141d85c52: 48 89 c1                    	movq	%rax, %rcx
141d85c55: 48 85 c0                    	testq	%rax, %rax
141d85c58: 0f 85 44 f3 ff ff           	jne	0x141d84fa2 <.text+0x1d83fa2>
141d85c5e: 48 8d 0d 6b 44 2c 01        	leaq	0x12c446b(%rip), %rcx   # 0x14304a0d0
141d85c65: e8 66 24 b5 00              	callq	0x1428d80d0 <.text+0x28d70d0>
141d85c6a: 48 8d 0d cf 44 2c 01        	leaq	0x12c44cf(%rip), %rcx   # 0x14304a140
141d85c71: e8 da 35 b5 00              	callq	0x1428d9250 <.text+0x28d8250>
141d85c76: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
141d85c7e: 4c 89 ad 28 01 00 00        	movq	%r13, 0x128(%rbp)
141d85c85: 48 8d 05 34 46 2c 01        	leaq	0x12c4634(%rip), %rax   # 0x14304a2c0
141d85c8c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141d85c91: 48 8d 95 d8 00 00 00        	leaq	0xd8(%rbp), %rdx
141d85c98: 4c 8d 4d c0                 	leaq	-0x40(%rbp), %r9
141d85c9c: 31 c9                       	xorl	%ecx, %ecx
141d85c9e: 49 89 d8                    	movq	%rbx, %r8
141d85ca1: e8 9b 4e b4 00              	callq	0x1428cab41 <.text+0x28c9b41>
141d85ca6: eb 13                       	jmp	0x141d85cbb <.text+0x1d84cbb>
141d85ca8: 4c 89 ad 28 01 00 00        	movq	%r13, 0x128(%rbp)
141d85caf: 48 8d 0d e2 22 1e 01        	leaq	0x11e22e2(%rip), %rcx   # 0x142f67f98
141d85cb6: e8 55 36 b5 00              	callq	0x1428d9310 <.text+0x28d8310>
141d85cbb: 0f 0b                       	ud2
141d85cbd: 0f 1f 00                    	nopl	(%rax)
141d85cc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141d85cc5: 55                          	pushq	%rbp
141d85cc6: 41 57                       	pushq	%r15
141d85cc8: 41 56                       	pushq	%r14
141d85cca: 41 55                       	pushq	%r13
141d85ccc: 41 54                       	pushq	%r12
141d85cce: 56                          	pushq	%rsi
141d85ccf: 57                          	pushq	%rdi
141d85cd0: 53                          	pushq	%rbx
141d85cd1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141d85cd8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141d85cdf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141d85ce5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141d85ceb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141d85cf1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141d85cf7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141d85cfd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141d85d06: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141d85d0f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141d85d18: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141d85d20: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141d85d28: 48 8b 85 48 01 00 00        	movq	0x148(%rbp), %rax
141d85d2f: 48 ff 00                    	incq	(%rax)
141d85d32: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141d85d3a: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141d85d42: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141d85d4b: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141d85d54: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141d85d5d: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141d85d63: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141d85d69: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141d85d6f: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141d85d75: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141d85d7b: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141d85d82: 5b                          	popq	%rbx
141d85d83: 5f                          	popq	%rdi
141d85d84: 5e                          	popq	%rsi
141d85d85: 41 5c                       	popq	%r12
141d85d87: 41 5d                       	popq	%r13
141d85d89: 41 5e                       	popq	%r14
141d85d8b: 41 5f                       	popq	%r15
141d85d8d: 5d                          	popq	%rbp
141d85d8e: c3                          	retq
141d85d8f: 90                          	nop
141d85d90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141d85d95: 55                          	pushq	%rbp
141d85d96: 41 57                       	pushq	%r15
141d85d98: 41 56                       	pushq	%r14
141d85d9a: 41 55                       	pushq	%r13
141d85d9c: 41 54                       	pushq	%r12
141d85d9e: 56                          	pushq	%rsi
141d85d9f: 57                          	pushq	%rdi
141d85da0: 53                          	pushq	%rbx
141d85da1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141d85da8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141d85daf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141d85db5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141d85dbb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141d85dc1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141d85dc7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141d85dcd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141d85dd6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141d85ddf: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141d85de8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141d85df0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141d85df8: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
141d85dff: 48 ff 00                    	incq	(%rax)
141d85e02: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141d85e0a: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141d85e12: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141d85e1b: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141d85e24: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141d85e2d: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141d85e33: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141d85e39: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141d85e3f: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141d85e45: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141d85e4b: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141d85e52: 5b                          	popq	%rbx
141d85e53: 5f                          	popq	%rdi
141d85e54: 5e                          	popq	%rsi
141d85e55: 41 5c                       	popq	%r12
141d85e57: 41 5d                       	popq	%r13
141d85e59: 41 5e                       	popq	%r14
141d85e5b: 41 5f                       	popq	%r15
141d85e5d: 5d                          	popq	%rbp
141d85e5e: c3                          	retq
141d85e5f: 90                          	nop
141d85e60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141d85e65: 55                          	pushq	%rbp
141d85e66: 41 57                       	pushq	%r15
141d85e68: 41 56                       	pushq	%r14
141d85e6a: 41 55                       	pushq	%r13
141d85e6c: 41 54                       	pushq	%r12
141d85e6e: 56                          	pushq	%rsi
141d85e6f: 57                          	pushq	%rdi
141d85e70: 53                          	pushq	%rbx
141d85e71: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141d85e78: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141d85e7f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141d85e85: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141d85e8b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141d85e91: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141d85e97: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141d85e9d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141d85ea6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141d85eaf: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141d85eb8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141d85ec0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141d85ec8: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
141d85ecf: 48 85 d2                    	testq	%rdx, %rdx
141d85ed2: 74 16                       	je	0x141d85eea <.text+0x1d84eea>
141d85ed4: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
141d85edb: 48 c1 e2 03                 	shlq	$0x3, %rdx
141d85edf: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141d85ee5: e8 36 dd 88 fe              	callq	0x140613c20 <.text+0x612c20>
141d85eea: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
141d85ef1: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
141d85ef8: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141d85f00: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141d85f08: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141d85f11: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141d85f1a: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141d85f23: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141d85f29: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141d85f2f: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141d85f35: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141d85f3b: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141d85f41: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141d85f48: 5b                          	popq	%rbx
141d85f49: 5f                          	popq	%rdi
141d85f4a: 5e                          	popq	%rsi
141d85f4b: 41 5c                       	popq	%r12
141d85f4d: 41 5d                       	popq	%r13
141d85f4f: 41 5e                       	popq	%r14
141d85f51: 41 5f                       	popq	%r15
141d85f53: 5d                          	popq	%rbp
141d85f54: c3                          	retq
141d85f55: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
141d85f60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141d85f65: 55                          	pushq	%rbp
141d85f66: 41 57                       	pushq	%r15
141d85f68: 41 56                       	pushq	%r14
141d85f6a: 41 55                       	pushq	%r13
141d85f6c: 41 54                       	pushq	%r12
141d85f6e: 56                          	pushq	%rsi
141d85f6f: 57                          	pushq	%rdi
141d85f70: 53                          	pushq	%rbx
141d85f71: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141d85f78: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141d85f7f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141d85f85: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141d85f8b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141d85f91: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141d85f97: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141d85f9d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141d85fa6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141d85faf: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141d85fb8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141d85fc0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141d85fc8: 48 83 bd 28 01 00 00 00     	cmpq	$0x0, 0x128(%rbp)
141d85fd0: 74 2f                       	je	0x141d86001 <.text+0x1d85001>
141d85fd2: 48 8d 85 c0 00 00 00        	leaq	0xc0(%rbp), %rax
141d85fd9: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
141d85fe0: 48 8d 85 98 00 00 00        	leaq	0x98(%rbp), %rax
141d85fe7: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
141d85fee: 48 8d 0d b3 1e 1e 01        	leaq	0x11e1eb3(%rip), %rcx   # 0x142f67ea8
141d85ff5: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
141d85ffc: e8 ef 85 58 00              	callq	0x14230e5f0 <.text+0x230d5f0>
141d86001: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141d86009: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141d86011: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141d8601a: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141d86023: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141d8602c: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141d86032: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141d86038: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141d8603e: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141d86044: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141d8604a: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141d86051: 5b                          	popq	%rbx
141d86052: 5f                          	popq	%rdi
141d86053: 5e                          	popq	%rsi
141d86054: 41 5c                       	popq	%r12
141d86056: 41 5d                       	popq	%r13
141d86058: 41 5e                       	popq	%r14
141d8605a: 41 5f                       	popq	%r15
141d8605c: 5d                          	popq	%rbp
141d8605d: c3                          	retq

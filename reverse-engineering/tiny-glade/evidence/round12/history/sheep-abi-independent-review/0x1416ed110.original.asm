
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416ed110: 55                          	pushq	%rbp
1416ed111: 41 57                       	pushq	%r15
1416ed113: 41 56                       	pushq	%r14
1416ed115: 41 55                       	pushq	%r13
1416ed117: 41 54                       	pushq	%r12
1416ed119: 56                          	pushq	%rsi
1416ed11a: 57                          	pushq	%rdi
1416ed11b: 53                          	pushq	%rbx
1416ed11c: 48 81 ec 18 03 00 00        	subq	$0x318, %rsp            # imm = 0x318
1416ed123: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1416ed12b: 48 c7 85 90 02 00 00 fe ff ff ff    	movq	$-0x2, 0x290(%rbp)
1416ed136: 48 89 d3                    	movq	%rdx, %rbx
1416ed139: 48 8b 31                    	movq	(%rcx), %rsi
1416ed13c: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
1416ed140: 31 c0                       	xorl	%eax, %eax
1416ed142: 48 3b 06                    	cmpq	(%rsi), %rax
1416ed145: 0f 80 3b 05 00 00           	jo	0x1416ed686 <.text+0x16ec686>
1416ed14b: 48 8d 15 4b 33 5a 01        	leaq	0x15a334b(%rip), %rdx   # 0x142c9049d
1416ed152: 4c 8d 75 b0                 	leaq	-0x50(%rbp), %r14
1416ed156: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1416ed15c: 4c 89 f1                    	movq	%r14, %rcx
1416ed15f: e8 0c f2 2e ff              	callq	0x1409dc370 <.text+0x9db370>
1416ed164: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ed16b: 4c 89 f2                    	movq	%r14, %rdx
1416ed16e: 49 89 d8                    	movq	%rbx, %r8
1416ed171: e8 aa 9b 55 ff              	callq	0x140c46d20 <.text+0xc45d20>
1416ed176: 80 bd c4 01 00 00 00        	cmpb	$0x0, 0x1c4(%rbp)
1416ed17d: 74 73                       	je	0x1416ed1f2 <.text+0x16ec1f2>
1416ed17f: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1416ed186: f0                          	lock
1416ed187: 48 ff 08                    	decq	(%rax)
1416ed18a: 75 0c                       	jne	0x1416ed198 <.text+0x16ec198>
1416ed18c: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1416ed193: e8 d8 3f 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ed198: 48 8d 15 02 33 5a 01        	leaq	0x15a3302(%rip), %rdx   # 0x142c904a1
1416ed19f: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
1416ed1a6: 41 b8 14 00 00 00           	movl	$0x14, %r8d
1416ed1ac: e8 4f 49 67 ff              	callq	0x140d61b00 <.text+0xd60b00>
1416ed1b1: 4c 8b b5 30 02 00 00        	movq	0x230(%rbp), %r14
1416ed1b8: 4d 85 f6                    	testq	%r14, %r14
1416ed1bb: 0f 88 c0 08 00 00           	js	0x1416eda81 <.text+0x16eca81>
1416ed1c1: 4c 8b bd 28 02 00 00        	movq	0x228(%rbp), %r15
1416ed1c8: 74 6f                       	je	0x1416ed239 <.text+0x16ec239>
1416ed1ca: 0f b6 05 38 7a f4 01        	movzbl	0x1f47a38(%rip), %eax   # 0x143634c09
1416ed1d1: 41 bd 01 00 00 00           	movl	$0x1, %r13d
1416ed1d7: ba 01 00 00 00              	movl	$0x1, %edx
1416ed1dc: 4c 89 f1                    	movq	%r14, %rcx
1416ed1df: e8 2c 6a f2 fe              	callq	0x140613c10 <.text+0x612c10>
1416ed1e4: 48 85 c0                    	testq	%rax, %rax
1416ed1e7: 0f 84 97 08 00 00           	je	0x1416eda84 <.text+0x16eca84>
1416ed1ed: 49 89 c4                    	movq	%rax, %r12
1416ed1f0: eb 4d                       	jmp	0x1416ed23f <.text+0x16ec23f>
1416ed1f2: c6 45 b0 00                 	movb	$0x0, -0x50(%rbp)
1416ed1f6: 80 bd c3 01 00 00 00        	cmpb	$0x0, 0x1c3(%rbp)
1416ed1fd: 74 18                       	je	0x1416ed217 <.text+0x16ec217>
1416ed1ff: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1416ed206: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1416ed20a: e8 61 56 95 fe              	callq	0x140042870 <.text+0x41870>
1416ed20f: 84 c0                       	testb	%al, %al
1416ed211: 0f 85 68 ff ff ff           	jne	0x1416ed17f <.text+0x16ec17f>
1416ed217: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1416ed21e: f0                          	lock
1416ed21f: 48 ff 08                    	decq	(%rax)
1416ed222: 0f 85 5e 04 00 00           	jne	0x1416ed686 <.text+0x16ec686>
1416ed228: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1416ed22f: e8 3c 3f 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ed234: e9 4d 04 00 00              	jmp	0x1416ed686 <.text+0x16ec686>
1416ed239: 41 bc 01 00 00 00           	movl	$0x1, %r12d
1416ed23f: 4c 89 e1                    	movq	%r12, %rcx
1416ed242: 4c 89 fa                    	movq	%r15, %rdx
1416ed245: 4d 89 f0                    	movq	%r14, %r8
1416ed248: e8 b3 69 23 01              	callq	0x142923c00 <.text+0x2922c00>
1416ed24d: 4c 89 75 b0                 	movq	%r14, -0x50(%rbp)
1416ed251: 4c 89 65 b8                 	movq	%r12, -0x48(%rbp)
1416ed255: 4c 89 75 c0                 	movq	%r14, -0x40(%rbp)
1416ed259: 0f b6 85 38 02 00 00        	movzbl	0x238(%rbp), %eax
1416ed260: 88 45 c8                    	movb	%al, -0x38(%rbp)
1416ed263: c6 85 70 01 00 00 01        	movb	$0x1, 0x170(%rbp)
1416ed26a: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1416ed26e: e8 5d 9c 68 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
1416ed273: 49 89 d0                    	movq	%rdx, %r8
1416ed276: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ed27d: 48 89 c2                    	movq	%rax, %rdx
1416ed280: e8 cb ce 66 ff              	callq	0x140d5a150 <.text+0xd59150>
1416ed285: 49 89 c6                    	movq	%rax, %r14
1416ed288: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
1416ed28c: 48 85 d2                    	testq	%rdx, %rdx
1416ed28f: 74 0f                       	je	0x1416ed2a0 <.text+0x16ec2a0>
1416ed291: 48 8b 4d b8                 	movq	-0x48(%rbp), %rcx
1416ed295: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416ed29b: e8 80 69 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ed2a0: 44 89 f0                    	movl	%r14d, %eax
1416ed2a3: 83 e0 03                    	andl	$0x3, %eax
1416ed2a6: 83 f8 01                    	cmpl	$0x1, %eax
1416ed2a9: 75 6b                       	jne	0x1416ed316 <.text+0x16ec316>
1416ed2ab: 4c 89 f0                    	movq	%r14, %rax
1416ed2ae: 48 ff c8                    	decq	%rax
1416ed2b1: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1416ed2b8: 49 8b 46 ff                 	movq	-0x1(%r14), %rax
1416ed2bc: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
1416ed2c3: 49 8b 46 07                 	movq	0x7(%r14), %rax
1416ed2c7: 48 89 85 78 02 00 00        	movq	%rax, 0x278(%rbp)
1416ed2ce: 48 8b 00                    	movq	(%rax), %rax
1416ed2d1: 48 85 c0                    	testq	%rax, %rax
1416ed2d4: 74 09                       	je	0x1416ed2df <.text+0x16ec2df>
1416ed2d6: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
1416ed2dd: ff d0                       	callq	*%rax
1416ed2df: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
1416ed2e6: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1416ed2ed: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1416ed2f1: 48 85 d2                    	testq	%rdx, %rdx
1416ed2f4: 74 09                       	je	0x1416ed2ff <.text+0x16ec2ff>
1416ed2f6: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1416ed2fa: e8 21 69 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ed2ff: ba 18 00 00 00              	movl	$0x18, %edx
1416ed304: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1416ed30a: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1416ed311: e8 0a 69 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ed316: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
1416ed31e: 48 c7 45 b8 08 00 00 00     	movq	$0x8, -0x48(%rbp)
1416ed326: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
1416ed32e: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1416ed338: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
1416ed33c: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1416ed340: 48 89 45 00                 	movq	%rax, (%rbp)
1416ed344: c7 45 18 0d 00 00 00        	movl	$0xd, 0x18(%rbp)
1416ed34b: 48 8d 05 6e 31 5a 01        	leaq	0x15a316e(%rip), %rax   # 0x142c904c0
1416ed352: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416ed357: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1416ed360: 4c 8d 05 4e 31 5a 01        	leaq	0x15a314e(%rip), %r8    # 0x142c904b5
1416ed367: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ed36e: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1416ed372: 41 b9 04 00 00 00           	movl	$0x4, %r9d
1416ed378: e8 53 27 49 ff              	callq	0x140b7fad0 <.text+0xb7ead0>
1416ed37d: c6 85 8f 02 00 00 01        	movb	$0x1, 0x28f(%rbp)
1416ed384: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
1416ed38b: e8 40 9b 68 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
1416ed390: 49 89 d0                    	movq	%rdx, %r8
1416ed393: c6 85 8f 02 00 00 01        	movb	$0x1, 0x28f(%rbp)
1416ed39a: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1416ed39e: 48 89 c2                    	movq	%rax, %rdx
1416ed3a1: e8 8a db 67 ff              	callq	0x140d6af30 <.text+0xd69f30>
1416ed3a6: 31 c0                       	xorl	%eax, %eax
1416ed3a8: 48 3b 45 b0                 	cmpq	-0x50(%rbp), %rax
1416ed3ac: 0f 80 e9 06 00 00           	jo	0x1416eda9b <.text+0x16eca9b>
1416ed3b2: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
1416ed3b6: 0f 10 4d c0                 	movups	-0x40(%rbp), %xmm1
1416ed3ba: 0f 29 8d 50 02 00 00        	movaps	%xmm1, 0x250(%rbp)
1416ed3c1: 0f 29 85 40 02 00 00        	movaps	%xmm0, 0x240(%rbp)
1416ed3c8: c6 85 8f 02 00 00 00        	movb	$0x0, 0x28f(%rbp)
1416ed3cf: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1416ed3d3: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
1416ed3da: 4c 8d b5 40 02 00 00        	leaq	0x240(%rbp), %r14
1416ed3e1: 4d 89 f0                    	movq	%r14, %r8
1416ed3e4: e8 67 98 9e fe              	callq	0x1400d6c50 <.text+0xd5c50>
1416ed3e9: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
1416ed3f0: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1416ed3f4: e8 b7 29 49 ff              	callq	0x140b7fdb0 <.text+0xb7edb0>
1416ed3f9: 31 c0                       	xorl	%eax, %eax
1416ed3fb: 48 3b 85 00 02 00 00        	cmpq	0x200(%rbp), %rax
1416ed402: 0f 80 60 02 00 00           	jo	0x1416ed668 <.text+0x16ec668>
1416ed408: 0f 10 85 00 02 00 00        	movups	0x200(%rbp), %xmm0
1416ed40f: 0f 10 8d 10 02 00 00        	movups	0x210(%rbp), %xmm1
1416ed416: 0f 29 8d 80 01 00 00        	movaps	%xmm1, 0x180(%rbp)
1416ed41d: 0f 29 85 70 01 00 00        	movaps	%xmm0, 0x170(%rbp)
1416ed424: 0f b6 05 de 77 f4 01        	movzbl	0x1f477de(%rip), %eax   # 0x143634c09
1416ed42b: b9 80 00 00 00              	movl	$0x80, %ecx
1416ed430: ba 01 00 00 00              	movl	$0x1, %edx
1416ed435: e8 d6 67 f2 fe              	callq	0x140613c10 <.text+0x612c10>
1416ed43a: 48 85 c0                    	testq	%rax, %rax
1416ed43d: 0f 84 90 06 00 00           	je	0x1416edad3 <.text+0x16ecad3>
1416ed443: 48 c7 85 40 02 00 00 80 00 00 00    	movq	$0x80, 0x240(%rbp)
1416ed44e: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
1416ed455: 48 c7 85 50 02 00 00 00 00 00 00    	movq	$0x0, 0x250(%rbp)
1416ed460: 4c 89 75 b0                 	movq	%r14, -0x50(%rbp)
1416ed464: 48 8d 05 b5 2a 5a 01        	leaq	0x15a2ab5(%rip), %rax   # 0x142c8ff20
1416ed46b: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
1416ed46f: 48 c7 45 c0 02 00 00 00     	movq	$0x2, -0x40(%rbp)
1416ed477: 48 c7 45 c8 00 00 00 00     	movq	$0x0, -0x38(%rbp)
1416ed47f: c6 45 d0 00                 	movb	$0x0, -0x30(%rbp)
1416ed483: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1416ed487: 48 89 f1                    	movq	%rsi, %rcx
1416ed48a: e8 51 cd 00 00              	callq	0x1416fa1e0 <.text+0x16f91e0>
1416ed48f: 48 85 c0                    	testq	%rax, %rax
1416ed492: 0f 85 53 06 00 00           	jne	0x1416edaeb <.text+0x16ecaeb>
1416ed498: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
1416ed49f: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
1416ed4a6: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
1416ed4ad: 48 89 8d 78 02 00 00        	movq	%rcx, 0x278(%rbp)
1416ed4b4: 48 89 c8                    	movq	%rcx, %rax
1416ed4b7: 48 f7 d8                    	negq	%rax
1416ed4ba: 0f 80 50 06 00 00           	jo	0x1416edb10 <.text+0x16ecb10>
1416ed4c0: 4c 8b a5 50 02 00 00        	movq	0x250(%rbp), %r12
1416ed4c7: 4c 8d bd 70 01 00 00        	leaq	0x170(%rbp), %r15
1416ed4ce: 4c 89 f9                    	movq	%r15, %rcx
1416ed4d1: e8 fa 99 68 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
1416ed4d6: 48 89 c1                    	movq	%rax, %rcx
1416ed4d9: 4c 8b 85 80 02 00 00        	movq	0x280(%rbp), %r8
1416ed4e0: 4d 89 e1                    	movq	%r12, %r9
1416ed4e3: e8 48 bf 66 ff              	callq	0x140d59430 <.text+0xd58430>
1416ed4e8: 49 89 c4                    	movq	%rax, %r12
1416ed4eb: 48 8b 95 78 02 00 00        	movq	0x278(%rbp), %rdx
1416ed4f2: 48 85 d2                    	testq	%rdx, %rdx
1416ed4f5: 74 12                       	je	0x1416ed509 <.text+0x16ec509>
1416ed4f7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416ed4fd: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
1416ed504: e8 17 67 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ed509: 4d 85 e4                    	testq	%r12, %r12
1416ed50c: 0f 84 81 00 00 00           	je	0x1416ed593 <.text+0x16ec593>
1416ed512: 44 89 e0                    	movl	%r12d, %eax
1416ed515: 83 e0 03                    	andl	$0x3, %eax
1416ed518: 83 f8 01                    	cmpl	$0x1, %eax
1416ed51b: 0f 85 29 01 00 00           	jne	0x1416ed64a <.text+0x16ec64a>
1416ed521: 4c 89 e0                    	movq	%r12, %rax
1416ed524: 48 ff c8                    	decq	%rax
1416ed527: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1416ed52e: 49 8b 44 24 ff              	movq	-0x1(%r12), %rax
1416ed533: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
1416ed53a: 49 8b 44 24 07              	movq	0x7(%r12), %rax
1416ed53f: 48 89 85 78 02 00 00        	movq	%rax, 0x278(%rbp)
1416ed546: 48 8b 00                    	movq	(%rax), %rax
1416ed549: 48 85 c0                    	testq	%rax, %rax
1416ed54c: 74 09                       	je	0x1416ed557 <.text+0x16ec557>
1416ed54e: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
1416ed555: ff d0                       	callq	*%rax
1416ed557: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
1416ed55e: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1416ed565: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1416ed569: 48 85 d2                    	testq	%rdx, %rdx
1416ed56c: 74 09                       	je	0x1416ed577 <.text+0x16ec577>
1416ed56e: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1416ed572: e8 a9 66 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ed577: ba 18 00 00 00              	movl	$0x18, %edx
1416ed57c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1416ed582: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1416ed589: e8 92 66 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ed58e: e9 b7 00 00 00              	jmp	0x1416ed64a <.text+0x16ec64a>
1416ed593: 48 8b 05 0e ee f3 01        	movq	0x1f3ee0e(%rip), %rax   # 0x14362c3a8
1416ed59a: 48 8b 00                    	movq	(%rax), %rax
1416ed59d: 48 83 f8 03                 	cmpq	$0x3, %rax
1416ed5a1: 0f 82 a3 00 00 00           	jb	0x1416ed64a <.text+0x16ec64a>
1416ed5a7: 4c 89 bd 40 02 00 00        	movq	%r15, 0x240(%rbp)
1416ed5ae: 48 8d 05 2b 45 67 ff        	leaq	-0x98bad5(%rip), %rax   # 0x140d61ae0 <.text+0xd60ae0>
1416ed5b5: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
1416ed5bc: 48 8d 0d 65 2f 5a 01        	leaq	0x15a2f65(%rip), %rcx   # 0x142c90528
1416ed5c3: e8 b8 07 f9 fe              	callq	0x14067dd80 <.text+0x67cd80>
1416ed5c8: 0f 10 00                    	movups	(%rax), %xmm0
1416ed5cb: 8b 40 10                    	movl	0x10(%rax), %eax
1416ed5ce: 48 c7 45 e0 03 00 00 00     	movq	$0x3, -0x20(%rbp)
1416ed5d6: 48 8d 0d 43 2e 5a 01        	leaq	0x15a2e43(%rip), %rcx   # 0x142c90420
1416ed5dd: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
1416ed5e1: 48 c7 45 f0 2c 00 00 00     	movq	$0x2c, -0x10(%rbp)
1416ed5e9: 48 8d 15 28 2f 5a 01        	leaq	0x15a2f28(%rip), %rdx   # 0x142c90518
1416ed5f0: 48 89 55 00                 	movq	%rdx, (%rbp)
1416ed5f4: 48 c7 45 08 01 00 00 00     	movq	$0x1, 0x8(%rbp)
1416ed5fc: 4c 89 75 10                 	movq	%r14, 0x10(%rbp)
1416ed600: 48 c7 45 18 01 00 00 00     	movq	$0x1, 0x18(%rbp)
1416ed608: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
1416ed610: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
1416ed618: 48 89 4d b8                 	movq	%rcx, -0x48(%rbp)
1416ed61c: 48 c7 45 c0 2c 00 00 00     	movq	$0x2c, -0x40(%rbp)
1416ed624: 48 c7 45 c8 00 00 00 00     	movq	$0x0, -0x38(%rbp)
1416ed62c: 0f 11 45 d0                 	movups	%xmm0, -0x30(%rbp)
1416ed630: c7 45 f8 01 00 00 00        	movl	$0x1, -0x8(%rbp)
1416ed637: 89 45 fc                    	movl	%eax, -0x4(%rbp)
1416ed63a: 48 8d 8d 6f 02 00 00        	leaq	0x26f(%rbp), %rcx
1416ed641: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1416ed645: e8 e6 cc 65 ff              	callq	0x140d4a330 <.text+0xd49330>
1416ed64a: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1416ed651: 48 85 d2                    	testq	%rdx, %rdx
1416ed654: 74 12                       	je	0x1416ed668 <.text+0x16ec668>
1416ed656: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
1416ed65d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416ed663: e8 b8 65 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ed668: 48 8b 95 20 02 00 00        	movq	0x220(%rbp), %rdx
1416ed66f: 48 85 d2                    	testq	%rdx, %rdx
1416ed672: 74 12                       	je	0x1416ed686 <.text+0x16ec686>
1416ed674: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
1416ed67b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416ed681: e8 9a 65 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ed686: 48 8d 15 b3 2e 5a 01        	leaq	0x15a2eb3(%rip), %rdx   # 0x142c90540
1416ed68d: 4c 8d 75 b0                 	leaq	-0x50(%rbp), %r14
1416ed691: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1416ed697: 4c 89 f1                    	movq	%r14, %rcx
1416ed69a: e8 d1 ec 2e ff              	callq	0x1409dc370 <.text+0x9db370>
1416ed69f: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ed6a6: 4c 89 f2                    	movq	%r14, %rdx
1416ed6a9: 49 89 d8                    	movq	%rbx, %r8
1416ed6ac: e8 6f 96 55 ff              	callq	0x140c46d20 <.text+0xc45d20>
1416ed6b1: 80 bd c4 01 00 00 00        	cmpb	$0x0, 0x1c4(%rbp)
1416ed6b8: 0f 84 fb 01 00 00           	je	0x1416ed8b9 <.text+0x16ec8b9>
1416ed6be: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1416ed6c5: f0                          	lock
1416ed6c6: 48 ff 08                    	decq	(%rax)
1416ed6c9: 75 0c                       	jne	0x1416ed6d7 <.text+0x16ec6d7>
1416ed6cb: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1416ed6d2: e8 99 3a 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ed6d7: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
1416ed6df: 48 c7 45 b8 08 00 00 00     	movq	$0x8, -0x48(%rbp)
1416ed6e7: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
1416ed6ef: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1416ed6f9: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
1416ed6fd: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1416ed701: 48 89 45 00                 	movq	%rax, (%rbp)
1416ed705: c7 45 18 0d 00 00 00        	movl	$0xd, 0x18(%rbp)
1416ed70c: 48 8d 05 ad 2d 5a 01        	leaq	0x15a2dad(%rip), %rax   # 0x142c904c0
1416ed713: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416ed718: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1416ed721: 4c 8d 05 8d 2d 5a 01        	leaq	0x15a2d8d(%rip), %r8    # 0x142c904b5
1416ed728: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ed72f: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1416ed733: 41 b9 04 00 00 00           	movl	$0x4, %r9d
1416ed739: e8 92 23 49 ff              	callq	0x140b7fad0 <.text+0xb7ead0>
1416ed73e: 48 8d 15 5c 2d 5a 01        	leaq	0x15a2d5c(%rip), %rdx   # 0x142c904a1
1416ed745: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1416ed74c: 41 b8 14 00 00 00           	movl	$0x14, %r8d
1416ed752: e8 a9 43 67 ff              	callq	0x140d61b00 <.text+0xd60b00>
1416ed757: c6 85 8e 02 00 00 01        	movb	$0x1, 0x28e(%rbp)
1416ed75e: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1416ed765: e8 66 97 68 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
1416ed76a: 49 89 d0                    	movq	%rdx, %r8
1416ed76d: c6 85 8e 02 00 00 01        	movb	$0x1, 0x28e(%rbp)
1416ed774: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1416ed778: 48 89 c2                    	movq	%rax, %rdx
1416ed77b: e8 b0 d7 67 ff              	callq	0x140d6af30 <.text+0xd69f30>
1416ed780: 31 c0                       	xorl	%eax, %eax
1416ed782: 48 3b 45 b0                 	cmpq	-0x50(%rbp), %rax
1416ed786: 0f 80 ba 02 00 00           	jo	0x1416eda46 <.text+0x16eca46>
1416ed78c: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
1416ed790: 0f 10 4d c0                 	movups	-0x40(%rbp), %xmm1
1416ed794: 0f 29 8d 10 02 00 00        	movaps	%xmm1, 0x210(%rbp)
1416ed79b: 0f 29 85 00 02 00 00        	movaps	%xmm0, 0x200(%rbp)
1416ed7a2: c6 85 8e 02 00 00 00        	movb	$0x0, 0x28e(%rbp)
1416ed7a9: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1416ed7ad: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
1416ed7b4: 4c 8d 85 00 02 00 00        	leaq	0x200(%rbp), %r8
1416ed7bb: e8 90 94 9e fe              	callq	0x1400d6c50 <.text+0xd5c50>
1416ed7c0: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
1416ed7c7: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1416ed7cb: e8 00 25 49 ff              	callq	0x140b7fcd0 <.text+0xb7ecd0>
1416ed7d0: 31 c0                       	xorl	%eax, %eax
1416ed7d2: 48 3b 85 20 02 00 00        	cmpq	0x220(%rbp), %rax
1416ed7d9: 0f 80 34 02 00 00           	jo	0x1416eda13 <.text+0x16eca13>
1416ed7df: 0f 10 85 20 02 00 00        	movups	0x220(%rbp), %xmm0
1416ed7e6: 0f 10 8d 30 02 00 00        	movups	0x230(%rbp), %xmm1
1416ed7ed: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
1416ed7f1: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1416ed7f5: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1416ed7f9: e8 d2 96 68 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
1416ed7fe: 49 89 d0                    	movq	%rdx, %r8
1416ed801: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ed808: 48 89 c2                    	movq	%rax, %rdx
1416ed80b: e8 30 b9 66 ff              	callq	0x140d59140 <.text+0xd58140>
1416ed810: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
1416ed814: 48 85 d2                    	testq	%rdx, %rdx
1416ed817: 74 0f                       	je	0x1416ed828 <.text+0x16ec828>
1416ed819: 48 8b 4d b8                 	movq	-0x48(%rbp), %rcx
1416ed81d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416ed823: e8 f8 63 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ed828: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
1416ed82f: 48 89 c8                    	movq	%rcx, %rax
1416ed832: 48 f7 d8                    	negq	%rax
1416ed835: 0f 81 c5 00 00 00           	jno	0x1416ed900 <.text+0x16ec900>
1416ed83b: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
1416ed842: 89 c1                       	movl	%eax, %ecx
1416ed844: 83 e1 03                    	andl	$0x3, %ecx
1416ed847: 83 f9 01                    	cmpl	$0x1, %ecx
1416ed84a: 0f 85 c3 01 00 00           	jne	0x1416eda13 <.text+0x16eca13>
1416ed850: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
1416ed854: 48 89 8d 70 02 00 00        	movq	%rcx, 0x270(%rbp)
1416ed85b: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
1416ed85f: 48 89 8d 80 02 00 00        	movq	%rcx, 0x280(%rbp)
1416ed866: 48 8b 40 07                 	movq	0x7(%rax), %rax
1416ed86a: 48 89 85 78 02 00 00        	movq	%rax, 0x278(%rbp)
1416ed871: 48 8b 00                    	movq	(%rax), %rax
1416ed874: 48 85 c0                    	testq	%rax, %rax
1416ed877: 74 09                       	je	0x1416ed882 <.text+0x16ec882>
1416ed879: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
1416ed880: ff d0                       	callq	*%rax
1416ed882: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
1416ed889: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1416ed890: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1416ed894: 48 85 d2                    	testq	%rdx, %rdx
1416ed897: 74 09                       	je	0x1416ed8a2 <.text+0x16ec8a2>
1416ed899: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1416ed89d: e8 7e 63 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ed8a2: ba 18 00 00 00              	movl	$0x18, %edx
1416ed8a7: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1416ed8ad: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1416ed8b4: e9 55 01 00 00              	jmp	0x1416eda0e <.text+0x16eca0e>
1416ed8b9: c6 45 b0 00                 	movb	$0x0, -0x50(%rbp)
1416ed8bd: 80 bd c3 01 00 00 00        	cmpb	$0x0, 0x1c3(%rbp)
1416ed8c4: 74 18                       	je	0x1416ed8de <.text+0x16ec8de>
1416ed8c6: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1416ed8cd: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1416ed8d1: e8 9a 4f 95 fe              	callq	0x140042870 <.text+0x41870>
1416ed8d6: 84 c0                       	testb	%al, %al
1416ed8d8: 0f 85 e0 fd ff ff           	jne	0x1416ed6be <.text+0x16ec6be>
1416ed8de: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1416ed8e5: f0                          	lock
1416ed8e6: 48 ff 08                    	decq	(%rax)
1416ed8e9: 0f 85 42 01 00 00           	jne	0x1416eda31 <.text+0x16eca31>
1416ed8ef: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1416ed8f6: e8 75 38 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ed8fb: e9 31 01 00 00              	jmp	0x1416eda31 <.text+0x16eca31>
1416ed900: 48 89 8d 80 02 00 00        	movq	%rcx, 0x280(%rbp)
1416ed907: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1416ed90e: 4c 8b 85 80 01 00 00        	movq	0x180(%rbp), %r8
1416ed915: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
1416ed91c: 48 89 95 78 02 00 00        	movq	%rdx, 0x278(%rbp)
1416ed923: e8 38 75 d1 ff              	callq	0x141404e60 <.text+0x1403e60>
1416ed928: 31 c0                       	xorl	%eax, %eax
1416ed92a: 48 3b 85 58 01 00 00        	cmpq	0x158(%rbp), %rax
1416ed931: 71 4c                       	jno	0x1416ed97f <.text+0x16ec97f>
1416ed933: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
1416ed93a: 48 8b 01                    	movq	(%rcx), %rax
1416ed93d: 48 83 f8 01                 	cmpq	$0x1, %rax
1416ed941: 48 89 8d 70 02 00 00        	movq	%rcx, 0x270(%rbp)
1416ed948: 0f 84 87 00 00 00           	je	0x1416ed9d5 <.text+0x16ec9d5>
1416ed94e: 48 85 c0                    	testq	%rax, %rax
1416ed951: 0f 85 87 00 00 00           	jne	0x1416ed9de <.text+0x16ec9de>
1416ed957: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
1416ed95e: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1416ed962: 48 85 d2                    	testq	%rdx, %rdx
1416ed965: 74 77                       	je	0x1416ed9de <.text+0x16ec9de>
1416ed967: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
1416ed96e: 48 8b 48 08                 	movq	0x8(%rax), %rcx
1416ed972: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416ed978: e8 a3 62 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ed97d: eb 5f                       	jmp	0x1416ed9de <.text+0x16ec9de>
1416ed97f: 48 8d 95 58 01 00 00        	leaq	0x158(%rbp), %rdx
1416ed986: 48 89 f9                    	movq	%rdi, %rcx
1416ed989: e8 62 4b 00 00              	callq	0x1416f24f0 <.text+0x16f14f0>
1416ed98e: 48 8b 06                    	movq	(%rsi), %rax
1416ed991: 48 89 c1                    	movq	%rax, %rcx
1416ed994: 48 f7 d9                    	negq	%rcx
1416ed997: 70 19                       	jo	0x1416ed9b2 <.text+0x16ec9b2>
1416ed999: 73 17                       	jae	0x1416ed9b2 <.text+0x16ec9b2>
1416ed99b: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
1416ed99f: 48 c1 e0 03                 	shlq	$0x3, %rax
1416ed9a3: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1416ed9a7: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1416ed9ad: e8 6e 62 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ed9b2: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
1416ed9b9: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
1416ed9bd: 0f 10 85 58 01 00 00        	movups	0x158(%rbp), %xmm0
1416ed9c4: 0f 11 06                    	movups	%xmm0, (%rsi)
1416ed9c7: 48 8b 95 80 02 00 00        	movq	0x280(%rbp), %rdx
1416ed9ce: 48 85 d2                    	testq	%rdx, %rdx
1416ed9d1: 75 2e                       	jne	0x1416eda01 <.text+0x16eca01>
1416ed9d3: eb 3e                       	jmp	0x1416eda13 <.text+0x16eca13>
1416ed9d5: 48 83 c1 08                 	addq	$0x8, %rcx
1416ed9d9: e8 22 a0 ef fe              	callq	0x1405e7a00 <.text+0x5e6a00>
1416ed9de: ba 28 00 00 00              	movl	$0x28, %edx
1416ed9e3: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1416ed9e9: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1416ed9f0: e8 2b 62 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ed9f5: 48 8b 95 80 02 00 00        	movq	0x280(%rbp), %rdx
1416ed9fc: 48 85 d2                    	testq	%rdx, %rdx
1416ed9ff: 74 12                       	je	0x1416eda13 <.text+0x16eca13>
1416eda01: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416eda07: 48 8b 8d 78 02 00 00        	movq	0x278(%rbp), %rcx
1416eda0e: e8 0d 62 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416eda13: 48 8b 95 40 02 00 00        	movq	0x240(%rbp), %rdx
1416eda1a: 48 85 d2                    	testq	%rdx, %rdx
1416eda1d: 74 12                       	je	0x1416eda31 <.text+0x16eca31>
1416eda1f: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
1416eda26: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416eda2c: e8 ef 61 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416eda31: 90                          	nop
1416eda32: 48 81 c4 18 03 00 00        	addq	$0x318, %rsp            # imm = 0x318
1416eda39: 5b                          	popq	%rbx
1416eda3a: 5f                          	popq	%rdi
1416eda3b: 5e                          	popq	%rsi
1416eda3c: 41 5c                       	popq	%r12
1416eda3e: 41 5d                       	popq	%r13
1416eda40: 41 5e                       	popq	%r14
1416eda42: 41 5f                       	popq	%r15
1416eda44: 5d                          	popq	%rbp
1416eda45: c3                          	retq
1416eda46: 48 8b 45 b8                 	movq	-0x48(%rbp), %rax
1416eda4a: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1416eda51: 48 8d 05 f0 2a 5a 01        	leaq	0x15a2af0(%rip), %rax   # 0x142c90548
1416eda58: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416eda5d: 48 8d 0d 4c 25 5a 01        	leaq	0x15a254c(%rip), %rcx   # 0x142c8ffb0
1416eda64: 4c 8d 0d 75 25 5a 01        	leaq	0x15a2575(%rip), %r9    # 0x142c8ffe0
1416eda6b: 4c 8d 85 00 02 00 00        	leaq	0x200(%rbp), %r8
1416eda72: ba 2b 00 00 00              	movl	$0x2b, %edx
1416eda77: e8 e4 bc 1e 01              	callq	0x1428d9760 <.text+0x28d8760>
1416eda7c: e9 c2 00 00 00              	jmp	0x1416edb43 <.text+0x16ecb43>
1416eda81: 45 31 ed                    	xorl	%r13d, %r13d
1416eda84: 4c 8d 05 ed b0 54 01        	leaq	0x154b0ed(%rip), %r8    # 0x142c38b78
1416eda8b: 4c 89 e9                    	movq	%r13, %rcx
1416eda8e: 4c 89 f2                    	movq	%r14, %rdx
1416eda91: e8 4d b5 1e 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1416eda96: e9 a8 00 00 00              	jmp	0x1416edb43 <.text+0x16ecb43>
1416eda9b: 48 8b 45 b8                 	movq	-0x48(%rbp), %rax
1416eda9f: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1416edaa6: 48 8d 05 23 2a 5a 01        	leaq	0x15a2a23(%rip), %rax   # 0x142c904d0
1416edaad: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416edab2: 48 8d 0d f7 24 5a 01        	leaq	0x15a24f7(%rip), %rcx   # 0x142c8ffb0
1416edab9: 4c 8d 0d 20 25 5a 01        	leaq	0x15a2520(%rip), %r9    # 0x142c8ffe0
1416edac0: 4c 8d 85 40 02 00 00        	leaq	0x240(%rbp), %r8
1416edac7: ba 2b 00 00 00              	movl	$0x2b, %edx
1416edacc: e8 8f bc 1e 01              	callq	0x1428d9760 <.text+0x28d8760>
1416edad1: eb 70                       	jmp	0x1416edb43 <.text+0x16ecb43>
1416edad3: 4c 8d 05 2e 24 5a 01        	leaq	0x15a242e(%rip), %r8    # 0x142c8ff08
1416edada: b9 01 00 00 00              	movl	$0x1, %ecx
1416edadf: ba 80 00 00 00              	movl	$0x80, %edx
1416edae4: e8 fa b4 1e 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1416edae9: eb 58                       	jmp	0x1416edb43 <.text+0x16ecb43>
1416edaeb: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
1416edaf2: 48 8b 95 40 02 00 00        	movq	0x240(%rbp), %rdx
1416edaf9: 48 85 d2                    	testq	%rdx, %rdx
1416edafc: 74 12                       	je	0x1416edb10 <.text+0x16ecb10>
1416edafe: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
1416edb05: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416edb0b: e8 10 61 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416edb10: 48 8b 85 80 02 00 00        	movq	0x280(%rbp), %rax
1416edb17: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
1416edb1b: 48 8d 05 c6 29 5a 01        	leaq	0x15a29c6(%rip), %rax   # 0x142c904e8
1416edb22: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416edb27: 48 8d 0d 82 24 5a 01        	leaq	0x15a2482(%rip), %rcx   # 0x142c8ffb0
1416edb2e: 4c 8d 0d cb 24 5a 01        	leaq	0x15a24cb(%rip), %r9    # 0x142c90000
1416edb35: 4c 8d 45 b0                 	leaq	-0x50(%rbp), %r8
1416edb39: ba 2b 00 00 00              	movl	$0x2b, %edx
1416edb3e: e8 1d bc 1e 01              	callq	0x1428d9760 <.text+0x28d8760>
1416edb43: 0f 0b                       	ud2
1416edb45: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1416edb50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416edb55: 55                          	pushq	%rbp
1416edb56: 41 57                       	pushq	%r15
1416edb58: 41 56                       	pushq	%r14
1416edb5a: 41 55                       	pushq	%r13
1416edb5c: 41 54                       	pushq	%r12
1416edb5e: 56                          	pushq	%rsi
1416edb5f: 57                          	pushq	%rdi
1416edb60: 53                          	pushq	%rbx
1416edb61: 48 83 ec 38                 	subq	$0x38, %rsp
1416edb65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416edb6c: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1416edb73: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1416edb7a: f0                          	lock
1416edb7b: 48 ff 08                    	decq	(%rax)
1416edb7e: 75 05                       	jne	0x1416edb85 <.text+0x16ecb85>
1416edb80: e8 eb 35 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416edb85: 90                          	nop
1416edb86: 48 83 c4 38                 	addq	$0x38, %rsp
1416edb8a: 5b                          	popq	%rbx
1416edb8b: 5f                          	popq	%rdi
1416edb8c: 5e                          	popq	%rsi
1416edb8d: 41 5c                       	popq	%r12
1416edb8f: 41 5d                       	popq	%r13
1416edb91: 41 5e                       	popq	%r14
1416edb93: 41 5f                       	popq	%r15
1416edb95: 5d                          	popq	%rbp
1416edb96: c3                          	retq
1416edb97: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1416edba0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416edba5: 55                          	pushq	%rbp
1416edba6: 41 57                       	pushq	%r15
1416edba8: 41 56                       	pushq	%r14
1416edbaa: 41 55                       	pushq	%r13
1416edbac: 41 54                       	pushq	%r12
1416edbae: 56                          	pushq	%rsi
1416edbaf: 57                          	pushq	%rdi
1416edbb0: 53                          	pushq	%rbx
1416edbb1: 48 83 ec 38                 	subq	$0x38, %rsp
1416edbb5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416edbbc: 48 8b 95 20 02 00 00        	movq	0x220(%rbp), %rdx
1416edbc3: 48 85 d2                    	testq	%rdx, %rdx
1416edbc6: 74 12                       	je	0x1416edbda <.text+0x16ecbda>
1416edbc8: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
1416edbcf: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416edbd5: e8 46 60 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416edbda: 90                          	nop
1416edbdb: 48 83 c4 38                 	addq	$0x38, %rsp
1416edbdf: 5b                          	popq	%rbx
1416edbe0: 5f                          	popq	%rdi
1416edbe1: 5e                          	popq	%rsi
1416edbe2: 41 5c                       	popq	%r12
1416edbe4: 41 5d                       	popq	%r13
1416edbe6: 41 5e                       	popq	%r14
1416edbe8: 41 5f                       	popq	%r15
1416edbea: 5d                          	popq	%rbp
1416edbeb: c3                          	retq
1416edbec: 0f 1f 40 00                 	nopl	(%rax)
1416edbf0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416edbf5: 55                          	pushq	%rbp
1416edbf6: 41 57                       	pushq	%r15
1416edbf8: 41 56                       	pushq	%r14
1416edbfa: 41 55                       	pushq	%r13
1416edbfc: 41 54                       	pushq	%r12
1416edbfe: 56                          	pushq	%rsi
1416edbff: 57                          	pushq	%rdi
1416edc00: 53                          	pushq	%rbx
1416edc01: 48 83 ec 38                 	subq	$0x38, %rsp
1416edc05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416edc0c: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
1416edc10: 48 85 d2                    	testq	%rdx, %rdx
1416edc13: 74 0f                       	je	0x1416edc24 <.text+0x16ecc24>
1416edc15: 48 8b 4d b8                 	movq	-0x48(%rbp), %rcx
1416edc19: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416edc1f: e8 fc 5f f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416edc24: 90                          	nop
1416edc25: 48 83 c4 38                 	addq	$0x38, %rsp
1416edc29: 5b                          	popq	%rbx
1416edc2a: 5f                          	popq	%rdi
1416edc2b: 5e                          	popq	%rsi
1416edc2c: 41 5c                       	popq	%r12
1416edc2e: 41 5d                       	popq	%r13
1416edc30: 41 5e                       	popq	%r14
1416edc32: 41 5f                       	popq	%r15
1416edc34: 5d                          	popq	%rbp
1416edc35: c3                          	retq
1416edc36: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1416edc40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416edc45: 55                          	pushq	%rbp
1416edc46: 41 57                       	pushq	%r15
1416edc48: 41 56                       	pushq	%r14
1416edc4a: 41 55                       	pushq	%r13
1416edc4c: 41 54                       	pushq	%r12
1416edc4e: 56                          	pushq	%rsi
1416edc4f: 57                          	pushq	%rdi
1416edc50: 53                          	pushq	%rbx
1416edc51: 48 83 ec 38                 	subq	$0x38, %rsp
1416edc55: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416edc5c: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
1416edc63: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1416edc6a: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1416edc6e: 48 85 d2                    	testq	%rdx, %rdx
1416edc71: 74 10                       	je	0x1416edc83 <.text+0x16ecc83>
1416edc73: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1416edc7a: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1416edc7e: e8 9d 5f f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416edc83: ba 18 00 00 00              	movl	$0x18, %edx
1416edc88: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1416edc8e: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1416edc95: e8 86 5f f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416edc9a: 90                          	nop
1416edc9b: 48 83 c4 38                 	addq	$0x38, %rsp
1416edc9f: 5b                          	popq	%rbx
1416edca0: 5f                          	popq	%rdi
1416edca1: 5e                          	popq	%rsi
1416edca2: 41 5c                       	popq	%r12
1416edca4: 41 5d                       	popq	%r13
1416edca6: 41 5e                       	popq	%r14
1416edca8: 41 5f                       	popq	%r15
1416edcaa: 5d                          	popq	%rbp
1416edcab: c3                          	retq
1416edcac: 0f 1f 40 00                 	nopl	(%rax)
1416edcb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416edcb5: 55                          	pushq	%rbp
1416edcb6: 41 57                       	pushq	%r15
1416edcb8: 41 56                       	pushq	%r14
1416edcba: 41 55                       	pushq	%r13
1416edcbc: 41 54                       	pushq	%r12
1416edcbe: 56                          	pushq	%rsi
1416edcbf: 57                          	pushq	%rdi
1416edcc0: 53                          	pushq	%rbx
1416edcc1: 48 83 ec 38                 	subq	$0x38, %rsp
1416edcc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416edccc: c6 85 8f 02 00 00 01        	movb	$0x1, 0x28f(%rbp)
1416edcd3: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1416edcda: e8 21 9d ef fe              	callq	0x1405e7a00 <.text+0x5e6a00>
1416edcdf: c6 85 8f 02 00 00 01        	movb	$0x1, 0x28f(%rbp)
1416edce6: 48 83 c4 38                 	addq	$0x38, %rsp
1416edcea: 5b                          	popq	%rbx
1416edceb: 5f                          	popq	%rdi
1416edcec: 5e                          	popq	%rsi
1416edced: 41 5c                       	popq	%r12
1416edcef: 41 5d                       	popq	%r13
1416edcf1: 41 5e                       	popq	%r14
1416edcf3: 41 5f                       	popq	%r15
1416edcf5: 5d                          	popq	%rbp
1416edcf6: c3                          	retq
1416edcf7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1416edd00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416edd05: 55                          	pushq	%rbp
1416edd06: 41 57                       	pushq	%r15
1416edd08: 41 56                       	pushq	%r14
1416edd0a: 41 55                       	pushq	%r13
1416edd0c: 41 54                       	pushq	%r12
1416edd0e: 56                          	pushq	%rsi
1416edd0f: 57                          	pushq	%rdi
1416edd10: 53                          	pushq	%rbx
1416edd11: 48 83 ec 38                 	subq	$0x38, %rsp
1416edd15: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416edd1c: 48 8b 95 40 02 00 00        	movq	0x240(%rbp), %rdx
1416edd23: 48 85 d2                    	testq	%rdx, %rdx
1416edd26: 74 12                       	je	0x1416edd3a <.text+0x16ecd3a>
1416edd28: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
1416edd2f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416edd35: e8 e6 5e f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416edd3a: 90                          	nop
1416edd3b: 48 83 c4 38                 	addq	$0x38, %rsp
1416edd3f: 5b                          	popq	%rbx
1416edd40: 5f                          	popq	%rdi
1416edd41: 5e                          	popq	%rsi
1416edd42: 41 5c                       	popq	%r12
1416edd44: 41 5d                       	popq	%r13
1416edd46: 41 5e                       	popq	%r14
1416edd48: 41 5f                       	popq	%r15
1416edd4a: 5d                          	popq	%rbp
1416edd4b: c3                          	retq
1416edd4c: 0f 1f 40 00                 	nopl	(%rax)
1416edd50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416edd55: 55                          	pushq	%rbp
1416edd56: 41 57                       	pushq	%r15
1416edd58: 41 56                       	pushq	%r14
1416edd5a: 41 55                       	pushq	%r13
1416edd5c: 41 54                       	pushq	%r12
1416edd5e: 56                          	pushq	%rsi
1416edd5f: 57                          	pushq	%rdi
1416edd60: 53                          	pushq	%rbx
1416edd61: 48 83 ec 38                 	subq	$0x38, %rsp
1416edd65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416edd6c: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1416edd73: 48 85 d2                    	testq	%rdx, %rdx
1416edd76: 74 12                       	je	0x1416edd8a <.text+0x16ecd8a>
1416edd78: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
1416edd7f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416edd85: e8 96 5e f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416edd8a: 90                          	nop
1416edd8b: 48 83 c4 38                 	addq	$0x38, %rsp
1416edd8f: 5b                          	popq	%rbx
1416edd90: 5f                          	popq	%rdi
1416edd91: 5e                          	popq	%rsi
1416edd92: 41 5c                       	popq	%r12
1416edd94: 41 5d                       	popq	%r13
1416edd96: 41 5e                       	popq	%r14
1416edd98: 41 5f                       	popq	%r15
1416edd9a: 5d                          	popq	%rbp
1416edd9b: c3                          	retq
1416edd9c: 0f 1f 40 00                 	nopl	(%rax)
1416edda0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416edda5: 55                          	pushq	%rbp
1416edda6: 41 57                       	pushq	%r15
1416edda8: 41 56                       	pushq	%r14
1416eddaa: 41 55                       	pushq	%r13
1416eddac: 41 54                       	pushq	%r12
1416eddae: 56                          	pushq	%rsi
1416eddaf: 57                          	pushq	%rdi
1416eddb0: 53                          	pushq	%rbx
1416eddb1: 48 83 ec 38                 	subq	$0x38, %rsp
1416eddb5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416eddbc: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1416eddc0: e8 db aa ef fe              	callq	0x1405e88a0 <.text+0x5e78a0>
1416eddc5: 90                          	nop
1416eddc6: 48 83 c4 38                 	addq	$0x38, %rsp
1416eddca: 5b                          	popq	%rbx
1416eddcb: 5f                          	popq	%rdi
1416eddcc: 5e                          	popq	%rsi
1416eddcd: 41 5c                       	popq	%r12
1416eddcf: 41 5d                       	popq	%r13
1416eddd1: 41 5e                       	popq	%r14
1416eddd3: 41 5f                       	popq	%r15
1416eddd5: 5d                          	popq	%rbp
1416eddd6: c3                          	retq
1416eddd7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1416edde0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416edde5: 55                          	pushq	%rbp
1416edde6: 41 57                       	pushq	%r15
1416edde8: 41 56                       	pushq	%r14
1416eddea: 41 55                       	pushq	%r13
1416eddec: 41 54                       	pushq	%r12
1416eddee: 56                          	pushq	%rsi
1416eddef: 57                          	pushq	%rdi
1416eddf0: 53                          	pushq	%rbx
1416eddf1: 48 83 ec 38                 	subq	$0x38, %rsp
1416eddf5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416eddfc: 48 83 bd 78 02 00 00 00     	cmpq	$0x0, 0x278(%rbp)
1416ede04: 74 19                       	je	0x1416ede1f <.text+0x16ece1f>
1416ede06: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416ede0c: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
1416ede13: 48 8b 95 78 02 00 00        	movq	0x278(%rbp), %rdx
1416ede1a: e8 01 5e f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ede1f: 90                          	nop
1416ede20: 48 83 c4 38                 	addq	$0x38, %rsp
1416ede24: 5b                          	popq	%rbx
1416ede25: 5f                          	popq	%rdi
1416ede26: 5e                          	popq	%rsi
1416ede27: 41 5c                       	popq	%r12
1416ede29: 41 5d                       	popq	%r13
1416ede2b: 41 5e                       	popq	%r14
1416ede2d: 41 5f                       	popq	%r15
1416ede2f: 5d                          	popq	%rbp
1416ede30: c3                          	retq
1416ede31: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1416ede40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416ede45: 55                          	pushq	%rbp
1416ede46: 41 57                       	pushq	%r15
1416ede48: 41 56                       	pushq	%r14
1416ede4a: 41 55                       	pushq	%r13
1416ede4c: 41 54                       	pushq	%r12
1416ede4e: 56                          	pushq	%rsi
1416ede4f: 57                          	pushq	%rdi
1416ede50: 53                          	pushq	%rbx
1416ede51: 48 83 ec 38                 	subq	$0x38, %rsp
1416ede55: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416ede5c: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
1416ede63: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1416ede6a: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1416ede6e: 48 85 d2                    	testq	%rdx, %rdx
1416ede71: 74 10                       	je	0x1416ede83 <.text+0x16ece83>
1416ede73: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1416ede7a: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1416ede7e: e8 9d 5d f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ede83: ba 18 00 00 00              	movl	$0x18, %edx
1416ede88: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1416ede8e: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1416ede95: e8 86 5d f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ede9a: 90                          	nop
1416ede9b: 48 83 c4 38                 	addq	$0x38, %rsp
1416ede9f: 5b                          	popq	%rbx
1416edea0: 5f                          	popq	%rdi
1416edea1: 5e                          	popq	%rsi
1416edea2: 41 5c                       	popq	%r12
1416edea4: 41 5d                       	popq	%r13
1416edea6: 41 5e                       	popq	%r14
1416edea8: 41 5f                       	popq	%r15
1416edeaa: 5d                          	popq	%rbp
1416edeab: c3                          	retq
1416edeac: 0f 1f 40 00                 	nopl	(%rax)
1416edeb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416edeb5: 55                          	pushq	%rbp
1416edeb6: 41 57                       	pushq	%r15
1416edeb8: 41 56                       	pushq	%r14
1416edeba: 41 55                       	pushq	%r13
1416edebc: 41 54                       	pushq	%r12
1416edebe: 56                          	pushq	%rsi
1416edebf: 57                          	pushq	%rdi
1416edec0: 53                          	pushq	%rbx
1416edec1: 48 83 ec 38                 	subq	$0x38, %rsp
1416edec5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416edecc: 80 bd 8f 02 00 00 00        	cmpb	$0x0, 0x28f(%rbp)
1416eded3: 74 1e                       	je	0x1416edef3 <.text+0x16ecef3>
1416eded5: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ededc: e8 9f d4 ef fe              	callq	0x1405eb380 <.text+0x5ea380>
1416edee1: 90                          	nop
1416edee2: 48 83 c4 38                 	addq	$0x38, %rsp
1416edee6: 5b                          	popq	%rbx
1416edee7: 5f                          	popq	%rdi
1416edee8: 5e                          	popq	%rsi
1416edee9: 41 5c                       	popq	%r12
1416edeeb: 41 5d                       	popq	%r13
1416edeed: 41 5e                       	popq	%r14
1416edeef: 41 5f                       	popq	%r15
1416edef1: 5d                          	popq	%rbp
1416edef2: c3                          	retq
1416edef3: 48 83 c4 38                 	addq	$0x38, %rsp
1416edef7: 5b                          	popq	%rbx
1416edef8: 5f                          	popq	%rdi
1416edef9: 5e                          	popq	%rsi
1416edefa: 41 5c                       	popq	%r12
1416edefc: 41 5d                       	popq	%r13
1416edefe: 41 5e                       	popq	%r14
1416edf00: 41 5f                       	popq	%r15
1416edf02: 5d                          	popq	%rbp
1416edf03: c3                          	retq
1416edf04: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1416edf10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416edf15: 55                          	pushq	%rbp
1416edf16: 41 57                       	pushq	%r15
1416edf18: 41 56                       	pushq	%r14
1416edf1a: 41 55                       	pushq	%r13
1416edf1c: 41 54                       	pushq	%r12
1416edf1e: 56                          	pushq	%rsi
1416edf1f: 57                          	pushq	%rdi
1416edf20: 53                          	pushq	%rbx
1416edf21: 48 83 ec 38                 	subq	$0x38, %rsp
1416edf25: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416edf2c: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1416edf33: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1416edf3a: f0                          	lock
1416edf3b: 48 ff 08                    	decq	(%rax)
1416edf3e: 75 05                       	jne	0x1416edf45 <.text+0x16ecf45>
1416edf40: e8 2b 32 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416edf45: 90                          	nop
1416edf46: 48 83 c4 38                 	addq	$0x38, %rsp
1416edf4a: 5b                          	popq	%rbx
1416edf4b: 5f                          	popq	%rdi
1416edf4c: 5e                          	popq	%rsi
1416edf4d: 41 5c                       	popq	%r12
1416edf4f: 41 5d                       	popq	%r13
1416edf51: 41 5e                       	popq	%r14
1416edf53: 41 5f                       	popq	%r15
1416edf55: 5d                          	popq	%rbp
1416edf56: c3                          	retq
1416edf57: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1416edf60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416edf65: 55                          	pushq	%rbp
1416edf66: 41 57                       	pushq	%r15
1416edf68: 41 56                       	pushq	%r14
1416edf6a: 41 55                       	pushq	%r13
1416edf6c: 41 54                       	pushq	%r12
1416edf6e: 56                          	pushq	%rsi
1416edf6f: 57                          	pushq	%rdi
1416edf70: 53                          	pushq	%rbx
1416edf71: 48 83 ec 38                 	subq	$0x38, %rsp
1416edf75: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416edf7c: 48 8b 95 40 02 00 00        	movq	0x240(%rbp), %rdx
1416edf83: 48 85 d2                    	testq	%rdx, %rdx
1416edf86: 74 12                       	je	0x1416edf9a <.text+0x16ecf9a>
1416edf88: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
1416edf8f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416edf95: e8 86 5c f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416edf9a: 90                          	nop
1416edf9b: 48 83 c4 38                 	addq	$0x38, %rsp
1416edf9f: 5b                          	popq	%rbx
1416edfa0: 5f                          	popq	%rdi
1416edfa1: 5e                          	popq	%rsi
1416edfa2: 41 5c                       	popq	%r12
1416edfa4: 41 5d                       	popq	%r13
1416edfa6: 41 5e                       	popq	%r14
1416edfa8: 41 5f                       	popq	%r15
1416edfaa: 5d                          	popq	%rbp
1416edfab: c3                          	retq
1416edfac: 0f 1f 40 00                 	nopl	(%rax)
1416edfb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416edfb5: 55                          	pushq	%rbp
1416edfb6: 41 57                       	pushq	%r15
1416edfb8: 41 56                       	pushq	%r14
1416edfba: 41 55                       	pushq	%r13
1416edfbc: 41 54                       	pushq	%r12
1416edfbe: 56                          	pushq	%rsi
1416edfbf: 57                          	pushq	%rdi
1416edfc0: 53                          	pushq	%rbx
1416edfc1: 48 83 ec 38                 	subq	$0x38, %rsp
1416edfc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416edfcc: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
1416edfd0: 48 85 d2                    	testq	%rdx, %rdx
1416edfd3: 74 0f                       	je	0x1416edfe4 <.text+0x16ecfe4>
1416edfd5: 48 8b 4d b8                 	movq	-0x48(%rbp), %rcx
1416edfd9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416edfdf: e8 3c 5c f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416edfe4: 90                          	nop
1416edfe5: 48 83 c4 38                 	addq	$0x38, %rsp
1416edfe9: 5b                          	popq	%rbx
1416edfea: 5f                          	popq	%rdi
1416edfeb: 5e                          	popq	%rsi
1416edfec: 41 5c                       	popq	%r12
1416edfee: 41 5d                       	popq	%r13
1416edff0: 41 5e                       	popq	%r14
1416edff2: 41 5f                       	popq	%r15
1416edff4: 5d                          	popq	%rbp
1416edff5: c3                          	retq
1416edff6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1416ee000: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416ee005: 55                          	pushq	%rbp
1416ee006: 41 57                       	pushq	%r15
1416ee008: 41 56                       	pushq	%r14
1416ee00a: 41 55                       	pushq	%r13
1416ee00c: 41 54                       	pushq	%r12
1416ee00e: 56                          	pushq	%rsi
1416ee00f: 57                          	pushq	%rdi
1416ee010: 53                          	pushq	%rbx
1416ee011: 48 83 ec 38                 	subq	$0x38, %rsp
1416ee015: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416ee01c: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
1416ee023: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1416ee02a: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1416ee02e: 48 85 d2                    	testq	%rdx, %rdx
1416ee031: 74 10                       	je	0x1416ee043 <.text+0x16ed043>
1416ee033: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1416ee03a: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1416ee03e: e8 dd 5b f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ee043: ba 18 00 00 00              	movl	$0x18, %edx
1416ee048: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1416ee04e: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1416ee055: e8 c6 5b f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ee05a: 90                          	nop
1416ee05b: 48 83 c4 38                 	addq	$0x38, %rsp
1416ee05f: 5b                          	popq	%rbx
1416ee060: 5f                          	popq	%rdi
1416ee061: 5e                          	popq	%rsi
1416ee062: 41 5c                       	popq	%r12
1416ee064: 41 5d                       	popq	%r13
1416ee066: 41 5e                       	popq	%r14
1416ee068: 41 5f                       	popq	%r15
1416ee06a: 5d                          	popq	%rbp
1416ee06b: c3                          	retq
1416ee06c: 0f 1f 40 00                 	nopl	(%rax)
1416ee070: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416ee075: 55                          	pushq	%rbp
1416ee076: 41 57                       	pushq	%r15
1416ee078: 41 56                       	pushq	%r14
1416ee07a: 41 55                       	pushq	%r13
1416ee07c: 41 54                       	pushq	%r12
1416ee07e: 56                          	pushq	%rsi
1416ee07f: 57                          	pushq	%rdi
1416ee080: 53                          	pushq	%rbx
1416ee081: 48 83 ec 38                 	subq	$0x38, %rsp
1416ee085: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416ee08c: 48 83 bd 80 02 00 00 00     	cmpq	$0x0, 0x280(%rbp)
1416ee094: 74 19                       	je	0x1416ee0af <.text+0x16ed0af>
1416ee096: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416ee09c: 48 8b 8d 78 02 00 00        	movq	0x278(%rbp), %rcx
1416ee0a3: 48 8b 95 80 02 00 00        	movq	0x280(%rbp), %rdx
1416ee0aa: e8 71 5b f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ee0af: 90                          	nop
1416ee0b0: 48 83 c4 38                 	addq	$0x38, %rsp
1416ee0b4: 5b                          	popq	%rbx
1416ee0b5: 5f                          	popq	%rdi
1416ee0b6: 5e                          	popq	%rsi
1416ee0b7: 41 5c                       	popq	%r12
1416ee0b9: 41 5d                       	popq	%r13
1416ee0bb: 41 5e                       	popq	%r14
1416ee0bd: 41 5f                       	popq	%r15
1416ee0bf: 5d                          	popq	%rbp
1416ee0c0: c3                          	retq
1416ee0c1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1416ee0d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416ee0d5: 55                          	pushq	%rbp
1416ee0d6: 41 57                       	pushq	%r15
1416ee0d8: 41 56                       	pushq	%r14
1416ee0da: 41 55                       	pushq	%r13
1416ee0dc: 41 54                       	pushq	%r12
1416ee0de: 56                          	pushq	%rsi
1416ee0df: 57                          	pushq	%rdi
1416ee0e0: 53                          	pushq	%rbx
1416ee0e1: 48 83 ec 38                 	subq	$0x38, %rsp
1416ee0e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416ee0ec: ba 28 00 00 00              	movl	$0x28, %edx
1416ee0f1: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1416ee0f7: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1416ee0fe: e8 1d 5b f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ee103: 90                          	nop
1416ee104: 48 83 c4 38                 	addq	$0x38, %rsp
1416ee108: 5b                          	popq	%rbx
1416ee109: 5f                          	popq	%rdi
1416ee10a: 5e                          	popq	%rsi
1416ee10b: 41 5c                       	popq	%r12
1416ee10d: 41 5d                       	popq	%r13
1416ee10f: 41 5e                       	popq	%r14
1416ee111: 41 5f                       	popq	%r15
1416ee113: 5d                          	popq	%rbp
1416ee114: c3                          	retq
1416ee115: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1416ee120: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416ee125: 55                          	pushq	%rbp
1416ee126: 41 57                       	pushq	%r15
1416ee128: 41 56                       	pushq	%r14
1416ee12a: 41 55                       	pushq	%r13
1416ee12c: 41 54                       	pushq	%r12
1416ee12e: 56                          	pushq	%rsi
1416ee12f: 57                          	pushq	%rdi
1416ee130: 53                          	pushq	%rbx
1416ee131: 48 83 ec 38                 	subq	$0x38, %rsp
1416ee135: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416ee13c: 80 bd 8e 02 00 00 00        	cmpb	$0x0, 0x28e(%rbp)
1416ee143: 74 1e                       	je	0x1416ee163 <.text+0x16ed163>
1416ee145: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ee14c: e8 2f d2 ef fe              	callq	0x1405eb380 <.text+0x5ea380>
1416ee151: 90                          	nop
1416ee152: 48 83 c4 38                 	addq	$0x38, %rsp
1416ee156: 5b                          	popq	%rbx
1416ee157: 5f                          	popq	%rdi
1416ee158: 5e                          	popq	%rsi
1416ee159: 41 5c                       	popq	%r12
1416ee15b: 41 5d                       	popq	%r13
1416ee15d: 41 5e                       	popq	%r14
1416ee15f: 41 5f                       	popq	%r15
1416ee161: 5d                          	popq	%rbp
1416ee162: c3                          	retq
1416ee163: 48 83 c4 38                 	addq	$0x38, %rsp
1416ee167: 5b                          	popq	%rbx
1416ee168: 5f                          	popq	%rdi
1416ee169: 5e                          	popq	%rsi
1416ee16a: 41 5c                       	popq	%r12
1416ee16c: 41 5d                       	popq	%r13
1416ee16e: 41 5e                       	popq	%r14
1416ee170: 41 5f                       	popq	%r15
1416ee172: 5d                          	popq	%rbp
1416ee173: c3                          	retq
1416ee174: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1416ee180: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416ee185: 55                          	pushq	%rbp
1416ee186: 41 57                       	pushq	%r15
1416ee188: 41 56                       	pushq	%r14
1416ee18a: 41 55                       	pushq	%r13
1416ee18c: 41 54                       	pushq	%r12
1416ee18e: 56                          	pushq	%rsi
1416ee18f: 57                          	pushq	%rdi
1416ee190: 53                          	pushq	%rbx
1416ee191: 48 83 ec 38                 	subq	$0x38, %rsp
1416ee195: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416ee19c: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ee1a3: e8 d8 d1 ef fe              	callq	0x1405eb380 <.text+0x5ea380>
1416ee1a8: 90                          	nop
1416ee1a9: 48 83 c4 38                 	addq	$0x38, %rsp
1416ee1ad: 5b                          	popq	%rbx
1416ee1ae: 5f                          	popq	%rdi
1416ee1af: 5e                          	popq	%rsi
1416ee1b0: 41 5c                       	popq	%r12
1416ee1b2: 41 5d                       	popq	%r13
1416ee1b4: 41 5e                       	popq	%r14
1416ee1b6: 41 5f                       	popq	%r15
1416ee1b8: 5d                          	popq	%rbp
1416ee1b9: c3                          	retq
1416ee1ba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1416ee1c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416ee1c5: 55                          	pushq	%rbp
1416ee1c6: 41 57                       	pushq	%r15
1416ee1c8: 41 56                       	pushq	%r14
1416ee1ca: 41 55                       	pushq	%r13
1416ee1cc: 41 54                       	pushq	%r12
1416ee1ce: 56                          	pushq	%rsi
1416ee1cf: 57                          	pushq	%rdi
1416ee1d0: 53                          	pushq	%rbx
1416ee1d1: 48 83 ec 38                 	subq	$0x38, %rsp
1416ee1d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416ee1dc: c6 85 8e 02 00 00 01        	movb	$0x1, 0x28e(%rbp)
1416ee1e3: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
1416ee1ea: e8 11 98 ef fe              	callq	0x1405e7a00 <.text+0x5e6a00>
1416ee1ef: c6 85 8e 02 00 00 01        	movb	$0x1, 0x28e(%rbp)
1416ee1f6: 48 83 c4 38                 	addq	$0x38, %rsp
1416ee1fa: 5b                          	popq	%rbx
1416ee1fb: 5f                          	popq	%rdi
1416ee1fc: 5e                          	popq	%rsi
1416ee1fd: 41 5c                       	popq	%r12
1416ee1ff: 41 5d                       	popq	%r13
1416ee201: 41 5e                       	popq	%r14
1416ee203: 41 5f                       	popq	%r15
1416ee205: 5d                          	popq	%rbp
1416ee206: c3                          	retq


D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141d86060: 41 57                       	pushq	%r15
141d86062: 41 56                       	pushq	%r14
141d86064: 41 55                       	pushq	%r13
141d86066: 41 54                       	pushq	%r12
141d86068: 56                          	pushq	%rsi
141d86069: 57                          	pushq	%rdi
141d8606a: 55                          	pushq	%rbp
141d8606b: 53                          	pushq	%rbx
141d8606c: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
141d86073: 44 0f 29 bc 24 40 01 00 00  	movaps	%xmm15, 0x140(%rsp)
141d8607c: 44 0f 29 b4 24 30 01 00 00  	movaps	%xmm14, 0x130(%rsp)
141d86085: 44 0f 29 ac 24 20 01 00 00  	movaps	%xmm13, 0x120(%rsp)
141d8608e: 44 0f 29 a4 24 10 01 00 00  	movaps	%xmm12, 0x110(%rsp)
141d86097: 44 0f 29 9c 24 00 01 00 00  	movaps	%xmm11, 0x100(%rsp)
141d860a0: 44 0f 29 94 24 f0 00 00 00  	movaps	%xmm10, 0xf0(%rsp)
141d860a9: 44 0f 29 8c 24 e0 00 00 00  	movaps	%xmm9, 0xe0(%rsp)
141d860b2: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
141d860bb: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
141d860c3: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141d860cb: 8b 42 1c                    	movl	0x1c(%rdx), %eax
141d860ce: 4c 8b 4a 10                 	movq	0x10(%rdx), %r9
141d860d2: 41 89 01                    	movl	%eax, (%r9)
141d860d5: 48 8b 02                    	movq	(%rdx), %rax
141d860d8: 48 89 84 24 a0 00 00 00     	movq	%rax, 0xa0(%rsp)
141d860e0: 49 8b 00                    	movq	(%r8), %rax
141d860e3: f3 0f 10 78 28              	movss	0x28(%rax), %xmm7
141d860e8: 48 8b 11                    	movq	(%rcx), %rdx
141d860eb: 4c 8b 51 08                 	movq	0x8(%rcx), %r10
141d860ef: 8b 41 14                    	movl	0x14(%rcx), %eax
141d860f2: 89 44 24 2c                 	movl	%eax, 0x2c(%rsp)
141d860f6: 4c 8b a2 f8 00 00 00        	movq	0xf8(%rdx), %r12
141d860fd: 48 8b 82 00 01 00 00        	movq	0x100(%rdx), %rax
141d86104: 4d 8d 1c 84                 	leaq	(%r12,%rax,4), %r11
141d86108: 48 89 54 24 40              	movq	%rdx, 0x40(%rsp)
141d8610d: 0f b6 82 08 01 00 00        	movzbl	0x108(%rdx), %eax
141d86114: 88 44 24 2b                 	movb	%al, 0x2b(%rsp)
141d86118: b8 08 00 00 00              	movl	$0x8, %eax
141d8611d: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
141d86125: 48 c7 44 24 38 00 00 00 00  	movq	$0x0, 0x38(%rsp)
141d8612e: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
141d86132: f3 44 0f 10 15 35 f7 b9 00  	movss	0xb9f735(%rip), %xmm10  # 0x142925870
141d8613b: f3 44 0f 10 3d ec 1b 1e 01  	movss	0x11e1bec(%rip), %xmm15 # 0x142f67d30
141d86144: f3 44 0f 10 0d b7 f7 b9 00  	movss	0xb9f7b7(%rip), %xmm9   # 0x142925904
141d8614d: f3 44 0f 10 1d ae e4 ba 00  	movss	0xbae4ae(%rip), %xmm11  # 0x142934604
141d86156: f3 44 0f 10 25 b5 d6 cf 00  	movss	0xcfd6b5(%rip), %xmm12  # 0x142a83814
141d8615f: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
141d86168: 48 c7 44 24 50 00 00 00 00  	movq	$0x0, 0x50(%rsp)
141d86171: 48 c7 44 24 48 00 00 00 00  	movq	$0x0, 0x48(%rsp)
141d8617a: 31 ff                       	xorl	%edi, %edi
141d8617c: 31 d2                       	xorl	%edx, %edx
141d8617e: 4c 89 94 24 98 00 00 00     	movq	%r10, 0x98(%rsp)
141d86186: 4c 89 9c 24 90 00 00 00     	movq	%r11, 0x90(%rsp)
141d8618e: 4c 8b ac 24 a0 00 00 00     	movq	0xa0(%rsp), %r13
141d86196: eb 27                       	jmp	0x141d861bf <.text+0x1d851bf>
141d86198: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141d861a0: 8b 44 24 2c                 	movl	0x2c(%rsp), %eax
141d861a4: 89 01                       	movl	%eax, (%rcx)
141d861a6: c7 06 01 00 00 00           	movl	$0x1, (%rsi)
141d861ac: f3 0f 11 46 04              	movss	%xmm0, 0x4(%rsi)
141d861b1: f3 0f 11 76 08              	movss	%xmm6, 0x8(%rsi)
141d861b6: f3 0f 16 c0                 	movshdup	%xmm0, %xmm0            # xmm0 = xmm0[1,1,3,3]
141d861ba: f3 0f 11 46 0c              	movss	%xmm0, 0xc(%rsi)
141d861bf: 80 7c 24 2b 00              	cmpb	$0x0, 0x2b(%rsp)
141d861c4: 0f 84 96 00 00 00           	je	0x141d86260 <.text+0x1d85260>
141d861ca: 48 39 fa                    	cmpq	%rdi, %rdx
141d861cd: 0f 85 11 02 00 00           	jne	0x141d863e4 <.text+0x1d853e4>
141d861d3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141d861e0: 4d 39 dc                    	cmpq	%r11, %r12
141d861e3: 0f 84 50 05 00 00           	je	0x141d86739 <.text+0x1d85739>
141d861e9: 41 8b 0c 24                 	movl	(%r12), %ecx
141d861ed: 49 83 c4 04                 	addq	$0x4, %r12
141d861f1: 49 8b 82 a8 01 00 00        	movq	0x1a8(%r10), %rax
141d861f8: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
141d861fc: 48 8b 7c c8 10              	movq	0x10(%rax,%rcx,8), %rdi
141d86201: 48 85 ff                    	testq	%rdi, %rdi
141d86204: 74 da                       	je	0x141d861e0 <.text+0x1d851e0>
141d86206: 48 8d 0c c8                 	leaq	(%rax,%rcx,8), %rcx
141d8620a: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
141d8620f: 48 8b 80 10 01 00 00        	movq	0x110(%rax), %rax
141d86216: 48 8b 51 40                 	movq	0x40(%rcx), %rdx
141d8621a: 48 39 c2                    	cmpq	%rax, %rdx
141d8621d: 0f 86 cd 00 00 00           	jbe	0x141d862f0 <.text+0x1d852f0>
141d86223: 4c 8b 41 38                 	movq	0x38(%rcx), %r8
141d86227: 49 8b 04 c0                 	movq	(%r8,%rax,8), %rax
141d8622b: 48 85 c0                    	testq	%rax, %rax
141d8622e: 0f 84 bc 00 00 00           	je	0x141d862f0 <.text+0x1d852f0>
141d86234: 4c 8b 41 18                 	movq	0x18(%rcx), %r8
141d86238: 48 f7 d0                    	notq	%rax
141d8623b: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
141d8623f: 48 c1 e0 04                 	shlq	$0x4, %rax
141d86243: 49 8b 44 00 10              	movq	0x10(%r8,%rax), %rax
141d86248: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
141d8624d: e9 a7 00 00 00              	jmp	0x141d862f9 <.text+0x1d852f9>
141d86252: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
141d86260: 48 39 fa                    	cmpq	%rdi, %rdx
141d86263: 0f 85 f6 01 00 00           	jne	0x141d8645f <.text+0x1d8545f>
141d86269: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141d86270: 4d 39 dc                    	cmpq	%r11, %r12
141d86273: 0f 84 c0 04 00 00           	je	0x141d86739 <.text+0x1d85739>
141d86279: 41 8b 0c 24                 	movl	(%r12), %ecx
141d8627d: 49 83 c4 04                 	addq	$0x4, %r12
141d86281: 49 8b 82 00 01 00 00        	movq	0x100(%r10), %rax
141d86288: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
141d8628c: 48 c1 e1 05                 	shlq	$0x5, %rcx
141d86290: 48 8b 7c 08 58              	movq	0x58(%rax,%rcx), %rdi
141d86295: 48 85 ff                    	testq	%rdi, %rdi
141d86298: 74 d6                       	je	0x141d86270 <.text+0x1d85270>
141d8629a: 48 01 c8                    	addq	%rcx, %rax
141d8629d: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
141d862a3: 4d 8b 82 a8 01 00 00        	movq	0x1a8(%r10), %r8
141d862aa: 4c 8d 0c c9                 	leaq	(%rcx,%rcx,8), %r9
141d862ae: 4b 8d 14 c8                 	leaq	(%r8,%r9,8), %rdx
141d862b2: 48 8b 4c 24 40              	movq	0x40(%rsp), %rcx
141d862b7: 48 8b 89 10 01 00 00        	movq	0x110(%rcx), %rcx
141d862be: 4f 8b 44 c8 40              	movq	0x40(%r8,%r9,8), %r8
141d862c3: 49 39 c8                    	cmpq	%rcx, %r8
141d862c6: 76 6a                       	jbe	0x141d86332 <.text+0x1d85332>
141d862c8: 4c 8b 4a 38                 	movq	0x38(%rdx), %r9
141d862cc: 49 8b 0c c9                 	movq	(%r9,%rcx,8), %rcx
141d862d0: 48 85 c9                    	testq	%rcx, %rcx
141d862d3: 74 5d                       	je	0x141d86332 <.text+0x1d85332>
141d862d5: 4c 8b 4a 18                 	movq	0x18(%rdx), %r9
141d862d9: 48 f7 d1                    	notq	%rcx
141d862dc: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
141d862e0: 48 c1 e1 04                 	shlq	$0x4, %rcx
141d862e4: 49 8b 4c 09 10              	movq	0x10(%r9,%rcx), %rcx
141d862e9: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
141d862ee: eb 4b                       	jmp	0x141d8633b <.text+0x1d8533b>
141d862f0: 48 c7 44 24 38 00 00 00 00  	movq	$0x0, 0x38(%rsp)
141d862f9: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
141d862fe: 4c 8b 80 18 01 00 00        	movq	0x118(%rax), %r8
141d86305: 48 8b 41 38                 	movq	0x38(%rcx), %rax
141d86309: 4c 39 c2                    	cmpq	%r8, %rdx
141d8630c: 76 6e                       	jbe	0x141d8637c <.text+0x1d8537c>
141d8630e: 4a 8b 14 c0                 	movq	(%rax,%r8,8), %rdx
141d86312: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
141d86316: 48 85 d2                    	testq	%rdx, %rdx
141d86319: 74 65                       	je	0x141d86380 <.text+0x1d85380>
141d8631b: 48 f7 d2                    	notq	%rdx
141d8631e: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
141d86322: 48 c1 e2 04                 	shlq	$0x4, %rdx
141d86326: 48 8b 54 11 10              	movq	0x10(%rcx,%rdx), %rdx
141d8632b: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
141d86330: eb 57                       	jmp	0x141d86389 <.text+0x1d85389>
141d86332: 48 c7 44 24 38 00 00 00 00  	movq	$0x0, 0x38(%rsp)
141d8633b: 48 8b 4c 24 40              	movq	0x40(%rsp), %rcx
141d86340: 4c 8b 89 18 01 00 00        	movq	0x118(%rcx), %r9
141d86347: 48 8b 4a 38                 	movq	0x38(%rdx), %rcx
141d8634b: 4d 39 c8                    	cmpq	%r9, %r8
141d8634e: 0f 86 97 00 00 00           	jbe	0x141d863eb <.text+0x1d853eb>
141d86354: 4e 8b 04 c9                 	movq	(%rcx,%r9,8), %r8
141d86358: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
141d8635c: 4d 85 c0                    	testq	%r8, %r8
141d8635f: 0f 84 8a 00 00 00           	je	0x141d863ef <.text+0x1d853ef>
141d86365: 49 f7 d0                    	notq	%r8
141d86368: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
141d8636c: 49 c1 e0 04                 	shlq	$0x4, %r8
141d86370: 4e 8b 44 02 10              	movq	0x10(%rdx,%r8), %r8
141d86375: 4c 89 44 24 30              	movq	%r8, 0x30(%rsp)
141d8637a: eb 7c                       	jmp	0x141d863f8 <.text+0x1d853f8>
141d8637c: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
141d86380: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
141d86389: 4c 8b 44 24 40              	movq	0x40(%rsp), %r8
141d8638e: 49 8b 90 20 01 00 00        	movq	0x120(%r8), %rdx
141d86395: 4d 8b 80 28 01 00 00        	movq	0x128(%r8), %r8
141d8639c: 48 8b 14 d0                 	movq	(%rax,%rdx,8), %rdx
141d863a0: 48 f7 d2                    	notq	%rdx
141d863a3: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
141d863a7: 48 c1 e2 04                 	shlq	$0x4, %rdx
141d863ab: 4c 8b 4c 11 10              	movq	0x10(%rcx,%rdx), %r9
141d863b0: 4c 89 4c 24 50              	movq	%r9, 0x50(%rsp)
141d863b5: 48 8b 54 11 28              	movq	0x28(%rcx,%rdx), %rdx
141d863ba: 48 89 54 24 70              	movq	%rdx, 0x70(%rsp)
141d863bf: 4a 8b 04 c0                 	movq	(%rax,%r8,8), %rax
141d863c3: 48 f7 d0                    	notq	%rax
141d863c6: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
141d863ca: 48 c1 e0 04                 	shlq	$0x4, %rax
141d863ce: 48 8b 54 01 10              	movq	0x10(%rcx,%rax), %rdx
141d863d3: 48 89 54 24 48              	movq	%rdx, 0x48(%rsp)
141d863d8: 48 8b 44 01 28              	movq	0x28(%rcx,%rax), %rax
141d863dd: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
141d863e2: 31 d2                       	xorl	%edx, %edx
141d863e4: 89 d6                       	movl	%edx, %esi
141d863e6: e9 87 00 00 00              	jmp	0x141d86472 <.text+0x1d85472>
141d863eb: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
141d863ef: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
141d863f8: 4c 8b 4c 24 40              	movq	0x40(%rsp), %r9
141d863fd: 4d 8b 81 20 01 00 00        	movq	0x120(%r9), %r8
141d86404: 4d 8b 89 28 01 00 00        	movq	0x128(%r9), %r9
141d8640b: 4e 8b 04 c1                 	movq	(%rcx,%r8,8), %r8
141d8640f: 49 f7 d0                    	notq	%r8
141d86412: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
141d86416: 49 c1 e0 04                 	shlq	$0x4, %r8
141d8641a: 4a 8b 74 02 10              	movq	0x10(%rdx,%r8), %rsi
141d8641f: 48 89 74 24 50              	movq	%rsi, 0x50(%rsp)
141d86424: 4e 8b 44 02 28              	movq	0x28(%rdx,%r8), %r8
141d86429: 4c 89 44 24 70              	movq	%r8, 0x70(%rsp)
141d8642e: 4a 8b 0c c9                 	movq	(%rcx,%r9,8), %rcx
141d86432: 48 f7 d1                    	notq	%rcx
141d86435: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
141d86439: 48 c1 e1 04                 	shlq	$0x4, %rcx
141d8643d: 4c 8b 44 0a 10              	movq	0x10(%rdx,%rcx), %r8
141d86442: 4c 89 44 24 48              	movq	%r8, 0x48(%rsp)
141d86447: 48 8b 4c 0a 28              	movq	0x28(%rdx,%rcx), %rcx
141d8644c: 48 89 4c 24 68              	movq	%rcx, 0x68(%rsp)
141d86451: 48 8b 40 50                 	movq	0x50(%rax), %rax
141d86455: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
141d8645d: 31 d2                       	xorl	%edx, %edx
141d8645f: 48 89 d0                    	movq	%rdx, %rax
141d86462: 48 c1 e0 04                 	shlq	$0x4, %rax
141d86466: 48 8b 8c 24 80 00 00 00     	movq	0x80(%rsp), %rcx
141d8646e: 8b 74 01 08                 	movl	0x8(%rcx,%rax), %esi
141d86472: 48 8d 2c 76                 	leaq	(%rsi,%rsi,2), %rbp
141d86476: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
141d8647b: 4c 8d 3c a8                 	leaq	(%rax,%rbp,4), %r15
141d8647f: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
141d86484: 48 8d 1c e8                 	leaq	(%rax,%rbp,8), %rbx
141d86488: 48 c1 e5 04                 	shlq	$0x4, %rbp
141d8648c: 48 03 6c 24 38              	addq	0x38(%rsp), %rbp
141d86491: 48 8b 44 24 70              	movq	0x70(%rsp), %rax
141d86496: 4c 8d 34 b0                 	leaq	(%rax,%rsi,4), %r14
141d8649a: 48 8b 44 24 68              	movq	0x68(%rsp), %rax
141d8649f: 48 8d 0c b0                 	leaq	(%rax,%rsi,4), %rcx
141d864a3: 48 c1 e6 04                 	shlq	$0x4, %rsi
141d864a7: 48 03 74 24 48              	addq	0x48(%rsp), %rsi
141d864ac: 48 ff c2                    	incq	%rdx
141d864af: 8b 44 24 2c                 	movl	0x2c(%rsp), %eax
141d864b3: 41 89 06                    	movl	%eax, (%r14)
141d864b6: f3 0f 10 43 08              	movss	0x8(%rbx), %xmm0
141d864bb: f3 0f 5c c7                 	subss	%xmm7, %xmm0
141d864bf: f3 0f 11 43 08              	movss	%xmm0, 0x8(%rbx)
141d864c4: 44 0f 2e c0                 	ucomiss	%xmm0, %xmm8
141d864c8: 8b 06                       	movl	(%rsi), %eax
141d864ca: 73 54                       	jae	0x141d86520 <.text+0x1d85520>
141d864cc: a8 01                       	testb	$0x1, %al
141d864ce: 74 50                       	je	0x141d86520 <.text+0x1d85520>
141d864d0: f3 0f 10 46 08              	movss	0x8(%rsi), %xmm0
141d864d5: f3 0f 5c 45 14              	subss	0x14(%rbp), %xmm0
141d864da: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
141d864de: f3 0f 10 4e 0c              	movss	0xc(%rsi), %xmm1
141d864e3: f3 0f 10 56 04              	movss	0x4(%rsi), %xmm2
141d864e8: 0f 14 d1                    	unpcklps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
141d864eb: f3 0f 10 4d 10              	movss	0x10(%rbp), %xmm1
141d864f0: f3 0f 10 5d 18              	movss	0x18(%rbp), %xmm3
141d864f5: 0f 14 cb                    	unpcklps	%xmm3, %xmm1            # xmm1 = xmm1[0],xmm3[0],xmm1[1],xmm3[1]
141d864f8: 0f 5c d1                    	subps	%xmm1, %xmm2
141d864fb: 0f 59 d2                    	mulps	%xmm2, %xmm2
141d864fe: f3 0f 58 c2                 	addss	%xmm2, %xmm0
141d86502: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
141d86506: f3 0f 58 c8                 	addss	%xmm0, %xmm1
141d8650a: f3 0f 10 05 ee 17 1e 01     	movss	0x11e17ee(%rip), %xmm0  # 0x142f67d00
141d86512: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
141d86515: 0f 86 a4 fc ff ff           	jbe	0x141d861bf <.text+0x1d851bf>
141d8651b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141d86520: 48 89 8c 24 88 00 00 00     	movq	%rcx, 0x88(%rsp)
141d86528: 48 89 94 24 a8 00 00 00     	movq	%rdx, 0xa8(%rsp)
141d86530: 4c 89 e9                    	movq	%r13, %rcx
141d86533: 89 44 24 7c                 	movl	%eax, 0x7c(%rsp)
141d86537: e8 a4 f5 89 ff              	callq	0x141625ae0 <.text+0x1624ae0>
141d8653c: f3 41 0f 59 c2              	mulss	%xmm10, %xmm0
141d86541: f3 0f 58 05 3b f4 b9 00     	addss	0xb9f43b(%rip), %xmm0   # 0x142925984
141d86549: 8b 44 24 2c                 	movl	0x2c(%rsp), %eax
141d8654d: 41 89 06                    	movl	%eax, (%r14)
141d86550: f3 0f 11 43 08              	movss	%xmm0, 0x8(%rbx)
141d86555: 66 44 0f 6e 2b              	movd	(%rbx), %xmm13
141d8655a: 4c 89 e9                    	movq	%r13, %rcx
141d8655d: e8 7e f5 89 ff              	callq	0x141625ae0 <.text+0x1624ae0>
141d86562: 0f 28 f0                    	movaps	%xmm0, %xmm6
141d86565: 4c 89 e9                    	movq	%r13, %rcx
141d86568: e8 73 f5 89 ff              	callq	0x141625ae0 <.text+0x1624ae0>
141d8656d: 66 0f 7e f2                 	movd	%xmm6, %edx
141d86571: 66 0f 7e c0                 	movd	%xmm0, %eax
141d86575: 48 c1 e0 20                 	shlq	$0x20, %rax
141d86579: 48 09 c2                    	orq	%rax, %rdx
141d8657c: 4c 8d 74 24 5c              	leaq	0x5c(%rsp), %r14
141d86581: 4c 89 f1                    	movq	%r14, %rcx
141d86584: e8 47 a2 f0 fe              	callq	0x140c907d0 <.text+0xc8f7d0>
141d86589: f3 44 0f 10 74 24 5c        	movss	0x5c(%rsp), %xmm14
141d86590: f3 0f 10 74 24 60           	movss	0x60(%rsp), %xmm6
141d86596: f3 0f 10 44 24 64           	movss	0x64(%rsp), %xmm0
141d8659c: 44 0f 14 f0                 	unpcklps	%xmm0, %xmm14           # xmm14 = xmm14[0],xmm0[0],xmm14[1],xmm0[1]
141d865a0: 4c 89 f1                    	movq	%r14, %rcx
141d865a3: 4c 89 fa                    	movq	%r15, %rdx
141d865a6: e8 85 9a f0 fe              	callq	0x140c90030 <.text+0xc8f030>
141d865ab: 66 0f 6e 44 24 7c           	movd	0x7c(%rsp), %xmm0
141d865b1: 66 44 0f 62 e8              	punpckldq	%xmm0, %xmm13   # xmm13 = xmm13[0],xmm0[0],xmm13[1],xmm0[1]
141d865b6: 66 44 0f 76 2d c1 55 d6 00  	pcmpeqd	0xd655c1(%rip), %xmm13  # 0x142aebb80
141d865bf: 66 41 0f 6f c5              	movdqa	%xmm13, %xmm0
141d865c4: 66 0f df 05 44 17 1e 01     	pandn	0x11e1744(%rip), %xmm0  # 0x142f67d10
141d865cc: 66 44 0f db 2d 4b 17 1e 01  	pand	0x11e174b(%rip), %xmm13 # 0x142f67d20
141d865d5: 66 44 0f eb e8              	por	%xmm0, %xmm13
141d865da: f3 0f 10 54 24 5c           	movss	0x5c(%rsp), %xmm2
141d865e0: f3 0f 10 44 24 60           	movss	0x60(%rsp), %xmm0
141d865e6: 0f 14 f0                    	unpcklps	%xmm0, %xmm6            # xmm6 = xmm6[0],xmm0[0],xmm6[1],xmm0[1]
141d865e9: f3 0f 10 44 24 64           	movss	0x64(%rsp), %xmm0
141d865ef: 0f 14 d0                    	unpcklps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0],xmm2[1],xmm0[1]
141d865f2: 41 0f 59 f5                 	mulps	%xmm13, %xmm6
141d865f6: f3 0f 16 c6                 	movshdup	%xmm6, %xmm0            # xmm0 = xmm6[1,1,3,3]
141d865fa: f3 41 0f 59 f2              	mulss	%xmm10, %xmm6
141d865ff: f3 0f 58 f0                 	addss	%xmm0, %xmm6
141d86603: 0f 28 de                    	movaps	%xmm6, %xmm3
141d86606: f3 0f 59 de                 	mulss	%xmm6, %xmm3
141d8660a: f3 0f 10 43 08              	movss	0x8(%rbx), %xmm0
141d8660f: f3 41 0f 59 c7              	mulss	%xmm15, %xmm0
141d86614: 66 41 0f 70 e5 a0           	pshufd	$0xa0, %xmm13, %xmm4    # xmm4 = xmm13[0,0,2,2]
141d8661a: 41 0f 59 e6                 	mulps	%xmm14, %xmm4
141d8661e: 66 41 0f 70 cd f5           	pshufd	$0xf5, %xmm13, %xmm1    # xmm1 = xmm13[1,1,3,3]
141d86624: 0f 59 ca                    	mulps	%xmm2, %xmm1
141d86627: 0f 58 cc                    	addps	%xmm4, %xmm1
141d8662a: 0f 28 d1                    	movaps	%xmm1, %xmm2
141d8662d: 0f 59 d1                    	mulps	%xmm1, %xmm2
141d86630: f3 0f 58 da                 	addss	%xmm2, %xmm3
141d86634: f3 0f 16 d2                 	movshdup	%xmm2, %xmm2            # xmm2 = xmm2[1,1,3,3]
141d86638: f3 0f 58 d3                 	addss	%xmm3, %xmm2
141d8663c: f3 0f 51 d2                 	sqrtss	%xmm2, %xmm2
141d86640: 41 0f 28 e1                 	movaps	%xmm9, %xmm4
141d86644: f3 0f 5e e2                 	divss	%xmm2, %xmm4
141d86648: 66 0f 7e e0                 	movd	%xmm4, %eax
141d8664c: 85 c0                       	testl	%eax, %eax
141d8664e: 0f 99 c1                    	setns	%cl
141d86651: 89 c2                       	movl	%eax, %edx
141d86653: 81 e2 ff ff ff 7f           	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
141d86659: 81 c2 00 00 80 ff           	addl	$0xff800000, %edx       # imm = 0xFF800000
141d8665f: 81 fa 00 00 00 7f           	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
141d86665: 0f 92 c2                    	setb	%dl
141d86668: 20 ca                       	andb	%cl, %dl
141d8666a: ff c8                       	decl	%eax
141d8666c: 3d ff ff 7f 00              	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
141d86671: 0f 92 c0                    	setb	%al
141d86674: 08 d0                       	orb	%dl, %al
141d86676: 75 08                       	jne	0x141d86680 <.text+0x1d85680>
141d86678: 0f 57 f6                    	xorps	%xmm6, %xmm6
141d8667b: eb 07                       	jmp	0x141d86684 <.text+0x1d85684>
141d8667d: 0f 1f 00                    	nopl	(%rax)
141d86680: f3 0f 59 f4                 	mulss	%xmm4, %xmm6
141d86684: 48 8b 8c 24 88 00 00 00     	movq	0x88(%rsp), %rcx
141d8668c: f3 0f 10 55 10              	movss	0x10(%rbp), %xmm2
141d86691: f3 0f 10 5d 18              	movss	0x18(%rbp), %xmm3
141d86696: f3 0f 12 e4                 	movsldup	%xmm4, %xmm4            # xmm4 = xmm4[0,0,2,2]
141d8669a: 0f 59 cc                    	mulps	%xmm4, %xmm1
141d8669d: 0f b6 c0                    	movzbl	%al, %eax
141d866a0: 66 0f 6e e0                 	movd	%eax, %xmm4
141d866a4: 66 0f 70 e4 50              	pshufd	$0x50, %xmm4, %xmm4     # xmm4 = xmm4[0,0,1,1]
141d866a9: 66 0f 72 f4 1f              	pslld	$0x1f, %xmm4
141d866ae: 66 0f 72 e4 1f              	psrad	$0x1f, %xmm4
141d866b3: 66 0f db e1                 	pand	%xmm1, %xmm4
141d866b7: f3 0f 59 f0                 	mulss	%xmm0, %xmm6
141d866bb: f3 0f 12 c8                 	movsldup	%xmm0, %xmm1            # xmm1 = xmm0[0,0,2,2]
141d866bf: f3 0f 58 75 14              	addss	0x14(%rbp), %xmm6
141d866c4: 0f 59 cc                    	mulps	%xmm4, %xmm1
141d866c7: 0f 28 c2                    	movaps	%xmm2, %xmm0
141d866ca: 0f 14 c3                    	unpcklps	%xmm3, %xmm0            # xmm0 = xmm0[0],xmm3[0],xmm0[1],xmm3[1]
141d866cd: 0f 58 c1                    	addps	%xmm1, %xmm0
141d866d0: f3 41 0f 5d f3              	minss	%xmm11, %xmm6
141d866d5: 0f 28 c8                    	movaps	%xmm0, %xmm1
141d866d8: 0f 59 c8                    	mulps	%xmm0, %xmm1
141d866db: f3 0f 16 e1                 	movshdup	%xmm1, %xmm4            # xmm4 = xmm1[1,1,3,3]
141d866df: f3 0f 58 e1                 	addss	%xmm1, %xmm4
141d866e3: 0f 57 c9                    	xorps	%xmm1, %xmm1
141d866e6: f3 0f 51 cc                 	sqrtss	%xmm4, %xmm1
141d866ea: 44 0f 2e e1                 	ucomiss	%xmm1, %xmm12
141d866ee: 4c 8b 94 24 98 00 00 00     	movq	0x98(%rsp), %r10
141d866f6: 4c 8b 9c 24 90 00 00 00     	movq	0x90(%rsp), %r11
141d866fe: 48 8b 94 24 a8 00 00 00     	movq	0xa8(%rsp), %rdx
141d86706: 0f 87 94 fa ff ff           	ja	0x141d861a0 <.text+0x1d851a0>
141d8670c: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
141d86710: f3 0f 59 db                 	mulss	%xmm3, %xmm3
141d86714: f3 0f 58 da                 	addss	%xmm2, %xmm3
141d86718: 0f 57 d2                    	xorps	%xmm2, %xmm2
141d8671b: f3 0f 51 d3                 	sqrtss	%xmm3, %xmm2
141d8671f: 0f 2e d1                    	ucomiss	%xmm1, %xmm2
141d86722: 0f 87 78 fa ff ff           	ja	0x141d861a0 <.text+0x1d851a0>
141d86728: 8b 44 24 2c                 	movl	0x2c(%rsp), %eax
141d8672c: 89 01                       	movl	%eax, (%rcx)
141d8672e: c7 06 00 00 00 00           	movl	$0x0, (%rsi)
141d86734: e9 86 fa ff ff              	jmp	0x141d861bf <.text+0x1d851bf>
141d86739: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
141d86741: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
141d86749: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
141d86752: 44 0f 28 8c 24 e0 00 00 00  	movaps	0xe0(%rsp), %xmm9
141d8675b: 44 0f 28 94 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm10
141d86764: 44 0f 28 9c 24 00 01 00 00  	movaps	0x100(%rsp), %xmm11
141d8676d: 44 0f 28 a4 24 10 01 00 00  	movaps	0x110(%rsp), %xmm12
141d86776: 44 0f 28 ac 24 20 01 00 00  	movaps	0x120(%rsp), %xmm13
141d8677f: 44 0f 28 b4 24 30 01 00 00  	movaps	0x130(%rsp), %xmm14
141d86788: 44 0f 28 bc 24 40 01 00 00  	movaps	0x140(%rsp), %xmm15
141d86791: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
141d86798: 5b                          	popq	%rbx
141d86799: 5d                          	popq	%rbp
141d8679a: 5f                          	popq	%rdi
141d8679b: 5e                          	popq	%rsi
141d8679c: 41 5c                       	popq	%r12
141d8679e: 41 5d                       	popq	%r13
141d867a0: 41 5e                       	popq	%r14
141d867a2: 41 5f                       	popq	%r15
141d867a4: c3                          	retq

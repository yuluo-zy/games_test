
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1414745e0: 55                          	pushq	%rbp
1414745e1: 41 57                       	pushq	%r15
1414745e3: 41 56                       	pushq	%r14
1414745e5: 41 55                       	pushq	%r13
1414745e7: 41 54                       	pushq	%r12
1414745e9: 56                          	pushq	%rsi
1414745ea: 57                          	pushq	%rdi
1414745eb: 53                          	pushq	%rbx
1414745ec: 48 81 ec 78 02 00 00        	subq	$0x278, %rsp            # imm = 0x278
1414745f3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1414745fb: 48 c7 85 f0 01 00 00 fe ff ff ff    	movq	$-0x2, 0x1f0(%rbp)
141474606: 48 89 d7                    	movq	%rdx, %rdi
141474609: 48 89 ca                    	movq	%rcx, %rdx
14147460c: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
141474610: 41 b8 90 00 00 00           	movl	$0x90, %r8d
141474616: e8 e5 f5 4a 01              	callq	0x142923c00 <.text+0x2922c00>
14147461b: 48 8b 07                    	movq	(%rdi), %rax
14147461e: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
141474625: 48 8b 47 08                 	movq	0x8(%rdi), %rax
141474629: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
141474630: 48 8b 47 10                 	movq	0x10(%rdi), %rax
141474634: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
14147463b: 48 8d 7d c8                 	leaq	-0x38(%rbp), %rdi
14147463f: 4c 8d bd b8 00 00 00        	leaq	0xb8(%rbp), %r15
141474646: 48 8b 5d 20                 	movq	0x20(%rbp), %rbx
14147464a: 4c 8d a5 a0 01 00 00        	leaq	0x1a0(%rbp), %r12
141474651: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
141474659: 48 85 db                    	testq	%rbx, %rbx
14147465c: 75 25                       	jne	0x141474683 <.text+0x1473683>
14147465e: e9 46 03 00 00              	jmp	0x1414749a9 <.text+0x14739a9>
141474663: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141474670: 31 db                       	xorl	%ebx, %ebx
141474672: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
14147467a: 48 85 db                    	testq	%rbx, %rbx
14147467d: 0f 84 26 03 00 00           	je	0x1414749a9 <.text+0x14739a9>
141474683: 41 b8 88 00 00 00           	movl	$0x88, %r8d
141474689: 48 8d 8d 18 01 00 00        	leaq	0x118(%rbp), %rcx
141474690: 48 8d 55 28                 	leaq	0x28(%rbp), %rdx
141474694: e8 67 f5 4a 01              	callq	0x142923c00 <.text+0x2922c00>
141474699: 48 89 9d 10 01 00 00        	movq	%rbx, 0x110(%rbp)
1414746a0: 4c 8b ad 40 01 00 00        	movq	0x140(%rbp), %r13
1414746a7: 4c 8b b5 48 01 00 00        	movq	0x148(%rbp), %r14
1414746ae: 4b 8d 04 2e                 	leaq	(%r14,%r13), %rax
1414746b2: 48 ff c0                    	incq	%rax
1414746b5: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
1414746bb: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
1414746bf: f7 d1                       	notl	%ecx
1414746c1: 49 8d 55 10                 	leaq	0x10(%r13), %rdx
1414746c5: 66 89 8d b8 01 00 00        	movw	%cx, 0x1b8(%rbp)
1414746cc: 4c 89 ad a0 01 00 00        	movq	%r13, 0x1a0(%rbp)
1414746d3: 48 89 95 a8 01 00 00        	movq	%rdx, 0x1a8(%rbp)
1414746da: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
1414746e1: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
1414746e8: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1414746ef: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
1414746f6: 4c 89 e2                    	movq	%r12, %rdx
1414746f9: e8 72 4f fd ff              	callq	0x141449670 <.text+0x1448670>
1414746fe: f3 0f 6f 85 b0 00 00 00     	movdqu	0xb0(%rbp), %xmm0
141474706: 0f 10 8d c0 00 00 00        	movups	0xc0(%rbp), %xmm1
14147470d: 0f 10 95 d0 00 00 00        	movups	0xd0(%rbp), %xmm2
141474714: 66 0f 7f 85 a0 01 00 00     	movdqa	%xmm0, 0x1a0(%rbp)
14147471c: 0f 29 8d b0 01 00 00        	movaps	%xmm1, 0x1b0(%rbp)
141474723: 0f 29 95 c0 01 00 00        	movaps	%xmm2, 0x1c0(%rbp)
14147472a: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
141474731: 0f b6 00                    	movzbl	(%rax), %eax
141474734: 88 44 24 30                 	movb	%al, 0x30(%rsp)
141474738: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
14147473d: c6 44 24 28 00              	movb	$0x0, 0x28(%rsp)
141474742: 41 b8 02 00 00 00           	movl	$0x2, %r8d
141474748: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
14147474c: 48 8b 95 d8 01 00 00        	movq	0x1d8(%rbp), %rdx
141474753: e8 18 77 6b ff              	callq	0x140b2be70 <.text+0xb2ae70>
141474758: 48 8b 75 c0                 	movq	-0x40(%rbp), %rsi
14147475c: 48 85 f6                    	testq	%rsi, %rsi
14147475f: 0f 84 9b 01 00 00           	je	0x141474900 <.text+0x1473900>
141474765: 48 8b 47 50                 	movq	0x50(%rdi), %rax
141474769: 49 89 47 50                 	movq	%rax, 0x50(%r15)
14147476d: 0f 10 47 40                 	movups	0x40(%rdi), %xmm0
141474771: 41 0f 11 47 40              	movups	%xmm0, 0x40(%r15)
141474776: f3 0f 6f 07                 	movdqu	(%rdi), %xmm0
14147477a: 0f 10 4f 10                 	movups	0x10(%rdi), %xmm1
14147477e: 0f 10 57 20                 	movups	0x20(%rdi), %xmm2
141474782: 0f 10 5f 30                 	movups	0x30(%rdi), %xmm3
141474786: 41 0f 11 5f 30              	movups	%xmm3, 0x30(%r15)
14147478b: 41 0f 11 57 20              	movups	%xmm2, 0x20(%r15)
141474790: 41 0f 11 4f 10              	movups	%xmm1, 0x10(%r15)
141474795: f3 41 0f 7f 07              	movdqu	%xmm0, (%r15)
14147479a: 4d 85 f6                    	testq	%r14, %r14
14147479d: 74 29                       	je	0x1414747c8 <.text+0x14737c8>
14147479f: 49 8d 46 01                 	leaq	0x1(%r14), %rax
1414747a3: b9 50 00 00 00              	movl	$0x50, %ecx
1414747a8: 48 f7 e1                    	mulq	%rcx
1414747ab: 49 01 c6                    	addq	%rax, %r14
1414747ae: 49 83 c6 11                 	addq	$0x11, %r14
1414747b2: 74 14                       	je	0x1414747c8 <.text+0x14737c8>
1414747b4: 49 29 c5                    	subq	%rax, %r13
1414747b7: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1414747bd: 4c 89 e9                    	movq	%r13, %rcx
1414747c0: 4c 89 f2                    	movq	%r14, %rdx
1414747c3: e8 58 f4 19 ff              	callq	0x140613c20 <.text+0x612c20>
1414747c8: 48 8b 95 18 01 00 00        	movq	0x118(%rbp), %rdx
1414747cf: 48 85 d2                    	testq	%rdx, %rdx
1414747d2: 74 26                       	je	0x1414747fa <.text+0x14737fa>
1414747d4: 48 8d 04 95 13 00 00 00     	leaq	0x13(,%rdx,4), %rax
1414747dc: 48 83 e0 f0                 	andq	$-0x10, %rax
1414747e0: 48 01 c2                    	addq	%rax, %rdx
1414747e3: 48 83 c2 11                 	addq	$0x11, %rdx
1414747e7: 74 11                       	je	0x1414747fa <.text+0x14737fa>
1414747e9: 48 29 c3                    	subq	%rax, %rbx
1414747ec: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1414747f2: 48 89 d9                    	movq	%rbx, %rcx
1414747f5: e8 26 f4 19 ff              	callq	0x140613c20 <.text+0x612c20>
1414747fa: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
141474801: 48 85 d2                    	testq	%rdx, %rdx
141474804: 74 2a                       	je	0x141474830 <.text+0x1473830>
141474806: 48 8d 04 95 13 00 00 00     	leaq	0x13(,%rdx,4), %rax
14147480e: 48 83 e0 f0                 	andq	$-0x10, %rax
141474812: 48 01 c2                    	addq	%rax, %rdx
141474815: 48 83 c2 11                 	addq	$0x11, %rdx
141474819: 74 15                       	je	0x141474830 <.text+0x1473830>
14147481b: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
141474822: 48 29 c1                    	subq	%rax, %rcx
141474825: 41 b8 10 00 00 00           	movl	$0x10, %r8d
14147482b: e8 f0 f3 19 ff              	callq	0x140613c20 <.text+0x612c20>
141474830: 48 89 b5 b0 00 00 00        	movq	%rsi, 0xb0(%rbp)
141474837: 48 8d 85 b0 00 00 00        	leaq	0xb0(%rbp), %rax
14147483e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141474843: 41 b8 02 00 00 00           	movl	$0x2, %r8d
141474849: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
141474850: 48 8b 95 e8 01 00 00        	movq	0x1e8(%rbp), %rdx
141474857: e8 e4 4f 0b 00              	callq	0x141529840 <.text+0x1528840>
14147485c: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
141474863: 48 85 c9                    	testq	%rcx, %rcx
141474866: 0f 84 04 fe ff ff           	je	0x141474670 <.text+0x1473670>
14147486c: 4c 8b 8d 18 01 00 00        	movq	0x118(%rbp), %r9
141474873: 4d 85 c9                    	testq	%r9, %r9
141474876: 74 2e                       	je	0x1414748a6 <.text+0x14738a6>
141474878: 49 8d 41 01                 	leaq	0x1(%r9), %rax
14147487c: ba 58 00 00 00              	movl	$0x58, %edx
141474881: 48 f7 e2                    	mulq	%rdx
141474884: 48 83 c0 0f                 	addq	$0xf, %rax
141474888: 48 83 e0 f0                 	andq	$-0x10, %rax
14147488c: 49 01 c1                    	addq	%rax, %r9
14147488f: 49 83 c1 11                 	addq	$0x11, %r9
141474893: 74 11                       	je	0x1414748a6 <.text+0x14738a6>
141474895: 48 29 c1                    	subq	%rax, %rcx
141474898: 41 b8 10 00 00 00           	movl	$0x10, %r8d
14147489e: 4c 89 ca                    	movq	%r9, %rdx
1414748a1: e8 7a f3 19 ff              	callq	0x140613c20 <.text+0x612c20>
1414748a6: 4c 8b 8d 48 01 00 00        	movq	0x148(%rbp), %r9
1414748ad: 4d 85 c9                    	testq	%r9, %r9
1414748b0: 0f 84 ba fd ff ff           	je	0x141474670 <.text+0x1473670>
1414748b6: 49 8d 41 01                 	leaq	0x1(%r9), %rax
1414748ba: b9 58 00 00 00              	movl	$0x58, %ecx
1414748bf: 48 f7 e1                    	mulq	%rcx
1414748c2: 48 83 c0 0f                 	addq	$0xf, %rax
1414748c6: 48 83 e0 f0                 	andq	$-0x10, %rax
1414748ca: 49 01 c1                    	addq	%rax, %r9
1414748cd: 49 83 c1 11                 	addq	$0x11, %r9
1414748d1: 0f 84 99 fd ff ff           	je	0x141474670 <.text+0x1473670>
1414748d7: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
1414748de: 48 29 c1                    	subq	%rax, %rcx
1414748e1: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1414748e7: 4c 89 ca                    	movq	%r9, %rdx
1414748ea: e8 31 f3 19 ff              	callq	0x140613c20 <.text+0x612c20>
1414748ef: e9 7c fd ff ff              	jmp	0x141474670 <.text+0x1473670>
1414748f4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141474900: 4d 85 f6                    	testq	%r14, %r14
141474903: 74 29                       	je	0x14147492e <.text+0x147392e>
141474905: 49 8d 46 01                 	leaq	0x1(%r14), %rax
141474909: b9 50 00 00 00              	movl	$0x50, %ecx
14147490e: 48 f7 e1                    	mulq	%rcx
141474911: 49 01 c6                    	addq	%rax, %r14
141474914: 49 83 c6 11                 	addq	$0x11, %r14
141474918: 74 14                       	je	0x14147492e <.text+0x147392e>
14147491a: 49 29 c5                    	subq	%rax, %r13
14147491d: 41 b8 10 00 00 00           	movl	$0x10, %r8d
141474923: 4c 89 e9                    	movq	%r13, %rcx
141474926: 4c 89 f2                    	movq	%r14, %rdx
141474929: e8 f2 f2 19 ff              	callq	0x140613c20 <.text+0x612c20>
14147492e: 48 8b 95 18 01 00 00        	movq	0x118(%rbp), %rdx
141474935: 48 85 d2                    	testq	%rdx, %rdx
141474938: 74 26                       	je	0x141474960 <.text+0x1473960>
14147493a: 48 8d 04 95 13 00 00 00     	leaq	0x13(,%rdx,4), %rax
141474942: 48 83 e0 f0                 	andq	$-0x10, %rax
141474946: 48 01 c2                    	addq	%rax, %rdx
141474949: 48 83 c2 11                 	addq	$0x11, %rdx
14147494d: 74 11                       	je	0x141474960 <.text+0x1473960>
14147494f: 48 29 c3                    	subq	%rax, %rbx
141474952: 41 b8 10 00 00 00           	movl	$0x10, %r8d
141474958: 48 89 d9                    	movq	%rbx, %rcx
14147495b: e8 c0 f2 19 ff              	callq	0x140613c20 <.text+0x612c20>
141474960: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
141474967: 48 85 d2                    	testq	%rdx, %rdx
14147496a: 74 2a                       	je	0x141474996 <.text+0x1473996>
14147496c: 48 8d 04 95 13 00 00 00     	leaq	0x13(,%rdx,4), %rax
141474974: 48 83 e0 f0                 	andq	$-0x10, %rax
141474978: 48 01 c2                    	addq	%rax, %rdx
14147497b: 48 83 c2 11                 	addq	$0x11, %rdx
14147497f: 74 15                       	je	0x141474996 <.text+0x1473996>
141474981: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
141474988: 48 29 c1                    	subq	%rax, %rcx
14147498b: 41 b8 10 00 00 00           	movl	$0x10, %r8d
141474991: e8 8a f2 19 ff              	callq	0x140613c20 <.text+0x612c20>
141474996: 31 db                       	xorl	%ebx, %ebx
141474998: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
1414749a0: 48 85 db                    	testq	%rbx, %rbx
1414749a3: 0f 85 da fc ff ff           	jne	0x141474683 <.text+0x1473683>
1414749a9: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
1414749ad: e8 2e ef 15 ff              	callq	0x1405d38e0 <.text+0x5d28e0>
1414749b2: 90                          	nop
1414749b3: 48 81 c4 78 02 00 00        	addq	$0x278, %rsp            # imm = 0x278
1414749ba: 5b                          	popq	%rbx
1414749bb: 5f                          	popq	%rdi
1414749bc: 5e                          	popq	%rsi
1414749bd: 41 5c                       	popq	%r12
1414749bf: 41 5d                       	popq	%r13
1414749c1: 41 5e                       	popq	%r14
1414749c3: 41 5f                       	popq	%r15
1414749c5: 5d                          	popq	%rbp
1414749c6: c3                          	retq
1414749c7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1414749d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1414749d5: 55                          	pushq	%rbp
1414749d6: 41 57                       	pushq	%r15
1414749d8: 41 56                       	pushq	%r14
1414749da: 41 55                       	pushq	%r13
1414749dc: 41 54                       	pushq	%r12
1414749de: 56                          	pushq	%rsi
1414749df: 57                          	pushq	%rdi
1414749e0: 53                          	pushq	%rbx
1414749e1: 48 83 ec 38                 	subq	$0x38, %rsp
1414749e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1414749ec: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
1414749f0: e8 eb ee 15 ff              	callq	0x1405d38e0 <.text+0x5d28e0>
1414749f5: 90                          	nop
1414749f6: 48 83 c4 38                 	addq	$0x38, %rsp
1414749fa: 5b                          	popq	%rbx
1414749fb: 5f                          	popq	%rdi
1414749fc: 5e                          	popq	%rsi
1414749fd: 41 5c                       	popq	%r12
1414749ff: 41 5d                       	popq	%r13
141474a01: 41 5e                       	popq	%r14
141474a03: 41 5f                       	popq	%r15
141474a05: 5d                          	popq	%rbp
141474a06: c3                          	retq
141474a07: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141474a10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141474a15: 55                          	pushq	%rbp
141474a16: 41 57                       	pushq	%r15
141474a18: 41 56                       	pushq	%r14
141474a1a: 41 55                       	pushq	%r13
141474a1c: 41 54                       	pushq	%r12
141474a1e: 56                          	pushq	%rsi
141474a1f: 57                          	pushq	%rdi
141474a20: 53                          	pushq	%rbx
141474a21: 48 83 ec 38                 	subq	$0x38, %rsp
141474a25: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141474a2c: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
141474a33: e8 58 00 19 ff              	callq	0x140604a90 <.text+0x603a90>
141474a38: 90                          	nop
141474a39: 48 83 c4 38                 	addq	$0x38, %rsp
141474a3d: 5b                          	popq	%rbx
141474a3e: 5f                          	popq	%rdi
141474a3f: 5e                          	popq	%rsi
141474a40: 41 5c                       	popq	%r12
141474a42: 41 5d                       	popq	%r13
141474a44: 41 5e                       	popq	%r14
141474a46: 41 5f                       	popq	%r15
141474a48: 5d                          	popq	%rbp
141474a49: c3                          	retq

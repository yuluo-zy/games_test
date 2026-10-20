
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416eeec0: 55                          	pushq	%rbp
1416eeec1: 41 57                       	pushq	%r15
1416eeec3: 41 56                       	pushq	%r14
1416eeec5: 41 55                       	pushq	%r13
1416eeec7: 41 54                       	pushq	%r12
1416eeec9: 56                          	pushq	%rsi
1416eeeca: 57                          	pushq	%rdi
1416eeecb: 53                          	pushq	%rbx
1416eeecc: 48 81 ec 38 02 00 00        	subq	$0x238, %rsp            # imm = 0x238
1416eeed3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1416eeedb: 48 c7 85 b0 01 00 00 fe ff ff ff    	movq	$-0x2, 0x1b0(%rbp)
1416eeee6: 48 89 d6                    	movq	%rdx, %rsi
1416eeee9: 48 89 cf                    	movq	%rcx, %rdi
1416eeeec: 48 8d 15 8d 15 5a 01        	leaq	0x15a158d(%rip), %rdx   # 0x142c90480
1416eeef3: 48 8d 5d a8                 	leaq	-0x58(%rbp), %rbx
1416eeef7: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
1416eeefd: 48 89 d9                    	movq	%rbx, %rcx
1416eef00: e8 6b d4 2e ff              	callq	0x1409dc370 <.text+0x9db370>
1416eef05: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1416eef0c: 48 89 da                    	movq	%rbx, %rdx
1416eef0f: 49 89 f0                    	movq	%rsi, %r8
1416eef12: e8 09 7e 55 ff              	callq	0x140c46d20 <.text+0xc45d20>
1416eef17: 80 bd a4 01 00 00 00        	cmpb	$0x0, 0x1a4(%rbp)
1416eef1e: 0f 84 17 01 00 00           	je	0x1416ef03b <.text+0x16ee03b>
1416eef24: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1416eef2b: f0                          	lock
1416eef2c: 48 ff 08                    	decq	(%rax)
1416eef2f: 75 0c                       	jne	0x1416eef3d <.text+0x16edf3d>
1416eef31: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1416eef38: e8 33 22 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416eef3d: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
1416eef41: 48 8b 01                    	movq	(%rcx), %rax
1416eef44: 48 8b 51 08                 	movq	0x8(%rcx), %rdx
1416eef48: 44 8b 41 10                 	movl	0x10(%rcx), %r8d
1416eef4c: 8b 49 14                    	movl	0x14(%rcx), %ecx
1416eef4f: 4c 8d 8a a0 01 00 00        	leaq	0x1a0(%rdx), %r9
1416eef56: 4c 8d 92 f8 00 00 00        	leaq	0xf8(%rdx), %r10
1416eef5d: 4c 8b 98 f8 00 00 00        	movq	0xf8(%rax), %r11
1416eef64: 48 8b 98 00 01 00 00        	movq	0x100(%rax), %rbx
1416eef6b: 49 8d 1c 9b                 	leaq	(%r11,%rbx,4), %rbx
1416eef6f: 44 0f b6 b0 08 01 00 00     	movzbl	0x108(%rax), %r14d
1416eef77: 44 88 75 68                 	movb	%r14b, 0x68(%rbp)
1416eef7b: 4c 89 5d c8                 	movq	%r11, -0x38(%rbp)
1416eef7f: 48 89 5d d0                 	movq	%rbx, -0x30(%rbp)
1416eef83: 48 c7 45 d8 08 00 00 00     	movq	$0x8, -0x28(%rbp)
1416eef8b: 48 c7 45 e0 00 00 00 00     	movq	$0x0, -0x20(%rbp)
1416eef93: 48 c7 45 e8 08 00 00 00     	movq	$0x8, -0x18(%rbp)
1416eef9b: 0f 57 c0                    	xorps	%xmm0, %xmm0
1416eef9e: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
1416eefa2: 44 89 45 10                 	movl	%r8d, 0x10(%rbp)
1416eefa6: 89 4d 14                    	movl	%ecx, 0x14(%rbp)
1416eefa9: 48 c7 45 18 00 00 00 00     	movq	$0x0, 0x18(%rbp)
1416eefb1: 44 89 45 30                 	movl	%r8d, 0x30(%rbp)
1416eefb5: 89 4d 34                    	movl	%ecx, 0x34(%rbp)
1416eefb8: 48 c7 45 38 00 00 00 00     	movq	$0x0, 0x38(%rbp)
1416eefc0: 44 89 45 50                 	movl	%r8d, 0x50(%rbp)
1416eefc4: 89 4d 54                    	movl	%ecx, 0x54(%rbp)
1416eefc7: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
1416eefcb: 48 89 55 a8                 	movq	%rdx, -0x58(%rbp)
1416eefcf: 4c 89 4d b0                 	movq	%r9, -0x50(%rbp)
1416eefd3: 4c 89 55 b8                 	movq	%r10, -0x48(%rbp)
1416eefd7: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
1416eefdb: 4c 8d 05 46 ee 57 01        	leaq	0x157ee46(%rip), %r8    # 0x142c6de28
1416eefe2: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1416eefe9: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
1416eefed: e8 de 5b c1 ff              	callq	0x141304bd0 <.text+0x1303bd0>
1416eeff2: 48 8b 5f 08                 	movq	0x8(%rdi), %rbx
1416eeff6: 48 8b 03                    	movq	(%rbx), %rax
1416eeff9: 48 89 c1                    	movq	%rax, %rcx
1416eeffc: 48 f7 d9                    	negq	%rcx
1416eefff: 70 19                       	jo	0x1416ef01a <.text+0x16ee01a>
1416ef001: 73 17                       	jae	0x1416ef01a <.text+0x16ee01a>
1416ef003: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1416ef007: 48 c1 e0 03                 	shlq	$0x3, %rax
1416ef00b: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1416ef00f: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1416ef015: e8 06 4c f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ef01a: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1416ef021: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1416ef025: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
1416ef02c: 0f 11 03                    	movups	%xmm0, (%rbx)
1416ef02f: 31 c0                       	xorl	%eax, %eax
1416ef031: 48 3b 03                    	cmpq	(%rbx), %rax
1416ef034: 71 52                       	jno	0x1416ef088 <.text+0x16ee088>
1416ef036: e9 42 02 00 00              	jmp	0x1416ef27d <.text+0x16ee27d>
1416ef03b: c6 45 a8 00                 	movb	$0x0, -0x58(%rbp)
1416ef03f: 80 bd a3 01 00 00 00        	cmpb	$0x0, 0x1a3(%rbp)
1416ef046: 74 18                       	je	0x1416ef060 <.text+0x16ee060>
1416ef048: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1416ef04f: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
1416ef053: e8 18 38 95 fe              	callq	0x140042870 <.text+0x41870>
1416ef058: 84 c0                       	testb	%al, %al
1416ef05a: 0f 85 c4 fe ff ff           	jne	0x1416eef24 <.text+0x16edf24>
1416ef060: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1416ef067: f0                          	lock
1416ef068: 48 ff 08                    	decq	(%rax)
1416ef06b: 75 0c                       	jne	0x1416ef079 <.text+0x16ee079>
1416ef06d: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1416ef074: e8 f7 20 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ef079: 48 8b 5f 08                 	movq	0x8(%rdi), %rbx
1416ef07d: 31 c0                       	xorl	%eax, %eax
1416ef07f: 48 3b 03                    	cmpq	(%rbx), %rax
1416ef082: 0f 80 f5 01 00 00           	jo	0x1416ef27d <.text+0x16ee27d>
1416ef088: 48 8d 15 fe 13 5a 01        	leaq	0x15a13fe(%rip), %rdx   # 0x142c9048d
1416ef08f: 4c 8d 75 a8                 	leaq	-0x58(%rbp), %r14
1416ef093: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1416ef099: 4c 89 f1                    	movq	%r14, %rcx
1416ef09c: e8 cf d2 2e ff              	callq	0x1409dc370 <.text+0x9db370>
1416ef0a1: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1416ef0a8: 4c 89 f2                    	movq	%r14, %rdx
1416ef0ab: 49 89 f0                    	movq	%rsi, %r8
1416ef0ae: e8 6d 7c 55 ff              	callq	0x140c46d20 <.text+0xc45d20>
1416ef0b3: 80 bd a4 01 00 00 00        	cmpb	$0x0, 0x1a4(%rbp)
1416ef0ba: 0f 84 40 01 00 00           	je	0x1416ef200 <.text+0x16ee200>
1416ef0c0: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1416ef0c7: f0                          	lock
1416ef0c8: 48 ff 08                    	decq	(%rax)
1416ef0cb: 75 0c                       	jne	0x1416ef0d9 <.text+0x16ee0d9>
1416ef0cd: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1416ef0d4: e8 97 20 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ef0d9: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
1416ef0dd: 48 89 da                    	movq	%rbx, %rdx
1416ef0e0: e8 0b 34 00 00              	callq	0x1416f24f0 <.text+0x16f14f0>
1416ef0e5: 4c 8b 2f                    	movq	(%rdi), %r13
1416ef0e8: 31 c0                       	xorl	%eax, %eax
1416ef0ea: 49 3b 45 00                 	cmpq	(%r13), %rax
1416ef0ee: 0f 80 89 01 00 00           	jo	0x1416ef27d <.text+0x16ee27d>
1416ef0f4: 4c 8b 53 10                 	movq	0x10(%rbx), %r10
1416ef0f8: 31 c9                       	xorl	%ecx, %ecx
1416ef0fa: ba 28 00 00 00              	movl	$0x28, %edx
1416ef0ff: 4c 89 d0                    	movq	%r10, %rax
1416ef102: 48 f7 e2                    	mulq	%rdx
1416ef105: 49 89 c4                    	movq	%rax, %r12
1416ef108: 0f 80 ea 01 00 00           	jo	0x1416ef2f8 <.text+0x16ee2f8>
1416ef10e: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
1416ef118: 49 39 c4                    	cmpq	%rax, %r12
1416ef11b: 0f 87 d7 01 00 00           	ja	0x1416ef2f8 <.text+0x16ee2f8>
1416ef121: 4d 85 e4                    	testq	%r12, %r12
1416ef124: 0f 84 16 01 00 00           	je	0x1416ef240 <.text+0x16ee240>
1416ef12a: 4c 8b 73 08                 	movq	0x8(%rbx), %r14
1416ef12e: 0f b6 05 d4 5a f4 01        	movzbl	0x1f45ad4(%rip), %eax   # 0x143634c09
1416ef135: ba 04 00 00 00              	movl	$0x4, %edx
1416ef13a: 4c 89 e1                    	movq	%r12, %rcx
1416ef13d: 4d 89 d7                    	movq	%r10, %r15
1416ef140: e8 cb 4a f2 fe              	callq	0x140613c10 <.text+0x612c10>
1416ef145: b9 04 00 00 00              	movl	$0x4, %ecx
1416ef14a: 48 85 c0                    	testq	%rax, %rax
1416ef14d: 0f 84 a5 01 00 00           	je	0x1416ef2f8 <.text+0x16ee2f8>
1416ef153: 4d 89 fa                    	movq	%r15, %r10
1416ef156: 49 89 c7                    	movq	%rax, %r15
1416ef159: 4d 85 d2                    	testq	%r10, %r10
1416ef15c: 0f 84 e4 00 00 00           	je	0x1416ef246 <.text+0x16ee246>
1416ef162: 49 8d 46 04                 	leaq	0x4(%r14), %rax
1416ef166: 31 c9                       	xorl	%ecx, %ecx
1416ef168: 4c 89 d2                    	movq	%r10, %rdx
1416ef16b: eb 69                       	jmp	0x1416ef1d6 <.text+0x16ee1d6>
1416ef16d: 0f 1f 00                    	nopl	(%rax)
1416ef170: 48 ff ca                    	decq	%rdx
1416ef173: 44 8b 4d b0                 	movl	-0x50(%rbp), %r9d
1416ef177: 44 89 8d 58 01 00 00        	movl	%r9d, 0x158(%rbp)
1416ef17e: 4c 8b 4d a8                 	movq	-0x58(%rbp), %r9
1416ef182: 4c 89 8d 50 01 00 00        	movq	%r9, 0x150(%rbp)
1416ef189: 45 89 04 0f                 	movl	%r8d, (%r15,%rcx)
1416ef18d: 44 8b 85 58 01 00 00        	movl	0x158(%rbp), %r8d
1416ef194: 45 89 44 0f 0c              	movl	%r8d, 0xc(%r15,%rcx)
1416ef199: 4c 8b 85 50 01 00 00        	movq	0x150(%rbp), %r8
1416ef1a0: 4d 89 44 0f 04              	movq	%r8, 0x4(%r15,%rcx)
1416ef1a5: 4d 8b 46 10                 	movq	0x10(%r14), %r8
1416ef1a9: 4d 89 44 0f 10              	movq	%r8, 0x10(%r15,%rcx)
1416ef1ae: 45 8b 46 18                 	movl	0x18(%r14), %r8d
1416ef1b2: 45 89 44 0f 18              	movl	%r8d, 0x18(%r15,%rcx)
1416ef1b7: 4d 8b 46 1c                 	movq	0x1c(%r14), %r8
1416ef1bb: 4d 89 44 0f 1c              	movq	%r8, 0x1c(%r15,%rcx)
1416ef1c0: 45 8b 46 24                 	movl	0x24(%r14), %r8d
1416ef1c4: 49 83 c6 28                 	addq	$0x28, %r14
1416ef1c8: 45 89 44 0f 24              	movl	%r8d, 0x24(%r15,%rcx)
1416ef1cd: 48 83 c1 28                 	addq	$0x28, %rcx
1416ef1d1: 48 85 d2                    	testq	%rdx, %rdx
1416ef1d4: 74 70                       	je	0x1416ef246 <.text+0x16ee246>
1416ef1d6: 49 39 cc                    	cmpq	%rcx, %r12
1416ef1d9: 74 6b                       	je	0x1416ef246 <.text+0x16ee246>
1416ef1db: 45 31 c0                    	xorl	%r8d, %r8d
1416ef1de: 41 83 3e 01                 	cmpl	$0x1, (%r14)
1416ef1e2: 75 8c                       	jne	0x1416ef170 <.text+0x16ee170>
1416ef1e4: 44 8b 44 08 08              	movl	0x8(%rax,%rcx), %r8d
1416ef1e9: 44 89 45 b0                 	movl	%r8d, -0x50(%rbp)
1416ef1ed: 4c 8b 04 08                 	movq	(%rax,%rcx), %r8
1416ef1f1: 4c 89 45 a8                 	movq	%r8, -0x58(%rbp)
1416ef1f5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1416ef1fb: e9 70 ff ff ff              	jmp	0x1416ef170 <.text+0x16ee170>
1416ef200: c6 45 a8 00                 	movb	$0x0, -0x58(%rbp)
1416ef204: 80 bd a3 01 00 00 00        	cmpb	$0x0, 0x1a3(%rbp)
1416ef20b: 74 18                       	je	0x1416ef225 <.text+0x16ee225>
1416ef20d: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1416ef214: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
1416ef218: e8 53 36 95 fe              	callq	0x140042870 <.text+0x41870>
1416ef21d: 84 c0                       	testb	%al, %al
1416ef21f: 0f 85 9b fe ff ff           	jne	0x1416ef0c0 <.text+0x16ee0c0>
1416ef225: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1416ef22c: f0                          	lock
1416ef22d: 48 ff 08                    	decq	(%rax)
1416ef230: 75 4b                       	jne	0x1416ef27d <.text+0x16ee27d>
1416ef232: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1416ef239: e8 32 1f 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ef23e: eb 3d                       	jmp	0x1416ef27d <.text+0x16ee27d>
1416ef240: 41 bf 04 00 00 00           	movl	$0x4, %r15d
1416ef246: 49 8b 45 00                 	movq	(%r13), %rax
1416ef24a: 48 89 c1                    	movq	%rax, %rcx
1416ef24d: 48 f7 d9                    	negq	%rcx
1416ef250: 70 1f                       	jo	0x1416ef271 <.text+0x16ee271>
1416ef252: 73 1d                       	jae	0x1416ef271 <.text+0x16ee271>
1416ef254: 49 8b 4d 08                 	movq	0x8(%r13), %rcx
1416ef258: 48 c1 e0 03                 	shlq	$0x3, %rax
1416ef25c: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1416ef260: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1416ef266: 4d 89 d6                    	movq	%r10, %r14
1416ef269: e8 b2 49 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416ef26e: 4d 89 f2                    	movq	%r14, %r10
1416ef271: 4d 89 55 00                 	movq	%r10, (%r13)
1416ef275: 4d 89 7d 08                 	movq	%r15, 0x8(%r13)
1416ef279: 4d 89 55 10                 	movq	%r10, 0x10(%r13)
1416ef27d: 48 8b 7f 10                 	movq	0x10(%rdi), %rdi
1416ef281: 0f b6 05 81 59 f4 01        	movzbl	0x1f45981(%rip), %eax   # 0x143634c09
1416ef288: b9 10 00 00 00              	movl	$0x10, %ecx
1416ef28d: ba 08 00 00 00              	movl	$0x8, %edx
1416ef292: e8 79 49 f2 fe              	callq	0x140613c10 <.text+0x612c10>
1416ef297: 48 85 c0                    	testq	%rax, %rax
1416ef29a: 74 4d                       	je	0x1416ef2e9 <.text+0x16ee2e9>
1416ef29c: 48 89 18                    	movq	%rbx, (%rax)
1416ef29f: 48 89 78 08                 	movq	%rdi, 0x8(%rax)
1416ef2a3: 48 8d 0d d6 0d 5a 01        	leaq	0x15a0dd6(%rip), %rcx   # 0x142c90080
1416ef2aa: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1416ef2af: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
1416ef2b3: 48 89 f2                    	movq	%rsi, %rdx
1416ef2b6: 45 31 c0                    	xorl	%r8d, %r8d
1416ef2b9: 49 89 c1                    	movq	%rax, %r9
1416ef2bc: e8 9f e9 3e ff              	callq	0x140addc60 <.text+0xadcc60>
1416ef2c1: 48 8b 45 b8                 	movq	-0x48(%rbp), %rax
1416ef2c5: f0                          	lock
1416ef2c6: 48 ff 08                    	decq	(%rax)
1416ef2c9: 75 09                       	jne	0x1416ef2d4 <.text+0x16ee2d4>
1416ef2cb: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
1416ef2cf: e8 9c 1e 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ef2d4: 90                          	nop
1416ef2d5: 48 81 c4 38 02 00 00        	addq	$0x238, %rsp            # imm = 0x238
1416ef2dc: 5b                          	popq	%rbx
1416ef2dd: 5f                          	popq	%rdi
1416ef2de: 5e                          	popq	%rsi
1416ef2df: 41 5c                       	popq	%r12
1416ef2e1: 41 5d                       	popq	%r13
1416ef2e3: 41 5e                       	popq	%r14
1416ef2e5: 41 5f                       	popq	%r15
1416ef2e7: 5d                          	popq	%rbp
1416ef2e8: c3                          	retq
1416ef2e9: b9 08 00 00 00              	movl	$0x8, %ecx
1416ef2ee: ba 10 00 00 00              	movl	$0x10, %edx
1416ef2f3: e8 08 9d 1e 01              	callq	0x1428d9000 <.text+0x28d8000>
1416ef2f8: 4c 8d 05 49 98 54 01        	leaq	0x1549849(%rip), %r8    # 0x142c38b48
1416ef2ff: 4c 89 e2                    	movq	%r12, %rdx
1416ef302: e8 dc 9c 1e 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1416ef307: cc                          	int3
1416ef308: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1416ef310: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416ef315: 55                          	pushq	%rbp
1416ef316: 41 57                       	pushq	%r15
1416ef318: 41 56                       	pushq	%r14
1416ef31a: 41 55                       	pushq	%r13
1416ef31c: 41 54                       	pushq	%r12
1416ef31e: 56                          	pushq	%rsi
1416ef31f: 57                          	pushq	%rdi
1416ef320: 53                          	pushq	%rbx
1416ef321: 48 83 ec 28                 	subq	$0x28, %rsp
1416ef325: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416ef32c: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1416ef333: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1416ef33a: f0                          	lock
1416ef33b: 48 ff 08                    	decq	(%rax)
1416ef33e: 75 05                       	jne	0x1416ef345 <.text+0x16ee345>
1416ef340: e8 2b 1e 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ef345: 90                          	nop
1416ef346: 48 83 c4 28                 	addq	$0x28, %rsp
1416ef34a: 5b                          	popq	%rbx
1416ef34b: 5f                          	popq	%rdi
1416ef34c: 5e                          	popq	%rsi
1416ef34d: 41 5c                       	popq	%r12
1416ef34f: 41 5d                       	popq	%r13
1416ef351: 41 5e                       	popq	%r14
1416ef353: 41 5f                       	popq	%r15
1416ef355: 5d                          	popq	%rbp
1416ef356: c3                          	retq
1416ef357: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1416ef360: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416ef365: 55                          	pushq	%rbp
1416ef366: 41 57                       	pushq	%r15
1416ef368: 41 56                       	pushq	%r14
1416ef36a: 41 55                       	pushq	%r13
1416ef36c: 41 54                       	pushq	%r12
1416ef36e: 56                          	pushq	%rsi
1416ef36f: 57                          	pushq	%rdi
1416ef370: 53                          	pushq	%rbx
1416ef371: 48 83 ec 28                 	subq	$0x28, %rsp
1416ef375: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416ef37c: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1416ef383: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1416ef38a: f0                          	lock
1416ef38b: 48 ff 08                    	decq	(%rax)
1416ef38e: 75 05                       	jne	0x1416ef395 <.text+0x16ee395>
1416ef390: e8 db 1d 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ef395: 90                          	nop
1416ef396: 48 83 c4 28                 	addq	$0x28, %rsp
1416ef39a: 5b                          	popq	%rbx
1416ef39b: 5f                          	popq	%rdi
1416ef39c: 5e                          	popq	%rsi
1416ef39d: 41 5c                       	popq	%r12
1416ef39f: 41 5d                       	popq	%r13
1416ef3a1: 41 5e                       	popq	%r14
1416ef3a3: 41 5f                       	popq	%r15
1416ef3a5: 5d                          	popq	%rbp
1416ef3a6: c3                          	retq

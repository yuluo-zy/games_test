
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1417cb9e0: 56                          	pushq	%rsi
1417cb9e1: 57                          	pushq	%rdi
1417cb9e2: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1417cb9e9: bf 01 00 00 00              	movl	$0x1, %edi
1417cb9ee: f0                          	lock
1417cb9ef: 0f c1 ba b0 05 00 00        	xaddl	%edi, 0x5b0(%rdx)
1417cb9f6: f6 01 01                    	testb	$0x1, (%rcx)
1417cb9f9: 0f 84 22 02 00 00           	je	0x1417cbc21 <.text+0x17cac21>
1417cb9ff: 48 89 ce                    	movq	%rcx, %rsi
1417cba02: 48 8b 41 08                 	movq	0x8(%rcx), %rax
1417cba06: 48 39 82 18 02 00 00        	cmpq	%rax, 0x218(%rdx)
1417cba0d: 0f 86 f2 00 00 00           	jbe	0x1417cbb05 <.text+0x17cab05>
1417cba13: 48 8b 8a 10 02 00 00        	movq	0x210(%rdx), %rcx
1417cba1a: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
1417cba1e: 48 85 c9                    	testq	%rcx, %rcx
1417cba21: 0f 84 de 00 00 00           	je	0x1417cbb05 <.text+0x17cab05>
1417cba27: 48 8b 82 e0 01 00 00        	movq	0x1e0(%rdx), %rax
1417cba2e: 48 f7 d1                    	notq	%rcx
1417cba31: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1417cba35: 48 c1 e1 05                 	shlq	$0x5, %rcx
1417cba39: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1417cba3f: 0f 84 c0 00 00 00           	je	0x1417cbb05 <.text+0x17cab05>
1417cba45: 48 01 c8                    	addq	%rcx, %rax
1417cba48: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1417cba4c: 44 8b 82 b0 05 00 00        	movl	0x5b0(%rdx), %r8d
1417cba53: 4c 8b 46 10                 	movq	0x10(%rsi), %r8
1417cba57: 4c 39 82 18 02 00 00        	cmpq	%r8, 0x218(%rdx)
1417cba5e: 0f 86 2f 01 00 00           	jbe	0x1417cbb93 <.text+0x17cab93>
1417cba64: 4c 8b 8a 10 02 00 00        	movq	0x210(%rdx), %r9
1417cba6b: 4f 8b 0c c1                 	movq	(%r9,%r8,8), %r9
1417cba6f: 4d 85 c9                    	testq	%r9, %r9
1417cba72: 0f 84 1b 01 00 00           	je	0x1417cbb93 <.text+0x17cab93>
1417cba78: 4c 8b 82 e0 01 00 00        	movq	0x1e0(%rdx), %r8
1417cba7f: 49 f7 d1                    	notq	%r9
1417cba82: 4b 8d 14 49                 	leaq	(%r9,%r9,2), %rdx
1417cba86: 48 c1 e2 05                 	shlq	$0x5, %rdx
1417cba8a: 49 83 7c 10 20 00           	cmpq	$0x0, 0x20(%r8,%rdx)
1417cba90: 0f 84 fd 00 00 00           	je	0x1417cbb93 <.text+0x17cab93>
1417cba96: 44 8b 8e 50 01 00 00        	movl	0x150(%rsi), %r9d
1417cba9d: 49 01 d0                    	addq	%rdx, %r8
1417cbaa0: 48 8d 50 5c                 	leaq	0x5c(%rax), %rdx
1417cbaa4: 48 83 c0 58                 	addq	$0x58, %rax
1417cbaa8: 4d 8b 50 10                 	movq	0x10(%r8), %r10
1417cbaac: 4d 8d 58 58                 	leaq	0x58(%r8), %r11
1417cbab0: 49 83 c0 5c                 	addq	$0x5c, %r8
1417cbab4: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
1417cbab9: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
1417cbabe: 48 89 54 24 48              	movq	%rdx, 0x48(%rsp)
1417cbac3: 44 89 4c 24 50              	movl	%r9d, 0x50(%rsp)
1417cbac8: 89 7c 24 54                 	movl	%edi, 0x54(%rsp)
1417cbacc: 48 8d 54 24 58              	leaq	0x58(%rsp), %rdx
1417cbad1: 4c 89 54 24 58              	movq	%r10, 0x58(%rsp)
1417cbad6: 4c 89 5c 24 60              	movq	%r11, 0x60(%rsp)
1417cbadb: 4c 89 44 24 68              	movq	%r8, 0x68(%rsp)
1417cbae0: 44 89 4c 24 70              	movl	%r9d, 0x70(%rsp)
1417cbae5: 89 7c 24 74                 	movl	%edi, 0x74(%rsp)
1417cbae9: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
1417cbaee: e8 cd 3e 0f ff              	callq	0x1408bf9c0 <.text+0x8be9c0>
1417cbaf3: 89 be 50 01 00 00           	movl	%edi, 0x150(%rsi)
1417cbaf9: 31 c0                       	xorl	%eax, %eax
1417cbafb: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1417cbb02: 5f                          	popq	%rdi
1417cbb03: 5e                          	popq	%rsi
1417cbb04: c3                          	retq
1417cbb05: 48 81 c6 b8 00 00 00        	addq	$0xb8, %rsi
1417cbb0c: 48 8d 05 61 76 51 01        	leaq	0x1517661(%rip), %rax   # 0x142ce3174
1417cbb13: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1417cbb18: 48 c7 44 24 30 3f 00 00 00  	movq	$0x3f, 0x30(%rsp)
1417cbb21: 48 89 74 24 78              	movq	%rsi, 0x78(%rsp)
1417cbb26: 48 8d 05 a3 ff 86 fe        	leaq	-0x179005d(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
1417cbb2d: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1417cbb35: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
1417cbb3a: 48 89 84 24 88 00 00 00     	movq	%rax, 0x88(%rsp)
1417cbb42: 48 8d 05 47 e3 93 fe        	leaq	-0x16c1cb9(%rip), %rax  # 0x140109e90 <.text+0x108e90>
1417cbb49: 48 89 84 24 90 00 00 00     	movq	%rax, 0x90(%rsp)
1417cbb51: 48 8d 05 50 72 51 01        	leaq	0x1517250(%rip), %rax   # 0x142ce2da8
1417cbb58: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1417cbb5d: 48 c7 44 24 40 02 00 00 00  	movq	$0x2, 0x40(%rsp)
1417cbb66: 48 c7 44 24 58 00 00 00 00  	movq	$0x0, 0x58(%rsp)
1417cbb6f: 48 8d 44 24 78              	leaq	0x78(%rsp), %rax
1417cbb74: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
1417cbb79: 48 c7 44 24 50 02 00 00 00  	movq	$0x2, 0x50(%rsp)
1417cbb82: 48 8d 15 37 75 51 01        	leaq	0x1517537(%rip), %rdx   # 0x142ce30c0
1417cbb89: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
1417cbb8e: e8 fd d7 10 01              	callq	0x1428d9390 <.text+0x28d8390>
1417cbb93: 48 81 c6 b8 00 00 00        	addq	$0xb8, %rsi
1417cbb9a: 48 8d 05 0d 73 51 01        	leaq	0x151730d(%rip), %rax   # 0x142ce2eae
1417cbba1: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1417cbba6: 48 c7 44 24 30 2f 00 00 00  	movq	$0x2f, 0x30(%rsp)
1417cbbaf: 48 89 74 24 78              	movq	%rsi, 0x78(%rsp)
1417cbbb4: 48 8d 05 15 ff 86 fe        	leaq	-0x17900eb(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
1417cbbbb: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1417cbbc3: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
1417cbbc8: 48 89 84 24 88 00 00 00     	movq	%rax, 0x88(%rsp)
1417cbbd0: 48 8d 05 b9 e2 93 fe        	leaq	-0x16c1d47(%rip), %rax  # 0x140109e90 <.text+0x108e90>
1417cbbd7: 48 89 84 24 90 00 00 00     	movq	%rax, 0x90(%rsp)
1417cbbdf: 48 8d 05 c2 71 51 01        	leaq	0x15171c2(%rip), %rax   # 0x142ce2da8
1417cbbe6: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1417cbbeb: 48 c7 44 24 40 02 00 00 00  	movq	$0x2, 0x40(%rsp)
1417cbbf4: 48 c7 44 24 58 00 00 00 00  	movq	$0x0, 0x58(%rsp)
1417cbbfd: 48 8d 44 24 78              	leaq	0x78(%rsp), %rax
1417cbc02: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
1417cbc07: 48 c7 44 24 50 02 00 00 00  	movq	$0x2, 0x50(%rsp)
1417cbc10: 48 8d 15 19 72 51 01        	leaq	0x1517219(%rip), %rdx   # 0x142ce2e30
1417cbc17: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
1417cbc1c: e8 6f d7 10 01              	callq	0x1428d9390 <.text+0x28d8390>
1417cbc21: 48 8d 0d d8 78 51 01        	leaq	0x15178d8(%rip), %rcx   # 0x142ce3500
1417cbc28: 4c 8d 05 31 79 51 01        	leaq	0x1517931(%rip), %r8    # 0x142ce3560
1417cbc2f: ba 59 00 00 00              	movl	$0x59, %edx
1417cbc34: e8 f7 d6 10 01              	callq	0x1428d9330 <.text+0x28d8330>
1417cbc39: cc                          	int3

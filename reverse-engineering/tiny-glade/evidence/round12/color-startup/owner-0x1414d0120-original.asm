
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1414d0120: 56                          	pushq	%rsi
1414d0121: 57                          	pushq	%rdi
1414d0122: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1414d0129: f6 01 01                    	testb	$0x1, (%rcx)
1414d012c: 0f 84 c0 02 00 00           	je	0x1414d03f2 <.text+0x14cf3f2>
1414d0132: 48 89 ce                    	movq	%rcx, %rsi
1414d0135: 48 8b 82 a0 05 00 00        	movq	0x5a0(%rdx), %rax
1414d013c: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
1414d0141: 48 39 41 18                 	cmpq	%rax, 0x18(%rcx)
1414d0145: 0f 85 2c 01 00 00           	jne	0x1414d0277 <.text+0x14cf277>
1414d014b: 48 8b 82 08 01 00 00        	movq	0x108(%rdx), %rax
1414d0152: 8b 8e 58 01 00 00           	movl	0x158(%rsi), %ecx
1414d0158: 89 86 58 01 00 00           	movl	%eax, 0x158(%rsi)
1414d015e: 48 39 c8                    	cmpq	%rcx, %rax
1414d0161: 0f 82 5a 01 00 00           	jb	0x1414d02c1 <.text+0x14cf2c1>
1414d0167: bf 01 00 00 00              	movl	$0x1, %edi
1414d016c: f0                          	lock
1414d016d: 0f c1 ba b0 05 00 00        	xaddl	%edi, 0x5b0(%rdx)
1414d0174: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1414d0178: 48 39 82 18 02 00 00        	cmpq	%rax, 0x218(%rdx)
1414d017f: 0f 86 4b 01 00 00           	jbe	0x1414d02d0 <.text+0x14cf2d0>
1414d0185: 48 8b 8a 10 02 00 00        	movq	0x210(%rdx), %rcx
1414d018c: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
1414d0190: 48 85 c9                    	testq	%rcx, %rcx
1414d0193: 0f 84 37 01 00 00           	je	0x1414d02d0 <.text+0x14cf2d0>
1414d0199: 48 8b 82 e0 01 00 00        	movq	0x1e0(%rdx), %rax
1414d01a0: 48 f7 d1                    	notq	%rcx
1414d01a3: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1414d01a7: 48 c1 e1 05                 	shlq	$0x5, %rcx
1414d01ab: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1414d01b1: 0f 84 19 01 00 00           	je	0x1414d02d0 <.text+0x14cf2d0>
1414d01b7: 48 01 c8                    	addq	%rcx, %rax
1414d01ba: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1414d01be: 44 8b 82 b0 05 00 00        	movl	0x5b0(%rdx), %r8d
1414d01c5: 4c 8b 46 10                 	movq	0x10(%rsi), %r8
1414d01c9: 4c 39 82 18 02 00 00        	cmpq	%r8, 0x218(%rdx)
1414d01d0: 0f 86 8b 01 00 00           	jbe	0x1414d0361 <.text+0x14cf361>
1414d01d6: 4c 8b 8a 10 02 00 00        	movq	0x210(%rdx), %r9
1414d01dd: 4f 8b 0c c1                 	movq	(%r9,%r8,8), %r9
1414d01e1: 4d 85 c9                    	testq	%r9, %r9
1414d01e4: 0f 84 77 01 00 00           	je	0x1414d0361 <.text+0x14cf361>
1414d01ea: 4c 8b 82 e0 01 00 00        	movq	0x1e0(%rdx), %r8
1414d01f1: 49 f7 d1                    	notq	%r9
1414d01f4: 4b 8d 14 49                 	leaq	(%r9,%r9,2), %rdx
1414d01f8: 48 c1 e2 05                 	shlq	$0x5, %rdx
1414d01fc: 49 83 7c 10 20 00           	cmpq	$0x0, 0x20(%r8,%rdx)
1414d0202: 0f 84 59 01 00 00           	je	0x1414d0361 <.text+0x14cf361>
1414d0208: 44 8b 8e 50 01 00 00        	movl	0x150(%rsi), %r9d
1414d020f: 49 01 d0                    	addq	%rdx, %r8
1414d0212: 48 8d 50 5c                 	leaq	0x5c(%rax), %rdx
1414d0216: 48 83 c0 58                 	addq	$0x58, %rax
1414d021a: 4d 8b 50 10                 	movq	0x10(%r8), %r10
1414d021e: 4d 8d 58 58                 	leaq	0x58(%r8), %r11
1414d0222: 49 83 c0 5c                 	addq	$0x5c, %r8
1414d0226: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
1414d022b: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1414d0230: 48 89 54 24 38              	movq	%rdx, 0x38(%rsp)
1414d0235: 44 89 4c 24 40              	movl	%r9d, 0x40(%rsp)
1414d023a: 89 7c 24 44                 	movl	%edi, 0x44(%rsp)
1414d023e: 48 8d 54 24 48              	leaq	0x48(%rsp), %rdx
1414d0243: 4c 89 54 24 48              	movq	%r10, 0x48(%rsp)
1414d0248: 4c 89 5c 24 50              	movq	%r11, 0x50(%rsp)
1414d024d: 4c 89 44 24 58              	movq	%r8, 0x58(%rsp)
1414d0252: 44 89 4c 24 60              	movl	%r9d, 0x60(%rsp)
1414d0257: 89 7c 24 64                 	movl	%edi, 0x64(%rsp)
1414d025b: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
1414d0260: e8 5b f7 3e ff              	callq	0x1408bf9c0 <.text+0x8be9c0>
1414d0265: 89 be 50 01 00 00           	movl	%edi, 0x150(%rsi)
1414d026b: 31 c0                       	xorl	%eax, %eax
1414d026d: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1414d0274: 5f                          	popq	%rdi
1414d0275: 5e                          	popq	%rsi
1414d0276: c3                          	retq
1414d0277: 48 8d 05 e2 76 78 01        	leaq	0x17876e2(%rip), %rax   # 0x142c57960
1414d027e: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1414d0283: 48 c7 44 24 30 01 00 00 00  	movq	$0x1, 0x30(%rsp)
1414d028c: 48 c7 44 24 38 08 00 00 00  	movq	$0x8, 0x38(%rsp)
1414d0295: 0f 57 c0                    	xorps	%xmm0, %xmm0
1414d0298: 0f 11 44 24 40              	movups	%xmm0, 0x40(%rsp)
1414d029d: 48 83 c6 18                 	addq	$0x18, %rsi
1414d02a1: 48 8d 05 c8 76 78 01        	leaq	0x17876c8(%rip), %rax   # 0x142c57970
1414d02a8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1414d02ad: 4c 8d 44 24 68              	leaq	0x68(%rsp), %r8
1414d02b2: 4c 8d 4c 24 28              	leaq	0x28(%rsp), %r9
1414d02b7: 31 c9                       	xorl	%ecx, %ecx
1414d02b9: 48 89 f2                    	movq	%rsi, %rdx
1414d02bc: e8 20 de 3f 01              	callq	0x1428ce0e1 <.text+0x28cd0e1>
1414d02c1: 4c 8d 05 c0 76 78 01        	leaq	0x17876c0(%rip), %r8    # 0x142c57988
1414d02c8: 48 89 c2                    	movq	%rax, %rdx
1414d02cb: e8 60 95 40 01              	callq	0x1428d9830 <.text+0x28d8830>
1414d02d0: 48 81 c6 b8 00 00 00        	addq	$0xb8, %rsi
1414d02d7: 48 8d 05 9f 6b 79 01        	leaq	0x1796b9f(%rip), %rax   # 0x142c66e7d
1414d02de: 48 89 84 24 88 00 00 00     	movq	%rax, 0x88(%rsp)
1414d02e6: 48 c7 84 24 90 00 00 00 3f 00 00 00 	movq	$0x3f, 0x90(%rsp)
1414d02f2: 48 89 74 24 68              	movq	%rsi, 0x68(%rsp)
1414d02f7: 48 8d 05 d2 b7 b6 fe        	leaq	-0x149482e(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
1414d02fe: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
1414d0303: 48 8d 84 24 88 00 00 00     	leaq	0x88(%rsp), %rax
1414d030b: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
1414d0310: 48 8d 05 79 9b c3 fe        	leaq	-0x13c6487(%rip), %rax  # 0x140109e90 <.text+0x108e90>
1414d0317: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1414d031f: 48 8d 05 5a 5b 79 01        	leaq	0x1795b5a(%rip), %rax   # 0x142c65e80
1414d0326: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1414d032b: 48 c7 44 24 30 02 00 00 00  	movq	$0x2, 0x30(%rsp)
1414d0334: 48 c7 44 24 48 00 00 00 00  	movq	$0x0, 0x48(%rsp)
1414d033d: 48 8d 44 24 68              	leaq	0x68(%rsp), %rax
1414d0342: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1414d0347: 48 c7 44 24 40 02 00 00 00  	movq	$0x2, 0x40(%rsp)
1414d0350: 48 8d 15 e9 94 79 01        	leaq	0x17994e9(%rip), %rdx   # 0x142c69840
1414d0357: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
1414d035c: e8 2f 90 40 01              	callq	0x1428d9390 <.text+0x28d8390>
1414d0361: 48 81 c6 b8 00 00 00        	addq	$0xb8, %rsi
1414d0368: 48 8d 05 d0 71 79 01        	leaq	0x17971d0(%rip), %rax   # 0x142c6753f
1414d036f: 48 89 84 24 88 00 00 00     	movq	%rax, 0x88(%rsp)
1414d0377: 48 c7 84 24 90 00 00 00 2f 00 00 00 	movq	$0x2f, 0x90(%rsp)
1414d0383: 48 89 74 24 68              	movq	%rsi, 0x68(%rsp)
1414d0388: 48 8d 05 41 b7 b6 fe        	leaq	-0x14948bf(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
1414d038f: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
1414d0394: 48 8d 84 24 88 00 00 00     	leaq	0x88(%rsp), %rax
1414d039c: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
1414d03a1: 48 8d 05 e8 9a c3 fe        	leaq	-0x13c6518(%rip), %rax  # 0x140109e90 <.text+0x108e90>
1414d03a8: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1414d03b0: 48 8d 05 c9 5a 79 01        	leaq	0x1795ac9(%rip), %rax   # 0x142c65e80
1414d03b7: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1414d03bc: 48 c7 44 24 30 02 00 00 00  	movq	$0x2, 0x30(%rsp)
1414d03c5: 48 c7 44 24 48 00 00 00 00  	movq	$0x0, 0x48(%rsp)
1414d03ce: 48 8d 44 24 68              	leaq	0x68(%rsp), %rax
1414d03d3: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1414d03d8: 48 c7 44 24 40 02 00 00 00  	movq	$0x2, 0x40(%rsp)
1414d03e1: 48 8d 15 20 5b 79 01        	leaq	0x1795b20(%rip), %rdx   # 0x142c65f08
1414d03e8: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
1414d03ed: e8 9e 8f 40 01              	callq	0x1428d9390 <.text+0x28d8390>
1414d03f2: 48 8d 0d b1 27 78 01        	leaq	0x17827b1(%rip), %rcx   # 0x142c52baa
1414d03f9: 4c 8d 05 d8 74 78 01        	leaq	0x17874d8(%rip), %r8    # 0x142c578d8
1414d0400: ba 59 00 00 00              	movl	$0x59, %edx
1414d0405: e8 26 8f 40 01              	callq	0x1428d9330 <.text+0x28d8330>
1414d040a: cc                          	int3

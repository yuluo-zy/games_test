
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407dd7c0: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1407dd7c4: 4e 89 3c f0                 	movq	%r15, (%rax,%r14,8)
1407dd7c8: 49 ff c6                    	incq	%r14
1407dd7cb: 4c 89 76 10                 	movq	%r14, 0x10(%rsi)
1407dd7cf: 48 83 c7 20                 	addq	$0x20, %rdi
1407dd7d3: 49 39 fd                    	cmpq	%rdi, %r13
1407dd7d6: 0f 84 ca 02 00 00           	je	0x1407ddaa6 <.text+0x7dcaa6>
1407dd7dc: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
1407dd7e3: 0f 10 04 38                 	movups	(%rax,%rdi), %xmm0
1407dd7e7: 0f 10 4c 38 10              	movups	0x10(%rax,%rdi), %xmm1
1407dd7ec: 0f 29 4d e0                 	movaps	%xmm1, -0x20(%rbp)
1407dd7f0: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
1407dd7f4: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
1407dd7f8: e8 c3 03 ac 00              	callq	0x14129dbc0 <.text+0x129cbc0>
1407dd7fd: 89 c0                       	movl	%eax, %eax
1407dd7ff: b9 ab aa aa aa              	movl	$0xaaaaaaab, %ecx       # imm = 0xAAAAAAAB
1407dd804: 48 0f af c1                 	imulq	%rcx, %rax
1407dd808: 48 c1 e8 21                 	shrq	$0x21, %rax
1407dd80c: 83 e8 01                    	subl	$0x1, %eax
1407dd80f: b9 00 00 00 00              	movl	$0x0, %ecx
1407dd814: 0f 42 c1                    	cmovbl	%ecx, %eax
1407dd817: 85 c0                       	testl	%eax, %eax
1407dd819: 0f 84 11 01 00 00           	je	0x1407dd930 <.text+0x7dc930>
1407dd81f: 0f bd c0                    	bsrl	%eax, %eax
1407dd822: 83 f0 1f                    	xorl	$0x1f, %eax
1407dd825: 31 c9                       	xorl	%ecx, %ecx
1407dd827: ba 1e 00 00 00              	movl	$0x1e, %edx
1407dd82c: 29 c2                       	subl	%eax, %edx
1407dd82e: 72 02                       	jb	0x1407dd832 <.text+0x7dc832>
1407dd830: 89 d1                       	movl	%edx, %ecx
1407dd832: e8 19 5a 05 00              	callq	0x140833250 <.text+0x832250>
1407dd837: 48 89 d9                    	movq	%rbx, %rcx
1407dd83a: 89 c2                       	movl	%eax, %edx
1407dd83c: e8 1f e5 04 00              	callq	0x14082bd60 <.text+0x82ad60>
1407dd841: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
1407dd848: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
1407dd84f: e8 2c e3 85 ff              	callq	0x14003bb80 <.text+0x3ab80>
1407dd854: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
1407dd85b: 0f 29 85 d0 00 00 00        	movaps	%xmm0, 0xd0(%rbp)
1407dd862: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
1407dd869: f0                          	lock
1407dd86a: 48 ff 00                    	incq	(%rax)
1407dd86d: 0f 8e b9 04 00 00           	jle	0x1407ddd2c <.text+0x7dcd2c>
1407dd873: 0f 28 45 d0                 	movaps	-0x30(%rbp), %xmm0
1407dd877: 0f 28 4d e0                 	movaps	-0x20(%rbp), %xmm1
1407dd87b: 41 0f 11 4c 24 4c           	movups	%xmm1, 0x4c(%r12)
1407dd881: 41 0f 11 44 24 3c           	movups	%xmm0, 0x3c(%r12)
1407dd887: 0f 29 b5 40 01 00 00        	movaps	%xmm6, 0x140(%rbp)
1407dd88e: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1407dd895: c7 41 08 00 00 00 00        	movl	$0x0, 0x8(%rcx)
1407dd89c: 48 c7 01 00 00 00 00        	movq	$0x0, (%rcx)
1407dd8a3: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
1407dd8ad: 49 89 0c 24                 	movq	%rcx, (%r12)
1407dd8b1: 41 c7 44 24 08 00 00 80 3f  	movl	$0x3f800000, 0x8(%r12)  # imm = 0x3F800000
1407dd8ba: 0f 28 85 d0 00 00 00        	movaps	0xd0(%rbp), %xmm0
1407dd8c1: 41 0f 29 44 24 14           	movaps	%xmm0, 0x14(%r12)
1407dd8c7: c6 85 80 01 00 00 1a        	movb	$0x1a, 0x180(%rbp)
1407dd8ce: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407dd8d5: 8b 85 d8 03 00 00           	movl	0x3d8(%rbp), %eax
1407dd8db: 89 85 b8 01 00 00           	movl	%eax, 0x1b8(%rbp)
1407dd8e1: 48 89 d9                    	movq	%rbx, %rcx
1407dd8e4: 48 8b 95 d0 03 00 00        	movq	0x3d0(%rbp), %rdx
1407dd8eb: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
1407dd8f2: 4c 8d 0d ff ab 2a 02        	leaq	0x22aabff(%rip), %r9    # 0x142a884f8
1407dd8f9: e8 d2 09 a9 00              	callq	0x14126e2d0 <.text+0x126d2d0>
1407dd8fe: 4c 8b bd f0 00 00 00        	movq	0xf0(%rbp), %r15
1407dd905: 4c 8b 76 10                 	movq	0x10(%rsi), %r14
1407dd909: 4c 3b 36                    	cmpq	(%rsi), %r14
1407dd90c: 0f 85 ae fe ff ff           	jne	0x1407dd7c0 <.text+0x7dc7c0>
1407dd912: 48 89 f1                    	movq	%rsi, %rcx
1407dd915: 48 8d 15 f4 ab 2a 02        	leaq	0x22aabf4(%rip), %rdx   # 0x142a88510
1407dd91c: e8 cf 44 3a 00              	callq	0x140b81df0 <.text+0xb80df0>
1407dd921: e9 9a fe ff ff              	jmp	0x1407dd7c0 <.text+0x7dc7c0>
1407dd926: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407dd930: b8 20 00 00 00              	movl	$0x20, %eax
1407dd935: 31 c9                       	xorl	%ecx, %ecx
1407dd937: ba 1e 00 00 00              	movl	$0x1e, %edx
1407dd93c: 29 c2                       	subl	%eax, %edx
1407dd93e: 0f 83 ec fe ff ff           	jae	0x1407dd830 <.text+0x7dc830>
1407dd944: e9 e9 fe ff ff              	jmp	0x1407dd832 <.text+0x7dc832>
1407dd949: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
1407dd94d: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
1407dd954: 0f 10 45 30                 	movups	0x30(%rbp), %xmm0
1407dd958: 0f 10 4d 40                 	movups	0x40(%rbp), %xmm1
1407dd95c: 0f 10 55 50                 	movups	0x50(%rbp), %xmm2
1407dd960: 0f 10 5d 60                 	movups	0x60(%rbp), %xmm3
1407dd964: 0f 29 9d 00 01 00 00        	movaps	%xmm3, 0x100(%rbp)
1407dd96b: 0f 29 95 f0 00 00 00        	movaps	%xmm2, 0xf0(%rbp)
1407dd972: 0f 29 8d e0 00 00 00        	movaps	%xmm1, 0xe0(%rbp)
1407dd979: 0f 29 85 d0 00 00 00        	movaps	%xmm0, 0xd0(%rbp)
1407dd980: 48 8b 05 21 ea e4 02        	movq	0x2e4ea21(%rip), %rax   # 0x14362c3a8
1407dd987: 48 8b 00                    	movq	(%rax), %rax
1407dd98a: 48 85 c0                    	testq	%rax, %rax
1407dd98d: 0f 84 da 00 00 00           	je	0x1407dda6d <.text+0x7dca6d>


B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14075c290: 49 89 59 30                 	movq	%rbx, 0x30(%r9)
14075c294: 49 ff 41 40                 	incq	0x40(%r9)
14075c298: 44 0f 28 c7                 	movaps	%xmm7, %xmm8
14075c29c: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
14075c2a3: 8b 8d f8 00 00 00           	movl	0xf8(%rbp), %ecx
14075c2a9: 89 08                       	movl	%ecx, (%rax)
14075c2ab: 49 8b 17                    	movq	(%r15), %rdx
14075c2ae: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c2b5: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
14075c2bc: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
14075c2c0: e8 0b 08 aa 00              	callq	0x1411fcad0 <.text+0x11fbad0>
14075c2c5: 45 0f 2e 45 f0              	ucomiss	-0x10(%r13), %xmm8
14075c2ca: 89 f0                       	movl	%esi, %eax
14075c2cc: 75 02                       	jne	0x14075c2d0 <.text+0x75b2d0>
14075c2ce: 7b 35                       	jnp	0x14075c305 <.text+0x75b305>
14075c2d0: 49 8b 1f                    	movq	(%r15), %rbx
14075c2d3: 48 8b 7d 60                 	movq	0x60(%rbp), %rdi
14075c2d7: 48 3b 7d 50                 	cmpq	0x50(%rbp), %rdi
14075c2db: 75 17                       	jne	0x14075c2f4 <.text+0x75b2f4>
14075c2dd: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c2e4: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
14075c2e8: 48 8d 15 31 0e 32 02        	leaq	0x2320e31(%rip), %rdx   # 0x142a7d120
14075c2ef: e8 7c 65 e0 ff              	callq	0x140562870 <.text+0x561870>
14075c2f4: 48 8b 45 58                 	movq	0x58(%rbp), %rax
14075c2f8: 48 89 1c f8                 	movq	%rbx, (%rax,%rdi,8)
14075c2fc: 48 ff c7                    	incq	%rdi
14075c2ff: 48 89 7d 60                 	movq	%rdi, 0x60(%rbp)
14075c303: 89 f0                       	movl	%esi, %eax

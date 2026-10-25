
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404dec5e: 48 8b 47 28                 	movq	0x28(%rdi), %rax
1404dec62: 4c 89 34 d8                 	movq	%r14, (%rax,%rbx,8)
1404dec66: 48 ff c3                    	incq	%rbx
1404dec69: 48 89 5f 30                 	movq	%rbx, 0x30(%rdi)
1404dec6d: 48 ff 47 40                 	incq	0x40(%rdi)
1404dec71: 48 8d 8d 30 0c 00 00        	leaq	0xc30(%rbp), %rcx
1404dec78: 48 89 f2                    	movq	%rsi, %rdx
1404dec7b: e8 d0 90 ff ff              	callq	0x1404d7d50 <.text+0x4d6d50>
1404dec80: c6 85 74 14 00 00 01        	movb	$0x1, 0x1474(%rbp)
1404dec87: 48 8b 8d 88 14 00 00        	movq	0x1488(%rbp), %rcx
1404dec8e: 48 ba 03 61 49 a6 34 0c 83 fe       	movabsq	$-0x17cf3cb59b69efd, %rdx # imm = 0xFE830C34A6496103
1404dec98: 49 b8 8d 08 29 5f cc ba 6e d6       	movabsq	$-0x29914533a0d6f773, %r8 # imm = 0xD66EBACC5F29088D
1404deca2: e8 49 ea bf ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1404deca7: 48 85 c0                    	testq	%rax, %rax
1404decaa: 4c 8b 85 78 14 00 00        	movq	0x1478(%rbp), %r8
1404decb1: 48 8d b5 30 0c 00 00        	leaq	0xc30(%rbp), %rsi
1404decb8: 0f 84 07 05 00 00           	je	0x1404df1c5 <.text+0x4de1c5>
1404decbe: 49 39 90 48 02 00 00        	cmpq	%rdx, 0x248(%r8)
1404decc5: 0f 86 fa 04 00 00           	jbe	0x1404df1c5 <.text+0x4de1c5>
1404deccb: 49 8b 80 40 02 00 00        	movq	0x240(%r8), %rax
1404decd2: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1404decd6: 48 85 c9                    	testq	%rcx, %rcx
1404decd9: 0f 84 e6 04 00 00           	je	0x1404df1c5 <.text+0x4de1c5>
1404decdf: 49 8b 80 10 02 00 00        	movq	0x210(%r8), %rax
1404dece6: 48 f7 d1                    	notq	%rcx
1404dece9: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1404deced: 48 c1 e1 05                 	shlq	$0x5, %rcx
1404decf1: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1404decf7: 0f 84 c8 04 00 00           	je	0x1404df1c5 <.text+0x4de1c5>
1404decfd: 48 01 c8                    	addq	%rcx, %rax
1404ded00: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1404ded04: 41 8b 88 e0 05 00 00        	movl	0x5e0(%r8), %ecx
1404ded0b: 89 48 5c                    	movl	%ecx, 0x5c(%rax)
1404ded0e: 31 c0                       	xorl	%eax, %eax
1404ded10: 48 89 95 98 14 00 00        	movq	%rdx, 0x1498(%rbp)
1404ded17: 48 3b 02                    	cmpq	(%rdx), %rax
1404ded1a: 0f 80 90 aa ff ff           	jo	0x1404d97b0 <.text+0x4d87b0>
1404ded20: 48 8b 8d 98 14 00 00        	movq	0x1498(%rbp), %rcx
1404ded27: e8 54 ee 11 00              	callq	0x1405fdb80 <.text+0x5fcb80>
1404ded2c: e9 7f aa ff ff              	jmp	0x1404d97b0 <.text+0x4d87b0>

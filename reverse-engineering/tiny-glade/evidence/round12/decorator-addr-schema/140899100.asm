
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140899100: 56                          	pushq	%rsi
140899101: 48 83 ec 20                 	subq	$0x20, %rsp
140899105: 48 89 ce                    	movq	%rcx, %rsi
140899108: 4c 8b 49 08                 	movq	0x8(%rcx), %r9
14089910c: 4d 85 c9                    	testq	%r9, %r9
14089910f: 74 31                       	je	0x140899142 <.text+0x898142>
140899111: 49 8d 41 01                 	leaq	0x1(%r9), %rax
140899115: b9 58 00 00 00              	movl	$0x58, %ecx
14089911a: 48 f7 e1                    	mulq	%rcx
14089911d: 48 83 c0 0f                 	addq	$0xf, %rax
140899121: 48 83 e0 f0                 	andq	$-0x10, %rax
140899125: 49 01 c1                    	addq	%rax, %r9
140899128: 49 83 c1 11                 	addq	$0x11, %r9
14089912c: 74 14                       	je	0x140899142 <.text+0x898142>
14089912e: 48 8b 0e                    	movq	(%rsi), %rcx
140899131: 48 29 c1                    	subq	%rax, %rcx
140899134: 41 b8 10 00 00 00           	movl	$0x10, %r8d
14089913a: 4c 89 ca                    	movq	%r9, %rdx
14089913d: e8 de aa d7 ff              	callq	0x140613c20 <.text+0x612c20>
140899142: 4c 8b 4e 38                 	movq	0x38(%rsi), %r9
140899146: 4d 85 c9                    	testq	%r9, %r9
140899149: 74 37                       	je	0x140899182 <.text+0x898182>
14089914b: 49 8d 41 01                 	leaq	0x1(%r9), %rax
14089914f: b9 58 00 00 00              	movl	$0x58, %ecx
140899154: 48 f7 e1                    	mulq	%rcx
140899157: 48 83 c0 0f                 	addq	$0xf, %rax
14089915b: 48 83 e0 f0                 	andq	$-0x10, %rax
14089915f: 49 01 c1                    	addq	%rax, %r9
140899162: 49 83 c1 11                 	addq	$0x11, %r9
140899166: 74 1a                       	je	0x140899182 <.text+0x898182>
140899168: 48 8b 4e 30                 	movq	0x30(%rsi), %rcx
14089916c: 48 29 c1                    	subq	%rax, %rcx
14089916f: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140899175: 4c 89 ca                    	movq	%r9, %rdx
140899178: 48 83 c4 20                 	addq	$0x20, %rsp
14089917c: 5e                          	popq	%rsi
14089917d: e9 9e aa d7 ff              	jmp	0x140613c20 <.text+0x612c20>
140899182: 90                          	nop
140899183: 48 83 c4 20                 	addq	$0x20, %rsp
140899187: 5e                          	popq	%rsi
140899188: c3                          	retq

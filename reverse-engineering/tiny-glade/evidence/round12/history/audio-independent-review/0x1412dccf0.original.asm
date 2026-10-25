
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412dccf0: 56                          	pushq	%rsi
1412dccf1: 48 83 ec 20                 	subq	$0x20, %rsp
1412dccf5: 49 83 00 04                 	addq	$0x4, (%r8)
1412dccf9: 48 85 d2                    	testq	%rdx, %rdx
1412dccfc: 74 27                       	je	0x1412dcd25 <.text+0x12dbd25>
1412dccfe: 48 89 d6                    	movq	%rdx, %rsi
1412dcd01: 8b 09                       	movl	(%rcx), %ecx
1412dcd03: e8 38 a5 d7 00              	callq	0x142057240 <.text+0x2056240>
1412dcd08: 48 89 f1                    	movq	%rsi, %rcx
1412dcd0b: e8 a0 b4 a1 ff              	callq	0x140cf81b0 <.text+0xcf71b0>
1412dcd10: 48 89 f1                    	movq	%rsi, %rcx
1412dcd13: e8 38 b5 a1 ff              	callq	0x140cf8250 <.text+0xcf7250>
1412dcd18: 48 89 f1                    	movq	%rsi, %rcx
1412dcd1b: 48 83 c4 20                 	addq	$0x20, %rsp
1412dcd1f: 5e                          	popq	%rsi
1412dcd20: e9 db b4 a1 ff              	jmp	0x140cf8200 <.text+0xcf7200>
1412dcd25: 90                          	nop
1412dcd26: 48 83 c4 20                 	addq	$0x20, %rsp
1412dcd2a: 5e                          	popq	%rsi
1412dcd2b: c3                          	retq

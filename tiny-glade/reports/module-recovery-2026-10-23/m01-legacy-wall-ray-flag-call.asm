
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408007eb: 48 8d 8d a0 02 00 00        	leaq	0x2a0(%rbp), %rcx
1408007f2: 8b 41 08                    	movl	0x8(%rcx), %eax
1408007f5: 89 45 78                    	movl	%eax, 0x78(%rbp)
1408007f8: 48 8b 01                    	movq	(%rcx), %rax
1408007fb: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1408007ff: f3 0f 7e 05 f9 d4 1d 02     	movq	0x21dd4f9(%rip), %xmm0  # 0x1429ddd00
140800807: 66 0f d6 45 7c              	movq	%xmm0, 0x7c(%rbp)
14080080c: c7 85 84 00 00 00 00 00 00 80       	movl	$0x80000000, 0x84(%rbp) # imm = 0x80000000
140800816: 48 8d 8d 38 01 00 00        	leaq	0x138(%rbp), %rcx
14080081d: e8 1e ba 21 00              	callq	0x140a1c240 <.text+0xa1b240>
140800822: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140800827: c7 44 24 28 00 00 00 00     	movl	$0x0, 0x28(%rsp)
14080082f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140800835: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14080083c: 48 8b 55 38                 	movq	0x38(%rbp), %rdx
140800840: 4c 8d 45 70                 	leaq	0x70(%rbp), %r8
140800844: e8 27 bf 0c 00              	callq	0x1408cc770 <.text+0x8cb770>
140800849: 83 bd f0 01 00 00 02        	cmpl	$0x2, 0x1f0(%rbp)
140800850: 0f 85 59 0d 00 00           	jne	0x1408015af <.text+0x8005af>

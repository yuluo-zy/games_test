
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407e3b84: 80 fb 09                    	cmpb	$0x9, %bl
1407e3b87: 74 0b                       	je	0x1407e3b94 <.text+0x7e2b94>
1407e3b89: 44 89 ea                    	movl	%r13d, %edx
1407e3b8c: 84 c0                       	testb	%al, %al
1407e3b8e: 0f 84 99 fd ff ff           	je	0x1407e392d <.text+0x7e292d>
1407e3b94: 8b 85 8c 02 00 00           	movl	0x28c(%rbp), %eax
1407e3b9a: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
1407e3ba1: 89 01                       	movl	%eax, (%rcx)
1407e3ba3: 4c 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %r8
1407e3baa: 4c 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %r9
1407e3bb1: c6 44 24 20 00              	movb	$0x0, 0x20(%rsp)
1407e3bb6: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
1407e3bbd: 89 fa                       	movl	%edi, %edx
1407e3bbf: e8 ec 1c 04 00              	callq	0x1408258b0 <.text+0x8248b0>
1407e3bc4: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
1407e3bcb: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
1407e3bd2: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
1407e3bd9: 41 89 f8                    	movl	%edi, %r8d
1407e3bdc: 4c 8d 0d 45 54 2a 02        	leaq	0x22a5445(%rip), %r9    # 0x142a89028
1407e3be3: e8 38 6e 23 00              	callq	0x140a1aa20 <.text+0xa19a20>

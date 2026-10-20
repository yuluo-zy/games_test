
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a9fa80: 55                          	pushq	%rbp
140a9fa81: 56                          	pushq	%rsi
140a9fa82: 57                          	pushq	%rdi
140a9fa83: 53                          	pushq	%rbx
140a9fa84: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
140a9fa8b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a9fa93: 48 c7 45 60 fe ff ff ff     	movq	$-0x2, 0x60(%rbp)
140a9fa9b: 48 89 ce                    	movq	%rcx, %rsi
140a9fa9e: 48 85 d2                    	testq	%rdx, %rdx
140a9faa1: 0f 84 94 00 00 00           	je	0x140a9fb3b <.text+0xa9eb3b>
140a9faa7: 48 8b ba 18 01 00 00        	movq	0x118(%rdx), %rdi
140a9faae: 48 8d 5d b0                 	leaq	-0x50(%rbp), %rbx
140a9fab2: 48 89 d9                    	movq	%rbx, %rcx
140a9fab5: 48 89 f2                    	movq	%rsi, %rdx
140a9fab8: e8 43 65 0a 00              	callq	0x140b46000 <.text+0xb45000>
140a9fabd: 48 89 d9                    	movq	%rbx, %rcx
140a9fac0: e8 0b 74 2d 00              	callq	0x140d76ed0 <.text+0xd75ed0>
140a9fac5: 49 89 d0                    	movq	%rdx, %r8
140a9fac8: 48 c7 44 24 20 0c 00 00 00  	movq	$0xc, 0x20(%rsp)
140a9fad1: 4c 8d 0d c1 c7 03 02        	leaq	0x203c7c1(%rip), %r9    # 0x142adc299
140a9fad8: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
140a9fadc: 48 89 c2                    	movq	%rax, %rdx
140a9fadf: e8 2c 2d 2c 00              	callq	0x140d62810 <.text+0xd61810>
140a9fae4: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
140a9fae8: e8 e3 73 2d 00              	callq	0x140d76ed0 <.text+0xd75ed0>
140a9faed: 49 89 d0                    	movq	%rdx, %r8
140a9faf0: 48 83 c7 10                 	addq	$0x10, %rdi
140a9faf4: 48 89 f9                    	movq	%rdi, %rcx
140a9faf7: 48 89 c2                    	movq	%rax, %rdx
140a9fafa: e8 a1 06 c8 00              	callq	0x1417201a0 <.text+0x171f1a0>
140a9faff: 89 c3                       	movl	%eax, %ebx
140a9fb01: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
140a9fb05: 48 85 d2                    	testq	%rdx, %rdx
140a9fb08: 74 0f                       	je	0x140a9fb19 <.text+0xa9eb19>
140a9fb0a: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140a9fb0e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a9fb14: e8 07 41 b7 ff              	callq	0x140613c20 <.text+0x612c20>
140a9fb19: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
140a9fb1d: 48 85 d2                    	testq	%rdx, %rdx
140a9fb20: 74 0f                       	je	0x140a9fb31 <.text+0xa9eb31>
140a9fb22: 48 8b 4d b8                 	movq	-0x48(%rbp), %rcx
140a9fb26: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a9fb2c: e8 ef 40 b7 ff              	callq	0x140613c20 <.text+0x612c20>
140a9fb31: b0 01                       	movb	$0x1, %al
140a9fb33: 84 db                       	testb	%bl, %bl
140a9fb35: 0f 85 20 01 00 00           	jne	0x140a9fc5b <.text+0xa9ec5b>
140a9fb3b: 48 8d 7d 30                 	leaq	0x30(%rbp), %rdi
140a9fb3f: 48 89 f9                    	movq	%rdi, %rcx

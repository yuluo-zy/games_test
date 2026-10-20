
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408f8d40: ca 57 41                    	lretl	$0x4157                 # imm = 0x4157
1408f8d43: 89 f1                       	movl	%esi, %ecx
1408f8d45: e8 26 28 07 00              	callq	0x14096b570 <.text+0x96a570>
1408f8d4a: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1408f8d50: 48 8d 8d a0 06 00 00        	leaq	0x6a0(%rbp), %rcx
1408f8d57: 48 8d 95 a8 05 00 00        	leaq	0x5a8(%rbp), %rdx
1408f8d5e: e8 9d ae 02 02              	callq	0x142923c00 <.text+0x2922c00>
1408f8d63: 49 83 fd 03                 	cmpq	$0x3, %r13
1408f8d67: 72 27                       	jb	0x1408f8d90 <.text+0x8f7d90>
1408f8d69: 41 b1 01                    	movb	$0x1, %r9b
1408f8d6c: 80 3d 1d c0 d3 02 00        	cmpb	$0x0, 0x2d3c01d(%rip)   # 0x143634d90
1408f8d73: 74 1b                       	je	0x1408f8d90 <.text+0x8f7d90>
1408f8d75: 80 bd 7e 07 00 00 08        	cmpb	$0x8, 0x77e(%rbp)
1408f8d7c: 4c 8d 6d d0                 	leaq	-0x30(%rbp), %r13
1408f8d80: 75 24                       	jne	0x1408f8da6 <.text+0x8f7da6>
1408f8d82: eb 47                       	jmp	0x1408f8dcb <.text+0x8f7dcb>
1408f8d84: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1408f8d90: 41 80 7f 18 00              	cmpb	$0x0, 0x18(%r15)
1408f8d95: 41 0f 95 c1                 	setne	%r9b
1408f8d99: 80 bd 7e 07 00 00 08        	cmpb	$0x8, 0x77e(%rbp)
1408f8da0: 4c 8d 6d d0                 	leaq	-0x30(%rbp), %r13
1408f8da4: 74 25                       	je	0x1408f8dcb <.text+0x8f7dcb>
1408f8da6: 48 8b 95 88 07 00 00        	movq	0x788(%rbp), %rdx
1408f8dad: b8 e8 00 00 00              	movl	$0xe8, %eax
1408f8db2: 48 01 c2                    	addq	%rax, %rdx
1408f8db5: 48 8d 8d 58 07 00 00        	leaq	0x758(%rbp), %rcx
1408f8dbc: 49 b8 41 54 59 3e 31 11 b7 10       	movabsq	$0x10b711313e595441, %r8 # imm = 0x10B711313E595441
1408f8dc6: e8 85 2a 07 00              	callq	0x14096b850 <.text+0x96a850>
1408f8dcb: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1408f8dd1: 4c 89 e9                    	movq	%r13, %rcx
1408f8dd4: 48 8d 95 a0 06 00 00        	leaq	0x6a0(%rbp), %rdx
1408f8ddb: e8 20 ae 02 02              	callq	0x142923c00 <.text+0x2922c00>

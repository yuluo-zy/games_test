
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408f6c70: 49 89 ce                    	movq	%rcx, %r14
1408f6c73: 48 8d 72 38                 	leaq	0x38(%rdx), %rsi
1408f6c77: 48 ba 7a 9c b7 7f 61 ae 8d d3       	movabsq	$-0x2c72519e80486386, %rdx # imm = 0xD38DAE617FB79C7A
1408f6c81: 49 b8 ae 1e ad 5d 5c df aa 22       	movabsq	$0x22aadf5c5dad1eae, %r8 # imm = 0x22AADF5C5DAD1EAE
1408f6c8b: 48 89 f1                    	movq	%rsi, %rcx
1408f6c8e: e8 5d 6a 7e ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1408f6c93: 48 85 c0                    	testq	%rax, %rax
1408f6c96: 0f 84 02 2b 00 00           	je	0x1408f979e <.text+0x8f879e>
1408f6c9c: 48 39 93 18 02 00 00        	cmpq	%rdx, 0x218(%rbx)
1408f6ca3: 0f 86 f5 2a 00 00           	jbe	0x1408f979e <.text+0x8f879e>
1408f6ca9: 48 8b 83 10 02 00 00        	movq	0x210(%rbx), %rax
1408f6cb0: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1408f6cb4: 48 85 c9                    	testq	%rcx, %rcx
1408f6cb7: 0f 84 e1 2a 00 00           	je	0x1408f979e <.text+0x8f879e>
1408f6cbd: 48 8b 83 e0 01 00 00        	movq	0x1e0(%rbx), %rax
1408f6cc4: 48 f7 d1                    	notq	%rcx
1408f6cc7: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1408f6ccb: 48 c1 e1 05                 	shlq	$0x5, %rcx
1408f6ccf: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1408f6cd5: 0f 84 c3 2a 00 00           	je	0x1408f979e <.text+0x8f879e>
1408f6cdb: 48 01 c8                    	addq	%rcx, %rax
1408f6cde: 48 8b 40 10                 	movq	0x10(%rax), %rax
1408f6ce2: 48 89 85 08 10 00 00        	movq	%rax, 0x1008(%rbp)
1408f6ce9: 48 8b 00                    	movq	(%rax), %rax
1408f6cec: 48 83 c0 fe                 	addq	$-0x2, %rax
1408f6cf0: 48 83 f8 09                 	cmpq	$0x9, %rax
1408f6cf4: 0f 92 c1                    	setb	%cl
1408f6cf7: 48 83 f8 04                 	cmpq	$0x4, %rax
1408f6cfb: 0f 95 c0                    	setne	%al
1408f6cfe: 84 c1                       	testb	%al, %cl
1408f6d00: 0f 85 47 28 00 00           	jne	0x1408f954d <.text+0x8f854d>
1408f6d06: 48 89 d9                    	movq	%rbx, %rcx
1408f6d09: e8 a2 f1 ff ff              	callq	0x1408f5eb0 <.text+0x8f4eb0>
1408f6d0e: 48 85 c0                    	testq	%rax, %rax

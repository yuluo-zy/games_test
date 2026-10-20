
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408a5a70: 55                          	pushq	%rbp
1408a5a71: 41 57                       	pushq	%r15
1408a5a73: 41 56                       	pushq	%r14
1408a5a75: 56                          	pushq	%rsi
1408a5a76: 57                          	pushq	%rdi
1408a5a77: 53                          	pushq	%rbx
1408a5a78: 48 81 ec 28 09 00 00        	subq	$0x928, %rsp            # imm = 0x928
1408a5a7f: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1408a5a87: 48 c7 85 a0 08 00 00 fe ff ff ff    	movq	$-0x2, 0x8a0(%rbp)
1408a5a92: 4c 89 c3                    	movq	%r8, %rbx
1408a5a95: 48 89 d6                    	movq	%rdx, %rsi
1408a5a98: 48 89 cf                    	movq	%rcx, %rdi
1408a5a9b: 4c 8d 72 38                 	leaq	0x38(%rdx), %r14
1408a5a9f: 48 ba 7a 9c b7 7f 61 ae 8d d3       	movabsq	$-0x2c72519e80486386, %rdx # imm = 0xD38DAE617FB79C7A
1408a5aa9: 49 b8 ae 1e ad 5d 5c df aa 22       	movabsq	$0x22aadf5c5dad1eae, %r8 # imm = 0x22AADF5C5DAD1EAE
1408a5ab3: 4c 89 f1                    	movq	%r14, %rcx
1408a5ab6: e8 35 7c 83 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1408a5abb: 48 85 c0                    	testq	%rax, %rax
1408a5abe: 0f 84 b4 03 00 00           	je	0x1408a5e78 <.text+0x8a4e78>
1408a5ac4: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
1408a5acb: 0f 86 a7 03 00 00           	jbe	0x1408a5e78 <.text+0x8a4e78>
1408a5ad1: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
1408a5ad8: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1408a5adc: 48 85 c9                    	testq	%rcx, %rcx
1408a5adf: 0f 84 93 03 00 00           	je	0x1408a5e78 <.text+0x8a4e78>
1408a5ae5: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
1408a5aec: 48 f7 d1                    	notq	%rcx
1408a5aef: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1408a5af3: 48 c1 e1 05                 	shlq	$0x5, %rcx
1408a5af7: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1408a5afd: 0f 84 75 03 00 00           	je	0x1408a5e78 <.text+0x8a4e78>
1408a5b03: 48 01 c8                    	addq	%rcx, %rax
1408a5b06: 48 8b 40 10                 	movq	0x10(%rax), %rax
1408a5b0a: 83 38 08                    	cmpl	$0x8, (%rax)
1408a5b0d: 0f 85 54 03 00 00           	jne	0x1408a5e67 <.text+0x8a4e67>
1408a5b13: 48 ba ce 37 a9 f6 d5 94 e9 b7       	movabsq	$-0x48166b2a0956c832, %rdx # imm = 0xB7E994D5F6A937CE
1408a5b1d: 49 b8 20 05 bc 2c e2 04 9c 62       	movabsq	$0x629c04e22cbc0520, %r8 # imm = 0x629C04E22CBC0520
1408a5b27: 4c 89 f1                    	movq	%r14, %rcx
1408a5b2a: e8 c1 7b 83 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1408a5b2f: 48 85 c0                    	testq	%rax, %rax
1408a5b32: 0f 84 4c 03 00 00           	je	0x1408a5e84 <.text+0x8a4e84>
1408a5b38: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
1408a5b3f: 0f 86 3f 03 00 00           	jbe	0x1408a5e84 <.text+0x8a4e84>
1408a5b45: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
1408a5b4c: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1408a5b50: 48 85 c9                    	testq	%rcx, %rcx
1408a5b53: 0f 84 2b 03 00 00           	je	0x1408a5e84 <.text+0x8a4e84>
1408a5b59: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
1408a5b60: 48 f7 d1                    	notq	%rcx
1408a5b63: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1408a5b67: 48 c1 e1 05                 	shlq	$0x5, %rcx
1408a5b6b: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1408a5b71: 0f 84 0d 03 00 00           	je	0x1408a5e84 <.text+0x8a4e84>
1408a5b77: 48 01 c8                    	addq	%rcx, %rax
1408a5b7a: 4c 8b 78 10                 	movq	0x10(%rax), %r15
1408a5b7e: 41 80 7f 44 01              	cmpb	$0x1, 0x44(%r15)
1408a5b83: 0f 85 de 02 00 00           	jne	0x1408a5e67 <.text+0x8a4e67>
1408a5b89: 41 80 7f 3e 07              	cmpb	$0x7, 0x3e(%r15)
1408a5b8e: 0f 83 d3 02 00 00           	jae	0x1408a5e67 <.text+0x8a4e67>
1408a5b94: 48 ba b3 61 b4 27 07 ee 97 85       	movabsq	$-0x7a6811f8d84b9e4d, %rdx # imm = 0x8597EE0727B461B3
1408a5b9e: 49 b8 5c f1 15 ae c5 7e 29 8b       	movabsq	$-0x74d6813a51ea0ea4, %r8 # imm = 0x8B297EC5AE15F15C
1408a5ba8: 4c 89 f1                    	movq	%r14, %rcx
1408a5bab: e8 40 7b 83 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1408a5bb0: 48 85 c0                    	testq	%rax, %rax
1408a5bb3: 0f 84 d7 02 00 00           	je	0x1408a5e90 <.text+0x8a4e90>
1408a5bb9: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
1408a5bc0: 0f 86 ca 02 00 00           	jbe	0x1408a5e90 <.text+0x8a4e90>
1408a5bc6: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
1408a5bcd: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1408a5bd1: 48 85 c9                    	testq	%rcx, %rcx
1408a5bd4: 0f 84 b6 02 00 00           	je	0x1408a5e90 <.text+0x8a4e90>
1408a5bda: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
1408a5be1: 48 f7 d1                    	notq	%rcx
1408a5be4: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1408a5be8: 48 c1 e1 05                 	shlq	$0x5, %rcx
1408a5bec: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1408a5bf2: 0f 84 98 02 00 00           	je	0x1408a5e90 <.text+0x8a4e90>
1408a5bf8: 48 01 c8                    	addq	%rcx, %rax
1408a5bfb: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1408a5bff: 48 8d 15 4a 59 1f 02        	leaq	0x21f594a(%rip), %rdx   # 0x142a9b550
1408a5c06: 41 b8 18 00 00 00           	movl	$0x18, %r8d
1408a5c0c: e8 8f 1e 09 00              	callq	0x140937aa0 <.text+0x936aa0>
1408a5c11: a8 01                       	testb	$0x1, %al
1408a5c13: 0f 84 83 02 00 00           	je	0x1408a5e9c <.text+0x8a4e9c>

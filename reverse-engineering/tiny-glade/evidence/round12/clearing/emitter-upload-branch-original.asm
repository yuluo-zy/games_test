
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a79683: 4c 8b a5 d0 02 00 00        	movq	0x2d0(%rbp), %r12
140a7968a: 48 8b 95 e0 02 00 00        	movq	0x2e0(%rbp), %rdx
140a79691: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140a79694: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140a79698: 89 01                       	movl	%eax, (%rcx)
140a7969a: 48 8b 0a                    	movq	(%rdx), %rcx
140a7969d: c6 85 27 02 00 00 00        	movb	$0x0, 0x227(%rbp)
140a796a4: e8 d7 dc 84 00              	callq	0x1412c7380 <.text+0x12c6380>
140a796a9: 48 8b bd 38 01 00 00        	movq	0x138(%rbp), %rdi
140a796b0: 48 85 c0                    	testq	%rax, %rax
140a796b3: 0f 84 80 00 00 00           	je	0x140a79739 <.text+0xa78739>
140a796b9: 48 85 ff                    	testq	%rdi, %rdi
140a796bc: 74 7b                       	je	0x140a79739 <.text+0xa78739>
140a796be: 48 89 f9                    	movq	%rdi, %rcx
140a796c1: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a796c5: 48 3b 48 18                 	cmpq	0x18(%rax), %rcx
140a796c9: 0f 87 5b 05 00 00           	ja	0x140a79c2a <.text+0xa78c2a>
140a796cf: 4c 8b 8d 30 01 00 00        	movq	0x130(%rbp), %r9
140a796d6: 48 c7 85 50 01 00 00 00 00 00 00    	movq	$0x0, 0x150(%rbp)
140a796e1: 48 89 8d 58 01 00 00        	movq	%rcx, 0x158(%rbp)
140a796e8: c6 85 27 02 00 00 00        	movb	$0x0, 0x227(%rbp)
140a796ef: 48 8d 95 50 01 00 00        	leaq	0x150(%rbp), %rdx
140a796f6: 48 89 54 24 28              	movq	%rdx, 0x28(%rsp)
140a796fb: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
140a79700: 48 c7 44 24 38 00 00 00 00  	movq	$0x0, 0x38(%rsp)
140a79709: 48 c7 44 24 30 01 00 00 00  	movq	$0x1, 0x30(%rsp)
140a79712: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
140a79719: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
140a79720: 49 89 c0                    	movq	%rax, %r8
140a79723: e8 78 58 17 00              	callq	0x140beefa0 <.text+0xbedfa0>
140a79728: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
140a7972f: 48 83 f8 0b                 	cmpq	$0xb, %rax
140a79733: 0f 85 f8 04 00 00           	jne	0x140a79c31 <.text+0xa78c31>
140a79739: 48 81 ff 4c 04 00 00        	cmpq	$0x44c, %rdi            # imm = 0x44C
140a79740: b8 4c 04 00 00              	movl	$0x44c, %eax            # imm = 0x44C
140a79745: 48 0f 42 c7                 	cmovbq	%rdi, %rax
140a79749: 4c 8b 85 e8 02 00 00        	movq	0x2e8(%rbp), %r8
140a79750: 41 8b 48 1c                 	movl	0x1c(%r8), %ecx
140a79754: 49 8b 50 10                 	movq	0x10(%r8), %rdx
140a79758: 89 0a                       	movl	%ecx, (%rdx)
140a7975a: 49 8b 08                    	movq	(%r8), %rcx
140a7975d: 48 89 01                    	movq	%rax, (%rcx)

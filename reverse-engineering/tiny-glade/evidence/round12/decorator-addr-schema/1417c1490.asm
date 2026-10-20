
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1417c1490: 56                          	pushq	%rsi
1417c1491: 57                          	pushq	%rdi
1417c1492: 53                          	pushq	%rbx
1417c1493: 48 83 ec 40                 	subq	$0x40, %rsp
1417c1497: 48 89 ce                    	movq	%rcx, %rsi
1417c149a: 48 8b 3a                    	movq	(%rdx), %rdi
1417c149d: 48 8b 47 10                 	movq	0x10(%rdi), %rax
1417c14a1: 48 39 07                    	cmpq	%rax, (%rdi)
1417c14a4: 0f 84 8c 00 00 00           	je	0x1417c1536 <.text+0x17c0536>
1417c14aa: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
1417c14ae: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1417c14b2: 48 ff c0                    	incq	%rax
1417c14b5: 48 89 47 10                 	movq	%rax, 0x10(%rdi)
1417c14b9: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
1417c14be: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
1417c14c3: 48 8d 15 d9 06 52 01        	leaq	0x15206d9(%rip), %rdx   # 0x142ce1ba3
1417c14ca: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1417c14cf: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1417c14d5: 49 89 f1                    	movq	%rsi, %r9
1417c14d8: e8 93 58 dc ff              	callq	0x141586d70 <.text+0x1585d70>
1417c14dd: 48 85 c0                    	testq	%rax, %rax
1417c14e0: 75 4c                       	jne	0x1417c152e <.text+0x17c052e>
1417c14e2: 48 83 c6 30                 	addq	$0x30, %rsi
1417c14e6: 48 8d 15 bc 06 52 01        	leaq	0x15206bc(%rip), %rdx   # 0x142ce1ba9
1417c14ed: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1417c14f2: 41 b8 05 00 00 00           	movl	$0x5, %r8d
1417c14f8: 49 89 f1                    	movq	%rsi, %r9
1417c14fb: e8 70 58 dc ff              	callq	0x141586d70 <.text+0x1585d70>
1417c1500: 48 85 c0                    	testq	%rax, %rax
1417c1503: 75 29                       	jne	0x1417c152e <.text+0x17c052e>
1417c1505: 80 7c 24 38 00              	cmpb	$0x0, 0x38(%rsp)
1417c150a: 74 20                       	je	0x1417c152c <.text+0x17c052c>
1417c150c: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
1417c1511: 48 8b 30                    	movq	(%rax), %rsi
1417c1514: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
1417c1518: 48 39 16                    	cmpq	%rdx, (%rsi)
1417c151b: 74 48                       	je	0x1417c1565 <.text+0x17c0565>
1417c151d: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1417c1521: c6 04 10 7d                 	movb	$0x7d, (%rax,%rdx)
1417c1525: 48 ff c2                    	incq	%rdx
1417c1528: 48 89 56 10                 	movq	%rdx, 0x10(%rsi)
1417c152c: 31 c0                       	xorl	%eax, %eax
1417c152e: 48 83 c4 40                 	addq	$0x40, %rsp
1417c1532: 5b                          	popq	%rbx
1417c1533: 5f                          	popq	%rdi
1417c1534: 5e                          	popq	%rsi
1417c1535: c3                          	retq
1417c1536: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1417c153f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1417c1545: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1417c154b: 48 89 f9                    	movq	%rdi, %rcx
1417c154e: 48 89 d3                    	movq	%rdx, %rbx
1417c1551: 48 89 c2                    	movq	%rax, %rdx
1417c1554: e8 07 85 0e 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1417c1559: 48 89 da                    	movq	%rbx, %rdx
1417c155c: 48 8b 47 10                 	movq	0x10(%rdi), %rax
1417c1560: e9 45 ff ff ff              	jmp	0x1417c14aa <.text+0x17c04aa>
1417c1565: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1417c156e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1417c1574: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1417c157a: 48 89 f1                    	movq	%rsi, %rcx
1417c157d: e8 de 84 0e 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1417c1582: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
1417c1586: eb 95                       	jmp	0x1417c151d <.text+0x17c051d>

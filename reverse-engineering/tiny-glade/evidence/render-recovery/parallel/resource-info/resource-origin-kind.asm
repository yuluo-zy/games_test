D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140bd6d10: 48 83 ec 58                 	subq	$0x58, %rsp
140bd6d14: 48 8b 01                    	movq	(%rcx), %rax
140bd6d17: 48 8d 15 ba 95 f3 01        	leaq	0x1f395ba(%rip), %rdx   # 0x142b102d8
140bd6d1e: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
140bd6d22: 48 01 d0                    	addq	%rdx, %rax
140bd6d25: ff e0                       	jmpq	*%rax
140bd6d27: 0f b6 41 08                 	movzbl	0x8(%rcx), %eax
140bd6d2b: 48 83 c4 58                 	addq	$0x58, %rsp
140bd6d2f: c3                          	retq
140bd6d30: 0f b6 49 36                 	movzbl	0x36(%rcx), %ecx
140bd6d34: 8d 51 f9                    	leal	-0x7(%rcx), %edx
140bd6d37: 48 83 c1 fa                 	addq	$-0x6, %rcx
140bd6d3b: 31 c0                       	xorl	%eax, %eax
140bd6d3d: 80 fa 02                    	cmpb	$0x2, %dl
140bd6d40: 48 0f 43 c8                 	cmovaeq	%rax, %rcx
140bd6d44: 48 85 c9                    	testq	%rcx, %rcx
140bd6d47: 74 e2                       	je	0x140bd6d2b <.text+0xbd5d2b>
140bd6d49: 48 83 f9 01                 	cmpq	$0x1, %rcx
140bd6d4d: 75 1f                       	jne	0x140bd6d6e <.text+0xbd5d6e>
140bd6d4f: b0 01                       	movb	$0x1, %al
140bd6d51: 48 83 c4 58                 	addq	$0x58, %rsp
140bd6d55: c3                          	retq
140bd6d56: 48 8d 0d 6b 94 f3 01        	leaq	0x1f3946b(%rip), %rcx   # 0x142b101c8
140bd6d5d: 4c 8d 05 44 95 f3 01        	leaq	0x1f39544(%rip), %r8    # 0x142b102a8
140bd6d64: ba 28 00 00 00              	movl	$0x28, %edx
140bd6d69: e8 c2 26 d0 01              	callq	0x1428d9430 <.text+0x28d8430>
140bd6d6e: 48 8d 05 e3 94 f3 01        	leaq	0x1f394e3(%rip), %rax   # 0x142b10258
140bd6d75: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140bd6d7a: 48 c7 44 24 30 01 00 00 00  	movq	$0x1, 0x30(%rsp)
140bd6d83: 48 8d 44 24 20              	leaq	0x20(%rsp), %rax
140bd6d88: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140bd6d8d: 0f 57 c0                    	xorps	%xmm0, %xmm0
140bd6d90: 0f 11 44 24 40              	movups	%xmm0, 0x40(%rsp)
140bd6d95: 48 8d 15 f4 94 f3 01        	leaq	0x1f394f4(%rip), %rdx   # 0x142b10290
140bd6d9c: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
140bd6da1: e8 ea 25 d0 01              	callq	0x1428d9390 <.text+0x28d8390>
140bd6da6: cc                          	int3

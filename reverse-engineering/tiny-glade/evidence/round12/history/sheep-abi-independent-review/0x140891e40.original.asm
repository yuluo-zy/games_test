
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140891e40: 56                          	pushq	%rsi
140891e41: 48 83 ec 20                 	subq	$0x20, %rsp
140891e45: 48 8b 11                    	movq	(%rcx), %rdx
140891e48: 48 89 d0                    	movq	%rdx, %rax
140891e4b: 48 f7 d8                    	negq	%rax
140891e4e: 70 22                       	jo	0x140891e72 <.text+0x890e72>
140891e50: 73 20                       	jae	0x140891e72 <.text+0x890e72>
140891e52: 48 8b 41 08                 	movq	0x8(%rcx), %rax
140891e56: 48 c1 e2 03                 	shlq	$0x3, %rdx
140891e5a: 48 8d 14 92                 	leaq	(%rdx,%rdx,4), %rdx
140891e5e: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140891e64: 48 89 ce                    	movq	%rcx, %rsi
140891e67: 48 89 c1                    	movq	%rax, %rcx
140891e6a: e8 b1 1d d8 ff              	callq	0x140613c20 <.text+0x612c20>
140891e6f: 48 89 f1                    	movq	%rsi, %rcx
140891e72: 48 8b 41 18                 	movq	0x18(%rcx), %rax
140891e76: 48 89 c2                    	movq	%rax, %rdx
140891e79: 48 f7 da                    	negq	%rdx
140891e7c: 70 1e                       	jo	0x140891e9c <.text+0x890e9c>
140891e7e: 73 1c                       	jae	0x140891e9c <.text+0x890e9c>
140891e80: 48 8b 49 20                 	movq	0x20(%rcx), %rcx
140891e84: 48 c1 e0 03                 	shlq	$0x3, %rax
140891e88: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
140891e8c: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140891e92: 48 83 c4 20                 	addq	$0x20, %rsp
140891e96: 5e                          	popq	%rsi
140891e97: e9 84 1d d8 ff              	jmp	0x140613c20 <.text+0x612c20>
140891e9c: 90                          	nop
140891e9d: 48 83 c4 20                 	addq	$0x20, %rsp
140891ea1: 5e                          	popq	%rsi
140891ea2: c3                          	retq


D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140943bd7:     	addb	%cl, 0x39(%rax)
140943bda:     	xchgl	%esi, %eax
140943bdb:     	sbbb	%al, (%rdx)
140943bdd:     	addb	%al, (%rax)
140943bdf:     	jbe	0x140943c99 <.text+0x942c99>
140943be5:     	movq	0x210(%rsi), %rax
140943bec:     	movq	(%rax,%rdx,8), %rcx
140943bf0:     	testq	%rcx, %rcx
140943bf3:     	je	0x140943c99 <.text+0x942c99>
140943bf9:     	movq	0x1e0(%rsi), %rax
140943c00:     	notq	%rcx
140943c03:     	leaq	(%rcx,%rcx,2), %rcx
140943c07:     	shlq	$0x5, %rcx
140943c0b:     	cmpq	$0x0, 0x20(%rax,%rcx)
140943c11:     	je	0x140943c99 <.text+0x942c99>
140943c17:     	addq	%rcx, %rax
140943c1a:     	movq	0x10(%rax), %rcx
140943c1e:     	movl	0x5b0(%rsi), %edx
140943c24:     	movl	%edx, 0x5c(%rax)
140943c27:     	callq	0x1416a0b70 <.text+0x169fb70>
140943c2c:     	movabsq	$0x2557ffe8a7986afe, %rdx # imm = 0x2557FFE8A7986AFE
140943c36:     	movabsq	$0x4932233c9b0e0737, %r8 # imm = 0x4932233C9B0E0737
140943c40:     	movq	%r14, %rcx
140943c43:     	callq	0x1400dd6f0 <.text+0xdc6f0>
140943c48:     	testq	%rax, %rax
140943c4b:     	je	0x140943c80 <.text+0x942c80>
140943c4d:     	cmpq	%rdx, 0x218(%rsi)
140943c54:     	jbe	0x140943c80 <.text+0x942c80>
140943c56:     	movq	0x210(%rsi), %rax
140943c5d:     	movq	(%rax,%rdx,8), %rax
140943c61:     	testq	%rax, %rax
140943c64:     	je	0x140943c80 <.text+0x942c80>
140943c66:     	movq	0x1e0(%rsi), %rcx
140943c6d:     	notq	%rax
140943c70:     	leaq	(%rax,%rax,2), %rax
140943c74:     	shlq	$0x5, %rax
140943c78:     	cmpq	$0x0, 0x20(%rcx,%rax)
140943c7e:     	jne	0x140943c88 <.text+0x942c88>
140943c80:     	movq	%rsi, %rcx
140943c83:     	callq	0x141513380 <.text+0x1512380>
140943c88:     	movq	%rsi, %rax


../B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140609eb0:     	pushq	%rsi
140609eb1:     	subq	$0x20, %rsp
140609eb5:     	movq	%rcx, %rsi
140609eb8:     	movq	(%rcx), %rax
140609ebb:     	testq	%rax, %rax
140609ebe:     	je	0x140609ed7 <.text+0x608ed7>
140609ec0:     	movq	0x8(%rsi), %rcx
140609ec4:     	shlq	$0x2, %rax
140609ec8:     	leaq	(%rax,%rax,2), %rdx
140609ecc:     	movl	$0x4, %r8d
140609ed2:     	callq	0x140613c20 <.text+0x612c20>
140609ed7:     	movq	0x18(%rsi), %rdx
140609edb:     	testq	%rdx, %rdx
140609ede:     	je	0x140609ef3 <.text+0x608ef3>
140609ee0:     	movq	0x20(%rsi), %rcx
140609ee4:     	shlq	$0x2, %rdx
140609ee8:     	movl	$0x4, %r8d
140609eee:     	callq	0x140613c20 <.text+0x612c20>
140609ef3:     	movq	0x38(%rsi), %rdx
140609ef7:     	testq	%rdx, %rdx
140609efa:     	je	0x140609f0f <.text+0x608f0f>
140609efc:     	movq	0x40(%rsi), %rcx
140609f00:     	shlq	$0x3, %rdx
140609f04:     	movl	$0x4, %r8d
140609f0a:     	callq	0x140613c20 <.text+0x612c20>
140609f0f:     	movq	0x50(%rsi), %rdx
140609f13:     	testq	%rdx, %rdx
140609f16:     	je	0x140609f2b <.text+0x608f2b>
140609f18:     	movq	0x58(%rsi), %rcx
140609f1c:     	shlq	$0x2, %rdx
140609f20:     	movl	$0x4, %r8d
140609f26:     	callq	0x140613c20 <.text+0x612c20>
140609f2b:     	movq	0x68(%rsi), %rdx
140609f2f:     	testq	%rdx, %rdx
140609f32:     	je	0x140609f4c <.text+0x608f4c>
140609f34:     	movq	0x70(%rsi), %rcx
140609f38:     	shlq	$0x2, %rdx
140609f3c:     	movl	$0x4, %r8d
140609f42:     	addq	$0x20, %rsp
140609f46:     	popq	%rsi
140609f47:     	jmp	0x140613c20 <.text+0x612c20>
140609f4c:     	nop
140609f4d:     	addq	$0x20, %rsp
140609f51:     	popq	%rsi
140609f52:     	retq

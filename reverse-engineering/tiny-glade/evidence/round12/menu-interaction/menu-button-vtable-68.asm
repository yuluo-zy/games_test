
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14067ea50:     	pushq	%r14
14067ea52:     	pushq	%rsi
14067ea53:     	pushq	%rdi
14067ea54:     	pushq	%rbx
14067ea55:     	subq	$0x28, %rsp
14067ea59:     	movq	%rdx, %rdi
14067ea5c:     	movq	%rcx, %rsi
14067ea5f:     	movl	$0xffffffff, %ecx       # imm = 0xFFFFFFFF
14067ea64:     	testb	$0x1, 0x2c(%rdx)
14067ea68:     	jne	0x14067ea79 <.text+0x67da79>
14067ea6a:     	cmpb	$0x0, 0x25c(%rsi)
14067ea71:     	jne	0x14067ea79 <.text+0x67da79>
14067ea73:     	movl	0x258(%rsi), %ecx
14067ea79:     	movq	0x8(%rdi), %rbx
14067ea7d:     	movq	0x10(%rdi), %r14
14067ea81:     	movss	0x18(%rdi), %xmm1
14067ea86:     	callq	0x140c918d0 <.text+0xc908d0>
14067ea8b:     	movq	(%rdi), %rcx
14067ea8e:     	movq	%rcx, 0x20(%rsp)
14067ea93:     	movq	%rsi, %rcx
14067ea96:     	movq	%rbx, %rdx
14067ea99:     	movq	%r14, %r8
14067ea9c:     	movl	%eax, %r9d
14067ea9f:     	callq	0x140b91120 <.text+0xb90120>
14067eaa4:     	nop
14067eaa5:     	addq	$0x28, %rsp
14067eaa9:     	popq	%rbx
14067eaaa:     	popq	%rdi
14067eaab:     	popq	%rsi
14067eaac:     	popq	%r14
14067eaae:     	retq

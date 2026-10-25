
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141e23b90:     	pushq	%rsi
141e23b91:     	subq	$0x60, %rsp
141e23b95:     	movq	%rcx, %rax
141e23b98:     	movq	%rdx, 0x28(%rsp)
141e23b9d:     	movq	%r8, 0x30(%rsp)
141e23ba2:     	movq	$0x0, 0x20(%rsp)
141e23bab:     	leaq	0x116481e(%rip), %r8    # 0x142f883d0
141e23bb2:     	leaq	0x38(%rsp), %rsi
141e23bb7:     	movl	$0x16, %r9d
141e23bbd:     	movq	%rsi, %rcx
141e23bc0:     	movq	%rax, %rdx
141e23bc3:     	callq	0x140f9e970 <.text+0xf9d970>
141e23bc8:     	leaq	0x11647a3(%rip), %rdx   # 0x142f88372
141e23bcf:     	movl	$0x6, %r8d
141e23bd5:     	movq	%rsi, %rcx
141e23bd8:     	callq	0x140fbc5b0 <.text+0xfbb5b0>
141e23bdd:     	leaq	0x1164802(%rip), %rdx   # 0x142f883e6
141e23be4:     	leaq	0x28(%rsp), %r9
141e23be9:     	movl	$0x5, %r8d
141e23bef:     	movq	%rsi, %rcx
141e23bf2:     	callq	0x140fba940 <.text+0xfb9940>
141e23bf7:     	nop
141e23bf8:     	addq	$0x60, %rsp
141e23bfc:     	popq	%rsi
141e23bfd:     	retq

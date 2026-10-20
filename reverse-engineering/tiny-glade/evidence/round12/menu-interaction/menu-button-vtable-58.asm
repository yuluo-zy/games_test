
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14067ea20:     	pushq	%rsi
14067ea21:     	subq	$0x20, %rsp
14067ea25:     	movq	%rdx, %rsi
14067ea28:     	movzbl	0x25d(%rcx), %edx
14067ea2f:     	cmpb	$0x28, %dl
14067ea32:     	je	0x14067ea47 <.text+0x67da47>
14067ea34:     	movq	0x8(%rsi), %rcx
14067ea38:     	callq	0x140946620 <.text+0x945620>
14067ea3d:     	testb	%al, %al
14067ea3f:     	je	0x14067ea47 <.text+0x67da47>
14067ea41:     	movq	(%rsi), %rax
14067ea44:     	orb	$0x20, (%rax)
14067ea47:     	addq	$0x20, %rsp
14067ea4b:     	popq	%rsi
14067ea4c:     	retq

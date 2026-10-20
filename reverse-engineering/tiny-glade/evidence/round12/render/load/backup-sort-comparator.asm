
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fe41e0:     	pushq	%rsi
140fe41e1:     	subq	$0x20, %rsp
140fe41e5:     	leaq	-0x1(%r8), %rax
140fe41e9:     	cmpq	%rdx, %rax
140fe41ec:     	jae	0x140fe42a9 <.text+0xfe32a9>
140fe41f2:     	cmpq	%rdx, %r8
140fe41f5:     	jne	0x140fe41fd <.text+0xfe31fd>
140fe41f7:     	addq	$0x20, %rsp
140fe41fb:     	popq	%rsi
140fe41fc:     	retq
140fe41fd:     	leaq	(%rdx,%rdx,2), %rax
140fe4201:     	shlq	$0x4, %rax
140fe4205:     	addq	%rcx, %rax
140fe4208:     	shlq	$0x4, %r8
140fe420c:     	leaq	(%r8,%r8,2), %rdx
140fe4210:     	leaq	(%rcx,%rdx), %r8
140fe4214:     	jmp	0x140fe424a <.text+0xfe324a>
140fe4216:     	nopw	%cs:(%rax,%rax)
140fe4220:     	movq	%rcx, %r11
140fe4223:     	movaps	(%rsp), %xmm0
140fe4227:     	movaps	0x10(%rsp), %xmm1
140fe422c:     	movups	%xmm1, 0x10(%r11)
140fe4231:     	movups	%xmm0, (%r11)
140fe4235:     	movq	%r9, 0x20(%r11)
140fe4239:     	movq	%r10, 0x28(%r11)
140fe423d:     	addq	$0x30, %r8
140fe4241:     	addq	$0x30, %rdx
140fe4245:     	cmpq	%rax, %r8
140fe4248:     	je	0x140fe41f7 <.text+0xfe31f7>
140fe424a:     	movq	0x20(%r8), %r9
140fe424e:     	cmpq	-0x10(%r8), %r9
140fe4252:     	jle	0x140fe423d <.text+0xfe323d>
140fe4254:     	movups	(%r8), %xmm0
140fe4258:     	movups	0x10(%r8), %xmm1
140fe425d:     	movaps	%xmm1, 0x10(%rsp)
140fe4262:     	movaps	%xmm0, (%rsp)
140fe4266:     	movq	0x28(%r8), %r10
140fe426a:     	movq	%rdx, %rsi
140fe426d:     	nopl	(%rax)
140fe4270:     	movups	-0x30(%rcx,%rsi), %xmm0
140fe4275:     	movups	-0x20(%rcx,%rsi), %xmm1
140fe427a:     	movups	-0x10(%rcx,%rsi), %xmm2
140fe427f:     	movups	%xmm2, 0x20(%rcx,%rsi)
140fe4284:     	movups	%xmm1, 0x10(%rcx,%rsi)
140fe4289:     	movups	%xmm0, (%rcx,%rsi)
140fe428d:     	cmpq	$0x30, %rsi
140fe4291:     	je	0x140fe4220 <.text+0xfe3220>
140fe4293:     	leaq	-0x30(%rsi), %r11
140fe4297:     	cmpq	-0x40(%rcx,%rsi), %r9
140fe429c:     	movq	%r11, %rsi
140fe429f:     	jg	0x140fe4270 <.text+0xfe3270>
140fe42a1:     	addq	%rcx, %r11
140fe42a4:     	jmp	0x140fe4223 <.text+0xfe3223>
140fe42a9:     	ud2


reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140839520: 48 89 c8                    	movq	%rcx, %rax
140839523: f3 0f 11 09                 	movss	%xmm1, (%rcx)
140839527: 44 88 41 04                 	movb	%r8b, 0x4(%rcx)
14083952b: c6 41 11 04                 	movb	$0x4, 0x11(%rcx)
14083952f: c3                          	retq
140839530: 48 89 c8                    	movq	%rcx, %rax
140839533: 88 11                       	movb	%dl, (%rcx)
140839535: c6 41 11 05                 	movb	$0x5, 0x11(%rcx)
140839539: c3                          	retq
14083953a: cc                          	int3
14083953b: cc                          	int3
14083953c: cc                          	int3
14083953d: cc                          	int3
14083953e: cc                          	int3
14083953f: cc                          	int3
140839540: 48 89 c8                    	movq	%rcx, %rax
140839543: 89 11                       	movl	%edx, (%rcx)
140839545: c6 41 11 06                 	movb	$0x6, 0x11(%rcx)
140839549: c3                          	retq
14083954a: cc                          	int3
14083954b: cc                          	int3
14083954c: cc                          	int3
14083954d: cc                          	int3
14083954e: cc                          	int3
14083954f: cc                          	int3
140839550: 48 89 c8                    	movq	%rcx, %rax
140839553: f3 0f 11 09                 	movss	%xmm1, (%rcx)
140839557: c6 41 11 07                 	movb	$0x7, 0x11(%rcx)
14083955b: c3                          	retq
14083955c: cc                          	int3
14083955d: cc                          	int3
14083955e: cc                          	int3
14083955f: cc                          	int3
140839560: 48 89 c8                    	movq	%rcx, %rax
140839563: 8b 4a 10                    	movl	0x10(%rdx), %ecx
140839566: 89 48 10                    	movl	%ecx, 0x10(%rax)
140839569: 0f 10 02                    	movups	(%rdx), %xmm0
14083956c: 0f 11 00                    	movups	%xmm0, (%rax)
14083956f: c3                          	retq

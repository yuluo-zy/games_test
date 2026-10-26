
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14165b330: 56                          	pushq	%rsi
14165b331: 48 83 ec 60                 	subq	$0x60, %rsp
14165b335: 48 89 ce                    	movq	%rcx, %rsi
14165b338: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
14165b33d: f3 0f 10 15 23 1d 37 01     	movss	0x1371d23(%rip), %xmm2  # 0x1429cd068
14165b345: 0f 57 db                    	xorps	%xmm3, %xmm3
14165b348: e8 c3 ca 63 ff              	callq	0x140c97e10 <.text+0xc96e10>
14165b34d: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
14165b352: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14165b356: 0f 10 44 24 28              	movups	0x28(%rsp), %xmm0
14165b35b: 0f 11 06                    	movups	%xmm0, (%rsi)
14165b35e: 48 8b 54 24 40              	movq	0x40(%rsp), %rdx
14165b363: 48 85 d2                    	testq	%rdx, %rdx
14165b366: 74 14                       	je	0x14165b37c <.text+0x165a37c>
14165b368: 48 8b 4c 24 48              	movq	0x48(%rsp), %rcx
14165b36d: 48 c1 e2 02                 	shlq	$0x2, %rdx
14165b371: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14165b377: e8 a4 88 fb fe              	callq	0x140613c20 <.text+0x612c20>
14165b37c: 48 89 f0                    	movq	%rsi, %rax
14165b37f: 48 83 c4 60                 	addq	$0x60, %rsp
14165b383: 5e                          	popq	%rsi
14165b384: c3                          	retq

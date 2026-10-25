
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409438a0: 55                          	pushq	%rbp
1409438a1: 41 56                       	pushq	%r14
1409438a3: 56                          	pushq	%rsi
1409438a4: 57                          	pushq	%rdi
1409438a5: 53                          	pushq	%rbx
1409438a6: 48 81 ec 20 04 00 00        	subq	$0x420, %rsp            # imm = 0x420
1409438ad: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409438b5: 48 c7 85 98 03 00 00 fe ff ff ff    	movq	$-0x2, 0x398(%rbp)
1409438c0: 48 89 ce                    	movq	%rcx, %rsi
1409438c3: 4c 8d 05 76 f0 16 02        	leaq	0x216f076(%rip), %r8    # 0x142ab2940
1409438ca: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1409438ce: b2 0e                       	movb	$0xe, %dl
1409438d0: e8 5b 4d df 00              	callq	0x141738630 <.text+0x1737630>
1409438d5: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409438dc: e8 cf 46 df 00              	callq	0x141737fb0 <.text+0x1736fb0>
1409438e1: c6 85 b0 02 00 00 23        	movb	$0x23, 0x2b0(%rbp)
1409438e8: 48 8b 0d 81 8a ce 02        	movq	0x2ce8a81(%rip), %rcx   # 0x14362c370
1409438ef: 4c 8d 05 6a c4 31 02        	leaq	0x231c46a(%rip), %r8    # 0x142c5fd60
1409438f6: 48 8d 95 b0 02 00 00        	leaq	0x2b0(%rbp), %rdx
1409438fd: e8 9e df 1d 00              	callq	0x140b218a0 <.text+0xb208a0>
140943902: 49 89 d0                    	movq	%rdx, %r8
140943905: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14094390c: 48 89 c2                    	movq	%rax, %rdx
14094390f: e8 ec f6 b0 00              	callq	0x141453000 <.text+0x1452000>
140943914: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
14094391b: 0f 29 85 80 03 00 00        	movaps	%xmm0, 0x380(%rbp)
140943922: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
140943929: 0f 29 85 70 03 00 00        	movaps	%xmm0, 0x370(%rbp)
140943930: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140943937: 0f 29 85 60 03 00 00        	movaps	%xmm0, 0x360(%rbp)
14094393e: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140943945: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
14094394c: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140943953: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
14094395a: 0f 29 9d 50 03 00 00        	movaps	%xmm3, 0x350(%rbp)
140943961: 0f 29 95 40 03 00 00        	movaps	%xmm2, 0x340(%rbp)
140943968: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
14094396f: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
140943976: 4c 8d 05 db ef 16 02        	leaq	0x216efdb(%rip), %r8    # 0x142ab2958
14094397d: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140943981: b2 28                       	movb	$0x28, %dl
140943983: e8 b8 fd 19 00              	callq	0x140ae3740 <.text+0xae2740>
140943988: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14094398f: e8 7c b4 04 00              	callq	0x14098ee10 <.text+0x98de10>
140943994: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
14094399b: 0f 29 85 10 03 00 00        	movaps	%xmm0, 0x310(%rbp)
1409439a2: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
1409439a9: 0f 29 85 00 03 00 00        	movaps	%xmm0, 0x300(%rbp)
1409439b0: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
1409439b7: 0f 29 85 f0 02 00 00        	movaps	%xmm0, 0x2f0(%rbp)
1409439be: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
1409439c5: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
1409439cc: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
1409439d3: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
1409439da: 0f 29 9d e0 02 00 00        	movaps	%xmm3, 0x2e0(%rbp)
1409439e1: 0f 29 95 d0 02 00 00        	movaps	%xmm2, 0x2d0(%rbp)
1409439e8: 0f 29 8d c0 02 00 00        	movaps	%xmm1, 0x2c0(%rbp)
1409439ef: 0f 29 85 b0 02 00 00        	movaps	%xmm0, 0x2b0(%rbp)
1409439f6: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1409439fd: 48 8d 95 b0 02 00 00        	leaq	0x2b0(%rbp), %rdx

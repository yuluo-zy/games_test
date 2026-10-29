
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141250720: 56                          	pushq	%rsi
141250721: 57                          	pushq	%rdi
141250722: 53                          	pushq	%rbx
141250723: 48 81 ec f0 00 00 00        	subq	$0xf0, %rsp
14125072a: 44 0f 29 84 24 e0 00 00 00  	movaps	%xmm8, 0xe0(%rsp)
141250733: 0f 29 bc 24 d0 00 00 00     	movaps	%xmm7, 0xd0(%rsp)
14125073b: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141250743: 4c 89 cf                    	movq	%r9, %rdi
141250746: 4c 89 c6                    	movq	%r8, %rsi
141250749: 48 89 d3                    	movq	%rdx, %rbx
14125074c: 48 89 ca                    	movq	%rcx, %rdx
14125074f: 48 8b 84 24 30 01 00 00     	movq	0x130(%rsp), %rax
141250757: 8b 4b 08                    	movl	0x8(%rbx), %ecx
14125075a: 89 8c 24 80 00 00 00        	movl	%ecx, 0x80(%rsp)
141250761: 48 8b 0b                    	movq	(%rbx), %rcx
141250764: 48 89 4c 24 78              	movq	%rcx, 0x78(%rsp)
141250769: f2 0f 10 05 8f d5 78 01     	movsd	0x178d58f(%rip), %xmm0  # 0x1429ddd00
141250771: f2 0f 11 84 24 84 00 00 00  	movsd	%xmm0, 0x84(%rsp)
14125077a: c7 84 24 8c 00 00 00 00 00 00 80    	movl	$0x80000000, 0x8c(%rsp) # imm = 0x80000000
141250785: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14125078a: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
141250793: c7 44 24 20 02 12 00 00     	movl	$0x1202, 0x20(%rsp)     # imm = 0x1202
14125079b: 48 8d 4c 24 48              	leaq	0x48(%rsp), %rcx
1412507a0: 4c 8d 44 24 78              	leaq	0x78(%rsp), %r8
1412507a5: f3 0f 10 1d d3 51 6d 01     	movss	0x16d51d3(%rip), %xmm3  # 0x142925980
1412507ad: e8 9e c4 67 ff              	callq	0x1408ccc50 <.text+0x8cbc50>

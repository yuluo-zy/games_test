
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408f79dd: 49 89 d0                    	movq	%rdx, %r8
1408f79e0: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1408f79e7: 48 89 c2                    	movq	%rax, %rdx
1408f79ea: e8 21 c0 2a 00              	callq	0x140ba3a10 <.text+0xba2a10>
1408f79ef: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
1408f79f6: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1408f79fd: 48 83 f9 02                 	cmpq	$0x2, %rcx
1408f7a01: 75 1d                       	jne	0x1408f7a20 <.text+0x8f6a20>
1408f7a03: 48 b9 09 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff7, %rcx # imm = 0x8000000000000009
1408f7a0d: 48 89 8d 00 0f 00 00        	movq	%rcx, 0xf00(%rbp)
1408f7a14: 48 89 85 08 0f 00 00        	movq	%rax, 0xf08(%rbp)
1408f7a1b: e9 b1 00 00 00              	jmp	0x1408f7ad1 <.text+0x8f6ad1>
1408f7a20: 4c 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %r8
1408f7a27: 49 8b 50 40                 	movq	0x40(%r8), %rdx
1408f7a2b: 4c 8d 8d d0 07 00 00        	leaq	0x7d0(%rbp), %r9
1408f7a32: 49 89 51 40                 	movq	%rdx, 0x40(%r9)
1408f7a36: 41 0f 10 00                 	movups	(%r8), %xmm0
1408f7a3a: 41 0f 10 48 10              	movups	0x10(%r8), %xmm1
1408f7a3f: 41 0f 10 50 20              	movups	0x20(%r8), %xmm2
1408f7a44: 41 0f 10 58 30              	movups	0x30(%r8), %xmm3
1408f7a49: 41 0f 11 59 30              	movups	%xmm3, 0x30(%r9)
1408f7a4e: 41 0f 11 51 20              	movups	%xmm2, 0x20(%r9)
1408f7a53: 41 0f 11 49 10              	movups	%xmm1, 0x10(%r9)
1408f7a58: 41 0f 11 01                 	movups	%xmm0, (%r9)
1408f7a5c: 48 89 8d c0 07 00 00        	movq	%rcx, 0x7c0(%rbp)
1408f7a63: 48 89 85 c8 07 00 00        	movq	%rax, 0x7c8(%rbp)
1408f7a6a: 4c 89 e9                    	movq	%r13, %rcx
1408f7a6d: 4c 89 fa                    	movq	%r15, %rdx
1408f7a70: e8 3b 41 02 00              	callq	0x14091bbb0 <.text+0x91abb0>
1408f7a75: 48 b8 0a 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff6, %rax # imm = 0x800000000000000A
1408f7a7f: 48 39 85 00 0f 00 00        	cmpq	%rax, 0xf00(%rbp)
1408f7a86: 75 49                       	jne	0x1408f7ad1 <.text+0x8f6ad1>
1408f7a88: 48 8b 8d 08 0f 00 00        	movq	0xf08(%rbp), %rcx
1408f7a8f: 48 8b 85 10 0f 00 00        	movq	0xf10(%rbp), %rax
1408f7a96: 48 8d 95 18 0f 00 00        	leaq	0xf18(%rbp), %rdx
1408f7a9d: 0f 10 02                    	movups	(%rdx), %xmm0
1408f7aa0: 0f 29 85 b0 0b 00 00        	movaps	%xmm0, 0xbb0(%rbp)
1408f7aa7: 48 8b 52 10                 	movq	0x10(%rdx), %rdx
1408f7aab: 48 89 95 c0 0b 00 00        	movq	%rdx, 0xbc0(%rbp)
1408f7ab2: 48 83 f9 0a                 	cmpq	$0xa, %rcx
1408f7ab6: 0f 85 26 05 00 00           	jne	0x1408f7fe2 <.text+0x8f6fe2>
1408f7abc: 48 85 c0                    	testq	%rax, %rax
1408f7abf: 48 8b 8d 30 10 00 00        	movq	0x1030(%rbp), %rcx
1408f7ac6: 0f 85 fb 00 00 00           	jne	0x1408f7bc7 <.text+0x8f6bc7>
1408f7acc: e9 2b fd ff ff              	jmp	0x1408f77fc <.text+0x8f67fc>


reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14146ec9c: 48 0f ba a5 10 09 00 00 28  	btq	$0x28, 0x910(%rbp)
14146eca5: 0f 83 f3 0a 00 00           	jae	0x14146f79e <.text+0x146e79e>
14146ecab: 48 8b 05 f6 d6 1b 02        	movq	0x21bd6f6(%rip), %rax   # 0x14362c3a8
14146ecb2: 48 8b 00                    	movq	(%rax), %rax
14146ecb5: 48 83 f8 03                 	cmpq	$0x3, %rax
14146ecb9: 0f 82 c4 00 00 00           	jb	0x14146ed83 <.text+0x146dd83>
14146ecbf: c6 85 73 09 00 00 01        	movb	$0x1, 0x973(%rbp)
14146ecc6: 48 8d 0d 33 5e 7f 01        	leaq	0x17f5e33(%rip), %rcx   # 0x142c64b00
14146eccd: e8 ae f0 20 ff              	callq	0x14067dd80 <.text+0x67cd80>
14146ecd2: 0f 10 00                    	movups	(%rax), %xmm0
14146ecd5: 8b 40 10                    	movl	0x10(%rax), %eax
14146ecd8: 48 c7 85 e0 05 00 00 03 00 00 00    	movq	$0x3, 0x5e0(%rbp)
14146ece3: 48 8d 0d 5e 54 7f 01        	leaq	0x17f545e(%rip), %rcx   # 0x142c64148
14146ecea: 48 89 8d e8 05 00 00        	movq	%rcx, 0x5e8(%rbp)
14146ecf1: 48 c7 85 f0 05 00 00 26 00 00 00    	movq	$0x26, 0x5f0(%rbp)
14146ecfc: 48 8d 15 ed 5d 7f 01        	leaq	0x17f5ded(%rip), %rdx   # 0x142c64af0
14146ed03: 48 89 95 00 06 00 00        	movq	%rdx, 0x600(%rbp)
14146ed0a: 48 c7 85 08 06 00 00 01 00 00 00    	movq	$0x1, 0x608(%rbp)
14146ed15: 48 c7 85 10 06 00 00 08 00 00 00    	movq	$0x8, 0x610(%rbp)
14146ed20: 48 c7 85 b0 05 00 00 00 00 00 00    	movq	$0x0, 0x5b0(%rbp)
14146ed2b: 0f 57 c9                    	xorps	%xmm1, %xmm1
14146ed2e: 0f 11 8d 18 06 00 00        	movups	%xmm1, 0x618(%rbp)
14146ed35: 48 89 8d b8 05 00 00        	movq	%rcx, 0x5b8(%rbp)
14146ed3c: 48 c7 85 c0 05 00 00 26 00 00 00    	movq	$0x26, 0x5c0(%rbp)
14146ed47: 48 c7 85 c8 05 00 00 00 00 00 00    	movq	$0x0, 0x5c8(%rbp)
14146ed52: 0f 11 85 d0 05 00 00        	movups	%xmm0, 0x5d0(%rbp)
14146ed59: c7 85 f8 05 00 00 01 00 00 00       	movl	$0x1, 0x5f8(%rbp)
14146ed63: 89 85 fc 05 00 00           	movl	%eax, 0x5fc(%rbp)
14146ed69: c6 85 73 09 00 00 01        	movb	$0x1, 0x973(%rbp)
14146ed70: 48 8d 8d 72 09 00 00        	leaq	0x972(%rbp), %rcx
14146ed77: 48 8d 95 b0 05 00 00        	leaq	0x5b0(%rbp), %rdx
14146ed7e: e8 ad b5 8d ff              	callq	0x140d4a330 <.text+0xd49330>
14146ed83: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
14146ed8a: 48 85 c0                    	testq	%rax, %rax
14146ed8d: 0f 84 45 09 00 00           	je	0x14146f6d8 <.text+0x146e6d8>
14146ed93: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
14146ed9a: 48 c1 e0 04                 	shlq	$0x4, %rax
14146ed9e: 31 d2                       	xorl	%edx, %edx
14146eda0: 4c 8d 05 01 5f 7f 01        	leaq	0x17f5f01(%rip), %r8    # 0x142c64ca8
14146eda7: eb 15                       	jmp	0x14146edbe <.text+0x146ddbe>
14146eda9: 45 89 d1                    	movl	%r10d, %r9d
14146edac: 44 89 4c 11 08              	movl	%r9d, 0x8(%rcx,%rdx)
14146edb1: 48 83 c2 10                 	addq	$0x10, %rdx
14146edb5: 48 39 d0                    	cmpq	%rdx, %rax
14146edb8: 0f 84 1a 09 00 00           	je	0x14146f6d8 <.text+0x146e6d8>
14146edbe: 44 8b 54 11 08              	movl	0x8(%rcx,%rdx), %r10d
14146edc3: 49 83 fa 08                 	cmpq	$0x8, %r10
14146edc7: 77 12                       	ja	0x14146eddb <.text+0x146dddb>
14146edc9: 4f 63 0c 90                 	movslq	(%r8,%r10,4), %r9
14146edcd: 4d 01 c1                    	addq	%r8, %r9
14146edd0: 41 ff e1                    	jmpq	*%r9
14146edd3: 41 b9 02 00 00 00           	movl	$0x2, %r9d
14146edd9: eb d1                       	jmp	0x14146edac <.text+0x146ddac>
14146eddb: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14146ede1: 41 81 fa e7 03 00 00        	cmpl	$0x3e7, %r10d           # imm = 0x3E7
14146ede8: 74 bf                       	je	0x14146eda9 <.text+0x146dda9>
14146edea: eb c0                       	jmp	0x14146edac <.text+0x146ddac>
14146edec: 41 b9 03 00 00 00           	movl	$0x3, %r9d
14146edf2: eb b8                       	jmp	0x14146edac <.text+0x146ddac>
14146edf4: 41 b9 05 00 00 00           	movl	$0x5, %r9d
14146edfa: eb b0                       	jmp	0x14146edac <.text+0x146ddac>
14146edfc: 41 b9 08 00 00 00           	movl	$0x8, %r9d
14146ee02: eb a8                       	jmp	0x14146edac <.text+0x146ddac>
14146ee04: 41 b9 04 00 00 00           	movl	$0x4, %r9d
14146ee0a: eb a0                       	jmp	0x14146edac <.text+0x146ddac>
14146ee0c: 48 c7 85 e8 08 00 00 00 00 00 00    	movq	$0x0, 0x8e8(%rbp)
14146ee17: 4c 8d a5 b0 01 00 00        	leaq	0x1b0(%rbp), %r12
14146ee1e: 48 8b b5 68 05 00 00        	movq	0x568(%rbp), %rsi
14146ee25: 0f b6 4e 42                 	movzbl	0x42(%rsi), %ecx
14146ee29: 0f b7 56 40                 	movzwl	0x40(%rsi), %edx
14146ee2d: 44 8b 46 38                 	movl	0x38(%rsi), %r8d
14146ee31: f3 0f 10 46 3c              	movss	0x3c(%rsi), %xmm0
14146ee36: 44 0f b6 4e 14              	movzbl	0x14(%rsi), %r9d
14146ee3b: 44 8b 56 10                 	movl	0x10(%rsi), %r10d
14146ee3f: 48 89 bd f8 05 00 00        	movq	%rdi, 0x5f8(%rbp)
14146ee46: 89 9d 70 06 00 00           	movl	%ebx, 0x670(%rbp)
14146ee4c: 0f 10 8d 60 03 00 00        	movups	0x360(%rbp), %xmm1
14146ee53: 0f 10 95 70 03 00 00        	movups	0x370(%rbp), %xmm2
14146ee5a: 0f 11 8d 74 06 00 00        	movups	%xmm1, 0x674(%rbp)
14146ee61: 0f 11 95 84 06 00 00        	movups	%xmm2, 0x684(%rbp)
14146ee68: 44 8b 9d 80 03 00 00        	movl	0x380(%rbp), %r11d
14146ee6f: 44 89 9d 94 06 00 00        	movl	%r11d, 0x694(%rbp)
14146ee76: 44 89 bd 44 06 00 00        	movl	%r15d, 0x644(%rbp)
14146ee7d: 44 0f 11 85 48 06 00 00     	movups	%xmm8, 0x648(%rbp)
14146ee85: 44 0f 11 8d 58 06 00 00     	movups	%xmm9, 0x658(%rbp)
14146ee8d: f3 44 0f 11 95 68 06 00 00  	movss	%xmm10, 0x668(%rbp)
14146ee96: 66 44 89 b5 6c 06 00 00     	movw	%r14w, 0x66c(%rbp)
14146ee9e: 0f 13 bd 98 06 00 00        	movlps	%xmm7, 0x698(%rbp)
14146eea5: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
14146eea9: 0f 29 8d b0 05 00 00        	movaps	%xmm1, 0x5b0(%rbp)
14146eeb0: 4c 8b 5d c0                 	movq	-0x40(%rbp), %r11
14146eeb4: 4c 89 9d c0 05 00 00        	movq	%r11, 0x5c0(%rbp)
14146eebb: 4c 8b 9d c0 01 00 00        	movq	0x1c0(%rbp), %r11
14146eec2: 4c 89 9d d8 05 00 00        	movq	%r11, 0x5d8(%rbp)
14146eec9: 0f 10 8d b0 01 00 00        	movups	0x1b0(%rbp), %xmm1
14146eed0: 0f 11 8d c8 05 00 00        	movups	%xmm1, 0x5c8(%rbp)
14146eed7: 4c 8b 9d e8 08 00 00        	movq	0x8e8(%rbp), %r11
14146eede: 4c 89 9d e0 05 00 00        	movq	%r11, 0x5e0(%rbp)
14146eee5: 48 89 85 e8 05 00 00        	movq	%rax, 0x5e8(%rbp)
14146eeec: 48 8b 85 d8 08 00 00        	movq	0x8d8(%rbp), %rax
14146eef3: 48 89 85 f0 05 00 00        	movq	%rax, 0x5f0(%rbp)
14146eefa: 0f 10 0e                    	movups	(%rsi), %xmm1
14146eefd: 0f 29 8d 00 06 00 00        	movaps	%xmm1, 0x600(%rbp)
14146ef04: 44 89 95 10 06 00 00        	movl	%r10d, 0x610(%rbp)
14146ef0b: 44 88 8d 14 06 00 00        	movb	%r9b, 0x614(%rbp)
14146ef12: 0f 10 4e 18                 	movups	0x18(%rsi), %xmm1
14146ef16: 0f 11 8d 18 06 00 00        	movups	%xmm1, 0x618(%rbp)
14146ef1d: 0f 10 4e 28                 	movups	0x28(%rsi), %xmm1
14146ef21: 0f 11 8d 28 06 00 00        	movups	%xmm1, 0x628(%rbp)
14146ef28: 44 89 85 38 06 00 00        	movl	%r8d, 0x638(%rbp)
14146ef2f: f3 0f 11 85 3c 06 00 00     	movss	%xmm0, 0x63c(%rbp)
14146ef37: 66 89 95 40 06 00 00        	movw	%dx, 0x640(%rbp)
14146ef3e: 88 8d 42 06 00 00           	movb	%cl, 0x642(%rbp)
14146ef44: 0f b6 05 be 5c 1c 02        	movzbl	0x21c5cbe(%rip), %eax   # 0x143634c09
14146ef4b: b9 f0 00 00 00              	movl	$0xf0, %ecx
14146ef50: ba 08 00 00 00              	movl	$0x8, %edx
14146ef55: e8 b6 4c 1a ff              	callq	0x140613c10 <.text+0x612c10>
14146ef5a: 48 85 c0                    	testq	%rax, %rax
14146ef5d: 48 8b 9d 48 09 00 00        	movq	0x948(%rbp), %rbx
14146ef64: 4c 8b b5 00 09 00 00        	movq	0x900(%rbp), %r14
14146ef6b: 0f 84 dd 09 00 00           	je	0x14146f94e <.text+0x146e94e>
14146ef71: 48 8d 95 b0 05 00 00        	leaq	0x5b0(%rbp), %rdx
14146ef78: 41 b8 f0 00 00 00           	movl	$0xf0, %r8d
14146ef7e: 48 89 85 58 09 00 00        	movq	%rax, 0x958(%rbp)
14146ef85: 48 89 c1                    	movq	%rax, %rcx
14146ef88: e8 73 4c 4b 01              	callq	0x142923c00 <.text+0x2922c00>
14146ef8d: 48 8b 05 14 d4 1b 02        	movq	0x21bd414(%rip), %rax   # 0x14362c3a8
14146ef94: 48 8b 00                    	movq	(%rax), %rax
14146ef97: 48 83 f8 04                 	cmpq	$0x4, %rax
14146ef9b: 48 8b bd 18 09 00 00        	movq	0x918(%rbp), %rdi
14146efa2: 0f 82 f3 00 00 00           	jb	0x14146f09b <.text+0x146e09b>
14146efa8: 48 8d 8d 90 05 00 00        	leaq	0x590(%rbp), %rcx
14146efaf: e8 5c 49 8f ff              	callq	0x140d63910 <.text+0xd62910>
14146efb4: 48 89 85 60 03 00 00        	movq	%rax, 0x360(%rbp)
14146efbb: 89 95 68 03 00 00           	movl	%edx, 0x368(%rbp)
14146efc1: 4c 89 ad b0 01 00 00        	movq	%r13, 0x1b0(%rbp)
14146efc8: 48 8d 05 81 b5 91 ff        	leaq	-0x6e4a7f(%rip), %rax   # 0x140d8a550 <.text+0xd89550>
14146efcf: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
14146efd6: 48 8d 0d 7b 5b 7f 01        	leaq	0x17f5b7b(%rip), %rcx   # 0x142c64b58
14146efdd: e8 9e ed 20 ff              	callq	0x14067dd80 <.text+0x67cd80>
14146efe2: 0f 10 00                    	movups	(%rax), %xmm0
14146efe5: 8b 40 10                    	movl	0x10(%rax), %eax
14146efe8: 48 c7 85 e0 05 00 00 04 00 00 00    	movq	$0x4, 0x5e0(%rbp)
14146eff3: 48 8d 0d 4e 51 7f 01        	leaq	0x17f514e(%rip), %rcx   # 0x142c64148
14146effa: 48 89 8d e8 05 00 00        	movq	%rcx, 0x5e8(%rbp)


D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a40f90: 85 35 02 00 00 8b           	testl	%esi, -0x74fffffe(%rip) # 0xcba40f98
140a40f96: 97                          	xchgl	%edi, %eax
140a40f97: 90                          	nop
140a40f98: 00 00                       	addb	%al, (%rax)
140a40f9a: 00 41 3b                    	addb	%al, 0x3b(%rcx)
140a40f9d: 55                          	pushq	%rbp
140a40f9e: 30 0f                       	xorb	%cl, (%rdi)
140a40fa0: 85 25 02 00 00 8b           	testl	%esp, -0x74fffffe(%rip) # 0xcba40fa8
140a40fa6: 97                          	xchgl	%edi, %eax
140a40fa7: 94                          	xchgl	%esp, %eax
140a40fa8: 00 00                       	addb	%al, (%rax)
140a40faa: 00 41 3b                    	addb	%al, 0x3b(%rcx)
140a40fad: 55                          	pushq	%rbp
140a40fae: 34 0f                       	xorb	$0xf, %al
140a40fb0: 85 15 02 00 00 41           	testl	%edx, 0x41000002(%rip)  # 0x181a40fb8
140a40fb6: 0f b6 55 00                 	movzbl	(%rbp), %edx
140a40fba: f6 47 60 01                 	testb	$0x1, 0x60(%rdi)
140a40fbe: 0f 84 5a 01 00 00           	je	0x140a4111e <.text+0xa4011e>
140a40fc4: 45 8b 45 04                 	movl	0x4(%r13), %r8d
140a40fc8: 44 39 47 64                 	cmpl	%r8d, 0x64(%rdi)
140a40fcc: 41 0f 94 c0                 	sete	%r8b
140a40fd0: 44 20 c2                    	andb	%r8b, %dl
140a40fd3: 80 fa 01                    	cmpb	$0x1, %dl
140a40fd6: 0f 84 4b 01 00 00           	je	0x140a41127 <.text+0xa40127>
140a40fdc: e9 e9 01 00 00              	jmp	0x140a411ca <.text+0xa401ca>
140a40fe1: 41 8b 55 1c                 	movl	0x1c(%r13), %edx
140a40fe5: e9 e2 01 00 00              	jmp	0x140a411cc <.text+0xa401cc>
140a40fea: 48 85 c0                    	testq	%rax, %rax
140a40fed: 0f 85 0c 0e 00 00           	jne	0x140a41dff <.text+0xa40dff>
140a40ff3: 31 d2                       	xorl	%edx, %edx
140a40ff5: e8 d6 77 eb 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140a40ffa: 48 89 c1                    	movq	%rax, %rcx
140a40ffd: 48 85 c0                    	testq	%rax, %rax
140a41000: 0f 85 04 e8 ff ff           	jne	0x140a3f80a <.text+0xa3e80a>
140a41006: e9 f4 0d 00 00              	jmp	0x140a41dff <.text+0xa40dff>
140a4100b: 41 b9 20 1c 00 00           	movl	$0x1c20, %r9d           # imm = 0x1C20
140a41011: 48 8b bd 48 03 00 00        	movq	0x348(%rbp), %rdi
140a41018: eb 1f                       	jmp	0x140a41039 <.text+0xa40039>
140a4101a: 41 b9 10 0e 00 00           	movl	$0xe10, %r9d            # imm = 0xE10
140a41020: 4d 89 d0                    	movq	%r10, %r8
140a41023: eb 14                       	jmp	0x140a41039 <.text+0xa40039>
140a41025: 41 b9 58 02 00 00           	movl	$0x258, %r9d            # imm = 0x258
140a4102b: 4d 89 d8                    	movq	%r11, %r8
140a4102e: eb 09                       	jmp	0x140a41039 <.text+0xa40039>
140a41030: 41 b9 2c 01 00 00           	movl	$0x12c, %r9d            # imm = 0x12C
140a41036: 49 89 c0                    	movq	%rax, %r8
140a41039: f3 0f 11 87 44 01 00 00     	movss	%xmm0, 0x144(%rdi)
140a41041: 41 8d 54 24 01              	leal	0x1(%r12), %edx
140a41046: 89 97 40 01 00 00           	movl	%edx, 0x140(%rdi)
140a4104c: 44 89 e2                    	movl	%r12d, %edx
140a4104f: 44 29 ca                    	subl	%r9d, %edx
140a41052: 73 19                       	jae	0x140a4106d <.text+0xa4006d>
140a41054: c7 85 b0 02 00 00 00 00 00 00       	movl	$0x0, 0x2b0(%rbp)
140a4105e: c7 85 b8 02 00 00 00 00 00 00       	movl	$0x0, 0x2b8(%rbp)
140a41068: e9 d5 ec ff ff              	jmp	0x140a3fd42 <.text+0xa3ed42>
140a4106d: 49 29 c0                    	subq	%rax, %r8
140a41070: 49 83 f8 0f                 	cmpq	$0xf, %r8
140a41074: 77 42                       	ja	0x140a410b8 <.text+0xa400b8>
140a41076: 4c 8d 15 cb e2 05 02        	leaq	0x205e2cb(%rip), %r10   # 0x142a9f348
140a4107d: 44 89 e7                    	movl	%r12d, %edi
140a41080: 44 29 cf                    	subl	%r9d, %edi
140a41083: 4d 89 c3                    	movq	%r8, %r11
140a41086: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140a41090: 49 83 fb 0c                 	cmpq	$0xc, %r11
140a41094: 74 2a                       	je	0x140a410c0 <.text+0xa400c0>
140a41096: 43 8b 74 13 04              	movl	0x4(%r11,%r10), %esi
140a4109b: 85 f6                       	testl	%esi, %esi
140a4109d: 0f 84 9c 0d 00 00           	je	0x140a41e3f <.text+0xa40e3f>
140a410a3: 89 f8                       	movl	%edi, %eax
140a410a5: 31 d2                       	xorl	%edx, %edx
140a410a7: f7 f6                       	divl	%esi
140a410a9: 49 83 c3 04                 	addq	$0x4, %r11
140a410ad: 85 d2                       	testl	%edx, %edx
140a410af: 75 df                       	jne	0x140a41090 <.text+0xa40090>
140a410b1: 44 39 cf                    	cmpl	%r9d, %edi
140a410b4: 73 ca                       	jae	0x140a41080 <.text+0xa40080>
140a410b6: eb 9c                       	jmp	0x140a41054 <.text+0xa40054>
140a410b8: 89 95 34 01 00 00           	movl	%edx, 0x134(%rbp)
140a410be: eb 06                       	jmp	0x140a410c6 <.text+0xa400c6>
140a410c0: 89 bd 34 01 00 00           	movl	%edi, 0x134(%rbp)
140a410c6: c7 85 b8 02 00 00 01 00 00 00       	movl	$0x1, 0x2b8(%rbp)
140a410d0: c7 85 b0 02 00 00 00 00 00 00       	movl	$0x0, 0x2b0(%rbp)
140a410da: e9 63 ec ff ff              	jmp	0x140a3fd42 <.text+0xa3ed42>
140a410df: 48 89 f1                    	movq	%rsi, %rcx
140a410e2: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140a410e8: e8 23 c1 e8 01              	callq	0x1428cd210 <.text+0x28cc210>
140a410ed: e9 bd e7 ff ff              	jmp	0x140a3f8af <.text+0xa3e8af>
140a410f2: 48 89 f1                    	movq	%rsi, %rcx
140a410f5: 31 d2                       	xorl	%edx, %edx
140a410f7: e8 94 ca e8 01              	callq	0x1428cdb90 <.text+0x28ccb90>
140a410fc: e9 fa e8 ff ff              	jmp	0x140a3f9fb <.text+0xa3e9fb>

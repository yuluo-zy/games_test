
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407febb0: 41 57                       	pushq	%r15
1407febb2: 41 56                       	pushq	%r14
1407febb4: 41 55                       	pushq	%r13
1407febb6: 41 54                       	pushq	%r12
1407febb8: 56                          	pushq	%rsi
1407febb9: 57                          	pushq	%rdi
1407febba: 55                          	pushq	%rbp
1407febbb: 53                          	pushq	%rbx
1407febbc: 48 81 ec c8 01 00 00        	subq	$0x1c8, %rsp            # imm = 0x1C8
1407febc3: 44 0f 29 ac 24 b0 01 00 00  	movaps	%xmm13, 0x1b0(%rsp)
1407febcc: 44 0f 29 a4 24 a0 01 00 00  	movaps	%xmm12, 0x1a0(%rsp)
1407febd5: 44 0f 29 9c 24 90 01 00 00  	movaps	%xmm11, 0x190(%rsp)
1407febde: 44 0f 29 94 24 80 01 00 00  	movaps	%xmm10, 0x180(%rsp)
1407febe7: 44 0f 29 8c 24 70 01 00 00  	movaps	%xmm9, 0x170(%rsp)
1407febf0: 44 0f 29 84 24 60 01 00 00  	movaps	%xmm8, 0x160(%rsp)
1407febf9: 0f 29 bc 24 50 01 00 00     	movaps	%xmm7, 0x150(%rsp)
1407fec01: 0f 29 b4 24 40 01 00 00     	movaps	%xmm6, 0x140(%rsp)
1407fec09: 89 d3                       	movl	%edx, %ebx
1407fec0b: 48 8b 94 24 48 02 00 00     	movq	0x248(%rsp), %rdx
1407fec13: 48 85 d2                    	testq	%rdx, %rdx
1407fec16: 0f 84 c3 04 00 00           	je	0x1407ff0df <.text+0x7fe0df>
1407fec1c: 44 89 c5                    	movl	%r8d, %ebp
1407fec1f: f3 0f 10 84 24 78 02 00 00  	movss	0x278(%rsp), %xmm0
1407fec28: f3 0f 10 8c 24 70 02 00 00  	movss	0x270(%rsp), %xmm1
1407fec31: 0f 14 c8                    	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
1407fec34: 48 8b 8c 24 40 02 00 00     	movq	0x240(%rsp), %rcx
1407fec3c: 48 8b 84 24 38 02 00 00     	movq	0x238(%rsp), %rax
1407fec44: f2 0f 10 01                 	movsd	(%rcx), %xmm0
1407fec48: 0f 59 c1                    	mulps	%xmm1, %xmm0
1407fec4b: f3 0f 16 f8                 	movshdup	%xmm0, %xmm7            # xmm7 = xmm0[1,1,3,3]
1407fec4f: f3 0f 58 f8                 	addss	%xmm0, %xmm7
1407fec53: 4c 8d 40 ff                 	leaq	-0x1(%rax), %r8
1407fec57: 31 ff                       	xorl	%edi, %edi
1407fec59: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407fec5c: 0f 2e f8                    	ucomiss	%xmm0, %xmm7
1407fec5f: 49 0f 47 f8                 	cmovaq	%r8, %rdi
1407fec63: 48 39 c7                    	cmpq	%rax, %rdi
1407fec66: 0f 83 83 04 00 00           	jae	0x1407ff0ef <.text+0x7fe0ef>
1407fec6c: 48 39 d7                    	cmpq	%rdx, %rdi
1407fec6f: 0f 83 8c 04 00 00           	jae	0x1407ff101 <.text+0x7fe101>
1407fec75: 4c 89 ce                    	movq	%r9, %rsi
1407fec78: 4c 8b b4 24 58 02 00 00     	movq	0x258(%rsp), %r14
1407fec80: 48 8b 84 24 30 02 00 00     	movq	0x230(%rsp), %rax
1407fec88: f3 0f 10 34 b8              	movss	(%rax,%rdi,4), %xmm6
1407fec8d: f3 44 0f 10 04 f9           	movss	(%rcx,%rdi,8), %xmm8
1407fec93: f3 44 0f 10 4c f9 04        	movss	0x4(%rcx,%rdi,8), %xmm9
1407fec9a: 0f 28 05 4f 6c 12 02        	movaps	0x2126c4f(%rip), %xmm0  # 0x1429258f0
1407feca1: 41 0f 57 c1                 	xorps	%xmm9, %xmm0
1407feca5: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1407feca9: f3 41 0f 59 c9              	mulss	%xmm9, %xmm1
1407fecae: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1407fecb2: f3 41 0f 59 d0              	mulss	%xmm8, %xmm2
1407fecb7: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407fecbb: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407fecbe: f3 0f 51 ca                 	sqrtss	%xmm2, %xmm1
1407fecc2: f3 44 0f 10 15 39 6c 12 02  	movss	0x2126c39(%rip), %xmm10 # 0x142925904
1407feccb: f3 44 0f 5e d1              	divss	%xmm1, %xmm10
1407fecd0: f3 41 0f 59 c2              	mulss	%xmm10, %xmm0
1407fecd5: f3 45 0f 59 c2              	mulss	%xmm10, %xmm8
1407fecda: f3 0f 11 84 24 80 00 00 00  	movss	%xmm0, 0x80(%rsp)
1407fece3: f3 44 0f 11 84 24 84 00 00 00       	movss	%xmm8, 0x84(%rsp)
1407feced: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
1407fecf2: 48 8d 94 24 80 00 00 00     	leaq	0x80(%rsp), %rdx
1407fecfa: e8 c1 e0 49 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407fecff: 4c 39 f7                    	cmpq	%r14, %rdi
1407fed02: 0f 83 08 04 00 00           	jae	0x1407ff110 <.text+0x7fe110>
1407fed08: 44 0f b6 bc 24 68 02 00 00  	movzbl	0x268(%rsp), %r15d
1407fed11: 44 0f b6 b4 24 60 02 00 00  	movzbl	0x260(%rsp), %r14d
1407fed1a: f3 0f 58 35 86 d8 28 02     	addss	0x228d886(%rip), %xmm6  # 0x142a8c5a8
1407fed22: f3 0f 5f 35 5a 6c 12 02     	maxss	0x2126c5a(%rip), %xmm6  # 0x142925984
1407fed2a: 45 0f 57 db                 	xorps	%xmm11, %xmm11
1407fed2e: 31 c0                       	xorl	%eax, %eax
1407fed30: 41 0f 2e fb                 	ucomiss	%xmm11, %xmm7
1407fed34: 0f 97 c0                    	seta	%al
1407fed37: 4c 8b a4 24 50 02 00 00     	movq	0x250(%rsp), %r12
1407fed3f: 4c 8d 2c 7f                 	leaq	(%rdi,%rdi,2), %r13
1407fed43: f3 47 0f 10 64 ac 08        	movss	0x8(%r12,%r13,4), %xmm12
1407fed4a: 48 8d 0d 9f ef 1d 02        	leaq	0x21def9f(%rip), %rcx   # 0x1429ddcf0
1407fed51: f3 0f 10 3c 81              	movss	(%rcx,%rax,4), %xmm7
1407fed56: f3 45 0f 59 ca              	mulss	%xmm10, %xmm9
1407fed5b: f3 44 0f 11 84 24 e0 00 00 00       	movss	%xmm8, 0xe0(%rsp)
1407fed65: f3 44 0f 11 8c 24 e4 00 00 00       	movss	%xmm9, 0xe4(%rsp)
1407fed6f: 48 8d 8c 24 80 00 00 00     	leaq	0x80(%rsp), %rcx
1407fed77: 48 8d 94 24 e0 00 00 00     	leaq	0xe0(%rsp), %rdx
1407fed7f: e8 3c e0 49 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407fed84: f3 0f 12 c7                 	movsldup	%xmm7, %xmm0            # xmm0 = xmm7[0,0,2,2]
1407fed88: f3 0f 59 bc 24 88 00 00 00  	mulss	0x88(%rsp), %xmm7
1407fed91: f3 0f 59 3d 9b d8 28 02     	mulss	0x228d89b(%rip), %xmm7  # 0x142a8c634
1407fed99: f3 0f 59 3d cf 6a 12 02     	mulss	0x2126acf(%rip), %xmm7  # 0x142925870
1407feda1: f3 41 0f 58 fc              	addss	%xmm12, %xmm7
1407feda6: 40 0f b6 c5                 	movzbl	%bpl, %eax
1407fedaa: 48 8d 0d 7f de 28 02        	leaq	0x228de7f(%rip), %rcx   # 0x142a8cc30
1407fedb1: f2 43 0f 10 0c ac           	movsd	(%r12,%r13,4), %xmm1
1407fedb7: f2 44 0f 10 94 24 80 00 00 00       	movsd	0x80(%rsp), %xmm10
1407fedc1: 44 0f 59 d0                 	mulps	%xmm0, %xmm10
1407fedc5: 44 0f 59 15 53 d9 28 02     	mulps	0x228d953(%rip), %xmm10 # 0x142a8c720
1407fedcd: 44 0f 59 15 0b 6b 12 02     	mulps	0x2126b0b(%rip), %xmm10 # 0x1429258e0
1407fedd5: 44 0f 58 d1                 	addps	%xmm1, %xmm10
1407fedd9: f3 44 0f 10 0c 81           	movss	(%rcx,%rax,4), %xmm9
1407feddf: f2 44 0f 10 64 24 50        	movsd	0x50(%rsp), %xmm12
1407fede6: f2 0f 10 44 24 54           	movsd	0x54(%rsp), %xmm0
1407fedec: f3 44 0f 10 6c 24 58        	movss	0x58(%rsp), %xmm13
1407fedf3: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
1407fedf7: 44 0f 59 c0                 	mulps	%xmm0, %xmm8
1407fedfb: f3 45 0f 59 dc              	mulss	%xmm12, %xmm11
1407fee00: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
1407fee04: 41 0f 14 c3                 	unpcklps	%xmm11, %xmm0           # xmm0 = xmm0[0],xmm11[0],xmm0[1],xmm11[1]
1407fee08: 45 0f 28 dc                 	movaps	%xmm12, %xmm11
1407fee0c: f3 45 0f 5c d8              	subss	%xmm8, %xmm11
1407fee11: 44 0f 5c c0                 	subps	%xmm0, %xmm8
1407fee15: 45 84 f6                    	testb	%r14b, %r14b
1407fee18: 0f 85 31 01 00 00           	jne	0x1407fef4f <.text+0x7fdf4f>
1407fee1e: 0f 28 c6                    	movaps	%xmm6, %xmm0
1407fee21: f3 41 0f 59 c5              	mulss	%xmm13, %xmm0
1407fee26: f3 0f 59 05 ba b2 15 02     	mulss	0x215b2ba(%rip), %xmm0  # 0x14295a0e8
1407fee2e: f3 0f 58 c7                 	addss	%xmm7, %xmm0
1407fee32: f3 0f 12 ce                 	movsldup	%xmm6, %xmm1            # xmm1 = xmm6[0,0,2,2]
1407fee36: 41 0f 59 cc                 	mulps	%xmm12, %xmm1
1407fee3a: 0f 59 0d af 83 21 02        	mulps	0x22183af(%rip), %xmm1  # 0x142a171f0
1407fee41: 41 0f 58 ca                 	addps	%xmm10, %xmm1
1407fee45: 0f 13 4c 24 60              	movlps	%xmm1, 0x60(%rsp)
1407fee4a: f3 0f 11 44 24 68           	movss	%xmm0, 0x68(%rsp)
1407fee50: f3 0f 10 05 dc d7 28 02     	movss	0x228d7dc(%rip), %xmm0  # 0x142a8c634
1407fee58: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
1407fee5c: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1407fee60: 0f 28 15 b9 d8 28 02        	movaps	0x228d8b9(%rip), %xmm2  # 0x142a8c720
1407fee67: 41 0f 28 dc                 	movaps	%xmm12, %xmm3
1407fee6b: 0f 59 da                    	mulps	%xmm2, %xmm3
1407fee6e: 0f 13 5c 24 40              	movlps	%xmm3, 0x40(%rsp)
1407fee73: f3 0f 11 4c 24 48           	movss	%xmm1, 0x48(%rsp)
1407fee79: c7 44 24 34 00 00 00 00     	movl	$0x0, 0x34(%rsp)
1407fee81: f3 44 0f 11 4c 24 38        	movss	%xmm9, 0x38(%rsp)
1407fee88: c7 44 24 3c 00 00 00 00     	movl	$0x0, 0x3c(%rsp)
1407fee90: f3 41 0f 59 c3              	mulss	%xmm11, %xmm0
1407fee95: 41 0f 59 d0                 	mulps	%xmm8, %xmm2
1407fee99: 0f 13 94 24 80 00 00 00     	movlps	%xmm2, 0x80(%rsp)
1407feea1: f3 0f 11 84 24 88 00 00 00  	movss	%xmm0, 0x88(%rsp)
1407feeaa: 48 8d 44 24 60              	leaq	0x60(%rsp), %rax
1407feeaf: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407feeb4: 4c 8d b4 24 e0 00 00 00     	leaq	0xe0(%rsp), %r14
1407feebc: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
1407feec1: 4c 8d 44 24 34              	leaq	0x34(%rsp), %r8
1407feec6: 4c 8d 8c 24 80 00 00 00     	leaq	0x80(%rsp), %r9
1407feece: 4c 89 f1                    	movq	%r14, %rcx
1407feed1: e8 2a e6 09 00              	callq	0x14089d500 <.text+0x89c500>
1407feed6: 89 d9                       	movl	%ebx, %ecx
1407feed8: e8 73 98 05 00              	callq	0x140858750 <.text+0x857750>
1407feedd: 89 c5                       	movl	%eax, %ebp
1407feedf: 48 8d 8c 24 80 00 00 00     	leaq	0x80(%rsp), %rcx
1407feee7: e8 d4 98 05 00              	callq	0x1408587c0 <.text+0x8577c0>
1407feeec: 0f 10 84 24 b0 00 00 00     	movups	0xb0(%rsp), %xmm0
1407feef4: 0f 11 84 24 10 01 00 00     	movups	%xmm0, 0x110(%rsp)
1407feefc: 48 8b 84 24 c0 00 00 00     	movq	0xc0(%rsp), %rax
1407fef04: 48 89 84 24 20 01 00 00     	movq	%rax, 0x120(%rsp)
1407fef0c: 8b 84 24 c8 00 00 00        	movl	0xc8(%rsp), %eax
1407fef13: 89 84 24 28 01 00 00        	movl	%eax, 0x128(%rsp)
1407fef1a: 89 bc 24 2c 01 00 00        	movl	%edi, 0x12c(%rsp)
1407fef21: 89 ac 24 30 01 00 00        	movl	%ebp, 0x130(%rsp)
1407fef28: f2 0f 10 05 80 56 13 02     	movsd	0x2135680(%rip), %xmm0  # 0x1429345b0
1407fef30: f2 0f 11 84 24 34 01 00 00  	movsd	%xmm0, 0x134(%rsp)
1407fef39: c7 84 24 3c 01 00 00 40 00 00 00    	movl	$0x40, 0x13c(%rsp)
1407fef44: 48 89 f1                    	movq	%rsi, %rcx
1407fef47: 4c 89 f2                    	movq	%r14, %rdx
1407fef4a: e8 81 de a8 00              	callq	0x14128cdd0 <.text+0x128bdd0>
1407fef4f: 45 84 ff                    	testb	%r15b, %r15b
1407fef52: 0f 85 2d 01 00 00           	jne	0x1407ff085 <.text+0x7fe085>
1407fef58: f3 0f 12 c6                 	movsldup	%xmm6, %xmm0            # xmm0 = xmm6[0,0,2,2]
1407fef5c: f3 41 0f 59 f5              	mulss	%xmm13, %xmm6
1407fef61: f3 0f 59 35 07 69 12 02     	mulss	0x2126907(%rip), %xmm6  # 0x142925870
1407fef69: 41 0f 59 c4                 	mulps	%xmm12, %xmm0
1407fef6d: f3 0f 58 fe                 	addss	%xmm6, %xmm7
1407fef71: 0f 59 05 68 69 12 02        	mulps	0x2126968(%rip), %xmm0  # 0x1429258e0
1407fef78: 44 0f 58 d0                 	addps	%xmm0, %xmm10
1407fef7c: 44 0f 13 54 24 70           	movlps	%xmm10, 0x70(%rsp)
1407fef82: f3 0f 11 7c 24 78           	movss	%xmm7, 0x78(%rsp)
1407fef88: f3 0f 10 05 a4 d6 28 02     	movss	0x228d6a4(%rip), %xmm0  # 0x142a8c634
1407fef90: f3 44 0f 59 e8              	mulss	%xmm0, %xmm13
1407fef95: 0f 28 0d 84 d7 28 02        	movaps	0x228d784(%rip), %xmm1  # 0x142a8c720
1407fef9c: 44 0f 59 e1                 	mulps	%xmm1, %xmm12
1407fefa0: 44 0f 13 64 24 40           	movlps	%xmm12, 0x40(%rsp)
1407fefa6: f3 44 0f 11 6c 24 48        	movss	%xmm13, 0x48(%rsp)
1407fefad: c7 44 24 34 00 00 00 00     	movl	$0x0, 0x34(%rsp)
1407fefb5: f3 44 0f 11 4c 24 38        	movss	%xmm9, 0x38(%rsp)
1407fefbc: c7 44 24 3c 00 00 00 00     	movl	$0x0, 0x3c(%rsp)
1407fefc4: 44 0f 59 c1                 	mulps	%xmm1, %xmm8
1407fefc8: f3 44 0f 59 d8              	mulss	%xmm0, %xmm11
1407fefcd: 44 0f 13 84 24 80 00 00 00  	movlps	%xmm8, 0x80(%rsp)
1407fefd6: f3 44 0f 11 9c 24 88 00 00 00       	movss	%xmm11, 0x88(%rsp)
1407fefe0: 48 8d 44 24 70              	leaq	0x70(%rsp), %rax
1407fefe5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407fefea: 4c 8d b4 24 e0 00 00 00     	leaq	0xe0(%rsp), %r14
1407feff2: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
1407feff7: 4c 8d 44 24 34              	leaq	0x34(%rsp), %r8
1407feffc: 4c 8d 8c 24 80 00 00 00     	leaq	0x80(%rsp), %r9
1407ff004: 4c 89 f1                    	movq	%r14, %rcx
1407ff007: e8 f4 e4 09 00              	callq	0x14089d500 <.text+0x89c500>
1407ff00c: 89 d9                       	movl	%ebx, %ecx
1407ff00e: e8 3d 97 05 00              	callq	0x140858750 <.text+0x857750>
1407ff013: 89 c3                       	movl	%eax, %ebx
1407ff015: 48 8d 8c 24 80 00 00 00     	leaq	0x80(%rsp), %rcx
1407ff01d: e8 9e 97 05 00              	callq	0x1408587c0 <.text+0x8577c0>
1407ff022: 0f 10 84 24 b0 00 00 00     	movups	0xb0(%rsp), %xmm0
1407ff02a: 0f 11 84 24 10 01 00 00     	movups	%xmm0, 0x110(%rsp)
1407ff032: 48 8b 84 24 c0 00 00 00     	movq	0xc0(%rsp), %rax
1407ff03a: 48 89 84 24 20 01 00 00     	movq	%rax, 0x120(%rsp)
1407ff042: 8b 84 24 c8 00 00 00        	movl	0xc8(%rsp), %eax
1407ff049: 89 84 24 28 01 00 00        	movl	%eax, 0x128(%rsp)
1407ff050: 89 bc 24 2c 01 00 00        	movl	%edi, 0x12c(%rsp)
1407ff057: 89 9c 24 30 01 00 00        	movl	%ebx, 0x130(%rsp)
1407ff05e: f2 0f 10 05 4a 55 13 02     	movsd	0x213554a(%rip), %xmm0  # 0x1429345b0
1407ff066: f2 0f 11 84 24 34 01 00 00  	movsd	%xmm0, 0x134(%rsp)
1407ff06f: c7 84 24 3c 01 00 00 40 00 00 00    	movl	$0x40, 0x13c(%rsp)
1407ff07a: 48 89 f1                    	movq	%rsi, %rcx
1407ff07d: 4c 89 f2                    	movq	%r14, %rdx
1407ff080: e8 4b dd a8 00              	callq	0x14128cdd0 <.text+0x128bdd0>
1407ff085: 0f 28 b4 24 40 01 00 00     	movaps	0x140(%rsp), %xmm6
1407ff08d: 0f 28 bc 24 50 01 00 00     	movaps	0x150(%rsp), %xmm7
1407ff095: 44 0f 28 84 24 60 01 00 00  	movaps	0x160(%rsp), %xmm8
1407ff09e: 44 0f 28 8c 24 70 01 00 00  	movaps	0x170(%rsp), %xmm9
1407ff0a7: 44 0f 28 94 24 80 01 00 00  	movaps	0x180(%rsp), %xmm10
1407ff0b0: 44 0f 28 9c 24 90 01 00 00  	movaps	0x190(%rsp), %xmm11
1407ff0b9: 44 0f 28 a4 24 a0 01 00 00  	movaps	0x1a0(%rsp), %xmm12
1407ff0c2: 44 0f 28 ac 24 b0 01 00 00  	movaps	0x1b0(%rsp), %xmm13
1407ff0cb: 48 81 c4 c8 01 00 00        	addq	$0x1c8, %rsp            # imm = 0x1C8
1407ff0d2: 5b                          	popq	%rbx
1407ff0d3: 5d                          	popq	%rbp
1407ff0d4: 5f                          	popq	%rdi
1407ff0d5: 5e                          	popq	%rsi
1407ff0d6: 41 5c                       	popq	%r12
1407ff0d8: 41 5d                       	popq	%r13
1407ff0da: 41 5e                       	popq	%r14
1407ff0dc: 41 5f                       	popq	%r15
1407ff0de: c3                          	retq
1407ff0df: 4c 8d 05 ea da 28 02        	leaq	0x228daea(%rip), %r8    # 0x142a8cbd0
1407ff0e6: 31 c9                       	xorl	%ecx, %ecx
1407ff0e8: 31 d2                       	xorl	%edx, %edx
1407ff0ea: e8 29 a4 0d 02              	callq	0x1428d9518 <.text+0x28d8518>
1407ff0ef: 4c 8d 05 f2 da 28 02        	leaq	0x228daf2(%rip), %r8    # 0x142a8cbe8
1407ff0f6: 48 89 f9                    	movq	%rdi, %rcx
1407ff0f9: 48 89 c2                    	movq	%rax, %rdx
1407ff0fc: e8 17 a4 0d 02              	callq	0x1428d9518 <.text+0x28d8518>
1407ff101: 4c 8d 05 f8 da 28 02        	leaq	0x228daf8(%rip), %r8    # 0x142a8cc00
1407ff108: 48 89 f9                    	movq	%rdi, %rcx
1407ff10b: e8 08 a4 0d 02              	callq	0x1428d9518 <.text+0x28d8518>
1407ff110: 4c 8d 05 01 db 28 02        	leaq	0x228db01(%rip), %r8    # 0x142a8cc18
1407ff117: 48 89 f9                    	movq	%rdi, %rcx
1407ff11a: 4c 89 f2                    	movq	%r14, %rdx
1407ff11d: e8 f6 a3 0d 02              	callq	0x1428d9518 <.text+0x28d8518>
1407ff122: cc                          	int3

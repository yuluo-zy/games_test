
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407fdb70: 41 57                       	pushq	%r15
1407fdb72: 41 56                       	pushq	%r14
1407fdb74: 56                          	pushq	%rsi
1407fdb75: 57                          	pushq	%rdi
1407fdb76: 53                          	pushq	%rbx
1407fdb77: 48 81 ec 40 01 00 00        	subq	$0x140, %rsp            # imm = 0x140
1407fdb7e: 44 0f 29 b4 24 30 01 00 00  	movaps	%xmm14, 0x130(%rsp)
1407fdb87: 44 0f 29 ac 24 20 01 00 00  	movaps	%xmm13, 0x120(%rsp)
1407fdb90: 44 0f 29 a4 24 10 01 00 00  	movaps	%xmm12, 0x110(%rsp)
1407fdb99: 44 0f 29 9c 24 00 01 00 00  	movaps	%xmm11, 0x100(%rsp)
1407fdba2: 44 0f 29 94 24 f0 00 00 00  	movaps	%xmm10, 0xf0(%rsp)
1407fdbab: 44 0f 29 8c 24 e0 00 00 00  	movaps	%xmm9, 0xe0(%rsp)
1407fdbb4: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407fdbbd: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1407fdbc5: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1407fdbcd: 4c 8b bc 24 90 01 00 00     	movq	0x190(%rsp), %r15
1407fdbd5: 48 8b 01                    	movq	(%rcx), %rax
1407fdbd8: 42 80 3c 38 00              	cmpb	$0x0, (%rax,%r15)
1407fdbdd: 0f 84 14 03 00 00           	je	0x1407fdef7 <.text+0x7fcef7>
1407fdbe3: 4c 89 cb                    	movq	%r9, %rbx
1407fdbe6: 89 d7                       	movl	%edx, %edi
1407fdbe8: 48 89 ce                    	movq	%rcx, %rsi
1407fdbeb: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
1407fdbef: 89 54 24 40                 	movl	%edx, 0x40(%rsp)
1407fdbf3: 44 89 44 24 44              	movl	%r8d, 0x44(%rsp)
1407fdbf8: 48 83 c1 60                 	addq	$0x60, %rcx
1407fdbfc: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
1407fdc01: e8 9a fa a5 00              	callq	0x14125d6a0 <.text+0x125c6a0>
1407fdc06: 49 89 c6                    	movq	%rax, %r14
1407fdc09: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
1407fdc0d: e8 9e 0c 10 00              	callq	0x1408fe8b0 <.text+0x8fd8b0>
1407fdc12: 89 7c 24 40                 	movl	%edi, 0x40(%rsp)
1407fdc16: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
1407fdc1b: 48 89 c1                    	movq	%rax, %rcx
1407fdc1e: e8 2d 02 a6 00              	callq	0x14125de50 <.text+0x125ce50>
1407fdc23: 48 85 c0                    	testq	%rax, %rax
1407fdc26: 0f 84 cb 02 00 00           	je	0x1407fdef7 <.text+0x7fcef7>
1407fdc2c: 48 8b 40 10                 	movq	0x10(%rax), %rax
1407fdc30: 48 85 c0                    	testq	%rax, %rax
1407fdc33: 0f 84 be 02 00 00           	je	0x1407fdef7 <.text+0x7fcef7>
1407fdc39: 4d 85 f6                    	testq	%r14, %r14
1407fdc3c: 0f 84 b5 02 00 00           	je	0x1407fdef7 <.text+0x7fcef7>
1407fdc42: 48 8d 04 c5 f8 ff ff ff     	leaq	-0x8(,%rax,8), %rax
1407fdc4a: 48 83 f8 07                 	cmpq	$0x7, %rax
1407fdc4e: 0f 87 a3 02 00 00           	ja	0x1407fdef7 <.text+0x7fcef7>
1407fdc54: 41 80 3e 01                 	cmpb	$0x1, (%r14)
1407fdc58: 0f 85 99 02 00 00           	jne	0x1407fdef7 <.text+0x7fcef7>
1407fdc5e: 48 8b 4e 18                 	movq	0x18(%rsi), %rcx
1407fdc62: 89 fa                       	movl	%edi, %edx
1407fdc64: e8 a7 e8 21 00              	callq	0x140a1c510 <.text+0xa1b510>
1407fdc69: 48 85 c0                    	testq	%rax, %rax
1407fdc6c: 0f 85 85 02 00 00           	jne	0x1407fdef7 <.text+0x7fcef7>
1407fdc72: 48 89 d7                    	movq	%rdx, %rdi
1407fdc75: 48 8b 56 28                 	movq	0x28(%rsi), %rdx
1407fdc79: 48 39 da                    	cmpq	%rbx, %rdx
1407fdc7c: 0f 86 d3 02 00 00           	jbe	0x1407fdf55 <.text+0x7fcf55>
1407fdc82: 48 8b 56 38                 	movq	0x38(%rsi), %rdx
1407fdc86: 48 39 da                    	cmpq	%rbx, %rdx
1407fdc89: 0f 86 d5 02 00 00           	jbe	0x1407fdf64 <.text+0x7fcf64>
1407fdc8f: 48 8d 04 5b                 	leaq	(%rbx,%rbx,2), %rax
1407fdc93: 48 8b 4e 20                 	movq	0x20(%rsi), %rcx
1407fdc97: f2 44 0f 10 04 81           	movsd	(%rcx,%rax,4), %xmm8
1407fdc9d: f3 44 0f 10 4c 81 08        	movss	0x8(%rcx,%rax,4), %xmm9
1407fdca4: 48 8b 46 30                 	movq	0x30(%rsi), %rax
1407fdca8: f2 0f 10 04 d8              	movsd	(%rax,%rbx,8), %xmm0
1407fdcad: f2 0f 11 44 24 38           	movsd	%xmm0, 0x38(%rsp)
1407fdcb3: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
1407fdcb7: 48 39 da                    	cmpq	%rbx, %rdx
1407fdcba: 0f 86 b3 02 00 00           	jbe	0x1407fdf73 <.text+0x7fcf73>
1407fdcc0: 48 8b 46 40                 	movq	0x40(%rsi), %rax
1407fdcc4: f3 44 0f 10 15 1b c4 15 02  	movss	0x215c41b(%rip), %xmm10 # 0x14295a0e8
1407fdccd: f3 0f 10 34 98              	movss	(%rax,%rbx,4), %xmm6
1407fdcd2: f3 41 0f 58 f2              	addss	%xmm10, %xmm6
1407fdcd7: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
1407fdcdc: 48 8d 5c 24 38              	leaq	0x38(%rsp), %rbx
1407fdce1: 48 89 da                    	movq	%rbx, %rdx
1407fdce4: e8 d7 f0 49 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407fdce9: f3 0f 10 3d 7f 7b 12 02     	movss	0x2127b7f(%rip), %xmm7  # 0x142925870
1407fdcf1: f3 44 0f 10 6c 24 48        	movss	0x48(%rsp), %xmm13
1407fdcf8: f3 44 0f 59 ef              	mulss	%xmm7, %xmm13
1407fdcfd: f3 44 0f 59 ef              	mulss	%xmm7, %xmm13
1407fdd02: f3 45 0f 58 e9              	addss	%xmm9, %xmm13
1407fdd07: f2 44 0f 10 74 24 40        	movsd	0x40(%rsp), %xmm14
1407fdd0e: 44 0f 28 1d ca 7b 12 02     	movaps	0x2127bca(%rip), %xmm11 # 0x1429258e0
1407fdd16: 45 0f 59 f3                 	mulps	%xmm11, %xmm14
1407fdd1a: 45 0f 59 f3                 	mulps	%xmm11, %xmm14
1407fdd1e: 45 0f 58 f0                 	addps	%xmm8, %xmm14
1407fdd22: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
1407fdd27: 48 89 da                    	movq	%rbx, %rdx
1407fdd2a: e8 91 f0 49 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407fdd2f: f3 0f 10 44 24 48           	movss	0x48(%rsp), %xmm0
1407fdd35: 44 0f 28 25 b3 7b 12 02     	movaps	0x2127bb3(%rip), %xmm12 # 0x1429258f0
1407fdd3d: 41 0f 57 c4                 	xorps	%xmm12, %xmm0
1407fdd41: f2 0f 10 4c 24 40           	movsd	0x40(%rsp), %xmm1
1407fdd47: 41 0f 57 cc                 	xorps	%xmm12, %xmm1
1407fdd4b: 44 0f 13 b4 24 80 00 00 00  	movlps	%xmm14, 0x80(%rsp)
1407fdd54: f3 44 0f 11 ac 24 88 00 00 00       	movss	%xmm13, 0x88(%rsp)
1407fdd5e: 0f 13 8c 24 8c 00 00 00     	movlps	%xmm1, 0x8c(%rsp)
1407fdd66: f3 0f 11 84 24 94 00 00 00  	movss	%xmm0, 0x94(%rsp)
1407fdd6f: f3 41 0f 58 f2              	addss	%xmm10, %xmm6
1407fdd74: f3 0f 5f 35 08 7c 12 02     	maxss	0x2127c08(%rip), %xmm6  # 0x142925984
1407fdd7c: f3 0f 10 44 24 38           	movss	0x38(%rsp), %xmm0
1407fdd82: f3 0f 10 4c 24 3c           	movss	0x3c(%rsp), %xmm1
1407fdd88: 41 0f 57 cc                 	xorps	%xmm12, %xmm1
1407fdd8c: f3 0f 11 4c 24 68           	movss	%xmm1, 0x68(%rsp)
1407fdd92: f3 0f 11 44 24 6c           	movss	%xmm0, 0x6c(%rsp)
1407fdd98: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
1407fdd9d: 48 8d 54 24 68              	leaq	0x68(%rsp), %rdx
1407fdda2: e8 19 f0 49 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407fdda7: f3 44 0f 10 6c 24 48        	movss	0x48(%rsp), %xmm13
1407fddae: f3 44 0f 59 ee              	mulss	%xmm6, %xmm13
1407fddb3: f3 45 0f 59 ea              	mulss	%xmm10, %xmm13
1407fddb8: f3 45 0f 58 e9              	addss	%xmm9, %xmm13
1407fddbd: 48 8d 8c 24 a4 00 00 00     	leaq	0xa4(%rsp), %rcx
1407fddc5: f2 0f 10 44 24 40           	movsd	0x40(%rsp), %xmm0
1407fddcb: f3 0f 12 ce                 	movsldup	%xmm6, %xmm1            # xmm1 = xmm6[0,0,2,2]
1407fddcf: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407fddd2: 41 0f 59 cb                 	mulps	%xmm11, %xmm1
1407fddd6: 45 0f 28 d0                 	movaps	%xmm8, %xmm10
1407fddda: 44 0f 5c d1                 	subps	%xmm1, %xmm10
1407fddde: f3 0f 10 44 24 38           	movss	0x38(%rsp), %xmm0
1407fdde4: f3 0f 10 4c 24 3c           	movss	0x3c(%rsp), %xmm1
1407fddea: 41 0f 57 cc                 	xorps	%xmm12, %xmm1
1407fddee: f3 0f 11 4c 24 40           	movss	%xmm1, 0x40(%rsp)
1407fddf4: f3 0f 11 44 24 44           	movss	%xmm0, 0x44(%rsp)
1407fddfa: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
1407fddff: e8 bc ef 49 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407fde04: 44 0f 13 94 24 98 00 00 00  	movlps	%xmm10, 0x98(%rsp)
1407fde0d: f3 44 0f 11 ac 24 a0 00 00 00       	movss	%xmm13, 0xa0(%rsp)
1407fde17: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407fde1a: f3 41 0f 58 c1              	addss	%xmm9, %xmm0
1407fde1f: 44 0f 58 05 79 e8 28 02     	addps	0x228e879(%rip), %xmm8  # 0x142a8c6a0
1407fde27: 44 0f 13 44 24 68           	movlps	%xmm8, 0x68(%rsp)
1407fde2d: f3 0f 11 44 24 70           	movss	%xmm0, 0x70(%rsp)
1407fde33: f2 0f 10 05 c5 fe 1d 02     	movsd	0x21dfec5(%rip), %xmm0  # 0x1429ddd00
1407fde3b: f2 0f 11 44 24 74           	movsd	%xmm0, 0x74(%rsp)
1407fde41: c7 44 24 7c 00 00 00 80     	movl	$0x80000000, 0x7c(%rsp) # imm = 0x80000000
1407fde49: 48 8b 5e 50                 	movq	0x50(%rsi), %rbx
1407fde4d: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1407fde52: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1407fde5b: c7 44 24 20 01 00 00 00     	movl	$0x1, 0x20(%rsp)
1407fde63: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
1407fde68: 4c 8d 84 24 80 00 00 00     	leaq	0x80(%rsp), %r8
1407fde70: 48 89 da                    	movq	%rbx, %rdx
1407fde73: 0f 28 df                    	movaps	%xmm7, %xmm3
1407fde76: e8 d5 ed 0c 00              	callq	0x1408ccc50 <.text+0x8cbc50>
1407fde7b: 80 7c 24 40 11              	cmpb	$0x11, 0x40(%rsp)
1407fde80: 75 75                       	jne	0x1407fdef7 <.text+0x7fcef7>
1407fde82: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1407fde87: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1407fde90: c7 44 24 20 01 00 00 00     	movl	$0x1, 0x20(%rsp)
1407fde98: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
1407fde9d: 4c 8d 84 24 98 00 00 00     	leaq	0x98(%rsp), %r8
1407fdea5: 48 89 da                    	movq	%rbx, %rdx
1407fdea8: 0f 28 de                    	movaps	%xmm6, %xmm3
1407fdeab: e8 a0 ed 0c 00              	callq	0x1408ccc50 <.text+0x8cbc50>
1407fdeb0: 80 7c 24 40 11              	cmpb	$0x11, 0x40(%rsp)
1407fdeb5: 75 40                       	jne	0x1407fdef7 <.text+0x7fcef7>
1407fdeb7: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1407fdebc: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1407fdec5: c7 44 24 20 01 00 00 00     	movl	$0x1, 0x20(%rsp)
1407fdecd: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
1407fded2: 4c 8d 44 24 68              	leaq	0x68(%rsp), %r8
1407fded7: f3 0f 10 1d 91 79 12 02     	movss	0x2127991(%rip), %xmm3  # 0x142925870
1407fdedf: 48 89 da                    	movq	%rbx, %rdx
1407fdee2: e8 69 ed 0c 00              	callq	0x1408ccc50 <.text+0x8cbc50>
1407fdee7: 80 7c 24 40 11              	cmpb	$0x11, 0x40(%rsp)
1407fdeec: 75 09                       	jne	0x1407fdef7 <.text+0x7fcef7>
1407fdeee: 48 8b 46 58                 	movq	0x58(%rsi), %rax
1407fdef2: 42 c6 04 38 01              	movb	$0x1, (%rax,%r15)
1407fdef7: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1407fdeff: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1407fdf07: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407fdf10: 44 0f 28 8c 24 e0 00 00 00  	movaps	0xe0(%rsp), %xmm9
1407fdf19: 44 0f 28 94 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm10
1407fdf22: 44 0f 28 9c 24 00 01 00 00  	movaps	0x100(%rsp), %xmm11
1407fdf2b: 44 0f 28 a4 24 10 01 00 00  	movaps	0x110(%rsp), %xmm12
1407fdf34: 44 0f 28 ac 24 20 01 00 00  	movaps	0x120(%rsp), %xmm13
1407fdf3d: 44 0f 28 b4 24 30 01 00 00  	movaps	0x130(%rsp), %xmm14
1407fdf46: 48 81 c4 40 01 00 00        	addq	$0x140, %rsp            # imm = 0x140
1407fdf4d: 5b                          	popq	%rbx
1407fdf4e: 5f                          	popq	%rdi
1407fdf4f: 5e                          	popq	%rsi
1407fdf50: 41 5e                       	popq	%r14
1407fdf52: 41 5f                       	popq	%r15
1407fdf54: c3                          	retq
1407fdf55: 4c 8d 05 fc eb 28 02        	leaq	0x228ebfc(%rip), %r8    # 0x142a8cb58
1407fdf5c: 48 89 d9                    	movq	%rbx, %rcx
1407fdf5f: e8 b4 b5 0d 02              	callq	0x1428d9518 <.text+0x28d8518>
1407fdf64: 4c 8d 05 05 ec 28 02        	leaq	0x228ec05(%rip), %r8    # 0x142a8cb70
1407fdf6b: 48 89 d9                    	movq	%rbx, %rcx
1407fdf6e: e8 a5 b5 0d 02              	callq	0x1428d9518 <.text+0x28d8518>
1407fdf73: 4c 8d 05 0e ec 28 02        	leaq	0x228ec0e(%rip), %r8    # 0x142a8cb88
1407fdf7a: 48 89 d9                    	movq	%rbx, %rcx
1407fdf7d: e8 96 b5 0d 02              	callq	0x1428d9518 <.text+0x28d8518>
1407fdf82: cc                          	int3

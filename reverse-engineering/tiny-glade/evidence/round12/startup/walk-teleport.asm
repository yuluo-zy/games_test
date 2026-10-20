
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140994130: 41 57                       	pushq	%r15
140994132: 41 56                       	pushq	%r14
140994134: 41 55                       	pushq	%r13
140994136: 41 54                       	pushq	%r12
140994138: 56                          	pushq	%rsi
140994139: 57                          	pushq	%rdi
14099413a: 55                          	pushq	%rbp
14099413b: 53                          	pushq	%rbx
14099413c: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140994143: 0f 29 bc 24 80 00 00 00     	movaps	%xmm7, 0x80(%rsp)
14099414b: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
140994150: 4c 89 c6                    	movq	%r8, %rsi
140994153: f2 0f 10 32                 	movsd	(%rdx), %xmm6
140994157: f3 0f 10 7a 08              	movss	0x8(%rdx), %xmm7
14099415c: 4c 8b 71 08                 	movq	0x8(%rcx), %r14
140994160: 48 8b 59 10                 	movq	0x10(%rcx), %rbx
140994164: 49 89 df                    	movq	%rbx, %r15
140994167: 49 c1 e7 04                 	shlq	$0x4, %r15
14099416b: 49 bc 66 1a fc 7a 57 63 84 6b       	movabsq	$0x6b8463577afc1a66, %r12 # imm = 0x6B8463577AFC1A66
140994175: 49 bd e3 d7 68 41 29 9f 4b 0e       	movabsq	$0xe4b9f294168d7e3, %r13 # imm = 0xE4B9F294168D7E3
14099417f: 4d 89 f0                    	movq	%r14, %r8
140994182: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140994190: 4d 85 ff                    	testq	%r15, %r15
140994193: 0f 84 e3 00 00 00           	je	0x14099427c <.text+0x99327c>
140994199: 49 8d 68 10                 	leaq	0x10(%r8), %rbp
14099419d: 49 8b 08                    	movq	(%r8), %rcx
1409941a0: 49 8b 40 08                 	movq	0x8(%r8), %rax
1409941a4: ff 50 40                    	callq	*0x40(%rax)
1409941a7: 48 89 c7                    	movq	%rax, %rdi
1409941aa: 48 89 c1                    	movq	%rax, %rcx
1409941ad: ff 52 18                    	callq	*0x18(%rdx)
1409941b0: 49 83 c7 f0                 	addq	$-0x10, %r15
1409941b4: 49 89 e8                    	movq	%rbp, %r8
1409941b7: 48 85 ff                    	testq	%rdi, %rdi
1409941ba: 74 d4                       	je	0x140994190 <.text+0x993190>
1409941bc: 49 89 e8                    	movq	%rbp, %r8
1409941bf: 4c 39 e0                    	cmpq	%r12, %rax
1409941c2: 75 cc                       	jne	0x140994190 <.text+0x993190>
1409941c4: 49 89 e8                    	movq	%rbp, %r8
1409941c7: 4c 39 ea                    	cmpq	%r13, %rdx
1409941ca: 75 c4                       	jne	0x140994190 <.text+0x993190>
1409941cc: 0f 13 37                    	movlps	%xmm6, (%rdi)
1409941cf: f3 0f 11 7f 08              	movss	%xmm7, 0x8(%rdi)
1409941d4: 48 c1 e3 04                 	shlq	$0x4, %rbx
1409941d8: 49 bf e5 0f 33 58 d2 4f 8e 71       	movabsq	$0x718e4fd258330fe5, %r15 # imm = 0x718E4FD258330FE5
1409941e2: 49 bc bb 44 27 da b0 f9 f9 07       	movabsq	$0x7f9f9b0da2744bb, %r12 # imm = 0x7F9F9B0DA2744BB
1409941ec: 0f 1f 40 00                 	nopl	(%rax)
1409941f0: 48 85 db                    	testq	%rbx, %rbx
1409941f3: 0f 84 9a 00 00 00           	je	0x140994293 <.text+0x993293>
1409941f9: 4d 8d 6e 10                 	leaq	0x10(%r14), %r13
1409941fd: 49 8b 0e                    	movq	(%r14), %rcx
140994200: 49 8b 46 08                 	movq	0x8(%r14), %rax
140994204: ff 50 40                    	callq	*0x40(%rax)
140994207: 48 89 c7                    	movq	%rax, %rdi
14099420a: 48 89 c1                    	movq	%rax, %rcx
14099420d: ff 52 18                    	callq	*0x18(%rdx)
140994210: 48 83 c3 f0                 	addq	$-0x10, %rbx
140994214: 4d 89 ee                    	movq	%r13, %r14
140994217: 48 85 ff                    	testq	%rdi, %rdi
14099421a: 74 d4                       	je	0x1409941f0 <.text+0x9931f0>
14099421c: 4d 89 ee                    	movq	%r13, %r14
14099421f: 4c 39 f8                    	cmpq	%r15, %rax
140994222: 75 cc                       	jne	0x1409941f0 <.text+0x9931f0>
140994224: 4d 89 ee                    	movq	%r13, %r14
140994227: 4c 39 e2                    	cmpq	%r12, %rdx
14099422a: 75 c4                       	jne	0x1409941f0 <.text+0x9931f0>
14099422c: 48 8d 5c 24 40              	leaq	0x40(%rsp), %rbx
140994231: 48 89 d9                    	movq	%rbx, %rcx
140994234: 48 89 f2                    	movq	%rsi, %rdx
140994237: e8 44 0b 9a ff              	callq	0x140334d80 <.text+0x333d80>
14099423c: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140994241: b2 02                       	movb	$0x2, %dl
140994243: 49 89 d8                    	movq	%rbx, %r8
140994246: e8 45 28 dd 00              	callq	0x141766a90 <.text+0x1765a90>
14099424b: f2 0f 10 44 24 20           	movsd	0x20(%rsp), %xmm0
140994251: 0f 59 05 88 67 12 02        	mulps	0x2126788(%rip), %xmm0  # 0x142aba9e0
140994258: 0f 13 07                    	movlps	%xmm0, (%rdi)
14099425b: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
140994260: 0f 28 bc 24 80 00 00 00     	movaps	0x80(%rsp), %xmm7
140994268: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14099426f: 5b                          	popq	%rbx
140994270: 5d                          	popq	%rbp
140994271: 5f                          	popq	%rdi
140994272: 5e                          	popq	%rsi
140994273: 41 5c                       	popq	%r12
140994275: 41 5d                       	popq	%r13
140994277: 41 5e                       	popq	%r14
140994279: 41 5f                       	popq	%r15
14099427b: c3                          	retq
14099427c: 48 8d 05 65 8a 15 02        	leaq	0x2158a65(%rip), %rax   # 0x142aecce8
140994283: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140994288: 48 c7 44 24 28 22 00 00 00  	movq	$0x22, 0x28(%rsp)
140994291: eb 15                       	jmp	0x1409942a8 <.text+0x9932a8>
140994293: 48 8d 05 26 8b 15 02        	leaq	0x2158b26(%rip), %rax   # 0x142aecdc0
14099429a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14099429f: 48 c7 44 24 28 23 00 00 00  	movq	$0x23, 0x28(%rsp)
1409942a8: 48 8d 44 24 20              	leaq	0x20(%rsp), %rax
1409942ad: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1409942b2: 48 8d 05 d7 5b 77 ff        	leaq	-0x88a429(%rip), %rax   # 0x140109e90 <.text+0x108e90>
1409942b9: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1409942be: 48 8d 05 6b 8a 15 02        	leaq	0x2158a6b(%rip), %rax   # 0x142aecd30
1409942c5: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
1409942ca: 48 c7 44 24 48 02 00 00 00  	movq	$0x2, 0x48(%rsp)
1409942d3: 48 c7 44 24 60 00 00 00 00  	movq	$0x0, 0x60(%rsp)
1409942dc: 48 8d 44 24 30              	leaq	0x30(%rsp), %rax
1409942e1: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
1409942e6: 48 c7 44 24 58 01 00 00 00  	movq	$0x1, 0x58(%rsp)
1409942ef: 48 8d 15 b2 8a 15 02        	leaq	0x2158ab2(%rip), %rdx   # 0x142aecda8
1409942f6: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
1409942fb: e8 90 50 f4 01              	callq	0x1428d9390 <.text+0x28d8390>
140994300: cc                          	int3

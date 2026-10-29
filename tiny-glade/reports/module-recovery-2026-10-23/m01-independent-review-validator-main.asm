
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140827130: 41 57                       	pushq	%r15
140827132: 41 56                       	pushq	%r14
140827134: 41 55                       	pushq	%r13
140827136: 41 54                       	pushq	%r12
140827138: 56                          	pushq	%rsi
140827139: 57                          	pushq	%rdi
14082713a: 55                          	pushq	%rbp
14082713b: 53                          	pushq	%rbx
14082713c: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140827143: 4d 89 ce                    	movq	%r9, %r14
140827146: 4c 89 c7                    	movq	%r8, %rdi
140827149: 48 89 d6                    	movq	%rdx, %rsi
14082714c: 48 89 cb                    	movq	%rcx, %rbx
14082714f: 41 8b 40 1c                 	movl	0x1c(%r8), %eax
140827153: 49 8b 48 10                 	movq	0x10(%r8), %rcx
140827157: 89 01                       	movl	%eax, (%rcx)
140827159: 49 8b 08                    	movq	(%r8), %rcx
14082715c: e8 bf 75 1f 00              	callq	0x140a1e720 <.text+0xa1d720>
140827161: 4d 8b 2e                    	movq	(%r14), %r13
140827164: 49 83 7d 10 00              	cmpq	$0x0, 0x10(%r13)
140827169: 0f 84 c8 01 00 00           	je	0x140827337 <.text+0x826337>
14082716f: 4c 8b a4 24 50 01 00 00     	movq	0x150(%rsp), %r12
140827177: 4c 8b bc 24 48 01 00 00     	movq	0x148(%rsp), %r15
14082717f: 48 8b 8c 24 30 01 00 00     	movq	0x130(%rsp), %rcx
140827187: 4c 8b b4 24 28 01 00 00     	movq	0x128(%rsp), %r14
14082718f: 48 8b ac 24 20 01 00 00     	movq	0x120(%rsp), %rbp
140827197: e8 d4 f8 2f 00              	callq	0x140b26a70 <.text+0xb25a70>
14082719c: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1408271a1: 48 89 54 24 38              	movq	%rdx, 0x38(%rsp)
1408271a6: c6 44 24 2f 00              	movb	$0x0, 0x2f(%rsp)
1408271ab: 49 8b 45 18                 	movq	0x18(%r13), %rax
1408271af: 49 8b 4d 20                 	movq	0x20(%r13), %rcx
1408271b3: 48 8d 4c 08 01              	leaq	0x1(%rax,%rcx), %rcx
1408271b8: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
1408271bc: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1408271c0: f7 d2                       	notl	%edx
1408271c2: 66 89 54 24 58              	movw	%dx, 0x58(%rsp)
1408271c7: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
1408271cc: 48 83 c0 10                 	addq	$0x10, %rax
1408271d0: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
1408271d5: 48 89 4c 24 50              	movq	%rcx, 0x50(%rsp)
1408271da: 49 8b 45 30                 	movq	0x30(%r13), %rax
1408271de: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
1408271e3: 48 89 7c 24 68              	movq	%rdi, 0x68(%rsp)
1408271e8: 48 89 5c 24 70              	movq	%rbx, 0x70(%rsp)
1408271ed: 48 8d 44 24 30              	leaq	0x30(%rsp), %rax
1408271f2: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
1408271f7: 48 89 b4 24 80 00 00 00     	movq	%rsi, 0x80(%rsp)
1408271ff: 4c 89 b4 24 88 00 00 00     	movq	%r14, 0x88(%rsp)
140827207: 4c 89 a4 24 90 00 00 00     	movq	%r12, 0x90(%rsp)
14082720f: 4c 89 bc 24 98 00 00 00     	movq	%r15, 0x98(%rsp)
140827217: 48 89 ac 24 a0 00 00 00     	movq	%rbp, 0xa0(%rsp)
14082721f: 48 8d 44 24 2f              	leaq	0x2f(%rsp), %rax
140827224: 48 89 84 24 a8 00 00 00     	movq	%rax, 0xa8(%rsp)
14082722c: 48 8b 84 24 58 01 00 00     	movq	0x158(%rsp), %rax
140827234: 48 89 84 24 b0 00 00 00     	movq	%rax, 0xb0(%rsp)
14082723c: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
140827241: 48 8d 54 24 68              	leaq	0x68(%rsp), %rdx
140827246: e8 a5 12 a9 00              	callq	0x1412b84f0 <.text+0x12b74f0>
14082724b: 84 c0                       	testb	%al, %al
14082724d: 0f 84 e4 00 00 00           	je	0x140827337 <.text+0x826337>
140827253: 48 8b bc 24 38 01 00 00     	movq	0x138(%rsp), %rdi
14082725b: 48 8b 37                    	movq	(%rdi), %rsi
14082725e: 80 be 81 02 00 00 00        	cmpb	$0x0, 0x281(%rsi)
140827265: 0f 84 cc 00 00 00           	je	0x140827337 <.text+0x826337>
14082726b: 48 8b 9c 24 68 01 00 00     	movq	0x168(%rsp), %rbx
140827273: 80 7c 24 2f 00              	cmpb	$0x0, 0x2f(%rsp)
140827278: 75 70                       	jne	0x1408272ea <.text+0x8262ea>
14082727a: 48 8b 84 24 40 01 00 00     	movq	0x140(%rsp), %rax
140827282: 48 8b 00                    	movq	(%rax), %rax
140827285: 48 8b 08                    	movq	(%rax), %rcx
140827288: 48 8b 50 08                 	movq	0x8(%rax), %rdx
14082728c: 48 83 c1 fe                 	addq	$-0x2, %rcx
140827290: 48 83 d2 ff                 	adcq	$-0x1, %rdx
140827294: 45 31 c0                    	xorl	%r8d, %r8d
140827297: 48 83 f9 05                 	cmpq	$0x5, %rcx
14082729b: 48 83 da 00                 	sbbq	$0x0, %rdx
14082729f: 48 8d 49 01                 	leaq	0x1(%rcx), %rcx
1408272a3: 4c 0f 42 c1                 	cmovbq	%rcx, %r8
1408272a7: 48 8d 0d 52 a9 26 02        	leaq	0x226a952(%rip), %rcx   # 0x142a91c00
1408272ae: 4a 63 14 81                 	movslq	(%rcx,%r8,4), %rdx
1408272b2: 48 01 ca                    	addq	%rcx, %rdx
1408272b5: ff e2                       	jmpq	*%rdx
1408272b7: b9 5c 00 00 00              	movl	$0x5c, %ecx
1408272bc: eb 0c                       	jmp	0x1408272ca <.text+0x8262ca>
1408272be: b9 60 00 00 00              	movl	$0x60, %ecx
1408272c3: eb 05                       	jmp	0x1408272ca <.text+0x8262ca>
1408272c5: b9 69 00 00 00              	movl	$0x69, %ecx
1408272ca: 48 8b 94 24 60 01 00 00     	movq	0x160(%rsp), %rdx
1408272d2: 0f b6 0c 08                 	movzbl	(%rax,%rcx), %ecx
1408272d6: 48 8b 02                    	movq	(%rdx), %rax
1408272d9: f3 0f 10 08                 	movss	(%rax), %xmm1
1408272dd: f3 0f 10 50 04              	movss	0x4(%rax), %xmm2
1408272e2: 4d 8b 0f                    	movq	(%r15), %r9
1408272e5: e8 36 68 f8 ff              	callq	0x1407adb20 <.text+0x7acb20>
1408272ea: 8b 47 1c                    	movl	0x1c(%rdi), %eax
1408272ed: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
1408272f1: 89 01                       	movl	%eax, (%rcx)
1408272f3: c6 86 82 02 00 00 02        	movb	$0x2, 0x282(%rsi)
1408272fa: 8b 43 1c                    	movl	0x1c(%rbx), %eax
1408272fd: 48 8b 4b 10                 	movq	0x10(%rbx), %rcx
140827301: 89 01                       	movl	%eax, (%rcx)
140827303: 48 8b 33                    	movq	(%rbx), %rsi
140827306: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
14082730a: 48 8b 5e 40                 	movq	0x40(%rsi), %rbx
14082730e: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
140827312: 75 10                       	jne	0x140827324 <.text+0x826324>
140827314: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
140827318: 48 8d 15 61 ac 3f 02        	leaq	0x23fac61(%rip), %rdx   # 0x142c21f80
14082731f: e8 4c b5 d3 ff              	callq	0x140562870 <.text+0x561870>
140827324: 48 8b 46 28                 	movq	0x28(%rsi), %rax
140827328: 48 89 1c f8                 	movq	%rbx, (%rax,%rdi,8)
14082732c: 48 ff c7                    	incq	%rdi
14082732f: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
140827333: 48 ff 46 40                 	incq	0x40(%rsi)
140827337: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14082733e: 5b                          	popq	%rbx
14082733f: 5d                          	popq	%rbp
140827340: 5f                          	popq	%rdi
140827341: 5e                          	popq	%rsi
140827342: 41 5c                       	popq	%r12
140827344: 41 5d                       	popq	%r13
140827346: 41 5e                       	popq	%r14
140827348: 41 5f                       	popq	%r15
14082734a: c3                          	retq

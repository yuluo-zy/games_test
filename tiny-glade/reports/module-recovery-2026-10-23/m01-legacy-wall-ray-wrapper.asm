
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408cc680: 56                          	pushq	%rsi
1408cc681: 48 81 ec c0 00 00 00        	subq	$0xc0, %rsp
1408cc688: 48 89 ce                    	movq	%rcx, %rsi
1408cc68b: c7 44 24 30 fb 00 00 00     	movl	$0xfb, 0x30(%rsp)
1408cc693: c7 44 24 28 ff ff 7f 7f     	movl	$0x7f7fffff, 0x28(%rsp) # imm = 0x7F7FFFFF
1408cc69b: 48 8d 8c 24 80 00 00 00     	leaq	0x80(%rsp), %rcx
1408cc6a3: 45 31 c9                    	xorl	%r9d, %r9d
1408cc6a6: e8 b5 15 00 00              	callq	0x1408cdc60 <.text+0x8ccc60>
1408cc6ab: 83 bc 24 b4 00 00 00 02     	cmpl	$0x2, 0xb4(%rsp)
1408cc6b3: 75 0c                       	jne	0x1408cc6c1 <.text+0x8cb6c1>
1408cc6b5: c7 46 30 02 00 00 00        	movl	$0x2, 0x30(%rsi)
1408cc6bc: e9 96 00 00 00              	jmp	0x1408cc757 <.text+0x8cb757>
1408cc6c1: 48 8d 84 24 b4 00 00 00     	leaq	0xb4(%rsp), %rax
1408cc6c9: 8b 8c 24 a0 00 00 00        	movl	0xa0(%rsp), %ecx
1408cc6d0: 89 4c 24 60                 	movl	%ecx, 0x60(%rsp)
1408cc6d4: 48 8b 8c 24 98 00 00 00     	movq	0x98(%rsp), %rcx
1408cc6dc: 48 89 4c 24 58              	movq	%rcx, 0x58(%rsp)
1408cc6e1: 8b 8c 24 ac 00 00 00        	movl	0xac(%rsp), %ecx
1408cc6e8: 89 4c 24 6c                 	movl	%ecx, 0x6c(%rsp)
1408cc6ec: 48 8b 8c 24 a4 00 00 00     	movq	0xa4(%rsp), %rcx
1408cc6f4: 48 89 4c 24 64              	movq	%rcx, 0x64(%rsp)
1408cc6f9: 8b 48 08                    	movl	0x8(%rax), %ecx
1408cc6fc: 89 4c 24 78                 	movl	%ecx, 0x78(%rsp)
1408cc700: 48 8b 00                    	movq	(%rax), %rax
1408cc703: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
1408cc708: 48 8b 84 24 90 00 00 00     	movq	0x90(%rsp), %rax
1408cc710: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
1408cc715: 0f 10 84 24 80 00 00 00     	movups	0x80(%rsp), %xmm0
1408cc71d: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
1408cc722: 0f 11 06                    	movups	%xmm0, (%rsi)
1408cc725: 8b 44 24 60                 	movl	0x60(%rsp), %eax
1408cc729: 89 46 20                    	movl	%eax, 0x20(%rsi)
1408cc72c: 48 8b 44 24 64              	movq	0x64(%rsp), %rax
1408cc731: 48 89 46 24                 	movq	%rax, 0x24(%rsi)
1408cc735: 8b 44 24 6c                 	movl	0x6c(%rsp), %eax
1408cc739: 89 46 2c                    	movl	%eax, 0x2c(%rsi)
1408cc73c: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
1408cc741: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
1408cc745: 48 8b 44 24 58              	movq	0x58(%rsp), %rax
1408cc74a: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
1408cc74e: 0f 28 44 24 70              	movaps	0x70(%rsp), %xmm0
1408cc753: 0f 11 46 30                 	movups	%xmm0, 0x30(%rsi)
1408cc757: 48 89 f0                    	movq	%rsi, %rax
1408cc75a: 48 81 c4 c0 00 00 00        	addq	$0xc0, %rsp
1408cc761: 5e                          	popq	%rsi
1408cc762: c3                          	retq
1408cc763: cc                          	int3
1408cc764: cc                          	int3
1408cc765: cc                          	int3
1408cc766: cc                          	int3
1408cc767: cc                          	int3
1408cc768: cc                          	int3
1408cc769: cc                          	int3
1408cc76a: cc                          	int3
1408cc76b: cc                          	int3
1408cc76c: cc                          	int3
1408cc76d: cc                          	int3
1408cc76e: cc                          	int3
1408cc76f: cc                          	int3
1408cc770: 56                          	pushq	%rsi
1408cc771: 48 81 ec c0 00 00 00        	subq	$0xc0, %rsp
1408cc778: 48 89 ce                    	movq	%rcx, %rsi
1408cc77b: 48 8b 84 24 f0 00 00 00     	movq	0xf0(%rsp), %rax
1408cc783: b9 f3 00 00 00              	movl	$0xf3, %ecx
1408cc788: 0b 8c 24 f8 00 00 00        	orl	0xf8(%rsp), %ecx
1408cc78f: 89 4c 24 30                 	movl	%ecx, 0x30(%rsp)
1408cc793: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408cc798: c7 44 24 28 ff ff 7f 7f     	movl	$0x7f7fffff, 0x28(%rsp) # imm = 0x7F7FFFFF
1408cc7a0: 48 8d 8c 24 80 00 00 00     	leaq	0x80(%rsp), %rcx
1408cc7a8: e8 b3 14 00 00              	callq	0x1408cdc60 <.text+0x8ccc60>
1408cc7ad: 83 bc 24 b4 00 00 00 02     	cmpl	$0x2, 0xb4(%rsp)
1408cc7b5: 75 0c                       	jne	0x1408cc7c3 <.text+0x8cb7c3>
1408cc7b7: c7 46 30 02 00 00 00        	movl	$0x2, 0x30(%rsi)
1408cc7be: e9 96 00 00 00              	jmp	0x1408cc859 <.text+0x8cb859>
1408cc7c3: 48 8d 84 24 b4 00 00 00     	leaq	0xb4(%rsp), %rax
1408cc7cb: 8b 8c 24 a0 00 00 00        	movl	0xa0(%rsp), %ecx
1408cc7d2: 89 4c 24 60                 	movl	%ecx, 0x60(%rsp)
1408cc7d6: 48 8b 8c 24 98 00 00 00     	movq	0x98(%rsp), %rcx
1408cc7de: 48 89 4c 24 58              	movq	%rcx, 0x58(%rsp)
1408cc7e3: 8b 8c 24 ac 00 00 00        	movl	0xac(%rsp), %ecx
1408cc7ea: 89 4c 24 6c                 	movl	%ecx, 0x6c(%rsp)
1408cc7ee: 48 8b 8c 24 a4 00 00 00     	movq	0xa4(%rsp), %rcx
1408cc7f6: 48 89 4c 24 64              	movq	%rcx, 0x64(%rsp)
1408cc7fb: 8b 48 08                    	movl	0x8(%rax), %ecx
1408cc7fe: 89 4c 24 78                 	movl	%ecx, 0x78(%rsp)
1408cc802: 48 8b 00                    	movq	(%rax), %rax
1408cc805: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
1408cc80a: 48 8b 84 24 90 00 00 00     	movq	0x90(%rsp), %rax
1408cc812: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
1408cc817: 0f 10 84 24 80 00 00 00     	movups	0x80(%rsp), %xmm0
1408cc81f: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
1408cc824: 0f 11 06                    	movups	%xmm0, (%rsi)
1408cc827: 8b 44 24 60                 	movl	0x60(%rsp), %eax
1408cc82b: 89 46 20                    	movl	%eax, 0x20(%rsi)
1408cc82e: 48 8b 44 24 64              	movq	0x64(%rsp), %rax
1408cc833: 48 89 46 24                 	movq	%rax, 0x24(%rsi)
1408cc837: 8b 44 24 6c                 	movl	0x6c(%rsp), %eax
1408cc83b: 89 46 2c                    	movl	%eax, 0x2c(%rsi)
1408cc83e: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
1408cc843: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
1408cc847: 48 8b 44 24 58              	movq	0x58(%rsp), %rax
1408cc84c: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
1408cc850: 0f 28 44 24 70              	movaps	0x70(%rsp), %xmm0
1408cc855: 0f 11 46 30                 	movups	%xmm0, 0x30(%rsi)
1408cc859: 48 89 f0                    	movq	%rsi, %rax
1408cc85c: 48 81 c4 c0 00 00 00        	addq	$0xc0, %rsp
1408cc863: 5e                          	popq	%rsi
1408cc864: c3                          	retq
1408cc865: cc                          	int3
1408cc866: cc                          	int3
1408cc867: cc                          	int3
1408cc868: cc                          	int3
1408cc869: cc                          	int3
1408cc86a: cc                          	int3
1408cc86b: cc                          	int3
1408cc86c: cc                          	int3
1408cc86d: cc                          	int3
1408cc86e: cc                          	int3
1408cc86f: cc                          	int3

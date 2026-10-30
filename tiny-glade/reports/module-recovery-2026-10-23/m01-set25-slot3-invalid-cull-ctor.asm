
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140104d90: 55                          	pushq	%rbp
140104d91: 56                          	pushq	%rsi
140104d92: 57                          	pushq	%rdi
140104d93: 48 81 ec c0 01 00 00        	subq	$0x1c0, %rsp            # imm = 0x1C0
140104d9a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140104da2: 48 c7 85 38 01 00 00 fe ff ff ff    	movq	$-0x2, 0x138(%rbp)
140104dad: 48 89 ce                    	movq	%rcx, %rsi
140104db0: 0f 57 c0                    	xorps	%xmm0, %xmm0
140104db3: 0f 11 45 18                 	movups	%xmm0, 0x18(%rbp)
140104db7: 0f 11 45 30                 	movups	%xmm0, 0x30(%rbp)
140104dbb: 0f 11 45 48                 	movups	%xmm0, 0x48(%rbp)
140104dbf: 0f 11 45 60                 	movups	%xmm0, 0x60(%rbp)
140104dc3: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
140104dc7: c7 85 88 00 00 00 00 00 00 00       	movl	$0x0, 0x88(%rbp)
140104dd1: 0f 11 85 b0 00 00 00        	movups	%xmm0, 0xb0(%rbp)
140104dd8: 0f 11 85 c8 00 00 00        	movups	%xmm0, 0xc8(%rbp)
140104ddf: 0f 11 85 e0 00 00 00        	movups	%xmm0, 0xe0(%rbp)
140104de6: 0f 11 85 f8 00 00 00        	movups	%xmm0, 0xf8(%rbp)
140104ded: 0f 11 85 10 01 00 00        	movups	%xmm0, 0x110(%rbp)
140104df4: c7 85 20 01 00 00 00 00 00 00       	movl	$0x0, 0x120(%rbp)
140104dfe: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
140104e06: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
140104e0e: 48 c7 45 00 08 00 00 00     	movq	$0x8, (%rbp)
140104e16: 48 c7 45 08 00 00 00 00     	movq	$0x0, 0x8(%rbp)
140104e1e: 48 c7 45 10 10 00 00 00     	movq	$0x10, 0x10(%rbp)
140104e26: 48 c7 45 28 10 00 00 00     	movq	$0x10, 0x28(%rbp)
140104e2e: 48 c7 45 40 10 00 00 00     	movq	$0x10, 0x40(%rbp)
140104e36: 48 c7 45 58 10 00 00 00     	movq	$0x10, 0x58(%rbp)
140104e3e: 48 c7 45 70 10 00 00 00     	movq	$0x10, 0x70(%rbp)
140104e46: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140104e50: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
140104e57: 48 8d 05 c4 91 84 02        	leaq	0x28491c4(%rip), %rax   # 0x14294e022
140104e5e: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
140104e65: 48 c7 85 a0 00 00 00 57 00 00 00    	movq	$0x57, 0xa0(%rbp)
140104e70: 48 c7 85 a8 00 00 00 10 00 00 00    	movq	$0x10, 0xa8(%rbp)
140104e7b: 48 c7 85 c0 00 00 00 10 00 00 00    	movq	$0x10, 0xc0(%rbp)
140104e86: 48 c7 85 d8 00 00 00 10 00 00 00    	movq	$0x10, 0xd8(%rbp)
140104e91: 48 c7 85 f0 00 00 00 10 00 00 00    	movq	$0x10, 0xf0(%rbp)
140104e9c: 48 c7 85 08 01 00 00 10 00 00 00    	movq	$0x10, 0x108(%rbp)
140104ea7: c7 85 28 01 00 00 00 00 00 00       	movl	$0x0, 0x128(%rbp)
140104eb1: 66 c7 85 2c 01 00 00 01 00  	movw	$0x1, 0x12c(%rbp)
140104eba: c7 85 30 01 00 00 00 00 00 00       	movl	$0x0, 0x130(%rbp)
140104ec4: 0f b6 05 3e fd 52 03        	movzbl	0x352fd3e(%rip), %eax   # 0x143634c09
140104ecb: b9 98 01 00 00              	movl	$0x198, %ecx            # imm = 0x198
140104ed0: ba 08 00 00 00              	movl	$0x8, %edx
140104ed5: e8 36 ed 50 00              	callq	0x140613c10 <.text+0x612c10>
140104eda: 48 85 c0                    	testq	%rax, %rax
140104edd: 74 35                       	je	0x140104f14 <.text+0x103f14>
140104edf: 48 89 c7                    	movq	%rax, %rdi
140104ee2: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140104ee6: 41 b8 98 01 00 00           	movl	$0x198, %r8d            # imm = 0x198
140104eec: 48 89 c1                    	movq	%rax, %rcx
140104eef: e8 0c ed 81 02              	callq	0x142923c00 <.text+0x2922c00>
140104ef4: 4c 8d 05 e5 8d 84 02        	leaq	0x2848de5(%rip), %r8    # 0x14294dce0
140104efb: 48 89 f1                    	movq	%rsi, %rcx
140104efe: 48 89 fa                    	movq	%rdi, %rdx
140104f01: e8 0a fc bd 00              	callq	0x140ce4b10 <.text+0xce3b10>
140104f06: 48 89 f0                    	movq	%rsi, %rax
140104f09: 48 81 c4 c0 01 00 00        	addq	$0x1c0, %rsp            # imm = 0x1C0
140104f10: 5f                          	popq	%rdi
140104f11: 5e                          	popq	%rsi
140104f12: 5d                          	popq	%rbp
140104f13: c3                          	retq
140104f14: b9 08 00 00 00              	movl	$0x8, %ecx
140104f19: ba 98 01 00 00              	movl	$0x198, %edx            # imm = 0x198
140104f1e: e8 dd 40 7d 02              	callq	0x1428d9000 <.text+0x28d8000>

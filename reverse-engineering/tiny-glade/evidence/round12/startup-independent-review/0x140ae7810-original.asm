
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140ae7810: 55                          	pushq	%rbp
140ae7811: 41 57                       	pushq	%r15
140ae7813: 41 56                       	pushq	%r14
140ae7815: 56                          	pushq	%rsi
140ae7816: 57                          	pushq	%rdi
140ae7817: 53                          	pushq	%rbx
140ae7818: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140ae781f: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140ae7827: 48 c7 45 30 fe ff ff ff     	movq	$-0x2, 0x30(%rbp)
140ae782f: 4c 89 c6                    	movq	%r8, %rsi
140ae7832: 48 89 d7                    	movq	%rdx, %rdi
140ae7835: 48 89 cb                    	movq	%rcx, %rbx
140ae7838: 4d 8b 39                    	movq	(%r9), %r15
140ae783b: 41 0f b6 47 58              	movzbl	0x58(%r15), %eax
140ae7840: c1 e0 03                    	shll	$0x3, %eax
140ae7843: 48 8d 0d 6e c6 ff 01        	leaq	0x1ffc66e(%rip), %rcx   # 0x142ae3eb8
140ae784a: 4c 8b 0c 08                 	movq	(%rax,%rcx), %r9
140ae784e: 48 8d 0d 8b c6 ff 01        	leaq	0x1ffc68b(%rip), %rcx   # 0x142ae3ee0
140ae7855: 48 8b 04 08                 	movq	(%rax,%rcx), %rax
140ae7859: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140ae785e: 48 8d 15 b5 7c 00 02        	leaq	0x2007cb5(%rip), %rdx   # 0x142aef51a
140ae7865: 4c 8d 75 e0                 	leaq	-0x20(%rbp), %r14
140ae7869: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
140ae786f: 4c 89 f1                    	movq	%r14, %rcx
140ae7872: e8 99 af 27 00              	callq	0x140d62810 <.text+0xd61810>
140ae7877: 4c 89 f1                    	movq	%r14, %rcx
140ae787a: e8 51 f6 28 00              	callq	0x140d76ed0 <.text+0xd75ed0>
140ae787f: 49 89 d0                    	movq	%rdx, %r8
140ae7882: 48 c7 44 24 20 08 00 00 00  	movq	$0x8, 0x20(%rsp)
140ae788b: 4c 8d 0d be bc ff 01        	leaq	0x1ffbcbe(%rip), %r9    # 0x142ae3550
140ae7892: 48 89 e9                    	movq	%rbp, %rcx
140ae7895: 48 89 c2                    	movq	%rax, %rdx
140ae7898: e8 73 af 27 00              	callq	0x140d62810 <.text+0xd61810>
140ae789d: 48 89 e9                    	movq	%rbp, %rcx
140ae78a0: e8 2b f6 28 00              	callq	0x140d76ed0 <.text+0xd75ed0>
140ae78a5: 49 89 d0                    	movq	%rdx, %r8
140ae78a8: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140ae78ac: 48 89 c2                    	movq	%rax, %rdx
140ae78af: e8 8c 18 27 00              	callq	0x140d59140 <.text+0xd58140>
140ae78b4: 48 8b 55 00                 	movq	(%rbp), %rdx
140ae78b8: 48 85 d2                    	testq	%rdx, %rdx
140ae78bb: 74 0f                       	je	0x140ae78cc <.text+0xae68cc>
140ae78bd: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
140ae78c1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140ae78c7: e8 54 c3 b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae78cc: 48 8b 4d a8                 	movq	-0x58(%rbp), %rcx
140ae78d0: 48 89 c8                    	movq	%rcx, %rax
140ae78d3: 48 f7 d8                    	negq	%rax
140ae78d6: 0f 81 28 01 00 00           	jno	0x140ae7a04 <.text+0xae6a04>
140ae78dc: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
140ae78e0: 89 c1                       	movl	%eax, %ecx
140ae78e2: 83 e1 03                    	andl	$0x3, %ecx
140ae78e5: 83 f9 01                    	cmpl	$0x1, %ecx
140ae78e8: 75 54                       	jne	0x140ae793e <.text+0xae693e>
140ae78ea: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
140ae78ee: 48 89 4d d8                 	movq	%rcx, -0x28(%rbp)
140ae78f2: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
140ae78f6: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
140ae78fa: 48 8b 40 07                 	movq	0x7(%rax), %rax
140ae78fe: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140ae7902: 48 8b 00                    	movq	(%rax), %rax
140ae7905: 48 85 c0                    	testq	%rax, %rax
140ae7908: 74 06                       	je	0x140ae7910 <.text+0xae6910>
140ae790a: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
140ae790e: ff d0                       	callq	*%rax
140ae7910: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
140ae7914: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140ae7918: 48 8b 50 08                 	movq	0x8(%rax), %rdx
140ae791c: 48 85 d2                    	testq	%rdx, %rdx
140ae791f: 74 09                       	je	0x140ae792a <.text+0xae692a>
140ae7921: 4c 8b 40 10                 	movq	0x10(%rax), %r8
140ae7925: e8 f6 c2 b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae792a: ba 18 00 00 00              	movl	$0x18, %edx
140ae792f: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140ae7935: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
140ae7939: e8 e2 c2 b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae793e: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
140ae7942: 48 85 d2                    	testq	%rdx, %rdx
140ae7945: 74 0f                       	je	0x140ae7956 <.text+0xae6956>
140ae7947: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
140ae794b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140ae7951: e8 ca c2 b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae7956: 48 8d 15 e3 1b fb 01        	leaq	0x1fb1be3(%rip), %rdx   # 0x142a99540
140ae795d: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140ae7961: 41 b8 17 00 00 00           	movl	$0x17, %r8d
140ae7967: e8 d4 17 27 00              	callq	0x140d59140 <.text+0xd58140>
140ae796c: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
140ae7970: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140ae7974: 48 f7 d8                    	negq	%rax
140ae7977: 0f 80 8a 01 00 00           	jo	0x140ae7b07 <.text+0xae6b07>
140ae797d: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
140ae7981: 4c 8b 45 f0                 	movq	-0x10(%rbp), %r8
140ae7985: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140ae7989: 48 89 55 20                 	movq	%rdx, 0x20(%rbp)
140ae798d: e8 4e d8 91 00              	callq	0x1414051e0 <.text+0x14041e0>
140ae7992: 31 c0                       	xorl	%eax, %eax
140ae7994: 48 3b 45 e0                 	cmpq	-0x20(%rbp), %rax
140ae7998: 0f 80 cc 01 00 00           	jo	0x140ae7b6a <.text+0xae6b6a>
140ae799e: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140ae79a2: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140ae79a6: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140ae79aa: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140ae79ae: 8b 43 1c                    	movl	0x1c(%rbx), %eax
140ae79b1: 48 8b 4b 10                 	movq	0x10(%rbx), %rcx
140ae79b5: 89 01                       	movl	%eax, (%rcx)
140ae79b7: 48 8b 1b                    	movq	(%rbx), %rbx
140ae79ba: 48 8b 03                    	movq	(%rbx), %rax
140ae79bd: 48 85 c0                    	testq	%rax, %rax
140ae79c0: 74 17                       	je	0x140ae79d9 <.text+0xae69d9>
140ae79c2: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140ae79c6: 48 c1 e0 04                 	shlq	$0x4, %rax
140ae79ca: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
140ae79ce: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140ae79d4: e8 47 c2 b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae79d9: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140ae79dd: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140ae79e1: 0f 28 45 00                 	movaps	(%rbp), %xmm0
140ae79e5: 0f 11 03                    	movups	%xmm0, (%rbx)
140ae79e8: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
140ae79ec: 48 85 d2                    	testq	%rdx, %rdx
140ae79ef: 0f 84 ad 00 00 00           	je	0x140ae7aa2 <.text+0xae6aa2>
140ae79f5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140ae79fb: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
140ae79ff: e9 99 00 00 00              	jmp	0x140ae7a9d <.text+0xae6a9d>
140ae7a04: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
140ae7a08: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
140ae7a0c: 4c 8b 45 b8                 	movq	-0x48(%rbp), %r8
140ae7a10: 48 89 e9                    	movq	%rbp, %rcx
140ae7a13: 48 89 55 20                 	movq	%rdx, 0x20(%rbp)
140ae7a17: e8 c4 d7 91 00              	callq	0x1414051e0 <.text+0x14041e0>
140ae7a1c: 31 c0                       	xorl	%eax, %eax
140ae7a1e: 48 3b 45 00                 	cmpq	(%rbp), %rax
140ae7a22: 0f 80 10 01 00 00           	jo	0x140ae7b38 <.text+0xae6b38>
140ae7a28: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140ae7a2c: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
140ae7a30: 0f 10 45 00                 	movups	(%rbp), %xmm0
140ae7a34: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
140ae7a38: 8b 43 1c                    	movl	0x1c(%rbx), %eax
140ae7a3b: 48 8b 4b 10                 	movq	0x10(%rbx), %rcx
140ae7a3f: 89 01                       	movl	%eax, (%rcx)
140ae7a41: 48 8b 1b                    	movq	(%rbx), %rbx
140ae7a44: 48 8b 03                    	movq	(%rbx), %rax
140ae7a47: 48 85 c0                    	testq	%rax, %rax
140ae7a4a: 74 17                       	je	0x140ae7a63 <.text+0xae6a63>
140ae7a4c: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140ae7a50: 48 c1 e0 04                 	shlq	$0x4, %rax
140ae7a54: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
140ae7a58: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140ae7a5e: e8 bd c1 b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae7a63: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
140ae7a67: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140ae7a6b: 0f 28 45 c0                 	movaps	-0x40(%rbp), %xmm0
140ae7a6f: 0f 11 03                    	movups	%xmm0, (%rbx)
140ae7a72: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
140ae7a76: 48 85 d2                    	testq	%rdx, %rdx
140ae7a79: 74 0f                       	je	0x140ae7a8a <.text+0xae6a8a>
140ae7a7b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140ae7a81: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
140ae7a85: e8 96 c1 b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae7a8a: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
140ae7a8e: 48 85 d2                    	testq	%rdx, %rdx
140ae7a91: 74 0f                       	je	0x140ae7aa2 <.text+0xae6aa2>
140ae7a93: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
140ae7a97: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140ae7a9d: e8 7e c1 b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae7aa2: 8b 47 1c                    	movl	0x1c(%rdi), %eax
140ae7aa5: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
140ae7aa9: 89 01                       	movl	%eax, (%rcx)
140ae7aab: 48 8b 17                    	movq	(%rdi), %rdx
140ae7aae: f2 0f 10 05 fa ca e4 01     	movsd	0x1e4cafa(%rip), %xmm0  # 0x1429345b0
140ae7ab6: 0f 11 02                    	movups	%xmm0, (%rdx)
140ae7ab9: 4c 8b 06                    	movq	(%rsi), %r8
140ae7abc: f3 41 0f 10 40 08           	movss	0x8(%r8), %xmm0
140ae7ac2: 89 01                       	movl	%eax, (%rcx)
140ae7ac4: f3 0f 11 02                 	movss	%xmm0, (%rdx)
140ae7ac8: f3 41 0f 10 40 0c           	movss	0xc(%r8), %xmm0
140ae7ace: f3 0f 11 42 04              	movss	%xmm0, 0x4(%rdx)
140ae7ad3: 89 01                       	movl	%eax, (%rcx)
140ae7ad5: c7 42 08 00 00 00 00        	movl	$0x0, 0x8(%rdx)
140ae7adc: 41 80 7f 58 04              	cmpb	$0x4, 0x58(%r15)
140ae7ae1: 74 05                       	je	0x140ae7ae8 <.text+0xae6ae8>
140ae7ae3: 0f 57 c0                    	xorps	%xmm0, %xmm0
140ae7ae6: eb 08                       	jmp	0x140ae7af0 <.text+0xae6af0>
140ae7ae8: f3 0f 10 05 14 de e3 01     	movss	0x1e3de14(%rip), %xmm0  # 0x142925904
140ae7af0: 89 01                       	movl	%eax, (%rcx)
140ae7af2: f3 0f 11 42 0c              	movss	%xmm0, 0xc(%rdx)
140ae7af7: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140ae7afe: 5b                          	popq	%rbx
140ae7aff: 5f                          	popq	%rdi
140ae7b00: 5e                          	popq	%rsi
140ae7b01: 41 5e                       	popq	%r14
140ae7b03: 41 5f                       	popq	%r15
140ae7b05: 5d                          	popq	%rbp
140ae7b06: c3                          	retq
140ae7b07: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
140ae7b0b: 48 89 45 00                 	movq	%rax, (%rbp)
140ae7b0f: 48 8d 05 42 1a fb 01        	leaq	0x1fb1a42(%rip), %rax   # 0x142a99558
140ae7b16: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140ae7b1b: 48 8d 0d 5e 13 fb 01        	leaq	0x1fb135e(%rip), %rcx   # 0x142a98e80
140ae7b22: 4c 8d 0d 87 13 fb 01        	leaq	0x1fb1387(%rip), %r9    # 0x142a98eb0
140ae7b29: 49 89 e8                    	movq	%rbp, %r8
140ae7b2c: ba 2b 00 00 00              	movl	$0x2b, %edx
140ae7b31: e8 2a 1c df 01              	callq	0x1428d9760 <.text+0x28d8760>
140ae7b36: eb 61                       	jmp	0x140ae7b99 <.text+0xae6b99>
140ae7b38: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140ae7b3c: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140ae7b40: 48 8d 05 51 ba ff 01        	leaq	0x1ffba51(%rip), %rax   # 0x142ae3598
140ae7b47: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140ae7b4c: 48 8d 0d 4d b5 ff 01        	leaq	0x1ffb54d(%rip), %rcx   # 0x142ae30a0
140ae7b53: 4c 8d 0d 26 b5 ff 01        	leaq	0x1ffb526(%rip), %r9    # 0x142ae3080
140ae7b5a: 4c 8d 45 c0                 	leaq	-0x40(%rbp), %r8
140ae7b5e: ba 2b 00 00 00              	movl	$0x2b, %edx
140ae7b63: e8 f8 1b df 01              	callq	0x1428d9760 <.text+0x28d8760>
140ae7b68: eb 2f                       	jmp	0x140ae7b99 <.text+0xae6b99>
140ae7b6a: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
140ae7b6e: 48 89 45 00                 	movq	%rax, (%rbp)
140ae7b72: 48 8d 05 37 ba ff 01        	leaq	0x1ffba37(%rip), %rax   # 0x142ae35b0
140ae7b79: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140ae7b7e: 48 8d 0d 1b b5 ff 01        	leaq	0x1ffb51b(%rip), %rcx   # 0x142ae30a0
140ae7b85: 4c 8d 0d f4 b4 ff 01        	leaq	0x1ffb4f4(%rip), %r9    # 0x142ae3080
140ae7b8c: 49 89 e8                    	movq	%rbp, %r8
140ae7b8f: ba 2b 00 00 00              	movl	$0x2b, %edx
140ae7b94: e8 c7 1b df 01              	callq	0x1428d9760 <.text+0x28d8760>
140ae7b99: 0f 0b                       	ud2
140ae7b9b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140ae7ba0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ae7ba5: 55                          	pushq	%rbp
140ae7ba6: 41 57                       	pushq	%r15
140ae7ba8: 41 56                       	pushq	%r14
140ae7baa: 56                          	pushq	%rsi
140ae7bab: 57                          	pushq	%rdi
140ae7bac: 53                          	pushq	%rbx
140ae7bad: 48 83 ec 28                 	subq	$0x28, %rsp
140ae7bb1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ae7bb8: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
140ae7bbc: 48 85 d2                    	testq	%rdx, %rdx
140ae7bbf: 74 0f                       	je	0x140ae7bd0 <.text+0xae6bd0>
140ae7bc1: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
140ae7bc5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140ae7bcb: e8 50 c0 b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae7bd0: 90                          	nop
140ae7bd1: 48 83 c4 28                 	addq	$0x28, %rsp
140ae7bd5: 5b                          	popq	%rbx
140ae7bd6: 5f                          	popq	%rdi
140ae7bd7: 5e                          	popq	%rsi
140ae7bd8: 41 5e                       	popq	%r14
140ae7bda: 41 5f                       	popq	%r15
140ae7bdc: 5d                          	popq	%rbp
140ae7bdd: c3                          	retq
140ae7bde: 66 90                       	nop
140ae7be0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ae7be5: 55                          	pushq	%rbp
140ae7be6: 41 57                       	pushq	%r15
140ae7be8: 41 56                       	pushq	%r14
140ae7bea: 56                          	pushq	%rsi
140ae7beb: 57                          	pushq	%rdi
140ae7bec: 53                          	pushq	%rbx
140ae7bed: 48 83 ec 28                 	subq	$0x28, %rsp
140ae7bf1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ae7bf8: 48 8b 55 00                 	movq	(%rbp), %rdx
140ae7bfc: 48 85 d2                    	testq	%rdx, %rdx
140ae7bff: 74 0f                       	je	0x140ae7c10 <.text+0xae6c10>
140ae7c01: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
140ae7c05: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140ae7c0b: e8 10 c0 b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae7c10: 90                          	nop
140ae7c11: 48 83 c4 28                 	addq	$0x28, %rsp
140ae7c15: 5b                          	popq	%rbx
140ae7c16: 5f                          	popq	%rdi
140ae7c17: 5e                          	popq	%rsi
140ae7c18: 41 5e                       	popq	%r14
140ae7c1a: 41 5f                       	popq	%r15
140ae7c1c: 5d                          	popq	%rbp
140ae7c1d: c3                          	retq
140ae7c1e: 66 90                       	nop
140ae7c20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ae7c25: 55                          	pushq	%rbp
140ae7c26: 41 57                       	pushq	%r15
140ae7c28: 41 56                       	pushq	%r14
140ae7c2a: 56                          	pushq	%rsi
140ae7c2b: 57                          	pushq	%rdi
140ae7c2c: 53                          	pushq	%rbx
140ae7c2d: 48 83 ec 28                 	subq	$0x28, %rsp
140ae7c31: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ae7c38: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
140ae7c3c: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140ae7c40: 48 8b 50 08                 	movq	0x8(%rax), %rdx
140ae7c44: 48 85 d2                    	testq	%rdx, %rdx
140ae7c47: 74 0d                       	je	0x140ae7c56 <.text+0xae6c56>
140ae7c49: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140ae7c4d: 4c 8b 40 10                 	movq	0x10(%rax), %r8
140ae7c51: e8 ca bf b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae7c56: ba 18 00 00 00              	movl	$0x18, %edx
140ae7c5b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140ae7c61: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
140ae7c65: e8 b6 bf b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae7c6a: 90                          	nop
140ae7c6b: 48 83 c4 28                 	addq	$0x28, %rsp
140ae7c6f: 5b                          	popq	%rbx
140ae7c70: 5f                          	popq	%rdi
140ae7c71: 5e                          	popq	%rsi
140ae7c72: 41 5e                       	popq	%r14
140ae7c74: 41 5f                       	popq	%r15
140ae7c76: 5d                          	popq	%rbp
140ae7c77: c3                          	retq
140ae7c78: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140ae7c80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ae7c85: 55                          	pushq	%rbp
140ae7c86: 41 57                       	pushq	%r15
140ae7c88: 41 56                       	pushq	%r14
140ae7c8a: 56                          	pushq	%rsi
140ae7c8b: 57                          	pushq	%rdi
140ae7c8c: 53                          	pushq	%rbx
140ae7c8d: 48 83 ec 28                 	subq	$0x28, %rsp
140ae7c91: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ae7c98: 48 83 7d 28 00              	cmpq	$0x0, 0x28(%rbp)
140ae7c9d: 74 13                       	je	0x140ae7cb2 <.text+0xae6cb2>
140ae7c9f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140ae7ca5: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
140ae7ca9: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
140ae7cad: e8 6e bf b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae7cb2: 90                          	nop
140ae7cb3: 48 83 c4 28                 	addq	$0x28, %rsp
140ae7cb7: 5b                          	popq	%rbx
140ae7cb8: 5f                          	popq	%rdi
140ae7cb9: 5e                          	popq	%rsi
140ae7cba: 41 5e                       	popq	%r14
140ae7cbc: 41 5f                       	popq	%r15
140ae7cbe: 5d                          	popq	%rbp
140ae7cbf: c3                          	retq
140ae7cc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ae7cc5: 55                          	pushq	%rbp
140ae7cc6: 41 57                       	pushq	%r15
140ae7cc8: 41 56                       	pushq	%r14
140ae7cca: 56                          	pushq	%rsi
140ae7ccb: 57                          	pushq	%rdi
140ae7ccc: 53                          	pushq	%rbx
140ae7ccd: 48 83 ec 28                 	subq	$0x28, %rsp
140ae7cd1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ae7cd8: 48 83 7d 28 00              	cmpq	$0x0, 0x28(%rbp)
140ae7cdd: 74 13                       	je	0x140ae7cf2 <.text+0xae6cf2>
140ae7cdf: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140ae7ce5: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
140ae7ce9: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
140ae7ced: e8 2e bf b2 ff              	callq	0x140613c20 <.text+0x612c20>
140ae7cf2: 90                          	nop
140ae7cf3: 48 83 c4 28                 	addq	$0x28, %rsp
140ae7cf7: 5b                          	popq	%rbx
140ae7cf8: 5f                          	popq	%rdi
140ae7cf9: 5e                          	popq	%rsi
140ae7cfa: 41 5e                       	popq	%r14
140ae7cfc: 41 5f                       	popq	%r15
140ae7cfe: 5d                          	popq	%rbp
140ae7cff: c3                          	retq
140ae7d00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ae7d05: 55                          	pushq	%rbp
140ae7d06: 41 57                       	pushq	%r15
140ae7d08: 41 56                       	pushq	%r14
140ae7d0a: 56                          	pushq	%rsi
140ae7d0b: 57                          	pushq	%rdi
140ae7d0c: 53                          	pushq	%rbx
140ae7d0d: 48 83 ec 28                 	subq	$0x28, %rsp
140ae7d11: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ae7d18: 48 89 e9                    	movq	%rbp, %rcx
140ae7d1b: e8 e0 fc af ff              	callq	0x1405e7a00 <.text+0x5e6a00>
140ae7d20: 90                          	nop
140ae7d21: 48 83 c4 28                 	addq	$0x28, %rsp
140ae7d25: 5b                          	popq	%rbx
140ae7d26: 5f                          	popq	%rdi
140ae7d27: 5e                          	popq	%rsi
140ae7d28: 41 5e                       	popq	%r14
140ae7d2a: 41 5f                       	popq	%r15
140ae7d2c: 5d                          	popq	%rbp
140ae7d2d: c3                          	retq
140ae7d2e: 66 90                       	nop
140ae7d30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ae7d35: 55                          	pushq	%rbp
140ae7d36: 41 57                       	pushq	%r15
140ae7d38: 41 56                       	pushq	%r14
140ae7d3a: 56                          	pushq	%rsi
140ae7d3b: 57                          	pushq	%rdi
140ae7d3c: 53                          	pushq	%rbx
140ae7d3d: 48 83 ec 28                 	subq	$0x28, %rsp
140ae7d41: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ae7d48: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140ae7d4c: e8 4f 0b b0 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
140ae7d51: 90                          	nop
140ae7d52: 48 83 c4 28                 	addq	$0x28, %rsp
140ae7d56: 5b                          	popq	%rbx
140ae7d57: 5f                          	popq	%rdi
140ae7d58: 5e                          	popq	%rsi
140ae7d59: 41 5e                       	popq	%r14
140ae7d5b: 41 5f                       	popq	%r15
140ae7d5d: 5d                          	popq	%rbp
140ae7d5e: c3                          	retq
140ae7d5f: 90                          	nop
140ae7d60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ae7d65: 55                          	pushq	%rbp
140ae7d66: 41 57                       	pushq	%r15
140ae7d68: 41 56                       	pushq	%r14
140ae7d6a: 56                          	pushq	%rsi
140ae7d6b: 57                          	pushq	%rdi
140ae7d6c: 53                          	pushq	%rbx
140ae7d6d: 48 83 ec 28                 	subq	$0x28, %rsp
140ae7d71: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ae7d78: 48 89 e9                    	movq	%rbp, %rcx
140ae7d7b: e8 20 0b b0 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
140ae7d80: 90                          	nop
140ae7d81: 48 83 c4 28                 	addq	$0x28, %rsp
140ae7d85: 5b                          	popq	%rbx
140ae7d86: 5f                          	popq	%rdi
140ae7d87: 5e                          	popq	%rsi
140ae7d88: 41 5e                       	popq	%r14
140ae7d8a: 41 5f                       	popq	%r15
140ae7d8c: 5d                          	popq	%rbp
140ae7d8d: c3                          	retq

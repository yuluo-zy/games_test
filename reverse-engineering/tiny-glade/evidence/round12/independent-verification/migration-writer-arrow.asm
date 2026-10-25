
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1414729c9: 45 8d 61 ff                 	leal	-0x1(%r9), %r12d
1414729cd: 45 21 cc                    	andl	%r9d, %r12d
1414729d0: 48 ff c8                    	decq	%rax
1414729d3: 44 89 cb                    	movl	%r9d, %ebx
1414729d6: 4d 89 df                    	movq	%r11, %r15
1414729d9: 45 31 c0                    	xorl	%r8d, %r8d
1414729dc: 45 89 e1                    	movl	%r12d, %r9d
1414729df: f3 0f bc db                 	tzcntl	%ebx, %ebx
1414729e3: 48 f7 db                    	negq	%rbx
1414729e6: 48 6b db 58                 	imulq	$0x58, %rbx, %rbx
1414729ea: 41 80 7c 1f fa 3a           	cmpb	$0x3a, -0x6(%r15,%rbx)
1414729f0: 0f 84 a9 00 00 00           	je	0x141472a9f <.text+0x1471a9f>
1414729f6: 4d 85 c0                    	testq	%r8, %r8
1414729f9: 74 54                       	je	0x141472a4f <.text+0x1471a4f>
1414729fb: 48 85 c9                    	testq	%rcx, %rcx
1414729fe: 74 4d                       	je	0x141472a4d <.text+0x1471a4d>
141472a00: 66 85 d2                    	testw	%dx, %dx
141472a03: 74 14                       	je	0x141472a19 <.text+0x1471a19>
141472a05: 44 8d 62 ff                 	leal	-0x1(%rdx), %r12d
141472a09: 41 21 d4                    	andl	%edx, %r12d
141472a0c: 48 ff c9                    	decq	%rcx
141472a0f: 89 d3                       	movl	%edx, %ebx
141472a11: 4d 89 c7                    	movq	%r8, %r15
141472a14: 44 89 e2                    	movl	%r12d, %edx
141472a17: eb c6                       	jmp	0x1414729df <.text+0x14719df>
141472a19: 66 41 0f 6f 02              	movdqa	(%r10), %xmm0
141472a1e: 66 0f d7 d8                 	pmovmskb	%xmm0, %ebx
141472a22: 49 81 c0 80 fa ff ff        	addq	$-0x580, %r8            # imm = 0xFA80
141472a29: 49 83 c2 10                 	addq	$0x10, %r10
141472a2d: 81 fb ff ff 00 00           	cmpl	$0xffff, %ebx           # imm = 0xFFFF
141472a33: 74 e4                       	je	0x141472a19 <.text+0x1471a19>
141472a35: 41 bf fe ff ff ff           	movl	$0xfffffffe, %r15d      # imm = 0xFFFFFFFE
141472a3b: 41 29 df                    	subl	%ebx, %r15d
141472a3e: f7 d3                       	notl	%ebx
141472a40: 89 da                       	movl	%ebx, %edx
141472a42: 44 21 fa                    	andl	%r15d, %edx
141472a45: 48 ff c9                    	decq	%rcx
141472a48: 4d 89 c7                    	movq	%r8, %r15
141472a4b: eb 92                       	jmp	0x1414729df <.text+0x14719df>
141472a4d: 31 c9                       	xorl	%ecx, %ecx
141472a4f: 4d 85 db                    	testq	%r11, %r11
141472a52: 0f 84 b9 fe ff ff           	je	0x141472911 <.text+0x1471911>
141472a58: 48 85 c0                    	testq	%rax, %rax
141472a5b: 0f 84 b0 fe ff ff           	je	0x141472911 <.text+0x1471911>
141472a61: 66 45 85 c9                 	testw	%r9w, %r9w
141472a65: 0f 85 5e ff ff ff           	jne	0x1414729c9 <.text+0x14719c9>
141472a6b: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
141472a6f: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
141472a74: 49 81 c3 80 fa ff ff        	addq	$-0x580, %r11           # imm = 0xFA80
141472a7b: 48 83 c6 10                 	addq	$0x10, %rsi
141472a7f: 41 81 f9 ff ff 00 00        	cmpl	$0xffff, %r9d           # imm = 0xFFFF
141472a86: 74 e3                       	je	0x141472a6b <.text+0x1471a6b>
141472a88: 41 b8 fe ff ff ff           	movl	$0xfffffffe, %r8d       # imm = 0xFFFFFFFE
141472a8e: 45 29 c8                    	subl	%r9d, %r8d
141472a91: 41 f7 d1                    	notl	%r9d
141472a94: 45 89 cc                    	movl	%r9d, %r12d
141472a97: 45 21 c4                    	andl	%r8d, %r12d
141472a9a: e9 31 ff ff ff              	jmp	0x1414729d0 <.text+0x14719d0>
141472a9f: 49 01 df                    	addq	%rbx, %r15
141472aa2: 41 c7 47 d8 e7 03 00 00     	movl	$0x3e7, -0x28(%r15)     # imm = 0x3E7
141472aaa: b3 01                       	movb	$0x1, %bl
141472aac: 89 9d 04 03 00 00           	movl	%ebx, 0x304(%rbp)

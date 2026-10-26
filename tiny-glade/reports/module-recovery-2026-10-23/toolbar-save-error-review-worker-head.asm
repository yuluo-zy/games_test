
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1417207c1: e8 1a 2e 64 ff              	callq	0x140d635e0 <.text+0xd625e0>
1417207c6: 49 89 c4                    	movq	%rax, %r12
1417207c9: 89 d6                       	movl	%edx, %esi
1417207cb: 31 c0                       	xorl	%eax, %eax
1417207cd: 4c 3b a5 20 06 00 00        	cmpq	0x620(%rbp), %r12
1417207d4: 0f 95 c0                    	setne	%al
1417207d7: 41 0f 42 c7                 	cmovbl	%r15d, %eax
1417207db: 31 c9                       	xorl	%ecx, %ecx
1417207dd: 3b 95 54 06 00 00           	cmpl	0x654(%rbp), %edx
1417207e3: 0f 95 c1                    	setne	%cl
1417207e6: 41 0f 42 cf                 	cmovbl	%r15d, %ecx
1417207ea: 84 c0                       	testb	%al, %al
1417207ec: 0f 45 c8                    	cmovnel	%eax, %ecx
1417207ef: 80 f9 01                    	cmpb	$0x1, %cl
1417207f2: 0f 85 38 06 00 00           	jne	0x141720e30 <.text+0x171fe30>
1417207f8: 48 c7 85 60 06 00 00 00 00 00 00    	movq	$0x0, 0x660(%rbp)
141720803: 48 c7 85 68 06 00 00 08 00 00 00    	movq	$0x8, 0x668(%rbp)
14172080e: 48 c7 85 70 06 00 00 00 00 00 00    	movq	$0x0, 0x670(%rbp)
141720819: 83 7d a8 03                 	cmpl	$0x3, -0x58(%rbp)
14172081d: 0f 84 fb 00 00 00           	je	0x14172091e <.text+0x171f91e>
141720823: 48 83 7d 30 00              	cmpq	$0x0, 0x30(%rbp)
141720828: 0f 84 f0 00 00 00           	je	0x14172091e <.text+0x171f91e>
14172082e: c6 85 87 06 00 00 01        	movb	$0x1, 0x687(%rbp)
141720835: 48 89 f9                    	movq	%rdi, %rcx
141720838: e8 f3 99 1f ff              	callq	0x14091a230 <.text+0x919230>
14172083d: 48 83 7d 30 00              	cmpq	$0x0, 0x30(%rbp)
141720842: 0f 84 d6 00 00 00           	je	0x14172091e <.text+0x171f91e>
141720848: c6 85 87 06 00 00 01        	movb	$0x1, 0x687(%rbp)
14172084f: 48 89 f9                    	movq	%rdi, %rcx
141720852: e8 79 66 65 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
141720857: 49 89 d0                    	movq	%rdx, %r8
14172085a: c6 85 87 06 00 00 01        	movb	$0x1, 0x687(%rbp)
141720861: 48 8d 8d 50 05 00 00        	leaq	0x550(%rbp), %rcx
141720868: 48 89 c2                    	movq	%rax, %rdx
14172086b: e8 90 12 64 ff              	callq	0x140d61b00 <.text+0xd60b00>
141720870: 48 8d 8d 90 05 00 00        	leaq	0x590(%rbp), %rcx
141720877: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
14172087b: e8 10 fc ff ff              	callq	0x141720490 <.text+0x171f490>
141720880: 0f 10 85 50 05 00 00        	movups	0x550(%rbp), %xmm0
141720887: 0f 10 8d 60 05 00 00        	movups	0x560(%rbp), %xmm1
14172088e: 0f 29 8d 60 02 00 00        	movaps	%xmm1, 0x260(%rbp)
141720895: 0f 29 85 50 02 00 00        	movaps	%xmm0, 0x250(%rbp)
14172089c: 0f 10 85 90 05 00 00        	movups	0x590(%rbp), %xmm0
1417208a3: 0f 10 8d a0 05 00 00        	movups	0x5a0(%rbp), %xmm1
1417208aa: 48 8d 85 70 02 00 00        	leaq	0x270(%rbp), %rax
1417208b1: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1417208b5: 0f 11 00                    	movups	%xmm0, (%rax)
1417208b8: 48 8b 9d 70 06 00 00        	movq	0x670(%rbp), %rbx
1417208bf: 48 3b 9d 60 06 00 00        	cmpq	0x660(%rbp), %rbx
1417208c6: 75 0f                       	jne	0x1417208d7 <.text+0x171f8d7>
1417208c8: 4c 89 e9                    	movq	%r13, %rcx
1417208cb: 48 8d 15 ee 9f 5a 01        	leaq	0x15a9fee(%rip), %rdx   # 0x142cca8c0
1417208d2: e8 d9 1e e4 fe              	callq	0x1405627b0 <.text+0x5617b0>
1417208d7: 48 8b 85 68 06 00 00        	movq	0x668(%rbp), %rax
1417208de: 48 89 d9                    	movq	%rbx, %rcx
1417208e1: 48 c1 e1 06                 	shlq	$0x6, %rcx
1417208e5: 0f 28 85 50 02 00 00        	movaps	0x250(%rbp), %xmm0
1417208ec: 0f 28 8d 60 02 00 00        	movaps	0x260(%rbp), %xmm1
1417208f3: 0f 28 95 70 02 00 00        	movaps	0x270(%rbp), %xmm2
1417208fa: 0f 28 9d 80 02 00 00        	movaps	0x280(%rbp), %xmm3
141720901: 0f 11 5c 08 30              	movups	%xmm3, 0x30(%rax,%rcx)
141720906: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
14172090b: 0f 11 4c 08 10              	movups	%xmm1, 0x10(%rax,%rcx)
141720910: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
141720914: 48 ff c3                    	incq	%rbx
141720917: 48 89 9d 70 06 00 00        	movq	%rbx, 0x670(%rbp)
14172091e: 48 8b 9d 10 07 00 00        	movq	0x710(%rbp), %rbx
141720925: 48 8d 53 10                 	leaq	0x10(%rbx), %rdx
141720929: 31 c0                       	xorl	%eax, %eax
14172092b: b1 01                       	movb	$0x1, %cl
14172092d: f0                          	lock
14172092e: 0f b0 4b 10                 	cmpxchgb	%cl, 0x10(%rbx)
141720932: 48 89 95 28 06 00 00        	movq	%rdx, 0x628(%rbp)
141720939: 0f 85 ba 08 00 00           	jne	0x1417211f9 <.text+0x17201f9>
14172093f: c6 85 87 06 00 00 01        	movb	$0x1, 0x687(%rbp)
141720946: 48 89 d1                    	movq	%rdx, %rcx
141720949: e8 22 13 56 ff              	callq	0x140c81c70 <.text+0xc80c70>
14172094e: 48 83 c3 18                 	addq	$0x18, %rbx

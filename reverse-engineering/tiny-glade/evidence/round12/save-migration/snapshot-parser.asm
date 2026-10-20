
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1417a6e50: 55                          	pushq	%rbp
1417a6e51: 41 57                       	pushq	%r15
1417a6e53: 41 56                       	pushq	%r14
1417a6e55: 41 55                       	pushq	%r13
1417a6e57: 41 54                       	pushq	%r12
1417a6e59: 56                          	pushq	%rsi
1417a6e5a: 57                          	pushq	%rdi
1417a6e5b: 53                          	pushq	%rbx
1417a6e5c: 48 81 ec 18 06 00 00        	subq	$0x618, %rsp            # imm = 0x618
1417a6e63: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1417a6e6b: 0f 29 b5 80 05 00 00        	movaps	%xmm6, 0x580(%rbp)
1417a6e72: 48 c7 85 78 05 00 00 fe ff ff ff    	movq	$-0x2, 0x578(%rbp)
1417a6e7d: 4c 89 cf                    	movq	%r9, %rdi
1417a6e80: 4c 89 85 20 04 00 00        	movq	%r8, 0x420(%rbp)
1417a6e87: 49 89 d7                    	movq	%rdx, %r15
1417a6e8a: 48 89 cb                    	movq	%rcx, %rbx
1417a6e8d: 48 8b 85 00 06 00 00        	movq	0x600(%rbp), %rax
1417a6e94: 0f 57 f6                    	xorps	%xmm6, %xmm6
1417a6e97: 48 8b 80 20 01 00 00        	movq	0x120(%rax), %rax
1417a6e9e: 48 89 85 38 02 00 00        	movq	%rax, 0x238(%rbp)
1417a6ea5: 48 8d 42 18                 	leaq	0x18(%rdx), %rax
1417a6ea9: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
1417a6eb0: 48 8d b5 e0 04 00 00        	leaq	0x4e0(%rbp), %rsi
1417a6eb7: 4c 89 8d c8 04 00 00        	movq	%r9, 0x4c8(%rbp)
1417a6ebe: eb 2a                       	jmp	0x1417a6eea <.text+0x17a5eea>
1417a6ec0: ba 18 00 00 00              	movl	$0x18, %edx
1417a6ec5: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a6ecb: 48 8b 8d d0 04 00 00        	movq	0x4d0(%rbp), %rcx
1417a6ed2: e8 49 cd e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a6ed7: c6 85 77 05 00 00 00        	movb	$0x0, 0x577(%rbp)
1417a6ede: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a6ee5: e8 46 5d 5a ff              	callq	0x140d4cc30 <.text+0xd4bc30>
1417a6eea: 45 8b 47 68                 	movl	0x68(%r15), %r8d
1417a6eee: 48 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %rcx
1417a6ef5: 4c 89 fa                    	movq	%r15, %rdx
1417a6ef8: e8 53 db 2d ff              	callq	0x140a84a50 <.text+0xa83a50>
1417a6efd: f6 85 f8 00 00 00 01        	testb	$0x1, 0xf8(%rbp)
1417a6f04: 0f 84 84 1c 00 00           	je	0x1417a8b8e <.text+0x17a7b8e>
1417a6f0a: 44 8b ad 08 01 00 00        	movl	0x108(%rbp), %r13d
1417a6f11: 4c 8b a5 00 01 00 00        	movq	0x100(%rbp), %r12
1417a6f18: 4c 89 a5 00 02 00 00        	movq	%r12, 0x200(%rbp)
1417a6f1f: 48 8d 85 00 02 00 00        	leaq	0x200(%rbp), %rax
1417a6f26: 48 89 85 30 04 00 00        	movq	%rax, 0x430(%rbp)
1417a6f2d: 48 8d 05 8c 67 5e ff        	leaq	-0xa19874(%rip), %rax   # 0x140d8d6c0 <.text+0xd8c6c0>
1417a6f34: 48 89 85 38 04 00 00        	movq	%rax, 0x438(%rbp)
1417a6f3b: 48 8d 05 26 0f 33 01        	leaq	0x1330f26(%rip), %rax   # 0x142ad7e68
1417a6f42: 48 89 85 40 04 00 00        	movq	%rax, 0x440(%rbp)
1417a6f49: 48 8d 05 40 2f 96 fe        	leaq	-0x169d0c0(%rip), %rax  # 0x140109e90 <.text+0x108e90>
1417a6f50: 48 89 85 48 04 00 00        	movq	%rax, 0x448(%rbp)
1417a6f57: 48 8d 05 22 0f 33 01        	leaq	0x1330f22(%rip), %rax   # 0x142ad7e80
1417a6f5e: 48 89 85 e0 04 00 00        	movq	%rax, 0x4e0(%rbp)
1417a6f65: 48 c7 85 e8 04 00 00 02 00 00 00    	movq	$0x2, 0x4e8(%rbp)
1417a6f70: 48 8d 85 30 04 00 00        	leaq	0x430(%rbp), %rax
1417a6f77: 48 89 85 f0 04 00 00        	movq	%rax, 0x4f0(%rbp)
1417a6f7e: 48 c7 85 f8 04 00 00 02 00 00 00    	movq	$0x2, 0x4f8(%rbp)
1417a6f89: 48 c7 85 00 05 00 00 00 00 00 00    	movq	$0x0, 0x500(%rbp)
1417a6f94: 48 8d 8d d8 03 00 00        	leaq	0x3d8(%rbp), %rcx
1417a6f9b: 48 89 f2                    	movq	%rsi, %rdx
1417a6f9e: e8 ed 2f 5d ff              	callq	0x140d79f90 <.text+0xd78f90>
1417a6fa3: 48 8b 95 e0 03 00 00        	movq	0x3e0(%rbp), %rdx
1417a6faa: 4c 8b 85 e8 03 00 00        	movq	0x3e8(%rbp), %r8
1417a6fb1: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
1417a6fb6: 48 8d 8d e8 02 00 00        	leaq	0x2e8(%rbp), %rcx
1417a6fbd: 48 89 95 68 05 00 00        	movq	%rdx, 0x568(%rbp)
1417a6fc4: 4c 8b 8d 20 04 00 00        	movq	0x420(%rbp), %r9
1417a6fcb: e8 00 fb b3 ff              	callq	0x1412e6ad0 <.text+0x12e5ad0>
1417a6fd0: 48 8b 95 d8 03 00 00        	movq	0x3d8(%rbp), %rdx
1417a6fd7: 48 85 d2                    	testq	%rdx, %rdx
1417a6fda: 74 12                       	je	0x1417a6fee <.text+0x17a5fee>
1417a6fdc: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1417a6fe2: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a6fe9: e8 32 cc e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a6fee: c6 85 77 05 00 00 01        	movb	$0x1, 0x577(%rbp)
1417a6ff5: 48 8d 8d e8 02 00 00        	leaq	0x2e8(%rbp), %rcx
1417a6ffc: e8 cf fe 5c ff              	callq	0x140d76ed0 <.text+0xd75ed0>
1417a7001: 49 89 d0                    	movq	%rdx, %r8
1417a7004: c7 85 e0 04 00 00 00 00 00 00       	movl	$0x0, 0x4e0(%rbp)
1417a700e: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7015: 0f 11 31                    	movups	%xmm6, (%rcx)
1417a7018: c7 85 f8 04 00 00 07 00 00 00       	movl	$0x7, 0x4f8(%rbp)
1417a7022: 66 c7 41 1c 00 00           	movw	$0x0, 0x1c(%rcx)
1417a7028: 48 c7 41 14 00 00 00 00     	movq	$0x0, 0x14(%rcx)
1417a7030: c6 85 00 05 00 00 01        	movb	$0x1, 0x500(%rbp)
1417a7037: c6 85 77 05 00 00 01        	movb	$0x1, 0x577(%rbp)
1417a703e: 48 89 f1                    	movq	%rsi, %rcx
1417a7041: 48 89 c2                    	movq	%rax, %rdx
1417a7044: e8 77 2d 5b ff              	callq	0x140d59dc0 <.text+0xd58dc0>
1417a7049: 48 85 c0                    	testq	%rax, %rax
1417a704c: 74 12                       	je	0x1417a7060 <.text+0x17a6060>
1417a704e: c6 85 77 05 00 00 01        	movb	$0x1, 0x577(%rbp)
1417a7055: 48 89 d1                    	movq	%rdx, %rcx
1417a7058: e8 33 25 10 01              	callq	0x1428a9590 <.text+0x28a8590>
1417a705d: eb 6d                       	jmp	0x1417a70cc <.text+0x17a60cc>
1417a705f: 90                          	nop
1417a7060: 48 89 95 68 05 00 00        	movq	%rdx, 0x568(%rbp)
1417a7067: 48 89 d1                    	movq	%rdx, %rcx
1417a706a: e8 a1 be 3e ff              	callq	0x140b92f10 <.text+0xb91f10>
1417a706f: 49 89 d6                    	movq	%rdx, %r14
1417a7072: 48 85 c0                    	testq	%rax, %rax
1417a7075: 75 39                       	jne	0x1417a70b0 <.text+0x17a60b0>
1417a7077: c6 44 24 20 00              	movb	$0x0, 0x20(%rsp)
1417a707c: 48 89 f1                    	movq	%rsi, %rcx
1417a707f: 4c 89 f2                    	movq	%r14, %rdx
1417a7082: 4c 8b 85 68 05 00 00        	movq	0x568(%rbp), %r8
1417a7089: 45 31 c9                    	xorl	%r9d, %r9d
1417a708c: e8 0f 5a ee ff              	callq	0x14168caa0 <.text+0x168baa0>
1417a7091: 48 8b bd e0 04 00 00        	movq	0x4e0(%rbp), %rdi
1417a7098: 4c 8b b5 e8 04 00 00        	movq	0x4e8(%rbp), %r14
1417a709f: 48 83 ff 02                 	cmpq	$0x2, %rdi
1417a70a3: 0f 85 52 03 00 00           	jne	0x1417a73fb <.text+0x17a63fb>
1417a70a9: 48 8b bd c8 04 00 00        	movq	0x4c8(%rbp), %rdi
1417a70b0: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a70b7: ff 15 33 b3 9e 01           	callq	*0x19eb333(%rip)        # 0x1431923f0
1417a70bd: c6 85 77 05 00 00 01        	movb	$0x1, 0x577(%rbp)
1417a70c4: 4c 89 f1                    	movq	%r14, %rcx
1417a70c7: e8 c4 24 10 01              	callq	0x1428a9590 <.text+0x28a8590>
1417a70cc: 49 89 c6                    	movq	%rax, %r14
1417a70cf: 4c 89 b5 90 03 00 00        	movq	%r14, 0x390(%rbp)
1417a70d6: 48 8b 05 cb 52 e8 01        	movq	0x1e852cb(%rip), %rax   # 0x14362c3a8
1417a70dd: 48 8b 00                    	movq	(%rax), %rax
1417a70e0: 48 83 f8 01                 	cmpq	$0x1, %rax
1417a70e4: 0f 86 c5 01 00 00           	jbe	0x1417a72af <.text+0x17a62af>
1417a70ea: c6 85 76 05 00 00 01        	movb	$0x1, 0x576(%rbp)
1417a70f1: 48 8d 8d e8 02 00 00        	leaq	0x2e8(%rbp), %rcx
1417a70f8: e8 d3 fd 5c ff              	callq	0x140d76ed0 <.text+0xd75ed0>
1417a70fd: 49 89 d0                    	movq	%rdx, %r8
1417a7100: c6 85 76 05 00 00 01        	movb	$0x1, 0x576(%rbp)
1417a7107: 41 b9 04 00 00 00           	movl	$0x4, %r9d
1417a710d: 48 8d 8d d8 03 00 00        	leaq	0x3d8(%rbp), %rcx
1417a7114: 48 89 c2                    	movq	%rax, %rdx
1417a7117: e8 a4 f9 f4 ff              	callq	0x1416f6ac0 <.text+0x16f5ac0>
1417a711c: 48 8d 85 d8 03 00 00        	leaq	0x3d8(%rbp), %rax
1417a7123: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1417a712a: 48 8d 05 af a9 5b ff        	leaq	-0xa45651(%rip), %rax   # 0x140d61ae0 <.text+0xd60ae0>
1417a7131: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
1417a7138: 48 8d 85 90 03 00 00        	leaq	0x390(%rbp), %rax
1417a713f: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1417a7146: 48 8d 05 d3 5a 5a ff        	leaq	-0xa5a52d(%rip), %rax   # 0x140d4cc20 <.text+0xd4bc20>
1417a714d: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
1417a7154: 48 c7 85 30 04 00 00 02 00 00 00    	movq	$0x2, 0x430(%rbp)
1417a715f: 48 c7 85 40 04 00 00 02 00 00 00    	movq	$0x2, 0x440(%rbp)
1417a716a: 48 c7 85 50 04 00 00 00 00 00 00    	movq	$0x0, 0x450(%rbp)
1417a7175: 48 c7 85 58 04 00 00 20 00 00 00    	movq	$0x20, 0x458(%rbp)
1417a7180: c6 85 60 04 00 00 03        	movb	$0x3, 0x460(%rbp)
1417a7187: 48 c7 85 68 04 00 00 02 00 00 00    	movq	$0x2, 0x468(%rbp)
1417a7192: 48 c7 85 78 04 00 00 02 00 00 00    	movq	$0x2, 0x478(%rbp)
1417a719d: 48 c7 85 88 04 00 00 01 00 00 00    	movq	$0x1, 0x488(%rbp)
1417a71a8: 48 b8 20 00 00 00 04 00 00 00       	movabsq	$0x400000020, %rax # imm = 0x400000020
1417a71b2: 48 89 85 90 04 00 00        	movq	%rax, 0x490(%rbp)
1417a71b9: c6 85 98 04 00 00 03        	movb	$0x3, 0x498(%rbp)
1417a71c0: 48 8d 0d e1 54 53 01        	leaq	0x15354e1(%rip), %rcx   # 0x142cdc6a8
1417a71c7: e8 b4 6b ed fe              	callq	0x14067dd80 <.text+0x67cd80>
1417a71cc: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1417a71d0: 8b 40 10                    	movl	0x10(%rax), %eax
1417a71d3: 48 c7 85 10 05 00 00 02 00 00 00    	movq	$0x2, 0x510(%rbp)
1417a71de: 48 8d 15 db 54 53 01        	leaq	0x15354db(%rip), %rdx   # 0x142cdc6c0
1417a71e5: 48 89 95 18 05 00 00        	movq	%rdx, 0x518(%rbp)
1417a71ec: 48 c7 85 20 05 00 00 25 00 00 00    	movq	$0x25, 0x520(%rbp)
1417a71f7: 48 8d 0d 8a 54 53 01        	leaq	0x153548a(%rip), %rcx   # 0x142cdc688
1417a71fe: 48 89 8d 30 05 00 00        	movq	%rcx, 0x530(%rbp)
1417a7205: 48 c7 85 38 05 00 00 02 00 00 00    	movq	$0x2, 0x538(%rbp)
1417a7210: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
1417a7217: 48 89 8d 40 05 00 00        	movq	%rcx, 0x540(%rbp)
1417a721e: 48 c7 85 48 05 00 00 02 00 00 00    	movq	$0x2, 0x548(%rbp)
1417a7229: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a7230: 48 89 8d 50 05 00 00        	movq	%rcx, 0x550(%rbp)
1417a7237: 48 c7 85 58 05 00 00 02 00 00 00    	movq	$0x2, 0x558(%rbp)
1417a7242: 48 c7 85 e0 04 00 00 00 00 00 00    	movq	$0x0, 0x4e0(%rbp)
1417a724d: 48 89 95 e8 04 00 00        	movq	%rdx, 0x4e8(%rbp)
1417a7254: 48 c7 85 f0 04 00 00 25 00 00 00    	movq	$0x25, 0x4f0(%rbp)
1417a725f: 48 c7 85 f8 04 00 00 00 00 00 00    	movq	$0x0, 0x4f8(%rbp)
1417a726a: f3 0f 7f 85 00 05 00 00     	movdqu	%xmm0, 0x500(%rbp)
1417a7272: c7 85 28 05 00 00 01 00 00 00       	movl	$0x1, 0x528(%rbp)
1417a727c: 89 85 2c 05 00 00           	movl	%eax, 0x52c(%rbp)
1417a7282: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1417a7289: 48 89 f2                    	movq	%rsi, %rdx
1417a728c: e8 9f 30 5a ff              	callq	0x140d4a330 <.text+0xd49330>
1417a7291: 48 8b 95 d8 03 00 00        	movq	0x3d8(%rbp), %rdx
1417a7298: 48 85 d2                    	testq	%rdx, %rdx
1417a729b: 74 12                       	je	0x1417a72af <.text+0x17a62af>
1417a729d: 48 8b 8d e0 03 00 00        	movq	0x3e0(%rbp), %rcx
1417a72a4: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1417a72aa: e8 71 c9 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a72af: 4c 89 a5 e0 04 00 00        	movq	%r12, 0x4e0(%rbp)
1417a72b6: c6 85 76 05 00 00 01        	movb	$0x1, 0x576(%rbp)
1417a72bd: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
1417a72c4: 48 89 f2                    	movq	%rsi, %rdx
1417a72c7: e8 94 ee b4 ff              	callq	0x1412f6160 <.text+0x12f5160>
1417a72cc: c6 85 76 05 00 00 01        	movb	$0x1, 0x576(%rbp)
1417a72d3: 48 8d 8d e8 02 00 00        	leaq	0x2e8(%rbp), %rcx
1417a72da: e8 f1 fb 5c ff              	callq	0x140d76ed0 <.text+0xd75ed0>
1417a72df: c6 85 76 05 00 00 01        	movb	$0x1, 0x576(%rbp)
1417a72e6: 41 b9 06 00 00 00           	movl	$0x6, %r9d
1417a72ec: 48 89 c1                    	movq	%rax, %rcx
1417a72ef: 4c 8d 05 ef 53 53 01        	leaq	0x15353ef(%rip), %r8    # 0x142cdc6e5
1417a72f6: e8 85 b0 5b ff              	callq	0x140d62380 <.text+0xd61380>
1417a72fb: 84 c0                       	testb	%al, %al
1417a72fd: 74 41                       	je	0x1417a7340 <.text+0x17a6340>
1417a72ff: c6 85 77 05 00 00 01        	movb	$0x1, 0x577(%rbp)
1417a7306: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a730d: e8 1e 59 5a ff              	callq	0x140d4cc30 <.text+0xd4bc30>
1417a7312: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
1417a7319: 48 85 d2                    	testq	%rdx, %rdx
1417a731c: 0f 84 c8 fb ff ff           	je	0x1417a6eea <.text+0x17a5eea>
1417a7322: 48 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %rcx
1417a7329: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1417a732f: e8 ec c8 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a7334: e9 b1 fb ff ff              	jmp	0x1417a6eea <.text+0x17a5eea>
1417a7339: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1417a7340: f3 0f 6f 85 e8 02 00 00     	movdqu	0x2e8(%rbp), %xmm0
1417a7348: 0f 10 8d f8 02 00 00        	movups	0x2f8(%rbp), %xmm1
1417a734f: 0f 29 8d f0 04 00 00        	movaps	%xmm1, 0x4f0(%rbp)
1417a7356: 66 0f 7f 85 e0 04 00 00     	movdqa	%xmm0, 0x4e0(%rbp)
1417a735e: 48 89 f1                    	movq	%rsi, %rcx
1417a7361: e8 6a fb 5c ff              	callq	0x140d76ed0 <.text+0xd75ed0>
1417a7366: 48 89 c1                    	movq	%rax, %rcx
1417a7369: e8 22 22 5c ff              	callq	0x140d69590 <.text+0xd68590>
1417a736e: 49 89 c6                    	movq	%rax, %r14
1417a7371: 48 8b 95 e0 04 00 00        	movq	0x4e0(%rbp), %rdx
1417a7378: 48 85 d2                    	testq	%rdx, %rdx
1417a737b: 74 12                       	je	0x1417a738f <.text+0x17a638f>
1417a737d: 48 8b 8d e8 04 00 00        	movq	0x4e8(%rbp), %rcx
1417a7384: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1417a738a: e8 91 c8 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a738f: 44 89 f0                    	movl	%r14d, %eax
1417a7392: 83 e0 03                    	andl	$0x3, %eax
1417a7395: 83 f8 01                    	cmpl	$0x1, %eax
1417a7398: 0f 85 39 fb ff ff           	jne	0x1417a6ed7 <.text+0x17a5ed7>
1417a739e: 4c 89 f0                    	movq	%r14, %rax
1417a73a1: 48 ff c8                    	decq	%rax
1417a73a4: 48 89 85 d0 04 00 00        	movq	%rax, 0x4d0(%rbp)
1417a73ab: 49 8b 46 ff                 	movq	-0x1(%r14), %rax
1417a73af: 48 89 85 68 05 00 00        	movq	%rax, 0x568(%rbp)
1417a73b6: 49 8b 46 07                 	movq	0x7(%r14), %rax
1417a73ba: 48 89 85 c0 04 00 00        	movq	%rax, 0x4c0(%rbp)
1417a73c1: 48 8b 00                    	movq	(%rax), %rax
1417a73c4: 48 85 c0                    	testq	%rax, %rax
1417a73c7: 74 09                       	je	0x1417a73d2 <.text+0x17a63d2>
1417a73c9: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a73d0: ff d0                       	callq	*%rax
1417a73d2: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a73d9: 48 8b 85 c0 04 00 00        	movq	0x4c0(%rbp), %rax
1417a73e0: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1417a73e4: 48 85 d2                    	testq	%rdx, %rdx
1417a73e7: 0f 84 d3 fa ff ff           	je	0x1417a6ec0 <.text+0x17a5ec0>
1417a73ed: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1417a73f1: e8 2a c8 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a73f6: e9 c5 fa ff ff              	jmp	0x1417a6ec0 <.text+0x17a5ec0>
1417a73fb: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7402: 48 8b 41 18                 	movq	0x18(%rcx), %rax
1417a7406: 48 89 85 40 04 00 00        	movq	%rax, 0x440(%rbp)
1417a740d: 0f 10 41 08                 	movups	0x8(%rcx), %xmm0
1417a7411: 0f 29 85 30 04 00 00        	movaps	%xmm0, 0x430(%rbp)
1417a7418: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a741f: ff 15 cb af 9e 01           	callq	*0x19eafcb(%rip)        # 0x1431923f0
1417a7425: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
1417a742c: 48 8d 8d e8 01 00 00        	leaq	0x1e8(%rbp), %rcx
1417a7433: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
1417a7437: 66 0f 6f 85 30 04 00 00     	movdqa	0x430(%rbp), %xmm0
1417a743f: f3 0f 7f 01                 	movdqu	%xmm0, (%rcx)
1417a7443: 48 89 bd d8 01 00 00        	movq	%rdi, 0x1d8(%rbp)
1417a744a: 4c 89 b5 e0 01 00 00        	movq	%r14, 0x1e0(%rbp)
1417a7451: 48 8b 8d e8 01 00 00        	movq	0x1e8(%rbp), %rcx
1417a7458: 48 8b 95 f0 01 00 00        	movq	0x1f0(%rbp), %rdx
1417a745f: e8 1c 69 ed fe              	callq	0x14067dd80 <.text+0x67cd80>
1417a7464: 48 8b bd c8 04 00 00        	movq	0x4c8(%rbp), %rdi
1417a746b: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
1417a7472: 48 89 95 d0 01 00 00        	movq	%rdx, 0x1d0(%rbp)
1417a7479: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
1417a7480: 48 8d 95 c8 01 00 00        	leaq	0x1c8(%rbp), %rdx
1417a7487: e8 c4 98 8d 00              	callq	0x142080d50 <.text+0x207fd50>
1417a748c: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a7493: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
1417a749a: e8 d1 98 8d 00              	callq	0x142080d70 <.text+0x207fd70>
1417a749f: 48 8b 85 30 04 00 00        	movq	0x430(%rbp), %rax
1417a74a6: 48 89 85 68 05 00 00        	movq	%rax, 0x568(%rbp)
1417a74ad: 48 83 f8 01                 	cmpq	$0x1, %rax
1417a74b1: 0f 85 bd 00 00 00           	jne	0x1417a7574 <.text+0x17a6574>
1417a74b7: 48 8b 95 38 04 00 00        	movq	0x438(%rbp), %rdx
1417a74be: 4c 8b 85 40 04 00 00        	movq	0x440(%rbp), %r8
1417a74c5: 48 85 d2                    	testq	%rdx, %rdx
1417a74c8: 0f 84 db 00 00 00           	je	0x1417a75a9 <.text+0x17a65a9>
1417a74ce: 48 8d 8d d8 03 00 00        	leaq	0x3d8(%rbp), %rcx
1417a74d5: e8 66 be c5 ff              	callq	0x141403340 <.text+0x1402340>
1417a74da: 0f b6 85 d8 03 00 00        	movzbl	0x3d8(%rbp), %eax
1417a74e1: 3c 06                       	cmpb	$0x6, %al
1417a74e3: 0f 85 e3 00 00 00           	jne	0x1417a75cc <.text+0x17a65cc>
1417a74e9: 48 8b bd e0 03 00 00        	movq	0x3e0(%rbp), %rdi
1417a74f0: 48 89 bd 90 03 00 00        	movq	%rdi, 0x390(%rbp)
1417a74f7: c6 85 67 05 00 00 01        	movb	$0x1, 0x567(%rbp)
1417a74fe: 48 89 f1                    	movq	%rsi, %rcx
1417a7501: e8 fa f1 5a ff              	callq	0x140d56700 <.text+0xd55700>
1417a7506: f3 0f 6f 85 e0 04 00 00     	movdqu	0x4e0(%rbp), %xmm0
1417a750e: 0f 10 8d f0 04 00 00        	movups	0x4f0(%rbp), %xmm1
1417a7515: 0f 10 95 00 05 00 00        	movups	0x500(%rbp), %xmm2
1417a751c: 0f 29 95 50 04 00 00        	movaps	%xmm2, 0x450(%rbp)
1417a7523: 0f 29 8d 40 04 00 00        	movaps	%xmm1, 0x440(%rbp)
1417a752a: 66 0f 7f 85 30 04 00 00     	movdqa	%xmm0, 0x430(%rbp)
1417a7532: 48 8d 05 b2 51 53 01        	leaq	0x15351b2(%rip), %rax   # 0x142cdc6eb
1417a7539: 48 89 85 e0 04 00 00        	movq	%rax, 0x4e0(%rbp)
1417a7540: 48 c7 85 e8 04 00 00 1e 00 00 00    	movq	$0x1e, 0x4e8(%rbp)
1417a754b: 48 89 bd f0 04 00 00        	movq	%rdi, 0x4f0(%rbp)
1417a7552: c6 85 67 05 00 00 00        	movb	$0x0, 0x567(%rbp)
1417a7559: 48 89 f1                    	movq	%rsi, %rcx
1417a755c: 48 8d 95 30 04 00 00        	leaq	0x430(%rbp), %rdx
1417a7563: e8 98 1c 10 01              	callq	0x1428a9200 <.text+0x28a8200>
1417a7568: 49 89 c6                    	movq	%rax, %r14
1417a756b: 48 8b bd c8 04 00 00        	movq	0x4c8(%rbp), %rdi
1417a7572: eb 40                       	jmp	0x1417a75b4 <.text+0x17a65b4>
1417a7574: 48 8d 05 d5 50 53 01        	leaq	0x15350d5(%rip), %rax   # 0x142cdc650
1417a757b: 48 89 85 e0 04 00 00        	movq	%rax, 0x4e0(%rbp)
1417a7582: 48 c7 85 e8 04 00 00 01 00 00 00    	movq	$0x1, 0x4e8(%rbp)
1417a758d: 48 c7 85 f0 04 00 00 08 00 00 00    	movq	$0x8, 0x4f0(%rbp)
1417a7598: 0f 11 b5 f8 04 00 00        	movups	%xmm6, 0x4f8(%rbp)
1417a759f: 48 89 f1                    	movq	%rsi, %rcx
1417a75a2: e8 79 0a 10 01              	callq	0x1428a8020 <.text+0x28a7020>
1417a75a7: eb 08                       	jmp	0x1417a75b1 <.text+0x17a65b1>
1417a75a9: 4c 89 c1                    	movq	%r8, %rcx
1417a75ac: e8 df 1f 10 01              	callq	0x1428a9590 <.text+0x28a8590>
1417a75b1: 49 89 c6                    	movq	%rax, %r14
1417a75b4: c6 85 77 05 00 00 01        	movb	$0x1, 0x577(%rbp)
1417a75bb: 48 8d 8d d8 01 00 00        	leaq	0x1d8(%rbp), %rcx
1417a75c2: e8 a9 52 ee ff              	callq	0x14168c870 <.text+0x168b870>
1417a75c7: e9 03 fb ff ff              	jmp	0x1417a70cf <.text+0x17a60cf>
1417a75cc: 4c 8d 8d e8 03 00 00        	leaq	0x3e8(%rbp), %r9
1417a75d3: 41 8b 49 f1                 	movl	-0xf(%r9), %ecx
1417a75d7: 41 8b 51 f4                 	movl	-0xc(%r9), %edx
1417a75db: 4c 8d 85 a1 04 00 00        	leaq	0x4a1(%rbp), %r8
1417a75e2: 41 89 50 03                 	movl	%edx, 0x3(%r8)
1417a75e6: 41 89 08                    	movl	%ecx, (%r8)
1417a75e9: 48 8b 8d e0 03 00 00        	movq	0x3e0(%rbp), %rcx
1417a75f0: f3 41 0f 6f 01              	movdqu	(%r9), %xmm0
1417a75f5: f3 41 0f 7f 40 0f           	movdqu	%xmm0, 0xf(%r8)
1417a75fb: 88 85 a0 04 00 00           	movb	%al, 0x4a0(%rbp)
1417a7601: 48 89 8d a8 04 00 00        	movq	%rcx, 0x4a8(%rbp)
1417a7608: ba 07 00 00 00              	movl	$0x7, %edx
1417a760d: 48 8d 0d 44 4f 53 01        	leaq	0x1534f44(%rip), %rcx   # 0x142cdc558
1417a7614: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a761b: e8 b0 ce cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a7620: 48 85 c0                    	testq	%rax, %rax
1417a7623: 75 07                       	jne	0x1417a762c <.text+0x17a662c>
1417a7625: 48 8b 05 2c 91 e8 01        	movq	0x1e8912c(%rip), %rax   # 0x143630758
1417a762c: 0f b6 08                    	movzbl	(%rax), %ecx
1417a762f: 48 8d 15 4a 54 53 01        	leaq	0x153544a(%rip), %rdx   # 0x142cdca80
1417a7636: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
1417a763a: 48 01 d1                    	addq	%rdx, %rcx
1417a763d: ff e1                       	jmpq	*%rcx
1417a763f: c6 85 e0 04 00 00 00        	movb	$0x0, 0x4e0(%rbp)
1417a7646: e9 b1 00 00 00              	jmp	0x1417a76fc <.text+0x17a66fc>
1417a764b: 48 83 c0 08                 	addq	$0x8, %rax
1417a764f: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7656: 48 89 c2                    	movq	%rax, %rdx
1417a7659: 4c 8d 05 38 4d 53 01        	leaq	0x1534d38(%rip), %r8    # 0x142cdc398
1417a7660: e8 0b 0c b5 ff              	callq	0x1412f8270 <.text+0x12f7270>
1417a7665: c6 85 e0 04 00 00 04        	movb	$0x4, 0x4e0(%rbp)
1417a766c: e9 8b 00 00 00              	jmp	0x1417a76fc <.text+0x17a66fc>
1417a7671: f3 0f 6f 40 08              	movdqu	0x8(%rax), %xmm0
1417a7676: 48 8d 85 e8 04 00 00        	leaq	0x4e8(%rbp), %rax
1417a767d: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
1417a7681: c6 85 e0 04 00 00 02        	movb	$0x2, 0x4e0(%rbp)
1417a7688: eb 72                       	jmp	0x1417a76fc <.text+0x17a66fc>
1417a768a: 48 83 c0 08                 	addq	$0x8, %rax
1417a768e: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7695: 48 89 c2                    	movq	%rax, %rdx
1417a7698: e8 f3 46 5d ff              	callq	0x140d7bd90 <.text+0xd7ad90>
1417a769d: c6 85 e0 04 00 00 03        	movb	$0x3, 0x4e0(%rbp)
1417a76a4: eb 56                       	jmp	0x1417a76fc <.text+0x17a66fc>
1417a76a6: 0f b6 40 01                 	movzbl	0x1(%rax), %eax
1417a76aa: 88 85 e1 04 00 00           	movb	%al, 0x4e1(%rbp)
1417a76b0: c6 85 e0 04 00 00 01        	movb	$0x1, 0x4e0(%rbp)
1417a76b7: eb 43                       	jmp	0x1417a76fc <.text+0x17a66fc>
1417a76b9: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
1417a76be: 74 1f                       	je	0x1417a76df <.text+0x17a66df>
1417a76c0: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1417a76c4: 48 85 d2                    	testq	%rdx, %rdx
1417a76c7: 0f 84 fa 18 00 00           	je	0x1417a8fc7 <.text+0x17a7fc7>
1417a76cd: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1417a76d1: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a76d8: e8 e3 51 c9 ff              	callq	0x14143c8c0 <.text+0x143b8c0>
1417a76dd: eb 16                       	jmp	0x1417a76f5 <.text+0x17a66f5>
1417a76df: 48 c7 85 e8 04 00 00 00 00 00 00    	movq	$0x0, 0x4e8(%rbp)
1417a76ea: 48 c7 85 f8 04 00 00 00 00 00 00    	movq	$0x0, 0x4f8(%rbp)
1417a76f5: c6 85 e0 04 00 00 05        	movb	$0x5, 0x4e0(%rbp)
1417a76fc: 48 8d 8d d8 03 00 00        	leaq	0x3d8(%rbp), %rcx
1417a7703: 48 89 f2                    	movq	%rsi, %rdx
1417a7706: e8 e5 31 df ff              	callq	0x14159a8f0 <.text+0x15998f0>
1417a770b: 83 bd d8 03 00 00 01        	cmpl	$0x1, 0x3d8(%rbp)
1417a7712: 0f 85 9e 00 00 00           	jne	0x1417a77b6 <.text+0x17a67b6>
1417a7718: 48 8b bd e0 03 00 00        	movq	0x3e0(%rbp), %rdi
1417a771f: 48 89 bd 90 03 00 00        	movq	%rdi, 0x390(%rbp)
1417a7726: c6 85 66 05 00 00 01        	movb	$0x1, 0x566(%rbp)
1417a772d: 48 89 f1                    	movq	%rsi, %rcx
1417a7730: e8 cb ef 5a ff              	callq	0x140d56700 <.text+0xd55700>
1417a7735: f3 0f 6f 85 e0 04 00 00     	movdqu	0x4e0(%rbp), %xmm0
1417a773d: 0f 10 8d f0 04 00 00        	movups	0x4f0(%rbp), %xmm1
1417a7744: 0f 10 95 00 05 00 00        	movups	0x500(%rbp), %xmm2
1417a774b: 0f 29 95 50 04 00 00        	movaps	%xmm2, 0x450(%rbp)
1417a7752: 0f 29 8d 40 04 00 00        	movaps	%xmm1, 0x440(%rbp)
1417a7759: 66 0f 7f 85 30 04 00 00     	movdqa	%xmm0, 0x430(%rbp)
1417a7761: 48 8d 05 a1 4f 53 01        	leaq	0x1534fa1(%rip), %rax   # 0x142cdc709
1417a7768: 48 89 85 e0 04 00 00        	movq	%rax, 0x4e0(%rbp)
1417a776f: 48 c7 85 e8 04 00 00 0f 00 00 00    	movq	$0xf, 0x4e8(%rbp)
1417a777a: 48 89 bd f0 04 00 00        	movq	%rdi, 0x4f0(%rbp)
1417a7781: c6 85 66 05 00 00 00        	movb	$0x0, 0x566(%rbp)
1417a7788: 48 89 f1                    	movq	%rsi, %rcx
1417a778b: 48 8d 95 30 04 00 00        	leaq	0x430(%rbp), %rdx
1417a7792: e8 69 1a 10 01              	callq	0x1428a9200 <.text+0x28a8200>
1417a7797: 49 89 c6                    	movq	%rax, %r14
1417a779a: 48 8b bd c8 04 00 00        	movq	0x4c8(%rbp), %rdi
1417a77a1: 0f b6 85 a0 04 00 00        	movzbl	0x4a0(%rbp), %eax
1417a77a8: 83 f8 03                    	cmpl	$0x3, %eax
1417a77ab: 0f 83 ef 00 00 00           	jae	0x1417a78a0 <.text+0x17a68a0>
1417a77b1: e9 fe fd ff ff              	jmp	0x1417a75b4 <.text+0x17a65b4>
1417a77b6: 8b 85 dc 03 00 00           	movl	0x3dc(%rbp), %eax
1417a77bc: 89 85 64 03 00 00           	movl	%eax, 0x364(%rbp)
1417a77c2: 83 f8 02                    	cmpl	$0x2, %eax
1417a77c5: 75 43                       	jne	0x1417a780a <.text+0x17a680a>
1417a77c7: ba 05 00 00 00              	movl	$0x5, %edx
1417a77cc: 48 8d 0d a9 4d 53 01        	leaq	0x1534da9(%rip), %rcx   # 0x142cdc57c
1417a77d3: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a77da: e8 f1 cc cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a77df: 48 85 c0                    	testq	%rax, %rax
1417a77e2: 75 07                       	jne	0x1417a77eb <.text+0x17a67eb>
1417a77e4: 48 8b 05 6d 8f e8 01        	movq	0x1e88f6d(%rip), %rax   # 0x143630758
1417a77eb: 0f b6 08                    	movzbl	(%rax), %ecx
1417a77ee: 48 8d 15 a3 52 53 01        	leaq	0x15352a3(%rip), %rdx   # 0x142cdca98
1417a77f5: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
1417a77f9: 48 01 d1                    	addq	%rdx, %rcx
1417a77fc: ff e1                       	jmpq	*%rcx
1417a77fe: c6 85 e0 04 00 00 00        	movb	$0x0, 0x4e0(%rbp)
1417a7805: e9 c3 01 00 00              	jmp	0x1417a79cd <.text+0x17a69cd>
1417a780a: 48 8d 05 67 4d 53 01        	leaq	0x1534d67(%rip), %rax   # 0x142cdc578
1417a7811: 48 89 85 30 04 00 00        	movq	%rax, 0x430(%rbp)
1417a7818: 48 8d 05 61 5d 5e ff        	leaq	-0xa1a29f(%rip), %rax   # 0x140d8d580 <.text+0xd8c580>
1417a781f: 48 89 85 38 04 00 00        	movq	%rax, 0x438(%rbp)
1417a7826: 48 8d 8d 64 03 00 00        	leaq	0x364(%rbp), %rcx
1417a782d: 48 89 8d 40 04 00 00        	movq	%rcx, 0x440(%rbp)
1417a7834: 48 89 85 48 04 00 00        	movq	%rax, 0x448(%rbp)
1417a783b: 48 8d 05 fe 4f 53 01        	leaq	0x1534ffe(%rip), %rax   # 0x142cdc840
1417a7842: 48 89 85 e0 04 00 00        	movq	%rax, 0x4e0(%rbp)
1417a7849: 48 c7 85 e8 04 00 00 02 00 00 00    	movq	$0x2, 0x4e8(%rbp)
1417a7854: 48 8d 85 30 04 00 00        	leaq	0x430(%rbp), %rax
1417a785b: 48 89 85 f0 04 00 00        	movq	%rax, 0x4f0(%rbp)
1417a7862: 48 c7 85 f8 04 00 00 02 00 00 00    	movq	$0x2, 0x4f8(%rbp)
1417a786d: 48 c7 85 00 05 00 00 00 00 00 00    	movq	$0x0, 0x500(%rbp)
1417a7878: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
1417a787c: 48 89 f2                    	movq	%rsi, %rdx
1417a787f: e8 0c 27 5d ff              	callq	0x140d79f90 <.text+0xd78f90>
1417a7884: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
1417a7888: e8 53 0b 10 01              	callq	0x1428a83e0 <.text+0x28a73e0>
1417a788d: 49 89 c6                    	movq	%rax, %r14
1417a7890: 0f b6 85 a0 04 00 00        	movzbl	0x4a0(%rbp), %eax
1417a7897: 83 f8 03                    	cmpl	$0x3, %eax
1417a789a: 0f 82 14 fd ff ff           	jb	0x1417a75b4 <.text+0x17a65b4>
1417a78a0: 74 42                       	je	0x1417a78e4 <.text+0x17a68e4>
1417a78a2: 83 f8 04                    	cmpl	$0x4, %eax
1417a78a5: 75 64                       	jne	0x1417a790b <.text+0x17a690b>
1417a78a7: 48 8b 8d b0 04 00 00        	movq	0x4b0(%rbp), %rcx
1417a78ae: 48 8b 95 b8 04 00 00        	movq	0x4b8(%rbp), %rdx
1417a78b5: 48 89 8d 68 05 00 00        	movq	%rcx, 0x568(%rbp)
1417a78bc: e8 af 14 0e ff              	callq	0x140888d70 <.text+0x887d70>
1417a78c1: 48 8b 95 a8 04 00 00        	movq	0x4a8(%rbp), %rdx
1417a78c8: 48 85 d2                    	testq	%rdx, %rdx
1417a78cb: 0f 84 e3 fc ff ff           	je	0x1417a75b4 <.text+0x17a65b4>
1417a78d1: 48 c1 e2 05                 	shlq	$0x5, %rdx
1417a78d5: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a78db: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a78e2: eb 1d                       	jmp	0x1417a7901 <.text+0x17a6901>
1417a78e4: 48 8b 95 a8 04 00 00        	movq	0x4a8(%rbp), %rdx
1417a78eb: 48 85 d2                    	testq	%rdx, %rdx
1417a78ee: 0f 84 c0 fc ff ff           	je	0x1417a75b4 <.text+0x17a65b4>
1417a78f4: 48 8b 8d b0 04 00 00        	movq	0x4b0(%rbp), %rcx
1417a78fb: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1417a7901: e8 1a c3 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a7906: e9 a9 fc ff ff              	jmp	0x1417a75b4 <.text+0x17a65b4>
1417a790b: 48 8d 8d a8 04 00 00        	leaq	0x4a8(%rbp), %rcx
1417a7912: e8 29 16 0f ff              	callq	0x140898f40 <.text+0x897f40>
1417a7917: e9 98 fc ff ff              	jmp	0x1417a75b4 <.text+0x17a65b4>
1417a791c: 48 83 c0 08                 	addq	$0x8, %rax
1417a7920: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7927: 48 89 c2                    	movq	%rax, %rdx
1417a792a: 4c 8d 05 67 4a 53 01        	leaq	0x1534a67(%rip), %r8    # 0x142cdc398
1417a7931: e8 3a 09 b5 ff              	callq	0x1412f8270 <.text+0x12f7270>
1417a7936: c6 85 e0 04 00 00 04        	movb	$0x4, 0x4e0(%rbp)
1417a793d: e9 8b 00 00 00              	jmp	0x1417a79cd <.text+0x17a69cd>
1417a7942: f3 0f 6f 40 08              	movdqu	0x8(%rax), %xmm0
1417a7947: 48 8d 85 e8 04 00 00        	leaq	0x4e8(%rbp), %rax
1417a794e: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
1417a7952: c6 85 e0 04 00 00 02        	movb	$0x2, 0x4e0(%rbp)
1417a7959: eb 72                       	jmp	0x1417a79cd <.text+0x17a69cd>
1417a795b: 48 83 c0 08                 	addq	$0x8, %rax
1417a795f: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7966: 48 89 c2                    	movq	%rax, %rdx
1417a7969: e8 22 44 5d ff              	callq	0x140d7bd90 <.text+0xd7ad90>
1417a796e: c6 85 e0 04 00 00 03        	movb	$0x3, 0x4e0(%rbp)
1417a7975: eb 56                       	jmp	0x1417a79cd <.text+0x17a69cd>
1417a7977: 0f b6 40 01                 	movzbl	0x1(%rax), %eax
1417a797b: 88 85 e1 04 00 00           	movb	%al, 0x4e1(%rbp)
1417a7981: c6 85 e0 04 00 00 01        	movb	$0x1, 0x4e0(%rbp)
1417a7988: eb 43                       	jmp	0x1417a79cd <.text+0x17a69cd>
1417a798a: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
1417a798f: 74 1f                       	je	0x1417a79b0 <.text+0x17a69b0>
1417a7991: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1417a7995: 48 85 d2                    	testq	%rdx, %rdx
1417a7998: 0f 84 37 16 00 00           	je	0x1417a8fd5 <.text+0x17a7fd5>
1417a799e: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1417a79a2: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a79a9: e8 12 4f c9 ff              	callq	0x14143c8c0 <.text+0x143b8c0>
1417a79ae: eb 16                       	jmp	0x1417a79c6 <.text+0x17a69c6>
1417a79b0: 48 c7 85 e8 04 00 00 00 00 00 00    	movq	$0x0, 0x4e8(%rbp)
1417a79bb: 48 c7 85 f8 04 00 00 00 00 00 00    	movq	$0x0, 0x4f8(%rbp)
1417a79c6: c6 85 e0 04 00 00 05        	movb	$0x5, 0x4e0(%rbp)
1417a79cd: 48 8d 8d d8 03 00 00        	leaq	0x3d8(%rbp), %rcx
1417a79d4: 48 89 f2                    	movq	%rsi, %rdx
1417a79d7: e8 74 3e df ff              	callq	0x14159b850 <.text+0x159a850>
1417a79dc: 48 8b 85 d8 03 00 00        	movq	0x3d8(%rbp), %rax
1417a79e3: 48 8b bd e0 03 00 00        	movq	0x3e0(%rbp), %rdi
1417a79ea: 48 89 c1                    	movq	%rax, %rcx
1417a79ed: 48 f7 d9                    	negq	%rcx
1417a79f0: 71 7d                       	jno	0x1417a7a6f <.text+0x17a6a6f>
1417a79f2: 48 89 bd 90 03 00 00        	movq	%rdi, 0x390(%rbp)
1417a79f9: c6 85 65 05 00 00 01        	movb	$0x1, 0x565(%rbp)
1417a7a00: 48 89 f1                    	movq	%rsi, %rcx
1417a7a03: e8 f8 ec 5a ff              	callq	0x140d56700 <.text+0xd55700>
1417a7a08: f3 0f 6f 85 e0 04 00 00     	movdqu	0x4e0(%rbp), %xmm0
1417a7a10: 0f 10 8d f0 04 00 00        	movups	0x4f0(%rbp), %xmm1
1417a7a17: 0f 10 95 00 05 00 00        	movups	0x500(%rbp), %xmm2
1417a7a1e: 0f 29 95 50 04 00 00        	movaps	%xmm2, 0x450(%rbp)
1417a7a25: 0f 29 8d 40 04 00 00        	movaps	%xmm1, 0x440(%rbp)
1417a7a2c: 66 0f 7f 85 30 04 00 00     	movdqa	%xmm0, 0x430(%rbp)
1417a7a34: 48 8d 05 dd 4c 53 01        	leaq	0x1534cdd(%rip), %rax   # 0x142cdc718
1417a7a3b: 48 89 85 e0 04 00 00        	movq	%rax, 0x4e0(%rbp)
1417a7a42: 48 c7 85 e8 04 00 00 0d 00 00 00    	movq	$0xd, 0x4e8(%rbp)
1417a7a4d: 48 89 bd f0 04 00 00        	movq	%rdi, 0x4f0(%rbp)
1417a7a54: c6 85 65 05 00 00 00        	movb	$0x0, 0x565(%rbp)
1417a7a5b: 48 89 f1                    	movq	%rsi, %rcx
1417a7a5e: 48 8d 95 30 04 00 00        	leaq	0x430(%rbp), %rdx
1417a7a65: e8 96 17 10 01              	callq	0x1428a9200 <.text+0x28a8200>
1417a7a6a: e9 28 fd ff ff              	jmp	0x1417a7797 <.text+0x17a6797>
1417a7a6f: 48 8b 8d e8 03 00 00        	movq	0x3e8(%rbp), %rcx
1417a7a76: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
1417a7a7d: 48 89 bd d8 02 00 00        	movq	%rdi, 0x2d8(%rbp)
1417a7a84: 48 89 8d e0 02 00 00        	movq	%rcx, 0x2e0(%rbp)
1417a7a8b: ba 05 00 00 00              	movl	$0x5, %edx
1417a7a90: 48 8d 0d ea 4a 53 01        	leaq	0x1534aea(%rip), %rcx   # 0x142cdc581
1417a7a97: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a7a9e: e8 2d ca cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a7aa3: 48 8b bd d8 04 00 00        	movq	0x4d8(%rbp), %rdi
1417a7aaa: 48 85 c0                    	testq	%rax, %rax
1417a7aad: 75 07                       	jne	0x1417a7ab6 <.text+0x17a6ab6>
1417a7aaf: 48 8b 05 a2 8c e8 01        	movq	0x1e88ca2(%rip), %rax   # 0x143630758
1417a7ab6: 0f b6 08                    	movzbl	(%rax), %ecx
1417a7ab9: 48 8d 15 f0 4f 53 01        	leaq	0x1534ff0(%rip), %rdx   # 0x142cdcab0
1417a7ac0: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
1417a7ac4: 48 01 d1                    	addq	%rdx, %rcx
1417a7ac7: ff e1                       	jmpq	*%rcx
1417a7ac9: c6 85 e0 04 00 00 00        	movb	$0x0, 0x4e0(%rbp)
1417a7ad0: e9 85 00 00 00              	jmp	0x1417a7b5a <.text+0x17a6b5a>
1417a7ad5: 48 83 c0 08                 	addq	$0x8, %rax
1417a7ad9: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7ae0: 48 89 c2                    	movq	%rax, %rdx
1417a7ae3: 4c 8d 05 ae 48 53 01        	leaq	0x15348ae(%rip), %r8    # 0x142cdc398
1417a7aea: e8 81 07 b5 ff              	callq	0x1412f8270 <.text+0x12f7270>
1417a7aef: c6 85 e0 04 00 00 04        	movb	$0x4, 0x4e0(%rbp)
1417a7af6: eb 62                       	jmp	0x1417a7b5a <.text+0x17a6b5a>
1417a7af8: f3 0f 6f 40 08              	movdqu	0x8(%rax), %xmm0
1417a7afd: 48 8d 85 e8 04 00 00        	leaq	0x4e8(%rbp), %rax
1417a7b04: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
1417a7b08: c6 85 e0 04 00 00 02        	movb	$0x2, 0x4e0(%rbp)
1417a7b0f: eb 49                       	jmp	0x1417a7b5a <.text+0x17a6b5a>
1417a7b11: 48 83 c0 08                 	addq	$0x8, %rax
1417a7b15: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7b1c: 48 89 c2                    	movq	%rax, %rdx
1417a7b1f: e8 6c 42 5d ff              	callq	0x140d7bd90 <.text+0xd7ad90>
1417a7b24: c6 85 e0 04 00 00 03        	movb	$0x3, 0x4e0(%rbp)
1417a7b2b: eb 2d                       	jmp	0x1417a7b5a <.text+0x17a6b5a>
1417a7b2d: 0f b6 40 01                 	movzbl	0x1(%rax), %eax
1417a7b31: 88 85 e1 04 00 00           	movb	%al, 0x4e1(%rbp)
1417a7b37: c6 85 e0 04 00 00 01        	movb	$0x1, 0x4e0(%rbp)
1417a7b3e: eb 1a                       	jmp	0x1417a7b5a <.text+0x17a6b5a>
1417a7b40: 48 83 c0 08                 	addq	$0x8, %rax
1417a7b44: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7b4b: 48 89 c2                    	movq	%rax, %rdx
1417a7b4e: e8 6d 56 c9 ff              	callq	0x14143d1c0 <.text+0x143c1c0>
1417a7b53: c6 85 e0 04 00 00 05        	movb	$0x5, 0x4e0(%rbp)
1417a7b5a: 48 8d 8d d8 03 00 00        	leaq	0x3d8(%rbp), %rcx
1417a7b61: 48 89 f2                    	movq	%rsi, %rdx
1417a7b64: e8 17 35 df ff              	callq	0x14159b080 <.text+0x159a080>
1417a7b69: 48 8b 8d d8 03 00 00        	movq	0x3d8(%rbp), %rcx
1417a7b70: 4c 8b b5 e0 03 00 00        	movq	0x3e0(%rbp), %r14
1417a7b77: 48 89 c8                    	movq	%rcx, %rax
1417a7b7a: 48 f7 d8                    	negq	%rax
1417a7b7d: 0f 81 80 00 00 00           	jno	0x1417a7c03 <.text+0x17a6c03>
1417a7b83: 4c 89 b5 90 03 00 00        	movq	%r14, 0x390(%rbp)
1417a7b8a: c6 85 64 05 00 00 01        	movb	$0x1, 0x564(%rbp)
1417a7b91: 48 89 f1                    	movq	%rsi, %rcx
1417a7b94: e8 67 eb 5a ff              	callq	0x140d56700 <.text+0xd55700>
1417a7b99: f3 0f 6f 85 e0 04 00 00     	movdqu	0x4e0(%rbp), %xmm0
1417a7ba1: 0f 10 8d f0 04 00 00        	movups	0x4f0(%rbp), %xmm1
1417a7ba8: 0f 10 95 00 05 00 00        	movups	0x500(%rbp), %xmm2
1417a7baf: 0f 29 95 50 04 00 00        	movaps	%xmm2, 0x450(%rbp)
1417a7bb6: 0f 29 8d 40 04 00 00        	movaps	%xmm1, 0x440(%rbp)
1417a7bbd: 66 0f 7f 85 30 04 00 00     	movdqa	%xmm0, 0x430(%rbp)
1417a7bc5: 48 8d 05 59 4b 53 01        	leaq	0x1534b59(%rip), %rax   # 0x142cdc725
1417a7bcc: 48 89 85 e0 04 00 00        	movq	%rax, 0x4e0(%rbp)
1417a7bd3: 48 c7 85 e8 04 00 00 0d 00 00 00    	movq	$0xd, 0x4e8(%rbp)
1417a7bde: 4c 89 b5 f0 04 00 00        	movq	%r14, 0x4f0(%rbp)
1417a7be5: c6 85 64 05 00 00 00        	movb	$0x0, 0x564(%rbp)
1417a7bec: 48 89 f1                    	movq	%rsi, %rcx
1417a7bef: 48 8d 95 30 04 00 00        	leaq	0x430(%rbp), %rdx
1417a7bf6: e8 05 16 10 01              	callq	0x1428a9200 <.text+0x28a8200>
1417a7bfb: 49 89 c6                    	movq	%rax, %r14
1417a7bfe: e9 ea 0e 00 00              	jmp	0x1417a8aed <.text+0x17a7aed>
1417a7c03: 48 89 8d c0 03 00 00        	movq	%rcx, 0x3c0(%rbp)
1417a7c0a: 4c 89 b5 68 03 00 00        	movq	%r14, 0x368(%rbp)
1417a7c11: 4c 8b b5 e8 03 00 00        	movq	0x3e8(%rbp), %r14
1417a7c18: ba 0a 00 00 00              	movl	$0xa, %edx
1417a7c1d: 48 8d 0d 62 49 53 01        	leaq	0x1534962(%rip), %rcx   # 0x142cdc586
1417a7c24: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a7c2b: e8 a0 c8 cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a7c30: 48 85 c0                    	testq	%rax, %rax
1417a7c33: 75 07                       	jne	0x1417a7c3c <.text+0x17a6c3c>
1417a7c35: 48 8b 05 1c 8b e8 01        	movq	0x1e88b1c(%rip), %rax   # 0x143630758
1417a7c3c: 0f b6 08                    	movzbl	(%rax), %ecx
1417a7c3f: 48 8d 15 82 4e 53 01        	leaq	0x1534e82(%rip), %rdx   # 0x142cdcac8
1417a7c46: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
1417a7c4a: 48 01 d1                    	addq	%rdx, %rcx
1417a7c4d: ff e1                       	jmpq	*%rcx
1417a7c4f: c6 85 e0 04 00 00 00        	movb	$0x0, 0x4e0(%rbp)
1417a7c56: e9 85 00 00 00              	jmp	0x1417a7ce0 <.text+0x17a6ce0>
1417a7c5b: 48 83 c0 08                 	addq	$0x8, %rax
1417a7c5f: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7c66: 48 89 c2                    	movq	%rax, %rdx
1417a7c69: 4c 8d 05 28 47 53 01        	leaq	0x1534728(%rip), %r8    # 0x142cdc398
1417a7c70: e8 fb 05 b5 ff              	callq	0x1412f8270 <.text+0x12f7270>
1417a7c75: c6 85 e0 04 00 00 04        	movb	$0x4, 0x4e0(%rbp)
1417a7c7c: eb 62                       	jmp	0x1417a7ce0 <.text+0x17a6ce0>
1417a7c7e: f3 0f 6f 40 08              	movdqu	0x8(%rax), %xmm0
1417a7c83: 48 8d 85 e8 04 00 00        	leaq	0x4e8(%rbp), %rax
1417a7c8a: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
1417a7c8e: c6 85 e0 04 00 00 02        	movb	$0x2, 0x4e0(%rbp)
1417a7c95: eb 49                       	jmp	0x1417a7ce0 <.text+0x17a6ce0>
1417a7c97: 48 83 c0 08                 	addq	$0x8, %rax
1417a7c9b: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7ca2: 48 89 c2                    	movq	%rax, %rdx
1417a7ca5: e8 e6 40 5d ff              	callq	0x140d7bd90 <.text+0xd7ad90>
1417a7caa: c6 85 e0 04 00 00 03        	movb	$0x3, 0x4e0(%rbp)
1417a7cb1: eb 2d                       	jmp	0x1417a7ce0 <.text+0x17a6ce0>
1417a7cb3: 0f b6 40 01                 	movzbl	0x1(%rax), %eax
1417a7cb7: 88 85 e1 04 00 00           	movb	%al, 0x4e1(%rbp)
1417a7cbd: c6 85 e0 04 00 00 01        	movb	$0x1, 0x4e0(%rbp)
1417a7cc4: eb 1a                       	jmp	0x1417a7ce0 <.text+0x17a6ce0>
1417a7cc6: 48 83 c0 08                 	addq	$0x8, %rax
1417a7cca: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7cd1: 48 89 c2                    	movq	%rax, %rdx
1417a7cd4: e8 e7 54 c9 ff              	callq	0x14143d1c0 <.text+0x143c1c0>
1417a7cd9: c6 85 e0 04 00 00 05        	movb	$0x5, 0x4e0(%rbp)
1417a7ce0: 48 8d 05 a1 6a 4c 01        	leaq	0x14c6aa1(%rip), %rax   # 0x142c6e788
1417a7ce7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1417a7cec: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1417a7cf5: 41 b9 18 00 00 00           	movl	$0x18, %r9d
1417a7cfb: 48 8d 8d d8 03 00 00        	leaq	0x3d8(%rbp), %rcx
1417a7d02: 48 89 f2                    	movq	%rsi, %rdx
1417a7d05: 4c 8d 05 5c 6a 4c 01        	leaq	0x14c6a5c(%rip), %r8    # 0x142c6e768
1417a7d0c: e8 2f b4 df ff              	callq	0x1415a3140 <.text+0x15a2140>
1417a7d11: 48 89 bd d8 04 00 00        	movq	%rdi, 0x4d8(%rbp)
1417a7d18: 48 8b 85 d8 03 00 00        	movq	0x3d8(%rbp), %rax
1417a7d1f: 48 8b bd e0 03 00 00        	movq	0x3e0(%rbp), %rdi
1417a7d26: 48 89 c1                    	movq	%rax, %rcx
1417a7d29: 48 f7 d9                    	negq	%rcx
1417a7d2c: 0f 81 80 00 00 00           	jno	0x1417a7db2 <.text+0x17a6db2>
1417a7d32: 48 89 bd 90 03 00 00        	movq	%rdi, 0x390(%rbp)
1417a7d39: c6 85 63 05 00 00 01        	movb	$0x1, 0x563(%rbp)
1417a7d40: 48 89 f1                    	movq	%rsi, %rcx
1417a7d43: e8 b8 e9 5a ff              	callq	0x140d56700 <.text+0xd55700>
1417a7d48: f3 0f 6f 85 e0 04 00 00     	movdqu	0x4e0(%rbp), %xmm0
1417a7d50: 0f 10 8d f0 04 00 00        	movups	0x4f0(%rbp), %xmm1
1417a7d57: 0f 10 95 00 05 00 00        	movups	0x500(%rbp), %xmm2
1417a7d5e: 0f 29 95 50 04 00 00        	movaps	%xmm2, 0x450(%rbp)
1417a7d65: 0f 29 8d 40 04 00 00        	movaps	%xmm1, 0x440(%rbp)
1417a7d6c: 66 0f 7f 85 30 04 00 00     	movdqa	%xmm0, 0x430(%rbp)
1417a7d74: 48 8d 05 b7 49 53 01        	leaq	0x15349b7(%rip), %rax   # 0x142cdc732
1417a7d7b: 48 89 85 e0 04 00 00        	movq	%rax, 0x4e0(%rbp)
1417a7d82: 48 c7 85 e8 04 00 00 12 00 00 00    	movq	$0x12, 0x4e8(%rbp)
1417a7d8d: 48 89 bd f0 04 00 00        	movq	%rdi, 0x4f0(%rbp)
1417a7d94: c6 85 63 05 00 00 00        	movb	$0x0, 0x563(%rbp)
1417a7d9b: 48 89 f1                    	movq	%rsi, %rcx
1417a7d9e: 48 8d 95 30 04 00 00        	leaq	0x430(%rbp), %rdx
1417a7da5: e8 56 14 10 01              	callq	0x1428a9200 <.text+0x28a8200>
1417a7daa: 49 89 c6                    	movq	%rax, %r14
1417a7dad: e9 0b 0d 00 00              	jmp	0x1417a8abd <.text+0x17a7abd>
1417a7db2: 48 8b 8d e8 03 00 00        	movq	0x3e8(%rbp), %rcx
1417a7db9: 48 89 85 b8 02 00 00        	movq	%rax, 0x2b8(%rbp)
1417a7dc0: 48 89 bd 68 05 00 00        	movq	%rdi, 0x568(%rbp)
1417a7dc7: 48 89 bd c0 02 00 00        	movq	%rdi, 0x2c0(%rbp)
1417a7dce: 48 89 8d c0 04 00 00        	movq	%rcx, 0x4c0(%rbp)
1417a7dd5: 48 89 8d c8 02 00 00        	movq	%rcx, 0x2c8(%rbp)
1417a7ddc: ba 0a 00 00 00              	movl	$0xa, %edx
1417a7de1: 48 8d 0d ae 47 53 01        	leaq	0x15347ae(%rip), %rcx   # 0x142cdc596
1417a7de8: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a7def: e8 dc c6 cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a7df4: 48 8b bd d8 04 00 00        	movq	0x4d8(%rbp), %rdi
1417a7dfb: 48 85 c0                    	testq	%rax, %rax
1417a7dfe: 75 07                       	jne	0x1417a7e07 <.text+0x17a6e07>
1417a7e00: 48 8b 05 51 89 e8 01        	movq	0x1e88951(%rip), %rax   # 0x143630758
1417a7e07: 0f b6 08                    	movzbl	(%rax), %ecx
1417a7e0a: 48 8d 15 cf 4c 53 01        	leaq	0x1534ccf(%rip), %rdx   # 0x142cdcae0
1417a7e11: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
1417a7e15: 48 01 d1                    	addq	%rdx, %rcx
1417a7e18: ff e1                       	jmpq	*%rcx
1417a7e1a: c6 85 e0 04 00 00 00        	movb	$0x0, 0x4e0(%rbp)
1417a7e21: e9 85 00 00 00              	jmp	0x1417a7eab <.text+0x17a6eab>
1417a7e26: 48 83 c0 08                 	addq	$0x8, %rax
1417a7e2a: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7e31: 48 89 c2                    	movq	%rax, %rdx
1417a7e34: 4c 8d 05 5d 45 53 01        	leaq	0x153455d(%rip), %r8    # 0x142cdc398
1417a7e3b: e8 30 04 b5 ff              	callq	0x1412f8270 <.text+0x12f7270>
1417a7e40: c6 85 e0 04 00 00 04        	movb	$0x4, 0x4e0(%rbp)
1417a7e47: eb 62                       	jmp	0x1417a7eab <.text+0x17a6eab>
1417a7e49: f3 0f 6f 40 08              	movdqu	0x8(%rax), %xmm0
1417a7e4e: 48 8d 85 e8 04 00 00        	leaq	0x4e8(%rbp), %rax
1417a7e55: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
1417a7e59: c6 85 e0 04 00 00 02        	movb	$0x2, 0x4e0(%rbp)
1417a7e60: eb 49                       	jmp	0x1417a7eab <.text+0x17a6eab>
1417a7e62: 48 83 c0 08                 	addq	$0x8, %rax
1417a7e66: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7e6d: 48 89 c2                    	movq	%rax, %rdx
1417a7e70: e8 1b 3f 5d ff              	callq	0x140d7bd90 <.text+0xd7ad90>
1417a7e75: c6 85 e0 04 00 00 03        	movb	$0x3, 0x4e0(%rbp)
1417a7e7c: eb 2d                       	jmp	0x1417a7eab <.text+0x17a6eab>
1417a7e7e: 0f b6 40 01                 	movzbl	0x1(%rax), %eax
1417a7e82: 88 85 e1 04 00 00           	movb	%al, 0x4e1(%rbp)
1417a7e88: c6 85 e0 04 00 00 01        	movb	$0x1, 0x4e0(%rbp)
1417a7e8f: eb 1a                       	jmp	0x1417a7eab <.text+0x17a6eab>
1417a7e91: 48 83 c0 08                 	addq	$0x8, %rax
1417a7e95: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7e9c: 48 89 c2                    	movq	%rax, %rdx
1417a7e9f: e8 1c 53 c9 ff              	callq	0x14143d1c0 <.text+0x143c1c0>
1417a7ea4: c6 85 e0 04 00 00 05        	movb	$0x5, 0x4e0(%rbp)
1417a7eab: 48 8d 8d d8 03 00 00        	leaq	0x3d8(%rbp), %rcx
1417a7eb2: 48 89 f2                    	movq	%rsi, %rdx
1417a7eb5: e8 16 49 df ff              	callq	0x14159c7d0 <.text+0x159b7d0>
1417a7eba: 4c 89 b5 50 02 00 00        	movq	%r14, 0x250(%rbp)
1417a7ec1: 48 8b 8d d8 03 00 00        	movq	0x3d8(%rbp), %rcx
1417a7ec8: 4c 8b b5 e0 03 00 00        	movq	0x3e0(%rbp), %r14
1417a7ecf: 48 89 c8                    	movq	%rcx, %rax
1417a7ed2: 48 f7 d8                    	negq	%rax
1417a7ed5: 0f 81 80 00 00 00           	jno	0x1417a7f5b <.text+0x17a6f5b>
1417a7edb: 4c 89 b5 90 03 00 00        	movq	%r14, 0x390(%rbp)
1417a7ee2: c6 85 62 05 00 00 01        	movb	$0x1, 0x562(%rbp)
1417a7ee9: 48 89 f1                    	movq	%rsi, %rcx
1417a7eec: e8 0f e8 5a ff              	callq	0x140d56700 <.text+0xd55700>
1417a7ef1: f3 0f 6f 85 e0 04 00 00     	movdqu	0x4e0(%rbp), %xmm0
1417a7ef9: 0f 10 8d f0 04 00 00        	movups	0x4f0(%rbp), %xmm1
1417a7f00: 0f 10 95 00 05 00 00        	movups	0x500(%rbp), %xmm2
1417a7f07: 0f 29 95 50 04 00 00        	movaps	%xmm2, 0x450(%rbp)
1417a7f0e: 0f 29 8d 40 04 00 00        	movaps	%xmm1, 0x440(%rbp)
1417a7f15: 66 0f 7f 85 30 04 00 00     	movdqa	%xmm0, 0x430(%rbp)
1417a7f1d: 48 8d 05 20 48 53 01        	leaq	0x1534820(%rip), %rax   # 0x142cdc744
1417a7f24: 48 89 85 e0 04 00 00        	movq	%rax, 0x4e0(%rbp)
1417a7f2b: 48 c7 85 e8 04 00 00 12 00 00 00    	movq	$0x12, 0x4e8(%rbp)
1417a7f36: 4c 89 b5 f0 04 00 00        	movq	%r14, 0x4f0(%rbp)
1417a7f3d: c6 85 62 05 00 00 00        	movb	$0x0, 0x562(%rbp)
1417a7f44: 48 89 f1                    	movq	%rsi, %rcx
1417a7f47: 48 8d 95 30 04 00 00        	leaq	0x430(%rbp), %rdx
1417a7f4e: e8 ad 12 10 01              	callq	0x1428a9200 <.text+0x28a8200>
1417a7f53: 49 89 c6                    	movq	%rax, %r14
1417a7f56: e9 e8 0a 00 00              	jmp	0x1417a8a43 <.text+0x17a7a43>
1417a7f5b: 48 89 8d c8 03 00 00        	movq	%rcx, 0x3c8(%rbp)
1417a7f62: 4c 89 b5 70 03 00 00        	movq	%r14, 0x370(%rbp)
1417a7f69: 4c 8b b5 e8 03 00 00        	movq	0x3e8(%rbp), %r14
1417a7f70: ba 0a 00 00 00              	movl	$0xa, %edx
1417a7f75: 48 8d 0d 24 46 53 01        	leaq	0x1534624(%rip), %rcx   # 0x142cdc5a0
1417a7f7c: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a7f83: e8 48 c5 cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a7f88: 48 85 c0                    	testq	%rax, %rax
1417a7f8b: 75 07                       	jne	0x1417a7f94 <.text+0x17a6f94>
1417a7f8d: 48 8b 05 c4 87 e8 01        	movq	0x1e887c4(%rip), %rax   # 0x143630758
1417a7f94: 0f b6 08                    	movzbl	(%rax), %ecx
1417a7f97: 48 8d 15 5a 4b 53 01        	leaq	0x1534b5a(%rip), %rdx   # 0x142cdcaf8
1417a7f9e: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
1417a7fa2: 48 01 d1                    	addq	%rdx, %rcx
1417a7fa5: ff e1                       	jmpq	*%rcx
1417a7fa7: c6 85 e0 04 00 00 00        	movb	$0x0, 0x4e0(%rbp)
1417a7fae: e9 85 00 00 00              	jmp	0x1417a8038 <.text+0x17a7038>
1417a7fb3: 48 83 c0 08                 	addq	$0x8, %rax
1417a7fb7: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7fbe: 48 89 c2                    	movq	%rax, %rdx
1417a7fc1: 4c 8d 05 d0 43 53 01        	leaq	0x15343d0(%rip), %r8    # 0x142cdc398
1417a7fc8: e8 a3 02 b5 ff              	callq	0x1412f8270 <.text+0x12f7270>
1417a7fcd: c6 85 e0 04 00 00 04        	movb	$0x4, 0x4e0(%rbp)
1417a7fd4: eb 62                       	jmp	0x1417a8038 <.text+0x17a7038>
1417a7fd6: f3 0f 6f 40 08              	movdqu	0x8(%rax), %xmm0
1417a7fdb: 48 8d 85 e8 04 00 00        	leaq	0x4e8(%rbp), %rax
1417a7fe2: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
1417a7fe6: c6 85 e0 04 00 00 02        	movb	$0x2, 0x4e0(%rbp)
1417a7fed: eb 49                       	jmp	0x1417a8038 <.text+0x17a7038>
1417a7fef: 48 83 c0 08                 	addq	$0x8, %rax
1417a7ff3: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a7ffa: 48 89 c2                    	movq	%rax, %rdx
1417a7ffd: e8 8e 3d 5d ff              	callq	0x140d7bd90 <.text+0xd7ad90>
1417a8002: c6 85 e0 04 00 00 03        	movb	$0x3, 0x4e0(%rbp)
1417a8009: eb 2d                       	jmp	0x1417a8038 <.text+0x17a7038>
1417a800b: 0f b6 40 01                 	movzbl	0x1(%rax), %eax
1417a800f: 88 85 e1 04 00 00           	movb	%al, 0x4e1(%rbp)
1417a8015: c6 85 e0 04 00 00 01        	movb	$0x1, 0x4e0(%rbp)
1417a801c: eb 1a                       	jmp	0x1417a8038 <.text+0x17a7038>
1417a801e: 48 83 c0 08                 	addq	$0x8, %rax
1417a8022: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a8029: 48 89 c2                    	movq	%rax, %rdx
1417a802c: e8 8f 51 c9 ff              	callq	0x14143d1c0 <.text+0x143c1c0>
1417a8031: c6 85 e0 04 00 00 05        	movb	$0x5, 0x4e0(%rbp)
1417a8038: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a803f: 48 89 f2                    	movq	%rsi, %rdx
1417a8042: e8 89 47 df ff              	callq	0x14159c7d0 <.text+0x159b7d0>
1417a8047: 41 b9 12 00 00 00           	movl	$0x12, %r9d
1417a804d: 48 8d 8d d8 03 00 00        	leaq	0x3d8(%rbp), %rcx
1417a8054: 48 8d 95 30 04 00 00        	leaq	0x430(%rbp), %rdx
1417a805b: 4c 8d 05 f4 46 53 01        	leaq	0x15346f4(%rip), %r8    # 0x142cdc756
1417a8062: e8 89 dd df ff              	callq	0x1415a5df0 <.text+0x15a4df0>
1417a8067: 4c 89 b5 58 02 00 00        	movq	%r14, 0x258(%rbp)
1417a806e: 48 8b 8d d8 03 00 00        	movq	0x3d8(%rbp), %rcx
1417a8075: 4c 8b b5 e0 03 00 00        	movq	0x3e0(%rbp), %r14
1417a807c: 48 89 c8                    	movq	%rcx, %rax
1417a807f: 48 f7 d8                    	negq	%rax
1417a8082: 0f 80 92 09 00 00           	jo	0x1417a8a1a <.text+0x17a7a1a>
1417a8088: 48 89 8d d0 03 00 00        	movq	%rcx, 0x3d0(%rbp)
1417a808f: 4c 89 b5 78 03 00 00        	movq	%r14, 0x378(%rbp)
1417a8096: 4c 8b b5 e8 03 00 00        	movq	0x3e8(%rbp), %r14
1417a809d: ba 10 00 00 00              	movl	$0x10, %edx
1417a80a2: 48 8d 0d 01 45 53 01        	leaq	0x1534501(%rip), %rcx   # 0x142cdc5aa
1417a80a9: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a80b0: e8 1b c4 cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a80b5: 48 85 c0                    	testq	%rax, %rax
1417a80b8: 75 07                       	jne	0x1417a80c1 <.text+0x17a70c1>
1417a80ba: 48 8b 05 97 86 e8 01        	movq	0x1e88697(%rip), %rax   # 0x143630758
1417a80c1: 48 89 f1                    	movq	%rsi, %rcx
1417a80c4: 48 89 c2                    	movq	%rax, %rdx
1417a80c7: e8 c4 82 ff ff              	callq	0x1417a0390 <.text+0x179f390>
1417a80cc: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a80d3: 48 89 f2                    	movq	%rsi, %rdx
1417a80d6: e8 f5 46 df ff              	callq	0x14159c7d0 <.text+0x159b7d0>
1417a80db: 48 c7 85 e0 04 00 00 00 00 00 00    	movq	$0x0, 0x4e0(%rbp)
1417a80e6: 48 c7 85 e8 04 00 00 08 00 00 00    	movq	$0x8, 0x4e8(%rbp)
1417a80f1: 48 c7 85 f0 04 00 00 00 00 00 00    	movq	$0x0, 0x4f0(%rbp)
1417a80fc: 48 8d 8d 88 02 00 00        	leaq	0x288(%rbp), %rcx
1417a8103: 48 8d 95 30 04 00 00        	leaq	0x430(%rbp), %rdx
1417a810a: 49 89 f0                    	movq	%rsi, %r8
1417a810d: e8 8e 81 ff ff              	callq	0x1417a02a0 <.text+0x179f2a0>
1417a8112: ba 07 00 00 00              	movl	$0x7, %edx
1417a8117: 48 8d 0d 9c 44 53 01        	leaq	0x153449c(%rip), %rcx   # 0x142cdc5ba
1417a811e: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a8125: e8 a6 c3 cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a812a: 48 85 c0                    	testq	%rax, %rax
1417a812d: 75 07                       	jne	0x1417a8136 <.text+0x17a7136>
1417a812f: 48 8b 05 22 86 e8 01        	movq	0x1e88622(%rip), %rax   # 0x143630758
1417a8136: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a813d: 48 89 c2                    	movq	%rax, %rdx
1417a8140: e8 4b 82 ff ff              	callq	0x1417a0390 <.text+0x179f390>
1417a8145: 48 8d 05 c4 db 2f 01        	leaq	0x12fdbc4(%rip), %rax   # 0x142aa5d10
1417a814c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1417a8151: 48 c7 44 24 28 03 00 00 00  	movq	$0x3, 0x28(%rsp)
1417a815a: 41 b9 12 00 00 00           	movl	$0x12, %r9d
1417a8160: 48 89 f1                    	movq	%rsi, %rcx
1417a8163: 48 8d 95 90 03 00 00        	leaq	0x390(%rbp), %rdx
1417a816a: 4c 8d 05 66 db 2f 01        	leaq	0x12fdb66(%rip), %r8    # 0x142aa5cd7
1417a8171: e8 ba 77 df ff              	callq	0x14159f930 <.text+0x159e930>
1417a8176: 41 b9 0f 00 00 00           	movl	$0xf, %r9d
1417a817c: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a8183: 48 89 f2                    	movq	%rsi, %rdx
1417a8186: 4c 8d 05 db 45 53 01        	leaq	0x15345db(%rip), %r8    # 0x142cdc768
1417a818d: e8 4e dd df ff              	callq	0x1415a5ee0 <.text+0x15a4ee0>
1417a8192: 8b 85 30 04 00 00           	movl	0x430(%rbp), %eax
1417a8198: 83 f8 02                    	cmpl	$0x2, %eax
1417a819b: 75 0c                       	jne	0x1417a81a9 <.text+0x17a71a9>
1417a819d: 4c 8b b5 38 04 00 00        	movq	0x438(%rbp), %r14
1417a81a4: e9 2d 08 00 00              	jmp	0x1417a89d6 <.text+0x17a79d6>
1417a81a9: 8b 8d 34 04 00 00           	movl	0x434(%rbp), %ecx
1417a81af: 48 8b 95 38 04 00 00        	movq	0x438(%rbp), %rdx
1417a81b6: 4c 8d 8d 40 04 00 00        	leaq	0x440(%rbp), %r9
1417a81bd: 4d 8b 41 30                 	movq	0x30(%r9), %r8
1417a81c1: 4c 8d 95 e8 03 00 00        	leaq	0x3e8(%rbp), %r10
1417a81c8: 4d 89 42 30                 	movq	%r8, 0x30(%r10)
1417a81cc: f3 41 0f 6f 01              	movdqu	(%r9), %xmm0
1417a81d1: 41 0f 10 49 10              	movups	0x10(%r9), %xmm1
1417a81d6: 41 0f 10 51 20              	movups	0x20(%r9), %xmm2
1417a81db: 41 0f 11 52 20              	movups	%xmm2, 0x20(%r10)
1417a81e0: 41 0f 11 4a 10              	movups	%xmm1, 0x10(%r10)
1417a81e5: f3 41 0f 7f 02              	movdqu	%xmm0, (%r10)
1417a81ea: 89 85 d8 03 00 00           	movl	%eax, 0x3d8(%rbp)
1417a81f0: 89 8d dc 03 00 00           	movl	%ecx, 0x3dc(%rbp)
1417a81f6: 48 89 95 e0 03 00 00        	movq	%rdx, 0x3e0(%rbp)
1417a81fd: ba 05 00 00 00              	movl	$0x5, %edx
1417a8202: 48 8d 0d ca 43 53 01        	leaq	0x15343ca(%rip), %rcx   # 0x142cdc5d3
1417a8209: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a8210: e8 bb c2 cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a8215: 48 85 c0                    	testq	%rax, %rax
1417a8218: 75 07                       	jne	0x1417a8221 <.text+0x17a7221>
1417a821a: 48 8b 05 37 85 e8 01        	movq	0x1e88537(%rip), %rax   # 0x143630758
1417a8221: 48 89 f1                    	movq	%rsi, %rcx
1417a8224: 48 89 c2                    	movq	%rax, %rdx
1417a8227: e8 64 81 ff ff              	callq	0x1417a0390 <.text+0x179f390>
1417a822c: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a8233: 48 89 f2                    	movq	%rsi, %rdx
1417a8236: e8 c5 3d df ff              	callq	0x14159c000 <.text+0x159b000>
1417a823b: 41 b9 0d 00 00 00           	movl	$0xd, %r9d
1417a8241: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a8248: 48 8d 95 30 04 00 00        	leaq	0x430(%rbp), %rdx
1417a824f: 4c 8d 05 21 45 53 01        	leaq	0x1534521(%rip), %r8    # 0x142cdc777
1417a8256: e8 95 db df ff              	callq	0x1415a5df0 <.text+0x15a4df0>
1417a825b: 4c 89 b5 60 02 00 00        	movq	%r14, 0x260(%rbp)
1417a8262: 48 8b 85 90 03 00 00        	movq	0x390(%rbp), %rax
1417a8269: 4c 8b b5 98 03 00 00        	movq	0x398(%rbp), %r14
1417a8270: 48 89 c1                    	movq	%rax, %rcx
1417a8273: 48 f7 d9                    	negq	%rcx
1417a8276: 0f 80 3b 07 00 00           	jo	0x1417a89b7 <.text+0x17a79b7>
1417a827c: 48 8b 8d a0 03 00 00        	movq	0x3a0(%rbp), %rcx
1417a8283: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1417a828a: 4c 89 b5 78 02 00 00        	movq	%r14, 0x278(%rbp)
1417a8291: 48 89 8d 80 02 00 00        	movq	%rcx, 0x280(%rbp)
1417a8298: ba 0b 00 00 00              	movl	$0xb, %edx
1417a829d: 48 8d 0d 34 43 53 01        	leaq	0x1534334(%rip), %rcx   # 0x142cdc5d8
1417a82a4: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a82ab: e8 20 c2 cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a82b0: 48 85 c0                    	testq	%rax, %rax
1417a82b3: 75 07                       	jne	0x1417a82bc <.text+0x17a72bc>
1417a82b5: 48 8b 05 9c 84 e8 01        	movq	0x1e8849c(%rip), %rax   # 0x143630758
1417a82bc: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a82c3: 48 89 c2                    	movq	%rax, %rdx
1417a82c6: e8 c5 80 ff ff              	callq	0x1417a0390 <.text+0x179f390>
1417a82cb: 48 89 f1                    	movq	%rsi, %rcx
1417a82ce: 48 8d 95 30 04 00 00        	leaq	0x430(%rbp), %rdx
1417a82d5: e8 96 38 df ff              	callq	0x14159bb70 <.text+0x159ab70>
1417a82da: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1417a82e1: 48 89 f2                    	movq	%rsi, %rdx
1417a82e4: e8 87 7e ff ff              	callq	0x1417a0170 <.text+0x179f170>
1417a82e9: ba 06 00 00 00              	movl	$0x6, %edx
1417a82ee: 48 8d 0d ee 42 53 01        	leaq	0x15342ee(%rip), %rcx   # 0x142cdc5e3
1417a82f5: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a82fc: e8 cf c1 cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a8301: 48 85 c0                    	testq	%rax, %rax
1417a8304: 75 07                       	jne	0x1417a830d <.text+0x17a730d>
1417a8306: 48 8b 05 4b 84 e8 01        	movq	0x1e8844b(%rip), %rax   # 0x143630758
1417a830d: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a8314: 48 89 c2                    	movq	%rax, %rdx
1417a8317: e8 74 80 ff ff              	callq	0x1417a0390 <.text+0x179f390>
1417a831c: 48 8d 05 d5 f4 4c 01        	leaq	0x14cf4d5(%rip), %rax   # 0x142c777f8
1417a8323: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1417a8328: 48 c7 44 24 28 02 00 00 00  	movq	$0x2, 0x28(%rsp)
1417a8331: 41 b9 0e 00 00 00           	movl	$0xe, %r9d
1417a8337: 48 89 f1                    	movq	%rsi, %rcx
1417a833a: 48 8d 95 30 04 00 00        	leaq	0x430(%rbp), %rdx
1417a8341: 4c 8d 05 90 f4 4c 01        	leaq	0x14cf490(%rip), %r8    # 0x142c777d8
1417a8348: e8 e3 5f df ff              	callq	0x14159e330 <.text+0x159d330>
1417a834d: 4c 8b b5 e0 04 00 00        	movq	0x4e0(%rbp), %r14
1417a8354: 49 83 fe 02                 	cmpq	$0x2, %r14
1417a8358: 75 33                       	jne	0x1417a838d <.text+0x17a738d>
1417a835a: 48 8d 8d e8 04 00 00        	leaq	0x4e8(%rbp), %rcx
1417a8361: e8 3a 05 e4 fe              	callq	0x1405e88a0 <.text+0x5e78a0>
1417a8366: b8 04 00 00 00              	movl	$0x4, %eax
1417a836b: 48 89 85 d0 04 00 00        	movq	%rax, 0x4d0(%rbp)
1417a8372: 45 31 f6                    	xorl	%r14d, %r14d
1417a8375: 48 c7 85 28 04 00 00 00 00 00 00    	movq	$0x0, 0x428(%rbp)
1417a8380: 48 c7 85 80 03 00 00 00 00 00 00    	movq	$0x0, 0x380(%rbp)
1417a838b: eb 31                       	jmp	0x1417a83be <.text+0x17a73be>
1417a838d: 48 8b bd e8 04 00 00        	movq	0x4e8(%rbp), %rdi
1417a8394: 48 8b 85 f0 04 00 00        	movq	0x4f0(%rbp), %rax
1417a839b: 48 89 85 28 04 00 00        	movq	%rax, 0x428(%rbp)
1417a83a2: 48 8b 85 f8 04 00 00        	movq	0x4f8(%rbp), %rax
1417a83a9: 48 89 85 d0 04 00 00        	movq	%rax, 0x4d0(%rbp)
1417a83b0: 48 8b 85 00 05 00 00        	movq	0x500(%rbp), %rax
1417a83b7: 48 89 85 80 03 00 00        	movq	%rax, 0x380(%rbp)
1417a83be: ba 15 00 00 00              	movl	$0x15, %edx
1417a83c3: 48 8d 0d 1f 42 53 01        	leaq	0x153421f(%rip), %rcx   # 0x142cdc5e9
1417a83ca: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a83d1: e8 fa c0 cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a83d6: 48 85 c0                    	testq	%rax, %rax
1417a83d9: 75 07                       	jne	0x1417a83e2 <.text+0x17a73e2>
1417a83db: 48 8b 05 76 83 e8 01        	movq	0x1e88376(%rip), %rax   # 0x143630758
1417a83e2: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a83e9: 48 89 c2                    	movq	%rax, %rdx
1417a83ec: e8 9f 7f ff ff              	callq	0x1417a0390 <.text+0x179f390>
1417a83f1: 48 8d 05 90 5e 34 01        	leaq	0x1345e90(%rip), %rax   # 0x142aee288
1417a83f8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1417a83fd: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1417a8406: 41 b9 15 00 00 00           	movl	$0x15, %r9d
1417a840c: 48 89 f1                    	movq	%rsi, %rcx
1417a840f: 48 8d 95 30 04 00 00        	leaq	0x430(%rbp), %rdx
1417a8416: 4c 8d 05 50 5e 34 01        	leaq	0x1345e50(%rip), %r8    # 0x142aee26d
1417a841d: e8 0e 7b df ff              	callq	0x14159ff30 <.text+0x159ef30>
1417a8422: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a8429: 48 89 f2                    	movq	%rsi, %rdx
1417a842c: e8 3f 7d ff ff              	callq	0x1417a0170 <.text+0x179f170>
1417a8431: 4c 89 b5 68 02 00 00        	movq	%r14, 0x268(%rbp)
1417a8438: 48 89 bd d8 04 00 00        	movq	%rdi, 0x4d8(%rbp)
1417a843f: 48 8b 85 c0 04 00 00        	movq	0x4c0(%rbp), %rax
1417a8446: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
1417a844a: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a8451: 48 8d 04 c1                 	leaq	(%rcx,%rax,8), %rax
1417a8455: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
1417a845c: 48 8b 85 d0 04 00 00        	movq	0x4d0(%rbp), %rax
1417a8463: 48 8b 95 80 03 00 00        	movq	0x380(%rbp), %rdx
1417a846a: 4c 8d 34 d0                 	leaq	(%rax,%rdx,8), %r14
1417a846e: 48 89 9d 88 03 00 00        	movq	%rbx, 0x388(%rbp)
1417a8475: eb 07                       	jmp	0x1417a847e <.text+0x17a747e>
1417a8477: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
1417a847e: 48 3b 8d 48 02 00 00        	cmpq	0x248(%rbp), %rcx
1417a8485: 0f 84 a0 00 00 00           	je	0x1417a852b <.text+0x17a752b>
1417a848b: 48 8d 41 28                 	leaq	0x28(%rcx), %rax
1417a848f: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1417a8496: 48 8b 51 18                 	movq	0x18(%rcx), %rdx
1417a849a: 48 6b 41 20 58              	imulq	$0x58, 0x20(%rcx), %rax
1417a849f: 48 01 d0                    	addq	%rdx, %rax
1417a84a2: 48 89 85 c0 04 00 00        	movq	%rax, 0x4c0(%rbp)
1417a84a9: eb 17                       	jmp	0x1417a84c2 <.text+0x17a74c2>
1417a84ab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1417a84b0: c6 47 56 00                 	movb	$0x0, 0x56(%rdi)
1417a84b4: 48 8b 85 c0 04 00 00        	movq	0x4c0(%rbp), %rax
1417a84bb: 48 8b 95 68 05 00 00        	movq	0x568(%rbp), %rdx
1417a84c2: 48 39 c2                    	cmpq	%rax, %rdx
1417a84c5: 74 b0                       	je	0x1417a8477 <.text+0x17a7477>
1417a84c7: 48 89 d7                    	movq	%rdx, %rdi
1417a84ca: 48 83 c2 58                 	addq	$0x58, %rdx
1417a84ce: 80 7f 56 01                 	cmpb	$0x1, 0x56(%rdi)
1417a84d2: 75 ee                       	jne	0x1417a84c2 <.text+0x17a74c2>
1417a84d4: 48 89 95 68 05 00 00        	movq	%rdx, 0x568(%rbp)
1417a84db: 48 8b 8d d0 04 00 00        	movq	0x4d0(%rbp), %rcx
1417a84e2: eb 28                       	jmp	0x1417a850c <.text+0x17a750c>
1417a84e4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1417a84f0: 44 39 c0                    	cmpl	%r8d, %eax
1417a84f3: 44 89 c2                    	movl	%r8d, %edx
1417a84f6: 0f 42 d0                    	cmovbl	%eax, %edx
1417a84f9: 44 0f 47 c0                 	cmoval	%eax, %r8d
1417a84fd: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a8504: e8 37 01 b2 fe              	callq	0x1402c8640 <.text+0x2c7640>
1417a8509: 48 89 d9                    	movq	%rbx, %rcx
1417a850c: 4c 39 f1                    	cmpq	%r14, %rcx
1417a850f: 74 9f                       	je	0x1417a84b0 <.text+0x17a74b0>
1417a8511: 48 8d 59 08                 	leaq	0x8(%rcx), %rbx
1417a8515: 8b 01                       	movl	(%rcx), %eax
1417a8517: 44 8b 41 04                 	movl	0x4(%rcx), %r8d
1417a851b: 8b 17                       	movl	(%rdi), %edx
1417a851d: 39 d0                       	cmpl	%edx, %eax
1417a851f: 74 cf                       	je	0x1417a84f0 <.text+0x17a74f0>
1417a8521: 48 89 d9                    	movq	%rbx, %rcx
1417a8524: 41 39 d0                    	cmpl	%edx, %r8d
1417a8527: 75 e3                       	jne	0x1417a850c <.text+0x17a750c>
1417a8529: eb c5                       	jmp	0x1417a84f0 <.text+0x17a74f0>
1417a852b: ba 06 00 00 00              	movl	$0x6, %edx
1417a8530: 48 8d 0d 59 40 53 01        	leaq	0x1534059(%rip), %rcx   # 0x142cdc590
1417a8537: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a853e: e8 8d bf cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a8543: 48 85 c0                    	testq	%rax, %rax
1417a8546: 75 07                       	jne	0x1417a854f <.text+0x17a754f>
1417a8548: 48 8b 05 09 82 e8 01        	movq	0x1e88209(%rip), %rax   # 0x143630758
1417a854f: 48 89 f1                    	movq	%rsi, %rcx
1417a8552: 48 89 c2                    	movq	%rax, %rdx
1417a8555: e8 36 7e ff ff              	callq	0x1417a0390 <.text+0x179f390>
1417a855a: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a8561: 48 89 f2                    	movq	%rsi, %rdx
1417a8564: e8 17 47 df ff              	callq	0x14159cc80 <.text+0x159bc80>
1417a8569: 41 b9 0e 00 00 00           	movl	$0xe, %r9d
1417a856f: 48 8d 8d 08 03 00 00        	leaq	0x308(%rbp), %rcx
1417a8576: 48 8d 95 30 04 00 00        	leaq	0x430(%rbp), %rdx
1417a857d: 4c 8d 05 00 42 53 01        	leaq	0x1534200(%rip), %r8    # 0x142cdc784
1417a8584: e8 67 d8 df ff              	callq	0x1415a5df0 <.text+0x15a4df0>
1417a8589: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
1417a8590: 4c 8b b5 10 03 00 00        	movq	0x310(%rbp), %r14
1417a8597: 48 89 c1                    	movq	%rax, %rcx
1417a859a: 48 f7 d9                    	negq	%rcx
1417a859d: 0f 80 c0 03 00 00           	jo	0x1417a8963 <.text+0x17a7963>
1417a85a3: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
1417a85aa: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1417a85ae: 49 8d 0c ce                 	leaq	(%r14,%rcx,8), %rcx
1417a85b2: 4c 89 b5 e0 04 00 00        	movq	%r14, 0x4e0(%rbp)
1417a85b9: 4c 89 b5 e8 04 00 00        	movq	%r14, 0x4e8(%rbp)
1417a85c0: 48 89 85 f0 04 00 00        	movq	%rax, 0x4f0(%rbp)
1417a85c7: 48 89 8d f8 04 00 00        	movq	%rcx, 0x4f8(%rbp)
1417a85ce: 48 8d 85 e0 00 00 00        	leaq	0xe0(%rbp), %rax
1417a85d5: 48 89 85 00 05 00 00        	movq	%rax, 0x500(%rbp)
1417a85dc: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
1417a85e3: 48 89 f2                    	movq	%rsi, %rdx
1417a85e6: e8 05 7a dd ff              	callq	0x14157fff0 <.text+0x157eff0>
1417a85eb: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1417a85f2: 4c 8b b5 08 02 00 00        	movq	0x208(%rbp), %r14
1417a85f9: 48 85 c0                    	testq	%rax, %rax
1417a85fc: 0f 84 61 03 00 00           	je	0x1417a8963 <.text+0x17a7963>
1417a8602: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1417a8609: f3 0f 6f 01                 	movdqu	(%rcx), %xmm0
1417a860d: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
1417a8611: 48 8d 8d 40 04 00 00        	leaq	0x440(%rbp), %rcx
1417a8618: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
1417a861c: f3 0f 7f 01                 	movdqu	%xmm0, (%rcx)
1417a8620: 48 89 85 30 04 00 00        	movq	%rax, 0x430(%rbp)
1417a8627: 4c 89 b5 38 04 00 00        	movq	%r14, 0x438(%rbp)
1417a862e: 48 8d 8d c8 00 00 00        	leaq	0xc8(%rbp), %rcx
1417a8635: 48 8b 95 00 06 00 00        	movq	0x600(%rbp), %rdx
1417a863c: e8 cf 64 09 ff              	callq	0x14083eb10 <.text+0x83db10>
1417a8641: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
1417a8648: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
1417a864f: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
1417a8656: 48 c1 e2 04                 	shlq	$0x4, %rdx
1417a865a: 48 01 ca                    	addq	%rcx, %rdx
1417a865d: 48 89 8d e0 04 00 00        	movq	%rcx, 0x4e0(%rbp)
1417a8664: 48 89 8d e8 04 00 00        	movq	%rcx, 0x4e8(%rbp)
1417a866b: 48 89 85 f0 04 00 00        	movq	%rax, 0x4f0(%rbp)
1417a8672: 48 89 95 f8 04 00 00        	movq	%rdx, 0x4f8(%rbp)
1417a8679: 48 8d 85 30 04 00 00        	leaq	0x430(%rbp), %rax
1417a8680: 48 89 85 00 05 00 00        	movq	%rax, 0x500(%rbp)
1417a8687: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
1417a868e: 48 89 f2                    	movq	%rsi, %rdx
1417a8691: 4c 8d 05 10 3b 53 01        	leaq	0x1533b10(%rip), %r8    # 0x142cdc1a8
1417a8698: e8 73 e7 b4 ff              	callq	0x1412f6e10 <.text+0x12f5e10>
1417a869d: 48 8b 85 48 04 00 00        	movq	0x448(%rbp), %rax
1417a86a4: 48 85 c0                    	testq	%rax, %rax
1417a86a7: 0f 84 46 01 00 00           	je	0x1417a87f3 <.text+0x17a77f3>
1417a86ad: 48 8b 0d f4 3c e8 01        	movq	0x1e83cf4(%rip), %rcx   # 0x14362c3a8
1417a86b4: 48 8b 09                    	movq	(%rcx), %rcx
1417a86b7: 48 83 f9 02                 	cmpq	$0x2, %rcx
1417a86bb: 0f 82 32 01 00 00           	jb	0x1417a87f3 <.text+0x17a77f3>
1417a86c1: 48 8b 8d 30 04 00 00        	movq	0x430(%rbp), %rcx
1417a86c8: 48 8b 95 38 04 00 00        	movq	0x438(%rbp), %rdx
1417a86cf: 48 01 ca                    	addq	%rcx, %rdx
1417a86d2: 48 ff c2                    	incq	%rdx
1417a86d5: 66 0f 6f 01                 	movdqa	(%rcx), %xmm0
1417a86d9: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
1417a86de: 41 f7 d0                    	notl	%r8d
1417a86e1: 48 89 8d e0 04 00 00        	movq	%rcx, 0x4e0(%rbp)
1417a86e8: 48 83 c1 10                 	addq	$0x10, %rcx
1417a86ec: 48 89 8d e8 04 00 00        	movq	%rcx, 0x4e8(%rbp)
1417a86f3: 48 89 95 f0 04 00 00        	movq	%rdx, 0x4f0(%rbp)
1417a86fa: 66 44 89 85 f8 04 00 00     	movw	%r8w, 0x4f8(%rbp)
1417a8702: 48 89 85 00 05 00 00        	movq	%rax, 0x500(%rbp)
1417a8709: c6 85 75 05 00 00 01        	movb	$0x1, 0x575(%rbp)
1417a8710: 48 8d 8d 30 03 00 00        	leaq	0x330(%rbp), %rcx
1417a8717: 48 89 f2                    	movq	%rsi, %rdx
1417a871a: 4c 8d 05 67 74 4b 01        	leaq	0x14b7467(%rip), %r8    # 0x142c5fb88
1417a8721: e8 6a bc b5 ff              	callq	0x141304390 <.text+0x1303390>
1417a8726: 48 8d 85 30 03 00 00        	leaq	0x330(%rbp), %rax
1417a872d: 48 89 85 48 03 00 00        	movq	%rax, 0x348(%rbp)
1417a8734: 48 8d 05 85 ef b4 ff        	leaq	-0x4b107b(%rip), %rax   # 0x1412f76c0 <.text+0x12f66c0>
1417a873b: 48 89 85 50 03 00 00        	movq	%rax, 0x350(%rbp)
1417a8742: 48 8d 05 77 40 53 01        	leaq	0x1534077(%rip), %rax   # 0x142cdc7c0
1417a8749: 48 89 85 e0 04 00 00        	movq	%rax, 0x4e0(%rbp)
1417a8750: 48 c7 85 e8 04 00 00 01 00 00 00    	movq	$0x1, 0x4e8(%rbp)
1417a875b: 48 c7 85 00 05 00 00 00 00 00 00    	movq	$0x0, 0x500(%rbp)
1417a8766: 48 8d 85 48 03 00 00        	leaq	0x348(%rbp), %rax
1417a876d: 48 89 85 f0 04 00 00        	movq	%rax, 0x4f0(%rbp)
1417a8774: 48 c7 85 f8 04 00 00 01 00 00 00    	movq	$0x1, 0x4f8(%rbp)
1417a877f: 48 8d 0d 4a 40 53 01        	leaq	0x153404a(%rip), %rcx   # 0x142cdc7d0
1417a8786: e8 f5 55 ed fe              	callq	0x14067dd80 <.text+0x67cd80>
1417a878b: 48 8d 0d 2e 3f 53 01        	leaq	0x1533f2e(%rip), %rcx   # 0x142cdc6c0
1417a8792: 48 89 8d 08 03 00 00        	movq	%rcx, 0x308(%rbp)
1417a8799: 48 c7 85 10 03 00 00 25 00 00 00    	movq	$0x25, 0x310(%rbp)
1417a87a4: 48 89 8d 18 03 00 00        	movq	%rcx, 0x318(%rbp)
1417a87ab: 48 c7 85 20 03 00 00 25 00 00 00    	movq	$0x25, 0x320(%rbp)
1417a87b6: 48 89 85 28 03 00 00        	movq	%rax, 0x328(%rbp)
1417a87bd: ba 02 00 00 00              	movl	$0x2, %edx
1417a87c2: 48 89 f1                    	movq	%rsi, %rcx
1417a87c5: 4c 8d 85 08 03 00 00        	leaq	0x308(%rbp), %r8
1417a87cc: e8 5f 8c d0 fe              	callq	0x1404b1430 <.text+0x4b0430>
1417a87d1: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
1417a87d8: 48 85 d2                    	testq	%rdx, %rdx
1417a87db: 74 16                       	je	0x1417a87f3 <.text+0x17a77f3>
1417a87dd: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
1417a87e4: 48 c1 e2 03                 	shlq	$0x3, %rdx
1417a87e8: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a87ee: e8 2d b4 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a87f3: f3 0f 6f 85 b0 00 00 00     	movdqu	0xb0(%rbp), %xmm0
1417a87fb: 66 0f 7f 85 a0 02 00 00     	movdqa	%xmm0, 0x2a0(%rbp)
1417a8803: 4c 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %r8
1417a880a: 4c 89 85 b0 02 00 00        	movq	%r8, 0x2b0(%rbp)
1417a8811: 48 8b 95 a8 02 00 00        	movq	0x2a8(%rbp), %rdx
1417a8818: 48 8b 8d 00 06 00 00        	movq	0x600(%rbp), %rcx
1417a881f: e8 ec 6b 09 ff              	callq	0x14083f410 <.text+0x83e410>
1417a8824: 49 89 c6                    	movq	%rax, %r14
1417a8827: 48 85 c0                    	testq	%rax, %rax
1417a882a: 0f 85 01 01 00 00           	jne	0x1417a8931 <.text+0x17a7931>
1417a8830: 48 8b 85 38 02 00 00        	movq	0x238(%rbp), %rax
1417a8837: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1417a883b: 48 8d 8d 48 03 00 00        	leaq	0x348(%rbp), %rcx
1417a8842: 31 d2                       	xorl	%edx, %edx
1417a8844: 4c 8d 0d 9d 3f 53 01        	leaq	0x1533f9d(%rip), %r9    # 0x142cdc7e8
1417a884b: e8 e0 e8 b4 ff              	callq	0x1412f7130 <.text+0x12f6130>
1417a8850: 48 8d 8d 08 03 00 00        	leaq	0x308(%rbp), %rcx
1417a8857: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
1417a885e: e8 0d 85 8d 00              	callq	0x142080d70 <.text+0x207fd70>
1417a8863: 48 8d 8d 30 03 00 00        	leaq	0x330(%rbp), %rcx
1417a886a: 48 8d 95 08 03 00 00        	leaq	0x308(%rbp), %rdx
1417a8871: e8 fa e4 ff ff              	callq	0x1417a6d70 <.text+0x17a5d70>
1417a8876: 48 8b bd 30 03 00 00        	movq	0x330(%rbp), %rdi
1417a887d: 48 85 ff                    	testq	%rdi, %rdi
1417a8880: 0f 84 82 00 00 00           	je	0x1417a8908 <.text+0x17a7908>
1417a8886: 48 8b 9d 38 03 00 00        	movq	0x338(%rbp), %rbx
1417a888d: 48 8d 8d f0 04 00 00        	leaq	0x4f0(%rbp), %rcx
1417a8894: 31 d2                       	xorl	%edx, %edx
1417a8896: e8 a5 73 41 ff              	callq	0x140bbfc40 <.text+0xbbec40>
1417a889b: 48 89 bd e0 04 00 00        	movq	%rdi, 0x4e0(%rbp)
1417a88a2: 48 89 9d e8 04 00 00        	movq	%rbx, 0x4e8(%rbp)
1417a88a9: 48 8b 95 50 03 00 00        	movq	0x350(%rbp), %rdx
1417a88b0: 48 8b bd 58 03 00 00        	movq	0x358(%rbp), %rdi
1417a88b7: 4c 8d 04 bd 00 00 00 00     	leaq	(,%rdi,4), %r8
1417a88bf: 48 89 f1                    	movq	%rsi, %rcx
1417a88c2: 48 89 95 68 05 00 00        	movq	%rdx, 0x568(%rbp)
1417a88c9: e8 c2 81 2f ff              	callq	0x140aa0a90 <.text+0xa9fa90>
1417a88ce: 41 b8 1a 00 00 00           	movl	$0x1a, %r8d
1417a88d4: 48 89 c1                    	movq	%rax, %rcx
1417a88d7: 48 8d 15 22 3f 53 01        	leaq	0x1533f22(%rip), %rdx   # 0x142cdc800
1417a88de: e8 fd d6 df ff              	callq	0x1415a5fe0 <.text+0x15a4fe0>
1417a88e3: 49 89 c6                    	movq	%rax, %r14
1417a88e6: 48 85 c0                    	testq	%rax, %rax
1417a88e9: 0f 84 c5 02 00 00           	je	0x1417a8bb4 <.text+0x17a7bb4>
1417a88ef: 48 8b 8d f0 04 00 00        	movq	0x4f0(%rbp), %rcx
1417a88f6: ba 08 ab 00 00              	movl	$0xab08, %edx           # imm = 0xAB08
1417a88fb: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a8901: e8 1a b3 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a8906: eb 07                       	jmp	0x1417a890f <.text+0x17a790f>
1417a8908: 4c 8b b5 38 03 00 00        	movq	0x338(%rbp), %r14
1417a890f: 48 8b 95 48 03 00 00        	movq	0x348(%rbp), %rdx
1417a8916: 48 85 d2                    	testq	%rdx, %rdx
1417a8919: 74 16                       	je	0x1417a8931 <.text+0x17a7931>
1417a891b: 48 8b 8d 50 03 00 00        	movq	0x350(%rbp), %rcx
1417a8922: 48 c1 e2 02                 	shlq	$0x2, %rdx
1417a8926: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1417a892c: e8 ef b2 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a8931: c6 85 75 05 00 00 00        	movb	$0x0, 0x575(%rbp)
1417a8938: 48 8d 8d a0 02 00 00        	leaq	0x2a0(%rbp), %rcx
1417a893f: e8 3c 12 0d ff              	callq	0x140879b80 <.text+0x878b80>
1417a8944: 41 b8 30 00 00 00           	movl	$0x30, %r8d
1417a894a: 41 b9 10 00 00 00           	movl	$0x10, %r9d
1417a8950: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a8957: 48 8d 95 50 04 00 00        	leaq	0x450(%rbp), %rdx
1417a895e: e8 4d 90 8a fe              	callq	0x1400519b0 <.text+0x509b0>
1417a8963: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a896a: e8 e1 15 9a fe              	callq	0x140149f50 <.text+0x148f50>
1417a896f: 48 8b 9d 88 03 00 00        	movq	0x388(%rbp), %rbx
1417a8976: 48 8b bd d8 04 00 00        	movq	0x4d8(%rbp), %rdi
1417a897d: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
1417a8984: 48 85 d2                    	testq	%rdx, %rdx
1417a8987: 74 16                       	je	0x1417a899f <.text+0x17a799f>
1417a8989: 48 c1 e2 03                 	shlq	$0x3, %rdx
1417a898d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1417a8993: 48 8b 8d d0 04 00 00        	movq	0x4d0(%rbp), %rcx
1417a899a: e8 81 b2 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a899f: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1417a89a6: e8 a5 15 9a fe              	callq	0x140149f50 <.text+0x148f50>
1417a89ab: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
1417a89b2: e8 69 69 0d ff              	callq	0x14087f320 <.text+0x87e320>
1417a89b7: 41 b8 30 00 00 00           	movl	$0x30, %r8d
1417a89bd: 41 b9 10 00 00 00           	movl	$0x10, %r9d
1417a89c3: 48 8d 8d e0 03 00 00        	leaq	0x3e0(%rbp), %rcx
1417a89ca: 48 8d 95 00 04 00 00        	leaq	0x400(%rbp), %rdx
1417a89d1: e8 9a 6b 8a fe              	callq	0x14004f570 <.text+0x4e570>
1417a89d6: 48 8b 95 88 02 00 00        	movq	0x288(%rbp), %rdx
1417a89dd: 48 85 d2                    	testq	%rdx, %rdx
1417a89e0: 74 16                       	je	0x1417a89f8 <.text+0x17a79f8>
1417a89e2: 48 8b 8d 90 02 00 00        	movq	0x290(%rbp), %rcx
1417a89e9: 48 c1 e2 04                 	shlq	$0x4, %rdx
1417a89ed: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a89f3: e8 28 b2 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a89f8: 48 8b 95 d0 03 00 00        	movq	0x3d0(%rbp), %rdx
1417a89ff: 48 85 d2                    	testq	%rdx, %rdx
1417a8a02: 74 16                       	je	0x1417a8a1a <.text+0x17a7a1a>
1417a8a04: 48 c1 e2 04                 	shlq	$0x4, %rdx
1417a8a08: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a8a0e: 48 8b 8d 78 03 00 00        	movq	0x378(%rbp), %rcx
1417a8a15: e8 06 b2 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a8a1a: 48 8b 95 c8 03 00 00        	movq	0x3c8(%rbp), %rdx
1417a8a21: 48 85 d2                    	testq	%rdx, %rdx
1417a8a24: 48 89 bd d8 04 00 00        	movq	%rdi, 0x4d8(%rbp)
1417a8a2b: 74 16                       	je	0x1417a8a43 <.text+0x17a7a43>
1417a8a2d: 48 c1 e2 04                 	shlq	$0x4, %rdx
1417a8a31: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a8a37: 48 8b 8d 70 03 00 00        	movq	0x370(%rbp), %rcx
1417a8a3e: e8 dd b1 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a8a43: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
1417a8a4a: 48 89 85 68 05 00 00        	movq	%rax, 0x568(%rbp)
1417a8a51: 48 8b bd c8 02 00 00        	movq	0x2c8(%rbp), %rdi
1417a8a58: 48 85 ff                    	testq	%rdi, %rdi
1417a8a5b: 74 3a                       	je	0x1417a8a97 <.text+0x17a7a97>
1417a8a5d: 48 8b 85 68 05 00 00        	movq	0x568(%rbp), %rax
1417a8a64: 4c 8d 68 18                 	leaq	0x18(%rax), %r13
1417a8a68: eb 0f                       	jmp	0x1417a8a79 <.text+0x17a7a79>
1417a8a6a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1417a8a70: 49 83 c5 28                 	addq	$0x28, %r13
1417a8a74: 48 ff cf                    	decq	%rdi
1417a8a77: 74 1e                       	je	0x1417a8a97 <.text+0x17a7a97>
1417a8a79: 49 8b 45 f8                 	movq	-0x8(%r13), %rax
1417a8a7d: 48 85 c0                    	testq	%rax, %rax
1417a8a80: 74 ee                       	je	0x1417a8a70 <.text+0x17a7a70>
1417a8a82: 49 8b 4d 00                 	movq	(%r13), %rcx
1417a8a86: 48 6b d0 58                 	imulq	$0x58, %rax, %rdx
1417a8a8a: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a8a90: e8 8b b1 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a8a95: eb d9                       	jmp	0x1417a8a70 <.text+0x17a7a70>
1417a8a97: 48 8b 85 b8 02 00 00        	movq	0x2b8(%rbp), %rax
1417a8a9e: 48 85 c0                    	testq	%rax, %rax
1417a8aa1: 74 1a                       	je	0x1417a8abd <.text+0x17a7abd>
1417a8aa3: 48 c1 e0 03                 	shlq	$0x3, %rax
1417a8aa7: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1417a8aab: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a8ab1: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a8ab8: e8 63 b1 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a8abd: 48 8b 8d d8 04 00 00        	movq	0x4d8(%rbp), %rcx
1417a8ac4: 48 8b 85 c0 03 00 00        	movq	0x3c0(%rbp), %rax
1417a8acb: 48 85 c0                    	testq	%rax, %rax
1417a8ace: 48 89 8d d8 04 00 00        	movq	%rcx, 0x4d8(%rbp)
1417a8ad5: 74 16                       	je	0x1417a8aed <.text+0x17a7aed>
1417a8ad7: 48 6b d0 58                 	imulq	$0x58, %rax, %rdx
1417a8adb: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a8ae1: 48 8b 8d 68 03 00 00        	movq	0x368(%rbp), %rcx
1417a8ae8: e8 33 b1 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a8aed: 48 8b 85 d8 02 00 00        	movq	0x2d8(%rbp), %rax
1417a8af4: 48 89 85 68 05 00 00        	movq	%rax, 0x568(%rbp)
1417a8afb: 48 8b bd e0 02 00 00        	movq	0x2e0(%rbp), %rdi
1417a8b02: 48 85 ff                    	testq	%rdi, %rdi
1417a8b05: 74 5c                       	je	0x1417a8b63 <.text+0x17a7b63>
1417a8b07: 48 8b 85 68 05 00 00        	movq	0x568(%rbp), %rax
1417a8b0e: 4c 8d 68 28                 	leaq	0x28(%rax), %r13
1417a8b12: eb 15                       	jmp	0x1417a8b29 <.text+0x17a7b29>
1417a8b14: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1417a8b20: 49 83 c5 78                 	addq	$0x78, %r13
1417a8b24: 48 ff cf                    	decq	%rdi
1417a8b27: 74 3a                       	je	0x1417a8b63 <.text+0x17a7b63>
1417a8b29: 49 8b 55 e0                 	movq	-0x20(%r13), %rdx
1417a8b2d: 48 85 d2                    	testq	%rdx, %rdx
1417a8b30: 74 13                       	je	0x1417a8b45 <.text+0x17a7b45>
1417a8b32: 49 8b 4d e8                 	movq	-0x18(%r13), %rcx
1417a8b36: 48 c1 e2 03                 	shlq	$0x3, %rdx
1417a8b3a: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1417a8b40: e8 db b0 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a8b45: 49 8b 55 f8                 	movq	-0x8(%r13), %rdx
1417a8b49: 48 85 d2                    	testq	%rdx, %rdx
1417a8b4c: 74 d2                       	je	0x1417a8b20 <.text+0x17a7b20>
1417a8b4e: 49 8b 4d 00                 	movq	(%r13), %rcx
1417a8b52: 48 c1 e2 02                 	shlq	$0x2, %rdx
1417a8b56: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1417a8b5c: e8 bf b0 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a8b61: eb bd                       	jmp	0x1417a8b20 <.text+0x17a7b20>
1417a8b63: 48 8b 85 d0 02 00 00        	movq	0x2d0(%rbp), %rax
1417a8b6a: 48 85 c0                    	testq	%rax, %rax
1417a8b6d: 0f 84 27 ec ff ff           	je	0x1417a779a <.text+0x17a679a>
1417a8b73: 48 6b d0 78                 	imulq	$0x78, %rax, %rdx
1417a8b77: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a8b7d: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a8b84: e8 97 b0 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a8b89: e9 0c ec ff ff              	jmp	0x1417a779a <.text+0x17a679a>
1417a8b8e: 48 c7 43 08 02 00 00 00     	movq	$0x2, 0x8(%rbx)
1417a8b96: 48 89 d8                    	movq	%rbx, %rax
1417a8b99: 0f 28 b5 80 05 00 00        	movaps	0x580(%rbp), %xmm6
1417a8ba0: 48 81 c4 18 06 00 00        	addq	$0x618, %rsp            # imm = 0x618
1417a8ba7: 5b                          	popq	%rbx
1417a8ba8: 5f                          	popq	%rdi
1417a8ba9: 5e                          	popq	%rsi
1417a8baa: 41 5c                       	popq	%r12
1417a8bac: 41 5d                       	popq	%r13
1417a8bae: 41 5e                       	popq	%r14
1417a8bb0: 41 5f                       	popq	%r15
1417a8bb2: 5d                          	popq	%rbp
1417a8bb3: c3                          	retq
1417a8bb4: 48 8b 8d f0 04 00 00        	movq	0x4f0(%rbp), %rcx
1417a8bbb: ba 08 ab 00 00              	movl	$0xab08, %edx           # imm = 0xAB08
1417a8bc0: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a8bc6: e8 55 b0 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a8bcb: 66 0f 6f 85 a0 02 00 00     	movdqa	0x2a0(%rbp), %xmm0
1417a8bd3: 66 0f 7f 85 b0 01 00 00     	movdqa	%xmm0, 0x1b0(%rbp)
1417a8bdb: 48 8b 85 b0 02 00 00        	movq	0x2b0(%rbp), %rax
1417a8be2: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1417a8be9: 48 8b 85 48 03 00 00        	movq	0x348(%rbp), %rax
1417a8bf0: 48 89 85 20 04 00 00        	movq	%rax, 0x420(%rbp)
1417a8bf7: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a8bfe: 41 b8 30 00 00 00           	movl	$0x30, %r8d
1417a8c04: 41 b9 10 00 00 00           	movl	$0x10, %r9d
1417a8c0a: 48 8d 95 50 04 00 00        	leaq	0x450(%rbp), %rdx
1417a8c11: e8 9a 8d 8a fe              	callq	0x1400519b0 <.text+0x509b0>
1417a8c16: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1417a8c1d: 48 89 85 40 04 00 00        	movq	%rax, 0x440(%rbp)
1417a8c24: 66 0f 6f 85 b0 01 00 00     	movdqa	0x1b0(%rbp), %xmm0
1417a8c2c: 66 0f 7f 85 30 04 00 00     	movdqa	%xmm0, 0x430(%rbp)
1417a8c34: 48 8d 0d 86 39 53 01        	leaq	0x1533986(%rip), %rcx   # 0x142cdc5c1
1417a8c3b: 4c 8d 85 a0 04 00 00        	leaq	0x4a0(%rbp), %r8
1417a8c42: ba 12 00 00 00              	movl	$0x12, %edx
1417a8c47: e8 84 b8 cc ff              	callq	0x1414744d0 <.text+0x14734d0>
1417a8c4c: 48 85 c0                    	testq	%rax, %rax
1417a8c4f: 75 07                       	jne	0x1417a8c58 <.text+0x17a7c58>
1417a8c51: 48 8b 05 00 7b e8 01        	movq	0x1e87b00(%rip), %rax   # 0x143630758
1417a8c58: 48 8d 8d e0 04 00 00        	leaq	0x4e0(%rbp), %rcx
1417a8c5f: 48 89 c2                    	movq	%rax, %rdx
1417a8c62: e8 29 77 ff ff              	callq	0x1417a0390 <.text+0x179f390>
1417a8c67: 48 8d 8d 08 03 00 00        	leaq	0x308(%rbp), %rcx
1417a8c6e: 48 8d 95 e0 04 00 00        	leaq	0x4e0(%rbp), %rdx
1417a8c75: e8 a6 14 df ff              	callq	0x14159a120 <.text+0x1599120>
1417a8c7a: 83 bd 08 03 00 00 01        	cmpl	$0x1, 0x308(%rbp)
1417a8c81: 75 16                       	jne	0x1417a8c99 <.text+0x17a7c99>
1417a8c83: 48 8d 8d 10 03 00 00        	leaq	0x310(%rbp), %rcx
1417a8c8a: e8 11 fc e3 fe              	callq	0x1405e88a0 <.text+0x5e78a0>
1417a8c8f: f2 0f 10 35 e9 33 53 01     	movsd	0x15333e9(%rip), %xmm6  # 0x142cdc080
1417a8c97: eb 08                       	jmp	0x1417a8ca1 <.text+0x17a7ca1>
1417a8c99: f2 0f 10 b5 0c 03 00 00     	movsd	0x30c(%rbp), %xmm6
1417a8ca1: 0f 10 85 d0 02 00 00        	movups	0x2d0(%rbp), %xmm0
1417a8ca8: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
1417a8caf: 48 8b 85 e0 02 00 00        	movq	0x2e0(%rbp), %rax
1417a8cb6: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1417a8cbd: 0f 10 85 b8 02 00 00        	movups	0x2b8(%rbp), %xmm0
1417a8cc4: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
1417a8ccb: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
1417a8cd2: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
1417a8cd9: 0f 10 85 88 02 00 00        	movups	0x288(%rbp), %xmm0
1417a8ce0: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
1417a8ce7: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1417a8cee: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1417a8cf5: 0f 10 85 d8 03 00 00        	movups	0x3d8(%rbp), %xmm0
1417a8cfc: 0f 10 8d e8 03 00 00        	movups	0x3e8(%rbp), %xmm1
1417a8d03: 0f 10 95 f8 03 00 00        	movups	0x3f8(%rbp), %xmm2
1417a8d0a: 0f 10 9d 08 04 00 00        	movups	0x408(%rbp), %xmm3
1417a8d11: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
1417a8d15: 0f 29 4d 40                 	movaps	%xmm1, 0x40(%rbp)
1417a8d19: 0f 29 55 50                 	movaps	%xmm2, 0x50(%rbp)
1417a8d1d: 0f 29 5d 60                 	movaps	%xmm3, 0x60(%rbp)
1417a8d21: 48 8b 85 18 04 00 00        	movq	0x418(%rbp), %rax
1417a8d28: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1417a8d2c: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
1417a8d33: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1417a8d3a: 0f 28 85 30 04 00 00        	movaps	0x430(%rbp), %xmm0
1417a8d41: 0f 29 85 70 01 00 00        	movaps	%xmm0, 0x170(%rbp)
1417a8d48: 48 8b 85 80 02 00 00        	movq	0x280(%rbp), %rax
1417a8d4f: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1417a8d56: 0f 10 85 70 02 00 00        	movups	0x270(%rbp), %xmm0
1417a8d5d: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
1417a8d64: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
1417a8d6b: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
1417a8d72: 0f 10 95 a0 00 00 00        	movups	0xa0(%rbp), %xmm2
1417a8d79: 0f 29 55 f0                 	movaps	%xmm2, -0x10(%rbp)
1417a8d7d: 0f 29 4d e0                 	movaps	%xmm1, -0x20(%rbp)
1417a8d81: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
1417a8d85: f3 0f 6f 85 90 03 00 00     	movdqu	0x390(%rbp), %xmm0
1417a8d8d: 0f 10 8d a0 03 00 00        	movups	0x3a0(%rbp), %xmm1
1417a8d94: 0f 10 95 b0 03 00 00        	movups	0x3b0(%rbp), %xmm2
1417a8d9b: 0f 29 55 20                 	movaps	%xmm2, 0x20(%rbp)
1417a8d9f: 0f 29 4d 10                 	movaps	%xmm1, 0x10(%rbp)
1417a8da3: 66 0f 7f 45 00              	movdqa	%xmm0, (%rbp)
1417a8da8: 48 8d 8d a0 04 00 00        	leaq	0x4a0(%rbp), %rcx
1417a8daf: e8 dc 4c 0b ff              	callq	0x14085da90 <.text+0x85ca90>
1417a8db4: 48 8b b5 88 03 00 00        	movq	0x388(%rbp), %rsi
1417a8dbb: c6 85 77 05 00 00 01        	movb	$0x1, 0x577(%rbp)
1417a8dc2: 48 8d 8d d8 01 00 00        	leaq	0x1d8(%rbp), %rcx
1417a8dc9: e8 a2 3a ee ff              	callq	0x14168c870 <.text+0x168b870>
1417a8dce: 48 8b 85 28 04 00 00        	movq	0x428(%rbp), %rax
1417a8dd5: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
1417a8dd9: 48 8b 85 d0 04 00 00        	movq	0x4d0(%rbp), %rax
1417a8de0: 48 89 46 20                 	movq	%rax, 0x20(%rsi)
1417a8de4: 48 8b 85 80 03 00 00        	movq	0x380(%rbp), %rax
1417a8deb: 48 89 46 28                 	movq	%rax, 0x28(%rsi)
1417a8def: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
1417a8df6: 48 89 46 40                 	movq	%rax, 0x40(%rsi)
1417a8dfa: 0f 28 85 10 01 00 00        	movaps	0x110(%rbp), %xmm0
1417a8e01: 0f 11 46 30                 	movups	%xmm0, 0x30(%rsi)
1417a8e05: 48 8b 85 c0 03 00 00        	movq	0x3c0(%rbp), %rax
1417a8e0c: 48 89 46 48                 	movq	%rax, 0x48(%rsi)
1417a8e10: 48 8b 85 68 03 00 00        	movq	0x368(%rbp), %rax
1417a8e17: 48 89 46 50                 	movq	%rax, 0x50(%rsi)
1417a8e1b: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
1417a8e22: 48 89 46 58                 	movq	%rax, 0x58(%rsi)
1417a8e26: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
1417a8e2d: 48 89 46 70                 	movq	%rax, 0x70(%rsi)
1417a8e31: 0f 28 85 30 01 00 00        	movaps	0x130(%rbp), %xmm0
1417a8e38: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
1417a8e3c: 48 8b 85 c8 03 00 00        	movq	0x3c8(%rbp), %rax
1417a8e43: 48 89 46 78                 	movq	%rax, 0x78(%rsi)
1417a8e47: 48 8b 85 70 03 00 00        	movq	0x370(%rbp), %rax
1417a8e4e: 48 89 86 80 00 00 00        	movq	%rax, 0x80(%rsi)
1417a8e55: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
1417a8e5c: 48 89 86 88 00 00 00        	movq	%rax, 0x88(%rsi)
1417a8e63: 48 8b 85 d0 03 00 00        	movq	0x3d0(%rbp), %rax
1417a8e6a: 48 89 86 90 00 00 00        	movq	%rax, 0x90(%rsi)
1417a8e71: 48 8b 85 78 03 00 00        	movq	0x378(%rbp), %rax
1417a8e78: 48 89 86 98 00 00 00        	movq	%rax, 0x98(%rsi)
1417a8e7f: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
1417a8e86: 48 89 86 a0 00 00 00        	movq	%rax, 0xa0(%rsi)
1417a8e8d: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1417a8e94: 48 89 86 b8 00 00 00        	movq	%rax, 0xb8(%rsi)
1417a8e9b: 0f 28 85 50 01 00 00        	movaps	0x150(%rbp), %xmm0
1417a8ea2: 0f 11 86 a8 00 00 00        	movups	%xmm0, 0xa8(%rsi)
1417a8ea9: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1417a8eb0: 48 89 86 d0 00 00 00        	movq	%rax, 0xd0(%rsi)
1417a8eb7: 0f 28 85 70 01 00 00        	movaps	0x170(%rbp), %xmm0
1417a8ebe: 0f 11 86 c0 00 00 00        	movups	%xmm0, 0xc0(%rsi)
1417a8ec5: 48 8b 85 20 04 00 00        	movq	0x420(%rbp), %rax
1417a8ecc: 48 89 86 d8 00 00 00        	movq	%rax, 0xd8(%rsi)
1417a8ed3: 48 8b 85 68 05 00 00        	movq	0x568(%rbp), %rax
1417a8eda: 48 89 86 e0 00 00 00        	movq	%rax, 0xe0(%rsi)
1417a8ee1: 48 89 be e8 00 00 00        	movq	%rdi, 0xe8(%rsi)
1417a8ee8: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
1417a8eef: 48 89 86 00 01 00 00        	movq	%rax, 0x100(%rsi)
1417a8ef6: 0f 28 85 90 01 00 00        	movaps	0x190(%rbp), %xmm0
1417a8efd: 0f 11 86 f0 00 00 00        	movups	%xmm0, 0xf0(%rsi)
1417a8f04: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1417a8f08: 48 89 86 48 01 00 00        	movq	%rax, 0x148(%rsi)
1417a8f0f: 0f 28 45 30                 	movaps	0x30(%rbp), %xmm0
1417a8f13: 0f 28 4d 40                 	movaps	0x40(%rbp), %xmm1
1417a8f17: 0f 28 55 50                 	movaps	0x50(%rbp), %xmm2
1417a8f1b: 0f 28 5d 60                 	movaps	0x60(%rbp), %xmm3
1417a8f1f: 0f 11 9e 38 01 00 00        	movups	%xmm3, 0x138(%rsi)
1417a8f26: 0f 11 96 28 01 00 00        	movups	%xmm2, 0x128(%rsi)
1417a8f2d: 0f 11 8e 18 01 00 00        	movups	%xmm1, 0x118(%rsi)
1417a8f34: 0f 11 86 08 01 00 00        	movups	%xmm0, 0x108(%rsi)
1417a8f3b: 0f 28 45 d0                 	movaps	-0x30(%rbp), %xmm0
1417a8f3f: 0f 28 4d e0                 	movaps	-0x20(%rbp), %xmm1
1417a8f43: 0f 28 55 f0                 	movaps	-0x10(%rbp), %xmm2
1417a8f47: 0f 11 96 70 01 00 00        	movups	%xmm2, 0x170(%rsi)
1417a8f4e: 0f 11 8e 60 01 00 00        	movups	%xmm1, 0x160(%rsi)
1417a8f55: 0f 11 86 50 01 00 00        	movups	%xmm0, 0x150(%rsi)
1417a8f5c: 0f 28 45 00                 	movaps	(%rbp), %xmm0
1417a8f60: 0f 28 4d 10                 	movaps	0x10(%rbp), %xmm1
1417a8f64: 0f 28 55 20                 	movaps	0x20(%rbp), %xmm2
1417a8f68: 0f 11 96 a0 01 00 00        	movups	%xmm2, 0x1a0(%rsi)
1417a8f6f: 0f 11 8e 90 01 00 00        	movups	%xmm1, 0x190(%rsi)
1417a8f76: 0f 11 86 80 01 00 00        	movups	%xmm0, 0x180(%rsi)
1417a8f7d: 44 89 2e                    	movl	%r13d, (%rsi)
1417a8f80: 48 8b 85 68 02 00 00        	movq	0x268(%rbp), %rax
1417a8f87: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1417a8f8b: 48 8b 85 d8 04 00 00        	movq	0x4d8(%rbp), %rax
1417a8f92: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
1417a8f96: 0f 13 b6 b0 01 00 00        	movlps	%xmm6, 0x1b0(%rsi)
1417a8f9d: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
1417a8fa4: 48 85 d2                    	testq	%rdx, %rdx
1417a8fa7: 74 12                       	je	0x1417a8fbb <.text+0x17a7fbb>
1417a8fa9: 48 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %rcx
1417a8fb0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1417a8fb6: e8 65 ac e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a8fbb: 48 8b 9d 88 03 00 00        	movq	0x388(%rbp), %rbx
1417a8fc2: e9 cf fb ff ff              	jmp	0x1417a8b96 <.text+0x17a7b96>
1417a8fc7: 48 8d 0d aa 60 4b 01        	leaq	0x14b60aa(%rip), %rcx   # 0x142c5f078
1417a8fce: e8 3d 03 13 01              	callq	0x1428d9310 <.text+0x28d8310>
1417a8fd3: eb 0c                       	jmp	0x1417a8fe1 <.text+0x17a7fe1>
1417a8fd5: 48 8d 0d 9c 60 4b 01        	leaq	0x14b609c(%rip), %rcx   # 0x142c5f078
1417a8fdc: e8 2f 03 13 01              	callq	0x1428d9310 <.text+0x28d8310>
1417a8fe1: 0f 0b                       	ud2
1417a8fe3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1417a8ff0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a8ff5: 55                          	pushq	%rbp
1417a8ff6: 41 57                       	pushq	%r15
1417a8ff8: 41 56                       	pushq	%r14
1417a8ffa: 41 55                       	pushq	%r13
1417a8ffc: 41 54                       	pushq	%r12
1417a8ffe: 56                          	pushq	%rsi
1417a8fff: 57                          	pushq	%rdi
1417a9000: 53                          	pushq	%rbx
1417a9001: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9005: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a900c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9011: 48 83 bd 20 04 00 00 00     	cmpq	$0x0, 0x420(%rbp)
1417a9019: 74 1d                       	je	0x1417a9038 <.text+0x17a8038>
1417a901b: 48 8b 95 20 04 00 00        	movq	0x420(%rbp), %rdx
1417a9022: 48 c1 e2 02                 	shlq	$0x2, %rdx
1417a9026: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1417a902c: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a9033: e8 e8 ab e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a9038: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a903f: e8 3c 0b 0d ff              	callq	0x140879b80 <.text+0x878b80>
1417a9044: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9049: 48 83 c4 48                 	addq	$0x48, %rsp
1417a904d: 5b                          	popq	%rbx
1417a904e: 5f                          	popq	%rdi
1417a904f: 5e                          	popq	%rsi
1417a9050: 41 5c                       	popq	%r12
1417a9052: 41 5d                       	popq	%r13
1417a9054: 41 5e                       	popq	%r14
1417a9056: 41 5f                       	popq	%r15
1417a9058: 5d                          	popq	%rbp
1417a9059: c3                          	retq
1417a905a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1417a9060: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9065: 55                          	pushq	%rbp
1417a9066: 41 57                       	pushq	%r15
1417a9068: 41 56                       	pushq	%r14
1417a906a: 41 55                       	pushq	%r13
1417a906c: 41 54                       	pushq	%r12
1417a906e: 56                          	pushq	%rsi
1417a906f: 57                          	pushq	%rdi
1417a9070: 53                          	pushq	%rbx
1417a9071: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9075: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a907c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9081: 48 8b 8d f0 04 00 00        	movq	0x4f0(%rbp), %rcx
1417a9088: ba 08 ab 00 00              	movl	$0xab08, %edx           # imm = 0xAB08
1417a908d: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a9093: e8 88 ab e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a9098: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a909d: 48 83 c4 48                 	addq	$0x48, %rsp
1417a90a1: 5b                          	popq	%rbx
1417a90a2: 5f                          	popq	%rdi
1417a90a3: 5e                          	popq	%rsi
1417a90a4: 41 5c                       	popq	%r12
1417a90a6: 41 5d                       	popq	%r13
1417a90a8: 41 5e                       	popq	%r14
1417a90aa: 41 5f                       	popq	%r15
1417a90ac: 5d                          	popq	%rbp
1417a90ad: c3                          	retq
1417a90ae: 66 90                       	nop
1417a90b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a90b5: 55                          	pushq	%rbp
1417a90b6: 41 57                       	pushq	%r15
1417a90b8: 41 56                       	pushq	%r14
1417a90ba: 41 55                       	pushq	%r13
1417a90bc: 41 54                       	pushq	%r12
1417a90be: 56                          	pushq	%rsi
1417a90bf: 57                          	pushq	%rdi
1417a90c0: 53                          	pushq	%rbx
1417a90c1: 48 83 ec 48                 	subq	$0x48, %rsp
1417a90c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a90cc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a90d1: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
1417a90d8: 48 85 d2                    	testq	%rdx, %rdx
1417a90db: 74 16                       	je	0x1417a90f3 <.text+0x17a80f3>
1417a90dd: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
1417a90e4: 48 c1 e2 03                 	shlq	$0x3, %rdx
1417a90e8: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a90ee: e8 2d ab e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a90f3: c6 85 75 05 00 00 01        	movb	$0x1, 0x575(%rbp)
1417a90fa: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a90ff: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9103: 5b                          	popq	%rbx
1417a9104: 5f                          	popq	%rdi
1417a9105: 5e                          	popq	%rsi
1417a9106: 41 5c                       	popq	%r12
1417a9108: 41 5d                       	popq	%r13
1417a910a: 41 5e                       	popq	%r14
1417a910c: 41 5f                       	popq	%r15
1417a910e: 5d                          	popq	%rbp
1417a910f: c3                          	retq
1417a9110: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9115: 55                          	pushq	%rbp
1417a9116: 41 57                       	pushq	%r15
1417a9118: 41 56                       	pushq	%r14
1417a911a: 41 55                       	pushq	%r13
1417a911c: 41 54                       	pushq	%r12
1417a911e: 56                          	pushq	%rsi
1417a911f: 57                          	pushq	%rdi
1417a9120: 53                          	pushq	%rbx
1417a9121: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9125: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a912c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9131: 48 8b 95 48 03 00 00        	movq	0x348(%rbp), %rdx
1417a9138: 48 85 d2                    	testq	%rdx, %rdx
1417a913b: 74 16                       	je	0x1417a9153 <.text+0x17a8153>
1417a913d: 48 8b 8d 50 03 00 00        	movq	0x350(%rbp), %rcx
1417a9144: 48 c1 e2 02                 	shlq	$0x2, %rdx
1417a9148: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1417a914e: e8 cd aa e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a9153: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9158: 48 83 c4 48                 	addq	$0x48, %rsp
1417a915c: 5b                          	popq	%rbx
1417a915d: 5f                          	popq	%rdi
1417a915e: 5e                          	popq	%rsi
1417a915f: 41 5c                       	popq	%r12
1417a9161: 41 5d                       	popq	%r13
1417a9163: 41 5e                       	popq	%r14
1417a9165: 41 5f                       	popq	%r15
1417a9167: 5d                          	popq	%rbp
1417a9168: c3                          	retq
1417a9169: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1417a9170: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9175: 55                          	pushq	%rbp
1417a9176: 41 57                       	pushq	%r15
1417a9178: 41 56                       	pushq	%r14
1417a917a: 41 55                       	pushq	%r13
1417a917c: 41 54                       	pushq	%r12
1417a917e: 56                          	pushq	%rsi
1417a917f: 57                          	pushq	%rdi
1417a9180: 53                          	pushq	%rbx
1417a9181: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9185: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a918c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9191: 48 8d 8d a0 02 00 00        	leaq	0x2a0(%rbp), %rcx
1417a9198: e8 e3 09 0d ff              	callq	0x140879b80 <.text+0x878b80>
1417a919d: c6 85 75 05 00 00 00        	movb	$0x0, 0x575(%rbp)
1417a91a4: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a91a9: 48 83 c4 48                 	addq	$0x48, %rsp
1417a91ad: 5b                          	popq	%rbx
1417a91ae: 5f                          	popq	%rdi
1417a91af: 5e                          	popq	%rsi
1417a91b0: 41 5c                       	popq	%r12
1417a91b2: 41 5d                       	popq	%r13
1417a91b4: 41 5e                       	popq	%r14
1417a91b6: 41 5f                       	popq	%r15
1417a91b8: 5d                          	popq	%rbp
1417a91b9: c3                          	retq
1417a91ba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1417a91c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a91c5: 55                          	pushq	%rbp
1417a91c6: 41 57                       	pushq	%r15
1417a91c8: 41 56                       	pushq	%r14
1417a91ca: 41 55                       	pushq	%r13
1417a91cc: 41 54                       	pushq	%r12
1417a91ce: 56                          	pushq	%rsi
1417a91cf: 57                          	pushq	%rdi
1417a91d0: 53                          	pushq	%rbx
1417a91d1: 48 83 ec 48                 	subq	$0x48, %rsp
1417a91d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a91dc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a91e1: 80 bd 75 05 00 00 00        	cmpb	$0x0, 0x575(%rbp)
1417a91e8: 74 22                       	je	0x1417a920c <.text+0x17a820c>
1417a91ea: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
1417a91f1: e8 8a 09 0d ff              	callq	0x140879b80 <.text+0x878b80>
1417a91f6: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a91fb: 48 83 c4 48                 	addq	$0x48, %rsp
1417a91ff: 5b                          	popq	%rbx
1417a9200: 5f                          	popq	%rdi
1417a9201: 5e                          	popq	%rsi
1417a9202: 41 5c                       	popq	%r12
1417a9204: 41 5d                       	popq	%r13
1417a9206: 41 5e                       	popq	%r14
1417a9208: 41 5f                       	popq	%r15
1417a920a: 5d                          	popq	%rbp
1417a920b: c3                          	retq
1417a920c: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9211: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9215: 5b                          	popq	%rbx
1417a9216: 5f                          	popq	%rdi
1417a9217: 5e                          	popq	%rsi
1417a9218: 41 5c                       	popq	%r12
1417a921a: 41 5d                       	popq	%r13
1417a921c: 41 5e                       	popq	%r14
1417a921e: 41 5f                       	popq	%r15
1417a9220: 5d                          	popq	%rbp
1417a9221: c3                          	retq
1417a9222: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1417a9230: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9235: 55                          	pushq	%rbp
1417a9236: 41 57                       	pushq	%r15
1417a9238: 41 56                       	pushq	%r14
1417a923a: 41 55                       	pushq	%r13
1417a923c: 41 54                       	pushq	%r12
1417a923e: 56                          	pushq	%rsi
1417a923f: 57                          	pushq	%rdi
1417a9240: 53                          	pushq	%rbx
1417a9241: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9245: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a924c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9251: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1417a9258: 41 b8 30 00 00 00           	movl	$0x30, %r8d
1417a925e: 41 b9 10 00 00 00           	movl	$0x10, %r9d
1417a9264: 48 8d 95 50 04 00 00        	leaq	0x450(%rbp), %rdx
1417a926b: e8 40 87 8a fe              	callq	0x1400519b0 <.text+0x509b0>
1417a9270: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9275: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9279: 5b                          	popq	%rbx
1417a927a: 5f                          	popq	%rdi
1417a927b: 5e                          	popq	%rsi
1417a927c: 41 5c                       	popq	%r12
1417a927e: 41 5d                       	popq	%r13
1417a9280: 41 5e                       	popq	%r14
1417a9282: 41 5f                       	popq	%r15
1417a9284: 5d                          	popq	%rbp
1417a9285: c3                          	retq
1417a9286: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1417a9290: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9295: 55                          	pushq	%rbp
1417a9296: 41 57                       	pushq	%r15
1417a9298: 41 56                       	pushq	%r14
1417a929a: 41 55                       	pushq	%r13
1417a929c: 41 54                       	pushq	%r12
1417a929e: 56                          	pushq	%rsi
1417a929f: 57                          	pushq	%rdi
1417a92a0: 53                          	pushq	%rbx
1417a92a1: 48 83 ec 48                 	subq	$0x48, %rsp
1417a92a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a92ac: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a92b1: 80 bd 62 05 00 00 00        	cmpb	$0x0, 0x562(%rbp)
1417a92b8: 74 0c                       	je	0x1417a92c6 <.text+0x17a82c6>
1417a92ba: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a92c1: e8 da f5 e3 fe              	callq	0x1405e88a0 <.text+0x5e78a0>
1417a92c6: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a92cb: 48 83 c4 48                 	addq	$0x48, %rsp
1417a92cf: 5b                          	popq	%rbx
1417a92d0: 5f                          	popq	%rdi
1417a92d1: 5e                          	popq	%rsi
1417a92d2: 41 5c                       	popq	%r12
1417a92d4: 41 5d                       	popq	%r13
1417a92d6: 41 5e                       	popq	%r14
1417a92d8: 41 5f                       	popq	%r15
1417a92da: 5d                          	popq	%rbp
1417a92db: c3                          	retq
1417a92dc: 0f 1f 40 00                 	nopl	(%rax)
1417a92e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a92e5: 55                          	pushq	%rbp
1417a92e6: 41 57                       	pushq	%r15
1417a92e8: 41 56                       	pushq	%r14
1417a92ea: 41 55                       	pushq	%r13
1417a92ec: 41 54                       	pushq	%r12
1417a92ee: 56                          	pushq	%rsi
1417a92ef: 57                          	pushq	%rdi
1417a92f0: 53                          	pushq	%rbx
1417a92f1: 48 83 ec 48                 	subq	$0x48, %rsp
1417a92f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a92fc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9301: 80 bd 63 05 00 00 00        	cmpb	$0x0, 0x563(%rbp)
1417a9308: 74 0c                       	je	0x1417a9316 <.text+0x17a8316>
1417a930a: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a9311: e8 8a f5 e3 fe              	callq	0x1405e88a0 <.text+0x5e78a0>
1417a9316: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a931b: 48 83 c4 48                 	addq	$0x48, %rsp
1417a931f: 5b                          	popq	%rbx
1417a9320: 5f                          	popq	%rdi
1417a9321: 5e                          	popq	%rsi
1417a9322: 41 5c                       	popq	%r12
1417a9324: 41 5d                       	popq	%r13
1417a9326: 41 5e                       	popq	%r14
1417a9328: 41 5f                       	popq	%r15
1417a932a: 5d                          	popq	%rbp
1417a932b: c3                          	retq
1417a932c: 0f 1f 40 00                 	nopl	(%rax)
1417a9330: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9335: 55                          	pushq	%rbp
1417a9336: 41 57                       	pushq	%r15
1417a9338: 41 56                       	pushq	%r14
1417a933a: 41 55                       	pushq	%r13
1417a933c: 41 54                       	pushq	%r12
1417a933e: 56                          	pushq	%rsi
1417a933f: 57                          	pushq	%rdi
1417a9340: 53                          	pushq	%rbx
1417a9341: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9345: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a934c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9351: 80 bd 64 05 00 00 00        	cmpb	$0x0, 0x564(%rbp)
1417a9358: 74 0c                       	je	0x1417a9366 <.text+0x17a8366>
1417a935a: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a9361: e8 3a f5 e3 fe              	callq	0x1405e88a0 <.text+0x5e78a0>
1417a9366: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a936b: 48 83 c4 48                 	addq	$0x48, %rsp
1417a936f: 5b                          	popq	%rbx
1417a9370: 5f                          	popq	%rdi
1417a9371: 5e                          	popq	%rsi
1417a9372: 41 5c                       	popq	%r12
1417a9374: 41 5d                       	popq	%r13
1417a9376: 41 5e                       	popq	%r14
1417a9378: 41 5f                       	popq	%r15
1417a937a: 5d                          	popq	%rbp
1417a937b: c3                          	retq
1417a937c: 0f 1f 40 00                 	nopl	(%rax)
1417a9380: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9385: 55                          	pushq	%rbp
1417a9386: 41 57                       	pushq	%r15
1417a9388: 41 56                       	pushq	%r14
1417a938a: 41 55                       	pushq	%r13
1417a938c: 41 54                       	pushq	%r12
1417a938e: 56                          	pushq	%rsi
1417a938f: 57                          	pushq	%rdi
1417a9390: 53                          	pushq	%rbx
1417a9391: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9395: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a939c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a93a1: 48 8b 95 a8 04 00 00        	movq	0x4a8(%rbp), %rdx
1417a93a8: 48 85 d2                    	testq	%rdx, %rdx
1417a93ab: 74 16                       	je	0x1417a93c3 <.text+0x17a83c3>
1417a93ad: 48 c1 e2 05                 	shlq	$0x5, %rdx
1417a93b1: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a93b7: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a93be: e8 5d a8 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a93c3: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a93c8: 48 83 c4 48                 	addq	$0x48, %rsp
1417a93cc: 5b                          	popq	%rbx
1417a93cd: 5f                          	popq	%rdi
1417a93ce: 5e                          	popq	%rsi
1417a93cf: 41 5c                       	popq	%r12
1417a93d1: 41 5d                       	popq	%r13
1417a93d3: 41 5e                       	popq	%r14
1417a93d5: 41 5f                       	popq	%r15
1417a93d7: 5d                          	popq	%rbp
1417a93d8: c3                          	retq
1417a93d9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1417a93e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a93e5: 55                          	pushq	%rbp
1417a93e6: 41 57                       	pushq	%r15
1417a93e8: 41 56                       	pushq	%r14
1417a93ea: 41 55                       	pushq	%r13
1417a93ec: 41 54                       	pushq	%r12
1417a93ee: 56                          	pushq	%rsi
1417a93ef: 57                          	pushq	%rdi
1417a93f0: 53                          	pushq	%rbx
1417a93f1: 48 83 ec 48                 	subq	$0x48, %rsp
1417a93f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a93fc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9401: 80 bd 65 05 00 00 00        	cmpb	$0x0, 0x565(%rbp)
1417a9408: 74 0c                       	je	0x1417a9416 <.text+0x17a8416>
1417a940a: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a9411: e8 8a f4 e3 fe              	callq	0x1405e88a0 <.text+0x5e78a0>
1417a9416: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a941b: 48 83 c4 48                 	addq	$0x48, %rsp
1417a941f: 5b                          	popq	%rbx
1417a9420: 5f                          	popq	%rdi
1417a9421: 5e                          	popq	%rsi
1417a9422: 41 5c                       	popq	%r12
1417a9424: 41 5d                       	popq	%r13
1417a9426: 41 5e                       	popq	%r14
1417a9428: 41 5f                       	popq	%r15
1417a942a: 5d                          	popq	%rbp
1417a942b: c3                          	retq
1417a942c: 0f 1f 40 00                 	nopl	(%rax)
1417a9430: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9435: 55                          	pushq	%rbp
1417a9436: 41 57                       	pushq	%r15
1417a9438: 41 56                       	pushq	%r14
1417a943a: 41 55                       	pushq	%r13
1417a943c: 41 54                       	pushq	%r12
1417a943e: 56                          	pushq	%rsi
1417a943f: 57                          	pushq	%rdi
1417a9440: 53                          	pushq	%rbx
1417a9441: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9445: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a944c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9451: 80 bd 66 05 00 00 00        	cmpb	$0x0, 0x566(%rbp)
1417a9458: 74 0c                       	je	0x1417a9466 <.text+0x17a8466>
1417a945a: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a9461: e8 3a f4 e3 fe              	callq	0x1405e88a0 <.text+0x5e78a0>
1417a9466: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a946b: 48 83 c4 48                 	addq	$0x48, %rsp
1417a946f: 5b                          	popq	%rbx
1417a9470: 5f                          	popq	%rdi
1417a9471: 5e                          	popq	%rsi
1417a9472: 41 5c                       	popq	%r12
1417a9474: 41 5d                       	popq	%r13
1417a9476: 41 5e                       	popq	%r14
1417a9478: 41 5f                       	popq	%r15
1417a947a: 5d                          	popq	%rbp
1417a947b: c3                          	retq
1417a947c: 0f 1f 40 00                 	nopl	(%rax)
1417a9480: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9485: 55                          	pushq	%rbp
1417a9486: 41 57                       	pushq	%r15
1417a9488: 41 56                       	pushq	%r14
1417a948a: 41 55                       	pushq	%r13
1417a948c: 41 54                       	pushq	%r12
1417a948e: 56                          	pushq	%rsi
1417a948f: 57                          	pushq	%rdi
1417a9490: 53                          	pushq	%rbx
1417a9491: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9495: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a949c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a94a1: 80 bd 67 05 00 00 00        	cmpb	$0x0, 0x567(%rbp)
1417a94a8: 74 0c                       	je	0x1417a94b6 <.text+0x17a84b6>
1417a94aa: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a94b1: e8 ea f3 e3 fe              	callq	0x1405e88a0 <.text+0x5e78a0>
1417a94b6: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a94bb: 48 83 c4 48                 	addq	$0x48, %rsp
1417a94bf: 5b                          	popq	%rbx
1417a94c0: 5f                          	popq	%rdi
1417a94c1: 5e                          	popq	%rsi
1417a94c2: 41 5c                       	popq	%r12
1417a94c4: 41 5d                       	popq	%r13
1417a94c6: 41 5e                       	popq	%r14
1417a94c8: 41 5f                       	popq	%r15
1417a94ca: 5d                          	popq	%rbp
1417a94cb: c3                          	retq
1417a94cc: 0f 1f 40 00                 	nopl	(%rax)
1417a94d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a94d5: 55                          	pushq	%rbp
1417a94d6: 41 57                       	pushq	%r15
1417a94d8: 41 56                       	pushq	%r14
1417a94da: 41 55                       	pushq	%r13
1417a94dc: 41 54                       	pushq	%r12
1417a94de: 56                          	pushq	%rsi
1417a94df: 57                          	pushq	%rdi
1417a94e0: 53                          	pushq	%rbx
1417a94e1: 48 83 ec 48                 	subq	$0x48, %rsp
1417a94e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a94ec: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a94f1: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a94f8: 48 85 c9                    	testq	%rcx, %rcx
1417a94fb: 0f 94 c0                    	sete	%al
1417a94fe: 08 c8                       	orb	%cl, %al
1417a9500: 75 16                       	jne	0x1417a9518 <.text+0x17a8518>
1417a9502: 48 83 bd 38 04 00 00 00     	cmpq	$0x0, 0x438(%rbp)
1417a950a: 75 0c                       	jne	0x1417a9518 <.text+0x17a8518>
1417a950c: 48 8d 8d 40 04 00 00        	leaq	0x440(%rbp), %rcx
1417a9513: e8 e8 e4 e3 fe              	callq	0x1405e7a00 <.text+0x5e6a00>
1417a9518: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a951d: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9521: 5b                          	popq	%rbx
1417a9522: 5f                          	popq	%rdi
1417a9523: 5e                          	popq	%rsi
1417a9524: 41 5c                       	popq	%r12
1417a9526: 41 5d                       	popq	%r13
1417a9528: 41 5e                       	popq	%r14
1417a952a: 41 5f                       	popq	%r15
1417a952c: 5d                          	popq	%rbp
1417a952d: c3                          	retq
1417a952e: 66 90                       	nop
1417a9530: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9535: 55                          	pushq	%rbp
1417a9536: 41 57                       	pushq	%r15
1417a9538: 41 56                       	pushq	%r14
1417a953a: 41 55                       	pushq	%r13
1417a953c: 41 54                       	pushq	%r12
1417a953e: 56                          	pushq	%rsi
1417a953f: 57                          	pushq	%rdi
1417a9540: 53                          	pushq	%rbx
1417a9541: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9545: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a954c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9551: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a9558: 48 8b 85 c0 04 00 00        	movq	0x4c0(%rbp), %rax
1417a955f: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1417a9563: 48 85 d2                    	testq	%rdx, %rdx
1417a9566: 74 10                       	je	0x1417a9578 <.text+0x17a8578>
1417a9568: 48 8b 85 c0 04 00 00        	movq	0x4c0(%rbp), %rax
1417a956f: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1417a9573: e8 a8 a6 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a9578: ba 18 00 00 00              	movl	$0x18, %edx
1417a957d: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a9583: 48 8b 8d d0 04 00 00        	movq	0x4d0(%rbp), %rcx
1417a958a: e8 91 a6 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a958f: c6 85 76 05 00 00 00        	movb	$0x0, 0x576(%rbp)
1417a9596: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a959b: 48 83 c4 48                 	addq	$0x48, %rsp
1417a959f: 5b                          	popq	%rbx
1417a95a0: 5f                          	popq	%rdi
1417a95a1: 5e                          	popq	%rsi
1417a95a2: 41 5c                       	popq	%r12
1417a95a4: 41 5d                       	popq	%r13
1417a95a6: 41 5e                       	popq	%r14
1417a95a8: 41 5f                       	popq	%r15
1417a95aa: 5d                          	popq	%rbp
1417a95ab: c3                          	retq
1417a95ac: 0f 1f 40 00                 	nopl	(%rax)
1417a95b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a95b5: 55                          	pushq	%rbp
1417a95b6: 41 57                       	pushq	%r15
1417a95b8: 41 56                       	pushq	%r14
1417a95ba: 41 55                       	pushq	%r13
1417a95bc: 41 54                       	pushq	%r12
1417a95be: 56                          	pushq	%rsi
1417a95bf: 57                          	pushq	%rdi
1417a95c0: 53                          	pushq	%rbx
1417a95c1: 48 83 ec 48                 	subq	$0x48, %rsp
1417a95c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a95cc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a95d1: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a95d8: e8 73 09 9a fe              	callq	0x140149f50 <.text+0x148f50>
1417a95dd: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a95e2: 48 83 c4 48                 	addq	$0x48, %rsp
1417a95e6: 5b                          	popq	%rbx
1417a95e7: 5f                          	popq	%rdi
1417a95e8: 5e                          	popq	%rsi
1417a95e9: 41 5c                       	popq	%r12
1417a95eb: 41 5d                       	popq	%r13
1417a95ed: 41 5e                       	popq	%r14
1417a95ef: 41 5f                       	popq	%r15
1417a95f1: 5d                          	popq	%rbp
1417a95f2: c3                          	retq
1417a95f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1417a9600: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9605: 55                          	pushq	%rbp
1417a9606: 41 57                       	pushq	%r15
1417a9608: 41 56                       	pushq	%r14
1417a960a: 41 55                       	pushq	%r13
1417a960c: 41 54                       	pushq	%r12
1417a960e: 56                          	pushq	%rsi
1417a960f: 57                          	pushq	%rdi
1417a9610: 53                          	pushq	%rbx
1417a9611: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9615: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a961c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9621: 48 83 bd 28 04 00 00 00     	cmpq	$0x0, 0x428(%rbp)
1417a9629: 74 1d                       	je	0x1417a9648 <.text+0x17a8648>
1417a962b: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
1417a9632: 48 c1 e2 03                 	shlq	$0x3, %rdx
1417a9636: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1417a963c: 48 8b 8d d0 04 00 00        	movq	0x4d0(%rbp), %rcx
1417a9643: e8 d8 a5 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a9648: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a964d: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9651: 5b                          	popq	%rbx
1417a9652: 5f                          	popq	%rdi
1417a9653: 5e                          	popq	%rsi
1417a9654: 41 5c                       	popq	%r12
1417a9656: 41 5d                       	popq	%r13
1417a9658: 41 5e                       	popq	%r14
1417a965a: 41 5f                       	popq	%r15
1417a965c: 5d                          	popq	%rbp
1417a965d: c3                          	retq
1417a965e: 66 90                       	nop
1417a9660: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9665: 55                          	pushq	%rbp
1417a9666: 41 57                       	pushq	%r15
1417a9668: 41 56                       	pushq	%r14
1417a966a: 41 55                       	pushq	%r13
1417a966c: 41 54                       	pushq	%r12
1417a966e: 56                          	pushq	%rsi
1417a966f: 57                          	pushq	%rdi
1417a9670: 53                          	pushq	%rbx
1417a9671: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9675: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a967c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9681: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1417a9688: e8 c3 08 9a fe              	callq	0x140149f50 <.text+0x148f50>
1417a968d: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9692: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9696: 5b                          	popq	%rbx
1417a9697: 5f                          	popq	%rdi
1417a9698: 5e                          	popq	%rsi
1417a9699: 41 5c                       	popq	%r12
1417a969b: 41 5d                       	popq	%r13
1417a969d: 41 5e                       	popq	%r14
1417a969f: 41 5f                       	popq	%r15
1417a96a1: 5d                          	popq	%rbp
1417a96a2: c3                          	retq
1417a96a3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1417a96b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a96b5: 55                          	pushq	%rbp
1417a96b6: 41 57                       	pushq	%r15
1417a96b8: 41 56                       	pushq	%r14
1417a96ba: 41 55                       	pushq	%r13
1417a96bc: 41 54                       	pushq	%r12
1417a96be: 56                          	pushq	%rsi
1417a96bf: 57                          	pushq	%rdi
1417a96c0: 53                          	pushq	%rbx
1417a96c1: 48 83 ec 48                 	subq	$0x48, %rsp
1417a96c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a96cc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a96d1: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
1417a96d8: e8 43 5c 0d ff              	callq	0x14087f320 <.text+0x87e320>
1417a96dd: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a96e2: 48 83 c4 48                 	addq	$0x48, %rsp
1417a96e6: 5b                          	popq	%rbx
1417a96e7: 5f                          	popq	%rdi
1417a96e8: 5e                          	popq	%rsi
1417a96e9: 41 5c                       	popq	%r12
1417a96eb: 41 5d                       	popq	%r13
1417a96ed: 41 5e                       	popq	%r14
1417a96ef: 41 5f                       	popq	%r15
1417a96f1: 5d                          	popq	%rbp
1417a96f2: c3                          	retq
1417a96f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1417a9700: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9705: 55                          	pushq	%rbp
1417a9706: 41 57                       	pushq	%r15
1417a9708: 41 56                       	pushq	%r14
1417a970a: 41 55                       	pushq	%r13
1417a970c: 41 54                       	pushq	%r12
1417a970e: 56                          	pushq	%rsi
1417a970f: 57                          	pushq	%rdi
1417a9710: 53                          	pushq	%rbx
1417a9711: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9715: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a971c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9721: 41 b8 30 00 00 00           	movl	$0x30, %r8d
1417a9727: 41 b9 10 00 00 00           	movl	$0x10, %r9d
1417a972d: 48 8d 8d e0 03 00 00        	leaq	0x3e0(%rbp), %rcx
1417a9734: 48 8d 95 00 04 00 00        	leaq	0x400(%rbp), %rdx
1417a973b: e8 30 5e 8a fe              	callq	0x14004f570 <.text+0x4e570>
1417a9740: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9745: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9749: 5b                          	popq	%rbx
1417a974a: 5f                          	popq	%rdi
1417a974b: 5e                          	popq	%rsi
1417a974c: 41 5c                       	popq	%r12
1417a974e: 41 5d                       	popq	%r13
1417a9750: 41 5e                       	popq	%r14
1417a9752: 41 5f                       	popq	%r15
1417a9754: 5d                          	popq	%rbp
1417a9755: c3                          	retq
1417a9756: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1417a9760: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9765: 55                          	pushq	%rbp
1417a9766: 41 57                       	pushq	%r15
1417a9768: 41 56                       	pushq	%r14
1417a976a: 41 55                       	pushq	%r13
1417a976c: 41 54                       	pushq	%r12
1417a976e: 56                          	pushq	%rsi
1417a976f: 57                          	pushq	%rdi
1417a9770: 53                          	pushq	%rbx
1417a9771: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9775: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a977c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9781: 48 8b 95 88 02 00 00        	movq	0x288(%rbp), %rdx
1417a9788: 48 85 d2                    	testq	%rdx, %rdx
1417a978b: 74 16                       	je	0x1417a97a3 <.text+0x17a87a3>
1417a978d: 48 8b 8d 90 02 00 00        	movq	0x290(%rbp), %rcx
1417a9794: 48 c1 e2 04                 	shlq	$0x4, %rdx
1417a9798: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a979e: e8 7d a4 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a97a3: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a97a8: 48 83 c4 48                 	addq	$0x48, %rsp
1417a97ac: 5b                          	popq	%rbx
1417a97ad: 5f                          	popq	%rdi
1417a97ae: 5e                          	popq	%rsi
1417a97af: 41 5c                       	popq	%r12
1417a97b1: 41 5d                       	popq	%r13
1417a97b3: 41 5e                       	popq	%r14
1417a97b5: 41 5f                       	popq	%r15
1417a97b7: 5d                          	popq	%rbp
1417a97b8: c3                          	retq
1417a97b9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1417a97c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a97c5: 55                          	pushq	%rbp
1417a97c6: 41 57                       	pushq	%r15
1417a97c8: 41 56                       	pushq	%r14
1417a97ca: 41 55                       	pushq	%r13
1417a97cc: 41 54                       	pushq	%r12
1417a97ce: 56                          	pushq	%rsi
1417a97cf: 57                          	pushq	%rdi
1417a97d0: 53                          	pushq	%rbx
1417a97d1: 48 83 ec 48                 	subq	$0x48, %rsp
1417a97d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a97dc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a97e1: 48 83 bd d0 03 00 00 00     	cmpq	$0x0, 0x3d0(%rbp)
1417a97e9: 74 1d                       	je	0x1417a9808 <.text+0x17a8808>
1417a97eb: 48 8b 95 d0 03 00 00        	movq	0x3d0(%rbp), %rdx
1417a97f2: 48 c1 e2 04                 	shlq	$0x4, %rdx
1417a97f6: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a97fc: 48 8b 8d 78 03 00 00        	movq	0x378(%rbp), %rcx
1417a9803: e8 18 a4 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a9808: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a980d: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9811: 5b                          	popq	%rbx
1417a9812: 5f                          	popq	%rdi
1417a9813: 5e                          	popq	%rsi
1417a9814: 41 5c                       	popq	%r12
1417a9816: 41 5d                       	popq	%r13
1417a9818: 41 5e                       	popq	%r14
1417a981a: 41 5f                       	popq	%r15
1417a981c: 5d                          	popq	%rbp
1417a981d: c3                          	retq
1417a981e: 66 90                       	nop
1417a9820: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9825: 55                          	pushq	%rbp
1417a9826: 41 57                       	pushq	%r15
1417a9828: 41 56                       	pushq	%r14
1417a982a: 41 55                       	pushq	%r13
1417a982c: 41 54                       	pushq	%r12
1417a982e: 56                          	pushq	%rsi
1417a982f: 57                          	pushq	%rdi
1417a9830: 53                          	pushq	%rbx
1417a9831: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9835: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a983c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9841: 48 83 bd c8 03 00 00 00     	cmpq	$0x0, 0x3c8(%rbp)
1417a9849: 74 1d                       	je	0x1417a9868 <.text+0x17a8868>
1417a984b: 48 8b 95 c8 03 00 00        	movq	0x3c8(%rbp), %rdx
1417a9852: 48 c1 e2 04                 	shlq	$0x4, %rdx
1417a9856: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a985c: 48 8b 8d 70 03 00 00        	movq	0x370(%rbp), %rcx
1417a9863: e8 b8 a3 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a9868: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a986d: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9871: 5b                          	popq	%rbx
1417a9872: 5f                          	popq	%rdi
1417a9873: 5e                          	popq	%rsi
1417a9874: 41 5c                       	popq	%r12
1417a9876: 41 5d                       	popq	%r13
1417a9878: 41 5e                       	popq	%r14
1417a987a: 41 5f                       	popq	%r15
1417a987c: 5d                          	popq	%rbp
1417a987d: c3                          	retq
1417a987e: 66 90                       	nop
1417a9880: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9885: 55                          	pushq	%rbp
1417a9886: 41 57                       	pushq	%r15
1417a9888: 41 56                       	pushq	%r14
1417a988a: 41 55                       	pushq	%r13
1417a988c: 41 54                       	pushq	%r12
1417a988e: 56                          	pushq	%rsi
1417a988f: 57                          	pushq	%rdi
1417a9890: 53                          	pushq	%rbx
1417a9891: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9895: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a989c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a98a1: 48 8d 8d b8 02 00 00        	leaq	0x2b8(%rbp), %rcx
1417a98a8: e8 f3 8c 0d ff              	callq	0x1408825a0 <.text+0x8815a0>
1417a98ad: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a98b2: 48 83 c4 48                 	addq	$0x48, %rsp
1417a98b6: 5b                          	popq	%rbx
1417a98b7: 5f                          	popq	%rdi
1417a98b8: 5e                          	popq	%rsi
1417a98b9: 41 5c                       	popq	%r12
1417a98bb: 41 5d                       	popq	%r13
1417a98bd: 41 5e                       	popq	%r14
1417a98bf: 41 5f                       	popq	%r15
1417a98c1: 5d                          	popq	%rbp
1417a98c2: c3                          	retq
1417a98c3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1417a98d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a98d5: 55                          	pushq	%rbp
1417a98d6: 41 57                       	pushq	%r15
1417a98d8: 41 56                       	pushq	%r14
1417a98da: 41 55                       	pushq	%r13
1417a98dc: 41 54                       	pushq	%r12
1417a98de: 56                          	pushq	%rsi
1417a98df: 57                          	pushq	%rdi
1417a98e0: 53                          	pushq	%rbx
1417a98e1: 48 83 ec 48                 	subq	$0x48, %rsp
1417a98e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a98ec: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a98f1: 48 83 bd c0 03 00 00 00     	cmpq	$0x0, 0x3c0(%rbp)
1417a98f9: 74 1a                       	je	0x1417a9915 <.text+0x17a8915>
1417a98fb: 48 6b 95 c0 03 00 00 58     	imulq	$0x58, 0x3c0(%rbp), %rdx
1417a9903: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1417a9909: 48 8b 8d 68 03 00 00        	movq	0x368(%rbp), %rcx
1417a9910: e8 0b a3 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a9915: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a991a: 48 83 c4 48                 	addq	$0x48, %rsp
1417a991e: 5b                          	popq	%rbx
1417a991f: 5f                          	popq	%rdi
1417a9920: 5e                          	popq	%rsi
1417a9921: 41 5c                       	popq	%r12
1417a9923: 41 5d                       	popq	%r13
1417a9925: 41 5e                       	popq	%r14
1417a9927: 41 5f                       	popq	%r15
1417a9929: 5d                          	popq	%rbp
1417a992a: c3                          	retq
1417a992b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1417a9930: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9935: 55                          	pushq	%rbp
1417a9936: 41 57                       	pushq	%r15
1417a9938: 41 56                       	pushq	%r14
1417a993a: 41 55                       	pushq	%r13
1417a993c: 41 54                       	pushq	%r12
1417a993e: 56                          	pushq	%rsi
1417a993f: 57                          	pushq	%rdi
1417a9940: 53                          	pushq	%rbx
1417a9941: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9945: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a994c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9951: 48 8d 8d d0 02 00 00        	leaq	0x2d0(%rbp), %rcx
1417a9958: e8 f3 5d 0d ff              	callq	0x14087f750 <.text+0x87e750>
1417a995d: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9962: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9966: 5b                          	popq	%rbx
1417a9967: 5f                          	popq	%rdi
1417a9968: 5e                          	popq	%rsi
1417a9969: 41 5c                       	popq	%r12
1417a996b: 41 5d                       	popq	%r13
1417a996d: 41 5e                       	popq	%r14
1417a996f: 41 5f                       	popq	%r15
1417a9971: 5d                          	popq	%rbp
1417a9972: c3                          	retq
1417a9973: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1417a9980: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9985: 55                          	pushq	%rbp
1417a9986: 41 57                       	pushq	%r15
1417a9988: 41 56                       	pushq	%r14
1417a998a: 41 55                       	pushq	%r13
1417a998c: 41 54                       	pushq	%r12
1417a998e: 56                          	pushq	%rsi
1417a998f: 57                          	pushq	%rdi
1417a9990: 53                          	pushq	%rbx
1417a9991: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9995: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a999c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a99a1: 48 8d 8d a0 04 00 00        	leaq	0x4a0(%rbp), %rcx
1417a99a8: e8 e3 40 0b ff              	callq	0x14085da90 <.text+0x85ca90>
1417a99ad: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a99b2: 48 83 c4 48                 	addq	$0x48, %rsp
1417a99b6: 5b                          	popq	%rbx
1417a99b7: 5f                          	popq	%rdi
1417a99b8: 5e                          	popq	%rsi
1417a99b9: 41 5c                       	popq	%r12
1417a99bb: 41 5d                       	popq	%r13
1417a99bd: 41 5e                       	popq	%r14
1417a99bf: 41 5f                       	popq	%r15
1417a99c1: 5d                          	popq	%rbp
1417a99c2: c3                          	retq
1417a99c3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1417a99d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a99d5: 55                          	pushq	%rbp
1417a99d6: 41 57                       	pushq	%r15
1417a99d8: 41 56                       	pushq	%r14
1417a99da: 41 55                       	pushq	%r13
1417a99dc: 41 54                       	pushq	%r12
1417a99de: 56                          	pushq	%rsi
1417a99df: 57                          	pushq	%rdi
1417a99e0: 53                          	pushq	%rbx
1417a99e1: 48 83 ec 48                 	subq	$0x48, %rsp
1417a99e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a99ec: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a99f1: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a99f8: ff 15 f2 89 9e 01           	callq	*0x19e89f2(%rip)        # 0x1431923f0
1417a99fe: c6 85 77 05 00 00 01        	movb	$0x1, 0x577(%rbp)
1417a9a05: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9a0a: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9a0e: 5b                          	popq	%rbx
1417a9a0f: 5f                          	popq	%rdi
1417a9a10: 5e                          	popq	%rsi
1417a9a11: 41 5c                       	popq	%r12
1417a9a13: 41 5d                       	popq	%r13
1417a9a15: 41 5e                       	popq	%r14
1417a9a17: 41 5f                       	popq	%r15
1417a9a19: 5d                          	popq	%rbp
1417a9a1a: c3                          	retq
1417a9a1b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1417a9a20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9a25: 55                          	pushq	%rbp
1417a9a26: 41 57                       	pushq	%r15
1417a9a28: 41 56                       	pushq	%r14
1417a9a2a: 41 55                       	pushq	%r13
1417a9a2c: 41 54                       	pushq	%r12
1417a9a2e: 56                          	pushq	%rsi
1417a9a2f: 57                          	pushq	%rdi
1417a9a30: 53                          	pushq	%rbx
1417a9a31: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9a35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a9a3c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9a41: c6 85 77 05 00 00 01        	movb	$0x1, 0x577(%rbp)
1417a9a48: 48 8d 8d d8 01 00 00        	leaq	0x1d8(%rbp), %rcx
1417a9a4f: e8 1c 2e ee ff              	callq	0x14168c870 <.text+0x168b870>
1417a9a54: c6 85 77 05 00 00 01        	movb	$0x1, 0x577(%rbp)
1417a9a5b: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9a60: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9a64: 5b                          	popq	%rbx
1417a9a65: 5f                          	popq	%rdi
1417a9a66: 5e                          	popq	%rsi
1417a9a67: 41 5c                       	popq	%r12
1417a9a69: 41 5d                       	popq	%r13
1417a9a6b: 41 5e                       	popq	%r14
1417a9a6d: 41 5f                       	popq	%r15
1417a9a6f: 5d                          	popq	%rbp
1417a9a70: c3                          	retq
1417a9a71: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1417a9a80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9a85: 55                          	pushq	%rbp
1417a9a86: 41 57                       	pushq	%r15
1417a9a88: 41 56                       	pushq	%r14
1417a9a8a: 41 55                       	pushq	%r13
1417a9a8c: 41 54                       	pushq	%r12
1417a9a8e: 56                          	pushq	%rsi
1417a9a8f: 57                          	pushq	%rdi
1417a9a90: 53                          	pushq	%rbx
1417a9a91: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9a95: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a9a9c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9aa1: 48 8b 95 d8 03 00 00        	movq	0x3d8(%rbp), %rdx
1417a9aa8: 48 85 d2                    	testq	%rdx, %rdx
1417a9aab: 74 12                       	je	0x1417a9abf <.text+0x17a8abf>
1417a9aad: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1417a9ab3: 48 8b 8d 68 05 00 00        	movq	0x568(%rbp), %rcx
1417a9aba: e8 61 a1 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a9abf: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9ac4: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9ac8: 5b                          	popq	%rbx
1417a9ac9: 5f                          	popq	%rdi
1417a9aca: 5e                          	popq	%rsi
1417a9acb: 41 5c                       	popq	%r12
1417a9acd: 41 5d                       	popq	%r13
1417a9acf: 41 5e                       	popq	%r14
1417a9ad1: 41 5f                       	popq	%r15
1417a9ad3: 5d                          	popq	%rbp
1417a9ad4: c3                          	retq
1417a9ad5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1417a9ae0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9ae5: 55                          	pushq	%rbp
1417a9ae6: 41 57                       	pushq	%r15
1417a9ae8: 41 56                       	pushq	%r14
1417a9aea: 41 55                       	pushq	%r13
1417a9aec: 41 54                       	pushq	%r12
1417a9aee: 56                          	pushq	%rsi
1417a9aef: 57                          	pushq	%rdi
1417a9af0: 53                          	pushq	%rbx
1417a9af1: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9af5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a9afc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9b01: 48 8b 95 e0 04 00 00        	movq	0x4e0(%rbp), %rdx
1417a9b08: 48 85 d2                    	testq	%rdx, %rdx
1417a9b0b: 74 12                       	je	0x1417a9b1f <.text+0x17a8b1f>
1417a9b0d: 48 8b 8d e8 04 00 00        	movq	0x4e8(%rbp), %rcx
1417a9b14: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1417a9b1a: e8 01 a1 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a9b1f: c6 85 76 05 00 00 00        	movb	$0x0, 0x576(%rbp)
1417a9b26: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9b2b: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9b2f: 5b                          	popq	%rbx
1417a9b30: 5f                          	popq	%rdi
1417a9b31: 5e                          	popq	%rsi
1417a9b32: 41 5c                       	popq	%r12
1417a9b34: 41 5d                       	popq	%r13
1417a9b36: 41 5e                       	popq	%r14
1417a9b38: 41 5f                       	popq	%r15
1417a9b3a: 5d                          	popq	%rbp
1417a9b3b: c3                          	retq
1417a9b3c: 0f 1f 40 00                 	nopl	(%rax)
1417a9b40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9b45: 55                          	pushq	%rbp
1417a9b46: 41 57                       	pushq	%r15
1417a9b48: 41 56                       	pushq	%r14
1417a9b4a: 41 55                       	pushq	%r13
1417a9b4c: 41 54                       	pushq	%r12
1417a9b4e: 56                          	pushq	%rsi
1417a9b4f: 57                          	pushq	%rdi
1417a9b50: 53                          	pushq	%rbx
1417a9b51: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9b55: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a9b5c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9b61: 48 8b 95 d8 03 00 00        	movq	0x3d8(%rbp), %rdx
1417a9b68: 48 85 d2                    	testq	%rdx, %rdx
1417a9b6b: 74 12                       	je	0x1417a9b7f <.text+0x17a8b7f>
1417a9b6d: 48 8b 8d e0 03 00 00        	movq	0x3e0(%rbp), %rcx
1417a9b74: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1417a9b7a: e8 a1 a0 e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a9b7f: c6 85 76 05 00 00 01        	movb	$0x1, 0x576(%rbp)
1417a9b86: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9b8b: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9b8f: 5b                          	popq	%rbx
1417a9b90: 5f                          	popq	%rdi
1417a9b91: 5e                          	popq	%rsi
1417a9b92: 41 5c                       	popq	%r12
1417a9b94: 41 5d                       	popq	%r13
1417a9b96: 41 5e                       	popq	%r14
1417a9b98: 41 5f                       	popq	%r15
1417a9b9a: 5d                          	popq	%rbp
1417a9b9b: c3                          	retq
1417a9b9c: 0f 1f 40 00                 	nopl	(%rax)
1417a9ba0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9ba5: 55                          	pushq	%rbp
1417a9ba6: 41 57                       	pushq	%r15
1417a9ba8: 41 56                       	pushq	%r14
1417a9baa: 41 55                       	pushq	%r13
1417a9bac: 41 54                       	pushq	%r12
1417a9bae: 56                          	pushq	%rsi
1417a9baf: 57                          	pushq	%rdi
1417a9bb0: 53                          	pushq	%rbx
1417a9bb1: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9bb5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a9bbc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9bc1: 0f b6 9d 76 05 00 00        	movzbl	0x576(%rbp), %ebx
1417a9bc8: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1417a9bcf: e8 5c 30 5a ff              	callq	0x140d4cc30 <.text+0xd4bc30>
1417a9bd4: 88 9d 77 05 00 00           	movb	%bl, 0x577(%rbp)
1417a9bda: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9bdf: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9be3: 5b                          	popq	%rbx
1417a9be4: 5f                          	popq	%rdi
1417a9be5: 5e                          	popq	%rsi
1417a9be6: 41 5c                       	popq	%r12
1417a9be8: 41 5d                       	popq	%r13
1417a9bea: 41 5e                       	popq	%r14
1417a9bec: 41 5f                       	popq	%r15
1417a9bee: 5d                          	popq	%rbp
1417a9bef: c3                          	retq
1417a9bf0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1417a9bf5: 55                          	pushq	%rbp
1417a9bf6: 41 57                       	pushq	%r15
1417a9bf8: 41 56                       	pushq	%r14
1417a9bfa: 41 55                       	pushq	%r13
1417a9bfc: 41 54                       	pushq	%r12
1417a9bfe: 56                          	pushq	%rsi
1417a9bff: 57                          	pushq	%rdi
1417a9c00: 53                          	pushq	%rbx
1417a9c01: 48 83 ec 48                 	subq	$0x48, %rsp
1417a9c05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1417a9c0c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1417a9c11: 80 bd 77 05 00 00 00        	cmpb	$0x0, 0x577(%rbp)
1417a9c18: 74 1e                       	je	0x1417a9c38 <.text+0x17a8c38>
1417a9c1a: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
1417a9c21: 48 85 d2                    	testq	%rdx, %rdx
1417a9c24: 74 12                       	je	0x1417a9c38 <.text+0x17a8c38>
1417a9c26: 48 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %rcx
1417a9c2d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1417a9c33: e8 e8 9f e6 fe              	callq	0x140613c20 <.text+0x612c20>
1417a9c38: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1417a9c3d: 48 83 c4 48                 	addq	$0x48, %rsp
1417a9c41: 5b                          	popq	%rbx
1417a9c42: 5f                          	popq	%rdi
1417a9c43: 5e                          	popq	%rsi
1417a9c44: 41 5c                       	popq	%r12
1417a9c46: 41 5d                       	popq	%r13
1417a9c48: 41 5e                       	popq	%r14
1417a9c4a: 41 5f                       	popq	%r15
1417a9c4c: 5d                          	popq	%rbp
1417a9c4d: c3                          	retq

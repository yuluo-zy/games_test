
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f7fc80: 55                          	pushq	%rbp
140f7fc81: 56                          	pushq	%rsi
140f7fc82: 57                          	pushq	%rdi
140f7fc83: 48 81 ec b0 03 00 00        	subq	$0x3b0, %rsp            # imm = 0x3B0
140f7fc8a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140f7fc92: 48 c7 85 28 03 00 00 fe ff ff ff    	movq	$-0x2, 0x328(%rbp)
140f7fc9d: 48 89 d7                    	movq	%rdx, %rdi
140f7fca0: 48 89 ce                    	movq	%rcx, %rsi
140f7fca3: 0f 10 42 60                 	movups	0x60(%rdx), %xmm0
140f7fca7: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
140f7fcab: 0f 10 42 50                 	movups	0x50(%rdx), %xmm0
140f7fcaf: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
140f7fcb3: 0f 10 42 40                 	movups	0x40(%rdx), %xmm0
140f7fcb7: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
140f7fcbb: 0f 10 02                    	movups	(%rdx), %xmm0
140f7fcbe: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
140f7fcc2: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
140f7fcc6: 0f 10 5a 30                 	movups	0x30(%rdx), %xmm3
140f7fcca: 0f 29 5d 40                 	movaps	%xmm3, 0x40(%rbp)
140f7fcce: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
140f7fcd2: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
140f7fcd6: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
140f7fcda: 0f b6 05 28 4f 6b 02        	movzbl	0x26b4f28(%rip), %eax   # 0x143634c09
140f7fce1: b9 10 03 00 00              	movl	$0x310, %ecx            # imm = 0x310
140f7fce6: ba 08 00 00 00              	movl	$0x8, %edx
140f7fceb: e8 20 3f 69 ff              	callq	0x140613c10 <.text+0x612c10>
140f7fcf0: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
140f7fcf7: 48 85 c0                    	testq	%rax, %rax
140f7fcfa: 0f 84 5e 03 00 00           	je	0x140f8005e <.text+0xf7f05e>
140f7fd00: 0f 10 47 60                 	movups	0x60(%rdi), %xmm0
140f7fd04: 0f 29 85 10 03 00 00        	movaps	%xmm0, 0x310(%rbp)
140f7fd0b: 0f 10 47 50                 	movups	0x50(%rdi), %xmm0
140f7fd0f: 0f 29 85 00 03 00 00        	movaps	%xmm0, 0x300(%rbp)
140f7fd16: 0f 10 47 40                 	movups	0x40(%rdi), %xmm0
140f7fd1a: 0f 29 85 f0 02 00 00        	movaps	%xmm0, 0x2f0(%rbp)
140f7fd21: 0f 10 07                    	movups	(%rdi), %xmm0
140f7fd24: 0f 10 4f 10                 	movups	0x10(%rdi), %xmm1
140f7fd28: 0f 10 57 20                 	movups	0x20(%rdi), %xmm2
140f7fd2c: 0f 10 5f 30                 	movups	0x30(%rdi), %xmm3
140f7fd30: 0f 29 9d e0 02 00 00        	movaps	%xmm3, 0x2e0(%rbp)
140f7fd37: 0f 29 95 d0 02 00 00        	movaps	%xmm2, 0x2d0(%rbp)
140f7fd3e: 0f 29 8d c0 02 00 00        	movaps	%xmm1, 0x2c0(%rbp)
140f7fd45: 0f 29 85 b0 02 00 00        	movaps	%xmm0, 0x2b0(%rbp)
140f7fd4c: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
140f7fd53: e8 e8 16 e1 00              	callq	0x141d91440 <.text+0x1d90440>
140f7fd58: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
140f7fd5f: e8 2c c8 e0 00              	callq	0x141d8c590 <.text+0x1d8b590>
140f7fd64: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
140f7fd6b: e8 50 fe e0 00              	callq	0x141d8fbc0 <.text+0x1d8ebc0>
140f7fd70: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140f7fd77: e8 74 08 e3 00              	callq	0x141db05f0 <.text+0x1daf5f0>
140f7fd7c: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140f7fd83: e8 18 0a e3 00              	callq	0x141db07a0 <.text+0x1daf7a0>
140f7fd88: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140f7fd8c: e8 df ff e0 00              	callq	0x141d8fd70 <.text+0x1d8ed70>
140f7fd91: 0f 28 85 10 03 00 00        	movaps	0x310(%rbp), %xmm0
140f7fd98: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
140f7fd9f: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140f7fda3: 0f 28 85 00 03 00 00        	movaps	0x300(%rbp), %xmm0
140f7fdaa: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140f7fdae: 0f 28 85 f0 02 00 00        	movaps	0x2f0(%rbp), %xmm0
140f7fdb5: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140f7fdb9: 0f 28 85 b0 02 00 00        	movaps	0x2b0(%rbp), %xmm0
140f7fdc0: 0f 28 8d c0 02 00 00        	movaps	0x2c0(%rbp), %xmm1
140f7fdc7: 0f 28 95 d0 02 00 00        	movaps	0x2d0(%rbp), %xmm2
140f7fdce: 0f 28 9d e0 02 00 00        	movaps	0x2e0(%rbp), %xmm3
140f7fdd5: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140f7fdd9: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140f7fddd: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140f7fde1: 0f 11 00                    	movups	%xmm0, (%rax)
140f7fde4: 0f 10 85 40 02 00 00        	movups	0x240(%rbp), %xmm0
140f7fdeb: 0f 10 8d 50 02 00 00        	movups	0x250(%rbp), %xmm1
140f7fdf2: 0f 10 95 60 02 00 00        	movups	0x260(%rbp), %xmm2
140f7fdf9: 0f 10 9d 70 02 00 00        	movups	0x270(%rbp), %xmm3
140f7fe00: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140f7fe04: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140f7fe0b: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140f7fe12: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140f7fe19: 0f 10 85 80 02 00 00        	movups	0x280(%rbp), %xmm0
140f7fe20: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140f7fe27: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
140f7fe2e: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140f7fe35: 0f 10 85 a0 02 00 00        	movups	0x2a0(%rbp), %xmm0
140f7fe3c: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140f7fe43: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
140f7fe4a: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
140f7fe51: 0f 10 85 20 02 00 00        	movups	0x220(%rbp), %xmm0
140f7fe58: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
140f7fe5f: 0f 10 85 10 02 00 00        	movups	0x210(%rbp), %xmm0
140f7fe66: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
140f7fe6d: 0f 10 85 d0 01 00 00        	movups	0x1d0(%rbp), %xmm0
140f7fe74: 0f 10 8d e0 01 00 00        	movups	0x1e0(%rbp), %xmm1
140f7fe7b: 0f 10 95 f0 01 00 00        	movups	0x1f0(%rbp), %xmm2
140f7fe82: 0f 10 9d 00 02 00 00        	movups	0x200(%rbp), %xmm3
140f7fe89: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
140f7fe90: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
140f7fe97: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
140f7fe9e: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
140f7fea5: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
140f7feac: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
140f7feb3: 0f 10 95 80 01 00 00        	movups	0x180(%rbp), %xmm2
140f7feba: 0f 10 9d 90 01 00 00        	movups	0x190(%rbp), %xmm3
140f7fec1: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
140f7fec8: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
140f7fecf: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
140f7fed6: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
140f7fedd: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
140f7fee4: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
140f7feeb: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140f7fef2: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
140f7fef9: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
140f7ff00: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
140f7ff07: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140f7ff0e: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
140f7ff15: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140f7ff1c: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
140f7ff23: 0f 11 80 c0 01 00 00        	movups	%xmm0, 0x1c0(%rax)
140f7ff2a: 0f 11 88 d0 01 00 00        	movups	%xmm1, 0x1d0(%rax)
140f7ff31: 0f 11 90 e0 01 00 00        	movups	%xmm2, 0x1e0(%rax)
140f7ff38: 0f 11 98 f0 01 00 00        	movups	%xmm3, 0x1f0(%rax)
140f7ff3f: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140f7ff46: 0f 11 80 00 02 00 00        	movups	%xmm0, 0x200(%rax)
140f7ff4d: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
140f7ff54: 0f 11 80 20 02 00 00        	movups	%xmm0, 0x220(%rax)
140f7ff5b: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
140f7ff62: 0f 11 80 10 02 00 00        	movups	%xmm0, 0x210(%rax)
140f7ff69: 0f 10 85 e0 00 00 00        	movups	0xe0(%rbp), %xmm0
140f7ff70: 0f 11 80 90 02 00 00        	movups	%xmm0, 0x290(%rax)
140f7ff77: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
140f7ff7e: 0f 11 80 80 02 00 00        	movups	%xmm0, 0x280(%rax)
140f7ff85: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
140f7ff8c: 0f 11 80 70 02 00 00        	movups	%xmm0, 0x270(%rax)
140f7ff93: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
140f7ff9a: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
140f7ffa1: 0f 10 95 a0 00 00 00        	movups	0xa0(%rbp), %xmm2
140f7ffa8: 0f 10 9d b0 00 00 00        	movups	0xb0(%rbp), %xmm3
140f7ffaf: 0f 11 98 60 02 00 00        	movups	%xmm3, 0x260(%rax)
140f7ffb6: 0f 11 90 50 02 00 00        	movups	%xmm2, 0x250(%rax)
140f7ffbd: 0f 11 88 40 02 00 00        	movups	%xmm1, 0x240(%rax)
140f7ffc4: 0f 11 80 30 02 00 00        	movups	%xmm0, 0x230(%rax)
140f7ffcb: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140f7ffcf: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140f7ffd3: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
140f7ffd7: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
140f7ffdb: 0f 11 80 a0 02 00 00        	movups	%xmm0, 0x2a0(%rax)
140f7ffe2: 0f 11 88 b0 02 00 00        	movups	%xmm1, 0x2b0(%rax)
140f7ffe9: 0f 11 90 c0 02 00 00        	movups	%xmm2, 0x2c0(%rax)
140f7fff0: 0f 11 98 d0 02 00 00        	movups	%xmm3, 0x2d0(%rax)
140f7fff7: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140f7fffb: 0f 11 80 e0 02 00 00        	movups	%xmm0, 0x2e0(%rax)
140f80002: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140f80006: 0f 11 80 f0 02 00 00        	movups	%xmm0, 0x2f0(%rax)
140f8000d: 0f 10 45 00                 	movups	(%rbp), %xmm0
140f80011: 0f 11 80 00 03 00 00        	movups	%xmm0, 0x300(%rax)
140f80018: 48 c7 46 08 07 00 00 00     	movq	$0x7, 0x8(%rsi)
140f80020: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
140f80024: 48 c7 46 18 07 00 00 00     	movq	$0x7, 0x18(%rsi)
140f8002c: 48 c7 46 20 00 00 00 00     	movq	$0x0, 0x20(%rsi)
140f80034: 48 c7 46 28 08 00 00 00     	movq	$0x8, 0x28(%rsi)
140f8003c: 0f 57 c0                    	xorps	%xmm0, %xmm0
140f8003f: 0f 11 46 30                 	movups	%xmm0, 0x30(%rsi)
140f80043: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140f8004d: 48 89 06                    	movq	%rax, (%rsi)
140f80050: 48 89 f0                    	movq	%rsi, %rax
140f80053: 48 81 c4 b0 03 00 00        	addq	$0x3b0, %rsp            # imm = 0x3B0
140f8005a: 5f                          	popq	%rdi
140f8005b: 5e                          	popq	%rsi
140f8005c: 5d                          	popq	%rbp
140f8005d: c3                          	retq
140f8005e: b9 08 00 00 00              	movl	$0x8, %ecx
140f80063: ba 10 03 00 00              	movl	$0x310, %edx            # imm = 0x310
140f80068: e8 93 8f 95 01              	callq	0x1428d9000 <.text+0x28d8000>
140f8006d: 0f 0b                       	ud2
140f8006f: 90                          	nop
140f80070: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f80075: 55                          	pushq	%rbp
140f80076: 56                          	pushq	%rsi
140f80077: 57                          	pushq	%rdi
140f80078: 48 83 ec 20                 	subq	$0x20, %rsp
140f8007c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f80083: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140f8008a: e8 f1 97 0a ff              	callq	0x140029880 <.text+0x28880>
140f8008f: ba 10 03 00 00              	movl	$0x310, %edx            # imm = 0x310
140f80094: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140f8009a: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
140f800a1: e8 7a 3b 69 ff              	callq	0x140613c20 <.text+0x612c20>
140f800a6: 90                          	nop
140f800a7: 48 83 c4 20                 	addq	$0x20, %rsp
140f800ab: 5f                          	popq	%rdi
140f800ac: 5e                          	popq	%rsi
140f800ad: 5d                          	popq	%rbp
140f800ae: c3                          	retq
140f800af: 90                          	nop
140f800b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f800b5: 55                          	pushq	%rbp
140f800b6: 56                          	pushq	%rsi
140f800b7: 57                          	pushq	%rdi
140f800b8: 48 83 ec 20                 	subq	$0x20, %rsp
140f800bc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f800c3: 48 83 bd 20 03 00 00 00     	cmpq	$0x0, 0x320(%rbp)
140f800cb: 74 08                       	je	0x140f800d5 <.text+0xf7f0d5>
140f800cd: 48 83 c4 20                 	addq	$0x20, %rsp
140f800d1: 5f                          	popq	%rdi
140f800d2: 5e                          	popq	%rsi
140f800d3: 5d                          	popq	%rbp
140f800d4: c3                          	retq
140f800d5: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140f800d9: e8 a2 97 0a ff              	callq	0x140029880 <.text+0x28880>
140f800de: eb ed                       	jmp	0x140f800cd <.text+0xf7f0cd>
140f800e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f800e5: 55                          	pushq	%rbp
140f800e6: 56                          	pushq	%rsi
140f800e7: 57                          	pushq	%rdi
140f800e8: 48 83 ec 20                 	subq	$0x20, %rsp
140f800ec: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f800f3: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
140f800fa: e8 81 97 0a ff              	callq	0x140029880 <.text+0x28880>
140f800ff: 90                          	nop
140f80100: 48 83 c4 20                 	addq	$0x20, %rsp
140f80104: 5f                          	popq	%rdi
140f80105: 5e                          	popq	%rsi
140f80106: 5d                          	popq	%rbp
140f80107: c3                          	retq
140f80108: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140f80110: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f80115: 55                          	pushq	%rbp
140f80116: 56                          	pushq	%rsi
140f80117: 57                          	pushq	%rdi
140f80118: 48 83 ec 20                 	subq	$0x20, %rsp
140f8011c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f80123: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
140f8012a: e8 51 97 0a ff              	callq	0x140029880 <.text+0x28880>
140f8012f: 90                          	nop
140f80130: 48 83 c4 20                 	addq	$0x20, %rsp
140f80134: 5f                          	popq	%rdi
140f80135: 5e                          	popq	%rsi
140f80136: 5d                          	popq	%rbp
140f80137: c3                          	retq
140f80138: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140f80140: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f80145: 55                          	pushq	%rbp
140f80146: 56                          	pushq	%rsi
140f80147: 57                          	pushq	%rdi
140f80148: 48 83 ec 20                 	subq	$0x20, %rsp
140f8014c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f80153: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
140f8015a: e8 21 97 0a ff              	callq	0x140029880 <.text+0x28880>
140f8015f: 90                          	nop
140f80160: 48 83 c4 20                 	addq	$0x20, %rsp
140f80164: 5f                          	popq	%rdi
140f80165: 5e                          	popq	%rsi
140f80166: 5d                          	popq	%rbp
140f80167: c3                          	retq
140f80168: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140f80170: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f80175: 55                          	pushq	%rbp
140f80176: 56                          	pushq	%rsi
140f80177: 57                          	pushq	%rdi
140f80178: 48 83 ec 20                 	subq	$0x20, %rsp
140f8017c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f80183: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140f8018a: e8 f1 96 0a ff              	callq	0x140029880 <.text+0x28880>
140f8018f: 90                          	nop
140f80190: 48 83 c4 20                 	addq	$0x20, %rsp
140f80194: 5f                          	popq	%rdi
140f80195: 5e                          	popq	%rsi
140f80196: 5d                          	popq	%rbp
140f80197: c3                          	retq
140f80198: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140f801a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f801a5: 55                          	pushq	%rbp
140f801a6: 56                          	pushq	%rsi
140f801a7: 57                          	pushq	%rdi
140f801a8: 48 83 ec 20                 	subq	$0x20, %rsp
140f801ac: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f801b3: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140f801ba: e8 c1 96 0a ff              	callq	0x140029880 <.text+0x28880>
140f801bf: 90                          	nop
140f801c0: 48 83 c4 20                 	addq	$0x20, %rsp
140f801c4: 5f                          	popq	%rdi
140f801c5: 5e                          	popq	%rsi
140f801c6: 5d                          	popq	%rbp
140f801c7: c3                          	retq

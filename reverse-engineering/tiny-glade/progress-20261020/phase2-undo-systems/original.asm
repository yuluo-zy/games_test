
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1423448a0: 41 57                       	pushq	%r15
1423448a2: 41 56                       	pushq	%r14
1423448a4: 41 54                       	pushq	%r12
1423448a6: 56                          	pushq	%rsi
1423448a7: 57                          	pushq	%rdi
1423448a8: 55                          	pushq	%rbp
1423448a9: 53                          	pushq	%rbx
1423448aa: 48 83 ec 20                 	subq	$0x20, %rsp
1423448ae: 49 8b 00                    	movq	(%r8), %rax
1423448b1: 44 8b 30                    	movl	(%rax), %r14d
1423448b4: 8b 68 04                    	movl	0x4(%rax), %ebp
1423448b7: 8b 78 08                    	movl	0x8(%rax), %edi
1423448ba: 8b 70 0c                    	movl	0xc(%rax), %esi
1423448bd: 48 8b 01                    	movq	(%rcx), %rax
1423448c0: 0f b6 80 f0 00 00 00        	movzbl	0xf0(%rax), %eax
1423448c7: 44 8b 7a 1c                 	movl	0x1c(%rdx), %r15d
1423448cb: 4c 8b 62 10                 	movq	0x10(%rdx), %r12
1423448cf: 45 89 3c 24                 	movl	%r15d, (%r12)
1423448d3: 48 8b 1a                    	movq	(%rdx), %rbx
1423448d6: 48 8d 0d 1b 9e d0 00        	leaq	0xd09e1b(%rip), %rcx    # 0x14304e6f8
1423448dd: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1423448e1: 48 01 c8                    	addq	%rcx, %rax
1423448e4: ff e0                       	jmpq	*%rax
1423448e6: 48 89 d9                    	movq	%rbx, %rcx
1423448e9: 44 89 f2                    	movl	%r14d, %edx
1423448ec: 41 89 e8                    	movl	%ebp, %r8d
1423448ef: e8 5c 17 03 00              	callq	0x142376050 <.text+0x2375050>
1423448f4: 45 89 3c 24                 	movl	%r15d, (%r12)
1423448f8: 48 89 d9                    	movq	%rbx, %rcx
1423448fb: 44 89 f2                    	movl	%r14d, %edx
1423448fe: 41 89 e8                    	movl	%ebp, %r8d
142344901: e8 fa 19 03 00              	callq	0x142376300 <.text+0x2375300>
142344906: 45 89 3c 24                 	movl	%r15d, (%r12)
14234490a: 48 89 d9                    	movq	%rbx, %rcx
14234490d: 89 fa                       	movl	%edi, %edx
14234490f: 41 89 f0                    	movl	%esi, %r8d
142344912: 48 83 c4 20                 	addq	$0x20, %rsp
142344916: 5b                          	popq	%rbx
142344917: 5d                          	popq	%rbp
142344918: 5f                          	popq	%rdi
142344919: 5e                          	popq	%rsi
14234491a: 41 5c                       	popq	%r12
14234491c: 41 5e                       	popq	%r14
14234491e: 41 5f                       	popq	%r15
142344920: e9 1b 12 03 00              	jmp	0x142375b40 <.text+0x2374b40>
142344925: 48 89 d9                    	movq	%rbx, %rcx
142344928: 44 89 f2                    	movl	%r14d, %edx
14234492b: 41 89 e8                    	movl	%ebp, %r8d
14234492e: e8 dd 12 03 00              	callq	0x142375c10 <.text+0x2374c10>
142344933: 45 89 3c 24                 	movl	%r15d, (%r12)
142344937: 48 89 d9                    	movq	%rbx, %rcx
14234493a: 44 89 f2                    	movl	%r14d, %edx
14234493d: 41 89 e8                    	movl	%ebp, %r8d
142344940: e8 3b 16 03 00              	callq	0x142375f80 <.text+0x2374f80>
142344945: 45 89 3c 24                 	movl	%r15d, (%r12)
142344949: 48 89 d9                    	movq	%rbx, %rcx
14234494c: 89 fa                       	movl	%edi, %edx
14234494e: 41 89 f0                    	movl	%esi, %r8d
142344951: 48 83 c4 20                 	addq	$0x20, %rsp
142344955: 5b                          	popq	%rbx
142344956: 5d                          	popq	%rbp
142344957: 5f                          	popq	%rdi
142344958: 5e                          	popq	%rsi
142344959: 41 5c                       	popq	%r12
14234495b: 41 5e                       	popq	%r14
14234495d: 41 5f                       	popq	%r15
14234495f: e9 4c 15 03 00              	jmp	0x142375eb0 <.text+0x2374eb0>
142344964: 48 89 d9                    	movq	%rbx, %rcx
142344967: 44 89 f2                    	movl	%r14d, %edx
14234496a: 41 89 e8                    	movl	%ebp, %r8d
14234496d: e8 de 16 03 00              	callq	0x142376050 <.text+0x2375050>
142344972: 45 89 3c 24                 	movl	%r15d, (%r12)
142344976: 48 89 d9                    	movq	%rbx, %rcx
142344979: 44 89 f2                    	movl	%r14d, %edx
14234497c: 41 89 e8                    	movl	%ebp, %r8d
14234497f: e8 5c 13 03 00              	callq	0x142375ce0 <.text+0x2374ce0>
142344984: 45 89 3c 24                 	movl	%r15d, (%r12)
142344988: 48 89 d9                    	movq	%rbx, %rcx
14234498b: 89 fa                       	movl	%edi, %edx
14234498d: 41 89 f0                    	movl	%esi, %r8d
142344990: 48 83 c4 20                 	addq	$0x20, %rsp
142344994: 5b                          	popq	%rbx
142344995: 5d                          	popq	%rbp
142344996: 5f                          	popq	%rdi
142344997: 5e                          	popq	%rsi
142344998: 41 5c                       	popq	%r12
14234499a: 41 5e                       	popq	%r14
14234499c: 41 5f                       	popq	%r15
14234499e: e9 9d 11 03 00              	jmp	0x142375b40 <.text+0x2374b40>
1423449a3: 48 89 d9                    	movq	%rbx, %rcx
1423449a6: 44 89 f2                    	movl	%r14d, %edx
1423449a9: 41 89 e8                    	movl	%ebp, %r8d
1423449ac: e8 7f 18 03 00              	callq	0x142376230 <.text+0x2375230>
1423449b1: 45 89 3c 24                 	movl	%r15d, (%r12)
1423449b5: 48 89 d9                    	movq	%rbx, %rcx
1423449b8: 44 89 f2                    	movl	%r14d, %edx
1423449bb: 41 89 e8                    	movl	%ebp, %r8d
1423449be: e8 bd 15 03 00              	callq	0x142375f80 <.text+0x2374f80>
1423449c3: 45 89 3c 24                 	movl	%r15d, (%r12)
1423449c7: 48 89 d9                    	movq	%rbx, %rcx
1423449ca: 89 fa                       	movl	%edi, %edx
1423449cc: 41 89 f0                    	movl	%esi, %r8d
1423449cf: 48 83 c4 20                 	addq	$0x20, %rsp
1423449d3: 5b                          	popq	%rbx
1423449d4: 5d                          	popq	%rbp
1423449d5: 5f                          	popq	%rdi
1423449d6: 5e                          	popq	%rsi
1423449d7: 41 5c                       	popq	%r12
1423449d9: 41 5e                       	popq	%r14
1423449db: 41 5f                       	popq	%r15
1423449dd: e9 ce 14 03 00              	jmp	0x142375eb0 <.text+0x2374eb0>
1423449e2: cc                          	int3
1423449e3: cc                          	int3
1423449e4: cc                          	int3
1423449e5: cc                          	int3
1423449e6: cc                          	int3
1423449e7: cc                          	int3
1423449e8: cc                          	int3
1423449e9: cc                          	int3
1423449ea: cc                          	int3
1423449eb: cc                          	int3
1423449ec: cc                          	int3
1423449ed: cc                          	int3
1423449ee: cc                          	int3
1423449ef: cc                          	int3
1423449f0: 41 57                       	pushq	%r15
1423449f2: 41 56                       	pushq	%r14
1423449f4: 41 55                       	pushq	%r13
1423449f6: 41 54                       	pushq	%r12
1423449f8: 56                          	pushq	%rsi
1423449f9: 57                          	pushq	%rdi
1423449fa: 53                          	pushq	%rbx
1423449fb: 48 81 ec a0 00 00 00        	subq	$0xa0, %rsp
142344a02: 44 0f 29 9c 24 90 00 00 00  	movaps	%xmm11, 0x90(%rsp)
142344a0b: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
142344a14: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
142344a1a: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
142344a20: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
142344a25: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
142344a2a: 4c 8b bc 24 18 01 00 00     	movq	0x118(%rsp), %r15
142344a32: 4c 8b ac 24 10 01 00 00     	movq	0x110(%rsp), %r13
142344a3a: 4d 8b 30                    	movq	(%r8), %r14
142344a3d: f3 45 0f 10 8e e8 00 00 00  	movss	0xe8(%r14), %xmm9
142344a46: 41 80 be f1 00 00 00 00     	cmpb	$0x0, 0xf1(%r14)
142344a4e: 74 6f                       	je	0x142344abf <.text+0x2343abf>
142344a50: 4c 89 ce                    	movq	%r9, %rsi
142344a53: 48 89 d3                    	movq	%rdx, %rbx
142344a56: 48 89 cf                    	movq	%rcx, %rdi
142344a59: 48 8b 84 24 00 01 00 00     	movq	0x100(%rsp), %rax
142344a61: 48 8b 08                    	movq	(%rax), %rcx
142344a64: e8 a7 51 60 fe              	callq	0x140949c10 <.text+0x948c10>
142344a69: 0f 28 f8                    	movaps	%xmm0, %xmm7
142344a6c: 0f 28 f1                    	movaps	%xmm1, %xmm6
142344a6f: f3 44 0f 10 1d 84 98 d0 00  	movss	0xd09884(%rip), %xmm11  # 0x14304e2fc
142344a78: f3 45 0f 59 d9              	mulss	%xmm9, %xmm11
142344a7d: f3 0f 10 0d 0f fc 5e 00     	movss	0x5efc0f(%rip), %xmm1   # 0x142934694
142344a85: f3 41 0f 59 cb              	mulss	%xmm11, %xmm1
142344a8a: f3 0f 10 05 a2 85 68 00     	movss	0x6885a2(%rip), %xmm0   # 0x1429cd034
142344a92: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
142344a97: f3 0f 58 c1                 	addss	%xmm1, %xmm0
142344a9b: 41 0f b6 86 f0 00 00 00     	movzbl	0xf0(%r14), %eax
142344aa3: 8d 48 ff                    	leal	-0x1(%rax), %ecx
142344aa6: 80 f9 02                    	cmpb	$0x2, %cl
142344aa9: 72 30                       	jb	0x142344adb <.text+0x2343adb>
142344aab: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
142344aaf: 48 8d 0d 52 9c d0 00        	leaq	0xd09c52(%rip), %rcx    # 0x14304e708
142344ab6: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
142344aba: 48 01 c8                    	addq	%rcx, %rax
142344abd: ff e0                       	jmpq	*%rax
142344abf: 41 8b 45 1c                 	movl	0x1c(%r13), %eax
142344ac3: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
142344ac7: 89 01                       	movl	%eax, (%rcx)
142344ac9: 4d 8b 65 00                 	movq	(%r13), %r12
142344acd: 41 c7 44 24 10 00 00 00 00  	movl	$0x0, 0x10(%r12)
142344ad6: e9 fa 01 00 00              	jmp	0x142344cd5 <.text+0x2343cd5>
142344adb: 45 0f 28 c3                 	movaps	%xmm11, %xmm8
142344adf: 44 0f 28 d8                 	movaps	%xmm0, %xmm11
142344ae3: 48 8d 0d 1e 9c d0 00        	leaq	0xd09c1e(%rip), %rcx    # 0x14304e708
142344aea: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
142344aee: 48 01 c8                    	addq	%rcx, %rax
142344af1: ff e0                       	jmpq	*%rax
142344af3: f3 0f 10 35 75 0d 5e 00     	movss	0x5e0d75(%rip), %xmm6   # 0x142925870
142344afb: f3 0f 59 fe                 	mulss	%xmm6, %xmm7
142344aff: f3 41 0f 59 f3              	mulss	%xmm11, %xmm6
142344b04: eb 41                       	jmp	0x142344b47 <.text+0x2343b47>
142344b06: f3 0f 10 05 62 0d 5e 00     	movss	0x5e0d62(%rip), %xmm0   # 0x142925870
142344b0e: 41 0f 28 f8                 	movaps	%xmm8, %xmm7
142344b12: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
142344b16: eb 2b                       	jmp	0x142344b43 <.text+0x2343b43>
142344b18: f3 0f 10 05 50 0d 5e 00     	movss	0x5e0d50(%rip), %xmm0   # 0x142925870
142344b20: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
142344b24: f3 41 0f 59 c3              	mulss	%xmm11, %xmm0
142344b29: f3 0f 5c f0                 	subss	%xmm0, %xmm6
142344b2d: eb 18                       	jmp	0x142344b47 <.text+0x2343b47>
142344b2f: f3 0f 10 05 39 0d 5e 00     	movss	0x5e0d39(%rip), %xmm0   # 0x142925870
142344b37: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
142344b3b: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
142344b3f: f3 0f 5c f9                 	subss	%xmm1, %xmm7
142344b43: f3 0f 59 f0                 	mulss	%xmm0, %xmm6
142344b47: 4d 8b 65 00                 	movq	(%r13), %r12
142344b4b: f3 45 0f 10 54 24 10        	movss	0x10(%r12), %xmm10
142344b52: 48 8b 0b                    	movq	(%rbx), %rcx
142344b55: e8 86 b0 5f fe              	callq	0x14093fbe0 <.text+0x93ebe0>
142344b5a: 84 c0                       	testb	%al, %al
142344b5c: 0f 84 80 00 00 00           	je	0x142344be2 <.text+0x2343be2>
142344b62: f3 0f 10 05 1e bc 65 00     	movss	0x65bc1e(%rip), %xmm0   # 0x1429a0788
142344b6a: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
142344b6f: f3 0f 58 c0                 	addss	%xmm0, %xmm0
142344b73: f3 44 0f 58 c0              	addss	%xmm0, %xmm8
142344b78: f3 41 0f 58 c3              	addss	%xmm11, %xmm0
142344b7d: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
142344b83: 48 8d 5c 24 30              	leaq	0x30(%rsp), %rbx
142344b88: 48 89 d9                    	movq	%rbx, %rcx
142344b8b: 0f 28 cf                    	movaps	%xmm7, %xmm1
142344b8e: 0f 28 d6                    	movaps	%xmm6, %xmm2
142344b91: 41 0f 28 d8                 	movaps	%xmm8, %xmm3
142344b95: e8 56 b4 ef fe              	callq	0x14123fff0 <.text+0x123eff0>
142344b9a: 48 8b 07                    	movq	(%rdi), %rax
142344b9d: f3 0f 10 08                 	movss	(%rax), %xmm1
142344ba1: f3 0f 10 50 04              	movss	0x4(%rax), %xmm2
142344ba6: 48 89 d9                    	movq	%rbx, %rcx
142344ba9: e8 92 b5 ef fe              	callq	0x141240140 <.text+0x123f140>
142344bae: 84 c0                       	testb	%al, %al
142344bb0: 74 30                       	je	0x142344be2 <.text+0x2343be2>
142344bb2: 48 8b 84 24 08 01 00 00     	movq	0x108(%rsp), %rax
142344bba: 48 8b 00                    	movq	(%rax), %rax
142344bbd: 8b 08                       	movl	(%rax), %ecx
142344bbf: f3 0f 10 48 04              	movss	0x4(%rax), %xmm1
142344bc4: e8 a7 ae 5f fe              	callq	0x14093fa70 <.text+0x93ea70>
142344bc9: 84 c0                       	testb	%al, %al
142344bcb: 0f 84 02 02 00 00           	je	0x142344dd3 <.text+0x2343dd3>
142344bd1: f3 41 0f 10 44 24 10        	movss	0x10(%r12), %xmm0
142344bd8: 41 8b 45 1c                 	movl	0x1c(%r13), %eax
142344bdc: 49 8b 55 10                 	movq	0x10(%r13), %rdx
142344be0: eb 21                       	jmp	0x142344c03 <.text+0x2343c03>
142344be2: 48 8b 06                    	movq	(%rsi), %rax
142344be5: f3 0f 10 40 58              	movss	0x58(%rax), %xmm0
142344bea: f3 0f 5e 05 e6 0c 5e 00     	divss	0x5e0ce6(%rip), %xmm0   # 0x1429258d8
142344bf2: 41 8b 45 1c                 	movl	0x1c(%r13), %eax
142344bf6: 49 8b 55 10                 	movq	0x10(%r13), %rdx
142344bfa: 89 02                       	movl	%eax, (%rdx)
142344bfc: f3 41 0f 58 44 24 10        	addss	0x10(%r12), %xmm0
142344c03: 48 8b 8c 24 20 01 00 00     	movq	0x120(%rsp), %rcx
142344c0b: 0f 57 d2                    	xorps	%xmm2, %xmm2
142344c0e: 0f 57 db                    	xorps	%xmm3, %xmm3
142344c11: f3 0f 5f d8                 	maxss	%xmm0, %xmm3
142344c15: f3 0f 10 05 e7 0c 5e 00     	movss	0x5e0ce7(%rip), %xmm0   # 0x142925904
142344c1d: 0f 28 c8                    	movaps	%xmm0, %xmm1
142344c20: f3 0f 5d cb                 	minss	%xmm3, %xmm1
142344c24: 89 02                       	movl	%eax, (%rdx)
142344c26: f3 41 0f 11 4c 24 10        	movss	%xmm1, 0x10(%r12)
142344c2d: 44 0f 2e d2                 	ucomiss	%xmm2, %xmm10
142344c31: 75 45                       	jne	0x142344c78 <.text+0x2343c78>
142344c33: 7a 43                       	jp	0x142344c78 <.text+0x2343c78>
142344c35: 0f 2e ca                    	ucomiss	%xmm2, %xmm1
142344c38: 76 3e                       	jbe	0x142344c78 <.text+0x2343c78>
142344c3a: 8b 41 1c                    	movl	0x1c(%rcx), %eax
142344c3d: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
142344c41: 89 02                       	movl	%eax, (%rdx)
142344c43: 48 8b 31                    	movq	(%rcx), %rsi
142344c46: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
142344c4a: 48 8b 5e 40                 	movq	0x40(%rsi), %rbx
142344c4e: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
142344c52: 75 10                       	jne	0x142344c64 <.text+0x2343c64>
142344c54: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
142344c58: 48 8d 15 11 06 87 00        	leaq	0x870611(%rip), %rdx    # 0x142bb5270
142344c5f: e8 cc df 21 fe              	callq	0x140562c30 <.text+0x561c30>
142344c64: 48 8b 46 28                 	movq	0x28(%rsi), %rax
142344c68: 48 89 f9                    	movq	%rdi, %rcx
142344c6b: 48 c1 e1 05                 	shlq	$0x5, %rcx
142344c6f: c7 04 08 10 00 00 00        	movl	$0x10, (%rax,%rcx)
142344c76: eb 4d                       	jmp	0x142344cc5 <.text+0x2343cc5>
142344c78: 44 0f 2e 15 84 0c 5e 00     	ucomiss	0x5e0c84(%rip), %xmm10  # 0x142925904
142344c80: 75 53                       	jne	0x142344cd5 <.text+0x2343cd5>
142344c82: 7a 51                       	jp	0x142344cd5 <.text+0x2343cd5>
142344c84: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
142344c87: 76 4c                       	jbe	0x142344cd5 <.text+0x2343cd5>
142344c89: 8b 41 1c                    	movl	0x1c(%rcx), %eax
142344c8c: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
142344c90: 89 02                       	movl	%eax, (%rdx)
142344c92: 48 8b 31                    	movq	(%rcx), %rsi
142344c95: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
142344c99: 48 8b 5e 40                 	movq	0x40(%rsi), %rbx
142344c9d: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
142344ca1: 75 10                       	jne	0x142344cb3 <.text+0x2343cb3>
142344ca3: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
142344ca7: 48 8d 15 c2 05 87 00        	leaq	0x8705c2(%rip), %rdx    # 0x142bb5270
142344cae: e8 7d df 21 fe              	callq	0x140562c30 <.text+0x561c30>
142344cb3: 48 8b 46 28                 	movq	0x28(%rsi), %rax
142344cb7: 48 89 f9                    	movq	%rdi, %rcx
142344cba: 48 c1 e1 05                 	shlq	$0x5, %rcx
142344cbe: c7 04 08 11 00 00 00        	movl	$0x11, (%rax,%rcx)
142344cc5: 48 89 5c 08 18              	movq	%rbx, 0x18(%rax,%rcx)
142344cca: 48 ff c7                    	incq	%rdi
142344ccd: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
142344cd1: 48 ff 46 40                 	incq	0x40(%rsi)
142344cd5: f3 44 0f 59 0d 8e 7a 5e 00  	mulss	0x5e7a8e(%rip), %xmm9   # 0x14292c76c
142344cde: f3 41 0f 10 4c 24 10        	movss	0x10(%r12), %xmm1
142344ce5: 48 8d 0d 7c 99 d0 00        	leaq	0xd0997c(%rip), %rcx    # 0x14304e668
142344cec: e8 ff 11 95 fe              	callq	0x140c95ef0 <.text+0xc94ef0>
142344cf1: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
142344cf6: 41 8b 47 1c                 	movl	0x1c(%r15), %eax
142344cfa: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
142344cfe: 89 01                       	movl	%eax, (%rcx)
142344d00: 49 8b 0f                    	movq	(%r15), %rcx
142344d03: f3 41 0f 7e 0c 24           	movq	(%r12), %xmm1
142344d09: 48 8b 81 a0 00 00 00        	movq	0xa0(%rcx), %rax
142344d10: 48 8b 89 a8 00 00 00        	movq	0xa8(%rcx), %rcx
142344d17: 48 c1 e1 03                 	shlq	$0x3, %rcx
142344d1b: 48 8d 14 89                 	leaq	(%rcx,%rcx,4), %rdx
142344d1f: 31 c9                       	xorl	%ecx, %ecx
142344d21: 49 89 c0                    	movq	%rax, %r8
142344d24: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
142344d30: 48 39 ca                    	cmpq	%rcx, %rdx
142344d33: 74 63                       	je	0x142344d98 <.text+0x2343d98>
142344d35: f3 41 0f 7e 50 10           	movq	0x10(%r8), %xmm2
142344d3b: 49 83 c0 28                 	addq	$0x28, %r8
142344d3f: 66 0f 76 d1                 	pcmpeqd	%xmm1, %xmm2
142344d43: 66 0f 70 d2 50              	pshufd	$0x50, %xmm2, %xmm2     # xmm2 = xmm2[0,0,1,1]
142344d48: 66 44 0f 50 ca              	movmskpd	%xmm2, %r9d
142344d4d: 48 83 c1 28                 	addq	$0x28, %rcx
142344d51: 41 83 f9 03                 	cmpl	$0x3, %r9d
142344d55: 75 d9                       	jne	0x142344d30 <.text+0x2343d30>
142344d57: 41 0f b6 96 f0 00 00 00     	movzbl	0xf0(%r14), %edx
142344d5f: 66 0f ef c9                 	pxor	%xmm1, %xmm1
142344d63: 4c 8d 05 ae 99 d0 00        	leaq	0xd099ae(%rip), %r8     # 0x14304e718
142344d6a: 49 63 14 90                 	movslq	(%r8,%rdx,4), %rdx
142344d6e: 4c 01 c2                    	addq	%r8, %rdx
142344d71: ff e2                       	jmpq	*%rdx
142344d73: 0f 57 05 76 0b 5e 00        	xorps	0x5e0b76(%rip), %xmm0   # 0x1429258f0
142344d7a: 0f 57 d2                    	xorps	%xmm2, %xmm2
142344d7d: 0f 28 c8                    	movaps	%xmm0, %xmm1
142344d80: eb 0a                       	jmp	0x142344d8c <.text+0x2343d8c>
142344d82: 0f 57 05 67 0b 5e 00        	xorps	0x5e0b67(%rip), %xmm0   # 0x1429258f0
142344d89: 0f 28 d0                    	movaps	%xmm0, %xmm2
142344d8c: f3 0f 11 54 08 e0           	movss	%xmm2, -0x20(%rax,%rcx)
142344d92: f3 0f 11 4c 08 e4           	movss	%xmm1, -0x1c(%rax,%rcx)
142344d98: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
142344d9d: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
142344da2: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
142344da8: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
142344dae: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
142344db7: 44 0f 28 9c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm11
142344dc0: 48 81 c4 a0 00 00 00        	addq	$0xa0, %rsp
142344dc7: 5b                          	popq	%rbx
142344dc8: 5f                          	popq	%rdi
142344dc9: 5e                          	popq	%rsi
142344dca: 41 5c                       	popq	%r12
142344dcc: 41 5d                       	popq	%r13
142344dce: 41 5e                       	popq	%r14
142344dd0: 41 5f                       	popq	%r15
142344dd2: c3                          	retq
142344dd3: 48 8b 06                    	movq	(%rsi), %rax
142344dd6: f3 0f 10 40 58              	movss	0x58(%rax), %xmm0
142344ddb: f3 0f 5e 05 1d 0b 5e 00     	divss	0x5e0b1d(%rip), %xmm0   # 0x142925900
142344de3: e9 0a fe ff ff              	jmp	0x142344bf2 <.text+0x2343bf2>

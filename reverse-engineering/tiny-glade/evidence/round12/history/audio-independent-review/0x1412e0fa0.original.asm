
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412e0fa0: 55                          	pushq	%rbp
1412e0fa1: 56                          	pushq	%rsi
1412e0fa2: 57                          	pushq	%rdi
1412e0fa3: 48 81 ec b0 00 00 00        	subq	$0xb0, %rsp
1412e0faa: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1412e0fb2: 48 c7 45 28 fe ff ff ff     	movq	$-0x2, 0x28(%rbp)
1412e0fba: 48 8b 71 08                 	movq	0x8(%rcx), %rsi
1412e0fbe: 48 85 f6                    	testq	%rsi, %rsi
1412e0fc1: 74 51                       	je	0x1412e1014 <.text+0x12e0014>
1412e0fc3: 48 8b 42 30                 	movq	0x30(%rdx), %rax
1412e0fc7: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
1412e0fcb: 0f 10 02                    	movups	(%rdx), %xmm0
1412e0fce: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
1412e0fd2: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
1412e0fd6: 0f 29 55 10                 	movaps	%xmm2, 0x10(%rbp)
1412e0fda: 0f 29 4d 00                 	movaps	%xmm1, (%rbp)
1412e0fde: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
1412e0fe2: 48 8b 06                    	movq	(%rsi), %rax
1412e0fe5: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
1412e0fe9: 48 29 f8                    	subq	%rdi, %rax
1412e0fec: 48 83 f8 3f                 	cmpq	$0x3f, %rax
1412e0ff0: 77 55                       	ja	0x1412e1047 <.text+0x12e0047>
1412e0ff2: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1412e0ffb: 41 b8 40 00 00 00           	movl	$0x40, %r8d
1412e1001: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1412e1007: 48 89 f1                    	movq	%rsi, %rcx
1412e100a: 48 89 fa                    	movq	%rdi, %rdx
1412e100d: e8 4e 8a 5c 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1412e1012: eb 33                       	jmp	0x1412e1047 <.text+0x12e0047>
1412e1014: 48 8b 71 10                 	movq	0x10(%rcx), %rsi
1412e1018: 48 8b 42 30                 	movq	0x30(%rdx), %rax
1412e101c: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
1412e1020: 0f 10 02                    	movups	(%rdx), %xmm0
1412e1023: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
1412e1027: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
1412e102b: 0f 29 55 10                 	movaps	%xmm2, 0x10(%rbp)
1412e102f: 0f 29 4d 00                 	movaps	%xmm1, (%rbp)
1412e1033: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
1412e1037: 48 8b 06                    	movq	(%rsi), %rax
1412e103a: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
1412e103e: 48 29 f8                    	subq	%rdi, %rax
1412e1041: 48 83 f8 3f                 	cmpq	$0x3f, %rax
1412e1045: 76 66                       	jbe	0x1412e10ad <.text+0x12e00ad>
1412e1047: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1412e104b: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
1412e104f: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
1412e1053: 0f 28 45 f0                 	movaps	-0x10(%rbp), %xmm0
1412e1057: 0f 28 4d 00                 	movaps	(%rbp), %xmm1
1412e105b: 0f 28 55 10                 	movaps	0x10(%rbp), %xmm2
1412e105f: 0f 29 55 d0                 	movaps	%xmm2, -0x30(%rbp)
1412e1063: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
1412e1067: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1412e106b: 48 8d 0d 1e 5c ff ff        	leaq	-0xa3e2(%rip), %rcx     # 0x1412d6c90 <.text+0x12d5c90>
1412e1072: 48 89 0c 38                 	movq	%rcx, (%rax,%rdi)
1412e1076: 0f 28 45 b0                 	movaps	-0x50(%rbp), %xmm0
1412e107a: 0f 28 4d c0                 	movaps	-0x40(%rbp), %xmm1
1412e107e: 0f 28 55 d0                 	movaps	-0x30(%rbp), %xmm2
1412e1082: 0f 11 44 38 08              	movups	%xmm0, 0x8(%rax,%rdi)
1412e1087: 0f 11 4c 38 18              	movups	%xmm1, 0x18(%rax,%rdi)
1412e108c: 0f 11 54 38 28              	movups	%xmm2, 0x28(%rax,%rdi)
1412e1091: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
1412e1095: 48 89 4c 38 38              	movq	%rcx, 0x38(%rax,%rdi)
1412e109a: 48 83 c7 40                 	addq	$0x40, %rdi
1412e109e: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
1412e10a2: 48 81 c4 b0 00 00 00        	addq	$0xb0, %rsp
1412e10a9: 5f                          	popq	%rdi
1412e10aa: 5e                          	popq	%rsi
1412e10ab: 5d                          	popq	%rbp
1412e10ac: c3                          	retq
1412e10ad: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1412e10b6: 41 b8 40 00 00 00           	movl	$0x40, %r8d
1412e10bc: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1412e10c2: 48 89 f1                    	movq	%rsi, %rcx
1412e10c5: 48 89 fa                    	movq	%rdi, %rdx
1412e10c8: e8 93 89 5c 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1412e10cd: e9 75 ff ff ff              	jmp	0x1412e1047 <.text+0x12e0047>
1412e10d2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1412e10e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412e10e5: 55                          	pushq	%rbp
1412e10e6: 56                          	pushq	%rsi
1412e10e7: 57                          	pushq	%rdi
1412e10e8: 48 83 ec 30                 	subq	$0x30, %rsp
1412e10ec: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412e10f3: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
1412e10f7: e8 44 bd 32 ff              	callq	0x14060ce40 <.text+0x60be40>
1412e10fc: 90                          	nop
1412e10fd: 48 83 c4 30                 	addq	$0x30, %rsp
1412e1101: 5f                          	popq	%rdi
1412e1102: 5e                          	popq	%rsi
1412e1103: 5d                          	popq	%rbp
1412e1104: c3                          	retq
1412e1105: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1412e1110: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412e1115: 55                          	pushq	%rbp
1412e1116: 56                          	pushq	%rsi
1412e1117: 57                          	pushq	%rdi
1412e1118: 48 83 ec 30                 	subq	$0x30, %rsp
1412e111c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412e1123: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
1412e1127: e8 14 bd 32 ff              	callq	0x14060ce40 <.text+0x60be40>
1412e112c: 90                          	nop
1412e112d: 48 83 c4 30                 	addq	$0x30, %rsp
1412e1131: 5f                          	popq	%rdi
1412e1132: 5e                          	popq	%rsi
1412e1133: 5d                          	popq	%rbp
1412e1134: c3                          	retq

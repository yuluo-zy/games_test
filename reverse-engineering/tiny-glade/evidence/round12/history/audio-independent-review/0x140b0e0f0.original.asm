
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b0e0f0: 56                          	pushq	%rsi
140b0e0f1: 57                          	pushq	%rdi
140b0e0f2: 53                          	pushq	%rbx
140b0e0f3: 48 81 ec e0 00 00 00        	subq	$0xe0, %rsp
140b0e0fa: 48 89 d6                    	movq	%rdx, %rsi
140b0e0fd: 48 89 cf                    	movq	%rcx, %rdi
140b0e100: e8 db 54 25 00              	callq	0x140d635e0 <.text+0xd625e0>
140b0e105: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140b0e10a: 89 54 24 38                 	movl	%edx, 0x38(%rsp)
140b0e10e: 8b 47 1c                    	movl	0x1c(%rdi), %eax
140b0e111: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
140b0e115: 89 01                       	movl	%eax, (%rcx)
140b0e117: 48 8b 0f                    	movq	(%rdi), %rcx
140b0e11a: e8 e1 8f f4 ff              	callq	0x140a57100 <.text+0xa56100>
140b0e11f: 48 8b 05 82 e2 b1 02        	movq	0x2b1e282(%rip), %rax   # 0x14362c3a8
140b0e126: 48 8b 00                    	movq	(%rax), %rax
140b0e129: 48 83 f8 02                 	cmpq	$0x2, %rax
140b0e12d: 0f 86 eb 00 00 00           	jbe	0x140b0e21e <.text+0xb0d21e>
140b0e133: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140b0e138: e8 d3 57 25 00              	callq	0x140d63910 <.text+0xd62910>
140b0e13d: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140b0e142: 89 54 24 58                 	movl	%edx, 0x58(%rsp)
140b0e146: 48 8d 44 24 50              	leaq	0x50(%rsp), %rax
140b0e14b: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140b0e150: 48 8d 05 f9 c3 27 00        	leaq	0x27c3f9(%rip), %rax    # 0x140d8a550 <.text+0xd89550>
140b0e157: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140b0e15c: 48 8d 0d 35 9c fd 01        	leaq	0x1fd9c35(%rip), %rcx   # 0x142ae7d98
140b0e163: e8 18 fc b6 ff              	callq	0x14067dd80 <.text+0x67cd80>
140b0e168: 0f 10 00                    	movups	(%rax), %xmm0
140b0e16b: 8b 40 10                    	movl	0x10(%rax), %eax
140b0e16e: 48 c7 84 24 90 00 00 00 03 00 00 00 	movq	$0x3, 0x90(%rsp)
140b0e17a: 48 8d 0d 2f 9c fd 01        	leaq	0x1fd9c2f(%rip), %rcx   # 0x142ae7db0
140b0e181: 48 89 8c 24 98 00 00 00     	movq	%rcx, 0x98(%rsp)
140b0e189: 48 c7 84 24 a0 00 00 00 21 00 00 00 	movq	$0x21, 0xa0(%rsp)
140b0e195: 48 8d 15 dc 9b fd 01        	leaq	0x1fd9bdc(%rip), %rdx   # 0x142ae7d78
140b0e19c: 48 89 94 24 b0 00 00 00     	movq	%rdx, 0xb0(%rsp)
140b0e1a4: 48 c7 84 24 b8 00 00 00 02 00 00 00 	movq	$0x2, 0xb8(%rsp)
140b0e1b0: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
140b0e1b5: 48 89 94 24 c0 00 00 00     	movq	%rdx, 0xc0(%rsp)
140b0e1bd: 48 c7 84 24 c8 00 00 00 01 00 00 00 	movq	$0x1, 0xc8(%rsp)
140b0e1c9: 48 c7 84 24 d0 00 00 00 00 00 00 00 	movq	$0x0, 0xd0(%rsp)
140b0e1d5: 48 c7 44 24 60 00 00 00 00  	movq	$0x0, 0x60(%rsp)
140b0e1de: 48 89 4c 24 68              	movq	%rcx, 0x68(%rsp)
140b0e1e3: 48 c7 44 24 70 21 00 00 00  	movq	$0x21, 0x70(%rsp)
140b0e1ec: 48 c7 44 24 78 00 00 00 00  	movq	$0x0, 0x78(%rsp)
140b0e1f5: 0f 11 84 24 80 00 00 00     	movups	%xmm0, 0x80(%rsp)
140b0e1fd: c7 84 24 a8 00 00 00 01 00 00 00    	movl	$0x1, 0xa8(%rsp)
140b0e208: 89 84 24 ac 00 00 00        	movl	%eax, 0xac(%rsp)
140b0e20f: 48 8d 4c 24 2f              	leaq	0x2f(%rsp), %rcx
140b0e214: 48 8d 54 24 60              	leaq	0x60(%rsp), %rdx
140b0e219: e8 12 c1 23 00              	callq	0x140d4a330 <.text+0xd49330>
140b0e21e: 0f b6 05 e4 69 b2 02        	movzbl	0x2b269e4(%rip), %eax   # 0x143634c09
140b0e225: b9 1c 00 00 00              	movl	$0x1c, %ecx
140b0e22a: ba 04 00 00 00              	movl	$0x4, %edx
140b0e22f: e8 dc 59 b0 ff              	callq	0x140613c10 <.text+0x612c10>
140b0e234: 48 85 c0                    	testq	%rax, %rax
140b0e237: 0f 84 1e 01 00 00           	je	0x140b0e35b <.text+0xb0d35b>
140b0e23d: c7 00 00 00 00 00           	movl	$0x0, (%rax)
140b0e243: c7 40 18 00 00 00 00        	movl	$0x0, 0x18(%rax)
140b0e24a: 48 c7 44 24 60 01 00 00 00  	movq	$0x1, 0x60(%rsp)
140b0e253: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
140b0e258: 48 c7 44 24 70 01 00 00 00  	movq	$0x1, 0x70(%rsp)
140b0e261: 48 c7 44 24 78 01 00 00 00  	movq	$0x1, 0x78(%rsp)
140b0e26a: 4c 8d 05 67 9b fd 01        	leaq	0x1fd9b67(%rip), %r8    # 0x142ae7dd8
140b0e271: 48 8d 54 24 60              	leaq	0x60(%rsp), %rdx
140b0e276: 48 89 f1                    	movq	%rsi, %rcx
140b0e279: e8 b2 10 7d 00              	callq	0x1412df330 <.text+0x12de330>
140b0e27e: 48 8b 7e 08                 	movq	0x8(%rsi), %rdi
140b0e282: 48 85 ff                    	testq	%rdi, %rdi
140b0e285: 75 04                       	jne	0x140b0e28b <.text+0xb0d28b>
140b0e287: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
140b0e28b: 48 8b 07                    	movq	(%rdi), %rax
140b0e28e: 48 8b 5f 10                 	movq	0x10(%rdi), %rbx
140b0e292: 48 29 d8                    	subq	%rbx, %rax
140b0e295: 48 83 f8 0b                 	cmpq	$0xb, %rax
140b0e299: 76 79                       	jbe	0x140b0e314 <.text+0xb0d314>
140b0e29b: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140b0e29f: 48 8d 0d 4a ea 7c 00        	leaq	0x7cea4a(%rip), %rcx    # 0x1412dccf0 <.text+0x12dbcf0>
140b0e2a6: 48 89 0c 18                 	movq	%rcx, (%rax,%rbx)
140b0e2aa: c7 44 18 08 00 00 00 00     	movl	$0x0, 0x8(%rax,%rbx)
140b0e2b2: 48 83 c3 0c                 	addq	$0xc, %rbx
140b0e2b6: 48 89 5f 10                 	movq	%rbx, 0x10(%rdi)
140b0e2ba: 48 c7 44 24 60 02 00 00 00  	movq	$0x2, 0x60(%rsp)
140b0e2c3: c7 84 24 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rsp)
140b0e2ce: 4c 8d 05 1b 9b fd 01        	leaq	0x1fd9b1b(%rip), %r8    # 0x142ae7df0
140b0e2d5: 48 8d 54 24 60              	leaq	0x60(%rsp), %rdx
140b0e2da: 48 89 f1                    	movq	%rsi, %rcx
140b0e2dd: e8 be 2c 7d 00              	callq	0x1412e0fa0 <.text+0x12dffa0>
140b0e2e2: 48 8b 07                    	movq	(%rdi), %rax
140b0e2e5: 48 8b 77 10                 	movq	0x10(%rdi), %rsi
140b0e2e9: 48 29 f0                    	subq	%rsi, %rax
140b0e2ec: 48 83 f8 07                 	cmpq	$0x7, %rax
140b0e2f0: 76 47                       	jbe	0x140b0e339 <.text+0xb0d339>
140b0e2f2: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140b0e2f6: 48 8d 0d b3 ae 7c 00        	leaq	0x7caeb3(%rip), %rcx    # 0x1412d91b0 <.text+0x12d81b0>
140b0e2fd: 48 89 0c 30                 	movq	%rcx, (%rax,%rsi)
140b0e301: 48 83 c6 08                 	addq	$0x8, %rsi
140b0e305: 48 89 77 10                 	movq	%rsi, 0x10(%rdi)
140b0e309: 48 81 c4 e0 00 00 00        	addq	$0xe0, %rsp
140b0e310: 5b                          	popq	%rbx
140b0e311: 5f                          	popq	%rdi
140b0e312: 5e                          	popq	%rsi
140b0e313: c3                          	retq
140b0e314: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140b0e31d: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
140b0e323: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140b0e329: 48 89 f9                    	movq	%rdi, %rcx
140b0e32c: 48 89 da                    	movq	%rbx, %rdx
140b0e32f: e8 2c b7 d9 01              	callq	0x1428a9a60 <.text+0x28a8a60>
140b0e334: e9 62 ff ff ff              	jmp	0x140b0e29b <.text+0xb0d29b>
140b0e339: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140b0e342: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140b0e348: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140b0e34e: 48 89 f9                    	movq	%rdi, %rcx
140b0e351: 48 89 f2                    	movq	%rsi, %rdx
140b0e354: e8 07 b7 d9 01              	callq	0x1428a9a60 <.text+0x28a8a60>
140b0e359: eb 97                       	jmp	0x140b0e2f2 <.text+0xb0d2f2>
140b0e35b: 4c 8d 05 66 f5 15 02        	leaq	0x215f566(%rip), %r8    # 0x142c6d8c8
140b0e362: b9 04 00 00 00              	movl	$0x4, %ecx
140b0e367: ba 1c 00 00 00              	movl	$0x1c, %edx
140b0e36c: e8 72 ac dc 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140b0e371: cc                          	int3

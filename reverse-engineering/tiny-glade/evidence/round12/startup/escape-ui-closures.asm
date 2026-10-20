
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405047f0: 55                          	pushq	%rbp
1405047f1: 56                          	pushq	%rsi
1405047f2: 57                          	pushq	%rdi
1405047f3: 48 81 ec c0 00 00 00        	subq	$0xc0, %rsp
1405047fa: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140504802: 0f 29 75 30                 	movaps	%xmm6, 0x30(%rbp)
140504806: 48 c7 45 28 fe ff ff ff     	movq	$-0x2, 0x28(%rbp)
14050480e: 48 89 d7                    	movq	%rdx, %rdi
140504811: 48 89 ce                    	movq	%rcx, %rsi
140504814: 48 8b 05 7d 78 12 03        	movq	0x312787d(%rip), %rax   # 0x14362c098
14050481b: 80 38 00                    	cmpb	$0x0, (%rax)
14050481e: 0f 84 8f 00 00 00           	je	0x1405048b3 <.text+0x5038b3>
140504824: 48 8b 06                    	movq	(%rsi), %rax
140504827: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
14050482b: 48 89 45 a8                 	movq	%rax, -0x58(%rbp)
14050482f: 48 8d 05 5a 56 c0 ff        	leaq	-0x3fa9a6(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140504836: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
14050483a: 48 89 4d b8                 	movq	%rcx, -0x48(%rbp)
14050483e: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140504842: 48 8d 05 af 95 4e 02        	leaq	0x24e95af(%rip), %rax   # 0x1429eddf8
140504849: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
14050484d: 48 c7 45 e0 02 00 00 00     	movq	$0x2, -0x20(%rbp)
140504855: 48 8d 45 a8                 	leaq	-0x58(%rbp), %rax
140504859: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
14050485d: 48 c7 45 f0 02 00 00 00     	movq	$0x2, -0x10(%rbp)
140504865: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
14050486d: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
140504871: 48 8d 55 d8                 	leaq	-0x28(%rbp), %rdx
140504875: e8 16 57 87 00              	callq	0x140d79f90 <.text+0xd78f90>
14050487a: 4c 8b 45 10                 	movq	0x10(%rbp), %r8
14050487e: 4c 8b 4d 18                 	movq	0x18(%rbp), %r9
140504882: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140504886: 48 89 fa                    	movq	%rdi, %rdx
140504889: 4c 89 45 20                 	movq	%r8, 0x20(%rbp)
14050488d: e8 6e 76 18 00              	callq	0x14068bf00 <.text+0x68af00>
140504892: 48 8b 46 10                 	movq	0x10(%rsi), %rax
140504896: f3 0f 10 30                 	movss	(%rax), %xmm6
14050489a: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
14050489e: e8 8d d4 17 00              	callq	0x140681d30 <.text+0x680d30>
1405048a3: 48 89 c1                    	movq	%rax, %rcx
1405048a6: 0f 28 ce                    	movaps	%xmm6, %xmm1
1405048a9: e8 a2 1a c0 ff              	callq	0x140106350 <.text+0x105350>
1405048ae: e9 96 00 00 00              	jmp	0x140504949 <.text+0x503949>
1405048b3: 48 8b 46 18                 	movq	0x18(%rsi), %rax
1405048b7: 48 8b 0e                    	movq	(%rsi), %rcx
1405048ba: 48 8b 56 08                 	movq	0x8(%rsi), %rdx
1405048be: 48 89 45 a8                 	movq	%rax, -0x58(%rbp)
1405048c2: 48 8d 05 c7 55 c0 ff        	leaq	-0x3faa39(%rip), %rax   # 0x140109e90 <.text+0x108e90>
1405048c9: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
1405048cd: 48 89 4d b8                 	movq	%rcx, -0x48(%rbp)
1405048d1: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
1405048d5: 48 89 55 c8                 	movq	%rdx, -0x38(%rbp)
1405048d9: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
1405048dd: 48 8d 05 dc 94 4e 02        	leaq	0x24e94dc(%rip), %rax   # 0x1429eddc0
1405048e4: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1405048e8: 48 c7 45 e0 03 00 00 00     	movq	$0x3, -0x20(%rbp)
1405048f0: 48 8d 45 a8                 	leaq	-0x58(%rbp), %rax
1405048f4: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1405048f8: 48 c7 45 f0 03 00 00 00     	movq	$0x3, -0x10(%rbp)
140504900: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
140504908: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
14050490c: 48 8d 55 d8                 	leaq	-0x28(%rbp), %rdx
140504910: e8 7b 56 87 00              	callq	0x140d79f90 <.text+0xd78f90>
140504915: 4c 8b 45 10                 	movq	0x10(%rbp), %r8
140504919: 4c 8b 4d 18                 	movq	0x18(%rbp), %r9
14050491d: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140504921: 48 89 fa                    	movq	%rdi, %rdx
140504924: 4c 89 45 20                 	movq	%r8, 0x20(%rbp)
140504928: e8 d3 75 18 00              	callq	0x14068bf00 <.text+0x68af00>
14050492d: 48 8b 46 10                 	movq	0x10(%rsi), %rax
140504931: f3 0f 10 30                 	movss	(%rax), %xmm6
140504935: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140504939: e8 f2 d3 17 00              	callq	0x140681d30 <.text+0x680d30>
14050493e: 48 89 c1                    	movq	%rax, %rcx
140504941: 0f 28 ce                    	movaps	%xmm6, %xmm1
140504944: e8 07 1a c0 ff              	callq	0x140106350 <.text+0x105350>
140504949: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
14050494d: 48 85 d2                    	testq	%rdx, %rdx
140504950: 74 0f                       	je	0x140504961 <.text+0x503961>
140504952: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140504958: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
14050495c: e8 bf f2 10 00              	callq	0x140613c20 <.text+0x612c20>
140504961: 0f 28 75 30                 	movaps	0x30(%rbp), %xmm6
140504965: 48 81 c4 c0 00 00 00        	addq	$0xc0, %rsp
14050496c: 5f                          	popq	%rdi
14050496d: 5e                          	popq	%rsi
14050496e: 5d                          	popq	%rbp
14050496f: c3                          	retq
140504970: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140504975: 55                          	pushq	%rbp
140504976: 56                          	pushq	%rsi
140504977: 57                          	pushq	%rdi
140504978: 48 83 ec 30                 	subq	$0x30, %rsp
14050497c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140504983: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
140504988: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
14050498c: 48 85 d2                    	testq	%rdx, %rdx
14050498f: 74 0f                       	je	0x1405049a0 <.text+0x5039a0>
140504991: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140504997: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
14050499b: e8 80 f2 10 00              	callq	0x140613c20 <.text+0x612c20>
1405049a0: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1405049a5: 48 83 c4 30                 	addq	$0x30, %rsp
1405049a9: 5f                          	popq	%rdi
1405049aa: 5e                          	popq	%rsi
1405049ab: 5d                          	popq	%rbp
1405049ac: c3                          	retq
1405049ad: 0f 1f 00                    	nopl	(%rax)
1405049b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1405049b5: 55                          	pushq	%rbp
1405049b6: 56                          	pushq	%rsi
1405049b7: 57                          	pushq	%rdi
1405049b8: 48 83 ec 30                 	subq	$0x30, %rsp
1405049bc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1405049c3: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1405049c8: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
1405049cc: 48 85 d2                    	testq	%rdx, %rdx
1405049cf: 74 0f                       	je	0x1405049e0 <.text+0x5039e0>
1405049d1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1405049d7: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
1405049db: e8 40 f2 10 00              	callq	0x140613c20 <.text+0x612c20>
1405049e0: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1405049e5: 48 83 c4 30                 	addq	$0x30, %rsp
1405049e9: 5f                          	popq	%rdi
1405049ea: 5e                          	popq	%rsi
1405049eb: 5d                          	popq	%rbp
1405049ec: c3                          	retq
1405049ed: cc                          	int3
1405049ee: cc                          	int3
1405049ef: cc                          	int3
1405049f0: 41 57                       	pushq	%r15
1405049f2: 41 56                       	pushq	%r14
1405049f4: 56                          	pushq	%rsi
1405049f5: 57                          	pushq	%rdi
1405049f6: 53                          	pushq	%rbx
1405049f7: 48 81 ec b0 00 00 00        	subq	$0xb0, %rsp
1405049fe: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140504a06: 48 89 d7                    	movq	%rdx, %rdi
140504a09: 48 89 ce                    	movq	%rcx, %rsi
140504a0c: 48 8b 01                    	movq	(%rcx), %rax
140504a0f: 48 8b 00                    	movq	(%rax), %rax
140504a12: 80 b8 f1 00 00 00 00        	cmpb	$0x0, 0xf1(%rax)
140504a19: 75 0a                       	jne	0x140504a25 <.text+0x503a25>
140504a1b: f3 0f 10 35 e1 0e 42 02     	movss	0x2420ee1(%rip), %xmm6  # 0x142925904
140504a23: eb 08                       	jmp	0x140504a2d <.text+0x503a2d>
140504a25: f3 0f 10 35 43 0e 42 02     	movss	0x2420e43(%rip), %xmm6  # 0x142925870
140504a2d: 4c 8d 05 e4 93 4e 02        	leaq	0x24e93e4(%rip), %r8    # 0x1429ede18
140504a34: 48 8d 5c 24 40              	leaq	0x40(%rsp), %rbx
140504a39: 41 b9 0f 00 00 00           	movl	$0xf, %r9d
140504a3f: 48 89 d9                    	movq	%rbx, %rcx
140504a42: 48 89 fa                    	movq	%rdi, %rdx
140504a45: e8 76 c7 17 00              	callq	0x1406811c0 <.text+0x6801c0>
140504a4a: 4c 8d 74 24 28              	leaq	0x28(%rsp), %r14
140504a4f: 4c 89 f1                    	movq	%r14, %rcx
140504a52: 48 89 da                    	movq	%rbx, %rdx
140504a55: 0f 28 d6                    	movaps	%xmm6, %xmm2
140504a58: e8 93 ca 17 00              	callq	0x1406814f0 <.text+0x6804f0>
140504a5d: 4c 89 f1                    	movq	%r14, %rcx
140504a60: e8 eb d2 17 00              	callq	0x140681d50 <.text+0x680d50>
140504a65: 48 89 c1                    	movq	%rax, %rcx
140504a68: e8 13 f2 17 00              	callq	0x140683c80 <.text+0x682c80>
140504a6d: 84 c0                       	testb	%al, %al
140504a6f: 74 69                       	je	0x140504ada <.text+0x503ada>
140504a71: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140504a75: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140504a78: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140504a7c: 89 0a                       	movl	%ecx, (%rdx)
140504a7e: 48 8b 18                    	movq	(%rax), %rbx
140504a81: 4c 8b 73 30                 	movq	0x30(%rbx), %r14
140504a85: 4c 8b 7b 40                 	movq	0x40(%rbx), %r15
140504a89: 4c 3b 73 20                 	cmpq	0x20(%rbx), %r14
140504a8d: 75 10                       	jne	0x140504a9f <.text+0x503a9f>
140504a8f: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
140504a93: 48 8d 15 8e e6 4e 02        	leaq	0x24ee68e(%rip), %rdx   # 0x1429f3128
140504a9a: e8 d1 dd 05 00              	callq	0x140562870 <.text+0x561870>
140504a9f: 48 8b 43 28                 	movq	0x28(%rbx), %rax
140504aa3: 4e 89 3c f0                 	movq	%r15, (%rax,%r14,8)
140504aa7: 49 ff c6                    	incq	%r14
140504aaa: 4c 89 73 30                 	movq	%r14, 0x30(%rbx)
140504aae: 48 ff 43 40                 	incq	0x40(%rbx)
140504ab2: 48 8b 46 10                 	movq	0x10(%rsi), %rax
140504ab6: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140504ab9: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140504abd: 89 0a                       	movl	%ecx, (%rdx)
140504abf: 48 8b 08                    	movq	(%rax), %rcx
140504ac2: c6 44 24 48 00              	movb	$0x0, 0x48(%rsp)
140504ac7: 48 c7 44 24 40 08 00 00 00  	movq	$0x8, 0x40(%rsp)
140504ad0: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
140504ad5: e8 f6 af 43 00              	callq	0x14093fad0 <.text+0x93ead0>
140504ada: 4c 8d 05 46 93 4e 02        	leaq	0x24e9346(%rip), %r8    # 0x1429ede27
140504ae1: 48 8d 5c 24 40              	leaq	0x40(%rsp), %rbx
140504ae6: 41 b9 12 00 00 00           	movl	$0x12, %r9d
140504aec: 48 89 d9                    	movq	%rbx, %rcx
140504aef: 48 89 fa                    	movq	%rdi, %rdx
140504af2: e8 c9 c6 17 00              	callq	0x1406811c0 <.text+0x6801c0>
140504af7: 48 8d 7c 24 28              	leaq	0x28(%rsp), %rdi
140504afc: 48 89 f9                    	movq	%rdi, %rcx
140504aff: 48 89 da                    	movq	%rbx, %rdx
140504b02: 0f 28 d6                    	movaps	%xmm6, %xmm2
140504b05: e8 e6 c9 17 00              	callq	0x1406814f0 <.text+0x6804f0>
140504b0a: 48 89 f9                    	movq	%rdi, %rcx
140504b0d: e8 3e d2 17 00              	callq	0x140681d50 <.text+0x680d50>
140504b12: 48 89 c1                    	movq	%rax, %rcx
140504b15: e8 66 f1 17 00              	callq	0x140683c80 <.text+0x682c80>
140504b1a: 84 c0                       	testb	%al, %al
140504b1c: 74 07                       	je	0x140504b25 <.text+0x503b25>
140504b1e: 48 8b 46 18                 	movq	0x18(%rsi), %rax
140504b22: c6 00 01                    	movb	$0x1, (%rax)
140504b25: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140504b2d: 48 81 c4 b0 00 00 00        	addq	$0xb0, %rsp
140504b34: 5b                          	popq	%rbx
140504b35: 5f                          	popq	%rdi
140504b36: 5e                          	popq	%rsi
140504b37: 41 5e                       	popq	%r14
140504b39: 41 5f                       	popq	%r15
140504b3b: c3                          	retq
140504b3c: cc                          	int3
140504b3d: cc                          	int3
140504b3e: cc                          	int3
140504b3f: cc                          	int3
140504b40: 55                          	pushq	%rbp
140504b41: 41 57                       	pushq	%r15
140504b43: 41 56                       	pushq	%r14
140504b45: 56                          	pushq	%rsi
140504b46: 57                          	pushq	%rdi
140504b47: 53                          	pushq	%rbx
140504b48: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140504b4f: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140504b57: 48 c7 45 40 fe ff ff ff     	movq	$-0x2, 0x40(%rbp)
140504b5f: 4c 89 c7                    	movq	%r8, %rdi
140504b62: 48 89 d6                    	movq	%rdx, %rsi
140504b65: 48 89 cb                    	movq	%rcx, %rbx
140504b68: 48 8d 0d 65 93 4e 02        	leaq	0x24e9365(%rip), %rcx   # 0x1429eded4
140504b6f: ba 0d 00 00 00              	movl	$0xd, %edx
140504b74: e8 07 89 69 00              	callq	0x140b9d480 <.text+0xb9c480>
140504b79: 49 89 d0                    	movq	%rdx, %r8
140504b7c: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140504b80: 48 89 c2                    	movq	%rax, %rdx
140504b83: e8 b8 61 87 00              	callq	0x140d7ad40 <.text+0xd79d40>
140504b88: 4c 8b 45 c8                 	movq	-0x38(%rbp), %r8
140504b8c: 4c 8b 4d d0                 	movq	-0x30(%rbp), %r9
140504b90: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140504b94: 48 89 fa                    	movq	%rdi, %rdx
140504b97: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
140504b9b: e8 20 74 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
140504ba0: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140504ba4: e8 d7 91 17 00              	callq	0x14067dd80 <.text+0x67cd80>
140504ba9: 48 89 c1                    	movq	%rax, %rcx
140504bac: e8 9f d2 17 00              	callq	0x140681e50 <.text+0x680e50>
140504bb1: 48 89 c1                    	movq	%rax, %rcx
140504bb4: e8 97 f0 17 00              	callq	0x140683c50 <.text+0x682c50>
140504bb9: 48 8b 45 30                 	movq	0x30(%rbp), %rax
140504bbd: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140504bc1: 0f 10 45 20                 	movups	0x20(%rbp), %xmm0
140504bc5: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
140504bc9: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140504bcd: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140504bd1: e8 4a 93 17 00              	callq	0x14067df20 <.text+0x67cf20>
140504bd6: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140504bda: e8 71 d1 17 00              	callq	0x140681d50 <.text+0x680d50>
140504bdf: 48 89 c1                    	movq	%rax, %rcx
140504be2: e8 99 f0 17 00              	callq	0x140683c80 <.text+0x682c80>
140504be7: 41 89 c6                    	movl	%eax, %r14d
140504bea: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
140504bee: 48 85 d2                    	testq	%rdx, %rdx
140504bf1: 74 0f                       	je	0x140504c02 <.text+0x503c02>
140504bf3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140504bf9: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140504bfd: e8 1e f0 10 00              	callq	0x140613c20 <.text+0x612c20>
140504c02: 45 84 f6                    	testb	%r14b, %r14b
140504c05: 74 66                       	je	0x140504c6d <.text+0x503c6d>
140504c07: 8b 43 1c                    	movl	0x1c(%rbx), %eax
140504c0a: 48 8b 4b 10                 	movq	0x10(%rbx), %rcx
140504c0e: 89 01                       	movl	%eax, (%rcx)
140504c10: 48 8b 1b                    	movq	(%rbx), %rbx
140504c13: 4c 8b 73 30                 	movq	0x30(%rbx), %r14
140504c17: 4c 8b 7b 40                 	movq	0x40(%rbx), %r15
140504c1b: 4c 3b 73 20                 	cmpq	0x20(%rbx), %r14
140504c1f: 75 10                       	jne	0x140504c31 <.text+0x503c31>
140504c21: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
140504c25: 48 8d 15 a4 88 45 02        	leaq	0x24588a4(%rip), %rdx   # 0x14295d4d0
140504c2c: e8 ff d9 05 00              	callq	0x140562630 <.text+0x561630>
140504c31: 48 8b 43 28                 	movq	0x28(%rbx), %rax
140504c35: 4c 89 f1                    	movq	%r14, %rcx
140504c38: 48 c1 e1 04                 	shlq	$0x4, %rcx
140504c3c: 4c 89 3c 08                 	movq	%r15, (%rax,%rcx)
140504c40: c6 44 08 08 0b              	movb	$0xb, 0x8(%rax,%rcx)
140504c45: 49 ff c6                    	incq	%r14
140504c48: 4c 89 73 30                 	movq	%r14, 0x30(%rbx)
140504c4c: 48 ff 43 40                 	incq	0x40(%rbx)
140504c50: 8b 46 1c                    	movl	0x1c(%rsi), %eax
140504c53: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
140504c57: 89 01                       	movl	%eax, (%rcx)
140504c59: 48 8b 0e                    	movq	(%rsi), %rcx
140504c5c: 48 c7 45 c0 02 00 00 00     	movq	$0x2, -0x40(%rbp)
140504c64: 48 8d 55 c0                 	leaq	-0x40(%rbp), %rdx
140504c68: e8 63 ae 43 00              	callq	0x14093fad0 <.text+0x93ead0>
140504c6d: 48 8d 0d 6d 92 4e 02        	leaq	0x24e926d(%rip), %rcx   # 0x1429edee1
140504c74: ba 08 00 00 00              	movl	$0x8, %edx
140504c79: e8 02 88 69 00              	callq	0x140b9d480 <.text+0xb9c480>
140504c7e: 49 89 d0                    	movq	%rdx, %r8
140504c81: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140504c85: 48 89 c2                    	movq	%rax, %rdx
140504c88: e8 b3 60 87 00              	callq	0x140d7ad40 <.text+0xd79d40>
140504c8d: 4c 8b 45 c8                 	movq	-0x38(%rbp), %r8
140504c91: 4c 8b 4d d0                 	movq	-0x30(%rbp), %r9
140504c95: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140504c99: 48 89 fa                    	movq	%rdi, %rdx
140504c9c: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
140504ca0: e8 1b 73 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
140504ca5: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140504ca9: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
140504cad: e8 6e 92 17 00              	callq	0x14067df20 <.text+0x67cf20>
140504cb2: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140504cb6: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140504cba: 41 b0 06                    	movb	$0x6, %r8b
140504cbd: e8 ee 98 17 00              	callq	0x14067e5b0 <.text+0x67d5b0>
140504cc2: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140504cc6: e8 85 d0 17 00              	callq	0x140681d50 <.text+0x680d50>
140504ccb: 48 89 c1                    	movq	%rax, %rcx
140504cce: e8 ad ef 17 00              	callq	0x140683c80 <.text+0x682c80>
140504cd3: 89 c3                       	movl	%eax, %ebx
140504cd5: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
140504cd9: 48 85 d2                    	testq	%rdx, %rdx
140504cdc: 74 0f                       	je	0x140504ced <.text+0x503ced>
140504cde: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140504ce4: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140504ce8: e8 33 ef 10 00              	callq	0x140613c20 <.text+0x612c20>
140504ced: 84 db                       	testb	%bl, %bl
140504cef: 74 1d                       	je	0x140504d0e <.text+0x503d0e>
140504cf1: 8b 46 1c                    	movl	0x1c(%rsi), %eax
140504cf4: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
140504cf8: 89 01                       	movl	%eax, (%rcx)
140504cfa: 48 8b 0e                    	movq	(%rsi), %rcx
140504cfd: 48 c7 45 c0 05 00 00 00     	movq	$0x5, -0x40(%rbp)
140504d05: 48 8d 55 c0                 	leaq	-0x40(%rbp), %rdx
140504d09: e8 c2 ad 43 00              	callq	0x14093fad0 <.text+0x93ead0>
140504d0e: 48 8d 0d d4 91 4e 02        	leaq	0x24e91d4(%rip), %rcx   # 0x1429edee9
140504d15: ba 09 00 00 00              	movl	$0x9, %edx
140504d1a: e8 61 87 69 00              	callq	0x140b9d480 <.text+0xb9c480>
140504d1f: 49 89 d0                    	movq	%rdx, %r8
140504d22: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140504d26: 48 89 c2                    	movq	%rax, %rdx
140504d29: e8 12 60 87 00              	callq	0x140d7ad40 <.text+0xd79d40>
140504d2e: 4c 8b 45 c8                 	movq	-0x38(%rbp), %r8
140504d32: 4c 8b 4d d0                 	movq	-0x30(%rbp), %r9
140504d36: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140504d3a: 48 89 fa                    	movq	%rdi, %rdx
140504d3d: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
140504d41: e8 7a 72 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
140504d46: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140504d4a: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
140504d4e: e8 cd 91 17 00              	callq	0x14067df20 <.text+0x67cf20>
140504d53: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140504d57: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140504d5b: 41 b0 06                    	movb	$0x6, %r8b
140504d5e: e8 4d 98 17 00              	callq	0x14067e5b0 <.text+0x67d5b0>
140504d63: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140504d67: e8 e4 cf 17 00              	callq	0x140681d50 <.text+0x680d50>
140504d6c: 48 89 c1                    	movq	%rax, %rcx
140504d6f: e8 0c ef 17 00              	callq	0x140683c80 <.text+0x682c80>
140504d74: 89 c3                       	movl	%eax, %ebx
140504d76: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
140504d7a: 48 85 d2                    	testq	%rdx, %rdx
140504d7d: 74 0f                       	je	0x140504d8e <.text+0x503d8e>
140504d7f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140504d85: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140504d89: e8 92 ee 10 00              	callq	0x140613c20 <.text+0x612c20>
140504d8e: 84 db                       	testb	%bl, %bl
140504d90: 0f 84 90 00 00 00           	je	0x140504e26 <.text+0x503e26>
140504d96: 8b 46 1c                    	movl	0x1c(%rsi), %eax
140504d99: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
140504d9d: 89 01                       	movl	%eax, (%rcx)
140504d9f: 48 8b 36                    	movq	(%rsi), %rsi
140504da2: 0f b6 05 60 fe 12 03        	movzbl	0x312fe60(%rip), %eax   # 0x143634c09
140504da9: b9 48 00 00 00              	movl	$0x48, %ecx
140504dae: ba 08 00 00 00              	movl	$0x8, %edx
140504db3: e8 58 ee 10 00              	callq	0x140613c10 <.text+0x612c10>
140504db8: 48 85 c0                    	testq	%rax, %rax
140504dbb: 74 7a                       	je	0x140504e37 <.text+0x503e37>
140504dbd: 0f 57 c0                    	xorps	%xmm0, %xmm0
140504dc0: 0f 11 45 dd                 	movups	%xmm0, -0x23(%rbp)
140504dc4: 0f 11 45 d4                 	movups	%xmm0, -0x2c(%rbp)
140504dc8: 0f 11 45 c4                 	movups	%xmm0, -0x3c(%rbp)
140504dcc: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
140504dd3: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
140504ddb: c6 40 10 00                 	movb	$0x0, 0x10(%rax)
140504ddf: c7 40 14 00 00 00 00        	movl	$0x0, 0x14(%rax)
140504de6: 0f 10 45 c0                 	movups	-0x40(%rbp), %xmm0
140504dea: 0f 10 4d d0                 	movups	-0x30(%rbp), %xmm1
140504dee: 0f 10 55 dd                 	movups	-0x23(%rbp), %xmm2
140504df2: 0f 11 50 35                 	movups	%xmm2, 0x35(%rax)
140504df6: 0f 11 48 28                 	movups	%xmm1, 0x28(%rax)
140504dfa: 0f 11 40 18                 	movups	%xmm0, 0x18(%rax)
140504dfe: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
140504e06: 48 c7 45 d0 00 00 00 00     	movq	$0x0, -0x30(%rbp)
140504e0e: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140504e12: c6 45 f8 00                 	movb	$0x0, -0x8(%rbp)
140504e16: c6 45 09 00                 	movb	$0x0, 0x9(%rbp)
140504e1a: 48 8d 55 c0                 	leaq	-0x40(%rbp), %rdx
140504e1e: 48 89 f1                    	movq	%rsi, %rcx
140504e21: e8 aa ac 43 00              	callq	0x14093fad0 <.text+0x93ead0>
140504e26: 90                          	nop
140504e27: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140504e2e: 5b                          	popq	%rbx
140504e2f: 5f                          	popq	%rdi
140504e30: 5e                          	popq	%rsi
140504e31: 41 5e                       	popq	%r14
140504e33: 41 5f                       	popq	%r15
140504e35: 5d                          	popq	%rbp
140504e36: c3                          	retq
140504e37: b9 08 00 00 00              	movl	$0x8, %ecx
140504e3c: ba 48 00 00 00              	movl	$0x48, %edx
140504e41: e8 ba 41 3d 02              	callq	0x1428d9000 <.text+0x28d8000>
140504e46: cc                          	int3
140504e47: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140504e50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140504e55: 55                          	pushq	%rbp
140504e56: 41 57                       	pushq	%r15
140504e58: 41 56                       	pushq	%r14
140504e5a: 56                          	pushq	%rsi
140504e5b: 57                          	pushq	%rdi
140504e5c: 53                          	pushq	%rbx
140504e5d: 48 83 ec 28                 	subq	$0x28, %rsp
140504e61: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140504e68: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
140504e6c: 48 85 d2                    	testq	%rdx, %rdx
140504e6f: 74 0f                       	je	0x140504e80 <.text+0x503e80>
140504e71: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140504e77: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140504e7b: e8 a0 ed 10 00              	callq	0x140613c20 <.text+0x612c20>
140504e80: 90                          	nop
140504e81: 48 83 c4 28                 	addq	$0x28, %rsp
140504e85: 5b                          	popq	%rbx
140504e86: 5f                          	popq	%rdi
140504e87: 5e                          	popq	%rsi
140504e88: 41 5e                       	popq	%r14
140504e8a: 41 5f                       	popq	%r15
140504e8c: 5d                          	popq	%rbp
140504e8d: c3                          	retq
140504e8e: 66 90                       	nop
140504e90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140504e95: 55                          	pushq	%rbp
140504e96: 41 57                       	pushq	%r15
140504e98: 41 56                       	pushq	%r14
140504e9a: 56                          	pushq	%rsi
140504e9b: 57                          	pushq	%rdi
140504e9c: 53                          	pushq	%rbx
140504e9d: 48 83 ec 28                 	subq	$0x28, %rsp
140504ea1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140504ea8: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
140504eac: 48 85 d2                    	testq	%rdx, %rdx
140504eaf: 74 0f                       	je	0x140504ec0 <.text+0x503ec0>
140504eb1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140504eb7: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140504ebb: e8 60 ed 10 00              	callq	0x140613c20 <.text+0x612c20>
140504ec0: 90                          	nop
140504ec1: 48 83 c4 28                 	addq	$0x28, %rsp
140504ec5: 5b                          	popq	%rbx
140504ec6: 5f                          	popq	%rdi
140504ec7: 5e                          	popq	%rsi
140504ec8: 41 5e                       	popq	%r14
140504eca: 41 5f                       	popq	%r15
140504ecc: 5d                          	popq	%rbp
140504ecd: c3                          	retq
140504ece: 66 90                       	nop
140504ed0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140504ed5: 55                          	pushq	%rbp
140504ed6: 41 57                       	pushq	%r15
140504ed8: 41 56                       	pushq	%r14
140504eda: 56                          	pushq	%rsi
140504edb: 57                          	pushq	%rdi
140504edc: 53                          	pushq	%rbx
140504edd: 48 83 ec 28                 	subq	$0x28, %rsp
140504ee1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140504ee8: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
140504eec: 48 85 d2                    	testq	%rdx, %rdx
140504eef: 74 0f                       	je	0x140504f00 <.text+0x503f00>
140504ef1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140504ef7: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140504efb: e8 20 ed 10 00              	callq	0x140613c20 <.text+0x612c20>
140504f00: 90                          	nop
140504f01: 48 83 c4 28                 	addq	$0x28, %rsp
140504f05: 5b                          	popq	%rbx
140504f06: 5f                          	popq	%rdi
140504f07: 5e                          	popq	%rsi
140504f08: 41 5e                       	popq	%r14
140504f0a: 41 5f                       	popq	%r15
140504f0c: 5d                          	popq	%rbp
140504f0d: c3                          	retq
140504f0e: cc                          	int3
140504f0f: cc                          	int3
140504f10: 41 56                       	pushq	%r14
140504f12: 56                          	pushq	%rsi
140504f13: 57                          	pushq	%rdi
140504f14: 53                          	pushq	%rbx
140504f15: 48 81 ec a8 00 00 00        	subq	$0xa8, %rsp
140504f1c: 48 89 d7                    	movq	%rdx, %rdi
140504f1f: 48 89 ce                    	movq	%rcx, %rsi
140504f22: 48 8d 05 ef 8e 4e 02        	leaq	0x24e8eef(%rip), %rax   # 0x1429ede18
140504f29: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140504f2e: 48 c7 44 24 28 0f 00 00 00  	movq	$0xf, 0x28(%rsp)
140504f37: 4c 8d 05 b4 8f 4e 02        	leaq	0x24e8fb4(%rip), %r8    # 0x1429edef2
140504f3e: 48 8d 5c 24 30              	leaq	0x30(%rsp), %rbx
140504f43: 41 b9 03 00 00 00           	movl	$0x3, %r9d
140504f49: 48 89 d9                    	movq	%rbx, %rcx
140504f4c: e8 7f 96 e2 ff              	callq	0x14032e5d0 <.text+0x32d5d0>
140504f51: 48 89 d9                    	movq	%rbx, %rcx
140504f54: e8 f7 cd 17 00              	callq	0x140681d50 <.text+0x680d50>
140504f59: 48 89 c1                    	movq	%rax, %rcx
140504f5c: e8 1f ed 17 00              	callq	0x140683c80 <.text+0x682c80>
140504f61: 84 c0                       	testb	%al, %al
140504f63: 74 24                       	je	0x140504f89 <.text+0x503f89>
140504f65: 8b 46 1c                    	movl	0x1c(%rsi), %eax
140504f68: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
140504f6c: 89 01                       	movl	%eax, (%rcx)
140504f6e: 48 8b 0e                    	movq	(%rsi), %rcx
140504f71: c6 44 24 38 00              	movb	$0x0, 0x38(%rsp)
140504f76: 48 c7 44 24 30 08 00 00 00  	movq	$0x8, 0x30(%rsp)
140504f7f: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
140504f84: e8 47 ab 43 00              	callq	0x14093fad0 <.text+0x93ead0>
140504f89: 48 8d 05 97 8e 4e 02        	leaq	0x24e8e97(%rip), %rax   # 0x1429ede27
140504f90: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140504f95: 48 c7 44 24 28 12 00 00 00  	movq	$0x12, 0x28(%rsp)
140504f9e: 4c 8d 05 50 8f 4e 02        	leaq	0x24e8f50(%rip), %r8    # 0x1429edef5
140504fa5: 48 8d 5c 24 30              	leaq	0x30(%rsp), %rbx
140504faa: 41 b9 08 00 00 00           	movl	$0x8, %r9d
140504fb0: 48 89 d9                    	movq	%rbx, %rcx
140504fb3: 48 89 fa                    	movq	%rdi, %rdx
140504fb6: e8 15 96 e2 ff              	callq	0x14032e5d0 <.text+0x32d5d0>
140504fbb: 4c 8d b4 24 90 00 00 00     	leaq	0x90(%rsp), %r14
140504fc3: 4c 89 f1                    	movq	%r14, %rcx
140504fc6: 48 89 da                    	movq	%rbx, %rdx
140504fc9: 41 b0 03                    	movb	$0x3, %r8b
140504fcc: e8 df 95 17 00              	callq	0x14067e5b0 <.text+0x67d5b0>
140504fd1: 4c 89 f1                    	movq	%r14, %rcx
140504fd4: e8 77 cd 17 00              	callq	0x140681d50 <.text+0x680d50>
140504fd9: 48 89 c1                    	movq	%rax, %rcx
140504fdc: e8 9f ec 17 00              	callq	0x140683c80 <.text+0x682c80>
140504fe1: 84 c0                       	testb	%al, %al
140504fe3: 74 24                       	je	0x140505009 <.text+0x504009>
140504fe5: 8b 46 1c                    	movl	0x1c(%rsi), %eax
140504fe8: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
140504fec: 89 01                       	movl	%eax, (%rcx)
140504fee: 48 8b 0e                    	movq	(%rsi), %rcx
140504ff1: c6 44 24 38 03              	movb	$0x3, 0x38(%rsp)
140504ff6: 48 c7 44 24 30 07 00 00 00  	movq	$0x7, 0x30(%rsp)
140504fff: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
140505004: e8 c7 aa 43 00              	callq	0x14093fad0 <.text+0x93ead0>
140505009: 48 8d 05 f1 8e 4e 02        	leaq	0x24e8ef1(%rip), %rax   # 0x1429edf01
140505010: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140505015: 48 c7 44 24 28 0e 00 00 00  	movq	$0xe, 0x28(%rsp)
14050501e: 4c 8d 05 d8 8e 4e 02        	leaq	0x24e8ed8(%rip), %r8    # 0x1429edefd
140505025: 48 8d 5c 24 30              	leaq	0x30(%rsp), %rbx
14050502a: 41 b9 04 00 00 00           	movl	$0x4, %r9d
140505030: 48 89 d9                    	movq	%rbx, %rcx
140505033: 48 89 fa                    	movq	%rdi, %rdx
140505036: e8 95 95 e2 ff              	callq	0x14032e5d0 <.text+0x32d5d0>
14050503b: 48 89 d9                    	movq	%rbx, %rcx
14050503e: e8 0d cd 17 00              	callq	0x140681d50 <.text+0x680d50>
140505043: 48 89 c1                    	movq	%rax, %rcx
140505046: e8 35 ec 17 00              	callq	0x140683c80 <.text+0x682c80>
14050504b: 84 c0                       	testb	%al, %al
14050504d: 74 1f                       	je	0x14050506e <.text+0x50406e>
14050504f: 8b 46 1c                    	movl	0x1c(%rsi), %eax
140505052: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
140505056: 89 01                       	movl	%eax, (%rcx)
140505058: 48 8b 0e                    	movq	(%rsi), %rcx
14050505b: 48 c7 44 24 30 04 00 00 00  	movq	$0x4, 0x30(%rsp)
140505064: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
140505069: e8 62 aa 43 00              	callq	0x14093fad0 <.text+0x93ead0>
14050506e: 90                          	nop
14050506f: 48 81 c4 a8 00 00 00        	addq	$0xa8, %rsp
140505076: 5b                          	popq	%rbx
140505077: 5f                          	popq	%rdi
140505078: 5e                          	popq	%rsi
140505079: 41 5e                       	popq	%r14
14050507b: c3                          	retq
14050507c: cc                          	int3
14050507d: cc                          	int3
14050507e: cc                          	int3
14050507f: cc                          	int3
140505080: 55                          	pushq	%rbp
140505081: 56                          	pushq	%rsi
140505082: 57                          	pushq	%rdi
140505083: 53                          	pushq	%rbx
140505084: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
14050508b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140505093: 48 c7 45 40 fe ff ff ff     	movq	$-0x2, 0x40(%rbp)
14050509b: 48 89 d7                    	movq	%rdx, %rdi
14050509e: 48 89 ce                    	movq	%rcx, %rsi
1405050a1: 48 8d 0d 67 8e 4e 02        	leaq	0x24e8e67(%rip), %rcx   # 0x1429edf0f
1405050a8: ba 0c 00 00 00              	movl	$0xc, %edx
1405050ad: e8 ce 83 69 00              	callq	0x140b9d480 <.text+0xb9c480>
1405050b2: 49 89 d0                    	movq	%rdx, %r8
1405050b5: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
1405050b9: 48 89 c2                    	movq	%rax, %rdx
1405050bc: e8 7f 5c 87 00              	callq	0x140d7ad40 <.text+0xd79d40>
1405050c1: 4c 8b 45 a8                 	movq	-0x58(%rbp), %r8
1405050c5: 4c 8b 4d b0                 	movq	-0x50(%rbp), %r9
1405050c9: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
1405050cd: 48 89 fa                    	movq	%rdi, %rdx
1405050d0: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
1405050d4: e8 e7 6e 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
1405050d9: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
1405050dd: e8 5e cc 17 00              	callq	0x140681d40 <.text+0x680d40>
1405050e2: f3 0f 10 0d ae 8a 4e 02     	movss	0x24e8aae(%rip), %xmm1  # 0x1429edb98
1405050ea: 48 89 c1                    	movq	%rax, %rcx
1405050ed: e8 5e 12 c0 ff              	callq	0x140106350 <.text+0x105350>
1405050f2: 48 8b 45 30                 	movq	0x30(%rbp), %rax
1405050f6: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1405050fa: 0f 10 45 20                 	movups	0x20(%rbp), %xmm0
1405050fe: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140505102: 48 89 e9                    	movq	%rbp, %rcx
140505105: e8 46 cc 17 00              	callq	0x140681d50 <.text+0x680d50>
14050510a: 48 89 c1                    	movq	%rax, %rcx
14050510d: e8 6e eb 17 00              	callq	0x140683c80 <.text+0x682c80>
140505112: 89 c3                       	movl	%eax, %ebx
140505114: 48 8b 55 a0                 	movq	-0x60(%rbp), %rdx
140505118: 48 85 d2                    	testq	%rdx, %rdx
14050511b: 74 0f                       	je	0x14050512c <.text+0x50412c>
14050511d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140505123: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140505127: e8 f4 ea 10 00              	callq	0x140613c20 <.text+0x612c20>
14050512c: 84 db                       	testb	%bl, %bl
14050512e: 74 1d                       	je	0x14050514d <.text+0x50414d>
140505130: 8b 46 1c                    	movl	0x1c(%rsi), %eax
140505133: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
140505137: 89 01                       	movl	%eax, (%rcx)
140505139: 48 8b 0e                    	movq	(%rsi), %rcx
14050513c: 48 c7 45 a0 09 00 00 00     	movq	$0x9, -0x60(%rbp)
140505144: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140505148: e8 83 a9 43 00              	callq	0x14093fad0 <.text+0x93ead0>
14050514d: 90                          	nop
14050514e: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140505155: 5b                          	popq	%rbx
140505156: 5f                          	popq	%rdi
140505157: 5e                          	popq	%rsi
140505158: 5d                          	popq	%rbp
140505159: c3                          	retq
14050515a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140505160: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140505165: 55                          	pushq	%rbp
140505166: 56                          	pushq	%rsi
140505167: 57                          	pushq	%rdi
140505168: 53                          	pushq	%rbx
140505169: 48 83 ec 28                 	subq	$0x28, %rsp
14050516d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140505174: 48 8b 55 a0                 	movq	-0x60(%rbp), %rdx
140505178: 48 85 d2                    	testq	%rdx, %rdx
14050517b: 74 0f                       	je	0x14050518c <.text+0x50418c>
14050517d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140505183: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140505187: e8 94 ea 10 00              	callq	0x140613c20 <.text+0x612c20>
14050518c: 90                          	nop
14050518d: 48 83 c4 28                 	addq	$0x28, %rsp
140505191: 5b                          	popq	%rbx
140505192: 5f                          	popq	%rdi
140505193: 5e                          	popq	%rsi
140505194: 5d                          	popq	%rbp
140505195: c3                          	retq
140505196: cc                          	int3
140505197: cc                          	int3
140505198: cc                          	int3
140505199: cc                          	int3
14050519a: cc                          	int3
14050519b: cc                          	int3
14050519c: cc                          	int3
14050519d: cc                          	int3
14050519e: cc                          	int3
14050519f: cc                          	int3
1405051a0: 55                          	pushq	%rbp
1405051a1: 41 56                       	pushq	%r14
1405051a3: 56                          	pushq	%rsi
1405051a4: 57                          	pushq	%rdi
1405051a5: 53                          	pushq	%rbx
1405051a6: 48 81 ec d0 00 00 00        	subq	$0xd0, %rsp
1405051ad: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1405051b5: 48 c7 45 48 fe ff ff ff     	movq	$-0x2, 0x48(%rbp)
1405051bd: 4c 89 c7                    	movq	%r8, %rdi
1405051c0: 48 89 d6                    	movq	%rdx, %rsi
1405051c3: 48 89 cb                    	movq	%rcx, %rbx
1405051c6: 48 8d 0d 4e 8d 4e 02        	leaq	0x24e8d4e(%rip), %rcx   # 0x1429edf1b
1405051cd: ba 09 00 00 00              	movl	$0x9, %edx
1405051d2: e8 a9 82 69 00              	callq	0x140b9d480 <.text+0xb9c480>
1405051d7: 49 89 d0                    	movq	%rdx, %r8
1405051da: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1405051de: 48 89 c2                    	movq	%rax, %rdx
1405051e1: e8 5a 5b 87 00              	callq	0x140d7ad40 <.text+0xd79d40>
1405051e6: 4c 8b 45 d0                 	movq	-0x30(%rbp), %r8
1405051ea: 4c 8b 4d d8                 	movq	-0x28(%rbp), %r9
1405051ee: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
1405051f2: 48 89 fa                    	movq	%rdi, %rdx
1405051f5: 4c 89 45 40                 	movq	%r8, 0x40(%rbp)
1405051f9: e8 c2 6d 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
1405051fe: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
140505202: 48 8d 55 28                 	leaq	0x28(%rbp), %rdx
140505206: 41 b0 09                    	movb	$0x9, %r8b
140505209: e8 a2 93 17 00              	callq	0x14067e5b0 <.text+0x67d5b0>
14050520e: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
140505212: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
140505216: 41 b0 12                    	movb	$0x12, %r8b
140505219: e8 b2 94 17 00              	callq	0x14067e6d0 <.text+0x67d6d0>
14050521e: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
140505222: e8 29 cc 17 00              	callq	0x140681e50 <.text+0x680e50>
140505227: 48 89 c1                    	movq	%rax, %rcx
14050522a: e8 21 ea 17 00              	callq	0x140683c50 <.text+0x682c50>
14050522f: 48 89 c1                    	movq	%rax, %rcx
140505232: e8 49 ea 17 00              	callq	0x140683c80 <.text+0x682c80>
140505237: 41 89 c6                    	movl	%eax, %r14d
14050523a: 48 8b 55 c8                 	movq	-0x38(%rbp), %rdx
14050523e: 48 85 d2                    	testq	%rdx, %rdx
140505241: 74 0f                       	je	0x140505252 <.text+0x504252>
140505243: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140505249: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
14050524d: e8 ce e9 10 00              	callq	0x140613c20 <.text+0x612c20>
140505252: 45 84 f6                    	testb	%r14b, %r14b
140505255: 74 1d                       	je	0x140505274 <.text+0x504274>
140505257: 8b 43 1c                    	movl	0x1c(%rbx), %eax
14050525a: 48 8b 4b 10                 	movq	0x10(%rbx), %rcx
14050525e: 89 01                       	movl	%eax, (%rcx)
140505260: 48 8b 0b                    	movq	(%rbx), %rcx
140505263: 48 c7 45 c8 03 00 00 00     	movq	$0x3, -0x38(%rbp)
14050526b: 48 8d 55 c8                 	leaq	-0x38(%rbp), %rdx
14050526f: e8 5c a8 43 00              	callq	0x14093fad0 <.text+0x93ead0>
140505274: 48 8d 0d a9 8c 4e 02        	leaq	0x24e8ca9(%rip), %rcx   # 0x1429edf24
14050527b: ba 09 00 00 00              	movl	$0x9, %edx
140505280: e8 fb 81 69 00              	callq	0x140b9d480 <.text+0xb9c480>
140505285: 49 89 d0                    	movq	%rdx, %r8
140505288: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
14050528c: 48 89 c2                    	movq	%rax, %rdx
14050528f: e8 ac 5a 87 00              	callq	0x140d7ad40 <.text+0xd79d40>
140505294: 4c 8b 45 d0                 	movq	-0x30(%rbp), %r8
140505298: 4c 8b 4d d8                 	movq	-0x28(%rbp), %r9
14050529c: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
1405052a0: 48 89 fa                    	movq	%rdi, %rdx
1405052a3: 4c 89 45 40                 	movq	%r8, 0x40(%rbp)
1405052a7: e8 14 6d 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
1405052ac: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
1405052b0: e8 9b ca 17 00              	callq	0x140681d50 <.text+0x680d50>
1405052b5: 48 89 c1                    	movq	%rax, %rcx
1405052b8: e8 c3 e9 17 00              	callq	0x140683c80 <.text+0x682c80>
1405052bd: 89 c3                       	movl	%eax, %ebx
1405052bf: 48 8b 55 c8                 	movq	-0x38(%rbp), %rdx
1405052c3: 48 85 d2                    	testq	%rdx, %rdx
1405052c6: 74 0f                       	je	0x1405052d7 <.text+0x5042d7>
1405052c8: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1405052ce: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
1405052d2: e8 49 e9 10 00              	callq	0x140613c20 <.text+0x612c20>
1405052d7: 84 db                       	testb	%bl, %bl
1405052d9: 74 34                       	je	0x14050530f <.text+0x50430f>
1405052db: 48 8b 7e 08                 	movq	0x8(%rsi), %rdi
1405052df: 48 85 ff                    	testq	%rdi, %rdi
1405052e2: 75 04                       	jne	0x1405052e8 <.text+0x5042e8>
1405052e4: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
1405052e8: 48 8b 07                    	movq	(%rdi), %rax
1405052eb: 48 8b 77 10                 	movq	0x10(%rdi), %rsi
1405052ef: 48 29 f0                    	subq	%rsi, %rax
1405052f2: 48 83 f8 07                 	cmpq	$0x7, %rax
1405052f6: 76 25                       	jbe	0x14050531d <.text+0x50431d>
1405052f8: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1405052fc: 48 8d 0d 0d 84 c4 ff        	leaq	-0x3b7bf3(%rip), %rcx   # 0x14014d710 <.text+0x14c710>
140505303: 48 89 0c 30                 	movq	%rcx, (%rax,%rsi)
140505307: 48 83 c6 08                 	addq	$0x8, %rsi
14050530b: 48 89 77 10                 	movq	%rsi, 0x10(%rdi)
14050530f: 48 81 c4 d0 00 00 00        	addq	$0xd0, %rsp
140505316: 5b                          	popq	%rbx
140505317: 5f                          	popq	%rdi
140505318: 5e                          	popq	%rsi
140505319: 41 5e                       	popq	%r14
14050531b: 5d                          	popq	%rbp
14050531c: c3                          	retq
14050531d: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140505326: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14050532c: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140505332: 48 89 f9                    	movq	%rdi, %rcx
140505335: 48 89 f2                    	movq	%rsi, %rdx
140505338: e8 a3 d6 39 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14050533d: eb b9                       	jmp	0x1405052f8 <.text+0x5042f8>
14050533f: 90                          	nop
140505340: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140505345: 55                          	pushq	%rbp
140505346: 41 56                       	pushq	%r14
140505348: 56                          	pushq	%rsi
140505349: 57                          	pushq	%rdi
14050534a: 53                          	pushq	%rbx
14050534b: 48 83 ec 30                 	subq	$0x30, %rsp
14050534f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140505356: 48 8b 55 c8                 	movq	-0x38(%rbp), %rdx
14050535a: 48 85 d2                    	testq	%rdx, %rdx
14050535d: 74 0f                       	je	0x14050536e <.text+0x50436e>
14050535f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140505365: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140505369: e8 b2 e8 10 00              	callq	0x140613c20 <.text+0x612c20>
14050536e: 90                          	nop
14050536f: 48 83 c4 30                 	addq	$0x30, %rsp
140505373: 5b                          	popq	%rbx
140505374: 5f                          	popq	%rdi
140505375: 5e                          	popq	%rsi
140505376: 41 5e                       	popq	%r14
140505378: 5d                          	popq	%rbp
140505379: c3                          	retq
14050537a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140505380: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140505385: 55                          	pushq	%rbp
140505386: 41 56                       	pushq	%r14
140505388: 56                          	pushq	%rsi
140505389: 57                          	pushq	%rdi
14050538a: 53                          	pushq	%rbx
14050538b: 48 83 ec 30                 	subq	$0x30, %rsp
14050538f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140505396: 48 8b 55 c8                 	movq	-0x38(%rbp), %rdx
14050539a: 48 85 d2                    	testq	%rdx, %rdx
14050539d: 74 0f                       	je	0x1405053ae <.text+0x5043ae>
14050539f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1405053a5: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
1405053a9: e8 72 e8 10 00              	callq	0x140613c20 <.text+0x612c20>
1405053ae: 90                          	nop
1405053af: 48 83 c4 30                 	addq	$0x30, %rsp
1405053b3: 5b                          	popq	%rbx
1405053b4: 5f                          	popq	%rdi
1405053b5: 5e                          	popq	%rsi
1405053b6: 41 5e                       	popq	%r14
1405053b8: 5d                          	popq	%rbp
1405053b9: c3                          	retq


/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141278ed0: 55                          	pushq	%rbp
141278ed1: 41 56                       	pushq	%r14
141278ed3: 56                          	pushq	%rsi
141278ed4: 57                          	pushq	%rdi
141278ed5: 53                          	pushq	%rbx
141278ed6: 48 81 ec 70 01 00 00        	subq	$0x170, %rsp            # imm = 0x170
141278edd: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141278ee5: 48 c7 85 e8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xe8(%rbp)
141278ef0: 4c 89 cf                    	movq	%r9, %rdi
141278ef3: 4c 89 c3                    	movq	%r8, %rbx
141278ef6: 49 89 d6                    	movq	%rdx, %r14
141278ef9: 48 89 ce                    	movq	%rcx, %rsi
141278efc: 0f b6 05 06 bd 3b 02        	movzbl	0x23bbd06(%rip), %eax   # 0x143634c09
141278f03: b9 08 00 00 00              	movl	$0x8, %ecx
141278f08: ba 08 00 00 00              	movl	$0x8, %edx
141278f0d: e8 fe ac 39 ff              	callq	0x140613c10 <.text+0x612c10>
141278f12: 48 85 c0                    	testq	%rax, %rax
141278f15: 0f 84 0c 01 00 00           	je	0x141279027 <.text+0x1278027>
141278f1b: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
141278f22: 0f b6 05 e0 bc 3b 02        	movzbl	0x23bbce0(%rip), %eax   # 0x143634c09
141278f29: b9 0b 00 00 00              	movl	$0xb, %ecx
141278f2e: ba 01 00 00 00              	movl	$0x1, %edx
141278f33: e8 d8 ac 39 ff              	callq	0x140613c10 <.text+0x612c10>
141278f38: 48 85 c0                    	testq	%rax, %rax
141278f3b: 0f 84 f5 00 00 00           	je	0x141279036 <.text+0x1278036>
141278f41: 48 b9 74 65 6d 70 5f 73 70 68       	movabsq	$0x6870735f706d6574, %rcx # imm = 0x6870735F706D6574
141278f4b: 48 89 08                    	movq	%rcx, (%rax)
141278f4e: c7 40 07 68 65 72 65        	movl	$0x65726568, 0x7(%rax)  # imm = 0x65726568
141278f55: 48 c7 45 48 0b 00 00 00     	movq	$0xb, 0x48(%rbp)
141278f5d: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
141278f61: 48 c7 45 58 0b 00 00 00     	movq	$0xb, 0x58(%rbp)
141278f69: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141278f73: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141278f77: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
141278f7b: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
141278f7f: e8 2c 4a 62 ff              	callq	0x14089d9b0 <.text+0x89c9b0>
141278f84: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
141278f88: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
141278f8c: f3 0f 10 15 f0 c9 6a 01     	movss	0x16ac9f0(%rip), %xmm2  # 0x142925984
141278f94: e8 f7 4c 62 ff              	callq	0x14089dc90 <.text+0x89cc90>
141278f99: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
141278f9d: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
141278fa1: f3 0f 10 15 c7 7d 6b 01     	movss	0x16b7dc7(%rip), %xmm2  # 0x142930d70
141278fa9: e8 b2 4c 62 ff              	callq	0x14089dc60 <.text+0x89cc60>
141278fae: 41 8b 46 08                 	movl	0x8(%r14), %eax
141278fb2: 89 85 d8 00 00 00           	movl	%eax, 0xd8(%rbp)
141278fb8: 49 8b 06                    	movq	(%r14), %rax
141278fbb: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
141278fc2: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
141278fc6: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
141278fca: 4c 8d 85 d0 00 00 00        	leaq	0xd0(%rbp), %r8
141278fd1: e8 2a 4c 62 ff              	callq	0x14089dc00 <.text+0x89cc00>
141278fd6: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
141278fdd: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141278fe2: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
141278fe6: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
141278fea: 49 89 d8                    	movq	%rbx, %r8
141278fed: 49 89 f9                    	movq	%rdi, %r9
141278ff0: e8 5b 4d 62 ff              	callq	0x14089dd50 <.text+0x89cd50>
141278ff5: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
141278ff9: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
141279000: 48 89 01                    	movq	%rax, (%rcx)
141279003: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
14127900a: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
14127900e: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
141279016: 48 89 f0                    	movq	%rsi, %rax
141279019: 48 81 c4 70 01 00 00        	addq	$0x170, %rsp            # imm = 0x170
141279020: 5b                          	popq	%rbx
141279021: 5f                          	popq	%rdi
141279022: 5e                          	popq	%rsi
141279023: 41 5e                       	popq	%r14
141279025: 5d                          	popq	%rbp
141279026: c3                          	retq
141279027: b9 08 00 00 00              	movl	$0x8, %ecx
14127902c: ba 08 00 00 00              	movl	$0x8, %edx
141279031: e8 ca ff 65 01              	callq	0x1428d9000 <.text+0x28d8000>
141279036: 4c 8d 05 b3 a9 9a 01        	leaq	0x19aa9b3(%rip), %r8    # 0x142c239f0
14127903d: b9 01 00 00 00              	movl	$0x1, %ecx
141279042: ba 0b 00 00 00              	movl	$0xb, %edx
141279047: e8 97 ff 65 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14127904c: 0f 0b                       	ud2
14127904e: 66 90                       	nop
141279050: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141279055: 55                          	pushq	%rbp
141279056: 41 56                       	pushq	%r14
141279058: 56                          	pushq	%rsi
141279059: 57                          	pushq	%rdi
14127905a: 53                          	pushq	%rbx
14127905b: 48 83 ec 30                 	subq	$0x30, %rsp
14127905f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141279066: ba 08 00 00 00              	movl	$0x8, %edx
14127906b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141279071: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
141279078: e8 a3 ab 39 ff              	callq	0x140613c20 <.text+0x612c20>
14127907d: 90                          	nop
14127907e: 48 83 c4 30                 	addq	$0x30, %rsp
141279082: 5b                          	popq	%rbx
141279083: 5f                          	popq	%rdi
141279084: 5e                          	popq	%rsi
141279085: 41 5e                       	popq	%r14
141279087: 5d                          	popq	%rbp
141279088: c3                          	retq
141279089: cc                          	int3
14127908a: cc                          	int3
14127908b: cc                          	int3
14127908c: cc                          	int3
14127908d: cc                          	int3
14127908e: cc                          	int3
14127908f: cc                          	int3
141279090: 55                          	pushq	%rbp
141279091: 41 56                       	pushq	%r14
141279093: 56                          	pushq	%rsi
141279094: 57                          	pushq	%rdi
141279095: 53                          	pushq	%rbx
141279096: 48 81 ec 70 01 00 00        	subq	$0x170, %rsp            # imm = 0x170
14127909d: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1412790a5: 48 c7 85 e8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xe8(%rbp)
1412790b0: 4c 89 cf                    	movq	%r9, %rdi
1412790b3: 4c 89 c3                    	movq	%r8, %rbx
1412790b6: 49 89 d6                    	movq	%rdx, %r14
1412790b9: 48 89 ce                    	movq	%rcx, %rsi
1412790bc: 0f b6 05 46 bb 3b 02        	movzbl	0x23bbb46(%rip), %eax   # 0x143634c09
1412790c3: b9 08 00 00 00              	movl	$0x8, %ecx
1412790c8: ba 08 00 00 00              	movl	$0x8, %edx
1412790cd: e8 3e ab 39 ff              	callq	0x140613c10 <.text+0x612c10>
1412790d2: 48 85 c0                    	testq	%rax, %rax
1412790d5: 0f 84 0c 01 00 00           	je	0x1412791e7 <.text+0x12781e7>
1412790db: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1412790e2: 0f b6 05 20 bb 3b 02        	movzbl	0x23bbb20(%rip), %eax   # 0x143634c09
1412790e9: b9 0b 00 00 00              	movl	$0xb, %ecx
1412790ee: ba 01 00 00 00              	movl	$0x1, %edx
1412790f3: e8 18 ab 39 ff              	callq	0x140613c10 <.text+0x612c10>
1412790f8: 48 85 c0                    	testq	%rax, %rax
1412790fb: 0f 84 f5 00 00 00           	je	0x1412791f6 <.text+0x12781f6>
141279101: 48 b9 74 65 6d 70 5f 73 70 68       	movabsq	$0x6870735f706d6574, %rcx # imm = 0x6870735F706D6574
14127910b: 48 89 08                    	movq	%rcx, (%rax)
14127910e: c7 40 07 68 65 72 65        	movl	$0x65726568, 0x7(%rax)  # imm = 0x65726568
141279115: 48 c7 45 48 0b 00 00 00     	movq	$0xb, 0x48(%rbp)
14127911d: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
141279121: 48 c7 45 58 0b 00 00 00     	movq	$0xb, 0x58(%rbp)
141279129: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141279133: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141279137: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14127913b: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
14127913f: e8 6c 48 62 ff              	callq	0x14089d9b0 <.text+0x89c9b0>
141279144: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
141279148: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
14127914c: f3 0f 10 15 30 c8 6a 01     	movss	0x16ac830(%rip), %xmm2  # 0x142925984
141279154: e8 37 4b 62 ff              	callq	0x14089dc90 <.text+0x89cc90>
141279159: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14127915d: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
141279161: f3 0f 10 15 ff 3e 75 01     	movss	0x1753eff(%rip), %xmm2  # 0x1429cd068
141279169: e8 f2 4a 62 ff              	callq	0x14089dc60 <.text+0x89cc60>
14127916e: 41 8b 46 08                 	movl	0x8(%r14), %eax
141279172: 89 85 d8 00 00 00           	movl	%eax, 0xd8(%rbp)
141279178: 49 8b 06                    	movq	(%r14), %rax
14127917b: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
141279182: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
141279186: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
14127918a: 4c 8d 85 d0 00 00 00        	leaq	0xd0(%rbp), %r8
141279191: e8 6a 4a 62 ff              	callq	0x14089dc00 <.text+0x89cc00>
141279196: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
14127919d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1412791a2: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1412791a6: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
1412791aa: 49 89 d8                    	movq	%rbx, %r8
1412791ad: 49 89 f9                    	movq	%rdi, %r9
1412791b0: e8 9b 4b 62 ff              	callq	0x14089dd50 <.text+0x89cd50>
1412791b5: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
1412791b9: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
1412791c0: 48 89 01                    	movq	%rax, (%rcx)
1412791c3: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
1412791ca: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
1412791ce: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
1412791d6: 48 89 f0                    	movq	%rsi, %rax
1412791d9: 48 81 c4 70 01 00 00        	addq	$0x170, %rsp            # imm = 0x170
1412791e0: 5b                          	popq	%rbx
1412791e1: 5f                          	popq	%rdi
1412791e2: 5e                          	popq	%rsi
1412791e3: 41 5e                       	popq	%r14
1412791e5: 5d                          	popq	%rbp
1412791e6: c3                          	retq
1412791e7: b9 08 00 00 00              	movl	$0x8, %ecx
1412791ec: ba 08 00 00 00              	movl	$0x8, %edx
1412791f1: e8 0a fe 65 01              	callq	0x1428d9000 <.text+0x28d8000>
1412791f6: 4c 8d 05 f3 a7 9a 01        	leaq	0x19aa7f3(%rip), %r8    # 0x142c239f0
1412791fd: b9 01 00 00 00              	movl	$0x1, %ecx
141279202: ba 0b 00 00 00              	movl	$0xb, %edx
141279207: e8 d7 fd 65 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14127920c: 0f 0b                       	ud2
14127920e: 66 90                       	nop
141279210: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141279215: 55                          	pushq	%rbp
141279216: 41 56                       	pushq	%r14
141279218: 56                          	pushq	%rsi
141279219: 57                          	pushq	%rdi
14127921a: 53                          	pushq	%rbx
14127921b: 48 83 ec 30                 	subq	$0x30, %rsp
14127921f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141279226: ba 08 00 00 00              	movl	$0x8, %edx
14127922b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141279231: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
141279238: e8 e3 a9 39 ff              	callq	0x140613c20 <.text+0x612c20>
14127923d: 90                          	nop
14127923e: 48 83 c4 30                 	addq	$0x30, %rsp
141279242: 5b                          	popq	%rbx
141279243: 5f                          	popq	%rdi
141279244: 5e                          	popq	%rsi
141279245: 41 5e                       	popq	%r14
141279247: 5d                          	popq	%rbp
141279248: c3                          	retq
141279249: cc                          	int3
14127924a: cc                          	int3
14127924b: cc                          	int3
14127924c: cc                          	int3
14127924d: cc                          	int3
14127924e: cc                          	int3
14127924f: cc                          	int3
141279250: 55                          	pushq	%rbp
141279251: 41 56                       	pushq	%r14
141279253: 56                          	pushq	%rsi
141279254: 57                          	pushq	%rdi
141279255: 53                          	pushq	%rbx
141279256: 48 81 ec 70 01 00 00        	subq	$0x170, %rsp            # imm = 0x170
14127925d: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141279265: 48 c7 85 e8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xe8(%rbp)
141279270: 4c 89 cf                    	movq	%r9, %rdi
141279273: 4c 89 c3                    	movq	%r8, %rbx
141279276: 49 89 d6                    	movq	%rdx, %r14
141279279: 48 89 ce                    	movq	%rcx, %rsi
14127927c: 0f b6 05 86 b9 3b 02        	movzbl	0x23bb986(%rip), %eax   # 0x143634c09
141279283: b9 08 00 00 00              	movl	$0x8, %ecx
141279288: ba 08 00 00 00              	movl	$0x8, %edx
14127928d: e8 7e a9 39 ff              	callq	0x140613c10 <.text+0x612c10>
141279292: 48 85 c0                    	testq	%rax, %rax
141279295: 0f 84 0c 01 00 00           	je	0x1412793a7 <.text+0x12783a7>
14127929b: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1412792a2: 0f b6 05 60 b9 3b 02        	movzbl	0x23bb960(%rip), %eax   # 0x143634c09
1412792a9: b9 0b 00 00 00              	movl	$0xb, %ecx
1412792ae: ba 01 00 00 00              	movl	$0x1, %edx
1412792b3: e8 58 a9 39 ff              	callq	0x140613c10 <.text+0x612c10>
1412792b8: 48 85 c0                    	testq	%rax, %rax
1412792bb: 0f 84 f5 00 00 00           	je	0x1412793b6 <.text+0x12783b6>
1412792c1: 48 b9 74 65 6d 70 5f 73 70 68       	movabsq	$0x6870735f706d6574, %rcx # imm = 0x6870735F706D6574
1412792cb: 48 89 08                    	movq	%rcx, (%rax)
1412792ce: c7 40 07 68 65 72 65        	movl	$0x65726568, 0x7(%rax)  # imm = 0x65726568
1412792d5: 48 c7 45 48 0b 00 00 00     	movq	$0xb, 0x48(%rbp)
1412792dd: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1412792e1: 48 c7 45 58 0b 00 00 00     	movq	$0xb, 0x58(%rbp)
1412792e9: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1412792f3: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1412792f7: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1412792fb: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
1412792ff: e8 ac 46 62 ff              	callq	0x14089d9b0 <.text+0x89c9b0>
141279304: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
141279308: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
14127930c: f3 0f 10 15 70 c6 6a 01     	movss	0x16ac670(%rip), %xmm2  # 0x142925984
141279314: e8 77 49 62 ff              	callq	0x14089dc90 <.text+0x89cc90>
141279319: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14127931d: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
141279321: f3 0f 10 15 4f 74 72 01     	movss	0x172744f(%rip), %xmm2  # 0x1429a0778
141279329: e8 32 49 62 ff              	callq	0x14089dc60 <.text+0x89cc60>
14127932e: 41 8b 46 08                 	movl	0x8(%r14), %eax
141279332: 89 85 d8 00 00 00           	movl	%eax, 0xd8(%rbp)
141279338: 49 8b 06                    	movq	(%r14), %rax
14127933b: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
141279342: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
141279346: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
14127934a: 4c 8d 85 d0 00 00 00        	leaq	0xd0(%rbp), %r8
141279351: e8 aa 48 62 ff              	callq	0x14089dc00 <.text+0x89cc00>
141279356: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
14127935d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141279362: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
141279366: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
14127936a: 49 89 d8                    	movq	%rbx, %r8
14127936d: 49 89 f9                    	movq	%rdi, %r9
141279370: e8 db 49 62 ff              	callq	0x14089dd50 <.text+0x89cd50>
141279375: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
141279379: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
141279380: 48 89 01                    	movq	%rax, (%rcx)
141279383: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
14127938a: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
14127938e: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
141279396: 48 89 f0                    	movq	%rsi, %rax
141279399: 48 81 c4 70 01 00 00        	addq	$0x170, %rsp            # imm = 0x170
1412793a0: 5b                          	popq	%rbx
1412793a1: 5f                          	popq	%rdi
1412793a2: 5e                          	popq	%rsi
1412793a3: 41 5e                       	popq	%r14
1412793a5: 5d                          	popq	%rbp
1412793a6: c3                          	retq
1412793a7: b9 08 00 00 00              	movl	$0x8, %ecx
1412793ac: ba 08 00 00 00              	movl	$0x8, %edx
1412793b1: e8 4a fc 65 01              	callq	0x1428d9000 <.text+0x28d8000>
1412793b6: 4c 8d 05 33 a6 9a 01        	leaq	0x19aa633(%rip), %r8    # 0x142c239f0
1412793bd: b9 01 00 00 00              	movl	$0x1, %ecx
1412793c2: ba 0b 00 00 00              	movl	$0xb, %edx
1412793c7: e8 17 fc 65 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1412793cc: 0f 0b                       	ud2
1412793ce: 66 90                       	nop
1412793d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412793d5: 55                          	pushq	%rbp
1412793d6: 41 56                       	pushq	%r14
1412793d8: 56                          	pushq	%rsi
1412793d9: 57                          	pushq	%rdi
1412793da: 53                          	pushq	%rbx
1412793db: 48 83 ec 30                 	subq	$0x30, %rsp
1412793df: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412793e6: ba 08 00 00 00              	movl	$0x8, %edx
1412793eb: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1412793f1: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
1412793f8: e8 23 a8 39 ff              	callq	0x140613c20 <.text+0x612c20>
1412793fd: 90                          	nop
1412793fe: 48 83 c4 30                 	addq	$0x30, %rsp
141279402: 5b                          	popq	%rbx
141279403: 5f                          	popq	%rdi
141279404: 5e                          	popq	%rsi
141279405: 41 5e                       	popq	%r14
141279407: 5d                          	popq	%rbp
141279408: c3                          	retq
141279409: cc                          	int3
14127940a: cc                          	int3
14127940b: cc                          	int3
14127940c: cc                          	int3
14127940d: cc                          	int3
14127940e: cc                          	int3
14127940f: cc                          	int3

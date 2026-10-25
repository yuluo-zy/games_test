
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140493120: 55                          	pushq	%rbp
140493121: 56                          	pushq	%rsi
140493122: 57                          	pushq	%rdi
140493123: 48 81 ec f0 01 00 00        	subq	$0x1f0, %rsp            # imm = 0x1F0
14049312a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140493132: 48 c7 85 68 01 00 00 fe ff ff ff    	movq	$-0x2, 0x168(%rbp)
14049313d: 48 89 ce                    	movq	%rcx, %rsi
140493140: 4c 8d 05 41 bf 54 02        	leaq	0x254bf41(%rip), %r8    # 0x1429df088
140493147: 48 8d 7d 10                 	leaq	0x10(%rbp), %rdi
14049314b: 48 89 f9                    	movq	%rdi, %rcx
14049314e: b2 11                       	movb	$0x11, %dl
140493150: e8 db fb c7 ff              	callq	0x140112d30 <.text+0x111d30>
140493155: 48 89 f1                    	movq	%rsi, %rcx
140493158: 48 89 fa                    	movq	%rdi, %rdx
14049315b: e8 90 45 e9 ff              	callq	0x1403276f0 <.text+0x3266f0>
140493160: 4c 8d 05 39 bf 54 02        	leaq	0x254bf39(%rip), %r8    # 0x1429df0a0
140493167: 48 89 f9                    	movq	%rdi, %rcx
14049316a: b2 2a                       	movb	$0x2a, %dl
14049316c: e8 cf fb e8 ff              	callq	0x140322d40 <.text+0x321d40>
140493171: 48 89 f1                    	movq	%rsi, %rcx
140493174: 48 89 fa                    	movq	%rdi, %rdx
140493177: e8 74 45 e9 ff              	callq	0x1403276f0 <.text+0x3266f0>
14049317c: 48 89 f9                    	movq	%rdi, %rcx
14049317f: e8 dc b7 0b 00              	callq	0x14054e960 <.text+0x54d960>
140493184: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140493188: 48 89 fa                    	movq	%rdi, %rdx
14049318b: e8 00 a1 07 00              	callq	0x14050d290 <.text+0x50c290>
140493190: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140493197: e8 94 29 ea ff              	callq	0x140335b30 <.text+0x334b30>
14049319c: 0f 10 45 00                 	movups	(%rbp), %xmm0
1404931a0: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
1404931a4: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
1404931a8: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
1404931ac: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
1404931b0: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
1404931b4: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
1404931b8: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
1404931bc: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
1404931c0: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
1404931c4: 0f 29 5d 40                 	movaps	%xmm3, 0x40(%rbp)
1404931c8: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
1404931cc: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
1404931d0: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
1404931d4: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
1404931db: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
1404931e2: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
1404931e9: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
1404931f0: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
1404931f7: 0f 29 8d 90 00 00 00        	movaps	%xmm1, 0x90(%rbp)
1404931fe: 0f 29 95 a0 00 00 00        	movaps	%xmm2, 0xa0(%rbp)
140493205: 0f 29 9d b0 00 00 00        	movaps	%xmm3, 0xb0(%rbp)
14049320c: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140493213: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
14049321a: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
140493221: 0f 29 85 d0 00 00 00        	movaps	%xmm0, 0xd0(%rbp)
140493228: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
14049322f: 0f 29 85 e0 00 00 00        	movaps	%xmm0, 0xe0(%rbp)
140493236: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14049323d: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
140493241: e8 3a 1e c4 ff              	callq	0x1400d5080 <.text+0xd4080>
140493246: c6 45 a0 13                 	movb	$0x13, -0x60(%rbp)
14049324a: 48 8b 0d 1f 91 19 03        	movq	0x319911f(%rip), %rcx   # 0x14362c370
140493251: 4c 8d 05 40 7d 49 02        	leaq	0x2497d40(%rip), %r8    # 0x14292af98
140493258: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
14049325c: e8 bf 1a ed ff              	callq	0x140364d20 <.text+0x363d20>
140493261: 49 89 d0                    	movq	%rdx, %r8
140493264: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14049326b: 48 89 c2                    	movq	%rax, %rdx
14049326e: e8 4d 36 c4 ff              	callq	0x1400d68c0 <.text+0xd58c0>
140493273: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
14049327a: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
14049327e: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
140493285: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
140493289: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140493290: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
140493294: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
14049329b: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
1404932a2: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
1404932a9: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
1404932b0: 0f 29 5d 40                 	movaps	%xmm3, 0x40(%rbp)
1404932b4: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
1404932b8: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
1404932bc: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
1404932c0: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
1404932c4: 48 89 f1                    	movq	%rsi, %rcx
1404932c7: e8 74 38 e9 ff              	callq	0x140326b40 <.text+0x325b40>
1404932cc: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
1404932d4: 48 c7 45 18 08 00 00 00     	movq	$0x8, 0x18(%rbp)
1404932dc: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
1404932e4: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
1404932e8: 48 89 f1                    	movq	%rsi, %rcx
1404932eb: e8 f0 8c c8 ff              	callq	0x14011bfe0 <.text+0x11afe0>
1404932f0: 48 89 c6                    	movq	%rax, %rsi
1404932f3: 48 89 c1                    	movq	%rax, %rcx
1404932f6: e8 d5 3d e9 ff              	callq	0x1403270d0 <.text+0x3260d0>
1404932fb: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1404932ff: e8 3c c8 e7 ff              	callq	0x14030fb40 <.text+0x30eb40>
140493304: c6 45 a0 18                 	movb	$0x18, -0x60(%rbp)
140493308: 48 8b 0d 61 90 19 03        	movq	0x3199061(%rip), %rcx   # 0x14362c370
14049330f: 4c 8d 05 82 7c 49 02        	leaq	0x2497c82(%rip), %r8    # 0x14292af98
140493316: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
14049331a: e8 01 1a ed ff              	callq	0x140364d20 <.text+0x363d20>
14049331f: 49 89 d0                    	movq	%rdx, %r8
140493322: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140493326: 48 89 c2                    	movq	%rax, %rdx
140493329: e8 92 35 c4 ff              	callq	0x1400d68c0 <.text+0xd58c0>
14049332e: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
140493332: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
140493339: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
14049333d: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
140493344: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140493348: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
14049334f: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140493353: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
140493357: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
14049335b: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
14049335f: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
140493366: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
14049336d: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
140493374: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
14049337b: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
140493382: 48 89 f1                    	movq	%rsi, %rcx
140493385: e8 66 43 e9 ff              	callq	0x1403276f0 <.text+0x3266f0>
14049338a: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14049338e: e8 5d ab ea ff              	callq	0x14033def0 <.text+0x33cef0>
140493393: c6 45 a0 1c                 	movb	$0x1c, -0x60(%rbp)
140493397: 48 8b 0d d2 8f 19 03        	movq	0x3198fd2(%rip), %rcx   # 0x14362c370
14049339e: 4c 8d 05 f3 7b 49 02        	leaq	0x2497bf3(%rip), %r8    # 0x14292af98
1404933a5: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
1404933a9: e8 72 19 ed ff              	callq	0x140364d20 <.text+0x363d20>
1404933ae: 49 89 d0                    	movq	%rdx, %r8
1404933b1: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1404933b5: 48 89 c2                    	movq	%rax, %rdx
1404933b8: e8 03 35 c4 ff              	callq	0x1400d68c0 <.text+0xd58c0>
1404933bd: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
1404933c1: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
1404933c8: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
1404933cc: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
1404933d3: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
1404933d7: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
1404933de: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
1404933e2: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
1404933e6: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
1404933ea: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
1404933ee: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
1404933f5: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
1404933fc: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
140493403: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
14049340a: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
140493411: 48 89 f1                    	movq	%rsi, %rcx
140493414: e8 d7 42 e9 ff              	callq	0x1403276f0 <.text+0x3266f0>
140493419: 4c 8d 05 98 bc 54 02        	leaq	0x254bc98(%rip), %r8    # 0x1429df0b8
140493420: 48 89 f9                    	movq	%rdi, %rcx
140493423: b2 24                       	movb	$0x24, %dl
140493425: e8 76 d0 e7 ff              	callq	0x1403104a0 <.text+0x30f4a0>
14049342a: 48 89 f1                    	movq	%rsi, %rcx
14049342d: 48 89 fa                    	movq	%rdi, %rdx
140493430: e8 bb 42 e9 ff              	callq	0x1403276f0 <.text+0x3266f0>
140493435: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140493439: e8 02 89 c3 ff              	callq	0x1400cbd40 <.text+0xcad40>
14049343e: c6 45 a0 22                 	movb	$0x22, -0x60(%rbp)
140493442: 48 8b 0d 27 8f 19 03        	movq	0x3198f27(%rip), %rcx   # 0x14362c370
140493449: 4c 8d 05 48 7b 49 02        	leaq	0x2497b48(%rip), %r8    # 0x14292af98
140493450: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140493454: e8 c7 18 ed ff              	callq	0x140364d20 <.text+0x363d20>
140493459: 49 89 d0                    	movq	%rdx, %r8
14049345c: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140493460: 48 89 c2                    	movq	%rax, %rdx
140493463: e8 58 34 c4 ff              	callq	0x1400d68c0 <.text+0xd58c0>
140493468: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
14049346c: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
140493473: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
140493477: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
14049347e: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140493482: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
140493489: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14049348d: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
140493491: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
140493495: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
140493499: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
1404934a0: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
1404934a7: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
1404934ae: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1404934b5: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
1404934bc: 48 89 f1                    	movq	%rsi, %rcx
1404934bf: e8 2c 42 e9 ff              	callq	0x1403276f0 <.text+0x3266f0>
1404934c4: 4c 8d 05 05 bc 54 02        	leaq	0x254bc05(%rip), %r8    # 0x1429df0d0
1404934cb: 48 89 f9                    	movq	%rdi, %rcx
1404934ce: b2 2c                       	movb	$0x2c, %dl
1404934d0: e8 0b 1c c6 ff              	callq	0x1400f50e0 <.text+0xf40e0>
1404934d5: 48 89 f1                    	movq	%rsi, %rcx
1404934d8: 48 89 fa                    	movq	%rdi, %rdx
1404934db: e8 10 42 e9 ff              	callq	0x1403276f0 <.text+0x3266f0>
1404934e0: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1404934e4: e8 37 f7 dd ff              	callq	0x140272c20 <.text+0x271c20>
1404934e9: c6 45 a0 20                 	movb	$0x20, -0x60(%rbp)
1404934ed: 48 8b 0d 7c 8e 19 03        	movq	0x3198e7c(%rip), %rcx   # 0x14362c370
1404934f4: 4c 8d 05 9d 7a 49 02        	leaq	0x2497a9d(%rip), %r8    # 0x14292af98
1404934fb: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
1404934ff: e8 1c 18 ed ff              	callq	0x140364d20 <.text+0x363d20>
140493504: 49 89 d0                    	movq	%rdx, %r8
140493507: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14049350b: 48 89 c2                    	movq	%rax, %rdx
14049350e: e8 ad 33 c4 ff              	callq	0x1400d68c0 <.text+0xd58c0>
140493513: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
140493517: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
14049351e: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
140493522: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
140493529: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
14049352d: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
140493534: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140493538: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
14049353c: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
140493540: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
140493544: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
14049354b: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
140493552: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
140493559: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
140493560: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
140493567: 48 89 f1                    	movq	%rsi, %rcx
14049356a: e8 81 41 e9 ff              	callq	0x1403276f0 <.text+0x3266f0>
14049356f: 48 89 f1                    	movq	%rsi, %rcx
140493572: b2 01                       	movb	$0x1, %dl
140493574: e8 a7 46 e9 ff              	callq	0x140327c20 <.text+0x326c20>
140493579: 48 89 f1                    	movq	%rsi, %rcx
14049357c: 48 81 c4 f0 01 00 00        	addq	$0x1f0, %rsp            # imm = 0x1F0
140493583: 5f                          	popq	%rdi
140493584: 5e                          	popq	%rsi
140493585: 5d                          	popq	%rbp
140493586: e9 55 45 e9 ff              	jmp	0x140327ae0 <.text+0x326ae0>
14049358b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140493590: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140493595: 55                          	pushq	%rbp
140493596: 56                          	pushq	%rsi
140493597: 57                          	pushq	%rdi
140493598: 48 83 ec 20                 	subq	$0x20, %rsp
14049359c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1404935a3: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
1404935a7: e8 d4 62 b9 ff              	callq	0x140029880 <.text+0x28880>
1404935ac: 90                          	nop
1404935ad: 48 83 c4 20                 	addq	$0x20, %rsp
1404935b1: 5f                          	popq	%rdi
1404935b2: 5e                          	popq	%rsi
1404935b3: 5d                          	popq	%rbp
1404935b4: c3                          	retq
1404935b5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1404935c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1404935c5: 55                          	pushq	%rbp
1404935c6: 56                          	pushq	%rsi
1404935c7: 57                          	pushq	%rdi
1404935c8: 48 83 ec 20                 	subq	$0x20, %rsp
1404935cc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1404935d3: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1404935da: e8 a1 62 b9 ff              	callq	0x140029880 <.text+0x28880>
1404935df: 90                          	nop
1404935e0: 48 83 c4 20                 	addq	$0x20, %rsp
1404935e4: 5f                          	popq	%rdi
1404935e5: 5e                          	popq	%rsi
1404935e6: 5d                          	popq	%rbp
1404935e7: c3                          	retq
1404935e8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1404935f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1404935f5: 55                          	pushq	%rbp
1404935f6: 56                          	pushq	%rsi
1404935f7: 57                          	pushq	%rdi
1404935f8: 48 83 ec 20                 	subq	$0x20, %rsp
1404935fc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140493603: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140493607: e8 74 62 b9 ff              	callq	0x140029880 <.text+0x28880>
14049360c: 90                          	nop
14049360d: 48 83 c4 20                 	addq	$0x20, %rsp
140493611: 5f                          	popq	%rdi
140493612: 5e                          	popq	%rsi
140493613: 5d                          	popq	%rbp
140493614: c3                          	retq
140493615: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140493620: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140493625: 55                          	pushq	%rbp
140493626: 56                          	pushq	%rsi
140493627: 57                          	pushq	%rdi
140493628: 48 83 ec 20                 	subq	$0x20, %rsp
14049362c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140493633: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140493637: e8 44 62 b9 ff              	callq	0x140029880 <.text+0x28880>
14049363c: 90                          	nop
14049363d: 48 83 c4 20                 	addq	$0x20, %rsp
140493641: 5f                          	popq	%rdi
140493642: 5e                          	popq	%rsi
140493643: 5d                          	popq	%rbp
140493644: c3                          	retq
140493645: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140493650: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140493655: 55                          	pushq	%rbp
140493656: 56                          	pushq	%rsi
140493657: 57                          	pushq	%rdi
140493658: 48 83 ec 20                 	subq	$0x20, %rsp
14049365c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140493663: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140493667: e8 14 62 b9 ff              	callq	0x140029880 <.text+0x28880>
14049366c: 90                          	nop
14049366d: 48 83 c4 20                 	addq	$0x20, %rsp
140493671: 5f                          	popq	%rdi
140493672: 5e                          	popq	%rsi
140493673: 5d                          	popq	%rbp
140493674: c3                          	retq
140493675: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140493680: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140493685: 55                          	pushq	%rbp
140493686: 56                          	pushq	%rsi
140493687: 57                          	pushq	%rdi
140493688: 48 83 ec 20                 	subq	$0x20, %rsp
14049368c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140493693: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140493697: e8 e4 61 b9 ff              	callq	0x140029880 <.text+0x28880>
14049369c: 90                          	nop
14049369d: 48 83 c4 20                 	addq	$0x20, %rsp
1404936a1: 5f                          	popq	%rdi
1404936a2: 5e                          	popq	%rsi
1404936a3: 5d                          	popq	%rbp
1404936a4: c3                          	retq

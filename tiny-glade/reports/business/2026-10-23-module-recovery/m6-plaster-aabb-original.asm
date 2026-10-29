
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412400c0: f3 0f 10 01                 	movss	(%rcx), %xmm0
1412400c4: f3 0f 58 41 08              	addss	0x8(%rcx), %xmm0
1412400c9: f3 0f 10 49 04              	movss	0x4(%rcx), %xmm1
1412400ce: f3 0f 58 49 0c              	addss	0xc(%rcx), %xmm1
1412400d3: f3 0f 10 15 95 57 6e 01     	movss	0x16e5795(%rip), %xmm2  # 0x142925870
1412400db: f3 0f 59 c2                 	mulss	%xmm2, %xmm0
1412400df: f3 0f 59 ca                 	mulss	%xmm2, %xmm1
1412400e3: c3                          	retq
1412400e4: cc                          	int3
1412400e5: cc                          	int3
1412400e6: cc                          	int3
1412400e7: cc                          	int3
1412400e8: cc                          	int3
1412400e9: cc                          	int3
1412400ea: cc                          	int3
1412400eb: cc                          	int3
1412400ec: cc                          	int3
1412400ed: cc                          	int3
1412400ee: cc                          	int3
1412400ef: cc                          	int3
1412400f0: f3 0f 10 41 08              	movss	0x8(%rcx), %xmm0
1412400f5: f3 0f 5c 01                 	subss	(%rcx), %xmm0
1412400f9: f3 0f 10 49 0c              	movss	0xc(%rcx), %xmm1
1412400fe: f3 0f 5c 49 04              	subss	0x4(%rcx), %xmm1
141240103: f3 0f 10 15 65 57 6e 01     	movss	0x16e5765(%rip), %xmm2  # 0x142925870
14124010b: f3 0f 59 c2                 	mulss	%xmm2, %xmm0
14124010f: f3 0f 59 ca                 	mulss	%xmm2, %xmm1
141240113: c3                          	retq

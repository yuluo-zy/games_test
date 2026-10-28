Pill hotkey: writes +44

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1406816c0: 41 57                       	pushq	%r15
1406816c2: 41 56                       	pushq	%r14
1406816c4: 41 55                       	pushq	%r13
1406816c6: 41 54                       	pushq	%r12
1406816c8: 56                          	pushq	%rsi
1406816c9: 57                          	pushq	%rdi
1406816ca: 53                          	pushq	%rbx
1406816cb: 48 83 ec 70                 	subq	$0x70, %rsp
1406816cf: 44 89 c3                    	movl	%r8d, %ebx
1406816d2: 48 89 d7                    	movq	%rdx, %rdi
1406816d5: 48 89 ce                    	movq	%rcx, %rsi
1406816d8: 48 89 d1                    	movq	%rdx, %rcx
1406816db: b2 01                       	movb	$0x1, %dl
1406816dd: e8 9e c7 91 00              	callq	0x140f9de80 <.text+0xf9ce80>
1406816e2: 88 58 44                    	movb	%bl, 0x44(%rax)
1406816e5: 80 fb 12                    	cmpb	$0x12, %bl
1406816e8: 0f 84 9a 01 00 00           	je	0x140681888 <.text+0x680888>

Pill pre_update: reads +44

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140681930: 56                          	pushq	%rsi
140681931: 48 83 ec 20                 	subq	$0x20, %rsp
140681935: 48 89 d6                    	movq	%rdx, %rsi
140681938: 0f b6 51 44                 	movzbl	0x44(%rcx), %edx
14068193c: 80 fa 28                    	cmpb	$0x28, %dl
14068193f: 74 13                       	je	0x140681954 <.text+0x680954>
140681941: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
140681945: e8 d6 4c 2c 00              	callq	0x140946620 <.text+0x945620>
14068194a: 84 c0                       	testb	%al, %al
14068194c: 74 06                       	je	0x140681954 <.text+0x680954>
14068194e: 48 8b 06                    	movq	(%rsi), %rax
140681951: 80 08 20                    	orb	$0x20, (%rax)
140681954: 48 83 c4 20                 	addq	$0x20, %rsp
140681958: 5e                          	popq	%rsi
140681959: c3                          	retq

Confirmed false default leaf

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140002370: 31 c0                       	xorl	%eax, %eax
140002372: c3                          	retq

Confirmed true/no-op default leaves

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14003bb70: b0 01                       	movb	$0x1, %al
14003bb72: c3                          	retq
14003bb73: cc                          	int3
14003bb74: cc                          	int3
14003bb75: cc                          	int3
14003bb76: cc                          	int3
14003bb77: cc                          	int3
14003bb78: cc                          	int3
14003bb79: cc                          	int3
14003bb7a: cc                          	int3
14003bb7b: cc                          	int3
14003bb7c: cc                          	int3
14003bb7d: cc                          	int3
14003bb7e: cc                          	int3
14003bb7f: cc                          	int3
14003bb80: c3                          	retq
14003bb81: cc                          	int3
14003bb82: cc                          	int3
14003bb83: cc                          	int3

MeiUiSounds default

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14068e840: 31 c0                       	xorl	%eax, %eax
14068e842: b2 02                       	movb	$0x2, %dl
14068e844: c3                          	retq

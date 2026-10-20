
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404ffeb0:     	pushq	%r15
1404ffeb2:     	pushq	%r14
1404ffeb4:     	pushq	%r13
1404ffeb6:     	pushq	%r12
1404ffeb8:     	pushq	%rsi
1404ffeb9:     	pushq	%rdi
1404ffeba:     	pushq	%rbx
1404ffebb:     	subq	$0xf0, %rsp
1404ffec2:     	movq	%rcx, %rdi
1404ffec5:     	movq	%rdx, 0x60(%rsp)
1404ffeca:     	movq	%r8, 0x68(%rsp)
1404ffecf:     	leaq	0x10(%rcx), %rsi
1404ffed3:     	movq	%rsi, %rcx
1404ffed6:     	callq	0x1406868b0 <.text+0x6858b0>
1404ffedb:     	movq	%rax, %r15
1404ffede:     	leaq	0x60(%rsp), %r14
1404ffee3:     	movq	%r14, %rcx
1404ffee6:     	callq	0x140683c10 <.text+0x682c10>
1404ffeeb:     	movq	%rax, 0x40(%rsp)
1404ffef0:     	leaq	0x40(%rsp), %rbx
1404ffef5:     	movq	%rbx, %rcx
1404ffef8:     	callq	0x1400885d0 <.text+0x875d0>
1404ffefd:     	movq	%rax, 0x70(%rsp)
1404fff02:     	movabsq	$0x41223169ff28813b, %r12 # imm = 0x41223169FF28813B
1404fff0c:     	movq	%r12, 0x78(%rsp)
1404fff11:     	movabsq	$-0x58648d975d569727, %r13 # imm = 0xA79B7268A2A968D9
1404fff1b:     	movq	%r13, 0x80(%rsp)
1404fff23:     	leaq	0x70(%rsp), %rcx
1404fff28:     	callq	0x1405bdf20 <.text+0x5bcf20>
1404fff2d:     	movq	%r15, 0x70(%rsp)
1404fff32:     	movq	%rax, 0x78(%rsp)
1404fff37:     	leaq	0x70(%rsp), %rcx
1404fff3c:     	callq	0x1405bd910 <.text+0x5bc910>
1404fff41:     	movq	%rax, %r15
1404fff44:     	movq	%rsi, %rcx
1404fff47:     	movq	%rax, %rdx
1404fff4a:     	callq	0x140686450 <.text+0x685450>
1404fff4f:     	testb	%al, %al
1404fff51:     	je	0x1404fff6b <.text+0x4fef6b>
1404fff53:     	leaq	0x40(%rsp), %rdi
1404fff58:     	movq	%rdi, %rcx
1404fff5b:     	movq	%rsi, %rdx
1404fff5e:     	movq	%r15, %r8
1404fff61:     	callq	0x140686530 <.text+0x685530>
1404fff66:     	jmp	0x1405000b7 <.text+0x4ff0b7>
1404fff6b:     	movq	0x312c436(%rip), %rax   # 0x14362c3a8
1404fff72:     	movq	(%rax), %rax
1404fff75:     	testq	%rax, %rax
1404fff78:     	je	0x14050005c <.text+0x4ff05c>
1404fff7e:     	movq	%rdi, 0x40(%rsp)
1404fff83:     	leaq	0x33916(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
1404fff8a:     	movq	%rax, 0x48(%rsp)
1404fff8f:     	movq	%r14, 0x50(%rsp)
1404fff94:     	movq	%rax, 0x58(%rsp)
1404fff99:     	leaq	0x24eddb8(%rip), %rcx   # 0x1429edd58
1404fffa0:     	callq	0x14067dd80 <.text+0x67cd80>
1404fffa5:     	movups	(%rax), %xmm0
1404fffa8:     	movl	0x10(%rax), %eax
1404fffab:     	movq	$0x1, 0xa0(%rsp)
1404fffb7:     	leaq	0x24edd6a(%rip), %rcx   # 0x1429edd28
1404fffbe:     	movq	%rcx, 0xa8(%rsp)
1404fffc6:     	movq	$0x14, 0xb0(%rsp)
1404fffd2:     	leaq	0x24edcbf(%rip), %rdx   # 0x1429edc98
1404fffd9:     	movq	%rdx, 0xc0(%rsp)
1404fffe1:     	movq	$0x2, 0xc8(%rsp)
1404fffed:     	movq	%rbx, 0xd0(%rsp)
1404ffff5:     	movq	$0x2, 0xd8(%rsp)
140500001:     	movq	$0x0, 0xe0(%rsp)
14050000d:     	movq	$0x0, 0x70(%rsp)
140500016:     	movq	%rcx, 0x78(%rsp)
14050001b:     	movq	$0x14, 0x80(%rsp)
140500027:     	movq	$0x0, 0x88(%rsp)
140500033:     	movups	%xmm0, 0x90(%rsp)
14050003b:     	movl	$0x1, 0xb8(%rsp)
140500046:     	movl	%eax, 0xbc(%rsp)
14050004d:     	leaq	0x3f(%rsp), %rcx
140500052:     	leaq	0x70(%rsp), %rdx
140500057:     	callq	0x140d4a330 <.text+0xd49330>
14050005c:     	movups	0x60(%rsp), %xmm0
140500061:     	movq	0x18(%rdi), %rdi
140500065:     	movups	%xmm0, 0x40(%rsp)
14050006a:     	leaq	0x40(%rsp), %rcx
14050006f:     	callq	0x140683c10 <.text+0x682c10>
140500074:     	movq	%rax, 0x70(%rsp)
140500079:     	movq	%r12, 0x78(%rsp)
14050007e:     	movq	%r13, 0x80(%rsp)
140500086:     	leaq	0x70(%rsp), %rcx
14050008b:     	callq	0x1405bdf20 <.text+0x5bcf20>
140500090:     	movq	%rdi, 0x70(%rsp)
140500095:     	movq	%rax, 0x78(%rsp)
14050009a:     	leaq	0x70(%rsp), %rcx
14050009f:     	callq	0x1405bd910 <.text+0x5bc910>
1405000a4:     	leaq	0x40(%rsp), %rdi
1405000a9:     	movq	%rdi, %rcx
1405000ac:     	movq	%rsi, %rdx
1405000af:     	movq	%rax, %r8
1405000b2:     	callq	0x1406861b0 <.text+0x6851b0>
1405000b7:     	leaq	0x24eddf5(%rip), %rax   # 0x1429edeb3
1405000be:     	movq	%rax, 0x20(%rsp)
1405000c3:     	movl	$0x43960000, 0x30(%rsp) # imm = 0x43960000
1405000cb:     	movq	$0x19, 0x28(%rsp)
1405000d4:     	leaq	0x24eddf1(%rip), %r8    # 0x1429edecc
1405000db:     	leaq	0x70(%rsp), %rcx
1405000e0:     	movl	$0x8, %r9d
1405000e6:     	movq	%rdi, %rdx
1405000e9:     	callq	0x14032e0b0 <.text+0x32d0b0>
1405000ee:     	nop
1405000ef:     	addq	$0xf0, %rsp
1405000f6:     	popq	%rbx
1405000f7:     	popq	%rdi
1405000f8:     	popq	%rsi
1405000f9:     	popq	%r12
1405000fb:     	popq	%r13
1405000fd:     	popq	%r14
1405000ff:     	popq	%r15
140500101:     	retq

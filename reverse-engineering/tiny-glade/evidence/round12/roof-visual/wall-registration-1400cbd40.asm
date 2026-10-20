
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400cbd40:     	pushq	%rbp
1400cbd41:     	pushq	%rsi
1400cbd42:     	pushq	%rdi
1400cbd43:     	pushq	%rbx
1400cbd44:     	subq	$0x538, %rsp            # imm = 0x538
1400cbd4b:     	leaq	0x80(%rsp), %rbp
1400cbd53:     	movq	$-0x2, 0x4b0(%rbp)
1400cbd5e:     	movq	%rcx, %rsi
1400cbd61:     	movzbl	0x3568ea1(%rip), %eax   # 0x143634c09
1400cbd68:     	movl	$0x1c0, %ecx            # imm = 0x1C0
1400cbd6d:     	movl	$0x8, %edx
1400cbd72:     	callq	0x140613c10 <.text+0x612c10>
1400cbd77:     	testq	%rax, %rax
1400cbd7a:     	je	0x1400cc0cc <.text+0xcb0cc>
1400cbd80:     	movq	%rax, 0x4a8(%rbp)
1400cbd87:     	xorps	%xmm0, %xmm0
1400cbd8a:     	movups	%xmm0, -0x38(%rbp)
1400cbd8e:     	movups	%xmm0, -0x20(%rbp)
1400cbd92:     	movups	%xmm0, -0x8(%rbp)
1400cbd96:     	movups	%xmm0, 0x10(%rbp)
1400cbd9a:     	movups	%xmm0, 0x28(%rbp)
1400cbd9e:     	movl	$0x0, 0x38(%rbp)
1400cbda5:     	movups	%xmm0, 0x60(%rbp)
1400cbda9:     	movups	%xmm0, 0x78(%rbp)
1400cbdad:     	movups	%xmm0, 0x90(%rbp)
1400cbdb4:     	movups	%xmm0, 0xa8(%rbp)
1400cbdbb:     	movups	%xmm0, 0xc0(%rbp)
1400cbdc2:     	movl	$0x0, 0xd0(%rbp)
1400cbdcc:     	movq	$0x0, -0x58(%rbp)
1400cbdd4:     	movq	$0x8, -0x50(%rbp)
1400cbddc:     	movq	$0x0, -0x48(%rbp)
1400cbde4:     	movq	$0x10, -0x40(%rbp)
1400cbdec:     	movq	$0x10, -0x28(%rbp)
1400cbdf4:     	movq	$0x10, -0x10(%rbp)
1400cbdfc:     	movq	$0x10, 0x8(%rbp)
1400cbe04:     	movq	$0x10, 0x20(%rbp)
1400cbe0c:     	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
1400cbe16:     	movq	%rbx, 0x40(%rbp)
1400cbe1a:     	leaq	0x286b83c(%rip), %rax   # 0x14293765d
1400cbe21:     	movq	%rax, 0x48(%rbp)
1400cbe25:     	movq	$0x6e, 0x50(%rbp)
1400cbe2d:     	movq	$0x10, 0x58(%rbp)
1400cbe35:     	movq	$0x10, 0x70(%rbp)
1400cbe3d:     	movq	$0x10, 0x88(%rbp)
1400cbe48:     	movq	$0x10, 0xa0(%rbp)
1400cbe53:     	movq	$0x10, 0xb8(%rbp)
1400cbe5e:     	movl	$0x0, 0xd8(%rbp)
1400cbe68:     	movw	$0x1, 0xdc(%rbp)
1400cbe71:     	movq	%rbx, 0x1d0(%rbp)
1400cbe78:     	movl	$0x0, 0x350(%rbp)
1400cbe82:     	movzbl	0x3568d80(%rip), %eax   # 0x143634c09
1400cbe89:     	movl	$0x3b0, %ecx            # imm = 0x3B0
1400cbe8e:     	movl	$0x8, %edx
1400cbe93:     	callq	0x140613c10 <.text+0x612c10>
1400cbe98:     	testq	%rax, %rax
1400cbe9b:     	je	0x1400cc0db <.text+0xcb0db>
1400cbea1:     	movq	%rax, %rdi
1400cbea4:     	leaq	-0x58(%rbp), %rdx
1400cbea8:     	movl	$0x3b0, %r8d            # imm = 0x3B0
1400cbeae:     	movq	%rax, %rcx
1400cbeb1:     	callq	0x142923c00 <.text+0x2922c00>
1400cbeb6:     	leaq	0x286b613(%rip), %r8    # 0x1429374d0
1400cbebd:     	leaq	0x438(%rbp), %rcx
1400cbec4:     	movq	%rdi, %rdx
1400cbec7:     	callq	0x140ce4b10 <.text+0xce3b10>
1400cbecc:     	leaq	0x3c8(%rbp), %rcx
1400cbed3:     	callq	0x140048c50 <.text+0x47c50>
1400cbed8:     	leaq	0x358(%rbp), %rcx
1400cbedf:     	callq	0x140048fe0 <.text+0x47fe0>
1400cbee4:     	leaq	-0x58(%rbp), %rcx
1400cbee8:     	callq	0x1402d4610 <.text+0x2d3610>
1400cbeed:     	movups	0x498(%rbp), %xmm0
1400cbef4:     	movq	0x4a8(%rbp), %rax
1400cbefb:     	movups	%xmm0, 0x60(%rax)
1400cbeff:     	movups	0x488(%rbp), %xmm0
1400cbf06:     	movups	%xmm0, 0x50(%rax)
1400cbf0a:     	movups	0x478(%rbp), %xmm0
1400cbf11:     	movups	%xmm0, 0x40(%rax)
1400cbf15:     	movups	0x438(%rbp), %xmm0
1400cbf1c:     	movups	0x448(%rbp), %xmm1
1400cbf23:     	movups	0x458(%rbp), %xmm2
1400cbf2a:     	movups	0x468(%rbp), %xmm3
1400cbf31:     	movups	%xmm3, 0x30(%rax)
1400cbf35:     	movups	%xmm2, 0x20(%rax)
1400cbf39:     	movups	%xmm1, 0x10(%rax)
1400cbf3d:     	movups	%xmm0, (%rax)
1400cbf40:     	movups	0x3c8(%rbp), %xmm0
1400cbf47:     	movups	0x3d8(%rbp), %xmm1
1400cbf4e:     	movups	0x3e8(%rbp), %xmm2
1400cbf55:     	movups	0x3f8(%rbp), %xmm3
1400cbf5c:     	movups	%xmm0, 0x70(%rax)
1400cbf60:     	movups	%xmm1, 0x80(%rax)
1400cbf67:     	movups	%xmm2, 0x90(%rax)
1400cbf6e:     	movups	%xmm3, 0xa0(%rax)
1400cbf75:     	movups	0x408(%rbp), %xmm0
1400cbf7c:     	movups	%xmm0, 0xb0(%rax)
1400cbf83:     	movups	0x418(%rbp), %xmm0
1400cbf8a:     	movups	%xmm0, 0xc0(%rax)
1400cbf91:     	movups	0x428(%rbp), %xmm0
1400cbf98:     	movups	%xmm0, 0xd0(%rax)
1400cbf9f:     	movups	0x3b8(%rbp), %xmm0
1400cbfa6:     	movups	%xmm0, 0x140(%rax)
1400cbfad:     	movups	0x3a8(%rbp), %xmm0
1400cbfb4:     	movups	%xmm0, 0x130(%rax)
1400cbfbb:     	movups	0x398(%rbp), %xmm0
1400cbfc2:     	movups	%xmm0, 0x120(%rax)
1400cbfc9:     	movups	0x358(%rbp), %xmm0
1400cbfd0:     	movups	0x368(%rbp), %xmm1
1400cbfd7:     	movups	0x378(%rbp), %xmm2
1400cbfde:     	movups	0x388(%rbp), %xmm3
1400cbfe5:     	movups	%xmm3, 0x110(%rax)
1400cbfec:     	movups	%xmm2, 0x100(%rax)
1400cbff3:     	movups	%xmm1, 0xf0(%rax)
1400cbffa:     	movups	%xmm0, 0xe0(%rax)
1400cc001:     	movups	-0x58(%rbp), %xmm0
1400cc005:     	movups	-0x48(%rbp), %xmm1
1400cc009:     	movups	-0x38(%rbp), %xmm2
1400cc00d:     	movups	-0x28(%rbp), %xmm3
1400cc011:     	movups	%xmm0, 0x150(%rax)
1400cc018:     	movups	%xmm1, 0x160(%rax)
1400cc01f:     	movups	%xmm2, 0x170(%rax)
1400cc026:     	movups	%xmm3, 0x180(%rax)
1400cc02d:     	movups	-0x18(%rbp), %xmm0
1400cc031:     	movups	%xmm0, 0x190(%rax)
1400cc038:     	movups	-0x8(%rbp), %xmm0
1400cc03c:     	movups	%xmm0, 0x1a0(%rax)
1400cc043:     	movups	0x8(%rbp), %xmm0
1400cc047:     	movups	%xmm0, 0x1b0(%rax)
1400cc04e:     	movq	$0x4, -0x50(%rbp)
1400cc056:     	movq	%rax, -0x48(%rbp)
1400cc05a:     	movq	$0x4, -0x40(%rbp)
1400cc062:     	movq	$0x0, -0x38(%rbp)
1400cc06a:     	movq	$0x8, -0x30(%rbp)
1400cc072:     	xorps	%xmm0, %xmm0
1400cc075:     	movups	%xmm0, -0x28(%rbp)
1400cc079:     	movq	%rbx, -0x58(%rbp)
1400cc07d:     	leaq	-0x20(%rbp), %rcx
1400cc081:     	callq	0x1401705d0 <.text+0x16f5d0>
1400cc086:     	movups	0x8(%rbp), %xmm0
1400cc08a:     	movups	%xmm0, 0x60(%rsi)
1400cc08e:     	movups	-0x8(%rbp), %xmm0
1400cc092:     	movups	%xmm0, 0x50(%rsi)
1400cc096:     	movups	-0x18(%rbp), %xmm0
1400cc09a:     	movups	%xmm0, 0x40(%rsi)
1400cc09e:     	movups	-0x58(%rbp), %xmm0
1400cc0a2:     	movups	-0x48(%rbp), %xmm1
1400cc0a6:     	movups	-0x38(%rbp), %xmm2
1400cc0aa:     	movups	-0x28(%rbp), %xmm3
1400cc0ae:     	movups	%xmm3, 0x30(%rsi)
1400cc0b2:     	movups	%xmm2, 0x20(%rsi)
1400cc0b6:     	movups	%xmm1, 0x10(%rsi)
1400cc0ba:     	movups	%xmm0, (%rsi)
1400cc0bd:     	movq	%rsi, %rax
1400cc0c0:     	addq	$0x538, %rsp            # imm = 0x538
1400cc0c7:     	popq	%rbx
1400cc0c8:     	popq	%rdi
1400cc0c9:     	popq	%rsi
1400cc0ca:     	popq	%rbp
1400cc0cb:     	retq
1400cc0cc:     	movl	$0x8, %ecx
1400cc0d1:     	movl	$0x1c0, %edx            # imm = 0x1C0
1400cc0d6:     	callq	0x1428d9000 <.text+0x28d8000>
1400cc0db:     	movl	$0x8, %ecx
1400cc0e0:     	movl	$0x3b0, %edx            # imm = 0x3B0
1400cc0e5:     	callq	0x1428d9000 <.text+0x28d8000>
1400cc0ea:     	ud2
1400cc0ec:     	nopl	(%rax)
1400cc0f0:     	movq	%rdx, 0x10(%rsp)
1400cc0f5:     	pushq	%rbp
1400cc0f6:     	pushq	%rsi
1400cc0f7:     	pushq	%rdi
1400cc0f8:     	pushq	%rbx
1400cc0f9:     	subq	$0x28, %rsp
1400cc0fd:     	leaq	0x80(%rdx), %rbp
1400cc104:     	leaq	-0x58(%rbp), %rcx
1400cc108:     	callq	0x1405d16b0 <.text+0x5d06b0>
1400cc10d:     	nop
1400cc10e:     	addq	$0x28, %rsp
1400cc112:     	popq	%rbx
1400cc113:     	popq	%rdi
1400cc114:     	popq	%rsi
1400cc115:     	popq	%rbp
1400cc116:     	retq
1400cc117:     	nopw	(%rax,%rax)
1400cc120:     	movq	%rdx, 0x10(%rsp)
1400cc125:     	pushq	%rbp
1400cc126:     	pushq	%rsi
1400cc127:     	pushq	%rdi
1400cc128:     	pushq	%rbx
1400cc129:     	subq	$0x28, %rsp
1400cc12d:     	leaq	0x80(%rdx), %rbp
1400cc134:     	movl	$0x1c0, %edx            # imm = 0x1C0
1400cc139:     	movl	$0x8, %r8d
1400cc13f:     	movq	0x4a8(%rbp), %rcx
1400cc146:     	callq	0x140613c20 <.text+0x612c20>
1400cc14b:     	nop
1400cc14c:     	addq	$0x28, %rsp
1400cc150:     	popq	%rbx
1400cc151:     	popq	%rdi
1400cc152:     	popq	%rsi
1400cc153:     	popq	%rbp
1400cc154:     	retq
1400cc155:     	nopw	%cs:(%rax,%rax)
1400cc160:     	movq	%rdx, 0x10(%rsp)
1400cc165:     	pushq	%rbp
1400cc166:     	pushq	%rsi
1400cc167:     	pushq	%rdi
1400cc168:     	pushq	%rbx
1400cc169:     	subq	$0x28, %rsp
1400cc16d:     	leaq	0x80(%rdx), %rbp
1400cc174:     	leaq	0x438(%rbp), %rcx
1400cc17b:     	callq	0x140029880 <.text+0x28880>
1400cc180:     	nop
1400cc181:     	addq	$0x28, %rsp
1400cc185:     	popq	%rbx
1400cc186:     	popq	%rdi
1400cc187:     	popq	%rsi
1400cc188:     	popq	%rbp
1400cc189:     	retq
1400cc18a:     	nopw	(%rax,%rax)
1400cc190:     	movq	%rdx, 0x10(%rsp)
1400cc195:     	pushq	%rbp
1400cc196:     	pushq	%rsi
1400cc197:     	pushq	%rdi
1400cc198:     	pushq	%rbx
1400cc199:     	subq	$0x28, %rsp
1400cc19d:     	leaq	0x80(%rdx), %rbp
1400cc1a4:     	leaq	0x3c8(%rbp), %rcx
1400cc1ab:     	callq	0x140029880 <.text+0x28880>
1400cc1b0:     	nop
1400cc1b1:     	addq	$0x28, %rsp
1400cc1b5:     	popq	%rbx
1400cc1b6:     	popq	%rdi
1400cc1b7:     	popq	%rsi
1400cc1b8:     	popq	%rbp
1400cc1b9:     	retq
1400cc1ba:     	nopw	(%rax,%rax)
1400cc1c0:     	movq	%rdx, 0x10(%rsp)
1400cc1c5:     	pushq	%rbp
1400cc1c6:     	pushq	%rsi
1400cc1c7:     	pushq	%rdi
1400cc1c8:     	pushq	%rbx
1400cc1c9:     	subq	$0x28, %rsp
1400cc1cd:     	leaq	0x80(%rdx), %rbp
1400cc1d4:     	leaq	0x358(%rbp), %rcx
1400cc1db:     	callq	0x140029880 <.text+0x28880>
1400cc1e0:     	nop
1400cc1e1:     	addq	$0x28, %rsp
1400cc1e5:     	popq	%rbx
1400cc1e6:     	popq	%rdi
1400cc1e7:     	popq	%rsi
1400cc1e8:     	popq	%rbp
1400cc1e9:     	retq
1400cc1ea:     	nopw	(%rax,%rax)
1400cc1f0:     	movq	%rdx, 0x10(%rsp)
1400cc1f5:     	pushq	%rbp
1400cc1f6:     	pushq	%rsi
1400cc1f7:     	pushq	%rdi
1400cc1f8:     	pushq	%rbx
1400cc1f9:     	subq	$0x28, %rsp
1400cc1fd:     	leaq	0x80(%rdx), %rbp
1400cc204:     	leaq	-0x58(%rbp), %rcx
1400cc208:     	callq	0x140029880 <.text+0x28880>
1400cc20d:     	nop
1400cc20e:     	addq	$0x28, %rsp
1400cc212:     	popq	%rbx
1400cc213:     	popq	%rdi
1400cc214:     	popq	%rsi
1400cc215:     	popq	%rbp
1400cc216:     	retq

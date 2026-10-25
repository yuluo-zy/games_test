
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140295030:     	pushq	%rbp
140295031:     	pushq	%rsi
140295032:     	pushq	%rdi
140295033:     	subq	$0x1d0, %rsp            # imm = 0x1D0
14029503a:     	leaq	0x80(%rsp), %rbp
140295042:     	movq	$-0x2, 0x148(%rbp)
14029504d:     	movq	%rcx, %rsi
140295050:     	xorps	%xmm0, %xmm0
140295053:     	movups	%xmm0, -0x40(%rbp)
140295057:     	movups	%xmm0, -0x28(%rbp)
14029505b:     	movups	%xmm0, -0x10(%rbp)
14029505f:     	movups	%xmm0, 0x8(%rbp)
140295063:     	movups	%xmm0, 0x20(%rbp)
140295067:     	movl	$0x0, 0x30(%rbp)
14029506e:     	movups	%xmm0, 0x58(%rbp)
140295072:     	movups	%xmm0, 0x70(%rbp)
140295076:     	movups	%xmm0, 0x88(%rbp)
14029507d:     	movups	%xmm0, 0xa0(%rbp)
140295084:     	movups	%xmm0, 0xb8(%rbp)
14029508b:     	movl	$0x0, 0xc8(%rbp)
140295095:     	movq	$0x0, -0x60(%rbp)
14029509d:     	movq	$0x8, -0x58(%rbp)
1402950a5:     	movq	$0x0, -0x50(%rbp)
1402950ad:     	movq	$0x10, -0x48(%rbp)
1402950b5:     	movq	$0x10, -0x30(%rbp)
1402950bd:     	movq	$0x10, -0x18(%rbp)
1402950c5:     	movq	$0x10, (%rbp)
1402950cd:     	movq	$0x10, 0x18(%rbp)
1402950d5:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1402950df:     	movq	%rax, 0x38(%rbp)
1402950e3:     	leaq	0x2716a1c(%rip), %rcx   # 0x1429abb06
1402950ea:     	movq	%rcx, 0x40(%rbp)
1402950ee:     	movq	$0x35, 0x48(%rbp)
1402950f6:     	movq	$0x10, 0x50(%rbp)
1402950fe:     	movq	$0x10, 0x68(%rbp)
140295106:     	movq	$0x10, 0x80(%rbp)
140295111:     	movq	$0x10, 0x98(%rbp)
14029511c:     	movq	$0x10, 0xb0(%rbp)
140295127:     	movl	$0x0, 0xd0(%rbp)
140295131:     	movw	$0x1, 0xd4(%rbp)
14029513a:     	movq	%rax, 0x100(%rbp)
140295141:     	movl	$0x0, 0x140(%rbp)
14029514b:     	movzbl	0x339fab7(%rip), %eax   # 0x143634c09
140295152:     	movl	$0x1a8, %ecx            # imm = 0x1A8
140295157:     	movl	$0x8, %edx
14029515c:     	callq	0x140613c10 <.text+0x612c10>
140295161:     	testq	%rax, %rax
140295164:     	je	0x14029519b <.text+0x29419b>
140295166:     	movq	%rax, %rdi
140295169:     	leaq	-0x60(%rbp), %rdx
14029516d:     	movl	$0x1a8, %r8d            # imm = 0x1A8
140295173:     	movq	%rax, %rcx
140295176:     	callq	0x142923c00 <.text+0x2922c00>
14029517b:     	leaq	0x271648e(%rip), %r8    # 0x1429ab610
140295182:     	movq	%rsi, %rcx
140295185:     	movq	%rdi, %rdx
140295188:     	callq	0x140ce4b10 <.text+0xce3b10>
14029518d:     	movq	%rsi, %rax
140295190:     	addq	$0x1d0, %rsp            # imm = 0x1D0
140295197:     	popq	%rdi
140295198:     	popq	%rsi
140295199:     	popq	%rbp
14029519a:     	retq
14029519b:     	movl	$0x8, %ecx
1402951a0:     	movl	$0x1a8, %edx            # imm = 0x1A8
1402951a5:     	callq	0x1428d9000 <.text+0x28d8000>
1402951aa:     	ud2


D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140940d80:     	pushq	%rsi
140940d81:     	pushq	%rdi
140940d82:     	subq	$0xd8, %rsp
140940d89:     	movaps	%xmm6, 0xc0(%rsp)
140940d91:     	callq	0x14153a940 <.text+0x1539940>
140940d96:     	movq	%rax, %rsi
140940d99:     	leaq	0x20(%rsp), %rdi
140940d9e:     	movq	%rdi, %rcx
140940da1:     	callq	0x140c97500 <.text+0xc96500>
140940da6:     	movq	%rsi, %rcx
140940da9:     	movq	%rdi, %rdx
140940dac:     	callq	0x1415375b0 <.text+0x15365b0>
140940db1:     	movl	$0x0, 0x20(%rsp)
140940db9:     	xorps	%xmm6, %xmm6
140940dbc:     	movups	%xmm6, 0x30(%rsp)
140940dc1:     	leaq	0x20(%rsp), %rdx
140940dc6:     	movq	%rax, %rcx
140940dc9:     	callq	0x141544c80 <.text+0x1543c80>
140940dce:     	xorps	%xmm1, %xmm1
140940dd1:     	xorps	%xmm2, %xmm2
140940dd4:     	movq	%rax, %rcx
140940dd7:     	callq	0x14153cc00 <.text+0x153bc00>
140940ddc:     	movl	$0x0, 0x28(%rsp)
140940de4:     	movq	$0x0, 0x20(%rsp)
140940ded:     	movsd	0x208c53b(%rip), %xmm0  # 0x1429cd330
140940df5:     	movsd	%xmm0, 0x2c(%rsp)
140940dfb:     	movl	$0xbf800000, 0x34(%rsp) # imm = 0xBF800000
140940e03:     	leaq	0x20(%rsp), %rdx
140940e08:     	movq	%rax, %rcx
140940e0b:     	callq	0x1415355b0 <.text+0x15345b0>
140940e10:     	movaps	%xmm6, 0x40(%rsp)
140940e15:     	movaps	%xmm6, 0x30(%rsp)
140940e1a:     	movaps	%xmm6, 0x20(%rsp)
140940e1f:     	movq	$0x0, 0x50(%rsp)
140940e28:     	leaq	0x20(%rsp), %rdx
140940e2d:     	movq	%rax, %rcx
140940e30:     	callq	0x141539380 <.text+0x1538380>
140940e35:     	movl	$0x0, 0x20(%rsp)
140940e3d:     	leaq	0x20(%rsp), %rdx
140940e42:     	movq	%rax, %rcx
140940e45:     	callq	0x14153d770 <.text+0x153c770>
140940e4a:     	movl	$0x0, 0x20(%rsp)
140940e52:     	leaq	0x20(%rsp), %rdx
140940e57:     	movq	%rax, %rcx
140940e5a:     	callq	0x141536030 <.text+0x1535030>
140940e5f:     	movl	$0x0, 0x30(%rsp)
140940e67:     	movl	$0x0, 0x38(%rsp)
140940e6f:     	movl	$0x0, 0x40(%rsp)
140940e77:     	movl	$0x0, 0x48(%rsp)
140940e7f:     	movq	$0x0, 0x20(%rsp)
140940e88:     	leaq	0x20(%rsp), %rdx
140940e8d:     	movq	%rax, %rcx
140940e90:     	callq	0x14153a3f0 <.text+0x15393f0>
140940e95:     	movq	$0x0, 0x48(%rsp)
140940e9e:     	movups	%xmm6, 0x30(%rsp)
140940ea3:     	movq	$0x0, 0x20(%rsp)
140940eac:     	movl	$0x0, 0x28(%rsp)
140940eb4:     	movw	$0x1, 0x50(%rsp)
140940ebb:     	movl	$0x0, 0x40(%rsp)
140940ec3:     	leaq	0x20(%rsp), %rdx
140940ec8:     	movq	%rax, %rcx
140940ecb:     	callq	0x14153ba70 <.text+0x153aa70>
140940ed0:     	movw	$0xcf, 0x48(%rsp)
140940ed7:     	movl	$0x0, 0x20(%rsp)
140940edf:     	movaps	0x217149a(%rip), %xmm0  # 0x142ab2380
140940ee6:     	movups	%xmm0, 0x28(%rsp)
140940eeb:     	movsd	0x217149d(%rip), %xmm0  # 0x142ab2390
140940ef3:     	movups	%xmm0, 0x38(%rsp)
140940ef8:     	leaq	0x20(%rsp), %rdx
140940efd:     	movq	%rax, %rcx
140940f00:     	callq	0x141539ea0 <.text+0x1538ea0>
140940f05:     	movsd	0x1ff36a3(%rip), %xmm0  # 0x1429345b0
140940f0d:     	movaps	%xmm0, 0x20(%rsp)
140940f12:     	leaq	0x20(%rsp), %rdx
140940f17:     	movq	%rax, %rcx
140940f1a:     	callq	0x14153e1f0 <.text+0x153d1f0>
140940f1f:     	movb	$0x0, 0x34(%rsp)
140940f24:     	movq	$0x0, 0x2c(%rsp)
140940f2d:     	movabsq	$0x800000000, %rcx      # imm = 0x800000000
140940f37:     	movq	%rcx, 0x38(%rsp)
140940f3c:     	movl	$0x0, 0x40(%rsp)
140940f44:     	movb	$0x1, 0x62(%rsp)
140940f49:     	movw	$0x0, 0x60(%rsp)
140940f50:     	movl	$0x0, 0x20(%rsp)
140940f58:     	movups	%xmm6, 0x44(%rsp)
140940f5d:     	movq	$0x0, 0x54(%rsp)
140940f66:     	movl	$0x3f800000, 0x5c(%rsp) # imm = 0x3F800000
140940f6e:     	movl	$0x186a3, 0x28(%rsp)    # imm = 0x186A3
140940f76:     	leaq	0x20(%rsp), %rdx
140940f7b:     	movq	%rax, %rcx
140940f7e:     	callq	0x14154e650 <.text+0x154d650>
140940f83:     	movups	%xmm6, 0x8c(%rsp)
140940f8b:     	movaps	%xmm6, 0x80(%rsp)
140940f93:     	movaps	%xmm6, 0x70(%rsp)
140940f98:     	movaps	%xmm6, 0x60(%rsp)
140940f9d:     	movaps	%xmm6, 0x50(%rsp)
140940fa2:     	movaps	%xmm6, 0x40(%rsp)
140940fa7:     	movaps	%xmm6, 0x30(%rsp)
140940fac:     	movaps	%xmm6, 0x20(%rsp)
140940fb1:     	leaq	0x20(%rsp), %rdx
140940fb6:     	movq	%rax, %rcx
140940fb9:     	callq	0x14154b300 <.text+0x154a300>
140940fbe:     	movq	$0x0, 0x20(%rsp)
140940fc7:     	movq	$0x8, 0x28(%rsp)
140940fd0:     	movq	$0x0, 0x30(%rsp)
140940fd9:     	leaq	0x20(%rsp), %rdx
140940fde:     	movq	%rax, %rcx
140940fe1:     	callq	0x141538780 <.text+0x1537780>
140940fe6:     	movq	%rax, %rsi
140940fe9:     	callq	0x140d4d740 <.text+0xd4c740>
140940fee:     	movq	%rax, %rdi
140940ff1:     	testb	$0x1, (%rax)
140940ff4:     	je	0x140941007 <.text+0x940007>
140940ff6:     	movq	%rdi, %rcx
140940ff9:     	addq	$0x8, %rcx
140940ffd:     	movq	0x8(%rdi), %rax
140941001:     	movq	0x10(%rdi), %rdx
140941005:     	jmp	0x14094101e <.text+0x94001e>
140941007:     	callq	0x140d64e00 <.text+0xd63e00>
14094100c:     	movq	%rdi, %rcx
14094100f:     	addq	$0x8, %rcx
140941013:     	movq	$0x1, (%rdi)
14094101a:     	movq	%rdx, 0x10(%rdi)
14094101e:     	leaq	0x1(%rax), %r8
140941022:     	movq	%r8, (%rcx)
140941025:     	movups	0x21714d4(%rip), %xmm0  # 0x142ab2500
14094102c:     	movaps	%xmm0, 0x30(%rsp)
140941031:     	movups	0x21714b8(%rip), %xmm0  # 0x142ab24f0
140941038:     	movaps	%xmm0, 0x20(%rsp)
14094103d:     	movq	%rax, 0x40(%rsp)
140941042:     	movq	%rdx, 0x48(%rsp)
140941047:     	leaq	0x20(%rsp), %rdx
14094104c:     	movq	%rsi, %rcx
14094104f:     	callq	0x1415408d0 <.text+0x153f8d0>
140941054:     	movq	%rax, %rsi
140941057:     	leaq	0x20(%rsp), %rdi
14094105c:     	movq	%rdi, %rcx
14094105f:     	xorl	%edx, %edx
140941061:     	callq	0x141449580 <.text+0x1448580>
140941066:     	movups	0x21a1f73(%rip), %xmm0  # 0x142ae2fe0
14094106d:     	movups	%xmm0, 0x60(%rsp)
140941072:     	movups	0x21a1f57(%rip), %xmm0  # 0x142ae2fd0
140941079:     	movups	%xmm0, 0x50(%rsp)
14094107e:     	movq	%rsi, %rcx
140941081:     	movq	%rdi, %rdx
140941084:     	callq	0x14153ae90 <.text+0x1539e90>
140941089:     	movq	%rax, %rsi
14094108c:     	movsd	0x217130c(%rip), %xmm0  # 0x142ab23a0
140941094:     	movsd	%xmm0, 0xa0(%rsp)
14094109d:     	movl	$0x41c00000, 0xa8(%rsp) # imm = 0x41C00000
1409410a8:     	movl	$0x0, 0xb8(%rsp)
1409410b3:     	movq	$0x0, 0xb0(%rsp)
1409410bf:     	leaq	0x20(%rsp), %rdi
1409410c4:     	leaq	0xa0(%rsp), %rdx
1409410cc:     	leaq	0xb0(%rsp), %r8
1409410d4:     	movq	%rdi, %rcx
1409410d7:     	callq	0x140b18550 <.text+0xb17550>
1409410dc:     	movq	%rsi, %rcx
1409410df:     	movq	%rdi, %rdx
1409410e2:     	callq	0x14154d590 <.text+0x154c590>
1409410e7:     	movq	%rax, %rcx
1409410ea:     	xorl	%edx, %edx
1409410ec:     	movaps	0xc0(%rsp), %xmm6
1409410f4:     	addq	$0xd8, %rsp
1409410fb:     	popq	%rdi
1409410fc:     	popq	%rsi
1409410fd:     	jmp	0x14152fac0 <.text+0x152eac0>
140941102:     	int3
140941103:     	int3
140941104:     	int3
140941105:     	int3
140941106:     	int3
140941107:     	int3
140941108:     	int3
140941109:     	int3
14094110a:     	int3
14094110b:     	int3
14094110c:     	int3
14094110d:     	int3
14094110e:     	int3
14094110f:     	int3


D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409ed860:     	pushq	%rbp
1409ed861:     	pushq	%r15
1409ed863:     	pushq	%r14
1409ed865:     	pushq	%r13
1409ed867:     	pushq	%r12
1409ed869:     	pushq	%rsi
1409ed86a:     	pushq	%rdi
1409ed86b:     	pushq	%rbx
1409ed86c:     	subq	$0x8d8, %rsp            # imm = 0x8D8
1409ed873:     	leaq	0x80(%rsp), %rbp
1409ed87b:     	movq	$-0x2, 0x850(%rbp)
1409ed886:     	movq	%rdx, %rdi
1409ed889:     	movq	%rcx, 0x7c8(%rbp)
1409ed890:     	leaq	0x794(%rbp), %rcx
1409ed897:     	movq	%r8, 0x7b8(%rbp)
1409ed89e:     	callq	0x1409ee1e0 <.text+0x9ed1e0>
1409ed8a3:     	movups	0x798(%rbp), %xmm0
1409ed8aa:     	movups	0x7a8(%rbp), %xmm1
1409ed8b1:     	movaps	%xmm1, 0x610(%rbp)
1409ed8b8:     	movaps	%xmm0, 0x600(%rbp)
1409ed8bf:     	leaq	0x470(%rbp), %rsi
1409ed8c6:     	leaq	0x600(%rbp), %rdx
1409ed8cd:     	movq	%rsi, %rcx
1409ed8d0:     	callq	0x140bd5370 <.text+0xbd4370>
1409ed8d5:     	leaq	0x2e0(%rbp), %rcx
1409ed8dc:     	movq	%rsi, %rdx
1409ed8df:     	callq	0x140bd5390 <.text+0xbd4390>
1409ed8e4:     	movups	0x2e0(%rbp), %xmm0
1409ed8eb:     	movups	0x2f0(%rbp), %xmm1
1409ed8f2:     	movaps	%xmm1, 0x480(%rbp)
1409ed8f9:     	movaps	%xmm0, 0x470(%rbp)
1409ed900:     	leaq	0x610(%rbp), %rcx
1409ed907:     	leaq	0x470(%rbp), %r15
1409ed90e:     	movq	%r15, %rdx
1409ed911:     	callq	0x140bd6ce0 <.text+0xbd5ce0>
1409ed916:     	leaq	0x20d7a13(%rip), %rax   # 0x142ac5330
1409ed91d:     	movq	%rax, 0x600(%rbp)
1409ed924:     	movq	$0x11, 0x608(%rbp)
1409ed92f:     	leaq	0x600(%rbp), %rdx
1409ed936:     	movq	%rdi, 0x7c0(%rbp)
1409ed93d:     	movq	%rdi, %rcx
1409ed940:     	callq	0x140be94d0 <.text+0xbe84d0>
1409ed945:     	movups	0x2e0(%rbp), %xmm0
1409ed94c:     	movups	0x2f0(%rbp), %xmm1
1409ed953:     	movups	%xmm1, 0x820(%rbp)
1409ed95a:     	movups	%xmm0, 0x810(%rbp)
1409ed961:     	movl	%eax, 0x80c(%rbp)
1409ed967:     	movzbl	0x82c(%rbp), %eax
1409ed96e:     	movq	%rax, 0x830(%rbp)
1409ed975:     	testl	%eax, %eax
1409ed977:     	je	0x1409eded9 <.text+0x9eced9>
1409ed97d:     	movq	0x830(%rbp), %rsi
1409ed984:     	leal	-0x1(%rsi), %eax
1409ed987:     	movl	%eax, 0x83c(%rbp)
1409ed98d:     	leaq	0x7d0(%rbp), %rdi
1409ed994:     	jmp	0x1409ed9aa <.text+0x9ec9aa>
1409ed996:     	nopw	%cs:(%rax,%rax)
1409ed9a0:     	movl	%ebx, %esi
1409ed9a2:     	testl	%ebx, %ebx
1409ed9a4:     	je	0x1409eded9 <.text+0x9eced9>
1409ed9aa:     	leal	-0x1(%rsi), %ebx
1409ed9ad:     	movl	$0x1, %eax
1409ed9b2:     	movl	%ebx, %ecx
1409ed9b4:     	shll	%cl, %eax
1409ed9b6:     	leaq	0x810(%rbp), %rcx
1409ed9bd:     	movups	(%rcx), %xmm0
1409ed9c0:     	movups	0x10(%rcx), %xmm1
1409ed9c4:     	movaps	%xmm1, 0x610(%rbp)
1409ed9cb:     	movaps	%xmm0, 0x600(%rbp)
1409ed9d2:     	movl	%eax, 0x2e0(%rbp)
1409ed9d8:     	movl	%eax, 0x2e4(%rbp)
1409ed9de:     	movl	$0x1, 0x2e8(%rbp)
1409ed9e8:     	movq	%r15, %rcx
1409ed9eb:     	leaq	0x600(%rbp), %r14
1409ed9f2:     	movq	%r14, %rdx
1409ed9f5:     	leaq	0x2e0(%rbp), %r8
1409ed9fc:     	callq	0x140bd56a0 <.text+0xbd46a0>
1409eda01:     	movq	0x47c(%rbp), %r13
1409eda08:     	movl	0x484(%rbp), %eax
1409eda0e:     	movl	%eax, 0x838(%rbp)
1409eda14:     	movq	0x7c0(%rbp), %rax
1409eda1b:     	movq	%rax, 0x30(%rsp)
1409eda20:     	movq	0x7b8(%rbp), %rax
1409eda27:     	movq	%rax, 0x28(%rsp)
1409eda2c:     	movq	$0x1e, 0x20(%rsp)
1409eda35:     	movl	$0x9, %r8d
1409eda3b:     	movq	%r14, %rcx
1409eda3e:     	leaq	0x20d78fc(%rip), %rdx   # 0x142ac5341
1409eda45:     	leaq	0x20d78fe(%rip), %r9    # 0x142ac534a
1409eda4c:     	callq	0x1416032a0 <.text+0x16022a0>
1409eda51:     	movb	$0x1, 0x84f(%rbp)
1409eda58:     	movq	%r15, %rcx
1409eda5b:     	callq	0x140bd57a0 <.text+0xbd47a0>
1409eda60:     	movb	$0x1, 0x84f(%rbp)
1409eda67:     	movq	%rdi, %rcx
1409eda6a:     	movq	%r15, %rdx
1409eda6d:     	movl	$0x4, %r8d
1409eda73:     	callq	0x140bd5820 <.text+0xbd4820>
1409eda78:     	movb	$0x1, 0x84f(%rbp)
1409eda7f:     	movq	%r15, %rcx
1409eda82:     	movq	%rdi, %rdx
1409eda85:     	movl	%ebx, %r8d
1409eda88:     	callq	0x140bd58e0 <.text+0xbd48e0>
1409eda8d:     	movb	$0x1, 0x84f(%rbp)
1409eda94:     	movq	%rdi, %rcx
1409eda97:     	movq	%r15, %rdx
1409eda9a:     	movl	$0x1, %r8d
1409edaa0:     	movl	$0x1, %r9d
1409edaa6:     	callq	0x140bd5920 <.text+0xbd4920>
1409edaab:     	leaq	0x794(%rbp), %rax
1409edab2:     	movq	%rax, 0x840(%rbp)
1409edab9:     	movups	0x7d0(%rbp), %xmm0
1409edac0:     	movups	0x7e0(%rbp), %xmm1
1409edac7:     	movups	0x7f0(%rbp), %xmm2
1409edace:     	movups	0x7fc(%rbp), %xmm3
1409edad5:     	movups	%xmm3, 0x49c(%rbp)
1409edadc:     	movaps	%xmm2, 0x490(%rbp)
1409edae3:     	movaps	%xmm1, 0x480(%rbp)
1409edaea:     	movaps	%xmm0, 0x470(%rbp)
1409edaf1:     	movq	%r15, 0x20(%rsp)
1409edaf6:     	leaq	0x840(%rbp), %rcx
1409edafd:     	movq	%r14, %rdx
1409edb00:     	movabsq	$-0x232030d7614fd198, %r8 # imm = 0xDCDFCF289EB02E68
1409edb0a:     	movb	$0x19, %r9b
1409edb0d:     	callq	0x1408ee240 <.text+0x8ed240>
1409edb12:     	movl	$0x190, %r8d            # imm = 0x190
1409edb18:     	movq	%r15, %r12
1409edb1b:     	movq	%r15, %rcx
1409edb1e:     	movq	%r14, %rdx
1409edb21:     	callq	0x142923c00 <.text+0x2922c00>
1409edb26:     	movb	$0x1, 0x84e(%rbp)
1409edb2d:     	movq	%r14, %rcx
1409edb30:     	callq	0x140bd57a0 <.text+0xbd47a0>
1409edb35:     	leaq	0x7d0(%rbp), %rdi
1409edb3c:     	movb	$0x1, 0x84e(%rbp)
1409edb43:     	movq	%rdi, %rcx
1409edb46:     	movq	%r14, %rdx
1409edb49:     	movl	$0x4, %r8d
1409edb4f:     	callq	0x140bd5820 <.text+0xbd4820>
1409edb54:     	movl	0x83c(%rbp), %eax
1409edb5a:     	cmpl	%esi, %eax
1409edb5c:     	movl	%esi, %r8d
1409edb5f:     	cmovbl	%eax, %r8d
1409edb63:     	movb	$0x1, 0x84e(%rbp)
1409edb6a:     	movq	%r14, %rcx
1409edb6d:     	movq	%rdi, %rdx
1409edb70:     	callq	0x140bd58e0 <.text+0xbd48e0>
1409edb75:     	movb	$0x1, 0x84e(%rbp)
1409edb7c:     	movq	%rdi, %rcx
1409edb7f:     	movq	%r14, %rdx
1409edb82:     	movl	$0x1, %r8d
1409edb88:     	movl	$0x1, %r9d
1409edb8e:     	callq	0x140bd5920 <.text+0xbd4920>
1409edb93:     	leaq	0x80c(%rbp), %rax
1409edb9a:     	movq	%rax, 0x840(%rbp)
1409edba1:     	movups	0x7d0(%rbp), %xmm0
1409edba8:     	movups	0x7e0(%rbp), %xmm1
1409edbaf:     	movups	0x7f0(%rbp), %xmm2
1409edbb6:     	movups	0x7fc(%rbp), %xmm3
1409edbbd:     	movups	%xmm3, 0x62c(%rbp)
1409edbc4:     	movaps	%xmm2, 0x620(%rbp)
1409edbcb:     	movaps	%xmm1, 0x610(%rbp)
1409edbd2:     	movaps	%xmm0, 0x600(%rbp)
1409edbd9:     	movq	%r14, 0x20(%rsp)
1409edbde:     	leaq	0x840(%rbp), %rcx
1409edbe5:     	movq	%r12, %rdx
1409edbe8:     	movabsq	$0x11ba5697cc40b3dd, %r8 # imm = 0x11BA5697CC40B3DD
1409edbf2:     	movb	$0x19, %r9b
1409edbf5:     	callq	0x1408ee240 <.text+0x8ed240>
1409edbfa:     	movl	$0x190, %r8d            # imm = 0x190
1409edc00:     	movq	%r14, %rcx
1409edc03:     	movq	%r12, %rdx
1409edc06:     	callq	0x142923c00 <.text+0x2922c00>
1409edc0b:     	movb	$0x1, 0x84d(%rbp)
1409edc12:     	movq	%r12, %rcx
1409edc15:     	callq	0x140bd57a0 <.text+0xbd47a0>
1409edc1a:     	leaq	-0x40(%rbp), %r14
1409edc1e:     	movq	%r12, %r15
1409edc21:     	movb	$0x1, 0x84d(%rbp)
1409edc28:     	movq	%rdi, %rcx
1409edc2b:     	movq	%r12, %rdx
1409edc2e:     	movl	$0x8, %r8d
1409edc34:     	callq	0x140bd5820 <.text+0xbd4820>
1409edc39:     	movb	$0x1, 0x84d(%rbp)
1409edc40:     	movq	%r15, %rcx
1409edc43:     	movq	%rdi, %rdx
1409edc46:     	movl	%ebx, %r8d
1409edc49:     	callq	0x140bd58e0 <.text+0xbd48e0>
1409edc4e:     	movb	$0x1, 0x84d(%rbp)
1409edc55:     	movq	%rdi, %rcx
1409edc58:     	movq	%r15, %rdx
1409edc5b:     	movl	$0x1, %r8d
1409edc61:     	movl	$0x1, %r9d
1409edc67:     	callq	0x140bd5920 <.text+0xbd4920>
1409edc6c:     	movups	0x7d0(%rbp), %xmm0
1409edc73:     	movups	0x7e0(%rbp), %xmm1
1409edc7a:     	movups	0x7f0(%rbp), %xmm2
1409edc81:     	movups	0x7fc(%rbp), %xmm3
1409edc88:     	movaps	%xmm0, 0x470(%rbp)
1409edc8f:     	movaps	%xmm1, 0x480(%rbp)
1409edc96:     	movaps	%xmm2, 0x490(%rbp)
1409edc9d:     	movups	%xmm3, 0x49c(%rbp)
1409edca4:     	leaq	0x80c(%rbp), %rax
1409edcab:     	movq	%rax, 0x7d0(%rbp)
1409edcb2:     	movq	%r15, 0x20(%rsp)
1409edcb7:     	movq	%rdi, %rcx
1409edcba:     	leaq	0x600(%rbp), %rdi
1409edcc1:     	movq	%rdi, %rdx
1409edcc4:     	movabsq	$-0x77de65c519676d4e, %r8 # imm = 0x88219A3AE69892B2
1409edcce:     	movb	$0x37, %r9b
1409edcd1:     	callq	0x1408edd00 <.text+0x8ecd00>
1409edcd6:     	movl	$0x190, %r8d            # imm = 0x190
1409edcdc:     	leaq	0x2e0(%rbp), %rcx
1409edce3:     	movq	%rdi, %rdx
1409edce6:     	callq	0x142923c00 <.text+0x2922c00>
1409edceb:     	cmpb	$0x8, 0x416(%rbp)
1409edcf2:     	je	0x1409edd1c <.text+0x9ecd1c>
1409edcf4:     	movq	0x418(%rbp), %rdx
1409edcfb:     	movl	$0xe8, %eax
1409edd00:     	addq	%rax, %rdx
1409edd03:     	leaq	0x3f0(%rbp), %rcx
1409edd0a:     	movabsq	$0x51fcf121d36542ad, %r8 # imm = 0x51FCF121D36542AD
1409edd14:     	movq	%r13, %r9
1409edd17:     	callq	0x14096b760 <.text+0x96a760>
1409edd1c:     	movl	$0x190, %r8d            # imm = 0x190
1409edd22:     	movq	%r14, %rcx
1409edd25:     	leaq	0x2e0(%rbp), %rdx
1409edd2c:     	callq	0x142923c00 <.text+0x2922c00>
1409edd31:     	cmpb	$0x8, 0xf6(%rbp)
1409edd38:     	je	0x1409edd69 <.text+0x9ecd69>
1409edd3a:     	cmpl	0x830(%rbp), %esi
1409edd40:     	setne	%r9b
1409edd44:     	movq	0xf8(%rbp), %rdx
1409edd4b:     	movl	$0xe8, %eax
1409edd50:     	addq	%rax, %rdx
1409edd53:     	leaq	0xd0(%rbp), %rcx
1409edd5a:     	movabsq	$-0x41597cdfb5b942ee, %r8 # imm = 0xBEA683204A46BD12
1409edd64:     	callq	0x14096b850 <.text+0x96a850>
1409edd69:     	movq	%r13, %rsi
1409edd6c:     	shrq	$0x20, %rsi
1409edd70:     	movl	$0x190, %r8d            # imm = 0x190
1409edd76:     	leaq	0x150(%rbp), %rcx
1409edd7d:     	movq	%r14, %rdx
1409edd80:     	callq	0x142923c00 <.text+0x2922c00>
1409edd85:     	leaq	0x1b0(%rbp), %r9
1409edd8c:     	movups	0x40(%r9), %xmm0
1409edd91:     	movaps	%xmm0, 0x4b0(%rbp)
1409edd98:     	movups	(%r9), %xmm0
1409edd9c:     	movups	0x10(%r9), %xmm1
1409edda1:     	movups	0x20(%r9), %xmm2
1409edda6:     	movups	0x30(%r9), %xmm3
1409eddab:     	movaps	%xmm3, 0x4a0(%rbp)
1409eddb2:     	movaps	%xmm2, 0x490(%rbp)
1409eddb9:     	movaps	%xmm1, 0x480(%rbp)
1409eddc0:     	movaps	%xmm0, 0x470(%rbp)
1409eddc7:     	movq	0x288(%rbp), %rax
1409eddce:     	movq	0x2d0(%rbp), %rcx
1409eddd5:     	movq	0x160(%rbp), %rdx
1409edddc:     	movq	%rdx, 0x610(%rbp)
1409edde3:     	movups	0x150(%rbp), %xmm0
1409eddea:     	movaps	%xmm0, 0x600(%rbp)
1409eddf1:     	movq	-0x38(%r9), %rdx
1409eddf5:     	leaq	0x630(%rbp), %r8
1409eddfc:     	movq	%rdx, 0x10(%r8)
1409ede00:     	movups	-0x48(%r9), %xmm0
1409ede05:     	movups	%xmm0, (%r8)
1409ede09:     	movq	-0x20(%r9), %rdx
1409ede0d:     	movq	%rdx, -0x8(%r8)
1409ede11:     	movups	-0x30(%r9), %xmm0
1409ede16:     	movups	%xmm0, -0x18(%r8)
1409ede1b:     	movups	0xa0(%r9), %xmm0
1409ede23:     	movups	%xmm0, 0x98(%r8)
1409ede2b:     	movups	0x90(%r9), %xmm0
1409ede33:     	movups	%xmm0, 0x88(%r8)
1409ede3b:     	movups	0x50(%r9), %xmm0
1409ede40:     	movups	0x60(%r9), %xmm1
1409ede45:     	movups	0x70(%r9), %xmm2
1409ede4a:     	movups	0x80(%r9), %xmm3
1409ede52:     	movups	%xmm3, 0x78(%r8)
1409ede57:     	movups	%xmm2, 0x68(%r8)
1409ede5c:     	movups	%xmm1, 0x58(%r8)
1409ede61:     	movups	%xmm0, 0x48(%r8)
1409ede66:     	movq	-0x8(%r9), %rdx
1409ede6a:     	movq	%rdx, 0x28(%r8)
1409ede6e:     	movups	-0x18(%r9), %xmm0
1409ede73:     	movups	%xmm0, 0x18(%r8)
1409ede78:     	movq	%rcx, 0x6e0(%rbp)
1409ede7f:     	movq	%rax, 0x6d8(%rbp)
1409ede86:     	movl	$0x1, 0x660(%rbp)
1409ede90:     	movl	%r13d, 0x664(%rbp)
1409ede97:     	movl	%esi, 0x668(%rbp)
1409ede9d:     	movl	0x838(%rbp), %eax
1409edea3:     	movl	%eax, 0x66c(%rbp)
1409edea9:     	movq	%r15, %rcx
1409edeac:     	movq	%rdi, %rdx
1409edeaf:     	callq	0x1409dd4d0 <.text+0x9dc4d0>
1409edeb4:     	cmpb	$0x8, 0x286(%rbp)
1409edebb:     	leaq	0x7d0(%rbp), %rdi
1409edec2:     	je	0x1409ed9a0 <.text+0x9ec9a0>
1409edec8:     	leaq	0x270(%rbp), %rcx
1409edecf:     	callq	0x140be24b0 <.text+0xbe14b0>
1409eded4:     	jmp	0x1409ed9a0 <.text+0x9ec9a0>
1409eded9:     	movl	0x82c(%rbp), %ecx
1409ededf:     	movq	0x7c8(%rbp), %rax
1409edee6:     	movl	%ecx, 0x20(%rax)
1409edee9:     	movl	0x80c(%rbp), %r8d
1409edef0:     	movq	0x810(%rbp), %rcx
1409edef7:     	movl	0x818(%rbp), %edx
1409edefd:     	movups	0x81c(%rbp), %xmm0
1409edf04:     	movups	%xmm0, 0x10(%rax)
1409edf08:     	movl	%r8d, (%rax)
1409edf0b:     	movq	%rcx, 0x4(%rax)
1409edf0f:     	movl	%edx, 0xc(%rax)
1409edf12:     	addq	$0x8d8, %rsp            # imm = 0x8D8
1409edf19:     	popq	%rbx
1409edf1a:     	popq	%rdi
1409edf1b:     	popq	%rsi
1409edf1c:     	popq	%r12
1409edf1e:     	popq	%r13
1409edf20:     	popq	%r14
1409edf22:     	popq	%r15
1409edf24:     	popq	%rbp
1409edf25:     	retq
1409edf26:     	nopw	%cs:(%rax,%rax)
1409edf30:     	movq	%rdx, 0x10(%rsp)
1409edf35:     	pushq	%rbp
1409edf36:     	pushq	%r15
1409edf38:     	pushq	%r14
1409edf3a:     	pushq	%r13
1409edf3c:     	pushq	%r12
1409edf3e:     	pushq	%rsi
1409edf3f:     	pushq	%rdi
1409edf40:     	pushq	%rbx
1409edf41:     	subq	$0x38, %rsp
1409edf45:     	leaq	0x80(%rdx), %rbp
1409edf4c:     	cmpb	$0x8, 0x286(%rbp)
1409edf53:     	je	0x1409edf61 <.text+0x9ecf61>
1409edf55:     	leaq	0x270(%rbp), %rcx
1409edf5c:     	callq	0x140be24b0 <.text+0xbe14b0>
1409edf61:     	nop
1409edf62:     	addq	$0x38, %rsp
1409edf66:     	popq	%rbx
1409edf67:     	popq	%rdi
1409edf68:     	popq	%rsi
1409edf69:     	popq	%r12
1409edf6b:     	popq	%r13
1409edf6d:     	popq	%r14
1409edf6f:     	popq	%r15
1409edf71:     	popq	%rbp
1409edf72:     	retq
1409edf73:     	nopw	%cs:(%rax,%rax)
1409edf80:     	movq	%rdx, 0x10(%rsp)
1409edf85:     	pushq	%rbp
1409edf86:     	pushq	%r15
1409edf88:     	pushq	%r14
1409edf8a:     	pushq	%r13
1409edf8c:     	pushq	%r12
1409edf8e:     	pushq	%rsi
1409edf8f:     	pushq	%rdi
1409edf90:     	pushq	%rbx
1409edf91:     	subq	$0x38, %rsp
1409edf95:     	leaq	0x80(%rdx), %rbp
1409edf9c:     	cmpb	$0x0, 0x84d(%rbp)
1409edfa3:     	jne	0x1409edfb6 <.text+0x9ecfb6>
1409edfa5:     	addq	$0x38, %rsp
1409edfa9:     	popq	%rbx
1409edfaa:     	popq	%rdi
1409edfab:     	popq	%rsi
1409edfac:     	popq	%r12
1409edfae:     	popq	%r13
1409edfb0:     	popq	%r14
1409edfb2:     	popq	%r15
1409edfb4:     	popq	%rbp
1409edfb5:     	retq
1409edfb6:     	leaq	0x600(%rbp), %rcx
1409edfbd:     	callq	0x140896fb0 <.text+0x895fb0>
1409edfc2:     	jmp	0x1409edfa5 <.text+0x9ecfa5>
1409edfc4:     	nopw	%cs:(%rax,%rax)
1409edfd0:     	movq	%rdx, 0x10(%rsp)
1409edfd5:     	pushq	%rbp
1409edfd6:     	pushq	%r15
1409edfd8:     	pushq	%r14
1409edfda:     	pushq	%r13
1409edfdc:     	pushq	%r12
1409edfde:     	pushq	%rsi
1409edfdf:     	pushq	%rdi
1409edfe0:     	pushq	%rbx
1409edfe1:     	subq	$0x38, %rsp
1409edfe5:     	leaq	0x80(%rdx), %rbp
1409edfec:     	cmpb	$0x0, 0x84e(%rbp)
1409edff3:     	jne	0x1409ee006 <.text+0x9ed006>
1409edff5:     	addq	$0x38, %rsp
1409edff9:     	popq	%rbx
1409edffa:     	popq	%rdi
1409edffb:     	popq	%rsi
1409edffc:     	popq	%r12
1409edffe:     	popq	%r13
1409ee000:     	popq	%r14
1409ee002:     	popq	%r15
1409ee004:     	popq	%rbp
1409ee005:     	retq
1409ee006:     	leaq	0x470(%rbp), %rcx
1409ee00d:     	callq	0x140896fb0 <.text+0x895fb0>
1409ee012:     	jmp	0x1409edff5 <.text+0x9ecff5>
1409ee014:     	nopw	%cs:(%rax,%rax)
1409ee020:     	movq	%rdx, 0x10(%rsp)
1409ee025:     	pushq	%rbp
1409ee026:     	pushq	%r15
1409ee028:     	pushq	%r14
1409ee02a:     	pushq	%r13
1409ee02c:     	pushq	%r12
1409ee02e:     	pushq	%rsi
1409ee02f:     	pushq	%rdi
1409ee030:     	pushq	%rbx
1409ee031:     	subq	$0x38, %rsp
1409ee035:     	leaq	0x80(%rdx), %rbp
1409ee03c:     	cmpb	$0x0, 0x84f(%rbp)
1409ee043:     	jne	0x1409ee056 <.text+0x9ed056>
1409ee045:     	addq	$0x38, %rsp
1409ee049:     	popq	%rbx
1409ee04a:     	popq	%rdi
1409ee04b:     	popq	%rsi
1409ee04c:     	popq	%r12
1409ee04e:     	popq	%r13
1409ee050:     	popq	%r14
1409ee052:     	popq	%r15
1409ee054:     	popq	%rbp
1409ee055:     	retq
1409ee056:     	leaq	0x600(%rbp), %rcx
1409ee05d:     	callq	0x140896fb0 <.text+0x895fb0>
1409ee062:     	jmp	0x1409ee045 <.text+0x9ed045>
1409ee064:     	nopw	%cs:(%rax,%rax)
1409ee070:     	movq	%rdx, 0x10(%rsp)
1409ee075:     	pushq	%rbp
1409ee076:     	pushq	%r15
1409ee078:     	pushq	%r14
1409ee07a:     	pushq	%r13
1409ee07c:     	pushq	%r12
1409ee07e:     	pushq	%rsi
1409ee07f:     	pushq	%rdi
1409ee080:     	pushq	%rbx
1409ee081:     	subq	$0x38, %rsp
1409ee085:     	leaq	0x80(%rdx), %rbp
1409ee08c:     	movb	$0x0, 0x84f(%rbp)
1409ee093:     	leaq	0x600(%rbp), %rcx
1409ee09a:     	callq	0x140896fb0 <.text+0x895fb0>
1409ee09f:     	movb	$0x0, 0x84f(%rbp)
1409ee0a6:     	addq	$0x38, %rsp
1409ee0aa:     	popq	%rbx
1409ee0ab:     	popq	%rdi
1409ee0ac:     	popq	%rsi
1409ee0ad:     	popq	%r12
1409ee0af:     	popq	%r13
1409ee0b1:     	popq	%r14
1409ee0b3:     	popq	%r15
1409ee0b5:     	popq	%rbp
1409ee0b6:     	retq
1409ee0b7:     	nopw	(%rax,%rax)
1409ee0c0:     	movq	%rdx, 0x10(%rsp)
1409ee0c5:     	pushq	%rbp
1409ee0c6:     	pushq	%r15
1409ee0c8:     	pushq	%r14
1409ee0ca:     	pushq	%r13
1409ee0cc:     	pushq	%r12
1409ee0ce:     	pushq	%rsi
1409ee0cf:     	pushq	%rdi
1409ee0d0:     	pushq	%rbx
1409ee0d1:     	subq	$0x38, %rsp
1409ee0d5:     	leaq	0x80(%rdx), %rbp
1409ee0dc:     	movb	$0x0, 0x84e(%rbp)
1409ee0e3:     	leaq	0x470(%rbp), %rcx
1409ee0ea:     	callq	0x140896fb0 <.text+0x895fb0>
1409ee0ef:     	movb	$0x0, 0x84e(%rbp)
1409ee0f6:     	addq	$0x38, %rsp
1409ee0fa:     	popq	%rbx
1409ee0fb:     	popq	%rdi
1409ee0fc:     	popq	%rsi
1409ee0fd:     	popq	%r12
1409ee0ff:     	popq	%r13
1409ee101:     	popq	%r14
1409ee103:     	popq	%r15
1409ee105:     	popq	%rbp
1409ee106:     	retq
1409ee107:     	nopw	(%rax,%rax)
1409ee110:     	movq	%rdx, 0x10(%rsp)
1409ee115:     	pushq	%rbp
1409ee116:     	pushq	%r15
1409ee118:     	pushq	%r14
1409ee11a:     	pushq	%r13
1409ee11c:     	pushq	%r12
1409ee11e:     	pushq	%rsi
1409ee11f:     	pushq	%rdi
1409ee120:     	pushq	%rbx
1409ee121:     	subq	$0x38, %rsp
1409ee125:     	leaq	0x80(%rdx), %rbp
1409ee12c:     	movb	$0x0, 0x84d(%rbp)
1409ee133:     	leaq	0x600(%rbp), %rcx
1409ee13a:     	callq	0x140896fb0 <.text+0x895fb0>
1409ee13f:     	movb	$0x0, 0x84d(%rbp)
1409ee146:     	addq	$0x38, %rsp
1409ee14a:     	popq	%rbx
1409ee14b:     	popq	%rdi
1409ee14c:     	popq	%rsi
1409ee14d:     	popq	%r12
1409ee14f:     	popq	%r13
1409ee151:     	popq	%r14
1409ee153:     	popq	%r15
1409ee155:     	popq	%rbp
1409ee156:     	retq
1409ee157:     	nopw	(%rax,%rax)
1409ee160:     	movq	%rdx, 0x10(%rsp)
1409ee165:     	pushq	%rbp
1409ee166:     	pushq	%r15
1409ee168:     	pushq	%r14
1409ee16a:     	pushq	%r13
1409ee16c:     	pushq	%r12
1409ee16e:     	pushq	%rsi
1409ee16f:     	pushq	%rdi
1409ee170:     	pushq	%rbx
1409ee171:     	subq	$0x38, %rsp
1409ee175:     	leaq	0x80(%rdx), %rbp
1409ee17c:     	leaq	0x2e0(%rbp), %rcx
1409ee183:     	callq	0x140896fb0 <.text+0x895fb0>
1409ee188:     	nop
1409ee189:     	addq	$0x38, %rsp
1409ee18d:     	popq	%rbx
1409ee18e:     	popq	%rdi
1409ee18f:     	popq	%rsi
1409ee190:     	popq	%r12
1409ee192:     	popq	%r13
1409ee194:     	popq	%r14
1409ee196:     	popq	%r15
1409ee198:     	popq	%rbp
1409ee199:     	retq
1409ee19a:     	nopw	(%rax,%rax)
1409ee1a0:     	movq	%rdx, 0x10(%rsp)
1409ee1a5:     	pushq	%rbp
1409ee1a6:     	pushq	%r15
1409ee1a8:     	pushq	%r14
1409ee1aa:     	pushq	%r13
1409ee1ac:     	pushq	%r12
1409ee1ae:     	pushq	%rsi
1409ee1af:     	pushq	%rdi
1409ee1b0:     	pushq	%rbx
1409ee1b1:     	subq	$0x38, %rsp
1409ee1b5:     	leaq	0x80(%rdx), %rbp
1409ee1bc:     	leaq	-0x40(%rbp), %rcx
1409ee1c0:     	callq	0x140896fb0 <.text+0x895fb0>
1409ee1c5:     	nop
1409ee1c6:     	addq	$0x38, %rsp
1409ee1ca:     	popq	%rbx
1409ee1cb:     	popq	%rdi
1409ee1cc:     	popq	%rsi
1409ee1cd:     	popq	%r12
1409ee1cf:     	popq	%r13
1409ee1d1:     	popq	%r14
1409ee1d3:     	popq	%r15
1409ee1d5:     	popq	%rbp
1409ee1d6:     	retq

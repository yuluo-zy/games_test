
../B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140008c90:     	pushq	%rbp
140008c91:     	pushq	%r15
140008c93:     	pushq	%r14
140008c95:     	pushq	%r13
140008c97:     	pushq	%r12
140008c99:     	pushq	%rsi
140008c9a:     	pushq	%rdi
140008c9b:     	pushq	%rbx
140008c9c:     	subq	$0x948, %rsp            # imm = 0x948
140008ca3:     	leaq	0x80(%rsp), %rbp
140008cab:     	movdqa	%xmm15, 0x8b0(%rbp)
140008cb4:     	movaps	%xmm14, 0x8a0(%rbp)
140008cbc:     	movdqa	%xmm13, 0x890(%rbp)
140008cc5:     	movdqa	%xmm12, 0x880(%rbp)
140008cce:     	movaps	%xmm11, 0x870(%rbp)
140008cd6:     	movaps	%xmm10, 0x860(%rbp)
140008cde:     	movaps	%xmm9, 0x850(%rbp)
140008ce6:     	movaps	%xmm8, 0x840(%rbp)
140008cee:     	movaps	%xmm7, 0x830(%rbp)
140008cf5:     	movaps	%xmm6, 0x820(%rbp)
140008cfc:     	movq	$-0x2, 0x818(%rbp)
140008d07:     	movq	%r9, %r15
140008d0a:     	movq	%r8, 0x378(%rbp)
140008d11:     	movq	%rdx, 0x268(%rbp)
140008d18:     	movq	%rcx, %rdi
140008d1b:     	movq	0x930(%rbp), %rax
140008d22:     	movq	(%rax), %rcx
140008d25:     	movq	%rcx, 0x7c0(%rbp)
140008d2c:     	movq	0x10(%rax), %r12
140008d30:     	movl	0x1c(%rax), %r13d
140008d34:     	movq	0x20(%rax), %rcx
140008d38:     	movq	%rcx, 0x2f0(%rbp)
140008d3f:     	movq	0x30(%rax), %rcx
140008d43:     	movq	%rcx, 0x90(%rbp)
140008d4a:     	movl	0x3c(%rax), %ecx
140008d4d:     	movl	%ecx, 0x238(%rbp)
140008d53:     	movq	0x40(%rax), %rcx
140008d57:     	movq	%rcx, 0x370(%rbp)
140008d5e:     	movq	0x50(%rax), %rcx
140008d62:     	movq	%rcx, 0x340(%rbp)
140008d69:     	movl	0x5c(%rax), %ecx
140008d6c:     	movl	%ecx, 0x53c(%rbp)
140008d72:     	movq	0x60(%rax), %rcx
140008d76:     	movq	%rcx, 0x348(%rbp)
140008d7d:     	movq	0x70(%rax), %rcx
140008d81:     	movq	%rcx, 0x258(%rbp)
140008d88:     	movl	0x7c(%rax), %ecx
140008d8b:     	movl	%ecx, 0x424(%rbp)
140008d91:     	movq	0x80(%rax), %rcx
140008d98:     	movq	%rcx, 0x240(%rbp)
140008d9f:     	movq	0xa0(%rax), %rax
140008da6:     	movq	%rax, 0x338(%rbp)
140008dad:     	movq	0x960(%rbp), %rax
140008db4:     	movq	(%rax), %rcx
140008db7:     	movq	%rcx, 0xe8(%rbp)
140008dbe:     	movq	0x20(%rax), %rcx
140008dc2:     	movq	%rcx, 0x260(%rbp)
140008dc9:     	movq	0x28(%rax), %rax
140008dcd:     	movq	%rax, 0x250(%rbp)
140008dd4:     	callq	0x140c16a00 <.text+0xc15a00>
140008dd9:     	movb	%al, 0x797(%rbp)
140008ddf:     	testb	%al, %al
140008de1:     	je	0x140009017 <.text+0x8017>
140008de7:     	movl	$0x4f, %r14d
140008ded:     	movl	$0x4f, %ecx
140008df2:     	callq	0x1400299b0 <.text+0x289b0>
140008df7:     	leaq	0x291e5b2(%rip), %rsi   # 0x1429273b0
140008dfe:     	cmpq	$0x1, %rax
140008e02:     	jne	0x140008ea9 <.text+0x7ea9>
140008e08:     	movq	%rdx, %r9
140008e0b:     	testq	%rdx, %rdx
140008e0e:     	je	0x140008e46 <.text+0x7e46>
140008e10:     	cmpq	$0x4f, %r9
140008e14:     	jae	0x140008e44 <.text+0x7e44>
140008e16:     	leaq	0x291e593(%rip), %rax   # 0x1429273b0
140008e1d:     	cmpb	$-0x41, (%r9,%rax)
140008e22:     	jg	0x140008e46 <.text+0x7e46>
140008e24:     	leaq	0x291e64d(%rip), %rax   # 0x142927478
140008e2b:     	movq	%rax, 0x20(%rsp)
140008e30:     	leaq	0x291e579(%rip), %rcx   # 0x1429273b0
140008e37:     	movl	$0x4f, %edx
140008e3c:     	xorl	%r8d, %r8d
140008e3f:     	callq	0x1428d9940 <.text+0x28d8940>
140008e44:     	jne	0x140008e24 <.text+0x7e24>
140008e46:     	movq	%r9, %rcx
140008e49:     	callq	0x1400299b0 <.text+0x289b0>
140008e4e:     	cmpq	$0x1, %rax
140008e52:     	jne	0x140008ea9 <.text+0x7ea9>
140008e54:     	movq	%rdx, %r8
140008e57:     	addq	$0x2, %r8
140008e5b:     	je	0x140008e96 <.text+0x7e96>
140008e5d:     	cmpq	$0x4f, %r8
140008e61:     	jae	0x140008e94 <.text+0x7e94>
140008e63:     	leaq	0x291e546(%rip), %rax   # 0x1429273b0
140008e6a:     	cmpb	$-0x41, (%r8,%rax)
140008e6f:     	jg	0x140008e96 <.text+0x7e96>
140008e71:     	leaq	0x291e618(%rip), %rax   # 0x142927490
140008e78:     	movq	%rax, 0x20(%rsp)
140008e7d:     	leaq	0x291e52c(%rip), %rcx   # 0x1429273b0
140008e84:     	movl	$0x4f, %edx
140008e89:     	movl	$0x4f, %r9d
140008e8f:     	callq	0x1428d9940 <.text+0x28d8940>
140008e94:     	jne	0x140008e71 <.text+0x7e71>
140008e96:     	movl	$0x4d, %r14d
140008e9c:     	subq	%rdx, %r14
140008e9f:     	leaq	0x291e50a(%rip), %rsi   # 0x1429273b0
140008ea6:     	addq	%r8, %rsi
140008ea9:     	movq	%r15, 0x800(%rbp)
140008eb0:     	leaq	0x291d5f1(%rip), %r8    # 0x1429264a8
140008eb7:     	leaq	0x291d5aa(%rip), %r15   # 0x142926468
140008ebe:     	nop
140008ec0:     	movsbl	-0x1(%r8), %eax
140008ec5:     	testl	%eax, %eax
140008ec7:     	js	0x140008ee0 <.text+0x7ee0>
140008ec9:     	decq	%r8
140008ecc:     	cmpl	$0x5c, %eax
140008ecf:     	jne	0x140008f34 <.text+0x7f34>
140008ed1:     	jmp	0x140008f4c <.text+0x7f4c>
140008ed3:     	nopw	%cs:(%rax,%rax)
140008ee0:     	movzbl	-0x2(%r8), %ecx
140008ee5:     	cmpb	$-0x40, %cl
140008ee8:     	jge	0x140008f0d <.text+0x7f0d>
140008eea:     	movzbl	-0x3(%r8), %edx
140008eef:     	cmpb	$-0x40, %dl
140008ef2:     	jge	0x140008f16 <.text+0x7f16>
140008ef4:     	movzbl	-0x4(%r8), %r9d
140008ef9:     	addq	$-0x4, %r8
140008efd:     	andl	$0x7, %r9d
140008f01:     	shll	$0x6, %r9d
140008f05:     	andl	$0x3f, %edx
140008f08:     	orl	%r9d, %edx
140008f0b:     	jmp	0x140008f1d <.text+0x7f1d>
140008f0d:     	addq	$-0x2, %r8
140008f11:     	andl	$0x1f, %ecx
140008f14:     	jmp	0x140008f25 <.text+0x7f25>
140008f16:     	addq	$-0x3, %r8
140008f1a:     	andl	$0xf, %edx
140008f1d:     	shll	$0x6, %edx
140008f20:     	andl	$0x3f, %ecx
140008f23:     	orl	%edx, %ecx
140008f25:     	shll	$0x6, %ecx
140008f28:     	andb	$0x3f, %al
140008f2a:     	movzbl	%al, %eax
140008f2d:     	orl	%ecx, %eax
140008f2f:     	cmpl	$0x5c, %eax
140008f32:     	je	0x140008f4c <.text+0x7f4c>
140008f34:     	cmpl	$0x2f, %eax
140008f37:     	je	0x140008f4c <.text+0x7f4c>
140008f39:     	cmpq	%r15, %r8
140008f3c:     	jne	0x140008ec0 <.text+0x7ec0>
140008f3e:     	movl	%r13d, %ebx
140008f41:     	movq	%r12, %r13
140008f44:     	movl	$0x40, %r12d
140008f4a:     	jmp	0x140008f9f <.text+0x7f9f>
140008f4c:     	leaq	0x291d515(%rip), %r15   # 0x142926468
140008f53:     	subq	%r15, %r8
140008f56:     	incq	%r8
140008f59:     	je	0x140008f8d <.text+0x7f8d>
140008f5b:     	cmpq	$0x40, %r8
140008f5f:     	jae	0x140008f8b <.text+0x7f8b>
140008f61:     	cmpb	$-0x41, (%r8,%r15)
140008f66:     	jg	0x140008f8d <.text+0x7f8d>
140008f68:     	leaq	0x291e4e9(%rip), %rax   # 0x142927458
140008f6f:     	movq	%rax, 0x20(%rsp)
140008f74:     	leaq	0x291d4ed(%rip), %rcx   # 0x142926468
140008f7b:     	movl	$0x40, %edx
140008f80:     	movl	$0x40, %r9d
140008f86:     	callq	0x1428d9940 <.text+0x28d8940>
140008f8b:     	jne	0x140008f68 <.text+0x7f68>
140008f8d:     	movl	%r13d, %ebx
140008f90:     	movq	%r12, %r13
140008f93:     	movl	$0x40, %r12d
140008f99:     	subq	%r8, %r12
140008f9c:     	addq	%r8, %r15
140008f9f:     	callq	0x140c18130 <.text+0xc17130>
140008fa4:     	movq	%rax, %rcx
140008fa7:     	movq	(%rax), %rax
140008faa:     	cmpq	$0x1, %rax
140008fae:     	jne	0x140010f9e <.text+0xff9e>
140008fb4:     	addq	$0x8, %rcx
140008fb8:     	cmpq	$0x0, (%rcx)
140008fbc:     	jne	0x1400111fb <.text+0x101fb>
140008fc2:     	movq	$-0x1, (%rcx)
140008fc9:     	movq	%rcx, 0x780(%rbp)
140008fd0:     	addq	$0x8, %rcx
140008fd4:     	movq	%r12, 0x20(%rsp)
140008fd9:     	movq	$0x0, 0x30(%rsp)
140008fe2:     	movq	$0x1, 0x28(%rsp)
140008feb:     	movq	%rsi, %rdx
140008fee:     	movq	%r14, %r8
140008ff1:     	movq	%r15, %r9
140008ff4:     	callq	0x140c16d40 <.text+0xc15d40>
140008ff9:     	movq	%rax, 0x270(%rbp)
140009000:     	movq	0x780(%rbp), %rax
140009007:     	incq	(%rax)
14000900a:     	movq	%r13, %r12
14000900d:     	movl	%ebx, %r13d
140009010:     	movq	0x800(%rbp), %r15
140009017:     	leaq	0x29a51a2(%rip), %rdx   # 0x1429ae1c0
14000901e:     	movq	%rdi, %rcx
140009021:     	callq	0x1405ca870 <.text+0x5c9870>
140009026:     	movq	%rax, %rbx
140009029:     	testq	%rax, %rax
14000902c:     	je	0x1400114f7 <.text+0x104f7>
140009032:     	leaq	0x29f3e78(%rip), %rax   # 0x1429fceb1
140009039:     	movq	%rax, 0x6a0(%rbp)
140009040:     	movq	$0x3a, 0x6a8(%rbp)
14000904b:     	movq	%rbx, %r14
14000904e:     	addq	$0x10, %r14
140009052:     	cmpq	$0x3a, 0x18(%rbx)
140009057:     	jne	0x140011134 <.text+0x10134>
14000905d:     	movq	(%r14), %rdx
140009060:     	leaq	0x29f3e4a(%rip), %rcx   # 0x1429fceb1
140009067:     	movl	$0x3a, %r8d
14000906d:     	callq	0x142923bf0 <.text+0x2922bf0>
140009072:     	testl	%eax, %eax
140009074:     	jne	0x140011134 <.text+0x10134>
14000907a:     	movq	(%rbx), %rcx
14000907d:     	movq	0x8(%rbx), %rax
140009081:     	callq	*0x18(%rax)
140009084:     	movq	%rax, 0x120(%rbp)
14000908b:     	cmpq	$0x0, 0x120(%rbp)
140009093:     	je	0x1400114f7 <.text+0x104f7>
140009099:     	leaq	0x29a5118(%rip), %rdx   # 0x1429ae1b8
1400090a0:     	movq	%rdi, %rcx
1400090a3:     	callq	0x1405ca870 <.text+0x5c9870>
1400090a8:     	movq	%rax, %rbx
1400090ab:     	testq	%rax, %rax
1400090ae:     	je	0x140011508 <.text+0x10508>
1400090b4:     	leaq	0x29f3b9a(%rip), %rax   # 0x1429fcc55
1400090bb:     	movq	%rax, 0x6a0(%rbp)
1400090c2:     	movq	$0x3d, 0x6a8(%rbp)
1400090cd:     	movq	%rbx, %r14
1400090d0:     	addq	$0x10, %r14
1400090d4:     	cmpq	$0x3d, 0x18(%rbx)
1400090d9:     	jne	0x140011168 <.text+0x10168>
1400090df:     	movq	(%r14), %rdx
1400090e2:     	leaq	0x29f3b6c(%rip), %rcx   # 0x1429fcc55
1400090e9:     	movl	$0x3d, %r8d
1400090ef:     	callq	0x142923bf0 <.text+0x2922bf0>
1400090f4:     	testl	%eax, %eax
1400090f6:     	jne	0x140011168 <.text+0x10168>
1400090fc:     	movq	(%rbx), %rcx
1400090ff:     	movq	0x8(%rbx), %rax
140009103:     	callq	*0x18(%rax)
140009106:     	movq	%rax, 0x128(%rbp)
14000910d:     	cmpq	$0x0, 0x128(%rbp)
140009115:     	je	0x140011508 <.text+0x10508>
14000911b:     	leaq	0x29a506e(%rip), %rdx   # 0x1429ae190
140009122:     	movq	%rdi, %rcx
140009125:     	callq	0x1405ca870 <.text+0x5c9870>
14000912a:     	movq	%rax, %rdi
14000912d:     	testq	%rax, %rax
140009130:     	je	0x140011519 <.text+0x10519>
140009136:     	leaq	0x29f3aeb(%rip), %rax   # 0x1429fcc28
14000913d:     	movq	%rax, 0x6a0(%rbp)
140009144:     	movq	$0x2d, 0x6a8(%rbp)
14000914f:     	movq	%rdi, %rbx
140009152:     	addq	$0x10, %rbx
140009156:     	cmpq	$0x2d, 0x18(%rdi)
14000915b:     	jne	0x14001119c <.text+0x1019c>
140009161:     	movq	(%rbx), %rdx
140009164:     	leaq	0x29f3abd(%rip), %rcx   # 0x1429fcc28
14000916b:     	movl	$0x2d, %r8d
140009171:     	callq	0x142923bf0 <.text+0x2922bf0>
140009176:     	testl	%eax, %eax
140009178:     	jne	0x14001119c <.text+0x1019c>
14000917e:     	movq	(%rdi), %rcx
140009181:     	movq	0x8(%rdi), %rax
140009185:     	callq	*0x18(%rax)
140009188:     	movq	%rax, 0x130(%rbp)
14000918f:     	cmpq	$0x0, 0x130(%rbp)
140009197:     	je	0x140011519 <.text+0x10519>
14000919d:     	movq	0x948(%rbp), %rax
1400091a4:     	movq	0x940(%rbp), %rcx
1400091ab:     	movl	%r13d, (%r12)
1400091af:     	movq	%rcx, %r13
1400091b2:     	movq	0x7c0(%rbp), %rcx
1400091b9:     	leaq	0x198(%rcx), %rdx
1400091c0:     	movq	%rdx, 0x2e0(%rbp)
1400091c7:     	movq	0x8(%rcx), %rdi
1400091cb:     	imulq	$0x150, 0x10(%rcx), %rdx # imm = 0x150
1400091d3:     	addq	%rdi, %rdx
1400091d6:     	movq	%rdx, 0xf0(%rbp)
1400091dd:     	movl	0x1c(%rax), %edx
1400091e0:     	movl	%edx, 0x23c(%rbp)
1400091e6:     	movq	(%rax), %rdx
1400091e9:     	movq	%rdx, 0x98(%rbp)
1400091f0:     	movq	0x10(%rax), %rax
1400091f4:     	movq	%rax, 0xa0(%rbp)
1400091fb:     	movl	0x1c(%r15), %eax
1400091ff:     	movl	%eax, 0x32c(%rbp)
140009205:     	movq	(%r15), %rdx
140009208:     	movq	0x10(%r15), %rax
14000920c:     	movq	%rax, 0x110(%rbp)
140009213:     	xorps	%xmm8, %xmm8
140009217:     	movq	0x2f0(%rbp), %rax
14000921e:     	movzbl	0x30(%rax), %eax
140009222:     	movb	%al, 0x648(%rbp)
140009228:     	leaq	0x18(%rcx), %rax
14000922c:     	movq	%rax, 0x248(%rbp)
140009233:     	leaq	0x1b8(%rcx), %rax
14000923a:     	movq	%rax, 0xc0(%rbp)
140009241:     	leaq	0x60(%rdx), %rax
140009245:     	movq	%rax, 0xb0(%rbp)
14000924c:     	movq	%rdx, 0xb8(%rbp)
140009253:     	leaq	0x30(%rdx), %rax
140009257:     	movq	%rax, 0xa8(%rbp)
14000925e:     	movq	0x338(%rbp), %rax
140009265:     	addq	$0x20, %rax
140009269:     	movq	%rax, 0xf8(%rbp)
140009270:     	leaq	0x60(%rcx), %rax
140009274:     	movq	%rax, 0x350(%rbp)
14000927b:     	xorl	%r14d, %r14d
14000927e:     	leaq	0x590(%rbp), %rbx
140009285:     	leaq	0x460(%rbp), %r12
14000928c:     	jmp	0x140009330 <.text+0x8330>
140009291:     	nopw	%cs:(%rax,%rax)
1400092a0:     	movdqu	(%rcx,%r8), %xmm1
1400092a6:     	movdqa	%xmm1, %xmm2
1400092aa:     	pcmpeqb	%xmm0, %xmm2
1400092ae:     	pmovmskb	%xmm2, %r9d
1400092b3:     	testl	%r9d, %r9d
1400092b6:     	je	0x140009300 <.text+0x8300>
1400092b8:     	movq	0x678(%rbp), %r10
1400092bf:     	movq	(%r10), %xmm2
1400092c4:     	tzcntl	%r9d, %r10d
1400092c9:     	addq	%r8, %r10
1400092cc:     	andq	%rdx, %r10
1400092cf:     	shlq	$0x3, %r10
1400092d3:     	movq	%rcx, %r11
1400092d6:     	subq	%r10, %r11
1400092d9:     	movq	-0x8(%r11), %xmm3
1400092df:     	pcmpeqd	%xmm2, %xmm3
1400092e3:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1400092e8:     	movmskpd	%xmm3, %r10d
1400092ed:     	cmpl	$0x3, %r10d
1400092f1:     	je	0x140009330 <.text+0x8330>
1400092f3:     	leal	-0x1(%r9), %r10d
1400092f7:     	andw	%r9w, %r10w
1400092fb:     	movl	%r10d, %r9d
1400092fe:     	jne	0x1400092c4 <.text+0x82c4>
140009300:     	pcmpeqb	0x291c688(%rip), %xmm1  # 0x142925990
140009308:     	pmovmskb	%xmm1, %r9d
14000930d:     	testl	%r9d, %r9d
140009310:     	jne	0x1400093b9 <.text+0x83b9>
140009316:     	addq	%rax, %r8
140009319:     	addq	$0x10, %r8
14000931d:     	addq	$0x10, %rax
140009321:     	andq	%rdx, %r8
140009324:     	jmp	0x1400092a0 <.text+0x82a0>
140009329:     	nopl	(%rax)
140009330:     	cmpq	0xf0(%rbp), %rdi
140009337:     	je	0x140010604 <.text+0xf604>
14000933d:     	movq	%rdi, %rax
140009340:     	addq	$0x150, %rdi            # imm = 0x150
140009347:     	cmpb	$0x1, 0x148(%rax)
14000934e:     	jne	0x140009330 <.text+0x8330>
140009350:     	cmpq	0x98(%rax), %r14
140009357:     	jo	0x140009330 <.text+0x8330>
140009359:     	movq	%rax, 0x7e0(%rbp)
140009360:     	leaq	0x128(%rax), %rdx
140009367:     	movq	0x338(%rbp), %rax
14000936e:     	cmpq	$0x0, 0x18(%rax)
140009373:     	movq	%rdx, 0x678(%rbp)
14000937a:     	je	0x1400093b9 <.text+0x83b9>
14000937c:     	movq	0xf8(%rbp), %rcx
140009383:     	callq	0x1405b1be0 <.text+0x5b0be0>
140009388:     	movq	0x338(%rbp), %rdx
14000938f:     	movq	(%rdx), %rcx
140009392:     	movq	0x8(%rdx), %rdx
140009396:     	movq	%rdx, %r8
140009399:     	andq	%rax, %r8
14000939c:     	shrq	$0x39, %rax
1400093a0:     	movd	%eax, %xmm0
1400093a4:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1400093a8:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1400093ad:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1400093b2:     	xorl	%eax, %eax
1400093b4:     	jmp	0x1400092a0 <.text+0x82a0>
1400093b9:     	movq	0x7e0(%rbp), %rax
1400093c0:     	movl	0x128(%rax), %ecx
1400093c6:     	movl	0x12c(%rax), %edx
1400093cc:     	callq	0x140959e70 <.text+0x958e70>
1400093d1:     	movq	%rax, 0x438(%rbp)
1400093d8:     	movq	0x438(%rbp), %rcx
1400093df:     	callq	0x140c96270 <.text+0xc95270>
1400093e4:     	movq	%rax, 0x428(%rbp)
1400093eb:     	movq	0x7e0(%rbp), %rdx
1400093f2:     	cmpq	0x98(%rdx), %r14
1400093f9:     	jo	0x1400114c2 <.text+0x104c2>
1400093ff:     	movq	0xa0(%rdx), %rax
140009406:     	movq	%rax, 0x578(%rbp)
14000940d:     	movq	0xa8(%rdx), %rax
140009414:     	movq	%rax, 0x570(%rbp)
14000941b:     	movq	0x958(%rbp), %rax
140009422:     	movq	(%rax), %rcx
140009425:     	movl	0x128(%rdx), %edx
14000942b:     	callq	0x140a1c930 <.text+0xa1b930>
140009430:     	testq	%rax, %rax
140009433:     	je	0x14000943a <.text+0x843a>
140009435:     	movl	0x28(%rax), %eax
140009438:     	jmp	0x14000943f <.text+0x843f>
14000943a:     	movl	$0x186a3, %eax          # imm = 0x186A3
14000943f:     	movq	%rax, 0x758(%rbp)
140009446:     	movq	0x958(%rbp), %rax
14000944d:     	movq	(%rax), %rcx
140009450:     	movq	0x678(%rbp), %rax
140009457:     	movl	(%rax), %edx
140009459:     	callq	0x140a1c930 <.text+0xa1b930>
14000945e:     	movl	$0x0, 0x528(%rbp)
140009468:     	testq	%rax, %rax
14000946b:     	je	0x140009487 <.text+0x8487>
14000946d:     	addq	$0x30, %rax
140009471:     	movq	%rax, %rcx
140009474:     	callq	0x140839490 <.text+0x838490>
140009479:     	testq	%rax, %rax
14000947c:     	je	0x140009487 <.text+0x8487>
14000947e:     	movl	0x8(%rax), %eax
140009481:     	movl	%eax, 0x528(%rbp)
140009487:     	movq	0x958(%rbp), %rax
14000948e:     	movq	(%rax), %rcx
140009491:     	movq	0x678(%rbp), %rax
140009498:     	movl	(%rax), %edx
14000949a:     	callq	0x140a1c930 <.text+0xa1b930>
14000949f:     	testq	%rax, %rax
1400094a2:     	je	0x1400094dc <.text+0x84dc>
1400094a4:     	addq	$0x30, %rax
1400094a8:     	movq	%rax, %rcx
1400094ab:     	callq	0x140839490 <.text+0x838490>
1400094b0:     	testq	%rax, %rax
1400094b3:     	je	0x1400094dc <.text+0x84dc>
1400094b5:     	movzbl	0x10(%rax), %eax
1400094b9:     	movl	%eax, 0x7b8(%rbp)
1400094bf:     	movl	$0x0, 0x588(%rbp)
1400094c9:     	movq	0x338(%rbp), %rax
1400094d0:     	cmpq	$0x0, 0x18(%rax)
1400094d5:     	jne	0x140009502 <.text+0x8502>
1400094d7:     	jmp	0x1400095d5 <.text+0x85d5>
1400094dc:     	movl	$0x0, 0x7b8(%rbp)
1400094e6:     	movl	$0x0, 0x588(%rbp)
1400094f0:     	movq	0x338(%rbp), %rax
1400094f7:     	cmpq	$0x0, 0x18(%rax)
1400094fc:     	je	0x1400095d5 <.text+0x85d5>
140009502:     	movq	0xf8(%rbp), %rcx
140009509:     	movq	0x678(%rbp), %rdx
140009510:     	callq	0x1405b1be0 <.text+0x5b0be0>
140009515:     	movq	0x338(%rbp), %rdx
14000951c:     	movq	(%rdx), %rcx
14000951f:     	movq	0x8(%rdx), %rdx
140009523:     	movq	%rdx, %r8
140009526:     	andq	%rax, %r8
140009529:     	shrq	$0x39, %rax
14000952d:     	movd	%eax, %xmm0
140009531:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140009535:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14000953a:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14000953f:     	xorl	%eax, %eax
140009541:     	movdqu	(%rcx,%r8), %xmm1
140009547:     	movdqa	%xmm1, %xmm2
14000954b:     	pcmpeqb	%xmm0, %xmm2
14000954f:     	pmovmskb	%xmm2, %r9d
140009554:     	testl	%r9d, %r9d
140009557:     	je	0x1400095b0 <.text+0x85b0>
140009559:     	movq	0x678(%rbp), %r10
140009560:     	movq	(%r10), %xmm2
140009565:     	tzcntl	%r9d, %r10d
14000956a:     	addq	%r8, %r10
14000956d:     	andq	%rdx, %r10
140009570:     	shlq	$0x3, %r10
140009574:     	movq	%rcx, %r11
140009577:     	subq	%r10, %r11
14000957a:     	movq	-0x8(%r11), %xmm3
140009580:     	pcmpeqd	%xmm2, %xmm3
140009584:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
140009589:     	movmskpd	%xmm3, %r10d
14000958e:     	movb	$0x1, %r11b
140009591:     	movl	%r11d, 0x538(%rbp)
140009598:     	cmpl	$0x3, %r10d
14000959c:     	je	0x1400095df <.text+0x85df>
14000959e:     	leal	-0x1(%r9), %r10d
1400095a2:     	andw	%r9w, %r10w
1400095a6:     	movl	%r10d, %r9d
1400095a9:     	jne	0x140009565 <.text+0x8565>
1400095ab:     	nopl	(%rax,%rax)
1400095b0:     	pcmpeqb	0x291c3d8(%rip), %xmm1  # 0x142925990
1400095b8:     	pmovmskb	%xmm1, %r9d
1400095bd:     	testl	%r9d, %r9d
1400095c0:     	jne	0x1400095d5 <.text+0x85d5>
1400095c2:     	addq	%rax, %r8
1400095c5:     	addq	$0x10, %r8
1400095c9:     	addq	$0x10, %rax
1400095cd:     	andq	%rdx, %r8
1400095d0:     	jmp	0x140009541 <.text+0x8541>
1400095d5:     	movl	$0x0, 0x538(%rbp)
1400095df:     	movq	0x7e0(%rbp), %rdx
1400095e6:     	leaq	0xd0(%rdx), %rax
1400095ed:     	movq	%rax, 0x2e8(%rbp)
1400095f4:     	cmpl	$0x1, (%rax)
1400095f7:     	jne	0x140009613 <.text+0x8613>
1400095f9:     	movss	0xd4(%rdx), %xmm0
140009601:     	movss	%xmm0, 0x560(%rbp)
140009609:     	movl	$0x1, 0x588(%rbp)
140009613:     	xorl	%eax, %eax
140009615:     	cmpl	$0x1, 0xf0(%rdx)
14000961c:     	jne	0x140009633 <.text+0x8633>
14000961e:     	movss	0xf4(%rdx), %xmm0
140009626:     	movss	%xmm0, 0x568(%rbp)
14000962e:     	movl	$0x1, %eax
140009633:     	movl	%eax, 0x580(%rbp)
140009639:     	movq	0x90(%rbp), %rax
140009640:     	movl	0x238(%rbp), %ecx
140009646:     	movl	%ecx, (%rax)
140009648:     	movl	0x128(%rdx), %r8d
14000964f:     	movl	0x12c(%rdx), %r9d
140009656:     	movq	0x128(%rbp), %rax
14000965d:     	movq	%rax, 0x38(%rsp)
140009662:     	movq	0x130(%rbp), %rax
140009669:     	movq	%rax, 0x30(%rsp)
14000966e:     	movq	0x120(%rbp), %rax
140009675:     	movq	%rax, 0x28(%rsp)
14000967a:     	movq	%r13, 0x20(%rsp)
14000967f:     	leaq	0x140(%rbp), %rcx
140009686:     	movq	0x2f0(%rbp), %rdx
14000968d:     	callq	0x140aa8a60 <.text+0xaa7a60>
140009692:     	movq	0x7e0(%rbp), %r13
140009699:     	leaq	0x130(%r13), %r15
1400096a0:     	leaq	0xf0(%r13), %rax
1400096a7:     	movq	%rax, 0x118(%rbp)
1400096ae:     	movq	0x570(%rbp), %r9
1400096b5:     	leaq	(%r9,%r9,2), %rax
1400096b9:     	movq	0x578(%rbp), %r10
1400096c0:     	leaq	(%r10,%rax,4), %rax
1400096c4:     	movq	0x58(%r13), %rdx
1400096c8:     	movq	0x60(%r13), %rcx
1400096cc:     	leaq	(%rdx,%rcx,4), %r8
1400096d0:     	cmpq	%r9, %rcx
1400096d3:     	cmovaeq	%r9, %rcx
1400096d7:     	movq	%r10, 0x6a0(%rbp)
1400096de:     	movq	%rax, 0x6a8(%rbp)
1400096e5:     	movq	%rdx, 0x6b0(%rbp)
1400096ec:     	movq	%r8, 0x6b8(%rbp)
1400096f3:     	movq	%rcx, 0x6c8(%rbp)
1400096fa:     	movq	%r9, 0x6d0(%rbp)
140009701:     	movq	$0x0, 0x6d8(%rbp)
14000970c:     	xorl	%eax, %eax
14000970e:     	jmp	0x14000971e <.text+0x871e>
140009710:     	movq	0x6c0(%rbp), %rax
140009717:     	movq	0x6c8(%rbp), %rcx
14000971e:     	cmpq	%rcx, %rax
140009721:     	jae	0x1400099f3 <.text+0x89f3>
140009727:     	leaq	0x1(%rax), %rcx
14000972b:     	movq	%rcx, 0x6c0(%rbp)
140009732:     	leaq	(%rax,%rax,2), %rcx
140009736:     	shlq	$0x2, %rcx
14000973a:     	addq	0x6a0(%rbp), %rcx
140009741:     	shlq	$0x2, %rax
140009745:     	addq	0x6b0(%rbp), %rax
14000974c:     	cmpq	$0x0, 0x6d8(%rbp)
140009754:     	je	0x140009790 <.text+0x8790>
140009756:     	movq	0x6e8(%rbp), %rdx
14000975d:     	movq	0x6f0(%rbp), %r8
140009764:     	movq	%rcx, 0x6e8(%rbp)
14000976b:     	movq	%rax, 0x6f0(%rbp)
140009772:     	movq	%rdx, 0x6d8(%rbp)
140009779:     	movq	%r8, 0x6e0(%rbp)
140009780:     	jmp	0x140009805 <.text+0x8805>
140009785:     	nopw	%cs:(%rax,%rax)
140009790:     	movq	$0x1, 0x460(%rbp)
14000979b:     	movq	%rcx, 0x468(%rbp)
1400097a2:     	movq	%rax, 0x470(%rbp)
1400097a9:     	leaq	0x6a0(%rbp), %rax
1400097b0:     	movq	%rax, 0x478(%rbp)
1400097b7:     	movq	%rbx, %rcx
1400097ba:     	movq	%r12, %rdx
1400097bd:     	callq	0x1402e13b0 <.text+0x2e03b0>
1400097c2:     	movups	0x590(%rbp), %xmm0
1400097c9:     	movdqu	0x5a0(%rbp), %xmm1
1400097d1:     	leaq	0x6d8(%rbp), %rax
1400097d8:     	movdqu	%xmm1, 0x10(%rax)
1400097dd:     	movups	%xmm0, (%rax)
1400097e0:     	movq	0x6d8(%rbp), %rdx
1400097e7:     	testq	%rdx, %rdx
1400097ea:     	je	0x1400099f3 <.text+0x89f3>
1400097f0:     	movq	0x6e0(%rbp), %r8
1400097f7:     	movq	0x6e8(%rbp), %rcx
1400097fe:     	movq	0x6f0(%rbp), %rax
140009805:     	movd	(%rdx), %xmm0
140009809:     	movss	0x8(%rdx), %xmm1
14000980e:     	movd	(%rcx), %xmm2
140009812:     	movss	0x8(%rcx), %xmm3
140009817:     	movss	(%r8), %xmm4
14000981c:     	movss	(%rax), %xmm5
140009820:     	movaps	%xmm4, %xmm6
140009823:     	cmpunordss	%xmm4, %xmm6
140009828:     	movaps	%xmm6, %xmm7
14000982b:     	andps	%xmm5, %xmm7
14000982e:     	maxss	%xmm4, %xmm5
140009832:     	andnps	%xmm5, %xmm6
140009835:     	orps	%xmm7, %xmm6
140009838:     	mulss	0x291c030(%rip), %xmm6  # 0x142925870
140009840:     	movq	0x1f8(%rbp), %rax
140009847:     	movq	%rax, 0x460(%rbp)
14000984e:     	mulss	0x291c01e(%rip), %xmm6  # 0x142925874
140009856:     	addss	0x291c01a(%rip), %xmm6  # 0x142925878
14000985e:     	movd	%xmm0, 0x590(%rbp)
140009866:     	movss	%xmm1, 0x594(%rbp)
14000986e:     	movd	%xmm2, 0x598(%rbp)
140009876:     	movss	%xmm3, 0x59c(%rbp)
14000987e:     	xorl	%eax, %eax
140009880:     	xorl	%esi, %esi
140009882:     	nopw	%cs:(%rax,%rax)
140009890:     	cmpq	$0x2, %rsi
140009894:     	je	0x140009710 <.text+0x8710>
14000989a:     	leaq	0x1(%rsi), %rcx
14000989e:     	movd	0x590(%rbp,%rsi,8), %xmm0
1400098a7:     	movd	0x594(%rbp,%rsi,8), %xmm2
1400098b0:     	testb	$0x1, %al
1400098b2:     	je	0x1400098d0 <.text+0x88d0>
1400098b4:     	movq	%rcx, %rsi
1400098b7:     	movdqa	%xmm0, %xmm12
1400098bc:     	movdqa	%xmm11, %xmm13
1400098c1:     	movdqa	%xmm1, %xmm0
1400098c5:     	movdqa	%xmm2, %xmm11
1400098ca:     	jmp	0x140009903 <.text+0x8903>
1400098cc:     	nopl	(%rax)
1400098d0:     	cmpq	$0x2, %rcx
1400098d4:     	je	0x140009710 <.text+0x8710>
1400098da:     	movq	$0x2, 0x5a0(%rbp)
1400098e5:     	movd	0x598(%rbp,%rsi,8), %xmm12
1400098ef:     	movd	0x59c(%rbp,%rsi,8), %xmm11
1400098f9:     	movl	$0x2, %esi
1400098fe:     	movdqa	%xmm2, %xmm13
140009903:     	movb	$0x0, 0x380(%rbp)
14000990a:     	movss	0x291bf62(%rip), %xmm7  # 0x142925874
140009912:     	movaps	%xmm7, %xmm1
140009915:     	movd	0x291bf5e(%rip), %xmm8  # 0x14292587c
14000991e:     	movdqa	%xmm8, %xmm2
140009923:     	callq	0x142923ef0 <.text+0x2922ef0>
140009928:     	movdqa	%xmm0, %xmm14
14000992d:     	movdqa	%xmm13, %xmm0
140009932:     	movaps	%xmm7, %xmm1
140009935:     	movdqa	%xmm8, %xmm2
14000993a:     	callq	0x142923ef0 <.text+0x2922ef0>
14000993f:     	movd	%xmm14, %ecx
140009944:     	movd	%xmm0, %eax
140009948:     	shlq	$0x20, %rax
14000994c:     	orq	%rax, %rcx
14000994f:     	callq	0x140b84e40 <.text+0xb83e40>
140009954:     	movdqa	%xmm0, %xmm13
140009959:     	movaps	%xmm1, %xmm14
14000995d:     	movdqa	%xmm12, %xmm0
140009962:     	movss	0x291bf0a(%rip), %xmm7  # 0x142925874
14000996a:     	movaps	%xmm7, %xmm1
14000996d:     	movss	0x291bf06(%rip), %xmm8  # 0x14292587c
140009976:     	movaps	%xmm8, %xmm2
14000997a:     	callq	0x142923ef0 <.text+0x2922ef0>
14000997f:     	movdqa	%xmm0, %xmm15
140009984:     	movdqa	%xmm11, %xmm0
140009989:     	movaps	%xmm7, %xmm1
14000998c:     	movaps	%xmm8, %xmm2
140009990:     	callq	0x142923ef0 <.text+0x2922ef0>
140009995:     	movd	%xmm15, %ecx
14000999a:     	xorps	%xmm8, %xmm8
14000999e:     	movd	%xmm0, %eax
1400099a2:     	shlq	$0x20, %rax
1400099a6:     	orq	%rax, %rcx
1400099a9:     	callq	0x140b84e40 <.text+0xb83e40>
1400099ae:     	movdqa	%xmm0, %xmm2
1400099b2:     	movaps	%xmm1, %xmm3
1400099b5:     	leaq	0x380(%rbp), %rax
1400099bc:     	movq	%rax, 0x30(%rsp)
1400099c1:     	movq	%r12, 0x28(%rsp)
1400099c6:     	movss	%xmm6, 0x20(%rsp)
1400099cc:     	movdqa	%xmm13, %xmm0
1400099d1:     	movaps	%xmm14, %xmm1
1400099d5:     	callq	0x1402582d0 <.text+0x2572d0>
1400099da:     	movb	$0x1, %al
1400099dc:     	cmpb	$0x1, 0x380(%rbp)
1400099e3:     	movdqa	%xmm12, %xmm1
1400099e8:     	jne	0x140009890 <.text+0x8890>
1400099ee:     	jmp	0x140009710 <.text+0x8710>
1400099f3:     	movl	0x23c(%rbp), %eax
1400099f9:     	movq	0xa0(%rbp), %rcx
140009a00:     	movl	%eax, (%rcx)
140009a02:     	movl	$0x1, %edx
140009a07:     	movq	0x98(%rbp), %rcx
140009a0e:     	movq	0x438(%rbp), %r8
140009a15:     	callq	0x1400cce50 <.text+0xcbe50>
140009a1a:     	movq	%rax, 0x648(%rbp)
140009a21:     	movq	0x648(%rbp), %rax
140009a28:     	movq	$0x0, 0x50(%rax)
140009a30:     	movq	0x758(%rbp), %rax
140009a37:     	addl	$0xfffe7960, %eax       # imm = 0xFFFE7960
140009a3c:     	cmpl	$0xb, %eax
140009a3f:     	ja	0x140009a70 <.text+0x8a70>
140009a41:     	leaq	0x291de38(%rip), %rcx   # 0x142927880
140009a48:     	movslq	(%rcx,%rax,4), %rax
140009a4c:     	addq	%rcx, %rax
140009a4f:     	jmpq	*%rax
140009a51:     	movsd	0x291be67(%rip), %xmm0  # 0x1429258c0
140009a59:     	movsd	%xmm0, 0x590(%rbp)
140009a61:     	movl	$0x3e2e147b, 0x598(%rbp) # imm = 0x3E2E147B
140009a6b:     	jmp	0x140009af5 <.text+0x8af5>
140009a70:     	movl	$0x0, 0x598(%rbp)
140009a7a:     	movq	$0x0, 0x590(%rbp)
140009a85:     	jmp	0x140009af5 <.text+0x8af5>
140009a87:     	movsd	0x291be11(%rip), %xmm0  # 0x1429258a0
140009a8f:     	movsd	%xmm0, 0x590(%rbp)
140009a97:     	movl	$0x3e051eb8, 0x598(%rbp) # imm = 0x3E051EB8
140009aa1:     	jmp	0x140009af5 <.text+0x8af5>
140009aa3:     	movsd	0x291be05(%rip), %xmm0  # 0x1429258b0
140009aab:     	movsd	%xmm0, 0x590(%rbp)
140009ab3:     	movl	$0x3e0f5c29, 0x598(%rbp) # imm = 0x3E0F5C29
140009abd:     	jmp	0x140009af5 <.text+0x8af5>
140009abf:     	movsd	0x291bdc9(%rip), %xmm0  # 0x142925890
140009ac7:     	movsd	%xmm0, 0x590(%rbp)
140009acf:     	movl	$0x3e570a3d, 0x598(%rbp) # imm = 0x3E570A3D
140009ad9:     	jmp	0x140009af5 <.text+0x8af5>
140009adb:     	movsd	0x291bd9d(%rip), %xmm0  # 0x142925880
140009ae3:     	movsd	%xmm0, 0x590(%rbp)
140009aeb:     	movl	$0x3e2147ae, 0x598(%rbp) # imm = 0x3E2147AE
140009af5:     	leaq	0x6a0(%rbp), %rcx
140009afc:     	movq	%rbx, %rdx
140009aff:     	callq	0x1408bc460 <.text+0x8bb460>
140009b04:     	testb	$0x1, 0x6a0(%rbp)
140009b0b:     	je	0x140009b2a <.text+0x8b2a>
140009b0d:     	movq	0x6a8(%rbp), %rax
140009b14:     	movq	%rax, 0x460(%rbp)
140009b1b:     	movq	%r12, %rcx
140009b1e:     	callq	0x1400885d0 <.text+0x875d0>
140009b23:     	orl	$0x1000000, %eax        # imm = 0x1000000
140009b28:     	jmp	0x140009b30 <.text+0x8b30>
140009b2a:     	movl	0x6a4(%rbp), %eax
140009b30:     	movq	0x648(%rbp), %rcx
140009b37:     	movl	%eax, 0x58(%rcx)
140009b3a:     	movq	$0x0, 0x440(%rbp)
140009b45:     	leaq	0x450(%rbp), %rax
140009b4c:     	movups	%xmm8, (%rax)
140009b50:     	movq	$0x8, 0x448(%rbp)
140009b5b:     	movq	$0x0, 0x278(%rbp)
140009b66:     	movq	$0x10, 0x280(%rbp)
140009b71:     	movq	$0x0, 0x288(%rbp)
140009b7c:     	movq	0x140(%r13), %rax
140009b83:     	cmpq	$0x2, %rax
140009b87:     	jb	0x140009b97 <.text+0x8b97>
140009b89:     	movq	0x130(%r13), %r15
140009b90:     	movq	0x138(%r13), %rax
140009b97:     	addq	$0x40, 0x648(%rbp)
140009b9f:     	leaq	(%r15,%rax,4), %rax
140009ba3:     	movq	%rax, 0x520(%rbp)
140009baa:     	movq	0x570(%rbp), %rax
140009bb1:     	decq	%rax
140009bb4:     	movq	%rax, 0x668(%rbp)
140009bbb:     	movq	%r15, %rax
140009bbe:     	xorl	%r15d, %r15d
140009bc1:     	movq	%rdi, 0x88(%rbp)
140009bc8:     	jmp	0x140009be5 <.text+0x8be5>
140009bca:     	nopw	(%rax,%rax)
140009bd0:     	movq	0x7e0(%rbp), %r13
140009bd7:     	movq	0x778(%rbp), %rax
140009bde:     	movq	0x7b0(%rbp), %r15
140009be5:     	cmpq	0x520(%rbp), %rax
140009bec:     	je	0x14000df2d <.text+0xcf2d>
140009bf2:     	movzwl	0x2(%rax), %r8d
140009bf7:     	leaq	(%r15,%r8), %r11
140009bfb:     	testq	%r15, %r15
140009bfe:     	sete	%sil
140009c02:     	cmpq	0x668(%rbp), %r11
140009c09:     	sete	%bl
140009c0c:     	cmpq	$-0x1, %r11
140009c10:     	je	0x140010f19 <.text+0xff19>
140009c16:     	leaq	0x1(%r11), %rcx
140009c1a:     	movq	%rcx, %rdi
140009c1d:     	subq	%r15, %rdi
140009c20:     	jb	0x140010f31 <.text+0xff31>
140009c26:     	cmpq	0x570(%rbp), %r11
140009c2d:     	jae	0x140010f4f <.text+0xff4f>
140009c33:     	movq	0x78(%r13), %rdx
140009c37:     	cmpq	%rdx, %r11
140009c3a:     	jae	0x140010f6e <.text+0xff6e>
140009c40:     	movq	0x60(%r13), %rdx
140009c44:     	cmpq	%rdx, %r11
140009c47:     	jae	0x140010f86 <.text+0xff86>
140009c4d:     	leaq	0x4(%rax), %rdx
140009c51:     	movq	%rdx, 0x778(%rbp)
140009c58:     	leaq	(%r15,%r15,2), %rdx
140009c5c:     	movq	0x578(%rbp), %r9
140009c63:     	leaq	(%r9,%rdx,4), %r12
140009c67:     	leaq	(,%r15,8), %rdx
140009c6f:     	addq	0x70(%r13), %rdx
140009c73:     	movq	%rdx, 0x800(%rbp)
140009c7a:     	leaq	(,%r15,4), %rdx
140009c82:     	addq	0x58(%r13), %rdx
140009c86:     	movq	%rdx, 0x780(%rbp)
140009c8d:     	movq	0x90(%r13), %rdx
140009c94:     	testq	%rdx, %rdx
140009c97:     	movq	%r11, 0x7b0(%rbp)
140009c9e:     	movb	%bl, 0x7a0(%rbp)
140009ca4:     	movb	%sil, 0x7a8(%rbp)
140009cab:     	je	0x140009d20 <.text+0x8d20>
140009cad:     	cmpq	%rdx, %r11
140009cb0:     	pxor	%xmm1, %xmm1
140009cb4:     	movq	0x678(%rbp), %r10
140009cbb:     	jae	0x140010fe9 <.text+0xffe9>
140009cc1:     	movq	0x88(%r13), %r9
140009cc8:     	movzbl	(%rax), %edx
140009ccb:     	movzbl	0x1(%rax), %eax
140009ccf:     	movb	%dl, 0x80f(%rbp)
140009cd5:     	movb	%al, 0x810(%rbp)
140009cdb:     	cmpq	%r15, %rcx
140009cde:     	je	0x140009d3a <.text+0x8d3a>
140009ce0:     	movzbl	%dl, %eax
140009ce3:     	leaq	(%r9,%r15,4), %rcx
140009ce7:     	leaq	0x4(,%r8,4), %r8
140009cef:     	xorl	%r9d, %r9d
140009cf2:     	nopw	%cs:(%rax,%rax)
140009d00:     	cmpq	%r9, %r8
140009d03:     	je	0x14000a2b4 <.text+0x92b4>
140009d09:     	movss	(%rcx,%r9), %xmm0
140009d0f:     	addq	$0x4, %r9
140009d13:     	ucomiss	%xmm1, %xmm0
140009d16:     	ja	0x140009d00 <.text+0x8d00>
140009d18:     	jmp	0x140009d3a <.text+0x8d3a>
140009d1a:     	nopw	(%rax,%rax)
140009d20:     	movzbl	(%rax), %edx
140009d23:     	movzbl	0x1(%rax), %eax
140009d27:     	movb	%dl, 0x80f(%rbp)
140009d2d:     	movb	%al, 0x810(%rbp)
140009d33:     	movq	0x678(%rbp), %r10
140009d3a:     	leaq	0x80f(%rbp), %rax
140009d41:     	movq	%rax, 0x2a0(%rbp)
140009d48:     	movq	0x958(%rbp), %rax
140009d4f:     	movq	%rax, 0x2a8(%rbp)
140009d56:     	movq	%r10, 0x2b0(%rbp)
140009d5d:     	movq	0x248(%rbp), %rax
140009d64:     	movq	%rax, 0x2b8(%rbp)
140009d6b:     	cmpb	$0x3, %dl
140009d6e:     	je	0x14000a79b <.text+0x979b>
140009d74:     	movq	%r15, 0x670(%rbp)
140009d7b:     	movq	0x200(%rbp), %rax
140009d82:     	movb	$0x1, 0x817(%rbp)
140009d89:     	movq	%rax, 0x48(%rsp)
140009d8e:     	movq	0x648(%rbp), %rax
140009d95:     	movq	%rax, 0x40(%rsp)
140009d9a:     	leaq	0x278(%rbp), %rax
140009da1:     	movq	%rax, 0x38(%rsp)
140009da6:     	movq	%rdi, 0x28(%rsp)
140009dab:     	movq	0x780(%rbp), %rax
140009db2:     	movq	%rax, 0x20(%rsp)
140009db7:     	movq	$0x1, 0x30(%rsp)
140009dc0:     	movq	%r12, %r14
140009dc3:     	movq	%r12, %rcx
140009dc6:     	movq	%rdi, %rdx
140009dc9:     	movq	0x800(%rbp), %r8
140009dd0:     	movq	%rdi, %r9
140009dd3:     	callq	0x1407de8b0 <.text+0x7dd8b0>
140009dd8:     	movq	%rdi, %rax
140009ddb:     	movl	$0xc, %ecx
140009de0:     	mulq	%rcx
140009de3:     	movq	%rax, %r12
140009de6:     	jo	0x140011358 <.text+0x10358>
140009dec:     	movl	$0x0, %ebx
140009df1:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140009dfb:     	cmpq	%rax, %r12
140009dfe:     	ja	0x14001135a <.text+0x1035a>
140009e04:     	movzbl	0x80f(%rbp), %eax
140009e0b:     	movb	%al, 0x760(%rbp)
140009e11:     	testq	%r12, %r12
140009e14:     	je	0x140009e40 <.text+0x8e40>
140009e16:     	movzbl	0x362adec(%rip), %eax   # 0x143634c09
140009e1d:     	movl	$0x4, %ebx
140009e22:     	movl	$0x4, %edx
140009e27:     	movq	%r12, %rcx
140009e2a:     	callq	0x140613c10 <.text+0x612c10>
140009e2f:     	testq	%rax, %rax
140009e32:     	je	0x14001135a <.text+0x1035a>
140009e38:     	movq	%rax, %rsi
140009e3b:     	movq	%rdi, %rbx
140009e3e:     	jmp	0x140009e47 <.text+0x8e47>
140009e40:     	movl	$0x4, %esi
140009e45:     	xorl	%ebx, %ebx
140009e47:     	movq	%rsi, %rcx
140009e4a:     	movq	%r14, %rdx
140009e4d:     	movq	%r12, %r8
140009e50:     	callq	0x142923c00 <.text+0x2922c00>
140009e55:     	movq	%rbx, 0x590(%rbp)
140009e5c:     	movq	%rsi, 0x598(%rbp)
140009e63:     	movq	%rdi, 0x5a0(%rbp)
140009e6a:     	movb	$0x1, 0x817(%rbp)
140009e71:     	leaq	0x6a0(%rbp), %rcx
140009e78:     	leaq	0x590(%rbp), %rdx
140009e7f:     	movl	$0x1, %r8d
140009e85:     	callq	0x140153640 <.text+0x152640>
140009e8a:     	xorl	%r14d, %r14d
140009e8d:     	cmpq	0x6a0(%rbp), %r14
140009e94:     	jo	0x140009bd0 <.text+0x8bd0>
140009e9a:     	movq	0x6d0(%rbp), %rax
140009ea1:     	movq	%rax, 0x3b0(%rbp)
140009ea8:     	movups	0x6a0(%rbp), %xmm0
140009eaf:     	movups	0x6b0(%rbp), %xmm1
140009eb6:     	movdqu	0x6c0(%rbp), %xmm2
140009ebe:     	movdqa	%xmm2, 0x3a0(%rbp)
140009ec6:     	movaps	%xmm1, 0x390(%rbp)
140009ecd:     	movaps	%xmm0, 0x380(%rbp)
140009ed4:     	movq	0x390(%rbp), %rbx
140009edb:     	movq	%rbx, %rax
140009ede:     	movl	$0xc, %ecx
140009ee3:     	mulq	%rcx
140009ee6:     	movq	%rax, %r12
140009ee9:     	jo	0x1400113ae <.text+0x103ae>
140009eef:     	movl	$0x0, %esi
140009ef4:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140009efe:     	cmpq	%rax, %r12
140009f01:     	ja	0x1400113b0 <.text+0x103b0>
140009f07:     	movq	0x388(%rbp), %r14
140009f0e:     	testq	%r12, %r12
140009f11:     	je	0x140009f3e <.text+0x8f3e>
140009f13:     	movzbl	0x362acef(%rip), %eax   # 0x143634c09
140009f1a:     	movl	$0x4, %esi
140009f1f:     	movl	$0x4, %edx
140009f24:     	movq	%r12, %rcx
140009f27:     	callq	0x140613c10 <.text+0x612c10>
140009f2c:     	testq	%rax, %rax
140009f2f:     	je	0x1400113b0 <.text+0x103b0>
140009f35:     	movq	%rbx, 0x7f0(%rbp)
140009f3c:     	jmp	0x140009f4e <.text+0x8f4e>
140009f3e:     	movl	$0x4, %eax
140009f43:     	movq	$0x0, 0x7f0(%rbp)
140009f4e:     	movq	%rax, 0x798(%rbp)
140009f55:     	movq	%rax, %rcx
140009f58:     	movq	%r14, %rdx
140009f5b:     	movq	%r12, %r8
140009f5e:     	callq	0x142923c00 <.text+0x2922c00>
140009f63:     	movq	0x3a8(%rbp), %r15
140009f6a:     	leaq	(,%r15,4), %r12
140009f72:     	movq	%r15, %rax
140009f75:     	shrq	$0x3e, %rax
140009f79:     	jne	0x1400113c7 <.text+0x103c7>
140009f7f:     	movl	$0x0, %r14d
140009f85:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140009f8f:     	cmpq	%rax, %r12
140009f92:     	ja	0x1400113ca <.text+0x103ca>
140009f98:     	movq	0x3a0(%rbp), %r13
140009f9f:     	testq	%r12, %r12
140009fa2:     	je	0x140009fcf <.text+0x8fcf>
140009fa4:     	movzbl	0x362ac5e(%rip), %eax   # 0x143634c09
140009fab:     	movl	$0x4, %r14d
140009fb1:     	movl	$0x4, %edx
140009fb6:     	movq	%r12, %rcx
140009fb9:     	callq	0x140613c10 <.text+0x612c10>
140009fbe:     	testq	%rax, %rax
140009fc1:     	je	0x1400113ca <.text+0x103ca>
140009fc7:     	movq	%rax, %rsi
140009fca:     	movq	%r15, %r14
140009fcd:     	jmp	0x140009fd7 <.text+0x8fd7>
140009fcf:     	movl	$0x4, %esi
140009fd4:     	xorl	%r14d, %r14d
140009fd7:     	movq	%rsi, %rcx
140009fda:     	movq	%r13, %rdx
140009fdd:     	movq	%r12, %r8
140009fe0:     	callq	0x142923c00 <.text+0x2922c00>
140009fe5:     	movss	0x3b0(%rbp), %xmm0
140009fed:     	movq	0x7f0(%rbp), %rax
140009ff4:     	movq	%rax, 0x6a0(%rbp)
140009ffb:     	movq	0x798(%rbp), %rax
14000a002:     	movq	%rax, 0x6a8(%rbp)
14000a009:     	movq	%rbx, 0x6b0(%rbp)
14000a010:     	movq	%r14, 0x6b8(%rbp)
14000a017:     	movq	%rsi, 0x6c0(%rbp)
14000a01e:     	movq	%r15, 0x6c8(%rbp)
14000a025:     	movss	%xmm0, 0x6d0(%rbp)
14000a02d:     	leaq	0x590(%rbp), %rcx
14000a034:     	leaq	0x6a0(%rbp), %rdx
14000a03b:     	movd	0x291b82d(%rip), %xmm2  # 0x142925870
14000a043:     	movl	$0x1, %r9d
14000a049:     	callq	0x140152b00 <.text+0x151b00>
14000a04e:     	movq	0x590(%rbp), %rax
14000a055:     	movq	%rax, %rcx
14000a058:     	negq	%rcx
14000a05b:     	movl	$0x0, %r14d
14000a061:     	movq	0x7e0(%rbp), %r13
14000a068:     	jno	0x14000a110 <.text+0x9110>
14000a06e:     	movq	0x6a0(%rbp), %rax
14000a075:     	testq	%rax, %rax
14000a078:     	je	0x14000a094 <.text+0x9094>
14000a07a:     	movq	0x6a8(%rbp), %rcx
14000a081:     	shlq	$0x2, %rax
14000a085:     	leaq	(%rax,%rax,2), %rdx
14000a089:     	movl	$0x4, %r8d
14000a08f:     	callq	0x140613c20 <.text+0x612c20>
14000a094:     	movq	0x6b8(%rbp), %rdx
14000a09b:     	testq	%rdx, %rdx
14000a09e:     	je	0x14000a0b6 <.text+0x90b6>
14000a0a0:     	movq	0x6c0(%rbp), %rcx
14000a0a7:     	shlq	$0x2, %rdx
14000a0ab:     	movl	$0x4, %r8d
14000a0b1:     	callq	0x140613c20 <.text+0x612c20>
14000a0b6:     	movq	0x380(%rbp), %rax
14000a0bd:     	testq	%rax, %rax
14000a0c0:     	je	0x14000a0dc <.text+0x90dc>
14000a0c2:     	movq	0x388(%rbp), %rcx
14000a0c9:     	shlq	$0x2, %rax
14000a0cd:     	leaq	(%rax,%rax,2), %rdx
14000a0d1:     	movl	$0x4, %r8d
14000a0d7:     	callq	0x140613c20 <.text+0x612c20>
14000a0dc:     	movq	0x398(%rbp), %rdx
14000a0e3:     	testq	%rdx, %rdx
14000a0e6:     	je	0x140009bd7 <.text+0x8bd7>
14000a0ec:     	movq	0x3a0(%rbp), %rcx
14000a0f3:     	shlq	$0x2, %rdx
14000a0f7:     	movl	$0x4, %r8d
14000a0fd:     	callq	0x140613c20 <.text+0x612c20>
14000a102:     	jmp	0x140009bd7 <.text+0x8bd7>
14000a107:     	nopw	(%rax,%rax)
14000a110:     	leaq	0x598(%rbp), %rcx
14000a117:     	movups	(%rcx), %xmm0
14000a11a:     	movups	0x10(%rcx), %xmm1
14000a11e:     	movdqu	0x20(%rcx), %xmm2
14000a123:     	leaq	0x468(%rbp), %rcx
14000a12a:     	movdqu	%xmm2, 0x20(%rcx)
14000a12f:     	movups	%xmm1, 0x10(%rcx)
14000a133:     	movups	%xmm0, (%rcx)
14000a136:     	movq	%rax, 0x460(%rbp)
14000a13d:     	movq	0x6a0(%rbp), %rax
14000a144:     	testq	%rax, %rax
14000a147:     	je	0x14000a163 <.text+0x9163>
14000a149:     	movq	0x6a8(%rbp), %rcx
14000a150:     	shlq	$0x2, %rax
14000a154:     	leaq	(%rax,%rax,2), %rdx
14000a158:     	movl	$0x4, %r8d
14000a15e:     	callq	0x140613c20 <.text+0x612c20>
14000a163:     	movq	0x6b8(%rbp), %rdx
14000a16a:     	testq	%rdx, %rdx
14000a16d:     	je	0x14000a185 <.text+0x9185>
14000a16f:     	movq	0x6c0(%rbp), %rcx
14000a176:     	shlq	$0x2, %rdx
14000a17a:     	movl	$0x4, %r8d
14000a180:     	callq	0x140613c20 <.text+0x612c20>
14000a185:     	movq	0x470(%rbp), %r14
14000a18c:     	testq	%r14, %r14
14000a18f:     	je	0x14000a30d <.text+0x930d>
14000a195:     	leaq	(,%r14,4), %rsi
14000a19d:     	movzbl	0x362aa65(%rip), %eax   # 0x143634c09
14000a1a4:     	movl	$0x4, %edx
14000a1a9:     	movq	%rsi, %rcx
14000a1ac:     	callq	0x140613c10 <.text+0x612c10>
14000a1b1:     	testq	%rax, %rax
14000a1b4:     	je	0x1400113e1 <.text+0x103e1>
14000a1ba:     	movq	0x470(%rbp), %rbx
14000a1c1:     	movq	%r14, 0x650(%rbp)
14000a1c8:     	movq	%rax, 0x658(%rbp)
14000a1cf:     	movq	$0x0, 0x660(%rbp)
14000a1da:     	testq	%rbx, %rbx
14000a1dd:     	je	0x14000a32e <.text+0x932e>
14000a1e3:     	leaq	(,%rbx,8), %rsi
14000a1eb:     	movzbl	0x362aa17(%rip), %eax   # 0x143634c09
14000a1f2:     	movl	$0x4, %edx
14000a1f7:     	movq	%rsi, %rcx
14000a1fa:     	callq	0x140613c10 <.text+0x612c10>
14000a1ff:     	testq	%rax, %rax
14000a202:     	je	0x14001145c <.text+0x1045c>
14000a208:     	movq	0x470(%rbp), %r14
14000a20f:     	movq	%rbx, 0x680(%rbp)
14000a216:     	movq	%rax, 0x688(%rbp)
14000a21d:     	movq	$0x0, 0x690(%rbp)
14000a228:     	testq	%r14, %r14
14000a22b:     	je	0x14000a34f <.text+0x934f>
14000a231:     	leaq	(,%r14,4), %rax
14000a239:     	leaq	(%rax,%rax,2), %rsi
14000a23d:     	movzbl	0x362a9c5(%rip), %eax   # 0x143634c09
14000a244:     	movl	$0x4, %edx
14000a249:     	movq	%rsi, %rcx
14000a24c:     	callq	0x140613c10 <.text+0x612c10>
14000a251:     	testq	%rax, %rax
14000a254:     	je	0x140011490 <.text+0x10490>
14000a25a:     	movq	0x470(%rbp), %rbx
14000a261:     	movq	%r14, 0x500(%rbp)
14000a268:     	movq	%rax, 0x508(%rbp)
14000a26f:     	movq	$0x0, 0x510(%rbp)
14000a27a:     	testq	%rbx, %rbx
14000a27d:     	je	0x14000a370 <.text+0x9370>
14000a283:     	leaq	(,%rbx,4), %rsi
14000a28b:     	movzbl	0x362a977(%rip), %eax   # 0x143634c09
14000a292:     	movl	$0x4, %edx
14000a297:     	movq	%rsi, %rcx
14000a29a:     	callq	0x140613c10 <.text+0x612c10>
14000a29f:     	testq	%rax, %rax
14000a2a2:     	je	0x1400114a9 <.text+0x104a9>
14000a2a8:     	movq	0x470(%rbp), %rsi
14000a2af:     	jmp	0x14000a379 <.text+0x9379>
14000a2b4:     	leal	-0x2(%rdx), %ecx
14000a2b7:     	decq	%rax
14000a2ba:     	cmpb	$0x3, %cl
14000a2bd:     	cmovaeq	%r14, %rax
14000a2c1:     	testq	%rax, %rax
14000a2c4:     	je	0x14000a2d0 <.text+0x92d0>
14000a2c6:     	cmpq	$0x3, %rax
14000a2ca:     	jne	0x140009d3a <.text+0x8d3a>
14000a2d0:     	movb	$0x2, 0x80f(%rbp)
14000a2d7:     	leaq	0x80f(%rbp), %rax
14000a2de:     	movq	%rax, 0x2a0(%rbp)
14000a2e5:     	movq	0x958(%rbp), %rax
14000a2ec:     	movq	%rax, 0x2a8(%rbp)
14000a2f3:     	movq	%r10, 0x2b0(%rbp)
14000a2fa:     	movq	0x248(%rbp), %rax
14000a301:     	movq	%rax, 0x2b8(%rbp)
14000a308:     	jmp	0x140009d74 <.text+0x8d74>
14000a30d:     	movq	$0x0, 0x650(%rbp)
14000a318:     	movq	$0x4, 0x658(%rbp)
14000a323:     	movq	$0x0, 0x660(%rbp)
14000a32e:     	movq	$0x0, 0x680(%rbp)
14000a339:     	movq	$0x4, 0x688(%rbp)
14000a344:     	movq	$0x0, 0x690(%rbp)
14000a34f:     	movq	$0x0, 0x500(%rbp)
14000a35a:     	movq	$0x4, 0x508(%rbp)
14000a365:     	movq	$0x0, 0x510(%rbp)
14000a370:     	movl	$0x4, %eax
14000a375:     	xorl	%ebx, %ebx
14000a377:     	xorl	%esi, %esi
14000a379:     	movq	%rbx, 0x590(%rbp)
14000a380:     	movq	%rax, 0x598(%rbp)
14000a387:     	movq	$0x0, 0x5a0(%rbp)
14000a392:     	movq	0x468(%rbp), %r14
14000a399:     	movq	0x480(%rbp), %rax
14000a3a0:     	movq	%rax, 0x7f0(%rbp)
14000a3a7:     	movq	0x488(%rbp), %rax
14000a3ae:     	cmpq	%rsi, %rax
14000a3b1:     	cmovbq	%rax, %rsi
14000a3b5:     	xorl	%ebx, %ebx
14000a3b7:     	xorl	%r13d, %r13d
14000a3ba:     	jmp	0x14000a3db <.text+0x93db>
14000a3bc:     	nopl	(%rax)
14000a3c0:     	incq	%r13
14000a3c3:     	movq	0x598(%rbp), %rax
14000a3ca:     	movl	$0x0, (%rax,%rbx,4)
14000a3d1:     	incq	%rbx
14000a3d4:     	movq	%rbx, 0x5a0(%rbp)
14000a3db:     	cmpq	%rsi, %r13
14000a3de:     	jae	0x14000a5d0 <.text+0x95d0>
14000a3e4:     	leaq	(,%r13,2), %rax
14000a3ec:     	addq	%r13, %rax
14000a3ef:     	movl	(%r14,%rax,4), %ebx
14000a3f3:     	movss	0x4(%r14,%rax,4), %xmm6
14000a3fa:     	movl	0x8(%r14,%rax,4), %r15d
14000a3ff:     	movq	0x7f0(%rbp), %rax
14000a406:     	movss	(%rax,%r13,4), %xmm1
14000a40c:     	leaq	0x380(%rbp), %rcx
14000a413:     	callq	0x140153380 <.text+0x152380>
14000a418:     	movl	%eax, %ecx
14000a41a:     	cmpq	%rcx, %rdi
14000a41d:     	jbe	0x1400111e7 <.text+0x101e7>
14000a423:     	leaq	0x1(%rcx), %rax
14000a427:     	cmpq	%rdi, %rax
14000a42a:     	jae	0x1400111d0 <.text+0x101d0>
14000a430:     	movaps	%xmm0, %xmm11
14000a434:     	movq	0x800(%rbp), %rax
14000a43b:     	movsd	(%rax,%rcx,8), %xmm7
14000a440:     	movsd	0x8(%rax,%rcx,8), %xmm8
14000a447:     	movq	0x780(%rbp), %rax
14000a44e:     	movss	(%rax,%rcx,4), %xmm10
14000a454:     	movss	0x4(%rax,%rcx,4), %xmm9
14000a45b:     	cmpb	$0x2, 0x760(%rbp)
14000a462:     	jae	0x14000a4c9 <.text+0x94c9>
14000a464:     	movd	%ebx, %xmm1
14000a468:     	movd	%r15d, %xmm2
14000a46d:     	movq	0x240(%rbp), %rdx
14000a474:     	movq	0x8(%rdx), %rax
14000a478:     	movq	0x10(%rdx), %rcx
14000a47c:     	addq	%rcx, %rcx
14000a47f:     	movq	%rax, 0x6a0(%rbp)
14000a486:     	movq	%rcx, 0x6a8(%rbp)
14000a48d:     	movsd	0x50(%rdx), %xmm0
14000a492:     	movsd	%xmm0, 0x6b0(%rbp)
14000a49a:     	leaq	0x6a0(%rbp), %rcx
14000a4a1:     	callq	0x140257ba0 <.text+0x256ba0>
14000a4a6:     	addss	0x291b472(%rip), %xmm0  # 0x142925920
14000a4ae:     	movaps	%xmm6, %xmm1
14000a4b1:     	cmpunordss	%xmm6, %xmm1
14000a4b6:     	movaps	%xmm1, %xmm2
14000a4b9:     	andps	%xmm0, %xmm2
14000a4bc:     	maxss	%xmm6, %xmm0
14000a4c0:     	andnps	%xmm0, %xmm1
14000a4c3:     	orps	%xmm2, %xmm1
14000a4c6:     	movaps	%xmm1, %xmm6
14000a4c9:     	movq	0x660(%rbp), %r12
14000a4d0:     	cmpq	0x650(%rbp), %r12
14000a4d7:     	jne	0x14000a4ec <.text+0x94ec>
14000a4d9:     	leaq	0x650(%rbp), %rcx
14000a4e0:     	leaq	0x291bec1(%rip), %rdx   # 0x1429263a8
14000a4e7:     	callq	0x140650050 <.text+0x64f050>
14000a4ec:     	subss	%xmm10, %xmm9
14000a4f1:     	mulss	%xmm11, %xmm9
14000a4f6:     	addss	%xmm10, %xmm9
14000a4fb:     	movq	0x658(%rbp), %rax
14000a502:     	movss	%xmm9, (%rax,%r12,4)
14000a508:     	incq	%r12
14000a50b:     	movq	%r12, 0x660(%rbp)
14000a512:     	movq	0x690(%rbp), %r12
14000a519:     	cmpq	0x680(%rbp), %r12
14000a520:     	jne	0x14000a535 <.text+0x9535>
14000a522:     	leaq	0x680(%rbp), %rcx
14000a529:     	leaq	0x291be90(%rip), %rdx   # 0x1429263c0
14000a530:     	callq	0x140b81f90 <.text+0xb80f90>
14000a535:     	subps	%xmm7, %xmm8
14000a539:     	movsldup	%xmm11, %xmm0           # xmm0 = xmm11[0,0,2,2]
14000a53e:     	mulps	%xmm8, %xmm0
14000a542:     	addps	%xmm7, %xmm0
14000a545:     	movq	0x688(%rbp), %rax
14000a54c:     	movlps	%xmm0, (%rax,%r12,8)
14000a551:     	incq	%r12
14000a554:     	movq	%r12, 0x690(%rbp)
14000a55b:     	movq	0x510(%rbp), %r12
14000a562:     	cmpq	0x500(%rbp), %r12
14000a569:     	jne	0x14000a57e <.text+0x957e>
14000a56b:     	leaq	0x500(%rbp), %rcx
14000a572:     	leaq	0x291be5f(%rip), %rdx   # 0x1429263d8
14000a579:     	callq	0x140b9f180 <.text+0xb9e180>
14000a57e:     	movq	0x508(%rbp), %rax
14000a585:     	leaq	(%r12,%r12,2), %rcx
14000a589:     	movl	%ebx, (%rax,%rcx,4)
14000a58c:     	movss	%xmm6, 0x4(%rax,%rcx,4)
14000a592:     	movl	%r15d, 0x8(%rax,%rcx,4)
14000a597:     	incq	%r12
14000a59a:     	movq	%r12, 0x510(%rbp)
14000a5a1:     	movq	0x5a0(%rbp), %rbx
14000a5a8:     	cmpq	0x590(%rbp), %rbx
14000a5af:     	jne	0x14000a3c0 <.text+0x93c0>
14000a5b5:     	leaq	0x590(%rbp), %rcx
14000a5bc:     	leaq	0x291be2d(%rip), %rdx   # 0x1429263f0
14000a5c3:     	callq	0x140650050 <.text+0x64f050>
14000a5c8:     	jmp	0x14000a3c0 <.text+0x93c0>
14000a5cd:     	nopl	(%rax)
14000a5d0:     	movq	0x650(%rbp), %rsi
14000a5d7:     	leaq	0x658(%rbp), %rax
14000a5de:     	movups	(%rax), %xmm0
14000a5e1:     	movaps	%xmm0, 0x6a0(%rbp)
14000a5e8:     	movq	0x690(%rbp), %rax
14000a5ef:     	leaq	0x6d8(%rbp), %rcx
14000a5f6:     	movq	%rax, -0x18(%rcx)
14000a5fa:     	movups	0x680(%rbp), %xmm0
14000a601:     	movups	%xmm0, -0x28(%rcx)
14000a605:     	movups	0x500(%rbp), %xmm0
14000a60c:     	movaps	%xmm0, 0x310(%rbp)
14000a613:     	movq	0x510(%rbp), %rax
14000a61a:     	movq	%rax, 0x320(%rbp)
14000a621:     	movq	0x590(%rbp), %rdi
14000a628:     	movq	0x598(%rbp), %r15
14000a62f:     	movaps	0x6a0(%rbp), %xmm0
14000a636:     	movaps	0x6b0(%rbp), %xmm1
14000a63d:     	movaps	%xmm0, 0x3e0(%rbp)
14000a644:     	movaps	%xmm1, 0x3f0(%rbp)
14000a64b:     	movq	0x6c0(%rbp), %rax
14000a652:     	movq	%rax, 0x400(%rbp)
14000a659:     	movq	0x460(%rbp), %rax
14000a660:     	testq	%rax, %rax
14000a663:     	je	0x14000a67f <.text+0x967f>
14000a665:     	movq	0x468(%rbp), %rcx
14000a66c:     	shlq	$0x2, %rax
14000a670:     	leaq	(%rax,%rax,2), %rdx
14000a674:     	movl	$0x4, %r8d
14000a67a:     	callq	0x140613c20 <.text+0x612c20>
14000a67f:     	movq	0x478(%rbp), %rdx
14000a686:     	testq	%rdx, %rdx
14000a689:     	movl	$0x0, %r14d
14000a68f:     	movq	0x7e0(%rbp), %r13
14000a696:     	je	0x14000a6ae <.text+0x96ae>
14000a698:     	movq	0x480(%rbp), %rcx
14000a69f:     	shlq	$0x2, %rdx
14000a6a3:     	movl	$0x4, %r8d
14000a6a9:     	callq	0x140613c20 <.text+0x612c20>
14000a6ae:     	movq	0x380(%rbp), %rax
14000a6b5:     	testq	%rax, %rax
14000a6b8:     	je	0x14000a6d4 <.text+0x96d4>
14000a6ba:     	movq	0x388(%rbp), %rcx
14000a6c1:     	shlq	$0x2, %rax
14000a6c5:     	leaq	(%rax,%rax,2), %rdx
14000a6c9:     	movl	$0x4, %r8d
14000a6cf:     	callq	0x140613c20 <.text+0x612c20>
14000a6d4:     	movq	0x398(%rbp), %rdx
14000a6db:     	testq	%rdx, %rdx
14000a6de:     	je	0x14000a6f6 <.text+0x96f6>
14000a6e0:     	movq	0x3a0(%rbp), %rcx
14000a6e7:     	shlq	$0x2, %rdx
14000a6eb:     	movl	$0x4, %r8d
14000a6f1:     	callq	0x140613c20 <.text+0x612c20>
14000a6f6:     	movq	%rsi, %rax
14000a6f9:     	negq	%rax
14000a6fc:     	jo	0x140009bd7 <.text+0x8bd7>
14000a702:     	movaps	0x3e0(%rbp), %xmm0
14000a709:     	leaq	0x550(%rbp), %rax
14000a710:     	movups	%xmm0, (%rax)
14000a713:     	leaq	0x3f0(%rbp), %rcx
14000a71a:     	movq	0x10(%rcx), %rax
14000a71e:     	movq	%rax, 0x660(%rbp)
14000a725:     	movups	(%rcx), %xmm0
14000a728:     	movaps	%xmm0, 0x650(%rbp)
14000a72f:     	movq	0x320(%rbp), %rax
14000a736:     	movq	%rax, 0x690(%rbp)
14000a73d:     	movaps	0x310(%rbp), %xmm0
14000a744:     	movaps	%xmm0, 0x680(%rbp)
14000a74b:     	movq	%rsi, 0x548(%rbp)
14000a752:     	movzbl	0x80f(%rbp), %esi
14000a759:     	leal	-0x2(%rsi), %eax
14000a75c:     	movq	%rsi, %rcx
14000a75f:     	decq	%rcx
14000a762:     	cmpb	$0x3, %al
14000a764:     	cmovaeq	%r14, %rcx
14000a768:     	leaq	0x291d141(%rip), %rdx   # 0x1429278b0
14000a76f:     	movslq	(%rdx,%rcx,4), %rax
14000a773:     	addq	%rdx, %rax
14000a776:     	movq	%rdi, 0x798(%rbp)
14000a77d:     	movq	%r15, 0x7f0(%rbp)
14000a784:     	jmpq	*%rax
14000a786:     	cmpb	$0x4, %sil
14000a78a:     	jne	0x14000add8 <.text+0x9dd8>
14000a790:     	xorl	%r15d, %r15d
14000a793:     	xorl	%r12d, %r12d
14000a796:     	jmp	0x14000b1d6 <.text+0xa1d6>
14000a79b:     	movq	%rdi, 0x78(%rbp)
14000a79f:     	cmpq	$0x2, %rdi
14000a7a3:     	jne	0x1400110f8 <.text+0x100f8>
14000a7a9:     	movsd	(%r12), %xmm14
14000a7af:     	movss	0x8(%r12), %xmm15
14000a7b6:     	movsd	0xc(%r12), %xmm12
14000a7bd:     	movss	0x14(%r12), %xmm13
14000a7c4:     	movq	0x950(%rbp), %rax
14000a7cb:     	movq	(%rax), %rax
14000a7ce:     	movq	0x7e0(%rbp), %rdx
14000a7d5:     	movl	0x128(%rdx), %ecx
14000a7db:     	movl	0x12c(%rdx), %edx
14000a7e1:     	movsd	%xmm14, 0x590(%rbp)
14000a7ea:     	movss	%xmm15, 0x598(%rbp)
14000a7f3:     	movsd	%xmm12, 0x6a0(%rbp)
14000a7fc:     	movss	%xmm13, 0x6a8(%rbp)
14000a805:     	movb	$0x1, 0x817(%rbp)
14000a80c:     	movl	%edx, 0x38(%rsp)
14000a810:     	movl	%ecx, 0x30(%rsp)
14000a814:     	movq	0x2e0(%rbp), %rcx
14000a81b:     	movq	%rcx, 0x28(%rsp)
14000a820:     	movq	%rax, 0x20(%rsp)
14000a825:     	leaq	0x650(%rbp), %rcx
14000a82c:     	movq	0x2e8(%rbp), %rdx
14000a833:     	leaq	0x590(%rbp), %r8
14000a83a:     	leaq	0x6a0(%rbp), %r9
14000a841:     	callq	0x1407d8fe0 <.text+0x7d7fe0>
14000a846:     	movq	0x7e0(%rbp), %r10
14000a84d:     	movl	0x128(%r10), %r8d
14000a854:     	movl	0x12c(%r10), %r9d
14000a85b:     	movl	%r8d, 0x590(%rbp)
14000a862:     	movl	%r9d, 0x594(%rbp)
14000a869:     	movq	0x7c0(%rbp), %rax
14000a870:     	movq	0x70(%rax), %rdi
14000a874:     	testq	%rdi, %rdi
14000a877:     	leaq	0x590(%rbp), %rsi
14000a87e:     	je	0x14000a98c <.text+0x998c>
14000a884:     	cmpq	$0x1, %rdi
14000a888:     	jne	0x14000a8ad <.text+0x98ad>
14000a88a:     	movq	0x7c0(%rbp), %rax
14000a891:     	movq	0x68(%rax), %rax
14000a895:     	cmpl	%r8d, 0x8(%rax)
14000a899:     	jne	0x14000a98c <.text+0x998c>
14000a89f:     	cmpl	%r9d, 0xc(%rax)
14000a8a3:     	jne	0x14000a98c <.text+0x998c>
14000a8a9:     	xorl	%edx, %edx
14000a8ab:     	jmp	0x14000a905 <.text+0x9905>
14000a8ad:     	movq	0x7c0(%rbp), %rax
14000a8b4:     	movq	0x98(%rax), %rcx
14000a8bb:     	movq	0xa0(%rax), %rdx
14000a8c2:     	callq	0x140106840 <.text+0x105840>
14000a8c7:     	movb	$0x1, 0x817(%rbp)
14000a8ce:     	movq	0x350(%rbp), %rcx
14000a8d5:     	movq	%rax, %rdx
14000a8d8:     	movq	%rsi, %r8
14000a8db:     	callq	0x1403ab1c0 <.text+0x3aa1c0>
14000a8e0:     	cmpq	$0x1, %rax
14000a8e4:     	movq	0x7e0(%rbp), %r10
14000a8eb:     	jne	0x14000a98c <.text+0x998c>
14000a8f1:     	cmpq	%rdi, %rdx
14000a8f4:     	jae	0x140011604 <.text+0x10604>
14000a8fa:     	movq	0x7c0(%rbp), %rax
14000a901:     	movq	0x68(%rax), %rax
14000a905:     	leaq	0x460(%rbp), %rcx
14000a90c:     	leaq	(%rdx,%rdx,4), %rdx
14000a910:     	movups	0x10(%rax,%rdx,8), %xmm0
14000a915:     	movaps	%xmm0, 0x460(%rbp)
14000a91c:     	movl	0x20(%rax,%rdx,8), %eax
14000a920:     	movl	%eax, 0x470(%rbp)
14000a926:     	movzbl	0x460(%rbp), %edx
14000a92d:     	movb	$0x2, 0x35c(%rbp)
14000a934:     	movl	0x36c(%rbp), %eax
14000a93a:     	movl	%eax, 0x5a0(%rbp)
14000a940:     	movzbl	0x35c(%rbp), %eax
14000a947:     	movb	%al, 0x590(%rbp)
14000a94d:     	movq	0x35d(%rbp), %rax
14000a954:     	movq	%rax, 0x591(%rbp)
14000a95b:     	movl	0x365(%rbp), %eax
14000a961:     	movl	%eax, 0x599(%rbp)
14000a967:     	movzwl	0x369(%rbp), %eax
14000a96e:     	movw	%ax, 0x59d(%rbp)
14000a975:     	movzbl	0x36b(%rbp), %eax
14000a97c:     	movb	%al, 0x59f(%rbp)
14000a982:     	movq	%rcx, %rax
14000a985:     	cmpb	$0x3, %dl
14000a988:     	je	0x14000a9ef <.text+0x99ef>
14000a98a:     	jmp	0x14000a9f2 <.text+0x99f2>
14000a98c:     	movb	$0x3, 0x460(%rbp)
14000a993:     	movb	$0x2, 0x35c(%rbp)
14000a99a:     	movl	0x36c(%rbp), %eax
14000a9a0:     	movl	%eax, 0x5a0(%rbp)
14000a9a6:     	movzbl	0x35c(%rbp), %eax
14000a9ad:     	movb	%al, 0x590(%rbp)
14000a9b3:     	movq	0x35d(%rbp), %rax
14000a9ba:     	movq	%rax, 0x591(%rbp)
14000a9c1:     	movl	0x365(%rbp), %eax
14000a9c7:     	movl	%eax, 0x599(%rbp)
14000a9cd:     	movzwl	0x369(%rbp), %eax
14000a9d4:     	movw	%ax, 0x59d(%rbp)
14000a9db:     	movzbl	0x36b(%rbp), %eax
14000a9e2:     	movb	%al, 0x59f(%rbp)
14000a9e8:     	leaq	0x460(%rbp), %rcx
14000a9ef:     	movq	%rsi, %rax
14000a9f2:     	movl	0x10(%rax), %edx
14000a9f5:     	movl	%edx, 0x390(%rbp)
14000a9fb:     	movups	(%rax), %xmm0
14000a9fe:     	movaps	%xmm0, 0x380(%rbp)
14000aa05:     	movl	0x128(%r10), %r9d
14000aa0c:     	movl	0x12c(%r10), %r8d
14000aa13:     	movl	%r8d, 0x590(%rbp)
14000aa1a:     	movl	%r9d, 0x594(%rbp)
14000aa21:     	movq	0x7c0(%rbp), %rax
14000aa28:     	movq	0x70(%rax), %rdi
14000aa2c:     	testq	%rdi, %rdi
14000aa2f:     	je	0x14000aae3 <.text+0x9ae3>
14000aa35:     	cmpq	$0x1, %rdi
14000aa39:     	jne	0x14000aa5e <.text+0x9a5e>
14000aa3b:     	movq	0x7c0(%rbp), %rax
14000aa42:     	movq	0x68(%rax), %rax
14000aa46:     	cmpl	%r8d, 0x8(%rax)
14000aa4a:     	jne	0x14000aae3 <.text+0x9ae3>
14000aa50:     	cmpl	%r9d, 0xc(%rax)
14000aa54:     	jne	0x14000aae3 <.text+0x9ae3>
14000aa5a:     	xorl	%edx, %edx
14000aa5c:     	jmp	0x14000aab2 <.text+0x9ab2>
14000aa5e:     	movq	0x7c0(%rbp), %rax
14000aa65:     	movq	0x98(%rax), %rcx
14000aa6c:     	movq	0xa0(%rax), %rdx
14000aa73:     	callq	0x140106840 <.text+0x105840>
14000aa78:     	movb	$0x1, 0x817(%rbp)
14000aa7f:     	movq	0x350(%rbp), %rcx
14000aa86:     	movq	%rax, %rdx
14000aa89:     	movq	%rsi, %r8
14000aa8c:     	callq	0x1403ab1c0 <.text+0x3aa1c0>
14000aa91:     	cmpq	$0x1, %rax
14000aa95:     	leaq	0x460(%rbp), %rcx
14000aa9c:     	jne	0x14000aae3 <.text+0x9ae3>
14000aa9e:     	cmpq	%rdi, %rdx
14000aaa1:     	jae	0x1400115e6 <.text+0x105e6>
14000aaa7:     	movq	0x7c0(%rbp), %rax
14000aaae:     	movq	0x68(%rax), %rax
14000aab2:     	leaq	(%rdx,%rdx,4), %rdx
14000aab6:     	movups	0x10(%rax,%rdx,8), %xmm0
14000aabb:     	movaps	%xmm0, 0x590(%rbp)
14000aac2:     	movl	0x20(%rax,%rdx,8), %eax
14000aac6:     	cmpb	$0x3, 0x590(%rbp)
14000aacd:     	movl	%eax, 0x5a0(%rbp)
14000aad3:     	movq	%rsi, %rax
14000aad6:     	leaq	0x35c(%rbp), %rdx
14000aadd:     	cmoveq	%rdx, %rax
14000aae1:     	jmp	0x14000aaf1 <.text+0x9af1>
14000aae3:     	movb	$0x3, 0x590(%rbp)
14000aaea:     	leaq	0x35c(%rbp), %rax
14000aaf1:     	movl	0x10(%rax), %r8d
14000aaf5:     	leaq	0x6d8(%rbp), %rdx
14000aafc:     	movl	%r8d, -0x14(%rdx)
14000ab00:     	movups	(%rax), %xmm0
14000ab03:     	movups	%xmm0, -0x24(%rdx)
14000ab07:     	movaps	0x380(%rbp), %xmm0
14000ab0e:     	movaps	%xmm0, 0x6a0(%rbp)
14000ab15:     	movl	0x390(%rbp), %eax
14000ab1b:     	movl	%eax, 0x6b0(%rbp)
14000ab21:     	movlps	%xmm14, 0x460(%rbp)
14000ab29:     	movss	%xmm15, 0x468(%rbp)
14000ab32:     	movlps	%xmm12, 0x590(%rbp)
14000ab3a:     	movss	%xmm13, 0x598(%rbp)
14000ab43:     	movb	$0x1, 0x817(%rbp)
14000ab4a:     	movq	%rsi, %rdx
14000ab4d:     	movq	0x2e8(%rbp), %r8
14000ab54:     	leaq	0x6a0(%rbp), %r9
14000ab5b:     	callq	0x1407de800 <.text+0x7dd800>
14000ab60:     	movb	%al, 0x7ef(%rbp)
14000ab66:     	movq	0x800(%rbp), %rax
14000ab6d:     	movsd	(%rax), %xmm6
14000ab71:     	movsd	0x8(%rax), %xmm11
14000ab77:     	movups	(%rax), %xmm0
14000ab7a:     	movaps	%xmm0, 0x310(%rbp)
14000ab81:     	movq	0x780(%rbp), %rax
14000ab88:     	movsd	(%rax), %xmm0
14000ab8c:     	movsd	%xmm0, 0x540(%rbp)
14000ab94:     	cmpb	$0x0, 0x538(%rbp)
14000ab9b:     	je	0x14000d04f <.text+0xc04f>
14000aba1:     	movb	$0x0, 0x7ef(%rbp)
14000aba8:     	movzbl	0x362a05a(%rip), %eax   # 0x143634c09
14000abaf:     	movl	$0x18, %ecx
14000abb4:     	movl	$0x4, %edx
14000abb9:     	callq	0x140613c10 <.text+0x612c10>
14000abbe:     	testq	%rax, %rax
14000abc1:     	je	0x1400115ac <.text+0x105ac>
14000abc7:     	movq	0x660(%rbp), %rcx
14000abce:     	movq	%rcx, 0x10(%rax)
14000abd2:     	movups	0x650(%rbp), %xmm0
14000abd9:     	movups	%xmm0, (%rax)
14000abdc:     	movq	$0x2, 0x6a0(%rbp)
14000abe7:     	movq	%rax, 0x6a8(%rbp)
14000abee:     	movq	$0x2, 0x6b0(%rbp)
14000abf9:     	movb	$0x1, 0x817(%rbp)
14000ac00:     	leaq	0x590(%rbp), %rcx
14000ac07:     	leaq	0x6a0(%rbp), %rdx
14000ac0e:     	xorl	%r8d, %r8d
14000ac11:     	callq	0x140153640 <.text+0x152640>
14000ac16:     	xorl	%esi, %esi
14000ac18:     	cmpq	0x590(%rbp), %rsi
14000ac1f:     	jo	0x14000d04f <.text+0xc04f>
14000ac25:     	movq	0x5c0(%rbp), %rax
14000ac2c:     	movq	%rax, 0x6d0(%rbp)
14000ac33:     	movups	0x590(%rbp), %xmm0
14000ac3a:     	movups	0x5a0(%rbp), %xmm1
14000ac41:     	movups	0x5b0(%rbp), %xmm2
14000ac48:     	movaps	%xmm2, 0x6c0(%rbp)
14000ac4f:     	movaps	%xmm1, 0x6b0(%rbp)
14000ac56:     	movaps	%xmm0, 0x6a0(%rbp)
14000ac5d:     	leaq	0x460(%rbp), %rcx
14000ac64:     	leaq	0x6a0(%rbp), %rdx
14000ac6b:     	movd	0x291abfd(%rip), %xmm2  # 0x142925870
14000ac73:     	movl	$0x1, %r9d
14000ac79:     	callq	0x140152b00 <.text+0x151b00>
14000ac7e:     	movq	0x6a0(%rbp), %rax
14000ac85:     	testq	%rax, %rax
14000ac88:     	je	0x14000aca4 <.text+0x9ca4>
14000ac8a:     	movq	0x6a8(%rbp), %rcx
14000ac91:     	shlq	$0x2, %rax
14000ac95:     	leaq	(%rax,%rax,2), %rdx
14000ac99:     	movl	$0x4, %r8d
14000ac9f:     	callq	0x140613c20 <.text+0x612c20>
14000aca4:     	movq	0x6b8(%rbp), %rdx
14000acab:     	testq	%rdx, %rdx
14000acae:     	je	0x14000acc6 <.text+0x9cc6>
14000acb0:     	movq	0x6c0(%rbp), %rcx
14000acb7:     	shlq	$0x2, %rdx
14000acbb:     	movl	$0x4, %r8d
14000acc1:     	callq	0x140613c20 <.text+0x612c20>
14000acc6:     	cmpq	0x460(%rbp), %rsi
14000accd:     	jo	0x14000d04f <.text+0xc04f>
14000acd3:     	movups	0x460(%rbp), %xmm0
14000acda:     	movups	0x470(%rbp), %xmm1
14000ace1:     	movups	0x480(%rbp), %xmm2
14000ace8:     	movaps	%xmm1, 0x5a0(%rbp)
14000acef:     	movaps	%xmm2, 0x5b0(%rbp)
14000acf6:     	movq	0x490(%rbp), %rax
14000acfd:     	movq	%rax, 0x5c0(%rbp)
14000ad04:     	movaps	%xmm0, 0x590(%rbp)
14000ad0b:     	movq	0x5a0(%rbp), %rbx
14000ad12:     	movq	0x5b8(%rbp), %rax
14000ad19:     	cmpq	%rbx, %rax
14000ad1c:     	cmovbq	%rax, %rbx
14000ad20:     	leaq	(,%rbx,8), %rdi
14000ad28:     	movq	%rbx, %rax
14000ad2b:     	shrq	$0x3d, %rax
14000ad2f:     	movq	0x348(%rbp), %r13
14000ad36:     	xorps	%xmm9, %xmm9
14000ad3a:     	jne	0x140011763 <.text+0x10763>
14000ad40:     	movl	$0x0, %esi
14000ad45:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14000ad4f:     	cmpq	%rax, %rdi
14000ad52:     	movss	0x291aba9(%rip), %xmm8  # 0x142925904
14000ad5b:     	movaps	0x291abcd(%rip), %xmm10 # 0x142925930
14000ad63:     	ja	0x140011765 <.text+0x10765>
14000ad69:     	movq	0x5b0(%rbp), %r14
14000ad70:     	testq	%rdi, %rdi
14000ad73:     	je	0x14000ae0c <.text+0x9e0c>
14000ad79:     	movzbl	0x3629e89(%rip), %eax   # 0x143634c09
14000ad80:     	movl	$0x4, %esi
14000ad85:     	movl	$0x4, %edx
14000ad8a:     	movq	%rdi, %rcx
14000ad8d:     	callq	0x140613c10 <.text+0x612c10>
14000ad92:     	testq	%rax, %rax
14000ad95:     	je	0x140011765 <.text+0x10765>
14000ad9b:     	movq	%rax, %r12
14000ad9e:     	movq	%rbx, 0x800(%rbp)
14000ada5:     	testq	%rbx, %rbx
14000ada8:     	jne	0x14000ae26 <.text+0x9e26>
14000adaa:     	jmp	0x14000aef3 <.text+0x9ef3>
14000adaf:     	cmpb	$0x0, 0x7b8(%rbp)
14000adb6:     	movq	0x670(%rbp), %rax
14000adbd:     	je	0x14000b942 <.text+0xa942>
14000adc3:     	cmpb	$0x4, %sil
14000adc7:     	jne	0x14000b55b <.text+0xa55b>
14000adcd:     	xorl	%r15d, %r15d
14000add0:     	xorl	%r12d, %r12d
14000add3:     	jmp	0x14000b852 <.text+0xa852>
14000add8:     	cmpq	$0x0, 0x670(%rbp)
14000ade0:     	je	0x14000b1ad <.text+0xa1ad>
14000ade6:     	xorl	%eax, %eax
14000ade8:     	movq	0x7b0(%rbp), %rcx
14000adef:     	cmpq	0x668(%rbp), %rcx
14000adf6:     	jne	0x14000b1cc <.text+0xa1cc>
14000adfc:     	cmpb	$0x1, 0x108(%r13)
14000ae04:     	sete	%cl
14000ae07:     	jmp	0x14000b1ce <.text+0xa1ce>
14000ae0c:     	movl	$0x4, %r12d
14000ae12:     	movq	$0x0, 0x800(%rbp)
14000ae1d:     	testq	%rbx, %rbx
14000ae20:     	je	0x14000aef3 <.text+0x9ef3>
14000ae26:     	cmpq	$0x8, %rbx
14000ae2a:     	jae	0x14000afe4 <.text+0x9fe4>
14000ae30:     	xorl	%eax, %eax
14000ae32:     	movq	%rax, %rcx
14000ae35:     	orq	$0x1, %rcx
14000ae39:     	testb	$0x1, %bl
14000ae3c:     	je	0x14000ae75 <.text+0x9e75>
14000ae3e:     	movss	(%r14,%rax,4), %xmm0
14000ae44:     	movaps	%xmm8, %xmm1
14000ae48:     	subss	%xmm0, %xmm1
14000ae4c:     	movsd	0x310(%rbp), %xmm2
14000ae54:     	movsd	0x318(%rbp), %xmm3
14000ae5c:     	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
14000ae60:     	mulps	%xmm2, %xmm1
14000ae63:     	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
14000ae67:     	mulps	%xmm3, %xmm0
14000ae6a:     	addps	%xmm1, %xmm0
14000ae6d:     	movlps	%xmm0, (%r12,%rax,8)
14000ae72:     	movq	%rcx, %rax
14000ae75:     	cmpq	%rcx, %rbx
14000ae78:     	je	0x14000aef3 <.text+0x9ef3>
14000ae7a:     	nopw	(%rax,%rax)
14000ae80:     	movss	(%r14,%rax,4), %xmm0
14000ae86:     	movaps	%xmm8, %xmm1
14000ae8a:     	subss	%xmm0, %xmm1
14000ae8e:     	movsd	0x310(%rbp), %xmm2
14000ae96:     	movsd	0x318(%rbp), %xmm3
14000ae9e:     	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
14000aea2:     	mulps	%xmm2, %xmm1
14000aea5:     	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
14000aea9:     	mulps	%xmm3, %xmm0
14000aeac:     	addps	%xmm1, %xmm0
14000aeaf:     	movlps	%xmm0, (%r12,%rax,8)
14000aeb4:     	movss	0x4(%r14,%rax,4), %xmm0
14000aebb:     	movaps	%xmm8, %xmm1
14000aebf:     	subss	%xmm0, %xmm1
14000aec3:     	movsd	0x310(%rbp), %xmm2
14000aecb:     	movsd	0x318(%rbp), %xmm3
14000aed3:     	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
14000aed7:     	mulps	%xmm2, %xmm1
14000aeda:     	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
14000aede:     	mulps	%xmm3, %xmm0
14000aee1:     	addps	%xmm1, %xmm0
14000aee4:     	movlps	%xmm0, 0x8(%r12,%rax,8)
14000aeea:     	addq	$0x2, %rax
14000aeee:     	cmpq	%rax, %rbx
14000aef1:     	jne	0x14000ae80 <.text+0x9e80>
14000aef3:     	movq	0x5a0(%rbp), %r14
14000aefa:     	movq	0x5b8(%rbp), %rax
14000af01:     	cmpq	%r14, %rax
14000af04:     	cmovbq	%rax, %r14
14000af08:     	leaq	(,%r14,4), %rdi
14000af10:     	movq	%r14, %rax
14000af13:     	shrq	$0x3e, %rax
14000af17:     	movq	%r12, 0x780(%rbp)
14000af1e:     	jne	0x1400117f1 <.text+0x107f1>
14000af24:     	movl	$0x0, %esi
14000af29:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14000af33:     	cmpq	%rax, %rdi
14000af36:     	ja	0x1400117f3 <.text+0x107f3>
14000af3c:     	movq	0x5b0(%rbp), %r15
14000af43:     	testq	%rdi, %rdi
14000af46:     	je	0x14000af7e <.text+0x9f7e>
14000af48:     	movzbl	0x3629cba(%rip), %eax   # 0x143634c09
14000af4f:     	movl	$0x4, %esi
14000af54:     	movl	$0x4, %edx
14000af59:     	movq	%rdi, %rcx
14000af5c:     	callq	0x140613c10 <.text+0x612c10>
14000af61:     	testq	%rax, %rax
14000af64:     	je	0x1400117f3 <.text+0x107f3>
14000af6a:     	movq	%rax, %r10
14000af6d:     	movq	%r14, 0x760(%rbp)
14000af74:     	testq	%r14, %r14
14000af77:     	jne	0x14000af98 <.text+0x9f98>
14000af79:     	jmp	0x14000cd9a <.text+0xbd9a>
14000af7e:     	movl	$0x4, %r10d
14000af84:     	movq	$0x0, 0x760(%rbp)
14000af8f:     	testq	%r14, %r14
14000af92:     	je	0x14000cd9a <.text+0xbd9a>
14000af98:     	cmpq	$0x8, %r14
14000af9c:     	jae	0x14000b0f6 <.text+0xa0f6>
14000afa2:     	xorl	%eax, %eax
14000afa4:     	movq	%rax, %rcx
14000afa7:     	orq	$0x1, %rcx
14000afab:     	testb	$0x1, %r14b
14000afaf:     	je	0x14000cd36 <.text+0xbd36>
14000afb5:     	movss	(%r15,%rax,4), %xmm0
14000afbb:     	movaps	%xmm8, %xmm1
14000afbf:     	subss	%xmm0, %xmm1
14000afc3:     	movsd	0x540(%rbp), %xmm2
14000afcb:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14000afce:     	mulps	%xmm2, %xmm1
14000afd1:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14000afd5:     	addss	%xmm1, %xmm0
14000afd9:     	movss	%xmm0, (%r10,%rax,4)
14000afdf:     	jmp	0x14000cd95 <.text+0xbd95>
14000afe4:     	leaq	(%r12,%rbx,8), %rax
14000afe8:     	addq	$-0x8, %rax
14000afec:     	cmpq	%r12, %rax
14000afef:     	jb	0x14000ae30 <.text+0x9e30>
14000aff5:     	leaq	0x4(%r12), %rax
14000affa:     	leaq	(%r12,%rbx,8), %rcx
14000affe:     	addq	$-0x4, %rcx
14000b002:     	cmpq	%rax, %rcx
14000b005:     	jb	0x14000ae30 <.text+0x9e30>
14000b00b:     	leaq	-0x1(%rbx), %rax
14000b00f:     	shrq	$0x3d, %rax
14000b013:     	movl	$0x0, %eax
14000b018:     	jne	0x14000ae32 <.text+0x9e32>
14000b01e:     	addq	%r12, %rdi
14000b021:     	leaq	(%r14,%rbx,4), %rax
14000b025:     	cmpq	%r12, %rax
14000b028:     	seta	%al
14000b02b:     	cmpq	%rdi, %r14
14000b02e:     	setb	%r8b
14000b032:     	leaq	0x320(%rbp), %rcx
14000b039:     	cmpq	%r12, %rcx
14000b03c:     	seta	%cl
14000b03f:     	leaq	0x310(%rbp), %rdx
14000b046:     	cmpq	%rdi, %rdx
14000b049:     	setb	%dl
14000b04c:     	testb	%r8b, %al
14000b04f:     	jne	0x14000ae30 <.text+0x9e30>
14000b055:     	movl	$0x0, %eax
14000b05a:     	andb	%dl, %cl
14000b05c:     	jne	0x14000ae32 <.text+0x9e32>
14000b062:     	movaps	%xmm6, 0x760(%rbp)
14000b069:     	movq	%rbx, %rax
14000b06c:     	movabsq	$0x1ffffffffffffffc, %rcx # imm = 0x1FFFFFFFFFFFFFFC
14000b076:     	andq	%rcx, %rax
14000b079:     	movaps	0x310(%rbp), %xmm0
14000b080:     	movaps	%xmm0, %xmm1
14000b083:     	shufps	$0xff, %xmm0, %xmm1     # xmm1 = xmm1[3,3],xmm0[3,3]
14000b087:     	movaps	%xmm0, %xmm2
14000b08a:     	shufps	$0xaa, %xmm0, %xmm2     # xmm2 = xmm2[2,2],xmm0[2,2]
14000b08e:     	movaps	%xmm0, %xmm3
14000b091:     	shufps	$0x55, %xmm0, %xmm3     # xmm3 = xmm3[1,1],xmm0[1,1]
14000b095:     	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14000b099:     	xorl	%ecx, %ecx
14000b09b:     	nopl	(%rax,%rax)
14000b0a0:     	movups	(%r14,%rcx,4), %xmm4
14000b0a5:     	movaps	%xmm10, %xmm5
14000b0a9:     	subps	%xmm4, %xmm5
14000b0ac:     	movaps	%xmm5, %xmm6
14000b0af:     	mulps	%xmm0, %xmm6
14000b0b2:     	mulps	%xmm3, %xmm5
14000b0b5:     	movaps	%xmm4, %xmm7
14000b0b8:     	mulps	%xmm2, %xmm7
14000b0bb:     	addps	%xmm6, %xmm7
14000b0be:     	mulps	%xmm1, %xmm4
14000b0c1:     	addps	%xmm5, %xmm4
14000b0c4:     	movaps	%xmm7, %xmm5
14000b0c7:     	unpcklps	%xmm4, %xmm5            # xmm5 = xmm5[0],xmm4[0],xmm5[1],xmm4[1]
14000b0ca:     	unpckhps	%xmm4, %xmm7            # xmm7 = xmm7[2],xmm4[2],xmm7[3],xmm4[3]
14000b0cd:     	movups	%xmm7, 0x10(%r12,%rcx,8)
14000b0d3:     	movups	%xmm5, (%r12,%rcx,8)
14000b0d8:     	addq	$0x4, %rcx
14000b0dc:     	cmpq	%rcx, %rax
14000b0df:     	jne	0x14000b0a0 <.text+0xa0a0>
14000b0e1:     	cmpq	%rax, %rbx
14000b0e4:     	movaps	0x760(%rbp), %xmm6
14000b0eb:     	jne	0x14000ae32 <.text+0x9e32>
14000b0f1:     	jmp	0x14000aef3 <.text+0x9ef3>
14000b0f6:     	leaq	(%r10,%rdi), %rax
14000b0fa:     	addq	%r15, %rdi
14000b0fd:     	leaq	0x548(%rbp), %rcx
14000b104:     	cmpq	%r10, %rcx
14000b107:     	seta	%r8b
14000b10b:     	leaq	0x540(%rbp), %rcx
14000b112:     	cmpq	%rax, %rcx
14000b115:     	setb	%r9b
14000b119:     	cmpq	%r10, %rdi
14000b11c:     	seta	%cl
14000b11f:     	cmpq	%rax, %r15
14000b122:     	setb	%dl
14000b125:     	testb	%r9b, %r8b
14000b128:     	jne	0x14000afa2 <.text+0x9fa2>
14000b12e:     	movl	$0x0, %eax
14000b133:     	andb	%dl, %cl
14000b135:     	jne	0x14000afa4 <.text+0x9fa4>
14000b13b:     	movq	%r14, %rax
14000b13e:     	movabsq	$0x3ffffffffffffff8, %rcx # imm = 0x3FFFFFFFFFFFFFF8
14000b148:     	andq	%rcx, %rax
14000b14b:     	movaps	0x540(%rbp), %xmm0
14000b152:     	movaps	%xmm0, %xmm1
14000b155:     	shufps	$0x0, %xmm0, %xmm1      # xmm1 = xmm1[0,0],xmm0[0,0]
14000b159:     	shufps	$0x55, %xmm0, %xmm0     # xmm0 = xmm0[1,1,1,1]
14000b15d:     	xorl	%ecx, %ecx
14000b15f:     	nop
14000b160:     	movups	(%r15,%rcx,4), %xmm2
14000b165:     	movups	0x10(%r15,%rcx,4), %xmm3
14000b16b:     	movaps	%xmm10, %xmm4
14000b16f:     	subps	%xmm2, %xmm4
14000b172:     	movaps	%xmm10, %xmm5
14000b176:     	subps	%xmm3, %xmm5
14000b179:     	mulps	%xmm1, %xmm4
14000b17c:     	mulps	%xmm1, %xmm5
14000b17f:     	mulps	%xmm0, %xmm2
14000b182:     	addps	%xmm4, %xmm2
14000b185:     	mulps	%xmm0, %xmm3
14000b188:     	addps	%xmm5, %xmm3
14000b18b:     	movups	%xmm2, (%r10,%rcx,4)
14000b190:     	movups	%xmm3, 0x10(%r10,%rcx,4)
14000b196:     	addq	$0x8, %rcx
14000b19a:     	cmpq	%rcx, %rax
14000b19d:     	jne	0x14000b160 <.text+0xa160>
14000b19f:     	cmpq	%rax, %r14
14000b1a2:     	jne	0x14000afa4 <.text+0x9fa4>
14000b1a8:     	jmp	0x14000cd9a <.text+0xbd9a>
14000b1ad:     	cmpb	$0x1, 0xe8(%r13)
14000b1b5:     	sete	%al
14000b1b8:     	movq	0x7b0(%rbp), %rcx
14000b1bf:     	cmpq	0x668(%rbp), %rcx
14000b1c6:     	je	0x14000adfc <.text+0x9dfc>
14000b1cc:     	xorl	%ecx, %ecx
14000b1ce:     	movzbl	%al, %r15d
14000b1d2:     	movzbl	%cl, %r12d
14000b1d6:     	movq	0x938(%rbp), %rax
14000b1dd:     	movq	(%rax), %rdi
14000b1e0:     	movb	0x810(%rbp), %r14b
14000b1e7:     	movb	$0x1, 0x816(%rbp)
14000b1ee:     	movb	$0x1, 0x815(%rbp)
14000b1f5:     	leaq	0x2a0(%rbp), %rcx
14000b1fc:     	callq	0x1400284b0 <.text+0x274b0>
14000b201:     	movzwl	%r12w, %ecx
14000b205:     	shll	$0x8, %ecx
14000b208:     	movzwl	%r15w, %r8d
14000b20c:     	orl	%ecx, %r8d
14000b20f:     	movl	0x588(%rbp), %ecx
14000b215:     	movl	%ecx, 0x6a0(%rbp)
14000b21b:     	movss	0x560(%rbp), %xmm0
14000b223:     	movss	%xmm0, 0x6a4(%rbp)
14000b22b:     	movl	0x580(%rbp), %ecx
14000b231:     	movl	%ecx, 0x6a8(%rbp)
14000b237:     	movss	0x568(%rbp), %xmm0
14000b23f:     	movss	%xmm0, 0x6ac(%rbp)
14000b247:     	movb	$0x1, 0x816(%rbp)
14000b24e:     	movb	$0x1, 0x815(%rbp)
14000b255:     	andb	$0x1, %dl
14000b258:     	movb	%dl, 0x68(%rsp)
14000b25c:     	andb	$0x1, %al
14000b25e:     	movb	%al, 0x60(%rsp)
14000b262:     	movb	%r14b, 0x58(%rsp)
14000b267:     	movq	%rbx, 0x80(%rsp)
14000b26f:     	movq	0x7f0(%rbp), %rax
14000b276:     	movq	%rax, 0x78(%rsp)
14000b27b:     	leaq	0x6a0(%rbp), %rax
14000b282:     	movq	%rax, 0x70(%rsp)
14000b287:     	movb	%sil, 0x50(%rsp)
14000b28c:     	movl	0x7b8(%rbp), %eax
14000b292:     	movb	%al, 0x48(%rsp)
14000b296:     	movw	%r8w, 0x40(%rsp)
14000b29c:     	movq	%rdi, 0x38(%rsp)
14000b2a1:     	movl	0x528(%rbp), %eax
14000b2a7:     	movl	%eax, 0x30(%rsp)
14000b2ab:     	movq	0x758(%rbp), %rax
14000b2b2:     	movl	%eax, 0x28(%rsp)
14000b2b6:     	leaq	0x428(%rbp), %rax
14000b2bd:     	movq	%rax, 0x20(%rsp)
14000b2c2:     	leaq	0x140(%rbp), %rcx
14000b2c9:     	leaq	0x548(%rbp), %rdx
14000b2d0:     	leaq	0x650(%rbp), %r8
14000b2d7:     	leaq	0x680(%rbp), %r9
14000b2de:     	callq	0x1407f98e0 <.text+0x7f88e0>
14000b2e3:     	movb	$0x1, 0x816(%rbp)
14000b2ea:     	movb	$0x1, 0x815(%rbp)
14000b2f1:     	movq	0x268(%rbp), %rcx
14000b2f8:     	callq	0x140987ca0 <.text+0x986ca0>
14000b2fd:     	movq	0x550(%rbp), %rdx
14000b304:     	movq	0x558(%rbp), %r8
14000b30b:     	movq	0x658(%rbp), %r9
14000b312:     	movq	0x660(%rbp), %rcx
14000b319:     	movups	0x688(%rbp), %xmm0
14000b320:     	movl	0x588(%rbp), %r10d
14000b327:     	movl	%r10d, 0x6a0(%rbp)
14000b32e:     	movss	0x560(%rbp), %xmm1
14000b336:     	movss	%xmm1, 0x6a4(%rbp)
14000b33e:     	movl	0x580(%rbp), %r10d
14000b345:     	movl	%r10d, 0x6a8(%rbp)
14000b34c:     	movss	0x568(%rbp), %xmm1
14000b354:     	movss	%xmm1, 0x6ac(%rbp)
14000b35c:     	movb	$0x1, 0x816(%rbp)
14000b363:     	movb	$0x1, 0x815(%rbp)
14000b36a:     	movq	%rbx, 0x60(%rsp)
14000b36f:     	movq	0x7f0(%rbp), %r10
14000b376:     	movq	%r10, 0x58(%rsp)
14000b37b:     	leaq	0x6a0(%rbp), %r10
14000b382:     	movq	%r10, 0x50(%rsp)
14000b387:     	movq	0x758(%rbp), %r10
14000b38e:     	movl	%r10d, 0x48(%rsp)
14000b393:     	movq	0x940(%rbp), %r10
14000b39a:     	movq	%r10, 0x40(%rsp)
14000b39f:     	leaq	0x140(%rbp), %r10
14000b3a6:     	movq	%r10, 0x38(%rsp)
14000b3ab:     	movups	%xmm0, 0x28(%rsp)
14000b3b0:     	movq	%rcx, 0x20(%rsp)
14000b3b5:     	movq	%rax, %rcx
14000b3b8:     	callq	0x1407dc170 <.text+0x7db170>
14000b3bd:     	movq	0x7e0(%rbp), %rax
14000b3c4:     	cmpb	$0x0, 0xe8(%rax)
14000b3cb:     	je	0x14000b51f <.text+0xa51f>
14000b3d1:     	xorl	%eax, %eax
14000b3d3:     	movq	0x7f0(%rbp), %r10
14000b3da:     	movd	0x291a4f2(%rip), %xmm0  # 0x1429258d4
14000b3e2:     	testb	%al, %al
14000b3e4:     	jne	0x14000b3ee <.text+0xa3ee>
14000b3e6:     	movd	0x291a4ea(%rip), %xmm0  # 0x1429258d8
14000b3ee:     	movq	0x550(%rbp), %rcx
14000b3f5:     	movq	0x558(%rbp), %rdx
14000b3fc:     	movq	0x658(%rbp), %r8
14000b403:     	movq	0x660(%rbp), %r9
14000b40a:     	movups	0x688(%rbp), %xmm1
14000b411:     	movq	0x938(%rbp), %rax
14000b418:     	movq	(%rax), %rax
14000b41b:     	movb	$0x1, 0x816(%rbp)
14000b422:     	movb	$0x1, 0x815(%rbp)
14000b429:     	movq	%rbx, 0x78(%rsp)
14000b42e:     	movq	%r10, 0x70(%rsp)
14000b433:     	movq	0x240(%rbp), %r10
14000b43a:     	movq	%r10, 0x68(%rsp)
14000b43f:     	movd	%xmm0, 0x60(%rsp)
14000b445:     	movl	0x7b8(%rbp), %r10d
14000b44c:     	movb	%r10b, 0x58(%rsp)
14000b451:     	movq	%rax, 0x48(%rsp)
14000b456:     	movq	0x758(%rbp), %rax
14000b45d:     	movl	%eax, 0x40(%rsp)
14000b461:     	leaq	0x140(%rbp), %rax
14000b468:     	movq	%rax, 0x38(%rsp)
14000b46d:     	leaq	0x428(%rbp), %rax
14000b474:     	movq	%rax, 0x30(%rsp)
14000b479:     	movups	%xmm1, 0x20(%rsp)
14000b47e:     	movb	$0x1, 0x80(%rsp)
14000b486:     	movw	$0x0, 0x50(%rsp)
14000b48d:     	callq	0x1407da0b0 <.text+0x7d90b0>
14000b492:     	cmpb	$0x0, 0x7b8(%rbp)
14000b499:     	je	0x14000bb1d <.text+0xab1d>
14000b49f:     	movzbl	0x80f(%rbp), %esi
14000b4a6:     	cmpb	$0x1, %sil
14000b4aa:     	ja	0x14000b545 <.text+0xa545>
14000b4b0:     	movzbl	0x810(%rbp), %r14d
14000b4b8:     	movq	0x7e0(%rbp), %r10
14000b4bf:     	movl	0x128(%r10), %r8d
14000b4c6:     	movl	0x12c(%r10), %r9d
14000b4cd:     	movl	%r8d, 0x6a0(%rbp)
14000b4d4:     	movl	%r9d, 0x6a4(%rbp)
14000b4db:     	movq	0x7c0(%rbp), %rax
14000b4e2:     	movq	0x70(%rax), %rdi
14000b4e6:     	testq	%rdi, %rdi
14000b4e9:     	je	0x14000b5f8 <.text+0xa5f8>
14000b4ef:     	cmpq	$0x1, %rdi
14000b4f3:     	jne	0x14000b58a <.text+0xa58a>
14000b4f9:     	movq	0x7c0(%rbp), %rax
14000b500:     	movq	0x68(%rax), %rax
14000b504:     	cmpl	%r8d, 0x8(%rax)
14000b508:     	jne	0x14000b5f8 <.text+0xa5f8>
14000b50e:     	cmpl	%r9d, 0xc(%rax)
14000b512:     	jne	0x14000b5f8 <.text+0xa5f8>
14000b518:     	xorl	%edx, %edx
14000b51a:     	jmp	0x14000b5e9 <.text+0xa5e9>
14000b51f:     	cmpb	$0x0, 0x108(%rax)
14000b526:     	sete	%al
14000b529:     	movq	0x7f0(%rbp), %r10
14000b530:     	movd	0x291a39c(%rip), %xmm0  # 0x1429258d4
14000b538:     	testb	%al, %al
14000b53a:     	je	0x14000b3e6 <.text+0xa3e6>
14000b540:     	jmp	0x14000b3ee <.text+0xa3ee>
14000b545:     	xorl	%r14d, %r14d
14000b548:     	movq	0x7e0(%rbp), %r13
14000b54f:     	movq	0x7f0(%rbp), %r15
14000b556:     	jmp	0x14000bd9c <.text+0xad9c>
14000b55b:     	testq	%rax, %rax
14000b55e:     	je	0x14000b829 <.text+0xa829>
14000b564:     	xorl	%eax, %eax
14000b566:     	movq	0x7b0(%rbp), %rcx
14000b56d:     	cmpq	0x668(%rbp), %rcx
14000b574:     	jne	0x14000b848 <.text+0xa848>
14000b57a:     	cmpb	$0x1, 0x108(%r13)
14000b582:     	sete	%cl
14000b585:     	jmp	0x14000b84a <.text+0xa84a>
14000b58a:     	movq	0x7c0(%rbp), %rax
14000b591:     	movq	0x98(%rax), %rcx
14000b598:     	movq	0xa0(%rax), %rdx
14000b59f:     	callq	0x140106840 <.text+0x105840>
14000b5a4:     	movb	$0x1, 0x816(%rbp)
14000b5ab:     	movb	$0x1, 0x815(%rbp)
14000b5b2:     	movq	0x350(%rbp), %rcx
14000b5b9:     	movq	%rax, %rdx
14000b5bc:     	leaq	0x6a0(%rbp), %r8
14000b5c3:     	callq	0x1403ab1c0 <.text+0x3aa1c0>
14000b5c8:     	cmpq	$0x1, %rax
14000b5cc:     	movq	0x7e0(%rbp), %r10
14000b5d3:     	jne	0x14000b5f8 <.text+0xa5f8>
14000b5d5:     	cmpq	%rdi, %rdx
14000b5d8:     	jae	0x1400119ec <.text+0x109ec>
14000b5de:     	movq	0x7c0(%rbp), %rax
14000b5e5:     	movq	0x68(%rax), %rax
14000b5e9:     	leaq	(%rdx,%rdx,4), %rcx
14000b5ed:     	cmpb	$0x0, 0x10(%rax,%rcx,8)
14000b5f2:     	je	0x14000b6bc <.text+0xa6bc>
14000b5f8:     	movl	0x128(%r10), %r9d
14000b5ff:     	movl	0x12c(%r10), %r8d
14000b606:     	movl	%r8d, 0x6a0(%rbp)
14000b60d:     	movl	%r9d, 0x6a4(%rbp)
14000b614:     	movq	0x7c0(%rbp), %rax
14000b61b:     	movq	0x70(%rax), %rdi
14000b61f:     	testq	%rdi, %rdi
14000b622:     	je	0x14000b742 <.text+0xa742>
14000b628:     	cmpq	$0x1, %rdi
14000b62c:     	jne	0x14000b651 <.text+0xa651>
14000b62e:     	movq	0x7c0(%rbp), %rax
14000b635:     	movq	0x68(%rax), %rax
14000b639:     	cmpl	%r8d, 0x8(%rax)
14000b63d:     	jne	0x14000b742 <.text+0xa742>
14000b643:     	cmpl	%r9d, 0xc(%rax)
14000b647:     	jne	0x14000b742 <.text+0xa742>
14000b64d:     	xorl	%edx, %edx
14000b64f:     	jmp	0x14000b6ad <.text+0xa6ad>
14000b651:     	movq	0x7c0(%rbp), %rax
14000b658:     	movq	0x98(%rax), %rcx
14000b65f:     	movq	0xa0(%rax), %rdx
14000b666:     	callq	0x140106840 <.text+0x105840>
14000b66b:     	movb	$0x1, 0x816(%rbp)
14000b672:     	movb	$0x1, 0x815(%rbp)
14000b679:     	movq	0x350(%rbp), %rcx
14000b680:     	movq	%rax, %rdx
14000b683:     	leaq	0x6a0(%rbp), %r8
14000b68a:     	callq	0x1403ab1c0 <.text+0x3aa1c0>
14000b68f:     	cmpq	$0x1, %rax
14000b693:     	jne	0x14000b742 <.text+0xa742>
14000b699:     	cmpq	%rdi, %rdx
14000b69c:     	jae	0x140011a0e <.text+0x10a0e>
14000b6a2:     	movq	0x7c0(%rbp), %rax
14000b6a9:     	movq	0x68(%rax), %rax
14000b6ad:     	leaq	(%rdx,%rdx,4), %rcx
14000b6b1:     	cmpb	$0x0, 0x10(%rax,%rcx,8)
14000b6b6:     	jne	0x14000b742 <.text+0xa742>
14000b6bc:     	leaq	(%rax,%rcx,8), %rax
14000b6c0:     	addq	$0x10, %rax
14000b6c4:     	movq	0x938(%rbp), %rcx
14000b6cb:     	movq	(%rcx), %rcx
14000b6ce:     	movdqu	0x550(%rbp), %xmm0
14000b6d6:     	movups	0x658(%rbp), %xmm1
14000b6dd:     	movdqu	0x688(%rbp), %xmm2
14000b6e5:     	movss	0x4(%rax), %xmm3
14000b6ea:     	movss	0x8(%rax), %xmm4
14000b6ef:     	movb	$0x1, 0x816(%rbp)
14000b6f6:     	movb	$0x1, 0x815(%rbp)
14000b6fd:     	movss	%xmm4, 0x68(%rsp)
14000b703:     	movss	%xmm3, 0x60(%rsp)
14000b709:     	movb	%r14b, 0x58(%rsp)
14000b70e:     	andb	$0x1, %sil
14000b712:     	movb	%sil, 0x50(%rsp)
14000b717:     	movdqu	%xmm2, 0x40(%rsp)
14000b71d:     	movups	%xmm1, 0x30(%rsp)
14000b722:     	movdqu	%xmm0, 0x20(%rsp)
14000b728:     	movq	0x758(%rbp), %rdx
14000b72f:     	movl	0x7b8(%rbp), %r8d
14000b736:     	leaq	0x140(%rbp), %r9
14000b73d:     	callq	0x1407febb0 <.text+0x7fdbb0>
14000b742:     	xorl	%edx, %edx
14000b744:     	movq	0x7b0(%rbp), %rax
14000b74b:     	cmpq	0x668(%rbp), %rax
14000b752:     	sete	%dl
14000b755:     	xorl	%eax, %eax
14000b757:     	cmpq	$0x0, 0x670(%rbp)
14000b75f:     	sete	%al
14000b762:     	movq	0x950(%rbp), %rcx
14000b769:     	movq	(%rcx), %rcx
14000b76c:     	movdqu	0x550(%rbp), %xmm0
14000b774:     	movups	0x658(%rbp), %xmm1
14000b77b:     	movdqu	0x688(%rbp), %xmm2
14000b783:     	movq	0x378(%rbp), %r8
14000b78a:     	movq	(%r8), %r8
14000b78d:     	movq	0x958(%rbp), %r9
14000b794:     	movq	(%r9), %r9
14000b797:     	movq	0x938(%rbp), %r10
14000b79e:     	movq	(%r10), %r10
14000b7a1:     	shll	$0x8, %edx
14000b7a4:     	orl	%edx, %eax
14000b7a6:     	movb	$0x1, 0x816(%rbp)
14000b7ad:     	movb	$0x1, 0x815(%rbp)
14000b7b4:     	movq	%r10, 0x80(%rsp)
14000b7bc:     	movq	%r9, 0x78(%rsp)
14000b7c1:     	movq	%r8, 0x70(%rsp)
14000b7c6:     	movdqu	%xmm2, 0x60(%rsp)
14000b7cc:     	movups	%xmm1, 0x50(%rsp)
14000b7d1:     	movdqu	%xmm0, 0x40(%rsp)
14000b7d7:     	leaq	0x140(%rbp), %rdx
14000b7de:     	movq	%rdx, 0x38(%rsp)
14000b7e3:     	movl	0x7b8(%rbp), %edx
14000b7e9:     	movb	%dl, 0x30(%rsp)
14000b7ed:     	movl	0x528(%rbp), %edx
14000b7f3:     	movl	%edx, 0x28(%rsp)
14000b7f7:     	movq	0x758(%rbp), %rdx
14000b7fe:     	movl	%edx, 0x20(%rsp)
14000b802:     	movw	%ax, 0x88(%rsp)
14000b80a:     	movq	0x7c0(%rbp), %rdx
14000b811:     	movq	0x678(%rbp), %r8
14000b818:     	leaq	0x428(%rbp), %r9
14000b81f:     	callq	0x1407fcc80 <.text+0x7fbc80>
14000b824:     	jmp	0x14000bb1d <.text+0xab1d>
14000b829:     	cmpb	$0x1, 0xe8(%r13)
14000b831:     	sete	%al
14000b834:     	movq	0x7b0(%rbp), %rcx
14000b83b:     	cmpq	0x668(%rbp), %rcx
14000b842:     	je	0x14000b57a <.text+0xa57a>
14000b848:     	xorl	%ecx, %ecx
14000b84a:     	movzbl	%al, %r15d
14000b84e:     	movzbl	%cl, %r12d
14000b852:     	movq	0x938(%rbp), %rax
14000b859:     	movq	(%rax), %rdi
14000b85c:     	movb	0x810(%rbp), %r14b
14000b863:     	movb	$0x1, 0x816(%rbp)
14000b86a:     	movb	$0x1, 0x815(%rbp)
14000b871:     	leaq	0x2a0(%rbp), %rcx
14000b878:     	callq	0x1400284b0 <.text+0x274b0>
14000b87d:     	movl	$0x0, 0x6a0(%rbp)
14000b887:     	movl	$0x0, 0x6a8(%rbp)
14000b891:     	movzwl	%r12w, %ecx
14000b895:     	shll	$0x8, %ecx
14000b898:     	movzwl	%r15w, %r8d
14000b89c:     	orl	%ecx, %r8d
14000b89f:     	movb	$0x1, 0x816(%rbp)
14000b8a6:     	movb	$0x1, 0x815(%rbp)
14000b8ad:     	andb	$0x1, %dl
14000b8b0:     	movb	%dl, 0x68(%rsp)
14000b8b4:     	andb	$0x1, %al
14000b8b6:     	movb	%al, 0x60(%rsp)
14000b8ba:     	movb	%r14b, 0x58(%rsp)
14000b8bf:     	movq	%rbx, 0x80(%rsp)
14000b8c7:     	movq	0x7f0(%rbp), %rax
14000b8ce:     	movq	%rax, 0x78(%rsp)
14000b8d3:     	leaq	0x6a0(%rbp), %rax
14000b8da:     	movq	%rax, 0x70(%rsp)
14000b8df:     	movb	%sil, 0x50(%rsp)
14000b8e4:     	movl	0x7b8(%rbp), %eax
14000b8ea:     	movb	%al, 0x48(%rsp)
14000b8ee:     	movw	%r8w, 0x40(%rsp)
14000b8f4:     	movq	%rdi, 0x38(%rsp)
14000b8f9:     	movl	0x528(%rbp), %eax
14000b8ff:     	movl	%eax, 0x30(%rsp)
14000b903:     	movq	0x758(%rbp), %rax
14000b90a:     	movl	%eax, 0x28(%rsp)
14000b90e:     	leaq	0x428(%rbp), %rax
14000b915:     	movq	%rax, 0x20(%rsp)
14000b91a:     	leaq	0x140(%rbp), %rcx
14000b921:     	leaq	0x548(%rbp), %rdx
14000b928:     	leaq	0x650(%rbp), %r8
14000b92f:     	leaq	0x680(%rbp), %r9
14000b936:     	callq	0x1407f98e0 <.text+0x7f88e0>
14000b93b:     	movq	0x7e0(%rbp), %r13
14000b942:     	cmpb	$0x0, 0xe8(%r13)
14000b94a:     	je	0x14000b958 <.text+0xa958>
14000b94c:     	movl	$0x0, 0x760(%rbp)
14000b956:     	jmp	0x14000b969 <.text+0xa969>
14000b958:     	cmpb	$0x0, 0x108(%r13)
14000b960:     	sete	%al
14000b963:     	movl	%eax, 0x760(%rbp)
14000b969:     	movq	0x550(%rbp), %rax
14000b970:     	movq	%rax, 0x800(%rbp)
14000b977:     	movq	0x558(%rbp), %rax
14000b97e:     	movq	%rax, 0x780(%rbp)
14000b985:     	movq	0x658(%rbp), %rdi
14000b98c:     	movq	0x660(%rbp), %r15
14000b993:     	movq	0x688(%rbp), %rsi
14000b99a:     	movq	0x690(%rbp), %r14
14000b9a1:     	movq	0x938(%rbp), %rax
14000b9a8:     	movq	(%rax), %r12
14000b9ab:     	movb	$0x1, 0x816(%rbp)
14000b9b2:     	movb	$0x1, 0x815(%rbp)
14000b9b9:     	movq	0x2e8(%rbp), %rcx
14000b9c0:     	callq	0x140959ef0 <.text+0x958ef0>
14000b9c5:     	testq	%rax, %rax
14000b9c8:     	je	0x14000b9f0 <.text+0xa9f0>
14000b9ca:     	cmpq	$0x0, 0x558(%rbp)
14000b9d2:     	je	0x140011837 <.text+0x10837>
14000b9d8:     	movq	0x550(%rbp), %rax
14000b9df:     	movss	0x2919ee9(%rip), %xmm0  # 0x1429258d0
14000b9e7:     	ucomiss	(%rax), %xmm0
14000b9ea:     	setae	%r13b
14000b9ee:     	jmp	0x14000b9f3 <.text+0xa9f3>
14000b9f0:     	xorl	%r13d, %r13d
14000b9f3:     	movb	$0x1, 0x816(%rbp)
14000b9fa:     	movb	$0x1, 0x815(%rbp)
14000ba01:     	movq	0x118(%rbp), %rcx
14000ba08:     	callq	0x140959ef0 <.text+0x958ef0>
14000ba0d:     	testq	%rax, %rax
14000ba10:     	je	0x14000ba66 <.text+0xaa66>
14000ba12:     	movq	0x558(%rbp), %rax
14000ba19:     	testq	%rax, %rax
14000ba1c:     	je	0x140011818 <.text+0x10818>
14000ba22:     	movq	0x550(%rbp), %rcx
14000ba29:     	leaq	(%rcx,%rax,4), %rax
14000ba2d:     	addq	$-0x4, %rax
14000ba31:     	testq	%rax, %rax
14000ba34:     	je	0x140011818 <.text+0x10818>
14000ba3a:     	movd	0x2919e92(%rip), %xmm0  # 0x1429258d4
14000ba42:     	cmpb	$0x0, 0x760(%rbp)
14000ba49:     	jne	0x14000ba53 <.text+0xaa53>
14000ba4b:     	movd	0x2919e85(%rip), %xmm0  # 0x1429258d8
14000ba53:     	movss	0x2919e75(%rip), %xmm1  # 0x1429258d0
14000ba5b:     	ucomiss	(%rax), %xmm1
14000ba5e:     	jb	0x14000ba7f <.text+0xaa7f>
14000ba60:     	movw	$0x100, %ax             # imm = 0x100
14000ba64:     	jmp	0x14000ba81 <.text+0xaa81>
14000ba66:     	movd	0x2919e66(%rip), %xmm0  # 0x1429258d4
14000ba6e:     	cmpb	$0x0, 0x760(%rbp)
14000ba75:     	jne	0x14000ba7f <.text+0xaa7f>
14000ba77:     	movd	0x2919e59(%rip), %xmm0  # 0x1429258d8
14000ba7f:     	xorl	%eax, %eax
14000ba81:     	movzbl	%r13b, %ecx
14000ba85:     	movzwl	%ax, %eax
14000ba88:     	orl	%ecx, %eax
14000ba8a:     	movb	$0x1, 0x816(%rbp)
14000ba91:     	movb	$0x1, 0x815(%rbp)
14000ba98:     	movw	%ax, 0x50(%rsp)
14000ba9d:     	movq	%r12, 0x48(%rsp)
14000baa2:     	movq	%r14, 0x28(%rsp)
14000baa7:     	movq	%rsi, 0x20(%rsp)
14000baac:     	movq	%rbx, 0x78(%rsp)
14000bab1:     	movq	0x7f0(%rbp), %rax
14000bab8:     	movq	%rax, 0x70(%rsp)
14000babd:     	movq	0x240(%rbp), %rax
14000bac4:     	movq	%rax, 0x68(%rsp)
14000bac9:     	movd	%xmm0, 0x60(%rsp)
14000bacf:     	movl	0x7b8(%rbp), %eax
14000bad5:     	movb	%al, 0x58(%rsp)
14000bad9:     	movq	0x758(%rbp), %rax
14000bae0:     	movl	%eax, 0x40(%rsp)
14000bae4:     	leaq	0x140(%rbp), %rax
14000baeb:     	movq	%rax, 0x38(%rsp)
14000baf0:     	leaq	0x428(%rbp), %rax
14000baf7:     	movq	%rax, 0x30(%rsp)
14000bafc:     	movb	$0x0, 0x80(%rsp)
14000bb04:     	movq	%rdi, %r8
14000bb07:     	movq	%r15, %r9
14000bb0a:     	movq	0x800(%rbp), %rcx
14000bb11:     	movq	0x780(%rbp), %rdx
14000bb18:     	callq	0x1407da0b0 <.text+0x7d90b0>
14000bb1d:     	movzbl	0x80f(%rbp), %esi
14000bb24:     	cmpb	$0x2, %sil
14000bb28:     	movl	$0x0, %r14d
14000bb2e:     	movq	0x7e0(%rbp), %r13
14000bb35:     	movq	0x7f0(%rbp), %r15
14000bb3c:     	jae	0x14000bd9c <.text+0xad9c>
14000bb42:     	movl	0x128(%r13), %r8d
14000bb49:     	movl	0x12c(%r13), %r9d
14000bb50:     	movl	%r8d, 0x6a0(%rbp)
14000bb57:     	movl	%r9d, 0x6a4(%rbp)
14000bb5e:     	movq	0x7c0(%rbp), %rax
14000bb65:     	movq	0x70(%rax), %rdi
14000bb69:     	testq	%rdi, %rdi
14000bb6c:     	je	0x14000bd95 <.text+0xad95>
14000bb72:     	cmpq	$0x1, %rdi
14000bb76:     	jne	0x14000bb9b <.text+0xab9b>
14000bb78:     	movq	0x7c0(%rbp), %rax
14000bb7f:     	movq	0x68(%rax), %rax
14000bb83:     	cmpl	%r8d, 0x8(%rax)
14000bb87:     	jne	0x14000bd95 <.text+0xad95>
14000bb8d:     	cmpl	%r9d, 0xc(%rax)
14000bb91:     	jne	0x14000bd95 <.text+0xad95>
14000bb97:     	xorl	%edx, %edx
14000bb99:     	jmp	0x14000bc0b <.text+0xac0b>
14000bb9b:     	movq	0x7c0(%rbp), %rax
14000bba2:     	movq	0x98(%rax), %rcx
14000bba9:     	movq	0xa0(%rax), %rdx
14000bbb0:     	callq	0x140106840 <.text+0x105840>
14000bbb5:     	movb	$0x1, 0x816(%rbp)
14000bbbc:     	movb	$0x1, 0x815(%rbp)
14000bbc3:     	movq	0x350(%rbp), %rcx
14000bbca:     	movq	%rax, %rdx
14000bbcd:     	leaq	0x6a0(%rbp), %r8
14000bbd4:     	callq	0x1403ab1c0 <.text+0x3aa1c0>
14000bbd9:     	cmpq	$0x1, %rax
14000bbdd:     	movl	$0x0, %r14d
14000bbe3:     	movq	0x7e0(%rbp), %r13
14000bbea:     	movq	0x7f0(%rbp), %r15
14000bbf1:     	jne	0x14000bd95 <.text+0xad95>
14000bbf7:     	cmpq	%rdi, %rdx
14000bbfa:     	jae	0x14001191b <.text+0x1091b>
14000bc00:     	movq	0x7c0(%rbp), %rax
14000bc07:     	movq	0x68(%rax), %rax
14000bc0b:     	leaq	(%rdx,%rdx,4), %rcx
14000bc0f:     	cmpb	$0x0, 0x10(%rax,%rcx,8)
14000bc14:     	jne	0x14000bd95 <.text+0xad95>
14000bc1a:     	movzbl	0x12(%rax,%rcx,8), %edx
14000bc1f:     	orb	0x11(%rax,%rcx,8), %dl
14000bc23:     	testb	$0x1, %dl
14000bc26:     	je	0x14000bd95 <.text+0xad95>
14000bc2c:     	movl	0x128(%r13), %ecx
14000bc33:     	movl	0x12c(%r13), %edx
14000bc3a:     	movb	$0x1, 0x816(%rbp)
14000bc41:     	movb	$0x1, 0x815(%rbp)
14000bc48:     	callq	0x140959e70 <.text+0x958e70>
14000bc4d:     	movq	%rax, %rdi
14000bc50:     	movq	%rax, 0x6a0(%rbp)
14000bc57:     	movq	0x7c0(%rbp), %rax
14000bc5e:     	cmpq	$0x0, 0x1b0(%rax)
14000bc66:     	movl	$0x0, %r14d
14000bc6c:     	movq	0x7f0(%rbp), %r15
14000bc73:     	je	0x14000bd8e <.text+0xad8e>
14000bc79:     	movq	0xc0(%rbp), %rcx
14000bc80:     	leaq	0x6a0(%rbp), %rdx
14000bc87:     	callq	0x1405b0d70 <.text+0x5afd70>
14000bc8c:     	movq	0x7c0(%rbp), %rdx
14000bc93:     	movq	0x198(%rdx), %rcx
14000bc9a:     	movq	0x1a0(%rdx), %rdx
14000bca1:     	movq	%rdx, %r8
14000bca4:     	andq	%rax, %r8
14000bca7:     	shrq	$0x39, %rax
14000bcab:     	movd	%eax, %xmm0
14000bcaf:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14000bcb3:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14000bcb8:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14000bcbd:     	leaq	-0x20(%rcx), %rax
14000bcc1:     	xorl	%r9d, %r9d
14000bcc4:     	movdqu	(%rcx,%r8), %xmm1
14000bcca:     	movdqa	%xmm1, %xmm2
14000bcce:     	pcmpeqb	%xmm0, %xmm2
14000bcd2:     	pmovmskb	%xmm2, %r10d
14000bcd7:     	testl	%r10d, %r10d
14000bcda:     	je	0x14000bd0c <.text+0xad0c>
14000bcdc:     	tzcntl	%r10d, %r11d
14000bce1:     	addq	%r8, %r11
14000bce4:     	andq	%rdx, %r11
14000bce7:     	movq	%r11, %rsi
14000bcea:     	shlq	$0x5, %rsi
14000bcee:     	movq	%rax, %r14
14000bcf1:     	subq	%rsi, %r14
14000bcf4:     	cmpq	(%r14), %rdi
14000bcf7:     	je	0x14000bd2e <.text+0xad2e>
14000bcf9:     	leal	-0x1(%r10), %r11d
14000bcfd:     	andw	%r10w, %r11w
14000bd01:     	movl	%r11d, %r10d
14000bd04:     	movl	$0x0, %r14d
14000bd0a:     	jne	0x14000bcdc <.text+0xacdc>
14000bd0c:     	pcmpeqb	0x2919c7c(%rip), %xmm1  # 0x142925990
14000bd14:     	pmovmskb	%xmm1, %r10d
14000bd19:     	testl	%r10d, %r10d
14000bd1c:     	jne	0x14000bd8e <.text+0xad8e>
14000bd1e:     	addq	%r9, %r8
14000bd21:     	addq	$0x10, %r8
14000bd25:     	addq	$0x10, %r9
14000bd29:     	andq	%rdx, %r8
14000bd2c:     	jmp	0x14000bcc4 <.text+0xacc4>
14000bd2e:     	negq	%r11
14000bd31:     	shlq	$0x5, %r11
14000bd35:     	movq	-0x8(%rcx,%r11), %rax
14000bd3a:     	cmpq	$0x2, %rax
14000bd3e:     	movl	$0x0, %r14d
14000bd44:     	jb	0x14000bd52 <.text+0xad52>
14000bd46:     	movq	-0x18(%rcx,%r11), %rdx
14000bd4b:     	movq	-0x10(%rcx,%r11), %rax
14000bd50:     	jmp	0x14000bd5a <.text+0xad5a>
14000bd52:     	leaq	(%rcx,%r11), %rdx
14000bd56:     	addq	$-0x18, %rdx
14000bd5a:     	shlq	$0x2, %rax
14000bd5e:     	xorl	%ecx, %ecx
14000bd60:     	cmpq	%rcx, %rax
14000bd63:     	je	0x14000bd8e <.text+0xad8e>
14000bd65:     	cmpb	$0x3, (%rdx,%rcx)
14000bd69:     	leaq	0x4(%rcx), %rcx
14000bd6d:     	jne	0x14000bd60 <.text+0xad60>
14000bd6f:     	movb	$0x1, %sil
14000bd72:     	movq	0x7e0(%rbp), %r13
14000bd79:     	movq	0x798(%rbp), %rdx
14000bd80:     	testq	%rdx, %rdx
14000bd83:     	jne	0x14000c308 <.text+0xb308>
14000bd89:     	jmp	0x14000c31a <.text+0xb31a>
14000bd8e:     	movq	0x7e0(%rbp), %r13
14000bd95:     	movzbl	0x80f(%rbp), %esi
14000bd9c:     	cmpb	$0x4, %sil
14000bda0:     	jne	0x14000c281 <.text+0xb281>
14000bda6:     	movq	0x558(%rbp), %rcx
14000bdad:     	testq	%rcx, %rcx
14000bdb0:     	movq	0x798(%rbp), %rdx
14000bdb7:     	je	0x140011622 <.text+0x10622>
14000bdbd:     	movq	0x550(%rbp), %rax
14000bdc4:     	leaq	(%rax,%rcx,4), %rcx
14000bdc8:     	addq	$-0x4, %rcx
14000bdcc:     	je	0x140011645 <.text+0x10645>
14000bdd2:     	movss	(%rax), %xmm0
14000bdd6:     	movss	(%rcx), %xmm1
14000bdda:     	movaps	%xmm0, %xmm2
14000bddd:     	cmpunordss	%xmm0, %xmm2
14000bde2:     	movaps	%xmm2, %xmm3
14000bde5:     	andps	%xmm1, %xmm3
14000bde8:     	maxss	%xmm0, %xmm1
14000bdec:     	andnps	%xmm1, %xmm2
14000bdef:     	orps	%xmm3, %xmm2
14000bdf2:     	ucomiss	0x2919ad7(%rip), %xmm2  # 0x1429258d0
14000bdf9:     	setbe	%al
14000bdfc:     	orb	0x538(%rbp), %al
14000be02:     	movb	$0x1, %sil
14000be05:     	jne	0x14000c303 <.text+0xb303>
14000be0b:     	movq	$0x0, 0x460(%rbp)
14000be16:     	movq	$0x1, 0x468(%rbp)
14000be21:     	movq	0x690(%rbp), %xmm0
14000be29:     	pshufd	$0x44, %xmm0, %xmm0     # xmm0 = xmm0[0,1,0,1]
14000be2e:     	paddq	0x2919ada(%rip), %xmm0  # 0x142925910
14000be36:     	movdqu	%xmm0, 0x470(%rbp)
14000be3e:     	xorl	%r13d, %r13d
14000be41:     	xorl	%esi, %esi
14000be43:     	nopw	%cs:(%rax,%rax)
14000be50:     	cmpq	$0x20, %r13
14000be54:     	je	0x14000c39c <.text+0xb39c>
14000be5a:     	movq	0x468(%rbp,%r13), %r14
14000be62:     	cmpq	$-0x1, %r14
14000be66:     	je	0x14001108e <.text+0x1008e>
14000be6c:     	movq	0x460(%rbp,%r13), %rcx
14000be74:     	leaq	0x1(%r14), %r9
14000be78:     	movq	%r9, %rdi
14000be7b:     	movq	%rcx, 0x800(%rbp)
14000be82:     	subq	%rcx, %rdi
14000be85:     	jb	0x1400110ad <.text+0x100ad>
14000be8b:     	movq	0x690(%rbp), %rdx
14000be92:     	cmpq	%rdx, %r14
14000be95:     	jae	0x1400110d6 <.text+0x100d6>
14000be9b:     	movq	%r9, 0x760(%rbp)
14000bea2:     	movq	%rsi, 0x780(%rbp)
14000bea9:     	movq	%rdi, %rax
14000beac:     	movl	$0xc, %ecx
14000beb1:     	mulq	%rcx
14000beb4:     	jo	0x14001156a <.text+0x1056a>
14000beba:     	movl	$0x0, %esi
14000bebf:     	movabsq	$0x7ffffffffffffffc, %rcx # imm = 0x7FFFFFFFFFFFFFFC
14000bec9:     	cmpq	%rcx, %rax
14000becc:     	movq	0x108(%rbp), %r15
14000bed3:     	ja	0x140011573 <.text+0x10573>
14000bed9:     	movq	0x688(%rbp), %r12
14000bee0:     	testq	%rax, %rax
14000bee3:     	je	0x14000bf20 <.text+0xaf20>
14000bee5:     	movzbl	0x3628d1d(%rip), %ecx   # 0x143634c09
14000beec:     	movl	$0x4, %esi
14000bef1:     	movl	$0x4, %edx
14000bef6:     	movq	%rax, %rcx
14000bef9:     	movq	%rax, %r15
14000befc:     	callq	0x140613c10 <.text+0x612c10>
14000bf01:     	testq	%rax, %rax
14000bf04:     	je	0x140011573 <.text+0x10573>
14000bf0a:     	movq	%rdi, %rsi
14000bf0d:     	movq	%rax, %r15
14000bf10:     	jmp	0x14000bf28 <.text+0xaf28>
14000bf12:     	nopw	%cs:(%rax,%rax)
14000bf20:     	movl	$0x4, %r15d
14000bf26:     	xorl	%esi, %esi
14000bf28:     	movq	0x800(%rbp), %rax
14000bf2f:     	leaq	(%rax,%rax,2), %rax
14000bf33:     	leaq	(%r12,%rax,4), %rdx
14000bf37:     	leaq	(,%rdi,4), %r12
14000bf3f:     	leaq	(%r12,%r12,2), %r8
14000bf43:     	movq	%r15, %rcx
14000bf46:     	callq	0x142923c00 <.text+0x2922c00>
14000bf4b:     	movq	%rsi, 0x6a0(%rbp)
14000bf52:     	movq	%r15, 0x108(%rbp)
14000bf59:     	movq	%r15, 0x6a8(%rbp)
14000bf60:     	movq	%rdi, 0x6b0(%rbp)
14000bf67:     	movb	$0x1, 0x816(%rbp)
14000bf6e:     	movb	$0x1, 0x815(%rbp)
14000bf75:     	leaq	0x18(%rbp), %rcx
14000bf79:     	leaq	0x6a0(%rbp), %rdx
14000bf80:     	movl	$0x1, %r8d
14000bf86:     	callq	0x140153640 <.text+0x152640>
14000bf8b:     	addq	$0x10, %r13
14000bf8f:     	movq	0x780(%rbp), %rax
14000bf96:     	leaq	0x1(%rax), %rsi
14000bf9a:     	xorl	%eax, %eax
14000bf9c:     	cmpq	0x18(%rbp), %rax
14000bfa0:     	jo	0x14000be50 <.text+0xae50>
14000bfa6:     	movq	0x340(%rbp), %rax
14000bfad:     	movl	0x53c(%rbp), %ecx
14000bfb3:     	movl	%ecx, (%rax)
14000bfb5:     	movq	0x48(%rbp), %rax
14000bfb9:     	movq	%rax, 0x5c0(%rbp)
14000bfc0:     	movups	0x18(%rbp), %xmm0
14000bfc4:     	movups	0x28(%rbp), %xmm1
14000bfc8:     	movdqu	0x38(%rbp), %xmm2
14000bfcd:     	movdqa	%xmm2, 0x5b0(%rbp)
14000bfd5:     	movaps	%xmm1, 0x5a0(%rbp)
14000bfdc:     	movaps	%xmm0, 0x590(%rbp)
14000bfe3:     	movq	0x558(%rbp), %rdx
14000bfea:     	cmpq	%rdx, %r14
14000bfed:     	jae	0x1400112b1 <.text+0x102b1>
14000bff3:     	movq	0x800(%rbp), %rax
14000bffa:     	leaq	(,%rax,4), %r15
14000c002:     	addq	0x550(%rbp), %r15
14000c009:     	testq	%rdi, %rdi
14000c00c:     	je	0x14000c034 <.text+0xb034>
14000c00e:     	movzbl	0x3628bf4(%rip), %eax   # 0x143634c09
14000c015:     	movl	$0x4, %edx
14000c01a:     	movq	%r12, %rcx
14000c01d:     	callq	0x140613c10 <.text+0x612c10>
14000c022:     	testq	%rax, %rax
14000c025:     	je	0x1400118de <.text+0x108de>
14000c02b:     	movq	%rdi, 0x7a0(%rbp)
14000c032:     	jmp	0x14000c044 <.text+0xb044>
14000c034:     	movl	$0x4, %eax
14000c039:     	movq	$0x0, 0x7a0(%rbp)
14000c044:     	movq	%rax, 0x138(%rbp)
14000c04b:     	movq	%rax, %rcx
14000c04e:     	movq	%r15, %rdx
14000c051:     	movq	%r12, %r8
14000c054:     	callq	0x142923c00 <.text+0x2922c00>
14000c059:     	movq	0x660(%rbp), %rdx
14000c060:     	cmpq	%rdx, %r14
14000c063:     	jae	0x1400112c9 <.text+0x102c9>
14000c069:     	movq	%rdi, %rax
14000c06c:     	shrq	$0x3c, %rax
14000c070:     	jne	0x140011811 <.text+0x10811>
14000c076:     	movq	0x800(%rbp), %rax
14000c07d:     	leaq	(,%rax,8), %rdx
14000c085:     	addq	0x658(%rbp), %rdx
14000c08c:     	leaq	(,%rdi,8), %r8
14000c094:     	testq	%rdi, %rdi
14000c097:     	je	0x14000c0d3 <.text+0xb0d3>
14000c099:     	movq	%rdx, 0x670(%rbp)
14000c0a0:     	movzbl	0x3628b62(%rip), %eax   # 0x143634c09
14000c0a7:     	movl	$0x4, %edx
14000c0ac:     	movq	%r8, %r15
14000c0af:     	movq	%r8, %rcx
14000c0b2:     	callq	0x140613c10 <.text+0x612c10>
14000c0b7:     	testq	%rax, %rax
14000c0ba:     	je	0x1400118f7 <.text+0x108f7>
14000c0c0:     	movq	%rdi, 0x7a8(%rbp)
14000c0c7:     	movq	%r15, %r8
14000c0ca:     	movq	0x670(%rbp), %rdx
14000c0d1:     	jmp	0x14000c0e3 <.text+0xb0e3>
14000c0d3:     	movl	$0x4, %eax
14000c0d8:     	movq	$0x0, 0x7a8(%rbp)
14000c0e3:     	movq	%rax, 0x308(%rbp)
14000c0ea:     	movq	%rax, %rcx
14000c0ed:     	callq	0x142923c00 <.text+0x2922c00>
14000c0f2:     	cmpq	%rbx, %r14
14000c0f5:     	jae	0x1400112e1 <.text+0x102e1>
14000c0fb:     	testq	%rdi, %rdi
14000c0fe:     	je	0x14000c125 <.text+0xb125>
14000c100:     	movzbl	0x3628b02(%rip), %eax   # 0x143634c09
14000c107:     	movl	$0x4, %edx
14000c10c:     	movq	%r12, %rcx
14000c10f:     	callq	0x140613c10 <.text+0x612c10>
14000c114:     	testq	%rax, %rax
14000c117:     	je	0x1400118c5 <.text+0x108c5>
14000c11d:     	movq	%rax, %r15
14000c120:     	movq	%rdi, %r14
14000c123:     	jmp	0x14000c12e <.text+0xb12e>
14000c125:     	movl	$0x4, %r15d
14000c12b:     	xorl	%r14d, %r14d
14000c12e:     	movq	0x800(%rbp), %rax
14000c135:     	movq	0x7f0(%rbp), %rdx
14000c13c:     	leaq	(%rdx,%rax,4), %rdx
14000c140:     	movq	%r15, %rcx
14000c143:     	movq	%r12, %r8
14000c146:     	callq	0x142923c00 <.text+0x2922c00>
14000c14b:     	cmpq	$0x1, 0x780(%rbp)
14000c153:     	movb	$0x2, %al
14000c155:     	sbbb	$0x0, %al
14000c157:     	movaps	0x590(%rbp), %xmm0
14000c15e:     	movaps	0x5a0(%rbp), %xmm1
14000c165:     	movdqa	0x5b0(%rbp), %xmm2
14000c16d:     	movaps	%xmm0, 0x6a0(%rbp)
14000c174:     	movaps	%xmm1, 0x6b0(%rbp)
14000c17b:     	movdqa	%xmm2, 0x6c0(%rbp)
14000c183:     	movq	0x5c0(%rbp), %rcx
14000c18a:     	movq	%rcx, 0x6d0(%rbp)
14000c191:     	movq	0x7a8(%rbp), %rcx
14000c198:     	movq	%rcx, 0x6d8(%rbp)
14000c19f:     	movq	0x308(%rbp), %rcx
14000c1a6:     	movq	%rcx, 0x6e0(%rbp)
14000c1ad:     	movq	%rdi, 0x6e8(%rbp)
14000c1b4:     	movq	0x7a0(%rbp), %rcx
14000c1bb:     	movq	%rcx, 0x6f0(%rbp)
14000c1c2:     	movq	0x138(%rbp), %rcx
14000c1c9:     	movq	%rcx, 0x6f8(%rbp)
14000c1d0:     	movq	%rdi, 0x700(%rbp)
14000c1d7:     	movq	%r14, 0x708(%rbp)
14000c1de:     	movq	%r15, 0x710(%rbp)
14000c1e5:     	movq	%rdi, 0x718(%rbp)
14000c1ec:     	movq	0x438(%rbp), %rcx
14000c1f3:     	movq	%rcx, 0x720(%rbp)
14000c1fa:     	movw	$0x0, 0x734(%rbp)
14000c203:     	movq	0x758(%rbp), %rcx
14000c20a:     	movl	%ecx, 0x730(%rbp)
14000c210:     	movq	0x678(%rbp), %rcx
14000c217:     	movq	(%rcx), %rcx
14000c21a:     	movq	%rcx, 0x728(%rbp)
14000c221:     	movb	%al, 0x73b(%rbp)
14000c227:     	movl	$0x0, 0x736(%rbp)
14000c231:     	movb	$0x0, 0x73a(%rbp)
14000c238:     	movq	0x370(%rbp), %r14
14000c23f:     	movq	0x10(%r14), %rdi
14000c243:     	cmpq	(%r14), %rdi
14000c246:     	jne	0x14000c257 <.text+0xb257>
14000c248:     	movq	%r14, %rcx
14000c24b:     	leaq	0x291a856(%rip), %rdx   # 0x142926aa8
14000c252:     	callq	0x140563480 <.text+0x562480>
14000c257:     	leaq	(%rdi,%rdi,4), %rcx
14000c25b:     	shlq	$0x5, %rcx
14000c25f:     	addq	0x8(%r14), %rcx
14000c263:     	movl	$0xa0, %r8d
14000c269:     	leaq	0x6a0(%rbp), %rdx
14000c270:     	callq	0x142923c10 <.text+0x2922c10>
14000c275:     	incq	%rdi
14000c278:     	movq	%rdi, 0x10(%r14)
14000c27c:     	jmp	0x14000be50 <.text+0xae50>
14000c281:     	cmpb	$0x0, 0xe8(%r13)
14000c289:     	leaq	0x460(%rbp), %rcx
14000c290:     	jne	0x14000c2a0 <.text+0xb2a0>
14000c292:     	cmpb	$0x0, 0x108(%r13)
14000c29a:     	je	0x14000c678 <.text+0xb678>
14000c2a0:     	movq	0x690(%rbp), %rax
14000c2a7:     	movq	%rax, 0x6b0(%rbp)
14000c2ae:     	movaps	0x680(%rbp), %xmm0
14000c2b5:     	movaps	%xmm0, 0x6a0(%rbp)
14000c2bc:     	movb	$0x0, 0x816(%rbp)
14000c2c3:     	movb	$0x1, 0x815(%rbp)
14000c2ca:     	leaq	0x6a0(%rbp), %rdx
14000c2d1:     	movl	$0x1, %r8d
14000c2d7:     	callq	0x140153640 <.text+0x152640>
14000c2dc:     	xorl	%r14d, %r14d
14000c2df:     	cmpq	0x460(%rbp), %r14
14000c2e6:     	jno	0x14000c3c5 <.text+0xb3c5>
14000c2ec:     	xorl	%esi, %esi
14000c2ee:     	movq	0x7e0(%rbp), %r13
14000c2f5:     	movq	0x798(%rbp), %rdx
14000c2fc:     	movq	0x7f0(%rbp), %r15
14000c303:     	testq	%rdx, %rdx
14000c306:     	je	0x14000c31a <.text+0xb31a>
14000c308:     	shlq	$0x2, %rdx
14000c30c:     	movl	$0x4, %r8d
14000c312:     	movq	%r15, %rcx
14000c315:     	callq	0x140613c20 <.text+0x612c20>
14000c31a:     	testb	%sil, %sil
14000c31d:     	movq	0x7b0(%rbp), %r15
14000c324:     	je	0x14000c34c <.text+0xb34c>
14000c326:     	movq	0x680(%rbp), %rax
14000c32d:     	testq	%rax, %rax
14000c330:     	je	0x14000c34c <.text+0xb34c>
14000c332:     	movq	0x688(%rbp), %rcx
14000c339:     	shlq	$0x2, %rax
14000c33d:     	leaq	(%rax,%rax,2), %rdx
14000c341:     	movl	$0x4, %r8d
14000c347:     	callq	0x140613c20 <.text+0x612c20>
14000c34c:     	movq	0x650(%rbp), %rdx
14000c353:     	testq	%rdx, %rdx
14000c356:     	je	0x14000c36e <.text+0xb36e>
14000c358:     	movq	0x658(%rbp), %rcx
14000c35f:     	shlq	$0x3, %rdx
14000c363:     	movl	$0x4, %r8d
14000c369:     	callq	0x140613c20 <.text+0x612c20>
14000c36e:     	movq	0x548(%rbp), %rdx
14000c375:     	testq	%rdx, %rdx
14000c378:     	je	0x14000c390 <.text+0xb390>
14000c37a:     	movq	0x550(%rbp), %rcx
14000c381:     	shlq	$0x2, %rdx
14000c385:     	movl	$0x4, %r8d
14000c38b:     	callq	0x140613c20 <.text+0x612c20>
14000c390:     	movq	0x778(%rbp), %rax
14000c397:     	jmp	0x140009be5 <.text+0x8be5>
14000c39c:     	xorl	%r14d, %r14d
14000c39f:     	movq	0x7e0(%rbp), %r13
14000c3a6:     	movq	0x798(%rbp), %rdx
14000c3ad:     	movq	0x7f0(%rbp), %r15
14000c3b4:     	movb	$0x1, %sil
14000c3b7:     	testq	%rdx, %rdx
14000c3ba:     	jne	0x14000c308 <.text+0xb308>
14000c3c0:     	jmp	0x14000c31a <.text+0xb31a>
14000c3c5:     	movq	0x490(%rbp), %rax
14000c3cc:     	movq	%rax, 0x5c0(%rbp)
14000c3d3:     	movups	0x460(%rbp), %xmm0
14000c3da:     	movups	0x470(%rbp), %xmm1
14000c3e1:     	movdqu	0x480(%rbp), %xmm2
14000c3e9:     	movdqa	%xmm2, 0x5b0(%rbp)
14000c3f1:     	movaps	%xmm1, 0x5a0(%rbp)
14000c3f8:     	movaps	%xmm0, 0x590(%rbp)
14000c3ff:     	movq	0x548(%rbp), %rax
14000c406:     	movq	%rax, 0x800(%rbp)
14000c40d:     	movq	0x550(%rbp), %rcx
14000c414:     	movq	0x558(%rbp), %rsi
14000c41b:     	movq	0x650(%rbp), %rax
14000c422:     	movq	%rax, 0x780(%rbp)
14000c429:     	movq	0x658(%rbp), %rdx
14000c430:     	movq	0x660(%rbp), %r14
14000c437:     	movb	$0x1, %dil
14000c43a:     	movq	0x7e0(%rbp), %r13
14000c441:     	cmpb	$0x1, 0xe8(%r13)
14000c449:     	movb	$0x1, %r15b
14000c44c:     	movq	0x678(%rbp), %r12
14000c453:     	movq	%rcx, 0x760(%rbp)
14000c45a:     	movq	%rdx, 0x530(%rbp)
14000c461:     	je	0x14000c495 <.text+0xb495>
14000c463:     	movl	0x128(%r13), %r8d
14000c46a:     	movl	0x12c(%r13), %r9d
14000c471:     	movq	0x950(%rbp), %rcx
14000c478:     	movq	0x2e0(%rbp), %rdx
14000c47f:     	callq	0x1400275b0 <.text+0x265b0>
14000c484:     	movl	%eax, %r15d
14000c487:     	movq	0x760(%rbp), %rcx
14000c48e:     	movq	0x530(%rbp), %rdx
14000c495:     	cmpb	$0x1, 0x108(%r13)
14000c49d:     	je	0x14000c4d0 <.text+0xb4d0>
14000c49f:     	movl	0x128(%r13), %r9d
14000c4a6:     	movl	0x12c(%r13), %r8d
14000c4ad:     	movq	0x950(%rbp), %rcx
14000c4b4:     	movq	0x2e0(%rbp), %rdx
14000c4bb:     	callq	0x1400275b0 <.text+0x265b0>
14000c4c0:     	movl	%eax, %edi
14000c4c2:     	movq	0x760(%rbp), %rcx
14000c4c9:     	movq	0x530(%rbp), %rdx
14000c4d0:     	movq	0x5c0(%rbp), %rax
14000c4d7:     	movq	%rax, 0x6d0(%rbp)
14000c4de:     	movaps	0x590(%rbp), %xmm0
14000c4e5:     	movaps	0x5a0(%rbp), %xmm1
14000c4ec:     	movdqa	0x5b0(%rbp), %xmm2
14000c4f4:     	movdqa	%xmm2, 0x6c0(%rbp)
14000c4fc:     	movaps	%xmm1, 0x6b0(%rbp)
14000c503:     	movaps	%xmm0, 0x6a0(%rbp)
14000c50a:     	movq	0x780(%rbp), %rax
14000c511:     	movq	%rax, 0x6d8(%rbp)
14000c518:     	movq	%rdx, 0x6e0(%rbp)
14000c51f:     	movq	%r14, 0x6e8(%rbp)
14000c526:     	movq	0x800(%rbp), %rax
14000c52d:     	movq	%rax, 0x6f0(%rbp)
14000c534:     	movq	%rcx, 0x6f8(%rbp)
14000c53b:     	movq	%rsi, 0x700(%rbp)
14000c542:     	movq	0x798(%rbp), %rax
14000c549:     	movq	%rax, 0x708(%rbp)
14000c550:     	movq	0x7f0(%rbp), %rax
14000c557:     	movq	%rax, 0x710(%rbp)
14000c55e:     	movq	%rbx, 0x718(%rbp)
14000c565:     	movq	0x438(%rbp), %rax
14000c56c:     	movq	%rax, 0x720(%rbp)
14000c573:     	movq	(%r12), %rax
14000c577:     	movq	%rax, 0x728(%rbp)
14000c57e:     	movq	0x758(%rbp), %rax
14000c585:     	movl	%eax, 0x730(%rbp)
14000c58b:     	movb	%r15b, 0x734(%rbp)
14000c592:     	movb	%dil, 0x735(%rbp)
14000c599:     	movw	$0x0, 0x736(%rbp)
14000c5a2:     	movzbl	0x7a8(%rbp), %eax
14000c5a9:     	movb	%al, 0x738(%rbp)
14000c5af:     	movzbl	0x7a0(%rbp), %eax
14000c5b6:     	movb	%al, 0x739(%rbp)
14000c5bc:     	movw	$0x300, 0x73a(%rbp)     # imm = 0x300
14000c5c5:     	movq	0x670(%rbp), %rax
14000c5cc:     	movq	%rax, 0x740(%rbp)
14000c5d3:     	movq	0x7b0(%rbp), %rax
14000c5da:     	movq	%rax, 0x748(%rbp)
14000c5e1:     	movb	$0x0, 0x750(%rbp)
14000c5e8:     	movq	0x440(%rbp), %rax
14000c5ef:     	movq	0x458(%rbp), %rsi
14000c5f6:     	cmpq	%rax, %rsi
14000c5f9:     	movl	$0x0, %r14d
14000c5ff:     	jne	0x14000c622 <.text+0xb622>
14000c601:     	leaq	0x440(%rbp), %rcx
14000c608:     	leaq	0x291a3f1(%rip), %rdx   # 0x142926a00
14000c60f:     	callq	0x1405114a0 <.text+0x5104a0>
14000c614:     	movq	0x440(%rbp), %rax
14000c61b:     	movq	0x458(%rbp), %rsi
14000c622:     	movq	0x450(%rbp), %rcx
14000c629:     	addq	%rsi, %rcx
14000c62c:     	cmpq	%rax, %rcx
14000c62f:     	movl	$0x0, %edx
14000c634:     	cmovaeq	%rax, %rdx
14000c638:     	subq	%rdx, %rcx
14000c63b:     	imulq	$0xb8, %rcx, %rcx
14000c642:     	addq	0x448(%rbp), %rcx
14000c649:     	movl	$0xb8, %r8d
14000c64f:     	leaq	0x6a0(%rbp), %rdx
14000c656:     	callq	0x142923c10 <.text+0x2922c10>
14000c65b:     	incq	%rsi
14000c65e:     	movq	%rsi, 0x458(%rbp)
14000c665:     	movq	0x7b0(%rbp), %r15
14000c66c:     	movq	0x778(%rbp), %rax
14000c673:     	jmp	0x140009be5 <.text+0x8be5>
14000c678:     	movq	0x688(%rbp), %rsi
14000c67f:     	movq	0x690(%rbp), %rax
14000c686:     	testq	%rax, %rax
14000c689:     	sete	%dl
14000c68c:     	leaq	(%rax,%rax,2), %rax
14000c690:     	leaq	(%rsi,%rax,4), %rdi
14000c694:     	cmpq	$0xc, %rdi
14000c698:     	sete	%al
14000c69b:     	orb	%dl, %al
14000c69d:     	jne	0x14000cb45 <.text+0xbb45>
14000c6a3:     	movq	0x660(%rbp), %rdx
14000c6aa:     	cmpq	$0x1, %rdx
14000c6ae:     	jbe	0x140011940 <.text+0x10940>
14000c6b4:     	movss	0x8(%rsi), %xmm6
14000c6b9:     	movss	0x29191af(%rip), %xmm0  # 0x142925870
14000c6c1:     	mulss	%xmm0, %xmm6
14000c6c5:     	movsd	(%rsi), %xmm8
14000c6ca:     	movsd	-0xc(%rdi), %xmm7
14000c6cf:     	movss	-0x4(%rdi), %xmm9
14000c6d5:     	mulss	%xmm0, %xmm9
14000c6da:     	movq	0x658(%rbp), %rax
14000c6e1:     	movss	0x8(%rax), %xmm0
14000c6e6:     	movss	0xc(%rax), %xmm1
14000c6eb:     	xorps	0x29191fe(%rip), %xmm1  # 0x1429258f0
14000c6f2:     	movss	%xmm1, 0x590(%rbp)
14000c6fa:     	movss	%xmm0, 0x594(%rbp)
14000c702:     	movb	$0x1, 0x816(%rbp)
14000c709:     	movb	$0x1, 0x815(%rbp)
14000c710:     	leaq	0x6a0(%rbp), %rcx
14000c717:     	leaq	0x590(%rbp), %rdx
14000c71e:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
14000c723:     	movq	0x558(%rbp), %rdx
14000c72a:     	cmpq	$0x1, %rdx
14000c72e:     	jbe	0x140011986 <.text+0x10986>
14000c734:     	movq	0x550(%rbp), %rax
14000c73b:     	movss	0x4(%rax), %xmm12
14000c741:     	mulss	0x2919126(%rip), %xmm12 # 0x142925870
14000c74a:     	addss	0x29191ad(%rip), %xmm12 # 0x142925900
14000c753:     	movq	0x660(%rbp), %rdx
14000c75a:     	cmpq	$0x1, %rdx
14000c75e:     	jbe	0x1400119aa <.text+0x109aa>
14000c764:     	movsd	0x6a0(%rbp), %xmm10
14000c76d:     	movss	0x6a8(%rbp), %xmm11
14000c776:     	mulss	%xmm12, %xmm11
14000c77b:     	movq	0x658(%rbp), %rax
14000c782:     	movss	0x8(%rax), %xmm0
14000c787:     	movss	0xc(%rax), %xmm1
14000c78c:     	xorps	0x291915d(%rip), %xmm1  # 0x1429258f0
14000c793:     	movss	%xmm1, 0x590(%rbp)
14000c79b:     	movss	%xmm0, 0x594(%rbp)
14000c7a3:     	movb	$0x1, 0x816(%rbp)
14000c7aa:     	movb	$0x1, 0x815(%rbp)
14000c7b1:     	leaq	0x6a0(%rbp), %rcx
14000c7b8:     	leaq	0x590(%rbp), %rdx
14000c7bf:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
14000c7c4:     	movq	0x558(%rbp), %rdx
14000c7cb:     	cmpq	$0x1, %rdx
14000c7cf:     	jbe	0x1400119cb <.text+0x109cb>
14000c7d5:     	movq	0x550(%rbp), %rax
14000c7dc:     	movss	0x4(%rax), %xmm14
14000c7e2:     	movsd	0x6a0(%rbp), %xmm13
14000c7eb:     	movss	0x6a8(%rbp), %xmm15
14000c7f4:     	movzbl	0x362840e(%rip), %eax   # 0x143634c09
14000c7fb:     	movl	$0x24, %ecx
14000c800:     	movl	$0x4, %edx
14000c805:     	callq	0x140613c10 <.text+0x612c10>
14000c80a:     	testq	%rax, %rax
14000c80d:     	leaq	0x460(%rbp), %rcx
14000c814:     	je	0x140011964 <.text+0x10964>
14000c81a:     	movaps	0x29190bf(%rip), %xmm0  # 0x1429258e0
14000c821:     	mulps	%xmm0, %xmm8
14000c825:     	mulps	%xmm0, %xmm7
14000c828:     	addps	%xmm8, %xmm7
14000c82c:     	addss	%xmm9, %xmm6
14000c831:     	movsldup	%xmm12, %xmm0           # xmm0 = xmm12[0,0,2,2]
14000c836:     	mulps	%xmm0, %xmm10
14000c83a:     	addps	%xmm7, %xmm10
14000c83e:     	addss	%xmm6, %xmm11
14000c843:     	mulss	0x2919024(%rip), %xmm14 # 0x142925870
14000c84c:     	addss	0x29190ab(%rip), %xmm14 # 0x142925900
14000c855:     	mulss	%xmm14, %xmm15
14000c85a:     	movlps	%xmm10, (%rax)
14000c85e:     	movss	%xmm11, 0x8(%rax)
14000c864:     	movlps	%xmm7, 0xc(%rax)
14000c868:     	movss	%xmm6, 0x14(%rax)
14000c86d:     	subss	%xmm15, %xmm6
14000c872:     	movsldup	%xmm14, %xmm0           # xmm0 = xmm14[0,0,2,2]
14000c877:     	mulps	%xmm0, %xmm13
14000c87b:     	subps	%xmm13, %xmm7
14000c87f:     	movlps	%xmm7, 0x18(%rax)
14000c883:     	movss	%xmm6, 0x20(%rax)
14000c888:     	movss	(%rsi), %xmm6
14000c88c:     	subss	-0xc(%rdi), %xmm6
14000c891:     	movss	0x8(%rsi), %xmm7
14000c896:     	subss	-0x4(%rdi), %xmm7
14000c89b:     	movq	$0x3, 0x6a0(%rbp)
14000c8a6:     	movq	%rax, 0x6a8(%rbp)
14000c8ad:     	movq	$0x3, 0x6b0(%rbp)
14000c8b8:     	movb	$0x1, 0x816(%rbp)
14000c8bf:     	movb	$0x1, 0x815(%rbp)
14000c8c6:     	leaq	0x6a0(%rbp), %rdx
14000c8cd:     	movl	$0x1, %r8d
14000c8d3:     	callq	0x140153640 <.text+0x152640>
14000c8d8:     	xorl	%eax, %eax
14000c8da:     	cmpq	0x460(%rbp), %rax
14000c8e1:     	jo	0x14000cb3e <.text+0xbb3e>
14000c8e7:     	movq	0x490(%rbp), %rax
14000c8ee:     	movq	%rax, 0x6d0(%rbp)
14000c8f5:     	movups	0x460(%rbp), %xmm0
14000c8fc:     	movups	0x470(%rbp), %xmm1
14000c903:     	movups	0x480(%rbp), %xmm2
14000c90a:     	movaps	%xmm2, 0x6c0(%rbp)
14000c911:     	movaps	%xmm1, 0x6b0(%rbp)
14000c918:     	movaps	%xmm0, 0x6a0(%rbp)
14000c91f:     	movl	$0x3, %r8d
14000c925:     	leaq	0x500(%rbp), %rcx
14000c92c:     	movss	0x2918fd4(%rip), %xmm1  # 0x142925908
14000c934:     	leaq	0x291a035(%rip), %r9    # 0x142926970
14000c93b:     	callq	0x1402b6ad0 <.text+0x2b5ad0>
14000c940:     	testq	%rbx, %rbx
14000c943:     	movss	0x2918fb8(%rip), %xmm8  # 0x142925904
14000c94c:     	je	0x140011a30 <.text+0x10a30>
14000c952:     	movq	0x7f0(%rbp), %rax
14000c959:     	movss	(%rax), %xmm1
14000c95d:     	movl	$0x3, %r8d
14000c963:     	leaq	0x3e0(%rbp), %rcx
14000c96a:     	leaq	0x291a02f(%rip), %r9    # 0x1429269a0
14000c971:     	callq	0x1402b6ad0 <.text+0x2b5ad0>
14000c976:     	movq	0x678(%rbp), %rsi
14000c97d:     	movaps	%xmm7, %xmm1
14000c980:     	xorps	0x2918f69(%rip), %xmm1  # 0x1429258f0
14000c987:     	mulss	%xmm7, %xmm7
14000c98b:     	movaps	%xmm6, %xmm0
14000c98e:     	mulss	%xmm6, %xmm0
14000c992:     	addss	%xmm7, %xmm0
14000c996:     	sqrtss	%xmm0, %xmm0
14000c99a:     	movaps	%xmm8, %xmm2
14000c99e:     	divss	%xmm0, %xmm2
14000c9a2:     	mulss	%xmm2, %xmm1
14000c9a6:     	mulss	%xmm6, %xmm2
14000c9aa:     	leaq	0x291a007(%rip), %rax   # 0x1429269b8
14000c9b1:     	movq	%rax, 0x20(%rsp)
14000c9b6:     	movl	$0x3, %r9d
14000c9bc:     	leaq	0x380(%rbp), %rcx
14000c9c3:     	callq	0x1402b6c60 <.text+0x2b5c60>
14000c9c8:     	movq	0x6d0(%rbp), %rax
14000c9cf:     	movq	%rax, 0x5c0(%rbp)
14000c9d6:     	movaps	0x6a0(%rbp), %xmm0
14000c9dd:     	movaps	0x6b0(%rbp), %xmm1
14000c9e4:     	movaps	0x6c0(%rbp), %xmm2
14000c9eb:     	movaps	%xmm2, 0x5b0(%rbp)
14000c9f2:     	movaps	%xmm1, 0x5a0(%rbp)
14000c9f9:     	movaps	%xmm0, 0x590(%rbp)
14000ca00:     	movq	0x390(%rbp), %rax
14000ca07:     	leaq	0x598(%rbp), %rcx
14000ca0e:     	movq	%rax, 0x40(%rcx)
14000ca12:     	movups	0x380(%rbp), %xmm0
14000ca19:     	movups	%xmm0, 0x30(%rcx)
14000ca1d:     	movq	0x510(%rbp), %rax
14000ca24:     	movq	%rax, 0x58(%rcx)
14000ca28:     	movups	0x500(%rbp), %xmm0
14000ca2f:     	movups	%xmm0, 0x48(%rcx)
14000ca33:     	movq	0x3f0(%rbp), %rax
14000ca3a:     	movq	%rax, 0x70(%rcx)
14000ca3e:     	movups	0x3e0(%rbp), %xmm0
14000ca45:     	movups	%xmm0, 0x60(%rcx)
14000ca49:     	movaps	0x590(%rbp), %xmm0
14000ca50:     	movaps	0x5a0(%rbp), %xmm1
14000ca57:     	movdqa	0x5b0(%rbp), %xmm2
14000ca5f:     	movapd	0x5c0(%rbp), %xmm3
14000ca67:     	movaps	%xmm0, 0x6a0(%rbp)
14000ca6e:     	movaps	%xmm1, 0x6b0(%rbp)
14000ca75:     	movdqa	%xmm2, 0x6c0(%rbp)
14000ca7d:     	movapd	%xmm3, 0x6d0(%rbp)
14000ca85:     	movaps	0x5d0(%rbp), %xmm0
14000ca8c:     	movaps	%xmm0, 0x6e0(%rbp)
14000ca93:     	movaps	0x5e0(%rbp), %xmm0
14000ca9a:     	movaps	%xmm0, 0x6f0(%rbp)
14000caa1:     	movaps	0x5f0(%rbp), %xmm0
14000caa8:     	movaps	%xmm0, 0x700(%rbp)
14000caaf:     	movaps	0x600(%rbp), %xmm0
14000cab6:     	movaps	%xmm0, 0x710(%rbp)
14000cabd:     	movq	0x438(%rbp), %rax
14000cac4:     	movq	%rax, 0x720(%rbp)
14000cacb:     	movq	(%rsi), %rax
14000cace:     	movq	%rax, 0x728(%rbp)
14000cad5:     	movq	0x758(%rbp), %rax
14000cadc:     	movl	%eax, 0x730(%rbp)
14000cae2:     	movl	$0x0, 0x734(%rbp)
14000caec:     	movzbl	0x7a8(%rbp), %eax
14000caf3:     	movb	%al, 0x738(%rbp)
14000caf9:     	movzbl	0x7a0(%rbp), %eax
14000cb00:     	movb	%al, 0x739(%rbp)
14000cb06:     	movw	$0x301, 0x73a(%rbp)     # imm = 0x301
14000cb0f:     	movb	$0x2, 0x750(%rbp)
14000cb16:     	movb	$0x1, 0x816(%rbp)
14000cb1d:     	movb	$0x1, 0x815(%rbp)
14000cb24:     	leaq	0x440(%rbp), %rcx
14000cb2b:     	leaq	0x6a0(%rbp), %rdx
14000cb32:     	leaq	0x2919e97(%rip), %r8    # 0x1429269d0
14000cb39:     	callq	0x140511a10 <.text+0x510a10>
14000cb3e:     	leaq	0x460(%rbp), %rcx
14000cb45:     	movq	0x690(%rbp), %rax
14000cb4c:     	movq	%rax, 0x6b0(%rbp)
14000cb53:     	movaps	0x680(%rbp), %xmm0
14000cb5a:     	movaps	%xmm0, 0x6a0(%rbp)
14000cb61:     	movb	$0x0, 0x816(%rbp)
14000cb68:     	movb	$0x1, 0x815(%rbp)
14000cb6f:     	leaq	0x6a0(%rbp), %rdx
14000cb76:     	movl	$0x1, %r8d
14000cb7c:     	callq	0x140153640 <.text+0x152640>
14000cb81:     	xorl	%r14d, %r14d
14000cb84:     	cmpq	0x460(%rbp), %r14
14000cb8b:     	jo	0x14000c2ec <.text+0xb2ec>
14000cb91:     	movq	0x490(%rbp), %rax
14000cb98:     	movq	%rax, 0x5c0(%rbp)
14000cb9f:     	movups	0x460(%rbp), %xmm0
14000cba6:     	movups	0x470(%rbp), %xmm1
14000cbad:     	movups	0x480(%rbp), %xmm2
14000cbb4:     	movaps	%xmm2, 0x5b0(%rbp)
14000cbbb:     	movaps	%xmm1, 0x5a0(%rbp)
14000cbc2:     	movaps	%xmm0, 0x590(%rbp)
14000cbc9:     	movq	0x558(%rbp), %rax
14000cbd0:     	leaq	0x598(%rbp), %rcx
14000cbd7:     	movq	%rax, 0x58(%rcx)
14000cbdb:     	movups	0x548(%rbp), %xmm0
14000cbe2:     	movups	%xmm0, 0x48(%rcx)
14000cbe6:     	movq	0x660(%rbp), %rax
14000cbed:     	movq	%rax, 0x40(%rcx)
14000cbf1:     	movaps	0x650(%rbp), %xmm0
14000cbf8:     	movups	%xmm0, 0x30(%rcx)
14000cbfc:     	movaps	0x590(%rbp), %xmm0
14000cc03:     	movaps	0x5a0(%rbp), %xmm1
14000cc0a:     	movdqa	0x5b0(%rbp), %xmm2
14000cc12:     	movq	0x5c0(%rbp), %rax
14000cc19:     	movq	0x5c8(%rbp), %rcx
14000cc20:     	movaps	%xmm0, 0x6a0(%rbp)
14000cc27:     	movaps	%xmm1, 0x6b0(%rbp)
14000cc2e:     	movdqa	%xmm2, 0x6c0(%rbp)
14000cc36:     	movq	%rax, 0x6d0(%rbp)
14000cc3d:     	movq	%rcx, 0x6d8(%rbp)
14000cc44:     	movaps	0x5d0(%rbp), %xmm0
14000cc4b:     	movaps	%xmm0, 0x6e0(%rbp)
14000cc52:     	movaps	0x5e0(%rbp), %xmm0
14000cc59:     	movaps	%xmm0, 0x6f0(%rbp)
14000cc60:     	movq	0x5f0(%rbp), %rax
14000cc67:     	movq	%rax, 0x700(%rbp)
14000cc6e:     	movq	0x798(%rbp), %rax
14000cc75:     	movq	%rax, 0x708(%rbp)
14000cc7c:     	movq	0x7f0(%rbp), %rax
14000cc83:     	movq	%rax, 0x710(%rbp)
14000cc8a:     	movq	%rbx, 0x718(%rbp)
14000cc91:     	movq	0x438(%rbp), %rax
14000cc98:     	movq	%rax, 0x720(%rbp)
14000cc9f:     	movq	0x678(%rbp), %rax
14000cca6:     	movq	(%rax), %rax
14000cca9:     	movq	%rax, 0x728(%rbp)
14000ccb0:     	movq	0x758(%rbp), %rax
14000ccb7:     	movl	%eax, 0x730(%rbp)
14000ccbd:     	movl	$0x0, 0x734(%rbp)
14000ccc7:     	movzbl	0x7a8(%rbp), %eax
14000ccce:     	movb	%al, 0x738(%rbp)
14000ccd4:     	movzbl	0x7a0(%rbp), %eax
14000ccdb:     	movb	%al, 0x739(%rbp)
14000cce1:     	movw	$0x301, 0x73a(%rbp)     # imm = 0x301
14000ccea:     	movb	$0x2, 0x750(%rbp)
14000ccf1:     	movb	$0x0, 0x816(%rbp)
14000ccf8:     	movb	$0x0, 0x815(%rbp)
14000ccff:     	leaq	0x440(%rbp), %rcx
14000cd06:     	leaq	0x6a0(%rbp), %rdx
14000cd0d:     	leaq	0x2919cd4(%rip), %r8    # 0x1429269e8
14000cd14:     	callq	0x140511a10 <.text+0x510a10>
14000cd19:     	xorl	%r14d, %r14d
14000cd1c:     	movq	0x7e0(%rbp), %r13
14000cd23:     	movq	0x7b0(%rbp), %r15
14000cd2a:     	movq	0x778(%rbp), %rax
14000cd31:     	jmp	0x140009be5 <.text+0x8be5>
14000cd36:     	cmpq	%rcx, %r14
14000cd39:     	je	0x14000cd9a <.text+0xbd9a>
14000cd3b:     	movss	(%r15,%rax,4), %xmm0
14000cd41:     	movaps	%xmm8, %xmm1
14000cd45:     	subss	%xmm0, %xmm1
14000cd49:     	movsd	0x540(%rbp), %xmm2
14000cd51:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14000cd54:     	mulps	%xmm2, %xmm1
14000cd57:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14000cd5b:     	addss	%xmm1, %xmm0
14000cd5f:     	movss	%xmm0, (%r10,%rax,4)
14000cd65:     	movss	0x4(%r15,%rax,4), %xmm0
14000cd6c:     	movaps	%xmm8, %xmm1
14000cd70:     	subss	%xmm0, %xmm1
14000cd74:     	movsd	0x540(%rbp), %xmm2
14000cd7c:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14000cd7f:     	mulps	%xmm2, %xmm1
14000cd82:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14000cd86:     	addss	%xmm1, %xmm0
14000cd8a:     	movss	%xmm0, 0x4(%r10,%rax,4)
14000cd91:     	leaq	0x2(%rax), %rcx
14000cd95:     	movq	%rcx, %rax
14000cd98:     	jmp	0x14000cd36 <.text+0xbd36>
14000cd9a:     	movq	%r10, 0x7f0(%rbp)
14000cda1:     	movq	0x5a0(%rbp), %r15
14000cda8:     	movq	0x5b8(%rbp), %rax
14000cdaf:     	cmpq	%r15, %rax
14000cdb2:     	cmovbq	%rax, %r15
14000cdb6:     	leaq	(,%r15,4), %rdi
14000cdbe:     	movq	%r15, %rax
14000cdc1:     	shrq	$0x3e, %rax
14000cdc5:     	jne	0x1400117aa <.text+0x107aa>
14000cdcb:     	movl	$0x0, %esi
14000cdd0:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14000cdda:     	cmpq	%rax, %rdi
14000cddd:     	ja	0x1400117ac <.text+0x107ac>
14000cde3:     	movq	0x5b0(%rbp), %r12
14000cdea:     	testq	%rdi, %rdi
14000cded:     	je	0x14000ce25 <.text+0xbe25>
14000cdef:     	movzbl	0x3627e13(%rip), %eax   # 0x143634c09
14000cdf6:     	movl	$0x4, %esi
14000cdfb:     	movl	$0x4, %edx
14000ce00:     	movq	%rdi, %rcx
14000ce03:     	callq	0x140613c10 <.text+0x612c10>
14000ce08:     	testq	%rax, %rax
14000ce0b:     	je	0x1400117ac <.text+0x107ac>
14000ce11:     	movq	%r15, %rcx
14000ce14:     	testq	%r15, %r15
14000ce17:     	movq	0x7f0(%rbp), %r9
14000ce1e:     	jne	0x14000ce3c <.text+0xbe3c>
14000ce20:     	jmp	0x14000cf3d <.text+0xbf3d>
14000ce25:     	movl	$0x4, %eax
14000ce2a:     	xorl	%ecx, %ecx
14000ce2c:     	testq	%r15, %r15
14000ce2f:     	movq	0x7f0(%rbp), %r9
14000ce36:     	je	0x14000cf3d <.text+0xbf3d>
14000ce3c:     	cmpq	$0x8, %r15
14000ce40:     	jb	0x14000ceb9 <.text+0xbeb9>
14000ce42:     	movq	%rax, %r8
14000ce45:     	subq	%r12, %r8
14000ce48:     	movl	$0x0, %edx
14000ce4d:     	cmpq	$0x20, %r8
14000ce51:     	jb	0x14000cebb <.text+0xbebb>
14000ce53:     	movq	%r15, %rdx
14000ce56:     	movabsq	$0x3ffffffffffffff8, %r8 # imm = 0x3FFFFFFFFFFFFFF8
14000ce60:     	andq	%r8, %rdx
14000ce63:     	xorl	%r8d, %r8d
14000ce66:     	xorps	%xmm4, %xmm4
14000ce69:     	nopl	(%rax)
14000ce70:     	movups	(%r12,%r8,4), %xmm0
14000ce75:     	movups	0x10(%r12,%r8,4), %xmm1
14000ce7b:     	movaps	%xmm10, %xmm2
14000ce7f:     	subps	%xmm0, %xmm2
14000ce82:     	movaps	%xmm10, %xmm3
14000ce86:     	subps	%xmm1, %xmm3
14000ce89:     	mulps	%xmm4, %xmm2
14000ce8c:     	mulps	%xmm4, %xmm3
14000ce8f:     	mulps	%xmm4, %xmm0
14000ce92:     	addps	%xmm2, %xmm0
14000ce95:     	mulps	%xmm4, %xmm1
14000ce98:     	addps	%xmm3, %xmm1
14000ce9b:     	movups	%xmm0, (%rax,%r8,4)
14000cea0:     	movups	%xmm1, 0x10(%rax,%r8,4)
14000cea6:     	addq	$0x8, %r8
14000ceaa:     	cmpq	%r8, %rdx
14000cead:     	jne	0x14000ce70 <.text+0xbe70>
14000ceaf:     	cmpq	%rdx, %r15
14000ceb2:     	jne	0x14000cebb <.text+0xbebb>
14000ceb4:     	jmp	0x14000cf3d <.text+0xbf3d>
14000ceb9:     	xorl	%edx, %edx
14000cebb:     	movq	%rdx, %r8
14000cebe:     	orq	$0x1, %r8
14000cec2:     	testb	$0x1, %r15b
14000cec6:     	je	0x14000ceeb <.text+0xbeeb>
14000cec8:     	movss	(%r12,%rdx,4), %xmm0
14000cece:     	movaps	%xmm8, %xmm1
14000ced2:     	subss	%xmm0, %xmm1
14000ced6:     	mulss	%xmm9, %xmm1
14000cedb:     	mulss	%xmm9, %xmm0
14000cee0:     	addss	%xmm1, %xmm0
14000cee4:     	movss	%xmm0, (%rax,%rdx,4)
14000cee9:     	jmp	0x14000cf38 <.text+0xbf38>
14000ceeb:     	cmpq	%r8, %r15
14000ceee:     	je	0x14000cf3d <.text+0xbf3d>
14000cef0:     	movss	(%r12,%rdx,4), %xmm0
14000cef6:     	movaps	%xmm8, %xmm1
14000cefa:     	subss	%xmm0, %xmm1
14000cefe:     	mulss	%xmm9, %xmm1
14000cf03:     	mulss	%xmm9, %xmm0
14000cf08:     	addss	%xmm1, %xmm0
14000cf0c:     	movss	%xmm0, (%rax,%rdx,4)
14000cf11:     	movss	0x4(%r12,%rdx,4), %xmm0
14000cf18:     	movaps	%xmm8, %xmm1
14000cf1c:     	subss	%xmm0, %xmm1
14000cf20:     	mulss	%xmm9, %xmm1
14000cf25:     	mulss	%xmm9, %xmm0
14000cf2a:     	addss	%xmm1, %xmm0
14000cf2e:     	movss	%xmm0, 0x4(%rax,%rdx,4)
14000cf34:     	leaq	0x2(%rdx), %r8
14000cf38:     	movq	%r8, %rdx
14000cf3b:     	jmp	0x14000ceeb <.text+0xbeeb>
14000cf3d:     	movq	0x258(%rbp), %rdx
14000cf44:     	movl	0x424(%rbp), %r8d
14000cf4b:     	movl	%r8d, (%rdx)
14000cf4e:     	movq	0x5c0(%rbp), %rdx
14000cf55:     	movq	%rdx, 0x6d0(%rbp)
14000cf5c:     	movaps	0x590(%rbp), %xmm0
14000cf63:     	movaps	0x5a0(%rbp), %xmm1
14000cf6a:     	movaps	0x5b0(%rbp), %xmm2
14000cf71:     	movaps	%xmm2, 0x6c0(%rbp)
14000cf78:     	movaps	%xmm1, 0x6b0(%rbp)
14000cf7f:     	movaps	%xmm0, 0x6a0(%rbp)
14000cf86:     	movq	0x800(%rbp), %rdx
14000cf8d:     	movq	%rdx, 0x6d8(%rbp)
14000cf94:     	movq	0x780(%rbp), %rdx
14000cf9b:     	movq	%rdx, 0x6e0(%rbp)
14000cfa2:     	movq	%rbx, 0x6e8(%rbp)
14000cfa9:     	movq	0x760(%rbp), %rdx
14000cfb0:     	movq	%rdx, 0x708(%rbp)
14000cfb7:     	movq	%r9, 0x710(%rbp)
14000cfbe:     	movq	%r14, 0x718(%rbp)
14000cfc5:     	movq	%rcx, 0x6f0(%rbp)
14000cfcc:     	movq	%rax, 0x6f8(%rbp)
14000cfd3:     	movq	%r15, 0x700(%rbp)
14000cfda:     	movq	0x758(%rbp), %rax
14000cfe1:     	movl	%eax, 0x740(%rbp)
14000cfe7:     	movq	0x678(%rbp), %rax
14000cfee:     	movq	(%rax), %rax
14000cff1:     	movq	%rax, 0x738(%rbp)
14000cff8:     	movw	$0x0, 0x744(%rbp)
14000d001:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14000d00b:     	movq	%rax, 0x720(%rbp)
14000d012:     	movq	0x10(%r13), %rsi
14000d016:     	cmpq	(%r13), %rsi
14000d01a:     	jne	0x14000d02b <.text+0xc02b>
14000d01c:     	movq	%r13, %rcx
14000d01f:     	leaq	0x2919802(%rip), %rdx   # 0x142926828
14000d026:     	callq	0x1405636c0 <.text+0x5626c0>
14000d02b:     	imulq	$0xa8, %rsi, %rcx
14000d032:     	addq	0x8(%r13), %rcx
14000d036:     	movl	$0xa8, %r8d
14000d03c:     	leaq	0x6a0(%rbp), %rdx
14000d043:     	callq	0x142923c10 <.text+0x2922c10>
14000d048:     	incq	%rsi
14000d04b:     	movq	%rsi, 0x10(%r13)
14000d04f:     	movq	0x200(%rbp), %rax
14000d056:     	movb	$0x1, 0x817(%rbp)
14000d05d:     	movq	%rax, 0x48(%rsp)
14000d062:     	movq	0x648(%rbp), %rax
14000d069:     	movq	%rax, 0x40(%rsp)
14000d06e:     	leaq	0x278(%rbp), %rax
14000d075:     	movq	%rax, 0x38(%rsp)
14000d07a:     	leaq	0x540(%rbp), %rax
14000d081:     	movq	%rax, 0x20(%rsp)
14000d086:     	movq	$0x1, 0x30(%rsp)
14000d08f:     	movq	$0x2, 0x28(%rsp)
14000d098:     	movl	$0x2, %edx
14000d09d:     	movl	$0x2, %r9d
14000d0a3:     	leaq	0x650(%rbp), %rcx
14000d0aa:     	leaq	0x310(%rbp), %r8
14000d0b1:     	callq	0x1407de8b0 <.text+0x7dd8b0>
14000d0b6:     	movb	$0x1, 0x817(%rbp)
14000d0bd:     	leaq	0x428(%rbp), %rax
14000d0c4:     	movq	%rax, 0x38(%rsp)
14000d0c9:     	movl	0x528(%rbp), %eax
14000d0cf:     	movl	%eax, 0x30(%rsp)
14000d0d3:     	leaq	0x7ef(%rbp), %rax
14000d0da:     	movq	%rax, 0x28(%rsp)
14000d0df:     	movq	$0x2, 0x20(%rsp)
14000d0e8:     	movl	$0x2, %r8d
14000d0ee:     	leaq	0x198(%rbp), %rcx
14000d0f5:     	leaq	0x650(%rbp), %rdx
14000d0fc:     	leaq	0x540(%rbp), %r9
14000d103:     	callq	0x1407db3e0 <.text+0x7da3e0>
14000d108:     	movzbl	0x7ef(%rbp), %eax
14000d10f:     	cmpl	$0x1, %eax
14000d112:     	movl	$0x0, %r14d
14000d118:     	jbe	0x140009bd0 <.text+0x8bd0>
14000d11e:     	cmpl	$0x3, %eax
14000d121:     	je	0x14000d222 <.text+0xc222>
14000d127:     	cmpl	$0x4, %eax
14000d12a:     	movss	0x29187d2(%rip), %xmm3  # 0x142925904
14000d132:     	jne	0x14000d259 <.text+0xc259>
14000d138:     	movaps	%xmm13, %xmm0
14000d13c:     	movlhps	%xmm12, %xmm0           # xmm0 = xmm0[0],xmm12[0]
14000d140:     	shufps	$0xe2, %xmm12, %xmm0    # xmm0 = xmm0[2,0],xmm12[2,3]
14000d145:     	movaps	%xmm15, %xmm1
14000d149:     	movlhps	%xmm14, %xmm1           # xmm1 = xmm1[0],xmm14[0]
14000d14d:     	shufps	$0xe2, %xmm14, %xmm1    # xmm1 = xmm1[2,0],xmm14[2,3]
14000d152:     	subps	%xmm1, %xmm0
14000d155:     	movaps	%xmm0, %xmm1
14000d158:     	mulps	%xmm0, %xmm1
14000d15b:     	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14000d15f:     	addss	%xmm1, %xmm2
14000d163:     	xorps	%xmm1, %xmm1
14000d166:     	sqrtss	%xmm2, %xmm1
14000d16a:     	movaps	%xmm3, %xmm2
14000d16d:     	divss	%xmm1, %xmm2
14000d171:     	movd	%xmm2, %eax
14000d175:     	testl	%eax, %eax
14000d177:     	setns	%cl
14000d17a:     	movl	%eax, %edx
14000d17c:     	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
14000d182:     	addl	$0xff800000, %edx       # imm = 0xFF800000
14000d188:     	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
14000d18e:     	setb	%dl
14000d191:     	andb	%cl, %dl
14000d193:     	decl	%eax
14000d195:     	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
14000d19a:     	setb	%al
14000d19d:     	orb	%dl, %al
14000d19f:     	movsldup	%xmm2, %xmm1            # xmm1 = xmm2[0,0,2,2]
14000d1a3:     	mulps	%xmm0, %xmm1
14000d1a6:     	movzbl	%al, %eax
14000d1a9:     	movd	%eax, %xmm0
14000d1ad:     	pshufd	$0x50, %xmm0, %xmm11    # xmm11 = xmm0[0,0,1,1]
14000d1b3:     	pslld	$0x1f, %xmm11
14000d1b9:     	psrad	$0x1f, %xmm11
14000d1bf:     	pand	%xmm1, %xmm11
14000d1c4:     	movq	%xmm11, 0x6a0(%rbp)
14000d1cd:     	movb	$0x1, 0x817(%rbp)
14000d1d4:     	leaq	0x68(%rbp), %rcx
14000d1d8:     	leaq	0x6a0(%rbp), %rdx
14000d1df:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
14000d1e4:     	movsd	0x68(%rbp), %xmm0
14000d1e9:     	mulps	0x2918750(%rip), %xmm0  # 0x142925940
14000d1f0:     	movss	0x70(%rbp), %xmm1
14000d1f5:     	mulss	0x2918753(%rip), %xmm1  # 0x142925950
14000d1fd:     	subss	%xmm1, %xmm15
14000d202:     	subps	%xmm0, %xmm14
14000d206:     	addps	%xmm0, %xmm12
14000d20a:     	addss	%xmm1, %xmm13
14000d20f:     	movq	0x540(%rbp), %rbx
14000d216:     	leaq	0x460(%rbp), %rcx
14000d21d:     	jmp	0x14000d4a2 <.text+0xc4a2>
14000d222:     	movaps	%xmm14, %xmm0
14000d226:     	cmpltps	%xmm12, %xmm0
14000d22b:     	pextrw	$0x2, %xmm0, %eax
14000d230:     	testb	$0x1, %al
14000d232:     	movss	0x29186ca(%rip), %xmm3  # 0x142925904
14000d23a:     	je	0x14000d26b <.text+0xc26b>
14000d23c:     	movl	0x540(%rbp), %esi
14000d242:     	movaps	%xmm6, %xmm1
14000d245:     	mulps	%xmm6, %xmm1
14000d248:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14000d24c:     	addss	%xmm1, %xmm0
14000d250:     	movb	$0x1, %dil
14000d253:     	movaps	%xmm6, %xmm11
14000d257:     	jmp	0x14000d283 <.text+0xc283>
14000d259:     	movaps	%xmm12, %xmm0
14000d25d:     	cmpltps	%xmm14, %xmm0
14000d262:     	pextrw	$0x2, %xmm0, %eax
14000d267:     	testb	$0x1, %al
14000d269:     	jne	0x14000d23c <.text+0xc23c>
14000d26b:     	movl	0x544(%rbp), %esi
14000d271:     	movaps	%xmm11, %xmm1
14000d275:     	mulps	%xmm11, %xmm1
14000d279:     	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14000d27d:     	addss	%xmm1, %xmm0
14000d281:     	xorl	%edi, %edi
14000d283:     	sqrtss	%xmm0, %xmm0
14000d287:     	movaps	%xmm3, %xmm1
14000d28a:     	divss	%xmm0, %xmm1
14000d28e:     	movd	%xmm1, %eax
14000d292:     	testl	%eax, %eax
14000d294:     	setns	%cl
14000d297:     	movl	%eax, %edx
14000d299:     	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
14000d29f:     	addl	$0xff800000, %edx       # imm = 0xFF800000
14000d2a5:     	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
14000d2ab:     	setb	%dl
14000d2ae:     	andb	%cl, %dl
14000d2b0:     	decl	%eax
14000d2b2:     	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
14000d2b7:     	setb	%al
14000d2ba:     	orb	%dl, %al
14000d2bc:     	movsldup	%xmm1, %xmm0            # xmm0 = xmm1[0,0,2,2]
14000d2c0:     	mulps	%xmm11, %xmm0
14000d2c4:     	movzbl	%al, %eax
14000d2c7:     	movd	%eax, %xmm1
14000d2cb:     	pshufd	$0x50, %xmm1, %xmm1     # xmm1 = xmm1[0,0,1,1]
14000d2d0:     	pslld	$0x1f, %xmm1
14000d2d5:     	psrad	$0x1f, %xmm1
14000d2da:     	pand	%xmm0, %xmm1
14000d2de:     	movq	%xmm1, 0x590(%rbp)
14000d2e6:     	movshdup	%xmm14, %xmm0           # xmm0 = xmm14[1,1,3,3]
14000d2eb:     	movshdup	%xmm12, %xmm1           # xmm1 = xmm12[1,1,3,3]
14000d2f0:     	movaps	%xmm0, %xmm11
14000d2f4:     	cmpunordss	%xmm0, %xmm11
14000d2fa:     	movaps	%xmm11, %xmm2
14000d2fe:     	andps	%xmm1, %xmm2
14000d301:     	minss	%xmm0, %xmm1
14000d305:     	andnps	%xmm1, %xmm11
14000d309:     	orps	%xmm2, %xmm11
14000d30d:     	movss	%xmm14, 0x6a0(%rbp)
14000d316:     	movss	%xmm15, 0x6a4(%rbp)
14000d31f:     	movb	$0x1, 0x817(%rbp)
14000d326:     	leaq	0x228(%rbp), %rcx
14000d32d:     	leaq	0x6a0(%rbp), %rdx
14000d334:     	movaps	%xmm11, %xmm2
14000d338:     	callq	0x140c9cde0 <.text+0xc9bde0>
14000d33d:     	movss	%xmm12, 0x6a0(%rbp)
14000d346:     	movss	%xmm13, 0x6a4(%rbp)
14000d34f:     	movb	$0x1, 0x817(%rbp)
14000d356:     	leaq	0x218(%rbp), %rcx
14000d35d:     	leaq	0x6a0(%rbp), %rdx
14000d364:     	movaps	%xmm11, %xmm2
14000d368:     	callq	0x140c9cde0 <.text+0xc9bde0>
14000d36d:     	testb	%dil, %dil
14000d370:     	je	0x14000d3ce <.text+0xc3ce>
14000d372:     	movb	$0x1, 0x817(%rbp)
14000d379:     	leaq	0x6a0(%rbp), %rcx
14000d380:     	leaq	0x590(%rbp), %rdx
14000d387:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
14000d38c:     	movsd	0x6a0(%rbp), %xmm12
14000d395:     	movss	0x6a8(%rbp), %xmm13
14000d39e:     	cmpb	$0x3, 0x7ef(%rbp)
14000d3a5:     	jne	0x14000d440 <.text+0xc440>
14000d3ab:     	mulps	0x291852d(%rip), %xmm12 # 0x1429258e0
14000d3b3:     	mulss	0x29184b4(%rip), %xmm13 # 0x142925870
14000d3bc:     	movsd	0x228(%rbp), %xmm0
14000d3c4:     	movss	0x230(%rbp), %xmm1
14000d3cc:     	jmp	0x14000d424 <.text+0xc424>
14000d3ce:     	movb	$0x1, 0x817(%rbp)
14000d3d5:     	leaq	0x6a0(%rbp), %rcx
14000d3dc:     	leaq	0x590(%rbp), %rdx
14000d3e3:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
14000d3e8:     	movsd	0x6a0(%rbp), %xmm12
14000d3f1:     	movss	0x6a8(%rbp), %xmm13
14000d3fa:     	cmpb	$0x3, 0x7ef(%rbp)
14000d401:     	jne	0x14000d45d <.text+0xc45d>
14000d403:     	mulps	0x29184d5(%rip), %xmm12 # 0x1429258e0
14000d40b:     	mulss	0x291845c(%rip), %xmm13 # 0x142925870
14000d414:     	movsd	0x218(%rbp), %xmm0
14000d41c:     	movss	0x220(%rbp), %xmm1
14000d424:     	movaps	%xmm0, %xmm14
14000d428:     	subps	%xmm12, %xmm14
14000d42c:     	movaps	%xmm1, %xmm15
14000d430:     	subss	%xmm13, %xmm15
14000d435:     	addps	%xmm0, %xmm12
14000d439:     	addss	%xmm1, %xmm13
14000d43e:     	jmp	0x14000d486 <.text+0xc486>
14000d440:     	movsd	0x228(%rbp), %xmm14
14000d449:     	movss	0x230(%rbp), %xmm15
14000d452:     	addps	%xmm14, %xmm12
14000d456:     	addss	%xmm15, %xmm13
14000d45b:     	jmp	0x14000d486 <.text+0xc486>
14000d45d:     	movsd	0x218(%rbp), %xmm0
14000d465:     	movss	0x220(%rbp), %xmm1
14000d46d:     	movaps	%xmm0, %xmm14
14000d471:     	subps	%xmm12, %xmm14
14000d475:     	movaps	%xmm1, %xmm15
14000d479:     	subss	%xmm13, %xmm15
14000d47e:     	movaps	%xmm1, %xmm13
14000d482:     	movaps	%xmm0, %xmm12
14000d486:     	leaq	0x460(%rbp), %rcx
14000d48d:     	movq	0x590(%rbp), %xmm11
14000d496:     	movl	%esi, %eax
14000d498:     	movq	%rax, %rbx
14000d49b:     	shlq	$0x20, %rbx
14000d49f:     	orq	%rax, %rbx
14000d4a2:     	movlps	%xmm14, 0x680(%rbp)
14000d4aa:     	movss	%xmm15, 0x688(%rbp)
14000d4b3:     	movlps	%xmm12, 0x68c(%rbp)
14000d4bb:     	movss	%xmm13, 0x694(%rbp)
14000d4c4:     	movshdup	%xmm14, %xmm0           # xmm0 = xmm14[1,1,3,3]
14000d4c9:     	minss	0x2918483(%rip), %xmm0  # 0x142925954
14000d4d1:     	movshdup	%xmm12, %xmm1           # xmm1 = xmm12[1,1,3,3]
14000d4d6:     	minss	%xmm0, %xmm1
14000d4da:     	movss	%xmm1, 0x548(%rbp)
14000d4e2:     	leaq	0x680(%rbp), %rax
14000d4e9:     	movq	%rax, 0x6a0(%rbp)
14000d4f0:     	leaq	0x698(%rbp), %rax
14000d4f7:     	movq	%rax, 0x6a8(%rbp)
14000d4fe:     	leaq	0x548(%rbp), %rax
14000d505:     	movq	%rax, 0x6b0(%rbp)
14000d50c:     	movb	$0x1, 0x817(%rbp)
14000d513:     	leaq	0x6a0(%rbp), %rdx
14000d51a:     	leaq	0x2919e0f(%rip), %r8    # 0x142927330
14000d521:     	callq	0x1402befa0 <.text+0x2bdfa0>
14000d526:     	movb	$0x1, 0x817(%rbp)
14000d52d:     	leaq	0x590(%rbp), %rcx
14000d534:     	leaq	0x460(%rbp), %rdx
14000d53b:     	xorl	%r8d, %r8d
14000d53e:     	callq	0x140153640 <.text+0x152640>
14000d543:     	xorl	%r14d, %r14d
14000d546:     	cmpq	0x590(%rbp), %r14
14000d54d:     	jo	0x140009bd0 <.text+0x8bd0>
14000d553:     	movq	0x5c0(%rbp), %rax
14000d55a:     	movq	%rax, 0x6d0(%rbp)
14000d561:     	movups	0x590(%rbp), %xmm0
14000d568:     	movups	0x5a0(%rbp), %xmm1
14000d56f:     	movups	0x5b0(%rbp), %xmm2
14000d576:     	movaps	%xmm2, 0x6c0(%rbp)
14000d57d:     	movaps	%xmm1, 0x6b0(%rbp)
14000d584:     	movaps	%xmm0, 0x6a0(%rbp)
14000d58b:     	leaq	0x380(%rbp), %rcx
14000d592:     	leaq	0x6a0(%rbp), %rdx
14000d599:     	movd	0x29182cf(%rip), %xmm2  # 0x142925870
14000d5a1:     	movl	$0x1, %r9d
14000d5a7:     	callq	0x140152b00 <.text+0x151b00>
14000d5ac:     	movq	0x6a0(%rbp), %rax
14000d5b3:     	testq	%rax, %rax
14000d5b6:     	je	0x14000d5d2 <.text+0xc5d2>
14000d5b8:     	movq	0x6a8(%rbp), %rcx
14000d5bf:     	shlq	$0x2, %rax
14000d5c3:     	leaq	(%rax,%rax,2), %rdx
14000d5c7:     	movl	$0x4, %r8d
14000d5cd:     	callq	0x140613c20 <.text+0x612c20>
14000d5d2:     	movq	0x6b8(%rbp), %rdx
14000d5d9:     	testq	%rdx, %rdx
14000d5dc:     	je	0x14000d5f4 <.text+0xc5f4>
14000d5de:     	movq	0x6c0(%rbp), %rcx
14000d5e5:     	shlq	$0x2, %rdx
14000d5e9:     	movl	$0x4, %r8d
14000d5ef:     	callq	0x140613c20 <.text+0x612c20>
14000d5f4:     	cmpq	0x380(%rbp), %r14
14000d5fb:     	jo	0x140009bd0 <.text+0x8bd0>
14000d601:     	movq	0x3b0(%rbp), %rax
14000d608:     	movq	%rax, 0x490(%rbp)
14000d60f:     	movupd	0x380(%rbp), %xmm0
14000d617:     	movups	0x390(%rbp), %xmm1
14000d61e:     	movups	0x3a0(%rbp), %xmm2
14000d625:     	movaps	%xmm2, 0x480(%rbp)
14000d62c:     	movaps	%xmm1, 0x470(%rbp)
14000d633:     	movapd	%xmm0, 0x460(%rbp)
14000d63b:     	movq	0x470(%rbp), %rdi
14000d642:     	testq	%rdi, %rdi
14000d645:     	je	0x14000d6fb <.text+0xc6fb>
14000d64b:     	leaq	(,%rdi,8), %rsi
14000d653:     	movzbl	0x36275af(%rip), %eax   # 0x143634c09
14000d65a:     	movl	$0x4, %edx
14000d65f:     	movq	%rsi, %rcx
14000d662:     	callq	0x140613c10 <.text+0x612c10>
14000d667:     	testq	%rax, %rax
14000d66a:     	je	0x140011696 <.text+0x10696>
14000d670:     	movl	$0x1, %r8d
14000d676:     	movq	%rax, %rcx
14000d679:     	cmpq	$0x1, %rdi
14000d67d:     	xorps	%xmm7, %xmm7
14000d680:     	movss	0x291827b(%rip), %xmm8  # 0x142925904
14000d689:     	xorps	%xmm9, %xmm9
14000d68d:     	movaps	0x291829b(%rip), %xmm10 # 0x142925930
14000d695:     	je	0x14000d6f4 <.text+0xc6f4>
14000d697:     	movl	$0x1, %edx
14000d69c:     	movq	%rax, %rcx
14000d69f:     	cmpq	$0x4, %rdi
14000d6a3:     	jbe	0x14000d6e0 <.text+0xc6e0>
14000d6a5:     	leaq	-0x1(%rdi), %r9
14000d6a9:     	movq	%r9, %r10
14000d6ac:     	andq	$-0x4, %r10
14000d6b0:     	leaq	(%rax,%r10,8), %rcx
14000d6b4:     	leaq	0x1(%r10), %rdx
14000d6b8:     	movddup	%xmm11, %xmm0           # xmm0 = xmm11[0,0]
14000d6bd:     	xorl	%r8d, %r8d
14000d6c0:     	movupd	%xmm0, (%rax,%r8,8)
14000d6c6:     	movupd	%xmm0, 0x10(%rax,%r8,8)
14000d6cd:     	addq	$0x4, %r8
14000d6d1:     	cmpq	%r8, %r10
14000d6d4:     	jne	0x14000d6c0 <.text+0xc6c0>
14000d6d6:     	movq	%rdi, %r8
14000d6d9:     	cmpq	%r10, %r9
14000d6dc:     	je	0x14000d6f4 <.text+0xc6f4>
14000d6de:     	nop
14000d6e0:     	incq	%rdx
14000d6e3:     	movlpd	%xmm11, (%rcx)
14000d6e8:     	addq	$0x8, %rcx
14000d6ec:     	cmpq	%rdx, %rdi
14000d6ef:     	jne	0x14000d6e0 <.text+0xc6e0>
14000d6f1:     	movq	%rdi, %r8
14000d6f4:     	movlpd	%xmm11, (%rcx)
14000d6f9:     	jmp	0x14000d71b <.text+0xc71b>
14000d6fb:     	movl	$0x4, %eax
14000d700:     	xorl	%r8d, %r8d
14000d703:     	xorps	%xmm7, %xmm7
14000d706:     	movss	0x29181f5(%rip), %xmm8  # 0x142925904
14000d70f:     	xorps	%xmm9, %xmm9
14000d713:     	movaps	0x2918215(%rip), %xmm10 # 0x142925930
14000d71b:     	movq	%r8, 0x500(%rbp)
14000d722:     	movq	%rax, 0x508(%rbp)
14000d729:     	movq	0x470(%rbp), %r15
14000d730:     	movq	0x488(%rbp), %rax
14000d737:     	cmpq	%r15, %rax
14000d73a:     	cmovbq	%rax, %r15
14000d73e:     	movq	%rdi, 0x510(%rbp)
14000d745:     	leaq	(,%r15,4), %rdi
14000d74d:     	movq	%r15, %rax
14000d750:     	shrq	$0x3e, %rax
14000d754:     	jne	0x140011783 <.text+0x10783>
14000d75a:     	movl	$0x0, %esi
14000d75f:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14000d769:     	cmpq	%rax, %rdi
14000d76c:     	ja	0x140011785 <.text+0x10785>
14000d772:     	movq	0x480(%rbp), %r14
14000d779:     	testq	%rdi, %rdi
14000d77c:     	je	0x14000d7ad <.text+0xc7ad>
14000d77e:     	movzbl	0x3627484(%rip), %eax   # 0x143634c09
14000d785:     	movl	$0x4, %esi
14000d78a:     	movl	$0x4, %edx
14000d78f:     	movq	%rdi, %rcx
14000d792:     	callq	0x140613c10 <.text+0x612c10>
14000d797:     	testq	%rax, %rax
14000d79a:     	je	0x140011785 <.text+0x10785>
14000d7a0:     	movq	%r15, %rcx
14000d7a3:     	testq	%r15, %r15
14000d7a6:     	jne	0x14000d7bd <.text+0xc7bd>
14000d7a8:     	jmp	0x14000d8dd <.text+0xc8dd>
14000d7ad:     	movl	$0x4, %eax
14000d7b2:     	xorl	%ecx, %ecx
14000d7b4:     	testq	%r15, %r15
14000d7b7:     	je	0x14000d8dd <.text+0xc8dd>
14000d7bd:     	movq	%rbx, %rdx
14000d7c0:     	shrq	$0x20, %rdx
14000d7c4:     	movd	%ebx, %xmm1
14000d7c8:     	movd	%edx, %xmm0
14000d7cc:     	punpckldq	%xmm1, %xmm0    # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
14000d7d0:     	cmpq	$0x8, %r15
14000d7d4:     	jb	0x14000d859 <.text+0xc859>
14000d7da:     	movq	%rax, %r8
14000d7dd:     	subq	%r14, %r8
14000d7e0:     	movl	$0x0, %edx
14000d7e5:     	cmpq	$0x20, %r8
14000d7e9:     	jb	0x14000d85b <.text+0xc85b>
14000d7eb:     	movq	%r15, %rdx
14000d7ee:     	movabsq	$0x3ffffffffffffff8, %r8 # imm = 0x3FFFFFFFFFFFFFF8
14000d7f8:     	andq	%r8, %rdx
14000d7fb:     	movdqa	%xmm0, %xmm1
14000d7ff:     	shufps	$0x55, %xmm0, %xmm1     # xmm1 = xmm1[1,1],xmm0[1,1]
14000d803:     	movaps	%xmm0, %xmm2
14000d806:     	shufps	$0x0, %xmm0, %xmm2      # xmm2 = xmm2[0,0],xmm0[0,0]
14000d80a:     	xorl	%r8d, %r8d
14000d80d:     	nopl	(%rax)
14000d810:     	movups	(%r14,%r8,4), %xmm3
14000d815:     	movups	0x10(%r14,%r8,4), %xmm4
14000d81b:     	movaps	%xmm10, %xmm5
14000d81f:     	subps	%xmm3, %xmm5
14000d822:     	movaps	%xmm10, %xmm6
14000d826:     	subps	%xmm4, %xmm6
14000d829:     	mulps	%xmm1, %xmm5
14000d82c:     	mulps	%xmm1, %xmm6
14000d82f:     	mulps	%xmm2, %xmm3
14000d832:     	addps	%xmm5, %xmm3
14000d835:     	mulps	%xmm2, %xmm4
14000d838:     	addps	%xmm6, %xmm4
14000d83b:     	movups	%xmm3, (%rax,%r8,4)
14000d840:     	movups	%xmm4, 0x10(%rax,%r8,4)
14000d846:     	addq	$0x8, %r8
14000d84a:     	cmpq	%r8, %rdx
14000d84d:     	jne	0x14000d810 <.text+0xc810>
14000d84f:     	cmpq	%rdx, %r15
14000d852:     	jne	0x14000d85b <.text+0xc85b>
14000d854:     	jmp	0x14000d8dd <.text+0xc8dd>
14000d859:     	xorl	%edx, %edx
14000d85b:     	movq	%rdx, %r8
14000d85e:     	orq	$0x1, %r8
14000d862:     	testb	$0x1, %r15b
14000d866:     	je	0x14000d88b <.text+0xc88b>
14000d868:     	movss	(%r14,%rdx,4), %xmm1
14000d86e:     	movaps	%xmm8, %xmm2
14000d872:     	subss	%xmm1, %xmm2
14000d876:     	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
14000d879:     	mulps	%xmm0, %xmm1
14000d87c:     	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14000d880:     	addss	%xmm1, %xmm2
14000d884:     	movss	%xmm2, (%rax,%rdx,4)
14000d889:     	jmp	0x14000d8d8 <.text+0xc8d8>
14000d88b:     	cmpq	%r8, %r15
14000d88e:     	je	0x14000d8dd <.text+0xc8dd>
14000d890:     	movss	(%r14,%rdx,4), %xmm1
14000d896:     	movaps	%xmm8, %xmm2
14000d89a:     	subss	%xmm1, %xmm2
14000d89e:     	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
14000d8a1:     	mulps	%xmm0, %xmm1
14000d8a4:     	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14000d8a8:     	addss	%xmm1, %xmm2
14000d8ac:     	movss	%xmm2, (%rax,%rdx,4)
14000d8b1:     	movss	0x4(%r14,%rdx,4), %xmm1
14000d8b8:     	movaps	%xmm8, %xmm2
14000d8bc:     	subss	%xmm1, %xmm2
14000d8c0:     	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
14000d8c3:     	mulps	%xmm0, %xmm1
14000d8c6:     	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14000d8ca:     	addss	%xmm1, %xmm2
14000d8ce:     	movss	%xmm2, 0x4(%rax,%rdx,4)
14000d8d4:     	leaq	0x2(%rdx), %r8
14000d8d8:     	movq	%r8, %rdx
14000d8db:     	jmp	0x14000d88b <.text+0xc88b>
14000d8dd:     	movq	%rcx, 0x3e0(%rbp)
14000d8e4:     	movq	%rax, 0x3e8(%rbp)
14000d8eb:     	movq	0x470(%rbp), %r14
14000d8f2:     	movq	0x488(%rbp), %rax
14000d8f9:     	cmpq	%r14, %rax
14000d8fc:     	cmovbq	%rax, %r14
14000d900:     	movq	%r15, 0x3f0(%rbp)
14000d907:     	leaq	(,%r14,4), %rdi
14000d90f:     	movq	%r14, %rax
14000d912:     	shrq	$0x3e, %rax
14000d916:     	jne	0x1400117ca <.text+0x107ca>
14000d91c:     	movl	$0x0, %esi
14000d921:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14000d92b:     	cmpq	%rax, %rdi
14000d92e:     	ja	0x1400117cc <.text+0x107cc>
14000d934:     	movq	0x480(%rbp), %rbx
14000d93b:     	testq	%rdi, %rdi
14000d93e:     	je	0x14000d973 <.text+0xc973>
14000d940:     	movzbl	0x36272c2(%rip), %eax   # 0x143634c09
14000d947:     	movl	$0x4, %esi
14000d94c:     	movl	$0x4, %edx
14000d951:     	movq	%rdi, %rcx
14000d954:     	callq	0x140613c10 <.text+0x612c10>
14000d959:     	testq	%rax, %rax
14000d95c:     	je	0x1400117cc <.text+0x107cc>
14000d962:     	movq	%r14, 0x780(%rbp)
14000d969:     	testq	%r14, %r14
14000d96c:     	jne	0x14000d98c <.text+0xc98c>
14000d96e:     	jmp	0x14000da86 <.text+0xca86>
14000d973:     	movl	$0x4, %eax
14000d978:     	movq	$0x0, 0x780(%rbp)
14000d983:     	testq	%r14, %r14
14000d986:     	je	0x14000da86 <.text+0xca86>
14000d98c:     	cmpq	$0x8, %r14
14000d990:     	jb	0x14000da05 <.text+0xca05>
14000d992:     	movq	%rax, %rcx
14000d995:     	subq	%rbx, %rcx
14000d998:     	movl	$0x0, %edx
14000d99d:     	cmpq	$0x20, %rcx
14000d9a1:     	jb	0x14000da07 <.text+0xca07>
14000d9a3:     	movq	%r14, %rdx
14000d9a6:     	movabsq	$0x3ffffffffffffff8, %rcx # imm = 0x3FFFFFFFFFFFFFF8
14000d9b0:     	andq	%rcx, %rdx
14000d9b3:     	xorl	%ecx, %ecx
14000d9b5:     	nopw	%cs:(%rax,%rax)
14000d9c0:     	movups	(%rbx,%rcx,4), %xmm0
14000d9c4:     	movups	0x10(%rbx,%rcx,4), %xmm1
14000d9c9:     	movaps	%xmm10, %xmm2
14000d9cd:     	subps	%xmm0, %xmm2
14000d9d0:     	movaps	%xmm10, %xmm3
14000d9d4:     	subps	%xmm1, %xmm3
14000d9d7:     	mulps	%xmm7, %xmm2
14000d9da:     	mulps	%xmm7, %xmm3
14000d9dd:     	mulps	%xmm7, %xmm0
14000d9e0:     	addps	%xmm2, %xmm0
14000d9e3:     	mulps	%xmm7, %xmm1
14000d9e6:     	addps	%xmm3, %xmm1
14000d9e9:     	movups	%xmm0, (%rax,%rcx,4)
14000d9ed:     	movups	%xmm1, 0x10(%rax,%rcx,4)
14000d9f2:     	addq	$0x8, %rcx
14000d9f6:     	cmpq	%rcx, %rdx
14000d9f9:     	jne	0x14000d9c0 <.text+0xc9c0>
14000d9fb:     	cmpq	%rdx, %r14
14000d9fe:     	jne	0x14000da07 <.text+0xca07>
14000da00:     	jmp	0x14000da86 <.text+0xca86>
14000da05:     	xorl	%edx, %edx
14000da07:     	movq	%rdx, %rcx
14000da0a:     	orq	$0x1, %rcx
14000da0e:     	testb	$0x1, %r14b
14000da12:     	je	0x14000da36 <.text+0xca36>
14000da14:     	movss	(%rbx,%rdx,4), %xmm0
14000da19:     	movaps	%xmm8, %xmm1
14000da1d:     	subss	%xmm0, %xmm1
14000da21:     	mulss	%xmm9, %xmm1
14000da26:     	mulss	%xmm9, %xmm0
14000da2b:     	addss	%xmm1, %xmm0
14000da2f:     	movss	%xmm0, (%rax,%rdx,4)
14000da34:     	jmp	0x14000da81 <.text+0xca81>
14000da36:     	cmpq	%rcx, %r14
14000da39:     	je	0x14000da86 <.text+0xca86>
14000da3b:     	movss	(%rbx,%rdx,4), %xmm0
14000da40:     	movaps	%xmm8, %xmm1
14000da44:     	subss	%xmm0, %xmm1
14000da48:     	mulss	%xmm9, %xmm1
14000da4d:     	mulss	%xmm9, %xmm0
14000da52:     	addss	%xmm1, %xmm0
14000da56:     	movss	%xmm0, (%rax,%rdx,4)
14000da5b:     	movss	0x4(%rbx,%rdx,4), %xmm0
14000da61:     	movaps	%xmm8, %xmm1
14000da65:     	subss	%xmm0, %xmm1
14000da69:     	mulss	%xmm9, %xmm1
14000da6e:     	mulss	%xmm9, %xmm0
14000da73:     	addss	%xmm1, %xmm0
14000da77:     	movss	%xmm0, 0x4(%rax,%rdx,4)
14000da7d:     	leaq	0x2(%rdx), %rcx
14000da81:     	movq	%rcx, %rdx
14000da84:     	jmp	0x14000da36 <.text+0xca36>
14000da86:     	movq	%rax, 0x800(%rbp)
14000da8d:     	movb	$0x1, 0x812(%rbp)
14000da94:     	movb	$0x1, 0x811(%rbp)
14000da9b:     	movq	0x268(%rbp), %rcx
14000daa2:     	callq	0x140987ca0 <.text+0x986ca0>
14000daa7:     	movq	%rax, %rdi
14000daaa:     	movq	0x3e8(%rbp), %r12
14000dab1:     	movq	0x3f0(%rbp), %r8
14000dab8:     	movq	0x508(%rbp), %r13
14000dabf:     	movq	0x510(%rbp), %r11
14000dac6:     	movq	0x468(%rbp), %rsi
14000dacd:     	movq	0x470(%rbp), %rbx
14000dad4:     	testq	%r8, %r8
14000dad7:     	je	0x14000db2f <.text+0xcb2f>
14000dad9:     	movq	%rbx, 0x7f0(%rbp)
14000dae0:     	movq	%r11, %rbx
14000dae3:     	movq	%r8, %r15
14000dae6:     	leaq	(,%r8,4), %rcx
14000daee:     	movzbl	0x3627114(%rip), %eax   # 0x143634c09
14000daf5:     	movl	$0x4, %edx
14000dafa:     	movq	%rcx, 0x670(%rbp)
14000db01:     	callq	0x140613c40 <.text+0x612c40>
14000db06:     	testq	%rax, %rax
14000db09:     	je	0x1400116cf <.text+0x106cf>
14000db0f:     	movq	%rax, %r10
14000db12:     	movl	0x580(%rbp), %ecx
14000db18:     	movss	0x568(%rbp), %xmm1
14000db20:     	movq	%r15, %r8
14000db23:     	movq	%rbx, %r11
14000db26:     	movq	0x7f0(%rbp), %rbx
14000db2d:     	jmp	0x14000db43 <.text+0xcb43>
14000db2f:     	movl	$0x4, %r10d
14000db35:     	movl	0x580(%rbp), %ecx
14000db3b:     	movss	0x568(%rbp), %xmm1
14000db43:     	movq	%r8, 0x760(%rbp)
14000db4a:     	movl	0x588(%rbp), %eax
14000db50:     	movl	%eax, 0x6a0(%rbp)
14000db56:     	movss	0x560(%rbp), %xmm0
14000db5e:     	movss	%xmm0, 0x6a4(%rbp)
14000db66:     	movl	%ecx, 0x6a8(%rbp)
14000db6c:     	movss	%xmm1, 0x6ac(%rbp)
14000db74:     	movq	%r8, 0x60(%rsp)
14000db79:     	movq	%r10, 0x7f0(%rbp)
14000db80:     	movq	%r10, 0x58(%rsp)
14000db85:     	leaq	0x6a0(%rbp), %rax
14000db8c:     	movq	%rax, 0x50(%rsp)
14000db91:     	movq	0x758(%rbp), %rax
14000db98:     	movl	%eax, 0x48(%rsp)
14000db9c:     	movq	0x940(%rbp), %rax
14000dba3:     	movq	%rax, 0x40(%rsp)
14000dba8:     	leaq	0x140(%rbp), %rax
14000dbaf:     	movq	%rax, 0x38(%rsp)
14000dbb4:     	movq	%rbx, 0x30(%rsp)
14000dbb9:     	movq	%rsi, 0x28(%rsp)
14000dbbe:     	movq	%r11, 0x20(%rsp)
14000dbc3:     	movq	%rdi, %rcx
14000dbc6:     	movq	%r12, %rdx
14000dbc9:     	movq	%r13, %r9
14000dbcc:     	callq	0x1407dc170 <.text+0x7db170>
14000dbd1:     	movq	0x760(%rbp), %rdx
14000dbd8:     	testq	%rdx, %rdx
14000dbdb:     	leaq	0x460(%rbp), %r15
14000dbe2:     	movq	0x678(%rbp), %r12
14000dbe9:     	je	0x14000dc01 <.text+0xcc01>
14000dbeb:     	shlq	$0x2, %rdx
14000dbef:     	movl	$0x4, %r8d
14000dbf5:     	movq	0x7f0(%rbp), %rcx
14000dbfc:     	callq	0x140613c20 <.text+0x612c20>
14000dc01:     	movq	0x938(%rbp), %rax
14000dc08:     	movq	(%rax), %rsi
14000dc0b:     	movb	0x80f(%rbp), %bl
14000dc11:     	movb	0x810(%rbp), %dil
14000dc18:     	movb	$0x1, 0x812(%rbp)
14000dc1f:     	movb	$0x1, 0x811(%rbp)
14000dc26:     	leaq	0x2a0(%rbp), %rcx
14000dc2d:     	callq	0x1400284b0 <.text+0x274b0>
14000dc32:     	movl	$0x0, 0x6a0(%rbp)
14000dc3c:     	movl	$0x0, 0x6a8(%rbp)
14000dc46:     	movb	$0x1, 0x812(%rbp)
14000dc4d:     	movb	$0x1, 0x811(%rbp)
14000dc54:     	movq	%r14, 0x80(%rsp)
14000dc5c:     	movq	0x800(%rbp), %rcx
14000dc63:     	movq	%rcx, 0x78(%rsp)
14000dc68:     	leaq	0x6a0(%rbp), %rcx
14000dc6f:     	movq	%rcx, 0x70(%rsp)
14000dc74:     	andb	$0x1, %dl
14000dc77:     	movb	%dl, 0x68(%rsp)
14000dc7b:     	andb	$0x1, %al
14000dc7d:     	movb	%al, 0x60(%rsp)
14000dc81:     	movb	%dil, 0x58(%rsp)
14000dc86:     	movb	%bl, 0x50(%rsp)
14000dc8a:     	movl	0x7b8(%rbp), %eax
14000dc90:     	movb	%al, 0x48(%rsp)
14000dc94:     	movq	%rsi, 0x38(%rsp)
14000dc99:     	movl	0x528(%rbp), %eax
14000dc9f:     	movl	%eax, 0x30(%rsp)
14000dca3:     	movq	0x758(%rbp), %rax
14000dcaa:     	movl	%eax, 0x28(%rsp)
14000dcae:     	leaq	0x428(%rbp), %rax
14000dcb5:     	movq	%rax, 0x20(%rsp)
14000dcba:     	movw	$0x101, 0x40(%rsp)      # imm = 0x101
14000dcc1:     	leaq	0x140(%rbp), %rcx
14000dcc8:     	leaq	0x3e0(%rbp), %rdx
14000dccf:     	leaq	0x500(%rbp), %r8
14000dcd6:     	movq	%r15, %r9
14000dcd9:     	callq	0x1407f98e0 <.text+0x7f88e0>
14000dcde:     	movq	0x490(%rbp), %rax
14000dce5:     	movq	%rax, 0x6d0(%rbp)
14000dcec:     	movaps	0x460(%rbp), %xmm0
14000dcf3:     	movaps	0x470(%rbp), %xmm1
14000dcfa:     	movaps	0x480(%rbp), %xmm2
14000dd01:     	movaps	%xmm2, 0x6c0(%rbp)
14000dd08:     	movaps	%xmm1, 0x6b0(%rbp)
14000dd0f:     	movaps	%xmm0, 0x6a0(%rbp)
14000dd16:     	movq	0x3f0(%rbp), %rdi
14000dd1d:     	testq	%rdi, %rdi
14000dd20:     	je	0x14000dd4e <.text+0xcd4e>
14000dd22:     	leaq	(,%rdi,4), %rsi
14000dd2a:     	movzbl	0x3626ed8(%rip), %eax   # 0x143634c09
14000dd31:     	movl	$0x4, %edx
14000dd36:     	movq	%rsi, %rcx
14000dd39:     	callq	0x140613c40 <.text+0x612c40>
14000dd3e:     	testq	%rax, %rax
14000dd41:     	movl	$0x0, %r14d
14000dd47:     	jne	0x14000dd56 <.text+0xcd56>
14000dd49:     	jmp	0x1400116b6 <.text+0x106b6>
14000dd4e:     	movl	$0x4, %eax
14000dd53:     	xorl	%r14d, %r14d
14000dd56:     	movq	0x3f0(%rbp), %rcx
14000dd5d:     	leaq	0x598(%rbp), %rdx
14000dd64:     	movq	%rcx, 0x58(%rdx)
14000dd68:     	movups	0x3e0(%rbp), %xmm0
14000dd6f:     	movups	%xmm0, 0x48(%rdx)
14000dd73:     	movq	0x510(%rbp), %rcx
14000dd7a:     	movq	%rcx, 0x40(%rdx)
14000dd7e:     	movups	0x500(%rbp), %xmm0
14000dd85:     	movups	%xmm0, 0x30(%rdx)
14000dd89:     	movq	0x6d0(%rbp), %rcx
14000dd90:     	movq	%rcx, 0x5c0(%rbp)
14000dd97:     	movaps	0x6a0(%rbp), %xmm0
14000dd9e:     	movaps	0x6b0(%rbp), %xmm1
14000dda5:     	movaps	0x6c0(%rbp), %xmm2
14000ddac:     	movaps	%xmm0, 0x590(%rbp)
14000ddb3:     	movaps	%xmm1, 0x5a0(%rbp)
14000ddba:     	movaps	%xmm2, 0x5b0(%rbp)
14000ddc1:     	movaps	%xmm0, 0x6a0(%rbp)
14000ddc8:     	movaps	%xmm1, 0x6b0(%rbp)
14000ddcf:     	movaps	%xmm2, 0x6c0(%rbp)
14000ddd6:     	movaps	0x5d0(%rbp), %xmm0
14000dddd:     	movaps	0x5e0(%rbp), %xmm1
14000dde4:     	movaps	%xmm0, 0x6e0(%rbp)
14000ddeb:     	movaps	%xmm1, 0x6f0(%rbp)
14000ddf2:     	movq	0x5f0(%rbp), %rcx
14000ddf9:     	movq	%rcx, 0x700(%rbp)
14000de00:     	movq	0x5c0(%rbp), %rcx
14000de07:     	movq	%rcx, 0x6d0(%rbp)
14000de0e:     	movq	0x5c8(%rbp), %rcx
14000de15:     	movq	%rcx, 0x6d8(%rbp)
14000de1c:     	movq	%rdi, 0x708(%rbp)
14000de23:     	movq	%rax, 0x710(%rbp)
14000de2a:     	movq	%rdi, 0x718(%rbp)
14000de31:     	movq	0x438(%rbp), %rax
14000de38:     	movq	%rax, 0x720(%rbp)
14000de3f:     	movq	(%r12), %rax
14000de43:     	movq	%rax, 0x728(%rbp)
14000de4a:     	movq	0x758(%rbp), %rax
14000de51:     	movl	%eax, 0x730(%rbp)
14000de57:     	movl	$0x101, 0x734(%rbp)     # imm = 0x101
14000de61:     	movzbl	0x7a8(%rbp), %eax
14000de68:     	movb	%al, 0x738(%rbp)
14000de6e:     	movzbl	0x7a0(%rbp), %eax
14000de75:     	movb	%al, 0x739(%rbp)
14000de7b:     	movw	$0x300, 0x73a(%rbp)     # imm = 0x300
14000de84:     	movb	$0x2, 0x750(%rbp)
14000de8b:     	movq	0x440(%rbp), %rax
14000de92:     	movq	0x458(%rbp), %rsi
14000de99:     	cmpq	%rax, %rsi
14000de9c:     	jne	0x14000debf <.text+0xcebf>
14000de9e:     	leaq	0x440(%rbp), %rcx
14000dea5:     	leaq	0x29189dc(%rip), %rdx   # 0x142926888
14000deac:     	callq	0x1405114a0 <.text+0x5104a0>
14000deb1:     	movq	0x440(%rbp), %rax
14000deb8:     	movq	0x458(%rbp), %rsi
14000debf:     	movq	0x450(%rbp), %rcx
14000dec6:     	addq	%rsi, %rcx
14000dec9:     	cmpq	%rax, %rcx
14000decc:     	movl	$0x0, %edx
14000ded1:     	cmovaeq	%rax, %rdx
14000ded5:     	subq	%rdx, %rcx
14000ded8:     	imulq	$0xb8, %rcx, %rcx
14000dedf:     	addq	0x448(%rbp), %rcx
14000dee6:     	movl	$0xb8, %r8d
14000deec:     	leaq	0x6a0(%rbp), %rdx
14000def3:     	callq	0x142923c10 <.text+0x2922c10>
14000def8:     	incq	%rsi
14000defb:     	movq	%rsi, 0x458(%rbp)
14000df02:     	movq	0x780(%rbp), %rdx
14000df09:     	testq	%rdx, %rdx
14000df0c:     	je	0x140009bd0 <.text+0x8bd0>
14000df12:     	shlq	$0x2, %rdx
14000df16:     	movl	$0x4, %r8d
14000df1c:     	movq	0x800(%rbp), %rcx
14000df23:     	callq	0x140613c20 <.text+0x612c20>
14000df28:     	jmp	0x140009bd0 <.text+0x8bd0>
14000df2d:     	movl	0x32c(%rbp), %eax
14000df33:     	movq	0x110(%rbp), %rcx
14000df3a:     	movl	%eax, (%rcx)
14000df3c:     	movq	0x378(%rbp), %rdx
14000df43:     	movl	0x1c(%rdx), %eax
14000df46:     	movq	0x10(%rdx), %rcx
14000df4a:     	movl	%eax, (%rcx)
14000df4c:     	movq	(%rdx), %rdx
14000df4f:     	movl	0x128(%r13), %r9d
14000df56:     	movl	0x12c(%r13), %eax
14000df5d:     	movb	$0x1, 0x817(%rbp)
14000df64:     	movl	%eax, 0x20(%rsp)
14000df68:     	leaq	0x6a0(%rbp), %rcx
14000df6f:     	movq	0xa8(%rbp), %r8
14000df76:     	callq	0x140252ad0 <.text+0x251ad0>
14000df7b:     	cmpb	$0x11, 0x6c0(%rbp)
14000df82:     	je	0x14000dfa4 <.text+0xcfa4>
14000df84:     	movq	0x6d8(%rbp), %rax
14000df8b:     	lock
14000df8c:     	decq	(%rax)
14000df8f:     	jne	0x14000dfa4 <.text+0xcfa4>
14000df91:     	movb	$0x1, 0x817(%rbp)
14000df98:     	leaq	0x6d8(%rbp), %rcx
14000df9f:     	callq	0x140b5c620 <.text+0xb5b620>
14000dfa4:     	movq	0x378(%rbp), %rdx
14000dfab:     	movl	0x1c(%rdx), %eax
14000dfae:     	movq	0x10(%rdx), %rcx
14000dfb2:     	movl	%eax, (%rcx)
14000dfb4:     	movq	(%rdx), %rdx
14000dfb7:     	movq	0x7e0(%rbp), %rax
14000dfbe:     	movl	0x128(%rax), %r9d
14000dfc5:     	movl	0x12c(%rax), %eax
14000dfcb:     	movb	$0x1, 0x817(%rbp)
14000dfd2:     	movl	%eax, 0x20(%rsp)
14000dfd6:     	leaq	0x6a0(%rbp), %rcx
14000dfdd:     	movq	0xb0(%rbp), %r8
14000dfe4:     	callq	0x140252ad0 <.text+0x251ad0>
14000dfe9:     	cmpb	$0x11, 0x6c0(%rbp)
14000dff0:     	je	0x14000e012 <.text+0xd012>
14000dff2:     	movq	0x6d8(%rbp), %rax
14000dff9:     	lock
14000dffa:     	decq	(%rax)
14000dffd:     	jne	0x14000e012 <.text+0xd012>
14000dfff:     	movb	$0x1, 0x817(%rbp)
14000e006:     	leaq	0x6d8(%rbp), %rcx
14000e00d:     	callq	0x140b5c620 <.text+0xb5b620>
14000e012:     	cmpq	$0x0, 0x288(%rbp)
14000e01a:     	je	0x14000e120 <.text+0xd120>
14000e020:     	movq	0x288(%rbp), %rax
14000e027:     	movq	%rax, 0x470(%rbp)
14000e02e:     	movups	0x278(%rbp), %xmm0
14000e035:     	movaps	%xmm0, 0x460(%rbp)
14000e03c:     	movaps	0x291858d(%rip), %xmm0  # 0x1429265d0
14000e043:     	movaps	%xmm0, 0x5a0(%rbp)
14000e04a:     	movaps	0x291856f(%rip), %xmm0  # 0x1429265c0
14000e051:     	movaps	%xmm0, 0x590(%rbp)
14000e058:     	movb	$0x0, 0x817(%rbp)
14000e05f:     	leaq	0x6b0(%rbp), %rcx
14000e066:     	leaq	0x460(%rbp), %rdx
14000e06d:     	leaq	0x590(%rbp), %r8
14000e074:     	callq	0x1408d0230 <.text+0x8cf230>
14000e079:     	movq	$0x1, 0x6a0(%rbp)
14000e084:     	movq	$0x1, 0x6a8(%rbp)
14000e08f:     	movzbl	0x3626b73(%rip), %eax   # 0x143634c09
14000e096:     	movl	$0x90, %ecx
14000e09b:     	movl	$0x10, %edx
14000e0a0:     	callq	0x140613c10 <.text+0x612c10>
14000e0a5:     	testq	%rax, %rax
14000e0a8:     	je	0x140011598 <.text+0x10598>
14000e0ae:     	movq	%rax, %rsi
14000e0b1:     	movl	$0x90, %r8d
14000e0b7:     	movq	%rax, %rcx
14000e0ba:     	leaq	0x6a0(%rbp), %rdi
14000e0c1:     	movq	%rdi, %rdx
14000e0c4:     	callq	0x142923c00 <.text+0x2922c00>
14000e0c9:     	movq	0x678(%rbp), %rax
14000e0d0:     	movq	(%rax), %rax
14000e0d3:     	movq	%rax, 0x6a4(%rbp)
14000e0da:     	movb	$0xd, 0x6a0(%rbp)
14000e0e1:     	movb	$0x0, 0x817(%rbp)
14000e0e8:     	leaq	0x500(%rbp), %rcx
14000e0ef:     	movq	%rsi, %rdx
14000e0f2:     	movq	%rdi, %r8
14000e0f5:     	callq	0x1408ceb10 <.text+0x8cdb10>
14000e0fa:     	xorps	%xmm8, %xmm8
14000e0fe:     	xorl	%esi, %esi
14000e100:     	movq	0x7e0(%rbp), %rdi
14000e107:     	cmpb	$0x11, 0x500(%rbp)
14000e10e:     	leaq	0x500(%rbp), %rbx
14000e115:     	cmoveq	%rsi, %rbx
14000e119:     	jne	0x14000e175 <.text+0xd175>
14000e11b:     	jmp	0x14000eb95 <.text+0xdb95>
14000e120:     	movq	0x278(%rbp), %rdx
14000e127:     	movq	0x280(%rbp), %rcx
14000e12e:     	movb	$0x11, 0x500(%rbp)
14000e135:     	testq	%rdx, %rdx
14000e138:     	xorps	%xmm8, %xmm8
14000e13c:     	movl	$0x0, %esi
14000e141:     	movq	0x7e0(%rbp), %rdi
14000e148:     	je	0x14000eb95 <.text+0xdb95>
14000e14e:     	shlq	$0x7, %rdx
14000e152:     	movl	$0x10, %r8d
14000e158:     	callq	0x140613c20 <.text+0x612c20>
14000e15d:     	cmpb	$0x11, 0x500(%rbp)
14000e164:     	leaq	0x500(%rbp), %rbx
14000e16b:     	cmoveq	%rsi, %rbx
14000e16f:     	je	0x14000eb95 <.text+0xdb95>
14000e175:     	movb	$0x1, 0x7ff(%rbp)
14000e17c:     	xorl	%ecx, %ecx
14000e17e:     	callq	0x1400d9b20 <.text+0xd8b20>
14000e183:     	testq	%rax, %rax
14000e186:     	leaq	0x460(%rbp), %rsi
14000e18d:     	je	0x1400115ce <.text+0x105ce>
14000e193:     	movups	(%rax), %xmm0
14000e196:     	incq	(%rax)
14000e199:     	movaps	%xmm0, 0x480(%rbp)
14000e1a0:     	movups	0x29a1e79(%rip), %xmm0  # 0x1429b0020
14000e1a7:     	movaps	%xmm0, 0x460(%rbp)
14000e1ae:     	movups	0x29a1e7b(%rip), %xmm0  # 0x1429b0030
14000e1b5:     	movaps	%xmm0, 0x470(%rbp)
14000e1bc:     	movq	0x378(%rbp), %rax
14000e1c3:     	movq	(%rax), %rdi
14000e1c6:     	movsd	0x2917792(%rip), %xmm0  # 0x142925960
14000e1ce:     	movsd	%xmm0, 0x6a0(%rbp)
14000e1d6:     	movl	$0x0, 0x6a8(%rbp)
14000e1e0:     	movb	$0x1, 0x7e9(%rbp)
14000e1e7:     	leaq	0x590(%rbp), %rcx
14000e1ee:     	leaq	0x6a0(%rbp), %rdx
14000e1f5:     	callq	0x140c91b30 <.text+0xc90b30>
14000e1fa:     	movb	$0x1, 0x7e9(%rbp)
14000e201:     	leaq	0x500(%rbp), %rcx
14000e208:     	callq	0x1408ceb30 <.text+0x8cdb30>
14000e20d:     	movaps	0x590(%rbp), %xmm0
14000e214:     	movaps	0x5a0(%rbp), %xmm1
14000e21b:     	movaps	%xmm1, 0x6b0(%rbp)
14000e222:     	movaps	%xmm0, 0x6a0(%rbp)
14000e229:     	movq	%rax, 0x6c0(%rbp)
14000e230:     	movb	$0x1, 0x7e9(%rbp)
14000e237:     	movq	%rsi, 0x20(%rsp)
14000e23c:     	movq	%rdi, %rcx
14000e23f:     	leaq	0x6a0(%rbp), %rdx
14000e246:     	movl	$0x2, %r8d
14000e24c:     	xorl	%r9d, %r9d
14000e24f:     	callq	0x140254890 <.text+0x253890>
14000e254:     	movq	0x460(%rbp), %r14
14000e25b:     	movq	0x468(%rbp), %rcx
14000e262:     	testq	%rcx, %rcx
14000e265:     	movq	%rcx, 0x668(%rbp)
14000e26c:     	je	0x14000e2a3 <.text+0xd2a3>
14000e26e:     	leaq	0x17(,%rcx,8), %rax
14000e276:     	andq	$-0x10, %rax
14000e27a:     	addq	%rax, %rcx
14000e27d:     	addq	$0x11, %rcx
14000e281:     	movq	%rcx, 0x7a0(%rbp)
14000e288:     	movq	%r14, %rcx
14000e28b:     	subq	%rax, %rcx
14000e28e:     	movq	%rcx, 0x670(%rbp)
14000e295:     	movl	$0x10, %eax
14000e29a:     	movq	%rax, 0x7a8(%rbp)
14000e2a1:     	jmp	0x14000e2ae <.text+0xd2ae>
14000e2a3:     	movq	$0x0, 0x7a8(%rbp)
14000e2ae:     	movq	0x478(%rbp), %rax
14000e2b5:     	movq	%rax, 0x520(%rbp)
14000e2bc:     	movdqa	(%r14), %xmm0
14000e2c1:     	leaq	0x10(%r14), %r13
14000e2c5:     	pmovmskb	%xmm0, %eax
14000e2c9:     	notl	%eax
14000e2cb:     	jmp	0x14000e2d6 <.text+0xd2d6>
14000e2cd:     	nopl	(%rax)
14000e2d0:     	movl	0x578(%rbp), %eax
14000e2d6:     	cmpq	$0x0, 0x520(%rbp)
14000e2de:     	je	0x14000eb65 <.text+0xdb65>
14000e2e4:     	testw	%ax, %ax
14000e2e7:     	je	0x14000e300 <.text+0xd300>
14000e2e9:     	leal	-0x1(%rax), %ecx
14000e2ec:     	andl	%eax, %ecx
14000e2ee:     	movl	%ecx, 0x578(%rbp)
14000e2f4:     	jmp	0x14000e32c <.text+0xd32c>
14000e2f6:     	nopw	%cs:(%rax,%rax)
14000e300:     	movdqa	(%r13), %xmm0
14000e306:     	pmovmskb	%xmm0, %eax
14000e30a:     	addq	$-0x80, %r14
14000e30e:     	addq	$0x10, %r13
14000e312:     	cmpl	$0xffff, %eax           # imm = 0xFFFF
14000e317:     	je	0x14000e300 <.text+0xd300>
14000e319:     	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14000e31e:     	subl	%eax, %ecx
14000e320:     	notl	%eax
14000e322:     	movl	%eax, %edx
14000e324:     	andl	%ecx, %edx
14000e326:     	movl	%edx, 0x578(%rbp)
14000e32c:     	tzcntl	%eax, %eax
14000e330:     	shll	$0x3, %eax
14000e333:     	movq	%r14, %rcx
14000e336:     	subq	%rax, %rcx
14000e339:     	movq	-0x8(%rcx), %rdx
14000e33d:     	movq	0xe8(%rbp), %rcx
14000e344:     	movq	%rdx, 0x648(%rbp)
14000e34b:     	leaq	0x291823e(%rip), %r8    # 0x142926590
14000e352:     	callq	0x140a91ef0 <.text+0xa90ef0>
14000e357:     	movq	%rax, %r15
14000e35a:     	decq	0x520(%rbp)
14000e361:     	movl	0x578(%rbp), %eax
14000e367:     	testq	%r15, %r15
14000e36a:     	je	0x14000e2d6 <.text+0xd2d6>
14000e370:     	movq	0xe8(%rbp), %rcx
14000e377:     	movq	0x648(%rbp), %rdx
14000e37e:     	callq	0x140a922b0 <.text+0xa912b0>
14000e383:     	movq	%rax, %rcx
14000e386:     	shrq	$0x20, %rcx
14000e38a:     	je	0x14000e480 <.text+0xd480>
14000e390:     	movl	%eax, %eax
14000e392:     	movq	0x250(%rbp), %rdx
14000e399:     	cmpq	0x10(%rdx), %rax
14000e39d:     	jae	0x14000e480 <.text+0xd480>
14000e3a3:     	movq	0x250(%rbp), %rdx
14000e3aa:     	movq	0x8(%rdx), %rdx
14000e3ae:     	leaq	(%rax,%rax,4), %rax
14000e3b2:     	cmpl	%ecx, (%rdx,%rax,4)
14000e3b5:     	jne	0x14000e480 <.text+0xd480>
14000e3bb:     	leaq	(%rdx,%rax,4), %rax
14000e3bf:     	movl	0x4(%rax), %ecx
14000e3c2:     	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
14000e3c7:     	cmpq	%rdx, %rcx
14000e3ca:     	je	0x14000e480 <.text+0xd480>
14000e3d0:     	movq	0x260(%rbp), %rdx
14000e3d7:     	cmpq	%rcx, 0x38(%rdx)
14000e3db:     	jbe	0x14000e480 <.text+0xd480>
14000e3e1:     	movq	0x260(%rbp), %rdx
14000e3e8:     	movq	0x28(%rdx), %rdx
14000e3ec:     	movl	%ecx, %r8d
14000e3ef:     	shrl	$0x6, %r8d
14000e3f3:     	movq	(%rdx,%r8,8), %rdx
14000e3f7:     	btq	%rcx, %rdx
14000e3fb:     	jae	0x14000e480 <.text+0xd480>
14000e401:     	movl	0xc(%rax), %ecx
14000e404:     	movq	0x250(%rbp), %rdx
14000e40b:     	movq	0x1a8(%rdx), %rdx
14000e412:     	leaq	(%rcx,%rcx,8), %r8
14000e416:     	movq	0x260(%rbp), %rcx
14000e41d:     	movq	0x110(%rcx), %rcx
14000e424:     	cmpq	%rcx, 0x40(%rdx,%r8,8)
14000e429:     	jbe	0x14000e452 <.text+0xd452>
14000e42b:     	leaq	(%rdx,%r8,8), %rdx
14000e42f:     	movq	0x38(%rdx), %r8
14000e433:     	movq	(%r8,%rcx,8), %rcx
14000e437:     	testq	%rcx, %rcx
14000e43a:     	je	0x14000e452 <.text+0xd452>
14000e43c:     	movq	0x18(%rdx), %rdx
14000e440:     	notq	%rcx
14000e443:     	leaq	(%rcx,%rcx,2), %rcx
14000e447:     	shlq	$0x4, %rcx
14000e44b:     	movq	0x10(%rdx,%rcx), %rcx
14000e450:     	jmp	0x14000e454 <.text+0xd454>
14000e452:     	xorl	%ecx, %ecx
14000e454:     	movl	0x10(%rax), %eax
14000e457:     	imulq	$0x58, %rax, %rax
14000e45b:     	movl	0x40(%rcx,%rax), %edx
14000e45f:     	xorl	%esi, %esi
14000e461:     	cmpb	$0x3, %dl
14000e464:     	cmovel	%esi, %edx
14000e467:     	cmpb	$0x2, %dl
14000e46a:     	je	0x14000eb59 <.text+0xdb59>
14000e470:     	movzbl	%dl, %eax
14000e473:     	cmpl	$0x1, %eax
14000e476:     	je	0x14000e48e <.text+0xd48e>
14000e478:     	nopl	(%rax,%rax)
14000e480:     	leaq	0x299(%r15), %rcx
14000e487:     	callq	0x1408c5670 <.text+0x8c4670>
14000e48c:     	movl	%eax, %esi
14000e48e:     	movq	%r15, %rcx
14000e491:     	callq	0x140b2a600 <.text+0xb29600>
14000e496:     	movq	%rax, %r12
14000e499:     	movl	0x578(%rbp), %eax
14000e49f:     	testq	%r12, %r12
14000e4a2:     	je	0x14000e2d6 <.text+0xd2d6>
14000e4a8:     	testb	$0x1, (%r12)
14000e4ad:     	je	0x14000e539 <.text+0xd539>
14000e4b3:     	movq	%r15, %rcx
14000e4b6:     	callq	0x140b2a6e0 <.text+0xb296e0>
14000e4bb:     	leaq	0x4(%r12), %rdx
14000e4c0:     	addss	0x2917438(%rip), %xmm0  # 0x142925900
14000e4c8:     	leaq	0x6a0(%rbp), %rcx
14000e4cf:     	movaps	%xmm0, %xmm2
14000e4d2:     	callq	0x140c98b90 <.text+0xc97b90>
14000e4d7:     	movzbl	0x362672b(%rip), %eax   # 0x143634c09
14000e4de:     	movl	$0x30, %ecx
14000e4e3:     	movl	$0x4, %edx
14000e4e8:     	callq	0x140613c10 <.text+0x612c10>
14000e4ed:     	testq	%rax, %rax
14000e4f0:     	je	0x140011475 <.text+0x10475>
14000e4f6:     	movups	0x6a0(%rbp), %xmm0
14000e4fd:     	movups	0x6b0(%rbp), %xmm1
14000e504:     	movups	0x6c0(%rbp), %xmm2
14000e50b:     	movups	%xmm2, 0x20(%rax)
14000e50f:     	movups	%xmm1, 0x10(%rax)
14000e513:     	movq	%rax, 0x780(%rbp)
14000e51a:     	movups	%xmm0, (%rax)
14000e51d:     	leaq	0xc(%r12), %rdi
14000e522:     	addq	$0x10, %r12
14000e526:     	movl	$0x4, %eax
14000e52b:     	movq	%rax, 0x800(%rbp)
14000e532:     	movl	$0x4, %ecx
14000e537:     	jmp	0x14000e59d <.text+0xd59d>
14000e539:     	movq	%r15, %rcx
14000e53c:     	callq	0x140b2a6e0 <.text+0xb296e0>
14000e541:     	leaq	0x4(%r12), %rdi
14000e546:     	addss	0x29173b2(%rip), %xmm0  # 0x142925900
14000e54e:     	leaq	0x6a0(%rbp), %rcx
14000e555:     	movq	%rdi, %rdx
14000e558:     	movss	0x2917410(%rip), %xmm2  # 0x142925970
14000e560:     	movaps	%xmm0, %xmm3
14000e563:     	callq	0x140c97c50 <.text+0xc96c50>
14000e568:     	leaq	0x50(%rbp), %rcx
14000e56c:     	leaq	0x6a0(%rbp), %rdx
14000e573:     	leaq	0x29cfb0e(%rip), %r8    # 0x1429de088
14000e57a:     	callq	0x1402bd6b0 <.text+0x2bc6b0>
14000e57f:     	addq	$0x8, %r12
14000e583:     	movq	0x50(%rbp), %rax
14000e587:     	movq	%rax, 0x800(%rbp)
14000e58e:     	movq	0x58(%rbp), %rax
14000e592:     	movq	%rax, 0x780(%rbp)
14000e599:     	movq	0x60(%rbp), %rcx
14000e59d:     	movl	%esi, 0x760(%rbp)
14000e5a3:     	movss	(%r12), %xmm0
14000e5a9:     	movss	(%rdi), %xmm1
14000e5ad:     	movss	%xmm1, 0x680(%rbp)
14000e5b5:     	movss	%xmm0, 0x684(%rbp)
14000e5bd:     	leaq	(,%rcx,4), %rax
14000e5c5:     	leaq	(%rax,%rax,2), %rdi
14000e5c9:     	movq	%rcx, 0x7f0(%rbp)
14000e5d0:     	testq	%rcx, %rcx
14000e5d3:     	je	0x14000e5f7 <.text+0xd5f7>
14000e5d5:     	movzbl	0x362662d(%rip), %eax   # 0x143634c09
14000e5dc:     	movl	$0x4, %edx
14000e5e1:     	movq	%rdi, %rcx
14000e5e4:     	callq	0x140613c10 <.text+0x612c10>
14000e5e9:     	testq	%rax, %rax
14000e5ec:     	je	0x140011443 <.text+0x10443>
14000e5f2:     	movq	%rax, %rsi
14000e5f5:     	jmp	0x14000e5fc <.text+0xd5fc>
14000e5f7:     	movl	$0x4, %esi
14000e5fc:     	movq	0x780(%rbp), %r12
14000e603:     	leaq	(%r12,%rdi), %rax
14000e607:     	movq	%rax, 0x7b0(%rbp)
14000e60e:     	movq	%rsi, %rcx
14000e611:     	movq	%r12, %rdx
14000e614:     	movq	%rdi, %r8
14000e617:     	callq	0x142923c00 <.text+0x2922c00>
14000e61c:     	addq	%rsi, %rdi
14000e61f:     	movq	0x7f0(%rbp), %rax
14000e626:     	movq	%rax, 0x6a0(%rbp)
14000e62d:     	movq	%rsi, 0x6a8(%rbp)
14000e634:     	movq	%rsi, 0x6b0(%rbp)
14000e63b:     	movq	%rax, 0x6b8(%rbp)
14000e642:     	movq	%rdi, 0x6c0(%rbp)
14000e649:     	movq	%r12, 0x6c8(%rbp)
14000e650:     	movq	%r12, 0x6d0(%rbp)
14000e657:     	movq	0x800(%rbp), %rax
14000e65e:     	movq	%rax, 0x6d8(%rbp)
14000e665:     	movq	0x7b0(%rbp), %rax
14000e66c:     	movq	%rax, 0x6e0(%rbp)
14000e673:     	movl	$0x0, 0x6e8(%rbp)
14000e67d:     	movss	0x29172ef(%rip), %xmm0  # 0x142925974
14000e685:     	movss	%xmm0, 0x570(%rbp)
14000e68d:     	testb	$0x1, 0x760(%rbp)
14000e694:     	jne	0x14000e6a6 <.text+0xd6a6>
14000e696:     	movss	0x29172da(%rip), %xmm0  # 0x142925978
14000e69e:     	movss	%xmm0, 0x570(%rbp)
14000e6a6:     	xorps	%xmm0, %xmm0
14000e6a9:     	jmp	0x14000e704 <.text+0xd704>
14000e6ab:     	nopl	(%rax,%rax)
14000e6b0:     	movss	0x778(%rbp), %xmm0
14000e6b8:     	subss	%xmm7, %xmm0
14000e6bc:     	movq	0x8(%r12), %rax
14000e6c1:     	leaq	(%rsi,%rsi,2), %rcx
14000e6c5:     	movq	0x648(%rbp), %rdx
14000e6cc:     	movq	%rdx, (%rax,%rcx,8)
14000e6d0:     	movss	%xmm12, 0x8(%rax,%rcx,8)
14000e6d7:     	movss	%xmm11, 0xc(%rax,%rcx,8)
14000e6de:     	movss	%xmm0, 0x10(%rax,%rcx,8)
14000e6e4:     	movl	$0x41200000, 0x14(%rax,%rcx,8) # imm = 0x41200000
14000e6ec:     	incq	%rsi
14000e6ef:     	movq	%rsi, 0x10(%r12)
14000e6f4:     	movss	0x7f0(%rbp), %xmm0
14000e6fc:     	addss	0x7b0(%rbp), %xmm0
14000e704:     	movss	%xmm0, 0x7f0(%rbp)
14000e70c:     	leaq	0x380(%rbp), %rcx
14000e713:     	leaq	0x6a0(%rbp), %rdx
14000e71a:     	callq	0x1404c2650 <.text+0x4c1650>
14000e71f:     	testb	$0x1, 0x380(%rbp)
14000e726:     	je	0x14000eb00 <.text+0xdb00>
14000e72c:     	movss	0x38c(%rbp), %xmm0
14000e734:     	movss	%xmm0, 0x800(%rbp)
14000e73c:     	movsd	0x384(%rbp), %xmm0
14000e744:     	movaps	%xmm0, 0x780(%rbp)
14000e74b:     	movsd	0x390(%rbp), %xmm0
14000e753:     	movaps	%xmm0, 0x760(%rbp)
14000e75a:     	movss	0x398(%rbp), %xmm8
14000e763:     	leaq	0x590(%rbp), %rcx
14000e76a:     	leaq	0x680(%rbp), %rdx
14000e771:     	callq	0x140c9cdc0 <.text+0xc9bdc0>
14000e776:     	movss	0x800(%rbp), %xmm11
14000e77f:     	addss	%xmm8, %xmm11
14000e784:     	mulss	0x29170e3(%rip), %xmm11 # 0x142925870
14000e78d:     	subss	0x598(%rbp), %xmm11
14000e796:     	movaps	%xmm11, %xmm1
14000e79a:     	mulss	%xmm11, %xmm1
14000e79f:     	movaps	0x780(%rbp), %xmm0
14000e7a6:     	addps	0x760(%rbp), %xmm0
14000e7ad:     	mulps	0x291712c(%rip), %xmm0  # 0x1429258e0
14000e7b4:     	movsd	0x590(%rbp), %xmm2
14000e7bc:     	subps	%xmm2, %xmm0
14000e7bf:     	movaps	%xmm0, %xmm2
14000e7c2:     	mulps	%xmm0, %xmm2
14000e7c5:     	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
14000e7c9:     	addss	%xmm2, %xmm3
14000e7cd:     	addss	%xmm1, %xmm3
14000e7d1:     	xorps	%xmm2, %xmm2
14000e7d4:     	sqrtss	%xmm3, %xmm2
14000e7d8:     	movss	0x2917124(%rip), %xmm1  # 0x142925904
14000e7e0:     	divss	%xmm2, %xmm1
14000e7e4:     	movd	%xmm1, %eax
14000e7e8:     	testl	%eax, %eax
14000e7ea:     	setns	%cl
14000e7ed:     	movl	%eax, %edx
14000e7ef:     	andl	$0x7fffffff, %edx       # imm = 0x7FFFFFFF
14000e7f5:     	addl	$0xff800000, %edx       # imm = 0xFF800000
14000e7fb:     	cmpl	$0x7f000000, %edx       # imm = 0x7F000000
14000e801:     	setb	%dl
14000e804:     	andb	%cl, %dl
14000e806:     	decl	%eax
14000e808:     	cmpl	$0x7fffff, %eax         # imm = 0x7FFFFF
14000e80d:     	setb	%al
14000e810:     	orb	%dl, %al
14000e812:     	jne	0x14000e820 <.text+0xd820>
14000e814:     	xorps	%xmm11, %xmm11
14000e818:     	jmp	0x14000e825 <.text+0xd825>
14000e81a:     	nopw	(%rax,%rax)
14000e820:     	mulss	%xmm1, %xmm11
14000e825:     	movaps	0x760(%rbp), %xmm4
14000e82c:     	movaps	%xmm4, %xmm9
14000e830:     	movaps	0x780(%rbp), %xmm12
14000e838:     	subps	%xmm12, %xmm9
14000e83c:     	movaps	%xmm8, %xmm6
14000e840:     	movss	0x800(%rbp), %xmm10
14000e849:     	subss	%xmm10, %xmm6
14000e84e:     	movaps	%xmm9, %xmm2
14000e852:     	mulps	%xmm9, %xmm2
14000e856:     	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
14000e85a:     	addss	%xmm2, %xmm3
14000e85e:     	movaps	%xmm6, %xmm2
14000e861:     	mulss	%xmm6, %xmm2
14000e865:     	addss	%xmm3, %xmm2
14000e869:     	sqrtss	%xmm2, %xmm2
14000e86d:     	movss	%xmm2, 0x7b0(%rbp)
14000e875:     	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
14000e879:     	mulps	%xmm1, %xmm0
14000e87c:     	movzbl	%al, %eax
14000e87f:     	movd	%eax, %xmm1
14000e883:     	pshufd	$0x50, %xmm1, %xmm14    # xmm14 = xmm1[0,0,1,1]
14000e889:     	pslld	$0x1f, %xmm14
14000e88f:     	psrad	$0x1f, %xmm14
14000e895:     	pand	%xmm0, %xmm14
14000e89a:     	movq	$0x2, 0x2a8(%rbp)
14000e8a5:     	movabsq	$0x3e99999a00000000, %rax # imm = 0x3E99999A00000000
14000e8af:     	movq	%rax, 0x2b0(%rbp)
14000e8b6:     	subps	%xmm4, %xmm12
14000e8ba:     	subss	%xmm8, %xmm10
14000e8bf:     	xorl	%esi, %esi
14000e8c1:     	movss	0x29170b7(%rip), %xmm7  # 0x142925980
14000e8c9:     	movss	0x29170ab(%rip), %xmm0  # 0x14292597c
14000e8d1:     	movss	%xmm0, 0x778(%rbp)
14000e8d9:     	nopl	(%rax)
14000e8e0:     	movq	%rsi, %rax
14000e8e3:     	cmpq	$0x2, %rsi
14000e8e7:     	je	0x14000ea60 <.text+0xda60>
14000e8ed:     	leaq	0x1(%rax), %rsi
14000e8f1:     	movq	%rsi, 0x2a0(%rbp)
14000e8f8:     	movss	0x2b0(%rbp,%rax,4), %xmm15
14000e902:     	movsldup	%xmm15, %xmm13          # xmm13 = xmm15[0,0,2,2]
14000e907:     	mulps	%xmm14, %xmm13
14000e90b:     	mulss	%xmm11, %xmm15
14000e910:     	movss	0x800(%rbp), %xmm0
14000e918:     	addss	%xmm15, %xmm0
14000e91d:     	movaps	0x780(%rbp), %xmm1
14000e924:     	addps	%xmm13, %xmm1
14000e928:     	movlps	%xmm1, 0x3e0(%rbp)
14000e92f:     	movss	%xmm0, 0x3e8(%rbp)
14000e937:     	movlps	%xmm9, 0x3ec(%rbp)
14000e93f:     	movss	%xmm6, 0x3f4(%rbp)
14000e947:     	movl	$0x3f800000, 0x20(%rsp) # imm = 0x3F800000
14000e94f:     	leaq	0x590(%rbp), %rcx
14000e956:     	movq	%rbx, %rdx
14000e959:     	leaq	0x2917c60(%rip), %r8    # 0x1429265c0
14000e960:     	leaq	0x3e0(%rbp), %r9
14000e967:     	callq	0x1408ceb40 <.text+0x8cdb40>
14000e96c:     	cmpl	$0x2, 0x5c4(%rbp)
14000e973:     	je	0x14000e9a8 <.text+0xd9a8>
14000e975:     	movss	0x5c0(%rbp), %xmm0
14000e97d:     	mulss	0x7b0(%rbp), %xmm0
14000e985:     	addss	0x7f0(%rbp), %xmm0
14000e98d:     	movaps	%xmm7, %xmm1
14000e990:     	cmpunordss	%xmm7, %xmm1
14000e995:     	movaps	%xmm1, %xmm2
14000e998:     	andps	%xmm0, %xmm2
14000e99b:     	minss	%xmm7, %xmm0
14000e99f:     	andnps	%xmm0, %xmm1
14000e9a2:     	orps	%xmm2, %xmm1
14000e9a5:     	movaps	%xmm1, %xmm7
14000e9a8:     	addss	%xmm8, %xmm15
14000e9ad:     	addps	0x760(%rbp), %xmm13
14000e9b5:     	movlps	%xmm13, 0x3e0(%rbp)
14000e9bd:     	movss	%xmm15, 0x3e8(%rbp)
14000e9c6:     	movlps	%xmm12, 0x3ec(%rbp)
14000e9ce:     	movss	%xmm10, 0x3f4(%rbp)
14000e9d7:     	movl	$0x3f800000, 0x20(%rsp) # imm = 0x3F800000
14000e9df:     	leaq	0x590(%rbp), %rcx
14000e9e6:     	movq	%rbx, %rdx
14000e9e9:     	leaq	0x2917bd0(%rip), %r8    # 0x1429265c0
14000e9f0:     	leaq	0x3e0(%rbp), %r9
14000e9f7:     	callq	0x1408ceb40 <.text+0x8cdb40>
14000e9fc:     	cmpl	$0x2, 0x5c4(%rbp)
14000ea03:     	je	0x14000e8e0 <.text+0xd8e0>
14000ea09:     	movss	0x2916ef3(%rip), %xmm0  # 0x142925904
14000ea11:     	subss	0x5c0(%rbp), %xmm0
14000ea19:     	mulss	0x7b0(%rbp), %xmm0
14000ea21:     	addss	0x7f0(%rbp), %xmm0
14000ea29:     	movss	0x778(%rbp), %xmm3
14000ea31:     	movaps	%xmm3, %xmm1
14000ea34:     	cmpunordss	%xmm3, %xmm1
14000ea39:     	movaps	%xmm1, %xmm2
14000ea3c:     	andps	%xmm0, %xmm2
14000ea3f:     	maxss	%xmm3, %xmm0
14000ea43:     	andnps	%xmm0, %xmm1
14000ea46:     	orps	%xmm2, %xmm1
14000ea49:     	movss	%xmm1, 0x778(%rbp)
14000ea51:     	jmp	0x14000e8e0 <.text+0xd8e0>
14000ea56:     	nopw	%cs:(%rax,%rax)
14000ea60:     	movaps	%xmm7, %xmm0
14000ea63:     	addss	0x2916f19(%rip), %xmm0  # 0x142925984
14000ea6b:     	movss	0x778(%rbp), %xmm1
14000ea73:     	ucomiss	%xmm0, %xmm1
14000ea76:     	jbe	0x14000e6f4 <.text+0xd6f4>
14000ea7c:     	movq	%r15, %rcx
14000ea7f:     	callq	0x140b2a620 <.text+0xb29620>
14000ea84:     	movaps	%xmm0, %xmm11
14000ea88:     	movq	0x208(%rbp), %r12
14000ea8f:     	movq	%r15, %rcx
14000ea92:     	callq	0x140b2a740 <.text+0xb29740>
14000ea97:     	subss	0x570(%rbp), %xmm11
14000eaa0:     	addss	0x2916edf(%rip), %xmm11 # 0x142925988
14000eaa9:     	movaps	%xmm7, %xmm1
14000eaac:     	addss	0x778(%rbp), %xmm1
14000eab4:     	mulss	0x2916db4(%rip), %xmm1  # 0x142925870
14000eabc:     	movl	%eax, 0x590(%rbp)
14000eac2:     	movss	%xmm0, 0x594(%rbp)
14000eaca:     	leaq	0x590(%rbp), %rcx
14000ead1:     	callq	0x140c91eb0 <.text+0xc90eb0>
14000ead6:     	movaps	%xmm0, %xmm12
14000eada:     	movq	0x10(%r12), %rsi
14000eadf:     	cmpq	(%r12), %rsi
14000eae3:     	jne	0x14000e6b0 <.text+0xd6b0>
14000eae9:     	movq	%r12, %rcx
14000eaec:     	leaq	0x2917ab5(%rip), %rdx   # 0x1429265a8
14000eaf3:     	callq	0x140562db0 <.text+0x561db0>
14000eaf8:     	jmp	0x14000e6b0 <.text+0xd6b0>
14000eafd:     	nopl	(%rax)
14000eb00:     	movq	0x6b8(%rbp), %rax
14000eb07:     	testq	%rax, %rax
14000eb0a:     	je	0x14000eb26 <.text+0xdb26>
14000eb0c:     	movq	0x6a8(%rbp), %rcx
14000eb13:     	shlq	$0x2, %rax
14000eb17:     	leaq	(%rax,%rax,2), %rdx
14000eb1b:     	movl	$0x4, %r8d
14000eb21:     	callq	0x140613c20 <.text+0x612c20>
14000eb26:     	movq	0x6d8(%rbp), %rax
14000eb2d:     	testq	%rax, %rax
14000eb30:     	xorps	%xmm8, %xmm8
14000eb34:     	je	0x14000e2d0 <.text+0xd2d0>
14000eb3a:     	movq	0x6c8(%rbp), %rcx
14000eb41:     	shlq	$0x2, %rax
14000eb45:     	leaq	(%rax,%rax,2), %rdx
14000eb49:     	movl	$0x4, %r8d
14000eb4f:     	callq	0x140613c20 <.text+0x612c20>
14000eb54:     	jmp	0x14000e2d0 <.text+0xd2d0>
14000eb59:     	addq	%rax, %rcx
14000eb5c:     	movzbl	0x41(%rcx), %esi
14000eb60:     	jmp	0x14000e48e <.text+0xd48e>
14000eb65:     	cmpq	$0x0, 0x668(%rbp)
14000eb6d:     	movq	0x7a0(%rbp), %rdx
14000eb74:     	je	0x14000eb8e <.text+0xdb8e>
14000eb76:     	testq	%rdx, %rdx
14000eb79:     	je	0x14000eb8e <.text+0xdb8e>
14000eb7b:     	movq	0x670(%rbp), %rcx
14000eb82:     	movq	0x7a8(%rbp), %r8
14000eb89:     	callq	0x140613c20 <.text+0x612c20>
14000eb8e:     	movq	0x7e0(%rbp), %rdi
14000eb95:     	movq	0x378(%rbp), %rdx
14000eb9c:     	movl	0x1c(%rdx), %eax
14000eb9f:     	movq	0x10(%rdx), %rcx
14000eba3:     	movl	%eax, (%rcx)
14000eba5:     	movq	(%rdx), %rcx
14000eba8:     	movl	0x32c(%rbp), %eax
14000ebae:     	movq	0x110(%rbp), %rdx
14000ebb5:     	movl	%eax, (%rdx)
14000ebb7:     	movzbl	0x500(%rbp), %eax
14000ebbe:     	movq	0x501(%rbp), %rdx
14000ebc5:     	movl	0x509(%rbp), %r8d
14000ebcc:     	movzwl	0x50d(%rbp), %r9d
14000ebd4:     	movzbl	0x50f(%rbp), %r10d
14000ebdc:     	movups	0x510(%rbp), %xmm0
14000ebe3:     	movaps	%xmm0, 0x6b0(%rbp)
14000ebea:     	movb	%al, 0x6a0(%rbp)
14000ebf0:     	movq	%rdx, 0x6a1(%rbp)
14000ebf7:     	movl	%r8d, 0x6a9(%rbp)
14000ebfe:     	movw	%r9w, 0x6ad(%rbp)
14000ec06:     	movb	%r10b, 0x6af(%rbp)
14000ec0d:     	movl	0x128(%rdi), %r9d
14000ec14:     	movl	0x12c(%rdi), %eax
14000ec1a:     	movb	$0x0, 0x7ff(%rbp)
14000ec21:     	movl	%eax, 0x20(%rsp)
14000ec25:     	movq	0xb8(%rbp), %rdx
14000ec2c:     	leaq	0x6a0(%rbp), %r8
14000ec33:     	callq	0x1407deb60 <.text+0x7ddb60>
14000ec38:     	movq	0x458(%rbp), %rax
14000ec3f:     	testq	%rax, %rax
14000ec42:     	movq	0x940(%rbp), %r13
14000ec49:     	je	0x14000eca7 <.text+0xdca7>
14000ec4b:     	movq	0x440(%rbp), %rcx
14000ec52:     	movq	0x450(%rbp), %rdx
14000ec59:     	leaq	0x1(%rdx), %r8
14000ec5d:     	cmpq	%rcx, %r8
14000ec60:     	movl	$0x0, %r8d
14000ec66:     	cmovaeq	%rcx, %r8
14000ec6a:     	negq	%r8
14000ec6d:     	leaq	(%rdx,%r8), %rcx
14000ec71:     	incq	%rcx
14000ec74:     	movq	%rcx, 0x450(%rbp)
14000ec7b:     	decq	%rax
14000ec7e:     	movq	%rax, 0x458(%rbp)
14000ec85:     	imulq	$0xb8, %rdx, %rdx
14000ec8c:     	addq	0x448(%rbp), %rdx
14000ec93:     	movl	$0xb8, %r8d
14000ec99:     	leaq	0x6a0(%rbp), %rcx
14000eca0:     	callq	0x142923c00 <.text+0x2922c00>
14000eca5:     	jmp	0x14000ecb8 <.text+0xdcb8>
14000eca7:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14000ecb1:     	movq	%rax, 0x6a0(%rbp)
14000ecb8:     	leaq	0x590(%rbp), %rbx
14000ecbf:     	leaq	0x460(%rbp), %r12
14000ecc6:     	jmp	0x14000ecf0 <.text+0xdcf0>
14000ecc8:     	imulq	$0xa8, %rsi, %rcx
14000eccf:     	addq	0x8(%rdi), %rcx
14000ecd3:     	movl	$0xa8, %r8d
14000ecd9:     	movq	%rbx, %rdx
14000ecdc:     	callq	0x142923c10 <.text+0x2922c10>
14000ece1:     	incq	%rsi
14000ece4:     	movq	%rsi, 0x10(%rdi)
14000ece8:     	nopl	(%rax,%rax)
14000ecf0:     	movq	0x6a0(%rbp), %rcx
14000ecf7:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14000ed01:     	movq	%rax, 0x6a0(%rbp)
14000ed08:     	movq	%rcx, 0x800(%rbp)
14000ed0f:     	movq	%rcx, %rax
14000ed12:     	negq	%rax
14000ed15:     	jo	0x140010592 <.text+0xf592>
14000ed1b:     	movq	0x6a8(%rbp), %rax
14000ed22:     	movq	%rax, 0x7f0(%rbp)
14000ed29:     	movq	0x6b0(%rbp), %r14
14000ed30:     	movq	0x6b8(%rbp), %r15
14000ed37:     	movq	0x6c0(%rbp), %rax
14000ed3e:     	movq	%rax, 0x528(%rbp)
14000ed45:     	leaq	0x6d8(%rbp), %rcx
14000ed4c:     	movdqu	-0x10(%rcx), %xmm0
14000ed51:     	movdqa	%xmm0, 0x310(%rbp)
14000ed59:     	movq	0x6d8(%rbp), %r8
14000ed60:     	movq	0x6e0(%rbp), %rax
14000ed67:     	movq	%rax, 0x7b0(%rbp)
14000ed6e:     	movq	0x6e8(%rbp), %rbx
14000ed75:     	movq	0x6f0(%rbp), %rax
14000ed7c:     	movq	%rax, 0x780(%rbp)
14000ed83:     	movq	0x6f8(%rbp), %rax
14000ed8a:     	movq	%rax, 0x760(%rbp)
14000ed91:     	movq	0x700(%rbp), %rdi
14000ed98:     	movq	0x708(%rbp), %rdx
14000ed9f:     	movq	0x710(%rbp), %rax
14000eda6:     	movq	%rax, 0x778(%rbp)
14000edad:     	movzwl	0x66(%rcx), %eax
14000edb1:     	movw	%ax, 0x54c(%rbp)
14000edb8:     	movl	0x62(%rcx), %eax
14000edbb:     	movl	%eax, 0x548(%rbp)
14000edc1:     	movq	0x718(%rbp), %rsi
14000edc8:     	movss	0x734(%rbp), %xmm6
14000edd0:     	movq	0x720(%rbp), %rax
14000edd7:     	movq	%rax, 0x7b8(%rbp)
14000edde:     	movl	0x728(%rbp), %eax
14000ede4:     	movl	%eax, 0x588(%rbp)
14000edea:     	movl	0x72c(%rbp), %r10d
14000edf1:     	movl	0x730(%rbp), %r9d
14000edf8:     	movzwl	0x738(%rbp), %r11d
14000ee00:     	movq	0x458(%rbp), %rcx
14000ee07:     	testq	%rcx, %rcx
14000ee0a:     	movq	%r15, 0x570(%rbp)
14000ee11:     	movq	%rdx, 0x7e0(%rbp)
14000ee18:     	movq	%r8, 0x648(%rbp)
14000ee1f:     	je	0x14000f5c5 <.text+0xe5c5>
14000ee25:     	movl	%r10d, 0x560(%rbp)
14000ee2c:     	movl	%r9d, 0x580(%rbp)
14000ee33:     	movq	0x450(%rbp), %rdx
14000ee3a:     	movq	0x440(%rbp), %r8
14000ee41:     	movq	0x448(%rbp), %rax
14000ee48:     	cmpq	%r8, %rdx
14000ee4b:     	movl	$0x0, %r9d
14000ee51:     	cmovaeq	%r8, %r9
14000ee55:     	movq	%rdx, %r10
14000ee58:     	subq	%r9, %r10
14000ee5b:     	imulq	$0xb8, %r10, %r9
14000ee62:     	cmpb	$0x2, 0xb0(%rax,%r9)
14000ee6b:     	je	0x14000f51e <.text+0xe51e>
14000ee71:     	cmpb	$0x2, 0x750(%rbp)
14000ee78:     	je	0x14000f51e <.text+0xe51e>
14000ee7e:     	movq	0x748(%rbp), %r10
14000ee85:     	addq	%rax, %r9
14000ee88:     	cmpq	0xa0(%r9), %r10
14000ee8f:     	jne	0x14000f51e <.text+0xe51e>
14000ee95:     	movq	0x740(%rbp), %r10
14000ee9c:     	movq	%r10, 0x330(%rbp)
14000eea3:     	movq	0xa8(%r9), %r9
14000eeaa:     	movq	%r9, 0x530(%rbp)
14000eeb1:     	leaq	0x1(%rdx), %r9
14000eeb5:     	cmpq	%r8, %r9
14000eeb8:     	movl	$0x0, %r9d
14000eebe:     	cmovaeq	%r8, %r9
14000eec2:     	negq	%r9
14000eec5:     	leaq	(%rdx,%r9), %r8
14000eec9:     	incq	%r8
14000eecc:     	movq	%r8, 0x450(%rbp)
14000eed3:     	decq	%rcx
14000eed6:     	movq	%rcx, 0x458(%rbp)
14000eedd:     	imulq	$0xb8, %rdx, %r9
14000eee4:     	movq	(%rax,%r9), %rcx
14000eee8:     	movq	%rcx, %rdx
14000eeeb:     	negq	%rdx
14000eeee:     	jo	0x1400112fc <.text+0x102fc>
14000eef4:     	movw	%r11w, 0x430(%rbp)
14000eefc:     	movq	0x8(%rax,%r9), %rdx
14000ef01:     	movq	0x10(%rax,%r9), %r8
14000ef06:     	movq	0x18(%rax,%r9), %r10
14000ef0b:     	movq	%r10, 0x7a8(%rbp)
14000ef12:     	movq	0x20(%rax,%r9), %r10
14000ef17:     	movq	%r10, 0x568(%rbp)
14000ef1e:     	movq	0x38(%rax,%r9), %r10
14000ef23:     	movq	%r10, 0x578(%rbp)
14000ef2a:     	movq	0x40(%rax,%r9), %r10
14000ef2f:     	movq	%r10, 0x668(%rbp)
14000ef36:     	movq	0x48(%rax,%r9), %r13
14000ef3b:     	movq	0x50(%rax,%r9), %r10
14000ef40:     	movq	%r10, 0x520(%rbp)
14000ef47:     	movq	0x58(%rax,%r9), %r10
14000ef4c:     	movq	%r10, 0x7a0(%rbp)
14000ef53:     	movq	0x60(%rax,%r9), %r12
14000ef58:     	movq	0x68(%rax,%r9), %r10
14000ef5d:     	movq	%r10, 0x670(%rbp)
14000ef64:     	movq	0x70(%rax,%r9), %r10
14000ef69:     	movq	%r10, 0x798(%rbp)
14000ef70:     	movq	0x78(%rax,%r9), %r15
14000ef75:     	movq	0x800(%rbp), %rax
14000ef7c:     	movq	%rax, 0x650(%rbp)
14000ef83:     	movq	0x7f0(%rbp), %rax
14000ef8a:     	movq	%rax, 0x658(%rbp)
14000ef91:     	movq	%r14, 0x660(%rbp)
14000ef98:     	testq	%r14, %r14
14000ef9b:     	je	0x14000efa7 <.text+0xdfa7>
14000ef9d:     	decq	%r14
14000efa0:     	movq	%r14, 0x660(%rbp)
14000efa7:     	leaq	(%r8,%r8,2), %rax
14000efab:     	leaq	(%rdx,%rax,4), %rax
14000efaf:     	movq	%rdx, 0x590(%rbp)
14000efb6:     	movq	%rdx, 0x598(%rbp)
14000efbd:     	movq	%rcx, 0x5a0(%rbp)
14000efc4:     	movq	%rax, 0x5a8(%rbp)
14000efcb:     	movq	$0x1, 0x5b0(%rbp)
14000efd6:     	movb	$0x1, 0x7dd(%rbp)
14000efdd:     	movb	$0x1, 0x7dc(%rbp)
14000efe4:     	movb	$0x1, 0x7db(%rbp)
14000efeb:     	movb	$0x1, 0x7da(%rbp)
14000eff2:     	leaq	0x650(%rbp), %rcx
14000eff9:     	leaq	0x590(%rbp), %rdx
14000f000:     	leaq	0x29175f1(%rip), %r8    # 0x1429265f8
14000f007:     	callq	0x1402b9e00 <.text+0x2b8e00>
14000f00c:     	movq	0x648(%rbp), %rax
14000f013:     	movq	%rax, 0x680(%rbp)
14000f01a:     	movq	0x7b0(%rbp), %rax
14000f021:     	movq	%rax, 0x688(%rbp)
14000f028:     	movq	%rbx, 0x690(%rbp)
14000f02f:     	testq	%rbx, %rbx
14000f032:     	movl	$0x0, %r14d
14000f038:     	je	0x14000f044 <.text+0xe044>
14000f03a:     	decq	%rbx
14000f03d:     	movq	%rbx, 0x690(%rbp)
14000f044:     	movq	0x668(%rbp), %rcx
14000f04b:     	leaq	(%rcx,%r13,8), %rax
14000f04f:     	movq	%rcx, 0x590(%rbp)
14000f056:     	movq	%rcx, 0x598(%rbp)
14000f05d:     	movq	0x578(%rbp), %rcx
14000f064:     	movq	%rcx, 0x5a0(%rbp)
14000f06b:     	movq	%rax, 0x5a8(%rbp)
14000f072:     	movq	$0x1, 0x5b0(%rbp)
14000f07d:     	movb	$0x1, 0x7d9(%rbp)
14000f084:     	movb	$0x1, 0x7d8(%rbp)
14000f08b:     	movb	$0x1, 0x7d7(%rbp)
14000f092:     	movb	$0x1, 0x7d6(%rbp)
14000f099:     	leaq	0x680(%rbp), %rcx
14000f0a0:     	leaq	0x590(%rbp), %rbx
14000f0a7:     	movq	%rbx, %rdx
14000f0aa:     	leaq	0x291755f(%rip), %r8    # 0x142926610
14000f0b1:     	callq	0x1402ba000 <.text+0x2b9000>
14000f0b6:     	movq	0x780(%rbp), %rax
14000f0bd:     	movq	%rax, 0x2a0(%rbp)
14000f0c4:     	movq	0x760(%rbp), %rax
14000f0cb:     	movq	%rax, 0x2a8(%rbp)
14000f0d2:     	movq	%rdi, 0x2b0(%rbp)
14000f0d9:     	testq	%rdi, %rdi
14000f0dc:     	movq	0x940(%rbp), %r13
14000f0e3:     	je	0x14000f0ef <.text+0xe0ef>
14000f0e5:     	decq	%rdi
14000f0e8:     	movq	%rdi, 0x2b0(%rbp)
14000f0ef:     	movq	0x7a0(%rbp), %rcx
14000f0f6:     	leaq	(%rcx,%r12,4), %rax
14000f0fa:     	movq	%rcx, 0x590(%rbp)
14000f101:     	movq	%rcx, 0x598(%rbp)
14000f108:     	movq	0x520(%rbp), %rcx
14000f10f:     	movq	%rcx, 0x5a0(%rbp)
14000f116:     	movq	%rax, 0x5a8(%rbp)
14000f11d:     	movq	$0x1, 0x5b0(%rbp)
14000f128:     	movb	$0x1, 0x7d5(%rbp)
14000f12f:     	movb	$0x1, 0x7d4(%rbp)
14000f136:     	movb	$0x1, 0x7d3(%rbp)
14000f13d:     	leaq	0x2a0(%rbp), %rcx
14000f144:     	movq	%rbx, %rdx
14000f147:     	leaq	0x29174da(%rip), %r8    # 0x142926628
14000f14e:     	callq	0x1402b9c50 <.text+0x2b8c50>
14000f153:     	movq	0x7e0(%rbp), %rax
14000f15a:     	movq	%rax, 0x3e0(%rbp)
14000f161:     	movq	0x778(%rbp), %rax
14000f168:     	movq	%rax, 0x3e8(%rbp)
14000f16f:     	movq	%rsi, 0x3f0(%rbp)
14000f176:     	testq	%rsi, %rsi
14000f179:     	leaq	0x460(%rbp), %r12
14000f180:     	je	0x14000f18c <.text+0xe18c>
14000f182:     	decq	%rsi
14000f185:     	movq	%rsi, 0x3f0(%rbp)
14000f18c:     	movq	0x798(%rbp), %rcx
14000f193:     	leaq	(%rcx,%r15,4), %rax
14000f197:     	movq	%rcx, 0x590(%rbp)
14000f19e:     	movq	%rcx, 0x598(%rbp)
14000f1a5:     	movq	0x670(%rbp), %rcx
14000f1ac:     	movq	%rcx, 0x5a0(%rbp)
14000f1b3:     	movq	%rax, 0x5a8(%rbp)
14000f1ba:     	movq	$0x1, 0x5b0(%rbp)
14000f1c5:     	movb	$0x1, 0x7d2(%rbp)
14000f1cc:     	movb	$0x1, 0x7d1(%rbp)
14000f1d3:     	leaq	0x3e0(%rbp), %rcx
14000f1da:     	movq	%rbx, %rdx
14000f1dd:     	leaq	0x291745c(%rip), %r8    # 0x142926640
14000f1e4:     	callq	0x1402b9c50 <.text+0x2b8c50>
14000f1e9:     	movq	0x660(%rbp), %rax
14000f1f0:     	movq	%rax, 0x5a0(%rbp)
14000f1f7:     	movdqu	0x650(%rbp), %xmm0
14000f1ff:     	movdqa	%xmm0, 0x590(%rbp)
14000f207:     	movb	$0x1, 0x7d2(%rbp)
14000f20e:     	movb	$0x0, 0x7d1(%rbp)
14000f215:     	leaq	0x380(%rbp), %rcx
14000f21c:     	movq	%rbx, %rdx
14000f21f:     	movl	$0x1, %r8d
14000f225:     	callq	0x140153640 <.text+0x152640>
14000f22a:     	cmpq	0x380(%rbp), %r14
14000f231:     	jno	0x14000f2c0 <.text+0xe2c0>
14000f237:     	movq	0x3e0(%rbp), %rdx
14000f23e:     	testq	%rdx, %rdx
14000f241:     	je	0x14000f259 <.text+0xe259>
14000f243:     	movq	0x3e8(%rbp), %rcx
14000f24a:     	shlq	$0x2, %rdx
14000f24e:     	movl	$0x4, %r8d
14000f254:     	callq	0x140613c20 <.text+0x612c20>
14000f259:     	movq	0x2a0(%rbp), %rdx
14000f260:     	testq	%rdx, %rdx
14000f263:     	movq	0x570(%rbp), %rsi
14000f26a:     	movq	0x7a8(%rbp), %rdi
14000f271:     	je	0x14000f289 <.text+0xe289>
14000f273:     	movq	0x2a8(%rbp), %rcx
14000f27a:     	shlq	$0x2, %rdx
14000f27e:     	movl	$0x4, %r8d
14000f284:     	callq	0x140613c20 <.text+0x612c20>
14000f289:     	movq	0x680(%rbp), %rdx
14000f290:     	testq	%rdx, %rdx
14000f293:     	je	0x14000f4d9 <.text+0xe4d9>
14000f299:     	movq	0x688(%rbp), %rcx
14000f2a0:     	shlq	$0x3, %rdx
14000f2a4:     	movl	$0x4, %r8d
14000f2aa:     	callq	0x140613c20 <.text+0x612c20>
14000f2af:     	jmp	0x14000f4d9 <.text+0xe4d9>
14000f2b4:     	nopw	%cs:(%rax,%rax)
14000f2c0:     	movaps	%xmm6, 0x290(%rbp)
14000f2c7:     	movq	0x3b0(%rbp), %rax
14000f2ce:     	movq	%rax, 0x5c0(%rbp)
14000f2d5:     	movups	0x380(%rbp), %xmm0
14000f2dc:     	movups	0x390(%rbp), %xmm1
14000f2e3:     	movups	0x3a0(%rbp), %xmm2
14000f2ea:     	movaps	%xmm2, 0x5b0(%rbp)
14000f2f1:     	movaps	%xmm1, 0x5a0(%rbp)
14000f2f8:     	movaps	%xmm0, 0x590(%rbp)
14000f2ff:     	movq	0x690(%rbp), %rax
14000f306:     	leaq	0x598(%rbp), %rcx
14000f30d:     	movq	%rax, 0x40(%rcx)
14000f311:     	movups	0x680(%rbp), %xmm0
14000f318:     	movups	%xmm0, 0x30(%rcx)
14000f31c:     	movq	0x2b0(%rbp), %rax
14000f323:     	movq	%rax, 0x58(%rcx)
14000f327:     	movups	0x2a0(%rbp), %xmm0
14000f32e:     	movups	%xmm0, 0x48(%rcx)
14000f332:     	movq	0x3f0(%rbp), %rax
14000f339:     	movq	%rax, 0x70(%rcx)
14000f33d:     	movups	0x3e0(%rbp), %xmm0
14000f344:     	movups	%xmm0, 0x60(%rcx)
14000f348:     	movaps	0x590(%rbp), %xmm0
14000f34f:     	movaps	0x5a0(%rbp), %xmm1
14000f356:     	movaps	0x5b0(%rbp), %xmm2
14000f35d:     	movaps	0x5c0(%rbp), %xmm3
14000f364:     	movaps	%xmm0, 0x460(%rbp)
14000f36b:     	movaps	%xmm1, 0x470(%rbp)
14000f372:     	movaps	%xmm2, 0x480(%rbp)
14000f379:     	movaps	%xmm3, 0x490(%rbp)
14000f380:     	movaps	0x5d0(%rbp), %xmm0
14000f387:     	movaps	%xmm0, 0x4a0(%rbp)
14000f38e:     	movaps	0x5e0(%rbp), %xmm0
14000f395:     	movaps	%xmm0, 0x4b0(%rbp)
14000f39c:     	movaps	0x5f0(%rbp), %xmm0
14000f3a3:     	movaps	%xmm0, 0x4c0(%rbp)
14000f3aa:     	movaps	0x600(%rbp), %xmm0
14000f3b1:     	movaps	%xmm0, 0x4d0(%rbp)
14000f3b8:     	cmpq	0x6a0(%rbp), %r14
14000f3bf:     	jo	0x14000f3cd <.text+0xe3cd>
14000f3c1:     	leaq	0x6a0(%rbp), %rcx
14000f3c8:     	callq	0x140609eb0 <.text+0x608eb0>
14000f3cd:     	movaps	0x4d0(%rbp), %xmm0
14000f3d4:     	movaps	%xmm0, 0x710(%rbp)
14000f3db:     	movaps	0x4c0(%rbp), %xmm0
14000f3e2:     	movaps	%xmm0, 0x700(%rbp)
14000f3e9:     	movaps	0x4b0(%rbp), %xmm0
14000f3f0:     	movaps	%xmm0, 0x6f0(%rbp)
14000f3f7:     	movaps	0x4a0(%rbp), %xmm0
14000f3fe:     	movaps	%xmm0, 0x6e0(%rbp)
14000f405:     	movaps	0x460(%rbp), %xmm0
14000f40c:     	movdqa	0x470(%rbp), %xmm1
14000f414:     	movdqa	0x480(%rbp), %xmm2
14000f41c:     	movapd	0x490(%rbp), %xmm3
14000f424:     	movapd	%xmm3, 0x6d0(%rbp)
14000f42c:     	movdqa	%xmm2, 0x6c0(%rbp)
14000f434:     	movdqa	%xmm1, 0x6b0(%rbp)
14000f43c:     	movaps	%xmm0, 0x6a0(%rbp)
14000f443:     	movq	0x7b8(%rbp), %rax
14000f44a:     	movq	%rax, 0x720(%rbp)
14000f451:     	movl	0x588(%rbp), %eax
14000f457:     	movl	%eax, 0x728(%rbp)
14000f45d:     	movl	0x560(%rbp), %eax
14000f463:     	movl	%eax, 0x72c(%rbp)
14000f469:     	movl	0x580(%rbp), %eax
14000f46f:     	movl	%eax, 0x730(%rbp)
14000f475:     	movdqa	0x290(%rbp), %xmm0
14000f47d:     	movd	%xmm0, %eax
14000f481:     	movw	%ax, 0x734(%rbp)
14000f488:     	movw	$0x0, 0x736(%rbp)
14000f491:     	movzwl	0x430(%rbp), %eax
14000f498:     	movw	%ax, 0x738(%rbp)
14000f49f:     	movw	$0x300, 0x73a(%rbp)     # imm = 0x300
14000f4a8:     	movq	0x330(%rbp), %rax
14000f4af:     	movq	%rax, 0x740(%rbp)
14000f4b6:     	movq	0x530(%rbp), %rax
14000f4bd:     	movq	%rax, 0x748(%rbp)
14000f4c4:     	movb	$0x0, 0x750(%rbp)
14000f4cb:     	movq	0x570(%rbp), %rsi
14000f4d2:     	movq	0x7a8(%rbp), %rdi
14000f4d9:     	testq	%rdi, %rdi
14000f4dc:     	je	0x14000f4f7 <.text+0xe4f7>
14000f4de:     	shlq	$0x2, %rdi
14000f4e2:     	movl	$0x4, %r8d
14000f4e8:     	movq	0x568(%rbp), %rcx
14000f4ef:     	movq	%rdi, %rdx
14000f4f2:     	callq	0x140613c20 <.text+0x612c20>
14000f4f7:     	testq	%rsi, %rsi
14000f4fa:     	je	0x14000ecf0 <.text+0xdcf0>
14000f500:     	shlq	$0x2, %rsi
14000f504:     	movl	$0x4, %r8d
14000f50a:     	movq	0x528(%rbp), %rcx
14000f511:     	movq	%rsi, %rdx
14000f514:     	callq	0x140613c20 <.text+0x612c20>
14000f519:     	jmp	0x14000ecf0 <.text+0xdcf0>
14000f51e:     	leaq	0x1(%rdx), %r9
14000f522:     	cmpq	%r8, %r9
14000f525:     	movl	$0x0, %r9d
14000f52b:     	cmovaeq	%r8, %r9
14000f52f:     	negq	%r9
14000f532:     	leaq	(%rdx,%r9), %r8
14000f536:     	incq	%r8
14000f539:     	movq	%r8, 0x450(%rbp)
14000f540:     	decq	%rcx
14000f543:     	movq	%rcx, 0x458(%rbp)
14000f54a:     	imulq	$0xb8, %rdx, %rcx
14000f551:     	movq	(%rax,%rcx), %r15
14000f555:     	leaq	(%rax,%rcx), %rdx
14000f559:     	addq	$0x8, %rdx
14000f55d:     	movl	$0xb0, %r8d
14000f563:     	leaq	0x590(%rbp), %r12
14000f56a:     	movq	%r12, %rcx
14000f56d:     	movl	%r11d, %r13d
14000f570:     	callq	0x142923c00 <.text+0x2922c00>
14000f575:     	movq	%r15, 0x6a0(%rbp)
14000f57c:     	movq	0x570(%rbp), %r15
14000f583:     	movl	$0xb0, %r8d
14000f589:     	leaq	0x6a8(%rbp), %rcx
14000f590:     	movq	%r12, %rdx
14000f593:     	leaq	0x460(%rbp), %r12
14000f59a:     	callq	0x142923c00 <.text+0x2922c00>
14000f59f:     	movl	%r13d, %r11d
14000f5a2:     	movq	0x940(%rbp), %r13
14000f5a9:     	movq	0x7e0(%rbp), %rdx
14000f5b0:     	movq	0x648(%rbp), %r8
14000f5b7:     	movl	0x580(%rbp), %r9d
14000f5be:     	movl	0x560(%rbp), %r10d
14000f5c5:     	movq	0x800(%rbp), %rax
14000f5cc:     	movq	%rax, 0x460(%rbp)
14000f5d3:     	movq	0x7f0(%rbp), %rax
14000f5da:     	movq	%rax, 0x468(%rbp)
14000f5e1:     	movq	%r14, 0x470(%rbp)
14000f5e8:     	movq	%r15, 0x478(%rbp)
14000f5ef:     	movq	0x528(%rbp), %rax
14000f5f6:     	movq	%rax, 0x480(%rbp)
14000f5fd:     	movaps	0x310(%rbp), %xmm0
14000f604:     	leaq	0x468(%rbp), %rcx
14000f60b:     	movups	%xmm0, 0x20(%rcx)
14000f60f:     	movq	%r8, 0x498(%rbp)
14000f616:     	movq	0x7b0(%rbp), %rax
14000f61d:     	movq	%rax, 0x4a0(%rbp)
14000f624:     	movq	%rbx, 0x4a8(%rbp)
14000f62b:     	movq	0x780(%rbp), %rax
14000f632:     	movq	%rax, 0x4b0(%rbp)
14000f639:     	movq	0x760(%rbp), %r8
14000f640:     	movq	%r8, 0x4b8(%rbp)
14000f647:     	movq	%rdi, 0x4c0(%rbp)
14000f64e:     	movq	%rdx, 0x4c8(%rbp)
14000f655:     	movq	0x778(%rbp), %rax
14000f65c:     	movq	%rax, 0x4d0(%rbp)
14000f663:     	movq	%rsi, 0x4d8(%rbp)
14000f66a:     	movq	0x7b8(%rbp), %rax
14000f671:     	movq	%rax, 0x4e0(%rbp)
14000f678:     	movl	0x588(%rbp), %eax
14000f67e:     	movl	%eax, 0x4e8(%rbp)
14000f684:     	movl	%r10d, 0x4ec(%rbp)
14000f68b:     	movl	%r9d, 0x4f0(%rbp)
14000f692:     	movss	%xmm6, 0x4f4(%rbp)
14000f69a:     	movw	%r11w, 0x4f8(%rbp)
14000f6a2:     	movzwl	0x54c(%rbp), %eax
14000f6a9:     	movw	%ax, 0x96(%rcx)
14000f6b0:     	movl	0x548(%rbp), %eax
14000f6b6:     	movl	%eax, 0x92(%rcx)
14000f6bc:     	testq	%rdi, %rdi
14000f6bf:     	je	0x1400113fa <.text+0x103fa>
14000f6c5:     	leaq	(%r8,%rdi,4), %rax
14000f6c9:     	addq	$-0x4, %rax
14000f6cd:     	leaq	0x590(%rbp), %rbx
14000f6d4:     	je	0x140011412 <.text+0x10412>
14000f6da:     	cmpb	$0x0, 0x538(%rbp)
14000f6e1:     	je	0x14000f7cb <.text+0xe7cb>
14000f6e7:     	movq	0x258(%rbp), %rax
14000f6ee:     	movl	0x424(%rbp), %ecx
14000f6f4:     	movl	%ecx, (%rax)
14000f6f6:     	movq	0x490(%rbp), %rax
14000f6fd:     	movq	%rax, 0x5c0(%rbp)
14000f704:     	movups	0x460(%rbp), %xmm0
14000f70b:     	movups	0x470(%rbp), %xmm1
14000f712:     	movups	0x480(%rbp), %xmm2
14000f719:     	movaps	%xmm2, 0x5b0(%rbp)
14000f720:     	movaps	%xmm1, 0x5a0(%rbp)
14000f727:     	movaps	%xmm0, 0x590(%rbp)
14000f72e:     	leaq	0x468(%rbp), %rdx
14000f735:     	movq	0x40(%rdx), %rax
14000f739:     	leaq	0x598(%rbp), %rcx
14000f740:     	movq	%rax, 0x40(%rcx)
14000f744:     	movups	0x30(%rdx), %xmm0
14000f748:     	movups	%xmm0, 0x30(%rcx)
14000f74c:     	movq	0x58(%rdx), %rax
14000f750:     	movq	%rax, 0x70(%rcx)
14000f754:     	movups	0x48(%rdx), %xmm0
14000f758:     	movups	%xmm0, 0x60(%rcx)
14000f75c:     	movq	0x70(%rdx), %rax
14000f760:     	movq	%rax, 0x58(%rcx)
14000f764:     	movups	0x60(%rdx), %xmm0
14000f768:     	movups	%xmm0, 0x48(%rcx)
14000f76c:     	movq	0x758(%rbp), %rax
14000f773:     	movl	%eax, 0x630(%rbp)
14000f779:     	movq	0x678(%rbp), %rax
14000f780:     	movq	(%rax), %rax
14000f783:     	movq	%rax, 0x628(%rbp)
14000f78a:     	movw	%r11w, 0x634(%rbp)
14000f792:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14000f79c:     	movq	%rax, 0x610(%rbp)
14000f7a3:     	movq	0x348(%rbp), %rdi
14000f7aa:     	movq	0x10(%rdi), %rsi
14000f7ae:     	cmpq	(%rdi), %rsi
14000f7b1:     	jne	0x14000ecc8 <.text+0xdcc8>
14000f7b7:     	movq	%rdi, %rcx
14000f7ba:     	leaq	0x2916fcf(%rip), %rdx   # 0x142926790
14000f7c1:     	callq	0x1405636c0 <.text+0x5626c0>
14000f7c6:     	jmp	0x14000ecc8 <.text+0xdcc8>
14000f7cb:     	movss	(%r8), %xmm0
14000f7d0:     	movss	(%rax), %xmm1
14000f7d4:     	movaps	%xmm0, %xmm2
14000f7d7:     	cmpunordss	%xmm0, %xmm2
14000f7dc:     	movaps	%xmm2, %xmm3
14000f7df:     	andps	%xmm1, %xmm3
14000f7e2:     	maxss	%xmm0, %xmm1
14000f7e6:     	andnps	%xmm1, %xmm2
14000f7e9:     	orps	%xmm3, %xmm2
14000f7ec:     	movss	0x29160dc(%rip), %xmm0  # 0x1429258d0
14000f7f4:     	ucomiss	%xmm2, %xmm0
14000f7f7:     	jb	0x14000f832 <.text+0xe832>
14000f7f9:     	movb	$0x1, 0x814(%rbp)
14000f800:     	movq	0x2e8(%rbp), %rcx
14000f807:     	callq	0x140959ef0 <.text+0x958ef0>
14000f80c:     	movq	%rax, %rdi
14000f80f:     	movb	$0x1, 0x814(%rbp)
14000f816:     	movq	0x118(%rbp), %rcx
14000f81d:     	callq	0x140959ef0 <.text+0x958ef0>
14000f822:     	testq	%rdi, %rdi
14000f825:     	sete	%cl
14000f828:     	testq	%rax, %rax
14000f82b:     	sete	%dl
14000f82e:     	orb	%cl, %dl
14000f830:     	je	0x14000f888 <.text+0xe888>
14000f832:     	movq	0x340(%rbp), %rax
14000f839:     	movl	0x53c(%rbp), %ecx
14000f83f:     	movl	%ecx, (%rax)
14000f841:     	movl	$0xa0, %r8d
14000f847:     	movq	%rbx, %rcx
14000f84a:     	movq	%r12, %rdx
14000f84d:     	callq	0x142923c00 <.text+0x2922c00>
14000f852:     	movq	0x370(%rbp), %rdi
14000f859:     	movq	0x10(%rdi), %rsi
14000f85d:     	cmpq	(%rdi), %rsi
14000f860:     	jne	0x14000f871 <.text+0xe871>
14000f862:     	movq	%rdi, %rcx
14000f865:     	leaq	0x2916f0c(%rip), %rdx   # 0x142926778
14000f86c:     	callq	0x140563480 <.text+0x562480>
14000f871:     	leaq	(%rsi,%rsi,4), %rcx
14000f875:     	shlq	$0x5, %rcx
14000f879:     	addq	0x8(%rdi), %rcx
14000f87d:     	movl	$0xa0, %r8d
14000f883:     	jmp	0x14000ecd9 <.text+0xdcd9>
14000f888:     	movq	(%rdi), %rcx
14000f88b:     	cmpq	(%rax), %rcx
14000f88e:     	jne	0x14000f89d <.text+0xe89d>
14000f890:     	movzwl	0x4f8(%rbp), %r11d
14000f898:     	jmp	0x14000f6e7 <.text+0xe6e7>
14000f89d:     	movb	$0x1, 0x814(%rbp)
14000f8a4:     	movq	%rdi, 0x20(%rsp)
14000f8a9:     	movq	%rbx, %rcx
14000f8ac:     	movq	0x378(%rbp), %rdx
14000f8b3:     	movq	0x938(%rbp), %r8
14000f8ba:     	movq	%r12, %r9
14000f8bd:     	callq	0x1407d94a0 <.text+0x7d84a0>
14000f8c2:     	movq	0x590(%rbp), %rax
14000f8c9:     	movq	%rax, 0x7a8(%rbp)
14000f8d0:     	movq	0x598(%rbp), %r13
14000f8d7:     	movq	0x5a0(%rbp), %rcx
14000f8de:     	movq	0x5a8(%rbp), %rax
14000f8e5:     	movq	%rax, 0x560(%rbp)
14000f8ec:     	movq	0x5b0(%rbp), %rax
14000f8f3:     	movq	%rax, 0x530(%rbp)
14000f8fa:     	movq	0x5b8(%rbp), %rax
14000f901:     	movq	%rax, 0x670(%rbp)
14000f908:     	movq	0x5c0(%rbp), %rax
14000f90f:     	movq	%rax, 0x580(%rbp)
14000f916:     	movq	0x5c8(%rbp), %rax
14000f91d:     	movq	%rax, 0x568(%rbp)
14000f924:     	movq	0x5d0(%rbp), %rax
14000f92b:     	movq	%rax, 0x330(%rbp)
14000f932:     	shlq	$0x4, %rcx
14000f936:     	addq	%r13, %rcx
14000f939:     	movq	%rcx, 0x520(%rbp)
14000f940:     	movq	%r13, 0x798(%rbp)
14000f947:     	nopw	(%rax,%rax)
14000f950:     	movq	%r13, %r12
14000f953:     	cmpq	0x520(%rbp), %r12
14000f95a:     	movq	%r12, 0x578(%rbp)
14000f961:     	je	0x14000fe23 <.text+0xee23>
14000f967:     	movq	0x8(%r12), %r14
14000f96c:     	cmpq	$-0x1, %r14
14000f970:     	je	0x140011001 <.text+0x10001>
14000f976:     	movq	(%r12), %r13
14000f97a:     	leaq	0x1(%r14), %rax
14000f97e:     	movq	%rax, %rdi
14000f981:     	subq	%r13, %rdi
14000f984:     	movq	0x80(%rbp), %r15
14000f98b:     	jb	0x140011012 <.text+0x10012>
14000f991:     	movq	0x470(%rbp), %rdx
14000f998:     	cmpq	%rdx, %r14
14000f99b:     	jae	0x140011029 <.text+0x10029>
14000f9a1:     	movq	%rax, 0x7a0(%rbp)
14000f9a8:     	movq	%rdi, %rax
14000f9ab:     	movl	$0xc, %ecx
14000f9b0:     	mulq	%rcx
14000f9b3:     	jo	0x14001142a <.text+0x1042a>
14000f9b9:     	movl	$0x0, %esi
14000f9be:     	movabsq	$0x7ffffffffffffffc, %rcx # imm = 0x7FFFFFFFFFFFFFFC
14000f9c8:     	cmpq	%rcx, %rax
14000f9cb:     	ja	0x14001142c <.text+0x1042c>
14000f9d1:     	movq	0x468(%rbp), %rbx
14000f9d8:     	testq	%rax, %rax
14000f9db:     	je	0x14000fa10 <.text+0xea10>
14000f9dd:     	movzbl	0x3625225(%rip), %ecx   # 0x143634c09
14000f9e4:     	movl	$0x4, %esi
14000f9e9:     	movl	$0x4, %edx
14000f9ee:     	movq	%rax, %rcx
14000f9f1:     	movq	%rax, %r15
14000f9f4:     	callq	0x140613c10 <.text+0x612c10>
14000f9f9:     	testq	%rax, %rax
14000f9fc:     	je	0x14001142c <.text+0x1042c>
14000fa02:     	movq	%rdi, %rsi
14000fa05:     	movq	%rax, %r15
14000fa08:     	jmp	0x14000fa18 <.text+0xea18>
14000fa0a:     	nopw	(%rax,%rax)
14000fa10:     	movl	$0x4, %r15d
14000fa16:     	xorl	%esi, %esi
14000fa18:     	movq	%r13, 0x668(%rbp)
14000fa1f:     	leaq	(,%r13,2), %rax
14000fa27:     	addq	%r13, %rax
14000fa2a:     	leaq	(%rbx,%rax,4), %rdx
14000fa2e:     	leaq	(,%rdi,4), %r12
14000fa36:     	leaq	(%r12,%r12,2), %r8
14000fa3a:     	movq	%r15, %rcx
14000fa3d:     	callq	0x142923c00 <.text+0x2922c00>
14000fa42:     	movq	%rsi, 0x380(%rbp)
14000fa49:     	movq	%r15, 0x388(%rbp)
14000fa50:     	movq	%rdi, 0x390(%rbp)
14000fa57:     	leaq	0x590(%rbp), %rcx
14000fa5e:     	leaq	0x380(%rbp), %rdx
14000fa65:     	movl	$0x1, %r8d
14000fa6b:     	callq	0x140153640 <.text+0x152640>
14000fa70:     	movq	%r15, 0x80(%rbp)
14000fa77:     	movq	0x578(%rbp), %rax
14000fa7e:     	leaq	0x10(%rax), %r13
14000fa82:     	movq	0x590(%rbp), %rax
14000fa89:     	movq	%rax, %rcx
14000fa8c:     	negq	%rcx
14000fa8f:     	jo	0x14000f950 <.text+0xe950>
14000fa95:     	movq	0x598(%rbp), %rcx
14000fa9c:     	movq	0x5a8(%rbp), %r15
14000faa3:     	movq	0x5b0(%rbp), %rbx
14000faaa:     	movss	0x5c0(%rbp), %xmm0
14000fab2:     	ucomiss	0x2915ed3(%rip), %xmm0  # 0x14292598c
14000fab9:     	ja	0x14000faf9 <.text+0xeaf9>
14000fabb:     	testq	%rax, %rax
14000fabe:     	je	0x14000fad3 <.text+0xead3>
14000fac0:     	shlq	$0x2, %rax
14000fac4:     	leaq	(%rax,%rax,2), %rdx
14000fac8:     	movl	$0x4, %r8d
14000face:     	callq	0x140613c20 <.text+0x612c20>
14000fad3:     	movq	%r13, %r12
14000fad6:     	testq	%r15, %r15
14000fad9:     	je	0x14000f953 <.text+0xe953>
14000fadf:     	shlq	$0x2, %r15
14000fae3:     	movl	$0x4, %r8d
14000fae9:     	movq	%rbx, %rcx
14000faec:     	movq	%r15, %rdx
14000faef:     	callq	0x140613c20 <.text+0x612c20>
14000faf4:     	jmp	0x14000f950 <.text+0xe950>
14000faf9:     	movq	0x5a0(%rbp), %rdx
14000fb00:     	movq	0x5b8(%rbp), %r8
14000fb07:     	movl	0x5c4(%rbp), %r9d
14000fb0e:     	movq	0x340(%rbp), %r10
14000fb15:     	movl	0x53c(%rbp), %r11d
14000fb1c:     	movl	%r11d, (%r10)
14000fb1f:     	movq	%rax, 0x380(%rbp)
14000fb26:     	movq	%rcx, 0x388(%rbp)
14000fb2d:     	movq	%rdx, 0x390(%rbp)
14000fb34:     	movq	%r15, 0x398(%rbp)
14000fb3b:     	movq	%rbx, 0x3a0(%rbp)
14000fb42:     	movq	%r8, 0x3a8(%rbp)
14000fb49:     	movss	%xmm0, 0x3b0(%rbp)
14000fb51:     	movl	%r9d, 0x3b4(%rbp)
14000fb58:     	movq	0x4a8(%rbp), %rdx
14000fb5f:     	cmpq	%rdx, %r14
14000fb62:     	jae	0x140011207 <.text+0x10207>
14000fb68:     	movq	%rdi, %rax
14000fb6b:     	shrq	$0x3c, %rax
14000fb6f:     	jne	0x140011664 <.text+0x10664>
14000fb75:     	movq	0x668(%rbp), %rax
14000fb7c:     	leaq	(,%rax,8), %r15
14000fb84:     	addq	0x4a0(%rbp), %r15
14000fb8b:     	leaq	(,%rdi,8), %rbx
14000fb93:     	testq	%rdi, %rdi
14000fb96:     	je	0x14000fbc3 <.text+0xebc3>
14000fb98:     	movzbl	0x362506a(%rip), %eax   # 0x143634c09
14000fb9f:     	movl	$0x4, %esi
14000fba4:     	movl	$0x4, %edx
14000fba9:     	movq	%rbx, %rcx
14000fbac:     	callq	0x140613c10 <.text+0x612c10>
14000fbb1:     	testq	%rax, %rax
14000fbb4:     	je	0x140011741 <.text+0x10741>
14000fbba:     	movq	%rdi, 0x588(%rbp)
14000fbc1:     	jmp	0x14000fbd3 <.text+0xebd3>
14000fbc3:     	movl	$0x4, %eax
14000fbc8:     	movq	$0x0, 0x588(%rbp)
14000fbd3:     	movq	%rax, 0x2f8(%rbp)
14000fbda:     	movq	%rax, %rcx
14000fbdd:     	movq	%r15, %rdx
14000fbe0:     	movq	%rbx, %r8
14000fbe3:     	callq	0x142923c00 <.text+0x2922c00>
14000fbe8:     	movq	0x4c0(%rbp), %rdx
14000fbef:     	cmpq	%rdx, %r14
14000fbf2:     	jae	0x14001121f <.text+0x1021f>
14000fbf8:     	movq	0x668(%rbp), %rax
14000fbff:     	leaq	(,%rax,4), %rsi
14000fc07:     	addq	0x4b8(%rbp), %rsi
14000fc0e:     	testq	%rdi, %rdi
14000fc11:     	je	0x14000fc8b <.text+0xec8b>
14000fc13:     	movzbl	0x3624fef(%rip), %eax   # 0x143634c09
14000fc1a:     	movl	$0x4, %edx
14000fc1f:     	movq	%r12, %rcx
14000fc22:     	callq	0x140613c10 <.text+0x612c10>
14000fc27:     	testq	%rax, %rax
14000fc2a:     	je	0x140011701 <.text+0x10701>
14000fc30:     	movq	%rax, %rbx
14000fc33:     	movq	%rax, %rcx
14000fc36:     	movq	%rsi, %rdx
14000fc39:     	movq	%r12, %r8
14000fc3c:     	callq	0x142923c00 <.text+0x2922c00>
14000fc41:     	movq	0x4d8(%rbp), %rdx
14000fc48:     	cmpq	%rdx, %r14
14000fc4b:     	jae	0x140011332 <.text+0x10332>
14000fc51:     	movq	%rbx, %rsi
14000fc54:     	movq	0x4d0(%rbp), %rbx
14000fc5b:     	movzbl	0x3624fa7(%rip), %eax   # 0x143634c09
14000fc62:     	movl	$0x4, %edx
14000fc67:     	movq	%r12, %rcx
14000fc6a:     	callq	0x140613c10 <.text+0x612c10>
14000fc6f:     	testq	%rax, %rax
14000fc72:     	je	0x14001171a <.text+0x1071a>
14000fc78:     	movq	%rax, %r15
14000fc7b:     	movq	%rdi, 0x7a0(%rbp)
14000fc82:     	movq	%rsi, 0x7b8(%rbp)
14000fc89:     	jmp	0x14000fcd4 <.text+0xecd4>
14000fc8b:     	movl	$0x4, %ebx
14000fc90:     	movl	$0x4, %ecx
14000fc95:     	movq	%rsi, %rdx
14000fc98:     	movq	%r12, %r8
14000fc9b:     	callq	0x142923c00 <.text+0x2922c00>
14000fca0:     	movq	0x4d8(%rbp), %rdx
14000fca7:     	cmpq	%rdx, %r14
14000fcaa:     	jae	0x140011330 <.text+0x10330>
14000fcb0:     	movl	$0x4, %eax
14000fcb5:     	movq	%rax, 0x7b8(%rbp)
14000fcbc:     	movq	0x4d0(%rbp), %rbx
14000fcc3:     	movq	$0x0, 0x7a0(%rbp)
14000fcce:     	movl	$0x4, %r15d
14000fcd4:     	movq	0x668(%rbp), %rsi
14000fcdb:     	leaq	(%rbx,%rsi,4), %rdx
14000fcdf:     	movq	%r15, %rcx
14000fce2:     	movq	%r12, %r8
14000fce5:     	callq	0x142923c00 <.text+0x2922c00>
14000fcea:     	movq	0x470(%rbp), %rax
14000fcf1:     	decq	%rax
14000fcf4:     	testq	%rsi, %rsi
14000fcf7:     	movups	0x380(%rbp), %xmm0
14000fcfe:     	movups	0x390(%rbp), %xmm1
14000fd05:     	movups	0x3a0(%rbp), %xmm2
14000fd0c:     	movaps	%xmm0, 0x590(%rbp)
14000fd13:     	movaps	%xmm1, 0x5a0(%rbp)
14000fd1a:     	movaps	%xmm2, 0x5b0(%rbp)
14000fd21:     	movq	0x3b0(%rbp), %rcx
14000fd28:     	movq	%rcx, 0x5c0(%rbp)
14000fd2f:     	movq	0x588(%rbp), %rcx
14000fd36:     	movq	%rcx, 0x5c8(%rbp)
14000fd3d:     	movq	0x2f8(%rbp), %rcx
14000fd44:     	movq	%rcx, 0x5d0(%rbp)
14000fd4b:     	movq	%rdi, 0x5d8(%rbp)
14000fd52:     	movq	0x7a0(%rbp), %rcx
14000fd59:     	movq	%rcx, 0x5e0(%rbp)
14000fd60:     	movq	0x7b8(%rbp), %rdx
14000fd67:     	movq	%rdx, 0x5e8(%rbp)
14000fd6e:     	movq	%rdi, 0x5f0(%rbp)
14000fd75:     	movq	%rcx, 0x5f8(%rbp)
14000fd7c:     	movq	%r15, 0x600(%rbp)
14000fd83:     	movq	%rdi, 0x608(%rbp)
14000fd8a:     	movq	0x438(%rbp), %rcx
14000fd91:     	movq	%rcx, 0x610(%rbp)
14000fd98:     	movw	$0x101, 0x624(%rbp)     # imm = 0x101
14000fda1:     	movq	0x758(%rbp), %rcx
14000fda8:     	movl	%ecx, 0x620(%rbp)
14000fdae:     	movq	0x678(%rbp), %rcx
14000fdb5:     	movq	(%rcx), %rcx
14000fdb8:     	setne	0x626(%rbp)
14000fdbf:     	cmpq	%rax, %r14
14000fdc2:     	movq	%rcx, 0x618(%rbp)
14000fdc9:     	setne	0x627(%rbp)
14000fdd0:     	movl	$0x3000000, 0x628(%rbp) # imm = 0x3000000
14000fdda:     	movq	0x370(%rbp), %rdi
14000fde1:     	movq	0x10(%rdi), %rsi
14000fde5:     	cmpq	(%rdi), %rsi
14000fde8:     	jne	0x14000fdf9 <.text+0xedf9>
14000fdea:     	movq	%rdi, %rcx
14000fded:     	leaq	0x291696c(%rip), %rdx   # 0x142926760
14000fdf4:     	callq	0x140563480 <.text+0x562480>
14000fdf9:     	leaq	(%rsi,%rsi,4), %rcx
14000fdfd:     	shlq	$0x5, %rcx
14000fe01:     	addq	0x8(%rdi), %rcx
14000fe05:     	movl	$0xa0, %r8d
14000fe0b:     	leaq	0x590(%rbp), %rdx
14000fe12:     	callq	0x142923c10 <.text+0x2922c10>
14000fe17:     	incq	%rsi
14000fe1a:     	movq	%rsi, 0x10(%rdi)
14000fe1e:     	jmp	0x14000f950 <.text+0xe950>
14000fe23:     	movq	0x7a8(%rbp), %rdx
14000fe2a:     	testq	%rdx, %rdx
14000fe2d:     	leaq	0x590(%rbp), %rbx
14000fe34:     	movq	0x330(%rbp), %rsi
14000fe3b:     	je	0x14000fe53 <.text+0xee53>
14000fe3d:     	shlq	$0x4, %rdx
14000fe41:     	movl	$0x8, %r8d
14000fe47:     	movq	0x798(%rbp), %rcx
14000fe4e:     	callq	0x140613c20 <.text+0x612c20>
14000fe53:     	movq	0x670(%rbp), %rdx
14000fe5a:     	shlq	$0x4, %rdx
14000fe5e:     	movq	0x530(%rbp), %rcx
14000fe65:     	addq	%rcx, %rdx
14000fe68:     	leaq	(%rsi,%rsi,2), %rax
14000fe6c:     	movq	0x568(%rbp), %r13
14000fe73:     	leaq	(,%rax,8), %r8
14000fe7b:     	addq	%r13, %r8
14000fe7e:     	leaq	0x3c0(%rbp), %rax
14000fe85:     	movups	%xmm8, (%rax)
14000fe89:     	movq	$0x0, 0x10(%rax)
14000fe91:     	movq	%rcx, 0x380(%rbp)
14000fe98:     	movq	%rcx, 0x388(%rbp)
14000fe9f:     	movq	0x560(%rbp), %rax
14000fea6:     	movq	%rax, 0x390(%rbp)
14000fead:     	movq	%rdx, 0x670(%rbp)
14000feb4:     	movq	%rdx, 0x398(%rbp)
14000febb:     	movq	%r13, 0x3a0(%rbp)
14000fec2:     	movq	%r13, 0x3a8(%rbp)
14000fec9:     	movq	0x580(%rbp), %rax
14000fed0:     	movq	%rax, 0x3b0(%rbp)
14000fed7:     	movq	%r8, 0x798(%rbp)
14000fede:     	movq	%r8, 0x3b8(%rbp)
14000fee5:     	movq	%r13, %r12
14000fee8:     	movq	%rcx, %r14
14000feeb:     	nopl	(%rax,%rax)
14000fef0:     	movq	%r14, %rax
14000fef3:     	cmpq	0x670(%rbp), %rax
14000fefa:     	je	0x140010409 <.text+0xf409>
14000ff00:     	leaq	0x10(%rax), %r13
14000ff04:     	movq	%r13, 0x388(%rbp)
14000ff0b:     	cmpq	0x798(%rbp), %r12
14000ff12:     	je	0x140010402 <.text+0xf402>
14000ff18:     	movq	(%rax), %rbx
14000ff1b:     	movq	0x8(%rax), %r15
14000ff1f:     	leaq	0x18(%r12), %rdx
14000ff24:     	movq	%rdx, 0x3a8(%rbp)
14000ff2b:     	movq	(%r12), %rcx
14000ff2f:     	movq	%rcx, %rax
14000ff32:     	negq	%rax
14000ff35:     	jo	0x14001056c <.text+0xf56c>
14000ff3b:     	movq	0x8(%r12), %rax
14000ff40:     	movq	%rax, 0x7a8(%rbp)
14000ff47:     	cmpq	$-0x1, %r15
14000ff4b:     	movq	%rcx, 0x668(%rbp)
14000ff52:     	je	0x14001103d <.text+0x1003d>
14000ff58:     	movq	%rdx, 0x7a0(%rbp)
14000ff5f:     	leaq	0x1(%r15), %rax
14000ff63:     	movq	%rax, %rdi
14000ff66:     	subq	%rbx, %rdi
14000ff69:     	jb	0x140011055 <.text+0x10055>
14000ff6f:     	movq	0x470(%rbp), %rdx
14000ff76:     	cmpq	%rdx, %r15
14000ff79:     	jae	0x140011073 <.text+0x10073>
14000ff7f:     	movq	%rax, 0x7b8(%rbp)
14000ff86:     	movq	%rdi, %rax
14000ff89:     	movl	$0xc, %ecx
14000ff8e:     	mulq	%rcx
14000ff91:     	jo	0x1400114d3 <.text+0x104d3>
14000ff97:     	movl	$0x0, %esi
14000ff9c:     	movabsq	$0x7ffffffffffffffc, %rcx # imm = 0x7FFFFFFFFFFFFFFC
14000ffa6:     	cmpq	%rcx, %rax
14000ffa9:     	ja	0x1400114d5 <.text+0x104d5>
14000ffaf:     	movq	0x10(%r12), %rcx
14000ffb4:     	movq	%rcx, 0x588(%rbp)
14000ffbb:     	movq	0x468(%rbp), %r12
14000ffc2:     	testq	%rax, %rax
14000ffc5:     	je	0x140010000 <.text+0xf000>
14000ffc7:     	movzbl	0x3624c3b(%rip), %ecx   # 0x143634c09
14000ffce:     	movl	$0x4, %esi
14000ffd3:     	movl	$0x4, %edx
14000ffd8:     	movq	%rax, %rcx
14000ffdb:     	movq	%rax, %r14
14000ffde:     	callq	0x140613c10 <.text+0x612c10>
14000ffe3:     	movq	%r14, 0x100(%rbp)
14000ffea:     	testq	%rax, %rax
14000ffed:     	je	0x1400114d5 <.text+0x104d5>
14000fff3:     	movq	%rdi, %rsi
14000fff6:     	movq	%rax, %r14
14000fff9:     	jmp	0x140010008 <.text+0xf008>
14000fffb:     	nopl	(%rax,%rax)
140010000:     	movl	$0x4, %r14d
140010006:     	xorl	%esi, %esi
140010008:     	leaq	(%rbx,%rbx,2), %rax
14001000c:     	leaq	(%r12,%rax,4), %rdx
140010010:     	leaq	(,%rdi,4), %r12
140010018:     	leaq	(%r12,%r12,2), %r8
14001001c:     	movq	%r14, %rcx
14001001f:     	callq	0x142923c00 <.text+0x2922c00>
140010024:     	movq	%rsi, 0x3e0(%rbp)
14001002b:     	movq	%r14, 0x100(%rbp)
140010032:     	movq	%r14, 0x3e8(%rbp)
140010039:     	movq	%rdi, 0x3f0(%rbp)
140010040:     	movb	$0x1, 0x813(%rbp)
140010047:     	leaq	0x590(%rbp), %rcx
14001004e:     	leaq	0x3e0(%rbp), %rdx
140010055:     	movl	$0x1, %r8d
14001005b:     	callq	0x140153640 <.text+0x152640>
140010060:     	xorl	%eax, %eax
140010062:     	cmpq	0x590(%rbp), %rax
140010069:     	movq	%r13, %r14
14001006c:     	jno	0x1400100b0 <.text+0xf0b0>
14001006e:     	movq	0x7a0(%rbp), %rsi
140010075:     	movq	%rsi, %r12
140010078:     	movq	%r14, %rax
14001007b:     	movq	0x668(%rbp), %rdx
140010082:     	testq	%rdx, %rdx
140010085:     	leaq	0x590(%rbp), %rbx
14001008c:     	je	0x14000fef3 <.text+0xeef3>
140010092:     	shlq	$0x2, %rdx
140010096:     	movl	$0x4, %r8d
14001009c:     	movq	0x7a8(%rbp), %rcx
1400100a3:     	callq	0x140613c20 <.text+0x612c20>
1400100a8:     	movq	%rsi, %r12
1400100ab:     	jmp	0x14000fef0 <.text+0xeef0>
1400100b0:     	movq	0x5c0(%rbp), %rax
1400100b7:     	movq	%rax, 0x2d0(%rbp)
1400100be:     	movups	0x590(%rbp), %xmm0
1400100c5:     	movups	0x5a0(%rbp), %xmm1
1400100cc:     	movups	0x5b0(%rbp), %xmm2
1400100d3:     	movaps	%xmm2, 0x2c0(%rbp)
1400100da:     	movaps	%xmm1, 0x2b0(%rbp)
1400100e1:     	movaps	%xmm0, 0x2a0(%rbp)
1400100e8:     	movq	0x258(%rbp), %rax
1400100ef:     	movl	0x424(%rbp), %ecx
1400100f5:     	movl	%ecx, (%rax)
1400100f7:     	movq	0x2d0(%rbp), %rax
1400100fe:     	movq	%rax, 0x410(%rbp)
140010105:     	movaps	0x2a0(%rbp), %xmm0
14001010c:     	movaps	0x2b0(%rbp), %xmm1
140010113:     	movaps	0x2c0(%rbp), %xmm2
14001011a:     	movaps	%xmm2, 0x400(%rbp)
140010121:     	movaps	%xmm1, 0x3f0(%rbp)
140010128:     	movaps	%xmm0, 0x3e0(%rbp)
14001012f:     	movq	0x4a8(%rbp), %rdx
140010136:     	cmpq	%rdx, %r15
140010139:     	jae	0x140011281 <.text+0x10281>
14001013f:     	movq	%rdi, %rax
140010142:     	shrq	$0x3c, %rax
140010146:     	jne	0x1400116fa <.text+0x106fa>
14001014c:     	leaq	(,%rbx,8), %r13
140010154:     	addq	0x4a0(%rbp), %r13
14001015b:     	leaq	(,%rdi,8), %r8
140010163:     	testq	%rdi, %rdi
140010166:     	je	0x140010194 <.text+0xf194>
140010168:     	movzbl	0x3624a9a(%rip), %eax   # 0x143634c09
14001016f:     	movl	$0x4, %edx
140010174:     	movq	%r8, %rsi
140010177:     	movq	%r8, %rcx
14001017a:     	callq	0x140613c10 <.text+0x612c10>
14001017f:     	testq	%rax, %rax
140010182:     	je	0x14001185a <.text+0x1085a>
140010188:     	movq	%rdi, 0x430(%rbp)
14001018f:     	movq	%rsi, %r8
140010192:     	jmp	0x1400101a4 <.text+0xf1a4>
140010194:     	movl	$0x4, %eax
140010199:     	movq	$0x0, 0x430(%rbp)
1400101a4:     	movq	%rax, 0x300(%rbp)
1400101ab:     	movq	%rax, %rcx
1400101ae:     	movq	%r13, %rdx
1400101b1:     	callq	0x142923c00 <.text+0x2922c00>
1400101b6:     	movq	0x4c0(%rbp), %rdx
1400101bd:     	cmpq	%rdx, %r15
1400101c0:     	jae	0x140011299 <.text+0x10299>
1400101c6:     	leaq	(,%rbx,4), %rsi
1400101ce:     	addq	0x4b8(%rbp), %rsi
1400101d5:     	testq	%rdi, %rdi
1400101d8:     	je	0x140010250 <.text+0xf250>
1400101da:     	movq	0x430(%rbp), %r13
1400101e1:     	movzbl	0x3624a21(%rip), %eax   # 0x143634c09
1400101e8:     	movl	$0x4, %edx
1400101ed:     	movq	%r12, %rcx
1400101f0:     	callq	0x140613c10 <.text+0x612c10>
1400101f5:     	testq	%rax, %rax
1400101f8:     	je	0x14001187e <.text+0x1087e>
1400101fe:     	movq	%rax, 0x290(%rbp)
140010205:     	movq	%rax, %rcx
140010208:     	movq	%rsi, %rdx
14001020b:     	movq	%r12, %r8
14001020e:     	callq	0x142923c00 <.text+0x2922c00>
140010213:     	movq	0x4d8(%rbp), %rdx
14001021a:     	cmpq	%rdx, %r15
14001021d:     	jae	0x140011381 <.text+0x10381>
140010223:     	movq	0x4d0(%rbp), %rsi
14001022a:     	movzbl	0x36249d8(%rip), %eax   # 0x143634c09
140010231:     	movl	$0x4, %edx
140010236:     	movq	%r12, %rcx
140010239:     	callq	0x140613c10 <.text+0x612c10>
14001023e:     	testq	%rax, %rax
140010241:     	je	0x140011897 <.text+0x10897>
140010247:     	movq	%rdi, 0x7b8(%rbp)
14001024e:     	jmp	0x140010295 <.text+0xf295>
140010250:     	movq	0x430(%rbp), %r13
140010257:     	movl	$0x4, %ecx
14001025c:     	movq	%rsi, %rdx
14001025f:     	movq	%r12, %r8
140010262:     	callq	0x142923c00 <.text+0x2922c00>
140010267:     	movq	0x4d8(%rbp), %rdx
14001026e:     	movl	$0x4, %eax
140010273:     	cmpq	%rdx, %r15
140010276:     	jae	0x140011378 <.text+0x10378>
14001027c:     	movq	%rax, 0x290(%rbp)
140010283:     	movq	0x4d0(%rbp), %rsi
14001028a:     	movq	$0x0, 0x7b8(%rbp)
140010295:     	leaq	(%rsi,%rbx,4), %rdx
140010299:     	movq	%rax, %rsi
14001029c:     	movq	%rax, %rcx
14001029f:     	movq	%r12, %r8
1400102a2:     	callq	0x142923c00 <.text+0x2922c00>
1400102a7:     	xorl	%eax, %eax
1400102a9:     	movl	$0x0, %ecx
1400102ae:     	testq	%rbx, %rbx
1400102b1:     	jne	0x1400102b9 <.text+0xf2b9>
1400102b3:     	movl	0x4f8(%rbp), %ecx
1400102b9:     	movq	0x470(%rbp), %rdx
1400102c0:     	decq	%rdx
1400102c3:     	cmpq	%rdx, %r15
1400102c6:     	movzbl	0x4f9(%rbp), %edx
1400102cd:     	cmovel	%edx, %eax
1400102d0:     	movq	0x668(%rbp), %rdx
1400102d7:     	movq	%rdx, 0x610(%rbp)
1400102de:     	movq	0x7a8(%rbp), %rdx
1400102e5:     	movq	%rdx, 0x618(%rbp)
1400102ec:     	movq	0x588(%rbp), %rdx
1400102f3:     	movq	%rdx, 0x620(%rbp)
1400102fa:     	movq	0x410(%rbp), %rdx
140010301:     	movq	%rdx, 0x5c0(%rbp)
140010308:     	movaps	0x3e0(%rbp), %xmm0
14001030f:     	movaps	0x3f0(%rbp), %xmm1
140010316:     	movaps	0x400(%rbp), %xmm2
14001031d:     	movaps	%xmm0, 0x590(%rbp)
140010324:     	movaps	%xmm1, 0x5a0(%rbp)
14001032b:     	movaps	%xmm2, 0x5b0(%rbp)
140010332:     	movq	%r13, 0x5c8(%rbp)
140010339:     	movq	0x300(%rbp), %rdx
140010340:     	movq	%rdx, 0x5d0(%rbp)
140010347:     	movq	%rdi, 0x5d8(%rbp)
14001034e:     	movq	0x7b8(%rbp), %rdx
140010355:     	movq	%rdx, 0x5e0(%rbp)
14001035c:     	movq	%rsi, 0x5e8(%rbp)
140010363:     	movq	%rdi, 0x5f0(%rbp)
14001036a:     	movq	%rdx, 0x5f8(%rbp)
140010371:     	movq	0x290(%rbp), %rdx
140010378:     	movq	%rdx, 0x600(%rbp)
14001037f:     	movq	%rdi, 0x608(%rbp)
140010386:     	movq	0x758(%rbp), %rdx
14001038d:     	movl	%edx, 0x630(%rbp)
140010393:     	movq	0x678(%rbp), %rdx
14001039a:     	movq	(%rdx), %rdx
14001039d:     	movq	%rdx, 0x628(%rbp)
1400103a4:     	movb	%cl, 0x634(%rbp)
1400103aa:     	movb	%al, 0x635(%rbp)
1400103b0:     	movq	0x348(%rbp), %rdi
1400103b7:     	movq	0x10(%rdi), %rsi
1400103bb:     	cmpq	(%rdi), %rsi
1400103be:     	leaq	0x590(%rbp), %rbx
1400103c5:     	jne	0x1400103d6 <.text+0xf3d6>
1400103c7:     	movq	%rdi, %rcx
1400103ca:     	leaq	0x2916317(%rip), %rdx   # 0x1429266e8
1400103d1:     	callq	0x1405636c0 <.text+0x5626c0>
1400103d6:     	imulq	$0xa8, %rsi, %rcx
1400103dd:     	addq	0x8(%rdi), %rcx
1400103e1:     	movl	$0xa8, %r8d
1400103e7:     	movq	%rbx, %rdx
1400103ea:     	callq	0x142923c10 <.text+0x2922c10>
1400103ef:     	incq	%rsi
1400103f2:     	movq	%rsi, 0x10(%rdi)
1400103f6:     	movq	0x7a0(%rbp), %r12
1400103fd:     	jmp	0x14000fef0 <.text+0xeef0>
140010402:     	movq	0x798(%rbp), %r12
140010409:     	movq	0x560(%rbp), %rdx
140010410:     	testq	%rdx, %rdx
140010413:     	movq	0x940(%rbp), %r13
14001041a:     	je	0x140010432 <.text+0xf432>
14001041c:     	shlq	$0x4, %rdx
140010420:     	movl	$0x8, %r8d
140010426:     	movq	0x530(%rbp), %rcx
14001042d:     	callq	0x140613c20 <.text+0x612c20>
140010432:     	movq	0x798(%rbp), %r8
140010439:     	movq	%r8, %rax
14001043c:     	subq	%r12, %rax
14001043f:     	movabsq	$-0x5555555555555555, %rcx # imm = 0xAAAAAAAAAAAAAAAB
140010449:     	mulq	%rcx
14001044c:     	cmpq	%r12, %r8
14001044f:     	je	0x140010488 <.text+0xf488>
140010451:     	movq	%rdx, %rdi
140010454:     	shrq	$0x4, %rdi
140010458:     	addq	$0x8, %r12
14001045c:     	jmp	0x140010469 <.text+0xf469>
14001045e:     	nop
140010460:     	addq	$0x18, %r12
140010464:     	decq	%rdi
140010467:     	je	0x140010488 <.text+0xf488>
140010469:     	movq	-0x8(%r12), %rdx
14001046e:     	testq	%rdx, %rdx
140010471:     	je	0x140010460 <.text+0xf460>
140010473:     	movq	(%r12), %rcx
140010477:     	shlq	$0x2, %rdx
14001047b:     	movl	$0x4, %r8d
140010481:     	callq	0x140613c20 <.text+0x612c20>
140010486:     	jmp	0x140010460 <.text+0xf460>
140010488:     	movq	0x580(%rbp), %rax
14001048f:     	testq	%rax, %rax
140010492:     	je	0x1400104ae <.text+0xf4ae>
140010494:     	shlq	$0x3, %rax
140010498:     	leaq	(%rax,%rax,2), %rdx
14001049c:     	movl	$0x8, %r8d
1400104a2:     	movq	0x568(%rbp), %rcx
1400104a9:     	callq	0x140613c20 <.text+0x612c20>
1400104ae:     	movq	0x460(%rbp), %rax
1400104b5:     	testq	%rax, %rax
1400104b8:     	leaq	0x460(%rbp), %r12
1400104bf:     	je	0x1400104db <.text+0xf4db>
1400104c1:     	movq	0x468(%rbp), %rcx
1400104c8:     	shlq	$0x2, %rax
1400104cc:     	leaq	(%rax,%rax,2), %rdx
1400104d0:     	movl	$0x4, %r8d
1400104d6:     	callq	0x140613c20 <.text+0x612c20>
1400104db:     	movq	0x478(%rbp), %rdx
1400104e2:     	testq	%rdx, %rdx
1400104e5:     	je	0x1400104fd <.text+0xf4fd>
1400104e7:     	movq	0x480(%rbp), %rcx
1400104ee:     	shlq	$0x2, %rdx
1400104f2:     	movl	$0x4, %r8d
1400104f8:     	callq	0x140613c20 <.text+0x612c20>
1400104fd:     	movq	0x498(%rbp), %rdx
140010504:     	testq	%rdx, %rdx
140010507:     	je	0x14001051f <.text+0xf51f>
140010509:     	movq	0x4a0(%rbp), %rcx
140010510:     	shlq	$0x3, %rdx
140010514:     	movl	$0x4, %r8d
14001051a:     	callq	0x140613c20 <.text+0x612c20>
14001051f:     	movq	0x4b0(%rbp), %rdx
140010526:     	testq	%rdx, %rdx
140010529:     	je	0x140010541 <.text+0xf541>
14001052b:     	movq	0x4b8(%rbp), %rcx
140010532:     	shlq	$0x2, %rdx
140010536:     	movl	$0x4, %r8d
14001053c:     	callq	0x140613c20 <.text+0x612c20>
140010541:     	movq	0x4c8(%rbp), %rdx
140010548:     	testq	%rdx, %rdx
14001054b:     	je	0x14000ecf0 <.text+0xdcf0>
140010551:     	movq	0x4d0(%rbp), %rcx
140010558:     	shlq	$0x2, %rdx
14001055c:     	movl	$0x4, %r8d
140010562:     	callq	0x140613c20 <.text+0x612c20>
140010567:     	jmp	0x14000ecf0 <.text+0xdcf0>
14001056c:     	movq	%rdx, %r12
14001056f:     	leaq	0x590(%rbp), %rbx
140010576:     	movq	0x560(%rbp), %rdx
14001057d:     	testq	%rdx, %rdx
140010580:     	movq	0x940(%rbp), %r13
140010587:     	jne	0x14001041c <.text+0xf41c>
14001058d:     	jmp	0x140010432 <.text+0xf432>
140010592:     	leaq	0x440(%rbp), %rcx
140010599:     	callq	0x1405dec70 <.text+0x5ddc70>
14001059e:     	movq	0x88(%rbp), %rdi
1400105a5:     	xorl	%r14d, %r14d
1400105a8:     	movq	0x160(%rbp), %rsi
1400105af:     	addq	$0x18, %rsi
1400105b3:     	movq	%rsi, %rcx
1400105b6:     	callq	0x140c81e10 <.text+0xc80e10>
1400105bb:     	movb	$0x1, %al
1400105bd:     	lock
1400105be:     	cmpxchgb	%r14b, (%rsi)
1400105c2:     	jne	0x1400105ec <.text+0xf5ec>
1400105c4:     	movq	0x1b8(%rbp), %rsi
1400105cb:     	addq	$0x18, %rsi
1400105cf:     	movq	%rsi, %rcx
1400105d2:     	callq	0x140c81e10 <.text+0xc80e10>
1400105d7:     	movb	$0x1, %al
1400105d9:     	lock
1400105da:     	cmpxchgb	%r14b, (%rsi)
1400105de:     	jne	0x1400105f8 <.text+0xf5f8>
1400105e0:     	movb	$0x1, 0x648(%rbp)
1400105e7:     	jmp	0x140009330 <.text+0x8330>
1400105ec:     	movq	%rsi, %rcx
1400105ef:     	xorl	%edx, %edx
1400105f1:     	callq	0x1428cdb90 <.text+0x28ccb90>
1400105f6:     	jmp	0x1400105c4 <.text+0xf5c4>
1400105f8:     	movq	%rsi, %rcx
1400105fb:     	xorl	%edx, %edx
1400105fd:     	callq	0x1428cdb90 <.text+0x28ccb90>
140010602:     	jmp	0x1400105e0 <.text+0xf5e0>
140010604:     	movq	0x370(%rbp), %rax
14001060b:     	movq	0x8(%rax), %rsi
14001060f:     	movq	0x10(%rax), %rax
140010613:     	leaq	(%rax,%rax,4), %rdi
140010617:     	shlq	$0x5, %rdi
14001061b:     	addq	%rsi, %rdi
14001061e:     	addq	$0x168, 0x7c0(%rbp)     # imm = 0x168
140010629:     	nopl	(%rax)
140010630:     	cmpq	%rdi, %rsi
140010633:     	je	0x140010661 <.text+0xf661>
140010635:     	movl	0x88(%rsi), %ecx
14001063b:     	movl	0x8c(%rsi), %edx
140010641:     	callq	0x140959e70 <.text+0x958e70>
140010646:     	addq	$0xa0, %rsi
14001064d:     	movq	0x7c0(%rbp), %rcx
140010654:     	movq	%rax, %rdx
140010657:     	movb	$0x1, %r8b
14001065a:     	callq	0x1402c8dc0 <.text+0x2c7dc0>
14001065f:     	jmp	0x140010630 <.text+0xf630>
140010661:     	movq	0x348(%rbp), %rax
140010668:     	movq	0x8(%rax), %rsi
14001066c:     	imulq	$0xa8, 0x10(%rax), %rdi
140010674:     	addq	%rsi, %rdi
140010677:     	nopw	(%rax,%rax)
140010680:     	cmpq	%rdi, %rsi
140010683:     	je	0x1400106b1 <.text+0xf6b1>
140010685:     	movl	0x98(%rsi), %ecx
14001068b:     	movl	0x9c(%rsi), %edx
140010691:     	callq	0x140959e70 <.text+0x958e70>
140010696:     	addq	$0xa8, %rsi
14001069d:     	movq	0x7c0(%rbp), %rcx
1400106a4:     	movq	%rax, %rdx
1400106a7:     	xorl	%r8d, %r8d
1400106aa:     	callq	0x1402c8dc0 <.text+0x2c7dc0>
1400106af:     	jmp	0x140010680 <.text+0xf680>
1400106b1:     	movq	0x348(%rbp), %rax
1400106b8:     	movq	0x8(%rax), %r12
1400106bc:     	imulq	$0xa8, 0x10(%rax), %r14
1400106c4:     	addq	%r12, %r14
1400106c7:     	leaq	0x460(%rbp), %rsi
1400106ce:     	leaq	0x6a0(%rbp), %rdi
1400106d5:     	leaq	0x380(%rbp), %rbx
1400106dc:     	xorl	%r15d, %r15d
1400106df:     	cmpq	%r14, %r12
1400106e2:     	jne	0x140010700 <.text+0xf700>
1400106e4:     	jmp	0x140010c35 <.text+0xfc35>
1400106e9:     	nopl	(%rax)
1400106f0:     	addq	$0xa8, %r12
1400106f7:     	cmpq	%r14, %r12
1400106fa:     	je	0x140010c35 <.text+0xfc35>
140010700:     	leaq	0x68(%r12), %rax
140010705:     	leaq	0x38(%r12), %rcx
14001070a:     	movq	0x950(%rbp), %rdx
140010711:     	movq	%rdx, 0x460(%rbp)
140010718:     	movq	0x7c0(%rbp), %rdx
14001071f:     	movq	%rdx, 0x468(%rbp)
140010726:     	movq	0x248(%rbp), %rdx
14001072d:     	movq	%rdx, 0x470(%rbp)
140010734:     	movq	0x2e0(%rbp), %rdx
14001073b:     	movq	%rdx, 0x478(%rbp)
140010742:     	movq	%r12, 0x380(%rbp)
140010749:     	movq	%rax, 0x388(%rbp)
140010750:     	movq	%rcx, 0x390(%rbp)
140010757:     	cmpb	$0x0, 0xa4(%r12)
140010760:     	je	0x1400109c0 <.text+0xf9c0>
140010766:     	movl	0x98(%r12), %edx
14001076e:     	movl	0x9c(%r12), %r8d
140010776:     	movq	%rsi, %rcx
140010779:     	movb	$0x1, %r9b
14001077c:     	callq	0x1400286d0 <.text+0x276d0>
140010781:     	testb	%al, %al
140010783:     	je	0x1400109c0 <.text+0xf9c0>
140010789:     	movq	%rdi, %rcx
14001078c:     	movq	%rbx, %rdx
14001078f:     	movb	$0x1, %r8b
140010792:     	callq	0x140027840 <.text+0x26840>
140010797:     	cmpq	0x6a0(%rbp), %r15
14001079e:     	jo	0x1400109c0 <.text+0xf9c0>
1400107a4:     	movq	0x6f0(%rbp), %rax
1400107ab:     	movq	%rax, 0x800(%rbp)
1400107b2:     	movq	0x6f8(%rbp), %rax
1400107b9:     	movq	%rax, 0x760(%rbp)
1400107c0:     	movq	0x700(%rbp), %rdx
1400107c7:     	movq	0x340(%rbp), %rax
1400107ce:     	movl	0x53c(%rbp), %ecx
1400107d4:     	movl	%ecx, (%rax)
1400107d6:     	movq	0x6d0(%rbp), %rax
1400107dd:     	movq	%rax, 0x5c0(%rbp)
1400107e4:     	movups	0x6a0(%rbp), %xmm0
1400107eb:     	movdqu	0x6b0(%rbp), %xmm1
1400107f3:     	movdqu	0x6c0(%rbp), %xmm2
1400107fb:     	movdqa	%xmm2, 0x5b0(%rbp)
140010803:     	movdqa	%xmm1, 0x5a0(%rbp)
14001080b:     	movaps	%xmm0, 0x590(%rbp)
140010812:     	movq	0x6d8(%rbp), %rax
140010819:     	movq	%rax, 0x780(%rbp)
140010820:     	movq	0x6e0(%rbp), %rax
140010827:     	movq	%rax, 0x7b0(%rbp)
14001082e:     	movq	0x6e8(%rbp), %r15
140010835:     	testq	%rdx, %rdx
140010838:     	movq	%rdx, 0x7f0(%rbp)
14001083f:     	je	0x140010867 <.text+0xf867>
140010841:     	leaq	(,%rdx,4), %r13
140010849:     	movzbl	0x36243b9(%rip), %eax   # 0x143634c09
140010850:     	movl	$0x4, %edx
140010855:     	movq	%r13, %rcx
140010858:     	callq	0x140613c40 <.text+0x612c40>
14001085d:     	testq	%rax, %rax
140010860:     	jne	0x14001086c <.text+0xf86c>
140010862:     	jmp	0x14001152a <.text+0x1052a>
140010867:     	movl	$0x4, %eax
14001086c:     	movq	%rax, 0x778(%rbp)
140010873:     	movl	0x98(%r12), %ecx
14001087b:     	movl	0x9c(%r12), %edx
140010883:     	callq	0x140959e70 <.text+0x958e70>
140010888:     	movl	0xa0(%r12), %ecx
140010890:     	movq	0x5c0(%rbp), %rdx
140010897:     	movq	%rdx, 0x170(%rbp)
14001089e:     	movaps	0x590(%rbp), %xmm0
1400108a5:     	movdqa	0x5a0(%rbp), %xmm1
1400108ad:     	movdqa	0x5b0(%rbp), %xmm2
1400108b5:     	movdqa	%xmm2, 0x160(%rbp)
1400108bd:     	movdqa	%xmm1, 0x150(%rbp)
1400108c5:     	movaps	%xmm0, 0x140(%rbp)
1400108cc:     	movq	0x780(%rbp), %rdx
1400108d3:     	movq	%rdx, 0x178(%rbp)
1400108da:     	movq	0x7b0(%rbp), %rdx
1400108e1:     	movq	%rdx, 0x180(%rbp)
1400108e8:     	movq	%r15, 0x188(%rbp)
1400108ef:     	movq	0x800(%rbp), %rdx
1400108f6:     	movq	%rdx, 0x190(%rbp)
1400108fd:     	movq	0x760(%rbp), %rdx
140010904:     	movq	%rdx, 0x198(%rbp)
14001090b:     	movq	0x7f0(%rbp), %rdx
140010912:     	movq	%rdx, 0x1a0(%rbp)
140010919:     	movq	%rdx, 0x1a8(%rbp)
140010920:     	movq	0x778(%rbp), %r8
140010927:     	movq	%r8, 0x1b0(%rbp)
14001092e:     	movq	%rdx, 0x1b8(%rbp)
140010935:     	movq	%rax, 0x1c0(%rbp)
14001093c:     	movw	$0x0, 0x1d4(%rbp)
140010945:     	movl	%ecx, 0x1d0(%rbp)
14001094b:     	movq	0x98(%r12), %rax
140010953:     	movq	%rax, 0x1c8(%rbp)
14001095a:     	movl	$0x0, 0x1d6(%rbp)
140010964:     	movw	$0x100, 0x1da(%rbp)     # imm = 0x100
14001096d:     	movq	0x370(%rbp), %r13
140010974:     	movq	0x10(%r13), %r15
140010978:     	cmpq	(%r13), %r15
14001097c:     	jne	0x14001098d <.text+0xf98d>
14001097e:     	movq	%r13, %rcx
140010981:     	leaq	0x2915ba8(%rip), %rdx   # 0x142926530
140010988:     	callq	0x140563480 <.text+0x562480>
14001098d:     	leaq	(%r15,%r15,4), %rcx
140010991:     	shlq	$0x5, %rcx
140010995:     	addq	0x8(%r13), %rcx
140010999:     	movl	$0xa0, %r8d
14001099f:     	leaq	0x140(%rbp), %rdx
1400109a6:     	callq	0x142923c10 <.text+0x2922c10>
1400109ab:     	incq	%r15
1400109ae:     	movq	%r15, 0x10(%r13)
1400109b2:     	movq	0x940(%rbp), %r13
1400109b9:     	xorl	%r15d, %r15d
1400109bc:     	nopl	(%rax)
1400109c0:     	cmpb	$0x0, 0xa5(%r12)
1400109c9:     	je	0x1400106f0 <.text+0xf6f0>
1400109cf:     	movl	0x98(%r12), %r8d
1400109d7:     	movl	0x9c(%r12), %edx
1400109df:     	movq	%rsi, %rcx
1400109e2:     	xorl	%r9d, %r9d
1400109e5:     	callq	0x1400286d0 <.text+0x276d0>
1400109ea:     	testb	%al, %al
1400109ec:     	je	0x1400106f0 <.text+0xf6f0>
1400109f2:     	movq	%rdi, %rcx
1400109f5:     	movq	%rbx, %rdx
1400109f8:     	xorl	%r8d, %r8d
1400109fb:     	callq	0x140027840 <.text+0x26840>
140010a00:     	cmpq	0x6a0(%rbp), %r15
140010a07:     	jo	0x1400106f0 <.text+0xf6f0>
140010a0d:     	movq	0x6f0(%rbp), %rax
140010a14:     	movq	%rax, 0x800(%rbp)
140010a1b:     	movq	0x6f8(%rbp), %rax
140010a22:     	movq	%rax, 0x760(%rbp)
140010a29:     	movq	0x700(%rbp), %rdx
140010a30:     	movq	0x340(%rbp), %rax
140010a37:     	movl	0x53c(%rbp), %ecx
140010a3d:     	movl	%ecx, (%rax)
140010a3f:     	movq	0x6d0(%rbp), %rax
140010a46:     	movq	%rax, 0x5c0(%rbp)
140010a4d:     	movups	0x6a0(%rbp), %xmm0
140010a54:     	movdqu	0x6b0(%rbp), %xmm1
140010a5c:     	movdqu	0x6c0(%rbp), %xmm2
140010a64:     	movdqa	%xmm2, 0x5b0(%rbp)
140010a6c:     	movdqa	%xmm1, 0x5a0(%rbp)
140010a74:     	movaps	%xmm0, 0x590(%rbp)
140010a7b:     	movq	0x6d8(%rbp), %rax
140010a82:     	movq	%rax, 0x780(%rbp)
140010a89:     	movq	0x6e0(%rbp), %rax
140010a90:     	movq	%rax, 0x7b0(%rbp)
140010a97:     	movq	0x6e8(%rbp), %r15
140010a9e:     	testq	%rdx, %rdx
140010aa1:     	movq	%rdx, 0x7f0(%rbp)
140010aa8:     	je	0x140010ad0 <.text+0xfad0>
140010aaa:     	leaq	(,%rdx,4), %r13
140010ab2:     	movzbl	0x3624150(%rip), %eax   # 0x143634c09
140010ab9:     	movl	$0x4, %edx
140010abe:     	movq	%r13, %rcx
140010ac1:     	callq	0x140613c40 <.text+0x612c40>
140010ac6:     	testq	%rax, %rax
140010ac9:     	jne	0x140010ad5 <.text+0xfad5>
140010acb:     	jmp	0x14001154a <.text+0x1054a>
140010ad0:     	movl	$0x4, %eax
140010ad5:     	movq	%rax, 0x778(%rbp)
140010adc:     	movl	0x98(%r12), %ecx
140010ae4:     	movl	0x9c(%r12), %edx
140010aec:     	callq	0x140959e70 <.text+0x958e70>
140010af1:     	movl	0xa0(%r12), %ecx
140010af9:     	movq	0x5c0(%rbp), %rdx
140010b00:     	movq	%rdx, 0x170(%rbp)
140010b07:     	movaps	0x590(%rbp), %xmm0
140010b0e:     	movdqa	0x5a0(%rbp), %xmm1
140010b16:     	movdqa	0x5b0(%rbp), %xmm2
140010b1e:     	movdqa	%xmm2, 0x160(%rbp)
140010b26:     	movdqa	%xmm1, 0x150(%rbp)
140010b2e:     	movaps	%xmm0, 0x140(%rbp)
140010b35:     	movq	0x780(%rbp), %rdx
140010b3c:     	movq	%rdx, 0x178(%rbp)
140010b43:     	movq	0x7b0(%rbp), %rdx
140010b4a:     	movq	%rdx, 0x180(%rbp)
140010b51:     	movq	%r15, 0x188(%rbp)
140010b58:     	movq	0x800(%rbp), %rdx
140010b5f:     	movq	%rdx, 0x190(%rbp)
140010b66:     	movq	0x760(%rbp), %rdx
140010b6d:     	movq	%rdx, 0x198(%rbp)
140010b74:     	movq	0x7f0(%rbp), %rdx
140010b7b:     	movq	%rdx, 0x1a0(%rbp)
140010b82:     	movq	%rdx, 0x1a8(%rbp)
140010b89:     	movq	0x778(%rbp), %r8
140010b90:     	movq	%r8, 0x1b0(%rbp)
140010b97:     	movq	%rdx, 0x1b8(%rbp)
140010b9e:     	movq	%rax, 0x1c0(%rbp)
140010ba5:     	movw	$0x0, 0x1d4(%rbp)
140010bae:     	movl	%ecx, 0x1d0(%rbp)
140010bb4:     	movq	0x98(%r12), %rax
140010bbc:     	movq	%rax, 0x1c8(%rbp)
140010bc3:     	movl	$0x0, 0x1d6(%rbp)
140010bcd:     	movw	$0x200, 0x1da(%rbp)     # imm = 0x200
140010bd6:     	movq	0x370(%rbp), %r13
140010bdd:     	movq	0x10(%r13), %r15
140010be1:     	cmpq	(%r13), %r15
140010be5:     	jne	0x140010bf6 <.text+0xfbf6>
140010be7:     	movq	%r13, %rcx
140010bea:     	leaq	0x291596f(%rip), %rdx   # 0x142926560
140010bf1:     	callq	0x140563480 <.text+0x562480>
140010bf6:     	leaq	(%r15,%r15,4), %rcx
140010bfa:     	shlq	$0x5, %rcx
140010bfe:     	addq	0x8(%r13), %rcx
140010c02:     	movl	$0xa0, %r8d
140010c08:     	leaq	0x140(%rbp), %rdx
140010c0f:     	callq	0x142923c10 <.text+0x2922c10>
140010c14:     	incq	%r15
140010c17:     	movq	%r15, 0x10(%r13)
140010c1b:     	movq	0x940(%rbp), %r13
140010c22:     	xorl	%r15d, %r15d
140010c25:     	addq	$0xa8, %r12
140010c2c:     	cmpq	%r14, %r12
140010c2f:     	jne	0x140010700 <.text+0xf700>
140010c35:     	testb	$0x1, 0x648(%rbp)
140010c3c:     	je	0x140010daf <.text+0xfdaf>
140010c42:     	movq	0x2f0(%rbp), %r8
140010c49:     	movq	(%r8), %rax
140010c4c:     	movq	0x8(%r8), %rcx
140010c50:     	addq	%rax, %rcx
140010c53:     	incq	%rcx
140010c56:     	movdqa	(%rax), %xmm0
140010c5a:     	pmovmskb	%xmm0, %edx
140010c5e:     	notl	%edx
140010c60:     	movq	0x18(%r8), %r8
140010c64:     	movq	$0x0, 0x140(%rbp)
140010c6f:     	movq	$0x0, 0x150(%rbp)
140010c7a:     	movq	%rax, 0x160(%rbp)
140010c81:     	addq	$0x10, %rax
140010c85:     	movq	%rax, 0x168(%rbp)
140010c8c:     	movq	%rcx, 0x170(%rbp)
140010c93:     	movw	%dx, 0x178(%rbp)
140010c9a:     	movq	%r8, 0x180(%rbp)
140010ca1:     	leaq	0x2916688(%rip), %r8    # 0x142927330
140010ca8:     	leaq	0x590(%rbp), %rcx
140010caf:     	leaq	0x140(%rbp), %rdx
140010cb6:     	callq	0x1402bcd50 <.text+0x2bbd50>
140010cbb:     	movq	0x598(%rbp), %rax
140010cc2:     	movq	%rax, 0x800(%rbp)
140010cc9:     	movq	0x5a0(%rbp), %r14
140010cd0:     	testq	%r14, %r14
140010cd3:     	je	0x140010dfb <.text+0xfdfb>
140010cd9:     	movq	0x268(%rbp), %rcx
140010ce0:     	callq	0x140987ca0 <.text+0x986ca0>
140010ce5:     	movq	%rax, %rsi
140010ce8:     	movq	%r14, %rdi
140010ceb:     	shlq	$0x4, %rdi
140010cef:     	leaq	0x6a0(%rbp), %rbx
140010cf6:     	movq	%rbx, %rcx
140010cf9:     	movq	%rdi, %rdx
140010cfc:     	movl	$0x20, %r8d
140010d02:     	callq	0x140bd4e20 <.text+0xbd3e20>
140010d07:     	movq	%rdi, 0x30(%rsp)
140010d0c:     	movq	0x800(%rbp), %rax
140010d13:     	movq	%rax, 0x28(%rsp)
140010d18:     	movq	%rbx, 0x20(%rsp)
140010d1d:     	leaq	0x29157cc(%rip), %r8    # 0x1429264f0
140010d24:     	leaq	0x140(%rbp), %rcx
140010d2b:     	movl	$0xf, %r9d
140010d31:     	movq	%rsi, %rdx
140010d34:     	callq	0x1405b93b0 <.text+0x5b83b0>
140010d39:     	cmpl	$0xb, 0x140(%rbp)
140010d40:     	jne	0x140010e35 <.text+0xfe35>
140010d46:     	movq	0x168(%rbp), %rax
140010d4d:     	movq	%rax, 0x6c0(%rbp)
140010d54:     	movdqu	0x148(%rbp), %xmm0
140010d5c:     	movdqu	0x158(%rbp), %xmm1
140010d64:     	movdqa	%xmm1, 0x6b0(%rbp)
140010d6c:     	movdqa	%xmm0, 0x6a0(%rbp)
140010d74:     	movl	%r14d, 0x6c8(%rbp)
140010d7b:     	leaq	0x291577e(%rip), %r8    # 0x142926500
140010d82:     	leaq	0x6a0(%rbp), %rdx
140010d89:     	movq	%r13, %rcx
140010d8c:     	callq	0x14014dd30 <.text+0x14cd30>
140010d91:     	cmpl	$0xb, 0x140(%rbp)
140010d98:     	je	0x140010e41 <.text+0xfe41>
140010d9e:     	leaq	0x140(%rbp), %rcx
140010da5:     	callq	0x1400297d0 <.text+0x287d0>
140010daa:     	jmp	0x140010e41 <.text+0xfe41>
140010daf:     	movq	0x2f0(%rbp), %rax
140010db6:     	cmpq	$0x0, 0x18(%rax)
140010dbb:     	jne	0x140010e63 <.text+0xfe63>
140010dc1:     	movq	0x8(%r13), %rsi
140010dc5:     	testq	%rsi, %rsi
140010dc8:     	jne	0x140010dce <.text+0xfdce>
140010dca:     	movq	0x10(%r13), %rsi
140010dce:     	movq	(%rsi), %rax
140010dd1:     	movq	0x10(%rsi), %rdi
140010dd5:     	subq	%rdi, %rax
140010dd8:     	cmpq	$0x7, %rax
140010ddc:     	jbe	0x140011237 <.text+0x10237>
140010de2:     	movq	0x8(%rsi), %rax
140010de6:     	leaq	0x13c7d3(%rip), %rcx    # 0x14014d5c0 <.text+0x14c5c0>
140010ded:     	movq	%rcx, (%rax,%rdi)
140010df1:     	addq	$0x8, %rdi
140010df5:     	movq	%rdi, 0x10(%rsi)
140010df9:     	jmp	0x140010e63 <.text+0xfe63>
140010dfb:     	movq	0x8(%r13), %rsi
140010dff:     	testq	%rsi, %rsi
140010e02:     	jne	0x140010e08 <.text+0xfe08>
140010e04:     	movq	0x10(%r13), %rsi
140010e08:     	movq	(%rsi), %rax
140010e0b:     	movq	0x10(%rsi), %rdi
140010e0f:     	subq	%rdi, %rax
140010e12:     	cmpq	$0x7, %rax
140010e16:     	jbe	0x14001125c <.text+0x1025c>
140010e1c:     	movq	0x8(%rsi), %rax
140010e20:     	leaq	0x13c799(%rip), %rcx    # 0x14014d5c0 <.text+0x14c5c0>
140010e27:     	movq	%rcx, (%rax,%rdi)
140010e2b:     	addq	$0x8, %rdi
140010e2f:     	movq	%rdi, 0x10(%rsi)
140010e33:     	jmp	0x140010e41 <.text+0xfe41>
140010e35:     	leaq	0x140(%rbp), %rcx
140010e3c:     	callq	0x1400297d0 <.text+0x287d0>
140010e41:     	movq	0x590(%rbp), %rdx
140010e48:     	testq	%rdx, %rdx
140010e4b:     	je	0x140010e63 <.text+0xfe63>
140010e4d:     	shlq	$0x4, %rdx
140010e51:     	movl	$0x10, %r8d
140010e57:     	movq	0x800(%rbp), %rcx
140010e5e:     	callq	0x140613c20 <.text+0x612c20>
140010e63:     	cmpb	$0x0, 0x797(%rbp)
140010e6a:     	je	0x140010eb7 <.text+0xfeb7>
140010e6c:     	callq	0x140c18130 <.text+0xc17130>
140010e71:     	movq	%rax, %rcx
140010e74:     	movq	(%rax), %rax
140010e77:     	cmpq	$0x1, %rax
140010e7b:     	jne	0x140010fb8 <.text+0xffb8>
140010e81:     	addq	$0x8, %rcx
140010e85:     	movq	0x270(%rbp), %rdx
140010e8c:     	cmpq	$0x0, (%rcx)
140010e90:     	jne	0x1400111fb <.text+0x101fb>
140010e96:     	movq	$-0x1, (%rcx)
140010e9d:     	movq	%rcx, 0x800(%rbp)
140010ea4:     	addq	$0x8, %rcx
140010ea8:     	callq	0x140c170f0 <.text+0xc160f0>
140010ead:     	movq	0x800(%rbp), %rax
140010eb4:     	incq	(%rax)
140010eb7:     	movaps	0x820(%rbp), %xmm6
140010ebe:     	movaps	0x830(%rbp), %xmm7
140010ec5:     	movaps	0x840(%rbp), %xmm8
140010ecd:     	movaps	0x850(%rbp), %xmm9
140010ed5:     	movaps	0x860(%rbp), %xmm10
140010edd:     	movaps	0x870(%rbp), %xmm11
140010ee5:     	movaps	0x880(%rbp), %xmm12
140010eed:     	movaps	0x890(%rbp), %xmm13
140010ef5:     	movaps	0x8a0(%rbp), %xmm14
140010efd:     	movaps	0x8b0(%rbp), %xmm15
140010f05:     	addq	$0x948, %rsp            # imm = 0x948
140010f0c:     	popq	%rbx
140010f0d:     	popq	%rdi
140010f0e:     	popq	%rsi
140010f0f:     	popq	%r12
140010f11:     	popq	%r13
140010f13:     	popq	%r14
140010f15:     	popq	%r15
140010f17:     	popq	%rbp
140010f18:     	retq
140010f19:     	movb	$0x1, 0x817(%rbp)
140010f20:     	leaq	0x2915881(%rip), %rcx   # 0x1429267a8
140010f27:     	callq	0x1428d98a0 <.text+0x28d88a0>
140010f2c:     	jmp	0x140011a40 <.text+0x10a40>
140010f31:     	movb	$0x1, 0x817(%rbp)
140010f38:     	leaq	0x2915869(%rip), %r8    # 0x1429267a8
140010f3f:     	movq	%rcx, %rdx
140010f42:     	movq	%r15, %rcx
140010f45:     	callq	0x1428d9850 <.text+0x28d8850>
140010f4a:     	jmp	0x140011a40 <.text+0x10a40>
140010f4f:     	movb	$0x1, 0x817(%rbp)
140010f56:     	leaq	0x291584b(%rip), %r8    # 0x1429267a8
140010f5d:     	movq	0x570(%rbp), %rdx
140010f64:     	callq	0x1428d9840 <.text+0x28d8840>
140010f69:     	jmp	0x140011a40 <.text+0x10a40>
140010f6e:     	movb	$0x1, 0x817(%rbp)
140010f75:     	leaq	0x2915844(%rip), %r8    # 0x1429267c0
140010f7c:     	callq	0x1428d9840 <.text+0x28d8840>
140010f81:     	jmp	0x140011a40 <.text+0x10a40>
140010f86:     	movb	$0x1, 0x817(%rbp)
140010f8d:     	leaq	0x2915844(%rip), %r8    # 0x1429267d8
140010f94:     	callq	0x1428d9840 <.text+0x28d8840>
140010f99:     	jmp	0x140011a40 <.text+0x10a40>
140010f9e:     	testq	%rax, %rax
140010fa1:     	jne	0x140010fdd <.text+0xffdd>
140010fa3:     	xorl	%edx, %edx
140010fa5:     	callq	0x1428a1640 <.text+0x28a0640>
140010faa:     	movq	%rax, %rcx
140010fad:     	testq	%rax, %rax
140010fb0:     	jne	0x140008fb8 <.text+0x7fb8>
140010fb6:     	jmp	0x140010fdd <.text+0xffdd>
140010fb8:     	testq	%rax, %rax
140010fbb:     	movq	0x270(%rbp), %rax
140010fc2:     	jne	0x140010fdd <.text+0xffdd>
140010fc4:     	movq	%rax, %rsi
140010fc7:     	xorl	%edx, %edx
140010fc9:     	callq	0x1428a1640 <.text+0x28a0640>
140010fce:     	movq	%rsi, %rdx
140010fd1:     	movq	%rax, %rcx
140010fd4:     	testq	%rax, %rax
140010fd7:     	jne	0x140010e8c <.text+0xfe8c>
140010fdd:     	leaq	0x2921a2c(%rip), %rcx   # 0x142932a10
140010fe4:     	callq	0x1428d80d0 <.text+0x28d70d0>
140010fe9:     	movb	$0x1, 0x817(%rbp)
140010ff0:     	leaq	0x29157f9(%rip), %r8    # 0x1429267f0
140010ff7:     	callq	0x1428d9840 <.text+0x28d8840>
140010ffc:     	jmp	0x140011a40 <.text+0x10a40>
140011001:     	leaq	0x29156f8(%rip), %rcx   # 0x142926700
140011008:     	callq	0x1428d98a0 <.text+0x28d88a0>
14001100d:     	jmp	0x140011a40 <.text+0x10a40>
140011012:     	leaq	0x29156e7(%rip), %r8    # 0x142926700
140011019:     	movq	%r13, %rcx
14001101c:     	movq	%rax, %rdx
14001101f:     	callq	0x1428d9850 <.text+0x28d8850>
140011024:     	jmp	0x140011a40 <.text+0x10a40>
140011029:     	leaq	0x29156d0(%rip), %r8    # 0x142926700
140011030:     	movq	%rax, %rcx
140011033:     	callq	0x1428d9840 <.text+0x28d8840>
140011038:     	jmp	0x140011a40 <.text+0x10a40>
14001103d:     	movb	$0x1, 0x813(%rbp)
140011044:     	leaq	0x291563d(%rip), %rcx   # 0x142926688
14001104b:     	callq	0x1428d98a0 <.text+0x28d88a0>
140011050:     	jmp	0x140011a40 <.text+0x10a40>
140011055:     	movb	$0x1, 0x813(%rbp)
14001105c:     	leaq	0x2915625(%rip), %r8    # 0x142926688
140011063:     	movq	%rbx, %rcx
140011066:     	movq	%rax, %rdx
140011069:     	callq	0x1428d9850 <.text+0x28d8850>
14001106e:     	jmp	0x140011a40 <.text+0x10a40>
140011073:     	movb	$0x1, 0x813(%rbp)
14001107a:     	leaq	0x2915607(%rip), %r8    # 0x142926688
140011081:     	movq	%rax, %rcx
140011084:     	callq	0x1428d9840 <.text+0x28d8840>
140011089:     	jmp	0x140011a40 <.text+0x10a40>
14001108e:     	movb	$0x1, 0x816(%rbp)
140011095:     	movb	$0x1, 0x815(%rbp)
14001109c:     	leaq	0x29159a5(%rip), %rcx   # 0x142926a48
1400110a3:     	callq	0x1428d98a0 <.text+0x28d88a0>
1400110a8:     	jmp	0x140011a40 <.text+0x10a40>
1400110ad:     	movb	$0x1, 0x816(%rbp)
1400110b4:     	movb	$0x1, 0x815(%rbp)
1400110bb:     	leaq	0x2915986(%rip), %r8    # 0x142926a48
1400110c2:     	movq	0x800(%rbp), %rcx
1400110c9:     	movq	%r9, %rdx
1400110cc:     	callq	0x1428d9850 <.text+0x28d8850>
1400110d1:     	jmp	0x140011a40 <.text+0x10a40>
1400110d6:     	movb	$0x1, 0x816(%rbp)
1400110dd:     	movb	$0x1, 0x815(%rbp)
1400110e4:     	leaq	0x291595d(%rip), %r8    # 0x142926a48
1400110eb:     	movq	%r9, %rcx
1400110ee:     	callq	0x1428d9840 <.text+0x28d8840>
1400110f3:     	jmp	0x140011a40 <.text+0x10a40>
1400110f8:     	movq	$0x0, 0x6a0(%rbp)
140011103:     	movb	$0x1, 0x817(%rbp)
14001110a:     	leaq	0x29156ff(%rip), %rax   # 0x142926810
140011111:     	movq	%rax, 0x20(%rsp)
140011116:     	leaq	0x29156eb(%rip), %r8    # 0x142926808
14001111d:     	leaq	0x78(%rbp), %rdx
140011121:     	leaq	0x6a0(%rbp), %r9
140011128:     	xorl	%ecx, %ecx
14001112a:     	callq	0x1428d9596 <.text+0x28d8596>
14001112f:     	jmp	0x140011a40 <.text+0x10a40>
140011134:     	movq	$0x0, 0x140(%rbp)
14001113f:     	leaq	0x299d052(%rip), %rax   # 0x1429ae198
140011146:     	movq	%rax, 0x20(%rsp)
14001114b:     	leaq	0x6a0(%rbp), %rdx
140011152:     	leaq	0x140(%rbp), %r9
140011159:     	xorl	%ecx, %ecx
14001115b:     	movq	%r14, %r8
14001115e:     	callq	0x1428cab41 <.text+0x28c9b41>
140011163:     	jmp	0x140011a40 <.text+0x10a40>
140011168:     	movq	$0x0, 0x140(%rbp)
140011173:     	leaq	0x299d01e(%rip), %rax   # 0x1429ae198
14001117a:     	movq	%rax, 0x20(%rsp)
14001117f:     	leaq	0x6a0(%rbp), %rdx
140011186:     	leaq	0x140(%rbp), %r9
14001118d:     	xorl	%ecx, %ecx
14001118f:     	movq	%r14, %r8
140011192:     	callq	0x1428cab41 <.text+0x28c9b41>
140011197:     	jmp	0x140011a40 <.text+0x10a40>
14001119c:     	movq	$0x0, 0x140(%rbp)
1400111a7:     	leaq	0x299cfea(%rip), %rax   # 0x1429ae198
1400111ae:     	movq	%rax, 0x20(%rsp)
1400111b3:     	leaq	0x6a0(%rbp), %rdx
1400111ba:     	leaq	0x140(%rbp), %r9
1400111c1:     	xorl	%ecx, %ecx
1400111c3:     	movq	%rbx, %r8
1400111c6:     	callq	0x1428cab41 <.text+0x28c9b41>
1400111cb:     	jmp	0x140011a40 <.text+0x10a40>
1400111d0:     	leaq	0x29151b9(%rip), %r8    # 0x142926390
1400111d7:     	movq	%rax, %rcx
1400111da:     	movq	%rdi, %rdx
1400111dd:     	callq	0x1428d9518 <.text+0x28d8518>
1400111e2:     	jmp	0x140011a40 <.text+0x10a40>
1400111e7:     	leaq	0x29151a2(%rip), %r8    # 0x142926390
1400111ee:     	movq	%rdi, %rdx
1400111f1:     	callq	0x1428d9518 <.text+0x28d8518>
1400111f6:     	jmp	0x140011a40 <.text+0x10a40>
1400111fb:     	leaq	0x29218b6(%rip), %rcx   # 0x142932ab8
140011202:     	callq	0x1428d9250 <.text+0x28d8250>
140011207:     	leaq	0x291550a(%rip), %r8    # 0x142926718
14001120e:     	movq	0x7a0(%rbp), %rcx
140011215:     	callq	0x1428d9840 <.text+0x28d8840>
14001121a:     	jmp	0x140011a40 <.text+0x10a40>
14001121f:     	leaq	0x291550a(%rip), %r8    # 0x142926730
140011226:     	movq	0x7a0(%rbp), %rcx
14001122d:     	callq	0x1428d9840 <.text+0x28d8840>
140011232:     	jmp	0x140011a40 <.text+0x10a40>
140011237:     	movq	$0x1, 0x20(%rsp)
140011240:     	movl	$0x8, %r8d
140011246:     	movl	$0x1, %r9d
14001124c:     	movq	%rsi, %rcx
14001124f:     	movq	%rdi, %rdx
140011252:     	callq	0x1428a29e0 <.text+0x28a19e0>
140011257:     	jmp	0x140010de2 <.text+0xfde2>
14001125c:     	movq	$0x1, 0x20(%rsp)
140011265:     	movl	$0x8, %r8d
14001126b:     	movl	$0x1, %r9d
140011271:     	movq	%rsi, %rcx
140011274:     	movq	%rdi, %rdx
140011277:     	callq	0x1428a29e0 <.text+0x28a19e0>
14001127c:     	jmp	0x140010e1c <.text+0xfe1c>
140011281:     	leaq	0x2915418(%rip), %r8    # 0x1429266a0
140011288:     	movq	0x7b8(%rbp), %rcx
14001128f:     	callq	0x1428d9840 <.text+0x28d8840>
140011294:     	jmp	0x140011a40 <.text+0x10a40>
140011299:     	leaq	0x2915418(%rip), %r8    # 0x1429266b8
1400112a0:     	movq	0x7b8(%rbp), %rcx
1400112a7:     	callq	0x1428d9840 <.text+0x28d8840>
1400112ac:     	jmp	0x140011a40 <.text+0x10a40>
1400112b1:     	leaq	0x29157a8(%rip), %r8    # 0x142926a60
1400112b8:     	movq	0x760(%rbp), %rcx
1400112bf:     	callq	0x1428d9840 <.text+0x28d8840>
1400112c4:     	jmp	0x140011a40 <.text+0x10a40>
1400112c9:     	leaq	0x29157a8(%rip), %r8    # 0x142926a78
1400112d0:     	movq	0x760(%rbp), %rcx
1400112d7:     	callq	0x1428d9840 <.text+0x28d8840>
1400112dc:     	jmp	0x140011a40 <.text+0x10a40>
1400112e1:     	leaq	0x29157a8(%rip), %r8    # 0x142926a90
1400112e8:     	movq	0x760(%rbp), %rcx
1400112ef:     	movq	%rbx, %rdx
1400112f2:     	callq	0x1428d9840 <.text+0x28d8840>
1400112f7:     	jmp	0x140011a40 <.text+0x10a40>
1400112fc:     	movb	$0x1, 0x7ee(%rbp)
140011303:     	movb	$0x1, 0x7ed(%rbp)
14001130a:     	movb	$0x1, 0x7ec(%rbp)
140011311:     	movb	$0x0, 0x7eb(%rbp)
140011318:     	movb	$0x0, 0x7ea(%rbp)
14001131f:     	leaq	0x29152ba(%rip), %rcx   # 0x1429265e0
140011326:     	callq	0x1428d9310 <.text+0x28d8310>
14001132b:     	jmp	0x140011a40 <.text+0x10a40>
140011330:     	xorl	%edi, %edi
140011332:     	movq	%rbx, 0xd0(%rbp)
140011339:     	movq	%rdi, 0xc8(%rbp)
140011340:     	leaq	0x2915401(%rip), %r8    # 0x142926748
140011347:     	movq	0x7a0(%rbp), %rcx
14001134e:     	callq	0x1428d9840 <.text+0x28d8840>
140011353:     	jmp	0x140011a40 <.text+0x10a40>
140011358:     	xorl	%ebx, %ebx
14001135a:     	movb	$0x1, 0x817(%rbp)
140011361:     	leaq	0x29a2bc8(%rip), %r8    # 0x1429b3f30
140011368:     	movq	%rbx, %rcx
14001136b:     	movq	%r12, %rdx
14001136e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140011373:     	jmp	0x140011a40 <.text+0x10a40>
140011378:     	movq	%rax, 0x290(%rbp)
14001137f:     	xorl	%edi, %edi
140011381:     	movq	0x290(%rbp), %rax
140011388:     	movq	%rax, 0xe0(%rbp)
14001138f:     	movq	%rdi, 0xd8(%rbp)
140011396:     	leaq	0x2915333(%rip), %r8    # 0x1429266d0
14001139d:     	movq	0x7b8(%rbp), %rcx
1400113a4:     	callq	0x1428d9840 <.text+0x28d8840>
1400113a9:     	jmp	0x140011a40 <.text+0x10a40>
1400113ae:     	xorl	%esi, %esi
1400113b0:     	leaq	0x299d081(%rip), %r8    # 0x1429ae438
1400113b7:     	movq	%rsi, %rcx
1400113ba:     	movq	%r12, %rdx
1400113bd:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400113c2:     	jmp	0x140011a40 <.text+0x10a40>
1400113c7:     	xorl	%r14d, %r14d
1400113ca:     	leaq	0x299d067(%rip), %r8    # 0x1429ae438
1400113d1:     	movq	%r14, %rcx
1400113d4:     	movq	%r12, %rdx
1400113d7:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400113dc:     	jmp	0x140011a40 <.text+0x10a40>
1400113e1:     	leaq	0x2914f48(%rip), %r8    # 0x142926330
1400113e8:     	movl	$0x4, %ecx
1400113ed:     	movq	%rsi, %rdx
1400113f0:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400113f5:     	jmp	0x140011a40 <.text+0x10a40>
1400113fa:     	movb	$0x1, 0x814(%rbp)
140011401:     	leaq	0x2915250(%rip), %rcx   # 0x142926658
140011408:     	callq	0x1428d9310 <.text+0x28d8310>
14001140d:     	jmp	0x140011a40 <.text+0x10a40>
140011412:     	movb	$0x1, 0x814(%rbp)
140011419:     	leaq	0x2915250(%rip), %rcx   # 0x142926670
140011420:     	callq	0x1428d9310 <.text+0x28d8310>
140011425:     	jmp	0x140011a40 <.text+0x10a40>
14001142a:     	xorl	%esi, %esi
14001142c:     	leaq	0x2915f65(%rip), %r8    # 0x142927398
140011433:     	movq	%rsi, %rcx
140011436:     	movq	%r15, %rdx
140011439:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14001143e:     	jmp	0x140011a40 <.text+0x10a40>
140011443:     	leaq	0x29c37ce(%rip), %r8    # 0x1429d4c18
14001144a:     	movl	$0x4, %ecx
14001144f:     	movq	%rdi, %rdx
140011452:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140011457:     	jmp	0x140011a40 <.text+0x10a40>
14001145c:     	leaq	0x2914ee5(%rip), %r8    # 0x142926348
140011463:     	movl	$0x4, %ecx
140011468:     	movq	%rsi, %rdx
14001146b:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140011470:     	jmp	0x140011a40 <.text+0x10a40>
140011475:     	leaq	0x2915f1c(%rip), %r8    # 0x142927398
14001147c:     	movl	$0x4, %ecx
140011481:     	movl	$0x30, %edx
140011486:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14001148b:     	jmp	0x140011a40 <.text+0x10a40>
140011490:     	leaq	0x2914ec9(%rip), %r8    # 0x142926360
140011497:     	movl	$0x4, %ecx
14001149c:     	movq	%rsi, %rdx
14001149f:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400114a4:     	jmp	0x140011a40 <.text+0x10a40>
1400114a9:     	leaq	0x2914ec8(%rip), %r8    # 0x142926378
1400114b0:     	movl	$0x4, %ecx
1400114b5:     	movq	%rsi, %rdx
1400114b8:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400114bd:     	jmp	0x140011a40 <.text+0x10a40>
1400114c2:     	leaq	0x29150af(%rip), %rcx   # 0x142926578
1400114c9:     	callq	0x1428d9310 <.text+0x28d8310>
1400114ce:     	jmp	0x140011a40 <.text+0x10a40>
1400114d3:     	xorl	%esi, %esi
1400114d5:     	movb	$0x1, 0x813(%rbp)
1400114dc:     	leaq	0x2915eb5(%rip), %r8    # 0x142927398
1400114e3:     	movq	%rsi, %rcx
1400114e6:     	movq	0x100(%rbp), %rdx
1400114ed:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400114f2:     	jmp	0x140011a40 <.text+0x10a40>
1400114f7:     	leaq	0x2914faa(%rip), %rcx   # 0x1429264a8
1400114fe:     	callq	0x1428d9310 <.text+0x28d8310>
140011503:     	jmp	0x140011a40 <.text+0x10a40>
140011508:     	leaq	0x2914fb1(%rip), %rcx   # 0x1429264c0
14001150f:     	callq	0x1428d9310 <.text+0x28d8310>
140011514:     	jmp	0x140011a40 <.text+0x10a40>
140011519:     	leaq	0x2914fb8(%rip), %rcx   # 0x1429264d8
140011520:     	callq	0x1428d9310 <.text+0x28d8310>
140011525:     	jmp	0x140011a40 <.text+0x10a40>
14001152a:     	movb	$0x1, 0x796(%rbp)
140011531:     	leaq	0x2914fe0(%rip), %r8    # 0x142926518
140011538:     	movl	$0x4, %ecx
14001153d:     	movq	%r13, %rdx
140011540:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140011545:     	jmp	0x140011a40 <.text+0x10a40>
14001154a:     	movb	$0x1, 0x795(%rbp)
140011551:     	leaq	0x2914ff0(%rip), %r8    # 0x142926548
140011558:     	movl	$0x4, %ecx
14001155d:     	movq	%r13, %rdx
140011560:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140011565:     	jmp	0x140011a40 <.text+0x10a40>
14001156a:     	xorl	%esi, %esi
14001156c:     	movq	0x108(%rbp), %r15
140011573:     	movb	$0x1, 0x816(%rbp)
14001157a:     	movb	$0x1, 0x815(%rbp)
140011581:     	leaq	0x2915e10(%rip), %r8    # 0x142927398
140011588:     	movq	%rsi, %rcx
14001158b:     	movq	%r15, %rdx
14001158e:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140011593:     	jmp	0x140011a40 <.text+0x10a40>
140011598:     	movl	$0x10, %ecx
14001159d:     	movl	$0x90, %edx
1400115a2:     	callq	0x1428d9000 <.text+0x28d8000>
1400115a7:     	jmp	0x140011a40 <.text+0x10a40>
1400115ac:     	movb	$0x1, 0x817(%rbp)
1400115b3:     	leaq	0x2915dde(%rip), %r8    # 0x142927398
1400115ba:     	movl	$0x4, %ecx
1400115bf:     	movl	$0x18, %edx
1400115c4:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400115c9:     	jmp	0x140011a40 <.text+0x10a40>
1400115ce:     	movb	$0x1, 0x7ff(%rbp)
1400115d5:     	leaq	0x2921434(%rip), %rcx   # 0x142932a10
1400115dc:     	callq	0x1428d80d0 <.text+0x28d70d0>
1400115e1:     	jmp	0x140011a40 <.text+0x10a40>
1400115e6:     	movb	$0x1, 0x817(%rbp)
1400115ed:     	leaq	0x293cd8c(%rip), %r8    # 0x14294e380
1400115f4:     	movq	%rdx, %rcx
1400115f7:     	movq	%rdi, %rdx
1400115fa:     	callq	0x1428d9518 <.text+0x28d8518>
1400115ff:     	jmp	0x140011a40 <.text+0x10a40>
140011604:     	movb	$0x1, 0x817(%rbp)
14001160b:     	leaq	0x293cd6e(%rip), %r8    # 0x14294e380
140011612:     	movq	%rdx, %rcx
140011615:     	movq	%rdi, %rdx
140011618:     	callq	0x1428d9518 <.text+0x28d8518>
14001161d:     	jmp	0x140011a40 <.text+0x10a40>
140011622:     	movb	$0x1, 0x816(%rbp)
140011629:     	movb	$0x1, 0x815(%rbp)
140011630:     	leaq	0x29153e1(%rip), %r8    # 0x142926a18
140011637:     	xorl	%ecx, %ecx
140011639:     	xorl	%edx, %edx
14001163b:     	callq	0x1428d9518 <.text+0x28d8518>
140011640:     	jmp	0x140011a40 <.text+0x10a40>
140011645:     	movb	$0x1, 0x816(%rbp)
14001164c:     	movb	$0x1, 0x815(%rbp)
140011653:     	leaq	0x29153d6(%rip), %rcx   # 0x142926a30
14001165a:     	callq	0x1428d9310 <.text+0x28d8310>
14001165f:     	jmp	0x140011a40 <.text+0x10a40>
140011664:     	xorl	%esi, %esi
140011666:     	jmp	0x140011748 <.text+0x10748>
14001166b:     	movb	$0x1, 0x816(%rbp)
140011672:     	movb	$0x1, 0x815(%rbp)
140011679:     	leaq	0x2915250(%rip), %rcx   # 0x1429268d0
140011680:     	leaq	0x2915271(%rip), %r8    # 0x1429268f8
140011687:     	movl	$0x28, %edx
14001168c:     	callq	0x1428d9430 <.text+0x28d8430>
140011691:     	jmp	0x140011a40 <.text+0x10a40>
140011696:     	movb	$0x1, 0x7cd(%rbp)
14001169d:     	leaq	0x291519c(%rip), %r8    # 0x142926840
1400116a4:     	movl	$0x4, %ecx
1400116a9:     	movq	%rsi, %rdx
1400116ac:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400116b1:     	jmp	0x140011a40 <.text+0x10a40>
1400116b6:     	leaq	0x29151b3(%rip), %r8    # 0x142926870
1400116bd:     	movl	$0x4, %ecx
1400116c2:     	movq	%rsi, %rdx
1400116c5:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400116ca:     	jmp	0x140011a40 <.text+0x10a40>
1400116cf:     	movb	$0x1, 0x812(%rbp)
1400116d6:     	movb	$0x1, 0x811(%rbp)
1400116dd:     	leaq	0x2915174(%rip), %r8    # 0x142926858
1400116e4:     	movl	$0x4, %ecx
1400116e9:     	movq	0x670(%rbp), %rdx
1400116f0:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400116f5:     	jmp	0x140011a40 <.text+0x10a40>
1400116fa:     	xorl	%ecx, %ecx
1400116fc:     	jmp	0x140011866 <.text+0x10866>
140011701:     	leaq	0x2915c90(%rip), %r8    # 0x142927398
140011708:     	movl	$0x4, %ecx
14001170d:     	movq	%r12, %rdx
140011710:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140011715:     	jmp	0x140011a40 <.text+0x10a40>
14001171a:     	movq	%rsi, 0xd0(%rbp)
140011721:     	movq	%rdi, 0xc8(%rbp)
140011728:     	leaq	0x2915c69(%rip), %r8    # 0x142927398
14001172f:     	movl	$0x4, %ecx
140011734:     	movq	%r12, %rdx
140011737:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14001173c:     	jmp	0x140011a40 <.text+0x10a40>
140011741:     	movq	%rbx, 0x2f8(%rbp)
140011748:     	leaq	0x2915c49(%rip), %r8    # 0x142927398
14001174f:     	movq	%rsi, %rcx
140011752:     	movq	0x2f8(%rbp), %rdx
140011759:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14001175e:     	jmp	0x140011a40 <.text+0x10a40>
140011763:     	xorl	%esi, %esi
140011765:     	movb	$0x1, 0x7d0(%rbp)
14001176c:     	leaq	0x2915bbd(%rip), %r8    # 0x142927330
140011773:     	movq	%rsi, %rcx
140011776:     	movq	%rdi, %rdx
140011779:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14001177e:     	jmp	0x140011a40 <.text+0x10a40>
140011783:     	xorl	%esi, %esi
140011785:     	movb	$0x1, 0x7cc(%rbp)
14001178c:     	movb	$0x1, 0x7cb(%rbp)
140011793:     	leaq	0x2915b96(%rip), %r8    # 0x142927330
14001179a:     	movq	%rsi, %rcx
14001179d:     	movq	%rdi, %rdx
1400117a0:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400117a5:     	jmp	0x140011a40 <.text+0x10a40>
1400117aa:     	xorl	%esi, %esi
1400117ac:     	movb	$0x1, 0x7ce(%rbp)
1400117b3:     	leaq	0x2915b76(%rip), %r8    # 0x142927330
1400117ba:     	movq	%rsi, %rcx
1400117bd:     	movq	%rdi, %rdx
1400117c0:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400117c5:     	jmp	0x140011a40 <.text+0x10a40>
1400117ca:     	xorl	%esi, %esi
1400117cc:     	movb	$0x1, 0x794(%rbp)
1400117d3:     	movb	$0x1, 0x793(%rbp)
1400117da:     	leaq	0x2915b4f(%rip), %r8    # 0x142927330
1400117e1:     	movq	%rsi, %rcx
1400117e4:     	movq	%rdi, %rdx
1400117e7:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400117ec:     	jmp	0x140011a40 <.text+0x10a40>
1400117f1:     	xorl	%esi, %esi
1400117f3:     	movb	$0x1, 0x7cf(%rbp)
1400117fa:     	leaq	0x2915b2f(%rip), %r8    # 0x142927330
140011801:     	movq	%rsi, %rcx
140011804:     	movq	%rdi, %rdx
140011807:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14001180c:     	jmp	0x140011a40 <.text+0x10a40>
140011811:     	xorl	%ecx, %ecx
140011813:     	jmp	0x140011903 <.text+0x10903>
140011818:     	movb	$0x1, 0x816(%rbp)
14001181f:     	movb	$0x1, 0x815(%rbp)
140011826:     	leaq	0x291508b(%rip), %rcx   # 0x1429268b8
14001182d:     	callq	0x1428d9310 <.text+0x28d8310>
140011832:     	jmp	0x140011a40 <.text+0x10a40>
140011837:     	movb	$0x1, 0x816(%rbp)
14001183e:     	movb	$0x1, 0x815(%rbp)
140011845:     	leaq	0x2915054(%rip), %r8    # 0x1429268a0
14001184c:     	xorl	%ecx, %ecx
14001184e:     	xorl	%edx, %edx
140011850:     	callq	0x1428d9518 <.text+0x28d8518>
140011855:     	jmp	0x140011a40 <.text+0x10a40>
14001185a:     	movl	$0x4, %ecx
14001185f:     	movq	%rsi, 0x300(%rbp)
140011866:     	leaq	0x2915b2b(%rip), %r8    # 0x142927398
14001186d:     	movq	0x300(%rbp), %rdx
140011874:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140011879:     	jmp	0x140011a40 <.text+0x10a40>
14001187e:     	leaq	0x2915b13(%rip), %r8    # 0x142927398
140011885:     	movl	$0x4, %ecx
14001188a:     	movq	%r12, %rdx
14001188d:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140011892:     	jmp	0x140011a40 <.text+0x10a40>
140011897:     	movq	0x290(%rbp), %rax
14001189e:     	movq	%rax, 0xe0(%rbp)
1400118a5:     	movq	%rdi, 0xd8(%rbp)
1400118ac:     	leaq	0x2915ae5(%rip), %r8    # 0x142927398
1400118b3:     	movl	$0x4, %ecx
1400118b8:     	movq	%r12, %rdx
1400118bb:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400118c0:     	jmp	0x140011a40 <.text+0x10a40>
1400118c5:     	leaq	0x2915acc(%rip), %r8    # 0x142927398
1400118cc:     	movl	$0x4, %ecx
1400118d1:     	movq	%r12, %rdx
1400118d4:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400118d9:     	jmp	0x140011a40 <.text+0x10a40>
1400118de:     	leaq	0x2915ab3(%rip), %r8    # 0x142927398
1400118e5:     	movl	$0x4, %ecx
1400118ea:     	movq	%r12, %rdx
1400118ed:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400118f2:     	jmp	0x140011a40 <.text+0x10a40>
1400118f7:     	movl	$0x4, %ecx
1400118fc:     	movq	%r15, 0x308(%rbp)
140011903:     	leaq	0x2915a8e(%rip), %r8    # 0x142927398
14001190a:     	movq	0x308(%rbp), %rdx
140011911:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140011916:     	jmp	0x140011a40 <.text+0x10a40>
14001191b:     	movb	$0x1, 0x816(%rbp)
140011922:     	movb	$0x1, 0x815(%rbp)
140011929:     	leaq	0x293ca50(%rip), %r8    # 0x14294e380
140011930:     	movq	%rdx, %rcx
140011933:     	movq	%rdi, %rdx
140011936:     	callq	0x1428d9518 <.text+0x28d8518>
14001193b:     	jmp	0x140011a40 <.text+0x10a40>
140011940:     	movb	$0x1, 0x816(%rbp)
140011947:     	movb	$0x1, 0x815(%rbp)
14001194e:     	leaq	0x2914fbb(%rip), %r8    # 0x142926910
140011955:     	movl	$0x1, %ecx
14001195a:     	callq	0x1428d9518 <.text+0x28d8518>
14001195f:     	jmp	0x140011a40 <.text+0x10a40>
140011964:     	movb	$0x1, 0x816(%rbp)
14001196b:     	movb	$0x1, 0x815(%rbp)
140011972:     	movl	$0x4, %ecx
140011977:     	movl	$0x24, %edx
14001197c:     	callq	0x1428d9000 <.text+0x28d8000>
140011981:     	jmp	0x140011a40 <.text+0x10a40>
140011986:     	movb	$0x1, 0x816(%rbp)
14001198d:     	movb	$0x1, 0x815(%rbp)
140011994:     	leaq	0x2914f8d(%rip), %r8    # 0x142926928
14001199b:     	movl	$0x1, %ecx
1400119a0:     	callq	0x1428d9518 <.text+0x28d8518>
1400119a5:     	jmp	0x140011a40 <.text+0x10a40>
1400119aa:     	movb	$0x1, 0x816(%rbp)
1400119b1:     	movb	$0x1, 0x815(%rbp)
1400119b8:     	leaq	0x2914f81(%rip), %r8    # 0x142926940
1400119bf:     	movl	$0x1, %ecx
1400119c4:     	callq	0x1428d9518 <.text+0x28d8518>
1400119c9:     	jmp	0x140011a40 <.text+0x10a40>
1400119cb:     	movb	$0x1, 0x816(%rbp)
1400119d2:     	movb	$0x1, 0x815(%rbp)
1400119d9:     	leaq	0x2914f78(%rip), %r8    # 0x142926958
1400119e0:     	movl	$0x1, %ecx
1400119e5:     	callq	0x1428d9518 <.text+0x28d8518>
1400119ea:     	jmp	0x140011a40 <.text+0x10a40>
1400119ec:     	movb	$0x1, 0x816(%rbp)
1400119f3:     	movb	$0x1, 0x815(%rbp)
1400119fa:     	leaq	0x293c97f(%rip), %r8    # 0x14294e380
140011a01:     	movq	%rdx, %rcx
140011a04:     	movq	%rdi, %rdx
140011a07:     	callq	0x1428d9518 <.text+0x28d8518>
140011a0c:     	jmp	0x140011a40 <.text+0x10a40>
140011a0e:     	movb	$0x1, 0x816(%rbp)
140011a15:     	movb	$0x1, 0x815(%rbp)
140011a1c:     	leaq	0x293c95d(%rip), %r8    # 0x14294e380
140011a23:     	movq	%rdx, %rcx
140011a26:     	movq	%rdi, %rdx
140011a29:     	callq	0x1428d9518 <.text+0x28d8518>
140011a2e:     	jmp	0x140011a40 <.text+0x10a40>
140011a30:     	leaq	0x2914f51(%rip), %r8    # 0x142926988
140011a37:     	xorl	%ecx, %ecx
140011a39:     	xorl	%edx, %edx
140011a3b:     	callq	0x1428d9518 <.text+0x28d8518>
140011a40:     	ud2
140011a42:     	nopw	%cs:(%rax,%rax)
140011a50:     	movq	%rdx, 0x10(%rsp)
140011a55:     	pushq	%rbp
140011a56:     	pushq	%r15
140011a58:     	pushq	%r14
140011a5a:     	pushq	%r13
140011a5c:     	pushq	%r12
140011a5e:     	pushq	%rsi
140011a5f:     	pushq	%rdi
140011a60:     	pushq	%rbx
140011a61:     	subq	$0x138, %rsp            # imm = 0x138
140011a68:     	leaq	0x80(%rdx), %rbp
140011a6f:     	movaps	%xmm15, 0x90(%rsp)
140011a78:     	movaps	%xmm14, 0xa0(%rsp)
140011a81:     	movaps	%xmm13, 0xb0(%rsp)
140011a8a:     	movaps	%xmm12, 0xc0(%rsp)
140011a93:     	movaps	%xmm11, 0xd0(%rsp)
140011a9c:     	movaps	%xmm10, 0xe0(%rsp)
140011aa5:     	movaps	%xmm9, 0xf0(%rsp)
140011aae:     	movaps	%xmm8, 0x100(%rsp)
140011ab7:     	movaps	%xmm7, 0x110(%rsp)
140011abf:     	movaps	%xmm6, 0x120(%rsp)
140011ac7:     	movq	0x3e0(%rbp), %rdx
140011ace:     	testq	%rdx, %rdx
140011ad1:     	je	0x140011ae9 <.text+0x10ae9>
140011ad3:     	movq	0x3e8(%rbp), %rcx
140011ada:     	shlq	$0x2, %rdx
140011ade:     	movl	$0x4, %r8d
140011ae4:     	callq	0x140613c20 <.text+0x612c20>
140011ae9:     	movaps	0x120(%rsp), %xmm6
140011af1:     	movaps	0x110(%rsp), %xmm7
140011af9:     	movaps	0x100(%rsp), %xmm8
140011b02:     	movaps	0xf0(%rsp), %xmm9
140011b0b:     	movaps	0xe0(%rsp), %xmm10
140011b14:     	movaps	0xd0(%rsp), %xmm11
140011b1d:     	movaps	0xc0(%rsp), %xmm12
140011b26:     	movaps	0xb0(%rsp), %xmm13
140011b2f:     	movaps	0xa0(%rsp), %xmm14
140011b38:     	movaps	0x90(%rsp), %xmm15
140011b41:     	addq	$0x138, %rsp            # imm = 0x138
140011b48:     	popq	%rbx
140011b49:     	popq	%rdi
140011b4a:     	popq	%rsi
140011b4b:     	popq	%r12
140011b4d:     	popq	%r13
140011b4f:     	popq	%r14
140011b51:     	popq	%r15
140011b53:     	popq	%rbp
140011b54:     	retq
140011b55:     	nopw	%cs:(%rax,%rax)
140011b60:     	movq	%rdx, 0x10(%rsp)
140011b65:     	pushq	%rbp
140011b66:     	pushq	%r15
140011b68:     	pushq	%r14
140011b6a:     	pushq	%r13
140011b6c:     	pushq	%r12
140011b6e:     	pushq	%rsi
140011b6f:     	pushq	%rdi
140011b70:     	pushq	%rbx
140011b71:     	subq	$0x138, %rsp            # imm = 0x138
140011b78:     	leaq	0x80(%rdx), %rbp
140011b7f:     	movaps	%xmm15, 0x90(%rsp)
140011b88:     	movaps	%xmm14, 0xa0(%rsp)
140011b91:     	movaps	%xmm13, 0xb0(%rsp)
140011b9a:     	movaps	%xmm12, 0xc0(%rsp)
140011ba3:     	movaps	%xmm11, 0xd0(%rsp)
140011bac:     	movaps	%xmm10, 0xe0(%rsp)
140011bb5:     	movaps	%xmm9, 0xf0(%rsp)
140011bbe:     	movaps	%xmm8, 0x100(%rsp)
140011bc7:     	movaps	%xmm7, 0x110(%rsp)
140011bcf:     	movaps	%xmm6, 0x120(%rsp)
140011bd7:     	movq	0x500(%rbp), %rdx
140011bde:     	testq	%rdx, %rdx
140011be1:     	je	0x140011bf9 <.text+0x10bf9>
140011be3:     	movq	0x508(%rbp), %rcx
140011bea:     	shlq	$0x2, %rdx
140011bee:     	movl	$0x4, %r8d
140011bf4:     	callq	0x140613c20 <.text+0x612c20>
140011bf9:     	movaps	0x120(%rsp), %xmm6
140011c01:     	movaps	0x110(%rsp), %xmm7
140011c09:     	movaps	0x100(%rsp), %xmm8
140011c12:     	movaps	0xf0(%rsp), %xmm9
140011c1b:     	movaps	0xe0(%rsp), %xmm10
140011c24:     	movaps	0xd0(%rsp), %xmm11
140011c2d:     	movaps	0xc0(%rsp), %xmm12
140011c36:     	movaps	0xb0(%rsp), %xmm13
140011c3f:     	movaps	0xa0(%rsp), %xmm14
140011c48:     	movaps	0x90(%rsp), %xmm15
140011c51:     	addq	$0x138, %rsp            # imm = 0x138
140011c58:     	popq	%rbx
140011c59:     	popq	%rdi
140011c5a:     	popq	%rsi
140011c5b:     	popq	%r12
140011c5d:     	popq	%r13
140011c5f:     	popq	%r14
140011c61:     	popq	%r15
140011c63:     	popq	%rbp
140011c64:     	retq
140011c65:     	nopw	%cs:(%rax,%rax)
140011c70:     	movq	%rdx, 0x10(%rsp)
140011c75:     	pushq	%rbp
140011c76:     	pushq	%r15
140011c78:     	pushq	%r14
140011c7a:     	pushq	%r13
140011c7c:     	pushq	%r12
140011c7e:     	pushq	%rsi
140011c7f:     	pushq	%rdi
140011c80:     	pushq	%rbx
140011c81:     	subq	$0x138, %rsp            # imm = 0x138
140011c88:     	leaq	0x80(%rdx), %rbp
140011c8f:     	movaps	%xmm15, 0x90(%rsp)
140011c98:     	movaps	%xmm14, 0xa0(%rsp)
140011ca1:     	movaps	%xmm13, 0xb0(%rsp)
140011caa:     	movaps	%xmm12, 0xc0(%rsp)
140011cb3:     	movaps	%xmm11, 0xd0(%rsp)
140011cbc:     	movaps	%xmm10, 0xe0(%rsp)
140011cc5:     	movaps	%xmm9, 0xf0(%rsp)
140011cce:     	movaps	%xmm8, 0x100(%rsp)
140011cd7:     	movaps	%xmm7, 0x110(%rsp)
140011cdf:     	movaps	%xmm6, 0x120(%rsp)
140011ce7:     	leaq	0x6a0(%rbp), %rcx
140011cee:     	callq	0x14014ba20 <.text+0x14aa20>
140011cf3:     	movb	$0x1, 0x816(%rbp)
140011cfa:     	movb	$0x1, 0x815(%rbp)
140011d01:     	movaps	0x120(%rsp), %xmm6
140011d09:     	movaps	0x110(%rsp), %xmm7
140011d11:     	movaps	0x100(%rsp), %xmm8
140011d1a:     	movaps	0xf0(%rsp), %xmm9
140011d23:     	movaps	0xe0(%rsp), %xmm10
140011d2c:     	movaps	0xd0(%rsp), %xmm11
140011d35:     	movdqa	0xc0(%rsp), %xmm12
140011d3f:     	movdqa	0xb0(%rsp), %xmm13
140011d49:     	movaps	0xa0(%rsp), %xmm14
140011d52:     	movdqa	0x90(%rsp), %xmm15
140011d5c:     	addq	$0x138, %rsp            # imm = 0x138
140011d63:     	popq	%rbx
140011d64:     	popq	%rdi
140011d65:     	popq	%rsi
140011d66:     	popq	%r12
140011d68:     	popq	%r13
140011d6a:     	popq	%r14
140011d6c:     	popq	%r15
140011d6e:     	popq	%rbp
140011d6f:     	retq
140011d70:     	movq	%rdx, 0x10(%rsp)
140011d75:     	pushq	%rbp
140011d76:     	pushq	%r15
140011d78:     	pushq	%r14
140011d7a:     	pushq	%r13
140011d7c:     	pushq	%r12
140011d7e:     	pushq	%rsi
140011d7f:     	pushq	%rdi
140011d80:     	pushq	%rbx
140011d81:     	subq	$0x138, %rsp            # imm = 0x138
140011d88:     	leaq	0x80(%rdx), %rbp
140011d8f:     	movdqa	%xmm15, 0x90(%rsp)
140011d99:     	movaps	%xmm14, 0xa0(%rsp)
140011da2:     	movdqa	%xmm13, 0xb0(%rsp)
140011dac:     	movdqa	%xmm12, 0xc0(%rsp)
140011db6:     	movaps	%xmm11, 0xd0(%rsp)
140011dbf:     	movaps	%xmm10, 0xe0(%rsp)
140011dc8:     	movaps	%xmm9, 0xf0(%rsp)
140011dd1:     	movaps	%xmm8, 0x100(%rsp)
140011dda:     	movaps	%xmm7, 0x110(%rsp)
140011de2:     	movaps	%xmm6, 0x120(%rsp)
140011dea:     	movb	$0x0, 0x816(%rbp)
140011df1:     	movb	$0x0, 0x815(%rbp)
140011df8:     	leaq	0x6a0(%rbp), %rcx
140011dff:     	callq	0x140609eb0 <.text+0x608eb0>
140011e04:     	movb	$0x0, 0x816(%rbp)
140011e0b:     	movb	$0x0, 0x815(%rbp)
140011e12:     	movaps	0x120(%rsp), %xmm6
140011e1a:     	movaps	0x110(%rsp), %xmm7
140011e22:     	movaps	0x100(%rsp), %xmm8
140011e2b:     	movaps	0xf0(%rsp), %xmm9
140011e34:     	movaps	0xe0(%rsp), %xmm10
140011e3d:     	movaps	0xd0(%rsp), %xmm11
140011e46:     	movdqa	0xc0(%rsp), %xmm12
140011e50:     	movdqa	0xb0(%rsp), %xmm13
140011e5a:     	movaps	0xa0(%rsp), %xmm14
140011e63:     	movdqa	0x90(%rsp), %xmm15
140011e6d:     	addq	$0x138, %rsp            # imm = 0x138
140011e74:     	popq	%rbx
140011e75:     	popq	%rdi
140011e76:     	popq	%rsi
140011e77:     	popq	%r12
140011e79:     	popq	%r13
140011e7b:     	popq	%r14
140011e7d:     	popq	%r15
140011e7f:     	popq	%rbp
140011e80:     	retq
140011e81:     	nopw	%cs:(%rax,%rax)
140011e90:     	movq	%rdx, 0x10(%rsp)
140011e95:     	pushq	%rbp
140011e96:     	pushq	%r15
140011e98:     	pushq	%r14
140011e9a:     	pushq	%r13
140011e9c:     	pushq	%r12
140011e9e:     	pushq	%rsi
140011e9f:     	pushq	%rdi
140011ea0:     	pushq	%rbx
140011ea1:     	subq	$0x138, %rsp            # imm = 0x138
140011ea8:     	leaq	0x80(%rdx), %rbp
140011eaf:     	movdqa	%xmm15, 0x90(%rsp)
140011eb9:     	movaps	%xmm14, 0xa0(%rsp)
140011ec2:     	movdqa	%xmm13, 0xb0(%rsp)
140011ecc:     	movdqa	%xmm12, 0xc0(%rsp)
140011ed6:     	movaps	%xmm11, 0xd0(%rsp)
140011edf:     	movaps	%xmm10, 0xe0(%rsp)
140011ee8:     	movaps	%xmm9, 0xf0(%rsp)
140011ef1:     	movaps	%xmm8, 0x100(%rsp)
140011efa:     	movaps	%xmm7, 0x110(%rsp)
140011f02:     	movaps	%xmm6, 0x120(%rsp)
140011f0a:     	movb	$0x1, 0x816(%rbp)
140011f11:     	movb	$0x1, 0x815(%rbp)
140011f18:     	leaq	0x6a0(%rbp), %rcx
140011f1f:     	callq	0x140609eb0 <.text+0x608eb0>
140011f24:     	movb	$0x1, 0x816(%rbp)
140011f2b:     	movb	$0x1, 0x815(%rbp)
140011f32:     	movaps	0x120(%rsp), %xmm6
140011f3a:     	movaps	0x110(%rsp), %xmm7
140011f42:     	movaps	0x100(%rsp), %xmm8
140011f4b:     	movaps	0xf0(%rsp), %xmm9
140011f54:     	movaps	0xe0(%rsp), %xmm10
140011f5d:     	movaps	0xd0(%rsp), %xmm11
140011f66:     	movdqa	0xc0(%rsp), %xmm12
140011f70:     	movdqa	0xb0(%rsp), %xmm13
140011f7a:     	movaps	0xa0(%rsp), %xmm14
140011f83:     	movdqa	0x90(%rsp), %xmm15
140011f8d:     	addq	$0x138, %rsp            # imm = 0x138
140011f94:     	popq	%rbx
140011f95:     	popq	%rdi
140011f96:     	popq	%rsi
140011f97:     	popq	%r12
140011f99:     	popq	%r13
140011f9b:     	popq	%r14
140011f9d:     	popq	%r15
140011f9f:     	popq	%rbp
140011fa0:     	retq
140011fa1:     	nopw	%cs:(%rax,%rax)
140011fb0:     	movq	%rdx, 0x10(%rsp)
140011fb5:     	pushq	%rbp
140011fb6:     	pushq	%r15
140011fb8:     	pushq	%r14
140011fba:     	pushq	%r13
140011fbc:     	pushq	%r12
140011fbe:     	pushq	%rsi
140011fbf:     	pushq	%rdi
140011fc0:     	pushq	%rbx
140011fc1:     	subq	$0x138, %rsp            # imm = 0x138
140011fc8:     	leaq	0x80(%rdx), %rbp
140011fcf:     	movdqa	%xmm15, 0x90(%rsp)
140011fd9:     	movaps	%xmm14, 0xa0(%rsp)
140011fe2:     	movdqa	%xmm13, 0xb0(%rsp)
140011fec:     	movdqa	%xmm12, 0xc0(%rsp)
140011ff6:     	movaps	%xmm11, 0xd0(%rsp)
140011fff:     	movaps	%xmm10, 0xe0(%rsp)
140012008:     	movaps	%xmm9, 0xf0(%rsp)
140012011:     	movaps	%xmm8, 0x100(%rsp)
14001201a:     	movaps	%xmm7, 0x110(%rsp)
140012022:     	movaps	%xmm6, 0x120(%rsp)
14001202a:     	movb	$0x0, 0x813(%rbp)
140012031:     	leaq	0x590(%rbp), %rcx
140012038:     	callq	0x140607a30 <.text+0x606a30>
14001203d:     	movb	$0x0, 0x813(%rbp)
140012044:     	movaps	0x120(%rsp), %xmm6
14001204c:     	movaps	0x110(%rsp), %xmm7
140012054:     	movaps	0x100(%rsp), %xmm8
14001205d:     	movaps	0xf0(%rsp), %xmm9
140012066:     	movaps	0xe0(%rsp), %xmm10
14001206f:     	movaps	0xd0(%rsp), %xmm11
140012078:     	movdqa	0xc0(%rsp), %xmm12
140012082:     	movdqa	0xb0(%rsp), %xmm13
14001208c:     	movaps	0xa0(%rsp), %xmm14
140012095:     	movdqa	0x90(%rsp), %xmm15
14001209f:     	addq	$0x138, %rsp            # imm = 0x138
1400120a6:     	popq	%rbx
1400120a7:     	popq	%rdi
1400120a8:     	popq	%rsi
1400120a9:     	popq	%r12
1400120ab:     	popq	%r13
1400120ad:     	popq	%r14
1400120af:     	popq	%r15
1400120b1:     	popq	%rbp
1400120b2:     	retq
1400120b3:     	nopw	%cs:(%rax,%rax)
1400120c0:     	movq	%rdx, 0x10(%rsp)
1400120c5:     	pushq	%rbp
1400120c6:     	pushq	%r15
1400120c8:     	pushq	%r14
1400120ca:     	pushq	%r13
1400120cc:     	pushq	%r12
1400120ce:     	pushq	%rsi
1400120cf:     	pushq	%rdi
1400120d0:     	pushq	%rbx
1400120d1:     	subq	$0x138, %rsp            # imm = 0x138
1400120d8:     	leaq	0x80(%rdx), %rbp
1400120df:     	movdqa	%xmm15, 0x90(%rsp)
1400120e9:     	movaps	%xmm14, 0xa0(%rsp)
1400120f2:     	movdqa	%xmm13, 0xb0(%rsp)
1400120fc:     	movdqa	%xmm12, 0xc0(%rsp)
140012106:     	movaps	%xmm11, 0xd0(%rsp)
14001210f:     	movaps	%xmm10, 0xe0(%rsp)
140012118:     	movaps	%xmm9, 0xf0(%rsp)
140012121:     	movaps	%xmm8, 0x100(%rsp)
14001212a:     	movaps	%xmm7, 0x110(%rsp)
140012132:     	movaps	%xmm6, 0x120(%rsp)
14001213a:     	cmpq	$0x0, 0x798(%rbp)
140012142:     	jne	0x140012206 <.text+0x11206>
140012148:     	cmpq	$0x0, 0x780(%rbp)
140012150:     	jne	0x140012235 <.text+0x11235>
140012156:     	cmpq	$0x0, 0x800(%rbp)
14001215e:     	je	0x14001217d <.text+0x1117d>
140012160:     	movq	0x800(%rbp), %rdx
140012167:     	shlq	$0x2, %rdx
14001216b:     	movl	$0x4, %r8d
140012171:     	movq	0x760(%rbp), %rcx
140012178:     	callq	0x140613c20 <.text+0x612c20>
14001217d:     	leaq	0x590(%rbp), %rcx
140012184:     	callq	0x14014ba20 <.text+0x14aa20>
140012189:     	movb	$0x0, 0x816(%rbp)
140012190:     	movb	$0x0, 0x815(%rbp)
140012197:     	movaps	0x120(%rsp), %xmm6
14001219f:     	movaps	0x110(%rsp), %xmm7
1400121a7:     	movaps	0x100(%rsp), %xmm8
1400121b0:     	movaps	0xf0(%rsp), %xmm9
1400121b9:     	movaps	0xe0(%rsp), %xmm10
1400121c2:     	movaps	0xd0(%rsp), %xmm11
1400121cb:     	movdqa	0xc0(%rsp), %xmm12
1400121d5:     	movdqa	0xb0(%rsp), %xmm13
1400121df:     	movaps	0xa0(%rsp), %xmm14
1400121e8:     	movdqa	0x90(%rsp), %xmm15
1400121f2:     	addq	$0x138, %rsp            # imm = 0x138
1400121f9:     	popq	%rbx
1400121fa:     	popq	%rdi
1400121fb:     	popq	%rsi
1400121fc:     	popq	%r12
1400121fe:     	popq	%r13
140012200:     	popq	%r14
140012202:     	popq	%r15
140012204:     	popq	%rbp
140012205:     	retq
140012206:     	movq	0x798(%rbp), %rax
14001220d:     	leaq	(,%rax,4), %rdx
140012215:     	movl	$0x4, %r8d
14001221b:     	movq	0x7f0(%rbp), %rcx
140012222:     	callq	0x140613c20 <.text+0x612c20>
140012227:     	cmpq	$0x0, 0x780(%rbp)
14001222f:     	je	0x140012156 <.text+0x11156>
140012235:     	movq	0x780(%rbp), %rdx
14001223c:     	shlq	$0x3, %rdx
140012240:     	movl	$0x4, %r8d
140012246:     	movq	0x530(%rbp), %rcx
14001224d:     	callq	0x140613c20 <.text+0x612c20>
140012252:     	cmpq	$0x0, 0x800(%rbp)
14001225a:     	jne	0x140012160 <.text+0x11160>
140012260:     	jmp	0x14001217d <.text+0x1117d>
140012265:     	nopw	%cs:(%rax,%rax)
140012270:     	movq	%rdx, 0x10(%rsp)
140012275:     	pushq	%rbp
140012276:     	pushq	%r15
140012278:     	pushq	%r14
14001227a:     	pushq	%r13
14001227c:     	pushq	%r12
14001227e:     	pushq	%rsi
14001227f:     	pushq	%rdi
140012280:     	pushq	%rbx
140012281:     	subq	$0x138, %rsp            # imm = 0x138
140012288:     	leaq	0x80(%rdx), %rbp
14001228f:     	movdqa	%xmm15, 0x90(%rsp)
140012299:     	movaps	%xmm14, 0xa0(%rsp)
1400122a2:     	movdqa	%xmm13, 0xb0(%rsp)
1400122ac:     	movdqa	%xmm12, 0xc0(%rsp)
1400122b6:     	movaps	%xmm11, 0xd0(%rsp)
1400122bf:     	movaps	%xmm10, 0xe0(%rsp)
1400122c8:     	movaps	%xmm9, 0xf0(%rsp)
1400122d1:     	movaps	%xmm8, 0x100(%rsp)
1400122da:     	movaps	%xmm7, 0x110(%rsp)
1400122e2:     	movaps	%xmm6, 0x120(%rsp)
1400122ea:     	leaq	0x590(%rbp), %rcx
1400122f1:     	callq	0x140609eb0 <.text+0x608eb0>
1400122f6:     	movaps	0x120(%rsp), %xmm6
1400122fe:     	movaps	0x110(%rsp), %xmm7
140012306:     	movaps	0x100(%rsp), %xmm8
14001230f:     	movaps	0xf0(%rsp), %xmm9
140012318:     	movaps	0xe0(%rsp), %xmm10
140012321:     	movaps	0xd0(%rsp), %xmm11
14001232a:     	movdqa	0xc0(%rsp), %xmm12
140012334:     	movdqa	0xb0(%rsp), %xmm13
14001233e:     	movaps	0xa0(%rsp), %xmm14
140012347:     	movdqa	0x90(%rsp), %xmm15
140012351:     	addq	$0x138, %rsp            # imm = 0x138
140012358:     	popq	%rbx
140012359:     	popq	%rdi
14001235a:     	popq	%rsi
14001235b:     	popq	%r12
14001235d:     	popq	%r13
14001235f:     	popq	%r14
140012361:     	popq	%r15
140012363:     	popq	%rbp
140012364:     	retq
140012365:     	nopw	%cs:(%rax,%rax)
140012370:     	movq	%rdx, 0x10(%rsp)
140012375:     	pushq	%rbp
140012376:     	pushq	%r15
140012378:     	pushq	%r14
14001237a:     	pushq	%r13
14001237c:     	pushq	%r12
14001237e:     	pushq	%rsi
14001237f:     	pushq	%rdi
140012380:     	pushq	%rbx
140012381:     	subq	$0x138, %rsp            # imm = 0x138
140012388:     	leaq	0x80(%rdx), %rbp
14001238f:     	movaps	%xmm15, 0x90(%rsp)
140012398:     	movaps	%xmm14, 0xa0(%rsp)
1400123a1:     	movaps	%xmm13, 0xb0(%rsp)
1400123aa:     	movaps	%xmm12, 0xc0(%rsp)
1400123b3:     	movaps	%xmm11, 0xd0(%rsp)
1400123bc:     	movaps	%xmm10, 0xe0(%rsp)
1400123c5:     	movaps	%xmm9, 0xf0(%rsp)
1400123ce:     	movaps	%xmm8, 0x100(%rsp)
1400123d7:     	movaps	%xmm7, 0x110(%rsp)
1400123df:     	movaps	%xmm6, 0x120(%rsp)
1400123e7:     	movb	$0x0, 0x812(%rbp)
1400123ee:     	movb	$0x0, 0x811(%rbp)
1400123f5:     	leaq	0x6a0(%rbp), %rcx
1400123fc:     	callq	0x140609eb0 <.text+0x608eb0>
140012401:     	movb	$0x0, 0x812(%rbp)
140012408:     	movb	$0x0, 0x811(%rbp)
14001240f:     	movaps	0x120(%rsp), %xmm6
140012417:     	movaps	0x110(%rsp), %xmm7
14001241f:     	movaps	0x100(%rsp), %xmm8
140012428:     	movaps	0xf0(%rsp), %xmm9
140012431:     	movaps	0xe0(%rsp), %xmm10
14001243a:     	movapd	0xd0(%rsp), %xmm11
140012444:     	movaps	0xc0(%rsp), %xmm12
14001244d:     	movaps	0xb0(%rsp), %xmm13
140012456:     	movaps	0xa0(%rsp), %xmm14
14001245f:     	movaps	0x90(%rsp), %xmm15
140012468:     	addq	$0x138, %rsp            # imm = 0x138
14001246f:     	popq	%rbx
140012470:     	popq	%rdi
140012471:     	popq	%rsi
140012472:     	popq	%r12
140012474:     	popq	%r13
140012476:     	popq	%r14
140012478:     	popq	%r15
14001247a:     	popq	%rbp
14001247b:     	retq
14001247c:     	nopl	(%rax)
140012480:     	movq	%rdx, 0x10(%rsp)
140012485:     	pushq	%rbp
140012486:     	pushq	%r15
140012488:     	pushq	%r14
14001248a:     	pushq	%r13
14001248c:     	pushq	%r12
14001248e:     	pushq	%rsi
14001248f:     	pushq	%rdi
140012490:     	pushq	%rbx
140012491:     	subq	$0x138, %rsp            # imm = 0x138
140012498:     	leaq	0x80(%rdx), %rbp
14001249f:     	movaps	%xmm15, 0x90(%rsp)
1400124a8:     	movaps	%xmm14, 0xa0(%rsp)
1400124b1:     	movaps	%xmm13, 0xb0(%rsp)
1400124ba:     	movaps	%xmm12, 0xc0(%rsp)
1400124c3:     	movaps	%xmm11, 0xd0(%rsp)
1400124cc:     	movaps	%xmm10, 0xe0(%rsp)
1400124d5:     	movaps	%xmm9, 0xf0(%rsp)
1400124de:     	movaps	%xmm8, 0x100(%rsp)
1400124e7:     	movaps	%xmm7, 0x110(%rsp)
1400124ef:     	movaps	%xmm6, 0x120(%rsp)
1400124f7:     	movb	$0x0, 0x7ce(%rbp)
1400124fe:     	leaq	0x6a0(%rbp), %rcx
140012505:     	callq	0x140607a30 <.text+0x606a30>
14001250a:     	movb	$0x0, 0x7ce(%rbp)
140012511:     	movaps	0x120(%rsp), %xmm6
140012519:     	movaps	0x110(%rsp), %xmm7
140012521:     	movaps	0x100(%rsp), %xmm8
14001252a:     	movaps	0xf0(%rsp), %xmm9
140012533:     	movaps	0xe0(%rsp), %xmm10
14001253c:     	movaps	0xd0(%rsp), %xmm11
140012545:     	movaps	0xc0(%rsp), %xmm12
14001254e:     	movaps	0xb0(%rsp), %xmm13
140012557:     	movaps	0xa0(%rsp), %xmm14
140012560:     	movaps	0x90(%rsp), %xmm15
140012569:     	addq	$0x138, %rsp            # imm = 0x138
140012570:     	popq	%rbx
140012571:     	popq	%rdi
140012572:     	popq	%rsi
140012573:     	popq	%r12
140012575:     	popq	%r13
140012577:     	popq	%r14
140012579:     	popq	%r15
14001257b:     	popq	%rbp
14001257c:     	retq
14001257d:     	nopl	(%rax)
140012580:     	movq	%rdx, 0x10(%rsp)
140012585:     	pushq	%rbp
140012586:     	pushq	%r15
140012588:     	pushq	%r14
14001258a:     	pushq	%r13
14001258c:     	pushq	%r12
14001258e:     	pushq	%rsi
14001258f:     	pushq	%rdi
140012590:     	pushq	%rbx
140012591:     	subq	$0x138, %rsp            # imm = 0x138
140012598:     	leaq	0x80(%rdx), %rbp
14001259f:     	movdqa	%xmm15, 0x90(%rsp)
1400125a9:     	movaps	%xmm14, 0xa0(%rsp)
1400125b2:     	movdqa	%xmm13, 0xb0(%rsp)
1400125bc:     	movdqa	%xmm12, 0xc0(%rsp)
1400125c6:     	movaps	%xmm11, 0xd0(%rsp)
1400125cf:     	movaps	%xmm10, 0xe0(%rsp)
1400125d8:     	movaps	%xmm9, 0xf0(%rsp)
1400125e1:     	movaps	%xmm8, 0x100(%rsp)
1400125ea:     	movaps	%xmm7, 0x110(%rsp)
1400125f2:     	movaps	%xmm6, 0x120(%rsp)
1400125fa:     	cmpl	$0xb, 0x140(%rbp)
140012601:     	je	0x14001267e <.text+0x1167e>
140012603:     	leaq	0x140(%rbp), %rcx
14001260a:     	callq	0x1400297d0 <.text+0x287d0>
14001260f:     	movaps	0x120(%rsp), %xmm6
140012617:     	movaps	0x110(%rsp), %xmm7
14001261f:     	movaps	0x100(%rsp), %xmm8
140012628:     	movaps	0xf0(%rsp), %xmm9
140012631:     	movaps	0xe0(%rsp), %xmm10
14001263a:     	movaps	0xd0(%rsp), %xmm11
140012643:     	movdqa	0xc0(%rsp), %xmm12
14001264d:     	movdqa	0xb0(%rsp), %xmm13
140012657:     	movaps	0xa0(%rsp), %xmm14
140012660:     	movdqa	0x90(%rsp), %xmm15
14001266a:     	addq	$0x138, %rsp            # imm = 0x138
140012671:     	popq	%rbx
140012672:     	popq	%rdi
140012673:     	popq	%rsi
140012674:     	popq	%r12
140012676:     	popq	%r13
140012678:     	popq	%r14
14001267a:     	popq	%r15
14001267c:     	popq	%rbp
14001267d:     	retq
14001267e:     	movaps	0x120(%rsp), %xmm6
140012686:     	movaps	0x110(%rsp), %xmm7
14001268e:     	movaps	0x100(%rsp), %xmm8
140012697:     	movaps	0xf0(%rsp), %xmm9
1400126a0:     	movaps	0xe0(%rsp), %xmm10
1400126a9:     	movaps	0xd0(%rsp), %xmm11
1400126b2:     	movdqa	0xc0(%rsp), %xmm12
1400126bc:     	movdqa	0xb0(%rsp), %xmm13
1400126c6:     	movaps	0xa0(%rsp), %xmm14
1400126cf:     	movdqa	0x90(%rsp), %xmm15
1400126d9:     	addq	$0x138, %rsp            # imm = 0x138
1400126e0:     	popq	%rbx
1400126e1:     	popq	%rdi
1400126e2:     	popq	%rsi
1400126e3:     	popq	%r12
1400126e5:     	popq	%r13
1400126e7:     	popq	%r14
1400126e9:     	popq	%r15
1400126eb:     	popq	%rbp
1400126ec:     	retq
1400126ed:     	nopl	(%rax)
1400126f0:     	movq	%rdx, 0x10(%rsp)
1400126f5:     	pushq	%rbp
1400126f6:     	pushq	%r15
1400126f8:     	pushq	%r14
1400126fa:     	pushq	%r13
1400126fc:     	pushq	%r12
1400126fe:     	pushq	%rsi
1400126ff:     	pushq	%rdi
140012700:     	pushq	%rbx
140012701:     	subq	$0x138, %rsp            # imm = 0x138
140012708:     	leaq	0x80(%rdx), %rbp
14001270f:     	movaps	%xmm15, 0x90(%rsp)
140012718:     	movaps	%xmm14, 0xa0(%rsp)
140012721:     	movaps	%xmm13, 0xb0(%rsp)
14001272a:     	movaps	%xmm12, 0xc0(%rsp)
140012733:     	movaps	%xmm11, 0xd0(%rsp)
14001273c:     	movaps	%xmm10, 0xe0(%rsp)
140012745:     	movaps	%xmm9, 0xf0(%rsp)
14001274e:     	movaps	%xmm8, 0x100(%rsp)
140012757:     	movaps	%xmm7, 0x110(%rsp)
14001275f:     	movaps	%xmm6, 0x120(%rsp)
140012767:     	leaq	0x6a0(%rbp), %rcx
14001276e:     	callq	0x14014ba20 <.text+0x14aa20>
140012773:     	movb	$0x1, 0x812(%rbp)
14001277a:     	movb	$0x0, 0x811(%rbp)
140012781:     	movaps	0x120(%rsp), %xmm6
140012789:     	movaps	0x110(%rsp), %xmm7
140012791:     	movaps	0x100(%rsp), %xmm8
14001279a:     	movaps	0xf0(%rsp), %xmm9
1400127a3:     	movaps	0xe0(%rsp), %xmm10
1400127ac:     	movapd	0xd0(%rsp), %xmm11
1400127b6:     	movaps	0xc0(%rsp), %xmm12
1400127bf:     	movaps	0xb0(%rsp), %xmm13
1400127c8:     	movaps	0xa0(%rsp), %xmm14
1400127d1:     	movaps	0x90(%rsp), %xmm15
1400127da:     	addq	$0x138, %rsp            # imm = 0x138
1400127e1:     	popq	%rbx
1400127e2:     	popq	%rdi
1400127e3:     	popq	%rsi
1400127e4:     	popq	%r12
1400127e6:     	popq	%r13
1400127e8:     	popq	%r14
1400127ea:     	popq	%r15
1400127ec:     	popq	%rbp
1400127ed:     	retq
1400127ee:     	nop
1400127f0:     	movq	%rdx, 0x10(%rsp)
1400127f5:     	pushq	%rbp
1400127f6:     	pushq	%r15
1400127f8:     	pushq	%r14
1400127fa:     	pushq	%r13
1400127fc:     	pushq	%r12
1400127fe:     	pushq	%rsi
1400127ff:     	pushq	%rdi
140012800:     	pushq	%rbx
140012801:     	subq	$0x138, %rsp            # imm = 0x138
140012808:     	leaq	0x80(%rdx), %rbp
14001280f:     	movaps	%xmm15, 0x90(%rsp)
140012818:     	movaps	%xmm14, 0xa0(%rsp)
140012821:     	movaps	%xmm13, 0xb0(%rsp)
14001282a:     	movaps	%xmm12, 0xc0(%rsp)
140012833:     	movaps	%xmm11, 0xd0(%rsp)
14001283c:     	movaps	%xmm10, 0xe0(%rsp)
140012845:     	movaps	%xmm9, 0xf0(%rsp)
14001284e:     	movaps	%xmm8, 0x100(%rsp)
140012857:     	movaps	%xmm7, 0x110(%rsp)
14001285f:     	movaps	%xmm6, 0x120(%rsp)
140012867:     	movq	0x760(%rbp), %rax
14001286e:     	testq	%rax, %rax
140012871:     	je	0x14001288c <.text+0x1188c>
140012873:     	movq	%rax, %rdx
140012876:     	shlq	$0x2, %rdx
14001287a:     	movl	$0x4, %r8d
140012880:     	movq	0x7f0(%rbp), %rcx
140012887:     	callq	0x140613c20 <.text+0x612c20>
14001288c:     	movb	$0x1, 0x812(%rbp)
140012893:     	movb	$0x1, 0x811(%rbp)
14001289a:     	movaps	0x120(%rsp), %xmm6
1400128a2:     	movaps	0x110(%rsp), %xmm7
1400128aa:     	movaps	0x100(%rsp), %xmm8
1400128b3:     	movaps	0xf0(%rsp), %xmm9
1400128bc:     	movaps	0xe0(%rsp), %xmm10
1400128c5:     	movapd	0xd0(%rsp), %xmm11
1400128cf:     	movaps	0xc0(%rsp), %xmm12
1400128d8:     	movaps	0xb0(%rsp), %xmm13
1400128e1:     	movaps	0xa0(%rsp), %xmm14
1400128ea:     	movaps	0x90(%rsp), %xmm15
1400128f3:     	addq	$0x138, %rsp            # imm = 0x138
1400128fa:     	popq	%rbx
1400128fb:     	popq	%rdi
1400128fc:     	popq	%rsi
1400128fd:     	popq	%r12
1400128ff:     	popq	%r13
140012901:     	popq	%r14
140012903:     	popq	%r15
140012905:     	popq	%rbp
140012906:     	retq
140012907:     	nopw	(%rax,%rax)
140012910:     	movq	%rdx, 0x10(%rsp)
140012915:     	pushq	%rbp
140012916:     	pushq	%r15
140012918:     	pushq	%r14
14001291a:     	pushq	%r13
14001291c:     	pushq	%r12
14001291e:     	pushq	%rsi
14001291f:     	pushq	%rdi
140012920:     	pushq	%rbx
140012921:     	subq	$0x138, %rsp            # imm = 0x138
140012928:     	leaq	0x80(%rdx), %rbp
14001292f:     	movdqa	%xmm15, 0x90(%rsp)
140012939:     	movaps	%xmm14, 0xa0(%rsp)
140012942:     	movdqa	%xmm13, 0xb0(%rsp)
14001294c:     	movdqa	%xmm12, 0xc0(%rsp)
140012956:     	movaps	%xmm11, 0xd0(%rsp)
14001295f:     	movaps	%xmm10, 0xe0(%rsp)
140012968:     	movaps	%xmm9, 0xf0(%rsp)
140012971:     	movaps	%xmm8, 0x100(%rsp)
14001297a:     	movaps	%xmm7, 0x110(%rsp)
140012982:     	movaps	%xmm6, 0x120(%rsp)
14001298a:     	movq	0x800(%rbp), %rax
140012991:     	incq	(%rax)
140012994:     	movaps	0x120(%rsp), %xmm6
14001299c:     	movaps	0x110(%rsp), %xmm7
1400129a4:     	movaps	0x100(%rsp), %xmm8
1400129ad:     	movaps	0xf0(%rsp), %xmm9
1400129b6:     	movaps	0xe0(%rsp), %xmm10
1400129bf:     	movaps	0xd0(%rsp), %xmm11
1400129c8:     	movaps	0xc0(%rsp), %xmm12
1400129d1:     	movaps	0xb0(%rsp), %xmm13
1400129da:     	movaps	0xa0(%rsp), %xmm14
1400129e3:     	movaps	0x90(%rsp), %xmm15
1400129ec:     	addq	$0x138, %rsp            # imm = 0x138
1400129f3:     	popq	%rbx
1400129f4:     	popq	%rdi
1400129f5:     	popq	%rsi
1400129f6:     	popq	%r12
1400129f8:     	popq	%r13
1400129fa:     	popq	%r14
1400129fc:     	popq	%r15
1400129fe:     	popq	%rbp
1400129ff:     	retq
140012a00:     	movq	%rdx, 0x10(%rsp)
140012a05:     	pushq	%rbp
140012a06:     	pushq	%r15
140012a08:     	pushq	%r14
140012a0a:     	pushq	%r13
140012a0c:     	pushq	%r12
140012a0e:     	pushq	%rsi
140012a0f:     	pushq	%rdi
140012a10:     	pushq	%rbx
140012a11:     	subq	$0x138, %rsp            # imm = 0x138
140012a18:     	leaq	0x80(%rdx), %rbp
140012a1f:     	movaps	%xmm15, 0x90(%rsp)
140012a28:     	movaps	%xmm14, 0xa0(%rsp)
140012a31:     	movaps	%xmm13, 0xb0(%rsp)
140012a3a:     	movaps	%xmm12, 0xc0(%rsp)
140012a43:     	movdqa	%xmm11, 0xd0(%rsp)
140012a4d:     	movaps	%xmm10, 0xe0(%rsp)
140012a56:     	movaps	%xmm9, 0xf0(%rsp)
140012a5f:     	movaps	%xmm8, 0x100(%rsp)
140012a68:     	movaps	%xmm7, 0x110(%rsp)
140012a70:     	movaps	%xmm6, 0x120(%rsp)
140012a78:     	movb	$0x1, 0x817(%rbp)
140012a7f:     	leaq	0x6a0(%rbp), %rcx
140012a86:     	callq	0x14014ba20 <.text+0x14aa20>
140012a8b:     	movb	$0x1, 0x817(%rbp)
140012a92:     	movaps	0x120(%rsp), %xmm6
140012a9a:     	movaps	0x110(%rsp), %xmm7
140012aa2:     	movaps	0x100(%rsp), %xmm8
140012aab:     	movaps	0xf0(%rsp), %xmm9
140012ab4:     	movaps	0xe0(%rsp), %xmm10
140012abd:     	movaps	0xd0(%rsp), %xmm11
140012ac6:     	movdqa	0xc0(%rsp), %xmm12
140012ad0:     	movdqa	0xb0(%rsp), %xmm13
140012ada:     	movaps	0xa0(%rsp), %xmm14
140012ae3:     	movdqa	0x90(%rsp), %xmm15
140012aed:     	addq	$0x138, %rsp            # imm = 0x138
140012af4:     	popq	%rbx
140012af5:     	popq	%rdi
140012af6:     	popq	%rsi
140012af7:     	popq	%r12
140012af9:     	popq	%r13
140012afb:     	popq	%r14
140012afd:     	popq	%r15
140012aff:     	popq	%rbp
140012b00:     	retq
140012b01:     	nopw	%cs:(%rax,%rax)
140012b10:     	movq	%rdx, 0x10(%rsp)
140012b15:     	pushq	%rbp
140012b16:     	pushq	%r15
140012b18:     	pushq	%r14
140012b1a:     	pushq	%r13
140012b1c:     	pushq	%r12
140012b1e:     	pushq	%rsi
140012b1f:     	pushq	%rdi
140012b20:     	pushq	%rbx
140012b21:     	subq	$0x138, %rsp            # imm = 0x138
140012b28:     	leaq	0x80(%rdx), %rbp
140012b2f:     	movaps	%xmm15, 0x90(%rsp)
140012b38:     	movaps	%xmm14, 0xa0(%rsp)
140012b41:     	movaps	%xmm13, 0xb0(%rsp)
140012b4a:     	movaps	%xmm12, 0xc0(%rsp)
140012b53:     	movaps	%xmm11, 0xd0(%rsp)
140012b5c:     	movaps	%xmm10, 0xe0(%rsp)
140012b65:     	movaps	%xmm9, 0xf0(%rsp)
140012b6e:     	movaps	%xmm8, 0x100(%rsp)
140012b77:     	movaps	%xmm7, 0x110(%rsp)
140012b7f:     	movaps	%xmm6, 0x120(%rsp)
140012b87:     	movb	$0x1, 0x817(%rbp)
140012b8e:     	leaq	0x6a0(%rbp), %rcx
140012b95:     	callq	0x14014ba20 <.text+0x14aa20>
140012b9a:     	movb	$0x1, 0x817(%rbp)
140012ba1:     	movaps	0x120(%rsp), %xmm6
140012ba9:     	movaps	0x110(%rsp), %xmm7
140012bb1:     	movaps	0x100(%rsp), %xmm8
140012bba:     	movaps	0xf0(%rsp), %xmm9
140012bc3:     	movaps	0xe0(%rsp), %xmm10
140012bcc:     	movaps	0xd0(%rsp), %xmm11
140012bd5:     	movdqa	0xc0(%rsp), %xmm12
140012bdf:     	movdqa	0xb0(%rsp), %xmm13
140012be9:     	movaps	0xa0(%rsp), %xmm14
140012bf2:     	movdqa	0x90(%rsp), %xmm15
140012bfc:     	addq	$0x138, %rsp            # imm = 0x138
140012c03:     	popq	%rbx
140012c04:     	popq	%rdi
140012c05:     	popq	%rsi
140012c06:     	popq	%r12
140012c08:     	popq	%r13
140012c0a:     	popq	%r14
140012c0c:     	popq	%r15
140012c0e:     	popq	%rbp
140012c0f:     	retq
140012c10:     	movq	%rdx, 0x10(%rsp)
140012c15:     	pushq	%rbp
140012c16:     	pushq	%r15
140012c18:     	pushq	%r14
140012c1a:     	pushq	%r13
140012c1c:     	pushq	%r12
140012c1e:     	pushq	%rsi
140012c1f:     	pushq	%rdi
140012c20:     	pushq	%rbx
140012c21:     	subq	$0x138, %rsp            # imm = 0x138
140012c28:     	leaq	0x80(%rdx), %rbp
140012c2f:     	movaps	%xmm15, 0x90(%rsp)
140012c38:     	movaps	%xmm14, 0xa0(%rsp)
140012c41:     	movaps	%xmm13, 0xb0(%rsp)
140012c4a:     	movaps	%xmm12, 0xc0(%rsp)
140012c53:     	movaps	%xmm11, 0xd0(%rsp)
140012c5c:     	movaps	%xmm10, 0xe0(%rsp)
140012c65:     	movaps	%xmm9, 0xf0(%rsp)
140012c6e:     	movaps	%xmm8, 0x100(%rsp)
140012c77:     	movaps	%xmm7, 0x110(%rsp)
140012c7f:     	movaps	%xmm6, 0x120(%rsp)
140012c87:     	movzbl	0x7ce(%rbp), %ebx
140012c8e:     	testb	$0x1, %bl
140012c91:     	je	0x140012d30 <.text+0x11d30>
140012c97:     	cmpq	$0x0, 0x760(%rbp)
140012c9f:     	je	0x140012cbe <.text+0x11cbe>
140012ca1:     	movq	0x760(%rbp), %rdx
140012ca8:     	shlq	$0x2, %rdx
140012cac:     	movl	$0x4, %r8d
140012cb2:     	movq	0x7f0(%rbp), %rcx
140012cb9:     	callq	0x140613c20 <.text+0x612c20>
140012cbe:     	movb	%bl, 0x7cf(%rbp)
140012cc4:     	movaps	0x120(%rsp), %xmm6
140012ccc:     	movaps	0x110(%rsp), %xmm7
140012cd4:     	movaps	0x100(%rsp), %xmm8
140012cdd:     	movaps	0xf0(%rsp), %xmm9
140012ce6:     	movaps	0xe0(%rsp), %xmm10
140012cef:     	movaps	0xd0(%rsp), %xmm11
140012cf8:     	movaps	0xc0(%rsp), %xmm12
140012d01:     	movaps	0xb0(%rsp), %xmm13
140012d0a:     	movaps	0xa0(%rsp), %xmm14
140012d13:     	movaps	0x90(%rsp), %xmm15
140012d1c:     	addq	$0x138, %rsp            # imm = 0x138
140012d23:     	popq	%rbx
140012d24:     	popq	%rdi
140012d25:     	popq	%rsi
140012d26:     	popq	%r12
140012d28:     	popq	%r13
140012d2a:     	popq	%r14
140012d2c:     	popq	%r15
140012d2e:     	popq	%rbp
140012d2f:     	retq
140012d30:     	movb	%bl, 0x7cf(%rbp)
140012d36:     	movaps	0x120(%rsp), %xmm6
140012d3e:     	movaps	0x110(%rsp), %xmm7
140012d46:     	movaps	0x100(%rsp), %xmm8
140012d4f:     	movaps	0xf0(%rsp), %xmm9
140012d58:     	movaps	0xe0(%rsp), %xmm10
140012d61:     	movaps	0xd0(%rsp), %xmm11
140012d6a:     	movaps	0xc0(%rsp), %xmm12
140012d73:     	movaps	0xb0(%rsp), %xmm13
140012d7c:     	movaps	0xa0(%rsp), %xmm14
140012d85:     	movaps	0x90(%rsp), %xmm15
140012d8e:     	addq	$0x138, %rsp            # imm = 0x138
140012d95:     	popq	%rbx
140012d96:     	popq	%rdi
140012d97:     	popq	%rsi
140012d98:     	popq	%r12
140012d9a:     	popq	%r13
140012d9c:     	popq	%r14
140012d9e:     	popq	%r15
140012da0:     	popq	%rbp
140012da1:     	retq
140012da2:     	nopw	%cs:(%rax,%rax)
140012db0:     	movq	%rdx, 0x10(%rsp)
140012db5:     	pushq	%rbp
140012db6:     	pushq	%r15
140012db8:     	pushq	%r14
140012dba:     	pushq	%r13
140012dbc:     	pushq	%r12
140012dbe:     	pushq	%rsi
140012dbf:     	pushq	%rdi
140012dc0:     	pushq	%rbx
140012dc1:     	subq	$0x138, %rsp            # imm = 0x138
140012dc8:     	leaq	0x80(%rdx), %rbp
140012dcf:     	movdqa	%xmm15, 0x90(%rsp)
140012dd9:     	movaps	%xmm14, 0xa0(%rsp)
140012de2:     	movdqa	%xmm13, 0xb0(%rsp)
140012dec:     	movdqa	%xmm12, 0xc0(%rsp)
140012df6:     	movaps	%xmm11, 0xd0(%rsp)
140012dff:     	movaps	%xmm10, 0xe0(%rsp)
140012e08:     	movaps	%xmm9, 0xf0(%rsp)
140012e11:     	movaps	%xmm8, 0x100(%rsp)
140012e1a:     	movaps	%xmm7, 0x110(%rsp)
140012e22:     	movaps	%xmm6, 0x120(%rsp)
140012e2a:     	movq	0x780(%rbp), %rax
140012e31:     	incq	(%rax)
140012e34:     	movaps	0x120(%rsp), %xmm6
140012e3c:     	movaps	0x110(%rsp), %xmm7
140012e44:     	movaps	0x100(%rsp), %xmm8
140012e4d:     	movaps	0xf0(%rsp), %xmm9
140012e56:     	movaps	0xe0(%rsp), %xmm10
140012e5f:     	movaps	0xd0(%rsp), %xmm11
140012e68:     	movaps	0xc0(%rsp), %xmm12
140012e71:     	movaps	0xb0(%rsp), %xmm13
140012e7a:     	movaps	0xa0(%rsp), %xmm14
140012e83:     	movaps	0x90(%rsp), %xmm15
140012e8c:     	addq	$0x138, %rsp            # imm = 0x138
140012e93:     	popq	%rbx
140012e94:     	popq	%rdi
140012e95:     	popq	%rsi
140012e96:     	popq	%r12
140012e98:     	popq	%r13
140012e9a:     	popq	%r14
140012e9c:     	popq	%r15
140012e9e:     	popq	%rbp
140012e9f:     	retq
140012ea0:     	movq	%rdx, 0x10(%rsp)
140012ea5:     	pushq	%rbp
140012ea6:     	pushq	%r15
140012ea8:     	pushq	%r14
140012eaa:     	pushq	%r13
140012eac:     	pushq	%r12
140012eae:     	pushq	%rsi
140012eaf:     	pushq	%rdi
140012eb0:     	pushq	%rbx
140012eb1:     	subq	$0x138, %rsp            # imm = 0x138
140012eb8:     	leaq	0x80(%rdx), %rbp
140012ebf:     	movaps	%xmm15, 0x90(%rsp)
140012ec8:     	movaps	%xmm14, 0xa0(%rsp)
140012ed1:     	movaps	%xmm13, 0xb0(%rsp)
140012eda:     	movaps	%xmm12, 0xc0(%rsp)
140012ee3:     	movaps	%xmm11, 0xd0(%rsp)
140012eec:     	movaps	%xmm10, 0xe0(%rsp)
140012ef5:     	movaps	%xmm9, 0xf0(%rsp)
140012efe:     	movaps	%xmm8, 0x100(%rsp)
140012f07:     	movaps	%xmm7, 0x110(%rsp)
140012f0f:     	movaps	%xmm6, 0x120(%rsp)
140012f17:     	movzbl	0x7cf(%rbp), %ebx
140012f1e:     	testb	$0x1, %bl
140012f21:     	je	0x140012fc0 <.text+0x11fc0>
140012f27:     	cmpq	$0x0, 0x800(%rbp)
140012f2f:     	je	0x140012f4e <.text+0x11f4e>
140012f31:     	movq	0x800(%rbp), %rdx
140012f38:     	shlq	$0x3, %rdx
140012f3c:     	movl	$0x4, %r8d
140012f42:     	movq	0x780(%rbp), %rcx
140012f49:     	callq	0x140613c20 <.text+0x612c20>
140012f4e:     	movb	%bl, 0x7d0(%rbp)
140012f54:     	movaps	0x120(%rsp), %xmm6
140012f5c:     	movaps	0x110(%rsp), %xmm7
140012f64:     	movaps	0x100(%rsp), %xmm8
140012f6d:     	movaps	0xf0(%rsp), %xmm9
140012f76:     	movaps	0xe0(%rsp), %xmm10
140012f7f:     	movaps	0xd0(%rsp), %xmm11
140012f88:     	movaps	0xc0(%rsp), %xmm12
140012f91:     	movaps	0xb0(%rsp), %xmm13
140012f9a:     	movaps	0xa0(%rsp), %xmm14
140012fa3:     	movaps	0x90(%rsp), %xmm15
140012fac:     	addq	$0x138, %rsp            # imm = 0x138
140012fb3:     	popq	%rbx
140012fb4:     	popq	%rdi
140012fb5:     	popq	%rsi
140012fb6:     	popq	%r12
140012fb8:     	popq	%r13
140012fba:     	popq	%r14
140012fbc:     	popq	%r15
140012fbe:     	popq	%rbp
140012fbf:     	retq
140012fc0:     	movb	%bl, 0x7d0(%rbp)
140012fc6:     	movaps	0x120(%rsp), %xmm6
140012fce:     	movaps	0x110(%rsp), %xmm7
140012fd6:     	movaps	0x100(%rsp), %xmm8
140012fdf:     	movaps	0xf0(%rsp), %xmm9
140012fe8:     	movaps	0xe0(%rsp), %xmm10
140012ff1:     	movaps	0xd0(%rsp), %xmm11
140012ffa:     	movaps	0xc0(%rsp), %xmm12
140013003:     	movaps	0xb0(%rsp), %xmm13
14001300c:     	movaps	0xa0(%rsp), %xmm14
140013015:     	movaps	0x90(%rsp), %xmm15
14001301e:     	addq	$0x138, %rsp            # imm = 0x138
140013025:     	popq	%rbx
140013026:     	popq	%rdi
140013027:     	popq	%rsi
140013028:     	popq	%r12
14001302a:     	popq	%r13
14001302c:     	popq	%r14
14001302e:     	popq	%r15
140013030:     	popq	%rbp
140013031:     	retq
140013032:     	nopw	%cs:(%rax,%rax)
140013040:     	movq	%rdx, 0x10(%rsp)
140013045:     	pushq	%rbp
140013046:     	pushq	%r15
140013048:     	pushq	%r14
14001304a:     	pushq	%r13
14001304c:     	pushq	%r12
14001304e:     	pushq	%rsi
14001304f:     	pushq	%rdi
140013050:     	pushq	%rbx
140013051:     	subq	$0x138, %rsp            # imm = 0x138
140013058:     	leaq	0x80(%rdx), %rbp
14001305f:     	movdqa	%xmm15, 0x90(%rsp)
140013069:     	movaps	%xmm14, 0xa0(%rsp)
140013072:     	movdqa	%xmm13, 0xb0(%rsp)
14001307c:     	movdqa	%xmm12, 0xc0(%rsp)
140013086:     	movaps	%xmm11, 0xd0(%rsp)
14001308f:     	movaps	%xmm10, 0xe0(%rsp)
140013098:     	movaps	%xmm9, 0xf0(%rsp)
1400130a1:     	movaps	%xmm8, 0x100(%rsp)
1400130aa:     	movaps	%xmm7, 0x110(%rsp)
1400130b2:     	movaps	%xmm6, 0x120(%rsp)
1400130ba:     	movq	0x590(%rbp), %rdx
1400130c1:     	testq	%rdx, %rdx
1400130c4:     	je	0x1400130dc <.text+0x120dc>
1400130c6:     	shlq	$0x4, %rdx
1400130ca:     	movl	$0x10, %r8d
1400130d0:     	movq	0x800(%rbp), %rcx
1400130d7:     	callq	0x140613c20 <.text+0x612c20>
1400130dc:     	movaps	0x120(%rsp), %xmm6
1400130e4:     	movaps	0x110(%rsp), %xmm7
1400130ec:     	movaps	0x100(%rsp), %xmm8
1400130f5:     	movaps	0xf0(%rsp), %xmm9
1400130fe:     	movaps	0xe0(%rsp), %xmm10
140013107:     	movaps	0xd0(%rsp), %xmm11
140013110:     	movdqa	0xc0(%rsp), %xmm12
14001311a:     	movdqa	0xb0(%rsp), %xmm13
140013124:     	movaps	0xa0(%rsp), %xmm14
14001312d:     	movdqa	0x90(%rsp), %xmm15
140013137:     	addq	$0x138, %rsp            # imm = 0x138
14001313e:     	popq	%rbx
14001313f:     	popq	%rdi
140013140:     	popq	%rsi
140013141:     	popq	%r12
140013143:     	popq	%r13
140013145:     	popq	%r14
140013147:     	popq	%r15
140013149:     	popq	%rbp
14001314a:     	retq
14001314b:     	nopl	(%rax,%rax)
140013150:     	movq	%rdx, 0x10(%rsp)
140013155:     	pushq	%rbp
140013156:     	pushq	%r15
140013158:     	pushq	%r14
14001315a:     	pushq	%r13
14001315c:     	pushq	%r12
14001315e:     	pushq	%rsi
14001315f:     	pushq	%rdi
140013160:     	pushq	%rbx
140013161:     	subq	$0x138, %rsp            # imm = 0x138
140013168:     	leaq	0x80(%rdx), %rbp
14001316f:     	movaps	%xmm15, 0x90(%rsp)
140013178:     	movaps	%xmm14, 0xa0(%rsp)
140013181:     	movaps	%xmm13, 0xb0(%rsp)
14001318a:     	movaps	%xmm12, 0xc0(%rsp)
140013193:     	movaps	%xmm11, 0xd0(%rsp)
14001319c:     	movaps	%xmm10, 0xe0(%rsp)
1400131a5:     	movaps	%xmm9, 0xf0(%rsp)
1400131ae:     	movaps	%xmm8, 0x100(%rsp)
1400131b7:     	movaps	%xmm7, 0x110(%rsp)
1400131bf:     	movaps	%xmm6, 0x120(%rsp)
1400131c7:     	testb	$0x1, 0x7d0(%rbp)
1400131ce:     	je	0x140013256 <.text+0x12256>
1400131d4:     	leaq	0x590(%rbp), %rcx
1400131db:     	callq	0x14014ba20 <.text+0x14aa20>
1400131e0:     	movb	$0x1, 0x817(%rbp)
1400131e7:     	movaps	0x120(%rsp), %xmm6
1400131ef:     	movaps	0x110(%rsp), %xmm7
1400131f7:     	movaps	0x100(%rsp), %xmm8
140013200:     	movaps	0xf0(%rsp), %xmm9
140013209:     	movaps	0xe0(%rsp), %xmm10
140013212:     	movaps	0xd0(%rsp), %xmm11
14001321b:     	movdqa	0xc0(%rsp), %xmm12
140013225:     	movdqa	0xb0(%rsp), %xmm13
14001322f:     	movaps	0xa0(%rsp), %xmm14
140013238:     	movdqa	0x90(%rsp), %xmm15
140013242:     	addq	$0x138, %rsp            # imm = 0x138
140013249:     	popq	%rbx
14001324a:     	popq	%rdi
14001324b:     	popq	%rsi
14001324c:     	popq	%r12
14001324e:     	popq	%r13
140013250:     	popq	%r14
140013252:     	popq	%r15
140013254:     	popq	%rbp
140013255:     	retq
140013256:     	movb	$0x1, 0x817(%rbp)
14001325d:     	movaps	0x120(%rsp), %xmm6
140013265:     	movaps	0x110(%rsp), %xmm7
14001326d:     	movaps	0x100(%rsp), %xmm8
140013276:     	movaps	0xf0(%rsp), %xmm9
14001327f:     	movaps	0xe0(%rsp), %xmm10
140013288:     	movaps	0xd0(%rsp), %xmm11
140013291:     	movdqa	0xc0(%rsp), %xmm12
14001329b:     	movdqa	0xb0(%rsp), %xmm13
1400132a5:     	movaps	0xa0(%rsp), %xmm14
1400132ae:     	movdqa	0x90(%rsp), %xmm15
1400132b8:     	addq	$0x138, %rsp            # imm = 0x138
1400132bf:     	popq	%rbx
1400132c0:     	popq	%rdi
1400132c1:     	popq	%rsi
1400132c2:     	popq	%r12
1400132c4:     	popq	%r13
1400132c6:     	popq	%r14
1400132c8:     	popq	%r15
1400132ca:     	popq	%rbp
1400132cb:     	retq
1400132cc:     	nopl	(%rax)
1400132d0:     	movq	%rdx, 0x10(%rsp)
1400132d5:     	pushq	%rbp
1400132d6:     	pushq	%r15
1400132d8:     	pushq	%r14
1400132da:     	pushq	%r13
1400132dc:     	pushq	%r12
1400132de:     	pushq	%rsi
1400132df:     	pushq	%rdi
1400132e0:     	pushq	%rbx
1400132e1:     	subq	$0x138, %rsp            # imm = 0x138
1400132e8:     	leaq	0x80(%rdx), %rbp
1400132ef:     	movaps	%xmm15, 0x90(%rsp)
1400132f8:     	movaps	%xmm14, 0xa0(%rsp)
140013301:     	movaps	%xmm13, 0xb0(%rsp)
14001330a:     	movaps	%xmm12, 0xc0(%rsp)
140013313:     	movaps	%xmm11, 0xd0(%rsp)
14001331c:     	movaps	%xmm10, 0xe0(%rsp)
140013325:     	movaps	%xmm9, 0xf0(%rsp)
14001332e:     	movaps	%xmm8, 0x100(%rsp)
140013337:     	movaps	%xmm7, 0x110(%rsp)
14001333f:     	movaps	%xmm6, 0x120(%rsp)
140013347:     	movzbl	0x811(%rbp), %ebx
14001334e:     	movzbl	0x812(%rbp), %esi
140013355:     	cmpq	$0x0, 0x780(%rbp)
14001335d:     	je	0x14001337c <.text+0x1237c>
14001335f:     	movq	0x780(%rbp), %rdx
140013366:     	shlq	$0x2, %rdx
14001336a:     	movl	$0x4, %r8d
140013370:     	movq	0x800(%rbp), %rcx
140013377:     	callq	0x140613c20 <.text+0x612c20>
14001337c:     	movb	%sil, 0x794(%rbp)
140013383:     	andb	$0x1, %bl
140013386:     	movb	%bl, 0x793(%rbp)
14001338c:     	movaps	0x120(%rsp), %xmm6
140013394:     	movaps	0x110(%rsp), %xmm7
14001339c:     	movaps	0x100(%rsp), %xmm8
1400133a5:     	movaps	0xf0(%rsp), %xmm9
1400133ae:     	movaps	0xe0(%rsp), %xmm10
1400133b7:     	movapd	0xd0(%rsp), %xmm11
1400133c1:     	movaps	0xc0(%rsp), %xmm12
1400133ca:     	movaps	0xb0(%rsp), %xmm13
1400133d3:     	movaps	0xa0(%rsp), %xmm14
1400133dc:     	movaps	0x90(%rsp), %xmm15
1400133e5:     	addq	$0x138, %rsp            # imm = 0x138
1400133ec:     	popq	%rbx
1400133ed:     	popq	%rdi
1400133ee:     	popq	%rsi
1400133ef:     	popq	%r12
1400133f1:     	popq	%r13
1400133f3:     	popq	%r14
1400133f5:     	popq	%r15
1400133f7:     	popq	%rbp
1400133f8:     	retq
1400133f9:     	nopl	(%rax)
140013400:     	movq	%rdx, 0x10(%rsp)
140013405:     	pushq	%rbp
140013406:     	pushq	%r15
140013408:     	pushq	%r14
14001340a:     	pushq	%r13
14001340c:     	pushq	%r12
14001340e:     	pushq	%rsi
14001340f:     	pushq	%rdi
140013410:     	pushq	%rbx
140013411:     	subq	$0x138, %rsp            # imm = 0x138
140013418:     	leaq	0x80(%rdx), %rbp
14001341f:     	movdqa	%xmm15, 0x90(%rsp)
140013429:     	movaps	%xmm14, 0xa0(%rsp)
140013432:     	movdqa	%xmm13, 0xb0(%rsp)
14001343c:     	movdqa	%xmm12, 0xc0(%rsp)
140013446:     	movaps	%xmm11, 0xd0(%rsp)
14001344f:     	movaps	%xmm10, 0xe0(%rsp)
140013458:     	movaps	%xmm9, 0xf0(%rsp)
140013461:     	movaps	%xmm8, 0x100(%rsp)
14001346a:     	movaps	%xmm7, 0x110(%rsp)
140013472:     	movaps	%xmm6, 0x120(%rsp)
14001347a:     	movb	$0x0, 0x817(%rbp)
140013481:     	leaq	0x6b0(%rbp), %rcx
140013488:     	callq	0x140601be0 <.text+0x600be0>
14001348d:     	movb	$0x0, 0x817(%rbp)
140013494:     	movaps	0x120(%rsp), %xmm6
14001349c:     	movaps	0x110(%rsp), %xmm7
1400134a4:     	movaps	0x100(%rsp), %xmm8
1400134ad:     	movaps	0xf0(%rsp), %xmm9
1400134b6:     	movaps	0xe0(%rsp), %xmm10
1400134bf:     	movaps	0xd0(%rsp), %xmm11
1400134c8:     	movdqa	0xc0(%rsp), %xmm12
1400134d2:     	movdqa	0xb0(%rsp), %xmm13
1400134dc:     	movaps	0xa0(%rsp), %xmm14
1400134e5:     	movdqa	0x90(%rsp), %xmm15
1400134ef:     	addq	$0x138, %rsp            # imm = 0x138
1400134f6:     	popq	%rbx
1400134f7:     	popq	%rdi
1400134f8:     	popq	%rsi
1400134f9:     	popq	%r12
1400134fb:     	popq	%r13
1400134fd:     	popq	%r14
1400134ff:     	popq	%r15
140013501:     	popq	%rbp
140013502:     	retq
140013503:     	nopw	%cs:(%rax,%rax)
140013510:     	movq	%rdx, 0x10(%rsp)
140013515:     	pushq	%rbp
140013516:     	pushq	%r15
140013518:     	pushq	%r14
14001351a:     	pushq	%r13
14001351c:     	pushq	%r12
14001351e:     	pushq	%rsi
14001351f:     	pushq	%rdi
140013520:     	pushq	%rbx
140013521:     	subq	$0x138, %rsp            # imm = 0x138
140013528:     	leaq	0x80(%rdx), %rbp
14001352f:     	movdqa	%xmm15, 0x90(%rsp)
140013539:     	movaps	%xmm14, 0xa0(%rsp)
140013542:     	movdqa	%xmm13, 0xb0(%rsp)
14001354c:     	movdqa	%xmm12, 0xc0(%rsp)
140013556:     	movaps	%xmm11, 0xd0(%rsp)
14001355f:     	movaps	%xmm10, 0xe0(%rsp)
140013568:     	movaps	%xmm9, 0xf0(%rsp)
140013571:     	movaps	%xmm8, 0x100(%rsp)
14001357a:     	movaps	%xmm7, 0x110(%rsp)
140013582:     	movaps	%xmm6, 0x120(%rsp)
14001358a:     	movb	$0x0, 0x814(%rbp)
140013591:     	leaq	0x590(%rbp), %rcx
140013598:     	callq	0x140609eb0 <.text+0x608eb0>
14001359d:     	movb	$0x0, 0x814(%rbp)
1400135a4:     	movaps	0x120(%rsp), %xmm6
1400135ac:     	movaps	0x110(%rsp), %xmm7
1400135b4:     	movaps	0x100(%rsp), %xmm8
1400135bd:     	movaps	0xf0(%rsp), %xmm9
1400135c6:     	movaps	0xe0(%rsp), %xmm10
1400135cf:     	movaps	0xd0(%rsp), %xmm11
1400135d8:     	movdqa	0xc0(%rsp), %xmm12
1400135e2:     	movdqa	0xb0(%rsp), %xmm13
1400135ec:     	movaps	0xa0(%rsp), %xmm14
1400135f5:     	movdqa	0x90(%rsp), %xmm15
1400135ff:     	addq	$0x138, %rsp            # imm = 0x138
140013606:     	popq	%rbx
140013607:     	popq	%rdi
140013608:     	popq	%rsi
140013609:     	popq	%r12
14001360b:     	popq	%r13
14001360d:     	popq	%r14
14001360f:     	popq	%r15
140013611:     	popq	%rbp
140013612:     	retq
140013613:     	nopw	%cs:(%rax,%rax)
140013620:     	movq	%rdx, 0x10(%rsp)
140013625:     	pushq	%rbp
140013626:     	pushq	%r15
140013628:     	pushq	%r14
14001362a:     	pushq	%r13
14001362c:     	pushq	%r12
14001362e:     	pushq	%rsi
14001362f:     	pushq	%rdi
140013630:     	pushq	%rbx
140013631:     	subq	$0x138, %rsp            # imm = 0x138
140013638:     	leaq	0x80(%rdx), %rbp
14001363f:     	movdqa	%xmm15, 0x90(%rsp)
140013649:     	movaps	%xmm14, 0xa0(%rsp)
140013652:     	movdqa	%xmm13, 0xb0(%rsp)
14001365c:     	movdqa	%xmm12, 0xc0(%rsp)
140013666:     	movaps	%xmm11, 0xd0(%rsp)
14001366f:     	movaps	%xmm10, 0xe0(%rsp)
140013678:     	movaps	%xmm9, 0xf0(%rsp)
140013681:     	movaps	%xmm8, 0x100(%rsp)
14001368a:     	movaps	%xmm7, 0x110(%rsp)
140013692:     	movaps	%xmm6, 0x120(%rsp)
14001369a:     	movb	$0x0, 0x7de(%rbp)
1400136a1:     	leaq	0x140(%rbp), %rcx
1400136a8:     	callq	0x140609eb0 <.text+0x608eb0>
1400136ad:     	movb	$0x0, 0x7de(%rbp)
1400136b4:     	movaps	0x120(%rsp), %xmm6
1400136bc:     	movaps	0x110(%rsp), %xmm7
1400136c4:     	movaps	0x100(%rsp), %xmm8
1400136cd:     	movaps	0xf0(%rsp), %xmm9
1400136d6:     	movaps	0xe0(%rsp), %xmm10
1400136df:     	movaps	0xd0(%rsp), %xmm11
1400136e8:     	movaps	0xc0(%rsp), %xmm12
1400136f1:     	movaps	0xb0(%rsp), %xmm13
1400136fa:     	movaps	0xa0(%rsp), %xmm14
140013703:     	movaps	0x90(%rsp), %xmm15
14001370c:     	addq	$0x138, %rsp            # imm = 0x138
140013713:     	popq	%rbx
140013714:     	popq	%rdi
140013715:     	popq	%rsi
140013716:     	popq	%r12
140013718:     	popq	%r13
14001371a:     	popq	%r14
14001371c:     	popq	%r15
14001371e:     	popq	%rbp
14001371f:     	retq
140013720:     	movq	%rdx, 0x10(%rsp)
140013725:     	pushq	%rbp
140013726:     	pushq	%r15
140013728:     	pushq	%r14
14001372a:     	pushq	%r13
14001372c:     	pushq	%r12
14001372e:     	pushq	%rsi
14001372f:     	pushq	%rdi
140013730:     	pushq	%rbx
140013731:     	subq	$0x138, %rsp            # imm = 0x138
140013738:     	leaq	0x80(%rdx), %rbp
14001373f:     	movdqa	%xmm15, 0x90(%rsp)
140013749:     	movaps	%xmm14, 0xa0(%rsp)
140013752:     	movdqa	%xmm13, 0xb0(%rsp)
14001375c:     	movdqa	%xmm12, 0xc0(%rsp)
140013766:     	movaps	%xmm11, 0xd0(%rsp)
14001376f:     	movaps	%xmm10, 0xe0(%rsp)
140013778:     	movaps	%xmm9, 0xf0(%rsp)
140013781:     	movaps	%xmm8, 0x100(%rsp)
14001378a:     	movaps	%xmm7, 0x110(%rsp)
140013792:     	movaps	%xmm6, 0x120(%rsp)
14001379a:     	movb	$0x0, 0x7df(%rbp)
1400137a1:     	leaq	0x140(%rbp), %rcx
1400137a8:     	callq	0x140609eb0 <.text+0x608eb0>
1400137ad:     	movb	$0x0, 0x7df(%rbp)
1400137b4:     	movaps	0x120(%rsp), %xmm6
1400137bc:     	movaps	0x110(%rsp), %xmm7
1400137c4:     	movaps	0x100(%rsp), %xmm8
1400137cd:     	movaps	0xf0(%rsp), %xmm9
1400137d6:     	movaps	0xe0(%rsp), %xmm10
1400137df:     	movaps	0xd0(%rsp), %xmm11
1400137e8:     	movaps	0xc0(%rsp), %xmm12
1400137f1:     	movaps	0xb0(%rsp), %xmm13
1400137fa:     	movaps	0xa0(%rsp), %xmm14
140013803:     	movaps	0x90(%rsp), %xmm15
14001380c:     	addq	$0x138, %rsp            # imm = 0x138
140013813:     	popq	%rbx
140013814:     	popq	%rdi
140013815:     	popq	%rsi
140013816:     	popq	%r12
140013818:     	popq	%r13
14001381a:     	popq	%r14
14001381c:     	popq	%r15
14001381e:     	popq	%rbp
14001381f:     	retq
140013820:     	movq	%rdx, 0x10(%rsp)
140013825:     	pushq	%rbp
140013826:     	pushq	%r15
140013828:     	pushq	%r14
14001382a:     	pushq	%r13
14001382c:     	pushq	%r12
14001382e:     	pushq	%rsi
14001382f:     	pushq	%rdi
140013830:     	pushq	%rbx
140013831:     	subq	$0x138, %rsp            # imm = 0x138
140013838:     	leaq	0x80(%rdx), %rbp
14001383f:     	movaps	%xmm15, 0x90(%rsp)
140013848:     	movaps	%xmm14, 0xa0(%rsp)
140013851:     	movaps	%xmm13, 0xb0(%rsp)
14001385a:     	movaps	%xmm12, 0xc0(%rsp)
140013863:     	movapd	%xmm11, 0xd0(%rsp)
14001386d:     	movaps	%xmm10, 0xe0(%rsp)
140013876:     	movaps	%xmm9, 0xf0(%rsp)
14001387f:     	movaps	%xmm8, 0x100(%rsp)
140013888:     	movaps	%xmm7, 0x110(%rsp)
140013890:     	movaps	%xmm6, 0x120(%rsp)
140013898:     	movzbl	0x793(%rbp), %ebx
14001389f:     	movzbl	0x794(%rbp), %esi
1400138a6:     	testb	$0x1, %sil
1400138aa:     	je	0x14001394f <.text+0x1294f>
1400138b0:     	movq	0x3e0(%rbp), %rdx
1400138b7:     	testq	%rdx, %rdx
1400138ba:     	je	0x1400138d2 <.text+0x128d2>
1400138bc:     	movq	0x3e8(%rbp), %rcx
1400138c3:     	shlq	$0x2, %rdx
1400138c7:     	movl	$0x4, %r8d
1400138cd:     	callq	0x140613c20 <.text+0x612c20>
1400138d2:     	movb	%sil, 0x7cc(%rbp)
1400138d9:     	andb	$0x1, %bl
1400138dc:     	movb	%bl, 0x7cb(%rbp)
1400138e2:     	movaps	0x120(%rsp), %xmm6
1400138ea:     	movaps	0x110(%rsp), %xmm7
1400138f2:     	movaps	0x100(%rsp), %xmm8
1400138fb:     	movaps	0xf0(%rsp), %xmm9
140013904:     	movaps	0xe0(%rsp), %xmm10
14001390d:     	movapd	0xd0(%rsp), %xmm11
140013917:     	movaps	0xc0(%rsp), %xmm12
140013920:     	movaps	0xb0(%rsp), %xmm13
140013929:     	movaps	0xa0(%rsp), %xmm14
140013932:     	movaps	0x90(%rsp), %xmm15
14001393b:     	addq	$0x138, %rsp            # imm = 0x138
140013942:     	popq	%rbx
140013943:     	popq	%rdi
140013944:     	popq	%rsi
140013945:     	popq	%r12
140013947:     	popq	%r13
140013949:     	popq	%r14
14001394b:     	popq	%r15
14001394d:     	popq	%rbp
14001394e:     	retq
14001394f:     	movb	%sil, 0x7cc(%rbp)
140013956:     	andb	$0x1, %bl
140013959:     	movb	%bl, 0x7cb(%rbp)
14001395f:     	movaps	0x120(%rsp), %xmm6
140013967:     	movaps	0x110(%rsp), %xmm7
14001396f:     	movaps	0x100(%rsp), %xmm8
140013978:     	movaps	0xf0(%rsp), %xmm9
140013981:     	movaps	0xe0(%rsp), %xmm10
14001398a:     	movapd	0xd0(%rsp), %xmm11
140013994:     	movaps	0xc0(%rsp), %xmm12
14001399d:     	movaps	0xb0(%rsp), %xmm13
1400139a6:     	movaps	0xa0(%rsp), %xmm14
1400139af:     	movaps	0x90(%rsp), %xmm15
1400139b8:     	addq	$0x138, %rsp            # imm = 0x138
1400139bf:     	popq	%rbx
1400139c0:     	popq	%rdi
1400139c1:     	popq	%rsi
1400139c2:     	popq	%r12
1400139c4:     	popq	%r13
1400139c6:     	popq	%r14
1400139c8:     	popq	%r15
1400139ca:     	popq	%rbp
1400139cb:     	retq
1400139cc:     	nopl	(%rax)
1400139d0:     	movq	%rdx, 0x10(%rsp)
1400139d5:     	pushq	%rbp
1400139d6:     	pushq	%r15
1400139d8:     	pushq	%r14
1400139da:     	pushq	%r13
1400139dc:     	pushq	%r12
1400139de:     	pushq	%rsi
1400139df:     	pushq	%rdi
1400139e0:     	pushq	%rbx
1400139e1:     	subq	$0x138, %rsp            # imm = 0x138
1400139e8:     	leaq	0x80(%rdx), %rbp
1400139ef:     	movaps	%xmm15, 0x90(%rsp)
1400139f8:     	movaps	%xmm14, 0xa0(%rsp)
140013a01:     	movaps	%xmm13, 0xb0(%rsp)
140013a0a:     	movaps	%xmm12, 0xc0(%rsp)
140013a13:     	movapd	%xmm11, 0xd0(%rsp)
140013a1d:     	movaps	%xmm10, 0xe0(%rsp)
140013a26:     	movaps	%xmm9, 0xf0(%rsp)
140013a2f:     	movaps	%xmm8, 0x100(%rsp)
140013a38:     	movaps	%xmm7, 0x110(%rsp)
140013a40:     	movaps	%xmm6, 0x120(%rsp)
140013a48:     	movzbl	0x7cb(%rbp), %ebx
140013a4f:     	testb	$0x1, 0x7cc(%rbp)
140013a56:     	je	0x140013af4 <.text+0x12af4>
140013a5c:     	movq	0x500(%rbp), %rdx
140013a63:     	testq	%rdx, %rdx
140013a66:     	je	0x140013a7e <.text+0x12a7e>
140013a68:     	movq	0x508(%rbp), %rcx
140013a6f:     	shlq	$0x3, %rdx
140013a73:     	movl	$0x4, %r8d
140013a79:     	callq	0x140613c20 <.text+0x612c20>
140013a7e:     	andb	$0x1, %bl
140013a81:     	movb	%bl, 0x7cd(%rbp)
140013a87:     	movaps	0x120(%rsp), %xmm6
140013a8f:     	movaps	0x110(%rsp), %xmm7
140013a97:     	movaps	0x100(%rsp), %xmm8
140013aa0:     	movaps	0xf0(%rsp), %xmm9
140013aa9:     	movaps	0xe0(%rsp), %xmm10
140013ab2:     	movapd	0xd0(%rsp), %xmm11
140013abc:     	movaps	0xc0(%rsp), %xmm12
140013ac5:     	movaps	0xb0(%rsp), %xmm13
140013ace:     	movaps	0xa0(%rsp), %xmm14
140013ad7:     	movaps	0x90(%rsp), %xmm15
140013ae0:     	addq	$0x138, %rsp            # imm = 0x138
140013ae7:     	popq	%rbx
140013ae8:     	popq	%rdi
140013ae9:     	popq	%rsi
140013aea:     	popq	%r12
140013aec:     	popq	%r13
140013aee:     	popq	%r14
140013af0:     	popq	%r15
140013af2:     	popq	%rbp
140013af3:     	retq
140013af4:     	andb	$0x1, %bl
140013af7:     	movb	%bl, 0x7cd(%rbp)
140013afd:     	movaps	0x120(%rsp), %xmm6
140013b05:     	movaps	0x110(%rsp), %xmm7
140013b0d:     	movaps	0x100(%rsp), %xmm8
140013b16:     	movaps	0xf0(%rsp), %xmm9
140013b1f:     	movaps	0xe0(%rsp), %xmm10
140013b28:     	movapd	0xd0(%rsp), %xmm11
140013b32:     	movaps	0xc0(%rsp), %xmm12
140013b3b:     	movaps	0xb0(%rsp), %xmm13
140013b44:     	movaps	0xa0(%rsp), %xmm14
140013b4d:     	movaps	0x90(%rsp), %xmm15
140013b56:     	addq	$0x138, %rsp            # imm = 0x138
140013b5d:     	popq	%rbx
140013b5e:     	popq	%rdi
140013b5f:     	popq	%rsi
140013b60:     	popq	%r12
140013b62:     	popq	%r13
140013b64:     	popq	%r14
140013b66:     	popq	%r15
140013b68:     	popq	%rbp
140013b69:     	retq
140013b6a:     	nopw	(%rax,%rax)
140013b70:     	movq	%rdx, 0x10(%rsp)
140013b75:     	pushq	%rbp
140013b76:     	pushq	%r15
140013b78:     	pushq	%r14
140013b7a:     	pushq	%r13
140013b7c:     	pushq	%r12
140013b7e:     	pushq	%rsi
140013b7f:     	pushq	%rdi
140013b80:     	pushq	%rbx
140013b81:     	subq	$0x138, %rsp            # imm = 0x138
140013b88:     	leaq	0x80(%rdx), %rbp
140013b8f:     	movdqa	%xmm15, 0x90(%rsp)
140013b99:     	movaps	%xmm14, 0xa0(%rsp)
140013ba2:     	movdqa	%xmm13, 0xb0(%rsp)
140013bac:     	movdqa	%xmm12, 0xc0(%rsp)
140013bb6:     	movaps	%xmm11, 0xd0(%rsp)
140013bbf:     	movaps	%xmm10, 0xe0(%rsp)
140013bc8:     	movaps	%xmm9, 0xf0(%rsp)
140013bd1:     	movaps	%xmm8, 0x100(%rsp)
140013bda:     	movaps	%xmm7, 0x110(%rsp)
140013be2:     	movaps	%xmm6, 0x120(%rsp)
140013bea:     	movq	0x1b8(%rbp), %rcx
140013bf1:     	callq	0x1405df260 <.text+0x5de260>
140013bf6:     	movaps	0x120(%rsp), %xmm6
140013bfe:     	movaps	0x110(%rsp), %xmm7
140013c06:     	movaps	0x100(%rsp), %xmm8
140013c0f:     	movaps	0xf0(%rsp), %xmm9
140013c18:     	movaps	0xe0(%rsp), %xmm10
140013c21:     	movaps	0xd0(%rsp), %xmm11
140013c2a:     	movaps	0xc0(%rsp), %xmm12
140013c33:     	movaps	0xb0(%rsp), %xmm13
140013c3c:     	movaps	0xa0(%rsp), %xmm14
140013c45:     	movaps	0x90(%rsp), %xmm15
140013c4e:     	addq	$0x138, %rsp            # imm = 0x138
140013c55:     	popq	%rbx
140013c56:     	popq	%rdi
140013c57:     	popq	%rsi
140013c58:     	popq	%r12
140013c5a:     	popq	%r13
140013c5c:     	popq	%r14
140013c5e:     	popq	%r15
140013c60:     	popq	%rbp
140013c61:     	retq
140013c62:     	nopw	%cs:(%rax,%rax)
140013c70:     	movq	%rdx, 0x10(%rsp)
140013c75:     	pushq	%rbp
140013c76:     	pushq	%r15
140013c78:     	pushq	%r14
140013c7a:     	pushq	%r13
140013c7c:     	pushq	%r12
140013c7e:     	pushq	%rsi
140013c7f:     	pushq	%rdi
140013c80:     	pushq	%rbx
140013c81:     	subq	$0x138, %rsp            # imm = 0x138
140013c88:     	leaq	0x80(%rdx), %rbp
140013c8f:     	movdqa	%xmm15, 0x90(%rsp)
140013c99:     	movaps	%xmm14, 0xa0(%rsp)
140013ca2:     	movdqa	%xmm13, 0xb0(%rsp)
140013cac:     	movdqa	%xmm12, 0xc0(%rsp)
140013cb6:     	movaps	%xmm11, 0xd0(%rsp)
140013cbf:     	movaps	%xmm10, 0xe0(%rsp)
140013cc8:     	movaps	%xmm9, 0xf0(%rsp)
140013cd1:     	movaps	%xmm8, 0x100(%rsp)
140013cda:     	movaps	%xmm7, 0x110(%rsp)
140013ce2:     	movaps	%xmm6, 0x120(%rsp)
140013cea:     	movb	$0x0, 0x814(%rbp)
140013cf1:     	leaq	0x590(%rbp), %rcx
140013cf8:     	callq	0x140607a30 <.text+0x606a30>
140013cfd:     	movb	$0x0, 0x814(%rbp)
140013d04:     	movaps	0x120(%rsp), %xmm6
140013d0c:     	movaps	0x110(%rsp), %xmm7
140013d14:     	movaps	0x100(%rsp), %xmm8
140013d1d:     	movaps	0xf0(%rsp), %xmm9
140013d26:     	movaps	0xe0(%rsp), %xmm10
140013d2f:     	movaps	0xd0(%rsp), %xmm11
140013d38:     	movdqa	0xc0(%rsp), %xmm12
140013d42:     	movdqa	0xb0(%rsp), %xmm13
140013d4c:     	movaps	0xa0(%rsp), %xmm14
140013d55:     	movdqa	0x90(%rsp), %xmm15
140013d5f:     	addq	$0x138, %rsp            # imm = 0x138
140013d66:     	popq	%rbx
140013d67:     	popq	%rdi
140013d68:     	popq	%rsi
140013d69:     	popq	%r12
140013d6b:     	popq	%r13
140013d6d:     	popq	%r14
140013d6f:     	popq	%r15
140013d71:     	popq	%rbp
140013d72:     	retq
140013d73:     	nopw	%cs:(%rax,%rax)
140013d80:     	movq	%rdx, 0x10(%rsp)
140013d85:     	pushq	%rbp
140013d86:     	pushq	%r15
140013d88:     	pushq	%r14
140013d8a:     	pushq	%r13
140013d8c:     	pushq	%r12
140013d8e:     	pushq	%rsi
140013d8f:     	pushq	%rdi
140013d90:     	pushq	%rbx
140013d91:     	subq	$0x138, %rsp            # imm = 0x138
140013d98:     	leaq	0x80(%rdx), %rbp
140013d9f:     	movaps	%xmm15, 0x90(%rsp)
140013da8:     	movaps	%xmm14, 0xa0(%rsp)
140013db1:     	movaps	%xmm13, 0xb0(%rsp)
140013dba:     	movaps	%xmm12, 0xc0(%rsp)
140013dc3:     	movaps	%xmm11, 0xd0(%rsp)
140013dcc:     	movaps	%xmm10, 0xe0(%rsp)
140013dd5:     	movaps	%xmm9, 0xf0(%rsp)
140013dde:     	movaps	%xmm8, 0x100(%rsp)
140013de7:     	movaps	%xmm7, 0x110(%rsp)
140013def:     	movaps	%xmm6, 0x120(%rsp)
140013df7:     	cmpb	$0x0, 0x7cd(%rbp)
140013dfe:     	je	0x140013e86 <.text+0x12e86>
140013e04:     	leaq	0x460(%rbp), %rcx
140013e0b:     	callq	0x14014ba20 <.text+0x14aa20>
140013e10:     	movb	$0x1, 0x817(%rbp)
140013e17:     	movaps	0x120(%rsp), %xmm6
140013e1f:     	movaps	0x110(%rsp), %xmm7
140013e27:     	movaps	0x100(%rsp), %xmm8
140013e30:     	movaps	0xf0(%rsp), %xmm9
140013e39:     	movaps	0xe0(%rsp), %xmm10
140013e42:     	movaps	0xd0(%rsp), %xmm11
140013e4b:     	movdqa	0xc0(%rsp), %xmm12
140013e55:     	movdqa	0xb0(%rsp), %xmm13
140013e5f:     	movaps	0xa0(%rsp), %xmm14
140013e68:     	movdqa	0x90(%rsp), %xmm15
140013e72:     	addq	$0x138, %rsp            # imm = 0x138
140013e79:     	popq	%rbx
140013e7a:     	popq	%rdi
140013e7b:     	popq	%rsi
140013e7c:     	popq	%r12
140013e7e:     	popq	%r13
140013e80:     	popq	%r14
140013e82:     	popq	%r15
140013e84:     	popq	%rbp
140013e85:     	retq
140013e86:     	movb	$0x1, 0x817(%rbp)
140013e8d:     	movaps	0x120(%rsp), %xmm6
140013e95:     	movaps	0x110(%rsp), %xmm7
140013e9d:     	movaps	0x100(%rsp), %xmm8
140013ea6:     	movaps	0xf0(%rsp), %xmm9
140013eaf:     	movaps	0xe0(%rsp), %xmm10
140013eb8:     	movaps	0xd0(%rsp), %xmm11
140013ec1:     	movdqa	0xc0(%rsp), %xmm12
140013ecb:     	movdqa	0xb0(%rsp), %xmm13
140013ed5:     	movaps	0xa0(%rsp), %xmm14
140013ede:     	movdqa	0x90(%rsp), %xmm15
140013ee8:     	addq	$0x138, %rsp            # imm = 0x138
140013eef:     	popq	%rbx
140013ef0:     	popq	%rdi
140013ef1:     	popq	%rsi
140013ef2:     	popq	%r12
140013ef4:     	popq	%r13
140013ef6:     	popq	%r14
140013ef8:     	popq	%r15
140013efa:     	popq	%rbp
140013efb:     	retq
140013efc:     	nopl	(%rax)
140013f00:     	movq	%rdx, 0x10(%rsp)
140013f05:     	pushq	%rbp
140013f06:     	pushq	%r15
140013f08:     	pushq	%r14
140013f0a:     	pushq	%r13
140013f0c:     	pushq	%r12
140013f0e:     	pushq	%rsi
140013f0f:     	pushq	%rdi
140013f10:     	pushq	%rbx
140013f11:     	subq	$0x138, %rsp            # imm = 0x138
140013f18:     	leaq	0x80(%rdx), %rbp
140013f1f:     	movdqa	%xmm15, 0x90(%rsp)
140013f29:     	movaps	%xmm14, 0xa0(%rsp)
140013f32:     	movdqa	%xmm13, 0xb0(%rsp)
140013f3c:     	movdqa	%xmm12, 0xc0(%rsp)
140013f46:     	movaps	%xmm11, 0xd0(%rsp)
140013f4f:     	movaps	%xmm10, 0xe0(%rsp)
140013f58:     	movaps	%xmm9, 0xf0(%rsp)
140013f61:     	movaps	%xmm8, 0x100(%rsp)
140013f6a:     	movaps	%xmm7, 0x110(%rsp)
140013f72:     	movaps	%xmm6, 0x120(%rsp)
140013f7a:     	cmpq	$0x0, 0x800(%rbp)
140013f82:     	je	0x140013fa5 <.text+0x12fa5>
140013f84:     	movq	0x800(%rbp), %rax
140013f8b:     	leaq	(,%rax,4), %rdx
140013f93:     	movl	$0x4, %r8d
140013f99:     	movq	0x760(%rbp), %rcx
140013fa0:     	callq	0x140613c20 <.text+0x612c20>
140013fa5:     	cmpq	$0x0, 0x7f0(%rbp)
140013fad:     	je	0x140013fcc <.text+0x12fcc>
140013faf:     	movq	0x7f0(%rbp), %rdx
140013fb6:     	shlq	$0x2, %rdx
140013fba:     	movl	$0x4, %r8d
140013fc0:     	movq	0x778(%rbp), %rcx
140013fc7:     	callq	0x140613c20 <.text+0x612c20>
140013fcc:     	movb	$0x0, 0x795(%rbp)
140013fd3:     	movaps	0x120(%rsp), %xmm6
140013fdb:     	movaps	0x110(%rsp), %xmm7
140013fe3:     	movaps	0x100(%rsp), %xmm8
140013fec:     	movaps	0xf0(%rsp), %xmm9
140013ff5:     	movaps	0xe0(%rsp), %xmm10
140013ffe:     	movaps	0xd0(%rsp), %xmm11
140014007:     	movdqa	0xc0(%rsp), %xmm12
140014011:     	movdqa	0xb0(%rsp), %xmm13
14001401b:     	movaps	0xa0(%rsp), %xmm14
140014024:     	movdqa	0x90(%rsp), %xmm15
14001402e:     	addq	$0x138, %rsp            # imm = 0x138
140014035:     	popq	%rbx
140014036:     	popq	%rdi
140014037:     	popq	%rsi
140014038:     	popq	%r12
14001403a:     	popq	%r13
14001403c:     	popq	%r14
14001403e:     	popq	%r15
140014040:     	popq	%rbp
140014041:     	retq
140014042:     	nopw	%cs:(%rax,%rax)
140014050:     	movq	%rdx, 0x10(%rsp)
140014055:     	pushq	%rbp
140014056:     	pushq	%r15
140014058:     	pushq	%r14
14001405a:     	pushq	%r13
14001405c:     	pushq	%r12
14001405e:     	pushq	%rsi
14001405f:     	pushq	%rdi
140014060:     	pushq	%rbx
140014061:     	subq	$0x138, %rsp            # imm = 0x138
140014068:     	leaq	0x80(%rdx), %rbp
14001406f:     	movdqa	%xmm15, 0x90(%rsp)
140014079:     	movaps	%xmm14, 0xa0(%rsp)
140014082:     	movdqa	%xmm13, 0xb0(%rsp)
14001408c:     	movdqa	%xmm12, 0xc0(%rsp)
140014096:     	movaps	%xmm11, 0xd0(%rsp)
14001409f:     	movaps	%xmm10, 0xe0(%rsp)
1400140a8:     	movaps	%xmm9, 0xf0(%rsp)
1400140b1:     	movaps	%xmm8, 0x100(%rsp)
1400140ba:     	movaps	%xmm7, 0x110(%rsp)
1400140c2:     	movaps	%xmm6, 0x120(%rsp)
1400140ca:     	cmpq	$0x0, 0x800(%rbp)
1400140d2:     	je	0x1400140f5 <.text+0x130f5>
1400140d4:     	movq	0x800(%rbp), %rax
1400140db:     	leaq	(,%rax,4), %rdx
1400140e3:     	movl	$0x4, %r8d
1400140e9:     	movq	0x760(%rbp), %rcx
1400140f0:     	callq	0x140613c20 <.text+0x612c20>
1400140f5:     	cmpq	$0x0, 0x7f0(%rbp)
1400140fd:     	je	0x14001411c <.text+0x1311c>
1400140ff:     	movq	0x7f0(%rbp), %rdx
140014106:     	shlq	$0x2, %rdx
14001410a:     	movl	$0x4, %r8d
140014110:     	movq	0x778(%rbp), %rcx
140014117:     	callq	0x140613c20 <.text+0x612c20>
14001411c:     	movb	$0x0, 0x796(%rbp)
140014123:     	movaps	0x120(%rsp), %xmm6
14001412b:     	movaps	0x110(%rsp), %xmm7
140014133:     	movaps	0x100(%rsp), %xmm8
14001413c:     	movaps	0xf0(%rsp), %xmm9
140014145:     	movaps	0xe0(%rsp), %xmm10
14001414e:     	movaps	0xd0(%rsp), %xmm11
140014157:     	movdqa	0xc0(%rsp), %xmm12
140014161:     	movdqa	0xb0(%rsp), %xmm13
14001416b:     	movaps	0xa0(%rsp), %xmm14
140014174:     	movdqa	0x90(%rsp), %xmm15
14001417e:     	addq	$0x138, %rsp            # imm = 0x138
140014185:     	popq	%rbx
140014186:     	popq	%rdi
140014187:     	popq	%rsi
140014188:     	popq	%r12
14001418a:     	popq	%r13
14001418c:     	popq	%r14
14001418e:     	popq	%r15
140014190:     	popq	%rbp
140014191:     	retq
140014192:     	nopw	%cs:(%rax,%rax)
1400141a0:     	movq	%rdx, 0x10(%rsp)
1400141a5:     	pushq	%rbp
1400141a6:     	pushq	%r15
1400141a8:     	pushq	%r14
1400141aa:     	pushq	%r13
1400141ac:     	pushq	%r12
1400141ae:     	pushq	%rsi
1400141af:     	pushq	%rdi
1400141b0:     	pushq	%rbx
1400141b1:     	subq	$0x138, %rsp            # imm = 0x138
1400141b8:     	leaq	0x80(%rdx), %rbp
1400141bf:     	movdqa	%xmm15, 0x90(%rsp)
1400141c9:     	movaps	%xmm14, 0xa0(%rsp)
1400141d2:     	movdqa	%xmm13, 0xb0(%rsp)
1400141dc:     	movdqa	%xmm12, 0xc0(%rsp)
1400141e6:     	movaps	%xmm11, 0xd0(%rsp)
1400141ef:     	movaps	%xmm10, 0xe0(%rsp)
1400141f8:     	movaps	%xmm9, 0xf0(%rsp)
140014201:     	movaps	%xmm8, 0x100(%rsp)
14001420a:     	movaps	%xmm7, 0x110(%rsp)
140014212:     	movaps	%xmm6, 0x120(%rsp)
14001421a:     	movzbl	0x795(%rbp), %ebx
140014221:     	cmpq	$0x0, 0x780(%rbp)
140014229:     	je	0x140014248 <.text+0x13248>
14001422b:     	movq	0x780(%rbp), %rdx
140014232:     	shlq	$0x3, %rdx
140014236:     	movl	$0x4, %r8d
14001423c:     	movq	0x7b0(%rbp), %rcx
140014243:     	callq	0x140613c20 <.text+0x612c20>
140014248:     	leaq	0x590(%rbp), %rcx
14001424f:     	callq	0x14014ba20 <.text+0x14aa20>
140014254:     	andb	$0x1, %bl
140014257:     	movb	%bl, 0x7de(%rbp)
14001425d:     	movaps	0x120(%rsp), %xmm6
140014265:     	movaps	0x110(%rsp), %xmm7
14001426d:     	movaps	0x100(%rsp), %xmm8
140014276:     	movaps	0xf0(%rsp), %xmm9
14001427f:     	movaps	0xe0(%rsp), %xmm10
140014288:     	movaps	0xd0(%rsp), %xmm11
140014291:     	movaps	0xc0(%rsp), %xmm12
14001429a:     	movaps	0xb0(%rsp), %xmm13
1400142a3:     	movaps	0xa0(%rsp), %xmm14
1400142ac:     	movaps	0x90(%rsp), %xmm15
1400142b5:     	addq	$0x138, %rsp            # imm = 0x138
1400142bc:     	popq	%rbx
1400142bd:     	popq	%rdi
1400142be:     	popq	%rsi
1400142bf:     	popq	%r12
1400142c1:     	popq	%r13
1400142c3:     	popq	%r14
1400142c5:     	popq	%r15
1400142c7:     	popq	%rbp
1400142c8:     	retq
1400142c9:     	nopl	(%rax)
1400142d0:     	movq	%rdx, 0x10(%rsp)
1400142d5:     	pushq	%rbp
1400142d6:     	pushq	%r15
1400142d8:     	pushq	%r14
1400142da:     	pushq	%r13
1400142dc:     	pushq	%r12
1400142de:     	pushq	%rsi
1400142df:     	pushq	%rdi
1400142e0:     	pushq	%rbx
1400142e1:     	subq	$0x138, %rsp            # imm = 0x138
1400142e8:     	leaq	0x80(%rdx), %rbp
1400142ef:     	movdqa	%xmm15, 0x90(%rsp)
1400142f9:     	movaps	%xmm14, 0xa0(%rsp)
140014302:     	movdqa	%xmm13, 0xb0(%rsp)
14001430c:     	movdqa	%xmm12, 0xc0(%rsp)
140014316:     	movaps	%xmm11, 0xd0(%rsp)
14001431f:     	movaps	%xmm10, 0xe0(%rsp)
140014328:     	movaps	%xmm9, 0xf0(%rsp)
140014331:     	movaps	%xmm8, 0x100(%rsp)
14001433a:     	movaps	%xmm7, 0x110(%rsp)
140014342:     	movaps	%xmm6, 0x120(%rsp)
14001434a:     	movzbl	0x796(%rbp), %ebx
140014351:     	cmpq	$0x0, 0x780(%rbp)
140014359:     	je	0x140014378 <.text+0x13378>
14001435b:     	movq	0x780(%rbp), %rdx
140014362:     	shlq	$0x3, %rdx
140014366:     	movl	$0x4, %r8d
14001436c:     	movq	0x7b0(%rbp), %rcx
140014373:     	callq	0x140613c20 <.text+0x612c20>
140014378:     	leaq	0x590(%rbp), %rcx
14001437f:     	callq	0x14014ba20 <.text+0x14aa20>
140014384:     	andb	$0x1, %bl
140014387:     	movb	%bl, 0x7df(%rbp)
14001438d:     	movaps	0x120(%rsp), %xmm6
140014395:     	movaps	0x110(%rsp), %xmm7
14001439d:     	movaps	0x100(%rsp), %xmm8
1400143a6:     	movaps	0xf0(%rsp), %xmm9
1400143af:     	movaps	0xe0(%rsp), %xmm10
1400143b8:     	movaps	0xd0(%rsp), %xmm11
1400143c1:     	movaps	0xc0(%rsp), %xmm12
1400143ca:     	movaps	0xb0(%rsp), %xmm13
1400143d3:     	movaps	0xa0(%rsp), %xmm14
1400143dc:     	movaps	0x90(%rsp), %xmm15
1400143e5:     	addq	$0x138, %rsp            # imm = 0x138
1400143ec:     	popq	%rbx
1400143ed:     	popq	%rdi
1400143ee:     	popq	%rsi
1400143ef:     	popq	%r12
1400143f1:     	popq	%r13
1400143f3:     	popq	%r14
1400143f5:     	popq	%r15
1400143f7:     	popq	%rbp
1400143f8:     	retq
1400143f9:     	nopl	(%rax)
140014400:     	movq	%rdx, 0x10(%rsp)
140014405:     	pushq	%rbp
140014406:     	pushq	%r15
140014408:     	pushq	%r14
14001440a:     	pushq	%r13
14001440c:     	pushq	%r12
14001440e:     	pushq	%rsi
14001440f:     	pushq	%rdi
140014410:     	pushq	%rbx
140014411:     	subq	$0x138, %rsp            # imm = 0x138
140014418:     	leaq	0x80(%rdx), %rbp
14001441f:     	movaps	%xmm15, 0x90(%rsp)
140014428:     	movaps	%xmm14, 0xa0(%rsp)
140014431:     	movaps	%xmm13, 0xb0(%rsp)
14001443a:     	movaps	%xmm12, 0xc0(%rsp)
140014443:     	movaps	%xmm11, 0xd0(%rsp)
14001444c:     	movaps	%xmm10, 0xe0(%rsp)
140014455:     	movaps	%xmm9, 0xf0(%rsp)
14001445e:     	movaps	%xmm8, 0x100(%rsp)
140014467:     	movaps	%xmm7, 0x110(%rsp)
14001446f:     	movaps	%xmm6, 0x120(%rsp)
140014477:     	cmpq	$0x0, 0x800(%rbp)
14001447f:     	setne	%al
140014482:     	testb	%al, 0x7de(%rbp)
140014488:     	je	0x1400144a7 <.text+0x134a7>
14001448a:     	movq	0x800(%rbp), %rdx
140014491:     	shlq	$0x2, %rdx
140014495:     	movl	$0x4, %r8d
14001449b:     	movq	0x760(%rbp), %rcx
1400144a2:     	callq	0x140613c20 <.text+0x612c20>
1400144a7:     	movaps	0x120(%rsp), %xmm6
1400144af:     	movaps	0x110(%rsp), %xmm7
1400144b7:     	movaps	0x100(%rsp), %xmm8
1400144c0:     	movaps	0xf0(%rsp), %xmm9
1400144c9:     	movaps	0xe0(%rsp), %xmm10
1400144d2:     	movaps	0xd0(%rsp), %xmm11
1400144db:     	movaps	0xc0(%rsp), %xmm12
1400144e4:     	movaps	0xb0(%rsp), %xmm13
1400144ed:     	movaps	0xa0(%rsp), %xmm14
1400144f6:     	movaps	0x90(%rsp), %xmm15
1400144ff:     	addq	$0x138, %rsp            # imm = 0x138
140014506:     	popq	%rbx
140014507:     	popq	%rdi
140014508:     	popq	%rsi
140014509:     	popq	%r12
14001450b:     	popq	%r13
14001450d:     	popq	%r14
14001450f:     	popq	%r15
140014511:     	popq	%rbp
140014512:     	retq
140014513:     	nopw	%cs:(%rax,%rax)
140014520:     	movq	%rdx, 0x10(%rsp)
140014525:     	pushq	%rbp
140014526:     	pushq	%r15
140014528:     	pushq	%r14
14001452a:     	pushq	%r13
14001452c:     	pushq	%r12
14001452e:     	pushq	%rsi
14001452f:     	pushq	%rdi
140014530:     	pushq	%rbx
140014531:     	subq	$0x138, %rsp            # imm = 0x138
140014538:     	leaq	0x80(%rdx), %rbp
14001453f:     	movaps	%xmm15, 0x90(%rsp)
140014548:     	movaps	%xmm14, 0xa0(%rsp)
140014551:     	movaps	%xmm13, 0xb0(%rsp)
14001455a:     	movaps	%xmm12, 0xc0(%rsp)
140014563:     	movaps	%xmm11, 0xd0(%rsp)
14001456c:     	movaps	%xmm10, 0xe0(%rsp)
140014575:     	movaps	%xmm9, 0xf0(%rsp)
14001457e:     	movaps	%xmm8, 0x100(%rsp)
140014587:     	movaps	%xmm7, 0x110(%rsp)
14001458f:     	movaps	%xmm6, 0x120(%rsp)
140014597:     	cmpq	$0x0, 0x800(%rbp)
14001459f:     	setne	%al
1400145a2:     	testb	%al, 0x7df(%rbp)
1400145a8:     	je	0x1400145c7 <.text+0x135c7>
1400145aa:     	movq	0x800(%rbp), %rdx
1400145b1:     	shlq	$0x2, %rdx
1400145b5:     	movl	$0x4, %r8d
1400145bb:     	movq	0x760(%rbp), %rcx
1400145c2:     	callq	0x140613c20 <.text+0x612c20>
1400145c7:     	movaps	0x120(%rsp), %xmm6
1400145cf:     	movaps	0x110(%rsp), %xmm7
1400145d7:     	movaps	0x100(%rsp), %xmm8
1400145e0:     	movaps	0xf0(%rsp), %xmm9
1400145e9:     	movaps	0xe0(%rsp), %xmm10
1400145f2:     	movaps	0xd0(%rsp), %xmm11
1400145fb:     	movaps	0xc0(%rsp), %xmm12
140014604:     	movaps	0xb0(%rsp), %xmm13
14001460d:     	movaps	0xa0(%rsp), %xmm14
140014616:     	movaps	0x90(%rsp), %xmm15
14001461f:     	addq	$0x138, %rsp            # imm = 0x138
140014626:     	popq	%rbx
140014627:     	popq	%rdi
140014628:     	popq	%rsi
140014629:     	popq	%r12
14001462b:     	popq	%r13
14001462d:     	popq	%r14
14001462f:     	popq	%r15
140014631:     	popq	%rbp
140014632:     	retq
140014633:     	nopw	%cs:(%rax,%rax)
140014640:     	movq	%rdx, 0x10(%rsp)
140014645:     	pushq	%rbp
140014646:     	pushq	%r15
140014648:     	pushq	%r14
14001464a:     	pushq	%r13
14001464c:     	pushq	%r12
14001464e:     	pushq	%rsi
14001464f:     	pushq	%rdi
140014650:     	pushq	%rbx
140014651:     	subq	$0x138, %rsp            # imm = 0x138
140014658:     	leaq	0x80(%rdx), %rbp
14001465f:     	movdqa	%xmm15, 0x90(%rsp)
140014669:     	movaps	%xmm14, 0xa0(%rsp)
140014672:     	movdqa	%xmm13, 0xb0(%rsp)
14001467c:     	movdqa	%xmm12, 0xc0(%rsp)
140014686:     	movaps	%xmm11, 0xd0(%rsp)
14001468f:     	movaps	%xmm10, 0xe0(%rsp)
140014698:     	movaps	%xmm9, 0xf0(%rsp)
1400146a1:     	movaps	%xmm8, 0x100(%rsp)
1400146aa:     	movaps	%xmm7, 0x110(%rsp)
1400146b2:     	movaps	%xmm6, 0x120(%rsp)
1400146ba:     	cmpq	$0x0, 0x800(%rbp)
1400146c2:     	je	0x1400146e5 <.text+0x136e5>
1400146c4:     	movq	0x800(%rbp), %rax
1400146cb:     	shlq	$0x2, %rax
1400146cf:     	leaq	(%rax,%rax,2), %rdx
1400146d3:     	movl	$0x4, %r8d
1400146d9:     	movq	0x780(%rbp), %rcx
1400146e0:     	callq	0x140613c20 <.text+0x612c20>
1400146e5:     	movaps	0x120(%rsp), %xmm6
1400146ed:     	movaps	0x110(%rsp), %xmm7
1400146f5:     	movaps	0x100(%rsp), %xmm8
1400146fe:     	movaps	0xf0(%rsp), %xmm9
140014707:     	movaps	0xe0(%rsp), %xmm10
140014710:     	movaps	0xd0(%rsp), %xmm11
140014719:     	movdqa	0xc0(%rsp), %xmm12
140014723:     	movdqa	0xb0(%rsp), %xmm13
14001472d:     	movaps	0xa0(%rsp), %xmm14
140014736:     	movdqa	0x90(%rsp), %xmm15
140014740:     	addq	$0x138, %rsp            # imm = 0x138
140014747:     	popq	%rbx
140014748:     	popq	%rdi
140014749:     	popq	%rsi
14001474a:     	popq	%r12
14001474c:     	popq	%r13
14001474e:     	popq	%r14
140014750:     	popq	%r15
140014752:     	popq	%rbp
140014753:     	retq
140014754:     	nopw	%cs:(%rax,%rax)
140014760:     	movq	%rdx, 0x10(%rsp)
140014765:     	pushq	%rbp
140014766:     	pushq	%r15
140014768:     	pushq	%r14
14001476a:     	pushq	%r13
14001476c:     	pushq	%r12
14001476e:     	pushq	%rsi
14001476f:     	pushq	%rdi
140014770:     	pushq	%rbx
140014771:     	subq	$0x138, %rsp            # imm = 0x138
140014778:     	leaq	0x80(%rdx), %rbp
14001477f:     	movdqa	%xmm15, 0x90(%rsp)
140014789:     	movaps	%xmm14, 0xa0(%rsp)
140014792:     	movdqa	%xmm13, 0xb0(%rsp)
14001479c:     	movdqa	%xmm12, 0xc0(%rsp)
1400147a6:     	movaps	%xmm11, 0xd0(%rsp)
1400147af:     	movaps	%xmm10, 0xe0(%rsp)
1400147b8:     	movaps	%xmm9, 0xf0(%rsp)
1400147c1:     	movaps	%xmm8, 0x100(%rsp)
1400147ca:     	movaps	%xmm7, 0x110(%rsp)
1400147d2:     	movaps	%xmm6, 0x120(%rsp)
1400147da:     	movaps	0x4d0(%rbp), %xmm0
1400147e1:     	movaps	%xmm0, 0x710(%rbp)
1400147e8:     	movaps	0x4c0(%rbp), %xmm0
1400147ef:     	movaps	%xmm0, 0x700(%rbp)
1400147f6:     	movaps	0x4b0(%rbp), %xmm0
1400147fd:     	movaps	%xmm0, 0x6f0(%rbp)
140014804:     	movaps	0x4a0(%rbp), %xmm0
14001480b:     	movaps	%xmm0, 0x6e0(%rbp)
140014812:     	movaps	0x460(%rbp), %xmm0
140014819:     	movdqa	0x470(%rbp), %xmm1
140014821:     	movdqa	0x480(%rbp), %xmm2
140014829:     	movapd	0x490(%rbp), %xmm3
140014831:     	movapd	%xmm3, 0x6d0(%rbp)
140014839:     	movdqa	%xmm2, 0x6c0(%rbp)
140014841:     	movdqa	%xmm1, 0x6b0(%rbp)
140014849:     	movaps	%xmm0, 0x6a0(%rbp)
140014850:     	movq	0x7b8(%rbp), %rax
140014857:     	movq	%rax, 0x720(%rbp)
14001485e:     	movl	0x588(%rbp), %eax
140014864:     	movl	%eax, 0x728(%rbp)
14001486a:     	movl	0x560(%rbp), %eax
140014870:     	movl	%eax, 0x72c(%rbp)
140014876:     	movl	0x580(%rbp), %eax
14001487c:     	movl	%eax, 0x730(%rbp)
140014882:     	movdqa	0x290(%rbp), %xmm0
14001488a:     	movd	%xmm0, %eax
14001488e:     	movw	%ax, 0x734(%rbp)
140014895:     	movw	$0x0, 0x736(%rbp)
14001489e:     	movzwl	0x430(%rbp), %eax
1400148a5:     	movw	%ax, 0x738(%rbp)
1400148ac:     	movw	$0x300, 0x73a(%rbp)     # imm = 0x300
1400148b5:     	movq	0x330(%rbp), %rax
1400148bc:     	movq	%rax, 0x740(%rbp)
1400148c3:     	movq	0x530(%rbp), %rax
1400148ca:     	movq	%rax, 0x748(%rbp)
1400148d1:     	movb	$0x0, 0x750(%rbp)
1400148d8:     	movb	$0x0, 0x7d2(%rbp)
1400148df:     	movb	$0x0, 0x7d1(%rbp)
1400148e6:     	movaps	0x120(%rsp), %xmm6
1400148ee:     	movaps	0x110(%rsp), %xmm7
1400148f6:     	movaps	0x100(%rsp), %xmm8
1400148ff:     	movaps	0xf0(%rsp), %xmm9
140014908:     	movaps	0xe0(%rsp), %xmm10
140014911:     	movaps	0xd0(%rsp), %xmm11
14001491a:     	movdqa	0xc0(%rsp), %xmm12
140014924:     	movdqa	0xb0(%rsp), %xmm13
14001492e:     	movaps	0xa0(%rsp), %xmm14
140014937:     	movdqa	0x90(%rsp), %xmm15
140014941:     	addq	$0x138, %rsp            # imm = 0x138
140014948:     	popq	%rbx
140014949:     	popq	%rdi
14001494a:     	popq	%rsi
14001494b:     	popq	%r12
14001494d:     	popq	%r13
14001494f:     	popq	%r14
140014951:     	popq	%r15
140014953:     	popq	%rbp
140014954:     	retq
140014955:     	nopw	%cs:(%rax,%rax)
140014960:     	movq	%rdx, 0x10(%rsp)
140014965:     	pushq	%rbp
140014966:     	pushq	%r15
140014968:     	pushq	%r14
14001496a:     	pushq	%r13
14001496c:     	pushq	%r12
14001496e:     	pushq	%rsi
14001496f:     	pushq	%rdi
140014970:     	pushq	%rbx
140014971:     	subq	$0x138, %rsp            # imm = 0x138
140014978:     	leaq	0x80(%rdx), %rbp
14001497f:     	movdqa	%xmm15, 0x90(%rsp)
140014989:     	movaps	%xmm14, 0xa0(%rsp)
140014992:     	movdqa	%xmm13, 0xb0(%rsp)
14001499c:     	movdqa	%xmm12, 0xc0(%rsp)
1400149a6:     	movaps	%xmm11, 0xd0(%rsp)
1400149af:     	movaps	%xmm10, 0xe0(%rsp)
1400149b8:     	movaps	%xmm9, 0xf0(%rsp)
1400149c1:     	movaps	%xmm8, 0x100(%rsp)
1400149ca:     	movaps	%xmm7, 0x110(%rsp)
1400149d2:     	movaps	%xmm6, 0x120(%rsp)
1400149da:     	leaq	0x6a0(%rbp), %rcx
1400149e1:     	callq	0x14014ba20 <.text+0x14aa20>
1400149e6:     	movaps	0x120(%rsp), %xmm6
1400149ee:     	movaps	0x110(%rsp), %xmm7
1400149f6:     	movaps	0x100(%rsp), %xmm8
1400149ff:     	movaps	0xf0(%rsp), %xmm9
140014a08:     	movaps	0xe0(%rsp), %xmm10
140014a11:     	movaps	0xd0(%rsp), %xmm11
140014a1a:     	movdqa	0xc0(%rsp), %xmm12
140014a24:     	movdqa	0xb0(%rsp), %xmm13
140014a2e:     	movaps	0xa0(%rsp), %xmm14
140014a37:     	movdqa	0x90(%rsp), %xmm15
140014a41:     	addq	$0x138, %rsp            # imm = 0x138
140014a48:     	popq	%rbx
140014a49:     	popq	%rdi
140014a4a:     	popq	%rsi
140014a4b:     	popq	%r12
140014a4d:     	popq	%r13
140014a4f:     	popq	%r14
140014a51:     	popq	%r15
140014a53:     	popq	%rbp
140014a54:     	retq
140014a55:     	nopw	%cs:(%rax,%rax)
140014a60:     	movq	%rdx, 0x10(%rsp)
140014a65:     	pushq	%rbp
140014a66:     	pushq	%r15
140014a68:     	pushq	%r14
140014a6a:     	pushq	%r13
140014a6c:     	pushq	%r12
140014a6e:     	pushq	%rsi
140014a6f:     	pushq	%rdi
140014a70:     	pushq	%rbx
140014a71:     	subq	$0x138, %rsp            # imm = 0x138
140014a78:     	leaq	0x80(%rdx), %rbp
140014a7f:     	movdqa	%xmm15, 0x90(%rsp)
140014a89:     	movaps	%xmm14, 0xa0(%rsp)
140014a92:     	movdqa	%xmm13, 0xb0(%rsp)
140014a9c:     	movdqa	%xmm12, 0xc0(%rsp)
140014aa6:     	movaps	%xmm11, 0xd0(%rsp)
140014aaf:     	movaps	%xmm10, 0xe0(%rsp)
140014ab8:     	movaps	%xmm9, 0xf0(%rsp)
140014ac1:     	movaps	%xmm8, 0x100(%rsp)
140014aca:     	movaps	%xmm7, 0x110(%rsp)
140014ad2:     	movaps	%xmm6, 0x120(%rsp)
140014ada:     	cmpq	$0x0, 0x7f0(%rbp)
140014ae2:     	je	0x140014b05 <.text+0x13b05>
140014ae4:     	movq	0x7f0(%rbp), %rax
140014aeb:     	shlq	$0x2, %rax
140014aef:     	leaq	(%rax,%rax,2), %rdx
140014af3:     	movl	$0x4, %r8d
140014af9:     	movq	0x798(%rbp), %rcx
140014b00:     	callq	0x140613c20 <.text+0x612c20>
140014b05:     	movaps	0x120(%rsp), %xmm6
140014b0d:     	movaps	0x110(%rsp), %xmm7
140014b15:     	movaps	0x100(%rsp), %xmm8
140014b1e:     	movaps	0xf0(%rsp), %xmm9
140014b27:     	movaps	0xe0(%rsp), %xmm10
140014b30:     	movaps	0xd0(%rsp), %xmm11
140014b39:     	movdqa	0xc0(%rsp), %xmm12
140014b43:     	movdqa	0xb0(%rsp), %xmm13
140014b4d:     	movaps	0xa0(%rsp), %xmm14
140014b56:     	movdqa	0x90(%rsp), %xmm15
140014b60:     	addq	$0x138, %rsp            # imm = 0x138
140014b67:     	popq	%rbx
140014b68:     	popq	%rdi
140014b69:     	popq	%rsi
140014b6a:     	popq	%r12
140014b6c:     	popq	%r13
140014b6e:     	popq	%r14
140014b70:     	popq	%r15
140014b72:     	popq	%rbp
140014b73:     	retq
140014b74:     	nopw	%cs:(%rax,%rax)
140014b80:     	movq	%rdx, 0x10(%rsp)
140014b85:     	pushq	%rbp
140014b86:     	pushq	%r15
140014b88:     	pushq	%r14
140014b8a:     	pushq	%r13
140014b8c:     	pushq	%r12
140014b8e:     	pushq	%rsi
140014b8f:     	pushq	%rdi
140014b90:     	pushq	%rbx
140014b91:     	subq	$0x138, %rsp            # imm = 0x138
140014b98:     	leaq	0x80(%rdx), %rbp
140014b9f:     	movdqa	%xmm15, 0x90(%rsp)
140014ba9:     	movaps	%xmm14, 0xa0(%rsp)
140014bb2:     	movdqa	%xmm13, 0xb0(%rsp)
140014bbc:     	movdqa	%xmm12, 0xc0(%rsp)
140014bc6:     	movaps	%xmm11, 0xd0(%rsp)
140014bcf:     	movaps	%xmm10, 0xe0(%rsp)
140014bd8:     	movaps	%xmm9, 0xf0(%rsp)
140014be1:     	movaps	%xmm8, 0x100(%rsp)
140014bea:     	movaps	%xmm7, 0x110(%rsp)
140014bf2:     	movaps	%xmm6, 0x120(%rsp)
140014bfa:     	cmpq	$0x0, 0x7a8(%rbp)
140014c02:     	je	0x140014c21 <.text+0x13c21>
140014c04:     	movq	0x7a8(%rbp), %rdx
140014c0b:     	shlq	$0x3, %rdx
140014c0f:     	movl	$0x4, %r8d
140014c15:     	movq	0x308(%rbp), %rcx
140014c1c:     	callq	0x140613c20 <.text+0x612c20>
140014c21:     	movaps	0x120(%rsp), %xmm6
140014c29:     	movaps	0x110(%rsp), %xmm7
140014c31:     	movaps	0x100(%rsp), %xmm8
140014c3a:     	movaps	0xf0(%rsp), %xmm9
140014c43:     	movaps	0xe0(%rsp), %xmm10
140014c4c:     	movaps	0xd0(%rsp), %xmm11
140014c55:     	movdqa	0xc0(%rsp), %xmm12
140014c5f:     	movdqa	0xb0(%rsp), %xmm13
140014c69:     	movaps	0xa0(%rsp), %xmm14
140014c72:     	movdqa	0x90(%rsp), %xmm15
140014c7c:     	addq	$0x138, %rsp            # imm = 0x138
140014c83:     	popq	%rbx
140014c84:     	popq	%rdi
140014c85:     	popq	%rsi
140014c86:     	popq	%r12
140014c88:     	popq	%r13
140014c8a:     	popq	%r14
140014c8c:     	popq	%r15
140014c8e:     	popq	%rbp
140014c8f:     	retq
140014c90:     	movq	%rdx, 0x10(%rsp)
140014c95:     	pushq	%rbp
140014c96:     	pushq	%r15
140014c98:     	pushq	%r14
140014c9a:     	pushq	%r13
140014c9c:     	pushq	%r12
140014c9e:     	pushq	%rsi
140014c9f:     	pushq	%rdi
140014ca0:     	pushq	%rbx
140014ca1:     	subq	$0x138, %rsp            # imm = 0x138
140014ca8:     	leaq	0x80(%rdx), %rbp
140014caf:     	movdqa	%xmm15, 0x90(%rsp)
140014cb9:     	movaps	%xmm14, 0xa0(%rsp)
140014cc2:     	movdqa	%xmm13, 0xb0(%rsp)
140014ccc:     	movdqa	%xmm12, 0xc0(%rsp)
140014cd6:     	movaps	%xmm11, 0xd0(%rsp)
140014cdf:     	movaps	%xmm10, 0xe0(%rsp)
140014ce8:     	movaps	%xmm9, 0xf0(%rsp)
140014cf1:     	movaps	%xmm8, 0x100(%rsp)
140014cfa:     	movaps	%xmm7, 0x110(%rsp)
140014d02:     	movaps	%xmm6, 0x120(%rsp)
140014d0a:     	movq	0xd8(%rbp), %rdx
140014d11:     	testq	%rdx, %rdx
140014d14:     	je	0x140014d2c <.text+0x13d2c>
140014d16:     	movq	0xe0(%rbp), %rcx
140014d1d:     	shlq	$0x2, %rdx
140014d21:     	movl	$0x4, %r8d
140014d27:     	callq	0x140613c20 <.text+0x612c20>
140014d2c:     	movaps	0x120(%rsp), %xmm6
140014d34:     	movaps	0x110(%rsp), %xmm7
140014d3c:     	movaps	0x100(%rsp), %xmm8
140014d45:     	movaps	0xf0(%rsp), %xmm9
140014d4e:     	movaps	0xe0(%rsp), %xmm10
140014d57:     	movaps	0xd0(%rsp), %xmm11
140014d60:     	movdqa	0xc0(%rsp), %xmm12
140014d6a:     	movdqa	0xb0(%rsp), %xmm13
140014d74:     	movaps	0xa0(%rsp), %xmm14
140014d7d:     	movdqa	0x90(%rsp), %xmm15
140014d87:     	addq	$0x138, %rsp            # imm = 0x138
140014d8e:     	popq	%rbx
140014d8f:     	popq	%rdi
140014d90:     	popq	%rsi
140014d91:     	popq	%r12
140014d93:     	popq	%r13
140014d95:     	popq	%r14
140014d97:     	popq	%r15
140014d99:     	popq	%rbp
140014d9a:     	retq
140014d9b:     	nopl	(%rax,%rax)
140014da0:     	movq	%rdx, 0x10(%rsp)
140014da5:     	pushq	%rbp
140014da6:     	pushq	%r15
140014da8:     	pushq	%r14
140014daa:     	pushq	%r13
140014dac:     	pushq	%r12
140014dae:     	pushq	%rsi
140014daf:     	pushq	%rdi
140014db0:     	pushq	%rbx
140014db1:     	subq	$0x138, %rsp            # imm = 0x138
140014db8:     	leaq	0x80(%rdx), %rbp
140014dbf:     	movdqa	%xmm15, 0x90(%rsp)
140014dc9:     	movaps	%xmm14, 0xa0(%rsp)
140014dd2:     	movdqa	%xmm13, 0xb0(%rsp)
140014ddc:     	movdqa	%xmm12, 0xc0(%rsp)
140014de6:     	movaps	%xmm11, 0xd0(%rsp)
140014def:     	movaps	%xmm10, 0xe0(%rsp)
140014df8:     	movaps	%xmm9, 0xf0(%rsp)
140014e01:     	movaps	%xmm8, 0x100(%rsp)
140014e0a:     	movaps	%xmm7, 0x110(%rsp)
140014e12:     	movaps	%xmm6, 0x120(%rsp)
140014e1a:     	movq	0xc8(%rbp), %rdx
140014e21:     	testq	%rdx, %rdx
140014e24:     	je	0x140014e3c <.text+0x13e3c>
140014e26:     	movq	0xd0(%rbp), %rcx
140014e2d:     	shlq	$0x2, %rdx
140014e31:     	movl	$0x4, %r8d
140014e37:     	callq	0x140613c20 <.text+0x612c20>
140014e3c:     	movaps	0x120(%rsp), %xmm6
140014e44:     	movaps	0x110(%rsp), %xmm7
140014e4c:     	movaps	0x100(%rsp), %xmm8
140014e55:     	movaps	0xf0(%rsp), %xmm9
140014e5e:     	movaps	0xe0(%rsp), %xmm10
140014e67:     	movaps	0xd0(%rsp), %xmm11
140014e70:     	movdqa	0xc0(%rsp), %xmm12
140014e7a:     	movdqa	0xb0(%rsp), %xmm13
140014e84:     	movaps	0xa0(%rsp), %xmm14
140014e8d:     	movdqa	0x90(%rsp), %xmm15
140014e97:     	addq	$0x138, %rsp            # imm = 0x138
140014e9e:     	popq	%rbx
140014e9f:     	popq	%rdi
140014ea0:     	popq	%rsi
140014ea1:     	popq	%r12
140014ea3:     	popq	%r13
140014ea5:     	popq	%r14
140014ea7:     	popq	%r15
140014ea9:     	popq	%rbp
140014eaa:     	retq
140014eab:     	nopl	(%rax,%rax)
140014eb0:     	movq	%rdx, 0x10(%rsp)
140014eb5:     	pushq	%rbp
140014eb6:     	pushq	%r15
140014eb8:     	pushq	%r14
140014eba:     	pushq	%r13
140014ebc:     	pushq	%r12
140014ebe:     	pushq	%rsi
140014ebf:     	pushq	%rdi
140014ec0:     	pushq	%rbx
140014ec1:     	subq	$0x138, %rsp            # imm = 0x138
140014ec8:     	leaq	0x80(%rdx), %rbp
140014ecf:     	movdqa	%xmm15, 0x90(%rsp)
140014ed9:     	movaps	%xmm14, 0xa0(%rsp)
140014ee2:     	movdqa	%xmm13, 0xb0(%rsp)
140014eec:     	movdqa	%xmm12, 0xc0(%rsp)
140014ef6:     	movaps	%xmm11, 0xd0(%rsp)
140014eff:     	movaps	%xmm10, 0xe0(%rsp)
140014f08:     	movaps	%xmm9, 0xf0(%rsp)
140014f11:     	movaps	%xmm8, 0x100(%rsp)
140014f1a:     	movaps	%xmm7, 0x110(%rsp)
140014f22:     	movaps	%xmm6, 0x120(%rsp)
140014f2a:     	movq	0x7a0(%rbp), %rdx
140014f31:     	testq	%rdx, %rdx
140014f34:     	je	0x140014f4c <.text+0x13f4c>
140014f36:     	shlq	$0x2, %rdx
140014f3a:     	movl	$0x4, %r8d
140014f40:     	movq	0x138(%rbp), %rcx
140014f47:     	callq	0x140613c20 <.text+0x612c20>
140014f4c:     	movaps	0x120(%rsp), %xmm6
140014f54:     	movaps	0x110(%rsp), %xmm7
140014f5c:     	movaps	0x100(%rsp), %xmm8
140014f65:     	movaps	0xf0(%rsp), %xmm9
140014f6e:     	movaps	0xe0(%rsp), %xmm10
140014f77:     	movaps	0xd0(%rsp), %xmm11
140014f80:     	movdqa	0xc0(%rsp), %xmm12
140014f8a:     	movdqa	0xb0(%rsp), %xmm13
140014f94:     	movaps	0xa0(%rsp), %xmm14
140014f9d:     	movdqa	0x90(%rsp), %xmm15
140014fa7:     	addq	$0x138, %rsp            # imm = 0x138
140014fae:     	popq	%rbx
140014faf:     	popq	%rdi
140014fb0:     	popq	%rsi
140014fb1:     	popq	%r12
140014fb3:     	popq	%r13
140014fb5:     	popq	%r14
140014fb7:     	popq	%r15
140014fb9:     	popq	%rbp
140014fba:     	retq
140014fbb:     	nopl	(%rax,%rax)
140014fc0:     	movq	%rdx, 0x10(%rsp)
140014fc5:     	pushq	%rbp
140014fc6:     	pushq	%r15
140014fc8:     	pushq	%r14
140014fca:     	pushq	%r13
140014fcc:     	pushq	%r12
140014fce:     	pushq	%rsi
140014fcf:     	pushq	%rdi
140014fd0:     	pushq	%rbx
140014fd1:     	subq	$0x138, %rsp            # imm = 0x138
140014fd8:     	leaq	0x80(%rdx), %rbp
140014fdf:     	movdqa	%xmm15, 0x90(%rsp)
140014fe9:     	movaps	%xmm14, 0xa0(%rsp)
140014ff2:     	movdqa	%xmm13, 0xb0(%rsp)
140014ffc:     	movdqa	%xmm12, 0xc0(%rsp)
140015006:     	movaps	%xmm11, 0xd0(%rsp)
14001500f:     	movaps	%xmm10, 0xe0(%rsp)
140015018:     	movaps	%xmm9, 0xf0(%rsp)
140015021:     	movaps	%xmm8, 0x100(%rsp)
14001502a:     	movaps	%xmm7, 0x110(%rsp)
140015032:     	movaps	%xmm6, 0x120(%rsp)
14001503a:     	movzbl	0x7d1(%rbp), %ebx
140015041:     	movzbl	0x7d2(%rbp), %esi
140015048:     	testb	$0x1, %sil
14001504c:     	je	0x1400150fa <.text+0x140fa>
140015052:     	movq	0x3e0(%rbp), %rdx
140015059:     	testq	%rdx, %rdx
14001505c:     	je	0x140015074 <.text+0x14074>
14001505e:     	movq	0x3e8(%rbp), %rcx
140015065:     	shlq	$0x2, %rdx
140015069:     	movl	$0x4, %r8d
14001506f:     	callq	0x140613c20 <.text+0x612c20>
140015074:     	movb	%sil, 0x7d5(%rbp)
14001507b:     	andb	$0x1, %bl
14001507e:     	movb	%bl, 0x7d4(%rbp)
140015084:     	movb	$0x0, 0x7d3(%rbp)
14001508b:     	movaps	0x120(%rsp), %xmm6
140015093:     	movaps	0x110(%rsp), %xmm7
14001509b:     	movaps	0x100(%rsp), %xmm8
1400150a4:     	movaps	0xf0(%rsp), %xmm9
1400150ad:     	movaps	0xe0(%rsp), %xmm10
1400150b6:     	movaps	0xd0(%rsp), %xmm11
1400150bf:     	movdqa	0xc0(%rsp), %xmm12
1400150c9:     	movdqa	0xb0(%rsp), %xmm13
1400150d3:     	movaps	0xa0(%rsp), %xmm14
1400150dc:     	movdqa	0x90(%rsp), %xmm15
1400150e6:     	addq	$0x138, %rsp            # imm = 0x138
1400150ed:     	popq	%rbx
1400150ee:     	popq	%rdi
1400150ef:     	popq	%rsi
1400150f0:     	popq	%r12
1400150f2:     	popq	%r13
1400150f4:     	popq	%r14
1400150f6:     	popq	%r15
1400150f8:     	popq	%rbp
1400150f9:     	retq
1400150fa:     	movb	%sil, 0x7d5(%rbp)
140015101:     	andb	$0x1, %bl
140015104:     	movb	%bl, 0x7d4(%rbp)
14001510a:     	movb	$0x0, 0x7d3(%rbp)
140015111:     	movaps	0x120(%rsp), %xmm6
140015119:     	movaps	0x110(%rsp), %xmm7
140015121:     	movaps	0x100(%rsp), %xmm8
14001512a:     	movaps	0xf0(%rsp), %xmm9
140015133:     	movaps	0xe0(%rsp), %xmm10
14001513c:     	movaps	0xd0(%rsp), %xmm11
140015145:     	movdqa	0xc0(%rsp), %xmm12
14001514f:     	movdqa	0xb0(%rsp), %xmm13
140015159:     	movaps	0xa0(%rsp), %xmm14
140015162:     	movdqa	0x90(%rsp), %xmm15
14001516c:     	addq	$0x138, %rsp            # imm = 0x138
140015173:     	popq	%rbx
140015174:     	popq	%rdi
140015175:     	popq	%rsi
140015176:     	popq	%r12
140015178:     	popq	%r13
14001517a:     	popq	%r14
14001517c:     	popq	%r15
14001517e:     	popq	%rbp
14001517f:     	retq
140015180:     	movq	%rdx, 0x10(%rsp)
140015185:     	pushq	%rbp
140015186:     	pushq	%r15
140015188:     	pushq	%r14
14001518a:     	pushq	%r13
14001518c:     	pushq	%r12
14001518e:     	pushq	%rsi
14001518f:     	pushq	%rdi
140015190:     	pushq	%rbx
140015191:     	subq	$0x138, %rsp            # imm = 0x138
140015198:     	leaq	0x80(%rdx), %rbp
14001519f:     	movdqa	%xmm15, 0x90(%rsp)
1400151a9:     	movaps	%xmm14, 0xa0(%rsp)
1400151b2:     	movdqa	%xmm13, 0xb0(%rsp)
1400151bc:     	movdqa	%xmm12, 0xc0(%rsp)
1400151c6:     	movaps	%xmm11, 0xd0(%rsp)
1400151cf:     	movaps	%xmm10, 0xe0(%rsp)
1400151d8:     	movaps	%xmm9, 0xf0(%rsp)
1400151e1:     	movaps	%xmm8, 0x100(%rsp)
1400151ea:     	movaps	%xmm7, 0x110(%rsp)
1400151f2:     	movaps	%xmm6, 0x120(%rsp)
1400151fa:     	movq	0x430(%rbp), %rdx
140015201:     	testq	%rdx, %rdx
140015204:     	je	0x14001521c <.text+0x1421c>
140015206:     	shlq	$0x3, %rdx
14001520a:     	movl	$0x4, %r8d
140015210:     	movq	0x300(%rbp), %rcx
140015217:     	callq	0x140613c20 <.text+0x612c20>
14001521c:     	movaps	0x120(%rsp), %xmm6
140015224:     	movaps	0x110(%rsp), %xmm7
14001522c:     	movaps	0x100(%rsp), %xmm8
140015235:     	movaps	0xf0(%rsp), %xmm9
14001523e:     	movaps	0xe0(%rsp), %xmm10
140015247:     	movaps	0xd0(%rsp), %xmm11
140015250:     	movdqa	0xc0(%rsp), %xmm12
14001525a:     	movdqa	0xb0(%rsp), %xmm13
140015264:     	movaps	0xa0(%rsp), %xmm14
14001526d:     	movdqa	0x90(%rsp), %xmm15
140015277:     	addq	$0x138, %rsp            # imm = 0x138
14001527e:     	popq	%rbx
14001527f:     	popq	%rdi
140015280:     	popq	%rsi
140015281:     	popq	%r12
140015283:     	popq	%r13
140015285:     	popq	%r14
140015287:     	popq	%r15
140015289:     	popq	%rbp
14001528a:     	retq
14001528b:     	nopl	(%rax,%rax)
140015290:     	movq	%rdx, 0x10(%rsp)
140015295:     	pushq	%rbp
140015296:     	pushq	%r15
140015298:     	pushq	%r14
14001529a:     	pushq	%r13
14001529c:     	pushq	%r12
14001529e:     	pushq	%rsi
14001529f:     	pushq	%rdi
1400152a0:     	pushq	%rbx
1400152a1:     	subq	$0x138, %rsp            # imm = 0x138
1400152a8:     	leaq	0x80(%rdx), %rbp
1400152af:     	movdqa	%xmm15, 0x90(%rsp)
1400152b9:     	movaps	%xmm14, 0xa0(%rsp)
1400152c2:     	movdqa	%xmm13, 0xb0(%rsp)
1400152cc:     	movdqa	%xmm12, 0xc0(%rsp)
1400152d6:     	movaps	%xmm11, 0xd0(%rsp)
1400152df:     	movaps	%xmm10, 0xe0(%rsp)
1400152e8:     	movaps	%xmm9, 0xf0(%rsp)
1400152f1:     	movaps	%xmm8, 0x100(%rsp)
1400152fa:     	movaps	%xmm7, 0x110(%rsp)
140015302:     	movaps	%xmm6, 0x120(%rsp)
14001530a:     	leaq	0x590(%rbp), %rcx
140015311:     	callq	0x14014ba20 <.text+0x14aa20>
140015316:     	movb	$0x1, 0x816(%rbp)
14001531d:     	movb	$0x1, 0x815(%rbp)
140015324:     	movaps	0x120(%rsp), %xmm6
14001532c:     	movaps	0x110(%rsp), %xmm7
140015334:     	movaps	0x100(%rsp), %xmm8
14001533d:     	movaps	0xf0(%rsp), %xmm9
140015346:     	movaps	0xe0(%rsp), %xmm10
14001534f:     	movaps	0xd0(%rsp), %xmm11
140015358:     	movdqa	0xc0(%rsp), %xmm12
140015362:     	movdqa	0xb0(%rsp), %xmm13
14001536c:     	movaps	0xa0(%rsp), %xmm14
140015375:     	movdqa	0x90(%rsp), %xmm15
14001537f:     	addq	$0x138, %rsp            # imm = 0x138
140015386:     	popq	%rbx
140015387:     	popq	%rdi
140015388:     	popq	%rsi
140015389:     	popq	%r12
14001538b:     	popq	%r13
14001538d:     	popq	%r14
14001538f:     	popq	%r15
140015391:     	popq	%rbp
140015392:     	retq
140015393:     	nopw	%cs:(%rax,%rax)
1400153a0:     	movq	%rdx, 0x10(%rsp)
1400153a5:     	pushq	%rbp
1400153a6:     	pushq	%r15
1400153a8:     	pushq	%r14
1400153aa:     	pushq	%r13
1400153ac:     	pushq	%r12
1400153ae:     	pushq	%rsi
1400153af:     	pushq	%rdi
1400153b0:     	pushq	%rbx
1400153b1:     	subq	$0x138, %rsp            # imm = 0x138
1400153b8:     	leaq	0x80(%rdx), %rbp
1400153bf:     	movdqa	%xmm15, 0x90(%rsp)
1400153c9:     	movaps	%xmm14, 0xa0(%rsp)
1400153d2:     	movdqa	%xmm13, 0xb0(%rsp)
1400153dc:     	movdqa	%xmm12, 0xc0(%rsp)
1400153e6:     	movaps	%xmm11, 0xd0(%rsp)
1400153ef:     	movaps	%xmm10, 0xe0(%rsp)
1400153f8:     	movaps	%xmm9, 0xf0(%rsp)
140015401:     	movaps	%xmm8, 0x100(%rsp)
14001540a:     	movaps	%xmm7, 0x110(%rsp)
140015412:     	movaps	%xmm6, 0x120(%rsp)
14001541a:     	movzbl	0x7d3(%rbp), %ebx
140015421:     	movzbl	0x7d4(%rbp), %esi
140015428:     	movzbl	0x7d5(%rbp), %edi
14001542f:     	testb	$0x1, %dil
140015433:     	je	0x1400154ec <.text+0x144ec>
140015439:     	movq	0x2a0(%rbp), %rdx
140015440:     	testq	%rdx, %rdx
140015443:     	je	0x14001545b <.text+0x1445b>
140015445:     	movq	0x2a8(%rbp), %rcx
14001544c:     	shlq	$0x2, %rdx
140015450:     	movl	$0x4, %r8d
140015456:     	callq	0x140613c20 <.text+0x612c20>
14001545b:     	movb	%dil, 0x7d9(%rbp)
140015462:     	andb	$0x1, %sil
140015466:     	movb	%sil, 0x7d8(%rbp)
14001546d:     	andb	$0x1, %bl
140015470:     	movb	%bl, 0x7d7(%rbp)
140015476:     	movb	$0x0, 0x7d6(%rbp)
14001547d:     	movaps	0x120(%rsp), %xmm6
140015485:     	movaps	0x110(%rsp), %xmm7
14001548d:     	movaps	0x100(%rsp), %xmm8
140015496:     	movaps	0xf0(%rsp), %xmm9
14001549f:     	movaps	0xe0(%rsp), %xmm10
1400154a8:     	movaps	0xd0(%rsp), %xmm11
1400154b1:     	movdqa	0xc0(%rsp), %xmm12
1400154bb:     	movdqa	0xb0(%rsp), %xmm13
1400154c5:     	movaps	0xa0(%rsp), %xmm14
1400154ce:     	movdqa	0x90(%rsp), %xmm15
1400154d8:     	addq	$0x138, %rsp            # imm = 0x138
1400154df:     	popq	%rbx
1400154e0:     	popq	%rdi
1400154e1:     	popq	%rsi
1400154e2:     	popq	%r12
1400154e4:     	popq	%r13
1400154e6:     	popq	%r14
1400154e8:     	popq	%r15
1400154ea:     	popq	%rbp
1400154eb:     	retq
1400154ec:     	movb	%dil, 0x7d9(%rbp)
1400154f3:     	andb	$0x1, %sil
1400154f7:     	movb	%sil, 0x7d8(%rbp)
1400154fe:     	andb	$0x1, %bl
140015501:     	movb	%bl, 0x7d7(%rbp)
140015507:     	movb	$0x0, 0x7d6(%rbp)
14001550e:     	movaps	0x120(%rsp), %xmm6
140015516:     	movaps	0x110(%rsp), %xmm7
14001551e:     	movaps	0x100(%rsp), %xmm8
140015527:     	movaps	0xf0(%rsp), %xmm9
140015530:     	movaps	0xe0(%rsp), %xmm10
140015539:     	movaps	0xd0(%rsp), %xmm11
140015542:     	movdqa	0xc0(%rsp), %xmm12
14001554c:     	movdqa	0xb0(%rsp), %xmm13
140015556:     	movaps	0xa0(%rsp), %xmm14
14001555f:     	movdqa	0x90(%rsp), %xmm15
140015569:     	addq	$0x138, %rsp            # imm = 0x138
140015570:     	popq	%rbx
140015571:     	popq	%rdi
140015572:     	popq	%rsi
140015573:     	popq	%r12
140015575:     	popq	%r13
140015577:     	popq	%r14
140015579:     	popq	%r15
14001557b:     	popq	%rbp
14001557c:     	retq
14001557d:     	nopl	(%rax)
140015580:     	movq	%rdx, 0x10(%rsp)
140015585:     	pushq	%rbp
140015586:     	pushq	%r15
140015588:     	pushq	%r14
14001558a:     	pushq	%r13
14001558c:     	pushq	%r12
14001558e:     	pushq	%rsi
14001558f:     	pushq	%rdi
140015590:     	pushq	%rbx
140015591:     	subq	$0x138, %rsp            # imm = 0x138
140015598:     	leaq	0x80(%rdx), %rbp
14001559f:     	movdqa	%xmm15, 0x90(%rsp)
1400155a9:     	movaps	%xmm14, 0xa0(%rsp)
1400155b2:     	movdqa	%xmm13, 0xb0(%rsp)
1400155bc:     	movdqa	%xmm12, 0xc0(%rsp)
1400155c6:     	movaps	%xmm11, 0xd0(%rsp)
1400155cf:     	movaps	%xmm10, 0xe0(%rsp)
1400155d8:     	movaps	%xmm9, 0xf0(%rsp)
1400155e1:     	movaps	%xmm8, 0x100(%rsp)
1400155ea:     	movaps	%xmm7, 0x110(%rsp)
1400155f2:     	movaps	%xmm6, 0x120(%rsp)
1400155fa:     	leaq	0x3e0(%rbp), %rcx
140015601:     	callq	0x14014ba20 <.text+0x14aa20>
140015606:     	movb	$0x1, 0x813(%rbp)
14001560d:     	movaps	0x120(%rsp), %xmm6
140015615:     	movaps	0x110(%rsp), %xmm7
14001561d:     	movaps	0x100(%rsp), %xmm8
140015626:     	movaps	0xf0(%rsp), %xmm9
14001562f:     	movaps	0xe0(%rsp), %xmm10
140015638:     	movaps	0xd0(%rsp), %xmm11
140015641:     	movdqa	0xc0(%rsp), %xmm12
14001564b:     	movdqa	0xb0(%rsp), %xmm13
140015655:     	movaps	0xa0(%rsp), %xmm14
14001565e:     	movdqa	0x90(%rsp), %xmm15
140015668:     	addq	$0x138, %rsp            # imm = 0x138
14001566f:     	popq	%rbx
140015670:     	popq	%rdi
140015671:     	popq	%rsi
140015672:     	popq	%r12
140015674:     	popq	%r13
140015676:     	popq	%r14
140015678:     	popq	%r15
14001567a:     	popq	%rbp
14001567b:     	retq
14001567c:     	nopl	(%rax)
140015680:     	movq	%rdx, 0x10(%rsp)
140015685:     	pushq	%rbp
140015686:     	pushq	%r15
140015688:     	pushq	%r14
14001568a:     	pushq	%r13
14001568c:     	pushq	%r12
14001568e:     	pushq	%rsi
14001568f:     	pushq	%rdi
140015690:     	pushq	%rbx
140015691:     	subq	$0x138, %rsp            # imm = 0x138
140015698:     	leaq	0x80(%rdx), %rbp
14001569f:     	movdqa	%xmm15, 0x90(%rsp)
1400156a9:     	movaps	%xmm14, 0xa0(%rsp)
1400156b2:     	movdqa	%xmm13, 0xb0(%rsp)
1400156bc:     	movdqa	%xmm12, 0xc0(%rsp)
1400156c6:     	movaps	%xmm11, 0xd0(%rsp)
1400156cf:     	movaps	%xmm10, 0xe0(%rsp)
1400156d8:     	movaps	%xmm9, 0xf0(%rsp)
1400156e1:     	movaps	%xmm8, 0x100(%rsp)
1400156ea:     	movaps	%xmm7, 0x110(%rsp)
1400156f2:     	movaps	%xmm6, 0x120(%rsp)
1400156fa:     	movq	0x588(%rbp), %rdx
140015701:     	testq	%rdx, %rdx
140015704:     	je	0x14001571c <.text+0x1471c>
140015706:     	shlq	$0x3, %rdx
14001570a:     	movl	$0x4, %r8d
140015710:     	movq	0x2f8(%rbp), %rcx
140015717:     	callq	0x140613c20 <.text+0x612c20>
14001571c:     	movaps	0x120(%rsp), %xmm6
140015724:     	movaps	0x110(%rsp), %xmm7
14001572c:     	movaps	0x100(%rsp), %xmm8
140015735:     	movaps	0xf0(%rsp), %xmm9
14001573e:     	movaps	0xe0(%rsp), %xmm10
140015747:     	movaps	0xd0(%rsp), %xmm11
140015750:     	movdqa	0xc0(%rsp), %xmm12
14001575a:     	movdqa	0xb0(%rsp), %xmm13
140015764:     	movaps	0xa0(%rsp), %xmm14
14001576d:     	movdqa	0x90(%rsp), %xmm15
140015777:     	addq	$0x138, %rsp            # imm = 0x138
14001577e:     	popq	%rbx
14001577f:     	popq	%rdi
140015780:     	popq	%rsi
140015781:     	popq	%r12
140015783:     	popq	%r13
140015785:     	popq	%r14
140015787:     	popq	%r15
140015789:     	popq	%rbp
14001578a:     	retq
14001578b:     	nopl	(%rax,%rax)
140015790:     	movq	%rdx, 0x10(%rsp)
140015795:     	pushq	%rbp
140015796:     	pushq	%r15
140015798:     	pushq	%r14
14001579a:     	pushq	%r13
14001579c:     	pushq	%r12
14001579e:     	pushq	%rsi
14001579f:     	pushq	%rdi
1400157a0:     	pushq	%rbx
1400157a1:     	subq	$0x138, %rsp            # imm = 0x138
1400157a8:     	leaq	0x80(%rdx), %rbp
1400157af:     	movdqa	%xmm15, 0x90(%rsp)
1400157b9:     	movaps	%xmm14, 0xa0(%rsp)
1400157c2:     	movdqa	%xmm13, 0xb0(%rsp)
1400157cc:     	movdqa	%xmm12, 0xc0(%rsp)
1400157d6:     	movaps	%xmm11, 0xd0(%rsp)
1400157df:     	movaps	%xmm10, 0xe0(%rsp)
1400157e8:     	movaps	%xmm9, 0xf0(%rsp)
1400157f1:     	movaps	%xmm8, 0x100(%rsp)
1400157fa:     	movaps	%xmm7, 0x110(%rsp)
140015802:     	movaps	%xmm6, 0x120(%rsp)
14001580a:     	movzbl	0x7d6(%rbp), %ebx
140015811:     	movzbl	0x7d7(%rbp), %esi
140015818:     	movzbl	0x7d8(%rbp), %edi
14001581f:     	testb	$0x1, 0x7d9(%rbp)
140015826:     	je	0x1400158e3 <.text+0x148e3>
14001582c:     	movq	0x680(%rbp), %rdx
140015833:     	testq	%rdx, %rdx
140015836:     	je	0x14001584e <.text+0x1484e>
140015838:     	movq	0x688(%rbp), %rcx
14001583f:     	shlq	$0x3, %rdx
140015843:     	movl	$0x4, %r8d
140015849:     	callq	0x140613c20 <.text+0x612c20>
14001584e:     	andb	$0x1, %dil
140015852:     	movb	%dil, 0x7dd(%rbp)
140015859:     	andb	$0x1, %sil
14001585d:     	movb	%sil, 0x7dc(%rbp)
140015864:     	andb	$0x1, %bl
140015867:     	movb	%bl, 0x7db(%rbp)
14001586d:     	movb	$0x0, 0x7da(%rbp)
140015874:     	movaps	0x120(%rsp), %xmm6
14001587c:     	movaps	0x110(%rsp), %xmm7
140015884:     	movaps	0x100(%rsp), %xmm8
14001588d:     	movaps	0xf0(%rsp), %xmm9
140015896:     	movaps	0xe0(%rsp), %xmm10
14001589f:     	movaps	0xd0(%rsp), %xmm11
1400158a8:     	movdqa	0xc0(%rsp), %xmm12
1400158b2:     	movdqa	0xb0(%rsp), %xmm13
1400158bc:     	movaps	0xa0(%rsp), %xmm14
1400158c5:     	movdqa	0x90(%rsp), %xmm15
1400158cf:     	addq	$0x138, %rsp            # imm = 0x138
1400158d6:     	popq	%rbx
1400158d7:     	popq	%rdi
1400158d8:     	popq	%rsi
1400158d9:     	popq	%r12
1400158db:     	popq	%r13
1400158dd:     	popq	%r14
1400158df:     	popq	%r15
1400158e1:     	popq	%rbp
1400158e2:     	retq
1400158e3:     	andb	$0x1, %dil
1400158e7:     	movb	%dil, 0x7dd(%rbp)
1400158ee:     	andb	$0x1, %sil
1400158f2:     	movb	%sil, 0x7dc(%rbp)
1400158f9:     	andb	$0x1, %bl
1400158fc:     	movb	%bl, 0x7db(%rbp)
140015902:     	movb	$0x0, 0x7da(%rbp)
140015909:     	movaps	0x120(%rsp), %xmm6
140015911:     	movaps	0x110(%rsp), %xmm7
140015919:     	movaps	0x100(%rsp), %xmm8
140015922:     	movaps	0xf0(%rsp), %xmm9
14001592b:     	movaps	0xe0(%rsp), %xmm10
140015934:     	movaps	0xd0(%rsp), %xmm11
14001593d:     	movdqa	0xc0(%rsp), %xmm12
140015947:     	movdqa	0xb0(%rsp), %xmm13
140015951:     	movaps	0xa0(%rsp), %xmm14
14001595a:     	movdqa	0x90(%rsp), %xmm15
140015964:     	addq	$0x138, %rsp            # imm = 0x138
14001596b:     	popq	%rbx
14001596c:     	popq	%rdi
14001596d:     	popq	%rsi
14001596e:     	popq	%r12
140015970:     	popq	%r13
140015972:     	popq	%r14
140015974:     	popq	%r15
140015976:     	popq	%rbp
140015977:     	retq
140015978:     	nopl	(%rax,%rax)
140015980:     	movq	%rdx, 0x10(%rsp)
140015985:     	pushq	%rbp
140015986:     	pushq	%r15
140015988:     	pushq	%r14
14001598a:     	pushq	%r13
14001598c:     	pushq	%r12
14001598e:     	pushq	%rsi
14001598f:     	pushq	%rdi
140015990:     	pushq	%rbx
140015991:     	subq	$0x138, %rsp            # imm = 0x138
140015998:     	leaq	0x80(%rdx), %rbp
14001599f:     	movdqa	%xmm15, 0x90(%rsp)
1400159a9:     	movaps	%xmm14, 0xa0(%rsp)
1400159b2:     	movdqa	%xmm13, 0xb0(%rsp)
1400159bc:     	movdqa	%xmm12, 0xc0(%rsp)
1400159c6:     	movaps	%xmm11, 0xd0(%rsp)
1400159cf:     	movaps	%xmm10, 0xe0(%rsp)
1400159d8:     	movaps	%xmm9, 0xf0(%rsp)
1400159e1:     	movaps	%xmm8, 0x100(%rsp)
1400159ea:     	movaps	%xmm7, 0x110(%rsp)
1400159f2:     	movaps	%xmm6, 0x120(%rsp)
1400159fa:     	movzbl	0x7da(%rbp), %ebx
140015a01:     	movzbl	0x7db(%rbp), %esi
140015a08:     	movzbl	0x7dc(%rbp), %edi
140015a0f:     	cmpb	$0x0, 0x7dd(%rbp)
140015a16:     	je	0x140015a3e <.text+0x14a3e>
140015a18:     	movq	0x650(%rbp), %rax
140015a1f:     	testq	%rax, %rax
140015a22:     	je	0x140015a3e <.text+0x14a3e>
140015a24:     	movq	0x658(%rbp), %rcx
140015a2b:     	shlq	$0x2, %rax
140015a2f:     	leaq	(%rax,%rax,2), %rdx
140015a33:     	movl	$0x4, %r8d
140015a39:     	callq	0x140613c20 <.text+0x612c20>
140015a3e:     	movq	0x7a8(%rbp), %rdx
140015a45:     	testq	%rdx, %rdx
140015a48:     	je	0x140015a60 <.text+0x14a60>
140015a4a:     	shlq	$0x2, %rdx
140015a4e:     	movl	$0x4, %r8d
140015a54:     	movq	0x568(%rbp), %rcx
140015a5b:     	callq	0x140613c20 <.text+0x612c20>
140015a60:     	movq	0x578(%rbp), %rdx
140015a67:     	testq	%rdx, %rdx
140015a6a:     	setne	%al
140015a6d:     	testb	%al, %bl
140015a6f:     	je	0x140015a87 <.text+0x14a87>
140015a71:     	shlq	$0x3, %rdx
140015a75:     	movl	$0x4, %r8d
140015a7b:     	movq	0x668(%rbp), %rcx
140015a82:     	callq	0x140613c20 <.text+0x612c20>
140015a87:     	movq	0x520(%rbp), %rdx
140015a8e:     	testq	%rdx, %rdx
140015a91:     	setne	%al
140015a94:     	testb	%al, %sil
140015a97:     	je	0x140015aaf <.text+0x14aaf>
140015a99:     	shlq	$0x2, %rdx
140015a9d:     	movl	$0x4, %r8d
140015aa3:     	movq	0x7a0(%rbp), %rcx
140015aaa:     	callq	0x140613c20 <.text+0x612c20>
140015aaf:     	testb	$0x1, %dil
140015ab3:     	je	0x140015b73 <.text+0x14b73>
140015ab9:     	movq	0x670(%rbp), %rdx
140015ac0:     	testq	%rdx, %rdx
140015ac3:     	je	0x140015adb <.text+0x14adb>
140015ac5:     	shlq	$0x2, %rdx
140015ac9:     	movl	$0x4, %r8d
140015acf:     	movq	0x798(%rbp), %rcx
140015ad6:     	callq	0x140613c20 <.text+0x612c20>
140015adb:     	movb	$0x1, 0x7ee(%rbp)
140015ae2:     	andb	$0x1, %sil
140015ae6:     	movb	%sil, 0x7ed(%rbp)
140015aed:     	andb	$0x1, %bl
140015af0:     	movb	%bl, 0x7ec(%rbp)
140015af6:     	movb	$0x0, 0x7eb(%rbp)
140015afd:     	movb	$0x1, 0x7ea(%rbp)
140015b04:     	movaps	0x120(%rsp), %xmm6
140015b0c:     	movaps	0x110(%rsp), %xmm7
140015b14:     	movaps	0x100(%rsp), %xmm8
140015b1d:     	movaps	0xf0(%rsp), %xmm9
140015b26:     	movaps	0xe0(%rsp), %xmm10
140015b2f:     	movaps	0xd0(%rsp), %xmm11
140015b38:     	movdqa	0xc0(%rsp), %xmm12
140015b42:     	movdqa	0xb0(%rsp), %xmm13
140015b4c:     	movaps	0xa0(%rsp), %xmm14
140015b55:     	movdqa	0x90(%rsp), %xmm15
140015b5f:     	addq	$0x138, %rsp            # imm = 0x138
140015b66:     	popq	%rbx
140015b67:     	popq	%rdi
140015b68:     	popq	%rsi
140015b69:     	popq	%r12
140015b6b:     	popq	%r13
140015b6d:     	popq	%r14
140015b6f:     	popq	%r15
140015b71:     	popq	%rbp
140015b72:     	retq
140015b73:     	movb	$0x0, 0x7ee(%rbp)
140015b7a:     	andb	$0x1, %sil
140015b7e:     	movb	%sil, 0x7ed(%rbp)
140015b85:     	andb	$0x1, %bl
140015b88:     	movb	%bl, 0x7ec(%rbp)
140015b8e:     	movb	$0x0, 0x7eb(%rbp)
140015b95:     	movb	$0x1, 0x7ea(%rbp)
140015b9c:     	movaps	0x120(%rsp), %xmm6
140015ba4:     	movaps	0x110(%rsp), %xmm7
140015bac:     	movaps	0x100(%rsp), %xmm8
140015bb5:     	movaps	0xf0(%rsp), %xmm9
140015bbe:     	movaps	0xe0(%rsp), %xmm10
140015bc7:     	movaps	0xd0(%rsp), %xmm11
140015bd0:     	movdqa	0xc0(%rsp), %xmm12
140015bda:     	movdqa	0xb0(%rsp), %xmm13
140015be4:     	movaps	0xa0(%rsp), %xmm14
140015bed:     	movdqa	0x90(%rsp), %xmm15
140015bf7:     	addq	$0x138, %rsp            # imm = 0x138
140015bfe:     	popq	%rbx
140015bff:     	popq	%rdi
140015c00:     	popq	%rsi
140015c01:     	popq	%r12
140015c03:     	popq	%r13
140015c05:     	popq	%r14
140015c07:     	popq	%r15
140015c09:     	popq	%rbp
140015c0a:     	retq
140015c0b:     	nopl	(%rax,%rax)
140015c10:     	movq	%rdx, 0x10(%rsp)
140015c15:     	pushq	%rbp
140015c16:     	pushq	%r15
140015c18:     	pushq	%r14
140015c1a:     	pushq	%r13
140015c1c:     	pushq	%r12
140015c1e:     	pushq	%rsi
140015c1f:     	pushq	%rdi
140015c20:     	pushq	%rbx
140015c21:     	subq	$0x138, %rsp            # imm = 0x138
140015c28:     	leaq	0x80(%rdx), %rbp
140015c2f:     	movdqa	%xmm15, 0x90(%rsp)
140015c39:     	movaps	%xmm14, 0xa0(%rsp)
140015c42:     	movdqa	%xmm13, 0xb0(%rsp)
140015c4c:     	movdqa	%xmm12, 0xc0(%rsp)
140015c56:     	movaps	%xmm11, 0xd0(%rsp)
140015c5f:     	movaps	%xmm10, 0xe0(%rsp)
140015c68:     	movaps	%xmm9, 0xf0(%rsp)
140015c71:     	movaps	%xmm8, 0x100(%rsp)
140015c7a:     	movaps	%xmm7, 0x110(%rsp)
140015c82:     	movaps	%xmm6, 0x120(%rsp)
140015c8a:     	leaq	0x380(%rbp), %rcx
140015c91:     	callq	0x14014ba20 <.text+0x14aa20>
140015c96:     	movaps	0x120(%rsp), %xmm6
140015c9e:     	movaps	0x110(%rsp), %xmm7
140015ca6:     	movaps	0x100(%rsp), %xmm8
140015caf:     	movaps	0xf0(%rsp), %xmm9
140015cb8:     	movaps	0xe0(%rsp), %xmm10
140015cc1:     	movaps	0xd0(%rsp), %xmm11
140015cca:     	movdqa	0xc0(%rsp), %xmm12
140015cd4:     	movdqa	0xb0(%rsp), %xmm13
140015cde:     	movaps	0xa0(%rsp), %xmm14
140015ce7:     	movdqa	0x90(%rsp), %xmm15
140015cf1:     	addq	$0x138, %rsp            # imm = 0x138
140015cf8:     	popq	%rbx
140015cf9:     	popq	%rdi
140015cfa:     	popq	%rsi
140015cfb:     	popq	%r12
140015cfd:     	popq	%r13
140015cff:     	popq	%r14
140015d01:     	popq	%r15
140015d03:     	popq	%rbp
140015d04:     	retq
140015d05:     	nopw	%cs:(%rax,%rax)
140015d10:     	movq	%rdx, 0x10(%rsp)
140015d15:     	pushq	%rbp
140015d16:     	pushq	%r15
140015d18:     	pushq	%r14
140015d1a:     	pushq	%r13
140015d1c:     	pushq	%r12
140015d1e:     	pushq	%rsi
140015d1f:     	pushq	%rdi
140015d20:     	pushq	%rbx
140015d21:     	subq	$0x138, %rsp            # imm = 0x138
140015d28:     	leaq	0x80(%rdx), %rbp
140015d2f:     	movdqa	%xmm15, 0x90(%rsp)
140015d39:     	movaps	%xmm14, 0xa0(%rsp)
140015d42:     	movdqa	%xmm13, 0xb0(%rsp)
140015d4c:     	movdqa	%xmm12, 0xc0(%rsp)
140015d56:     	movaps	%xmm11, 0xd0(%rsp)
140015d5f:     	movaps	%xmm10, 0xe0(%rsp)
140015d68:     	movaps	%xmm9, 0xf0(%rsp)
140015d71:     	movaps	%xmm8, 0x100(%rsp)
140015d7a:     	movaps	%xmm7, 0x110(%rsp)
140015d82:     	movaps	%xmm6, 0x120(%rsp)
140015d8a:     	movq	0x590(%rbp), %rdx
140015d91:     	testq	%rdx, %rdx
140015d94:     	je	0x140015dac <.text+0x14dac>
140015d96:     	movq	0x598(%rbp), %rcx
140015d9d:     	shlq	$0x2, %rdx
140015da1:     	movl	$0x4, %r8d
140015da7:     	callq	0x140613c20 <.text+0x612c20>
140015dac:     	movaps	0x120(%rsp), %xmm6
140015db4:     	movaps	0x110(%rsp), %xmm7
140015dbc:     	movaps	0x100(%rsp), %xmm8
140015dc5:     	movaps	0xf0(%rsp), %xmm9
140015dce:     	movaps	0xe0(%rsp), %xmm10
140015dd7:     	movaps	0xd0(%rsp), %xmm11
140015de0:     	movdqa	0xc0(%rsp), %xmm12
140015dea:     	movdqa	0xb0(%rsp), %xmm13
140015df4:     	movaps	0xa0(%rsp), %xmm14
140015dfd:     	movdqa	0x90(%rsp), %xmm15
140015e07:     	addq	$0x138, %rsp            # imm = 0x138
140015e0e:     	popq	%rbx
140015e0f:     	popq	%rdi
140015e10:     	popq	%rsi
140015e11:     	popq	%r12
140015e13:     	popq	%r13
140015e15:     	popq	%r14
140015e17:     	popq	%r15
140015e19:     	popq	%rbp
140015e1a:     	retq
140015e1b:     	nopl	(%rax,%rax)
140015e20:     	movq	%rdx, 0x10(%rsp)
140015e25:     	pushq	%rbp
140015e26:     	pushq	%r15
140015e28:     	pushq	%r14
140015e2a:     	pushq	%r13
140015e2c:     	pushq	%r12
140015e2e:     	pushq	%rsi
140015e2f:     	pushq	%rdi
140015e30:     	pushq	%rbx
140015e31:     	subq	$0x138, %rsp            # imm = 0x138
140015e38:     	leaq	0x80(%rdx), %rbp
140015e3f:     	movdqa	%xmm15, 0x90(%rsp)
140015e49:     	movaps	%xmm14, 0xa0(%rsp)
140015e52:     	movdqa	%xmm13, 0xb0(%rsp)
140015e5c:     	movdqa	%xmm12, 0xc0(%rsp)
140015e66:     	movaps	%xmm11, 0xd0(%rsp)
140015e6f:     	movaps	%xmm10, 0xe0(%rsp)
140015e78:     	movaps	%xmm9, 0xf0(%rsp)
140015e81:     	movaps	%xmm8, 0x100(%rsp)
140015e8a:     	movaps	%xmm7, 0x110(%rsp)
140015e92:     	movaps	%xmm6, 0x120(%rsp)
140015e9a:     	movq	0x500(%rbp), %rax
140015ea1:     	testq	%rax, %rax
140015ea4:     	je	0x140015ec0 <.text+0x14ec0>
140015ea6:     	movq	0x508(%rbp), %rcx
140015ead:     	shlq	$0x2, %rax
140015eb1:     	leaq	(%rax,%rax,2), %rdx
140015eb5:     	movl	$0x4, %r8d
140015ebb:     	callq	0x140613c20 <.text+0x612c20>
140015ec0:     	movaps	0x120(%rsp), %xmm6
140015ec8:     	movaps	0x110(%rsp), %xmm7
140015ed0:     	movaps	0x100(%rsp), %xmm8
140015ed9:     	movaps	0xf0(%rsp), %xmm9
140015ee2:     	movaps	0xe0(%rsp), %xmm10
140015eeb:     	movaps	0xd0(%rsp), %xmm11
140015ef4:     	movdqa	0xc0(%rsp), %xmm12
140015efe:     	movdqa	0xb0(%rsp), %xmm13
140015f08:     	movaps	0xa0(%rsp), %xmm14
140015f11:     	movdqa	0x90(%rsp), %xmm15
140015f1b:     	addq	$0x138, %rsp            # imm = 0x138
140015f22:     	popq	%rbx
140015f23:     	popq	%rdi
140015f24:     	popq	%rsi
140015f25:     	popq	%r12
140015f27:     	popq	%r13
140015f29:     	popq	%r14
140015f2b:     	popq	%r15
140015f2d:     	popq	%rbp
140015f2e:     	retq
140015f2f:     	nop
140015f30:     	movq	%rdx, 0x10(%rsp)
140015f35:     	pushq	%rbp
140015f36:     	pushq	%r15
140015f38:     	pushq	%r14
140015f3a:     	pushq	%r13
140015f3c:     	pushq	%r12
140015f3e:     	pushq	%rsi
140015f3f:     	pushq	%rdi
140015f40:     	pushq	%rbx
140015f41:     	subq	$0x138, %rsp            # imm = 0x138
140015f48:     	leaq	0x80(%rdx), %rbp
140015f4f:     	movdqa	%xmm15, 0x90(%rsp)
140015f59:     	movaps	%xmm14, 0xa0(%rsp)
140015f62:     	movdqa	%xmm13, 0xb0(%rsp)
140015f6c:     	movdqa	%xmm12, 0xc0(%rsp)
140015f76:     	movaps	%xmm11, 0xd0(%rsp)
140015f7f:     	movaps	%xmm10, 0xe0(%rsp)
140015f88:     	movaps	%xmm9, 0xf0(%rsp)
140015f91:     	movaps	%xmm8, 0x100(%rsp)
140015f9a:     	movaps	%xmm7, 0x110(%rsp)
140015fa2:     	movaps	%xmm6, 0x120(%rsp)
140015faa:     	movq	0x680(%rbp), %rdx
140015fb1:     	testq	%rdx, %rdx
140015fb4:     	je	0x140015fcc <.text+0x14fcc>
140015fb6:     	movq	0x688(%rbp), %rcx
140015fbd:     	shlq	$0x3, %rdx
140015fc1:     	movl	$0x4, %r8d
140015fc7:     	callq	0x140613c20 <.text+0x612c20>
140015fcc:     	movaps	0x120(%rsp), %xmm6
140015fd4:     	movaps	0x110(%rsp), %xmm7
140015fdc:     	movaps	0x100(%rsp), %xmm8
140015fe5:     	movaps	0xf0(%rsp), %xmm9
140015fee:     	movaps	0xe0(%rsp), %xmm10
140015ff7:     	movaps	0xd0(%rsp), %xmm11
140016000:     	movdqa	0xc0(%rsp), %xmm12
14001600a:     	movdqa	0xb0(%rsp), %xmm13
140016014:     	movaps	0xa0(%rsp), %xmm14
14001601d:     	movdqa	0x90(%rsp), %xmm15
140016027:     	addq	$0x138, %rsp            # imm = 0x138
14001602e:     	popq	%rbx
14001602f:     	popq	%rdi
140016030:     	popq	%rsi
140016031:     	popq	%r12
140016033:     	popq	%r13
140016035:     	popq	%r14
140016037:     	popq	%r15
140016039:     	popq	%rbp
14001603a:     	retq
14001603b:     	nopl	(%rax,%rax)
140016040:     	movq	%rdx, 0x10(%rsp)
140016045:     	pushq	%rbp
140016046:     	pushq	%r15
140016048:     	pushq	%r14
14001604a:     	pushq	%r13
14001604c:     	pushq	%r12
14001604e:     	pushq	%rsi
14001604f:     	pushq	%rdi
140016050:     	pushq	%rbx
140016051:     	subq	$0x138, %rsp            # imm = 0x138
140016058:     	leaq	0x80(%rdx), %rbp
14001605f:     	movdqa	%xmm15, 0x90(%rsp)
140016069:     	movaps	%xmm14, 0xa0(%rsp)
140016072:     	movdqa	%xmm13, 0xb0(%rsp)
14001607c:     	movdqa	%xmm12, 0xc0(%rsp)
140016086:     	movaps	%xmm11, 0xd0(%rsp)
14001608f:     	movaps	%xmm10, 0xe0(%rsp)
140016098:     	movaps	%xmm9, 0xf0(%rsp)
1400160a1:     	movaps	%xmm8, 0x100(%rsp)
1400160aa:     	movaps	%xmm7, 0x110(%rsp)
1400160b2:     	movaps	%xmm6, 0x120(%rsp)
1400160ba:     	movq	0x650(%rbp), %rdx
1400160c1:     	testq	%rdx, %rdx
1400160c4:     	je	0x1400160dc <.text+0x150dc>
1400160c6:     	movq	0x658(%rbp), %rcx
1400160cd:     	shlq	$0x2, %rdx
1400160d1:     	movl	$0x4, %r8d
1400160d7:     	callq	0x140613c20 <.text+0x612c20>
1400160dc:     	movaps	0x120(%rsp), %xmm6
1400160e4:     	movaps	0x110(%rsp), %xmm7
1400160ec:     	movaps	0x100(%rsp), %xmm8
1400160f5:     	movaps	0xf0(%rsp), %xmm9
1400160fe:     	movaps	0xe0(%rsp), %xmm10
140016107:     	movaps	0xd0(%rsp), %xmm11
140016110:     	movdqa	0xc0(%rsp), %xmm12
14001611a:     	movdqa	0xb0(%rsp), %xmm13
140016124:     	movaps	0xa0(%rsp), %xmm14
14001612d:     	movdqa	0x90(%rsp), %xmm15
140016137:     	addq	$0x138, %rsp            # imm = 0x138
14001613e:     	popq	%rbx
14001613f:     	popq	%rdi
140016140:     	popq	%rsi
140016141:     	popq	%r12
140016143:     	popq	%r13
140016145:     	popq	%r14
140016147:     	popq	%r15
140016149:     	popq	%rbp
14001614a:     	retq
14001614b:     	nopl	(%rax,%rax)
140016150:     	movq	%rdx, 0x10(%rsp)
140016155:     	pushq	%rbp
140016156:     	pushq	%r15
140016158:     	pushq	%r14
14001615a:     	pushq	%r13
14001615c:     	pushq	%r12
14001615e:     	pushq	%rsi
14001615f:     	pushq	%rdi
140016160:     	pushq	%rbx
140016161:     	subq	$0x138, %rsp            # imm = 0x138
140016168:     	leaq	0x80(%rdx), %rbp
14001616f:     	movdqa	%xmm15, 0x90(%rsp)
140016179:     	movaps	%xmm14, 0xa0(%rsp)
140016182:     	movdqa	%xmm13, 0xb0(%rsp)
14001618c:     	movdqa	%xmm12, 0xc0(%rsp)
140016196:     	movaps	%xmm11, 0xd0(%rsp)
14001619f:     	movaps	%xmm10, 0xe0(%rsp)
1400161a8:     	movaps	%xmm9, 0xf0(%rsp)
1400161b1:     	movaps	%xmm8, 0x100(%rsp)
1400161ba:     	movaps	%xmm7, 0x110(%rsp)
1400161c2:     	movaps	%xmm6, 0x120(%rsp)
1400161ca:     	leaq	0x460(%rbp), %rcx
1400161d1:     	callq	0x14014ba20 <.text+0x14aa20>
1400161d6:     	movaps	0x120(%rsp), %xmm6
1400161de:     	movaps	0x110(%rsp), %xmm7
1400161e6:     	movaps	0x100(%rsp), %xmm8
1400161ef:     	movaps	0xf0(%rsp), %xmm9
1400161f8:     	movaps	0xe0(%rsp), %xmm10
140016201:     	movaps	0xd0(%rsp), %xmm11
14001620a:     	movdqa	0xc0(%rsp), %xmm12
140016214:     	movdqa	0xb0(%rsp), %xmm13
14001621e:     	movaps	0xa0(%rsp), %xmm14
140016227:     	movdqa	0x90(%rsp), %xmm15
140016231:     	addq	$0x138, %rsp            # imm = 0x138
140016238:     	popq	%rbx
140016239:     	popq	%rdi
14001623a:     	popq	%rsi
14001623b:     	popq	%r12
14001623d:     	popq	%r13
14001623f:     	popq	%r14
140016241:     	popq	%r15
140016243:     	popq	%rbp
140016244:     	retq
140016245:     	nopw	%cs:(%rax,%rax)
140016250:     	movq	%rdx, 0x10(%rsp)
140016255:     	pushq	%rbp
140016256:     	pushq	%r15
140016258:     	pushq	%r14
14001625a:     	pushq	%r13
14001625c:     	pushq	%r12
14001625e:     	pushq	%rsi
14001625f:     	pushq	%rdi
140016260:     	pushq	%rbx
140016261:     	subq	$0x138, %rsp            # imm = 0x138
140016268:     	leaq	0x80(%rdx), %rbp
14001626f:     	movdqa	%xmm15, 0x90(%rsp)
140016279:     	movaps	%xmm14, 0xa0(%rsp)
140016282:     	movdqa	%xmm13, 0xb0(%rsp)
14001628c:     	movdqa	%xmm12, 0xc0(%rsp)
140016296:     	movaps	%xmm11, 0xd0(%rsp)
14001629f:     	movaps	%xmm10, 0xe0(%rsp)
1400162a8:     	movaps	%xmm9, 0xf0(%rsp)
1400162b1:     	movaps	%xmm8, 0x100(%rsp)
1400162ba:     	movaps	%xmm7, 0x110(%rsp)
1400162c2:     	movaps	%xmm6, 0x120(%rsp)
1400162ca:     	movb	$0x1, 0x817(%rbp)
1400162d1:     	leaq	0x380(%rbp), %rcx
1400162d8:     	callq	0x14014ba20 <.text+0x14aa20>
1400162dd:     	movb	$0x1, 0x817(%rbp)
1400162e4:     	movaps	0x120(%rsp), %xmm6
1400162ec:     	movaps	0x110(%rsp), %xmm7
1400162f4:     	movaps	0x100(%rsp), %xmm8
1400162fd:     	movaps	0xf0(%rsp), %xmm9
140016306:     	movaps	0xe0(%rsp), %xmm10
14001630f:     	movaps	0xd0(%rsp), %xmm11
140016318:     	movdqa	0xc0(%rsp), %xmm12
140016322:     	movdqa	0xb0(%rsp), %xmm13
14001632c:     	movaps	0xa0(%rsp), %xmm14
140016335:     	movdqa	0x90(%rsp), %xmm15
14001633f:     	addq	$0x138, %rsp            # imm = 0x138
140016346:     	popq	%rbx
140016347:     	popq	%rdi
140016348:     	popq	%rsi
140016349:     	popq	%r12
14001634b:     	popq	%r13
14001634d:     	popq	%r14
14001634f:     	popq	%r15
140016351:     	popq	%rbp
140016352:     	retq
140016353:     	nopw	%cs:(%rax,%rax)
140016360:     	movq	%rdx, 0x10(%rsp)
140016365:     	pushq	%rbp
140016366:     	pushq	%r15
140016368:     	pushq	%r14
14001636a:     	pushq	%r13
14001636c:     	pushq	%r12
14001636e:     	pushq	%rsi
14001636f:     	pushq	%rdi
140016370:     	pushq	%rbx
140016371:     	subq	$0x138, %rsp            # imm = 0x138
140016378:     	leaq	0x80(%rdx), %rbp
14001637f:     	movdqa	%xmm15, 0x90(%rsp)
140016389:     	movaps	%xmm14, 0xa0(%rsp)
140016392:     	movdqa	%xmm13, 0xb0(%rsp)
14001639c:     	movdqa	%xmm12, 0xc0(%rsp)
1400163a6:     	movaps	%xmm11, 0xd0(%rsp)
1400163af:     	movaps	%xmm10, 0xe0(%rsp)
1400163b8:     	movaps	%xmm9, 0xf0(%rsp)
1400163c1:     	movaps	%xmm8, 0x100(%rsp)
1400163ca:     	movaps	%xmm7, 0x110(%rsp)
1400163d2:     	movaps	%xmm6, 0x120(%rsp)
1400163da:     	movzbl	0x815(%rbp), %eax
1400163e1:     	movzbl	0x816(%rbp), %ecx
1400163e8:     	movb	%al, 0x800(%rbp)
1400163ee:     	testb	%al, %al
1400163f0:     	movb	%cl, 0x780(%rbp)
1400163f6:     	je	0x14001648a <.text+0x1548a>
1400163fc:     	movq	0x798(%rbp), %rdx
140016403:     	testq	%rdx, %rdx
140016406:     	je	0x14001641e <.text+0x1541e>
140016408:     	shlq	$0x2, %rdx
14001640c:     	movl	$0x4, %r8d
140016412:     	movq	0x7f0(%rbp), %rcx
140016419:     	callq	0x140613c20 <.text+0x612c20>
14001641e:     	movaps	0x120(%rsp), %xmm6
140016426:     	movaps	0x110(%rsp), %xmm7
14001642e:     	movaps	0x100(%rsp), %xmm8
140016437:     	movaps	0xf0(%rsp), %xmm9
140016440:     	movaps	0xe0(%rsp), %xmm10
140016449:     	movaps	0xd0(%rsp), %xmm11
140016452:     	movaps	0xc0(%rsp), %xmm12
14001645b:     	movaps	0xb0(%rsp), %xmm13
140016464:     	movaps	0xa0(%rsp), %xmm14
14001646d:     	movaps	0x90(%rsp), %xmm15
140016476:     	addq	$0x138, %rsp            # imm = 0x138
14001647d:     	popq	%rbx
14001647e:     	popq	%rdi
14001647f:     	popq	%rsi
140016480:     	popq	%r12
140016482:     	popq	%r13
140016484:     	popq	%r14
140016486:     	popq	%r15
140016488:     	popq	%rbp
140016489:     	retq
14001648a:     	movaps	0x120(%rsp), %xmm6
140016492:     	movaps	0x110(%rsp), %xmm7
14001649a:     	movaps	0x100(%rsp), %xmm8
1400164a3:     	movaps	0xf0(%rsp), %xmm9
1400164ac:     	movaps	0xe0(%rsp), %xmm10
1400164b5:     	movaps	0xd0(%rsp), %xmm11
1400164be:     	movaps	0xc0(%rsp), %xmm12
1400164c7:     	movaps	0xb0(%rsp), %xmm13
1400164d0:     	movaps	0xa0(%rsp), %xmm14
1400164d9:     	movaps	0x90(%rsp), %xmm15
1400164e2:     	addq	$0x138, %rsp            # imm = 0x138
1400164e9:     	popq	%rbx
1400164ea:     	popq	%rdi
1400164eb:     	popq	%rsi
1400164ec:     	popq	%r12
1400164ee:     	popq	%r13
1400164f0:     	popq	%r14
1400164f2:     	popq	%r15
1400164f4:     	popq	%rbp
1400164f5:     	retq
1400164f6:     	nopw	%cs:(%rax,%rax)
140016500:     	movq	%rdx, 0x10(%rsp)
140016505:     	pushq	%rbp
140016506:     	pushq	%r15
140016508:     	pushq	%r14
14001650a:     	pushq	%r13
14001650c:     	pushq	%r12
14001650e:     	pushq	%rsi
14001650f:     	pushq	%rdi
140016510:     	pushq	%rbx
140016511:     	subq	$0x138, %rsp            # imm = 0x138
140016518:     	leaq	0x80(%rdx), %rbp
14001651f:     	movaps	%xmm15, 0x90(%rsp)
140016528:     	movaps	%xmm14, 0xa0(%rsp)
140016531:     	movaps	%xmm13, 0xb0(%rsp)
14001653a:     	movaps	%xmm12, 0xc0(%rsp)
140016543:     	movaps	%xmm11, 0xd0(%rsp)
14001654c:     	movaps	%xmm10, 0xe0(%rsp)
140016555:     	movaps	%xmm9, 0xf0(%rsp)
14001655e:     	movaps	%xmm8, 0x100(%rsp)
140016567:     	movaps	%xmm7, 0x110(%rsp)
14001656f:     	movaps	%xmm6, 0x120(%rsp)
140016577:     	testb	$0x1, 0x780(%rbp)
14001657e:     	je	0x140016616 <.text+0x15616>
140016584:     	movq	0x680(%rbp), %rax
14001658b:     	testq	%rax, %rax
14001658e:     	je	0x1400165aa <.text+0x155aa>
140016590:     	movq	0x688(%rbp), %rcx
140016597:     	shlq	$0x2, %rax
14001659b:     	leaq	(%rax,%rax,2), %rdx
14001659f:     	movl	$0x4, %r8d
1400165a5:     	callq	0x140613c20 <.text+0x612c20>
1400165aa:     	movaps	0x120(%rsp), %xmm6
1400165b2:     	movaps	0x110(%rsp), %xmm7
1400165ba:     	movaps	0x100(%rsp), %xmm8
1400165c3:     	movaps	0xf0(%rsp), %xmm9
1400165cc:     	movaps	0xe0(%rsp), %xmm10
1400165d5:     	movaps	0xd0(%rsp), %xmm11
1400165de:     	movaps	0xc0(%rsp), %xmm12
1400165e7:     	movaps	0xb0(%rsp), %xmm13
1400165f0:     	movaps	0xa0(%rsp), %xmm14
1400165f9:     	movaps	0x90(%rsp), %xmm15
140016602:     	addq	$0x138, %rsp            # imm = 0x138
140016609:     	popq	%rbx
14001660a:     	popq	%rdi
14001660b:     	popq	%rsi
14001660c:     	popq	%r12
14001660e:     	popq	%r13
140016610:     	popq	%r14
140016612:     	popq	%r15
140016614:     	popq	%rbp
140016615:     	retq
140016616:     	movaps	0x120(%rsp), %xmm6
14001661e:     	movaps	0x110(%rsp), %xmm7
140016626:     	movaps	0x100(%rsp), %xmm8
14001662f:     	movaps	0xf0(%rsp), %xmm9
140016638:     	movaps	0xe0(%rsp), %xmm10
140016641:     	movaps	0xd0(%rsp), %xmm11
14001664a:     	movaps	0xc0(%rsp), %xmm12
140016653:     	movaps	0xb0(%rsp), %xmm13
14001665c:     	movaps	0xa0(%rsp), %xmm14
140016665:     	movaps	0x90(%rsp), %xmm15
14001666e:     	addq	$0x138, %rsp            # imm = 0x138
140016675:     	popq	%rbx
140016676:     	popq	%rdi
140016677:     	popq	%rsi
140016678:     	popq	%r12
14001667a:     	popq	%r13
14001667c:     	popq	%r14
14001667e:     	popq	%r15
140016680:     	popq	%rbp
140016681:     	retq
140016682:     	nopw	%cs:(%rax,%rax)
140016690:     	movq	%rdx, 0x10(%rsp)
140016695:     	pushq	%rbp
140016696:     	pushq	%r15
140016698:     	pushq	%r14
14001669a:     	pushq	%r13
14001669c:     	pushq	%r12
14001669e:     	pushq	%rsi
14001669f:     	pushq	%rdi
1400166a0:     	pushq	%rbx
1400166a1:     	subq	$0x138, %rsp            # imm = 0x138
1400166a8:     	leaq	0x80(%rdx), %rbp
1400166af:     	movaps	%xmm15, 0x90(%rsp)
1400166b8:     	movaps	%xmm14, 0xa0(%rsp)
1400166c1:     	movaps	%xmm13, 0xb0(%rsp)
1400166ca:     	movaps	%xmm12, 0xc0(%rsp)
1400166d3:     	movaps	%xmm11, 0xd0(%rsp)
1400166dc:     	movaps	%xmm10, 0xe0(%rsp)
1400166e5:     	movaps	%xmm9, 0xf0(%rsp)
1400166ee:     	movaps	%xmm8, 0x100(%rsp)
1400166f7:     	movaps	%xmm7, 0x110(%rsp)
1400166ff:     	movaps	%xmm6, 0x120(%rsp)
140016707:     	cmpb	$0x0, 0x800(%rbp)
14001670e:     	je	0x1400167a2 <.text+0x157a2>
140016714:     	movq	0x650(%rbp), %rdx
14001671b:     	testq	%rdx, %rdx
14001671e:     	je	0x140016736 <.text+0x15736>
140016720:     	movq	0x658(%rbp), %rcx
140016727:     	shlq	$0x3, %rdx
14001672b:     	movl	$0x4, %r8d
140016731:     	callq	0x140613c20 <.text+0x612c20>
140016736:     	movaps	0x120(%rsp), %xmm6
14001673e:     	movaps	0x110(%rsp), %xmm7
140016746:     	movaps	0x100(%rsp), %xmm8
14001674f:     	movaps	0xf0(%rsp), %xmm9
140016758:     	movaps	0xe0(%rsp), %xmm10
140016761:     	movaps	0xd0(%rsp), %xmm11
14001676a:     	movaps	0xc0(%rsp), %xmm12
140016773:     	movaps	0xb0(%rsp), %xmm13
14001677c:     	movaps	0xa0(%rsp), %xmm14
140016785:     	movaps	0x90(%rsp), %xmm15
14001678e:     	addq	$0x138, %rsp            # imm = 0x138
140016795:     	popq	%rbx
140016796:     	popq	%rdi
140016797:     	popq	%rsi
140016798:     	popq	%r12
14001679a:     	popq	%r13
14001679c:     	popq	%r14
14001679e:     	popq	%r15
1400167a0:     	popq	%rbp
1400167a1:     	retq
1400167a2:     	movaps	0x120(%rsp), %xmm6
1400167aa:     	movaps	0x110(%rsp), %xmm7
1400167b2:     	movaps	0x100(%rsp), %xmm8
1400167bb:     	movaps	0xf0(%rsp), %xmm9
1400167c4:     	movaps	0xe0(%rsp), %xmm10
1400167cd:     	movaps	0xd0(%rsp), %xmm11
1400167d6:     	movaps	0xc0(%rsp), %xmm12
1400167df:     	movaps	0xb0(%rsp), %xmm13
1400167e8:     	movaps	0xa0(%rsp), %xmm14
1400167f1:     	movaps	0x90(%rsp), %xmm15
1400167fa:     	addq	$0x138, %rsp            # imm = 0x138
140016801:     	popq	%rbx
140016802:     	popq	%rdi
140016803:     	popq	%rsi
140016804:     	popq	%r12
140016806:     	popq	%r13
140016808:     	popq	%r14
14001680a:     	popq	%r15
14001680c:     	popq	%rbp
14001680d:     	retq
14001680e:     	nop
140016810:     	movq	%rdx, 0x10(%rsp)
140016815:     	pushq	%rbp
140016816:     	pushq	%r15
140016818:     	pushq	%r14
14001681a:     	pushq	%r13
14001681c:     	pushq	%r12
14001681e:     	pushq	%rsi
14001681f:     	pushq	%rdi
140016820:     	pushq	%rbx
140016821:     	subq	$0x138, %rsp            # imm = 0x138
140016828:     	leaq	0x80(%rdx), %rbp
14001682f:     	movaps	%xmm15, 0x90(%rsp)
140016838:     	movaps	%xmm14, 0xa0(%rsp)
140016841:     	movaps	%xmm13, 0xb0(%rsp)
14001684a:     	movaps	%xmm12, 0xc0(%rsp)
140016853:     	movaps	%xmm11, 0xd0(%rsp)
14001685c:     	movaps	%xmm10, 0xe0(%rsp)
140016865:     	movaps	%xmm9, 0xf0(%rsp)
14001686e:     	movaps	%xmm8, 0x100(%rsp)
140016877:     	movaps	%xmm7, 0x110(%rsp)
14001687f:     	movaps	%xmm6, 0x120(%rsp)
140016887:     	cmpb	$0x0, 0x800(%rbp)
14001688e:     	je	0x14001692c <.text+0x1592c>
140016894:     	movq	0x548(%rbp), %rdx
14001689b:     	testq	%rdx, %rdx
14001689e:     	je	0x1400168b6 <.text+0x158b6>
1400168a0:     	movq	0x550(%rbp), %rcx
1400168a7:     	shlq	$0x2, %rdx
1400168ab:     	movl	$0x4, %r8d
1400168b1:     	callq	0x140613c20 <.text+0x612c20>
1400168b6:     	movb	$0x1, 0x817(%rbp)
1400168bd:     	movaps	0x120(%rsp), %xmm6
1400168c5:     	movaps	0x110(%rsp), %xmm7
1400168cd:     	movaps	0x100(%rsp), %xmm8
1400168d6:     	movaps	0xf0(%rsp), %xmm9
1400168df:     	movaps	0xe0(%rsp), %xmm10
1400168e8:     	movaps	0xd0(%rsp), %xmm11
1400168f1:     	movdqa	0xc0(%rsp), %xmm12
1400168fb:     	movdqa	0xb0(%rsp), %xmm13
140016905:     	movaps	0xa0(%rsp), %xmm14
14001690e:     	movdqa	0x90(%rsp), %xmm15
140016918:     	addq	$0x138, %rsp            # imm = 0x138
14001691f:     	popq	%rbx
140016920:     	popq	%rdi
140016921:     	popq	%rsi
140016922:     	popq	%r12
140016924:     	popq	%r13
140016926:     	popq	%r14
140016928:     	popq	%r15
14001692a:     	popq	%rbp
14001692b:     	retq
14001692c:     	movb	$0x1, 0x817(%rbp)
140016933:     	movaps	0x120(%rsp), %xmm6
14001693b:     	movaps	0x110(%rsp), %xmm7
140016943:     	movaps	0x100(%rsp), %xmm8
14001694c:     	movaps	0xf0(%rsp), %xmm9
140016955:     	movaps	0xe0(%rsp), %xmm10
14001695e:     	movaps	0xd0(%rsp), %xmm11
140016967:     	movdqa	0xc0(%rsp), %xmm12
140016971:     	movdqa	0xb0(%rsp), %xmm13
14001697b:     	movaps	0xa0(%rsp), %xmm14
140016984:     	movdqa	0x90(%rsp), %xmm15
14001698e:     	addq	$0x138, %rsp            # imm = 0x138
140016995:     	popq	%rbx
140016996:     	popq	%rdi
140016997:     	popq	%rsi
140016998:     	popq	%r12
14001699a:     	popq	%r13
14001699c:     	popq	%r14
14001699e:     	popq	%r15
1400169a0:     	popq	%rbp
1400169a1:     	retq
1400169a2:     	nopw	%cs:(%rax,%rax)
1400169b0:     	movq	%rdx, 0x10(%rsp)
1400169b5:     	pushq	%rbp
1400169b6:     	pushq	%r15
1400169b8:     	pushq	%r14
1400169ba:     	pushq	%r13
1400169bc:     	pushq	%r12
1400169be:     	pushq	%rsi
1400169bf:     	pushq	%rdi
1400169c0:     	pushq	%rbx
1400169c1:     	subq	$0x138, %rsp            # imm = 0x138
1400169c8:     	leaq	0x80(%rdx), %rbp
1400169cf:     	movdqa	%xmm15, 0x90(%rsp)
1400169d9:     	movaps	%xmm14, 0xa0(%rsp)
1400169e2:     	movdqa	%xmm13, 0xb0(%rsp)
1400169ec:     	movdqa	%xmm12, 0xc0(%rsp)
1400169f6:     	movaps	%xmm11, 0xd0(%rsp)
1400169ff:     	movaps	%xmm10, 0xe0(%rsp)
140016a08:     	movaps	%xmm9, 0xf0(%rsp)
140016a11:     	movaps	%xmm8, 0x100(%rsp)
140016a1a:     	movaps	%xmm7, 0x110(%rsp)
140016a22:     	movaps	%xmm6, 0x120(%rsp)
140016a2a:     	cmpb	$0x0, 0x813(%rbp)
140016a31:     	je	0x140016ac5 <.text+0x15ac5>
140016a37:     	movq	0x668(%rbp), %rdx
140016a3e:     	testq	%rdx, %rdx
140016a41:     	je	0x140016a59 <.text+0x15a59>
140016a43:     	shlq	$0x2, %rdx
140016a47:     	movl	$0x4, %r8d
140016a4d:     	movq	0x7a8(%rbp), %rcx
140016a54:     	callq	0x140613c20 <.text+0x612c20>
140016a59:     	movaps	0x120(%rsp), %xmm6
140016a61:     	movaps	0x110(%rsp), %xmm7
140016a69:     	movaps	0x100(%rsp), %xmm8
140016a72:     	movaps	0xf0(%rsp), %xmm9
140016a7b:     	movaps	0xe0(%rsp), %xmm10
140016a84:     	movaps	0xd0(%rsp), %xmm11
140016a8d:     	movaps	0xc0(%rsp), %xmm12
140016a96:     	movaps	0xb0(%rsp), %xmm13
140016a9f:     	movaps	0xa0(%rsp), %xmm14
140016aa8:     	movaps	0x90(%rsp), %xmm15
140016ab1:     	addq	$0x138, %rsp            # imm = 0x138
140016ab8:     	popq	%rbx
140016ab9:     	popq	%rdi
140016aba:     	popq	%rsi
140016abb:     	popq	%r12
140016abd:     	popq	%r13
140016abf:     	popq	%r14
140016ac1:     	popq	%r15
140016ac3:     	popq	%rbp
140016ac4:     	retq
140016ac5:     	movaps	0x120(%rsp), %xmm6
140016acd:     	movaps	0x110(%rsp), %xmm7
140016ad5:     	movaps	0x100(%rsp), %xmm8
140016ade:     	movaps	0xf0(%rsp), %xmm9
140016ae7:     	movaps	0xe0(%rsp), %xmm10
140016af0:     	movaps	0xd0(%rsp), %xmm11
140016af9:     	movaps	0xc0(%rsp), %xmm12
140016b02:     	movaps	0xb0(%rsp), %xmm13
140016b0b:     	movaps	0xa0(%rsp), %xmm14
140016b14:     	movaps	0x90(%rsp), %xmm15
140016b1d:     	addq	$0x138, %rsp            # imm = 0x138
140016b24:     	popq	%rbx
140016b25:     	popq	%rdi
140016b26:     	popq	%rsi
140016b27:     	popq	%r12
140016b29:     	popq	%r13
140016b2b:     	popq	%r14
140016b2d:     	popq	%r15
140016b2f:     	popq	%rbp
140016b30:     	retq
140016b31:     	nopw	%cs:(%rax,%rax)
140016b40:     	movq	%rdx, 0x10(%rsp)
140016b45:     	pushq	%rbp
140016b46:     	pushq	%r15
140016b48:     	pushq	%r14
140016b4a:     	pushq	%r13
140016b4c:     	pushq	%r12
140016b4e:     	pushq	%rsi
140016b4f:     	pushq	%rdi
140016b50:     	pushq	%rbx
140016b51:     	subq	$0x138, %rsp            # imm = 0x138
140016b58:     	leaq	0x80(%rdx), %rbp
140016b5f:     	movaps	%xmm15, 0x90(%rsp)
140016b68:     	movaps	%xmm14, 0xa0(%rsp)
140016b71:     	movaps	%xmm13, 0xb0(%rsp)
140016b7a:     	movaps	%xmm12, 0xc0(%rsp)
140016b83:     	movaps	%xmm11, 0xd0(%rsp)
140016b8c:     	movaps	%xmm10, 0xe0(%rsp)
140016b95:     	movaps	%xmm9, 0xf0(%rsp)
140016b9e:     	movaps	%xmm8, 0x100(%rsp)
140016ba7:     	movaps	%xmm7, 0x110(%rsp)
140016baf:     	movaps	%xmm6, 0x120(%rsp)
140016bb7:     	leaq	0x380(%rbp), %rcx
140016bbe:     	callq	0x1405e16f0 <.text+0x5e06f0>
140016bc3:     	movaps	0x120(%rsp), %xmm6
140016bcb:     	movaps	0x110(%rsp), %xmm7
140016bd3:     	movaps	0x100(%rsp), %xmm8
140016bdc:     	movaps	0xf0(%rsp), %xmm9
140016be5:     	movaps	0xe0(%rsp), %xmm10
140016bee:     	movaps	0xd0(%rsp), %xmm11
140016bf7:     	movdqa	0xc0(%rsp), %xmm12
140016c01:     	movdqa	0xb0(%rsp), %xmm13
140016c0b:     	movaps	0xa0(%rsp), %xmm14
140016c14:     	movdqa	0x90(%rsp), %xmm15
140016c1e:     	addq	$0x138, %rsp            # imm = 0x138
140016c25:     	popq	%rbx
140016c26:     	popq	%rdi
140016c27:     	popq	%rsi
140016c28:     	popq	%r12
140016c2a:     	popq	%r13
140016c2c:     	popq	%r14
140016c2e:     	popq	%r15
140016c30:     	popq	%rbp
140016c31:     	retq
140016c32:     	nopw	%cs:(%rax,%rax)
140016c40:     	movq	%rdx, 0x10(%rsp)
140016c45:     	pushq	%rbp
140016c46:     	pushq	%r15
140016c48:     	pushq	%r14
140016c4a:     	pushq	%r13
140016c4c:     	pushq	%r12
140016c4e:     	pushq	%rsi
140016c4f:     	pushq	%rdi
140016c50:     	pushq	%rbx
140016c51:     	subq	$0x138, %rsp            # imm = 0x138
140016c58:     	leaq	0x80(%rdx), %rbp
140016c5f:     	movdqa	%xmm15, 0x90(%rsp)
140016c69:     	movaps	%xmm14, 0xa0(%rsp)
140016c72:     	movdqa	%xmm13, 0xb0(%rsp)
140016c7c:     	movdqa	%xmm12, 0xc0(%rsp)
140016c86:     	movaps	%xmm11, 0xd0(%rsp)
140016c8f:     	movaps	%xmm10, 0xe0(%rsp)
140016c98:     	movaps	%xmm9, 0xf0(%rsp)
140016ca1:     	movaps	%xmm8, 0x100(%rsp)
140016caa:     	movaps	%xmm7, 0x110(%rsp)
140016cb2:     	movaps	%xmm6, 0x120(%rsp)
140016cba:     	leaq	0x6a0(%rbp), %rcx
140016cc1:     	callq	0x1405e1b90 <.text+0x5e0b90>
140016cc6:     	movaps	0x120(%rsp), %xmm6
140016cce:     	movaps	0x110(%rsp), %xmm7
140016cd6:     	movaps	0x100(%rsp), %xmm8
140016cdf:     	movaps	0xf0(%rsp), %xmm9
140016ce8:     	movaps	0xe0(%rsp), %xmm10
140016cf1:     	movaps	0xd0(%rsp), %xmm11
140016cfa:     	movdqa	0xc0(%rsp), %xmm12
140016d04:     	movdqa	0xb0(%rsp), %xmm13
140016d0e:     	movaps	0xa0(%rsp), %xmm14
140016d17:     	movdqa	0x90(%rsp), %xmm15
140016d21:     	addq	$0x138, %rsp            # imm = 0x138
140016d28:     	popq	%rbx
140016d29:     	popq	%rdi
140016d2a:     	popq	%rsi
140016d2b:     	popq	%r12
140016d2d:     	popq	%r13
140016d2f:     	popq	%r14
140016d31:     	popq	%r15
140016d33:     	popq	%rbp
140016d34:     	retq
140016d35:     	nopw	%cs:(%rax,%rax)
140016d40:     	movq	%rdx, 0x10(%rsp)
140016d45:     	pushq	%rbp
140016d46:     	pushq	%r15
140016d48:     	pushq	%r14
140016d4a:     	pushq	%r13
140016d4c:     	pushq	%r12
140016d4e:     	pushq	%rsi
140016d4f:     	pushq	%rdi
140016d50:     	pushq	%rbx
140016d51:     	subq	$0x138, %rsp            # imm = 0x138
140016d58:     	leaq	0x80(%rdx), %rbp
140016d5f:     	movdqa	%xmm15, 0x90(%rsp)
140016d69:     	movaps	%xmm14, 0xa0(%rsp)
140016d72:     	movdqa	%xmm13, 0xb0(%rsp)
140016d7c:     	movdqa	%xmm12, 0xc0(%rsp)
140016d86:     	movaps	%xmm11, 0xd0(%rsp)
140016d8f:     	movaps	%xmm10, 0xe0(%rsp)
140016d98:     	movaps	%xmm9, 0xf0(%rsp)
140016da1:     	movaps	%xmm8, 0x100(%rsp)
140016daa:     	movaps	%xmm7, 0x110(%rsp)
140016db2:     	movaps	%xmm6, 0x120(%rsp)
140016dba:     	cmpq	$0x0, 0x668(%rbp)
140016dc2:     	movq	0x7a0(%rbp), %rdx
140016dc9:     	je	0x140016de3 <.text+0x15de3>
140016dcb:     	testq	%rdx, %rdx
140016dce:     	je	0x140016de3 <.text+0x15de3>
140016dd0:     	movq	0x670(%rbp), %rcx
140016dd7:     	movq	0x7a8(%rbp), %r8
140016dde:     	callq	0x140613c20 <.text+0x612c20>
140016de3:     	movb	$0x0, 0x7e9(%rbp)
140016dea:     	movaps	0x120(%rsp), %xmm6
140016df2:     	movaps	0x110(%rsp), %xmm7
140016dfa:     	movaps	0x100(%rsp), %xmm8
140016e03:     	movaps	0xf0(%rsp), %xmm9
140016e0c:     	movaps	0xe0(%rsp), %xmm10
140016e15:     	movaps	0xd0(%rsp), %xmm11
140016e1e:     	movdqa	0xc0(%rsp), %xmm12
140016e28:     	movdqa	0xb0(%rsp), %xmm13
140016e32:     	movaps	0xa0(%rsp), %xmm14
140016e3b:     	movdqa	0x90(%rsp), %xmm15
140016e45:     	addq	$0x138, %rsp            # imm = 0x138
140016e4c:     	popq	%rbx
140016e4d:     	popq	%rdi
140016e4e:     	popq	%rsi
140016e4f:     	popq	%r12
140016e51:     	popq	%r13
140016e53:     	popq	%r14
140016e55:     	popq	%r15
140016e57:     	popq	%rbp
140016e58:     	retq
140016e59:     	nopl	(%rax)
140016e60:     	movq	%rdx, 0x10(%rsp)
140016e65:     	pushq	%rbp
140016e66:     	pushq	%r15
140016e68:     	pushq	%r14
140016e6a:     	pushq	%r13
140016e6c:     	pushq	%r12
140016e6e:     	pushq	%rsi
140016e6f:     	pushq	%rdi
140016e70:     	pushq	%rbx
140016e71:     	subq	$0x138, %rsp            # imm = 0x138
140016e78:     	leaq	0x80(%rdx), %rbp
140016e7f:     	movdqa	%xmm15, 0x90(%rsp)
140016e89:     	movaps	%xmm14, 0xa0(%rsp)
140016e92:     	movdqa	%xmm13, 0xb0(%rsp)
140016e9c:     	movdqa	%xmm12, 0xc0(%rsp)
140016ea6:     	movaps	%xmm11, 0xd0(%rsp)
140016eaf:     	movaps	%xmm10, 0xe0(%rsp)
140016eb8:     	movaps	%xmm9, 0xf0(%rsp)
140016ec1:     	movaps	%xmm8, 0x100(%rsp)
140016eca:     	movaps	%xmm7, 0x110(%rsp)
140016ed2:     	movaps	%xmm6, 0x120(%rsp)
140016eda:     	cmpb	$0x0, 0x7e9(%rbp)
140016ee1:     	je	0x140016f69 <.text+0x15f69>
140016ee7:     	leaq	0x460(%rbp), %rcx
140016eee:     	callq	0x140149f50 <.text+0x148f50>
140016ef3:     	movb	$0x1, 0x7ff(%rbp)
140016efa:     	movaps	0x120(%rsp), %xmm6
140016f02:     	movaps	0x110(%rsp), %xmm7
140016f0a:     	movaps	0x100(%rsp), %xmm8
140016f13:     	movaps	0xf0(%rsp), %xmm9
140016f1c:     	movaps	0xe0(%rsp), %xmm10
140016f25:     	movaps	0xd0(%rsp), %xmm11
140016f2e:     	movdqa	0xc0(%rsp), %xmm12
140016f38:     	movdqa	0xb0(%rsp), %xmm13
140016f42:     	movaps	0xa0(%rsp), %xmm14
140016f4b:     	movdqa	0x90(%rsp), %xmm15
140016f55:     	addq	$0x138, %rsp            # imm = 0x138
140016f5c:     	popq	%rbx
140016f5d:     	popq	%rdi
140016f5e:     	popq	%rsi
140016f5f:     	popq	%r12
140016f61:     	popq	%r13
140016f63:     	popq	%r14
140016f65:     	popq	%r15
140016f67:     	popq	%rbp
140016f68:     	retq
140016f69:     	movb	$0x1, 0x7ff(%rbp)
140016f70:     	movaps	0x120(%rsp), %xmm6
140016f78:     	movaps	0x110(%rsp), %xmm7
140016f80:     	movaps	0x100(%rsp), %xmm8
140016f89:     	movaps	0xf0(%rsp), %xmm9
140016f92:     	movaps	0xe0(%rsp), %xmm10
140016f9b:     	movaps	0xd0(%rsp), %xmm11
140016fa4:     	movdqa	0xc0(%rsp), %xmm12
140016fae:     	movdqa	0xb0(%rsp), %xmm13
140016fb8:     	movaps	0xa0(%rsp), %xmm14
140016fc1:     	movdqa	0x90(%rsp), %xmm15
140016fcb:     	addq	$0x138, %rsp            # imm = 0x138
140016fd2:     	popq	%rbx
140016fd3:     	popq	%rdi
140016fd4:     	popq	%rsi
140016fd5:     	popq	%r12
140016fd7:     	popq	%r13
140016fd9:     	popq	%r14
140016fdb:     	popq	%r15
140016fdd:     	popq	%rbp
140016fde:     	retq
140016fdf:     	nop
140016fe0:     	movq	%rdx, 0x10(%rsp)
140016fe5:     	pushq	%rbp
140016fe6:     	pushq	%r15
140016fe8:     	pushq	%r14
140016fea:     	pushq	%r13
140016fec:     	pushq	%r12
140016fee:     	pushq	%rsi
140016fef:     	pushq	%rdi
140016ff0:     	pushq	%rbx
140016ff1:     	subq	$0x138, %rsp            # imm = 0x138
140016ff8:     	leaq	0x80(%rdx), %rbp
140016fff:     	movdqa	%xmm15, 0x90(%rsp)
140017009:     	movaps	%xmm14, 0xa0(%rsp)
140017012:     	movdqa	%xmm13, 0xb0(%rsp)
14001701c:     	movdqa	%xmm12, 0xc0(%rsp)
140017026:     	movaps	%xmm11, 0xd0(%rsp)
14001702f:     	movaps	%xmm10, 0xe0(%rsp)
140017038:     	movaps	%xmm9, 0xf0(%rsp)
140017041:     	movaps	%xmm8, 0x100(%rsp)
14001704a:     	movaps	%xmm7, 0x110(%rsp)
140017052:     	movaps	%xmm6, 0x120(%rsp)
14001705a:     	movq	0x7a8(%rbp), %rdx
140017061:     	testq	%rdx, %rdx
140017064:     	je	0x14001707c <.text+0x1607c>
140017066:     	shlq	$0x4, %rdx
14001706a:     	movl	$0x8, %r8d
140017070:     	movq	0x798(%rbp), %rcx
140017077:     	callq	0x140613c20 <.text+0x612c20>
14001707c:     	movaps	0x120(%rsp), %xmm6
140017084:     	movaps	0x110(%rsp), %xmm7
14001708c:     	movaps	0x100(%rsp), %xmm8
140017095:     	movaps	0xf0(%rsp), %xmm9
14001709e:     	movaps	0xe0(%rsp), %xmm10
1400170a7:     	movaps	0xd0(%rsp), %xmm11
1400170b0:     	movdqa	0xc0(%rsp), %xmm12
1400170ba:     	movdqa	0xb0(%rsp), %xmm13
1400170c4:     	movaps	0xa0(%rsp), %xmm14
1400170cd:     	movdqa	0x90(%rsp), %xmm15
1400170d7:     	addq	$0x138, %rsp            # imm = 0x138
1400170de:     	popq	%rbx
1400170df:     	popq	%rdi
1400170e0:     	popq	%rsi
1400170e1:     	popq	%r12
1400170e3:     	popq	%r13
1400170e5:     	popq	%r14
1400170e7:     	popq	%r15
1400170e9:     	popq	%rbp
1400170ea:     	retq
1400170eb:     	nopl	(%rax,%rax)
1400170f0:     	movq	%rdx, 0x10(%rsp)
1400170f5:     	pushq	%rbp
1400170f6:     	pushq	%r15
1400170f8:     	pushq	%r14
1400170fa:     	pushq	%r13
1400170fc:     	pushq	%r12
1400170fe:     	pushq	%rsi
1400170ff:     	pushq	%rdi
140017100:     	pushq	%rbx
140017101:     	subq	$0x138, %rsp            # imm = 0x138
140017108:     	leaq	0x80(%rdx), %rbp
14001710f:     	movdqa	%xmm15, 0x90(%rsp)
140017119:     	movaps	%xmm14, 0xa0(%rsp)
140017122:     	movdqa	%xmm13, 0xb0(%rsp)
14001712c:     	movdqa	%xmm12, 0xc0(%rsp)
140017136:     	movaps	%xmm11, 0xd0(%rsp)
14001713f:     	movaps	%xmm10, 0xe0(%rsp)
140017148:     	movaps	%xmm9, 0xf0(%rsp)
140017151:     	movaps	%xmm8, 0x100(%rsp)
14001715a:     	movaps	%xmm7, 0x110(%rsp)
140017162:     	movaps	%xmm6, 0x120(%rsp)
14001716a:     	movq	0x578(%rbp), %rax
140017171:     	cmpq	0x520(%rbp), %rax
140017178:     	jne	0x1400171f0 <.text+0x161f0>
14001717a:     	movb	$0x1, 0x814(%rbp)
140017181:     	movaps	0x120(%rsp), %xmm6
140017189:     	movaps	0x110(%rsp), %xmm7
140017191:     	movaps	0x100(%rsp), %xmm8
14001719a:     	movaps	0xf0(%rsp), %xmm9
1400171a3:     	movaps	0xe0(%rsp), %xmm10
1400171ac:     	movaps	0xd0(%rsp), %xmm11
1400171b5:     	movdqa	0xc0(%rsp), %xmm12
1400171bf:     	movdqa	0xb0(%rsp), %xmm13
1400171c9:     	movaps	0xa0(%rsp), %xmm14
1400171d2:     	movdqa	0x90(%rsp), %xmm15
1400171dc:     	addq	$0x138, %rsp            # imm = 0x138
1400171e3:     	popq	%rbx
1400171e4:     	popq	%rdi
1400171e5:     	popq	%rsi
1400171e6:     	popq	%r12
1400171e8:     	popq	%r13
1400171ea:     	popq	%r14
1400171ec:     	popq	%r15
1400171ee:     	popq	%rbp
1400171ef:     	retq
1400171f0:     	movq	0x330(%rbp), %r14
1400171f7:     	testq	%r14, %r14
1400171fa:     	movq	0x580(%rbp), %rdi
140017201:     	movq	0x560(%rbp), %rbx
140017208:     	jne	0x1400172c1 <.text+0x162c1>
14001720e:     	testq	%rdi, %rdi
140017211:     	je	0x14001722d <.text+0x1622d>
140017213:     	shlq	$0x3, %rdi
140017217:     	leaq	(%rdi,%rdi,2), %rdx
14001721b:     	movl	$0x8, %r8d
140017221:     	movq	0x568(%rbp), %rcx
140017228:     	callq	0x140613c20 <.text+0x612c20>
14001722d:     	testq	%rbx, %rbx
140017230:     	je	0x14001724b <.text+0x1624b>
140017232:     	shlq	$0x4, %rbx
140017236:     	movl	$0x8, %r8d
14001723c:     	movq	0x530(%rbp), %rcx
140017243:     	movq	%rbx, %rdx
140017246:     	callq	0x140613c20 <.text+0x612c20>
14001724b:     	movb	$0x1, 0x814(%rbp)
140017252:     	movaps	0x120(%rsp), %xmm6
14001725a:     	movaps	0x110(%rsp), %xmm7
140017262:     	movaps	0x100(%rsp), %xmm8
14001726b:     	movaps	0xf0(%rsp), %xmm9
140017274:     	movaps	0xe0(%rsp), %xmm10
14001727d:     	movaps	0xd0(%rsp), %xmm11
140017286:     	movdqa	0xc0(%rsp), %xmm12
140017290:     	movdqa	0xb0(%rsp), %xmm13
14001729a:     	movaps	0xa0(%rsp), %xmm14
1400172a3:     	movdqa	0x90(%rsp), %xmm15
1400172ad:     	addq	$0x138, %rsp            # imm = 0x138
1400172b4:     	popq	%rbx
1400172b5:     	popq	%rdi
1400172b6:     	popq	%rsi
1400172b7:     	popq	%r12
1400172b9:     	popq	%r13
1400172bb:     	popq	%r14
1400172bd:     	popq	%r15
1400172bf:     	popq	%rbp
1400172c0:     	retq
1400172c1:     	movq	0x568(%rbp), %rax
1400172c8:     	leaq	0x8(%rax), %rsi
1400172cc:     	jmp	0x1400172dd <.text+0x162dd>
1400172ce:     	nop
1400172d0:     	addq	$0x18, %rsi
1400172d4:     	decq	%r14
1400172d7:     	je	0x14001720e <.text+0x1620e>
1400172dd:     	movq	-0x8(%rsi), %rdx
1400172e1:     	testq	%rdx, %rdx
1400172e4:     	je	0x1400172d0 <.text+0x162d0>
1400172e6:     	movq	(%rsi), %rcx
1400172e9:     	shlq	$0x2, %rdx
1400172ed:     	movl	$0x4, %r8d
1400172f3:     	callq	0x140613c20 <.text+0x612c20>
1400172f8:     	jmp	0x1400172d0 <.text+0x162d0>
1400172fa:     	nopw	(%rax,%rax)
140017300:     	movq	%rdx, 0x10(%rsp)
140017305:     	pushq	%rbp
140017306:     	pushq	%r15
140017308:     	pushq	%r14
14001730a:     	pushq	%r13
14001730c:     	pushq	%r12
14001730e:     	pushq	%rsi
14001730f:     	pushq	%rdi
140017310:     	pushq	%rbx
140017311:     	subq	$0x138, %rsp            # imm = 0x138
140017318:     	leaq	0x80(%rdx), %rbp
14001731f:     	movdqa	%xmm15, 0x90(%rsp)
140017329:     	movaps	%xmm14, 0xa0(%rsp)
140017332:     	movdqa	%xmm13, 0xb0(%rsp)
14001733c:     	movdqa	%xmm12, 0xc0(%rsp)
140017346:     	movaps	%xmm11, 0xd0(%rsp)
14001734f:     	movaps	%xmm10, 0xe0(%rsp)
140017358:     	movaps	%xmm9, 0xf0(%rsp)
140017361:     	movaps	%xmm8, 0x100(%rsp)
14001736a:     	movaps	%xmm7, 0x110(%rsp)
140017372:     	movaps	%xmm6, 0x120(%rsp)
14001737a:     	cmpb	$0x0, 0x814(%rbp)
140017381:     	je	0x14001747b <.text+0x1647b>
140017387:     	movq	0x460(%rbp), %rax
14001738e:     	testq	%rax, %rax
140017391:     	jne	0x14001750d <.text+0x1650d>
140017397:     	movq	0x478(%rbp), %rdx
14001739e:     	testq	%rdx, %rdx
1400173a1:     	jne	0x140017537 <.text+0x16537>
1400173a7:     	movq	0x498(%rbp), %rdx
1400173ae:     	testq	%rdx, %rdx
1400173b1:     	jne	0x14001755d <.text+0x1655d>
1400173b7:     	movq	0x4b0(%rbp), %rdx
1400173be:     	testq	%rdx, %rdx
1400173c1:     	jne	0x140017583 <.text+0x16583>
1400173c7:     	movq	0x4c8(%rbp), %rdx
1400173ce:     	testq	%rdx, %rdx
1400173d1:     	je	0x1400173e9 <.text+0x163e9>
1400173d3:     	movq	0x4d0(%rbp), %rcx
1400173da:     	shlq	$0x2, %rdx
1400173de:     	movl	$0x4, %r8d
1400173e4:     	callq	0x140613c20 <.text+0x612c20>
1400173e9:     	movb	$0x0, 0x7ee(%rbp)
1400173f0:     	movb	$0x0, 0x7ed(%rbp)
1400173f7:     	movb	$0x0, 0x7ec(%rbp)
1400173fe:     	movb	$0x1, 0x7eb(%rbp)
140017405:     	movb	$0x1, 0x7ea(%rbp)
14001740c:     	movaps	0x120(%rsp), %xmm6
140017414:     	movaps	0x110(%rsp), %xmm7
14001741c:     	movaps	0x100(%rsp), %xmm8
140017425:     	movaps	0xf0(%rsp), %xmm9
14001742e:     	movaps	0xe0(%rsp), %xmm10
140017437:     	movaps	0xd0(%rsp), %xmm11
140017440:     	movdqa	0xc0(%rsp), %xmm12
14001744a:     	movdqa	0xb0(%rsp), %xmm13
140017454:     	movaps	0xa0(%rsp), %xmm14
14001745d:     	movdqa	0x90(%rsp), %xmm15
140017467:     	addq	$0x138, %rsp            # imm = 0x138
14001746e:     	popq	%rbx
14001746f:     	popq	%rdi
140017470:     	popq	%rsi
140017471:     	popq	%r12
140017473:     	popq	%r13
140017475:     	popq	%r14
140017477:     	popq	%r15
140017479:     	popq	%rbp
14001747a:     	retq
14001747b:     	movb	$0x0, 0x7ee(%rbp)
140017482:     	movb	$0x0, 0x7ed(%rbp)
140017489:     	movb	$0x0, 0x7ec(%rbp)
140017490:     	movb	$0x1, 0x7eb(%rbp)
140017497:     	movb	$0x1, 0x7ea(%rbp)
14001749e:     	movaps	0x120(%rsp), %xmm6
1400174a6:     	movaps	0x110(%rsp), %xmm7
1400174ae:     	movaps	0x100(%rsp), %xmm8
1400174b7:     	movaps	0xf0(%rsp), %xmm9
1400174c0:     	movaps	0xe0(%rsp), %xmm10
1400174c9:     	movaps	0xd0(%rsp), %xmm11
1400174d2:     	movdqa	0xc0(%rsp), %xmm12
1400174dc:     	movdqa	0xb0(%rsp), %xmm13
1400174e6:     	movaps	0xa0(%rsp), %xmm14
1400174ef:     	movdqa	0x90(%rsp), %xmm15
1400174f9:     	addq	$0x138, %rsp            # imm = 0x138
140017500:     	popq	%rbx
140017501:     	popq	%rdi
140017502:     	popq	%rsi
140017503:     	popq	%r12
140017505:     	popq	%r13
140017507:     	popq	%r14
140017509:     	popq	%r15
14001750b:     	popq	%rbp
14001750c:     	retq
14001750d:     	movq	0x468(%rbp), %rcx
140017514:     	shlq	$0x2, %rax
140017518:     	leaq	(%rax,%rax,2), %rdx
14001751c:     	movl	$0x4, %r8d
140017522:     	callq	0x140613c20 <.text+0x612c20>
140017527:     	movq	0x478(%rbp), %rdx
14001752e:     	testq	%rdx, %rdx
140017531:     	je	0x1400173a7 <.text+0x163a7>
140017537:     	movq	0x480(%rbp), %rcx
14001753e:     	shlq	$0x2, %rdx
140017542:     	movl	$0x4, %r8d
140017548:     	callq	0x140613c20 <.text+0x612c20>
14001754d:     	movq	0x498(%rbp), %rdx
140017554:     	testq	%rdx, %rdx
140017557:     	je	0x1400173b7 <.text+0x163b7>
14001755d:     	movq	0x4a0(%rbp), %rcx
140017564:     	shlq	$0x3, %rdx
140017568:     	movl	$0x4, %r8d
14001756e:     	callq	0x140613c20 <.text+0x612c20>
140017573:     	movq	0x4b0(%rbp), %rdx
14001757a:     	testq	%rdx, %rdx
14001757d:     	je	0x1400173c7 <.text+0x163c7>
140017583:     	movq	0x4b8(%rbp), %rcx
14001758a:     	shlq	$0x2, %rdx
14001758e:     	movl	$0x4, %r8d
140017594:     	callq	0x140613c20 <.text+0x612c20>
140017599:     	movq	0x4c8(%rbp), %rdx
1400175a0:     	testq	%rdx, %rdx
1400175a3:     	jne	0x1400173d3 <.text+0x163d3>
1400175a9:     	jmp	0x1400173e9 <.text+0x163e9>
1400175ae:     	nop
1400175b0:     	movq	%rdx, 0x10(%rsp)
1400175b5:     	pushq	%rbp
1400175b6:     	pushq	%r15
1400175b8:     	pushq	%r14
1400175ba:     	pushq	%r13
1400175bc:     	pushq	%r12
1400175be:     	pushq	%rsi
1400175bf:     	pushq	%rdi
1400175c0:     	pushq	%rbx
1400175c1:     	subq	$0x138, %rsp            # imm = 0x138
1400175c8:     	leaq	0x80(%rdx), %rbp
1400175cf:     	movdqa	%xmm15, 0x90(%rsp)
1400175d9:     	movaps	%xmm14, 0xa0(%rsp)
1400175e2:     	movdqa	%xmm13, 0xb0(%rsp)
1400175ec:     	movdqa	%xmm12, 0xc0(%rsp)
1400175f6:     	movaps	%xmm11, 0xd0(%rsp)
1400175ff:     	movaps	%xmm10, 0xe0(%rsp)
140017608:     	movaps	%xmm9, 0xf0(%rsp)
140017611:     	movaps	%xmm8, 0x100(%rsp)
14001761a:     	movaps	%xmm7, 0x110(%rsp)
140017622:     	movaps	%xmm6, 0x120(%rsp)
14001762a:     	movzbl	0x7eb(%rbp), %r14d
140017632:     	movzbl	0x7ec(%rbp), %edi
140017639:     	movzbl	0x7ed(%rbp), %esi
140017640:     	movzbl	0x7ee(%rbp), %ebx
140017647:     	cmpq	$0x0, 0x800(%rbp)
14001764f:     	je	0x14001767b <.text+0x1667b>
140017651:     	testb	$0x1, 0x7ea(%rbp)
140017658:     	jne	0x14001767b <.text+0x1667b>
14001765a:     	movq	0x800(%rbp), %rax
140017661:     	shlq	$0x2, %rax
140017665:     	leaq	(%rax,%rax,2), %rdx
140017669:     	movl	$0x4, %r8d
14001766f:     	movq	0x7f0(%rbp), %rcx
140017676:     	callq	0x140613c20 <.text+0x612c20>
14001767b:     	movq	0x570(%rbp), %rdx
140017682:     	testq	%rdx, %rdx
140017685:     	sete	%al
140017688:     	orb	%al, %r14b
14001768b:     	testb	$0x1, %r14b
14001768f:     	jne	0x1400176a7 <.text+0x166a7>
140017691:     	shlq	$0x2, %rdx
140017695:     	movl	$0x4, %r8d
14001769b:     	movq	0x528(%rbp), %rcx
1400176a2:     	callq	0x140613c20 <.text+0x612c20>
1400176a7:     	movq	0x648(%rbp), %rdx
1400176ae:     	testq	%rdx, %rdx
1400176b1:     	setne	%al
1400176b4:     	testb	%al, %dil
1400176b7:     	movq	0x780(%rbp), %rdi
1400176be:     	movq	0x760(%rbp), %r14
1400176c5:     	je	0x140017787 <.text+0x16787>
1400176cb:     	shlq	$0x3, %rdx
1400176cf:     	movl	$0x4, %r8d
1400176d5:     	movq	0x7b0(%rbp), %rcx
1400176dc:     	callq	0x140613c20 <.text+0x612c20>
1400176e1:     	testq	%rdi, %rdi
1400176e4:     	setne	%al
1400176e7:     	testb	%al, %sil
1400176ea:     	jne	0x140017796 <.text+0x16796>
1400176f0:     	testb	$0x1, %bl
1400176f3:     	je	0x1400177b4 <.text+0x167b4>
1400176f9:     	movq	0x7e0(%rbp), %rdx
140017700:     	testq	%rdx, %rdx
140017703:     	je	0x14001771b <.text+0x1671b>
140017705:     	shlq	$0x2, %rdx
140017709:     	movl	$0x4, %r8d
14001770f:     	movq	0x778(%rbp), %rcx
140017716:     	callq	0x140613c20 <.text+0x612c20>
14001771b:     	movaps	0x120(%rsp), %xmm6
140017723:     	movaps	0x110(%rsp), %xmm7
14001772b:     	movaps	0x100(%rsp), %xmm8
140017734:     	movaps	0xf0(%rsp), %xmm9
14001773d:     	movaps	0xe0(%rsp), %xmm10
140017746:     	movaps	0xd0(%rsp), %xmm11
14001774f:     	movaps	0xc0(%rsp), %xmm12
140017758:     	movaps	0xb0(%rsp), %xmm13
140017761:     	movaps	0xa0(%rsp), %xmm14
14001776a:     	movaps	0x90(%rsp), %xmm15
140017773:     	addq	$0x138, %rsp            # imm = 0x138
14001777a:     	popq	%rbx
14001777b:     	popq	%rdi
14001777c:     	popq	%rsi
14001777d:     	popq	%r12
14001777f:     	popq	%r13
140017781:     	popq	%r14
140017783:     	popq	%r15
140017785:     	popq	%rbp
140017786:     	retq
140017787:     	testq	%rdi, %rdi
14001778a:     	setne	%al
14001778d:     	testb	%al, %sil
140017790:     	je	0x1400176f0 <.text+0x166f0>
140017796:     	shlq	$0x2, %rdi
14001779a:     	movl	$0x4, %r8d
1400177a0:     	movq	%r14, %rcx
1400177a3:     	movq	%rdi, %rdx
1400177a6:     	callq	0x140613c20 <.text+0x612c20>
1400177ab:     	testb	$0x1, %bl
1400177ae:     	jne	0x1400176f9 <.text+0x166f9>
1400177b4:     	movaps	0x120(%rsp), %xmm6
1400177bc:     	movaps	0x110(%rsp), %xmm7
1400177c4:     	movaps	0x100(%rsp), %xmm8
1400177cd:     	movaps	0xf0(%rsp), %xmm9
1400177d6:     	movaps	0xe0(%rsp), %xmm10
1400177df:     	movaps	0xd0(%rsp), %xmm11
1400177e8:     	movaps	0xc0(%rsp), %xmm12
1400177f1:     	movaps	0xb0(%rsp), %xmm13
1400177fa:     	movaps	0xa0(%rsp), %xmm14
140017803:     	movaps	0x90(%rsp), %xmm15
14001780c:     	addq	$0x138, %rsp            # imm = 0x138
140017813:     	popq	%rbx
140017814:     	popq	%rdi
140017815:     	popq	%rsi
140017816:     	popq	%r12
140017818:     	popq	%r13
14001781a:     	popq	%r14
14001781c:     	popq	%r15
14001781e:     	popq	%rbp
14001781f:     	retq
140017820:     	movq	%rdx, 0x10(%rsp)
140017825:     	pushq	%rbp
140017826:     	pushq	%r15
140017828:     	pushq	%r14
14001782a:     	pushq	%r13
14001782c:     	pushq	%r12
14001782e:     	pushq	%rsi
14001782f:     	pushq	%rdi
140017830:     	pushq	%rbx
140017831:     	subq	$0x138, %rsp            # imm = 0x138
140017838:     	leaq	0x80(%rdx), %rbp
14001783f:     	movaps	%xmm15, 0x90(%rsp)
140017848:     	movaps	%xmm14, 0xa0(%rsp)
140017851:     	movaps	%xmm13, 0xb0(%rsp)
14001785a:     	movaps	%xmm12, 0xc0(%rsp)
140017863:     	movaps	%xmm11, 0xd0(%rsp)
14001786c:     	movaps	%xmm10, 0xe0(%rsp)
140017875:     	movaps	%xmm9, 0xf0(%rsp)
14001787e:     	movaps	%xmm8, 0x100(%rsp)
140017887:     	movaps	%xmm7, 0x110(%rsp)
14001788f:     	movaps	%xmm6, 0x120(%rsp)
140017897:     	xorl	%eax, %eax
140017899:     	cmpq	0x6a0(%rbp), %rax
1400178a0:     	jo	0x1400178ae <.text+0x168ae>
1400178a2:     	leaq	0x6a0(%rbp), %rcx
1400178a9:     	callq	0x140609eb0 <.text+0x608eb0>
1400178ae:     	movb	$0x0, 0x7ff(%rbp)
1400178b5:     	movaps	0x120(%rsp), %xmm6
1400178bd:     	movaps	0x110(%rsp), %xmm7
1400178c5:     	movaps	0x100(%rsp), %xmm8
1400178ce:     	movaps	0xf0(%rsp), %xmm9
1400178d7:     	movaps	0xe0(%rsp), %xmm10
1400178e0:     	movaps	0xd0(%rsp), %xmm11
1400178e9:     	movdqa	0xc0(%rsp), %xmm12
1400178f3:     	movdqa	0xb0(%rsp), %xmm13
1400178fd:     	movaps	0xa0(%rsp), %xmm14
140017906:     	movdqa	0x90(%rsp), %xmm15
140017910:     	addq	$0x138, %rsp            # imm = 0x138
140017917:     	popq	%rbx
140017918:     	popq	%rdi
140017919:     	popq	%rsi
14001791a:     	popq	%r12
14001791c:     	popq	%r13
14001791e:     	popq	%r14
140017920:     	popq	%r15
140017922:     	popq	%rbp
140017923:     	retq
140017924:     	nopw	%cs:(%rax,%rax)
140017930:     	movq	%rdx, 0x10(%rsp)
140017935:     	pushq	%rbp
140017936:     	pushq	%r15
140017938:     	pushq	%r14
14001793a:     	pushq	%r13
14001793c:     	pushq	%r12
14001793e:     	pushq	%rsi
14001793f:     	pushq	%rdi
140017940:     	pushq	%rbx
140017941:     	subq	$0x138, %rsp            # imm = 0x138
140017948:     	leaq	0x80(%rdx), %rbp
14001794f:     	movdqa	%xmm15, 0x90(%rsp)
140017959:     	movaps	%xmm14, 0xa0(%rsp)
140017962:     	movdqa	%xmm13, 0xb0(%rsp)
14001796c:     	movdqa	%xmm12, 0xc0(%rsp)
140017976:     	movaps	%xmm11, 0xd0(%rsp)
14001797f:     	movaps	%xmm10, 0xe0(%rsp)
140017988:     	movaps	%xmm9, 0xf0(%rsp)
140017991:     	movaps	%xmm8, 0x100(%rsp)
14001799a:     	movaps	%xmm7, 0x110(%rsp)
1400179a2:     	movaps	%xmm6, 0x120(%rsp)
1400179aa:     	cmpb	$0x0, 0x7ff(%rbp)
1400179b1:     	je	0x140017a4f <.text+0x16a4f>
1400179b7:     	cmpb	$0x11, 0x500(%rbp)
1400179be:     	je	0x1400179d9 <.text+0x169d9>
1400179c0:     	movq	0x518(%rbp), %rax
1400179c7:     	lock
1400179c8:     	decq	(%rax)
1400179cb:     	jne	0x1400179d9 <.text+0x169d9>
1400179cd:     	leaq	0x518(%rbp), %rcx
1400179d4:     	callq	0x140b5c620 <.text+0xb5b620>
1400179d9:     	movb	$0x0, 0x817(%rbp)
1400179e0:     	movaps	0x120(%rsp), %xmm6
1400179e8:     	movaps	0x110(%rsp), %xmm7
1400179f0:     	movaps	0x100(%rsp), %xmm8
1400179f9:     	movaps	0xf0(%rsp), %xmm9
140017a02:     	movaps	0xe0(%rsp), %xmm10
140017a0b:     	movaps	0xd0(%rsp), %xmm11
140017a14:     	movdqa	0xc0(%rsp), %xmm12
140017a1e:     	movdqa	0xb0(%rsp), %xmm13
140017a28:     	movaps	0xa0(%rsp), %xmm14
140017a31:     	movdqa	0x90(%rsp), %xmm15
140017a3b:     	addq	$0x138, %rsp            # imm = 0x138
140017a42:     	popq	%rbx
140017a43:     	popq	%rdi
140017a44:     	popq	%rsi
140017a45:     	popq	%r12
140017a47:     	popq	%r13
140017a49:     	popq	%r14
140017a4b:     	popq	%r15
140017a4d:     	popq	%rbp
140017a4e:     	retq
140017a4f:     	movb	$0x0, 0x817(%rbp)
140017a56:     	movaps	0x120(%rsp), %xmm6
140017a5e:     	movaps	0x110(%rsp), %xmm7
140017a66:     	movaps	0x100(%rsp), %xmm8
140017a6f:     	movaps	0xf0(%rsp), %xmm9
140017a78:     	movaps	0xe0(%rsp), %xmm10
140017a81:     	movaps	0xd0(%rsp), %xmm11
140017a8a:     	movdqa	0xc0(%rsp), %xmm12
140017a94:     	movdqa	0xb0(%rsp), %xmm13
140017a9e:     	movaps	0xa0(%rsp), %xmm14
140017aa7:     	movdqa	0x90(%rsp), %xmm15
140017ab1:     	addq	$0x138, %rsp            # imm = 0x138
140017ab8:     	popq	%rbx
140017ab9:     	popq	%rdi
140017aba:     	popq	%rsi
140017abb:     	popq	%r12
140017abd:     	popq	%r13
140017abf:     	popq	%r14
140017ac1:     	popq	%r15
140017ac3:     	popq	%rbp
140017ac4:     	retq
140017ac5:     	nopw	%cs:(%rax,%rax)
140017ad0:     	movq	%rdx, 0x10(%rsp)
140017ad5:     	pushq	%rbp
140017ad6:     	pushq	%r15
140017ad8:     	pushq	%r14
140017ada:     	pushq	%r13
140017adc:     	pushq	%r12
140017ade:     	pushq	%rsi
140017adf:     	pushq	%rdi
140017ae0:     	pushq	%rbx
140017ae1:     	subq	$0x138, %rsp            # imm = 0x138
140017ae8:     	leaq	0x80(%rdx), %rbp
140017aef:     	movdqa	%xmm15, 0x90(%rsp)
140017af9:     	movaps	%xmm14, 0xa0(%rsp)
140017b02:     	movdqa	%xmm13, 0xb0(%rsp)
140017b0c:     	movdqa	%xmm12, 0xc0(%rsp)
140017b16:     	movaps	%xmm11, 0xd0(%rsp)
140017b1f:     	movaps	%xmm10, 0xe0(%rsp)
140017b28:     	movaps	%xmm9, 0xf0(%rsp)
140017b31:     	movaps	%xmm8, 0x100(%rsp)
140017b3a:     	movaps	%xmm7, 0x110(%rsp)
140017b42:     	movaps	%xmm6, 0x120(%rsp)
140017b4a:     	cmpb	$0x0, 0x817(%rbp)
140017b51:     	je	0x140017c11 <.text+0x16c11>
140017b57:     	movq	0x280(%rbp), %rsi
140017b5e:     	movq	0x288(%rbp), %rdi
140017b65:     	xorl	%ebx, %ebx
140017b67:     	movq	%rsi, %rcx
140017b6a:     	cmpq	%rbx, %rdi
140017b6d:     	je	0x140017b87 <.text+0x16b87>
140017b6f:     	nop
140017b70:     	incq	%rbx
140017b73:     	leaq	0x80(%rcx), %r14
140017b7a:     	callq	0x140601be0 <.text+0x600be0>
140017b7f:     	movq	%r14, %rcx
140017b82:     	cmpq	%rbx, %rdi
140017b85:     	jne	0x140017b70 <.text+0x16b70>
140017b87:     	movq	0x278(%rbp), %rdx
140017b8e:     	testq	%rdx, %rdx
140017b91:     	je	0x140017ba5 <.text+0x16ba5>
140017b93:     	shlq	$0x7, %rdx
140017b97:     	movl	$0x10, %r8d
140017b9d:     	movq	%rsi, %rcx
140017ba0:     	callq	0x140613c20 <.text+0x612c20>
140017ba5:     	movaps	0x120(%rsp), %xmm6
140017bad:     	movaps	0x110(%rsp), %xmm7
140017bb5:     	movaps	0x100(%rsp), %xmm8
140017bbe:     	movaps	0xf0(%rsp), %xmm9
140017bc7:     	movaps	0xe0(%rsp), %xmm10
140017bd0:     	movaps	0xd0(%rsp), %xmm11
140017bd9:     	movaps	0xc0(%rsp), %xmm12
140017be2:     	movaps	0xb0(%rsp), %xmm13
140017beb:     	movaps	0xa0(%rsp), %xmm14
140017bf4:     	movaps	0x90(%rsp), %xmm15
140017bfd:     	addq	$0x138, %rsp            # imm = 0x138
140017c04:     	popq	%rbx
140017c05:     	popq	%rdi
140017c06:     	popq	%rsi
140017c07:     	popq	%r12
140017c09:     	popq	%r13
140017c0b:     	popq	%r14
140017c0d:     	popq	%r15
140017c0f:     	popq	%rbp
140017c10:     	retq
140017c11:     	movaps	0x120(%rsp), %xmm6
140017c19:     	movaps	0x110(%rsp), %xmm7
140017c21:     	movaps	0x100(%rsp), %xmm8
140017c2a:     	movaps	0xf0(%rsp), %xmm9
140017c33:     	movaps	0xe0(%rsp), %xmm10
140017c3c:     	movaps	0xd0(%rsp), %xmm11
140017c45:     	movaps	0xc0(%rsp), %xmm12
140017c4e:     	movaps	0xb0(%rsp), %xmm13
140017c57:     	movaps	0xa0(%rsp), %xmm14
140017c60:     	movaps	0x90(%rsp), %xmm15
140017c69:     	addq	$0x138, %rsp            # imm = 0x138
140017c70:     	popq	%rbx
140017c71:     	popq	%rdi
140017c72:     	popq	%rsi
140017c73:     	popq	%r12
140017c75:     	popq	%r13
140017c77:     	popq	%r14
140017c79:     	popq	%r15
140017c7b:     	popq	%rbp
140017c7c:     	retq
140017c7d:     	nopl	(%rax)
140017c80:     	movq	%rdx, 0x10(%rsp)
140017c85:     	pushq	%rbp
140017c86:     	pushq	%r15
140017c88:     	pushq	%r14
140017c8a:     	pushq	%r13
140017c8c:     	pushq	%r12
140017c8e:     	pushq	%rsi
140017c8f:     	pushq	%rdi
140017c90:     	pushq	%rbx
140017c91:     	subq	$0x138, %rsp            # imm = 0x138
140017c98:     	leaq	0x80(%rdx), %rbp
140017c9f:     	movaps	%xmm15, 0x90(%rsp)
140017ca8:     	movaps	%xmm14, 0xa0(%rsp)
140017cb1:     	movaps	%xmm13, 0xb0(%rsp)
140017cba:     	movaps	%xmm12, 0xc0(%rsp)
140017cc3:     	movaps	%xmm11, 0xd0(%rsp)
140017ccc:     	movaps	%xmm10, 0xe0(%rsp)
140017cd5:     	movaps	%xmm9, 0xf0(%rsp)
140017cde:     	movaps	%xmm8, 0x100(%rsp)
140017ce7:     	movaps	%xmm7, 0x110(%rsp)
140017cef:     	movaps	%xmm6, 0x120(%rsp)
140017cf7:     	leaq	0x440(%rbp), %rcx
140017cfe:     	callq	0x1405dec70 <.text+0x5ddc70>
140017d03:     	movaps	0x120(%rsp), %xmm6
140017d0b:     	movaps	0x110(%rsp), %xmm7
140017d13:     	movaps	0x100(%rsp), %xmm8
140017d1c:     	movaps	0xf0(%rsp), %xmm9
140017d25:     	movaps	0xe0(%rsp), %xmm10
140017d2e:     	movaps	0xd0(%rsp), %xmm11
140017d37:     	movdqa	0xc0(%rsp), %xmm12
140017d41:     	movdqa	0xb0(%rsp), %xmm13
140017d4b:     	movaps	0xa0(%rsp), %xmm14
140017d54:     	movdqa	0x90(%rsp), %xmm15
140017d5e:     	addq	$0x138, %rsp            # imm = 0x138
140017d65:     	popq	%rbx
140017d66:     	popq	%rdi
140017d67:     	popq	%rsi
140017d68:     	popq	%r12
140017d6a:     	popq	%r13
140017d6c:     	popq	%r14
140017d6e:     	popq	%r15
140017d70:     	popq	%rbp
140017d71:     	retq
140017d72:     	nopw	%cs:(%rax,%rax)
140017d80:     	movq	%rdx, 0x10(%rsp)
140017d85:     	pushq	%rbp
140017d86:     	pushq	%r15
140017d88:     	pushq	%r14
140017d8a:     	pushq	%r13
140017d8c:     	pushq	%r12
140017d8e:     	pushq	%rsi
140017d8f:     	pushq	%rdi
140017d90:     	pushq	%rbx
140017d91:     	subq	$0x138, %rsp            # imm = 0x138
140017d98:     	leaq	0x80(%rdx), %rbp
140017d9f:     	movdqa	%xmm15, 0x90(%rsp)
140017da9:     	movaps	%xmm14, 0xa0(%rsp)
140017db2:     	movdqa	%xmm13, 0xb0(%rsp)
140017dbc:     	movdqa	%xmm12, 0xc0(%rsp)
140017dc6:     	movaps	%xmm11, 0xd0(%rsp)
140017dcf:     	movaps	%xmm10, 0xe0(%rsp)
140017dd8:     	movaps	%xmm9, 0xf0(%rsp)
140017de1:     	movaps	%xmm8, 0x100(%rsp)
140017dea:     	movaps	%xmm7, 0x110(%rsp)
140017df2:     	movaps	%xmm6, 0x120(%rsp)
140017dfa:     	movq	0x160(%rbp), %rdi
140017e01:     	leaq	0x18(%rdi), %rsi
140017e05:     	movq	%rsi, %rcx
140017e08:     	callq	0x140c81e10 <.text+0xc80e10>
140017e0d:     	xorl	%ecx, %ecx
140017e0f:     	movb	$0x1, %al
140017e11:     	lock
140017e12:     	cmpxchgb	%cl, 0x18(%rdi)
140017e16:     	jne	0x140017ea6 <.text+0x16ea6>
140017e1c:     	movq	0x1b8(%rbp), %rdi
140017e23:     	leaq	0x18(%rdi), %rsi
140017e27:     	movq	%rsi, %rcx
140017e2a:     	callq	0x140c81e10 <.text+0xc80e10>
140017e2f:     	xorl	%ecx, %ecx
140017e31:     	movb	$0x1, %al
140017e33:     	lock
140017e34:     	cmpxchgb	%cl, 0x18(%rdi)
140017e38:     	jne	0x140017eb5 <.text+0x16eb5>
140017e3a:     	movaps	0x120(%rsp), %xmm6
140017e42:     	movaps	0x110(%rsp), %xmm7
140017e4a:     	movaps	0x100(%rsp), %xmm8
140017e53:     	movaps	0xf0(%rsp), %xmm9
140017e5c:     	movaps	0xe0(%rsp), %xmm10
140017e65:     	movaps	0xd0(%rsp), %xmm11
140017e6e:     	movaps	0xc0(%rsp), %xmm12
140017e77:     	movaps	0xb0(%rsp), %xmm13
140017e80:     	movaps	0xa0(%rsp), %xmm14
140017e89:     	movaps	0x90(%rsp), %xmm15
140017e92:     	addq	$0x138, %rsp            # imm = 0x138
140017e99:     	popq	%rbx
140017e9a:     	popq	%rdi
140017e9b:     	popq	%rsi
140017e9c:     	popq	%r12
140017e9e:     	popq	%r13
140017ea0:     	popq	%r14
140017ea2:     	popq	%r15
140017ea4:     	popq	%rbp
140017ea5:     	retq
140017ea6:     	movq	%rsi, %rcx
140017ea9:     	xorl	%edx, %edx
140017eab:     	callq	0x1428cdb90 <.text+0x28ccb90>
140017eb0:     	jmp	0x140017e1c <.text+0x16e1c>
140017eb5:     	movq	%rsi, %rcx
140017eb8:     	xorl	%edx, %edx
140017eba:     	callq	0x1428cdb90 <.text+0x28ccb90>
140017ebf:     	jmp	0x140017e3a <.text+0x16e3a>
140017ec4:     	nopw	%cs:(%rax,%rax)
140017ed0:     	movq	%rdx, 0x10(%rsp)
140017ed5:     	pushq	%rbp
140017ed6:     	pushq	%r15
140017ed8:     	pushq	%r14
140017eda:     	pushq	%r13
140017edc:     	pushq	%r12
140017ede:     	pushq	%rsi
140017edf:     	pushq	%rdi
140017ee0:     	pushq	%rbx
140017ee1:     	subq	$0x138, %rsp            # imm = 0x138
140017ee8:     	leaq	0x80(%rdx), %rbp
140017eef:     	movdqa	%xmm15, 0x90(%rsp)
140017ef9:     	movaps	%xmm14, 0xa0(%rsp)
140017f02:     	movdqa	%xmm13, 0xb0(%rsp)
140017f0c:     	movdqa	%xmm12, 0xc0(%rsp)
140017f16:     	movaps	%xmm11, 0xd0(%rsp)
140017f1f:     	movaps	%xmm10, 0xe0(%rsp)
140017f28:     	movaps	%xmm9, 0xf0(%rsp)
140017f31:     	movaps	%xmm8, 0x100(%rsp)
140017f3a:     	movaps	%xmm7, 0x110(%rsp)
140017f42:     	movaps	%xmm6, 0x120(%rsp)
140017f4a:     	cmpb	$0x0, 0x797(%rbp)
140017f51:     	je	0x140017f90 <.text+0x16f90>
140017f53:     	callq	0x140c18130 <.text+0xc17130>
140017f58:     	movq	%rax, %rsi
140017f5b:     	movq	(%rax), %rax
140017f5e:     	cmpq	$0x1, %rax
140017f62:     	movq	0x270(%rbp), %rdx
140017f69:     	jne	0x140017ffc <.text+0x16ffc>
140017f6f:     	addq	$0x8, %rsi
140017f73:     	cmpq	$0x0, (%rsi)
140017f77:     	jne	0x140018029 <.text+0x17029>
140017f7d:     	movq	$-0x1, (%rsi)
140017f84:     	leaq	0x8(%rsi), %rcx
140017f88:     	callq	0x140c170f0 <.text+0xc160f0>
140017f8d:     	incq	(%rsi)
140017f90:     	movaps	0x120(%rsp), %xmm6
140017f98:     	movaps	0x110(%rsp), %xmm7
140017fa0:     	movaps	0x100(%rsp), %xmm8
140017fa9:     	movaps	0xf0(%rsp), %xmm9
140017fb2:     	movaps	0xe0(%rsp), %xmm10
140017fbb:     	movaps	0xd0(%rsp), %xmm11
140017fc4:     	movaps	0xc0(%rsp), %xmm12
140017fcd:     	movaps	0xb0(%rsp), %xmm13
140017fd6:     	movaps	0xa0(%rsp), %xmm14
140017fdf:     	movaps	0x90(%rsp), %xmm15
140017fe8:     	addq	$0x138, %rsp            # imm = 0x138
140017fef:     	popq	%rbx
140017ff0:     	popq	%rdi
140017ff1:     	popq	%rsi
140017ff2:     	popq	%r12
140017ff4:     	popq	%r13
140017ff6:     	popq	%r14
140017ff8:     	popq	%r15
140017ffa:     	popq	%rbp
140017ffb:     	retq
140017ffc:     	testq	%rax, %rax
140017fff:     	jne	0x14001801d <.text+0x1701d>
140018001:     	movq	%rsi, %rcx
140018004:     	movq	%rdx, %rsi
140018007:     	xorl	%edx, %edx
140018009:     	callq	0x1428a1640 <.text+0x28a0640>
14001800e:     	movq	%rsi, %rdx
140018011:     	movq	%rax, %rsi
140018014:     	testq	%rax, %rax
140018017:     	jne	0x140017f73 <.text+0x16f73>
14001801d:     	leaq	0x291a9ec(%rip), %rcx   # 0x142932a10
140018024:     	callq	0x1428d80d0 <.text+0x28d70d0>
140018029:     	leaq	0x291aa88(%rip), %rcx   # 0x142932ab8
140018030:     	callq	0x1428d9250 <.text+0x28d8250>
140018035:     	int3

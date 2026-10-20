
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14237a9c0:     	pushq	%r15
14237a9c2:     	pushq	%r14
14237a9c4:     	pushq	%r13
14237a9c6:     	pushq	%r12
14237a9c8:     	pushq	%rsi
14237a9c9:     	pushq	%rdi
14237a9ca:     	pushq	%rbp
14237a9cb:     	pushq	%rbx
14237a9cc:     	subq	$0xb8, %rsp
14237a9d3:     	movl	%r9d, %ebx
14237a9d6:     	movl	%r8d, %r13d
14237a9d9:     	movq	%rcx, %rsi
14237a9dc:     	movl	0x128(%rsp), %eax
14237a9e3:     	movq	0x120(%rsp), %r14
14237a9eb:     	movl	%r8d, %edi
14237a9ee:     	shrl	$0x8, %edi
14237a9f1:     	movl	%r8d, %ebp
14237a9f4:     	shrl	$0x10, %ebp
14237a9f7:     	movl	%r8d, %r15d
14237a9fa:     	shrl	$0x18, %r15d
14237a9fe:     	movq	%r14, %rcx
14237aa01:     	shrq	$0x38, %rcx
14237aa05:     	movl	%eax, %r12d
14237aa08:     	shrl	$0x8, %r12d
14237aa0c:     	leal	-0x2(%r13), %r8d
14237aa10:     	cmpb	$0x11, %r8b
14237aa14:     	movzbl	%r8b, %r8d
14237aa18:     	movl	$0x5, %r9d
14237aa1e:     	cmovbl	%r8d, %r9d
14237aa22:     	movzbl	%r9b, %r8d
14237aa26:     	leaq	0xcd9523(%rip), %r9     # 0x143053f50
14237aa2d:     	movslq	(%r9,%r8,4), %r8
14237aa31:     	addq	%r9, %r8
14237aa34:     	jmpq	*%r8
14237aa37:     	xorl	%edi, %edi
14237aa39:     	jmp	0x14237b07a <.text+0x237a07a>
14237aa3e:     	testl	%ebx, %ebx
14237aa40:     	setne	%r8b
14237aa44:     	xorb	$0x1, %cl
14237aa47:     	andb	%r8b, %cl
14237aa4a:     	testb	%al, %cl
14237aa4c:     	je	0x14237aeaf <.text+0x2379eaf>
14237aa52:     	movq	%rdx, %r15
14237aa55:     	movl	%ebx, 0x30(%rsp)
14237aa59:     	movq	%r14, 0x34(%rsp)
14237aa5e:     	movw	%r12w, 0x3c(%rsp)
14237aa64:     	shrl	$0x10, %r12d
14237aa68:     	movb	%r12b, 0x3e(%rsp)
14237aa6d:     	leaq	0x58(%rsp), %rbx
14237aa72:     	leaq	0x30(%rsp), %rdx
14237aa77:     	movq	%rbx, %rcx
14237aa7a:     	xorl	%r8d, %r8d
14237aa7d:     	callq	0x140cafc70 <.text+0xcaec70>
14237aa82:     	leaq	0xcd9393(%rip), %rdx    # 0x143053e1c
14237aa89:     	movq	%rbx, %rcx
14237aa8c:     	callq	0x140cafb70 <.text+0xcaeb70>
14237aa91:     	cmpb	$-0x1, %al
14237aa93:     	sete	%cl
14237aa96:     	orb	%dil, %cl
14237aa99:     	testb	$0x1, %cl
14237aa9c:     	je	0x14237aac7 <.text+0x2379ac7>
14237aa9e:     	cmpb	$-0x1, %al
14237aaa0:     	leaq	0xcd9372(%rip), %rax    # 0x143053e19
14237aaa7:     	leaq	0xcd936a(%rip), %rdx    # 0x143053e18
14237aaae:     	cmoveq	%rax, %rdx
14237aab2:     	leaq	0xcd915f(%rip), %r9     # 0x143053c18
14237aab9:     	movl	$0x1, %r8d
14237aabf:     	movq	%r15, %rcx
14237aac2:     	callq	0x140fcd9c0 <.text+0xfcc9c0>
14237aac7:     	movzbl	%bpl, %eax
14237aacb:     	leaq	0xcd94c2(%rip), %rcx    # 0x143053f94
14237aad2:     	movslq	(%rcx,%rax,4), %rax
14237aad6:     	addq	%rcx, %rax
14237aad9:     	jmpq	*%rax
14237aadb:     	movdqu	0x58(%rsp), %xmm0
14237aae1:     	movdqa	%xmm0, 0x30(%rsp)
14237aae7:     	leaq	0x30(%rsp), %rcx
14237aaec:     	callq	0x140caff50 <.text+0xcaef50>
14237aaf1:     	movq	%rax, %rcx
14237aaf4:     	negq	%rcx
14237aaf7:     	cmovsq	%rax, %rcx
14237aafb:     	movl	$0x14, %eax
14237ab00:     	cmpq	$0x2710, %rcx           # imm = 0x2710
14237ab07:     	jb	0x14237b5fa <.text+0x237a5fa>
14237ab0d:     	movl	$0x14, %r10d
14237ab13:     	movabsq	$0x346dc5d63886594b, %r8 # imm = 0x346DC5D63886594B
14237ab1d:     	leaq	0xcd910c(%rip), %r9     # 0x143053c30
14237ab24:     	movq	%rcx, %rax
14237ab27:     	mulq	%r8
14237ab2a:     	shrq	$0xb, %rdx
14237ab2e:     	imull	$0x2710, %edx, %eax     # imm = 0x2710
14237ab34:     	movl	%ecx, %r11d
14237ab37:     	subl	%eax, %r11d
14237ab3a:     	imull	$0x147b, %r11d, %edi    # imm = 0x147B
14237ab41:     	shrl	$0x13, %edi
14237ab44:     	imull	$0x64, %edi, %eax
14237ab47:     	subl	%eax, %r11d
14237ab4a:     	movzwl	%r11w, %r11d
14237ab4e:     	leaq	-0x4(%r10), %rax
14237ab52:     	movzwl	(%r9,%rdi,2), %edi
14237ab57:     	movw	%di, 0x2c(%rsp,%r10)
14237ab5d:     	movzwl	(%r9,%r11,2), %r11d
14237ab62:     	movw	%r11w, 0x2e(%rsp,%r10)
14237ab68:     	movq	%rax, %r10
14237ab6b:     	cmpq	$0x5f5e0ff, %rcx        # imm = 0x5F5E0FF
14237ab72:     	movq	%rdx, %rcx
14237ab75:     	ja	0x14237ab24 <.text+0x2379b24>
14237ab77:     	jmp	0x14237b5fd <.text+0x237a5fd>
14237ab7c:     	testb	$0x1, %al
14237ab7e:     	je	0x14237aeaf <.text+0x2379eaf>
14237ab84:     	testb	%r12b, %r12b
14237ab87:     	js	0x14237b110 <.text+0x237a110>
14237ab8d:     	movl	%eax, %ecx
14237ab8f:     	shrl	$0x10, %ecx
14237ab92:     	testb	%cl, %cl
14237ab94:     	sets	%cl
14237ab97:     	testl	%eax, %eax
14237ab99:     	sets	%al
14237ab9c:     	orb	%cl, %al
14237ab9e:     	orb	%al, %dil
14237aba1:     	leaq	0xcd9271(%rip), %rcx    # 0x143053e19
14237aba8:     	leaq	0xcd9269(%rip), %rbx    # 0x143053e18
14237abaf:     	testb	%al, %al
14237abb1:     	cmovneq	%rcx, %rbx
14237abb5:     	testb	$0x1, %dil
14237abb9:     	jne	0x14237b117 <.text+0x237a117>
14237abbf:     	xorl	%ebx, %ebx
14237abc1:     	jmp	0x14237b13c <.text+0x237a13c>
14237abc6:     	testl	%ebx, %ebx
14237abc8:     	je	0x14237aeaf <.text+0x2379eaf>
14237abce:     	movq	%rdx, %r14
14237abd1:     	testb	%bpl, %bpl
14237abd4:     	je	0x14237af90 <.text+0x2379f90>
14237abda:     	movzbl	%bpl, %eax
14237abde:     	movl	%ebx, %ecx
14237abe0:     	cmpl	$0x1, %eax
14237abe3:     	jne	0x14237b065 <.text+0x237a065>
14237abe9:     	callq	0x140cb1170 <.text+0xcb0170>
14237abee:     	jmp	0x14237b06a <.text+0x237a06a>
14237abf3:     	testb	$0x1, %al
14237abf5:     	je	0x14237aeaf <.text+0x2379eaf>
14237abfb:     	shrl	$0x10, %eax
14237abfe:     	movl	%eax, %ecx
14237ac00:     	sarb	$0x7, %cl
14237ac03:     	xorb	%cl, %al
14237ac05:     	subb	%cl, %al
14237ac07:     	jmp	0x14237ad90 <.text+0x2379d90>
14237ac0c:     	testb	$0x1, %cl
14237ac0f:     	jne	0x14237aeaf <.text+0x2379eaf>
14237ac15:     	shrq	$0x20, %r14
14237ac19:     	jmp	0x14237acee <.text+0x2379cee>
14237ac1e:     	testl	%ebx, %ebx
14237ac20:     	je	0x14237aeaf <.text+0x2379eaf>
14237ac26:     	andl	$0x1ff, %ebx            # imm = 0x1FF
14237ac2c:     	testb	%dil, %dil
14237ac2f:     	je	0x14237af9c <.text+0x2379f9c>
14237ac35:     	movzbl	%dil, %eax
14237ac39:     	cmpl	$0x1, %eax
14237ac3c:     	jne	0x14237b09c <.text+0x237a09c>
14237ac42:     	testw	%bx, %bx
14237ac45:     	je	0x14237b519 <.text+0x237a519>
14237ac4b:     	leal	0x5fff6(%rbx), %eax
14237ac51:     	leal	0x7ff9c(%rbx), %ecx
14237ac57:     	andl	%eax, %ecx
14237ac59:     	leal	0x5fc18(%rbx), %r8d
14237ac60:     	movl	%ebx, %eax
14237ac62:     	addl	$0x7d8f0, %eax          # imm = 0x7D8F0
14237ac67:     	andl	%r8d, %eax
14237ac6a:     	xorl	%ecx, %eax
14237ac6c:     	cmpl	$0x3ffff, %eax          # imm = 0x3FFFF
14237ac71:     	ja	0x14237afd1 <.text+0x2379fd1>
14237ac77:     	shrl	$0x11, %eax
14237ac7a:     	movb	$0x2, %cl
14237ac7c:     	subb	%al, %cl
14237ac7e:     	jmp	0x14237b51b <.text+0x237a51b>
14237ac83:     	testl	%ebx, %ebx
14237ac85:     	je	0x14237aeaf <.text+0x2379eaf>
14237ac8b:     	movq	%rdx, %r14
14237ac8e:     	movl	%ebx, %ecx
14237ac90:     	callq	0x140cb1370 <.text+0xcb0370>
14237ac95:     	movl	%r15d, %ecx
14237ac98:     	leaq	0xcd9329(%rip), %rdx    # 0x143053fc8
14237ac9f:     	movslq	(%rdx,%rcx,4), %rcx
14237aca3:     	addq	%rdx, %rcx
14237aca6:     	jmpq	*%rcx
14237aca8:     	movzbl	%al, %eax
14237acab:     	shll	$0x4, %eax
14237acae:     	leaq	0xcd907b(%rip), %r8     # 0x143053d30
14237acb5:     	movq	0x8(%rax,%r8), %rdx
14237acba:     	cmpq	$0x2, %rdx
14237acbe:     	jbe	0x14237bb77 <.text+0x237ab77>
14237acc4:     	addq	%rax, %r8
14237acc7:     	movq	(%r8), %rdx
14237acca:     	leaq	0xcd8f47(%rip), %r9     # 0x143053c18
14237acd1:     	movl	$0x3, %edi
14237acd6:     	movl	$0x3, %r8d
14237acdc:     	jmp	0x14237b2bc <.text+0x237a2bc>
14237ace1:     	testb	$0x1, %cl
14237ace4:     	jne	0x14237aeaf <.text+0x2379eaf>
14237acea:     	shrq	$0x28, %r14
14237acee:     	movq	%rdx, %rcx
14237acf1:     	movl	%r14d, %edx
14237acf4:     	jmp	0x14237b06f <.text+0x237a06f>
14237acf9:     	testl	%ebx, %ebx
14237acfb:     	je	0x14237aeaf <.text+0x2379eaf>
14237ad01:     	movl	%ebx, %ecx
14237ad03:     	movq	%rdx, %rbx
14237ad06:     	callq	0x140cb0ed0 <.text+0xcafed0>
14237ad0b:     	movq	%rbx, %rcx
14237ad0e:     	jmp	0x14237b06d <.text+0x237a06d>
14237ad13:     	testl	%ebx, %ebx
14237ad15:     	je	0x14237aeaf <.text+0x2379eaf>
14237ad1b:     	movq	%rdx, %rdi
14237ad1e:     	movl	%ebx, %ecx
14237ad20:     	callq	0x140cb0ec0 <.text+0xcafec0>
14237ad25:     	testl	%r15d, %r15d
14237ad28:     	je	0x14237b05b <.text+0x237a05b>
14237ad2e:     	cmpl	$0x1, %r15d
14237ad32:     	movq	%rdi, %rcx
14237ad35:     	movzbl	%al, %eax
14237ad38:     	jne	0x14237b0d4 <.text+0x237a0d4>
14237ad3e:     	shll	$0x4, %eax
14237ad41:     	leaq	0xcd9130(%rip), %r8     # 0x143053e78
14237ad48:     	movq	-0x10(%rax,%r8), %rdx
14237ad4d:     	movq	-0x8(%rax,%r8), %rdi
14237ad52:     	jmp	0x14237b66e <.text+0x237a66e>
14237ad57:     	testl	%ebx, %ebx
14237ad59:     	je	0x14237aeaf <.text+0x2379eaf>
14237ad5f:     	testl	$0x100, %r13d           # imm = 0x100
14237ad66:     	jne	0x14237af18 <.text+0x2379f18>
14237ad6c:     	sarl	$0x9, %ebx
14237ad6f:     	jmp	0x14237af27 <.text+0x2379f27>
14237ad74:     	testb	$0x1, %al
14237ad76:     	je	0x14237aeaf <.text+0x2379eaf>
14237ad7c:     	movl	%eax, %ecx
14237ad7e:     	shrl	$0x18, %ecx
14237ad81:     	movl	%ecx, %r8d
14237ad84:     	negb	%r8b
14237ad87:     	testl	%eax, %eax
14237ad89:     	movzbl	%r8b, %eax
14237ad8d:     	cmovnsl	%ecx, %eax
14237ad90:     	movq	%rdx, %rcx
14237ad93:     	movl	%eax, %edx
14237ad95:     	jmp	0x14237b06f <.text+0x237a06f>
14237ad9a:     	testb	$0x1, %cl
14237ad9d:     	jne	0x14237aeaf <.text+0x2379eaf>
14237ada3:     	shrq	$0x30, %r14
14237ada7:     	testb	$0x1, %dil
14237adab:     	je	0x14237af83 <.text+0x2379f83>
14237adb1:     	movb	$0xc, %al
14237adb3:     	testb	%r14b, %r14b
14237adb6:     	je	0x14237af86 <.text+0x2379f86>
14237adbc:     	movzbl	%r14b, %ecx
14237adc0:     	cmpl	$0xc, %ecx
14237adc3:     	je	0x14237af86 <.text+0x2379f86>
14237adc9:     	subb	$0xc, %r14b
14237adcd:     	movzbl	%r14b, %eax
14237add1:     	cmovbl	%ecx, %eax
14237add4:     	jmp	0x14237af86 <.text+0x2379f86>
14237add9:     	testb	$0x1, %cl
14237addc:     	jne	0x14237aeaf <.text+0x2379eaf>
14237ade2:     	cmpb	$0x8, %dil
14237ade6:     	je	0x14237ae13 <.text+0x2379e13>
14237ade8:     	movl	%r14d, %r13d
14237adeb:     	movl	$0xcccccccd, %ebx       # imm = 0xCCCCCCCD
14237adf0:     	imulq	%r13, %rbx
14237adf4:     	shrq	$0x23, %rbx
14237adf8:     	cmpb	$0x9, %dil
14237adfc:     	jne	0x14237b433 <.text+0x237a433>
14237ae02:     	leal	(%rbx,%rbx), %eax
14237ae05:     	leal	(%rax,%rax,4), %eax
14237ae08:     	movl	%r14d, %ecx
14237ae0b:     	subl	%eax, %ecx
14237ae0d:     	je	0x14237b433 <.text+0x237a433>
14237ae13:     	movq	%rdx, %r15
14237ae16:     	movl	%r14d, %ecx
14237ae19:     	callq	0x140cb3fe0 <.text+0xcb2fe0>
14237ae1e:     	cmpb	$0x8, %al
14237ae20:     	jbe	0x14237b161 <.text+0x237a161>
14237ae26:     	xorl	%edi, %edi
14237ae28:     	movq	%r15, %rax
14237ae2b:     	movl	$0xa, %ecx
14237ae30:     	cmpl	$0x2710, %r14d          # imm = 0x2710
14237ae37:     	jb	0x14237b1c4 <.text+0x237a1c4>
14237ae3d:     	movl	$0xa, %r10d
14237ae43:     	movl	$0xd1b71759, %edx       # imm = 0xD1B71759
14237ae48:     	leaq	0xcd8de1(%rip), %r9     # 0x143053c30
14237ae4f:     	nop
14237ae50:     	movl	%r14d, %r8d
14237ae53:     	imulq	%rdx, %r8
14237ae57:     	shrq	$0x2d, %r8
14237ae5b:     	imull	$0x2710, %r8d, %ecx     # imm = 0x2710
14237ae62:     	movl	%r14d, %r11d
14237ae65:     	subl	%ecx, %r11d
14237ae68:     	imull	$0x147b, %r11d, %ebx    # imm = 0x147B
14237ae6f:     	shrl	$0x13, %ebx
14237ae72:     	imull	$0x64, %ebx, %ecx
14237ae75:     	subl	%ecx, %r11d
14237ae78:     	movzwl	%r11w, %r11d
14237ae7c:     	leaq	-0x4(%r10), %rcx
14237ae80:     	movzwl	(%r9,%rbx,2), %ebx
14237ae85:     	movw	%bx, 0x2c(%rsp,%r10)
14237ae8b:     	movzwl	(%r9,%r11,2), %r11d
14237ae90:     	movw	%r11w, 0x2e(%rsp,%r10)
14237ae96:     	movq	%rcx, %r10
14237ae99:     	cmpl	$0x5f5e0ff, %r14d       # imm = 0x5F5E0FF
14237aea0:     	movl	%r8d, %r14d
14237aea3:     	ja	0x14237ae50 <.text+0x2379e50>
14237aea5:     	jmp	0x14237b1c7 <.text+0x237a1c7>
14237aeaa:     	testb	$0x1, %cl
14237aead:     	je	0x14237aebb <.text+0x2379ebb>
14237aeaf:     	movq	$0x0, (%rsi)
14237aeb6:     	jmp	0x14237b085 <.text+0x237a085>
14237aebb:     	shrq	$0x30, %r14
14237aebf:     	movq	(%rdx), %rcx
14237aec2:     	movq	0x10(%rdx), %rax
14237aec6:     	subq	%rax, %rcx
14237aec9:     	cmpq	$0x1, %rcx
14237aecd:     	jbe	0x14237bad9 <.text+0x237aad9>
14237aed3:     	testb	$0x1, %dil
14237aed7:     	movl	$0x4d50, %ecx           # imm = 0x4D50
14237aedc:     	movl	$0x6d70, %r8d           # imm = 0x6D70
14237aee2:     	cmovnel	%ecx, %r8d
14237aee6:     	movl	$0x4d41, %ecx           # imm = 0x4D41
14237aeeb:     	movl	$0x6d61, %r9d           # imm = 0x6D61
14237aef1:     	cmovnel	%ecx, %r9d
14237aef5:     	cmpb	$0xc, %r14b
14237aef9:     	movq	0x8(%rdx), %rcx
14237aefd:     	cmovael	%r8d, %r9d
14237af01:     	movw	%r9w, (%rcx,%rax)
14237af06:     	addq	$0x2, %rax
14237af0a:     	movq	%rax, 0x10(%rdx)
14237af0e:     	movl	$0x2, %edi
14237af13:     	jmp	0x14237b07a <.text+0x237a07a>
14237af18:     	movl	%ebx, %ecx
14237af1a:     	movq	%rdx, %rdi
14237af1d:     	callq	0x140cb1010 <.text+0xcb0010>
14237af22:     	movq	%rdi, %rdx
14237af25:     	movl	%eax, %ebx
14237af27:     	testb	$0x1, %r13b
14237af2b:     	je	0x14237af60 <.text+0x2379f60>
14237af2d:     	movslq	%ebx, %rax
14237af30:     	imulq	$0x51eb851f, %rax, %rax # imm = 0x51EB851F
14237af37:     	movq	%rax, %rcx
14237af3a:     	shrq	$0x3f, %rcx
14237af3e:     	sarq	$0x25, %rax
14237af42:     	addl	%ecx, %eax
14237af44:     	imull	$0x64, %eax, %eax
14237af47:     	subl	%eax, %ebx
14237af49:     	movl	%ebx, %eax
14237af4b:     	negl	%eax
14237af4d:     	cmovsl	%ebx, %eax
14237af50:     	leaq	-0xfa7(%rip), %r14      # 0x142379fb0 <.text+0x2378fb0>
14237af57:     	xorl	%edi, %edi
14237af59:     	movl	%eax, %ebx
14237af5b:     	jmp	0x14237b4e7 <.text+0x237a4e7>
14237af60:     	testl	%ebx, %ebx
14237af62:     	js	0x14237b46f <.text+0x237a46f>
14237af68:     	leaq	-0x91f(%rip), %r14      # 0x14237a650 <.text+0x2379650>
14237af6f:     	testl	$0x10000, %r13d         # imm = 0x10000
14237af76:     	jne	0x14237b4bd <.text+0x237a4bd>
14237af7c:     	xorl	%edi, %edi
14237af7e:     	jmp	0x14237b4e7 <.text+0x237a4e7>
14237af83:     	movl	%r14d, %eax
14237af86:     	movq	%rdx, %rcx
14237af89:     	movl	%eax, %edx
14237af8b:     	jmp	0x14237b060 <.text+0x237a060>
14237af90:     	movl	%ebx, %ecx
14237af92:     	callq	0x140cb1160 <.text+0xcb0160>
14237af97:     	jmp	0x14237b06a <.text+0x237a06a>
14237af9c:     	testw	%bx, %bx
14237af9f:     	je	0x14237b58a <.text+0x237a58a>
14237afa5:     	leal	0x5fff6(%rbx), %eax
14237afab:     	leal	0x7ff9c(%rbx), %ecx
14237afb1:     	andl	%eax, %ecx
14237afb3:     	leal	0x5fc18(%rbx), %r8d
14237afba:     	movl	%ebx, %eax
14237afbc:     	addl	$0x7d8f0, %eax          # imm = 0x7D8F0
14237afc1:     	andl	%r8d, %eax
14237afc4:     	xorl	%ecx, %eax
14237afc6:     	cmpl	$0x3ffff, %eax          # imm = 0x3FFFF
14237afcb:     	jbe	0x14237b58e <.text+0x237a58e>
14237afd1:     	xorl	%r14d, %r14d
14237afd4:     	movl	$0x5, %eax
14237afd9:     	cmpw	$0x64, %bx
14237afdd:     	jb	0x14237b008 <.text+0x237a008>
14237afdf:     	movzwl	%bx, %eax
14237afe2:     	imull	$0x290, %eax, %ebx      # imm = 0x290
14237afe8:     	shrl	$0x10, %ebx
14237afeb:     	imull	$0x64, %ebx, %ecx
14237afee:     	subl	%ecx, %eax
14237aff0:     	movzwl	%ax, %eax
14237aff3:     	leaq	0xcd8c36(%rip), %rcx    # 0x143053c30
14237affa:     	movzwl	(%rcx,%rax,2), %eax
14237affe:     	movw	%ax, 0x33(%rsp)
14237b003:     	movl	$0x3, %eax
14237b008:     	cmpw	$0xa, %bx
14237b00c:     	jae	0x14237b01a <.text+0x237a01a>
14237b00e:     	orb	$0x30, %bl
14237b011:     	movb	%bl, 0x2f(%rsp,%rax)
14237b015:     	decq	%rax
14237b018:     	jmp	0x14237b032 <.text+0x237a032>
14237b01a:     	movzwl	%bx, %ecx
14237b01d:     	leaq	0xcd8c0c(%rip), %r8     # 0x143053c30
14237b024:     	movzwl	(%r8,%rcx,2), %ecx
14237b029:     	movw	%cx, 0x2e(%rsp,%rax)
14237b02e:     	addq	$-0x2, %rax
14237b032:     	movl	$0x5, %edi
14237b037:     	subq	%rax, %rdi
14237b03a:     	addq	%rsp, %rax
14237b03d:     	addq	$0x30, %rax
14237b041:     	leaq	0xcd8bd0(%rip), %r9     # 0x143053c18
14237b048:     	movq	%rdx, %rcx
14237b04b:     	movq	%rax, %rdx
14237b04e:     	movq	%rdi, %r8
14237b051:     	callq	0x140fcd9c0 <.text+0xfcc9c0>
14237b056:     	addq	%r14, %rdi
14237b059:     	jmp	0x14237b07a <.text+0x237a07a>
14237b05b:     	movq	%rdi, %rcx
14237b05e:     	movl	%eax, %edx
14237b060:     	movl	%ebp, %r8d
14237b063:     	jmp	0x14237b072 <.text+0x237a072>
14237b065:     	callq	0x140cb1270 <.text+0xcb0270>
14237b06a:     	movq	%r14, %rcx
14237b06d:     	movl	%eax, %edx
14237b06f:     	movl	%edi, %r8d
14237b072:     	callq	0x14237a320 <.text+0x2379320>
14237b077:     	movq	%rax, %rdi
14237b07a:     	movq	%rdi, 0x8(%rsi)
14237b07e:     	movq	$0x3, (%rsi)
14237b085:     	movq	%rsi, %rax
14237b088:     	addq	$0xb8, %rsp
14237b08f:     	popq	%rbx
14237b090:     	popq	%rbp
14237b091:     	popq	%rdi
14237b092:     	popq	%rsi
14237b093:     	popq	%r12
14237b095:     	popq	%r13
14237b097:     	popq	%r14
14237b099:     	popq	%r15
14237b09b:     	retq
14237b09c:     	cmpw	$0x64, %bx
14237b0a0:     	jb	0x14237b4a2 <.text+0x237a4a2>
14237b0a6:     	movzwl	%bx, %eax
14237b0a9:     	imull	$0x290, %eax, %ebx      # imm = 0x290
14237b0af:     	shrl	$0x10, %ebx
14237b0b2:     	imull	$0x64, %ebx, %ecx
14237b0b5:     	subl	%ecx, %eax
14237b0b7:     	movzwl	%ax, %eax
14237b0ba:     	leaq	0xcd8b6f(%rip), %rcx    # 0x143053c30
14237b0c1:     	movzwl	(%rcx,%rax,2), %eax
14237b0c5:     	movw	%ax, 0x33(%rsp)
14237b0ca:     	movl	$0x2, %eax
14237b0cf:     	jmp	0x14237b4b1 <.text+0x237a4b1>
14237b0d4:     	shll	$0x4, %eax
14237b0d7:     	leaq	0xcd8d9a(%rip), %r8     # 0x143053e78
14237b0de:     	movq	-0x8(%rax,%r8), %rdx
14237b0e3:     	cmpq	$0x2, %rdx
14237b0e7:     	jbe	0x14237bb66 <.text+0x237ab66>
14237b0ed:     	addq	%rax, %r8
14237b0f0:     	movq	-0x10(%r8), %rdx
14237b0f4:     	leaq	0xcd8b1d(%rip), %r9     # 0x143053c18
14237b0fb:     	movl	$0x3, %edi
14237b100:     	movl	$0x3, %r8d
14237b106:     	callq	0x140fcd9c0 <.text+0xfcc9c0>
14237b10b:     	jmp	0x14237b07a <.text+0x237a07a>
14237b110:     	leaq	0xcd8d02(%rip), %rbx    # 0x143053e19
14237b117:     	movq	0x10(%rdx), %rax
14237b11b:     	cmpq	%rax, (%rdx)
14237b11e:     	je	0x14237bb37 <.text+0x237ab37>
14237b124:     	movq	0x8(%rdx), %rcx
14237b128:     	movzbl	(%rbx), %r8d
14237b12c:     	movb	%r8b, (%rcx,%rax)
14237b130:     	incq	%rax
14237b133:     	movq	%rax, 0x10(%rdx)
14237b137:     	movl	$0x1, %ebx
14237b13c:     	movl	%r12d, %eax
14237b13f:     	sarb	$0x7, %al
14237b142:     	xorb	%al, %r12b
14237b145:     	subb	%al, %r12b
14237b148:     	movq	%rdx, %rcx
14237b14b:     	movl	%r12d, %edx
14237b14e:     	movl	%ebp, %r8d
14237b151:     	callq	0x14237a320 <.text+0x2379320>
14237b156:     	movq	%rax, %rdi
14237b159:     	addq	%rbx, %rdi
14237b15c:     	jmp	0x14237b07a <.text+0x237a07a>
14237b161:     	movb	$0x9, %cl
14237b163:     	subb	%al, %cl
14237b165:     	movzbl	%cl, %edi
14237b168:     	movq	%r15, %rax
14237b16b:     	movq	0x10(%r15), %rdx
14237b16f:     	movq	%rdi, %rbx
14237b172:     	nopw	%cs:(%rax,%rax)
14237b180:     	cmpq	%rdx, (%rax)
14237b183:     	je	0x14237b19e <.text+0x237a19e>
14237b185:     	movq	0x8(%rax), %rcx
14237b189:     	movb	$0x30, (%rcx,%rdx)
14237b18d:     	incq	%rdx
14237b190:     	movq	%rdx, 0x10(%rax)
14237b194:     	decq	%rbx
14237b197:     	jne	0x14237b180 <.text+0x237a180>
14237b199:     	jmp	0x14237ae2b <.text+0x2379e2b>
14237b19e:     	movq	$0x1, 0x20(%rsp)
14237b1a7:     	movl	$0x1, %r8d
14237b1ad:     	movl	$0x1, %r9d
14237b1b3:     	movq	%rax, %rcx
14237b1b6:     	callq	0x1428a58d0 <.text+0x28a48d0>
14237b1bb:     	movq	%r15, %rax
14237b1be:     	movq	0x10(%r15), %rdx
14237b1c2:     	jmp	0x14237b185 <.text+0x237a185>
14237b1c4:     	movl	%r14d, %r8d
14237b1c7:     	movl	%r8d, %edx
14237b1ca:     	cmpl	$0x63, %r8d
14237b1ce:     	jbe	0x14237b205 <.text+0x237a205>
14237b1d0:     	movzwl	%dx, %r8d
14237b1d4:     	shrl	$0x2, %r8d
14237b1d8:     	imull	$0x147b, %r8d, %r8d     # imm = 0x147B
14237b1df:     	shrl	$0x11, %r8d
14237b1e3:     	imull	$0x64, %r8d, %r9d
14237b1e7:     	subl	%r9d, %edx
14237b1ea:     	movzwl	%dx, %edx
14237b1ed:     	leaq	0xcd8a3c(%rip), %r9     # 0x143053c30
14237b1f4:     	movzwl	(%r9,%rdx,2), %edx
14237b1f9:     	movw	%dx, 0x2e(%rsp,%rcx)
14237b1fe:     	addq	$-0x2, %rcx
14237b202:     	movq	%r8, %rdx
14237b205:     	cmpl	$0xa, %edx
14237b208:     	jae	0x14237b216 <.text+0x237a216>
14237b20a:     	orb	$0x30, %dl
14237b20d:     	movb	%dl, 0x2f(%rsp,%rcx)
14237b211:     	decq	%rcx
14237b214:     	jmp	0x14237b22b <.text+0x237a22b>
14237b216:     	leaq	0xcd8a13(%rip), %r8     # 0x143053c30
14237b21d:     	movzwl	(%r8,%rdx,2), %edx
14237b222:     	movw	%dx, 0x2e(%rsp,%rcx)
14237b227:     	addq	$-0x2, %rcx
14237b22b:     	movl	$0xa, %ebx
14237b230:     	subq	%rcx, %rbx
14237b233:     	leaq	(%rsp,%rcx), %rdx
14237b237:     	addq	$0x30, %rdx
14237b23b:     	movq	(%rax), %rcx
14237b23e:     	movq	0x10(%rax), %r14
14237b242:     	subq	%r14, %rcx
14237b245:     	cmpq	%rbx, %rcx
14237b248:     	jb	0x14237bb08 <.text+0x237ab08>
14237b24e:     	movq	0x8(%rax), %rcx
14237b252:     	addq	%r14, %rcx
14237b255:     	movq	%rax, %r15
14237b258:     	movq	%rbx, %r8
14237b25b:     	callq	0x142923c00 <.text+0x2922c00>
14237b260:     	addq	%rbx, %r14
14237b263:     	movq	%r14, 0x10(%r15)
14237b267:     	addq	%rbx, %rdi
14237b26a:     	jmp	0x14237b07a <.text+0x237a07a>
14237b26f:     	shlb	$0x3, %al
14237b272:     	movabsq	$0x60504030201, %rdx    # imm = 0x60504030201
14237b27c:     	movl	%eax, %ecx
14237b27e:     	shrq	%cl, %rdx
14237b281:     	andb	$0x1, %dil
14237b285:     	addb	%dl, %dil
14237b288:     	leaq	0x32(%rsp), %rdx
14237b28d:     	orb	$0x30, %dil
14237b291:     	movb	%dil, 0x32(%rsp)
14237b296:     	jmp	0x14237b2aa <.text+0x237a2aa>
14237b298:     	andb	$0x1, %dil
14237b29c:     	addb	%dil, %al
14237b29f:     	leaq	0x32(%rsp), %rdx
14237b2a4:     	orb	$0x30, %al
14237b2a6:     	movb	%al, 0x32(%rsp)
14237b2aa:     	leaq	0xcd8967(%rip), %r9     # 0x143053c18
14237b2b1:     	movl	$0x1, %edi
14237b2b6:     	movl	$0x1, %r8d
14237b2bc:     	movq	%r14, %rcx
14237b2bf:     	callq	0x140fcd9c0 <.text+0xfcc9c0>
14237b2c4:     	jmp	0x14237b07a <.text+0x237a07a>
14237b2c9:     	movzbl	%al, %eax
14237b2cc:     	shll	$0x4, %eax
14237b2cf:     	leaq	0xcd8a5a(%rip), %rcx    # 0x143053d30
14237b2d6:     	movq	(%rax,%rcx), %rdx
14237b2da:     	movq	0x8(%rax,%rcx), %rdi
14237b2df:     	leaq	0xcd8932(%rip), %r9     # 0x143053c18
14237b2e6:     	movq	%r14, %rcx
14237b2e9:     	jmp	0x14237b6ab <.text+0x237a6ab>
14237b2ee:     	movdqu	0x58(%rsp), %xmm0
14237b2f4:     	movdqa	%xmm0, 0x30(%rsp)
14237b2fa:     	leaq	0x30(%rsp), %rcx
14237b2ff:     	callq	0x140cb0030 <.text+0xcaf030>
14237b304:     	movq	%rax, %rdi
14237b307:     	movq	%rdx, %rbx
14237b30a:     	movq	%rdx, 0xa8(%rsp)
14237b312:     	movq	%rax, 0xa0(%rsp)
14237b31a:     	movq	$0x3e8, 0x90(%rsp)      # imm = 0x3E8
14237b326:     	movq	$0x0, 0x98(%rsp)
14237b332:     	leaq	0xa0(%rsp), %rcx
14237b33a:     	leaq	0x90(%rsp), %rdx
14237b342:     	callq	0x140d8f0a0 <.text+0xd8e0a0>
14237b347:     	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
14237b34c:     	movq	%xmm1, %rax
14237b351:     	movq	%xmm0, %rcx
14237b356:     	xorl	%r8d, %r8d
14237b359:     	movq	%rcx, %rdx
14237b35c:     	negq	%rdx
14237b35f:     	sbbq	%rax, %r8
14237b362:     	cmpq	$-0x3e7, %rdi           # imm = 0xFC19
14237b369:     	jmp	0x14237b417 <.text+0x237a417>
14237b36e:     	movdqu	0x58(%rsp), %xmm0
14237b374:     	movdqa	%xmm0, 0x30(%rsp)
14237b37a:     	leaq	0x30(%rsp), %rcx
14237b37f:     	callq	0x140cb0030 <.text+0xcaf030>
14237b384:     	movq	%rdx, %r8
14237b387:     	movq	%rdx, %rcx
14237b38a:     	sarq	$0x3f, %rcx
14237b38e:     	xorq	%rcx, %r8
14237b391:     	xorq	%rcx, %rax
14237b394:     	subq	%rcx, %rax
14237b397:     	sbbq	%rcx, %r8
14237b39a:     	movq	%r15, %rcx
14237b39d:     	movq	%rax, %rdx
14237b3a0:     	jmp	0x14237b426 <.text+0x237a426>
14237b3a5:     	movdqu	0x58(%rsp), %xmm0
14237b3ab:     	movdqa	%xmm0, 0x30(%rsp)
14237b3b1:     	leaq	0x30(%rsp), %rcx
14237b3b6:     	callq	0x140cb0030 <.text+0xcaf030>
14237b3bb:     	movq	%rax, %rdi
14237b3be:     	movq	%rdx, %rbx
14237b3c1:     	movq	%rdx, 0x88(%rsp)
14237b3c9:     	movq	%rax, 0x80(%rsp)
14237b3d1:     	movq	$0xf4240, 0x70(%rsp)    # imm = 0xF4240
14237b3da:     	movq	$0x0, 0x78(%rsp)
14237b3e3:     	leaq	0x80(%rsp), %rcx
14237b3eb:     	leaq	0x70(%rsp), %rdx
14237b3f0:     	callq	0x140d8f0a0 <.text+0xd8e0a0>
14237b3f5:     	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
14237b3fa:     	movq	%xmm1, %rax
14237b3ff:     	movq	%xmm0, %rcx
14237b404:     	xorl	%r8d, %r8d
14237b407:     	movq	%rcx, %rdx
14237b40a:     	negq	%rdx
14237b40d:     	sbbq	%rax, %r8
14237b410:     	cmpq	$-0xf423f, %rdi         # imm = 0xFFF0BDC1
14237b417:     	sbbq	$-0x1, %rbx
14237b41b:     	cmovgeq	%rcx, %rdx
14237b41f:     	cmovgeq	%rax, %r8
14237b423:     	movq	%r15, %rcx
14237b426:     	callq	0x14237bb90 <.text+0x237ab90>
14237b42b:     	movq	%rdx, %rdi
14237b42e:     	jmp	0x14237b07a <.text+0x237a07a>
14237b433:     	movzbl	%dil, %eax
14237b437:     	decl	%eax
14237b439:     	cmpl	$0x8, %eax
14237b43c:     	ja	0x14237babc <.text+0x237aabc>
14237b442:     	leaq	0xcd8b5b(%rip), %rcx    # 0x143053fa4
14237b449:     	movslq	(%rcx,%rax,4), %rax
14237b44d:     	addq	%rcx, %rax
14237b450:     	jmpq	*%rax
14237b452:     	imulq	$0x6b5fca6b, %r13, %rax # imm = 0x6B5FCA6B
14237b459:     	shrq	$0x36, %rax
14237b45d:     	movq	%rdx, %rcx
14237b460:     	movl	%eax, %edx
14237b462:     	callq	0x14237c3d0 <.text+0x237b3d0>
14237b467:     	movq	%rdx, %rdi
14237b46a:     	jmp	0x14237b07a <.text+0x237a07a>
14237b46f:     	leaq	0xcd89a3(%rip), %rax    # 0x143053e19
14237b476:     	leaq	0xcd879b(%rip), %r9     # 0x143053c18
14237b47d:     	movl	$0x1, %edi
14237b482:     	movl	$0x1, %r8d
14237b488:     	movq	%rdx, %rcx
14237b48b:     	movq	%rdx, %r14
14237b48e:     	movq	%rax, %rdx
14237b491:     	callq	0x140fcd9c0 <.text+0xfcc9c0>
14237b496:     	movq	%r14, %rdx
14237b499:     	leaq	-0xe50(%rip), %r14      # 0x14237a650 <.text+0x2379650>
14237b4a0:     	jmp	0x14237b4e7 <.text+0x237a4e7>
14237b4a2:     	movl	$0x4, %eax
14237b4a7:     	cmpw	$0xa, %bx
14237b4ab:     	jae	0x14237b677 <.text+0x237a677>
14237b4b1:     	orb	$0x30, %bl
14237b4b4:     	movb	%bl, 0x30(%rsp,%rax)
14237b4b8:     	jmp	0x14237b68f <.text+0x237a68f>
14237b4bd:     	leaq	0xcd8954(%rip), %rax    # 0x143053e18
14237b4c4:     	leaq	0xcd874d(%rip), %r9     # 0x143053c18
14237b4cb:     	movl	$0x1, %edi
14237b4d0:     	movl	$0x1, %r8d
14237b4d6:     	movq	%rdx, %rcx
14237b4d9:     	movq	%rdx, %r12
14237b4dc:     	movq	%rax, %rdx
14237b4df:     	callq	0x140fcd9c0 <.text+0xfcc9c0>
14237b4e4:     	movq	%r12, %rdx
14237b4e7:     	movl	%ebx, %eax
14237b4e9:     	negl	%eax
14237b4eb:     	cmovsl	%ebx, %eax
14237b4ee:     	movq	%rdx, %rcx
14237b4f1:     	movl	%eax, %edx
14237b4f3:     	movl	%r15d, %r8d
14237b4f6:     	callq	*%r14
14237b4f9:     	addq	%rdx, %rdi
14237b4fc:     	testq	%rax, %rax
14237b4ff:     	cmovneq	%rdx, %rdi
14237b503:     	je	0x14237b07a <.text+0x237a07a>
14237b509:     	movq	$0x2, (%rsi)
14237b510:     	movq	%rdx, 0x8(%rsi)
14237b514:     	jmp	0x14237b085 <.text+0x237a085>
14237b519:     	movb	$0x2, %cl
14237b51b:     	movzbl	%cl, %eax
14237b51e:     	cmpb	$0x2, %al
14237b520:     	movl	$0x1, %ecx
14237b525:     	cmovael	%eax, %ecx
14237b528:     	movzbl	%cl, %r14d
14237b52c:     	movq	0x10(%rdx), %rax
14237b530:     	movq	%r14, %rdi
14237b533:     	nopw	%cs:(%rax,%rax)
14237b540:     	cmpq	%rax, (%rdx)
14237b543:     	je	0x14237b55e <.text+0x237a55e>
14237b545:     	movq	0x8(%rdx), %rcx
14237b549:     	movb	$0x30, (%rcx,%rax)
14237b54d:     	incq	%rax
14237b550:     	movq	%rax, 0x10(%rdx)
14237b554:     	decq	%rdi
14237b557:     	jne	0x14237b540 <.text+0x237a540>
14237b559:     	jmp	0x14237afd4 <.text+0x2379fd4>
14237b55e:     	movq	$0x1, 0x20(%rsp)
14237b567:     	movl	$0x1, %r8d
14237b56d:     	movl	$0x1, %r9d
14237b573:     	movq	%rdx, %rcx
14237b576:     	movq	%rdx, %r15
14237b579:     	movq	%rax, %rdx
14237b57c:     	callq	0x1428a58d0 <.text+0x28a48d0>
14237b581:     	movq	%r15, %rdx
14237b584:     	movq	0x10(%r15), %rax
14237b588:     	jmp	0x14237b545 <.text+0x237a545>
14237b58a:     	movb	$0x2, %cl
14237b58c:     	jmp	0x14237b595 <.text+0x237a595>
14237b58e:     	shrl	$0x11, %eax
14237b591:     	movb	$0x2, %cl
14237b593:     	subb	%al, %cl
14237b595:     	movzbl	%cl, %eax
14237b598:     	cmpb	$0x2, %al
14237b59a:     	movl	$0x1, %ecx
14237b59f:     	cmovael	%eax, %ecx
14237b5a2:     	movzbl	%cl, %r14d
14237b5a6:     	movq	0x10(%rdx), %rax
14237b5aa:     	movq	%r14, %rdi
14237b5ad:     	nopl	(%rax)
14237b5b0:     	cmpq	%rax, (%rdx)
14237b5b3:     	je	0x14237b5ce <.text+0x237a5ce>
14237b5b5:     	movq	0x8(%rdx), %rcx
14237b5b9:     	movb	$0x20, (%rcx,%rax)
14237b5bd:     	incq	%rax
14237b5c0:     	movq	%rax, 0x10(%rdx)
14237b5c4:     	decq	%rdi
14237b5c7:     	jne	0x14237b5b0 <.text+0x237a5b0>
14237b5c9:     	jmp	0x14237afd4 <.text+0x2379fd4>
14237b5ce:     	movq	$0x1, 0x20(%rsp)
14237b5d7:     	movl	$0x1, %r8d
14237b5dd:     	movl	$0x1, %r9d
14237b5e3:     	movq	%rdx, %rcx
14237b5e6:     	movq	%rdx, %r15
14237b5e9:     	movq	%rax, %rdx
14237b5ec:     	callq	0x1428a58d0 <.text+0x28a48d0>
14237b5f1:     	movq	%r15, %rdx
14237b5f4:     	movq	0x10(%r15), %rax
14237b5f8:     	jmp	0x14237b5b5 <.text+0x237a5b5>
14237b5fa:     	movq	%rcx, %rdx
14237b5fd:     	cmpq	$0x63, %rdx
14237b601:     	jbe	0x14237b634 <.text+0x237a634>
14237b603:     	movzwl	%dx, %ecx
14237b606:     	shrl	$0x2, %ecx
14237b609:     	imull	$0x147b, %ecx, %ecx     # imm = 0x147B
14237b60f:     	shrl	$0x11, %ecx
14237b612:     	imull	$0x64, %ecx, %r8d
14237b616:     	subl	%r8d, %edx
14237b619:     	movzwl	%dx, %edx
14237b61c:     	leaq	0xcd860d(%rip), %r8     # 0x143053c30
14237b623:     	movzwl	(%r8,%rdx,2), %edx
14237b628:     	movw	%dx, 0x2e(%rsp,%rax)
14237b62d:     	addq	$-0x2, %rax
14237b631:     	movq	%rcx, %rdx
14237b634:     	movq	%r15, %rcx
14237b637:     	cmpq	$0xa, %rdx
14237b63b:     	jae	0x14237b649 <.text+0x237a649>
14237b63d:     	orb	$0x30, %dl
14237b640:     	movb	%dl, 0x2f(%rsp,%rax)
14237b644:     	decq	%rax
14237b647:     	jmp	0x14237b65e <.text+0x237a65e>
14237b649:     	leaq	0xcd85e0(%rip), %r8     # 0x143053c30
14237b650:     	movzwl	(%r8,%rdx,2), %edx
14237b655:     	movw	%dx, 0x2e(%rsp,%rax)
14237b65a:     	addq	$-0x2, %rax
14237b65e:     	movl	$0x14, %edi
14237b663:     	subq	%rax, %rdi
14237b666:     	leaq	(%rsp,%rax), %rdx
14237b66a:     	addq	$0x30, %rdx
14237b66e:     	leaq	0xcd85a3(%rip), %r9     # 0x143053c18
14237b675:     	jmp	0x14237b6ab <.text+0x237a6ab>
14237b677:     	movzwl	%bx, %eax
14237b67a:     	leaq	0xcd85af(%rip), %rcx    # 0x143053c30
14237b681:     	movzwl	(%rcx,%rax,2), %eax
14237b685:     	movw	%ax, 0x33(%rsp)
14237b68a:     	movl	$0x3, %eax
14237b68f:     	movl	$0x5, %edi
14237b694:     	subq	%rax, %rdi
14237b697:     	addq	%rsp, %rax
14237b69a:     	addq	$0x30, %rax
14237b69e:     	leaq	0xcd8573(%rip), %r9     # 0x143053c18
14237b6a5:     	movq	%rdx, %rcx
14237b6a8:     	movq	%rax, %rdx
14237b6ab:     	movq	%rdi, %r8
14237b6ae:     	callq	0x140fcd9c0 <.text+0xfcc9c0>
14237b6b3:     	jmp	0x14237b07a <.text+0x237a07a>
14237b6b8:     	imulq	$0x51eb851f, %r13, %rdi # imm = 0x51EB851F
14237b6bf:     	shrq	$0x25, %rdi
14237b6c3:     	movq	%rdx, %rbx
14237b6c6:     	movl	%edi, %ecx
14237b6c8:     	callq	0x140cb3fe0 <.text+0xcb2fe0>
14237b6cd:     	cmpb	$0x6, %al
14237b6cf:     	jbe	0x14237b8c7 <.text+0x237a8c7>
14237b6d5:     	xorl	%r15d, %r15d
14237b6d8:     	jmp	0x14237b8ff <.text+0x237a8ff>
14237b6dd:     	imulq	$0x431bde83, %r13, %rax # imm = 0x431BDE83
14237b6e4:     	shrq	$0x32, %rax
14237b6e8:     	movq	%rdx, %rcx
14237b6eb:     	movl	%eax, %edx
14237b6ed:     	callq	0x14237c280 <.text+0x237b280>
14237b6f2:     	movq	%rdx, %rdi
14237b6f5:     	jmp	0x14237b07a <.text+0x237a07a>
14237b6fa:     	movl	$0xcccccccd, %ebx       # imm = 0xCCCCCCCD
14237b6ff:     	imulq	%r13, %rbx
14237b703:     	shrq	$0x23, %rbx
14237b707:     	jmp	0x14237b71c <.text+0x237a71c>
14237b709:     	imull	$0xcccccccd, %ebx, %eax # imm = 0xCCCCCCCD
14237b70f:     	rorl	%eax
14237b711:     	cmpl	$0x19999999, %eax       # imm = 0x19999999
14237b716:     	jbe	0x14237b9fb <.text+0x237a9fb>
14237b71c:     	movq	%rdx, %rdi
14237b71f:     	movl	%ebx, %ecx
14237b721:     	callq	0x140cb3fe0 <.text+0xcb2fe0>
14237b726:     	cmpb	$0x7, %al
14237b728:     	jbe	0x14237b874 <.text+0x237a874>
14237b72e:     	xorl	%r15d, %r15d
14237b731:     	movq	%rdi, %rcx
14237b734:     	movl	$0xa, %eax
14237b739:     	cmpl	$0x1869f, %r14d         # imm = 0x1869F
14237b740:     	jbe	0x14237b7a5 <.text+0x237a7a5>
14237b742:     	movl	%ebx, %eax
14237b744:     	imulq	$0x346dc5d7, %rax, %rdx # imm = 0x346DC5D7
14237b74b:     	shrq	$0x2b, %rdx
14237b74f:     	imull	$0x2710, %edx, %eax     # imm = 0x2710
14237b755:     	subl	%eax, %ebx
14237b757:     	imull	$0x147b, %ebx, %eax     # imm = 0x147B
14237b75d:     	shrl	$0x13, %eax
14237b760:     	imull	$0x64, %eax, %r8d
14237b764:     	subl	%r8d, %ebx
14237b767:     	movzwl	%bx, %r8d
14237b76b:     	leaq	0xcd84be(%rip), %r9     # 0x143053c30
14237b772:     	movzwl	(%r9,%rax,2), %eax
14237b777:     	movw	%ax, 0x36(%rsp)
14237b77c:     	movzwl	(%r9,%r8,2), %eax
14237b781:     	movw	%ax, 0x38(%rsp)
14237b786:     	movl	$0x6, %eax
14237b78b:     	movl	%edx, %ebx
14237b78d:     	movl	%ebx, %edx
14237b78f:     	cmpl	$0x63, %ebx
14237b792:     	ja	0x14237b7ac <.text+0x237a7ac>
14237b794:     	cmpl	$0xa, %edx
14237b797:     	jae	0x14237b7e6 <.text+0x237a7e6>
14237b799:     	orb	$0x30, %dl
14237b79c:     	movb	%dl, 0x2f(%rsp,%rax)
14237b7a0:     	decq	%rax
14237b7a3:     	jmp	0x14237b7fb <.text+0x237a7fb>
14237b7a5:     	movl	%ebx, %edx
14237b7a7:     	cmpl	$0x63, %ebx
14237b7aa:     	jbe	0x14237b794 <.text+0x237a794>
14237b7ac:     	movzwl	%dx, %r8d
14237b7b0:     	shrl	$0x2, %r8d
14237b7b4:     	imull	$0x147b, %r8d, %r8d     # imm = 0x147B
14237b7bb:     	shrl	$0x11, %r8d
14237b7bf:     	imull	$0x64, %r8d, %r9d
14237b7c3:     	subl	%r9d, %edx
14237b7c6:     	movzwl	%dx, %edx
14237b7c9:     	leaq	0xcd8460(%rip), %r9     # 0x143053c30
14237b7d0:     	movzwl	(%r9,%rdx,2), %edx
14237b7d5:     	movw	%dx, 0x2e(%rsp,%rax)
14237b7da:     	addq	$-0x2, %rax
14237b7de:     	movq	%r8, %rdx
14237b7e1:     	cmpl	$0xa, %edx
14237b7e4:     	jb	0x14237b799 <.text+0x237a799>
14237b7e6:     	leaq	0xcd8443(%rip), %r8     # 0x143053c30
14237b7ed:     	movzwl	(%r8,%rdx,2), %edx
14237b7f2:     	movw	%dx, 0x2e(%rsp,%rax)
14237b7f7:     	addq	$-0x2, %rax
14237b7fb:     	movl	$0xa, %edi
14237b800:     	subq	%rax, %rdi
14237b803:     	leaq	(%rsp,%rax), %rdx
14237b807:     	addq	$0x30, %rdx
14237b80b:     	leaq	0xcd8406(%rip), %r9     # 0x143053c18
14237b812:     	jmp	0x14237b9eb <.text+0x237a9eb>
14237b817:     	shrl	$0x5, %r14d
14237b81b:     	imulq	$0xa7c5ac5, %r14, %rax  # imm = 0xA7C5AC5
14237b822:     	shrq	$0x27, %rax
14237b826:     	movq	%rdx, %rcx
14237b829:     	movl	%eax, %edx
14237b82b:     	callq	0x14237bf00 <.text+0x237af00>
14237b830:     	movq	%rdx, %rdi
14237b833:     	jmp	0x14237b07a <.text+0x237a07a>
14237b838:     	movl	$0xd1b71759, %eax       # imm = 0xD1B71759
14237b83d:     	imulq	%r13, %rax
14237b841:     	shrq	$0x2d, %rax
14237b845:     	movq	%rdx, %rcx
14237b848:     	movl	%eax, %edx
14237b84a:     	callq	0x14237c0d0 <.text+0x237b0d0>
14237b84f:     	movq	%rdx, %rdi
14237b852:     	jmp	0x14237b07a <.text+0x237a07a>
14237b857:     	imulq	$0x10624dd3, %r13, %rax # imm = 0x10624DD3
14237b85e:     	shrq	$0x26, %rax
14237b862:     	movq	%rdx, %rcx
14237b865:     	movl	%eax, %edx
14237b867:     	callq	0x14237c5a0 <.text+0x237b5a0>
14237b86c:     	movq	%rdx, %rdi
14237b86f:     	jmp	0x14237b07a <.text+0x237a07a>
14237b874:     	movb	$0x8, %cl
14237b876:     	subb	%al, %cl
14237b878:     	movzbl	%cl, %r15d
14237b87c:     	movq	%rdi, %rcx
14237b87f:     	movq	0x10(%rdi), %rdx
14237b883:     	movq	%r15, %r12
14237b886:     	cmpq	%rdx, (%rcx)
14237b889:     	je	0x14237b8a4 <.text+0x237a8a4>
14237b88b:     	movq	0x8(%rcx), %rax
14237b88f:     	movb	$0x30, (%rax,%rdx)
14237b893:     	incq	%rdx
14237b896:     	movq	%rdx, 0x10(%rcx)
14237b89a:     	decq	%r12
14237b89d:     	jne	0x14237b886 <.text+0x237a886>
14237b89f:     	jmp	0x14237b734 <.text+0x237a734>
14237b8a4:     	movq	$0x1, 0x20(%rsp)
14237b8ad:     	movl	$0x1, %r8d
14237b8b3:     	movl	$0x1, %r9d
14237b8b9:     	callq	0x1428a58d0 <.text+0x28a48d0>
14237b8be:     	movq	%rdi, %rcx
14237b8c1:     	movq	0x10(%rdi), %rdx
14237b8c5:     	jmp	0x14237b88b <.text+0x237a88b>
14237b8c7:     	movb	$0x7, %cl
14237b8c9:     	subb	%al, %cl
14237b8cb:     	movzbl	%cl, %ebp
14237b8ce:     	leaq	0xcd8557(%rip), %r15    # 0x143053e2c
14237b8d5:     	leaq	0xcd833c(%rip), %r12    # 0x143053c18
14237b8dc:     	movq	%rbp, 0x68(%rsp)
14237b8e1:     	movl	$0x1, %r8d
14237b8e7:     	movq	%rbx, %rcx
14237b8ea:     	movq	%r15, %rdx
14237b8ed:     	movq	%r12, %r9
14237b8f0:     	callq	0x140fcd9c0 <.text+0xfcc9c0>
14237b8f5:     	decq	%rbp
14237b8f8:     	jne	0x14237b8e1 <.text+0x237a8e1>
14237b8fa:     	movq	0x68(%rsp), %r15
14237b8ff:     	movl	$0xa, %eax
14237b904:     	cmpl	$0xf423f, %r14d         # imm = 0xF423F
14237b90b:     	jbe	0x14237b97b <.text+0x237a97b>
14237b90d:     	movl	%edi, %eax
14237b90f:     	imulq	$0x1a36e2f, %rax, %rax  # imm = 0x1A36E2F
14237b916:     	shrq	$0x26, %rax
14237b91a:     	imull	$0x2710, %eax, %eax     # imm = 0x2710
14237b920:     	subl	%eax, %edi
14237b922:     	imulq	$0x431bde83, %r13, %rdx # imm = 0x431BDE83
14237b929:     	shrq	$0x32, %rdx
14237b92d:     	imull	$0x147b, %edi, %eax     # imm = 0x147B
14237b933:     	shrl	$0x13, %eax
14237b936:     	imull	$0x64, %eax, %r8d
14237b93a:     	subl	%r8d, %edi
14237b93d:     	movzwl	%di, %r8d
14237b941:     	leaq	0xcd82e8(%rip), %r9     # 0x143053c30
14237b948:     	movzwl	(%r9,%rax,2), %eax
14237b94d:     	movw	%ax, 0x36(%rsp)
14237b952:     	movzwl	(%r9,%r8,2), %eax
14237b957:     	movw	%ax, 0x38(%rsp)
14237b95c:     	movl	$0x6, %eax
14237b961:     	movl	%edx, %edi
14237b963:     	movl	%edi, %edx
14237b965:     	cmpl	$0x63, %edi
14237b968:     	ja	0x14237b982 <.text+0x237a982>
14237b96a:     	cmpl	$0xa, %edx
14237b96d:     	jae	0x14237b9bc <.text+0x237a9bc>
14237b96f:     	orb	$0x30, %dl
14237b972:     	movb	%dl, 0x2f(%rsp,%rax)
14237b976:     	decq	%rax
14237b979:     	jmp	0x14237b9d1 <.text+0x237a9d1>
14237b97b:     	movl	%edi, %edx
14237b97d:     	cmpl	$0x63, %edi
14237b980:     	jbe	0x14237b96a <.text+0x237a96a>
14237b982:     	movzwl	%dx, %r8d
14237b986:     	shrl	$0x2, %r8d
14237b98a:     	imull	$0x147b, %r8d, %r8d     # imm = 0x147B
14237b991:     	shrl	$0x11, %r8d
14237b995:     	imull	$0x64, %r8d, %r9d
14237b999:     	subl	%r9d, %edx
14237b99c:     	movzwl	%dx, %edx
14237b99f:     	leaq	0xcd828a(%rip), %r9     # 0x143053c30
14237b9a6:     	movzwl	(%r9,%rdx,2), %edx
14237b9ab:     	movw	%dx, 0x2e(%rsp,%rax)
14237b9b0:     	addq	$-0x2, %rax
14237b9b4:     	movq	%r8, %rdx
14237b9b7:     	cmpl	$0xa, %edx
14237b9ba:     	jb	0x14237b96f <.text+0x237a96f>
14237b9bc:     	leaq	0xcd826d(%rip), %r8     # 0x143053c30
14237b9c3:     	movzwl	(%r8,%rdx,2), %edx
14237b9c8:     	movw	%dx, 0x2e(%rsp,%rax)
14237b9cd:     	addq	$-0x2, %rax
14237b9d1:     	movl	$0xa, %edi
14237b9d6:     	subq	%rax, %rdi
14237b9d9:     	leaq	(%rsp,%rax), %rdx
14237b9dd:     	addq	$0x30, %rdx
14237b9e1:     	leaq	0xcd8230(%rip), %r9     # 0x143053c18
14237b9e8:     	movq	%rbx, %rcx
14237b9eb:     	movq	%rdi, %r8
14237b9ee:     	callq	0x140fcd9c0 <.text+0xfcc9c0>
14237b9f3:     	addq	%r15, %rdi
14237b9f6:     	jmp	0x14237b07a <.text+0x237a07a>
14237b9fb:     	imulq	$0x51eb851f, %r13, %rdi # imm = 0x51EB851F
14237ba02:     	shrq	$0x25, %rdi
14237ba06:     	imull	$0xcccccccd, %edi, %eax # imm = 0xCCCCCCCD
14237ba0c:     	rorl	%eax
14237ba0e:     	cmpl	$0x19999999, %eax       # imm = 0x19999999
14237ba13:     	ja	0x14237b6c3 <.text+0x237a6c3>
14237ba19:     	imulq	$0x10624dd3, %r13, %rax # imm = 0x10624DD3
14237ba20:     	shrq	$0x26, %rax
14237ba24:     	imull	$0xcccccccd, %eax, %ecx # imm = 0xCCCCCCCD
14237ba2a:     	rorl	%ecx
14237ba2c:     	cmpl	$0x19999999, %ecx       # imm = 0x19999999
14237ba32:     	ja	0x14237b862 <.text+0x237a862>
14237ba38:     	movl	$0xd1b71759, %eax       # imm = 0xD1B71759
14237ba3d:     	imulq	%r13, %rax
14237ba41:     	shrq	$0x2d, %rax
14237ba45:     	imull	$0xcccccccd, %eax, %ecx # imm = 0xCCCCCCCD
14237ba4b:     	rorl	%ecx
14237ba4d:     	cmpl	$0x19999999, %ecx       # imm = 0x19999999
14237ba53:     	ja	0x14237b845 <.text+0x237a845>
14237ba59:     	shrl	$0x5, %r14d
14237ba5d:     	imulq	$0xa7c5ac5, %r14, %rax  # imm = 0xA7C5AC5
14237ba64:     	shrq	$0x27, %rax
14237ba68:     	imull	$0xffffcccd, %eax, %ecx # imm = 0xFFFFCCCD
14237ba6e:     	rorw	%cx
14237ba71:     	movzwl	%cx, %ecx
14237ba74:     	cmpl	$0x1999, %ecx           # imm = 0x1999
14237ba7a:     	ja	0x14237b826 <.text+0x237a826>
14237ba80:     	imulq	$0x431bde83, %r13, %rax # imm = 0x431BDE83
14237ba87:     	shrq	$0x32, %rax
14237ba8b:     	imull	$0xffffcccd, %eax, %ecx # imm = 0xFFFFCCCD
14237ba91:     	rorw	%cx
14237ba94:     	movzwl	%cx, %ecx
14237ba97:     	cmpl	$0x1999, %ecx           # imm = 0x1999
14237ba9d:     	ja	0x14237b6e8 <.text+0x237a6e8>
14237baa3:     	imulq	$0x6b5fca6b, %r13, %rax # imm = 0x6B5FCA6B
14237baaa:     	shrq	$0x36, %rax
14237baae:     	imull	$-0x33, %eax, %ecx
14237bab1:     	rorb	%cl
14237bab3:     	cmpb	$0x19, %cl
14237bab6:     	ja	0x14237b45d <.text+0x237a45d>
14237babc:     	imulq	$0x55e63b89, %r13, %rax # imm = 0x55E63B89
14237bac3:     	shrq	$0x39, %rax
14237bac7:     	movq	%rdx, %rcx
14237baca:     	movl	%eax, %edx
14237bacc:     	callq	0x14237c750 <.text+0x237b750>
14237bad1:     	movq	%rdx, %rdi
14237bad4:     	jmp	0x14237b07a <.text+0x237a07a>
14237bad9:     	movq	$0x1, 0x20(%rsp)
14237bae2:     	movl	$0x2, %r8d
14237bae8:     	movl	$0x1, %r9d
14237baee:     	movq	%rdx, %rcx
14237baf1:     	movq	%rdx, %rbx
14237baf4:     	movq	%rax, %rdx
14237baf7:     	callq	0x1428a58d0 <.text+0x28a48d0>
14237bafc:     	movq	%rbx, %rdx
14237baff:     	movq	0x10(%rbx), %rax
14237bb03:     	jmp	0x14237aed3 <.text+0x2379ed3>
14237bb08:     	movq	$0x1, 0x20(%rsp)
14237bb11:     	movl	$0x1, %r9d
14237bb17:     	movq	%rax, %rcx
14237bb1a:     	movq	%rdx, %r12
14237bb1d:     	movq	%r14, %rdx
14237bb20:     	movq	%rbx, %r8
14237bb23:     	callq	0x1428a58d0 <.text+0x28a48d0>
14237bb28:     	movq	%r12, %rdx
14237bb2b:     	movq	%r15, %rax
14237bb2e:     	movq	0x10(%r15), %r14
14237bb32:     	jmp	0x14237b24e <.text+0x237a24e>
14237bb37:     	movq	$0x1, 0x20(%rsp)
14237bb40:     	movl	$0x1, %r8d
14237bb46:     	movl	$0x1, %r9d
14237bb4c:     	movq	%rdx, %rcx
14237bb4f:     	movq	%rdx, %rdi
14237bb52:     	movq	%rax, %rdx
14237bb55:     	callq	0x1428a58d0 <.text+0x28a48d0>
14237bb5a:     	movq	%rdi, %rdx
14237bb5d:     	movq	0x10(%rdi), %rax
14237bb61:     	jmp	0x14237b124 <.text+0x237a124>
14237bb66:     	leaq	0xcd83cb(%rip), %r8     # 0x143053f38
14237bb6d:     	movl	$0x3, %ecx
14237bb72:     	callq	0x1428d9840 <.text+0x28d8840>
14237bb77:     	leaq	0xcd8282(%rip), %r8     # 0x143053e00
14237bb7e:     	movl	$0x3, %ecx
14237bb83:     	callq	0x1428d9840 <.text+0x28d8840>
14237bb88:     	int3

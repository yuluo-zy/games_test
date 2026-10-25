
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141e11780:     	pushq	%r15
141e11782:     	pushq	%r14
141e11784:     	pushq	%r13
141e11786:     	pushq	%r12
141e11788:     	pushq	%rsi
141e11789:     	pushq	%rdi
141e1178a:     	pushq	%rbp
141e1178b:     	pushq	%rbx
141e1178c:     	subq	$0x248, %rsp            # imm = 0x248
141e11793:     	movq	%rdx, %rdi
141e11796:     	movq	%rcx, %rsi
141e11799:     	callq	0x141df9140 <.text+0x1df8140>
141e1179e:     	movl	$0x1, %r14d
141e117a4:     	lock
141e117a5:     	xaddl	%r14d, 0x5b0(%rdi)
141e117ad:     	xorl	%eax, %eax
141e117af:     	cmpq	0x138(%rsi), %rax
141e117b6:     	jo	0x141e1202c <.text+0x1e1102c>
141e117bc:     	movq	0x1b8(%rsi), %rcx
141e117c3:     	movq	0x218(%rdi), %rax
141e117ca:     	cmpq	%rcx, %rax
141e117cd:     	jbe	0x141e11ded <.text+0x1e10ded>
141e117d3:     	movq	0x210(%rdi), %r9
141e117da:     	movq	(%r9,%rcx,8), %rcx
141e117de:     	testq	%rcx, %rcx
141e117e1:     	je	0x141e11ded <.text+0x1e10ded>
141e117e7:     	movq	0x1e0(%rdi), %r10
141e117ee:     	notq	%rcx
141e117f1:     	leaq	(%rcx,%rcx,2), %rcx
141e117f5:     	shlq	$0x5, %rcx
141e117f9:     	cmpq	$0x0, 0x20(%r10,%rcx)
141e117ff:     	je	0x141e11ded <.text+0x1e10ded>
141e11805:     	movq	0x1c0(%rsi), %rdx
141e1180c:     	cmpq	%rdx, %rax
141e1180f:     	jbe	0x141e11e0e <.text+0x1e10e0e>
141e11815:     	movq	(%r9,%rdx,8), %rdx
141e11819:     	testq	%rdx, %rdx
141e1181c:     	je	0x141e11e0e <.text+0x1e10e0e>
141e11822:     	notq	%rdx
141e11825:     	leaq	(%rdx,%rdx,2), %rdx
141e11829:     	shlq	$0x5, %rdx
141e1182d:     	cmpq	$0x0, 0x20(%r10,%rdx)
141e11833:     	je	0x141e11e0e <.text+0x1e10e0e>
141e11839:     	movq	0x1c8(%rsi), %r8
141e11840:     	cmpq	%r8, %rax
141e11843:     	jbe	0x141e11e2f <.text+0x1e10e2f>
141e11849:     	movq	(%r9,%r8,8), %r8
141e1184d:     	testq	%r8, %r8
141e11850:     	je	0x141e11e2f <.text+0x1e10e2f>
141e11856:     	notq	%r8
141e11859:     	leaq	(%r8,%r8,2), %r8
141e1185d:     	shlq	$0x5, %r8
141e11861:     	cmpq	$0x0, 0x20(%r10,%r8)
141e11867:     	je	0x141e11e2f <.text+0x1e10e2f>
141e1186d:     	movq	0x1d0(%rsi), %r11
141e11874:     	cmpq	%r11, %rax
141e11877:     	jbe	0x141e11e50 <.text+0x1e10e50>
141e1187d:     	movq	(%r9,%r11,8), %rax
141e11881:     	testq	%rax, %rax
141e11884:     	je	0x141e11e50 <.text+0x1e10e50>
141e1188a:     	notq	%rax
141e1188d:     	leaq	(%rax,%rax,2), %r9
141e11891:     	shlq	$0x5, %r9
141e11895:     	cmpq	$0x0, 0x20(%r10,%r9)
141e1189b:     	je	0x141e11e50 <.text+0x1e10e50>
141e118a1:     	addq	%r10, %rcx
141e118a4:     	movq	0x10(%rcx), %rbx
141e118a8:     	movl	0x130(%rsi), %eax
141e118ae:     	addq	%r10, %rdx
141e118b1:     	movq	0x10(%rdx), %r13
141e118b5:     	addq	%r10, %r8
141e118b8:     	movq	0x10(%r8), %r12
141e118bc:     	addq	%r10, %r9
141e118bf:     	movq	0x10(%r9), %r15
141e118c3:     	movl	0x5b0(%rdi), %r10d
141e118ca:     	movq	0x1d8(%rsi), %r10
141e118d1:     	cmpq	%r10, 0x218(%rdi)
141e118d8:     	jbe	0x141e11e71 <.text+0x1e10e71>
141e118de:     	movq	0x210(%rdi), %r11
141e118e5:     	movq	(%r11,%r10,8), %r10
141e118e9:     	testq	%r10, %r10
141e118ec:     	je	0x141e11e71 <.text+0x1e10e71>
141e118f2:     	movq	0x1e0(%rdi), %r11
141e118f9:     	notq	%r10
141e118fc:     	leaq	(%r10,%r10,2), %r10
141e11900:     	shlq	$0x5, %r10
141e11904:     	cmpq	$0x0, 0x20(%r11,%r10)
141e1190a:     	je	0x141e11e71 <.text+0x1e10e71>
141e11910:     	movq	%rbx, 0x240(%rsp)
141e11918:     	addq	%r10, %r11
141e1191b:     	movq	0x10(%r11), %r10
141e1191f:     	movq	%r10, 0x238(%rsp)
141e11927:     	movl	0x5b0(%rdi), %r10d
141e1192e:     	movq	0x1e0(%rsi), %r10
141e11935:     	cmpq	%r10, 0x218(%rdi)
141e1193c:     	jbe	0x141e11e92 <.text+0x1e10e92>
141e11942:     	movq	0x210(%rdi), %rbx
141e11949:     	movq	(%rbx,%r10,8), %r10
141e1194d:     	testq	%r10, %r10
141e11950:     	je	0x141e11e92 <.text+0x1e10e92>
141e11956:     	movq	0x1e0(%rdi), %rbp
141e1195d:     	notq	%r10
141e11960:     	leaq	(%r10,%r10,2), %r10
141e11964:     	shlq	$0x5, %r10
141e11968:     	cmpq	$0x0, 0x20(%rbp,%r10)
141e1196e:     	je	0x141e11e92 <.text+0x1e10e92>
141e11974:     	movq	%r13, 0x220(%rsp)
141e1197c:     	movq	%r12, 0x228(%rsp)
141e11984:     	movq	%r15, 0x230(%rsp)
141e1198c:     	movl	%r14d, 0x94(%rsp)
141e11994:     	addq	%r10, %rbp
141e11997:     	movq	0x10(%rbp), %r10
141e1199b:     	movq	%r10, 0x218(%rsp)
141e119a3:     	movl	0x5b0(%rdi), %r10d
141e119aa:     	movq	0x1e8(%rsi), %r10
141e119b1:     	cmpq	%r10, 0x218(%rdi)
141e119b8:     	jbe	0x141e11eb3 <.text+0x1e10eb3>
141e119be:     	movq	0x210(%rdi), %rbx
141e119c5:     	movq	(%rbx,%r10,8), %r10
141e119c9:     	testq	%r10, %r10
141e119cc:     	je	0x141e11eb3 <.text+0x1e10eb3>
141e119d2:     	movq	0x1e0(%rdi), %r14
141e119d9:     	notq	%r10
141e119dc:     	leaq	(%r10,%r10,2), %r10
141e119e0:     	shlq	$0x5, %r10
141e119e4:     	cmpq	$0x0, 0x20(%r14,%r10)
141e119ea:     	je	0x141e11eb3 <.text+0x1e10eb3>
141e119f0:     	addq	%r10, %r14
141e119f3:     	movq	0x10(%r14), %r10
141e119f7:     	movq	%r10, 0x210(%rsp)
141e119ff:     	movl	0x5b0(%rdi), %r10d
141e11a06:     	movq	0x1f0(%rsi), %r10
141e11a0d:     	movq	0x218(%rdi), %r12
141e11a14:     	cmpq	%r10, %r12
141e11a17:     	jbe	0x141e11ed4 <.text+0x1e10ed4>
141e11a1d:     	movq	0x210(%rdi), %r13
141e11a24:     	movq	(%r13,%r10,8), %r15
141e11a29:     	testq	%r15, %r15
141e11a2c:     	je	0x141e11ed4 <.text+0x1e10ed4>
141e11a32:     	movq	0x1e0(%rdi), %r10
141e11a39:     	notq	%r15
141e11a3c:     	leaq	(%r15,%r15,2), %r15
141e11a40:     	shlq	$0x5, %r15
141e11a44:     	cmpq	$0x0, 0x20(%r10,%r15)
141e11a4a:     	je	0x141e11ed4 <.text+0x1e10ed4>
141e11a50:     	movq	0x1f8(%rsi), %rbx
141e11a57:     	cmpq	%rbx, %r12
141e11a5a:     	jbe	0x141e11f71 <.text+0x1e10f71>
141e11a60:     	movq	(%r13,%rbx,8), %r12
141e11a65:     	testq	%r12, %r12
141e11a68:     	je	0x141e11f71 <.text+0x1e10f71>
141e11a6e:     	notq	%r12
141e11a71:     	leaq	(%r12,%r12,2), %r13
141e11a75:     	shlq	$0x5, %r13
141e11a79:     	cmpq	$0x0, 0x20(%r10,%r13)
141e11a7f:     	je	0x141e11f71 <.text+0x1e10f71>
141e11a85:     	addq	%r10, %r15
141e11a88:     	movq	0x10(%r15), %rbx
141e11a8c:     	movq	%rbx, 0x200(%rsp)
141e11a94:     	addq	%r10, %r13
141e11a97:     	movq	0x10(%r13), %r10
141e11a9b:     	movq	%r10, 0x208(%rsp)
141e11aa3:     	movl	0x5b0(%rdi), %r10d
141e11aaa:     	movq	0x200(%rsi), %r10
141e11ab1:     	cmpq	%r10, 0x218(%rdi)
141e11ab8:     	jbe	0x141e11f8f <.text+0x1e10f8f>
141e11abe:     	movq	0x210(%rdi), %rbx
141e11ac5:     	movq	(%rbx,%r10,8), %r10
141e11ac9:     	testq	%r10, %r10
141e11acc:     	je	0x141e11f8f <.text+0x1e10f8f>
141e11ad2:     	movq	0x1e0(%rdi), %r12
141e11ad9:     	notq	%r10
141e11adc:     	leaq	(%r10,%r10,2), %r10
141e11ae0:     	shlq	$0x5, %r10
141e11ae4:     	cmpq	$0x0, 0x20(%r12,%r10)
141e11aea:     	je	0x141e11f8f <.text+0x1e10f8f>
141e11af0:     	addq	%r10, %r12
141e11af3:     	movq	0x10(%r12), %r10
141e11af8:     	movl	0x5b0(%rdi), %ebx
141e11afe:     	movq	0x240(%rsp), %rbx
141e11b06:     	movq	%rbx, 0x98(%rsp)
141e11b0e:     	leaq	0x5c(%rcx), %rbx
141e11b12:     	addq	$0x58, %rcx
141e11b16:     	movq	%rcx, 0xa0(%rsp)
141e11b1e:     	movq	%rbx, 0xa8(%rsp)
141e11b26:     	movl	%eax, 0xb0(%rsp)
141e11b2d:     	movl	0x94(%rsp), %ebx
141e11b34:     	movl	%ebx, 0xb4(%rsp)
141e11b3b:     	movq	0x220(%rsp), %rcx
141e11b43:     	movq	%rcx, 0xb8(%rsp)
141e11b4b:     	leaq	0x5c(%rdx), %rcx
141e11b4f:     	addq	$0x58, %rdx
141e11b53:     	movq	%rdx, 0xc0(%rsp)
141e11b5b:     	movq	%rcx, 0xc8(%rsp)
141e11b63:     	movl	%eax, 0xd0(%rsp)
141e11b6a:     	movl	%ebx, 0xd4(%rsp)
141e11b71:     	movq	0x228(%rsp), %rcx
141e11b79:     	movq	%rcx, 0xd8(%rsp)
141e11b81:     	leaq	0x5c(%r8), %rcx
141e11b85:     	addq	$0x58, %r8
141e11b89:     	movq	%r8, 0xe0(%rsp)
141e11b91:     	movq	%rcx, 0xe8(%rsp)
141e11b99:     	movl	%eax, 0xf0(%rsp)
141e11ba0:     	movl	%ebx, 0xf4(%rsp)
141e11ba7:     	movq	0x230(%rsp), %rcx
141e11baf:     	movq	%rcx, 0xf8(%rsp)
141e11bb7:     	leaq	0x5c(%r9), %rcx
141e11bbb:     	addq	$0x58, %r9
141e11bbf:     	movq	%r9, 0x100(%rsp)
141e11bc7:     	movq	%rcx, 0x108(%rsp)
141e11bcf:     	movl	%eax, 0x110(%rsp)
141e11bd6:     	movl	%ebx, 0x114(%rsp)
141e11bdd:     	movq	0x238(%rsp), %rcx
141e11be5:     	movq	%rcx, 0x118(%rsp)
141e11bed:     	leaq	0x5c(%r11), %rcx
141e11bf1:     	addq	$0x58, %r11
141e11bf5:     	movq	%r11, 0x120(%rsp)
141e11bfd:     	movq	%rcx, 0x128(%rsp)
141e11c05:     	movl	%eax, 0x130(%rsp)
141e11c0c:     	movl	%ebx, 0x134(%rsp)
141e11c13:     	movq	0x218(%rsp), %rcx
141e11c1b:     	movq	%rcx, 0x138(%rsp)
141e11c23:     	leaq	0x5c(%rbp), %rcx
141e11c27:     	addq	$0x58, %rbp
141e11c2b:     	movq	%rbp, 0x140(%rsp)
141e11c33:     	movq	%rcx, 0x148(%rsp)
141e11c3b:     	leaq	0x138(%rsi), %rdx
141e11c42:     	movl	%eax, 0x150(%rsp)
141e11c49:     	movl	%ebx, 0x154(%rsp)
141e11c50:     	movq	0x210(%rsp), %rcx
141e11c58:     	movq	%rcx, 0x158(%rsp)
141e11c60:     	leaq	0x5c(%r14), %rcx
141e11c64:     	addq	$0x58, %r14
141e11c68:     	movq	%r14, 0x160(%rsp)
141e11c70:     	leaq	0x5c(%r13), %r11
141e11c74:     	addq	$0x58, %r13
141e11c78:     	movq	%rcx, 0x168(%rsp)
141e11c80:     	leaq	0x5c(%r15), %r9
141e11c84:     	addq	$0x58, %r15
141e11c88:     	movl	%eax, 0x170(%rsp)
141e11c8f:     	movl	%ebx, 0x174(%rsp)
141e11c96:     	movq	%rdi, 0x178(%rsp)
141e11c9e:     	leaq	0x170(%rsi), %rcx
141e11ca5:     	movq	$0x0, 0x180(%rsp)
141e11cb1:     	movq	%rdx, 0x188(%rsp)
141e11cb9:     	leaq	0x58(%r12), %rdi
141e11cbe:     	addq	$0x5c, %r12
141e11cc2:     	movq	0x200(%rsp), %rdx
141e11cca:     	movq	%rdx, 0x198(%rsp)
141e11cd2:     	leaq	0xb8(%rsp), %rdx
141e11cda:     	movq	%r15, 0x1a0(%rsp)
141e11ce2:     	leaq	0xd8(%rsp), %r8
141e11cea:     	movq	%r9, 0x1a8(%rsp)
141e11cf2:     	leaq	0xf8(%rsp), %r9
141e11cfa:     	movl	%eax, 0x1b0(%rsp)
141e11d01:     	movl	%ebx, 0x1b4(%rsp)
141e11d08:     	movq	%rcx, 0x1b8(%rsp)
141e11d10:     	movq	0x208(%rsp), %r14
141e11d18:     	movq	%r14, 0x1c0(%rsp)
141e11d20:     	leaq	0x118(%rsp), %rbp
141e11d28:     	movq	%r13, 0x1c8(%rsp)
141e11d30:     	leaq	0x138(%rsp), %r14
141e11d38:     	movq	%r11, 0x1d0(%rsp)
141e11d40:     	leaq	0x158(%rsp), %r11
141e11d48:     	movl	%eax, 0x1d8(%rsp)
141e11d4f:     	movl	%ebx, 0x1dc(%rsp)
141e11d56:     	movq	%r10, 0x1e0(%rsp)
141e11d5e:     	leaq	0x178(%rsp), %r10
141e11d66:     	movq	%rdi, 0x1e8(%rsp)
141e11d6e:     	leaq	0x198(%rsp), %rdi
141e11d76:     	movq	%r12, 0x1f0(%rsp)
141e11d7e:     	leaq	0x1c0(%rsp), %r15
141e11d86:     	movl	%eax, 0x1f8(%rsp)
141e11d8d:     	leaq	0x1e0(%rsp), %rax
141e11d95:     	movl	%ebx, 0x1fc(%rsp)
141e11d9c:     	movq	%rax, 0x58(%rsp)
141e11da1:     	movq	%r15, 0x50(%rsp)
141e11da6:     	movq	%rcx, 0x48(%rsp)
141e11dab:     	movq	%rdi, 0x40(%rsp)
141e11db0:     	movq	%r10, 0x38(%rsp)
141e11db5:     	movq	%r11, 0x30(%rsp)
141e11dba:     	movq	%r14, 0x28(%rsp)
141e11dbf:     	movq	%rbp, 0x20(%rsp)
141e11dc4:     	leaq	0x98(%rsp), %rcx
141e11dcc:     	callq	0x141e1fe80 <.text+0x1e1ee80>
141e11dd1:     	movl	%ebx, 0x130(%rsi)
141e11dd7:     	xorl	%eax, %eax
141e11dd9:     	addq	$0x248, %rsp            # imm = 0x248
141e11de0:     	popq	%rbx
141e11de1:     	popq	%rbp
141e11de2:     	popq	%rdi
141e11de3:     	popq	%rsi
141e11de4:     	popq	%r12
141e11de6:     	popq	%r13
141e11de8:     	popq	%r14
141e11dea:     	popq	%r15
141e11dec:     	retq
141e11ded:     	addq	$0x98, %rsi
141e11df4:     	leaq	0x11747a9(%rip), %rax   # 0x142f865a4
141e11dfb:     	movq	%rax, 0x60(%rsp)
141e11e00:     	movq	$0x2f, 0x68(%rsp)
141e11e09:     	jmp	0x141e11ef0 <.text+0x1e10ef0>
141e11e0e:     	addq	$0x98, %rsi
141e11e15:     	leaq	0x1174a24(%rip), %rax   # 0x142f86840
141e11e1c:     	movq	%rax, 0x60(%rsp)
141e11e21:     	movq	$0x29, 0x68(%rsp)
141e11e2a:     	jmp	0x141e11ef0 <.text+0x1e10ef0>
141e11e2f:     	addq	$0x98, %rsi
141e11e36:     	leaq	0x1174098(%rip), %rax   # 0x142f85ed5
141e11e3d:     	movq	%rax, 0x60(%rsp)
141e11e42:     	movq	$0x35, 0x68(%rsp)
141e11e4b:     	jmp	0x141e11ef0 <.text+0x1e10ef0>
141e11e50:     	addq	$0x98, %rsi
141e11e57:     	leaq	0x11747d3(%rip), %rax   # 0x142f86631
141e11e5e:     	movq	%rax, 0x60(%rsp)
141e11e63:     	movq	$0x2a, 0x68(%rsp)
141e11e6c:     	jmp	0x141e11fab <.text+0x1e10fab>
141e11e71:     	addq	$0x98, %rsi
141e11e78:     	leaq	0x1174e26(%rip), %rax   # 0x142f86ca5
141e11e7f:     	movq	%rax, 0x60(%rsp)
141e11e84:     	movq	$0x2e, 0x68(%rsp)
141e11e8d:     	jmp	0x141e11fab <.text+0x1e10fab>
141e11e92:     	addq	$0x98, %rsi
141e11e99:     	leaq	0x1174c76(%rip), %rax   # 0x142f86b16
141e11ea0:     	movq	%rax, 0x60(%rsp)
141e11ea5:     	movq	$0x1e, 0x68(%rsp)
141e11eae:     	jmp	0x141e11fab <.text+0x1e10fab>
141e11eb3:     	addq	$0x98, %rsi
141e11eba:     	leaq	0x1174fdb(%rip), %rax   # 0x142f86e9c
141e11ec1:     	movq	%rax, 0x60(%rsp)
141e11ec6:     	movq	$0x59, 0x68(%rsp)
141e11ecf:     	jmp	0x141e11fab <.text+0x1e10fab>
141e11ed4:     	addq	$0x98, %rsi
141e11edb:     	leaq	0x1174b04(%rip), %rax   # 0x142f869e6
141e11ee2:     	movq	%rax, 0x60(%rsp)
141e11ee7:     	movq	$0x32, 0x68(%rsp)
141e11ef0:     	movq	%rsi, 0x70(%rsp)
141e11ef5:     	leaq	-0x1dd642c(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
141e11efc:     	movq	%rax, 0x78(%rsp)
141e11f01:     	leaq	0x60(%rsp), %rax
141e11f06:     	movq	%rax, 0x80(%rsp)
141e11f0e:     	leaq	-0x1d08085(%rip), %rax  # 0x140109e90 <.text+0x108e90>
141e11f15:     	movq	%rax, 0x88(%rsp)
141e11f1d:     	leaq	0x1173dec(%rip), %rax   # 0x142f85d10
141e11f24:     	movq	%rax, 0x98(%rsp)
141e11f2c:     	movq	$0x2, 0xa0(%rsp)
141e11f38:     	movq	$0x0, 0xb8(%rsp)
141e11f44:     	leaq	0x70(%rsp), %rax
141e11f49:     	movq	%rax, 0xa8(%rsp)
141e11f51:     	movq	$0x2, 0xb0(%rsp)
141e11f5d:     	leaq	0x1173e34(%rip), %rdx   # 0x142f85d98
141e11f64:     	leaq	0x98(%rsp), %rcx
141e11f6c:     	callq	0x1428d9390 <.text+0x28d8390>
141e11f71:     	addq	$0x98, %rsi
141e11f78:     	leaq	0x1174bb5(%rip), %rax   # 0x142f86b34
141e11f7f:     	movq	%rax, 0x60(%rsp)
141e11f84:     	movq	$0x5f, 0x68(%rsp)
141e11f8d:     	jmp	0x141e11fab <.text+0x1e10fab>
141e11f8f:     	addq	$0x98, %rsi
141e11f96:     	leaq	0x1174caa(%rip), %rax   # 0x142f86c47
141e11f9d:     	movq	%rax, 0x60(%rsp)
141e11fa2:     	movq	$0x5e, 0x68(%rsp)
141e11fab:     	movq	%rsi, 0x70(%rsp)
141e11fb0:     	leaq	-0x1dd64e7(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
141e11fb7:     	movq	%rax, 0x78(%rsp)
141e11fbc:     	leaq	0x60(%rsp), %rax
141e11fc1:     	movq	%rax, 0x80(%rsp)
141e11fc9:     	leaq	-0x1d08140(%rip), %rax  # 0x140109e90 <.text+0x108e90>
141e11fd0:     	movq	%rax, 0x88(%rsp)
141e11fd8:     	leaq	0x1173d31(%rip), %rax   # 0x142f85d10
141e11fdf:     	movq	%rax, 0x98(%rsp)
141e11fe7:     	movq	$0x2, 0xa0(%rsp)
141e11ff3:     	movq	$0x0, 0xb8(%rsp)
141e11fff:     	leaq	0x70(%rsp), %rax
141e12004:     	movq	%rax, 0xa8(%rsp)
141e1200c:     	movq	$0x2, 0xb0(%rsp)
141e12018:     	leaq	0x1174a79(%rip), %rdx   # 0x142f86a98
141e1201f:     	leaq	0x98(%rsp), %rcx
141e12027:     	callq	0x1428d9390 <.text+0x28d8390>
141e1202c:     	leaq	0x1175335(%rip), %rcx   # 0x142f87368
141e12033:     	leaq	0x11753f6(%rip), %r8    # 0x142f87430
141e1203a:     	movl	$0x59, %edx
141e1203f:     	callq	0x1428d9330 <.text+0x28d8330>
141e12044:     	int3

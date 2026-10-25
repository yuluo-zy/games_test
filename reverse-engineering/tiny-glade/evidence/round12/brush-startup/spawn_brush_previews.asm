
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140924b40:     	pushq	%rbp
140924b41:     	pushq	%r15
140924b43:     	pushq	%r14
140924b45:     	pushq	%r13
140924b47:     	pushq	%r12
140924b49:     	pushq	%rsi
140924b4a:     	pushq	%rdi
140924b4b:     	pushq	%rbx
140924b4c:     	subq	$0x298, %rsp            # imm = 0x298
140924b53:     	leaq	0x80(%rsp), %rbp
140924b5b:     	movaps	%xmm7, 0x200(%rbp)
140924b62:     	movaps	%xmm6, 0x1f0(%rbp)
140924b69:     	movq	$-0x2, 0x1e8(%rbp)
140924b74:     	movq	%r8, %rdi
140924b77:     	movq	%rdx, %rsi
140924b7a:     	movq	0x280(%rbp), %rax
140924b81:     	movq	(%rcx), %rcx
140924b84:     	movq	(%rcx), %rcx
140924b87:     	addq	$0x10, %rcx
140924b8b:     	movq	(%rax), %rax
140924b8e:     	movq	(%r9), %r14
140924b91:     	movq	%r14, 0x30(%rsp)
140924b96:     	movq	%rax, 0x28(%rsp)
140924b9b:     	movq	$0x1a, 0x20(%rsp)
140924ba4:     	leaq	0x2189e3d(%rip), %rdx   # 0x142aae9e8
140924bab:     	leaq	0x2189e56(%rip), %r9    # 0x142aaea08
140924bb2:     	movl	$0x20, %r8d
140924bb8:     	callq	0x140b3fe40 <.text+0xb3ee40>
140924bbd:     	movq	%rax, 0x90(%rbp)
140924bc4:     	leaq	0x2189e57(%rip), %rdx   # 0x142aaea22
140924bcb:     	leaq	0xc0(%rbp), %rcx
140924bd2:     	movl	$0x1e, %r8d
140924bd8:     	callq	0x140b0eeb0 <.text+0xb0deb0>
140924bdd:     	xorl	%eax, %eax
140924bdf:     	cmpq	0xc0(%rbp), %rax
140924be6:     	jo	0x1409264b3 <.text+0x9254b3>
140924bec:     	movq	0x130(%rbp), %rax
140924bf3:     	movq	%rax, 0x1e0(%rbp)
140924bfa:     	movups	0x120(%rbp), %xmm0
140924c01:     	movaps	%xmm0, 0x1d0(%rbp)
140924c08:     	movups	0x110(%rbp), %xmm0
140924c0f:     	movaps	%xmm0, 0x1c0(%rbp)
140924c16:     	movups	0x100(%rbp), %xmm0
140924c1d:     	movaps	%xmm0, 0x1b0(%rbp)
140924c24:     	movups	0xc0(%rbp), %xmm0
140924c2b:     	movups	0xd0(%rbp), %xmm1
140924c32:     	movups	0xe0(%rbp), %xmm2
140924c39:     	movups	0xf0(%rbp), %xmm3
140924c40:     	movaps	%xmm3, 0x1a0(%rbp)
140924c47:     	movaps	%xmm2, 0x190(%rbp)
140924c4e:     	movaps	%xmm1, 0x180(%rbp)
140924c55:     	movaps	%xmm0, 0x170(%rbp)
140924c5c:     	movsd	0x2009a4c(%rip), %xmm6  # 0x14292e6b0
140924c64:     	movsd	%xmm6, (%rbp)
140924c69:     	movl	$0x3f800000, 0x8(%rbp)  # imm = 0x3F800000
140924c70:     	leaq	0xc0(%rbp), %rbx
140924c77:     	leaq	0x170(%rbp), %rdx
140924c7e:     	movq	%rbp, %r8
140924c81:     	movq	%rbx, %rcx
140924c84:     	callq	0x140a07680 <.text+0xa06680>
140924c89:     	movq	(%rdi), %rdi
140924c8c:     	movq	(%rsi), %r15
140924c8f:     	movq	%r15, 0x20(%rsp)
140924c94:     	leaq	0x2189df5(%rip), %rdx   # 0x142aaea90
140924c9b:     	movl	$0xb, %r8d
140924ca1:     	movq	%rbx, %rcx
140924ca4:     	movq	%rdi, %r9
140924ca7:     	callq	0x140b1aca0 <.text+0xb19ca0>
140924cac:     	movq	%rax, %r13
140924caf:     	leaq	0x2189de5(%rip), %rdx   # 0x142aaea9b
140924cb6:     	leaq	0xc0(%rbp), %rcx
140924cbd:     	movl	$0x1f, %r8d
140924cc3:     	callq	0x140b0eeb0 <.text+0xb0deb0>
140924cc8:     	xorl	%eax, %eax
140924cca:     	cmpq	0xc0(%rbp), %rax
140924cd1:     	jo	0x1409264f1 <.text+0x9254f1>
140924cd7:     	movq	0x130(%rbp), %rax
140924cde:     	movq	%rax, 0x1e0(%rbp)
140924ce5:     	movups	0x120(%rbp), %xmm0
140924cec:     	movaps	%xmm0, 0x1d0(%rbp)
140924cf3:     	movups	0x110(%rbp), %xmm0
140924cfa:     	movaps	%xmm0, 0x1c0(%rbp)
140924d01:     	movups	0x100(%rbp), %xmm0
140924d08:     	movaps	%xmm0, 0x1b0(%rbp)
140924d0f:     	movups	0xc0(%rbp), %xmm0
140924d16:     	movups	0xd0(%rbp), %xmm1
140924d1d:     	movups	0xe0(%rbp), %xmm2
140924d24:     	movups	0xf0(%rbp), %xmm3
140924d2b:     	movaps	%xmm3, 0x1a0(%rbp)
140924d32:     	movaps	%xmm2, 0x190(%rbp)
140924d39:     	movaps	%xmm1, 0x180(%rbp)
140924d40:     	movaps	%xmm0, 0x170(%rbp)
140924d47:     	movsd	%xmm6, (%rbp)
140924d4c:     	movl	$0x3f800000, 0x8(%rbp)  # imm = 0x3F800000
140924d53:     	leaq	0xc0(%rbp), %rsi
140924d5a:     	leaq	0x170(%rbp), %rdx
140924d61:     	movq	%rbp, %r8
140924d64:     	movq	%rsi, %rcx
140924d67:     	callq	0x140a07680 <.text+0xa06680>
140924d6c:     	movq	%r15, 0x20(%rsp)
140924d71:     	leaq	0x2189d60(%rip), %rdx   # 0x142aaead8
140924d78:     	movl	$0xc, %r8d
140924d7e:     	movq	%rsi, %rcx
140924d81:     	movq	%rdi, %r9
140924d84:     	callq	0x140b1aca0 <.text+0xb19ca0>
140924d89:     	movq	%rax, -0x30(%rbp)
140924d8d:     	leaq	0x2189d50(%rip), %rdx   # 0x142aaeae4
140924d94:     	leaq	0xc0(%rbp), %rcx
140924d9b:     	movl	$0x25, %r8d
140924da1:     	callq	0x140b0eeb0 <.text+0xb0deb0>
140924da6:     	xorl	%eax, %eax
140924da8:     	cmpq	0xc0(%rbp), %rax
140924daf:     	jo	0x14092652f <.text+0x92552f>
140924db5:     	movq	0x130(%rbp), %rax
140924dbc:     	movq	%rax, 0x1e0(%rbp)
140924dc3:     	movups	0x120(%rbp), %xmm0
140924dca:     	movaps	%xmm0, 0x1d0(%rbp)
140924dd1:     	movups	0x110(%rbp), %xmm0
140924dd8:     	movaps	%xmm0, 0x1c0(%rbp)
140924ddf:     	movups	0x100(%rbp), %xmm0
140924de6:     	movaps	%xmm0, 0x1b0(%rbp)
140924ded:     	movups	0xc0(%rbp), %xmm0
140924df4:     	movups	0xd0(%rbp), %xmm1
140924dfb:     	movups	0xe0(%rbp), %xmm2
140924e02:     	movups	0xf0(%rbp), %xmm3
140924e09:     	movaps	%xmm3, 0x1a0(%rbp)
140924e10:     	movaps	%xmm2, 0x190(%rbp)
140924e17:     	movaps	%xmm1, 0x180(%rbp)
140924e1e:     	movaps	%xmm0, 0x170(%rbp)
140924e25:     	movq	%r15, 0x20(%rsp)
140924e2a:     	leaq	0x2189cf7(%rip), %rdx   # 0x142aaeb28
140924e31:     	leaq	0x170(%rbp), %rcx
140924e38:     	movl	$0x12, %r8d
140924e3e:     	movq	%rdi, %r9
140924e41:     	callq	0x140b1aca0 <.text+0xb19ca0>
140924e46:     	movq	%rax, %r12
140924e49:     	leaq	0x2189cea(%rip), %rdx   # 0x142aaeb3a
140924e50:     	leaq	0xc0(%rbp), %rcx
140924e57:     	movl	$0x28, %r8d
140924e5d:     	callq	0x140b0eeb0 <.text+0xb0deb0>
140924e62:     	xorl	%eax, %eax
140924e64:     	cmpq	0xc0(%rbp), %rax
140924e6b:     	jo	0x14092656d <.text+0x92556d>
140924e71:     	movq	0x130(%rbp), %rax
140924e78:     	movq	%rax, 0x1e0(%rbp)
140924e7f:     	movups	0x120(%rbp), %xmm0
140924e86:     	movaps	%xmm0, 0x1d0(%rbp)
140924e8d:     	movups	0x110(%rbp), %xmm0
140924e94:     	movaps	%xmm0, 0x1c0(%rbp)
140924e9b:     	movups	0x100(%rbp), %xmm0
140924ea2:     	movaps	%xmm0, 0x1b0(%rbp)
140924ea9:     	movups	0xc0(%rbp), %xmm0
140924eb0:     	movups	0xd0(%rbp), %xmm1
140924eb7:     	movups	0xe0(%rbp), %xmm2
140924ebe:     	movups	0xf0(%rbp), %xmm3
140924ec5:     	movaps	%xmm3, 0x1a0(%rbp)
140924ecc:     	movaps	%xmm2, 0x190(%rbp)
140924ed3:     	movaps	%xmm1, 0x180(%rbp)
140924eda:     	movaps	%xmm0, 0x170(%rbp)
140924ee1:     	movss	0x2000987(%rip), %xmm0  # 0x142925870
140924ee9:     	movlps	%xmm0, (%rbp)
140924eed:     	movl	$0x0, 0x8(%rbp)
140924ef4:     	leaq	0x170(%rbp), %rdx
140924efb:     	movq	%rbp, %r8
140924efe:     	movq	%rsi, %rcx
140924f01:     	callq	0x140a07680 <.text+0xa06680>
140924f06:     	movq	%r15, 0x20(%rsp)
140924f0b:     	leaq	0x2189c6e(%rip), %rdx   # 0x142aaeb80
140924f12:     	movl	$0x15, %r8d
140924f18:     	movq	%rsi, %rcx
140924f1b:     	movq	%rdi, %r9
140924f1e:     	callq	0x140b1aca0 <.text+0xb19ca0>
140924f23:     	movq	%rax, 0xb8(%rbp)
140924f2a:     	leaq	0x2189c64(%rip), %rdx   # 0x142aaeb95
140924f31:     	leaq	0xc0(%rbp), %rcx
140924f38:     	movl	$0x23, %r8d
140924f3e:     	callq	0x140b0eeb0 <.text+0xb0deb0>
140924f43:     	xorl	%eax, %eax
140924f45:     	cmpq	0xc0(%rbp), %rax
140924f4c:     	jo	0x1409265ab <.text+0x9255ab>
140924f52:     	movq	0x130(%rbp), %rax
140924f59:     	movq	%rax, 0x1e0(%rbp)
140924f60:     	movups	0x120(%rbp), %xmm0
140924f67:     	movaps	%xmm0, 0x1d0(%rbp)
140924f6e:     	movups	0x110(%rbp), %xmm0
140924f75:     	movaps	%xmm0, 0x1c0(%rbp)
140924f7c:     	movups	0x100(%rbp), %xmm0
140924f83:     	movaps	%xmm0, 0x1b0(%rbp)
140924f8a:     	movups	0xc0(%rbp), %xmm0
140924f91:     	movups	0xd0(%rbp), %xmm1
140924f98:     	movups	0xe0(%rbp), %xmm2
140924f9f:     	movups	0xf0(%rbp), %xmm3
140924fa6:     	movaps	%xmm3, 0x1a0(%rbp)
140924fad:     	movaps	%xmm2, 0x190(%rbp)
140924fb4:     	movaps	%xmm1, 0x180(%rbp)
140924fbb:     	movaps	%xmm0, 0x170(%rbp)
140924fc2:     	movsd	%xmm6, (%rbp)
140924fc7:     	movl	$0x3f800000, 0x8(%rbp)  # imm = 0x3F800000
140924fce:     	leaq	0x170(%rbp), %rdx
140924fd5:     	movq	%rbp, %r8
140924fd8:     	movq	%rsi, %rcx
140924fdb:     	callq	0x140a07680 <.text+0xa06680>
140924fe0:     	movq	%r15, 0x20(%rsp)
140924fe5:     	leaq	0x2189be4(%rip), %rdx   # 0x142aaebd0
140924fec:     	movl	$0x10, %r8d
140924ff2:     	movq	%rsi, %rcx
140924ff5:     	movq	%rdi, %r9
140924ff8:     	callq	0x140b1aca0 <.text+0xb19ca0>
140924ffd:     	movq	%rax, %rbx
140925000:     	leaq	0x2189bd9(%rip), %rdx   # 0x142aaebe0
140925007:     	leaq	0xc0(%rbp), %rcx
14092500e:     	movl	$0x21, %r8d
140925014:     	callq	0x140b0eeb0 <.text+0xb0deb0>
140925019:     	xorl	%eax, %eax
14092501b:     	cmpq	0xc0(%rbp), %rax
140925022:     	jo	0x1409265e9 <.text+0x9255e9>
140925028:     	movq	0x130(%rbp), %rax
14092502f:     	movq	%rax, 0x1e0(%rbp)
140925036:     	movups	0x120(%rbp), %xmm0
14092503d:     	movaps	%xmm0, 0x1d0(%rbp)
140925044:     	movups	0x110(%rbp), %xmm0
14092504b:     	movaps	%xmm0, 0x1c0(%rbp)
140925052:     	movups	0x100(%rbp), %xmm0
140925059:     	movaps	%xmm0, 0x1b0(%rbp)
140925060:     	movups	0xc0(%rbp), %xmm0
140925067:     	movups	0xd0(%rbp), %xmm1
14092506e:     	movups	0xe0(%rbp), %xmm2
140925075:     	movups	0xf0(%rbp), %xmm3
14092507c:     	movaps	%xmm3, 0x1a0(%rbp)
140925083:     	movaps	%xmm2, 0x190(%rbp)
14092508a:     	movaps	%xmm1, 0x180(%rbp)
140925091:     	movaps	%xmm0, 0x170(%rbp)
140925098:     	movsd	%xmm6, (%rbp)
14092509d:     	movl	$0x3f800000, 0x8(%rbp)  # imm = 0x3F800000
1409250a4:     	leaq	0x170(%rbp), %rdx
1409250ab:     	movq	%rbp, %r8
1409250ae:     	movq	%rsi, %rcx
1409250b1:     	callq	0x140a07680 <.text+0xa06680>
1409250b6:     	movq	%r15, 0x20(%rsp)
1409250bb:     	leaq	0x2189b5e(%rip), %rdx   # 0x142aaec20
1409250c2:     	movl	$0x12, %r8d
1409250c8:     	movq	%rsi, %rcx
1409250cb:     	movq	%rdi, %r9
1409250ce:     	callq	0x140b1aca0 <.text+0xb19ca0>
1409250d3:     	movq	%rax, %rsi
1409250d6:     	leaq	0x2189b55(%rip), %rdx   # 0x142aaec32
1409250dd:     	leaq	0xc0(%rbp), %rcx
1409250e4:     	movl	$0x21, %r8d
1409250ea:     	callq	0x140b0eeb0 <.text+0xb0deb0>
1409250ef:     	xorl	%eax, %eax
1409250f1:     	cmpq	0xc0(%rbp), %rax
1409250f8:     	jo	0x140926627 <.text+0x925627>
1409250fe:     	movq	%r13, -0x8(%rbp)
140925102:     	movq	%r12, -0x10(%rbp)
140925106:     	movq	%rbx, -0x18(%rbp)
14092510a:     	movq	%rsi, -0x28(%rbp)
14092510e:     	movq	%r14, 0xa0(%rbp)
140925115:     	movq	0x130(%rbp), %rax
14092511c:     	movq	%rax, 0x1e0(%rbp)
140925123:     	movups	0x120(%rbp), %xmm0
14092512a:     	movaps	%xmm0, 0x1d0(%rbp)
140925131:     	movups	0x110(%rbp), %xmm0
140925138:     	movaps	%xmm0, 0x1c0(%rbp)
14092513f:     	movups	0x100(%rbp), %xmm0
140925146:     	movaps	%xmm0, 0x1b0(%rbp)
14092514d:     	movups	0xc0(%rbp), %xmm0
140925154:     	movups	0xd0(%rbp), %xmm1
14092515b:     	movups	0xe0(%rbp), %xmm2
140925162:     	movups	0xf0(%rbp), %xmm3
140925169:     	movaps	%xmm3, 0x1a0(%rbp)
140925170:     	movaps	%xmm2, 0x190(%rbp)
140925177:     	movaps	%xmm1, 0x180(%rbp)
14092517e:     	movaps	%xmm0, 0x170(%rbp)
140925185:     	movsd	%xmm6, (%rbp)
14092518a:     	movl	$0x3f800000, 0x8(%rbp)  # imm = 0x3F800000
140925191:     	leaq	0xc0(%rbp), %rsi
140925198:     	leaq	0x170(%rbp), %rdx
14092519f:     	movq	%rbp, %r8
1409251a2:     	movq	%rsi, %rcx
1409251a5:     	callq	0x140a07680 <.text+0xa06680>
1409251aa:     	movq	%r15, 0x20(%rsp)
1409251af:     	leaq	0x2189aba(%rip), %rdx   # 0x142aaec70
1409251b6:     	movl	$0x14, %r8d
1409251bc:     	movq	%rsi, %rcx
1409251bf:     	movq	%rdi, %r9
1409251c2:     	callq	0x140b1aca0 <.text+0xb19ca0>
1409251c7:     	movq	%rax, -0x20(%rbp)
1409251cb:     	leaq	0x2189ab2(%rip), %rdx   # 0x142aaec84
1409251d2:     	leaq	0xc0(%rbp), %rcx
1409251d9:     	movl	$0x21, %r8d
1409251df:     	callq	0x140b0eeb0 <.text+0xb0deb0>
1409251e4:     	xorl	%eax, %eax
1409251e6:     	cmpq	0xc0(%rbp), %rax
1409251ed:     	jo	0x140926665 <.text+0x925665>
1409251f3:     	movq	0x130(%rbp), %rax
1409251fa:     	movq	%rax, 0x1e0(%rbp)
140925201:     	movups	0x120(%rbp), %xmm0
140925208:     	movaps	%xmm0, 0x1d0(%rbp)
14092520f:     	movups	0x110(%rbp), %xmm0
140925216:     	movaps	%xmm0, 0x1c0(%rbp)
14092521d:     	movups	0x100(%rbp), %xmm0
140925224:     	movaps	%xmm0, 0x1b0(%rbp)
14092522b:     	movups	0xc0(%rbp), %xmm0
140925232:     	movups	0xd0(%rbp), %xmm1
140925239:     	movups	0xe0(%rbp), %xmm2
140925240:     	movups	0xf0(%rbp), %xmm3
140925247:     	movaps	%xmm3, 0x1a0(%rbp)
14092524e:     	movaps	%xmm2, 0x190(%rbp)
140925255:     	movaps	%xmm1, 0x180(%rbp)
14092525c:     	movaps	%xmm0, 0x170(%rbp)
140925263:     	movsd	%xmm6, (%rbp)
140925268:     	movl	$0x3f800000, 0x8(%rbp)  # imm = 0x3F800000
14092526f:     	leaq	0xc0(%rbp), %rbx
140925276:     	leaq	0x170(%rbp), %r13
14092527d:     	movq	%rbp, %rsi
140925280:     	movq	%rbx, %rcx
140925283:     	movq	%r13, %rdx
140925286:     	movq	%rsi, %r8
140925289:     	callq	0x140a07680 <.text+0xa06680>
14092528e:     	movq	%r15, 0xb0(%rbp)
140925295:     	movq	%r15, 0x20(%rsp)
14092529a:     	leaq	0x2189a1f(%rip), %rdx   # 0x142aaecc0
1409252a1:     	movl	$0xc, %r8d
1409252a7:     	movq	%rbx, %rcx
1409252aa:     	movq	%rdi, 0xa8(%rbp)
1409252b1:     	movq	%rdi, %r9
1409252b4:     	callq	0x140b1aca0 <.text+0xb19ca0>
1409252b9:     	movq	%rax, 0x98(%rbp)
1409252c0:     	leaq	0x2189a05(%rip), %rax   # 0x142aaeccc
1409252c7:     	movq	%rax, 0xc0(%rbp)
1409252ce:     	movq	$0x1a, 0xc8(%rbp)
1409252d9:     	leaq	0x2189a06(%rip), %rax   # 0x142aaece6
1409252e0:     	movq	%rax, 0xd0(%rbp)
1409252e7:     	movq	$0x1e, 0xd8(%rbp)
1409252f2:     	leaq	0x2189a0b(%rip), %rax   # 0x142aaed04
1409252f9:     	movq	%rax, 0xe0(%rbp)
140925300:     	movq	$0x18, 0xe8(%rbp)
14092530b:     	leaq	0x2189a0a(%rip), %rax   # 0x142aaed1c
140925312:     	movq	%rax, 0xf0(%rbp)
140925319:     	movq	$0xa, 0xf8(%rbp)
140925324:     	leaq	0x21899fb(%rip), %rax   # 0x142aaed26
14092532b:     	movq	%rax, 0x100(%rbp)
140925332:     	movq	$0x12, 0x108(%rbp)
14092533d:     	leaq	0x21899f4(%rip), %rax   # 0x142aaed38
140925344:     	movq	%rax, 0x110(%rbp)
14092534b:     	movq	$0xc, 0x118(%rbp)
140925356:     	leaq	0x21899e7(%rip), %rax   # 0x142aaed44
14092535d:     	movq	%rax, 0x120(%rbp)
140925364:     	movq	$0x14, 0x128(%rbp)
14092536f:     	leaq	0x21899e2(%rip), %rax   # 0x142aaed58
140925376:     	movq	%rax, 0x130(%rbp)
14092537d:     	movq	$0x1d, 0x138(%rbp)
140925388:     	leaq	0x21899e6(%rip), %rax   # 0x142aaed75
14092538f:     	movq	%rax, 0x140(%rbp)
140925396:     	movq	$0xa, 0x148(%rbp)
1409253a1:     	leaq	0x21899d7(%rip), %rax   # 0x142aaed7f
1409253a8:     	movq	%rax, 0x150(%rbp)
1409253af:     	movq	$0x12, 0x158(%rbp)
1409253ba:     	movq	$0x0, 0x160(%rbp)
1409253c5:     	movq	$0xa, 0x168(%rbp)
1409253d0:     	movl	$0x8, %r12d
1409253d6:     	leaq	-0x81b54d(%rip), %r14   # 0x140109e90 <.text+0x108e90>
1409253dd:     	leaq	0x2189b0c(%rip), %rdi   # 0x142aaeef0
1409253e4:     	leaq	0x80(%rbp), %rbx
1409253eb:     	xorl	%r15d, %r15d
1409253ee:     	nop
1409253f0:     	movups	0xb8(%rbp,%r12), %xmm0
1409253f9:     	movups	%xmm0, -0x40(%rbp)
1409253fd:     	leaq	-0x40(%rbp), %rax
140925401:     	movq	%rax, 0x80(%rbp)
140925408:     	movq	%r14, 0x88(%rbp)
14092540f:     	movq	%rdi, 0x170(%rbp)
140925416:     	movq	$0x2, 0x178(%rbp)
140925421:     	movq	%rbx, 0x180(%rbp)
140925428:     	movq	$0x1, 0x188(%rbp)
140925433:     	movq	$0x0, 0x190(%rbp)
14092543e:     	movq	%rsi, %rcx
140925441:     	movq	%r13, %rdx
140925444:     	callq	0x140d79f90 <.text+0xd78f90>
140925449:     	movq	%r13, %rcx
14092544c:     	movq	%rsi, %rdx
14092544f:     	callq	0x140b10ed0 <.text+0xb0fed0>
140925454:     	cmpq	0x170(%rbp), %r15
14092545b:     	jo	0x14092647c <.text+0x92547c>
140925461:     	movq	0x1e0(%rbp), %rax
140925468:     	movq	%rax, 0x70(%rbp)
14092546c:     	movups	0x1d0(%rbp), %xmm0
140925473:     	movaps	%xmm0, 0x60(%rbp)
140925477:     	movups	0x1c0(%rbp), %xmm0
14092547e:     	movaps	%xmm0, 0x50(%rbp)
140925482:     	movups	0x1b0(%rbp), %xmm0
140925489:     	movaps	%xmm0, 0x40(%rbp)
14092548d:     	movups	0x170(%rbp), %xmm0
140925494:     	movups	0x180(%rbp), %xmm1
14092549b:     	movups	0x190(%rbp), %xmm2
1409254a2:     	movups	0x1a0(%rbp), %xmm3
1409254a9:     	movaps	%xmm3, 0x30(%rbp)
1409254ad:     	movaps	%xmm2, 0x20(%rbp)
1409254b1:     	movaps	%xmm1, 0x10(%rbp)
1409254b5:     	movaps	%xmm0, (%rbp)
1409254b9:     	movsd	%xmm6, 0x80(%rbp)
1409254c1:     	movl	$0x3f800000, 0x88(%rbp) # imm = 0x3F800000
1409254cb:     	movq	%r13, %rcx
1409254ce:     	movq	%rsi, %rdx
1409254d1:     	movq	%rbx, %r8
1409254d4:     	callq	0x140a07680 <.text+0xa06680>
1409254d9:     	movq	-0x40(%rbp), %rdx
1409254dd:     	movq	-0x38(%rbp), %r8
1409254e1:     	movq	0xb0(%rbp), %rax
1409254e8:     	movq	%rax, 0x20(%rsp)
1409254ed:     	movq	%r13, %rcx
1409254f0:     	movq	0xa8(%rbp), %r9
1409254f7:     	callq	0x140b1aca0 <.text+0xb19ca0>
1409254fc:     	addq	$0x10, %r12
140925500:     	cmpq	$0xa8, %r12
140925507:     	jne	0x1409253f0 <.text+0x9243f0>
14092550d:     	leaq	0x218987d(%rip), %rdx   # 0x142aaed91
140925514:     	movl	$0x13, %r8d
14092551a:     	movq	0xa0(%rbp), %rcx
140925521:     	callq	0x140937aa0 <.text+0x936aa0>
140925526:     	testb	$0x1, %al
140925528:     	je	0x1409266a3 <.text+0x9256a3>
14092552e:     	movq	%rdx, %r13
140925531:     	leaq	0xc0(%rbp), %rcx
140925538:     	movq	0x288(%rbp), %r14
14092553f:     	movq	%r14, %rdx
140925542:     	callq	0x140d146a0 <.text+0xd136a0>
140925547:     	movq	0xc8(%rbp), %rsi
14092554e:     	testq	%rsi, %rsi
140925551:     	movq	0xb0(%rbp), %r15
140925558:     	movq	-0x30(%rbp), %r12
14092555c:     	jne	0x140925565 <.text+0x924565>
14092555e:     	movq	0xd0(%rbp), %rsi
140925565:     	movq	0xe0(%rbp), %rdi
14092556c:     	movq	(%rsi), %rax
14092556f:     	movq	0x10(%rsi), %rbx
140925573:     	subq	%rbx, %rax
140925576:     	cmpq	$0x87, %rax
14092557c:     	jbe	0x140926207 <.text+0x925207>
140925582:     	movq	0x8(%rsi), %rax
140925586:     	leaq	0x9b2553(%rip), %rcx    # 0x1412d7ae0 <.text+0x12d6ae0>
14092558d:     	movq	%rcx, (%rax,%rbx)
140925591:     	movaps	0x2018b28(%rip), %xmm0  # 0x14293e0c0
140925598:     	movups	%xmm0, 0x8(%rax,%rbx)
14092559d:     	movq	$0x0, 0x18(%rax,%rbx)
1409255a6:     	movl	$0x0, 0x20(%rax,%rbx)
1409255ae:     	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
1409255b8:     	movq	%rcx, 0x24(%rax,%rbx)
1409255bd:     	movl	$0x3f800000, 0x2c(%rax,%rbx) # imm = 0x3F800000
1409255c5:     	movq	-0x8(%rbp), %rcx
1409255c9:     	movq	%rcx, 0x38(%rax,%rbx)
1409255ce:     	movq	%r13, 0x40(%rax,%rbx)
1409255d3:     	movw	$0x1, 0x48(%rax,%rbx)
1409255da:     	movb	$0x0, 0x58(%rax,%rbx)
1409255df:     	movq	%rdi, 0x68(%rax,%rbx)
1409255e4:     	leaq	0xc3f745(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1409255eb:     	movq	%rcx, 0x78(%rax,%rbx)
1409255f0:     	addq	$0x88, %rbx
1409255f7:     	movq	%rbx, 0x10(%rsi)
1409255fb:     	leaq	0xc0(%rbp), %rcx
140925602:     	movq	%r14, %rdx
140925605:     	callq	0x140d146a0 <.text+0xd136a0>
14092560a:     	movq	0xc8(%rbp), %rsi
140925611:     	testq	%rsi, %rsi
140925614:     	jne	0x14092561d <.text+0x92461d>
140925616:     	movq	0xd0(%rbp), %rsi
14092561d:     	movq	0xe0(%rbp), %rdi
140925624:     	movq	(%rsi), %rax
140925627:     	movq	0x10(%rsi), %rbx
14092562b:     	subq	%rbx, %rax
14092562e:     	cmpq	$0x37, %rax
140925632:     	jbe	0x14092622c <.text+0x92522c>
140925638:     	movq	0x8(%rsi), %rax
14092563c:     	leaq	0x9b1a4d(%rip), %rcx    # 0x1412d7090 <.text+0x12d6090>
140925643:     	movq	%rcx, (%rax,%rbx)
140925647:     	leaq	0xc3f6e2(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
14092564e:     	movq	%rcx, 0x8(%rax,%rbx)
140925653:     	movq	%rdi, 0x10(%rax,%rbx)
140925658:     	movq	%r12, 0x18(%rax,%rbx)
14092565d:     	movq	%r13, 0x20(%rax,%rbx)
140925662:     	movw	$0x300, 0x28(%rax,%rbx) # imm = 0x300
140925669:     	movb	$0x0, 0x30(%rax,%rbx)
14092566e:     	addq	$0x38, %rbx
140925672:     	movq	%rbx, 0x10(%rsi)
140925676:     	leaq	0xc0(%rbp), %rcx
14092567d:     	movq	%r14, %rdx
140925680:     	callq	0x140d146a0 <.text+0xd136a0>
140925685:     	movq	0xc8(%rbp), %rsi
14092568c:     	testq	%rsi, %rsi
14092568f:     	jne	0x140925698 <.text+0x924698>
140925691:     	movq	0xd0(%rbp), %rsi
140925698:     	movq	0xe0(%rbp), %r14
14092569f:     	movq	(%rsi), %rax
1409256a2:     	movq	0x10(%rsi), %rbx
1409256a6:     	subq	%rbx, %rax
1409256a9:     	cmpq	$0x37, %rax
1409256ad:     	jbe	0x140926251 <.text+0x925251>
1409256b3:     	movq	0x8(%rsi), %rax
1409256b7:     	leaq	0x9b0da2(%rip), %rdi    # 0x1412d6460 <.text+0x12d5460>
1409256be:     	movq	%rdi, (%rax,%rbx)
1409256c2:     	leaq	0xc3f667(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1409256c9:     	movq	%rcx, 0x8(%rax,%rbx)
1409256ce:     	movq	%r14, 0x10(%rax,%rbx)
1409256d3:     	movq	%r12, 0x18(%rax,%rbx)
1409256d8:     	movq	%r13, 0x20(%rax,%rbx)
1409256dd:     	movw	$0x4, 0x28(%rax,%rbx)
1409256e4:     	movb	$0x0, 0x30(%rax,%rbx)
1409256e9:     	addq	$0x38, %rbx
1409256ed:     	movq	%rbx, 0x10(%rsi)
1409256f1:     	leaq	0xc0(%rbp), %rcx
1409256f8:     	movq	0x288(%rbp), %rdx
1409256ff:     	callq	0x140d146a0 <.text+0xd136a0>
140925704:     	movq	0xc8(%rbp), %rsi
14092570b:     	testq	%rsi, %rsi
14092570e:     	jne	0x140925717 <.text+0x924717>
140925710:     	movq	0xd0(%rbp), %rsi
140925717:     	movq	0xe0(%rbp), %r14
14092571e:     	movq	(%rsi), %rax
140925721:     	movq	0x10(%rsi), %rbx
140925725:     	subq	%rbx, %rax
140925728:     	cmpq	$0x37, %rax
14092572c:     	jbe	0x140926276 <.text+0x925276>
140925732:     	movq	0x8(%rsi), %rax
140925736:     	movq	%rdi, (%rax,%rbx)
14092573a:     	leaq	0xc3f5ef(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140925741:     	movq	%rcx, 0x8(%rax,%rbx)
140925746:     	movq	%r14, 0x10(%rax,%rbx)
14092574b:     	movq	%r12, 0x18(%rax,%rbx)
140925750:     	movq	%r13, 0x20(%rax,%rbx)
140925755:     	movw	$0x5, 0x28(%rax,%rbx)
14092575c:     	movb	$0x0, 0x30(%rax,%rbx)
140925761:     	addq	$0x38, %rbx
140925765:     	movq	%rbx, 0x10(%rsi)
140925769:     	leaq	0xc0(%rbp), %rcx
140925770:     	movq	0x288(%rbp), %r14
140925777:     	movq	%r14, %rdx
14092577a:     	callq	0x140d146a0 <.text+0xd136a0>
14092577f:     	movq	0xc8(%rbp), %rsi
140925786:     	testq	%rsi, %rsi
140925789:     	jne	0x140925792 <.text+0x924792>
14092578b:     	movq	0xd0(%rbp), %rsi
140925792:     	movq	0xe0(%rbp), %rdi
140925799:     	movq	(%rsi), %rax
14092579c:     	movq	0x10(%rsi), %rbx
1409257a0:     	subq	%rbx, %rax
1409257a3:     	cmpq	$0x37, %rax
1409257a7:     	jbe	0x14092629b <.text+0x92529b>
1409257ad:     	movq	0x8(%rsi), %rax
1409257b1:     	leaq	0x9b1f18(%rip), %r12    # 0x1412d76d0 <.text+0x12d66d0>
1409257b8:     	movq	%r12, (%rax,%rbx)
1409257bc:     	leaq	0xc3f56d(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1409257c3:     	movq	%rcx, 0x8(%rax,%rbx)
1409257c8:     	movq	%rdi, 0x10(%rax,%rbx)
1409257cd:     	movq	-0x10(%rbp), %rcx
1409257d1:     	movq	%rcx, 0x18(%rax,%rbx)
1409257d6:     	movq	%r13, 0x20(%rax,%rbx)
1409257db:     	movw	$0x6, 0x28(%rax,%rbx)
1409257e2:     	movb	$0x0, 0x30(%rax,%rbx)
1409257e7:     	addq	$0x38, %rbx
1409257eb:     	movq	%rbx, 0x10(%rsi)
1409257ef:     	leaq	0xc0(%rbp), %rcx
1409257f6:     	movq	%r14, %rdx
1409257f9:     	callq	0x140d146a0 <.text+0xd136a0>
1409257fe:     	movq	0xc8(%rbp), %rsi
140925805:     	testq	%rsi, %rsi
140925808:     	jne	0x140925811 <.text+0x924811>
14092580a:     	movq	0xd0(%rbp), %rsi
140925811:     	movq	0xe0(%rbp), %rdi
140925818:     	movq	(%rsi), %rax
14092581b:     	movq	0x10(%rsi), %rbx
14092581f:     	subq	%rbx, %rax
140925822:     	cmpq	$0x37, %rax
140925826:     	jbe	0x1409262c0 <.text+0x9252c0>
14092582c:     	movq	0x8(%rsi), %rax
140925830:     	movq	%r12, (%rax,%rbx)
140925834:     	leaq	0xc3f4f5(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
14092583b:     	movq	%rcx, 0x8(%rax,%rbx)
140925840:     	movq	%rdi, 0x10(%rax,%rbx)
140925845:     	movq	0xb8(%rbp), %rcx
14092584c:     	movq	%rcx, 0x18(%rax,%rbx)
140925851:     	movq	%r13, 0x20(%rax,%rbx)
140925856:     	movw	$0x7, 0x28(%rax,%rbx)
14092585d:     	movb	$0x0, 0x30(%rax,%rbx)
140925862:     	addq	$0x38, %rbx
140925866:     	movq	%rbx, 0x10(%rsi)
14092586a:     	leaq	0xc0(%rbp), %rcx
140925871:     	movq	%r14, %rdx
140925874:     	callq	0x140d146a0 <.text+0xd136a0>
140925879:     	movq	0xc8(%rbp), %rsi
140925880:     	testq	%rsi, %rsi
140925883:     	jne	0x14092588c <.text+0x92488c>
140925885:     	movq	0xd0(%rbp), %rsi
14092588c:     	movq	0xe0(%rbp), %rdi
140925893:     	movq	(%rsi), %rax
140925896:     	movq	0x10(%rsi), %rbx
14092589a:     	subq	%rbx, %rax
14092589d:     	cmpq	$0x37, %rax
1409258a1:     	jbe	0x1409262e5 <.text+0x9252e5>
1409258a7:     	movq	0x8(%rsi), %rax
1409258ab:     	movq	%r12, (%rax,%rbx)
1409258af:     	leaq	0xc3f47a(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1409258b6:     	movq	%rcx, 0x8(%rax,%rbx)
1409258bb:     	movq	%rdi, 0x10(%rax,%rbx)
1409258c0:     	movq	0xb8(%rbp), %rcx
1409258c7:     	movq	%rcx, 0x18(%rax,%rbx)
1409258cc:     	movq	%r13, 0x20(%rax,%rbx)
1409258d1:     	movw	$0x8, 0x28(%rax,%rbx)
1409258d8:     	movb	$0x0, 0x30(%rax,%rbx)
1409258dd:     	addq	$0x38, %rbx
1409258e1:     	movq	%rbx, 0x10(%rsi)
1409258e5:     	leaq	0xc0(%rbp), %rcx
1409258ec:     	movq	%r14, %rdx
1409258ef:     	callq	0x140d146a0 <.text+0xd136a0>
1409258f4:     	movq	0xc8(%rbp), %rsi
1409258fb:     	testq	%rsi, %rsi
1409258fe:     	jne	0x140925907 <.text+0x924907>
140925900:     	movq	0xd0(%rbp), %rsi
140925907:     	movq	0xe0(%rbp), %rdi
14092590e:     	movq	(%rsi), %rax
140925911:     	movq	0x10(%rsi), %rbx
140925915:     	subq	%rbx, %rax
140925918:     	cmpq	$0x37, %rax
14092591c:     	jbe	0x14092630a <.text+0x92530a>
140925922:     	movq	0x8(%rsi), %rax
140925926:     	movq	%r12, (%rax,%rbx)
14092592a:     	leaq	0xc3f3ff(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140925931:     	movq	%rcx, 0x8(%rax,%rbx)
140925936:     	movq	%rdi, 0x10(%rax,%rbx)
14092593b:     	movq	0xb8(%rbp), %rcx
140925942:     	movq	%rcx, 0x18(%rax,%rbx)
140925947:     	movq	%r13, 0x20(%rax,%rbx)
14092594c:     	movw	$0x9, 0x28(%rax,%rbx)
140925953:     	movb	$0x0, 0x30(%rax,%rbx)
140925958:     	addq	$0x38, %rbx
14092595c:     	movq	%rbx, 0x10(%rsi)
140925960:     	leaq	0xc0(%rbp), %rcx
140925967:     	movq	%r14, %rdx
14092596a:     	callq	0x140d146a0 <.text+0xd136a0>
14092596f:     	movq	0xc8(%rbp), %rsi
140925976:     	testq	%rsi, %rsi
140925979:     	jne	0x140925982 <.text+0x924982>
14092597b:     	movq	0xd0(%rbp), %rsi
140925982:     	movq	0xe0(%rbp), %rdi
140925989:     	movq	(%rsi), %rax
14092598c:     	movq	0x10(%rsi), %rbx
140925990:     	subq	%rbx, %rax
140925993:     	cmpq	$0x37, %rax
140925997:     	jbe	0x14092632f <.text+0x92532f>
14092599d:     	movq	0x8(%rsi), %rax
1409259a1:     	movq	%r12, (%rax,%rbx)
1409259a5:     	leaq	0xc3f384(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1409259ac:     	movq	%rcx, 0x8(%rax,%rbx)
1409259b1:     	movq	%rdi, 0x10(%rax,%rbx)
1409259b6:     	movq	0xb8(%rbp), %rcx
1409259bd:     	movq	%rcx, 0x18(%rax,%rbx)
1409259c2:     	movq	%r13, 0x20(%rax,%rbx)
1409259c7:     	movw	$0xa, 0x28(%rax,%rbx)
1409259ce:     	movb	$0x0, 0x30(%rax,%rbx)
1409259d3:     	addq	$0x38, %rbx
1409259d7:     	movq	%rbx, 0x10(%rsi)
1409259db:     	leaq	0xc0(%rbp), %rcx
1409259e2:     	movq	%r14, %rdx
1409259e5:     	callq	0x140d146a0 <.text+0xd136a0>
1409259ea:     	movq	0xc8(%rbp), %rsi
1409259f1:     	testq	%rsi, %rsi
1409259f4:     	jne	0x1409259fd <.text+0x9249fd>
1409259f6:     	movq	0xd0(%rbp), %rsi
1409259fd:     	movq	0xe0(%rbp), %rdi
140925a04:     	movq	(%rsi), %rax
140925a07:     	movq	0x10(%rsi), %rbx
140925a0b:     	subq	%rbx, %rax
140925a0e:     	cmpq	$0x37, %rax
140925a12:     	movq	0xa8(%rbp), %r12
140925a19:     	jbe	0x140926354 <.text+0x925354>
140925a1f:     	movq	0x8(%rsi), %rax
140925a23:     	leaq	0x9b6556(%rip), %rcx    # 0x1412dbf80 <.text+0x12daf80>
140925a2a:     	movq	%rcx, (%rax,%rbx)
140925a2e:     	leaq	0xc3f2fb(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140925a35:     	movq	%rcx, 0x8(%rax,%rbx)
140925a3a:     	movq	%rdi, 0x10(%rax,%rbx)
140925a3f:     	movq	-0x18(%rbp), %rcx
140925a43:     	movq	%rcx, 0x18(%rax,%rbx)
140925a48:     	movq	%r13, 0x20(%rax,%rbx)
140925a4d:     	movb	$0x2, 0x28(%rax,%rbx)
140925a52:     	movb	$0x0, 0x30(%rax,%rbx)
140925a57:     	addq	$0x38, %rbx
140925a5b:     	movq	%rbx, 0x10(%rsi)
140925a5f:     	leaq	0xc0(%rbp), %rcx
140925a66:     	movq	%r14, %rdx
140925a69:     	callq	0x140d146a0 <.text+0xd136a0>
140925a6e:     	movq	0xc8(%rbp), %rsi
140925a75:     	testq	%rsi, %rsi
140925a78:     	jne	0x140925a81 <.text+0x924a81>
140925a7a:     	movq	0xd0(%rbp), %rsi
140925a81:     	movq	0xe0(%rbp), %r14
140925a88:     	movq	(%rsi), %rax
140925a8b:     	movq	0x10(%rsi), %rbx
140925a8f:     	subq	%rbx, %rax
140925a92:     	cmpq	$0x3f, %rax
140925a96:     	jbe	0x140926379 <.text+0x925379>
140925a9c:     	movq	0x8(%rsi), %rax
140925aa0:     	leaq	0x9b2589(%rip), %rdi    # 0x1412d8030 <.text+0x12d7030>
140925aa7:     	movq	%rdi, (%rax,%rbx)
140925aab:     	leaq	0xc3f27e(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140925ab2:     	movq	%rcx, 0x8(%rax,%rbx)
140925ab7:     	movq	%r14, 0x10(%rax,%rbx)
140925abc:     	movq	0x98(%rbp), %rcx
140925ac3:     	movq	%rcx, 0x18(%rax,%rbx)
140925ac8:     	movq	%r13, 0x20(%rax,%rbx)
140925acd:     	movq	$0x1, 0x28(%rax,%rbx)
140925ad6:     	movb	$0xd, 0x30(%rax,%rbx)
140925adb:     	movb	$0x0, 0x38(%rax,%rbx)
140925ae0:     	addq	$0x40, %rbx
140925ae4:     	movq	%rbx, 0x10(%rsi)
140925ae8:     	leaq	0xc0(%rbp), %rcx
140925aef:     	movq	0x288(%rbp), %rdx
140925af6:     	callq	0x140d146a0 <.text+0xd136a0>
140925afb:     	movq	0xc8(%rbp), %rsi
140925b02:     	testq	%rsi, %rsi
140925b05:     	jne	0x140925b0e <.text+0x924b0e>
140925b07:     	movq	0xd0(%rbp), %rsi
140925b0e:     	movq	0xe0(%rbp), %r14
140925b15:     	movq	(%rsi), %rax
140925b18:     	movq	0x10(%rsi), %rbx
140925b1c:     	subq	%rbx, %rax
140925b1f:     	cmpq	$0x3f, %rax
140925b23:     	jbe	0x14092639e <.text+0x92539e>
140925b29:     	movq	0x8(%rsi), %rax
140925b2d:     	movq	%rdi, (%rax,%rbx)
140925b31:     	leaq	0xc3f1f8(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140925b38:     	movq	%rcx, 0x8(%rax,%rbx)
140925b3d:     	movq	%r14, 0x10(%rax,%rbx)
140925b42:     	movq	0x98(%rbp), %rcx
140925b49:     	movq	%rcx, 0x18(%rax,%rbx)
140925b4e:     	movq	%r13, 0x20(%rax,%rbx)
140925b53:     	movq	$0x2, 0x28(%rax,%rbx)
140925b5c:     	movb	$0xd, 0x30(%rax,%rbx)
140925b61:     	movb	$0x0, 0x38(%rax,%rbx)
140925b66:     	addq	$0x40, %rbx
140925b6a:     	movq	%rbx, 0x10(%rsi)
140925b6e:     	leaq	0xc0(%rbp), %rcx
140925b75:     	movq	0x288(%rbp), %rdx
140925b7c:     	callq	0x140d146a0 <.text+0xd136a0>
140925b81:     	movq	0xc8(%rbp), %rsi
140925b88:     	testq	%rsi, %rsi
140925b8b:     	jne	0x140925b94 <.text+0x924b94>
140925b8d:     	movq	0xd0(%rbp), %rsi
140925b94:     	movq	0xe0(%rbp), %r14
140925b9b:     	movq	(%rsi), %rax
140925b9e:     	movq	0x10(%rsi), %rbx
140925ba2:     	subq	%rbx, %rax
140925ba5:     	cmpq	$0x3f, %rax
140925ba9:     	jbe	0x1409263c3 <.text+0x9253c3>
140925baf:     	movq	0x8(%rsi), %rax
140925bb3:     	leaq	0x9b7396(%rip), %rdi    # 0x1412dcf50 <.text+0x12dbf50>
140925bba:     	movq	%rdi, (%rax,%rbx)
140925bbe:     	leaq	0xc3f16b(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140925bc5:     	movq	%rcx, 0x8(%rax,%rbx)
140925bca:     	movq	%r14, 0x10(%rax,%rbx)
140925bcf:     	movq	-0x28(%rbp), %rcx
140925bd3:     	movq	%rcx, 0x18(%rax,%rbx)
140925bd8:     	movq	0x90(%rbp), %rcx
140925bdf:     	movq	%rcx, 0x20(%rax,%rbx)
140925be4:     	movq	$0x0, 0x28(%rax,%rbx)
140925bed:     	movw	$0xb01, 0x30(%rax,%rbx) # imm = 0xB01
140925bf4:     	movb	$0x0, 0x38(%rax,%rbx)
140925bf9:     	addq	$0x40, %rbx
140925bfd:     	movq	%rbx, 0x10(%rsi)
140925c01:     	leaq	0xc0(%rbp), %rcx
140925c08:     	movq	0x288(%rbp), %rdx
140925c0f:     	callq	0x140d146a0 <.text+0xd136a0>
140925c14:     	movq	0xc8(%rbp), %rsi
140925c1b:     	testq	%rsi, %rsi
140925c1e:     	jne	0x140925c27 <.text+0x924c27>
140925c20:     	movq	0xd0(%rbp), %rsi
140925c27:     	movq	0xe0(%rbp), %r14
140925c2e:     	movq	(%rsi), %rax
140925c31:     	movq	0x10(%rsi), %rbx
140925c35:     	subq	%rbx, %rax
140925c38:     	cmpq	$0x3f, %rax
140925c3c:     	jbe	0x1409263e8 <.text+0x9253e8>
140925c42:     	movq	0x8(%rsi), %rax
140925c46:     	movq	%rdi, (%rax,%rbx)
140925c4a:     	leaq	0xc3f0df(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140925c51:     	movq	%rcx, 0x8(%rax,%rbx)
140925c56:     	movq	%r14, 0x10(%rax,%rbx)
140925c5b:     	movq	-0x20(%rbp), %rcx
140925c5f:     	movq	%rcx, 0x18(%rax,%rbx)
140925c64:     	movq	0x90(%rbp), %rcx
140925c6b:     	movq	%rcx, 0x20(%rax,%rbx)
140925c70:     	movq	$0x0, 0x28(%rax,%rbx)
140925c79:     	movw	$0xc01, 0x30(%rax,%rbx) # imm = 0xC01
140925c80:     	movb	$0x0, 0x38(%rax,%rbx)
140925c85:     	addq	$0x40, %rbx
140925c89:     	movq	%rbx, 0x10(%rsi)
140925c8d:     	xorl	%esi, %esi
140925c8f:     	leaq	0xc0(%rbp), %rcx
140925c96:     	movss	0x1fffc66(%rip), %xmm2  # 0x142925904
140925c9e:     	xorl	%edx, %edx
140925ca0:     	callq	0x140a06b00 <.text+0xa05b00>
140925ca5:     	cmpq	0xc0(%rbp), %rsi
140925cac:     	jo	0x1409266af <.text+0x9256af>
140925cb2:     	movq	0x130(%rbp), %rax
140925cb9:     	movq	%rax, 0x1e0(%rbp)
140925cc0:     	movups	0x120(%rbp), %xmm0
140925cc7:     	movaps	%xmm0, 0x1d0(%rbp)
140925cce:     	movups	0x110(%rbp), %xmm0
140925cd5:     	movaps	%xmm0, 0x1c0(%rbp)
140925cdc:     	movups	0x100(%rbp), %xmm0
140925ce3:     	movaps	%xmm0, 0x1b0(%rbp)
140925cea:     	movups	0xc0(%rbp), %xmm0
140925cf1:     	movups	0xd0(%rbp), %xmm1
140925cf8:     	movups	0xe0(%rbp), %xmm2
140925cff:     	movups	0xf0(%rbp), %xmm3
140925d06:     	movaps	%xmm3, 0x1a0(%rbp)
140925d0d:     	movaps	%xmm2, 0x190(%rbp)
140925d14:     	movaps	%xmm1, 0x180(%rbp)
140925d1b:     	movaps	%xmm0, 0x170(%rbp)
140925d22:     	movsd	%xmm6, (%rbp)
140925d27:     	movl	$0x3f800000, 0x8(%rbp)  # imm = 0x3F800000
140925d2e:     	leaq	0xc0(%rbp), %rsi
140925d35:     	leaq	0x170(%rbp), %rdx
140925d3c:     	movq	%rbp, %r8
140925d3f:     	movq	%rsi, %rcx
140925d42:     	callq	0x140a07680 <.text+0xa06680>
140925d47:     	movq	%r15, 0x20(%rsp)
140925d4c:     	leaq	0x2189085(%rip), %rdx   # 0x142aaedd8
140925d53:     	movl	$0xb, %r8d
140925d59:     	movq	%rsi, %rcx
140925d5c:     	movq	%r12, %r9
140925d5f:     	callq	0x140b1aca0 <.text+0xb19ca0>
140925d64:     	leaq	0x2189078(%rip), %rdx   # 0x142aaede3
140925d6b:     	leaq	0xc0(%rbp), %rcx
140925d72:     	movl	$0x17, %r8d
140925d78:     	callq	0x140b0eeb0 <.text+0xb0deb0>
140925d7d:     	xorl	%eax, %eax
140925d7f:     	cmpq	0xc0(%rbp), %rax
140925d86:     	movq	0x288(%rbp), %rdi
140925d8d:     	jo	0x1409266fb <.text+0x9256fb>
140925d93:     	movq	0x130(%rbp), %rax
140925d9a:     	movq	%rax, 0x1e0(%rbp)
140925da1:     	movups	0x120(%rbp), %xmm0
140925da8:     	movaps	%xmm0, 0x1d0(%rbp)
140925daf:     	movups	0x110(%rbp), %xmm0
140925db6:     	movaps	%xmm0, 0x1c0(%rbp)
140925dbd:     	movups	0x100(%rbp), %xmm0
140925dc4:     	movaps	%xmm0, 0x1b0(%rbp)
140925dcb:     	movups	0xc0(%rbp), %xmm0
140925dd2:     	movups	0xd0(%rbp), %xmm1
140925dd9:     	movups	0xe0(%rbp), %xmm2
140925de0:     	movups	0xf0(%rbp), %xmm3
140925de7:     	movaps	%xmm3, 0x1a0(%rbp)
140925dee:     	movaps	%xmm2, 0x190(%rbp)
140925df5:     	movaps	%xmm1, 0x180(%rbp)
140925dfc:     	movaps	%xmm0, 0x170(%rbp)
140925e03:     	movq	%r15, 0x20(%rsp)
140925e08:     	leaq	0x2189009(%rip), %rdx   # 0x142aaee18
140925e0f:     	leaq	0x170(%rbp), %rcx
140925e16:     	movl	$0x8, %r8d
140925e1c:     	movq	%r12, %r9
140925e1f:     	callq	0x140b1aca0 <.text+0xb19ca0>
140925e24:     	movq	%rax, %rsi
140925e27:     	leaq	0xc0(%rbp), %rcx
140925e2e:     	movq	%rdi, %rdx
140925e31:     	callq	0x140d146a0 <.text+0xd136a0>
140925e36:     	movq	0xc8(%rbp), %rbx
140925e3d:     	testq	%rbx, %rbx
140925e40:     	jne	0x140925e49 <.text+0x924e49>
140925e42:     	movq	0xd0(%rbp), %rbx
140925e49:     	movq	0xe0(%rbp), %rdi
140925e50:     	movq	(%rbx), %rax
140925e53:     	movq	0x10(%rbx), %r15
140925e57:     	subq	%r15, %rax
140925e5a:     	cmpq	$0x37, %rax
140925e5e:     	jbe	0x14092640d <.text+0x92540d>
140925e64:     	movq	0x8(%rbx), %rax
140925e68:     	leaq	0x9b2fb1(%rip), %rcx    # 0x1412d8e20 <.text+0x12d7e20>
140925e6f:     	movq	%rcx, (%rax,%r15)
140925e73:     	leaq	0xc3eeb6(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140925e7a:     	movq	%rcx, 0x8(%rax,%r15)
140925e7f:     	movq	%rdi, 0x10(%rax,%r15)
140925e84:     	movq	%rsi, 0x18(%rax,%r15)
140925e89:     	movq	%r13, 0x20(%rax,%r15)
140925e8e:     	movw	$0xe00, 0x28(%rax,%r15) # imm = 0xE00
140925e96:     	movb	$0x0, 0x30(%rax,%r15)
140925e9c:     	addq	$0x38, %r15
140925ea0:     	movq	%r15, 0x10(%rbx)
140925ea4:     	leaq	0x2188f75(%rip), %rdx   # 0x142aaee20
140925eab:     	leaq	0xc0(%rbp), %rcx
140925eb2:     	movl	$0x20, %r8d
140925eb8:     	callq	0x140b0eeb0 <.text+0xb0deb0>
140925ebd:     	xorl	%eax, %eax
140925ebf:     	cmpq	0xc0(%rbp), %rax
140925ec6:     	jo	0x140926739 <.text+0x925739>
140925ecc:     	movq	0x130(%rbp), %rax
140925ed3:     	movq	%rax, 0x1e0(%rbp)
140925eda:     	movups	0x120(%rbp), %xmm0
140925ee1:     	movaps	%xmm0, 0x1d0(%rbp)
140925ee8:     	movups	0x110(%rbp), %xmm0
140925eef:     	movaps	%xmm0, 0x1c0(%rbp)
140925ef6:     	movups	0x100(%rbp), %xmm0
140925efd:     	movaps	%xmm0, 0x1b0(%rbp)
140925f04:     	movups	0xc0(%rbp), %xmm0
140925f0b:     	movups	0xd0(%rbp), %xmm1
140925f12:     	movups	0xe0(%rbp), %xmm2
140925f19:     	movups	0xf0(%rbp), %xmm3
140925f20:     	movaps	%xmm3, 0x1a0(%rbp)
140925f27:     	movaps	%xmm2, 0x190(%rbp)
140925f2e:     	movaps	%xmm1, 0x180(%rbp)
140925f35:     	movaps	%xmm0, 0x170(%rbp)
140925f3c:     	movq	0xb0(%rbp), %rax
140925f43:     	movq	%rax, 0x20(%rsp)
140925f48:     	leaq	0x2188f09(%rip), %rdx   # 0x142aaee58
140925f4f:     	leaq	0x170(%rbp), %rcx
140925f56:     	movl	$0x6, %r8d
140925f5c:     	movq	%r12, %r9
140925f5f:     	callq	0x140b1aca0 <.text+0xb19ca0>
140925f64:     	movq	%rax, %rsi
140925f67:     	leaq	0x2188e23(%rip), %rdx   # 0x142aaed91
140925f6e:     	movl	$0x13, %r8d
140925f74:     	movq	0xa0(%rbp), %r14
140925f7b:     	movq	%r14, %rcx
140925f7e:     	callq	0x140937aa0 <.text+0x936aa0>
140925f83:     	testb	$0x1, %al
140925f85:     	je	0x140926774 <.text+0x925774>
140925f8b:     	movq	%rdx, %rbx
140925f8e:     	leaq	0xc0(%rbp), %rcx
140925f95:     	callq	0x140c9d180 <.text+0xc9c180>
140925f9a:     	movaps	0xc0(%rbp), %xmm6
140925fa1:     	leaq	0xc0(%rbp), %rcx
140925fa8:     	movq	0x288(%rbp), %rdx
140925faf:     	callq	0x140d146a0 <.text+0xd136a0>
140925fb4:     	movq	0xc8(%rbp), %r15
140925fbb:     	testq	%r15, %r15
140925fbe:     	jne	0x140925fc7 <.text+0x924fc7>
140925fc0:     	movq	0xd0(%rbp), %r15
140925fc7:     	movq	0xe0(%rbp), %rdi
140925fce:     	movq	(%r15), %rax
140925fd1:     	movq	0x10(%r15), %r12
140925fd5:     	subq	%r12, %rax
140925fd8:     	cmpq	$0x77, %rax
140925fdc:     	jbe	0x140926432 <.text+0x925432>
140925fe2:     	movq	0x8(%r15), %rax
140925fe6:     	leaq	0x9b75e3(%rip), %rcx    # 0x1412dd5d0 <.text+0x12dc5d0>
140925fed:     	movq	%rcx, (%rax,%r12)
140925ff1:     	movups	%xmm6, 0x8(%rax,%r12)
140925ff7:     	movsd	0x21808b1(%rip), %xmm6  # 0x142aa68b0
140925fff:     	movsd	%xmm6, 0x18(%rax,%r12)
140926006:     	movl	$0x80000000, 0x20(%rax,%r12) # imm = 0x80000000
14092600f:     	movq	$0x0, 0x24(%rax,%r12)
140926018:     	movl	$0x0, 0x2c(%rax,%r12)
140926021:     	movq	%rsi, 0x38(%rax,%r12)
140926026:     	movq	%rbx, 0x40(%rax,%r12)
14092602b:     	movb	$0x0, 0x48(%rax,%r12)
140926031:     	movq	%rdi, 0x58(%rax,%r12)
140926036:     	leaq	0xc3ecf3(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
14092603d:     	movq	%rcx, 0x68(%rax,%r12)
140926042:     	addq	$0x78, %r12
140926046:     	movq	%r12, 0x10(%r15)
14092604a:     	leaq	0x2188e27(%rip), %rdx   # 0x142aaee78
140926051:     	leaq	0xc0(%rbp), %rcx
140926058:     	movl	$0x1f, %r8d
14092605e:     	callq	0x140b0eeb0 <.text+0xb0deb0>
140926063:     	xorl	%eax, %eax
140926065:     	cmpq	0xc0(%rbp), %rax
14092606c:     	jo	0x140926780 <.text+0x925780>
140926072:     	movq	0x130(%rbp), %rax
140926079:     	movq	%rax, 0x1e0(%rbp)
140926080:     	movups	0x120(%rbp), %xmm0
140926087:     	movaps	%xmm0, 0x1d0(%rbp)
14092608e:     	movups	0x110(%rbp), %xmm0
140926095:     	movaps	%xmm0, 0x1c0(%rbp)
14092609c:     	movups	0x100(%rbp), %xmm0
1409260a3:     	movaps	%xmm0, 0x1b0(%rbp)
1409260aa:     	movups	0xc0(%rbp), %xmm0
1409260b1:     	movups	0xd0(%rbp), %xmm1
1409260b8:     	movups	0xe0(%rbp), %xmm2
1409260bf:     	movups	0xf0(%rbp), %xmm3
1409260c6:     	movaps	%xmm3, 0x1a0(%rbp)
1409260cd:     	movaps	%xmm2, 0x190(%rbp)
1409260d4:     	movaps	%xmm1, 0x180(%rbp)
1409260db:     	movaps	%xmm0, 0x170(%rbp)
1409260e2:     	movq	0xb0(%rbp), %rax
1409260e9:     	movq	%rax, 0x20(%rsp)
1409260ee:     	leaq	0x2188dbb(%rip), %rdx   # 0x142aaeeb0
1409260f5:     	leaq	0x170(%rbp), %rcx
1409260fc:     	movl	$0xd, %r8d
140926102:     	movq	0xa8(%rbp), %r9
140926109:     	callq	0x140b1aca0 <.text+0xb19ca0>
14092610e:     	movq	%rax, %rsi
140926111:     	leaq	0x2188c79(%rip), %rdx   # 0x142aaed91
140926118:     	movl	$0x13, %r8d
14092611e:     	movq	%r14, %rcx
140926121:     	callq	0x140937aa0 <.text+0x936aa0>
140926126:     	testb	$0x1, %al
140926128:     	je	0x1409267bb <.text+0x9257bb>
14092612e:     	movq	%rdx, %rdi
140926131:     	leaq	0xc0(%rbp), %rcx
140926138:     	callq	0x140c9d180 <.text+0xc9c180>
14092613d:     	movaps	0xc0(%rbp), %xmm7
140926144:     	leaq	0xc0(%rbp), %rcx
14092614b:     	movq	0x288(%rbp), %rdx
140926152:     	callq	0x140d146a0 <.text+0xd136a0>
140926157:     	movq	0xc8(%rbp), %rbx
14092615e:     	testq	%rbx, %rbx
140926161:     	jne	0x14092616a <.text+0x92516a>
140926163:     	movq	0xd0(%rbp), %rbx
14092616a:     	movq	0xe0(%rbp), %r15
140926171:     	movq	(%rbx), %rax
140926174:     	movq	0x10(%rbx), %r14
140926178:     	subq	%r14, %rax
14092617b:     	cmpq	$0x77, %rax
14092617f:     	jbe	0x140926457 <.text+0x925457>
140926185:     	movq	0x8(%rbx), %rax
140926189:     	leaq	0x9b1010(%rip), %rcx    # 0x1412d71a0 <.text+0x12d61a0>
140926190:     	movq	%rcx, (%rax,%r14)
140926194:     	movups	%xmm7, 0x8(%rax,%r14)
14092619a:     	movsd	%xmm6, 0x18(%rax,%r14)
1409261a1:     	movl	$0x80000000, 0x20(%rax,%r14) # imm = 0x80000000
1409261aa:     	movq	$0x0, 0x24(%rax,%r14)
1409261b3:     	movl	$0x0, 0x2c(%rax,%r14)
1409261bc:     	movq	%rsi, 0x38(%rax,%r14)
1409261c1:     	movq	%rdi, 0x40(%rax,%r14)
1409261c6:     	movb	$0x0, 0x48(%rax,%r14)
1409261cc:     	movq	%r15, 0x58(%rax,%r14)
1409261d1:     	leaq	0xc3eb58(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1409261d8:     	movq	%rcx, 0x68(%rax,%r14)
1409261dd:     	addq	$0x78, %r14
1409261e1:     	movq	%r14, 0x10(%rbx)
1409261e5:     	movaps	0x1f0(%rbp), %xmm6
1409261ec:     	movaps	0x200(%rbp), %xmm7
1409261f3:     	addq	$0x298, %rsp            # imm = 0x298
1409261fa:     	popq	%rbx
1409261fb:     	popq	%rdi
1409261fc:     	popq	%rsi
1409261fd:     	popq	%r12
1409261ff:     	popq	%r13
140926201:     	popq	%r14
140926203:     	popq	%r15
140926205:     	popq	%rbp
140926206:     	retq
140926207:     	movq	$0x1, 0x20(%rsp)
140926210:     	movl	$0x88, %r8d
140926216:     	movl	$0x1, %r9d
14092621c:     	movq	%rsi, %rcx
14092621f:     	movq	%rbx, %rdx
140926222:     	callq	0x1428a9a60 <.text+0x28a8a60>
140926227:     	jmp	0x140925582 <.text+0x924582>
14092622c:     	movq	$0x1, 0x20(%rsp)
140926235:     	movl	$0x38, %r8d
14092623b:     	movl	$0x1, %r9d
140926241:     	movq	%rsi, %rcx
140926244:     	movq	%rbx, %rdx
140926247:     	callq	0x1428a9a60 <.text+0x28a8a60>
14092624c:     	jmp	0x140925638 <.text+0x924638>
140926251:     	movq	$0x1, 0x20(%rsp)
14092625a:     	movl	$0x38, %r8d
140926260:     	movl	$0x1, %r9d
140926266:     	movq	%rsi, %rcx
140926269:     	movq	%rbx, %rdx
14092626c:     	callq	0x1428a9a60 <.text+0x28a8a60>
140926271:     	jmp	0x1409256b3 <.text+0x9246b3>
140926276:     	movq	$0x1, 0x20(%rsp)
14092627f:     	movl	$0x38, %r8d
140926285:     	movl	$0x1, %r9d
14092628b:     	movq	%rsi, %rcx
14092628e:     	movq	%rbx, %rdx
140926291:     	callq	0x1428a9a60 <.text+0x28a8a60>
140926296:     	jmp	0x140925732 <.text+0x924732>
14092629b:     	movq	$0x1, 0x20(%rsp)
1409262a4:     	movl	$0x38, %r8d
1409262aa:     	movl	$0x1, %r9d
1409262b0:     	movq	%rsi, %rcx
1409262b3:     	movq	%rbx, %rdx
1409262b6:     	callq	0x1428a9a60 <.text+0x28a8a60>
1409262bb:     	jmp	0x1409257ad <.text+0x9247ad>
1409262c0:     	movq	$0x1, 0x20(%rsp)
1409262c9:     	movl	$0x38, %r8d
1409262cf:     	movl	$0x1, %r9d
1409262d5:     	movq	%rsi, %rcx
1409262d8:     	movq	%rbx, %rdx
1409262db:     	callq	0x1428a9a60 <.text+0x28a8a60>
1409262e0:     	jmp	0x14092582c <.text+0x92482c>
1409262e5:     	movq	$0x1, 0x20(%rsp)
1409262ee:     	movl	$0x38, %r8d
1409262f4:     	movl	$0x1, %r9d
1409262fa:     	movq	%rsi, %rcx
1409262fd:     	movq	%rbx, %rdx
140926300:     	callq	0x1428a9a60 <.text+0x28a8a60>
140926305:     	jmp	0x1409258a7 <.text+0x9248a7>
14092630a:     	movq	$0x1, 0x20(%rsp)
140926313:     	movl	$0x38, %r8d
140926319:     	movl	$0x1, %r9d
14092631f:     	movq	%rsi, %rcx
140926322:     	movq	%rbx, %rdx
140926325:     	callq	0x1428a9a60 <.text+0x28a8a60>
14092632a:     	jmp	0x140925922 <.text+0x924922>
14092632f:     	movq	$0x1, 0x20(%rsp)
140926338:     	movl	$0x38, %r8d
14092633e:     	movl	$0x1, %r9d
140926344:     	movq	%rsi, %rcx
140926347:     	movq	%rbx, %rdx
14092634a:     	callq	0x1428a9a60 <.text+0x28a8a60>
14092634f:     	jmp	0x14092599d <.text+0x92499d>
140926354:     	movq	$0x1, 0x20(%rsp)
14092635d:     	movl	$0x38, %r8d
140926363:     	movl	$0x1, %r9d
140926369:     	movq	%rsi, %rcx
14092636c:     	movq	%rbx, %rdx
14092636f:     	callq	0x1428a9a60 <.text+0x28a8a60>
140926374:     	jmp	0x140925a1f <.text+0x924a1f>
140926379:     	movq	$0x1, 0x20(%rsp)
140926382:     	movl	$0x40, %r8d
140926388:     	movl	$0x1, %r9d
14092638e:     	movq	%rsi, %rcx
140926391:     	movq	%rbx, %rdx
140926394:     	callq	0x1428a9a60 <.text+0x28a8a60>
140926399:     	jmp	0x140925a9c <.text+0x924a9c>
14092639e:     	movq	$0x1, 0x20(%rsp)
1409263a7:     	movl	$0x40, %r8d
1409263ad:     	movl	$0x1, %r9d
1409263b3:     	movq	%rsi, %rcx
1409263b6:     	movq	%rbx, %rdx
1409263b9:     	callq	0x1428a9a60 <.text+0x28a8a60>
1409263be:     	jmp	0x140925b29 <.text+0x924b29>
1409263c3:     	movq	$0x1, 0x20(%rsp)
1409263cc:     	movl	$0x40, %r8d
1409263d2:     	movl	$0x1, %r9d
1409263d8:     	movq	%rsi, %rcx
1409263db:     	movq	%rbx, %rdx
1409263de:     	callq	0x1428a9a60 <.text+0x28a8a60>
1409263e3:     	jmp	0x140925baf <.text+0x924baf>
1409263e8:     	movq	$0x1, 0x20(%rsp)
1409263f1:     	movl	$0x40, %r8d
1409263f7:     	movl	$0x1, %r9d
1409263fd:     	movq	%rsi, %rcx
140926400:     	movq	%rbx, %rdx
140926403:     	callq	0x1428a9a60 <.text+0x28a8a60>
140926408:     	jmp	0x140925c42 <.text+0x924c42>
14092640d:     	movq	$0x1, 0x20(%rsp)
140926416:     	movl	$0x38, %r8d
14092641c:     	movl	$0x1, %r9d
140926422:     	movq	%rbx, %rcx
140926425:     	movq	%r15, %rdx
140926428:     	callq	0x1428a9a60 <.text+0x28a8a60>
14092642d:     	jmp	0x140925e64 <.text+0x924e64>
140926432:     	movq	$0x1, 0x20(%rsp)
14092643b:     	movl	$0x78, %r8d
140926441:     	movl	$0x1, %r9d
140926447:     	movq	%r15, %rcx
14092644a:     	movq	%r12, %rdx
14092644d:     	callq	0x1428a9a60 <.text+0x28a8a60>
140926452:     	jmp	0x140925fe2 <.text+0x924fe2>
140926457:     	movq	$0x1, 0x20(%rsp)
140926460:     	movl	$0x78, %r8d
140926466:     	movl	$0x1, %r9d
14092646c:     	movq	%rbx, %rcx
14092646f:     	movq	%r14, %rdx
140926472:     	callq	0x1428a9a60 <.text+0x28a8a60>
140926477:     	jmp	0x140926185 <.text+0x925185>
14092647c:     	movq	0x178(%rbp), %rax
140926483:     	movq	%rax, (%rbp)
140926487:     	leaq	0x2188a82(%rip), %rax   # 0x142aaef10
14092648e:     	movq	%rax, 0x20(%rsp)
140926493:     	leaq	0x2188076(%rip), %rcx   # 0x142aae510
14092649a:     	leaq	0x218804f(%rip), %r9    # 0x142aae4f0
1409264a1:     	movq	%rbp, %r8
1409264a4:     	movl	$0x2b, %edx
1409264a9:     	callq	0x1428d9760 <.text+0x28d8760>
1409264ae:     	jmp	0x1409267b9 <.text+0x9257b9>
1409264b3:     	movq	0xc8(%rbp), %rax
1409264ba:     	movq	%rax, 0x170(%rbp)
1409264c1:     	leaq	0x21885b0(%rip), %rax   # 0x142aaea78
1409264c8:     	movq	%rax, 0x20(%rsp)
1409264cd:     	leaq	0x218803c(%rip), %rcx   # 0x142aae510
1409264d4:     	leaq	0x2188015(%rip), %r9    # 0x142aae4f0
1409264db:     	leaq	0x170(%rbp), %r8
1409264e2:     	movl	$0x2b, %edx
1409264e7:     	callq	0x1428d9760 <.text+0x28d8760>
1409264ec:     	jmp	0x1409267b9 <.text+0x9257b9>
1409264f1:     	movq	0xc8(%rbp), %rax
1409264f8:     	movq	%rax, 0x170(%rbp)
1409264ff:     	leaq	0x21885ba(%rip), %rax   # 0x142aaeac0
140926506:     	movq	%rax, 0x20(%rsp)
14092650b:     	leaq	0x2187ffe(%rip), %rcx   # 0x142aae510
140926512:     	leaq	0x2187fd7(%rip), %r9    # 0x142aae4f0
140926519:     	leaq	0x170(%rbp), %r8
140926520:     	movl	$0x2b, %edx
140926525:     	callq	0x1428d9760 <.text+0x28d8760>
14092652a:     	jmp	0x1409267b9 <.text+0x9257b9>
14092652f:     	movq	0xc8(%rbp), %rax
140926536:     	movq	%rax, 0x170(%rbp)
14092653d:     	leaq	0x21885cc(%rip), %rax   # 0x142aaeb10
140926544:     	movq	%rax, 0x20(%rsp)
140926549:     	leaq	0x2187fc0(%rip), %rcx   # 0x142aae510
140926550:     	leaq	0x2187f99(%rip), %r9    # 0x142aae4f0
140926557:     	leaq	0x170(%rbp), %r8
14092655e:     	movl	$0x2b, %edx
140926563:     	callq	0x1428d9760 <.text+0x28d8760>
140926568:     	jmp	0x1409267b9 <.text+0x9257b9>
14092656d:     	movq	0xc8(%rbp), %rax
140926574:     	movq	%rax, 0x170(%rbp)
14092657b:     	leaq	0x21885e6(%rip), %rax   # 0x142aaeb68
140926582:     	movq	%rax, 0x20(%rsp)
140926587:     	leaq	0x2187f82(%rip), %rcx   # 0x142aae510
14092658e:     	leaq	0x2187f5b(%rip), %r9    # 0x142aae4f0
140926595:     	leaq	0x170(%rbp), %r8
14092659c:     	movl	$0x2b, %edx
1409265a1:     	callq	0x1428d9760 <.text+0x28d8760>
1409265a6:     	jmp	0x1409267b9 <.text+0x9257b9>
1409265ab:     	movq	0xc8(%rbp), %rax
1409265b2:     	movq	%rax, 0x170(%rbp)
1409265b9:     	leaq	0x21885f8(%rip), %rax   # 0x142aaebb8
1409265c0:     	movq	%rax, 0x20(%rsp)
1409265c5:     	leaq	0x2187f44(%rip), %rcx   # 0x142aae510
1409265cc:     	leaq	0x2187f1d(%rip), %r9    # 0x142aae4f0
1409265d3:     	leaq	0x170(%rbp), %r8
1409265da:     	movl	$0x2b, %edx
1409265df:     	callq	0x1428d9760 <.text+0x28d8760>
1409265e4:     	jmp	0x1409267b9 <.text+0x9257b9>
1409265e9:     	movq	0xc8(%rbp), %rax
1409265f0:     	movq	%rax, 0x170(%rbp)
1409265f7:     	leaq	0x218860a(%rip), %rax   # 0x142aaec08
1409265fe:     	movq	%rax, 0x20(%rsp)
140926603:     	leaq	0x2187f06(%rip), %rcx   # 0x142aae510
14092660a:     	leaq	0x2187edf(%rip), %r9    # 0x142aae4f0
140926611:     	leaq	0x170(%rbp), %r8
140926618:     	movl	$0x2b, %edx
14092661d:     	callq	0x1428d9760 <.text+0x28d8760>
140926622:     	jmp	0x1409267b9 <.text+0x9257b9>
140926627:     	movq	0xc8(%rbp), %rax
14092662e:     	movq	%rax, 0x170(%rbp)
140926635:     	leaq	0x218861c(%rip), %rax   # 0x142aaec58
14092663c:     	movq	%rax, 0x20(%rsp)
140926641:     	leaq	0x2187ec8(%rip), %rcx   # 0x142aae510
140926648:     	leaq	0x2187ea1(%rip), %r9    # 0x142aae4f0
14092664f:     	leaq	0x170(%rbp), %r8
140926656:     	movl	$0x2b, %edx
14092665b:     	callq	0x1428d9760 <.text+0x28d8760>
140926660:     	jmp	0x1409267b9 <.text+0x9257b9>
140926665:     	movq	0xc8(%rbp), %rax
14092666c:     	movq	%rax, 0x170(%rbp)
140926673:     	leaq	0x218862e(%rip), %rax   # 0x142aaeca8
14092667a:     	movq	%rax, 0x20(%rsp)
14092667f:     	leaq	0x2187e8a(%rip), %rcx   # 0x142aae510
140926686:     	leaq	0x2187e63(%rip), %r9    # 0x142aae4f0
14092668d:     	leaq	0x170(%rbp), %r8
140926694:     	movl	$0x2b, %edx
140926699:     	callq	0x1428d9760 <.text+0x28d8760>
14092669e:     	jmp	0x1409267b9 <.text+0x9257b9>
1409266a3:     	leaq	0x21886fe(%rip), %rcx   # 0x142aaeda8
1409266aa:     	callq	0x1428d9310 <.text+0x28d8310>
1409266af:     	movq	0xd8(%rbp), %rax
1409266b6:     	movq	%rax, 0x180(%rbp)
1409266bd:     	movups	0xc8(%rbp), %xmm0
1409266c4:     	movaps	%xmm0, 0x170(%rbp)
1409266cb:     	leaq	0x21886ee(%rip), %rax   # 0x142aaedc0
1409266d2:     	movq	%rax, 0x20(%rsp)
1409266d7:     	leaq	0x2187e32(%rip), %rcx   # 0x142aae510
1409266de:     	leaq	0x2187e5b(%rip), %r9    # 0x142aae540
1409266e5:     	leaq	0x170(%rbp), %r8
1409266ec:     	movl	$0x2b, %edx
1409266f1:     	callq	0x1428d9760 <.text+0x28d8760>
1409266f6:     	jmp	0x1409267b9 <.text+0x9257b9>
1409266fb:     	movq	0xc8(%rbp), %rax
140926702:     	movq	%rax, 0x170(%rbp)
140926709:     	leaq	0x21886f0(%rip), %rax   # 0x142aaee00
140926710:     	movq	%rax, 0x20(%rsp)
140926715:     	leaq	0x2187df4(%rip), %rcx   # 0x142aae510
14092671c:     	leaq	0x2187dcd(%rip), %r9    # 0x142aae4f0
140926723:     	leaq	0x170(%rbp), %r8
14092672a:     	movl	$0x2b, %edx
14092672f:     	callq	0x1428d9760 <.text+0x28d8760>
140926734:     	jmp	0x1409267b9 <.text+0x9257b9>
140926739:     	movq	0xc8(%rbp), %rax
140926740:     	movq	%rax, 0x170(%rbp)
140926747:     	leaq	0x21886f2(%rip), %rax   # 0x142aaee40
14092674e:     	movq	%rax, 0x20(%rsp)
140926753:     	leaq	0x2187db6(%rip), %rcx   # 0x142aae510
14092675a:     	leaq	0x2187d8f(%rip), %r9    # 0x142aae4f0
140926761:     	leaq	0x170(%rbp), %r8
140926768:     	movl	$0x2b, %edx
14092676d:     	callq	0x1428d9760 <.text+0x28d8760>
140926772:     	jmp	0x1409267b9 <.text+0x9257b9>
140926774:     	leaq	0x21886e5(%rip), %rcx   # 0x142aaee60
14092677b:     	callq	0x1428d9310 <.text+0x28d8310>
140926780:     	movq	0xc8(%rbp), %rax
140926787:     	movq	%rax, 0x170(%rbp)
14092678e:     	leaq	0x2188703(%rip), %rax   # 0x142aaee98
140926795:     	movq	%rax, 0x20(%rsp)
14092679a:     	leaq	0x2187d6f(%rip), %rcx   # 0x142aae510
1409267a1:     	leaq	0x2187d48(%rip), %r9    # 0x142aae4f0
1409267a8:     	leaq	0x170(%rbp), %r8
1409267af:     	movl	$0x2b, %edx
1409267b4:     	callq	0x1428d9760 <.text+0x28d8760>
1409267b9:     	ud2
1409267bb:     	leaq	0x21886fe(%rip), %rcx   # 0x142aaeec0
1409267c2:     	callq	0x1428d9310 <.text+0x28d8310>
1409267c7:     	int3
1409267c8:     	nopl	(%rax,%rax)
1409267d0:     	movq	%rdx, 0x10(%rsp)
1409267d5:     	pushq	%rbp
1409267d6:     	pushq	%r15
1409267d8:     	pushq	%r14
1409267da:     	pushq	%r13
1409267dc:     	pushq	%r12
1409267de:     	pushq	%rsi
1409267df:     	pushq	%rdi
1409267e0:     	pushq	%rbx
1409267e1:     	subq	$0x58, %rsp
1409267e5:     	leaq	0x80(%rdx), %rbp
1409267ec:     	movaps	%xmm7, 0x30(%rsp)
1409267f1:     	movaps	%xmm6, 0x40(%rsp)
1409267f6:     	movq	0x170(%rbp), %rdx
1409267fd:     	testq	%rdx, %rdx
140926800:     	je	0x140926814 <.text+0x925814>
140926802:     	movq	0x178(%rbp), %rcx
140926809:     	movl	$0x1, %r8d
14092680f:     	callq	0x140613c20 <.text+0x612c20>
140926814:     	movaps	0x40(%rsp), %xmm6
140926819:     	movaps	0x30(%rsp), %xmm7
14092681e:     	addq	$0x58, %rsp
140926822:     	popq	%rbx
140926823:     	popq	%rdi
140926824:     	popq	%rsi
140926825:     	popq	%r12
140926827:     	popq	%r13
140926829:     	popq	%r14
14092682b:     	popq	%r15
14092682d:     	popq	%rbp
14092682e:     	retq
14092682f:     	nop
140926830:     	movq	%rdx, 0x10(%rsp)
140926835:     	pushq	%rbp
140926836:     	pushq	%r15
140926838:     	pushq	%r14
14092683a:     	pushq	%r13
14092683c:     	pushq	%r12
14092683e:     	pushq	%rsi
14092683f:     	pushq	%rdi
140926840:     	pushq	%rbx
140926841:     	subq	$0x58, %rsp
140926845:     	leaq	0x80(%rdx), %rbp
14092684c:     	movaps	%xmm7, 0x30(%rsp)
140926851:     	movaps	%xmm6, 0x40(%rsp)
140926856:     	leaq	0x170(%rbp), %rcx
14092685d:     	callq	0x140d4cc30 <.text+0xd4bc30>
140926862:     	movaps	0x40(%rsp), %xmm6
140926867:     	movaps	0x30(%rsp), %xmm7
14092686c:     	addq	$0x58, %rsp
140926870:     	popq	%rbx
140926871:     	popq	%rdi
140926872:     	popq	%rsi
140926873:     	popq	%r12
140926875:     	popq	%r13
140926877:     	popq	%r14
140926879:     	popq	%r15
14092687b:     	popq	%rbp
14092687c:     	retq
14092687d:     	nopl	(%rax)
140926880:     	movq	%rdx, 0x10(%rsp)
140926885:     	pushq	%rbp
140926886:     	pushq	%r15
140926888:     	pushq	%r14
14092688a:     	pushq	%r13
14092688c:     	pushq	%r12
14092688e:     	pushq	%rsi
14092688f:     	pushq	%rdi
140926890:     	pushq	%rbx
140926891:     	subq	$0x58, %rsp
140926895:     	leaq	0x80(%rdx), %rbp
14092689c:     	movaps	%xmm7, 0x30(%rsp)
1409268a1:     	movaps	%xmm6, 0x40(%rsp)
1409268a6:     	leaq	0x170(%rbp), %rcx
1409268ad:     	callq	0x140d4cc30 <.text+0xd4bc30>
1409268b2:     	movaps	0x40(%rsp), %xmm6
1409268b7:     	movaps	0x30(%rsp), %xmm7
1409268bc:     	addq	$0x58, %rsp
1409268c0:     	popq	%rbx
1409268c1:     	popq	%rdi
1409268c2:     	popq	%rsi
1409268c3:     	popq	%r12
1409268c5:     	popq	%r13
1409268c7:     	popq	%r14
1409268c9:     	popq	%r15
1409268cb:     	popq	%rbp
1409268cc:     	retq
1409268cd:     	nopl	(%rax)
1409268d0:     	movq	%rdx, 0x10(%rsp)
1409268d5:     	pushq	%rbp
1409268d6:     	pushq	%r15
1409268d8:     	pushq	%r14
1409268da:     	pushq	%r13
1409268dc:     	pushq	%r12
1409268de:     	pushq	%rsi
1409268df:     	pushq	%rdi
1409268e0:     	pushq	%rbx
1409268e1:     	subq	$0x58, %rsp
1409268e5:     	leaq	0x80(%rdx), %rbp
1409268ec:     	movaps	%xmm7, 0x30(%rsp)
1409268f1:     	movaps	%xmm6, 0x40(%rsp)
1409268f6:     	leaq	0x170(%rbp), %rcx
1409268fd:     	callq	0x140d4cc30 <.text+0xd4bc30>
140926902:     	movaps	0x40(%rsp), %xmm6
140926907:     	movaps	0x30(%rsp), %xmm7
14092690c:     	addq	$0x58, %rsp
140926910:     	popq	%rbx
140926911:     	popq	%rdi
140926912:     	popq	%rsi
140926913:     	popq	%r12
140926915:     	popq	%r13
140926917:     	popq	%r14
140926919:     	popq	%r15
14092691b:     	popq	%rbp
14092691c:     	retq
14092691d:     	nopl	(%rax)
140926920:     	movq	%rdx, 0x10(%rsp)
140926925:     	pushq	%rbp
140926926:     	pushq	%r15
140926928:     	pushq	%r14
14092692a:     	pushq	%r13
14092692c:     	pushq	%r12
14092692e:     	pushq	%rsi
14092692f:     	pushq	%rdi
140926930:     	pushq	%rbx
140926931:     	subq	$0x58, %rsp
140926935:     	leaq	0x80(%rdx), %rbp
14092693c:     	movaps	%xmm7, 0x30(%rsp)
140926941:     	movaps	%xmm6, 0x40(%rsp)
140926946:     	leaq	0x170(%rbp), %rcx
14092694d:     	callq	0x140d4cc30 <.text+0xd4bc30>
140926952:     	movaps	0x40(%rsp), %xmm6
140926957:     	movaps	0x30(%rsp), %xmm7
14092695c:     	addq	$0x58, %rsp
140926960:     	popq	%rbx
140926961:     	popq	%rdi
140926962:     	popq	%rsi
140926963:     	popq	%r12
140926965:     	popq	%r13
140926967:     	popq	%r14
140926969:     	popq	%r15
14092696b:     	popq	%rbp
14092696c:     	retq
14092696d:     	nopl	(%rax)
140926970:     	movq	%rdx, 0x10(%rsp)
140926975:     	pushq	%rbp
140926976:     	pushq	%r15
140926978:     	pushq	%r14
14092697a:     	pushq	%r13
14092697c:     	pushq	%r12
14092697e:     	pushq	%rsi
14092697f:     	pushq	%rdi
140926980:     	pushq	%rbx
140926981:     	subq	$0x58, %rsp
140926985:     	leaq	0x80(%rdx), %rbp
14092698c:     	movaps	%xmm7, 0x30(%rsp)
140926991:     	movaps	%xmm6, 0x40(%rsp)
140926996:     	leaq	0x170(%rbp), %rcx
14092699d:     	callq	0x140d4cc30 <.text+0xd4bc30>
1409269a2:     	movaps	0x40(%rsp), %xmm6
1409269a7:     	movaps	0x30(%rsp), %xmm7
1409269ac:     	addq	$0x58, %rsp
1409269b0:     	popq	%rbx
1409269b1:     	popq	%rdi
1409269b2:     	popq	%rsi
1409269b3:     	popq	%r12
1409269b5:     	popq	%r13
1409269b7:     	popq	%r14
1409269b9:     	popq	%r15
1409269bb:     	popq	%rbp
1409269bc:     	retq
1409269bd:     	nopl	(%rax)
1409269c0:     	movq	%rdx, 0x10(%rsp)
1409269c5:     	pushq	%rbp
1409269c6:     	pushq	%r15
1409269c8:     	pushq	%r14
1409269ca:     	pushq	%r13
1409269cc:     	pushq	%r12
1409269ce:     	pushq	%rsi
1409269cf:     	pushq	%rdi
1409269d0:     	pushq	%rbx
1409269d1:     	subq	$0x58, %rsp
1409269d5:     	leaq	0x80(%rdx), %rbp
1409269dc:     	movaps	%xmm7, 0x30(%rsp)
1409269e1:     	movaps	%xmm6, 0x40(%rsp)
1409269e6:     	leaq	0x170(%rbp), %rcx
1409269ed:     	callq	0x140d4cc30 <.text+0xd4bc30>
1409269f2:     	movaps	0x40(%rsp), %xmm6
1409269f7:     	movaps	0x30(%rsp), %xmm7
1409269fc:     	addq	$0x58, %rsp
140926a00:     	popq	%rbx
140926a01:     	popq	%rdi
140926a02:     	popq	%rsi
140926a03:     	popq	%r12
140926a05:     	popq	%r13
140926a07:     	popq	%r14
140926a09:     	popq	%r15
140926a0b:     	popq	%rbp
140926a0c:     	retq
140926a0d:     	nopl	(%rax)
140926a10:     	movq	%rdx, 0x10(%rsp)
140926a15:     	pushq	%rbp
140926a16:     	pushq	%r15
140926a18:     	pushq	%r14
140926a1a:     	pushq	%r13
140926a1c:     	pushq	%r12
140926a1e:     	pushq	%rsi
140926a1f:     	pushq	%rdi
140926a20:     	pushq	%rbx
140926a21:     	subq	$0x58, %rsp
140926a25:     	leaq	0x80(%rdx), %rbp
140926a2c:     	movaps	%xmm7, 0x30(%rsp)
140926a31:     	movaps	%xmm6, 0x40(%rsp)
140926a36:     	leaq	0x170(%rbp), %rcx
140926a3d:     	callq	0x140d4cc30 <.text+0xd4bc30>
140926a42:     	movaps	0x40(%rsp), %xmm6
140926a47:     	movaps	0x30(%rsp), %xmm7
140926a4c:     	addq	$0x58, %rsp
140926a50:     	popq	%rbx
140926a51:     	popq	%rdi
140926a52:     	popq	%rsi
140926a53:     	popq	%r12
140926a55:     	popq	%r13
140926a57:     	popq	%r14
140926a59:     	popq	%r15
140926a5b:     	popq	%rbp
140926a5c:     	retq
140926a5d:     	nopl	(%rax)
140926a60:     	movq	%rdx, 0x10(%rsp)
140926a65:     	pushq	%rbp
140926a66:     	pushq	%r15
140926a68:     	pushq	%r14
140926a6a:     	pushq	%r13
140926a6c:     	pushq	%r12
140926a6e:     	pushq	%rsi
140926a6f:     	pushq	%rdi
140926a70:     	pushq	%rbx
140926a71:     	subq	$0x58, %rsp
140926a75:     	leaq	0x80(%rdx), %rbp
140926a7c:     	movaps	%xmm7, 0x30(%rsp)
140926a81:     	movaps	%xmm6, 0x40(%rsp)
140926a86:     	leaq	0x170(%rbp), %rcx
140926a8d:     	callq	0x140d4cc30 <.text+0xd4bc30>
140926a92:     	movaps	0x40(%rsp), %xmm6
140926a97:     	movaps	0x30(%rsp), %xmm7
140926a9c:     	addq	$0x58, %rsp
140926aa0:     	popq	%rbx
140926aa1:     	popq	%rdi
140926aa2:     	popq	%rsi
140926aa3:     	popq	%r12
140926aa5:     	popq	%r13
140926aa7:     	popq	%r14
140926aa9:     	popq	%r15
140926aab:     	popq	%rbp
140926aac:     	retq
140926aad:     	nopl	(%rax)
140926ab0:     	movq	%rdx, 0x10(%rsp)
140926ab5:     	pushq	%rbp
140926ab6:     	pushq	%r15
140926ab8:     	pushq	%r14
140926aba:     	pushq	%r13
140926abc:     	pushq	%r12
140926abe:     	pushq	%rsi
140926abf:     	pushq	%rdi
140926ac0:     	pushq	%rbx
140926ac1:     	subq	$0x58, %rsp
140926ac5:     	leaq	0x80(%rdx), %rbp
140926acc:     	movaps	%xmm7, 0x30(%rsp)
140926ad1:     	movaps	%xmm6, 0x40(%rsp)
140926ad6:     	leaq	0x170(%rbp), %rcx
140926add:     	callq	0x140d4cc30 <.text+0xd4bc30>
140926ae2:     	movaps	0x40(%rsp), %xmm6
140926ae7:     	movaps	0x30(%rsp), %xmm7
140926aec:     	addq	$0x58, %rsp
140926af0:     	popq	%rbx
140926af1:     	popq	%rdi
140926af2:     	popq	%rsi
140926af3:     	popq	%r12
140926af5:     	popq	%r13
140926af7:     	popq	%r14
140926af9:     	popq	%r15
140926afb:     	popq	%rbp
140926afc:     	retq
140926afd:     	nopl	(%rax)
140926b00:     	movq	%rdx, 0x10(%rsp)
140926b05:     	pushq	%rbp
140926b06:     	pushq	%r15
140926b08:     	pushq	%r14
140926b0a:     	pushq	%r13
140926b0c:     	pushq	%r12
140926b0e:     	pushq	%rsi
140926b0f:     	pushq	%rdi
140926b10:     	pushq	%rbx
140926b11:     	subq	$0x58, %rsp
140926b15:     	leaq	0x80(%rdx), %rbp
140926b1c:     	movaps	%xmm7, 0x30(%rsp)
140926b21:     	movaps	%xmm6, 0x40(%rsp)
140926b26:     	leaq	0x170(%rbp), %rcx
140926b2d:     	callq	0x140d4cc30 <.text+0xd4bc30>
140926b32:     	movaps	0x40(%rsp), %xmm6
140926b37:     	movaps	0x30(%rsp), %xmm7
140926b3c:     	addq	$0x58, %rsp
140926b40:     	popq	%rbx
140926b41:     	popq	%rdi
140926b42:     	popq	%rsi
140926b43:     	popq	%r12
140926b45:     	popq	%r13
140926b47:     	popq	%r14
140926b49:     	popq	%r15
140926b4b:     	popq	%rbp
140926b4c:     	retq
140926b4d:     	nopl	(%rax)
140926b50:     	movq	%rdx, 0x10(%rsp)
140926b55:     	pushq	%rbp
140926b56:     	pushq	%r15
140926b58:     	pushq	%r14
140926b5a:     	pushq	%r13
140926b5c:     	pushq	%r12
140926b5e:     	pushq	%rsi
140926b5f:     	pushq	%rdi
140926b60:     	pushq	%rbx
140926b61:     	subq	$0x58, %rsp
140926b65:     	leaq	0x80(%rdx), %rbp
140926b6c:     	movaps	%xmm7, 0x30(%rsp)
140926b71:     	movaps	%xmm6, 0x40(%rsp)
140926b76:     	leaq	0x170(%rbp), %rcx
140926b7d:     	callq	0x140d4cc30 <.text+0xd4bc30>
140926b82:     	movaps	0x40(%rsp), %xmm6
140926b87:     	movaps	0x30(%rsp), %xmm7
140926b8c:     	addq	$0x58, %rsp
140926b90:     	popq	%rbx
140926b91:     	popq	%rdi
140926b92:     	popq	%rsi
140926b93:     	popq	%r12
140926b95:     	popq	%r13
140926b97:     	popq	%r14
140926b99:     	popq	%r15
140926b9b:     	popq	%rbp
140926b9c:     	retq
140926b9d:     	nopl	(%rax)
140926ba0:     	movq	%rdx, 0x10(%rsp)
140926ba5:     	pushq	%rbp
140926ba6:     	pushq	%r15
140926ba8:     	pushq	%r14
140926baa:     	pushq	%r13
140926bac:     	pushq	%r12
140926bae:     	pushq	%rsi
140926baf:     	pushq	%rdi
140926bb0:     	pushq	%rbx
140926bb1:     	subq	$0x58, %rsp
140926bb5:     	leaq	0x80(%rdx), %rbp
140926bbc:     	movaps	%xmm7, 0x30(%rsp)
140926bc1:     	movaps	%xmm6, 0x40(%rsp)
140926bc6:     	movq	%rbp, %rcx
140926bc9:     	callq	0x140d4cc30 <.text+0xd4bc30>
140926bce:     	movaps	0x40(%rsp), %xmm6
140926bd3:     	movaps	0x30(%rsp), %xmm7
140926bd8:     	addq	$0x58, %rsp
140926bdc:     	popq	%rbx
140926bdd:     	popq	%rdi
140926bde:     	popq	%rsi
140926bdf:     	popq	%r12
140926be1:     	popq	%r13
140926be3:     	popq	%r14
140926be5:     	popq	%r15
140926be7:     	popq	%rbp
140926be8:     	retq

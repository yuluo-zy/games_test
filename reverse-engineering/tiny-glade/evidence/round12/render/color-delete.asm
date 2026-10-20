
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142297ff0:     	pushq	%r15
142297ff2:     	pushq	%r14
142297ff4:     	pushq	%r13
142297ff6:     	pushq	%r12
142297ff8:     	pushq	%rsi
142297ff9:     	pushq	%rdi
142297ffa:     	pushq	%rbp
142297ffb:     	pushq	%rbx
142297ffc:     	subq	$0x88, %rsp
142298003:     	movaps	%xmm6, 0x70(%rsp)
142298008:     	movq	(%rcx), %rbx
14229800b:     	movq	0x8(%rcx), %r14
14229800f:     	movq	0xf8(%rbx), %r15
142298016:     	movq	0x100(%rbx), %rax
14229801d:     	leaq	(%r15,%rax,4), %r12
142298021:     	movq	(%rdx), %rax
142298024:     	movq	%rax, 0x40(%rsp)
142298029:     	movq	0x8(%rdx), %rax
14229802d:     	movq	%rax, 0x38(%rsp)
142298032:     	movq	0x18(%rdx), %rax
142298036:     	movq	%rax, 0x30(%rsp)
14229803b:     	movq	0x10(%rdx), %rax
14229803f:     	movq	%rax, 0x28(%rsp)
142298044:     	cmpb	$0x0, 0x108(%rbx)
14229804b:     	je	0x142298139 <.text+0x2297139>
142298051:     	movl	$0x8, %esi
142298056:     	xorl	%ebp, %ebp
142298058:     	xorps	%xmm6, %xmm6
14229805b:     	xorl	%r13d, %r13d
14229805e:     	xorl	%edi, %edi
142298060:     	cmpq	%r13, %rdi
142298063:     	jne	0x1422980d0 <.text+0x22970d0>
142298065:     	nopw	%cs:(%rax,%rax)
142298070:     	cmpq	%r12, %r15
142298073:     	je	0x142298241 <.text+0x2297241>
142298079:     	movl	(%r15), %ecx
14229807c:     	addq	$0x4, %r15
142298080:     	movq	0x1a8(%r14), %rax
142298087:     	leaq	(%rcx,%rcx,8), %rcx
14229808b:     	movq	0x10(%rax,%rcx,8), %r13
142298090:     	testq	%r13, %r13
142298093:     	je	0x142298070 <.text+0x2297070>
142298095:     	leaq	(%rax,%rcx,8), %rax
142298099:     	movq	0x110(%rbx), %rcx
1422980a0:     	cmpq	%rcx, 0x40(%rax)
1422980a4:     	jbe	0x1422980d5 <.text+0x22970d5>
1422980a6:     	movq	0x38(%rax), %rdx
1422980aa:     	movq	(%rdx,%rcx,8), %rcx
1422980ae:     	testq	%rcx, %rcx
1422980b1:     	je	0x1422980d5 <.text+0x22970d5>
1422980b3:     	movq	0x18(%rax), %rdx
1422980b7:     	notq	%rcx
1422980ba:     	leaq	(%rcx,%rcx,2), %rcx
1422980be:     	shlq	$0x4, %rcx
1422980c2:     	movq	0x10(%rdx,%rcx), %rbp
1422980c7:     	jmp	0x1422980d7 <.text+0x22970d7>
1422980c9:     	nopl	(%rax)
1422980d0:     	movq	%rdi, %rax
1422980d3:     	jmp	0x1422980dd <.text+0x22970dd>
1422980d5:     	xorl	%ebp, %ebp
1422980d7:     	movq	0x8(%rax), %rsi
1422980db:     	xorl	%eax, %eax
1422980dd:     	movl	%eax, %ecx
1422980df:     	leaq	0x1(%rax), %rdi
1422980e3:     	leaq	(%rcx,%rcx,8), %rcx
1422980e7:     	ucomiss	0x18(%rbp,%rcx,4), %xmm6
1422980ec:     	jb	0x142298060 <.text+0x2297060>
1422980f2:     	movq	(%rsi,%rax,8), %rax
1422980f6:     	movq	%rax, 0x68(%rsp)
1422980fb:     	movq	0x40(%rsp), %rax
142298100:     	movq	%rax, 0x48(%rsp)
142298105:     	movq	0x38(%rsp), %rax
14229810a:     	movq	%rax, 0x50(%rsp)
14229810f:     	movq	0x28(%rsp), %rax
142298114:     	movq	%rax, 0x58(%rsp)
142298119:     	movq	0x30(%rsp), %rax
14229811e:     	movq	%rax, 0x60(%rsp)
142298123:     	leaq	0x48(%rsp), %rcx
142298128:     	leaq	0xda38d1(%rip), %rdx    # 0x14303ba00
14229812f:     	callq	0x140d14770 <.text+0xd13770>
142298134:     	jmp	0x142298060 <.text+0x2297060>
142298139:     	movl	$0x8, %esi
14229813e:     	xorl	%ebp, %ebp
142298140:     	xorps	%xmm6, %xmm6
142298143:     	xorl	%r13d, %r13d
142298146:     	xorl	%edi, %edi
142298148:     	cmpq	%r13, %rdi
14229814b:     	jne	0x1422981db <.text+0x22971db>
142298151:     	nopw	%cs:(%rax,%rax)
142298160:     	cmpq	%r12, %r15
142298163:     	je	0x142298241 <.text+0x2297241>
142298169:     	movl	(%r15), %ecx
14229816c:     	addq	$0x4, %r15
142298170:     	movq	0x100(%r14), %rax
142298177:     	leaq	(%rcx,%rcx,4), %rcx
14229817b:     	shlq	$0x5, %rcx
14229817f:     	movq	0x58(%rax,%rcx), %r13
142298184:     	testq	%r13, %r13
142298187:     	je	0x142298160 <.text+0x2297160>
142298189:     	addq	%rcx, %rax
14229818c:     	movl	0x94(%rax), %edx
142298192:     	movq	0x1a8(%r14), %rcx
142298199:     	leaq	(%rdx,%rdx,8), %rdx
14229819d:     	movq	0x110(%rbx), %r8
1422981a4:     	cmpq	%r8, 0x40(%rcx,%rdx,8)
1422981a9:     	jbe	0x1422981d3 <.text+0x22971d3>
1422981ab:     	movq	0x38(%rcx,%rdx,8), %r9
1422981b0:     	movq	(%r9,%r8,8), %r8
1422981b4:     	testq	%r8, %r8
1422981b7:     	je	0x1422981d3 <.text+0x22971d3>
1422981b9:     	leaq	(%rcx,%rdx,8), %rcx
1422981bd:     	movq	0x18(%rcx), %rcx
1422981c1:     	notq	%r8
1422981c4:     	leaq	(%r8,%r8,2), %rdx
1422981c8:     	shlq	$0x4, %rdx
1422981cc:     	movq	0x10(%rcx,%rdx), %rbp
1422981d1:     	jmp	0x1422981d5 <.text+0x22971d5>
1422981d3:     	xorl	%ebp, %ebp
1422981d5:     	movq	0x50(%rax), %rsi
1422981d9:     	xorl	%edi, %edi
1422981db:     	movq	%rdi, %rax
1422981de:     	shlq	$0x4, %rax
1422981e2:     	movl	0x8(%rsi,%rax), %ecx
1422981e6:     	incq	%rdi
1422981e9:     	leaq	(%rcx,%rcx,8), %rcx
1422981ed:     	ucomiss	0x18(%rbp,%rcx,4), %xmm6
1422981f2:     	jb	0x142298148 <.text+0x2297148>
1422981f8:     	addq	%rsi, %rax
1422981fb:     	movq	(%rax), %rax
1422981fe:     	movq	%rax, 0x68(%rsp)
142298203:     	movq	0x40(%rsp), %rax
142298208:     	movq	%rax, 0x48(%rsp)
14229820d:     	movq	0x38(%rsp), %rax
142298212:     	movq	%rax, 0x50(%rsp)
142298217:     	movq	0x28(%rsp), %rax
14229821c:     	movq	%rax, 0x58(%rsp)
142298221:     	movq	0x30(%rsp), %rax
142298226:     	movq	%rax, 0x60(%rsp)
14229822b:     	leaq	0x48(%rsp), %rcx
142298230:     	leaq	0xda37c9(%rip), %rdx    # 0x14303ba00
142298237:     	callq	0x140d14770 <.text+0xd13770>
14229823c:     	jmp	0x142298148 <.text+0x2297148>
142298241:     	movaps	0x70(%rsp), %xmm6
142298246:     	addq	$0x88, %rsp
14229824d:     	popq	%rbx
14229824e:     	popq	%rbp
14229824f:     	popq	%rdi
142298250:     	popq	%rsi
142298251:     	popq	%r12
142298253:     	popq	%r13
142298255:     	popq	%r14
142298257:     	popq	%r15
142298259:     	retq

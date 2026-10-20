
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142623cb0:     	pushq	%r15
142623cb2:     	pushq	%r14
142623cb4:     	pushq	%r13
142623cb6:     	pushq	%r12
142623cb8:     	pushq	%rsi
142623cb9:     	pushq	%rdi
142623cba:     	pushq	%rbp
142623cbb:     	pushq	%rbx
142623cbc:     	subq	$0x1f8, %rsp            # imm = 0x1F8
142623cc3:     	movaps	%xmm6, 0x1e0(%rsp)
142623ccb:     	movq	%r9, %rdi
142623cce:     	movl	%r8d, %ebx
142623cd1:     	movq	%rdx, %r14
142623cd4:     	movq	%rcx, %rsi
142623cd7:     	movw	$0x0, 0x1c8(%rsp)
142623ce1:     	movups	0x59ea50(%rip), %xmm0   # 0x142bc2738
142623ce8:     	movups	%xmm0, 0x138(%rsp)
142623cf0:     	movups	0x59ea51(%rip), %xmm0   # 0x142bc2748
142623cf7:     	movups	%xmm0, 0x148(%rsp)
142623cff:     	movups	0x59ea52(%rip), %xmm0   # 0x142bc2758
142623d06:     	movups	%xmm0, 0x158(%rsp)
142623d0e:     	movups	0x59ea53(%rip), %xmm0   # 0x142bc2768
142623d15:     	movups	%xmm0, 0x168(%rsp)
142623d1d:     	movups	0x59ea54(%rip), %xmm0   # 0x142bc2778
142623d24:     	movups	%xmm0, 0x178(%rsp)
142623d2c:     	movq	$0x0, 0xe0(%rsp)
142623d38:     	movups	0x59ea49(%rip), %xmm0   # 0x142bc2788
142623d3f:     	movups	%xmm0, 0x188(%rsp)
142623d47:     	movups	0x59ea4a(%rip), %xmm0   # 0x142bc2798
142623d4e:     	movups	%xmm0, 0x198(%rsp)
142623d56:     	movups	0x59e99b(%rip), %xmm0   # 0x142bc26f8
142623d5d:     	movups	%xmm0, 0xf8(%rsp)
142623d65:     	movups	0x59e97c(%rip), %xmm0   # 0x142bc26e8
142623d6c:     	movups	%xmm0, 0xe8(%rsp)
142623d74:     	movups	0x59e99d(%rip), %xmm0   # 0x142bc2718
142623d7b:     	movups	%xmm0, 0x118(%rsp)
142623d83:     	movups	0x59e97e(%rip), %xmm0   # 0x142bc2708
142623d8a:     	movups	%xmm0, 0x108(%rsp)
142623d92:     	movl	$0x7070704, 0x1cd(%rsp) # imm = 0x7070704
142623d9d:     	movw	$0x409, 0x1d1(%rsp)     # imm = 0x409
142623da7:     	movups	0x59e97a(%rip), %xmm0   # 0x142bc2728
142623dae:     	movups	%xmm0, 0x128(%rsp)
142623db6:     	movw	$0x0, 0x1ca(%rsp)
142623dc0:     	movq	$0x2, 0x1a8(%rsp)
142623dcc:     	movsd	0x3107dc(%rip), %xmm0   # 0x1429345b0
142623dd4:     	movsd	%xmm0, 0x1c0(%rsp)
142623ddd:     	movups	0x59e89c(%rip), %xmm0   # 0x142bc2680
142623de4:     	movaps	%xmm0, 0x80(%rsp)
142623dec:     	movq	0x59e89d(%rip), %rax    # 0x142bc2690
142623df3:     	movq	%rax, 0x90(%rsp)
142623dfb:     	movq	0x59e8a6(%rip), %rax    # 0x142bc26a8
142623e02:     	movq	%rax, 0xa8(%rsp)
142623e0a:     	movups	0x59e887(%rip), %xmm0   # 0x142bc2698
142623e11:     	movups	%xmm0, 0x98(%rsp)
142623e19:     	movq	0x59e8a0(%rip), %rax    # 0x142bc26c0
142623e20:     	movq	%rax, 0xc0(%rsp)
142623e28:     	movups	0x59e881(%rip), %xmm0   # 0x142bc26b0
142623e2f:     	movaps	%xmm0, 0xb0(%rsp)
142623e37:     	movq	0x59e89a(%rip), %rax    # 0x142bc26d8
142623e3e:     	movq	%rax, 0xd8(%rsp)
142623e46:     	movups	0x59e87b(%rip), %xmm0   # 0x142bc26c8
142623e4d:     	movups	%xmm0, 0xc8(%rsp)
142623e55:     	movb	$0x0, 0x1cc(%rsp)
142623e5d:     	xorps	%xmm0, %xmm0
142623e60:     	movaps	%xmm0, 0x1b0(%rsp)
142623e68:     	leaq	0x60(%rsp), %rcx
142623e6d:     	leaq	0x80(%rsp), %r8
142623e75:     	callq	0x140fe7600 <.text+0xfe6600>
142623e7a:     	cmpl	$0x4, 0x60(%rsp)
142623e7f:     	jne	0x14262463e <.text+0x262363e>
142623e85:     	movl	0x64(%rsp), %ebp
142623e89:     	movl	0x68(%rsp), %eax
142623e8d:     	movl	0x20(%rsi), %r8d
142623e91:     	movl	0x24(%rsi), %r9d
142623e95:     	movl	%eax, 0x5c(%rsp)
142623e99:     	movl	%eax, 0x28(%rsp)
142623e9d:     	movl	%ebp, 0x20(%rsp)
142623ea1:     	leaq	0x30(%rsp), %rcx
142623ea6:     	movq	%r14, %rdx
142623ea9:     	callq	0x140fe7790 <.text+0xfe6790>
142623eae:     	cmpl	$0x4, 0x30(%rsp)
142623eb3:     	jne	0x14262467b <.text+0x262367b>
142623eb9:     	movl	0x278(%rsp), %eax
142623ec0:     	movq	0x260(%rsp), %r14
142623ec8:     	movss	0xa2a42c(%rip), %xmm6   # 0x14304e2fc
142623ed0:     	testb	$0x1, %al
142623ed2:     	je	0x142623edd <.text+0x2622edd>
142623ed4:     	mulss	0x280(%rsp), %xmm6
142623edd:     	xorps	%xmm0, %xmm0
142623ee0:     	movups	%xmm0, 0xb0(%rsp)
142623ee8:     	testq	%r14, %r14
142623eeb:     	js	0x1426246c1 <.text+0x26236c1>
142623ef1:     	je	0x142623f1b <.text+0x2622f1b>
142623ef3:     	movzbl	0x1010d0f(%rip), %eax   # 0x143634c09
142623efa:     	movl	$0x1, %r12d
142623f00:     	movl	$0x1, %edx
142623f05:     	movq	%r14, %rcx
142623f08:     	callq	0x140613c10 <.text+0x612c10>
142623f0d:     	testq	%rax, %rax
142623f10:     	je	0x1426246c4 <.text+0x26236c4>
142623f16:     	movq	%rax, %r15
142623f19:     	jmp	0x142623f21 <.text+0x2622f21>
142623f1b:     	movl	$0x1, %r15d
142623f21:     	movq	0x270(%rsp), %r13
142623f29:     	movq	0x268(%rsp), %r12
142623f31:     	movq	%r15, %rcx
142623f34:     	movq	%rdi, %rdx
142623f37:     	movq	%r14, %r8
142623f3a:     	callq	0x142923c00 <.text+0x2922c00>
142623f3f:     	movq	%r14, 0x80(%rsp)
142623f47:     	movq	%r15, 0x88(%rsp)
142623f4f:     	movq	%r14, 0x90(%rsp)
142623f57:     	movl	$0x0, 0x98(%rsp)
142623f62:     	movl	%ebp, 0xc0(%rsp)
142623f69:     	movl	0x5c(%rsp), %eax
142623f6d:     	movl	%eax, 0xc4(%rsp)
142623f74:     	movss	%xmm6, 0xc8(%rsp)
142623f7d:     	movss	%xmm6, 0xcc(%rsp)
142623f86:     	movw	$0x1, 0xd8(%rsp)
142623f90:     	movb	$0x0, 0xda(%rsp)
142623f98:     	movss	0x301964(%rip), %xmm0   # 0x142925904
142623fa0:     	movlps	%xmm0, 0xa0(%rsp)
142623fa8:     	movl	$0x0, 0xa8(%rsp)
142623fb3:     	movb	$0x2, 0xac(%rsp)
142623fbb:     	movsd	0x96b64d(%rip), %xmm0   # 0x142f8f610
142623fc3:     	movsd	%xmm0, 0xd0(%rsp)
142623fcc:     	movq	$0x0, 0xe0(%rsp)
142623fd8:     	leaq	0x96b6e1(%rip), %r9     # 0x142f8f6c0
142623fdf:     	leaq	0x30(%rsp), %rcx
142623fe4:     	leaq	0x80(%rsp), %r8
142623fec:     	movq	%r12, %rdx
142623fef:     	callq	0x140fe0470 <.text+0xfdf470>
142623ff4:     	movzbl	(%r13), %eax
142623ff9:     	addl	$-0xb, %eax
142623ffc:     	cmpl	$0xa, %eax
142623fff:     	ja	0x14262404f <.text+0x262304f>
142624001:     	leaq	0xb25848(%rip), %rcx    # 0x143149850
142624008:     	movslq	(%rcx,%rax,4), %rax
14262400c:     	addq	%rcx, %rax
14262400f:     	jmpq	*%rax
142624011:     	movq	0x50(%rsp), %r12
142624016:     	movq	0x38(%rsp), %r13
14262401b:     	movq	0x40(%rsp), %rdi
142624020:     	testq	%r13, %r13
142624023:     	movq	%r13, %r14
142624026:     	cmoveq	%rdi, %r14
14262402a:     	movq	(%r14), %rax
14262402d:     	movq	0x10(%r14), %r15
142624031:     	subq	%r15, %rax
142624034:     	cmpq	$0x1f, %rax
142624038:     	jbe	0x142624560 <.text+0x2623560>
14262403e:     	movq	0x8(%r14), %rax
142624042:     	addq	$0x10, %r14
142624046:     	leaq	-0x16453ad(%rip), %rcx  # 0x140fdeca0 <.text+0xfddca0>
14262404d:     	jmp	0x14262408b <.text+0x262308b>
14262404f:     	movq	0x50(%rsp), %r12
142624054:     	movq	0x38(%rsp), %r13
142624059:     	movq	0x40(%rsp), %rdi
14262405e:     	testq	%r13, %r13
142624061:     	movq	%r13, %r14
142624064:     	cmoveq	%rdi, %r14
142624068:     	movq	(%r14), %rax
14262406b:     	movq	0x10(%r14), %r15
14262406f:     	subq	%r15, %rax
142624072:     	cmpq	$0x1f, %rax
142624076:     	jbe	0x142624516 <.text+0x2623516>
14262407c:     	movq	0x8(%r14), %rax
142624080:     	addq	$0x10, %r14
142624084:     	leaq	-0x1644b8b(%rip), %rcx  # 0x140fdf500 <.text+0xfde500>
14262408b:     	movq	%rcx, (%rax,%r15)
14262408f:     	leaq	-0x7d4836(%rip), %rcx   # 0x141e4f860 <.text+0x1e4e860>
142624096:     	movq	%rcx, 0x8(%rax,%r15)
14262409b:     	movq	%r12, 0x10(%rax,%r15)
1426240a0:     	movb	$0x0, 0x18(%rax,%r15)
1426240a6:     	jmp	0x1426242e7 <.text+0x26232e7>
1426240ab:     	movq	0x1007fe6(%rip), %rax   # 0x14362c098
1426240b2:     	movq	0x50(%rsp), %r12
1426240b7:     	movq	0x38(%rsp), %r13
1426240bc:     	movq	0x40(%rsp), %rdi
1426240c1:     	cmpb	$0x0, (%rax)
1426240c4:     	jne	0x1426242ee <.text+0x26232ee>
1426240ca:     	testq	%r13, %r13
1426240cd:     	movq	%r13, %r14
1426240d0:     	cmoveq	%rdi, %r14
1426240d4:     	movq	(%r14), %rax
1426240d7:     	movq	0x10(%r14), %r15
1426240db:     	subq	%r15, %rax
1426240de:     	cmpq	$0x1f, %rax
1426240e2:     	jbe	0x142624619 <.text+0x2623619>
1426240e8:     	movq	0x8(%r14), %rax
1426240ec:     	addq	$0x10, %r14
1426240f0:     	leaq	-0x1645b37(%rip), %rcx  # 0x140fde5c0 <.text+0xfdd5c0>
1426240f7:     	movq	%rcx, (%rax,%r15)
1426240fb:     	leaq	-0x7d48a2(%rip), %rcx   # 0x141e4f860 <.text+0x1e4e860>
142624102:     	movq	%rcx, 0x8(%rax,%r15)
142624107:     	movq	%r12, 0x10(%rax,%r15)
14262410c:     	movw	$0xb00, 0x18(%rax,%r15) # imm = 0xB00
142624114:     	jmp	0x1426242e7 <.text+0x26232e7>
142624119:     	movq	0x50(%rsp), %r12
14262411e:     	movq	0x38(%rsp), %r13
142624123:     	movq	0x40(%rsp), %rdi
142624128:     	testq	%r13, %r13
14262412b:     	movq	%r13, %r14
14262412e:     	cmoveq	%rdi, %r14
142624132:     	movq	(%r14), %rax
142624135:     	movq	0x10(%r14), %r15
142624139:     	subq	%r15, %rax
14262413c:     	cmpq	$0x1f, %rax
142624140:     	jbe	0x14262453b <.text+0x262353b>
142624146:     	movq	0x8(%r14), %rax
14262414a:     	addq	$0x10, %r14
14262414e:     	leaq	-0x1645b95(%rip), %rcx  # 0x140fde5c0 <.text+0xfdd5c0>
142624155:     	movq	%rcx, (%rax,%r15)
142624159:     	leaq	-0x7d4900(%rip), %rcx   # 0x141e4f860 <.text+0x1e4e860>
142624160:     	movq	%rcx, 0x8(%rax,%r15)
142624165:     	movq	%r12, 0x10(%rax,%r15)
14262416a:     	movw	$0x400, 0x18(%rax,%r15) # imm = 0x400
142624172:     	jmp	0x1426242e7 <.text+0x26232e7>
142624177:     	movq	0x50(%rsp), %r12
14262417c:     	movq	0x38(%rsp), %r13
142624181:     	movq	0x40(%rsp), %rdi
142624186:     	testq	%r13, %r13
142624189:     	movq	%r13, %r14
14262418c:     	cmoveq	%rdi, %r14
142624190:     	movq	(%r14), %rax
142624193:     	movq	0x10(%r14), %r15
142624197:     	subq	%r15, %rax
14262419a:     	cmpq	$0x1f, %rax
14262419e:     	jbe	0x142624585 <.text+0x2623585>
1426241a4:     	movq	0x8(%r14), %rax
1426241a8:     	addq	$0x10, %r14
1426241ac:     	leaq	-0x1645bf3(%rip), %rcx  # 0x140fde5c0 <.text+0xfdd5c0>
1426241b3:     	movq	%rcx, (%rax,%r15)
1426241b7:     	leaq	-0x7d495e(%rip), %rcx   # 0x141e4f860 <.text+0x1e4e860>
1426241be:     	movq	%rcx, 0x8(%rax,%r15)
1426241c3:     	movq	%r12, 0x10(%rax,%r15)
1426241c8:     	movw	$0x1100, 0x18(%rax,%r15) # imm = 0x1100
1426241d0:     	jmp	0x1426242e7 <.text+0x26232e7>
1426241d5:     	movq	0x50(%rsp), %r12
1426241da:     	movq	0x38(%rsp), %r13
1426241df:     	movq	0x40(%rsp), %rdi
1426241e4:     	testq	%r13, %r13
1426241e7:     	movq	%r13, %r14
1426241ea:     	cmoveq	%rdi, %r14
1426241ee:     	movq	(%r14), %rax
1426241f1:     	movq	0x10(%r14), %r15
1426241f5:     	subq	%r15, %rax
1426241f8:     	cmpq	$0x1f, %rax
1426241fc:     	jbe	0x1426245aa <.text+0x26235aa>
142624202:     	movq	0x8(%r14), %rax
142624206:     	addq	$0x10, %r14
14262420a:     	leaq	-0x1645c51(%rip), %rcx  # 0x140fde5c0 <.text+0xfdd5c0>
142624211:     	movq	%rcx, (%rax,%r15)
142624215:     	leaq	-0x7d49bc(%rip), %rcx   # 0x141e4f860 <.text+0x1e4e860>
14262421c:     	movq	%rcx, 0x8(%rax,%r15)
142624221:     	movq	%r12, 0x10(%rax,%r15)
142624226:     	movw	$0x500, 0x18(%rax,%r15) # imm = 0x500
14262422e:     	jmp	0x1426242e7 <.text+0x26232e7>
142624233:     	movq	0x50(%rsp), %r12
142624238:     	movq	0x38(%rsp), %r13
14262423d:     	movq	0x40(%rsp), %rdi
142624242:     	testq	%r13, %r13
142624245:     	movq	%r13, %r14
142624248:     	cmoveq	%rdi, %r14
14262424c:     	movq	(%r14), %rax
14262424f:     	movq	0x10(%r14), %r15
142624253:     	subq	%r15, %rax
142624256:     	cmpq	$0x1f, %rax
14262425a:     	jbe	0x1426245cf <.text+0x26235cf>
142624260:     	movq	0x8(%r14), %rax
142624264:     	addq	$0x10, %r14
142624268:     	leaq	-0x1645caf(%rip), %rcx  # 0x140fde5c0 <.text+0xfdd5c0>
14262426f:     	movq	%rcx, (%rax,%r15)
142624273:     	leaq	-0x7d4a1a(%rip), %rcx   # 0x141e4f860 <.text+0x1e4e860>
14262427a:     	movq	%rcx, 0x8(%rax,%r15)
14262427f:     	movq	%r12, 0x10(%rax,%r15)
142624284:     	movw	$0xf00, 0x18(%rax,%r15) # imm = 0xF00
14262428c:     	jmp	0x1426242e7 <.text+0x26232e7>
14262428e:     	movq	0x50(%rsp), %r12
142624293:     	movq	0x38(%rsp), %r13
142624298:     	movq	0x40(%rsp), %rdi
14262429d:     	testq	%r13, %r13
1426242a0:     	movq	%r13, %r14
1426242a3:     	cmoveq	%rdi, %r14
1426242a7:     	movq	(%r14), %rax
1426242aa:     	movq	0x10(%r14), %r15
1426242ae:     	subq	%r15, %rax
1426242b1:     	cmpq	$0x1f, %rax
1426242b5:     	jbe	0x1426245f4 <.text+0x26235f4>
1426242bb:     	movq	0x8(%r14), %rax
1426242bf:     	addq	$0x10, %r14
1426242c3:     	leaq	-0x1645d0a(%rip), %rcx  # 0x140fde5c0 <.text+0xfdd5c0>
1426242ca:     	movq	%rcx, (%rax,%r15)
1426242ce:     	leaq	-0x7d4a75(%rip), %rcx   # 0x141e4f860 <.text+0x1e4e860>
1426242d5:     	movq	%rcx, 0x8(%rax,%r15)
1426242da:     	movq	%r12, 0x10(%rax,%r15)
1426242df:     	movw	$0x900, 0x18(%rax,%r15) # imm = 0x900
1426242e7:     	addq	$0x20, %r15
1426242eb:     	movq	%r15, (%r14)
1426242ee:     	testq	%r13, %r13
1426242f1:     	cmovneq	%r13, %rdi
1426242f5:     	movq	(%rdi), %rax
1426242f8:     	movq	0x10(%rdi), %r14
1426242fc:     	subq	%r14, %rax
1426242ff:     	cmpq	$0x1f, %rax
142624303:     	jbe	0x14262448b <.text+0x262348b>
142624309:     	movq	0x8(%rdi), %rax
14262430d:     	leaq	-0x1644be4(%rip), %rcx  # 0x140fdf730 <.text+0xfde730>
142624314:     	movq	%rcx, (%rax,%r14)
142624318:     	leaq	-0x7d4abf(%rip), %r15   # 0x141e4f860 <.text+0x1e4e860>
14262431f:     	movq	%r15, 0x8(%rax,%r14)
142624324:     	movq	%r12, 0x10(%rax,%r14)
142624329:     	movb	$0x0, 0x18(%rax,%r14)
14262432f:     	leaq	0x20(%r14), %rdx
142624333:     	movq	%rdx, 0x10(%rdi)
142624337:     	movq	(%rdi), %rax
14262433a:     	subq	%rdx, %rax
14262433d:     	cmpq	$0x1f, %rax
142624341:     	jbe	0x1426244b0 <.text+0x26234b0>
142624347:     	movq	0x8(%rdi), %rax
14262434b:     	leaq	-0x16457b2(%rip), %rcx  # 0x140fdeba0 <.text+0xfddba0>
142624352:     	movq	%rcx, 0x20(%rax,%r14)
142624357:     	movq	%r15, 0x28(%rax,%r14)
14262435c:     	movq	%r12, 0x30(%rax,%r14)
142624361:     	movl	$0x0, 0x38(%rax,%r14)
14262436a:     	movb	$0x0, 0x3c(%rax,%r14)
142624370:     	leaq	0x40(%r14), %rdx
142624374:     	movq	%rdx, 0x10(%rdi)
142624378:     	movq	(%rdi), %rax
14262437b:     	subq	%rdx, %rax
14262437e:     	cmpq	$0x1f, %rax
142624382:     	jbe	0x1426244d2 <.text+0x26234d2>
142624388:     	movq	0x8(%rdi), %rax
14262438c:     	leaq	-0x1645393(%rip), %rcx  # 0x140fdf000 <.text+0xfde000>
142624393:     	movq	%rcx, 0x40(%rax,%r14)
142624398:     	movq	%r15, 0x48(%rax,%r14)
14262439d:     	movq	%r12, 0x50(%rax,%r14)
1426243a2:     	movb	$0x0, 0x58(%rax,%r14)
1426243a8:     	leaq	0x60(%r14), %rdx
1426243ac:     	movq	%rdx, 0x10(%rdi)
1426243b0:     	movq	(%rdi), %rax
1426243b3:     	subq	%rdx, %rax
1426243b6:     	cmpq	$0x1f, %rax
1426243ba:     	jbe	0x1426244f4 <.text+0x26234f4>
1426243c0:     	movq	0x8(%rdi), %rax
1426243c4:     	leaq	-0x1645b0b(%rip), %rcx  # 0x140fde8c0 <.text+0xfdd8c0>
1426243cb:     	movq	%rcx, 0x60(%rax,%r14)
1426243d0:     	movq	%r15, 0x68(%rax,%r14)
1426243d5:     	movq	%r12, 0x70(%rax,%r14)
1426243da:     	movb	$0x0, 0x78(%rax,%r14)
1426243e0:     	subq	$-0x80, %r14
1426243e4:     	movq	%r14, 0x10(%rdi)
1426243e8:     	movq	0x8(%rsi), %rax
1426243ec:     	movq	0x10(%rsi), %rdi
1426243f0:     	movq	%rdi, %rcx
1426243f3:     	shlq	$0x5, %rcx
1426243f7:     	movq	%rax, %rdx
1426243fa:     	nopw	(%rax,%rax)
142624400:     	testq	%rcx, %rcx
142624403:     	je	0x14262442d <.text+0x262342d>
142624405:     	leaq	0x20(%rdx), %r8
142624409:     	addq	$-0x20, %rcx
14262440d:     	cmpb	%bl, 0x18(%rdx)
142624410:     	movq	%r8, %rdx
142624413:     	jne	0x142624400 <.text+0x2623400>
142624415:     	leaq	0xb253bc(%rip), %rcx    # 0x1431497d8
14262441c:     	leaq	0xb25415(%rip), %r8     # 0x143149838
142624423:     	movl	$0x59, %edx
142624428:     	callq	0x1428d9430 <.text+0x28d8430>
14262442d:     	cmpq	(%rsi), %rdi
142624430:     	jne	0x142624445 <.text+0x2623445>
142624432:     	leaq	0xb25387(%rip), %rdx    # 0x1431497c0
142624439:     	movq	%rsi, %rcx
14262443c:     	callq	0x140562c30 <.text+0x561c30>
142624441:     	movq	0x8(%rsi), %rax
142624445:     	movq	%rdi, %rcx
142624448:     	shlq	$0x5, %rcx
14262444c:     	movl	%ebp, (%rax,%rcx)
14262444f:     	movl	0x5c(%rsp), %edx
142624453:     	movl	%edx, 0x4(%rax,%rcx)
142624457:     	movq	%r12, 0x8(%rax,%rcx)
14262445c:     	movl	$0x0, 0x10(%rax,%rcx)
142624464:     	movb	%bl, 0x18(%rax,%rcx)
142624468:     	incq	%rdi
14262446b:     	movq	%rdi, 0x10(%rsi)
14262446f:     	movaps	0x1e0(%rsp), %xmm6
142624477:     	addq	$0x1f8, %rsp            # imm = 0x1F8
14262447e:     	popq	%rbx
14262447f:     	popq	%rbp
142624480:     	popq	%rdi
142624481:     	popq	%rsi
142624482:     	popq	%r12
142624484:     	popq	%r13
142624486:     	popq	%r14
142624488:     	popq	%r15
14262448a:     	retq
14262448b:     	movq	$0x1, 0x20(%rsp)
142624494:     	movl	$0x20, %r8d
14262449a:     	movl	$0x1, %r9d
1426244a0:     	movq	%rdi, %rcx
1426244a3:     	movq	%r14, %rdx
1426244a6:     	callq	0x1428a58d0 <.text+0x28a48d0>
1426244ab:     	jmp	0x142624309 <.text+0x2623309>
1426244b0:     	movq	$0x1, 0x20(%rsp)
1426244b9:     	movl	$0x20, %r8d
1426244bf:     	movl	$0x1, %r9d
1426244c5:     	movq	%rdi, %rcx
1426244c8:     	callq	0x1428a58d0 <.text+0x28a48d0>
1426244cd:     	jmp	0x142624347 <.text+0x2623347>
1426244d2:     	movq	$0x1, 0x20(%rsp)
1426244db:     	movl	$0x20, %r8d
1426244e1:     	movl	$0x1, %r9d
1426244e7:     	movq	%rdi, %rcx
1426244ea:     	callq	0x1428a58d0 <.text+0x28a48d0>
1426244ef:     	jmp	0x142624388 <.text+0x2623388>
1426244f4:     	movq	$0x1, 0x20(%rsp)
1426244fd:     	movl	$0x20, %r8d
142624503:     	movl	$0x1, %r9d
142624509:     	movq	%rdi, %rcx
14262450c:     	callq	0x1428a58d0 <.text+0x28a48d0>
142624511:     	jmp	0x1426243c0 <.text+0x26233c0>
142624516:     	movq	$0x1, 0x20(%rsp)
14262451f:     	movl	$0x20, %r8d
142624525:     	movl	$0x1, %r9d
14262452b:     	movq	%r14, %rcx
14262452e:     	movq	%r15, %rdx
142624531:     	callq	0x1428a58d0 <.text+0x28a48d0>
142624536:     	jmp	0x14262407c <.text+0x262307c>
14262453b:     	movq	$0x1, 0x20(%rsp)
142624544:     	movl	$0x20, %r8d
14262454a:     	movl	$0x1, %r9d
142624550:     	movq	%r14, %rcx
142624553:     	movq	%r15, %rdx
142624556:     	callq	0x1428a58d0 <.text+0x28a48d0>
14262455b:     	jmp	0x142624146 <.text+0x2623146>
142624560:     	movq	$0x1, 0x20(%rsp)
142624569:     	movl	$0x20, %r8d
14262456f:     	movl	$0x1, %r9d
142624575:     	movq	%r14, %rcx
142624578:     	movq	%r15, %rdx
14262457b:     	callq	0x1428a58d0 <.text+0x28a48d0>
142624580:     	jmp	0x14262403e <.text+0x262303e>
142624585:     	movq	$0x1, 0x20(%rsp)
14262458e:     	movl	$0x20, %r8d
142624594:     	movl	$0x1, %r9d
14262459a:     	movq	%r14, %rcx
14262459d:     	movq	%r15, %rdx
1426245a0:     	callq	0x1428a58d0 <.text+0x28a48d0>
1426245a5:     	jmp	0x1426241a4 <.text+0x26231a4>
1426245aa:     	movq	$0x1, 0x20(%rsp)
1426245b3:     	movl	$0x20, %r8d
1426245b9:     	movl	$0x1, %r9d
1426245bf:     	movq	%r14, %rcx
1426245c2:     	movq	%r15, %rdx
1426245c5:     	callq	0x1428a58d0 <.text+0x28a48d0>
1426245ca:     	jmp	0x142624202 <.text+0x2623202>
1426245cf:     	movq	$0x1, 0x20(%rsp)
1426245d8:     	movl	$0x20, %r8d
1426245de:     	movl	$0x1, %r9d
1426245e4:     	movq	%r14, %rcx
1426245e7:     	movq	%r15, %rdx
1426245ea:     	callq	0x1428a58d0 <.text+0x28a48d0>
1426245ef:     	jmp	0x142624260 <.text+0x2623260>
1426245f4:     	movq	$0x1, 0x20(%rsp)
1426245fd:     	movl	$0x20, %r8d
142624603:     	movl	$0x1, %r9d
142624609:     	movq	%r14, %rcx
14262460c:     	movq	%r15, %rdx
14262460f:     	callq	0x1428a58d0 <.text+0x28a48d0>
142624614:     	jmp	0x1426242bb <.text+0x26232bb>
142624619:     	movq	$0x1, 0x20(%rsp)
142624622:     	movl	$0x20, %r8d
142624628:     	movl	$0x1, %r9d
14262462e:     	movq	%r14, %rcx
142624631:     	movq	%r15, %rdx
142624634:     	callq	0x1428a58d0 <.text+0x28a48d0>
142624639:     	jmp	0x1426240e8 <.text+0x26230e8>
14262463e:     	movups	0x60(%rsp), %xmm0
142624643:     	movups	0x70(%rsp), %xmm1
142624648:     	movaps	%xmm1, 0x40(%rsp)
14262464d:     	movaps	%xmm0, 0x30(%rsp)
142624652:     	leaq	0x96ae8f(%rip), %rax    # 0x142f8f4e8
142624659:     	movq	%rax, 0x20(%rsp)
14262465e:     	leaq	0x96acf3(%rip), %rcx    # 0x142f8f358
142624665:     	leaq	0x96accc(%rip), %r9     # 0x142f8f338
14262466c:     	leaq	0x30(%rsp), %r8
142624671:     	movl	$0x2b, %edx
142624676:     	callq	0x1428d9760 <.text+0x28d8760>
14262467b:     	movups	0x30(%rsp), %xmm0
142624680:     	movups	0x40(%rsp), %xmm1
142624685:     	movaps	%xmm1, 0x90(%rsp)
14262468d:     	movaps	%xmm0, 0x80(%rsp)
142624695:     	leaq	0x96af1c(%rip), %rax    # 0x142f8f5b8
14262469c:     	movq	%rax, 0x20(%rsp)
1426246a1:     	leaq	0x96acb0(%rip), %rcx    # 0x142f8f358
1426246a8:     	leaq	0x96ac89(%rip), %r9     # 0x142f8f338
1426246af:     	leaq	0x80(%rsp), %r8
1426246b7:     	movl	$0x2b, %edx
1426246bc:     	callq	0x1428d9760 <.text+0x28d8760>
1426246c1:     	xorl	%r12d, %r12d
1426246c4:     	leaq	0x96815d(%rip), %r8     # 0x142f8c828
1426246cb:     	movq	%r12, %rcx
1426246ce:     	movq	%r14, %rdx
1426246d1:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1426246d6:     	int3
1426246d7:     	int3
1426246d8:     	int3
1426246d9:     	int3
1426246da:     	int3
1426246db:     	int3
1426246dc:     	int3
1426246dd:     	int3
1426246de:     	int3
1426246df:     	int3

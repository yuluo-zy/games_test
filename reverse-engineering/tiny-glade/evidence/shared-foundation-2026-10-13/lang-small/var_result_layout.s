	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"var_result_layout.a49ccc86763fff83-cgu.0"
	.def	fallback_enabled;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,fallback_enabled
	.globl	fallback_enabled
	.p2align	4
fallback_enabled:
.seh_proc fallback_enabled
	pushq	%rsi
	.seh_pushreg %rsi
	pushq	%rdi
	.seh_pushreg %rdi
	subq	$72, %rsp
	.seh_stackalloc 72
	.seh_endprologue
	leaq	anon.660a563ee3e6231acf568e68a69e9e80.0(%rip), %rdx
	leaq	40(%rsp), %rcx
	movl	$24, %r8d
	callq	_ZN3std3env4_var17h6ba9c502b168573bE
	movq	40(%rsp), %rsi
	movq	%rsi, %rax
	negq	%rax
	movabsq	$-9223372036854775807, %rdi
	jo	.LBB0_7
	jae	.LBB0_7
	cmpq	%rdi, %rsi
	jne	.LBB0_3
	movq	48(%rsp), %rdx
	testq	%rdx, %rdx
	je	.LBB0_7
	leaq	56(%rsp), %rax
	jmp	.LBB0_6
.LBB0_3:
	leaq	48(%rsp), %rax
	movq	%rsi, %rdx
.LBB0_6:
	movq	(%rax), %rcx
	movl	$1, %r8d
	callq	_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
.LBB0_7:
	cmpq	%rdi, %rsi
	setne	%al
	addq	$72, %rsp
	popq	%rdi
	popq	%rsi
	retq
	.seh_endproc

	.section	.rdata,"dr",one_only,anon.660a563ee3e6231acf568e68a69e9e80.0
anon.660a563ee3e6231acf568e68a69e9e80.0:
	.ascii	"TG_DISABLE_LANG_FALLBACK"


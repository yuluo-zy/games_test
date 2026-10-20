
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14067d810: 56                          	pushq	%rsi
14067d811: 57                          	pushq	%rdi
14067d812: 53                          	pushq	%rbx
14067d813: 48 81 ec 00 01 00 00        	subq	$0x100, %rsp            # imm = 0x100
14067d81a: 48 89 d6                    	movq	%rdx, %rsi
14067d81d: 48 8d bc 24 90 00 00 00     	leaq	0x90(%rsp), %rdi
14067d825: 48 89 f9                    	movq	%rdi, %rcx
14067d828: e8 93 88 91 00              	callq	0x140f960c0 <.text+0xf950c0>
14067d82d: 48 8d 5c 24 20              	leaq	0x20(%rsp), %rbx
14067d832: 48 89 d9                    	movq	%rbx, %rcx
14067d835: 48 89 fa                    	movq	%rdi, %rdx
14067d838: e8 c3 1f 90 00              	callq	0x140f7f800 <.text+0xf7e800>
14067d83d: 48 89 f1                    	movq	%rsi, %rcx
14067d840: 48 89 da                    	movq	%rbx, %rdx
14067d843: e8 d8 2d 90 00              	callq	0x140f80620 <.text+0xf7f620>
14067d848: 48 8d 7e 38                 	leaq	0x38(%rsi), %rdi
14067d84c: 48 ba ea b7 55 45 34 f8 bf eb       	movabsq	$-0x144007cbbaaa4816, %rdx # imm = 0xEBBFF8344555B7EA
14067d856: 49 b8 04 cb 30 c7 fb 89 7c f7       	movabsq	$-0x883760438cf34fc, %r8 # imm = 0xF77C89FBC730CB04
14067d860: 48 89 f9                    	movq	%rdi, %rcx
14067d863: e8 88 fe a5 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14067d868: 48 85 c0                    	testq	%rax, %rax
14067d86b: 74 33                       	je	0x14067d8a0 <.text+0x67c8a0>
14067d86d: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14067d874: 76 2a                       	jbe	0x14067d8a0 <.text+0x67c8a0>
14067d876: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14067d87d: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14067d881: 48 85 c0                    	testq	%rax, %rax
14067d884: 74 1a                       	je	0x14067d8a0 <.text+0x67c8a0>
14067d886: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14067d88d: 48 f7 d0                    	notq	%rax
14067d890: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14067d894: 48 c1 e0 05                 	shlq	$0x5, %rax
14067d898: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14067d89e: 75 08                       	jne	0x14067d8a8 <.text+0x67c8a8>
14067d8a0: 48 89 f1                    	movq	%rsi, %rcx
14067d8a3: e8 28 64 91 00              	callq	0x140f93cd0 <.text+0xf92cd0>
14067d8a8: 48 ba 3b 7a 1d 15 1c 15 73 11       	movabsq	$0x1173151c151d7a3b, %rdx # imm = 0x1173151C151D7A3B
14067d8b2: 49 b8 1c db 2a 8c b8 a7 82 69       	movabsq	$0x6982a7b88c2adb1c, %r8 # imm = 0x6982A7B88C2ADB1C
14067d8bc: 48 89 f9                    	movq	%rdi, %rcx
14067d8bf: e8 2c fe a5 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14067d8c4: 48 85 c0                    	testq	%rax, %rax
14067d8c7: 74 33                       	je	0x14067d8fc <.text+0x67c8fc>
14067d8c9: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14067d8d0: 76 2a                       	jbe	0x14067d8fc <.text+0x67c8fc>
14067d8d2: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14067d8d9: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14067d8dd: 48 85 c0                    	testq	%rax, %rax
14067d8e0: 74 1a                       	je	0x14067d8fc <.text+0x67c8fc>
14067d8e2: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14067d8e9: 48 f7 d0                    	notq	%rax
14067d8ec: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14067d8f0: 48 c1 e0 05                 	shlq	$0x5, %rax
14067d8f4: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14067d8fa: 75 08                       	jne	0x14067d904 <.text+0x67c904>
14067d8fc: 48 89 f1                    	movq	%rsi, %rcx
14067d8ff: e8 3c 51 91 00              	callq	0x140f92a40 <.text+0xf91a40>
14067d904: 48 ba 94 4b 7b 60 e7 c9 88 18       	movabsq	$0x1888c9e7607b4b94, %rdx # imm = 0x1888C9E7607B4B94
14067d90e: 49 b8 f4 ab ca 66 d7 08 ce 9a       	movabsq	$-0x6531f7289935540c, %r8 # imm = 0x9ACE08D766CAABF4
14067d918: 48 89 f9                    	movq	%rdi, %rcx
14067d91b: e8 d0 fd a5 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14067d920: 48 85 c0                    	testq	%rax, %rax
14067d923: 74 3e                       	je	0x14067d963 <.text+0x67c963>
14067d925: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14067d92c: 76 35                       	jbe	0x14067d963 <.text+0x67c963>
14067d92e: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14067d935: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14067d939: 48 85 c0                    	testq	%rax, %rax
14067d93c: 74 25                       	je	0x14067d963 <.text+0x67c963>
14067d93e: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14067d945: 48 f7 d0                    	notq	%rax
14067d948: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14067d94c: 48 c1 e0 05                 	shlq	$0x5, %rax
14067d950: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14067d956: 74 0b                       	je	0x14067d963 <.text+0x67c963>
14067d958: 48 81 c4 00 01 00 00        	addq	$0x100, %rsp            # imm = 0x100
14067d95f: 5b                          	popq	%rbx
14067d960: 5f                          	popq	%rdi
14067d961: 5e                          	popq	%rsi
14067d962: c3                          	retq
14067d963: 48 89 f1                    	movq	%rsi, %rcx
14067d966: 48 81 c4 00 01 00 00        	addq	$0x100, %rsp            # imm = 0x100
14067d96d: 5b                          	popq	%rbx
14067d96e: 5f                          	popq	%rdi
14067d96f: 5e                          	popq	%rsi
14067d970: e9 2b 5d 91 00              	jmp	0x140f936a0 <.text+0xf926a0>

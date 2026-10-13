
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1421bb740: 55                          	pushq	%rbp
1421bb741: 56                          	pushq	%rsi
1421bb742: 57                          	pushq	%rdi
1421bb743: 48 81 ec a0 00 00 00        	subq	$0xa0, %rsp
1421bb74a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1421bb752: 48 c7 45 18 fe ff ff ff     	movq	$-0x2, 0x18(%rbp)
1421bb75a: 48 89 ce                    	movq	%rcx, %rsi
1421bb75d: 49 8b 00                    	movq	(%r8), %rax
1421bb760: 48 8d 0d a9 f1 e3 00        	leaq	0xe3f1a9(%rip), %rcx    # 0x142ffa910
1421bb767: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1421bb76b: 48 01 c8                    	addq	%rcx, %rax
1421bb76e: ff e0                       	jmpq	*%rax
1421bb770: 49 8b 40 08                 	movq	0x8(%r8), %rax
1421bb774: 48 c7 02 01 00 00 00        	movq	$0x1, (%rdx)
1421bb77b: 48 89 42 08                 	movq	%rax, 0x8(%rdx)
1421bb77f: e9 d5 01 00 00              	jmp	0x1421bb959 <.text+0x21ba959>
1421bb784: 41 0f 10 40 08              	movups	0x8(%r8), %xmm0
1421bb789: 0f 11 45 a8                 	movups	%xmm0, -0x58(%rbp)
1421bb78d: 48 8d 45 a8                 	leaq	-0x58(%rbp), %rax
1421bb791: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1421bb795: 48 8d 05 04 81 37 fe        	leaq	-0x1c87efc(%rip), %rax  # 0x1405338a0 <.text+0x5328a0>
1421bb79c: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1421bb7a0: 48 8d 05 b1 e8 e3 00        	leaq	0xe3e8b1(%rip), %rax    # 0x142ffa058
1421bb7a7: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
1421bb7ab: 48 c7 45 c0 02 00 00 00     	movq	$0x2, -0x40(%rbp)
1421bb7b3: 48 c7 45 d8 00 00 00 00     	movq	$0x0, -0x28(%rbp)
1421bb7bb: 48 8d 7d e8                 	leaq	-0x18(%rbp), %rdi
1421bb7bf: 48 89 7d c8                 	movq	%rdi, -0x38(%rbp)
1421bb7c3: 48 c7 45 d0 01 00 00 00     	movq	$0x1, -0x30(%rbp)
1421bb7cb: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
1421bb7cf: e8 bc 11 ba fe              	callq	0x140d5c990 <.text+0xd5b990>
1421bb7d4: 48 8b 4d a8                 	movq	-0x58(%rbp), %rcx
1421bb7d8: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
1421bb7dc: e8 ef 3a 9c fe              	callq	0x140b7f2d0 <.text+0xb7e2d0>
1421bb7e1: 48 85 c0                    	testq	%rax, %rax
1421bb7e4: 0f 84 87 00 00 00           	je	0x1421bb871 <.text+0x21ba871>
1421bb7ea: 48 89 45 00                 	movq	%rax, (%rbp)
1421bb7ee: 48 89 e8                    	movq	%rbp, %rax
1421bb7f1: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1421bb7f5: 48 8d 05 e4 f4 b9 fe        	leaq	-0x1460b1c(%rip), %rax  # 0x140d5ace0 <.text+0xd59ce0>
1421bb7fc: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1421bb800: 48 8d 05 91 e8 e3 00        	leaq	0xe3e891(%rip), %rax    # 0x142ffa098
1421bb807: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
1421bb80b: 48 c7 45 c0 02 00 00 00     	movq	$0x2, -0x40(%rbp)
1421bb813: 48 c7 45 d8 00 00 00 00     	movq	$0x0, -0x28(%rbp)
1421bb81b: 48 89 7d c8                 	movq	%rdi, -0x38(%rbp)
1421bb81f: 48 c7 45 d0 01 00 00 00     	movq	$0x1, -0x30(%rbp)
1421bb827: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
1421bb82b: e8 70 12 ba fe              	callq	0x140d5caa0 <.text+0xd5baa0>
1421bb830: 48 8b 45 00                 	movq	(%rbp), %rax
1421bb834: 89 c1                       	movl	%eax, %ecx
1421bb836: 83 e1 03                    	andl	$0x3, %ecx
1421bb839: 83 f9 01                    	cmpl	$0x1, %ecx
1421bb83c: 0f 85 17 01 00 00           	jne	0x1421bb959 <.text+0x21ba959>
1421bb842: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
1421bb846: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
1421bb84a: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
1421bb84e: 48 89 4d 08                 	movq	%rcx, 0x8(%rbp)
1421bb852: 48 8b 40 07                 	movq	0x7(%rax), %rax
1421bb856: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1421bb85a: 48 8b 00                    	movq	(%rax), %rax
1421bb85d: 48 85 c0                    	testq	%rax, %rax
1421bb860: 0f 84 c5 00 00 00           	je	0x1421bb92b <.text+0x21ba92b>
1421bb866: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
1421bb86a: ff d0                       	callq	*%rax
1421bb86c: e9 ba 00 00 00              	jmp	0x1421bb92b <.text+0x21ba92b>
1421bb871: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
1421bb87b: 48 89 06                    	movq	%rax, (%rsi)
1421bb87e: 48 c7 46 08 00 00 00 00     	movq	$0x0, 0x8(%rsi)
1421bb886: e9 db 00 00 00              	jmp	0x1421bb966 <.text+0x21ba966>
1421bb88b: c6 82 90 00 00 00 01        	movb	$0x1, 0x90(%rdx)
1421bb892: e9 c2 00 00 00              	jmp	0x1421bb959 <.text+0x21ba959>
1421bb897: e8 24 23 9e fe              	callq	0x140b9dbc0 <.text+0xb9cbc0>
1421bb89c: 48 89 c1                    	movq	%rax, %rcx
1421bb89f: e8 bc 3e 3d 00              	callq	0x14258f760 <.text+0x258e760>
1421bb8a4: 48 85 c0                    	testq	%rax, %rax
1421bb8a7: 0f 84 ac 00 00 00           	je	0x1421bb959 <.text+0x21ba959>
1421bb8ad: 48 89 45 00                 	movq	%rax, (%rbp)
1421bb8b1: 48 89 e8                    	movq	%rbp, %rax
1421bb8b4: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1421bb8b8: 48 8d 05 21 f4 b9 fe        	leaq	-0x1460bdf(%rip), %rax  # 0x140d5ace0 <.text+0xd59ce0>
1421bb8bf: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1421bb8c3: 48 8d 05 06 e8 e3 00        	leaq	0xe3e806(%rip), %rax    # 0x142ffa0d0
1421bb8ca: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
1421bb8ce: 48 c7 45 c0 02 00 00 00     	movq	$0x2, -0x40(%rbp)
1421bb8d6: 48 c7 45 d8 00 00 00 00     	movq	$0x0, -0x28(%rbp)
1421bb8de: 48 8d 45 e8                 	leaq	-0x18(%rbp), %rax
1421bb8e2: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
1421bb8e6: 48 c7 45 d0 01 00 00 00     	movq	$0x1, -0x30(%rbp)
1421bb8ee: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
1421bb8f2: e8 a9 11 ba fe              	callq	0x140d5caa0 <.text+0xd5baa0>
1421bb8f7: 48 8b 45 00                 	movq	(%rbp), %rax
1421bb8fb: 89 c1                       	movl	%eax, %ecx
1421bb8fd: 83 e1 03                    	andl	$0x3, %ecx
1421bb900: 83 f9 01                    	cmpl	$0x1, %ecx
1421bb903: 75 54                       	jne	0x1421bb959 <.text+0x21ba959>
1421bb905: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
1421bb909: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
1421bb90d: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
1421bb911: 48 89 4d 08                 	movq	%rcx, 0x8(%rbp)
1421bb915: 48 8b 40 07                 	movq	0x7(%rax), %rax
1421bb919: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1421bb91d: 48 8b 00                    	movq	(%rax), %rax
1421bb920: 48 85 c0                    	testq	%rax, %rax
1421bb923: 74 06                       	je	0x1421bb92b <.text+0x21ba92b>
1421bb925: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
1421bb929: ff d0                       	callq	*%rax
1421bb92b: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
1421bb92f: 48 8b 45 10                 	movq	0x10(%rbp), %rax
1421bb933: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1421bb937: 48 85 d2                    	testq	%rdx, %rdx
1421bb93a: 74 09                       	je	0x1421bb945 <.text+0x21ba945>
1421bb93c: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1421bb940: e8 db 82 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421bb945: ba 18 00 00 00              	movl	$0x18, %edx
1421bb94a: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1421bb950: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
1421bb954: e8 c7 82 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421bb959: 48 b8 1e 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffe2, %rax # imm = 0x800000000000001E
1421bb963: 48 89 06                    	movq	%rax, (%rsi)
1421bb966: 48 89 f0                    	movq	%rsi, %rax
1421bb969: 48 81 c4 a0 00 00 00        	addq	$0xa0, %rsp
1421bb970: 5f                          	popq	%rdi
1421bb971: 5e                          	popq	%rsi
1421bb972: 5d                          	popq	%rbp
1421bb973: c3                          	retq

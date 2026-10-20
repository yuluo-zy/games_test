
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141d87140: 55                          	pushq	%rbp
141d87141: 56                          	pushq	%rsi
141d87142: 57                          	pushq	%rdi
141d87143: 48 81 ec 00 01 00 00        	subq	$0x100, %rsp            # imm = 0x100
141d8714a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141d87152: 48 c7 45 78 fe ff ff ff     	movq	$-0x2, 0x78(%rbp)
141d8715a: 48 89 d6                    	movq	%rdx, %rsi
141d8715d: 48 8b 01                    	movq	(%rcx), %rax
141d87160: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
141d87164: 48 8d 91 a0 01 00 00        	leaq	0x1a0(%rcx), %rdx
141d8716b: 4c 8d 81 f8 00 00 00        	leaq	0xf8(%rcx), %r8
141d87172: 4c 8b 88 f8 00 00 00        	movq	0xf8(%rax), %r9
141d87179: 4c 8b 90 00 01 00 00        	movq	0x100(%rax), %r10
141d87180: 4f 8d 14 91                 	leaq	(%r9,%r10,4), %r10
141d87184: 44 0f b6 98 08 01 00 00     	movzbl	0x108(%rax), %r11d
141d8718c: 44 88 5d 48                 	movb	%r11b, 0x48(%rbp)
141d87190: 4c 89 4d f8                 	movq	%r9, -0x8(%rbp)
141d87194: 4c 89 55 00                 	movq	%r10, (%rbp)
141d87198: 48 c7 45 08 08 00 00 00     	movq	$0x8, 0x8(%rbp)
141d871a0: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
141d871a8: 48 c7 45 18 08 00 00 00     	movq	$0x8, 0x18(%rbp)
141d871b0: 0f 57 c0                    	xorps	%xmm0, %xmm0
141d871b3: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
141d871b7: 0f 11 45 30                 	movups	%xmm0, 0x30(%rbp)
141d871bb: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
141d871c3: 48 89 4d d8                 	movq	%rcx, -0x28(%rbp)
141d871c7: 48 89 55 e0                 	movq	%rdx, -0x20(%rbp)
141d871cb: 4c 89 45 e8                 	movq	%r8, -0x18(%rbp)
141d871cf: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
141d871d3: 4c 8d 05 7e 0c 1e 01        	leaq	0x11e0c7e(%rip), %r8    # 0x142f67e58
141d871da: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
141d871de: 48 8d 55 d8                 	leaq	-0x28(%rbp), %rdx
141d871e2: e8 d9 71 03 00              	callq	0x141dbe3c0 <.text+0x1dbd3c0>
141d871e7: 48 8b 45 70                 	movq	0x70(%rbp), %rax
141d871eb: 48 85 c0                    	testq	%rax, %rax
141d871ee: 74 7c                       	je	0x141d8726c <.text+0x1d8626c>
141d871f0: 48 83 f8 18                 	cmpq	$0x18, %rax
141d871f4: 0f 85 9a 00 00 00           	jne	0x141d87294 <.text+0x1d86294>
141d871fa: 48 8b 0e                    	movq	(%rsi), %rcx
141d871fd: 48 8d 15 d4 30 2c 01        	leaq	0x12c30d4(%rip), %rdx   # 0x14304a2d8
141d87204: e8 77 75 58 00              	callq	0x14230e780 <.text+0x230d780>
141d87209: 48 89 c6                    	movq	%rax, %rsi
141d8720c: 48 85 c0                    	testq	%rax, %rax
141d8720f: 74 5b                       	je	0x141d8726c <.text+0x1d8626c>
141d87211: 48 8d 05 f7 31 2c 01        	leaq	0x12c31f7(%rip), %rax   # 0x14304a40f
141d87218: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
141d8721c: 48 c7 45 58 46 00 00 00     	movq	$0x46, 0x58(%rbp)
141d87224: 48 89 f7                    	movq	%rsi, %rdi
141d87227: 48 83 c7 10                 	addq	$0x10, %rdi
141d8722b: 48 83 7e 18 46              	cmpq	$0x46, 0x18(%rsi)
141d87230: 75 7c                       	jne	0x141d872ae <.text+0x1d862ae>
141d87232: 48 8b 17                    	movq	(%rdi), %rdx
141d87235: 48 8d 0d d3 31 2c 01        	leaq	0x12c31d3(%rip), %rcx   # 0x14304a40f
141d8723c: 41 b8 46 00 00 00           	movl	$0x46, %r8d
141d87242: e8 a9 c9 b9 00              	callq	0x142923bf0 <.text+0x2922bf0>
141d87247: 85 c0                       	testl	%eax, %eax
141d87249: 75 63                       	jne	0x141d872ae <.text+0x1d862ae>
141d8724b: 48 8b 0e                    	movq	(%rsi), %rcx
141d8724e: 48 8b 46 08                 	movq	0x8(%rsi), %rax
141d87252: ff 50 18                    	callq	*0x18(%rax)
141d87255: 48 85 c0                    	testq	%rax, %rax
141d87258: 74 12                       	je	0x141d8726c <.text+0x1d8626c>
141d8725a: 48 8b 55 68                 	movq	0x68(%rbp), %rdx
141d8725e: 41 b8 18 00 00 00           	movl	$0x18, %r8d
141d87264: 48 89 c1                    	movq	%rax, %rcx
141d87267: e8 44 7b 58 00              	callq	0x14230edb0 <.text+0x230ddb0>
141d8726c: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
141d87270: 48 85 d2                    	testq	%rdx, %rdx
141d87273: 74 13                       	je	0x141d87288 <.text+0x1d86288>
141d87275: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
141d87279: 48 c1 e2 05                 	shlq	$0x5, %rdx
141d8727d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141d87283: e8 98 c9 88 fe              	callq	0x140613c20 <.text+0x612c20>
141d87288: 90                          	nop
141d87289: 48 81 c4 00 01 00 00        	addq	$0x100, %rsp            # imm = 0x100
141d87290: 5f                          	popq	%rdi
141d87291: 5e                          	popq	%rsi
141d87292: 5d                          	popq	%rbp
141d87293: c3                          	retq
141d87294: 48 8d 0d 15 0d 1e 01        	leaq	0x11e0d15(%rip), %rcx   # 0x142f67fb0
141d8729b: 4c 8d 05 4e 0d 1e 01        	leaq	0x11e0d4e(%rip), %r8    # 0x142f67ff0
141d872a2: ba 39 00 00 00              	movl	$0x39, %edx
141d872a7: e8 84 21 b5 00              	callq	0x1428d9430 <.text+0x28d8430>
141d872ac: eb 26                       	jmp	0x141d872d4 <.text+0x1d862d4>
141d872ae: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
141d872b6: 48 8d 05 03 30 2c 01        	leaq	0x12c3003(%rip), %rax   # 0x14304a2c0
141d872bd: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141d872c2: 48 8d 55 50                 	leaq	0x50(%rbp), %rdx
141d872c6: 4c 8d 4d a8                 	leaq	-0x58(%rbp), %r9
141d872ca: 31 c9                       	xorl	%ecx, %ecx
141d872cc: 49 89 f8                    	movq	%rdi, %r8
141d872cf: e8 6d 38 b4 00              	callq	0x1428cab41 <.text+0x28c9b41>
141d872d4: 0f 0b                       	ud2
141d872d6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141d872e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141d872e5: 55                          	pushq	%rbp
141d872e6: 56                          	pushq	%rsi
141d872e7: 57                          	pushq	%rdi
141d872e8: 48 83 ec 30                 	subq	$0x30, %rsp
141d872ec: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141d872f3: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
141d872f7: 48 85 d2                    	testq	%rdx, %rdx
141d872fa: 74 13                       	je	0x141d8730f <.text+0x1d8630f>
141d872fc: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
141d87300: 48 c1 e2 05                 	shlq	$0x5, %rdx
141d87304: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141d8730a: e8 11 c9 88 fe              	callq	0x140613c20 <.text+0x612c20>
141d8730f: 90                          	nop
141d87310: 48 83 c4 30                 	addq	$0x30, %rsp
141d87314: 5f                          	popq	%rdi
141d87315: 5e                          	popq	%rsi
141d87316: 5d                          	popq	%rbp
141d87317: c3                          	retq

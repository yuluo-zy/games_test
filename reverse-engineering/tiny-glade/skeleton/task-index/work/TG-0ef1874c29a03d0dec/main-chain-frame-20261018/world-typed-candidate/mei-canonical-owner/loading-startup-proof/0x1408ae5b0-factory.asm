
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408ae5b0: 55                          	push	rbp
1408ae5b1: 56                          	push	rsi
1408ae5b2: 57                          	push	rdi
1408ae5b3: 48 81 ec 90 01 00 00        	sub	rsp, 0x190
1408ae5ba: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1408ae5c2: 48 c7 85 08 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x108], -0x2
1408ae5cd: 48 89 ce                    	mov	rsi, rcx
1408ae5d0: 0f 57 c0                    	xorps	xmm0, xmm0
1408ae5d3: 0f 11 45 e8                 	movups	xmmword ptr [rbp - 0x18], xmm0
1408ae5d7: 0f 11 45 00                 	movups	xmmword ptr [rbp], xmm0
1408ae5db: 0f 11 45 18                 	movups	xmmword ptr [rbp + 0x18], xmm0
1408ae5df: 0f 11 45 30                 	movups	xmmword ptr [rbp + 0x30], xmm0
1408ae5e3: 0f 11 45 48                 	movups	xmmword ptr [rbp + 0x48], xmm0
1408ae5e7: c7 45 58 00 00 00 00        	mov	dword ptr [rbp + 0x58], 0x0
1408ae5ee: 0f 11 85 80 00 00 00        	movups	xmmword ptr [rbp + 0x80], xmm0
1408ae5f5: 0f 11 85 98 00 00 00        	movups	xmmword ptr [rbp + 0x98], xmm0
1408ae5fc: 0f 11 85 b0 00 00 00        	movups	xmmword ptr [rbp + 0xb0], xmm0
1408ae603: 0f 11 85 c8 00 00 00        	movups	xmmword ptr [rbp + 0xc8], xmm0
1408ae60a: 0f 11 85 e0 00 00 00        	movups	xmmword ptr [rbp + 0xe0], xmm0
1408ae611: c7 85 f0 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xf0], 0x0
1408ae61b: 48 c7 45 a8 00 00 00 00     	mov	qword ptr [rbp - 0x58], 0x0
1408ae623: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1408ae62b: 48 c7 45 d0 08 00 00 00     	mov	qword ptr [rbp - 0x30], 0x8
1408ae633: 48 c7 45 d8 00 00 00 00     	mov	qword ptr [rbp - 0x28], 0x0
1408ae63b: 48 c7 45 e0 10 00 00 00     	mov	qword ptr [rbp - 0x20], 0x10
1408ae643: 48 c7 45 f8 10 00 00 00     	mov	qword ptr [rbp - 0x8], 0x10
1408ae64b: 48 c7 45 10 10 00 00 00     	mov	qword ptr [rbp + 0x10], 0x10
1408ae653: 48 c7 45 28 10 00 00 00     	mov	qword ptr [rbp + 0x28], 0x10
1408ae65b: 48 c7 45 40 10 00 00 00     	mov	qword ptr [rbp + 0x40], 0x10
1408ae663: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1408ae66d: 48 89 45 60                 	mov	qword ptr [rbp + 0x60], rax
1408ae671: 48 8d 05 a0 e4 1e 02        	lea	rax, [rip + 0x21ee4a0]  # 0x142a9cb18
1408ae678: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
1408ae67c: 48 c7 45 70 43 00 00 00     	mov	qword ptr [rbp + 0x70], 0x43
1408ae684: 48 c7 45 78 10 00 00 00     	mov	qword ptr [rbp + 0x78], 0x10
1408ae68c: 48 c7 85 90 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x90], 0x10
1408ae697: 48 c7 85 a8 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0xa8], 0x10
1408ae6a2: 48 c7 85 c0 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0xc0], 0x10
1408ae6ad: 48 c7 85 d8 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0xd8], 0x10
1408ae6b8: c7 85 f8 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xf8], 0x0
1408ae6c2: 66 c7 85 fc 00 00 00 01 00  	mov	word ptr [rbp + 0xfc], 0x1
1408ae6cb: c7 85 00 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x100], 0x0
1408ae6d5: 0f b6 05 2d 65 d8 02        	movzx	eax, byte ptr [rip + 0x2d8652d] # 0x143634c09
1408ae6dc: b9 60 01 00 00              	mov	ecx, 0x160
1408ae6e1: ba 08 00 00 00              	mov	edx, 0x8
1408ae6e6: e8 25 55 d6 ff              	call	0x140613c10 <.text+0x612c10>
1408ae6eb: 48 85 c0                    	test	rax, rax
1408ae6ee: 74 35                       	je	0x1408ae725 <.text+0x8ad725>
1408ae6f0: 48 89 c7                    	mov	rdi, rax
1408ae6f3: 48 8d 55 a8                 	lea	rdx, [rbp - 0x58]
1408ae6f7: 41 b8 60 01 00 00           	mov	r8d, 0x160
1408ae6fd: 48 89 c1                    	mov	rcx, rax
1408ae700: e8 fb 54 07 02              	call	0x142923c00 <.text+0x2922c00>
1408ae705: 4c 8d 05 04 e3 1e 02        	lea	r8, [rip + 0x21ee304]   # 0x142a9ca10
1408ae70c: 48 89 f1                    	mov	rcx, rsi
1408ae70f: 48 89 fa                    	mov	rdx, rdi
1408ae712: e8 f9 63 43 00              	call	0x140ce4b10 <.text+0xce3b10>
1408ae717: 48 89 f0                    	mov	rax, rsi
1408ae71a: 48 81 c4 90 01 00 00        	add	rsp, 0x190
1408ae721: 5f                          	pop	rdi
1408ae722: 5e                          	pop	rsi
1408ae723: 5d                          	pop	rbp
1408ae724: c3                          	ret
1408ae725: b9 08 00 00 00              	mov	ecx, 0x8
1408ae72a: ba 60 01 00 00              	mov	edx, 0x160
1408ae72f: e8 cc a8 02 02              	call	0x1428d9000 <.text+0x28d8000>
1408ae734: 0f 0b                       	ud2
1408ae736: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1408ae740: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408ae745: 55                          	push	rbp
1408ae746: 56                          	push	rsi
1408ae747: 57                          	push	rdi
1408ae748: 48 83 ec 20                 	sub	rsp, 0x20
1408ae74c: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408ae753: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1408ae757: e8 14 77 d3 ff              	call	0x1405e5e70 <.text+0x5e4e70>
1408ae75c: 90                          	nop
1408ae75d: 48 83 c4 20                 	add	rsp, 0x20
1408ae761: 5f                          	pop	rdi
1408ae762: 5e                          	pop	rsi
1408ae763: 5d                          	pop	rbp
1408ae764: c3                          	ret

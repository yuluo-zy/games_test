
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14179f4e0: 55                          	push	rbp
14179f4e1: 56                          	push	rsi
14179f4e2: 57                          	push	rdi
14179f4e3: 48 81 ec 90 01 00 00        	sub	rsp, 0x190
14179f4ea: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14179f4f2: 48 c7 85 08 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x108], -0x2
14179f4fd: 48 89 ce                    	mov	rsi, rcx
14179f500: 0f 57 c0                    	xorps	xmm0, xmm0
14179f503: 0f 11 45 e8                 	movups	xmmword ptr [rbp - 0x18], xmm0
14179f507: 0f 11 45 00                 	movups	xmmword ptr [rbp], xmm0
14179f50b: 0f 11 45 18                 	movups	xmmword ptr [rbp + 0x18], xmm0
14179f50f: 0f 11 45 30                 	movups	xmmword ptr [rbp + 0x30], xmm0
14179f513: 0f 11 45 48                 	movups	xmmword ptr [rbp + 0x48], xmm0
14179f517: c7 45 58 00 00 00 00        	mov	dword ptr [rbp + 0x58], 0x0
14179f51e: 0f 11 85 80 00 00 00        	movups	xmmword ptr [rbp + 0x80], xmm0
14179f525: 0f 11 85 98 00 00 00        	movups	xmmword ptr [rbp + 0x98], xmm0
14179f52c: 0f 11 85 b0 00 00 00        	movups	xmmword ptr [rbp + 0xb0], xmm0
14179f533: 0f 11 85 c8 00 00 00        	movups	xmmword ptr [rbp + 0xc8], xmm0
14179f53a: 0f 11 85 e0 00 00 00        	movups	xmmword ptr [rbp + 0xe0], xmm0
14179f541: c7 85 f0 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xf0], 0x0
14179f54b: 48 c7 45 a0 00 00 00 00     	mov	qword ptr [rbp - 0x60], 0x0
14179f553: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
14179f55b: 48 c7 45 d0 08 00 00 00     	mov	qword ptr [rbp - 0x30], 0x8
14179f563: 48 c7 45 d8 00 00 00 00     	mov	qword ptr [rbp - 0x28], 0x0
14179f56b: 48 c7 45 e0 10 00 00 00     	mov	qword ptr [rbp - 0x20], 0x10
14179f573: 48 c7 45 f8 10 00 00 00     	mov	qword ptr [rbp - 0x8], 0x10
14179f57b: 48 c7 45 10 10 00 00 00     	mov	qword ptr [rbp + 0x10], 0x10
14179f583: 48 c7 45 28 10 00 00 00     	mov	qword ptr [rbp + 0x28], 0x10
14179f58b: 48 c7 45 40 10 00 00 00     	mov	qword ptr [rbp + 0x40], 0x10
14179f593: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14179f59d: 48 89 45 60                 	mov	qword ptr [rbp + 0x60], rax
14179f5a1: 48 8d 05 c5 c9 53 01        	lea	rax, [rip + 0x153c9c5]  # 0x142cdbf6d
14179f5a8: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
14179f5ac: 48 c7 45 70 34 00 00 00     	mov	qword ptr [rbp + 0x70], 0x34
14179f5b4: 48 c7 45 78 10 00 00 00     	mov	qword ptr [rbp + 0x78], 0x10
14179f5bc: 48 c7 85 90 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x90], 0x10
14179f5c7: 48 c7 85 a8 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0xa8], 0x10
14179f5d2: 48 c7 85 c0 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0xc0], 0x10
14179f5dd: 48 c7 85 d8 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0xd8], 0x10
14179f5e8: c7 85 f8 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xf8], 0x0
14179f5f2: 66 c7 85 fc 00 00 00 01 00  	mov	word ptr [rbp + 0xfc], 0x1
14179f5fb: c7 85 00 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x100], 0x0
14179f605: 0f b6 05 fd 55 e9 01        	movzx	eax, byte ptr [rip + 0x1e955fd] # 0x143634c09
14179f60c: b9 68 01 00 00              	mov	ecx, 0x168
14179f611: ba 08 00 00 00              	mov	edx, 0x8
14179f616: e8 f5 45 e7 fe              	call	0x140613c10 <.text+0x612c10>
14179f61b: 48 85 c0                    	test	rax, rax
14179f61e: 74 35                       	je	0x14179f655 <.text+0x179e655>
14179f620: 48 89 c7                    	mov	rdi, rax
14179f623: 48 8d 55 a0                 	lea	rdx, [rbp - 0x60]
14179f627: 41 b8 68 01 00 00           	mov	r8d, 0x168
14179f62d: 48 89 c1                    	mov	rcx, rax
14179f630: e8 cb 45 18 01              	call	0x142923c00 <.text+0x2922c00>
14179f635: 4c 8d 05 34 c6 53 01        	lea	r8, [rip + 0x153c634]   # 0x142cdbc70
14179f63c: 48 89 f1                    	mov	rcx, rsi
14179f63f: 48 89 fa                    	mov	rdx, rdi
14179f642: e8 c9 54 54 ff              	call	0x140ce4b10 <.text+0xce3b10>
14179f647: 48 89 f0                    	mov	rax, rsi
14179f64a: 48 81 c4 90 01 00 00        	add	rsp, 0x190
14179f651: 5f                          	pop	rdi
14179f652: 5e                          	pop	rsi
14179f653: 5d                          	pop	rbp
14179f654: c3                          	ret
14179f655: b9 08 00 00 00              	mov	ecx, 0x8
14179f65a: ba 68 01 00 00              	mov	edx, 0x168
14179f65f: e8 9c 99 13 01              	call	0x1428d9000 <.text+0x28d8000>
14179f664: 0f 0b                       	ud2
14179f666: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
14179f670: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14179f675: 55                          	push	rbp
14179f676: 56                          	push	rsi
14179f677: 57                          	push	rdi
14179f678: 48 83 ec 20                 	sub	rsp, 0x20
14179f67c: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14179f683: 48 8d 4d a0                 	lea	rcx, [rbp - 0x60]
14179f687: e8 84 78 e4 fe              	call	0x1405e6f10 <.text+0x5e5f10>
14179f68c: 90                          	nop
14179f68d: 48 83 c4 20                 	add	rsp, 0x20
14179f691: 5f                          	pop	rdi
14179f692: 5e                          	pop	rsi
14179f693: 5d                          	pop	rbp
14179f694: c3                          	ret

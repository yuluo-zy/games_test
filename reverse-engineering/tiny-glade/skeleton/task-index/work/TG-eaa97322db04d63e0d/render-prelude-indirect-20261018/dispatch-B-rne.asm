
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141613db0: 55                          	push	rbp
141613db1: 41 57                       	push	r15
141613db3: 41 56                       	push	r14
141613db5: 41 54                       	push	r12
141613db7: 56                          	push	rsi
141613db8: 57                          	push	rdi
141613db9: 53                          	push	rbx
141613dba: 48 81 ec 20 01 00 00        	sub	rsp, 0x120
141613dc1: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
141613dc9: 48 c7 85 98 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x98], -0x2
141613dd4: 48 89 d7                    	mov	rdi, rdx
141613dd7: 48 89 ce                    	mov	rsi, rcx
141613dda: 4d 85 c0                    	test	r8, r8
141613ddd: 74 3a                       	je	0x141613e19 <.text+0x1612e19>
141613ddf: 4c 89 cb                    	mov	rbx, r9
141613de2: 4c 89 c8                    	mov	rax, r9
141613de5: 48 83 e0 0f                 	and	rax, 0xf
141613de9: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
141613ded: 0f 85 3f 01 00 00           	jne	0x141613f32 <.text+0x1612f32>
141613df3: 4d 89 c7                    	mov	r15, r8
141613df6: 4c 89 c0                    	mov	rax, r8
141613df9: 48 83 e0 03                 	and	rax, 0x3
141613dfd: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
141613e01: 0f 85 3c 01 00 00           	jne	0x141613f43 <.text+0x1612f43>
141613e07: 48 81 fb 90 00 00 00        	cmp	rbx, 0x90
141613e0e: 0f 83 b9 01 00 00           	jae	0x141613fcd <.text+0x1612fcd>
141613e14: 45 31 e4                    	xor	r12d, r12d
141613e17: eb 50                       	jmp	0x141613e69 <.text+0x1612e69>
141613e19: 0f b6 05 e9 0d 02 02        	movzx	eax, byte ptr [rip + 0x2020de9] # 0x143634c09
141613e20: bb 80 00 00 00              	mov	ebx, 0x80
141613e25: b9 80 00 00 00              	mov	ecx, 0x80
141613e2a: ba 04 00 00 00              	mov	edx, 0x4
141613e2f: e8 dc fd ff fe              	call	0x140613c10 <.text+0x612c10>
141613e34: 48 85 c0                    	test	rax, rax
141613e37: 0f 84 fa 01 00 00           	je	0x141614037 <.text+0x1613037>
141613e3d: 49 89 c7                    	mov	r15, rax
141613e40: 0f 28 05 99 63 49 01        	movaps	xmm0, xmmword ptr [rip + 0x1496399] # 0x142aaa1e0
141613e47: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
141613e4a: 0f 11 40 10                 	movups	xmmword ptr [rax + 0x10], xmm0
141613e4e: 0f 11 40 20                 	movups	xmmword ptr [rax + 0x20], xmm0
141613e52: 0f 11 40 30                 	movups	xmmword ptr [rax + 0x30], xmm0
141613e56: 0f 11 40 40                 	movups	xmmword ptr [rax + 0x40], xmm0
141613e5a: 0f 11 40 50                 	movups	xmmword ptr [rax + 0x50], xmm0
141613e5e: 0f 11 40 60                 	movups	xmmword ptr [rax + 0x60], xmm0
141613e62: 0f 11 40 70                 	movups	xmmword ptr [rax + 0x70], xmm0
141613e66: 41 b4 01                    	mov	r12b, 0x1
141613e69: 4c 89 bd 88 00 00 00        	mov	qword ptr [rbp + 0x88], r15
141613e70: 44 88 a5 97 00 00 00        	mov	byte ptr [rbp + 0x97], r12b
141613e77: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
141613e7b: ba 80 00 00 00              	mov	edx, 0x80
141613e80: 41 b8 22 01 00 00           	mov	r8d, 0x122
141613e86: 41 b1 01                    	mov	r9b, 0x1
141613e89: e8 52 0f 5c ff              	call	0x140bd4de0 <.text+0xbd3de0>
141613e8e: 4c 89 bd 88 00 00 00        	mov	qword ptr [rbp + 0x88], r15
141613e95: 44 88 a5 97 00 00 00        	mov	byte ptr [rbp + 0x97], r12b
141613e9c: 4c 8d 75 68                 	lea	r14, [rbp + 0x68]
141613ea0: 48 8d 55 10                 	lea	rdx, [rbp + 0x10]
141613ea4: 41 b8 04 00 00 00           	mov	r8d, 0x4
141613eaa: 4c 89 f1                    	mov	rcx, r14
141613ead: e8 6e 10 5c ff              	call	0x140bd4f20 <.text+0xbd3f20>
141613eb2: 4c 89 bd 88 00 00 00        	mov	qword ptr [rbp + 0x88], r15
141613eb9: 44 88 a5 97 00 00 00        	mov	byte ptr [rbp + 0x97], r12b
141613ec0: 48 89 5c 24 30              	mov	qword ptr [rsp + 0x30], rbx
141613ec5: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
141613eca: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
141613ecf: 4c 8d 05 96 c0 66 01        	lea	r8, [rip + 0x166c096]   # 0x142c7ff6c
141613ed6: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
141613eda: 41 b9 16 00 00 00           	mov	r9d, 0x16
141613ee0: 48 89 fa                    	mov	rdx, rdi
141613ee3: e8 d8 ee 08 00              	call	0x1416a2dc0 <.text+0x16a1dc0>
141613ee8: 83 7d c0 0b                 	cmp	dword ptr [rbp - 0x40], 0xb
141613eec: 75 7f                       	jne	0x141613f6d <.text+0x1612f6d>
141613eee: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
141613ef2: 48 89 46 20                 	mov	qword ptr [rsi + 0x20], rax
141613ef6: 0f 10 45 c8                 	movups	xmm0, xmmword ptr [rbp - 0x38]
141613efa: 0f 10 4d d8                 	movups	xmm1, xmmword ptr [rbp - 0x28]
141613efe: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
141613f02: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
141613f05: 45 84 e4                    	test	r12b, r12b
141613f08: 74 13                       	je	0x141613f1d <.text+0x1612f1d>
141613f0a: ba 80 00 00 00              	mov	edx, 0x80
141613f0f: 41 b8 04 00 00 00           	mov	r8d, 0x4
141613f15: 4c 89 f9                    	mov	rcx, r15
141613f18: e8 03 fd ff fe              	call	0x140613c20 <.text+0x612c20>
141613f1d: 48 89 f0                    	mov	rax, rsi
141613f20: 48 81 c4 20 01 00 00        	add	rsp, 0x120
141613f27: 5b                          	pop	rbx
141613f28: 5f                          	pop	rdi
141613f29: 5e                          	pop	rsi
141613f2a: 41 5c                       	pop	r12
141613f2c: 41 5e                       	pop	r14
141613f2e: 41 5f                       	pop	r15
141613f30: 5d                          	pop	rbp
141613f31: c3                          	ret
141613f32: 48 c7 45 10 00 00 00 00     	mov	qword ptr [rbp + 0x10], 0x0
141613f3a: 48 8d 05 07 60 6b 01        	lea	rax, [rip + 0x16b6007]  # 0x142cc9f48
141613f41: eb 0f                       	jmp	0x141613f52 <.text+0x1612f52>
141613f43: 48 c7 45 10 00 00 00 00     	mov	qword ptr [rbp + 0x10], 0x0
141613f4b: 48 8d 05 0e 60 6b 01        	lea	rax, [rip + 0x16b600e]  # 0x142cc9f60
141613f52: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141613f57: 4c 8d 05 e2 5f 6b 01        	lea	r8, [rip + 0x16b5fe2]   # 0x142cc9f40
141613f5e: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
141613f62: 4c 8d 4d 10                 	lea	r9, [rbp + 0x10]
141613f66: 31 c9                       	xor	ecx, ecx
141613f68: e8 29 56 2c 01              	call	0x1428d9596 <.text+0x28d8596>
141613f6d: 44 89 a5 84 00 00 00        	mov	dword ptr [rbp + 0x84], r12d
141613f74: 4c 89 7d 78                 	mov	qword ptr [rbp + 0x78], r15
141613f78: 0f 10 45 00                 	movups	xmm0, xmmword ptr [rbp]
141613f7c: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
141613f80: 0f 10 45 c0                 	movups	xmm0, xmmword ptr [rbp - 0x40]
141613f84: 0f 10 4d d0                 	movups	xmm1, xmmword ptr [rbp - 0x30]
141613f88: 0f 10 55 e0                 	movups	xmm2, xmmword ptr [rbp - 0x20]
141613f8c: 0f 10 5d f0                 	movups	xmm3, xmmword ptr [rbp - 0x10]
141613f90: 0f 29 5d 40                 	movaps	xmmword ptr [rbp + 0x40], xmm3
141613f94: 0f 29 55 30                 	movaps	xmmword ptr [rbp + 0x30], xmm2
141613f98: 0f 29 4d 20                 	movaps	xmmword ptr [rbp + 0x20], xmm1
141613f9c: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
141613fa0: 48 8d 05 01 c2 66 01        	lea	rax, [rip + 0x166c201]  # 0x142c801a8
141613fa7: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141613fac: 48 8d 0d 35 ab 66 01        	lea	rcx, [rip + 0x166ab35]  # 0x142c7eae8
141613fb3: 4c 8d 0d 0e ab 66 01        	lea	r9, [rip + 0x166ab0e]   # 0x142c7eac8
141613fba: 4c 8d 45 10                 	lea	r8, [rbp + 0x10]
141613fbe: ba 2b 00 00 00              	mov	edx, 0x2b
141613fc3: e8 98 57 2c 01              	call	0x1428d9760 <.text+0x28d8760>
141613fc8: e9 87 00 00 00              	jmp	0x141614054 <.text+0x1613054>
141613fcd: 48 8d 05 2b 9d 48 01        	lea	rax, [rip + 0x1489d2b]  # 0x142a9dcff
141613fd4: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
141613fd8: 48 c7 45 c8 4b 00 00 00     	mov	qword ptr [rbp - 0x38], 0x4b
141613fe0: 48 8d 45 c0                 	lea	rax, [rbp - 0x40]
141613fe4: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
141613fe8: 48 8d 05 a1 5e af fe        	lea	rax, [rip - 0x150a15f]  # 0x140109e90 <.text+0x108e90>
141613fef: 48 89 45 70                 	mov	qword ptr [rbp + 0x70], rax
141613ff3: 48 8d 05 ce b7 66 01        	lea	rax, [rip + 0x166b7ce]  # 0x142c7f7c8
141613ffa: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
141613ffe: 48 c7 45 18 01 00 00 00     	mov	qword ptr [rbp + 0x18], 0x1
141614006: 48 c7 45 30 00 00 00 00     	mov	qword ptr [rbp + 0x30], 0x0
14161400e: 48 8d 45 68                 	lea	rax, [rbp + 0x68]
141614012: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
141614016: 48 c7 45 28 01 00 00 00     	mov	qword ptr [rbp + 0x28], 0x1
14161401e: c6 85 97 00 00 00 00        	mov	byte ptr [rbp + 0x97], 0x0
141614025: 48 8d 15 ac b7 66 01        	lea	rdx, [rip + 0x166b7ac]  # 0x142c7f7d8
14161402c: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
141614030: e8 5b 53 2c 01              	call	0x1428d9390 <.text+0x28d8390>
141614035: eb 1d                       	jmp	0x141614054 <.text+0x1613054>
141614037: c6 85 97 00 00 00 00        	mov	byte ptr [rbp + 0x97], 0x0
14161403e: 4c 8d 05 6b aa 66 01        	lea	r8, [rip + 0x166aa6b]   # 0x142c7eab0
141614045: b9 04 00 00 00              	mov	ecx, 0x4
14161404a: ba 80 00 00 00              	mov	edx, 0x80
14161404f: e8 8f 4f 2c 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
141614054: 0f 0b                       	ud2
141614056: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
141614060: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141614065: 55                          	push	rbp
141614066: 41 57                       	push	r15
141614068: 41 56                       	push	r14
14161406a: 41 54                       	push	r12
14161406c: 56                          	push	rsi
14161406d: 57                          	push	rdi
14161406e: 53                          	push	rbx
14161406f: 48 83 ec 40                 	sub	rsp, 0x40
141614073: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14161407a: 48 8b 75 78                 	mov	rsi, qword ptr [rbp + 0x78]
14161407e: 48 89 b5 88 00 00 00        	mov	qword ptr [rbp + 0x88], rsi
141614085: 8b bd 84 00 00 00           	mov	edi, dword ptr [rbp + 0x84]
14161408b: 40 88 bd 97 00 00 00        	mov	byte ptr [rbp + 0x97], dil
141614092: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
141614096: e8 d5 a9 1b ff              	call	0x1407cea70 <.text+0x7cda70>
14161409b: 48 89 b5 88 00 00 00        	mov	qword ptr [rbp + 0x88], rsi
1416140a2: 40 88 bd 97 00 00 00        	mov	byte ptr [rbp + 0x97], dil
1416140a9: 48 83 c4 40                 	add	rsp, 0x40
1416140ad: 5b                          	pop	rbx
1416140ae: 5f                          	pop	rdi
1416140af: 5e                          	pop	rsi
1416140b0: 41 5c                       	pop	r12
1416140b2: 41 5e                       	pop	r14
1416140b4: 41 5f                       	pop	r15
1416140b6: 5d                          	pop	rbp
1416140b7: c3                          	ret
1416140b8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1416140c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1416140c5: 55                          	push	rbp
1416140c6: 41 57                       	push	r15
1416140c8: 41 56                       	push	r14
1416140ca: 41 54                       	push	r12
1416140cc: 56                          	push	rsi
1416140cd: 57                          	push	rdi
1416140ce: 53                          	push	rbx
1416140cf: 48 83 ec 40                 	sub	rsp, 0x40
1416140d3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1416140da: f6 85 97 00 00 00 01        	test	byte ptr [rbp + 0x97], 0x1
1416140e1: 74 17                       	je	0x1416140fa <.text+0x16130fa>
1416140e3: 48 8b 8d 88 00 00 00        	mov	rcx, qword ptr [rbp + 0x88]
1416140ea: ba 80 00 00 00              	mov	edx, 0x80
1416140ef: 41 b8 04 00 00 00           	mov	r8d, 0x4
1416140f5: e8 26 fb ff fe              	call	0x140613c20 <.text+0x612c20>
1416140fa: 90                          	nop
1416140fb: 48 83 c4 40                 	add	rsp, 0x40
1416140ff: 5b                          	pop	rbx
141614100: 5f                          	pop	rdi
141614101: 5e                          	pop	rsi
141614102: 41 5c                       	pop	r12
141614104: 41 5e                       	pop	r14
141614106: 41 5f                       	pop	r15
141614108: 5d                          	pop	rbp
141614109: c3                          	ret

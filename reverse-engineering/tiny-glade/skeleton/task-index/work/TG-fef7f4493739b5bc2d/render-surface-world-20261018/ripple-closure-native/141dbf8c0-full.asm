
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141dbf8c0: 55                          	push	rbp
141dbf8c1: 41 57                       	push	r15
141dbf8c3: 41 56                       	push	r14
141dbf8c5: 41 55                       	push	r13
141dbf8c7: 41 54                       	push	r12
141dbf8c9: 56                          	push	rsi
141dbf8ca: 57                          	push	rdi
141dbf8cb: 53                          	push	rbx
141dbf8cc: 48 81 ec f8 00 00 00        	sub	rsp, 0xf8
141dbf8d3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
141dbf8db: 48 c7 45 70 fe ff ff ff     	mov	qword ptr [rbp + 0x70], -0x2
141dbf8e3: 4c 89 c7                    	mov	rdi, r8
141dbf8e6: 48 89 d3                    	mov	rbx, rdx
141dbf8e9: 48 89 ce                    	mov	rsi, rcx
141dbf8ec: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
141dbf8f0: e8 4b cd ff ff              	call	0x141dbc640 <.text+0x1dbb640>
141dbf8f5: f6 45 a8 01                 	test	byte ptr [rbp - 0x58], 0x1
141dbf8f9: 74 4c                       	je	0x141dbf947 <.text+0x1dbe947>
141dbf8fb: 48 83 3b 00                 	cmp	qword ptr [rbx], 0x0
141dbf8ff: 0f 84 90 00 00 00           	je	0x141dbf995 <.text+0x1dbe995>
141dbf905: 48 8b 43 40                 	mov	rax, qword ptr [rbx + 0x40]
141dbf909: 4c 8b 43 48                 	mov	r8, qword ptr [rbx + 0x48]
141dbf90d: 80 bb 90 00 00 00 00        	cmp	byte ptr [rbx + 0x90], 0x0
141dbf914: 74 4d                       	je	0x141dbf963 <.text+0x1dbe963>
141dbf916: 4c 39 c0                    	cmp	rax, r8
141dbf919: 74 7a                       	je	0x141dbf995 <.text+0x1dbe995>
141dbf91b: 48 8b 4b 28                 	mov	rcx, qword ptr [rbx + 0x28]
141dbf91f: 48 8b 51 10                 	mov	rdx, qword ptr [rcx + 0x10]
141dbf923: 49 29 c0                    	sub	r8, rax
141dbf926: 49 c1 e8 02                 	shr	r8, 0x2
141dbf92a: 45 31 c9                    	xor	r9d, r9d
141dbf92d: 0f 1f 00                    	nop	dword ptr [rax]
141dbf930: 42 8b 0c 88                 	mov	ecx, dword ptr [rax + 4*r9]
141dbf934: 48 39 ca                    	cmp	rdx, rcx
141dbf937: 0f 86 de 01 00 00           	jbe	0x141dbfb1b <.text+0x1dbeb1b>
141dbf93d: 49 ff c1                    	inc	r9
141dbf940: 4d 39 c8                    	cmp	r8, r9
141dbf943: 75 eb                       	jne	0x141dbf930 <.text+0x1dbe930>
141dbf945: eb 4e                       	jmp	0x141dbf995 <.text+0x1dbe995>
141dbf947: 48 c7 06 00 00 00 00        	mov	qword ptr [rsi], 0x0
141dbf94e: 48 c7 46 08 04 00 00 00     	mov	qword ptr [rsi + 0x8], 0x4
141dbf956: 48 c7 46 10 00 00 00 00     	mov	qword ptr [rsi + 0x10], 0x0
141dbf95e: e9 85 01 00 00              	jmp	0x141dbfae8 <.text+0x1dbeae8>
141dbf963: 4c 39 c0                    	cmp	rax, r8
141dbf966: 74 2d                       	je	0x141dbf995 <.text+0x1dbe995>
141dbf968: 48 8b 4b 30                 	mov	rcx, qword ptr [rbx + 0x30]
141dbf96c: 48 8b 51 10                 	mov	rdx, qword ptr [rcx + 0x10]
141dbf970: 49 29 c0                    	sub	r8, rax
141dbf973: 49 c1 e8 02                 	shr	r8, 0x2
141dbf977: 45 31 c9                    	xor	r9d, r9d
141dbf97a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
141dbf980: 42 8b 0c 88                 	mov	ecx, dword ptr [rax + 4*r9]
141dbf984: 48 39 ca                    	cmp	rdx, rcx
141dbf987: 0f 86 9a 01 00 00           	jbe	0x141dbfb27 <.text+0x1dbeb27>
141dbf98d: 49 ff c1                    	inc	r9
141dbf990: 4d 39 c8                    	cmp	r8, r9
141dbf993: 75 eb                       	jne	0x141dbf980 <.text+0x1dbe980>
141dbf995: 0f b6 05 6d 52 87 01        	movzx	eax, byte ptr [rip + 0x187526d] # 0x143634c09
141dbf99c: b9 40 00 00 00              	mov	ecx, 0x40
141dbf9a1: ba 04 00 00 00              	mov	edx, 0x4
141dbf9a6: e8 65 42 85 fe              	call	0x140613c10 <.text+0x612c10>
141dbf9ab: 48 85 c0                    	test	rax, rax
141dbf9ae: 0f 84 7f 01 00 00           	je	0x141dbfb33 <.text+0x1dbeb33>
141dbf9b4: 49 89 c4                    	mov	r12, rax
141dbf9b7: 48 8d 45 ac                 	lea	rax, [rbp - 0x54]
141dbf9bb: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
141dbf9be: 41 0f 11 04 24              	movups	xmmword ptr [r12], xmm0
141dbf9c3: 48 c7 45 58 04 00 00 00     	mov	qword ptr [rbp + 0x58], 0x4
141dbf9cb: 4c 89 65 60                 	mov	qword ptr [rbp + 0x60], r12
141dbf9cf: 48 c7 45 68 01 00 00 00     	mov	qword ptr [rbp + 0x68], 0x1
141dbf9d7: 48 8d 7d a8                 	lea	rdi, [rbp - 0x58]
141dbf9db: 41 b8 98 00 00 00           	mov	r8d, 0x98
141dbf9e1: 48 89 f9                    	mov	rcx, rdi
141dbf9e4: 48 89 da                    	mov	rdx, rbx
141dbf9e7: e8 14 42 b6 00              	call	0x142923c00 <.text+0x2922c00>
141dbf9ec: bb 01 00 00 00              	mov	ebx, 0x1
141dbf9f1: 4c 8d 6d 48                 	lea	r13, [rbp + 0x48]
141dbf9f5: 4c 8d 75 44                 	lea	r14, [rbp + 0x44]
141dbf9f9: 4c 8d 7d 58                 	lea	r15, [rbp + 0x58]
141dbf9fd: eb 19                       	jmp	0x141dbfa18 <.text+0x1dbea18>
141dbf9ff: 90                          	nop
141dbfa00: 48 89 d8                    	mov	rax, rbx
141dbfa03: 48 c1 e0 04                 	shl	rax, 0x4
141dbfa07: 41 0f 10 45 00              	movups	xmm0, xmmword ptr [r13]
141dbfa0c: 41 0f 11 04 04              	movups	xmmword ptr [r12 + rax], xmm0
141dbfa11: 48 ff c3                    	inc	rbx
141dbfa14: 48 89 5d 68                 	mov	qword ptr [rbp + 0x68], rbx
141dbfa18: 4c 89 f1                    	mov	rcx, r14
141dbfa1b: 48 89 fa                    	mov	rdx, rdi
141dbfa1e: e8 1d cc ff ff              	call	0x141dbc640 <.text+0x1dbb640>
141dbfa23: 83 7d 44 01                 	cmp	dword ptr [rbp + 0x44], 0x1
141dbfa27: 0f 85 ac 00 00 00           	jne	0x141dbfad9 <.text+0x1dbead9>
141dbfa2d: 48 3b 5d 58                 	cmp	rbx, qword ptr [rbp + 0x58]
141dbfa31: 75 cd                       	jne	0x141dbfa00 <.text+0x1dbea00>
141dbfa33: 48 83 7d a8 00              	cmp	qword ptr [rbp - 0x58], 0x0
141dbfa38: 74 76                       	je	0x141dbfab0 <.text+0x1dbeab0>
141dbfa3a: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
141dbfa3e: 4c 8b 45 f0                 	mov	r8, qword ptr [rbp - 0x10]
141dbfa42: 80 7d 38 00                 	cmp	byte ptr [rbp + 0x38], 0x0
141dbfa46: 74 2f                       	je	0x141dbfa77 <.text+0x1dbea77>
141dbfa48: 4c 39 c0                    	cmp	rax, r8
141dbfa4b: 74 63                       	je	0x141dbfab0 <.text+0x1dbeab0>
141dbfa4d: 48 8b 4d d0                 	mov	rcx, qword ptr [rbp - 0x30]
141dbfa51: 48 8b 51 10                 	mov	rdx, qword ptr [rcx + 0x10]
141dbfa55: 49 29 c0                    	sub	r8, rax
141dbfa58: 49 c1 e8 02                 	shr	r8, 0x2
141dbfa5c: 45 31 c9                    	xor	r9d, r9d
141dbfa5f: 90                          	nop
141dbfa60: 42 8b 0c 88                 	mov	ecx, dword ptr [rax + 4*r9]
141dbfa64: 48 39 ca                    	cmp	rdx, rcx
141dbfa67: 0f 86 a0 00 00 00           	jbe	0x141dbfb0d <.text+0x1dbeb0d>
141dbfa6d: 49 ff c1                    	inc	r9
141dbfa70: 4d 39 c8                    	cmp	r8, r9
141dbfa73: 75 eb                       	jne	0x141dbfa60 <.text+0x1dbea60>
141dbfa75: eb 39                       	jmp	0x141dbfab0 <.text+0x1dbeab0>
141dbfa77: 4c 39 c0                    	cmp	rax, r8
141dbfa7a: 74 34                       	je	0x141dbfab0 <.text+0x1dbeab0>
141dbfa7c: 48 8b 4d d8                 	mov	rcx, qword ptr [rbp - 0x28]
141dbfa80: 48 8b 51 10                 	mov	rdx, qword ptr [rcx + 0x10]
141dbfa84: 49 29 c0                    	sub	r8, rax
141dbfa87: 49 c1 e8 02                 	shr	r8, 0x2
141dbfa8b: 45 31 c9                    	xor	r9d, r9d
141dbfa8e: 66 90                       	nop
141dbfa90: 42 8b 0c 88                 	mov	ecx, dword ptr [rax + 4*r9]
141dbfa94: 48 39 ca                    	cmp	rdx, rcx
141dbfa97: 76 66                       	jbe	0x141dbfaff <.text+0x1dbeaff>
141dbfa99: 49 ff c1                    	inc	r9
141dbfa9c: 4d 39 c8                    	cmp	r8, r9
141dbfa9f: 75 ef                       	jne	0x141dbfa90 <.text+0x1dbea90>
141dbfaa1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
141dbfab0: 48 c7 44 24 20 10 00 00 00  	mov	qword ptr [rsp + 0x20], 0x10
141dbfab9: 41 b8 01 00 00 00           	mov	r8d, 0x1
141dbfabf: 41 b9 04 00 00 00           	mov	r9d, 0x4
141dbfac5: 4c 89 f9                    	mov	rcx, r15
141dbfac8: 48 89 da                    	mov	rdx, rbx
141dbfacb: e8 00 5d ae 00              	call	0x1428a57d0 <.text+0x28a47d0>
141dbfad0: 4c 8b 65 60                 	mov	r12, qword ptr [rbp + 0x60]
141dbfad4: e9 27 ff ff ff              	jmp	0x141dbfa00 <.text+0x1dbea00>
141dbfad9: 48 8b 45 68                 	mov	rax, qword ptr [rbp + 0x68]
141dbfadd: 48 89 46 10                 	mov	qword ptr [rsi + 0x10], rax
141dbfae1: 0f 10 45 58                 	movups	xmm0, xmmword ptr [rbp + 0x58]
141dbfae5: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
141dbfae8: 48 89 f0                    	mov	rax, rsi
141dbfaeb: 48 81 c4 f8 00 00 00        	add	rsp, 0xf8
141dbfaf2: 5b                          	pop	rbx
141dbfaf3: 5f                          	pop	rdi
141dbfaf4: 5e                          	pop	rsi
141dbfaf5: 41 5c                       	pop	r12
141dbfaf7: 41 5d                       	pop	r13
141dbfaf9: 41 5e                       	pop	r14
141dbfafb: 41 5f                       	pop	r15
141dbfafd: 5d                          	pop	rbp
141dbfafe: c3                          	ret
141dbfaff: 4c 8d 05 f2 a9 28 01        	lea	r8, [rip + 0x128a9f2]   # 0x14304a4f8
141dbfb06: e8 0d 9a b1 00              	call	0x1428d9518 <.text+0x28d8518>
141dbfb0b: eb 0c                       	jmp	0x141dbfb19 <.text+0x1dbeb19>
141dbfb0d: 4c 8d 05 fc a9 28 01        	lea	r8, [rip + 0x128a9fc]   # 0x14304a510
141dbfb14: e8 ff 99 b1 00              	call	0x1428d9518 <.text+0x28d8518>
141dbfb19: 0f 0b                       	ud2
141dbfb1b: 4c 8d 05 ee a9 28 01        	lea	r8, [rip + 0x128a9ee]   # 0x14304a510
141dbfb22: e8 f1 99 b1 00              	call	0x1428d9518 <.text+0x28d8518>
141dbfb27: 4c 8d 05 ca a9 28 01        	lea	r8, [rip + 0x128a9ca]   # 0x14304a4f8
141dbfb2e: e8 e5 99 b1 00              	call	0x1428d9518 <.text+0x28d8518>
141dbfb33: b9 04 00 00 00              	mov	ecx, 0x4
141dbfb38: ba 40 00 00 00              	mov	edx, 0x40
141dbfb3d: 49 89 f8                    	mov	r8, rdi
141dbfb40: e8 9e 94 b1 00              	call	0x1428d8fe3 <.text+0x28d7fe3>
141dbfb45: cc                          	int3
141dbfb46: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
141dbfb50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141dbfb55: 55                          	push	rbp
141dbfb56: 41 57                       	push	r15
141dbfb58: 41 56                       	push	r14
141dbfb5a: 41 55                       	push	r13
141dbfb5c: 41 54                       	push	r12
141dbfb5e: 56                          	push	rsi
141dbfb5f: 57                          	push	rdi
141dbfb60: 53                          	push	rbx
141dbfb61: 48 83 ec 28                 	sub	rsp, 0x28
141dbfb65: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141dbfb6c: 48 8b 55 58                 	mov	rdx, qword ptr [rbp + 0x58]
141dbfb70: 48 85 d2                    	test	rdx, rdx
141dbfb73: 74 13                       	je	0x141dbfb88 <.text+0x1dbeb88>
141dbfb75: 48 8b 4d 60                 	mov	rcx, qword ptr [rbp + 0x60]
141dbfb79: 48 c1 e2 04                 	shl	rdx, 0x4
141dbfb7d: 41 b8 04 00 00 00           	mov	r8d, 0x4
141dbfb83: e8 98 40 85 fe              	call	0x140613c20 <.text+0x612c20>
141dbfb88: 90                          	nop
141dbfb89: 48 83 c4 28                 	add	rsp, 0x28
141dbfb8d: 5b                          	pop	rbx
141dbfb8e: 5f                          	pop	rdi
141dbfb8f: 5e                          	pop	rsi
141dbfb90: 41 5c                       	pop	r12
141dbfb92: 41 5d                       	pop	r13
141dbfb94: 41 5e                       	pop	r14
141dbfb96: 41 5f                       	pop	r15
141dbfb98: 5d                          	pop	rbp
141dbfb99: c3                          	ret

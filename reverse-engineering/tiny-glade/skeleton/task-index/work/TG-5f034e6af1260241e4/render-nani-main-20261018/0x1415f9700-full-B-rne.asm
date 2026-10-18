
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415f9700: 41 57                       	push	r15
1415f9702: 41 56                       	push	r14
1415f9704: 41 55                       	push	r13
1415f9706: 41 54                       	push	r12
1415f9708: 56                          	push	rsi
1415f9709: 57                          	push	rdi
1415f970a: 55                          	push	rbp
1415f970b: 53                          	push	rbx
1415f970c: 48 83 ec 68                 	sub	rsp, 0x68
1415f9710: 0f b6 79 08                 	movzx	edi, byte ptr [rcx + 0x8]
1415f9714: 0f b6 41 09                 	movzx	eax, byte ptr [rcx + 0x9]
1415f9718: 48 89 44 24 60              	mov	qword ptr [rsp + 0x60], rax
1415f971d: 40 38 c7                    	cmp	dil, al
1415f9720: 0f 83 a1 03 00 00           	jae	0x1415f9ac7 <.text+0x15f8ac7>
1415f9726: 48 8b 32                    	mov	rsi, qword ptr [rdx]
1415f9729: 48 8b 42 08                 	mov	rax, qword ptr [rdx + 0x8]
1415f972d: 48 89 44 24 58              	mov	qword ptr [rsp + 0x58], rax
1415f9732: 8b 42 10                    	mov	eax, dword ptr [rdx + 0x10]
1415f9735: 89 44 24 24                 	mov	dword ptr [rsp + 0x24], eax
1415f9739: 4c 8b 29                    	mov	r13, qword ptr [rcx]
1415f973c: 89 f8                       	mov	eax, edi
1415f973e: c1 e0 04                    	shl	eax, 0x4
1415f9741: 48 05 80 0d 00 00           	add	rax, 0xd80
1415f9747: 48 89 44 24 50              	mov	qword ptr [rsp + 0x50], rax
1415f974c: 31 c9                       	xor	ecx, ecx
1415f974e: 48 89 7c 24 48              	mov	qword ptr [rsp + 0x48], rdi
1415f9753: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1415f9760: 89 c8                       	mov	eax, ecx
1415f9762: 44 0f b6 c1                 	movzx	r8d, cl
1415f9766: 48 8d 15 4b 49 68 01        	lea	rdx, [rip + 0x168494b]  # 0x142c7e0b8
1415f976d: 4a 63 0c 82                 	movsxd	rcx, dword ptr [rdx + 4*r8]
1415f9771: 48 01 d1                    	add	rcx, rdx
1415f9774: ff e1                       	jmp	rcx
1415f9776: b1 01                       	mov	cl, 0x1
1415f9778: 80 3d 1d b6 03 02 00        	cmp	byte ptr [rip + 0x203b61d], 0x0 # 0x143634d9c
1415f977f: 0f 84 db 01 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f9785: e9 eb 01 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f978a: b1 11                       	mov	cl, 0x11
1415f978c: 80 3d 09 b6 03 02 00        	cmp	byte ptr [rip + 0x203b609], 0x0 # 0x143634d9c
1415f9793: 0f 84 c7 01 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f9799: e9 d7 01 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f979e: b1 0f                       	mov	cl, 0xf
1415f97a0: 80 3d f5 b5 03 02 00        	cmp	byte ptr [rip + 0x203b5f5], 0x0 # 0x143634d9c
1415f97a7: 0f 84 b3 01 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f97ad: e9 c3 01 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f97b2: b1 0c                       	mov	cl, 0xc
1415f97b4: 80 3d e1 b5 03 02 00        	cmp	byte ptr [rip + 0x203b5e1], 0x0 # 0x143634d9c
1415f97bb: 0f 84 9f 01 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f97c1: e9 af 01 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f97c6: b1 05                       	mov	cl, 0x5
1415f97c8: 80 3d cd b5 03 02 00        	cmp	byte ptr [rip + 0x203b5cd], 0x0 # 0x143634d9c
1415f97cf: 0f 84 8b 01 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f97d5: e9 9b 01 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f97da: b1 0d                       	mov	cl, 0xd
1415f97dc: 80 3d b9 b5 03 02 00        	cmp	byte ptr [rip + 0x203b5b9], 0x0 # 0x143634d9c
1415f97e3: 0f 84 77 01 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f97e9: e9 87 01 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f97ee: b1 0a                       	mov	cl, 0xa
1415f97f0: 80 3d a5 b5 03 02 00        	cmp	byte ptr [rip + 0x203b5a5], 0x0 # 0x143634d9c
1415f97f7: 0f 84 63 01 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f97fd: e9 73 01 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f9802: b1 1a                       	mov	cl, 0x1a
1415f9804: 80 3d 91 b5 03 02 00        	cmp	byte ptr [rip + 0x203b591], 0x0 # 0x143634d9c
1415f980b: 0f 84 4f 01 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f9811: e9 5f 01 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f9816: b1 19                       	mov	cl, 0x19
1415f9818: 80 3d 7d b5 03 02 00        	cmp	byte ptr [rip + 0x203b57d], 0x0 # 0x143634d9c
1415f981f: 0f 84 3b 01 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f9825: e9 4b 01 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f982a: b1 03                       	mov	cl, 0x3
1415f982c: 80 3d 69 b5 03 02 00        	cmp	byte ptr [rip + 0x203b569], 0x0 # 0x143634d9c
1415f9833: 0f 84 27 01 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f9839: e9 37 01 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f983e: b1 10                       	mov	cl, 0x10
1415f9840: 80 3d 55 b5 03 02 00        	cmp	byte ptr [rip + 0x203b555], 0x0 # 0x143634d9c
1415f9847: 0f 84 13 01 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f984d: e9 23 01 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f9852: b1 04                       	mov	cl, 0x4
1415f9854: 80 3d 41 b5 03 02 00        	cmp	byte ptr [rip + 0x203b541], 0x0 # 0x143634d9c
1415f985b: 0f 84 ff 00 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f9861: e9 0f 01 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f9866: b1 08                       	mov	cl, 0x8
1415f9868: 80 3d 2d b5 03 02 00        	cmp	byte ptr [rip + 0x203b52d], 0x0 # 0x143634d9c
1415f986f: 0f 84 eb 00 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f9875: e9 fb 00 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f987a: b1 02                       	mov	cl, 0x2
1415f987c: 80 3d 19 b5 03 02 00        	cmp	byte ptr [rip + 0x203b519], 0x0 # 0x143634d9c
1415f9883: 0f 84 d7 00 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f9889: e9 e7 00 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f988e: b1 12                       	mov	cl, 0x12
1415f9890: 80 3d 05 b5 03 02 00        	cmp	byte ptr [rip + 0x203b505], 0x0 # 0x143634d9c
1415f9897: 0f 84 c3 00 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f989d: e9 d3 00 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f98a2: b1 15                       	mov	cl, 0x15
1415f98a4: 80 3d f1 b4 03 02 00        	cmp	byte ptr [rip + 0x203b4f1], 0x0 # 0x143634d9c
1415f98ab: 0f 84 af 00 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f98b1: e9 bf 00 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f98b6: b1 06                       	mov	cl, 0x6
1415f98b8: 80 3d dd b4 03 02 00        	cmp	byte ptr [rip + 0x203b4dd], 0x0 # 0x143634d9c
1415f98bf: 0f 84 9b 00 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f98c5: e9 ab 00 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f98ca: b1 16                       	mov	cl, 0x16
1415f98cc: 80 3d c9 b4 03 02 00        	cmp	byte ptr [rip + 0x203b4c9], 0x0 # 0x143634d9c
1415f98d3: 0f 84 87 00 00 00           	je	0x1415f9960 <.text+0x15f8960>
1415f98d9: e9 97 00 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f98de: b1 0e                       	mov	cl, 0xe
1415f98e0: 80 3d b5 b4 03 02 00        	cmp	byte ptr [rip + 0x203b4b5], 0x0 # 0x143634d9c
1415f98e7: 74 77                       	je	0x1415f9960 <.text+0x15f8960>
1415f98e9: e9 87 00 00 00              	jmp	0x1415f9975 <.text+0x15f8975>
1415f98ee: b1 07                       	mov	cl, 0x7
1415f98f0: 80 3d a5 b4 03 02 00        	cmp	byte ptr [rip + 0x203b4a5], 0x0 # 0x143634d9c
1415f98f7: 74 67                       	je	0x1415f9960 <.text+0x15f8960>
1415f98f9: eb 7a                       	jmp	0x1415f9975 <.text+0x15f8975>
1415f98fb: b1 13                       	mov	cl, 0x13
1415f98fd: 80 3d 98 b4 03 02 00        	cmp	byte ptr [rip + 0x203b498], 0x0 # 0x143634d9c
1415f9904: 74 5a                       	je	0x1415f9960 <.text+0x15f8960>
1415f9906: eb 6d                       	jmp	0x1415f9975 <.text+0x15f8975>
1415f9908: b1 0b                       	mov	cl, 0xb
1415f990a: 80 3d 8b b4 03 02 00        	cmp	byte ptr [rip + 0x203b48b], 0x0 # 0x143634d9c
1415f9911: 74 4d                       	je	0x1415f9960 <.text+0x15f8960>
1415f9913: eb 60                       	jmp	0x1415f9975 <.text+0x15f8975>
1415f9915: b1 09                       	mov	cl, 0x9
1415f9917: 80 3d 7e b4 03 02 00        	cmp	byte ptr [rip + 0x203b47e], 0x0 # 0x143634d9c
1415f991e: 74 40                       	je	0x1415f9960 <.text+0x15f8960>
1415f9920: eb 53                       	jmp	0x1415f9975 <.text+0x15f8975>
1415f9922: b1 17                       	mov	cl, 0x17
1415f9924: 80 3d 71 b4 03 02 00        	cmp	byte ptr [rip + 0x203b471], 0x0 # 0x143634d9c
1415f992b: 74 33                       	je	0x1415f9960 <.text+0x15f8960>
1415f992d: eb 46                       	jmp	0x1415f9975 <.text+0x15f8975>
1415f992f: b1 14                       	mov	cl, 0x14
1415f9931: 80 3d 64 b4 03 02 00        	cmp	byte ptr [rip + 0x203b464], 0x0 # 0x143634d9c
1415f9938: 74 26                       	je	0x1415f9960 <.text+0x15f8960>
1415f993a: eb 39                       	jmp	0x1415f9975 <.text+0x15f8975>
1415f993c: b1 18                       	mov	cl, 0x18
1415f993e: 80 3d 57 b4 03 02 00        	cmp	byte ptr [rip + 0x203b457], 0x0 # 0x143634d9c
1415f9945: 74 19                       	je	0x1415f9960 <.text+0x15f8960>
1415f9947: eb 2c                       	jmp	0x1415f9975 <.text+0x15f8975>
1415f9949: b1 1b                       	mov	cl, 0x1b
1415f994b: 80 3d 4a b4 03 02 00        	cmp	byte ptr [rip + 0x203b44a], 0x0 # 0x143634d9c
1415f9952: 75 21                       	jne	0x1415f9975 <.text+0x15f8975>
1415f9954: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1415f9960: 3c 01                       	cmp	al, 0x1
1415f9962: 0f 87 f8 fd ff ff           	ja	0x1415f9760 <.text+0x15f8760>
1415f9968: 80 3d 2e b4 03 02 00        	cmp	byte ptr [rip + 0x203b42e], 0x0 # 0x143634d9d
1415f996f: 0f 85 eb fd ff ff           	jne	0x1415f9760 <.text+0x15f8760>
1415f9975: 89 4c 24 20                 	mov	dword ptr [rsp + 0x20], ecx
1415f9979: c0 e0 02                    	shl	al, 0x2
1415f997c: 0f b6 e8                    	movzx	ebp, al
1415f997f: 45 89 c6                    	mov	r14d, r8d
1415f9982: 41 c1 e6 06                 	shl	r14d, 0x6
1415f9986: 4c 03 74 24 50              	add	r14, qword ptr [rsp + 0x50]
1415f998b: 4c 89 44 24 28              	mov	qword ptr [rsp + 0x28], r8
1415f9990: 4c 89 c3                    	mov	rbx, r8
1415f9993: c1 e3 04                    	shl	ebx, 0x4
1415f9996: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1415f99a0: 4c 8d 0c 2f                 	lea	r9, [rdi + rbp]
1415f99a4: 49 8b 55 40                 	mov	rdx, qword ptr [r13 + 0x40]
1415f99a8: 4c 39 ca                    	cmp	rdx, r9
1415f99ab: 0f 86 d2 02 00 00           	jbe	0x1415f9c83 <.text+0x15f8c83>
1415f99b1: 49 8b 45 38                 	mov	rax, qword ptr [r13 + 0x38]
1415f99b5: 48 01 d8                    	add	rax, rbx
1415f99b8: 4c 8d 04 2f                 	lea	r8, [rdi + rbp]
1415f99bc: 49 ff c0                    	inc	r8
1415f99bf: 49 39 d0                    	cmp	r8, rdx
1415f99c2: 72 1c                       	jb	0x1415f99e0 <.text+0x15f89e0>
1415f99c4: 49 39 d0                    	cmp	r8, rdx
1415f99c7: 73 20                       	jae	0x1415f99e9 <.text+0x15f89e9>
1415f99c9: 49 8b 55 58                 	mov	rdx, qword ptr [r13 + 0x58]
1415f99cd: 4c 39 ca                    	cmp	rdx, r9
1415f99d0: 77 28                       	ja	0x1415f99fa <.text+0x15f89fa>
1415f99d2: e9 c1 02 00 00              	jmp	0x1415f9c98 <.text+0x15f8c98>
1415f99d7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1415f99e0: 8b 4c b8 04                 	mov	ecx, dword ptr [rax + 4*rdi + 0x4]
1415f99e4: 49 39 d0                    	cmp	r8, rdx
1415f99e7: 72 e0                       	jb	0x1415f99c9 <.text+0x15f89c9>
1415f99e9: 41 8b 4d 68                 	mov	ecx, dword ptr [r13 + 0x68]
1415f99ed: 49 8b 55 58                 	mov	rdx, qword ptr [r13 + 0x58]
1415f99f1: 4c 39 ca                    	cmp	rdx, r9
1415f99f4: 0f 86 9e 02 00 00           	jbe	0x1415f9c98 <.text+0x15f8c98>
1415f99fa: 4d 8b 4d 50                 	mov	r9, qword ptr [r13 + 0x50]
1415f99fe: 49 01 d9                    	add	r9, rbx
1415f9a01: 49 39 d0                    	cmp	r8, rdx
1415f9a04: 73 05                       	jae	0x1415f9a0b <.text+0x15f8a0b>
1415f9a06: 45 8b 54 b9 04              	mov	r10d, dword ptr [r9 + 4*rdi + 0x4]
1415f9a0b: 49 39 d0                    	cmp	r8, rdx
1415f9a0e: 72 04                       	jb	0x1415f9a14 <.text+0x15f8a14>
1415f9a10: 45 8b 55 6c                 	mov	r10d, dword ptr [r13 + 0x6c]
1415f9a14: 8b 04 b8                    	mov	eax, dword ptr [rax + 4*rdi]
1415f9a17: 41 8b 14 b9                 	mov	edx, dword ptr [r9 + 4*rdi]
1415f9a1b: 89 44 24 34                 	mov	dword ptr [rsp + 0x34], eax
1415f9a1f: 89 4c 24 38                 	mov	dword ptr [rsp + 0x38], ecx
1415f9a23: 89 54 24 3c                 	mov	dword ptr [rsp + 0x3c], edx
1415f9a27: 44 89 54 24 40              	mov	dword ptr [rsp + 0x40], r10d
1415f9a2c: 8d 44 3d 00                 	lea	eax, [rbp + rdi]
1415f9a30: 89 44 24 44                 	mov	dword ptr [rsp + 0x44], eax
1415f9a34: 4c 8d 44 24 34              	lea	r8, [rsp + 0x34]
1415f9a39: 41 b9 14 00 00 00           	mov	r9d, 0x14
1415f9a3f: 48 89 f1                    	mov	rcx, rsi
1415f9a42: 48 8b 54 24 58              	mov	rdx, qword ptr [rsp + 0x58]
1415f9a47: e8 04 e0 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415f9a4c: 4c 8b 26                    	mov	r12, qword ptr [rsi]
1415f9a4f: 4c 8b 7e 08                 	mov	r15, qword ptr [rsi + 0x8]
1415f9a53: 41 80 bf ba 0a 00 00 01     	cmp	byte ptr [r15 + 0xaba], 0x1
1415f9a5b: 75 0d                       	jne	0x1415f9a6a <.text+0x15f8a6a>
1415f9a5d: 49 8b 54 24 48              	mov	rdx, qword ptr [r12 + 0x48]
1415f9a62: 4c 89 f9                    	mov	rcx, r15
1415f9a65: e8 26 46 5f ff              	call	0x140bee090 <.text+0xbed090>
1415f9a6a: f0                          	lock
1415f9a6b: 49 ff 87 a0 0a 00 00        	inc	qword ptr [r15 + 0xaa0]
1415f9a72: 4d 8b 64 24 48              	mov	r12, qword ptr [r12 + 0x48]
1415f9a77: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
1415f9a7b: 8b 54 24 24                 	mov	edx, dword ptr [rsp + 0x24]
1415f9a7f: e8 4c df 5e ff              	call	0x140be79d0 <.text+0xbe69d0>
1415f9a84: 4c 89 f9                    	mov	rcx, r15
1415f9a87: 48 89 c2                    	mov	rdx, rax
1415f9a8a: 45 31 c0                    	xor	r8d, r8d
1415f9a8d: e8 de 59 5f ff              	call	0x140bef470 <.text+0xbee470>
1415f9a92: a8 01                       	test	al, 0x1
1415f9a94: 0f 84 13 02 00 00           	je	0x1415f9cad <.text+0x15f8cad>
1415f9a9a: 48 ff c7                    	inc	rdi
1415f9a9d: 4c 89 e1                    	mov	rcx, r12
1415f9aa0: 4d 89 f0                    	mov	r8, r14
1415f9aa3: 41 ff 97 68 03 00 00        	call	qword ptr [r15 + 0x368]
1415f9aaa: 49 83 c6 10                 	add	r14, 0x10
1415f9aae: 48 3b 7c 24 60              	cmp	rdi, qword ptr [rsp + 0x60]
1415f9ab3: 0f 82 e7 fe ff ff           	jb	0x1415f99a0 <.text+0x15f89a0>
1415f9ab9: 48 8b 7c 24 48              	mov	rdi, qword ptr [rsp + 0x48]
1415f9abe: 8b 4c 24 20                 	mov	ecx, dword ptr [rsp + 0x20]
1415f9ac2: e9 99 fc ff ff              	jmp	0x1415f9760 <.text+0x15f8760>
1415f9ac7: f6 05 ce b2 03 02 01        	test	byte ptr [rip + 0x203b2ce], 0x1 # 0x143634d9c
1415f9ace: 0f 84 8e 00 00 00           	je	0x1415f9b62 <.text+0x15f8b62>
1415f9ad4: 31 c9                       	xor	ecx, ecx
1415f9ad6: 48 8d 05 6b 45 68 01        	lea	rax, [rip + 0x168456b]  # 0x142c7e048
1415f9add: eb 11                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9adf: b1 18                       	mov	cl, 0x18
1415f9ae1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1415f9af0: 0f b6 c9                    	movzx	ecx, cl
1415f9af3: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415f9af7: 48 01 c2                    	add	rdx, rax
1415f9afa: b1 02                       	mov	cl, 0x2
1415f9afc: ff e2                       	jmp	rdx
1415f9afe: b1 01                       	mov	cl, 0x1
1415f9b00: eb ee                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b02: b1 14                       	mov	cl, 0x14
1415f9b04: eb ea                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b06: b1 0f                       	mov	cl, 0xf
1415f9b08: eb e6                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b0a: b1 1a                       	mov	cl, 0x1a
1415f9b0c: eb e2                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b0e: b1 16                       	mov	cl, 0x16
1415f9b10: eb de                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b12: b1 09                       	mov	cl, 0x9
1415f9b14: eb da                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b16: b1 07                       	mov	cl, 0x7
1415f9b18: eb d6                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b1a: b1 03                       	mov	cl, 0x3
1415f9b1c: eb d2                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b1e: b1 08                       	mov	cl, 0x8
1415f9b20: eb ce                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b22: b1 19                       	mov	cl, 0x19
1415f9b24: eb ca                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b26: b1 0a                       	mov	cl, 0xa
1415f9b28: eb c6                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b2a: b1 13                       	mov	cl, 0x13
1415f9b2c: eb c2                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b2e: b1 06                       	mov	cl, 0x6
1415f9b30: eb be                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b32: b1 05                       	mov	cl, 0x5
1415f9b34: eb ba                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b36: b1 04                       	mov	cl, 0x4
1415f9b38: eb b6                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b3a: b1 1b                       	mov	cl, 0x1b
1415f9b3c: eb b2                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b3e: b1 0e                       	mov	cl, 0xe
1415f9b40: eb ae                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b42: b1 15                       	mov	cl, 0x15
1415f9b44: eb aa                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b46: b1 0d                       	mov	cl, 0xd
1415f9b48: eb a6                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b4a: b1 11                       	mov	cl, 0x11
1415f9b4c: eb a2                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b4e: b1 10                       	mov	cl, 0x10
1415f9b50: eb 9e                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b52: b1 12                       	mov	cl, 0x12
1415f9b54: eb 9a                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b56: b1 0b                       	mov	cl, 0xb
1415f9b58: eb 96                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b5a: b1 17                       	mov	cl, 0x17
1415f9b5c: eb 92                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b5e: b1 0c                       	mov	cl, 0xc
1415f9b60: eb 8e                       	jmp	0x1415f9af0 <.text+0x15f8af0>
1415f9b62: f6 05 34 b2 03 02 01        	test	byte ptr [rip + 0x203b234], 0x1 # 0x143634d9d
1415f9b69: 0f 84 83 00 00 00           	je	0x1415f9bf2 <.text+0x15f8bf2>
1415f9b6f: 31 c9                       	xor	ecx, ecx
1415f9b71: 48 8d 05 60 44 68 01        	lea	rax, [rip + 0x1684460]  # 0x142c7dfd8
1415f9b78: eb 06                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9b7a: b1 18                       	mov	cl, 0x18
1415f9b7c: 0f 1f 40 00                 	nop	dword ptr [rax]
1415f9b80: 0f b6 c9                    	movzx	ecx, cl
1415f9b83: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415f9b87: 48 01 c2                    	add	rdx, rax
1415f9b8a: b1 02                       	mov	cl, 0x2
1415f9b8c: ff e2                       	jmp	rdx
1415f9b8e: b1 01                       	mov	cl, 0x1
1415f9b90: eb ee                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9b92: b1 14                       	mov	cl, 0x14
1415f9b94: eb ea                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9b96: b1 0f                       	mov	cl, 0xf
1415f9b98: eb e6                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9b9a: b1 1a                       	mov	cl, 0x1a
1415f9b9c: eb e2                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9b9e: b1 16                       	mov	cl, 0x16
1415f9ba0: eb de                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9ba2: b1 09                       	mov	cl, 0x9
1415f9ba4: eb da                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9ba6: b1 07                       	mov	cl, 0x7
1415f9ba8: eb d6                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9baa: b1 03                       	mov	cl, 0x3
1415f9bac: eb d2                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bae: b1 08                       	mov	cl, 0x8
1415f9bb0: eb ce                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bb2: b1 19                       	mov	cl, 0x19
1415f9bb4: eb ca                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bb6: b1 0a                       	mov	cl, 0xa
1415f9bb8: eb c6                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bba: b1 13                       	mov	cl, 0x13
1415f9bbc: eb c2                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bbe: b1 06                       	mov	cl, 0x6
1415f9bc0: eb be                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bc2: b1 05                       	mov	cl, 0x5
1415f9bc4: eb ba                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bc6: b1 04                       	mov	cl, 0x4
1415f9bc8: eb b6                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bca: b1 1b                       	mov	cl, 0x1b
1415f9bcc: eb b2                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bce: b1 0e                       	mov	cl, 0xe
1415f9bd0: eb ae                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bd2: b1 15                       	mov	cl, 0x15
1415f9bd4: eb aa                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bd6: b1 0d                       	mov	cl, 0xd
1415f9bd8: eb a6                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bda: b1 11                       	mov	cl, 0x11
1415f9bdc: eb a2                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bde: b1 10                       	mov	cl, 0x10
1415f9be0: eb 9e                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9be2: b1 12                       	mov	cl, 0x12
1415f9be4: eb 9a                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9be6: b1 0b                       	mov	cl, 0xb
1415f9be8: eb 96                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bea: b1 17                       	mov	cl, 0x17
1415f9bec: eb 92                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bee: b1 0c                       	mov	cl, 0xc
1415f9bf0: eb 8e                       	jmp	0x1415f9b80 <.text+0x15f8b80>
1415f9bf2: 31 c9                       	xor	ecx, ecx
1415f9bf4: 48 8d 05 6d 43 68 01        	lea	rax, [rip + 0x168436d]  # 0x142c7df68
1415f9bfb: eb 03                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9bfd: b1 18                       	mov	cl, 0x18
1415f9bff: 90                          	nop
1415f9c00: 0f b6 c9                    	movzx	ecx, cl
1415f9c03: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415f9c07: 48 01 c2                    	add	rdx, rax
1415f9c0a: b1 02                       	mov	cl, 0x2
1415f9c0c: ff e2                       	jmp	rdx
1415f9c0e: b1 01                       	mov	cl, 0x1
1415f9c10: eb ee                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c12: b1 14                       	mov	cl, 0x14
1415f9c14: eb ea                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c16: b1 0f                       	mov	cl, 0xf
1415f9c18: eb e6                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c1a: b1 1a                       	mov	cl, 0x1a
1415f9c1c: eb e2                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c1e: b1 16                       	mov	cl, 0x16
1415f9c20: eb de                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c22: b1 09                       	mov	cl, 0x9
1415f9c24: eb da                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c26: b1 07                       	mov	cl, 0x7
1415f9c28: eb d6                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c2a: b1 03                       	mov	cl, 0x3
1415f9c2c: eb d2                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c2e: b1 08                       	mov	cl, 0x8
1415f9c30: eb ce                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c32: b1 19                       	mov	cl, 0x19
1415f9c34: eb ca                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c36: b1 0a                       	mov	cl, 0xa
1415f9c38: eb c6                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c3a: b1 13                       	mov	cl, 0x13
1415f9c3c: eb c2                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c3e: b1 06                       	mov	cl, 0x6
1415f9c40: eb be                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c42: b1 05                       	mov	cl, 0x5
1415f9c44: eb ba                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c46: b1 04                       	mov	cl, 0x4
1415f9c48: eb b6                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c4a: b1 1b                       	mov	cl, 0x1b
1415f9c4c: eb b2                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c4e: b1 0e                       	mov	cl, 0xe
1415f9c50: eb ae                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c52: b1 15                       	mov	cl, 0x15
1415f9c54: eb aa                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c56: b1 0d                       	mov	cl, 0xd
1415f9c58: eb a6                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c5a: b1 11                       	mov	cl, 0x11
1415f9c5c: eb a2                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c5e: b1 10                       	mov	cl, 0x10
1415f9c60: eb 9e                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c62: b1 12                       	mov	cl, 0x12
1415f9c64: eb 9a                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c66: b1 0b                       	mov	cl, 0xb
1415f9c68: eb 96                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c6a: b1 17                       	mov	cl, 0x17
1415f9c6c: eb 92                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c6e: b1 0c                       	mov	cl, 0xc
1415f9c70: eb 8e                       	jmp	0x1415f9c00 <.text+0x15f8c00>
1415f9c72: 48 83 c4 68                 	add	rsp, 0x68
1415f9c76: 5b                          	pop	rbx
1415f9c77: 5d                          	pop	rbp
1415f9c78: 5f                          	pop	rdi
1415f9c79: 5e                          	pop	rsi
1415f9c7a: 41 5c                       	pop	r12
1415f9c7c: 41 5d                       	pop	r13
1415f9c7e: 41 5e                       	pop	r14
1415f9c80: 41 5f                       	pop	r15
1415f9c82: c3                          	ret
1415f9c83: 48 8b 44 24 28              	mov	rax, qword ptr [rsp + 0x28]
1415f9c88: 48 8d 0c 87                 	lea	rcx, [rdi + 4*rax]
1415f9c8c: 4c 8d 05 65 6c 4a 01        	lea	r8, [rip + 0x14a6c65]   # 0x142aa08f8
1415f9c93: e8 80 f8 2d 01              	call	0x1428d9518 <.text+0x28d8518>
1415f9c98: 48 8b 44 24 28              	mov	rax, qword ptr [rsp + 0x28]
1415f9c9d: 48 8d 0c 87                 	lea	rcx, [rdi + 4*rax]
1415f9ca1: 4c 8d 05 68 6c 4a 01        	lea	r8, [rip + 0x14a6c68]   # 0x142aa0910
1415f9ca8: e8 6b f8 2d 01              	call	0x1428d9518 <.text+0x28d8518>
1415f9cad: 48 8d 0d fc c4 4b 01        	lea	rcx, [rip + 0x14bc4fc]  # 0x142ab61b0
1415f9cb4: e8 57 f6 2d 01              	call	0x1428d9310 <.text+0x28d8310>
1415f9cb9: cc                          	int3

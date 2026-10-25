   1409d6b90:	55                   	push   rbp
   1409d6b91:	41 57                	push   r15
   1409d6b93:	41 56                	push   r14
   1409d6b95:	41 55                	push   r13
   1409d6b97:	41 54                	push   r12
   1409d6b99:	56                   	push   rsi
   1409d6b9a:	57                   	push   rdi
   1409d6b9b:	53                   	push   rbx
   1409d6b9c:	48 81 ec b8 01 00 00 	sub    rsp,0x1b8
   1409d6ba3:	48 8d ac 24 80 00 00 	lea    rbp,[rsp+0x80]
   1409d6baa:	00 
   1409d6bab:	44 0f 29 bd 20 01 00 	movaps XMMWORD PTR [rbp+0x120],xmm15
   1409d6bb2:	00 
   1409d6bb3:	44 0f 29 b5 10 01 00 	movaps XMMWORD PTR [rbp+0x110],xmm14
   1409d6bba:	00 
   1409d6bbb:	44 0f 29 ad 00 01 00 	movaps XMMWORD PTR [rbp+0x100],xmm13
   1409d6bc2:	00 
   1409d6bc3:	44 0f 29 a5 f0 00 00 	movaps XMMWORD PTR [rbp+0xf0],xmm12
   1409d6bca:	00 
   1409d6bcb:	44 0f 29 9d e0 00 00 	movaps XMMWORD PTR [rbp+0xe0],xmm11
   1409d6bd2:	00 
   1409d6bd3:	44 0f 29 95 d0 00 00 	movaps XMMWORD PTR [rbp+0xd0],xmm10
   1409d6bda:	00 
   1409d6bdb:	44 0f 29 8d c0 00 00 	movaps XMMWORD PTR [rbp+0xc0],xmm9
   1409d6be2:	00 
   1409d6be3:	44 0f 29 85 b0 00 00 	movaps XMMWORD PTR [rbp+0xb0],xmm8
   1409d6bea:	00 
   1409d6beb:	0f 29 bd a0 00 00 00 	movaps XMMWORD PTR [rbp+0xa0],xmm7
   1409d6bf2:	0f 29 b5 90 00 00 00 	movaps XMMWORD PTR [rbp+0x90],xmm6
   1409d6bf9:	48 c7 85 88 00 00 00 	mov    QWORD PTR [rbp+0x88],0xfffffffffffffffe
   1409d6c00:	fe ff ff ff 
   1409d6c04:	4d 89 ce             	mov    r14,r9
   1409d6c07:	4d 89 c7             	mov    r15,r8
   1409d6c0a:	48 89 d7             	mov    rdi,rdx
   1409d6c0d:	48 89 8d 80 00 00 00 	mov    QWORD PTR [rbp+0x80],rcx
   1409d6c14:	e8 e7 fd 23 00       	call   0x140c16a00
   1409d6c19:	84 c0                	test   al,al
   1409d6c1b:	88 45 7f             	mov    BYTE PTR [rbp+0x7f],al
   1409d6c1e:	4c 89 75 00          	mov    QWORD PTR [rbp+0x0],r14
   1409d6c22:	48 89 7d 08          	mov    QWORD PTR [rbp+0x8],rdi
   1409d6c26:	74 5f                	je     0x1409d6c87
   1409d6c28:	4c 8d 25 a9 b2 0e 02 	lea    r12,[rip+0x20eb2a9]        # 0x142ac1ed8
   1409d6c2f:	be 45 00 00 00       	mov    esi,0x45
   1409d6c34:	ba 45 00 00 00       	mov    edx,0x45
   1409d6c39:	4c 89 e1             	mov    rcx,r12
   1409d6c3c:	e8 8f d6 ff ff       	call   0x1409d42d0
   1409d6c41:	48 83 f8 01          	cmp    rax,0x1
   1409d6c45:	0f 85 b1 00 00 00    	jne    0x1409d6cfc
   1409d6c4b:	49 89 d1             	mov    r9,rdx
   1409d6c4e:	48 85 d2             	test   rdx,rdx
   1409d6c51:	74 3d                	je     0x1409d6c90
   1409d6c53:	49 83 f9 45          	cmp    r9,0x45
   1409d6c57:	73 35                	jae    0x1409d6c8e
   1409d6c59:	48 8d 05 78 b2 0e 02 	lea    rax,[rip+0x20eb278]        # 0x142ac1ed8
   1409d6c60:	41 80 3c 01 bf       	cmp    BYTE PTR [r9+rax*1],0xbf
   1409d6c65:	7f 29                	jg     0x1409d6c90
   1409d6c67:	48 8d 05 92 b3 0e 02 	lea    rax,[rip+0x20eb392]        # 0x142ac2000
   1409d6c6e:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   1409d6c73:	48 8d 0d 5e b2 0e 02 	lea    rcx,[rip+0x20eb25e]        # 0x142ac1ed8
   1409d6c7a:	ba 45 00 00 00       	mov    edx,0x45
   1409d6c7f:	45 31 c0             	xor    r8d,r8d
   1409d6c82:	e8 b9 2c f0 01       	call   0x1428d9940
   1409d6c87:	31 c0                	xor    eax,eax
   1409d6c89:	e9 ca 01 00 00       	jmp    0x1409d6e58
   1409d6c8e:	75 d7                	jne    0x1409d6c67
   1409d6c90:	4c 8d 25 41 b2 0e 02 	lea    r12,[rip+0x20eb241]        # 0x142ac1ed8
   1409d6c97:	4c 89 e1             	mov    rcx,r12
   1409d6c9a:	4c 89 ca             	mov    rdx,r9
   1409d6c9d:	e8 2e d6 ff ff       	call   0x1409d42d0
   1409d6ca2:	48 83 f8 01          	cmp    rax,0x1
   1409d6ca6:	75 54                	jne    0x1409d6cfc
   1409d6ca8:	49 89 d0             	mov    r8,rdx
   1409d6cab:	49 83 c0 02          	add    r8,0x2
   1409d6caf:	74 39                	je     0x1409d6cea
   1409d6cb1:	49 83 f8 45          	cmp    r8,0x45
   1409d6cb5:	73 31                	jae    0x1409d6ce8
   1409d6cb7:	48 8d 05 1a b2 0e 02 	lea    rax,[rip+0x20eb21a]        # 0x142ac1ed8
   1409d6cbe:	41 80 3c 00 bf       	cmp    BYTE PTR [r8+rax*1],0xbf
   1409d6cc3:	7f 25                	jg     0x1409d6cea
   1409d6cc5:	48 8d 05 4c b3 0e 02 	lea    rax,[rip+0x20eb34c]        # 0x142ac2018
   1409d6ccc:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   1409d6cd1:	48 8d 0d 00 b2 0e 02 	lea    rcx,[rip+0x20eb200]        # 0x142ac1ed8
   1409d6cd8:	ba 45 00 00 00       	mov    edx,0x45
   1409d6cdd:	41 b9 45 00 00 00    	mov    r9d,0x45
   1409d6ce3:	e8 58 2c f0 01       	call   0x1428d9940
   1409d6ce8:	75 db                	jne    0x1409d6cc5
   1409d6cea:	be 43 00 00 00       	mov    esi,0x43
   1409d6cef:	48 29 d6             	sub    rsi,rdx
   1409d6cf2:	4c 8d 25 df b1 0e 02 	lea    r12,[rip+0x20eb1df]        # 0x142ac1ed8
   1409d6cf9:	4d 01 c4             	add    r12,r8
   1409d6cfc:	48 8d 05 ab b7 0e 02 	lea    rax,[rip+0x20eb7ab]        # 0x142ac24ae
   1409d6d03:	4c 8d 2d 6c b7 0e 02 	lea    r13,[rip+0x20eb76c]        # 0x142ac2476
   1409d6d0a:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
   1409d6d10:	0f be 48 ff          	movsx  ecx,BYTE PTR [rax-0x1]
   1409d6d14:	85 c9                	test   ecx,ecx
   1409d6d16:	78 18                	js     0x1409d6d30
   1409d6d18:	48 ff c8             	dec    rax
   1409d6d1b:	83 f9 5c             	cmp    ecx,0x5c
   1409d6d1e:	75 69                	jne    0x1409d6d89
   1409d6d20:	eb 7c                	jmp    0x1409d6d9e
   1409d6d22:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   1409d6d29:	1f 84 00 00 00 00 00 
   1409d6d30:	0f b6 50 fe          	movzx  edx,BYTE PTR [rax-0x2]
   1409d6d34:	80 fa c0             	cmp    dl,0xc0
   1409d6d37:	7d 25                	jge    0x1409d6d5e
   1409d6d39:	44 0f b6 40 fd       	movzx  r8d,BYTE PTR [rax-0x3]
   1409d6d3e:	41 80 f8 c0          	cmp    r8b,0xc0
   1409d6d42:	7d 23                	jge    0x1409d6d67
   1409d6d44:	44 0f b6 48 fc       	movzx  r9d,BYTE PTR [rax-0x4]
   1409d6d49:	48 83 c0 fc          	add    rax,0xfffffffffffffffc
   1409d6d4d:	41 83 e1 07          	and    r9d,0x7
   1409d6d51:	41 c1 e1 06          	shl    r9d,0x6
   1409d6d55:	41 83 e0 3f          	and    r8d,0x3f
   1409d6d59:	45 09 c8             	or     r8d,r9d
   1409d6d5c:	eb 11                	jmp    0x1409d6d6f
   1409d6d5e:	48 83 c0 fe          	add    rax,0xfffffffffffffffe
   1409d6d62:	83 e2 1f             	and    edx,0x1f
   1409d6d65:	eb 12                	jmp    0x1409d6d79
   1409d6d67:	48 83 c0 fd          	add    rax,0xfffffffffffffffd
   1409d6d6b:	41 83 e0 0f          	and    r8d,0xf
   1409d6d6f:	41 c1 e0 06          	shl    r8d,0x6
   1409d6d73:	83 e2 3f             	and    edx,0x3f
   1409d6d76:	44 09 c2             	or     edx,r8d
   1409d6d79:	c1 e2 06             	shl    edx,0x6
   1409d6d7c:	80 e1 3f             	and    cl,0x3f
   1409d6d7f:	0f b6 c9             	movzx  ecx,cl
   1409d6d82:	09 d1                	or     ecx,edx
   1409d6d84:	83 f9 5c             	cmp    ecx,0x5c
   1409d6d87:	74 15                	je     0x1409d6d9e
   1409d6d89:	83 f9 2f             	cmp    ecx,0x2f
   1409d6d8c:	74 10                	je     0x1409d6d9e
   1409d6d8e:	4c 39 e8             	cmp    rax,r13
   1409d6d91:	0f 85 79 ff ff ff    	jne    0x1409d6d10
   1409d6d97:	bf 38 00 00 00       	mov    edi,0x38
   1409d6d9c:	eb 4f                	jmp    0x1409d6ded
   1409d6d9e:	4c 8d 2d d1 b6 0e 02 	lea    r13,[rip+0x20eb6d1]        # 0x142ac2476
   1409d6da5:	4c 29 e8             	sub    rax,r13
   1409d6da8:	49 89 c0             	mov    r8,rax
   1409d6dab:	49 ff c0             	inc    r8
   1409d6dae:	74 32                	je     0x1409d6de2
   1409d6db0:	49 83 f8 38          	cmp    r8,0x38
   1409d6db4:	73 2a                	jae    0x1409d6de0
   1409d6db6:	43 80 3c 28 bf       	cmp    BYTE PTR [r8+r13*1],0xbf
   1409d6dbb:	7f 25                	jg     0x1409d6de2
   1409d6dbd:	48 8d 05 1c b2 0e 02 	lea    rax,[rip+0x20eb21c]        # 0x142ac1fe0
   1409d6dc4:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   1409d6dc9:	48 8d 0d a6 b6 0e 02 	lea    rcx,[rip+0x20eb6a6]        # 0x142ac2476
   1409d6dd0:	ba 38 00 00 00       	mov    edx,0x38
   1409d6dd5:	41 b9 38 00 00 00    	mov    r9d,0x38
   1409d6ddb:	e8 60 2b f0 01       	call   0x1428d9940
   1409d6de0:	75 db                	jne    0x1409d6dbd
   1409d6de2:	bf 37 00 00 00       	mov    edi,0x37
   1409d6de7:	48 29 c7             	sub    rdi,rax
   1409d6dea:	4d 01 c5             	add    r13,r8
   1409d6ded:	e8 3e 13 24 00       	call   0x140c18130
   1409d6df2:	48 89 c1             	mov    rcx,rax
   1409d6df5:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d6df8:	48 83 f8 01          	cmp    rax,0x1
   1409d6dfc:	0f 85 4b 10 00 00    	jne    0x1409d7e4d
   1409d6e02:	48 83 c1 08          	add    rcx,0x8
   1409d6e06:	48 83 39 00          	cmp    QWORD PTR [rcx],0x0
   1409d6e0a:	0f 85 1e 15 00 00    	jne    0x1409d832e
   1409d6e10:	48 c7 01 ff ff ff ff 	mov    QWORD PTR [rcx],0xffffffffffffffff
   1409d6e17:	48 89 4d f8          	mov    QWORD PTR [rbp-0x8],rcx
   1409d6e1b:	48 83 c1 08          	add    rcx,0x8
   1409d6e1f:	48 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],rdi
   1409d6e24:	48 c7 44 24 30 00 00 	mov    QWORD PTR [rsp+0x30],0x0
   1409d6e2b:	00 00 
   1409d6e2d:	48 c7 44 24 28 01 00 	mov    QWORD PTR [rsp+0x28],0x1
   1409d6e34:	00 00 
   1409d6e36:	4c 89 e2             	mov    rdx,r12
   1409d6e39:	49 89 f0             	mov    r8,rsi
   1409d6e3c:	4d 89 e9             	mov    r9,r13
   1409d6e3f:	e8 fc fe 23 00       	call   0x140c16d40
   1409d6e44:	48 8b 4d f8          	mov    rcx,QWORD PTR [rbp-0x8]
   1409d6e48:	48 ff 01             	inc    QWORD PTR [rcx]
   1409d6e4b:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
   1409d6e4f:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
   1409d6e53:	b8 01 00 00 00       	mov    eax,0x1
   1409d6e58:	48 89 45 b8          	mov    QWORD PTR [rbp-0x48],rax
   1409d6e5c:	49 8b 07             	mov    rax,QWORD PTR [r15]
   1409d6e5f:	4d 8b 57 08          	mov    r10,QWORD PTR [r15+0x8]
   1409d6e63:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d6e66:	45 31 db             	xor    r11d,r11d
   1409d6e69:	48 89 ca             	mov    rdx,rcx
   1409d6e6c:	49 2b 52 18          	sub    rdx,QWORD PTR [r10+0x18]
   1409d6e70:	49 0f 42 d3          	cmovb  rdx,r11
   1409d6e74:	49 2b 4a 38          	sub    rcx,QWORD PTR [r10+0x38]
   1409d6e78:	49 0f 42 cb          	cmovb  rcx,r11
   1409d6e7c:	49 8b 72 10          	mov    rsi,QWORD PTR [r10+0x10]
   1409d6e80:	4d 8b 4a 30          	mov    r9,QWORD PTR [r10+0x30]
   1409d6e84:	4d 89 c8             	mov    r8,r9
   1409d6e87:	49 29 c8             	sub    r8,rcx
   1409d6e8a:	4d 0f 42 c3          	cmovb  r8,r11
   1409d6e8e:	41 0f 96 c4          	setbe  r12b
   1409d6e92:	48 89 f7             	mov    rdi,rsi
   1409d6e95:	48 29 d7             	sub    rdi,rdx
   1409d6e98:	49 0f 42 fb          	cmovb  rdi,r11
   1409d6e9c:	49 8b 5a 40          	mov    rbx,QWORD PTR [r10+0x40]
   1409d6ea0:	41 0f 96 c5          	setbe  r13b
   1409d6ea4:	49 89 db             	mov    r11,rbx
   1409d6ea7:	4d 29 c3             	sub    r11,r8
   1409d6eaa:	49 29 fb             	sub    r11,rdi
   1409d6ead:	4d 8b 7a 08          	mov    r15,QWORD PTR [r10+0x8]
   1409d6eb1:	4d 8b 72 28          	mov    r14,QWORD PTR [r10+0x28]
   1409d6eb5:	4c 89 18             	mov    QWORD PTR [rax],r11
   1409d6eb8:	4c 8b 95 b8 01 00 00 	mov    r10,QWORD PTR [rbp+0x1b8]
   1409d6ebf:	45 84 e5             	test   r13b,r12b
   1409d6ec2:	4c 8b 65 08          	mov    r12,QWORD PTR [rbp+0x8]
   1409d6ec6:	75 67                	jne    0x1409d6f2f
   1409d6ec8:	49 89 cb             	mov    r11,rcx
   1409d6ecb:	49 c1 e3 04          	shl    r11,0x4
   1409d6ecf:	4d 01 de             	add    r14,r11
   1409d6ed2:	49 c1 e0 04          	shl    r8,0x4
   1409d6ed6:	49 89 d3             	mov    r11,rdx
   1409d6ed9:	49 c1 e3 04          	shl    r11,0x4
   1409d6edd:	4d 01 df             	add    r15,r11
   1409d6ee0:	48 c1 e7 04          	shl    rdi,0x4
   1409d6ee4:	45 31 db             	xor    r11d,r11d
   1409d6ee7:	48 39 d6             	cmp    rsi,rdx
   1409d6eea:	ba 08 00 00 00       	mov    edx,0x8
   1409d6eef:	4c 0f 42 fa          	cmovb  r15,rdx
   1409d6ef3:	49 8d 74 3f f0       	lea    rsi,[r15+rdi*1-0x10]
   1409d6ef8:	4c 0f 47 de          	cmova  r11,rsi
   1409d6efc:	49 39 c9             	cmp    r9,rcx
   1409d6eff:	4c 0f 42 f2          	cmovb  r14,rdx
   1409d6f03:	4b 8d 4c 06 f0       	lea    rcx,[r14+r8*1-0x10]
   1409d6f08:	49 0f 46 cb          	cmovbe rcx,r11
   1409d6f0c:	48 89 18             	mov    QWORD PTR [rax],rbx
   1409d6f0f:	48 8b 41 08          	mov    rax,QWORD PTR [rcx+0x8]
   1409d6f13:	41 8b 4c 24 1c       	mov    ecx,DWORD PTR [r12+0x1c]
   1409d6f18:	49 8b 54 24 10       	mov    rdx,QWORD PTR [r12+0x10]
   1409d6f1d:	89 0a                	mov    DWORD PTR [rdx],ecx
   1409d6f1f:	49 8b 0c 24          	mov    rcx,QWORD PTR [r12]
   1409d6f23:	48 c7 41 30 01 00 00 	mov    QWORD PTR [rcx+0x30],0x1
   1409d6f2a:	00 
   1409d6f2b:	48 89 41 38          	mov    QWORD PTR [rcx+0x38],rax
   1409d6f2f:	49 8b 02             	mov    rax,QWORD PTR [r10]
   1409d6f32:	49 8b 4a 08          	mov    rcx,QWORD PTR [r10+0x8]
   1409d6f36:	4c 8b 00             	mov    r8,QWORD PTR [rax]
   1409d6f39:	31 d2                	xor    edx,edx
   1409d6f3b:	4d 89 c2             	mov    r10,r8
   1409d6f3e:	4c 2b 51 18          	sub    r10,QWORD PTR [rcx+0x18]
   1409d6f42:	4c 0f 42 d2          	cmovb  r10,rdx
   1409d6f46:	4c 89 d6             	mov    rsi,r10
   1409d6f49:	48 c1 e6 05          	shl    rsi,0x5
   1409d6f4d:	48 03 71 08          	add    rsi,QWORD PTR [rcx+0x8]
   1409d6f51:	48 8b 79 10          	mov    rdi,QWORD PTR [rcx+0x10]
   1409d6f55:	4c 8b 59 30          	mov    r11,QWORD PTR [rcx+0x30]
   1409d6f59:	49 89 f9             	mov    r9,rdi
   1409d6f5c:	4d 29 d1             	sub    r9,r10
   1409d6f5f:	4c 0f 42 ca          	cmovb  r9,rdx
   1409d6f63:	4c 89 cb             	mov    rbx,r9
   1409d6f66:	48 c1 e3 05          	shl    rbx,0x5
   1409d6f6a:	4c 39 d7             	cmp    rdi,r10
   1409d6f6d:	bf 08 00 00 00       	mov    edi,0x8
   1409d6f72:	48 0f 42 f7          	cmovb  rsi,rdi
   1409d6f76:	41 0f 96 c2          	setbe  r10b
   1409d6f7a:	48 8d 5c 1e e0       	lea    rbx,[rsi+rbx*1-0x20]
   1409d6f7f:	48 0f 46 da          	cmovbe rbx,rdx
   1409d6f83:	4c 2b 41 38          	sub    r8,QWORD PTR [rcx+0x38]
   1409d6f87:	4c 0f 42 c2          	cmovb  r8,rdx
   1409d6f8b:	4c 89 c6             	mov    rsi,r8
   1409d6f8e:	48 c1 e6 05          	shl    rsi,0x5
   1409d6f92:	48 03 71 28          	add    rsi,QWORD PTR [rcx+0x28]
   1409d6f96:	4d 89 de             	mov    r14,r11
   1409d6f99:	4d 29 c6             	sub    r14,r8
   1409d6f9c:	4c 0f 42 f2          	cmovb  r14,rdx
   1409d6fa0:	4d 89 f7             	mov    r15,r14
   1409d6fa3:	49 c1 e7 05          	shl    r15,0x5
   1409d6fa7:	4d 39 c3             	cmp    r11,r8
   1409d6faa:	48 0f 42 f7          	cmovb  rsi,rdi
   1409d6fae:	41 0f 96 c0          	setbe  r8b
   1409d6fb2:	4a 8d 74 3e e0       	lea    rsi,[rsi+r15*1-0x20]
   1409d6fb7:	48 0f 46 f3          	cmovbe rsi,rbx
   1409d6fbb:	4d 01 ce             	add    r14,r9
   1409d6fbe:	45 84 c2             	test   r10b,r8b
   1409d6fc1:	4c 0f 44 f2          	cmove  r14,rdx
   1409d6fc5:	48 8b 49 40          	mov    rcx,QWORD PTR [rcx+0x40]
   1409d6fc9:	4c 29 f1             	sub    rcx,r14
   1409d6fcc:	48 89 08             	mov    QWORD PTR [rax],rcx
   1409d6fcf:	45 84 c2             	test   r10b,r8b
   1409d6fd2:	0f 84 90 0b 00 00    	je     0x1409d7b68
   1409d6fd8:	41 8b 44 24 1c       	mov    eax,DWORD PTR [r12+0x1c]
   1409d6fdd:	49 8b 4c 24 10       	mov    rcx,QWORD PTR [r12+0x10]
   1409d6fe2:	89 01                	mov    DWORD PTR [rcx],eax
   1409d6fe4:	49 8b 04 24          	mov    rax,QWORD PTR [r12]
   1409d6fe8:	48 89 45 08          	mov    QWORD PTR [rbp+0x8],rax
   1409d6fec:	48 8d 88 d0 00 00 00 	lea    rcx,[rax+0xd0]
   1409d6ff3:	e8 38 24 08 00       	call   0x140a59430
   1409d6ff8:	48 8b 85 c0 01 00 00 	mov    rax,QWORD PTR [rbp+0x1c0]
   1409d6fff:	48 8b 18             	mov    rbx,QWORD PTR [rax]
   1409d7002:	48 8b 50 08          	mov    rdx,QWORD PTR [rax+0x8]
   1409d7006:	4c 8b 0b             	mov    r9,QWORD PTR [rbx]
   1409d7009:	45 31 c0             	xor    r8d,r8d
   1409d700c:	4c 89 c9             	mov    rcx,r9
   1409d700f:	48 2b 4a 18          	sub    rcx,QWORD PTR [rdx+0x18]
   1409d7013:	49 0f 42 c8          	cmovb  rcx,r8
   1409d7017:	4c 2b 4a 38          	sub    r9,QWORD PTR [rdx+0x38]
   1409d701b:	4d 0f 42 c8          	cmovb  r9,r8
   1409d701f:	48 89 c8             	mov    rax,rcx
   1409d7022:	48 c1 e0 04          	shl    rax,0x4
   1409d7026:	48 03 42 08          	add    rax,QWORD PTR [rdx+0x8]
   1409d702a:	4c 8b 72 10          	mov    r14,QWORD PTR [rdx+0x10]
   1409d702e:	48 8b 7a 30          	mov    rdi,QWORD PTR [rdx+0x30]
   1409d7032:	49 29 ce             	sub    r14,rcx
   1409d7035:	4d 0f 42 f0          	cmovb  r14,r8
   1409d7039:	41 ba 08 00 00 00    	mov    r10d,0x8
   1409d703f:	49 0f 42 c2          	cmovb  rax,r10
   1409d7043:	4c 89 c9             	mov    rcx,r9
   1409d7046:	48 c1 e1 04          	shl    rcx,0x4
   1409d704a:	48 03 4a 28          	add    rcx,QWORD PTR [rdx+0x28]
   1409d704e:	4c 29 cf             	sub    rdi,r9
   1409d7051:	49 0f 42 f8          	cmovb  rdi,r8
   1409d7055:	49 0f 42 ca          	cmovb  rcx,r10
   1409d7059:	48 8b 52 40          	mov    rdx,QWORD PTR [rdx+0x40]
   1409d705d:	48 29 fa             	sub    rdx,rdi
   1409d7060:	4c 29 f2             	sub    rdx,r14
   1409d7063:	48 89 13             	mov    QWORD PTR [rbx],rdx
   1409d7066:	49 c1 e6 04          	shl    r14,0x4
   1409d706a:	49 01 c6             	add    r14,rax
   1409d706d:	48 c1 e7 04          	shl    rdi,0x4
   1409d7071:	48 01 cf             	add    rdi,rcx
   1409d7074:	4c 8d 7d 10          	lea    r15,[rbp+0x10]
   1409d7078:	4c 8d 6d 48          	lea    r13,[rbp+0x48]
   1409d707c:	eb 08                	jmp    0x1409d7086
   1409d707e:	66 90                	xchg   ax,ax
   1409d7080:	48 89 f0             	mov    rax,rsi
   1409d7083:	4c 89 e1             	mov    rcx,r12
   1409d7086:	48 85 c0             	test   rax,rax
   1409d7089:	74 55                	je     0x1409d70e0
   1409d708b:	4c 39 f0             	cmp    rax,r14
   1409d708e:	74 50                	je     0x1409d70e0
   1409d7090:	48 8d 70 10          	lea    rsi,[rax+0x10]
   1409d7094:	49 89 cc             	mov    r12,rcx
   1409d7097:	48 ff 03             	inc    QWORD PTR [rbx]
   1409d709a:	80 78 08 00          	cmp    BYTE PTR [rax+0x8],0x0
   1409d709e:	74 64                	je     0x1409d7104
   1409d70a0:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d70a7:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d70aa:	48 8b 10             	mov    rdx,QWORD PTR [rax]
   1409d70ad:	48 85 d2             	test   rdx,rdx
   1409d70b0:	0f 84 8d 00 00 00    	je     0x1409d7143
   1409d70b6:	48 83 c2 10          	add    rdx,0x10
   1409d70ba:	41 b9 1f 00 00 00    	mov    r9d,0x1f
   1409d70c0:	4c 89 e9             	mov    rcx,r13
   1409d70c3:	4c 8d 05 98 b8 0e 02 	lea    r8,[rip+0x20eb898]        # 0x142ac2962
   1409d70ca:	e8 71 d3 07 00       	call   0x140a54440
   1409d70cf:	48 83 7d 48 02       	cmp    QWORD PTR [rbp+0x48],0x2
   1409d70d4:	0f 85 bf 00 00 00    	jne    0x1409d7199
   1409d70da:	e9 95 12 00 00       	jmp    0x1409d8374
   1409d70df:	90                   	nop
   1409d70e0:	48 85 c9             	test   rcx,rcx
   1409d70e3:	0f 84 e4 01 00 00    	je     0x1409d72cd
   1409d70e9:	48 39 f9             	cmp    rcx,rdi
   1409d70ec:	0f 84 db 01 00 00    	je     0x1409d72cd
   1409d70f2:	4c 8d 61 10          	lea    r12,[rcx+0x10]
   1409d70f6:	31 f6                	xor    esi,esi
   1409d70f8:	48 89 c8             	mov    rax,rcx
   1409d70fb:	48 ff 03             	inc    QWORD PTR [rbx]
   1409d70fe:	80 78 08 00          	cmp    BYTE PTR [rax+0x8],0x0
   1409d7102:	75 9c                	jne    0x1409d70a0
   1409d7104:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d710b:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d710e:	48 8b 10             	mov    rdx,QWORD PTR [rax]
   1409d7111:	48 85 d2             	test   rdx,rdx
   1409d7114:	0f 84 e0 00 00 00    	je     0x1409d71fa
   1409d711a:	48 83 c2 10          	add    rdx,0x10
   1409d711e:	41 b9 1e 00 00 00    	mov    r9d,0x1e
   1409d7124:	4c 89 e9             	mov    rcx,r13
   1409d7127:	4c 8d 05 16 b8 0e 02 	lea    r8,[rip+0x20eb816]        # 0x142ac2944
   1409d712e:	e8 0d d3 07 00       	call   0x140a54440
   1409d7133:	48 83 7d 48 02       	cmp    QWORD PTR [rbp+0x48],0x2
   1409d7138:	0f 85 12 01 00 00    	jne    0x1409d7250
   1409d713e:	e9 66 12 00 00       	jmp    0x1409d83a9
   1409d7143:	0f b6 05 bf da c5 02 	movzx  eax,BYTE PTR [rip+0x2c5dabf]        # 0x143634c09
   1409d714a:	b9 1f 00 00 00       	mov    ecx,0x1f
   1409d714f:	ba 01 00 00 00       	mov    edx,0x1
   1409d7154:	e8 b7 ca c3 ff       	call   0x140613c10
   1409d7159:	48 85 c0             	test   rax,rax
   1409d715c:	0f 84 91 12 00 00    	je     0x1409d83f3
   1409d7162:	0f 10 05 08 b8 0e 02 	movups xmm0,XMMWORD PTR [rip+0x20eb808]        # 0x142ac2971
   1409d7169:	0f 11 40 0f          	movups XMMWORD PTR [rax+0xf],xmm0
   1409d716d:	f3 0f 6f 05 ed b7 0e 	movdqu xmm0,XMMWORD PTR [rip+0x20eb7ed]        # 0x142ac2962
   1409d7174:	02 
   1409d7175:	f3 0f 7f 00          	movdqu XMMWORD PTR [rax],xmm0
   1409d7179:	48 c7 45 48 00 00 00 	mov    QWORD PTR [rbp+0x48],0x0
   1409d7180:	00 
   1409d7181:	48 c7 45 58 1f 00 00 	mov    QWORD PTR [rbp+0x58],0x1f
   1409d7188:	00 
   1409d7189:	48 89 45 60          	mov    QWORD PTR [rbp+0x60],rax
   1409d718d:	48 c7 45 68 1f 00 00 	mov    QWORD PTR [rbp+0x68],0x1f
   1409d7194:	00 
   1409d7195:	c6 45 70 02          	mov    BYTE PTR [rbp+0x70],0x2
   1409d7199:	48 8b 45 48          	mov    rax,QWORD PTR [rbp+0x48]
   1409d719d:	48 8b 4d 50          	mov    rcx,QWORD PTR [rbp+0x50]
   1409d71a1:	48 8b 55 58          	mov    rdx,QWORD PTR [rbp+0x58]
   1409d71a5:	4c 8b 45 60          	mov    r8,QWORD PTR [rbp+0x60]
   1409d71a9:	4c 8b 4d 68          	mov    r9,QWORD PTR [rbp+0x68]
   1409d71ad:	4c 89 4d 30          	mov    QWORD PTR [rbp+0x30],r9
   1409d71b1:	44 0f b6 4d 70       	movzx  r9d,BYTE PTR [rbp+0x70]
   1409d71b6:	44 88 4d 38          	mov    BYTE PTR [rbp+0x38],r9b
   1409d71ba:	44 8b 4d 71          	mov    r9d,DWORD PTR [rbp+0x71]
   1409d71be:	44 89 4d 39          	mov    DWORD PTR [rbp+0x39],r9d
   1409d71c2:	44 0f b7 4d 75       	movzx  r9d,WORD PTR [rbp+0x75]
   1409d71c7:	66 44 89 4d 3d       	mov    WORD PTR [rbp+0x3d],r9w
   1409d71cc:	44 0f b6 4d 77       	movzx  r9d,BYTE PTR [rbp+0x77]
   1409d71d1:	44 88 4d 3f          	mov    BYTE PTR [rbp+0x3f],r9b
   1409d71d5:	48 89 55 20          	mov    QWORD PTR [rbp+0x20],rdx
   1409d71d9:	4c 89 45 28          	mov    QWORD PTR [rbp+0x28],r8
   1409d71dd:	48 89 45 10          	mov    QWORD PTR [rbp+0x10],rax
   1409d71e1:	48 89 4d 18          	mov    QWORD PTR [rbp+0x18],rcx
   1409d71e5:	4c 89 f9             	mov    rcx,r15
   1409d71e8:	e8 13 01 08 00       	call   0x140a57300
   1409d71ed:	4c 89 f9             	mov    rcx,r15
   1409d71f0:	e8 8b 30 ec ff       	call   0x14089a280
   1409d71f5:	e9 b2 00 00 00       	jmp    0x1409d72ac
   1409d71fa:	0f b6 05 08 da c5 02 	movzx  eax,BYTE PTR [rip+0x2c5da08]        # 0x143634c09
   1409d7201:	b9 1e 00 00 00       	mov    ecx,0x1e
   1409d7206:	ba 01 00 00 00       	mov    edx,0x1
   1409d720b:	e8 00 ca c3 ff       	call   0x140613c10
   1409d7210:	48 85 c0             	test   rax,rax
   1409d7213:	0f 84 c2 11 00 00    	je     0x1409d83db
   1409d7219:	0f 10 05 32 b7 0e 02 	movups xmm0,XMMWORD PTR [rip+0x20eb732]        # 0x142ac2952
   1409d7220:	0f 11 40 0e          	movups XMMWORD PTR [rax+0xe],xmm0
   1409d7224:	f3 0f 6f 05 18 b7 0e 	movdqu xmm0,XMMWORD PTR [rip+0x20eb718]        # 0x142ac2944
   1409d722b:	02 
   1409d722c:	f3 0f 7f 00          	movdqu XMMWORD PTR [rax],xmm0
   1409d7230:	48 c7 45 48 00 00 00 	mov    QWORD PTR [rbp+0x48],0x0
   1409d7237:	00 
   1409d7238:	48 c7 45 58 1e 00 00 	mov    QWORD PTR [rbp+0x58],0x1e
   1409d723f:	00 
   1409d7240:	48 89 45 60          	mov    QWORD PTR [rbp+0x60],rax
   1409d7244:	48 c7 45 68 1e 00 00 	mov    QWORD PTR [rbp+0x68],0x1e
   1409d724b:	00 
   1409d724c:	c6 45 70 02          	mov    BYTE PTR [rbp+0x70],0x2
   1409d7250:	48 8b 45 48          	mov    rax,QWORD PTR [rbp+0x48]
   1409d7254:	48 8b 4d 50          	mov    rcx,QWORD PTR [rbp+0x50]
   1409d7258:	48 8b 55 58          	mov    rdx,QWORD PTR [rbp+0x58]
   1409d725c:	4c 8b 45 60          	mov    r8,QWORD PTR [rbp+0x60]
   1409d7260:	4c 8b 4d 68          	mov    r9,QWORD PTR [rbp+0x68]
   1409d7264:	4c 89 4d 30          	mov    QWORD PTR [rbp+0x30],r9
   1409d7268:	44 0f b6 4d 70       	movzx  r9d,BYTE PTR [rbp+0x70]
   1409d726d:	44 88 4d 38          	mov    BYTE PTR [rbp+0x38],r9b
   1409d7271:	44 8b 4d 71          	mov    r9d,DWORD PTR [rbp+0x71]
   1409d7275:	44 89 4d 39          	mov    DWORD PTR [rbp+0x39],r9d
   1409d7279:	44 0f b7 4d 75       	movzx  r9d,WORD PTR [rbp+0x75]
   1409d727e:	66 44 89 4d 3d       	mov    WORD PTR [rbp+0x3d],r9w
   1409d7283:	44 0f b6 4d 77       	movzx  r9d,BYTE PTR [rbp+0x77]
   1409d7288:	44 88 4d 3f          	mov    BYTE PTR [rbp+0x3f],r9b
   1409d728c:	48 89 55 20          	mov    QWORD PTR [rbp+0x20],rdx
   1409d7290:	4c 89 45 28          	mov    QWORD PTR [rbp+0x28],r8
   1409d7294:	48 89 45 10          	mov    QWORD PTR [rbp+0x10],rax
   1409d7298:	48 89 4d 18          	mov    QWORD PTR [rbp+0x18],rcx
   1409d729c:	4c 89 f9             	mov    rcx,r15
   1409d729f:	e8 5c 00 08 00       	call   0x140a57300
   1409d72a4:	4c 89 f9             	mov    rcx,r15
   1409d72a7:	e8 d4 2f ec ff       	call   0x14089a280
   1409d72ac:	48 8b 55 20          	mov    rdx,QWORD PTR [rbp+0x20]
   1409d72b0:	48 85 d2             	test   rdx,rdx
   1409d72b3:	0f 84 c7 fd ff ff    	je     0x1409d7080
   1409d72b9:	48 8b 4d 28          	mov    rcx,QWORD PTR [rbp+0x28]
   1409d72bd:	41 b8 01 00 00 00    	mov    r8d,0x1
   1409d72c3:	e8 58 c9 c3 ff       	call   0x140613c20
   1409d72c8:	e9 b3 fd ff ff       	jmp    0x1409d7080
   1409d72cd:	48 8b 45 00          	mov    rax,QWORD PTR [rbp+0x0]
   1409d72d1:	48 8b 30             	mov    rsi,QWORD PTR [rax]
   1409d72d4:	48 8b 40 08          	mov    rax,QWORD PTR [rax+0x8]
   1409d72d8:	48 8b 0e             	mov    rcx,QWORD PTR [rsi]
   1409d72db:	45 31 d2             	xor    r10d,r10d
   1409d72de:	48 89 ca             	mov    rdx,rcx
   1409d72e1:	48 2b 50 18          	sub    rdx,QWORD PTR [rax+0x18]
   1409d72e5:	49 0f 42 d2          	cmovb  rdx,r10
   1409d72e9:	48 2b 48 38          	sub    rcx,QWORD PTR [rax+0x38]
   1409d72ed:	49 0f 42 ca          	cmovb  rcx,r10
   1409d72f1:	4c 8d 34 92          	lea    r14,[rdx+rdx*4]
   1409d72f5:	49 c1 e6 03          	shl    r14,0x3
   1409d72f9:	4c 03 70 08          	add    r14,QWORD PTR [rax+0x8]
   1409d72fd:	4c 8b 40 10          	mov    r8,QWORD PTR [rax+0x10]
   1409d7301:	49 29 d0             	sub    r8,rdx
   1409d7304:	4d 0f 42 c2          	cmovb  r8,r10
   1409d7308:	ba 08 00 00 00       	mov    edx,0x8
   1409d730d:	4c 0f 42 f2          	cmovb  r14,rdx
   1409d7311:	4c 8d 24 89          	lea    r12,[rcx+rcx*4]
   1409d7315:	49 c1 e4 03          	shl    r12,0x3
   1409d7319:	4c 03 60 28          	add    r12,QWORD PTR [rax+0x28]
   1409d731d:	4c 8b 48 30          	mov    r9,QWORD PTR [rax+0x30]
   1409d7321:	49 29 c9             	sub    r9,rcx
   1409d7324:	4d 0f 42 ca          	cmovb  r9,r10
   1409d7328:	4c 0f 42 e2          	cmovb  r12,rdx
   1409d732c:	48 8b 40 40          	mov    rax,QWORD PTR [rax+0x40]
   1409d7330:	4c 29 c8             	sub    rax,r9
   1409d7333:	4c 29 c0             	sub    rax,r8
   1409d7336:	48 89 06             	mov    QWORD PTR [rsi],rax
   1409d7339:	4b 8d 04 80          	lea    rax,[r8+r8*4]
   1409d733d:	49 8d 1c c6          	lea    rbx,[r14+rax*8]
   1409d7341:	4b 8d 04 89          	lea    rax,[r9+r9*4]
   1409d7345:	4d 8d 3c c4          	lea    r15,[r12+rax*8]
   1409d7349:	48 8b 85 c8 01 00 00 	mov    rax,QWORD PTR [rbp+0x1c8]
   1409d7350:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7353:	48 89 45 00          	mov    QWORD PTR [rbp+0x0],rax
   1409d7357:	48 83 c0 20          	add    rax,0x20
   1409d735b:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
   1409d735f:	48 8d 3d 12 ba 0e 02 	lea    rdi,[rip+0x20eba12]        # 0x142ac2d78
   1409d7366:	f3 0f 10 3d 92 6a 0a 	movss  xmm7,DWORD PTR [rip+0x20a6a92]        # 0x142a7de00
   1409d736d:	02 
   1409d736e:	f3 44 0f 10 05 01 5d 	movss  xmm8,DWORD PTR [rip+0x1ff5d01]        # 0x1429cd078
   1409d7375:	ff 01 
   1409d7377:	f3 44 0f 10 0d 84 e5 	movss  xmm9,DWORD PTR [rip+0x1f4e584]        # 0x142925904
   1409d737e:	f4 01 
   1409d7380:	f3 44 0f 10 15 67 73 	movss  xmm10,DWORD PTR [rip+0x1f57367]        # 0x14292e6f0
   1409d7387:	f5 01 
   1409d7389:	f3 44 0f 10 25 fe d2 	movss  xmm12,DWORD PTR [rip+0x1f5d2fe]        # 0x142934690
   1409d7390:	f5 01 
   1409d7392:	f3 44 0f 10 2d f9 d2 	movss  xmm13,DWORD PTR [rip+0x1f5d2f9]        # 0x142934694
   1409d7399:	f5 01 
   1409d739b:	f3 44 0f 10 35 a4 59 	movss  xmm14,DWORD PTR [rip+0x20a59a4]        # 0x142a7cd48
   1409d73a2:	0a 02 
   1409d73a4:	f3 44 0f 10 3d 8b a5 	movss  xmm15,DWORD PTR [rip+0x20ea58b]        # 0x142ac1938
   1409d73ab:	0e 02 
   1409d73ad:	45 31 ed             	xor    r13d,r13d
   1409d73b0:	f3 44 0f 10 1d 4b d2 	movss  xmm11,DWORD PTR [rip+0x1f5d24b]        # 0x142934604
   1409d73b7:	f5 01 
   1409d73b9:	f3 0f 10 35 73 a5 0e 	movss  xmm6,DWORD PTR [rip+0x20ea573]        # 0x142ac1934
   1409d73c0:	02 
   1409d73c1:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   1409d73c8:	0f 1f 84 00 00 00 00 
   1409d73cf:	00 
   1409d73d0:	4d 85 f6             	test   r14,r14
   1409d73d3:	74 1b                	je     0x1409d73f0
   1409d73d5:	49 39 de             	cmp    r14,rbx
   1409d73d8:	74 16                	je     0x1409d73f0
   1409d73da:	49 8d 4e 28          	lea    rcx,[r14+0x28]
   1409d73de:	4c 89 f0             	mov    rax,r14
   1409d73e1:	eb 28                	jmp    0x1409d740b
   1409d73e3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   1409d73ea:	84 00 00 00 00 00 
   1409d73f0:	4d 85 e4             	test   r12,r12
   1409d73f3:	0f 84 f5 06 00 00    	je     0x1409d7aee
   1409d73f9:	4d 39 fc             	cmp    r12,r15
   1409d73fc:	0f 84 ec 06 00 00    	je     0x1409d7aee
   1409d7402:	4c 89 e0             	mov    rax,r12
   1409d7405:	49 83 c4 28          	add    r12,0x28
   1409d7409:	31 c9                	xor    ecx,ecx
   1409d740b:	48 ff 06             	inc    QWORD PTR [rsi]
   1409d740e:	49 89 ce             	mov    r14,rcx
   1409d7411:	8b 08                	mov    ecx,DWORD PTR [rax]
   1409d7413:	83 f9 0d             	cmp    ecx,0xd
   1409d7416:	74 b8                	je     0x1409d73d0
   1409d7418:	83 78 10 0d          	cmp    DWORD PTR [rax+0x10],0xd
   1409d741c:	74 b2                	je     0x1409d73d0
   1409d741e:	8d 51 fa             	lea    edx,[rcx-0x6]
   1409d7421:	4c 8d 42 01          	lea    r8,[rdx+0x1]
   1409d7425:	83 fa 0f             	cmp    edx,0xf
   1409d7428:	ba 00 00 00 00       	mov    edx,0x0
   1409d742d:	4c 0f 43 c2          	cmovae r8,rdx
   1409d7431:	41 b5 01             	mov    r13b,0x1
   1409d7434:	4a 63 14 87          	movsxd rdx,DWORD PTR [rdi+r8*4]
   1409d7438:	48 01 fa             	add    rdx,rdi
   1409d743b:	ff e2                	jmp    rdx
   1409d743d:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d7444:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7447:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d744a:	48 85 c9             	test   rcx,rcx
   1409d744d:	74 81                	je     0x1409d73d0
   1409d744f:	48 83 c1 10          	add    rcx,0x10
   1409d7453:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d7459:	48 8d 15 e0 b4 0e 02 	lea    rdx,[rip+0x20eb4e0]        # 0x142ac2940
   1409d7460:	41 0f 28 d9          	movaps xmm3,xmm9
   1409d7464:	e8 47 c9 07 00       	call   0x140a53db0
   1409d7469:	e9 62 ff ff ff       	jmp    0x1409d73d0
   1409d746e:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d7475:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7478:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d747b:	48 85 c9             	test   rcx,rcx
   1409d747e:	0f 84 4c ff ff ff    	je     0x1409d73d0
   1409d7484:	48 83 c1 10          	add    rcx,0x10
   1409d7488:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d748e:	48 8d 15 ab b4 0e 02 	lea    rdx,[rip+0x20eb4ab]        # 0x142ac2940
   1409d7495:	41 0f 28 dd          	movaps xmm3,xmm13
   1409d7499:	e8 12 c9 07 00       	call   0x140a53db0
   1409d749e:	e9 2d ff ff ff       	jmp    0x1409d73d0
   1409d74a3:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d74aa:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d74ad:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d74b0:	48 85 c9             	test   rcx,rcx
   1409d74b3:	0f 84 17 ff ff ff    	je     0x1409d73d0
   1409d74b9:	48 83 c1 10          	add    rcx,0x10
   1409d74bd:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d74c3:	48 8d 15 76 b4 0e 02 	lea    rdx,[rip+0x20eb476]        # 0x142ac2940
   1409d74ca:	0f 28 de             	movaps xmm3,xmm6
   1409d74cd:	e8 de c8 07 00       	call   0x140a53db0
   1409d74d2:	e9 f9 fe ff ff       	jmp    0x1409d73d0
   1409d74d7:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d74de:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d74e1:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d74e4:	48 85 c9             	test   rcx,rcx
   1409d74e7:	0f 84 e3 fe ff ff    	je     0x1409d73d0
   1409d74ed:	48 83 c1 10          	add    rcx,0x10
   1409d74f1:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d74f7:	48 8d 15 42 b4 0e 02 	lea    rdx,[rip+0x20eb442]        # 0x142ac2940
   1409d74fe:	41 0f 28 df          	movaps xmm3,xmm15
   1409d7502:	e8 a9 c8 07 00       	call   0x140a53db0
   1409d7507:	e9 c4 fe ff ff       	jmp    0x1409d73d0
   1409d750c:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d7513:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7516:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d7519:	48 85 c9             	test   rcx,rcx
   1409d751c:	0f 84 ae fe ff ff    	je     0x1409d73d0
   1409d7522:	48 83 c1 10          	add    rcx,0x10
   1409d7526:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d752c:	48 8d 15 0d b4 0e 02 	lea    rdx,[rip+0x20eb40d]        # 0x142ac2940
   1409d7533:	41 0f 28 db          	movaps xmm3,xmm11
   1409d7537:	e8 74 c8 07 00       	call   0x140a53db0
   1409d753c:	e9 8f fe ff ff       	jmp    0x1409d73d0
   1409d7541:	80 78 04 00          	cmp    BYTE PTR [rax+0x4],0x0
   1409d7545:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d754c:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d754f:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d7552:	0f 84 79 02 00 00    	je     0x1409d77d1
   1409d7558:	48 85 c9             	test   rcx,rcx
   1409d755b:	0f 84 6f fe ff ff    	je     0x1409d73d0
   1409d7561:	48 83 c1 10          	add    rcx,0x10
   1409d7565:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d756b:	0f 57 db             	xorps  xmm3,xmm3
   1409d756e:	48 8d 15 cb b3 0e 02 	lea    rdx,[rip+0x20eb3cb]        # 0x142ac2940
   1409d7575:	e8 36 c8 07 00       	call   0x140a53db0
   1409d757a:	e9 51 fe ff ff       	jmp    0x1409d73d0
   1409d757f:	83 f9 05             	cmp    ecx,0x5
   1409d7582:	0f 85 67 01 00 00    	jne    0x1409d76ef
   1409d7588:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d758f:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7592:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d7595:	48 85 c9             	test   rcx,rcx
   1409d7598:	0f 84 32 fe ff ff    	je     0x1409d73d0
   1409d759e:	48 83 c1 10          	add    rcx,0x10
   1409d75a2:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d75a8:	48 8d 15 91 b3 0e 02 	lea    rdx,[rip+0x20eb391]        # 0x142ac2940
   1409d75af:	41 0f 28 d8          	movaps xmm3,xmm8
   1409d75b3:	e8 f8 c7 07 00       	call   0x140a53db0
   1409d75b8:	e9 13 fe ff ff       	jmp    0x1409d73d0
   1409d75bd:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d75c4:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d75c7:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d75ca:	48 85 c9             	test   rcx,rcx
   1409d75cd:	0f 84 fd fd ff ff    	je     0x1409d73d0
   1409d75d3:	48 83 c1 10          	add    rcx,0x10
   1409d75d7:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d75dd:	48 8d 15 5c b3 0e 02 	lea    rdx,[rip+0x20eb35c]        # 0x142ac2940
   1409d75e4:	41 0f 28 dc          	movaps xmm3,xmm12
   1409d75e8:	e8 c3 c7 07 00       	call   0x140a53db0
   1409d75ed:	e9 de fd ff ff       	jmp    0x1409d73d0
   1409d75f2:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d75f9:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d75fc:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d75ff:	48 85 c9             	test   rcx,rcx
   1409d7602:	0f 84 c8 fd ff ff    	je     0x1409d73d0
   1409d7608:	48 83 c1 10          	add    rcx,0x10
   1409d760c:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d7612:	48 8d 15 27 b3 0e 02 	lea    rdx,[rip+0x20eb327]        # 0x142ac2940
   1409d7619:	0f 28 df             	movaps xmm3,xmm7
   1409d761c:	e8 8f c7 07 00       	call   0x140a53db0
   1409d7621:	e9 aa fd ff ff       	jmp    0x1409d73d0
   1409d7626:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d762d:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7630:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d7633:	48 85 c9             	test   rcx,rcx
   1409d7636:	0f 84 94 fd ff ff    	je     0x1409d73d0
   1409d763c:	48 83 c1 10          	add    rcx,0x10
   1409d7640:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d7646:	48 8d 15 f3 b2 0e 02 	lea    rdx,[rip+0x20eb2f3]        # 0x142ac2940
   1409d764d:	41 0f 28 da          	movaps xmm3,xmm10
   1409d7651:	e8 5a c7 07 00       	call   0x140a53db0
   1409d7656:	e9 75 fd ff ff       	jmp    0x1409d73d0
   1409d765b:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d7662:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7665:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d7668:	48 85 c9             	test   rcx,rcx
   1409d766b:	0f 84 5f fd ff ff    	je     0x1409d73d0
   1409d7671:	48 83 c1 10          	add    rcx,0x10
   1409d7675:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d767b:	48 8d 15 be b2 0e 02 	lea    rdx,[rip+0x20eb2be]        # 0x142ac2940
   1409d7682:	41 0f 28 de          	movaps xmm3,xmm14
   1409d7686:	e8 25 c7 07 00       	call   0x140a53db0
   1409d768b:	e9 40 fd ff ff       	jmp    0x1409d73d0
   1409d7690:	0f b6 40 04          	movzx  eax,BYTE PTR [rax+0x4]
   1409d7694:	04 c8                	add    al,0xc8
   1409d7696:	3c 08                	cmp    al,0x8
   1409d7698:	0f b6 c0             	movzx  eax,al
   1409d769b:	b9 06 00 00 00       	mov    ecx,0x6
   1409d76a0:	0f 43 c1             	cmovae eax,ecx
   1409d76a3:	0f b6 c0             	movzx  eax,al
   1409d76a6:	48 8d 0d 0b b7 0e 02 	lea    rcx,[rip+0x20eb70b]        # 0x142ac2db8
   1409d76ad:	48 63 04 81          	movsxd rax,DWORD PTR [rcx+rax*4]
   1409d76b1:	48 01 c8             	add    rax,rcx
   1409d76b4:	ff e0                	jmp    rax
   1409d76b6:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d76bd:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d76c0:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d76c3:	48 85 c9             	test   rcx,rcx
   1409d76c6:	0f 84 04 fd ff ff    	je     0x1409d73d0
   1409d76cc:	48 83 c1 10          	add    rcx,0x10
   1409d76d0:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d76d6:	48 8d 15 63 b2 0e 02 	lea    rdx,[rip+0x20eb263]        # 0x142ac2940
   1409d76dd:	f3 0f 10 1d a3 e2 f4 	movss  xmm3,DWORD PTR [rip+0x1f4e2a3]        # 0x142925988
   1409d76e4:	01 
   1409d76e5:	e8 c6 c6 07 00       	call   0x140a53db0
   1409d76ea:	e9 e1 fc ff ff       	jmp    0x1409d73d0
   1409d76ef:	83 f9 01             	cmp    ecx,0x1
   1409d76f2:	0f 85 05 01 00 00    	jne    0x1409d77fd
   1409d76f8:	48 8b 40 08          	mov    rax,QWORD PTR [rax+0x8]
   1409d76fc:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
   1409d7700:	48 89 45 48          	mov    QWORD PTR [rbp+0x48],rax
   1409d7704:	48 8b 45 00          	mov    rax,QWORD PTR [rbp+0x0]
   1409d7708:	48 83 78 18 00       	cmp    QWORD PTR [rax+0x18],0x0
   1409d770d:	0f 84 1e 01 00 00    	je     0x1409d7831
   1409d7713:	48 8b 4d f8          	mov    rcx,QWORD PTR [rbp-0x8]
   1409d7717:	48 8d 55 48          	lea    rdx,[rbp+0x48]
   1409d771b:	e8 50 96 bd ff       	call   0x1405b0d70
   1409d7720:	48 8b 55 00          	mov    rdx,QWORD PTR [rbp+0x0]
   1409d7724:	48 8b 0a             	mov    rcx,QWORD PTR [rdx]
   1409d7727:	48 8b 52 08          	mov    rdx,QWORD PTR [rdx+0x8]
   1409d772b:	49 89 d0             	mov    r8,rdx
   1409d772e:	49 21 c0             	and    r8,rax
   1409d7731:	48 c1 e8 39          	shr    rax,0x39
   1409d7735:	66 0f 6e c0          	movd   xmm0,eax
   1409d7739:	66 0f 60 c0          	punpcklbw xmm0,xmm0
   1409d773d:	f2 0f 70 c0 00       	pshuflw xmm0,xmm0,0x0
   1409d7742:	66 0f 70 c0 00       	pshufd xmm0,xmm0,0x0
   1409d7747:	48 8d 81 58 fd ff ff 	lea    rax,[rcx-0x2a8]
   1409d774e:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
   1409d7752:	45 31 c9             	xor    r9d,r9d
   1409d7755:	f3 42 0f 6f 0c 01    	movdqu xmm1,XMMWORD PTR [rcx+r8*1]
   1409d775b:	66 0f 6f d1          	movdqa xmm2,xmm1
   1409d775f:	66 0f 74 d0          	pcmpeqb xmm2,xmm0
   1409d7763:	66 44 0f d7 d2       	pmovmskb r10d,xmm2
   1409d7768:	45 85 d2             	test   r10d,r10d
   1409d776b:	74 42                	je     0x1409d77af
   1409d776d:	4c 89 4d e8          	mov    QWORD PTR [rbp-0x18],r9
   1409d7771:	f3 45 0f bc da       	tzcnt  r11d,r10d
   1409d7776:	4d 01 c3             	add    r11,r8
   1409d7779:	49 21 d3             	and    r11,rdx
   1409d777c:	49 f7 db             	neg    r11
   1409d777f:	4d 69 db a8 02 00 00 	imul   r11,r11,0x2a8
   1409d7786:	4c 8b 4d f0          	mov    r9,QWORD PTR [rbp-0x10]
   1409d778a:	48 89 d0             	mov    rax,rdx
   1409d778d:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
   1409d7791:	4e 39 0c 1a          	cmp    QWORD PTR [rdx+r11*1],r9
   1409d7795:	0f 84 83 01 00 00    	je     0x1409d791e
   1409d779b:	45 8d 5a ff          	lea    r11d,[r10-0x1]
   1409d779f:	66 45 21 d3          	and    r11w,r10w
   1409d77a3:	45 89 da             	mov    r10d,r11d
   1409d77a6:	4c 8b 4d e8          	mov    r9,QWORD PTR [rbp-0x18]
   1409d77aa:	48 89 c2             	mov    rdx,rax
   1409d77ad:	75 c2                	jne    0x1409d7771
   1409d77af:	66 0f 74 0d d9 e1 f4 	pcmpeqb xmm1,XMMWORD PTR [rip+0x1f4e1d9]        # 0x142925990
   1409d77b6:	01 
   1409d77b7:	66 44 0f d7 d1       	pmovmskb r10d,xmm1
   1409d77bc:	45 85 d2             	test   r10d,r10d
   1409d77bf:	75 70                	jne    0x1409d7831
   1409d77c1:	4d 01 c8             	add    r8,r9
   1409d77c4:	49 83 c0 10          	add    r8,0x10
   1409d77c8:	49 83 c1 10          	add    r9,0x10
   1409d77cc:	49 21 d0             	and    r8,rdx
   1409d77cf:	eb 84                	jmp    0x1409d7755
   1409d77d1:	48 85 c9             	test   rcx,rcx
   1409d77d4:	0f 84 f6 fb ff ff    	je     0x1409d73d0
   1409d77da:	48 83 c1 10          	add    rcx,0x10
   1409d77de:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d77e4:	48 8d 15 55 b1 0e 02 	lea    rdx,[rip+0x20eb155]        # 0x142ac2940
   1409d77eb:	f3 0f 10 1d 79 58 ff 	movss  xmm3,DWORD PTR [rip+0x1ff5879]        # 0x1429cd06c
   1409d77f2:	01 
   1409d77f3:	e8 b8 c5 07 00       	call   0x140a53db0
   1409d77f8:	e9 d3 fb ff ff       	jmp    0x1409d73d0
   1409d77fd:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d7804:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7807:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d780a:	48 85 c9             	test   rcx,rcx
   1409d780d:	0f 84 bd fb ff ff    	je     0x1409d73d0
   1409d7813:	48 83 c1 10          	add    rcx,0x10
   1409d7817:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d781d:	0f 57 db             	xorps  xmm3,xmm3
   1409d7820:	48 8d 15 19 b1 0e 02 	lea    rdx,[rip+0x20eb119]        # 0x142ac2940
   1409d7827:	e8 84 c5 07 00       	call   0x140a53db0
   1409d782c:	e9 9f fb ff ff       	jmp    0x1409d73d0
   1409d7831:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
   1409d7835:	e9 4b 01 00 00       	jmp    0x1409d7985
   1409d783a:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d7841:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7844:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d7847:	48 85 c9             	test   rcx,rcx
   1409d784a:	0f 84 80 fb ff ff    	je     0x1409d73d0
   1409d7850:	48 83 c1 10          	add    rcx,0x10
   1409d7854:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d785a:	48 8d 15 df b0 0e 02 	lea    rdx,[rip+0x20eb0df]        # 0x142ac2940
   1409d7861:	f3 0f 10 1d d3 a0 0e 	movss  xmm3,DWORD PTR [rip+0x20ea0d3]        # 0x142ac193c
   1409d7868:	02 
   1409d7869:	e8 42 c5 07 00       	call   0x140a53db0
   1409d786e:	e9 5d fb ff ff       	jmp    0x1409d73d0
   1409d7873:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d787a:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d787d:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d7880:	48 85 c9             	test   rcx,rcx
   1409d7883:	0f 84 47 fb ff ff    	je     0x1409d73d0
   1409d7889:	48 83 c1 10          	add    rcx,0x10
   1409d788d:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d7893:	48 8d 15 a6 b0 0e 02 	lea    rdx,[rip+0x20eb0a6]        # 0x142ac2940
   1409d789a:	f3 0f 10 1d e2 41 0a 	movss  xmm3,DWORD PTR [rip+0x20a41e2]        # 0x142a7ba84
   1409d78a1:	02 
   1409d78a2:	e8 09 c5 07 00       	call   0x140a53db0
   1409d78a7:	e9 24 fb ff ff       	jmp    0x1409d73d0
   1409d78ac:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d78b3:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d78b6:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d78b9:	48 85 c9             	test   rcx,rcx
   1409d78bc:	0f 84 0e fb ff ff    	je     0x1409d73d0
   1409d78c2:	48 83 c1 10          	add    rcx,0x10
   1409d78c6:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d78cc:	48 8d 15 6d b0 0e 02 	lea    rdx,[rip+0x20eb06d]        # 0x142ac2940
   1409d78d3:	f3 0f 10 1d cd cc f5 	movss  xmm3,DWORD PTR [rip+0x1f5cccd]        # 0x1429345a8
   1409d78da:	01 
   1409d78db:	e8 d0 c4 07 00       	call   0x140a53db0
   1409d78e0:	e9 eb fa ff ff       	jmp    0x1409d73d0
   1409d78e5:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d78ec:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d78ef:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d78f2:	48 85 c9             	test   rcx,rcx
   1409d78f5:	0f 84 d5 fa ff ff    	je     0x1409d73d0
   1409d78fb:	48 83 c1 10          	add    rcx,0x10
   1409d78ff:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d7905:	48 8d 15 34 b0 0e 02 	lea    rdx,[rip+0x20eb034]        # 0x142ac2940
   1409d790c:	f3 0f 10 1d c0 59 ff 	movss  xmm3,DWORD PTR [rip+0x1ff59c0]        # 0x1429cd2d4
   1409d7913:	01 
   1409d7914:	e8 97 c4 07 00       	call   0x140a53db0
   1409d7919:	e9 b2 fa ff ff       	jmp    0x1409d73d0
   1409d791e:	4a 83 bc 19 70 fd ff 	cmp    QWORD PTR [rcx+r11*1-0x290],0x2
   1409d7925:	ff 02 
   1409d7927:	0f 82 0d 0a 00 00    	jb     0x1409d833a
   1409d792d:	f3 42 0f 10 84 19 90 	movss  xmm0,DWORD PTR [rcx+r11*1-0x270]
   1409d7934:	fd ff ff 
   1409d7937:	0f 2e 05 52 8d f8 01 	ucomiss xmm0,DWORD PTR [rip+0x1f88d52]        # 0x142960690
   1409d793e:	0f 86 f6 09 00 00    	jbe    0x1409d833a
   1409d7944:	4c 01 d9             	add    rcx,r11
   1409d7947:	80 79 f6 00          	cmp    BYTE PTR [rcx-0xa],0x0
   1409d794b:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
   1409d794f:	74 34                	je     0x1409d7985
   1409d7951:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d7958:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d795b:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d795e:	48 85 c9             	test   rcx,rcx
   1409d7961:	0f 84 69 fa ff ff    	je     0x1409d73d0
   1409d7967:	48 83 c1 10          	add    rcx,0x10
   1409d796b:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d7971:	48 8d 15 c8 af 0e 02 	lea    rdx,[rip+0x20eafc8]        # 0x142ac2940
   1409d7978:	0f 28 df             	movaps xmm3,xmm7
   1409d797b:	e8 30 c4 07 00       	call   0x140a53db0
   1409d7980:	e9 4b fa ff ff       	jmp    0x1409d73d0
   1409d7985:	48 89 45 48          	mov    QWORD PTR [rbp+0x48],rax
   1409d7989:	48 8b 45 00          	mov    rax,QWORD PTR [rbp+0x0]
   1409d798d:	48 83 78 18 00       	cmp    QWORD PTR [rax+0x18],0x0
   1409d7992:	0f 84 1d 01 00 00    	je     0x1409d7ab5
   1409d7998:	48 8b 4d f8          	mov    rcx,QWORD PTR [rbp-0x8]
   1409d799c:	48 8d 55 48          	lea    rdx,[rbp+0x48]
   1409d79a0:	e8 cb 93 bd ff       	call   0x1405b0d70
   1409d79a5:	48 8b 55 00          	mov    rdx,QWORD PTR [rbp+0x0]
   1409d79a9:	48 8b 0a             	mov    rcx,QWORD PTR [rdx]
   1409d79ac:	48 8b 52 08          	mov    rdx,QWORD PTR [rdx+0x8]
   1409d79b0:	49 89 d0             	mov    r8,rdx
   1409d79b3:	49 21 c0             	and    r8,rax
   1409d79b6:	48 c1 e8 39          	shr    rax,0x39
   1409d79ba:	66 0f 6e c0          	movd   xmm0,eax
   1409d79be:	66 0f 60 c0          	punpcklbw xmm0,xmm0
   1409d79c2:	f2 0f 70 c0 00       	pshuflw xmm0,xmm0,0x0
   1409d79c7:	66 0f 70 c0 00       	pshufd xmm0,xmm0,0x0
   1409d79cc:	48 8d 81 58 fd ff ff 	lea    rax,[rcx-0x2a8]
   1409d79d3:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
   1409d79d7:	45 31 c9             	xor    r9d,r9d
   1409d79da:	f3 42 0f 6f 0c 01    	movdqu xmm1,XMMWORD PTR [rcx+r8*1]
   1409d79e0:	66 0f 6f d1          	movdqa xmm2,xmm1
   1409d79e4:	66 0f 74 d0          	pcmpeqb xmm2,xmm0
   1409d79e8:	66 44 0f d7 d2       	pmovmskb r10d,xmm2
   1409d79ed:	45 85 d2             	test   r10d,r10d
   1409d79f0:	74 3e                	je     0x1409d7a30
   1409d79f2:	4c 89 4d e8          	mov    QWORD PTR [rbp-0x18],r9
   1409d79f6:	f3 45 0f bc da       	tzcnt  r11d,r10d
   1409d79fb:	4d 01 c3             	add    r11,r8
   1409d79fe:	49 21 d3             	and    r11,rdx
   1409d7a01:	49 f7 db             	neg    r11
   1409d7a04:	4d 69 db a8 02 00 00 	imul   r11,r11,0x2a8
   1409d7a0b:	4c 8b 4d f0          	mov    r9,QWORD PTR [rbp-0x10]
   1409d7a0f:	48 89 d0             	mov    rax,rdx
   1409d7a12:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
   1409d7a16:	4e 39 0c 1a          	cmp    QWORD PTR [rdx+r11*1],r9
   1409d7a1a:	74 36                	je     0x1409d7a52
   1409d7a1c:	45 8d 5a ff          	lea    r11d,[r10-0x1]
   1409d7a20:	66 45 21 d3          	and    r11w,r10w
   1409d7a24:	45 89 da             	mov    r10d,r11d
   1409d7a27:	4c 8b 4d e8          	mov    r9,QWORD PTR [rbp-0x18]
   1409d7a2b:	48 89 c2             	mov    rdx,rax
   1409d7a2e:	75 c6                	jne    0x1409d79f6
   1409d7a30:	66 0f 74 0d 58 df f4 	pcmpeqb xmm1,XMMWORD PTR [rip+0x1f4df58]        # 0x142925990
   1409d7a37:	01 
   1409d7a38:	66 44 0f d7 d1       	pmovmskb r10d,xmm1
   1409d7a3d:	45 85 d2             	test   r10d,r10d
   1409d7a40:	75 73                	jne    0x1409d7ab5
   1409d7a42:	4d 01 c8             	add    r8,r9
   1409d7a45:	49 83 c0 10          	add    r8,0x10
   1409d7a49:	49 83 c1 10          	add    r9,0x10
   1409d7a4d:	49 21 d0             	and    r8,rdx
   1409d7a50:	eb 88                	jmp    0x1409d79da
   1409d7a52:	4a 83 bc 19 70 fd ff 	cmp    QWORD PTR [rcx+r11*1-0x290],0x2
   1409d7a59:	ff 02 
   1409d7a5b:	0f 82 f6 08 00 00    	jb     0x1409d8357
   1409d7a61:	f3 42 0f 10 84 19 90 	movss  xmm0,DWORD PTR [rcx+r11*1-0x270]
   1409d7a68:	fd ff ff 
   1409d7a6b:	0f 2e 05 1e 8c f8 01 	ucomiss xmm0,DWORD PTR [rip+0x1f88c1e]        # 0x142960690
   1409d7a72:	0f 86 df 08 00 00    	jbe    0x1409d8357
   1409d7a78:	4c 01 d9             	add    rcx,r11
   1409d7a7b:	83 79 b8 02          	cmp    DWORD PTR [rcx-0x48],0x2
   1409d7a7f:	73 34                	jae    0x1409d7ab5
   1409d7a81:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d7a88:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7a8b:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d7a8e:	48 85 c9             	test   rcx,rcx
   1409d7a91:	0f 84 39 f9 ff ff    	je     0x1409d73d0
   1409d7a97:	48 83 c1 10          	add    rcx,0x10
   1409d7a9b:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d7aa1:	0f 57 db             	xorps  xmm3,xmm3
   1409d7aa4:	48 8d 15 95 ae 0e 02 	lea    rdx,[rip+0x20eae95]        # 0x142ac2940
   1409d7aab:	e8 00 c3 07 00       	call   0x140a53db0
   1409d7ab0:	e9 1b f9 ff ff       	jmp    0x1409d73d0
   1409d7ab5:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d7abc:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7abf:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d7ac2:	48 85 c9             	test   rcx,rcx
   1409d7ac5:	0f 84 05 f9 ff ff    	je     0x1409d73d0
   1409d7acb:	48 83 c1 10          	add    rcx,0x10
   1409d7acf:	41 b8 04 00 00 00    	mov    r8d,0x4
   1409d7ad5:	48 8d 15 64 ae 0e 02 	lea    rdx,[rip+0x20eae64]        # 0x142ac2940
   1409d7adc:	f3 0f 10 1d 88 55 ff 	movss  xmm3,DWORD PTR [rip+0x1ff5588]        # 0x1429cd06c
   1409d7ae3:	01 
   1409d7ae4:	e8 c7 c2 07 00       	call   0x140a53db0
   1409d7ae9:	e9 e2 f8 ff ff       	jmp    0x1409d73d0
   1409d7aee:	41 f6 c5 01          	test   r13b,0x1
   1409d7af2:	0f 84 7e 01 00 00    	je     0x1409d7c76
   1409d7af8:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d7aff:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7b02:	48 8b 10             	mov    rdx,QWORD PTR [rax]
   1409d7b05:	48 85 d2             	test   rdx,rdx
   1409d7b08:	0f 84 99 00 00 00    	je     0x1409d7ba7
   1409d7b0e:	48 83 c2 10          	add    rdx,0x10
   1409d7b12:	4c 8d 05 df ad 0e 02 	lea    r8,[rip+0x20eaddf]        # 0x142ac28f8
   1409d7b19:	48 8d 4d 48          	lea    rcx,[rbp+0x48]
   1409d7b1d:	41 b9 18 00 00 00    	mov    r9d,0x18
   1409d7b23:	e8 18 c9 07 00       	call   0x140a54440
   1409d7b28:	48 83 7d 48 02       	cmp    QWORD PTR [rbp+0x48],0x2
   1409d7b2d:	0f 85 cd 00 00 00    	jne    0x1409d7c00
   1409d7b33:	48 8b 45 50          	mov    rax,QWORD PTR [rbp+0x50]
   1409d7b37:	48 89 45 10          	mov    QWORD PTR [rbp+0x10],rax
   1409d7b3b:	48 8d 05 46 ae 0e 02 	lea    rax,[rip+0x20eae46]        # 0x142ac2988
   1409d7b42:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   1409d7b47:	48 8d 0d 2a a3 0e 02 	lea    rcx,[rip+0x20ea32a]        # 0x142ac1e78
   1409d7b4e:	4c 8d 0d 03 a3 0e 02 	lea    r9,[rip+0x20ea303]        # 0x142ac1e58
   1409d7b55:	4c 8d 45 10          	lea    r8,[rbp+0x10]
   1409d7b59:	ba 2b 00 00 00       	mov    edx,0x2b
   1409d7b5e:	e8 fd 1b f0 01       	call   0x1428d9760
   1409d7b63:	e9 d5 08 00 00       	jmp    0x1409d843d
   1409d7b68:	8b 06                	mov    eax,DWORD PTR [rsi]
   1409d7b6a:	48 8d 0d 77 b1 0e 02 	lea    rcx,[rip+0x20eb177]        # 0x142ac2ce8
   1409d7b71:	48 63 04 81          	movsxd rax,DWORD PTR [rcx+rax*4]
   1409d7b75:	48 01 c8             	add    rax,rcx
   1409d7b78:	ff e0                	jmp    rax
   1409d7b7a:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d7b81:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7b84:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d7b87:	48 85 c9             	test   rcx,rcx
   1409d7b8a:	0f 84 90 07 00 00    	je     0x1409d8320
   1409d7b90:	48 8b 46 08          	mov    rax,QWORD PTR [rsi+0x8]
   1409d7b94:	48 85 c0             	test   rax,rax
   1409d7b97:	0f 88 58 07 00 00    	js     0x1409d82f5
   1409d7b9d:	f3 48 0f 2a d8       	cvtsi2ss xmm3,rax
   1409d7ba2:	e9 63 07 00 00       	jmp    0x1409d830a
   1409d7ba7:	0f b6 05 5b d0 c5 02 	movzx  eax,BYTE PTR [rip+0x2c5d05b]        # 0x143634c09
   1409d7bae:	b9 18 00 00 00       	mov    ecx,0x18
   1409d7bb3:	ba 01 00 00 00       	mov    edx,0x1
   1409d7bb8:	e8 53 c0 c3 ff       	call   0x140613c10
   1409d7bbd:	48 85 c0             	test   rax,rax
   1409d7bc0:	0f 84 45 08 00 00    	je     0x1409d840b
   1409d7bc6:	f3 0f 6f 05 2a ad 0e 	movdqu xmm0,XMMWORD PTR [rip+0x20ead2a]        # 0x142ac28f8
   1409d7bcd:	02 
   1409d7bce:	f3 0f 7f 00          	movdqu XMMWORD PTR [rax],xmm0
   1409d7bd2:	48 b9 6c 20 43 68 61 	movabs rcx,0x65676e616843206c
   1409d7bd9:	6e 67 65 
   1409d7bdc:	48 89 48 10          	mov    QWORD PTR [rax+0x10],rcx
   1409d7be0:	48 c7 45 48 00 00 00 	mov    QWORD PTR [rbp+0x48],0x0
   1409d7be7:	00 
   1409d7be8:	48 c7 45 58 18 00 00 	mov    QWORD PTR [rbp+0x58],0x18
   1409d7bef:	00 
   1409d7bf0:	48 89 45 60          	mov    QWORD PTR [rbp+0x60],rax
   1409d7bf4:	48 c7 45 68 18 00 00 	mov    QWORD PTR [rbp+0x68],0x18
   1409d7bfb:	00 
   1409d7bfc:	c6 45 70 02          	mov    BYTE PTR [rbp+0x70],0x2
   1409d7c00:	48 8b 45 48          	mov    rax,QWORD PTR [rbp+0x48]
   1409d7c04:	48 8b 4d 50          	mov    rcx,QWORD PTR [rbp+0x50]
   1409d7c08:	48 8b 55 58          	mov    rdx,QWORD PTR [rbp+0x58]
   1409d7c0c:	4c 8b 45 60          	mov    r8,QWORD PTR [rbp+0x60]
   1409d7c10:	4c 8b 4d 68          	mov    r9,QWORD PTR [rbp+0x68]
   1409d7c14:	4c 89 4d 30          	mov    QWORD PTR [rbp+0x30],r9
   1409d7c18:	44 0f b6 4d 70       	movzx  r9d,BYTE PTR [rbp+0x70]
   1409d7c1d:	44 88 4d 38          	mov    BYTE PTR [rbp+0x38],r9b
   1409d7c21:	44 8b 4d 71          	mov    r9d,DWORD PTR [rbp+0x71]
   1409d7c25:	44 89 4d 39          	mov    DWORD PTR [rbp+0x39],r9d
   1409d7c29:	44 0f b7 4d 75       	movzx  r9d,WORD PTR [rbp+0x75]
   1409d7c2e:	66 44 89 4d 3d       	mov    WORD PTR [rbp+0x3d],r9w
   1409d7c33:	44 0f b6 4d 77       	movzx  r9d,BYTE PTR [rbp+0x77]
   1409d7c38:	44 88 4d 3f          	mov    BYTE PTR [rbp+0x3f],r9b
   1409d7c3c:	48 89 55 20          	mov    QWORD PTR [rbp+0x20],rdx
   1409d7c40:	4c 89 45 28          	mov    QWORD PTR [rbp+0x28],r8
   1409d7c44:	48 89 45 10          	mov    QWORD PTR [rbp+0x10],rax
   1409d7c48:	48 89 4d 18          	mov    QWORD PTR [rbp+0x18],rcx
   1409d7c4c:	48 8d 4d 10          	lea    rcx,[rbp+0x10]
   1409d7c50:	e8 ab f6 07 00       	call   0x140a57300
   1409d7c55:	48 8d 4d 10          	lea    rcx,[rbp+0x10]
   1409d7c59:	e8 22 26 ec ff       	call   0x14089a280
   1409d7c5e:	48 8b 55 20          	mov    rdx,QWORD PTR [rbp+0x20]
   1409d7c62:	48 85 d2             	test   rdx,rdx
   1409d7c65:	74 0f                	je     0x1409d7c76
   1409d7c67:	48 8b 4d 28          	mov    rcx,QWORD PTR [rbp+0x28]
   1409d7c6b:	41 b8 01 00 00 00    	mov    r8d,0x1
   1409d7c71:	e8 aa bf c3 ff       	call   0x140613c20
   1409d7c76:	48 8b 85 a0 01 00 00 	mov    rax,QWORD PTR [rbp+0x1a0]
   1409d7c7d:	48 8b 08             	mov    rcx,QWORD PTR [rax]
   1409d7c80:	48 8b 50 08          	mov    rdx,QWORD PTR [rax+0x8]
   1409d7c84:	4c 8b 01             	mov    r8,QWORD PTR [rcx]
   1409d7c87:	31 c0                	xor    eax,eax
   1409d7c89:	4d 89 c1             	mov    r9,r8
   1409d7c8c:	4c 2b 4a 18          	sub    r9,QWORD PTR [rdx+0x18]
   1409d7c90:	4c 0f 42 c8          	cmovb  r9,rax
   1409d7c94:	4c 2b 42 38          	sub    r8,QWORD PTR [rdx+0x38]
   1409d7c98:	4c 0f 42 c0          	cmovb  r8,rax
   1409d7c9c:	4c 8b 52 10          	mov    r10,QWORD PTR [rdx+0x10]
   1409d7ca0:	4c 8b 5a 30          	mov    r11,QWORD PTR [rdx+0x30]
   1409d7ca4:	4c 89 d6             	mov    rsi,r10
   1409d7ca7:	4c 29 ce             	sub    rsi,r9
   1409d7caa:	48 0f 42 f0          	cmovb  rsi,rax
   1409d7cae:	4c 89 df             	mov    rdi,r11
   1409d7cb1:	4c 29 c7             	sub    rdi,r8
   1409d7cb4:	48 0f 42 f8          	cmovb  rdi,rax
   1409d7cb8:	48 01 f7             	add    rdi,rsi
   1409d7cbb:	4d 39 c3             	cmp    r11,r8
   1409d7cbe:	48 0f 47 f8          	cmova  rdi,rax
   1409d7cc2:	4d 39 ca             	cmp    r10,r9
   1409d7cc5:	48 0f 47 f8          	cmova  rdi,rax
   1409d7cc9:	48 8b 52 40          	mov    rdx,QWORD PTR [rdx+0x40]
   1409d7ccd:	48 29 fa             	sub    rdx,rdi
   1409d7cd0:	48 89 11             	mov    QWORD PTR [rcx],rdx
   1409d7cd3:	48 8b 8d a8 01 00 00 	mov    rcx,QWORD PTR [rbp+0x1a8]
   1409d7cda:	4c 8b 01             	mov    r8,QWORD PTR [rcx]
   1409d7cdd:	48 8b 51 08          	mov    rdx,QWORD PTR [rcx+0x8]
   1409d7ce1:	49 8b 08             	mov    rcx,QWORD PTR [r8]
   1409d7ce4:	49 89 c9             	mov    r9,rcx
   1409d7ce7:	4c 2b 4a 18          	sub    r9,QWORD PTR [rdx+0x18]
   1409d7ceb:	4c 0f 42 c8          	cmovb  r9,rax
   1409d7cef:	48 2b 4a 38          	sub    rcx,QWORD PTR [rdx+0x38]
   1409d7cf3:	48 0f 42 c8          	cmovb  rcx,rax
   1409d7cf7:	4c 8b 52 10          	mov    r10,QWORD PTR [rdx+0x10]
   1409d7cfb:	4c 8b 5a 30          	mov    r11,QWORD PTR [rdx+0x30]
   1409d7cff:	49 29 cb             	sub    r11,rcx
   1409d7d02:	4c 0f 42 d8          	cmovb  r11,rax
   1409d7d06:	0f 96 c1             	setbe  cl
   1409d7d09:	4d 29 ca             	sub    r10,r9
   1409d7d0c:	4c 0f 42 d0          	cmovb  r10,rax
   1409d7d10:	4c 8b 4a 40          	mov    r9,QWORD PTR [rdx+0x40]
   1409d7d14:	0f 96 c2             	setbe  dl
   1409d7d17:	4d 01 da             	add    r10,r11
   1409d7d1a:	84 ca                	test   dl,cl
   1409d7d1c:	4c 0f 44 d0          	cmove  r10,rax
   1409d7d20:	4d 29 d1             	sub    r9,r10
   1409d7d23:	4d 89 08             	mov    QWORD PTR [r8],r9
   1409d7d26:	4c 8b 8d b0 01 00 00 	mov    r9,QWORD PTR [rbp+0x1b0]
   1409d7d2d:	4d 8b 01             	mov    r8,QWORD PTR [r9]
   1409d7d30:	4d 8b 49 08          	mov    r9,QWORD PTR [r9+0x8]
   1409d7d34:	4d 8b 10             	mov    r10,QWORD PTR [r8]
   1409d7d37:	4d 89 d3             	mov    r11,r10
   1409d7d3a:	4d 2b 59 18          	sub    r11,QWORD PTR [r9+0x18]
   1409d7d3e:	4c 0f 42 d8          	cmovb  r11,rax
   1409d7d42:	4d 2b 51 38          	sub    r10,QWORD PTR [r9+0x38]
   1409d7d46:	4c 0f 42 d0          	cmovb  r10,rax
   1409d7d4a:	49 8b 71 30          	mov    rsi,QWORD PTR [r9+0x30]
   1409d7d4e:	4c 29 d6             	sub    rsi,r10
   1409d7d51:	48 0f 42 f0          	cmovb  rsi,rax
   1409d7d55:	4d 8b 51 10          	mov    r10,QWORD PTR [r9+0x10]
   1409d7d59:	40 0f 96 c7          	setbe  dil
   1409d7d5d:	4d 29 da             	sub    r10,r11
   1409d7d60:	4c 0f 42 d0          	cmovb  r10,rax
   1409d7d64:	0f 96 c3             	setbe  bl
   1409d7d67:	49 01 f2             	add    r10,rsi
   1409d7d6a:	40 20 fb             	and    bl,dil
   1409d7d6d:	4c 0f 44 d0          	cmove  r10,rax
   1409d7d71:	49 8b 41 40          	mov    rax,QWORD PTR [r9+0x40]
   1409d7d75:	4c 29 d0             	sub    rax,r10
   1409d7d78:	49 89 00             	mov    QWORD PTR [r8],rax
   1409d7d7b:	84 ca                	test   dl,cl
   1409d7d7d:	75 0d                	jne    0x1409d7d8c
   1409d7d7f:	48 8b 45 08          	mov    rax,QWORD PTR [rbp+0x8]
   1409d7d83:	48 8d 48 40          	lea    rcx,[rax+0x40]
   1409d7d87:	e8 74 f5 07 00       	call   0x140a57300
   1409d7d8c:	84 db                	test   bl,bl
   1409d7d8e:	75 0d                	jne    0x1409d7d9d
   1409d7d90:	48 8b 4d 08          	mov    rcx,QWORD PTR [rbp+0x8]
   1409d7d94:	48 83 c1 70          	add    rcx,0x70
   1409d7d98:	e8 63 f5 07 00       	call   0x140a57300
   1409d7d9d:	80 7d 7f 00          	cmp    BYTE PTR [rbp+0x7f],0x0
   1409d7da1:	74 48                	je     0x1409d7deb
   1409d7da3:	e8 88 03 24 00       	call   0x140c18130
   1409d7da8:	48 89 c1             	mov    rcx,rax
   1409d7dab:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7dae:	48 83 f8 01          	cmp    rax,0x1
   1409d7db2:	0f 85 af 00 00 00    	jne    0x1409d7e67
   1409d7db8:	48 83 c1 08          	add    rcx,0x8
   1409d7dbc:	48 83 39 00          	cmp    QWORD PTR [rcx],0x0
   1409d7dc0:	0f 85 68 05 00 00    	jne    0x1409d832e
   1409d7dc6:	48 c7 01 ff ff ff ff 	mov    QWORD PTR [rcx],0xffffffffffffffff
   1409d7dcd:	48 89 8d 80 00 00 00 	mov    QWORD PTR [rbp+0x80],rcx
   1409d7dd4:	48 83 c1 08          	add    rcx,0x8
   1409d7dd8:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
   1409d7ddc:	e8 0f f3 23 00       	call   0x140c170f0
   1409d7de1:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d7de8:	48 ff 00             	inc    QWORD PTR [rax]
   1409d7deb:	0f 28 b5 90 00 00 00 	movaps xmm6,XMMWORD PTR [rbp+0x90]
   1409d7df2:	0f 28 bd a0 00 00 00 	movaps xmm7,XMMWORD PTR [rbp+0xa0]
   1409d7df9:	44 0f 28 85 b0 00 00 	movaps xmm8,XMMWORD PTR [rbp+0xb0]
   1409d7e00:	00 
   1409d7e01:	44 0f 28 8d c0 00 00 	movaps xmm9,XMMWORD PTR [rbp+0xc0]
   1409d7e08:	00 
   1409d7e09:	44 0f 28 95 d0 00 00 	movaps xmm10,XMMWORD PTR [rbp+0xd0]
   1409d7e10:	00 
   1409d7e11:	44 0f 28 9d e0 00 00 	movaps xmm11,XMMWORD PTR [rbp+0xe0]
   1409d7e18:	00 
   1409d7e19:	44 0f 28 a5 f0 00 00 	movaps xmm12,XMMWORD PTR [rbp+0xf0]
   1409d7e20:	00 
   1409d7e21:	44 0f 28 ad 00 01 00 	movaps xmm13,XMMWORD PTR [rbp+0x100]
   1409d7e28:	00 
   1409d7e29:	44 0f 28 b5 10 01 00 	movaps xmm14,XMMWORD PTR [rbp+0x110]
   1409d7e30:	00 
   1409d7e31:	44 0f 28 bd 20 01 00 	movaps xmm15,XMMWORD PTR [rbp+0x120]
   1409d7e38:	00 
   1409d7e39:	48 81 c4 b8 01 00 00 	add    rsp,0x1b8
   1409d7e40:	5b                   	pop    rbx
   1409d7e41:	5f                   	pop    rdi
   1409d7e42:	5e                   	pop    rsi
   1409d7e43:	41 5c                	pop    r12
   1409d7e45:	41 5d                	pop    r13
   1409d7e47:	41 5e                	pop    r14
   1409d7e49:	41 5f                	pop    r15
   1409d7e4b:	5d                   	pop    rbp
   1409d7e4c:	c3                   	ret
   1409d7e4d:	48 85 c0             	test   rax,rax
   1409d7e50:	75 2d                	jne    0x1409d7e7f
   1409d7e52:	31 d2                	xor    edx,edx
   1409d7e54:	e8 77 09 f2 01       	call   0x1428f87d0
   1409d7e59:	48 89 c1             	mov    rcx,rax
   1409d7e5c:	48 85 c0             	test   rax,rax
   1409d7e5f:	0f 85 a1 ef ff ff    	jne    0x1409d6e06
   1409d7e65:	eb 18                	jmp    0x1409d7e7f
   1409d7e67:	48 85 c0             	test   rax,rax
   1409d7e6a:	75 13                	jne    0x1409d7e7f
   1409d7e6c:	31 d2                	xor    edx,edx
   1409d7e6e:	e8 5d 09 f2 01       	call   0x1428f87d0
   1409d7e73:	48 89 c1             	mov    rcx,rax
   1409d7e76:	48 85 c0             	test   rax,rax
   1409d7e79:	0f 85 3d ff ff ff    	jne    0x1409d7dbc
   1409d7e7f:	48 8d 0d 1a 2c 26 02 	lea    rcx,[rip+0x2262c1a]        # 0x142c3aaa0
   1409d7e86:	e8 45 02 f0 01       	call   0x1428d80d0
   1409d7e8b:	48 8d 15 46 aa 0e 02 	lea    rdx,[rip+0x20eaa46]        # 0x142ac28d8
   1409d7e92:	41 b8 1b 00 00 00    	mov    r8d,0x1b
   1409d7e98:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d7e9f:	e8 ac 0f 00 00       	call   0x1409d8e50
   1409d7ea4:	e9 2f f1 ff ff       	jmp    0x1409d6fd8
   1409d7ea9:	48 8d 15 6d a9 0e 02 	lea    rdx,[rip+0x20ea96d]        # 0x142ac281d
   1409d7eb0:	41 b8 19 00 00 00    	mov    r8d,0x19
   1409d7eb6:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d7ebd:	e8 8e 0f 00 00       	call   0x1409d8e50
   1409d7ec2:	e9 11 f1 ff ff       	jmp    0x1409d6fd8
   1409d7ec7:	41 8b 44 24 1c       	mov    eax,DWORD PTR [r12+0x1c]
   1409d7ecc:	49 8b 4c 24 10       	mov    rcx,QWORD PTR [r12+0x10]
   1409d7ed1:	89 01                	mov    DWORD PTR [rcx],eax
   1409d7ed3:	49 8b 04 24          	mov    rax,QWORD PTR [r12]
   1409d7ed7:	48 89 45 08          	mov    QWORD PTR [rbp+0x8],rax
   1409d7edb:	48 8d 88 d0 00 00 00 	lea    rcx,[rax+0xd0]
   1409d7ee2:	e8 c9 17 08 00       	call   0x140a596b0
   1409d7ee7:	4c 8b 45 08          	mov    r8,QWORD PTR [rbp+0x8]
   1409d7eeb:	49 8b 90 e0 00 00 00 	mov    rdx,QWORD PTR [r8+0xe0]
   1409d7ef2:	49 8b 88 f8 00 00 00 	mov    rcx,QWORD PTR [r8+0xf8]
   1409d7ef9:	48 39 d1             	cmp    rcx,rdx
   1409d7efc:	0f 83 21 05 00 00    	jae    0x1409d8423
   1409d7f02:	48 8d 46 08          	lea    rax,[rsi+0x8]
   1409d7f06:	48 8d 0c 49          	lea    rcx,[rcx+rcx*2]
   1409d7f0a:	48 c1 e1 04          	shl    rcx,0x4
   1409d7f0e:	49 03 88 d8 00 00 00 	add    rcx,QWORD PTR [r8+0xd8]
   1409d7f15:	8b 50 08             	mov    edx,DWORD PTR [rax+0x8]
   1409d7f18:	89 55 50             	mov    DWORD PTR [rbp+0x50],edx
   1409d7f1b:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409d7f1e:	48 89 45 48          	mov    QWORD PTR [rbp+0x48],rax
   1409d7f22:	48 8d 55 48          	lea    rdx,[rbp+0x48]
   1409d7f26:	e8 75 fe 07 00       	call   0x140a57da0
   1409d7f2b:	48 8b 45 08          	mov    rax,QWORD PTR [rbp+0x8]
   1409d7f2f:	48 8b 90 e0 00 00 00 	mov    rdx,QWORD PTR [rax+0xe0]
   1409d7f36:	48 8b 88 f8 00 00 00 	mov    rcx,QWORD PTR [rax+0xf8]
   1409d7f3d:	48 39 d1             	cmp    rcx,rdx
   1409d7f40:	0f 83 eb 04 00 00    	jae    0x1409d8431
   1409d7f46:	48 8d 0c 49          	lea    rcx,[rcx+rcx*2]
   1409d7f4a:	48 c1 e1 04          	shl    rcx,0x4
   1409d7f4e:	48 03 88 d8 00 00 00 	add    rcx,QWORD PTR [rax+0xd8]
   1409d7f55:	f3 0f 10 5e 04       	movss  xmm3,DWORD PTR [rsi+0x4]
   1409d7f5a:	48 8d 15 92 a9 0e 02 	lea    rdx,[rip+0x20ea992]        # 0x142ac28f3
   1409d7f61:	41 b8 05 00 00 00    	mov    r8d,0x5
   1409d7f67:	e8 04 f7 07 00       	call   0x140a57670
   1409d7f6c:	e9 87 f0 ff ff       	jmp    0x1409d6ff8
   1409d7f71:	48 8d 15 51 a7 0e 02 	lea    rdx,[rip+0x20ea751]        # 0x142ac26c9
   1409d7f78:	41 b8 13 00 00 00    	mov    r8d,0x13
   1409d7f7e:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d7f85:	e8 c6 0e 00 00       	call   0x1409d8e50
   1409d7f8a:	e9 49 f0 ff ff       	jmp    0x1409d6fd8
   1409d7f8f:	48 8d 15 06 a7 0e 02 	lea    rdx,[rip+0x20ea706]        # 0x142ac269c
   1409d7f96:	41 b8 1a 00 00 00    	mov    r8d,0x1a
   1409d7f9c:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d7fa3:	e8 a8 0e 00 00       	call   0x1409d8e50
   1409d7fa8:	e9 2b f0 ff ff       	jmp    0x1409d6fd8
   1409d7fad:	48 8d 15 0a a9 0e 02 	lea    rdx,[rip+0x20ea90a]        # 0x142ac28be
   1409d7fb4:	41 b8 1a 00 00 00    	mov    r8d,0x1a
   1409d7fba:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d7fc1:	e8 8a 0e 00 00       	call   0x1409d8e50
   1409d7fc6:	e9 0d f0 ff ff       	jmp    0x1409d6fd8
   1409d7fcb:	48 8d 15 1b a6 0e 02 	lea    rdx,[rip+0x20ea61b]        # 0x142ac25ed
   1409d7fd2:	41 b8 1a 00 00 00    	mov    r8d,0x1a
   1409d7fd8:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d7fdf:	e8 6c 0e 00 00       	call   0x1409d8e50
   1409d7fe4:	e9 ef ef ff ff       	jmp    0x1409d6fd8
   1409d7fe9:	48 8d 15 17 a6 0e 02 	lea    rdx,[rip+0x20ea617]        # 0x142ac2607
   1409d7ff0:	41 b8 16 00 00 00    	mov    r8d,0x16
   1409d7ff6:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d7ffd:	e8 4e 0e 00 00       	call   0x1409d8e50
   1409d8002:	e9 d1 ef ff ff       	jmp    0x1409d6fd8
   1409d8007:	48 8d 15 40 a6 0e 02 	lea    rdx,[rip+0x20ea640]        # 0x142ac264e
   1409d800e:	41 b8 19 00 00 00    	mov    r8d,0x19
   1409d8014:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d801b:	e8 30 0e 00 00       	call   0x1409d8e50
   1409d8020:	e9 b3 ef ff ff       	jmp    0x1409d6fd8
   1409d8025:	48 8d 15 cc a4 0e 02 	lea    rdx,[rip+0x20ea4cc]        # 0x142ac24f8
   1409d802c:	41 b8 15 00 00 00    	mov    r8d,0x15
   1409d8032:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d8039:	e8 12 0e 00 00       	call   0x1409d8e50
   1409d803e:	e9 95 ef ff ff       	jmp    0x1409d6fd8
   1409d8043:	48 8d 15 8c a5 0e 02 	lea    rdx,[rip+0x20ea58c]        # 0x142ac25d6
   1409d804a:	41 b8 17 00 00 00    	mov    r8d,0x17
   1409d8050:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d8057:	e8 f4 0d 00 00       	call   0x1409d8e50
   1409d805c:	e9 77 ef ff ff       	jmp    0x1409d6fd8
   1409d8061:	48 8d 15 60 a4 0e 02 	lea    rdx,[rip+0x20ea460]        # 0x142ac24c8
   1409d8068:	41 b8 12 00 00 00    	mov    r8d,0x12
   1409d806e:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d8075:	e8 d6 0d 00 00       	call   0x1409d8e50
   1409d807a:	e9 59 ef ff ff       	jmp    0x1409d6fd8
   1409d807f:	48 8d 15 e1 a5 0e 02 	lea    rdx,[rip+0x20ea5e1]        # 0x142ac2667
   1409d8086:	41 b8 1c 00 00 00    	mov    r8d,0x1c
   1409d808c:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d8093:	e8 b8 0d 00 00       	call   0x1409d8e50
   1409d8098:	e9 3b ef ff ff       	jmp    0x1409d6fd8
   1409d809d:	48 8d 15 79 a5 0e 02 	lea    rdx,[rip+0x20ea579]        # 0x142ac261d
   1409d80a4:	41 b8 18 00 00 00    	mov    r8d,0x18
   1409d80aa:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d80b1:	e8 9a 0d 00 00       	call   0x1409d8e50
   1409d80b6:	e9 1d ef ff ff       	jmp    0x1409d6fd8
   1409d80bb:	48 8d 15 20 a7 0e 02 	lea    rdx,[rip+0x20ea720]        # 0x142ac27e2
   1409d80c2:	41 b8 20 00 00 00    	mov    r8d,0x20
   1409d80c8:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d80cf:	e8 7c 0d 00 00       	call   0x1409d8e50
   1409d80d4:	e9 ff ee ff ff       	jmp    0x1409d6fd8
   1409d80d9:	48 8d 15 e5 a6 0e 02 	lea    rdx,[rip+0x20ea6e5]        # 0x142ac27c5
   1409d80e0:	41 b8 1d 00 00 00    	mov    r8d,0x1d
   1409d80e6:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d80ed:	e8 5e 0d 00 00       	call   0x1409d8e50
   1409d80f2:	e9 e1 ee ff ff       	jmp    0x1409d6fd8
   1409d80f7:	48 8d 15 c5 a4 0e 02 	lea    rdx,[rip+0x20ea4c5]        # 0x142ac25c3
   1409d80fe:	41 b8 13 00 00 00    	mov    r8d,0x13
   1409d8104:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d810b:	e8 40 0d 00 00       	call   0x1409d8e50
   1409d8110:	e9 c3 ee ff ff       	jmp    0x1409d6fd8
   1409d8115:	48 8d 15 9a a5 0e 02 	lea    rdx,[rip+0x20ea59a]        # 0x142ac26b6
   1409d811c:	41 b8 13 00 00 00    	mov    r8d,0x13
   1409d8122:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d8129:	e8 22 0d 00 00       	call   0x1409d8e50
   1409d812e:	e9 a5 ee ff ff       	jmp    0x1409d6fd8
   1409d8133:	48 8d 15 4c a7 0e 02 	lea    rdx,[rip+0x20ea74c]        # 0x142ac2886
   1409d813a:	41 b8 20 00 00 00    	mov    r8d,0x20
   1409d8140:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d8147:	e8 04 0d 00 00       	call   0x1409d8e50
   1409d814c:	e9 87 ee ff ff       	jmp    0x1409d6fd8
   1409d8151:	48 8d 15 84 a5 0e 02 	lea    rdx,[rip+0x20ea584]        # 0x142ac26dc
   1409d8158:	41 b8 1f 00 00 00    	mov    r8d,0x1f
   1409d815e:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d8165:	e8 e6 0c 00 00       	call   0x1409d8e50
   1409d816a:	e9 69 ee ff ff       	jmp    0x1409d6fd8
   1409d816f:	48 8d 15 d1 a5 0e 02 	lea    rdx,[rip+0x20ea5d1]        # 0x142ac2747
   1409d8176:	41 b8 1d 00 00 00    	mov    r8d,0x1d
   1409d817c:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d8183:	e8 c8 0c 00 00       	call   0x1409d8e50
   1409d8188:	e9 4b ee ff ff       	jmp    0x1409d6fd8
   1409d818d:	48 8d 15 d0 a5 0e 02 	lea    rdx,[rip+0x20ea5d0]        # 0x142ac2764
   1409d8194:	41 b8 20 00 00 00    	mov    r8d,0x20
   1409d819a:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d81a1:	e8 aa 0c 00 00       	call   0x1409d8e50
   1409d81a6:	e9 2d ee ff ff       	jmp    0x1409d6fd8
   1409d81ab:	48 8d 15 d1 a4 0e 02 	lea    rdx,[rip+0x20ea4d1]        # 0x142ac2683
   1409d81b2:	41 b8 19 00 00 00    	mov    r8d,0x19
   1409d81b8:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d81bf:	e8 8c 0c 00 00       	call   0x1409d8e50
   1409d81c4:	e9 0f ee ff ff       	jmp    0x1409d6fd8
   1409d81c9:	48 8d 15 2b a5 0e 02 	lea    rdx,[rip+0x20ea52b]        # 0x142ac26fb
   1409d81d0:	41 b8 1f 00 00 00    	mov    r8d,0x1f
   1409d81d6:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d81dd:	e8 6e 0c 00 00       	call   0x1409d8e50
   1409d81e2:	e9 f1 ed ff ff       	jmp    0x1409d6fd8
   1409d81e7:	48 8d 15 47 a4 0e 02 	lea    rdx,[rip+0x20ea447]        # 0x142ac2635
   1409d81ee:	41 b8 19 00 00 00    	mov    r8d,0x19
   1409d81f4:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d81fb:	e8 50 0c 00 00       	call   0x1409d8e50
   1409d8200:	e9 d3 ed ff ff       	jmp    0x1409d6fd8
   1409d8205:	48 8d 15 5c a6 0e 02 	lea    rdx,[rip+0x20ea65c]        # 0x142ac2868
   1409d820c:	41 b8 1e 00 00 00    	mov    r8d,0x1e
   1409d8212:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d8219:	e8 32 0c 00 00       	call   0x1409d8e50
   1409d821e:	e9 b5 ed ff ff       	jmp    0x1409d6fd8
   1409d8223:	48 8d 15 7c a6 0e 02 	lea    rdx,[rip+0x20ea67c]        # 0x142ac28a6
   1409d822a:	41 b8 18 00 00 00    	mov    r8d,0x18
   1409d8230:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d8237:	e8 14 0c 00 00       	call   0x1409d8e50
   1409d823c:	e9 97 ed ff ff       	jmp    0x1409d6fd8
   1409d8241:	48 8d 15 3c a5 0e 02 	lea    rdx,[rip+0x20ea53c]        # 0x142ac2784
   1409d8248:	41 b8 23 00 00 00    	mov    r8d,0x23
   1409d824e:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d8255:	e8 f6 0b 00 00       	call   0x1409d8e50
   1409d825a:	e9 79 ed ff ff       	jmp    0x1409d6fd8
   1409d825f:	48 8d 15 b4 a4 0e 02 	lea    rdx,[rip+0x20ea4b4]        # 0x142ac271a
   1409d8266:	41 b8 2d 00 00 00    	mov    r8d,0x2d
   1409d826c:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d8273:	e8 d8 0b 00 00       	call   0x1409d8e50
   1409d8278:	e9 5b ed ff ff       	jmp    0x1409d6fd8
   1409d827d:	48 8d 15 23 a5 0e 02 	lea    rdx,[rip+0x20ea523]        # 0x142ac27a7
   1409d8284:	41 b8 1e 00 00 00    	mov    r8d,0x1e
   1409d828a:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d8291:	e8 ba 0b 00 00       	call   0x1409d8e50
   1409d8296:	e9 3d ed ff ff       	jmp    0x1409d6fd8
   1409d829b:	48 8d 15 94 a5 0e 02 	lea    rdx,[rip+0x20ea594]        # 0x142ac2836
   1409d82a2:	41 b8 16 00 00 00    	mov    r8d,0x16
   1409d82a8:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d82af:	e8 9c 0b 00 00       	call   0x1409d8e50
   1409d82b4:	e9 1f ed ff ff       	jmp    0x1409d6fd8
   1409d82b9:	48 8d 15 8c a5 0e 02 	lea    rdx,[rip+0x20ea58c]        # 0x142ac284c
   1409d82c0:	41 b8 1c 00 00 00    	mov    r8d,0x1c
   1409d82c6:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d82cd:	e8 7e 0b 00 00       	call   0x1409d8e50
   1409d82d2:	e9 01 ed ff ff       	jmp    0x1409d6fd8
   1409d82d7:	48 8d 15 24 a5 0e 02 	lea    rdx,[rip+0x20ea524]        # 0x142ac2802
   1409d82de:	41 b8 1b 00 00 00    	mov    r8d,0x1b
   1409d82e4:	48 8b 8d 80 00 00 00 	mov    rcx,QWORD PTR [rbp+0x80]
   1409d82eb:	e8 60 0b 00 00       	call   0x1409d8e50
   1409d82f0:	e9 e3 ec ff ff       	jmp    0x1409d6fd8
   1409d82f5:	48 89 c2             	mov    rdx,rax
   1409d82f8:	48 d1 ea             	shr    rdx,1
   1409d82fb:	83 e0 01             	and    eax,0x1
   1409d82fe:	48 09 d0             	or     rax,rdx
   1409d8301:	f3 48 0f 2a d8       	cvtsi2ss xmm3,rax
   1409d8306:	f3 0f 58 db          	addss  xmm3,xmm3
   1409d830a:	48 83 c1 10          	add    rcx,0x10
   1409d830e:	48 8d 15 a3 a2 0e 02 	lea    rdx,[rip+0x20ea2a3]        # 0x142ac25b8
   1409d8315:	41 b8 0b 00 00 00    	mov    r8d,0xb
   1409d831b:	e8 90 ba 07 00       	call   0x140a53db0
   1409d8320:	49 8b 0c 24          	mov    rcx,QWORD PTR [r12]
   1409d8324:	e8 d7 ef 07 00       	call   0x140a57300
   1409d8329:	e9 aa ec ff ff       	jmp    0x1409d6fd8
   1409d832e:	48 8d 0d 3b 28 26 02 	lea    rcx,[rip+0x226283b]        # 0x142c3ab70
   1409d8335:	e8 16 0f f0 01       	call   0x1428d9250
   1409d833a:	48 8d 0d 52 1e 10 02 	lea    rcx,[rip+0x2101e52]        # 0x142ada193
   1409d8341:	4c 8d 05 c8 a5 0e 02 	lea    r8,[rip+0x20ea5c8]        # 0x142ac2910
   1409d8348:	ba 32 00 00 00       	mov    edx,0x32
   1409d834d:	e8 de 10 f0 01       	call   0x1428d9430
   1409d8352:	e9 e6 00 00 00       	jmp    0x1409d843d
   1409d8357:	48 8d 0d 35 1e 10 02 	lea    rcx,[rip+0x2101e35]        # 0x142ada193
   1409d835e:	4c 8d 05 c3 a5 0e 02 	lea    r8,[rip+0x20ea5c3]        # 0x142ac2928
   1409d8365:	ba 32 00 00 00       	mov    edx,0x32
   1409d836a:	e8 c1 10 f0 01       	call   0x1428d9430
   1409d836f:	e9 c9 00 00 00       	jmp    0x1409d843d
   1409d8374:	48 8b 45 50          	mov    rax,QWORD PTR [rbp+0x50]
   1409d8378:	48 89 45 10          	mov    QWORD PTR [rbp+0x10],rax
   1409d837c:	48 8d 05 05 a6 0e 02 	lea    rax,[rip+0x20ea605]        # 0x142ac2988
   1409d8383:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   1409d8388:	48 8d 0d e9 9a 0e 02 	lea    rcx,[rip+0x20e9ae9]        # 0x142ac1e78
   1409d838f:	4c 8d 0d c2 9a 0e 02 	lea    r9,[rip+0x20e9ac2]        # 0x142ac1e58
   1409d8396:	4c 8d 45 10          	lea    r8,[rbp+0x10]
   1409d839a:	ba 2b 00 00 00       	mov    edx,0x2b
   1409d839f:	e8 bc 13 f0 01       	call   0x1428d9760
   1409d83a4:	e9 94 00 00 00       	jmp    0x1409d843d
   1409d83a9:	48 8b 45 50          	mov    rax,QWORD PTR [rbp+0x50]
   1409d83ad:	48 89 45 10          	mov    QWORD PTR [rbp+0x10],rax
   1409d83b1:	48 8d 05 d0 a5 0e 02 	lea    rax,[rip+0x20ea5d0]        # 0x142ac2988
   1409d83b8:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   1409d83bd:	48 8d 0d b4 9a 0e 02 	lea    rcx,[rip+0x20e9ab4]        # 0x142ac1e78
   1409d83c4:	4c 8d 0d 8d 9a 0e 02 	lea    r9,[rip+0x20e9a8d]        # 0x142ac1e58
   1409d83cb:	4c 8d 45 10          	lea    r8,[rbp+0x10]
   1409d83cf:	ba 2b 00 00 00       	mov    edx,0x2b
   1409d83d4:	e8 87 13 f0 01       	call   0x1428d9760
   1409d83d9:	eb 62                	jmp    0x1409d843d
   1409d83db:	4c 8d 05 ce 83 0f 02 	lea    r8,[rip+0x20f83ce]        # 0x142ad07b0
   1409d83e2:	b9 01 00 00 00       	mov    ecx,0x1
   1409d83e7:	ba 1e 00 00 00       	mov    edx,0x1e
   1409d83ec:	e8 f2 0b f0 01       	call   0x1428d8fe3
   1409d83f1:	eb 4a                	jmp    0x1409d843d
   1409d83f3:	4c 8d 05 b6 83 0f 02 	lea    r8,[rip+0x20f83b6]        # 0x142ad07b0
   1409d83fa:	b9 01 00 00 00       	mov    ecx,0x1
   1409d83ff:	ba 1f 00 00 00       	mov    edx,0x1f
   1409d8404:	e8 da 0b f0 01       	call   0x1428d8fe3
   1409d8409:	eb 32                	jmp    0x1409d843d
   1409d840b:	4c 8d 05 9e 83 0f 02 	lea    r8,[rip+0x20f839e]        # 0x142ad07b0
   1409d8412:	b9 01 00 00 00       	mov    ecx,0x1
   1409d8417:	ba 18 00 00 00       	mov    edx,0x18
   1409d841c:	e8 c2 0b f0 01       	call   0x1428d8fe3
   1409d8421:	eb 1a                	jmp    0x1409d843d
   1409d8423:	4c 8d 05 fe aa 0f 02 	lea    r8,[rip+0x20faafe]        # 0x142ad2f28
   1409d842a:	e8 e9 10 f0 01       	call   0x1428d9518
   1409d842f:	eb 0c                	jmp    0x1409d843d
   1409d8431:	4c 8d 05 f0 aa 0f 02 	lea    r8,[rip+0x20faaf0]        # 0x142ad2f28
   1409d8438:	e8 db 10 f0 01       	call   0x1428d9518
   1409d843d:	0f 0b                	ud2

   1409d8e50:	55                   	push   rbp
   1409d8e51:	41 56                	push   r14
   1409d8e53:	56                   	push   rsi
   1409d8e54:	57                   	push   rdi
   1409d8e55:	53                   	push   rbx
   1409d8e56:	48 81 ec a0 00 00 00 	sub    rsp,0xa0
   1409d8e5d:	48 8d ac 24 80 00 00 	lea    rbp,[rsp+0x80]
   1409d8e64:	00 
   1409d8e65:	48 c7 45 18 fe ff ff 	mov    QWORD PTR [rbp+0x18],0xfffffffffffffffe
   1409d8e6c:	ff 
   1409d8e6d:	4c 89 c6             	mov    rsi,r8
   1409d8e70:	48 89 d7             	mov    rdi,rdx
   1409d8e73:	48 8b 01             	mov    rax,QWORD PTR [rcx]
   1409d8e76:	48 8b 10             	mov    rdx,QWORD PTR [rax]
   1409d8e79:	48 85 d2             	test   rdx,rdx
   1409d8e7c:	74 50                	je     0x1409d8ece
   1409d8e7e:	48 83 c2 10          	add    rdx,0x10
   1409d8e82:	48 8d 4d b0          	lea    rcx,[rbp-0x50]
   1409d8e86:	49 89 f8             	mov    r8,rdi
   1409d8e89:	49 89 f1             	mov    r9,rsi
   1409d8e8c:	e8 af b5 07 00       	call   0x140a54440
   1409d8e91:	48 83 7d b0 02       	cmp    QWORD PTR [rbp-0x50],0x2
   1409d8e96:	0f 85 90 00 00 00    	jne    0x1409d8f2c
   1409d8e9c:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
   1409d8ea0:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
   1409d8ea4:	48 8d 05 dd 9a 0e 02 	lea    rax,[rip+0x20e9add]        # 0x142ac2988
   1409d8eab:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   1409d8eb0:	48 8d 0d c1 8f 0e 02 	lea    rcx,[rip+0x20e8fc1]        # 0x142ac1e78
   1409d8eb7:	4c 8d 0d 9a 8f 0e 02 	lea    r9,[rip+0x20e8f9a]        # 0x142ac1e58
   1409d8ebe:	4c 8d 45 e0          	lea    r8,[rbp-0x20]
   1409d8ec2:	ba 2b 00 00 00       	mov    edx,0x2b
   1409d8ec7:	e8 94 08 f0 01       	call   0x1428d9760
   1409d8ecc:	0f 0b                	ud2
   1409d8ece:	48 85 f6             	test   rsi,rsi
   1409d8ed1:	0f 88 da 00 00 00    	js     0x1409d8fb1
   1409d8ed7:	74 28                	je     0x1409d8f01
   1409d8ed9:	0f b6 05 29 bd c5 02 	movzx  eax,BYTE PTR [rip+0x2c5bd29]        # 0x143634c09
   1409d8ee0:	41 be 01 00 00 00    	mov    r14d,0x1
   1409d8ee6:	ba 01 00 00 00       	mov    edx,0x1
   1409d8eeb:	48 89 f1             	mov    rcx,rsi
   1409d8eee:	e8 1d ad c3 ff       	call   0x140613c10
   1409d8ef3:	48 85 c0             	test   rax,rax
   1409d8ef6:	0f 84 b8 00 00 00    	je     0x1409d8fb4
   1409d8efc:	48 89 c3             	mov    rbx,rax
   1409d8eff:	eb 05                	jmp    0x1409d8f06
   1409d8f01:	bb 01 00 00 00       	mov    ebx,0x1
   1409d8f06:	48 89 d9             	mov    rcx,rbx
   1409d8f09:	48 89 fa             	mov    rdx,rdi
   1409d8f0c:	49 89 f0             	mov    r8,rsi
   1409d8f0f:	e8 ec ac f4 01       	call   0x142923c00
   1409d8f14:	48 c7 45 b0 00 00 00 	mov    QWORD PTR [rbp-0x50],0x0
   1409d8f1b:	00 
   1409d8f1c:	48 89 75 c0          	mov    QWORD PTR [rbp-0x40],rsi
   1409d8f20:	48 89 5d c8          	mov    QWORD PTR [rbp-0x38],rbx
   1409d8f24:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
   1409d8f28:	c6 45 d8 02          	mov    BYTE PTR [rbp-0x28],0x2
   1409d8f2c:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
   1409d8f30:	48 8b 4d b8          	mov    rcx,QWORD PTR [rbp-0x48]
   1409d8f34:	48 8b 55 c0          	mov    rdx,QWORD PTR [rbp-0x40]
   1409d8f38:	4c 8b 45 c8          	mov    r8,QWORD PTR [rbp-0x38]
   1409d8f3c:	4c 8b 4d d0          	mov    r9,QWORD PTR [rbp-0x30]
   1409d8f40:	4c 89 4d 00          	mov    QWORD PTR [rbp+0x0],r9
   1409d8f44:	44 0f b6 4d d8       	movzx  r9d,BYTE PTR [rbp-0x28]
   1409d8f49:	44 88 4d 08          	mov    BYTE PTR [rbp+0x8],r9b
   1409d8f4d:	44 8b 4d d9          	mov    r9d,DWORD PTR [rbp-0x27]
   1409d8f51:	44 89 4d 09          	mov    DWORD PTR [rbp+0x9],r9d
   1409d8f55:	44 0f b7 4d dd       	movzx  r9d,WORD PTR [rbp-0x23]
   1409d8f5a:	66 44 89 4d 0d       	mov    WORD PTR [rbp+0xd],r9w
   1409d8f5f:	44 0f b6 4d df       	movzx  r9d,BYTE PTR [rbp-0x21]
   1409d8f64:	44 88 4d 0f          	mov    BYTE PTR [rbp+0xf],r9b
   1409d8f68:	48 89 55 f0          	mov    QWORD PTR [rbp-0x10],rdx
   1409d8f6c:	4c 89 45 f8          	mov    QWORD PTR [rbp-0x8],r8
   1409d8f70:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
   1409d8f74:	48 89 4d e8          	mov    QWORD PTR [rbp-0x18],rcx
   1409d8f78:	48 8d 4d e0          	lea    rcx,[rbp-0x20]
   1409d8f7c:	e8 7f e3 07 00       	call   0x140a57300
   1409d8f81:	48 8d 4d e0          	lea    rcx,[rbp-0x20]
   1409d8f85:	e8 f6 12 ec ff       	call   0x14089a280
   1409d8f8a:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
   1409d8f8e:	48 85 d2             	test   rdx,rdx
   1409d8f91:	74 0f                	je     0x1409d8fa2
   1409d8f93:	48 8b 4d f8          	mov    rcx,QWORD PTR [rbp-0x8]
   1409d8f97:	41 b8 01 00 00 00    	mov    r8d,0x1
   1409d8f9d:	e8 7e ac c3 ff       	call   0x140613c20
   1409d8fa2:	90                   	nop
   1409d8fa3:	48 81 c4 a0 00 00 00 	add    rsp,0xa0
   1409d8faa:	5b                   	pop    rbx
   1409d8fab:	5f                   	pop    rdi
   1409d8fac:	5e                   	pop    rsi
   1409d8fad:	41 5e                	pop    r14
   1409d8faf:	5d                   	pop    rbp
   1409d8fb0:	c3                   	ret
   1409d8fb1:	45 31 f6             	xor    r14d,r14d
   1409d8fb4:	4c 8d 05 f5 77 0f 02 	lea    r8,[rip+0x20f77f5]        # 0x142ad07b0
   1409d8fbb:	4c 89 f1             	mov    rcx,r14
   1409d8fbe:	48 89 f2             	mov    rdx,rsi
   1409d8fc1:	e8 1d 00 f0 01       	call   0x1428d8fe3
   1409d8fc6:	cc                   	int3

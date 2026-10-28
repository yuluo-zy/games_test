
tiny-glade.exe:     file format pei-x86-64


Disassembly of section .text:

0000000140a69c30 <.text+0xa68c30>:
   140a69c30:	55                   	push   rbp
   140a69c31:	41 57                	push   r15
   140a69c33:	41 56                	push   r14
   140a69c35:	41 55                	push   r13
   140a69c37:	41 54                	push   r12
   140a69c39:	56                   	push   rsi
   140a69c3a:	57                   	push   rdi
   140a69c3b:	53                   	push   rbx
   140a69c3c:	48 81 ec b8 00 00 00 	sub    rsp,0xb8
   140a69c43:	48 8d ac 24 80 00 00 	lea    rbp,[rsp+0x80]
   140a69c4a:	00 
   140a69c4b:	48 c7 45 30 fe ff ff 	mov    QWORD PTR [rbp+0x30],0xfffffffffffffffe
   140a69c52:	ff 
   140a69c53:	48 89 d6             	mov    rsi,rdx
   140a69c56:	48 89 cf             	mov    rdi,rcx
   140a69c59:	e8 a2 cd 1a 00       	call   0x140c16a00
   140a69c5e:	84 c0                	test   al,al
   140a69c60:	74 5f                	je     0x140a69cc1
   140a69c62:	4c 8d 35 43 af 06 02 	lea    r14,[rip+0x206af43]        # 0x142ad4bac
   140a69c69:	bb 45 00 00 00       	mov    ebx,0x45
   140a69c6e:	ba 45 00 00 00       	mov    edx,0x45
   140a69c73:	4c 89 f1             	mov    rcx,r14
   140a69c76:	e8 05 d7 ff ff       	call   0x140a67380
   140a69c7b:	48 83 f8 01          	cmp    rax,0x1
   140a69c7f:	0f 85 b1 00 00 00    	jne    0x140a69d36
   140a69c85:	49 89 d1             	mov    r9,rdx
   140a69c88:	48 85 d2             	test   rdx,rdx
   140a69c8b:	74 3d                	je     0x140a69cca
   140a69c8d:	49 83 f9 45          	cmp    r9,0x45
   140a69c91:	73 35                	jae    0x140a69cc8
   140a69c93:	48 8d 05 12 af 06 02 	lea    rax,[rip+0x206af12]        # 0x142ad4bac
   140a69c9a:	41 80 3c 01 bf       	cmp    BYTE PTR [r9+rax*1],0xbf
   140a69c9f:	7f 29                	jg     0x140a69cca
   140a69ca1:	48 8d 05 d0 af 06 02 	lea    rax,[rip+0x206afd0]        # 0x142ad4c78
   140a69ca8:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140a69cad:	48 8d 0d f8 ae 06 02 	lea    rcx,[rip+0x206aef8]        # 0x142ad4bac
   140a69cb4:	ba 45 00 00 00       	mov    edx,0x45
   140a69cb9:	45 31 c0             	xor    r8d,r8d
   140a69cbc:	e8 7f fc e6 01       	call   0x1428d9940
   140a69cc1:	31 c0                	xor    eax,eax
   140a69cc3:	e9 ce 01 00 00       	jmp    0x140a69e96
   140a69cc8:	75 d7                	jne    0x140a69ca1
   140a69cca:	4c 8d 35 db ae 06 02 	lea    r14,[rip+0x206aedb]        # 0x142ad4bac
   140a69cd1:	4c 89 f1             	mov    rcx,r14
   140a69cd4:	4c 89 ca             	mov    rdx,r9
   140a69cd7:	e8 a4 d6 ff ff       	call   0x140a67380
   140a69cdc:	48 83 f8 01          	cmp    rax,0x1
   140a69ce0:	75 54                	jne    0x140a69d36
   140a69ce2:	49 89 d0             	mov    r8,rdx
   140a69ce5:	49 83 c0 02          	add    r8,0x2
   140a69ce9:	74 39                	je     0x140a69d24
   140a69ceb:	49 83 f8 45          	cmp    r8,0x45
   140a69cef:	73 31                	jae    0x140a69d22
   140a69cf1:	48 8d 05 b4 ae 06 02 	lea    rax,[rip+0x206aeb4]        # 0x142ad4bac
   140a69cf8:	41 80 3c 00 bf       	cmp    BYTE PTR [r8+rax*1],0xbf
   140a69cfd:	7f 25                	jg     0x140a69d24
   140a69cff:	48 8d 05 8a af 06 02 	lea    rax,[rip+0x206af8a]        # 0x142ad4c90
   140a69d06:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140a69d0b:	48 8d 0d 9a ae 06 02 	lea    rcx,[rip+0x206ae9a]        # 0x142ad4bac
   140a69d12:	ba 45 00 00 00       	mov    edx,0x45
   140a69d17:	41 b9 45 00 00 00    	mov    r9d,0x45
   140a69d1d:	e8 1e fc e6 01       	call   0x1428d9940
   140a69d22:	75 db                	jne    0x140a69cff
   140a69d24:	bb 43 00 00 00       	mov    ebx,0x43
   140a69d29:	48 29 d3             	sub    rbx,rdx
   140a69d2c:	4c 8d 35 79 ae 06 02 	lea    r14,[rip+0x206ae79]        # 0x142ad4bac
   140a69d33:	4d 01 c6             	add    r14,r8
   140a69d36:	48 8d 05 a1 b4 06 02 	lea    rax,[rip+0x206b4a1]        # 0x142ad51de
   140a69d3d:	4c 8d 3d 60 b4 06 02 	lea    r15,[rip+0x206b460]        # 0x142ad51a4
   140a69d44:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   140a69d4b:	00 00 00 00 00 
   140a69d50:	0f be 48 ff          	movsx  ecx,BYTE PTR [rax-0x1]
   140a69d54:	85 c9                	test   ecx,ecx
   140a69d56:	78 18                	js     0x140a69d70
   140a69d58:	48 ff c8             	dec    rax
   140a69d5b:	83 f9 5c             	cmp    ecx,0x5c
   140a69d5e:	75 69                	jne    0x140a69dc9
   140a69d60:	eb 7d                	jmp    0x140a69ddf
   140a69d62:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   140a69d69:	1f 84 00 00 00 00 00 
   140a69d70:	0f b6 50 fe          	movzx  edx,BYTE PTR [rax-0x2]
   140a69d74:	80 fa c0             	cmp    dl,0xc0
   140a69d77:	7d 25                	jge    0x140a69d9e
   140a69d79:	44 0f b6 40 fd       	movzx  r8d,BYTE PTR [rax-0x3]
   140a69d7e:	41 80 f8 c0          	cmp    r8b,0xc0
   140a69d82:	7d 23                	jge    0x140a69da7
   140a69d84:	44 0f b6 48 fc       	movzx  r9d,BYTE PTR [rax-0x4]
   140a69d89:	48 83 c0 fc          	add    rax,0xfffffffffffffffc
   140a69d8d:	41 83 e1 07          	and    r9d,0x7
   140a69d91:	41 c1 e1 06          	shl    r9d,0x6
   140a69d95:	41 83 e0 3f          	and    r8d,0x3f
   140a69d99:	45 09 c8             	or     r8d,r9d
   140a69d9c:	eb 11                	jmp    0x140a69daf
   140a69d9e:	48 83 c0 fe          	add    rax,0xfffffffffffffffe
   140a69da2:	83 e2 1f             	and    edx,0x1f
   140a69da5:	eb 12                	jmp    0x140a69db9
   140a69da7:	48 83 c0 fd          	add    rax,0xfffffffffffffffd
   140a69dab:	41 83 e0 0f          	and    r8d,0xf
   140a69daf:	41 c1 e0 06          	shl    r8d,0x6
   140a69db3:	83 e2 3f             	and    edx,0x3f
   140a69db6:	44 09 c2             	or     edx,r8d
   140a69db9:	c1 e2 06             	shl    edx,0x6
   140a69dbc:	80 e1 3f             	and    cl,0x3f
   140a69dbf:	0f b6 c9             	movzx  ecx,cl
   140a69dc2:	09 d1                	or     ecx,edx
   140a69dc4:	83 f9 5c             	cmp    ecx,0x5c
   140a69dc7:	74 16                	je     0x140a69ddf
   140a69dc9:	83 f9 2f             	cmp    ecx,0x2f
   140a69dcc:	74 11                	je     0x140a69ddf
   140a69dce:	4c 39 f8             	cmp    rax,r15
   140a69dd1:	0f 85 79 ff ff ff    	jne    0x140a69d50
   140a69dd7:	41 bc 3a 00 00 00    	mov    r12d,0x3a
   140a69ddd:	eb 50                	jmp    0x140a69e2f
   140a69ddf:	4c 8d 3d be b3 06 02 	lea    r15,[rip+0x206b3be]        # 0x142ad51a4
   140a69de6:	4c 29 f8             	sub    rax,r15
   140a69de9:	49 89 c0             	mov    r8,rax
   140a69dec:	49 ff c0             	inc    r8
   140a69def:	74 32                	je     0x140a69e23
   140a69df1:	49 83 f8 3a          	cmp    r8,0x3a
   140a69df5:	73 2a                	jae    0x140a69e21
   140a69df7:	43 80 3c 38 bf       	cmp    BYTE PTR [r8+r15*1],0xbf
   140a69dfc:	7f 25                	jg     0x140a69e23
   140a69dfe:	48 8d 05 53 ae 06 02 	lea    rax,[rip+0x206ae53]        # 0x142ad4c58
   140a69e05:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140a69e0a:	48 8d 0d 93 b3 06 02 	lea    rcx,[rip+0x206b393]        # 0x142ad51a4
   140a69e11:	ba 3a 00 00 00       	mov    edx,0x3a
   140a69e16:	41 b9 3a 00 00 00    	mov    r9d,0x3a
   140a69e1c:	e8 1f fb e6 01       	call   0x1428d9940
   140a69e21:	75 db                	jne    0x140a69dfe
   140a69e23:	41 bc 39 00 00 00    	mov    r12d,0x39
   140a69e29:	49 29 c4             	sub    r12,rax
   140a69e2c:	4d 01 c7             	add    r15,r8
   140a69e2f:	e8 fc e2 1a 00       	call   0x140c18130
   140a69e34:	48 89 c1             	mov    rcx,rax
   140a69e37:	48 8b 00             	mov    rax,QWORD PTR [rax]
   140a69e3a:	48 83 f8 01          	cmp    rax,0x1
   140a69e3e:	0f 85 15 05 00 00    	jne    0x140a6a359
   140a69e44:	48 83 c1 08          	add    rcx,0x8
   140a69e48:	48 83 39 00          	cmp    QWORD PTR [rcx],0x0
   140a69e4c:	0f 85 2b 05 00 00    	jne    0x140a6a37d
   140a69e52:	48 c7 01 ff ff ff ff 	mov    QWORD PTR [rcx],0xffffffffffffffff
   140a69e59:	48 89 4d 20          	mov    QWORD PTR [rbp+0x20],rcx
   140a69e5d:	48 83 c1 08          	add    rcx,0x8
   140a69e61:	4c 89 64 24 20       	mov    QWORD PTR [rsp+0x20],r12
   140a69e66:	48 c7 44 24 30 00 00 	mov    QWORD PTR [rsp+0x30],0x0
   140a69e6d:	00 00 
   140a69e6f:	48 c7 44 24 28 01 00 	mov    QWORD PTR [rsp+0x28],0x1
   140a69e76:	00 00 
   140a69e78:	4c 89 f2             	mov    rdx,r14
   140a69e7b:	49 89 d8             	mov    r8,rbx
   140a69e7e:	4d 89 f9             	mov    r9,r15
   140a69e81:	e8 ba ce 1a 00       	call   0x140c16d40
   140a69e86:	48 8b 4d 20          	mov    rcx,QWORD PTR [rbp+0x20]
   140a69e8a:	48 ff 01             	inc    QWORD PTR [rcx]
   140a69e8d:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
   140a69e91:	b8 01 00 00 00       	mov    eax,0x1
   140a69e96:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
   140a69e9a:	48 8b 07             	mov    rax,QWORD PTR [rdi]
   140a69e9d:	4c 8b 47 08          	mov    r8,QWORD PTR [rdi+0x8]
   140a69ea1:	8b 4f 14             	mov    ecx,DWORD PTR [rdi+0x14]
   140a69ea4:	89 4d 1c             	mov    DWORD PTR [rbp+0x1c],ecx
   140a69ea7:	4c 8b 90 f8 00 00 00 	mov    r10,QWORD PTR [rax+0xf8]
   140a69eae:	48 8b 88 00 01 00 00 	mov    rcx,QWORD PTR [rax+0x100]
   140a69eb5:	4d 8d 1c 8a          	lea    r11,[r10+rcx*4]
   140a69eb9:	0f b6 88 08 01 00 00 	movzx  ecx,BYTE PTR [rax+0x108]
   140a69ec0:	88 4d 20             	mov    BYTE PTR [rbp+0x20],cl
   140a69ec3:	48 8b 0e             	mov    rcx,QWORD PTR [rsi]
   140a69ec6:	48 89 4d d0          	mov    QWORD PTR [rbp-0x30],rcx
   140a69eca:	41 be 08 00 00 00    	mov    r14d,0x8
   140a69ed0:	31 f6                	xor    esi,esi
   140a69ed2:	31 db                	xor    ebx,ebx
   140a69ed4:	45 31 c9             	xor    r9d,r9d
   140a69ed7:	45 31 ff             	xor    r15d,r15d
   140a69eda:	45 31 e4             	xor    r12d,r12d
   140a69edd:	48 c7 45 10 00 00 00 	mov    QWORD PTR [rbp+0x10],0x0
   140a69ee4:	00 
   140a69ee5:	c7 45 2c 00 00 00 00 	mov    DWORD PTR [rbp+0x2c],0x0
   140a69eec:	80 7d 20 00          	cmp    BYTE PTR [rbp+0x20],0x0
   140a69ef0:	0f 84 8a 00 00 00    	je     0x140a69f80
   140a69ef6:	4d 39 e7             	cmp    r15,r12
   140a69ef9:	0f 85 a0 02 00 00    	jne    0x140a6a19f
   140a69eff:	90                   	nop
   140a69f00:	4d 39 da             	cmp    r10,r11
   140a69f03:	0f 84 e5 03 00 00    	je     0x140a6a2ee
   140a69f09:	41 8b 12             	mov    edx,DWORD PTR [r10]
   140a69f0c:	49 83 c2 04          	add    r10,0x4
   140a69f10:	49 8b 88 a8 01 00 00 	mov    rcx,QWORD PTR [r8+0x1a8]
   140a69f17:	48 8d 14 d2          	lea    rdx,[rdx+rdx*8]
   140a69f1b:	4c 8b 64 d1 10       	mov    r12,QWORD PTR [rcx+rdx*8+0x10]
   140a69f20:	4d 85 e4             	test   r12,r12
   140a69f23:	74 db                	je     0x140a69f00
   140a69f25:	4c 89 cf             	mov    rdi,r9
   140a69f28:	48 8d 14 d1          	lea    rdx,[rcx+rdx*8]
   140a69f2c:	48 8b b0 10 01 00 00 	mov    rsi,QWORD PTR [rax+0x110]
   140a69f33:	48 8b 4a 38          	mov    rcx,QWORD PTR [rdx+0x38]
   140a69f37:	4c 8b 4a 40          	mov    r9,QWORD PTR [rdx+0x40]
   140a69f3b:	49 39 f1             	cmp    r9,rsi
   140a69f3e:	0f 86 d8 00 00 00    	jbe    0x140a6a01c
   140a69f44:	48 8b 34 f1          	mov    rsi,QWORD PTR [rcx+rsi*8]
   140a69f48:	48 8b 52 18          	mov    rdx,QWORD PTR [rdx+0x18]
   140a69f4c:	48 85 f6             	test   rsi,rsi
   140a69f4f:	0f 84 bb 01 00 00    	je     0x140a6a110
   140a69f55:	48 f7 d6             	not    rsi
   140a69f58:	48 8d 34 76          	lea    rsi,[rsi+rsi*2]
   140a69f5c:	48 c1 e6 04          	shl    rsi,0x4
   140a69f60:	48 8b 5c 32 10       	mov    rbx,QWORD PTR [rdx+rsi*1+0x10]
   140a69f65:	48 8b b0 20 01 00 00 	mov    rsi,QWORD PTR [rax+0x120]
   140a69f6c:	49 39 f1             	cmp    r9,rsi
   140a69f6f:	0f 87 a9 01 00 00    	ja     0x140a6a11e
   140a69f75:	e9 d5 01 00 00       	jmp    0x140a6a14f
   140a69f7a:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
   140a69f80:	4d 39 e7             	cmp    r15,r12
   140a69f83:	0f 85 76 01 00 00    	jne    0x140a6a0ff
   140a69f89:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
   140a69f90:	4d 39 da             	cmp    r10,r11
   140a69f93:	0f 84 55 03 00 00    	je     0x140a6a2ee
   140a69f99:	41 8b 12             	mov    edx,DWORD PTR [r10]
   140a69f9c:	49 83 c2 04          	add    r10,0x4
   140a69fa0:	49 8b 88 00 01 00 00 	mov    rcx,QWORD PTR [r8+0x100]
   140a69fa7:	48 8d 14 92          	lea    rdx,[rdx+rdx*4]
   140a69fab:	48 c1 e2 05          	shl    rdx,0x5
   140a69faf:	4c 8b 64 11 58       	mov    r12,QWORD PTR [rcx+rdx*1+0x58]
   140a69fb4:	4d 85 e4             	test   r12,r12
   140a69fb7:	74 d7                	je     0x140a69f90
   140a69fb9:	4d 89 cf             	mov    r15,r9
   140a69fbc:	48 01 d1             	add    rcx,rdx
   140a69fbf:	8b 91 94 00 00 00    	mov    edx,DWORD PTR [rcx+0x94]
   140a69fc5:	49 8b b8 a8 01 00 00 	mov    rdi,QWORD PTR [r8+0x1a8]
   140a69fcc:	48 8d 1c d2          	lea    rbx,[rdx+rdx*8]
   140a69fd0:	4c 8d 0c df          	lea    r9,[rdi+rbx*8]
   140a69fd4:	48 8b b0 10 01 00 00 	mov    rsi,QWORD PTR [rax+0x110]
   140a69fdb:	48 8b 54 df 38       	mov    rdx,QWORD PTR [rdi+rbx*8+0x38]
   140a69fe0:	48 8b 5c df 40       	mov    rbx,QWORD PTR [rdi+rbx*8+0x40]
   140a69fe5:	48 39 f3             	cmp    rbx,rsi
   140a69fe8:	76 4d                	jbe    0x140a6a037
   140a69fea:	48 8b 3c f2          	mov    rdi,QWORD PTR [rdx+rsi*8]
   140a69fee:	49 8b 71 18          	mov    rsi,QWORD PTR [r9+0x18]
   140a69ff2:	48 85 ff             	test   rdi,rdi
   140a69ff5:	74 44                	je     0x140a6a03b
   140a69ff7:	48 f7 d7             	not    rdi
   140a69ffa:	4c 8d 0c 7f          	lea    r9,[rdi+rdi*2]
   140a69ffe:	49 c1 e1 04          	shl    r9,0x4
   140a6a002:	4e 8b 6c 0e 10       	mov    r13,QWORD PTR [rsi+r9*1+0x10]
   140a6a007:	4c 8b b0 20 01 00 00 	mov    r14,QWORD PTR [rax+0x120]
   140a6a00e:	4c 8b 89 88 00 00 00 	mov    r9,QWORD PTR [rcx+0x88]
   140a6a015:	4d 39 f1             	cmp    r9,r14
   140a6a018:	77 37                	ja     0x140a6a051
   140a6a01a:	eb 72                	jmp    0x140a6a08e
   140a6a01c:	48 8b 52 18          	mov    rdx,QWORD PTR [rdx+0x18]
   140a6a020:	31 db                	xor    ebx,ebx
   140a6a022:	48 8b b0 20 01 00 00 	mov    rsi,QWORD PTR [rax+0x120]
   140a6a029:	49 39 f1             	cmp    r9,rsi
   140a6a02c:	0f 86 1d 01 00 00    	jbe    0x140a6a14f
   140a6a032:	e9 e7 00 00 00       	jmp    0x140a6a11e
   140a6a037:	49 8b 71 18          	mov    rsi,QWORD PTR [r9+0x18]
   140a6a03b:	45 31 ed             	xor    r13d,r13d
   140a6a03e:	4c 8b b0 20 01 00 00 	mov    r14,QWORD PTR [rax+0x120]
   140a6a045:	4c 8b 89 88 00 00 00 	mov    r9,QWORD PTR [rcx+0x88]
   140a6a04c:	4d 39 f1             	cmp    r9,r14
   140a6a04f:	76 3d                	jbe    0x140a6a08e
   140a6a051:	48 8b b9 80 00 00 00 	mov    rdi,QWORD PTR [rcx+0x80]
   140a6a058:	4a 83 3c f7 00       	cmp    QWORD PTR [rdi+r14*8],0x0
   140a6a05d:	74 2f                	je     0x140a6a08e
   140a6a05f:	40 b7 01             	mov    dil,0x1
   140a6a062:	4c 39 f3             	cmp    rbx,r14
   140a6a065:	76 4b                	jbe    0x140a6a0b2
   140a6a067:	4a 8b 1c f2          	mov    rbx,QWORD PTR [rdx+r14*8]
   140a6a06b:	48 85 db             	test   rbx,rbx
   140a6a06e:	74 42                	je     0x140a6a0b2
   140a6a070:	48 f7 d3             	not    rbx
   140a6a073:	48 8d 1c 5b          	lea    rbx,[rbx+rbx*2]
   140a6a077:	48 c1 e3 04          	shl    rbx,0x4
   140a6a07b:	4c 8b 7c 1e 10       	mov    r15,QWORD PTR [rsi+rbx*1+0x10]
   140a6a080:	48 8b 98 28 01 00 00 	mov    rbx,QWORD PTR [rax+0x128]
   140a6a087:	49 39 d9             	cmp    r9,rbx
   140a6a08a:	77 10                	ja     0x140a6a09c
   140a6a08c:	eb 33                	jmp    0x140a6a0c1
   140a6a08e:	31 ff                	xor    edi,edi
   140a6a090:	48 8b 98 28 01 00 00 	mov    rbx,QWORD PTR [rax+0x128]
   140a6a097:	49 39 d9             	cmp    r9,rbx
   140a6a09a:	76 25                	jbe    0x140a6a0c1
   140a6a09c:	4c 8b 89 80 00 00 00 	mov    r9,QWORD PTR [rcx+0x80]
   140a6a0a3:	49 83 3c d9 00       	cmp    QWORD PTR [r9+rbx*8],0x0
   140a6a0a8:	41 0f 95 c1          	setne  r9b
   140a6a0ac:	44 89 4d 2c          	mov    DWORD PTR [rbp+0x2c],r9d
   140a6a0b0:	eb 16                	jmp    0x140a6a0c8
   140a6a0b2:	45 31 ff             	xor    r15d,r15d
   140a6a0b5:	48 8b 98 28 01 00 00 	mov    rbx,QWORD PTR [rax+0x128]
   140a6a0bc:	49 39 d9             	cmp    r9,rbx
   140a6a0bf:	77 db                	ja     0x140a6a09c
   140a6a0c1:	c7 45 2c 00 00 00 00 	mov    DWORD PTR [rbp+0x2c],0x0
   140a6a0c8:	4d 89 f9             	mov    r9,r15
   140a6a0cb:	48 8b 98 18 01 00 00 	mov    rbx,QWORD PTR [rax+0x118]
   140a6a0d2:	48 8b 14 da          	mov    rdx,QWORD PTR [rdx+rbx*8]
   140a6a0d6:	48 f7 d2             	not    rdx
   140a6a0d9:	48 8d 14 52          	lea    rdx,[rdx+rdx*2]
   140a6a0dd:	48 c1 e2 04          	shl    rdx,0x4
   140a6a0e1:	48 8b 5c 16 10       	mov    rbx,QWORD PTR [rsi+rdx*1+0x10]
   140a6a0e6:	48 89 5d 10          	mov    QWORD PTR [rbp+0x10],rbx
   140a6a0ea:	48 8b 54 16 28       	mov    rdx,QWORD PTR [rsi+rdx*1+0x28]
   140a6a0ef:	48 89 55 08          	mov    QWORD PTR [rbp+0x8],rdx
   140a6a0f3:	4c 8b 71 50          	mov    r14,QWORD PTR [rcx+0x50]
   140a6a0f7:	45 31 ff             	xor    r15d,r15d
   140a6a0fa:	4c 89 ee             	mov    rsi,r13
   140a6a0fd:	89 fb                	mov    ebx,edi
   140a6a0ff:	4c 89 f9             	mov    rcx,r15
   140a6a102:	48 c1 e1 04          	shl    rcx,0x4
   140a6a106:	41 8b 4c 0e 08       	mov    ecx,DWORD PTR [r14+rcx*1+0x8]
   140a6a10b:	e9 92 00 00 00       	jmp    0x140a6a1a2
   140a6a110:	31 db                	xor    ebx,ebx
   140a6a112:	48 8b b0 20 01 00 00 	mov    rsi,QWORD PTR [rax+0x120]
   140a6a119:	49 39 f1             	cmp    r9,rsi
   140a6a11c:	76 31                	jbe    0x140a6a14f
   140a6a11e:	48 8b 34 f1          	mov    rsi,QWORD PTR [rcx+rsi*8]
   140a6a122:	48 85 f6             	test   rsi,rsi
   140a6a125:	74 28                	je     0x140a6a14f
   140a6a127:	48 f7 d6             	not    rsi
   140a6a12a:	48 8d 34 76          	lea    rsi,[rsi+rsi*2]
   140a6a12e:	48 c1 e6 04          	shl    rsi,0x4
   140a6a132:	48 8b 7c 32 10       	mov    rdi,QWORD PTR [rdx+rsi*1+0x10]
   140a6a137:	41 b5 01             	mov    r13b,0x1
   140a6a13a:	48 8b b0 28 01 00 00 	mov    rsi,QWORD PTR [rax+0x128]
   140a6a141:	49 39 f1             	cmp    r9,rsi
   140a6a144:	77 18                	ja     0x140a6a15e
   140a6a146:	c7 45 2c 00 00 00 00 	mov    DWORD PTR [rbp+0x2c],0x0
   140a6a14d:	eb 1c                	jmp    0x140a6a16b
   140a6a14f:	45 31 ed             	xor    r13d,r13d
   140a6a152:	48 8b b0 28 01 00 00 	mov    rsi,QWORD PTR [rax+0x128]
   140a6a159:	49 39 f1             	cmp    r9,rsi
   140a6a15c:	76 e8                	jbe    0x140a6a146
   140a6a15e:	48 83 3c f1 00       	cmp    QWORD PTR [rcx+rsi*8],0x0
   140a6a163:	41 0f 95 c1          	setne  r9b
   140a6a167:	44 89 4d 2c          	mov    DWORD PTR [rbp+0x2c],r9d
   140a6a16b:	49 89 f9             	mov    r9,rdi
   140a6a16e:	48 8b b0 18 01 00 00 	mov    rsi,QWORD PTR [rax+0x118]
   140a6a175:	48 8b 0c f1          	mov    rcx,QWORD PTR [rcx+rsi*8]
   140a6a179:	48 f7 d1             	not    rcx
   140a6a17c:	48 8d 0c 49          	lea    rcx,[rcx+rcx*2]
   140a6a180:	48 c1 e1 04          	shl    rcx,0x4
   140a6a184:	48 8b 74 0a 10       	mov    rsi,QWORD PTR [rdx+rcx*1+0x10]
   140a6a189:	48 89 75 10          	mov    QWORD PTR [rbp+0x10],rsi
   140a6a18d:	48 8b 4c 0a 28       	mov    rcx,QWORD PTR [rdx+rcx*1+0x28]
   140a6a192:	48 89 4d 08          	mov    QWORD PTR [rbp+0x8],rcx
   140a6a196:	45 31 ff             	xor    r15d,r15d
   140a6a199:	48 89 de             	mov    rsi,rbx
   140a6a19c:	44 89 eb             	mov    ebx,r13d
   140a6a19f:	44 89 f9             	mov    ecx,r15d
   140a6a1a2:	48 8d 14 c9          	lea    rdx,[rcx+rcx*8]
   140a6a1a6:	4c 8d 2c 52          	lea    r13,[rdx+rdx*2]
   140a6a1aa:	49 01 f5             	add    r13,rsi
   140a6a1ad:	49 01 cd             	add    r13,rcx
   140a6a1b0:	48 8d 14 49          	lea    rdx,[rcx+rcx*2]
   140a6a1b4:	48 c1 e2 04          	shl    rdx,0x4
   140a6a1b8:	48 03 55 10          	add    rdx,QWORD PTR [rbp+0x10]
   140a6a1bc:	48 8b 7d 08          	mov    rdi,QWORD PTR [rbp+0x8]
   140a6a1c0:	48 8d 3c 8f          	lea    rdi,[rdi+rcx*4]
   140a6a1c4:	48 89 7d 00          	mov    QWORD PTR [rbp+0x0],rdi
   140a6a1c8:	48 c1 e1 04          	shl    rcx,0x4
   140a6a1cc:	4c 01 c9             	add    rcx,r9
   140a6a1cf:	f6 c3 01             	test   bl,0x1
   140a6a1d2:	bf 00 00 00 00       	mov    edi,0x0
   140a6a1d7:	48 0f 44 cf          	cmove  rcx,rdi
   140a6a1db:	41 80 7d 08 00       	cmp    BYTE PTR [r13+0x8],0x0
   140a6a1e0:	75 37                	jne    0x140a6a219
   140a6a1e2:	f3 41 0f 10 4d 0c    	movss  xmm1,DWORD PTR [r13+0xc]
   140a6a1e8:	f3 41 0f 10 45 18    	movss  xmm0,DWORD PTR [r13+0x18]
   140a6a1ee:	f3 0f 59 c1          	mulss  xmm0,xmm1
   140a6a1f2:	41 80 7d 09 00       	cmp    BYTE PTR [r13+0x9],0x0
   140a6a1f7:	0f 28 d0             	movaps xmm2,xmm0
   140a6a1fa:	75 03                	jne    0x140a6a1ff
   140a6a1fc:	0f 28 d1             	movaps xmm2,xmm1
   140a6a1ff:	44 8b 6d 1c          	mov    r13d,DWORD PTR [rbp+0x1c]
   140a6a203:	48 8b 7d 00          	mov    rdi,QWORD PTR [rbp+0x0]
   140a6a207:	44 89 2f             	mov    DWORD PTR [rdi],r13d
   140a6a20a:	f3 0f 11 52 1c       	movss  DWORD PTR [rdx+0x1c],xmm2
   140a6a20f:	f3 0f 11 42 20       	movss  DWORD PTR [rdx+0x20],xmm0
   140a6a214:	f3 0f 11 52 24       	movss  DWORD PTR [rdx+0x24],xmm2
   140a6a219:	49 ff c7             	inc    r15
   140a6a21c:	48 85 c9             	test   rcx,rcx
   140a6a21f:	48 8b 7d d0          	mov    rdi,QWORD PTR [rbp-0x30]
   140a6a223:	0f 84 c3 fc ff ff    	je     0x140a69eec
   140a6a229:	f6 45 2c 01          	test   BYTE PTR [rbp+0x2c],0x1
   140a6a22d:	0f 84 b9 fc ff ff    	je     0x140a69eec
   140a6a233:	80 7f 18 00          	cmp    BYTE PTR [rdi+0x18],0x0
   140a6a237:	0f 84 af fc ff ff    	je     0x140a69eec
   140a6a23d:	4c 8b 6f 38          	mov    r13,QWORD PTR [rdi+0x38]
   140a6a241:	4c 3b 69 08          	cmp    r13,QWORD PTR [rcx+0x8]
   140a6a245:	0f 85 a1 fc ff ff    	jne    0x140a69eec
   140a6a24b:	4c 8b 6f 20          	mov    r13,QWORD PTR [rdi+0x20]
   140a6a24f:	41 0f b6 8d 98 00 00 	movzx  ecx,BYTE PTR [r13+0x98]
   140a6a256:	00 
   140a6a257:	89 5d fc             	mov    DWORD PTR [rbp-0x4],ebx
   140a6a25a:	4c 89 f3             	mov    rbx,r14
   140a6a25d:	49 89 c6             	mov    r14,rax
   140a6a260:	48 89 f0             	mov    rax,rsi
   140a6a263:	4c 89 ce             	mov    rsi,r9
   140a6a266:	4c 8d 0d 3f b3 06 02 	lea    r9,[rip+0x206b33f]        # 0x142ad55ac
   140a6a26d:	49 63 0c 89          	movsxd rcx,DWORD PTR [r9+rcx*4]
   140a6a271:	4c 01 c9             	add    rcx,r9
   140a6a274:	49 89 f1             	mov    r9,rsi
   140a6a277:	48 89 c6             	mov    rsi,rax
   140a6a27a:	4c 89 f0             	mov    rax,r14
   140a6a27d:	49 89 de             	mov    r14,rbx
   140a6a280:	8b 5d fc             	mov    ebx,DWORD PTR [rbp-0x4]
   140a6a283:	ff e1                	jmp    rcx
   140a6a285:	b9 08 00 00 00       	mov    ecx,0x8
   140a6a28a:	eb 05                	jmp    0x140a6a291
   140a6a28c:	b9 04 00 00 00       	mov    ecx,0x4
   140a6a291:	49 81 c5 98 00 00 00 	add    r13,0x98
   140a6a298:	49 01 cd             	add    r13,rcx
   140a6a29b:	eb 26                	jmp    0x140a6a2c3
   140a6a29d:	48 8b 4f 40          	mov    rcx,QWORD PTR [rdi+0x40]
   140a6a2a1:	49 8b bd b0 00 00 00 	mov    rdi,QWORD PTR [r13+0xb0]
   140a6a2a8:	48 39 cf             	cmp    rdi,rcx
   140a6a2ab:	0f 86 d8 00 00 00    	jbe    0x140a6a389
   140a6a2b1:	48 8d 0c 49          	lea    rcx,[rcx+rcx*2]
   140a6a2b5:	48 c1 e1 02          	shl    rcx,0x2
   140a6a2b9:	49 03 8d a8 00 00 00 	add    rcx,QWORD PTR [r13+0xa8]
   140a6a2c0:	49 89 cd             	mov    r13,rcx
   140a6a2c3:	41 8b 4d 08          	mov    ecx,DWORD PTR [r13+0x8]
   140a6a2c7:	89 4d e0             	mov    DWORD PTR [rbp-0x20],ecx
   140a6a2ca:	49 8b 4d 00          	mov    rcx,QWORD PTR [r13+0x0]
   140a6a2ce:	48 89 4d d8          	mov    QWORD PTR [rbp-0x28],rcx
   140a6a2d2:	8b 4d 1c             	mov    ecx,DWORD PTR [rbp+0x1c]
   140a6a2d5:	48 8b 7d 00          	mov    rdi,QWORD PTR [rbp+0x0]
   140a6a2d9:	89 0f                	mov    DWORD PTR [rdi],ecx
   140a6a2db:	8b 4d e0             	mov    ecx,DWORD PTR [rbp-0x20]
   140a6a2de:	89 4a 18             	mov    DWORD PTR [rdx+0x18],ecx
   140a6a2e1:	48 8b 4d d8          	mov    rcx,QWORD PTR [rbp-0x28]
   140a6a2e5:	48 89 4a 10          	mov    QWORD PTR [rdx+0x10],rcx
   140a6a2e9:	e9 fe fb ff ff       	jmp    0x140a69eec
   140a6a2ee:	48 83 7d e8 00       	cmp    QWORD PTR [rbp-0x18],0x0
   140a6a2f3:	74 3a                	je     0x140a6a32f
   140a6a2f5:	e8 36 de 1a 00       	call   0x140c18130
   140a6a2fa:	48 89 c1             	mov    rcx,rax
   140a6a2fd:	48 8b 00             	mov    rax,QWORD PTR [rax]
   140a6a300:	48 83 f8 01          	cmp    rax,0x1
   140a6a304:	75 3d                	jne    0x140a6a343
   140a6a306:	48 83 c1 08          	add    rcx,0x8
   140a6a30a:	48 83 39 00          	cmp    QWORD PTR [rcx],0x0
   140a6a30e:	75 6d                	jne    0x140a6a37d
   140a6a310:	48 c7 01 ff ff ff ff 	mov    QWORD PTR [rcx],0xffffffffffffffff
   140a6a317:	48 89 4d 20          	mov    QWORD PTR [rbp+0x20],rcx
   140a6a31b:	48 83 c1 08          	add    rcx,0x8
   140a6a31f:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
   140a6a323:	e8 c8 cd 1a 00       	call   0x140c170f0
   140a6a328:	48 8b 45 20          	mov    rax,QWORD PTR [rbp+0x20]
   140a6a32c:	48 ff 00             	inc    QWORD PTR [rax]
   140a6a32f:	48 81 c4 b8 00 00 00 	add    rsp,0xb8
   140a6a336:	5b                   	pop    rbx
   140a6a337:	5f                   	pop    rdi
   140a6a338:	5e                   	pop    rsi
   140a6a339:	41 5c                	pop    r12
   140a6a33b:	41 5d                	pop    r13
   140a6a33d:	41 5e                	pop    r14
   140a6a33f:	41 5f                	pop    r15
   140a6a341:	5d                   	pop    rbp
   140a6a342:	c3                   	ret
   140a6a343:	48 85 c0             	test   rax,rax
   140a6a346:	75 29                	jne    0x140a6a371
   140a6a348:	31 d2                	xor    edx,edx
   140a6a34a:	e8 81 e4 e8 01       	call   0x1428f87d0
   140a6a34f:	48 89 c1             	mov    rcx,rax
   140a6a352:	48 85 c0             	test   rax,rax
   140a6a355:	75 b3                	jne    0x140a6a30a
   140a6a357:	eb 18                	jmp    0x140a6a371
   140a6a359:	48 85 c0             	test   rax,rax
   140a6a35c:	75 13                	jne    0x140a6a371
   140a6a35e:	31 d2                	xor    edx,edx
   140a6a360:	e8 6b e4 e8 01       	call   0x1428f87d0
   140a6a365:	48 89 c1             	mov    rcx,rax
   140a6a368:	48 85 c0             	test   rax,rax
   140a6a36b:	0f 85 d7 fa ff ff    	jne    0x140a69e48
   140a6a371:	48 8d 0d 28 07 1d 02 	lea    rcx,[rip+0x21d0728]        # 0x142c3aaa0
   140a6a378:	e8 53 dd e6 01       	call   0x1428d80d0
   140a6a37d:	48 8d 0d ec 07 1d 02 	lea    rcx,[rip+0x21d07ec]        # 0x142c3ab70
   140a6a384:	e8 c7 ee e6 01       	call   0x1428d9250
   140a6a389:	4c 8d 05 98 35 05 02 	lea    r8,[rip+0x2053598]        # 0x142abd928
   140a6a390:	48 89 fa             	mov    rdx,rdi
   140a6a393:	e8 80 f1 e6 01       	call   0x1428d9518
   140a6a398:	0f 0b                	ud2

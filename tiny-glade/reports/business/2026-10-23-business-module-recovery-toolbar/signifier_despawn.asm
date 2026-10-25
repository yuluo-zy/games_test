
tiny-glade.exe:     file format pei-x86-64


Disassembly of section .text:

0000000140929a90 <.text+0x928a90>:
   140929a90:	55                   	push   rbp
   140929a91:	41 57                	push   r15
   140929a93:	41 56                	push   r14
   140929a95:	41 55                	push   r13
   140929a97:	41 54                	push   r12
   140929a99:	56                   	push   rsi
   140929a9a:	57                   	push   rdi
   140929a9b:	53                   	push   rbx
   140929a9c:	48 81 ec f8 00 00 00 	sub    rsp,0xf8
   140929aa3:	48 8d ac 24 80 00 00 	lea    rbp,[rsp+0x80]
   140929aaa:	00 
   140929aab:	0f 29 75 60          	movaps XMMWORD PTR [rbp+0x60],xmm6
   140929aaf:	48 c7 45 58 fe ff ff 	mov    QWORD PTR [rbp+0x58],0xfffffffffffffffe
   140929ab6:	ff 
   140929ab7:	48 89 d7             	mov    rdi,rdx
   140929aba:	48 89 cb             	mov    rbx,rcx
   140929abd:	e8 3e cf 2e 00       	call   0x140c16a00
   140929ac2:	84 c0                	test   al,al
   140929ac4:	88 45 57             	mov    BYTE PTR [rbp+0x57],al
   140929ac7:	74 5f                	je     0x140929b28
   140929ac9:	4c 8d 35 95 4b 18 02 	lea    r14,[rip+0x2184b95]        # 0x142aae665
   140929ad0:	be 50 00 00 00       	mov    esi,0x50
   140929ad5:	ba 50 00 00 00       	mov    edx,0x50
   140929ada:	4c 89 f1             	mov    rcx,r14
   140929add:	e8 3e 9e ff ff       	call   0x140923920
   140929ae2:	48 83 f8 01          	cmp    rax,0x1
   140929ae6:	0f 85 b1 00 00 00    	jne    0x140929b9d
   140929aec:	49 89 d1             	mov    r9,rdx
   140929aef:	48 85 d2             	test   rdx,rdx
   140929af2:	74 3d                	je     0x140929b31
   140929af4:	49 83 f9 50          	cmp    r9,0x50
   140929af8:	73 35                	jae    0x140929b2f
   140929afa:	48 8d 05 64 4b 18 02 	lea    rax,[rip+0x2184b64]        # 0x142aae665
   140929b01:	41 80 3c 01 bf       	cmp    BYTE PTR [r9+rax*1],0xbf
   140929b06:	7f 29                	jg     0x140929b31
   140929b08:	48 8d 05 29 4c 18 02 	lea    rax,[rip+0x2184c29]        # 0x142aae738
   140929b0f:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140929b14:	48 8d 0d 4a 4b 18 02 	lea    rcx,[rip+0x2184b4a]        # 0x142aae665
   140929b1b:	ba 50 00 00 00       	mov    edx,0x50
   140929b20:	45 31 c0             	xor    r8d,r8d
   140929b23:	e8 18 fe fa 01       	call   0x1428d9940
   140929b28:	31 c0                	xor    eax,eax
   140929b2a:	e9 cb 01 00 00       	jmp    0x140929cfa
   140929b2f:	75 d7                	jne    0x140929b08
   140929b31:	4c 8d 35 2d 4b 18 02 	lea    r14,[rip+0x2184b2d]        # 0x142aae665
   140929b38:	4c 89 f1             	mov    rcx,r14
   140929b3b:	4c 89 ca             	mov    rdx,r9
   140929b3e:	e8 dd 9d ff ff       	call   0x140923920
   140929b43:	48 83 f8 01          	cmp    rax,0x1
   140929b47:	75 54                	jne    0x140929b9d
   140929b49:	49 89 d0             	mov    r8,rdx
   140929b4c:	49 83 c0 02          	add    r8,0x2
   140929b50:	74 39                	je     0x140929b8b
   140929b52:	49 83 f8 50          	cmp    r8,0x50
   140929b56:	73 31                	jae    0x140929b89
   140929b58:	48 8d 05 06 4b 18 02 	lea    rax,[rip+0x2184b06]        # 0x142aae665
   140929b5f:	41 80 3c 00 bf       	cmp    BYTE PTR [r8+rax*1],0xbf
   140929b64:	7f 25                	jg     0x140929b8b
   140929b66:	48 8d 05 e3 4b 18 02 	lea    rax,[rip+0x2184be3]        # 0x142aae750
   140929b6d:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140929b72:	48 8d 0d ec 4a 18 02 	lea    rcx,[rip+0x2184aec]        # 0x142aae665
   140929b79:	ba 50 00 00 00       	mov    edx,0x50
   140929b7e:	41 b9 50 00 00 00    	mov    r9d,0x50
   140929b84:	e8 b7 fd fa 01       	call   0x1428d9940
   140929b89:	75 db                	jne    0x140929b66
   140929b8b:	be 4e 00 00 00       	mov    esi,0x4e
   140929b90:	48 29 d6             	sub    rsi,rdx
   140929b93:	4c 8d 35 cb 4a 18 02 	lea    r14,[rip+0x2184acb]        # 0x142aae665
   140929b9a:	4d 01 c6             	add    r14,r8
   140929b9d:	48 8d 05 e2 59 18 02 	lea    rax,[rip+0x21859e2]        # 0x142aaf586
   140929ba4:	4c 8d 3d 91 59 18 02 	lea    r15,[rip+0x2185991]        # 0x142aaf53c
   140929bab:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
   140929bb0:	0f be 48 ff          	movsx  ecx,BYTE PTR [rax-0x1]
   140929bb4:	85 c9                	test   ecx,ecx
   140929bb6:	78 18                	js     0x140929bd0
   140929bb8:	48 ff c8             	dec    rax
   140929bbb:	83 f9 5c             	cmp    ecx,0x5c
   140929bbe:	75 69                	jne    0x140929c29
   140929bc0:	eb 7d                	jmp    0x140929c3f
   140929bc2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   140929bc9:	1f 84 00 00 00 00 00 
   140929bd0:	0f b6 50 fe          	movzx  edx,BYTE PTR [rax-0x2]
   140929bd4:	80 fa c0             	cmp    dl,0xc0
   140929bd7:	7d 25                	jge    0x140929bfe
   140929bd9:	44 0f b6 40 fd       	movzx  r8d,BYTE PTR [rax-0x3]
   140929bde:	41 80 f8 c0          	cmp    r8b,0xc0
   140929be2:	7d 23                	jge    0x140929c07
   140929be4:	44 0f b6 48 fc       	movzx  r9d,BYTE PTR [rax-0x4]
   140929be9:	48 83 c0 fc          	add    rax,0xfffffffffffffffc
   140929bed:	41 83 e1 07          	and    r9d,0x7
   140929bf1:	41 c1 e1 06          	shl    r9d,0x6
   140929bf5:	41 83 e0 3f          	and    r8d,0x3f
   140929bf9:	45 09 c8             	or     r8d,r9d
   140929bfc:	eb 11                	jmp    0x140929c0f
   140929bfe:	48 83 c0 fe          	add    rax,0xfffffffffffffffe
   140929c02:	83 e2 1f             	and    edx,0x1f
   140929c05:	eb 12                	jmp    0x140929c19
   140929c07:	48 83 c0 fd          	add    rax,0xfffffffffffffffd
   140929c0b:	41 83 e0 0f          	and    r8d,0xf
   140929c0f:	41 c1 e0 06          	shl    r8d,0x6
   140929c13:	83 e2 3f             	and    edx,0x3f
   140929c16:	44 09 c2             	or     edx,r8d
   140929c19:	c1 e2 06             	shl    edx,0x6
   140929c1c:	80 e1 3f             	and    cl,0x3f
   140929c1f:	0f b6 c9             	movzx  ecx,cl
   140929c22:	09 d1                	or     ecx,edx
   140929c24:	83 f9 5c             	cmp    ecx,0x5c
   140929c27:	74 16                	je     0x140929c3f
   140929c29:	83 f9 2f             	cmp    ecx,0x2f
   140929c2c:	74 11                	je     0x140929c3f
   140929c2e:	4c 39 f8             	cmp    rax,r15
   140929c31:	0f 85 79 ff ff ff    	jne    0x140929bb0
   140929c37:	41 bc 4a 00 00 00    	mov    r12d,0x4a
   140929c3d:	eb 50                	jmp    0x140929c8f
   140929c3f:	4c 8d 3d f6 58 18 02 	lea    r15,[rip+0x21858f6]        # 0x142aaf53c
   140929c46:	4c 29 f8             	sub    rax,r15
   140929c49:	49 89 c0             	mov    r8,rax
   140929c4c:	49 ff c0             	inc    r8
   140929c4f:	74 32                	je     0x140929c83
   140929c51:	49 83 f8 4a          	cmp    r8,0x4a
   140929c55:	73 2a                	jae    0x140929c81
   140929c57:	43 80 3c 38 bf       	cmp    BYTE PTR [r8+r15*1],0xbf
   140929c5c:	7f 25                	jg     0x140929c83
   140929c5e:	48 8d 05 b3 4a 18 02 	lea    rax,[rip+0x2184ab3]        # 0x142aae718
   140929c65:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140929c6a:	48 8d 0d cb 58 18 02 	lea    rcx,[rip+0x21858cb]        # 0x142aaf53c
   140929c71:	ba 4a 00 00 00       	mov    edx,0x4a
   140929c76:	41 b9 4a 00 00 00    	mov    r9d,0x4a
   140929c7c:	e8 bf fc fa 01       	call   0x1428d9940
   140929c81:	75 db                	jne    0x140929c5e
   140929c83:	41 bc 49 00 00 00    	mov    r12d,0x49
   140929c89:	49 29 c4             	sub    r12,rax
   140929c8c:	4d 01 c7             	add    r15,r8
   140929c8f:	e8 9c e4 2e 00       	call   0x140c18130
   140929c94:	48 89 c1             	mov    rcx,rax
   140929c97:	48 8b 00             	mov    rax,QWORD PTR [rax]
   140929c9a:	48 83 f8 01          	cmp    rax,0x1
   140929c9e:	0f 85 27 03 00 00    	jne    0x140929fcb
   140929ca4:	48 83 c1 08          	add    rcx,0x8
   140929ca8:	48 83 39 00          	cmp    QWORD PTR [rcx],0x0
   140929cac:	0f 85 53 03 00 00    	jne    0x14092a005
   140929cb2:	48 c7 01 ff ff ff ff 	mov    QWORD PTR [rcx],0xffffffffffffffff
   140929cb9:	48 89 4d 48          	mov    QWORD PTR [rbp+0x48],rcx
   140929cbd:	48 83 c1 08          	add    rcx,0x8
   140929cc1:	4c 89 64 24 20       	mov    QWORD PTR [rsp+0x20],r12
   140929cc6:	48 c7 44 24 30 00 00 	mov    QWORD PTR [rsp+0x30],0x0
   140929ccd:	00 00 
   140929ccf:	48 c7 44 24 28 01 00 	mov    QWORD PTR [rsp+0x28],0x1
   140929cd6:	00 00 
   140929cd8:	4c 89 f2             	mov    rdx,r14
   140929cdb:	49 89 f0             	mov    r8,rsi
   140929cde:	4d 89 f9             	mov    r9,r15
   140929ce1:	e8 5a d0 2e 00       	call   0x140c16d40
   140929ce6:	48 8b 4d 48          	mov    rcx,QWORD PTR [rbp+0x48]
   140929cea:	48 ff 01             	inc    QWORD PTR [rcx]
   140929ced:	48 89 45 20          	mov    QWORD PTR [rbp+0x20],rax
   140929cf1:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
   140929cf5:	b8 01 00 00 00       	mov    eax,0x1
   140929cfa:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
   140929cfe:	48 8b 0b             	mov    rcx,QWORD PTR [rbx]
   140929d01:	4c 8b 7b 08          	mov    r15,QWORD PTR [rbx+0x8]
   140929d05:	4c 8b a1 f8 00 00 00 	mov    r12,QWORD PTR [rcx+0xf8]
   140929d0c:	48 8b 81 00 01 00 00 	mov    rax,QWORD PTR [rcx+0x100]
   140929d13:	4d 8d 2c 84          	lea    r13,[r12+rax*4]
   140929d17:	48 89 4d 38          	mov    QWORD PTR [rbp+0x38],rcx
   140929d1b:	0f b6 b1 08 01 00 00 	movzx  esi,BYTE PTR [rcx+0x108]
   140929d22:	48 8b 07             	mov    rax,QWORD PTR [rdi]
   140929d25:	48 89 45 48          	mov    QWORD PTR [rbp+0x48],rax
   140929d29:	48 8b 47 08          	mov    rax,QWORD PTR [rdi+0x8]
   140929d2d:	48 89 45 08          	mov    QWORD PTR [rbp+0x8],rax
   140929d31:	48 8b 47 18          	mov    rax,QWORD PTR [rdi+0x18]
   140929d35:	48 89 45 10          	mov    QWORD PTR [rbp+0x10],rax
   140929d39:	48 8b 47 10          	mov    rax,QWORD PTR [rdi+0x10]
   140929d3d:	48 89 45 18          	mov    QWORD PTR [rbp+0x18],rax
   140929d41:	b8 08 00 00 00       	mov    eax,0x8
   140929d46:	48 89 45 30          	mov    QWORD PTR [rbp+0x30],rax
   140929d4a:	31 db                	xor    ebx,ebx
   140929d4c:	0f 57 f6             	xorps  xmm6,xmm6
   140929d4f:	48 89 45 28          	mov    QWORD PTR [rbp+0x28],rax
   140929d53:	45 31 f6             	xor    r14d,r14d
   140929d56:	48 c7 45 40 00 00 00 	mov    QWORD PTR [rbp+0x40],0x0
   140929d5d:	00 
   140929d5e:	31 ff                	xor    edi,edi
   140929d60:	eb 45                	jmp    0x140929da7
   140929d62:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   140929d69:	1f 84 00 00 00 00 00 
   140929d70:	49 ff c6             	inc    r14
   140929d73:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
   140929d77:	48 8b 45 48          	mov    rax,QWORD PTR [rbp+0x48]
   140929d7b:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
   140929d7f:	48 8b 45 08          	mov    rax,QWORD PTR [rbp+0x8]
   140929d83:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
   140929d87:	48 8b 45 18          	mov    rax,QWORD PTR [rbp+0x18]
   140929d8b:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
   140929d8f:	48 8b 45 10          	mov    rax,QWORD PTR [rbp+0x10]
   140929d93:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
   140929d97:	48 8d 4d c0          	lea    rcx,[rbp-0x40]
   140929d9b:	48 8d 15 e6 57 18 02 	lea    rdx,[rip+0x21857e6]        # 0x142aaf588
   140929da2:	e8 c9 a9 3e 00       	call   0x140d14770
   140929da7:	40 84 f6             	test   sil,sil
   140929daa:	0f 84 80 00 00 00    	je     0x140929e30
   140929db0:	49 39 de             	cmp    r14,rbx
   140929db3:	0f 85 3a 01 00 00    	jne    0x140929ef3
   140929db9:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
   140929dc0:	4d 39 ec             	cmp    r12,r13
   140929dc3:	0f 84 aa 01 00 00    	je     0x140929f73
   140929dc9:	41 8b 0c 24          	mov    ecx,DWORD PTR [r12]
   140929dcd:	49 83 c4 04          	add    r12,0x4
   140929dd1:	49 8b 87 a8 01 00 00 	mov    rax,QWORD PTR [r15+0x1a8]
   140929dd8:	48 8d 0c c9          	lea    rcx,[rcx+rcx*8]
   140929ddc:	48 8b 5c c8 10       	mov    rbx,QWORD PTR [rax+rcx*8+0x10]
   140929de1:	48 85 db             	test   rbx,rbx
   140929de4:	74 da                	je     0x140929dc0
   140929de6:	48 8d 04 c8          	lea    rax,[rax+rcx*8]
   140929dea:	48 8b 4d 38          	mov    rcx,QWORD PTR [rbp+0x38]
   140929dee:	48 8b 89 18 01 00 00 	mov    rcx,QWORD PTR [rcx+0x118]
   140929df5:	48 39 48 40          	cmp    QWORD PTR [rax+0x40],rcx
   140929df9:	0f 86 e7 00 00 00    	jbe    0x140929ee6
   140929dff:	48 8b 50 38          	mov    rdx,QWORD PTR [rax+0x38]
   140929e03:	48 8b 0c ca          	mov    rcx,QWORD PTR [rdx+rcx*8]
   140929e07:	48 85 c9             	test   rcx,rcx
   140929e0a:	0f 84 d6 00 00 00    	je     0x140929ee6
   140929e10:	48 8b 50 18          	mov    rdx,QWORD PTR [rax+0x18]
   140929e14:	48 f7 d1             	not    rcx
   140929e17:	48 8d 0c 49          	lea    rcx,[rcx+rcx*2]
   140929e1b:	48 c1 e1 04          	shl    rcx,0x4
   140929e1f:	48 8b 4c 0a 10       	mov    rcx,QWORD PTR [rdx+rcx*1+0x10]
   140929e24:	48 89 4d 40          	mov    QWORD PTR [rbp+0x40],rcx
   140929e28:	40 b7 01             	mov    dil,0x1
   140929e2b:	e9 b8 00 00 00       	jmp    0x140929ee8
   140929e30:	49 39 de             	cmp    r14,rbx
   140929e33:	0f 85 a7 00 00 00    	jne    0x140929ee0
   140929e39:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
   140929e40:	4d 39 ec             	cmp    r12,r13
   140929e43:	0f 84 2a 01 00 00    	je     0x140929f73
   140929e49:	41 8b 0c 24          	mov    ecx,DWORD PTR [r12]
   140929e4d:	49 83 c4 04          	add    r12,0x4
   140929e51:	49 8b 87 00 01 00 00 	mov    rax,QWORD PTR [r15+0x100]
   140929e58:	48 8d 0c 89          	lea    rcx,[rcx+rcx*4]
   140929e5c:	48 c1 e1 05          	shl    rcx,0x5
   140929e60:	48 8b 5c 08 58       	mov    rbx,QWORD PTR [rax+rcx*1+0x58]
   140929e65:	48 85 db             	test   rbx,rbx
   140929e68:	74 d6                	je     0x140929e40
   140929e6a:	48 01 c8             	add    rax,rcx
   140929e6d:	48 8b 4d 38          	mov    rcx,QWORD PTR [rbp+0x38]
   140929e71:	48 8b 89 18 01 00 00 	mov    rcx,QWORD PTR [rcx+0x118]
   140929e78:	48 39 88 88 00 00 00 	cmp    QWORD PTR [rax+0x88],rcx
   140929e7f:	0f 86 85 00 00 00    	jbe    0x140929f0a
   140929e85:	48 8b 90 80 00 00 00 	mov    rdx,QWORD PTR [rax+0x80]
   140929e8c:	48 83 3c ca 00       	cmp    QWORD PTR [rdx+rcx*8],0x0
   140929e91:	74 77                	je     0x140929f0a
   140929e93:	44 8b 80 94 00 00 00 	mov    r8d,DWORD PTR [rax+0x94]
   140929e9a:	49 8b 97 a8 01 00 00 	mov    rdx,QWORD PTR [r15+0x1a8]
   140929ea1:	4f 8d 04 c0          	lea    r8,[r8+r8*8]
   140929ea5:	40 b7 01             	mov    dil,0x1
   140929ea8:	4a 39 4c c2 40       	cmp    QWORD PTR [rdx+r8*8+0x40],rcx
   140929ead:	76 5f                	jbe    0x140929f0e
   140929eaf:	4e 8b 4c c2 38       	mov    r9,QWORD PTR [rdx+r8*8+0x38]
   140929eb4:	49 8b 0c c9          	mov    rcx,QWORD PTR [r9+rcx*8]
   140929eb8:	48 85 c9             	test   rcx,rcx
   140929ebb:	74 51                	je     0x140929f0e
   140929ebd:	4a 8d 14 c2          	lea    rdx,[rdx+r8*8]
   140929ec1:	48 8b 52 18          	mov    rdx,QWORD PTR [rdx+0x18]
   140929ec5:	48 f7 d1             	not    rcx
   140929ec8:	48 8d 0c 49          	lea    rcx,[rcx+rcx*2]
   140929ecc:	48 c1 e1 04          	shl    rcx,0x4
   140929ed0:	48 8b 4c 0a 10       	mov    rcx,QWORD PTR [rdx+rcx*1+0x10]
   140929ed5:	48 89 4d 40          	mov    QWORD PTR [rbp+0x40],rcx
   140929ed9:	eb 3b                	jmp    0x140929f16
   140929edb:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
   140929ee0:	48 8b 45 28          	mov    rax,QWORD PTR [rbp+0x28]
   140929ee4:	eb 37                	jmp    0x140929f1d
   140929ee6:	31 ff                	xor    edi,edi
   140929ee8:	48 8b 40 08          	mov    rax,QWORD PTR [rax+0x8]
   140929eec:	48 89 45 30          	mov    QWORD PTR [rbp+0x30],rax
   140929ef0:	45 31 f6             	xor    r14d,r14d
   140929ef3:	48 8b 45 30          	mov    rax,QWORD PTR [rbp+0x30]
   140929ef7:	4a 8b 04 f0          	mov    rax,QWORD PTR [rax+r14*8]
   140929efb:	40 f6 c7 01          	test   dil,0x1
   140929eff:	0f 84 6b fe ff ff    	je     0x140929d70
   140929f05:	44 89 f1             	mov    ecx,r14d
   140929f08:	eb 35                	jmp    0x140929f3f
   140929f0a:	31 ff                	xor    edi,edi
   140929f0c:	eb 08                	jmp    0x140929f16
   140929f0e:	48 c7 45 40 00 00 00 	mov    QWORD PTR [rbp+0x40],0x0
   140929f15:	00 
   140929f16:	48 8b 40 50          	mov    rax,QWORD PTR [rax+0x50]
   140929f1a:	45 31 f6             	xor    r14d,r14d
   140929f1d:	4c 89 f1             	mov    rcx,r14
   140929f20:	48 c1 e1 04          	shl    rcx,0x4
   140929f24:	48 89 c2             	mov    rdx,rax
   140929f27:	48 8b 04 08          	mov    rax,QWORD PTR [rax+rcx*1]
   140929f2b:	40 f6 c7 01          	test   dil,0x1
   140929f2f:	48 89 55 28          	mov    QWORD PTR [rbp+0x28],rdx
   140929f33:	0f 84 37 fe ff ff    	je     0x140929d70
   140929f39:	48 01 d1             	add    rcx,rdx
   140929f3c:	8b 49 08             	mov    ecx,DWORD PTR [rcx+0x8]
   140929f3f:	48 8d 14 c9          	lea    rdx,[rcx+rcx*8]
   140929f43:	48 8d 14 52          	lea    rdx,[rdx+rdx*2]
   140929f47:	48 03 4d 40          	add    rcx,QWORD PTR [rbp+0x40]
   140929f4b:	48 01 d1             	add    rcx,rdx
   140929f4e:	49 ff c6             	inc    r14
   140929f51:	48 85 c9             	test   rcx,rcx
   140929f54:	0f 84 19 fe ff ff    	je     0x140929d73
   140929f5a:	f3 0f 10 41 14       	movss  xmm0,DWORD PTR [rcx+0x14]
   140929f5f:	0f 2e c6             	ucomiss xmm0,xmm6
   140929f62:	0f 85 3f fe ff ff    	jne    0x140929da7
   140929f68:	0f 8b 05 fe ff ff    	jnp    0x140929d73
   140929f6e:	e9 34 fe ff ff       	jmp    0x140929da7
   140929f73:	80 7d 57 00          	cmp    BYTE PTR [rbp+0x57],0x0
   140929f77:	74 3a                	je     0x140929fb3
   140929f79:	e8 b2 e1 2e 00       	call   0x140c18130
   140929f7e:	48 89 c1             	mov    rcx,rax
   140929f81:	48 8b 00             	mov    rax,QWORD PTR [rax]
   140929f84:	48 83 f8 01          	cmp    rax,0x1
   140929f88:	75 5b                	jne    0x140929fe5
   140929f8a:	48 83 c1 08          	add    rcx,0x8
   140929f8e:	48 83 39 00          	cmp    QWORD PTR [rcx],0x0
   140929f92:	75 71                	jne    0x14092a005
   140929f94:	48 c7 01 ff ff ff ff 	mov    QWORD PTR [rcx],0xffffffffffffffff
   140929f9b:	48 89 4d 48          	mov    QWORD PTR [rbp+0x48],rcx
   140929f9f:	48 83 c1 08          	add    rcx,0x8
   140929fa3:	48 8b 55 20          	mov    rdx,QWORD PTR [rbp+0x20]
   140929fa7:	e8 44 d1 2e 00       	call   0x140c170f0
   140929fac:	48 8b 45 48          	mov    rax,QWORD PTR [rbp+0x48]
   140929fb0:	48 ff 00             	inc    QWORD PTR [rax]
   140929fb3:	0f 28 75 60          	movaps xmm6,XMMWORD PTR [rbp+0x60]
   140929fb7:	48 81 c4 f8 00 00 00 	add    rsp,0xf8
   140929fbe:	5b                   	pop    rbx
   140929fbf:	5f                   	pop    rdi
   140929fc0:	5e                   	pop    rsi
   140929fc1:	41 5c                	pop    r12
   140929fc3:	41 5d                	pop    r13
   140929fc5:	41 5e                	pop    r14
   140929fc7:	41 5f                	pop    r15
   140929fc9:	5d                   	pop    rbp
   140929fca:	c3                   	ret
   140929fcb:	48 85 c0             	test   rax,rax
   140929fce:	75 29                	jne    0x140929ff9
   140929fd0:	31 d2                	xor    edx,edx
   140929fd2:	e8 f9 e7 fc 01       	call   0x1428f87d0
   140929fd7:	48 89 c1             	mov    rcx,rax
   140929fda:	48 85 c0             	test   rax,rax
   140929fdd:	0f 85 c5 fc ff ff    	jne    0x140929ca8
   140929fe3:	eb 14                	jmp    0x140929ff9
   140929fe5:	48 85 c0             	test   rax,rax
   140929fe8:	75 0f                	jne    0x140929ff9
   140929fea:	31 d2                	xor    edx,edx
   140929fec:	e8 df e7 fc 01       	call   0x1428f87d0
   140929ff1:	48 89 c1             	mov    rcx,rax
   140929ff4:	48 85 c0             	test   rax,rax
   140929ff7:	75 95                	jne    0x140929f8e
   140929ff9:	48 8d 0d a0 0a 31 02 	lea    rcx,[rip+0x2310aa0]        # 0x142c3aaa0
   14092a000:	e8 cb e0 fa 01       	call   0x1428d80d0
   14092a005:	48 8d 0d 64 0b 31 02 	lea    rcx,[rip+0x2310b64]        # 0x142c3ab70
   14092a00c:	e8 3f f2 fa 01       	call   0x1428d9250
   14092a011:	cc                   	int3

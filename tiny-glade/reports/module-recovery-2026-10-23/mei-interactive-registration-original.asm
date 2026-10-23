
Tiny Glade/tiny-glade.exe:     file format pei-x86-64


Disassembly of section .text:

0000000140684c98 <.text+0x683c98>:
   140684c98:	f6 c1 02             	test   cl,0x2
   140684c9b:	75 33                	jne    0x140684cd0
   140684c9d:	b0 01                	mov    al,0x1
   140684c9f:	f6 c1 01             	test   cl,0x1
   140684ca2:	75 2e                	jne    0x140684cd2
   140684ca4:	49 8b 4d c8          	mov    rcx,QWORD PTR [r13-0x38]
   140684ca8:	48 85 c9             	test   rcx,rcx
   140684cab:	74 23                	je     0x140684cd0
   140684cad:	44 89 c3             	mov    ebx,r8d
   140684cb0:	48 89 d7             	mov    rdi,rdx
   140684cb3:	49 8b 45 d0          	mov    rax,QWORD PTR [r13-0x30]
   140684cb7:	ff 50 40             	call   QWORD PTR [rax+0x40]
   140684cba:	48 89 fa             	mov    rdx,rdi
   140684cbd:	41 89 d8             	mov    r8d,ebx
   140684cc0:	eb 10                	jmp    0x140684cd2
   140684cc2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   140684cc9:	1f 84 00 00 00 00 00 
   140684cd0:	31 c0                	xor    eax,eax
   140684cd2:	f2 45 0f 10 5d b8    	movsd  xmm11,QWORD PTR [r13-0x48]
   140684cd8:	f2 45 0f 10 55 c0    	movsd  xmm10,QWORD PTR [r13-0x40]
   140684cde:	45 0f 59 d0          	mulps  xmm10,xmm8
   140684ce2:	45 0f 59 d8          	mulps  xmm11,xmm8
   140684ce6:	45 0f 58 da          	addps  xmm11,xmm10
   140684cea:	41 0f 28 c2          	movaps xmm0,xmm10
   140684cee:	0f c2 c7 06          	cmpnleps xmm0,xmm7
   140684cf2:	0f 28 cf             	movaps xmm1,xmm7
   140684cf5:	41 0f c2 cb 06       	cmpnleps xmm1,xmm11
   140684cfa:	0f 56 c8             	orps   xmm1,xmm0
   140684cfd:	66 0f 70 c1 50       	pshufd xmm0,xmm1,0x50
   140684d02:	66 0f 50 c8          	movmskpd ecx,xmm0
   140684d06:	85 c9                	test   ecx,ecx
   140684d08:	41 0f 95 c1          	setne  r9b
   140684d0c:	74 32                	je     0x140684d40
   140684d0e:	48 8b 8d e8 03 00 00 	mov    rcx,QWORD PTR [rbp+0x3e8]
   140684d15:	83 79 10 02          	cmp    DWORD PTR [rcx+0x10],0x2
   140684d19:	75 35                	jne    0x140684d50
   140684d1b:	48 8b 8d e8 03 00 00 	mov    rcx,QWORD PTR [rbp+0x3e8]
   140684d22:	48 8b 49 18          	mov    rcx,QWORD PTR [rcx+0x18]
   140684d26:	48 3b 8d 88 03 00 00 	cmp    rcx,QWORD PTR [rbp+0x388]
   140684d2d:	75 21                	jne    0x140684d50
   140684d2f:	48 8b 8d e8 03 00 00 	mov    rcx,QWORD PTR [rbp+0x3e8]
   140684d36:	80 79 30 00          	cmp    BYTE PTR [rcx+0x30],0x0
   140684d3a:	74 14                	je     0x140684d50
   140684d3c:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]
   140684d40:	41 0f b6 4d fe       	movzx  ecx,BYTE PTR [r13-0x2]
   140684d45:	80 c9 01             	or     cl,0x1
   140684d48:	eb 0e                	jmp    0x140684d58
   140684d4a:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
   140684d50:	41 0f b6 4d fe       	movzx  ecx,BYTE PTR [r13-0x2]
   140684d55:	80 e1 fe             	and    cl,0xfe
   140684d58:	48 ff ca             	dec    rdx
   140684d5b:	41 88 4d fe          	mov    BYTE PTR [r13-0x2],cl
   140684d5f:	44 89 c1             	mov    ecx,r8d
   140684d62:	84 c0                	test   al,al
   140684d64:	0f 84 b4 fe ff ff    	je     0x140684c1e
   140684d6a:	44 88 8d b8 03 00 00 	mov    BYTE PTR [rbp+0x3b8],r9b
   140684d71:	44 89 85 e0 03 00 00 	mov    DWORD PTR [rbp+0x3e0],r8d
   140684d78:	48 89 95 a8 03 00 00 	mov    QWORD PTR [rbp+0x3a8],rdx
   140684d7f:	4d 8d 65 c8          	lea    r12,[r13-0x38]
   140684d83:	48 83 bd d0 03 00 00 	cmp    QWORD PTR [rbp+0x3d0],0x0
   140684d8a:	00 
   140684d8b:	48 8d 85 40 01 00 00 	lea    rax,[rbp+0x140]
   140684d92:	0f 84 68 08 00 00    	je     0x140685600
   140684d98:	48 8d 8d 88 03 00 00 	lea    rcx,[rbp+0x388]
   140684d9f:	48 89 8d 40 01 00 00 	mov    QWORD PTR [rbp+0x140],rcx
   140684da6:	48 8d 0d 43 80 70 00 	lea    rcx,[rip+0x708043]        # 0x140d8cdf0
   140684dad:	48 89 8d 48 01 00 00 	mov    QWORD PTR [rbp+0x148],rcx
   140684db4:	48 8d 0d 9d 2e 39 02 	lea    rcx,[rip+0x2392e9d]        # 0x142a17c58
   140684dbb:	48 89 8d 80 02 00 00 	mov    QWORD PTR [rbp+0x280],rcx
   140684dc2:	48 c7 85 88 02 00 00 	mov    QWORD PTR [rbp+0x288],0x1
   140684dc9:	01 00 00 00 
   140684dcd:	48 89 85 90 02 00 00 	mov    QWORD PTR [rbp+0x290],rax
   140684dd4:	48 c7 85 98 02 00 00 	mov    QWORD PTR [rbp+0x298],0x1
   140684ddb:	01 00 00 00 
   140684ddf:	48 c7 85 a0 02 00 00 	mov    QWORD PTR [rbp+0x2a0],0x0
   140684de6:	00 00 00 00 
   140684dea:	48 8d 8d 28 01 00 00 	lea    rcx,[rbp+0x128]
   140684df1:	48 8d 95 80 02 00 00 	lea    rdx,[rbp+0x280]
   140684df8:	e8 93 51 6f 00       	call   0x140d79f90
   140684dfd:	0f 10 85 28 01 00 00 	movups xmm0,XMMWORD PTR [rbp+0x128]
   140684e04:	0f 29 85 60 03 00 00 	movaps XMMWORD PTR [rbp+0x360],xmm0
   140684e0b:	48 8b 85 38 01 00 00 	mov    rax,QWORD PTR [rbp+0x138]
   140684e12:	48 89 85 70 03 00 00 	mov    QWORD PTR [rbp+0x370],rax
   140684e19:	f3 41 0f 10 45 f4    	movss  xmm0,DWORD PTR [r13-0xc]
   140684e1f:	41 0f 57 c1          	xorps  xmm0,xmm9
   140684e23:	45 0f 16 d3          	movlhps xmm10,xmm11
   140684e27:	44 0f 11 95 48 01 00 	movups XMMWORD PTR [rbp+0x148],xmm10
   140684e2e:	00 
   140684e2f:	f3 0f 11 85 44 01 00 	movss  DWORD PTR [rbp+0x144],xmm0
   140684e36:	00 
   140684e37:	c6 85 40 01 00 00 03 	mov    BYTE PTR [rbp+0x140],0x3
   140684e3e:	c6 85 20 02 00 00 00 	mov    BYTE PTR [rbp+0x220],0x0
   140684e45:	c6 85 f7 03 00 00 01 	mov    BYTE PTR [rbp+0x3f7],0x1
   140684e4c:	48 8d 9d 80 02 00 00 	lea    rbx,[rbp+0x280]
   140684e53:	48 89 d9             	mov    rcx,rbx
   140684e56:	48 8d bd 40 01 00 00 	lea    rdi,[rbp+0x140]
   140684e5d:	48 89 fa             	mov    rdx,rdi
   140684e60:	4c 8d 85 20 02 00 00 	lea    r8,[rbp+0x220]
   140684e67:	e8 a4 3f 32 00       	call   0x1409a8e10
   140684e6c:	c6 85 f7 03 00 00 01 	mov    BYTE PTR [rbp+0x3f7],0x1
   140684e73:	48 89 f9             	mov    rcx,rdi
   140684e76:	48 89 da             	mov    rdx,rbx
   140684e79:	e8 92 40 32 00       	call   0x1409a8f10
   140684e7e:	c6 85 f7 03 00 00 01 	mov    BYTE PTR [rbp+0x3f7],0x1
   140684e85:	48 89 d9             	mov    rcx,rbx
   140684e88:	48 89 fa             	mov    rdx,rdi
   140684e8b:	45 31 c0             	xor    r8d,r8d
   140684e8e:	e8 4d 40 32 00       	call   0x1409a8ee0
   140684e93:	c6 85 f7 03 00 00 01 	mov    BYTE PTR [rbp+0x3f7],0x1
   140684e9a:	48 89 f9             	mov    rcx,rdi
   140684e9d:	48 89 da             	mov    rdx,rbx
   140684ea0:	41 b0 01             	mov    r8b,0x1
   140684ea3:	e8 98 40 32 00       	call   0x1409a8f40
   140684ea8:	c6 85 f5 03 00 00 01 	mov    BYTE PTR [rbp+0x3f5],0x1
   140684eaf:	48 8d bd 20 02 00 00 	lea    rdi,[rbp+0x220]
   140684eb6:	48 89 f9             	mov    rcx,rdi
   140684eb9:	e8 f2 40 32 00       	call   0x1409a8fb0
   140684ebe:	c6 85 f5 03 00 00 00 	mov    BYTE PTR [rbp+0x3f5],0x0
   140684ec5:	48 89 d9             	mov    rcx,rbx
   140684ec8:	48 8d 95 40 01 00 00 	lea    rdx,[rbp+0x140]
   140684ecf:	49 89 f8             	mov    r8,rdi
   140684ed2:	e8 a9 3f 32 00       	call   0x1409a8e80
   140684ed7:	c6 85 f7 03 00 00 00 	mov    BYTE PTR [rbp+0x3f7],0x0
   140684ede:	48 8b 8d d0 03 00 00 	mov    rcx,QWORD PTR [rbp+0x3d0]
   140684ee5:	48 8d 95 60 03 00 00 	lea    rdx,[rbp+0x360]
   140684eec:	49 89 d8             	mov    r8,rbx
   140684eef:	e8 fc e2 29 00       	call   0x1409231f0
   140684ef4:	48 89 85 00 02 00 00 	mov    QWORD PTR [rbp+0x200],rax
   140684efb:	48 89 95 08 02 00 00 	mov    QWORD PTR [rbp+0x208],rdx
   140684f02:	48 8d 8d 00 02 00 00 	lea    rcx,[rbp+0x200]
   140684f09:	e8 02 90 2a 00       	call   0x14092df10
   140684f0e:	88 85 f6 03 00 00    	mov    BYTE PTR [rbp+0x3f6],al
   140684f14:	48 8d 8d f6 03 00 00 	lea    rcx,[rbp+0x3f6]
   140684f1b:	e8 90 97 2a 00       	call   0x14092e6b0
   140684f20:	41 0f b6 4d fe       	movzx  ecx,BYTE PTR [r13-0x2]
   140684f25:	80 e1 fb             	and    cl,0xfb
   140684f28:	c0 e0 02             	shl    al,0x2
   140684f2b:	08 c8                	or     al,cl
   140684f2d:	41 88 45 fe          	mov    BYTE PTR [r13-0x2],al
   140684f31:	48 8d 8d f6 03 00 00 	lea    rcx,[rbp+0x3f6]
   140684f38:	e8 83 97 2a 00       	call   0x14092e6c0
   140684f3d:	41 0f b6 4d fe       	movzx  ecx,BYTE PTR [r13-0x2]
   140684f42:	80 e1 fd             	and    cl,0xfd
   140684f45:	00 c0                	add    al,al
   140684f47:	08 c8                	or     al,cl
   140684f49:	41 88 45 fe          	mov    BYTE PTR [r13-0x2],al
   140684f4d:	48 8d 8d f6 03 00 00 	lea    rcx,[rbp+0x3f6]
   140684f54:	e8 77 97 2a 00       	call   0x14092e6d0
   140684f59:	41 0f b6 4d fe       	movzx  ecx,BYTE PTR [r13-0x2]
   140684f5e:	80 e1 f7             	and    cl,0xf7
   140684f61:	c0 e0 03             	shl    al,0x3
   140684f64:	08 c8                	or     al,cl
   140684f66:	41 88 45 fe          	mov    BYTE PTR [r13-0x2],al
   140684f6a:	48 8d 8d 00 02 00 00 	lea    rcx,[rbp+0x200]
   140684f71:	e8 7a 96 2a 00       	call   0x14092e5f0
   140684f76:	84 c0                	test   al,al
   140684f78:	74 16                	je     0x140684f90
   140684f7a:	41 80 4d fe 10       	or     BYTE PTR [r13-0x2],0x10
   140684f7f:	eb 4f                	jmp    0x140684fd0
   140684f81:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   140684f88:	0f 1f 84 00 00 00 00 
   140684f8f:	00 
   140684f90:	48 8b 85 e8 03 00 00 	mov    rax,QWORD PTR [rbp+0x3e8]
   140684f97:	83 78 10 02          	cmp    DWORD PTR [rax+0x10],0x2
   140684f9b:	75 33                	jne    0x140684fd0
   140684f9d:	48 8b 85 e8 03 00 00 	mov    rax,QWORD PTR [rbp+0x3e8]
   140684fa4:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
   140684fa8:	48 3b 85 88 03 00 00 	cmp    rax,QWORD PTR [rbp+0x388]
   140684faf:	75 1f                	jne    0x140684fd0
   140684fb1:	48 8b 85 c8 03 00 00 	mov    rax,QWORD PTR [rbp+0x3c8]
   140684fb8:	8b 88 c8 00 00 00    	mov    ecx,DWORD PTR [rax+0xc8]
   140684fbe:	f3 0f 10 88 cc 00 00 	movss  xmm1,DWORD PTR [rax+0xcc]
   140684fc5:	00 
   140684fc6:	e8 c5 aa 2b 00       	call   0x14093fa90
   140684fcb:	84 c0                	test   al,al
   140684fcd:	75 ab                	jne    0x140684f7a
   140684fcf:	90                   	nop
   140684fd0:	49 8b 04 24          	mov    rax,QWORD PTR [r12]
   140684fd4:	48 85 c0             	test   rax,rax
   140684fd7:	4c 0f 44 e0          	cmove  r12,rax
   140684fdb:	0f 84 bf 03 00 00    	je     0x1406853a0
   140684fe1:	41 f6 45 fe 40       	test   BYTE PTR [r13-0x2],0x40
   140684fe6:	0f 84 f9 02 00 00    	je     0x1406852e5
   140684fec:	48 8b 95 c8 03 00 00 	mov    rdx,QWORD PTR [rbp+0x3c8]
   140684ff3:	48 8b 42 20          	mov    rax,QWORD PTR [rdx+0x20]
   140684ff7:	48 8b 4a 28          	mov    rcx,QWORD PTR [rdx+0x28]
   140684ffb:	48 c7 42 28 00 00 00 	mov    QWORD PTR [rdx+0x28],0x0
   140685002:	00 
   140685003:	48 6b d1 70          	imul   rdx,rcx,0x70
   140685007:	48 01 c2             	add    rdx,rax
   14068500a:	48 89 95 28 02 00 00 	mov    QWORD PTR [rbp+0x228],rdx
   140685011:	4c 8b 85 10 02 00 00 	mov    r8,QWORD PTR [rbp+0x210]
   140685018:	4c 89 85 30 02 00 00 	mov    QWORD PTR [rbp+0x230],r8
   14068501f:	48 89 8d 38 02 00 00 	mov    QWORD PTR [rbp+0x238],rcx
   140685026:	48 c7 85 40 02 00 00 	mov    QWORD PTR [rbp+0x240],0x0
   14068502d:	00 00 00 00 
   140685031:	48 8d bd dc 02 00 00 	lea    rdi,[rbp+0x2dc]
   140685038:	eb 14                	jmp    0x14068504e
   14068503a:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
   140685040:	48 8b 85 20 02 00 00 	mov    rax,QWORD PTR [rbp+0x220]
   140685047:	48 8b 95 28 02 00 00 	mov    rdx,QWORD PTR [rbp+0x228]
   14068504e:	48 39 d0             	cmp    rax,rdx
   140685051:	0f 84 90 01 00 00    	je     0x1406851e7
   140685057:	48 8d 48 70          	lea    rcx,[rax+0x70]
   14068505b:	48 89 8d 20 02 00 00 	mov    QWORD PTR [rbp+0x220],rcx
   140685062:	0f 10 00             	movups xmm0,XMMWORD PTR [rax]
   140685065:	0f 10 48 10          	movups xmm1,XMMWORD PTR [rax+0x10]
   140685069:	0f 10 50 20          	movups xmm2,XMMWORD PTR [rax+0x20]
   14068506d:	0f 10 58 30          	movups xmm3,XMMWORD PTR [rax+0x30]
   140685071:	0f 29 85 40 01 00 00 	movaps XMMWORD PTR [rbp+0x140],xmm0
   140685078:	0f 29 8d 50 01 00 00 	movaps XMMWORD PTR [rbp+0x150],xmm1
   14068507f:	0f 29 95 60 01 00 00 	movaps XMMWORD PTR [rbp+0x160],xmm2
   140685086:	0f 29 9d 70 01 00 00 	movaps XMMWORD PTR [rbp+0x170],xmm3
   14068508d:	0f 10 40 40          	movups xmm0,XMMWORD PTR [rax+0x40]
   140685091:	0f 29 85 80 01 00 00 	movaps XMMWORD PTR [rbp+0x180],xmm0
   140685098:	4c 8b 40 50          	mov    r8,QWORD PTR [rax+0x50]
   14068509c:	4c 89 85 90 01 00 00 	mov    QWORD PTR [rbp+0x190],r8
   1406850a3:	44 8b 40 58          	mov    r8d,DWORD PTR [rax+0x58]
   1406850a7:	44 8b 48 6c          	mov    r9d,DWORD PTR [rax+0x6c]
   1406850ab:	44 89 8d 70 03 00 00 	mov    DWORD PTR [rbp+0x370],r9d
   1406850b2:	0f 10 40 5c          	movups xmm0,XMMWORD PTR [rax+0x5c]
   1406850b6:	0f 29 85 60 03 00 00 	movaps XMMWORD PTR [rbp+0x360],xmm0
   1406850bd:	41 81 f8 c6 00 00 00 	cmp    r8d,0xc6
   1406850c4:	0f 84 29 01 00 00    	je     0x1406851f3
   1406850ca:	48 8b 85 90 01 00 00 	mov    rax,QWORD PTR [rbp+0x190]
   1406850d1:	48 89 85 d0 02 00 00 	mov    QWORD PTR [rbp+0x2d0],rax
   1406850d8:	0f 28 85 80 01 00 00 	movaps xmm0,XMMWORD PTR [rbp+0x180]
   1406850df:	0f 29 85 c0 02 00 00 	movaps XMMWORD PTR [rbp+0x2c0],xmm0
   1406850e6:	0f 28 85 40 01 00 00 	movaps xmm0,XMMWORD PTR [rbp+0x140]
   1406850ed:	0f 28 8d 50 01 00 00 	movaps xmm1,XMMWORD PTR [rbp+0x150]
   1406850f4:	0f 28 95 60 01 00 00 	movaps xmm2,XMMWORD PTR [rbp+0x160]
   1406850fb:	0f 28 9d 70 01 00 00 	movaps xmm3,XMMWORD PTR [rbp+0x170]
   140685102:	0f 29 9d b0 02 00 00 	movaps XMMWORD PTR [rbp+0x2b0],xmm3
   140685109:	0f 29 95 a0 02 00 00 	movaps XMMWORD PTR [rbp+0x2a0],xmm2
   140685110:	0f 29 8d 90 02 00 00 	movaps XMMWORD PTR [rbp+0x290],xmm1
   140685117:	0f 29 85 80 02 00 00 	movaps XMMWORD PTR [rbp+0x280],xmm0
   14068511e:	44 89 85 d8 02 00 00 	mov    DWORD PTR [rbp+0x2d8],r8d
   140685125:	8b 85 70 03 00 00    	mov    eax,DWORD PTR [rbp+0x370]
   14068512b:	89 47 10             	mov    DWORD PTR [rdi+0x10],eax
   14068512e:	0f 28 85 60 03 00 00 	movaps xmm0,XMMWORD PTR [rbp+0x360]
   140685135:	0f 11 07             	movups XMMWORD PTR [rdi],xmm0
   140685138:	49 8b 0c 24          	mov    rcx,QWORD PTR [r12]
   14068513c:	49 8b 45 d0          	mov    rax,QWORD PTR [r13-0x30]
   140685140:	48 8d 95 80 02 00 00 	lea    rdx,[rbp+0x280]
   140685147:	ff 50 48             	call   QWORD PTR [rax+0x48]
   14068514a:	8b 85 d8 02 00 00    	mov    eax,DWORD PTR [rbp+0x2d8]
   140685150:	b9 3d ff ff ff       	mov    ecx,0xffffff3d
   140685155:	01 c8                	add    eax,ecx
   140685157:	83 f8 03             	cmp    eax,0x3
   14068515a:	b9 01 00 00 00       	mov    ecx,0x1
   14068515f:	0f 43 c1             	cmovae eax,ecx
   140685162:	85 c0                	test   eax,eax
   140685164:	0f 84 d6 fe ff ff    	je     0x140685040
   14068516a:	83 f8 01             	cmp    eax,0x1
   14068516d:	75 51                	jne    0x1406851c0
   14068516f:	48 8d 8d 98 02 00 00 	lea    rcx,[rbp+0x298]
   140685176:	e8 b5 c5 f6 ff       	call   0x1405f1730
   14068517b:	0f b6 85 80 02 00 00 	movzx  eax,BYTE PTR [rbp+0x280]
   140685182:	3c 1b                	cmp    al,0x1b
   140685184:	0f 84 b6 fe ff ff    	je     0x140685040
   14068518a:	3c 18                	cmp    al,0x18
   14068518c:	0f 85 ae fe ff ff    	jne    0x140685040
   140685192:	48 8b 85 88 02 00 00 	mov    rax,QWORD PTR [rbp+0x288]
   140685199:	f0 48 ff 08          	lock dec QWORD PTR [rax]
   14068519d:	0f 85 9d fe ff ff    	jne    0x140685040
   1406851a3:	48 8d 8d 88 02 00 00 	lea    rcx,[rbp+0x288]
   1406851aa:	e8 81 99 fc ff       	call   0x14064eb30
   1406851af:	e9 8c fe ff ff       	jmp    0x140685040
   1406851b4:	66                   	data16
   1406851b5:	66                   	data16
   1406851b6:	66                   	data16
   1406851b7:	2e                   	cs
   1406851b8:	0f                   	.byte 0xf
   1406851b9:	1f                   	(bad)
   1406851ba:	84 00                	test   BYTE PTR [rax],al
   1406851bc:	00 00                	add    BYTE PTR [rax],al
	...

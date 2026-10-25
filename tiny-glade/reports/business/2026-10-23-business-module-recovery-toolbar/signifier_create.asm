
tiny-glade.exe:     file format pei-x86-64


Disassembly of section .text:

0000000140b2cfa0 <.text+0xb2bfa0>:
   140b2cfa0:	55                   	push   rbp
   140b2cfa1:	41 57                	push   r15
   140b2cfa3:	41 56                	push   r14
   140b2cfa5:	41 55                	push   r13
   140b2cfa7:	41 54                	push   r12
   140b2cfa9:	56                   	push   rsi
   140b2cfaa:	57                   	push   rdi
   140b2cfab:	53                   	push   rbx
   140b2cfac:	48 81 ec f8 02 00 00 	sub    rsp,0x2f8
   140b2cfb3:	48 8d ac 24 80 00 00 	lea    rbp,[rsp+0x80]
   140b2cfba:	00 
   140b2cfbb:	66 0f 7f b5 60 02 00 	movdqa XMMWORD PTR [rbp+0x260],xmm6
   140b2cfc2:	00 
   140b2cfc3:	48 c7 85 58 02 00 00 	mov    QWORD PTR [rbp+0x258],0xfffffffffffffffe
   140b2cfca:	fe ff ff ff 
   140b2cfce:	4c 89 8d 48 02 00 00 	mov    QWORD PTR [rbp+0x248],r9
   140b2cfd5:	4c 89 85 28 02 00 00 	mov    QWORD PTR [rbp+0x228],r8
   140b2cfdc:	48 89 95 40 02 00 00 	mov    QWORD PTR [rbp+0x240],rdx
   140b2cfe3:	48 89 cf             	mov    rdi,rcx
   140b2cfe6:	e8 15 9a 0e 00       	call   0x140c16a00
   140b2cfeb:	84 c0                	test   al,al
   140b2cfed:	48 89 bd 38 02 00 00 	mov    QWORD PTR [rbp+0x238],rdi
   140b2cff4:	74 60                	je     0x140b2d056
   140b2cff6:	4c 8d 25 8b e2 fb 01 	lea    r12,[rip+0x1fbe28b]        # 0x142aeb288
   140b2cffd:	41 bf 49 00 00 00    	mov    r15d,0x49
   140b2d003:	ba 49 00 00 00       	mov    edx,0x49
   140b2d008:	4c 89 e1             	mov    rcx,r12
   140b2d00b:	e8 60 de ff ff       	call   0x140b2ae70
   140b2d010:	48 83 f8 01          	cmp    rax,0x1
   140b2d014:	0f 85 b3 00 00 00    	jne    0x140b2d0cd
   140b2d01a:	49 89 d1             	mov    r9,rdx
   140b2d01d:	48 85 d2             	test   rdx,rdx
   140b2d020:	74 3e                	je     0x140b2d060
   140b2d022:	49 83 f9 49          	cmp    r9,0x49
   140b2d026:	73 36                	jae    0x140b2d05e
   140b2d028:	48 8d 05 59 e2 fb 01 	lea    rax,[rip+0x1fbe259]        # 0x142aeb288
   140b2d02f:	41 80 3c 01 bf       	cmp    BYTE PTR [r9+rax*1],0xbf
   140b2d034:	7f 2a                	jg     0x140b2d060
   140b2d036:	48 8d 05 ab e3 fb 01 	lea    rax,[rip+0x1fbe3ab]        # 0x142aeb3e8
   140b2d03d:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140b2d042:	48 8d 0d 3f e2 fb 01 	lea    rcx,[rip+0x1fbe23f]        # 0x142aeb288
   140b2d049:	ba 49 00 00 00       	mov    edx,0x49
   140b2d04e:	45 31 c0             	xor    r8d,r8d
   140b2d051:	e8 ea c8 da 01       	call   0x1428d9940
   140b2d056:	45 31 f6             	xor    r14d,r14d
   140b2d059:	e9 d0 01 00 00       	jmp    0x140b2d22e
   140b2d05e:	75 d6                	jne    0x140b2d036
   140b2d060:	4c 8d 25 21 e2 fb 01 	lea    r12,[rip+0x1fbe221]        # 0x142aeb288
   140b2d067:	4c 89 e1             	mov    rcx,r12
   140b2d06a:	4c 89 ca             	mov    rdx,r9
   140b2d06d:	e8 fe dd ff ff       	call   0x140b2ae70
   140b2d072:	48 83 f8 01          	cmp    rax,0x1
   140b2d076:	75 55                	jne    0x140b2d0cd
   140b2d078:	49 89 d0             	mov    r8,rdx
   140b2d07b:	49 83 c0 02          	add    r8,0x2
   140b2d07f:	74 39                	je     0x140b2d0ba
   140b2d081:	49 83 f8 49          	cmp    r8,0x49
   140b2d085:	73 31                	jae    0x140b2d0b8
   140b2d087:	48 8d 05 fa e1 fb 01 	lea    rax,[rip+0x1fbe1fa]        # 0x142aeb288
   140b2d08e:	41 80 3c 00 bf       	cmp    BYTE PTR [r8+rax*1],0xbf
   140b2d093:	7f 25                	jg     0x140b2d0ba
   140b2d095:	48 8d 05 64 e3 fb 01 	lea    rax,[rip+0x1fbe364]        # 0x142aeb400
   140b2d09c:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140b2d0a1:	48 8d 0d e0 e1 fb 01 	lea    rcx,[rip+0x1fbe1e0]        # 0x142aeb288
   140b2d0a8:	ba 49 00 00 00       	mov    edx,0x49
   140b2d0ad:	41 b9 49 00 00 00    	mov    r9d,0x49
   140b2d0b3:	e8 88 c8 da 01       	call   0x1428d9940
   140b2d0b8:	75 db                	jne    0x140b2d095
   140b2d0ba:	41 bf 47 00 00 00    	mov    r15d,0x47
   140b2d0c0:	49 29 d7             	sub    r15,rdx
   140b2d0c3:	4c 8d 25 be e1 fb 01 	lea    r12,[rip+0x1fbe1be]        # 0x142aeb288
   140b2d0ca:	4d 01 c4             	add    r12,r8
   140b2d0cd:	48 8d 05 85 e7 fb 01 	lea    rax,[rip+0x1fbe785]        # 0x142aeb859
   140b2d0d4:	4c 8d 2d 3d e7 fb 01 	lea    r13,[rip+0x1fbe73d]        # 0x142aeb818
   140b2d0db:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
   140b2d0e0:	0f be 48 ff          	movsx  ecx,BYTE PTR [rax-0x1]
   140b2d0e4:	85 c9                	test   ecx,ecx
   140b2d0e6:	78 18                	js     0x140b2d100
   140b2d0e8:	48 ff c8             	dec    rax
   140b2d0eb:	83 f9 5c             	cmp    ecx,0x5c
   140b2d0ee:	75 69                	jne    0x140b2d159
   140b2d0f0:	eb 7c                	jmp    0x140b2d16e
   140b2d0f2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   140b2d0f9:	1f 84 00 00 00 00 00 
   140b2d100:	0f b6 50 fe          	movzx  edx,BYTE PTR [rax-0x2]
   140b2d104:	80 fa c0             	cmp    dl,0xc0
   140b2d107:	7d 25                	jge    0x140b2d12e
   140b2d109:	44 0f b6 40 fd       	movzx  r8d,BYTE PTR [rax-0x3]
   140b2d10e:	41 80 f8 c0          	cmp    r8b,0xc0
   140b2d112:	7d 23                	jge    0x140b2d137
   140b2d114:	44 0f b6 48 fc       	movzx  r9d,BYTE PTR [rax-0x4]
   140b2d119:	48 83 c0 fc          	add    rax,0xfffffffffffffffc
   140b2d11d:	41 83 e1 07          	and    r9d,0x7
   140b2d121:	41 c1 e1 06          	shl    r9d,0x6
   140b2d125:	41 83 e0 3f          	and    r8d,0x3f
   140b2d129:	45 09 c8             	or     r8d,r9d
   140b2d12c:	eb 11                	jmp    0x140b2d13f
   140b2d12e:	48 83 c0 fe          	add    rax,0xfffffffffffffffe
   140b2d132:	83 e2 1f             	and    edx,0x1f
   140b2d135:	eb 12                	jmp    0x140b2d149
   140b2d137:	48 83 c0 fd          	add    rax,0xfffffffffffffffd
   140b2d13b:	41 83 e0 0f          	and    r8d,0xf
   140b2d13f:	41 c1 e0 06          	shl    r8d,0x6
   140b2d143:	83 e2 3f             	and    edx,0x3f
   140b2d146:	44 09 c2             	or     edx,r8d
   140b2d149:	c1 e2 06             	shl    edx,0x6
   140b2d14c:	80 e1 3f             	and    cl,0x3f
   140b2d14f:	0f b6 c9             	movzx  ecx,cl
   140b2d152:	09 d1                	or     ecx,edx
   140b2d154:	83 f9 5c             	cmp    ecx,0x5c
   140b2d157:	74 15                	je     0x140b2d16e
   140b2d159:	83 f9 2f             	cmp    ecx,0x2f
   140b2d15c:	74 10                	je     0x140b2d16e
   140b2d15e:	4c 39 e8             	cmp    rax,r13
   140b2d161:	0f 85 79 ff ff ff    	jne    0x140b2d0e0
   140b2d167:	be 41 00 00 00       	mov    esi,0x41
   140b2d16c:	eb 4f                	jmp    0x140b2d1bd
   140b2d16e:	4c 8d 2d a3 e6 fb 01 	lea    r13,[rip+0x1fbe6a3]        # 0x142aeb818
   140b2d175:	4c 29 e8             	sub    rax,r13
   140b2d178:	49 89 c0             	mov    r8,rax
   140b2d17b:	49 ff c0             	inc    r8
   140b2d17e:	74 32                	je     0x140b2d1b2
   140b2d180:	49 83 f8 41          	cmp    r8,0x41
   140b2d184:	73 2a                	jae    0x140b2d1b0
   140b2d186:	43 80 3c 28 bf       	cmp    BYTE PTR [r8+r13*1],0xbf
   140b2d18b:	7f 25                	jg     0x140b2d1b2
   140b2d18d:	48 8d 05 34 e2 fb 01 	lea    rax,[rip+0x1fbe234]        # 0x142aeb3c8
   140b2d194:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140b2d199:	48 8d 0d 78 e6 fb 01 	lea    rcx,[rip+0x1fbe678]        # 0x142aeb818
   140b2d1a0:	ba 41 00 00 00       	mov    edx,0x41
   140b2d1a5:	41 b9 41 00 00 00    	mov    r9d,0x41
   140b2d1ab:	e8 90 c7 da 01       	call   0x1428d9940
   140b2d1b0:	75 db                	jne    0x140b2d18d
   140b2d1b2:	be 40 00 00 00       	mov    esi,0x40
   140b2d1b7:	48 29 c6             	sub    rsi,rax
   140b2d1ba:	4d 01 c5             	add    r13,r8
   140b2d1bd:	e8 6e af 0e 00       	call   0x140c18130
   140b2d1c2:	48 89 c1             	mov    rcx,rax
   140b2d1c5:	48 8b 00             	mov    rax,QWORD PTR [rax]
   140b2d1c8:	48 83 f8 01          	cmp    rax,0x1
   140b2d1cc:	0f 85 38 07 00 00    	jne    0x140b2d90a
   140b2d1d2:	48 83 c1 08          	add    rcx,0x8
   140b2d1d6:	48 83 39 00          	cmp    QWORD PTR [rcx],0x0
   140b2d1da:	0f 85 4e 07 00 00    	jne    0x140b2d92e
   140b2d1e0:	48 c7 01 ff ff ff ff 	mov    QWORD PTR [rcx],0xffffffffffffffff
   140b2d1e7:	48 89 8d 50 02 00 00 	mov    QWORD PTR [rbp+0x250],rcx
   140b2d1ee:	48 83 c1 08          	add    rcx,0x8
   140b2d1f2:	48 89 74 24 20       	mov    QWORD PTR [rsp+0x20],rsi
   140b2d1f7:	48 c7 44 24 30 00 00 	mov    QWORD PTR [rsp+0x30],0x0
   140b2d1fe:	00 00 
   140b2d200:	48 c7 44 24 28 01 00 	mov    QWORD PTR [rsp+0x28],0x1
   140b2d207:	00 00 
   140b2d209:	4c 89 e2             	mov    rdx,r12
   140b2d20c:	4d 89 f8             	mov    r8,r15
   140b2d20f:	4d 89 e9             	mov    r9,r13
   140b2d212:	e8 29 9b 0e 00       	call   0x140c16d40
   140b2d217:	48 8b 8d 50 02 00 00 	mov    rcx,QWORD PTR [rbp+0x250]
   140b2d21e:	48 ff 01             	inc    QWORD PTR [rcx]
   140b2d221:	48 89 85 e8 01 00 00 	mov    QWORD PTR [rbp+0x1e8],rax
   140b2d228:	41 be 01 00 00 00    	mov    r14d,0x1
   140b2d22e:	48 8b 85 e0 02 00 00 	mov    rax,QWORD PTR [rbp+0x2e0]
   140b2d235:	4c 89 b5 e0 01 00 00 	mov    QWORD PTR [rbp+0x1e0],r14
   140b2d23c:	48 8b 30             	mov    rsi,QWORD PTR [rax]
   140b2d23f:	4c 8b 60 08          	mov    r12,QWORD PTR [rax+0x8]
   140b2d243:	48 8b be f8 00 00 00 	mov    rdi,QWORD PTR [rsi+0xf8]
   140b2d24a:	4c 8b ae 00 01 00 00 	mov    r13,QWORD PTR [rsi+0x100]
   140b2d251:	0f b6 9e 08 01 00 00 	movzx  ebx,BYTE PTR [rsi+0x108]
   140b2d258:	e8 e3 04 22 00       	call   0x140d4d740
   140b2d25d:	49 89 c7             	mov    r15,rax
   140b2d260:	f6 00 01             	test   BYTE PTR [rax],0x1
   140b2d263:	4c 89 b5 50 02 00 00 	mov    QWORD PTR [rbp+0x250],r14
   140b2d26a:	74 11                	je     0x140b2d27d
   140b2d26c:	4c 89 f9             	mov    rcx,r15
   140b2d26f:	48 83 c1 08          	add    rcx,0x8
   140b2d273:	49 8b 47 08          	mov    rax,QWORD PTR [r15+0x8]
   140b2d277:	49 8b 57 10          	mov    rdx,QWORD PTR [r15+0x10]
   140b2d27b:	eb 22                	jmp    0x140b2d29f
   140b2d27d:	4c 89 b5 30 02 00 00 	mov    QWORD PTR [rbp+0x230],r14
   140b2d284:	e8 77 7b 23 00       	call   0x140d64e00
   140b2d289:	4c 89 f9             	mov    rcx,r15
   140b2d28c:	48 83 c1 08          	add    rcx,0x8
   140b2d290:	49 c7 07 01 00 00 00 	mov    QWORD PTR [r15],0x1
   140b2d297:	49 89 47 08          	mov    QWORD PTR [r15+0x8],rax
   140b2d29b:	49 89 57 10          	mov    QWORD PTR [r15+0x10],rdx
   140b2d29f:	4e 8d 04 af          	lea    r8,[rdi+r13*4]
   140b2d2a3:	4d 8d 8c 24 f8 00 00 	lea    r9,[r12+0xf8]
   140b2d2aa:	00 
   140b2d2ab:	4d 8d 94 24 a0 01 00 	lea    r10,[r12+0x1a0]
   140b2d2b2:	00 
   140b2d2b3:	4c 8d 58 01          	lea    r11,[rax+0x1]
   140b2d2b7:	4c 89 19             	mov    QWORD PTR [rcx],r11
   140b2d2ba:	0f 10 05 3f de fb 01 	movups xmm0,XMMWORD PTR [rip+0x1fbde3f]        # 0x142aeb100
   140b2d2c1:	0f 29 85 00 02 00 00 	movaps XMMWORD PTR [rbp+0x200],xmm0
   140b2d2c8:	f3 0f 6f 05 20 de fb 	movdqu xmm0,XMMWORD PTR [rip+0x1fbde20]        # 0x142aeb0f0
   140b2d2cf:	01 
   140b2d2d0:	66 0f 7f 85 f0 01 00 	movdqa XMMWORD PTR [rbp+0x1f0],xmm0
   140b2d2d7:	00 
   140b2d2d8:	48 89 85 10 02 00 00 	mov    QWORD PTR [rbp+0x210],rax
   140b2d2df:	48 89 95 18 02 00 00 	mov    QWORD PTR [rbp+0x218],rdx
   140b2d2e6:	66 0f ef c0          	pxor   xmm0,xmm0
   140b2d2ea:	f3 0f 7f 85 c8 01 00 	movdqu XMMWORD PTR [rbp+0x1c8],xmm0
   140b2d2f1:	00 
   140b2d2f2:	f3 0f 7f 85 b8 01 00 	movdqu XMMWORD PTR [rbp+0x1b8],xmm0
   140b2d2f9:	00 
   140b2d2fa:	4c 89 a5 70 01 00 00 	mov    QWORD PTR [rbp+0x170],r12
   140b2d301:	4c 89 95 78 01 00 00 	mov    QWORD PTR [rbp+0x178],r10
   140b2d308:	4c 89 8d 80 01 00 00 	mov    QWORD PTR [rbp+0x180],r9
   140b2d30f:	48 89 b5 88 01 00 00 	mov    QWORD PTR [rbp+0x188],rsi
   140b2d316:	48 89 bd 90 01 00 00 	mov    QWORD PTR [rbp+0x190],rdi
   140b2d31d:	4c 89 85 98 01 00 00 	mov    QWORD PTR [rbp+0x198],r8
   140b2d324:	48 c7 85 a0 01 00 00 	mov    QWORD PTR [rbp+0x1a0],0x8
   140b2d32b:	08 00 00 00 
   140b2d32f:	48 c7 85 a8 01 00 00 	mov    QWORD PTR [rbp+0x1a8],0x0
   140b2d336:	00 00 00 00 
   140b2d33a:	48 c7 85 b0 01 00 00 	mov    QWORD PTR [rbp+0x1b0],0x8
   140b2d341:	08 00 00 00 
   140b2d345:	88 9d d8 01 00 00    	mov    BYTE PTR [rbp+0x1d8],bl
   140b2d34b:	48 8d 4d c0          	lea    rcx,[rbp-0x40]
   140b2d34f:	48 8d 95 70 01 00 00 	lea    rdx,[rbp+0x170]
   140b2d356:	e8 b5 86 c6 00       	call   0x141795a10
   140b2d35b:	48 8b 55 c0          	mov    rdx,QWORD PTR [rbp-0x40]
   140b2d35f:	48 39 95 00 02 00 00 	cmp    QWORD PTR [rbp+0x200],rdx
   140b2d366:	0f 82 ce 05 00 00    	jb     0x140b2d93a
   140b2d36c:	0f 10 85 d0 01 00 00 	movups xmm0,XMMWORD PTR [rbp+0x1d0]
   140b2d373:	0f 29 45 20          	movaps XMMWORD PTR [rbp+0x20],xmm0
   140b2d377:	0f 10 85 c0 01 00 00 	movups xmm0,XMMWORD PTR [rbp+0x1c0]
   140b2d37e:	0f 29 45 10          	movaps XMMWORD PTR [rbp+0x10],xmm0
   140b2d382:	0f 10 85 b0 01 00 00 	movups xmm0,XMMWORD PTR [rbp+0x1b0]
   140b2d389:	0f 29 45 00          	movaps XMMWORD PTR [rbp+0x0],xmm0
   140b2d38d:	f3 0f 6f 85 70 01 00 	movdqu xmm0,XMMWORD PTR [rbp+0x170]
   140b2d394:	00 
   140b2d395:	f3 0f 6f 8d 80 01 00 	movdqu xmm1,XMMWORD PTR [rbp+0x180]
   140b2d39c:	00 
   140b2d39d:	f3 0f 6f 95 90 01 00 	movdqu xmm2,XMMWORD PTR [rbp+0x190]
   140b2d3a4:	00 
   140b2d3a5:	0f 10 9d a0 01 00 00 	movups xmm3,XMMWORD PTR [rbp+0x1a0]
   140b2d3ac:	0f 29 5d f0          	movaps XMMWORD PTR [rbp-0x10],xmm3
   140b2d3b0:	66 0f 7f 55 e0       	movdqa XMMWORD PTR [rbp-0x20],xmm2
   140b2d3b5:	66 0f 7f 4d d0       	movdqa XMMWORD PTR [rbp-0x30],xmm1
   140b2d3ba:	66 0f 7f 45 c0       	movdqa XMMWORD PTR [rbp-0x40],xmm0
   140b2d3bf:	48 8d 4d c0          	lea    rcx,[rbp-0x40]
   140b2d3c3:	48 8d 95 f0 01 00 00 	lea    rdx,[rbp+0x1f0]
   140b2d3ca:	e8 61 81 94 00       	call   0x141475530
   140b2d3cf:	66 0f 6f 85 f0 01 00 	movdqa xmm0,XMMWORD PTR [rbp+0x1f0]
   140b2d3d6:	00 
   140b2d3d7:	66 0f 6f 8d 00 02 00 	movdqa xmm1,XMMWORD PTR [rbp+0x200]
   140b2d3de:	00 
   140b2d3df:	66 0f 6f 95 10 02 00 	movdqa xmm2,XMMWORD PTR [rbp+0x210]
   140b2d3e6:	00 
   140b2d3e7:	66 0f 7f 85 70 01 00 	movdqa XMMWORD PTR [rbp+0x170],xmm0
   140b2d3ee:	00 
   140b2d3ef:	66 0f 7f 8d 80 01 00 	movdqa XMMWORD PTR [rbp+0x180],xmm1
   140b2d3f6:	00 
   140b2d3f7:	66 0f 7f 95 90 01 00 	movdqa XMMWORD PTR [rbp+0x190],xmm2
   140b2d3fe:	00 
   140b2d3ff:	48 8b 95 38 02 00 00 	mov    rdx,QWORD PTR [rbp+0x238]
   140b2d406:	8b 42 1c             	mov    eax,DWORD PTR [rdx+0x1c]
   140b2d409:	48 8b 4a 10          	mov    rcx,QWORD PTR [rdx+0x10]
   140b2d40d:	89 01                	mov    DWORD PTR [rcx],eax
   140b2d40f:	48 8b 02             	mov    rax,QWORD PTR [rdx]
   140b2d412:	48 89 85 50 02 00 00 	mov    QWORD PTR [rbp+0x250],rax
   140b2d419:	80 78 18 00          	cmp    BYTE PTR [rax+0x18],0x0
   140b2d41d:	0f 84 33 04 00 00    	je     0x140b2d856
   140b2d423:	48 8b 85 e8 02 00 00 	mov    rax,QWORD PTR [rbp+0x2e8]
   140b2d42a:	8b 48 1c             	mov    ecx,DWORD PTR [rax+0x1c]
   140b2d42d:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
   140b2d431:	89 0a                	mov    DWORD PTR [rdx],ecx
   140b2d433:	4c 8b 38             	mov    r15,QWORD PTR [rax]
   140b2d436:	48 8d 4d c0          	lea    rcx,[rbp-0x40]
   140b2d43a:	41 b8 70 01 00 00    	mov    r8d,0x170
   140b2d440:	4c 89 fa             	mov    rdx,r15
   140b2d443:	e8 b8 67 df 01       	call   0x142923c00
   140b2d448:	45 31 f6             	xor    r14d,r14d
   140b2d44b:	41 b8 70 01 00 00    	mov    r8d,0x170
   140b2d451:	4c 89 f9             	mov    rcx,r15
   140b2d454:	31 d2                	xor    edx,edx
   140b2d456:	e8 c5 67 df 01       	call   0x142923c20
   140b2d45b:	48 8b b5 20 01 00 00 	mov    rsi,QWORD PTR [rbp+0x120]
   140b2d462:	48 c7 c7 ff ff ff ff 	mov    rdi,0xffffffffffffffff
   140b2d469:	4c 8d bd 30 01 00 00 	lea    r15,[rbp+0x130]
   140b2d470:	4c 8d 25 01 e4 fb 01 	lea    r12,[rip+0x1fbe401]        # 0x142aeb878
   140b2d477:	eb 47                	jmp    0x140b2d4c0
   140b2d479:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
   140b2d480:	49 ff c6             	inc    r14
   140b2d483:	48 8b 43 18          	mov    rax,QWORD PTR [rbx+0x18]
   140b2d487:	4a 8d 0c ad 00 00 00 	lea    rcx,[r13*4+0x0]
   140b2d48e:	00 
   140b2d48f:	4c 01 e9             	add    rcx,r13
   140b2d492:	66 0f 6f 85 f0 01 00 	movdqa xmm0,XMMWORD PTR [rbp+0x1f0]
   140b2d499:	00 
   140b2d49a:	66 0f 6f 8d 00 02 00 	movdqa xmm1,XMMWORD PTR [rbp+0x200]
   140b2d4a1:	00 
   140b2d4a2:	48 8b 95 10 02 00 00 	mov    rdx,QWORD PTR [rbp+0x210]
   140b2d4a9:	48 89 54 c8 20       	mov    QWORD PTR [rax+rcx*8+0x20],rdx
   140b2d4ae:	f3 0f 7f 4c c8 10    	movdqu XMMWORD PTR [rax+rcx*8+0x10],xmm1
   140b2d4b4:	f3 0f 7f 04 c8       	movdqu XMMWORD PTR [rax+rcx*8],xmm0
   140b2d4b9:	49 ff c5             	inc    r13
   140b2d4bc:	4c 89 6b 20          	mov    QWORD PTR [rbx+0x20],r13
   140b2d4c0:	49 39 f6             	cmp    r14,rsi
   140b2d4c3:	0f 83 c4 00 00 00    	jae    0x140b2d58d
   140b2d4c9:	4c 89 f1             	mov    rcx,r14
   140b2d4cc:	e8 df 3c 0e 00       	call   0x140c111b0
   140b2d4d1:	89 c1                	mov    ecx,eax
   140b2d4d3:	83 f8 2c             	cmp    eax,0x2c
   140b2d4d6:	0f 83 87 04 00 00    	jae    0x140b2d963
   140b2d4dc:	4c 8d 2c 92          	lea    r13,[rdx+rdx*4]
   140b2d4e0:	49 c1 e5 03          	shl    r13,0x3
   140b2d4e4:	48 8b 85 50 02 00 00 	mov    rax,QWORD PTR [rbp+0x250]
   140b2d4eb:	48 8b 50 20          	mov    rdx,QWORD PTR [rax+0x20]
   140b2d4ef:	b8 01 00 00 00       	mov    eax,0x1
   140b2d4f4:	4c 03 6c cd c0       	add    r13,QWORD PTR [rbp+rcx*8-0x40]
   140b2d4f9:	f0 48 0f b1 7a 08    	lock cmpxchg QWORD PTR [rdx+0x8],rdi
   140b2d4ff:	0f 85 50 04 00 00    	jne    0x140b2d955
   140b2d505:	48 8b 02             	mov    rax,QWORD PTR [rdx]
   140b2d508:	48 c7 42 08 01 00 00 	mov    QWORD PTR [rdx+0x8],0x1
   140b2d50f:	00 
   140b2d510:	48 83 f8 01          	cmp    rax,0x1
   140b2d514:	0f 85 3b 04 00 00    	jne    0x140b2d955
   140b2d51a:	48 8b 85 50 02 00 00 	mov    rax,QWORD PTR [rbp+0x250]
   140b2d521:	48 8b 58 20          	mov    rbx,QWORD PTR [rax+0x20]
   140b2d525:	4c 89 f9             	mov    rcx,r15
   140b2d528:	4c 89 ea             	mov    rdx,r13
   140b2d52b:	e8 60 e8 24 00       	call   0x140d7bd90
   140b2d530:	49 8b 45 18          	mov    rax,QWORD PTR [r13+0x18]
   140b2d534:	49 8b 4d 20          	mov    rcx,QWORD PTR [r13+0x20]
   140b2d538:	f0 48 ff 00          	lock inc QWORD PTR [rax]
   140b2d53c:	0f 8e 32 04 00 00    	jle    0x140b2d974
   140b2d542:	48 8b 95 40 01 00 00 	mov    rdx,QWORD PTR [rbp+0x140]
   140b2d549:	48 89 95 00 02 00 00 	mov    QWORD PTR [rbp+0x200],rdx
   140b2d550:	f3 0f 6f 85 30 01 00 	movdqu xmm0,XMMWORD PTR [rbp+0x130]
   140b2d557:	00 
   140b2d558:	66 0f 7f 85 f0 01 00 	movdqa XMMWORD PTR [rbp+0x1f0],xmm0
   140b2d55f:	00 
   140b2d560:	48 89 85 08 02 00 00 	mov    QWORD PTR [rbp+0x208],rax
   140b2d567:	48 89 8d 10 02 00 00 	mov    QWORD PTR [rbp+0x210],rcx
   140b2d56e:	4c 8b 6b 20          	mov    r13,QWORD PTR [rbx+0x20]
   140b2d572:	4c 3b 6b 10          	cmp    r13,QWORD PTR [rbx+0x10]
   140b2d576:	0f 85 04 ff ff ff    	jne    0x140b2d480
   140b2d57c:	48 8d 4b 10          	lea    rcx,[rbx+0x10]
   140b2d580:	4c 89 e2             	mov    rdx,r12
   140b2d583:	e8 28 55 a3 ff       	call   0x140562ab0
   140b2d588:	e9 f3 fe ff ff       	jmp    0x140b2d480
   140b2d58d:	48 8d 4d c0          	lea    rcx,[rbp-0x40]
   140b2d591:	e8 aa 01 d5 ff       	call   0x14087d740
   140b2d596:	48 8b 85 50 02 00 00 	mov    rax,QWORD PTR [rbp+0x250]
   140b2d59d:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
   140b2d5a1:	48 8b 78 18          	mov    rdi,QWORD PTR [rax+0x18]
   140b2d5a5:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
   140b2d5a9:	48 8d 04 80          	lea    rax,[rax+rax*4]
   140b2d5ad:	48 8d 04 c7          	lea    rax,[rdi+rax*8]
   140b2d5b1:	48 89 85 58 01 00 00 	mov    QWORD PTR [rbp+0x158],rax
   140b2d5b8:	48 8b 85 40 02 00 00 	mov    rax,QWORD PTR [rbp+0x240]
   140b2d5bf:	48 8b 00             	mov    rax,QWORD PTR [rax]
   140b2d5c2:	48 89 85 60 01 00 00 	mov    QWORD PTR [rbp+0x160],rax
   140b2d5c9:	48 8b 85 28 02 00 00 	mov    rax,QWORD PTR [rbp+0x228]
   140b2d5d0:	48 8b 00             	mov    rax,QWORD PTR [rax]
   140b2d5d3:	48 89 85 68 01 00 00 	mov    QWORD PTR [rbp+0x168],rax
   140b2d5da:	48 8d 5d c0          	lea    rbx,[rbp-0x40]
   140b2d5de:	66 0f 76 f6          	pcmpeqd xmm6,xmm6
   140b2d5e2:	e9 89 00 00 00       	jmp    0x140b2d670
   140b2d5e7:	66 0f 1f 84 00 00 00 	nop    WORD PTR [rax+rax*1+0x0]
   140b2d5ee:	00 00 
   140b2d5f0:	f3 42 0f 6f 0c 01    	movdqu xmm1,XMMWORD PTR [rcx+r8*1]
   140b2d5f6:	66 0f 6f d1          	movdqa xmm2,xmm1
   140b2d5fa:	66 0f 74 d0          	pcmpeqb xmm2,xmm0
   140b2d5fe:	66 44 0f d7 ca       	pmovmskb r9d,xmm2
   140b2d603:	45 85 c9             	test   r9d,r9d
   140b2d606:	74 38                	je     0x140b2d640
   140b2d608:	f3 45 0f bc d1       	tzcnt  r10d,r9d
   140b2d60d:	4d 01 c2             	add    r10,r8
   140b2d610:	49 21 d2             	and    r10,rdx
   140b2d613:	49 c1 e2 03          	shl    r10,0x3
   140b2d617:	49 89 cb             	mov    r11,rcx
   140b2d61a:	4d 29 d3             	sub    r11,r10
   140b2d61d:	4d 3b 6b f8          	cmp    r13,QWORD PTR [r11-0x8]
   140b2d621:	0f 84 26 02 00 00    	je     0x140b2d84d
   140b2d627:	45 8d 51 ff          	lea    r10d,[r9-0x1]
   140b2d62b:	66 45 21 ca          	and    r10w,r9w
   140b2d62f:	45 89 d1             	mov    r9d,r10d
   140b2d632:	75 d4                	jne    0x140b2d608
   140b2d634:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   140b2d63b:	00 00 00 00 00 
   140b2d640:	66 0f 74 ce          	pcmpeqb xmm1,xmm6
   140b2d644:	66 44 0f d7 c9       	pmovmskb r9d,xmm1
   140b2d649:	45 85 c9             	test   r9d,r9d
   140b2d64c:	0f 85 ae 00 00 00    	jne    0x140b2d700
   140b2d652:	49 01 c0             	add    r8,rax
   140b2d655:	49 83 c0 10          	add    r8,0x10
   140b2d659:	48 83 c0 10          	add    rax,0x10
   140b2d65d:	49 21 d0             	and    r8,rdx
   140b2d660:	eb 8e                	jmp    0x140b2d5f0
   140b2d662:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   140b2d669:	1f 84 00 00 00 00 00 
   140b2d670:	48 3b bd 58 01 00 00 	cmp    rdi,QWORD PTR [rbp+0x158]
   140b2d677:	0f 84 d9 01 00 00    	je     0x140b2d856
   140b2d67d:	48 8b 85 50 02 00 00 	mov    rax,QWORD PTR [rbp+0x250]
   140b2d684:	48 8b 70 38          	mov    rsi,QWORD PTR [rax+0x38]
   140b2d688:	f3 0f 6f 47 08       	movdqu xmm0,XMMWORD PTR [rdi+0x8]
   140b2d68d:	48 89 75 c0          	mov    QWORD PTR [rbp-0x40],rsi
   140b2d691:	f3 0f 7f 45 c8       	movdqu XMMWORD PTR [rbp-0x38],xmm0
   140b2d696:	48 89 d9             	mov    rcx,rbx
   140b2d699:	e8 72 d0 92 00       	call   0x14145a710
   140b2d69e:	49 89 c5             	mov    r13,rax
   140b2d6a1:	48 89 85 f0 01 00 00 	mov    QWORD PTR [rbp+0x1f0],rax
   140b2d6a8:	48 83 bd 88 01 00 00 	cmp    QWORD PTR [rbp+0x188],0x0
   140b2d6af:	00 
   140b2d6b0:	74 4e                	je     0x140b2d700
   140b2d6b2:	48 8d 8d 90 01 00 00 	lea    rcx,[rbp+0x190]
   140b2d6b9:	48 8d 95 f0 01 00 00 	lea    rdx,[rbp+0x1f0]
   140b2d6c0:	e8 ab 36 a8 ff       	call   0x1405b0d70
   140b2d6c5:	48 8b 8d 70 01 00 00 	mov    rcx,QWORD PTR [rbp+0x170]
   140b2d6cc:	48 8b 95 78 01 00 00 	mov    rdx,QWORD PTR [rbp+0x178]
   140b2d6d3:	49 89 d0             	mov    r8,rdx
   140b2d6d6:	49 21 c0             	and    r8,rax
   140b2d6d9:	48 c1 e8 39          	shr    rax,0x39
   140b2d6dd:	66 0f 6e c0          	movd   xmm0,eax
   140b2d6e1:	66 0f 60 c0          	punpcklbw xmm0,xmm0
   140b2d6e5:	f2 0f 70 c0 00       	pshuflw xmm0,xmm0,0x0
   140b2d6ea:	66 0f 70 c0 00       	pshufd xmm0,xmm0,0x0
   140b2d6ef:	31 c0                	xor    eax,eax
   140b2d6f1:	e9 fa fe ff ff       	jmp    0x140b2d5f0
   140b2d6f6:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   140b2d6fd:	00 00 00 
   140b2d700:	48 8b 47 18          	mov    rax,QWORD PTR [rdi+0x18]
   140b2d704:	48 8b 4f 20          	mov    rcx,QWORD PTR [rdi+0x20]
   140b2d708:	48 8b 51 10          	mov    rdx,QWORD PTR [rcx+0x10]
   140b2d70c:	4c 8b 51 28          	mov    r10,QWORD PTR [rcx+0x28]
   140b2d710:	48 ff ca             	dec    rdx
   140b2d713:	48 83 e2 f0          	and    rdx,0xfffffffffffffff0
   140b2d717:	48 01 c2             	add    rdx,rax
   140b2d71a:	48 83 c2 10          	add    rdx,0x10
   140b2d71e:	48 8b 85 48 02 00 00 	mov    rax,QWORD PTR [rbp+0x248]
   140b2d725:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140b2d72a:	48 89 d9             	mov    rcx,rbx
   140b2d72d:	4c 8b 85 60 01 00 00 	mov    r8,QWORD PTR [rbp+0x160]
   140b2d734:	4c 8b 8d 68 01 00 00 	mov    r9,QWORD PTR [rbp+0x168]
   140b2d73b:	41 ff d2             	call   r10
   140b2d73e:	48 89 bd 28 02 00 00 	mov    QWORD PTR [rbp+0x228],rdi
   140b2d745:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
   140b2d749:	48 89 85 40 02 00 00 	mov    QWORD PTR [rbp+0x240],rax
   140b2d750:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
   140b2d754:	48 89 85 38 02 00 00 	mov    QWORD PTR [rbp+0x238],rax
   140b2d75b:	4c 8b 7d d0          	mov    r15,QWORD PTR [rbp-0x30]
   140b2d75f:	4d 85 ff             	test   r15,r15
   140b2d762:	0f 84 b8 00 00 00    	je     0x140b2d820
   140b2d768:	49 c1 e7 03          	shl    r15,0x3
   140b2d76c:	31 db                	xor    ebx,ebx
   140b2d76e:	66 90                	xchg   ax,ax
   140b2d770:	48 8b 85 38 02 00 00 	mov    rax,QWORD PTR [rbp+0x238]
   140b2d777:	48 8b 3c 18          	mov    rdi,QWORD PTR [rax+rbx*1]
   140b2d77b:	48 89 f8             	mov    rax,rdi
   140b2d77e:	48 c1 e8 20          	shr    rax,0x20
   140b2d782:	0f 84 98 00 00 00    	je     0x140b2d820
   140b2d788:	48 8b 85 48 02 00 00 	mov    rax,QWORD PTR [rbp+0x248]
   140b2d78f:	4c 8b 70 08          	mov    r14,QWORD PTR [rax+0x8]
   140b2d793:	4d 85 f6             	test   r14,r14
   140b2d796:	75 0b                	jne    0x140b2d7a3
   140b2d798:	48 8b 85 48 02 00 00 	mov    rax,QWORD PTR [rbp+0x248]
   140b2d79f:	4c 8b 70 10          	mov    r14,QWORD PTR [rax+0x10]
   140b2d7a3:	49 8b 06             	mov    rax,QWORD PTR [r14]
   140b2d7a6:	4d 8b 66 10          	mov    r12,QWORD PTR [r14+0x10]
   140b2d7aa:	4c 29 e0             	sub    rax,r12
   140b2d7ad:	48 83 f8 2f          	cmp    rax,0x2f
   140b2d7b1:	76 47                	jbe    0x140b2d7fa
   140b2d7b3:	49 8b 46 08          	mov    rax,QWORD PTR [r14+0x8]
   140b2d7b7:	48 8d 0d 52 c2 7a 00 	lea    rcx,[rip+0x7ac252]        # 0x1412d9a10
   140b2d7be:	4a 89 0c 20          	mov    QWORD PTR [rax+r12*1],rcx
   140b2d7c2:	48 8d 0d 67 75 a3 00 	lea    rcx,[rip+0xa37567]        # 0x141564d30
   140b2d7c9:	4a 89 4c 20 08       	mov    QWORD PTR [rax+r12*1+0x8],rcx
   140b2d7ce:	4a 89 7c 20 10       	mov    QWORD PTR [rax+r12*1+0x10],rdi
   140b2d7d3:	4e 89 6c 20 18       	mov    QWORD PTR [rax+r12*1+0x18],r13
   140b2d7d8:	4a 89 74 20 20       	mov    QWORD PTR [rax+r12*1+0x20],rsi
   140b2d7dd:	42 c6 44 20 28 00    	mov    BYTE PTR [rax+r12*1+0x28],0x0
   140b2d7e3:	49 83 c4 30          	add    r12,0x30
   140b2d7e7:	4d 89 66 10          	mov    QWORD PTR [r14+0x10],r12
   140b2d7eb:	48 83 c3 08          	add    rbx,0x8
   140b2d7ef:	49 39 df             	cmp    r15,rbx
   140b2d7f2:	0f 85 78 ff ff ff    	jne    0x140b2d770
   140b2d7f8:	eb 26                	jmp    0x140b2d820
   140b2d7fa:	48 c7 44 24 20 01 00 	mov    QWORD PTR [rsp+0x20],0x1
   140b2d801:	00 00 
   140b2d803:	41 b8 30 00 00 00    	mov    r8d,0x30
   140b2d809:	41 b9 01 00 00 00    	mov    r9d,0x1
   140b2d80f:	4c 89 f1             	mov    rcx,r14
   140b2d812:	4c 89 e2             	mov    rdx,r12
   140b2d815:	e8 46 c2 d7 01       	call   0x1428a9a60
   140b2d81a:	eb 97                	jmp    0x140b2d7b3
   140b2d81c:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]
   140b2d820:	48 8b 95 40 02 00 00 	mov    rdx,QWORD PTR [rbp+0x240]
   140b2d827:	48 85 d2             	test   rdx,rdx
   140b2d82a:	48 8b bd 28 02 00 00 	mov    rdi,QWORD PTR [rbp+0x228]
   140b2d831:	48 8d 5d c0          	lea    rbx,[rbp-0x40]
   140b2d835:	74 16                	je     0x140b2d84d
   140b2d837:	48 c1 e2 03          	shl    rdx,0x3
   140b2d83b:	41 b8 08 00 00 00    	mov    r8d,0x8
   140b2d841:	48 8b 8d 38 02 00 00 	mov    rcx,QWORD PTR [rbp+0x238]
   140b2d848:	e8 d3 63 ae ff       	call   0x140613c20
   140b2d84d:	48 83 c7 28          	add    rdi,0x28
   140b2d851:	e9 1a fe ff ff       	jmp    0x140b2d670
   140b2d856:	48 8b 95 78 01 00 00 	mov    rdx,QWORD PTR [rbp+0x178]
   140b2d85d:	48 85 d2             	test   rdx,rdx
   140b2d860:	74 2a                	je     0x140b2d88c
   140b2d862:	48 8d 04 d5 17 00 00 	lea    rax,[rdx*8+0x17]
   140b2d869:	00 
   140b2d86a:	48 83 e0 f0          	and    rax,0xfffffffffffffff0
   140b2d86e:	48 01 c2             	add    rdx,rax
   140b2d871:	48 83 c2 11          	add    rdx,0x11
   140b2d875:	74 15                	je     0x140b2d88c
   140b2d877:	48 8b 8d 70 01 00 00 	mov    rcx,QWORD PTR [rbp+0x170]
   140b2d87e:	48 29 c1             	sub    rcx,rax
   140b2d881:	41 b8 10 00 00 00    	mov    r8d,0x10
   140b2d887:	e8 94 63 ae ff       	call   0x140613c20
   140b2d88c:	48 83 bd e0 01 00 00 	cmp    QWORD PTR [rbp+0x1e0],0x0
   140b2d893:	00 
   140b2d894:	74 43                	je     0x140b2d8d9
   140b2d896:	e8 95 a8 0e 00       	call   0x140c18130
   140b2d89b:	48 89 c1             	mov    rcx,rax
   140b2d89e:	48 8b 00             	mov    rax,QWORD PTR [rax]
   140b2d8a1:	48 83 f8 01          	cmp    rax,0x1
   140b2d8a5:	75 4d                	jne    0x140b2d8f4
   140b2d8a7:	48 83 c1 08          	add    rcx,0x8
   140b2d8ab:	48 83 39 00          	cmp    QWORD PTR [rcx],0x0
   140b2d8af:	75 7d                	jne    0x140b2d92e
   140b2d8b1:	48 c7 01 ff ff ff ff 	mov    QWORD PTR [rcx],0xffffffffffffffff
   140b2d8b8:	48 89 8d 48 02 00 00 	mov    QWORD PTR [rbp+0x248],rcx
   140b2d8bf:	48 83 c1 08          	add    rcx,0x8
   140b2d8c3:	48 8b 95 e8 01 00 00 	mov    rdx,QWORD PTR [rbp+0x1e8]
   140b2d8ca:	e8 21 98 0e 00       	call   0x140c170f0
   140b2d8cf:	48 8b 85 48 02 00 00 	mov    rax,QWORD PTR [rbp+0x248]
   140b2d8d6:	48 ff 00             	inc    QWORD PTR [rax]
   140b2d8d9:	0f 28 b5 60 02 00 00 	movaps xmm6,XMMWORD PTR [rbp+0x260]
   140b2d8e0:	48 81 c4 f8 02 00 00 	add    rsp,0x2f8
   140b2d8e7:	5b                   	pop    rbx
   140b2d8e8:	5f                   	pop    rdi
   140b2d8e9:	5e                   	pop    rsi
   140b2d8ea:	41 5c                	pop    r12
   140b2d8ec:	41 5d                	pop    r13
   140b2d8ee:	41 5e                	pop    r14
   140b2d8f0:	41 5f                	pop    r15
   140b2d8f2:	5d                   	pop    rbp
   140b2d8f3:	c3                   	ret
   140b2d8f4:	48 85 c0             	test   rax,rax
   140b2d8f7:	75 29                	jne    0x140b2d922
   140b2d8f9:	31 d2                	xor    edx,edx
   140b2d8fb:	e8 d0 ae dc 01       	call   0x1428f87d0
   140b2d900:	48 89 c1             	mov    rcx,rax
   140b2d903:	48 85 c0             	test   rax,rax
   140b2d906:	75 a3                	jne    0x140b2d8ab
   140b2d908:	eb 18                	jmp    0x140b2d922
   140b2d90a:	48 85 c0             	test   rax,rax
   140b2d90d:	75 13                	jne    0x140b2d922
   140b2d90f:	31 d2                	xor    edx,edx
   140b2d911:	e8 ba ae dc 01       	call   0x1428f87d0
   140b2d916:	48 89 c1             	mov    rcx,rax
   140b2d919:	48 85 c0             	test   rax,rax
   140b2d91c:	0f 85 b4 f8 ff ff    	jne    0x140b2d1d6
   140b2d922:	48 8d 0d 77 d1 10 02 	lea    rcx,[rip+0x210d177]        # 0x142c3aaa0
   140b2d929:	e8 a2 a7 da 01       	call   0x1428d80d0
   140b2d92e:	48 8d 0d 3b d2 10 02 	lea    rcx,[rip+0x210d23b]        # 0x142c3ab70
   140b2d935:	e8 16 b9 da 01       	call   0x1428d9250
   140b2d93a:	4c 8d 85 10 02 00 00 	lea    r8,[rbp+0x210]
   140b2d941:	48 8d 8d f0 01 00 00 	lea    rcx,[rbp+0x1f0]
   140b2d948:	41 b1 01             	mov    r9b,0x1
   140b2d94b:	e8 50 ce d7 01       	call   0x1428aa7a0
   140b2d950:	e9 17 fa ff ff       	jmp    0x140b2d36c
   140b2d955:	48 8d 0d 04 df fb 01 	lea    rcx,[rip+0x1fbdf04]        # 0x142aeb860
   140b2d95c:	e8 af b9 da 01       	call   0x1428d9310
   140b2d961:	eb 11                	jmp    0x140b2d974
   140b2d963:	4c 8d 05 0e 64 13 02 	lea    r8,[rip+0x213640e]        # 0x142c63d78
   140b2d96a:	ba 2c 00 00 00       	mov    edx,0x2c
   140b2d96f:	e8 a4 bb da 01       	call   0x1428d9518
   140b2d974:	0f 0b                	ud2

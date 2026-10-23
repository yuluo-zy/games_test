
Tiny Glade/tiny-glade.exe:     file format pei-x86-64


Disassembly of section .text:

000000014234b3f1 <.text+0x234a3f1>:
   14234b3f1:	48 8b 0e             	mov    rcx,QWORD PTR [rsi]
   14234b3f4:	31 c0                	xor    eax,eax
   14234b3f6:	48 89 ca             	mov    rdx,rcx
   14234b3f9:	4c 8b 85 00 02 00 00 	mov    r8,QWORD PTR [rbp+0x200]
   14234b400:	49 2b 50 18          	sub    rdx,QWORD PTR [r8+0x18]
   14234b404:	48 0f 42 d0          	cmovb  rdx,rax
   14234b408:	49 2b 48 38          	sub    rcx,QWORD PTR [r8+0x38]
   14234b40c:	48 0f 42 c8          	cmovb  rcx,rax
   14234b410:	49 89 d7             	mov    r15,rdx
   14234b413:	49 c1 e7 04          	shl    r15,0x4
   14234b417:	4d 03 78 08          	add    r15,QWORD PTR [r8+0x8]
   14234b41b:	4d 8b 60 10          	mov    r12,QWORD PTR [r8+0x10]
   14234b41f:	4d 8b 48 30          	mov    r9,QWORD PTR [r8+0x30]
   14234b423:	49 29 d4             	sub    r12,rdx
   14234b426:	4c 0f 42 e0          	cmovb  r12,rax
   14234b42a:	ba 08 00 00 00       	mov    edx,0x8
   14234b42f:	4c 0f 42 fa          	cmovb  r15,rdx
   14234b433:	48 89 cf             	mov    rdi,rcx
   14234b436:	48 c1 e7 04          	shl    rdi,0x4
   14234b43a:	49 03 78 28          	add    rdi,QWORD PTR [r8+0x28]
   14234b43e:	49 29 c9             	sub    r9,rcx
   14234b441:	4c 0f 42 c8          	cmovb  r9,rax
   14234b445:	48 0f 42 fa          	cmovb  rdi,rdx
   14234b449:	49 8b 40 40          	mov    rax,QWORD PTR [r8+0x40]
   14234b44d:	4c 29 c8             	sub    rax,r9
   14234b450:	4c 29 e0             	sub    rax,r12
   14234b453:	48 89 06             	mov    QWORD PTR [rsi],rax
   14234b456:	49 c1 e4 04          	shl    r12,0x4
   14234b45a:	4d 01 fc             	add    r12,r15
   14234b45d:	49 c1 e1 04          	shl    r9,0x4
   14234b461:	49 01 f9             	add    r9,rdi
   14234b464:	4c 89 8d 00 02 00 00 	mov    QWORD PTR [rbp+0x200],r9
   14234b46b:	48 8b 8d 18 02 00 00 	mov    rcx,QWORD PTR [rbp+0x218]
   14234b472:	8b 41 1c             	mov    eax,DWORD PTR [rcx+0x1c]
   14234b475:	89 85 54 02 00 00    	mov    DWORD PTR [rbp+0x254],eax
   14234b47b:	48 8b 01             	mov    rax,QWORD PTR [rcx]
   14234b47e:	48 89 85 58 02 00 00 	mov    QWORD PTR [rbp+0x258],rax
   14234b485:	48 8b 41 10          	mov    rax,QWORD PTR [rcx+0x10]
   14234b489:	48 89 85 28 02 00 00 	mov    QWORD PTR [rbp+0x228],rax
   14234b490:	8b 43 1c             	mov    eax,DWORD PTR [rbx+0x1c]
   14234b493:	89 85 18 02 00 00    	mov    DWORD PTR [rbp+0x218],eax
   14234b499:	48 8b 43 10          	mov    rax,QWORD PTR [rbx+0x10]
   14234b49d:	48 89 85 40 02 00 00 	mov    QWORD PTR [rbp+0x240],rax
   14234b4a4:	49 8d 45 20          	lea    rax,[r13+0x20]
   14234b4a8:	48 89 85 20 02 00 00 	mov    QWORD PTR [rbp+0x220],rax
   14234b4af:	eb 69                	jmp    0x14234b51a
   14234b4b1:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   14234b4b8:	0f 1f 84 00 00 00 00 
   14234b4bf:	00 
   14234b4c0:	49 8b 45 28          	mov    rax,QWORD PTR [r13+0x28]
   14234b4c4:	48 89 f1             	mov    rcx,rsi
   14234b4c7:	48 c1 e1 04          	shl    rcx,0x4
   14234b4cb:	4c 89 34 08          	mov    QWORD PTR [rax+rcx*1],r14
   14234b4cf:	c6 44 08 08 01       	mov    BYTE PTR [rax+rcx*1+0x8],0x1
   14234b4d4:	48 ff c6             	inc    rsi
   14234b4d7:	49 89 75 30          	mov    QWORD PTR [r13+0x30],rsi
   14234b4db:	49 ff 45 40          	inc    QWORD PTR [r13+0x40]
   14234b4df:	4c 8b b5 48 02 00 00 	mov    r14,QWORD PTR [rbp+0x248]
   14234b4e6:	48 8b b5 08 02 00 00 	mov    rsi,QWORD PTR [rbp+0x208]
   14234b4ed:	0f b6 4b 08          	movzx  ecx,BYTE PTR [rbx+0x8]
   14234b4f1:	e8 0a 30 5c fe       	call   0x14090e500
   14234b4f6:	8b 8d 18 02 00 00    	mov    ecx,DWORD PTR [rbp+0x218]
   14234b4fc:	48 8b 95 40 02 00 00 	mov    rdx,QWORD PTR [rbp+0x240]
   14234b503:	89 0a                	mov    DWORD PTR [rdx],ecx
   14234b505:	48 8b 8d f8 01 00 00 	mov    rcx,QWORD PTR [rbp+0x1f8]
   14234b50c:	88 81 03 01 00 00    	mov    BYTE PTR [rcx+0x103],al
   14234b512:	0f b6 43 08          	movzx  eax,BYTE PTR [rbx+0x8]
   14234b516:	41 88 46 09          	mov    BYTE PTR [r14+0x9],al
   14234b51a:	4d 85 ff             	test   r15,r15
   14234b51d:	74 11                	je     0x14234b530
   14234b51f:	4d 39 e7             	cmp    r15,r12
   14234b522:	74 0c                	je     0x14234b530
   14234b524:	49 8d 47 10          	lea    rax,[r15+0x10]
   14234b528:	4c 89 fb             	mov    rbx,r15
   14234b52b:	eb 22                	jmp    0x14234b54f
   14234b52d:	0f 1f 00             	nop    DWORD PTR [rax]
   14234b530:	48 85 ff             	test   rdi,rdi
   14234b533:	0f 84 1a 01 00 00    	je     0x14234b653
   14234b539:	48 3b bd 00 02 00 00 	cmp    rdi,QWORD PTR [rbp+0x200]
   14234b540:	0f 84 0d 01 00 00    	je     0x14234b653
   14234b546:	48 89 fb             	mov    rbx,rdi
   14234b549:	48 83 c7 10          	add    rdi,0x10
   14234b54d:	31 c0                	xor    eax,eax
   14234b54f:	48 ff 06             	inc    QWORD PTR [rsi]
   14234b552:	49 89 c7             	mov    r15,rax
   14234b555:	0f b6 43 08          	movzx  eax,BYTE PTR [rbx+0x8]
   14234b559:	3c 04                	cmp    al,0x4
   14234b55b:	74 bd                	je     0x14234b51a
   14234b55d:	48 8b 8d f8 01 00 00 	mov    rcx,QWORD PTR [rbp+0x1f8]
   14234b564:	80 b9 03 01 00 00 03 	cmp    BYTE PTR [rcx+0x103],0x3
   14234b56b:	75 43                	jne    0x14234b5b0
   14234b56d:	48 8b 85 e0 01 00 00 	mov    rax,QWORD PTR [rbp+0x1e0]
   14234b574:	8b 8d 3c 02 00 00    	mov    ecx,DWORD PTR [rbp+0x23c]
   14234b57a:	89 08                	mov    DWORD PTR [rax],ecx
   14234b57c:	49 8b 75 30          	mov    rsi,QWORD PTR [r13+0x30]
   14234b580:	4d 8b 75 40          	mov    r14,QWORD PTR [r13+0x40]
   14234b584:	49 3b 75 20          	cmp    rsi,QWORD PTR [r13+0x20]
   14234b588:	0f 85 32 ff ff ff    	jne    0x14234b4c0
   14234b58e:	48 8b 8d 20 02 00 00 	mov    rcx,QWORD PTR [rbp+0x220]
   14234b595:	48 8d 15 d4 9c 86 00 	lea    rdx,[rip+0x869cd4]        # 0x142bb5270
   14234b59c:	e8 8f 70 21 fe       	call   0x140562630
   14234b5a1:	e9 1a ff ff ff       	jmp    0x14234b4c0
   14234b5a6:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   14234b5ad:	00 00 00 
   14234b5b0:	3c 02                	cmp    al,0x2
   14234b5b2:	0f 85 35 ff ff ff    	jne    0x14234b4ed
   14234b5b8:	48 8b 85 e0 01 00 00 	mov    rax,QWORD PTR [rbp+0x1e0]
   14234b5bf:	8b 8d 3c 02 00 00    	mov    ecx,DWORD PTR [rbp+0x23c]
   14234b5c5:	89 08                	mov    DWORD PTR [rax],ecx
   14234b5c7:	49 8b 75 30          	mov    rsi,QWORD PTR [r13+0x30]
   14234b5cb:	4d 8b 75 40          	mov    r14,QWORD PTR [r13+0x40]
   14234b5cf:	49 3b 75 20          	cmp    rsi,QWORD PTR [r13+0x20]
   14234b5d3:	75 13                	jne    0x14234b5e8
   14234b5d5:	48 8b 8d 20 02 00 00 	mov    rcx,QWORD PTR [rbp+0x220]
   14234b5dc:	48 8d 15 8d 9c 86 00 	lea    rdx,[rip+0x869c8d]        # 0x142bb5270
   14234b5e3:	e8 48 70 21 fe       	call   0x140562630
   14234b5e8:	49 8b 45 28          	mov    rax,QWORD PTR [r13+0x28]
   14234b5ec:	48 89 f1             	mov    rcx,rsi
   14234b5ef:	48 c1 e1 04          	shl    rcx,0x4
   14234b5f3:	4c 89 34 08          	mov    QWORD PTR [rax+rcx*1],r14
   14234b5f7:	c6 44 08 08 00       	mov    BYTE PTR [rax+rcx*1+0x8],0x0
   14234b5fc:	48 ff c6             	inc    rsi
   14234b5ff:	49 89 75 30          	mov    QWORD PTR [r13+0x30],rsi
   14234b603:	49 ff 45 40          	inc    QWORD PTR [r13+0x40]
   14234b607:	4c 8b b5 48 02 00 00 	mov    r14,QWORD PTR [rbp+0x248]
   14234b60e:	41 80 7e 0c 00       	cmp    BYTE PTR [r14+0xc],0x0
   14234b613:	48 8b b5 08 02 00 00 	mov    rsi,QWORD PTR [rbp+0x208]
   14234b61a:	0f 85 cd fe ff ff    	jne    0x14234b4ed
   14234b620:	8b 85 54 02 00 00    	mov    eax,DWORD PTR [rbp+0x254]
   14234b626:	48 8b 8d 28 02 00 00 	mov    rcx,QWORD PTR [rbp+0x228]
   14234b62d:	89 01                	mov    DWORD PTR [rcx],eax
   14234b62f:	48 8b 8d 58 02 00 00 	mov    rcx,QWORD PTR [rbp+0x258]
   14234b636:	48 8b 95 70 01 00 00 	mov    rdx,QWORD PTR [rbp+0x170]
   14234b63d:	4c 8b 85 78 01 00 00 	mov    r8,QWORD PTR [rbp+0x178]
   14234b644:	e8 c7 2e 5c fe       	call   0x14090e510
   14234b649:	41 c6 46 0c 01       	mov    BYTE PTR [r14+0xc],0x1
   14234b64e:	e9 9a fe ff ff       	jmp    0x14234b4ed

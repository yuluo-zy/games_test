
tiny-glade.exe:     file format pei-x86-64


Disassembly of section .text:

0000000140a69340 <.text+0xa68340>:
   140a69340:	55                   	push   rbp
   140a69341:	41 57                	push   r15
   140a69343:	41 56                	push   r14
   140a69345:	41 55                	push   r13
   140a69347:	41 54                	push   r12
   140a69349:	56                   	push   rsi
   140a6934a:	57                   	push   rdi
   140a6934b:	53                   	push   rbx
   140a6934c:	48 81 ec 18 01 00 00 	sub    rsp,0x118
   140a69353:	48 8d ac 24 80 00 00 	lea    rbp,[rsp+0x80]
   140a6935a:	00 
   140a6935b:	44 0f 29 85 80 00 00 	movaps XMMWORD PTR [rbp+0x80],xmm8
   140a69362:	00 
   140a69363:	0f 29 7d 70          	movaps XMMWORD PTR [rbp+0x70],xmm7
   140a69367:	0f 29 75 60          	movaps XMMWORD PTR [rbp+0x60],xmm6
   140a6936b:	48 c7 45 58 fe ff ff 	mov    QWORD PTR [rbp+0x58],0xfffffffffffffffe
   140a69372:	ff 
   140a69373:	48 89 d6             	mov    rsi,rdx
   140a69376:	48 89 cf             	mov    rdi,rcx
   140a69379:	e8 82 d6 1a 00       	call   0x140c16a00
   140a6937e:	84 c0                	test   al,al
   140a69380:	74 5f                	je     0x140a693e1
   140a69382:	4c 8d 35 97 b7 06 02 	lea    r14,[rip+0x206b797]        # 0x142ad4b20
   140a69389:	bb 47 00 00 00       	mov    ebx,0x47
   140a6938e:	ba 47 00 00 00       	mov    edx,0x47
   140a69393:	4c 89 f1             	mov    rcx,r14
   140a69396:	e8 e5 df ff ff       	call   0x140a67380
   140a6939b:	48 83 f8 01          	cmp    rax,0x1
   140a6939f:	0f 85 b1 00 00 00    	jne    0x140a69456
   140a693a5:	49 89 d1             	mov    r9,rdx
   140a693a8:	48 85 d2             	test   rdx,rdx
   140a693ab:	74 3d                	je     0x140a693ea
   140a693ad:	49 83 f9 47          	cmp    r9,0x47
   140a693b1:	73 35                	jae    0x140a693e8
   140a693b3:	48 8d 05 66 b7 06 02 	lea    rax,[rip+0x206b766]        # 0x142ad4b20
   140a693ba:	41 80 3c 01 bf       	cmp    BYTE PTR [r9+rax*1],0xbf
   140a693bf:	7f 29                	jg     0x140a693ea
   140a693c1:	48 8d 05 b0 b8 06 02 	lea    rax,[rip+0x206b8b0]        # 0x142ad4c78
   140a693c8:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140a693cd:	48 8d 0d 4c b7 06 02 	lea    rcx,[rip+0x206b74c]        # 0x142ad4b20
   140a693d4:	ba 47 00 00 00       	mov    edx,0x47
   140a693d9:	45 31 c0             	xor    r8d,r8d
   140a693dc:	e8 5f 05 e7 01       	call   0x1428d9940
   140a693e1:	31 c0                	xor    eax,eax
   140a693e3:	e9 ce 01 00 00       	jmp    0x140a695b6
   140a693e8:	75 d7                	jne    0x140a693c1
   140a693ea:	4c 8d 35 2f b7 06 02 	lea    r14,[rip+0x206b72f]        # 0x142ad4b20
   140a693f1:	4c 89 f1             	mov    rcx,r14
   140a693f4:	4c 89 ca             	mov    rdx,r9
   140a693f7:	e8 84 df ff ff       	call   0x140a67380
   140a693fc:	48 83 f8 01          	cmp    rax,0x1
   140a69400:	75 54                	jne    0x140a69456
   140a69402:	49 89 d0             	mov    r8,rdx
   140a69405:	49 83 c0 02          	add    r8,0x2
   140a69409:	74 39                	je     0x140a69444
   140a6940b:	49 83 f8 47          	cmp    r8,0x47
   140a6940f:	73 31                	jae    0x140a69442
   140a69411:	48 8d 05 08 b7 06 02 	lea    rax,[rip+0x206b708]        # 0x142ad4b20
   140a69418:	41 80 3c 00 bf       	cmp    BYTE PTR [r8+rax*1],0xbf
   140a6941d:	7f 25                	jg     0x140a69444
   140a6941f:	48 8d 05 6a b8 06 02 	lea    rax,[rip+0x206b86a]        # 0x142ad4c90
   140a69426:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140a6942b:	48 8d 0d ee b6 06 02 	lea    rcx,[rip+0x206b6ee]        # 0x142ad4b20
   140a69432:	ba 47 00 00 00       	mov    edx,0x47
   140a69437:	41 b9 47 00 00 00    	mov    r9d,0x47
   140a6943d:	e8 fe 04 e7 01       	call   0x1428d9940
   140a69442:	75 db                	jne    0x140a6941f
   140a69444:	bb 45 00 00 00       	mov    ebx,0x45
   140a69449:	48 29 d3             	sub    rbx,rdx
   140a6944c:	4c 8d 35 cd b6 06 02 	lea    r14,[rip+0x206b6cd]        # 0x142ad4b20
   140a69453:	4d 01 c6             	add    r14,r8
   140a69456:	48 8d 05 81 bd 06 02 	lea    rax,[rip+0x206bd81]        # 0x142ad51de
   140a6945d:	4c 8d 3d 40 bd 06 02 	lea    r15,[rip+0x206bd40]        # 0x142ad51a4
   140a69464:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   140a6946b:	00 00 00 00 00 
   140a69470:	0f be 48 ff          	movsx  ecx,BYTE PTR [rax-0x1]
   140a69474:	85 c9                	test   ecx,ecx
   140a69476:	78 18                	js     0x140a69490
   140a69478:	48 ff c8             	dec    rax
   140a6947b:	83 f9 5c             	cmp    ecx,0x5c
   140a6947e:	75 69                	jne    0x140a694e9
   140a69480:	eb 7d                	jmp    0x140a694ff
   140a69482:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   140a69489:	1f 84 00 00 00 00 00 
   140a69490:	0f b6 50 fe          	movzx  edx,BYTE PTR [rax-0x2]
   140a69494:	80 fa c0             	cmp    dl,0xc0
   140a69497:	7d 25                	jge    0x140a694be
   140a69499:	44 0f b6 40 fd       	movzx  r8d,BYTE PTR [rax-0x3]
   140a6949e:	41 80 f8 c0          	cmp    r8b,0xc0
   140a694a2:	7d 23                	jge    0x140a694c7
   140a694a4:	44 0f b6 48 fc       	movzx  r9d,BYTE PTR [rax-0x4]
   140a694a9:	48 83 c0 fc          	add    rax,0xfffffffffffffffc
   140a694ad:	41 83 e1 07          	and    r9d,0x7
   140a694b1:	41 c1 e1 06          	shl    r9d,0x6
   140a694b5:	41 83 e0 3f          	and    r8d,0x3f
   140a694b9:	45 09 c8             	or     r8d,r9d
   140a694bc:	eb 11                	jmp    0x140a694cf
   140a694be:	48 83 c0 fe          	add    rax,0xfffffffffffffffe
   140a694c2:	83 e2 1f             	and    edx,0x1f
   140a694c5:	eb 12                	jmp    0x140a694d9
   140a694c7:	48 83 c0 fd          	add    rax,0xfffffffffffffffd
   140a694cb:	41 83 e0 0f          	and    r8d,0xf
   140a694cf:	41 c1 e0 06          	shl    r8d,0x6
   140a694d3:	83 e2 3f             	and    edx,0x3f
   140a694d6:	44 09 c2             	or     edx,r8d
   140a694d9:	c1 e2 06             	shl    edx,0x6
   140a694dc:	80 e1 3f             	and    cl,0x3f
   140a694df:	0f b6 c9             	movzx  ecx,cl
   140a694e2:	09 d1                	or     ecx,edx
   140a694e4:	83 f9 5c             	cmp    ecx,0x5c
   140a694e7:	74 16                	je     0x140a694ff
   140a694e9:	83 f9 2f             	cmp    ecx,0x2f
   140a694ec:	74 11                	je     0x140a694ff
   140a694ee:	4c 39 f8             	cmp    rax,r15
   140a694f1:	0f 85 79 ff ff ff    	jne    0x140a69470
   140a694f7:	41 bc 3a 00 00 00    	mov    r12d,0x3a
   140a694fd:	eb 50                	jmp    0x140a6954f
   140a694ff:	4c 8d 3d 9e bc 06 02 	lea    r15,[rip+0x206bc9e]        # 0x142ad51a4
   140a69506:	4c 29 f8             	sub    rax,r15
   140a69509:	49 89 c0             	mov    r8,rax
   140a6950c:	49 ff c0             	inc    r8
   140a6950f:	74 32                	je     0x140a69543
   140a69511:	49 83 f8 3a          	cmp    r8,0x3a
   140a69515:	73 2a                	jae    0x140a69541
   140a69517:	43 80 3c 38 bf       	cmp    BYTE PTR [r8+r15*1],0xbf
   140a6951c:	7f 25                	jg     0x140a69543
   140a6951e:	48 8d 05 33 b7 06 02 	lea    rax,[rip+0x206b733]        # 0x142ad4c58
   140a69525:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140a6952a:	48 8d 0d 73 bc 06 02 	lea    rcx,[rip+0x206bc73]        # 0x142ad51a4
   140a69531:	ba 3a 00 00 00       	mov    edx,0x3a
   140a69536:	41 b9 3a 00 00 00    	mov    r9d,0x3a
   140a6953c:	e8 ff 03 e7 01       	call   0x1428d9940
   140a69541:	75 db                	jne    0x140a6951e
   140a69543:	41 bc 39 00 00 00    	mov    r12d,0x39
   140a69549:	49 29 c4             	sub    r12,rax
   140a6954c:	4d 01 c7             	add    r15,r8
   140a6954f:	e8 dc eb 1a 00       	call   0x140c18130
   140a69554:	48 89 c1             	mov    rcx,rax
   140a69557:	48 8b 00             	mov    rax,QWORD PTR [rax]
   140a6955a:	48 83 f8 01          	cmp    rax,0x1
   140a6955e:	0f 85 56 05 00 00    	jne    0x140a69aba
   140a69564:	48 83 c1 08          	add    rcx,0x8
   140a69568:	48 83 39 00          	cmp    QWORD PTR [rcx],0x0
   140a6956c:	0f 85 6c 05 00 00    	jne    0x140a69ade
   140a69572:	48 c7 01 ff ff ff ff 	mov    QWORD PTR [rcx],0xffffffffffffffff
   140a69579:	48 89 4d 40          	mov    QWORD PTR [rbp+0x40],rcx
   140a6957d:	48 83 c1 08          	add    rcx,0x8
   140a69581:	4c 89 64 24 20       	mov    QWORD PTR [rsp+0x20],r12
   140a69586:	48 c7 44 24 30 00 00 	mov    QWORD PTR [rsp+0x30],0x0
   140a6958d:	00 00 
   140a6958f:	48 c7 44 24 28 01 00 	mov    QWORD PTR [rsp+0x28],0x1
   140a69596:	00 00 
   140a69598:	4c 89 f2             	mov    rdx,r14
   140a6959b:	49 89 d8             	mov    r8,rbx
   140a6959e:	4d 89 f9             	mov    r9,r15
   140a695a1:	e8 9a d7 1a 00       	call   0x140c16d40
   140a695a6:	48 8b 4d 40          	mov    rcx,QWORD PTR [rbp+0x40]
   140a695aa:	48 ff 01             	inc    QWORD PTR [rcx]
   140a695ad:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
   140a695b1:	b8 01 00 00 00       	mov    eax,0x1
   140a695b6:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
   140a695ba:	48 8b 0f             	mov    rcx,QWORD PTR [rdi]
   140a695bd:	4c 8b 77 08          	mov    r14,QWORD PTR [rdi+0x8]
   140a695c1:	8b 47 14             	mov    eax,DWORD PTR [rdi+0x14]
   140a695c4:	89 45 3c             	mov    DWORD PTR [rbp+0x3c],eax
   140a695c7:	4c 8b b9 f8 00 00 00 	mov    r15,QWORD PTR [rcx+0xf8]
   140a695ce:	48 8b 81 00 01 00 00 	mov    rax,QWORD PTR [rcx+0x100]
   140a695d5:	4d 8d 24 87          	lea    r12,[r15+rax*4]
   140a695d9:	48 89 4d 28          	mov    QWORD PTR [rbp+0x28],rcx
   140a695dd:	0f b6 81 08 01 00 00 	movzx  eax,BYTE PTR [rcx+0x108]
   140a695e4:	88 45 4f             	mov    BYTE PTR [rbp+0x4f],al
   140a695e7:	48 8b 06             	mov    rax,QWORD PTR [rsi]
   140a695ea:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
   140a695ee:	b8 08 00 00 00       	mov    eax,0x8
   140a695f3:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
   140a695f7:	48 c7 45 10 00 00 00 	mov    QWORD PTR [rbp+0x10],0x0
   140a695fe:	00 
   140a695ff:	0f 57 f6             	xorps  xmm6,xmm6
   140a69602:	f3 0f 10 3d fa c2 eb 	movss  xmm7,DWORD PTR [rip+0x1ebc2fa]        # 0x142925904
   140a69609:	01 
   140a6960a:	44 0f 28 05 de c2 eb 	movaps xmm8,XMMWORD PTR [rip+0x1ebc2de]        # 0x1429258f0
   140a69611:	01 
   140a69612:	c7 45 50 00 00 00 00 	mov    DWORD PTR [rbp+0x50],0x0
   140a69619:	48 c7 45 30 00 00 00 	mov    QWORD PTR [rbp+0x30],0x0
   140a69620:	00 
   140a69621:	31 ff                	xor    edi,edi
   140a69623:	45 31 ed             	xor    r13d,r13d
   140a69626:	48 c7 45 20 00 00 00 	mov    QWORD PTR [rbp+0x20],0x0
   140a6962d:	00 
   140a6962e:	c7 45 54 00 00 00 00 	mov    DWORD PTR [rbp+0x54],0x0
   140a69635:	80 7d 4f 00          	cmp    BYTE PTR [rbp+0x4f],0x0
   140a69639:	0f 84 91 00 00 00    	je     0x140a696d0
   140a6963f:	4c 39 ef             	cmp    rdi,r13
   140a69642:	0f 85 58 01 00 00    	jne    0x140a697a0
   140a69648:	0f 1f 84 00 00 00 00 	nop    DWORD PTR [rax+rax*1+0x0]
   140a6964f:	00 
   140a69650:	4d 39 e7             	cmp    r15,r12
   140a69653:	0f 84 e6 03 00 00    	je     0x140a69a3f
   140a69659:	41 8b 0f             	mov    ecx,DWORD PTR [r15]
   140a6965c:	49 83 c7 04          	add    r15,0x4
   140a69660:	49 8b 86 a8 01 00 00 	mov    rax,QWORD PTR [r14+0x1a8]
   140a69667:	48 8d 0c c9          	lea    rcx,[rcx+rcx*8]
   140a6966b:	4c 8b 6c c8 10       	mov    r13,QWORD PTR [rax+rcx*8+0x10]
   140a69670:	4d 85 ed             	test   r13,r13
   140a69673:	74 db                	je     0x140a69650
   140a69675:	48 8d 0c c8          	lea    rcx,[rax+rcx*8]
   140a69679:	48 8b 51 40          	mov    rdx,QWORD PTR [rcx+0x40]
   140a6967d:	48 8b 41 18          	mov    rax,QWORD PTR [rcx+0x18]
   140a69681:	48 8b 49 38          	mov    rcx,QWORD PTR [rcx+0x38]
   140a69685:	4c 8b 45 28          	mov    r8,QWORD PTR [rbp+0x28]
   140a69689:	4d 8b 80 18 01 00 00 	mov    r8,QWORD PTR [r8+0x118]
   140a69690:	4c 39 c2             	cmp    rdx,r8
   140a69693:	0f 86 1f 01 00 00    	jbe    0x140a697b8
   140a69699:	4e 8b 04 c1          	mov    r8,QWORD PTR [rcx+r8*8]
   140a6969d:	4d 85 c0             	test   r8,r8
   140a696a0:	0f 84 12 01 00 00    	je     0x140a697b8
   140a696a6:	49 f7 d0             	not    r8
   140a696a9:	4f 8d 04 40          	lea    r8,[r8+r8*2]
   140a696ad:	49 c1 e0 04          	shl    r8,0x4
   140a696b1:	4e 8b 44 00 10       	mov    r8,QWORD PTR [rax+r8*1+0x10]
   140a696b6:	4c 89 45 30          	mov    QWORD PTR [rbp+0x30],r8
   140a696ba:	41 b0 01             	mov    r8b,0x1
   140a696bd:	44 89 45 50          	mov    DWORD PTR [rbp+0x50],r8d
   140a696c1:	e9 f9 00 00 00       	jmp    0x140a697bf
   140a696c6:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
   140a696cd:	00 00 00 
   140a696d0:	4c 39 ef             	cmp    rdi,r13
   140a696d3:	0f 85 d7 00 00 00    	jne    0x140a697b0
   140a696d9:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
   140a696e0:	4d 39 e7             	cmp    r15,r12
   140a696e3:	0f 84 56 03 00 00    	je     0x140a69a3f
   140a696e9:	41 8b 0f             	mov    ecx,DWORD PTR [r15]
   140a696ec:	49 83 c7 04          	add    r15,0x4
   140a696f0:	49 8b 86 00 01 00 00 	mov    rax,QWORD PTR [r14+0x100]
   140a696f7:	48 8d 0c 89          	lea    rcx,[rcx+rcx*4]
   140a696fb:	48 c1 e1 05          	shl    rcx,0x5
   140a696ff:	4c 8b 6c 08 58       	mov    r13,QWORD PTR [rax+rcx*1+0x58]
   140a69704:	4d 85 ed             	test   r13,r13
   140a69707:	74 d7                	je     0x140a696e0
   140a69709:	48 01 c8             	add    rax,rcx
   140a6970c:	8b 88 94 00 00 00    	mov    ecx,DWORD PTR [rax+0x94]
   140a69712:	4d 8b 8e a8 01 00 00 	mov    r9,QWORD PTR [r14+0x1a8]
   140a69719:	4c 8d 1c c9          	lea    r11,[rcx+rcx*8]
   140a6971d:	4b 8b 4c d9 18       	mov    rcx,QWORD PTR [r9+r11*8+0x18]
   140a69722:	4b 8b 54 d9 38       	mov    rdx,QWORD PTR [r9+r11*8+0x38]
   140a69727:	4c 8b 45 28          	mov    r8,QWORD PTR [rbp+0x28]
   140a6972b:	4d 8b 90 18 01 00 00 	mov    r10,QWORD PTR [r8+0x118]
   140a69732:	4c 8b 80 88 00 00 00 	mov    r8,QWORD PTR [rax+0x88]
   140a69739:	4d 39 d0             	cmp    r8,r10
   140a6973c:	0f 86 e4 00 00 00    	jbe    0x140a69826
   140a69742:	48 8b b0 80 00 00 00 	mov    rsi,QWORD PTR [rax+0x80]
   140a69749:	4a 83 3c d6 00       	cmp    QWORD PTR [rsi+r10*8],0x0
   140a6974e:	0f 84 d2 00 00 00    	je     0x140a69826
   140a69754:	40 b6 01             	mov    sil,0x1
   140a69757:	89 75 50             	mov    DWORD PTR [rbp+0x50],esi
   140a6975a:	4f 39 54 d9 40       	cmp    QWORD PTR [r9+r11*8+0x40],r10
   140a6975f:	0f 86 45 01 00 00    	jbe    0x140a698aa
   140a69765:	4e 8b 0c d2          	mov    r9,QWORD PTR [rdx+r10*8]
   140a69769:	4d 85 c9             	test   r9,r9
   140a6976c:	0f 84 38 01 00 00    	je     0x140a698aa
   140a69772:	49 f7 d1             	not    r9
   140a69775:	4f 8d 0c 49          	lea    r9,[r9+r9*2]
   140a69779:	49 c1 e1 04          	shl    r9,0x4
   140a6977d:	4e 8b 4c 09 10       	mov    r9,QWORD PTR [rcx+r9*1+0x10]
   140a69782:	4c 89 4d 30          	mov    QWORD PTR [rbp+0x30],r9
   140a69786:	4c 8b 4d 28          	mov    r9,QWORD PTR [rbp+0x28]
   140a6978a:	4d 8b 89 20 01 00 00 	mov    r9,QWORD PTR [r9+0x120]
   140a69791:	4d 39 c8             	cmp    r8,r9
   140a69794:	0f 87 28 01 00 00    	ja     0x140a698c2
   140a6979a:	e9 5b 01 00 00       	jmp    0x140a698fa
   140a6979f:	90                   	nop
   140a697a0:	48 89 f8             	mov    rax,rdi
   140a697a3:	e9 a7 00 00 00       	jmp    0x140a6984f
   140a697a8:	0f 1f 84 00 00 00 00 	nop    DWORD PTR [rax+rax*1+0x0]
   140a697af:	00 
   140a697b0:	48 89 f8             	mov    rax,rdi
   140a697b3:	e9 7f 01 00 00       	jmp    0x140a69937
   140a697b8:	c7 45 50 00 00 00 00 	mov    DWORD PTR [rbp+0x50],0x0
   140a697bf:	4c 8b 4d 28          	mov    r9,QWORD PTR [rbp+0x28]
   140a697c3:	4d 8b 81 10 01 00 00 	mov    r8,QWORD PTR [r9+0x110]
   140a697ca:	4e 8b 04 c1          	mov    r8,QWORD PTR [rcx+r8*8]
   140a697ce:	49 f7 d0             	not    r8
   140a697d1:	4f 8d 04 40          	lea    r8,[r8+r8*2]
   140a697d5:	49 c1 e0 04          	shl    r8,0x4
   140a697d9:	4e 8b 54 00 10       	mov    r10,QWORD PTR [rax+r8*1+0x10]
   140a697de:	4c 89 55 10          	mov    QWORD PTR [rbp+0x10],r10
   140a697e2:	4e 8b 44 00 28       	mov    r8,QWORD PTR [rax+r8*1+0x28]
   140a697e7:	4c 89 45 00          	mov    QWORD PTR [rbp+0x0],r8
   140a697eb:	4d 8b 81 20 01 00 00 	mov    r8,QWORD PTR [r9+0x120]
   140a697f2:	4c 39 c2             	cmp    rdx,r8
   140a697f5:	76 4f                	jbe    0x140a69846
   140a697f7:	4a 8b 0c c1          	mov    rcx,QWORD PTR [rcx+r8*8]
   140a697fb:	48 85 c9             	test   rcx,rcx
   140a697fe:	74 46                	je     0x140a69846
   140a69800:	48 f7 d1             	not    rcx
   140a69803:	48 8d 0c 49          	lea    rcx,[rcx+rcx*2]
   140a69807:	48 c1 e1 04          	shl    rcx,0x4
   140a6980b:	48 8b 54 08 10       	mov    rdx,QWORD PTR [rax+rcx*1+0x10]
   140a69810:	48 89 55 20          	mov    QWORD PTR [rbp+0x20],rdx
   140a69814:	48 8b 44 08 28       	mov    rax,QWORD PTR [rax+rcx*1+0x28]
   140a69819:	48 89 45 08          	mov    QWORD PTR [rbp+0x8],rax
   140a6981d:	b0 01                	mov    al,0x1
   140a6981f:	89 45 54             	mov    DWORD PTR [rbp+0x54],eax
   140a69822:	31 c0                	xor    eax,eax
   140a69824:	eb 29                	jmp    0x140a6984f
   140a69826:	c7 45 50 00 00 00 00 	mov    DWORD PTR [rbp+0x50],0x0
   140a6982d:	4c 8b 4d 28          	mov    r9,QWORD PTR [rbp+0x28]
   140a69831:	4d 8b 89 20 01 00 00 	mov    r9,QWORD PTR [r9+0x120]
   140a69838:	4d 39 c8             	cmp    r8,r9
   140a6983b:	0f 87 81 00 00 00    	ja     0x140a698c2
   140a69841:	e9 b4 00 00 00       	jmp    0x140a698fa
   140a69846:	31 c0                	xor    eax,eax
   140a69848:	c7 45 54 00 00 00 00 	mov    DWORD PTR [rbp+0x54],0x0
   140a6984f:	48 89 c7             	mov    rdi,rax
   140a69852:	89 c1                	mov    ecx,eax
   140a69854:	48 8d 04 c9          	lea    rax,[rcx+rcx*8]
   140a69858:	48 8d 34 40          	lea    rsi,[rax+rax*2]
   140a6985c:	48 8b 45 10          	mov    rax,QWORD PTR [rbp+0x10]
   140a69860:	48 01 c8             	add    rax,rcx
   140a69863:	48 01 c6             	add    rsi,rax
   140a69866:	48 8b 45 00          	mov    rax,QWORD PTR [rbp+0x0]
   140a6986a:	48 8d 04 88          	lea    rax,[rax+rcx*4]
   140a6986e:	48 89 45 40          	mov    QWORD PTR [rbp+0x40],rax
   140a69872:	f6 45 50 01          	test   BYTE PTR [rbp+0x50],0x1
   140a69876:	b8 00 00 00 00       	mov    eax,0x0
   140a6987b:	48 0f 45 45 30       	cmovne rax,QWORD PTR [rbp+0x30]
   140a69880:	f6 45 54 01          	test   BYTE PTR [rbp+0x54],0x1
   140a69884:	74 19                	je     0x140a6989f
   140a69886:	48 8b 55 20          	mov    rdx,QWORD PTR [rbp+0x20]
   140a6988a:	48 8d 1c ca          	lea    rbx,[rdx+rcx*8]
   140a6988e:	48 8b 55 08          	mov    rdx,QWORD PTR [rbp+0x8]
   140a69892:	48 8d 0c 8a          	lea    rcx,[rdx+rcx*4]
   140a69896:	48 89 4d d0          	mov    QWORD PTR [rbp-0x30],rcx
   140a6989a:	e9 f5 00 00 00       	jmp    0x140a69994
   140a6989f:	31 db                	xor    ebx,ebx
   140a698a1:	48 8b 4d d0          	mov    rcx,QWORD PTR [rbp-0x30]
   140a698a5:	e9 ea 00 00 00       	jmp    0x140a69994
   140a698aa:	48 c7 45 30 00 00 00 	mov    QWORD PTR [rbp+0x30],0x0
   140a698b1:	00 
   140a698b2:	4c 8b 4d 28          	mov    r9,QWORD PTR [rbp+0x28]
   140a698b6:	4d 8b 89 20 01 00 00 	mov    r9,QWORD PTR [r9+0x120]
   140a698bd:	4d 39 c8             	cmp    r8,r9
   140a698c0:	76 38                	jbe    0x140a698fa
   140a698c2:	4c 8b 80 80 00 00 00 	mov    r8,QWORD PTR [rax+0x80]
   140a698c9:	4b 83 3c c8 00       	cmp    QWORD PTR [r8+r9*8],0x0
   140a698ce:	74 2a                	je     0x140a698fa
   140a698d0:	4e 8b 04 ca          	mov    r8,QWORD PTR [rdx+r9*8]
   140a698d4:	49 f7 d0             	not    r8
   140a698d7:	4f 8d 04 40          	lea    r8,[r8+r8*2]
   140a698db:	49 c1 e0 04          	shl    r8,0x4
   140a698df:	4e 8b 4c 01 10       	mov    r9,QWORD PTR [rcx+r8*1+0x10]
   140a698e4:	4c 89 4d 20          	mov    QWORD PTR [rbp+0x20],r9
   140a698e8:	4e 8b 44 01 28       	mov    r8,QWORD PTR [rcx+r8*1+0x28]
   140a698ed:	4c 89 45 08          	mov    QWORD PTR [rbp+0x8],r8
   140a698f1:	41 b0 01             	mov    r8b,0x1
   140a698f4:	44 89 45 54          	mov    DWORD PTR [rbp+0x54],r8d
   140a698f8:	eb 07                	jmp    0x140a69901
   140a698fa:	c7 45 54 00 00 00 00 	mov    DWORD PTR [rbp+0x54],0x0
   140a69901:	4c 8b 45 28          	mov    r8,QWORD PTR [rbp+0x28]
   140a69905:	4d 8b 80 10 01 00 00 	mov    r8,QWORD PTR [r8+0x110]
   140a6990c:	4a 8b 14 c2          	mov    rdx,QWORD PTR [rdx+r8*8]
   140a69910:	48 f7 d2             	not    rdx
   140a69913:	48 8d 14 52          	lea    rdx,[rdx+rdx*2]
   140a69917:	48 c1 e2 04          	shl    rdx,0x4
   140a6991b:	4c 8b 44 11 10       	mov    r8,QWORD PTR [rcx+rdx*1+0x10]
   140a69920:	4c 89 45 10          	mov    QWORD PTR [rbp+0x10],r8
   140a69924:	48 8b 4c 11 28       	mov    rcx,QWORD PTR [rcx+rdx*1+0x28]
   140a69929:	48 89 4d 00          	mov    QWORD PTR [rbp+0x0],rcx
   140a6992d:	48 8b 40 50          	mov    rax,QWORD PTR [rax+0x50]
   140a69931:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
   140a69935:	31 c0                	xor    eax,eax
   140a69937:	48 89 c7             	mov    rdi,rax
   140a6993a:	48 c1 e0 04          	shl    rax,0x4
   140a6993e:	48 8b 4d f8          	mov    rcx,QWORD PTR [rbp-0x8]
   140a69942:	8b 4c 01 08          	mov    ecx,DWORD PTR [rcx+rax*1+0x8]
   140a69946:	48 8d 04 c9          	lea    rax,[rcx+rcx*8]
   140a6994a:	48 8d 34 40          	lea    rsi,[rax+rax*2]
   140a6994e:	48 8b 45 10          	mov    rax,QWORD PTR [rbp+0x10]
   140a69952:	48 01 c8             	add    rax,rcx
   140a69955:	48 01 c6             	add    rsi,rax
   140a69958:	48 8b 45 00          	mov    rax,QWORD PTR [rbp+0x0]
   140a6995c:	48 8d 04 88          	lea    rax,[rax+rcx*4]
   140a69960:	48 89 45 40          	mov    QWORD PTR [rbp+0x40],rax
   140a69964:	f6 45 50 01          	test   BYTE PTR [rbp+0x50],0x1
   140a69968:	b8 00 00 00 00       	mov    eax,0x0
   140a6996d:	48 0f 45 45 30       	cmovne rax,QWORD PTR [rbp+0x30]
   140a69972:	f6 45 54 01          	test   BYTE PTR [rbp+0x54],0x1
   140a69976:	74 16                	je     0x140a6998e
   140a69978:	48 8b 55 20          	mov    rdx,QWORD PTR [rbp+0x20]
   140a6997c:	48 8d 1c ca          	lea    rbx,[rdx+rcx*8]
   140a69980:	48 8b 55 08          	mov    rdx,QWORD PTR [rbp+0x8]
   140a69984:	48 8d 0c 8a          	lea    rcx,[rdx+rcx*4]
   140a69988:	48 89 4d d8          	mov    QWORD PTR [rbp-0x28],rcx
   140a6998c:	eb 06                	jmp    0x140a69994
   140a6998e:	31 db                	xor    ebx,ebx
   140a69990:	48 8b 4d d8          	mov    rcx,QWORD PTR [rbp-0x28]
   140a69994:	48 89 4d 18          	mov    QWORD PTR [rbp+0x18],rcx
   140a69998:	f3 0f 10 46 10       	movss  xmm0,DWORD PTR [rsi+0x10]
   140a6999d:	0f 2e c6             	ucomiss xmm0,xmm6
   140a699a0:	75 1e                	jne    0x140a699c0
   140a699a2:	7a 1c                	jp     0x140a699c0
   140a699a4:	0f 28 cf             	movaps xmm1,xmm7
   140a699a7:	48 85 c0             	test   rax,rax
   140a699aa:	74 03                	je     0x140a699af
   140a699ac:	0f 57 c9             	xorps  xmm1,xmm1
   140a699af:	8b 45 3c             	mov    eax,DWORD PTR [rbp+0x3c]
   140a699b2:	48 8b 4d 40          	mov    rcx,QWORD PTR [rbp+0x40]
   140a699b6:	89 01                	mov    DWORD PTR [rcx],eax
   140a699b8:	eb 3f                	jmp    0x140a699f9
   140a699ba:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
   140a699c0:	48 8b 4d f0          	mov    rcx,QWORD PTR [rbp-0x10]
   140a699c4:	f3 0f 10 49 28       	movss  xmm1,DWORD PTR [rcx+0x28]
   140a699c9:	48 85 c0             	test   rax,rax
   140a699cc:	74 04                	je     0x140a699d2
   140a699ce:	41 0f 57 c8          	xorps  xmm1,xmm8
   140a699d2:	0f 28 d7             	movaps xmm2,xmm7
   140a699d5:	f3 0f 5e d0          	divss  xmm2,xmm0
   140a699d9:	f3 0f 59 d1          	mulss  xmm2,xmm1
   140a699dd:	8b 45 3c             	mov    eax,DWORD PTR [rbp+0x3c]
   140a699e0:	48 8b 4d 40          	mov    rcx,QWORD PTR [rbp+0x40]
   140a699e4:	89 01                	mov    DWORD PTR [rcx],eax
   140a699e6:	f3 0f 58 56 14       	addss  xmm2,DWORD PTR [rsi+0x14]
   140a699eb:	0f 57 c0             	xorps  xmm0,xmm0
   140a699ee:	f3 0f 5f c2          	maxss  xmm0,xmm2
   140a699f2:	0f 28 cf             	movaps xmm1,xmm7
   140a699f5:	f3 0f 5d c8          	minss  xmm1,xmm0
   140a699f9:	f3 0f 11 4e 14       	movss  DWORD PTR [rsi+0x14],xmm1
   140a699fe:	48 89 f1             	mov    rcx,rsi
   140a69a01:	e8 ea c4 22 00       	call   0x140c95ef0
   140a69a06:	48 ff c7             	inc    rdi
   140a69a09:	8b 45 3c             	mov    eax,DWORD PTR [rbp+0x3c]
   140a69a0c:	48 8b 4d 40          	mov    rcx,QWORD PTR [rbp+0x40]
   140a69a10:	89 01                	mov    DWORD PTR [rcx],eax
   140a69a12:	f3 0f 11 46 18       	movss  DWORD PTR [rsi+0x18],xmm0
   140a69a17:	48 85 db             	test   rbx,rbx
   140a69a1a:	0f 84 15 fc ff ff    	je     0x140a69635
   140a69a20:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
   140a69a24:	f3 0f 10 40 28       	movss  xmm0,DWORD PTR [rax+0x28]
   140a69a29:	8b 45 3c             	mov    eax,DWORD PTR [rbp+0x3c]
   140a69a2c:	48 8b 4d 18          	mov    rcx,QWORD PTR [rbp+0x18]
   140a69a30:	89 01                	mov    DWORD PTR [rcx],eax
   140a69a32:	f3 0f 58 03          	addss  xmm0,DWORD PTR [rbx]
   140a69a36:	f3 0f 11 03          	movss  DWORD PTR [rbx],xmm0
   140a69a3a:	e9 f6 fb ff ff       	jmp    0x140a69635
   140a69a3f:	48 83 7d e0 00       	cmp    QWORD PTR [rbp-0x20],0x0
   140a69a44:	74 3a                	je     0x140a69a80
   140a69a46:	e8 e5 e6 1a 00       	call   0x140c18130
   140a69a4b:	48 89 c1             	mov    rcx,rax
   140a69a4e:	48 8b 00             	mov    rax,QWORD PTR [rax]
   140a69a51:	48 83 f8 01          	cmp    rax,0x1
   140a69a55:	75 4d                	jne    0x140a69aa4
   140a69a57:	48 83 c1 08          	add    rcx,0x8
   140a69a5b:	48 83 39 00          	cmp    QWORD PTR [rcx],0x0
   140a69a5f:	75 7d                	jne    0x140a69ade
   140a69a61:	48 c7 01 ff ff ff ff 	mov    QWORD PTR [rcx],0xffffffffffffffff
   140a69a68:	48 89 4d 40          	mov    QWORD PTR [rbp+0x40],rcx
   140a69a6c:	48 83 c1 08          	add    rcx,0x8
   140a69a70:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
   140a69a74:	e8 77 d6 1a 00       	call   0x140c170f0
   140a69a79:	48 8b 45 40          	mov    rax,QWORD PTR [rbp+0x40]
   140a69a7d:	48 ff 00             	inc    QWORD PTR [rax]
   140a69a80:	0f 28 75 60          	movaps xmm6,XMMWORD PTR [rbp+0x60]
   140a69a84:	0f 28 7d 70          	movaps xmm7,XMMWORD PTR [rbp+0x70]
   140a69a88:	44 0f 28 85 80 00 00 	movaps xmm8,XMMWORD PTR [rbp+0x80]
   140a69a8f:	00 
   140a69a90:	48 81 c4 18 01 00 00 	add    rsp,0x118
   140a69a97:	5b                   	pop    rbx
   140a69a98:	5f                   	pop    rdi
   140a69a99:	5e                   	pop    rsi
   140a69a9a:	41 5c                	pop    r12
   140a69a9c:	41 5d                	pop    r13
   140a69a9e:	41 5e                	pop    r14
   140a69aa0:	41 5f                	pop    r15
   140a69aa2:	5d                   	pop    rbp
   140a69aa3:	c3                   	ret
   140a69aa4:	48 85 c0             	test   rax,rax
   140a69aa7:	75 29                	jne    0x140a69ad2
   140a69aa9:	31 d2                	xor    edx,edx
   140a69aab:	e8 20 ed e8 01       	call   0x1428f87d0
   140a69ab0:	48 89 c1             	mov    rcx,rax
   140a69ab3:	48 85 c0             	test   rax,rax
   140a69ab6:	75 a3                	jne    0x140a69a5b
   140a69ab8:	eb 18                	jmp    0x140a69ad2
   140a69aba:	48 85 c0             	test   rax,rax
   140a69abd:	75 13                	jne    0x140a69ad2
   140a69abf:	31 d2                	xor    edx,edx
   140a69ac1:	e8 0a ed e8 01       	call   0x1428f87d0
   140a69ac6:	48 89 c1             	mov    rcx,rax
   140a69ac9:	48 85 c0             	test   rax,rax
   140a69acc:	0f 85 96 fa ff ff    	jne    0x140a69568
   140a69ad2:	48 8d 0d c7 0f 1d 02 	lea    rcx,[rip+0x21d0fc7]        # 0x142c3aaa0
   140a69ad9:	e8 f2 e5 e6 01       	call   0x1428d80d0
   140a69ade:	48 8d 0d 8b 10 1d 02 	lea    rcx,[rip+0x21d108b]        # 0x142c3ab70
   140a69ae5:	e8 66 f7 e6 01       	call   0x1428d9250
   140a69aea:	cc                   	int3

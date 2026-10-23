   1409d6320:	55                   	push   rbp
   1409d6321:	56                   	push   rsi
   1409d6322:	57                   	push   rdi
   1409d6323:	48 81 ec 40 02 00 00 	sub    rsp,0x240
   1409d632a:	48 8d ac 24 80 00 00 	lea    rbp,[rsp+0x80]
   1409d6331:	00 
   1409d6332:	48 c7 85 b8 01 00 00 	mov    QWORD PTR [rbp+0x1b8],0xfffffffffffffffe
   1409d6339:	fe ff ff ff 
   1409d633d:	48 89 d6             	mov    rsi,rdx
   1409d6340:	48 8b 39             	mov    rdi,QWORD PTR [rcx]
   1409d6343:	48 8b 17             	mov    rdx,QWORD PTR [rdi]
   1409d6346:	48 85 d2             	test   rdx,rdx
   1409d6349:	74 63                	je     0x1409d63ae
   1409d634b:	48 83 c2 10          	add    rdx,0x10
   1409d634f:	4c 8d 05 0f c1 0e 02 	lea    r8,[rip+0x20ec10f]        # 0x142ac2465
   1409d6356:	48 8d 8d b0 00 00 00 	lea    rcx,[rbp+0xb0]
   1409d635d:	41 b9 11 00 00 00    	mov    r9d,0x11
   1409d6363:	e8 d8 e0 07 00       	call   0x140a54440
   1409d6368:	48 83 bd b0 00 00 00 	cmp    QWORD PTR [rbp+0xb0],0x2
   1409d636f:	02 
   1409d6370:	0f 85 94 00 00 00    	jne    0x1409d640a
   1409d6376:	48 8b 85 b8 00 00 00 	mov    rax,QWORD PTR [rbp+0xb8]
   1409d637d:	48 89 45 78          	mov    QWORD PTR [rbp+0x78],rax
   1409d6381:	48 8d 05 28 c1 0e 02 	lea    rax,[rip+0x20ec128]        # 0x142ac24b0
   1409d6388:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   1409d638d:	48 8d 0d e4 ba 0e 02 	lea    rcx,[rip+0x20ebae4]        # 0x142ac1e78
   1409d6394:	4c 8d 0d bd ba 0e 02 	lea    r9,[rip+0x20ebabd]        # 0x142ac1e58
   1409d639b:	4c 8d 45 78          	lea    r8,[rbp+0x78]
   1409d639f:	ba 2b 00 00 00       	mov    edx,0x2b
   1409d63a4:	e8 b7 33 f0 01       	call   0x1428d9760
   1409d63a9:	e9 2a 06 00 00       	jmp    0x1409d69d8
   1409d63ae:	0f b6 05 54 e8 c5 02 	movzx  eax,BYTE PTR [rip+0x2c5e854]        # 0x143634c09
   1409d63b5:	b9 11 00 00 00       	mov    ecx,0x11
   1409d63ba:	ba 01 00 00 00       	mov    edx,0x1
   1409d63bf:	e8 4c d8 c3 ff       	call   0x140613c10
   1409d63c4:	48 85 c0             	test   rax,rax
   1409d63c7:	0f 84 af 05 00 00    	je     0x1409d697c
   1409d63cd:	0f 10 05 91 c0 0e 02 	movups xmm0,XMMWORD PTR [rip+0x20ec091]        # 0x142ac2465
   1409d63d4:	0f 11 00             	movups XMMWORD PTR [rax],xmm0
   1409d63d7:	c6 40 10 70          	mov    BYTE PTR [rax+0x10],0x70
   1409d63db:	48 c7 85 b0 00 00 00 	mov    QWORD PTR [rbp+0xb0],0x0
   1409d63e2:	00 00 00 00 
   1409d63e6:	48 c7 85 c0 00 00 00 	mov    QWORD PTR [rbp+0xc0],0x11
   1409d63ed:	11 00 00 00 
   1409d63f1:	48 89 85 c8 00 00 00 	mov    QWORD PTR [rbp+0xc8],rax
   1409d63f8:	48 c7 85 d0 00 00 00 	mov    QWORD PTR [rbp+0xd0],0x11
   1409d63ff:	11 00 00 00 
   1409d6403:	c6 85 d8 00 00 00 02 	mov    BYTE PTR [rbp+0xd8],0x2
   1409d640a:	48 8b 85 b0 00 00 00 	mov    rax,QWORD PTR [rbp+0xb0]
   1409d6411:	48 8b 8d b8 00 00 00 	mov    rcx,QWORD PTR [rbp+0xb8]
   1409d6418:	48 8b 95 c0 00 00 00 	mov    rdx,QWORD PTR [rbp+0xc0]
   1409d641f:	4c 8b 85 c8 00 00 00 	mov    r8,QWORD PTR [rbp+0xc8]
   1409d6426:	4c 8b 8d d0 00 00 00 	mov    r9,QWORD PTR [rbp+0xd0]
   1409d642d:	4c 89 4d 60          	mov    QWORD PTR [rbp+0x60],r9
   1409d6431:	44 0f b6 8d d8 00 00 	movzx  r9d,BYTE PTR [rbp+0xd8]
   1409d6438:	00 
   1409d6439:	44 88 4d 68          	mov    BYTE PTR [rbp+0x68],r9b
   1409d643d:	44 8b 8d d9 00 00 00 	mov    r9d,DWORD PTR [rbp+0xd9]
   1409d6444:	44 89 4d 69          	mov    DWORD PTR [rbp+0x69],r9d
   1409d6448:	44 0f b7 8d dd 00 00 	movzx  r9d,WORD PTR [rbp+0xdd]
   1409d644f:	00 
   1409d6450:	66 44 89 4d 6d       	mov    WORD PTR [rbp+0x6d],r9w
   1409d6455:	44 0f b6 8d df 00 00 	movzx  r9d,BYTE PTR [rbp+0xdf]
   1409d645c:	00 
   1409d645d:	44 88 4d 6f          	mov    BYTE PTR [rbp+0x6f],r9b
   1409d6461:	48 89 55 50          	mov    QWORD PTR [rbp+0x50],rdx
   1409d6465:	4c 89 45 58          	mov    QWORD PTR [rbp+0x58],r8
   1409d6469:	48 89 45 40          	mov    QWORD PTR [rbp+0x40],rax
   1409d646d:	48 89 4d 48          	mov    QWORD PTR [rbp+0x48],rcx
   1409d6471:	48 8b 17             	mov    rdx,QWORD PTR [rdi]
   1409d6474:	48 85 d2             	test   rdx,rdx
   1409d6477:	74 63                	je     0x1409d64dc
   1409d6479:	48 83 c2 10          	add    rdx,0x10
   1409d647d:	4c 8d 05 44 c0 0e 02 	lea    r8,[rip+0x20ec044]        # 0x142ac24c8
   1409d6484:	48 8d 8d b0 00 00 00 	lea    rcx,[rbp+0xb0]
   1409d648b:	41 b9 12 00 00 00    	mov    r9d,0x12
   1409d6491:	e8 aa df 07 00       	call   0x140a54440
   1409d6496:	48 83 bd b0 00 00 00 	cmp    QWORD PTR [rbp+0xb0],0x2
   1409d649d:	02 
   1409d649e:	0f 85 96 00 00 00    	jne    0x1409d653a
   1409d64a4:	48 8b 85 b8 00 00 00 	mov    rax,QWORD PTR [rbp+0xb8]
   1409d64ab:	48 89 45 78          	mov    QWORD PTR [rbp+0x78],rax
   1409d64af:	48 8d 05 2a c0 0e 02 	lea    rax,[rip+0x20ec02a]        # 0x142ac24e0
   1409d64b6:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   1409d64bb:	48 8d 0d b6 b9 0e 02 	lea    rcx,[rip+0x20eb9b6]        # 0x142ac1e78
   1409d64c2:	4c 8d 0d 8f b9 0e 02 	lea    r9,[rip+0x20eb98f]        # 0x142ac1e58
   1409d64c9:	4c 8d 45 78          	lea    r8,[rbp+0x78]
   1409d64cd:	ba 2b 00 00 00       	mov    edx,0x2b
   1409d64d2:	e8 89 32 f0 01       	call   0x1428d9760
   1409d64d7:	e9 fc 04 00 00       	jmp    0x1409d69d8
   1409d64dc:	0f b6 05 26 e7 c5 02 	movzx  eax,BYTE PTR [rip+0x2c5e726]        # 0x143634c09
   1409d64e3:	b9 12 00 00 00       	mov    ecx,0x12
   1409d64e8:	ba 01 00 00 00       	mov    edx,0x1
   1409d64ed:	e8 1e d7 c3 ff       	call   0x140613c10
   1409d64f2:	48 85 c0             	test   rax,rax
   1409d64f5:	0f 84 97 04 00 00    	je     0x1409d6992
   1409d64fb:	0f 10 05 c6 bf 0e 02 	movups xmm0,XMMWORD PTR [rip+0x20ebfc6]        # 0x142ac24c8
   1409d6502:	0f 11 00             	movups XMMWORD PTR [rax],xmm0
   1409d6505:	66 c7 40 10 61 62    	mov    WORD PTR [rax+0x10],0x6261
   1409d650b:	48 c7 85 b0 00 00 00 	mov    QWORD PTR [rbp+0xb0],0x0
   1409d6512:	00 00 00 00 
   1409d6516:	48 c7 85 c0 00 00 00 	mov    QWORD PTR [rbp+0xc0],0x12
   1409d651d:	12 00 00 00 
   1409d6521:	48 89 85 c8 00 00 00 	mov    QWORD PTR [rbp+0xc8],rax
   1409d6528:	48 c7 85 d0 00 00 00 	mov    QWORD PTR [rbp+0xd0],0x12
   1409d652f:	12 00 00 00 
   1409d6533:	c6 85 d8 00 00 00 02 	mov    BYTE PTR [rbp+0xd8],0x2
   1409d653a:	48 8b 85 b0 00 00 00 	mov    rax,QWORD PTR [rbp+0xb0]
   1409d6541:	48 8b 8d b8 00 00 00 	mov    rcx,QWORD PTR [rbp+0xb8]
   1409d6548:	48 8b 95 c0 00 00 00 	mov    rdx,QWORD PTR [rbp+0xc0]
   1409d654f:	4c 8b 85 c8 00 00 00 	mov    r8,QWORD PTR [rbp+0xc8]
   1409d6556:	4c 8b 8d d0 00 00 00 	mov    r9,QWORD PTR [rbp+0xd0]
   1409d655d:	4c 89 4d 30          	mov    QWORD PTR [rbp+0x30],r9
   1409d6561:	44 0f b6 8d d8 00 00 	movzx  r9d,BYTE PTR [rbp+0xd8]
   1409d6568:	00 
   1409d6569:	44 88 4d 38          	mov    BYTE PTR [rbp+0x38],r9b
   1409d656d:	44 8b 8d d9 00 00 00 	mov    r9d,DWORD PTR [rbp+0xd9]
   1409d6574:	44 89 4d 39          	mov    DWORD PTR [rbp+0x39],r9d
   1409d6578:	44 0f b7 8d dd 00 00 	movzx  r9d,WORD PTR [rbp+0xdd]
   1409d657f:	00 
   1409d6580:	66 44 89 4d 3d       	mov    WORD PTR [rbp+0x3d],r9w
   1409d6585:	44 0f b6 8d df 00 00 	movzx  r9d,BYTE PTR [rbp+0xdf]
   1409d658c:	00 
   1409d658d:	44 88 4d 3f          	mov    BYTE PTR [rbp+0x3f],r9b
   1409d6591:	48 89 55 20          	mov    QWORD PTR [rbp+0x20],rdx
   1409d6595:	4c 89 45 28          	mov    QWORD PTR [rbp+0x28],r8
   1409d6599:	48 89 45 10          	mov    QWORD PTR [rbp+0x10],rax
   1409d659d:	48 89 4d 18          	mov    QWORD PTR [rbp+0x18],rcx
   1409d65a1:	48 8b 17             	mov    rdx,QWORD PTR [rdi]
   1409d65a4:	48 85 d2             	test   rdx,rdx
   1409d65a7:	74 63                	je     0x1409d660c
   1409d65a9:	48 83 c2 10          	add    rdx,0x10
   1409d65ad:	4c 8d 05 44 bf 0e 02 	lea    r8,[rip+0x20ebf44]        # 0x142ac24f8
   1409d65b4:	48 8d 8d b0 00 00 00 	lea    rcx,[rbp+0xb0]
   1409d65bb:	41 b9 15 00 00 00    	mov    r9d,0x15
   1409d65c1:	e8 7a de 07 00       	call   0x140a54440
   1409d65c6:	48 83 bd b0 00 00 00 	cmp    QWORD PTR [rbp+0xb0],0x2
   1409d65cd:	02 
   1409d65ce:	0f 85 9e 00 00 00    	jne    0x1409d6672
   1409d65d4:	48 8b 85 b8 00 00 00 	mov    rax,QWORD PTR [rbp+0xb8]
   1409d65db:	48 89 45 78          	mov    QWORD PTR [rbp+0x78],rax
   1409d65df:	48 8d 05 2a bf 0e 02 	lea    rax,[rip+0x20ebf2a]        # 0x142ac2510
   1409d65e6:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   1409d65eb:	48 8d 0d 86 b8 0e 02 	lea    rcx,[rip+0x20eb886]        # 0x142ac1e78
   1409d65f2:	4c 8d 0d 5f b8 0e 02 	lea    r9,[rip+0x20eb85f]        # 0x142ac1e58
   1409d65f9:	4c 8d 45 78          	lea    r8,[rbp+0x78]
   1409d65fd:	ba 2b 00 00 00       	mov    edx,0x2b
   1409d6602:	e8 59 31 f0 01       	call   0x1428d9760
   1409d6607:	e9 cc 03 00 00       	jmp    0x1409d69d8
   1409d660c:	0f b6 05 f6 e5 c5 02 	movzx  eax,BYTE PTR [rip+0x2c5e5f6]        # 0x143634c09
   1409d6613:	b9 15 00 00 00       	mov    ecx,0x15
   1409d6618:	ba 01 00 00 00       	mov    edx,0x1
   1409d661d:	e8 ee d5 c3 ff       	call   0x140613c10
   1409d6622:	48 85 c0             	test   rax,rax
   1409d6625:	0f 84 7f 03 00 00    	je     0x1409d69aa
   1409d662b:	0f 10 05 c6 be 0e 02 	movups xmm0,XMMWORD PTR [rip+0x20ebec6]        # 0x142ac24f8
   1409d6632:	0f 11 00             	movups XMMWORD PTR [rax],xmm0
   1409d6635:	48 b9 2f 52 65 6c 65 	movabs rcx,0x657361656c65522f
   1409d663c:	61 73 65 
   1409d663f:	48 89 48 0d          	mov    QWORD PTR [rax+0xd],rcx
   1409d6643:	48 c7 85 b0 00 00 00 	mov    QWORD PTR [rbp+0xb0],0x0
   1409d664a:	00 00 00 00 
   1409d664e:	48 c7 85 c0 00 00 00 	mov    QWORD PTR [rbp+0xc0],0x15
   1409d6655:	15 00 00 00 
   1409d6659:	48 89 85 c8 00 00 00 	mov    QWORD PTR [rbp+0xc8],rax
   1409d6660:	48 c7 85 d0 00 00 00 	mov    QWORD PTR [rbp+0xd0],0x15
   1409d6667:	15 00 00 00 
   1409d666b:	c6 85 d8 00 00 00 02 	mov    BYTE PTR [rbp+0xd8],0x2
   1409d6672:	48 8b 85 b0 00 00 00 	mov    rax,QWORD PTR [rbp+0xb0]
   1409d6679:	48 8b 8d b8 00 00 00 	mov    rcx,QWORD PTR [rbp+0xb8]
   1409d6680:	48 8b 95 c0 00 00 00 	mov    rdx,QWORD PTR [rbp+0xc0]
   1409d6687:	4c 8b 85 c8 00 00 00 	mov    r8,QWORD PTR [rbp+0xc8]
   1409d668e:	4c 8b 8d d0 00 00 00 	mov    r9,QWORD PTR [rbp+0xd0]
   1409d6695:	4c 89 4d 00          	mov    QWORD PTR [rbp+0x0],r9
   1409d6699:	44 0f b6 8d d8 00 00 	movzx  r9d,BYTE PTR [rbp+0xd8]
   1409d66a0:	00 
   1409d66a1:	44 88 4d 08          	mov    BYTE PTR [rbp+0x8],r9b
   1409d66a5:	44 8b 8d d9 00 00 00 	mov    r9d,DWORD PTR [rbp+0xd9]
   1409d66ac:	44 89 4d 09          	mov    DWORD PTR [rbp+0x9],r9d
   1409d66b0:	44 0f b7 8d dd 00 00 	movzx  r9d,WORD PTR [rbp+0xdd]
   1409d66b7:	00 
   1409d66b8:	66 44 89 4d 0d       	mov    WORD PTR [rbp+0xd],r9w
   1409d66bd:	44 0f b6 8d df 00 00 	movzx  r9d,BYTE PTR [rbp+0xdf]
   1409d66c4:	00 
   1409d66c5:	44 88 4d 0f          	mov    BYTE PTR [rbp+0xf],r9b
   1409d66c9:	48 89 55 f0          	mov    QWORD PTR [rbp-0x10],rdx
   1409d66cd:	4c 89 45 f8          	mov    QWORD PTR [rbp-0x8],r8
   1409d66d1:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
   1409d66d5:	48 89 4d e8          	mov    QWORD PTR [rbp-0x18],rcx
   1409d66d9:	48 8b 17             	mov    rdx,QWORD PTR [rdi]
   1409d66dc:	48 85 d2             	test   rdx,rdx
   1409d66df:	74 63                	je     0x1409d6744
   1409d66e1:	48 83 c2 10          	add    rdx,0x10
   1409d66e5:	4c 8d 05 3c be 0e 02 	lea    r8,[rip+0x20ebe3c]        # 0x142ac2528
   1409d66ec:	48 8d 8d b0 00 00 00 	lea    rcx,[rbp+0xb0]
   1409d66f3:	41 b9 1b 00 00 00    	mov    r9d,0x1b
   1409d66f9:	e8 42 dd 07 00       	call   0x140a54440
   1409d66fe:	48 83 bd b0 00 00 00 	cmp    QWORD PTR [rbp+0xb0],0x2
   1409d6705:	02 
   1409d6706:	0f 85 9b 00 00 00    	jne    0x1409d67a7
   1409d670c:	48 8b 85 b8 00 00 00 	mov    rax,QWORD PTR [rbp+0xb8]
   1409d6713:	48 89 45 78          	mov    QWORD PTR [rbp+0x78],rax
   1409d6717:	48 8d 05 2a be 0e 02 	lea    rax,[rip+0x20ebe2a]        # 0x142ac2548
   1409d671e:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   1409d6723:	48 8d 0d 4e b7 0e 02 	lea    rcx,[rip+0x20eb74e]        # 0x142ac1e78
   1409d672a:	4c 8d 0d 27 b7 0e 02 	lea    r9,[rip+0x20eb727]        # 0x142ac1e58
   1409d6731:	4c 8d 45 78          	lea    r8,[rbp+0x78]
   1409d6735:	ba 2b 00 00 00       	mov    edx,0x2b
   1409d673a:	e8 21 30 f0 01       	call   0x1428d9760
   1409d673f:	e9 94 02 00 00       	jmp    0x1409d69d8
   1409d6744:	0f b6 05 be e4 c5 02 	movzx  eax,BYTE PTR [rip+0x2c5e4be]        # 0x143634c09
   1409d674b:	b9 1b 00 00 00       	mov    ecx,0x1b
   1409d6750:	ba 01 00 00 00       	mov    edx,0x1
   1409d6755:	e8 b6 d4 c3 ff       	call   0x140613c10
   1409d675a:	48 85 c0             	test   rax,rax
   1409d675d:	0f 84 5f 02 00 00    	je     0x1409d69c2
   1409d6763:	0f 10 05 c9 bd 0e 02 	movups xmm0,XMMWORD PTR [rip+0x20ebdc9]        # 0x142ac2533
   1409d676a:	0f 11 40 0b          	movups XMMWORD PTR [rax+0xb],xmm0
   1409d676e:	0f 10 05 b3 bd 0e 02 	movups xmm0,XMMWORD PTR [rip+0x20ebdb3]        # 0x142ac2528
   1409d6775:	0f 11 00             	movups XMMWORD PTR [rax],xmm0
   1409d6778:	48 c7 85 b0 00 00 00 	mov    QWORD PTR [rbp+0xb0],0x0
   1409d677f:	00 00 00 00 
   1409d6783:	48 c7 85 c0 00 00 00 	mov    QWORD PTR [rbp+0xc0],0x1b
   1409d678a:	1b 00 00 00 
   1409d678e:	48 89 85 c8 00 00 00 	mov    QWORD PTR [rbp+0xc8],rax
   1409d6795:	48 c7 85 d0 00 00 00 	mov    QWORD PTR [rbp+0xd0],0x1b
   1409d679c:	1b 00 00 00 
   1409d67a0:	c6 85 d8 00 00 00 02 	mov    BYTE PTR [rbp+0xd8],0x2
   1409d67a7:	48 8b 85 b0 00 00 00 	mov    rax,QWORD PTR [rbp+0xb0]
   1409d67ae:	48 8b 8d b8 00 00 00 	mov    rcx,QWORD PTR [rbp+0xb8]
   1409d67b5:	48 8b 95 c0 00 00 00 	mov    rdx,QWORD PTR [rbp+0xc0]
   1409d67bc:	4c 8b 85 c8 00 00 00 	mov    r8,QWORD PTR [rbp+0xc8]
   1409d67c3:	4c 8b 8d d0 00 00 00 	mov    r9,QWORD PTR [rbp+0xd0]
   1409d67ca:	4c 89 4d d0          	mov    QWORD PTR [rbp-0x30],r9
   1409d67ce:	44 0f b6 8d d8 00 00 	movzx  r9d,BYTE PTR [rbp+0xd8]
   1409d67d5:	00 
   1409d67d6:	44 88 4d d8          	mov    BYTE PTR [rbp-0x28],r9b
   1409d67da:	44 8b 8d d9 00 00 00 	mov    r9d,DWORD PTR [rbp+0xd9]
   1409d67e1:	44 89 4d d9          	mov    DWORD PTR [rbp-0x27],r9d
   1409d67e5:	44 0f b7 8d dd 00 00 	movzx  r9d,WORD PTR [rbp+0xdd]
   1409d67ec:	00 
   1409d67ed:	66 44 89 4d dd       	mov    WORD PTR [rbp-0x23],r9w
   1409d67f2:	44 0f b6 8d df 00 00 	movzx  r9d,BYTE PTR [rbp+0xdf]
   1409d67f9:	00 
   1409d67fa:	44 88 4d df          	mov    BYTE PTR [rbp-0x21],r9b
   1409d67fe:	48 89 55 c0          	mov    QWORD PTR [rbp-0x40],rdx
   1409d6802:	4c 89 45 c8          	mov    QWORD PTR [rbp-0x38],r8
   1409d6806:	48 89 45 b0          	mov    QWORD PTR [rbp-0x50],rax
   1409d680a:	48 89 4d b8          	mov    QWORD PTR [rbp-0x48],rcx
   1409d680e:	48 c7 85 c0 00 00 00 	mov    QWORD PTR [rbp+0xc0],0x3
   1409d6815:	03 00 00 00 
   1409d6819:	c7 85 c8 00 00 00 32 	mov    DWORD PTR [rbp+0xc8],0x32
   1409d6820:	00 00 00 
   1409d6823:	48 8d 05 36 bd 0e 02 	lea    rax,[rip+0x20ebd36]        # 0x142ac2560
   1409d682a:	48 89 85 b0 00 00 00 	mov    QWORD PTR [rbp+0xb0],rax
   1409d6831:	48 c7 85 b8 00 00 00 	mov    QWORD PTR [rbp+0xb8],0x21
   1409d6838:	21 00 00 00 
   1409d683c:	48 8d 4d 78          	lea    rcx,[rbp+0x78]
   1409d6840:	4c 8d 85 b0 00 00 00 	lea    r8,[rbp+0xb0]
   1409d6847:	48 89 fa             	mov    rdx,rdi
   1409d684a:	e8 d1 29 08 00       	call   0x140a59220
   1409d684f:	31 c0                	xor    eax,eax
   1409d6851:	48 3b 45 78          	cmp    rax,QWORD PTR [rbp+0x78]
   1409d6855:	0f 80 e6 00 00 00    	jo     0x1409d6941
   1409d685b:	48 8b 85 a8 00 00 00 	mov    rax,QWORD PTR [rbp+0xa8]
   1409d6862:	48 89 85 b0 01 00 00 	mov    QWORD PTR [rbp+0x1b0],rax
   1409d6869:	0f 10 45 78          	movups xmm0,XMMWORD PTR [rbp+0x78]
   1409d686d:	0f 10 8d 88 00 00 00 	movups xmm1,XMMWORD PTR [rbp+0x88]
   1409d6874:	0f 10 95 98 00 00 00 	movups xmm2,XMMWORD PTR [rbp+0x98]
   1409d687b:	0f 29 95 a0 01 00 00 	movaps XMMWORD PTR [rbp+0x1a0],xmm2
   1409d6882:	0f 29 8d 90 01 00 00 	movaps XMMWORD PTR [rbp+0x190],xmm1
   1409d6889:	0f 29 85 80 01 00 00 	movaps XMMWORD PTR [rbp+0x180],xmm0
   1409d6890:	0f 28 45 40          	movaps xmm0,XMMWORD PTR [rbp+0x40]
   1409d6894:	0f 28 4d 50          	movaps xmm1,XMMWORD PTR [rbp+0x50]
   1409d6898:	0f 28 55 60          	movaps xmm2,XMMWORD PTR [rbp+0x60]
   1409d689c:	0f 29 85 b0 00 00 00 	movaps XMMWORD PTR [rbp+0xb0],xmm0
   1409d68a3:	0f 29 8d c0 00 00 00 	movaps XMMWORD PTR [rbp+0xc0],xmm1
   1409d68aa:	0f 29 95 d0 00 00 00 	movaps XMMWORD PTR [rbp+0xd0],xmm2
   1409d68b1:	48 c7 85 e0 00 00 00 	mov    QWORD PTR [rbp+0xe0],0x0
   1409d68b8:	00 00 00 00 
   1409d68bc:	0f 28 45 10          	movaps xmm0,XMMWORD PTR [rbp+0x10]
   1409d68c0:	0f 28 4d 20          	movaps xmm1,XMMWORD PTR [rbp+0x20]
   1409d68c4:	0f 28 55 30          	movaps xmm2,XMMWORD PTR [rbp+0x30]
   1409d68c8:	0f 29 85 f0 00 00 00 	movaps XMMWORD PTR [rbp+0xf0],xmm0
   1409d68cf:	0f 29 8d 00 01 00 00 	movaps XMMWORD PTR [rbp+0x100],xmm1
   1409d68d6:	0f 29 95 10 01 00 00 	movaps XMMWORD PTR [rbp+0x110],xmm2
   1409d68dd:	0f 28 45 e0          	movaps xmm0,XMMWORD PTR [rbp-0x20]
   1409d68e1:	0f 28 4d f0          	movaps xmm1,XMMWORD PTR [rbp-0x10]
   1409d68e5:	0f 28 55 00          	movaps xmm2,XMMWORD PTR [rbp+0x0]
   1409d68e9:	0f 29 95 40 01 00 00 	movaps XMMWORD PTR [rbp+0x140],xmm2
   1409d68f0:	0f 29 8d 30 01 00 00 	movaps XMMWORD PTR [rbp+0x130],xmm1
   1409d68f7:	0f 29 85 20 01 00 00 	movaps XMMWORD PTR [rbp+0x120],xmm0
   1409d68fe:	0f 28 45 b0          	movaps xmm0,XMMWORD PTR [rbp-0x50]
   1409d6902:	0f 28 4d c0          	movaps xmm1,XMMWORD PTR [rbp-0x40]
   1409d6906:	0f 28 55 d0          	movaps xmm2,XMMWORD PTR [rbp-0x30]
   1409d690a:	0f 29 95 70 01 00 00 	movaps XMMWORD PTR [rbp+0x170],xmm2
   1409d6911:	0f 29 8d 60 01 00 00 	movaps XMMWORD PTR [rbp+0x160],xmm1
   1409d6918:	0f 29 85 50 01 00 00 	movaps XMMWORD PTR [rbp+0x150],xmm0
   1409d691f:	4c 8d 05 7a bc 0e 02 	lea    r8,[rip+0x20ebc7a]        # 0x142ac25a0
   1409d6926:	48 8d 95 b0 00 00 00 	lea    rdx,[rbp+0xb0]
   1409d692d:	48 89 f1             	mov    rcx,rsi
   1409d6930:	e8 bb 8f 90 00       	call   0x1412df8f0
   1409d6935:	90                   	nop
   1409d6936:	48 81 c4 40 02 00 00 	add    rsp,0x240
   1409d693d:	5f                   	pop    rdi
   1409d693e:	5e                   	pop    rsi
   1409d693f:	5d                   	pop    rbp
   1409d6940:	c3                   	ret
   1409d6941:	48 8b 85 80 00 00 00 	mov    rax,QWORD PTR [rbp+0x80]
   1409d6948:	48 89 85 b0 00 00 00 	mov    QWORD PTR [rbp+0xb0],rax
   1409d694f:	48 8d 05 32 bc 0e 02 	lea    rax,[rip+0x20ebc32]        # 0x142ac2588
   1409d6956:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   1409d695b:	48 8d 0d 16 b5 0e 02 	lea    rcx,[rip+0x20eb516]        # 0x142ac1e78
   1409d6962:	4c 8d 0d ef b4 0e 02 	lea    r9,[rip+0x20eb4ef]        # 0x142ac1e58
   1409d6969:	4c 8d 85 b0 00 00 00 	lea    r8,[rbp+0xb0]
   1409d6970:	ba 2b 00 00 00       	mov    edx,0x2b
   1409d6975:	e8 e6 2d f0 01       	call   0x1428d9760
   1409d697a:	eb 5c                	jmp    0x1409d69d8
   1409d697c:	4c 8d 05 2d 9e 0f 02 	lea    r8,[rip+0x20f9e2d]        # 0x142ad07b0
   1409d6983:	b9 01 00 00 00       	mov    ecx,0x1
   1409d6988:	ba 11 00 00 00       	mov    edx,0x11
   1409d698d:	e8 51 26 f0 01       	call   0x1428d8fe3
   1409d6992:	4c 8d 05 17 9e 0f 02 	lea    r8,[rip+0x20f9e17]        # 0x142ad07b0
   1409d6999:	b9 01 00 00 00       	mov    ecx,0x1
   1409d699e:	ba 12 00 00 00       	mov    edx,0x12
   1409d69a3:	e8 3b 26 f0 01       	call   0x1428d8fe3
   1409d69a8:	eb 2e                	jmp    0x1409d69d8
   1409d69aa:	4c 8d 05 ff 9d 0f 02 	lea    r8,[rip+0x20f9dff]        # 0x142ad07b0
   1409d69b1:	b9 01 00 00 00       	mov    ecx,0x1
   1409d69b6:	ba 15 00 00 00       	mov    edx,0x15
   1409d69bb:	e8 23 26 f0 01       	call   0x1428d8fe3
   1409d69c0:	eb 16                	jmp    0x1409d69d8
   1409d69c2:	4c 8d 05 e7 9d 0f 02 	lea    r8,[rip+0x20f9de7]        # 0x142ad07b0
   1409d69c9:	b9 01 00 00 00       	mov    ecx,0x1
   1409d69ce:	ba 1b 00 00 00       	mov    edx,0x1b
   1409d69d3:	e8 0b 26 f0 01       	call   0x1428d8fe3
   1409d69d8:	0f 0b                	ud2

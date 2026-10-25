
tiny-glade.exe:     file format pei-x86-64


Disassembly of section .text:

0000000140929160 <.text+0x928160>:
   140929160:	55                   	push   rbp
   140929161:	41 57                	push   r15
   140929163:	41 56                	push   r14
   140929165:	41 55                	push   r13
   140929167:	41 54                	push   r12
   140929169:	56                   	push   rsi
   14092916a:	57                   	push   rdi
   14092916b:	53                   	push   rbx
   14092916c:	48 81 ec 18 01 00 00 	sub    rsp,0x118
   140929173:	48 8d ac 24 80 00 00 	lea    rbp,[rsp+0x80]
   14092917a:	00 
   14092917b:	66 0f 7f b5 80 00 00 	movdqa XMMWORD PTR [rbp+0x80],xmm6
   140929182:	00 
   140929183:	48 c7 45 78 fe ff ff 	mov    QWORD PTR [rbp+0x78],0xfffffffffffffffe
   14092918a:	ff 
   14092918b:	4c 89 c3             	mov    rbx,r8
   14092918e:	49 89 d6             	mov    r14,rdx
   140929191:	48 89 cf             	mov    rdi,rcx
   140929194:	e8 67 d8 2e 00       	call   0x140c16a00
   140929199:	84 c0                	test   al,al
   14092919b:	88 45 77             	mov    BYTE PTR [rbp+0x77],al
   14092919e:	74 5f                	je     0x1409291ff
   1409291a0:	4c 8d 3d 61 54 18 02 	lea    r15,[rip+0x2185461]        # 0x142aae608
   1409291a7:	be 5a 00 00 00       	mov    esi,0x5a
   1409291ac:	ba 5a 00 00 00       	mov    edx,0x5a
   1409291b1:	4c 89 f9             	mov    rcx,r15
   1409291b4:	e8 67 a7 ff ff       	call   0x140923920
   1409291b9:	48 83 f8 01          	cmp    rax,0x1
   1409291bd:	0f 85 b1 00 00 00    	jne    0x140929274
   1409291c3:	49 89 d1             	mov    r9,rdx
   1409291c6:	48 85 d2             	test   rdx,rdx
   1409291c9:	74 3d                	je     0x140929208
   1409291cb:	49 83 f9 5a          	cmp    r9,0x5a
   1409291cf:	73 35                	jae    0x140929206
   1409291d1:	48 8d 05 30 54 18 02 	lea    rax,[rip+0x2185430]        # 0x142aae608
   1409291d8:	41 80 3c 01 bf       	cmp    BYTE PTR [r9+rax*1],0xbf
   1409291dd:	7f 29                	jg     0x140929208
   1409291df:	48 8d 05 52 55 18 02 	lea    rax,[rip+0x2185552]        # 0x142aae738
   1409291e6:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   1409291eb:	48 8d 0d 16 54 18 02 	lea    rcx,[rip+0x2185416]        # 0x142aae608
   1409291f2:	ba 5a 00 00 00       	mov    edx,0x5a
   1409291f7:	45 31 c0             	xor    r8d,r8d
   1409291fa:	e8 41 07 fb 01       	call   0x1428d9940
   1409291ff:	31 c0                	xor    eax,eax
   140929201:	e9 d4 01 00 00       	jmp    0x1409293da
   140929206:	75 d7                	jne    0x1409291df
   140929208:	4c 8d 3d f9 53 18 02 	lea    r15,[rip+0x21853f9]        # 0x142aae608
   14092920f:	4c 89 f9             	mov    rcx,r15
   140929212:	4c 89 ca             	mov    rdx,r9
   140929215:	e8 06 a7 ff ff       	call   0x140923920
   14092921a:	48 83 f8 01          	cmp    rax,0x1
   14092921e:	75 54                	jne    0x140929274
   140929220:	49 89 d0             	mov    r8,rdx
   140929223:	49 83 c0 02          	add    r8,0x2
   140929227:	74 39                	je     0x140929262
   140929229:	49 83 f8 5a          	cmp    r8,0x5a
   14092922d:	73 31                	jae    0x140929260
   14092922f:	48 8d 05 d2 53 18 02 	lea    rax,[rip+0x21853d2]        # 0x142aae608
   140929236:	41 80 3c 00 bf       	cmp    BYTE PTR [r8+rax*1],0xbf
   14092923b:	7f 25                	jg     0x140929262
   14092923d:	48 8d 05 0c 55 18 02 	lea    rax,[rip+0x218550c]        # 0x142aae750
   140929244:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   140929249:	48 8d 0d b8 53 18 02 	lea    rcx,[rip+0x21853b8]        # 0x142aae608
   140929250:	ba 5a 00 00 00       	mov    edx,0x5a
   140929255:	41 b9 5a 00 00 00    	mov    r9d,0x5a
   14092925b:	e8 e0 06 fb 01       	call   0x1428d9940
   140929260:	75 db                	jne    0x14092923d
   140929262:	be 58 00 00 00       	mov    esi,0x58
   140929267:	48 29 d6             	sub    rsi,rdx
   14092926a:	4c 8d 3d 97 53 18 02 	lea    r15,[rip+0x2185397]        # 0x142aae608
   140929271:	4d 01 c7             	add    r15,r8
   140929274:	48 8d 05 0b 63 18 02 	lea    rax,[rip+0x218630b]        # 0x142aaf586
   14092927b:	4c 8d 25 ba 62 18 02 	lea    r12,[rip+0x21862ba]        # 0x142aaf53c
   140929282:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   140929289:	1f 84 00 00 00 00 00 
   140929290:	0f be 48 ff          	movsx  ecx,BYTE PTR [rax-0x1]
   140929294:	85 c9                	test   ecx,ecx
   140929296:	78 18                	js     0x1409292b0
   140929298:	48 ff c8             	dec    rax
   14092929b:	83 f9 5c             	cmp    ecx,0x5c
   14092929e:	75 69                	jne    0x140929309
   1409292a0:	eb 7d                	jmp    0x14092931f
   1409292a2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   1409292a9:	1f 84 00 00 00 00 00 
   1409292b0:	0f b6 50 fe          	movzx  edx,BYTE PTR [rax-0x2]
   1409292b4:	80 fa c0             	cmp    dl,0xc0
   1409292b7:	7d 25                	jge    0x1409292de
   1409292b9:	44 0f b6 40 fd       	movzx  r8d,BYTE PTR [rax-0x3]
   1409292be:	41 80 f8 c0          	cmp    r8b,0xc0
   1409292c2:	7d 23                	jge    0x1409292e7
   1409292c4:	44 0f b6 48 fc       	movzx  r9d,BYTE PTR [rax-0x4]
   1409292c9:	48 83 c0 fc          	add    rax,0xfffffffffffffffc
   1409292cd:	41 83 e1 07          	and    r9d,0x7
   1409292d1:	41 c1 e1 06          	shl    r9d,0x6
   1409292d5:	41 83 e0 3f          	and    r8d,0x3f
   1409292d9:	45 09 c8             	or     r8d,r9d
   1409292dc:	eb 11                	jmp    0x1409292ef
   1409292de:	48 83 c0 fe          	add    rax,0xfffffffffffffffe
   1409292e2:	83 e2 1f             	and    edx,0x1f
   1409292e5:	eb 12                	jmp    0x1409292f9
   1409292e7:	48 83 c0 fd          	add    rax,0xfffffffffffffffd
   1409292eb:	41 83 e0 0f          	and    r8d,0xf
   1409292ef:	41 c1 e0 06          	shl    r8d,0x6
   1409292f3:	83 e2 3f             	and    edx,0x3f
   1409292f6:	44 09 c2             	or     edx,r8d
   1409292f9:	c1 e2 06             	shl    edx,0x6
   1409292fc:	80 e1 3f             	and    cl,0x3f
   1409292ff:	0f b6 c9             	movzx  ecx,cl
   140929302:	09 d1                	or     ecx,edx
   140929304:	83 f9 5c             	cmp    ecx,0x5c
   140929307:	74 16                	je     0x14092931f
   140929309:	83 f9 2f             	cmp    ecx,0x2f
   14092930c:	74 11                	je     0x14092931f
   14092930e:	4c 39 e0             	cmp    rax,r12
   140929311:	0f 85 79 ff ff ff    	jne    0x140929290
   140929317:	41 bd 4a 00 00 00    	mov    r13d,0x4a
   14092931d:	eb 50                	jmp    0x14092936f
   14092931f:	4c 8d 25 16 62 18 02 	lea    r12,[rip+0x2186216]        # 0x142aaf53c
   140929326:	4c 29 e0             	sub    rax,r12
   140929329:	49 89 c0             	mov    r8,rax
   14092932c:	49 ff c0             	inc    r8
   14092932f:	74 32                	je     0x140929363
   140929331:	49 83 f8 4a          	cmp    r8,0x4a
   140929335:	73 2a                	jae    0x140929361
   140929337:	43 80 3c 20 bf       	cmp    BYTE PTR [r8+r12*1],0xbf
   14092933c:	7f 25                	jg     0x140929363
   14092933e:	48 8d 05 d3 53 18 02 	lea    rax,[rip+0x21853d3]        # 0x142aae718
   140929345:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
   14092934a:	48 8d 0d eb 61 18 02 	lea    rcx,[rip+0x21861eb]        # 0x142aaf53c
   140929351:	ba 4a 00 00 00       	mov    edx,0x4a
   140929356:	41 b9 4a 00 00 00    	mov    r9d,0x4a
   14092935c:	e8 df 05 fb 01       	call   0x1428d9940
   140929361:	75 db                	jne    0x14092933e
   140929363:	41 bd 49 00 00 00    	mov    r13d,0x49
   140929369:	49 29 c5             	sub    r13,rax
   14092936c:	4d 01 c4             	add    r12,r8
   14092936f:	e8 bc ed 2e 00       	call   0x140c18130
   140929374:	48 89 c1             	mov    rcx,rax
   140929377:	48 8b 00             	mov    rax,QWORD PTR [rax]
   14092937a:	48 83 f8 01          	cmp    rax,0x1
   14092937e:	0f 85 87 05 00 00    	jne    0x14092990b
   140929384:	48 83 c1 08          	add    rcx,0x8
   140929388:	48 83 39 00          	cmp    QWORD PTR [rcx],0x0
   14092938c:	0f 85 b3 05 00 00    	jne    0x140929945
   140929392:	48 c7 01 ff ff ff ff 	mov    QWORD PTR [rcx],0xffffffffffffffff
   140929399:	48 89 4d 60          	mov    QWORD PTR [rbp+0x60],rcx
   14092939d:	48 83 c1 08          	add    rcx,0x8
   1409293a1:	4c 89 6c 24 20       	mov    QWORD PTR [rsp+0x20],r13
   1409293a6:	48 c7 44 24 30 00 00 	mov    QWORD PTR [rsp+0x30],0x0
   1409293ad:	00 00 
   1409293af:	48 c7 44 24 28 01 00 	mov    QWORD PTR [rsp+0x28],0x1
   1409293b6:	00 00 
   1409293b8:	4c 89 fa             	mov    rdx,r15
   1409293bb:	49 89 f0             	mov    r8,rsi
   1409293be:	4d 89 e1             	mov    r9,r12
   1409293c1:	e8 7a d9 2e 00       	call   0x140c16d40
   1409293c6:	48 8b 4d 60          	mov    rcx,QWORD PTR [rbp+0x60]
   1409293ca:	48 ff 01             	inc    QWORD PTR [rcx]
   1409293cd:	48 89 45 30          	mov    QWORD PTR [rbp+0x30],rax
   1409293d1:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
   1409293d5:	b8 01 00 00 00       	mov    eax,0x1
   1409293da:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
   1409293de:	31 c9                	xor    ecx,ecx
   1409293e0:	e8 3b 07 7b ff       	call   0x1400d9b20
   1409293e5:	48 85 c0             	test   rax,rax
   1409293e8:	0f 84 63 05 00 00    	je     0x140929951
   1409293ee:	0f 10 00             	movups xmm0,XMMWORD PTR [rax]
   1409293f1:	48 ff 00             	inc    QWORD PTR [rax]
   1409293f4:	0f 29 45 00          	movaps XMMWORD PTR [rbp+0x0],xmm0
   1409293f8:	0f 10 05 11 2b 34 02 	movups xmm0,XMMWORD PTR [rip+0x2342b11]        # 0x142c6bf10
   1409293ff:	0f 29 45 f0          	movaps XMMWORD PTR [rbp-0x10],xmm0
   140929403:	f3 0f 6f 05 f5 2a 34 	movdqu xmm0,XMMWORD PTR [rip+0x2342af5]        # 0x142c6bf00
   14092940a:	02 
   14092940b:	66 0f 7f 45 e0       	movdqa XMMWORD PTR [rbp-0x20],xmm0
   140929410:	48 8b 37             	mov    rsi,QWORD PTR [rdi]
   140929413:	80 7e 18 00          	cmp    BYTE PTR [rsi+0x18],0x0
   140929417:	74 59                	je     0x140929472
   140929419:	48 8b 46 20          	mov    rax,QWORD PTR [rsi+0x20]
   14092941d:	4c 8b 60 18          	mov    r12,QWORD PTR [rax+0x18]
   140929421:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
   140929425:	48 8d 04 80          	lea    rax,[rax+rax*4]
   140929429:	4d 8d 2c c4          	lea    r13,[r12+rax*8]
   14092942d:	48 8d 7d b8          	lea    rdi,[rbp-0x48]
   140929431:	4c 8d 7d e0          	lea    r15,[rbp-0x20]
   140929435:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
   14092943c:	00 00 00 00 
   140929440:	4d 39 ec             	cmp    r12,r13
   140929443:	74 2d                	je     0x140929472
   140929445:	48 8b 46 38          	mov    rax,QWORD PTR [rsi+0x38]
   140929449:	f3 41 0f 6f 44 24 08 	movdqu xmm0,XMMWORD PTR [r12+0x8]
   140929450:	48 89 45 b8          	mov    QWORD PTR [rbp-0x48],rax
   140929454:	f3 0f 7f 45 c0       	movdqu XMMWORD PTR [rbp-0x40],xmm0
   140929459:	48 89 f9             	mov    rcx,rdi
   14092945c:	e8 af 12 b3 00       	call   0x14145a710
   140929461:	49 83 c4 28          	add    r12,0x28
   140929465:	4c 89 f9             	mov    rcx,r15
   140929468:	48 89 c2             	mov    rdx,rax
   14092946b:	e8 80 0e 9a ff       	call   0x1402ca2f0
   140929470:	eb ce                	jmp    0x140929440
   140929472:	49 8b 0e             	mov    rcx,QWORD PTR [r14]
   140929475:	4c 8b a1 f8 00 00 00 	mov    r12,QWORD PTR [rcx+0xf8]
   14092947c:	48 8b 81 00 01 00 00 	mov    rax,QWORD PTR [rcx+0x100]
   140929483:	48 8b 53 08          	mov    rdx,QWORD PTR [rbx+0x8]
   140929487:	48 85 d2             	test   rdx,rdx
   14092948a:	75 04                	jne    0x140929490
   14092948c:	48 8b 53 10          	mov    rdx,QWORD PTR [rbx+0x10]
   140929490:	48 89 55 28          	mov    QWORD PTR [rbp+0x28],rdx
   140929494:	4d 8b 6e 08          	mov    r13,QWORD PTR [r14+0x8]
   140929498:	49 8d 34 84          	lea    rsi,[r12+rax*4]
   14092949c:	48 89 4d 50          	mov    QWORD PTR [rbp+0x50],rcx
   1409294a0:	0f b6 81 08 01 00 00 	movzx  eax,BYTE PTR [rcx+0x108]
   1409294a7:	88 45 60             	mov    BYTE PTR [rbp+0x60],al
   1409294aa:	b8 08 00 00 00       	mov    eax,0x8
   1409294af:	48 89 45 40          	mov    QWORD PTR [rbp+0x40],rax
   1409294b3:	66 0f 76 f6          	pcmpeqd xmm6,xmm6
   1409294b7:	48 89 45 48          	mov    QWORD PTR [rbp+0x48],rax
   1409294bb:	31 db                	xor    ebx,ebx
   1409294bd:	45 31 ff             	xor    r15d,r15d
   1409294c0:	48 c7 45 68 00 00 00 	mov    QWORD PTR [rbp+0x68],0x0
   1409294c7:	00 
   1409294c8:	48 c7 45 58 00 00 00 	mov    QWORD PTR [rbp+0x58],0x0
   1409294cf:	00 
   1409294d0:	c7 45 70 00 00 00 00 	mov    DWORD PTR [rbp+0x70],0x0
   1409294d7:	e9 84 00 00 00       	jmp    0x140929560
   1409294dc:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]
   1409294e0:	f3 42 0f 6f 0c 02    	movdqu xmm1,XMMWORD PTR [rdx+r8*1]
   1409294e6:	66 0f 6f d1          	movdqa xmm2,xmm1
   1409294ea:	66 0f 74 d0          	pcmpeqb xmm2,xmm0
   1409294ee:	66 44 0f d7 ca       	pmovmskb r9d,xmm2
   1409294f3:	45 85 c9             	test   r9d,r9d
   1409294f6:	74 38                	je     0x140929530
   1409294f8:	4d 8b 16             	mov    r10,QWORD PTR [r14]
   1409294fb:	48 89 c8             	mov    rax,rcx
   1409294fe:	f3 45 0f bc d9       	tzcnt  r11d,r9d
   140929503:	4d 01 c3             	add    r11,r8
   140929506:	4c 23 5d 38          	and    r11,QWORD PTR [rbp+0x38]
   14092950a:	49 c1 e3 03          	shl    r11,0x3
   14092950e:	48 89 d1             	mov    rcx,rdx
   140929511:	4c 29 d9             	sub    rcx,r11
   140929514:	4c 3b 51 f8          	cmp    r10,QWORD PTR [rcx-0x8]
   140929518:	74 46                	je     0x140929560
   14092951a:	41 8d 49 ff          	lea    ecx,[r9-0x1]
   14092951e:	66 44 21 c9          	and    cx,r9w
   140929522:	41 89 c9             	mov    r9d,ecx
   140929525:	48 89 c1             	mov    rcx,rax
   140929528:	75 d1                	jne    0x1409294fb
   14092952a:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
   140929530:	66 0f 74 ce          	pcmpeqb xmm1,xmm6
   140929534:	66 0f d7 c1          	pmovmskb eax,xmm1
   140929538:	85 c0                	test   eax,eax
   14092953a:	0f 85 c0 02 00 00    	jne    0x140929800
   140929540:	49 01 c8             	add    r8,rcx
   140929543:	49 83 c0 10          	add    r8,0x10
   140929547:	48 83 c1 10          	add    rcx,0x10
   14092954b:	4c 23 45 38          	and    r8,QWORD PTR [rbp+0x38]
   14092954f:	eb 8f                	jmp    0x1409294e0
   140929551:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   140929558:	0f 1f 84 00 00 00 00 
   14092955f:	00 
   140929560:	80 7d 60 00          	cmp    BYTE PTR [rbp+0x60],0x0
   140929564:	0f 84 96 00 00 00    	je     0x140929600
   14092956a:	49 39 df             	cmp    r15,rbx
   14092956d:	0f 85 dd 01 00 00    	jne    0x140929750
   140929573:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   14092957a:	84 00 00 00 00 00 
   140929580:	49 39 f4             	cmp    r12,rsi
   140929583:	0f 84 f7 02 00 00    	je     0x140929880
   140929589:	41 8b 0c 24          	mov    ecx,DWORD PTR [r12]
   14092958d:	49 83 c4 04          	add    r12,0x4
   140929591:	49 8b 85 a8 01 00 00 	mov    rax,QWORD PTR [r13+0x1a8]
   140929598:	48 8d 0c c9          	lea    rcx,[rcx+rcx*8]
   14092959c:	48 8b 5c c8 10       	mov    rbx,QWORD PTR [rax+rcx*8+0x10]
   1409295a1:	48 85 db             	test   rbx,rbx
   1409295a4:	74 da                	je     0x140929580
   1409295a6:	48 8d 04 c8          	lea    rax,[rax+rcx*8]
   1409295aa:	4c 8b 4d 50          	mov    r9,QWORD PTR [rbp+0x50]
   1409295ae:	49 8b 91 10 01 00 00 	mov    rdx,QWORD PTR [r9+0x110]
   1409295b5:	48 8b 48 40          	mov    rcx,QWORD PTR [rax+0x40]
   1409295b9:	48 39 d1             	cmp    rcx,rdx
   1409295bc:	0f 86 df 00 00 00    	jbe    0x1409296a1
   1409295c2:	4c 8b 40 38          	mov    r8,QWORD PTR [rax+0x38]
   1409295c6:	49 8b 14 d0          	mov    rdx,QWORD PTR [r8+rdx*8]
   1409295ca:	48 85 d2             	test   rdx,rdx
   1409295cd:	0f 84 ce 00 00 00    	je     0x1409296a1
   1409295d3:	4c 8b 40 18          	mov    r8,QWORD PTR [rax+0x18]
   1409295d7:	48 f7 d2             	not    rdx
   1409295da:	48 8d 14 52          	lea    rdx,[rdx+rdx*2]
   1409295de:	48 c1 e2 04          	shl    rdx,0x4
   1409295e2:	49 8b 54 10 10       	mov    rdx,QWORD PTR [r8+rdx*1+0x10]
   1409295e7:	48 89 55 68          	mov    QWORD PTR [rbp+0x68],rdx
   1409295eb:	49 8b 91 18 01 00 00 	mov    rdx,QWORD PTR [r9+0x118]
   1409295f2:	48 39 d1             	cmp    rcx,rdx
   1409295f5:	0f 87 be 00 00 00    	ja     0x1409296b9
   1409295fb:	e9 3e 01 00 00       	jmp    0x14092973e
   140929600:	49 39 df             	cmp    r15,rbx
   140929603:	0f 85 70 01 00 00    	jne    0x140929779
   140929609:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
   140929610:	49 39 f4             	cmp    r12,rsi
   140929613:	0f 84 67 02 00 00    	je     0x140929880
   140929619:	41 8b 0c 24          	mov    ecx,DWORD PTR [r12]
   14092961d:	49 83 c4 04          	add    r12,0x4
   140929621:	49 8b 85 00 01 00 00 	mov    rax,QWORD PTR [r13+0x100]
   140929628:	48 8d 0c 89          	lea    rcx,[rcx+rcx*4]
   14092962c:	48 c1 e1 05          	shl    rcx,0x5
   140929630:	48 8b 5c 08 58       	mov    rbx,QWORD PTR [rax+rcx*1+0x58]
   140929635:	48 85 db             	test   rbx,rbx
   140929638:	74 d6                	je     0x140929610
   14092963a:	48 01 c8             	add    rax,rcx
   14092963d:	8b 88 94 00 00 00    	mov    ecx,DWORD PTR [rax+0x94]
   140929643:	49 8b 95 a8 01 00 00 	mov    rdx,QWORD PTR [r13+0x1a8]
   14092964a:	4c 8d 0c c9          	lea    r9,[rcx+rcx*8]
   14092964e:	4a 8d 0c ca          	lea    rcx,[rdx+r9*8]
   140929652:	4c 8b 55 50          	mov    r10,QWORD PTR [rbp+0x50]
   140929656:	4d 8b 82 10 01 00 00 	mov    r8,QWORD PTR [r10+0x110]
   14092965d:	4a 8b 54 ca 40       	mov    rdx,QWORD PTR [rdx+r9*8+0x40]
   140929662:	4c 39 c2             	cmp    rdx,r8
   140929665:	76 7e                	jbe    0x1409296e5
   140929667:	4c 8b 49 38          	mov    r9,QWORD PTR [rcx+0x38]
   14092966b:	4f 8b 04 c1          	mov    r8,QWORD PTR [r9+r8*8]
   14092966f:	4d 85 c0             	test   r8,r8
   140929672:	74 71                	je     0x1409296e5
   140929674:	4c 8b 49 18          	mov    r9,QWORD PTR [rcx+0x18]
   140929678:	49 f7 d0             	not    r8
   14092967b:	4f 8d 04 40          	lea    r8,[r8+r8*2]
   14092967f:	49 c1 e0 04          	shl    r8,0x4
   140929683:	4f 8b 44 01 10       	mov    r8,QWORD PTR [r9+r8*1+0x10]
   140929688:	4c 89 45 68          	mov    QWORD PTR [rbp+0x68],r8
   14092968c:	4d 8b 82 18 01 00 00 	mov    r8,QWORD PTR [r10+0x118]
   140929693:	4c 39 80 88 00 00 00 	cmp    QWORD PTR [rax+0x88],r8
   14092969a:	77 61                	ja     0x1409296fd
   14092969c:	e9 bc 00 00 00       	jmp    0x14092975d
   1409296a1:	48 c7 45 68 00 00 00 	mov    QWORD PTR [rbp+0x68],0x0
   1409296a8:	00 
   1409296a9:	49 8b 91 18 01 00 00 	mov    rdx,QWORD PTR [r9+0x118]
   1409296b0:	48 39 d1             	cmp    rcx,rdx
   1409296b3:	0f 86 85 00 00 00    	jbe    0x14092973e
   1409296b9:	48 8b 48 38          	mov    rcx,QWORD PTR [rax+0x38]
   1409296bd:	48 8b 0c d1          	mov    rcx,QWORD PTR [rcx+rdx*8]
   1409296c1:	48 85 c9             	test   rcx,rcx
   1409296c4:	74 78                	je     0x14092973e
   1409296c6:	48 8b 50 18          	mov    rdx,QWORD PTR [rax+0x18]
   1409296ca:	48 f7 d1             	not    rcx
   1409296cd:	48 8d 0c 49          	lea    rcx,[rcx+rcx*2]
   1409296d1:	48 c1 e1 04          	shl    rcx,0x4
   1409296d5:	48 8b 4c 0a 10       	mov    rcx,QWORD PTR [rdx+rcx*1+0x10]
   1409296da:	48 89 4d 58          	mov    QWORD PTR [rbp+0x58],rcx
   1409296de:	b1 01                	mov    cl,0x1
   1409296e0:	89 4d 70             	mov    DWORD PTR [rbp+0x70],ecx
   1409296e3:	eb 60                	jmp    0x140929745
   1409296e5:	48 c7 45 68 00 00 00 	mov    QWORD PTR [rbp+0x68],0x0
   1409296ec:	00 
   1409296ed:	4d 8b 82 18 01 00 00 	mov    r8,QWORD PTR [r10+0x118]
   1409296f4:	4c 39 80 88 00 00 00 	cmp    QWORD PTR [rax+0x88],r8
   1409296fb:	76 60                	jbe    0x14092975d
   1409296fd:	4c 8b 88 80 00 00 00 	mov    r9,QWORD PTR [rax+0x80]
   140929704:	4b 83 3c c1 00       	cmp    QWORD PTR [r9+r8*8],0x0
   140929709:	74 52                	je     0x14092975d
   14092970b:	41 b1 01             	mov    r9b,0x1
   14092970e:	44 89 4d 70          	mov    DWORD PTR [rbp+0x70],r9d
   140929712:	4c 39 c2             	cmp    rdx,r8
   140929715:	76 4f                	jbe    0x140929766
   140929717:	48 8b 51 38          	mov    rdx,QWORD PTR [rcx+0x38]
   14092971b:	4a 8b 14 c2          	mov    rdx,QWORD PTR [rdx+r8*8]
   14092971f:	48 85 d2             	test   rdx,rdx
   140929722:	74 42                	je     0x140929766
   140929724:	48 8b 49 18          	mov    rcx,QWORD PTR [rcx+0x18]
   140929728:	48 f7 d2             	not    rdx
   14092972b:	48 8d 14 52          	lea    rdx,[rdx+rdx*2]
   14092972f:	48 c1 e2 04          	shl    rdx,0x4
   140929733:	48 8b 4c 11 10       	mov    rcx,QWORD PTR [rcx+rdx*1+0x10]
   140929738:	48 89 4d 58          	mov    QWORD PTR [rbp+0x58],rcx
   14092973c:	eb 30                	jmp    0x14092976e
   14092973e:	c7 45 70 00 00 00 00 	mov    DWORD PTR [rbp+0x70],0x0
   140929745:	48 8b 40 08          	mov    rax,QWORD PTR [rax+0x8]
   140929749:	48 89 45 40          	mov    QWORD PTR [rbp+0x40],rax
   14092974d:	45 31 ff             	xor    r15d,r15d
   140929750:	48 8b 45 40          	mov    rax,QWORD PTR [rbp+0x40]
   140929754:	4a 8d 04 f8          	lea    rax,[rax+r15*8]
   140929758:	45 89 fe             	mov    r14d,r15d
   14092975b:	eb 30                	jmp    0x14092978d
   14092975d:	c7 45 70 00 00 00 00 	mov    DWORD PTR [rbp+0x70],0x0
   140929764:	eb 08                	jmp    0x14092976e
   140929766:	48 c7 45 58 00 00 00 	mov    QWORD PTR [rbp+0x58],0x0
   14092976d:	00 
   14092976e:	48 8b 40 50          	mov    rax,QWORD PTR [rax+0x50]
   140929772:	48 89 45 48          	mov    QWORD PTR [rbp+0x48],rax
   140929776:	45 31 ff             	xor    r15d,r15d
   140929779:	4c 89 f9             	mov    rcx,r15
   14092977c:	48 c1 e1 04          	shl    rcx,0x4
   140929780:	48 8b 55 48          	mov    rdx,QWORD PTR [rbp+0x48]
   140929784:	48 8d 04 0a          	lea    rax,[rdx+rcx*1]
   140929788:	44 8b 74 0a 08       	mov    r14d,DWORD PTR [rdx+rcx*1+0x8]
   14092978d:	48 8b 4d 58          	mov    rcx,QWORD PTR [rbp+0x58]
   140929791:	4a 8d 3c f1          	lea    rdi,[rcx+r14*8]
   140929795:	49 c1 e6 04          	shl    r14,0x4
   140929799:	4c 03 75 68          	add    r14,QWORD PTR [rbp+0x68]
   14092979d:	f6 45 70 01          	test   BYTE PTR [rbp+0x70],0x1
   1409297a1:	b9 00 00 00 00       	mov    ecx,0x0
   1409297a6:	48 0f 44 f9          	cmove  rdi,rcx
   1409297aa:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409297ad:	48 89 45 20          	mov    QWORD PTR [rbp+0x20],rax
   1409297b1:	49 ff c7             	inc    r15
   1409297b4:	48 83 7d f8 00       	cmp    QWORD PTR [rbp-0x8],0x0
   1409297b9:	74 45                	je     0x140929800
   1409297bb:	48 8d 4d 00          	lea    rcx,[rbp+0x0]
   1409297bf:	4c 89 f2             	mov    rdx,r14
   1409297c2:	e8 a9 75 c8 ff       	call   0x1405b0d70
   1409297c7:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
   1409297cb:	4c 8b 45 e8          	mov    r8,QWORD PTR [rbp-0x18]
   1409297cf:	4c 89 45 38          	mov    QWORD PTR [rbp+0x38],r8
   1409297d3:	49 21 c0             	and    r8,rax
   1409297d6:	48 c1 e8 39          	shr    rax,0x39
   1409297da:	66 0f 6e c0          	movd   xmm0,eax
   1409297de:	66 0f 60 c0          	punpcklbw xmm0,xmm0
   1409297e2:	f2 0f 70 c0 00       	pshuflw xmm0,xmm0,0x0
   1409297e7:	66 0f 70 c0 00       	pshufd xmm0,xmm0,0x0
   1409297ec:	31 c9                	xor    ecx,ecx
   1409297ee:	e9 ed fc ff ff       	jmp    0x1409294e0
   1409297f3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   1409297fa:	84 00 00 00 00 00 
   140929800:	48 85 ff             	test   rdi,rdi
   140929803:	74 0e                	je     0x140929813
   140929805:	f3 0f 10 47 04       	movss  xmm0,DWORD PTR [rdi+0x4]
   14092980a:	0f 2e 07             	ucomiss xmm0,DWORD PTR [rdi]
   14092980d:	0f 87 4d fd ff ff    	ja     0x140929560
   140929813:	48 8b 7d 28          	mov    rdi,QWORD PTR [rbp+0x28]
   140929817:	48 8b 07             	mov    rax,QWORD PTR [rdi]
   14092981a:	4c 8b 77 10          	mov    r14,QWORD PTR [rdi+0x10]
   14092981e:	4c 29 f0             	sub    rax,r14
   140929821:	48 83 f8 1f          	cmp    rax,0x1f
   140929825:	76 37                	jbe    0x14092985e
   140929827:	48 8b 47 08          	mov    rax,QWORD PTR [rdi+0x8]
   14092982b:	48 8d 0d fe 34 9b 00 	lea    rcx,[rip+0x9b34fe]        # 0x1412dcd30
   140929832:	4a 89 0c 30          	mov    QWORD PTR [rax+r14*1],rcx
   140929836:	48 8d 0d f3 b4 c3 00 	lea    rcx,[rip+0xc3b4f3]        # 0x141564d30
   14092983d:	4a 89 4c 30 08       	mov    QWORD PTR [rax+r14*1+0x8],rcx
   140929842:	48 8b 4d 20          	mov    rcx,QWORD PTR [rbp+0x20]
   140929846:	4a 89 4c 30 10       	mov    QWORD PTR [rax+r14*1+0x10],rcx
   14092984b:	42 c6 44 30 18 00    	mov    BYTE PTR [rax+r14*1+0x18],0x0
   140929851:	49 83 c6 20          	add    r14,0x20
   140929855:	4c 89 77 10          	mov    QWORD PTR [rdi+0x10],r14
   140929859:	e9 02 fd ff ff       	jmp    0x140929560
   14092985e:	48 c7 44 24 20 01 00 	mov    QWORD PTR [rsp+0x20],0x1
   140929865:	00 00 
   140929867:	41 b8 20 00 00 00    	mov    r8d,0x20
   14092986d:	41 b9 01 00 00 00    	mov    r9d,0x1
   140929873:	48 89 f9             	mov    rcx,rdi
   140929876:	4c 89 f2             	mov    rdx,r14
   140929879:	e8 e2 01 f8 01       	call   0x1428a9a60
   14092987e:	eb a7                	jmp    0x140929827
   140929880:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
   140929884:	48 85 d2             	test   rdx,rdx
   140929887:	74 27                	je     0x1409298b0
   140929889:	48 8d 04 d5 17 00 00 	lea    rax,[rdx*8+0x17]
   140929890:	00 
   140929891:	48 83 e0 f0          	and    rax,0xfffffffffffffff0
   140929895:	48 01 c2             	add    rdx,rax
   140929898:	48 83 c2 11          	add    rdx,0x11
   14092989c:	74 12                	je     0x1409298b0
   14092989e:	48 8b 4d e0          	mov    rcx,QWORD PTR [rbp-0x20]
   1409298a2:	48 29 c1             	sub    rcx,rax
   1409298a5:	41 b8 10 00 00 00    	mov    r8d,0x10
   1409298ab:	e8 70 a3 ce ff       	call   0x140613c20
   1409298b0:	80 7d 77 00          	cmp    BYTE PTR [rbp+0x77],0x0
   1409298b4:	74 3a                	je     0x1409298f0
   1409298b6:	e8 75 e8 2e 00       	call   0x140c18130
   1409298bb:	48 89 c1             	mov    rcx,rax
   1409298be:	48 8b 00             	mov    rax,QWORD PTR [rax]
   1409298c1:	48 83 f8 01          	cmp    rax,0x1
   1409298c5:	75 5e                	jne    0x140929925
   1409298c7:	48 83 c1 08          	add    rcx,0x8
   1409298cb:	48 83 39 00          	cmp    QWORD PTR [rcx],0x0
   1409298cf:	75 74                	jne    0x140929945
   1409298d1:	48 c7 01 ff ff ff ff 	mov    QWORD PTR [rcx],0xffffffffffffffff
   1409298d8:	48 89 4d 60          	mov    QWORD PTR [rbp+0x60],rcx
   1409298dc:	48 83 c1 08          	add    rcx,0x8
   1409298e0:	48 8b 55 30          	mov    rdx,QWORD PTR [rbp+0x30]
   1409298e4:	e8 07 d8 2e 00       	call   0x140c170f0
   1409298e9:	48 8b 45 60          	mov    rax,QWORD PTR [rbp+0x60]
   1409298ed:	48 ff 00             	inc    QWORD PTR [rax]
   1409298f0:	0f 28 b5 80 00 00 00 	movaps xmm6,XMMWORD PTR [rbp+0x80]
   1409298f7:	48 81 c4 18 01 00 00 	add    rsp,0x118
   1409298fe:	5b                   	pop    rbx
   1409298ff:	5f                   	pop    rdi
   140929900:	5e                   	pop    rsi
   140929901:	41 5c                	pop    r12
   140929903:	41 5d                	pop    r13
   140929905:	41 5e                	pop    r14
   140929907:	41 5f                	pop    r15
   140929909:	5d                   	pop    rbp
   14092990a:	c3                   	ret
   14092990b:	48 85 c0             	test   rax,rax
   14092990e:	75 29                	jne    0x140929939
   140929910:	31 d2                	xor    edx,edx
   140929912:	e8 b9 ee fc 01       	call   0x1428f87d0
   140929917:	48 89 c1             	mov    rcx,rax
   14092991a:	48 85 c0             	test   rax,rax
   14092991d:	0f 85 65 fa ff ff    	jne    0x140929388
   140929923:	eb 14                	jmp    0x140929939
   140929925:	48 85 c0             	test   rax,rax
   140929928:	75 0f                	jne    0x140929939
   14092992a:	31 d2                	xor    edx,edx
   14092992c:	e8 9f ee fc 01       	call   0x1428f87d0
   140929931:	48 89 c1             	mov    rcx,rax
   140929934:	48 85 c0             	test   rax,rax
   140929937:	75 92                	jne    0x1409298cb
   140929939:	48 8d 0d 60 11 31 02 	lea    rcx,[rip+0x2311160]        # 0x142c3aaa0
   140929940:	e8 8b e7 fa 01       	call   0x1428d80d0
   140929945:	48 8d 0d 24 12 31 02 	lea    rcx,[rip+0x2311224]        # 0x142c3ab70
   14092994c:	e8 ff f8 fa 01       	call   0x1428d9250
   140929951:	48 8d 0d 48 11 31 02 	lea    rcx,[rip+0x2311148]        # 0x142c3aaa0
   140929958:	e8 73 e7 fa 01       	call   0x1428d80d0
   14092995d:	0f 0b                	ud2

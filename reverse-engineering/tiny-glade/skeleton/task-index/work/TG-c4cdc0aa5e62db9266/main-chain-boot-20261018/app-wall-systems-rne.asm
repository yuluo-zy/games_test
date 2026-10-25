0x140493120: push rbp
0x140493121: push rsi
0x140493122: push rdi
0x140493123: sub rsp, 0x1f0
0x14049312a: lea rbp, [rsp + 0x80]
0x140493132: mov qword ptr [rbp + 0x168], 0xfffffffffffffffe
0x14049313d: mov rsi, rcx
0x140493140: lea r8, [rip + 0x254bf41]
0x140493147: lea rdi, [rbp + 0x10]
0x14049314b: mov rcx, rdi
0x14049314e: mov dl, 0x11
0x140493150: call 0x140112d30
0x140493155: mov rcx, rsi
0x140493158: mov rdx, rdi
0x14049315b: call 0x1403276f0
0x140493160: lea r8, [rip + 0x254bf39]
0x140493167: mov rcx, rdi
0x14049316a: mov dl, 0x2a
0x14049316c: call 0x140322d40
0x140493171: mov rcx, rsi
0x140493174: mov rdx, rdi
0x140493177: call 0x1403276f0
0x14049317c: mov rcx, rdi
0x14049317f: call 0x14054e960
0x140493184: lea rcx, [rbp - 0x60]
0x140493188: mov rdx, rdi
0x14049318b: call 0x14050d290
0x140493190: lea rcx, [rbp + 0xf0]
0x140493197: call 0x140335b30
0x14049319c: movups xmm0, xmmword ptr [rbp]
0x1404931a0: movaps xmmword ptr [rbp + 0x70], xmm0
0x1404931a4: movups xmm0, xmmword ptr [rbp - 0x10]
0x1404931a8: movaps xmmword ptr [rbp + 0x60], xmm0
0x1404931ac: movups xmm0, xmmword ptr [rbp - 0x20]
0x1404931b0: movaps xmmword ptr [rbp + 0x50], xmm0
0x1404931b4: movups xmm0, xmmword ptr [rbp - 0x60]
0x1404931b8: movups xmm1, xmmword ptr [rbp - 0x50]
0x1404931bc: movups xmm2, xmmword ptr [rbp - 0x40]
0x1404931c0: movups xmm3, xmmword ptr [rbp - 0x30]
0x1404931c4: movaps xmmword ptr [rbp + 0x40], xmm3
0x1404931c8: movaps xmmword ptr [rbp + 0x30], xmm2
0x1404931cc: movaps xmmword ptr [rbp + 0x20], xmm1
0x1404931d0: movaps xmmword ptr [rbp + 0x10], xmm0
0x1404931d4: movups xmm0, xmmword ptr [rbp + 0xf0]
0x1404931db: movups xmm1, xmmword ptr [rbp + 0x100]
0x1404931e2: movups xmm2, xmmword ptr [rbp + 0x110]
0x1404931e9: movups xmm3, xmmword ptr [rbp + 0x120]
0x1404931f0: movaps xmmword ptr [rbp + 0x80], xmm0
0x1404931f7: movaps xmmword ptr [rbp + 0x90], xmm1
0x1404931fe: movaps xmmword ptr [rbp + 0xa0], xmm2
0x140493205: movaps xmmword ptr [rbp + 0xb0], xmm3
0x14049320c: movups xmm0, xmmword ptr [rbp + 0x130]
0x140493213: movaps xmmword ptr [rbp + 0xc0], xmm0
0x14049321a: movups xmm0, xmmword ptr [rbp + 0x140]
0x140493221: movaps xmmword ptr [rbp + 0xd0], xmm0
0x140493228: movups xmm0, xmmword ptr [rbp + 0x150]
0x14049322f: movaps xmmword ptr [rbp + 0xe0], xmm0
0x140493236: lea rcx, [rbp + 0xf0]
0x14049323d: lea rdx, [rbp + 0x10]
0x140493241: call 0x1400d5080
0x140493246: mov byte ptr [rbp - 0x60], 0x13
0x14049324a: mov rcx, qword ptr [rip + 0x319911f]
0x140493251: lea r8, [rip + 0x2497d40]
0x140493258: lea rdx, [rbp - 0x60]
0x14049325c: call 0x140364d20
0x140493261: mov r8, rdx
0x140493264: lea rcx, [rbp + 0xf0]
0x14049326b: mov rdx, rax
0x14049326e: call 0x1400d68c0
0x140493273: movups xmm0, xmmword ptr [rbp + 0x150]
0x14049327a: movaps xmmword ptr [rbp + 0x70], xmm0
0x14049327e: movups xmm0, xmmword ptr [rbp + 0x140]
0x140493285: movaps xmmword ptr [rbp + 0x60], xmm0
0x140493289: movups xmm0, xmmword ptr [rbp + 0x130]
0x140493290: movaps xmmword ptr [rbp + 0x50], xmm0
0x140493294: movups xmm0, xmmword ptr [rbp + 0xf0]
0x14049329b: movups xmm1, xmmword ptr [rbp + 0x100]
0x1404932a2: movups xmm2, xmmword ptr [rbp + 0x110]
0x1404932a9: movups xmm3, xmmword ptr [rbp + 0x120]
0x1404932b0: movaps xmmword ptr [rbp + 0x40], xmm3
0x1404932b4: movaps xmmword ptr [rbp + 0x30], xmm2
0x1404932b8: movaps xmmword ptr [rbp + 0x20], xmm1
0x1404932bc: movaps xmmword ptr [rbp + 0x10], xmm0
0x1404932c0: lea rdx, [rbp + 0x10]
0x1404932c4: mov rcx, rsi
0x1404932c7: call 0x140326b40
0x1404932cc: mov qword ptr [rbp + 0x10], 0
0x1404932d4: mov qword ptr [rbp + 0x18], 8
0x1404932dc: mov qword ptr [rbp + 0x20], 0
0x1404932e4: lea rdx, [rbp + 0x10]
0x1404932e8: mov rcx, rsi
0x1404932eb: call 0x14011bfe0
0x1404932f0: mov rsi, rax
0x1404932f3: mov rcx, rax
0x1404932f6: call 0x1403270d0
0x1404932fb: lea rcx, [rbp + 0x10]
0x1404932ff: call 0x14030fb40
0x140493304: mov byte ptr [rbp - 0x60], 0x18
0x140493308: mov rcx, qword ptr [rip + 0x3199061]
0x14049330f: lea r8, [rip + 0x2497c82]
0x140493316: lea rdx, [rbp - 0x60]
0x14049331a: call 0x140364d20
0x14049331f: mov r8, rdx
0x140493322: lea rcx, [rbp + 0x10]
0x140493326: mov rdx, rax
0x140493329: call 0x1400d68c0
0x14049332e: movups xmm0, xmmword ptr [rbp + 0x70]
0x140493332: movaps xmmword ptr [rbp + 0x150], xmm0
0x140493339: movups xmm0, xmmword ptr [rbp + 0x60]
0x14049333d: movaps xmmword ptr [rbp + 0x140], xmm0
0x140493344: movups xmm0, xmmword ptr [rbp + 0x50]
0x140493348: movaps xmmword ptr [rbp + 0x130], xmm0
0x14049334f: movups xmm0, xmmword ptr [rbp + 0x10]
0x140493353: movups xmm1, xmmword ptr [rbp + 0x20]
0x140493357: movups xmm2, xmmword ptr [rbp + 0x30]
0x14049335b: movups xmm3, xmmword ptr [rbp + 0x40]
0x14049335f: movaps xmmword ptr [rbp + 0x120], xmm3
0x140493366: movaps xmmword ptr [rbp + 0x110], xmm2
0x14049336d: movaps xmmword ptr [rbp + 0x100], xmm1
0x140493374: movaps xmmword ptr [rbp + 0xf0], xmm0
0x14049337b: lea rdx, [rbp + 0xf0]
0x140493382: mov rcx, rsi
0x140493385: call 0x1403276f0
0x14049338a: lea rcx, [rbp + 0x10]
0x14049338e: call 0x14033def0
0x140493393: mov byte ptr [rbp - 0x60], 0x1c
0x140493397: mov rcx, qword ptr [rip + 0x3198fd2]
0x14049339e: lea r8, [rip + 0x2497bf3]
0x1404933a5: lea rdx, [rbp - 0x60]
0x1404933a9: call 0x140364d20
0x1404933ae: mov r8, rdx
0x1404933b1: lea rcx, [rbp + 0x10]
0x1404933b5: mov rdx, rax
0x1404933b8: call 0x1400d68c0
0x1404933bd: movups xmm0, xmmword ptr [rbp + 0x70]
0x1404933c1: movaps xmmword ptr [rbp + 0x150], xmm0
0x1404933c8: movups xmm0, xmmword ptr [rbp + 0x60]
0x1404933cc: movaps xmmword ptr [rbp + 0x140], xmm0
0x1404933d3: movups xmm0, xmmword ptr [rbp + 0x50]
0x1404933d7: movaps xmmword ptr [rbp + 0x130], xmm0
0x1404933de: movups xmm0, xmmword ptr [rbp + 0x10]
0x1404933e2: movups xmm1, xmmword ptr [rbp + 0x20]
0x1404933e6: movups xmm2, xmmword ptr [rbp + 0x30]
0x1404933ea: movups xmm3, xmmword ptr [rbp + 0x40]
0x1404933ee: movaps xmmword ptr [rbp + 0x120], xmm3
0x1404933f5: movaps xmmword ptr [rbp + 0x110], xmm2
0x1404933fc: movaps xmmword ptr [rbp + 0x100], xmm1
0x140493403: movaps xmmword ptr [rbp + 0xf0], xmm0
0x14049340a: lea rdx, [rbp + 0xf0]
0x140493411: mov rcx, rsi
0x140493414: call 0x1403276f0
0x140493419: lea r8, [rip + 0x254bc98]
0x140493420: mov rcx, rdi
0x140493423: mov dl, 0x24
0x140493425: call 0x1403104a0
0x14049342a: mov rcx, rsi
0x14049342d: mov rdx, rdi
0x140493430: call 0x1403276f0
0x140493435: lea rcx, [rbp + 0x10]
0x140493439: call 0x1400cbd40
0x14049343e: mov byte ptr [rbp - 0x60], 0x22
0x140493442: mov rcx, qword ptr [rip + 0x3198f27]
0x140493449: lea r8, [rip + 0x2497b48]
0x140493450: lea rdx, [rbp - 0x60]
0x140493454: call 0x140364d20
0x140493459: mov r8, rdx
0x14049345c: lea rcx, [rbp + 0x10]
0x140493460: mov rdx, rax
0x140493463: call 0x1400d68c0
0x140493468: movups xmm0, xmmword ptr [rbp + 0x70]
0x14049346c: movaps xmmword ptr [rbp + 0x150], xmm0
0x140493473: movups xmm0, xmmword ptr [rbp + 0x60]
0x140493477: movaps xmmword ptr [rbp + 0x140], xmm0
0x14049347e: movups xmm0, xmmword ptr [rbp + 0x50]
0x140493482: movaps xmmword ptr [rbp + 0x130], xmm0
0x140493489: movups xmm0, xmmword ptr [rbp + 0x10]
0x14049348d: movups xmm1, xmmword ptr [rbp + 0x20]
0x140493491: movups xmm2, xmmword ptr [rbp + 0x30]
0x140493495: movups xmm3, xmmword ptr [rbp + 0x40]
0x140493499: movaps xmmword ptr [rbp + 0x120], xmm3
0x1404934a0: movaps xmmword ptr [rbp + 0x110], xmm2
0x1404934a7: movaps xmmword ptr [rbp + 0x100], xmm1
0x1404934ae: movaps xmmword ptr [rbp + 0xf0], xmm0
0x1404934b5: lea rdx, [rbp + 0xf0]
0x1404934bc: mov rcx, rsi
0x1404934bf: call 0x1403276f0
0x1404934c4: lea r8, [rip + 0x254bc05]
0x1404934cb: mov rcx, rdi
0x1404934ce: mov dl, 0x2c
0x1404934d0: call 0x1400f50e0
0x1404934d5: mov rcx, rsi
0x1404934d8: mov rdx, rdi
0x1404934db: call 0x1403276f0
0x1404934e0: lea rcx, [rbp + 0x10]
0x1404934e4: call 0x140272c20
0x1404934e9: mov byte ptr [rbp - 0x60], 0x20
0x1404934ed: mov rcx, qword ptr [rip + 0x3198e7c]
0x1404934f4: lea r8, [rip + 0x2497a9d]
0x1404934fb: lea rdx, [rbp - 0x60]
0x1404934ff: call 0x140364d20
0x140493504: mov r8, rdx
0x140493507: lea rcx, [rbp + 0x10]
0x14049350b: mov rdx, rax
0x14049350e: call 0x1400d68c0
0x140493513: movups xmm0, xmmword ptr [rbp + 0x70]
0x140493517: movaps xmmword ptr [rbp + 0x150], xmm0
0x14049351e: movups xmm0, xmmword ptr [rbp + 0x60]
0x140493522: movaps xmmword ptr [rbp + 0x140], xmm0
0x140493529: movups xmm0, xmmword ptr [rbp + 0x50]
0x14049352d: movaps xmmword ptr [rbp + 0x130], xmm0
0x140493534: movups xmm0, xmmword ptr [rbp + 0x10]
0x140493538: movups xmm1, xmmword ptr [rbp + 0x20]
0x14049353c: movups xmm2, xmmword ptr [rbp + 0x30]
0x140493540: movups xmm3, xmmword ptr [rbp + 0x40]
0x140493544: movaps xmmword ptr [rbp + 0x120], xmm3
0x14049354b: movaps xmmword ptr [rbp + 0x110], xmm2
0x140493552: movaps xmmword ptr [rbp + 0x100], xmm1
0x140493559: movaps xmmword ptr [rbp + 0xf0], xmm0
0x140493560: lea rdx, [rbp + 0xf0]
0x140493567: mov rcx, rsi
0x14049356a: call 0x1403276f0
0x14049356f: mov rcx, rsi
0x140493572: mov dl, 1
0x140493574: call 0x140327c20
0x140493579: mov rcx, rsi
0x14049357c: add rsp, 0x1f0
0x140493583: pop rdi
0x140493584: pop rsi
0x140493585: pop rbp
0x140493586: jmp 0x140327ae0
0x14049358b: nop dword ptr [rax + rax]
0x140493590: mov qword ptr [rsp + 0x10], rdx
0x140493595: push rbp
0x140493596: push rsi
0x140493597: push rdi
0x140493598: sub rsp, 0x20
0x14049359c: lea rbp, [rdx + 0x80]
0x1404935a3: lea rcx, [rbp - 0x60]
0x1404935a7: call 0x140029880
0x1404935ac: nop 
0x1404935ad: add rsp, 0x20
0x1404935b1: pop rdi
0x1404935b2: pop rsi
0x1404935b3: pop rbp
0x1404935b4: ret 
0x1404935b5: nop word ptr cs:[rax + rax]
0x1404935c0: mov qword ptr [rsp + 0x10], rdx
0x1404935c5: push rbp
0x1404935c6: push rsi
0x1404935c7: push rdi
0x1404935c8: sub rsp, 0x20
0x1404935cc: lea rbp, [rdx + 0x80]
0x1404935d3: lea rcx, [rbp + 0xf0]
0x1404935da: call 0x140029880
0x1404935df: nop 
0x1404935e0: add rsp, 0x20
0x1404935e4: pop rdi
0x1404935e5: pop rsi
0x1404935e6: pop rbp
0x1404935e7: ret 
0x1404935e8: nop dword ptr [rax + rax]
0x1404935f0: mov qword ptr [rsp + 0x10], rdx
0x1404935f5: push rbp
0x1404935f6: push rsi
0x1404935f7: push rdi
0x1404935f8: sub rsp, 0x20
0x1404935fc: lea rbp, [rdx + 0x80]
0x140493603: lea rcx, [rbp + 0x10]
0x140493607: call 0x140029880
0x14049360c: nop 
0x14049360d: add rsp, 0x20
0x140493611: pop rdi
0x140493612: pop rsi
0x140493613: pop rbp
0x140493614: ret 
0x140493615: nop word ptr cs:[rax + rax]
0x140493620: mov qword ptr [rsp + 0x10], rdx
0x140493625: push rbp
0x140493626: push rsi
0x140493627: push rdi
0x140493628: sub rsp, 0x20
0x14049362c: lea rbp, [rdx + 0x80]
0x140493633: lea rcx, [rbp + 0x10]
0x140493637: call 0x140029880
0x14049363c: nop 
0x14049363d: add rsp, 0x20
0x140493641: pop rdi
0x140493642: pop rsi
0x140493643: pop rbp
0x140493644: ret 
0x140493645: nop word ptr cs:[rax + rax]
0x140493650: mov qword ptr [rsp + 0x10], rdx
0x140493655: push rbp
0x140493656: push rsi
0x140493657: push rdi
0x140493658: sub rsp, 0x20
0x14049365c: lea rbp, [rdx + 0x80]
0x140493663: lea rcx, [rbp + 0x10]
0x140493667: call 0x140029880
0x14049366c: nop 
0x14049366d: add rsp, 0x20
0x140493671: pop rdi
0x140493672: pop rsi
0x140493673: pop rbp
0x140493674: ret 
0x140493675: nop word ptr cs:[rax + rax]
0x140493680: mov qword ptr [rsp + 0x10], rdx
0x140493685: push rbp
0x140493686: push rsi
0x140493687: push rdi
0x140493688: sub rsp, 0x20
0x14049368c: lea rbp, [rdx + 0x80]
0x140493693: lea rcx, [rbp + 0x10]
0x140493697: call 0x140029880
0x14049369c: nop 
0x14049369d: add rsp, 0x20
0x1404936a1: pop rdi
0x1404936a2: pop rsi
0x1404936a3: pop rbp
0x1404936a4: ret 

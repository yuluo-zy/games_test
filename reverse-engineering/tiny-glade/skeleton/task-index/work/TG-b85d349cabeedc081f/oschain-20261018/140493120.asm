140493120: push rbp
140493121: push rsi
140493122: push rdi
140493123: sub rsp, 0x1f0
14049312a: lea rbp, [rsp + 0x80]
140493132: mov qword ptr [rbp + 0x168], 0xfffffffffffffffe
14049313d: mov rsi, rcx
140493140: lea r8, [rip + 0x254bf41]
140493147: lea rdi, [rbp + 0x10]
14049314b: mov rcx, rdi
14049314e: mov dl, 0x11
140493150: call 0x140112d30
140493155: mov rcx, rsi
140493158: mov rdx, rdi
14049315b: call 0x1403276f0
140493160: lea r8, [rip + 0x254bf39]
140493167: mov rcx, rdi
14049316a: mov dl, 0x2a
14049316c: call 0x140322d40
140493171: mov rcx, rsi
140493174: mov rdx, rdi
140493177: call 0x1403276f0
14049317c: mov rcx, rdi
14049317f: call 0x14054e960
140493184: lea rcx, [rbp - 0x60]
140493188: mov rdx, rdi
14049318b: call 0x14050d290
140493190: lea rcx, [rbp + 0xf0]
140493197: call 0x140335b30
14049319c: movups xmm0, xmmword ptr [rbp]
1404931a0: movaps xmmword ptr [rbp + 0x70], xmm0
1404931a4: movups xmm0, xmmword ptr [rbp - 0x10]
1404931a8: movaps xmmword ptr [rbp + 0x60], xmm0
1404931ac: movups xmm0, xmmword ptr [rbp - 0x20]
1404931b0: movaps xmmword ptr [rbp + 0x50], xmm0
1404931b4: movups xmm0, xmmword ptr [rbp - 0x60]
1404931b8: movups xmm1, xmmword ptr [rbp - 0x50]
1404931bc: movups xmm2, xmmword ptr [rbp - 0x40]
1404931c0: movups xmm3, xmmword ptr [rbp - 0x30]
1404931c4: movaps xmmword ptr [rbp + 0x40], xmm3
1404931c8: movaps xmmword ptr [rbp + 0x30], xmm2
1404931cc: movaps xmmword ptr [rbp + 0x20], xmm1
1404931d0: movaps xmmword ptr [rbp + 0x10], xmm0
1404931d4: movups xmm0, xmmword ptr [rbp + 0xf0]
1404931db: movups xmm1, xmmword ptr [rbp + 0x100]
1404931e2: movups xmm2, xmmword ptr [rbp + 0x110]
1404931e9: movups xmm3, xmmword ptr [rbp + 0x120]
1404931f0: movaps xmmword ptr [rbp + 0x80], xmm0
1404931f7: movaps xmmword ptr [rbp + 0x90], xmm1
1404931fe: movaps xmmword ptr [rbp + 0xa0], xmm2
140493205: movaps xmmword ptr [rbp + 0xb0], xmm3
14049320c: movups xmm0, xmmword ptr [rbp + 0x130]
140493213: movaps xmmword ptr [rbp + 0xc0], xmm0
14049321a: movups xmm0, xmmword ptr [rbp + 0x140]
140493221: movaps xmmword ptr [rbp + 0xd0], xmm0
140493228: movups xmm0, xmmword ptr [rbp + 0x150]
14049322f: movaps xmmword ptr [rbp + 0xe0], xmm0
140493236: lea rcx, [rbp + 0xf0]
14049323d: lea rdx, [rbp + 0x10]
140493241: call 0x1400d5080
140493246: mov byte ptr [rbp - 0x60], 0x13
14049324a: mov rcx, qword ptr [rip + 0x319911f]
140493251: lea r8, [rip + 0x2497d40]
140493258: lea rdx, [rbp - 0x60]
14049325c: call 0x140364d20
140493261: mov r8, rdx
140493264: lea rcx, [rbp + 0xf0]
14049326b: mov rdx, rax
14049326e: call 0x1400d68c0
140493273: movups xmm0, xmmword ptr [rbp + 0x150]
14049327a: movaps xmmword ptr [rbp + 0x70], xmm0
14049327e: movups xmm0, xmmword ptr [rbp + 0x140]
140493285: movaps xmmword ptr [rbp + 0x60], xmm0
140493289: movups xmm0, xmmword ptr [rbp + 0x130]
140493290: movaps xmmword ptr [rbp + 0x50], xmm0
140493294: movups xmm0, xmmword ptr [rbp + 0xf0]
14049329b: movups xmm1, xmmword ptr [rbp + 0x100]
1404932a2: movups xmm2, xmmword ptr [rbp + 0x110]
1404932a9: movups xmm3, xmmword ptr [rbp + 0x120]
1404932b0: movaps xmmword ptr [rbp + 0x40], xmm3
1404932b4: movaps xmmword ptr [rbp + 0x30], xmm2
1404932b8: movaps xmmword ptr [rbp + 0x20], xmm1
1404932bc: movaps xmmword ptr [rbp + 0x10], xmm0
1404932c0: lea rdx, [rbp + 0x10]
1404932c4: mov rcx, rsi
1404932c7: call 0x140326b40
1404932cc: mov qword ptr [rbp + 0x10], 0
1404932d4: mov qword ptr [rbp + 0x18], 8
1404932dc: mov qword ptr [rbp + 0x20], 0
1404932e4: lea rdx, [rbp + 0x10]
1404932e8: mov rcx, rsi
1404932eb: call 0x14011bfe0
1404932f0: mov rsi, rax
1404932f3: mov rcx, rax
1404932f6: call 0x1403270d0
1404932fb: lea rcx, [rbp + 0x10]
1404932ff: call 0x14030fb40
140493304: mov byte ptr [rbp - 0x60], 0x18
140493308: mov rcx, qword ptr [rip + 0x3199061]
14049330f: lea r8, [rip + 0x2497c82]
140493316: lea rdx, [rbp - 0x60]
14049331a: call 0x140364d20
14049331f: mov r8, rdx
140493322: lea rcx, [rbp + 0x10]
140493326: mov rdx, rax
140493329: call 0x1400d68c0
14049332e: movups xmm0, xmmword ptr [rbp + 0x70]
140493332: movaps xmmword ptr [rbp + 0x150], xmm0
140493339: movups xmm0, xmmword ptr [rbp + 0x60]
14049333d: movaps xmmword ptr [rbp + 0x140], xmm0
140493344: movups xmm0, xmmword ptr [rbp + 0x50]
140493348: movaps xmmword ptr [rbp + 0x130], xmm0
14049334f: movups xmm0, xmmword ptr [rbp + 0x10]
140493353: movups xmm1, xmmword ptr [rbp + 0x20]
140493357: movups xmm2, xmmword ptr [rbp + 0x30]
14049335b: movups xmm3, xmmword ptr [rbp + 0x40]
14049335f: movaps xmmword ptr [rbp + 0x120], xmm3
140493366: movaps xmmword ptr [rbp + 0x110], xmm2
14049336d: movaps xmmword ptr [rbp + 0x100], xmm1
140493374: movaps xmmword ptr [rbp + 0xf0], xmm0
14049337b: lea rdx, [rbp + 0xf0]
140493382: mov rcx, rsi
140493385: call 0x1403276f0
14049338a: lea rcx, [rbp + 0x10]
14049338e: call 0x14033def0
140493393: mov byte ptr [rbp - 0x60], 0x1c
140493397: mov rcx, qword ptr [rip + 0x3198fd2]
14049339e: lea r8, [rip + 0x2497bf3]
1404933a5: lea rdx, [rbp - 0x60]
1404933a9: call 0x140364d20
1404933ae: mov r8, rdx
1404933b1: lea rcx, [rbp + 0x10]
1404933b5: mov rdx, rax
1404933b8: call 0x1400d68c0
1404933bd: movups xmm0, xmmword ptr [rbp + 0x70]
1404933c1: movaps xmmword ptr [rbp + 0x150], xmm0
1404933c8: movups xmm0, xmmword ptr [rbp + 0x60]
1404933cc: movaps xmmword ptr [rbp + 0x140], xmm0
1404933d3: movups xmm0, xmmword ptr [rbp + 0x50]
1404933d7: movaps xmmword ptr [rbp + 0x130], xmm0
1404933de: movups xmm0, xmmword ptr [rbp + 0x10]
1404933e2: movups xmm1, xmmword ptr [rbp + 0x20]
1404933e6: movups xmm2, xmmword ptr [rbp + 0x30]
1404933ea: movups xmm3, xmmword ptr [rbp + 0x40]
1404933ee: movaps xmmword ptr [rbp + 0x120], xmm3
1404933f5: movaps xmmword ptr [rbp + 0x110], xmm2
1404933fc: movaps xmmword ptr [rbp + 0x100], xmm1
140493403: movaps xmmword ptr [rbp + 0xf0], xmm0
14049340a: lea rdx, [rbp + 0xf0]
140493411: mov rcx, rsi
140493414: call 0x1403276f0
140493419: lea r8, [rip + 0x254bc98]
140493420: mov rcx, rdi
140493423: mov dl, 0x24
140493425: call 0x1403104a0
14049342a: mov rcx, rsi
14049342d: mov rdx, rdi
140493430: call 0x1403276f0
140493435: lea rcx, [rbp + 0x10]
140493439: call 0x1400cbd40
14049343e: mov byte ptr [rbp - 0x60], 0x22
140493442: mov rcx, qword ptr [rip + 0x3198f27]
140493449: lea r8, [rip + 0x2497b48]
140493450: lea rdx, [rbp - 0x60]
140493454: call 0x140364d20
140493459: mov r8, rdx
14049345c: lea rcx, [rbp + 0x10]
140493460: mov rdx, rax
140493463: call 0x1400d68c0
140493468: movups xmm0, xmmword ptr [rbp + 0x70]
14049346c: movaps xmmword ptr [rbp + 0x150], xmm0
140493473: movups xmm0, xmmword ptr [rbp + 0x60]
140493477: movaps xmmword ptr [rbp + 0x140], xmm0
14049347e: movups xmm0, xmmword ptr [rbp + 0x50]
140493482: movaps xmmword ptr [rbp + 0x130], xmm0
140493489: movups xmm0, xmmword ptr [rbp + 0x10]
14049348d: movups xmm1, xmmword ptr [rbp + 0x20]
140493491: movups xmm2, xmmword ptr [rbp + 0x30]
140493495: movups xmm3, xmmword ptr [rbp + 0x40]
140493499: movaps xmmword ptr [rbp + 0x120], xmm3
1404934a0: movaps xmmword ptr [rbp + 0x110], xmm2
1404934a7: movaps xmmword ptr [rbp + 0x100], xmm1
1404934ae: movaps xmmword ptr [rbp + 0xf0], xmm0
1404934b5: lea rdx, [rbp + 0xf0]
1404934bc: mov rcx, rsi
1404934bf: call 0x1403276f0
1404934c4: lea r8, [rip + 0x254bc05]
1404934cb: mov rcx, rdi
1404934ce: mov dl, 0x2c
1404934d0: call 0x1400f50e0
1404934d5: mov rcx, rsi
1404934d8: mov rdx, rdi
1404934db: call 0x1403276f0
1404934e0: lea rcx, [rbp + 0x10]
1404934e4: call 0x140272c20
1404934e9: mov byte ptr [rbp - 0x60], 0x20
1404934ed: mov rcx, qword ptr [rip + 0x3198e7c]
1404934f4: lea r8, [rip + 0x2497a9d]
1404934fb: lea rdx, [rbp - 0x60]
1404934ff: call 0x140364d20
140493504: mov r8, rdx
140493507: lea rcx, [rbp + 0x10]
14049350b: mov rdx, rax
14049350e: call 0x1400d68c0
140493513: movups xmm0, xmmword ptr [rbp + 0x70]
140493517: movaps xmmword ptr [rbp + 0x150], xmm0
14049351e: movups xmm0, xmmword ptr [rbp + 0x60]
140493522: movaps xmmword ptr [rbp + 0x140], xmm0
140493529: movups xmm0, xmmword ptr [rbp + 0x50]
14049352d: movaps xmmword ptr [rbp + 0x130], xmm0
140493534: movups xmm0, xmmword ptr [rbp + 0x10]
140493538: movups xmm1, xmmword ptr [rbp + 0x20]
14049353c: movups xmm2, xmmword ptr [rbp + 0x30]
140493540: movups xmm3, xmmword ptr [rbp + 0x40]
140493544: movaps xmmword ptr [rbp + 0x120], xmm3
14049354b: movaps xmmword ptr [rbp + 0x110], xmm2
140493552: movaps xmmword ptr [rbp + 0x100], xmm1
140493559: movaps xmmword ptr [rbp + 0xf0], xmm0
140493560: lea rdx, [rbp + 0xf0]
140493567: mov rcx, rsi
14049356a: call 0x1403276f0
14049356f: mov rcx, rsi
140493572: mov dl, 1
140493574: call 0x140327c20
140493579: mov rcx, rsi
14049357c: add rsp, 0x1f0
140493583: pop rdi
140493584: pop rsi
140493585: pop rbp
140493586: jmp 0x140327ae0
14049358b: nop dword ptr [rax + rax]
140493590: mov qword ptr [rsp + 0x10], rdx
140493595: push rbp
140493596: push rsi
140493597: push rdi
140493598: sub rsp, 0x20
14049359c: lea rbp, [rdx + 0x80]
1404935a3: lea rcx, [rbp - 0x60]
1404935a7: call 0x140029880
1404935ac: nop 
1404935ad: add rsp, 0x20
1404935b1: pop rdi
1404935b2: pop rsi
1404935b3: pop rbp
1404935b4: ret 
1404935b5: nop word ptr cs:[rax + rax]
1404935c0: mov qword ptr [rsp + 0x10], rdx
1404935c5: push rbp
1404935c6: push rsi
1404935c7: push rdi
1404935c8: sub rsp, 0x20
1404935cc: lea rbp, [rdx + 0x80]
1404935d3: lea rcx, [rbp + 0xf0]
1404935da: call 0x140029880
1404935df: nop 
1404935e0: add rsp, 0x20
1404935e4: pop rdi
1404935e5: pop rsi
1404935e6: pop rbp
1404935e7: ret 
1404935e8: nop dword ptr [rax + rax]
1404935f0: mov qword ptr [rsp + 0x10], rdx
1404935f5: push rbp
1404935f6: push rsi
1404935f7: push rdi
1404935f8: sub rsp, 0x20
1404935fc: lea rbp, [rdx + 0x80]
140493603: lea rcx, [rbp + 0x10]
140493607: call 0x140029880
14049360c: nop 
14049360d: add rsp, 0x20
140493611: pop rdi
140493612: pop rsi
140493613: pop rbp
140493614: ret 
140493615: nop word ptr cs:[rax + rax]
140493620: mov qword ptr [rsp + 0x10], rdx
140493625: push rbp
140493626: push rsi
140493627: push rdi
140493628: sub rsp, 0x20
14049362c: lea rbp, [rdx + 0x80]
140493633: lea rcx, [rbp + 0x10]
140493637: call 0x140029880
14049363c: nop 
14049363d: add rsp, 0x20
140493641: pop rdi
140493642: pop rsi
140493643: pop rbp
140493644: ret 
140493645: nop word ptr cs:[rax + rax]
140493650: mov qword ptr [rsp + 0x10], rdx
140493655: push rbp
140493656: push rsi
140493657: push rdi
140493658: sub rsp, 0x20
14049365c: lea rbp, [rdx + 0x80]
140493663: lea rcx, [rbp + 0x10]
140493667: call 0x140029880
14049366c: nop 
14049366d: add rsp, 0x20
140493671: pop rdi
140493672: pop rsi
140493673: pop rbp
140493674: ret 
140493675: nop word ptr cs:[rax + rax]
140493680: mov qword ptr [rsp + 0x10], rdx
140493685: push rbp
140493686: push rsi
140493687: push rdi
140493688: sub rsp, 0x20
14049368c: lea rbp, [rdx + 0x80]
140493693: lea rcx, [rbp + 0x10]
140493697: call 0x140029880
14049369c: nop 
14049369d: add rsp, 0x20
1404936a1: pop rdi
1404936a2: pop rsi
1404936a3: pop rbp
1404936a4: ret 

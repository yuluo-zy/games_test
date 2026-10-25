0x140493120 55 push rbp
0x140493121 56 push rsi
0x140493122 57 push rdi
0x140493123 4881ecf0010000 sub rsp, 0x1f0
0x14049312a 488dac2480000000 lea rbp, [rsp + 0x80]
0x140493132 48c78568010000feffffff mov qword ptr [rbp + 0x168], 0xfffffffffffffffe
0x14049313d 4889ce mov rsi, rcx
0x140493140 4c8d0541bf5402 lea r8, [rip + 0x254bf41]
0x140493147 488d7d10 lea rdi, [rbp + 0x10]
0x14049314b 4889f9 mov rcx, rdi
0x14049314e b211 mov dl, 0x11
0x140493150 e8dbfbc7ff call 0x140112d30
0x140493155 4889f1 mov rcx, rsi
0x140493158 4889fa mov rdx, rdi
0x14049315b e89045e9ff call 0x1403276f0
0x140493160 4c8d0539bf5402 lea r8, [rip + 0x254bf39]
0x140493167 4889f9 mov rcx, rdi
0x14049316a b22a mov dl, 0x2a
0x14049316c e8cffbe8ff call 0x140322d40
0x140493171 4889f1 mov rcx, rsi
0x140493174 4889fa mov rdx, rdi
0x140493177 e87445e9ff call 0x1403276f0
0x14049317c 4889f9 mov rcx, rdi
0x14049317f e8dcb70b00 call 0x14054e960
0x140493184 488d4da0 lea rcx, [rbp - 0x60]
0x140493188 4889fa mov rdx, rdi
0x14049318b e800a10700 call 0x14050d290
0x140493190 488d8df0000000 lea rcx, [rbp + 0xf0]
0x140493197 e89429eaff call 0x140335b30
0x14049319c 0f104500 movups xmm0, xmmword ptr [rbp]
0x1404931a0 0f294570 movaps xmmword ptr [rbp + 0x70], xmm0
0x1404931a4 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
0x1404931a8 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
0x1404931ac 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
0x1404931b0 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
0x1404931b4 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
0x1404931b8 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
0x1404931bc 0f1055c0 movups xmm2, xmmword ptr [rbp - 0x40]
0x1404931c0 0f105dd0 movups xmm3, xmmword ptr [rbp - 0x30]
0x1404931c4 0f295d40 movaps xmmword ptr [rbp + 0x40], xmm3
0x1404931c8 0f295530 movaps xmmword ptr [rbp + 0x30], xmm2
0x1404931cc 0f294d20 movaps xmmword ptr [rbp + 0x20], xmm1
0x1404931d0 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x1404931d4 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
0x1404931db 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
0x1404931e2 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
0x1404931e9 0f109d20010000 movups xmm3, xmmword ptr [rbp + 0x120]
0x1404931f0 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
0x1404931f7 0f298d90000000 movaps xmmword ptr [rbp + 0x90], xmm1
0x1404931fe 0f2995a0000000 movaps xmmword ptr [rbp + 0xa0], xmm2
0x140493205 0f299db0000000 movaps xmmword ptr [rbp + 0xb0], xmm3
0x14049320c 0f108530010000 movups xmm0, xmmword ptr [rbp + 0x130]
0x140493213 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0
0x14049321a 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
0x140493221 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
0x140493228 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
0x14049322f 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
0x140493236 488d8df0000000 lea rcx, [rbp + 0xf0]
0x14049323d 488d5510 lea rdx, [rbp + 0x10]
0x140493241 e83a1ec4ff call 0x1400d5080
0x140493246 c645a013 mov byte ptr [rbp - 0x60], 0x13
0x14049324a 488b0d1f911903 mov rcx, qword ptr [rip + 0x319911f]
0x140493251 4c8d05407d4902 lea r8, [rip + 0x2497d40]
0x140493258 488d55a0 lea rdx, [rbp - 0x60]
0x14049325c e8bf1aedff call 0x140364d20
0x140493261 4989d0 mov r8, rdx
0x140493264 488d8df0000000 lea rcx, [rbp + 0xf0]
0x14049326b 4889c2 mov rdx, rax
0x14049326e e84d36c4ff call 0x1400d68c0
0x140493273 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
0x14049327a 0f294570 movaps xmmword ptr [rbp + 0x70], xmm0
0x14049327e 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
0x140493285 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
0x140493289 0f108530010000 movups xmm0, xmmword ptr [rbp + 0x130]
0x140493290 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
0x140493294 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
0x14049329b 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
0x1404932a2 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
0x1404932a9 0f109d20010000 movups xmm3, xmmword ptr [rbp + 0x120]
0x1404932b0 0f295d40 movaps xmmword ptr [rbp + 0x40], xmm3
0x1404932b4 0f295530 movaps xmmword ptr [rbp + 0x30], xmm2
0x1404932b8 0f294d20 movaps xmmword ptr [rbp + 0x20], xmm1
0x1404932bc 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x1404932c0 488d5510 lea rdx, [rbp + 0x10]
0x1404932c4 4889f1 mov rcx, rsi
0x1404932c7 e87438e9ff call 0x140326b40
0x1404932cc 48c7451000000000 mov qword ptr [rbp + 0x10], 0
0x1404932d4 48c7451808000000 mov qword ptr [rbp + 0x18], 8
0x1404932dc 48c7452000000000 mov qword ptr [rbp + 0x20], 0
0x1404932e4 488d5510 lea rdx, [rbp + 0x10]
0x1404932e8 4889f1 mov rcx, rsi
0x1404932eb e8f08cc8ff call 0x14011bfe0
0x1404932f0 4889c6 mov rsi, rax
0x1404932f3 4889c1 mov rcx, rax
0x1404932f6 e8d53de9ff call 0x1403270d0
0x1404932fb 488d4d10 lea rcx, [rbp + 0x10]
0x1404932ff e83cc8e7ff call 0x14030fb40
0x140493304 c645a018 mov byte ptr [rbp - 0x60], 0x18
0x140493308 488b0d61901903 mov rcx, qword ptr [rip + 0x3199061]
0x14049330f 4c8d05827c4902 lea r8, [rip + 0x2497c82]
0x140493316 488d55a0 lea rdx, [rbp - 0x60]
0x14049331a e8011aedff call 0x140364d20
0x14049331f 4989d0 mov r8, rdx
0x140493322 488d4d10 lea rcx, [rbp + 0x10]
0x140493326 4889c2 mov rdx, rax
0x140493329 e89235c4ff call 0x1400d68c0
0x14049332e 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
0x140493332 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
0x140493339 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x14049333d 0f298540010000 movaps xmmword ptr [rbp + 0x140], xmm0
0x140493344 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
0x140493348 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
0x14049334f 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x140493353 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
0x140493357 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
0x14049335b 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
0x14049335f 0f299d20010000 movaps xmmword ptr [rbp + 0x120], xmm3
0x140493366 0f299510010000 movaps xmmword ptr [rbp + 0x110], xmm2
0x14049336d 0f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm1
0x140493374 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
0x14049337b 488d95f0000000 lea rdx, [rbp + 0xf0]
0x140493382 4889f1 mov rcx, rsi
0x140493385 e86643e9ff call 0x1403276f0
0x14049338a 488d4d10 lea rcx, [rbp + 0x10]
0x14049338e e85dabeaff call 0x14033def0
0x140493393 c645a01c mov byte ptr [rbp - 0x60], 0x1c
0x140493397 488b0dd28f1903 mov rcx, qword ptr [rip + 0x3198fd2]
0x14049339e 4c8d05f37b4902 lea r8, [rip + 0x2497bf3]
0x1404933a5 488d55a0 lea rdx, [rbp - 0x60]
0x1404933a9 e87219edff call 0x140364d20
0x1404933ae 4989d0 mov r8, rdx
0x1404933b1 488d4d10 lea rcx, [rbp + 0x10]
0x1404933b5 4889c2 mov rdx, rax
0x1404933b8 e80335c4ff call 0x1400d68c0
0x1404933bd 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
0x1404933c1 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
0x1404933c8 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x1404933cc 0f298540010000 movaps xmmword ptr [rbp + 0x140], xmm0
0x1404933d3 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
0x1404933d7 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
0x1404933de 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x1404933e2 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
0x1404933e6 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
0x1404933ea 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
0x1404933ee 0f299d20010000 movaps xmmword ptr [rbp + 0x120], xmm3
0x1404933f5 0f299510010000 movaps xmmword ptr [rbp + 0x110], xmm2
0x1404933fc 0f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm1
0x140493403 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
0x14049340a 488d95f0000000 lea rdx, [rbp + 0xf0]
0x140493411 4889f1 mov rcx, rsi
0x140493414 e8d742e9ff call 0x1403276f0
0x140493419 4c8d0598bc5402 lea r8, [rip + 0x254bc98]
0x140493420 4889f9 mov rcx, rdi
0x140493423 b224 mov dl, 0x24
0x140493425 e876d0e7ff call 0x1403104a0
0x14049342a 4889f1 mov rcx, rsi
0x14049342d 4889fa mov rdx, rdi
0x140493430 e8bb42e9ff call 0x1403276f0
0x140493435 488d4d10 lea rcx, [rbp + 0x10]
0x140493439 e80289c3ff call 0x1400cbd40
0x14049343e c645a022 mov byte ptr [rbp - 0x60], 0x22
0x140493442 488b0d278f1903 mov rcx, qword ptr [rip + 0x3198f27]
0x140493449 4c8d05487b4902 lea r8, [rip + 0x2497b48]
0x140493450 488d55a0 lea rdx, [rbp - 0x60]
0x140493454 e8c718edff call 0x140364d20
0x140493459 4989d0 mov r8, rdx
0x14049345c 488d4d10 lea rcx, [rbp + 0x10]
0x140493460 4889c2 mov rdx, rax
0x140493463 e85834c4ff call 0x1400d68c0
0x140493468 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
0x14049346c 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
0x140493473 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x140493477 0f298540010000 movaps xmmword ptr [rbp + 0x140], xmm0
0x14049347e 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
0x140493482 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
0x140493489 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x14049348d 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
0x140493491 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
0x140493495 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
0x140493499 0f299d20010000 movaps xmmword ptr [rbp + 0x120], xmm3
0x1404934a0 0f299510010000 movaps xmmword ptr [rbp + 0x110], xmm2
0x1404934a7 0f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm1
0x1404934ae 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
0x1404934b5 488d95f0000000 lea rdx, [rbp + 0xf0]
0x1404934bc 4889f1 mov rcx, rsi
0x1404934bf e82c42e9ff call 0x1403276f0
0x1404934c4 4c8d0505bc5402 lea r8, [rip + 0x254bc05]
0x1404934cb 4889f9 mov rcx, rdi
0x1404934ce b22c mov dl, 0x2c
0x1404934d0 e80b1cc6ff call 0x1400f50e0
0x1404934d5 4889f1 mov rcx, rsi
0x1404934d8 4889fa mov rdx, rdi
0x1404934db e81042e9ff call 0x1403276f0
0x1404934e0 488d4d10 lea rcx, [rbp + 0x10]
0x1404934e4 e837f7ddff call 0x140272c20
0x1404934e9 c645a020 mov byte ptr [rbp - 0x60], 0x20
0x1404934ed 488b0d7c8e1903 mov rcx, qword ptr [rip + 0x3198e7c]
0x1404934f4 4c8d059d7a4902 lea r8, [rip + 0x2497a9d]
0x1404934fb 488d55a0 lea rdx, [rbp - 0x60]
0x1404934ff e81c18edff call 0x140364d20
0x140493504 4989d0 mov r8, rdx
0x140493507 488d4d10 lea rcx, [rbp + 0x10]
0x14049350b 4889c2 mov rdx, rax
0x14049350e e8ad33c4ff call 0x1400d68c0
0x140493513 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
0x140493517 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
0x14049351e 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x140493522 0f298540010000 movaps xmmword ptr [rbp + 0x140], xmm0
0x140493529 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
0x14049352d 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
0x140493534 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x140493538 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
0x14049353c 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
0x140493540 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
0x140493544 0f299d20010000 movaps xmmword ptr [rbp + 0x120], xmm3
0x14049354b 0f299510010000 movaps xmmword ptr [rbp + 0x110], xmm2
0x140493552 0f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm1
0x140493559 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
0x140493560 488d95f0000000 lea rdx, [rbp + 0xf0]
0x140493567 4889f1 mov rcx, rsi
0x14049356a e88141e9ff call 0x1403276f0
0x14049356f 4889f1 mov rcx, rsi
0x140493572 b201 mov dl, 1
0x140493574 e8a746e9ff call 0x140327c20
0x140493579 4889f1 mov rcx, rsi
0x14049357c 4881c4f0010000 add rsp, 0x1f0
0x140493583 5f pop rdi
0x140493584 5e pop rsi
0x140493585 5d pop rbp
0x140493586 e95545e9ff jmp 0x140327ae0
0x14049358b 0f1f440000 nop dword ptr [rax + rax]
0x140493590 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140493595 55 push rbp
0x140493596 56 push rsi
0x140493597 57 push rdi
0x140493598 4883ec20 sub rsp, 0x20
0x14049359c 488daa80000000 lea rbp, [rdx + 0x80]
0x1404935a3 488d4da0 lea rcx, [rbp - 0x60]
0x1404935a7 e8d462b9ff call 0x140029880
0x1404935ac 90 nop 
0x1404935ad 4883c420 add rsp, 0x20
0x1404935b1 5f pop rdi
0x1404935b2 5e pop rsi
0x1404935b3 5d pop rbp
0x1404935b4 c3 ret 
0x1404935b5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404935c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404935c5 55 push rbp
0x1404935c6 56 push rsi
0x1404935c7 57 push rdi
0x1404935c8 4883ec20 sub rsp, 0x20
0x1404935cc 488daa80000000 lea rbp, [rdx + 0x80]
0x1404935d3 488d8df0000000 lea rcx, [rbp + 0xf0]
0x1404935da e8a162b9ff call 0x140029880
0x1404935df 90 nop 
0x1404935e0 4883c420 add rsp, 0x20
0x1404935e4 5f pop rdi
0x1404935e5 5e pop rsi
0x1404935e6 5d pop rbp
0x1404935e7 c3 ret 
0x1404935e8 0f1f840000000000 nop dword ptr [rax + rax]
0x1404935f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404935f5 55 push rbp
0x1404935f6 56 push rsi
0x1404935f7 57 push rdi
0x1404935f8 4883ec20 sub rsp, 0x20
0x1404935fc 488daa80000000 lea rbp, [rdx + 0x80]
0x140493603 488d4d10 lea rcx, [rbp + 0x10]
0x140493607 e87462b9ff call 0x140029880
0x14049360c 90 nop 
0x14049360d 4883c420 add rsp, 0x20
0x140493611 5f pop rdi
0x140493612 5e pop rsi
0x140493613 5d pop rbp
0x140493614 c3 ret 
0x140493615 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140493620 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140493625 55 push rbp
0x140493626 56 push rsi
0x140493627 57 push rdi
0x140493628 4883ec20 sub rsp, 0x20
0x14049362c 488daa80000000 lea rbp, [rdx + 0x80]
0x140493633 488d4d10 lea rcx, [rbp + 0x10]
0x140493637 e84462b9ff call 0x140029880
0x14049363c 90 nop 
0x14049363d 4883c420 add rsp, 0x20
0x140493641 5f pop rdi
0x140493642 5e pop rsi
0x140493643 5d pop rbp
0x140493644 c3 ret 
0x140493645 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140493650 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140493655 55 push rbp
0x140493656 56 push rsi
0x140493657 57 push rdi
0x140493658 4883ec20 sub rsp, 0x20
0x14049365c 488daa80000000 lea rbp, [rdx + 0x80]
0x140493663 488d4d10 lea rcx, [rbp + 0x10]
0x140493667 e81462b9ff call 0x140029880
0x14049366c 90 nop 
0x14049366d 4883c420 add rsp, 0x20
0x140493671 5f pop rdi
0x140493672 5e pop rsi
0x140493673 5d pop rbp
0x140493674 c3 ret 
0x140493675 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140493680 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140493685 55 push rbp
0x140493686 56 push rsi
0x140493687 57 push rdi
0x140493688 4883ec20 sub rsp, 0x20
0x14049368c 488daa80000000 lea rbp, [rdx + 0x80]
0x140493693 488d4d10 lea rcx, [rbp + 0x10]
0x140493697 e8e461b9ff call 0x140029880
0x14049369c 90 nop 
0x14049369d 4883c420 add rsp, 0x20
0x1404936a1 5f pop rdi
0x1404936a2 5e pop rsi
0x1404936a3 5d pop rbp
0x1404936a4 c3 ret 

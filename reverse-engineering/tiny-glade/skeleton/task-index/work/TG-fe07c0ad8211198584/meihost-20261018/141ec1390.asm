0x141ec1390 push r15
0x141ec1392 push r14
0x141ec1394 push rsi
0x141ec1395 push rdi
0x141ec1396 push rbx
0x141ec1397 sub rsp, 0x60
0x141ec139b mov rdi, r8
0x141ec139e mov rsi, rdx
0x141ec13a1 mov dword ptr [rsp + 0x20], 0x43160000
0x141ec13a9 mov rax, qword ptr [r9]
0x141ec13ac movss xmm0, dword ptr [rax + 0x10]
0x141ec13b1 divss xmm0, dword ptr [rax + 0x24]
0x141ec13b6 xorps xmm1, xmm1
0x141ec13b9 maxss xmm1, xmm0
0x141ec13bd movss xmm0, dword ptr [rip + 0xa6453f] ; 0x142925904 0000803f9a99193f00000000feffffff f32=1.0 str=b''
0x141ec13c5 movaps xmm2, xmm0
0x141ec13c8 minss xmm2, xmm1
0x141ec13cc subss xmm0, xmm2
0x141ec13d0 movss dword ptr [rsp + 0x24], xmm0
0x141ec13d6 mov eax, dword ptr [rcx + 0x1c]
0x141ec13d9 mov rdx, qword ptr [rcx + 0x10]
0x141ec13dd mov dword ptr [rdx], eax
0x141ec13df mov r14, qword ptr [rcx]
0x141ec13e2 lea rax, [rip + 0x10da277] ; 0x142f9b660 6c6f676f73000000007a5e4001000000 f32=7.1625738050288085e+28 str=b'logos'
0x141ec13e9 mov qword ptr [rsp + 0x40], rax
0x141ec13ee mov qword ptr [rsp + 0x48], 5
0x141ec13f7 lea rcx, [rsp + 0x40]
0x141ec13fc call 0x1405bdda0
0x141ec1401 movabs rbx, 0xb1695320004fb443
0x141ec140b xor rbx, rax
0x141ec140e mov rcx, r14
0x141ec1411 mov rdx, rbx
0x141ec1414 call 0x140683ec0
0x141ec1419 mov byte ptr [rax + 0x5d], 0
0x141ec141d test dl, 1
0x141ec1420 jne 0x141ec1455
0x141ec1422 mov r15, qword ptr [r14 + 0x10]
0x141ec1426 cmp r15, qword ptr [r14]
0x141ec1429 jne 0x141ec143a
0x141ec142b lea rdx, [rip + 0xcedb96] ; 0x142baefc8 a0efba42010000002600000000000000 f32=93.468017578125 str=b'\xa0\xef\xbaB\x01'
0x141ec1432 mov rcx, r14
0x141ec1435 call 0x1406319d0
0x141ec143a mov rax, qword ptr [r14 + 8]
0x141ec143e imul rcx, r15, 0x38
0x141ec1442 mov dword ptr [rax + rcx], 0
0x141ec1449 mov qword ptr [rax + rcx + 0x30], rbx
0x141ec144e inc r15
0x141ec1451 mov qword ptr [r14 + 0x10], r15
0x141ec1455 mov qword ptr [rsp + 0x28], r14
0x141ec145a mov qword ptr [rsp + 0x30], rbx
0x141ec145f mov qword ptr [rsp + 0x38], 0
0x141ec1468 cmp qword ptr [r14 + 0x1b8], 0
0x141ec1470 je 0x141ec15a4
0x141ec1476 lea rdx, [rsp + 0x30]
0x141ec147b lea rcx, [r14 + 0x1c0]
0x141ec1482 call 0x1405b0d70
0x141ec1487 mov rcx, qword ptr [r14 + 0x1a0]
0x141ec148e mov rdx, qword ptr [r14 + 0x1a8]
0x141ec1495 mov r8, rdx
0x141ec1498 and r8, rax
0x141ec149b shr rax, 0x39
0x141ec149f movd xmm0, eax
0x141ec14a3 punpcklbw xmm0, xmm0
0x141ec14a7 pshuflw xmm0, xmm0, 0
0x141ec14ac pshufd xmm0, xmm0, 0
0x141ec14b1 lea rax, [rcx - 0xc0]
0x141ec14b8 xor r9d, r9d
0x141ec14bb pcmpeqd xmm1, xmm1
0x141ec14bf movdqu xmm2, xmmword ptr [rcx + r8]
0x141ec14c5 movdqa xmm3, xmm2
0x141ec14c9 pcmpeqb xmm3, xmm0
0x141ec14cd pmovmskb r10d, xmm3
0x141ec14d2 test r10d, r10d
0x141ec14d5 je 0x141ec1500
0x141ec14d7 tzcnt r11d, r10d
0x141ec14dc add r11, r8
0x141ec14df and r11, rdx
0x141ec14e2 neg r11
0x141ec14e5 lea r11, [r11 + r11*2]
0x141ec14e9 shl r11, 6
0x141ec14ed cmp rbx, qword ptr [rax + r11]
0x141ec14f1 je 0x141ec1522
0x141ec14f3 lea r11d, [r10 - 1]
0x141ec14f7 and r11w, r10w
0x141ec14fb mov r10d, r11d
0x141ec14fe jne 0x141ec14d7
0x141ec1500 pcmpeqb xmm2, xmm1
0x141ec1504 pmovmskb r10d, xmm2
0x141ec1509 test r10d, r10d
0x141ec150c jne 0x141ec15a4
0x141ec1512 add r8, r9
0x141ec1515 add r8, 0x10
0x141ec1519 add r9, 0x10
0x141ec151d and r8, rdx
0x141ec1520 jmp 0x141ec14bf
0x141ec1522 movaps xmm0, xmmword ptr [rip + 0x10da127] ; 0x142f9b650 00000000000048420000000000004842 f32=0.0 str=b''
0x141ec1529 movups xmmword ptr [rcx + r11 - 0xa0], xmm0
0x141ec1532 movups xmmword ptr [rcx + r11 - 0x90], xmm0
0x141ec153b xorps xmm0, xmm0
0x141ec153e movups xmmword ptr [rcx + r11 - 0x80], xmm0
0x141ec1544 movups xmmword ptr [rcx + r11 - 0x70], xmm0
0x141ec154a mov qword ptr [rcx + r11 - 0x63], 0
0x141ec1553 mov word ptr [rcx + r11 - 0x5b], 0x303
0x141ec155b mov rax, qword ptr [rsp + 0x38]
0x141ec1560 mov qword ptr [rsp + 0x50], rax
0x141ec1565 movups xmm0, xmmword ptr [rsp + 0x28]
0x141ec156a movaps xmmword ptr [rsp + 0x40], xmm0
0x141ec156f lea rcx, [rsp + 0x28]
0x141ec1574 lea rdx, [rsp + 0x40]
0x141ec1579 lea r8, [rsp + 0x24]
0x141ec157e lea r9, [rsp + 0x20]
0x141ec1583 call 0x140f99420
0x141ec1588 mov rax, qword ptr [rdi]
0x141ec158b movss xmm0, dword ptr [rax + 0x28]
0x141ec1590 addss xmm0, dword ptr [rsi]
0x141ec1594 movss dword ptr [rsi], xmm0
0x141ec1598 add rsp, 0x60
0x141ec159c pop rbx
0x141ec159d pop rdi
0x141ec159e pop rsi
0x141ec159f pop r14
0x141ec15a1 pop r15
0x141ec15a3 ret 
0x141ec15a4 lea rcx, [rip + 0xb569b5] ; 0x142a17f60 f87ba142010000002600000000000000 f32=80.74212646484375 str=b'\xf8{\xa1B\x01'
0x141ec15ab call 0x1428d9310
0x141ec15b0 int3 
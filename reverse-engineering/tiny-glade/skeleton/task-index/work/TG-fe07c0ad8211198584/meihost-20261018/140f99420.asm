0x140f99420 push r15
0x140f99422 push r14
0x140f99424 push rsi
0x140f99425 push rdi
0x140f99426 push rbp
0x140f99427 push rbx
0x140f99428 sub rsp, 0xa8
0x140f9942f movaps xmmword ptr [rsp + 0x90], xmm7
0x140f99437 movaps xmmword ptr [rsp + 0x80], xmm6
0x140f9943f mov rbx, qword ptr [rdx]
0x140f99442 cmp qword ptr [rbx + 0x1b8], 0
0x140f9944a je 0x140f99d84
0x140f99450 mov rdi, rdx
0x140f99453 mov rsi, rcx
0x140f99456 movss xmm7, dword ptr [r8]
0x140f9945b movss xmm6, dword ptr [r9]
0x140f99460 add rdx, 8
0x140f99464 lea rcx, [rbx + 0x1c0]
0x140f9946b call 0x1405b0d70
0x140f99470 mov rcx, qword ptr [rbx + 0x1a0]
0x140f99477 mov rdx, qword ptr [rbx + 0x1a8]
0x140f9947e mov r8, rdx
0x140f99481 and r8, rax
0x140f99484 shr rax, 0x39
0x140f99488 movd xmm0, eax
0x140f9948c punpcklbw xmm0, xmm0
0x140f99490 pshuflw xmm0, xmm0, 0
0x140f99495 pshufd xmm0, xmm0, 0
0x140f9949a lea rax, [rcx - 0xc0]
0x140f994a1 mov r9, qword ptr [rdi + 8]
0x140f994a5 xor r10d, r10d
0x140f994a8 pcmpeqd xmm1, xmm1
0x140f994ac movdqu xmm2, xmmword ptr [rcx + r8]
0x140f994b2 movdqa xmm3, xmm2
0x140f994b6 pcmpeqb xmm3, xmm0
0x140f994ba pmovmskb r11d, xmm3
0x140f994bf test r11d, r11d
0x140f994c2 je 0x140f994f0
0x140f994c4 tzcnt ebx, r11d
0x140f994c9 add rbx, r8
0x140f994cc and rbx, rdx
0x140f994cf neg rbx
0x140f994d2 lea rbx, [rbx + rbx*2]
0x140f994d6 shl rbx, 6
0x140f994da cmp r9, qword ptr [rax + rbx]
0x140f994de je 0x140f99512
0x140f994e0 lea ebx, [r11 - 1]
0x140f994e4 and bx, r11w
0x140f994e8 mov r11d, ebx
0x140f994eb jne 0x140f994c4
0x140f994ed nop dword ptr [rax]
0x140f994f0 pcmpeqb xmm2, xmm1
0x140f994f4 pmovmskb r11d, xmm2
0x140f994f9 test r11d, r11d
0x140f994fc jne 0x140f99d84
0x140f99502 add r8, r10
0x140f99505 add r8, 0x10
0x140f99509 add r10, 0x10
0x140f9950d and r8, rdx
0x140f99510 jmp 0x140f994ac
0x140f99512 movss dword ptr [rcx + rbx - 8], xmm7
0x140f99518 mov r15, qword ptr [rdi + 8]
0x140f9951c lea rax, [rip + 0x1c171ee] ; 0x142bb0711 64657673666d6f646d656e752f666d6f f32=1.9521506857082944e+31 str=b'devsfmodmenu/fmod_logopounce lightmenu/pounce_light_logopad0heartmenu/heartpad1noknokmenu/noknok_log'
0x140f99523 mov qword ptr [rsp + 0x50], rax
0x140f99528 mov qword ptr [rsp + 0x58], 4
0x140f99531 lea rcx, [rsp + 0x50]
0x140f99536 call 0x1405bdda0
0x140f9953b mov qword ptr [rsp + 0x38], rax
0x140f99540 movabs rbx, 0x41223169ff28813b
0x140f9954a mov qword ptr [rsp + 0x40], rbx
0x140f9954f movabs r14, 0xa79b7268a2a968d9
0x140f99559 mov qword ptr [rsp + 0x48], r14
0x140f9955e lea rcx, [rsp + 0x38]
0x140f99563 call 0x1405bdf20
0x140f99568 mov qword ptr [rsp + 0x38], r15
0x140f9956d mov qword ptr [rsp + 0x40], rax
0x140f99572 lea rcx, [rsp + 0x38]
0x140f99577 call 0x1405bd910
0x140f9957c lea rcx, [rsp + 0x38]
0x140f99581 mov rdx, rdi
0x140f99584 mov r8, rax
0x140f99587 call 0x1406861b0
0x140f9958c mov r15, qword ptr [rsp + 0x38]
0x140f99591 cmp qword ptr [r15 + 0x1b8], 0
0x140f99599 je 0x140f99b34
0x140f9959f lea rdx, [rsp + 0x40]
0x140f995a4 lea rcx, [r15 + 0x1c0]
0x140f995ab call 0x1405b0d70
0x140f995b0 mov rcx, qword ptr [r15 + 0x1a0]
0x140f995b7 mov rdx, qword ptr [r15 + 0x1a8]
0x140f995be mov r8, rdx
0x140f995c1 and r8, rax
0x140f995c4 shr rax, 0x39
0x140f995c8 movd xmm0, eax
0x140f995cc punpcklbw xmm0, xmm0
0x140f995d0 pshuflw xmm0, xmm0, 0
0x140f995d5 pshufd xmm0, xmm0, 0
0x140f995da lea rax, [rcx - 0xc0]
0x140f995e1 xor r9d, r9d
0x140f995e4 mov r10, qword ptr [rsp + 0x40]
0x140f995e9 pcmpeqd xmm1, xmm1
0x140f995ed movdqu xmm2, xmmword ptr [rcx + r8]
0x140f995f3 movdqa xmm3, xmm2
0x140f995f7 pcmpeqb xmm3, xmm0
0x140f995fb pmovmskb r11d, xmm3
0x140f99600 test r11d, r11d
0x140f99603 je 0x140f99630
0x140f99605 tzcnt r15d, r11d
0x140f9960a add r15, r8
0x140f9960d and r15, rdx
0x140f99610 neg r15
0x140f99613 lea r15, [r15 + r15*2]
0x140f99617 shl r15, 6
0x140f9961b cmp r10, qword ptr [rax + r15]
0x140f9961f je 0x140f99652
0x140f99621 lea ebp, [r11 - 1]
0x140f99625 and bp, r11w
0x140f99629 mov r11d, ebp
0x140f9962c jne 0x140f99605
0x140f9962e nop 
0x140f99630 pcmpeqb xmm2, xmm1
0x140f99634 pmovmskb r11d, xmm2
0x140f99639 test r11d, r11d
0x140f9963c jne 0x140f99b34
0x140f99642 add r8, r9
0x140f99645 add r8, 0x10
0x140f99649 add r9, 0x10
0x140f9964d and r8, rdx
0x140f99650 jmp 0x140f995ed
0x140f99652 pxor xmm0, xmm0
0x140f99656 movdqu xmmword ptr [rcx + r15 - 0x70], xmm0
0x140f9965d movdqu xmmword ptr [rcx + r15 - 0x80], xmm0
0x140f99664 movdqu xmmword ptr [rcx + r15 - 0x90], xmm0
0x140f9966e movdqu xmmword ptr [rcx + r15 - 0xa0], xmm0
0x140f99678 mov qword ptr [rcx + r15 - 0x62], 0
0x140f99681 mov byte ptr [rcx + r15 - 0x5a], 4
0x140f99687 mov rax, qword ptr [rsp + 0x48]
0x140f9968c mov qword ptr [rsp + 0x60], rax
0x140f99691 movups xmm0, xmmword ptr [rsp + 0x38]
0x140f99696 movaps xmmword ptr [rsp + 0x50], xmm0
0x140f9969b movss dword ptr [rsp + 0x30], xmm6
0x140f996a1 lea rax, [rip + 0x1c1708b] ; 0x142bb0733 6d656e752f706f756e63655f6c696768 f32=3.022030789968193e+32 str=b'menu/pounce_light_logopad0heartmenu/heartpad1noknokmenu/noknok_logocrates/systems/toolbar/src/mei/co'
0x140f996a8 mov qword ptr [rsp + 0x20], rax
0x140f996ad mov qword ptr [rsp + 0x28], 0x16
0x140f996b6 lea r8, [rip + 0x1c1706a] ; 0x142bb0727 706f756e6365206c696768746d656e75 f32=1.8989636769313212e+28 str=b'pounce lightmenu/pounce_light_logopad0heartmenu/heartpad1noknokmenu/noknok_logocrates/systems/toolba'
0x140f996bd lea rcx, [rsp + 0x38]
0x140f996c2 lea rdx, [rsp + 0x50]
0x140f996c7 mov r9d, 0xc
0x140f996cd call 0x140f9fca0
0x140f996d2 mov r15, qword ptr [rsp + 0x38]
0x140f996d7 cmp qword ptr [r15 + 0x1b8], 0
0x140f996df je 0x140f99d84
0x140f996e5 lea rdx, [rsp + 0x40]
0x140f996ea lea rcx, [r15 + 0x1c0]
0x140f996f1 call 0x1405b0d70
0x140f996f6 mov rcx, qword ptr [r15 + 0x1a0]
0x140f996fd mov rdx, qword ptr [r15 + 0x1a8]
0x140f99704 mov r8, rdx
0x140f99707 and r8, rax
0x140f9970a shr rax, 0x39
0x140f9970e movd xmm0, eax
0x140f99712 punpcklbw xmm0, xmm0
0x140f99716 pshuflw xmm0, xmm0, 0
0x140f9971b pshufd xmm0, xmm0, 0
0x140f99720 lea rax, [rcx - 0xc0]
0x140f99727 xor r9d, r9d
0x140f9972a mov r10, qword ptr [rsp + 0x40]
0x140f9972f pcmpeqd xmm1, xmm1
0x140f99733 movdqu xmm2, xmmword ptr [rcx + r8]
0x140f99739 movdqa xmm3, xmm2
0x140f9973d pcmpeqb xmm3, xmm0
0x140f99741 pmovmskb r11d, xmm3
0x140f99746 test r11d, r11d
0x140f99749 je 0x140f99774
0x140f9974b tzcnt r15d, r11d
0x140f99750 add r15, r8
0x140f99753 and r15, rdx
0x140f99756 neg r15
0x140f99759 lea r15, [r15 + r15*2]
0x140f9975d shl r15, 6
0x140f99761 cmp r10, qword ptr [rax + r15]
0x140f99765 je 0x140f99796
0x140f99767 lea ebp, [r11 - 1]
0x140f9976b and bp, r11w
0x140f9976f mov r11d, ebp
0x140f99772 jne 0x140f9974b
0x140f99774 pcmpeqb xmm2, xmm1
0x140f99778 pmovmskb r11d, xmm2
0x140f9977d test r11d, r11d
0x140f99780 jne 0x140f99d84
0x140f99786 add r8, r9
0x140f99789 add r8, 0x10
0x140f9978d add r9, 0x10
0x140f99791 and r8, rdx
0x140f99794 jmp 0x140f99733
0x140f99796 or byte ptr [rcx + r15 - 1], 2
0x140f9979c mov r15, qword ptr [rsp + 0x58]
0x140f997a1 lea rax, [rip + 0x1c16fa1] ; 0x142bb0749 7061643068656172746d656e752f6865 f32=8.30843838173223e-10 str=b'pad0heartmenu/heartpad1noknokmenu/noknok_logocrates/systems/toolbar/src/mei/config.rsNo UI template '
0x140f997a8 mov qword ptr [rsp + 0x70], rax
0x140f997ad mov qword ptr [rsp + 0x78], 4
0x140f997b6 lea rcx, [rsp + 0x70]
0x140f997bb call 0x1405bdda0
0x140f997c0 mov qword ptr [rsp + 0x38], rax
0x140f997c5 mov qword ptr [rsp + 0x40], rbx
0x140f997ca mov qword ptr [rsp + 0x48], r14
0x140f997cf lea rcx, [rsp + 0x38]
0x140f997d4 call 0x1405bdf20
0x140f997d9 mov qword ptr [rsp + 0x38], r15
0x140f997de mov qword ptr [rsp + 0x40], rax
0x140f997e3 lea rcx, [rsp + 0x38]
0x140f997e8 call 0x1405bd910
0x140f997ed lea rcx, [rsp + 0x38]
0x140f997f2 lea rdx, [rsp + 0x50]
0x140f997f7 mov r8, rax
0x140f997fa call 0x1406861b0
0x140f997ff mov r15, qword ptr [rsp + 0x38]
0x140f99804 cmp qword ptr [r15 + 0x1b8], 0
0x140f9980c je 0x140f99b34
0x140f99812 lea rdx, [rsp + 0x40]
0x140f99817 lea rcx, [r15 + 0x1c0]
0x140f9981e call 0x1405b0d70
0x140f99823 mov rcx, qword ptr [r15 + 0x1a0]
0x140f9982a mov rdx, qword ptr [r15 + 0x1a8]
0x140f99831 mov r8, rdx
0x140f99834 and r8, rax
0x140f99837 shr rax, 0x39
0x140f9983b movd xmm0, eax
0x140f9983f punpcklbw xmm0, xmm0
0x140f99843 pshuflw xmm0, xmm0, 0
0x140f99848 pshufd xmm0, xmm0, 0
0x140f9984d lea rax, [rcx - 0xc0]
0x140f99854 xor r9d, r9d
0x140f99857 mov r10, qword ptr [rsp + 0x40]
0x140f9985c pcmpeqd xmm1, xmm1
0x140f99860 movdqu xmm2, xmmword ptr [rcx + r8]
0x140f99866 movdqa xmm3, xmm2
0x140f9986a pcmpeqb xmm3, xmm0
0x140f9986e pmovmskb r11d, xmm3
0x140f99873 test r11d, r11d
0x140f99876 je 0x140f998a1
0x140f99878 tzcnt r15d, r11d
0x140f9987d add r15, r8
0x140f99880 and r15, rdx
0x140f99883 neg r15
0x140f99886 lea r15, [r15 + r15*2]
0x140f9988a shl r15, 6
0x140f9988e cmp r10, qword ptr [rax + r15]
0x140f99892 je 0x140f998c3
0x140f99894 lea ebp, [r11 - 1]
0x140f99898 and bp, r11w
0x140f9989c mov r11d, ebp
0x140f9989f jne 0x140f99878
0x140f998a1 pcmpeqb xmm2, xmm1
0x140f998a5 pmovmskb r11d, xmm2
0x140f998aa test r11d, r11d
0x140f998ad jne 0x140f99b34
0x140f998b3 add r8, r9
0x140f998b6 add r8, 0x10
0x140f998ba add r9, 0x10
0x140f998be and r8, rdx
0x140f998c1 jmp 0x140f99860
0x140f998c3 pxor xmm0, xmm0
0x140f998c7 movdqu xmmword ptr [rcx + r15 - 0x84], xmm0
0x140f998d1 movdqu xmmword ptr [rcx + r15 - 0x90], xmm0
0x140f998db movdqu xmmword ptr [rcx + r15 - 0xa0], xmm0
0x140f998e5 mov dword ptr [rcx + r15 - 0x74], 0x41a00000
0x140f998ee movdqu xmmword ptr [rcx + r15 - 0x70], xmm0
0x140f998f5 mov qword ptr [rcx + r15 - 0x62], 0
0x140f998fe mov byte ptr [rcx + r15 - 0x5a], 4
0x140f99904 movss xmm0, dword ptr [rip + 0x198bf64] ; 0x142925870 0000003fc10ffc3e34a6c93900000042 f32=0.5 str=b''
0x140f9990c mulss xmm0, xmm6
0x140f99910 movss dword ptr [rsp + 0x30], xmm0
0x140f99916 lea rax, [rip + 0x1c16e35] ; 0x142bb0752 6d656e752f6865617274706164316e6f f32=3.022030789968193e+32 str=b'menu/heartpad1noknokmenu/noknok_logocrates/systems/toolbar/src/mei/config.rsNo UI template named . A'
0x140f9991d mov qword ptr [rsp + 0x20], rax
0x140f99922 mov qword ptr [rsp + 0x28], 0xa
0x140f9992b lea r8, [rip + 0x1c16e1b] ; 0x142bb074d 68656172746d656e752f686561727470 f32=4.464430069996668e+30 str=b'heartmenu/heartpad1noknokmenu/noknok_logocrates/systems/toolbar/src/mei/config.rsNo UI template name'
0x140f99932 lea rcx, [rsp + 0x38]
0x140f99937 lea rdx, [rsp + 0x50]
0x140f9993c mov r9d, 5
0x140f99942 call 0x140f9fca0
0x140f99947 mov r15, qword ptr [rsp + 0x38]
0x140f9994c cmp qword ptr [r15 + 0x1b8], 0
0x140f99954 je 0x140f99d84
0x140f9995a lea rdx, [rsp + 0x40]
0x140f9995f lea rcx, [r15 + 0x1c0]
0x140f99966 call 0x1405b0d70
0x140f9996b mov rcx, qword ptr [r15 + 0x1a0]
0x140f99972 mov rdx, qword ptr [r15 + 0x1a8]
0x140f99979 mov r8, rdx
0x140f9997c and r8, rax
0x140f9997f shr rax, 0x39
0x140f99983 movd xmm0, eax
0x140f99987 punpcklbw xmm0, xmm0
0x140f9998b pshuflw xmm0, xmm0, 0
0x140f99990 pshufd xmm0, xmm0, 0
0x140f99995 lea rax, [rcx - 0xc0]
0x140f9999c xor r9d, r9d
0x140f9999f mov r10, qword ptr [rsp + 0x40]
0x140f999a4 pcmpeqd xmm1, xmm1
0x140f999a8 movdqu xmm2, xmmword ptr [rcx + r8]
0x140f999ae movdqa xmm3, xmm2
0x140f999b2 pcmpeqb xmm3, xmm0
0x140f999b6 pmovmskb r11d, xmm3
0x140f999bb test r11d, r11d
0x140f999be je 0x140f999e9
0x140f999c0 tzcnt r15d, r11d
0x140f999c5 add r15, r8
0x140f999c8 and r15, rdx
0x140f999cb neg r15
0x140f999ce lea r15, [r15 + r15*2]
0x140f999d2 shl r15, 6
0x140f999d6 cmp r10, qword ptr [rax + r15]
0x140f999da je 0x140f99a0b
0x140f999dc lea ebp, [r11 - 1]
0x140f999e0 and bp, r11w
0x140f999e4 mov r11d, ebp
0x140f999e7 jne 0x140f999c0
0x140f999e9 pcmpeqb xmm2, xmm1
0x140f999ed pmovmskb r11d, xmm2
0x140f999f2 test r11d, r11d
0x140f999f5 jne 0x140f99d84
0x140f999fb add r8, r9
0x140f999fe add r8, 0x10
0x140f99a02 add r9, 0x10
0x140f99a06 and r8, rdx
0x140f99a09 jmp 0x140f999a8
0x140f99a0b or byte ptr [rcx + r15 - 1], 2
0x140f99a11 mov r15, qword ptr [rsp + 0x58]
0x140f99a16 lea rax, [rip + 0x1c16d3f] ; 0x142bb075c 706164316e6f6b6e6f6b6d656e752f6e f32=3.323375352692892e-09 str=b'pad1noknokmenu/noknok_logocrates/systems/toolbar/src/mei/config.rsNo UI template named . Available: '
0x140f99a1d mov qword ptr [rsp + 0x70], rax
0x140f99a22 mov qword ptr [rsp + 0x78], 4
0x140f99a2b lea rcx, [rsp + 0x70]
0x140f99a30 call 0x1405bdda0
0x140f99a35 mov qword ptr [rsp + 0x38], rax
0x140f99a3a mov qword ptr [rsp + 0x40], rbx
0x140f99a3f mov qword ptr [rsp + 0x48], r14
0x140f99a44 lea rcx, [rsp + 0x38]
0x140f99a49 call 0x1405bdf20
0x140f99a4e mov qword ptr [rsp + 0x38], r15
0x140f99a53 mov qword ptr [rsp + 0x40], rax
0x140f99a58 lea rcx, [rsp + 0x38]
0x140f99a5d call 0x1405bd910
0x140f99a62 lea rcx, [rsp + 0x38]
0x140f99a67 lea rdx, [rsp + 0x50]
0x140f99a6c mov r8, rax
0x140f99a6f call 0x1406861b0
0x140f99a74 mov rbx, qword ptr [rsp + 0x38]
0x140f99a79 cmp qword ptr [rbx + 0x1b8], 0
0x140f99a81 je 0x140f99b34
0x140f99a87 lea rdx, [rsp + 0x40]
0x140f99a8c lea rcx, [rbx + 0x1c0]
0x140f99a93 call 0x1405b0d70
0x140f99a98 mov rcx, qword ptr [rbx + 0x1a0]
0x140f99a9f mov rdx, qword ptr [rbx + 0x1a8]
0x140f99aa6 mov r8, rdx
0x140f99aa9 and r8, rax
0x140f99aac shr rax, 0x39
0x140f99ab0 movd xmm0, eax
0x140f99ab4 punpcklbw xmm0, xmm0
0x140f99ab8 pshuflw xmm0, xmm0, 0
0x140f99abd pshufd xmm0, xmm0, 0
0x140f99ac2 lea rax, [rcx - 0xc0]
0x140f99ac9 xor r9d, r9d
0x140f99acc mov r10, qword ptr [rsp + 0x40]
0x140f99ad1 pcmpeqd xmm1, xmm1
0x140f99ad5 movdqu xmm2, xmmword ptr [rcx + r8]
0x140f99adb movdqa xmm3, xmm2
0x140f99adf pcmpeqb xmm3, xmm0
0x140f99ae3 pmovmskb r11d, xmm3
0x140f99ae8 test r11d, r11d
0x140f99aeb je 0x140f99b16
0x140f99aed tzcnt ebx, r11d
0x140f99af2 add rbx, r8
0x140f99af5 and rbx, rdx
0x140f99af8 neg rbx
0x140f99afb lea rbx, [rbx + rbx*2]
0x140f99aff shl rbx, 6
0x140f99b03 cmp r10, qword ptr [rax + rbx]
0x140f99b07 je 0x140f99b40
0x140f99b09 lea ebx, [r11 - 1]
0x140f99b0d and bx, r11w
0x140f99b11 mov r11d, ebx
0x140f99b14 jne 0x140f99aed
0x140f99b16 pcmpeqb xmm2, xmm1
0x140f99b1a pmovmskb r11d, xmm2
0x140f99b1f test r11d, r11d
0x140f99b22 jne 0x140f99b34
0x140f99b24 add r8, r9
0x140f99b27 add r8, 0x10
0x140f99b2b add r9, 0x10
0x140f99b2f and r8, rdx
0x140f99b32 jmp 0x140f99ad5
0x140f99b34 lea rcx, [rip + 0x1a7e425] ; 0x142a17f60 f87ba142010000002600000000000000 f32=80.74212646484375 str=b'\xf8{\xa1B\x01'
0x140f99b3b call 0x1428d9310
0x140f99b40 pxor xmm0, xmm0
0x140f99b44 movdqu xmmword ptr [rcx + rbx - 0x84], xmm0
0x140f99b4d movdqu xmmword ptr [rcx + rbx - 0x90], xmm0
0x140f99b56 movdqu xmmword ptr [rcx + rbx - 0xa0], xmm0
0x140f99b5f mov dword ptr [rcx + rbx - 0x74], 0x41200000
0x140f99b67 movdqu xmmword ptr [rcx + rbx - 0x70], xmm0
0x140f99b6d mov qword ptr [rcx + rbx - 0x62], 0
0x140f99b76 mov byte ptr [rcx + rbx - 0x5a], 4
0x140f99b7b movss xmm0, dword ptr [rip + 0x1a7d135] ; 0x142a16cb8 cdcc4c3f6372617465732f7379737465 f32=0.800000011920929 str=b'\xcd\xccL?crates/systems/critters/src/lib.rs'
0x140f99b83 mulss xmm0, xmm6
0x140f99b87 movss dword ptr [rsp + 0x30], xmm0
0x140f99b8d lea rax, [rip + 0x1c16bd2] ; 0x142bb0766 6d656e752f6e6f6b6e6f6b5f6c6f676f f32=3.022030789968193e+32 str=b'menu/noknok_logocrates/systems/toolbar/src/mei/config.rsNo UI template named . Available: \x9e\x07\xbbB\x01'
0x140f99b94 mov qword ptr [rsp + 0x20], rax
0x140f99b99 mov qword ptr [rsp + 0x28], 0x10
0x140f99ba2 lea r8, [rip + 0x1c16bb7] ; 0x142bb0760 6e6f6b6e6f6b6d656e752f6e6f6b6e6f f32=1.8215921883576608e+28 str=b'noknokmenu/noknok_logocrates/systems/toolbar/src/mei/config.rsNo UI template named . Available: \x9e\x07\xbbB'
0x140f99ba9 lea rcx, [rsp + 0x38]
0x140f99bae lea rdx, [rsp + 0x50]
0x140f99bb3 mov r9d, 6
0x140f99bb9 call 0x140f9fca0
0x140f99bbe mov rbx, qword ptr [rsp + 0x38]
0x140f99bc3 cmp qword ptr [rbx + 0x1b8], 0
0x140f99bcb je 0x140f99d84
0x140f99bd1 lea rdx, [rsp + 0x40]
0x140f99bd6 lea rcx, [rbx + 0x1c0]
0x140f99bdd call 0x1405b0d70
0x140f99be2 mov rcx, qword ptr [rbx + 0x1a0]
0x140f99be9 mov rdx, qword ptr [rbx + 0x1a8]
0x140f99bf0 mov r8, rdx
0x140f99bf3 and r8, rax
0x140f99bf6 shr rax, 0x39
0x140f99bfa movd xmm0, eax
0x140f99bfe punpcklbw xmm0, xmm0
0x140f99c02 pshuflw xmm0, xmm0, 0
0x140f99c07 pshufd xmm0, xmm0, 0
0x140f99c0c lea rax, [rcx - 0xc0]
0x140f99c13 xor r9d, r9d
0x140f99c16 mov r10, qword ptr [rsp + 0x40]
0x140f99c1b pcmpeqd xmm1, xmm1
0x140f99c1f movdqu xmm2, xmmword ptr [rcx + r8]
0x140f99c25 movdqa xmm3, xmm2
0x140f99c29 pcmpeqb xmm3, xmm0
0x140f99c2d pmovmskb r11d, xmm3
0x140f99c32 test r11d, r11d
0x140f99c35 je 0x140f99c60
0x140f99c37 tzcnt ebx, r11d
0x140f99c3c add rbx, r8
0x140f99c3f and rbx, rdx
0x140f99c42 neg rbx
0x140f99c45 lea rbx, [rbx + rbx*2]
0x140f99c49 shl rbx, 6
0x140f99c4d cmp r10, qword ptr [rax + rbx]
0x140f99c51 je 0x140f99c82
0x140f99c53 lea ebx, [r11 - 1]
0x140f99c57 and bx, r11w
0x140f99c5b mov r11d, ebx
0x140f99c5e jne 0x140f99c37
0x140f99c60 pcmpeqb xmm2, xmm1
0x140f99c64 pmovmskb r11d, xmm2
0x140f99c69 test r11d, r11d
0x140f99c6c jne 0x140f99d84
0x140f99c72 add r8, r9
0x140f99c75 add r8, 0x10
0x140f99c79 add r9, 0x10
0x140f99c7d and r8, rdx
0x140f99c80 jmp 0x140f99c1f
0x140f99c82 or byte ptr [rcx + rbx - 1], 2
0x140f99c87 mulss xmm6, dword ptr [rip + 0x198bce1] ; 0x142925970 cdcccc3e5d8f423fc3f5283fffff7fff f32=0.4000000059604645 str=b'\xcd\xcc\xcc>]\x8fB?\xc3\xf5(?\xff\xff\x7f\xff\xff\xff\x7f\x7f\xcd\xcc\xcc='
0x140f99c8f movss dword ptr [rsp + 0x30], xmm6
0x140f99c95 lea rax, [rip + 0x1c16a7d] ; 0x142bb0719 6d656e752f666d6f645f6c6f676f706f f32=3.022030789968193e+32 str=b'menu/fmod_logopounce lightmenu/pounce_light_logopad0heartmenu/heartpad1noknokmenu/noknok_logocrates/'
0x140f99c9c mov qword ptr [rsp + 0x20], rax
0x140f99ca1 mov qword ptr [rsp + 0x28], 0xe
0x140f99caa lea r8, [rip + 0x1c16a64] ; 0x142bb0715 666d6f646d656e752f666d6f645f6c6f f32=1.7666619287257366e+22 str=b'fmodmenu/fmod_logopounce lightmenu/pounce_light_logopad0heartmenu/heartpad1noknokmenu/noknok_logocra'
0x140f99cb1 lea rcx, [rsp + 0x38]
0x140f99cb6 mov r9d, 4
0x140f99cbc mov rdx, rdi
0x140f99cbf call 0x140f9fca0
0x140f99cc4 mov rbx, qword ptr [rsp + 0x38]
0x140f99cc9 cmp qword ptr [rbx + 0x1b8], 0
0x140f99cd1 je 0x140f99d84
0x140f99cd7 lea rdx, [rsp + 0x40]
0x140f99cdc lea rcx, [rbx + 0x1c0]
0x140f99ce3 call 0x1405b0d70
0x140f99ce8 mov rcx, qword ptr [rbx + 0x1a0]
0x140f99cef mov rdx, qword ptr [rbx + 0x1a8]
0x140f99cf6 mov r8, rdx
0x140f99cf9 and r8, rax
0x140f99cfc shr rax, 0x39
0x140f99d00 movd xmm0, eax
0x140f99d04 punpcklbw xmm0, xmm0
0x140f99d08 pshuflw xmm0, xmm0, 0
0x140f99d0d pshufd xmm0, xmm0, 0
0x140f99d12 lea rax, [rcx - 0xc0]
0x140f99d19 xor r9d, r9d
0x140f99d1c mov r10, qword ptr [rsp + 0x40]
0x140f99d21 pcmpeqd xmm1, xmm1
0x140f99d25 movdqu xmm2, xmmword ptr [rcx + r8]
0x140f99d2b movdqa xmm3, xmm2
0x140f99d2f pcmpeqb xmm3, xmm0
0x140f99d33 pmovmskb r11d, xmm3
0x140f99d38 test r11d, r11d
0x140f99d3b je 0x140f99d66
0x140f99d3d tzcnt ebx, r11d
0x140f99d42 add rbx, r8
0x140f99d45 and rbx, rdx
0x140f99d48 neg rbx
0x140f99d4b lea rbx, [rbx + rbx*2]
0x140f99d4f shl rbx, 6
0x140f99d53 cmp r10, qword ptr [rax + rbx]
0x140f99d57 je 0x140f99d90
0x140f99d59 lea ebx, [r11 - 1]
0x140f99d5d and bx, r11w
0x140f99d61 mov r11d, ebx
0x140f99d64 jne 0x140f99d3d
0x140f99d66 pcmpeqb xmm2, xmm1
0x140f99d6a pmovmskb r11d, xmm2
0x140f99d6f test r11d, r11d
0x140f99d72 jne 0x140f99d84
0x140f99d74 add r8, r9
0x140f99d77 add r8, 0x10
0x140f99d7b add r9, 0x10
0x140f99d7f and r8, rdx
0x140f99d82 jmp 0x140f99d25
0x140f99d84 lea rcx, [rip + 0x1a7e265] ; 0x142a17ff0 f87ba142010000002600000000000000 f32=80.74212646484375 str=b'\xf8{\xa1B\x01'
0x140f99d8b call 0x1428d9310
0x140f99d90 or byte ptr [rcx + rbx - 1], 2
0x140f99d95 mov rax, qword ptr [rdi + 0x10]
0x140f99d99 mov qword ptr [rsi + 0x10], rax
0x140f99d9d movups xmm0, xmmword ptr [rdi]
0x140f99da0 movups xmmword ptr [rsi], xmm0
0x140f99da3 mov rax, rsi
0x140f99da6 movaps xmm6, xmmword ptr [rsp + 0x80]
0x140f99dae movaps xmm7, xmmword ptr [rsp + 0x90]
0x140f99db6 add rsp, 0xa8
0x140f99dbd pop rbx
0x140f99dbe pop rbp
0x140f99dbf pop rdi
0x140f99dc0 pop rsi
0x140f99dc1 pop r14
0x140f99dc3 pop r15
0x140f99dc5 ret 
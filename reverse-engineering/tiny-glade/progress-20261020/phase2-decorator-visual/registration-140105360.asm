140105360 push rbp
140105361 push rsi
140105362 push rdi
140105363 push rbx
140105364 sub rsp, 0x2f8
14010536b lea rbp, [rsp + 0x80]
140105373 mov qword ptr [rbp + 0x270], 0xfffffffffffffffe
14010537e mov rsi, rcx
140105381 movzx eax, byte ptr [rip + 0x352f881]
140105388 mov ecx, 0x1c0
14010538d mov edx, 8
140105392 call 0x140613c10
140105397 test rax, rax
14010539a je 0x140105747
1401053a0 mov qword ptr [rbp + 0x268], rax
1401053a7 xorps xmm0, xmm0
1401053aa movups xmmword ptr [rbp + 0x148], xmm0
1401053b1 movups xmmword ptr [rbp + 0x160], xmm0
1401053b8 movups xmmword ptr [rbp + 0x178], xmm0
1401053bf movups xmmword ptr [rbp + 0x190], xmm0
1401053c6 movups xmmword ptr [rbp + 0x1a8], xmm0
1401053cd mov dword ptr [rbp + 0x1b8], 0
1401053d7 movups xmmword ptr [rbp + 0x1e0], xmm0
1401053de movups xmmword ptr [rbp + 0x1f8], xmm0
1401053e5 movups xmmword ptr [rbp + 0x210], xmm0
1401053ec movups xmmword ptr [rbp + 0x228], xmm0
1401053f3 movups xmmword ptr [rbp + 0x240], xmm0
1401053fa mov dword ptr [rbp + 0x250], 0
140105404 mov qword ptr [rbp + 0xf0], 0
14010540f mov qword ptr [rbp + 0x128], 0
14010541a mov qword ptr [rbp + 0x130], 8
140105425 mov qword ptr [rbp + 0x138], 0
140105430 mov qword ptr [rbp + 0x140], 0x10
14010543b mov qword ptr [rbp + 0x158], 0x10
140105446 mov qword ptr [rbp + 0x170], 0x10
140105451 mov qword ptr [rbp + 0x188], 0x10
14010545c mov qword ptr [rbp + 0x1a0], 0x10
140105467 movabs rbx, 0x8000000000000000
140105471 mov qword ptr [rbp + 0x1c0], rbx
140105478 lea rax, [rip + 0x2848bfa]
14010547f mov qword ptr [rbp + 0x1c8], rax
140105486 mov qword ptr [rbp + 0x1d0], 0x43
140105491 mov qword ptr [rbp + 0x1d8], 0x10
14010549c mov qword ptr [rbp + 0x1f0], 0x10
1401054a7 mov qword ptr [rbp + 0x208], 0x10
1401054b2 mov qword ptr [rbp + 0x220], 0x10
1401054bd mov qword ptr [rbp + 0x238], 0x10
1401054c8 mov dword ptr [rbp + 0x258], 0
1401054d2 mov word ptr [rbp + 0x25c], 1
1401054db mov dword ptr [rbp + 0x260], 0
1401054e5 movzx eax, byte ptr [rip + 0x352f71d]
1401054ec mov ecx, 0x178
1401054f1 mov edx, 8
1401054f6 call 0x140613c10
1401054fb test rax, rax
1401054fe je 0x140105756
140105504 mov rdi, rax
140105507 lea rdx, [rbp + 0xf0]
14010550e mov r8d, 0x178
140105514 mov rcx, rax
140105517 call 0x142923c00
14010551c lea r8, [rip + 0x28489e5]
140105523 lea rcx, [rbp + 0x80]
14010552a mov rdx, rdi
14010552d call 0x140ce4b10
140105532 lea rcx, [rbp + 0x10]
140105536 call 0x1400c5cc0
14010553b lea rcx, [rbp - 0x60]
14010553f call 0x14037d280
140105544 lea rcx, [rbp + 0xf0]
14010554b call 0x1400c6200
140105550 movups xmm0, xmmword ptr [rbp + 0xe0]
140105557 mov rax, qword ptr [rbp + 0x268]
14010555e movups xmmword ptr [rax + 0x60], xmm0
140105562 movups xmm0, xmmword ptr [rbp + 0xd0]
140105569 movups xmmword ptr [rax + 0x50], xmm0
14010556d movups xmm0, xmmword ptr [rbp + 0xc0]
140105574 movups xmmword ptr [rax + 0x40], xmm0
140105578 movups xmm0, xmmword ptr [rbp + 0x80]
14010557f movups xmm1, xmmword ptr [rbp + 0x90]
140105586 movups xmm2, xmmword ptr [rbp + 0xa0]
14010558d movups xmm3, xmmword ptr [rbp + 0xb0]
140105594 movups xmmword ptr [rax + 0x30], xmm3
140105598 movups xmmword ptr [rax + 0x20], xmm2
14010559c movups xmmword ptr [rax + 0x10], xmm1
1401055a0 movups xmmword ptr [rax], xmm0
1401055a3 movups xmm0, xmmword ptr [rbp + 0x10]
1401055a7 movups xmm1, xmmword ptr [rbp + 0x20]
1401055ab movups xmm2, xmmword ptr [rbp + 0x30]
1401055af movups xmm3, xmmword ptr [rbp + 0x40]
1401055b3 movups xmmword ptr [rax + 0x70], xmm0
1401055b7 movups xmmword ptr [rax + 0x80], xmm1
1401055be movups xmmword ptr [rax + 0x90], xmm2
1401055c5 movups xmmword ptr [rax + 0xa0], xmm3
1401055cc movups xmm0, xmmword ptr [rbp + 0x50]
1401055d0 movups xmmword ptr [rax + 0xb0], xmm0
1401055d7 movups xmm0, xmmword ptr [rbp + 0x60]
1401055db movups xmmword ptr [rax + 0xc0], xmm0
1401055e2 movups xmm0, xmmword ptr [rbp + 0x70]
1401055e6 movups xmmword ptr [rax + 0xd0], xmm0
1401055ed movups xmm0, xmmword ptr [rbp]
1401055f1 movups xmmword ptr [rax + 0x140], xmm0
1401055f8 movups xmm0, xmmword ptr [rbp - 0x10]
1401055fc movups xmmword ptr [rax + 0x130], xmm0
140105603 movups xmm0, xmmword ptr [rbp - 0x20]
140105607 movups xmmword ptr [rax + 0x120], xmm0
14010560e movups xmm0, xmmword ptr [rbp - 0x60]
140105612 movups xmm1, xmmword ptr [rbp - 0x50]
140105616 movups xmm2, xmmword ptr [rbp - 0x40]
14010561a movups xmm3, xmmword ptr [rbp - 0x30]
14010561e movups xmmword ptr [rax + 0x110], xmm3
140105625 movups xmmword ptr [rax + 0x100], xmm2
14010562c movups xmmword ptr [rax + 0xf0], xmm1
140105633 movups xmmword ptr [rax + 0xe0], xmm0
14010563a movups xmm0, xmmword ptr [rbp + 0xf0]
140105641 movups xmm1, xmmword ptr [rbp + 0x100]
140105648 movups xmm2, xmmword ptr [rbp + 0x110]
14010564f movups xmm3, xmmword ptr [rbp + 0x120]
140105656 movups xmmword ptr [rax + 0x150], xmm0
14010565d movups xmmword ptr [rax + 0x160], xmm1
140105664 movups xmmword ptr [rax + 0x170], xmm2
14010566b movups xmmword ptr [rax + 0x180], xmm3
140105672 movups xmm0, xmmword ptr [rbp + 0x130]
140105679 movups xmmword ptr [rax + 0x190], xmm0
140105680 movups xmm0, xmmword ptr [rbp + 0x140]
140105687 movups xmmword ptr [rax + 0x1a0], xmm0
14010568e movups xmm0, xmmword ptr [rbp + 0x150]
140105695 movups xmmword ptr [rax + 0x1b0], xmm0
14010569c mov qword ptr [rbp + 0xf8], 4
1401056a7 mov qword ptr [rbp + 0x100], rax
1401056ae mov qword ptr [rbp + 0x108], 4
1401056b9 mov qword ptr [rbp + 0x110], 0
1401056c4 mov qword ptr [rbp + 0x118], 8
1401056cf xorps xmm0, xmm0
1401056d2 movups xmmword ptr [rbp + 0x120], xmm0
1401056d9 mov qword ptr [rbp + 0xf0], rbx
1401056e0 lea rcx, [rbp + 0x128]
1401056e7 call 0x140d2fa20
1401056ec movups xmm0, xmmword ptr [rbp + 0x150]
1401056f3 movups xmmword ptr [rsi + 0x60], xmm0
1401056f7 movups xmm0, xmmword ptr [rbp + 0x140]
1401056fe movups xmmword ptr [rsi + 0x50], xmm0
140105702 movups xmm0, xmmword ptr [rbp + 0x130]
140105709 movups xmmword ptr [rsi + 0x40], xmm0
14010570d movups xmm0, xmmword ptr [rbp + 0xf0]
140105714 movups xmm1, xmmword ptr [rbp + 0x100]
14010571b movups xmm2, xmmword ptr [rbp + 0x110]
140105722 movups xmm3, xmmword ptr [rbp + 0x120]
140105729 movups xmmword ptr [rsi + 0x30], xmm3
14010572d movups xmmword ptr [rsi + 0x20], xmm2
140105731 movups xmmword ptr [rsi + 0x10], xmm1
140105735 movups xmmword ptr [rsi], xmm0
140105738 mov rax, rsi
14010573b add rsp, 0x2f8
140105742 pop rbx
140105743 pop rdi
140105744 pop rsi
140105745 pop rbp
140105746 ret 
140105747 mov ecx, 8
14010574c mov edx, 0x1c0
140105751 call 0x1428d9000
140105756 mov ecx, 8
14010575b mov edx, 0x178
140105760 call 0x1428d9000
140105765 ud2 
140105767 nop word ptr [rax + rax]
140105770 mov qword ptr [rsp + 0x10], rdx
140105775 push rbp
140105776 push rsi
140105777 push rdi
140105778 push rbx
140105779 sub rsp, 0x28
14010577d lea rbp, [rdx + 0x80]
140105784 lea rcx, [rbp + 0xf0]
14010578b call 0x1405e9af0
140105790 nop 
140105791 add rsp, 0x28
140105795 pop rbx
140105796 pop rdi
140105797 pop rsi
140105798 pop rbp
140105799 ret 
14010579a nop word ptr [rax + rax]
1401057a0 mov qword ptr [rsp + 0x10], rdx
1401057a5 push rbp
1401057a6 push rsi
1401057a7 push rdi
1401057a8 push rbx
1401057a9 sub rsp, 0x28
1401057ad lea rbp, [rdx + 0x80]
1401057b4 mov edx, 0x1c0
1401057b9 mov r8d, 8
1401057bf mov rcx, qword ptr [rbp + 0x268]
1401057c6 call 0x140613c20
1401057cb nop 
1401057cc add rsp, 0x28
1401057d0 pop rbx
1401057d1 pop rdi
1401057d2 pop rsi
1401057d3 pop rbp
1401057d4 ret 
1401057d5 nop word ptr cs:[rax + rax]
1401057e0 mov qword ptr [rsp + 0x10], rdx
1401057e5 push rbp
1401057e6 push rsi
1401057e7 push rdi
1401057e8 push rbx
1401057e9 sub rsp, 0x28
1401057ed lea rbp, [rdx + 0x80]
1401057f4 lea rcx, [rbp + 0x80]
1401057fb call 0x140029880
140105800 nop 
140105801 add rsp, 0x28
140105805 pop rbx
140105806 pop rdi
140105807 pop rsi
140105808 pop rbp
140105809 ret 
14010580a nop word ptr [rax + rax]
140105810 mov qword ptr [rsp + 0x10], rdx
140105815 push rbp
140105816 push rsi
140105817 push rdi
140105818 push rbx
140105819 sub rsp, 0x28
14010581d lea rbp, [rdx + 0x80]
140105824 lea rcx, [rbp + 0x10]
140105828 call 0x140029880
14010582d nop 
14010582e add rsp, 0x28
140105832 pop rbx
140105833 pop rdi
140105834 pop rsi
140105835 pop rbp
140105836 ret 
140105837 nop word ptr [rax + rax]
140105840 mov qword ptr [rsp + 0x10], rdx
140105845 push rbp
140105846 push rsi
140105847 push rdi
140105848 push rbx
140105849 sub rsp, 0x28
14010584d lea rbp, [rdx + 0x80]
140105854 lea rcx, [rbp - 0x60]
140105858 call 0x140029880
14010585d nop 
14010585e add rsp, 0x28
140105862 pop rbx
140105863 pop rdi
140105864 pop rsi
140105865 pop rbp
140105866 ret 
140105867 nop word ptr [rax + rax]
140105870 mov qword ptr [rsp + 0x10], rdx
140105875 push rbp
140105876 push rsi
140105877 push rdi
140105878 push rbx
140105879 sub rsp, 0x28
14010587d lea rbp, [rdx + 0x80]
140105884 lea rcx, [rbp + 0xf0]
14010588b call 0x140029880
140105890 nop 
140105891 add rsp, 0x28
140105895 pop rbx
140105896 pop rdi
140105897 pop rsi
140105898 pop rbp
140105899 ret 
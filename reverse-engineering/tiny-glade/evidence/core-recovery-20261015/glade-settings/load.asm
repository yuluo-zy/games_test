0x1408bbbc0  push     rbp
0x1408bbbc1  push     rsi
0x1408bbbc2  push     rbx
0x1408bbbc3  sub      rsp, 0xb0
0x1408bbbca  lea      rbp, [rsp + 0x80]
0x1408bbbd2  mov      qword ptr [rbp + 0x28], 0xfffffffffffffffe
0x1408bbbda  mov      rsi, rcx
0x1408bbbdd  mov      qword ptr [rbp + 8], rdx
0x1408bbbe1  mov      rcx, rdx
0x1408bbbe4  call     0x140d76ed0
0x1408bbbe9  mov      r8, rdx
0x1408bbbec  mov      qword ptr [rsp + 0x20], 0xd
0x1408bbbf5  lea      r9, [rip + 0x21e4d8c]
0x1408bbbfc  lea      rcx, [rbp - 0x30]
0x1408bbc00  mov      rdx, rax
0x1408bbc03  call     0x140d62810
0x1408bbc08  lea      rcx, [rbp - 0x30]
0x1408bbc0c  call     0x140d76ed0
0x1408bbc11  mov      r8, rdx
0x1408bbc14  lea      rcx, [rbp - 0x10]
0x1408bbc18  mov      rdx, rax
0x1408bbc1b  call     0x140d59140
0x1408bbc20  mov      rdx, qword ptr [rbp - 0x30]
0x1408bbc24  test     rdx, rdx
0x1408bbc27  je       0x1408bbc38
0x1408bbc29  mov      rcx, qword ptr [rbp - 0x28]
0x1408bbc2d  mov      r8d, 1
0x1408bbc33  call     0x140613c20
0x1408bbc38  mov      rcx, qword ptr [rbp - 0x10]
0x1408bbc3c  mov      rax, rcx
0x1408bbc3f  neg      rax
0x1408bbc42  jno      0x1408bbcaa
0x1408bbc44  mov      rax, qword ptr [rbp - 8]
0x1408bbc48  mov      ecx, eax
0x1408bbc4a  and      ecx, 3
0x1408bbc4d  cmp      ecx, 1
0x1408bbc50  jne      0x1408bbdb9
0x1408bbc56  lea      rcx, [rax - 1]
0x1408bbc5a  mov      qword ptr [rbp + 0x18], rcx
0x1408bbc5e  mov      rcx, qword ptr [rax - 1]
0x1408bbc62  mov      qword ptr [rbp + 0x20], rcx
0x1408bbc66  mov      rax, qword ptr [rax + 7]
0x1408bbc6a  mov      qword ptr [rbp + 0x10], rax
0x1408bbc6e  mov      rax, qword ptr [rax]
0x1408bbc71  test     rax, rax
0x1408bbc74  je       0x1408bbc7c
0x1408bbc76  mov      rcx, qword ptr [rbp + 0x20]
0x1408bbc7a  call     rax
0x1408bbc7c  mov      rcx, qword ptr [rbp + 0x20]
0x1408bbc80  mov      rax, qword ptr [rbp + 0x10]
0x1408bbc84  mov      rdx, qword ptr [rax + 8]
0x1408bbc88  test     rdx, rdx
0x1408bbc8b  je       0x1408bbc96
0x1408bbc8d  mov      r8, qword ptr [rax + 0x10]
0x1408bbc91  call     0x140613c20
0x1408bbc96  mov      edx, 0x18
0x1408bbc9b  mov      r8d, 8
0x1408bbca1  mov      rcx, qword ptr [rbp + 0x18]
0x1408bbca5  jmp      0x1408bbdb4
0x1408bbcaa  mov      qword ptr [rbp + 0x20], rcx
0x1408bbcae  mov      rdx, qword ptr [rbp - 8]
0x1408bbcb2  mov      r8, qword ptr [rbp]
0x1408bbcb6  lea      rcx, [rbp - 0x58]
0x1408bbcba  mov      qword ptr [rbp + 0x10], rdx
0x1408bbcbe  call     0x1414057b0
0x1408bbcc3  movzx    ebx, byte ptr [rbp - 0x38]
0x1408bbcc7  cmp      bl, 3
0x1408bbcca  jne      0x1408bbd10
0x1408bbccc  mov      rcx, qword ptr [rbp - 0x58]
0x1408bbcd0  mov      rax, qword ptr [rcx]
0x1408bbcd3  cmp      rax, 1
0x1408bbcd7  mov      qword ptr [rbp + 0x18], rcx
0x1408bbcdb  je       0x1408bbd84
0x1408bbce1  test     rax, rax
0x1408bbce4  jne      0x1408bbd8d
0x1408bbcea  mov      rax, qword ptr [rbp + 0x18]
0x1408bbcee  mov      rdx, qword ptr [rax + 0x10]
0x1408bbcf2  test     rdx, rdx
0x1408bbcf5  je       0x1408bbd8d
0x1408bbcfb  mov      rax, qword ptr [rbp + 0x18]
0x1408bbcff  mov      rcx, qword ptr [rax + 8]
0x1408bbd03  mov      r8d, 1
0x1408bbd09  call     0x140613c20
0x1408bbd0e  jmp      0x1408bbd8d
0x1408bbd10  movups   xmm0, xmmword ptr [rbp - 0x58]
0x1408bbd14  movups   xmm1, xmmword ptr [rbp - 0x48]
0x1408bbd18  movaps   xmmword ptr [rbp - 0x20], xmm1
0x1408bbd1c  movaps   xmmword ptr [rbp - 0x30], xmm0
0x1408bbd20  mov      eax, dword ptr [rbp - 0x37]
0x1408bbd23  mov      dword ptr [rbp - 0x10], eax
0x1408bbd26  mov      eax, dword ptr [rbp - 0x34]
0x1408bbd29  mov      dword ptr [rbp - 0xd], eax
0x1408bbd2c  mov      rdx, qword ptr [rbp + 0x20]
0x1408bbd30  test     rdx, rdx
0x1408bbd33  je       0x1408bbd44
0x1408bbd35  mov      r8d, 1
0x1408bbd3b  mov      rcx, qword ptr [rbp + 0x10]
0x1408bbd3f  call     0x140613c20
0x1408bbd44  mov      rax, qword ptr [rbp + 8]
0x1408bbd48  mov      rdx, qword ptr [rax]
0x1408bbd4b  test     rdx, rdx
0x1408bbd4e  je       0x1408bbd5f
0x1408bbd50  mov      rcx, qword ptr [rax + 8]
0x1408bbd54  mov      r8d, 1
0x1408bbd5a  call     0x140613c20
0x1408bbd5f  cmp      bl, 2
0x1408bbd62  je       0x1408bbdd4
0x1408bbd64  movaps   xmm0, xmmword ptr [rbp - 0x30]
0x1408bbd68  movaps   xmm1, xmmword ptr [rbp - 0x20]
0x1408bbd6c  movups   xmmword ptr [rsi + 0x10], xmm1
0x1408bbd70  movups   xmmword ptr [rsi], xmm0
0x1408bbd73  mov      byte ptr [rsi + 0x20], bl
0x1408bbd76  mov      eax, dword ptr [rbp - 0x10]
0x1408bbd79  mov      ecx, dword ptr [rbp - 0xd]
0x1408bbd7c  mov      dword ptr [rsi + 0x21], eax
0x1408bbd7f  mov      dword ptr [rsi + 0x24], ecx
0x1408bbd82  jmp      0x1408bbe02
0x1408bbd84  add      rcx, 8
0x1408bbd88  call     0x1405e7a00
0x1408bbd8d  mov      edx, 0x28
0x1408bbd92  mov      r8d, 8
0x1408bbd98  mov      rcx, qword ptr [rbp + 0x18]
0x1408bbd9c  call     0x140613c20
0x1408bbda1  mov      rdx, qword ptr [rbp + 0x20]
0x1408bbda5  test     rdx, rdx
0x1408bbda8  je       0x1408bbdb9
0x1408bbdaa  mov      r8d, 1
0x1408bbdb0  mov      rcx, qword ptr [rbp + 0x10]
0x1408bbdb4  call     0x140613c20
0x1408bbdb9  mov      rax, qword ptr [rbp + 8]
0x1408bbdbd  mov      rdx, qword ptr [rax]
0x1408bbdc0  test     rdx, rdx
0x1408bbdc3  je       0x1408bbdd4
0x1408bbdc5  mov      rcx, qword ptr [rax + 8]
0x1408bbdc9  mov      r8d, 1
0x1408bbdcf  call     0x140613c20
0x1408bbdd4  mov      qword ptr [rsi], 0
0x1408bbddb  movsd    xmm0, qword ptr [rip + 0x21e439d]
0x1408bbde3  movsd    qword ptr [rsi + 8], xmm0
0x1408bbde8  mov      byte ptr [rsi + 0x20], 1
0x1408bbdec  mov      qword ptr [rsi + 0x10], 8
0x1408bbdf4  movabs   rax, 0x100000000
0x1408bbdfe  mov      qword ptr [rsi + 0x18], rax
0x1408bbe02  mov      rax, rsi

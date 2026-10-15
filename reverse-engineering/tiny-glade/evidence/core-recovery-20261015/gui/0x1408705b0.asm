0x1408705b0 mov      rax, rcx
0x1408705b3 cmp      byte ptr [rip + 0x2dc47d6], 0
0x1408705ba jne      0x1408705c6
0x1408705bc movss    xmm0, dword ptr [rip + 0x215cab4]
0x1408705c4 jmp      0x1408705ce ; 
0x1408705c6 movss    xmm0, dword ptr [rip + 0x20b5336]
0x1408705ce movss    dword ptr [rax], xmm0
0x1408705d2 mov      word ptr [rax + 0xc], 0x100
0x1408705d8 mov      ecx, 0xffffffff
0x1408705dd mov      qword ptr [rax + 4], rcx
0x1408705e1 ret      
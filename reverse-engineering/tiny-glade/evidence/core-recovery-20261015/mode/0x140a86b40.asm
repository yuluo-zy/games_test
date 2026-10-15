0x140a86b40 mov      ecx, dword ptr [rcx]
0x140a86b42 lea      eax, [rcx - 6]
0x140a86b45 cmp      eax, 0xf
0x140a86b48 setb     dl
0x140a86b4b add      cl, 0xfb
0x140a86b4e mov      eax, 0x3cfe
0x140a86b53 shr      eax, cl
0x140a86b55 and      al, dl
0x140a86b57 ret      
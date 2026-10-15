0x140a826d0 mov      rax, qword ptr [rcx]
0x140a826d3 lock dec qword ptr [rax]
0x140a826d7 jne      0x140a826de
0x140a826d9 jmp      0x140b5cfc0 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::hbcb6927a13bbf4f2
0x140a826de ret      
0x140a826e0 mov      rax, qword ptr [rcx]
0x140a826e3 lock dec qword ptr [rax]
0x140a826e7 jne      0x140a826ee
0x140a826e9 jmp      0x140b5c2a0 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h28f4482cd6b59500
0x140a826ee ret      
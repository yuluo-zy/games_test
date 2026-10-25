0x140a826f0 mov      rax, qword ptr [rcx]
0x140a826f3 lock dec qword ptr [rax]
0x140a826f7 jne      0x140a826fe
0x140a826f9 jmp      0x1402e5d80 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h359ec8eb5ea8a918 / alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::hc8cf01ba4f172df6
0x140a826fe ret      
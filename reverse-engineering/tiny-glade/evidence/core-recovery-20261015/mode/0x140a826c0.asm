0x140a826c0 mov      rax, qword ptr [rcx]
0x140a826c3 lock dec qword ptr [rax]
0x140a826c7 jne      0x140a826ce
0x140a826c9 jmp      0x140b5d010 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::hbe49ad65a4eee4c5 / alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::hf8b9db1fc7c70ae8
0x140a826ce ret      
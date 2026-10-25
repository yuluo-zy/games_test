0x140a82700 mov      rax, qword ptr [rcx]
0x140a82703 lock dec qword ptr [rax]
0x140a82707 jne      0x140a8270e
0x140a82709 jmp      0x1406978e0 ; _$LT$alloc..sync..Weak$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop::h0d608b4fde675181 / alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::hda2fa96c289317cd
0x140a8270e ret      
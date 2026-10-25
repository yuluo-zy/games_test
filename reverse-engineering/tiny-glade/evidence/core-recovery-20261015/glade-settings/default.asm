0x1408c2240  mov      rax, rcx
0x1408c2243  mov      qword ptr [rcx], 0
0x1408c224a  movsd    xmm0, qword ptr [rip + 0x21ddf2e]
0x1408c2252  movsd    qword ptr [rcx + 8], xmm0
0x1408c2257  mov      byte ptr [rcx + 0x20], 1
0x1408c225b  mov      qword ptr [rcx + 0x10], 8
0x1408c2263  movabs   rcx, 0x100000000
0x1408c226d  mov      qword ptr [rax + 0x18], rcx
0x1408c2271  ret      

0x1416ebe70 56 push rsi
0x1416ebe71 4883ec20 sub rsp, 0x20
0x1416ebe75 4889ce mov rsi, rcx
0x1416ebe78 4889d1 mov rcx, rdx
0x1416ebe7b 4c89c2 mov rdx, r8
0x1416ebe7e e83ddd2001 call 0x1428f9bc0
0x1416ebe83 48894608 mov qword ptr [rsi + 8], rax
0x1416ebe87 c70601000000 mov dword ptr [rsi], 1
0x1416ebe8d 4889f0 mov rax, rsi
0x1416ebe90 4883c420 add rsp, 0x20
0x1416ebe94 5e pop rsi
0x1416ebe95 c3 ret 

140aa6600: 4889c8                       mov rax, rcx
140aa6603: f20f124250                   movddup xmm0, qword ptr [rdx + 0x50]
140aa6608: 0f590551f7ff01               mulps xmm0, xmmword ptr [rip + 0x1fff751]
140aa660f: 0f1101                       movups xmmword ptr [rcx], xmm0
140aa6612: c3                           ret 

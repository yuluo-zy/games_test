140c98140: 4889c8                       mov rax, rcx
140c98143: 0f1002                       movups xmm0, xmmword ptr [rdx]
140c98146: f20f100a                     movsd xmm1, qword ptr [rdx]
140c9814a: f20f12d1                     movddup xmm2, xmm1
140c9814e: 0fc6c0aa                     shufps xmm0, xmm0, 0xaa
140c98152: 0f5cc8                       subps xmm1, xmm0
140c98155: 0f58c2                       addps xmm0, xmm2
140c98158: f20f10c1                     movsd xmm0, xmm1
140c9815c: 660f1101                     movupd xmmword ptr [rcx], xmm0
140c98160: c3                           ret 

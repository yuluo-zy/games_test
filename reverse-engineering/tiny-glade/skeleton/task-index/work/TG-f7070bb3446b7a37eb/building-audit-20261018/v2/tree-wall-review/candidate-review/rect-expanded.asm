140c9a730: 4889c8                       mov rax, rcx
140c9a733: f20f104210                   movsd xmm0, qword ptr [rdx + 0x10]
140c9a738: 0f14d3                       unpcklps xmm2, xmm3
140c9a73b: 0f58d0                       addps xmm2, xmm0
140c9a73e: 0f135110                     movlps qword ptr [rcx + 0x10], xmm2
140c9a742: 0f1002                       movups xmm0, xmmword ptr [rdx]
140c9a745: 0f1101                       movups xmmword ptr [rcx], xmm0
140c9a748: c3                           ret 
140c9a749: cc                           int3 
140c9a74a: cc                           int3 
140c9a74b: cc                           int3 
140c9a74c: cc                           int3 
140c9a74d: cc                           int3 
140c9a74e: cc                           int3 
140c9a74f: cc                           int3 
140c9a750: 4883ec38                     sub rsp, 0x38
140c9a754: 0f29742420                   movaps xmmword ptr [rsp + 0x20], xmm6
140c9a759: f20f1019                     movsd xmm3, qword ptr [rcx]

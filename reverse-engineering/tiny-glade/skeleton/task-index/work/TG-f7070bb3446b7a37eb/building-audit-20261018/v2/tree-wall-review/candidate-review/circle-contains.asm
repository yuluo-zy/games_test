140c980f0: f20f1001                     movsd xmm0, qword ptr [rcx]
140c980f4: 0f14ca                       unpcklps xmm1, xmm2
140c980f7: 0f5cc8                       subps xmm1, xmm0
140c980fa: 0f59c9                       mulps xmm1, xmm1
140c980fd: f30f16c1                     movshdup xmm0, xmm1
140c98101: f30f58c1                     addss xmm0, xmm1
140c98105: f30f104908                   movss xmm1, dword ptr [rcx + 8]
140c9810a: f30f59c9                     mulss xmm1, xmm1
140c9810e: 0f2ec8                       ucomiss xmm1, xmm0
140c98111: 0f97c0                       seta al
140c98114: c3                           ret 
140c98115: cc                           int3 
140c98116: cc                           int3 
140c98117: cc                           int3 
140c98118: cc                           int3 
140c98119: cc                           int3 
140c9811a: cc                           int3 
140c9811b: cc                           int3 
140c9811c: cc                           int3 
140c9811d: cc                           int3 
140c9811e: cc                           int3 
140c9811f: cc                           int3 

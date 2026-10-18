140c9cde0: mov rax, rcx
140c9cde3: movss xmm0, dword ptr [rdx]
140c9cde7: movss xmm1, dword ptr [rdx + 4]
140c9cdec: movss dword ptr [rcx], xmm0
140c9cdf0: movss dword ptr [rcx + 4], xmm2
140c9cdf5: movss dword ptr [rcx + 8], xmm1
140c9cdfa: ret 

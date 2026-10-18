140c9cdc0: mov rax, rcx
140c9cdc3: movss xmm0, dword ptr [rdx]
140c9cdc7: movss xmm1, dword ptr [rdx + 4]
140c9cdcc: movss dword ptr [rcx], xmm0
140c9cdd0: mov dword ptr [rcx + 4], 0
140c9cdd7: movss dword ptr [rcx + 8], xmm1
140c9cddc: ret 

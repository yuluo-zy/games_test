140b2a740: 31c0 xor eax, eax
140b2a742: 83b95802000002 cmp dword ptr [rcx + 0x258], 2
140b2a749: 0f95c0 setne al
140b2a74c: f30f10818c020000 movss xmm0, dword ptr [rcx + 0x28c]
140b2a754: c3 ret 

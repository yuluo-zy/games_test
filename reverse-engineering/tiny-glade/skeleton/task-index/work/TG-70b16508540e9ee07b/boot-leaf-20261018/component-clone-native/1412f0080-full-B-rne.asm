1412f0080: 488b4108 mov rax, qword ptr [rcx + 8]
1412f0084: 48833800 cmp qword ptr [rax], 0
1412f0088: 742f je 0x1412f00b9
1412f008a: 49b89ca7c6dc7b709a92 movabs r8, 0x929a707bdcc6a79c
1412f0094: 4c394010 cmp qword ptr [rax + 0x10], r8
1412f0098: 751f jne 0x1412f00b9
1412f009a: 49b8e5c4d1de6e5c3a98 movabs r8, 0x983a5c6eded1c4e5
1412f00a4: 4c394008 cmp qword ptr [rax + 8], r8
1412f00a8: 750f jne 0x1412f00b9
1412f00aa: 488b01 mov rax, qword ptr [rcx]
1412f00ad: 8b00 mov eax, dword ptr [rax]
1412f00af: 4889d1 mov rcx, rdx
1412f00b2: 89c2 mov edx, eax
1412f00b4: e9377c4d00 jmp 0x1417c7cf0
1412f00b9: c3 ret 

140a84260: 4889c8  mov rax, rcx
140a84263: 8b4a08  mov ecx, dword ptr [rdx + 8]
140a84266: 488b12  mov rdx, qword ptr [rdx]
140a84269: 4c8b4210  mov r8, qword ptr [rdx + 0x10]
140a8426d: ba01000000  mov edx, 1
140a84272: 4983f802  cmp r8, 2
140a84276: 7209  jb 0x140a84281
140a84278: 4c894008  mov qword ptr [rax + 8], r8
140a8427c: 894810  mov dword ptr [rax + 0x10], ecx
140a8427f: 31d2  xor edx, edx
140a84281: 488910  mov qword ptr [rax], rdx
140a84284: c3  ret

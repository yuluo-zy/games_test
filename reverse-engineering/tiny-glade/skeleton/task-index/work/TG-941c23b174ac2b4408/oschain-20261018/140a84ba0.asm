140a84ba0: push rsi
140a84ba1: sub rsp, 0x90
140a84ba8: mov rsi, rcx
140a84bab: lea rax, [rdx + 0x68]
140a84baf: mov ecx, dword ptr [rdx + 0x10]
140a84bb2: dec ecx
140a84bb4: mov dword ptr [rsp + 0x2c], ecx
140a84bb8: add rdx, 0x6c
140a84bbc: mov qword ptr [rsp + 0x30], rax
140a84bc1: lea rax, [rip + 0xad0c8]
140a84bc8: mov qword ptr [rsp + 0x38], rax
140a84bcd: lea rcx, [rsp + 0x2c]
140a84bd2: mov qword ptr [rsp + 0x40], rcx
140a84bd7: lea rcx, [rip + 0x3089a2]
140a84bde: mov qword ptr [rsp + 0x48], rcx
140a84be3: mov qword ptr [rsp + 0x50], rdx
140a84be8: mov qword ptr [rsp + 0x58], rax
140a84bed: lea rax, [rip + 0x20534dc]
140a84bf4: mov qword ptr [rsp + 0x60], rax
140a84bf9: mov qword ptr [rsp + 0x68], 3
140a84c02: lea rax, [rsp + 0x30]
140a84c07: mov qword ptr [rsp + 0x70], rax
140a84c0c: mov qword ptr [rsp + 0x78], 3
140a84c15: mov qword ptr [rsp + 0x80], 0
140a84c21: lea rdx, [rsp + 0x60]
140a84c26: mov rcx, rsi
140a84c29: call 0x140d79f90
140a84c2e: mov rax, rsi
140a84c31: add rsp, 0x90
140a84c38: pop rsi
140a84c39: ret 

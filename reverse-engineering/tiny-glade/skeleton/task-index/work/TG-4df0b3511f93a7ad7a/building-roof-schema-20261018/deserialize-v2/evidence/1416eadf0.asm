0x1416eadf0 4889c8 mov rax, rcx
0x1416eadf3 4983c0fa add r8, -6
0x1416eadf7 4983f809 cmp r8, 9
0x1416eadfb 0f8771010000 ja 0x1416eaf72
0x1416eae01 488d0dd04f5a01 lea rcx, [rip + 0x15a4fd0]
0x1416eae08 4e630481 movsxd r8, dword ptr [rcx + r8*4]
0x1416eae0c 4901c8 add r8, rcx
0x1416eae0f 41ffe0 jmp r8
0x1416eae12 b968656967 mov ecx, 0x67696568
0x1416eae17 330a xor ecx, dword ptr [rdx]
0x1416eae19 0fb75204 movzx edx, word ptr [rdx + 4]
0x1416eae1d 81f268740000 xor edx, 0x7468
0x1416eae23 09ca or edx, ecx
0x1416eae25 0f8547010000 jne 0x1416eaf72
0x1416eae2b e955010000 jmp 0x1416eaf85
0x1416eae30 48b9656176655f6c656e movabs rcx, 0x6e656c5f65766165
0x1416eae3a 48330a xor rcx, qword ptr [rdx]
0x1416eae3d 49b8656e6774685f3031 movabs r8, 0x31305f6874676e65
0x1416eae47 4c334206 xor r8, qword ptr [rdx + 6]
0x1416eae4b 4909c8 or r8, rcx
0x1416eae4e 0f8439010000 je 0x1416eaf8d
0x1416eae54 49b8666c61745f72696d movabs r8, 0x6d69725f74616c66
0x1416eae5e 4c3302 xor r8, qword ptr [rdx]
0x1416eae61 49b9696d5f7374796c65 movabs r9, 0x656c7974735f6d69
0x1416eae6b 4c334a06 xor r9, qword ptr [rdx + 6]
0x1416eae6f 488d4801 lea rcx, [rax + 1]
0x1416eae73 4d09c1 or r9, r8
0x1416eae76 0f8419010000 je 0x1416eaf95
0x1416eae7c c60107 mov byte ptr [rcx], 7
0x1416eae7f c60000 mov byte ptr [rax], 0
0x1416eae82 c3 ret 
0x1416eae83 b970726f66 mov ecx, 0x666f7270
0x1416eae88 330a xor ecx, dword ptr [rdx]
0x1416eae8a 41b866696c65 mov r8d, 0x656c6966
0x1416eae90 44334203 xor r8d, dword ptr [rdx + 3]
0x1416eae94 4109c8 or r8d, ecx
0x1416eae97 0f85d5000000 jne 0x1416eaf72
0x1416eae9d e9db000000 jmp 0x1416eaf7d
0x1416eaea2 48b972696467655f6c65 movabs rcx, 0x656c5f6567646972
0x1416eaeac 48330a xor rcx, qword ptr [rdx]
0x1416eaeaf 49b8656e6774685f3031 movabs r8, 0x31305f6874676e65
0x1416eaeb9 4c334207 xor r8, qword ptr [rdx + 7]
0x1416eaebd 4909c8 or r8, rcx
0x1416eaec0 0f85ac000000 jne 0x1416eaf72
0x1416eaec6 c6400103 mov byte ptr [rax + 1], 3
0x1416eaeca c60000 mov byte ptr [rax], 0
0x1416eaecd c3 ret 
0x1416eaece 48b96865696768745f30 movabs rcx, 0x305f746867696568
0x1416eaed8 48330a xor rcx, qword ptr [rdx]
0x1416eaedb 440fb64208 movzx r8d, byte ptr [rdx + 8]
0x1416eaee0 4983f031 xor r8, 0x31
0x1416eaee4 4909c8 or r8, rcx
0x1416eaee7 0f8498000000 je 0x1416eaf85
0x1416eaeed 48b972696467655f6469 movabs rcx, 0x69645f6567646972
0x1416eaef7 48330a xor rcx, qword ptr [rdx]
0x1416eaefa 0fb65208 movzx edx, byte ptr [rdx + 8]
0x1416eaefe 4883f272 xor rdx, 0x72
0x1416eaf02 4809ca or rdx, rcx
0x1416eaf05 756b jne 0x1416eaf72
0x1416eaf07 c6400104 mov byte ptr [rax + 1], 4
0x1416eaf0b c60000 mov byte ptr [rax], 0
0x1416eaf0e c3 ret 
0x1416eaf0f 48b970726f66696c655f movabs rcx, 0x5f656c69666f7270
0x1416eaf19 48330a xor rcx, qword ptr [rdx]
0x1416eaf1c 440fb74208 movzx r8d, word ptr [rdx + 8]
0x1416eaf21 4981f030310000 xor r8, 0x3130
0x1416eaf28 4909c8 or r8, rcx
0x1416eaf2b 7450 je 0x1416eaf7d
0x1416eaf2d 48b97469705f6f666673 movabs rcx, 0x7366666f5f706974
0x1416eaf37 48330a xor rcx, qword ptr [rdx]
0x1416eaf3a 0fb75208 movzx edx, word ptr [rdx + 8]
0x1416eaf3e 4881f265740000 xor rdx, 0x7465
0x1416eaf45 4809ca or rdx, rcx
0x1416eaf48 7528 jne 0x1416eaf72
0x1416eaf4a c6400102 mov byte ptr [rax + 1], 2
0x1416eaf4e c60000 mov byte ptr [rax], 0
0x1416eaf51 c3 ret 
0x1416eaf52 48b97469705f6f666673 movabs rcx, 0x7366666f5f706974
0x1416eaf5c 48330a xor rcx, qword ptr [rdx]
0x1416eaf5f 49b866667365745f3031 movabs r8, 0x31305f7465736666
0x1416eaf69 4c334205 xor r8, qword ptr [rdx + 5]
0x1416eaf6d 4909c8 or r8, rcx
0x1416eaf70 74d8 je 0x1416eaf4a
0x1416eaf72 488d4801 lea rcx, [rax + 1]
0x1416eaf76 c60107 mov byte ptr [rcx], 7
0x1416eaf79 c60000 mov byte ptr [rax], 0
0x1416eaf7c c3 ret 
0x1416eaf7d c6400100 mov byte ptr [rax + 1], 0
0x1416eaf81 c60000 mov byte ptr [rax], 0
0x1416eaf84 c3 ret 
0x1416eaf85 c6400101 mov byte ptr [rax + 1], 1
0x1416eaf89 c60000 mov byte ptr [rax], 0
0x1416eaf8c c3 ret 
0x1416eaf8d c6400105 mov byte ptr [rax + 1], 5
0x1416eaf91 c60000 mov byte ptr [rax], 0
0x1416eaf94 c3 ret 
0x1416eaf95 c60106 mov byte ptr [rcx], 6
0x1416eaf98 c60000 mov byte ptr [rax], 0
0x1416eaf9b c3 ret 

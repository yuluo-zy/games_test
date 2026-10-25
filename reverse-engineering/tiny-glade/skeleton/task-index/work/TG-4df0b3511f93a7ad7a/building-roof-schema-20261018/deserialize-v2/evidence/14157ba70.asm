0x14157ba70 4889c8 mov rax, rcx
0x14157ba73 4983f805 cmp r8, 5
0x14157ba77 0f84b3000000 je 0x14157bb30
0x14157ba7d 4983f806 cmp r8, 6
0x14157ba81 7437 je 0x14157baba
0x14157ba83 4983f80b cmp r8, 0xb
0x14157ba87 0f85b5000000 jne 0x14157bb42
0x14157ba8d 48b96f7269656e746174 movabs rcx, 0x7461746e6569726f
0x14157ba97 48330a xor rcx, qword ptr [rdx]
0x14157ba9a 49b8656e746174696f6e movabs r8, 0x6e6f697461746e65
0x14157baa4 4c334203 xor r8, qword ptr [rdx + 3]
0x14157baa8 4909c8 or r8, rcx
0x14157baab 0f8591000000 jne 0x14157bb42
0x14157bab1 31c9 xor ecx, ecx
0x14157bab3 884801 mov byte ptr [rax + 1], cl
0x14157bab6 c60000 mov byte ptr [rax], 0
0x14157bab9 c3 ret 
0x14157baba b96c656e67 mov ecx, 0x676e656c
0x14157babf 330a xor ecx, dword ptr [rdx]
0x14157bac1 440fb74204 movzx r8d, word ptr [rdx + 4]
0x14157bac6 4181f074680000 xor r8d, 0x6874
0x14157bacd 4109c8 or r8d, ecx
0x14157bad0 7479 je 0x14157bb4b
0x14157bad2 b963656e74 mov ecx, 0x746e6563
0x14157bad7 330a xor ecx, dword ptr [rdx]
0x14157bad9 440fb74204 movzx r8d, word ptr [rdx + 4]
0x14157bade 4181f065720000 xor r8d, 0x7265
0x14157bae5 4109c8 or r8d, ecx
0x14157bae8 7473 je 0x14157bb5d
0x14157baea 8b0a mov ecx, dword ptr [rdx]
0x14157baec 0fc9 bswap ecx
0x14157baee 41b86769726f mov r8d, 0x6f726967
0x14157baf4 81f96769726f cmp ecx, 0x6f726967
0x14157bafa 751b jne 0x14157bb17
0x14157bafc 0fb74a04 movzx ecx, word ptr [rdx + 4]
0x14157bb00 66c1c108 rol cx, 8
0x14157bb04 0fb7c9 movzx ecx, cx
0x14157bb07 41b86e690000 mov r8d, 0x696e
0x14157bb0d 31d2 xor edx, edx
0x14157bb0f 81f96e690000 cmp ecx, 0x696e
0x14157bb15 740a je 0x14157bb21
0x14157bb17 31d2 xor edx, edx
0x14157bb19 4439c1 cmp ecx, r8d
0x14157bb1c 19d2 sbb edx, edx
0x14157bb1e 83ca01 or edx, 1
0x14157bb21 83fa01 cmp edx, 1
0x14157bb24 b104 mov cl, 4
0x14157bb26 80d900 sbb cl, 0
0x14157bb29 884801 mov byte ptr [rax + 1], cl
0x14157bb2c c60000 mov byte ptr [rax], 0
0x14157bb2f c3 ret 
0x14157bb30 b977696474 mov ecx, 0x74646977
0x14157bb35 330a xor ecx, dword ptr [rdx]
0x14157bb37 0fb65204 movzx edx, byte ptr [rdx + 4]
0x14157bb3b 83f268 xor edx, 0x68
0x14157bb3e 09ca or edx, ecx
0x14157bb40 7412 je 0x14157bb54
0x14157bb42 b104 mov cl, 4
0x14157bb44 884801 mov byte ptr [rax + 1], cl
0x14157bb47 c60000 mov byte ptr [rax], 0
0x14157bb4a c3 ret 
0x14157bb4b b102 mov cl, 2
0x14157bb4d 884801 mov byte ptr [rax + 1], cl
0x14157bb50 c60000 mov byte ptr [rax], 0
0x14157bb53 c3 ret 
0x14157bb54 b101 mov cl, 1
0x14157bb56 884801 mov byte ptr [rax + 1], cl
0x14157bb59 c60000 mov byte ptr [rax], 0
0x14157bb5c c3 ret 
0x14157bb5d b103 mov cl, 3
0x14157bb5f 884801 mov byte ptr [rax + 1], cl
0x14157bb62 c60000 mov byte ptr [rax], 0
0x14157bb65 c3 ret 

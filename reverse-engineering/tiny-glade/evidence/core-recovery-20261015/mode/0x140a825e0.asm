0x140a825e0 mov      r8d, dword ptr [rcx]
0x140a825e3 lea      r9d, [r8 - 6]
0x140a825e7 mov      r10, r8
0x140a825ea add      r10, -5
0x140a825ee xor      eax, eax
0x140a825f0 cmp      r9d, 0xf
0x140a825f4 cmovae   r10, rax
0x140a825f8 mov      r9d, dword ptr [rdx]
0x140a825fb lea      r11, [r9 - 5]
0x140a825ff cmp      r9, 6
0x140a82603 cmovb    r11, rax
0x140a82607 cmp      r10, r11
0x140a8260a jne      0x140a826b0
0x140a82610 mov      al, 1
0x140a82612 cmp      r10, 6
0x140a82616 ja       0x140a826b0
0x140a8261c lea      r11, [rip + 0x20561c1]
0x140a82623 movsxd   r10, dword ptr [r11 + r10*4]
0x140a82627 add      r10, r11
0x140a8262a jmp      r10
0x140a8262d movzx    eax, byte ptr [rcx + 4]
0x140a82631 cmp      al, byte ptr [rdx + 4]
0x140a82634 sete     al
0x140a82637 xor      al, 1
0x140a82639 ret      
0x140a8263a cmp      r9d, 5
0x140a8263e ja       0x140a826b0
0x140a82640 cmp      r8d, r9d
0x140a82643 jne      0x140a826b3
0x140a82645 mov      r8d, r8d
0x140a82648 lea      r9, [rip + 0x20561b1]
0x140a8264f movsxd   r8, dword ptr [r9 + r8*4]
0x140a82653 add      r8, r9
0x140a82656 jmp      r8
0x140a82659 mov      eax, dword ptr [rcx + 4]
0x140a8265c cmp      eax, dword ptr [rdx + 4]
0x140a8265f sete     al
0x140a82662 xor      al, 1
0x140a82664 ret      
0x140a82665 mov      rax, qword ptr [rcx + 8]
0x140a82669 cmp      rax, qword ptr [rdx + 8]
0x140a8266d sete     al
0x140a82670 xor      al, 1
0x140a82672 ret      
0x140a82673 movzx    eax, byte ptr [rcx + 4]
0x140a82677 lea      ecx, [rax - 0x38]
0x140a8267a cmp      cl, 8
0x140a8267d movzx    ecx, cl
0x140a82680 mov      r8d, 6
0x140a82686 cmovae   ecx, r8d
0x140a8268a movzx    edx, byte ptr [rdx + 4]
0x140a8268e lea      r9d, [rdx - 0x38]
0x140a82692 cmp      r9b, 8
0x140a82696 movzx    r9d, r9b
0x140a8269a cmovae   r9d, r8d
0x140a8269e cmp      cl, r9b
0x140a826a1 jne      0x140a826b3
0x140a826a3 cmp      cl, 6
0x140a826a6 setne    cl
0x140a826a9 cmp      al, dl
0x140a826ab sete     al
0x140a826ae or       al, cl
0x140a826b0 xor      al, 1
0x140a826b2 ret      
0x140a826b3 xor      eax, eax
0x140a826b5 xor      al, 1
0x140a826b7 ret      
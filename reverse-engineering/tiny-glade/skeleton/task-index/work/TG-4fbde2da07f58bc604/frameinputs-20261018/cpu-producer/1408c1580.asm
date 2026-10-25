1408c1580: push rbp
1408c1581: push r15
1408c1583: push r14
1408c1585: push r13
1408c1587: push r12
1408c1589: push rsi
1408c158a: push rdi
1408c158b: push rbx
1408c158c: sub rsp, 0x198
1408c1593: lea rbp, [rsp + 0x80]
1408c159b: mov qword ptr [rbp + 0x110], 0xfffffffffffffffe
1408c15a6: mov qword ptr [rbp + 0x78], r9
1408c15aa: mov r14, r8
1408c15ad: mov rdi, rdx
1408c15b0: mov rbx, rcx
1408c15b3: call 0x140c16a00
1408c15b8: test al, al
1408c15ba: je 0x1408c161c
1408c15bc: lea r12, [rip + 0x21df065]
1408c15c3: mov r15d, 0x46
1408c15c9: mov edx, 0x46
1408c15ce: mov rcx, r12
1408c15d1: call 0x1408b9b00
1408c15d6: cmp rax, 1
1408c15da: jne 0x1408c16ad
1408c15e0: mov r9, rdx
1408c15e3: test rdx, rdx
1408c15e6: je 0x1408c1640
1408c15e8: cmp r9, 0x46
1408c15ec: jae 0x1408c163e
1408c15ee: lea rax, [rip + 0x21df033]
1408c15f5: cmp byte ptr [r9 + rax], 0xbf
1408c15fa: jg 0x1408c1640
1408c15fc: lea rax, [rip + 0x21df0f5]
1408c1603: mov qword ptr [rsp + 0x20], rax
1408c1608: lea rcx, [rip + 0x21df019]
1408c160f: mov edx, 0x46
1408c1614: xor r8d, r8d
1408c1617: call 0x1428d9940
1408c161c: xor eax, eax
1408c161e: mov qword ptr [rbp + 0x68], rax
1408c1622: mov r12, qword ptr [r14]
1408c1625: cmp byte ptr [r12 + 0x1d], 0
1408c162b: jne 0x1408c181d
1408c1631: movss xmm0, dword ptr [rip + 0x20642cb]
1408c1639: jmp 0x1408c1825
1408c163e: jne 0x1408c15fc
1408c1640: lea r12, [rip + 0x21defe1]
1408c1647: mov rcx, r12
1408c164a: mov rdx, r9
1408c164d: call 0x1408b9b00
1408c1652: cmp rax, 1
1408c1656: jne 0x1408c16ad
1408c1658: mov r8, rdx
1408c165b: add r8, 2
1408c165f: je 0x1408c169a
1408c1661: cmp r8, 0x46
1408c1665: jae 0x1408c1698
1408c1667: lea rax, [rip + 0x21defba]
1408c166e: cmp byte ptr [r8 + rax], 0xbf
1408c1673: jg 0x1408c169a
1408c1675: lea rax, [rip + 0x21df094]
1408c167c: mov qword ptr [rsp + 0x20], rax
1408c1681: lea rcx, [rip + 0x21defa0]
1408c1688: mov edx, 0x46
1408c168d: mov r9d, 0x46
1408c1693: call 0x1428d9940
1408c1698: jne 0x1408c1675
1408c169a: mov r15d, 0x44
1408c16a0: sub r15, rdx
1408c16a3: lea r12, [rip + 0x21def7e]
1408c16aa: add r12, r8
1408c16ad: lea rax, [rip + 0x21dfd1f]
1408c16b4: lea r13, [rip + 0x21dfcdd]
1408c16bb: nop dword ptr [rax + rax]
1408c16c0: movsx ecx, byte ptr [rax - 1]
1408c16c4: test ecx, ecx
1408c16c6: js 0x1408c16e0
1408c16c8: dec rax
1408c16cb: cmp ecx, 0x5c
1408c16ce: jne 0x1408c1739
1408c16d0: jmp 0x1408c174e
1408c16d2: nop word ptr cs:[rax + rax]
1408c16e0: movzx edx, byte ptr [rax - 2]
1408c16e4: cmp dl, 0xc0
1408c16e7: jge 0x1408c170e
1408c16e9: movzx r8d, byte ptr [rax - 3]
1408c16ee: cmp r8b, 0xc0
1408c16f2: jge 0x1408c1717
1408c16f4: movzx r9d, byte ptr [rax - 4]
1408c16f9: add rax, -4
1408c16fd: and r9d, 7
1408c1701: shl r9d, 6
1408c1705: and r8d, 0x3f
1408c1709: or r8d, r9d
1408c170c: jmp 0x1408c171f
1408c170e: add rax, -2
1408c1712: and edx, 0x1f
1408c1715: jmp 0x1408c1729
1408c1717: add rax, -3
1408c171b: and r8d, 0xf
1408c171f: shl r8d, 6
1408c1723: and edx, 0x3f
1408c1726: or edx, r8d
1408c1729: shl edx, 6
1408c172c: and cl, 0x3f
1408c172f: movzx ecx, cl
1408c1732: or ecx, edx
1408c1734: cmp ecx, 0x5c
1408c1737: je 0x1408c174e
1408c1739: cmp ecx, 0x2f
1408c173c: je 0x1408c174e
1408c173e: cmp rax, r13
1408c1741: jne 0x1408c16c0
1408c1747: mov esi, 0x3b
1408c174c: jmp 0x1408c179d
1408c174e: lea r13, [rip + 0x21dfc43]
1408c1755: sub rax, r13
1408c1758: mov r8, rax
1408c175b: inc r8
1408c175e: je 0x1408c1792
1408c1760: cmp r8, 0x3b
1408c1764: jae 0x1408c1790
1408c1766: cmp byte ptr [r8 + r13], 0xbf
1408c176b: jg 0x1408c1792
1408c176d: lea rax, [rip + 0x21def64]
1408c1774: mov qword ptr [rsp + 0x20], rax
1408c1779: lea rcx, [rip + 0x21dfc18]
1408c1780: mov edx, 0x3b
1408c1785: mov r9d, 0x3b
1408c178b: call 0x1428d9940
1408c1790: jne 0x1408c176d
1408c1792: mov esi, 0x3a
1408c1797: sub rsi, rax
1408c179a: add r13, r8
1408c179d: call 0x140c18130
1408c17a2: mov rcx, rax
1408c17a5: mov rax, qword ptr [rax]
1408c17a8: cmp rax, 1
1408c17ac: jne 0x1408c1d1e
1408c17b2: add rcx, 8
1408c17b6: cmp qword ptr [rcx], 0
1408c17ba: jne 0x1408c1d42
1408c17c0: mov qword ptr [rcx], 0xffffffffffffffff
1408c17c7: mov qword ptr [rbp + 0x108], rcx
1408c17ce: add rcx, 8
1408c17d2: mov qword ptr [rsp + 0x20], rsi
1408c17d7: mov qword ptr [rsp + 0x30], 0
1408c17e0: mov qword ptr [rsp + 0x28], 1
1408c17e9: mov rdx, r12
1408c17ec: mov r8, r15
1408c17ef: mov r9, r13
1408c17f2: call 0x140c16d40
1408c17f7: mov rcx, qword ptr [rbp + 0x108]
1408c17fe: inc qword ptr [rcx]
1408c1801: mov qword ptr [rbp + 0x70], rax
1408c1805: mov eax, 1
1408c180a: mov qword ptr [rbp + 0x68], rax
1408c180e: mov r12, qword ptr [r14]
1408c1811: cmp byte ptr [r12 + 0x1d], 0
1408c1817: je 0x1408c1631
1408c181d: movss xmm0, dword ptr [rip + 0x21cbd9b]
1408c1825: movss dword ptr [rbp + 0x84], xmm0
1408c182d: mov r13, qword ptr [rbx]
1408c1830: mov rax, qword ptr [r13]
1408c1834: mov rcx, qword ptr [r13 + 8]
1408c1838: add rcx, rax
1408c183b: inc rcx
1408c183e: movdqa xmm0, xmmword ptr [rax]
1408c1842: pmovmskb edx, xmm0
1408c1846: not edx
1408c1848: mov r8, qword ptr [r13 + 0x18]
1408c184c: mov qword ptr [rbp + 0xa0], 0
1408c1857: mov qword ptr [rbp + 0xb0], 0
1408c1862: mov qword ptr [rbp + 0xc0], rax
1408c1869: add rax, 0x10
1408c186d: mov qword ptr [rbp + 0xc8], rax
1408c1874: mov qword ptr [rbp + 0xd0], rcx
1408c187b: mov word ptr [rbp + 0xd8], dx
1408c1882: mov qword ptr [rbp + 0xe0], r8
1408c1889: lea r8, [rip + 0x21e7920]
1408c1890: lea rcx, [rbp + 0xf0]
1408c1897: lea rdx, [rbp + 0xa0]
1408c189e: call 0x1412fa4d0
1408c18a3: mov rsi, qword ptr [rbp + 0x180]
1408c18aa: mov rcx, qword ptr [rbp + 0xf8]
1408c18b1: mov rax, qword ptr [rbp + 0x100]
1408c18b8: lea rax, [rcx + rax*8]
1408c18bc: mov qword ptr [rbp + 0x108], rcx
1408c18c3: mov qword ptr [rbp + 0xa0], rcx
1408c18ca: mov qword ptr [rbp + 0xa8], rax
1408c18d1: lea rbx, [rbp + 0x84]
1408c18d8: mov qword ptr [rbp + 0xb0], rbx
1408c18df: mov qword ptr [rbp + 0xb8], rsi
1408c18e6: lea r8, [rip + 0x21dea53]
1408c18ed: lea rcx, [rbp + 0x88]
1408c18f4: lea rdx, [rbp + 0xa0]
1408c18fb: call 0x1413081e0
1408c1900: mov rdx, qword ptr [rbp + 0xf0]
1408c1907: test rdx, rdx
1408c190a: je 0x1408c1922
1408c190c: shl rdx, 3
1408c1910: mov r8d, 8
1408c1916: mov rcx, qword ptr [rbp + 0x108]
1408c191d: call 0x140613c20
1408c1922: mov rdi, qword ptr [rdi]
1408c1925: lea rdx, [rip + 0x236c91c]
1408c192c: mov rcx, rdi
1408c192f: call 0x140969720
1408c1934: mov r14, rax
1408c1937: test rax, rax
1408c193a: je 0x1408c1a34
1408c1940: lea rax, [rip + 0x21dd240]
1408c1947: mov qword ptr [rbp + 0xf0], rax
1408c194e: mov qword ptr [rbp + 0xf8], 0x3d
1408c1959: mov r15, r14
1408c195c: add r15, 0x10
1408c1960: cmp qword ptr [r14 + 0x18], 0x3d
1408c1965: jne 0x1408c1d4e
1408c196b: mov rdx, qword ptr [r15]
1408c196e: lea rcx, [rip + 0x21dd212]
1408c1975: mov r8d, 0x3d
1408c197b: call 0x142923bf0
1408c1980: test eax, eax
1408c1982: jne 0x1408c1d4e
1408c1988: mov rcx, qword ptr [r14]
1408c198b: mov rax, qword ptr [r14 + 8]
1408c198f: call qword ptr [rax + 0x18]
1408c1992: mov r14, rax
1408c1995: test rax, rax
1408c1998: je 0x1408c1a34
1408c199e: mov r15, qword ptr [rbp + 0x98]
1408c19a5: test r15, r15
1408c19a8: je 0x1408c1a31
1408c19ae: mov rax, r15
1408c19b1: shl rax, 4
1408c19b5: lea rax, [rax + rax*2]
1408c19b9: cmp rax, qword ptr [r14 + 0x20]
1408c19bd: ja 0x1408c1db6
1408c19c3: mov rcx, qword ptr [rbp + 0x78]
1408c19c7: mov rcx, qword ptr [rcx]
1408c19ca: mov rdx, qword ptr [rcx]
1408c19cd: add rdx, 0x10
1408c19d1: mov r9, qword ptr [rbp + 0x90]
1408c19d8: mov r8, r14
1408c19db: add r8, 8
1408c19df: mov qword ptr [rbp + 0xf0], 0
1408c19ea: mov qword ptr [rbp + 0xf8], rax
1408c19f1: lea rcx, [rbp + 0xf0]
1408c19f8: mov qword ptr [rsp + 0x28], rcx
1408c19fd: mov qword ptr [rsp + 0x20], rax
1408c1a02: mov qword ptr [rsp + 0x38], 0
1408c1a0b: mov qword ptr [rsp + 0x30], 1
1408c1a14: lea rcx, [rbp + 0xa0]
1408c1a1b: call 0x140beefa0
1408c1a20: mov rax, qword ptr [rbp + 0xa0]
1408c1a27: cmp rax, 0xb
1408c1a2b: jne 0x1408c1dbd
1408c1a31: mov qword ptr [r14], r15
1408c1a34: mov rax, qword ptr [rbp + 0x88]
1408c1a3b: test rax, rax
1408c1a3e: je 0x1408c1a5a
1408c1a40: mov rcx, qword ptr [rbp + 0x90]
1408c1a47: shl rax, 4
1408c1a4b: lea rdx, [rax + rax*2]
1408c1a4f: mov r8d, 4
1408c1a55: call 0x140613c20
1408c1a5a: cmp byte ptr [r12 + 0x1d], 0
1408c1a60: je 0x1408c1b57
1408c1a66: mov rax, qword ptr [r13]
1408c1a6a: mov rcx, qword ptr [r13 + 8]
1408c1a6e: add rcx, rax
1408c1a71: inc rcx
1408c1a74: movdqa xmm0, xmmword ptr [rax]
1408c1a78: pmovmskb edx, xmm0
1408c1a7c: not edx
1408c1a7e: mov r8, qword ptr [r13 + 0x18]
1408c1a82: mov qword ptr [rbp + 0xa0], 0
1408c1a8d: mov qword ptr [rbp + 0xb0], 0
1408c1a98: mov qword ptr [rbp + 0xc0], rax
1408c1a9f: add rax, 0x10
1408c1aa3: mov qword ptr [rbp + 0xc8], rax
1408c1aaa: mov qword ptr [rbp + 0xd0], rcx
1408c1ab1: mov word ptr [rbp + 0xd8], dx
1408c1ab8: mov qword ptr [rbp + 0xe0], r8
1408c1abf: lea r8, [rip + 0x21e76ea]
1408c1ac6: lea rcx, [rbp + 0xf0]
1408c1acd: lea rdx, [rbp + 0xa0]
1408c1ad4: call 0x1412fa4d0
1408c1ad9: mov rcx, qword ptr [rbp + 0xf8]
1408c1ae0: mov rax, qword ptr [rbp + 0x100]
1408c1ae7: lea rax, [rcx + rax*8]
1408c1aeb: mov qword ptr [rbp + 0x108], rcx
1408c1af2: mov qword ptr [rbp + 0xa0], rcx
1408c1af9: mov qword ptr [rbp + 0xa8], rax
1408c1b00: mov qword ptr [rbp + 0xb0], rbx
1408c1b07: mov qword ptr [rbp + 0xb8], rsi
1408c1b0e: mov qword ptr [rbp + 0xc0], 0x3e8
1408c1b19: lea r8, [rip + 0x21de820]
1408c1b20: lea rcx, [rbp + 0x88]
1408c1b27: lea rdx, [rbp + 0xa0]
1408c1b2e: call 0x141300ee0
1408c1b33: mov rdx, qword ptr [rbp + 0xf0]
1408c1b3a: test rdx, rdx
1408c1b3d: je 0x1408c1b78
1408c1b3f: shl rdx, 3
1408c1b43: mov r8d, 8
1408c1b49: mov rcx, qword ptr [rbp + 0x108]
1408c1b50: call 0x140613c20
1408c1b55: jmp 0x1408c1b78
1408c1b57: mov qword ptr [rbp + 0x88], 0
1408c1b62: mov qword ptr [rbp + 0x90], 4
1408c1b6d: mov qword ptr [rbp + 0x98], 0
1408c1b78: lea rdx, [rip + 0x236c701]
1408c1b7f: mov rcx, rdi
1408c1b82: call 0x140969720
1408c1b87: mov rdi, rax
1408c1b8a: test rax, rax
1408c1b8d: je 0x1408c1c87
1408c1b93: lea rax, [rip + 0x21dbeea]
1408c1b9a: mov qword ptr [rbp + 0xf0], rax
1408c1ba1: mov qword ptr [rbp + 0xf8], 0x3f
1408c1bac: mov r14, rdi
1408c1baf: add r14, 0x10
1408c1bb3: cmp qword ptr [rdi + 0x18], 0x3f
1408c1bb8: jne 0x1408c1d82
1408c1bbe: mov rdx, qword ptr [r14]
1408c1bc1: lea rcx, [rip + 0x21dbebc]
1408c1bc8: mov r8d, 0x3f
1408c1bce: call 0x142923bf0
1408c1bd3: test eax, eax
1408c1bd5: jne 0x1408c1d82
1408c1bdb: mov rcx, qword ptr [rdi]
1408c1bde: mov rax, qword ptr [rdi + 8]
1408c1be2: call qword ptr [rax + 0x18]
1408c1be5: mov rdi, rax
1408c1be8: test rax, rax
1408c1beb: je 0x1408c1c87
1408c1bf1: mov rsi, qword ptr [rbp + 0x98]
1408c1bf8: test rsi, rsi
1408c1bfb: je 0x1408c1c84
1408c1c01: mov rax, rsi
1408c1c04: shl rax, 4
1408c1c08: lea rax, [rax + rax*2]
1408c1c0c: cmp rax, qword ptr [rdi + 0x20]
1408c1c10: ja 0x1408c1e66
1408c1c16: mov rcx, qword ptr [rbp + 0x78]
1408c1c1a: mov rcx, qword ptr [rcx]
1408c1c1d: mov rdx, qword ptr [rcx]
1408c1c20: add rdx, 0x10
1408c1c24: mov r9, qword ptr [rbp + 0x90]
1408c1c2b: mov r8, rdi
1408c1c2e: add r8, 8
1408c1c32: mov qword ptr [rbp + 0xf0], 0
1408c1c3d: mov qword ptr [rbp + 0xf8], rax
1408c1c44: lea rcx, [rbp + 0xf0]
1408c1c4b: mov qword ptr [rsp + 0x28], rcx
1408c1c50: mov qword ptr [rsp + 0x20], rax
1408c1c55: mov qword ptr [rsp + 0x38], 0
1408c1c5e: mov qword ptr [rsp + 0x30], 1
1408c1c67: lea rcx, [rbp + 0xa0]
1408c1c6e: call 0x140beefa0
1408c1c73: mov rax, qword ptr [rbp + 0xa0]
1408c1c7a: cmp rax, 0xb
1408c1c7e: jne 0x1408c1e6d
1408c1c84: mov qword ptr [rdi], rsi
1408c1c87: mov rax, qword ptr [rbp + 0x88]
1408c1c8e: test rax, rax
1408c1c91: je 0x1408c1cad
1408c1c93: mov rcx, qword ptr [rbp + 0x90]
1408c1c9a: shl rax, 4
1408c1c9e: lea rdx, [rax + rax*2]
1408c1ca2: mov r8d, 4
1408c1ca8: call 0x140613c20
1408c1cad: cmp qword ptr [rbp + 0x68], 0
1408c1cb2: je 0x1408c1cf4
1408c1cb4: call 0x140c18130
1408c1cb9: mov rcx, rax
1408c1cbc: mov rax, qword ptr [rax]
1408c1cbf: cmp rax, 1
1408c1cc3: jne 0x1408c1d08
1408c1cc5: add rcx, 8
1408c1cc9: cmp qword ptr [rcx], 0
1408c1ccd: jne 0x1408c1d42
1408c1ccf: mov qword ptr [rcx], 0xffffffffffffffff
1408c1cd6: mov qword ptr [rbp + 0x108], rcx
1408c1cdd: add rcx, 8
1408c1ce1: mov rdx, qword ptr [rbp + 0x70]
1408c1ce5: call 0x140c170f0
1408c1cea: mov rax, qword ptr [rbp + 0x108]
1408c1cf1: inc qword ptr [rax]
1408c1cf4: add rsp, 0x198
1408c1cfb: pop rbx
1408c1cfc: pop rdi
1408c1cfd: pop rsi
1408c1cfe: pop r12
1408c1d00: pop r13
1408c1d02: pop r14
1408c1d04: pop r15
1408c1d06: pop rbp
1408c1d07: ret 
1408c1d08: test rax, rax
1408c1d0b: jne 0x1408c1d36
1408c1d0d: xor edx, edx
1408c1d0f: call 0x1428f87d0
1408c1d14: mov rcx, rax
1408c1d17: test rax, rax
1408c1d1a: jne 0x1408c1cc9
1408c1d1c: jmp 0x1408c1d36
1408c1d1e: test rax, rax
1408c1d21: jne 0x1408c1d36
1408c1d23: xor edx, edx
1408c1d25: call 0x1428f87d0
1408c1d2a: mov rcx, rax
1408c1d2d: test rax, rax
1408c1d30: jne 0x1408c17b6
1408c1d36: lea rcx, [rip + 0x2378d63]
1408c1d3d: call 0x1428d80d0
1408c1d42: lea rcx, [rip + 0x2378e27]
1408c1d49: call 0x1428d9250
1408c1d4e: mov qword ptr [rbp + 0xa0], 0
1408c1d59: lea rax, [rip + 0x236c420]
1408c1d60: mov qword ptr [rsp + 0x20], rax
1408c1d65: lea rdx, [rbp + 0xf0]
1408c1d6c: lea r9, [rbp + 0xa0]
1408c1d73: xor ecx, ecx
1408c1d75: mov r8, r15
1408c1d78: call 0x1428cab41
1408c1d7d: jmp 0x1408c1f11
1408c1d82: mov qword ptr [rbp + 0xa0], 0
1408c1d8d: lea rax, [rip + 0x236c3ec]
1408c1d94: mov qword ptr [rsp + 0x20], rax
1408c1d99: lea rdx, [rbp + 0xf0]
1408c1da0: lea r9, [rbp + 0xa0]
1408c1da7: xor ecx, ecx
1408c1da9: mov r8, r14
1408c1dac: call 0x1428cab41
1408c1db1: jmp 0x1408c1f11
1408c1db6: mov eax, 0xb
1408c1dbb: jmp 0x1408c1df6
1408c1dbd: mov rcx, qword ptr [rbp + 0xe8]
1408c1dc4: mov qword ptr [rbp + 0x50], rcx
1408c1dc8: movdqu xmm0, xmmword ptr [rbp + 0xa8]
1408c1dd0: movups xmm1, xmmword ptr [rbp + 0xb8]
1408c1dd7: movups xmm2, xmmword ptr [rbp + 0xc8]
1408c1dde: movups xmm3, xmmword ptr [rbp + 0xd8]
1408c1de5: movaps xmmword ptr [rbp + 0x40], xmm3
1408c1de9: movaps xmmword ptr [rbp + 0x30], xmm2
1408c1ded: movaps xmmword ptr [rbp + 0x20], xmm1
1408c1df1: movdqa xmmword ptr [rbp + 0x10], xmm0
1408c1df6: mov qword ptr [rbp + 0xa0], rax
1408c1dfd: movdqa xmm0, xmmword ptr [rbp + 0x10]
1408c1e02: movaps xmm1, xmmword ptr [rbp + 0x20]
1408c1e06: movaps xmm2, xmmword ptr [rbp + 0x30]
1408c1e0a: movaps xmm3, xmmword ptr [rbp + 0x40]
1408c1e0e: movdqu xmmword ptr [rbp + 0xa8], xmm0
1408c1e16: movups xmmword ptr [rbp + 0xb8], xmm1
1408c1e1d: movups xmmword ptr [rbp + 0xc8], xmm2
1408c1e24: movups xmmword ptr [rbp + 0xd8], xmm3
1408c1e2b: mov rax, qword ptr [rbp + 0x50]
1408c1e2f: mov qword ptr [rbp + 0xe8], rax
1408c1e36: lea rax, [rip + 0x21df59b]
1408c1e3d: mov qword ptr [rsp + 0x20], rax
1408c1e42: lea rcx, [rip + 0x21de56f]
1408c1e49: lea r9, [rip + 0x21de5b8]
1408c1e50: lea r8, [rbp + 0xa0]
1408c1e57: mov edx, 0x2b
1408c1e5c: call 0x1428d9760
1408c1e61: jmp 0x1408c1f11
1408c1e66: mov eax, 0xb
1408c1e6b: jmp 0x1408c1ea6
1408c1e6d: mov rcx, qword ptr [rbp + 0xe8]
1408c1e74: mov qword ptr [rbp], rcx
1408c1e78: movdqu xmm0, xmmword ptr [rbp + 0xa8]
1408c1e80: movups xmm1, xmmword ptr [rbp + 0xb8]
1408c1e87: movups xmm2, xmmword ptr [rbp + 0xc8]
1408c1e8e: movups xmm3, xmmword ptr [rbp + 0xd8]
1408c1e95: movaps xmmword ptr [rbp - 0x10], xmm3
1408c1e99: movaps xmmword ptr [rbp - 0x20], xmm2
1408c1e9d: movaps xmmword ptr [rbp - 0x30], xmm1
1408c1ea1: movdqa xmmword ptr [rbp - 0x40], xmm0
1408c1ea6: mov qword ptr [rbp + 0xa0], rax
1408c1ead: movdqa xmm0, xmmword ptr [rbp - 0x40]
1408c1eb2: movaps xmm1, xmmword ptr [rbp - 0x30]
1408c1eb6: movaps xmm2, xmmword ptr [rbp - 0x20]
1408c1eba: movaps xmm3, xmmword ptr [rbp - 0x10]
1408c1ebe: movdqu xmmword ptr [rbp + 0xa8], xmm0
1408c1ec6: movups xmmword ptr [rbp + 0xb8], xmm1
1408c1ecd: movups xmmword ptr [rbp + 0xc8], xmm2
1408c1ed4: movups xmmword ptr [rbp + 0xd8], xmm3
1408c1edb: mov rax, qword ptr [rbp]
1408c1edf: mov qword ptr [rbp + 0xe8], rax
1408c1ee6: lea rax, [rip + 0x21df503]
1408c1eed: mov qword ptr [rsp + 0x20], rax
1408c1ef2: lea rcx, [rip + 0x21de4bf]
1408c1ef9: lea r9, [rip + 0x21de508]
1408c1f00: lea r8, [rbp + 0xa0]
1408c1f07: mov edx, 0x2b
1408c1f0c: call 0x1428d9760
1408c1f11: ud2 
1408c1f13: nop word ptr cs:[rax + rax]
1408c1f20: mov qword ptr [rsp + 0x10], rdx
1408c1f25: push rbp
1408c1f26: push r15
1408c1f28: push r14
1408c1f2a: push r13
1408c1f2c: push r12
1408c1f2e: push rsi
1408c1f2f: push rdi
1408c1f30: push rbx
1408c1f31: sub rsp, 0x48
1408c1f35: lea rbp, [rdx + 0x80]
1408c1f3c: mov rax, qword ptr [rbp + 0x108]
1408c1f43: inc qword ptr [rax]
1408c1f46: add rsp, 0x48
1408c1f4a: pop rbx
1408c1f4b: pop rdi
1408c1f4c: pop rsi
1408c1f4d: pop r12
1408c1f4f: pop r13
1408c1f51: pop r14
1408c1f53: pop r15
1408c1f55: pop rbp
1408c1f56: ret 
1408c1f57: nop word ptr [rax + rax]
1408c1f60: mov qword ptr [rsp + 0x10], rdx
1408c1f65: push rbp
1408c1f66: push r15
1408c1f68: push r14
1408c1f6a: push r13
1408c1f6c: push r12
1408c1f6e: push rsi
1408c1f6f: push rdi
1408c1f70: push rbx
1408c1f71: sub rsp, 0x48
1408c1f75: lea rbp, [rdx + 0x80]
1408c1f7c: cmp qword ptr [rbp + 0x68], 0
1408c1f81: je 0x1408c1fa3
1408c1f83: lea rax, [rbp + 0x70]
1408c1f87: mov qword ptr [rbp + 0x60], rax
1408c1f8b: lea rax, [rbp + 0x60]
1408c1f8f: mov qword ptr [rbp + 0x58], rax
1408c1f93: lea rcx, [rip + 0x21de6de]
1408c1f9a: lea rdx, [rbp + 0x58]
1408c1f9e: call 0x14132f2d0
1408c1fa3: nop 
1408c1fa4: add rsp, 0x48
1408c1fa8: pop rbx
1408c1fa9: pop rdi
1408c1faa: pop rsi
1408c1fab: pop r12
1408c1fad: pop r13
1408c1faf: pop r14
1408c1fb1: pop r15
1408c1fb3: pop rbp
1408c1fb4: ret 
1408c1fb5: nop word ptr cs:[rax + rax]
1408c1fc0: mov qword ptr [rsp + 0x10], rdx
1408c1fc5: push rbp
1408c1fc6: push r15
1408c1fc8: push r14
1408c1fca: push r13
1408c1fcc: push r12
1408c1fce: push rsi
1408c1fcf: push rdi
1408c1fd0: push rbx
1408c1fd1: sub rsp, 0x48
1408c1fd5: lea rbp, [rdx + 0x80]
1408c1fdc: mov rdx, qword ptr [rbp + 0xf0]
1408c1fe3: test rdx, rdx
1408c1fe6: je 0x1408c1ffe
1408c1fe8: shl rdx, 3
1408c1fec: mov r8d, 8
1408c1ff2: mov rcx, qword ptr [rbp + 0x108]
1408c1ff9: call 0x140613c20
1408c1ffe: nop 
1408c1fff: add rsp, 0x48
1408c2003: pop rbx
1408c2004: pop rdi
1408c2005: pop rsi
1408c2006: pop r12
1408c2008: pop r13
1408c200a: pop r14
1408c200c: pop r15
1408c200e: pop rbp
1408c200f: ret 
1408c2010: mov qword ptr [rsp + 0x10], rdx
1408c2015: push rbp
1408c2016: push r15
1408c2018: push r14
1408c201a: push r13
1408c201c: push r12
1408c201e: push rsi
1408c201f: push rdi
1408c2020: push rbx
1408c2021: sub rsp, 0x48
1408c2025: lea rbp, [rdx + 0x80]
1408c202c: mov rax, qword ptr [rbp + 0x88]
1408c2033: test rax, rax
1408c2036: je 0x1408c2052
1408c2038: mov rcx, qword ptr [rbp + 0x90]
1408c203f: shl rax, 4
1408c2043: lea rdx, [rax + rax*2]
1408c2047: mov r8d, 4
1408c204d: call 0x140613c20
1408c2052: nop 
1408c2053: add rsp, 0x48
1408c2057: pop rbx
1408c2058: pop rdi
1408c2059: pop rsi
1408c205a: pop r12
1408c205c: pop r13
1408c205e: pop r14
1408c2060: pop r15
1408c2062: pop rbp
1408c2063: ret 
1408c2064: nop word ptr cs:[rax + rax]
1408c2070: mov qword ptr [rsp + 0x10], rdx
1408c2075: push rbp
1408c2076: push r15
1408c2078: push r14
1408c207a: push r13
1408c207c: push r12
1408c207e: push rsi
1408c207f: push rdi
1408c2080: push rbx
1408c2081: sub rsp, 0x48
1408c2085: lea rbp, [rdx + 0x80]
1408c208c: mov rdx, qword ptr [rbp + 0xf0]
1408c2093: test rdx, rdx
1408c2096: je 0x1408c20ae
1408c2098: shl rdx, 3
1408c209c: mov r8d, 8
1408c20a2: mov rcx, qword ptr [rbp + 0x108]
1408c20a9: call 0x140613c20
1408c20ae: nop 
1408c20af: add rsp, 0x48
1408c20b3: pop rbx
1408c20b4: pop rdi
1408c20b5: pop rsi
1408c20b6: pop r12
1408c20b8: pop r13
1408c20ba: pop r14
1408c20bc: pop r15
1408c20be: pop rbp
1408c20bf: ret 
1408c20c0: mov qword ptr [rsp + 0x10], rdx
1408c20c5: push rbp
1408c20c6: push r15
1408c20c8: push r14
1408c20ca: push r13
1408c20cc: push r12
1408c20ce: push rsi
1408c20cf: push rdi
1408c20d0: push rbx
1408c20d1: sub rsp, 0x48
1408c20d5: lea rbp, [rdx + 0x80]
1408c20dc: mov rax, qword ptr [rbp + 0x88]
1408c20e3: test rax, rax
1408c20e6: je 0x1408c2102
1408c20e8: mov rcx, qword ptr [rbp + 0x90]
1408c20ef: shl rax, 4
1408c20f3: lea rdx, [rax + rax*2]
1408c20f7: mov r8d, 4
1408c20fd: call 0x140613c20
1408c2102: nop 
1408c2103: add rsp, 0x48
1408c2107: pop rbx
1408c2108: pop rdi
1408c2109: pop rsi
1408c210a: pop r12
1408c210c: pop r13
1408c210e: pop r14
1408c2110: pop r15
1408c2112: pop rbp
1408c2113: ret 
1408c2114: nop word ptr cs:[rax + rax]
1408c2120: mov qword ptr [rsp + 0x10], rdx
1408c2125: push rbp
1408c2126: push r15
1408c2128: push r14
1408c212a: push r13
1408c212c: push r12
1408c212e: push rsi
1408c212f: push rdi
1408c2130: push rbx
1408c2131: sub rsp, 0x48
1408c2135: lea rbp, [rdx + 0x80]
1408c213c: mov rax, qword ptr [rbp + 0x108]
1408c2143: inc qword ptr [rax]
1408c2146: add rsp, 0x48
1408c214a: pop rbx
1408c214b: pop rdi
1408c214c: pop rsi
1408c214d: pop r12
1408c214f: pop r13
1408c2151: pop r14
1408c2153: pop r15
1408c2155: pop rbp
1408c2156: ret 
1408c2157: nop word ptr [rax + rax]
1408c2160: mov qword ptr [rsp + 0x10], rdx
1408c2165: push rbp
1408c2166: push r15
1408c2168: push r14
1408c216a: push r13
1408c216c: push r12
1408c216e: push rsi
1408c216f: push rdi
1408c2170: push rbx
1408c2171: sub rsp, 0x48
1408c2175: lea rbp, [rdx + 0x80]
1408c217c: lea rcx, [rbp + 0xa0]
1408c2183: call 0x1407b1810
1408c2188: nop 
1408c2189: add rsp, 0x48
1408c218d: pop rbx
1408c218e: pop rdi
1408c218f: pop rsi
1408c2190: pop r12
1408c2192: pop r13
1408c2194: pop r14
1408c2196: pop r15
1408c2198: pop rbp
1408c2199: ret 
1408c219a: nop word ptr [rax + rax]
1408c21a0: mov qword ptr [rsp + 0x10], rdx
1408c21a5: push rbp
1408c21a6: push r15
1408c21a8: push r14
1408c21aa: push r13
1408c21ac: push r12
1408c21ae: push rsi
1408c21af: push rdi
1408c21b0: push rbx
1408c21b1: sub rsp, 0x48
1408c21b5: lea rbp, [rdx + 0x80]
1408c21bc: lea rcx, [rbp + 0xa0]
1408c21c3: call 0x1407b1810
1408c21c8: nop 
1408c21c9: add rsp, 0x48
1408c21cd: pop rbx
1408c21ce: pop rdi
1408c21cf: pop rsi
1408c21d0: pop r12
1408c21d2: pop r13
1408c21d4: pop r14
1408c21d6: pop r15
1408c21d8: pop rbp
1408c21d9: ret 

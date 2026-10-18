140a49820: 55  push rbp
140a49821: 4157  push r15
140a49823: 4156  push r14
140a49825: 4154  push r12
140a49827: 56  push rsi
140a49828: 57  push rdi
140a49829: 53  push rbx
140a4982a: 4881ec40010000  sub rsp, 0x140
140a49831: 488dac2480000000  lea rbp, [rsp + 0x80]
140a49839: 48c785b8000000feffffff  mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
140a49844: 4889ce  mov rsi, rcx
140a49847: 0fb602  movzx eax, byte ptr [rdx]
140a4984a: 83f803  cmp eax, 3
140a4984d: 8985a0000000  mov dword ptr [rbp + 0xa0], eax
140a49853: 48895578  mov qword ptr [rbp + 0x78], rdx
140a49857: 0f84ed000000  je 0x140a4994a
140a4985d: 83f805  cmp eax, 5
140a49860: 0f85cf010000  jne 0x140a49a35
140a49866: 4c8b7a08  mov r15, qword ptr [rdx + 8]
140a4986a: 4c8b6210  mov r12, qword ptr [rdx + 0x10]
140a4986e: 488b7a18  mov rdi, qword ptr [rdx + 0x18]
140a49872: 31c0  xor eax, eax
140a49874: 4d85ff  test r15, r15
140a49877: 0f95c0  setne al
140a4987a: 4889f9  mov rcx, rdi
140a4987d: 490f44cf  cmove rcx, r15
140a49881: 48894520  mov qword ptr [rbp + 0x20], rax
140a49885: 48c7452800000000  mov qword ptr [rbp + 0x28], 0
140a4988d: 4c897d30  mov qword ptr [rbp + 0x30], r15
140a49891: 4c896538  mov qword ptr [rbp + 0x38], r12
140a49895: 48894540  mov qword ptr [rbp + 0x40], rax
140a49899: 48c7454800000000  mov qword ptr [rbp + 0x48], 0
140a498a1: 4c897d50  mov qword ptr [rbp + 0x50], r15
140a498a5: 4c896558  mov qword ptr [rbp + 0x58], r12
140a498a9: 48894d60  mov qword ptr [rbp + 0x60], rcx
140a498ad: 488d8d80000000  lea rcx, [rbp + 0x80]
140a498b4: 488d5520  lea rdx, [rbp + 0x20]
140a498b8: e8932b9f00  call 0x14143c450
140a498bd: 488b8580000000  mov rax, qword ptr [rbp + 0x80]
140a498c4: 4885c0  test rax, rax
140a498c7: 7444  je 0x140a4990d
140a498c9: 488b8d90000000  mov rcx, qword ptr [rbp + 0x90]
140a498d0: 488d1449  lea rdx, [rcx + rcx*2]
140a498d4: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
140a498dc: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
140a498e4: 488b9cd078010000  mov rbx, qword ptr [rax + rdx*8 + 0x178]
140a498ec: 48c1e105  shl rcx, 5
140a498f0: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
140a498f4: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
140a498f9: 0f2945e0  movaps xmmword ptr [rbp - 0x20], xmm0
140a498fd: 0f294df0  movaps xmmword ptr [rbp - 0x10], xmm1
140a49901: 4c89c0  mov rax, r8
140a49904: 48f7d8  neg rax
140a49907: 0f81b5010000  jno 0x140a49ac2
140a4990d: c685800000000b  mov byte ptr [rbp + 0x80], 0xb
140a49914: 488d152d4b0802  lea rdx, [rip + 0x2084b2d]
140a4991b: 4c8d05364b0802  lea r8, [rip + 0x2084b36]
140a49922: 488d8d80000000  lea rcx, [rbp + 0x80]
140a49929: e81202eb01  call 0x1428f9b40
140a4992e: 48894608  mov qword ptr [rsi + 8], rax
140a49932: c60601  mov byte ptr [rsi], 1
140a49935: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4993c: 488d4d20  lea rcx, [rbp + 0x20]
140a49940: e8abe4e3ff  call 0x140887df0
140a49945: e973020000  jmp 0x140a49bbd
140a4994a: 4c8b7a08  mov r15, qword ptr [rdx + 8]
140a4994e: 4c8b6210  mov r12, qword ptr [rdx + 0x10]
140a49952: 488b7a18  mov rdi, qword ptr [rdx + 0x18]
140a49956: 41b606  mov r14b, 6
140a49959: 4c89f8  mov rax, r15
140a4995c: 4c89e1  mov rcx, r12
140a4995f: 4889fb  mov rbx, rdi
140a49962: 48894520  mov qword ptr [rbp + 0x20], rax
140a49966: 48894d28  mov qword ptr [rbp + 0x28], rcx
140a4996a: 48895d30  mov qword ptr [rbp + 0x30], rbx
140a4996e: 0f2845c0  movaps xmm0, xmmword ptr [rbp - 0x40]
140a49972: 0f114539  movups xmmword ptr [rbp + 0x39], xmm0
140a49976: 0f1045cf  movups xmm0, xmmword ptr [rbp - 0x31]
140a4997a: 0f114548  movups xmmword ptr [rbp + 0x48], xmm0
140a4997e: 44887538  mov byte ptr [rbp + 0x38], r14b
140a49982: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a49989: 488d4de0  lea rcx, [rbp - 0x20]
140a4998d: 488d5520  lea rdx, [rbp + 0x20]
140a49991: e81a82b800  call 0x1415d1bb0
140a49996: 0fb65de0  movzx ebx, byte ptr [rbp - 0x20]
140a4999a: 488b45e8  mov rax, qword ptr [rbp - 0x18]
140a4999e: 80fb02  cmp bl, 2
140a499a1: 7439  je 0x140a499dc
140a499a3: 488b4d00  mov rcx, qword ptr [rbp]
140a499a7: 48898d98000000  mov qword ptr [rbp + 0x98], rcx
140a499ae: 0f1045f0  movups xmm0, xmmword ptr [rbp - 0x10]
140a499b2: 0f118588000000  movups xmmword ptr [rbp + 0x88], xmm0
140a499b9: 48898580000000  mov qword ptr [rbp + 0x80], rax
140a499c0: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a499c7: 488d8d80000000  lea rcx, [rbp + 0x80]
140a499ce: e82de61300  call 0x140b88000
140a499d3: 4885c0  test rax, rax
140a499d6: 0f849f010000  je 0x140a49b7b
140a499dc: 48894608  mov qword ptr [rsi + 8], rax
140a499e0: b001  mov al, 1
140a499e2: 8806  mov byte ptr [rsi], al
140a499e4: 8b85a0000000  mov eax, dword ptr [rbp + 0xa0]
140a499ea: 83f803  cmp eax, 3
140a499ed: 0f84ca010000  je 0x140a49bbd
140a499f3: 83f805  cmp eax, 5
140a499f6: 0f84c1010000  je 0x140a49bbd
140a499fc: 4c89bda0000000  mov qword ptr [rbp + 0xa0], r15
140a49a03: 4c896578  mov qword ptr [rbp + 0x78], r12
140a49a07: 4c89e1  mov rcx, r12
140a49a0a: 4889fa  mov rdx, rdi
140a49a0d: e85ef3e3ff  call 0x140888d70
140a49a12: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
140a49a19: 4885d2  test rdx, rdx
140a49a1c: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
140a49a20: 0f8497010000  je 0x140a49bbd
140a49a26: 48c1e205  shl rdx, 5
140a49a2a: 41b808000000  mov r8d, 8
140a49a30: e971010000  jmp 0x140a49ba6
140a49a35: 0f1002  movups xmm0, xmmword ptr [rdx]
140a49a38: 0f104a10  movups xmm1, xmmword ptr [rdx + 0x10]
140a49a3c: 0f294d30  movaps xmmword ptr [rbp + 0x30], xmm1
140a49a40: 0f294520  movaps xmmword ptr [rbp + 0x20], xmm0
140a49a44: 488d4de0  lea rcx, [rbp - 0x20]
140a49a48: 488d5520  lea rdx, [rbp + 0x20]
140a49a4c: e8bf01e801  call 0x1428c9c10
140a49a51: 488d15304a0802  lea rdx, [rip + 0x2084a30]
140a49a58: 4c8d05f9490802  lea r8, [rip + 0x20849f9]
140a49a5f: 488d4de0  lea rcx, [rbp - 0x20]
140a49a63: e898ffe701  call 0x1428c9a00
140a49a68: 48894608  mov qword ptr [rsi + 8], rax
140a49a6c: c60601  mov byte ptr [rsi], 1
140a49a6f: 0fb64520  movzx eax, byte ptr [rbp + 0x20]
140a49a73: 83f803  cmp eax, 3
140a49a76: 0f8241010000  jb 0x140a49bbd
140a49a7c: 0f8411010000  je 0x140a49b93
140a49a82: 83f804  cmp eax, 4
140a49a85: 0f8522010000  jne 0x140a49bad
140a49a8b: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
140a49a8f: 488b5538  mov rdx, qword ptr [rbp + 0x38]
140a49a93: 48898da8000000  mov qword ptr [rbp + 0xa8], rcx
140a49a9a: e8d1f2e3ff  call 0x140888d70
140a49a9f: 488b5528  mov rdx, qword ptr [rbp + 0x28]
140a49aa3: 4885d2  test rdx, rdx
140a49aa6: 0f8411010000  je 0x140a49bbd
140a49aac: 48c1e205  shl rdx, 5
140a49ab0: 41b808000000  mov r8d, 8
140a49ab6: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
140a49abd: e9e4000000  jmp 0x140a49ba6
140a49ac2: 4c894d70  mov qword ptr [rbp + 0x70], r9
140a49ac6: 4c8985a8000000  mov qword ptr [rbp + 0xa8], r8
140a49acd: 0f2845e0  movaps xmm0, xmmword ptr [rbp - 0x20]
140a49ad1: 0f284df0  movaps xmm1, xmmword ptr [rbp - 0x10]
140a49ad5: 0f298d90000000  movaps xmmword ptr [rbp + 0x90], xmm1
140a49adc: 0f298580000000  movaps xmmword ptr [rbp + 0x80], xmm0
140a49ae3: 488d4da8  lea rcx, [rbp - 0x58]
140a49ae7: 488d5520  lea rdx, [rbp + 0x20]
140a49aeb: e860299f00  call 0x14143c450
140a49af0: 488b45a8  mov rax, qword ptr [rbp - 0x58]
140a49af4: 4885c0  test rax, rax
140a49af7: 7445  je 0x140a49b3e
140a49af9: 488b4db8  mov rcx, qword ptr [rbp - 0x48]
140a49afd: 488d1449  lea rdx, [rcx + rcx*2]
140a49b01: 4c8b84d078010000  mov r8, qword ptr [rax + rdx*8 + 0x178]
140a49b09: 4c8945f0  mov qword ptr [rbp - 0x10], r8
140a49b0d: 0f1084d068010000  movups xmm0, xmmword ptr [rax + rdx*8 + 0x168]
140a49b15: 0f2945e0  movaps xmmword ptr [rbp - 0x20], xmm0
140a49b19: 48c1e105  shl rcx, 5
140a49b1d: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
140a49b21: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
140a49b26: 0f1145f8  movups xmmword ptr [rbp - 8], xmm0
140a49b2a: 0f114d08  movups xmmword ptr [rbp + 8], xmm1
140a49b2e: 488b55e0  mov rdx, qword ptr [rbp - 0x20]
140a49b32: 4889d0  mov rax, rdx
140a49b35: 48f7d8  neg rax
140a49b38: 0f8194000000  jno 0x140a49bd2
140a49b3e: 448ab580000000  mov r14b, byte ptr [rbp + 0x80]
140a49b45: 0f108581000000  movups xmm0, xmmword ptr [rbp + 0x81]
140a49b4c: 0f2945c0  movaps xmmword ptr [rbp - 0x40], xmm0
140a49b50: 0f108590000000  movups xmm0, xmmword ptr [rbp + 0x90]
140a49b57: 0f1145cf  movups xmmword ptr [rbp - 0x31], xmm0
140a49b5b: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a49b62: 488d4d20  lea rcx, [rbp + 0x20]
140a49b66: e885e2e3ff  call 0x140887df0
140a49b6b: 488b85a8000000  mov rax, qword ptr [rbp + 0xa8]
140a49b72: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
140a49b76: e9e7fdffff  jmp 0x140a49962
140a49b7b: 885e01  mov byte ptr [rsi + 1], bl
140a49b7e: 31c0  xor eax, eax
140a49b80: 8806  mov byte ptr [rsi], al
140a49b82: 8b85a0000000  mov eax, dword ptr [rbp + 0xa0]
140a49b88: 83f803  cmp eax, 3
140a49b8b: 0f8562feffff  jne 0x140a499f3
140a49b91: eb2a  jmp 0x140a49bbd
140a49b93: 488b5528  mov rdx, qword ptr [rbp + 0x28]
140a49b97: 4885d2  test rdx, rdx
140a49b9a: 7421  je 0x140a49bbd
140a49b9c: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
140a49ba0: 41b801000000  mov r8d, 1
140a49ba6: e875a0bcff  call 0x140613c20
140a49bab: eb10  jmp 0x140a49bbd
140a49bad: 488d4d28  lea rcx, [rbp + 0x28]
140a49bb1: c685b700000000  mov byte ptr [rbp + 0xb7], 0
140a49bb8: e883f3e4ff  call 0x140898f40
140a49bbd: 4889f0  mov rax, rsi
140a49bc0: 4881c440010000  add rsp, 0x140
140a49bc7: 5b  pop rbx
140a49bc8: 5f  pop rdi
140a49bc9: 5e  pop rsi
140a49bca: 415c  pop r12
140a49bcc: 415e  pop r14
140a49bce: 415f  pop r15
140a49bd0: 5d  pop rbp
140a49bd1: c3  ret 
140a49bd2: 730f  jae 0x140a49be3
140a49bd4: 488b4de8  mov rcx, qword ptr [rbp - 0x18]
140a49bd8: 41b801000000  mov r8d, 1
140a49bde: e83da0bcff  call 0x140613c20
140a49be3: 0fb645f8  movzx eax, byte ptr [rbp - 8]
140a49be7: 83f803  cmp eax, 3
140a49bea: 7231  jb 0x140a49c1d
140a49bec: 7410  je 0x140a49bfe
140a49bee: 488d4d00  lea rcx, [rbp]
140a49bf2: 83f804  cmp eax, 4
140a49bf5: 7521  jne 0x140a49c18
140a49bf7: e894f7e3ff  call 0x140889390
140a49bfc: eb1f  jmp 0x140a49c1d
140a49bfe: 488b5500  mov rdx, qword ptr [rbp]
140a49c02: 4885d2  test rdx, rdx
140a49c05: 7416  je 0x140a49c1d
140a49c07: 488b4d08  mov rcx, qword ptr [rbp + 8]
140a49c0b: 41b801000000  mov r8d, 1
140a49c11: e80aa0bcff  call 0x140613c20
140a49c16: eb05  jmp 0x140a49c1d
140a49c18: e823f3e4ff  call 0x140898f40
140a49c1d: c645e00b  mov byte ptr [rbp - 0x20], 0xb
140a49c21: 488d1520480802  lea rdx, [rip + 0x2084820]
140a49c28: 4c8d0529480802  lea r8, [rip + 0x2084829]
140a49c2f: 488d4de0  lea rcx, [rbp - 0x20]
140a49c33: e808ffea01  call 0x1428f9b40
140a49c38: 48894608  mov qword ptr [rsi + 8], rax
140a49c3c: c60601  mov byte ptr [rsi], 1
140a49c3f: 0fb68580000000  movzx eax, byte ptr [rbp + 0x80]
140a49c46: 83f803  cmp eax, 3
140a49c49: 723a  jb 0x140a49c85
140a49c4b: 7413  je 0x140a49c60
140a49c4d: 488d8d88000000  lea rcx, [rbp + 0x88]
140a49c54: 83f804  cmp eax, 4
140a49c57: 7527  jne 0x140a49c80
140a49c59: e832f7e3ff  call 0x140889390
140a49c5e: eb25  jmp 0x140a49c85
140a49c60: 488b9588000000  mov rdx, qword ptr [rbp + 0x88]
140a49c67: 4885d2  test rdx, rdx
140a49c6a: 7419  je 0x140a49c85
140a49c6c: 488b8d90000000  mov rcx, qword ptr [rbp + 0x90]
140a49c73: 41b801000000  mov r8d, 1
140a49c79: e8a29fbcff  call 0x140613c20
140a49c7e: eb05  jmp 0x140a49c85
140a49c80: e8bbf2e4ff  call 0x140898f40
140a49c85: 488b95a8000000  mov rdx, qword ptr [rbp + 0xa8]
140a49c8c: 4885d2  test rdx, rdx
140a49c8f: 0f84a0fcffff  je 0x140a49935
140a49c95: 41b801000000  mov r8d, 1
140a49c9b: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
140a49c9f: e87c9fbcff  call 0x140613c20
140a49ca4: e98cfcffff  jmp 0x140a49935
140a49ca9: 0f1f8000000000  nop dword ptr [rax]
140a49cb0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a49cb5: 55  push rbp
140a49cb6: 4157  push r15
140a49cb8: 4156  push r14
140a49cba: 4154  push r12
140a49cbc: 56  push rsi
140a49cbd: 57  push rdi
140a49cbe: 53  push rbx
140a49cbf: 4883ec20  sub rsp, 0x20
140a49cc3: 488daa80000000  lea rbp, [rdx + 0x80]
140a49cca: 488d4d20  lea rcx, [rbp + 0x20]
140a49cce: e81de1e3ff  call 0x140887df0
140a49cd3: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a49cda: 4883c420  add rsp, 0x20
140a49cde: 5b  pop rbx
140a49cdf: 5f  pop rdi
140a49ce0: 5e  pop rsi
140a49ce1: 415c  pop r12
140a49ce3: 415e  pop r14
140a49ce5: 415f  pop r15
140a49ce7: 5d  pop rbp
140a49ce8: c3  ret 
140a49ce9: 0f1f8000000000  nop dword ptr [rax]
140a49cf0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a49cf5: 55  push rbp
140a49cf6: 4157  push r15
140a49cf8: 4156  push r14
140a49cfa: 4154  push r12
140a49cfc: 56  push rsi
140a49cfd: 57  push rdi
140a49cfe: 53  push rbx
140a49cff: 4883ec20  sub rsp, 0x20
140a49d03: 488daa80000000  lea rbp, [rdx + 0x80]
140a49d0a: 4883bda800000000  cmp qword ptr [rbp + 0xa8], 0
140a49d12: 7416  je 0x140a49d2a
140a49d14: 41b801000000  mov r8d, 1
140a49d1a: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
140a49d1e: 488b95a8000000  mov rdx, qword ptr [rbp + 0xa8]
140a49d25: e8f69ebcff  call 0x140613c20
140a49d2a: 90  nop 
140a49d2b: 4883c420  add rsp, 0x20
140a49d2f: 5b  pop rbx
140a49d30: 5f  pop rdi
140a49d31: 5e  pop rsi
140a49d32: 415c  pop r12
140a49d34: 415e  pop r14
140a49d36: 415f  pop r15
140a49d38: 5d  pop rbp
140a49d39: c3  ret 
140a49d3a: 660f1f440000  nop word ptr [rax + rax]
140a49d40: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a49d45: 55  push rbp
140a49d46: 4157  push r15
140a49d48: 4156  push r14
140a49d4a: 4154  push r12
140a49d4c: 56  push rsi
140a49d4d: 57  push rdi
140a49d4e: 53  push rbx
140a49d4f: 4883ec20  sub rsp, 0x20
140a49d53: 488daa80000000  lea rbp, [rdx + 0x80]
140a49d5a: 4883bda000000000  cmp qword ptr [rbp + 0xa0], 0
140a49d62: 741a  je 0x140a49d7e
140a49d64: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
140a49d6b: 48c1e205  shl rdx, 5
140a49d6f: 41b808000000  mov r8d, 8
140a49d75: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
140a49d79: e8a29ebcff  call 0x140613c20
140a49d7e: 90  nop 
140a49d7f: 4883c420  add rsp, 0x20
140a49d83: 5b  pop rbx
140a49d84: 5f  pop rdi
140a49d85: 5e  pop rsi
140a49d86: 415c  pop r12
140a49d88: 415e  pop r14
140a49d8a: 415f  pop r15
140a49d8c: 5d  pop rbp
140a49d8d: c3  ret 
140a49d8e: 6690  nop 
140a49d90: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a49d95: 55  push rbp
140a49d96: 4157  push r15
140a49d98: 4156  push r14
140a49d9a: 4154  push r12
140a49d9c: 56  push rsi
140a49d9d: 57  push rdi
140a49d9e: 53  push rbx
140a49d9f: 4883ec20  sub rsp, 0x20
140a49da3: 488daa80000000  lea rbp, [rdx + 0x80]
140a49daa: 488d4d20  lea rcx, [rbp + 0x20]
140a49dae: e8dd3ce1ff  call 0x14085da90
140a49db3: c685b700000000  mov byte ptr [rbp + 0xb7], 0
140a49dba: 4883c420  add rsp, 0x20
140a49dbe: 5b  pop rbx
140a49dbf: 5f  pop rdi
140a49dc0: 5e  pop rsi
140a49dc1: 415c  pop r12
140a49dc3: 415e  pop r14
140a49dc5: 415f  pop r15
140a49dc7: 5d  pop rbp
140a49dc8: c3  ret 
140a49dc9: 0f1f8000000000  nop dword ptr [rax]
140a49dd0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a49dd5: 55  push rbp
140a49dd6: 4157  push r15
140a49dd8: 4156  push r14
140a49dda: 4154  push r12
140a49ddc: 56  push rsi
140a49ddd: 57  push rdi
140a49dde: 53  push rbx
140a49ddf: 4883ec20  sub rsp, 0x20
140a49de3: 488daa80000000  lea rbp, [rdx + 0x80]
140a49dea: 488b5528  mov rdx, qword ptr [rbp + 0x28]
140a49dee: 4885d2  test rdx, rdx
140a49df1: 7416  je 0x140a49e09
140a49df3: 48c1e205  shl rdx, 5
140a49df7: 41b808000000  mov r8d, 8
140a49dfd: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
140a49e04: e8179ebcff  call 0x140613c20
140a49e09: c685b700000000  mov byte ptr [rbp + 0xb7], 0
140a49e10: 4883c420  add rsp, 0x20
140a49e14: 5b  pop rbx
140a49e15: 5f  pop rdi
140a49e16: 5e  pop rsi
140a49e17: 415c  pop r12
140a49e19: 415e  pop r14
140a49e1b: 415f  pop r15
140a49e1d: 5d  pop rbp
140a49e1e: c3  ret 
140a49e1f: 90  nop 
140a49e20: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a49e25: 55  push rbp
140a49e26: 4157  push r15
140a49e28: 4156  push r14
140a49e2a: 4154  push r12
140a49e2c: 56  push rsi
140a49e2d: 57  push rdi
140a49e2e: 53  push rbx
140a49e2f: 4883ec20  sub rsp, 0x20
140a49e33: 488daa80000000  lea rbp, [rdx + 0x80]
140a49e3a: 80bdb700000000  cmp byte ptr [rbp + 0xb7], 0
140a49e41: 7415  je 0x140a49e58
140a49e43: 80bda000000003  cmp byte ptr [rbp + 0xa0], 3
140a49e4a: 740c  je 0x140a49e58
140a49e4c: 0fb685a0000000  movzx eax, byte ptr [rbp + 0xa0]
140a49e53: 83f805  cmp eax, 5
140a49e56: 750f  jne 0x140a49e67
140a49e58: 4883c420  add rsp, 0x20
140a49e5c: 5b  pop rbx
140a49e5d: 5f  pop rdi
140a49e5e: 5e  pop rsi
140a49e5f: 415c  pop r12
140a49e61: 415e  pop r14
140a49e63: 415f  pop r15
140a49e65: 5d  pop rbp
140a49e66: c3  ret 
140a49e67: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
140a49e6b: e8203ce1ff  call 0x14085da90
140a49e70: ebe6  jmp 0x140a49e58
140a49e72: 66666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
140a49e80: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a49e85: 55  push rbp
140a49e86: 4157  push r15
140a49e88: 4156  push r14
140a49e8a: 4154  push r12
140a49e8c: 56  push rsi
140a49e8d: 57  push rdi
140a49e8e: 53  push rbx
140a49e8f: 4883ec20  sub rsp, 0x20
140a49e93: 488daa80000000  lea rbp, [rdx + 0x80]
140a49e9a: 488d8d80000000  lea rcx, [rbp + 0x80]
140a49ea1: e8ea3be1ff  call 0x14085da90
140a49ea6: 90  nop 
140a49ea7: 4883c420  add rsp, 0x20
140a49eab: 5b  pop rbx
140a49eac: 5f  pop rdi
140a49ead: 5e  pop rsi
140a49eae: 415c  pop r12
140a49eb0: 415e  pop r14
140a49eb2: 415f  pop r15
140a49eb4: 5d  pop rbp
140a49eb5: c3  ret 

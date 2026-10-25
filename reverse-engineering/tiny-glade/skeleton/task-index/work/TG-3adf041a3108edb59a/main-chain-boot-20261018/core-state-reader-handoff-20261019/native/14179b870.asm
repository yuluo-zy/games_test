14179b870: 4157  push r15
14179b872: 4156  push r14
14179b874: 4155  push r13
14179b876: 4154  push r12
14179b878: 56  push rsi
14179b879: 57  push rdi
14179b87a: 55  push rbp
14179b87b: 53  push rbx
14179b87c: 4883ec38  sub rsp, 0x38
14179b880: 4c89c7  mov rdi, r8
14179b883: 4889d3  mov rbx, rdx
14179b886: 4889ce  mov rsi, rcx
14179b889: 4983f80c  cmp r8, 0xc
14179b88d: 0f859e000000  jne 0x14179b931
14179b893: 48b8706c616e745f706f  movabs rax, 0x6f705f746e616c70
14179b89d: 483303  xor rax, qword ptr [rbx]
14179b8a0: 8b4b08  mov ecx, dword ptr [rbx + 8]
14179b8a3: 4881f1745f7631  xor rcx, 0x31765f74
14179b8aa: 4809c1  or rcx, rax
14179b8ad: 0f84c1000000  je 0x14179b974
14179b8b3: 48b87761746572696e67  movabs rax, 0x676e697265746177
14179b8bd: 483303  xor rax, qword ptr [rbx]
14179b8c0: 8b4b08  mov ecx, dword ptr [rbx + 8]
14179b8c3: 4881f15f63616e  xor rcx, 0x6e61635f
14179b8ca: 4809c1  or rcx, rax
14179b8cd: 0f84aa000000  je 0x14179b97d
14179b8d3: 488d15f0ec5301  lea rdx, [rip + 0x153ecf0]
14179b8da: 4889d9  mov rcx, rbx
14179b8dd: 4989f8  mov r8, rdi
14179b8e0: e80b831801  call 0x142923bf0
14179b8e5: 85c0  test eax, eax
14179b8e7: 0f8499000000  je 0x14179b986
14179b8ed: c744242c00000000  mov dword ptr [rsp + 0x2c], 0
14179b8f5: 4531ed  xor r13d, r13d
14179b8f8: 4531e4  xor r12d, r12d
14179b8fb: 31ed  xor ebp, ebp
14179b8fd: c744243000000000  mov dword ptr [rsp + 0x30], 0
14179b905: 4531ff  xor r15d, r15d
14179b908: 48b86661627269635f72  movabs rax, 0x725f636972626166
14179b912: 483303  xor rax, qword ptr [rbx]
14179b915: 8b4b08  mov ecx, dword ptr [rbx + 8]
14179b918: 4881f16f6c6c73  xor rcx, 0x736c6c6f
14179b91f: 4809c1  or rcx, rax
14179b922: 0f850c070000  jne 0x14179c034
14179b928: c646012d  mov byte ptr [rsi + 1], 0x2d
14179b92c: e908090000  jmp 0x14179c239
14179b931: 31c0  xor eax, eax
14179b933: 488d4ffc  lea rcx, [rdi - 4]
14179b937: 4883f910  cmp rcx, 0x10
14179b93b: 0f87d3030000  ja 0x14179bd14
14179b941: 488d1500f95301  lea rdx, [rip + 0x153f900]
14179b948: 48630c8a  movsxd rcx, dword ptr [rdx + rcx*4]
14179b94c: 4801d1  add rcx, rdx
14179b94f: 31ed  xor ebp, ebp
14179b951: ffe1  jmp rcx
14179b953: 813b63617274  cmp dword ptr [rbx], 0x74726163
14179b959: 0f84e3040000  je 0x14179be42
14179b95f: 813b72656564  cmp dword ptr [rbx], 0x64656572
14179b965: 0f85f4020000  jne 0x14179bc5f
14179b96b: c646011e  mov byte ptr [rsi + 1], 0x1e
14179b96f: e9c5080000  jmp 0x14179c239
14179b974: c6460100  mov byte ptr [rsi + 1], 0
14179b978: e9bc080000  jmp 0x14179c239
14179b97d: c6460106  mov byte ptr [rsi + 1], 6
14179b981: e9b3080000  jmp 0x14179c239
14179b986: c646010f  mov byte ptr [rsi + 1], 0xf
14179b98a: e9aa080000  jmp 0x14179c239
14179b98f: b86c616464  mov eax, 0x6464616c
14179b994: 3303  xor eax, dword ptr [rbx]
14179b996: 0fb74b04  movzx ecx, word ptr [rbx + 4]
14179b99a: 81f165720000  xor ecx, 0x7265
14179b9a0: 09c1  or ecx, eax
14179b9a2: 0f8488040000  je 0x14179be30
14179b9a8: b873686f76  mov eax, 0x766f6873
14179b9ad: 3303  xor eax, dword ptr [rbx]
14179b9af: 0fb74b04  movzx ecx, word ptr [rbx + 4]
14179b9b3: 81f1656c0000  xor ecx, 0x6c65
14179b9b9: 09c1  or ecx, eax
14179b9bb: 0f840a060000  je 0x14179bfcb
14179b9c1: b86275636b  mov eax, 0x6b637562
14179b9c6: 3303  xor eax, dword ptr [rbx]
14179b9c8: 0fb74b04  movzx ecx, word ptr [rbx + 4]
14179b9cc: 81f165740000  xor ecx, 0x7465
14179b9d2: 09c1  or ecx, eax
14179b9d4: 0f84fb070000  je 0x14179c1d5
14179b9da: 488d15feeb5301  lea rdx, [rip + 0x153ebfe]
14179b9e1: 4889d9  mov rcx, rbx
14179b9e4: 4989f8  mov r8, rdi
14179b9e7: e804821801  call 0x142923bf0
14179b9ec: 85c0  test eax, eax
14179b9ee: 0f851e030000  jne 0x14179bd12
14179b9f4: c6460111  mov byte ptr [rsi + 1], 0x11
14179b9f8: e93c080000  jmp 0x14179c239
14179b9fd: 48b8626f785f775f7374  movabs rax, 0x74735f775f786f62
14179ba07: 483303  xor rax, qword ptr [rbx]
14179ba0a: 48b95f775f7374756666  movabs rcx, 0x66667574735f775f
14179ba14: 48334b03  xor rcx, qword ptr [rbx + 3]
14179ba18: 4809c1  or rcx, rax
14179ba1b: 0f8418040000  je 0x14179be39
14179ba21: 48b862656e63685f7368  movabs rax, 0x68735f68636e6562
14179ba2b: 483303  xor rax, qword ptr [rbx]
14179ba2e: 48b963685f73686f7274  movabs rcx, 0x74726f68735f6863
14179ba38: 48334b03  xor rcx, qword ptr [rbx + 3]
14179ba3c: 4809c1  or rcx, rax
14179ba3f: 0f848f050000  je 0x14179bfd4
14179ba45: 488d15baeb5301  lea rdx, [rip + 0x153ebba]
14179ba4c: 4889d9  mov rcx, rbx
14179ba4f: 4989f8  mov r8, rdi
14179ba52: e899811801  call 0x142923bf0
14179ba57: 85c0  test eax, eax
14179ba59: 0f847c070000  je 0x14179c1db
14179ba5f: 41b701  mov r15b, 1
14179ba62: 31d2  xor edx, edx
14179ba64: 4531f6  xor r14d, r14d
14179ba67: e936040000  jmp 0x14179bea2
14179ba6c: b877686565  mov eax, 0x65656877
14179ba71: 3303  xor eax, dword ptr [rbx]
14179ba73: 0fb64b04  movzx ecx, byte ptr [rbx + 4]
14179ba77: 83f16c  xor ecx, 0x6c
14179ba7a: 09c1  or ecx, eax
14179ba7c: 0f84c9030000  je 0x14179be4b
14179ba82: b862656e63  mov eax, 0x636e6562
14179ba87: 3303  xor eax, dword ptr [rbx]
14179ba89: 0fb64b04  movzx ecx, byte ptr [rbx + 4]
14179ba8d: 83f168  xor ecx, 0x68
14179ba90: 09c1  or ecx, eax
14179ba92: 0f8445050000  je 0x14179bfdd
14179ba98: 488d1588eb5301  lea rdx, [rip + 0x153eb88]
14179ba9f: 4889d9  mov rcx, rbx
14179baa2: 4989f8  mov r8, rdi
14179baa5: e846811801  call 0x142923bf0
14179baaa: 85c0  test eax, eax
14179baac: 0f842f070000  je 0x14179c1e1
14179bab2: b201  mov dl, 1
14179bab4: 31ed  xor ebp, ebp
14179bab6: e9d4020000  jmp 0x14179bd8f
14179babb: 48b86261736b65745f77  movabs rax, 0x775f74656b736162
14179bac5: 483303  xor rax, qword ptr [rbx]
14179bac8: 48b9745f775f666f6f64  movabs rcx, 0x646f6f665f775f74
14179bad2: 48334b05  xor rcx, qword ptr [rbx + 5]
14179bad6: 4809c1  or rcx, rax
14179bad9: 0f8475030000  je 0x14179be54
14179badf: 40b501  mov bpl, 1
14179bae2: 31c0  xor eax, eax
14179bae4: 4883ff10  cmp rdi, 0x10
14179bae8: 0f8532020000  jne 0x14179bd20
14179baee: f30f6f03  movdqu xmm0, xmmword ptr [rbx]
14179baf2: 660f740546e95301  pcmpeqb xmm0, xmmword ptr [rip + 0x153e946]
14179bafa: 660fd7c8  pmovmskb ecx, xmm0
14179bafe: 81f9ffff0000  cmp ecx, 0xffff
14179bb04: 0f84be020000  je 0x14179bdc8
14179bb0a: f30f6f03  movdqu xmm0, xmmword ptr [rbx]
14179bb0e: 660f74053ae95301  pcmpeqb xmm0, xmmword ptr [rip + 0x153e93a]
14179bb16: 660fd7c8  pmovmskb ecx, xmm0
14179bb1a: 81f9ffff0000  cmp ecx, 0xffff
14179bb20: 0f8464030000  je 0x14179be8a
14179bb26: f30f6f03  movdqu xmm0, xmmword ptr [rbx]
14179bb2a: 660f7405fee85301  pcmpeqb xmm0, xmmword ptr [rip + 0x153e8fe]
14179bb32: 660fd7c8  pmovmskb ecx, xmm0
14179bb36: 81f9ffff0000  cmp ecx, 0xffff
14179bb3c: 0f847b060000  je 0x14179c1bd
14179bb42: b101  mov cl, 1
14179bb44: 4531ff  xor r15d, r15d
14179bb47: c744243000000000  mov dword ptr [rsp + 0x30], 0
14179bb4f: c744243400000000  mov dword ptr [rsp + 0x34], 0
14179bb57: 41bc00000000  mov r12d, 0
14179bb5d: 41bd00000000  mov r13d, 0
14179bb63: 84c0  test al, al
14179bb65: 0f85b4030000  jne 0x14179bf1f
14179bb6b: e9da030000  jmp 0x14179bf4a
14179bb70: 48b862656e63685f775f  movabs rax, 0x5f775f68636e6562
14179bb7a: 483303  xor rax, qword ptr [rbx]
14179bb7d: 48b9775f6261736b6574  movabs rcx, 0x74656b7361625f77
14179bb87: 48334b06  xor rcx, qword ptr [rbx + 6]
14179bb8b: 4809c1  or rcx, rax
14179bb8e: 0f84c9020000  je 0x14179be5d
14179bb94: 488d1591ea5301  lea rdx, [rip + 0x153ea91]
14179bb9b: 4889d9  mov rcx, rbx
14179bb9e: 4989f8  mov r8, rdi
14179bba1: e84a801801  call 0x142923bf0
14179bba6: 85c0  test eax, eax
14179bba8: 0f8438040000  je 0x14179bfe6
14179bbae: 488d15a5ea5301  lea rdx, [rip + 0x153eaa5]
14179bbb5: 4889d9  mov rcx, rbx
14179bbb8: 4989f8  mov r8, rdi
14179bbbb: e830801801  call 0x142923bf0
14179bbc0: 85c0  test eax, eax
14179bbc2: 0f841f060000  je 0x14179c1e7
14179bbc8: 488d15d9ea5301  lea rdx, [rip + 0x153ead9]
14179bbcf: 4889d9  mov rcx, rbx
14179bbd2: 4989f8  mov r8, rdi
14179bbd5: e816801801  call 0x142923bf0
14179bbda: 85c0  test eax, eax
14179bbdc: 0f843b060000  je 0x14179c21d
14179bbe2: 488d15ecea5301  lea rdx, [rip + 0x153eaec]
14179bbe9: 4889d9  mov rcx, rbx
14179bbec: 4989f8  mov r8, rdi
14179bbef: e8fc7f1801  call 0x142923bf0
14179bbf4: 85c0  test eax, eax
14179bbf6: 0f8555060000  jne 0x14179c251
14179bbfc: c6460127  mov byte ptr [rsi + 1], 0x27
14179bc00: e934060000  jmp 0x14179c239
14179bc05: 48b866697265776f6f64  movabs rax, 0x646f6f7765726966
14179bc0f: 483903  cmp qword ptr [rbx], rax
14179bc12: 0f844e020000  je 0x14179be66
14179bc18: 48b8626972646e657374  movabs rax, 0x7473656e64726962
14179bc22: 483903  cmp qword ptr [rbx], rax
14179bc25: 0f84c4030000  je 0x14179bfef
14179bc2b: 488d1590e95301  lea rdx, [rip + 0x153e990]
14179bc32: 4889d9  mov rcx, rbx
14179bc35: 4989f8  mov r8, rdi
14179bc38: e8b37f1801  call 0x142923bf0
14179bc3d: 85c0  test eax, eax
14179bc3f: 0f84a8050000  je 0x14179c1ed
14179bc45: 488d1581ea5301  lea rdx, [rip + 0x153ea81]
14179bc4c: 4889d9  mov rcx, rbx
14179bc4f: 4989f8  mov r8, rdi
14179bc52: e8997f1801  call 0x142923bf0
14179bc57: 85c0  test eax, eax
14179bc59: 0f84c4050000  je 0x14179c223
14179bc5f: 4531ff  xor r15d, r15d
14179bc62: 4531f6  xor r14d, r14d
14179bc65: 31d2  xor edx, edx
14179bc67: e967020000  jmp 0x14179bed3
14179bc6c: 48b87069746368666f72  movabs rax, 0x726f666863746970
14179bc76: 483303  xor rax, qword ptr [rbx]
14179bc79: 0fb64b08  movzx ecx, byte ptr [rbx + 8]
14179bc7d: 4883f16b  xor rcx, 0x6b
14179bc81: 4809c1  or rcx, rax
14179bc84: 0f84e5010000  je 0x14179be6f
14179bc8a: 48b862697264686f7573  movabs rax, 0x73756f6864726962
14179bc94: 483303  xor rax, qword ptr [rbx]
14179bc97: 0fb64b08  movzx ecx, byte ptr [rbx + 8]
14179bc9b: 4883f165  xor rcx, 0x65
14179bc9f: 4809c1  or rcx, rax
14179bca2: 0f8450030000  je 0x14179bff8
14179bca8: 488d1562e95301  lea rdx, [rip + 0x153e962]
14179bcaf: 4889d9  mov rcx, rbx
14179bcb2: 4989f8  mov r8, rdi
14179bcb5: e8367f1801  call 0x142923bf0
14179bcba: 85c0  test eax, eax
14179bcbc: 0f8431050000  je 0x14179c1f3
14179bcc2: 488d1551e95301  lea rdx, [rip + 0x153e951]
14179bcc9: 4889d9  mov rcx, rbx
14179bccc: 4989f8  mov r8, rdi
14179bccf: e81c7f1801  call 0x142923bf0
14179bcd4: 85c0  test eax, eax
14179bcd6: 0f844d050000  je 0x14179c229
14179bcdc: 41b601  mov r14b, 1
14179bcdf: 31d2  xor edx, edx
14179bce1: e9b9010000  jmp 0x14179be9f
14179bce6: f30f6f03  movdqu xmm0, xmmword ptr [rbx]
14179bcea: 660f6e4b10  movd xmm1, dword ptr [rbx + 0x10]
14179bcef: 660f740df9e65301  pcmpeqb xmm1, xmmword ptr [rip + 0x153e6f9]
14179bcf7: 660f740501e75301  pcmpeqb xmm0, xmmword ptr [rip + 0x153e701]
14179bcff: 660fdbc1  pand xmm0, xmm1
14179bd03: 660fd7c0  pmovmskb eax, xmm0
14179bd07: 3dffff0000  cmp eax, 0xffff
14179bd0c: 0f8466010000  je 0x14179be78
14179bd12: 31c0  xor eax, eax
14179bd14: 31ed  xor ebp, ebp
14179bd16: 4883ff10  cmp rdi, 0x10
14179bd1a: 0f84cefdffff  je 0x14179baee
14179bd20: 84c0  test al, al
14179bd22: 7469  je 0x14179bd8d
14179bd24: 488d153de95301  lea rdx, [rip + 0x153e93d]
14179bd2b: 4889d9  mov rcx, rbx
14179bd2e: 4989f8  mov r8, rdi
14179bd31: e8ba7e1801  call 0x142923bf0
14179bd36: 85c0  test eax, eax
14179bd38: 0f8455010000  je 0x14179be93
14179bd3e: 41b501  mov r13b, 1
14179bd41: 4531ff  xor r15d, r15d
14179bd44: 4531f6  xor r14d, r14d
14179bd47: 31d2  xor edx, edx
14179bd49: e98a010000  jmp 0x14179bed8
14179bd4e: f30f6f03  movdqu xmm0, xmmword ptr [rbx]
14179bd52: 0fb64310  movzx eax, byte ptr [rbx + 0x10]
14179bd56: 660f6ec8  movd xmm1, eax
14179bd5a: 660f7405aee65301  pcmpeqb xmm0, xmmword ptr [rip + 0x153e6ae]
14179bd62: 660f740df61b1a01  pcmpeqb xmm1, xmmword ptr [rip + 0x11a1bf6]
14179bd6a: 660fdbc8  pand xmm1, xmm0
14179bd6e: 660fd7c1  pmovmskb eax, xmm1
14179bd72: 3dffff0000  cmp eax, 0xffff
14179bd77: 0f8404010000  je 0x14179be81
14179bd7d: b001  mov al, 1
14179bd7f: 31ed  xor ebp, ebp
14179bd81: 4883ff10  cmp rdi, 0x10
14179bd85: 0f8463fdffff  je 0x14179baee
14179bd8b: eb93  jmp 0x14179bd20
14179bd8d: 31d2  xor edx, edx
14179bd8f: 4883ff07  cmp rdi, 7
14179bd93: 753c  jne 0x14179bdd1
14179bd95: b86c696c79  mov eax, 0x796c696c
14179bd9a: 3303  xor eax, dword ptr [rbx]
14179bd9c: b979706164  mov ecx, 0x64617079
14179bda1: 334b03  xor ecx, dword ptr [rbx + 3]
14179bda4: 09c1  or ecx, eax
14179bda6: 0f841d040000  je 0x14179c1c9
14179bdac: b870756d70  mov eax, 0x706d7570
14179bdb1: 3303  xor eax, dword ptr [rbx]
14179bdb3: b9706b696e  mov ecx, 0x6e696b70
14179bdb8: 334b03  xor ecx, dword ptr [rbx + 3]
14179bdbb: 09c1  or ecx, eax
14179bdbd: 7566  jne 0x14179be25
14179bdbf: c6460125  mov byte ptr [rsi + 1], 0x25
14179bdc3: e971040000  jmp 0x14179c239
14179bdc8: c646011a  mov byte ptr [rsi + 1], 0x1a
14179bdcc: e968040000  jmp 0x14179c239
14179bdd1: 4084ed  test bpl, bpl
14179bdd4: 0f84c2000000  je 0x14179be9c
14179bdda: 48b866616c6c656e5f6c  movabs rax, 0x6c5f6e656c6c6166
14179bde4: 483303  xor rax, qword ptr [rbx]
14179bde7: 48b96e5f6c6561766573  movabs rcx, 0x73657661656c5f6e
14179bdf1: 48334b05  xor rcx, qword ptr [rbx + 5]
14179bdf5: 4809c1  or rcx, rax
14179bdf8: 0f8419040000  je 0x14179c217
14179bdfe: 48b866616c6c656e5f70  movabs rax, 0x705f6e656c6c6166
14179be08: 483303  xor rax, qword ptr [rbx]
14179be0b: 48b96e5f706574616c73  movabs rcx, 0x736c617465705f6e
14179be15: 48334b05  xor rcx, qword ptr [rbx + 5]
14179be19: 4809c1  or rcx, rax
14179be1c: 0f8413040000  je 0x14179c235
14179be22: 40b501  mov bpl, 1
14179be25: 4531ff  xor r15d, r15d
14179be28: 4531f6  xor r14d, r14d
14179be2b: e9a5000000  jmp 0x14179bed5
14179be30: c6460101  mov byte ptr [rsi + 1], 1
14179be34: e900040000  jmp 0x14179c239
14179be39: c6460105  mov byte ptr [rsi + 1], 5
14179be3d: e9f7030000  jmp 0x14179c239
14179be42: c6460117  mov byte ptr [rsi + 1], 0x17
14179be46: e9ee030000  jmp 0x14179c239
14179be4b: c6460102  mov byte ptr [rsi + 1], 2
14179be4f: e9e5030000  jmp 0x14179c239
14179be54: c6460112  mov byte ptr [rsi + 1], 0x12
14179be58: e9dc030000  jmp 0x14179c239
14179be5d: c646010b  mov byte ptr [rsi + 1], 0xb
14179be61: e9d3030000  jmp 0x14179c239
14179be66: c6460107  mov byte ptr [rsi + 1], 7
14179be6a: e9ca030000  jmp 0x14179c239
14179be6f: c6460103  mov byte ptr [rsi + 1], 3
14179be73: e9c1030000  jmp 0x14179c239
14179be78: c6460113  mov byte ptr [rsi + 1], 0x13
14179be7c: e9b8030000  jmp 0x14179c239
14179be81: c646010d  mov byte ptr [rsi + 1], 0xd
14179be85: e9af030000  jmp 0x14179c239
14179be8a: c646011b  mov byte ptr [rsi + 1], 0x1b
14179be8e: e9a6030000  jmp 0x14179c239
14179be93: c646011d  mov byte ptr [rsi + 1], 0x1d
14179be97: e99d030000  jmp 0x14179c239
14179be9c: 4531f6  xor r14d, r14d
14179be9f: 4531ff  xor r15d, r15d
14179bea2: 4883ff0a  cmp rdi, 0xa
14179bea6: 752b  jne 0x14179bed3
14179bea8: 48b86f6c64656e5f726f  movabs rax, 0x6f725f6e65646c6f
14179beb2: 483303  xor rax, qword ptr [rbx]
14179beb5: 0fb74b08  movzx ecx, word ptr [rbx + 8]
14179beb9: 4881f1636b0000  xor rcx, 0x6b63
14179bec0: 4809c1  or rcx, rax
14179bec3: 0f843c030000  je 0x14179c205
14179bec9: 41b401  mov r12b, 1
14179becc: 31ed  xor ebp, ebp
14179bece: 4531ed  xor r13d, r13d
14179bed1: eb08  jmp 0x14179bedb
14179bed3: 31ed  xor ebp, ebp
14179bed5: 4531ed  xor r13d, r13d
14179bed8: 4531e4  xor r12d, r12d
14179bedb: 4883ff0f  cmp rdi, 0xf
14179bedf: 752e  jne 0x14179bf0f
14179bee1: 48b8636172745f775f70  movabs rax, 0x705f775f74726163
14179beeb: 483303  xor rax, qword ptr [rbx]
14179beee: 48b970756d706b696e73  movabs rcx, 0x736e696b706d7570
14179bef8: 48334b07  xor rcx, qword ptr [rbx + 7]
14179befc: 4809c1  or rcx, rax
14179beff: 0f84b4000000  je 0x14179bfb9
14179bf05: 4489742430  mov dword ptr [rsp + 0x30], r14d
14179bf0a: e9f5000000  jmp 0x14179c004
14179bf0f: 4584ed  test r13b, r13b
14179bf12: 4489742430  mov dword ptr [rsp + 0x30], r14d
14179bf17: 7463  je 0x14179bf7c
14179bf19: 89542434  mov dword ptr [rsp + 0x34], edx
14179bf1d: 31c9  xor ecx, ecx
14179bf1f: 4589e6  mov r14d, r12d
14179bf22: 4189cc  mov r12d, ecx
14179bf25: 488d15c6e75301  lea rdx, [rip + 0x153e7c6]
14179bf2c: 4889d9  mov rcx, rbx
14179bf2f: 4989f8  mov r8, rdi
14179bf32: e8b97c1801  call 0x142923bf0
14179bf37: 85c0  test eax, eax
14179bf39: 0f8483000000  je 0x14179bfc2
14179bf3f: 41b501  mov r13b, 1
14179bf42: 4584e4  test r12b, r12b
14179bf45: 4589f4  mov r12d, r14d
14179bf48: 741a  je 0x14179bf64
14179bf4a: 488d15c9e75301  lea rdx, [rip + 0x153e7c9]
14179bf51: 4889d9  mov rcx, rbx
14179bf54: 4989f8  mov r8, rdi
14179bf57: e8947c1801  call 0x142923bf0
14179bf5c: 85c0  test eax, eax
14179bf5e: 0f8495020000  je 0x14179c1f9
14179bf64: c744242c00000000  mov dword ptr [rsp + 0x2c], 0
14179bf6c: 807c243400  cmp byte ptr [rsp + 0x34], 0
14179bf71: 0f8599000000  jne 0x14179c010
14179bf77: e9ae000000  jmp 0x14179c02a
14179bf7c: 4883ff17  cmp rdi, 0x17
14179bf80: 757f  jne 0x14179c001
14179bf82: f30f6f03  movdqu xmm0, xmmword ptr [rbx]
14179bf86: f30f6f4b07  movdqu xmm1, xmmword ptr [rbx + 7]
14179bf8b: 660f740d8de45301  pcmpeqb xmm1, xmmword ptr [rip + 0x153e48d]
14179bf93: 660f740595e45301  pcmpeqb xmm0, xmmword ptr [rip + 0x153e495]
14179bf9b: 660fdbc1  pand xmm0, xmm1
14179bf9f: 660fd7c0  pmovmskb eax, xmm0
14179bfa3: 3dffff0000  cmp eax, 0xffff
14179bfa8: 0f8481020000  je 0x14179c22f
14179bfae: b001  mov al, 1
14179bfb0: 8944242c  mov dword ptr [rsp + 0x2c], eax
14179bfb4: 4531ed  xor r13d, r13d
14179bfb7: eb53  jmp 0x14179c00c
14179bfb9: c6460128  mov byte ptr [rsi + 1], 0x28
14179bfbd: e977020000  jmp 0x14179c239
14179bfc2: c6460129  mov byte ptr [rsi + 1], 0x29
14179bfc6: e96e020000  jmp 0x14179c239
14179bfcb: c6460104  mov byte ptr [rsi + 1], 4
14179bfcf: e965020000  jmp 0x14179c239
14179bfd4: c646010c  mov byte ptr [rsi + 1], 0xc
14179bfd8: e95c020000  jmp 0x14179c239
14179bfdd: c646010a  mov byte ptr [rsi + 1], 0xa
14179bfe1: e953020000  jmp 0x14179c239
14179bfe6: c6460119  mov byte ptr [rsi + 1], 0x19
14179bfea: e94a020000  jmp 0x14179c239
14179bfef: c6460109  mov byte ptr [rsi + 1], 9
14179bff3: e941020000  jmp 0x14179c239
14179bff8: c6460110  mov byte ptr [rsi + 1], 0x10
14179bffc: e938020000  jmp 0x14179c239
14179c001: 4531ed  xor r13d, r13d
14179c004: c744242c00000000  mov dword ptr [rsp + 0x2c], 0
14179c00c: 84d2  test dl, dl
14179c00e: 741a  je 0x14179c02a
14179c010: 488d1513e75301  lea rdx, [rip + 0x153e713]
14179c017: 4889d9  mov rcx, rbx
14179c01a: 4989f8  mov r8, rdi
14179c01d: e8ce7b1801  call 0x142923bf0
14179c022: 85c0  test eax, eax
14179c024: 0f848d010000  je 0x14179c1b7
14179c02a: 4883ff0c  cmp rdi, 0xc
14179c02e: 0f84d4f8ffff  je 0x14179b908
14179c034: 4584ff  test r15b, r15b
14179c037: 741a  je 0x14179c053
14179c039: 488d15fbe65301  lea rdx, [rip + 0x153e6fb]
14179c040: 4889d9  mov rcx, rbx
14179c043: 4989f8  mov r8, rdi
14179c046: e8a57b1801  call 0x142923bf0
14179c04b: 85c0  test eax, eax
14179c04d: 0f8440010000  je 0x14179c193
14179c053: 4084ed  test bpl, bpl
14179c056: 7424  je 0x14179c07c
14179c058: 48b87374616c6c5f7665  movabs rax, 0x65765f6c6c617473
14179c062: 483303  xor rax, qword ptr [rbx]
14179c065: 48b95f76656767696573  movabs rcx, 0x736569676765765f
14179c06f: 48334b05  xor rcx, qword ptr [rbx + 5]
14179c073: 4809c1  or rcx, rax
14179c076: 0f8420010000  je 0x14179c19c
14179c07c: 4584e4  test r12b, r12b
14179c07f: 741a  je 0x14179c09b
14179c081: 488d15cbe65301  lea rdx, [rip + 0x153e6cb]
14179c088: 4889d9  mov rcx, rbx
14179c08b: 4989f8  mov r8, rdi
14179c08e: e85d7b1801  call 0x142923bf0
14179c093: 85c0  test eax, eax
14179c095: 0f840a010000  je 0x14179c1a5
14179c09b: 4584ff  test r15b, r15b
14179c09e: 741a  je 0x14179c0ba
14179c0a0: 488d15b6e65301  lea rdx, [rip + 0x153e6b6]
14179c0a7: 4889d9  mov rcx, rbx
14179c0aa: 4989f8  mov r8, rdi
14179c0ad: e83e7b1801  call 0x142923bf0
14179c0b2: 85c0  test eax, eax
14179c0b4: 0f84f4000000  je 0x14179c1ae
14179c0ba: 807c243000  cmp byte ptr [rsp + 0x30], 0
14179c0bf: 7434  je 0x14179c0f5
14179c0c1: 488d15a0e65301  lea rdx, [rip + 0x153e6a0]
14179c0c8: 4889d9  mov rcx, rbx
14179c0cb: 4989f8  mov r8, rdi
14179c0ce: e81d7b1801  call 0x142923bf0
14179c0d3: 85c0  test eax, eax
14179c0d5: 0f84e8000000  je 0x14179c1c3
14179c0db: 488d158fe65301  lea rdx, [rip + 0x153e68f]
14179c0e2: 4889d9  mov rcx, rbx
14179c0e5: 4989f8  mov r8, rdi
14179c0e8: e8037b1801  call 0x142923bf0
14179c0ed: 85c0  test eax, eax
14179c0ef: 0f840a010000  je 0x14179c1ff
14179c0f5: 4584e4  test r12b, r12b
14179c0f8: 741a  je 0x14179c114
14179c0fa: 488d1579e65301  lea rdx, [rip + 0x153e679]
14179c101: 4889d9  mov rcx, rbx
14179c104: 4989f8  mov r8, rdi
14179c107: e8e47a1801  call 0x142923bf0
14179c10c: 85c0  test eax, eax
14179c10e: 0f84bb000000  je 0x14179c1cf
14179c114: 4584ed  test r13b, r13b
14179c117: 741a  je 0x14179c133
14179c119: 488d1564e65301  lea rdx, [rip + 0x153e664]
14179c120: 4889d9  mov rcx, rbx
14179c123: 4989f8  mov r8, rdi
14179c126: e8c57a1801  call 0x142923bf0
14179c12b: 85c0  test eax, eax
14179c12d: 0f84d8000000  je 0x14179c20b
14179c133: 807c242c00  cmp byte ptr [rsp + 0x2c], 0
14179c138: 741a  je 0x14179c154
14179c13a: 488d1554e65301  lea rdx, [rip + 0x153e654]
14179c141: 4889d9  mov rcx, rbx
14179c144: 4989f8  mov r8, rdi
14179c147: e8a47a1801  call 0x142923bf0
14179c14c: 85c0  test eax, eax
14179c14e: 0f84bd000000  je 0x14179c211
14179c154: 4883ff19  cmp rdi, 0x19
14179c158: 0f85f3000000  jne 0x14179c251
14179c15e: f30f6f03  movdqu xmm0, xmmword ptr [rbx]
14179c162: f30f6f4b09  movdqu xmm1, xmmword ptr [rbx + 9]
14179c167: 660f740d113b3001  pcmpeqb xmm1, xmmword ptr [rip + 0x1303b11]
14179c16f: 660f7405e9e25301  pcmpeqb xmm0, xmmword ptr [rip + 0x153e2e9]
14179c177: 660fdbc1  pand xmm0, xmm1
14179c17b: 660fd7c0  pmovmskb eax, xmm0
14179c17f: 3dffff0000  cmp eax, 0xffff
14179c184: 0f85c7000000  jne 0x14179c251
14179c18a: c6460137  mov byte ptr [rsi + 1], 0x37
14179c18e: e9a6000000  jmp 0x14179c239
14179c193: c646012e  mov byte ptr [rsi + 1], 0x2e
14179c197: e99d000000  jmp 0x14179c239
14179c19c: c646012f  mov byte ptr [rsi + 1], 0x2f
14179c1a0: e994000000  jmp 0x14179c239
14179c1a5: c6460130  mov byte ptr [rsi + 1], 0x30
14179c1a9: e98b000000  jmp 0x14179c239
14179c1ae: c6460131  mov byte ptr [rsi + 1], 0x31
14179c1b2: e982000000  jmp 0x14179c239
14179c1b7: c646012c  mov byte ptr [rsi + 1], 0x2c
14179c1bb: eb7c  jmp 0x14179c239
14179c1bd: c6460124  mov byte ptr [rsi + 1], 0x24
14179c1c1: eb76  jmp 0x14179c239
14179c1c3: c6460132  mov byte ptr [rsi + 1], 0x32
14179c1c7: eb70  jmp 0x14179c239
14179c1c9: c646011f  mov byte ptr [rsi + 1], 0x1f
14179c1cd: eb6a  jmp 0x14179c239
14179c1cf: c6460134  mov byte ptr [rsi + 1], 0x34
14179c1d3: eb64  jmp 0x14179c239
14179c1d5: c6460108  mov byte ptr [rsi + 1], 8
14179c1d9: eb5e  jmp 0x14179c239
14179c1db: c6460114  mov byte ptr [rsi + 1], 0x14
14179c1df: eb58  jmp 0x14179c239
14179c1e1: c6460118  mov byte ptr [rsi + 1], 0x18
14179c1e5: eb52  jmp 0x14179c239
14179c1e7: c646011c  mov byte ptr [rsi + 1], 0x1c
14179c1eb: eb4c  jmp 0x14179c239
14179c1ed: c646010e  mov byte ptr [rsi + 1], 0xe
14179c1f1: eb46  jmp 0x14179c239
14179c1f3: c6460115  mov byte ptr [rsi + 1], 0x15
14179c1f7: eb40  jmp 0x14179c239
14179c1f9: c646012b  mov byte ptr [rsi + 1], 0x2b
14179c1fd: eb3a  jmp 0x14179c239
14179c1ff: c6460133  mov byte ptr [rsi + 1], 0x33
14179c203: eb34  jmp 0x14179c239
14179c205: c6460122  mov byte ptr [rsi + 1], 0x22
14179c209: eb2e  jmp 0x14179c239
14179c20b: c6460135  mov byte ptr [rsi + 1], 0x35
14179c20f: eb28  jmp 0x14179c239
14179c211: c6460136  mov byte ptr [rsi + 1], 0x36
14179c215: eb22  jmp 0x14179c239
14179c217: c6460120  mov byte ptr [rsi + 1], 0x20
14179c21b: eb1c  jmp 0x14179c239
14179c21d: c6460123  mov byte ptr [rsi + 1], 0x23
14179c221: eb16  jmp 0x14179c239
14179c223: c6460126  mov byte ptr [rsi + 1], 0x26
14179c227: eb10  jmp 0x14179c239
14179c229: c6460116  mov byte ptr [rsi + 1], 0x16
14179c22d: eb0a  jmp 0x14179c239
14179c22f: c646012a  mov byte ptr [rsi + 1], 0x2a
14179c233: eb04  jmp 0x14179c239
14179c235: c6460121  mov byte ptr [rsi + 1], 0x21
14179c239: 31c0  xor eax, eax
14179c23b: 8806  mov byte ptr [rsi], al
14179c23d: 4889f0  mov rax, rsi
14179c240: 4883c438  add rsp, 0x38
14179c244: 5b  pop rbx
14179c245: 5d  pop rbp
14179c246: 5f  pop rdi
14179c247: 5e  pop rsi
14179c248: 415c  pop r12
14179c24a: 415d  pop r13
14179c24c: 415e  pop r14
14179c24e: 415f  pop r15
14179c250: c3  ret 
14179c251: 4c8d0570e55301  lea r8, [rip + 0x153e570]
14179c258: 41b938000000  mov r9d, 0x38
14179c25e: 4889d9  mov rcx, rbx
14179c261: 4889fa  mov rdx, rdi
14179c264: e8b7db1501  call 0x1428f9e20
14179c269: 48894608  mov qword ptr [rsi + 8], rax
14179c26d: b001  mov al, 1
14179c26f: ebca  jmp 0x14179c23b

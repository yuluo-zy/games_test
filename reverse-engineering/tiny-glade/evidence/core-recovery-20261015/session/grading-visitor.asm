0x1415ce710  push     rbp
0x1415ce711  push     r15
0x1415ce713  push     r14
0x1415ce715  push     r13
0x1415ce717  push     r12
0x1415ce719  push     rsi
0x1415ce71a  push     rdi
0x1415ce71b  push     rbx
0x1415ce71c  sub      rsp, 0x188
0x1415ce723  lea      rbp, [rsp + 0x80]
0x1415ce72b  movaps   xmmword ptr [rbp + 0xf0], xmm9
0x1415ce733  movaps   xmmword ptr [rbp + 0xe0], xmm8
0x1415ce73b  movaps   xmmword ptr [rbp + 0xd0], xmm7
0x1415ce742  movaps   xmmword ptr [rbp + 0xc0], xmm6
0x1415ce749  mov      qword ptr [rbp + 0xb8], 0xfffffffffffffffe
0x1415ce754  mov      rsi, rcx
0x1415ce757  mov      rax, qword ptr [rdx + 0x10]
0x1415ce75b  mov      qword ptr [rbp - 0x58], rax
0x1415ce75f  lea      rbx, [rbp - 0x40]
0x1415ce763  mov      rcx, rbx
0x1415ce766  call     0x140b880d0
0x1415ce76b  lea      r14, [rbp + 0x31]
0x1415ce76f  mov      byte ptr [rbp + 0xb6], 2
0x1415ce776  mov      dword ptr [rbp + 0x74], 0
0x1415ce77d  movabs   r13, 0x8000000000000000
0x1415ce787  lea      r15, [rbp + 0x78]
0x1415ce78b  lea      r12, [rbp + 0x30]
0x1415ce78f  lea      rdi, [rip + 0x16ad182]
0x1415ce796  mov      dword ptr [rbp + 0x70], 0
0x1415ce79d  mov      dword ptr [rbp + 0x6c], 0
0x1415ce7a4  mov      dword ptr [rbp + 0x68], 0
0x1415ce7ab  mov      qword ptr [rbp + 0x98], r13
0x1415ce7b2  nop      word ptr cs:[rax + rax]
0x1415ce7c0  mov      qword ptr [rbp + 0xa8], r13
0x1415ce7c7  mov      byte ptr [rbp + 0xb7], 1
0x1415ce7ce  mov      rcx, r15
0x1415ce7d1  mov      rdx, rbx
0x1415ce7d4  call     0x14143c450
0x1415ce7d9  mov      rax, qword ptr [rbp + 0x78]
0x1415ce7dd  test     rax, rax
0x1415ce7e0  je       0x1415cebca
0x1415ce7e6  mov      rcx, qword ptr [rbp + 0x88]
0x1415ce7ed  lea      rdx, [rcx + rcx*2]
0x1415ce7f1  mov      r8, qword ptr [rax + rdx*8 + 0x168]
0x1415ce7f9  mov      r9, qword ptr [rax + rdx*8 + 0x170]
0x1415ce801  mov      qword ptr [rbp + 0x58], r9
0x1415ce805  mov      r13, qword ptr [rax + rdx*8 + 0x178]
0x1415ce80d  shl      rcx, 5
0x1415ce811  movups   xmm0, xmmword ptr [rax + rcx]
0x1415ce815  movups   xmm1, xmmword ptr [rax + rcx + 0x10]
0x1415ce81a  movaps   xmmword ptr [rbp + 0x30], xmm0
0x1415ce81e  movaps   xmmword ptr [rbp + 0x40], xmm1
0x1415ce822  mov      qword ptr [rbp + 0x60], r8
0x1415ce826  mov      rax, r8
0x1415ce829  neg      rax
0x1415ce82c  jo       0x1415cebca
0x1415ce832  movzx    eax, byte ptr [rbp + 8]
0x1415ce836  cmp      rax, 6
0x1415ce83a  ja       0x1415ce9b1
0x1415ce840  movsxd   rax, dword ptr [rdi + rax*4]
0x1415ce844  add      rax, rdi
0x1415ce847  jmp      rax
0x1415ce849  movaps   xmm0, xmmword ptr [rbp + 0x30]
0x1415ce84d  movaps   xmm1, xmmword ptr [rbp + 0x40]
0x1415ce851  lea      rax, [rbp + 8]
0x1415ce855  movups   xmmword ptr [rax + 0x10], xmm1
0x1415ce859  movups   xmmword ptr [rax], xmm0
0x1415ce85c  mov      rcx, r15
0x1415ce85f  mov      rdx, qword ptr [rbp + 0x58]
0x1415ce863  mov      r8, r13
0x1415ce866  call     0x140af02d0
0x1415ce86b  mov      rdx, qword ptr [rbp + 0x60]
0x1415ce86f  test     rdx, rdx
0x1415ce872  je       0x1415ce883
0x1415ce874  mov      r8d, 1
0x1415ce87a  mov      rcx, qword ptr [rbp + 0x58]
0x1415ce87e  call     0x140613c20
0x1415ce883  cmp      byte ptr [rbp + 0x78], 0
0x1415ce887  mov      r13, qword ptr [rbp + 0x98]
0x1415ce88e  jne      0x1415cec16
0x1415ce894  movzx    eax, byte ptr [rbp + 0x79]
0x1415ce898  cmp      eax, 6
0x1415ce89b  jne      0x1415ce8e5
0x1415ce89d  movzx    eax, byte ptr [rbp + 8]
0x1415ce8a1  mov      byte ptr [rbp + 8], 6
0x1415ce8a5  cmp      al, 6
0x1415ce8a7  je       0x1415cec8b
0x1415ce8ad  mov      byte ptr [rbp + 0x30], al
0x1415ce8b0  movups   xmm0, xmmword ptr [rbx + 0x49]
0x1415ce8b4  movups   xmm1, xmmword ptr [rbx + 0x58]
0x1415ce8b8  movups   xmmword ptr [r14 + 0xf], xmm1
0x1415ce8bd  movups   xmmword ptr [r14], xmm0
0x1415ce8c1  mov      qword ptr [rbp + 0xa8], r13
0x1415ce8c8  mov      byte ptr [rbp + 0xb7], 1
0x1415ce8cf  mov      rcx, r12
0x1415ce8d2  call     0x1415a3940
0x1415ce8d7  test     rax, rax
0x1415ce8da  je       0x1415ce7c0
0x1415ce8e0  jmp      0x1415cebfc
0x1415ce8e5  mov      eax, eax
0x1415ce8e7  lea      rcx, [rip + 0x16ad046]
0x1415ce8ee  movsxd   rax, dword ptr [rcx + rax*4]
0x1415ce8f2  add      rax, rcx
0x1415ce8f5  jmp      rax
0x1415ce8f7  cmp      byte ptr [rbp + 0xb6], 2
0x1415ce8fe  jne      0x1415cef18
0x1415ce904  movzx    eax, byte ptr [rbp + 8]
0x1415ce908  mov      byte ptr [rbp + 8], 6
0x1415ce90c  cmp      al, 6
0x1415ce90e  je       0x1415cedda
0x1415ce914  mov      byte ptr [rbp + 0x30], al
0x1415ce917  movups   xmm0, xmmword ptr [rbx + 0x49]
0x1415ce91b  movups   xmm1, xmmword ptr [rbx + 0x58]
0x1415ce91f  movups   xmmword ptr [r14 + 0xf], xmm1
0x1415ce924  movups   xmmword ptr [r14], xmm0
0x1415ce928  mov      qword ptr [rbp + 0xa8], r13
0x1415ce92f  mov      byte ptr [rbp + 0xb7], 1
0x1415ce936  mov      rcx, r15
0x1415ce939  mov      rdx, r12
0x1415ce93c  call     0x14159d4f0
0x1415ce941  test     byte ptr [rbp + 0x78], 1
0x1415ce945  jne      0x1415cec16
0x1415ce94b  movzx    eax, byte ptr [rbp + 0x79]
0x1415ce94f  mov      byte ptr [rbp + 0xb6], al
0x1415ce955  jmp      0x1415ce7c0
0x1415ce95a  mov      rdx, qword ptr [rbp + 0x10]
0x1415ce95e  test     rdx, rdx
0x1415ce961  je       0x1415ce849
0x1415ce967  mov      rcx, qword ptr [rbp + 0x18]
0x1415ce96b  mov      r8d, 1
0x1415ce971  call     0x140613c20
0x1415ce976  jmp      0x1415ce849
0x1415ce97b  mov      rcx, qword ptr [rbp + 0x18]
0x1415ce97f  mov      rdx, qword ptr [rbp + 0x20]
0x1415ce983  mov      qword ptr [rbp + 0x28], rcx
0x1415ce987  call     0x140888d70
0x1415ce98c  mov      rdx, qword ptr [rbp + 0x10]
0x1415ce990  test     rdx, rdx
0x1415ce993  je       0x1415ce849
0x1415ce999  shl      rdx, 5
0x1415ce99d  mov      r8d, 8
0x1415ce9a3  mov      rcx, qword ptr [rbp + 0x28]
0x1415ce9a7  call     0x140613c20
0x1415ce9ac  jmp      0x1415ce849
0x1415ce9b1  lea      rcx, [rbp + 0x10]
0x1415ce9b5  call     0x140898f40
0x1415ce9ba  jmp      0x1415ce849
0x1415ce9bf  cmp      dword ptr [rbp + 0x70], 0
0x1415ce9c3  jne      0x1415ceef4
0x1415ce9c9  movzx    eax, byte ptr [rbp + 8]
0x1415ce9cd  mov      byte ptr [rbp + 8], 6
0x1415ce9d1  cmp      al, 6
0x1415ce9d3  je       0x1415cedb6
0x1415ce9d9  mov      byte ptr [rbp + 0x30], al
0x1415ce9dc  movups   xmm0, xmmword ptr [rbx + 0x49]
0x1415ce9e0  movups   xmm1, xmmword ptr [rbx + 0x58]
0x1415ce9e4  movups   xmmword ptr [r14 + 0xf], xmm1
0x1415ce9e9  movups   xmmword ptr [r14], xmm0
0x1415ce9ed  mov      qword ptr [rbp + 0xa8], r13
0x1415ce9f4  mov      byte ptr [rbp + 0xb7], 1
0x1415ce9fb  mov      rcx, r15
0x1415ce9fe  mov      rdx, r12
0x1415cea01  call     0x14159a370
0x1415cea06  test     byte ptr [rbp + 0x78], 1
0x1415cea0a  jne      0x1415cec16
0x1415cea10  movss    xmm7, dword ptr [rbp + 0x7c]
0x1415cea15  mov      dword ptr [rbp + 0x70], 1
0x1415cea1c  jmp      0x1415ce7c0
0x1415cea21  cmp      dword ptr [rbp + 0x68], 0
0x1415cea25  jne      0x1415cef3c
0x1415cea2b  movzx    eax, byte ptr [rbp + 8]
0x1415cea2f  mov      byte ptr [rbp + 8], 6
0x1415cea33  cmp      al, 6
0x1415cea35  je       0x1415cee22
0x1415cea3b  mov      byte ptr [rbp + 0x30], al
0x1415cea3e  movups   xmm0, xmmword ptr [rbx + 0x49]
0x1415cea42  movups   xmm1, xmmword ptr [rbx + 0x58]
0x1415cea46  movups   xmmword ptr [r14 + 0xf], xmm1
0x1415cea4b  movups   xmmword ptr [r14], xmm0
0x1415cea4f  mov      qword ptr [rbp + 0xa8], r13
0x1415cea56  mov      byte ptr [rbp + 0xb7], 1
0x1415cea5d  mov      rcx, r15
0x1415cea60  mov      rdx, r12
0x1415cea63  call     0x14159a370
0x1415cea68  test     byte ptr [rbp + 0x78], 1
0x1415cea6c  jne      0x1415cec16
0x1415cea72  movss    xmm9, dword ptr [rbp + 0x7c]
0x1415cea78  mov      dword ptr [rbp + 0x68], 1
0x1415cea7f  jmp      0x1415ce7c0
0x1415cea84  cmp      dword ptr [rbp + 0x6c], 0
0x1415cea88  jne      0x1415cef84
0x1415cea8e  movzx    eax, byte ptr [rbp + 8]
0x1415cea92  mov      byte ptr [rbp + 8], 6
0x1415cea96  cmp      al, 6
0x1415cea98  je       0x1415cedfe
0x1415cea9e  mov      byte ptr [rbp + 0x30], al
0x1415ceaa1  movups   xmm0, xmmword ptr [rbx + 0x49]
0x1415ceaa5  movups   xmm1, xmmword ptr [rbx + 0x58]
0x1415ceaa9  movups   xmmword ptr [r14 + 0xf], xmm1
0x1415ceaae  movups   xmmword ptr [r14], xmm0
0x1415ceab2  mov      qword ptr [rbp + 0xa8], r13
0x1415ceab9  mov      byte ptr [rbp + 0xb7], 1
0x1415ceac0  mov      rcx, r15
0x1415ceac3  mov      rdx, r12
0x1415ceac6  call     0x14159a370
0x1415ceacb  test     byte ptr [rbp + 0x78], 1
0x1415ceacf  jne      0x1415cec16
0x1415cead5  movss    xmm8, dword ptr [rbp + 0x7c]
0x1415ceadb  mov      dword ptr [rbp + 0x6c], 1
0x1415ceae2  jmp      0x1415ce7c0
0x1415ceae7  cmp      dword ptr [rbp + 0x74], 0
0x1415ceaeb  jne      0x1415cef60
0x1415ceaf1  movzx    eax, byte ptr [rbp + 8]
0x1415ceaf5  mov      byte ptr [rbp + 8], 6
0x1415ceaf9  cmp      al, 6
0x1415ceafb  je       0x1415ced92
0x1415ceb01  mov      byte ptr [rbp + 0x30], al
0x1415ceb04  movups   xmm0, xmmword ptr [rbx + 0x49]
0x1415ceb08  movups   xmm1, xmmword ptr [rbx + 0x58]
0x1415ceb0c  movups   xmmword ptr [r14 + 0xf], xmm1
0x1415ceb11  movups   xmmword ptr [r14], xmm0
0x1415ceb15  mov      qword ptr [rbp + 0xa8], r13
0x1415ceb1c  mov      byte ptr [rbp + 0xb7], 1
0x1415ceb23  mov      rcx, r15
0x1415ceb26  mov      rdx, r12
0x1415ceb29  call     0x14159a370
0x1415ceb2e  test     byte ptr [rbp + 0x78], 1
0x1415ceb32  jne      0x1415cec16
0x1415ceb38  movss    xmm6, dword ptr [rbp + 0x7c]
0x1415ceb3d  mov      dword ptr [rbp + 0x74], 1
0x1415ceb44  jmp      0x1415ce7c0
0x1415ceb49  mov      rax, r13
0x1415ceb4c  neg      rax
0x1415ceb4f  jno      0x1415ceed0
0x1415ceb55  movzx    eax, byte ptr [rbp + 8]
0x1415ceb59  mov      byte ptr [rbp + 8], 6
0x1415ceb5d  cmp      al, 6
0x1415ceb5f  je       0x1415cee46
0x1415ceb65  mov      byte ptr [rbp + 0x30], al
0x1415ceb68  movups   xmm0, xmmword ptr [rbx + 0x49]
0x1415ceb6c  movups   xmm1, xmmword ptr [rbx + 0x58]
0x1415ceb70  movups   xmmword ptr [r14 + 0xf], xmm1
0x1415ceb75  movups   xmmword ptr [r14], xmm0
0x1415ceb79  movabs   rax, 0x8000000000000000
0x1415ceb83  mov      qword ptr [rbp + 0xa8], rax
0x1415ceb8a  mov      byte ptr [rbp + 0xb7], 1
0x1415ceb91  mov      rcx, r15
0x1415ceb94  mov      rdx, r12
0x1415ceb97  call     0x14159de50
0x1415ceb9c  mov      r13, qword ptr [rbp + 0x78]
0x1415ceba0  mov      rax, qword ptr [rbp + 0x80]
0x1415ceba7  mov      qword ptr [rbp + 0x90], rax
0x1415cebae  mov      rax, r13
0x1415cebb1  neg      rax
0x1415cebb4  jo       0x1415cee71
0x1415cebba  mov      rax, qword ptr [rbp + 0x88]
0x1415cebc1  mov      qword ptr [rbp - 0x50], rax
0x1415cebc5  jmp      0x1415ce7ab
0x1415cebca  movzx    edx, byte ptr [rbp + 0xb6]
0x1415cebd1  cmp      dl, 2
0x1415cebd4  jne      0x1415cec4e
0x1415cebd6  mov      r13, qword ptr [rbp + 0x98]
0x1415cebdd  mov      qword ptr [rbp + 0xa8], r13
0x1415cebe4  mov      byte ptr [rbp + 0xb7], 1
0x1415cebeb  lea      rcx, [rip + 0x16ab936]
0x1415cebf2  mov      edx, 0xc
0x1415cebf7  call     0x1428f9bc0
0x1415cebfc  lea      rcx, [r13*2]
0x1415cec04  test     rcx, rcx
0x1415cec07  mov      rcx, qword ptr [rbp + 0x90]
0x1415cec0e  je       0x1415cee78
0x1415cec14  jmp      0x1415cec35
0x1415cec16  mov      rax, qword ptr [rbp + 0x80]
0x1415cec1d  lea      rcx, [r13*2]
0x1415cec25  test     rcx, rcx
0x1415cec28  mov      rcx, qword ptr [rbp + 0x90]
0x1415cec2f  je       0x1415cee78
0x1415cec35  mov      r8d, 1
0x1415cec3b  mov      rdx, r13
0x1415cec3e  mov      rdi, rax
0x1415cec41  call     0x140613c20
0x1415cec46  mov      rax, rdi
0x1415cec49  jmp      0x1415cee78
0x1415cec4e  mov      rcx, qword ptr [rbp + 0x98]
0x1415cec55  mov      rax, rcx
0x1415cec58  neg      rax
0x1415cec5b  jno      0x1415cecaf
0x1415cec5d  movabs   rax, 0x8000000000000000
0x1415cec67  mov      qword ptr [rbp + 0xa8], rax
0x1415cec6e  mov      byte ptr [rbp + 0xb7], 1
0x1415cec75  lea      rcx, [rip + 0x16ab8b8]
0x1415cec7c  mov      edx, 0x13
0x1415cec81  call     0x1428f9bc0
0x1415cec86  jmp      0x1415cee78
0x1415cec8b  mov      qword ptr [rbp + 0xa8], r13
0x1415cec92  mov      byte ptr [rbp + 0xb7], 1
0x1415cec99  lea      rcx, [rip + 0x16ab5c8]
0x1415ceca0  mov      edx, 0x10
0x1415ceca5  call     0x1428fa000
0x1415cecaa  jmp      0x1415cebfc
0x1415cecaf  test     byte ptr [rbp + 0x68], 1
0x1415cecb3  je       0x1415ced26
0x1415cecb5  test     byte ptr [rbp + 0x6c], 1
0x1415cecb9  je       0x1415ced39
0x1415cecbb  test     byte ptr [rbp + 0x70], 1
0x1415cecbf  je       0x1415ced4c
0x1415cecc5  test     byte ptr [rbp + 0x74], 1
0x1415cecc9  je       0x1415ced5f
0x1415ceccf  cmp      qword ptr [rbp], 0
0x1415cecd4  mov      rax, qword ptr [rbp + 0x90]
0x1415cecdb  jne      0x1415cefa8
0x1415cece1  and      dl, 1
0x1415cece4  mov      qword ptr [rsi], rcx
0x1415cece7  mov      qword ptr [rsi + 8], rax
0x1415ceceb  mov      rax, qword ptr [rbp - 0x50]
0x1415cecef  mov      qword ptr [rsi + 0x10], rax
0x1415cecf3  movss    dword ptr [rsi + 0x18], xmm9
0x1415cecf9  movss    dword ptr [rsi + 0x1c], xmm8
0x1415cecff  movss    dword ptr [rsi + 0x20], xmm7
0x1415ced04  movss    dword ptr [rsi + 0x24], xmm6
0x1415ced09  mov      byte ptr [rsi + 0x28], dl
0x1415ced0c  mov      eax, dword ptr [rbp - 0x47]
0x1415ced0f  mov      ecx, dword ptr [rbp - 0x44]
0x1415ced12  mov      dword ptr [rsi + 0x29], eax
0x1415ced15  mov      dword ptr [rsi + 0x2c], ecx
0x1415ced18  lea      rcx, [rbp - 0x40]
0x1415ced1c  call     0x140887df0
0x1415ced21  jmp      0x1415cee92
0x1415ced26  lea      rcx, [rip + 0x16ab81a]
0x1415ced2d  mov      edx, 0x10
0x1415ced32  call     0x1428f9bc0
0x1415ced37  jmp      0x1415ced70
0x1415ced39  lea      rcx, [rip + 0x16ab817]
0x1415ced40  mov      edx, 8
0x1415ced45  call     0x1428f9bc0
0x1415ced4a  jmp      0x1415ced70
0x1415ced4c  lea      rcx, [rip + 0x16ab80c]
0x1415ced53  mov      edx, 8
0x1415ced58  call     0x1428f9bc0
0x1415ced5d  jmp      0x1415ced70
0x1415ced5f  lea      rcx, [rip + 0x16ab801]
0x1415ced66  mov      edx, 0xa
0x1415ced6b  call     0x1428f9bc0
0x1415ced70  mov      rdx, qword ptr [rbp + 0x98]
0x1415ced77  test     rdx, rdx
0x1415ced7a  mov      rcx, qword ptr [rbp + 0x90]
0x1415ced81  je       0x1415cee78
0x1415ced87  mov      r8d, 1
0x1415ced8d  jmp      0x1415cec3e
0x1415ced92  mov      qword ptr [rbp + 0xa8], r13
0x1415ced99  mov      byte ptr [rbp + 0xb7], 1
0x1415ceda0  lea      rcx, [rip + 0x16ab4c1]
0x1415ceda7  mov      edx, 0x10
0x1415cedac  call     0x1428fa000
0x1415cedb1  jmp      0x1415cebfc
0x1415cedb6  mov      qword ptr [rbp + 0xa8], r13
0x1415cedbd  mov      byte ptr [rbp + 0xb7], 1
0x1415cedc4  lea      rcx, [rip + 0x16ab49d]
0x1415cedcb  mov      edx, 0x10
0x1415cedd0  call     0x1428fa000
0x1415cedd5  jmp      0x1415cebfc
0x1415cedda  mov      qword ptr [rbp + 0xa8], r13
0x1415cede1  mov      byte ptr [rbp + 0xb7], 1
0x1415cede8  lea      rcx, [rip + 0x16ab479]
0x1415cedef  mov      edx, 0x10
0x1415cedf4  call     0x1428fa000
0x1415cedf9  jmp      0x1415cebfc
0x1415cedfe  mov      qword ptr [rbp + 0xa8], r13
0x1415cee05  mov      byte ptr [rbp + 0xb7], 1
0x1415cee0c  lea      rcx, [rip + 0x16ab455]
0x1415cee13  mov      edx, 0x10
0x1415cee18  call     0x1428fa000
0x1415cee1d  jmp      0x1415cebfc
0x1415cee22  mov      qword ptr [rbp + 0xa8], r13
0x1415cee29  mov      byte ptr [rbp + 0xb7], 1
0x1415cee30  lea      rcx, [rip + 0x16ab431]
0x1415cee37  mov      edx, 0x10
0x1415cee3c  call     0x1428fa000
0x1415cee41  jmp      0x1415cebfc
0x1415cee46  movabs   rax, 0x8000000000000000
0x1415cee50  mov      qword ptr [rbp + 0xa8], rax
0x1415cee57  mov      byte ptr [rbp + 0xb7], 1
0x1415cee5e  lea      rcx, [rip + 0x16ab403]
0x1415cee65  mov      edx, 0x10
0x1415cee6a  call     0x1428fa000
0x1415cee6f  jmp      0x1415cee78
0x1415cee71  mov      rax, qword ptr [rbp + 0x90]
0x1415cee78  mov      qword ptr [rsi + 8], rax
0x1415cee7c  movabs   rax, 0x8000000000000000
0x1415cee86  mov      qword ptr [rsi], rax
0x1415cee89  lea      rcx, [rbp - 0x40]
0x1415cee8d  call     0x140887df0
0x1415cee92  lea      rcx, [rbp + 8]
0x1415cee96  call     0x14088e430
0x1415cee9b  mov      rax, rsi
0x1415cee9e  movaps   xmm6, xmmword ptr [rbp + 0xc0]
0x1415ceea5  movaps   xmm7, xmmword ptr [rbp + 0xd0]
0x1415ceeac  movaps   xmm8, xmmword ptr [rbp + 0xe0]
0x1415ceeb4  movaps   xmm9, xmmword ptr [rbp + 0xf0]
0x1415ceebc  add      rsp, 0x188
0x1415ceec3  pop      rbx
0x1415ceec4  pop      rdi
0x1415ceec5  pop      rsi
0x1415ceec6  pop      r12
0x1415ceec8  pop      r13
0x1415ceeca  pop      r14
0x1415ceecc  pop      r15
0x1415ceece  pop      rbp
0x1415ceecf  ret      
0x1415ceed0  mov      qword ptr [rbp + 0xa8], r13
0x1415ceed7  mov      byte ptr [rbp + 0xb7], 1
0x1415ceede  lea      rcx, [rip + 0x16ab64f]
0x1415ceee5  mov      edx, 0x13
0x1415ceeea  call     0x1428f9db0
0x1415ceeef  jmp      0x1415cebfc
0x1415ceef4  mov      qword ptr [rbp + 0xa8], r13
0x1415ceefb  mov      byte ptr [rbp + 0xb7], 1
0x1415cef02  lea      rcx, [rip + 0x16ab656]
0x1415cef09  mov      edx, 8
0x1415cef0e  call     0x1428f9db0
0x1415cef13  jmp      0x1415cebfc
0x1415cef18  mov      qword ptr [rbp + 0xa8], r13
0x1415cef1f  mov      byte ptr [rbp + 0xb7], 1
0x1415cef26  lea      rcx, [rip + 0x16ab5fb]
0x1415cef2d  mov      edx, 0xc
0x1415cef32  call     0x1428f9db0
0x1415cef37  jmp      0x1415cebfc
0x1415cef3c  mov      qword ptr [rbp + 0xa8], r13
0x1415cef43  mov      byte ptr [rbp + 0xb7], 1
0x1415cef4a  lea      rcx, [rip + 0x16ab5f6]
0x1415cef51  mov      edx, 0x10
0x1415cef56  call     0x1428f9db0
0x1415cef5b  jmp      0x1415cebfc
0x1415cef60  mov      qword ptr [rbp + 0xa8], r13
0x1415cef67  mov      byte ptr [rbp + 0xb7], 1
0x1415cef6e  lea      rcx, [rip + 0x16ab5f2]
0x1415cef75  mov      edx, 0xa
0x1415cef7a  call     0x1428f9db0
0x1415cef7f  jmp      0x1415cebfc
0x1415cef84  mov      qword ptr [rbp + 0xa8], r13
0x1415cef8b  mov      byte ptr [rbp + 0xb7], 1
0x1415cef92  lea      rcx, [rip + 0x16ab5be]
0x1415cef99  mov      edx, 8
0x1415cef9e  call     0x1428f9db0
0x1415cefa3  jmp      0x1415cebfc
0x1415cefa8  lea      rdx, [rip + 0x16ab0e9]
0x1415cefaf  lea      r8, [rip + 0x16ab0aa]
0x1415cefb6  mov      rcx, qword ptr [rbp - 0x58]
0x1415cefba  call     0x1428f9d20
0x1415cefbf  mov      qword ptr [rsi + 8], rax
0x1415cefc3  movabs   rax, 0x8000000000000000
0x1415cefcd  mov      qword ptr [rsi], rax
0x1415cefd0  cmp      qword ptr [rbp + 0x98], 0
0x1415cefd8  je       0x1415ced18
0x1415cefde  mov      r8d, 1
0x1415cefe4  mov      rcx, qword ptr [rbp + 0x90]
0x1415cefeb  mov      rdx, qword ptr [rbp + 0x98]
0x1415ceff2  call     0x140613c20
0x1415ceff7  jmp      0x1415ced18
0x1415ceffc  nop      dword ptr [rax]
0x1415cf000  mov      qword ptr [rsp + 0x10], rdx
0x1415cf005  push     rbp
0x1415cf006  push     r15
0x1415cf008  push     r14
0x1415cf00a  push     r13
0x1415cf00c  push     r12
0x1415cf00e  push     rsi
0x1415cf00f  push     rdi
0x1415cf010  push     rbx
0x1415cf011  sub      rsp, 0x68
0x1415cf015  lea      rbp, [rdx + 0x80]
0x1415cf01c  movaps   xmmword ptr [rsp + 0x20], xmm9
0x1415cf022  movaps   xmmword ptr [rsp + 0x30], xmm8
0x1415cf028  movaps   xmmword ptr [rsp + 0x40], xmm7
0x1415cf02d  movaps   xmmword ptr [rsp + 0x50], xmm6
0x1415cf032  mov      rdx, qword ptr [rbp + 0x10]
0x1415cf036  test     rdx, rdx
0x1415cf039  je       0x1415cf04e
0x1415cf03b  shl      rdx, 5
0x1415cf03f  mov      r8d, 8
0x1415cf045  mov      rcx, qword ptr [rbp + 0x28]
0x1415cf049  call     0x140613c20
0x1415cf04e  movaps   xmm6, xmmword ptr [rsp + 0x50]
0x1415cf053  movaps   xmm7, xmmword ptr [rsp + 0x40]
0x1415cf058  movaps   xmm8, xmmword ptr [rsp + 0x30]
0x1415cf05e  movaps   xmm9, xmmword ptr [rsp + 0x20]
0x1415cf064  add      rsp, 0x68
0x1415cf068  pop      rbx
0x1415cf069  pop      rdi
0x1415cf06a  pop      rsi
0x1415cf06b  pop      r12
0x1415cf06d  pop      r13
0x1415cf06f  pop      r14
0x1415cf071  pop      r15
0x1415cf073  pop      rbp
0x1415cf074  ret      
0x1415cf075  nop      word ptr cs:[rax + rax]
0x1415cf080  mov      qword ptr [rsp + 0x10], rdx
0x1415cf085  push     rbp
0x1415cf086  push     r15
0x1415cf088  push     r14
0x1415cf08a  push     r13
0x1415cf08c  push     r12
0x1415cf08e  push     rsi
0x1415cf08f  push     rdi
0x1415cf090  push     rbx
0x1415cf091  sub      rsp, 0x68
0x1415cf095  lea      rbp, [rdx + 0x80]
0x1415cf09c  movaps   xmmword ptr [rsp + 0x20], xmm9
0x1415cf0a2  movaps   xmmword ptr [rsp + 0x30], xmm8
0x1415cf0a8  movaps   xmmword ptr [rsp + 0x40], xmm7
0x1415cf0ad  movaps   xmmword ptr [rsp + 0x50], xmm6
0x1415cf0b2  movaps   xmm0, xmmword ptr [rbp + 0x30]
0x1415cf0b6  movaps   xmm1, xmmword ptr [rbp + 0x40]
0x1415cf0ba  lea      rax, [rbp + 8]
0x1415cf0be  movups   xmmword ptr [rax + 0x10], xmm1
0x1415cf0c2  movups   xmmword ptr [rax], xmm0
0x1415cf0c5  mov      byte ptr [rbp + 0xa7], 1
0x1415cf0cc  movaps   xmm6, xmmword ptr [rsp + 0x50]
0x1415cf0d1  movaps   xmm7, xmmword ptr [rsp + 0x40]
0x1415cf0d6  movaps   xmm8, xmmword ptr [rsp + 0x30]
0x1415cf0dc  movaps   xmm9, xmmword ptr [rsp + 0x20]
0x1415cf0e2  add      rsp, 0x68
0x1415cf0e6  pop      rbx
0x1415cf0e7  pop      rdi
0x1415cf0e8  pop      rsi
0x1415cf0e9  pop      r12
0x1415cf0eb  pop      r13
0x1415cf0ed  pop      r14
0x1415cf0ef  pop      r15
0x1415cf0f1  pop      rbp
0x1415cf0f2  ret      
0x1415cf0f3  nop      word ptr cs:[rax + rax]
0x1415cf100  mov      qword ptr [rsp + 0x10], rdx
0x1415cf105  push     rbp
0x1415cf106  push     r15
0x1415cf108  push     r14
0x1415cf10a  push     r13
0x1415cf10c  push     r12
0x1415cf10e  push     rsi
0x1415cf10f  push     rdi
0x1415cf110  push     rbx
0x1415cf111  sub      rsp, 0x68
0x1415cf115  lea      rbp, [rdx + 0x80]
0x1415cf11c  movaps   xmmword ptr [rsp + 0x20], xmm9
0x1415cf122  movaps   xmmword ptr [rsp + 0x30], xmm8
0x1415cf128  movaps   xmmword ptr [rsp + 0x40], xmm7
0x1415cf12d  movaps   xmmword ptr [rsp + 0x50], xmm6
0x1415cf132  cmp      qword ptr [rbp + 0x60], 0
0x1415cf137  je       0x1415cf14c
0x1415cf139  mov      r8d, 1
0x1415cf13f  mov      rcx, qword ptr [rbp + 0x58]
0x1415cf143  mov      rdx, qword ptr [rbp + 0x60]
0x1415cf147  call     0x140613c20
0x1415cf14c  mov      byte ptr [rbp + 0xa7], 0
0x1415cf153  movaps   xmm6, xmmword ptr [rsp + 0x50]
0x1415cf158  movaps   xmm7, xmmword ptr [rsp + 0x40]
0x1415cf15d  movaps   xmm8, xmmword ptr [rsp + 0x30]
0x1415cf163  movaps   xmm9, xmmword ptr [rsp + 0x20]
0x1415cf169  add      rsp, 0x68
0x1415cf16d  pop      rbx
0x1415cf16e  pop      rdi
0x1415cf16f  pop      rsi
0x1415cf170  pop      r12
0x1415cf172  pop      r13
0x1415cf174  pop      r14
0x1415cf176  pop      r15
0x1415cf178  pop      rbp
0x1415cf179  ret      
0x1415cf17a  nop      word ptr [rax + rax]
0x1415cf180  mov      qword ptr [rsp + 0x10], rdx
0x1415cf185  push     rbp
0x1415cf186  push     r15
0x1415cf188  push     r14
0x1415cf18a  push     r13
0x1415cf18c  push     r12
0x1415cf18e  push     rsi
0x1415cf18f  push     rdi
0x1415cf190  push     rbx
0x1415cf191  sub      rsp, 0x68
0x1415cf195  lea      rbp, [rdx + 0x80]
0x1415cf19c  movaps   xmmword ptr [rsp + 0x20], xmm9
0x1415cf1a2  movaps   xmmword ptr [rsp + 0x30], xmm8
0x1415cf1a8  movaps   xmmword ptr [rsp + 0x40], xmm7
0x1415cf1ad  movaps   xmmword ptr [rsp + 0x50], xmm6
0x1415cf1b2  cmp      byte ptr [rbp + 0xa7], 0
0x1415cf1b9  je       0x1415cf211
0x1415cf1bb  cmp      qword ptr [rbp + 0x60], 0
0x1415cf1c0  je       0x1415cf1d5
0x1415cf1c2  mov      r8d, 1
0x1415cf1c8  mov      rcx, qword ptr [rbp + 0x58]
0x1415cf1cc  mov      rdx, qword ptr [rbp + 0x60]
0x1415cf1d0  call     0x140613c20
0x1415cf1d5  mov      rax, qword ptr [rbp + 0x98]
0x1415cf1dc  mov      qword ptr [rbp + 0xa8], rax
0x1415cf1e3  mov      byte ptr [rbp + 0xb7], 1
0x1415cf1ea  movaps   xmm6, xmmword ptr [rsp + 0x50]
0x1415cf1ef  movaps   xmm7, xmmword ptr [rsp + 0x40]
0x1415cf1f4  movaps   xmm8, xmmword ptr [rsp + 0x30]
0x1415cf1fa  movaps   xmm9, xmmword ptr [rsp + 0x20]
0x1415cf200  add      rsp, 0x68
0x1415cf204  pop      rbx
0x1415cf205  pop      rdi
0x1415cf206  pop      rsi
0x1415cf207  pop      r12
0x1415cf209  pop      r13
0x1415cf20b  pop      r14
0x1415cf20d  pop      r15
0x1415cf20f  pop      rbp
0x1415cf210  ret      
0x1415cf211  mov      rax, qword ptr [rbp + 0x98]
0x1415cf218  mov      qword ptr [rbp + 0xa8], rax
0x1415cf21f  mov      byte ptr [rbp + 0xb7], 1
0x1415cf226  movaps   xmm6, xmmword ptr [rsp + 0x50]
0x1415cf22b  movaps   xmm7, xmmword ptr [rsp + 0x40]
0x1415cf230  movaps   xmm8, xmmword ptr [rsp + 0x30]
0x1415cf236  movaps   xmm9, xmmword ptr [rsp + 0x20]
0x1415cf23c  add      rsp, 0x68
0x1415cf240  pop      rbx
0x1415cf241  pop      rdi
0x1415cf242  pop      rsi
0x1415cf243  pop      r12
0x1415cf245  pop      r13
0x1415cf247  pop      r14
0x1415cf249  pop      r15
0x1415cf24b  pop      rbp
0x1415cf24c  ret      
0x1415cf24d  nop      dword ptr [rax]
0x1415cf250  mov      qword ptr [rsp + 0x10], rdx
0x1415cf255  push     rbp
0x1415cf256  push     r15
0x1415cf258  push     r14
0x1415cf25a  push     r13
0x1415cf25c  push     r12
0x1415cf25e  push     rsi
0x1415cf25f  push     rdi
0x1415cf260  push     rbx
0x1415cf261  sub      rsp, 0x68
0x1415cf265  lea      rbp, [rdx + 0x80]
0x1415cf26c  movaps   xmmword ptr [rsp + 0x20], xmm9
0x1415cf272  movaps   xmmword ptr [rsp + 0x30], xmm8
0x1415cf278  movaps   xmmword ptr [rsp + 0x40], xmm7
0x1415cf27d  movaps   xmmword ptr [rsp + 0x50], xmm6
0x1415cf282  cmp      qword ptr [rbp + 0x98], 0
0x1415cf28a  je       0x1415cf2a5
0x1415cf28c  mov      r8d, 1
0x1415cf292  mov      rcx, qword ptr [rbp + 0x90]
0x1415cf299  mov      rdx, qword ptr [rbp + 0x98]
0x1415cf2a0  call     0x140613c20
0x1415cf2a5  mov      rax, qword ptr [rbp + 0x98]
0x1415cf2ac  mov      qword ptr [rbp + 0xa8], rax
0x1415cf2b3  mov      byte ptr [rbp + 0xb7], 0
0x1415cf2ba  movaps   xmm6, xmmword ptr [rsp + 0x50]
0x1415cf2bf  movaps   xmm7, xmmword ptr [rsp + 0x40]
0x1415cf2c4  movaps   xmm8, xmmword ptr [rsp + 0x30]
0x1415cf2ca  movaps   xmm9, xmmword ptr [rsp + 0x20]
0x1415cf2d0  add      rsp, 0x68
0x1415cf2d4  pop      rbx
0x1415cf2d5  pop      rdi
0x1415cf2d6  pop      rsi
0x1415cf2d7  pop      r12
0x1415cf2d9  pop      r13
0x1415cf2db  pop      r14
0x1415cf2dd  pop      r15
0x1415cf2df  pop      rbp
0x1415cf2e0  ret      
0x1415cf2e1  nop      word ptr cs:[rax + rax]
0x1415cf2f0  mov      qword ptr [rsp + 0x10], rdx
0x1415cf2f5  push     rbp
0x1415cf2f6  push     r15
0x1415cf2f8  push     r14
0x1415cf2fa  push     r13
0x1415cf2fc  push     r12
0x1415cf2fe  push     rsi
0x1415cf2ff  push     rdi
0x1415cf300  push     rbx
0x1415cf301  sub      rsp, 0x68
0x1415cf305  lea      rbp, [rdx + 0x80]
0x1415cf30c  movaps   xmmword ptr [rsp + 0x20], xmm9
0x1415cf312  movaps   xmmword ptr [rsp + 0x30], xmm8
0x1415cf318  movaps   xmmword ptr [rsp + 0x40], xmm7
0x1415cf31d  movaps   xmmword ptr [rsp + 0x50], xmm6
0x1415cf322  mov      rdx, qword ptr [rbp + 0xa8]
0x1415cf329  mov      rax, rdx
0x1415cf32c  neg      rax
0x1415cf32f  jno      0x1415cf358
0x1415cf331  movaps   xmm6, xmmword ptr [rsp + 0x50]
0x1415cf336  movaps   xmm7, xmmword ptr [rsp + 0x40]
0x1415cf33b  movaps   xmm8, xmmword ptr [rsp + 0x30]
0x1415cf341  movaps   xmm9, xmmword ptr [rsp + 0x20]
0x1415cf347  add      rsp, 0x68
0x1415cf34b  pop      rbx
0x1415cf34c  pop      rdi
0x1415cf34d  pop      rsi
0x1415cf34e  pop      r12
0x1415cf350  pop      r13
0x1415cf352  pop      r14
0x1415cf354  pop      r15
0x1415cf356  pop      rbp
0x1415cf357  ret      
0x1415cf358  test     byte ptr [rbp + 0xb7], 1
0x1415cf35f  je       0x1415cf39f
0x1415cf361  test     rdx, rdx
0x1415cf364  je       0x1415cf378
0x1415cf366  mov      r8d, 1
0x1415cf36c  mov      rcx, qword ptr [rbp + 0x90]
0x1415cf373  call     0x140613c20
0x1415cf378  movaps   xmm6, xmmword ptr [rsp + 0x50]
0x1415cf37d  movaps   xmm7, xmmword ptr [rsp + 0x40]
0x1415cf382  movaps   xmm8, xmmword ptr [rsp + 0x30]
0x1415cf388  movaps   xmm9, xmmword ptr [rsp + 0x20]
0x1415cf38e  add      rsp, 0x68
0x1415cf392  pop      rbx
0x1415cf393  pop      rdi
0x1415cf394  pop      rsi
0x1415cf395  pop      r12
0x1415cf397  pop      r13
0x1415cf399  pop      r14
0x1415cf39b  pop      r15
0x1415cf39d  pop      rbp
0x1415cf39e  ret      
0x1415cf39f  movaps   xmm6, xmmword ptr [rsp + 0x50]
0x1415cf3a4  movaps   xmm7, xmmword ptr [rsp + 0x40]
0x1415cf3a9  movaps   xmm8, xmmword ptr [rsp + 0x30]
0x1415cf3af  movaps   xmm9, xmmword ptr [rsp + 0x20]
0x1415cf3b5  add      rsp, 0x68
0x1415cf3b9  pop      rbx
0x1415cf3ba  pop      rdi
0x1415cf3bb  pop      rsi
0x1415cf3bc  pop      r12
0x1415cf3be  pop      r13
0x1415cf3c0  pop      r14
0x1415cf3c2  pop      r15
0x1415cf3c4  pop      rbp
0x1415cf3c5  ret      
0x1415cf3c6  nop      word ptr cs:[rax + rax]
0x1415cf3d0  mov      qword ptr [rsp + 0x10], rdx
0x1415cf3d5  push     rbp
0x1415cf3d6  push     r15
0x1415cf3d8  push     r14
0x1415cf3da  push     r13
0x1415cf3dc  push     r12
0x1415cf3de  push     rsi
0x1415cf3df  push     rdi
0x1415cf3e0  push     rbx
0x1415cf3e1  sub      rsp, 0x68
0x1415cf3e5  lea      rbp, [rdx + 0x80]
0x1415cf3ec  movaps   xmmword ptr [rsp + 0x20], xmm9
0x1415cf3f2  movaps   xmmword ptr [rsp + 0x30], xmm8
0x1415cf3f8  movaps   xmmword ptr [rsp + 0x40], xmm7
0x1415cf3fd  movaps   xmmword ptr [rsp + 0x50], xmm6
0x1415cf402  lea      rcx, [rbp - 0x40]
0x1415cf406  call     0x14088a500
0x1415cf40b  movaps   xmm6, xmmword ptr [rsp + 0x50]
0x1415cf410  movaps   xmm7, xmmword ptr [rsp + 0x40]
0x1415cf415  movaps   xmm8, xmmword ptr [rsp + 0x30]
0x1415cf41b  movaps   xmm9, xmmword ptr [rsp + 0x20]
0x1415cf421  add      rsp, 0x68
0x1415cf425  pop      rbx
0x1415cf426  pop      rdi
0x1415cf427  pop      rsi
0x1415cf428  pop      r12
0x1415cf42a  pop      r13
0x1415cf42c  pop      r14
0x1415cf42e  pop      r15
0x1415cf430  pop      rbp
0x1415cf431  ret      
0x1415cf432  nop      word ptr cs:[rax + rax]
0x1415cf440  mov      qword ptr [rsp + 0x10], rdx
0x1415cf445  push     rbp
0x1415cf446  push     r15
0x1415cf448  push     r14
0x1415cf44a  push     r13
0x1415cf44c  push     r12
0x1415cf44e  push     rsi
0x1415cf44f  push     rdi
0x1415cf450  push     rbx
0x1415cf451  sub      rsp, 0x68
0x1415cf455  lea      rbp, [rdx + 0x80]
0x1415cf45c  movaps   xmmword ptr [rsp + 0x20], xmm9
0x1415cf462  movaps   xmmword ptr [rsp + 0x30], xmm8
0x1415cf468  movaps   xmmword ptr [rsp + 0x40], xmm7
0x1415cf46d  movaps   xmmword ptr [rsp + 0x50], xmm6
0x1415cf472  lea      rcx, [rbp + 8]
0x1415cf476  call     0x14088e430
0x1415cf47b  movaps   xmm6, xmmword ptr [rsp + 0x50]
0x1415cf480  movaps   xmm7, xmmword ptr [rsp + 0x40]
0x1415cf485  movaps   xmm8, xmmword ptr [rsp + 0x30]
0x1415cf48b  movaps   xmm9, xmmword ptr [rsp + 0x20]
0x1415cf491  add      rsp, 0x68
0x1415cf495  pop      rbx
0x1415cf496  pop      rdi
0x1415cf497  pop      rsi
0x1415cf498  pop      r12
0x1415cf49a  pop      r13
0x1415cf49c  pop      r14
0x1415cf49e  pop      r15
0x1415cf4a0  pop      rbp
0x1415cf4a1  ret      
0x1415cf4a2  nop      word ptr cs:[rax + rax]
0x1415cf4b0  mov      qword ptr [rsp + 0x10], rdx
0x1415cf4b5  push     rbp
0x1415cf4b6  push     r15
0x1415cf4b8  push     r14
0x1415cf4ba  push     r13
0x1415cf4bc  push     r12
0x1415cf4be  push     rsi
0x1415cf4bf  push     rdi
0x1415cf4c0  push     rbx
0x1415cf4c1  sub      rsp, 0x68
0x1415cf4c5  lea      rbp, [rdx + 0x80]
0x1415cf4cc  movaps   xmmword ptr [rsp + 0x20], xmm9
0x1415cf4d2  movaps   xmmword ptr [rsp + 0x30], xmm8
0x1415cf4d8  movaps   xmmword ptr [rsp + 0x40], xmm7
0x1415cf4dd  movaps   xmmword ptr [rsp + 0x50], xmm6
0x1415cf4e2  cmp      qword ptr [rbp + 0x98], 0
0x1415cf4ea  je       0x1415cf505
0x1415cf4ec  mov      r8d, 1
0x1415cf4f2  mov      rcx, qword ptr [rbp + 0x90]
0x1415cf4f9  mov      rdx, qword ptr [rbp + 0x98]
0x1415cf500  call     0x140613c20
0x1415cf505  movaps   xmm6, xmmword ptr [rsp + 0x50]
0x1415cf50a  movaps   xmm7, xmmword ptr [rsp + 0x40]
0x1415cf50f  movaps   xmm8, xmmword ptr [rsp + 0x30]
0x1415cf515  movaps   xmm9, xmmword ptr [rsp + 0x20]
0x1415cf51b  add      rsp, 0x68
0x1415cf51f  pop      rbx
0x1415cf520  pop      rdi
0x1415cf521  pop      rsi
0x1415cf522  pop      r12
0x1415cf524  pop      r13
0x1415cf526  pop      r14
0x1415cf528  pop      r15
0x1415cf52a  pop      rbp
0x1415cf52b  ret      
0x1415cf52c  nop      dword ptr [rax]
0x1415cf530  mov      qword ptr [rsp + 0x10], rdx
0x1415cf535  push     rbp
0x1415cf536  push     r15
0x1415cf538  push     r14
0x1415cf53a  push     r13
0x1415cf53c  push     r12
0x1415cf53e  push     rsi
0x1415cf53f  push     rdi
0x1415cf540  push     rbx
0x1415cf541  sub      rsp, 0x68
0x1415cf545  lea      rbp, [rdx + 0x80]
0x1415cf54c  movaps   xmmword ptr [rsp + 0x20], xmm9
0x1415cf552  movaps   xmmword ptr [rsp + 0x30], xmm8
0x1415cf558  movaps   xmmword ptr [rsp + 0x40], xmm7
0x1415cf55d  movaps   xmmword ptr [rsp + 0x50], xmm6
0x1415cf562  lea      rcx, [rbp + 8]
0x1415cf566  call     0x14088e430
0x1415cf56b  movaps   xmm6, xmmword ptr [rsp + 0x50]
0x1415cf570  movaps   xmm7, xmmword ptr [rsp + 0x40]
0x1415cf575  movaps   xmm8, xmmword ptr [rsp + 0x30]
0x1415cf57b  movaps   xmm9, xmmword ptr [rsp + 0x20]
0x1415cf581  add      rsp, 0x68
0x1415cf585  pop      rbx
0x1415cf586  pop      rdi
0x1415cf587  pop      rsi
0x1415cf588  pop      r12
0x1415cf58a  pop      r13
0x1415cf58c  pop      r14
0x1415cf58e  pop      r15
0x1415cf590  pop      rbp
0x1415cf591  ret      

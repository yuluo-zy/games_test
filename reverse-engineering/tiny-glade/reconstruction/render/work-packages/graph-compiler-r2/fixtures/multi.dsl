access-info 1 0x20 false
access-info 2 0x40 true
access-info 3 0x80 true
access-info 4 0x800 false
batch per-pass
resource 0 7 image
resource 1 9 buffer import-unowned 4
pass upload 0:0:1 1:0:2
pass empty
pass draw 0:0:3
export 0 4

# Ghidra (Jython) 符号统计
fm = currentProgram.getFunctionManager()
st = currentProgram.getSymbolTable()
funcs = fm.getFunctionCount()
syms = st.getNumSymbols()
rust = 0
demangled = 0
sample = []
for s in st.getAllSymbols(True):
    n = s.getName()
    if "::" in n:
        rust += 1
        if len(sample) < 8:
            sample.append(n)
    if "<" in n and ">" in n:
        demangled += 1
print("=== GHIDRA SYMBOL REPORT ===")
print("functions: %d" % funcs)
print("symbols:   %d" % syms)
print("rust-style (::): %d" % rust)
print("generics/demangled: %d" % demangled)
print("--- samples ---")
for n in sample:
    print(n)

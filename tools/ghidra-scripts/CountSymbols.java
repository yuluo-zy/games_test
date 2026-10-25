import ghidra.app.script.GhidraScript;
import ghidra.program.model.symbol.Symbol;
import ghidra.program.model.symbol.SymbolTable;

public class CountSymbols extends GhidraScript {
    public void run() throws Exception {
        SymbolTable st = currentProgram.getSymbolTable();
        long funcs = currentProgram.getFunctionManager().getFunctionCount();
        int syms = st.getNumSymbols();
        int rust = 0, generic = 0, shown = 0;
        println("=== GHIDRA SYMBOL REPORT ===");
        println("functions: " + funcs);
        println("symbols:   " + syms);
        for (Symbol s : st.getAllSymbols(true)) {
            String n = s.getName();
            if (n.contains("::")) {
                rust++;
                if (shown < 8) { println("  sample: " + n); shown++; }
            }
            if (n.contains("<") && n.contains(">")) generic++;
        }
        println("rust-style names (::): " + rust);
        println("generic/demangled:     " + generic);
    }
}

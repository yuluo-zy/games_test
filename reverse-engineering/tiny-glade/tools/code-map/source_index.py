"""Lexical source navigation with explicit owners; never semantic acceptance."""
import ast
import bisect
import hashlib
from pathlib import Path
import re
import subprocess

def lexer(workspace):
    # Reuse the already tested project lexer without importing the verifier's
    # command-line side effects or maintaining a second Rust string/comment lexer.
    path=workspace/'tiny-glade/tools/verify_inventory.py'
    tree=ast.parse(path.read_text(encoding='utf-8'))
    selected=[n for n in tree.body if
              isinstance(n,ast.FunctionDef) and n.name=='rust_code' or
              isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id.startswith('RUST_') for t in n.targets)]
    scope={'re':re}
    exec(compile(ast.Module(body=selected,type_ignores=[]),str(path),'exec'),scope)
    return scope['rust_code']

def brace_pairs(code):
    stack=[];pairs={}
    for i,c in enumerate(code):
        if c=='{':stack.append(i)
        elif c=='}' and stack:pairs[stack.pop()]=i
    return pairs

def declaration_body(code,start,pairs):
    paren=bracket=angle=0
    for i in range(start,len(code)):
        c=code[i]
        if c=='(':paren+=1
        elif c==')':paren-=1
        elif c=='[':bracket+=1
        elif c==']':bracket-=1
        elif c=='<' and not paren and not bracket:angle+=1
        elif c=='>' and (i==0 or code[i-1]!='-') and not paren and not bracket:angle=max(0,angle-1)
        elif c==';' and not (paren or bracket or angle):return None
        elif c=='{' and not (paren or bracket or angle):return (i,pairs[i]) if i in pairs else None
    return None

def without_generics(text):
    depth=0;out=[]
    for c in text:
        if c=='<':depth+=1
        elif c=='>':depth=max(0,depth-1)
        elif not depth:out.append(c)
    return ''.join(out)

def functions(text,path,lex):
    code=lex(text,True);preserved=lex(text,False);pairs=brace_pairs(code)
    excluded=[]
    # Exclude only the attributed item, not the remainder of its source file.
    for hit in re.finditer(r'#\[\s*(?:cfg\(\s*test\s*\)|test)\s*\]',code):
        body=declaration_body(code,hit.end(),pairs)
        if body:excluded.append((hit.start(),body[1]+1))
    for hit in re.finditer(r'\bmacro_rules\s*!\s*\w+',code):
        body=declaration_body(code,hit.end(),pairs)
        if body:excluded.append((hit.start(),body[1]+1))
    impls=[]
    for hit in re.finditer(r'\bimpl\b',code):
        body=declaration_body(code,hit.end(),pairs)
        if not body:continue
        header=without_generics(code[hit.end():body[0]]).split('where',1)[0].strip()
        trait,owner=(header.rsplit(' for ',1) if ' for ' in header else ('',header))
        names=re.findall(r'[A-Za-z_]\w*',owner)
        impls.append((body[0],body[1],names[-1] if names else None,trait.strip()))
    newlines=[i for i,c in enumerate(text) if c=='\n']
    records=[]
    for hit in re.finditer(r'\bfn\s+(?:r#)?([A-Za-z_]\w*)\s*(?:<|\()',code):
        if any(a<=hit.start()<b for a,b in excluded):continue
        body=declaration_body(code,hit.start()+len('fn')+len(hit[1])+1,pairs)
        if not body:continue
        owners=[r for r in impls if r[0]<hit.start()<r[1]]
        owner=min(owners,key=lambda r:r[1]-r[0]) if owners else None
        masked=code[body[0]+1:body[1]].strip()
        real=preserved[body[0]+1:body[1]].strip()
        state='body_candidate'
        if not real:state='empty_body_review'
        elif re.fullmatch(r'(?:todo|unimplemented)\s*!\s*(?:\([\s\S]*\)|\{[\s\S]*\}|\[[\s\S]*\])\s*;?',masked):state='explicit_stub'
        elif re.search(r'\b(?:todo|unimplemented)\s*!\s*[(\[{]',masked):state='partial_stub'
        records.append({'path':path,'line':bisect.bisect_left(newlines,hit.start())+1,
                        'name':hit[1],'owner':owner[2] if owner else None,
                        'trait':owner[3] if owner else None,'state':state,
                        'body_sha256':hashlib.sha256(text[body[0]+1:body[1]].encode()).hexdigest(),
                        'body_start':body[0]+1,'body_end':body[1]})
    return records

def scan(workspace):
    lex=lexer(workspace);records=[];files=[]
    names=subprocess.check_output(['rg','--files',str(workspace/'tiny-glade/crates'),'-g','*.rs',
        '-g','!**/target/**','-g','!**/recovery/**','-g','!**/thirdparty/**',
        '-g','!**/tests/**','-g','!**/examples/**','-g','!**/benches/**'],text=True).splitlines()
    for name in sorted(names):
        path=Path(name)
        if 'src' not in path.parts or path.stem.endswith(('_tests','_test')) or path.stem in ('tests','test'):continue
        raw=path.read_bytes();sha=hashlib.sha256(raw).hexdigest()
        relative=path.relative_to(workspace).as_posix()
        files.append({'path':relative,'sha256':sha})
        for row in functions(raw.decode('utf-8',errors='replace'),relative,lex):
            row['source_sha256']=sha;records.append(row)
    return records,files

def associate(name,original_path,records):
    from graph_filter import normalized_name
    norm=normalized_name(name);parts=norm.split('::');leaf=parts[-1]
    owner=parts[-2] if len(parts)>1 and not parts[-2].startswith('impl$') else None
    # Uppercase nominal types carry stronger evidence than module/impl$ names.
    owner=owner if owner and owner[0].isupper() else None
    candidates=[r for r in records if r['name']==leaf]
    current='tiny-glade/'+original_path
    same=[r for r in candidates if r['path']==current]
    if owner:
        owned=[r for r in candidates if r['owner']==owner]
        same_owned=[r for r in owned if r['path']==current]
        if same_owned:return same_owned,'same_file_and_owner_candidate'
        if owned:return owned,'owner_candidate_across_files'
        # Do not associate another nominal type just because the file matches.
        same=[r for r in same if r['owner'] is None]
        candidates=[r for r in candidates if r['owner'] is None]
    if same:return same,'same_file_candidate'
    # Relocated free functions can be located by matching a meaningful filename.
    # mod.rs/lib.rs are shared by many unrelated owners and must not match.
    filename=Path(original_path).name
    if filename not in ('mod.rs','lib.rs','main.rs'):
        moved=[r for r in candidates if Path(r['path']).name==filename]
        if moved:return moved,'same_filename_relocation_candidate'
    return [],'unresolved_source_association'

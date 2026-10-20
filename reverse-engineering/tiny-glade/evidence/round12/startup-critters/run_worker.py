"""Launch only this worker with settings redirected into its workspace root."""
from pathlib import Path
import os,runpy,sys
job_prefix=sys.argv[1] if len(sys.argv)>1 else 'decomp'
root=Path('D:/game');settings=root/'tools/reverse-runtime/startup-critters-full/appdata';settings.mkdir(parents=True,exist_ok=True)
# Process-local settings location, not a machine/user environment mutation.
os.environ['APPDATA']=str(settings)
os.environ.pop('SSLKEYLOGFILE',None)
entry=root/'reverse-engineering/tiny-glade/scripts/reverse_tools/reverse_operator.py'
sys.path.insert(0,str(entry.parent))
sys.argv=[str(entry),'ghidra',str(root/f'reverse-engineering/tiny-glade/evidence/round12/startup-critters/{job_prefix}-job.json'),str(root/f'reverse-engineering/tiny-glade/evidence/round12/startup-critters/{job_prefix}-result.json'),'--worker','startup-critters-full','--write']
runpy.run_path(str(entry),run_name='__main__')

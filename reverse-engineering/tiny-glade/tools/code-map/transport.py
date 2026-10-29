"""Thread-local persistent local HTTP, with no automatic transaction replay."""
import base64
from functools import lru_cache
import http.client
import os
from pathlib import Path
import threading
import time
from urllib.parse import urlsplit
from artifacts import load

_state=threading.local()

@lru_cache(maxsize=8)
def _configuration(path,stamp):
    config=load(path)
    parsed=urlsplit(config['url'])
    if parsed.scheme!='http' or parsed.hostname not in ('127.0.0.1','localhost','::1'):
        raise ValueError('Code-map transport requires the configured local HTTP endpoint')
    auth=base64.b64encode((config['user']+':'+config['password']).encode()).decode()
    return config,parsed.hostname,parsed.port or 80,parsed.path.rstrip('/'),auth

def configuration(path):
    path=Path(path);stat=path.stat()
    stamp=(stat.st_dev,stat.st_ino,stat.st_size,stat.st_mtime_ns,stat.st_ctime_ns)
    return _configuration(str(path),stamp)

def close_connection():
    connection=getattr(_state,'connection',None)
    if connection is not None:connection.close()
    _state.connection=None

def request(config,payload,timeout):
    _,host,port,prefix,auth=config
    key=(os.getpid(),host,port,prefix,auth)
    if (getattr(_state,'key',None)!=key or
            time.monotonic()-getattr(_state,'last_used',0)>5):
        close_connection()
    connection=getattr(_state,'connection',None)
    if connection is None:
        connection=http.client.HTTPConnection(host,port,timeout=timeout)
        _state.connection=connection;_state.key=key
    connection.timeout=timeout
    if connection.sock is not None:connection.sock.settimeout(timeout)
    try:
        connection.request('POST',prefix+'/db/neo4j/tx/commit',body=payload,
            headers={'Authorization':'Basic '+auth,'Content-Type':'application/json'})
        response=connection.getresponse()
        raw=response.read()
        if response.will_close:close_connection()
        if not 200<=response.status<300:
            raise RuntimeError(f'Neo4j HTTP status {response.status}')
        _state.last_used=time.monotonic()
        return raw
    except Exception:
        # A server may have committed before a broken connection was observed.
        # Never retry an ambiguous transaction, including write/import requests.
        close_connection()
        raise

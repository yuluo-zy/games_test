#!/usr/bin/env python3
"""Fetch only the pinned official Neo4j archive into the ignored local runtime."""
import os
import tarfile
import urllib.request
from artifacts import RUNTIME, digest, space
from graph import ARCHIVE_SHA, runtime

def main():
    RUNTIME.mkdir(parents=True,exist_ok=True);space(50)
    folder=RUNTIME/'downloads';folder.mkdir(exist_ok=True)
    archive=folder/'neo4j-community-5.26.32-unix.tar.gz'
    if not archive.exists():
        opener=urllib.request.build_opener(urllib.request.ProxyHandler({}))
        temp=archive.with_suffix('.download')
        with opener.open('https://dist.neo4j.org/'+archive.name,timeout=60) as source,temp.open('xb') as out:
            while block:=source.read(2**20):out.write(block)
        if digest(temp)!=ARCHIVE_SHA:raise ValueError('Official archive hash mismatch; partial download retained')
        temp.replace(archive)
    if digest(archive)!=ARCHIVE_SHA:raise ValueError('Archive hash mismatch')
    if not (RUNTIME/'neo4j-community-5.26.32').exists():
        with tarfile.open(archive) as tar:tar.extractall(RUNTIME,filter='data')
    if not (RUNTIME/'connection.json').exists():print(runtime('configure'))
    print('Pinned runtime ready; start separately with code_map.py runtime start')

if __name__=='__main__':main()

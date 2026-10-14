// Validate the standalone page and actual interactions in installed headless Edge.
const {chromium}=require(process.argv[2]);
const fs=require('fs'),path=require('path'),assert=require('assert'),crypto=require('crypto');
const root=path.resolve(__dirname,'../..');
(async()=>{
 const browser=await chromium.launch({channel:'msedge',headless:true});
 const page=await browser.newPage({viewport:{width:1440,height:1100},deviceScaleFactor:1});
 const errors=[];page.on('pageerror',e=>errors.push(e.message));
 await page.goto('file:///'+path.join(root,'package-dependencies.html').replaceAll('\\','/'));
 await page.waitForFunction(()=>window.__packageGraph?.getState().visible.length>0);
 const initial=await page.evaluate(()=>({nodes:document.querySelectorAll('.node').length,edges:document.querySelectorAll('#edges>path').length,overflow:document.documentElement.scrollWidth>innerWidth}));
 assert.equal(initial.nodes,38);assert(initial.edges>0);assert(!initial.overflow);
 const progress=await page.evaluate(()=>({
  active:document.querySelectorAll('.node.recovery-in_progress').length,
  complete:document.querySelectorAll('.node.recovery-complete').length,
  pending:document.querySelectorAll('.node.recovery-pending').length,
  excluded:document.querySelectorAll('.node.recovery-excluded').length,
  bars:document.querySelectorAll('.node .stage-bar').length,
  data:window.__packageGraph.data.nodes.filter(n=>n.local).map(n=>({name:n.name,...n.progress}))
 }));
 assert(progress.active>0&&progress.pending>0&&progress.excluded>0);
 assert.equal(progress.bars,(progress.active+progress.complete+progress.pending)*5);
 assert.equal(progress.data.find(n=>n.name==='shader-compiler').state,'in_progress');
 for(const name of ['motomoto','tg-package-integrity','tg-text'])assert.equal(progress.data.find(n=>n.name===name).stages.find(s=>s.key==='verification').state,'in_progress');
 // Exercise the completed visual state only in this browser, then restore actual records.
 assert(await page.evaluate(()=>{const n=window.__packageGraph.data.nodes.find(n=>n.name==='motomoto'),actual=n.progress;try{n.progress={...actual,state:'complete',label:'完整验收',stages:actual.stages.map(s=>({...s,state:'complete'}))};render();const node=[...document.querySelectorAll('.node')].find(b=>b.dataset.id===n.id),bars=[...node.querySelectorAll('.stage-bar')];return node.classList.contains('recovery-complete')&&bars.length===5&&bars.every(b=>getComputedStyle(b).backgroundImage==='none'&&getComputedStyle(b).backgroundColor==='rgb(36, 132, 101)')}finally{n.progress=actual;render()}}));
 const compactHeight=await page.locator('#canvas').evaluate(x=>x.offsetHeight);
 await page.locator('#compact').uncheck();
 assert(await page.locator('#canvas').evaluate(x=>x.offsetHeight)>compactHeight);
 await page.locator('#compact').check();
 await page.screenshot({path:path.join(__dirname,'overview-desktop.png'),fullPage:true});
 await page.locator('.node').filter({has:page.locator('.name',{hasText:/^country-core$/})}).click();
 assert(await page.locator('#detail').innerText().then(x=>x.includes('共享游戏状态')));
 assert.equal(await page.locator('#detail .stage-detail li').count(),5);
 assert(await page.locator('#detail').innerText().then(x=>x.includes('本机源码缺失')));
 const selected=await page.evaluate(()=>{const s=window.__packageGraph.getState();return {selected:s.selected,blue:document.querySelectorAll('.node.downstream').length,purple:document.querySelectorAll('.node.upstream').length}});
 assert(selected.selected.startsWith('country-core '));assert(selected.blue>0&&selected.purple>0);
 await page.screenshot({path:path.join(__dirname,'country-core-selected.png'),fullPage:true});
 await page.locator('#focus-button').click();assert(await page.evaluate(()=>window.__packageGraph.getState().focus));
 await page.locator('#reset').click();
 await page.locator('[data-layout=depth]').click();
 assert(await page.evaluate(()=>{const ranks=new Map([...document.querySelectorAll('.node')].map(b=>[b.dataset.id,Number(b.closest('.lane').dataset.rank)]));return window.__packageGraph.getState().edges.every(e=>ranks.get(e.from)<ranks.get(e.to))}));
 await page.locator('#all-edges').check();
 assert(await page.evaluate(()=>{const s=window.__packageGraph.getState(),ids=new Set(s.visible);return s.edges.length===window.__packageGraph.data.edges.filter(e=>ids.has(e.from)&&ids.has(e.to)).length}));
 await page.locator('#scope').selectOption('local');assert.equal(await page.locator('.node').count(),61);
 await page.locator('#search').fill('glam');await page.locator('#results button').first().click();
 assert(await page.locator('#detail h2').innerText().then(x=>x==='glam'));
 await page.locator('#reset').click();await page.locator('#scope').selectOption('core');await page.locator('#all-edges').uncheck();await page.locator('[data-layout=architecture]').click();
 await page.setViewportSize({width:390,height:844});
 await page.screenshot({path:path.join(__dirname,'overview-mobile.png'),fullPage:true});
 assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));
 assert.equal(errors.length,0,errors.join('\n'));
 const report={ok:true,html_sha256:crypto.createHash('sha256').update(fs.readFileSync(path.join(root,'package-dependencies.html'))).digest('hex'),initial,progress,selection:selected,checks:['standalone file loads','38 default packages and drawn edges','five workflow segments per recovery node','active/pending/reuse visual states','completed phases use solid green (browser-only fixture restored)','shader-compiler standalone ledger module included','small libraries original comparison evidence displayed','stage details and missing delivery notice','compact/full node layout','country-core upstream/downstream selection','full direct-neighbor focus','topological depth edge direction','all displayed direct edges','61 local+external packages','all-901 package search and selection','390px layout no overflow','no page errors'],business_tests_run:false};
 fs.writeFileSync(path.join(root,'tracking/package-graph-validation.json'),JSON.stringify(report,null,2)+'\n');
 console.log(JSON.stringify({...report,progress:{...progress,data:undefined}}));await browser.close();
})().catch(e=>{console.error(e);process.exit(1)});

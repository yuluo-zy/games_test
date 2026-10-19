# In-progress Cottage10926B derivation, not an accepted design or implementation

Native function14128db20/10926B and its1362B cap callback1412905d0 are retained complete in native-roof-producer-pack-v1; call-bindings and inline-symbolized copies remain unchanged. This note is only a continuation aid. The retained Ghidra compact body is not trusted ABI evidence. No source/Cargo/test/original execution is included.

Actual ABI: RCX=&CachedDecoratorTransform; XMM1=width; XMM2=height_offset; R9=rank; stack20=&Roof,28=&emit,30=expansion_margin,38=UI stream,40=boolflag. Required rank1..3 follows Roof.height() call. Rank multipliers[.1,.15,.2]. The actual transform owner is CachedDecoratorTransform. Callback output is canonical64B RoofTileSsbo, never a Vec<twohalfcoords>.

Proven native setup PCddde8..e156:
- baseY=transform.translation.y. baseHeight=baseY+height_offset; topHeight=Roof.height()*factor+baseHeight.
- initial ShapeParameters is Roof.get_roof_shape_from_pos_ws_clamped(actual translation), getClosestU(actual XZ), getPosU; then lift that point with baseHeight (the shape query itself used baseY before offset).
- top query uses actual translation XZ/topHeight, projects via closestU/getPosU and lifts with topHeight.
- slope length is distance between those two lifted projected points. One RNG seeded from length(translation*10) as saturatingu64, same original f32 grouping. No default rectangle/transform/RNG.
- row boundary count max2(ceilf(slopeLength/.4) as usize), jitter exact bits3da3d70b/slopeLength.142925970=.4, not12;142c290f0=.08000000566244125.
- initial normalized width interval is(width/initialShape.perimeter())*[-.5,+.5].

Proven row setup PCe1cc..e497:
- For each row split s first interpolate the projected lower/upper Y as upperY*s+lowerY*(1-s), then inverse_lerp(baseHeight,topHeight,value). Keep this explicit remap; do not simplify to s.
- Pair these remapped t values in original iterator order. Actual low/high row Y is topHeight*t+baseY*(1-t), not baseHeight*(1-t); this drops height_offset at the first actual row. The original cap capture keeps these same section-query heights in2f8/358; vertex lifts later use baseHeight*(1-t)+topHeight*t instead. Keep the two distinct pairs.
- Get actual roof clamped sections at each rowY and expand each by (1-t)*margin+t*0, preserving zero product. Expand returns a new record (pure SRET); it does not mutate Roof.
- Query each expanded shape's actual closestU at cached translation XZ.
- low row width=(lowExpanded.perimeter()/initialShape.perimeter())*inputWidth.
- columns count is saturatingusize(lowRowWidth/.3), max2; there is no ceilf call at e418. Jitter=.06/lowRowWidth (142c20b3c=.06000000238418579).
- Both row normalized-U widths retain the initial width/perimeter interval, which is consistent with scaling physical width by current perimeter. Endpoints wrap using fmodf with period1, adding1 only when remainder<0; forward interpolation crosses0 by raising endU+1 when needed.

Actual shared dependency defect proven separately: ShapeParameters1411ff660 directly tail-dispatches canonical Rectangle140c9b000/Circle140c976f0, but current Shape::Rectangle calls an old duplicate loop. Root delegated TGf2 historical rootclaim narrowly; independent contract d218e5ab pending07. Do not write that source before design review. This note does not authorize broader Shape changes.

Still to derive and independently review: exact four corners/per-column failure ordering; special first-row clipping/lip writes; general body orientation/random/overlap; flags controlling edge/cap closure; all1362B cap callback order; full main cache/generator integration. Do not register current placeholder or use uniform quads/capacity-continue/dummygeometry. The Gothic consumer has separate frozen business design69551120 with explicit strict-circle/constructor/section-domain gates; the two geometry paths must not be conflated.


Cap callback body1362B newly re-read (not yet separate accepted design):
- Actual RCX is SRET canonical64B tile, RDX captures two height refs / UIunused / oneRNGref / Roofref, R8=&from, R9=&to, stack20=sidebool. It RETURNS a RoofTileSsbo; it does not emit itself. Parent emits the returned record atf852/fb22 after callsf80b/fadb. Keep this distinction when implementing.
- bottom=f32::min(height0,height1)-.1 with original NaN-min selection; centerXZ=(fromXZ+toXZ)*.5; centerY=toY*.5+bottom*.5. width=sqrt(dz²+dx²),height=toY-bottom,depth=.1.
- Direction is normalize(toXZ-fromXZ) liftedX0Z, all raw f32. Normal=direction cross Y, unnormalized extra cross but direction alreadyunit. Quaternion from_mat3(cols(direction,Y,normal)). True flag keeps quaternion and uses center-normal*.08; false uses center+normal*.08 and multiplies by nativeYpi quaternion(0,1,0,bitsb33bbd2e), not W0.
- Clipping normal=(to-from) cross normal; not normalized. Last plane component=-dot(to,clippingnormal) with native x/y/z accumulation. Pack quat,plane,3dims+zero u16, then one sameRNG i32(-100..100). Afterwards read Roof's raw low32id and return all64 fields; currentHeight0,velocity0,animation1,flags0.
- side±offset uses exact142c290f4=.08 and142a88890 vector; parent laterpush still ignores NoFreeChunks. No RNG in emitter wrapper. Required signedzero/Quaternion multiplication grouping must be in actual fullcontract/review before candidate.


Continuation2026-10-20 (still working notes, not independently accepted):
- Direct original PE constant142a16d10 is f32 -10 (bitsc1200000), not +2. The general body random rotation atf3a4..f3e6 is (((r*-10)-((1-r)+(1-r)))*degree)*.5, corresponding to negative2..negative10 degrees. One f32 RNG then sinf/cosf, quaternion packing/dims packing then one i32 seed. No extra jitter/rng draws may be added. Exact constant reads are in constants-read-v1.json.
- Side trim height expansion1429cd120 is .16 (bits3e23d70a), not .1; dimensions are(.15,length(A-C)+.16,.2) at first column and(.15,length(B-D)+.16,.2) at last column. Actual rotation_from_normal140c90a40 call atf605/f8d3 then multiply originalQuatX90 bits3f3504f3. Current shared public rotation_from_normal deliberately uses finite/fallback policies; strict original invalid-domain compatibility remains a shared dependency gate, not permission to revert the public helper.
- Once four corners are lifted, column output order is first-row extra lip if rowIndex0, then general body, then first-column trim if colIndex0, optional first cap(side=false), then last-column trim if colIndex==splitLen-2, optional last cap(side=true). First and last tests are independent; if a single interval has both edges, both branches execute. Caps return64B and parentpush; flag only gates caps, never trim strips.
- Corner names for the derived semantic implementation: A=lowerShape at first columnU; B=lowerShape at nextU; C=upperShape at first columnU; D=upperShape at nextU. General body midpoint=((A+B)+C+D)*.25; width=length(B-A); slope dimension=length(A-C)+.1; thickness=.1; orientation columns normalize(B-A), normalize((C*.5+D*.5)-(A*.5+B*.5)), cross(first,second). Preserve actual per-component scalar order rather than algebraically simplifying. This naming is tied to original locals uStack220/uStack230/uStack240/plStack250 and still needs exact lane trace in a frozen contract.
- The first-row lip width=length(B-A), depth=max(.3,length((overall lower-span endpoints midpoint)-lowerShape closest center)+.1). A separate upper half-point projection provides its horizontal direction from lower midpoint; it is raw normalized XZ, liftedX0Z; orientation matches ridge Mat3(normalize(dir.crossY),dir,Y). Position is lower midpoint+dir*(depth*.5). Emission occurs before general body/its random draw.
- Original cap bottom f32min(h0,h1)-.1 uses the Rust min NaN-selection sequence. Cap width sqrt(dz²+dx²); raw lifted-direction normalization separately evaluates ((dx²+0²)+dz²), retaining zero products. Offset .08 is bits3da3d70a. Native false-side Ypi W is nonzero bitsb33bbd2e. Cap q/plane/dims pack precedes i32 draw then Roof low32 read.


Height-lane clarification from nativee27f..e4f1 (not a frozen design correction because this file is working notes): baseY is rbp+3b0, baseHeight=baseY+offset is3c0, topHeight is3bc. Before the two section queries, h0=topHeight*t0+baseY*(1-t0) goes to2f8, h1=topHeight*t1+baseY*(1-t1) goes to358. These are exactly the two references captured atf57e/f58c for caps, and are also used as section-query Y. Only AFTER column-split RNG consumption at e497 does e4bd..e4f1 replace3c4/3b8 by vertexY0=topHeight*t0+baseHeight*(1-t0), vertexY1=topHeight*t1+baseHeight*(1-t1). A/B/C/D are lifted using those latter vertex heights. Do not call the cap/section heights the offset vertex heights. First-row extra lip samples the upper section's XZ at matching columnUs but lifts them with lower vertexY0 (ed1e/ed58); it then extracts only XZ for its horizontal direction.

Sharedrotation policy status: the provisional strict-policy gate above was withdrawn by frozen independentaddendum rotation-normal-native-policy-correction-v1.json fb13159c after complete original936B reuse/directPE audit. The native helper itself includes finite assertion, both reciprocal-validity fallbacks and final finite quaternion fallback. Do not ask for a new strict entry merely because current helper has those policies. Its complete arithmetic equivalence is still a bounded reuse audit, not assumed by spelling.

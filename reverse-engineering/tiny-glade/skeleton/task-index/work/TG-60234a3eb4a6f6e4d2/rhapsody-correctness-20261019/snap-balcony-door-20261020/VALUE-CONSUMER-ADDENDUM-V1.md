# Snap value-consumer addendum v1

This addendum is read-only original-value tracing, not authorization to rewrite calculate_decorator_interaction_intent or its corner/merge/output schema. The first design/frozen manifests stay immutable. B full calculate_decorator_interaction_intent is8546B (PDB1522:1440), not48KB native code; its full1713-instruction body and identity are now frozen. It contains one direct Snap call at14127e774. The separate actual location call14081090d is already frozen.

## Full-value UI caller14127e774

The UI caller writes prefer_wall_outline=false at e757 and passes require_upper_clearance=false at e765. It supplies actual Info/worldposition, probeXZ from its existing transform calculation, wall distance inXMM11, currentWallId/wall state, actual Query<&Roof>, mesh library and actual RaycastWorld. Do not normalize/recompute substitute direction values in the Snap callee. The call does not need initialized second-coordinateY/UiStream slots, consistent with those slots being unread by the complete Snap function.

- e779 reads tag from returned16B+12. e780..e785 checks `(tag-3) as u8 <2`, so native tags3 and4 take the no-payload branch. e787 sets accepted/applied flagfalse; keeps existing platform/wall reference and existing height value from prior caller state. It must never read uninitialized wall/height payload for these variants. The original caller still carries native tag onward; no need to make these two states identical in the public enum. Tag4 is unreachable from this caller's false upper-clearance flag, but the consumer branch is explicitly shared.
- e792/e799 read returnedWallId andreturnedHeight ONLY for tags0/1/2. e7a4 calls actual DecoratorInfo::try_dst_as_wall1408333a0. If Some, e7ae..e7c3 checks platform WallId at info+8 using WallId::is_valid140b34f20, OR native bottom-door byte info+21. None starts false. e7c7 ORs a real incoming caller flag at frame+2e8; e7ce keeps its lowbit. The caller flag's author name is not recovered here; preserve its producer, do not default it.
- e7db/e7e2: only when that real application gate istrue does returnedWallId replace the caller's prior platform/wall value. e7e6..e7f2: when false restore priorheight (XMM9); when true carry returnedheight (XMM6) intoXMM9. This gate does not authorize clearing or replacing actual Info/source state inside Snap.
- e7d2..e7ed computes `applied && native_tag==1` as a separate bool. AdjacentRoof tag0 andColliderSource13 tag2 can apply wall/height but must not be mistaken for the tag1-specific flag. Native tag1 means WallOutline in the proposed enum.
- final selected caller branch writes accepted height bits at output+2c(e87f), selectedplatform/wall at+30(e886), tag1-specific flag at+39(e897), and nativeSnap tag extended intoqword+44(e8a9). These are observed fields of this original output variant only. They do not establish the current simplified DecoratorInteractionIntent struct's fullABI or permit rewriting its fields under this Snap-only task. Final branch selection and parentproposal fields remain outside current implementation scope.

Thus the actual full-value consumer policy is:

| Snap variant | Original payload source | Read payload? | Application condition | Special bool |
| --- | --- | --- | --- | --- |
| tag0 AdjacentFlatRoof | first qualifying otherflatRoof.wall_id, max(elevation,roof.floorY-.79)+halfheight | yes | actual platformvalid OR bottomdoor OR incomingflag | false |
| tag1 WallOutline | currentWallId, max(halfheight,sampledoutlineY+halfheight+.05) | yes | same actual gate | applied |
| tag2 ColliderSource13 | currentWallId, max(halfheight,actual finite-source13 hitY+halfheight+.05) | yes | same actual gate | false |
| tag3 Unavailable | no initialized payload | no | never applies | false |
| tag4 RejectedUpperClearance | no initialized payload | no | never applies | false |

A later genuine UI caller adapter must preserve these values and the real incoming gate. It cannot take Option<WallId>, replace all three selected variants by bool, use a fabricatedheight/defaultplatform or normalize every result as success. This round only supplies the five-state Snap result and actual validate/location adapter; the current partial UIcalculate remains unmodified/unaccepted.

## Location validation caller14081090d

Actual location invokes true/true and only observes tag4 at140810912. Tag4 -> Reason6{currentWallId}; every other tag continues its actual subsequent original checks. It does not apply a Snap height/wall or treat Unavailable as a blanket successful placement. Incoming01adapter owns this continuation and actual source/outline feedback. Exact enum matching RejectedUpperClearance is sufficient for this particular caller while keeping the other four values available and typed.

## Current integration boundary

Production world now919bfa6980ff3de6b2d19d394fc1d4b0833dd0ff3e1f40d5096a9fd6fef3d481 contains the reviewed finite simple-ray API; source integration is not runtime acceptance. Snap source remains da0e2a17... until design review and candidate. The original helper calls/actual values above are evidence for consumer completeness, not a claim that the current calculate or location producer is already implemented and tested.

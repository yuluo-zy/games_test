# Retraction of Window native48 naming conclusion

FACT_CORRECTION_V1(6ba23ced) and FACT_MANIFEST_V1(ded4de15) remain historical rejected artifacts. Their assertion that the second append receives the early modified ID at40 and the earlier shutter_right hash at48 is withdrawn. Independent07 review found, and the complete producer confirms, intervening writes to both reused stack slots. Local address equality is not a def-use proof across those writes. No field rename, source candidate or production change was made.

For the first/right append, local280's earlier hash is read at1407ee863 and written to candidate40 at ee86a. The second/left append instead uses the new hash written to local278 at ee8c9, then reads it at ee9c7 into candidate40. Before the second native48 load, ee8e1 has overwritten local280 from local1f0. The original native40 -> drain output40 -> manager HashMap key -> AutoClutter.key chain remains independently proven; it does not validate the rejected producer link.

No proof that the existing physical-wall label is wrong has been established by this packet.04 is separately completing the real last definitions of local1f8/local1f0 for native48. Preserve both values and offsets; no swapping or original behavior repair is authorized from the rejected inference. The attached JSON distinguishes the actual two append lifetimes; all unknown source definitions remain explicit.

No CPU/GPU test, Cargo, compiler or original target execution occurred.

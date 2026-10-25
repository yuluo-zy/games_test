#![allow(dead_code)]
#[path = "D:/game/reverse-engineering/tiny-glade/reconstruction/render/work-packages/device-counters-r6/src/lib.rs"] mod device;
#[path = "D:/game/reverse-engineering/tiny-glade/reconstruction/render/work-packages/sequence-domain-r5/src/lib.rs"] mod sequence;
#[path = "D:/game/reverse-engineering/tiny-glade/reconstruction/render/work-packages/record-submit-r6/src/lib.rs"] mod record;
#[test] fn queue_domain_and_fixture_seed_boundary() {
 let counters=device::DeviceCounters::new(1);
 assert_eq!(counters.begin_sequence(device::QueueIdentity::Primary),Some(1));
 assert_eq!(counters.begin_sequence(device::QueueIdentity::Auxiliary(0)),Some(1));
 let mut d=sequence::SequenceDomain::default();
 let p=sequence::QueueIdentity{tag:0,aux_index:0};let a=sequence::QueueIdentity{tag:1,aux_index:0};
 assert_eq!(d.begin_cb(p),Ok(0)); // Explicit R5B fixture seed, not constructor behavior.
 assert_eq!(d.begin_cb(a),Err(sequence::Error::SequenceCollision(0)));
}
#[test] fn record_submit_timeline_domain() {
 let c=device::DeviceCounters::new(0);let mut d=sequence::SequenceDomain::default();
 let p=sequence::QueueIdentity{tag:0,aux_index:0};let a=sequence::QueueIdentity{tag:1,aux_index:0};
 let original_sequence=c.begin_sequence(device::QueueIdentity::Primary).unwrap();let fixture_sequence=d.begin_cb(p).unwrap();
 assert_eq!(original_sequence,fixture_sequence+1); // Bounded fixture adapter, not production mapping.
 let r=record::record(4,None,true);assert!(!r.panic);assert_eq!(r.command_states,[2,1]);
 let s=record::submit(r.command_states,false,false);assert_eq!(s.command_states,[2,2]);
 let (queue,timeline)=s.receipt.unwrap();assert_eq!(queue,0);assert_eq!(timeline,41);assert_ne!(timeline,original_sequence);
 assert_eq!(d.submit(a,fixture_sequence,timeline),Err(sequence::Error::QueueMismatch));
 d.submit(p,fixture_sequence,timeline).unwrap();assert!(d.complete(p,timeline-1).unwrap().is_empty());
 assert_eq!(d.complete(p,timeline).unwrap(),[fixture_sequence]);d.dispose(fixture_sequence).unwrap();
}
#[test] fn callback_and_end_failure_block_submit() {
 let r=record::record(3,Some(1),true);assert!(r.panic);assert_eq!(r.command_states,[1]);assert_eq!(r.consumed_passes,2);
 let s=record::submit(vec![2,1],true,false);assert!(s.panic);assert!(s.receipt.is_none());assert!(!s.events.iter().any(|e|matches!(e,record::Event::Queue(_))));
}

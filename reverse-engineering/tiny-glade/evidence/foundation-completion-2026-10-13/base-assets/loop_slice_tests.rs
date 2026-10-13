use super::*;
#[test]
fn original_b_loop_slice_state_transitions() {
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:0,end_idx:0,period:1}.map_slice(&source);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:0,end_idx:0,period:1}.map_slice(&source);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:0,end_idx:0,period:1}.map_slice(&source);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:0,end_idx:1,period:1}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:0,end_idx:1,period:1}.map_slice(&source);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:0,end_idx:1,period:1}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:0,end_idx:2,period:1}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 2, 1]);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:0,end_idx:2,period:1}.map_slice(&source);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 1, 1]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:0,end_idx:2,period:1}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 2, 1]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:-1,end_idx:1,period:1}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 1, 1]);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:-1,end_idx:1,period:1}.map_slice(&source);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, 0, 1]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, -1, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, -1, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, -1, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:-1,end_idx:1,period:1}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 1, 1]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:1,end_idx:2,period:1}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:1,end_idx:2,period:1}.map_slice(&source);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 1]);
}
{let source:Vec<_>=(0..1).collect();let mut view=LoopRange{start_idx:1,end_idx:2,period:1}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 1]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 1]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 1]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:0,end_idx:0,period:2}.map_slice(&source);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:0,end_idx:0,period:2}.map_slice(&source);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:0,end_idx:0,period:2}.map_slice(&source);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:0,end_idx:2,period:2}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 2, 2]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:0,end_idx:2,period:2}.map_slice(&source);
assert_eq!(view.next_back(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 1, 2]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:0,end_idx:2,period:2}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 2, 2]);
assert_eq!(view.next_back(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:0,end_idx:3,period:2}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 3, 2]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 3, 2]);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:0,end_idx:3,period:2}.map_slice(&source);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 2, 2]);
assert_eq!(view.next_back(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 1, 2]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:0,end_idx:3,period:2}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 3, 2]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 2, 2]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:-1,end_idx:2,period:2}.map_slice(&source);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 2, 2]);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 2, 2]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:-1,end_idx:2,period:2}.map_slice(&source);
assert_eq!(view.next_back(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, 1, 2]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, 0, 2]);
assert_eq!(view.next_back(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, -1, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, -1, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, -1, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:-1,end_idx:2,period:2}.map_slice(&source);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 2, 2]);
assert_eq!(view.next_back(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 1, 2]);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:2,end_idx:3,period:2}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:2,end_idx:3,period:2}.map_slice(&source);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 2]);
}
{let source:Vec<_>=(0..2).collect();let mut view=LoopRange{start_idx:2,end_idx:3,period:2}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 2]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 2]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 2]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:0,end_idx:0,period:3}.map_slice(&source);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:0,end_idx:0,period:3}.map_slice(&source);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:0,end_idx:0,period:3}.map_slice(&source);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:0,end_idx:3,period:3}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 3, 3]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 3, 3]);
assert_eq!(view.next(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:0,end_idx:3,period:3}.map_slice(&source);
assert_eq!(view.next_back(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 2, 3]);
assert_eq!(view.next_back(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 1, 3]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:0,end_idx:3,period:3}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 3, 3]);
assert_eq!(view.next_back(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 2, 3]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:0,end_idx:4,period:3}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 4, 3]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 4, 3]);
assert_eq!(view.next(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 4, 3]);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:0,end_idx:4,period:3}.map_slice(&source);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 3, 3]);
assert_eq!(view.next_back(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 2, 3]);
assert_eq!(view.next_back(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 1, 3]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:0,end_idx:4,period:3}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 4, 3]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 3, 3]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 3, 3]);
assert_eq!(view.next_back(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 2, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:-1,end_idx:3,period:3}.map_slice(&source);
assert_eq!(view.next(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 3, 3]);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 3, 3]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 3, 3]);
assert_eq!(view.next(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:-1,end_idx:3,period:3}.map_slice(&source);
assert_eq!(view.next_back(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, 2, 3]);
assert_eq!(view.next_back(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, 1, 3]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, 0, 3]);
assert_eq!(view.next_back(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, -1, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, -1, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, -1, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:-1,end_idx:3,period:3}.map_slice(&source);
assert_eq!(view.next(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 3, 3]);
assert_eq!(view.next_back(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 2, 3]);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 2, 3]);
assert_eq!(view.next_back(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 1, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:3,end_idx:4,period:3}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:3,end_idx:4,period:3}.map_slice(&source);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 3]);
}
{let source:Vec<_>=(0..3).collect();let mut view=LoopRange{start_idx:3,end_idx:4,period:3}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 3]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 3]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 3]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:0,end_idx:0,period:7}.map_slice(&source);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:0,end_idx:0,period:7}.map_slice(&source);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:0,end_idx:0,period:7}.map_slice(&source);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:0,end_idx:7,period:7}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 7, 7]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 7, 7]);
assert_eq!(view.next(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 7, 7]);
assert_eq!(view.next(),Some(&3));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 7, 7]);
assert_eq!(view.next(),Some(&4));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[5, 7, 7]);
assert_eq!(view.next(),Some(&5));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[6, 7, 7]);
assert_eq!(view.next(),Some(&6));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[7, 7, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[7, 7, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[7, 7, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:0,end_idx:7,period:7}.map_slice(&source);
assert_eq!(view.next_back(),Some(&6));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 6, 7]);
assert_eq!(view.next_back(),Some(&5));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 5, 7]);
assert_eq!(view.next_back(),Some(&4));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 4, 7]);
assert_eq!(view.next_back(),Some(&3));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 3, 7]);
assert_eq!(view.next_back(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 2, 7]);
assert_eq!(view.next_back(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 1, 7]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:0,end_idx:7,period:7}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 7, 7]);
assert_eq!(view.next_back(),Some(&6));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 6, 7]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 6, 7]);
assert_eq!(view.next_back(),Some(&5));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 5, 7]);
assert_eq!(view.next(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 5, 7]);
assert_eq!(view.next_back(),Some(&4));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 4, 7]);
assert_eq!(view.next(),Some(&3));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:0,end_idx:8,period:7}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 8, 7]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 8, 7]);
assert_eq!(view.next(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 8, 7]);
assert_eq!(view.next(),Some(&3));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 8, 7]);
assert_eq!(view.next(),Some(&4));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[5, 8, 7]);
assert_eq!(view.next(),Some(&5));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[6, 8, 7]);
assert_eq!(view.next(),Some(&6));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[7, 8, 7]);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[8, 8, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[8, 8, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[8, 8, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:0,end_idx:8,period:7}.map_slice(&source);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 7, 7]);
assert_eq!(view.next_back(),Some(&6));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 6, 7]);
assert_eq!(view.next_back(),Some(&5));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 5, 7]);
assert_eq!(view.next_back(),Some(&4));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 4, 7]);
assert_eq!(view.next_back(),Some(&3));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 3, 7]);
assert_eq!(view.next_back(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 2, 7]);
assert_eq!(view.next_back(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 1, 7]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 0, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:0,end_idx:8,period:7}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 8, 7]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 7, 7]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 7, 7]);
assert_eq!(view.next_back(),Some(&6));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 6, 7]);
assert_eq!(view.next(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 6, 7]);
assert_eq!(view.next_back(),Some(&5));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 5, 7]);
assert_eq!(view.next(),Some(&3));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 5, 7]);
assert_eq!(view.next_back(),Some(&4));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 4, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:-1,end_idx:7,period:7}.map_slice(&source);
assert_eq!(view.next(),Some(&6));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 7, 7]);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 7, 7]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 7, 7]);
assert_eq!(view.next(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 7, 7]);
assert_eq!(view.next(),Some(&3));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[4, 7, 7]);
assert_eq!(view.next(),Some(&4));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[5, 7, 7]);
assert_eq!(view.next(),Some(&5));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[6, 7, 7]);
assert_eq!(view.next(),Some(&6));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[7, 7, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[7, 7, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[7, 7, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:-1,end_idx:7,period:7}.map_slice(&source);
assert_eq!(view.next_back(),Some(&6));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, 6, 7]);
assert_eq!(view.next_back(),Some(&5));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, 5, 7]);
assert_eq!(view.next_back(),Some(&4));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, 4, 7]);
assert_eq!(view.next_back(),Some(&3));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, 3, 7]);
assert_eq!(view.next_back(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, 2, 7]);
assert_eq!(view.next_back(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, 1, 7]);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, 0, 7]);
assert_eq!(view.next_back(),Some(&6));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, -1, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, -1, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[-1, -1, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:-1,end_idx:7,period:7}.map_slice(&source);
assert_eq!(view.next(),Some(&6));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 7, 7]);
assert_eq!(view.next_back(),Some(&6));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[0, 6, 7]);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 6, 7]);
assert_eq!(view.next_back(),Some(&5));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[1, 5, 7]);
assert_eq!(view.next(),Some(&1));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 5, 7]);
assert_eq!(view.next_back(),Some(&4));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[2, 4, 7]);
assert_eq!(view.next(),Some(&2));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 4, 7]);
assert_eq!(view.next_back(),Some(&3));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[3, 3, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:7,end_idx:8,period:7}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[8, 8, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[8, 8, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[8, 8, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:7,end_idx:8,period:7}.map_slice(&source);
assert_eq!(view.next_back(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[7, 7, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[7, 7, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[7, 7, 7]);
}
{let source:Vec<_>=(0..7).collect();let mut view=LoopRange{start_idx:7,end_idx:8,period:7}.map_slice(&source);
assert_eq!(view.next(),Some(&0));assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[8, 8, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[8, 8, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[8, 8, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[8, 8, 7]);
assert_eq!(view.next(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[8, 8, 7]);
assert_eq!(view.next_back(),None);assert_eq!([view.range.start_idx,view.range.end_idx,view.range.period],[8, 8, 7]);
}
}

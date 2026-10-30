// 原1407d32d1..3755全部junction分支。
const ORIGINAL_JUNCTIONS: &[OriginalJunction] = &[
OriginalJunction { center:1, neighbors:&[2,3,4], counts:&[(1,2,2),(1,3,3),(1,4,4)], panic:false, edges:&[[1,2,0,1,3,0],[1,2,0,1,4,0],[1,3,0,1,4,0]] },
OriginalJunction { center:1, neighbors:&[2,3,4], counts:&[(1,3,3),(1,4,4)], panic:false, edges:&[[1,2,0,1,3,0],[1,2,0,1,4,0],[1,3,0,1,4,0]] },
OriginalJunction { center:1, neighbors:&[2,3,4], counts:&[(1,2,0),(1,3,3),(1,4,4)], panic:false, edges:&[[1,2,0,1,3,0],[1,2,0,1,4,0],[1,3,0,1,4,0]] },
OriginalJunction { center:1, neighbors:&[2,3,4], counts:&[], panic:false, edges:&[[1,2,0,1,3,0],[1,2,0,1,4,0],[1,3,0,1,4,0]] },
OriginalJunction { center:3, neighbors:&[1,5,8], counts:&[(1,3,2),(3,5,3),(3,8,4)], panic:false, edges:&[[1,3,1,3,5,0],[1,3,1,3,8,0],[3,5,0,3,8,0]] },
OriginalJunction { center:3, neighbors:&[1,5,8], counts:&[(3,5,3),(3,8,4)], panic:false, edges:&[[3,5,0,3,8,0]] },
OriginalJunction { center:3, neighbors:&[1,5,8], counts:&[(1,3,0),(3,5,3),(3,8,4)], panic:true, edges:&[] },
OriginalJunction { center:3, neighbors:&[1,5,8], counts:&[], panic:false, edges:&[[3,5,0,3,8,0]] },
OriginalJunction { center:10, neighbors:&[2,3,4], counts:&[(2,10,2),(3,10,3),(4,10,4)], panic:false, edges:&[[2,10,1,3,10,2],[2,10,1,4,10,3],[3,10,2,4,10,3]] },
OriginalJunction { center:10, neighbors:&[2,3,4], counts:&[(3,10,3),(4,10,4)], panic:false, edges:&[[3,10,2,4,10,3]] },
OriginalJunction { center:10, neighbors:&[2,3,4], counts:&[(2,10,0),(3,10,3),(4,10,4)], panic:true, edges:&[] },
OriginalJunction { center:10, neighbors:&[2,3,4], counts:&[], panic:false, edges:&[] },
OriginalJunction { center:3, neighbors:&[8,1,5], counts:&[(3,8,2),(1,3,3),(3,5,4)], panic:false, edges:&[[3,8,0,1,3,2],[3,8,0,3,5,0],[1,3,2,3,5,0]] },
OriginalJunction { center:3, neighbors:&[8,1,5], counts:&[(1,3,3),(3,5,4)], panic:false, edges:&[[3,8,0,1,3,2],[3,8,0,3,5,0],[1,3,2,3,5,0]] },
OriginalJunction { center:3, neighbors:&[8,1,5], counts:&[(3,8,0),(1,3,3),(3,5,4)], panic:false, edges:&[[3,8,0,1,3,2],[3,8,0,3,5,0],[1,3,2,3,5,0]] },
OriginalJunction { center:3, neighbors:&[8,1,5], counts:&[], panic:false, edges:&[[3,8,0,3,5,0]] },
];

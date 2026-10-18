use super::*;use glam::{Vec2,Vec3};
fn f(b:u32)->f32 {f32::from_bits(b)}
fn v2(b:[u32;2])->Vec2 {Vec2::new(f(b[0]),f(b[1]))}
fn v3(b:[u32;3])->Vec3 {Vec3::new(f(b[0]),f(b[1]),f(b[2]))}
fn eq(a:f32,b:u32){if f(b).is_nan(){assert!(a.is_nan())}else{assert_eq!(a.to_bits(),b)}}
#[test]fn original_b_aabb3_ray_segment(){
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([0,0,0]),direction:v3([1065353216,0,0])};let got=b.classify_ray_intersection(ray);
match got{RayBoxIntersection::Inside{enter,exit}=>{eq(enter,3212836864);eq(exit,1065353216);},_=>panic!("classification mismatch")}
let p=b.intersect_segment(ray.origin,v3([1065353216,0,0]));for (a,e)in p.unwrap().to_array().into_iter().zip([0,0,0]){eq(a,e)}}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3212836864,0,0]),direction:v3([1065353216,0,0])};let got=b.classify_ray_intersection(ray);
match got{RayBoxIntersection::Hit{enter,exit}=>{eq(enter,0);eq(exit,1073741824);},_=>panic!("classification mismatch")}
let p=b.intersect_segment(ray.origin,v3([0,0,0]));for (a,e)in p.unwrap().to_array().into_iter().zip([3212836864,0,0]){eq(a,e)}}
// Corrected: rays parallel to a face still intersect the closed box.
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1065353216,0,0]),direction:v3([0,1065353216,0])};let got=b.classify_ray_intersection(ray);
assert_eq!(got,RayBoxIntersection::Hit{enter:0.0,exit:2.0});
let p=b.intersect_segment(ray.origin,v3([1065353216,1065353216,0]));assert_eq!(p,Some(ray.origin));}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1077936128,0,0]),direction:v3([1065353216,0,0])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1082130432,0,0]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([0,0,0]),direction:v3([0,0,0])};let got=b.classify_ray_intersection(ray);
assert_eq!(got,RayBoxIntersection::NoHit); // A stationary point is not a ray.
let p=b.intersect_segment(ray.origin,v3([0,0,0]));for (a,e)in p.unwrap().to_array().into_iter().zip([0,0,0]){eq(a,e)}}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1073741824,1077936128,1082130432]),direction:v3([3212836864,3212836864,3212836864])};let got=b.classify_ray_intersection(ray);
match got{RayBoxIntersection::Hit{enter,exit}=>{eq(enter,1065353216);eq(exit,1077936128);},_=>panic!("classification mismatch")}
let p=b.intersect_segment(ray.origin,v3([1065353216,1073741824,1077936128]));for (a,e)in p.unwrap().to_array().into_iter().zip([1065353216,1073741824,1077936128]){eq(a,e)}}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3230096455,3217253588,1083179902]),direction:v3([1051265422,3230933555,1070031530])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3229194076,3234134888,1086446633]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3198148723,3213753522,1044188732]),direction:v3([1078201932,1076508544,3222447394])};let got=b.classify_ray_intersection(ray);
match got{RayBoxIntersection::Inside{enter,exit}=>{eq(enter,3194348668);eq(exit,1054561130);},_=>panic!("classification mismatch")}
let p=b.intersect_segment(ray.origin,v3([1076892221,1069969997,3221673022]));for (a,e)in p.unwrap().to_array().into_iter().zip([3198148723,3213753522,1044188732]){eq(a,e)}}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3218723670,3223420533,1081945898]),direction:v3([1029997290,1043460630,1082589905])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3218255678,3222691668,1090702643]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3229907904,3209547672,1081703663]),direction:v3([1055181973,1057187180,1075104643])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3228327406,3197424728,1086792761]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1063598905,1071306741,1083107249]),direction:v3([1078439301,1076871997,3213310543])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1082162730,1083086748,1079652922]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3222155954,3223168535,1082274137]),direction:v3([1059433293,1045564973,3226370704])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3217657789,3222308148,1062699143]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3225465098,3216716192,1081734908]),direction:v3([3201054745,3226294514,3225813136])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3227138061,3231021281,1062197682]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1079334202,3225494531,3230777931]),direction:v3([3218436826,3208647378,3229987999])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1070938011,3228641464,3238771573]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3228480837,1079652584,1073698318]),direction:v3([3230115165,3219640509,3216248280])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3237937152,1070371091,1058443373]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1083287319,1083048505,1069610124]),direction:v3([1082702255,3213422410,3229281606])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1091383395,1079479501,3222958849]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3191758217,3228754605,1080706741]),direction:v3([3221192867,1075699769,3216543968])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3221988906,3215590631,1074658885]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1078723137,1083640310,3222845006]),direction:v3([3218423056,1075614750,3219814123])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1069729650,1088771077,3230071010]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1074845322,3229027017,1083908002]),direction:v3([3230338505,3231167023,1081955160])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3221570825,3238632394,1091364007]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1070989155,3222099245,1057598596]),direction:v3([1080995858,3219847197,3221664563])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1085069282,3229706398,3217592356]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1076018100,1074190436,1072258681]),direction:v3([3210394778,1084184933,1069117482])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1071126812,1088603543,1079076689]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1066110652,3221773560,3215470803]),direction:v3([3228314833,1079402712,1079222242])};let got=b.classify_ray_intersection(ray);
match got{RayBoxIntersection::Hit{enter,exit}=>{eq(enter,1025493563);eq(exit,1058079270);},_=>panic!("classification mismatch")}
let p=b.intersect_segment(ray.origin,v3([3223741811,1067190207,1073680114]));for (a,e)in p.unwrap().to_array().into_iter().zip([1064452830,3221225472,3214388708]){eq(a,e)}}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3224591848,3228830868,3230217561]),direction:v3([1075480614,1054236271,3205526133])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3200691728,3227074758,3231400871]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3226506383,3229103347,1079811044]),direction:v3([1040931974,3229217893,3226395924])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3225935558,3237549228,1046179066]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3229369318,3207245508,1074964111]),direction:v3([3230276583,3220219112,3230858274])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3238272749,3223518757,3222491572]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3203770476,3205533555,3230120742]),direction:v3([1077648327,1077620222,3215563760])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1075635897,1075251746,3232899618]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1084106082,3216567762,3228065434]),direction:v3([1076432481,1083111089,1082386394])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1089645714,1078031993,1056671951]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1081647871,1072626430,3220769213]),direction:v3([3196889099,3215799110,3214893860])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1080495613,1057197937,3226220144]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3207314445,3219234071,3231665995]),direction:v3([3217473390,1078254303,3231578429])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3222163130,1067980967,3240010820]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3216030073,1082774037,1083964943]),direction:v3([3228345266,3218788260,3226219925])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3231875127,1076247640,1071090960]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1081873066,3230914494,3213200448]),direction:v3([3209622075,1082248816,3207127836])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1078482460,3205515889,3218734541]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1031253254,1073943219,1071699603]),direction:v3([3208127473,3223455190,3222370270])};let got=b.classify_ray_intersection(ray);
match got{RayBoxIntersection::Hit{enter,exit}=>{eq(enter,1016815650);eq(exit,1069332003);},_=>panic!("classification mismatch")}
let p=b.intersect_segment(ray.origin,v3([3207112993,3203897625,3204723282]));for (a,e)in p.unwrap().to_array().into_iter().zip([1027591058,1073741824,1071337968]){eq(a,e)}}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1070990937,3222385805,3218230608]),direction:v3([3224751152,3207617120,3231322721])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3214250504,3225275173,3234768309]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1083860107,1078093967,3224206522]),direction:v3([3211398696,1076829834,1058557105])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1081755019,1085850508,3221711245]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1072821352,3228570020,1057905407]),direction:v3([3207520103,1054921559,3231548982])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1067091124,3226728249,3230382806]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3222604554,1079405586,1080267682]),direction:v3([3220363293,3228412013,3230433196])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3230088076,3199853276,3210063574]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1054810539,1063961476,3226594162]),direction:v3([1074076990,3222680137,3213627713])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1075904883,3216442064,3230398985]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3198927931,1076676665,3224613584]),direction:v3([1075141755,1083076145,1078529040])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1073727462,1088737870,1051380224]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3200727180,3217993609,1081888595]),direction:v3([3208867075,3222632136,3223556416])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3214116005,3229404813,1068596262]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3214762282,3230177020,1061801496]),direction:v3([1077099740,3228994301,1081077473])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1070143631,3238129213,1083257140]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1072458595,1082733206,1076521496]),direction:v3([3228552263,1080251353,1082920267])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3220385067,1090182275,1088504407]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1053029605,3228864505,3229796345]),direction:v3([3214165424,1082329434,3212433236])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3209072877,1049368026,3231843043]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1073861376,1074810951,1084083428]),direction:v3([3227311325,3192155494,1080866303])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3216380857,1074006385,1091179506]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3223701613,3228351827,1061650180]),direction:v3([1061471938,3219472772,3230711855])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3219729784,3232739083,3228541085]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3224676558,1045248096,1082912943]),direction:v3([3231343514,3230617019,3229108237])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3237263361,3230196728,1056754312]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1058798620,3220993055,1078590779]),direction:v3([1029440432,1082821216,1061140434])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1059699799,1075239600,1081731888]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3211835628,3221231176,3230499301]),direction:v3([3231463767,3221258570,3221438329])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3233435765,3229633481,3234800033]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1083771267,1074590006,3228705833]),direction:v3([3227460897,1067502057,1060570916])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1067834316,1079858730,3225707104]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3214134346,3226737172,1079421967]),direction:v3([3228389050,1071917013,3220856728])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3231423087,3217296467,1068693638]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3223974460,1082239990,3230678992]),direction:v3([3213252994,1078515457,3223167735])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3228376829,1088821111,3235844427]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1073968059,1076483307,1075045337]),direction:v3([1064941836,1074756295,3214155180])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1078059518,1084008409,1066641925]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1069764296,1067210378,1083027005]),direction:v3([1076779788,1062974566,3231044538])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1082655032,1074075743,3196213194]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1077146231,1009701369,3230813895]),direction:v3([3197756921,3230977583,1068822492])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1075885495,3230955206,3226084767]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3212692912,3229625926,3231409796]),direction:v3([1074430910,1077602003,3212142841])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1066803363,3213552498,3233420195]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1070872055,3219404031,1063260593]),direction:v3([3220313592,3206783234,3216312652])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3194945552,3222995648,3205103848]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3224989664,3229012763,3225252259]),direction:v3([3199286350,3220320635,1079953260])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3226441578,3233281516,1057314598]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3171152524,1078220078,1082854861]),direction:v3([1071914263,1068158955,1075133900])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1071644034,1082973841,1087745204]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1066357310,3221300366,3220668412]),direction:v3([3228366899,1069675710,1082436696])};let got=b.classify_ray_intersection(ray);
match got{RayBoxIntersection::Hit{enter,exit}=>{eq(enter,1023699412);eq(exit,1058180643);},_=>panic!("classification mismatch")}
let p=b.intersect_segment(ray.origin,v3([3223670548,3204491451,1074632882]));for (a,e)in p.unwrap().to_array().into_iter().zip([1065353216,3220964342,3219544078]){eq(a,e)}}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1044364051,3227115482,3231619650]),direction:v3([1065534298,1057027898,3226275548])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1067104956,3225002508,3238170840]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1077894280,3224164683,1076484557]),direction:v3([3221438643,3228511951,1071122821])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1064333137,3234726925,1082847047]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1048173801,3227055550,1071729028]),direction:v3([1079344169,3212310827,1081687808])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1080367607,3230366213,1085600225]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1077479458,1080305289,3231707225]),direction:v3([1083073030,3188674788,3231389544])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1089136151,1079718267,3239936993]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1047911323,3214916322,1075755783]),direction:v3([1059285094,1078931284,3223115964])};let got=b.classify_ray_intersection(ray);
match got{RayBoxIntersection::Inside{enter,exit}=>{eq(enter,3193517188);eq(exit,1065380753);},_=>panic!("classification mismatch")}
let p=b.intersect_segment(ray.origin,v3([1063313229,1073652678,1022436712]));for (a,e)in p.unwrap().to_array().into_iter().zip([1047911323,3214916322,1075755783]){eq(a,e)}}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1081200607,3228993790,3229437655]),direction:v3([3222216871,1081645006,1054368293])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1069899375,1023652810,3227665043]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1076502104,3213348017,1073175430]),direction:v3([3222061486,3223409687,3231186747])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1055581527,3227859568,3224654004]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3227830475,1071575303,1075387679]),direction:v3([3229551977,3222505054,3218489353])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3237079834,3205511018,1060631656]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1048795816,1073144838,3217802859]),direction:v3([3220893712,3223221759,3220874891])};let got=b.classify_ray_intersection(ray);
match got{RayBoxIntersection::Inside{enter,exit}=>{eq(enter,3169547887);eq(exit,1059329356);},_=>panic!("classification mismatch")}
let p=b.intersect_segment(ray.origin,v3([3218741606,3205238766,3227727483]));for (a,e)in p.unwrap().to_array().into_iter().zip([1048795816,1073144838,3217802859]){eq(a,e)}}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3223130345,3231426438,3230316437]),direction:v3([3218642306,3217061829,1078580059])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3229920725,3234579831,3214358432]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([3214282806,1049007275,1077852537]),direction:v3([3218168612,3222263938,3230156739])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([3224614317,3221097434,3215174682]));assert!(p.is_none());}
{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3([1068195430,1073989930,1075840460]),direction:v3([1071065282,1077625634,3214025546])};let got=b.classify_ray_intersection(ray);
assert!(matches!(got,RayBoxIntersection::NoHit));
let p=b.intersect_segment(ray.origin,v3([1078018964,1084196390,1068361806]));assert!(p.is_none());}
}
#[test]fn original_b_plane_ray(){
{let p=Plane{point:v3([3219588576,1051911252,1074309270]),normal:v3([1008140185,1031969410,1053837503])};let r=Ray{origin:v3([1065340341,3223249316,1074209809]),direction:v3([3221900184,1080462450,3223216425])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([3184482504,3229733175,1082178018]),normal:v3([3201173352,1045986861,1060123374])};let r=Ray{origin:v3([1069570635,3222135313,3226568677]),direction:v3([3230356462,3217146278,3228155088])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([1072523750,3223507028,1075574420]),normal:v3([1053172097,1042398520,3209432026])};let r=Ray{origin:v3([1045802523,1069970896,1073142993]),direction:v3([1081873702,3209408939,3197816537])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([1074106162,1079468793,3193195851]),normal:v3([1060445815,1045962914,3196497395])};let r=Ray{origin:v3([1060600671,3217035244,3223060265]),direction:v3([3219567500,1064786014,1067432123])};let got=p.intersect_ray(r,f(3212836864));assert!(got.is_none());}
{let p=Plane{point:v3([1076864520,3197489130,1068469116]),normal:v3([1061480673,1044587995,1064547587])};let r=Ray{origin:v3([1066518743,3210755596,3228779616]),direction:v3([3228260077,1082407933,3229498330])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([3222940843,3230720238,3196585345]),normal:v3([3167555805,3209246933,3211874847])};let r=Ray{origin:v3([1074322470,3223527198,1049521273]),direction:v3([1082970644,1067639734,3205071418])};let got=p.intersect_ray(r,f(3212836864));assert!(got.is_none());}
{let p=Plane{point:v3([1072841180,1067225898,1062259431]),normal:v3([1061948699,1057573414,3199579568])};let r=Ray{origin:v3([1006971364,3208038535,1077622189]),direction:v3([1079988505,1074711321,3228035605])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1059061120);for (a,e)in p.to_array().into_iter().zip([1074533803,1059990947,1059657360]){eq(a,e)}}
{let p=Plane{point:v3([3220646779,3215007199,1082859737]),normal:v3([1030719918,1045060375,3207895779])};let r=Ray{origin:v3([3222358951,1079211216,3224871531]),direction:v3([1082405044,1074079319,3209833801])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([3229795521,3228459502,1065609077]),normal:v3([3202920287,3168997983,3196050270])};let r=Ray{origin:v3([1070972523,3214538839,1083791453]),direction:v3([3231048065,3220702200,3228513662])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1066725101);for (a,e)in p.to_array().into_iter().zip([3228690894,3227338224,1055056624]){eq(a,e)}}
{let p=Plane{point:v3([3189930646,3227678514,1059640417]),normal:v3([1064639477,1044201946,3170200933])};let r=Ray{origin:v3([3230465121,3229380370,3229168653]),direction:v3([1076042939,1034063903,3225385152])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1070168215);for (a,e)in p.to_array().into_iter().zip([3200901056,3228856342,3238634755]){eq(a,e)}}
{let p=Plane{point:v3([1071729188,1066167254,1077781619]),normal:v3([3208738500,3197087606,1044010480])};let r=Ray{origin:v3([1083189828,3231583658,1083284578]),direction:v3([3222387474,3221765431,1082466248])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1022645406);for (a,e)in p.to_array().into_iter().zip([1083047402,3231716808,1083544788]){eq(a,e)}}
{let p=Plane{point:v3([1073400495,1075846975,1080701926]),normal:v3([1062364980,3211662885,1040689874])};let r=Ray{origin:v3([1039395018,3211471551,1054706913]),direction:v3([1079507344,3228513394,1068199584])};let got=p.intersect_ray(r,f(3212836864));let(t,p)=got.unwrap();eq(t,3192242952);for (a,e)in p.to_array().into_iter().zip([3204995709,3192489116,1043479626]){eq(a,e)}}
{let p=Plane{point:v3([1083541364,3225455384,1082007271]),normal:v3([1062286578,3197167256,3212607992])};let r=Ray{origin:v3([1078113951,3227838113,3231696364]),direction:v3([1076367248,3229477584,3218175003])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([3231021376,3223184323,3219732158]),normal:v3([3211517922,3209133775,3209774073])};let r=Ray{origin:v3([3207616908,1076367352,3224285707]),direction:v3([1077726692,3229776005,1020486907])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1098638014);for (a,e)in p.to_array().into_iter().zip([1110901682,3262529140,3222581795]){eq(a,e)}}
{let p=Plane{point:v3([3181029541,1069467752,1069593516]),normal:v3([3187163445,1055194527,3205564407])};let r=Ray{origin:v3([1059565226,1081159692,1082723761]),direction:v3([1068637363,1071042428,3230435681])};let got=p.intersect_ray(r,f(3212836864));let(t,p)=got.unwrap();eq(t,1045858813);for (a,e)in p.to_array().into_iter().zip([1064456362,1082382426,1079457814]){eq(a,e)}}
{let p=Plane{point:v3([1062031401,1074537804,1075426324]),normal:v3([3196195160,3206775615,1046785705])};let r=Ray{origin:v3([1075854715,1076993351,3204967396]),direction:v3([1075867754,3206513187,1070787234])};let got=p.intersect_ray(r,f(3212836864));let(t,p)=got.unwrap();eq(t,1093996166);for (a,e)in p.to_array().into_iter().zip([1106704601,3230191996,1100016765]){eq(a,e)}}
{let p=Plane{point:v3([3214334247,3217356498,1083761570]),normal:v3([3183139099,1065146147,1058243948])};let r=Ray{origin:v3([3224058777,1058995805,1077666801]),direction:v3([3211629552,1079728787,1063096740])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([1073016152,1065104407,3227720462]),normal:v3([3193483988,1061939318,1053119589])};let r=Ray{origin:v3([3217105831,1084211104,1082719670]),direction:v3([1069290034,3219308285,1051055896])};let got=p.intersect_ray(r,f(1065353216));let(t,p)=got.unwrap();eq(t,1082846173);for (a,e)in p.to_array().into_iter().zip([1083954498,3224154166,1085668629]){eq(a,e)}}
{let p=Plane{point:v3([1083224368,3231320662,1061613191]),normal:v3([1037315378,3206012986,1059674470])};let r=Ray{origin:v3([1080861814,3202732799,1082428518]),direction:v3([3222546963,3222789608,1071693585])};let got=p.intersect_ray(r,f(3212836864));let(t,p)=got.unwrap();eq(t,1044729862);for (a,e)in p.to_array().into_iter().zip([1078990794,3211261628,1083138043]){eq(a,e)}}
{let p=Plane{point:v3([1080323865,3231321672,1082277284]),normal:v3([3204463413,1059654460,3206232270])};let r=Ray{origin:v3([1079447123,3229357629,3231475972]),direction:v3([3215835313,1064784123,3229892710])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([3229153714,1076724948,3230661141]),normal:v3([1026881021,3177529928,1058111135])};let r=Ray{origin:v3([3203112673,1045760433,3223174734]),direction:v3([1080594657,3197346272,3223274153])};let got=p.intersect_ray(r,f(3212836864));let(t,p)=got.unwrap();eq(t,1066779037);for (a,e)in p.to_array().into_iter().zip([1081255058,3187960267,3232500070]){eq(a,e)}}
{let p=Plane{point:v3([1082978776,1044844318,1061613596]),normal:v3([1061168570,3208136123,1007794050])};let r=Ray{origin:v3([3221240675,3228739509,3231299023]),direction:v3([1064078636,1074473761,3210163970])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([3191503216,1063986082,1081010130]),normal:v3([1055821133,1055406611,1056823811])};let r=Ray{origin:v3([1079641021,3221674017,1068107786]),direction:v3([3172978143,3229152110,3226022549])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([3221987385,3206203519,3216784132]),normal:v3([1061377562,1054782525,1062272990])};let r=Ray{origin:v3([1077983822,1083399236,3227809977]),direction:v3([1080174031,3231004387,3230870412])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1069233664);for (a,e)in p.to_array().into_iter().zip([1090707260,3222127038,3240410633]){eq(a,e)}}
{let p=Plane{point:v3([1083042155,3189736506,3222774340]),normal:v3([1061192332,1058026665,3210801481])};let r=Ray{origin:v3([3210544694,1077391993,1084010051]),direction:v3([3230761891,1068767925,3209357555])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([1078627187,1071544928,3222617703]),normal:v3([1059650806,1060094486,1063849662])};let r=Ray{origin:v3([3222898932,1068312382,1083229132]),direction:v3([1078187483,3215107869,1045121272])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([1074846283,3223328173,3195468238]),normal:v3([3200945988,1044726002,3173574799])};let r=Ray{origin:v3([1073389042,3224391353,3230332102]),direction:v3([1054102299,3228067696,1067435501])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1049028853);for (a,e)in p.to_array().into_iter().zip([1074023748,3228404618,3229642342]){eq(a,e)}}
{let p=Plane{point:v3([1075773661,3219190069,3216540350]),normal:v3([1043141050,1048822549,3192771092])};let r=Ray{origin:v3([3222119477,1081158590,1081374908]),direction:v3([1074751605,1075141652,3230397125])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1047307299);for (a,e)in p.to_array().into_iter().zip([3218669643,1082775540,1077135864]){eq(a,e)}}
{let p=Plane{point:v3([1084124542,3226991843,1074324902]),normal:v3([1064138176,3174184705,1055819727])};let r=Ray{origin:v3([1072596625,3215225833,3225986866]),direction:v3([1079079842,1066980634,3224314002])};let got=p.intersect_ray(r,f(3212836864));let(t,p)=got.unwrap();eq(t,1078636057);for (a,e)in p.to_array().into_iter().zip([1094952065,1075824184,3241988248]){eq(a,e)}}
{let p=Plane{point:v3([1076090412,1075765817,3229696643]),normal:v3([3208968969,3208831568,1059428208])};let r=Ray{origin:v3([1076349224,3205119664,3221443204]),direction:v3([1082623771,3222330367,1076325500])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([3223894778,1066751901,3201788610]),normal:v3([998698453,1057945847,1053241767])};let r=Ray{origin:v3([1084186799,3222857243,3221203781]),direction:v3([1068646912,1082241818,1064999309])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([1053254244,3215350543,1064276829]),normal:v3([3194943283,1055955584,3196786367])};let r=Ray{origin:v3([3208914671,1074411380,1074188129]),direction:v3([3208898522,3231543927,1059744722])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([1048654642,3221525994,3225650461]),normal:v3([1038823501,1026444105,1052005130])};let r=Ray{origin:v3([1078694399,3219776150,1080899896]),direction:v3([3223172666,1064206399,3220959248])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1077570823);for (a,e)in p.to_array().into_iter().zip([3229602885,1063450072,3221337656]){eq(a,e)}}
{let p=Plane{point:v3([3209483741,1065363804,1077050653]),normal:v3([3203882962,3203268819,1063102437])};let r=Ray{origin:v3([1051730565,1075435786,3199519102]),direction:v3([3210093400,3214785715,3226686906])};let got=p.intersect_ray(r,f(3212836864));assert!(got.is_none());}
{let p=Plane{point:v3([3230860130,1084170749,3221549352]),normal:v3([3198632626,1062430182,3161347896])};let r=Ray{origin:v3([3230665333,3229556707,1083396850]),direction:v3([3197414171,1080743513,1061867198])};let got=p.intersect_ray(r,f(3212836864));let(t,p)=got.unwrap();eq(t,1075491931);for (a,e)in p.to_array().into_iter().zip([3232137312,1083983062,1087412882]){eq(a,e)}}
{let p=Plane{point:v3([1032652610,1064648584,3231066974]),normal:v3([1045815704,3161218644,3208095258])};let r=Ray{origin:v3([1081969174,1079726536,1044613373]),direction:v3([3230738919,1084108158,3225471269])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1075364269);for (a,e)in p.to_array().into_iter().zip([3235624389,1098095054,3235903048]){eq(a,e)}}
{let p=Plane{point:v3([1066765452,3228361872,3228368300]),normal:v3([1060441868,1047448562,3202830345])};let r=Ray{origin:v3([1034245637,3227410365,3223071713]),direction:v3([1071318486,3221514486,1081701954])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([1079238692,3230453981,3231661168]),normal:v3([1041129637,1053285047,1064230480])};let r=Ray{origin:v3([1069526906,1053073999,1072027982]),direction:v3([3199435952,3217821902,1080150450])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([3211130616,3219206730,1083030959]),normal:v3([3202966518,3198310582,3201353034])};let r=Ray{origin:v3([1074335950,3231242050,1080973065]),direction:v3([3226901925,1079312065,1070573272])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([3230635447,1076149042,1083243743]),normal:v3([3210712176,1026855177,1062435703])};let r=Ray{origin:v3([1073262416,1077960167,3225521850]),direction:v3([3219965400,1083736325,3231182674])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([1078904614,3228212932,1052556552]),normal:v3([1061899466,3175399483,1042629436])};let r=Ray{origin:v3([3221445835,3230659117,1080559914]),direction:v3([3214874015,1063587767,1071765120])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([3231675798,1082537520,1075573305]),normal:v3([3203233896,3198779131,3194321346])};let r=Ray{origin:v3([3230270710,1067888799,3231651293]),direction:v3([3229940819,1058014427,3228844300])};let got=p.intersect_ray(r,f(3212836864));let(t,p)=got.unwrap();eq(t,3210964314);for (a,e)in p.to_array().into_iter().zip([3206480720,1062039403,3217709934]){eq(a,e)}}
{let p=Plane{point:v3([1083738360,1019553904,1053493123]),normal:v3([1052167630,3208552273,1062996188])};let r=Ray{origin:v3([3215408093,3225401398,3230316643]),direction:v3([1068165242,3215985074,1074941532])};let got=p.intersect_ray(r,f(3212836864));let(t,p)=got.unwrap();eq(t,1066614336);for (a,e)in p.to_array().into_iter().zip([1047196136,3230825547,3218753346]){eq(a,e)}}
{let p=Plane{point:v3([1044344025,3231594395,1066217409]),normal:v3([3205143868,3204752574,3197543010])};let r=Ray{origin:v3([3216510155,3222375390,1078972551]),direction:v3([3227199590,3220810084,3229102701])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([3230728592,1080817547,1083555902]),normal:v3([3209125166,3203661042,3179530096])};let r=Ray{origin:v3([1071594644,3225733140,3193538438]),direction:v3([3224797973,3230625149,1080694982])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1051179490);for (a,e)in p.to_array().into_iter().zip([1062162711,3230752851,1065115238]){eq(a,e)}}
{let p=Plane{point:v3([1081628108,1059261901,1055423548]),normal:v3([1040691008,1033782205,1037100018])};let r=Ray{origin:v3([1075977142,1083185313,1065675804]),direction:v3([1061475742,1066069304,1073757036])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([1071416655,3231358805,3231216042]),normal:v3([3198076438,1054771414,1049724507])};let r=Ray{origin:v3([1073620941,3231483437,1075796868]),direction:v3([1066597918,3222706675,1041096052])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1069204126);for (a,e)in p.to_array().into_iter().zip([1080709192,3238343234,1076644700]){eq(a,e)}}
{let p=Plane{point:v3([3228231795,1037978243,1040277762]),normal:v3([1038006094,1058025027,1052106422])};let r=Ray{origin:v3([1082085988,1070593846,3228780677]),direction:v3([3228676220,1073250034,1078473437])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([3218385989,1082058126,1077811909]),normal:v3([3199083882,1040230475,1061270403])};let r=Ray{origin:v3([3231336786,3224194395,1080967426]),direction:v3([3211931044,1082599855,3212683221])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([1053996505,3176832404,1021809618]),normal:v3([3203325278,3193470978,3202200837])};let r=Ray{origin:v3([1079925524,3206375561,1082128736]),direction:v3([3225915222,3208676990,3228881455])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([1068084109,3207190823,3216343024]),normal:v3([1054620540,3207491900,1060835057])};let r=Ray{origin:v3([3217257014,1076340198,3224371343]),direction:v3([3212335251,1079530510,1062385060])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([1078162763,1078168332,1078853245]),normal:v3([1059890837,1053812277,1053681096])};let r=Ray{origin:v3([3221487326,1074943654,3226415102]),direction:v3([1084139439,1076823479,1079472589])};let got=p.intersect_ray(r,f(3212836864));let(t,p)=got.unwrap();eq(t,1066149207);for (a,e)in p.to_array().into_iter().zip([1079471234,1084816378,1055234720]){eq(a,e)}}
{let p=Plane{point:v3([3229710559,1058891598,1058330890]),normal:v3([1057476448,3204880722,3197455090])};let r=Ray{origin:v3([1065325995,1070423303,3219241041]),direction:v3([1066171975,3227775608,3216449992])};let got=p.intersect_ray(r,f(3212836864));let(t,p)=got.unwrap();eq(t,3212646600);for (a,e)in p.to_array().into_iter().zip([3182541064,1084491162,3199379664]){eq(a,e)}}
{let p=Plane{point:v3([3222535208,3228019705,3222348373]),normal:v3([3204943064,991277095,1032843539])};let r=Ray{origin:v3([1075305376,3219281761,1078896979]),direction:v3([1080774266,1039399550,3231062244])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([3186482811,3219917178,1081117862]),normal:v3([3177448479,3205950291,1058310471])};let r=Ray{origin:v3([1082726339,1083588220,1075204622]),direction:v3([3215730842,3221565221,3189446451])};let got=p.intersect_ray(r,f(1065353216));let(t,p)=got.unwrap();eq(t,1082238879);for (a,e)in p.to_array().into_iter().zip([3214223980,3228508856,1071519711]){eq(a,e)}}
{let p=Plane{point:v3([1047146879,3223785239,1074140617]),normal:v3([1034187111,1065192437,3200509561])};let r=Ray{origin:v3([1060462313,3228236721,1067160670]),direction:v3([1081452321,1083457298,1083074220])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1046004268);for (a,e)in p.to_array().into_iter().zip([1069723424,3224123613,1074402177]){eq(a,e)}}
{let p=Plane{point:v3([3221523684,3219665527,1055122706]),normal:v3([3210973063,1051889434,3212738322])};let r=Ray{origin:v3([3225750489,1076223829,3198015958]),direction:v3([3224457946,3215806738,3214762993])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([3225715075,1058955417,1081756302]),normal:v3([3209333060,3206400422,3199511951])};let r=Ray{origin:v3([1083222501,1072102897,1081663607]),direction:v3([1082587471,3221441911,3226632343])};let got=p.intersect_ray(r,f(3212836864));assert!(got.is_none());}
{let p=Plane{point:v3([1075132809,1083755123,3227348809]),normal:v3([3132723596,3199080201,1063197348])};let r=Ray{origin:v3([1068407398,3227835531,1062214809]),direction:v3([1069509178,1083505852,3219018960])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1074238044);for (a,e)in p.to_array().into_iter().zip([1083245838,1086925710,3224859782]){eq(a,e)}}
{let p=Plane{point:v3([3176901103,3226109370,3215298235]),normal:v3([3204593104,3162802844,3204503799])};let r=Ray{origin:v3([1074153136,3212707194,3214104356]),direction:v3([1074050696,3229493259,3230207425])};let got=p.intersect_ray(r,f(0));let(t,p)=got.unwrap();eq(t,1065612595);for (a,e)in p.to_array().into_iter().zip([1082624988,3231892124,3232899175]){eq(a,e)}}
{let p=Plane{point:v3([1056975539,3223626157,1080491052]),normal:v3([3212671316,1015709590,1061074390])};let r=Ray{origin:v3([1083319572,1082237244,3224819296]),direction:v3([3229417602,1058140250,3225560914])};let got=p.intersect_ray(r,f(1065353216));let(t,p)=got.unwrap();eq(t,1084746906);for (a,e)in p.to_array().into_iter().zip([3246484452,1088510952,3247846884]){eq(a,e)}}
{let p=Plane{point:v3([1078914859,1077493901,3205673648]),normal:v3([3203931298,1051556552,3199377975])};let r=Ray{origin:v3([1077289764,1030338590,3215474840]),direction:v3([1071889869,3225259893,1070188554])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([3223182938,1079648679,3226406642]),normal:v3([3206555525,3207084098,1060330510])};let r=Ray{origin:v3([3229702882,3206902012,1040513865]),direction:v3([1077006720,3225198390,1069785844])};let got=p.intersect_ray(r,f(0));assert!(got.is_none());}
{let p=Plane{point:v3([1047818584,3224622269,3227623934]),normal:v3([1045516790,1057040535,3205034422])};let r=Ray{origin:v3([3228569933,1049858807,3226984478]),direction:v3([1079923110,3231299542,1075685909])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([1071960279,1080100327,3212251703]),normal:v3([3198285243,1057226143,1051965858])};let r=Ray{origin:v3([3221180567,1059754735,1082396425]),direction:v3([1083664792,1084105723,3219676696])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
{let p=Plane{point:v3([1069463939,1080812586,1082251154]),normal:v3([3201254099,1052772726,1044220151])};let r=Ray{origin:v3([1073265949,3229925089,1083051763]),direction:v3([1039663436,3231481100,1080250841])};let got=p.intersect_ray(r,f(3212836864));assert!(got.is_none());}
{let p=Plane{point:v3([1074800865,1081365814,3224175213]),normal:v3([1047660495,1054661796,1059765113])};let r=Ray{origin:v3([1057801828,3221172526,3229522712]),direction:v3([3224203228,1067095777,3224274743])};let got=p.intersect_ray(r,f(3212836864));assert!(got.is_none());}
{let p=Plane{point:v3([1074720391,3227987560,1055055915]),normal:v3([1064895073,3207288212,1032670736])};let r=Ray{origin:v3([3201685171,1075383027,1076967992]),direction:v3([1082346794,1076573711,1079108519])};let got=p.intersect_ray(r,f(1065353216));let(t,p)=got.unwrap();eq(t,1076476066);for (a,e)in p.to_array().into_iter().zip([1093102225,1092076776,1094153528]){eq(a,e)}}
{let p=Plane{point:v3([1066144745,1075701889,1073987813]),normal:v3([3202639979,1044834413,3201062244])};let r=Ray{origin:v3([3230817027,3229887889,1057202034]),direction:v3([3219998729,1079993956,3212868638])};let got=p.intersect_ray(r,f(3212836864));let(t,p)=got.unwrap();eq(t,3212464381);for (a,e)in p.to_array().into_iter().zip([3224417361,3237045734,1069511060]){eq(a,e)}}
{let p=Plane{point:v3([3228542000,1071634760,3228474975]),normal:v3([1053563802,1060285102,3166834880])};let r=Ray{origin:v3([1061499600,3229628773,1065445607]),direction:v3([3227214181,1049995398,3215970613])};let got=p.intersect_ray(r,f(1065353216));assert!(got.is_none());}
}
#[test]fn original_b_line_relation(){
{let a=line_relation::LineInterval::line_segment(v2([0,0]),v2([1065353216,1065353216]));let b=line_relation::LineInterval::line_segment(v2([1065353216,1065353216]),v2([1073741824,1073741824]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Collinear);
}
{let a=line_relation::LineInterval::line_segment(v2([0,0]),v2([1065353216,1065353216]));let b=line_relation::LineInterval::line_segment(v2([0,1065353216]),v2([1065353216,1073741824]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Parallel);
}
{let a=line_relation::LineInterval::line_segment(v2([0,0]),v2([1065353216,1065353216]));let b=line_relation::LineInterval::line_segment(v2([0,1065353216]),v2([1065353216,0]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1056964608,1056964608]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([3210294673,3231590314]),v2([3230442851,3222515619]));let b=line_relation::LineInterval::line_segment(v2([3226674664,1049858971]),v2([3227468270,1076567436]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1078393748,3227862367]),v2([3214594126,3228149354]));let b=line_relation::LineInterval::line_segment(v2([3231101328,3223667065]),v2([1039646585,1083756306]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3206925006,1071795240]),v2([1070429728,1066187436]));let b=line_relation::LineInterval::line_segment(v2([1028865767,1082835897]),v2([1076905281,3226031732]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1066818135,1067259154]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([3225945004,3214353166]),v2([1077370032,1040331925]));let b=line_relation::LineInterval::line_segment(v2([1068613400,1083225966]),v2([3222655625,3222958957]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([3216911102,3209863626]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([3224836763,1077382638]),v2([3209747919,1069828024]));let b=line_relation::LineInterval::line_segment(v2([3227324287,1083027982]),v2([3218008608,3225269440]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1063705427,1067334983]),v2([1079423580,3228243728]));let b=line_relation::LineInterval::line_segment(v2([1057281413,3217636131]),v2([3231636199,1078648312]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1068734151,1082471138]),v2([1082847443,1064559681]));let b=line_relation::LineInterval::line_segment(v2([1041101405,3206392038]),v2([1074163908,1083167172]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1072103980,1080971616]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([3215225635,1062117377]),v2([3222981200,1078033035]));let b=line_relation::LineInterval::line_segment(v2([1084072106,1045546941]),v2([1068507135,1075603478]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3227467692,3224582157]),v2([3231363024,1077789415]));let b=line_relation::LineInterval::line_segment(v2([1078302842,3228413731]),v2([1080160470,1063217224]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3229721767,3230627496]),v2([1071191533,1074476892]));let b=line_relation::LineInterval::line_segment(v2([1074362302,3228219819]),v2([1068505684,1082437116]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1070247556,1073930076]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([1079013024,3183033410]),v2([1051282087,3204788021]));let b=line_relation::LineInterval::line_segment(v2([1053796674,1082235447]),v2([3210845508,3209295227]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1077294931,1058042203]),v2([1072597780,3231539335]));let b=line_relation::LineInterval::line_segment(v2([1082318180,1058943458]),v2([3219281072,1073818871]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3165643274,1041802534]),v2([1054377321,1081947350]));let b=line_relation::LineInterval::line_segment(v2([3224616711,3213950441]),v2([3222568417,3230199030]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1069931757,3209533889]),v2([3228819160,3224040407]));let b=line_relation::LineInterval::line_segment(v2([3207263041,3224234087]),v2([1050647353,1081151201]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([3203966012,3217128396]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([1079515804,1084025189]),v2([1081903605,1083166099]));let b=line_relation::LineInterval::line_segment(v2([1042845048,3231093914]),v2([1078148070,3212163670]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3225917788,3221783732]),v2([3227045900,3206807134]));let b=line_relation::LineInterval::line_segment(v2([1077389142,1082720014]),v2([3216708670,3224027058]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1081351220,3207435898]),v2([1064415189,3228269256]));let b=line_relation::LineInterval::line_segment(v2([3206890197,1060109117]),v2([1072601388,3222687976]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3230925512,3227656828]),v2([1071669762,3231057008]));let b=line_relation::LineInterval::line_segment(v2([3219030341,3231086997]),v2([3230277985,3217431737]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([3222728330,3229378199]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([3225840316,3216544822]),v2([3227970114,3230925099]));let b=line_relation::LineInterval::line_segment(v2([3228706963,3227944826]),v2([1066408637,3202843922]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([3227123488,3226928911]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([3230250677,3228598321]),v2([1062667341,3225434602]));let b=line_relation::LineInterval::line_segment(v2([1052669633,1069630446]),v2([1055078376,1061743175]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3196496361,1080578554]),v2([3221921156,3230184478]));let b=line_relation::LineInterval::line_segment(v2([1082790950,1083295182]),v2([1078248959,3227937245]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1067252296,1059217230]),v2([3208005151,1068827565]));let b=line_relation::LineInterval::line_segment(v2([3230390850,1075275020]),v2([3205065779,1082891338]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1075337272,3229397291]),v2([3188375897,3209500554]));let b=line_relation::LineInterval::line_segment(v2([1077666877,1083542083]),v2([3218629818,3222517472]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1073129771,3223296821]),v2([3225845680,1077435986]));let b=line_relation::LineInterval::line_segment(v2([3212950163,1078845470]),v2([1059853701,3196787868]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1068218093,3224687782]),v2([3213890715,1077486192]));let b=line_relation::LineInterval::line_segment(v2([1025548828,3226290808]),v2([3228898082,1073061001]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1073912099,3202694763]),v2([3223991985,1083502532]));let b=line_relation::LineInterval::line_segment(v2([1065979954,3206360540]),v2([3230362691,1083312760]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3220042506,3218367015]),v2([1074422006,1082301676]));let b=line_relation::LineInterval::line_segment(v2([3201222841,3205885996]),v2([1042554571,3189724323]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3218396686,3228202625]),v2([1079107852,3226175723]));let b=line_relation::LineInterval::line_segment(v2([3210129669,1057933101]),v2([3231626460,1083443482]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1060506686,3226028284]),v2([1073513581,1083226421]));let b=line_relation::LineInterval::line_segment(v2([1051278944,1067175304]),v2([1079179671,1056914714]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1068597895,1064703336]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([1051258686,3230192109]),v2([1074128008,1074025035]));let b=line_relation::LineInterval::line_segment(v2([1074155346,3228684080]),v2([1083057611,3222702963]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1063058906,1075267685]),v2([1076631342,1059304518]));let b=line_relation::LineInterval::line_segment(v2([1055044543,1078931926]),v2([1083072465,3209592680]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3228789764,3206379965]),v2([1073737487,1074602012]));let b=line_relation::LineInterval::line_segment(v2([1083224021,3225202404]),v2([1056907888,1082241295]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1070856580,1073901954]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([3231622775,3196606518]),v2([1058219372,3227738173]));let b=line_relation::LineInterval::line_segment(v2([1077544590,3205860105]),v2([3226686310,3231539012]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([3203208224,3225152434]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([1072172026,3197808504]),v2([1075216663,1083623067]));let b=line_relation::LineInterval::line_segment(v2([1067163981,1071259900]),v2([1075601071,3228405942]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1073059329,3229628356]),v2([3219610869,1075530362]));let b=line_relation::LineInterval::line_segment(v2([1083147021,3228844409]),v2([3205779199,1073443800]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1082373471,3224080602]),v2([1084081050,1063700028]));let b=line_relation::LineInterval::line_segment(v2([3229598961,1077473396]),v2([3200211473,3230098871]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1079356568,3223636663]),v2([3227887994,3231080658]));let b=line_relation::LineInterval::line_segment(v2([1064728089,3218997697]),v2([3210373123,3226452765]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1023864959,3206763947]),v2([1074735646,1074582345]));let b=line_relation::LineInterval::line_segment(v2([3196183807,3228091706]),v2([1064331555,1082660079]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1047907131,3200130696]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([3222960104,1067130704]),v2([1082503869,1078307195]));let b=line_relation::LineInterval::line_segment(v2([3218595235,3186900126]),v2([1068110453,1060837812]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3223553868,1078004136]),v2([3214837208,3230259976]));let b=line_relation::LineInterval::line_segment(v2([1080010385,1079535077]),v2([3216754066,3206039431]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1072152877,1076793880]),v2([1070970966,1083534700]));let b=line_relation::LineInterval::line_segment(v2([3226434899,3213196368]),v2([3199869585,1080657381]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1081112362,1082149086]),v2([3211194760,3230614121]));let b=line_relation::LineInterval::line_segment(v2([1077308147,1084042057]),v2([3213056282,3213412152]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3215064871,3225216492]),v2([3224210451,3223461814]));let b=line_relation::LineInterval::line_segment(v2([1084102313,1075782227]),v2([1050651709,3219399899]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3230094422,3206204163]),v2([3218515425,3230192355]));let b=line_relation::LineInterval::line_segment(v2([3229910861,1080884146]),v2([3214405499,1076276518]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3231366494,1075548984]),v2([3227509913,1041515392]));let b=line_relation::LineInterval::line_segment(v2([3211647686,3226095097]),v2([3228574074,1083322714]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3231169884,3222589341]),v2([3185107748,1062773905]));let b=line_relation::LineInterval::line_segment(v2([1070044946,1082550797]),v2([1035190700,1065813460]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1076024634,1078065211]),v2([3231297771,1081025045]));let b=line_relation::LineInterval::line_segment(v2([1073842408,1083279304]),v2([3213312833,3225445892]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1069113203,1078506758]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([1082484698,1077390780]),v2([3222557924,1057658248]));let b=line_relation::LineInterval::line_segment(v2([1079803832,1077490797]),v2([3216640748,1079644429]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3225818145,1061052274]),v2([1061340460,1069398101]));let b=line_relation::LineInterval::line_segment(v2([1066422498,1081803923]),v2([1077694561,3221310532]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1076439100,3212310332]),v2([3215256393,3227165134]));let b=line_relation::LineInterval::line_segment(v2([3221327146,1065183685]),v2([3230081264,3221753221]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3219712386,1074937172]),v2([1084166878,3195110365]));let b=line_relation::LineInterval::line_segment(v2([3226761290,3193481935]),v2([1074274927,1073922505]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1058125532,1068693646]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([1084031951,3214022946]),v2([3201948919,3222296474]));let b=line_relation::LineInterval::line_segment(v2([1050636700,3220348189]),v2([1081127691,3231274710]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1057096280,3221478633]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([3209696232,3231560182]),v2([3220107278,1068601129]));let b=line_relation::LineInterval::line_segment(v2([3230595588,3222941288]),v2([1051742720,3229507847]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([3213234979,3227608676]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([1065927317,1075288484]),v2([1077515004,3230015631]));let b=line_relation::LineInterval::line_segment(v2([1080307413,3228998358]),v2([3224363022,3231087599]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1077234375,3229411812]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([3211920084,3223156239]),v2([1056538256,1079440412]));let b=line_relation::LineInterval::line_segment(v2([3230524209,3196875789]),v2([1074416632,1077266173]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1039875056,1072715666]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([1076308326,3223901575]),v2([3231073407,1039609946]));let b=line_relation::LineInterval::line_segment(v2([1082391260,3216253145]),v2([1066017053,1076288832]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1083729406,3229286053]),v2([3196389920,1063983399]));let b=line_relation::LineInterval::line_segment(v2([1015035511,3223565746]),v2([1074119186,1067224634]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1066761794,3203026360]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([1084021442,3227701988]),v2([1084028766,1077089120]));let b=line_relation::LineInterval::line_segment(v2([3201416666,1075022547]),v2([1082260314,3230908486]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3222003719,1065803954]),v2([1083374762,3231476180]));let b=line_relation::LineInterval::line_segment(v2([1070571954,1071698456]),v2([3210087164,1083933776]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1082247833,3215044958]),v2([3219004880,3226255501]));let b=line_relation::LineInterval::line_segment(v2([1066460128,3224608887]),v2([1072218483,3222350410]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1080343201,1083494350]),v2([3231181434,1083517154]));let b=line_relation::LineInterval::line_segment(v2([1069444065,1058607365]),v2([3214621598,3229171954]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1065047688,1079417642]),v2([1046932280,1074001849]));let b=line_relation::LineInterval::line_segment(v2([3192837576,3212721917]),v2([3208486708,3223041043]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3207756796,1078505826]),v2([1080042668,3205896815]));let b=line_relation::LineInterval::line_segment(v2([1075039725,1063553843]),v2([3215954055,3220220196]));let got=a.relate(b);
match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip([1073960585,1060302200]){eq(a,e)}},_=>panic!("intersection mismatch")}
}
{let a=line_relation::LineInterval::line_segment(v2([3213730505,3227042436]),v2([3224891044,1082171001]));let b=line_relation::LineInterval::line_segment(v2([1071505583,1080118426]),v2([1069810445,3220843988]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1079372037,1083063255]),v2([3227025651,1083901414]));let b=line_relation::LineInterval::line_segment(v2([1063520660,1078315171]),v2([3211037526,1079547708]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([3221626157,1076028074]),v2([1077842727,1084052393]));let b=line_relation::LineInterval::line_segment(v2([3229531939,1040339012]),v2([1077919553,1077844335]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
{let a=line_relation::LineInterval::line_segment(v2([1079614995,3224641613]),v2([3215345175,3231625508]));let b=line_relation::LineInterval::line_segment(v2([1070261942,1066425381]),v2([3230324437,3228409074]));let got=a.relate(b);
assert_eq!(got,line_relation::LineRelation::Disjoint);
}
}
#[test]fn original_b_barycentric(){
{let p=barycentric::barycentric(v3([3230755568,3230612485,3200967038]),v3([3228317851,3209454401,1083773370]),v3([3227296956,1066523784,1081868210]),v3([3229570034,3190635179,3208305656]));for(a,e)in p.to_array().into_iter().zip([1073813644,3222524700,1067808031]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3201021652,1051030047,1068819973]),v3([1081016846,1077965480,3194831045]),v3([1079632878,1084138522,1061367284]),v3([3225583121,3209430180,1082290097]));for(a,e)in p.to_array().into_iter().zip([1061685659,3199458555,1058137315]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1078885819,3223743478,1082262464]),v3([1082382541,3175700927,1076951602]),v3([1068974159,3223844510,1074181811]),v3([3223418237,3228723892,3212309974]));for(a,e)in p.to_array().into_iter().zip([3198382560,1073764651,3207572412]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3229216241,3194854019,1083873981]),v3([3230028016,1076271317,3222642436]),v3([1049551630,1059127952,1082600062]),v3([1062113810,3231684616,3230454491]));for(a,e)in p.to_array().into_iter().zip([1049755734,1062925118,3188706725]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3208573307,3224971062,1041310013]),v3([1074671691,3229426105,3209027602]),v3([1082389894,3224629602,1034631164]),v3([3198875895,3205367195,1078674998]));for(a,e)in p.to_array().into_iter().zip([1068345897,3210448814,1056979804]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3201882983,1077564038,1047458225]),v3([3225930518,1068875160,3227736311]),v3([3229927437,3191503675,3217309422]),v3([1059027022,1065760684,3221323674]));for(a,e)in p.to_array().into_iter().zip([3196920160,1051679796,1064231574]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1083559686,3201665033,3215433772]),v3([1071642841,1077931008,3222830295]),v3([1083281499,1083117193,3223430893]),v3([1083967713,3223357118,1081002300]));for(a,e)in p.to_array().into_iter().zip([1047985156,1050660254,1055175776]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1081096923,3221553706,3229120103]),v3([1074747632,1082496966,1082048817]),v3([1074956820,3231224595,3208708261]),v3([3227600286,1053427236,3226673045]));for(a,e)in p.to_array().into_iter().zip([1021245536,1061784511,1044247032]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1070122510,3212996803,1080101760]),v3([1058389807,3227007326,3222628074]),v3([1064588150,1079904912,1084027689]),v3([1067424619,3209979668,1072636800]));for(a,e)in p.to_array().into_iter().zip([3207489000,3202077176,1074205625]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3217668822,3226555313,3230196594]),v3([3221778896,1083433364,3225184670]),v3([1074332341,3216572291,1074144378]),v3([3218853732,1070051838,3230955115]));for(a,e)in p.to_array().into_iter().zip([3214173749,1049010856,1072872843]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1075792859,1083877858,3214990662]),v3([1071431075,3211697033,3215022291]),v3([1073732137,1083224447,3230710744]),v3([1068631771,3229960775,3222753436]));for(a,e)in p.to_array().into_iter().zip([1071765130,1053000482,3214063443]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3209766511,1081080648,3210445487]),v3([3221335500,1083005451,3229887949]),v3([1074637538,3217377105,1080332671]),v3([1065991960,1048163540,3227140427]));for(a,e)in p.to_array().into_iter().zip([1064431188,1055657929,3201297519]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3225540279,3229792433,1063808416]),v3([3230107795,1069230190,1069374359]),v3([3207192112,1081140859,1081229528]),v3([3231262042,1078719618,1056744377]));for(a,e)in p.to_array().into_iter().zip([1081073991,3202596566,3222497644]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1069248084,3216485351,3226046283]),v3([3197955307,3212660939,3228802361]),v3([3219509687,3215848466,3228495189]),v3([3214301729,3226653957,3223349034]));for(a,e)in p.to_array().into_iter().zip([1073757668,3216809337,1055950525]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1079509394,1081042002,3221982093]),v3([1074773254,3228843450,3224707665]),v3([3221933070,1035833929,1074196192]),v3([3191859372,3210033039,1076945577]));for(a,e)in p.to_array().into_iter().zip([1057511606,1056284658,3158977477]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1068593099,1061641100,1077157431]),v3([1068500459,3224400258,3215901744]),v3([3223697508,1041401724,3218137844]),v3([3222242218,1043460674,3226837390]));for(a,e)in p.to_array().into_iter().zip([1060334876,1075972747,3222201810]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3228170974,1067290799,1050748919]),v3([1081935080,3191090081,1065919538]),v3([3221949475,3224302594,3231051235]),v3([1039455372,3221156154,3213066549]));for(a,e)in p.to_array().into_iter().zip([3209073554,3206949046,1075523346]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1058800464,1083088493,1082349330]),v3([1078034178,1071896698,3230120480]),v3([3223967805,3226563957,1082967906]),v3([1080917185,1084168550,1083139071]));for(a,e)in p.to_array().into_iter().zip([985135104,1046626939,1061622629]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1075973555,1075386152,3213635326]),v3([1062827720,3213765560,3229131395]),v3([3231143377,3222595805,1075170393]),v3([3223996779,3204772619,1084085168]));for(a,e)in p.to_array().into_iter().zip([1065150290,3214353434,1066971249]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3230261887,3219406600,3227055709]),v3([3229738628,3189826704,3230840959]),v3([1076902437,3225840104,3191403116]),v3([1078384563,3222390418,1078133414]));for(a,e)in p.to_array().into_iter().zip([1063787992,3148772429,1036646530]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1083332955,3193204235,1078701834]),v3([3226649569,1077120402,1069789157]),v3([1076242331,3227979471,3225358390]),v3([1061192060,3216991404,3223596284]));for(a,e)in p.to_array().into_iter().zip([1071821984,1082671055,3231771895]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1079166890,3206867639,3214819919]),v3([1082554623,1075751827,3161177387]),v3([3212627564,3231186885,3213693191]),v3([1084144473,1083937540,1083981691]));for(a,e)in p.to_array().into_iter().zip([1064234938,1049730045,3193894625]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1068131177,1074930023,1078909293]),v3([1079803147,1053001571,1075386361]),v3([1066921490,1065252214,3225019687]),v3([3199754143,1079710654,3224973215]));for(a,e)in p.to_array().into_iter().zip([1066229728,3213833371,1065473212]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1072855508,1081883479,1058460191]),v3([3204398672,1055658425,3222217964]),v3([1083181049,1079631120,3225120681]),v3([1083379924,3225473219,3231313981]));for(a,e)in p.to_array().into_iter().zip([1059418392,1062526769,3203702929]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1076785584,1048603243,3189143856]),v3([3209866576,1076896657,3203701452]),v3([1066636580,1078840982,1077596651]),v3([1070533998,3207655909,1056405453]));for(a,e)in p.to_array().into_iter().zip([1016364192,1032237866,1063945654]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3227120078,3215509053,1078451505]),v3([3226245452,1061185591,3229013898]),v3([1060714287,1078360103,1078609171]),v3([1067677975,1079164746,1083637357]));for(a,e)in p.to_array().into_iter().zip([1077652240,3245473122,1095963294]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1073687570,1072501534,3224822978]),v3([1076105740,1062929896,1076152306]),v3([3230737813,1056177233,1075448436]),v3([3215338885,3188807491,3224246152]));for(a,e)in p.to_array().into_iter().zip([1056337968,3203454020,1065169418]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3214631571,3230794590,3229966994]),v3([1084208144,3198606498,3212286766]),v3([1083763398,1078619278,1082819543]),v3([1075607347,1072467104,1082553558]));for(a,e)in p.to_array().into_iter().zip([1069166336,3225411588,1076021380]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3224812360,3229351884,1067878113]),v3([1070937032,3205270011,3228479440]),v3([3229303158,1081419769,3228463250]),v3([3226922179,3218148532,3230385205]));for(a,e)in p.to_array().into_iter().zip([1043729480,3198481916,1066564598]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1071812210,1068593469,3228383058]),v3([3216624845,3228422437,3204840667]),v3([3222204416,3228750721,1078566739]),v3([1058396971,3213855572,3215424812]));for(a,e)in p.to_array().into_iter().zip([3197792448,3202883405,1071686659]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1078004356,1068183488,1078994785]),v3([3229516402,1065116695,3209035075]),v3([3209441877,1077109251,3210316192]),v3([1059311031,3209689050,1070612360]));for(a,e)in p.to_array().into_iter().zip([3210437872,1061735572,1065962542]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3224829481,3226788271,3204418301]),v3([3228983345,1050244667,1081758922]),v3([1071351495,3222376680,1083137930]),v3([3226396044,3229764596,1019420456]));for(a,e)in p.to_array().into_iter().zip([1019812384,1004036777,1064830570]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3212024026,1058239758,3210313799]),v3([1072466579,3204882338,1083662256]),v3([1079795937,3227373342,3225180303]),v3([1083694141,3175617929,3181597573]));for(a,e)in p.to_array().into_iter().zip([1057644450,1057919328,3183970318]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3214390535,1070679895,3217026040]),v3([3219920774,1073156800,1073334840]),v3([3231528183,1075879460,3229078938]),v3([1081239824,3218706738,3226972183]));for(a,e)in p.to_array().into_iter().zip([1053525007,1051605705,1048985897]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3217322601,1080910362,3219757454]),v3([3213280308,3226095901,3189088392]),v3([3213156663,1076605573,3231699783]),v3([1080336731,3216276885,1084079717]));for(a,e)in p.to_array().into_iter().zip([3192168728,1063807726,1049721520]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3230476772,3230403855,1081864281]),v3([3224514242,1062989410,1066661047]),v3([1067500391,3228591579,1046809733]),v3([3229009472,3217172202,1045022072]));for(a,e)in p.to_array().into_iter().zip([3204317312,1039592161,1068503410]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1078911482,1054602308,3223643792]),v3([1078776841,3213375873,1077349323]),v3([3222907357,1078363150,3207171438]),v3([3208939126,1079089264,1070223132]));for(a,e)in p.to_array().into_iter().zip([1065297662,1069831911,3217287782]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3227850416,1056415562,3223238353]),v3([1083291839,1077851629,1073871512]),v3([1084205613,1060217204,3231081735]),v3([3201004919,1083284367,1082954735]));for(a,e)in p.to_array().into_iter().zip([3221487252,1068419154,1071199447]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1069609278,1069790719,1081983163]),v3([3204779390,1067718968,3221994331]),v3([3211816682,3199284381,1067922473]),v3([1043575482,3216447841,1061177731]));for(a,e)in p.to_array().into_iter().zip([3198999532,1069927461,3193219409]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1078872223,3231589646,3225183632]),v3([1082669129,3230415725,3181957540]),v3([3229104858,3202917476,1081063593]),v3([1061637264,3198297502,1078590430]));for(a,e)in p.to_array().into_iter().zip([1068853651,1056202626,3211068135]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3230057825,1050296001,3230322027]),v3([3228114720,1057262200,1081531698]),v3([1071652266,3227014068,1080006598]),v3([3223000298,1065789632,1079950006]));for(a,e)in p.to_array().into_iter().zip([3200916636,3188905958,1069867556]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1080309102,1082342717,3229316657]),v3([3211848292,3227549611,3226117037]),v3([3227716379,3225614686,3215813266]),v3([3208334395,1079548264,3222826958]));for(a,e)in p.to_array().into_iter().zip([1067020305,3216308351,1067157615]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1083898333,3178880773,1081511966]),v3([3223641659,3223246509,1042911402]),v3([3227393361,1083546508,3231579512]),v3([1073042223,3224016968,1072568004]));for(a,e)in p.to_array().into_iter().zip([3213290808,1046812974,1072318994]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3226006860,1083273371,1049518175]),v3([3230801817,3220314846,1081768767]),v3([1071484148,1083460241,1042433836]),v3([1001153608,1071242981,3230485172]));for(a,e)in p.to_array().into_iter().zip([1050663730,1057990060,1040298731]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1070402299,1053892798,3201022370]),v3([3220558682,1075210053,1067108715]),v3([1054322117,3230846241,1065449155]),v3([1083669365,1065420467,1063339356]));for(a,e)in p.to_array().into_iter().zip([1051489058,1044625598,1056026750]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3227493476,3230297611,3207947515]),v3([1081259471,3201783359,3207869933]),v3([1056027492,3230210997,3227967431]),v3([3228507365,1067072314,1027116460]));for(a,e)in p.to_array().into_iter().zip([3202392452,1063149133,1058140789]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3231686048,3222385623,1083437172]),v3([3204827008,3230364943,3207831324]),v3([1081376799,1075072101,1068727561]),v3([3222526704,3230043468,1075343443]));for(a,e)in p.to_array().into_iter().zip([3213134477,1035667233,1073273371]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3222777778,3199996963,1082439153]),v3([1061806501,1058029150,3223713264]),v3([3231436626,3225406809,3231610970]),v3([3198642133,3220962388,1068225288]));for(a,e)in p.to_array().into_iter().zip([3199482244,3179654599,1068853565]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1075873003,1073866357,1073888543]),v3([1066463063,3219633301,3228776787]),v3([3218163148,3224703449,1076809210]),v3([1061229179,3205219259,3226673617]));for(a,e)in p.to_array().into_iter().zip([3223017978,1054073104,1077992920]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3206409897,3230129453,3220801893]),v3([1075412209,1072800707,3231011060]),v3([1080717279,3226083206,3183614846]),v3([1084016710,3217338044,1076518636]));for(a,e)in p.to_array().into_iter().zip([1028240704,1073411127,3212919265]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3227461567,1075584066,3229218038]),v3([3218447227,1060817108,1082854238]),v3([3213292138,1066886189,1079553375]),v3([1076337959,1082864906,3216164020]));for(a,e)in p.to_array().into_iter().zip([3253030124,1107855669,3230602744]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1070388181,3212174059,3221031457]),v3([1074809576,3216719702,1079296048]),v3([1074792763,3218308037,1082107240]),v3([3225923311,3202609480,1072986565]));for(a,e)in p.to_array().into_iter().zip([1090690994,3236472131,1037662182]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3223089600,3230261452,3203576449]),v3([1078660433,1074041726,3220130108]),v3([1048673108,3231260712,1080349719]),v3([3197177043,3212316814,1065650706]));for(a,e)in p.to_array().into_iter().zip([3202496892,1007514263,1068987258]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3204816672,3223649233,1076230191]),v3([3230093169,1072900221,3230853088]),v3([3201640589,1060572414,1066270936]),v3([3230998133,1059957111,1080158144]));for(a,e)in p.to_array().into_iter().zip([3196278394,1065832078,1045182597]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1079774897,3217897417,3210366836]),v3([3211942795,1074291203,1082713435]),v3([1075166360,3219807939,1076132300]),v3([1044101543,3221818384,3230872306]));for(a,e)in p.to_array().into_iter().zip([3193411400,1062684330,1052589648]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1066274246,1077514770,1077680562]),v3([3223926492,3227009568,3218191089]),v3([3205073375,1069886817,3197578415]),v3([3230518951,1074809477,1069503610]));for(a,e)in p.to_array().into_iter().zip([3203845900,1068753307,1033708161]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1066580998,1081268404,1067284004]),v3([3218188109,3219428171,1083178170]),v3([3217471211,3223317755,3224145472]),v3([3216412058,1066803368,1049089713]));for(a,e)in p.to_array().into_iter().zip([3192309320,3205950861,1071927183]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3213534201,1077737129,3207641706]),v3([3226521043,1082798853,1074237425]),v3([3227247916,3229137732,1074918883]),v3([1083005303,1083540421,1076694367]));for(a,e)in p.to_array().into_iter().zip([1058183128,1043193837,1048830042]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1074453172,1063606269,3189655634]),v3([1077184024,1076838301,1018591636]),v3([1070672888,1077251178,3224624149]),v3([3205765226,3185600811,1071812218]));for(a,e)in p.to_array().into_iter().zip([1057305842,1042311411,1051025826]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1065545577,3221574142,3219376059]),v3([3226957168,3214029150,1080661071]),v3([1057805043,3221748860,3220053427]),v3([3222875862,1080497405,1081944359]));for(a,e)in p.to_array().into_iter().zip([3179385798,1065613533,1023675373]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3201184343,1074484975,1067117253]),v3([3195790164,3226005733,3225689694]),v3([1070308934,3222569429,1032941737]),v3([1075456802,1047442259,3225795445]));for(a,e)in p.to_array().into_iter().zip([3204617837,1064658694,1057828711]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3207842373,1083742725,1073927279]),v3([3225942533,3231668845,1075471224]),v3([1067444208,1076074462,1077375810]),v3([1073726134,1083639917,1083008933]));for(a,e)in p.to_array().into_iter().zip([3184896800,1066301843,3155128452]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3213578207,3205326975,3230353307]),v3([1078344177,1058021438,1080159815]),v3([3204614706,1079494451,3230319022]),v3([3227657781,1070713768,3205544578]));for(a,e)in p.to_array().into_iter().zip([1017108560,1059918997,1050401105]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1079103003,1078127995,3230152587]),v3([3229653857,3227011671,1078796299]),v3([1083558506,3230208791,3204236230]),v3([3205673469,3230704536,3215447976]));for(a,e)in p.to_array().into_iter().zip([3197990438,1057616478,1061472437]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1057983805,3205440688,1073821949]),v3([3213044884,1082469493,1073661688]),v3([1082871045,3228368023,3202116432]),v3([3229934730,3217188365,1080005773]));for(a,e)in p.to_array().into_iter().zip([1051023829,1053712412,1049380367]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1080330145,1071859253,1065834877]),v3([3221765061,1080168517,3223554513]),v3([3219151809,3226185975,1083784791]),v3([1075706242,1080960744,1060856105]));for(a,e)in p.to_array().into_iter().zip([3193597452,1041357603,1065947741]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1070899393,3223044107,3230006768]),v3([1078595846,3218713012,1070747021]),v3([3211732569,1069707182,3214293007]),v3([3226575789,1050879186,1083835461]));for(a,e)in p.to_array().into_iter().zip([1058969272,1065269992,3206369696]){eq(a,e)}}
{let p=barycentric::barycentric(v3([3224745277,1083851878,1080270782]),v3([1063234032,3208388014,3227815975]),v3([1068382768,1082463089,3199274907]),v3([1071708393,3227202560,1080444746]));for(a,e)in p.to_array().into_iter().zip([3206568392,1067874234,1051120807]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1083987727,3231441944,1054564272]),v3([1072152912,3230675198,3225091399]),v3([1046012708,3181567703,3216232919]),v3([3219936666,1075575792,3228527195]));for(a,e)in p.to_array().into_iter().zip([1056451796,1072113767,3215274908]){eq(a,e)}}
{let p=barycentric::barycentric(v3([1066551662,1074250194,1073756984]),v3([3230440132,3224646479,1056950181]),v3([3225990422,3231070048,3224264726]),v3([1083904637,1060371161,3214678612]));for(a,e)in p.to_array().into_iter().zip([1067197848,3212452315,1061279404]){eq(a,e)}}
}

fn skirt_split_boundaries(height: f32, maximum_y: f32, length: f32, ranges: &[[f32; 2]]) -> Vec<f32> {
    let seed = ((height + (maximum_y + length)) * 1000.0) as u64;
    let rng = fastrand::Rng::with_seed(seed);
    let mut boundaries = vec![0.0];
    let mut splits = Vec::new();
    for &[start, end] in ranges {
        let span = (end - start) * length;
        let count = ((span / 0.69).ceil() as usize).wrapping_add(1);
        splits.clear();
        utils::random_splits_into(count, 0.32 / span, &rng, &mut splits);
        boundaries.reserve(splits.len().saturating_sub(1));
        for &t in splits.iter().skip(1) {
            assert!((0.0..=1.0).contains(&t), "裙边分割坐标超出范围");
            boundaries.push(start + (end - start) * t);
        }
    }
    boundaries
}


fn main(){ let b=skirt_split_boundaries(4.5,6.0,12.0,&[[0.,0.25],[0.25,0.5],[0.5,0.75],[0.75,1.]]);let e=[1057962523, 1065789848, 1072644543, 1075196890, 1077936128, 1080848870, 1082630177, 1083564778, 1085086958, 1086324736, 1087518676, 1088730376, 1090129480, 1090884123, 1091567616, 1092214557, 1092740529, 1093312712, 1094045956, 1094713344];assert_eq!(b.len(),e.len()+1);for (i,w) in b.windows(2).enumerate(){let c=(w[0]+w[1])*0.5;let h=(w[1]-w[0])*0.5;assert_eq!(((c+h)*12.).to_bits(),e[i],"{i}");} println!("20 original rectangle skirt endpoints match");}

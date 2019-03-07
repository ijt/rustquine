fn main() {
	let f = r#"fn main() {
	let f = <F>;
    let left = (60u8 as char).to_string();
    let left2:String = vec![114u8, 35u8, 34u8].into_iter().map(|x| (x as char).to_string()).collect();
    let right = (62u8 as char).to_string();
    let right2:String = vec![34u8, 35u8].into_iter().map(|x| (x as char).to_string()).collect();
    let f2 = f.replace(&left, &left2).replace(&right, &right2);
	let f3 = f2.replacen("F", f, 1);
    println!("{}", f3);
}"#;
    let left = (60u8 as char).to_string();
    let left2:String = vec![114u8, 35u8, 34u8].into_iter().map(|x| (x as char).to_string()).collect();
    let right = (62u8 as char).to_string();
    let right2:String = vec![34u8, 35u8].into_iter().map(|x| (x as char).to_string()).collect();
    let f2 = f.replace(&left, &left2).replace(&right, &right2);
	let f3 = f2.replacen("F", f, 1);
    println!("{}", f3);
}

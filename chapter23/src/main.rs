fn main() {
    // cannot borrow `v` as mutable because it is also borrowed as immutable
    // let mut v = vec![12];
    // let ref_to_first = &v[0];
    // v.push(13);
    // println!("{}", *ref_to_first);

    println!("23.2 ===================");
    // 借用
    let a = 12;
    let mut b = &a;
    let c = &a;
    print!("{} {} ", b, c);
    b = &23;
    println!("{}", b);

    println!("23.5 ===================");
    let mut _n = 12;
    {
        let _ref_n = &_n; // _nの非可変借用を始める
        let _m = _n; // _nの一時的な非可変借用
        let _k = *_ref_n; // _nの最初の非可変借用を終える
    }
    _n = 13; // _nの一時的な可変借用
    _n += 1; // _nの一時的な可変借用
}

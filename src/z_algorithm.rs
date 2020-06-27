//提出時は下と一番下のコメントアウトを外す。
//mod z_algorithm {

#[allow(dead_code)]
///Z-Algorithm
///
/// 文字列の任意の部分からの最長共通接頭辞の長さを求める
pub fn z_algorithm(s: &str) -> Vec<usize> {
    let mut res = vec![0; s.len()];

    res[0] = s.len();

    let mut i = 1;
    let mut j = 0;

    while i < s.len() {
        while i + j < s.len() && s[i + j..i + j + 1] == s[j..j + 1] {
            j += 1;
        }
        res[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while k < j && k + res[k] < j {
            res[i + k] = res[k];
            k += 1;
        }
        i += k;
        j -= k;
    }

    res
}
//}

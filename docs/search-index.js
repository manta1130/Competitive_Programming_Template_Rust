var N=null,E="",T="t",U="u",searchIndex={};
var R=["cp_template","modint","usize","option","cp_template::modint","string","result","try_from","try_into","borrow_mut","cp_template::prime_number","type_id","formatter","DPFactorial","PrimeFactorization","ToCharVec","cp_template::ordfloat","ordfloat","ordering","OrdFloat"];
searchIndex["cp_template"]={"doc":E,"i":[[0,"input",R[0],"標準入力の簡易入力関数モジュール",N,N],[5,"input_line_str","cp_template::input","文字列を一行読み込む # Example ```ignore use cp_template::*;",N,[[],[R[5]]]],[5,"p",E,"一つの変数を出力する。 ``` use cp_template::*;",N,[[[T]]]],[5,"input_vector2d",E,"指定した行数を読み込み、二次元配列に変換する。 # Examples ```ignore use…",N,[[[R[2]]],[["vec",["vec"]],["vec"]]]],[5,"input_vector",E,"一行読み込み、配列(Vec)に変換する。 # Examples ```ignore use…",N,[[],["vec"]]],[5,"input_vector_row",E,"指定された行数を読み込む",N,[[[R[2]]],["vec"]]],[8,R[15],E,"StringをVecに変換するトレイト",N,N],[10,"to_charvec",E,E,0,[[["self"]],[["vec",["char"]],["char"]]]],[0,R[1],R[0],"任意の素数を法とする変数",N,N],[3,"Modint",R[4],E,N,N],[3,R[13],E,E,N,N],[11,"new",E,E,1,[[[R[2]]],[R[1]]]],[11,"from",E,E,1,[[[R[2]]],[R[1]]]],[11,"add_uint",E,E,1,[[["self"],[R[2]]],["self"]]],[11,"sub_uint",E,E,1,[[["self"],[R[2]]],["self"]]],[11,"mul_uint",E,E,1,[[["self"],[R[2]]],["self"]]],[11,"div_uint",E,E,1,[[["self"],[R[2]]],["self"]]],[11,"inv",E,E,1,[[["self"]],["self"]]],[11,"pow",E,E,1,[[["self"],[R[2]]],["self"]]],[11,"get_value",E,E,1,[[["self"]],[R[2]]]],[11,"new",E,E,2,[[[R[2]]],["dpfactorial"]]],[11,"get_factorial",E,E,2,[[["self"],[R[2]]],[R[1]]]],[11,"get_factorial_inv",E,E,2,[[["self"],[R[2]]],[R[1]]]],[11,"get_combination",E,E,2,[[["self"],[R[2]]],[R[1]]]],[11,"get_permutation",E,E,2,[[["self"],[R[2]]],[R[1]]]],[0,"prime_number",R[0],"素数関係のライブラリ",N,N],[3,R[14],R[10],E,N,N],[11,"calc",E,"素因数を計算するイテレータを返す。",3,[[[R[2]]],["primefactorization"]]],[0,"graph",R[0],"グラフ関係のライブラリ",N,N],[5,"bellman_ford","cp_template::graph","最短経路計算(ベルマンフォード法)",N,[[["vec"],["vec"]],["bool"]]],[5,"dijkstra",E,"最短経路計算(ダイクストラ法)",N,[[["vec"],[R[2]]],[["vec",[R[3]]],[R[3],["isize"]]]]],[5,"warshall_floyd",E,"最短経路計算(ワーシャルフロイド法)",N,[[["vec"]]]],[0,"math",R[0],"数学関係のライブラリ",N,N],[5,"gcd","cp_template::math","最大公約数を求める。",N,[[[T]],[T]]],[5,"lcm",E,"最小公倍数を求める。",N,[[[T]],[T]]],[5,"extgcd",E,"拡張ユーグリッドの互除法",N,[[[T],[T]],[T]]],[0,"vectools",R[0],"配列に関する補助関数ライブラリ",N,N],[5,"upper_bound","cp_template::vectools","C++のupper_boundと同等の実装",N,[[["vec"],[T]],[R[2]]]],[5,"lower_bound",E,"C++のlower_boundと同等の実装",N,[[["vec"],[T]],[R[2]]]],[5,"next_permutation",E,"次の順列を求める。",N,[[["vec"]],["bool"]]],[0,R[17],R[0],"Ordを実装したf64ラッパー",N,N],[3,R[19],R[16],E,N,N],[12,"0",E,E,4,N],[14,"input",R[0],"空白で区切られた複数の値の読み込む。 # Example ```ignore use cp_template::*;",N,N],[11,"to_string",R[4],E,1,[[["self"]],[R[5]]]],[11,"from",E,E,1,[[[T]],[T]]],[11,"to_owned",E,E,1,[[["self"]],[T]]],[11,"clone_into",E,E,1,[[["self"],[T]]]],[11,"into",E,E,1,[[],[U]]],[11,R[7],E,E,1,[[[U]],[R[6]]]],[11,R[8],E,E,1,[[],[R[6]]]],[11,R[9],E,E,1,[[["self"]],[T]]],[11,"borrow",E,E,1,[[["self"]],[T]]],[11,R[11],E,E,1,[[["self"]],["typeid"]]],[11,"from",E,E,2,[[[T]],[T]]],[11,"into",E,E,2,[[],[U]]],[11,R[7],E,E,2,[[[U]],[R[6]]]],[11,R[8],E,E,2,[[],[R[6]]]],[11,R[9],E,E,2,[[["self"]],[T]]],[11,"borrow",E,E,2,[[["self"]],[T]]],[11,R[11],E,E,2,[[["self"]],["typeid"]]],[11,"into_iter",R[10],E,3,[[],["i"]]],[11,"from",E,E,3,[[[T]],[T]]],[11,"into",E,E,3,[[],[U]]],[11,R[7],E,E,3,[[[U]],[R[6]]]],[11,R[8],E,E,3,[[],[R[6]]]],[11,R[9],E,E,3,[[["self"]],[T]]],[11,"borrow",E,E,3,[[["self"]],[T]]],[11,R[11],E,E,3,[[["self"]],["typeid"]]],[11,"from",R[16],E,4,[[[T]],[T]]],[11,"into",E,E,4,[[],[U]]],[11,R[7],E,E,4,[[[U]],[R[6]]]],[11,R[8],E,E,4,[[],[R[6]]]],[11,R[9],E,E,4,[[["self"]],[T]]],[11,"borrow",E,E,4,[[["self"]],[T]]],[11,R[11],E,E,4,[[["self"]],["typeid"]]],[11,"eq",R[4],E,1,[[["self"],[R[1]]],["bool"]]],[11,"ne",E,E,1,[[["self"],[R[1]]],["bool"]]],[11,"eq",R[16],E,4,[[["self"],[R[17]]],["bool"]]],[11,"ne",E,E,4,[[["self"],[R[17]]],["bool"]]],[11,"clone",R[4],E,1,[[["self"]],[R[1]]]],[11,"partial_cmp",R[16],E,4,[[["self"]],[[R[18]],[R[3],[R[18]]]]]],[11,"next",R[10],E,3,[[["self"]],[R[3]]]],[11,"cmp",R[16],E,4,[[["self"]],[R[18]]]],[11,"fmt",R[4],E,1,[[["self"],[R[12]]],[R[6]]]],[11,"fmt",E,E,1,[[["self"],[R[12]]],[R[6]]]],[11,"from_str",E,E,1,[[["str"]],[R[6]]]],[11,"deref",R[16],E,4,[[["self"]],["f64"]]]],"p":[[8,R[15]],[3,"Modint"],[3,R[13]],[3,R[14]],[3,R[19]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);
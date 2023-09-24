---
---
# Golang Study Note
## 変数
複数定義することができる。型定義は後ろにつける
```
x int, y int
i, j int = 42
x int, y int = 42, 21 // これはできない
var a [10]int // array
```
`:=`で暗黙的に型定義をすることができる
```
i, j := 42, 2701 // 42, 2701 is int
```
## ポインタ
&はポインタを示す
*はポインタの指す先の変数を示す
具体的に以下のような変数定義がある時を見てみる。
変数`n`のアドレスは`123`, 変数`n`の値は`hoge`になります。
```
var n := "hoge"
fmt.Println(&n) // 123
fmt.Println(*n) // hoge
```
A Tour of Goのサンプルで見てみる
```
i, j := 42, 2701

p := &i         // iとpは同じアドレスを参照していることになる
fmt.Println(*p) // iのアドレスに格納されている値42が出力される
*p = 21         // pの値を21に変更
fmt.Println(i)  // pと同じアドレスを参照しているiは上記代入された21が出力される

p = &j         // pとjのアドレスが同じになる
*p = *p / 37   // pの値を73に変更
fmt.Println(j) // pと同じアドレスを参照しているjは上記代入された73が出力される
fmt.Println(i) // iとpは別のアドレスを参照しているため73ではなく21が出力される
```
構造体は
```
type Vertex struct {
	X int
	Y int
}

func main() {
	v := Vertex{1, 2}
	p := &v
	p.X = 1e9 // (*p).Xと同等
	fmt.Println(v)　// {1000000000 2}
}
```

## メソッドと関数
メソッドは、レシーバ引数を伴う関数です。
以下は、AbsメソッドがvというVertex型のレシーバを持つことを意味する
```
type Vertex struct {
	X, Y float64
}

func (v Vertex) Abs() float64 {
	return math.Sqrt(v.X*v.X + v.Y*v.Y)
}

func main() {
	v := Vertex{3, 4}
	fmt.Println(v.Abs())
}
```
これをメソッドではなく関数で表した場合
```
type Vertex struct {
	X, Y float64
}

func Abs(v Vertex) float64 {
	return math.Sqrt(v.X*v.X + v.Y*v.Y)
}

func main() {
	v := Vertex{3, 4}
	fmt.Println(Abs(v))
}
```
structだけではなく任意の方にもメソッドを宣言できる
```
type MyFloat float64
func (f MyFloat) Abs() float64 {
	// 略
}
```
---
---
# React Study Note
## Reactとは
ユーザインタフェース構築のためのJavaScriptライブラリ
[公式ドキュメント](https://ja.reactjs.org/tutorial/tutorial.html)

render関数（ビューファイルをhtmlにレンダリングする機能）を中心に作られており、ページ内容が更新されるためにrender関数を呼び出し仮想的なページを構築し、最終的にDOMを変更します。
この際、前回のDOMと今回のDOMを比較し必要最低限の更新のみ行われるため、パフォーマンスが維持されます。

# JSX(JavaScript XML)
JavaScriptのコードの中でXML風宣言的な記述を行うことができます。
## JSXを利用しない場合
JSXのマークアップ部分は最終的にJavaScriptに変換されるので、ReactではJSXを利用しないこともできますが、利用は推奨されています。

## 使い方
1.Node.jsのインストール  
2.Create React Appのインストール
```
# Node.jsがインストールされている環境で実行
npx create-react-app {アプリケーション名}

# TypeScriptで開発したいとき
npx create-react-app {アプリケーション名} --templete typescript
```

## Create React App
Create React Appはツールチェインです。
Create React Appは内部では、`Babel`と`Webpack`を利用しているためCreate React Appではなく個別にインストールしてゼロからJavaScriptツールチェインを設定した環境構築をすることも可能です。

## 構造
```
{アプリケーション名}/
  ├── src/
  |    ├── components/
  |    |　　   └── App.js 
  |    └── index.js 
  ├── stylesheet.css
  └── index.html  // index.jsをロードする
```
App.js
```
import React from 'react';

class App extends React.Component {
  render() {
    // Componentのrender()の中ではjsが書ける
    const text = 'JavaScript';
        
    return (
      <div>
        {/* returnの中ではJSX。これはJSXのコメントの書き方 */}
        <h1>{ text }</h1>        
      </div>
    );
  }
}
export default App;
```
index.js
```
import React from 'react';
import ReactDOM from 'react-dom';
import App from './components/App';

// この記述によりApp.jsのJSXが、HTMLに変換
ReactDOM.render(<App />, document.getElementById('root'));
```
index.html
```
<!DOCTYPE html>
<html>
  <head>
    <link rel="stylesheet" href="stylesheet.css">
    <title>React App</title>
  </head>
  <body>
    <-- 変換されたHTMLはgetElementByIdで定義されたidに埋め込まれる -->
    <div id="root"></div>
    <script src="bundle.js"></script>
  </body>
</html>

```

## ライフサイクル
### コンポーネント作成時
必須なのはrenderのみです。
- getDefaultProps（コンポーネント作成時のみ）  
インスタンス作成時おやコンポーネントが値を指定しなかったとき使用される。コンポーネント作成時のみ呼ばれる

- getInitialState  
インスタンス作成時に状態を初期化。以降this.propsが利用可能になる。

- componentWillMount  
renderメソッドが呼び出される前にコンポーネントの状態を変更する

- render  
必須。コンポーネントの出力表現（仮想DOM）が作成される。

- componentDidMount  
React.findDOMNode経由で表示後のDOMにアクセスできる。
表示後の要素の高さを測ったり、タイマーを設定したりする場合利用。

### コンポーネント作成後
この時点でコンポーネントは描画済みのため、ユーザはそれを操作することができる。
- componentWillReceiveProps  
親コンポーネントにより任意のタイミングでpropsを変更する際に呼ばれ、stateを変更したりその他の処理を行います。

- shouldComponentUpdate  
propsやstateが変更されたにもかかわらず、表示に影響しないことがわかっている場合falseを返します。
renderやcomponentWillUpdateやcomponentDidUpdateをスキップすることができるので、処理速度が最適化できます。

- componentWullUpdate  
propsと　stateの変更による描画が行われる直前に呼び出されます。
- render  
- componentDidUpdate  
表示後の要素の高さを測ったり、タイマーを設定したりする場合利用。

## イベント
onClick()イベントの例
```
<button onClick={() => {処理}}>
```
Example
```
<button onClick={() => { console.log('ひつじ仙人')}}>ひつじ仙人</button>
```

## state
stateを使った表示の変更は、
1.stateの定義、2.stateの表示、3.stateの変更の順に行なっていく。
state、constructorの中で、オブジェクトとして定義します。
```
class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = { text: 'HelloWorld'}; // 1.stateの定義
  }

  render() {        
    return (
      <div>
        {/* 2.stateの表示 */}
        <h1>{ this.state.text }</h1>
        
        {/* 3.stateの変更 */}
        <button onClick={() => {this.setState({text:'HelloReact'})}}>
            React
        </button>
        
        <button onClick={() => {this.setState({text:'HelloJavaScript'})}}>
            JavaScript
        </button>     
      </div>
    );
  }
}
```

### メソッド化
```
class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = { text: 'HelloWorld'}; // 1.stateの定義
  }

  handleClick(text) {
    this.setState({text:text});
  }
  render() {        
    return (
      <div>
        {/* 2.stateの表示 */}
        <h1>{ this.state.text }</h1>
        
        {/* 3.stateの変更 */}
        <button onClick={() => {this.handleClick('HelloReact')}}>
            React
        </button>
        
        <button onClick={() => {this.handleClick('HelloJavaScript')}}>
            JavaScript
        </button>     
      </div>
    );
  }
}
```

## jsxのクラス
classではなくclassNameになる
```
<h1 className='title'>Hello World</h1>
```

cssでのクラス指定
```
.title {
  color: #e6855e;
  background-color: #f3f372;
}
```
        
## props
外部から注入される変数。
以下は、Board の renderSquare メソッド内で、props として value という名前の値を Square に渡すコード
```
class Board extends React.Component {
  renderSquare(i) {
    return <Square value={i} />;
  }
}
```
Square の render メソッドで、渡された値を表示するコード。
```
class Square extends React.Component {
  render() {
    return (
      <button className="square">
        {this.props.value}
      </button>
    );
  }
}
```
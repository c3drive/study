---
---
# Ruby on Rails Study Note

## フォルダ構成
```
{アプリケーション名}/
  ├── app/           　　　　　　　　 // メインフォルダ
  |    ├── assets/    　　　　　　　  // リソースフォルダ
  |    |　　├── images/
  |    |　　├── javascript/
  |    |　　└── stylesheets/
  |    |　　   └── home.scss        // ■
  |    ├── models/    　　　　　　　  // モデルに関するフォルダ
  |    |　　   └── post.rb          // ▲
  |    ├── views/    　　　　　　　　 // ビュー（見た目）フォルダ
  |    |　　├── layout/
  |    |　　|  └── application.html.erb // 共通HTML
  |    |　　└── home/               // ■
  |    |　　   └── top.html.erb     // ■
  |    └── controllers/    　　　　　// コントロールフォルダ
  |     　　└── home_controller.rb  // ■
  ├── config/ 　　　　　　　　　　　　 // 設定情報に関するフォルダ
  |    └── routes.rb               // ■
  ├── db/     　　　　　　　　　　　　 // データベースに関するフォルダ
  |    └── migrate/
  |     　　└── YYYYMMDDHHMMSS_create_posts.rb // ▲
  ├── public/                      // 静的コンテンツフォルダ
  ├── spec/                        // XXXXフォルダ
  |    └── posts_spec.rb           // ▲
  └── …/      　　　　　　　　　　　　 // その他
```
■：`例）rails g controller home top`で作成されるもの
▲：`例）rails g model Post content:text`で作成されるもの


## 各種操作
### アプリケーション作成
```
rails new {アプリケーション名}
```

### サーバー起動
```
rails server
```
作成されるURL：http://localhost:3000

### トップページの作成
```
rails generate controller {コントローラ名} {アクション名}
// 例　rails generate controller home top
```
もしくは
```
rails g controller {コントローラ名} {アクション名}
```
作成されるURL：http://localhost:3000/home/top

### ページを構成するのに必要なもの
- views  
*.erbは、HTMLなどの文章の中にRubyスクリプトを埋め込むためのライブラリファイル
```
<h1>Home#top</h1>
<p>Find me in app/views/home/top.html.erb</p>
```

- controller  
コントローラと同じ名前のビューフォルダから、アクションと同じ名前のHTMLファイルを探してブラウザに返します。
```:home_controller.rb 
class HomeController < ApplicationController
  def top
  end
end
```
`class A < B`は、クラスAがクラスBを継承している状態。

- routing  
Rails内ではコントローラを経由してビューを返していますが、ブラウザとコントローラを繋ぐ役割を担うのがルーティングです。
ページが表示されるまでに、ルーティング→コントローラ→ビューという順で処理が行われている
```:routes.rb 
Rails.application.routes.draw do
  get "home/top" => "home#top"
end
```

### ページの追加（既存のコントローラにアクションを追加する場合）
ルーティングとアクションを追加することで可能
```:routes.rb 
Rails.application.routes.draw do
  get "top" => "home#top"
  get "about" => "home#about"
end
```
```:home_controller.rb 
class HomeController < ApplicationController
  def top
  end

  def about
  end
end
```
ファイルは手動で追加：app/views/home/about.html.erb

### ページの追加（既存のコントローラとは別にアクションを追加する場合）
例えば既存のhomeとは別の機能を有するためコントロールを分ける
```
// 例 rails generate controller posts index
```
posts：投稿コントローラの意。  
index：一覧ページの意。

##  erb ファイル
`erb`とは、`Embedded Ruby(埋め込みRuby)`の略。

### Rubyコードの埋め込み方の基本
```
// Rubyコードの埋め込み（変数の定義）
<% post1 = "XXXXX" %>
<% posts = [
    "XXXXX",
    "YYYYY"
    ]
%>

// Rubyコードをブラウザに表示したい場合（変数の参照）
<%= post1 %>

// each文
<% posts.each do | post | %>
    <p>
        <%= post %>
    </p>
<% end %>
```
### 変数をコントローラ内のアクションで定義する
変数を直接ビューに定義するのは一般的ではなく、通常アクションに記載する。
なお、アクションで定義した変数をビューで参照できるようにするには、`@`をつける。
```
class PostsController < ApplicationController
  def index
    @posts = [
        "XXXXX",
        "YYYYY"
        ]
  end
end
```
ビュー側でも`@`をつけないと参照ができない。
```
// each文
<% @posts.each do | post | %>
    <p>
        <%= post %>
    </p>
<% end %>
```
### 共通レイアウト
個別のビューの記述は、共通HTMLの`application.html.erb`の`<%= yield %>`の部分に代入されるため、`views/layout/application.html.erb`にヘッダー部などの共通のHTMLをまとめることができる。

### link_toメソッド
ビューでlink_toメソッドを使うと、`<a href="{URL"}>{リンクテキスト}</a>`に変換される。
```
<%= link_to("{リンクテキスト}", "{URL}") %>
// 例 <%= link_to("About", "/about") %>
```

## データベース
### テーブル作成の準備
まずは、マイグレーションファイルを作成。
```
rails generate model {テーブル名} {カラム名}:{データ型}
// 例 rails g model Post content:text
```
generate：gに略すことが可能。
Post：モデル名。postsテーブルを作成する際、単数形にする。
text：長い文字列

作成されるファイル：
- app/db/migrate/YYYYMMDDHHMMSS_create_posts.rb
- app/models/post.rd

### データベースに反映
```
rails db:migrate
```
ここで作成されるテーブルには定義したカラム以外に`id`、`created_at`、`updated_at`が追加される。

### 注意
データベースに反映されていないマイグレーションファイルが存在する状態でページアクセスをするとマイグレーションエラーが発生する。

## rails console
ターミナルで`rails console`を実行することでRubyのコードを実行できる環境を使える
```
// console起動
rails console

// Rubyコマンド。定義した変数はconsole終了まで使える
> text = "Hello"
> text + "World"
// 実行結果が表示
=> "HelloWorld"

// console終了
quit
```

### データ保存
モデルにて、`{モデル名}.new`でモデルのインスタンスを作成して対応するテーブルに保存する。
```
// インスタンス作成
post = {モデル名}.new(content: "Hello world")

// テーブルに保存
post.save
```
`new`、`save`は、モデルが継承している`ApplicationRecord`のメソッド
```
class {モデル名} < ApplicationRecord
end
```

### データ取得
```
// 最初のデータ取得
post = {モデル名}.first

// カラムの値を取得
post.{カラム名}

// 全てのデータ取得（配列で取得）
{モデル名}.all

// データの最初のデータの値を取得
{モデル名}.all[0].content
```
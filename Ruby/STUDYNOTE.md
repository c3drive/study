## 拡張子
index.rb

## コンソール出力
```
# comment
puts 1 + 2 % 2 / 1 * 9
puts "Hello" + " " + "Ruby"
```

## 変数
```
# よい変数名のルールは、英単語。2語以上はアンダーバー
user_name = "John"
num = 1
puts "こんにちは、" + name

# 代入
num = num + 2
num += 2

# 変数展開はダブルコーテーション
# シングルコーテーションの場合展開されない
puts "こんにちは、#{name}さん" ＃Johnに展開される
puts 'こんにちは、#{name}さん' "#{name}で出力"

```

### 変数展開のメリット
```
age = 13
puts age + "歳" # 数字＋文字列は連結できないのでエラー
puts "#{age}歳" #数字を文字列として表示
```

## 条件分岐
### if文
```
if age != 0
    puts "0ではありません"
elsif age >= 100# elseifではないので注意
    puts age >=100 # trueと表示
elsif age > 0 && age <= 99
    puts "1-99の間です"
else age == 0 || age == 100
    puts "それ以外"
end

if age != 
end
``` 
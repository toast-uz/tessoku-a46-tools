これは、書籍「競技プログラミングの鉄則」演習問題A46のローカルテストツールです。
・本ツールの著作権は、toast-uzが保有します。
・本ツールの作成にあたっては、AtCoder AHCコンテストのローカルテストツールを参考にしています。
・本ツールのライセンスはMITライセンスを適用します。
・本ツールには、「競技プログラミングの鉄則」著者やAtCoder社は全く関与していません。

実行には、Rust言語のコンパイル環境が必要です。
以下の実行例は、このREADMEが置かれているディレクトリで作業することを想定しています。

1) テストケース作成

in ディレクトリに、あらかじめ、seed 0-50に対する入力ファイルが作成されています。
seeds.txtに乱数seed値を記述して、以下を実行することで、入力ファイルを作成することが可能です。

  cargo run --release --bin gen seeds.txt

あらかじめ作成してある入力ファイル数は、AtCoderにおけるシステムテストのテストケース数に合わせています。
個々の入力ファイルは、必ずしもシステムテストと同一ではありません。

2) スコア計算

入力ファイル名をin.txt、出力ファイル名をout.txtとする場合、以下を実行することで、スコアを表示します。

  cargo run --release --bin vis in.txt out.txt

出力のビジュアライズ機能はありません。

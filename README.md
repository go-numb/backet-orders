# Backet orders

## 概要

このプロジェクトは、取引所のAPIを使用して、自動的に注文を出すRustプログラムです。取引量に基づいて上位の銘柄を選び、指定された条件で指値注文を出します。

## 主な機能

- Bybit APIを使用して市場データを取得
- 取引量に基づいて銘柄をソート
- 上位N銘柄に対して自動的に指値注文を出す
- コマンドライン引数を使用してパラメータをカスタマイズ

## 使用方法

1. 環境変数 `BYBIT` に API キーとシークレットを[.]ドット区切りで設定します（形式: `KEY.SECRET`）
2. 以下のコマンドラインオプションを使用してプログラムを実行します：


cargo run -- [OPTIONS]



オプション：
- `-n, --top-n <TOP_N>`: 取得する上位ランキングアイテムの数 (デフォルト: 30)
- `-r, --price-ratio <PRICE_RATIO>`: 価格比率のしきい値 (デフォルト: 0.01)
- `-u, --usd <USD>`: 注文に使用するUSD金額 (デフォルト: 100.0)

## 依存関係

- `clap`: コマンドライン引数の解析
- `log` と `env_logger`: ロギング
- `serde_json`: JSONデータの処理
- `tokio`: 非同期ランタイム
- `crypto-botters`: Bybit APIとの通信

## コードの構造

- `main()`: プログラムのエントリーポイント。引数の解析、クライアントの初期化、注文処理を行う
- `get_decimal_places()`: 文字列から小数点以下の桁数を取得する補助関数
- `bybit_v5` モジュール: Bybit API v5 のレスポンス構造体を定義

## 注意事項

- このプログラムは実際の取引を行います。使用する前に十分にテストし、リスクを理解してください。
- API キーとシークレットは安全に管理してください。

## Support
- [x] Bybit
- [ ] [TODO]Binance
  

## Author

[@_numbP](https://twitter.com/_numbP)

## License

[MIT](https://github.com/go-numb/backet_orders/blob/master/LICENSE)
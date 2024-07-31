use std::collections::HashSet; // HashSetをインポート

use lindera::{
    // linderaクレートから必要な要素をインポート
    Analyzer,
    BoxCharacterFilter,
    BoxTokenFilter,
    DictionaryConfig,
    DictionaryKind,
    JapaneseCompoundWordTokenFilter,
    JapaneseCompoundWordTokenFilterConfig,
    JapaneseIterationMarkCharacterFilter,
    JapaneseIterationMarkCharacterFilterConfig,
    JapaneseNumberTokenFilter,
    JapaneseNumberTokenFilterConfig,
    JapaneseStopTagsTokenFilter,
    JapaneseStopTagsTokenFilterConfig,
    LinderaResult,
    Mode,
    Tokenizer,
    TokenizerConfig,
    UnicodeNormalizeCharacterFilter,
    UnicodeNormalizeCharacterFilterConfig,
    UnicodeNormalizeKind,
};

fn main() -> LinderaResult<()> {
    // main関数を定義し、LinderaResultを返す
    let mut character_filters: Vec<BoxCharacterFilter> = Vec::new(); // character_filtersベクターを初期化

    // Unicode正規化フィルタの設定を作成
    let unicode_normalize_character_filter_config =
        UnicodeNormalizeCharacterFilterConfig::new(UnicodeNormalizeKind::NFKC);
    // Unicode正規化フィルタを作成
    let unicode_normalize_character_filter =
        UnicodeNormalizeCharacterFilter::new(unicode_normalize_character_filter_config);
    // フィルタをベクターに追加
    character_filters.push(BoxCharacterFilter::from(unicode_normalize_character_filter));

    // 繰り返し記号フィルタの設定を作成
    let japanese_iteration_mark_character_filter_config =
        JapaneseIterationMarkCharacterFilterConfig::new(true, true);
    // 繰り返し記号フィルタを作成
    let japanese_iteration_mark_character_filter =
        JapaneseIterationMarkCharacterFilter::new(japanese_iteration_mark_character_filter_config);
    // フィルタをベクターに追加
    character_filters.push(BoxCharacterFilter::from(
        japanese_iteration_mark_character_filter,
    ));

    // 辞書の設定を作成
    let dictionary = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC), // IPADIC辞書を使用
        path: None,                         // デフォルトパスを使用
    };

    // トークナイザーの設定を作成
    let config = TokenizerConfig {
        dictionary,            // 辞書設定を適用
        user_dictionary: None, // ユーザー辞書は使用しない
        mode: Mode::Normal,    // モードはノーマル
    };

    // トークナイザーを設定から作成
    let tokenizer = Tokenizer::from_config(config).unwrap();

    let mut token_filters: Vec<BoxTokenFilter> = Vec::new(); // token_filtersベクターを初期化

    // 複合語トークンフィルタの設定を作成
    let japanese_compound_word_token_filter_config = JapaneseCompoundWordTokenFilterConfig::new(
        DictionaryKind::IPADIC,                          // IPADIC辞書を使用
        HashSet::from_iter(vec!["名詞,数".to_string()]), // 数詞の名詞をターゲットにする
        Some("名詞,数".to_string()),                     // 数詞の名詞にマッチ
    )?;
    // 複合語トークンフィルタを作成
    let japanese_compound_word_token_filter =
        JapaneseCompoundWordTokenFilter::new(japanese_compound_word_token_filter_config);
    // フィルタをベクターに追加
    token_filters.push(BoxTokenFilter::from(japanese_compound_word_token_filter));

    // 数詞トークンフィルタの設定を作成
    let japanese_number_token_filter_config =
        JapaneseNumberTokenFilterConfig::new(Some(HashSet::from_iter(vec!["名詞,数".to_string()])));
    // 数詞トークンフィルタを作成
    let japanese_number_token_filter =
        JapaneseNumberTokenFilter::new(japanese_number_token_filter_config);
    // フィルタをベクターに追加
    token_filters.push(BoxTokenFilter::from(japanese_number_token_filter));

    // ストップタグトークンフィルタの設定を作成
    let japanese_stop_tags_token_filter_config =
        JapaneseStopTagsTokenFilterConfig::new(HashSet::from_iter(vec![
            "接続詞".to_string(),
            "助詞".to_string(),
            "助詞,格助詞".to_string(),
            "助詞,格助詞,一般".to_string(),
            "助詞,格助詞,引用".to_string(),
            "助詞,格助詞,連語".to_string(),
            "助詞,係助詞".to_string(),
            "助詞,副助詞".to_string(),
            "助詞,間投助詞".to_string(),
            "助詞,並立助詞".to_string(),
            "助詞,終助詞".to_string(),
            "助詞,副助詞／並立助詞／終助詞".to_string(),
            "助詞,連体化".to_string(),
            "助詞,副詞化".to_string(),
            "助詞,特殊".to_string(),
            "助動詞".to_string(),
            "記号".to_string(),
            "記号,一般".to_string(),
            "記号,読点".to_string(),
            "記号,句点".to_string(),
            "記号,空白".to_string(),
            "記号,括弧閉".to_string(),
            "その他,間投".to_string(),
            "フィラー".to_string(),
            "非言語音".to_string(),
        ]));
    // ストップタグトークンフィルタを作成
    let japanese_stop_tags_token_filter =
        JapaneseStopTagsTokenFilter::new(japanese_stop_tags_token_filter_config);
    // フィルタをベクターに追加
    token_filters.push(BoxTokenFilter::from(japanese_stop_tags_token_filter));

    // アナライザーを作成
    let analyzer = Analyzer::new(character_filters, tokenizer, token_filters);

    // テキストを定義
    let text = "".to_string();
    println!("text: {}", text); // テキストを表示

    // テキストをトークナイズ
    let tokens = analyzer.analyze(&text)?;

    // トークンを出力 (改行や空白を含まないトークンのみ)
    for token in tokens {
        if !token.text.chars().all(char::is_whitespace) && !token.text.contains('\n') {
            println!(
                "token: {:?}, start: {:?}, end: {:?}, details: {:?}",
                token.text, token.byte_start, token.byte_end, token.details
            );
        }
    }

    Ok(()) // 正常終了を返す
}

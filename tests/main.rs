use iter_seq_ext::prelude::*;

#[test]
fn contains() {
    let datas = [
        ("", ""),
        ("", "x"),
        ("abcdef", ""),
        ("abcdef", "a"),
        ("abcdef", "c"),
        ("abcdef", "f"),
        ("abcdef", "ab"),
        ("abcdef", "cd"),
        ("abcdef", "ef"),
        ("abcdef", "x"),
        ("abcdef", "cx"),
    ];
    for (text, pat) in datas {
        let target = text.chars();
        let pat_chars = &pat.chars().collect::<Vec<_>>();
        let asis = IteratorSeqExt::contains(target, pat_chars);
        let tobe = text.contains(pat);
        assert_eq!(asis, tobe);
    }
}

#[test]
fn replace() {
    let datas = [
        ("", "", ""),
        ("", "x", "X"),
        ("abcdefghi", "", ""),
        ("abcdefghi", "", "X"),
        ("abcdefghi", "x", "X"),
        ("abcdefghi", "cx", "CX"),
        ("abcdefghi", "a", "A"),
        ("abcdefghi", "e", "E"),
        ("abcdefghi", "i", "I"),
        ("abcdefghi", "abc", ""),
        ("abcdefghi", "def", ""),
        ("abcdefghi", "ghi", ""),
        ("abcdefghi", "abc", "A"),
        ("abcdefghi", "def", "D"),
        ("abcdefghi", "ghi", "G"),
        ("abcdefghi", "abc", "ABC"),
        ("abcdefghi", "def", "DEF"),
        ("abcdefghi", "ghi", "GHI"),
        ("abcdefghi", "abc", "ABCX"),
        ("abcdefghi", "def", "DEFX"),
        ("abcdefghi", "ghi", "GHIX"),
        ("abcdef-abcdef", "a", "A"),
        ("abcdef-abcdef", "c", "C"),
        ("abcdef-abcdef", "e", "E"),
        ("abcdef-abcdef", "ab", ""),
        ("abcdef-abcdef", "cd", ""),
        ("abcdef-abcdef", "ef", ""),
        ("abcdef-abcdef", "ab", "AB"),
        ("abcdef-abcdef", "cd", "CD"),
        ("abcdef-abcdef", "ef", "EF"),
    ];

    for (text, pat, rep) in datas {
        let target = text.chars();
        let pat_chars = &pat.chars().collect::<Vec<_>>();
        let rep_chars = &rep.chars().collect::<Vec<_>>();
        let asis = IteratorSeqExt::replace(target, pat_chars, rep_chars);
        let tobe = text.replace(pat, rep);
        assert_eq!(asis.collect::<String>(), tobe);
    }
}

#![feature(test)]
extern crate test;
extern crate pegtokenizer;

use test::Bencher;

use pegtokenizer::tokenize;

#[bench]
fn bench_syslog_wpa_supplicant_group_rekeying(b: &mut Bencher) {
    let log = r#"Oct 15 06:23:44 localhost wpa_supplicant[1212]: wlan0: WPA: Group rekeying completed with 64:7c:34:ab:93:88 [GTK=TKIP]"#;
    b.iter(|| tokenize(log));
}

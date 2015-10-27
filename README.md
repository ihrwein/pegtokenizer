# pegtokenizer

A simple log tokenizer written in Rust.

The purpose is to be able to tokenize log messages in most cases.

I run `pegtokenizer` on my notebook's `/var/log/syslog` file and got `342`
success from `344` entries which is above `99.4%`.

## Requirements

`pegtokenizer` supports only nightly Rust at the moment. The grammar has to be
generated with nightly builds and the other parts are so small that you would
always end up regenerating the grammar with nightly and building `pegtokenizer`
with stable.

## Usage

`pegtokenizer` reads the logs from the standard input and writes the result to
standard output.  The result is literally a `Result`, so if the parsing is
successful the values are wrapped in `Ok()` and in case of some failure the
error message is wrapped in `Err()`.

The first three lines in `/var/log/syslog`:

```
Oct 15 06:21:04 localhost rsyslogd: [origin software="rsyslogd" swVersion="7.4.4" x-pid="770" x-info="http://www.rsyslog.com"] rsyslogd was HUPed
Oct 15 06:21:05 localhost anacron[1198]: Job `cron.daily' terminated
Oct 15 06:21:05 localhost anacron[1198]: Normal exit (1 job run)
```

`pegtokenizer` creates the following tokens:

```
$ head /var/log/syslog | target/debug/pegtokenizer
Ok([Literal("Oct"), Int("15"), Int("06"), Int("21"), Int("04"), Literal("localhost"), Literal("rsyslogd"), Bracket([Literal("origin"), KVPair(Literal("software"), QuotedLiteral("\"rsyslogd\"")), KVPair(Literal("swVersion"), QuotedLiteral("\"7.4.4\"")), KVPair(Literal("x-pid"), QuotedLiteral("\"770\"")), KVPair(Literal("x-info"), QuotedLiteral("\"http://www.rsyslog.com\""))]), Literal("rsyslogd"), Literal("was"), Literal("HUPed")])
Ok([Literal("Oct"), Int("15"), Int("06"), Int("21"), Int("05"), Literal("localhost"), ProgramPid("anacron", "1198"), Literal("Job"), Literal("`cron.daily\'"), Literal("terminated")])
Ok([Literal("Oct"), Int("15"), Int("06"), Int("21"), Int("05"), Literal("localhost"), ProgramPid("anacron", "1198"), Literal("Normal"), Literal("exit"), Paren([Int("1"), Literal("job"), Literal("run")])])
```

## Token types
* `Brace`: tokens in braces `{`, `}`
* `Bracket`: tokens in brackets `[`, `]`
* `Paren`:  tokens in parens `(`, `)`
* `KVPair`: text like `key=value`
* `Audit`: audit log timestamp & id like `audit(1364481363.243:24287)`
* `ProgramPid`: `anacron[1198]`
* `QuotedLiteral`: literals  quoted by `'` or `"`
* `Float`: a floating point number with optinal exponents
* `Int`: `[0-9]+`
* `HexString`: hex strings with `0x` or `0X` prefix
* `MAC`: various MAC address formats
* `IPv4`: IPv4 address
* `Literal`: anything that wasn't matched and not a separator

## Token separator characters
* `:`
* `;`
* `,`
* ` `

# Benchmarks
All benchmarks tokenized a single log message.

```
test bench_auditd_cwd_log     ... bench:       9,792 ns/iter (+/- 5,005)
test bench_auditd_path_log    ... bench:      36,865 ns/iter (+/- 4,451)
test bench_auditd_syscall_log ... bench:      82,686 ns/iter (+/- 8,018)
```

```
test bench_syslog_wpa_supplicant_group_rekeying ... bench:      36,774 ns/iter (+/- 5,408)
```

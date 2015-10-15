#![feature(test)]
extern crate test;
extern crate pegtokenizer;

use test::Bencher;

use pegtokenizer::tokenize;

#[bench]
fn bench_auditd_syscall_log(b: &mut Bencher) {
    let log = r##"type=SYSCALL msg=audit(1364481363.243:24287): arch=c000003e syscall=2 success=no exit=-13 a0=7fffd19c5592 a1=0 a2=7fffd19c4b50 a3=a items=1 ppid=2686 pid=3538 auid=500 uid=500 gid=500 euid=500 suid=500 fsuid=500 egid=500 sgid=500 fsgid=500 tty=pts0 ses=1 comm="cat" exe="/bin/cat" subj=unconfined_u:unconfined_r:unconfined_t:s0-s0:c0.c1023 key="sshd_config""##;
    b.iter(|| tokenize(log));
}

#[bench]
fn bench_auditd_cwd_log(b: &mut Bencher) {
    let log = r##"type=CWD msg=audit(1364481363.243:24287):  cwd="/home/shadowman""##;
    b.iter(|| tokenize(log));
}

#[bench]
fn bench_auditd_path_log(b: &mut Bencher) {
    let log = r##"type=PATH msg=audit(1364481363.243:24287): item=0 name="/etc/ssh/sshd_config" inode=409248 dev=fd:00 mode=0100600 ouid=0 ogid=0 rdev=00:00 obj=system_u:object_r:etc_t:s0"##;
    b.iter(|| tokenize(log));
}

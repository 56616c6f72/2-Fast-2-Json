use criterion::{black_box, criterion_group, criterion_main, Criterion};
use twojson::run;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn generate_file() {
    let mut sample_data: String = "Time,Entry Location,Entry,Enabled,Category,Profile,Description,Signer,Company,Image Path,Version,Launch String,MD5,SHA-1,PESHA-1,PESHA-256,SHA-256,IMP".to_owned();
    let sample_data_content: &str = r#"20230106-150455,HKLM\System\CurrentControlSet\Control\Session Manager\BootExecute,,,"Boot Execute",System-wide,,,,,,,,,,,
19910522-071728,"HKLM\System\CurrentControlSet\Control\Session Manager\BootExecute","autocheck autochk *",enabled,"Boot Execute",System-wide,"Auto Check Utility","(Verified) Microsoft Windows","Microsoft Corporation","c:\windows\system32\autochk.exe",10.0.17763.1,"autocheck autochk *",990D01F2A6D10A33C382191A24BBAAAF,2E6C38958917FB86F09026D41337C7460EFBE5F5,26199469AC02EBCABAA9DC6AE5C54331B3F05AC6,E07FE812DBCEFDF0ECBB643B8C38F85DE61697B914649CFD8A44FDF31CF6FEAA,644417B839762A3325920A87C3D955CA974A4EC1D6F008216910267435921255,262DAC4DB20D08B06C59A7F5DBE43E61
20200929-171856,HKLM\SOFTWARE\Classes\Htmlfile\Shell\Open\Command\(Default),,,"Hijacks",System-wide,,,,,,,,,,,
19951211-041831,"HKLM\SOFTWARE\Classes\Htmlfile\Shell\Open\Command\(Default)","C:\Program Files\Internet Explorer\iexplore.exe",enabled,"Hijacks",System-wide,"Internet Explorer","(Verified) Microsoft Corporation","Microsoft Corporation","c:\program files\internet explorer\iexplore.exe",11.0.17763.1,"",EAC888C884C5AE875B16E8C714B4D2E6,021415D73D02C6247001BAD6E5C9BC6E220F34FC,386FC0D46EDCC2F75E213D56B449B538DE9A16F8,D62A6A7524CE9610AE65D21332C07583417A42D01BA4035FB9F73FB0915DBA24,FAA971F17A142EBD5518E5B3C55AF0F0C264ADF43644D8CCF972440048620D07,BF1B4238FCDBB117EDF39418CA0D205C
20230203-105249,HKLM\System\CurrentControlSet\Services,,,"Services",System-wide,,,,,,,,,,,"#;
    
    sample_data.push_str(&sample_data_content.repeat(1000));

    let mut file = File::create("tmp.csv").unwrap();
    file.write_all(sample_data.as_bytes()).unwrap();

}

fn run_benchmark(c: &mut Criterion) {   

    generate_file();
    c.bench_function(
        "Speed test 1k autoruns events",
        |b | b.iter(|| run(
            black_box("tmp.csv".to_string()),
            black_box("tmp.json".to_string()),
            black_box(",".to_string()),
            black_box(false))),
    );
    fs::remove_file("tmp.csv").unwrap();
    fs::remove_file("tmp.json").unwrap();
}

criterion_group!(benches, run_benchmark);
criterion_main!(benches);

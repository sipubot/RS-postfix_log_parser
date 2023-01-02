use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use std::io::BufReader;
use chrono::DateTime;

use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;
use std::io::prelude::*;
use flate2::read::GzDecoder;
use dateparser::DateTimeUtc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub from_log_path: String,
    pub to_log_path: String,
    pub job_log_path: String,
    pub header_key : String,
    pub delete_trigger_year: i16,
    pub job_period_second: i32,
    pub is_fail_set_period: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Loglist {
    pub path: String,
    pub read: bool,
    pub parse_data: String,
    pub stat: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  LogData {
    pub postfix_code: String,
    pub uuid: String,
    pub code: String,
    pub send_time: DateTime<Utc>,
    pub message: String,
    pub etc: String,
}

const CONFIG_PATH: &str = "./config.json";

fn main() {
    let mut _config : Config = serde_json::from_value(file_read_to_json(CONFIG_PATH).unwrap()).unwrap();

    println!("Hello, world!");
    //test
    let _str = "Aug 11 03:30:58 mail postfix/smtp[9944]: 8ECF240F31: to=<aa@ezweb.ne.jp>, relay=lsean.ezweb.ne.jp[27.85.176.228]:25, delay=0.1, delays=0.04/0/0.02/0.03, dsn=2.0.0, status=sent (250 Ok: queued as 9FFCB76)
    Aug 11 03:37:17 cst-postfix postfix/smtp[10305]: 4F64840DFC: to=<bb@yahoo.co.jp>, relay=mx2.mail.yahoo.co.jp[124.83.142.119]:25, delay=13, delays=0.05/0/0.03/13, dsn=2.0.0, status=sent (250 ok dirdel)
    Aug 10 03:44:46 cst-postfix postfix/smtp[14879]: E5D4840DA0: to=<cc@docomo.ne.je>, relay=docomo.ne.je[157.112.187.13]:25, delay=151541, delays=151540/0.03/0.12/0.5, dsn=4.7.1, status=deferred (host docomo.ne.je[157.112.187.13] said: 454 4.7.1 <cc@docomo.ne.je>: Relay access denied (in reply to RCPT TO command))
    Aug 10 04:10:15 cst-postfix postfix/smtp[16703]: 8495840E93: to=<dd@gmail.cm>, relay=none, delay=315562, delays=315532/0/30/0, dsn=4.4.1, status=deferred (connect to gmail.cm[172.217.25.69]:25: Connection timed out)
    Aug 10 08:23:14 cst-postfix postfix/smtp[2352]: 66C1E40D18: to=<ee@yahoo.co.jp>, relay=mx1.mail.yahoo.co.jp[202.93.66.124]:25, delay=10, delays=0.05/0/0.02/10, dsn=5.0.0, status=bounced (host mx1.mail.yahoo.co.jp[202.93.66.124] said: 554 delivery error: dd This user doesn't have a yahoo.co.jp account (ee@yahoo.co.jp) [-5] - mta086.mail.bbt.yahoo.co.jp (in reply to end of DATA command))
    Aug 11 10:35:20 cst-postfix postfix/smtp[9657]: 998864068E: to=<ff@ezweb.ne.jp>, relay=lsean.ezweb.ne.jp[27.85.176.228]:25, delay=29, delays=0.04/0/0.03/28, dsn=5.0.0, status=bounced (host lsean.ezweb.ne.jp[27.85.176.228] said: 550 : User unknown (in reply to end of DATA command))
    Aug 10 03:49:45 cst-postfix postfix/qmgr[2748]: 76DEF40E80: from=<>, size=2351, nrcpt=1 (queue active)
    Aug 10 03:50:15 cst-postfix postfix/smtp[15245]: connect to mail.gmail.ne.jp[160.16.210.40]:25: Connection timed outs";
    let mut _s_str :Vec<&str> = _str.lines().collect::<Vec<_>>();


}
pub fn processing_log_data(_exist_log : HashMap<String, LogData>, _key : String) -> HashMap<String, LogData> {
    //결과 전송 하면서 데이터 부족한건 남기고 나머지는 삭제
    //특정시간이상 초과되면 실패로 간주
    let mut _r:HashMap<String, LogData> = HashMap::new();
    _r
}

pub fn parse_log_data(_log_lines: Vec<&str>, _exist_log : HashMap<String, LogData>, _key : String) -> HashMap<String, LogData> {
    
    let mut _r:HashMap<String, LogData> = HashMap::new();
    //
    if _exist_log.len() > 0 {
        _r = _exist_log;
    }

    for _line in _log_lines.iter() {
        if _line.contains("info: header") && _line.contains(&_key) {
            //time
            let time_pars = _line.split(" cst-postfix").nth(0).unwrap().parse::<DateTimeUtc>();
            //id
            let postfix_id = _line.split("]: ").nth(1).unwrap().split(": ").nth(0).unwrap();
            //uuid
            let mut _s_key:String = String::from(&_key);
            _s_key.push_str(": ");

            let uuid = _line.split(&_s_key).nth(1).unwrap().split(" ").nth(0).unwrap();

            let _time = match time_pars {
                Ok(_ts) => _ts.0,
                Err(_) => chrono::offset::Utc::now()
            };

            if postfix_id.len() > 10 && !_r.contains_key(postfix_id) {
                let _new = LogData{ 
                    postfix_code: postfix_id.to_string(), 
                    uuid: uuid.to_string(), 
                    code: "".to_string(), 
                    send_time: _time, 
                    message: "".to_string(), 
                    etc:"".to_string()  
                };
                _r.insert(postfix_id.to_string(), _new);
            } else if _r.contains_key(postfix_id) {
                let before_date = &_r[postfix_id];
                *_r.get_mut(postfix_id).unwrap() = LogData {
                    uuid: uuid.to_string(),
                    send_time: _time,
                    ..before_date.clone()
                };
            }

        }
        if _line.contains("status=") {
            let postfix_id = _line.split("]: ").nth(1).unwrap().split(": ").nth(0).unwrap();
            let stat = _line.split("status=").nth(1).unwrap();
            let mut status = "undefined";
            let mut said = "";

            //ok
            if stat.starts_with("sent ") {
                status = "ok";
            }
            //??
            if stat.starts_with("bounce ") {
                status = "conflict";
            }
            //""
            if stat.starts_with("deferred") {
                status = "fail";
            }
            if _line.contains("said:") {
                said = _line.split("said:").nth(1).unwrap();
            }

            
            if postfix_id.len() > 10 && !_r.contains_key(postfix_id) {
                let _new = LogData{ 
                    postfix_code: postfix_id.to_string(), 
                    uuid: "".to_string(), 
                    code: status.to_string(), 
                    send_time: chrono::offset::Utc::now(), 
                    message: said.to_string(), 
                    etc:"".to_string()  
                };
                _r.insert(postfix_id.to_string(), _new);

            } else if _r.contains_key(postfix_id) {
                let before_date = &_r[postfix_id];
                *_r.get_mut(postfix_id).unwrap() = LogData {
                    code: status.to_string(),
                    message: said.to_string(),
                    ..before_date.clone()
                };
            }

            
        }

    }
    _r
}

pub fn file_read_unzip_to_json(_filepath: &str) -> Vec<LogData> { 
    let pathstring = _filepath;
    match fs::read_to_string(&pathstring) {
        Err(e) => {
            logger(&e.to_string());
            vec![] 
        }
        Ok(file) => {
            let mut byte = GzDecoder::new(file.as_bytes());
            let mut str = String::new();
            byte.read_to_string(&mut str).unwrap();
            let mut _str :Vec<&str> = str.lines().collect::<Vec<_>>();

            let mut _r:Vec<LogData> = vec![];
            _r
        },
    }
}

pub fn load_log(file_path:&str) -> Vec<String> {
    //println!("In file {}", file_path);
    let file = File::open(Path::new(file_path)).unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    return lines;
}

pub fn load_config(file_path:&str) -> Vec<String> {
    //println!("In file {}", file_path);
    let file = File::open(Path::new(file_path)).unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    return lines;
}

pub fn logger(_log: &str) {
    let filename = Utc::now().format("%Y-%m").to_string();
    let utc = Utc::now().format("%Y-%m-%d  %H:%M:%S").to_string();
    let pathstr = format!("./log/log{}.log", filename);
    if !folder_exist(&pathstr) {
        File::create(&pathstr).unwrap();
    }
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(pathstr)
        .unwrap();

    if let Err(e) = writeln!(file, "[{}]:{}\n", utc, _log) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn json_re_parse(_res: &str) -> Value {
    json!({ "result": _res })
}

pub fn folder_exist(_path: &str) -> bool {
    let path = Path::new(&_path);
    path.exists()
}

pub fn file_read_to_json(_filepath: &str) -> serde_json::Result<Value> {
    let pathstring = _filepath;
    match fs::read_to_string(&pathstring) {
        Err(e) => {
            logger(&e.to_string());
            Ok(json_re_parse(&e.to_string()))
        }
        Ok(file) => serde_json::from_str(&*file),
    }
}

pub fn file_save_from_json(_filepath: &str, _v: &Value) -> serde_json::Result<bool> {
    let path = Path::new(&_filepath);
    let json = serde_json::to_string(_v).unwrap();
    match File::create(&path) {
        Err(e) => {
            logger(&e.to_string());
            Ok(false)
        }
        Ok(mut file) => match file.write_all(&json.as_bytes()) {
            Err(e) => {
                logger(&e.to_string());
                Ok(false)
            }
            Ok(_) => Ok(true),
        },
    }
}
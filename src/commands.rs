use redis::Commands;
use models::{PomoScore, PomoScoreOld};
use std::env;
use serde_json;
use redis;
use getopts::Options;
use regex::Regex;

fn get_key(user_id: &String) -> String {
    format!("{}_pomo", user_id)
}

pub fn redis_con() -> redis::RedisResult<redis::Connection> {
    let mut redis_host = env::var("REDIS_HOST").unwrap_or("127.0.0.1".to_string());
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or("".to_string());
    if redis_password.len() > 0 {
        redis_host = format!(":{}@{}", redis_password, redis_host);
    }
    let redis_address = format!("redis://{}/", redis_host);
    let client = redis::Client::open(&*redis_address)?;
    let con = client.get_connection()?;
    Ok(con)
}

pub fn init_pomo(user_id: String) -> redis::RedisResult<()> {
    let con = redis_con()?;
    let key = get_key(&user_id);
    let score = PomoScore { remaining: 8, ..PomoScore::blank_score() };
    let _: () = con.set(key.to_string(), serde_json::to_string(&score).unwrap())?;
    Ok(())
}

pub fn get_or_create_pomo(user_id: String) -> redis::RedisResult<PomoScore> {
    let con = redis_con()?;
    let key = get_key(&user_id);
    let exists: bool = con.exists(key.to_string())?;
    if !exists {
        let _ = init_pomo(user_id);
    }
    let score = get_pomo_from_redis(&con, key);
    Ok(score)
}

pub fn set_pomo(user_id: String, count: i32) -> redis::RedisResult<PomoScore> {
    let con = redis_con()?;
    let key = get_key(&user_id);
    let score = get_or_create_pomo(user_id)?;
    let new_score = PomoScore {
        remaining: count,
        histories: vec![],
        ..score
    };
    let _: () = con.set(key, serde_json::to_string(&new_score).unwrap())?;
    Ok(new_score)
}

pub fn set_remaining(user_id: String, count: i32) -> redis::RedisResult<PomoScore> {
    let con = redis_con()?;
    let key = get_key(&user_id);
    let score = get_or_create_pomo(user_id)?;
    let new_score = PomoScore { remaining: count, ..score };
    let _: () = con.set(key, serde_json::to_string(&new_score).unwrap())?;
    Ok(new_score)
}

pub fn done_pomo(user_id: String, options: &str) -> redis::RedisResult<PomoScore> {
    let mut optsfmt = Options::new();
    optsfmt.optopt("m", "comment", "set comment", "COMMENT");
    optsfmt.optopt("p", "point", "set point", "POINT");
    let re_words = Regex::new(r#"(["“](?P<word1>[^"]+)["”])|(?P<word2>\S+)"#).unwrap();
    let parsed_opts = match optsfmt.parse(re_words.captures_iter(options)
        .map(|caps| {
            caps.name("word1").map_or(caps.name("word2").map_or("", |c| c.as_str()),
                                      |c| c.as_str())
        })
        .collect::<Vec<&str>>()) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    let comment = parsed_opts.opt_str("m").unwrap_or("".to_string());
    let point = parsed_opts.opt_str("p").unwrap_or("".to_string());
    let con = redis_con()?;
    let key = get_key(&user_id);
    let score = get_or_create_pomo(user_id)?;
    if score.remaining > 0 {
        let new_score = score.done(comment, point);
        let _: () = con.set(key.to_string(), serde_json::to_string(&new_score).unwrap())?;
        Ok(new_score)
    } else {
        Ok(score)
    }
}

pub fn show_pomo(user_id: String) -> redis::RedisResult<PomoScore> {
    let score = get_or_create_pomo(user_id)?;
    Ok(score)
}

pub fn set_tomato_emoji(user_id: String, emoji: &str) -> redis::RedisResult<()> {
    let con = redis_con()?;
    let key = get_key(&user_id);
    let score = get_or_create_pomo(user_id)?;
    let new_score = score.set_tomato_emoji(emoji);
    let _: () = con.set(key.to_string(), serde_json::to_string(&new_score).unwrap())?;
    Ok(())
}

pub fn set_icon_emoji(user_id: String, emoji: &str) -> redis::RedisResult<()> {
    let con = redis_con()?;
    let key = get_key(&user_id);
    let score = get_or_create_pomo(user_id)?;
    let new_score = score.set_icon_emoji(emoji);
    let _: () = con.set(key.to_string(), serde_json::to_string(&new_score).unwrap())?;
    Ok(())
}


pub fn get_all_pomos() -> redis::RedisResult<Vec<PomoScore>> {
    let con = redis_con()?;
    let keys: Vec<String> = con.keys("*_pomo")?;
    let scores = keys.iter()
        .map(|key| get_pomo_from_redis(&con, key.clone()))
        .collect();
    Ok(scores)
}

fn get_pomo_from_redis(con: &redis::Connection, key: String) -> PomoScore {
    match con.get::<String, String>(key) {
        Ok(val) => {
            match serde_json::from_str(&*val) {
                Ok(value) => value,
                _ => {
                    match serde_json::from_str::<PomoScoreOld>(&*val) {
                        Ok(value) => value.convert_to_new(),
                        _ => PomoScore::blank_score(),
                    }
                }
            }
        }
        _ => PomoScore::blank_score(),

    }
}

#[cfg(test)]
mod tests {
    use super::set_pomo;
    use super::done_pomo;
    use super::show_pomo;
    use super::set_icon_emoji;
    use super::set_tomato_emoji;
    use models::PomoHistory;

    #[test]
    fn can_set_pomo() {
        let res = set_pomo("test_pomo_user".to_string(), 8);
        assert_eq!(8, res.unwrap().remaining);
    }

    #[test]
    fn can_done_pomo() {
        let u = "test_pomo_user_done".to_string();
        let _ = set_pomo(u.clone(), 8);
        let res = done_pomo(u.clone(), "-m test -p 1").unwrap();
        assert_eq!(7, res.remaining);
        assert_eq!(PomoHistory {
                       comment: "test".to_string(),
                       point: "1".to_string(),
                   },
                   res.histories[0]);
    }

    #[test]
    fn can_set_icon_emoji() {
        let u = "test_pomo_user_icon_emoji".to_string();
        let _ = set_pomo(u.clone(), 8);
        let _ = set_icon_emoji(u.clone(), ":icon_test:");
        let res = show_pomo(u.clone());
        assert_eq!(":icon_test:", res.unwrap().icon_emoji);
    }

    #[test]
    fn can_set_tomato_emoji() {
        let u = "test_pomo_user_tomato_emoji".to_string();
        let _ = set_pomo(u.clone(), 8);
        let _ = set_tomato_emoji(u.clone(), ":tomato_test:");
        let res = show_pomo(u.clone());
        assert_eq!(":tomato_test:", res.unwrap().tomato_emoji);
    }
}

use redis::Commands;
use models::PomoScore;
use std::env;
use serde_json;
use redis;

fn get_key(user_id: &String) -> String {
    format!("{}_pomo", user_id)
}

pub fn redis_con() -> redis::RedisResult<redis::Connection> {
    let redis_host = env::var("REDIS_HOST").unwrap_or("127.0.0.1".to_string());
    let redis_address = format!("redis://{}/", redis_host);
    let client = redis::Client::open(&*redis_address)?;
    let con = client.get_connection()?;
    Ok(con)
}

pub fn init_pomo(user_id: String) -> redis::RedisResult<()> {
    let con = redis_con()?;
    let key = get_key(&user_id);
    let score = PomoScore { remaining: 8, done: 0, tomato_emoji: ":tomato:".to_string(), icon_emoji:  "".to_string()} ;
    let _ : () = con.set(key.to_string(), serde_json::to_string(&score).unwrap())?;
    Ok(())
}

pub fn get_or_create_pomo(user_id: String) -> redis::RedisResult<PomoScore> {
    let con = redis_con()?;
    let key = get_key(&user_id);
    let exists: bool = con.exists(key.to_string())?;;
    if !exists {
        let _ = init_pomo(user_id);
    }
    let score_json: String = con.get(key.to_string())?;
    let score = serde_json::from_str(&*score_json).unwrap();
    Ok(score)
}

pub fn set_pomo(user_id: String, count: i32) -> redis::RedisResult<PomoScore> {
    let con = redis_con()?;
    let key = get_key(&user_id);
    let score = get_or_create_pomo(user_id)?;
    let new_score = PomoScore { remaining: count, done: 0, tomato_emoji: score.tomato_emoji, icon_emoji: score.icon_emoji } ;
    let _ : () = con.set(key, serde_json::to_string(&new_score).unwrap())?;
    Ok(new_score)
}

pub fn set_remaining(user_id: String, count: i32) -> redis::RedisResult<PomoScore> {
    let con = redis_con()?;
    let key = get_key(&user_id);
    let score = get_or_create_pomo(user_id)?;
    let new_score = PomoScore { remaining: count, done: score.done, tomato_emoji: score.tomato_emoji, icon_emoji: score.icon_emoji } ;
    let _ : () = con.set(key, serde_json::to_string(&new_score).unwrap())?;
    Ok(new_score)
}

pub fn done_pomo(user_id: String) -> redis::RedisResult<PomoScore> {
    let con = redis_con()?;
    let key = get_key(&user_id);
    let score = get_or_create_pomo(user_id)?;
    if score.remaining > 0 {
        let new_score = score.done();
        let _ : () = con.set(key.to_string(), serde_json::to_string(&new_score).unwrap())?;
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
    let _ : () = con.set(key.to_string(), serde_json::to_string(&new_score).unwrap())?;
    Ok(())
}    

pub fn set_icon_emoji(user_id: String, emoji: &str) -> redis::RedisResult<()> {
    let con = redis_con()?;
    let key = get_key(&user_id);
    let score = get_or_create_pomo(user_id)?;
    let new_score = score.set_icon_emoji(emoji);
    let _ : () = con.set(key.to_string(), serde_json::to_string(&new_score).unwrap())?;
    Ok(())
}    



use std::str::FromStr;

#[allow(dead_code)]
#[derive(FromForm)]
pub struct SlashParams {
    pub token: String,
    pub text: String,
    pub channel_id: String,
    pub team_id: String,
    pub team_domain: String,
    pub channel_name: String,
    pub user_id: String,
    pub user_name: String,
    pub command: String,
    pub response_url: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PomoScore {
    pub remaining: i32,
    pub tomato_emoji: String,
    pub icon_emoji: String,
    pub histories: Vec<PomoHistory>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PomoHistory {
    pub comment: String,
    pub point: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PomoScoreOld {
    pub remaining: i32,
    pub done: i32,
    pub tomato_emoji: String,
    pub icon_emoji: String,
}

impl PomoScoreOld {
    pub fn convert_to_new(&self) -> PomoScore {
        PomoScore {
            remaining: self.remaining,
            tomato_emoji: self.tomato_emoji.clone(),
            icon_emoji: self.icon_emoji.clone(),
            histories: PomoHistory::blank_histories(self.done)
        }
    }
}

impl PomoHistory {
    fn blank_histories(size: i32) -> Vec<PomoHistory> {
        let mut histories = Vec::with_capacity(size as usize);
        for _ in 0..size {
            histories.push(PomoHistory {comment: "".to_string(), point: "".to_string()});
        }
        return histories;
    }
}

impl PomoScore {
    pub fn show_remaining(&self) -> String {
        self.show_emoji_by(self.remaining)
    }

    pub fn get_done(&self) -> i32 {
        self.histories.len() as i32
    }

    pub fn show_detail(&self) -> String {
        format!(
            "remaining: {}\ndone: {}\ntomato_emoji: {}\nicon_emoji: {}\nhistories:\n```{}\n```",
            self.show_emoji_by(self.remaining),
            self.show_emoji_by(self.get_done()),
            self.tomato_emoji,
            self.icon_emoji,
            self.show_histories()
        )
    }

    pub fn done(&self, comment: String, point: String) -> PomoScore {
        let mut histories = self.histories.clone();
        histories.push( PomoHistory { comment: comment, point: point } );
        PomoScore { remaining: self.remaining - 1, histories: histories, .. self.clone() }
    }

    pub fn set_tomato_emoji(&self, emoji: &str) -> PomoScore {
        PomoScore { tomato_emoji: emoji.to_string(), .. self.clone() }
    }
    
    pub fn set_icon_emoji(&self, emoji: &str) -> PomoScore {
        PomoScore { icon_emoji: emoji.to_string(), .. self.clone() }
    }

    pub fn blank_score() -> PomoScore {
        PomoScore { remaining: 0, tomato_emoji: "".to_string(), icon_emoji:  "".to_string(), histories: vec!() }
    }

    fn show_emoji_by(&self, count: i32) -> String {
        let mut result = self.icon_emoji.clone();
        for _ in 0..count {
            result = format!("{} {}", result, self.tomato_emoji);
        }
        result
    }

    fn show_histories(&self) -> String {
        let mut hst = "".to_string();
        let mut sum = 0;
        let mut real_len = 0;
        for h in self.histories.clone() {
            let _p = if h.point.len() > 0 { h.point.clone() } else { "-".to_string() };
            let _c = if h.comment.len() > 0 { h.comment } else { "--".to_string() };
            if h.point.len() > 0 {
                sum += match i32::from_str(&*h.point) {
                    Ok(n) => n,
                    _ => 0
                };
                real_len += 1;
            }
            hst = format!("{}\n{}: {}", hst, _p, _c)
        }
        let average = if real_len == 0 { 0.0 } else { (((sum as f32 / real_len as f32) * 10.0).round() / 10.0) };
        hst = format!("{}\n===\naverage: {}", hst, average);
        hst
    }

}

#[derive(Serialize, Debug)]
pub struct Message {
    pub response_type: String,
    pub text: String
}

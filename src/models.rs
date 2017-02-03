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

#[derive(Serialize, Deserialize, Debug)]
pub struct PomoScore {
    pub remaining: i32,
    pub done: i32,
    pub tomato_emoji: String,
    pub icon_emoji: String
}

impl PomoScore {
    pub fn show_remaining(&self) -> String {
        self.show_emoji_by(self.remaining)
    }

    pub fn show_detail(&self) -> String {
        format!("remaining: {}\ndone: {}\ntomato_emoji: {}\nicon_emoji: {}\n", self.show_emoji_by(self.remaining), self.show_emoji_by(self.done), self.tomato_emoji, self.icon_emoji)
    }

    pub fn done(&self) -> PomoScore {
        PomoScore { remaining: self.remaining - 1, done: self.done + 1, tomato_emoji: self.tomato_emoji.clone(), icon_emoji: self.icon_emoji.clone() }
    }

    pub fn set_tomato_emoji(&self, emoji: &str) -> PomoScore {
        PomoScore { remaining: self.remaining, done: self.done, tomato_emoji: emoji.to_string(), icon_emoji: self.icon_emoji.clone() }
    }
    
    pub fn set_icon_emoji(&self, emoji: &str) -> PomoScore {
        PomoScore { remaining: self.remaining, done: self.done, tomato_emoji: self.tomato_emoji.clone(), icon_emoji: emoji.to_string() }
    }

    fn show_emoji_by(&self, count: i32) -> String {
        let mut result = self.icon_emoji.clone();
        for _ in 0..count {
            result = format!("{} {}", result, self.tomato_emoji);
        }
        result
    }
}

#[derive(Serialize, Debug)]
pub struct Message {
    pub response_type: String,
    pub text: String
}

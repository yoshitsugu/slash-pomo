# `/pomo` command
[![Build Status](https://travis-ci.org/yoshitsugu/slash-pomo.svg?branch=master)](https://travis-ci.org/yoshitsugu/slash-pomo)
Count down pomodoro by Slack Slash Command

## What is this?
This is a web server for Slack Slash Command. It provides counting your daily pomodoro. You can set the number of pomodoros, record the pomodoro with comment and self evaluated point, and review the finished pomodoros in the end of the day.

## Commands
```
/pomo [show/s]: show detail
/pomo [done/d]: pomo - 1
/pomo [reset/r] (count=8): reset count (default 8)
/pomo [reset remaining/rr] (count=8): reset only remaining count (default 8)
/pomo [set tomato/st] (emoji): set alternative :tomato: emoji
/pomo [set icon/si] (emoji): set alternative icon emoji
```

<img src="https://github.com/yoshitsugu/slash-pomo/blob/master/images/pomo_example.jpg" alt="slash pomo command" width="300" >

## How to use?
You need Rust environment and redis.  
I recommend to use [IBM Bluemix](https://console.bluemix.net/registration/).  
This repository contains manifest.yml to deploy to Bluemix.  
  
Once you could deploy the server and prepared redis, you need to introduce Slash Command in your Slack team. Please see [here](https://api.slack.com/slash-commands)

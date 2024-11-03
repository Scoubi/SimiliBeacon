# Living Of the Web Beacon Simulator

## What is it?
This Rust program emulates the configuration (commands to run) retreival.  
The idea is to use a known good website (such as GitHub) and use it to store and retreive the configuration for the Nodes. 

By defaut 
- The time between each pull is **60** minutes with a **100%** jitter.
  - To change the jitter change the number accordingly. For example a 30% jitter would be (0.7,1.3) a 50% jitter would be (0.5,1.50)
- https://github.com/microsoft/vscode is used as the base URL
- There's a list of 10 files the Node will cycle through to mimic the C2 behavior of changing URL for each new sets of commands
- The number of time each URL is fetch is randomly selected between 1 and 10

All of these are easily configurable in the first part of the code. 

## Why did I release this?
This software aims to help Defenders (Blue Teamers) to test their beaconing detections if it occurs on "trusted" sites.  
As mentioned, it's easy to change the URL to test against 
- Azure VM IP Space
- Gitlab
- Wordpress Sites
- Amazon (hiding C2 in comments?)
- AWS IP Space
- CloudFlare
- Internal sites such as SharePoint
- Teams
- O365
- etc.  

# Jiradoro

An app that saves Pomodoro work segments into a Jira account as time spent.

## Sources

- Pomodoro App: [https://medium.com/@maxjt11/create-a-full-stack-rust-desktop-app-with-tauri-yew-and-tailwind-css-694fc74281b3]
- Custom Events: [https://bitbucket.org/ftegtmeyer/tauri-yew-stopwatch]

## TODO

- Login button

  - [x] Click on profile writes to the console
  - [x] Click dispatches to the server. Server logs to the Tauri console
  - [x] Click returns an ack(UUID) to the front end and writes that to the server's log
  - [x] Add a root listener to the client for LongRunner events
  - [x] Tauri should emit an event that could be heard by the root listener asynchronously after
        replying to the initial call

- LongRunner

  - [ ] Make a trait to describe an implemented process
  - [ ] Add the process to the queue
  - [ ] If the queue is empty, start the process running
  - [ ] Initialize the channels
  - [ ] Heartbeat should emit a string every second saying it is alive

- [ ] Make a heartbeat
  - [ ] Client GUI display
    - [ ] One line to display the latest count from the server heartbeat
    - [ ] One line to display the last time the heartbeat was received
  - [ ] Make a timer for the time since the last message was received.
    - [ ] Every second it should update the page.
    - [ ] To start, it should never reset
    - [ ] Should reset every time the counter is updated
  - [ ] Call to the LongRunner - command should be StartHeartbeat
    - [ ] Store the UUID returned in the app state
  - [ ] Add a listener for the Heartbeat emission
  - [ ] Forward emissions for the Heartbeat UUID to the proper callback

# Jiradoro

An app that saves Pomodoro work segments into a Jira account as time spent.

## TODO

- Login button
  - Click on profile writes to the console
  - Click dispatches to the server. Server logs to the Tauri console
  - Click returns an ack(UUID) to the front end and writes that to the server's log
  - Add a root listener to the client for LongRunner events
  - Tauri should emit an event that could be heard by the root listener asynchronously after
    replying to the initial call

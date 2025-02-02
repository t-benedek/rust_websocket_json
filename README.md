# Easy example how to use rust with websockets and JSON
This websocket client connects to the echo websocket server hosted at https://echo.websocket.org/. It sends predefined JSON data and reads it back from the echo websocket server. 
Finally it parses the JSON data structure and prints its fields. 

# Challenges and Lessons Learned
1. How to easily make use of https://github.com/snapview/tungstenite-rs including the TLS feature
2. How to make use of https://github.com/nox/serde_json to parse JSON data structures

# How to use
Just build and run using 'cargo run'.

use serde_json::Value;
use tungstenite::{connect, Message};

static ECHO_WEBSERVER: &str = "wss://echo.websocket.org";

fn main() {
    // create a JSON data strucutre. Using rust RAW strings
    // for more detail refer to: https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
    let json_data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // use tungsenite client connect to call the echo server
    // TLS (wss) is activated using the feature "rustls-tls-native-roots" in the TOML file
    let (mut socket, _) = connect(ECHO_WEBSERVER)
        .expect("Can't connect to echo server");
        
    // throw away first message from echo server saying that connection was established. We only need the echo messages.
    socket.read().unwrap();
    socket.send(Message::Text(json_data.into()))
        .expect("Error sending message");
    let result  = socket.read()
        .expect("Error reading message");
    
    // parse JSON data using serde and printing dedicated fields of the JSON data structure
    if result.is_text() {
        let v: Value = serde_json::from_str(&result.to_text().unwrap()).unwrap();
        println!("\nEste hombre {} tiene {} anos. Su numero de telefone es {}\n", v["name"], v["age"], v["phones"][0]);
    }
}
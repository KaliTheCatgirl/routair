use std::{
    error::Error,
    io::Write,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use futures_util::{SinkExt, StreamExt};
use openai::{
    Credentials,
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
};
use tokio::{
    select,
    sync::{
        Mutex,
        mpsc::{self},
    },
};
use warp::{Filter, filters::ws::Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // http entry point subsystem
    let (ws_tx, mut ws_rx) = mpsc::channel(8);
    tokio::spawn(
        warp::serve(
            warp::path("feed")
                .and(warp::ws())
                .map(move |ws: warp::ws::Ws| {
                    let ws_tx = ws_tx.clone();
                    ws.on_upgrade(async move |ws| {
                        let _ = ws_tx.send(ws).await;
                    })
                })
                .or(warp::fs::dir(".")),
        )
        .run(([127, 0, 0, 1], 8228)),
    );

    // atomic socket registration subsystem
    let registered_new_socket = Arc::new(AtomicBool::new(true));
    let (new_socket_tx, mut new_socket_rx) = mpsc::channel::<()>(1);
    let (socket_tx, socket_rx) = (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    let (tx, rx) = (socket_tx.clone(), socket_rx.clone());
    tokio::spawn(async move {
        while let Some(ws) = ws_rx.recv().await {
            let (tx_local, rx_local) = ws.split();
            println!("new socket");
            new_socket_tx.send(()).await.unwrap();
            registered_new_socket.store(false, Ordering::SeqCst);
            *tx.lock().await = Some(tx_local);
            *rx.lock().await = Some(rx_local);
            registered_new_socket.store(true, Ordering::SeqCst);
        }
    });

    // packet request handler subsystem
    let (want_packet_tx, mut want_packet_rx) = mpsc::channel::<()>(1);
    let rx = socket_rx.clone();
    tokio::spawn(async move {
        loop {
            let mut guard = rx.lock().await;
            let Some(rx) = guard.as_mut() else {
                tokio::task::yield_now().await;
                continue;
            };
            select! {
                _ = rx.next() => {
                    want_packet_tx.send(()).await.unwrap();
                }
                _ = new_socket_rx.recv() => {
                    drop(guard);
                }
            };
        }
    });

    // packet analysis and user feedback using basic discriminators (0 = pcap output, 1 = analysis) for binary packet type differentiation subsystem
    let packet = Arc::new(Mutex::new(None));
    let sender_packet = packet.clone();
    let (tx, rx) = (socket_tx.clone(), socket_rx.clone());
    tokio::spawn(async move {
        'send_loop: loop {
            want_packet_rx.recv().await;
            let mut packet;
            loop {
                let Some(new_packet): Option<Vec<u8>> = sender_packet.lock().await.take() else {
                    tokio::task::yield_now().await;
                    continue;
                };
                packet = new_packet;
                break;
            }
            let mut result;
            'check: {
                if let Some(socket) = tx.lock().await.as_mut() {
                    let packet_string = packet
                        .iter()
                        .copied()
                        .map(|byte| format!("{byte:02}"))
                        .collect::<Vec<_>>()
                        .join(" ");
                    packet.insert(0, 0);
                    result = socket.send(Message::binary(packet)).await;
                    if result.is_err() {
                        break 'check;
                    }
                    println!("qwerty");

                    let fake = true;
                    let mut packet = vec![1u8];
                    if fake {
                        tokio::time::sleep(Duration::from_secs_f64(fastrand::f64() * 3.0 + 4.0))
                            .await;
                        packet
                            .write_all(if fastrand::bool() { b"YES" } else { b"NO" })
                            .unwrap();
                    } else {
                        let messages = vec![
                        ChatCompletionMessage {
                            role: ChatCompletionMessageRole::System,
                            content: Some("You are a packet analyser. You will be given a packet, and you will respond with \"YES\" if the packet is a valid and important internet packet, and \"NO\" if the packet is invalid.".to_owned()),
                            ..Default::default()
                        },
                        ChatCompletionMessage {
                            role: ChatCompletionMessageRole::User,
                            content: Some(format!("Packet data: {packet_string}")),
                            ..Default::default()
                        },
                    ];
                        let credentials = Credentials::from_env();
                        println!("asking");

                        let completion = ChatCompletion::builder(
                            "gpt-5-nano",
                            messages.into_iter().collect::<Vec<_>>(),
                        )
                        .credentials(credentials.clone())
                        .create()
                        .await
                        .unwrap();
                        println!("asked");

                        let completion = completion;
                        let message = completion
                            .choices
                            .first()
                            .unwrap()
                            .message
                            .content
                            .as_ref()
                            .unwrap();
                        println!("\"{message}\"");
                        if let Some(usage) = completion.usage {
                            println!("- completion: {}", usage.completion_tokens);
                            println!("- prompt: {}", usage.prompt_tokens);
                            println!("- tokens: {}", usage.total_tokens);
                        }
                        packet.write_all(message.as_bytes()).unwrap();
                    }
                    println!("sending");
                    result = socket.send(Message::binary(packet)).await;
                    if result.is_err() {
                        println!("nope");
                        break 'check;
                    }
                    println!("sent");
                } else {
                    continue 'send_loop;
                }
            }
            if result.is_err() {
                *tx.lock().await = None;
                *rx.lock().await = None;
            }
            while let Ok(_) = want_packet_rx.try_recv() {}
        }
    });

    println!("Hosting on 127.0.0.1:8228");

    // libpcap main thread process loop
    let mut capture_number = 0;
    loop {
        capture_number += 1;
        println!("starting capture #{capture_number}");
        let mut capture = pcap::Capture::from_device("any")?
            .immediate_mode(true)
            .open()?;
        while let Ok(new_packet) = capture.next_packet() {
            if !(512..=1024).contains(&new_packet.data.len()) {
                continue;
            }
            *packet.lock().await = Some(new_packet.to_vec());
        }
    }
}

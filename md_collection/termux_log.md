~/termux_stt $ make run
cargo run --
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.60s
     Running `target/debug/termux_stt`
🎤 Welcome to Termux STT Professional Pipeline!
Current AI Engine: OFFLINE (Local Llama)
✔ What would you like to do? · Transcribe Audio File
✔ Choose a file · It_that_your_real_hair.mp3
📂 Transcription results already exist at "./output/It_that_your_real_hair.txt"
✔ Skip transcription and use existing results? · yes
⏩ Skipping transcription...
✔ Would you like to translate and generate audio? · yes
🚀 Starting local AI Engine (llama-server)...
⏳ Waiting for AI Engine to wake up...
🌐 [2/3] Translating 1 batches to Myanmar...
2026-04-17T02:23:53.581935Z  WARN termux_stt::services::translator: Batch translation failed via local Llama: 1 - {"error":{"message":"Loading model","type":"unavailable_error","code":503}}

# Offline Translation using Gemma (4-bit Quantization)

Based on your system specifications (ThinkPad X280 with 8GB RAM, i7-8550U, and Intel UHD Graphics) and your interest in Termux (Android), here is a strategic breakdown for implementing offline translation using Gemma models (specifically 4-bit quantized versions to manage your memory).

## 1. Hardware Feasibility

### On Your ThinkPad X280 (Ubuntu)
*   **Memory Constraints:** You have 8GB of RAM. Unquantized Large Language Models (LLMs) take up massive amounts of memory. However, **4-bit quantized models (.gguf)** drastically reduce this. 
    *   A **Gemma 2B (4-bit)** uses roughly **1.6 GB** of memory. It will run very comfortably.
    *   A **Gemma 7B/9B (4-bit)** uses around **5.5 GB** of memory. This will fit, but leaving only ~2.5GB for your Ubuntu OS, browsing, and Termux STT background processing. It might cause the system to swap to the disk and slow down.
*   **Compute (CPU vs GPU):** Since you only have Intel UHD Graphics 620, you do not have dedicated GPU VRAM (like an NVIDIA card). You will be completely reliant on **CPU Inference**. The i7-8550U has 4 physical cores, which means inference will be relatively dependent on raw CPU threading.

### On Termux (Android smartphone)
*   Most modern smartphones have ARM processors (Snapdragon, MediaTek) with 6GB to 12GB of RAM. 
*   Because mobile CPUs are heavily optimized for power/efficiency, inferencing on mobile natively with Termux is actually very viable if you stick to **Gemma 2B (4-bit)**. 

---

## 2. Recommended AI Engine: `llama.cpp`

The standard way to run models in purely CPU environments (and inside Android Termux) is via **[`llama.cpp`](https://github.com/ggerganov/llama.cpp)**.

1.  It is written in highly optimized C/C++ which compiles trivially on both Termux (`pkg install clang make`) and Ubuntu.
2.  It uses `.gguf` weight formats, which perfectly supports 4-bit Gemma models.

### Integrating it into `termux_stt`
Instead of using the online `GeminiClient`, you have two architectural choices for offline integration:

**Method A: The Local Server approach (Recommended & Easiest)**
1.  You run `llama-server` (an included binary in `llama.cpp`) in the background. This exposes an API on `http://localhost:8080` that perfectly mimics the OpenAI interface.
2.  In `termux_stt`, you write a `LocalLlmClient` that simply makes `reqwest` HTTP calls to this local server, giving it the English prompt and extracting the Myanmar outputs.

**Method B: The Native Rust approach (Advanced)**
1.  Instead of a server, you tightly integrate it using pure Rust crates like **`candle`** (by HuggingFace) or **`llama_cpp_rs`**. 
2.  This means your `make run` compiles the AI inference directly into the app memory. However, compiling heavy ML libraries on Termux inside Android can be highly error-prone and take a very long time.

---

## 3. The Prompting Strategy

Generalist LLMs like Gemma need firm instructions to act purely as translators. Your system prompt would need to be structured heavily to prevent conversational bleeding.

```text
System: You are an expert English to Myanmar translator. You must only output the direct Myanmar translation of the provided English text, without any explanations, conversational filler, or formatting.

English: <Your text here>
Myanmar: 
```

## 4. Next Steps to Build This

If you want to move forward with adding an offline translation option, the roadmap would look like this:

1.  **Download Model Weights:** Download `gemma-2b-it-Q4_K_M.gguf` from HuggingFace.
2.  **Add Offline Configuration:** Update `Config` and the UI to let the user select between "Gemini API (Cloud)" or "Gemma (Offline Local)". Include a `local_model_path` in `config.toml`.
3.  **Create the LocalClient:** Write a new struct in `src/services/translator.rs` that interacts with a local instance of the model.

Let me know if you would like me to draft an implementation plan to integrate the offline translator natively into the current Rust application!

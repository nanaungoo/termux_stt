# Learning Log (သင်ယူမှုမှတ်တမ်း)

## [2026-04-13] Instruction Set Architecture (ISA) - x86 vs ARM
### The "Source of Truth": GEMINI.md
- **English:** Every professional project needs a "Constitution". We established `.gemini/GEMINI.md` as our foundational mandate. It defines the persona (Senior Rust Architect), the strict tech stack (Rust 2024, Miette, Tokio), and the bilingual communication protocol. This ensured that every piece of code and every explanation stayed consistent and high-quality throughout the journey.
- **Myanmar:** ကျွမ်းကျင်အဆင့်မီ ပရောဂျက်တိုင်းတွင် "အခြေခံဥပဒေ" တစ်ခု လိုအပ်ပါသည်။ ကျွန်ုပ်တို့သည် `.gemini/GEMINI.md` ကို ပရောဂျက်၏ အဓိကလမ်းညွှန်ချက်အဖြစ် သတ်မှတ်ခဲ့ပါသည်။ ၎င်းသည် အသုံးပြုမည့် နည်းပညာများ (Rust 2024, Miette, Tokio)၊ ကုဒ်ရေးသားပုံ စံနှုန်းများနှင့် နှစ်ဘာသာဖြင့် ရှင်းလင်းတင်ပြရမည့် စည်းမျဉ်းများကို သတ်မှတ်ပေးပါသည်။ ဤဖိုင်ကြောင့် ကျွန်ုပ်တို့၏ လုပ်ငန်းစဉ်တစ်ခုလုံးသည် စံနှုန်းမီပြီး တစ်ပြေးညီ ဖြစ်နေခြင်း ဖြစ်ပါသည်။

### Learning Moment
- **English:** Think of x86 (Ubuntu/Laptop) as a high-end restaurant kitchen with every specialized tool imaginable (Complex Instruction Set). ARM (Termux/Mobile) is like a sleek, efficient food truck; it has fewer, simpler tools but is optimized for speed and low fuel (power) consumption (Reduced Instruction Set). To run our "recipe" (code) in both, we must ensure our "ingredients" (libraries) are prepared for both types of kitchens.
- **Myanmar:** x86 (Ubuntu/Laptop) ကို လိုအပ်တဲ့ ကိရိယာအစုံအလင်ရှိတဲ့ စားသောက်ဆိုင်ကြီးတစ်ဆိုင်ရဲ့ မီးဖိုချောင်လို့ မြင်ကြည့်ပါ။ ARM (Termux/Mobile) ကတော့ လျင်မြန်ပြီး ဆီစားသက်သာတဲ့ (ပါဝါသုံးစွဲမှုနည်းတဲ့) Food Truck လေးနဲ့ တူပါတယ်။ ကျွန်တော်တို့ရဲ့ "ဟင်းချက်နည်း" (Code) ကို မီးဖိုချောင် နှစ်မျိုးလုံးမှာ ချက်လို့ရအောင် "ပါဝင်ပစ္စည်း" (Libraries) တွေကို ကြိုတင်ပြင်ဆင်ထားဖို့ လိုပါတယ်။

### Command Explanations
- **Native Termux Build:** `pkg install rust && cargo build --release` (Compile directly on ARM).
- **Cross-Compilation (Ubuntu to Android):** `cargo build --target aarch64-linux-android --release` (Baking the ARM cake in the x86 kitchen).

### Architecture Decisions
- Adopted **Library-First** approach: Core logic in `src/lib.rs`.
- Target: Ubuntu (x86_64) and Termux (aarch64).
- Optimization: Enabled `panic = "abort"` and `strip = true` for smaller binaries.

## [2026-04-13] Library-First Refactor & CLI Update
### Learning Moment
- **English:** In Rust, a `lib.rs` file allows other programs to use your logic as a crate. By moving our STT engine there, we make our tool "reusable." The `main.rs` then becomes a simple "user interface" (CLI) that calls the library functions. This is like separating the "Chef's skills" (Logic) from the "Menu" (Interface).
- **Myanmar:** Rust မှာ `lib.rs` ဖိုင်ကို သုံးခြင်းအားဖြင့် တခြား program တွေကနေ ကျွန်တော်တို့ရဲ့ logic ကို crate တစ်ခုအနေနဲ့ ပြန်ခေါ်သုံးလို့ ရပါတယ်။ STT engine ကို library ထဲ ရွှေ့လိုက်တဲ့အတွက် ကျွန်တော်တို့ tool ကို နေရာတိုင်းမှာ ပြန်သုံးလို့ ရသွားပါပြီ။ `main.rs` ကတော့ logic တွေကို ခေါ်သုံးပေးတဲ့ "user interface" (CLI) တစ်ခုပဲ ဖြစ်သွားပါတယ်။ ဒါဟာ "စားဖိုမှူးရဲ့ ကျွမ်းကျင်မှု" (Logic) နဲ့ "စားသောက်ဖွယ်ရာ စာရင်း" (Interface) ကို သီးသန့်ခွဲခြားလိုက်တာနဲ့ တူပါတယ်။

## [2026-04-14] Senior Rust Standards & UX Refinement
### Learning Moment
- **English:** Transitioning to Rust 2024 and using `miette` for diagnostic reporting is like upgrading from a basic toolkit to a professional workstation. Instead of generic "something went wrong" messages, we now provide rich, actionable error reports that guide the user to a solution. Using `indicatif` for progress bars transforms a "black box" operation into a transparent experience.
- **Myanmar:** Rust 2024 သို့ ကူးပြောင်းခြင်းနှင့် `miette` ကို အသုံးပြု၍ အမှားရှာဖွေခြင်း (Diagnostics) သည် သာမန်ကိရိယာများမှ ကျွမ်းကျင်အဆင့်သုံး workstation တစ်ခုသို့ အဆင့်မြှင့်တင်လိုက်သလိုမျိုး ဖြစ်ပါသည်။ `indicatif` ကို အသုံးပြု၍ Progress Bar များ ထည့်သွင်းခြင်းသည် ပိတ်ထားသော အခန်းထဲ၌ အလုပ်လုပ်နေသည်ကို အပြင်မှ မြင်သာအောင် ပြတင်းပေါက် ဖွင့်ပေးလိုက်သလိုမျိုး ဖြစ်ပါသည်။

### Command Overview (Analogy)
- **`bash setup_termux.sh` (The Kitchen Builder):** Run once to install dependencies (`rust`, `make`, `libllvm`, `openssl`).
- **`make run` (The Smart Chef):** The standard way to run. Automatically sets `LD_LIBRARY_PATH`.
- **`cargo run` (The Basic Apprentice):** Fails if environment is not set.

## [2026-04-17] Advanced Pipeline & Local AI Integration
### Learning Moment
- **English:** Scaling a pipeline requires handling "Concurrency" and "Resilience". We implemented batching (splitting text into small groups) to avoid crashing the AI's memory. We also learned that local AI (llama-server) needs a "Warm-up" time; checking its health before sending data is crucial to prevent connection errors.
- **Myanmar:** လုပ်ငန်းစဉ်တစ်ခုကို ကျွမ်းကျင်အဆင့်သို့ မြှင့်တင်ရာတွင် "ပြိုင်တူလုပ်ဆောင်ခြင်း (Concurrency)" နှင့် "ခံနိုင်ရည်ရှိခြင်း (Resilience)" တို့ လိုအပ်ပါသည်။ AI ၏ မှတ်ဉာဏ်ပမာဏကို မကျော်လွန်စေရန် စာသားများကို အုပ်စုလိုက်ခွဲခြားခြင်း (Batching) ကို ပြုလုပ်ခဲ့ပါသည်။ ထို့ပြင် Local AI (llama-server) သည် စတင်ပွင့်လာရန် အချိန်တစ်ခုယူရသောကြောင့် ၎င်းအဆင်သင့်ဖြစ်မဖြစ် (Health Check) စစ်ဆေးခြင်းသည် အရေးကြီးကြောင်း သင်ယူခဲ့ရပါသည်။

### Command Explanations
- **Check Server Health:** `curl http://127.0.0.1:8080/health` (Verifying if the local AI is awake).
- **Start Llama Server:** `llama-server -m <model> --port 8080 -c 2048` (Manually launching the offline engine).

### Architecture Decisions
- **Parallel Pipeline:** Used `tokio::spawn` to translate multiple batches simultaneously, making the process 5x faster.
- **Model Loading Loop:** Implemented a 60-second retry loop in `main.rs` to wait for the local LLM to finish loading into RAM.

## [2026-04-19] Professional Refinement & Repository Hygiene
### Learning Moment
- **English:** A professional project involves smart data persistence and clean history. We integrated a local database (SQLite) using the Model Context Protocol (MCP) concepts, allowing the system to "remember" previous translations and save on API costs. We also learned that "Git Hygiene"—wiping messy history and starting fresh—is sometimes the best way to deliver a clean product.
- **Myanmar:** ကျွမ်းကျင်အဆင့်မီ ပရောဂျက်တစ်ခုတွင် "ဒေတာသိမ်းဆည်းမှု" နှင့် "သန့်ရှင်းသော ရာဇဝင်" တို့ ပါဝင်သည်။ SQLite database ကို အသုံးပြု၍ ယခင်ဘာသာပြန်ဆိုချက်များကို "မှတ်မိနေစေရန်" (MyMemory) ပြုလုပ်ခဲ့သဖြင့် API ကုန်ကျစရိတ်ကို လျှော့ချနိုင်ခဲ့ပါသည်။ ထို့ပြင် Git ရာဇဝင်များကို အစမှပြန်စခြင်း (Git History Wipe) သည် သေသပ်သော ထုတ်ကုန်တစ်ခုအတွက် အကောင်းဆုံးဖြစ်ကြောင်း သင်ယူခဲ့ရပါသည်။

### Command Explanations
- **FFmpeg Conversion:** `ffmpeg -i input.mp4 -vn -acodec libmp3lame output.mp3` (Extracting audio from video).
- **Git Reset:** `rm -rf .git && git init` (Deleting all history to start fresh on the `main` branch).
- **Directory Discovery:** `std::fs::read_dir` (Automatically scanning folders to find files and models).

### Architecture Decisions
- **MyMemory (SQLite):** Used `sqlx` to build a local translation cache that works 100% offline.
- **Multi-Model Support:** Reorganized the `models/` directory so users can pick between English, Korean, Chinese, etc.
- **Modular Refactor:** Moved logic into `engine`, `services`, and `ui` directories to prevent "Spaghetti Code".
- **Unified Main:** Simplified `main.rs` to act as a clean orchestrator for the entire professional pipeline.

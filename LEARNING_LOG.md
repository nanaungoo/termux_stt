# Learning Log (သင်ယူမှုမှတ်တမ်း)

## [2026-04-13] Instruction Set Architecture (ISA) - x86 vs ARM
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
- **Cross-Platform Abstraction**: Used `#[cfg(target_os = "android")]` to handle Termux-only microphone commands without breaking Ubuntu builds.

## [2026-04-13] Library-First Refactor & CLI Update
### Learning Moment
- **English:** In Rust, a `lib.rs` file allows other programs to use your logic as a crate. By moving our STT engine there, we make our tool "reusable." The `main.rs` then becomes a simple "user interface" (CLI) that calls the library functions. This is like separating the "Chef's skills" (Logic) from the "Menu" (Interface).
- **Myanmar:** Rust မှာ `lib.rs` ဖိုင်ကို သုံးခြင်းအားဖြင့် တခြား program တွေကနေ ကျွန်တော်တို့ရဲ့ logic ကို crate တစ်ခုအနေနဲ့ ပြန်ခေါ်သုံးလို့ ရပါတယ်။ STT engine ကို library ထဲ ရွှေ့လိုက်တဲ့အတွက် ကျွန်တော်တို့ tool ကို နေရာတိုင်းမှာ ပြန်သုံးလို့ ရသွားပါပြီ။ `main.rs` ကတော့ logic တွေကို ခေါ်သုံးပေးတဲ့ "user interface" (CLI) တစ်ခုပဲ ဖြစ်သွားပါတယ်။ ဒါဟာ "စားဖိုမှူးရဲ့ ကျွမ်းကျင်မှု" (Logic) နဲ့ "စားသောက်ဖွယ်ရာ စာရင်း" (Interface) ကို သီးသန့်ခွဲခြားလိုက်တာနဲ့ တူပါတယ်။

### Command Explanations
- **Transcribe File:** `cargo run -- transcribe <path_to_audio>`
- **Record Mode (Termux):** `cargo run -- record`
- **Help Menu:** `cargo run -- --help`

## [2026-04-13] Senior Rust Architect Standards & UX Refinement
### Learning Moment
- **English:** Transitioning to Rust 2024 and using `miette` for diagnostic reporting is like upgrading from a basic toolkit to a professional workstation. Instead of generic "something went wrong" messages, we now provide rich, actionable error reports that guide the user to a solution. Using `indicatif` for progress bars transforms a "black box" operation into a transparent experience, giving the user real-time feedback on long-running tasks.
- **Myanmar:** Rust 2024 သို့ ကူးပြောင်းခြင်းနှင့် `miette` ကို အသုံးပြု၍ အမှားရှာဖွေခြင်း (Diagnostics) သည် သာမန်ကိရိယာများမှ ကျွမ်းကျင်အဆင့်သုံး workstation တစ်ခုသို့ အဆင့်မြှင့်တင်လိုက်သလိုမျိုး ဖြစ်ပါသည်။ "တစ်ခုခု မှားယွင်းနေသည်" ဟုသာ ပြမည့်အစား အမှားကို မည်သို့ ဖြေရှင်းရမည်ဆိုသည့် အကြံပြုချက်များအထိ အသေးစိတ် ဖော်ပြပေးနိုင်ပါသည်။ `indicatif` ကို အသုံးပြု၍ Progress Bar များ ထည့်သွင်းခြင်းသည် ပိတ်ထားသော အခန်းထဲ၌ အလုပ်လုပ်နေသည်ကို အပြင်မှ မြင်သာအောင် ပြတင်းပေါက် ဖွင့်ပေးလိုက်သလိုမျိုး ဖြစ်ပါသည်။ ၎င်းသည် အသုံးပြုသူအား လုပ်ငန်းစဉ်၏ အခြေအနေကို အချိန်နှင့်တပြေးညီ သိရှိစေပါသည်။

### Command Explanations
- **Run Tests with Makefile:** `make test` (Automates LD_LIBRARY_PATH and runs all unit tests).
- **Build with Release Profile:** `cargo build --release` (Optimized for speed and smaller binary size).
- **Clean Build Artifacts:** `make clean` (Clears target directory to resolve build conflicts).

### Architecture Decisions
- **Zero `unwrap()` Policy:** Enforced total error propagation in library modules to ensure the application never crashes unexpectedly.
- **Asynchronous Process Handling:** Leveraged `tokio::process` for the microphone stream to keep the async runtime responsive on Android/Termux.
- **Makefile Automation:** Created a unified entry point for building and testing that automatically detects system architecture and sets shared library paths.

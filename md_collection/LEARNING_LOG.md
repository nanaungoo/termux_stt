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

## [2026-04-13] Library-First Refactor & CLI Update
### Learning Moment
- **English:** In Rust, a `lib.rs` file allows other programs to use your logic as a crate. By moving our STT engine there, we make our tool "reusable." The `main.rs` then becomes a simple "user interface" (CLI) that calls the library functions. This is like separating the "Chef's skills" (Logic) from the "Menu" (Interface).
- **Myanmar:** Rust မှာ `lib.rs` ဖိုင်ကို သုံးခြင်းအားဖြင့် တခြား program တွေကနေ ကျွန်တော်တို့ရဲ့ logic ကို crate တစ်ခုအနေနဲ့ ပြန်ခေါ်သုံးလို့ ရပါတယ်။ STT engine ကို library ထဲ ရွှေ့လိုက်တဲ့အတွက် ကျွန်တော်တို့ tool ကို နေရာတိုင်းမှာ ပြန်သုံးလို့ ရသွားပါပြီ။ `main.rs` ကတော့ logic တွေကို ခေါ်သုံးပေးတဲ့ "user interface" (CLI) တစ်ခုပဲ ဖြစ်သွားပါတယ်။ ဒါဟာ "စားဖိုမှူးရဲ့ ကျွမ်းကျင်မှု" (Logic) နဲ့ "စားသောက်ဖွယ်ရာ စာရင်း" (Interface) ကို သီးသန့်ခွဲခြားလိုက်တာနဲ့ တူပါတယ်။

## [2026-04-14] Senior Rust Standards & UX Refinement
### Learning Moment
- **English:** Transitioning to Rust 2024 and using `miette` for diagnostic reporting is like upgrading from a basic toolkit to a professional workstation. Instead of generic "something went wrong" messages, we now provide rich, actionable error reports that guide the user to a solution. Using `indicatif` for progress bars transforms a "black box" operation into a transparent experience.
- **Myanmar:** Rust 2024 သို့ ကူးပြောင်းခြင်းနှင့် `miette` ကို အသုံးပြု၍ အမှားရှာဖွေခြင်း (Diagnostics) သည် သာမန်ကိရိယာများမှ ကျွမ်းကျင်အဆင့်သုံး workstation တစ်ခုသို့ အဆင့်မြှင့်တင်လိုက်သလိုမျိုး ဖြစ်ပါသည်။ `indicatif` ကို အသုံးပြု၍ Progress Bar များ ထည့်သွင်းခြင်းသည် ပိတ်ထားသော အခန်းထဲ၌ အလုပ်လုပ်နေသည်ကို အပြင်မှ မြင်သာအောင် ပြတင်းပေါက် ဖွင့်ပေးလိုက်သလိုမျိုး ဖြစ်ပါသည်။

### Command Overview (Analogy)
- **`bash setup_termux.sh` (The Kitchen Builder):**
    - *English:* Run once. Installs the stove, sink, and fridge (Dependencies & Model).
    - *Myanmar:* တစ်ကြိမ်သာ run ရန်။ မီးဖို၊ လက်ဆေးကန်နှင့် ရေခဲသေတ္တာတို့ကို လာရောက်တပ်ဆင်ပေးသည့်အဖွဲ့နှင့် တူပါသည်။
- **`make run` (The Smart Chef):**
    - *English:* The standard way to run. Automatically brings the right knives to the counter (`LD_LIBRARY_PATH`).
    - *Myanmar:* ပုံမှန်အသုံးပြုရန် နည်းလမ်း။ ဟင်းမချက်မီ မည်သည့်ဓားကို သုံးရမည်ကို သိပြီး အသင့်ယူဆောင်လာသော ကျွမ်းကျင်စားဖိုမှူးနှင့် တူပါသည်။
- **`cargo run` (The Basic Apprentice):**
    - *English:* Follows the recipe but forgets the tools. Fails if environment is not set.
    - *Myanmar:* ဟင်းချက်နည်းကိုသာ သိပြီး လိုအပ်သော ကိရိယာများကို အသင့်ယူမလာဘဲ ဟင်းစချက်သည့် အလုပ်သင်နှင့် တူပါသည်။

## [2026-04-14] Library Dependencies & Interactive UX
### Learning Moment
- **English:** We learned that binary files (`.so`) are environment-sensitive. A library built for Android won't work on standard Ubuntu due to missing "Wall Sockets" like `liblog.so`. Replacing them with native Linux binaries fixed the "Invalid ELF Header" errors. Additionally, we learned that automated file listing removes the "Memory Burden" from the user, making the CLI intuitive and error-proof.
- **Myanmar:** Binary ဖိုင်တွေ (`.so`) ဟာ သူတို့အလုပ်လုပ်မယ့် ပတ်ဝန်းကျင်အပေါ် မူတည်တယ်ဆိုတာ ကျွန်တော်တို့ သင်ယူခဲ့ရပါတယ်။ Android အတွက် တည်ဆောက်ထားတဲ့ library တွေဟာ Ubuntu ပေါ်မှာ `liblog.so` လိုမျိုး လိုအပ်ချက်တွေ မရှိတာကြောင့် အလုပ်မလုပ်ပါဘူး။ ၎င်းတို့ကို native Linux binaries တွေနဲ့ အစားထိုးခြင်းက ELF Header အမှားတွေကို ပြေလည်စေခဲ့ပါတယ်။ ဒါ့အပြင် ဖိုင်စာရင်းတွေကို အလိုအလျောက် ပြသပေးခြင်းဟာ အသုံးပြုသူအတွက် မှတ်ဉာဏ်ဝန်ထုပ်ဝန်ပိုးကို လျှော့ချပေးပြီး CLI ကို ပိုမိုလွယ်ကူစေပါတယ်။

### Key Command Insights
- **Robust Path Finding:** `${BASH_SOURCE[0]}` ensures scripts identify their own location even when "sourced" (using `.` command).
- **Library Linkage:** `ln -s libatomic.so libatomic.so.1` bridges the gap between different library version names in Termux.
- **Interactive Discovery:** Using `std::fs::read_dir` to populate menus dynamically is safer than manual string input.

### Architecture Decisions
- **Unified Wrapper:** The `Makefile` is now the mandatory entry point to abstract away platform-specific environment variables.
- **Input/Output Persistence:** Added `.gitkeep` to empty folders to ensure the project structure is always "visible" and ready after a fresh clone.
- **Clean Library Code:** Enforced a strict "No `unwrap()`" policy in `core.rs`, `config.rs`, and `error.rs` for professional stability.

## [2026-04-14] Audio Engineering & Processing Accuracy
### Learning Moment
- **English:** We learned that a Speech-to-Text engine is like a specialized translator who only understands a specific dialect at a specific speed. If you feed it 48,000 samples per second when it expects 16,000, or speak into two microphones (stereo) at once, the "translator" gets confused. By implementing dynamic sample rate detection and stereo-to-mono merging, we made our "translator" adaptable to any audio file "dialect," drastically improving transcription accuracy.
- **Myanmar:** Speech-to-Text engine တစ်ခုဟာ သတ်မှတ်ထားတဲ့ အမြန်နှုန်းနဲ့ လေယူလေသိမ်းကိုပဲ နားလည်တဲ့ ဘာသာပြန်ဆရာတစ်ယောက်နဲ့ တူတယ်ဆိုတာ ကျွန်တော်တို့ သိခဲ့ရပါတယ်။ ၁၆,၀၀၀ Hz ကို မျှော်လင့်ထားတဲ့ ဆရာ့ကို ၄၈,၀၀၀ Hz အမြန်နှုန်းနဲ့ ပြောရင် ဒါမှမဟုတ် မိုက်ခရိုဖုန်း နှစ်လုံး (Stereo) နဲ့ တစ်ပြိုင်နက် ပြောရင် ဘာသာပြန်ဆရာ မျက်စိလည်သွားတတ်ပါတယ်။ Sample rate ကို အလိုအလျောက် စစ်ဆေးခြင်းနဲ့ Stereo အသံကို Mono အဖြစ် ပေါင်းစပ်ပေးခြင်းအားဖြင့် ကျွန်တော်တို့ရဲ့ "ဘာသာပြန်ဆရာ" ကို ဘယ်လိုအော်ဒီယိုဖိုင်မျိုးမဆို နားလည်အောင် ပြုလုပ်ပေးနိုင်ခဲ့ပြီး ရလဒ်တွေကို ပိုမိုတိကျစေခဲ့ပါတယ်။

### Key Technical Insights
- **Audio Probing:** Used `ffprobe` to "X-ray" audio files and discover their internal properties (Sample Rate, Channels, Bitrate) before processing.
- **Late Initialization:** Learned that waiting to initialize the `Recognizer` until the audio properties are known is better than using hardcoded defaults.
- **Signal Processing:** Implemented a "Sum and Average" DSP algorithm to merge stereo channels into a clean mono stream for the STT engine.

### Architecture Refinements
- **API Encapsulation:** Refactored the core library to hide internal complexity; the application now simply gives a file path to the engine, and the library handles the rest (probing, decoding, converting, and transcribing).
- **Error Transparency:** Used `miette` to provide beautiful and helpful error messages when files are missing or audio formats are unsupported.

## [2026-04-14] Modular Refactoring & Code Organization
### Learning Moment
- **English:** We learned that as a project grows, putting everything in one place is like keeping all your clothes in one giant pile. **Modularization** is like building a wardrobe with specific drawers for shirts, pants, and socks. By separating our logic into `engine`, `services`, and `ui`, we made the codebase easier to navigate, test, and expand without creating a "Spaghetti Code" mess.
- **Myanmar:** ပရောဂျက်တစ်ခု ကြီးထွားလာတဲ့အခါ အရာအားလုံးကို နေရာတစ်ခုတည်းမှာ စုထားတာဟာ အဝတ်အစားအားလုံးကို ပုံကြီးတစ်ပုံတည်း စုပုံထားတာနဲ့ တူပါတယ်။ **Modularization** ဆိုတာကတော့ အင်္ကျီ၊ ဘောင်းဘီနဲ့ ခြေအိတ်တွေအတွက် သီးသန့်အကန့်တွေပါတဲ့ အဝတ်ဘီရိုတစ်ခု တည်ဆောက်လိုက်တာနဲ့ တူပါတယ်။ ကျွန်တော်တို့ရဲ့ logic တွေကို `engine`၊ `services` နဲ့ `ui` ဆိုပြီး ခွဲထုတ်လိုက်ခြင်းအားဖြင့် ကုဒ်တွေကို ရှာဖွေရလွယ်ကူစေပြီး ရှုပ်ထွေးပွေလီတဲ့ "Spaghetti Code" တွေ မဖြစ်အောင် ကာကွယ်ပေးနိုင်ခဲ့ပါတယ်။

### Architecture Decisions
- **Domain-Driven Design:** Grouped code based on its functional domain (Internal Engines vs. External Services vs. User Interface).
- **Module Encapsulation:** Used `mod.rs` files to control exactly what logic is exposed to the rest of the application, keeping internals private and safe.
- **Centralized Exports:** Updated `lib.rs` to re-export commonly used types, allowing the `main.rs` to stay clean and focused.

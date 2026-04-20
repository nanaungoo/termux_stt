# 🎤 Transcribe Command Documentation

This document explains the usage and technical details of the `transcribe` command in `termux_stt`.
ဤစာရွက်စာတမ်းသည် `termux_stt` ရှိ `transcribe` command ၏ အသုံးပြုပုံနှင့် နည်းပညာဆိုင်ရာ အသေးစိတ်အချက်အလက်များကို ရှင်းလင်းဖော်ပြထားပါသည်။

---

## 1. Overview (ခြုံငုံသုံးသပ်ချက်)

The `transcribe` command converts audio or video files into text (`.txt`) and subtitle (`.srt`) files using the Vosk STT engine.
`transcribe` command သည် Vosk STT engine ကို အသုံးပြု၍ အသံ သို့မဟုတ် ဗီဒီယိုဖိုင်များကို စာသားဖိုင် (`.txt`) နှင့် စာတန်းထိုးဖိုင် (`.srt`) များအဖြစ် ပြောင်းလဲပေးပါသည်။

---

## 2. CLI Usage (Command Line မှ အသုံးပြုပုံ)

You can run the transcription directly from the terminal by passing the file path and optional arguments.
ဖိုင်လမ်းကြောင်းနှင့် လိုအပ်သော argument များကို ထည့်သွင်း၍ terminal မှ တိုက်ရိုက် run နိုင်ပါသည်။

### Syntax:
```bash
make run ARGS="transcribe <FILE_PATH> [OPTIONS]"
```

### Options:
| Option | Short | Description (ရှင်းလင်းချက်) |
| :--- | :--- | :--- |
| `file` | - | **Required:** Path to the audio/video file. (မဖြစ်မနေလိုအပ်သော အဝင်ဖိုင်လမ်းကြောင်း) |
| `--output` | `-o` | **Optional:** Custom base name for output files. (output ဖိုင်အမည်ကို စိတ်ကြိုက်ပေးရန်) |
| `--out-dir` | - | **Optional:** Custom directory to save results. (output ဖိုင်သိမ်းဆည်းမည့် ဖိုဒါလမ်းကြောင်း) |

### Example:
```bash
make run ARGS="transcribe data/input/meeting.mp4 --output summary --out-dir /sdcard/Documents"
```

---

## 3. Interactive Usage (အပြန်အလှန် ညွှန်ကြားချက်ဖြင့် အသုံးပြုပုံ)

If you run `make run` without any arguments, the application enters interactive mode.
အကယ်၍ `make run` ကို argument မပါဘဲ run ပါက interactive mode သို့ ရောက်ရှိမည် ဖြစ်ပါသည်။

1.  **Select "Transcribe Audio File":** Use arrow keys to select the option.
    ("Transcribe Audio File" ကို ရွေးချယ်ပါ။)
2.  **Choose Model:** Select the language model you want to use.
    (အသုံးပြုလိုသော Language Model ကို ရွေးချယ်ပါ။)
3.  **Choose File:** Select a file from your `data/input` directory.
    (input ဖိုဒါထဲရှိ ဖိုင်တစ်ခုကို ရွေးချယ်ပါ။)
4.  **Confirm Output Name:** Enter a base name or press Enter for default.
    (Output ဖိုင်အမည်ကို အတည်ပြုပါ သို့မဟုတ် default အတိုင်း ထားရှိပါ။)
5.  **Confirm Output Directory:** Confirm the destination path for your results.
    (Output သိမ်းဆည်းမည့် ဖိုဒါလမ်းကြောင်းကို အတည်ပြုပါ။)

---

## 4. Technical Explanation (နည်းပညာဆိုင်ရာ ရှင်းလင်းချက်)

### 4.1. Automatic Audio Optimization
Before transcription starts, the engine automatically optimizes the audio using `ffmpeg` to ensure maximum accuracy:
transcription မစတင်မီ ပိုမိုတိကျသော ရလဒ်ရရှိရန် `ffmpeg` ကို အသုံးပြု၍ အောက်ပါအတိုင်း အလိုအလျောက် ပြင်ဆင်ပေးပါသည် -
- **Format:** Forced to `WAV` (PCM 16-bit Little Endian).
- **Channels:** Converted to `Mono` (1 channel).
- **Sample Rate:** Re-sampled to `16,000 Hz` (Vosk Native Standard).
- **Cleanup:** Strips all metadata to prevent audio decoding errors.

### 4.2. Output Generation
The command generates two files in the output directory:
ဤ command သည် output ဖိုဒါထဲတွင် ဖိုင်နှစ်ခုကို ထုတ်ပေးပါသည် -
1.  **`.txt` File:** Contains the full transcribed text. (transcription ရလဒ် စာသားများ အားလုံး ပါဝင်သည်။)
2.  **`.srt` File:** Subtitle file with timestamps for video/audio players. (အချိန်မှတ်တမ်းများ ပါဝင်သော စာတန်းထိုးဖိုင်။)

---

## 5. Troubleshooting (ပြဿနာဖြေရှင်းခြင်း)

- **"Unsupported Codec":** Ensure you have `ffmpeg` installed via `./setup_termux.sh`.
  (ffmpeg တပ်ဆင်ထားကြောင်း သေချာပါစေ။)
- **"Invalid Input":** Check if your audio file is corrupted or empty.
  (အဝင်ဖိုင် ပျက်စီးနေခြင်း သို့မဟုတ် ဗလာဖြစ်နေခြင်း ရှိမရှိ စစ်ဆေးပါ။)
- **Slow Transcription:** Using the "Small" model is faster for lower-end devices.
  (ဖုန်းအနိမ့်ပိုင်းများတွင် Small model ကို အသုံးပြုခြင်းက ပိုမိုမြန်ဆန်စေပါသည်။)

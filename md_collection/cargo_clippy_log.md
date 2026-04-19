warning: redundant closure
  --> src/engine/audio.rs:43:44
   |
43 |     let metadata = file.metadata().map_err(|e| SttError::Io(e))?;
   |                                            ^^^^^^^^^^^^^^^^^^^ help: replace the closure with the tuple variant itself: `SttError::Io`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#redundant_closure
   = note: `#[warn(clippy::redundant_closure)]` on by default

warning: accessing first element with `format
                 .tracks().get(0)`
  --> src/engine/audio.rs:66:17
   |
66 |       let track = format
   |  _________________^
67 | |         .tracks()
68 | |         .get(0)
   | |_______________^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#get_first
   = note: `#[warn(clippy::get_first)]` on by default
help: try
   |
66 ~     let track = format
67 +         .tracks().first()
   |

warning: this `if` statement can be collapsed
   --> src/engine/audio.rs:118:9
    |
118 | /         if let Ok(vosk::DecodingState::Finalized) = recognizer.accept_waveform(&mono_samples) {
119 | |             if let Some(res) = recognizer.result().single() {
120 | |                 all_results.push(OwnedResult {
121 | |                     text: res.text.to_string(),
...   |
133 | |         }
    | |_________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
    = note: `#[warn(clippy::collapsible_if)]` on by default
help: collapse nested if block
    |
118 ~         if let Ok(vosk::DecodingState::Finalized) = recognizer.accept_waveform(&mono_samples)
119 ~             && let Some(res) = recognizer.result().single() {
120 |                 all_results.push(OwnedResult {
...
131 |                 });
132 ~             }
    |

warning: unnecessary closure used to substitute value for `Option::None`
  --> src/engine/vosk.rs:31:13
   |
31 |             Recognizer::new(&self.model, sample_rate).ok_or_else(|| SttError::RecognizerError)?;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#unnecessary_lazy_evaluations
   = note: `#[warn(clippy::unnecessary_lazy_evaluations)]` on by default
help: use `ok_or` instead
   |
31 -             Recognizer::new(&self.model, sample_rate).ok_or_else(|| SttError::RecognizerError)?;
31 +             Recognizer::new(&self.model, sample_rate).ok_or(SttError::RecognizerError)?;
   |

warning: this `if` statement can be collapsed
   --> src/services/translator.rs:159:17
    |
159 | /                 if let Some(candidates) = gemini_resp.candidates {
160 | |                     if let Some(candidate) = candidates.first() {
161 | |                         if let Some(content) = &candidate.content {
162 | |                             let mut results = Vec::new();
...   |
198 | |                 }
    | |_________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
help: collapse nested if block
    |
159 ~                 if let Some(candidates) = gemini_resp.candidates
160 ~                     && let Some(candidate) = candidates.first() {
161 |                         if let Some(content) = &candidate.content {
...
196 |                         }
197 ~                     }
    |

warning: this `if` statement can be collapsed
   --> src/services/translator.rs:160:21
    |
160 | /                     if let Some(candidate) = candidates.first() {
161 | |                         if let Some(content) = &candidate.content {
162 | |                             let mut results = Vec::new();
163 | |                             let id_regex =
...   |
197 | |                     }
    | |_____________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
help: collapse nested if block
    |
160 ~                     if let Some(candidate) = candidates.first()
161 ~                         && let Some(content) = &candidate.content {
162 |                             let mut results = Vec::new();
...
195 |                             );
196 ~                         }
    |

warning: compiling a regex in a loop
   --> src/services/translator.rs:164:33
    |
164 | ...                   regex::Regex::new(r"(?m)^\[?(\d+)\]?[:\s.\-]*\s*(.*)$").unwrap();
    |                       ^^^^^^^^^^^^^^^^^
    |
help: move the regex construction outside this loop
   --> src/services/translator.rs:139:9
    |
139 |         for attempt in 0..3 {
    |         ^^^^^^^^^^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#regex_creation_in_loops
    = note: `#[warn(clippy::regex_creation_in_loops)]` on by default

warning: this `if` statement can be collapsed
   --> src/services/translator.rs:168:37
    |
168 | / ...                   if let Some(caps) = id_regex.captures(line.trim()) {
169 | | ...                       if let (Some(id_match), Some(text_match)) =
170 | | ...                           (caps.get(1), caps.get(2))
...   |
184 | | ...                   }
    | |_______________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
help: collapse nested if block
    |
168 ~                                     if let Some(caps) = id_regex.captures(line.trim())
169 ~                                         && let (Some(id_match), Some(text_match)) =
170 |                                             (caps.get(1), caps.get(2))
...
182 |                                             }
183 ~                                         }
    |

warning: this `if` statement can be collapsed
   --> src/services/translator.rs:169:41
    |
169 | / ...                   if let (Some(id_match), Some(text_match)) =
170 | | ...                       (caps.get(1), caps.get(2))
171 | | ...                   {
172 | | ...                       if let Ok(id) = id_match.as_str().parse::<usize>() {
...   |
183 | | ...                   }
    | |_______________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
help: collapse nested if block
    |
170 ~                                             (caps.get(1), caps.get(2))
171 ~                                             && let Ok(id) = id_match.as_str().parse::<usize>() {
172 |                                                 let text = text_match.as_str().trim();
...
180 |                                                 results.push((id, fixed_text));
181 ~                                             }
    |

warning: this `if` statement can be collapsed
   --> src/services/translator.rs:336:17
    |
336 | /                 if let Some(choices) = llama_resp.choices {
337 | |                     if let Some(choice) = choices.first() {
338 | |                         if let Some(message) = &choice.message {
339 | |                             let mut results = Vec::new();
...   |
372 | |                 }
    | |_________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
help: collapse nested if block
    |
336 ~                 if let Some(choices) = llama_resp.choices
337 ~                     && let Some(choice) = choices.first() {
338 |                         if let Some(message) = &choice.message {
...
370 |                         }
371 ~                     }
    |

warning: this `if` statement can be collapsed
   --> src/services/translator.rs:337:21
    |
337 | /                     if let Some(choice) = choices.first() {
338 | |                         if let Some(message) = &choice.message {
339 | |                             let mut results = Vec::new();
340 | |                             let id_regex =
...   |
371 | |                     }
    | |_____________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
help: collapse nested if block
    |
337 ~                     if let Some(choice) = choices.first()
338 ~                         && let Some(message) = &choice.message {
339 |                             let mut results = Vec::new();
...
369 |                             );
370 ~                         }
    |

warning: compiling a regex in a loop
   --> src/services/translator.rs:341:33
    |
341 | ...                   regex::Regex::new(r"(?m)^\[?(\d+)\]?[:\s.\-]*\s*(.*)$").unwrap();
    |                       ^^^^^^^^^^^^^^^^^
    |
help: move the regex construction outside this loop
   --> src/services/translator.rs:316:9
    |
316 |         for attempt in 0..3 {
    |         ^^^^^^^^^^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#regex_creation_in_loops

warning: this `if` statement can be collapsed
   --> src/services/translator.rs:344:33
    |
344 | / ...                   if let Some(caps) = id_regex.captures(line.trim()) {
345 | | ...                       if let (Some(id_match), Some(text_match)) =
346 | | ...                           (caps.get(1), caps.get(2))
...   |
359 | | ...                   }
    | |_______________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
help: collapse nested if block
    |
344 ~                                 if let Some(caps) = id_regex.captures(line.trim())
345 ~                                     && let (Some(id_match), Some(text_match)) =
346 |                                         (caps.get(1), caps.get(2))
...
357 |                                         }
358 ~                                     }
    |

warning: this `if` statement can be collapsed
   --> src/services/translator.rs:345:37
    |
345 | / ...                   if let (Some(id_match), Some(text_match)) =
346 | | ...                       (caps.get(1), caps.get(2))
347 | | ...                   {
348 | | ...                       if let Ok(id) = id_match.as_str().parse::<usize>() {
...   |
358 | | ...                   }
    | |_______________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
help: collapse nested if block
    |
346 ~                                         (caps.get(1), caps.get(2))
347 ~                                         && let Ok(id) = id_match.as_str().parse::<usize>() {
348 |                                             let text = text_match.as_str().trim();
...
355 |                                             results.push((id, fixed_text));
356 ~                                         }
    |

warning: this `if` statement can be collapsed
   --> src/services/tts.rs:120:21
    |
120 | /                     if let Message::Text(txt) = msg {
121 | |                         if let Ok(live_msg) = serde_json::from_str::<LiveServerMessage>(&txt) {
122 | |                             if let Some(server_content) = live_msg.server_content {
123 | |                                 if let Some(model_turn) = server_content.model_turn {
...   |
141 | |                     }
    | |_____________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
help: collapse nested if block
    |
120 ~                     if let Message::Text(txt) = msg
121 ~                         && let Ok(live_msg) = serde_json::from_str::<LiveServerMessage>(&txt) {
122 |                             if let Some(server_content) = live_msg.server_content {
...
139 |                             }
140 ~                         }
    |

warning: this `if` statement can be collapsed
   --> src/services/tts.rs:121:25
    |
121 | /                         if let Ok(live_msg) = serde_json::from_str::<LiveServerMessage>(&txt) {
122 | |                             if let Some(server_content) = live_msg.server_content {
123 | |                                 if let Some(model_turn) = server_content.model_turn {
124 | |                                     for part in model_turn.parts {
...   |
140 | |                         }
    | |_________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
help: collapse nested if block
    |
121 ~                         if let Ok(live_msg) = serde_json::from_str::<LiveServerMessage>(&txt)
122 ~                             && let Some(server_content) = live_msg.server_content {
123 |                                 if let Some(model_turn) = server_content.model_turn {
...
138 |                                 }
139 ~                             }
    |

warning: this `if` statement can be collapsed
   --> src/services/tts.rs:125:41
    |
125 | / ...                   if let Some(inline_data) = part.inline_data {
126 | | ...                       if let Some(b64) = inline_data.data {
127 | | ...                           if let Ok(decoded) =
128 | | ...                               general_purpose::STANDARD.decode(b64)
...   |
133 | | ...                   }
    | |_______________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
help: collapse nested if block
    |
125 ~                                         if let Some(inline_data) = part.inline_data
126 ~                                             && let Some(b64) = inline_data.data {
127 |                                                 if let Ok(decoded) =
...
131 |                                                 }
132 ~                                             }
    |

warning: this `if` statement can be collapsed
   --> src/services/tts.rs:126:45
    |
126 | / ...                   if let Some(b64) = inline_data.data {
127 | | ...                       if let Ok(decoded) =
128 | | ...                           general_purpose::STANDARD.decode(b64)
...   |
132 | | ...                   }
    | |_______________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
help: collapse nested if block
    |
126 ~                                             if let Some(b64) = inline_data.data
127 ~                                                 && let Ok(decoded) =
128 |                                                     general_purpose::STANDARD.decode(b64)
129 |                                                 {
130 |                                                     audio_data.extend(decoded);
131 ~                                                 }
    |

warning: this can be `std::io::Error::other(_)`
   --> src/services/tts.rs:191:39
    |
191 |             .map_err(|e| SttError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#io_other_error
    = note: `#[warn(clippy::io_other_error)]` on by default
help: use `std::io::Error::other`
    |
191 -             .map_err(|e| SttError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
191 +             .map_err(|e| SttError::Io(std::io::Error::other(e)))?;
    |

warning: this can be `std::io::Error::other(_)`
   --> src/services/tts.rs:198:47
    |
198 |                     .map_err(|e| SttError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#io_other_error
help: use `std::io::Error::other`
    |
198 -                     .map_err(|e| SttError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
198 +                     .map_err(|e| SttError::Io(std::io::Error::other(e)))?;
    |

warning: `termux_stt` (lib) generated 20 warnings (run `cargo clippy --fix --lib -p termux_stt` to apply 18 suggestions)
warning: this `if` statement can be collapsed
   --> src/main.rs:414:21
    |
414 | /                     if !text.is_empty() {
415 | |                         if let Ok(data) = tts.generate_audio(&text).await {
416 | |                             audio_segments.push(data);
417 | |                         }
418 | |                     }
    | |_____________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#collapsible_if
    = note: `#[warn(clippy::collapsible_if)]` on by default
help: collapse nested if block
    |
414 ~                     if !text.is_empty()
415 ~                         && let Ok(data) = tts.generate_audio(&text).await {
416 |                             audio_segments.push(data);
417 ~                         }
    |

warning: `termux_stt` (bin "termux_stt") generated 1 warning (run `cargo clippy --fix --bin "termux_stt" -p termux_stt` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s

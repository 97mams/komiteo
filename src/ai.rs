use std::process::Command;

pub fn ai_agent(api_key: &str, prompt: &str)-> String {
    let response = Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg("https://api.openai.com/v1/chat/completions")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-H")
        .arg(format!("Authorization: Bearer {}", api_key))
        .arg("-d")
        .arg(format!(r#"{{"model": "arcee-ai/trinity-large-preview:free",
                "messages": [
                    {{
                        "role": "system",
                        "content": "Generate a short Git commit message in English based on this git diff.\n\n  Format:\n  <type>: <body>\n\n  Rules:\n  - Use conventional types (feat, fix, refactor, chore, docs, test)\n  - Present tense\n  - One line only\n\n  Git diff:{p}"
                    }},
                    {{
                        "role": "user",
                        "content": "If you built the world'\''s tallest skyscraper, what would you name it?"
                    }}
                ],
                "stream": true
            }}'"#, p = prompt)
        )
        .output()
        .expect("Failed to execute AI agent");
     String::from_utf8_lossy(&response.stdout).to_string()
}
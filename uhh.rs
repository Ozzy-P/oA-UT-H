// Enum representing the states of a [REDACTED]
enum LockState {
    Locked,
    Unlocked,
    Jammed,
    LowBattery,
}

enum Command {
    Lock,
    Unlock,
    Jam,
}

// oAuth credentials for [REDACTED]
enum OAuthCredentials {
    ClientId(String),
    ClientSecret(String),
    RedirectUri(String),
    Token(String),
}

// Impl for LockState
impl LockState {
    fn description(&self) -> &str {
        match self {
            LockState::Locked => "Secured.",
            LockState::Unlocked => "Open!! Please do not enter.",
            LockState::Jammed => "The lock is jammed. Please slam with maximum force.",
            LockState::LowBattery => "Battery low. How did you even get in?",
        }
    }
    fn process(&self, command: Command) -> &str {
        match command {
            Command::Lock => {
                match self {
                    LockState::Locked => "Already locked, fool.",
                    LockState::Unlocked => "Locking now. Hope you didn't leave anything important inside.",
                    LockState::Jammed => "Jammed due to CIA interference. Cannot lock.",
                    LockState::LowBattery => "Warning. Battery is low. Ignoring, unless you wanna get locked out, which I'll do anyway.",
                }
            }
            Command::Unlock => {
                match self {
                    LockState::Locked => "Unlocking now. Watch out for that robber behind you!!",
                    LockState::Unlocked => "Already unlocked, fool.",
                    LockState::Jammed => "Cannot unlock. The lock is jammed (aka it didnt lock).",
                    LockState::LowBattery => "Unlocking in 10000 years.",
                }
            }
            Command::Jam => {
                match self {
                    LockState::Locked => "You can't jam a locked lock. Try unlocking it first.",
                    LockState::Unlocked => "Jamming now. Brace yourself, FBI incoming at the front!",
                    LockState::Jammed => "Already jammed, fool.",
                    LockState::LowBattery => "Okay fine. Jamming now. But you owe me a battery or two.",
                }
            }
        }
    }
}

fn main() {
    // TODO: Everything. Obviously.
}

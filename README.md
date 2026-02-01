# RoutAIr
A conceptual web application that performs packet priority optimisation using the judgement of AI to, in the case of an emergency services outage, prioritise Internet traffic towards emergency services to facilitate critical response.

Created for the HackTheFuture hackathon.

# Technology
LLM interaction is done through API calls to OpenAI, interacting with `gpt-5-nano`. The backend is written in concurrent Rust, utilising Warp, Tokio and bindings to libpcap, as well as employing MPSC and subsystem modularisation. The frontend is written in HTML, CSS, and JavaScript.

# Usage
The backend will perpetually scan for network activity. When `Request Packet` is clicked, the latest packet the backend retrieved will be shown. The backend will then run analysis, returning the verdict of the importance of the packet (whether it should be passed through, or held back).

# Notes
- The OPENAI_KEY environment variable is required to be set to your OpenAI key to run the program.
- `libpcap` must be installed.
- The executable must have the appropriate packet capturing capabilities.
- Refreshing the page while packet analysis is running requires a restart of the backend.

See `run_nokey.sh` for an example build & run script that automatically sets the capabilities of the output executable.

# RoutAIr
An emergency network optimizer that performs packet priority optimisation using the judgements from deterministic AI prompting, and monitored using a conceptual web application, to prioritise Internet traffic towards emergency services when required to facilitate critical response.

Created for the HackTheFuture Carleton MSA hackathon.

# Applications
Integration into existing cellular network an mobile service stations through partnerships with network providers for large scale implemention without the need for individual users to modify their existing setups. Emergency services may also implement their own version of this service for their local networks to filter non emergency traffic.

# Examples
An on-campus example of the application of this service would be in the event of a severe snowstorm that disables large amounts of networking infastructure. The event would activate this emergency network optimizer, which would proceed to monitor all incoming and outgoing network packets sent on the remaining networking infastructure. The packets are then analysed and graded using a neural network designed to prioritize network connectivity based on urgency. If a packet is judged by the machine learning algorithm to be essential, such as an emergency call to family or emergency services, such as first responders and 911 calls, the packet is prioritized over others the neural network deems non-essential. 

A more extreme example would be in the event a hurricane devistates a district, city, or large areas of land, taking communications infrastructure down with it. The service's activation would be more wide-spread in this situation and have a stricter guideline for what data is priritized. In this case, emergency and humanitarian aid services are prioritized over all other data to priotitize saving human ives first.

# Technology
LLM interaction is done through API calls to OpenAI, interacting with `gpt-5-nano`. The service and web application backend is written in concurrent Rust, utilising Warp, Tokio and bindings to libpcap, as well as employing MPSC and subsystem modularisation. The frontend is written in HTML, CSS, and JavaScript.

# Usage
The backend will perpetually process network activity. On the frontend, the web application allows for the live visualization of the process through an interactible GUI. When a user clicks the `Request Packet` button, the latest packet the service retrieved and processed will be shown, along with the analysis that ran, and return the verdict of the importance of the packet (whether it should be passed through, or held back).

# Notes
- The OPENAI_KEY environment variable is required to be set to your OpenAI key to run the program.
- `libpcap` must be installed.
- The executable must have the appropriate packet capturing capabilities.
- Refreshing the page while packet analysis is running requires a restart of the backend.

See `run_nokey.sh` for an example build & run script that automatically sets the capabilities of the output executable.

## Atticus

Atticus is an attempt to simplify the "API runner" application

# Motivation
 Postman, being the default API runner, and testing utility, had me frustrated.
  1. it gets painfully slow (especially with larger payloads, or if you have tons of tabs open..)
  2. Occasionally, it just refuses to work (presumably because it's having difficulty phoning home? )
  3. That's all I can think of right now, but it was enough for me to start this project.

# Some information about the project

 - Written in Rust
 - Uses the lapce/floem UI library
 - Uses Reqwest in te background
 - serde_json for the json-related work (formatting, and saving and loading the envirnonment) 
 - Local binary only
 - No Electron
 - No Cloud sync
   
# Planned features:
 - Collections
 - Environments
 - import curl requests
 - import Postman Collections

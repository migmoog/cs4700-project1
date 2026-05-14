# Approach

This program was implemented in the rust programming language. All packages used are in [ Cargo.toml ]
1. Check flags and choose to run in TLS or non-TLS mode
2. create accompanying TcpStream to connect to the server
3. intialize a message queue, populate it with a beginning hello message. Then initialize the "Wordleizer" struct with the wordbank.
4. serialize the message and write it to the stream
5. read stream until encountering a newline
6. serialize json bytes into message structure
7. match the received message
  - "start": make random guess from wordleizer and put guess into message queue
  - "retry": take last guess result and feed it into wordleizer's "adjust" method. Then make another guess and put it into message queue
  - "bye": print flag and end program

# Challenges

- the socket wouldn't send the entire structure, so I spent a good chunk of time configuring the read step in my program's loop
- TLS streams are difficult to set up with popular rust libraries due to the expectation of certificates

# Assistance

- The guessing strategy is derived from the `rs_wordle_solver` package. The specific pieces I used were the RandomGuesser struct, where it will 
eliminate unsuitable matches based on the wordbank provided. The `Wordleizer` struct acts as a translation layer between the protocol's json and rs_wordle_solver's own internals.
- The only use of LLM's involved searching for alternative TlsStream libraries for this project.

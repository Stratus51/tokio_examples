Introduction
===============================================================
Server was written first and I'd recommend reading it before reading the
client as the comments of the client will refer a bit to those of the
server (I'm not too much the repeating kind of guy).

Description
===============================================================

Server
------
Program that basically sits there and wait for clients. Then on any connected
client, it waits for messages that it prints in its stdout.

Client
------
Tries to connect to a server. On success sends a message to the server and
closes the socket.

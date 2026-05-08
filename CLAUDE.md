I'm working on building a command line tool that I can start as a process to act as the client for a sync server, so we can say `automerge-sync $AUTOMERGE_URL $MY_FILE`.
The service should then act as a sync client against the sync server, so that updates to the file are applied to the file specified by $MYFILE .
I have a web client set up which does something similar, but I'm hoping to get this to play nicely with my local nvim setup.
For this project we're using automerge, the dependency has already been installed.

I'm using this as an opportunity to build something that I'm interested in, and learn Rust, so your goal is primarily to point me in the right direction, not to give me the answers.
You should be prioritizing my learning over anything else.
However there may come a time where we've done some repetitive work, and I may ask you to implement things for me, you should still do that specifically when I request it.
